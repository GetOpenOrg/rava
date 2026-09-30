//! 被调方法签名解析（← `instr/invoke_sig.py`）。
//!
//! Python 以 Rust 类型串为形参 / 返回类型的载体；这里一律为 [`RsType`]，类以 binary 名定位。
//! 形参列表中的 `None` 表示该位置降级到描述符擦除类型（`jvm_to_rust(descriptor)`）。
//! 名字集合判定（`_downcast_target_valid` 等）按渲染文本的标识符记号进行，与 Python 同口径。

mod coerce_arg;
mod params;
mod ret;

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use ir::Expr;
use sim::StackSim;
use ty::{ClassInfo, JvmType, RsType};

use crate::build::{ir_ty, ty_text};
use crate::ctx::InstrCtx;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy;
use crate::invoke::CallRef;
use crate::log::InstrLog;
use crate::naming;

/// 类型形参 → 实参映射
pub type TargMap = BTreeMap<String, RsType>;

/// [`lookup_method_sig_params`] 的接收者视角
#[derive(Debug, Clone, Copy, Default)]
pub struct RecvView<'r> {
    /// callee 类型形参 → 接收者实参
    pub targ_map: Option<&'r TargMap>,
    /// 接收者是 `this`
    pub is_this: bool,
    /// 接收者静态类型
    pub ty: Option<&'r RsType>,
}

/// 渲染文本中的标识符记号（`[A-Za-z_][A-Za-z0-9_]*`）
pub(crate) fn ident_tokens(s: &str) -> Vec<&str> {
    let b = s.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        if b[i].is_ascii_alphabetic() || b[i] == b'_' {
            let j = (i..b.len()).find(|&j| !(b[j].is_ascii_alphanumeric() || b[j] == b'_')).unwrap_or(b.len());
            out.push(&s[i..j]);
            i = j;
        } else {
            i += 1;
        }
    }
    out
}

/// 基本类型 / `()`（Python `PRIMITIVE_RUST_TYPES`）
pub(crate) fn is_prim(t: &RsType) -> bool {
    matches!(t, RsType::Prim(_) | RsType::Unit)
}

/// 作用域外类型形参集的空集（Python `from_rust_type(t, registry)` 不带形参集）
pub(crate) fn jvm(ctx: &InstrCtx, t: &RsType) -> JvmType {
    ctx.ty.from_rs_type(t, &BTreeSet::new())
}

/// 类名（binary 或短名）→ 注册表内 binary（`_rust_type_to_binary` 口径）
pub(crate) fn resolve_cls(ctx: &InstrCtx, cls: &str) -> Option<String> {
    if ctx.reg().contains(cls) {
        return Some(cls.to_string());
    }
    let short = if cls.contains('/') { ctx.short(cls) } else { cls.to_string() };
    hierarchy::short_binary(ctx, &short)
}

/// 注册表内（任意）类的短名（`{short_cls(k) for k in registry}`）
pub(crate) fn is_registry_short(ctx: &InstrCtx, name: &str) -> bool {
    hierarchy::short_binary(ctx, name).is_some()
}

/// 注册表内非接口类的短名（`_generated_concrete_shorts`）
fn is_concrete_short(ctx: &InstrCtx, name: &str) -> bool {
    hierarchy::short_binary(ctx, name).and_then(|b| ctx.reg().get(&b)).is_some_and(|ci| !ci.is_interface())
}

/// 有类上界的类型变量接收者 → 上界类型视图；无类上界 → None
pub(crate) fn bound_view(env: &InstrEnv, sim: &StackSim, obj: &Expr, obj_ty: &RsType) -> InstrResult<Option<(Expr, RsType)>> {
    let Some(bound) = sim.state.type_var_bounds.get(&ty_text(env, obj_ty)) else {
        return Ok(None);
    };
    let src = sim::exprs::clone_moved_var(obj.clone(), obj_ty)?;
    let as_object = sim::exprs::into_call(sim::exprs::object_type()?, src)?;
    let view = sim::exprs::into_call(ir_ty(env, bound)?, as_object)?;
    Ok(Some((view, bound.clone())))
}

/// 有类上界的类型变量接收者 → 上界类型视图（`type_var_receiver_bound_view`）；
/// 无类上界时原样返回
pub fn type_var_receiver_bound_view(env: &InstrEnv, sim: &StackSim, obj: Expr, obj_ty: RsType) -> InstrResult<(Expr, RsType)> {
    Ok(bound_view(env, sim, &obj, &obj_ty)?.unwrap_or((obj, obj_ty)))
}

/// 被调方法 `call`（owner 为查找起点类 binary 名）的真实形参类型（`_lookup_method_sig_params`）；
/// 查不到 / 签名无效 → None
pub fn lookup_method_sig_params(
    env: &InstrEnv,
    call: &CallRef,
    caller_tparams: &[String],
    recv: RecvView,
) -> InstrResult<Option<Vec<Option<RsType>>>> {
    params::lookup(env, call, caller_tparams, recv)
}

/// callee 的该重载是否由 runtime 手写 `_impl.rs` 伴生文件提供（`_handwritten_boundary_method`）
pub fn handwritten_boundary_method(ctx: &InstrCtx, cls_bin: &str, mname: &str, full_desc: &str) -> InstrResult<bool> {
    if mname == "<init>" {
        return Ok(false);
    }
    let rust = naming::mangle_if_overloaded(ctx, cls_bin, mname, Some(full_desc))?;
    Ok(ctx.facts.impl_has_fn(cls_bin, &rust))
}

fn zip_map(params: &[String], args: Vec<RsType>) -> TargMap {
    params.iter().cloned().zip(args).collect()
}

/// 类接收者 → 接口声明者的形参映射（`_iface_view_targ_map`）
pub fn iface_view_targ_map(ctx: &InstrCtx, recv_ty: &RsType, iface: &ClassInfo) -> Option<TargMap> {
    let recv_ci = ctx.reg().get(&hierarchy::type_binary(ctx, recv_ty)?)?;
    let owner_params = ctx.ty.effective_class_type_params(iface);
    for (if_bin, if_args) in ctx.ty.implemented_interface_views(recv_ci) {
        if if_bin != iface.name() {
            continue;
        }
        let recv_params = ctx.ty.effective_class_type_params(recv_ci);
        let recv_args = recv_ty.type_args();
        let if_args: Vec<RsType> = if !recv_args.is_empty() && recv_args.len() == recv_params.len() {
            let amap = zip_map(&recv_params, recv_args.to_vec());
            if_args.iter().map(|a| a.substitute(&|n| amap.get(n).cloned())).collect()
        } else {
            if_args
        };
        return (if_args.len() == owner_params.len()).then(|| zip_map(&owner_params, if_args));
    }
    None
}

/// 接收者静态类型视角下声明类 `owner_bin` 的类型形参 → 实参映射（`receiver_type_arg_map`）
pub fn receiver_type_arg_map(ctx: &InstrCtx, recv_ty: &RsType, owner_bin: &str) -> Option<TargMap> {
    if ctx.reg().is_empty() || owner_bin.is_empty() {
        return None;
    }
    let recv_ci = ctx.reg().get(&hierarchy::type_binary(ctx, recv_ty)?)?;
    let owner_ci = ctx.reg().get(&resolve_cls(ctx, owner_bin)?)?;
    let owner_params = ctx.ty.effective_class_type_params(owner_ci);
    if owner_params.is_empty() {
        return None;
    }
    let recv_args = recv_ty.type_args();
    if recv_ci.name() == owner_ci.name() {
        return (recv_args.len() == owner_params.len()).then(|| zip_map(&owner_params, recv_args.to_vec()));
    }
    if recv_args.len() != ctx.ty.effective_class_type_params(recv_ci).len() {
        return None;
    }
    let (_, args) = ctx.ty.ancestor_type_args(recv_ci, Some(recv_args)).into_iter().find(|(b, _)| b == owner_ci.name())?;
    (args.len() == owner_params.len()).then(|| zip_map(&owner_params, args))
}

/// downcast / checkcast 目标类型合法性（`_downcast_target_valid`）：类型文本的全部标识符须为
/// 具体类、类级类型形参、基本类型或内建容器名
pub fn downcast_target_valid(env: &InstrEnv, sim: &StackSim, expected: &RsType) -> bool {
    let text = ty_text(env, expected);
    let names = ident_tokens(&text);
    if names.is_empty() {
        return false;
    }
    const BUILTIN: [&str; 5] = ["Rc", "__Shared", "Vec", "RefCell", "Option"];
    const PRIMS: [&str; 12] = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize"];
    let anchors = [ir::anchors::OBJECT, ir::anchors::STRING, ir::anchors::CLASS];
    names.iter().all(|n| {
        sim.cfg.class_type_params.iter().any(|p| p == n)
            || anchors.contains(n)
            || BUILTIN.contains(n)
            || PRIMS.contains(n)
            || is_concrete_short(&env.ctx, n)
    })
}

/// 注册表内由字节码生成的具体（非接口）类（`_is_generated_concrete_class`）
pub fn is_generated_concrete_class(env: &InstrEnv, sim: &StackSim, actual: &RsType) -> bool {
    let text = ty_text(env, actual);
    let head = text.split('<').next().unwrap_or("");
    let head = head.rsplit("::").next().unwrap_or("").trim();
    if head.is_empty() || sim.cfg.class_type_params.iter().any(|p| p == head) {
        return false;
    }
    is_concrete_short(&env.ctx, head)
}

/// 被调方法泛型签名解析出的真实返回类型（`_lookup_method_sig_ret`）；None → 降级擦除类型
pub fn lookup_method_sig_ret(
    env: &InstrEnv,
    call: &CallRef,
    caller_class: Option<&str>,
    caller_tparams: &[String],
    receiver_type: Option<&RsType>,
) -> InstrResult<Option<RsType>> {
    ret::lookup(env, call, caller_class, caller_tparams, receiver_type)
}

/// 被调方法（沿超类 / 接口链）泛型签名返回类型是否为裸类型变量（`_erased_ret_is_type_var`）
pub fn erased_ret_is_type_var(ctx: &InstrCtx, cls: &str, mname: &str, full_desc: &str) -> bool {
    if cls.is_empty() || ctx.reg().is_empty() {
        return false;
    }
    let start = if cls.contains('/') { cls.to_string() } else { resolve_cls(ctx, cls).unwrap_or_else(|| cls.to_string()) };
    let mut queue = VecDeque::from([start]);
    let mut seen = BTreeSet::new();
    while let Some(cur) = queue.pop_front() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        let Some(ci) = ctx.reg().get(&cur) else {
            continue;
        };
        if let Some(m) = ci.methods().iter().find(|m| m.name == mname && m.desc == full_desc) {
            let sig = ty::registry::method_signature(m);
            return sig.rsplit_once(')').is_some_and(|(_, r)| r.starts_with('T'));
        }
        if !ci.super_class().is_empty() {
            queue.push_back(ci.super_class().to_string());
        }
        queue.extend(ci.interfaces().iter().cloned());
    }
    false
}

/// 静态类型 `actual` 沿超类链到祖先 `ancestor_bin` 的精确实例化（`_exact_ancestor_type`）
pub fn exact_ancestor_type(ctx: &InstrCtx, actual: &RsType, ancestor_bin: &str) -> Option<RsType> {
    let JvmType::Class { binary, .. } = jvm(ctx, actual) else {
        return None;
    };
    let ci = ctx.reg().get(&binary)?;
    let (anc, args) = ctx.ty.ancestor_type_args(ci, Some(actual.type_args())).into_iter().find(|(b, _)| b == ancestor_bin)?;
    Some(RsType::class(anc, args))
}

/// 实参 → 形参类型的强转（`coerce_arg_node`）
pub fn coerce_arg(env: &InstrEnv, sim: &StackSim, log: &mut InstrLog, e: Expr, actual: &RsType, expected: &RsType) -> InstrResult<Expr> {
    coerce_arg::coerce(env, sim, log, e, actual, expected)
}

#[cfg(test)]
mod tests {
    use super::ident_tokens;

    #[test]
    fn tokens() {
        assert_eq!(ident_tokens("HashMap_Node<K, JArray<i32>>"), vec!["HashMap_Node", "K", "JArray", "i32"]);
        assert_eq!(ident_tokens("()"), Vec::<&str>::new());
    }
}

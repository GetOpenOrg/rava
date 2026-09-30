//! 实参 / 接收者弹出（`_pop_receiver_and_args`）与特殊接收者早路径
//! （`_try_early_receiver_paths` / `_emit_object_direct_call`）。

use ir::{Expr, Raw};
use sim::StackSim;
use ty::RsType;

use super::{is_prim, let_push, raw, Site, O};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::{sig, CallRef};
use crate::log::{Audit, InstrLog};

/// 弹出并强转全部实参与接收者；构造结果直接作接收者时，待推断的 `_` 类型实参按擦除
/// 闭合为 Object（接收者位置不提供推断上下文，E0283）
pub(super) fn pop_receiver_and_args(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    sig_params: Option<Vec<Option<RsType>>>,
) -> InstrResult<Site> {
    let n = call.params.len();
    let mut arg_nodes = Vec::with_capacity(n);
    for (idx, param_jvm) in call.params.iter().enumerate().rev() {
        let e = sim.pop()?;
        let expected = match sig_params.as_ref().and_then(|s| s.get(idx)) {
            Some(Some(t)) => t.clone(),
            _ => env.ctx.ty.jvm_to_rust(param_jvm),
        };
        arg_nodes.push(sig::coerce_arg(env, sim, log, e.expr, &e.ty, &expected)?);
    }
    arg_nodes.reverse();
    let args = arg_nodes.iter().map(|a| text(env, a)).collect();
    let obj = sim.pop()?;
    let mut site = Site {
        args,
        arg_nodes,
        obj_e: text(env, &obj.expr),
        obj_ty: obj.ty,
        obj_node: obj.expr,
        sig_params,
    };
    close_constructed_receiver(env, &mut site);
    Ok(site)
}

/// `new Foo<>(..).m()`：构造器 turbofish 里待推断的 `_` 永远无解 → 按擦除落为 Object。
/// 已绑定到局部变量的构造结果不在此列（后续赋值 / 传参仍可提供推断）
fn close_constructed_receiver(env: &InstrEnv, site: &mut Site) {
    let RsType::Class { binary, args } = &site.obj_ty else {
        return;
    };
    let head = env.ctx.short(binary);
    let word = !head.is_empty() && head.chars().all(|c| c.is_alphanumeric() || c == '_');
    if !word || args.is_empty() || !args.iter().all(|a| matches!(a, RsType::Param(p) if p == sim::INFER_PARAM)) {
        return;
    }
    let open = format!("{head}::<{}>::", vec!["_"; args.len()].join(", "));
    let Some(rest) = site.obj_e.strip_prefix(&open) else {
        return;
    };
    let erased = vec![O; args.len()].join(", ");
    site.obj_e = format!("{head}::<{erased}>::{rest}");
    site.obj_ty = RsType::class(binary.clone(), vec![RsType::Object; args.len()]);
    site.obj_node = Expr::Raw(Raw(site.obj_e.clone()));
}

/// 结果形态：void → `{call};`，否则 `let v: R = {call};` 入栈
fn emit_value(env: &InstrEnv, sim: &mut StackSim, call: &str, ret: RsType) -> InstrResult<()> {
    if ret == RsType::Unit {
        raw(sim, format!("{call};"));
        Ok(())
    } else {
        let_push(env, sim, "_t", call, ret)
    }
}

/// 四类特殊接收者路径（装箱类型变量的 Object 手写直调 / 基本类型 equals 的 `==` /
/// 基本类型接收者的根类方法 / 数组 getClass），命中即发射并返回 true
pub(super) fn try_early_receiver_paths(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    obj_is_typevar: bool,
    call: &CallRef,
    site: &Site,
) -> InstrResult<bool> {
    let mname = call.name.as_str();
    let obj_e = &site.obj_e;
    if obj_is_typevar && matches!(mname, "equals" | "hashCode" | "toString") {
        // [equiv-audit] identity-hash：类型变量接收者装箱后直调 Object 手写实现
        if mname == "hashCode" {
            log.audit(Audit::IdentityHash);
        }
        let ret = env.ctx.ty.jvm_to_rust(&call.ret);
        emit_value(env, sim, &format!("{obj_e}.{mname}({})?", site.arg_str()), ret)?;
        return Ok(true);
    }
    // 基本类型 .equals(x) → `==` 比较（基本类型无 equals 方法）
    if mname == "equals" && site.args.len() == 1 && is_prim(&site.obj_ty) {
        let raw_arg = site.args[0].strip_suffix(".into()").unwrap_or(&site.args[0]);
        let_push(env, sim, "_t", &format!("({obj_e} == {raw_arg})"), RsType::Prim(ty::Prim::Bool))?;
        return Ok(true);
    }
    // 基本类型接收者调用根类声明的方法：装箱为 Object 后走根 vtable
    if is_prim(&site.obj_ty) && env.ctx.facts.root_virtual.contains(&(call.name.clone(), call.param_desc().to_string())) {
        let ret = env.ctx.ty.jvm_to_rust(&call.ret);
        let c = format!(
            "{O}::from_any({obj_e}).{}({})?",
            ty::ident::safe_ident(mname),
            site.arg_str()
        );
        emit_value(env, sim, &c, ret)?;
        return Ok(true);
    }
    // 数组.getClass() → `Object::from(数组).0.getClass()`：数组类由创建时的元素类型决定
    // （JLS §10.8），协变视图经 JArray 的 getClass 委托源数组
    if mname == "getClass" && matches!(site.obj_ty, RsType::Array(_)) {
        // [equiv-audit] class-literal：数组 getClass 的发射早路径
        log.audit(Audit::ClassLiteral);
        let recv = obj_e.strip_prefix('&').unwrap_or(obj_e);
        let class_ty = RsType::class(ty::consts::CLASS.to_string(), Vec::new());
        let value = format!("{O}::from(Clone::clone(&{recv})).0.getClass()?");
        let_push(env, sim, "_t", &value, class_ty)?;
        return Ok(true);
    }
    Ok(false)
}

/// bare Object 接收者 + 根类方法：仅 void / 基本类型返回值经包装器直调
/// （手写实现返回 Object 而非具体类型，引用返回不能直接赋值）
pub(super) fn emit_object_direct_call(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    site: &Site,
    rust_mname: &str,
    rust_ret: &RsType,
) -> InstrResult<bool> {
    if !is_prim(rust_ret) {
        return Ok(false);
    }
    // [equiv-audit] identity-hash：bare Object 接收者的 hashCode 直调
    if rust_mname == "hashCode" {
        log.audit(Audit::IdentityHash);
    }
    // Python 在两个分支前都取了 fresh 名（计数器同步）
    let v = sim.fresh("_t")?;
    let c = format!("{}.{rust_mname}({})?", site.obj_e, site.arg_str());
    if *rust_ret == RsType::Unit {
        raw(sim, format!("{c};"));
    } else {
        raw(sim, format!("let {v}: {} = {c};", ty_text(env, rust_ret)));
        sim.push(Expr::Var(v), rust_ret.clone());
    }
    Ok(true)
}

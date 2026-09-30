//! invokevirtual / invokeinterface（← `instr/invoke_virtual.py`）。
//!
//! 模块分工：
//! - [`args`]：实参 / 接收者弹出与四类特殊接收者早路径；
//! - [`bare`]：bare `Object` 接收者的多态分派（接口载体 / 根 vtable / 类 vtable）；
//! - [`vtable`]：类 vtable 视图分派与子类继承成员登记；
//! - [`direct`]：非 bare 接收者的签名解析与调用结果记录；
//! - [`cs`]：@CallerSensitive 包装。
//!
//! Python 的字符串管线（实参串、接收者串、`RawStmt` f-string）在这里保持同文本：
//! 语句以 [`ir::Raw`] 承载 Python 同形文本，入栈值为 `Var` + 结构化 [`RsType`]。

mod args;
mod bare;
mod cs;
mod direct;
mod vtable;

use ir::{Expr, Raw, Stmt};
use sim::StackSim;
use ty::{JvmType, RsType};

use crate::build::ty_text;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::{recv, CallRef};
use crate::log::{Audit, InstrLog};
use crate::naming;

use cs::CsCtx;

/// 根类的 Rust 锚点名
const O: &str = ir::anchors::OBJECT;

/// 弹栈后的调用点：实参（渲染文本 + 节点）、接收者（文本 + 类型 + 原节点）
struct Site {
    args: Vec<String>,
    arg_nodes: Vec<Expr>,
    obj_e: String,
    obj_ty: RsType,
    obj_node: Expr,
    sig_params: Option<Vec<Option<RsType>>>,
}

impl Site {
    fn arg_str(&self) -> String {
        self.args.join(", ")
    }
}

/// `Raw` 语句（Python `RawStmt(f"..")` 同文本）
fn raw(sim: &mut StackSim, text: String) {
    sim.emit(Stmt::Raw(Raw(text)));
}

/// `let {v}: {ty} = {value};`，压 `Var(v)`（fresh 前缀缺省 `_t`）
fn let_push(env: &InstrEnv, sim: &mut StackSim, prefix: &str, value: &str, t: RsType) -> InstrResult<()> {
    let v = sim.fresh(prefix)?;
    raw(sim, format!("let {v}: {} = {value};", ty_text(env, &t)));
    sim.push(Expr::Var(v), t);
    Ok(())
}

/// Python `PRIMITIVE_RUST_TYPES`（含 `()`）
fn is_prim(t: &RsType) -> bool {
    matches!(t, RsType::Prim(_) | RsType::Unit)
}

/// 类型文本为 `Object`（Python 串比较口径）
fn is_object(env: &InstrEnv, t: &RsType) -> bool {
    ty_text(env, t) == O
}

/// 类型文本相等（Python 串比较口径）
fn same_text(env: &InstrEnv, a: &RsType, b: &RsType) -> bool {
    ty_text(env, a) == ty_text(env, b)
}

/// 类型的擦除头名（`rust_head_name(from_rust_type(t).erasure())`；不按作用域形参解读）
fn erased_head(env: &InstrEnv, t: &RsType) -> String {
    let jt = env.ctx.ty.from_rs_type(t, &Default::default());
    env.ctx.ty.rust_head_name(&jt.erasure())
}

/// 类型文本 `{Short}<Object, ..>`：类的有效类型形参全取 Object（无形参 → 裸短名）
fn erased_inst(env: &InstrEnv, binary: &str) -> RsType {
    let n = env.ctx.reg().get(binary).map_or(0, |ci| env.ctx.ty.effective_class_type_params(ci).len());
    RsType::class(binary.to_string(), vec![RsType::Object; n])
}

/// 调用目标名：`safe_ident(mangle_if_overloaded(cls, mname, desc))`
fn member_name(env: &InstrEnv, cls: &str, call: &CallRef) -> InstrResult<String> {
    Ok(ty::ident::safe_ident(&naming::mangle_if_overloaded(&env.ctx, cls, &call.name, Some(&call.desc))?))
}

/// invokevirtual / invokeinterface 的发射（`_gen_invokevirtual`）
pub fn gen_invokevirtual(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    // @CallerSensitive 虚调用：调用处类经 __caller_sensitive 显式传入（与 invokestatic 同一机制）
    let cs = CsCtx::new(call, env.ctx.class_name);
    // [equiv-audit] 按常量池方法名匹配的近似等价形态，只计数不改发射
    if call.name == "intern" {
        log.audit(Audit::InternIdentity);
    } else if call.name == "getClass" {
        log.audit(Audit::ClassLiteral);
    }
    let sig_params = recv::resolve_virtual_sig_params(env, sim, call)?;
    let mut site = args::pop_receiver_and_args(env, sim, log, call, sig_params)?;
    // 泛型参数接收者：inherent 方法在类型参数上不可见（E0599）→ 装箱为 Object
    let obj_is_typevar = sim.cfg.class_type_params.contains(&ty_text(env, &site.obj_ty));
    if obj_is_typevar {
        site.obj_e = format!("Into::<{O}>::into(Clone::clone(&{}))", site.obj_e);
        site.obj_ty = RsType::Object;
    }
    if args::try_early_receiver_paths(env, sim, log, obj_is_typevar, call, &site)? {
        return Ok(());
    }
    if !env.ctx.reg().is_empty() && private_iface_call(env, sim, call, &site)? {
        return Ok(());
    }
    // 接收方是手写根类：API 名面固定，仅根类同名重载按描述符后缀取名；
    // 否则优先按接收者实际类型 mangle（接口视角可能漏判重载）
    let obj_base = erased_head(env, &site.obj_ty);
    let rust_mname = if obj_base == env.ctx.short(ty::consts::OBJECT) {
        member_name(env, ty::consts::OBJECT, call)?
    } else if !obj_base.is_empty() && obj_base != "()" {
        member_name(env, &obj_base, call)?
    } else {
        member_name(env, &call.owner, call)?
    };
    let rust_ret = env.ctx.ty.jvm_to_rust(&call.ret);
    // E0599 防护：Object 接收者 → bare 多态分派
    let obj_is_bare = is_object(env, &site.obj_ty);
    let has_reg = !env.ctx.reg().is_empty();
    if obj_is_bare && has_reg && env.ctx.short(&call.owner) == O
        && args::emit_object_direct_call(env, sim, log, &site, &rust_mname, &rust_ret)?
    {
        return Ok(());
    }
    if obj_is_bare && has_reg {
        return bare::dispatch_bare_object(env, sim, log, call, &site, &rust_mname, &rust_ret);
    }
    let d = direct::resolve_direct_call_sig(env, sim, log, call, rust_ret, &obj_base, &site)?;
    direct::emit_call_result(env, sim, &cs, call, &site, obj_is_bare, &rust_mname, d)
}

/// 接口私有实例方法（Java 9+）：直接执行接口自身实现（非契约成员，不进 vtable）。
/// 接收者静态类型即接口载体时直调，否则经载体视图转换。命中返回 true
fn private_iface_call(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, site: &Site) -> InstrResult<bool> {
    let Some(pv_bin) = crate::owner::private_interface_method_target(env.ctx.reg(), &call.owner, &call.name, &call.desc) else {
        return Ok(false);
    };
    let pv_mname = member_name(env, &pv_bin, call)?;
    let pv_view = ty_text(env, &erased_inst(env, &pv_bin));
    let rust_ret = env.ctx.ty.jvm_to_rust(&call.ret);
    let obj_e = &site.obj_e;
    let recv_erasure = env.ctx.ty.from_rs_type(&site.obj_ty, &Default::default()).erasure();
    let recv = if recv_erasure == JvmType::class_of(&pv_bin, env.ctx.reg()) {
        obj_e.clone()
    } else if obj_e == "this" {
        format!("Into::<{pv_view}>::into(Clone::clone(this))")
    } else if let Some(r) = obj_e.strip_prefix('&') {
        format!("Into::<{pv_view}>::into(Clone::clone({r}))")
    } else {
        format!("Into::<{pv_view}>::into(Clone::clone(&{obj_e}))")
    };
    let pv_call = format!("{recv}.{pv_mname}({})?", site.arg_str());
    if rust_ret == RsType::Unit {
        raw(sim, format!("{pv_call};"));
    } else {
        let_push(env, sim, "_t", &pv_call, rust_ret)?;
    }
    Ok(true)
}

//! 被调方法签名解析（← `instr/invoke_sig.py`）。
//!
//! Python 以 Rust 类型串为形参 / 返回类型的载体；这里一律为 [`RsType`]，类以 binary 名定位。
//! 形参列表中的 `None` 表示该位置降级到描述符擦除类型（`jvm_to_rust(descriptor)`）。

use std::collections::BTreeMap;

use ir::Expr;
use sim::StackSim;
use ty::{ClassInfo, RsType};

use crate::ctx::InstrCtx;
use crate::env::InstrEnv;
use crate::error::{unported, InstrResult};
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

/// 有类上界的类型变量接收者 → 上界类型视图（`type_var_receiver_bound_view`）；
/// 无类上界时原样返回
pub fn type_var_receiver_bound_view(env: &InstrEnv, sim: &StackSim, obj: Expr, obj_ty: RsType) -> InstrResult<(Expr, RsType)> {
    let _ = (env, sim);
    let _ = (obj, obj_ty);
    unported("type_var_receiver_bound_view")
}

/// 被调方法 `call`（owner 为查找起点类 binary 名） 的真实形参类型（`_lookup_method_sig_params`）；
/// 查不到 / 签名无效 → None
pub fn lookup_method_sig_params(
    env: &InstrEnv,
    call: &CallRef,
    caller_tparams: &[String],
    recv: RecvView,
) -> InstrResult<Option<Vec<Option<RsType>>>> {
    let _ = (env, call, caller_tparams, recv);
    unported("lookup_method_sig_params")
}

/// callee 的该重载是否由 runtime 手写 `_impl.rs` 伴生文件提供（`_handwritten_boundary_method`）
pub fn handwritten_boundary_method(ctx: &InstrCtx, cls_bin: &str, mname: &str, full_desc: &str) -> InstrResult<bool> {
    if mname == "<init>" {
        return Ok(false);
    }
    let rust = naming::mangle_if_overloaded(ctx, cls_bin, mname, Some(full_desc))?;
    Ok(ctx.facts.impl_has_fn(cls_bin, &rust))
}

/// 类接收者 → 接口声明者的形参映射（`_iface_view_targ_map`）
pub fn iface_view_targ_map(ctx: &InstrCtx, recv_ty: &RsType, iface: &ClassInfo) -> Option<TargMap> {
    let _ = (ctx, recv_ty, iface);
    None
}

/// 接收者静态类型视角下声明类 `owner_bin` 的类型形参 → 实参映射（`receiver_type_arg_map`）
pub fn receiver_type_arg_map(ctx: &InstrCtx, recv_ty: &RsType, owner_bin: &str) -> Option<TargMap> {
    let _ = (ctx, recv_ty, owner_bin);
    None
}

/// downcast / checkcast 目标类型合法性（`_downcast_target_valid`）
pub fn downcast_target_valid(env: &InstrEnv, sim: &StackSim, expected: &RsType) -> bool {
    let _ = (env, sim, expected);
    false
}

/// 注册表内由字节码生成的具体（非接口）类（`_is_generated_concrete_class`）
pub fn is_generated_concrete_class(env: &InstrEnv, sim: &StackSim, actual: &RsType) -> bool {
    let _ = (env, sim, actual);
    false
}

/// 被调方法泛型签名解析出的真实返回类型（`_lookup_method_sig_ret`）；None → 降级擦除类型
pub fn lookup_method_sig_ret(
    env: &InstrEnv,
    call: &CallRef,
    caller_class: Option<&str>,
    caller_tparams: &[String],
    receiver_type: Option<&RsType>,
) -> InstrResult<Option<RsType>> {
    let _ = (env, call, caller_class, caller_tparams, receiver_type);
    unported("lookup_method_sig_ret")
}

/// 被调方法（沿超类 / 接口链）泛型签名返回类型是否为裸类型变量（`_erased_ret_is_type_var`）
pub fn erased_ret_is_type_var(ctx: &InstrCtx, cls: &str, mname: &str, full_desc: &str) -> bool {
    let _ = (ctx, cls, mname, full_desc);
    false
}

/// 实参 → 形参类型的强转（`coerce_arg_node`）
pub fn coerce_arg(env: &InstrEnv, sim: &StackSim, log: &mut InstrLog, e: Expr, actual: &RsType, expected: &RsType) -> InstrResult<Expr> {
    let _ = (env, sim, log, e, actual, expected);
    unported("coerce_arg")
}

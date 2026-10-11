//! invokespecial（← `instr/invoke.py` 的 `_gen_invokespecial`）：非构造器（`super.m()` /
//! `Iface.super.m()` / 私有方法自调用）在本文件，构造器调用见 [`ctor`]。
//!
//! 调用形态沿用 Python 的文本（[`ir::Raw`]）：base 自由函数调用 `Owner__m_base(..)?` 的
//! 首参（`this` / `&*(obj).vtable`）由宏重写约定决定，结构化节点无对应形态。

mod ctor;

use ir::{Expr, PathSegment, Stmt, Type};
use sim::StackSim;
use ty::{ClassInfo, RsType};

use crate::build::{seg, seg_g, text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::sig::{self, CallBind, RecvView, TargMap};
use crate::invoke::CallRef;
use crate::log::InstrLog;
use crate::naming::mangle_if_overloaded;
use crate::owner::{interface_special_member_name, resolve_interface_special_target, resolve_special_method_owner};

/// invokespecial 翻译入口
pub fn gen_invokespecial(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    if call.name == "<init>" {
        ctor::gen_ctor(env, sim, log, call)
    } else {
        gen_super_method(env, sim, log, call)
    }
}

/// 类短名（可能是限定改名 `a::B`）→ 路径段，泛型实参挂在末段
pub(crate) fn class_segs(short: &str, generics: Vec<Type>) -> InstrResult<Vec<PathSegment>> {
    let parts: Vec<&str> = short.split("::").collect();
    let last = parts.len().saturating_sub(1);
    let mut segs = Vec::with_capacity(parts.len() + 1);
    let mut generics = Some(generics);
    for (i, p) in parts.iter().enumerate() {
        segs.push(if i == last { seg_g(p, generics.take().unwrap_or_default())? } else { seg(p)? });
    }
    Ok(segs)
}

/// `<A, B>` 实参表文本（无括号，逗号分隔）
pub(crate) fn join_types(env: &InstrEnv, ts: &[RsType]) -> String {
    ts.iter().map(|t| ty_text(env, t)).collect::<Vec<_>>().join(", ")
}

/// 本类视角的自身类型 `Self<T..>`（有效类型形参原样）
fn self_type(env: &InstrEnv, ci: &ClassInfo) -> RsType {
    let tps = env.ctx.ty.effective_class_type_params(ci);
    RsType::class(ci.name(), tps.iter().map(|p| RsType::Param(p.clone())).collect())
}

/// 基类调用的被调方是 `Owner__m_base` 自由函数：sig 查询在 `this` 语境把接口载体形参降级
/// None 的位置，按「声明签名的裸类型变量形参 + 祖先链实例化」重建期望类型
fn rebuild_base_params(env: &InstrEnv, call: &CallRef, self_ci: &ClassInfo, self_ty: &RsType, sp_owner: &str, sig_params: &mut [Option<RsType>]) {
    let ctx = &env.ctx;
    let Some(owner_ci) = ctx.reg().get(sp_owner) else {
        return;
    };
    let owner_eff = ctx.ty.effective_class_type_params(owner_ci);
    let owner_short = ctx.short(sp_owner);
    let inst = ctx.ty.ancestor_vtable_args_by_short(self_ci, self_ty).remove(&owner_short).unwrap_or_default();
    if inst.is_empty() || inst.len() != owner_eff.len() {
        return;
    }
    let var_inst: TargMap = owner_eff.iter().cloned().zip(inst).collect();
    let Some(decl_m) = owner_ci.methods().iter().find(|m| m.name == call.name && m.desc == call.desc) else {
        return;
    };
    let decl_types = ctx.ty.method_sig_types(owner_ci, decl_m, &owner_eff).params;
    if decl_types.is_empty() || decl_types.len() != sig_params.len() {
        return;
    }
    for (sp, dt) in sig_params.iter_mut().zip(&decl_types) {
        if sp.is_none() {
            if let RsType::Param(n) = dt {
                if let Some(t) = var_inst.get(n) {
                    *sp = Some(t.clone());
                }
            }
        }
    }
}

/// 弹出实参并按形参类型强转，返回渲染文本
fn pop_args_text(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef, sig_params: Option<&[Option<RsType>]>) -> InstrResult<Vec<String>> {
    let n = call.params.len();
    let mut args = Vec::with_capacity(n);
    for pi in (0..n).rev() {
        let e = sim.pop()?;
        let expected = match sig_params.and_then(|sp| sp.get(pi)).cloned().flatten() {
            Some(t) => t,
            None => env.ctx.ty.jvm_to_rust(&call.params[pi]),
        };
        let node = sig::coerce_arg(env, sim, log, e.expr, &e.ty, &expected)?;
        args.push(text(env, &node));
    }
    args.reverse();
    Ok(args)
}

/// 调用结果落 `let _tN`（`()` 返回 → 语句）：返回类型按声明者 `sig_owner` 的签名在本类
/// 自身类型（`this`）视角下实例化，与 direct 调用共用 [`sig::bind_call_result`] 对齐规则
fn emit_call_result(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, sig_owner: &str, call_text: &str) -> InstrResult<()> {
    let ctx = &env.ctx;
    let rust_ret = ctx.ty.jvm_to_rust(&call.ret);
    if matches!(rust_ret, RsType::Unit) {
        sim.emit(Stmt::raw(format!("{call_text};")))?;
        return Ok(());
    }
    let self_ty = ctx.reg().get(ctx.class_name).map(|ci| self_type(env, ci));
    let sig_call = CallRef::with(sig_owner, &call.name, &call.desc);
    let sig_ret = sig::lookup_method_sig_ret(env, &sig_call, Some(ctx.class_name), &sim.cfg.class_type_params, self_ty.as_ref())?;
    let erased_tv = || sig::erased_ret_is_type_var(ctx, &call.owner, &call.name, &call.desc);
    let v = sim.fresh("_t")?;
    let o = ir::anchors::OBJECT;
    let r = ty_text(env, &rust_ret);
    let (stmt, pushed) = match sig::bind_call_result(env, &rust_ret, sig_ret, erased_tv) {
        CallBind::Boxed(s) => {
            let boxed = crate::coerce::to_object(env, Expr::raw(call_text.to_string()), &s, true)?;
            (format!("let {v} = {};", text(env, &boxed)), rust_ret)
        }
        CallBind::Precise(s) => (format!("let {v} = {call_text};"), s),
        CallBind::Carrier => (
            format!("let {v}: {r} = <{r} as ::std::convert::From<{o}>>::from(::std::convert::Into::<{o}>::into({call_text}));"),
            rust_ret,
        ),
        CallBind::TypeVar => (format!("let {v} = ::std::convert::Into::<{o}>::into({call_text});"), rust_ret),
        CallBind::Erased => (format!("let {v} = {call_text};"), rust_ret),
    };
    sim.emit(Stmt::raw(stmt))?;
    sim.push(Expr::Var(v), pushed);
    Ok(())
}

/// `super.m(..)`：精确路由到声明类的 `Owner__m_base` 自由函数（绕过虚分派，
/// 否则 `this.m()` 对被覆盖方法无限递归）；`Iface.super.m()` / 接口私有方法 → 成员
/// `Iface_super_m`（接口方法体展开在调用者所在类上）
fn gen_super_method(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    let ctx = &env.ctx;
    let reg = ctx.reg();
    let self_ci = reg.get(ctx.class_name);
    let self_ty = self_ci.map(|ci| self_type(env, ci));
    let recv_map = self_ty.as_ref().and_then(|t| sig::receiver_type_arg_map(ctx, t, &call.owner));
    let recv = RecvView { targ_map: recv_map.as_ref(), is_this: true, ty: self_ty.as_ref() };
    let mut sig_params = sig::lookup_method_sig_params(env, call, &sim.cfg.class_type_params, recv)?;
    let sp_owner = resolve_special_method_owner(reg, &call.owner, &call.name, &call.desc);
    let owner_short = ctx.short(&sp_owner);
    let self_short = ctx.short(ctx.class_name);
    if let (Some(sp), Some(ci), Some(st)) = (sig_params.as_mut(), self_ci, self_ty.as_ref()) {
        if sp.iter().any(Option::is_none) && owner_short != self_short {
            rebuild_base_params(env, call, ci, st, &sp_owner, sp);
        }
    }
    let args = pop_args_text(env, sim, log, call, sig_params.as_deref())?;
    let obj = sim.pop()?;
    let obj_e = text(env, &obj.expr);
    if let Some(iface_owner) = resolve_interface_special_target(reg, &call.owner, &call.name, &call.desc) {
        let member = ty::ident::safe_ident(&interface_special_member_name(ctx, &iface_owner, &call.name, &call.desc));
        return emit_call_result(env, sim, call, &iface_owner, &format!("{obj_e}.{member}({})?", args.join(", ")));
    }
    let rust_m = ty::ident::safe_ident(&mangle_if_overloaded(ctx, &sp_owner, &call.name, Some(&call.desc))?);
    if ctx.hooks.slot_pruned(&sp_owner, &call.name, &call.desc) {
        return gen_slotless_special(env, sim, call, &sp_owner, &obj_e, &rust_m, &args);
    }
    let mut base_fn = format!("{owner_short}__{rust_m}_base");
    // base 函数的泛型形参 = 声明类的类型形参（接收者是 `&dyn Owner__VTable`，不参与泛型）；
    // 实参无处推断时（E0283）显式给出
    if let (Some(ci), Some(st)) = (self_ci, self_ty.as_ref()) {
        if owner_short != self_short {
            let targs = ctx.ty.ancestor_vtable_args_by_short(ci, st).remove(&owner_short).unwrap_or_default();
            if !targs.is_empty() {
                base_fn.push_str(&format!("::<{}>", join_types(env, &targs)));
            }
        } else if !sim.cfg.class_type_params.is_empty() {
            base_fn.push_str(&format!("::<{}>", sim.cfg.class_type_params.join(", ")));
        }
    }
    // 首参是 vtable 引用：宏把字面 this/self 接收者重写为 `this.__vt()`；其余按同一形态发射
    let recv_arg = if obj_e == "this" || obj_e == "self" { obj_e } else { format!("({obj_e}).__vt()") };
    let all: Vec<String> = std::iter::once(recv_arg).chain(args).collect();
    emit_call_result(env, sim, call, &sp_owner, &format!("{base_fn}({})?", all.join(", ")))
}

/// 不占 vtable 槽的落点（无 `Owner__m_base`）：本类 / private → 接收者 wrapper 直接调用；
/// 祖先 → 上转到声明者 wrapper 后直接调用（NonVirtual 方法不经分派，与 super 语义一致）
fn gen_slotless_special(
    env: &InstrEnv,
    sim: &mut StackSim,
    call: &CallRef,
    sp_owner: &str,
    obj_e: &str,
    rust_m: &str,
    args: &[String],
) -> InstrResult<()> {
    let ctx = &env.ctx;
    let reg = ctx.reg();
    let joined = args.join(", ");
    let (Some(self_ci), Some(owner_ci)) = (reg.get(ctx.class_name), reg.get(sp_owner)) else {
        return emit_call_result(env, sim, call, sp_owner, &format!("{obj_e}.{rust_m}({joined})?"));
    };
    if sp_owner == ctx.class_name {
        return emit_call_result(env, sim, call, sp_owner, &format!("{obj_e}.{rust_m}({joined})?"));
    }
    let self_ty = self_type(env, self_ci);
    let owner_short = ctx.short(sp_owner);
    let mut targs = ctx.ty.ancestor_vtable_args_by_short(self_ci, &self_ty).remove(&owner_short).unwrap_or_default();
    let n_owner = ctx.ty.effective_class_type_params(owner_ci).len();
    if targs.len() != n_owner {
        targs = vec![RsType::Object; n_owner];
    }
    let owner_ty = if targs.is_empty() { owner_short } else { format!("{owner_short}<{}>", join_types(env, &targs)) };
    let self_text = join_types(env, std::slice::from_ref(&self_ty));
    let text = format!(
        "<{owner_ty} as ::std::convert::From<{self_text}>>::from(::std::clone::Clone::clone({obj_e})).{rust_m}({joined})?"
    );
    emit_call_result(env, sim, call, sp_owner, &text)
}

//! 虚调用接收者解析（← `member_owner.py` 中依赖栈状态的部分：`_close_open_type_args`、
//! `_erase_slot_params`、`_resolve_virtual_sig_params`）。

use ir::{Expr, FnPath, Path, Raw, Stmt, Type};
use sim::{StackEntry, StackSim};
use ty::{ClassInfo, JvmType, RsType};

use super::sig::{bound_view, jvm, lookup_method_sig_params, receiver_type_arg_map, resolve_cls, RecvView};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::invoke::CallRef;
use crate::owner;

/// 类型节点中的待推断占位 `_` → 根类
fn close_type(t: &mut Type) -> InstrResult<()> {
    match t {
        Type::Infer => *t = sim::exprs::object_type()?,
        Type::Path(p) => close_path(p)?,
        Type::Ref { inner, .. } | Type::Slice(inner) => close_type(inner)?,
        Type::Tuple(ts) => ts.iter_mut().try_for_each(close_type)?,
        Type::Prim(_) => {}
    }
    Ok(())
}

fn close_path(p: &mut Path) -> InstrResult<()> {
    p.segments.iter_mut().flat_map(|s| s.generics.iter_mut()).try_for_each(close_type)
}

/// 构造表达式的 turbofish 闭合：Raw 文本前缀替换；`X::<..>::new(..)` / `X::<..>::new(..)?`
/// 的路径泛型改写。形态不符 → false
fn close_ctor(expr: &mut Expr, open_tf: &str, closed_tf: &str) -> InstrResult<bool> {
    match expr {
        Expr::Raw(Raw(code)) => match code.strip_prefix(open_tf) {
            Some(rest) => {
                *code = format!("{closed_tf}{rest}");
                Ok(true)
            }
            None => Ok(false),
        },
        Expr::Try(inner) => close_ctor(inner, open_tf, closed_tf),
        Expr::Call { func: FnPath::Path(p), .. } => {
            close_path(p)?;
            Ok(true)
        }
        _ => Ok(false),
    }
}

/// 替换栈位（新身份：Python 以新元组替换栈位）
fn replace_entry(sim: &mut StackSim, idx: usize, expr: Expr, ty: RsType) {
    let id = sim.state.next_id;
    sim.state.next_id += 1;
    if let Some(slot) = sim.state.stack.get_mut(idx) {
        *slot = StackEntry { expr, ty, id };
    }
}

/// 菱形构造结果（`X<_, A>`）作接收者：待推断实参取擦除，写回构造处 turbofish
/// （`_close_open_type_args`；`stack_idx` 为自栈底的下标）
pub fn close_open_type_args(env: &InstrEnv, sim: &mut StackSim, stack_idx: usize) -> InstrResult<()> {
    let Some(entry) = sim.state.stack.get(stack_idx).cloned() else {
        return Ok(());
    };
    let ty_s = ty_text(env, &entry.ty);
    let closed = entry.ty.substitute(&|n| (n == sim::INFER_PARAM).then_some(RsType::Object));
    let (Some((base, args)), true) = (ty_s.split_once('<'), closed != entry.ty) else {
        return Ok(());
    };
    let args = args.strip_suffix('>').unwrap_or(args);
    let open_tf = format!("{base}::<{args}>::");
    let closed_s = ty_text(env, &closed);
    let closed_args = closed_s.split_once('<').map_or("", |(_, a)| a.strip_suffix('>').unwrap_or(a));
    let closed_tf = format!("{base}::<{closed_args}>::");
    let Expr::Var(name) = &entry.expr else {
        // 构造表达式本身在栈上（`new X<>(..).m()` 未绑定临时变量）
        let mut expr = entry.expr.clone();
        if text(env, &expr).starts_with(&open_tf) && close_ctor(&mut expr, &open_tf, &closed_tf)? {
            replace_entry(sim, stack_idx, expr, closed);
        }
        return Ok(());
    };
    if sim.state.locals.values().any(|l| l.name == *name) {
        return Ok(()); // Java 局部变量：类型实参由声明 / 后续用法确定
    }
    let closed_ir = crate::build::ir_ty(env, &closed)?;
    for stmt in sim.state.stmts.iter_mut().rev() {
        let Stmt::Let(l) = stmt else {
            continue;
        };
        if l.name != *name {
            continue;
        }
        let Some(value) = l.value.as_mut() else {
            return Ok(());
        };
        let shaped = match value {
            Expr::Raw(_) => true,
            Expr::Try(inner) => matches!(**inner, Expr::Call { func: FnPath::Path(_), .. }),
            _ => false,
        };
        if !shaped || !text(env, value).starts_with(&open_tf) || !close_ctor(value, &open_tf, &closed_tf)? {
            return Ok(());
        }
        if l.ty.is_some() {
            l.ty = Some(closed_ir);
        }
        let expr = entry.expr.clone();
        replace_entry(sim, stack_idx, expr, closed);
        return Ok(());
    }
    Ok(())
}

/// 祖先类声明的虚方法：发射签名字面为 Object 的位对齐为 Object（`_erase_slot_params`）
pub fn erase_slot_params(env: &InstrEnv, owner: &ClassInfo, mname: &str, desc: &str, sig_params: Vec<Option<RsType>>) -> Vec<Option<RsType>> {
    let param_desc = owner::param_part(desc);
    let Some(m) = owner.methods().iter().find(|m| m.name == mname && !m.is_synthetic() && !m.is_static() && m.desc.starts_with(param_desc)) else {
        return sig_params;
    };
    let tps = env.ctx.ty.effective_class_type_params(owner);
    let slots = env.ctx.ty.emitted_method_sig_types(owner, m, &tps).params;
    if slots.len() != sig_params.len() {
        return sig_params;
    }
    slots.iter().zip(sig_params).map(|(s, sp)| if ty_text(env, s) == ir::anchors::OBJECT { Some(RsType::Object) } else { sp }).collect()
}

/// 接收者类链（超类方向）上是否有真实（非 synthetic）精确声明
fn has_real_decl(env: &InstrEnv, ci: &ClassInfo, call: &CallRef) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut walk = Some(ci);
    while let Some(w) = walk.filter(|w| seen.insert(w.name().to_string())) {
        if w.methods().iter().any(|m| !m.is_synthetic() && m.name == call.name && m.desc == call.desc) {
            return true;
        }
        walk = if w.super_class().is_empty() { None } else { env.ctx.reg().get(w.super_class()) };
    }
    false
}

/// 调用描述符在接收者类上只命中 synthetic bridge：形参类型按被桥接的真实方法确定
fn bridged_sig_params(env: &InstrEnv, sim: &StackSim, call: &CallRef, recv_bin: &str, recv: RecvView) -> InstrResult<Option<Vec<Option<RsType>>>> {
    let ctx = &env.ctx;
    let Some(recv_ci) = ctx.reg().get(recv_bin).filter(|c| !c.is_interface()) else {
        return Ok(None);
    };
    if has_real_decl(env, recv_ci, call) {
        return Ok(None);
    }
    let Some((target_bin, target_desc)) = owner::resolve_bridge_target(ctx.reg(), recv_ci, &call.name, &call.desc) else {
        return Ok(None);
    };
    if target_desc == call.desc {
        return Ok(None);
    }
    let bcall = CallRef::with(&target_bin, &call.name, &target_desc);
    if bcall.params.len() != call.params.len() {
        return Ok(None);
    }
    let targ = recv.ty.and_then(|t| receiver_type_arg_map(ctx, t, &target_bin));
    let view = RecvView { targ_map: targ.as_ref(), ..recv };
    let params = match lookup_method_sig_params(env, &bcall, &sim.cfg.class_type_params, view)? {
        Some(p) if !p.is_empty() => p,
        _ => bcall.params.iter().map(|p| Some(ctx.ty.jvm_to_rust(p))).collect(),
    };
    Ok(Some(match ctx.reg().get(&target_bin) {
        // 继承成员（祖先声明）：形参位按槽位擦除对齐
        Some(t) if !t.is_interface() && t.name() != recv_ci.name() => erase_slot_params(env, t, &call.name, &target_desc, params),
        _ => params,
    }))
}

/// 接收者解析与泛型实参映射（`_resolve_virtual_sig_params`）：会改写接收者栈位
/// （类型变量 → 上界视图、菱形 `_` 实参 → 擦除闭合）
pub fn resolve_virtual_sig_params(env: &InstrEnv, sim: &mut StackSim, call: &CallRef) -> InstrResult<Option<Vec<Option<RsType>>>> {
    let ctx = &env.ctx;
    let depth = call.params.len();
    let mut recv_ty: Option<RsType> = None;
    let mut is_this = false;
    if sim.state.stack.len() > depth {
        let pos = sim.state.stack.len() - depth - 1;
        let entry = sim.state.stack[pos].clone();
        is_this = text(env, &entry.expr) == "this";
        // 类型变量接收者 → 上界类型视图（与 getfield/putfield 同规则）
        if let Some((e, t)) = bound_view(env, sim, &entry.expr, &entry.ty)? {
            replace_entry(sim, pos, e, t);
        }
        close_open_type_args(env, sim, pos)?;
        recv_ty = Some(sim.state.stack[pos].ty.clone());
    }
    let targ_map = recv_ty.as_ref().and_then(|t| receiver_type_arg_map(ctx, t, &call.owner));
    let recv_base = recv_ty.as_ref().map_or(String::new(), |t| t.head_name(ctx.ty.names).unwrap_or_else(|| ty_text(env, t)));
    let recv_bin = match recv_ty.as_ref().map(|t| jvm(ctx, t)) {
        Some(JvmType::Class { binary, .. }) if ctx.reg().contains(&binary) => binary,
        _ => String::new(),
    };
    let caller_tparams = &sim.cfg.class_type_params;
    let this_in_class = is_this && ctx.reg().get(ctx.class_name).is_some_and(|c| !c.is_interface());
    let mut result = None;
    if !ctx.reg().is_empty() && !recv_base.is_empty() && recv_base != ctx.short(&call.owner) && (!is_this || this_in_class) {
        // 接口方法经具体类接收者调用：形参类型按类的声明签名 + 接收者实参确定
        let recv_ci = ctx.reg().get(&recv_bin).filter(|c| !c.is_interface());
        let call_is_iface = resolve_cls(ctx, &call.owner).and_then(|b| ctx.reg().get(&b)).is_some_and(|c| c.is_interface());
        if let Some(rc) = recv_ci.filter(|rc| call_is_iface && !owner::first_decl_is_bridge(ctx.reg(), rc, &call.name, &call.desc)) {
            let targ = recv_ty.as_ref().and_then(|t| receiver_type_arg_map(ctx, t, rc.name()));
            let view = RecvView { targ_map: targ.as_ref(), is_this: false, ty: recv_ty.as_ref() };
            result = lookup_method_sig_params(env, &CallRef::with(rc.name(), &call.name, &call.desc), caller_tparams, view)?;
        }
    }
    if !ctx.reg().is_empty() && result.is_none() {
        let br_recv = if is_this { ctx.class_name.to_string() } else if recv_base.is_empty() { String::new() } else { recv_bin.clone() };
        let view = RecvView { targ_map: None, is_this, ty: recv_ty.as_ref() };
        result = bridged_sig_params(env, sim, call, &br_recv, view)?;
    }
    if result.is_none() {
        let view = RecvView { targ_map: targ_map.as_ref(), is_this, ty: recv_ty.as_ref() };
        result = lookup_method_sig_params(env, call, caller_tparams, view)?;
    }
    Ok(result)
}

//! invokestatic（← `instr/invoke.py` 的 `_gen_invokestatic`）。
//!
//! 调用节点 `Self::m(args)?` / `Cls::<T..>::m(args)?` 以结构化节点发射；@CallerSensitive 包装、
//! 装箱返回与 panic 存根沿用 Python 的文本形态（[`ir::Raw`]）。

use ir::{Expr, Path, Stmt};
use sim::StackSim;
use ty::{ClassInfo, RsType};

use crate::build::{call_path, expr_stmt, id, ir_ty, let_typed, seg, text, try_, ty_text};
use crate::coerce;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy::short_binary;
use crate::invoke::bind::{bind_type_args, caller_sensitive_wrap};
use crate::invoke::sig::{self, RecvView, TargMap};
use crate::invoke::turbofish::static_call_turbofish;
use crate::invoke::CallRef;
use crate::log::{Audit, InstrLog};
use crate::naming::{class_known, mangle_if_overloaded};
use crate::owner::resolve_static_method_owner;

fn same(env: &InstrEnv, a: &RsType, b: &RsType) -> bool {
    ty_text(env, a) == ty_text(env, b)
}

#[track_caller]
fn raw_stmt(s: String) -> Stmt {
    Stmt::raw(s)
}

/// 常量池类 → 静态方法的声明类（JVM 方法解析：常量池类可以是子类）；
/// 短名在注册表中跨包重名 → 调用点改用完整模块路径（返回路径段）
fn resolve_owner(env: &InstrEnv, call: &mut CallRef) -> Vec<String> {
    let ctx = &env.ctx;
    let reg = ctx.reg();
    if reg.is_empty() || call.owner.is_empty() {
        return Vec::new();
    }
    if let Some(o) = resolve_static_method_owner(reg, &call.owner, &call.name, &call.desc) {
        if o != call.owner {
            call.owner = o;
        }
    }
    if !reg.contains(&call.owner) || short_binary(ctx, &ctx.short(&call.owner)).as_deref() == Some(call.owner.as_str()) {
        return Vec::new();
    }
    let krate = if ctx.class_name.contains('/') { "crate" } else { "java_runtime" };
    let pkgs = call.owner.split('/').collect::<Vec<_>>();
    let mut out = vec![krate.to_string()];
    for p in &pkgs[..pkgs.len().saturating_sub(1)] {
        out.push(if ty::ident::is_rust_keyword(p) { format!("r#{p}") } else { (*p).to_string() });
    }
    out
}

/// 目标声明类中的同名同描述符方法
fn find_method<'c>(ci: &'c ClassInfo, call: &CallRef) -> Option<&'c classfile::Method> {
    ci.methods().iter().find(|m| m.name == call.name && m.desc == call.desc)
}

/// 栈顶 n 个实参的静态类型
fn peek_arg_tys(sim: &StackSim, n: usize) -> Vec<RsType> {
    let st = &sim.state.stack;
    st[st.len().saturating_sub(n)..].iter().map(|e| e.ty.clone()).collect()
}

/// 跨类静态调用的实例化（类级类型变量由实参静态类型合一，未绑定取 Object）与
/// 「祖先精化」标记（`_static_inst` / `_static_anc_refined`）
fn static_instance(env: &InstrEnv, sim: &StackSim, call: &CallRef) -> (Option<Vec<RsType>>, bool) {
    let ctx = &env.ctx;
    let Some(ci) = ctx.reg().get(&call.owner) else {
        return (None, false);
    };
    if call.owner == ctx.class_name || ci.generic_signature().is_empty() || sim.state.stack.len() < call.params.len() {
        return (None, false);
    }
    let tps = ty::class_params::parse_class_type_params(ci.generic_signature());
    let Some(m) = find_method(ci, call).filter(|_| !tps.is_empty()) else {
        return (None, false);
    };
    let raw_sig = ctx.ty.method_sig_types(ci, m, &tps).params;
    let bound = bind_type_args(env, &raw_sig, &peek_arg_tys(sim, call.params.len()), &tps);
    let mut inst: Vec<RsType> = tps.iter().map(|t| bound.get(t).cloned().unwrap_or(RsType::Object)).collect();
    // 祖先精化：常量池类是当前类的祖先（enum 子类 valueOf 调基类方法）时，未绑定的
    // 类型参数按子类泛型签名实例化（javac 在该位置的静态推断）
    let mut refined_flag = false;
    if let Some(cur) = ctx.reg().get(ctx.class_name).filter(|_| !ctx.class_name.is_empty()) {
        if let Some((_, anc_args)) = ctx.ty.ancestor_type_args(cur, None).into_iter().find(|(b, _)| *b == call.owner) {
            let refined: Vec<RsType> = inst
                .iter()
                .enumerate()
                .map(|(i, a)| match anc_args.get(i) {
                    Some(x) if !matches!(x, RsType::Object) => x.clone(),
                    _ => a.clone(),
                })
                .collect();
            let changed = refined.iter().zip(&inst).any(|(r, a)| !same(env, r, a));
            if changed {
                inst = refined;
                refined_flag = inst.iter().any(|a| !matches!(a, RsType::Object));
            }
        }
    }
    (Some(inst), refined_flag)
}

/// 同类泛型静态调用：参数化形参的结构合一先于裸类型变量的实参绑定（`_static_tbind` 初值）
fn same_class_bindings(env: &InstrEnv, sim: &StackSim, call: &CallRef) -> TargMap {
    let ctx = &env.ctx;
    let Some(ci) = ctx.reg().get(&call.owner) else {
        return TargMap::new();
    };
    if call.owner != ctx.class_name
        || ci.generic_signature().is_empty()
        || sim.cfg.class_type_params.is_empty()
        || sim.state.stack.len() < call.params.len()
    {
        return TargMap::new();
    }
    let tps = ty::class_params::parse_class_type_params(ci.generic_signature());
    match find_method(ci, call).filter(|_| !tps.is_empty()) {
        Some(m) => {
            let sig = ctx.ty.method_sig_types(ci, m, &tps).params;
            bind_type_args(env, &sig, &peek_arg_tys(sim, call.params.len()), &tps)
        }
        None => TargMap::new(),
    }
}

/// 调用路径带 turbofish（`Cls::<..>::m`，非 `Self::m`）且无跨类实例化时，形参里嵌套出现的
/// 被调类形参（`Exchange<T>`）按 turbofish 的实例化代换（[`static_call_turbofish`]：同类按实参
/// 结构合一的绑定优先、其次调用者 impl 形参，否则 Object），与实参强转目标一致；
/// 裸类型变量形参已由签名查找留空（按描述符擦除），不受影响
fn instantiate_owner_params(env: &InstrEnv, sim: &StackSim, call: &CallRef, tbind: &TargMap, sig_params: Option<&mut Vec<Option<RsType>>>) {
    let Some(sp) = sig_params else {
        return;
    };
    let Some(ci) = env.ctx.reg().get(&call.owner) else {
        return;
    };
    let tps = env.ctx.ty.effective_class_type_params(ci);
    let tf = static_call_turbofish(&env.ctx, sim, &RsType::class(call.owner.clone(), Vec::new()), Some(tbind));
    if tps.is_empty() || tf.len() != tps.len() {
        return;
    }
    let map: TargMap = tps.iter().cloned().zip(tf).collect();
    for t in sp.iter_mut().flatten() {
        *t = t.substitute(&|n| map.get(n).cloned());
    }
}

/// 弹出实参并按形参类型强转；同类静态泛型方法的裸类型变量形参按实参补绑定
fn pop_args(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    sig_params: Option<&[Option<RsType>]>,
    tbind: &mut TargMap,
) -> InstrResult<Vec<Expr>> {
    let n = call.params.len();
    let mut args = Vec::with_capacity(n);
    for pi in (0..n).rev() {
        let e = sim.pop()?;
        let expected = match sig_params.and_then(|sp| sp.get(pi)).cloned().flatten() {
            Some(t) => t,
            None => env.ctx.ty.jvm_to_rust(&call.params[pi]),
        };
        if let RsType::Param(p) = &expected {
            if sim.cfg.class_type_params.contains(p)
                && !same(env, &e.ty, &expected)
                && !matches!(e.ty, RsType::Prim(_) | RsType::Unit)
            {
                tbind.entry(p.clone()).or_insert_with(|| e.ty.clone());
            }
        }
        args.push(sig::coerce_arg(env, sim, log, e.expr, &e.ty, &expected)?);
    }
    args.reverse();
    Ok(args)
}

/// 目标类不在生成范围（被截断的内部类）→ `panic!("stub: Cls.m")` 存根
fn emit_unknown_stub(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, cls_short: &str) -> InstrResult<()> {
    let rust_ret = env.ctx.ty.jvm_to_rust(&call.ret);
    let msg = format!("stub: {cls_short}.{}", call.name);
    if matches!(rust_ret, RsType::Unit) {
        sim.emit(raw_stmt(format!("__stub(\"{msg}\");")))?;
    } else {
        let v = sim.fresh("_t")?;
        sim.emit(raw_stmt(format!("let {v}: {} = __stub(\"{msg}\");", ty_text(env, &rust_ret))))?;
        sim.push(Expr::Var(v), rust_ret);
    }
    Ok(())
}

/// 调用路径与 turbofish 是否采用了实参绑定
struct Target {
    path: Path,
    turbofish_bound: bool,
}

fn call_target(env: &InstrEnv, sim: &StackSim, call: &CallRef, cls_path: &[String], inst: Option<&[RsType]>, tbind: &TargMap) -> InstrResult<Target> {
    let ctx = &env.ctx;
    let cls_short = ctx.short(&call.owner);
    let rust_m = ty::ident::safe_ident(&mangle_if_overloaded(ctx, &call.owner, &call.name, Some(&call.desc))?);
    if cls_short == ctx.class_name {
        return Ok(Target { path: Path::new(vec![seg("Self")?, seg(&rust_m)?]), turbofish_bound: false });
    }
    let turbofish = match inst {
        Some(i) => i.to_vec(),
        None => static_call_turbofish(ctx, sim, &RsType::class(call.owner.clone(), Vec::new()), Some(tbind)),
    };
    let turbofish_bound = !turbofish.is_empty()
        && short_binary(ctx, &cls_short).as_deref() == Some(ctx.class_name)
        && !sim.cfg.class_type_params.is_empty();
    let mut segs = cls_path.iter().map(|s| seg(s)).collect::<InstrResult<Vec<_>>>()?;
    let tf = turbofish.iter().map(|t| ir_ty(env, t)).collect::<InstrResult<Vec<_>>>()?;
    segs.extend(crate::invoke::special::class_segs(&cls_short, tf)?);
    segs.push(seg(&rust_m)?);
    Ok(Target { path: Path::new(segs), turbofish_bound })
}

/// invokestatic 翻译：实参弹栈强转、调用发射、返回值落 `let _tN`
pub fn gen_invokestatic(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    let ctx = &env.ctx;
    // [equiv-audit] identity-hash：按方法名匹配（该名只由根系统类声明），只计数不改发射
    if call.name == "identityHashCode" {
        log.audit(Audit::IdentityHash);
    }
    let mut call = call.clone();
    let cls_path = resolve_owner(env, &mut call);
    // [equiv-audit] class-init：目标是手写静态 native（入口无宏注入的类初始化触发）
    if let Some(ci) = ctx.reg().get(&call.owner) {
        let pp = call.param_desc();
        if ci.methods().iter().any(|m| m.name == call.name && m.is_static() && m.is_native() && m.desc.starts_with(pp)) {
            log.audit(Audit::ClassInit);
        }
    }
    let (inst, anc_refined) = static_instance(env, sim, &call);
    let targ_map: Option<TargMap> = match (&inst, ctx.reg().get(&call.owner)) {
        (Some(i), Some(ci)) => Some(ty::class_params::parse_class_type_params(ci.generic_signature()).into_iter().zip(i.iter().cloned()).collect()),
        _ => None,
    };
    let caller_tps: &[String] = if targ_map.is_some() { &[] } else { &sim.cfg.class_type_params };
    let mut sig_params = sig::lookup_method_sig_params(env, &call, caller_tps, RecvView { targ_map: targ_map.as_ref(), ..RecvView::default() })?;
    let mut tbind = same_class_bindings(env, sim, &call);
    if targ_map.is_none() && ctx.short(&call.owner) != ctx.class_name {
        instantiate_owner_params(env, sim, &call, &tbind, sig_params.as_mut());
    }
    let args = pop_args(env, sim, log, &call, sig_params.as_deref(), &mut tbind)?;
    let cls_short = ctx.short(&call.owner);
    if !class_known(ctx, &cls_short) {
        return emit_unknown_stub(env, sim, &call, &cls_short);
    }
    let target = call_target(env, sim, &call, &cls_path, inst.as_deref(), &tbind)?;
    let node = call_path(target.path, args);
    let owner = if call.owner.is_empty() { ctx.class_name.to_string() } else { call.owner.clone() };
    // @CallerSensitive 包装生效：沿用字符串形态
    let call_e = match caller_sensitive_wrap(env, &text(env, &node), &owner, &call.name, &call.desc) {
        Some(w) => Expr::raw(format!("{w}?")),
        None => try_(node),
    };
    let rust_ret = ctx.ty.jvm_to_rust(&call.ret);
    if matches!(rust_ret, RsType::Unit) {
        sim.emit(match call_e {
            Expr::Raw(r) => raw_stmt(format!("{};", r.as_str())),
            e => expr_stmt(e),
        })?;
        return Ok(());
    }
    let recv_ty = inst.as_ref().map(|i| RsType::class(call.owner.clone(), i.clone()));
    let mut sig_ret = sig::lookup_method_sig_ret(env, &call, Some(ctx.class_name), &sim.cfg.class_type_params, recv_ty.as_ref())?;
    if target.turbofish_bound && !tbind.is_empty() {
        sig_ret = sig_ret.map(|t| t.substitute(&|n| tbind.get(n).cloned()));
    }
    if anc_refined
        && sig_ret.as_ref().is_none_or(|s| same(env, s, &rust_ret))
        && sim::erased_base(&rust_ret, env) == cls_short
    {
        // 擦除返回即被调类自身的擦除实例化（Enum<Object>）：与 turbofish 同步为祖先精化后的实例化
        sig_ret = recv_ty.clone();
    }
    emit_ret(env, sim, call_e, rust_ret, sig_ret)
}

/// 返回值落 `let _tN`：签名返回与擦除映射不一致时按 Python 三分支对齐
fn emit_ret(env: &InstrEnv, sim: &mut StackSim, call_e: Expr, rust_ret: RsType, sig_ret: Option<RsType>) -> InstrResult<()> {
    let v = sim.fresh("_t")?;
    let is_raw = matches!(call_e, Expr::Raw(_));
    let typed = |sim: &mut StackSim, call_e: Expr| -> InstrResult<()> {
        let stmt = if is_raw {
            raw_stmt(format!("let {v}: {} = {};", ty_text(env, &rust_ret), text(env, &call_e)))
        } else {
            let_typed(v.clone(), Some(ir_ty(env, &rust_ret)?), call_e)
        };
        sim.emit(stmt)?;
        Ok(())
    };
    if matches!(rust_ret, RsType::Object) {
        match sig_ret.filter(|s| !matches!(s, RsType::Object)) {
            Some(s) => {
                // 签名真实返回类型装箱（S-3.1）：身份保持的 Object 上转
                let leaf = Expr::raw(text(env, &call_e));
                let boxed = coerce::to_object(env, leaf, &s, true)?;
                sim.emit(raw_stmt(format!("let {v} = {};", text(env, &boxed))))?;
            }
            None => typed(sim, call_e)?,
        }
        sim.push(Expr::Var(id(v.as_str())?), rust_ret);
        return Ok(());
    }
    match sig_ret {
        Some(s) if !same(env, &s, &rust_ret) && !matches!(rust_ret, RsType::Prim(_)) => {
            sim.emit(if is_raw {
                raw_stmt(format!("let {v} = {};", text(env, &call_e)))
            } else {
                let_typed(v.clone(), None, call_e)
            })?;
            sim.push(Expr::Var(v), s);
        }
        _ => {
            typed(sim, call_e)?;
            sim.push(Expr::Var(v), rust_ret);
        }
    }
    Ok(())
}

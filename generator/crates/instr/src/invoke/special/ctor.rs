//! invokespecial 构造器调用（← `_gen_invokespecial` 的 `<init>` 分支）：
//! `new` 待定对象 → `C::<T..>::new_x(args)?`；`super(..)` → 父类视图上的 `__init_on`；
//! 同类 `this(..)` → `this = Self::__init_on(this, ..)?`。

use classfile::Operand;
use ir::{Expr, Path, Stmt};
use sim::StackSim;
use ty::RsType;

use super::{class_segs, join_types};
use crate::build::{call_path, ir_ty, let_mut, seg, text, try_};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::hierarchy::super_chain_to_class;
use crate::invoke::bind::{ctor_outer_ref_base, is_infer, resolve_ctor_turbofish_args, super_ctor_view_args};
use crate::invoke::sig::{self, RecvView, TargMap};
use crate::invoke::CallRef;
use crate::log::InstrLog;
use crate::naming::mangle_if_overloaded;

#[track_caller]
fn raw_stmt(s: String) -> Stmt {
    Stmt::raw(s)
}

/// javac 私有构造器访问桥（synthetic `<init>(.., X$1)`）→ 委托目标构造器：
/// 桥 token（末位实参）弹栈弃置，调用按目标建模
fn redirect_bridge(env: &InstrEnv, sim: &mut StackSim, call: &mut CallRef) -> InstrResult<()> {
    let ctx = &env.ctx;
    let Some(ci) = ctx.reg().get(&call.owner) else {
        return Ok(());
    };
    let Some(bridge) = ci.methods().iter().find(|m| m.name == "<init>" && m.desc == call.desc) else {
        return Ok(());
    };
    if !bridge.is_synthetic() {
        return Ok(());
    }
    let insns = bridge.code.as_ref().map(|c| c.insns.as_slice()).unwrap_or_default();
    for ins in insns {
        let Operand::Method(m, _) = &ins.operand else {
            continue;
        };
        if ins.name() != "invokespecial" || m.name != "<init>" {
            continue;
        }
        let target = CallRef::new(m);
        if ctx.short(&m.owner) == ctx.short(&call.owner) && target.params.len() < call.params.len() {
            for _ in 0..call.params.len() - target.params.len() {
                sim.pop()?;
            }
            *call = target;
        }
        break;
    }
    Ok(())
}

/// 构造目标的类型实参（调用点静态可知时）：局部 / 匿名类按外围作用域；`super(..)` 按本类
/// SuperclassSignature；`new` 待定对象按栈上实参合一。返回 (形参映射, 作用域实参, 合一实参)
struct CtorTargs {
    map: Option<TargMap>,
    scope: Option<Vec<RsType>>,
    resolved: Option<Vec<RsType>>,
}

fn ctor_targs(env: &InstrEnv, sim: &StackSim, call: &CallRef) -> CtorTargs {
    let ctx = &env.ctx;
    let mut out = CtorTargs { map: None, scope: None, resolved: None };
    let Some(ctor_ci) = ctx.reg().get(&call.owner) else {
        return out;
    };
    let eff = ctx.ty.effective_class_type_params(ctor_ci);
    let caller_ci = ctx.reg().get(ctx.class_name);
    if !eff.is_empty() && !ctor_ci.enclosing_class().is_empty() {
        let scope = ctx.ty.enclosing_scope_type_args(ctor_ci, &sim.cfg.class_type_params);
        out.map = Some(eff.iter().cloned().zip(scope.iter().cloned()).collect());
        out.scope = Some(scope);
    } else if let Some(caller) = caller_ci.filter(|c| !eff.is_empty() && c.super_class() == call.owner && call.owner != ctx.class_name) {
        let sup = ctx.ty.superclass_type_args(caller);
        if sup.len() == eff.len() {
            out.map = Some(eff.iter().cloned().zip(sup).collect());
        }
    }
    let n = call.params.len();
    let st = &sim.state.stack;
    if out.map.is_none() && n > 0 && st.len() > n && matches!(st[st.len() - n - 1].expr, Expr::NewPending { .. }) {
        // 实例化先于实参转换确定：形参期望类型与 turbofish 使用同一份实例化
        let peek: Vec<RsType> = st[st.len() - n..].iter().map(|e| e.ty.clone()).collect();
        let resolved = resolve_ctor_turbofish_args(env, sim, &call.owner, &call.params, &peek);
        if let Some(r) = resolved.as_ref().filter(|r| r.len() == eff.len() && r.iter().any(|t| !is_infer(t))) {
            out.map = Some(eff.iter().cloned().zip(r.iter().cloned()).filter(|(_, t)| !is_infer(t)).collect());
        }
        out.resolved = resolved;
    }
    out
}

/// 类型中出现的类型形参名
fn param_names(t: &RsType, out: &mut Vec<String>) {
    match t {
        RsType::Param(n) => out.push(n.clone()),
        RsType::Class { args, .. } => args.iter().for_each(|a| param_names(a, out)),
        RsType::Array(e) => param_names(e, out),
        _ => {}
    }
}

/// 弹出构造实参：返回 (实参节点, 实参静态类型)
fn pop_ctor_args(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef, targs: &CtorTargs) -> InstrResult<(Vec<Expr>, Vec<RsType>)> {
    let ctx = &env.ctx;
    let caller_tps: &[String] = if targs.map.is_some() { &[] } else { &sim.cfg.class_type_params };
    let recv = RecvView { targ_map: targs.map.as_ref(), ..RecvView::default() };
    let sig_params = sig::lookup_method_sig_params(env, call, caller_tps, recv)?;
    let outer_base = ctor_outer_ref_base(env, &call.owner, &call.params);
    let eff_all = ctx.reg().get(&call.owner).map(|ci| ctx.ty.effective_class_type_params(ci));
    let n = call.params.len();
    let (mut nodes, mut tys) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for pi in (0..n).rev() {
        let e = sim.pop()?;
        let mut expected = match sig_params.as_ref().and_then(|sp| sp.get(pi)).cloned().flatten() {
            Some(t) => t,
            None => ctx.ty.jvm_to_rust(&call.params[pi]),
        };
        if pi == 0 {
            if let Some(ob) = &outer_base {
                let mut names = Vec::new();
                param_names(&expected, &mut names);
                let open = names.iter().any(|tp| {
                    eff_all.as_ref().is_some_and(|eff| eff.contains(tp)) && !targs.map.as_ref().is_some_and(|m| m.contains_key(tp))
                });
                // 外部实例形参里还有未确定的内部类类型变量：其实例化由实参决定（Rust 从实参推断）
                if open && e.ty.head_name(ctx.ty.names) == ob.head_name(ctx.ty.names) {
                    expected = e.ty.clone();
                }
            }
        }
        nodes.push(sig::coerce_arg(env, sim, log, e.expr, &e.ty, &expected)?);
        tys.push(e.ty);
    }
    nodes.reverse();
    tys.reverse();
    Ok((nodes, tys))
}

/// 构造器的 Rust 名（重载按 `<init>` 查再替换前缀）
fn ctor_rust_name(env: &InstrEnv, cls: &str, desc: &str, replacement: &str) -> InstrResult<String> {
    let m = mangle_if_overloaded(&env.ctx, cls, "<init>", Some(desc))?;
    Ok(ty::ident::safe_ident(&m.replace("<init>", replacement)))
}

/// invokespecial `<init>` 翻译
pub(super) fn gen_ctor(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    let mut call = call.clone();
    redirect_bridge(env, sim, &mut call)?;
    let targs = ctor_targs(env, sim, &call);
    let (nodes, tys) = pop_ctor_args(env, sim, log, &call, &targs)?;
    let obj = sim.pop()?;
    match &obj.expr {
        Expr::NewPending { class } => {
            let class = class.clone();
            new_object(env, sim, &call, &targs, &class, nodes, &tys)
        }
        e => {
            let obj_e = text(env, e);
            let args: Vec<String> = nodes.iter().map(|n| text(env, n)).collect();
            init_on(env, sim, &call, &obj_e, &args)
        }
    }
}

/// 泛型构造的 turbofish 实参：作用域实参 > 栈上合一实参 > 重新推导
fn ctor_tparams(env: &InstrEnv, sim: &StackSim, call: &CallRef, targs: &CtorTargs, full_cls: &str, tys: &[RsType]) -> Option<Vec<RsType>> {
    let same = full_cls == call.owner;
    match (&targs.scope, &targs.resolved) {
        (Some(s), _) if same && !s.is_empty() => Some(s.clone()),
        (_, Some(r)) if same && !r.is_empty() => Some(r.clone()),
        _ => resolve_ctor_turbofish_args(env, sim, full_cls, &call.params, tys),
    }
}

/// 形参是类型变量且 turbofish 实参已具体化：撤销实参向 Object 的上转，保留 `Clone::clone(&x)`
fn strip_boxing(env: &InstrEnv, call: &CallRef, full_cls: &str, tparams: &[RsType], nodes: &mut [Expr]) {
    let o = ir::anchors::OBJECT;
    let box_prefixes = [format!("{o}::from_any("), format!("{o}::from("), format!("Into::<{o}>::into(")];
    let ctx = &env.ctx;
    let Some(ci) = ctx.reg().get(full_cls) else {
        return;
    };
    let tps = ctx.ty.effective_class_type_params(ci);
    let Some(m) = ci.methods().iter().find(|m| m.name == "<init>" && m.desc == call.desc && !ty::registry::method_signature(m).is_empty()) else {
        return;
    };
    let Some(raw_sp) = ctx.ty.parse_method_param_types(ty::registry::method_signature(m), &tps, false).map(|s| s.params) else {
        return;
    };
    if raw_sp.len() != call.params.len() {
        return;
    }
    for (i, sp) in raw_sp.iter().enumerate() {
        let RsType::Param(p) = sp else {
            continue;
        };
        let (Some(idx), Some(node)) = (tps.iter().position(|t| t == p), nodes.get_mut(i)) else {
            continue;
        };
        let arg = text(env, node);
        if !arg.ends_with("))") {
            continue;
        }
        let Some(pfx) = box_prefixes.iter().find(|bp| arg.starts_with(&format!("{bp}Clone::clone(&"))) else {
            continue;
        };
        if tparams.get(idx).is_some_and(|t| !matches!(t, RsType::Object) && !is_infer(t)) {
            *node = Expr::raw(arg[pfx.len()..arg.len() - 1].to_string());
        }
    }
}

/// `new` 待定对象的构造：`C::<T..>::new_x(args)?`；装箱类构造直接用值
fn new_object(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, targs: &CtorTargs, full_cls: &str, mut nodes: Vec<Expr>, tys: &[RsType]) -> InstrResult<()> {
    let ctx = &env.ctx;
    let raw_cls = ctx.short(full_cls);
    let (init, rust_ty): (Expr, RsType) = if full_cls.contains('/') {
        let rust_ty = ctx.ty.jvm_to_rust(&format!("L{full_cls};"));
        if matches!(rust_ty, RsType::Prim(_)) {
            // 自动装箱优化：原始包装类型直接用值，跳过构造器调用
            let v = nodes.first().map_or_else(|| "0".to_string(), |n| text(env, n));
            (Expr::raw(v), rust_ty)
        } else if !sim::types::is_object(&rust_ty) && !rust_ty.type_args().is_empty() {
            let tparams = ctor_tparams(env, sim, call, targs, full_cls, tys).unwrap_or_default();
            if !tparams.is_empty() {
                strip_boxing(env, call, full_cls, &tparams, &mut nodes);
            }
            ctor_call(env, &raw_cls, full_cls, &call.desc, tparams, nodes)?
        } else {
            ctor_call(env, &raw_cls, full_cls, &call.desc, Vec::new(), nodes)?
        }
    } else if !raw_cls.is_empty() {
        // 无包类（缺省包）：短名表对任何 binary 名都给出不含 `/` 的短名，仅空类名得到空短名
        let generic = ctx.reg().get(&call.owner).is_some_and(|ci| !ctx.ty.effective_class_type_params(ci).is_empty());
        let tparams = if generic { ctor_tparams(env, sim, call, targs, full_cls, tys).unwrap_or_default() } else { Vec::new() };
        ctor_call(env, &raw_cls, &raw_cls, &call.desc, tparams, nodes)?
    } else {
        return Err(InstrError::BadInsn(format!("new 的类名为空（{}.<init>{}）", call.owner, call.desc)));
    };
    let dup_pending = sim.state.stack.last().is_some_and(|e| matches!(e.expr, Expr::NewPending { .. }));
    if dup_pending {
        let id = sim.state.next_id;
        sim.state.next_id += 1;
        if let Some(top) = sim.state.stack.last_mut() {
            *top = sim::StackEntry { expr: init, ty: rust_ty, id };
        }
        return Ok(());
    }
    let v = sim.fresh("_obj")?;
    let stmt = match &init {
        Expr::Raw(r) => raw_stmt(format!("let mut {v}: {} = {};", crate::build::ty_text(env, &rust_ty), r.as_str())),
        _ => let_mut(v.clone(), Some(ir_ty(env, &rust_ty)?), init),
    };
    sim.emit(stmt)?;
    sim.push(Expr::Var(v), rust_ty);
    Ok(())
}

/// `Cls::<T..>::new_x(args)?` 与其静态类型
fn ctor_call(env: &InstrEnv, raw_cls: &str, mangle_cls: &str, desc: &str, tparams: Vec<RsType>, nodes: Vec<Expr>) -> InstrResult<(Expr, RsType)> {
    let name = ctor_rust_name(env, mangle_cls, desc, "new")?;
    let tf = tparams.iter().map(|t| ir_ty(env, t)).collect::<InstrResult<Vec<_>>>()?;
    let mut segs = class_segs(raw_cls, tf)?;
    segs.push(seg(&name)?);
    let path = Path::new(segs);
    let binary = env.ctx.ty.names.binary_of(raw_cls).map_or_else(|| mangle_cls.to_string(), str::to_string);
    let bin = if mangle_cls.contains('/') { mangle_cls.to_string() } else { binary };
    Ok((try_(call_path(path, nodes)), RsType::class(bin, tparams)))
}

/// 已存在对象上的构造器体：`super(..)` 走父类视图的 `__init_on`（K-5 单一对象模型），
/// 同类 `this(..)` 委托在同一身份上执行；根类 `super()` 为 no-op 注释
fn init_on(env: &InstrEnv, sim: &mut StackSim, call: &CallRef, obj_e: &str, args: &[String]) -> InstrResult<()> {
    let ctx = &env.ctx;
    let comment = format!("Method {}.{}:{}", call.owner, call.name, call.desc);
    if !(obj_e == "this" || obj_e == "self") || call.owner.is_empty() {
        // JVMS §4.10.1.9：`<init>` 的接收者只能是未初始化对象——`new` 的待定对象（上一分支）
        // 或构造器内的 uninitializedThis（局部 0 = this）。其余形态是校验器拒绝的字节码
        return Err(InstrError::BadInsn(format!("invokespecial {comment} 的接收者 {obj_e} 不是未初始化对象")));
    }
    let raw_cls = ctx.short(&call.owner);
    if call.owner == ty::consts::OBJECT || raw_cls == ctx.short(ty::consts::OBJECT) {
        sim.emit(raw_stmt(format!("/* invokespecial {comment} ({} no-op) */", ir::anchors::OBJECT)))?;
        return Ok(());
    }
    let init_on = ctor_rust_name(env, &call.owner, &call.desc, "__init_on")?;
    let tail = if args.is_empty() { String::new() } else { format!(", {}", args.join(", ")) };
    if super_chain_to_class(ctx, ctx.class_name, &raw_cls) == 0 {
        // 同类构造器委托 this(args)：同一身份上执行被委托构造器体
        sim.emit(raw_stmt(format!("this = Self::{init_on}(this{tail})?;")))?;
        return Ok(());
    }
    // super(..)：已存在的 this 经 From<Self> for Parent（宏生成的 vtable 上转）以父类视图
    // 传入父类 __init_on；泛型父类按本类 SuperclassSignature 的实参给出视图实例化
    let sup_args = super_ctor_view_args(env, &call.owner);
    let (sup_ty, path) = if sup_args.is_empty() {
        (raw_cls.clone(), format!("{raw_cls}::{init_on}"))
    } else {
        let a = join_types(env, &sup_args);
        (format!("{raw_cls}<{a}>"), format!("{raw_cls}::<{a}>::{init_on}"))
    };
    let view = format!("<{sup_ty} as ::std::convert::From<Self>>::from(::std::clone::Clone::clone(&this))");
    sim.emit(raw_stmt(format!("{path}({view}{tail})?;")))?;
    Ok(())
}

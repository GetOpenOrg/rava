//! invokestatic / invokevirtual / invokeinterface（← `instr/sim/methods.py`）：签名多态调用
//! （JVMS §2.9.3）的实参装箱与返回还原、接口 default 方法初始化缺口审计，其余分发到
//! [`crate::invoke::static_call`] / [`crate::invoke::virtual_`]。

use classfile::{Insn, Operand};
use ir::{Expr, Stmt};
use sim::StackSim;
use ty::RsType;

use crate::build::{text, ty_text};
use crate::coerce;
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::invoke::static_call::gen_invokestatic;
use crate::invoke::virtual_::gen_invokevirtual;
use crate::invoke::CallRef;
use crate::log::{Audit, InstrLog};

const ACC_VARARGS: u16 = 0x0080;
const ACC_NATIVE: u16 = 0x0100;

/// 本组指令；非本组 → Ok(false)
pub fn sim_methods(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    let op = ins.name();
    if !matches!(op, "invokestatic" | "invokevirtual" | "invokeinterface") {
        return Ok(false);
    }
    let Operand::Method(m, _) = &ins.operand else {
        return Err(InstrError::BadInsn(format!("{op} 的操作数不是方法引用")));
    };
    let call = CallRef::new(m);
    if op != "invokeinterface" {
        if let Some(decl_desc) = signature_polymorphic_descriptor(env, &call) {
            gen_signature_polymorphic(env, sim, log, &call, &decl_desc, op == "invokestatic")?;
            return Ok(true);
        }
    }
    if op == "invokestatic" {
        gen_invokestatic(env, sim, log, &call)?;
    } else {
        // [equiv-audit] class-init：invokeinterface → 接口 default 方法，接口自身初始化未触发
        // （JVMS §5.5 触发点缺口），只计数不改发射
        if op == "invokeinterface" && iface_default_init_gap(env, &call) {
            log.audit(Audit::ClassInit);
        }
        gen_invokevirtual(env, sim, log, &call)?;
    }
    Ok(true)
}

/// 调用目标是签名多态方法时返回其声明描述符：常量池类上同名方法唯一、带
/// ACC_VARARGS | ACC_NATIVE、形参恰为一个引用数组（判定完全来自类文件）
fn signature_polymorphic_descriptor(env: &InstrEnv, call: &CallRef) -> Option<String> {
    let ci = env.ctx.reg().get(&call.owner)?;
    let mut named = ci.methods().iter().filter(|m| m.name == call.name);
    let m = named.next()?;
    // 同名唯一：调用点恰为 `([Object)Object` 时实参是一个数组值，不得按声明的可变参数组平铺
    if named.next().is_some() || m.access & (ACC_VARARGS | ACC_NATIVE) != ACC_VARARGS | ACC_NATIVE {
        return None;
    }
    let decl = ty::type_map::parse_descriptor_params(&m.desc);
    (decl.len() == 1 && decl[0].starts_with("[L")).then(|| m.desc.clone())
}

/// `let {v}: {target} = <{target} as ::std::convert::From<Object>>::from({src});`，压 `Var(v)`
fn from_object_push(env: &InstrEnv, sim: &mut StackSim, target: RsType, src: &str) -> InstrResult<()> {
    let v = sim.fresh("_t")?;
    let t = ty_text(env, &target);
    let o = ir::anchors::OBJECT;
    sim.emit(Stmt::raw(format!("let {v}: {t} = <{t} as ::std::convert::From<{o}>>::from({src});")));
    sim.push(Expr::Var(v), target);
    Ok(())
}

/// 签名多态调用：实参按 Java 语义装进声明的 Object[] 形参，返回值按调用点描述符还原
/// （等价 `(R) mh.invokeBasic(new Object[]{a, b, c})`）。清单登记需要调用点类型的成员
/// 改发 `recv.<m>__site("<调用点描述符>", 实参数组)`：调用点 MethodType 只存在于字节码
fn gen_signature_polymorphic(
    env: &InstrEnv,
    sim: &mut StackSim,
    log: &mut InstrLog,
    call: &CallRef,
    decl_desc: &str,
    is_static: bool,
) -> InstrResult<()> {
    let ty = &env.ctx.ty;
    let mut packed = Vec::with_capacity(call.params.len());
    for p in call.params.iter().rev() {
        let e = sim.pop()?;
        let mut t = e.ty;
        let mut s = text(env, &e.expr);
        // 子 int 基本类型（C/S/B/Z）在栈上是 int：按调用点描述符收窄后再装箱
        // （Object[] 元素的装箱类型与 Java 自动装箱一致）
        if matches!(p.as_str(), "C" | "S" | "B" | "Z") {
            let p_ty = ty.jvm_to_rust(p);
            if ty_text(env, &p_ty) != ty_text(env, &t) {
                s = if p == "Z" { format!("({s} != 0)") } else { format!("({s} as {})", ty_text(env, &p_ty)) };
                t = p_ty;
            }
        }
        let item = if ty_text(env, &t) == ir::anchors::OBJECT {
            if s == "this" { "Clone::clone(this)".to_string() } else { format!("Clone::clone(&{s})") }
        } else {
            text(env, &coerce::to_object(env, Expr::raw(s), &t, true)?)
        };
        packed.push(item);
    }
    packed.reverse();
    let decl_params = ty::type_map::parse_descriptor_params(decl_desc);
    let elem_ty = ty.jvm_to_rust(decl_params.first().map_or("", String::as_str));
    let arr = sim.fresh("_t")?;
    sim.emit(Stmt::raw(format!(
        "let {arr}: {} = {}::from(vec![{}]);",
        ty_text(env, &elem_ty),
        ir::anchors::ARRAY,
        packed.join(", ")
    )));
    let decl_ret = ty::type_map::parse_descriptor_return(decl_desc);
    let site_key = format!("{}.{}", call.owner, call.name);
    if !is_static && env.ctx.rt.sigpoly_callsite_typed.contains(&site_key) {
        let recv = text(env, &sim.pop()?.expr);
        let res = sim.fresh("_t")?;
        let ret_ty = ty.jvm_to_rust(decl_ret);
        sim.emit(Stmt::raw(format!(
            "let {res}: {} = {recv}.{}__site(\"{}\", {arr})?;",
            ty_text(env, &ret_ty),
            call.name,
            call.desc
        )));
        if call.ret == "V" {
            return Ok(());
        }
        let target = ty.jvm_to_rust(&call.ret);
        if ty_text(env, &target) == ty_text(env, &ret_ty) {
            sim.push(Expr::Var(res), ret_ty);
            return Ok(());
        }
        return from_object_push(env, sim, target, res.as_str());
    }
    sim.push(Expr::Var(arr), elem_ty);
    let decl_call = CallRef::with(&call.owner, &call.name, decl_desc);
    if is_static {
        gen_invokestatic(env, sim, log, &decl_call)?;
    } else {
        gen_invokevirtual(env, sim, log, &decl_call)?;
    }
    if decl_ret == "V" {
        return Ok(());
    }
    let r = sim.pop()?;
    if call.ret == "V" {
        return Ok(());
    }
    let target = ty.jvm_to_rust(&call.ret);
    if ty_text(env, &target) == ty_text(env, &r.ty) {
        sim.push(r.expr, r.ty);
        return Ok(());
    }
    from_object_push(env, sim, target, &text(env, &r.expr))
}

/// invokeinterface 的目标是接口上带体的 default 方法（粗口径：只查常量池接口自身声明）
fn iface_default_init_gap(env: &InstrEnv, call: &CallRef) -> bool {
    env.ctx
        .reg()
        .get(&call.owner)
        .filter(|ci| ci.is_interface())
        .is_some_and(|ci| ci.methods().iter().any(|m| m.name == call.name && m.desc == call.desc && !m.is_abstract()))
}

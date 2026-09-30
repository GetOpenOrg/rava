//! `ObjectMethods` 类引导方法调用点（record 的 toString / hashCode / equals，`[indy] object_methods`）。
//!
//! 引导静态实参：`[record 类, "a;b;..." 分量名, getter 句柄...]`（getter 为 REF_getField）。
//! 调用点名决定语义（`java.lang.runtime.ObjectMethods`）：
//! - `toString (R)String`：`简单名[a=.., b=..]`，分量按 `String.valueOf` 字符串化（与拼接同一翻译）；
//! - `hashCode (R)I`：`h = 31 * h + hash(分量)` 自 0 起，基本类型按装箱类 `hashCode`，引用按 `Objects.hashCode`；
//! - `equals (R, Object)Z`：实参是本 record 类实例，且逐分量相等（浮点按 `compare == 0`，引用按 `Objects.equals`）。
//!
//! 分量读取与 getfield 同一翻译（访问器命名、声明类型恢复都复用 [`crate::sim::fields::getfield`]）。

use classfile::{Const, MemberRef};
use ir::{ElseBranch, Expr, IfStmt, Raw, Stmt};
use sim::StackSim;
use ty::{Prim, RsType};

use super::boxing::obj_text;
use super::concat::concat_from_stack;
use super::{raw_stmt, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::sim::fields::getfield;

/// 引导静态实参：(record 类, 分量名, getter 字段引用)
fn components(site: &IndySite) -> InstrResult<(String, Vec<String>, Vec<MemberRef>)> {
    let bad = || InstrError::BadInsn(format!("ObjectMethods 调用点 {}{} 的引导实参形态", site.name, site.desc));
    let args = &site.bsm.ok_or_else(bad)?.args;
    let (Some(Const::Class(cls)), Some(Const::String(names))) = (args.first(), args.get(1)) else {
        return Err(bad());
    };
    let getters = args[2..]
        .iter()
        .map(|a| match a {
            Const::MethodHandle(h) if h.kind == 1 => Ok(h.member.clone()),
            _ => Err(bad()),
        })
        .collect::<InstrResult<Vec<_>>>()?;
    let names: Vec<String> = if names.is_empty() { Vec::new() } else { names.split(';').map(str::to_string).collect() };
    if names.len() != getters.len() {
        return Err(bad());
    }
    Ok((cls.clone(), names, getters))
}

/// `Class.getSimpleName`：InnerClasses 登记的简单名（匿名类为空），否则取包路径后的类名
fn simple_name(env: &InstrEnv, cls: &str) -> String {
    let declared = env.ctx.reg().get(cls).and_then(|ci| {
        ci.class_file().inner_classes.iter().find(|ic| ic.inner == cls).map(|ic| ic.simple_name.clone().unwrap_or_default())
    });
    declared.unwrap_or_else(|| cls.rsplit('/').next().unwrap_or(cls).to_string())
}

/// 弹出接收者并物化为变量（多次读取分量只求值一次）
fn pop_bound(sim: &mut StackSim) -> InstrResult<sim::StackEntry> {
    let e = sim.pop()?;
    if matches!(e.expr, Expr::Var(_)) {
        return Ok(e);
    }
    let v = sim.fresh_let("__om_recv", e.expr, &e.ty)?;
    Ok(sim::StackEntry { expr: v, ty: e.ty, id: e.id })
}

/// 在接收者上读一个分量（getfield 同一翻译），返回 (值文本, 类型)
fn read(env: &InstrEnv, sim: &mut StackSim, recv: &sim::StackEntry, g: &MemberRef) -> InstrResult<(String, RsType)> {
    sim.push(recv.expr.clone(), recv.ty.clone());
    getfield(env, sim, g)?;
    let e = sim.pop()?;
    Ok((text(env, &e.expr), e.ty))
}

/// 浮点位模式（NaN 归一：`floatToIntBits` / `doubleToLongBits`）
fn float_bits(v: &str, desc: &str) -> String {
    if desc == "F" {
        format!("(if {v}.is_nan() {{ 0x7fc00000u32 }} else {{ {v}.to_bits() }})")
    } else {
        format!("(if {v}.is_nan() {{ 0x7ff8000000000000u64 }} else {{ {v}.to_bits() }})")
    }
}

/// 分量的 hashCode（装箱类 hashCode / Objects.hashCode）
fn hash_of(env: &InstrEnv, v: &str, t: &RsType, desc: &str) -> String {
    match desc {
        "Z" => format!("(if {v} {{ 1231i32 }} else {{ 1237i32 }})"),
        "J" => format!("{{ let v = {v}; (v ^ ((v as u64) >> 32) as i64) as i32 }}"),
        "F" => format!("({} as i32)", float_bits(v, desc)),
        "D" => format!("{{ let b = {}; (b ^ (b >> 32)) as i32 }}", float_bits(v, desc)),
        _ if matches!(t, RsType::Prim(_)) => format!("({v} as i32)"),
        _ => {
            let o = obj_text(env, v, t);
            format!("{{ let c: {O} = {o}; if _is_jnull(&c) {{ 0i32 }} else {{ c.hashCode()? }} }}", O = ir::anchors::OBJECT)
        }
    }
}

/// 两分量相等（基本类型 `==`，浮点 `compare == 0`，引用 `Objects.equals`：null 仅等于 null，
/// 否则虚分派 equals）
fn eq_of(env: &InstrEnv, (a, at): &(String, RsType), (b, bt): &(String, RsType), desc: &str) -> String {
    match desc {
        "F" | "D" => format!("{} == {}", float_bits(a, desc), float_bits(b, desc)),
        _ if matches!(at, RsType::Prim(_)) => format!("{a} == {b}"),
        _ => format!(
            "{{ let x: {O} = {}; let y: {O} = {}; if _is_jnull(&x) {{ _is_jnull(&y) }} else {{ x.equals(y)? }} }}",
            obj_text(env, a, at),
            obj_text(env, b, bt),
            O = ir::anchors::OBJECT
        ),
    }
}

pub(super) fn gen_object_methods(env: &InstrEnv, sim: &mut StackSim, site: &IndySite) -> InstrResult<()> {
    let (cls, names, getters) = components(site)?;
    match (site.name, site.desc.ends_with(")Z")) {
        ("toString", _) => {
            let recv = pop_bound(sim)?;
            for g in &getters {
                sim.push(recv.expr.clone(), recv.ty.clone());
                getfield(env, sim, g)?;
            }
            let parts: Vec<String> = names.iter().map(|n| format!("{n}=\u{1}")).collect();
            let template = format!("{}[{}]", simple_name(env, &cls), parts.join(", "));
            let descs: Vec<String> = getters.iter().map(|g| g.desc.clone()).collect();
            concat_from_stack(env, sim, &descs, Some((template, Vec::new())))
        }
        ("hashCode", _) => {
            let recv = pop_bound(sim)?;
            let h = sim.fresh("__om_hash")?;
            sim.emit(raw_stmt(format!("let mut {h}: i32 = 0;")));
            for g in &getters {
                let (v, t) = read(env, sim, &recv, g)?;
                let term = hash_of(env, &v, &t, &g.desc);
                sim.emit(raw_stmt(format!("{h} = {h}.wrapping_mul(31).wrapping_add({term});")));
            }
            sim.push(Expr::Var(h), RsType::Prim(Prim::I32));
            Ok(())
        }
        ("equals", true) => {
            let other = sim.pop()?;
            let recv = pop_bound(sim)?;
            let other_o = obj_text(env, &text(env, &other.expr), &other.ty);
            let o = sim.fresh_let("__om_other", Expr::Raw(Raw(other_o)), &RsType::Object)?;
            let o_s = text(env, &o);
            let r_ty = ty_text(env, &recv.ty);
            let v = sim.fresh("__om_eq")?;
            sim.emit(raw_stmt(format!("let mut {v}: bool = {o_s}.is_instance_of(\"{cls}\");")));
            // 类型相符分支：实参转为本 record 类型后逐分量比较（分量读取可能物化的语句一并收入分支）
            let mark = sim.state.stmts.len();
            let that = sim.fresh("__om_that")?;
            sim.emit(raw_stmt(format!("let {that}: {r_ty} = Into::<{r_ty}>::into(Clone::clone(&{o_s}));")));
            let that_e = sim::StackEntry { expr: Expr::Var(that), ty: recv.ty.clone(), id: recv.id };
            let mut cmps = Vec::with_capacity(getters.len());
            for g in &getters {
                let mine = read(env, sim, &recv, g)?;
                let theirs = read(env, sim, &that_e, g)?;
                cmps.push(eq_of(env, &mine, &theirs, &g.desc));
            }
            let all = if cmps.is_empty() { "true".to_string() } else { cmps.join(" && ") };
            sim.emit(raw_stmt(format!("{v} = {all};")));
            let then = sim.state.stmts.split_off(mark);
            sim.emit(Stmt::If(IfStmt { cond: Expr::Var(v.clone()), then, else_: ElseBranch::None }));
            sim.push(Expr::Var(v), RsType::Prim(Prim::Bool));
            Ok(())
        }
        _ => Err(InstrError::BadInsn(format!("ObjectMethods 调用点 {}{} 不是 toString / hashCode / equals", site.name, site.desc))),
    }
}

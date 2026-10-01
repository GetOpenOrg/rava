//! `ObjectMethods` 类引导方法调用点（record 的 toString / hashCode / equals，`[indy] object_methods`）。
//!
//! 引导静态实参：`[record 类, "a;b;..." 分量名, getter 句柄...]`（getter 为 REF_getField）。
//! 调用点名决定语义（`java.lang.runtime.ObjectMethods`）：
//! - `toString (R)String`：`简单名[a=.., b=..]`，分量按 `String.valueOf` 字符串化（与拼接同一翻译）；
//! - `hashCode (R)I`：`h = 31 * h + hash(分量)` 自 0 起，基本类型按装箱类 `hashCode`，引用分量调用清单
//!   `[indy] component_hash`（`Objects.hashCode(Object)`）；
//! - `equals (R, Object)Z`：实参是本 record 类实例，且逐分量相等（浮点按 `compare == 0`，引用分量调用清单
//!   `[indy] component_equals`（`Objects.equals(Object,Object)`））。引用分量的入口方法体均由字节码翻译。
//!
//! 分量读取与 getfield 同一翻译（访问器命名、声明类型恢复都复用 [`crate::sim::fields::getfield`]）。

use classfile::{Const, MemberRef};
use ir::{Expr};
use sim::StackSim;
use ty::{Prim, RsType};

use super::boxing::obj_text;
use super::concat::{call_indy_helper, concat_from_stack, Recipe};
use super::{raw_stmt, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::log::InstrLog;
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

/// 在接收者上读一个分量（getfield 同一翻译）
fn read(env: &InstrEnv, sim: &mut StackSim, recv: &sim::StackEntry, g: &MemberRef) -> InstrResult<sim::StackEntry> {
    sim.push(recv.expr.clone(), recv.ty.clone());
    getfield(env, sim, g)?;
    Ok(sim.pop()?)
}

/// 引用分量：实参压栈后调用清单登记的分量处理入口（`[indy] component_hash` / `component_equals`），返回 (结果文本, 类型)
fn ref_call(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, key: &str, args: Vec<sim::StackEntry>) -> InstrResult<(String, RsType)> {
    for a in args {
        sim.push(a.expr, a.ty);
    }
    let r = call_indy_helper(env, sim, log, key)?;
    Ok((text(env, &r.expr), r.ty))
}

/// 浮点位模式（NaN 归一：`floatToIntBits` / `doubleToLongBits`）
fn float_bits(v: &str, desc: &str) -> String {
    if desc == "F" {
        format!("(if {v}.is_nan() {{ 0x7fc00000u32 }} else {{ {v}.to_bits() }})")
    } else {
        format!("(if {v}.is_nan() {{ 0x7ff8000000000000u64 }} else {{ {v}.to_bits() }})")
    }
}

/// 基本类型分量的 hashCode（装箱类 hashCode）
fn prim_hash(v: &str, desc: &str) -> String {
    match desc {
        "Z" => format!("(if {v} {{ 1231i32 }} else {{ 1237i32 }})"),
        "J" => format!("{{ let v = {v}; (v ^ ((v as u64) >> 32) as i64) as i32 }}"),
        "F" => format!("({} as i32)", float_bits(v, desc)),
        "D" => format!("{{ let b = {}; (b ^ (b >> 32)) as i32 }}", float_bits(v, desc)),
        _ => format!("({v} as i32)"),
    }
}

/// 两基本类型分量相等（`==`，浮点 `compare == 0`）
fn prim_eq(a: &str, b: &str, desc: &str) -> String {
    match desc {
        "F" | "D" => format!("{} == {}", float_bits(a, desc), float_bits(b, desc)),
        _ => format!("{a} == {b}"),
    }
}

pub(super) fn gen_object_methods(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, site: &IndySite) -> InstrResult<()> {
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
            concat_from_stack(env, sim, log, &descs, Some(Recipe::text(&template)))
        }
        ("hashCode", _) => {
            let recv = pop_bound(sim)?;
            let h = sim.fresh("__om_hash")?;
            sim.emit(raw_stmt(format!("let mut {h}: i32 = 0;")))?;
            for g in &getters {
                let e = read(env, sim, &recv, g)?;
                let term = match &e.ty {
                    RsType::Prim(_) => prim_hash(&text(env, &e.expr), &g.desc),
                    _ => ref_call(env, sim, log, "component_hash", vec![e])?.0,
                };
                sim.emit(raw_stmt(format!("{h} = {h}.wrapping_mul(31).wrapping_add({term});")))?;
            }
            sim.push(Expr::Var(h), RsType::Prim(Prim::I32));
            Ok(())
        }
        ("equals", true) => {
            let other = sim.pop()?;
            let recv = pop_bound(sim)?;
            // 实参按描述符为 Object：已是 Object 变量时直接使用，否则物化一次
            let o_s = match (&other.expr, &other.ty) {
                (Expr::Var(v), RsType::Object) => v.as_str().to_string(),
                (e, RsType::Object) => {
                    let t = text(env, e);
                    text(env, &sim.fresh_let("__om_other", Expr::raw(format!("Clone::clone(&{t})")), &RsType::Object)?)
                }
                (e, t) => {
                    let o = obj_text(env, &text(env, e), t);
                    text(env, &sim.fresh_let("__om_other", Expr::raw(o), &RsType::Object)?)
                }
            };
            let r_ty = ty_text(env, &recv.ty);
            // 类型相符分支：实参转为本 record 类型后逐分量比较。每个分量读取 / 比较物化的语句收入
            // 该分量自己的块（`{ 语句; 比较 }`），只在 instanceof 成立且前序分量相等时求值（短路）
            let that = sim.fresh("__om_that")?;
            let that_e = sim::StackEntry { expr: Expr::Var(that.clone()), ty: recv.ty.clone(), id: recv.id };
            let mut cmps = Vec::with_capacity(getters.len());
            for g in &getters {
                let mark = sim.state.stmts.len();
                let mine = read(env, sim, &recv, g)?;
                let theirs = read(env, sim, &that_e, g)?;
                let cmp = match &mine.ty {
                    RsType::Prim(_) => prim_eq(&text(env, &mine.expr), &text(env, &theirs.expr), &g.desc),
                    _ => match ref_call(env, sim, log, "component_equals", vec![mine, theirs])? {
                        (t, RsType::Prim(Prim::Bool)) => t,
                        // 布尔结果按 i32（1/0）流动时归一为 bool
                        (t, _) => format!("({t} != 0)"),
                    },
                };
                let names = sim::TyNames(env);
                let renderer = ir::Renderer::new(&names);
                let captured: Vec<String> = sim.state.stmts.split_off(mark).iter().map(|st| renderer.stmt(st, 0)).collect();
                cmps.push(if captured.is_empty() { cmp } else { format!("{{ {} {cmp} }}", captured.join(" ")) });
            }
            // 语句位置的 `{ .. } && ..` 会被解析为块语句，整体加括号成表达式
            let all = if cmps.is_empty() { "true".to_string() } else { format!("({})", cmps.join(" && ")) };
            let block = format!("let {that}: {r_ty} = Into::<{r_ty}>::into(Clone::clone(&{o_s}));");
            let v = format!("{o_s}.is_instance_of(\"{cls}\") && {{ {block} {all} }}");
            let e = sim.fresh_let("__om_eq", Expr::raw(v), &RsType::Prim(Prim::Bool))?;
            sim.push(e, RsType::Prim(Prim::Bool));
            Ok(())
        }
        _ => Err(InstrError::BadInsn(format!("ObjectMethods 调用点 {}{} 不是 toString / hashCode / equals", site.name, site.desc))),
    }
}

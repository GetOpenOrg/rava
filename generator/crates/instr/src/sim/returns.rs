//! 返回指令（← `instr/sim/returns.py`）：`return` / `ireturn..dreturn` / `areturn`。
//!
//! areturn 的值按声明返回类型对齐（this 克隆、Object 边界装箱 / 取回、接口载体、子类上转、
//! 祖先另一实例化的 Object 边界重建）；判定与 Python 同口径作用于类型渲染文本。

use classfile::Insn;
use ir::anchors::OBJECT;
use ir::{Expr, FnPath, Lit, Path, Stmt};
use sim::exprs::{clone_plain, default_value, from_call, object_from, object_type, qualified_from};
use sim::StackSim;
use ty::{JvmType, Prim, RsType};

use crate::build::{call, ir_ty, seg, str_leaf, text, ty_text, var};
use crate::coerce::{self, to_object};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy::{is_subtype, type_binary};
use crate::invoke::sig::upcast_to_ancestor_instantiation;
use crate::log::InstrLog;
use crate::sim::control::{erased_base_ty, erased_shape, words};

/// `return Ok(v);`
fn return_ok(v: Expr) -> InstrResult<Stmt> {
    Ok(Stmt::Return(Some(call(&["Ok"], vec![v])?)))
}

/// `PRIMITIVE_RUST_TYPES`（基本类型与 `()`）
fn is_prim_text(s: &str) -> bool {
    matches!(s, "i8" | "i16" | "i32" | "i64" | "u8" | "u16" | "u32" | "u64" | "f32" | "f64" | "bool" | "usize" | "()")
}

/// `::std::convert::From::from(v)`
fn std_from(v: Expr) -> InstrResult<Expr> {
    let p = Path { global: true, segments: vec![seg("std")?, seg("convert")?, seg("From")?, seg("from")?] };
    Ok(Expr::Call { func: FnPath::Path(p), args: vec![v] })
}

/// 本组指令；非本组 → Ok(false)
pub fn sim_returns(env: &InstrEnv, sim: &mut StackSim, _log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    match ins.name() {
        "return" => {
            // 构造函数 return：返回 Ok(this) 而非 Ok(())
            let v = if sim.cfg.is_constructor { var("this")? } else { Expr::Lit(Lit::Unit) };
            sim.emit(return_ok(v)?)?;
        }
        "ireturn" | "lreturn" | "freturn" | "dreturn" => {
            let e = sim.pop()?;
            // 返回类型与栈类型不匹配（窄类型 / bool ↔ i32）：显式转换
            let ret = sim.cfg.return_type.clone();
            let narrow = matches!(ret, RsType::Prim(Prim::I8 | Prim::I16 | Prim::U16 | Prim::Bool | Prim::I32));
            let v = if ty_text(env, &e.ty) != ty_text(env, &ret) && narrow { coerce::value(e.expr, &e.ty, &ret) } else { e.expr };
            sim.emit(return_ok(v)?)?;
        }
        "areturn" => {
            let e = sim.pop()?;
            let v = areturn_value(env, sim, e.expr, &e.ty)?;
            sim.emit(return_ok(v)?)?;
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// areturn 的返回值（Python `expr_s` 的逐分支改写）
fn areturn_value(env: &InstrEnv, sim: &StackSim, e: Expr, actual: &RsType) -> InstrResult<Expr> {
    let ret = &sim.cfg.return_type;
    let (ret_s, act_s) = (ty_text(env, ret), ty_text(env, actual));
    let ctps = &sim.cfg.class_type_params;
    let (ret_base, act_base) = (erased_base_ty(ret), erased_base_ty(actual));
    // 实例方法返回 this：this 是 &Self，需 Clone::clone 取得 owned 值
    let returns_this = text(env, &e) == "this" && !sim.cfg.is_static;
    let cur = if returns_this { clone_plain(var("this")?)? } else { str_leaf(e) };
    let carrier_ret = env.ctx.ty.carrier_type_for_ident(ret);
    let is_obj = |s: &str| s == OBJECT;
    if returns_this && ctps.contains(&ret_s) {
        // 声明返回类型变量（`return (S) this`）：经 Object 边界取回（宏补 From<Object> bound）
        return Ok(from_call(to_object(env, cur, actual, false)?)?);
    }
    if returns_this && is_obj(&ret_s) && !matches!(act_s.as_str(), OBJECT | "()") {
        // 声明返回 Object / 接口：身份保持的向上转型
        return to_object(env, cur, actual, false);
    }
    if returns_this && ret_s.contains('<') && !ctps.is_empty() && !env.ctx.reg().is_empty()
        && type_binary(&env.ctx, &ret_base).as_deref() == Some(env.ctx.class_name)
    {
        // 泛型类 `return this` 且声明返回本类（另一）实例化：经 Object 边界按返回形态重建
        if sim::erased_base(ret, env) == env.ctx.short(env.ctx.class_name) {
            return Ok(qualified_from(ir_ty(env, ret)?, object_type()?, object_from(cur)?)?);
        }
        return Ok(cur);
    }
    if carrier_ret.is_some_and(|c| ty_text(env, &c) == ret_s) {
        // 声明返回已铺设的接口载体：同载体原样；Object → From::from 取回；其余经 Object 边界上转
        return Ok(if act_s == ret_s {
            cur
        } else if is_obj(&act_s) {
            std_from(cur)?
        } else {
            std_from(to_object(env, cur, actual, true)?)?
        });
    }
    if returns_this
        && (ret_s == act_s
            || (sim::erased_base(ret, env) == sim::erased_base(actual, env) && !ret_s.contains('<') && !act_s.contains('<'))
            || !is_subtype(&env.ctx, &act_base, &ret_base))
    {
        // 返回类型就是本类（同形态）：this 的克隆即返回值
        return Ok(cur);
    }
    if is_obj(&ret_s) && !matches!(act_s.as_str(), OBJECT | "()") {
        return to_object(env, cur, actual, true);
    }
    if !is_obj(&ret_s) && is_obj(&act_s) {
        // Object 引用按声明返回类型返回（javac checkcast / unchecked cast）：类型变量、类 wrapper、
        // 数组经 From<Object> 取回；null 字面量 / 无运行时类的返回类型取零值
        let ret_is_class = match env.ctx.ty.from_rs_type(ret, &Default::default()) {
            JvmType::Class { binary, .. } => env.ctx.reg().get(&binary).is_some_and(|ci| !ci.is_interface()),
            _ => false,
        };
        let recover = text(env, &cur) != "Object::default()"
            && (ctps.contains(&ret_s) || sim::types::is_jvm_array(ret) || ret_is_class);
        return if recover { Ok(from_call(cur)?) } else { Ok(default_value()?) };
    }
    if words(&act_s).contains(&"_") && erased_shape(&act_s) == erased_shape(&ret_s) {
        // 同一擦除类型、类型实参待推断：值原样返回，`_` 由返回类型推断
        return Ok(cur);
    }
    let both_ref = !is_prim_text(&ret_s) && !is_prim_text(&act_s) && !is_obj(&ret_s) && ret_s != act_s;
    if both_ref && is_subtype(&env.ctx, &act_base, &ret_base) {
        // 返回值是类祖先的子类型：按值上转（宏 From<Self> for Ancestor）
        return match upcast_to_ancestor_instantiation(env, sim, &cur, actual, ret)? {
            Some(x) => Ok(x),
            None => Ok(Expr::upcast(cur, ir::UpcastWrap::Bare)),
        };
    }
    if both_ref && !is_obj(&act_s) {
        // 静态类型互不为子类型：经 Object 边界按声明返回类型取回（运行时校验）
        return Ok(from_call(to_object(env, cur, actual, true)?)?);
    }
    Ok(cur)
}

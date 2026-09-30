//! 算术 / 位运算 / 比较 / 数值转换（← `instr/sim/arith.py`）。
//!
//! Python 以 `RawExpr` 拼接的运算形态（`(a+b)`、`idiv(a, b)?` 等）这里同样以 [`Raw`] 承载，
//! 文本逐字一致；Python 已节点化的形态（`(a).wrapping_add(b)`、`(a as i64)`）以节点构造。

use classfile::Insn;
use ir::{Expr, Type};
use sim::{StackEntry, StackSim};
use ty::{Prim, RsType};

use crate::build::{cast, mcall, paren, text};
use crate::coerce::to_i32;
use crate::env::InstrEnv;
use crate::error::InstrResult;

#[track_caller]
fn raw(s: String) -> Expr {
    Expr::raw(s)
}

fn p(t: Prim) -> RsType {
    RsType::Prim(t)
}

/// 除数为非零正字面量（`^\(?(\d+)i(?:32|64)\)?$`）
fn safe_divisor(d: &str) -> bool {
    let d = d.strip_prefix('(').unwrap_or(d);
    let d = d.strip_suffix(')').unwrap_or(d);
    let Some(digits) = d.strip_suffix("i32").or_else(|| d.strip_suffix("i64")) else {
        return false;
    };
    !digits.is_empty() && digits.bytes().all(|b| b.is_ascii_digit()) && digits.bytes().any(|b| b != b'0')
}

/// 整数除法 / 取余（JVMS §6.5）：除数为非零正字面量时保留运算符形态，否则走运行时
/// 同名函数（除零抛 ArithmeticException、MIN / -1 回绕）
fn int_division(opcode: &str, operator: char, a: &str, b: &str) -> Expr {
    if safe_divisor(b) {
        raw(format!("({a}{operator}{b})"))
    } else {
        raw(format!("{opcode}({a}, {b})?"))
    }
}

/// `(a).wrapping_<op>(b)`
fn wrapping(a: Expr, method: &str, b: Option<Expr>) -> InstrResult<Expr> {
    mcall(paren(a), method, b.into_iter().collect())
}

/// long 运算的操作数：非 i64 → `(x as i64)`
fn as_i64(e: Expr, t: &RsType) -> Expr {
    if matches!(t, RsType::Prim(Prim::I64)) {
        e
    } else {
        cast(e, Type::I64, true)
    }
}

fn is_bool(t: &RsType) -> bool {
    matches!(t, RsType::Prim(Prim::Bool))
}

fn pop2(sim: &mut StackSim) -> InstrResult<(StackEntry, StackEntry)> {
    let b = sim.pop()?;
    let a = sim.pop()?;
    Ok((a, b))
}

/// int 二元运算（iadd..ixor）
fn int_binary(env: &InstrEnv, sim: &mut StackSim, name: &str) -> InstrResult<()> {
    let (a, b) = pop2(sim)?;
    // Java boolean & | ^（非短路）：两侧都是 boolean 时结果保持 boolean
    if matches!(name, "iand" | "ior" | "ixor") && is_bool(&a.ty) && is_bool(&b.ty) {
        let o = match name {
            "iand" => '&',
            "ior" => '|',
            _ => '^',
        };
        sim.push(raw(format!("({}{o}{})", text(env, &a.expr), text(env, &b.expr))), p(Prim::Bool));
        return Ok(());
    }
    let an = to_i32(env, a.expr, &a.ty);
    let bn = to_i32(env, b.expr, &b.ty);
    let (at, bt) = (text(env, &an), text(env, &bn));
    let e = match name {
        "iadd" => wrapping(an, "wrapping_add", Some(bn))?,
        "isub" => wrapping(an, "wrapping_sub", Some(bn))?,
        "imul" => wrapping(an, "wrapping_mul", Some(bn))?,
        "idiv" => int_division("idiv", '/', &at, &bt),
        "irem" => int_division("irem", '%', &at, &bt),
        "ishl" => raw(format!("({at}<<({bt}&0x1f))")),
        "ishr" => raw(format!("({at}>>(({bt}&0x1f)))")),
        "iushr" => raw(format!("(({at} as u32>>({bt}&0x1f)) as i32)")),
        "iand" => raw(format!("({at}&{bt})")),
        "ior" => raw(format!("({at}|{bt})")),
        _ => raw(format!("({at}^{bt})")),
    };
    sim.push(e, p(Prim::I32));
    Ok(())
}

/// long 二元运算（ladd..lxor、lcmp）
fn long_binary(env: &InstrEnv, sim: &mut StackSim, name: &str) -> InstrResult<()> {
    let (a, b) = pop2(sim)?;
    let an = as_i64(a.expr, &a.ty);
    let bn = as_i64(b.expr, &b.ty);
    let (at, bt) = (text(env, &an), text(env, &bn));
    let (e, t) = match name {
        "ladd" => (wrapping(an, "wrapping_add", Some(bn))?, Prim::I64),
        "lsub" => (wrapping(an, "wrapping_sub", Some(bn))?, Prim::I64),
        "lmul" => (wrapping(an, "wrapping_mul", Some(bn))?, Prim::I64),
        "ldiv" => (int_division("ldiv", '/', &at, &bt), Prim::I64),
        "lrem" => (int_division("lrem", '%', &at, &bt), Prim::I64),
        "land" => (raw(format!("({at}&({bt}))")), Prim::I64),
        "lor" => (raw(format!("({at}|({bt}))")), Prim::I64),
        "lxor" => (raw(format!("(({at})^({bt}))")), Prim::I64),
        _ => (raw(format!("(({at}>({bt})) as i32-(({at})<({bt})) as i32)")), Prim::I32),
    };
    sim.push(e, p(t));
    Ok(())
}

/// 不做操作数对齐的二元运算（浮点、long 移位、浮点比较）
fn plain_binary(env: &InstrEnv, sim: &mut StackSim, name: &str) -> InstrResult<()> {
    let (a, b) = pop2(sim)?;
    let (at, bt) = (text(env, &a.expr), text(env, &b.expr));
    let t = if name.starts_with('f') { Prim::F32 } else { Prim::F64 };
    let (s, t) = match name {
        "fadd" | "dadd" => (format!("({at}+{bt})"), t),
        "fsub" | "dsub" => (format!("({at}-{bt})"), t),
        "fmul" | "dmul" => (format!("({at}*{bt})"), t),
        "fdiv" | "ddiv" => (format!("({at}/{bt})"), t),
        "frem" | "drem" => (format!("({at}%({bt}))"), t),
        "lshl" => (format!("({at}).wrapping_shl(({bt}&0x3f) as u32)"), Prim::I64),
        "lshr" => (format!("({at}).wrapping_shr(({bt}&0x3f) as u32)"), Prim::I64),
        "lushr" => (format!("(({at} as u64).wrapping_shr(({bt}&0x3f) as u32) as i64)"), Prim::I64),
        _ => {
            // JVMS §6.5：NaN 时 fcmpl/dcmpl 为 -1、fcmpg/dcmpg 为 +1
            let nan = if name.ends_with('l') { "-1" } else { "1" };
            (format!("(({at}).partial_cmp(&({bt})).map_or({nan}i32, |o| o as i32))"), Prim::I32)
        }
    };
    sim.push(raw(s), p(t));
    Ok(())
}

/// 一元运算与数值转换
fn unary(env: &InstrEnv, sim: &mut StackSim, name: &str) -> InstrResult<bool> {
    let conv = |t: Prim| -> (Type, Prim) {
        (Type::Prim(crate::coerce::sim_prim(t)), t)
    };
    let target = match name {
        "i2l" | "f2l" | "d2l" => Some(conv(Prim::I64)),
        "i2f" | "l2f" | "d2f" => Some(conv(Prim::F32)),
        "i2d" | "l2d" | "f2d" => Some(conv(Prim::F64)),
        "l2i" | "f2i" | "d2i" => Some(conv(Prim::I32)),
        "ineg" | "lneg" | "fneg" | "dneg" | "i2b" | "i2s" | "i2c" => None,
        _ => return Ok(false),
    };
    let a = sim.pop()?;
    if let Some((ty, t)) = target {
        sim.push(cast(a.expr, ty, true), p(t));
        return Ok(true);
    }
    let (e, t) = match name {
        "ineg" => (wrapping(to_i32(env, a.expr, &a.ty), "wrapping_neg", None)?, Prim::I32),
        "lneg" => (wrapping(a.expr, "wrapping_neg", None)?, Prim::I64),
        "fneg" => (raw(format!("(-({}))", text(env, &a.expr))), Prim::F32),
        "dneg" => (raw(format!("(-({}))", text(env, &a.expr))), Prim::F64),
        "i2b" => (raw(format!("(({}) as i8 as i32)", text(env, &a.expr))), Prim::I32),
        "i2s" => (raw(format!("(({}) as i16 as i32)", text(env, &a.expr))), Prim::I32),
        _ => (raw(format!("(({}) as u16 as i32)", text(env, &a.expr))), Prim::I32),
    };
    sim.push(e, p(t));
    Ok(true)
}

/// 算术指令；非本组 → Ok(false)
pub fn sim_arith(env: &InstrEnv, sim: &mut StackSim, ins: &Insn) -> InstrResult<bool> {
    let name = ins.name();
    match name {
        "iadd" | "isub" | "imul" | "idiv" | "irem" | "ishl" | "ishr" | "iushr" | "iand" | "ior" | "ixor" => {
            int_binary(env, sim, name)?;
        }
        "ladd" | "lsub" | "lmul" | "ldiv" | "lrem" | "land" | "lor" | "lxor" | "lcmp" => long_binary(env, sim, name)?,
        "fadd" | "fsub" | "fmul" | "fdiv" | "frem" | "dadd" | "dsub" | "dmul" | "ddiv" | "drem" | "lshl" | "lshr"
        | "lushr" | "fcmpl" | "fcmpg" | "dcmpl" | "dcmpg" => plain_binary(env, sim, name)?,
        _ => return unary(env, sim, name),
    }
    Ok(true)
}

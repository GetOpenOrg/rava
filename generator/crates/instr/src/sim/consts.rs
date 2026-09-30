//! 常量装载（← `instr/sim/consts.py`）：iconst / lconst / fconst / dconst / bipush / sipush /
//! ldc 系列 / aconst_null。

use classfile::{Const, Insn, Operand};
use ir::{Expr, FloatLit, FloatTy, IntTy, Lit};
use sim::StackSim;
use ty::{Prim, RsType};

use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::log::{Audit, InstrLog};
use crate::text::py_float_repr;

fn int(v: i128, t: IntTy) -> Expr {
    Expr::Lit(Lit::int(v, t))
}

/// Python `_float_lit(repr(x), ty)`：有限值按 Python repr 记号加后缀，特殊值为关联常量
fn float(x: f64, t: FloatTy) -> InstrResult<Expr> {
    let value = if x.is_nan() {
        FloatLit::Nan
    } else if x.is_infinite() {
        if x > 0.0 { FloatLit::Inf } else { FloatLit::NegInf }
    } else {
        FloatLit::parse(&py_float_repr(x))?
    };
    Ok(Expr::Lit(Lit::Float { value, ty: t }))
}

fn prim(p: Prim) -> RsType {
    RsType::Prim(p)
}

/// 常量池 String：含孤立代理项（Rust `&str` 无法表示）→ UTF-16 码元数组形态
fn string_lit(s: &str) -> Lit {
    // classfile 解码为 Rust String，孤立代理项已按替换字符解码（见 GOLDEN_DIFF.md）
    Lit::JString(s.to_string())
}

fn ldc(_env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, c: &Const) -> InstrResult<()> {
    match c {
        Const::String(s) => {
            sim.push(Expr::Lit(string_lit(s)), RsType::class(ty::consts::STRING.to_string(), Vec::new()));
        }
        Const::Int(i) => {
            sim.push(int((*i).into(), IntTy::I32), prim(Prim::I32));
        }
        Const::Long(l) => {
            sim.push(int((*l).into(), IntTy::I64), prim(Prim::I64));
        }
        Const::Float(bits) => {
            sim.push(float(f64::from(f32::from_bits(*bits)), FloatTy::F32)?, prim(Prim::F32));
        }
        Const::Double(bits) => {
            sim.push(float(f64::from_bits(*bits), FloatTy::F64)?, prim(Prim::F64));
        }
        Const::Class(b) => {
            // [equiv-audit] class-literal：每次构造新 Class 对象（只计数，不改发射）
            log.audit(Audit::ClassLiteral);
            sim.push(Expr::Lit(Lit::ClassRef(b.clone())), RsType::class(ty::consts::CLASS.to_string(), Vec::new()));
        }
        // javac 只在 invokedynamic 的引导实参里使用 MethodType / MethodHandle / 动态常量，
        // 从不以 ldc 装载（JDK 与用户类同为 javac 产物）；其余编译器产出的此类 ldc 显式拒绝
        other => {
            return Err(InstrError::OutOfScope(format!("ldc 常量形态 {other:?}")));
        }
    }
    Ok(())
}

/// 常量装载指令；非本组 → Ok(false)
pub fn sim_consts(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    let name = ins.name();
    // `xconst_N` 的 N（iconst_m1 单独处理）
    let digit = || name.bytes().last().map_or(0, |b| i128::from(b) - i128::from(b'0'));
    match name {
        "iconst_m1" => {
            sim.push(int(-1, IntTy::I32), prim(Prim::I32));
        }
        _ if name.starts_with("iconst_") => {
            sim.push(int(digit(), IntTy::I32), prim(Prim::I32));
        }
        "lconst_0" | "lconst_1" => {
            sim.push(int(digit(), IntTy::I64), prim(Prim::I64));
        }
        "fconst_0" | "fconst_1" | "fconst_2" => {
            // Python `Lit(f"{n}f32")`：整数记号的浮点字面量（`0f32`）
            sim.push(int_float(digit(), FloatTy::F32)?, prim(Prim::F32));
        }
        "dconst_0" | "dconst_1" => {
            sim.push(int_float(digit(), FloatTy::F64)?, prim(Prim::F64));
        }
        "bipush" | "sipush" => {
            let Operand::Int(v) = ins.operand else {
                return Err(InstrError::BadInsn(format!("{name} 缺整数操作数")));
            };
            sim.push(int(v.into(), IntTy::I32), prim(Prim::I32));
        }
        "ldc" | "ldc_w" | "ldc2_w" => {
            let Operand::Ldc(c) = &ins.operand else {
                return Err(InstrError::BadInsn(format!("{name} 缺常量操作数")));
            };
            ldc(env, sim, log, c)?;
        }
        "aconst_null" => {
            sim.push(Expr::Lit(Lit::Null), RsType::Object);
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// 整数记号的浮点字面量（fconst / dconst：`2f32`，不经 `repr(float)`）
fn int_float(n: i128, ty: FloatTy) -> InstrResult<Expr> {
    Ok(Expr::Lit(Lit::Float { value: FloatLit::parse(&n.to_string())?, ty }))
}

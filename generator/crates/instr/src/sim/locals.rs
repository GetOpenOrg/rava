//! 局部变量装载 / 存储 / iinc（← `instr/sim/locals.py`）。

use classfile::{Insn, Operand};
use ir::{AssignStmt, Expr, Lit, Stmt, VarOrigin};
use sim::StackSim;
use ty::{Prim, RsType};

use crate::build::{id, mcall};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::text::refs_local;

/// 槽位（`_parse_slot`：`xload_N` 取名字后缀，否则取操作数）
pub fn parse_slot(ins: &Insn) -> InstrResult<u16> {
    let name = ins.name();
    if let Some((_, n)) = name.rsplit_once('_') {
        if let Ok(slot) = n.parse::<u16>() {
            return Ok(slot);
        }
    }
    match ins.operand {
        Operand::Local(s) => Ok(s),
        Operand::None => Ok(0),
        _ => Err(InstrError::BadInsn(format!("{name} 缺槽位操作数"))),
    }
}

fn load(sim: &mut StackSim, ins: &Insn, force: Option<Prim>) -> InstrResult<()> {
    let (e, t) = sim.load_local(parse_slot(ins)?)?;
    sim.push(e, force.map_or(t, RsType::Prim));
    Ok(())
}

fn store(sim: &mut StackSim, ins: &Insn, force: Option<Prim>) -> InstrResult<()> {
    let mut v = sim.pop_for_store()?;
    if let Some(p) = force {
        v.ty = RsType::Prim(p);
    }
    // int 家族到局部声明类型的对齐统一在 store_local 内完成
    sim.store_local(parse_slot(ins)?, v)?;
    Ok(())
}

/// iinc：栈上引用该变量旧值的条目先快照，再 `x = x.wrapping_add(k)`
fn iinc(env: &InstrEnv, sim: &mut StackSim, slot: u16, delta: i16) -> InstrResult<()> {
    let name = match sim.state.locals.get(&slot) {
        Some(l) => l.name.as_str().to_string(),
        None => format!("local_{slot}"),
    };
    // iload X; iinc X K（后缀自增）：栈上的 Var(name) 是递增前的值，先快照；
    // 复合栈值内嵌该变量的活引用时整体物化（JVM 语义：iinc 之前压栈的值按旧值参与运算）
    for j in 0..sim.state.stack.len() {
        let entry = sim.state.stack[j].clone();
        let snap = if let Expr::Var(v) = &entry.expr {
            if v.as_str() != name {
                continue;
            }
            sim.fresh_let(&format!("_{name}_pre"), Expr::Var(v.clone()), &entry.ty)?
        } else if refs_local(&crate::build::text(env, &entry.expr), &name) {
            sim.fresh_let(&format!("_{name}_held"), entry.expr, &entry.ty)?
        } else {
            continue;
        };
        // 新元组：身份不再与其他副本共享（Python 以对象 `is` 判定身份）
        let fresh_id = sim.state.next_id;
        sim.state.next_id += 1;
        sim.state.stack[j] = sim::StackEntry { expr: snap, ty: entry.ty, id: fresh_id };
    }
    let (method, k) = if delta >= 0 { ("wrapping_add", i32::from(delta)) } else { ("wrapping_sub", -i32::from(delta)) };
    let target = Expr::Var(id(&name)?);
    let value = mcall(target.clone(), method, vec![Expr::Lit(Lit::i32(k))])?;
    sim.emit(Stmt::Assign(AssignStmt { target, value, origin: VarOrigin::default() }));
    Ok(())
}

/// 局部变量指令；非本组 → Ok(false)
pub fn sim_locals(env: &InstrEnv, sim: &mut StackSim, ins: &Insn) -> InstrResult<bool> {
    let name = ins.name();
    let kind = name.split('_').next().unwrap_or(name);
    match kind {
        "iload" | "aload" => load(sim, ins, None)?,
        "lload" => load(sim, ins, Some(Prim::I64))?,
        "fload" => load(sim, ins, Some(Prim::F32))?,
        "dload" => load(sim, ins, Some(Prim::F64))?,
        "istore" | "astore" => store(sim, ins, None)?,
        "lstore" => store(sim, ins, Some(Prim::I64))?,
        "fstore" => store(sim, ins, Some(Prim::F32))?,
        "dstore" => store(sim, ins, Some(Prim::F64))?,
        "iinc" => {
            let Operand::Iinc { index, delta } = ins.operand else {
                return Err(InstrError::BadInsn("iinc 缺操作数".to_string()));
            };
            iinc(env, sim, index, delta)?;
        }
        _ => return Ok(false),
    }
    Ok(true)
}

//! checkcast / instanceof（← `instr/sim/control.py`）（移植中：本组指令暂返回 [`crate::error::InstrError::Unported`]）。

use classfile::Insn;
use sim::StackSim;

use crate::env::InstrEnv;
use crate::error::{unported, InstrResult};
use crate::log::InstrLog;

/// 本组指令；非本组 → Ok(false)
pub fn sim_control(_env: &InstrEnv, _sim: &mut StackSim, _log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    match ins.name() {
        "checkcast" | "instanceof" => unported(format!("{} 指令", ins.name())),
        _ => Ok(false),
    }
}

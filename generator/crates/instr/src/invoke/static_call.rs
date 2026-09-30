//! invokestatic（← `instr/invoke.py` 的 `_gen_invokestatic`）（桩：实现由并行移植提供）。

use sim::StackSim;

use crate::env::InstrEnv;
use crate::error::{unported, InstrResult};
use crate::invoke::CallRef;
use crate::log::InstrLog;

/// 静态调用 `call` 的发射
pub fn gen_invokestatic(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call: &CallRef) -> InstrResult<()> {
    let _ = (env, sim, log, call);
    unported("invokestatic")
}

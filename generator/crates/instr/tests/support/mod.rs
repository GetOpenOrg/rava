//! instr golden 回放支撑：环境重建（[`env`]）、指令视图 → NInsn（[`insn`]）、单条回放
//! （[`replay`]）与分类统计（[`tally`]）。Python 类型文本解析、状态重建与快照复用
//! sim golden 的支撑模块（`crate::sim_support`）。

pub mod env;
pub mod insn;
pub mod replay;
pub mod tally;

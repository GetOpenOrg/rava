//! 指令分发（← `instr/sim/__init__.py`）：按 Python 处理器顺序
//! consts → locals → arith → stack → fields → arrays → methods → returns → control → dynamic
//! 逐一尝试；折叠点（[`NInsn::FoldField`] / [`NInsn::FoldCall`]）先按原语义处理再压常量；
//! 未知指令发射 `panic!("stub: unsupported bytecode ..")` 存根（不得静默丢弃）。

pub mod abrupt;
pub mod arith;
pub mod arrays;
pub mod consts;
pub mod control;
pub mod dynamic;
pub mod fields;
pub mod locals;
pub mod methods;
pub mod returns;
pub mod stack;

use classfile::{Insn, Operand};
use input::NInsn;
use ir::{Expr, Lit, Stmt};
use sim::StackSim;

use crate::build::let_discard;
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::log::InstrLog;

/// 翻译一条规范化指令：语句追加到 `sim.state.stmts`，栈 / 局部变量在 `sim.state` 上演进，
/// 审计与副作用记录追加到 `log`。调用方负责先设置 `state.current_offset` / `next_offset`。
pub fn sim_instr(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &NInsn) -> InstrResult<()> {
    match ins {
        NInsn::Op(i) => sim_insn(env, sim, log, i),
        NInsn::FoldField { load, .. } => {
            // getfield 折叠点：弹出 receiver（有副作用的按求值序保留），再压常量
            stack::fold_pop(env, sim, 1)?;
            load_const(env, sim, log, load)
        }
        NInsn::FoldCall { call, load } => {
            // invoke 折叠点：调用照常翻译（被调方副作用保留），丢弃返回值，改压折叠常量
            let depth = fold_call_depth(sim, call);
            sim_insn(env, sim, log, call)?;
            if sim.state.stack.len() as i64 > depth {
                let e = sim.pop()?;
                sim.emit(let_discard(e.expr))?;
            }
            load_const(env, sim, log, load)
        }
        NInsn::NullRecv { call } => abrupt::null_recv(env, sim, call),
        NInsn::NoReturn { call } => abrupt::noreturn(env, sim, log, call),
    }
}

/// 调用前的栈深度减去实参与 receiver（调用后栈高于它 = 压了返回值）
fn fold_call_depth(sim: &StackSim, call: &Insn) -> i64 {
    let (nargs, is_static) = match &call.operand {
        Operand::Method(m, _) => (ty::type_map::parse_descriptor_params(&m.desc).len(), call.name() == "invokestatic"),
        _ => (0, false),
    };
    sim.state.stack.len() as i64 - nargs as i64 - i64::from(!is_static)
}

fn load_const(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, load: &Insn) -> InstrResult<()> {
    if !consts::sim_consts(env, sim, log, load)? {
        return Err(crate::error::InstrError::BadInsn(format!("折叠常量装载指令 {} 不是常量指令", load.name())));
    }
    Ok(())
}

pub(crate) fn sim_insn(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<()> {
    let handled = consts::sim_consts(env, sim, log, ins)?
        || locals::sim_locals(env, sim, ins)?
        || arith::sim_arith(env, sim, ins)?
        || stack::sim_stack(env, sim, ins)?
        || fields::sim_fields(env, sim, log, ins)?
        || arrays::sim_arrays(env, sim, log, ins)?
        || methods::sim_methods(env, sim, log, ins)?
        || returns::sim_returns(env, sim, log, ins)?
        || control::sim_control(env, sim, log, ins)?
        || dynamic::sim_dynamic(env, sim, log, ins)?;
    if !handled {
        // 未支持的字节码不得静默丢弃：发射存根（共享冷路径 `__stub`），运行时命中即精确报出指令
        let msg = format!("stub: unsupported bytecode {} {}", ins.name(), operand_text(&ins.operand));
        sim.emit(Stmt::Expr(crate::build::call(&["__stub"], vec![Expr::Lit(Lit::Str(msg))])?))?;
    }
    Ok(())
}

/// Python 指令对象的 operand 文本（只用于存根消息；复杂操作数不会落到存根）
fn operand_text(o: &Operand) -> String {
    match o {
        Operand::None => String::new(),
        Operand::Int(v) => v.to_string(),
        Operand::Local(s) => s.to_string(),
        Operand::Iinc { index, delta } => format!("{index}, {delta}"),
        Operand::Branch(t) => t.to_string(),
        other => format!("{other:?}"),
    }
}

//! 操作数栈指令（← `instr/sim/stack.py`）：dup 系列（同一身份，不物化）、swap、pop / pop2、
//! getfield 折叠点（`fold_const`）的 receiver 弹出。

use classfile::Insn;
use sim::{StackEntry, StackSim};
use ty::{Prim, RsType};

use crate::build::{let_discard, text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::text::looks_effectful;

/// JVM category-2 计算类型（long / double）：模拟栈中占一个条目
fn is_cat2(e: &StackEntry) -> bool {
    matches!(e.ty, RsType::Prim(Prim::I64 | Prim::F64))
}

fn take(stack: &mut Vec<StackEntry>, n: usize) -> Vec<StackEntry> {
    // 返回 [v1, v2, ..]（v1 为栈顶）
    (0..n).filter_map(|_| stack.pop()).collect()
}

fn dup_family(stack: &mut Vec<StackEntry>, name: &str) -> bool {
    let n = stack.len();
    let cat2 = |i: usize| n >= i && is_cat2(&stack[n - i]);
    // `order` 以 v1=栈顶 编号，写回自底向上
    let (pop, order): (usize, &[usize]) = match name {
        "dup" if n >= 1 => (1, &[1, 1]),
        "dup_x1" if n >= 2 => (2, &[1, 2, 1]),
        "dup2" if cat2(1) => (1, &[1, 1]),
        "dup2" if n >= 2 => (2, &[2, 1, 2, 1]),
        "dup2" if n >= 1 => (1, &[1, 1]),
        "dup_x2" if n >= 2 && cat2(2) => (2, &[1, 2, 1]),
        "dup_x2" if n >= 3 => (3, &[1, 3, 2, 1]),
        "dup_x2" if n == 2 => (2, &[1, 2, 1]),
        "dup2_x1" if n >= 2 && cat2(1) => (2, &[1, 2, 1]),
        "dup2_x1" if n >= 3 => (3, &[2, 1, 3, 2, 1]),
        "dup2_x2" if n >= 2 && cat2(1) && cat2(2) => (2, &[1, 2, 1]),
        "dup2_x2" if n >= 3 && cat2(1) => (3, &[1, 3, 2, 1]),
        "dup2_x2" if n >= 3 && cat2(3) => (3, &[2, 1, 3, 2, 1]),
        "dup2_x2" if n >= 4 => (4, &[2, 1, 4, 3, 2, 1]),
        "swap" if n >= 2 => (2, &[1, 2]),
        "dup" | "dup_x1" | "dup2" | "dup_x2" | "dup2_x1" | "dup2_x2" | "swap" => return true,
        _ => return false,
    };
    let vs = take(stack, pop);
    stack.extend(order.iter().map(|&k| vs[k - 1].clone()));
    true
}

/// 弹出值有副作用时按求值序保留为 `let _ = e;`
fn keep_effect(env: &InstrEnv, sim: &mut StackSim, e: StackEntry) -> InstrResult<()> {
    if looks_effectful(&text(env, &e.expr)) {
        sim.emit(let_discard(e.expr))?;
    }
    Ok(())
}

/// getfield 折叠点：弹出 `npop` 个 receiver（有副作用的按求值序保留）；常量由调用方压入
pub fn fold_pop(env: &InstrEnv, sim: &mut StackSim, npop: usize) -> InstrResult<()> {
    let n = npop.min(sim.state.stack.len());
    let mut popped = Vec::with_capacity(n);
    for _ in 0..n {
        popped.push(sim.pop()?);
    }
    for e in popped.into_iter().rev() {
        keep_effect(env, sim, e)?;
    }
    Ok(())
}

/// 栈指令；非本组 → Ok(false)
pub fn sim_stack(env: &InstrEnv, sim: &mut StackSim, ins: &Insn) -> InstrResult<bool> {
    let name = ins.name();
    if dup_family(&mut sim.state.stack, name) {
        return Ok(true);
    }
    match name {
        "pop" => {
            if !sim.state.stack.is_empty() {
                let e = sim.pop()?;
                keep_effect(env, sim, e)?;
            }
        }
        "pop2" => {
            // category-2 栈顶只弹一项；两个 category-1 弹两项
            if let Some(top) = sim.state.stack.last() {
                let top_cat2 = is_cat2(top);
                sim.pop()?;
                if !top_cat2 && !sim.state.stack.is_empty() {
                    sim.pop()?;
                }
            }
        }
        _ => return Ok(false),
    }
    Ok(true)
}

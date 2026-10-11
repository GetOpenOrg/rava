//! 方法内数据流：每条指令入口一帧，合流点取并；异常处理器入口取抛出值之并。
//!
//! 必抛剪枝：区域只在正常出口整组释放，异常离开区域时只放掉登记引用（对象照常按引用计数存活）。
//! 因此「必然以未捕获异常离开本方法」的指令段，在其异常不可能于区域动态范围内被捕获时（本摘要
//! 的全部调用点都不受处理器覆盖，见 [`Analysis::catch_ctx`]）其效果不影响整组释放的正确性，跳过不分析。

use std::collections::HashMap;

use classfile::{op, Method, Operand};
use input::{NInsn, NormCode};
use ty::registry::ClassInfo;

use crate::analysis::{reject, Analysis, R};
use crate::step::step;
use crate::val::{Frame, Val, FRESH_UNKNOWN, OLD};

enum Exit {
    Return,
    Throw,
    Next(Vec<u32>, bool),
}

/// 指令的静态后继：`Next(跳转目标, 是否顺延)`
fn exit_of(insn: &NInsn) -> Exit {
    let i = match insn {
        NInsn::Op(i) => i,
        NInsn::FoldField { .. } | NInsn::FoldCall { .. } => return Exit::Next(Vec::new(), true),
        NInsn::NullRecv { .. } | NInsn::NoReturn { .. } | NInsn::NoClassDef { .. } => return Exit::Throw,
    };
    match i.opcode {
        0xac..=0xb1 | op::JSR | op::JSR_W | op::RET => Exit::Return,
        op::ATHROW => Exit::Throw,
        op::GOTO | op::GOTO_W => match i.operand {
            Operand::Branch(t) => Exit::Next(vec![t], false),
            _ => Exit::Return,
        },
        0x99..=0xa6 | op::IFNULL | op::IFNONNULL => match i.operand {
            Operand::Branch(t) => Exit::Next(vec![t], true),
            _ => Exit::Return,
        },
        _ => match &i.operand {
            Operand::TableSwitch { .. } | Operand::LookupSwitch { .. } => {
                Exit::Next(input::norm::switch_targets(&i.operand), false)
            }
            _ => Exit::Next(Vec::new(), true),
        },
    }
}

/// 各指令是否必然以本方法内未捕获的异常结束（不会正常返回）
fn must_throw(code: &NormCode, pos: &HashMap<u32, usize>) -> Vec<bool> {
    let n = code.insns.len();
    let covered: Vec<bool> = code
        .insns
        .iter()
        .map(|x| code.exception_table.iter().any(|e| e.start <= x.offset() && x.offset() < e.end))
        .collect();
    let exits: Vec<Exit> = code.insns.iter().map(exit_of).collect();
    let mut mt: Vec<bool> = (0..n).map(|i| !covered[i] && !matches!(exits[i], Exit::Return)).collect();
    loop {
        let mut changed = false;
        for i in (0..n).rev() {
            if !mt[i] {
                continue;
            }
            let Exit::Next(ts, fall) = &exits[i] else { continue };
            let succ_ok = ts.iter().all(|t| pos.get(t).is_some_and(|&j| mt[j]))
                && (!fall || (i + 1 < n && mt[i + 1]));
            if !succ_ok || (ts.is_empty() && !fall) {
                mt[i] = false;
                changed = true;
            }
        }
        if !changed {
            return mt;
        }
    }
}

/// 方法体数据流；返回各 return 值之并（None = 不正常返回）
pub fn run_method<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, m: &'a Method, locals: Vec<Val>) -> R<Option<Val>> {
    let Some(code) = a.w.input.code(ci.name(), m) else { return Ok(None) };
    let frames = flow(a, ci, &code, locals)?;
    Ok(frames.1)
}

/// 方法体各指令入口帧（浅模式下用于候选识别）
pub fn entry_frames<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, code: &NormCode, locals: Vec<Val>) -> R<Vec<Option<Frame>>> {
    Ok(flow(a, ci, code, locals)?.0)
}

type Flow = (Vec<Option<Frame>>, Option<Val>);

fn flow<'a>(a: &mut Analysis<'_, 'a>, ci: &'a ClassInfo, code: &NormCode, mut locals: Vec<Val>) -> R<Flow> {
    let n = code.insns.len();
    if n == 0 {
        return Ok((Vec::new(), None));
    }
    let pos: HashMap<u32, usize> = code.insns.iter().enumerate().map(|(i, x)| (x.offset(), i)).collect();
    let at = |off: u32| -> R<usize> {
        match pos.get(&off) {
            Some(&i) => Ok(i),
            None => reject(format!("跳转目标不在指令边界：{off}")),
        }
    };
    if locals.len() < code.max_locals as usize {
        locals.resize(code.max_locals as usize, Val::Void);
    }
    let mut frames: Vec<Option<Frame>> = vec![None; n];
    let mut inq = vec![false; n];
    let mut work = vec![0usize];
    frames[0] = Some(Frame { locals, stack: Vec::new() });
    inq[0] = true;
    let mut ret: Option<Val> = None;
    let skip = if a.shallow || a.catch_ctx { vec![false; n] } else { must_throw(code, &pos) };
    let covered: Vec<bool> = code
        .insns
        .iter()
        .map(|x| code.exception_table.iter().any(|e| e.start <= x.offset() && x.offset() < e.end))
        .collect();
    let merge = |frames: &mut Vec<Option<Frame>>, work: &mut Vec<usize>, inq: &mut Vec<bool>, j: usize, f: Frame| -> R<()> {
        let grew = match &mut frames[j] {
            None => {
                frames[j] = Some(f);
                true
            }
            Some(old) => match old.join_in(&f) {
                Some(c) => c,
                None => {
                    return reject(format!(
                        "合流栈形不一致：@{}（栈 {:?} / {:?}）",
                        code.insns[j].offset(),
                        old.stack,
                        f.stack
                    ))
                }
            },
        };
        if grew && !inq[j] {
            inq[j] = true;
            work.push(j);
        }
        Ok(())
    };
    while let Some(i) = work.pop() {
        inq[i] = false;
        if skip[i] {
            continue;
        }
        a.tick(1)?;
        a.site_covered = a.catch_ctx || covered[i];
        let f = frames[i].clone().expect("入队指令有帧");
        let off = code.insns[i].offset();
        for e in &code.exception_table {
            if e.start <= off && off < e.end {
                let mut hv = a.thrown.clone();
                hv.insert(OLD);
                hv.insert(FRESH_UNKNOWN);
                let hf = Frame { locals: f.locals.clone(), stack: vec![Val::Ref(hv)] };
                merge(&mut frames, &mut work, &mut inq, at(e.handler)?, hf)?;
            }
        }
        let out = step(a, ci, &code.insns[i], f)?;
        if let Some(nf) = out.fall {
            if i + 1 < n {
                merge(&mut frames, &mut work, &mut inq, i + 1, nf)?;
            }
        }
        for (t, tf) in out.jumps {
            merge(&mut frames, &mut work, &mut inq, at(t)?, tf)?;
        }
        if let Some(r) = out.ret {
            match &mut ret {
                Some(old) => {
                    old.join_in(&r);
                }
                None => ret = Some(r),
            }
        }
    }
    Ok((frames, ret))
}


//! 区域候选识别与判定。
//!
//! 候选 `[s, e]`（用户方法内，按指令偏移）：
//! - 处在一个基本块内（`s` 之后无跳转目标 / 处理器入口），不受任何异常处理器覆盖；
//! - `s` 前操作数栈为空；区间内没有局部变量写、分支、return、athrow、监视器指令；
//! - `e` 是返回基本类型或 void 的调用，执行后栈上只剩它的结果——区域内产生的引用都已被消费。
//!
//! 判定：区域代码以「区域前已有」的局部变量起步做全局逃逸分析，未被拒绝且区域对象之间成环
//! （引用计数回收不了）才插入区域。

use classfile::{op, Insn, Method, Operand};
use input::{NInsn, NormCode};
use ty::registry::ClassInfo;

use crate::analysis::{Analysis, R};
use crate::flow::entry_frames;
use crate::step::step;
use crate::val::{Frame, Val};
use crate::world::{method_kinds, Kind, World};

/// 区间内不允许的指令（局部变量写、控制流、监视器）
fn barrier(insn: &NInsn) -> bool {
    let NInsn::Op(i) = insn else { return insn.is_abrupt() };
    matches!(i.opcode, 0x36..=0x4e | op::IINC | 0x99..=0xb1 | op::ATHROW | op::MONITORENTER | op::MONITOREXIT
        | op::IFNULL | op::IFNONNULL | op::GOTO_W | op::JSR_W)
}

fn targets(i: &Insn) -> Vec<u32> {
    match &i.operand {
        Operand::Branch(t) => vec![*t],
        Operand::TableSwitch { .. } | Operand::LookupSwitch { .. } => input::norm::switch_targets(&i.operand),
        _ => Vec::new(),
    }
}

/// 以返回基本类型 / void 的调用结束的指令：返回其结果类别（void 为 None）
fn prim_call(insn: &NInsn) -> Option<Option<Kind>> {
    let NInsn::Op(i) = insn else { return None };
    let desc = match &i.operand {
        Operand::Method(m, _) => &m.desc,
        Operand::InvokeDynamic { desc, .. } => desc,
        _ => return None,
    };
    let rk = method_kinds(desc).1;
    (rk != Some(Kind::Ref)).then_some(rk)
}

fn candidates(code: &NormCode, frames: &[Option<Frame>]) -> Vec<(usize, usize)> {
    let n = code.insns.len();
    let pos = |off: u32| code.insns.iter().position(|x| x.offset() == off);
    let mut leader = vec![false; n + 1];
    let mut covered = vec![false; n];
    for e in &code.exception_table {
        if let Some(h) = pos(e.handler) {
            leader[h] = true;
        }
        for (k, x) in code.insns.iter().enumerate() {
            if e.start <= x.offset() && x.offset() < e.end {
                covered[k] = true;
            }
        }
    }
    for x in &code.insns {
        if let NInsn::Op(i) = x {
            for t in targets(i) {
                if let Some(j) = pos(t) {
                    leader[j] = true;
                }
            }
        }
    }
    let depth = |k: usize| frames.get(k).and_then(|f| f.as_ref()).map(|f| f.stack.len());
    let mut out = Vec::new();
    let mut s = 0;
    while s < n {
        if depth(s) != Some(0) || covered[s] || barrier(&code.insns[s]) {
            s += 1;
            continue;
        }
        let mut best = None;
        let mut j = s;
        while j < n && (j == s || !leader[j]) && !covered[j] && !barrier(&code.insns[j]) && depth(j).is_some() {
            if let Some(rk) = prim_call(&code.insns[j]) {
                let want = usize::from(rk.is_some());
                if depth(j + 1) == Some(want) {
                    best = Some(j);
                }
            }
            j += 1;
        }
        match best {
            Some(e) => {
                out.push((s, e));
                s = e + 1;
            }
            None => s += 1,
        }
    }
    out
}

/// 用户方法的区域：`[(起始偏移, 结束偏移)]`（含两端指令）
/// `trace`：逐候选打印判定（`rava build --escape-trace`）
pub fn plan_regions(w: &World, ci: &ClassInfo, m: &Method, trace: bool) -> Vec<(u32, u32)> {
    if m.name == "<clinit>" {
        return Vec::new();
    }
    let Some(code) = w.input.code(ci.name(), m) else { return Vec::new() };
    let live = vec![Val::Live; code.max_locals as usize];
    let mut shallow = Analysis::new(w, true);
    let frames = match entry_frames(&mut shallow, ci, &code, live.clone()) {
        Ok(f) => f,
        Err(r) => {
            if trace {
                eprintln!("[escape] {}.{}{}：入口帧推导失败：{}", ci.name(), m.name, m.desc, r.0);
            }
            return Vec::new();
        }
    };
    let mut out = Vec::new();
    for (s, e) in candidates(&code, &frames) {
        let (so, eo) = (code.insns[s].offset(), code.insns[e].offset());
        let mut an = Analysis::new(w, false);
        let res: R<()> = an.fixpoint(|a| {
            let mut f = Frame { locals: live.clone(), stack: Vec::new() };
            for k in s..=e {
                match step(a, ci, &code.insns[k], f)?.fall {
                    Some(nf) => f = nf,
                    None => break,
                }
            }
            Ok(())
        });
        let verdict = match &res {
            Err(r) => format!("拒绝：{}", r.0),
            Ok(()) if an.has_cycle() => {
                out.push((so, eo));
                "插入区域".to_string()
            }
            Ok(()) => "无成环对象，不插入".to_string(),
        };
        if trace {
            eprintln!("[escape] {}.{}{} @{so}..={eo}：{verdict}（{} 节点，{} 步）", ci.name(), m.name, m.desc, an.nodes.len(), an.steps);
        }
    }
    out
}

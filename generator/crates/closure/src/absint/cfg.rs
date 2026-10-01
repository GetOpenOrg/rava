//! absint：基本块控制流图（按指令偏移查询）。
//!
//! 用途：拼接链的构建器被复用时（循环体内先清空再追加），判断「追加点之前最近一次清空」是否在每条路径上
//! 都成立——即从追加点出发、不经过任何清空点能否回到追加点（能 = 上一轮内容可能残留，拆段不成立）。
//! 异常边按保守处理：块被某 try 区间覆盖时，从块首即可进入其处理器（块内清空点之前也可能抛出）。


use classfile::{insn, Code, Operand};

#[derive(Debug, Default)]
pub struct Cfg {
    /// 各块首指令偏移（升序）
    starts: Vec<u32>,
    /// 各块末指令偏移
    ends: Vec<u32>,
    /// 正常后继块
    succ: Vec<Vec<usize>>,
    /// 异常处理器块（块内任一指令被覆盖即计入）
    handlers: Vec<Vec<usize>>,
}

impl Cfg {
    pub fn build(code: &Code) -> Cfg {
        let insns = &code.insns;
        let n = insns.len();
        if n == 0 {
            return Cfg::default();
        }
        // 指令按偏移升序：偏移 → 下标用二分（免逐次分析建表）
        let idx = |off: &u32| insns.binary_search_by_key(off, |x| x.offset).ok();
        let targets = |o: &Operand| -> Vec<u32> {
            match o {
                Operand::Branch(t) => vec![*t],
                Operand::TableSwitch { default, targets, .. } => targets.iter().copied().chain([*default]).collect(),
                Operand::LookupSwitch { default, pairs } => pairs.iter().map(|p| p.1).chain([*default]).collect(),
                _ => vec![],
            }
        };
        let mut leader = vec![false; n];
        leader[0] = true;
        for (i, ins) in insns.iter().enumerate() {
            let ts = targets(&ins.operand);
            for t in &ts {
                if let Some(j) = idx(t) {
                    leader[j] = true;
                }
            }
            if (!ts.is_empty() || insn::is_terminal(ins.opcode)) && i + 1 < n {
                leader[i + 1] = true;
            }
        }
        for h in &code.exception_table {
            if let Some(j) = idx(&h.handler) {
                leader[j] = true;
            }
        }
        let first: Vec<usize> = (0..n).filter(|&i| leader[i]).collect();
        let block_of_insn = |i: usize| first.partition_point(|&s| s <= i) - 1;
        let mut cfg = Cfg::default();
        for (b, &s) in first.iter().enumerate() {
            let e = first.get(b + 1).map_or(n, |&x| x) - 1;
            let last = &insns[e];
            let mut succ: Vec<usize> = targets(&last.operand).iter().filter_map(|t| idx(t)).map(block_of_insn).collect();
            if !insn::is_terminal(last.opcode) && e + 1 < n {
                succ.push(b + 1);
            }
            succ.sort_unstable();
            succ.dedup();
            let (lo, hi) = (insns[s].offset, last.offset);
            let mut hs: Vec<usize> = code
                .exception_table
                .iter()
                .filter(|h| h.start <= hi && lo < h.end)
                .filter_map(|h| idx(&h.handler))
                .map(block_of_insn)
                .collect();
            hs.sort_unstable();
            hs.dedup();
            cfg.starts.push(lo);
            cfg.ends.push(hi);
            cfg.succ.push(succ);
            cfg.handlers.push(hs);
        }
        cfg
    }

    fn block(&self, off: u32) -> Option<usize> {
        let b = self.starts.partition_point(|&s| s <= off).checked_sub(1)?;
        (off <= self.ends[b]).then_some(b)
    }

    /// 从偏移 p 之后出发，不经过 `clear` 中任一偏移能否再次到达 p
    pub fn recurs_avoiding(&self, p: u32, clear: &[u32]) -> bool {
        let Some(bp) = self.block(p) else { return true };
        let blocked_in = |b: usize, from: u32, to: u32| clear.iter().any(|&c| self.block(c) == Some(b) && c >= from && c < to);
        let mut seen = vec![false; self.starts.len()];
        let mut work: Vec<usize> = self.handlers[bp].clone();
        // p 所在块的剩余部分：其中有清空点则正常后继被挡住
        if !clear.iter().any(|&c| self.block(c) == Some(bp) && c > p) {
            work.extend(&self.succ[bp]);
        }
        while let Some(b) = work.pop() {
            if std::mem::replace(&mut seen[b], true) {
                continue;
            }
            work.extend(&self.handlers[b]);
            let start = self.starts[b];
            // 块首到 p（p 在本块时）之间没有清空点：回到了 p
            if b == bp && !blocked_in(b, start, p) {
                return true;
            }
            if blocked_in(b, start, self.ends[b] + 1) {
                continue;
            }
            work.extend(&self.succ[b]);
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use classfile::{op, Insn};

    fn i(offset: u32, opcode: u8, operand: Operand) -> Insn {
        Insn { offset, opcode, operand }
    }

    /// 循环：0 init；2 头部判定 → 20 出口；5 清空（可选）；8 追加；11 goto 2
    fn loop_code() -> Code {
        let insns = vec![
            i(0, 0x00, Operand::None),
            i(2, 0x99, Operand::Branch(20)),
            i(5, 0x00, Operand::None),
            i(8, 0x00, Operand::None),
            i(11, op::GOTO, Operand::Branch(2)),
            i(20, op::RETURN, Operand::None),
        ];
        Code { max_stack: 1, max_locals: 1, code_len: 21, insns, exception_table: vec![] }
    }

    #[test]
    fn loop_append_needs_reset_in_body() {
        let c = Cfg::build(&loop_code());
        // 只有循环外的 init 作清空点：第二轮追加时上一轮内容残留
        assert!(c.recurs_avoiding(8, &[0]));
        // 循环体内追加前有清空：每轮都从空开始
        assert!(!c.recurs_avoiding(8, &[0, 5]));
        // 追加不在循环内
        assert!(!c.recurs_avoiding(20, &[0]));
    }
}

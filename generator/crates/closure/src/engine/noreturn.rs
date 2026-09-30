//! 引擎：「尚无返回」答复的收尾（返回常量格 `rvals` 缺席时调用点的答复）。
//!
//! 乐观阶段：被调方法尚无返回路径即按不返回答复（调用之后不可达），答复过的方法记入 `never`。
//! 队列排空后进入收尾：导出的不可达代码只从跳转 / switch / return / athrow 之后开始（folds 规则 7），
//! 所以「不返回」的答复要换成值未知重算。收尾期间继续入链的方法同样会遇到尚未分析的被调方法——
//! 这时的「尚无返回」不是定论：按值未知答复会把 Top 并入调用方的返回常量格，被调方法随后给出常量也
//! 无法撤回（格只升不降），结果依赖处理顺序。因此收尾阶段区分两种缺席：
//! - 被调方法还没有节点（调用方的调用边尚未接上）、有节点尚无分析结果，或其当前分析自身含未定论的
//!   「不返回」答复：仍按不返回答复并记入 `never`，下次排空时重算；
//! - 否则（全部节点已分析且没有返回路径）：定论，按值未知求值。
//! 每次排空重算 `never` 中的方法，直到为空；两次排空的 `never` 相同（互相等待的环、接收者恒空而永不建节点的目标）
//! 或收尾轮数达到上限时转为全部按值未知，保证终止。

use super::*;

#[derive(Default)]
pub(super) struct NoReturn {
    /// 已建节点的字节码方法
    created: HashSet<MemberRef>,
    /// 字节码方法：尚无分析结果的节点数（缺席 = 0）
    unanalyzed: HashMap<MemberRef, u32>,
    /// 当前分析含未定论「不返回」答复的节点数（缺席 = 0）
    waiting: HashMap<MemberRef, u32>,
    /// 收尾阶段：乐观假设已关
    closing: bool,
    /// 定论阶段：缺席一律按值未知
    settled: bool,
    last: Option<BTreeSet<usize>>,
    rounds: u32,
}

/// 收尾轮数上限（停滞判定之外的终止保证）
const MAX_ROUNDS: u32 = 32;

fn inc(m: &mut HashMap<MemberRef, u32>, k: &MemberRef) {
    *m.entry(k.clone()).or_default() += 1;
}

fn dec(m: &mut HashMap<MemberRef, u32>, k: &MemberRef) {
    if let Some(n) = m.get_mut(k) {
        *n -= 1;
        if *n == 0 {
            m.remove(k);
        }
    }
}

impl NoReturn {
    /// 被调方法返回常量格缺席时，本次答复是否按不返回（并记入 `never`）
    pub(super) fn answer_never(&self, t: &MemberRef) -> bool {
        if !self.closing {
            return true;
        }
        !self.settled && (!self.created.contains(t) || self.unanalyzed.contains_key(t) || self.waiting.contains_key(t))
    }

    /// 排空时的收尾步：返回要重算的方法（空 = 结束）
    pub(super) fn drain(&mut self, never: BTreeSet<usize>) -> Vec<usize> {
        self.waiting.clear();
        if never.is_empty() {
            return vec![];
        }
        if !self.closing {
            self.closing = true;
        } else if self.last.as_ref() == Some(&never) || self.rounds >= MAX_ROUNDS {
            self.settled = true;
        }
        self.rounds += 1;
        let out = never.iter().copied().collect();
        self.last = Some(never);
        out
    }
}

impl<'a> Engine<'a> {
    /// 新建字节码方法节点
    pub(super) fn nr_created(&self, key: &MemberRef) {
        let mut nr = self.ctx.noreturn.borrow_mut();
        nr.created.insert(key.clone());
        inc(&mut nr.unanalyzed, key);
    }

    /// 节点的分析结果被丢弃（失效）
    pub(super) fn nr_dropped(&self, m: usize) {
        if self.methods[m].kind == Kind::Bytecode {
            let key = self.methods[m].key.clone();
            inc(&mut self.ctx.noreturn.borrow_mut().unanalyzed, &key);
        }
    }

    /// 节点开始分析：上一次分析的「不返回」答复作废
    pub(super) fn nr_begin(&self, m: usize) {
        if self.ctx.never.borrow_mut().remove(&m) {
            let key = self.methods[m].key.clone();
            dec(&mut self.ctx.noreturn.borrow_mut().waiting, &key);
        }
    }

    /// 节点分析完成
    pub(super) fn nr_end(&self, m: usize) {
        let key = self.methods[m].key.clone();
        let waiting = self.ctx.never.borrow().contains(&m);
        let mut nr = self.ctx.noreturn.borrow_mut();
        if self.methods[m].kind == Kind::Bytecode {
            dec(&mut nr.unanalyzed, &key);
        }
        if waiting {
            inc(&mut nr.waiting, &key);
        }
    }

    /// 队列排空（补种已收敛）：还有要重算的方法时返回 true
    pub(super) fn nr_drain(&mut self) -> bool {
        let never = std::mem::take(&mut *self.ctx.never.borrow_mut());
        let redo = self.ctx.noreturn.borrow_mut().drain(never);
        if redo.is_empty() {
            return false;
        }
        self.ctx.stats.borrow_mut().mark_rss("optimistic");
        for m in redo {
            self.invalidate(m, Why::Never);
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(n: &str) -> MemberRef {
        MemberRef { owner: "a/A".into(), name: n.into(), desc: "()I".into() }
    }

    /// 乐观阶段一律不返回；收尾阶段只对未分析 / 等待中的被调方法不返回；环停滞后转定论
    #[test]
    fn closing_answers_and_stall() {
        let mut nr = NoReturn::default();
        let (t, u) = (key("t"), key("u"));
        assert!(nr.answer_never(&t));
        assert!(nr.drain(BTreeSet::new()).is_empty());
        let never: BTreeSet<usize> = [1, 2].into_iter().collect();
        assert_eq!(nr.drain(never.clone()), vec![1, 2]);
        assert!(nr.answer_never(&t), "尚无节点的目标不是定论");
        nr.created.insert(t.clone());
        nr.created.insert(u.clone());
        assert!(!nr.answer_never(&t));
        inc(&mut nr.unanalyzed, &t);
        inc(&mut nr.waiting, &u);
        assert!(nr.answer_never(&t) && nr.answer_never(&u));
        dec(&mut nr.unanalyzed, &t);
        assert!(!nr.answer_never(&t));
        // 同一 never 集合再次出现：停滞，转定论
        assert_eq!(nr.drain(never.clone()), vec![1, 2]);
        assert!(!nr.answer_never(&u));
        assert!(nr.drain(BTreeSet::new()).is_empty());
    }
}

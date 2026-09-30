//! 引擎：辅助分析记忆的上下文无关性。
//!
//! `<clinit>` 常量、构造器摘要、属性读取摘要、常量实参求值都按键记忆，但计算时可能被两种截断影响：
//! 递归保护（键已在进行中 → 按未知处理）与常量实参求值的深度上限。截断取决于外层正在计算什么，
//! 同一个键在不同外层下会得到不同答复；若把这种答复记忆下来，结果就随处理次序变化。
//!
//! 规则：
//! - 每次记忆化计算是一帧，层号 = 进入时进行中的帧数。递归保护命中键 K 时记下 K 所在帧的层号（截断层）；
//!   一帧的子树里截断层都不低于本帧层号时，其结果与外层无关（截断只来自本帧自身的递归），可以记忆，
//!   否则只供本次使用、不写入记忆。
//! - 深度上限：`<clinit>` 常量 / 构造器摘要 / 属性读取摘要从深度 0 开始计算（与外层深度无关），
//!   常量实参求值的记忆键带起始深度。
//!
//! 由此记忆里的每个答复都等于「在空上下文中计算该键」的答复，与处理次序无关。

use super::*;

/// 进行中的记忆化计算帧
pub(super) struct Frame {
    key: String,
    level: usize,
    saved_cut: usize,
    saved_depth: u32,
}

/// 帧栈：进行中的键 → 层号；子树里递归保护命中的最低层号（无命中 = usize::MAX）
#[derive(Default)]
pub(super) struct Guards {
    active: HashMap<String, usize>,
    cut: usize,
}

impl Guards {
    pub(super) fn new() -> Guards {
        Guards { active: HashMap::default(), cut: usize::MAX }
    }

    /// 进入键 key 的计算；已在进行中 → 记下截断层并返回 None
    fn enter(&mut self, key: String, depth: u32) -> Result<Frame, ()> {
        if let Some(&l) = self.active.get(&key) {
            self.cut = self.cut.min(l);
            return Err(());
        }
        let level = self.active.len();
        self.active.insert(key.clone(), level);
        let saved_cut = std::mem::replace(&mut self.cut, usize::MAX);
        Ok(Frame { key, level, saved_cut, saved_depth: depth })
    }

    /// 离开帧；返回本帧结果是否与外层无关（可记忆）
    fn leave(&mut self, f: &Frame) -> bool {
        self.active.remove(&f.key);
        let c = self.cut;
        self.cut = f.saved_cut.min(c);
        c >= f.level
    }
}

impl Ctx<'_> {
    /// 进入记忆化计算；`fresh` = 从深度 0 开始（结果与外层深度无关）
    pub(super) fn memo_enter(&self, key: String, fresh: bool) -> Option<Frame> {
        let depth = self.ceval_depth.get();
        let f = self.guards.borrow_mut().enter(key, depth).ok()?;
        if fresh {
            self.ceval_depth.set(0);
        }
        Some(f)
    }

    /// 离开记忆化计算，恢复深度；返回结果是否可记忆
    pub(super) fn memo_leave(&self, f: Frame) -> bool {
        self.ceval_depth.set(f.saved_depth);
        self.guards.borrow_mut().leave(&f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cut_below_frame_blocks_memo() {
        let mut g = Guards::new();
        let a = g.enter("a".into(), 0).ok().unwrap();
        let b = g.enter("b".into(), 0).ok().unwrap();
        // b 的子树里撞上进行中的 a：b 的答复取决于外层在算 a
        assert!(g.enter("a".into(), 0).is_err());
        assert!(!g.leave(&b));
        // 对 a 自身而言是自递归截断：与外层无关
        assert!(g.leave(&a));
    }

    #[test]
    fn self_recursion_keeps_memo() {
        let mut g = Guards::new();
        let a = g.enter("a".into(), 0).ok().unwrap();
        let b = g.enter("b".into(), 0).ok().unwrap();
        assert!(g.enter("b".into(), 0).is_err());
        assert!(g.leave(&b));
        // 兄弟帧不受此前子树的截断影响
        let c = g.enter("c".into(), 0).ok().unwrap();
        assert!(g.leave(&c));
        assert!(g.leave(&a));
        assert!(g.active.is_empty());
    }
}

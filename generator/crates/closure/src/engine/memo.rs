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

/// 一次记忆化计算的外部输入：读时尚未放开的字段（字段放开后答复可能变化）、
/// 是否查询过系统属性不折叠集合（集合增长后答复可能变化）。
/// 嵌套计算与记忆命中的输入并入外层，于是每条记忆的输入记录覆盖其全部传递输入
#[derive(Default)]
pub(super) struct MemoRec {
    reads: Vec<MemberRef>,
    taint: bool,
}

/// 记忆条目附带的输入记录。id ≠ 0：可记忆且可能作废的条目编号——方法体分析取用时只登记该编号
/// （`Dep::Memo`），条目作废时经 `memo_consumers` 令取用者失效；id = 0：不可记忆（每次重算）或
/// 永不作废（无输入）
#[derive(Clone, Default)]
pub(super) struct Inputs {
    pub(super) reads: Rc<[MemberRef]>,
    pub(super) taint: bool,
    pub(super) id: u32,
}

impl Ctx<'_> {
    /// 进入记忆化计算；`fresh` = 从深度 0 开始（结果与外层深度无关）
    pub(super) fn memo_enter(&self, key: String, fresh: bool) -> Option<Frame> {
        let depth = self.ceval_depth.get();
        let f = self.guards.borrow_mut().enter(key, depth).ok()?;
        if fresh {
            self.ceval_depth.set(0);
        }
        self.mrecs.borrow_mut().push(MemoRec::default());
        Some(f)
    }

    /// 离开记忆化计算，恢复深度；返回结果是否可记忆与本帧的输入记录
    /// （调用方取用结果时以 `memo_use` 并入外层）
    pub(super) fn memo_leave(&self, f: Frame) -> (bool, Inputs) {
        self.ceval_depth.set(f.saved_depth);
        let mut r = self.mrecs.borrow_mut().pop().unwrap_or_default();
        r.reads.sort_unstable();
        r.reads.dedup();
        let clean = self.guards.borrow_mut().leave(&f);
        let id = if clean && (r.taint || !r.reads.is_empty()) {
            let n = self.memo_next.get() + 1;
            self.memo_next.set(n);
            n
        } else {
            0
        };
        (clean, Inputs { reads: r.reads.into(), taint: r.taint, id })
    }

    /// 取用一份记忆化结果：外层是方法体分析（me）→ 登记依赖；外层是另一次记忆化计算 → 输入并入其记录。
    /// 方法体分析的依赖：记忆条目 → 条目编号（条目作废时失效）；不可记忆的结果 → 读过的字段（放开 / 写入时
    /// 失效）与属性读者（不折叠集合增长时失效：其嵌套取用的记忆可能随之作废）
    pub(super) fn memo_use(&self, me: Option<usize>, inp: &Inputs) {
        match me {
            Some(me) if inp.id != 0 => self.dep(me, Dep::Memo(inp.id)),
            Some(me) => {
                for f in inp.reads.iter() {
                    self.dep(me, Dep::Field(f.clone()));
                }
                if inp.taint || !inp.reads.is_empty() {
                    self.dep(me, Dep::Props);
                }
            }
            None => {
                if let Some(top) = self.mrecs.borrow_mut().last_mut() {
                    top.reads.extend(inp.reads.iter().cloned());
                    top.taint |= inp.taint;
                }
            }
        }
    }

    /// 辅助分析（m = None）读到尚未放开的字段：计入当前记忆化计算的输入
    pub(super) fn note_aux_read(&self, f: &MemberRef) {
        if let Some(top) = self.mrecs.borrow_mut().last_mut() {
            top.reads.push(f.clone());
        }
    }

    /// 查询系统属性不折叠集合：方法体分析登记为属性读者，辅助分析计入当前记忆化计算的输入
    pub(super) fn note_props(&self, me: Option<usize>) {
        match me {
            Some(me) => self.dep(me, Dep::Props),
            None => {
                if let Some(top) = self.mrecs.borrow_mut().last_mut() {
                    top.taint = true;
                }
            }
        }
    }

    /// 作废条目 ids 的取用者（编号不复用，登记随之清除）
    pub(super) fn memo_consumers(&self, ids: impl IntoIterator<Item = u32>) -> BTreeSet<usize> {
        let mut md = self.mdeps.borrow_mut();
        ids.into_iter().filter(|&i| i != 0).flat_map(|i| md.remove(&i).unwrap_or_default()).collect()
    }

    /// 不折叠集合增长后答复可能变化的记忆：查询过该集合，或读过的字段此后已放开
    pub(super) fn memo_stale(&self, inp: &Inputs, open: &mut HashMap<MemberRef, bool>) -> bool {
        inp.taint
            || inp.reads.iter().any(|r| match open.get(r) {
                Some(&o) => o,
                None => {
                    let o = self.field_info(r).is_none_or(|fi| self.field_open(&fi));
                    open.insert(r.clone(), o);
                    o
                }
            })
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

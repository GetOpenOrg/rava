//! 引擎：枢纽 lambda 读者的归属——读者效果归枢纽，与锚点调用点无关。
//!
//! 枢纽上的 lambda 接收者只在锚点（首个接入的字节码调用点）以 `HP` / `HR` 为实参 / 结果各建一个读者（`hub.rs`）。
//! 锚点随接入先后而定，读者若按锚点调用点记账，结果就随处理顺序变化（batch-1011b `closure_independent_of_order`：
//! DeepCopy 的 lambda 实现目标只记在先接入的那个调用点上）。故读者的一切按调用点的效果都改归枢纽：
//! - 接边目标记入枢纽的 `ltargets`，报告中计入枢纽的每个接入点（与经枢纽中转的 `plain` 同口径），不写锚点的派发表；
//! - 读者接入的枢纽（方法引用的虚分派）记入 `lhubs`，不写锚点的枢纽表，其目标同样计入本枢纽的每个接入点；
//! - 读者接边时不取锚点方法的克隆上下文与调用点字面常量（实参本就是全部接入点在 `HP` 上的汇合）：
//!   静态目标不继承调用方上下文、不按调用点克隆，中继方法不继承调用方上下文，选择子不按调用点克隆。
//!
//! 读者归属：锚点派发期间显式指定（`reader_over`），读者单元此后的增量接边按其登记（`hub_reader_lcalls`）。
//!
//! 多个枢纽可锚定在同一调用点（调用点接入的精确集合枢纽与其父链）。锚点调用点上的去重记录因此一律带上所属枢纽，
//! 否则后到枢纽的读者被先到者的记录挡掉、效果记到先到者名下，结果仍随处理顺序变化（DeepCopy batch 4096 时
//! IntConsumer 调用点上 `ReferencePipeline.lambda$collect$1` 的有无）：
//! - lambda 读者单元的键（`LambdaKey`）含所属枢纽读者；`hub_lsent` 按（偏移, 接收者, 枢纽）记；
//! - 方法引用逐接收者派发的 `dispatched` 去重按读者单元（`lcalls` 下标）而非 lambda；
//! - 读者再接入已接入的枢纽时，接入关系同样记入其枢纽的 `lhubs`。

use super::*;

impl Engine<'_> {
    /// 当前接边所属的枢纽 lambda 读者（None = 调用点自身接边）
    pub(super) fn reader_hub(&self) -> Option<u32> {
        match self.reader_over {
            Some(x) => x,
            None => self.cur_lcall.and_then(|id| self.hub_reader_lcalls.get(&id).copied()),
        }
    }

    /// 以显式归属 host 执行 f（枢纽锚点派发、按接入记录重接边）
    pub(super) fn with_reader<R>(&mut self, host: Option<u32>, f: impl FnOnce(&mut Self) -> R) -> R {
        let outer = self.reader_over.replace(host);
        let r = f(self);
        self.reader_over = outer;
        r
    }

    /// lambda 读者单元 id 的增量接边：归属按其登记，不沿用外层的显式归属
    pub(super) fn reader_scope<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        let outer = self.reader_over.take();
        let r = f(self);
        self.reader_over = outer;
        r
    }

    /// 新建的 lambda 读者单元登记归属
    pub(super) fn reader_register(&mut self, id: u32) {
        if let Some(h) = self.reader_hub() {
            self.hub_reader_lcalls.insert(id, h);
        }
    }

    /// 调用边的目标记账：枢纽读者的记入枢纽，否则记入调用点派发表（增长时通知派发集查询者）
    pub(super) fn note_target(&mut self, m: usize, off: u32, t: usize) {
        if let Some(h) = self.reader_hub() {
            self.hubs[h as usize].ltargets.insert(t);
            return;
        }
        if self.dispatch.entry((m, off)).or_default().insert(t) {
            self.ctx.stats.borrow_mut().sprof.dispatch_new += 1;
            self.vdisp_note(m, off, Some(t));
        }
    }

    /// 调用点接入枢纽 h 的记账：枢纽读者代接的记入其枢纽，否则记入调用点枢纽表（并标派发集不透明）
    pub(super) fn note_hub_site(&mut self, host: Option<u32>, h: u32, m: usize, off: u32) {
        if let Some(g) = host {
            self.hubs[g as usize].lhubs.insert(h);
            return;
        }
        self.hub_sites.entry((m, off)).or_default().insert(h);
        self.vdisp_note(m, off, None);
    }
}

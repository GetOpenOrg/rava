//! 引擎：方法对象上下文的克隆预算（计划 2026-09-30-closure-analyzer-performance.md P5 / §4.10）。
//!
//! 同一成员按抽象对象上下文（接收者对象，或静态辅助方法继承的调用方对象）建出的克隆达到 `CTX_BUDGET` 后，
//! 新到的对象上下文不再各建一个克隆，而按「类型 + 堆上下文尾」合并为类型上下文（`^类|尾`）：同一类、同一属主链的
//! 接收者共用一个克隆。接收者仍逐个以 `Recv::Exact` 注入合并克隆的 this，分派按各接收者的确切类型进行；
//! 合并只让同类同属主的不同分配点共享形参、返回值与克隆体内的分配。
//! 不回落无上下文本体：本体汇合全部接收者与按成员的返回常量格，精度崩塌（试验 929de3b8）。
//! 类型上下文像对象一样作堆上下文：克隆体内的分配以 `^类|尾` 为链段，按类型与属主分开。
//! 档位上下文、具体求值上下文、调用点 / 形参常量上下文不参与合并。
//!
//! 预算取 e2e 语料实测每成员对象上下文克隆数的上界之上（DeepCopy / TestSerialLookupPairing 最多 1736，
//! `ConcurrentHashMap$Node.<init>`），语料用例不触发，闭包与不设预算时逐项相同；只在框架规模的档案上生效。

use super::*;

/// 同一成员按抽象对象上下文建克隆的上限；超出后按类型上下文合并
const CTX_BUDGET: u32 = 2048;

impl Engine<'_> {
    /// 建节点前对对象上下文做预算收口（见模块说明）
    pub(super) fn budget_ctx(&mut self, key: &MemberRef, ctx: u32) -> u32 {
        if ctx == NOCTX || !self.objs.contains_key(&ctx) || self.ctx_heap.contains_key(&ctx) || self.level_ctxs.contains_key(&ctx) {
            return ctx;
        }
        // 已建的克隆照常沿用（同一成员同一上下文始终落到同一节点）
        if self.methods.contains_key(&(key.clone(), ctx)) {
            return ctx;
        }
        let n = self.ctx_fine.entry(key.clone()).or_default();
        if *n < CTX_BUDGET {
            *n += 1;
            return ctx;
        }
        self.ctx_stats.merged += 1;
        self.typed_ctx(ctx)
    }

    /// 对象上下文 r 的类型上下文：`^类|堆上下文尾`（尾 = r 的分配点链去掉首段）
    fn typed_ctx(&mut self, r: u32) -> u32 {
        let tid = self.objs[&r];
        let tail = self.obj_chain.get(&r).and_then(|c| c.split_once('#').map(|(_, t)| t.to_string())).unwrap_or_default();
        let name = format!("^{}|{tail}", self.names[tid as usize]);
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        let id = self.id(&name);
        let seg: Rc<str> = Rc::from(name.as_str());
        self.seg_cls.entry(seg.clone()).or_insert(tid);
        self.obj_chain.insert(id, seg);
        self.typed_ctxs.insert(id, tid);
        self.ctx_stats.typed += 1;
        id
    }

    /// 上下文 c 作为堆上下文时的类型（抽象对象或类型上下文）
    pub(super) fn ctx_obj_cls(&self, c: u32) -> Option<u32> {
        self.objs.get(&c).or_else(|| self.typed_ctxs.get(&c)).copied()
    }
}

/// 预算收口计数（诊断）
#[derive(Default)]
pub(super) struct CtxStats {
    /// 超预算按类型上下文合并的次数
    pub(super) merged: u64,
    /// 建出的类型上下文数
    pub(super) typed: u64,
}

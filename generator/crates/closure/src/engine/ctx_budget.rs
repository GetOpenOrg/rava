//! 引擎：方法上下文的预算与合并（计划 2026-09-30-closure-analyzer-performance.md P5 / §4.10）。
//!
//! 克隆上下文由 `ctxsel.rs` 按调用形态选出；本文件在建方法节点前对选出的上下文再做两步收口：
//!
//! 1. **上下文无关方法不克隆**（`ctx_free`）：字节码方法返回 void，方法体只做局部变量 / 常量 / 算术 / 比较 / 分支，
//!    异常表为空，调用只有 `invokespecial` / `invokestatic` 且目标同样上下文无关（典型：`Object.<init>` 与只调用
//!    超类无参构造器的构造器链）。这类方法不读写任何字段 / 元素、不分配、不返回值、不抛出，克隆体与本体的效果都只是
//!    「到达其被调方」，克隆不带来任何精度：一律进本体。闭包结果与克隆时逐项相同。
//! 2. **对象上下文预算**（`CTX_BUDGET`）：同一成员按抽象对象上下文（接收者对象，或静态辅助方法继承的调用方对象）
//!    建出的克隆达到预算后，新到的对象上下文不再各建一个克隆，而按「类型 + 堆上下文尾」合并为类型上下文
//!    （`^类|尾`）：同一类、同一属主链的接收者共用一个克隆。接收者仍逐个以 `Recv::Exact` 注入合并克隆的 this，
//!    分派按各接收者的确切类型进行；合并只让同类同属主的不同分配点共享形参与返回值。
//!    不回落无上下文本体：本体汇合全部接收者，容器读写跨属主混合（试验 929de3b8 已证实精度崩塌）。
//!    类型上下文像对象一样作堆上下文：克隆体内的分配以 `^类|尾` 为链段，按类型与属主分开。
//!    档位上下文、具体求值上下文、调用点 / 形参常量上下文不参与合并。
//!
//! 健全性：克隆只是把方法节点按上下文拆分；合并克隆的形参取自其全部调用方实参之并，任何一组上下文的取法都是
//! 原程序行为的上界。预算只在成员克隆数超过 `CTX_BUDGET` 时生效，常规用例（e2e 语料）不触发。

use super::*;

/// 同一成员按抽象对象上下文建克隆的上限；超出后按类型上下文合并
const CTX_BUDGET: u32 = 256;

impl Engine<'_> {
    /// 建节点前对上下文做预算收口（见模块说明）
    pub(super) fn budget_ctx(&mut self, key: &MemberRef, ctx: u32) -> u32 {
        if ctx == NOCTX || ctx == self.concrete.ctx || self.level_ctxs.contains_key(&ctx) {
            return ctx;
        }
        if self.ctx_free(key) {
            self.ctx_stats.free += 1;
            return NOCTX;
        }
        if !self.objs.contains_key(&ctx) || self.ctx_heap.contains_key(&ctx) {
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

    /// 上下文无关方法（见模块说明）。按成员缓存；递归成环处取否
    pub(super) fn ctx_free(&mut self, key: &MemberRef) -> bool {
        if let Some(&r) = self.ctx_free_memo.get(key) {
            return r;
        }
        self.ctx_free_memo.insert(key.clone(), false);
        let r = self.ctx_free_uncached(key);
        self.ctx_free_memo.insert(key.clone(), r);
        r
    }

    fn ctx_free_uncached(&mut self, key: &MemberRef) -> bool {
        // 有选择子形参的方法按形参常量剪枝分支（`selector.rs`），汇合后可能多出可达被调方
        if !key.desc.ends_with(")V") || self.ctx.selector_slots(key) != 0 {
            return false;
        }
        let Some(cf) = self.h.class(&key.owner) else { return false };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return false };
        if self.kind_of(&cf, meth) != Kind::Bytecode {
            return false;
        }
        let Some(code) = meth.code.as_ref() else { return false };
        if !code.exception_table.is_empty() {
            return false;
        }
        let mut callees = Vec::new();
        for i in &code.insns {
            match i.opcode {
                // nop、常量（不含 ldc）、局部变量读写、栈操作
                0x00..=0x11 | 0x15..=0x2d | 0x36..=0x4e | 0x57..=0x5f => {}
                // 整数除 / 取余可抛 ArithmeticException
                0x6c | 0x6d | 0x70 | 0x71 => return false,
                // 算术、iinc、类型转换、比较、条件 / 无条件分支、switch、void return
                0x60..=0x98 | 0x99..=0xa7 | 0xaa | 0xab | 0xb1 => {}
                // invokespecial / invokestatic：目标须同样上下文无关
                0xb7 | 0xb8 => {
                    let classfile::insn::Operand::Method(mr, iface) = &i.operand else { return false };
                    let Some(site) = self.h.resolve_method(&mr.owner, &mr.name, &mr.desc, *iface) else { return false };
                    let (o, n, d) = site.key();
                    callees.push(MemberRef { owner: o, name: n, desc: d });
                }
                _ => return false,
            }
        }
        callees.iter().all(|c| c != key && self.ctx_free(c))
    }
}

/// 预算收口计数（诊断）
#[derive(Default)]
pub(super) struct CtxStats {
    /// 上下文无关方法进本体的次数
    pub(super) free: u64,
    /// 超预算按类型上下文合并的次数
    pub(super) merged: u64,
    /// 建出的类型上下文数
    pub(super) typed: u64,
}

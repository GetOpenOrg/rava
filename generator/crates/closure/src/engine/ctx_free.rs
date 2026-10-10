//! 引擎：上下文无关方法不克隆（计划 2026-09-30-closure-analyzer-performance.md P5 / §4.10）。
//!
//! 字节码方法返回 void，方法体只做局部变量 / 常量 / 算术 / 比较 / 分支，异常表为空，无选择子形参，
//! 调用只有 `invokespecial` / `invokestatic` 且目标同样上下文无关（典型：`Object.<init>` 与只调用超类无参构造器的
//! 构造器链）。这类方法不读写任何字段 / 元素、不分配、不返回值、不抛出、不做虚调用，克隆体与本体的效果都只是
//! 「到达其被调方」，克隆不带来任何精度：一律进本体。
//!
//! 不能推广到返回基本类型或读字段 / 做虚调用的方法（试验 1a0111c1，§4.10）：调用点上下文与对象上下文的克隆
//! 携带逐调用点的返回常量（`site_rets.rs`）与逐对象的字段常量，调用方据此剪枝分支；并入本体后常量汇合，
//! DeepCopy 闭包多出 2085 个类。

use super::*;

impl Engine<'_> {
    /// 建节点前的上下文收口：上下文无关方法进本体（见模块说明）
    pub(super) fn free_ctx(&mut self, key: &MemberRef, ctx: u32) -> u32 {
        if ctx == NOCTX || ctx == self.concrete.ctx || self.level_ctxs.contains_key(&ctx) {
            return ctx;
        }
        if self.ctx_free(key) {
            self.ctx_free_hits += 1;
            return NOCTX;
        }
        ctx
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

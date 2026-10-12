//! 引擎：选择性克隆上下文——上下文平方法（计划 2026-09-30-closure-analyzer-performance.md §4.13）。
//!
//! §4.13 的实测：继承（位 1）/ 接收者（位 2）两路**建点时急切并入本体**在 DeepCopy 上必然精度崩塌
//! （超时 / 18 GiB OOM）——逐上下文的形参常量格（`pvals` / `pstr`）是语料精度的主通道，并入即常量汇合。
//! §4.14 ①「常量保持的合并」实施后，位 1 / 位 2 的语义改为**排空点等价类合并的通道开关**
//! （`eqmerge.rs`）：克隆照常建立（精度与基线相同），工作队列排空时只把「类型流 + 常量载体 + 出入边」
//! 全同的克隆经 `scc.rs` 并路径合并——常量不同的上下文被载体签名区分、不会被并入本体。
//! 按调用点新建的上下文（工厂 / 转发 / 选择子常量克隆、档位 / 具体求值）不参与合并。
//!
//! 位 4（lambda 创建上下文收口）保留 §4.13 的急切语义（诊断开关）；位 8 反射池静态成员短路、
//! 位 16 宽档（容器分配放行）不变。判据（`ctx_flat`，按字节码、建节点前、与求值次序无关）见下。

use super::*;

impl Engine<'_> {
    /// 继承 / 接收者来源的上下文：位 1 / 2 不再建点时并入本体（§4.13 实测的精度崩塌通道），
    /// 只由 `eqmerge.rs` 在排空点按等价类合并；位 4（lambda 创建上下文）保留急切收口。
    /// 档位与具体求值上下文不收口（引导查询折叠 / 具体求值不与本体共享摘要）
    pub(super) fn flat_ctx(&mut self, key: &MemberRef, ctx: u32, bit: u8) -> u32 {
        if ctx == NOCTX || ctx == self.concrete.ctx || self.level_ctxs.contains_key(&ctx) || self.ctx_flat_mode & bit == 0 {
            return ctx;
        }
        if bit & 3 != 0 {
            // 位 1 / 2：等价类合并通道（`eqmerge.rs`），建点不收口
            return ctx;
        }
        if self.ctx_flat(key) {
            self.ctx_flat_hits += 1;
            return NOCTX;
        }
        ctx
    }

    /// 上下文平方法（见模块说明）。按成员缓存
    pub(super) fn ctx_flat(&mut self, key: &MemberRef) -> bool {
        if let Some(&r) = self.ctx_flat_memo.get(key) {
            return r;
        }
        let r = self.ctx_flat_uncached(key);
        self.ctx_flat_memo.insert(key.clone(), r);
        r
    }

    fn ctx_flat_uncached(&mut self, key: &MemberRef) -> bool {
        let Some(cf) = self.h.class(&key.owner) else { return false };
        let Some(meth) = cf.method(&key.name, &key.desc) else { return false };
        if self.kind_of(&cf, meth) != Kind::Bytecode {
            return false;
        }
        let Some(code) = meth.code.as_ref() else { return false };
        if self.ctx.selector_slots(key) != 0 {
            return false;
        }
        for i in &code.insns {
            match i.opcode {
                // nop、常量（含 ldc / ldc2_w）、局部变量读写、栈操作
                0x00..=0x2d | 0x36..=0x4e | 0x57..=0x5f => {}
                // 数组读写与分配（newarray / anewarray / multianewarray / arraylength）：
                // 数组分配点只按站点（`array_site`，无堆上下文链），元素节点全调用方共享，与克隆无关
                0x2e..=0x35 | 0x4f..=0x56 | 0xbc | 0xbd | 0xc5 | 0xbe => {}
                // 算术（除可抛 ArithmeticException 的整除 / 取余）、iinc、转换、比较、分支、switch、返回、
                // athrow、checkcast / instanceof、监视器、ifnull / ifnonnull
                0x60..=0x6b | 0x6e..=0x6f | 0x72..=0xa7 | 0xaa | 0xab | 0xac..=0xb1 | 0xbf..=0xc3 | 0xc6 | 0xc7 => {}
                // 静态字段：节点按成员全局键
                0xb2 | 0xb3 => {}
                // 实例字段：克隆携带逐对象字段常量，并入即汇合
                0xb4 | 0xb5 => return false,
                // new：非容器类分配无堆身份（异常等）；容器类分配点带堆上下文链，宽档（位 16）放行（粗化）
                0xbb => {
                    let classfile::insn::Operand::Class(c) = &i.operand else { return false };
                    if self.ctx_flat_mode & 16 == 0 && self.container(c) {
                        return false;
                    }
                }
                // 虚 / 接口 / 特殊调用：目标按接收者值选择，与调用方上下文无关
                0xb6 | 0xb7 | 0xb9 => {}
                // 静态调用：目标不得是按调用点克隆（工厂 / 转发）或带选择子的方法（堆划分互不涵盖）
                0xb8 => {
                    let classfile::insn::Operand::Method(mr, iface) = &i.operand else { return false };
                    let Some(site) = self.h.resolve_method(&mr.owner, &mr.name, &mr.desc, *iface) else { return false };
                    let (o, n, d) = site.key();
                    let t = MemberRef { owner: o, name: n, desc: d };
                    if !static_heap(&mr.desc) {
                        continue;
                    }
                    if self.fresh_factory(&t) || self.forwarder(&t) || self.ctx.selector_slots(&t) != 0 {
                        return false;
                    }
                }
                // invokedynamic（lambda 创建带上下文）、jsr / ret、wide、整除 / 取余
                _ => return false,
            }
        }
        true
    }
}

/// 描述符的返回或形参含引用（`static_ctx` 的 heap 判定，决定目标是否继承调用方上下文）
fn static_heap(desc: &str) -> bool {
    parse_method(desc).is_some_and(|md| md.ret.iter().chain(&md.params).any(|r| r.is_reference()))
}

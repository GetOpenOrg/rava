//! 引擎：上下文透明方法不克隆（计划 2026-09-30-closure-analyzer-performance.md P5 / §4.10）。
//!
//! 克隆上下文由 `ctxsel.rs` 按调用形态选出（接收者对象、继承的调用方上下文、调用点）。克隆的精度来自两处：
//! 克隆体内的值不与其他上下文汇合后**流出**（返回值、字段 / 元素 / 静态写、传给被调方的引用实参），
//! 以及克隆体内的分配按上下文命名。一个方法若这两处都没有，其各克隆体的效果之并与无上下文本体相同，
//! 克隆只增加节点而不带来精度——这类方法称为**上下文透明**，一律进本体：
//!
//! - 返回 void 或基本类型；字节码方法、异常表为空、无选择子形参（`selector.rs` 按形参常量剪枝的方法不算）；
//! - 方法体只有局部变量 / 常量 / 算术 / 比较 / 分支 / 字段与元素读取 / 类型检查，不分配、不写字段 / 元素 / 静态、
//!   不抛出（整数除 / 取余也排除）、不含 invokedynamic；
//! - 虚调用（invokevirtual / invokeinterface）的实参全为基本类型：接收者逐个以 `Recv::Exact` 注入被调方，
//!   被调方按接收者对象选上下文，与克隆体内的分派结果之并相同；被调方的返回值只在本方法内使用；
//! - invokespecial / invokestatic 的目标同样上下文透明（递归判定，成环处取否）。
//!
//! 典型：`Object.<init>` 与只调超类无参构造器的构造器链、`Math.min`、`HashMap.hash`、`Integer.numberOfLeadingZeros`。
//! 形参常量上下文、档位上下文、具体求值上下文保留（常量折叠与档位语义依赖它们）。

use super::*;

impl Engine<'_> {
    /// 建节点前的上下文收口：上下文透明方法的对象 / 调用点上下文改进本体（见模块说明）
    pub(super) fn free_ctx(&mut self, key: &MemberRef, ctx: u32) -> u32 {
        if ctx == NOCTX || ctx == self.concrete.ctx || self.level_ctxs.contains_key(&ctx) || self.ctx_heap.contains_key(&ctx) {
            return ctx;
        }
        if self.ctx_free(key) {
            self.ctx_free_hits += 1;
            return NOCTX;
        }
        ctx
    }

    /// 上下文透明方法（见模块说明）。按成员缓存；递归成环处取否
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
        if !prim_ret(&key.desc) || self.ctx.selector_slots(key) != 0 {
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
                // 整数除 / 取余可抛 ArithmeticException
                0x6c | 0x6d | 0x70 | 0x71 => return false,
                // nop、常量（含 ldc）、局部变量与元素读取、局部变量写、栈操作、算术、iinc、类型转换、比较、分支、switch
                0x00..=0x4e | 0x57..=0xab => {}
                // 基本类型 / void 返回
                0xac..=0xaf | 0xb1 => {}
                // getstatic / getfield、arraylength、checkcast / instanceof、monitor、ifnull / ifnonnull
                0xb2 | 0xb4 | 0xbe | 0xc0..=0xc3 | 0xc6 | 0xc7 => {}
                // 虚调用：实参须全为基本类型
                0xb6 | 0xb9 => {
                    let classfile::insn::Operand::Method(mr, _) = &i.operand else { return false };
                    if !prim_args(&mr.desc) {
                        return false;
                    }
                }
                // invokespecial / invokestatic：目标须同样上下文透明
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

/// 描述符的返回类型为 void 或基本类型
fn prim_ret(desc: &str) -> bool {
    desc.rsplit_once(')').is_some_and(|(_, r)| r.len() == 1 && "VZBCSIJFD".contains(r))
}

/// 描述符的形参全为基本类型
fn prim_args(desc: &str) -> bool {
    desc.strip_prefix('(').and_then(|d| d.split_once(')')).is_some_and(|(p, _)| !p.contains(['L', '[']))
}

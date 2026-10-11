//! 引擎：选择性克隆上下文——上下文平方法（计划 2026-09-30-closure-analyzer-performance.md §4.13）。
//!
//! S0 / sb3 的主增长项是方法上下文克隆（§4.12：S0 A 60.2 万、sb3 45.1 万，其中约 75% 是对象上下文），
//! 而其中大部分克隆不携带任何按对象区分的信息。本文件按字节码结构预先（建节点前、与求值次序无关）判定
//! 哪些方法的克隆是「平」的：所有经**继承**（静态辅助方法取调用方上下文）与**接收者**（实例方法按接收者
//! 抽象对象克隆、反射池按接收者派发）得来的上下文一律并入本体；按调用点新建的上下文（工厂 / 转发方法的
//! 调用点克隆、选择子常量克隆、档位 / 具体求值上下文）不在收口范围内，逐调用点的分离保持不变。
//!
//! 并入本体为什么是精度单调（只会变宽、不会漏）：
//! - 形参来源并集 ⊇ 各克隆分别的来源；形参常量（`pvals`）按节点汇合，并入即取并；
//! - 返回值经 `R(t)` 汇到所接调用点，本体被全部调用方接边，结果是各克隆之并的超集；
//! - 方法体内读写的静态字段 / 数组元素节点按成员 / 分配点全局键，与克隆无关；
//! - 容器分配点链取分配方法的上下文（`obj_at`）：本体内分配即按站点合并，各克隆链上是其细化
//!   （对象更少、字段值更窄），并入是堆划分的粗化；
//! - 虚分派按接收者**值**选择目标，与调用方上下文无关：合并只让接收者值集变宽，目标只会多不会少。
//! 因此收口只会让闭包变大或不變（§4.9 试验 929de3b8 的失败即精度变宽的代价），四例集合一致须实测验证。
//!
//! 判据（`ctx_flat`，`RAVA_CTX_FLAT`：0 关 / 1 紧（缺省）/ 2 宽）：
//! - 字节码方法、无选择子形参（选择子克隆与调用点绑定）；
//! - 不读写实例字段：克隆携带逐对象的字段常量（`oracle.getfield` 按接收者回答），并入即汇合（§4.9）；
//! - 紧档不分配容器对象（分配点堆上下文链是按对象精度的主通道）；宽档放行（并入即按站点粗化）；
//! - `invokedynamic` 不参与（lambda 创建带上下文）；jsr / ret / wide、可抛 ArithmeticException 的整除取余不参与
//!   （与 `ctx_free` 同口径）；
//! - `invokestatic` 的目标经继承取调用方上下文：目标是新鲜工厂 / 分派转发方法 / 带选择子形参时，
//!   调用方并入本体会让目标从「按对象继承」换成「按调用点克隆」，堆划分互不涵盖，不允许；
//!   其余目标（含引用签名）被并入本体后按同一粗化口径取本体，安全。

use super::*;

impl Engine<'_> {
    /// 继承 / 接收者来源的上下文收口：key 是上下文平方法时并入本体（`NOCTX`）。
    /// 档位与具体求值上下文不收口（引导查询折叠 / 具体求值不与本体共享摘要）
    pub(super) fn flat_ctx(&mut self, key: &MemberRef, ctx: u32) -> u32 {
        if ctx == NOCTX || ctx == self.concrete.ctx || self.level_ctxs.contains_key(&ctx) || self.ctx_flat_mode == 0 {
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
                // new：非容器类分配无堆身份（异常等）；容器类分配点带堆上下文链，宽档放行（粗化）
                0xbb => {
                    let classfile::insn::Operand::Class(c) = &i.operand else { return false };
                    if self.ctx_flat_mode < 2 && self.container(c) {
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

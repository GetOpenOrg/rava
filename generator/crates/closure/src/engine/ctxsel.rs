//! 引擎：克隆上下文的选择策略（计划 2026-09-30-closure-analyzer-performance.md P3「上下文克隆收拢为单一策略点」）。
//!
//! 方法节点按 (成员, 上下文) 区分；上下文是一个抽象对象或调用点命名的堆上下文，`NOCTX` 即方法本体。
//! 选择规则全部在本文件，调用方只描述调用的形态：
//!
//! - **接收者**（`recv_ctx`）：实例方法按接收者抽象对象克隆（对象敏感，容器对象各进其克隆）；非对象接收者进本体。
//! - **静态调用**（`static_ctx`，按 `Call` 形态）：
//!   1. 字节码 `invokestatic`：返回引用或有引用形参的辅助方法继承调用方上下文（返回值与经实参写入的字段 / 元素
//!      按容器对象分开，如 `casTabAt(tab, i, null, node)` 只写进本容器的表）；只有基本类型形参与返回的
//!      按本体共享；上下文无关的调用方调用新鲜工厂（`fresh_factory`）按调用点克隆。
//!      选择子形参（`selector.rs`）上传常量时按调用点克隆、链尾接调用方上下文；否则调用方在上下文中则继承。
//!   2. lambda 静态实现方法：继承 lambda 创建时的上下文。
//!   3. 方法句柄常量（`MethodHandle` 静态引用）：本体。
//!   4. 以上结果为本体、调用方是字节码方法、被调是分派转发方法（`forward.rs`）时：按调用点克隆
//!      （调用方在上下文中则继承之，转发链随最外层调用点分开）。
//!   5. lambda 创建点预建实现方法节点（`Call::Eager`）：继承创建方上下文，不做 4。
//!
//! 调用点上下文（`site_ctx` / `site_ctx_in`）是以调用点命名的堆上下文，克隆体内的容器分配以它为链首。

use super::*;

/// 静态调用的形态
pub(super) enum Call<'x> {
    /// 字节码 invokestatic：返回类型或形参是否含引用（可能经返回值 / 实参对象的字段与元素读写调用方上下文中的对象）、
    /// 实参（不含接收者）
    Invoke { heap: bool, args: &'x [V] },
    /// lambda 静态实现方法的 SAM 调用：lambda 创建时的上下文
    Lambda(u32),
    /// 方法句柄常量引用的静态方法
    Handle,
    /// lambda 创建点预建实现方法节点
    Eager,
}

impl Engine<'_> {
    /// 接收者 r 调用实例方法时的上下文
    /// （字段句柄来源标记只是身份标签、不是堆抽象，不作上下文）
    pub(super) fn recv_ctx(&self, r: u32) -> u32 {
        if self.objs.contains_key(&r) && !self.fh_marks.contains_key(&r) {
            r
        } else {
            NOCTX
        }
    }

    /// 调用点 (m, off) 以形态 call 调用静态方法 key 时的上下文。在建节点前判定，不建出无调用方的本体
    pub(super) fn static_ctx(&mut self, m: usize, off: u32, key: &MemberRef, call: Call) -> u32 {
        let caller = self.methods[m].ctx;
        let ctx = match call {
            Call::Invoke { heap, args } => {
                let c = match caller {
                    _ if !heap => NOCTX,
                    NOCTX if self.fresh_factory(key) => self.site_ctx(m, off),
                    c => c,
                };
                self.selector_ctx(m, off, key, args).unwrap_or(c)
            }
            Call::Lambda(c) => c,
            Call::Handle => NOCTX,
            Call::Eager => return caller,
        };
        if ctx == NOCTX && self.methods[m].kind == Kind::Bytecode && self.forwarder(key) {
            return match caller {
                NOCTX => self.site_ctx(m, off),
                c => c,
            };
        }
        ctx
    }

    /// 按选择子形参克隆的上下文（None = 不克隆）
    fn selector_ctx(&mut self, m: usize, off: u32, key: &MemberRef, args: &[V]) -> Option<u32> {
        if self.methods[m].kind != Kind::Bytecode {
            return None;
        }
        let mask = self.ctx.selector_slots(key);
        if mask == 0 {
            return None;
        }
        let c = self.methods[m].ctx;
        if selector::const_selector(mask, args) {
            // 同一调用方（克隆）里不同调用点传不同常量：按调用点分开，链尾接调用方上下文
            Some(self.site_ctx_in(m, off, c))
        } else {
            (c != NOCTX).then_some(c)
        }
    }

    /// 调用点上下文：以调用点命名的堆上下文（不是对象，不进入值集），克隆体内的容器分配以它为链首
    pub(super) fn site_ctx(&mut self, m: usize, off: u32) -> u32 {
        self.site_ctx_in(m, off, NOCTX)
    }

    /// 调用点上下文，链尾接外层上下文 outer 的链（截断到 HEAP_DEPTH；outer = NOCTX 即 `site_ctx`）
    fn site_ctx_in(&mut self, m: usize, off: u32, outer: u32) -> u32 {
        let mut chain = format!("@{}:{off}", self.mbase[&self.methods[m].key]);
        if outer != NOCTX {
            for seg in self.obj_chain.get(&outer).map_or("", |c| &**c).split('#').filter(|g| !g.is_empty()).take(HEAP_DEPTH - 1) {
                chain.push('#');
                chain.push_str(seg);
            }
        }
        if let Some(&id) = self.ids.get(chain.as_str()) {
            return id;
        }
        let id = self.id(&chain);
        self.obj_chain.insert(id, Rc::from(chain));
        id
    }
}

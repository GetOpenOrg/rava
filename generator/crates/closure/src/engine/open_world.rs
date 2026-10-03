//! 引擎：开放世界折叠（T1 档案化，计划 `2026-10-01-cross-test-compile-reuse.md` §3.3）。
//!
//! 档案只依赖 JDK 侧事实：依赖「实例化集合」的折叠（`null_recv`）只对用户无法扩展的类型做。
//! 非用户方法里的虚 / 接口调用点，若接收者的静态类型（调用属主）可被用户扩展，则本次分析里
//! 接收者集合为空也不判 null_recv——别的用户程序可能把自己的子类 / 实现类对象传到这里。
//!
//! 这样的调用点照常翻译为虚调用：属主须有方法载体（至少 L2，见 `levels.rs`），被调方法不在调用链上时
//! 由发射层按调用链外方法发精确存根；接收者真为 null 时由生成代码的空值检查抛 NPE。
//! 用户方法里的调用点不受影响（用户 crate 按各自程序生成）。
//!
//! 可被用户扩展（[`Engine::user_extensible`]）：
//! - 接口；非 final、且有 public / protected 构造器的类；
//! - sealed 类型：任一许可子类可被扩展；
//! - JDK 的非公开类型不可扩展；依赖库（`--lib`）的非公开类型可经同名包扩展，按可扩展处理；
//! - 数组不可扩展；找不到类文件的按可扩展（保守）。

use super::*;

/// sealed 许可链的递归深度上限（防御环状许可）
const SEALED_DEPTH: u8 = 8;

impl Engine<'_> {
    /// 方法 m 的虚调用点（属主 owner）按开放世界处理：调用方不是用户类，且属主可被用户扩展
    pub(super) fn open_site(&self, m: usize, owner: &str) -> bool {
        self.domain(&self.methods[m].key.owner) != Domain::User && self.user_extensible(owner)
    }

    pub(super) fn user_extensible(&self, cls: &str) -> bool {
        self.extensible_at(cls, 0)
    }

    fn extensible_at(&self, cls: &str, depth: u8) -> bool {
        if cls.starts_with('[') {
            return false;
        }
        let Some(cf) = self.h.class(cls) else { return true };
        let origin = self.cp.origin(cls);
        if origin == Some(Origin::User) {
            return true;
        }
        if !cf.permitted_subclasses.is_empty() {
            return depth < SEALED_DEPTH && cf.permitted_subclasses.iter().any(|s| self.extensible_at(s, depth + 1));
        }
        let lib = origin == Some(Origin::Lib);
        if cf.access & acc::PUBLIC == 0 && !lib {
            return false;
        }
        if cf.is_interface() {
            return true;
        }
        if cf.access & acc::FINAL != 0 {
            return false;
        }
        // 子类须能调用某个构造器：JDK 类要 public / protected；依赖库类（同名包可见）非 private 即可
        cf.methods.iter().any(|m| m.name == "<init>" && (m.access & (acc::PUBLIC | acc::PROTECTED) != 0 || lib && m.access & acc::PRIVATE == 0))
    }

    /// 开放世界调用点的属主停在 L1（不透明，无方法）时升 L2：调用照常翻译，须有方法载体。
    /// 在 `promote_layout` 的超类型闭合之前调用（升起的属主的超类型随之闭合）
    pub(super) fn promote_open_owners(&mut self) {
        let mut owners: Vec<(String, usize, u32)> = Vec::new();
        for (i, mn) in self.methods.values().enumerate() {
            let Some(a) = &mn.analysis else { continue };
            for (pc, e) in &a.events {
                let Event::Invoke { opcode: classfile::op::INVOKEVIRTUAL | classfile::op::INVOKEINTERFACE, mref, .. } = e else {
                    continue;
                };
                let opaque = self.classes.get(mref.owner.as_str()).is_some_and(|c| c.level == Level::Type);
                if opaque && self.open_site(i, &mref.owner) {
                    owners.push((mref.owner.to_string(), i, *pc));
                }
            }
        }
        for (owner, m, pc) in owners {
            let Some(node) = self.classes.get_mut(owner.as_str()) else { continue };
            if node.level < Level::Layout {
                node.level = Level::Layout;
                node.level_via.entry(Level::Layout).or_insert_with(|| Via::method("layout-open-world", m, Some(pc)));
            }
        }
    }
}

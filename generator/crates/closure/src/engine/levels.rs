//! 类层级收尾：L2 布局的超类型闭合（C3 第 6 项）。
//!
//! 发射层按层级取形态：L1（`type`）类发不透明类型（无字段、无方法、无 vtable），其余照常发射。
//! 发射层的类层次构件（struct 平铺祖先字段、`impl Anc__VTable for X__inner`、接口 impl 链、
//! `From<X> for Anc` 上转、iface_upcasts）要求每个非 L1 类的全部超类型都有载体。规则：
//!
//! - 非 L1 类（含 lambda 的函数式接口、手写实现对象的超类型——二者在登记时已至少 L2）的
//!   全部传递超类型（超类与超接口）至少 L2；
//! - 于是 L1 类只可能是「没有任何非 L1 子类型」的类型：值只可能是 null。
//!
//! 例外是运行期定义的类：VM 承载的类定义点（动态代理等，`vm_intrinsics.toml` 的 class_definition）
//! 交出的对象可实现任意接口，这些接口在闭包里没有静态的实现类。这类对象只经未建模来源（手写 / VM
//! 产出，`unmodeled.rs`）进入字节码值流，故：虚调用点的接收者可能来自未建模来源时，调用属主至少 L2
//! （`promote_unmodeled_owners`，先于超类型闭合）。之后「属主为 L1 ⇒ 接收者恒 null」对全部调用点成立。
//! 开放世界调用点（非用户方法里属主可被用户扩展，`open_world.rs`）不折叠为 null_recv，其属主同样至少 L2
//! （`promote_open_owners`，先于超类型闭合）。
//!
//! 只改层级，不新增可达方法 / 类，在工作队列排空后一次完成。

use super::*;

impl Engine<'_> {
    /// 接收者可能来自未建模来源的虚调用点（invokevirtual / invokeinterface），其属主升 L2
    fn promote_unmodeled_owners(&mut self) {
        let um = self.unmodeled();
        let mut owners: Vec<(String, usize, u32)> = Vec::new();
        for (i, mn) in self.methods.values().enumerate() {
            let Some(a) = &mn.analysis else { continue };
            for (pc, e) in &a.events {
                let Event::Invoke { opcode: classfile::op::INVOKEVIRTUAL | classfile::op::INVOKEINTERFACE, mref, args, .. } = e else {
                    continue;
                };
                let opaque = self.classes.get(mref.owner.as_str()).is_some_and(|c| c.level == Level::Type);
                if opaque && args.first().is_none_or(|r| self.recv_unmodeled(i, r, &um)) {
                    owners.push((mref.owner.to_string(), i, *pc));
                }
            }
        }
        for (owner, m, pc) in owners {
            let Some(node) = self.classes.get_mut(owner.as_str()) else { continue };
            if node.level < Level::Layout {
                node.level = Level::Layout;
                node.level_via.entry(Level::Layout).or_insert_with(|| Via::method("layout-unmodeled-recv", m, Some(pc)));
            }
        }
    }

    pub(super) fn promote_layout(&mut self) {
        self.promote_unmodeled_owners();
        self.promote_open_owners();
        let mut work: Vec<String> =
            self.classes.iter().filter(|(_, c)| c.level > Level::Type).map(|(n, _)| n.clone()).collect();
        while let Some(cls) = work.pop() {
            let Some(cf) = self.h.class(&cls) else { continue };
            for up in cf.super_name.iter().chain(cf.interfaces.iter()) {
                let Some(node) = self.classes.get_mut(up.as_str()) else { continue };
                if node.level >= Level::Layout {
                    continue;
                }
                node.level = Level::Layout;
                node.level_via.entry(Level::Layout).or_insert_with(|| Via::class("layout-supertype", &cls));
                work.push(up.clone());
            }
        }
    }
}

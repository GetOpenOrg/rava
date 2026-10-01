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
//! 只改层级，不新增可达方法 / 类，在工作队列排空后一次完成。

use super::{Engine, Level, Via};

impl Engine<'_> {
    pub(super) fn promote_layout(&mut self) {
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

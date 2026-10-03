//! 引擎：序列化构造器的分配目标（`[facts.reflect] serial_allocators`）。
//!
//! 反序列化经序列化构造器创建对象：不运行目标类自身的构造器分配实例（L3 分派闭包的 `<alloc>` 臂），再在其上
//! 运行首个不可序列化超类的无参构造体（`<init_on>` 臂）。分配目标 = 生成点上 Class 形参值集里镜像所指的可序列化
//! 具体类 ∩ 已实例化的类——流中的类由程序自身写出时已实例化；只出现在外部数据里的类同按名取类的未知名字，
//! 不在闭包内。候选与实例化集合各自只增，两侧任一增长时求交（与处理顺序无关）。

use super::*;

impl<'a> Engine<'a> {
    /// 序列化构造器生成点（实参 args 含接收者）：分配目标形参 idx 的镜像所指类登记为分配候选
    pub(super) fn serial_alloc_site(&mut self, m: usize, off: u32, k: &str, opcode: u8, args: &[V], idx: usize) {
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let Some(v) = args.get(skip + idx).cloned() else { return };
        let (classes, _) = self.mirror_classes_of(m, off, k, &v, true);
        for c in classes {
            if c.starts_with('[') || !self.class_serializable(&c) {
                continue;
            }
            let id = self.id(&c);
            if self.salloc_cands.insert(id) && self.g.contains(&id) {
                self.serial_alloc_target(id);
            }
        }
    }

    /// 类 id 进 G：是分配候选时成为分配目标
    pub(super) fn serial_alloc_on_grow(&mut self, id: u32) {
        if self.salloc_cands.contains(&id) {
            self.serial_alloc_target(id);
        }
    }

    /// 分配目标：登记 `<alloc>`，首个不可序列化超类的无参构造器入链并成为反射构造成员（`<init_on>` 臂）
    fn serial_alloc_target(&mut self, id: u32) {
        let cls = self.names[id as usize].to_string();
        if !self.serial_allocs.insert(cls.clone()) {
            return;
        }
        let mut cur = self.h.class(&cls).and_then(|cf| cf.super_name.clone());
        while let Some(s) = cur {
            if self.class_serializable(&s) {
                cur = self.h.class(&s).and_then(|cf| cf.super_name.clone());
                continue;
            }
            let Some(cf) = self.h.class(&s) else { return };
            if !cf.methods.iter().any(|mm| mm.name == "<init>" && mm.desc == "()V") {
                return;
            }
            let key = MemberRef { owner: s.clone(), name: "<init>".into(), desc: "()V".into() };
            if self.reflect_members.insert((Members::Constructors, key.clone())) {
                self.method(key, Via::class("serial-alloc", &cls));
            }
            return;
        }
    }

    /// 类可序列化（清单 `serializable_markers` 的子类型；清单未登记标记时一律视为可序列化）
    fn class_serializable(&self, cls: &str) -> bool {
        let markers = self.man.serializable_markers();
        markers.is_empty() || markers.iter().any(|x| self.h.is_subtype(cls, x))
    }
}

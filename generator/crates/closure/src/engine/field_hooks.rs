//! 引擎：VM 状态字段的访问钩子（vm_intrinsics.toml `[vm_state.field_hooks]`）。
//!
//! 访问点连到钩子的手写体（发射层在访问前调用钩子），读站点的结果另接钩子值池——钩子落地写入的值
//! （如定义加载器）经手写体产出。
//!
//! 接收者钩子按接收者值集判定是否可达：钩子只为「尚未落地」的对象做事。类镜像的定义加载器钩子对引导
//! 加载器定义的类是空操作（镜像的 `classLoader` 恒 null），故接收者值集只含引导类镜像时不接入钩子、读结果
//! 只有字段值集（null）；含应用 / 平台类镜像、所指未知的 Class 对象或 open 时接入。求值器同口径：接收者为
//! 引导类的类字面量时读结果折叠为 null（`mirror_hook_field`）。

use super::*;
use crate::loaders::{DefiningLoaders, Loader};

impl<'a> Engine<'a> {
    pub(super) fn field_hook(&mut self, decl: &str, f: &MemberRef, opcode: u8, via: &Via, res: Node) {
        use classfile::op;
        let Some(h) = self.man.vm_state.field_hook(decl, &f.name, &f.desc).cloned() else { return };
        self.touch(&h.host, Level::Type, via.clone());
        let t = self.rt_fn_node(&h.host, &h.func, via.clone());
        if opcode == op::GETFIELD || opcode == op::GETSTATIC {
            let Some(tid) = parse_field(&f.desc).and_then(|ft| self.ptype(&ft)) else { return };
            self.flow(Node::S(t, POOL), res, tid);
        }
    }

    /// 接收者钩子字段：接收者值集 s（已按属主过滤）中是否有钩子需落地的对象
    pub(super) fn recv_hook_needed(&mut self, decl: &str, f: &MemberRef, s: &TypeSet) -> bool {
        match self.man.vm_state.field_hook(decl, &f.name, &f.desc) {
            Some(h) if h.receiver => {}
            _ => return false,
        }
        if !s.open.is_empty() {
            return true;
        }
        let xs: Vec<u32> = s.classes.iter().collect();
        xs.into_iter().any(|x| match self.mirrors.get(&x).copied() {
            Some(c) => self.defining_loader(c) != Loader::Boot,
            None => true,
        })
    }

    /// 类型序号 c 的定义加载器
    fn defining_loader(&self, c: u32) -> Loader {
        self.ctx.defining_loader(&self.names[c as usize])
    }
}

impl Ctx<'_> {
    /// 类（binary name 或数组描述符）的定义加载器：数组按元素类，基本类型为引导
    pub(super) fn defining_loader(&self, name: &str) -> Loader {
        let elem = name.trim_start_matches('[');
        let elem = if elem.len() < name.len() {
            match elem.strip_prefix('L').and_then(|e| e.strip_suffix(';')) {
                Some(e) => e,
                None => return Loader::Boot,
            }
        } else {
            elem
        };
        let (cp, man) = (self.cp, self.man);
        self.loaders.get_or_init(|| DefiningLoaders::new(cp, man.vm_state.loader_map.as_ref())).loader_of(cp, elem)
    }

    /// 求值器读接收者钩子字段：接收者是引导类的类字面量时钩子为空操作，字段恒为 null
    /// （引导类 `<clinit>` 的 `desiredAssertionStatus()` 由此经常量求值折叠为 false）
    pub(super) fn mirror_hook_field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        let Some(V::Class(c, _)) = recv else { return None };
        if opcode != classfile::op::GETFIELD {
            return None;
        }
        let fi = self.field_info(f)?;
        let h = self.man.vm_state.field_hook(&fi.key.owner, &fi.key.name, &fi.key.desc)?;
        (h.receiver && self.defining_loader(c) == Loader::Boot).then_some(V::Null)
    }
}

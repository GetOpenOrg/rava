//! 引擎：VM 状态字段的访问钩子（vm_intrinsics.toml `[vm_state.field_hooks]`）。
//!
//! 访问点连到钩子的手写体（发射层在访问前调用钩子），读站点的结果另接钩子值池——钩子落地写入的值
//! （如定义加载器）经手写体产出。
//!
//! 接收者钩子按接收者值集判定是否可达：钩子只为「尚未落地」的对象做事。登记为 `boot_noop` 的钩子（类镜像的
//! 定义加载器）对引导加载器定义的类是空操作（镜像的 `classLoader` 恒 null），故接收者值集只含引导类镜像时不接入
//! 钩子、读结果只有字段值集（null）；含应用 / 平台类镜像、所指未知的 Class 对象或 open 时接入。求值器同口径：
//! 接收者为引导类的类字面量时读结果折叠为 null（`mirror_hook_field`）。其余接收者钩子（类镜像的模块）对每个镜像
//! 都落地，有接收者即接入；类镜像模块的钩子值池另含映像 VM 模块表（`image_start.rs` `image_module_table`），
//! 抽象解释按同一张表把已知镜像的模块读折叠为映像对象（`mirrors_hook_field`，引用相等按对象身份折叠）。

use super::*;
use crate::absint::IMAGE_PENDING;
use crate::loaders::{DefiningLoaders, Loader};

impl<'a> Engine<'a> {
    pub(super) fn field_hook(&mut self, decl: &str, f: &MemberRef, opcode: u8, via: &Via, res: Node) {
        use classfile::op;
        let Some(h) = self.man.vm_state.field_hook(decl, &f.name, &f.desc).cloned() else { return };
        self.touch(&h.host, Level::Type, via.clone());
        let t = self.rt_fn_node(&h.host, &h.func, via.clone());
        if opcode == op::GETFIELD || opcode == op::GETSTATIC {
            let Some(tid) = parse_field(&f.desc).and_then(|ft| self.ptype(&ft)) else { return };
            self.image_module_table(decl, &f.name, t, tid);
            self.flow(Node::S(t, POOL), res, tid);
        }
    }

    /// 字段登记了接收者钩子（`receiver = true`）
    pub(super) fn recv_hook_field(&self, decl: &str, f: &MemberRef) -> bool {
        self.man.vm_state.field_hook(decl, &f.name, &f.desc).is_some_and(|h| h.receiver)
    }

    /// 接收者钩子字段：接收者值集 s（已按属主过滤）中是否有钩子需落地的对象
    pub(super) fn recv_hook_needed(&mut self, decl: &str, f: &MemberRef, s: &TypeSet) -> bool {
        let boot_noop = match self.man.vm_state.field_hook(decl, &f.name, &f.desc) {
            Some(h) if h.receiver => h.boot_noop,
            _ => return false,
        };
        if !s.open.is_empty() || (!boot_noop && !s.classes.is_empty()) {
            return true;
        }
        let xs: Vec<u32> = s.classes.iter().collect();
        xs.into_iter().any(|x| match self.mirrors.get(&x).copied() {
            Some(c) => self.defining_loader(c) != Loader::Boot,
            None => true,
        })
    }

    /// 类型序号 c 的定义加载器
    pub(super) fn defining_loader(&self, c: u32) -> Loader {
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
        self.mirrors_hook_field(f, [&**c])
    }

    /// 接收者钩子字段在一组类镜像上的读结果：
    /// - 每个镜像的钩子都是空操作（引导加载器定义的类）时恒为 null；
    /// - 类镜像的模块（清单 `[concrete.vm_fields] class_module`）：每个镜像按「定义加载器 + 包」在映像 VM 模块表中
    ///   查到同一个模块时为该映像对象（带 `Obj::Image` 标签的非空引用，类型由调用点按描述符补上）——运行期钩子
    ///   以同一张表（启动序列登记）查得同一个对象；
    /// - 否则未知。空集合给乐观值（null / `IMAGE_PENDING` 占位；调用方登记乐观答复，值集增长后重分析）
    pub(super) fn mirrors_hook_field<'c>(&self, f: &MemberRef, classes: impl IntoIterator<Item = &'c str>) -> Option<V> {
        let fi = self.field_info(f)?;
        let h = self.man.vm_state.field_hook(&fi.key.owner, &fi.key.name, &fi.key.desc)?;
        if !h.receiver {
            return None;
        }
        if h.boot_noop {
            return classes.into_iter().all(|c| self.defining_loader(c) == Loader::Boot).then_some(V::Null);
        }
        if !self.is_class_module_field(&fi.key) {
            return None;
        }
        let mut obj = IMAGE_PENDING;
        for c in classes {
            let m = self.image_module_of(c)?;
            if obj != IMAGE_PENDING && obj != m {
                return None;
            }
            obj = m;
        }
        let tag = self.img_module_tags.get().and_then(|t| t.get(&obj).cloned()).unwrap_or_else(|| Rc::new(crate::absint::Obj::Image(obj, Vec::new())));
        Some(V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(tag) })
    }

    /// 字段是否为类镜像的模块字段（清单 `[concrete.vm_fields] class_module`）
    fn is_class_module_field(&self, k: &MemberRef) -> bool {
        self.man.concrete.vm_fields.get("class_module").and_then(|q| q.rsplit_once('.')).is_some_and(|(o, n)| o == k.owner && n == k.name)
    }

    /// 类镜像所属模块的映像对象：与运行期钩子同规则按「定义加载器 + 包」查映像 VM 模块表。数组类、未登记的包
    /// （加载器的无名模块）、非 JDK 的非引导类不折叠（None）
    fn image_module_of(&self, c: &str) -> Option<u32> {
        if c.starts_with('[') {
            return None;
        }
        let boot = self.defining_loader(c) == Loader::Boot;
        if !boot && !matches!(self.cp.origin(c), Some(resolve::Origin::Jdk)) {
            return None;
        }
        let pkg = c.rsplit_once('/').map_or("", |(p, _)| p);
        self.img_modules.get()?.get(pkg)?.iter().find(|&&(_, b)| b == boot).map(|&(o, _)| o)
    }

    /// 实例调用 m 在一组接收者类镜像上的结果：m 的唯一目标是接收者钩子字段的平凡取值（`return this.f`）时
    /// 为该字段在这组镜像上的读结果（[`Ctx::mirrors_hook_field`]）；否则未知
    pub(super) fn mirrors_call<'c>(&self, m: &MemberRef, classes: impl IntoIterator<Item = &'c str>) -> Option<V> {
        let site = self.h.resolve_method(&m.owner, &m.name, &m.desc, false)?;
        let (owner, name, desc) = site.key();
        let cf = self.cp.get(&owner)?;
        let (fname, fdesc) = super::method_lookup::getter_field(cf.method(&name, &desc)?.code.as_ref()?, &owner)?;
        self.mirrors_hook_field(&MemberRef { owner, name: fname, desc: fdesc }, classes)
    }
}

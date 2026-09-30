//! 引擎：手写实现对象——手写文件里实现 Java 类型 vtable trait 的 struct（`impl X__VTable for S`）。
//!
//! 手写边界方法把这类对象当作 `X` 交给建模代码（`LocaleProviderAdapter.getAdapter` 的适配器单例、
//! `SharedSecrets.getJavaLangAccess` 的 JLA 实现……），运行期它们是真实的接收者。引擎把每个对象登记为
//! 伪类型（名 `宿主类::S`，与 lambda 合成类同为非 Java 类的值）：手写体引用它即实例化进 G，
//! 子类型关系取其实现的 Java 类型，虚调用按 trait impl 的 fn 派发到伪方法节点（手写体效果照常建模），
//! 对象未覆盖的方法按实现类型的虚方法选择落到 Java 方法体。伪类型与伪方法不进输出。

use super::*;

/// 伪方法节点的种类标签
pub(super) const HWOBJ_KIND: &str = "object";

/// 手写实现对象
#[derive(Clone)]
pub(super) struct HwObj {
    /// 所在手写文件的宿主类（解析其 use 表与 fn）
    pub host: String,
    /// struct 名
    pub rust: String,
    /// 实现的 Java 类型
    pub supers: Vec<String>,
}

impl<'a> Engine<'a> {
    /// 手写体引用的实现对象（`S` 在宿主类手写文件里；`…::<类>_impl::S` 在该类的手写文件里）→ 伪类型 id
    pub(super) fn hwobj(&mut self, host: &str, t: &TypeRef) -> Option<u32> {
        let (owner, rust) = match t.0.as_slice() {
            [s] => (host.to_string(), s.clone()),
            [.., module, s] => {
                let snake = MODULE_SUFFIXES.iter().find_map(|x| module.strip_suffix(x))?;
                (self.class_of_module(host, &t.0[..t.0.len() - 1], snake)?, s.clone())
            }
            [] => return None,
        };
        let name = format!("{owner}::{rust}");
        if let Some(&id) = self.ids.get(name.as_str()) {
            if self.hwobjs.contains_key(&id) {
                return Some(id);
            }
        }
        let hwc = self.hw.class(&owner);
        let o = hwc.objects.get(&rust)?;
        let supers: Vec<String> = o.supers.iter().filter_map(|s| self.resolve_tref(&owner, s)).collect();
        let id = self.id(&name);
        for s in &supers {
            self.touch(s, Level::Type, Via::class("hw-object", &owner));
        }
        self.hwobjs.insert(id, HwObj { host: owner, rust, supers });
        Some(id)
    }

    /// 手写体引用的实现对象进入 G，并作为手写体产出
    pub(super) fn hwobj_made(&mut self, m: usize, host: &str, mh: &MemberHw) {
        for t in &mh.objects {
            let Some(id) = self.hwobj(host, t) else { continue };
            if self.g.insert(id) {
                self.on_g_grow(id);
            }
            self.add_to(Node::S(m, PROD), &TypeSet::exact(id));
        }
    }

    /// 实现对象 x 是否 f 的子类型（None = x 不是实现对象）
    pub(super) fn hwobj_sub(&self, x: u32, f: &str) -> Option<bool> {
        let o = self.hwobjs.get(&x)?;
        Some(f == OBJECT || o.supers.iter().any(|s| self.h.is_subtype(s, f)))
    }

    /// 实现对象 r 上的虚调用：trait impl 里 Rust 名对应被调成员的 fn → 伪方法节点；
    /// 对象未提供 → 按实现类型的虚方法选择（接口默认方法 / 抽象类的具体方法 / Object）
    pub(super) fn hwobj_target(&mut self, r: u32, site: &resolve::MethodSite, via: Via) -> Option<usize> {
        let o = self.hwobjs[&r].clone();
        let (decl, name, desc) = site.key();
        let (plain, mangled) = self.rust_names(&site.class, &name, &desc);
        let hwc = self.hw.class(&o.host);
        let fns = &hwc.objects[&o.rust].fns;
        let mut hit: Vec<String> =
            fns.keys().filter(|f| **f == mangled || plain.as_deref() == Some(f.as_str())).cloned().collect();
        if hit.is_empty() {
            hit = fns.keys().filter(|f| member_matches(f, &name)).cloned().collect();
        }
        if !hit.is_empty() {
            hit.sort();
            let key = MemberRef { owner: self.names[r as usize].to_string(), name, desc };
            return Some(self.hwobj_method(r, key, hit, via));
        }
        for s in o.supers.iter().map(String::as_str).chain([OBJECT]) {
            if let Some(sel) = self.h.select(s, site).filter(|x| !x.method().is_abstract()) {
                let (o, n, d) = sel.key();
                return Some(self.method(MemberRef { owner: o, name: n, desc: d }, via));
            }
        }
        self.unresolved.insert(format!("select {} {decl}.{}", self.names[r as usize], site.method().name));
        None
    }

    /// 实现对象的伪方法节点：接收者即该对象，形参 / 返回按被调成员描述符
    fn hwobj_method(&mut self, r: u32, key: MemberRef, fns: Vec<String>, via: Via) -> usize {
        if let Some(i) = self.methods.get_index_of(&(key.clone(), NOCTX)) {
            return i;
        }
        let mut ptypes = vec![Some(r)];
        let mut rtype = None;
        if let Some(md) = parse_method(&key.desc) {
            for p in &md.params {
                let t = self.ptype(p);
                ptypes.push(t);
            }
            rtype = md.ret.as_ref().and_then(|t| self.ptype(t));
        }
        let idx = self.methods.len();
        let desc = key.desc.clone();
        self.methods.insert(
            (key.clone(), NOCTX),
            MNode {
                key: key.clone(),
                kind: Kind::Handwritten(HWOBJ_KIND),
                via,
                is_static: false,
                ptypes,
                rtype,
                analysis: None,
                applied: None,
                aseq: 0,
                applied_seq: 0,
                returned: None,
                hw_fns: fns,
                ctx: NOCTX,
                ret_model: RetModel::Plain,
            },
        );
        self.mbase.entry(key).or_insert(idx);
        self.touch_desc(&desc, &Via::method("signature", idx, None));
        self.push_m(idx);
        idx
    }

    /// 伪方法节点：返回 open(返回类型)，形参与产出汇入值池，手写体效果按宿主文件的 use 表建模
    pub(super) fn process_hwobj_method(&mut self, m: usize) {
        let via = Via::method("handwritten", m, None);
        if let Some(rt) = self.methods[m].rtype {
            self.add_to(Node::R(m), &TypeSet::open(rt));
        }
        let pts = self.methods[m].ptypes.clone();
        for (i, pt) in pts.iter().enumerate() {
            if let Some(pt) = pt {
                self.flow(Node::P(m, i as u16), Node::S(m, POOL), *pt);
            }
        }
        let obj = self.id(OBJECT);
        self.flow(Node::S(m, PROD), Node::S(m, POOL), obj);
        let Some((host, mh)) = self.hw_body(m) else { return };
        let rt = self.methods[m].rtype;
        for t in self.hw_exports(&host, &mh, rt, false) {
            self.flow(Node::S(m, POOL), Node::Esc, t);
        }
        self.apply_hw(m, &host, &mh, &via);
    }

    /// 手写方法节点的手写体（宿主类, 汇总）：伪方法取实现对象的 fn，其余按成员匹配宿主类手写文件
    pub(super) fn hw_body(&self, t: usize) -> Option<(String, MemberHw)> {
        let mn = &self.methods[t];
        if mn.kind == Kind::Handwritten(HWOBJ_KIND) {
            let o = mn.ptypes.first().copied().flatten().and_then(|r| self.hwobjs.get(&r))?;
            return Some((o.host.clone(), self.hw.object_member(&o.host, &o.rust, &mn.hw_fns)));
        }
        let cf = self.h.class(&mn.key.owner)?;
        Some((mn.key.owner.clone(), self.hw_member(&cf, &mn.key.name, &mn.key.desc)))
    }

    /// 伪方法节点（不进输出）
    pub(super) fn is_hwobj_method(&self, t: usize) -> bool {
        self.methods[t].kind == Kind::Handwritten(HWOBJ_KIND)
    }
}

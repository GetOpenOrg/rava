//! 虚方法槽位归属（← `emitter/vtable_util.py`）。
//!
//! 返回值沿用 Python 口径：槽位声明者的 Rust 短名；空串 = 非虚方法。

use std::collections::{BTreeSet, VecDeque};

use classfile::Method;
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::text::contains_word;

const ACC_PRIVATE: u16 = 0x0002;
const INIT: &str = "<init>";
const CLINIT: &str = "<clinit>";

/// 描述符的参数部分（含右括号）
pub fn param_part(desc: &str) -> &str {
    desc.find(')').map_or(desc, |i| &desc[..=i])
}

/// 描述符的返回类型部分
pub fn return_part(desc: &str) -> &str {
    desc.find(')').map_or("", |i| &desc[i + 1..])
}

/// 同名且参数描述符一致（JVMS §5.4.5 覆盖判定，返回可协变）
pub(crate) fn same_slot(a: &Method, m: &Method) -> bool {
    a.name == m.name && param_part(&a.desc) == param_part(&m.desc)
}

fn is_private(m: &Method) -> bool {
    m.access & ACC_PRIVATE != 0
}

impl<'a> EmitCtx<'a> {
    /// 类的共置手写 `pub fn` 名集合是否含该方法（原名或 mangle 名）
    pub(crate) fn hw_has(&self, cls: &str, m: &Method) -> bool {
        self.input.handwritten.get(cls).is_some_and(|h| {
            h.methods.contains(&safe_ident(&m.name))
                || h.methods.contains(&safe_ident(&mangle_name(&self.manifest.ty, &m.name, &m.desc)))
        })
    }

    /// 方法 Rust 名（按接收者 `ci` 的重载态 mangle，不做关键字转义）
    pub fn member_rust_name(&self, ci: &ClassInfo, m: &Method) -> String {
        if self.ty.method_name_is_mangled(ci, &m.name) {
            mangle_name(&self.manifest.ty, &m.name, &m.desc)
        } else {
            m.name.clone()
        }
    }

    /// 类发射时注入的接口 default 方法 (name, descriptor)（与 default 继承段同源）
    pub fn injected_default_sigs(&self, ci: &ClassInfo) -> BTreeSet<(String, String)> {
        let reg = self.ty.reg;
        let mut out = BTreeSet::new();
        if ci.is_interface() || ci.interfaces().is_empty() {
            return out;
        }
        let mut covered: BTreeSet<(String, String)> = BTreeSet::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur {
            if !seen.insert(c.name()) {
                break;
            }
            let is_self = std::ptr::eq(c, ci);
            for m in c.methods() {
                if is_self && m.is_synthetic() {
                    continue;
                }
                if !is_self && (m.is_static() || m.is_synthetic() || m.name == INIT || m.name == CLINIT || is_private(m)) {
                    continue;
                }
                covered.insert((m.name.clone(), param_part(&m.desc).to_string()));
            }
            cur = reg.get(c.super_class());
        }
        let mut anc_ifaces: BTreeSet<String> = BTreeSet::new();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut cur = reg.get(ci.super_class());
        while let Some(c) = cur {
            if !seen.insert(c.name()) {
                break;
            }
            let mut q: VecDeque<String> = c.interfaces().iter().cloned().collect();
            while let Some(n) = q.pop_front() {
                if !anc_ifaces.insert(n.clone()) {
                    continue;
                }
                if let Some(x) = reg.get(&n) {
                    q.extend(x.interfaces().iter().cloned());
                }
            }
            cur = reg.get(c.super_class());
        }
        let mut q: VecDeque<String> = ci.interfaces().iter().cloned().collect();
        let mut visited = anc_ifaces;
        while let Some(n) = q.pop_front() {
            if !visited.insert(n.clone()) {
                continue;
            }
            let Some(ici) = reg.get(&n) else { continue };
            q.extend(ici.interfaces().iter().cloned());
            for dm in ici.methods() {
                if dm.is_abstract() || dm.is_static() || dm.is_synthetic() || is_private(dm) || dm.name == INIT || dm.name == CLINIT {
                    continue;
                }
                if covered.insert((dm.name.clone(), param_part(&dm.desc).to_string())) {
                    out.insert((dm.name.clone(), dm.desc.clone()));
                }
            }
        }
        out
    }

    /// `ci` 注入的 default 中与 `m` 同槽位者的 Rust 名；无则空串
    fn injected_default_rust_name(&self, ci: &ClassInfo, m: &Method) -> String {
        let injected = self.injected_default_sigs(ci);
        let Some((hn, hd)) = injected
            .iter()
            .find(|(n, d)| *n == m.name && param_part(d) == param_part(&m.desc))
        else {
            return String::new();
        };
        let own: BTreeSet<String> = ci
            .methods()
            .iter()
            .filter(|x| !x.is_synthetic() && x.name != INIT && x.name != CLINIT)
            .map(|x| self.member_rust_name(ci, x))
            .collect();
        let same_name = injected.iter().filter(|(n, _)| n == hn).count();
        if self.ty.hierarchy_overloaded_names(ci).contains(hn) || own.contains(hn) || same_name > 1 {
            mangle_name(&self.manifest.ty, hn, hd)
        } else {
            hn.clone()
        }
    }

    /// 虚方法归属的 vtable 类 Rust 名（精确描述符的最远非私有非手写声明者）；
    /// 未被派发到、不占槽的实现（[`Self::slot_pruned`]）为空串
    pub fn find_virtual_in(&self, m: &Method, ci: &ClassInfo) -> String {
        if self.slot_pruned(m, ci) {
            return String::new();
        }
        self.raw_find_virtual_in(m, ci)
    }

    /// 槽族归属（不按分派结果裁剪）：精确描述符的最远非私有非手写声明者
    fn raw_find_virtual_in(&self, m: &Method, ci: &ClassInfo) -> String {
        if ci.is_constructor(m) || m.is_static() || m.is_native() {
            return String::new();
        }
        if ci.is_interface() {
            return self.short(ci.name());
        }
        let reg = self.ty.reg;
        let mut oldest: Option<&str> = None;
        let mut cur = ci.super_class();
        // Python 此循环无 seen 守卫；注册表继承链无环
        while !cur.is_empty() && cur != ty::consts::OBJECT {
            let Some(anc) = reg.get(cur) else { break };
            match anc.methods().iter().find(|am| am.name == m.name && am.desc == m.desc) {
                Some(am) => {
                    if !self.hw_has(cur, am) && !is_private(am) {
                        oldest = Some(anc.name());
                    }
                }
                None => {
                    if self.injected_default_sigs(anc).contains(&(m.name.clone(), m.desc.clone())) {
                        oldest = Some(anc.name());
                    }
                }
            }
            cur = anc.super_class();
        }
        self.short(oldest.unwrap_or(ci.name()))
    }

    /// 声明者 `decl_m` 的槽位返回是否为 Object 位
    fn slot_return_is_object(&self, decl_ci: &ClassInfo, decl_m: &Method) -> bool {
        let params = self.ty.effective_class_type_params(decl_ci);
        let ret = self.ty.emitted_method_sig_types(decl_ci, decl_m, &params).ret;
        let inner = ret.render(self.ty.names);
        if inner.is_empty() || inner == "Object" {
            return true;
        }
        params.iter().any(|p| contains_word(&inner, p))
    }

    /// 声明 `am`（属 `anc`）的最终槽位落点
    fn covariant_landing_slot<'m>(&self, am: &'m Method, anc: &'m ClassInfo) -> (&'m ClassInfo, &'m Method)
    where
        'a: 'm,
    {
        let owner = self.raw_covariant_virtual_owner(am, anc);
        if owner.is_empty() || owner == self.short(anc.name()) {
            return (anc, am);
        }
        let reg = self.ty.reg;
        let mut seen: BTreeSet<&str> = BTreeSet::from([anc.name()]);
        let mut cur = anc.super_class();
        while !cur.is_empty() && cur != ty::consts::OBJECT && seen.insert(cur) {
            let Some(c) = reg.get(cur) else { break };
            if self.short(cur) == owner {
                if let Some(hit) = c.methods().iter().find(|x| !x.is_synthetic() && same_slot(x, am)) {
                    return (c, hit);
                }
            }
            cur = c.super_class();
        }
        (anc, am)
    }

    /// 同名 + 同参数描述符（返回可协变）的最远槽位声明者（K-6a）；空串 = 无
    fn raw_covariant_virtual_owner(&self, m: &Method, ci: &ClassInfo) -> String {
        if ci.is_constructor(m) || m.is_static() || m.is_native() || ci.is_interface() {
            return String::new();
        }
        let reg = self.ty.reg;
        let cur_mangled = self.ty.hierarchy_overloaded_names(ci).contains(&m.name);
        let mut prev_return = return_part(&m.desc).to_string();
        let mut oldest: Option<&str> = None;
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut cur = ci.super_class();
        while !cur.is_empty() && cur != ty::consts::OBJECT && seen.insert(cur) {
            let Some(anc) = reg.get(cur) else { break };
            if let Some(am) = anc.methods().iter().find(|x| !x.is_synthetic() && same_slot(x, m)) {
                if !self.hw_has(cur, am) && !is_private(am) {
                    if cur_mangled && !self.ty.hierarchy_overloaded_names(anc).contains(&m.name) {
                        break;
                    }
                    let am_return = return_part(&am.desc);
                    if am_return != prev_return {
                        let (land_ci, land_m) = self.covariant_landing_slot(am, anc);
                        if !self.slot_return_is_object(land_ci, land_m) {
                            break;
                        }
                    }
                    prev_return = am_return.to_string();
                    oldest = Some(anc.name());
                }
            }
            cur = anc.super_class();
        }
        oldest.map(|o| self.short(o)).unwrap_or_default()
    }

    /// `owner_rust` 在 `ci` 超类链上的位置（直接父类 0；不在链上 -1）
    fn chain_depth(&self, owner_rust: &str, ci: &ClassInfo) -> i32 {
        let mut seen: BTreeSet<&str> = BTreeSet::from([ci.name()]);
        let mut cur = ci.super_class();
        let mut depth = 0;
        while !cur.is_empty() && seen.insert(cur) {
            if self.short(cur) == owner_rust {
                return depth;
            }
            let Some(c) = self.ty.reg.get(cur) else { return -1 };
            cur = c.super_class();
            depth += 1;
        }
        -1
    }

    /// 虚方法槽位归属的完整解析（精确描述符与协变模型取链上更远者）；不占槽的实现为空串
    pub fn resolve_virtual_slot(&self, m: &Method, ci: &ClassInfo) -> String {
        if self.slot_pruned(m, ci) {
            return String::new();
        }
        self.raw_resolve_virtual_slot(m, ci)
    }

    /// 槽族根（不按分派结果裁剪）：精确描述符与协变模型取链上更远者
    pub(crate) fn raw_resolve_virtual_slot(&self, m: &Method, ci: &ClassInfo) -> String {
        let slot = self.raw_find_virtual_in(m, ci);
        if slot.is_empty() || ci.is_interface() {
            return slot;
        }
        let cov = self.raw_covariant_virtual_owner(m, ci);
        if !cov.is_empty() && cov != slot {
            if slot == self.short(ci.name()) {
                return cov;
            }
            return if self.chain_depth(&slot, ci) >= self.chain_depth(&cov, ci) { slot } else { cov };
        }
        if cov.is_empty() {
            slot
        } else {
            cov
        }
    }

    /// 覆盖条目对应的祖先 vtable 槽位成员名（声明者视角）；不解耦时空串
    pub fn slot_member_rust_name(&self, m: &Method, ci: &ClassInfo) -> String {
        let owner = self.resolve_virtual_slot(m, ci);
        if owner.is_empty() || owner == self.short(ci.name()) {
            return String::new();
        }
        let reg = self.ty.reg;
        let mut seen: BTreeSet<&str> = BTreeSet::from([ci.name()]);
        let mut cur = ci.super_class();
        while !cur.is_empty() && seen.insert(cur) {
            let Some(c) = reg.get(cur) else { return String::new() };
            if self.short(cur) == owner {
                return match c.methods().iter().find(|x| !x.is_synthetic() && same_slot(x, m)) {
                    Some(decl) => self.member_rust_name(c, decl),
                    None => self.injected_default_rust_name(c, m),
                };
            }
            cur = c.super_class();
        }
        String::new()
    }
}

//! 方法 / 构造器签名类型与层次重载命名判定。
//! 构造器 / 覆盖方法签名见 [`ctor`]，发射签名见 [`emitted`]。

mod ctor;
mod emitted;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;

pub use ctor::{is_anonymous_class, SigTypes};
pub use emitted::EmittedSig;

use crate::ident::safe_ident;
use crate::registry::ClassInfo;
use crate::type_map::mangle_name;
use crate::TyCtx;

/// 方法名 → 参数段集合（描述符 `)` 之前的部分）
type ParamSets = BTreeMap<String, BTreeSet<String>>;

fn param_section(desc: &str) -> String {
    desc.split(')').next().unwrap_or("").to_string()
}

impl TyCtx<'_> {
    /// ci 在 Rust impl 块中的方法表：(全部方法, 仅实例方法)。含注入的接口 default
    /// 方法；synthetic / `<clinit>` 不计；协变返回（参数相同）只算一种
    pub(crate) fn class_method_param_sets(&self, ci: &ClassInfo) -> (ParamSets, ParamSets) {
        let mut all = ParamSets::new();
        let mut inst = ParamSets::new();
        let mut add = |owner: &ClassInfo, m: &classfile::Method| {
            let p = param_section(&m.desc);
            all.entry(m.name.clone()).or_default().insert(p.clone());
            if !m.is_static() && !owner.is_constructor(m) {
                inst.entry(m.name.clone()).or_default().insert(p);
            }
        };
        for m in ci.methods() {
            if !m.is_synthetic() && m.name != "<clinit>" {
                add(ci, m);
            }
        }
        if !self.reg.is_empty() && !ci.interfaces().is_empty() && !ci.is_interface() {
            let mut queue: VecDeque<&str> = ci.interfaces().iter().map(String::as_str).collect();
            let mut seen = BTreeSet::new();
            while let Some(iname) = queue.pop_front() {
                if !seen.insert(iname) {
                    continue;
                }
                let Some(ici) = self.reg.get(iname) else {
                    continue;
                };
                queue.extend(ici.interfaces().iter().map(String::as_str));
                for m in ici.methods() {
                    if !m.is_abstract()
                        && !m.is_static()
                        && !m.is_synthetic()
                        && m.name != "<init>"
                        && m.name != "<clinit>"
                    {
                        add(ici, m);
                    }
                }
            }
        }
        (all, inst)
    }

    /// 类中需要按描述符 mangle 的方法名集合（定义侧与调用侧的唯一判定来源；规则见
    /// Python `hierarchy_overloaded_names`）。按注册表实例缓存
    pub fn hierarchy_overloaded_names(&self, ci: &ClassInfo) -> Arc<BTreeSet<String>> {
        let mut visiting = BTreeSet::new();
        self.overloaded_rec(ci, &mut visiting)
    }

    fn overloaded_rec(
        &self,
        ci: &ClassInfo,
        visiting: &mut BTreeSet<String>,
    ) -> Arc<BTreeSet<String>> {
        let cacheable = self.reg.get(ci.name()).is_some_and(|r| std::ptr::eq(r, ci));
        if cacheable {
            if let Some(hit) = self.reg.caches.overloaded.get(ci.name()) {
                return hit.clone();
            }
        }
        visiting.insert(ci.name().to_string());
        let (own_all, own_inst) = self.class_method_param_sets(ci);
        let mut result: BTreeSet<String> = own_all
            .iter()
            .filter(|(_, ps)| ps.len() > 1)
            .map(|(n, _)| n.clone())
            .collect();
        if !self.reg.is_empty() && !ci.is_interface() {
            let parent = self.reg.get(ci.super_class());
            if let Some(p) =
                parent.filter(|p| p.name() != ci.name() && !visiting.contains(p.name()))
            {
                // 单调继承：名字一旦在祖先处 mangle，后代全部沿用
                result.extend(
                    self.overloaded_rec(p, visiting)
                        .iter()
                        .filter(|n| *n != "<init>")
                        .cloned(),
                );
            }
            let mut inherited = ParamSets::new();
            let mut seen_cls = BTreeSet::from([ci.name().to_string()]);
            let mut anc = parent;
            while let Some(a) = anc.filter(|a| !seen_cls.contains(a.name())) {
                seen_cls.insert(a.name().to_string());
                for (n, ps) in self.class_method_param_sets(a).1 {
                    inherited.entry(n).or_default().extend(ps);
                }
                anc = self.reg.get(a.super_class());
            }
            self.add_unimplemented_interface_members(ci, &mut inherited);
            let union_len = |n: &str, ps: &BTreeSet<String>| {
                inherited
                    .get(n)
                    .map_or(ps.len(), |inh| ps.union(inh).count())
            };
            for (n, ps) in &own_inst {
                if union_len(n, ps) > 1 {
                    result.insert(n.clone());
                }
            }
            // 规则 4：本类 static 方法与祖先实例方法同名异参
            for (n, ps) in &own_all {
                if !own_inst.contains_key(n)
                    && n != "<init>"
                    && inherited.contains_key(n)
                    && union_len(n, ps) > 1
                {
                    result.insert(n.clone());
                }
            }
        }
        visiting.remove(ci.name());
        let result = Arc::new(result);
        if cacheable {
            self.reg
                .caches
                .overloaded
                .insert(ci.name(), result.clone());
        }
        result
    }

    /// 接口（含超接口传递、祖先类实现的接口）上声明、而 ci 及其祖先类均未以同描述符
    /// 实现的实例成员，并入继承参数段：类内 `this.m()` 可经 invokevirtual 指向这类只声明在
    /// 接口上的成员，它与类自有的同名异参方法须按描述符区分（定义侧与调用侧同用本判定）。
    /// 类链上已有同名同描述符方法（含泛型桥）的接口成员即由该实现承载，不另计参数段——
    /// 桥方法的擦除描述符不制造重载。
    fn add_unimplemented_interface_members(&self, ci: &ClassInfo, inherited: &mut ParamSets) {
        let mut chain: Vec<&ClassInfo> = Vec::new();
        let mut seen_cls = BTreeSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur.filter(|c| seen_cls.insert(c.name().to_string())) {
            chain.push(c);
            cur = self.reg.get(c.super_class());
        }
        let implemented = |name: &str, desc: &str| {
            chain
                .iter()
                .any(|c| c.methods().iter().any(|m| m.name == name && m.desc == desc && !m.is_static()))
        };
        let mut queue: VecDeque<&str> = chain
            .iter()
            .flat_map(|c| c.interfaces().iter().map(String::as_str))
            .collect();
        let mut seen = BTreeSet::new();
        while let Some(iname) = queue.pop_front() {
            if !seen.insert(iname) {
                continue;
            }
            let Some(ici) = self.reg.get(iname) else {
                continue;
            };
            queue.extend(ici.interfaces().iter().map(String::as_str));
            for m in ici.methods() {
                if m.is_static()
                    || m.is_synthetic()
                    || m.is_private()
                    || m.name.starts_with('<')
                    || implemented(&m.name, &m.desc)
                {
                    continue;
                }
                inherited
                    .entry(m.name.clone())
                    .or_default()
                    .insert(param_section(&m.desc));
            }
        }
    }

    /// 实例字段的 Rust 名：隐藏祖先同名字段的声明取 `<name>_<DeclaringClass>`
    pub fn instance_field_rust_name(&self, owner_bin: &str, safe_name: &str) -> String {
        if self.reg.is_empty() || owner_bin.is_empty() {
            return safe_name.to_string();
        }
        let declares = |ci: &ClassInfo| {
            ci.fields()
                .iter()
                .any(|f| !f.is_static() && safe_ident(&f.name) == safe_name)
        };
        let mut ci = self.reg.get(owner_bin);
        let mut seen = BTreeSet::new();
        while let Some(c) = ci.filter(|c| !seen.contains(c.name()) && !declares(c)) {
            seen.insert(c.name().to_string());
            ci = self.reg.get(c.super_class());
        }
        let Some(decl) = ci.filter(|c| declares(c)) else {
            return safe_name.to_string();
        };
        let mut seen = BTreeSet::from([decl.name().to_string()]);
        let mut anc = self.reg.get(decl.super_class());
        while let Some(a) = anc.filter(|a| !seen.contains(a.name())) {
            if declares(a) {
                let simple = decl
                    .name()
                    .rsplit('/')
                    .next()
                    .unwrap_or("")
                    .replace('$', "_");
                return format!("{safe_name}_{simple}");
            }
            seen.insert(a.name().to_string());
            anc = self.reg.get(a.super_class());
        }
        safe_name.to_string()
    }

    /// 类 ci 视角下「只声明在接口上的成员」的 Rust 方法名
    pub fn interface_member_local_name(&self, ci: &ClassInfo, mname: &str, desc: &str) -> String {
        if self.hierarchy_overloaded_names(ci).contains(mname) {
            return mangle_name(self.manifest, mname, desc);
        }
        let params = param_section(desc);
        let mut seen = BTreeSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
            seen.insert(c.name().to_string());
            if let Some(declared) = self.class_method_param_sets(c).1.get(mname) {
                // 同参数列表 = 该成员就在类的方法表里（注入的接口 default）→ 名字由类链决定
                return if declared.contains(&params) {
                    mname.to_string()
                } else {
                    mangle_name(self.manifest, mname, desc)
                };
            }
            cur = if self.reg.is_empty() {
                None
            } else {
                self.reg.get(c.super_class())
            };
        }
        mname.to_string()
    }

    /// 类 ci 声明的方法名是否带描述符后缀
    pub fn method_name_is_mangled(&self, ci: &ClassInfo, method_name: &str) -> bool {
        self.hierarchy_overloaded_names(ci).contains(method_name)
    }

    /// 继承成员在接收者 wrapper 上的名字（接收者重载态 mangle + 关键字转义）
    pub fn receiver_member_name(&self, m_name: &str, m_desc: &str, recv: &ClassInfo) -> String {
        if self.hierarchy_overloaded_names(recv).contains(m_name) {
            return safe_ident(&mangle_name(self.manifest, m_name, m_desc));
        }
        safe_ident(m_name)
    }
}

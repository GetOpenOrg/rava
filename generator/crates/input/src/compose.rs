//! 档案发射的事实合成（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）。
//!
//! 档案（`profile.json`，[`closure::profile`]）给出非用户侧（JDK / 依赖库）的全部事实，单例闭包给出本程序的
//! 用户侧事实；两者合成一份 [`ClosureFacts`] 交给发射层：
//! - 非用户侧一律取档案：同一档案下，JDK 侧发射输入与用户程序无关；
//! - 用户侧（属主为本程序用户类的类、方法、折叠、各集合项）取单例；
//! - 单例的非用户类与方法须被档案覆盖（类在档案内且层级不高于档案、方法在档案内），否则报错——
//!   档案未覆盖的程序须先并入档案（档案键随之改变），不能拿旧档案发射。

use std::collections::{BTreeMap, BTreeSet};

use closure::manifest::Domain;

use crate::facts::{ClassFact, ClosureFacts, MethodFact, SeedFacts};
use crate::InputError;

/// 成员串 / 类名的属主类（`类.名:描述符`、`类` 皆可）
fn owner_of(s: &str) -> &str {
    s.split(['.', '@', ' ']).next().unwrap_or(s)
}

fn sorted_union<T: Ord + Clone>(a: &[T], b: impl IntoIterator<Item = T>) -> Vec<T> {
    let set: BTreeSet<T> = a.iter().cloned().chain(b).collect();
    set.into_iter().collect()
}

impl ClosureFacts {
    /// 档案事实 `profile`（只含非用户侧）+ 单例闭包 `single` 的用户侧 → 发射用事实。
    /// 单例的非用户类 / 方法不被档案覆盖时报错（列出前几项）
    pub fn compose(profile: &ClosureFacts, single: &ClosureFacts) -> Result<ClosureFacts, InputError> {
        if let Some(c) = profile.classes.iter().find(|c| c.domain == Domain::User) {
            return Err(InputError::Format(format!("档案含用户类：{}", c.name)));
        }
        let user: BTreeSet<&str> = single.classes.iter().filter(|c| c.domain == Domain::User).map(|c| c.name.as_str()).collect();
        check_covered(profile, single, &user)?;
        let is_user = |s: &str| user.contains(owner_of(s));
        let user_refs = |v: &[classfile::MemberRef]| -> Vec<classfile::MemberRef> {
            v.iter().filter(|r| user.contains(r.owner.as_str())).cloned().collect()
        };
        let user_strs = |v: &[String]| -> Vec<String> { v.iter().filter(|s| is_user(s)).cloned().collect() };

        let mut classes: Vec<ClassFact> = profile.classes.clone();
        classes.extend(single.classes.iter().filter(|c| c.domain == Domain::User).cloned());
        classes.sort_by(|a, b| a.name.cmp(&b.name));
        let mut methods: Vec<(String, MethodFact)> =
            profile.methods.iter().chain(single.methods.iter().filter(|m| user.contains(m.id.owner.as_str()))).map(|m| (m.id.to_string(), m.clone())).collect();
        methods.sort_by(|a, b| a.0.cmp(&b.0));
        let mut folds = profile.folds.clone();
        folds.extend(single.folds.iter().filter(|(k, _)| is_user(k)).map(|(k, v)| (k.clone(), v.clone())));
        let by_str = |a: &[classfile::MemberRef], b: Vec<classfile::MemberRef>| -> Vec<classfile::MemberRef> {
            let m: BTreeMap<String, classfile::MemberRef> = a.iter().cloned().chain(b).map(|r| (r.to_string(), r)).collect();
            m.into_values().collect()
        };
        let s = &single.seeds;
        let p = &profile.seeds;
        let mut reflect_names = p.reflect_names.clone();
        for (owner, names) in s.reflect_names.iter().filter(|(o, _)| is_user(o)) {
            reflect_names.entry(owner.clone()).or_default().extend(names.iter().cloned());
        }
        let mut module_services = p.module_services.clone();
        module_services.extend(s.module_services.iter().filter(|(svc, prov)| is_user(svc) || is_user(prov)).cloned());
        Ok(ClosureFacts {
            classes,
            methods: methods.into_iter().map(|(_, m)| m).collect(),
            clinit: sorted_union(&profile.clinit, user_strs(&single.clinit)),
            refs: by_str(&profile.refs, user_refs(&single.refs)),
            missing: sorted_union(&profile.missing, user_strs(&single.missing)),
            unresolved: sorted_union(&profile.unresolved, user_strs(&single.unresolved)),
            folds,
            reflect_members: by_str(&profile.reflect_members, user_refs(&single.reflect_members)),
            reflect_gaps: sorted_union(&profile.reflect_gaps, user_strs(&single.reflect_gaps)),
            reflect_fields: sorted_union(&profile.reflect_fields, single.reflect_fields.iter().filter(|(o, _)| is_user(o)).cloned()),
            reflect_field_names: sorted_union(&profile.reflect_field_names, single.reflect_field_names.iter().cloned()),
            reflect_static_fields: sorted_union(
                &profile.reflect_static_fields,
                single.reflect_static_fields.iter().filter(|(o, _)| is_user(o)).cloned(),
            ),
            reflect_allocations: sorted_union(&profile.reflect_allocations, user_strs(&single.reflect_allocations)),
            reflect_meta_methods: sorted_union(&profile.reflect_meta_methods, user_strs(&single.reflect_meta_methods)),
            reflect_meta_fields: sorted_union(&profile.reflect_meta_fields, user_strs(&single.reflect_meta_fields)),
            seeds: SeedFacts {
                annotation_enums: sorted_union(&p.annotation_enums, user_strs(&s.annotation_enums)),
                mirror_inits: sorted_union(&p.mirror_inits, user_strs(&s.mirror_inits)),
                reflect_names,
                reflect_all: p.reflect_all.iter().cloned().chain(s.reflect_all.iter().filter(|c| is_user(c)).cloned()).collect(),
                module_services,
                // 资源路径不分属主：两侧取并（嵌入时只取本例类路径上存在的）
                named_resources: p.named_resources.union(&s.named_resources).cloned().collect(),
            },
            dispatched: by_str(&profile.dispatched, user_refs(&single.dispatched)),
            instantiated: sorted_union(&profile.instantiated, user_strs(&single.instantiated)),
            hw_inherited: by_str(&profile.hw_inherited, user_refs(&single.hw_inherited)),
            sam_types: sorted_union(&profile.sam_types, user_strs(&single.sam_types)),
            system_properties: profile.system_properties.clone(),
            boot_image: profile.boot_image.clone(),
        })
    }
}

/// 单例的非用户类 / 方法须被档案覆盖
fn check_covered(profile: &ClosureFacts, single: &ClosureFacts, user: &BTreeSet<&str>) -> Result<(), InputError> {
    let classes: BTreeMap<&str, &ClassFact> = profile.classes.iter().map(|c| (c.name.as_str(), c)).collect();
    let methods: BTreeSet<String> = profile.methods.iter().map(|m| m.id.to_string()).collect();
    let mut gaps: Vec<String> = Vec::new();
    for c in single.classes.iter().filter(|c| c.domain != Domain::User) {
        match classes.get(c.name.as_str()) {
            None => gaps.push(format!("类 {}", c.name)),
            Some(p) if p.domain != c.domain || p.level < c.level => {
                gaps.push(format!("类 {}（档案 {:?}/{:?}，本程序 {:?}/{:?}）", c.name, p.domain, p.level, c.domain, c.level))
            }
            Some(_) => {}
        }
    }
    for m in single.methods.iter().filter(|m| !user.contains(m.id.owner.as_str())) {
        if !methods.contains(&m.id.to_string()) {
            gaps.push(format!("方法 {}", m.id));
        }
    }
    let refs: BTreeSet<String> = profile.refs.iter().map(ToString::to_string).collect();
    for r in single.refs.iter().filter(|r| !user.contains(r.owner.as_str())) {
        if !refs.contains(&r.to_string()) {
            gaps.push(format!("调用点引用 {r}"));
        }
    }
    let sam: BTreeSet<&str> = profile.sam_types.iter().map(String::as_str).collect();
    for t in single.sam_types.iter().filter(|t| !user.contains(t.as_str()) && !sam.contains(t.as_str())) {
        gaps.push(format!("lambda 接口 {t}"));
    }
    if gaps.is_empty() {
        return Ok(());
    }
    let n = gaps.len();
    gaps.truncate(8);
    Err(InputError::Format(format!("档案未覆盖本程序（{n} 项，先把本程序并入档案）：{}", gaps.join("；"))))
}

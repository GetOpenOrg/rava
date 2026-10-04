//! 档案（profile）：一个构建单元全体入口的调用链并集（T1 档案化，计划
//! `docs/plans/2026-10-01-cross-test-compile-reuse.md` §5.3 第 1 步）。
//!
//! 档案 = 各入口单例闭包（开放世界，见 `engine/open_world.rs`）的并（join），只含非用户侧（domain ≠ user）的事实：
//! - 每个入口在自己的用户命名空间里单独分析（语料里不同测试有同名用户类），档案恰为各单例闭包非用户部分的并，
//!   不是超集；
//! - 并运算可交换、可结合，与入口给出顺序无关；溯源（via）取名字最小的入口，只作诊断，不入内容摘要；
//! - 类：domain 须一致，层级取最高；方法：种类须一致；各集合字段取并；系统属性表须一致（同一清单）；
//! - 折叠（folds）：在到达该方法的全部入口上逐点取格的并（[`fold_join`]），并后的折叠对每个入口都成立。
//!
//! 产出 `profile.json` 与 closure.json 同形（`input::ClosureFacts::from_json` 可直接读取），另带 `profile` 段：
//! 格式版本、档案键、内容摘要、键的各输入、各入口统计。诊断字段（indy_models / sigpoly_sites / dispatch /
//! hw_untyped_sites）不入档案。

mod digest;
mod fold_join;
#[cfg(test)]
mod tests;

pub use digest::{archives_digest, content_digest, entries_digest, files_digest, profile_key, runtime_digest, KeyInputs};

use std::collections::{BTreeMap, BTreeSet};

use serde_json::{json, Value};

/// profile.json 格式版本（入档案键）
pub const PROFILE_FORMAT: u32 = 1;

/// 一个入口的单例闭包（`Closure::to_json()` 形）
pub struct EntryClosure {
    pub name: String,
    pub closure: Value,
}

/// 字节码方法的异常处理器起点（`类.方法:描述符` → 处理器偏移；找不到方法字节码时 None）
pub type HandlerTable<'a> = dyn Fn(&str) -> Option<Vec<u32>> + 'a;

/// 成员 id / 类名 / 调用点串的属主类（内部名不含 `.`、`@`、空格）
fn owner_of(s: &str) -> &str {
    s.split(['.', '@', ' ']).next().unwrap_or(s)
}

fn level_rank(l: &str) -> Option<u8> {
    ["type", "layout", "init", "alloc", "code"].iter().position(|x| *x == l).map(|i| i as u8)
}

fn arr<'v>(v: &'v Value, key: &str) -> Result<&'v Vec<Value>, String> {
    v.get(key).and_then(Value::as_array).ok_or_else(|| format!("closure 缺字段：{key}"))
}

fn str_at<'v>(v: &'v Value, key: &str) -> Result<&'v str, String> {
    v.get(key).and_then(Value::as_str).ok_or_else(|| format!("closure 条目缺字符串字段 {key}：{v}"))
}

/// 按点分路径取值（`reflect.gaps`）
fn at<'v>(v: &'v Value, path: &str) -> Option<&'v Value> {
    path.split('.').try_fold(v, |v, k| v.get(k))
}

/// 取并的字符串集合字段，及是否按属主过滤用户类
const SETS: &[(&str, bool)] = &[
    ("clinit", true),
    ("instantiated", true),
    ("dispatched", true),
    ("refs", true),
    ("unresolved", true),
    ("hw_inherited", true),
    ("sam_types", true),
    ("reflect.gaps", true),
    ("reflect.field_names", false),
    ("reflect.field_enum_gaps", true),
    ("reflect.allocations", true),
    ("seeds.annotation_enums", true),
    ("seeds.mirror_inits", true),
    ("seeds.reflect_all", true),
];

struct ClassRec {
    domain: String,
    level: String,
    via: Value,
    entry: String,
}

struct MethodRec {
    kind: String,
    via: Value,
    fns: Option<Value>,
    entry: String,
}

#[derive(Default)]
struct Acc {
    classes: BTreeMap<String, ClassRec>,
    methods: BTreeMap<String, MethodRec>,
    /// 字节码方法 → 各到达入口的折叠记录（无记录即空折叠）
    folds: BTreeMap<String, Vec<Option<Value>>>,
    sets: BTreeMap<&'static str, BTreeSet<String>>,
    missing: BTreeMap<String, Value>,
    reflect_members: BTreeSet<(String, String)>,
    reflect_fields: BTreeSet<(String, String)>,
    reflect_names: BTreeMap<String, BTreeSet<String>>,
    services: BTreeMap<String, BTreeSet<(Option<String>, String)>>,
    services_unknown: bool,
    sysprops: Option<Value>,
}

impl Acc {
    fn add(&mut self, e: &EntryClosure) -> Result<(), String> {
        let c = &e.closure;
        let fv = c.get("folds_version").and_then(Value::as_u64);
        if fv != Some(crate::FOLDS_VERSION as u64) {
            return Err(format!("入口 {} 的 folds_version 为 {fv:?}，应为 {}", e.name, crate::FOLDS_VERSION));
        }
        let mut user: BTreeSet<&str> = BTreeSet::new();
        for k in arr(c, "classes")? {
            let (name, domain, level) = (str_at(k, "name")?, str_at(k, "domain")?, str_at(k, "level")?);
            if domain == "user" {
                user.insert(name);
                continue;
            }
            let rank = level_rank(level).ok_or_else(|| format!("类 {name} 的层级未知：{level}"))?;
            match self.classes.get_mut(name) {
                None => {
                    let via = k.get("via").cloned().unwrap_or(Value::Null);
                    self.classes.insert(name.into(), ClassRec { domain: domain.into(), level: level.into(), via, entry: e.name.clone() });
                }
                Some(r) if r.domain != domain => {
                    return Err(format!("类 {name} 的 domain 不一致：入口 {} 为 {}，入口 {} 为 {domain}", r.entry, r.domain, e.name));
                }
                Some(r) => {
                    if level_rank(&r.level) < Some(rank) {
                        r.level = level.into();
                    }
                }
            }
        }
        let is_user = |s: &str| user.contains(owner_of(s));
        let folds: BTreeMap<&str, &Value> =
            arr(c, "folds")?.iter().map(|f| Ok((str_at(f, "method")?, f))).collect::<Result<_, String>>()?;
        for m in arr(c, "methods")? {
            let (id, kind) = (str_at(m, "id")?, str_at(m, "kind")?);
            if is_user(id) {
                continue;
            }
            match self.methods.get(id) {
                None => {
                    let rec = MethodRec { kind: kind.into(), via: m.get("via").cloned().unwrap_or(Value::Null), fns: m.get("fns").cloned(), entry: e.name.clone() };
                    self.methods.insert(id.into(), rec);
                }
                Some(r) if r.kind != kind => {
                    return Err(format!("方法 {id} 的种类不一致：入口 {} 为 {}，入口 {} 为 {kind}", r.entry, r.kind, e.name));
                }
                Some(_) => {}
            }
            if kind == "bytecode" {
                self.folds.entry(id.into()).or_default().push(folds.get(id).map(|f| (*f).clone()));
            }
        }
        for &(path, filter) in SETS {
            let set = self.sets.entry(path).or_default();
            for s in at(c, path).and_then(Value::as_array).into_iter().flatten().filter_map(Value::as_str) {
                if !(filter && is_user(s)) {
                    set.insert(s.to_string());
                }
            }
        }
        for m in arr(c, "missing")? {
            let name = str_at(m, "name")?;
            if !is_user(name) && !self.missing.contains_key(name) {
                self.missing.insert(name.into(), m.get("via").cloned().unwrap_or(Value::Null));
            }
        }
        let reflect = c.get("reflect").ok_or("closure 缺字段：reflect")?;
        for r in arr(reflect, "members")? {
            let member = str_at(r, "member")?;
            if !is_user(member) {
                self.reflect_members.insert((str_at(r, "kind")?.into(), member.into()));
            }
        }
        for r in arr(reflect, "fields")? {
            let owner = str_at(r, "owner")?;
            if !is_user(owner) {
                self.reflect_fields.insert((owner.into(), str_at(r, "name")?.into()));
            }
        }
        let seeds = c.get("seeds").ok_or("closure 缺字段：seeds")?;
        for (owner, names) in seeds.get("reflect_names").and_then(Value::as_object).into_iter().flatten() {
            if !is_user(owner) {
                let set = self.reflect_names.entry(owner.clone()).or_default();
                set.extend(names.as_array().into_iter().flatten().filter_map(Value::as_str).map(String::from));
            }
        }
        for s in seeds.get("services").and_then(Value::as_array).into_iter().flatten() {
            let service = str_at(s, "service")?;
            if is_user(service) {
                continue;
            }
            for p in arr(s, "providers")? {
                let class = str_at(p, "class")?;
                if !is_user(class) {
                    let module = p.get("module").and_then(Value::as_str).map(String::from);
                    self.services.entry(service.into()).or_default().insert((module, class.into()));
                }
            }
        }
        self.services_unknown |= seeds.get("services_unknown").and_then(Value::as_bool).unwrap_or(false);
        let sp = c.get("system_properties").ok_or("closure 缺字段：system_properties")?;
        match &self.sysprops {
            None => self.sysprops = Some(sp.clone()),
            Some(p) if p != sp => return Err(format!("入口 {} 的系统属性表与其他入口不一致（运行时清单不同）", e.name)),
            Some(_) => {}
        }
        Ok(())
    }

    fn finish(self, handlers: &HandlerTable) -> Result<Value, String> {
        let set = |k: &str| self.sets.get(k).map(|s| s.iter().cloned().collect::<Vec<_>>()).unwrap_or_default();
        let mut folds = Vec::new();
        for (id, inputs) in &self.folds {
            if let Some(f) = fold_join::join(id, inputs, handlers)? {
                folds.push(f);
            }
        }
        let classes: Vec<Value> = self
            .classes
            .iter()
            .map(|(n, r)| json!({"name": n, "domain": r.domain, "level": r.level, "via": r.via, "entry": r.entry}))
            .collect();
        let methods: Vec<Value> = self
            .methods
            .iter()
            .map(|(id, r)| {
                let mut v = json!({"id": id, "kind": r.kind, "via": r.via, "entry": r.entry});
                if let Some(f) = &r.fns {
                    v["fns"] = f.clone();
                }
                v
            })
            .collect();
        let summary = summary(&self, folds.len());
        Ok(json!({
            "summary": summary,
            "classes": classes,
            "methods": methods,
            "instantiated": set("instantiated"),
            "clinit": set("clinit"),
            "missing": self.missing.iter().map(|(n, v)| json!({"name": n, "via": v})).collect::<Vec<_>>(),
            "unresolved": set("unresolved"),
            "refs": set("refs"),
            "dispatched": set("dispatched"),
            "hw_inherited": set("hw_inherited"),
            "sam_types": set("sam_types"),
            "folds_version": crate::FOLDS_VERSION,
            "folds": folds,
            "system_properties": self.sysprops.clone().unwrap_or_else(|| json!({"values": {}, "dynamic": []})),
            "reflect": {
                "members": self.reflect_members.iter().map(|(k, m)| json!({"kind": k, "member": m})).collect::<Vec<_>>(),
                "gaps": set("reflect.gaps"),
                "fields": self.reflect_fields.iter().map(|(o, n)| json!({"owner": o, "name": n})).collect::<Vec<_>>(),
                "field_names": set("reflect.field_names"),
                "field_enum_gaps": set("reflect.field_enum_gaps"),
                "allocations": set("reflect.allocations"),
            },
            "seeds": {
                "annotation_enums": set("seeds.annotation_enums"),
                "mirror_inits": set("seeds.mirror_inits"),
                "reflect_names": self.reflect_names,
                "reflect_all": set("seeds.reflect_all"),
                "services": self.services.iter().map(|(s, ps)| json!({
                    "service": s,
                    "providers": ps.iter().map(|(m, c)| json!({"module": m, "class": c})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "services_unknown": self.services_unknown,
            },
        }))
    }
}

fn summary(a: &Acc, fold_methods: usize) -> Value {
    let mut by_domain: BTreeMap<&str, usize> = BTreeMap::new();
    let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
    for r in a.classes.values() {
        *by_domain.entry(&r.domain).or_default() += 1;
        *by_level.entry(&r.level).or_default() += 1;
    }
    let mut by_kind: BTreeMap<&str, usize> = BTreeMap::new();
    for r in a.methods.values() {
        *by_kind.entry(r.kind.split(':').next().unwrap_or(&r.kind)).or_default() += 1;
    }
    json!({
        "classes": a.classes.len(),
        "classes_by_domain": by_domain,
        "classes_by_level": by_level,
        "methods": a.methods.len(),
        "methods_by_kind": by_kind,
        "fold_methods": fold_methods,
    })
}

/// 各入口单例闭包 → 档案事实（closure.json 形，不含 `profile` 段）。入口名须唯一；结果与入口顺序无关
pub fn merge(entries: &[EntryClosure], handlers: &HandlerTable) -> Result<Value, String> {
    let mut order: Vec<&EntryClosure> = entries.iter().collect();
    order.sort_by(|a, b| a.name.cmp(&b.name));
    if let Some(w) = order.windows(2).find(|w| w[0].name == w[1].name) {
        return Err(format!("入口名重复：{}", w[0].name));
    }
    let mut acc = Acc::default();
    for e in order {
        acc.add(e)?;
    }
    acc.finish(handlers)
}

/// 入口统计（`profile.entries`）
fn entry_stats(e: &EntryClosure) -> Value {
    let c = &e.closure;
    let classes = c.get("classes").and_then(Value::as_array).map(Vec::as_slice).unwrap_or_default();
    let user = classes.iter().filter(|k| k.get("domain").and_then(Value::as_str) == Some("user")).count();
    let s = c.get("summary");
    json!({
        "name": e.name,
        "user_classes": user,
        "classes": classes.len() - user,
        "methods": c.get("methods").and_then(Value::as_array).map_or(0, Vec::len),
        "elapsed_ms": s.and_then(|s| s.get("elapsed_ms")).cloned().unwrap_or(Value::Null),
        "peak_mem_mb": s.and_then(|s| at(s, "perf.peak_mem_mb")).cloned().unwrap_or(Value::Null),
    })
}

/// 构建档案：并集事实 + `profile` 段（格式版本、键、内容摘要、键输入、入口统计）
pub fn build(entries: &[EntryClosure], handlers: &HandlerTable, inputs: &KeyInputs) -> Result<Value, String> {
    let mut v = merge(entries, handlers)?;
    let content = content_digest(&v);
    let mut stats: Vec<Value> = entries.iter().map(entry_stats).collect();
    stats.sort_by(|a, b| a["name"].as_str().cmp(&b["name"].as_str()));
    v["profile"] = json!({
        "format": PROFILE_FORMAT,
        "key": profile_key(inputs, &content),
        "content_digest": content,
        "inputs": {
            "generator": inputs.generator,
            "runtime": inputs.runtime,
            "jdk_major": inputs.jdk_major,
            "archives": inputs.archives,
            "entries": inputs.entries,
        },
        "entries": stats,
    });
    Ok(v)
}

/// 档案 P 是否覆盖入口 E（§4.1 子集复用）：E 的单例闭包并入 P 后内容不变
pub fn covers(profile: &Value, entry: &EntryClosure, handlers: &HandlerTable) -> Result<bool, String> {
    let p = EntryClosure { name: String::new(), closure: profile.clone() };
    let joined = merge(&[p, EntryClosure { name: format!("\u{1}{}", entry.name), closure: entry.closure.clone() }], handlers)?;
    Ok(content_digest(&joined) == content_digest(profile))
}

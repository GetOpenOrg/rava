//! rava closure：精确闭包分析器（计划 docs/plans/2026-09-29-rust-closure-analyzer.md）。
//!
//! 入口 [`analyze`]：用户类 main 为根，加 VM 基础设施种子（`error.rs` vm-upcalls、boot_init），
//! 以 XTA + 抽象解释计算调用链实际需要的类、方法、字段、实例化与初始化集合。
//! 每个节点带溯源（via），`why` 沿溯源回溯到根。

pub mod absint;
pub mod engine;
pub mod handwritten;
pub mod manifest;

use std::collections::BTreeMap;
use std::path::Path;

use classfile::MemberRef;
use engine::{Engine, From, Kind, Level, Via};
use handwritten::Handwritten;
use manifest::{Domain, Manifest};
use resolve::{ClassPath, Hierarchy};
use serde_json::{json, Value};

pub struct Input<'a> {
    pub cp: &'a ClassPath,
    /// runtime/java_runtime
    pub runtime_dir: &'a Path,
    /// 入口方法
    pub roots: Vec<MemberRef>,
}

pub struct Closure<'a> {
    pub engine: Engine<'a>,
    pub elapsed_ms: u128,
}

/// 运行分析。`h` / `man` / `hw` 由调用方持有（引擎借用）
pub fn analyze<'a>(input: &Input<'a>, h: &'a Hierarchy<'a>, man: &'a Manifest, hw: &'a Handwritten) -> Closure<'a> {
    let t0 = std::time::Instant::now();
    let mut e = Engine::new(h, input.cp, man, hw);
    for r in &input.roots {
        e.root(r.clone(), "main");
    }
    for u in hw.vm_upcalls() {
        e.root_upcall(&u, "vm-upcalls");
    }
    for c in &man.boot_init {
        e.root_init(c, "boot_init");
    }
    e.run();
    Closure { engine: e, elapsed_ms: t0.elapsed().as_millis() }
}

fn domain_str(d: Domain) -> &'static str {
    match d {
        Domain::User => "user",
        Domain::Translate => "translate",
        Domain::Boundary => "boundary",
        Domain::Root => "root",
    }
}

fn level_str(l: Level) -> &'static str {
    match l {
        Level::Type => "type",
        Level::Init => "init",
        Level::Alloc => "alloc",
        Level::Code => "code",
    }
}

fn kind_str(k: Kind) -> String {
    match k {
        Kind::Bytecode => "bytecode".into(),
        Kind::Handwritten(w) => format!("handwritten:{w}"),
        Kind::Abstract => "abstract".into(),
        Kind::Missing => "missing".into(),
    }
}

impl Closure<'_> {
    fn via_json(&self, v: &Via) -> Value {
        let from = match &v.from {
            From::Root(s) => json!({"root": s}),
            From::Method(i) => json!({"method": self.engine.method_label(*i)}),
            From::Class(c) => json!({"class": c}),
        };
        json!({"kind": v.kind, "from": from, "off": v.off})
    }

    pub fn summary(&self) -> Value {
        let e = &self.engine;
        let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
        let mut by_domain: BTreeMap<&str, usize> = BTreeMap::new();
        for c in e.classes.values() {
            *by_level.entry(level_str(c.level)).or_default() += 1;
            *by_domain.entry(domain_str(c.domain)).or_default() += 1;
        }
        let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for m in e.methods.values() {
            *by_kind.entry(kind_str(m.kind)).or_default() += 1;
        }
        let code_translate = e
            .classes
            .values()
            .filter(|c| c.level == Level::Code && matches!(c.domain, Domain::Translate))
            .count();
        json!({
            "classes": e.classes.len(),
            "classes_by_level": by_level,
            "classes_by_domain": by_domain,
            "translate_code_classes": code_translate,
            "methods": e.methods.len(),
            "methods_by_kind": by_kind,
            "instantiated": e.instantiated().len(),
            "lambdas": e.lambda_count(),
            "clinit": e.inited.len(),
            "missing_classes": e.missing.len(),
            "unresolved": e.unresolved.len(),
            "dead_branch_methods": e.dead.len(),
            "elapsed_ms": self.elapsed_ms,
        })
    }

    pub fn to_json(&self) -> Value {
        let e = &self.engine;
        let classes: Vec<Value> = e
            .classes
            .iter()
            .map(|(n, c)| {
                json!({"name": n, "domain": domain_str(c.domain), "level": level_str(c.level), "via": self.via_json(&c.via)})
            })
            .collect();
        let methods: Vec<Value> = e
            .methods
            .values()
            .map(|m| {
                let mut v = json!({"id": m.key.to_string(), "kind": kind_str(m.kind), "via": self.via_json(&m.via)});
                if !m.hw_fns.is_empty() {
                    v["fns"] = json!(m.hw_fns);
                }
                v
            })
            .collect();
        let dispatch: Vec<Value> = e
            .dispatch
            .iter()
            .filter(|(_, t)| t.len() > 1)
            .map(|((m, off), t)| {
                json!({"site": format!("{}@{off}", e.method_label(*m)),
                       "targets": t.iter().map(|x| e.method_label(*x)).collect::<Vec<_>>()})
            })
            .collect();
        let dead: Vec<Value> = e
            .dead
            .iter()
            .map(|(m, v)| json!({"method": e.method_label(*m), "branches": v}))
            .collect();
        json!({
            "summary": self.summary(),
            "classes": classes,
            "methods": methods,
            "instantiated": e.instantiated(),
            "clinit": e.inited.keys().collect::<Vec<_>>(),
            "missing": e.missing.iter().map(|(n, v)| json!({"name": n, "via": self.via_json(v)})).collect::<Vec<_>>(),
            "unresolved": e.unresolved,
            "dispatch": dispatch,
            "dead_branches": dead,
        })
    }

    fn via_line(&self, v: &Via) -> (String, Option<Via>) {
        let e = &self.engine;
        let off = v.off.map(|o| format!("@{o}")).unwrap_or_default();
        match &v.from {
            From::Root(s) => (format!("[{}] 根 {s}", v.kind), None),
            From::Method(i) => {
                let m = &e.methods[*i];
                (format!("[{}] {}{off}", v.kind, m.key), Some(m.via.clone()))
            }
            From::Class(c) => {
                let next = e.classes.get(c).map(|n| n.via.clone());
                let next = match (v.kind, e.inited.get(c)) {
                    ("clinit" | "super-init" | "iface-init", Some(iv)) => Some(iv.clone()),
                    _ => next,
                };
                (format!("[{}] 类 {c}", v.kind), next)
            }
        }
    }

    /// 溯源链：类名（`a/b/C`）或方法（`a/b/C.m:(..)V`）
    pub fn why(&self, target: &str) -> Vec<String> {
        let e = &self.engine;
        let mut out = Vec::new();
        let start = if let Some(c) = e.classes.get(target) {
            let mut lines = vec![format!("{target}（{}，{}）", domain_str(c.domain), level_str(c.level))];
            for (l, v) in &c.level_via {
                if *l != Level::Type || c.level_via.len() == 1 {
                    lines.push(format!("  {} 首次：{}", level_str(*l), self.via_line(v).0));
                }
            }
            out.extend(lines);
            Some(c.level_via.values().next_back().cloned().unwrap_or_else(|| c.via.clone()))
        } else if let Some(m) = e.methods.values().find(|m| m.key.to_string() == target) {
            out.push(format!("{target}（{}）", kind_str(m.kind)));
            Some(m.via.clone())
        } else {
            out.push(format!("{target}：不在闭包内"));
            None
        };
        let mut cur = start;
        let mut depth = 0;
        while let Some(v) = cur {
            let (line, next) = self.via_line(&v);
            out.push(format!("{}← {line}", "  ".repeat(depth + 1)));
            cur = next;
            depth += 1;
            if depth > 200 {
                out.push("  …（截断）".into());
                break;
            }
        }
        out
    }

    /// Markdown 报告：总量、分层、按包分布、Top 引入者
    pub fn report_md(&self, title: &str) -> String {
        let e = &self.engine;
        let s = self.summary();
        let mut md = format!("# 闭包报告：{title}\n\n");
        md.push_str("| 指标 | 值 |\n|---|---:|\n");
        for k in [
            "classes",
            "translate_code_classes",
            "methods",
            "instantiated",
            "lambdas",
            "clinit",
            "missing_classes",
            "unresolved",
            "dead_branch_methods",
            "elapsed_ms",
        ] {
            md.push_str(&format!("| {k} | {} |\n", s[k]));
        }
        md.push_str("\n## 类按层级\n\n| 层级 | 类数 |\n|---|---:|\n");
        for (k, v) in s["classes_by_level"].as_object().into_iter().flatten() {
            md.push_str(&format!("| {k} | {v} |\n"));
        }
        md.push_str("\n## 类按域\n\n| 域 | 类数 |\n|---|---:|\n");
        for (k, v) in s["classes_by_domain"].as_object().into_iter().flatten() {
            md.push_str(&format!("| {k} | {v} |\n"));
        }
        md.push_str("\n## 方法按种类\n\n| 种类 | 方法数 |\n|---|---:|\n");
        for (k, v) in s["methods_by_kind"].as_object().into_iter().flatten() {
            md.push_str(&format!("| {k} | {v} |\n"));
        }
        let mut pkgs: BTreeMap<&str, [usize; 4]> = BTreeMap::new();
        for (n, c) in &e.classes {
            let p = resolve::package_of(n);
            pkgs.entry(p).or_default()[c.level as usize] += 1;
        }
        let mut pv: Vec<_> = pkgs.into_iter().collect();
        pv.sort_by_key(|(_, v)| std::cmp::Reverse(v.iter().sum::<usize>()));
        md.push_str("\n## 包分布（Top 40）\n\n| 包 | type | init | alloc | code |\n|---|---:|---:|---:|---:|\n");
        for (p, v) in pv.into_iter().take(40) {
            md.push_str(&format!("| {p} | {} | {} | {} | {} |\n", v[0], v[1], v[2], v[3]));
        }
        // 引入者：code 层类的首个 via 所在方法的类
        let mut intro: BTreeMap<String, usize> = BTreeMap::new();
        for c in e.classes.values() {
            if let From::Method(i) = &c.via.from {
                *intro.entry(e.methods[*i].key.owner.clone()).or_default() += 1;
            }
        }
        let mut iv: Vec<_> = intro.into_iter().collect();
        iv.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
        md.push_str("\n## 引入类最多的类（Top 30）\n\n| 类 | 首次引入类数 |\n|---|---:|\n");
        for (c, n) in iv.into_iter().take(30) {
            md.push_str(&format!("| {c} | {n} |\n"));
        }
        md
    }
}

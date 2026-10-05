//! rava closure：精确闭包分析器（计划 docs/plans/2026-09-29-rust-closure-analyzer.md）。
//!
//! 入口 [`analyze`]：用户类 main 为根，加 VM 基础设施种子（VM 规则、boot_init），
//! 以 XTA + 抽象解释计算调用链实际需要的类、方法、字段、实例化与初始化集合。
//! 每个节点带溯源（via），`why` 沿溯源回溯到根。

pub mod absint;
pub mod cache;
pub mod cold;
pub mod engine;
pub mod handwritten;
pub mod manifest;
pub mod modules_json;
pub mod loaders;
pub mod profile;
pub mod seeds;

use std::collections::BTreeMap;
use std::path::Path;

use classfile::MemberRef;
use absint::V;
use engine::{Engine, Fold, From, Kind, Level, Via};
use handwritten::Handwritten;
use manifest::{Domain, Manifest, Members};
use resolve::{ClassPath, Hierarchy};
use serde_json::{json, Value};

pub struct Input<'a> {
    pub cp: &'a ClassPath,
    /// runtime/java_runtime
    pub runtime_dir: &'a Path,
    /// 入口方法
    pub roots: Vec<MemberRef>,
    /// 外部种子方法（`--root` / `--seed-class` 展开）
    pub seed_roots: Vec<MemberRef>,
    /// `--locale` 显式给出的 locale 标签
    pub locales: Vec<String>,
    /// 诊断选项（反事实切除 / 触发边转储；缺省关闭，不影响闭包结果）
    pub diag: engine::Diag,
    /// 诊断 `--cold-cut`：丢弃冷路径事件（不健全，只用于测量冷路径独占规模）
    pub cold_cut: bool,
    /// 流传播批量（`--flow-batch`；缺省 `engine::FLOW_BATCH`）
    pub flow_batch: Option<usize>,
}

pub struct Closure<'a> {
    pub engine: Engine<'a>,
    /// 类路径（输出的模块事实取自它，见 [`modules_json`]）
    pub cp: &'a ClassPath,
    pub elapsed_ms: u128,
}

/// 运行分析。`h` / `man` / `hw` 由调用方持有（引擎借用）
pub fn analyze<'a>(input: &Input<'a>, h: &'a Hierarchy<'a>, man: &'a Manifest, hw: &'a Handwritten) -> Closure<'a> {
    let t0 = std::time::Instant::now();
    let mut e = Engine::new(h, input.cp, man, hw);
    e.cuts = engine::cut::Cuts::parse(&input.diag.cuts);
    e.arm_probes(&input.diag.flows);
    if input.diag.dump_edges.is_some() {
        engine::cut::edges_begin();
    }
    e.seeds.locales = input.locales.clone();
    e.cold_cut = input.cold_cut;
    e.set_site_prof(input.diag.site_prof);
    if let Some(n) = input.flow_batch.filter(|&n| n > 0) {
        e.flow_batch = n;
    }
    for r in &input.roots {
        e.root(r.clone(), "main");
    }
    for r in &input.seed_roots {
        e.root_seed(r.clone(), "seed");
    }
    for n in hw.vm_field_writes() {
        e.open_vm_field_write(&n);
    }
    e.root_vm_rules();
    for c in &man.boot_init {
        e.root_init(c, "boot_init");
    }
    for m in &man.boot_calls {
        e.root_boot_call(m, "boot_init");
    }
    e.run();
    if let Some(p) = &input.diag.dump_edges {
        if let Err(err) = engine::cut::edges_finish(p) {
            eprintln!("[closure] 触发边转储写入失败：{}：{err}", p.display());
        }
    }
    Closure { engine: e, cp: input.cp, elapsed_ms: t0.elapsed().as_millis() }
}

/// closure.json 折叠点格式版本（计划 §7.3「折叠点导出」）
pub const FOLDS_VERSION: u32 = 2;

/// 常量值按 folds 约定编码：Z → 布尔；B/C/S/I → 整数；J → 字符串；null → null；字符串常量 → 字符串
fn const_json(v: &V, ty: &str) -> Value {
    match v {
        V::Int(i) if ty == "Z" => json!(*i != 0),
        V::Int(i) => json!(i),
        V::Long(l) => json!(l.to_string()),
        V::Str(s, _) => json!(s.as_ref()),
        _ => Value::Null,
    }
}

fn fold_json(f: &Fold) -> Value {
    let kind = |op: u8| match op {
        classfile::op::GETFIELD => "getfield",
        classfile::op::GETSTATIC => "getstatic",
        _ => "invoke",
    };
    json!({
        "method": f.method,
        "dead_pcs": f.dead_pcs.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(),
        "dead_handlers": f.dead_handlers,
        "dead_catches": f.dead_catches.iter().map(|c| json!({"start": c.start, "end": c.end, "handler": c.handler, "catch_type": c.catch_type})).collect::<Vec<_>>(),
        "consts": f.consts.iter().map(|(pc, op, v, ty)| json!({"pc": pc, "kind": kind(*op), "value": const_json(v, ty), "type": ty})).collect::<Vec<_>>(),
        "null_recv": f.null_recv,
        "noreturn_calls": f.noreturn_calls,
        "noreturn_dead_pcs": f.noreturn_dead_pcs.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(),
    })
}

fn domain_str(d: Domain) -> &'static str {
    match d {
        Domain::User => "user",
        Domain::Translate => "translate",
        Domain::Boundary => "boundary",
        Domain::Root => "root",
    }
}

fn members_str(k: Members) -> &'static str {
    match k {
        Members::Methods => "method",
        Members::Constructors => "constructor",
        Members::RecordAccessors => "record_accessor",
    }
}

fn level_str(l: Level) -> &'static str {
    match l {
        Level::Type => "type",
        Level::Layout => "layout",
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
        self.summary_with(&self.engine.folds())
    }

    fn summary_with(&self, folds: &[engine::Fold]) -> Value {
        let e = &self.engine;
        let mut by_level: BTreeMap<&str, usize> = BTreeMap::new();
        let mut by_domain: BTreeMap<&str, usize> = BTreeMap::new();
        for c in e.classes.values() {
            *by_level.entry(level_str(c.level)).or_default() += 1;
            *by_domain.entry(domain_str(c.domain)).or_default() += 1;
        }
        let mut by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for m in e.method_nodes() {
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
            "methods": e.method_count(),
            "method_contexts": e.methods.len(),
            "context_objects": e.objs.len(),
            "methods_by_kind": by_kind,
            "instantiated": e.instantiated().len(),
            "lambdas": e.lambda_count(),
            "clinit": e.inited.len(),
            "missing_classes": e.missing.len(),
            "unresolved": e.unresolved.len(),
            "fold_methods": folds.len(),
            "fold_consts": folds.iter().map(|f| f.consts.len()).sum::<usize>(),
            "fold_violations": folds.iter().map(|f| f.violations.len()).sum::<usize>(),
            "fold_null_recv": folds.iter().map(|f| f.null_recv.len()).sum::<usize>(),
            "fold_noreturn_calls": folds.iter().map(|f| f.noreturn_calls.len()).sum::<usize>(),
            "fold_noreturn_dead_bytes": folds.iter().flat_map(|f| &f.noreturn_dead_pcs).map(|(a, b)| b - a).sum::<u32>(),
            "fold_props": folds.iter().map(|f| f.props.len()).sum::<usize>(),
            "sysprops_unstable": e.sysprops_report(),
            "reflect_members": e.reflect_members.len(),
            "reflect_gaps": e.reflect_gaps.len(),
            "field_enum_gaps": e.field_enum_gaps.len(),
            "rcall": e.rcall_summary(),
            "field_writer_live": e.field_handles().0,
            "field_writer_cause": e.field_writer_cause(),
            "field_handle_released": e.field_handle_released(),
            "fields_open_all": e.field_handles().1,
            "hw_written_fields": e.hw_written.len(),
            "hw_written_names": e.hw_written_names,
            "elapsed_ms": self.elapsed_ms,
            // 性能观测（计时 / 内存 / 重分析分布）：不属于分析结果，对照输出时与 elapsed_ms 一并剔除
            "perf": e.perf_json(20),
        })
    }

    pub fn to_json(&self) -> Value {
        let e = &self.engine;
        let mut classes: Vec<Value> = e
            .class_entries()
            .into_iter()
            .map(|(n, c)| {
                json!({"name": n, "domain": domain_str(c.domain), "level": level_str(c.level), "via": self.via_json(&c.via)})
            })
            .collect();
        let modules = modules_json::annotate(&resolve::ModuleFacts::build(self.cp).graph(self.cp), &mut classes);
        let methods: Vec<Value> = e
            .method_entries()
            .into_iter()
            .map(|m| {
                let mut v = json!({"id": m.key.to_string(), "kind": kind_str(m.kind), "via": self.via_json(&m.via)});
                if !m.hw_fns.is_empty() {
                    v["fns"] = json!(m.hw_fns);
                }
                v
            })
            .collect();
        let dispatch: Vec<Value> = e
            .dispatch_sites()
            .into_iter()
            .filter(|(_, t)| t.len() > 1)
            .map(|((m, off), t)| json!({"site": format!("{m}@{off}"), "targets": t}))
            .collect();
        let fold_list = e.folds();
        let folds: Vec<Value> = fold_list.iter().map(fold_json).collect();
        json!({
            "summary": self.summary_with(&fold_list),
            "classes": classes,
            "modules": modules,
            "methods": methods,
            "instantiated": e.instantiated(),
            "clinit": e.clinit_list(),
            "missing": e.missing.iter().map(|(n, v)| json!({"name": n, "via": self.via_json(v)})).collect::<Vec<_>>(),
            "unresolved": e.unresolved,
            "refs": e.refs,
            "indy_models": e.indy_models.iter().map(|(site, (bsm, k))| json!({"site": site, "bootstrap": bsm, "kind": indy_str(*k)})).collect::<Vec<_>>(),
            "sigpoly_sites": e.sigpoly_sites,
            "dispatch": dispatch,
            "dispatched": e.dispatched(),
            "hw_inherited": e.hw_inherited_requests().iter().map(|r| r.to_string()).collect::<Vec<_>>(),
            "hw_untyped_sites": e.hw_untyped_sites(),
            "sam_types": e.sam_types(),
            "folds_version": FOLDS_VERSION,
            "folds": folds,
            "system_properties": {
                "values": e.sysprops().values(),
                "dynamic": e.sysprops().dynamic(),
            },
            "reflect": {
                "members": e.reflect_members.iter().map(|(k, m)| json!({"kind": members_str(*k), "member": m.to_string()})).collect::<Vec<_>>(),
                "gaps": e.reflect_gaps,
                "fields": e.reflect_fields.iter().map(|(c, n)| json!({"owner": c, "name": n})).collect::<Vec<_>>(),
                "field_names": e.reflect_field_names,
                "static_fields": e.static_field_handles().iter().map(|(c, n)| json!({"owner": c, "name": n})).collect::<Vec<_>>(),
                "field_enum_gaps": e.field_enum_gaps,
                "allocations": e.serial_allocs,
                "meta_methods": e.meta_method_classes(),
                "meta_fields": e.meta_field_classes(),
            },
            "seeds": {
                "annotation_enums": e.seeds.annotation_enums,
                "mirror_inits": e.seeds.mirror_inits,
                "reflect_names": e.seeds.reflect_names,
                "reflect_all": e.seeds.reflect_all,
                "services": e.seeds.services.selected.iter().map(|(s, ps)| json!({
                    "service": s,
                    "providers": ps.iter().map(|p| json!({"module": p.module, "class": p.class})).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "services_unknown": e.seeds.services.unknown,
            },
        })
    }

    fn via_line(&self, v: &Via) -> (String, Option<Via>) {
        let e = &self.engine;
        let off = v.off.map(|o| format!("@{o}")).unwrap_or_default();
        match &v.from {
            From::Root(s) => (format!("[{}] 根 {s}", v.kind), None),
            From::Method(i) => {
                // 方法节点按 (成员, 克隆上下文) 区分，溯源链沿节点自身的首达边走：标出上下文，
                // 同一成员的不同克隆在链上可辨（出口归因按上下文，见 c1d 计划 §23.6 精度项 1）
                let m = &e.methods[*i];
                (format!("[{}] {}{off}", v.kind, e.ctx_label(*i)), Some(m.via.clone()))
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

    /// 类型流诊断（`--flows <方法标签片段>`）
    pub fn flows(&self, pat: &str) -> Vec<String> {
        self.engine.flows_of(pat)
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
        } else if let Some(m) = e.method_nodes().find(|m| m.key.to_string() == target) {
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
        let mut pkgs: BTreeMap<&str, [usize; 5]> = BTreeMap::new();
        for (n, c) in &e.classes {
            let p = resolve::package_of(n);
            pkgs.entry(p).or_default()[c.level as usize] += 1;
        }
        let mut pv: Vec<_> = pkgs.into_iter().collect();
        pv.sort_by_key(|(_, v)| std::cmp::Reverse(v.iter().sum::<usize>()));
        md.push_str("\n## 包分布（Top 40）\n\n| 包 | type | layout | init | alloc | code |\n|---|---:|---:|---:|---:|---:|\n");
        for (p, v) in pv.into_iter().take(40) {
            md.push_str(&format!("| {p} | {} | {} | {} | {} | {} |\n", v[0], v[1], v[2], v[3], v[4]));
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

/// indy 运行模型类别名（closure.json `indy_models`）
fn indy_str(k: manifest::IndyKind) -> &'static str {
    match k {
        manifest::IndyKind::Lambda => "lambda",
        manifest::IndyKind::Concat => "concat",
        manifest::IndyKind::Native => "native",
        manifest::IndyKind::ObjectMethods => "object_methods",
    }
}

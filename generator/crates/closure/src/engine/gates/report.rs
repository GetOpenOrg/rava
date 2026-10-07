//! 门排名输出：机读 JSON 与人读排名表。实测（子进程 `--cut-file` 重跑）有结果时排名按实测单切 Δ，否则按模型 Δ。

use std::collections::{HashMap, HashSet};

use serde_json::{json, Value};

use super::{category_counts, GatePlan, ModelDelta, RealRun, Target, VerifyReq};

/// 一次实测相对基线的变化
pub(super) struct Real {
    /// 类 / 方法减少量（负 = 反升：切除非单调）
    pub classes: i64,
    pub methods: i64,
    /// 基线没有、切除后出现的类数（> 0 即非单调）
    pub added: usize,
    pub packages: Vec<(String, usize)>,
}

pub(super) struct RealIndex {
    by: HashMap<Target, Result<Real, String>>,
}

impl RealIndex {
    pub fn new(p: &GatePlan, reqs: &[VerifyReq], runs: &[Result<RealRun, String>]) -> RealIndex {
        let base: HashSet<&str> = p.base_classes.iter().map(String::as_str).collect();
        let mut by = HashMap::new();
        for (r, run) in reqs.iter().zip(runs) {
            let v = run.as_ref().map_err(Clone::clone).map(|run| {
                let now: HashSet<&str> = run.classes.iter().map(String::as_str).collect();
                let mut pk: HashMap<&str, usize> = HashMap::new();
                for c in base.iter().filter(|c| !now.contains(*c)) {
                    *pk.entry(c.rsplit_once('/').map_or("", |x| x.0)).or_default() += 1;
                }
                let mut packages: Vec<(String, usize)> = pk.into_iter().map(|(k, n)| (k.to_string(), n)).collect();
                packages.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
                packages.truncate(6);
                Real {
                    classes: p.base.classes as i64 - run.classes.len() as i64,
                    methods: p.base.methods as i64 - run.methods as i64,
                    added: now.iter().filter(|c| !base.contains(*c)).count(),
                    packages,
                }
            });
            by.insert(r.target, v);
        }
        RealIndex { by }
    }

    pub fn get(&self, t: Target) -> Option<&Result<Real, String>> {
        self.by.get(&t)
    }

    fn ok(&self, t: Target) -> Option<&Real> {
        self.get(t).and_then(|r| r.as_ref().ok())
    }
}

fn pk_json(v: &[(String, usize)]) -> Value {
    Value::Array(v.iter().map(|(p, n)| json!({"package": p, "classes": n})).collect())
}

fn model_json(d: &ModelDelta) -> Value {
    json!({"classes": d.classes, "methods": d.methods, "packages": pk_json(&d.packages)})
}

fn real_json(r: Option<&Result<Real, String>>) -> Value {
    match r {
        None => Value::Null,
        Some(Err(e)) => json!({"error": e}),
        Some(Ok(r)) => json!({"classes": r.classes, "methods": r.methods, "added": r.added, "packages": pk_json(&r.packages)}),
    }
}

fn pk_text(v: &[(String, usize)]) -> String {
    v.iter().take(3).map(|(p, n)| format!("{p} {n}")).collect::<Vec<_>>().join("，")
}

/// 排名序：实测单切 Δ（有）优先，否则模型 Δ；同值按首达树经过类数
pub(super) fn order(p: &GatePlan, real: &RealIndex) -> Vec<usize> {
    let key = |i: usize| real.ok(Target::Single(i)).map_or(p.gates[i].single.classes as i64, |r| r.classes);
    let mut v: Vec<usize> = (0..p.gates.len()).collect();
    v.sort_by(|&a, &b| key(b).cmp(&key(a)).then(p.gates[b].tree_classes.cmp(&p.gates[a].tree_classes)).then(a.cmp(&b)));
    v
}

pub(super) fn json(p: &GatePlan, real: &RealIndex, reqs: &[VerifyReq], runs: &[Result<RealRun, String>], meta: Value) -> Value {
    let step_of: HashMap<usize, usize> = p.greedy.iter().enumerate().map(|(k, s)| (s.gate, k)).collect();
    let gates: Vec<Value> = order(p, real)
        .into_iter()
        .enumerate()
        .map(|(rank, i)| {
            let g = &p.gates[i];
            let step = step_of.get(&i).copied();
            json!({
                "rank": rank + 1,
                "id": g.id,
                "kind": g.kind,
                "tree_classes": g.tree_classes,
                "depth": g.depth,
                "single": {"model": model_json(&g.single), "real": real_json(real.get(Target::Single(i)))},
                "greedy_step": step.map(|k| k + 1),
                "greedy_cumulative": step.map(|k| json!({
                    "model": {"classes": p.greedy[k].classes, "methods": p.greedy[k].methods},
                    "real": real_json(real.get(Target::Greedy(k))),
                })),
                "group": g.group,
                "category": g.category.key(),
                "category_label": g.category.label(),
                "evidence": g.evidence,
                "example": g.example,
                "chain": g.chain,
            })
        })
        .collect();
    let greedy: Vec<Value> = p
        .greedy
        .iter()
        .enumerate()
        .map(|(k, s)| {
            json!({"step": k + 1, "gate": p.gates[s.gate].id, "gain": s.gain, "entry": s.entry,
                "model": {"classes": s.classes, "methods": s.methods}, "real": real_json(real.get(Target::Greedy(k)))})
        })
        .collect();
    let groups: Vec<Value> = p
        .groups
        .iter()
        .enumerate()
        .map(|(gi, g)| {
            json!({"group": gi, "members": g.members.iter().map(|&m| p.gates[m].id.clone()).collect::<Vec<_>>(),
                "model": model_json(&g.model), "real": real_json(real.get(Target::Group(gi)))})
        })
        .collect();
    let verify: Vec<Value> = reqs
        .iter()
        .zip(runs)
        .map(|(r, run)| {
            let t = match r.target {
                Target::Single(i) => json!({"single": p.gates[i].id}),
                Target::Greedy(k) => json!({"greedy_prefix": k + 1}),
                Target::Group(g) => json!({"group": g}),
            };
            match run {
                Ok(x) => json!({"target": t, "cuts": r.cuts, "classes": x.classes.len(), "methods": x.methods, "elapsed_ms": x.elapsed_ms, "peak_mem_mb": x.peak_mem_mb}),
                Err(e) => json!({"target": t, "cuts": r.cuts, "error": e}),
            }
        })
        .collect();
    let cats: serde_json::Map<String, Value> = category_counts(&p.gates).into_iter().map(|(c, n)| (c.key().to_string(), json!(n))).collect();
    let b = &p.base;
    json!({
        "version": 1,
        "base": {"classes": b.classes, "methods": b.methods, "model_classes": b.model_classes, "model_methods": b.model_methods,
            "graph_nodes": b.nodes, "graph_edges": b.edges, "tree_candidates": b.tree_candidates, "pool": b.pool, "model_ms": b.model_ms as u64},
        "params": {"top": p.params.top, "pool": p.params.pool, "steps": p.params.steps, "min_gain": p.params.min_gain,
            "streak": p.params.streak, "extra": p.params.extra},
        "meta": meta,
        "categories": cats,
        "gates": gates,
        "greedy": greedy,
        "groups": groups,
        "verify": verify,
    })
}

fn real_cell(r: Option<&Result<Real, String>>) -> String {
    match r {
        None => "—".into(),
        Some(Err(_)) => "失败".into(),
        Some(Ok(r)) if r.added > 0 => format!("{}（反升 {}，非单调）", r.classes, r.added),
        Some(Ok(r)) => r.classes.to_string(),
    }
}

pub(super) fn markdown(p: &GatePlan, real: &RealIndex) -> String {
    let b = &p.base;
    let mut md = String::new();
    md.push_str(&format!(
        "# 门排名\n\n基线 {} 类 / {} 方法；触发图模型 {} 类 / {} 方法（{} 节点、{} 边）；首达树候选 {}，候选池（去重后）{}；模型计算 {} ms。\n\n",
        b.classes, b.methods, b.model_classes, b.model_methods, b.nodes, b.edges, b.tree_candidates, b.pool, b.model_ms
    ));
    md.push_str("Δ 为切除后减少的类数：「模型」为触发图上的「与」可达性（上近似，不含值流折叠），「实测」为 `--cut-file` 重跑。\n\n");
    md.push_str("| # | 门 | 形态 | 首达树类数 | 单切 Δ 模型 | 单切 Δ 实测 | Δ 方法（模型） | 贪心步 | 组 | 类别 | 主要包（模型） |\n|---:|---|---|---:|---:|---:|---:|---:|---:|---|---|\n");
    let step_of: HashMap<usize, usize> = p.greedy.iter().enumerate().map(|(k, s)| (s.gate, k)).collect();
    let ord = order(p, real);
    for (rank, &i) in ord.iter().enumerate() {
        let g = &p.gates[i];
        md.push_str(&format!(
            "| {} | `{}` | {} | {} | {} | {} | {} | {} | {} | {} | {} |\n",
            rank + 1,
            g.id,
            if g.kind == "site" { "调用点" } else { "方法体" },
            g.tree_classes,
            g.single.classes,
            real_cell(real.get(Target::Single(i))),
            g.single.methods,
            step_of.get(&i).map_or("—".into(), |k| (k + 1).to_string()),
            g.group.map_or("—".into(), |x| x.to_string()),
            g.category.label(),
            pk_text(&g.single.packages),
        ));
    }
    md.push_str("\n## 贪心组合累计\n\n逐步取边际收益最大的门；「入口」步单独切除无收益，与同段其余入口一起切除多连通团块。\n\n| 步 | 门 | 边际 Δ | 累计 Δ 类（模型） | 累计 Δ 方法（模型） | 累计 Δ 类（实测） |\n|---:|---|---:|---:|---:|---:|\n");
    for (k, s) in p.greedy.iter().enumerate() {
        md.push_str(&format!(
            "| {}{} | `{}` | {} | {} | {} | {} |\n",
            k + 1,
            if s.entry { "（入口）" } else { "" },
            p.gates[s.gate].id,
            s.gain,
            s.classes,
            s.methods,
            real_cell(real.get(Target::Greedy(k)))
        ));
    }
    if !p.groups.is_empty() {
        md.push_str("\n## 同机制组合切除（上界）\n\n| 组 | 成员 | Δ 类（模型） | Δ 类（实测） | 主要包 |\n|---:|---|---:|---:|---|\n");
        for (gi, g) in p.groups.iter().enumerate() {
            let names: Vec<String> = g.members.iter().map(|&m| format!("`{}`", p.gates[m].id)).collect();
            md.push_str(&format!("| {gi} | {} | {} | {} | {} |\n", names.join("<br>"), g.model.classes, real_cell(real.get(Target::Group(gi))), pk_text(&g.model.packages)));
        }
    }
    md.push_str("\n## 类别证据与示例首达链（前 10）\n");
    for &i in ord.iter().take(10) {
        let g = &p.gates[i];
        md.push_str(&format!("\n### `{}`\n\n- 类别：{}（{}）\n", g.id, g.category.label(), g.evidence.join("；")));
        if !g.example.is_empty() {
            md.push_str(&format!("- 示例下游类 `{}`：\n\n```\n{}\n```\n", g.example, g.chain.join("\n")));
        }
    }
    md
}

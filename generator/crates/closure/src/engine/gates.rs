//! 门自动排名（`rava closure --gates`）：找出让闭包膨胀的枢纽（门），量每个门与门组合各带入多少类，
//! 并标处理类别。取代闭包构成报告（docs/reports/2026-10-07-closure-composition.md）里的手工切除集。
//!
//! 流程（[`Engine::gate_plan`] 在引擎存活时完成全部模型计算，产出不借用引擎的 [`GatePlan`]）：
//! 1. 候选发现（`tree.rs`）：沿各类首达溯源链统计每个消费型节点（方法体、调用点）下游经过的类数，取前 `pool` 个；
//! 2. 反事实量测（`graph.rs` / `rank.rs`）：在基线运行登记的触发图上做「与」可达性——单切 Δ、
//!    贪心组合累计曲线（跨多连通团块）、同机制组合切除上界；
//! 3. 分类（`classify.rs`）：只用分析器事实与清单；
//! 4. 实测校准：调用方（driver）按 [`GatePlan::verify`] 以 `--cut-file` 子进程重跑，结果交 [`GatePlan::finish`]
//!    出 JSON 与人读排名表（`report.rs`）。

mod classify;
mod graph;
mod rank;
mod report;
mod tree;

use std::collections::{BTreeMap, HashMap, HashSet};

use crate::manifest::GateCategory;

use super::cut::Edges;
use super::Engine;
use graph::{split_site, TriggerGraph};
use rank::{GreedyParams, Model};
use tree::TreeKey;

pub use classify::{classify, Facts};

/// 门排名参数
#[derive(Debug, Clone)]
pub struct GateParams {
    /// 排名表行数
    pub top: usize,
    /// 候选池（按首达树经过类数取前若干；0 = max(4 × top, 40)）
    pub pool: usize,
    /// 贪心步数（0 = top）
    pub steps: usize,
    /// 贪心单步最少减少类数
    pub min_gain: usize,
    /// 多连通入口段最长步数
    pub streak: usize,
    /// 每步在候选池之外追加评估的模型首达树大子树节点数
    pub extra: usize,
}

impl Default for GateParams {
    fn default() -> Self {
        GateParams { top: 20, pool: 0, steps: 0, min_gain: 5, streak: 4, extra: 24 }
    }
}

/// 模型量测的减少量
#[derive(Debug, Clone, Default)]
pub struct ModelDelta {
    pub classes: usize,
    pub methods: usize,
    /// 消失类按自身包计数（降序，至多 6 项）
    pub packages: Vec<(String, usize)>,
}

#[derive(Debug, Clone)]
pub struct GateEntry {
    /// 切除条目（`--cut` / `--cut-file` 同形）
    pub id: String,
    /// `body`（方法体）/ `site`（调用点）
    pub kind: &'static str,
    /// 首达树上经过本门的类数与到根步数（贪心经模型首达树选入、不在引擎首达树上的门为 0）
    pub tree_classes: usize,
    pub depth: usize,
    /// 示例首达链（示例下游类 → 根）
    pub example: String,
    pub chain: Vec<String>,
    pub single: ModelDelta,
    pub category: GateCategory,
    pub evidence: Vec<String>,
    pub group: Option<usize>,
    /// 消失类节点（模型，分组用）
    removed: Vec<u32>,
}

#[derive(Debug, Clone)]
pub struct GreedyEntry {
    /// 门下标（`GatePlan::gates`）
    pub gate: usize,
    pub gain: usize,
    pub classes: usize,
    pub methods: usize,
    pub entry: bool,
}

#[derive(Debug, Clone)]
pub struct GroupEntry {
    pub members: Vec<usize>,
    pub model: ModelDelta,
}

/// 实测重跑的对象
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Target {
    Single(usize),
    /// 贪心前缀（含第 k 步）
    Greedy(usize),
    Group(usize),
}

#[derive(Debug, Clone)]
pub struct VerifyReq {
    pub target: Target,
    pub cuts: Vec<String>,
}

/// 实测重跑结果（子进程 `--gates-child` 输出）
#[derive(Debug, Clone, Default)]
pub struct RealRun {
    pub classes: Vec<String>,
    pub methods: usize,
    pub elapsed_ms: u64,
    pub peak_mem_mb: u64,
}

#[derive(Debug, Clone, Default)]
pub struct BaseInfo {
    pub classes: usize,
    pub methods: usize,
    pub model_classes: usize,
    pub model_methods: usize,
    pub nodes: usize,
    pub edges: usize,
    pub tree_candidates: usize,
    pub pool: usize,
    pub model_ms: u128,
}

pub struct GatePlan {
    pub base: BaseInfo,
    /// 基线类集（实测对照用）
    pub base_classes: Vec<String>,
    pub gates: Vec<GateEntry>,
    pub greedy: Vec<GreedyEntry>,
    pub groups: Vec<GroupEntry>,
    pub params: GateParams,
}

fn packages(g: &TriggerGraph, removed: &[u32]) -> Vec<(String, usize)> {
    let mut by: HashMap<&str, usize> = HashMap::new();
    for &n in removed {
        let c = &g.names[n as usize][2..];
        *by.entry(c.rsplit_once('/').map_or("", |p| p.0)).or_default() += 1;
    }
    let mut v: Vec<(String, usize)> = by.into_iter().map(|(k, n)| (k.to_string(), n)).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    v.truncate(6);
    v
}

fn model_delta(g: &TriggerGraph, d: &rank::Delta) -> ModelDelta {
    ModelDelta { classes: d.classes, methods: d.methods, packages: packages(g, &d.removed) }
}

impl Engine<'_> {
    /// 门排名的全部模型计算（`edges` 为同一次运行登记的触发边，方法源带偏移）
    pub fn gate_plan(&self, edges: Edges, params: GateParams) -> GatePlan {
        let t0 = std::time::Instant::now();
        let mut params = params;
        params.top = params.top.max(1);
        if params.pool == 0 {
            params.pool = (params.top * 4).max(40);
        }
        if params.steps == 0 {
            params.steps = params.top;
        }
        let edge_count = edges.edges.len();
        // 方法键 → 代表序号；计入方法数的是 `method_nodes` 口径（代表、非伪方法）
        let canon: HashMap<String, usize> = self.mbase.iter().map(|(k, &i)| (k.to_string(), i)).collect();
        let counted: HashSet<String> = self.method_nodes().map(|m| m.key.to_string()).collect();
        let g = TriggerGraph::build(edges, |k| counted.contains(k));
        let model = Model::new(&g);
        // 图节点 ↔ 候选键
        let mut node_key: HashMap<u32, TreeKey> = HashMap::new();
        let mut eligible = vec![false; g.len()];
        let mut elig_memo: HashMap<TreeKey, bool> = HashMap::new();
        for i in 0..g.len() {
            let Some(k) = g.names[i].strip_prefix("M:") else { continue };
            let key = match split_site(k) {
                Some((b, off)) => canon.get(b).map(|&m| (m, Some(off))),
                None => canon.get(k).map(|&m| (m, None)),
            };
            let Some(key) = key else { continue };
            node_key.insert(i as u32, key);
            eligible[i] = *elig_memo.entry(key).or_insert_with(|| self.gate_eligible(key));
        }
        let key_node: HashMap<TreeKey, u32> = node_key.iter().map(|(&n, &k)| (k, n)).collect();

        // 1. 候选：首达树经过类数前 pool 个
        let tree = self.gate_tree();
        let mut cands: Vec<(&TreeKey, &tree::TreeGate)> = tree.iter().filter(|(k, _)| key_node.contains_key(k)).collect();
        cands.sort_by(|a, b| b.1.classes.cmp(&a.1.classes).then(b.1.depth.cmp(&a.1.depth)).then(a.0.cmp(b.0)));
        cands.truncate(params.pool);

        // 2. 单切；消失集合相同的只留最深者（离团块最近）
        let mut singles: Vec<(TreeKey, u32, rank::Delta)> = cands.iter().map(|(k, _)| (**k, key_node[k], model.delta(&[key_node[k]]))).collect();
        singles.sort_by(|a, b| b.2.classes.cmp(&a.2.classes).then(tree[&b.0].depth.cmp(&tree[&a.0].depth)).then(a.0.cmp(&b.0)));
        let mut seen_sets: HashSet<Vec<u32>> = HashSet::new();
        singles.retain(|(_, _, d)| d.removed.is_empty() || seen_sets.insert(d.removed.clone()));
        let pool: Vec<u32> = singles.iter().map(|s| s.1).collect();

        // 3. 贪心组合
        let gp = GreedyParams { steps: params.steps, min_gain: params.min_gain, streak: params.streak, extra: params.extra };
        let steps = rank::greedy(&model, &pool, &eligible, &gp);

        // 排名表：单切前 top + 贪心选入的门
        let mut listed: Vec<(TreeKey, u32, rank::Delta)> = singles.into_iter().take(params.top).collect();
        for s in &steps {
            if !listed.iter().any(|l| l.1 == s.node) {
                if let Some(&k) = node_key.get(&s.node) {
                    listed.push((k, s.node, model.delta(&[s.node])));
                }
            }
        }
        let dispatch = self.dispatch_sites();
        let child_kinds = self.gate_child_kinds();
        let mut gates: Vec<GateEntry> = listed
            .iter()
            .map(|(k, _, d)| {
                let tg = tree.get(k).cloned().unwrap_or_default();
                let start = self.classes.get(&tg.example).map(|c| c.via.clone()).unwrap_or_else(|| self.methods[k.0].via.clone());
                let (chain, _) = self.gate_chain(&start, 14);
                let (category, evidence) = classify(&self.gate_facts(*k, &dispatch, &child_kinds));
                GateEntry {
                    id: self.gate_id(*k),
                    kind: if k.1.is_some() { "site" } else { "body" },
                    tree_classes: tg.classes,
                    depth: if tg.depth == usize::MAX { 0 } else { tg.depth },
                    example: tg.example,
                    chain,
                    single: model_delta(&g, d),
                    category,
                    evidence,
                    group: None,
                    removed: d.removed.clone(),
                }
            })
            .collect();
        let greedy = steps
            .iter()
            .filter_map(|s| {
                let gate = listed.iter().position(|l| l.1 == s.node)?;
                Some(GreedyEntry { gate, gain: s.gain, classes: s.classes, methods: s.methods, entry: s.entry })
            })
            .collect();

        // 4. 同机制组合：同包或消失集合重合
        let pkgs: Vec<String> = gates.iter().map(|x| x.id.split('.').next().unwrap_or("").rsplit_once('/').map_or(String::new(), |p| p.0.to_string())).collect();
        let removed: Vec<&[u32]> = gates.iter().map(|x| x.removed.as_slice()).collect();
        let mut groups: Vec<GroupEntry> = rank::groups(&pkgs, &removed)
            .into_iter()
            .map(|members| {
                let nodes: Vec<u32> = members.iter().map(|&i| listed[i].1).collect();
                GroupEntry { model: model_delta(&g, &model.delta(&nodes)), members }
            })
            .collect();
        groups.sort_by(|a, b| b.model.classes.cmp(&a.model.classes).then(a.members.cmp(&b.members)));
        for (gi, grp) in groups.iter().enumerate() {
            for &m in &grp.members {
                gates[m].group = Some(gi);
            }
        }

        let base = BaseInfo {
            classes: self.classes.len(),
            methods: self.method_count(),
            model_classes: model.base_count.0,
            model_methods: model.base_count.1,
            nodes: g.len(),
            edges: edge_count,
            tree_candidates: tree.len(),
            pool: pool.len(),
            model_ms: t0.elapsed().as_millis(),
        };
        let mut base_classes: Vec<String> = self.classes.keys().cloned().collect();
        base_classes.sort_unstable();
        GatePlan { base, base_classes, gates, greedy, groups, params }
    }
}

impl GatePlan {
    /// 实测重跑清单：贪心前缀（第 1、2、4、8… 步与末步）→ 组合 → 单切（按模型 Δ），截到 `budget` 条
    pub fn verify(&self, budget: usize) -> Vec<VerifyReq> {
        let mut out = Vec::new();
        let n = self.greedy.len();
        let mut marks: Vec<usize> = (0..).map(|k| (1usize << k) - 1).take_while(|&i| i < n).collect();
        if n > 0 && !marks.contains(&(n - 1)) {
            marks.push(n - 1);
        }
        // 多连通入口段内的前缀不单独测（模型下零收益），取段末
        for k in marks.into_iter().filter(|&k| !self.greedy[k].entry) {
            out.push(VerifyReq { target: Target::Greedy(k), cuts: self.greedy[..=k].iter().map(|s| self.gates[s.gate].id.clone()).collect() });
        }
        for (gi, g) in self.groups.iter().enumerate().take(3) {
            out.push(VerifyReq { target: Target::Group(gi), cuts: g.members.iter().map(|&m| self.gates[m].id.clone()).collect() });
        }
        let mut order: Vec<usize> = (0..self.gates.len()).collect();
        order.sort_by(|&a, &b| self.gates[b].single.classes.cmp(&self.gates[a].single.classes).then(a.cmp(&b)));
        for i in order {
            out.push(VerifyReq { target: Target::Single(i), cuts: vec![self.gates[i].id.clone()] });
        }
        // 预算优先给单切前几名：贪心 / 组合至多占一半
        let half = budget / 2;
        let (mut multi, single): (Vec<_>, Vec<_>) = out.into_iter().partition(|r| !matches!(r.target, Target::Single(_)));
        multi.truncate(half);
        let rest = budget.saturating_sub(multi.len());
        multi.extend(single.into_iter().take(rest));
        multi
    }

    /// 合并实测结果，出 (JSON, Markdown 排名表)
    pub fn finish(&self, reqs: &[VerifyReq], runs: &[Result<RealRun, String>], meta: serde_json::Value) -> (serde_json::Value, String) {
        let real = report::RealIndex::new(self, reqs, runs);
        (report::json(self, &real, reqs, runs, meta), report::markdown(self, &real))
    }
}

/// 门类别按序统计（报告用）
pub fn category_counts(gates: &[GateEntry]) -> BTreeMap<GateCategory, usize> {
    let mut m = BTreeMap::new();
    for g in gates {
        *m.entry(g.category).or_default() += 1;
    }
    m
}


//! 引擎：性能观测（计划 2026-09-30-closure-analyzer-performance.md P0）——分阶段自耗时、
//! 抽象解释次数与失效原因、峰值内存。只进 `summary.perf`，不影响其余输出。

use std::time::{Duration, Instant};

use super::*;

/// 计时类别（自耗时：进入内层类别时外层暂停）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Phase {
    /// 根 / 种子登记（run 之前）
    Setup,
    /// 流边差分传播
    Flows,
    /// 反射成员枚举、反射调用实参池派发
    Enumerate,
    /// 方法处理（执行事件、手写体），不含抽象解释
    Process,
    /// 方法体抽象解释（`analysis`）
    Analyze,
    /// 辅助抽象解释（`<clinit>` 常量、构造器摘要、转发 / 属性判定等）
    AuxAnalyze,
    /// 读者站点重跑
    Sites,
    /// lambda 调用重跑
    Lcalls,
    /// 清单补种轮
    Seeds,
}

const PHASES: [(Phase, &str); 9] = [
    (Phase::Setup, "setup"),
    (Phase::Flows, "flows"),
    (Phase::Enumerate, "enumerate"),
    (Phase::Process, "process"),
    (Phase::Analyze, "analyze"),
    (Phase::AuxAnalyze, "aux_analyze"),
    (Phase::Sites, "sites"),
    (Phase::Lcalls, "lcalls"),
    (Phase::Seeds, "seeds"),
];

/// 方法重分析的原因（首次分析 = First）
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Why {
    First,
    /// 字段值集变化（`field_put`）
    FieldPut,
    /// 字段放开（`open_field`）
    FieldOpen,
    /// 同名字段放开（`open_field_name`）
    FieldOpenName,
    /// 全部字段放开（反射枚举 / 反序列化）
    FieldsAll,
    /// 系统属性不折叠集合增长
    Sysprops,
    /// 被调方返回常量变化
    RetConst,
    /// 形参常量变化
    ParamConst,
    /// 乐观阶段收尾（「尚无返回」的答复作废）
    Never,
    /// catch 类型变为存活
    Catch,
    /// Class 形参值集增长（引用比较的镜像答复作废）
    Mirror,
}

const WHYS: [(Why, &str); 11] = [
    (Why::First, "first"),
    (Why::FieldPut, "field_put"),
    (Why::FieldOpen, "field_open"),
    (Why::FieldOpenName, "field_open_name"),
    (Why::FieldsAll, "fields_all"),
    (Why::Sysprops, "sysprops"),
    (Why::RetConst, "ret_const"),
    (Why::ParamConst, "param_const"),
    (Why::Never, "never"),
    (Why::Catch, "catch"),
    (Why::Mirror, "mirror"),
];

pub(super) struct Stats {
    cur: Phase,
    since: Instant,
    stack: Vec<Phase>,
    acc: [Duration; PHASES.len()],
    /// 方法体抽象解释次数（按方法节点）
    pub(super) per_method: Vec<u32>,
    /// 方法节点待重分析的原因（首个失效原因）
    pending: HashMap<usize, Why>,
    /// 按原因：失效请求 / 实际导致重分析 / 重分析结果与已执行的事件完全相同
    requested: [u64; WHYS.len()],
    analyses: [u64; WHYS.len()],
    unchanged: [u64; WHYS.len()],
    /// 被调方透传摘要变化引起的调用方整方法重接
    pub(super) reapply: u64,
    /// 同一分析结果的整方法重处理（open 展开的 G 增长等）
    pub(super) reprocess: u64,
    /// 按入口状态复用共享摘要（免分析）的次数
    pub(super) shared: u64,
    pub(super) site_reruns: u64,
    pub(super) lcall_reruns: u64,
    pub(super) aux_analyses: u64,
    /// 常量实参求值：记忆命中 / 未命中 / 未命中中实际分析（consteval.rs）
    pub(super) ceval: [u64; 3],
    /// 各阶段结束时的峰值 RSS（MB）
    rss_marks: Vec<(&'static str, u64)>,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            cur: Phase::Setup,
            since: Instant::now(),
            stack: Vec::new(),
            acc: Default::default(),
            per_method: Vec::new(),
            pending: HashMap::default(),
            requested: Default::default(),
            analyses: Default::default(),
            unchanged: Default::default(),
            reapply: 0,
            reprocess: 0,
            shared: 0,
            site_reruns: 0,
            lcall_reruns: 0,
            aux_analyses: 0,
            ceval: [0; 3],
            rss_marks: Vec::new(),
        }
    }
}

fn phase_index(p: Phase) -> usize {
    PHASES.iter().position(|x| x.0 == p).unwrap_or(0)
}

fn why_index(w: Why) -> usize {
    WHYS.iter().position(|x| x.0 == w).unwrap_or(0)
}

/// 进程峰值内存占用（MB），内存预算与 A/B 对照的主指标。
/// macOS 取物理占用峰值（`proc_pid_rusage` 的 `ri_lifetime_max_phys_footprint`：含被压缩 / 换出的脏页，
/// 与 `/usr/bin/time -l` 的 peak memory footprint 同口径）；驻留集 RSS 在系统内存压力下会被压缩器收走，
/// 同一二进制多次运行可差 30% 以上（DeepCopy 实测 1621–2090 MB，占用恒为约 2.15 GB），不作预算依据。
/// 其它平台取峰值 RSS。
pub fn peak_mem_mb() -> u64 {
    #[cfg(target_os = "macos")]
    {
        extern "C" {
            fn proc_pid_rusage(pid: i32, flavor: i32, buffer: *mut u64) -> i32;
            fn getpid() -> i32;
        }
        // rusage_info_v4：16 字节 uuid 后为 u64 字段序列，ri_lifetime_max_phys_footprint 是第 28 个（0 起）
        const RUSAGE_INFO_V4: i32 = 4;
        const LIFETIME_MAX_PHYS_FOOTPRINT: usize = 2 + 28;
        let mut buf = [0u64; 64];
        // SAFETY：缓冲区 512 字节，大于 rusage_info_v4（304 字节）
        if unsafe { proc_pid_rusage(getpid(), RUSAGE_INFO_V4, buf.as_mut_ptr()) } == 0 {
            return buf[LIFETIME_MAX_PHYS_FOOTPRINT] >> 20;
        }
    }
    peak_rss_mb()
}

/// 进程峰值 RSS（MB）：getrusage；macOS 以字节计，Linux 以 KB 计
pub fn peak_rss_mb() -> u64 {
    #[repr(C)]
    struct Rusage {
        utime: [i64; 2],
        stime: [i64; 2],
        maxrss: i64,
        rest: [i64; 13],
    }
    extern "C" {
        fn getrusage(who: i32, usage: *mut Rusage) -> i32;
    }
    let mut r = Rusage { utime: [0; 2], stime: [0; 2], maxrss: 0, rest: [0; 13] };
    // SAFETY：RUSAGE_SELF = 0；结构体布局与 64 位 macOS / Linux 的 struct rusage 一致
    if unsafe { getrusage(0, &mut r) } != 0 {
        return 0;
    }
    let bytes = if cfg!(target_os = "macos") { r.maxrss as u64 } else { r.maxrss as u64 * 1024 };
    bytes >> 20
}

impl Stats {
    pub(super) fn enter(&mut self, p: Phase) {
        let now = Instant::now();
        self.acc[phase_index(self.cur)] += now - self.since;
        self.stack.push(self.cur);
        self.cur = p;
        self.since = now;
    }

    pub(super) fn leave(&mut self) {
        let now = Instant::now();
        self.acc[phase_index(self.cur)] += now - self.since;
        self.cur = self.stack.pop().unwrap_or(Phase::Setup);
        self.since = now;
    }

    pub(super) fn mark_rss(&mut self, at: &'static str) {
        self.rss_marks.push((at, peak_mem_mb()));
    }

    /// 失效请求；`effective` = 该方法确有分析结果被丢弃（随后必重分析）
    pub(super) fn invalidated(&mut self, m: usize, why: Why, effective: bool) {
        self.requested[why_index(why)] += 1;
        if effective {
            self.pending.entry(m).or_insert(why);
        }
    }

    /// 一次方法体抽象解释完成；`unchanged` = 事件与已执行的分析完全相同
    pub(super) fn analyzed(&mut self, m: usize, unchanged: bool) {
        if self.per_method.len() <= m {
            self.per_method.resize(m + 1, 0);
        }
        let first = self.per_method[m] == 0;
        self.per_method[m] += 1;
        let why = self.pending.remove(&m).unwrap_or(if first { Why::First } else { Why::Catch });
        self.analyses[why_index(why)] += 1;
        if unchanged {
            self.unchanged[why_index(why)] += 1;
        }
    }
}

impl<'a> Engine<'a> {
    pub(super) fn stat_enter(&self, p: Phase) {
        self.ctx.stats.borrow_mut().enter(p);
    }

    pub(super) fn stat_leave(&self) {
        self.ctx.stats.borrow_mut().leave();
    }

    /// `summary.perf`：分阶段自耗时、峰值内存、重分析分布与失效原因
    pub fn perf_json(&self, top: usize) -> serde_json::Value {
        use serde_json::json;
        let s = self.ctx.stats.borrow();
        let ms = |d: Duration| d.as_millis() as u64;
        let phases: serde_json::Map<String, serde_json::Value> =
            PHASES.iter().map(|(p, n)| (n.to_string(), json!(ms(s.acc[phase_index(*p)])))).collect();
        let reasons: serde_json::Map<String, serde_json::Value> = WHYS
            .iter()
            .enumerate()
            .filter(|(i, _)| s.requested[*i] + s.analyses[*i] > 0)
            .map(|(i, (_, n))| {
                (n.to_string(), json!({"requested": s.requested[i], "analyses": s.analyses[i], "unchanged": s.unchanged[i]}))
            })
            .collect();
        let total: u64 = s.per_method.iter().map(|&c| u64::from(c)).sum();
        let mut by_ctx: Vec<(usize, u32)> = s.per_method.iter().copied().enumerate().filter(|x| x.1 > 1).collect();
        by_ctx.sort_by_key(|&(m, c)| (std::cmp::Reverse(c), m));
        let mut by_member: BTreeMap<String, (u32, u32)> = BTreeMap::new();
        for (m, &c) in s.per_method.iter().enumerate() {
            if c > 0 {
                let e = by_member.entry(self.method_label(m)).or_default();
                e.0 += c;
                e.1 += 1;
            }
        }
        let mut members: Vec<(String, (u32, u32))> = by_member.into_iter().collect();
        members.sort_by(|a, b| b.1 .0.cmp(&a.1 .0).then_with(|| a.0.cmp(&b.0)));
        json!({
            "phases_ms": phases,
            "peak_rss_mb": peak_rss_mb(),
            "peak_mem_mb": peak_mem_mb(),
            "mem_marks_mb": s.rss_marks.iter().map(|(a, b)| json!([a, b])).collect::<Vec<_>>(),
            "analyses": total,
            "analyzed_contexts": s.per_method.iter().filter(|&&c| c > 0).count(),
            "aux_analyses": s.aux_analyses,
            "ceval_memo": s.ceval,
            "reasons": reasons,
            "reapply_callee_summary": s.reapply,
            "reprocess_same_analysis": s.reprocess,
            "shared_analyses": s.shared,
            "site_reruns": s.site_reruns,
            "lcall_reruns": s.lcall_reruns,
            "flow_edges": self.graph.edge_count,
            "adds": self.graph.adds,
            // 环合并：检测次数 / 合并掉的节点数 / 检测耗时 ms（scc.rs）
            "scc": self.graph.scc_stats,
            // 新接边收窄记忆：命中 / 未命中（flow.rs）
            "fmemo": self.graph.fmemo_stats,
            // 类型集驻留：写入 / 写后共享已有内容 / 表清理次数，及不同内容份数（setstore.rs）
            "set_intern": [self.graph.sets.stats[0], self.graph.sets.stats[1], self.graph.sets.stats[2], self.graph.sets.unique() as u64],
            // 传播推送按边种类：次数 / 有增量次数（前 20）
            "pushes_by_kind": self.push_kinds(),
            // 枢纽数 / 调用点接入枢纽总数 / 单调用点最多接入数（hub.rs）
            "hubs": [self.hubs.len(), self.hub_sites.values().map(|h| h.len()).sum::<usize>(), self.hub_sites.values().map(|h| h.len()).max().unwrap_or(0)],
            // 枢纽重放去重记录的规模：(调用点, lambda) 条数（hub.rs；按调用点建模目标按祖先枢纽判定，不留记录）
            "hub_sent": self.hub_lsent.values().map(|d| d.len()).sum::<usize>(),
            "edges_by_kind": self.edge_kinds(),
            "top_out_degree": self.top_degree(top, false),
            "top_in_degree": self.top_degree(top, true),
            "type_nodes": self.graph.len(),
            "top_contexts": by_ctx.iter().take(top).map(|&(m, c)| json!([self.ctx_label(m), c])).collect::<Vec<_>>(),
            "top_members": members.iter().take(top).map(|(k, (c, n))| json!([k, c, n])).collect::<Vec<_>>(),
        })
    }
}

impl Ctx<'_> {
    /// 辅助抽象解释（不登记为方法体分析），计入 `aux_analyze`
    pub(super) fn aux_analyze(&self, owner: &str, desc: &str, is_static: bool, code: &classfile::Code, facts: &Facts) -> Analysis {
        self.stats.borrow_mut().enter(Phase::AuxAnalyze);
        let a = absint::analyze(owner, desc, is_static, code, facts);
        let mut s = self.stats.borrow_mut();
        s.aux_analyses += 1;
        s.leave();
        a
    }
}

/// 节点种类数与序号（推送计数用；与 `node_kind` 同序）
pub(super) const KINDS: usize = 17;
const KIND_NAMES: [&str; KINDS] = ["P", "R", "Spool", "Scatch", "S", "F", "U", "O", "E", "Array", "A", "W", "HP", "HR", "Esc", "G", "Rcall"];

#[inline]
pub(super) fn kind_ix(n: &Node) -> usize {
    match n {
        Node::P(..) => 0,
        Node::R(..) => 1,
        Node::S(_, o) if *o == POOL || *o == PROD || *o == ARRAY_RET => 2,
        Node::S(_, o) if o & CATCH != 0 => 3,
        Node::S(..) => 4,
        Node::F(..) => 5,
        Node::U(..) => 6,
        Node::O(..) => 7,
        Node::E(..) => 8,
        Node::Array => 9,
        Node::A(..) => 10,
        Node::W(..) => 11,
        Node::HP(..) => 12,
        Node::HR(..) => 13,
        Node::Esc => 14,
        Node::G(..) => 15,
        Node::RP(_) | Node::RN(_) | Node::RA(_) => 16,
    }
}

/// 节点类别名（诊断）
fn node_kind(n: &Node) -> &'static str {
    KIND_NAMES[kind_ix(n)]
}

impl<'a> Engine<'a> {
    /// 传播推送按（源种类 → 目标种类）计数：次数 / 有增量次数，按次数降序前 20
    fn push_kinds(&self) -> serde_json::Value {
        let mut v: Vec<(String, [u64; 2])> = self
            .graph
            .pushes
            .iter()
            .enumerate()
            .filter(|(_, p)| p[0] > 0)
            .map(|(k, p)| (format!("{}->{}", KIND_NAMES[k / KINDS], KIND_NAMES[k % KINDS]), *p))
            .collect();
        v.sort_by(|a, b| b.1[0].cmp(&a.1[0]).then_with(|| a.0.cmp(&b.0)));
        serde_json::json!(v.into_iter().take(20).collect::<Vec<_>>())
    }

    /// 流边按（源类别 → 目标类别）计数
    fn edge_kinds(&self) -> serde_json::Value {
        // 先按种类序号对计数，最后才拼键（避免逐边 format!）
        let mut m = [0u64; KINDS * KINDS];
        for (s, es) in self.graph.edges.iter().enumerate() {
            if es.is_empty() {
                continue;
            }
            let sk = kind_ix(self.graph.node_at(s as u32)) * KINDS;
            for &(d, _) in es {
                m[sk + kind_ix(self.graph.node_at(d))] += 1;
            }
        }
        let mut v: Vec<(String, u64)> = m
            .iter()
            .enumerate()
            .filter(|x| *x.1 > 0)
            .map(|(k, &c)| (format!("{}->{}", KIND_NAMES[k / KINDS], KIND_NAMES[k % KINDS]), c))
            .collect();
        v.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
        serde_json::json!(v.into_iter().take(30).collect::<Vec<_>>())
    }

    fn node_label(&self, n: &Node) -> String {
        match *n {
            Node::P(m, i) => format!("P{i} {}", self.ctx_label(m)),
            Node::R(m) => format!("R {}", self.ctx_label(m)),
            Node::S(m, o) => format!("S{o} {}", self.ctx_label(m)),
            Node::F(f) | Node::U(f) => format!("{} {}", node_kind(n), self.fields.get_index(f).map(|x| x.0.to_string()).unwrap_or_default()),
            Node::O(o, f) => format!("O {} {}", self.names[o as usize], self.fields.get_index(f).map(|x| x.0.to_string()).unwrap_or_default()),
            Node::E(o, p) => format!("E{p} {}", self.names[o as usize]),
            other => format!("{other:?}"),
        }
    }

    /// 出度 / 入度最大的节点
    fn top_degree(&self, top: usize, incoming: bool) -> serde_json::Value {
        // 节点驻留唯一，按序号计度数
        let es = &self.graph.edges;
        let mut deg = vec![0u64; es.len()];
        for (s, out) in es.iter().enumerate() {
            if incoming {
                for &(d, _) in out {
                    deg[d as usize] += 1;
                }
            } else {
                deg[s] += out.len() as u64;
            }
        }
        let mut v: Vec<(&Node, u64)> =
            deg.iter().enumerate().filter(|x| *x.1 > 0).map(|(i, &d)| (self.graph.node_at(i as u32), d)).collect();
        // 节点唯一，排序键全序：先选出前 top 再排，结果同全排
        let cmp = |a: &(&Node, u64), b: &(&Node, u64)| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0));
        if v.len() > top && top > 0 {
            v.select_nth_unstable_by(top - 1, cmp);
            v.truncate(top);
        }
        v.sort_by(cmp);
        serde_json::json!(v.into_iter().take(top).map(|(n, d)| serde_json::json!([self.node_label(n), d])).collect::<Vec<_>>())
    }
}

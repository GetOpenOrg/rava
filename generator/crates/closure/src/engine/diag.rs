//! 引擎：类型流诊断中需要传播期信息的两类查询。
//!
//! **open 来源**（`--flows @openorig:[=]<类型>|<节点子串>` / `@openinj:<类型>`）：open 值只有两种来历：
//! 直接注入（手写返回、未知值按声明类型、open 数组的元素读……）与沿流边传播。注入点在 `add_to` 处登记
//! （[`Engine::open_inj`]），诊断沿流边反向走到注入点，回答「这个节点上的 open(T) 从哪些注入点来」。
//!
//! **记录型查询**（分析前经 `Diag::flows` 登记，传播中逐条记录，分析后按原查询文本取回）：
//! - `@grow:<节点子串>`：匹配节点（含并入同一代表的成员）每次增长——序号、来源、新增部分；
//! - `@trace:<类或分配名>` / `@trace:open:<类型>`：每个获得该值的节点及其来源，按到达先后（定位「值从哪进来」）；
//! - `@edge:<节点子串>`：以匹配节点为目标新建的流边及建边时源集合。
//!
//! 来源为流边源节点，或直接注入时的当前字节码站点。每条记录同时实时写 stderr（`[flows <查询>] …`），
//! 分析未结束（超时被杀）时仍可读到。未登记任何记录型查询时引擎不持有 [`Probes`]，传播热路径只多判一次空指针。

use super::*;

/// `flow_src` 的空值：下一次 `add_to` 是直接注入而非流边推送
pub(super) const NO_SRC: u32 = u32::MAX;

enum ProbeKind {
    Grow(String),
    Edge(String),
    /// 名字、是否 open、类序号（类首次出现时解析）
    Trace(String, bool, Option<u32>),
}

struct Probe {
    query: String,
    kind: ProbeKind,
    log: Vec<String>,
}

/// 记录型 `--flows` 查询的登记与记录
#[derive(Default)]
pub(super) struct Probes {
    list: Vec<Probe>,
    /// 节点序号 → 匹配的节点类查询（`list` 下标）；节点标签只格式化一次
    node_hits: HashMap<u32, Rc<[usize]>>,
    seq: u64,
}

impl Probes {
    /// 从 `--flows` 查询全文里取出记录型查询；其余查询忽略（分析后求值）
    pub(super) fn parse(queries: &[String]) -> Probes {
        let mut p = Probes::default();
        for q in queries {
            let kind = if let Some(n) = q.strip_prefix("@grow:") {
                ProbeKind::Grow(n.to_string())
            } else if let Some(n) = q.strip_prefix("@edge:") {
                ProbeKind::Edge(n.to_string())
            } else if let Some(c) = q.strip_prefix("@trace:") {
                match c.strip_prefix("open:") {
                    Some(c) => ProbeKind::Trace(c.to_string(), true, None),
                    None => ProbeKind::Trace(c.to_string(), false, None),
                }
            } else {
                continue;
            };
            if p.list.iter().any(|x| &x.query == q) {
                continue;
            }
            p.list.push(Probe { query: q.clone(), kind, log: Vec::new() });
        }
        p
    }

    fn record(&mut self, k: usize, line: String) {
        self.seq += 1;
        let line = format!("#{} {line}", self.seq);
        eprintln!("[flows {}] {line}", self.list[k].query);
        self.list[k].log.push(line);
    }
}

impl Engine<'_> {
    /// 分析前登记记录型 `--flows` 查询
    pub(crate) fn arm_probes(&mut self, queries: &[String]) {
        let p = Probes::parse(queries);
        self.probes = (!p.list.is_empty()).then(|| Box::new(p));
    }

    /// 节点 i 匹配的节点类查询（记忆）
    fn probe_hits(&self, ps: &mut Probes, i: u32) -> Rc<[usize]> {
        if let Some(h) = ps.node_hits.get(&i) {
            return h.clone();
        }
        let ns = self.node_str(self.graph.node(i));
        let h: Rc<[usize]> = ps
            .list
            .iter()
            .enumerate()
            .filter(|(_, p)| matches!(&p.kind, ProbeKind::Grow(n) | ProbeKind::Edge(n) if ns.contains(n.as_str())))
            .map(|(k, _)| k)
            .collect();
        ps.node_hits.insert(i, h.clone());
        h
    }

    /// 增长来源：流边源节点，或直接注入时的当前站点
    fn probe_src(&self, src: u32) -> String {
        if src != NO_SRC {
            return self.node_str(self.graph.node(src));
        }
        match (self.cur_call, self.cur_site) {
            (Some(c), _) => format!("直接 lambda 调用 #{c}"),
            (None, Some((m, off))) => format!("直接 {}@{off}", self.ctx_label(m)),
            (None, None) => "直接（站点外）".into(),
        }
    }

    /// 节点 i（代表 r）新增 delta，来源 src
    #[cold]
    pub(super) fn probe_grown(&mut self, i: u32, r: u32, src: u32, delta: &TypeSet) {
        let Some(mut ps) = self.probes.take() else { return };
        let members = self.graph.members.get(&r).cloned().unwrap_or_else(|| vec![r]);
        for k in 0..ps.list.len() {
            match &ps.list[k].kind {
                ProbeKind::Grow(_) => {
                    for &m in &members {
                        if self.probe_hits(&mut ps, m).contains(&k) {
                            let line = format!("{} ← {} +{{{}}}", self.node_str(self.graph.node(m)), self.probe_src(src), self.set_str(delta));
                            ps.record(k, line);
                        }
                    }
                }
                ProbeKind::Trace(name, open, id) => {
                    let (open, id) = match id {
                        Some(c) => (*open, *c),
                        None => {
                            let Some(&c) = self.ids.get(name.as_str()) else { continue };
                            let open = *open;
                            if let ProbeKind::Trace(_, _, id) = &mut ps.list[k].kind {
                                *id = Some(c);
                            }
                            (open, c)
                        }
                    };
                    let hit = if open { delta.open.contains(&id) } else { delta.classes.contains(&id) };
                    if hit {
                        let mut line = format!("{} ← {}", self.node_str(self.graph.node(i)), self.probe_src(src));
                        if members.len() > 1 {
                            line.push_str(&format!("（同代表 {} 个节点）", members.len()));
                        }
                        ps.record(k, line);
                    }
                }
                ProbeKind::Edge(_) => {}
            }
        }
        self.probes = Some(ps);
    }

    /// 新建流边 src → dst（过滤 filter）
    #[cold]
    pub(super) fn probe_edge(&mut self, src: u32, dst: u32, filter: u32) {
        let Some(mut ps) = self.probes.take() else { return };
        for &k in self.probe_hits(&mut ps, dst).iter() {
            if matches!(ps.list[k].kind, ProbeKind::Edge(_)) {
                let s = self.graph.set(self.graph.rep(src)).clone();
                let line = format!(
                    "{} → {} [{}] {{{}}}",
                    self.node_str(self.graph.node(src)),
                    self.node_str(self.graph.node(dst)),
                    self.filter_label(filter),
                    self.set_str(&s)
                );
                ps.record(k, line);
            }
        }
        self.probes = Some(ps);
    }

    /// 记录型查询的结果：未登记时提示须在分析前给出
    fn probe_report(&self, pat: &str) -> Option<Vec<String>> {
        if !["@grow:", "@trace:", "@edge:"].iter().any(|p| pat.starts_with(p)) {
            return None;
        }
        let Some(p) = self.probes.as_deref().and_then(|ps| ps.list.iter().find(|p| p.query == pat)) else {
            return Some(vec![format!("  {pat}：记录型查询须在分析前登记（rava closure --flows 自动登记）")]);
        };
        let mut out = vec![format!("  {pat}：{} 条", p.log.len())];
        out.extend(p.log.iter().map(|l| format!("  {l}")));
        Some(out)
    }

    /// 接收者恒为 null 的活虚调用点（folds `null_recv`）：成员@偏移 → 被调成员
    fn null_recv_sites(&self) -> Vec<String> {
        let mut clones: BTreeMap<&MemberRef, Vec<usize>> = BTreeMap::new();
        for (i, mn) in self.methods.values().enumerate() {
            if mn.kind == Kind::Bytecode {
                clones.entry(&mn.key).or_default().push(i);
            }
        }
        let mut out: Vec<String> = Vec::new();
        let um = self.unmodeled();
        for (k, cs) in &clones {
            for pc in self.null_recv(cs, &um) {
                let callee = cs.iter().filter_map(|&i| self.methods[i].analysis.as_ref()).find_map(|a| {
                    a.events.iter().find_map(|(p, e)| match e {
                        Event::Invoke { mref, .. } if *p == pc => Some(mref.to_string()),
                        _ => None,
                    })
                });
                out.push(format!("  {k}@{pc} → {}", callee.unwrap_or_default()));
            }
        }
        out.insert(0, format!("  {} 个接收者恒为 null 的活虚调用点", out.len()));
        out
    }

    pub(super) fn diag_open(&self, pat: &str) -> Option<Vec<String>> {
        if let Some(r) = self.probe_report(pat) {
            return Some(r);
        }
        if let Some(q) = pat.strip_prefix("@xpath:") {
            return Some(self.xpath_report(q));
        }
        if pat == "@keyed" {
            return Some(self.keyed_report());
        }
        if pat == "@nullrecv" {
            return Some(self.null_recv_sites());
        }
        if pat == "@bynamesites" {
            return Some(self.byname_sites());
        }
        if pat == "@concrete" {
            return Some(self.concrete.diag.iter().flat_map(|(k, vs)| vs.iter().map(move |v| format!("  {k}：{v}"))).collect());
        }
        if pat == "@foldfields" {
            return Some(self.fold_fields());
        }
        if let Some(q) = pat.strip_prefix("@openinj:") {
            let Some(&cid) = self.ids.get(q) else { return Some(vec![format!("无此类：{q}")]) };
            let mut v: Vec<String> =
                self.open_inj.iter().filter(|(_, ts)| ts.contains(&cid)).map(|(n, _)| format!("  {}", self.node_str(*n))).collect();
            v.sort();
            return Some(v);
        }
        let (q, np) = pat.strip_prefix("@openorig:")?.split_once('|')?;
        // `=类型`：只认该类型本身的 open（不含经声明类型收窄而来的子 / 超类型 open）
        let (exact, q) = match q.strip_prefix('=') {
            Some(q) => (true, q),
            None => (false, q),
        };
        let Some(&cid) = self.ids.get(q) else { return Some(vec![format!("无此类：{q}")]) };
        // 流边按声明类型收窄：open(Object) 经 Object[] 过滤成为 open(Object[])，沿途的 open 类型与目标有子类型关系即算同源
        let q_name = self.names[cid as usize].clone();
        let related = |o: u32| {
            let n = &self.names[o as usize];
            o == cid || !exact && (self.h.is_subtype(n, &q_name) || self.h.is_subtype(&q_name, n))
        };
        let has = |x: &Node| self.graph.get(x).is_some_and(|s| s.open.iter().any(related));
        let mut rev: HashMap<Node, Vec<Node>> = HashMap::default();
        for (src, edges) in &self.graph.flow_list() {
            if has(src) {
                for (dst, _) in edges {
                    rev.entry(*dst).or_default().push(*src);
                }
            }
        }
        let mut seen: HashSet<Node> = self.graph.keys().filter(|n| has(n) && self.node_str(**n).contains(np)).copied().collect();
        let mut q: VecDeque<Node> = seen.iter().copied().collect();
        // 注入点 → 到起点的最短距离
        let mut hits: Vec<(Node, usize)> = Vec::new();
        let mut dist: HashMap<Node, usize> = seen.iter().map(|n| (*n, 0)).collect();
        let mut next: HashMap<Node, Node> = HashMap::default();
        while let Some(n) = q.pop_front() {
            let d = dist[&n];
            if self.open_inj.get(&n).is_some_and(|ts| ts.iter().any(|&o| related(o))) {
                hits.push((n, d));
            }
            for p in rev.get(&n).into_iter().flatten() {
                if seen.insert(*p) {
                    dist.insert(*p, d + 1);
                    next.insert(*p, n);
                    q.push_back(*p);
                }
            }
        }
        // 每个注入点附到起点的一条最短路径（注入点 → … → 起点）
        let mut out: Vec<String> = hits
            .iter()
            .map(|(n, d)| {
                let mut path = vec![];
                let mut cur = *n;
                while let Some(&x) = next.get(&cur) {
                    path.push(self.node_str(x));
                    cur = x;
                }
                format!("  [{d:3}] {}\n        → {}", self.node_str(*n), path.join("\n        → "))
            })
            .collect();
        out.sort();
        out.insert(0, format!("  经 {} 个节点，注入点 {} 个", seen.len(), hits.len()));
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::{ProbeKind, Probes};

    fn q(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_keeps_recording_queries_only() {
        let p = Probes::parse(&q(&["@path:x|y", "@grow:P0 a/B.m", "@trace:a/B", "@trace:open:a/C", "@edge:R a/B.m", "@grow:P0 a/B.m", "a/B.m"]));
        assert!(!p.list.is_empty());
        let got: Vec<String> = p
            .list
            .iter()
            .map(|x| match &x.kind {
                ProbeKind::Grow(n) => format!("grow {n}"),
                ProbeKind::Edge(n) => format!("edge {n}"),
                ProbeKind::Trace(n, open, id) => format!("trace {n} {open} {id:?}"),
            })
            .collect();
        assert_eq!(got, ["grow P0 a/B.m", "trace a/B false None", "trace a/C true None", "edge R a/B.m"]);
    }

    #[test]
    fn parse_off_without_recording_queries() {
        let p = Probes::parse(&q(&["@array", "elem:x", "@openinj:a/B"]));
        assert!(p.list.is_empty());
    }

    #[test]
    fn record_numbers_lines() {
        let mut p = Probes::parse(&q(&["@grow:a", "@trace:b"]));
        p.record(1, "x".into());
        p.record(0, "y".into());
        assert_eq!(p.list[1].log, ["#1 x"]);
        assert_eq!(p.list[0].log, ["#2 y"]);
    }
}

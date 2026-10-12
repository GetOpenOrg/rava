//! 引擎：常量载体与排空点等价类合并（性能计划 §4.14 ①「常量保持的合并」的实施）。
//!
//! §4.13/§4.14 的结论：内存主体是逐克隆节点的类型集（census `graph.sets`），可行的合并必须**连集合一起合并、
//! 且合并不改变不动点**。等价类合并只在「类型流 + 常量」都一致时发生：
//!
//! - **常量载体**（`bind_params` → `join_pvs` 通道）：调用点（方法, 偏移, 形参槽）上的常量实参 intern 成内容序号，
//!   各被调形参累积「到达过的载体内容集」（只增）。两个上下文克隆常量恒同 ⟺ 常量格（`pvals`）与载体内容集都相同；
//!   只看汇合后的 `pvals` 不够——同一汇合值可来自不同的调用边结构，日后某条边变宽时两者分化。
//! - **排空点等价类合并**：主循环工作队列排空（`seed_round` 前）周期性做一轮分划细化：克隆级前置（`pvals`、
//!   分析共享（`Rc` 同指针）、载体内容集、`pstr` 槽集、污染槽全同）+ 节点级签名（类型集内容（驻留共享指针）、
//!   open 界、出入边按（对方等价类, 过滤类型）多重集全同），迭代到稳定；同类节点经 `scc.rs` 的 `union_into` +
//!   出边改指路径合并。签名含当前内容 ⟹ 合并是**纯存储操作**（内容全同，并集即原集合，无增量、无钩子重放）。
//! - **锁步扇出**：合并过的克隆组在侧通道上保持全同——`pvals` 并入、载体内容集增长、`pstr` 槽集 / 污染标记
//!   增长都扇出到组内全部成员；镜像集 / 对象参数集等经节点增长钩子驱动（合并保留成员身份，`flow.rs::grown`
//!   逐成员触发），天然同步；分析经 `share.rs` 共享（入口状态相同即同一 `Rc`）。
//! - **通道开关**：`RAVA_CTX_FLAT` 位 1（继承来源）/ 位 2（接收者来源）从 §4.13 的「建点时急切并入本体」
//!   改为「该来源的克隆参与等价类合并」——急切并入是 §4.13 实测的精度崩塌通道（DeepCopy 18 GiB OOM）；
//!   等价类合并只在值流与常量全同时并入，**常量不同的上下文被载体签名区分、不会被并入本体**。
//!
//! 与基线的一致性：合并不引入新值（内容全同才合并）、侧通道锁步保持克隆间分化只发生在基线同样分化的地方；
//! 四例语料 × 三种子集合一致是硬判据（试金石：位 1/2 单开 DeepCopy 不再崩）。

use std::time::Instant;

use super::*;

/// 参与合并的最小图规模与两次合并间的最小新增节点数（等价类轮成本分档）。
/// 诊断覆盖：`RAVA_EQ_AT` / `RAVA_EQ_STEP`（缺省同常量；`RAVA_EQ_AT` 取超大值即全程不做合并，
/// 用于隔离「载体 / 标记 / 扇出记账」与「合并本身」各自对输出的影响）
fn eq_at() -> usize {
    std::env::var("RAVA_EQ_AT").ok().and_then(|v| v.parse().ok()).unwrap_or(8192)
}
fn eq_step() -> usize {
    std::env::var("RAVA_EQ_STEP").ok().and_then(|v| v.parse().ok()).unwrap_or(32_768)
}
/// 分划细化最大轮数（稳定即提前退出）
const EQ_ROUNDS: usize = 8;

/// 常量格内容（`PV` 只有 `PartialEq`）的可哈希 / 可比较键：载体与克隆签名用
#[derive(PartialEq, Eq, Hash)]
enum PvKey {
    Top,
    Int(i32),
    Ints(Rc<[i32]>),
    Long(i64),
    Null,
    Par(bool),
    Arg(u16),
    Str(Rc<str>),
    /// 确定非空、类型未知的引用
    NnRef,
    /// 带对象标签的引用（按标签对象身份区分）
    ObjTag(usize),
    Class(Rc<str>),
    Offset(Rc<MemberRef>),
}

fn pv_key(p: &PV) -> PvKey {
    match p {
        PV::Top => PvKey::Top,
        PV::Const(v) => match v {
            V::Int(x) => PvKey::Int(*x),
            V::Ints(xs) => PvKey::Ints(xs.clone()),
            V::Long(x) => PvKey::Long(*x),
            V::Null => PvKey::Null,
            V::Par(x) => PvKey::Par(*x),
            V::Arg(x) => PvKey::Arg(*x),
            V::Str(s, _) => PvKey::Str(s.clone()),
            V::Offset(k) => PvKey::Offset(k.clone()),
            V::Class(c, _) => PvKey::Class(c.clone()),
            // 非空引用（类型未知、无标签）与带对象标签的引用（`PV::of` / `PV::of_ret` 的引用形态）
            V::Ref { obj: Some(o), .. } => PvKey::ObjTag(Rc::as_ptr(o) as usize),
            V::Ref { .. } => PvKey::NnRef,
            V::Top | V::Hi => PvKey::Top,
        },
    }
}

/// 等价类合并的状态（引擎侧表；不参与时零开销）
pub(super) struct Eqm {
    /// 常量格内容 → 内容序号
    ids: HashMap<PvKey, u32>,
    /// 载体（调用点, 偏移, 实参序号，不含接收者）→ 当前内容序号（只换不变窄）
    site: HashMap<(usize, u32, usize), u32>,
    /// 被调形参（方法, 槽，含接收者计位）→ 到达过的载体内容集（只增）
    pub(super) node: HashMap<(usize, usize), IdSet>,
    /// 方法克隆 → 合并通道位（1 继承来源 / 2 接收者来源；建点时标，合并时按档位过滤）
    pub(super) marks: HashMap<usize, u8>,
    /// 等价组成员（含自身；锁步扇出用）——每个成员都映射到整组
    pub(super) mates: HashMap<usize, Vec<usize>>,
    /// 观测：检查点数 / 合并的节点等价类数 / 并掉的节点数 / 登记的克隆等价组数 / 累计细化轮数 / 耗时 ms
    pub(super) stats: [u64; 6],
    last_nodes: usize,
}

impl Default for Eqm {
    fn default() -> Self {
        Eqm {
            ids: HashMap::default(),
            site: HashMap::default(),
            node: HashMap::default(),
            marks: HashMap::default(),
            mates: HashMap::default(),
            stats: [0; 6],
            last_nodes: 0,
        }
    }
}

impl Eqm {
    /// 常量格内容的序号（首次出现时驻留）
    fn id(&mut self, p: &PV) -> u32 {
        let k = pv_key(p);
        let n = self.ids.len() as u32;
        *self.ids.entry(k).or_insert(n)
    }
}

impl<'a> Engine<'a> {
    /// 载体并入：调用点 (m, off) 的实参 vals（不含接收者）流入 t 的形参槽 base..
    pub(super) fn carrier_join(&mut self, m: usize, off: u32, t: usize, base: usize, n: usize, vals: &[V]) {
        for j in base..n {
            let Some(v) = vals.get(j - base) else { continue };
            let id = self.eqm.id(&PV::of_ret(v));
            let at = (m, off, j - base);
            if self.eqm.site.get(&at) != Some(&id) {
                self.eqm.site.insert(at, id);
                self.carrier_add(t, j, id);
            }
        }
    }

    /// 载体内容 id 到达 t 的槽 j：记入内容集，锁步扇出到等价组成员
    fn carrier_add(&mut self, t: usize, j: usize, id: u32) {
        let group = self.eqm.mates.get(&t).cloned().unwrap_or_default();
        for u in group {
            self.eqm.node.entry((u, j)).or_default().insert(id);
        }
    }

    /// 克隆 t 标记合并通道（1 继承 / 2 接收者来源）
    pub(super) fn mark_merge(&mut self, t: usize, bit: u8) {
        *self.eqm.marks.entry(t).or_insert(0) |= bit;
    }

    pub(super) fn eq_merge_due(&self) -> bool {
        if self.ctx_flat_mode & 3 == 0 {
            return false;
        }
        let n = self.graph.node_count();
        let (at, step) = (eq_at(), eq_step());
        n >= at && n >= self.eqm.last_nodes + step
    }

    /// 排空点等价类合并（见模块说明）。工作队列全空时调用；`fwork` 已排空，合并是纯存储操作
    pub(super) fn eq_merge_checkpoint(&mut self) {
        let t0 = Instant::now();
        self.eqm.last_nodes = self.graph.node_count();
        self.eqm.stats[0] += 1;
        let mode = self.ctx_flat_mode & 3;
        // ── 克隆级：同成员的候选（标记通道开的克隆 + 本体），按克隆签名分等价组 ──────────
        let mut by_anchor: HashMap<usize, (Option<usize>, Vec<usize>)> = HashMap::default();
        for (i, ((key, ctx), mn)) in self.methods.iter().enumerate() {
            if mn.kind != Kind::Bytecode || mn.analysis.is_none() {
                continue;
            }
            let Some(&anchor) = self.mbase.get(key) else { continue };
            if *ctx == NOCTX {
                by_anchor.entry(anchor).or_default().0 = Some(i);
            } else if self.eqm.marks.get(&i).is_some_and(|b| b & mode != 0) {
                by_anchor.entry(anchor).or_default().1.push(i);
            }
        }
        let mut groups: Vec<Vec<usize>> = by_anchor
            .into_iter()
            .filter(|(_, (_, ms))| !ms.is_empty())
            .map(|(_, (base, mut ms))| {
                // 本体（NOCTX 条目）有则一并参与（作合并锚点；无本体时克隆间互并）
                if let Some(b) = base {
                    ms.push(b);
                }
                ms.sort_unstable();
                ms.dedup();
                ms
            })
            .collect();
        groups.sort_unstable();
        if groups.is_empty() {
            self.eqm.stats[5] += t0.elapsed().as_millis() as u64;
            return;
        }
        // 克隆签名：（pvals 内容序号向量, 分析指针, 各槽载体内容集, 各槽 pstr 集, 污染槽位）
        let mut clone_class: HashMap<usize, u32> = HashMap::default();
        let mut mate_groups: Vec<Vec<usize>> = Vec::new();
        for members in &groups {
            let mut sigs: Vec<(CloneSig, usize)> = Vec::with_capacity(members.len());
            for &t in members {
                if let Some(sig) = self.clone_sig(t) {
                    sigs.push((sig, t));
                }
            }
            sigs.sort_unstable();
            let mut i = 0;
            while i < sigs.len() {
                let mut j = i + 1;
                while j < sigs.len() && sigs[j].0 == sigs[i].0 {
                    j += 1;
                }
                if j - i > 1 {
                    let cls = mate_groups.len() as u32;
                    for &(_, t) in &sigs[i..j] {
                        clone_class.insert(t, cls);
                    }
                    mate_groups.push(sigs[i..j].iter().map(|x| x.1).collect());
                }
                i = j;
            }
        }
        if mate_groups.is_empty() {
            self.eqm.stats[5] += t0.elapsed().as_millis() as u64;
            return;
        }
        // ── 节点级：参与克隆的 P/R/S 节点（按代表），初始键 =（克隆等价组, 种类, 位置）多重集 ──
        // 代表还挂着不属于任何克隆等价组的成员位置时，键加毒（u32::MAX + 宿主），恒为单元素类
        let mut by_rep: HashMap<u32, Vec<(u32, u8, u32, u32)>> = HashMap::default();
        for i in 0..self.graph.node_count() as u32 {
            let (t, kind, slot) = match self.graph.node_at(i) {
                Node::P(t, s) => (*t, 0u8, u32::from(*s)),
                Node::R(t) => (*t, 1, 0),
                Node::S(t, o) => (*t, 2, *o),
                _ => continue,
            };
            let Some(&cls) = clone_class.get(&t) else { continue };
            let host = self.graph.rep(i);
            by_rep.entry(host).or_default().push((cls, kind, slot, 0));
        }
        // 毒化：同一代表上挂着未参与克隆等价组的 P/R/S 成员（它们不应随代表并入）
        for i in 0..self.graph.node_count() as u32 {
            let (t, kind, slot) = match self.graph.node_at(i) {
                Node::P(t, s) => (*t, 0u8, u32::from(*s)),
                Node::R(t) => (*t, 1, 0),
                Node::S(t, o) => (*t, 2, *o),
                _ => continue,
            };
            if !clone_class.contains_key(&t) {
                let host = self.graph.rep(i);
                if let Some(ks) = by_rep.get_mut(&host) {
                    ks.push((u32::MAX, kind, slot, t as u32));
                }
            }
        }
        let mut keyed: Vec<(Vec<(u32, u8, u32, u32)>, u32)> = by_rep
            .into_iter()
            .map(|(r, mut ks)| {
                ks.sort_unstable();
                ks.dedup();
                (ks, r)
            })
            .collect();
        keyed.sort_unstable();
        let part: Vec<u32> = keyed.iter().map(|x| x.1).collect();
        let keys: Vec<Vec<(u32, u8, u32, u32)>> = keyed.into_iter().map(|x| x.0).collect();
        let order: Vec<u32> = {
            let mut o: Vec<u32> = (0..part.len() as u32).collect();
            o.sort_unstable_by_key(|&k| part[k as usize]);
            o
        };
        // 初始类 = 键相同者
        let mut cur: Vec<u32> = vec![0; part.len()];
        let mut classes: Vec<Vec<usize>> = Vec::new();
        {
            let mut ix = 0;
            while ix < keys.len() {
                let mut j = ix + 1;
                while j < keys.len() && keys[j] == keys[ix] {
                    j += 1;
                }
                let cls = classes.len() as u32;
                classes.push((ix..j).collect());
                for k in ix..j {
                    cur[k] = cls;
                }
                ix = j;
            }
        }
        // ── 分划细化：签名 =（内容指针, open 界, 出边 (类, 过滤) 多重集, 入边 (类, 过滤) 多重集）──
        let graph = &self.graph;
        let bounds = &self.open_bounds;
        let mut rounds = 0u64;
        loop {
            rounds += 1;
            let node_class = |i: u32| -> u32 {
                // 参与代表 → 当前类；其余节点 → 恒等类（不参与合并）
                let r = graph.rep(i);
                match order.binary_search_by(|&k| part[k as usize].cmp(&r)) {
                    Ok(k) => cur[order[k] as usize],
                    Err(_) => r | 0x8000_0000,
                }
            };
            let mut out: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();
            let mut inn: HashMap<u32, Vec<(u32, u32)>> = HashMap::default();
            for s in 0..graph.node_count() as u32 {
                if graph.edges[s as usize].is_empty() {
                    continue;
                }
                let cs = node_class(s);
                for &(d, f) in &graph.edges[s as usize] {
                    let cd = node_class(d);
                    if cs & 0x8000_0000 == 0 {
                        out.entry(s).or_default().push((cd, f));
                    }
                    if cd & 0x8000_0000 == 0 {
                        inn.entry(d).or_default().push((cs, f));
                    }
                }
            }
            let mut sigs: Vec<(usize, u32, Vec<(u32, u32)>, Vec<(u32, u32)>, usize)> = Vec::with_capacity(part.len());
            for (k, &r) in part.iter().enumerate() {
                let content = Rc::as_ptr(&graph.sets.get_rc(r as usize)) as usize;
                let bound = bounds.get(graph.node_at(r)).copied().unwrap_or(u32::MAX);
                let mut o = out.remove(&r).unwrap_or_default();
                let mut i2 = inn.remove(&r).unwrap_or_default();
                o.sort_unstable();
                i2.sort_unstable();
                sigs.push((content, bound, o, i2, k));
            }
            sigs.sort_unstable();
            let mut next: Vec<u32> = vec![0; part.len()];
            let mut nclasses: Vec<Vec<usize>> = Vec::new();
            let mut ix = 0;
            while ix < sigs.len() {
                let mut j = ix + 1;
                while j < sigs.len() && sigs[j].0 == sigs[ix].0 && sigs[j].1 == sigs[ix].1 && sigs[j].2 == sigs[ix].2 && sigs[j].3 == sigs[ix].3 {
                    j += 1;
                }
                let cls = nclasses.len() as u32;
                nclasses.push(sigs[ix..j].iter().map(|x| x.4).collect());
                for k in ix..j {
                    next[sigs[k].4] = cls;
                }
                ix = j;
            }
            let same = nclasses.len() == classes.len() && nclasses.iter().zip(&classes).all(|(a, b)| a == b);
            classes = nclasses;
            cur = next;
            if same || rounds >= EQ_ROUNDS as u64 {
                break;
            }
        }
        self.eqm.stats[4] += rounds;
        // ── 合并：同类代表经 scc 的并路径（内容全同 ⟹ 纯存储操作）─────────────────────
        let mut merged_cls = 0u64;
        let mut merged_nodes = 0u64;
        let mut merged_groups: HashSet<u32> = HashSet::default();
        let mut unions: Vec<Vec<u32>> = Vec::new();
        for c in &classes {
            if c.len() < 2 {
                continue;
            }
            let mut ms: Vec<u32> = c.iter().map(|&k| part[k]).collect();
            ms.sort_unstable();
            unions.push(ms);
        }
        if !unions.is_empty() {
            for ms in &unions {
                let a = ms[0];
                for &b in &ms[1..] {
                    // 内容全同（签名含驻留共享指针）：并集即原集合；增量在排空点为空，非空时按增长处理
                    let dc = std::mem::take(&mut self.graph.delta[b as usize]);
                    if !dc.is_empty() {
                        self.fwork_bytes -= super::flow::tset_bytes(&dc);
                        self.graph.grow(a, &dc);
                        self.queue_delta(a, &dc);
                    }
                    drop(self.graph.take_own(b));
                    let es = std::mem::take(&mut self.graph.edges[b as usize]);
                    self.graph.edges[a as usize].extend(es);
                    self.graph.union_into(a, b);
                    self.refresh_rep_taus(a);
                    merged_nodes += 1;
                }
                merged_cls += 1;
                // 该节点等价类初始键上的全部克隆等价组（细化只分不并，成员同初始键；
                // 代表可挂多个克隆等价组的成员位置，逐一登记——这些组的节点已实际合并）
                if let Some(ix) = part.iter().position(|&r| r == ms[0]) {
                    for &(cls, ..) in &keys[ix] {
                        merged_groups.insert(cls);
                    }
                }
            }
            // 全图出边改指代表并去重（同 `collapse_cycles`）
            self.graph.clear_seen();
            self.graph.edges_since = 0;
            let obj = self.id(OBJECT);
            let mut seen: HashSet<(u32, u32)> = HashSet::default();
            for s in 0..self.graph.node_count() {
                if self.graph.edges[s].is_empty() {
                    continue;
                }
                let es = std::mem::take(&mut self.graph.edges[s]);
                let s32 = s as u32;
                seen.clear();
                let mut kept: Vec<(u32, u32)> = Vec::with_capacity(es.len());
                for (t, f) in es {
                    let r = self.graph.rep(t);
                    if r == s32 && self.ident_edge(s32, f, obj) {
                        continue;
                    }
                    if seen.insert((r, f)) {
                        kept.push((r, f));
                    }
                }
                self.graph.set_edges(s32, kept);
            }
        }
        // ── 锁步组登记：只登记确有节点合并的克隆等价组（无合并的组不扇出，避免无谓变宽）──
        let mut new_mates = 0u64;
        for (cls, group) in mate_groups.iter().enumerate() {
            if !merged_groups.contains(&(cls as u32)) {
                continue;
            }
            let mut all: Vec<usize> = group.clone();
            for &t in group {
                if let Some(prev) = self.eqm.mates.get(&t) {
                    for &u in prev {
                        if !all.contains(&u) {
                            all.push(u);
                        }
                    }
                }
            }
            all.sort_unstable();
            all.dedup();
            if all.len() > 1 {
                for &t in &all {
                    self.eqm.mates.insert(t, all.clone());
                }
                new_mates += 1;
            }
        }
        self.eqm.stats[1] += merged_cls;
        self.eqm.stats[2] += merged_nodes;
        self.eqm.stats[3] += new_mates;
        self.eqm.stats[5] += t0.elapsed().as_millis() as u64;
    }

    /// 克隆 t 的等价前置签名：常量格、分析共享指针、各槽载体内容集、各槽 pstr 集、污染槽位。
    /// None = 不参与（无分析 / 非字节码）
    fn clone_sig(&mut self, t: usize) -> Option<CloneSig> {
        let a = self.methods[t].analysis.as_ref()?;
        let n = self.methods[t].ptypes.len();
        let mut pv = Vec::with_capacity(n);
        if let Some(vs) = self.pvals.get(&t) {
            for v in vs {
                pv.push(self.eqm.id(v));
            }
        } else {
            pv.extend(std::iter::repeat(u32::MAX).take(n));
        }
        let mut carriers = Vec::new();
        let mut pstrs = Vec::new();
        let mut taints = Vec::new();
        for j in 0..n {
            if let Some(cs) = self.eqm.node.get(&(t, j)) {
                let mut xs: Vec<u32> = cs.iter().collect();
                xs.sort_unstable();
                carriers.push((j as u16, xs));
            }
            if let Some(lits) = self.pstr_set_ids(pstrs::PSlot::M(t, j)) {
                pstrs.push((j as u16, lits));
            }
            if self.ptaint.contains(&(t, j)) {
                taints.push(j as u16);
            }
        }
        carriers.sort_unstable();
        Some(CloneSig { pv, analysis: Rc::as_ptr(a) as usize, carriers, pstrs, taints })
    }
}

/// 克隆等价前置签名（全字段参与相等判定；`pvals` 以内容序号编码）
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct CloneSig {
    pv: Vec<u32>,
    analysis: usize,
    carriers: Vec<(u16, Vec<u32>)>,
    pstrs: Vec<(u16, Vec<u32>)>,
    taints: Vec<u16>,
}

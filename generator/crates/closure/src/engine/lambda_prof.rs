//! 引擎：枢纽 lambda 重放与方法引用接收者展开剖析（`rava closure --lambda-prof <秒>`，只读诊断，缺省关闭，不影响分析结果）。
//!
//! 记三类计数，每隔给定秒数向 stderr 打一段前列（分析未结束、被看门狗终止时仍可读到）：
//! - 枢纽重放：枢纽把已登记的 lambda / 手写实现对象接收者逐调用点送去派发（`link_hub` / `hub_recv`）的次数，
//!   按枢纽计，附接入调用点数、lambda 数、集合规模；
//! - lambda 调用：登记的 lambda 调用读者（`LCall`）数与其重跑次数，按 lambda 计，附不同调用点数；
//! - 方法引用接收者展开：虚 / 接口方法引用的 lambda 在调用点逐接收者派发的次数与不同接收者数，按 lambda 计。

use std::time::{Duration, Instant};

use super::*;

/// 每多少次计数查一次时钟
const TICK_MASK: u32 = (1 << 14) - 1;
/// 每段打印的前列条数
const TOP: usize = 25;

#[derive(Default)]
struct LamStat {
    /// 登记的调用读者数
    calls: u64,
    /// 读者步进次数（含首次）
    steps: u64,
    /// 方法引用逐接收者派发次数（接收者表长度之和，含 `dispatched` 去重挡下的）
    recv_dispatch: u64,
    /// 不同调用点 / 不同接收者
    sites: HashSet<(usize, u32)>,
    recvs: HashSet<u32>,
}

pub(super) struct LambdaProf {
    every: Duration,
    t0: Instant,
    last: Instant,
    tick: u32,
    seq: u32,
    /// 枢纽 → 重放次数
    hub_replay: HashMap<u32, u64>,
    /// 枢纽 → 重放过的不同 (调用点, 接收者)
    hub_pairs: HashMap<u32, u64>,
    lams: HashMap<u32, LamStat>,
    replay_total: u64,
    recv_total: u64,
    /// 方法引用分派分列：非字节码临时调用逐接收者 / 字节码调用点少量接收者逐个 / 接入枢纽次数
    recv_by: [u64; 3],
    /// 非字节码调用方的临时 lambda 调用次数
    transient: u64,
    /// 新建读者时同一调用点已有同一 lambda 的其他读者；其中实参 / 返回类型 / 结果节点不同者
    dup_site: u64,
    dup_by: [u64; 3],
    /// 读者复活次数
    revived: u64,
    /// 本段 dump 间隙的 RSS 采样峰值（`lprof_rss_sample`；dump 时打印后清零）
    rss_max: u64,
    /// 全程 RSS 采样峰值
    rss_max_all: u64,
}

impl LambdaProf {
    pub(super) fn new(secs: u64) -> LambdaProf {
        let now = Instant::now();
        LambdaProf {
            every: Duration::from_secs(secs.max(1)),
            t0: now,
            last: now,
            tick: 0,
            seq: 0,
            hub_replay: HashMap::default(),
            hub_pairs: HashMap::default(),
            lams: HashMap::default(),
            replay_total: 0,
            recv_total: 0,
            recv_by: [0; 3],
            transient: 0,
            dup_site: 0,
            dup_by: [0; 3],
            revived: 0,
            rss_max: 0,
            rss_max_all: 0,
        }
    }
}

impl Engine<'_> {
    /// 开启剖析（`secs` = 打印间隔秒数，0 = 关闭）
    pub fn set_lambda_prof(&mut self, secs: u64) {
        crate::alloc_prof::ENABLED.store(secs > 0, std::sync::atomic::Ordering::Relaxed);
        self.lprof = (secs > 0).then(|| Box::new(LambdaProf::new(secs)));
    }

    /// RSS 采样（剖析开启时记录段内 / 全程峰值；由 `lprof_tick` 与流排空周期调用，
    /// 捕捉 dump 间隙的尖峰幅度）
    pub(super) fn lprof_rss_sample(&mut self) {
        let Some(p) = self.lprof.as_mut() else { return };
        let r = stats::cur_rss_mb();
        if r > p.rss_max {
            p.rss_max = r;
        }
        if r > p.rss_max_all {
            p.rss_max_all = r;
        }
    }

    /// 分析结束时打印末段（未开剖析时无输出）
    pub fn lambda_prof_final(&mut self) {
        if let Some(p) = self.lprof.as_mut() {
            p.seq += 1;
        }
        self.lprof_dump();
    }

    /// 枢纽 h 向调用点重放接收者 r（`fresh` = 该调用点首次收到 r）
    pub(super) fn lprof_replay(&mut self, h: u32, fresh: bool) {
        let Some(p) = self.lprof.as_mut() else { return };
        *p.hub_replay.entry(h).or_default() += 1;
        if fresh {
            *p.hub_pairs.entry(h).or_default() += 1;
        }
        p.replay_total += 1;
        self.lprof_tick();
    }

    /// 登记 lambda 调用读者（新读者）或读者步进
    pub(super) fn lprof_call(&mut self, lid: u32, m: usize, off: u32, new: bool) {
        let Some(p) = self.lprof.as_mut() else { return };
        let s = p.lams.entry(lid).or_default();
        s.steps += 1;
        if new {
            s.calls += 1;
            s.sites.insert((m, off));
        }
        self.lprof_tick();
    }

    /// 方法引用 lid 在调用点按接收者表 recv 逐个派发
    pub(super) fn lprof_recv(&mut self, lid: u32, recv: &[u32], class: usize) {
        let Some(p) = self.lprof.as_mut() else { return };
        p.recv_by[class] += recv.len() as u64;
        let s = p.lams.entry(lid).or_default();
        s.recv_dispatch += recv.len() as u64;
        s.recvs.extend(recv.iter().copied());
        p.recv_total += recv.len() as u64;
        self.lprof_tick();
    }

    /// 非字节码调用方的临时调用 / 新读者的调用点已有同一 lambda 的读者
    pub(super) fn lprof_kind(&mut self, transient: bool, dup: Option<[bool; 3]>) {
        let Some(p) = self.lprof.as_mut() else { return };
        p.transient += u64::from(transient);
        if let Some(d) = dup {
            p.dup_site += 1;
            for (n, x) in p.dup_by.iter_mut().zip(d) {
                *n += u64::from(x);
            }
        }
    }

    pub(super) fn lprof_revive(&mut self) {
        if let Some(p) = self.lprof.as_mut() {
            p.revived += 1;
        }
    }

    pub(super) fn lprof_tick(&mut self) {
        let mut due = false;
        let mut sample = false;
        if let Some(p) = self.lprof.as_mut() {
            p.tick = p.tick.wrapping_add(1);
            sample = p.tick & TICK_MASK == 0;
            due = sample && p.last.elapsed() >= p.every;
        }
        if sample {
            self.lprof_rss_sample();
        }
        if !due {
            return;
        }
        if let Some(p) = self.lprof.as_mut() {
            p.last = Instant::now();
            p.seq += 1;
        }
        self.lprof_dump();
        if let Some(p) = self.lprof.as_mut() {
            p.rss_max = 0;
        }
        self.fwork_bytes_peak = 0;
    }

    /// 打印一段前列（stderr，`[lambda-prof #序号 秒]` 前缀）
    pub(super) fn lprof_dump(&self) {
        let Some(p) = self.lprof.as_ref() else { return };
        let tag = format!("[lambda-prof #{} {}s]", p.seq, p.t0.elapsed().as_secs());
        eprintln!(
            "{tag} rss_peak={}MiB methods={} hubs={} lcalls={} g={} lambdas={} replay_total={} recv_total={} recv_by(transient/direct/hub_link)={:?} transient={} dup_site={} dup_by(args/ret/res)={:?} revived={}",
            stats::peak_rss_mb(),
            self.methods.len(),
            self.hubs.len(),
            self.lcalls.len(),
            self.g.len(),
            self.lambdas.len(),
            p.replay_total,
            p.recv_total,
            p.recv_by,
            p.transient,
            p.dup_site,
            p.dup_by,
            p.revived
        );
        // 工作表与大表瞬时状态（定位周期性 RSS 尖峰：工作表突发 / 大表扩容）
        eprintln!(
            "{tag} wip mwork={} swork={} cwork={} fwork={} watch={} site_nodes={} dispatched={} objs={} hubs_sites={} mref_keys={}",
            self.mwork.len(),
            self.swork.len(),
            self.cwork.len(),
            self.fwork.len(),
            self.watch.len(),
            self.site_nodes.len(),
            self.dispatched.values().map(|s| s.len()).sum::<usize>(),
            self.objs.len(),
            self.hub_sites.len(),
            self.mref_keys.len(),
        );
        // 排空点等价类合并（eqmerge.rs）：累计检查点 / 合并类 / 并掉节点 / 克隆组 / 细化轮 / 耗时 / 标记克隆数 /
        // 预算降级（预算字节 / 触发数 / 最近触发 RSS；预算关时全 0）
        eprintln!(
            "{tag} eqmerge ckpts={} classes={} nodes={} groups={} rounds={} ms={} marked={} budget={}MiB bhits={} btrig_rss={}MiB",
            self.eqm.stats[0],
            self.eqm.stats[1],
            self.eqm.stats[2],
            self.eqm.stats[3],
            self.eqm.stats[4],
            self.eqm.stats[5],
            self.eqm.marks.len(),
            self.eq_budget >> 20,
            self.eqm.budget_hits,
            self.eqm.budget_trig_rss,
        );
        // 内存剖析（同上，定位 RSS 尖峰）：分配器计数 + 主要驻留结构的字节估计。
        // rss_max 为上一段 dump 间隙的采样峰值；live（活跃字节）与 rss 之差是分配器滞留
        let ac = &crate::alloc_prof::COUNTERS;
        let mib = |b: u64| b >> 20;
        eprintln!(
            "{tag} mem rss={}MiB rss_max={}MiB all_max={}MiB live={}MiB live_peak={}MiB alloc_total={}GiB allocs={} frees={}",
            stats::cur_rss_mb(),
            p.rss_max,
            p.rss_max_all,
            mib(ac.live_bytes()),
            mib(ac.peak_live.load(std::sync::atomic::Ordering::Relaxed)),
            ac.alloc_bytes.load(std::sync::atomic::Ordering::Relaxed) >> 30,
            ac.allocs.load(std::sync::atomic::Ordering::Relaxed),
            ac.frees.load(std::sync::atomic::Ordering::Relaxed),
        );
        let mut sizes = String::new();
        for (k, b) in ac.live.iter().enumerate() {
            let v = b.load(std::sync::atomic::Ordering::Relaxed);
            if v > 64 << 20 {
                sizes.push_str(&format!(" 2^{k}:{}MiB", v >> 20));
            }
        }
        eprintln!("{tag} sizeclass(since-enable){sizes}");
        // 类型流图：集合 / 边 / 待推增量的字节；queued_delta 随 fwork 突发增长即为尖峰主项
        let (sets_bytes, sets_n) = self.graph.sets.census();
        let edges: usize = self.graph.edges.iter().map(|e| e.len()).sum();
        let mut delta_bytes = 0usize;
        let mut delta_nodes = 0usize;
        let mut by_kind: [u64; 19] = [0; 19];
        let mut top: Vec<(usize, u32)> = Vec::new();
        for &i in &self.fwork {
            let d = &self.graph.delta[i as usize];
            if d.is_empty() {
                continue;
            }
            let b = d.classes.heap_bytes() + d.open.heap_bytes() + std::mem::size_of::<TypeSet>();
            delta_bytes += b;
            delta_nodes += 1;
            by_kind[stats::kind_ix(&self.graph.node(i))] += b as u64;
            top.push((b, i));
        }
        eprintln!(
            "{tag} census graph nodes={} sets={:.0}MiB/{}(uniq {}) edges={}MiB queued_delta={:.0}MiB/{} peak={}MiB seen={} members={} fmemo={}MiB nodes_dense≈{}MiB",
            self.graph.node_count(),
            sets_bytes as f64 / 1048576.0,
            sets_n,
            self.graph.sets.unique(),
            (edges * 8) as f64 / 1048576.0,
            delta_bytes as f64 / 1048576.0,
            delta_nodes,
            self.fwork_bytes_peak >> 20,
            self.graph.seen_len(),
            self.graph.members.len(),
            self.graph.fmemo_bytes >> 20,
            (self.graph.node_count() * 160) >> 20,
        );
        let kinds: Vec<String> = by_kind
            .iter()
            .enumerate()
            .filter(|(_, b)| **b > 4 << 20)
            .map(|(k, b)| format!("{}:{:.0}MiB", stats::KIND_NAMES[k], *b as f64 / 1048576.0))
            .collect();
        top.sort_unstable_by(|a, b| b.0.cmp(&a.0));
        let tops: Vec<String> = top
            .into_iter()
            .take(6)
            .map(|(b, i)| format!("{}={:.0}MiB", self.node_str(self.graph.node(i)), b as f64 / 1048576.0))
            .collect();
        eprintln!("{tag} census fwork kinds: {} top: [{}]", kinds.join(" "), tops.join(" "));
        // uniq 内容表征画像（§4.16 相异内容表征压缩的画像，只读）：驻留表逐内容累计——真实驻留字节
        // （原始 census 按节点重复计数共享内容）/ 稠密 words 字节与非零占比（fill）/ 块式（4096 id）估计。
        // lcalls.done 与收窄记忆值未驻留、逐条目持有，同画像并列（bytes 即其真实驻留）
        let (uc, uo, uovh, un) = self.graph.sets.uniq_profile();
        let (mut lc, mut lo) = (idset::ReprProfile::default(), idset::ReprProfile::default());
        for c in &self.lcalls {
            c.done.classes.profile(&mut lc);
            c.done.open.profile(&mut lo);
        }
        let (mut fc, mut fo) = (idset::ReprProfile::default(), idset::ReprProfile::default());
        for v in self.graph.fmemo.values() {
            v.1.classes.profile(&mut fc);
            v.1.open.profile(&mut fo);
        }
        let hp = |p: &idset::ReprProfile| {
            format!(
                "n={} dense={} sp={} bytes={:.0}MiB words={:.0}MiB fill={} est_b={:.0}MiB est_s={:.0}MiB est_a={:.0}MiB",
                p.sets,
                p.dense_n,
                p.sparse_n,
                p.bytes as f64 / 1048576.0,
                p.words_cap as f64 / 1048576.0,
                if p.words == 0 { 0 } else { p.nz_words * 100 / p.words },
                p.est_blocks as f64 / 1048576.0,
                p.est_sparse as f64 / 1048576.0,
                p.est_adaptive as f64 / 1048576.0,
            )
        };
        eprintln!(
            "{tag} census uniq sets n={un} true={:.0}MiB ovh={:.0}MiB cls[{}] open[{}] fillhist={:?} lenhist(cls)={:?}",
            (uc.bytes + uo.bytes) as f64 / 1048576.0,
            uovh as f64 / 1048576.0,
            hp(&uc),
            hp(&uo),
            uc.fill_hist,
            uc.len_hist,
        );
        eprintln!("{tag} census uniq lcalls[{}] fmemo[{}]", hp(&lc) + &format!(" open[{}]", hp(&lo)), hp(&fc));
        // 主要驻留表的条目数（字节估计按条目均摊，够归因量级即可）
        let watch_n: usize = self.watch.values().map(|s| s.len()).sum();
        let call_watch_n: usize = self.call_watch.values().map(|s| s.len()).sum();
        let disp_n: usize = self.dispatched.values().map(|s| s.len()).sum();
        let recv_done_n: usize = self.recv_done.values().flat_map(|m| m.values()).map(|v| v.len()).sum::<usize>() + self.recv_fp.len();
        let ldone_n: usize = self
            .lambda_done
            .values()
            .flat_map(|m| m.values())
            .map(|h| h.len())
            .sum::<usize>()
            + self.hub_linked.values().map(|s| s.len()).sum::<usize>()
            + self.hub_lsent.values().map(|s| s.len()).sum::<usize>();
        let lcall_live = self.lcalls.iter().filter(|c| c.live).count();
        let lcall_done_b: usize = self.lcalls.iter().map(|c| c.done.classes.heap_bytes() + c.done.open.heap_bytes()).sum();
        let ctx_n = self.methods.values().filter(|m| m.ctx != NOCTX).count();
        let anev: usize = self.methods.values().filter_map(|m| m.analysis.as_ref()).map(|a| a.events.len()).sum();
        let mnode_b = self.methods.len() * std::mem::size_of::<MNode>();
        eprintln!(
            "{tag} census tables watch={}/{}={}MiB call_watch={} dispatched={} recv_done={} ldone={} lcalls={}/{} done={}MiB methods={}={}MiB ctx={} anev={}≈{}MiB objs={} ids={} lambdas={} mref={} site_nodes={}",
            self.watch.len(),
            watch_n,
            (self.watch.len() * 80 + watch_n * 24) >> 20,
            call_watch_n,
            disp_n,
            recv_done_n,
            ldone_n,
            lcall_live,
            self.lcalls.len(),
            lcall_done_b >> 20,
            self.methods.len(),
            mnode_b >> 20,
            ctx_n,
            anev,
            (anev * 48) >> 20,
            self.objs.len(),
            self.ids.len(),
            self.lambdas.len(),
            self.mref_keys.len(),
            self.site_nodes.len(),
        );
        // 大表深字节（§4.16 画像：把活跃字节归因到结构——capacity 口径，与计数分配器同量级即可）：
        // 图固定列 / ids 表 / 读者与分派表 / 三级 lambda_done / recv_done 内层 / lcalls 列与实参值
        let (gids, gcols, gedges, _gdelta, gseen, gextra) = self.graph.prof_columns();
        let hb = |m_cap: usize, slot: usize| m_cap * slot * 9 / 7;
        let watch_b = hb(self.watch.capacity(), std::mem::size_of::<(Node, HashSet<(usize, u32)>)>())
            + self.watch.values().map(|s| hb(s.capacity(), 12)).sum::<usize>();
        let call_watch_b = hb(self.call_watch.capacity(), std::mem::size_of::<(Node, HashSet<u32>)>())
            + self.call_watch.values().map(|s| hb(s.capacity(), 4)).sum::<usize>();
        let disp_b = hb(self.dispatched.capacity(), std::mem::size_of::<(usize, HashSet<(u32, u32, u32)>)>())
            + self.dispatched.values().map(|s| hb(s.capacity(), 12)).sum::<usize>()
            + hb(self.hub_linked.capacity(), std::mem::size_of::<(usize, HashSet<(u32, u32)>)>())
            + self.hub_linked.values().map(|s| hb(s.capacity(), 8)).sum::<usize>()
            + hb(self.hub_lsent.capacity(), std::mem::size_of::<(usize, HashSet<(u32, u32, u32)>)>())
            + self.hub_lsent.values().map(|s| hb(s.capacity(), 12)).sum::<usize>();
        let ldone_b = hb(self.lambda_done.capacity(), std::mem::size_of::<(usize, HashMap<u32, HashMap<LambdaKey, u32>>)>())
            + self.lambda_done.values().map(|m| hb(m.capacity(), std::mem::size_of::<(u32, HashMap<LambdaKey, u32>)>()) + m.values().map(|im| hb(im.capacity(), std::mem::size_of::<(LambdaKey, u32)>())).sum::<usize>()).sum::<usize>();
        let recv_done_b = hb(self.recv_done.capacity(), std::mem::size_of::<(usize, HashMap<u32, Vec<u32>>)>())
            + self.recv_done.values().map(|m| hb(m.capacity(), std::mem::size_of::<(u32, Vec<u32>)>()) + m.values().map(|v| v.capacity() * 4).sum::<usize>()).sum::<usize>();
        // lcalls：列 + 实参值（Feed::S 的 TypeSet 深堆）+ done 画像已在上一行
        let mut args_b = 0usize;
        for c in &self.lcalls {
            args_b += c.call.1.capacity() * 24;
            for a in &c.call.1 {
                if let Some(v) = a {
                    args_b += v.capacity() * std::mem::size_of::<Feed>();
                    for f in v {
                        if let Feed::S(s) = f {
                            args_b += s.classes.heap_bytes() + s.open.heap_bytes();
                        }
                    }
                }
            }
        }
        let lcall_col = self.lcalls.capacity() * std::mem::size_of::<LCall>();
        // 子类型判定缓存（SubRow：按过滤类型的 known/yes 位图，尺寸随查过的 x id 增长——2^16 段候选）
        let subrow_b: usize = self.sub_rows.iter().map(|r| r.prof_bytes()).sum();
        eprintln!(
            "{tag} census deep gcols={:.0}MiB gids={:.0}MiB gedges={:.0}MiB gseen={:.0}MiB gextra={:.0}MiB watch={:.0}MiB call_watch={:.0}MiB disp={:.0}MiB ldone={:.0}MiB recv_done={:.0}MiB lcall_col={:.0}MiB largs={:.0}MiB subrows={:.0}MiB lcalls_n={}",
            gcols as f64 / 1048576.0,
            gids as f64 / 1048576.0,
            gedges as f64 / 1048576.0,
            gseen as f64 / 1048576.0,
            gextra as f64 / 1048576.0,
            watch_b as f64 / 1048576.0,
            call_watch_b as f64 / 1048576.0,
            disp_b as f64 / 1048576.0,
            ldone_b as f64 / 1048576.0,
            recv_done_b as f64 / 1048576.0,
            lcall_col as f64 / 1048576.0,
            args_b as f64 / 1048576.0,
            subrow_b as f64 / 1048576.0,
            self.lcalls.len(),
        );
        let hub_recvs: usize = self.hubs.iter().map(|h| h.recvs.len()).sum();
        let hub_special: usize = self.hubs.iter().flat_map(|h| h.special.values()).map(|v| v.len()).sum();
        let hub_links: usize = self.hubs.iter().map(|h| h.links.len()).sum();
        let hub_edged: usize = self.hubs.iter().map(|h| h.edged.len()).sum();
        let hub_lam: usize = self.hubs.iter().map(|h| h.lambdas.len()).sum();
        eprintln!(
            "{tag} census hubs n={} recvs={}MiB special={}MiB links={}MiB lambdas={} edged={} grow_max={} upgraded={}",
            self.hubs.len(),
            hub_recvs * 4 >> 20,
            hub_special * 4 >> 20,
            (hub_links * 16) >> 20,
            hub_lam * 4 >> 20,
            hub_edged >> 20,
            self.grow_max,
            self.hub_upgraded,
        );
        let mut hs: Vec<(&u32, &u64)> = p.hub_replay.iter().collect();
        hs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (&h, &n) in hs.into_iter().take(TOP) {
            let hub = &self.hubs[h as usize];
            let (o, nm, d) = hub.site.key();
            let set = match hub.open {
                Some(o) => format!("open {}", self.names[o as usize]),
                // 增长枢纽（`HubSet::Grow`）
                None => format!("grow {}", hub.recvs.len()),
            };
            eprintln!(
                "{tag} hub H{h} replay={n} fresh={} links={} lambdas={} special={} plain={} [{set}] {o}.{nm}{d} via={}",
                p.hub_pairs.get(&h).copied().unwrap_or(0),
                hub.links.len(),
                hub.lambdas.len(),
                hub.special.len(),
                hub.plain.len(),
                self.via_label(&hub.via),
            );
        }
        // 方法上下文克隆前列：同一方法的上下文数、上下文对象所属类型的方法数
        let mut by_m: HashMap<String, u64> = HashMap::default();
        let mut by_c: HashMap<u32, u64> = HashMap::default();
        for i in 0..self.methods.len() {
            let c = self.methods[i].ctx;
            if c == NOCTX {
                continue;
            }
            *by_m.entry(self.method_label(i)).or_default() += 1;
            *by_c.entry(c).or_default() += 1;
        }
        let mut ms: Vec<(String, u64)> = by_m.into_iter().collect();
        ms.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        for (l, n) in ms.into_iter().take(TOP) {
            eprintln!("{tag} ctxm n={n} {l}");
        }
        let mut cs: Vec<(u32, u64)> = by_c.into_iter().collect();
        cs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        for (c, n) in cs.into_iter().take(TOP) {
            eprintln!("{tag} ctxc n={n} {}", self.names[c as usize]);
        }
        self.ctx_prof_dump(&tag);
        self.hub_prof_dump(&tag);
        let mut ls: Vec<(&u32, &LamStat)> = p.lams.iter().collect();
        ls.sort_by(|a, b| (b.1.recv_dispatch + b.1.steps).cmp(&(a.1.recv_dispatch + a.1.steps)).then(a.0.cmp(b.0)));
        for (&lid, s) in ls.into_iter().take(TOP) {
            let (imh, iface) = self.lambdas.get(&lid).map_or((String::new(), String::new()), |l| (format!("k{} {}", l.imh.kind, l.imh.member), l.iface.clone()));
            eprintln!(
                "{tag} lambda {} calls={} steps={} sites={} recv_dispatch={} recvs={} iface={iface} impl={imh}",
                self.names[lid as usize],
                s.calls,
                s.steps,
                s.sites.len(),
                s.recv_dispatch,
                s.recvs.len(),
            );
        }
    }

    fn via_label(&self, v: &Via) -> String {
        match &v.from {
            From::Method(m) => format!("{} {}@{}", v.kind, self.ctx_label(*m), v.off.unwrap_or(0)),
            other => format!("{} {other:?}", v.kind),
        }
    }
}

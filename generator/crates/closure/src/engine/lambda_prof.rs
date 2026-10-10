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
        }
    }
}

impl Engine<'_> {
    /// 开启剖析（`secs` = 打印间隔秒数，0 = 关闭）
    pub fn set_lambda_prof(&mut self, secs: u64) {
        self.lprof = (secs > 0).then(|| Box::new(LambdaProf::new(secs)));
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
    pub(super) fn lprof_recv(&mut self, lid: u32, recv: &[u32]) {
        let Some(p) = self.lprof.as_mut() else { return };
        let s = p.lams.entry(lid).or_default();
        s.recv_dispatch += recv.len() as u64;
        s.recvs.extend(recv.iter().copied());
        p.recv_total += recv.len() as u64;
        self.lprof_tick();
    }

    fn lprof_tick(&mut self) {
        let Some(p) = self.lprof.as_mut() else { return };
        p.tick = p.tick.wrapping_add(1);
        if p.tick & TICK_MASK != 0 || p.last.elapsed() < p.every {
            return;
        }
        p.last = Instant::now();
        p.seq += 1;
        self.lprof_dump();
    }

    /// 打印一段前列（stderr，`[lambda-prof #序号 秒]` 前缀）
    pub(super) fn lprof_dump(&self) {
        let Some(p) = self.lprof.as_ref() else { return };
        let tag = format!("[lambda-prof #{} {}s]", p.seq, p.t0.elapsed().as_secs());
        eprintln!(
            "{tag} rss_peak={}MiB methods={} hubs={} lcalls={} g={} lambdas={} replay_total={} recv_total={}",
            stats::peak_rss_mb(),
            self.methods.len(),
            self.hubs.len(),
            self.lcalls.len(),
            self.g.len(),
            self.lambdas.len(),
            p.replay_total,
            p.recv_total
        );
        let mut hs: Vec<(&u32, &u64)> = p.hub_replay.iter().collect();
        hs.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
        for (&h, &n) in hs.into_iter().take(TOP) {
            let hub = &self.hubs[h as usize];
            let (o, nm, d) = hub.site.key();
            let set = match (hub.open, &hub.set) {
                (Some(o), _) => format!("open {}", self.names[o as usize]),
                (None, Some(s)) => format!("exact {}", s.len()),
                _ => "vm".into(),
            };
            eprintln!(
                "{tag} hub H{h} replay={n} fresh={} links={} lambdas={} special={} plain={} parent={:?} [{set}] {o}.{nm}{d} via={}",
                p.hub_pairs.get(&h).copied().unwrap_or(0),
                hub.links.len(),
                hub.lambdas.len(),
                hub.special.len(),
                hub.plain.len(),
                hub.parent,
                self.via_label(&hub.via),
            );
        }
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

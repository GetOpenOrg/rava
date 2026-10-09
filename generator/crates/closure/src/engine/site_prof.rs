//! 引擎：读者站点重跑剖析（V12，只读诊断，`rava closure --site-prof` 开启，只进 `summary.perf.site_prof`，不影响分析结果）。
//!
//! - 触发源：站点入 `swork` 时记下首个触发（增长的节点种类，或 open 重展开 / 名字槽 / 放行等非节点来源），
//!   入队期间的后续触发计为合并；
//! - 产出：重跑前后对照工作量快照（流边数、有增量的并入、方法节点数、新分派目标、枢纽数、方法入队），
//!   任一变化即「有产出」，按变化种类记掩码；
//! - 调用点：记分派路径（静态 / special / 非虚 / 虚调用版本命中 / 未命中逐个派发 / 未命中经枢纽）、
//!   触发节点在接收者来源还是实参来源，以及各段耗时（按有无产出分开累计）。

use super::*;

/// 非节点触发源（序号接在节点种类之后）
pub(super) const TRIG_REOPEN: u8 = stats::KINDS as u8;
pub(super) const TRIG_PSTR: u8 = TRIG_REOPEN + 1;
pub(super) const TRIG_PSTR_WAKE: u8 = TRIG_REOPEN + 2;
pub(super) const TRIG_PSTR_REANALYZED: u8 = TRIG_REOPEN + 3;
pub(super) const TRIG_SERVICES: u8 = TRIG_REOPEN + 4;
pub(super) const TRIG_TAINT: u8 = TRIG_REOPEN + 5;
pub(super) const TRIG_RELEASE: u8 = TRIG_REOPEN + 6;
pub(super) const TRIG_PATTERN: u8 = TRIG_REOPEN + 7;
/// 无名字查找点的共享反射对象标记所指增长（`method_marks.rs`）
pub(super) const TRIG_MARK_ALL: u8 = TRIG_REOPEN + 8;
const TRIG_NAMES: [&str; 9] = ["reopen", "pstr", "pstr_wake", "pstr_reanalyzed", "services", "taint", "release", "pattern", "mark_all"];

/// 调用点路径
pub(super) const PATH_STATIC: u8 = 1;
pub(super) const PATH_SPECIAL: u8 = 2;
pub(super) const PATH_NONVIRT: u8 = 3;
pub(super) const PATH_VFP_HIT: u8 = 4;
pub(super) const PATH_VSMALL: u8 = 5;
pub(super) const PATH_VHUB_SAME: u8 = 6;
pub(super) const PATH_VHUB_NEW: u8 = 7;
const PATH_NAMES: [&str; 8] = ["-", "static", "special", "nonvirt", "vfp_hit", "vsmall", "vhub_same", "vhub_new"];

/// 触发节点相对调用点来源的角色
const ROLE_NAMES: [&str; 5] = ["-", "recv", "arg", "other_node", "not_node"];
pub(super) const ROLE_RECV: u8 = 1;
pub(super) const ROLE_ARG: u8 = 2;
pub(super) const ROLE_OTHER: u8 = 3;
pub(super) const ROLE_NOT_NODE: u8 = 4;

/// 调用点分段
pub(super) const SEG_PRE: usize = 0;
pub(super) const SEG_ARGS: usize = 1;
pub(super) const SEG_RECV: usize = 2;
pub(super) const SEG_DISPATCH: usize = 3;
pub(super) const SEG_HUB: usize = 4;
pub(super) const SEG_LINK: usize = 5;
pub(super) const SEG_EDGE_RECV: usize = 6;
pub(super) const SEG_STATIC: usize = 7;
pub(super) const SEG_POST: usize = 8;
/// 枢纽接入的细分：实参汇入 / 名字槽与常量并入 / lambda 与按调用点目标重放 / 展开
pub(super) const SEG_LINK_FEED: usize = 9;
pub(super) const SEG_LINK_VALS: usize = 10;
pub(super) const SEG_LINK_REPLAY: usize = 11;
pub(super) const SEG_LINK_EXPAND: usize = 12;
const SEG_NAMES: [&str; 13] = [
    "pre_hooks", "args_feeds", "recv_set", "dispatch_one", "hub_get", "link_hub", "edge_recv", "static_edge", "post_hooks",
    "link_feed", "link_vals", "link_replay", "link_expand",
];

/// 产出掩码位
const PROD_NAMES: [&str; 6] = ["edges", "grow", "methods", "targets", "hubs", "mpush"];

#[derive(Clone, Copy)]
pub(super) struct Trig {
    src: u8,
    node: Option<Node>,
}

#[derive(Default)]
pub(super) struct SiteProf {
    trig: HashMap<(usize, u32), Trig>,
    /// 按触发源：入队次数 / 已在队中被合并次数
    pushes: BTreeMap<u8, [u64; 2]>,
    /// 剖析开关（`rava closure --site-prof`；缺省关闭，关闭时各钩子空操作）
    pub(super) enabled: bool,
    /// 当前重跑（剖析开着时）
    on: bool,
    cur: Option<Trig>,
    pub(super) path: u8,
    pub(super) role: u8,
    seg_t: Option<(usize, std::time::Instant)>,
    seg_cur: [u64; SEG_NAMES.len()],
    /// 工作量计数（快照用）：新分派目标、方法入队
    pub(super) dispatch_new: u64,
    pub(super) mpush: u64,
    /// (事件种类, 触发源, 路径, 角色) → [次数, 有产出次数, 总 ns, 有产出 ns]
    agg: BTreeMap<(u8, u8, u8, u8), [u64; 4]>,
    /// 产出掩码 → 次数
    masks: BTreeMap<u8, u64>,
    /// 调用点分段耗时：[无产出, 有产出] × 段 → ns
    seg: [[u64; SEG_NAMES.len()]; 2],
    /// 虚调用版本未命中：[次数, Σ|接收者|, Σ|新增接收者|, 新增为 0 的次数]
    pub(super) vmiss: [u64; 4],
}

type Snap = [u64; 6];

impl<'a> Engine<'a> {
    /// 开关读者站点重跑剖析（`--site-prof`）
    pub fn set_site_prof(&mut self, on: bool) {
        self.ctx.stats.borrow_mut().sprof.enabled = on;
    }

    /// 站点入队（剖析记首个触发源）
    pub(super) fn push_site(&mut self, w: (usize, u32), src: u8, node: Option<Node>) {
        let new = self.in_swork.insert(w);
        if new {
            self.swork.push_back(w);
        }
        self.ctx.stats.borrow_mut().sprof.note_push(w, src, node, new);
    }

    fn prof_snap(&self) -> Snap {
        let st = self.ctx.stats.borrow();
        [
            self.graph.edge_count as u64,
            self.graph.adds[1],
            self.methods.len() as u64,
            st.sprof.dispatch_new,
            self.hubs.len() as u64,
            st.sprof.mpush,
        ]
    }

    /// 读者站点重跑开始：取出触发源，开计时
    pub(super) fn prof_begin(&mut self, w: (usize, u32)) -> Snap {
        if !self.ctx.stats.borrow().sprof.enabled {
            return Snap::default();
        }
        let snap = self.prof_snap();
        let mut st = self.ctx.stats.borrow_mut();
        let p = &mut st.sprof;
        p.cur = p.trig.remove(&w);
        p.on = true;
        p.path = 0;
        p.role = 0;
        p.seg_t = None;
        p.seg_cur = Default::default();
        snap
    }

    /// 一个事件重跑结束（同一偏移多个事件各记一次）
    pub(super) fn prof_event(&mut self, kind: usize, before: Snap, ns: u64) -> Snap {
        if !self.ctx.stats.borrow().sprof.on {
            return before;
        }
        let after = self.prof_snap();
        let mask = before.iter().zip(&after).enumerate().fold(0u8, |m, (i, (a, b))| if a != b { m | (1 << i) } else { m });
        let mut st = self.ctx.stats.borrow_mut();
        let p = &mut st.sprof;
        p.seg_close();
        let src = p.cur.map_or(u8::MAX, |t| t.src);
        let prod = mask != 0;
        let e = p.agg.entry((kind as u8, src, p.path, p.role)).or_default();
        e[0] += 1;
        e[2] += ns;
        if prod {
            e[1] += 1;
            e[3] += ns;
        }
        *p.masks.entry(mask).or_default() += 1;
        let segs = p.seg_cur;
        for (k, v) in segs.iter().enumerate() {
            p.seg[usize::from(prod)][k] += v;
        }
        p.seg_cur = Default::default();
        p.path = 0;
        p.role = 0;
        after
    }

    pub(super) fn prof_end(&mut self) {
        let mut st = self.ctx.stats.borrow_mut();
        st.sprof.on = false;
        st.sprof.cur = None;
    }

    /// 调用点分段：切到段 k（剖析关着时空操作）
    pub(super) fn prof_seg(&self, k: usize) {
        let mut st = self.ctx.stats.borrow_mut();
        let p = &mut st.sprof;
        if !p.on {
            return;
        }
        p.seg_close();
        p.seg_t = Some((k, std::time::Instant::now()));
    }

    pub(super) fn prof_path(&self, path: u8) {
        let mut st = self.ctx.stats.borrow_mut();
        if st.sprof.on && st.sprof.path == 0 {
            st.sprof.path = path;
        }
    }

    /// 触发节点相对调用点来源的角色：在接收者来源 / 实参来源 / 其余
    pub(super) fn prof_role(&self, recv: &[Feed], args: &Args) {
        let mut st = self.ctx.stats.borrow_mut();
        let p = &mut st.sprof;
        if !p.on || p.role != 0 {
            return;
        }
        let has = |fs: &[Feed], n: &Node| fs.iter().any(|f| matches!(f, Feed::N(x) if x == n));
        p.role = match p.cur.and_then(|t| t.node) {
            None => ROLE_NOT_NODE,
            Some(n) if has(recv, &n) => ROLE_RECV,
            Some(n) if args.iter().flatten().any(|fs| has(fs, &n)) => ROLE_ARG,
            Some(_) => ROLE_OTHER,
        };
    }

    /// 虚调用版本未命中：本次接收者与上次记录的对照
    /// 虚调用值集未命中：本次接收者数与其中相对上次新增的个数（新增由增量合并直接得出，O(1)）
    pub(super) fn prof_vmiss(&self, total: usize, new: usize) {
        let mut st = self.ctx.stats.borrow_mut();
        let p = &mut st.sprof;
        if !p.on {
            return;
        }
        p.vmiss[0] += 1;
        p.vmiss[1] += total as u64;
        p.vmiss[2] += new as u64;
        p.vmiss[3] += u64::from(new == 0);
    }

    pub(super) fn site_prof_json(&self) -> serde_json::Value {
        use serde_json::json;
        let st = self.ctx.stats.borrow();
        let p = &st.sprof;
        if !p.enabled {
            return serde_json::Value::Null;
        }
        let trig_name = |s: u8| -> String {
            if s == u8::MAX {
                "?".into()
            } else if (s as usize) < stats::KINDS {
                format!("node:{}", stats::KIND_NAMES[s as usize])
            } else {
                TRIG_NAMES.get((s - TRIG_REOPEN) as usize).copied().unwrap_or("?").into()
            }
        };
        let mut rows: Vec<_> = p.agg.iter().collect();
        rows.sort_by(|a, b| b.1[2].cmp(&a.1[2]).then_with(|| a.0.cmp(b.0)));
        let ms = |ns: u64| ns / 1_000_000;
        // 按事件种类 / 触发源 / 路径 / 角色各自的边际汇总
        let mut by: [BTreeMap<String, [u64; 4]>; 4] = Default::default();
        for (k, v) in &p.agg {
            let keys = [
                stats::RERUN_KINDS.get(k.0 as usize).copied().unwrap_or("?").to_string(),
                trig_name(k.1),
                PATH_NAMES[k.2 as usize].to_string(),
                ROLE_NAMES[k.3 as usize].to_string(),
            ];
            for (i, key) in keys.into_iter().enumerate() {
                let e = by[i].entry(key).or_default();
                for j in 0..4 {
                    e[j] += v[j];
                }
            }
        }
        let marg = |m: &BTreeMap<String, [u64; 4]>| {
            let mut v: Vec<_> = m.iter().collect();
            v.sort_by(|a, b| b.1[2].cmp(&a.1[2]));
            v.into_iter().map(|(k, x)| json!([k, x[0], x[1], ms(x[2]), ms(x[3])])).collect::<Vec<_>>()
        };
        let mask_name = |m: u8| -> String {
            if m == 0 {
                return "none".into();
            }
            PROD_NAMES.iter().enumerate().filter(|(i, _)| m & (1 << i) != 0).map(|(_, n)| *n).collect::<Vec<_>>().join("+")
        };
        let mut masks: Vec<_> = p.masks.iter().collect();
        masks.sort_by(|a, b| b.1.cmp(a.1));
        json!({
            // 每行：[键, 次数, 有产出次数, 总 ms, 有产出 ms]
            "by_event": marg(&by[0]),
            "by_trigger": marg(&by[1]),
            "by_path": marg(&by[2]),
            "by_role": marg(&by[3]),
            "rows": rows.into_iter().take(40).map(|(k, v)| json!([
                stats::RERUN_KINDS.get(k.0 as usize).copied().unwrap_or("?"), trig_name(k.1), PATH_NAMES[k.2 as usize], ROLE_NAMES[k.3 as usize],
                v[0], v[1], ms(v[2]), ms(v[3])
            ])).collect::<Vec<_>>(),
            "masks": masks.into_iter().map(|(m, c)| json!([mask_name(*m), c])).collect::<Vec<_>>(),
            // [触发源, 入队, 合并]
            "pushes": p.pushes.iter().map(|(s, c)| json!([trig_name(*s), c[0], c[1]])).collect::<Vec<_>>(),
            // [段, 无产出 ms, 有产出 ms]
            "invoke_segs": SEG_NAMES.iter().enumerate().map(|(k, n)| json!([n, ms(p.seg[0][k]), ms(p.seg[1][k])])).collect::<Vec<_>>(),
            "vmiss": p.vmiss,
        })
    }
}

impl SiteProf {
    pub(super) fn note_push(&mut self, w: (usize, u32), src: u8, node: Option<Node>, new: bool) {
        if !self.enabled {
            return;
        }
        self.pushes.entry(src).or_default()[usize::from(!new)] += 1;
        if new {
            self.trig.insert(w, Trig { src, node });
        }
    }

    fn seg_close(&mut self) {
        if let Some((k, t)) = self.seg_t.take() {
            self.seg_cur[k] += t.elapsed().as_nanos() as u64;
        }
    }
}

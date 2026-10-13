//! 引擎：形参字符串常量集——按名查找的名字来自本方法形参时，取流到该形参的全部字符串常量。
//!
//! 子集边的存储（§4.18）：succ / pred 行**内容共享**（同形调用点的相同目标集 / 同一枢纽扇出到多目标的
//! 相同前驱集——形态画像重复度 succ 95% / pred 88%）；pred 更**惰性物化**——只建被按名求值遍历到的槽
//! （demand 槽仅占 pred 行 0.5%），首访沿 succ 全量重建、其后由 `pstr_edge` 增量维护，内容与急切双存
//! 逐项相同。
//!
//! 单调且与处理顺序无关：
//! - 集合只并不减，不走形参常量（`pvals`）的汇合格。汇合格先到的常量会被后到的调用点抬为 Top，
//!   若按汇合结果取常量，结果就取决于调用点接入的先后；
//! - 实参是字符串常量或含字面量来源的合流值：各字面量并入被调形参槽；实参来自调用方形参（透传，含合流）：登记子集边「调用方形参槽 → 被调形参槽」，
//!   常量集沿边传递（调用方形参上已有与日后新增的常量都会到达）；
//! - 派发枢纽同样有形参槽：调用点实参并入枢纽槽，枢纽槽流向父枢纽槽与各目标的形参槽；
//! - 方法形参槽增长时，读过该槽的按名查找站点入站点队列重跑（不在接边中途重入）。
//!
//! 按名取类另需形参上的**全部**名字（不止字面量）：
//! - String 形参槽另记非常量实参的调用点（调用方, 偏移, 实参序号），求值时在调用方帧里按拼接段求出名字；
//! - 有实参值未知的调用边（非字节码调用方、无调用点记录即被分析的入口、实参未知的枢纽接入）的方法 / 枢纽，
//!   其形参槽推不出（与 `pvals` 的 Top 同口径，恒不撤）；
//! - 读者沿子集边逆向遍历全部上游槽：任一推不出即整体推不出；字面量取起点槽（子集边已传递）；
//!   上游槽变化（新常量、新流入边、新非常量实参、推不出）、非常量实参所在调用方重分析时读者重跑；
//! - 递归传参成环（槽在求值栈上）或上游槽过多时推不出。
//!
//! String 字段同样有槽（`PSlot::F`，按字段、不分接收者）：字节码写入的字面量并入、写入本方法形参时登记子集边
//! 「形参槽 → 字段槽」，其余写入（拼接、调用结果等）与非常量实参同一口径登记为字段槽的输入，读者在写入方帧里
//! 按拼接段求名字（求不出即推不出）；值未知的写入使字段槽推不出；字段可经字节码外途径写入（`field_open`）时
//! 读者不取槽。读取 String 字段的名字段由此取得全部写入名字（如按类型名查找服务时，类型名存于列表对象的字段）。
//!
//! 值映射字段同样有槽（`PSlot::V`，按字段、不分接收者）：经读字段的写入入口调用点把写入值并入，口径同 String 字段槽
//! （字面量、形参子集边、其余值登记为输入，输入为调用点第 1 个实参即写入值）；读者见 `map_slot.rs`。

use super::class_lookup::{event_at, expand, Gap, Part, MAX_NAMES};
use super::name_eval::Frame;
use super::sealed::{flatten, is_field};
use super::*;

/// 字符串常量集的槽：方法形参（方法，形参槽）/ 枢纽形参（枢纽，形参序号，不含接收者）/ String 字段（字段节点序号）/
/// 值映射字段的存入值（字段节点序号）
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Debug)]
pub(super) enum PSlot {
    M(usize, usize),
    H(u32, usize),
    F(usize),
    V(usize),
}

#[derive(Default)]
pub(super) struct PStrs {
    /// 字面量序号集（`absint::lit`；传递只做位运算）
    sets: HashMap<PSlot, IdSet>,
    /// 子集边与逆：行内容共享（§4.18——形态画像 succ 行内容重复度 82–93%：同形调用点的相同目标集 /
    /// 同一枢纽扇出到多目标的相同前驱集），同内容行共享一份 `Rc<SortedSet>`，共享行插入时克隆改写后
    /// 重新并池。集合语义（插入去重 + 升序遍历）与逐行独立存储完全一致——行为与确定性不变
    succ: RowSet,
    /// 读过方法形参槽的按名查找站点（偏移）
    sites: HashMap<(usize, usize), SortedSet<u32>>,
    pred: RowSet,
    /// 流入 String 形参槽的非常量实参：(调用方, 调用偏移, 实参序号（不含接收者）)；
    /// 流入 String 字段槽的非常量写入：(写入方, putfield / putstatic 偏移, [`FIELD_VALUE`])
    inputs: HashMap<PSlot, SortedSet<(usize, u32, usize)>>,
    /// 有实参值未知的调用边的方法 / 枢纽：形参槽推不出
    top_m: HashSet<usize>,
    top_h: HashSet<u32>,
    /// 经字节码调用点以外的入口接入的方法（枢纽、引导阶段、反射调用、无调用点记录）：字段配对看不到这些入口
    offsite: HashSet<usize>,
    /// 有非常量写入的 String 字段：字段槽推不出
    top_f: HashSet<usize>,
    /// 读过槽（按名取类遍历到的上游槽）的站点
    demand: HashMap<PSlot, SortedSet<(usize, u32)>>,
    /// 求值读过其调用点实参的调用方 → 读者站点
    xdemand: HashMap<usize, SortedSet<(usize, u32)>>,
    /// 正在求值的起点槽
    active: Vec<PSlot>,
    /// 惰性逆边首访重建观测（§4.19 墙钟回归归因，只累加计数 / 时长，不改变行为）
    rebuild_n: u64,
    rebuild_rows: u64,
    rebuild_ns: u64,
}

/// 行内容共享的「槽 → 槽集合」表（§4.18，succ / pred 共用）。同内容行只存一份（`Rc` 按指针共享，
/// 内容哈希索引 + 内容比对找同类行）；行插入新元素时：唯一持有的行原地改，共享行克隆改写后重新并池
/// （并到既有同内容行则共享之）。插入返回值、去重口径、遍历升序与逐行独立的 `SortedSet` 完全一致——
/// 行为与确定性不变。索引按精确指针记账（改写前摘旧、并池后挂新），无死条目积累。
struct RowSet {
    rows: HashMap<PSlot, Rc<SortedSet<PSlot>>>,
    /// 内容哈希 → 行弱引用（碰撞经内容比对）
    ix: HashMap<u64, Vec<std::rc::Weak<SortedSet<PSlot>>>>,
    /// 热路径观测（§4.19 墙钟回归归因，只累加计数 / 时长，不改变行为）
    st: RowStat,
}

/// RowSet 热路径观测：插边 / 去重命中次数、内容哈希走过的元素数（两遍 O(行长) 的量）、
/// 唯一持有原地改与共享行分叉克隆的耗时、find_shared 并池查找耗时
#[derive(Default)]
struct RowStat {
    inserts: u64,
    dups: u64,
    hash_elems: u64,
    uniq_ns: u64,
    fork_n: u64,
    fork_ns: u64,
    find_ns: u64,
}

fn content_hash(v: &SortedSet<PSlot>) -> u64 {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    v.len().hash(&mut h);
    for e in v.iter() {
        e.hash(&mut h);
    }
    h.finish()
}

impl Default for RowSet {
    fn default() -> RowSet {
        RowSet { rows: HashMap::default(), ix: HashMap::default(), st: RowStat::default() }
    }
}

impl RowSet {
    fn get(&self, s: &PSlot) -> Option<&SortedSet<PSlot>> {
        self.rows.get(s).map(|r| &**r)
    }
    /// 在 s 行内插入 x：true = 新插入（与 `SortedSet::insert` 同口径）。
    /// 行先摘出再放回：唯一持有的行（仅本表引用）原地改，共享行克隆改写后并池
    fn insert(&mut self, s: PSlot, x: PSlot) -> bool {
        self.insert_opt(s, x, true)
    }
    /// 行已存在才插入（惰性逆边的增量维护：未物化的槽不建行）；返回是否新插入
    fn insert_existing(&mut self, s: PSlot, x: PSlot) -> bool {
        self.insert_opt(s, x, false)
    }
    /// 行是否已物化（含空行——空行也是「已按 succ 全量重建过」的标记）
    fn has_row(&self, s: &PSlot) -> bool {
        self.rows.contains_key(s)
    }
    /// 整行放入并并池（惰性逆边首访重建用；同内容行自动共享）
    fn put_row(&mut self, s: PSlot, row: SortedSet<PSlot>) {
        let rc = Rc::new(row);
        self.st.hash_elems += rc.len() as u64;
        let h = content_hash(&rc);
        let t = std::time::Instant::now();
        let shared = self.find_shared(h, &rc).unwrap_or_else(|| {
            self.ix.entry(h).or_default().push(Rc::downgrade(&rc));
            rc.clone()
        });
        self.st.find_ns += t.elapsed().as_nanos() as u64;
        self.rows.insert(s, shared);
    }
    fn insert_opt(&mut self, s: PSlot, x: PSlot, create: bool) -> bool {
        let mut rc = match self.rows.remove(&s) {
            Some(rc) => rc,
            None if create => {
                let mut set = SortedSet::default();
                set.insert(x);
                let rc = Rc::new(set);
                self.st.inserts += 1;
                self.st.hash_elems += 1;
                let h = content_hash(&rc);
                self.ix.entry(h).or_default().push(Rc::downgrade(&rc));
                self.rows.insert(s, rc);
                return true;
            }
            None => return false,
        };
        if rc.contains(&x) {
            self.st.dups += 1;
            self.rows.insert(s, rc);
            return false;
        }
        self.st.inserts += 1;
        let forked = Rc::strong_count(&rc) > 1;
        let t = std::time::Instant::now();
        let shared = if !forked {
            // 唯一持有：原地改。改前按指针摘旧索引——本表对每行至多持一个弱引用，摘除后
            // `Rc::get_mut` 才可用（weak 计数非零时 `get_mut` 返回 None）；改后按新内容并池
            self.st.hash_elems += rc.len() as u64;
            let old = content_hash(&rc);
            self.unlist(old, Rc::as_ptr(&rc));
            Rc::get_mut(&mut rc).unwrap().insert(x);
            self.st.hash_elems += rc.len() as u64;
            let new = content_hash(&rc);
            match self.find_shared(new, &rc) {
                Some(other) => other,
                None => {
                    self.ix.entry(new).or_default().push(Rc::downgrade(&rc));
                    rc
                }
            }
        } else {
            // 共享行：克隆改写后并池（rc 摘出后仍有他行引用，其索引条目保持有效）
            self.st.fork_n += 1;
            let mut v = (*rc).clone();
            v.insert(x);
            self.st.hash_elems += v.len() as u64;
            let h = content_hash(&v);
            let cand = Rc::new(v);
            match self.find_shared(h, &cand) {
                Some(other) => other,
                None => {
                    self.ix.entry(h).or_default().push(Rc::downgrade(&cand));
                    cand
                }
            }
        };
        let dt = t.elapsed().as_nanos() as u64;
        if forked {
            self.st.fork_ns += dt;
        } else {
            self.st.uniq_ns += dt;
        }
        self.rows.insert(s, shared);
        true
    }
    /// 内容哈希桶内找同内容行（排除自身指针——`&SortedSet` 即 `Rc` 数据指针，与 `Weak::as_ptr` 同基；
    /// 顺带清死引用）。找到即共享
    fn find_shared(&mut self, h: u64, want: &SortedSet<PSlot>) -> Option<Rc<SortedSet<PSlot>>> {
        let want_ptr = want as *const SortedSet<PSlot>;
        let bucket = self.ix.get_mut(&h)?;
        bucket.retain(|w| w.strong_count() > 0);
        let mut hit = None;
        for w in bucket.iter() {
            let rc = w.upgrade().unwrap();
            if Rc::as_ptr(&rc) as *const SortedSet<PSlot> != want_ptr && *rc == *want {
                hit = Some(rc);
                break;
            }
        }
        hit
    }
    /// 从内容哈希桶摘除指向 p 的条目（原地改前调用；桶通常长 1）
    fn unlist(&mut self, h: u64, p: *const SortedSet<PSlot>) {
        if let Some(bucket) = self.ix.get_mut(&h) {
            bucket.retain(|w| std::rc::Weak::as_ptr(w) as *const SortedSet<PSlot> != p);
            if bucket.is_empty() {
                self.ix.remove(&h);
            }
        }
    }
    /// 深堆字节（census deep3，只读）：行表列 + **按指针去重的相异行内容**（`Rc` 头 + 容量字节）+ 索引
    fn deep_bytes(&self) -> usize {
        let mut seen: HashSet<*const SortedSet<PSlot>> = HashSet::default();
        let mut b = self.rows.capacity() * (std::mem::size_of::<(PSlot, Rc<SortedSet<PSlot>>)>()) * 9 / 7;
        for rc in self.rows.values() {
            if seen.insert(Rc::as_ptr(rc)) {
                b += 16 + rc.cap_bytes();
            }
        }
        b + self.ix.capacity() * 40 * 9 / 7 + self.ix.values().map(|v| 24 + v.len() * 16).sum::<usize>()
    }
}

#[cfg(test)]
mod tests {
    use super::{PSlot, RowSet, SortedSet};
    use std::rc::Rc;

    fn row(v: &RowSet, s: PSlot) -> Vec<PSlot> {
        v.get(&s).into_iter().flatten().copied().collect()
    }

    /// 行内容共享的语义与逐行独立存储一致：插入去重、升序、共享 / 分叉 / 并回
    #[test]
    fn rowset_semantics() {
        let mut v = RowSet::default();
        let (a, b, c) = (PSlot::M(1, 0), PSlot::M(2, 0), PSlot::M(3, 0));
        let (x, y, z) = (PSlot::H(7, 0), PSlot::H(8, 0), PSlot::H(9, 0));
        // 去重口径与 BTreeSet::insert 相同
        assert!(v.insert(a, x));
        assert!(!v.insert(a, x));
        assert!(v.insert(a, y));
        assert!(v.insert(a, z));
        assert_eq!(row(&v, a), vec![x, y, z]);
        // 同内容行共享（同一分配）；分叉（插入新元素）后各自独立、再并回
        assert!(v.insert(b, x));
        assert!(v.insert(b, y));
        assert!(v.insert(b, z));
        assert_eq!(Rc::as_ptr(&v.rows[&a]), Rc::as_ptr(&v.rows[&b]));
        assert!(v.insert(b, PSlot::H(1, 0)));
        assert_eq!(row(&v, b), vec![PSlot::H(1, 0), x, y, z]);
        assert_eq!(row(&v, a), vec![x, y, z]); // a 不受 b 分叉影响
        assert!(v.insert(c, x));
        assert!(v.insert(c, y));
        assert!(v.insert(c, z));
        assert!(v.insert(c, PSlot::H(1, 0)));
        assert_eq!(Rc::as_ptr(&v.rows[&b]), Rc::as_ptr(&v.rows[&c])); // 分叉后同内容并回
        // 唯一持有行原地改（共享计数不破坏）
        assert!(v.insert(c, PSlot::F(5)));
        assert_eq!(row(&v, c).len(), 5);
        assert_eq!(row(&v, b).len(), 4);
    }

    /// 惰性逆边口径：未物化的槽不建行（insert_existing 无操作），put_row 后增量可见
    #[test]
    fn rowset_lazy_rows() {
        let mut v = RowSet::default();
        let (a, b) = (PSlot::M(1, 0), PSlot::M(2, 0));
        assert!(!v.has_row(&a));
        assert!(!v.insert_existing(a, b)); // 行不存在：不建
        assert!(!v.has_row(&a));
        let mut r = SortedSet::default();
        r.insert(b);
        v.put_row(a, r);
        assert!(v.has_row(&a));
        assert_eq!(row(&v, a), vec![b]);
        assert!(v.insert_existing(a, PSlot::M(3, 0)));
        assert!(!v.insert_existing(a, PSlot::M(3, 0)));
        assert_eq!(row(&v, a), vec![b, PSlot::M(3, 0)]);
        // 空行也是「已物化」标记
        let mut w = RowSet::default();
        w.put_row(PSlot::F(1), SortedSet::default());
        assert!(w.has_row(&PSlot::F(1)));
        assert!(w.insert_existing(PSlot::F(1), PSlot::M(9, 9)));
        assert_eq!(row(&w, PSlot::F(1)), vec![PSlot::M(9, 9)]);
    }
}

impl PStrs {
    /// 墙钟回归归因行（§4.19，随 `--lambda-prof` dump 打印，只读）：succ / pred 两表的插边计数
    /// （写入 / 去重命中 / 内容哈希元素数 / 原地改与分叉克隆耗时）与惰性逆边首访重建
    /// （次数 / 走过的 succ 行数 / 耗时）
    pub(super) fn perf_line(&self) -> String {
        let ms = |ns: u64| ns as f64 / 1e6;
        let f = |name: &str, st: &RowStat| {
            format!(
                "{name}(ins={} dup={} hash_elems={} uniq={:.0}ms fork={} fork={:.0}ms find={:.0}ms)",
                st.inserts, st.dups, st.hash_elems, ms(st.uniq_ns), st.fork_n, ms(st.fork_ns), ms(st.find_ns)
            )
        };
        format!(
            "{} {} rebuild(n={} rows={} t={:.0}ms)",
            f("succ", &self.succ.st),
            f("pred", &self.pred.st),
            self.rebuild_n,
            self.rebuild_rows,
            ms(self.rebuild_ns)
        )
    }

    /// 深堆字节（census deep3 行，只读；`IdSet` 经 `heap_bytes`、B 树按节点摊还）——逐子表
    pub(super) fn deep_items(&self) -> Vec<(&'static str, usize)> {        vec![
            ("pstr_sets", self.sets.capacity() * 40 * 9 / 7 + self.sets.values().map(|s| s.heap_bytes()).sum::<usize>()),
            ("pstr_succ", self.succ.deep_bytes()),
            ("pstr_pred", self.pred.deep_bytes()),
            ("pstr_sites", self.sites.capacity() * 40 * 9 / 7 + self.sites.values().map(|s| s.cap_bytes()).sum::<usize>()),
            ("pstr_inputs", self.inputs.capacity() * 40 * 9 / 7 + self.inputs.values().map(|s| s.cap_bytes()).sum::<usize>()),
            ("pstr_demand", self.demand.capacity() * 40 * 9 / 7 + self.demand.values().map(|s| s.cap_bytes()).sum::<usize>()),
            ("pstr_xdemand", self.xdemand.capacity() * 40 * 9 / 7 + self.xdemand.values().map(|s| s.cap_bytes()).sum::<usize>()),
            ("pstr_top", self.top_m.capacity() * 8 * 9 / 7 + self.top_h.capacity() * 4 * 9 / 7 + self.offsite.capacity() * 8 * 9 / 7 + self.top_f.capacity() * 8 * 9 / 7 + self.active.capacity() * 16),
        ]
    }
}

/// 边集形态画像开关（`RAVA_PSTR_SHAPE=1`，只读诊断；逐 dump 全量走边有秒级代价，验收运行不开）
pub(super) static SHAPE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
pub(super) fn shape_on() -> bool {
    *SHAPE.get_or_init(|| std::env::var("RAVA_PSTR_SHAPE").ok().as_deref() == Some("1"))
}

/// 非常量输入为字段写入值（而非调用点实参）时的序号
const FIELD_VALUE: usize = usize::MAX;

/// 读者遍历的上游槽数上限：超出按推不出处理
const MAX_SLOTS: usize = 256;
/// 形参名字求值的嵌套层数上限（调用方实参又来自其形参……）
const MAX_NEST: usize = 4;

/// 实参值是否不全由字面量与形参透传构成（需在调用方帧里求值）
fn computed(v: &V) -> bool {
    match v {
        V::Str(..) | V::Null => false,
        V::Ref { src, .. } => src.is_empty() || src.iter().any(|s| !matches!(s, Src::Param(_) | Src::Str(_))),
        _ => true,
    }
}

impl<'a> Engine<'a> {
    /// 方法 m 形参槽 i 上的字符串常量（按字符串序）；登记站点 (m, off) 为读者
    pub(super) fn pstr_read(&mut self, m: usize, i: usize, off: u32) -> Vec<Rc<str>> {
        self.pstr.sites.entry((m, i)).or_default().insert(off);
        let mut out: Vec<Rc<str>> = self.pstr.sets.get(&PSlot::M(m, i)).into_iter().flatten().map(crate::absint::lit_str).collect();
        out.sort_unstable();
        out
    }

    /// 读过方法形参槽的按名查找站点（形参槽被污染时重跑，engine/field_names.rs）
    pub(super) fn pstr_readers(&self, m: usize, i: usize) -> Vec<u32> {
        self.pstr.sites.get(&(m, i)).into_iter().flatten().copied().collect()
    }

    /// 槽的常量集（升序字面量序号；克隆等价前置签名用，`eqmerge.rs`）
    pub(super) fn pstr_set_ids(&self, s: PSlot) -> Option<Vec<u32>> {
        self.pstr.sets.get(&s).map(|xs| {
            let mut v: Vec<u32> = xs.iter().collect();
            v.sort_unstable();
            v
        })
    }

    /// 槽 s 的子集边后继中记污染的槽（方法 / 枢纽形参槽）
    pub(super) fn pstr_succ_slots(&self, s: PSlot) -> Vec<PSlot> {
        self.pstr.succ.get(&s).into_iter().flatten().copied().filter(|u| matches!(u, PSlot::M(..) | PSlot::H(..))).collect()
    }

    /// 常量并入槽 at，沿子集边传递（只传新增部分）。方法形参槽的增长锁步扇出到等价组成员（`eqmerge.rs`）
    fn pstr_add(&mut self, at: PSlot, strs: IdSet) {
        let mut work = vec![(at, strs.clone())];
        if let PSlot::M(t, i) = at {
            for u in self.eqm.mates.get(&t).into_iter().flatten().copied().collect::<Vec<_>>() {
                if u != t {
                    work.push((PSlot::M(u, i), strs.clone()));
                }
            }
        }
        while let Some((s, xs)) = work.pop() {
            let set = self.pstr.sets.entry(s).or_default();
            let new = xs.minus(set);
            if new.is_empty() {
                continue;
            }
            set.union_with(&new);
            self.pstr_wake(s);
            if let PSlot::M(t, i) = s {
                let offs: Vec<u32> = self.pstr.sites.get(&(t, i)).into_iter().flatten().copied().collect();
                for off in offs {
                    self.push_site((t, off), site_prof::TRIG_PSTR, None);
                }
            }
            for &n in self.pstr.succ.get(&s).into_iter().flatten() {
                work.push((n, new.clone()));
            }
        }
    }

    /// 子集边 from → to：from 已有的常量立即传递
    pub(super) fn pstr_edge(&mut self, from: PSlot, to: PSlot) {
        if from == to || !self.pstr.succ.insert(from, to) {
            return;
        }
        // 惰性逆边（§4.18）：pred 只物化被读者遍历到的槽（形态画像 demand 槽仅占 pred 行的 0.5%，
        // 3MiB/1013MiB @sb3 1224s）——未物化的槽不建行，首访时沿 succ 全量重建
        self.pstr.pred.insert_existing(to, from);
        self.pstr_wake(to);
        // 污染与常量同沿子集边：新边的来源槽已污染时目标槽随之污染
        if self.slot_tainted(from) {
            self.taint_slot(to, super::field_names::TaintWhy::Pass(from));
        }
        let xs = self.pstr.sets.get(&from).cloned().unwrap_or_default();
        if !xs.is_empty() {
            self.pstr_add(to, xs);
        }
    }

    /// 调用方 m 偏移 off 的调用点实参值 vals（不含接收者）流入槽 to(j)（None = 该实参不入形参槽）；
    /// string(j) = 槽 j 是 String 形参
    pub(super) fn pstr_site(&mut self, m: usize, off: u32, vals: &[V], to: impl Fn(usize) -> Option<PSlot>, string: impl Fn(usize) -> bool) {
        for (j, v) in vals.iter().enumerate() {
            let Some(slot) = to(j) else { continue };
            if string(j) && computed(v) && self.pstr.inputs.entry(slot).or_default().insert((m, off, j)) {
                self.pstr_wake(slot);
            }
            let lits = v.lit_ids();
            if !lits.is_empty() {
                self.pstr_add(slot, lits.into_iter().collect());
            }
            if let V::Ref { src, .. } = v {
                for s in src.iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), slot);
                    }
                }
            }
        }
    }

    /// 读过槽 s 的按名取类站点入站点队列重跑
    fn pstr_wake(&mut self, s: PSlot) {
        let Some(ws) = self.pstr.demand.get(&s) else { return };
        for w in ws.iter().copied().collect::<Vec<_>>() {
            self.push_site(w, site_prof::TRIG_PSTR_WAKE, None);
        }
    }

    /// 方法 t 有实参值未知的调用边（或无调用点记录即被分析）：形参槽推不出。锁步扇出到等价组成员（`eqmerge.rs`）
    pub(super) fn pstr_top_m(&mut self, t: usize) {
        if self.pstr.top_m.insert(t) {
            for i in 0..self.methods[t].ptypes.len() {
                self.pstr_wake(PSlot::M(t, i));
            }
        }
        for u in self.eqm.mates.get(&t).into_iter().flatten().copied().collect::<Vec<_>>() {
            if u != t && self.pstr.top_m.insert(u) {
                for i in 0..self.methods[u].ptypes.len() {
                    self.pstr_wake(PSlot::M(u, i));
                }
            }
        }
    }

    /// 方法 t 经字节码调用点以外的入口接入：包装配对（`lookup_pair.rs`）只在字节码调用点上配对，这类方法形参上的
    /// 名字按形参常量集与污染口径取（engine/field_names.rs）；首次登记时重跑读过其形参槽的站点
    pub(super) fn pstr_offsite(&mut self, t: usize) {
        if self.pstr.offsite.insert(t) {
            for i in 0..self.methods[t].ptypes.len() {
                for off in self.pstr_readers(t, i) {
                    self.push_site((t, off), site_prof::TRIG_TAINT, None);
                }
            }
        }
        // 锁步扇出（`eqmerge.rs` 等价组）
        for u in self.eqm.mates.get(&t).into_iter().flatten().copied().collect::<Vec<_>>() {
            if u != t && self.pstr.offsite.insert(u) {
                for i in 0..self.methods[u].ptypes.len() {
                    for off in self.pstr_readers(u, i) {
                        self.push_site((u, off), site_prof::TRIG_TAINT, None);
                    }
                }
            }
        }
    }

    pub(super) fn pstr_is_offsite(&self, t: usize) -> bool {
        self.pstr.offsite.contains(&t)
    }

    /// 枢纽 h 有实参值未知的调用点接入：形参槽推不出
    pub(super) fn pstr_top_h(&mut self, h: u32) {
        if self.pstr.top_h.insert(h) {
            for j in 0..self.hubs[h as usize].ptypes.len() {
                self.pstr_wake(PSlot::H(h, j));
            }
        }
    }

    /// 调用方 m 重分析：读过其调用点实参的读者站点重跑
    pub(super) fn pstr_reanalyzed(&mut self, m: usize) {
        let Some(ws) = self.pstr.xdemand.get(&m) else { return };
        for w in ws.iter().copied().collect::<Vec<_>>() {
            self.push_site(w, site_prof::TRIG_PSTR_REANALYZED, None);
        }
    }

    /// 方法 m 偏移 off 写入 String 字段（字段节点 fi）的值 v（None = 值未知）并入字段槽。非常量写入有字节码写入点时
    /// 与非常量实参同一口径登记为槽的输入（读者在写入方帧里按拼接段求名字），否则（具体求值物化）槽推不出
    pub(super) fn pstr_field_put(&mut self, m: usize, off: Option<u32>, fi: usize, v: Option<&V>) {
        let slot = PSlot::F(fi);
        match v {
            Some(V::Null) => {}
            Some(v) if !computed(v) => {
                let lits = v.lit_ids();
                if !lits.is_empty() {
                    self.pstr_add(slot, lits.into_iter().collect());
                }
                for s in v.srcs().iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), slot);
                    }
                }
            }
            Some(v) if off.is_some() => {
                let lits = v.lit_ids();
                if !lits.is_empty() {
                    self.pstr_add(slot, lits.into_iter().collect());
                }
                if self.pstr.inputs.entry(slot).or_default().insert((m, off.unwrap_or_default(), FIELD_VALUE)) {
                    self.pstr_wake(slot);
                }
            }
            _ => {
                if self.pstr.top_f.insert(fi) {
                    self.pstr_wake(slot);
                }
            }
        }
    }

    /// 方法 m 偏移 off 的写入入口调用把值 v（调用点第 1 个实参，不含接收者）存入值映射字段（字段节点 fi）：并入值映射槽。
    /// 字面量并入、形参透传登记子集边，其余值登记为槽的输入（读者在写入方帧里按拼接段求名字）
    pub(super) fn pstr_map_put(&mut self, m: usize, off: u32, fi: usize, v: &V) {
        let slot = PSlot::V(fi);
        let lits = v.lit_ids();
        if !lits.is_empty() {
            self.pstr_add(slot, lits.into_iter().collect());
        }
        match v {
            V::Null | V::Str(..) => {}
            v if !computed(v) => {
                for s in v.srcs().iter() {
                    if let Src::Param(i) = s {
                        self.pstr_edge(PSlot::M(m, *i as usize), slot);
                    }
                }
            }
            _ => {
                if self.pstr.inputs.entry(slot).or_default().insert((m, off, 1)) {
                    self.pstr_wake(slot);
                }
            }
        }
    }

    /// 值映射字段（字段节点 fi）经读字段写入的全部名字与是否推得出（当前站点为读者）
    pub(super) fn map_slot_names(&mut self, fi: usize, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        self.slot_names(PSlot::V(fi), depth)
    }

    /// 方法 m 的 String 形参槽 i 上的名字与是否推得出（当前站点为读者）；None = 非 String 形参 / 不在站点内
    pub(super) fn param_names(&mut self, m: usize, i: usize, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let string = self.id(STRING);
        if self.methods[m].ptypes.get(i).copied().flatten() != Some(string) {
            return None;
        }
        self.slot_names(PSlot::M(m, i), depth)
    }

    /// String 字段（字段节点 fi）各字节码写入的名字与是否推得出（当前站点为读者）
    pub(super) fn field_names(&mut self, fi: usize, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        self.slot_names(PSlot::F(fi), depth)
    }

    /// 名字段读自 String 字段（方法 m 中偏移 o 的 getfield / getstatic）时该字段各字节码写入的全部名字。
    /// static final 字段由常量求值给出，不经字段槽；字段可经字节码外途径写入时推不出——仍给出字节码写入的名字，
    /// 只标记推不出（字段转为不折叠时 m 失效重分析）
    pub(super) fn read_field_names(&mut self, m: usize, a: &Analysis, o: u32, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let Some(Event::Field { opcode, mref, .. }) = event_at(a, o, is_field) else { return None };
        if !matches!(*opcode, classfile::op::GETFIELD | classfile::op::GETSTATIC) || mref.desc != format!("L{STRING};") {
            return None;
        }
        let fi = self.ctx.field_info(mref)?;
        if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
            return None;
        }
        self.ctx.dep(m, Dep::Field(fi.key.clone()));
        let open = self.ctx.field_open(&fi);
        let n = match self.fields.get_index_of(&fi.key) {
            Some(n) => n,
            None if open => return self.cur_site.map(|_| (BTreeSet::new(), false)),
            None => self.field_node(fi.key.clone()),
        };
        self.field_names(n, depth).map(|(names, complete)| (names, complete && !open))
    }

    /// 槽 start 上的全部名字：沿子集边逆向遍历上游槽，字面量取起点槽，非常量实参在调用方帧里求值
    fn slot_names(&mut self, start: PSlot, depth: u8) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let (reader, inputs, complete) = self.slot_upstream(start)?;
        let mut out: BTreeSet<Rc<str>> = self.pstr.sets.get(&start).into_iter().flatten().map(crate::absint::lit_str).collect();
        self.pstr.active.push(start);
        let evaluated = self.param_inputs(reader, &inputs, depth, &mut out);
        self.pstr.active.pop();
        Some((out, complete && evaluated))
    }

    /// 方法 m 的 String 形参槽 i 上的全部候选模式（推不出的段记为任意串，见 `class_lookup.rs::Gap::Class`；
    /// 当前站点为读者）；None = 槽推不出或模式超出上限
    pub(super) fn param_patterns(&mut self, m: usize, i: usize) -> Option<Vec<Vec<Part>>> {
        let string = self.id(STRING);
        if self.methods[m].ptypes.get(i).copied().flatten() != Some(string) {
            return None;
        }
        let start = PSlot::M(m, i);
        let (reader, inputs, complete) = self.slot_upstream(start)?;
        if !complete {
            return None;
        }
        let mut out: Vec<Vec<Part>> = self.pstr.sets.get(&start).into_iter().flatten().map(|l| vec![Part::Lit(crate::absint::lit_str(l))]).collect();
        self.pstr.active.push(start);
        let r = self.input_patterns(reader, &inputs, &mut out);
        self.pstr.active.pop();
        r.map(|_| out)
    }

    /// 槽 start 的读者登记与上游遍历：返回读者站点、全部上游槽的非常量实参与槽是否推得出（任一上游槽推不出、
    /// 成环或超限即推不出）。推不出只作标记、不截断遍历：名字是状态的单调函数——上游槽转为推不出后，
    /// 其更上游的字面量（已沿子集边到达起点槽）与非常量实参照样计入，结果只并不减，与求值先后无关。
    /// 当前不在站点内（无读者可登记）为 None
    fn slot_upstream(&mut self, start: PSlot) -> Option<((usize, u32), BTreeSet<(usize, u32, usize)>, bool)> {
        let reader = self.cur_site?;
        let mut inputs: BTreeSet<(usize, u32, usize)> = BTreeSet::new();
        if self.pstr.active.contains(&start) || self.pstr.active.len() >= MAX_NEST {
            if !self.pstr.active.contains(&start) {
                stats::cap_hit(stats::CAP_NEST);
            }
            return Some((reader, inputs, false));
        }
        let mut complete = true;
        let mut seen: HashSet<PSlot> = HashSet::default();
        let mut stack = vec![start];
        while let Some(s) = stack.pop() {
            if !seen.insert(s) {
                continue;
            }
            if seen.len() > MAX_SLOTS {
                stats::cap_hit(stats::CAP_SLOTS);
                // 超限：遍历到的子集取决于图的形状，非常量实参一概不取（起点槽字面量已含全部上游字面量）
                inputs.clear();
                complete = false;
                break;
            }
            self.pstr.demand.entry(s).or_default().insert(reader);
            // 惰性逆边：槽首被读者遍历到时沿 succ 全量重建其前驱行（其后由 `pstr_edge` 增量维护）；
            // 重建结果为确定的全集（逐行二分收集后排序插入），与急切双存的内容逐项相同
            if !self.pstr.pred.has_row(&s) {
                let t = std::time::Instant::now();
                let succ_rows = self.pstr.succ.rows.len() as u64;
                let froms: Vec<PSlot> = self.pstr
                    .succ
                    .rows
                    .iter()
                    .filter(|(_, rc)| rc.may_contain(&s) && rc.contains(&s))
                    .map(|(f, _)| *f)
                    .collect();
                let mut row = SortedSet::default();
                for f in froms {
                    row.insert(f);
                }
                self.pstr.pred.put_row(s, row);
                let p = &mut self.pstr;
                p.rebuild_n += 1;
                p.rebuild_rows += succ_rows;
                p.rebuild_ns += t.elapsed().as_nanos() as u64;
            }
            let top = match s {
                PSlot::M(t, _) => self.pstr.top_m.contains(&t),
                PSlot::H(h, _) => self.pstr.top_h.contains(&h),
                PSlot::F(fi) => self.pstr.top_f.contains(&fi),
                PSlot::V(_) => false,
            };
            complete &= !top;
            inputs.extend(self.pstr.inputs.get(&s).into_iter().flatten().copied());
            stack.extend(self.pstr.pred.get(&s).into_iter().flatten().copied());
        }
        Some((reader, inputs, complete))
    }

    /// 非常量实参所在调用点的实参值与调用方帧数据；调用方待重分析 / 调用点已不可达为 Ok(None)，保守分析为 Err
    fn input_value(&mut self, reader: (usize, u32), (cm, off, j): (usize, u32, usize)) -> Result<Option<(Rc<Analysis>, V)>, ()> {
        self.pstr.xdemand.entry(cm).or_default().insert(reader);
        let Some(ca) = self.methods[cm].analysis.clone() else { return Ok(None) };
        if ca.conservative {
            return Err(());
        }
        // 调用点实参不含接收者；lambda 捕获值为创建点 indy 的实参（`lambda_vals.rs`）；字段槽的输入为写入值
        let v = match event_at(&ca, off, |e| matches!(e, Event::Invoke { .. } | Event::Indy { .. } | Event::Field { .. })) {
            Some(Event::Invoke { opcode, args, .. }) if j != FIELD_VALUE => args.get(usize::from(*opcode != classfile::op::INVOKESTATIC) + j),
            Some(Event::Indy { args, .. }) if j != FIELD_VALUE => args.get(j),
            Some(Event::Field { opcode: classfile::op::PUTFIELD | classfile::op::PUTSTATIC, value, .. }) if j == FIELD_VALUE => value.as_ref(),
            _ => return Ok(None),
        };
        let v = v.ok_or(())?.clone();
        Ok(Some((ca, v)))
    }

    /// 非常量实参在各自调用方帧里按 `Gap::Class` 拆段、展开成候选模式并入 out
    fn input_patterns(&mut self, reader: (usize, u32), inputs: &BTreeSet<(usize, u32, usize)>, out: &mut Vec<Vec<Part>>) -> Option<()> {
        for &inp in inputs {
            let Some((ca, v)) = self.input_value(reader, inp).ok()? else { continue };
            let owner = self.methods[inp.0].key.owner.clone();
            let f = Frame { m: Some(inp.0), a: &ca, owner: &owner, up: None };
            let parts = self.name_parts(&f, &v, Gap::Class, 0)?;
            out.extend(expand(&parts)?);
            if out.len() > MAX_NAMES {
                stats::cap_hit(stats::CAP_PNAMES);
                return None;
            }
        }
        Some(())
    }

    /// 非常量实参在各自调用方帧里求出的名字并入 out；返回是否全部求出（求不出的实参只作标记，其余照常并入）
    fn param_inputs(&mut self, reader: (usize, u32), inputs: &BTreeSet<(usize, u32, usize)>, depth: u8, out: &mut BTreeSet<Rc<str>>) -> bool {
        let mut complete = true;
        for &(cm, off, j) in inputs {
            // 调用方正待重分析（重分析后重跑）/ 调用点在当前分析里已不可达（不再流入）：跳过
            let (ca, v) = match self.input_value(reader, (cm, off, j)) {
                Ok(Some(x)) => x,
                Ok(None) => continue,
                Err(()) => {
                    complete = false;
                    continue;
                }
            };
            // 服务实现类的反射构造点（`[jca] instantiation_hosts`）把类名字段传给被调方的按名取类（如经类加载器
            // 按名加载）：所指类同宿主内的按名取类，由 JCA 规则按被请求的算法补种，不按类名字段的字符串集解析——
            // 否则注册表里全部算法的实现类都经被调方站点按名入链。该实参按推不出处理
            if self.lookup_hosts.contains(&self.methods[cm].key) {
                complete = false;
                continue;
            }
            let owner = self.methods[cm].key.owner.clone();
            let f = Frame { m: Some(cm), a: &ca, owner: &owner, up: None };
            // 内部求值（Gap::Fail）的推不出只记在引擎级标志上：在这里收进本槽的「是否推得出」，不外泄给外层求值——
            // 外层按自己的 gap 处理推不出的槽（按名取类为「已知名字 | 任意串」，按生成范围内的类名匹配），
            // 而不是因上游某个调用方推不出把整个站点记为推不出
            let saved = (std::mem::take(&mut self.lookup_incomplete), std::mem::take(&mut self.lookup_partial));
            let r = self.name_parts(&f, &v, Gap::Fail, depth).as_deref().and_then(flatten);
            let inner = std::mem::replace(&mut self.lookup_incomplete, saved.0) | std::mem::replace(&mut self.lookup_partial, saved.1);
            match r {
                Some(names) => {
                    out.extend(names);
                    complete &= !inner;
                }
                None => complete = false,
            }
        }
        if out.len() > MAX_NAMES {
            stats::cap_hit(stats::CAP_PNAMES);
            return false;
        }
        complete
    }

    /// pstr 边集形态画像（§4.18 选型证据，`RAVA_PSTR_SHAPE=1` 时随 `--lambda-prof` dump 打印，只读）：
    /// succ / pred 逐表——行数 / 边数（按槽类与「类×类」分解）、行大小分布、**后继表内容重复度**
    /// （哈希去重后的相弧行数与摊平字节——共享后继表的收益上限）、H 槽行按枢纽聚合的目标方法集一致性
    /// （按枢纽存目标表的可派生性）、pred 行里已入 demand 的槽占比（惰性逆边的收益上限）。
    /// 哈希按内容（PSlot::hash），碰撞概率对画像量级可忽略；打印前列排序，可复现
    pub(super) fn pstr_shape_dump(&self, tag: &str) {
        use std::hash::{Hash, Hasher};
        let p = &self.pstr;
        let mib = |b: usize| b as f64 / 1048576.0;
        let label = |s: &PSlot| match s {
            PSlot::M(t, i) => format!("M{t}.{i}"),
            PSlot::H(h, j) => format!("H{h}.{j}"),
            PSlot::F(f) => format!("F{f}"),
            PSlot::V(f) => format!("V{f}"),
        };
        // 行大小分桶：[1,1] [2,4] [5,16] [17,64] [65,256] [257,1024] [1025,∞)
        let bucket = |n: usize| n.min(7).checked_sub(1).map_or(6, |x| match x {
            0 => 0,
            1..=3 => 1,
            4..=15 => 2,
            16..=63 => 3,
            64..=255 => 4,
            256..=1023 => 5,
            _ => 6,
        });
        let analyze = |name: &str, t: &RowSet| {
            let kidx = |s: &PSlot| match s {
                PSlot::M(..) => 0,
                PSlot::H(..) => 1,
                PSlot::F(..) => 2,
                _ => 3,
            };
            let mut rows = 0usize;
            let mut edges = 0usize;
            let mut by_kind = [0usize; 4]; // 行按 from 类
            let mut pair = [[0usize; 4]; 4]; // 边按 from 类 × to 类
            let mut b_rows = [0usize; 7];
            let mut b_edges = [0usize; 7];
            let mut uniq: HashMap<u64, (usize, usize)> = HashMap::default(); // 内容哈希 → (len, 出现次数)
            let mut hub_j: HashMap<u32, Vec<(usize, std::collections::BTreeSet<usize>)>> = HashMap::default(); // H 行：hub → 各 j 的目标方法集
            let mut top: Vec<(usize, String)> = Vec::new();
            for (s, rc) in &t.rows {
                let v: &SortedSet<PSlot> = rc;
                let n = v.len();
                rows += 1;
                edges += n;
                by_kind[kidx(s)] += 1;
                b_rows[bucket(n)] += 1;
                b_edges[bucket(n)] += n;
                let fi = kidx(s);
                let mut h = std::collections::hash_map::DefaultHasher::new();
                n.hash(&mut h);
                for x in v.iter() {
                    x.hash(&mut h);
                    pair[fi][kidx(x)] += 1;
                }
                let e = uniq.entry(h.finish()).or_insert((n, 0));
                e.1 += 1;
                if let PSlot::H(hu, j) = s {
                    hub_j.entry(*hu).or_default().push((*j, v.iter().filter_map(|x| match x {
                        PSlot::M(t, _) => Some(*t),
                        _ => None,
                    }).collect()));
                }
                top.push((n, label(s)));
            }
            let distinct = uniq.len();
            let uniq_edges: usize = uniq.values().map(|(n, _)| n).sum();
            let dup_rows: usize = uniq.values().map(|(_, c)| c - 1).sum();
            let dup_edges = edges - uniq_edges;
            top.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
            let tops = top.iter().take(5).map(|(n, l)| format!("{l}:{n}")).collect::<Vec<_>>().join(" ");
            // H 行按枢纽：同一枢纽各 j 的目标方法集是否一致（一致 ⟹ 按枢纽存一份目标表可派生各行）
            let mut hubs_same = 0usize;
            let mut hubs_diff = 0usize;
            let mut edges_same = 0usize;
            let mut edges_diff = 0usize;
            for (_, js) in hub_j.iter() {
                let base = js.first().map(|(_, m)| m);
                let same = js.iter().all(|(_, m)| Some(m) == base);
                let e: usize = js.iter().map(|(_, m)| m.len()).sum();
                if same {
                    hubs_same += 1;
                    edges_same += e;
                } else {
                    hubs_diff += 1;
                    edges_diff += e;
                }
            }
            eprintln!(
                "{tag} pstr-shape {name} rows={rows} edges={edges} bytes~{:.0}MiB rows_by_kind(M/H/F/V)={by_kind:?} \
                 edges_pair(fromM->M/H/F/V|fromH->M/H/F/V)=[{:?}|{:?}] \
                 bucket_rows(=1/2-4/5-16/17-64/65-256/257-1k/>1k)={b_rows:?} bucket_edges={b_edges:?} \
                 uniq_rows={distinct} uniq_edges={uniq_edges} dup_rows={dup_rows} dup_edges={dup_edges} intern_save={:.0}MiB \
                 hub_j_same={hubs_same}({edges_same}e) hub_j_diff={hubs_diff}({edges_diff}e) top={tops}",
                mib(edges * std::mem::size_of::<PSlot>() + rows * 40 * 9 / 7),
                pair[0], pair[1],
                mib(dup_edges.saturating_mul(std::mem::size_of::<PSlot>()).saturating_sub(distinct * 32)),
            );
            if name == "pred" {
                // 惰性逆边收益：pred 行中键已入 demand（按名求值遍历到过）的槽的行字节占比
                let mut hot = 0usize;
                let mut hot_rows = 0usize;
                for (s, rc) in &t.rows {
                    if p.demand.contains_key(s) {
                        hot += rc.cap_bytes();
                        hot_rows += 1;
                    }
                }
                let all: usize = t.rows.values().map(|rc| rc.cap_bytes()).sum();
                eprintln!(
                    "{tag} pstr-shape {name} demand_slots={} hot_rows={}/{} hot_bytes={:.0}MiB/{:.0}MiB",
                    p.demand.len(), hot_rows, rows, mib(hot), mib(all),
                );
            }
        };
        analyze("succ", &p.succ);
        analyze("pred", &p.pred);
    }
}

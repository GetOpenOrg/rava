//! 引擎：汇集节点——同一槽位（字段 / 数组元素）、同一分配点集合的读取（或写入）站点共用一个汇集节点。
//!
//! 字节码字段站点按接收者抽象对象拆分：读站点每个对象的字段节点 `O(o, f)` 各接一条边到站点结果，
//! 写站点的写入值各接一条边到每个对象的字段节点。数组读站点同理：每个数组分配点的元素节点 `E(x, p)` 各接一条边到站点。
//! 同一批对象（如某容器节点类的全部分配点、某类数组的全部分配点）被大量站点（各克隆上下文里的同一读写）访问时，
//! 边数是站点数 × 对象数，每次增量都要在全部边上重复推送。
//!
//! 汇集节点 `G(g)` 按 (槽位, 方向, 对象集合) 共享：
//! - 读向：各对象的槽位节点流入它，它再流向各站点结果；
//! - 写向：各站点写入值流入它，它再流向各对象字段节点（只含写向站点的写入值，均应到达集合内全部对象）。
//!
//! 字段槽位的过滤类型都是字段类型；数组元素槽位（只有读向）各元素节点以 Object 流入、汇集节点按站点静态分量类型流出。
//! 过滤逐元素进行，于是到达的集合与逐对象接边相同。站点的对象集合只增不减（重跑只带来新增对象）：
//! 集合增长时换接新集合的汇集节点，新旧节点间按子集关系接边——读向「旧 → 新」、写向「新 → 旧」，
//! 其余只接新增对象。对象少于 `GATHER_MIN` 的站点仍逐对象接边。

use super::*;

/// 站点累计对象数达到此数时改经汇集节点
const GATHER_MIN: usize = 4;

/// 无汇集节点（站点仍逐对象接边）
const NO_GATHER: u32 = u32::MAX;

/// 汇集的槽位：字段（字段序号）/ 数组元素（下标奇偶）
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(super) enum Slot {
    Field(usize),
    Elem(u8),
}

/// 对象集元素哈希（可交换和式内容哈希的元素项；与 `setstore.rs` 同式）
fn u32_elem_hash(x: u32) -> u64 {
    let m = u128::from(u64::from(x).wrapping_add(1)) * 0x9e37_79b9_7f4a_7c15;
    (m as u64) ^ (m >> 64) as u64
}

/// 对象集差分链节点（§4.20 第二层压缩）：`parent` 内容 ⊂ 本节点内容，`delta` 为新增（升序、
/// 与 parent 内容不相交——沿链各 delta 两两不相交，物化 = d 路归并无去重）。len / hash（元素哈希
/// 之和）随节点存储，并入子节点时 O(|delta|) 增量更新。
///
/// 选型证据（g1c11-prof，sb3 死前 4544s）：驻留后相异内容 2357 MiB / 617M 元素中 **97.2% 住在
/// 「仅键集持有」的链中间步**（站点已换接更大集合、`gather_ids` 键仍持有每步物化的整集）；
/// **Σ|delta| 仅 15.8M 元素（链式表征 ~70 MiB，−97%）**、字典序前缀共享 88%（trie 上界 278 MiB）、
/// 区间表征无收益（fill 0.044、run 数 ≈ 元素数之半）——单调增长前缀链是唯一同时大幅优于
/// 数组 / 区间的形态。delta 直方图主体 ≤4（70k/115k），平均链深 ~40。
pub(super) struct GSet {
    parent: Option<Rc<GSet>>,
    delta: Box<[u32]>,
    len: u32,
    hash: u64,
}

impl GSet {
    /// delta 元素数（census deep3 只读）
    pub(super) fn delta_len(&self) -> usize {
        self.delta.len()
    }
    /// 父节点（census deep3 只读）
    pub(super) fn parent_rc(&self) -> Option<&Rc<GSet>> {
        self.parent.as_ref()
    }
}

/// 链内容物化（升序）：沿祖先收集 delta 做 d 路归并（两两不相交，无去重）。用于
/// `gather_step` 的归并 / 内容判等兜底 / 画像走查
fn materialize(node: &Rc<GSet>, out: &mut Vec<u32>) {
    use std::collections::BinaryHeap;
    use std::cmp::Reverse;
    let mut chains: Vec<&[u32]> = Vec::new();
    let mut cur: Option<&Rc<GSet>> = Some(node);
    while let Some(n) = cur {
        if !n.delta.is_empty() {
            chains.push(&n.delta);
        }
        cur = n.parent.as_ref();
    }
    out.clear();
    if chains.is_empty() {
        return;
    }
    let mut heap = chains
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.first().map(|&x| Reverse((x, i))))
        .collect::<BinaryHeap<_>>();
    let mut pos = vec![0usize; chains.len()];
    while let Some(Reverse((x, i))) = heap.pop() {
        out.push(x);
        pos[i] += 1;
        if pos[i] < chains[i].len() {
            heap.push(Reverse((chains[i][pos[i]], i)));
        }
    }
}

/// 候选链内容与查询序列（prev_m 与 delta 两段升序且不相交）的流式比对：候选沿祖先 delta 做
/// d 路归并（不物化），查询侧两指针走 prev_m / delta——等长前提下逐元素相等即内容相等。
/// intern 的异链兜底路径（哈希桶内碰撞 / 跨链同内容）
fn chain_eq(node: &Rc<GSet>, prev_m: &[u32], delta: &[u32]) -> bool {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;
    if node.len as usize != prev_m.len() + delta.len() {
        return false;
    }
    let mut chains: Vec<&[u32]> = Vec::new();
    let mut cur: Option<&Rc<GSet>> = Some(node);
    while let Some(n) = cur {
        if !n.delta.is_empty() {
            chains.push(&n.delta);
        }
        cur = n.parent.as_ref();
    }
    let mut heap = chains
        .iter()
        .enumerate()
        .filter_map(|(i, c)| c.first().map(|&x| Reverse((x, i))))
        .collect::<BinaryHeap<_>>();
    let mut pos = vec![0usize; chains.len()];
    let (mut pi, mut di) = (0usize, 0usize);
    while let Some(Reverse((x, i))) = heap.pop() {
        // 查询侧下一个元素 = min(prev_m[pi], delta[di])
        let q = if pi < prev_m.len() && (di >= delta.len() || prev_m[pi] <= delta[di]) {
            let q = prev_m[pi];
            pi += 1;
            q
        } else {
            let q = delta[di];
            di += 1;
            q
        };
        if x != q {
            return false;
        }
        pos[i] += 1;
        if pos[i] < chains[i].len() {
            heap.push(Reverse((chains[i][pos[i]], i)));
        }
    }
    pi == prev_m.len() && di == delta.len()
}

/// 对象集内容驻留池（§4.19；§4.20 改差分链节点）：同内容的活节点只此一份——站点累计集
/// （`gather_last` 族）与汇集节点键（`gather_ids`）共享同一节点。哈希为元素哈希之和（并入
/// 时增量），桶内按 (len, 同 parent 同 delta 快路径, 物化兜底) 判等（碰撞安全）；弱引用记账，
/// 死节点随桶清理出表。只影响存储共享，不影响任何集合内容
#[derive(Default)]
pub(super) struct SetPool {
    /// 内容哈希 → 活节点弱引用
    ix: HashMap<u64, Vec<std::rc::Weak<GSet>>>,
    /// 空集正规节点（每引擎一份；`gather_step` 无 last 时的 prev）
    empty: Option<Rc<GSet>>,
    /// 观测：驻留命中（复用既有节点）/ 新登记
    pub(super) hits: u64,
    pub(super) news: u64,
    /// 画像（§4.20）：新登记时的链式增量 |full∖prev|——Σ|delta| 即差分链表示的元素量；
    /// 直方图分桶 ≤1/≤4/≤16/≤64/≤256/>256
    pub(super) delta_sum: u64,
    pub(super) delta_hist: [u64; 6],
}

/// gathers 相异内容形态画像开关（`RAVA_GATHERS_SHAPE=1`，只读诊断；逐 dump 全量走集有秒级代价，验收运行不开）
pub(super) fn shape_on() -> bool {
    static SHAPE: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *SHAPE.get_or_init(|| std::env::var("RAVA_GATHERS_SHAPE").ok().as_deref() == Some("1"))
}

impl SetPool {
    /// 空集正规节点（缓存）
    pub(super) fn empty(&mut self) -> Rc<GSet> {
        self.empty
            .get_or_insert_with(|| {
                Rc::new(GSet { parent: None, delta: Box::from([]), len: 0, hash: 0 })
            })
            .clone()
    }

    /// prev ∪ delta 驻留（delta 升序且与 prev 内容不相交、prev_m 为 prev 的物化内容；返回节点与内容哈希）：
    /// - delta 为空：内容即 prev（prev 必为驻留正规节点），O(1) 短路；
    /// - 命中判定：同 (len, hash) 下先试「同 parent 同 delta」（克隆上下文锁步重导出的常态，
    ///   O(|delta|)）；同 parent 异 delta 且同 len 时内容必不等（两 delta 与 parent 均不相交、
    ///   等长互异），跳过；异链同内容（哈希桶内）以候选链 d 路归并流式比对 prev_m ∪ delta 兜底。
    pub(super) fn intern(&mut self, prev: &Rc<GSet>, prev_m: &[u32], delta: &[u32], dh: u64, full_len: u32) -> (Rc<GSet>, u64) {
        if delta.is_empty() {
            self.hits += 1;
            return (prev.clone(), prev.hash);
        }
        let h = prev.hash.wrapping_add(dh);
        let bucket = self.ix.entry(h).or_default();
        bucket.retain(|w| w.strong_count() > 0);
        for w in bucket.iter() {
            let rc = w.upgrade().unwrap();
            if rc.len != full_len {
                continue;
            }
            let same_parent = rc.parent.as_ref().is_some_and(|p| Rc::ptr_eq(p, prev));
            if same_parent {
                if rc.delta.as_ref() == delta {
                    self.hits += 1;
                    return (rc, h);
                }
                continue;
            }
            // 兜底：候选链内容与查询（prev_m ∪ delta，两段升序不相交）流式比对（等长 ⟹ 逐元素相等即相等）
            if chain_eq(&rc, prev_m, delta) {
                self.hits += 1;
                return (rc, h);
            }
        }
        self.news += 1;
        let rc = Rc::new(GSet {
            parent: Some(prev.clone()),
            delta: delta.into(),
            len: full_len,
            hash: h,
        });
        bucket.push(Rc::downgrade(&rc));
        (rc, h)
    }
    /// 深堆字节（census deep3，只读）：索引桶
    pub(super) fn deep_bytes(&self) -> usize {
        self.ix.capacity() * 40 * 9 / 7 + self.ix.values().map(|v| 24 + v.len() * 16).sum::<usize>()
    }
    /// 活节点数（观测）
    pub(super) fn live(&self) -> usize {
        self.ix.values().map(|v| v.iter().filter(|w| w.strong_count() > 0).count()).sum::<usize>()
    }
}

/// 汇集节点键的对象集项：按预存内容哈希入桶（§4.19——免对整段走 SipHash）；判等指针先试
/// （驻留池保证同内容至多一个活节点，键均取自池），(len, hash) + 物化兜底防哈希碰撞
pub(super) struct SetKey(pub(super) Rc<GSet>, u64);

impl PartialEq for SetKey {
    fn eq(&self, o: &Self) -> bool {
        if Rc::ptr_eq(&self.0, &o.0) {
            return true;
        }
        if self.1 != o.1 || self.0.len != o.0.len {
            return false;
        }
        let (mut a, mut b) = (Vec::new(), Vec::new());
        materialize(&self.0, &mut a);
        materialize(&o.0, &mut b);
        a == b
    }
}
impl Eq for SetKey {}
impl std::fmt::Debug for SetKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SetKey(len={}, hash={})", self.0.len, self.1)
    }
}
impl std::hash::Hash for SetKey {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.1.hash(state);
    }
}

impl Slot {
    /// 分配点 o 的该槽位节点
    fn node(self, o: u32) -> Node {
        match self {
            Slot::Field(fi) => Node::O(o, fi),
            Slot::Elem(p) => Node::E(o, p),
        }
    }
}

/// 汇集节点与槽位节点间的流边过滤
#[derive(Clone, Copy)]
enum Filt {
    /// 各对象同一过滤类型
    Same(u32),
    /// 手写写入数组元素：逐数组按分量类型（[`Engine::elem_write_filter`]）；汇集节点自身不过滤（Object）。
    /// 只用于写向元素槽（字节码数组元素站点只有读向），故按 (槽位, 方向, 集合) 共享的汇集节点过滤口径唯一
    ArrayWrite,
}

impl Engine<'_> {
    fn filt_of(&mut self, f: Filt, o: u32) -> u32 {
        match f {
            Filt::Same(t) => t,
            Filt::ArrayWrite => self.elem_write_filter(o).unwrap_or_else(|| self.id(OBJECT)),
        }
    }

    /// 汇集节点之间（子集链）的过滤
    fn filt_chain(&mut self, f: Filt) -> u32 {
        match f {
            Filt::Same(t) => t,
            Filt::ArrayWrite => self.id(OBJECT),
        }
    }

    /// 字节码字段读站点 (m, off) 新接上抽象对象 objs（升序、此前未接过）：字段 fi（类型 tid）的值流到 res
    pub(super) fn gather_read(&mut self, m: usize, off: u32, fi: usize, tid: u32, objs: &[u32], res: Node) {
        match self.gather_node(m, off, Slot::Field(fi), tid, objs, false) {
            Some(g) => self.flow(Node::G(g), res, tid),
            None => {
                for &o in objs {
                    self.flow(Node::O(o, fi), res, tid);
                }
            }
        }
    }

    /// 字节码字段写站点 (m, off) 新接上抽象对象 objs（升序、此前未接过）：写入值 fs 流到各对象的字段 fi（类型 tid）
    pub(super) fn gather_write(&mut self, m: usize, off: u32, fi: usize, tid: u32, objs: &[u32], fs: &[Feed]) {
        match self.gather_node(m, off, Slot::Field(fi), tid, objs, true) {
            Some(g) => self.feed(fs, Node::G(g), tid),
            None => {
                for &o in objs {
                    self.feed(fs, Node::O(o, fi), tid);
                }
            }
        }
    }

    /// 字节码数组读站点 (m, off) 的数组分配点 xs（升序，可含已接过的）：元素槽 p 的值按静态分量类型 tid 流到 res
    pub(super) fn gather_elems(&mut self, m: usize, off: u32, p: u8, tid: u32, xs: &[u32], res: Node) {
        let obj = self.id(OBJECT);
        match self.gather_node(m, off, Slot::Elem(p), obj, xs, false) {
            Some(g) => self.flow(Node::G(g), res, tid),
            None => {
                for &x in xs {
                    self.flow(Node::E(x, p), res, tid);
                }
            }
        }
    }

    /// 手写方法调用点 s 的第 i 个实参新增数组 ys（升序）：元素槽 p 的值以 Object 流到写入来源 ws（`W(s, j)`）。
    /// 同一批数组被大量调用点（各克隆上下文里的 arraycopy 等）传入时经汇集节点共享
    pub(super) fn gather_hw_elems(&mut self, s: u32, i: u16, p: u8, ys: &[u32], ws: &[Node]) {
        let obj = self.id(OBJECT);
        let key = (s, i, p);
        let last = self.hw_gather_last.get(&key).cloned();
        let (now, g) = self.gather_step(last, Slot::Elem(p), Filt::Same(obj), ys, false);
        if let Some(now) = now {
            self.hw_gather_last.insert(key, now);
        }
        match g {
            Some(g) => {
                for &w in ws {
                    self.flow(Node::G(g), w, obj);
                }
            }
            None => {
                for &y in ys {
                    for &w in ws {
                        self.flow(Node::E(y, p), w, obj);
                    }
                }
            }
        }
    }

    /// 手写读内存调用点 s 的源实参新增数组 xs（升序，可含已接过的）：元素槽 p 的值按 rt 流到结果 res。
    /// 同一批数组（如各上下文的哈希表数组）被大量调用点（中继读方法的各克隆）读取时经汇集节点共享
    pub(super) fn gather_hw_read(&mut self, s: u32, p: u8, xs: &[u32], res: Node, rt: u32) {
        let obj = self.id(OBJECT);
        let last = self.hw_rgather_last.get(&(s, p)).cloned();
        let (now, g) = self.gather_step(last, Slot::Elem(p), Filt::Same(obj), xs, false);
        if let Some(now) = now {
            self.hw_rgather_last.insert((s, p), now);
        }
        match g {
            Some(g) => self.flow(Node::G(g), res, rt),
            None => {
                for &x in xs {
                    self.flow(Node::E(x, p), res, rt);
                }
            }
        }
    }

    /// 手写方法调用点 s 的写入目标实参 i 新增引用元素数组 ys（升序，可含已接过的）：写入来源 `W(s, i)` 流到各数组的
    /// 元素槽 p（逐数组按 [`Self::elem_write_filter`] 过滤）。同一批数组被大量调用点（CAS / 有序写的各克隆）写入时经汇集节点共享
    pub(super) fn gather_hw_write(&mut self, s: u32, i: u16, p: u8, ys: &[u32]) {
        let obj = self.id(OBJECT);
        let last = self.hw_wgather_last.get(&(s, i, p)).cloned();
        let (now, g) = self.gather_step(last, Slot::Elem(p), Filt::ArrayWrite, ys, true);
        if let Some(now) = now {
            self.hw_wgather_last.insert((s, i, p), now);
        }
        match g {
            Some(g) => self.flow(Node::W(s, i), Node::G(g), obj),
            None => {
                for &y in ys {
                    if let Some(f) = self.elem_write_filter(y) {
                        self.flow(Node::W(s, i), Node::E(y, p), f);
                    }
                }
            }
        }
    }

    /// 字节码站点 (m, off) 累计对象（原有 ∪ objs）对应的汇集节点；累计不足 `GATHER_MIN` 时 None（调用方逐对象接边）
    fn gather_node(&mut self, m: usize, off: u32, slot: Slot, f: u32, objs: &[u32], put: bool) -> Option<u32> {
        let last = self.gather_last.get(&m).and_then(|s| s.get(&(off, slot))).cloned();
        let (now, g) = self.gather_step(last, slot, Filt::Same(f), objs, put);
        if let Some(now) = now {
            self.gather_last.entry(m).or_default().insert((off, slot), now);
        }
        g
    }

    /// 站点原有记录 last（汇集节点, 累计对象链）并入 objs（升序，可含已接过的）后的新记录（不变时 None）与汇集节点；
    /// 槽位节点与汇集节点间按 f 过滤。差分链（§4.20）：只物化 delta = objs∖prev，新集合驻留为
    /// prev 上的链节点——行为与数组版逐项相同（集合内容、接边对象集、遍历升序）
    fn gather_step(&mut self, last: Option<(u32, Rc<GSet>)>, slot: Slot, f: Filt, objs: &[u32], put: bool) -> (Option<(u32, Rc<GSet>)>, Option<u32>) {
        let (g0, prev) = last.unwrap_or_else(|| (NO_GATHER, self.gather_pool.empty()));
        // 物化 prev 链（升序）与 objs 归并：delta = objs∖prev（升序）与增量哈希一次产出
        let mut prevs: Vec<u32> = Vec::new();
        materialize(&prev, &mut prevs);
        let mut delta: Vec<u32> = Vec::with_capacity(objs.len());
        let mut dh = 0u64;
        {
            let (mut i, mut j) = (0usize, 0usize);
            while j < objs.len() {
                if j > 0 && objs[j] == objs[j - 1] {
                    j += 1; // objs 内部相邻重复（API 不产，防御）
                    continue;
                }
                while i < prevs.len() && prevs[i] < objs[j] {
                    i += 1;
                }
                if i < prevs.len() && prevs[i] == objs[j] {
                    i += 1;
                } else {
                    dh = dh.wrapping_add(u32_elem_hash(objs[j]));
                    delta.push(objs[j]);
                }
                j += 1;
            }
        }
        // 无新增对象：沿用原汇集节点（重跑的常态）
        if delta.is_empty() && g0 != NO_GATHER {
            return (None, Some(g0));
        }
        let full_len = prev.len + delta.len() as u32;
        if (full_len as usize) < GATHER_MIN {
            // 小集合也驻留：站点数远多于汇集节点数，同名小集合（未达阈值前的公共前缀）跨站点重复
            let (full, _) = self.gather_pool.intern(&prev, &prevs, &delta, dh, full_len);
            return (Some((NO_GATHER, full)), None);
        }
        // 内容驻留（§4.19/§4.20）：同内容集合只此一份链节点——站点累计集与汇集节点键共享
        let n0 = self.gather_pool.news;
        let (full, kh) = self.gather_pool.intern(&prev, &prevs, &delta, dh, full_len);
        if self.gather_pool.news > n0 {
            // 画像（§4.20）：该相异内容首次物化时的链式增量（只计数，不改行为）
            let d = delta.len() as u64;
            self.gather_pool.delta_sum += d;
            self.gather_pool.delta_hist[if d <= 1 {
                0
            } else if d <= 4 {
                1
            } else if d <= 16 {
                2
            } else if d <= 64 {
                3
            } else if d <= 256 {
                4
            } else {
                5
            }] += 1;
        }
        let g = match self.gather_ids.get(&(slot, put, SetKey(full.clone(), kh))) {
            Some(&g) => g,
            None => {
                let g = self.gathers.len() as u32;
                self.gathers.push((slot, full_len, put));
                self.gather_ids.insert((slot, put, SetKey(full.clone(), kh)), g);
                // 新集合 = 原汇集节点的集合（子集）∪ 其余对象；无原节点时全部对象逐个接边
                let rest: Vec<u32> = if g0 == NO_GATHER {
                    let mut v = Vec::with_capacity(prevs.len() + delta.len());
                    v.extend_from_slice(&prevs);
                    v.extend_from_slice(&delta);
                    v.sort_unstable();
                    v.dedup();
                    v
                } else {
                    let fc = self.filt_chain(f);
                    if put {
                        self.flow(Node::G(g), Node::G(g0), fc);
                    } else {
                        self.flow(Node::G(g0), Node::G(g), fc);
                    }
                    delta.clone()
                };
                for o in rest {
                    let fo = self.filt_of(f, o);
                    if put {
                        self.flow(Node::G(g), slot.node(o), fo);
                    } else {
                        self.flow(slot.node(o), Node::G(g), fo);
                    }
                }
                g
            }
        };
        (Some((g, full)), Some(g))
    }

    /// gathers 相异内容形态画像（§4.20，`RAVA_GATHERS_SHAPE=1` 时随 `--lambda-prof` dump 打印，只读）：
    /// 活相异集合（键集 ∪ 站点累计，节点指针去重）按内容形态分桶——len 桶（n 与 Σlen）、区间画像
    /// （逐集物化后 run 数与 fill）、差分链实际驻留（Σ(节点 56B + 4|delta|)）与增量计数。
    /// 逐 dump 全量物化（Σlen·log 深度 元素级）有秒级代价，验收运行不开。
    /// 选型前的字典序 lcp / trie 上界列已随选型定案移除（证据见 §4.20 首表）
    pub(super) fn gathers_shape_dump(&self, tag: &str) {
        let mib = |b: f64| b / 1048576.0;
        // 活相异集合 = 键集（gather_ids 键）∪ 站点累计（gather_last / hw 族），节点指针去重；
        // 「仅键集持有」= 链中间步（站点已换接到更大集合），单独计长
        let mut site_ptrs: HashSet<usize> = HashSet::default();
        let mut site_of: Vec<Rc<GSet>> = Vec::new();
        let add_site = |v: &Rc<GSet>, site_ptrs: &mut HashSet<usize>, site_of: &mut Vec<Rc<GSet>>| {
            if site_ptrs.insert(Rc::as_ptr(v) as *const u8 as usize) {
                site_of.push(v.clone());
            }
        };
        for m in self.gather_last.values() {
            for v in m.values() {
                add_site(&v.1, &mut site_ptrs, &mut site_of);
            }
        }
        for v in self.hw_gather_last.values().chain(self.hw_rgather_last.values()).chain(self.hw_wgather_last.values()) {
            add_site(&v.1, &mut site_ptrs, &mut site_of);
        }
        let mut all: Vec<Rc<GSet>> = Vec::new();
        let mut seen: HashSet<usize> = HashSet::default();
        let mut key_only_len = 0u64;
        let mut key_n = 0u64;
        for k in self.gather_ids.keys() {
            if seen.insert(Rc::as_ptr(&k.2.0) as *const u8 as usize) {
                key_n += 1;
                if !site_ptrs.contains(&(Rc::as_ptr(&k.2.0) as *const u8 as usize)) {
                    key_only_len += k.2.0.len as u64;
                }
                all.push(k.2.0.clone());
            }
        }
        let mut site_n = 0u64;
        for v in &site_of {
            if seen.insert(Rc::as_ptr(v) as *const u8 as usize) {
                site_n += 1;
                all.push(v.clone());
            }
        }
        drop(seen);
        drop(site_ptrs);
        // 分桶与区间画像：len 桶（n 与 Σlen）、逐集 run 数（极大连续 id 段）与跨度（物化到复用缓冲）
        let mut n_bucket = [0u64; 6];
        let mut l_bucket = [0u64; 6];
        let (mut sum_len, mut sum_runs, mut sum_span, mut chain_bytes) = (0u64, 0u64, 0u64, 0usize);
        let mut scratch: Vec<u32> = Vec::new();
        let mut node_seen: HashSet<usize> = HashSet::default();
        for s in &all {
            let l = s.len as u64;
            materialize(s, &mut scratch);
            let (r, span) = if scratch.is_empty() {
                (0, 0)
            } else {
                let mut r = 1u64;
                for w in scratch.windows(2) {
                    r += u64::from(w[1] != w[0] + 1);
                }
                (r, (scratch[scratch.len() - 1] - scratch[0]) as u64 + 1)
            };
            let b = if l < 4 {
                0
            } else if l < 32 {
                1
            } else if l < 256 {
                2
            } else if l < 4096 {
                3
            } else if l < 65536 {
                4
            } else {
                5
            };
            n_bucket[b] += 1;
            l_bucket[b] += l;
            sum_len += l;
            sum_runs += r;
            sum_span += span;
            // 链实际驻留：沿祖先每节点计一次（56 = Rc 头 16 + 节点 40）
            let mut cur = Some(Rc::clone(s));
            while let Some(n) = cur {
                if node_seen.insert(Rc::as_ptr(&n) as *const u8 as usize) {
                    chain_bytes += 56 + n.delta.len() * 4;
                }
                cur = n.parent.clone();
            }
        }
        let n = all.len() as u64;
        let run_bytes = 16.0 * n as f64 + 8.0 * sum_runs as f64;
        eprintln!(
            "{tag} gathers-shape n={n} sum_len={sum_len} chain_now={:.0}MiB keys(n={},only_len={}) sites(n={}) \
             lenbucket(n)={n_bucket:?} lenbucket(len)={l_bucket:?} runs={sum_runs} run_repr={:.0}MiB fill={:.3} \
             delta_sum={} chain_repr={:.0}MiB delta_hist={:?}",
            mib(chain_bytes as f64),
            key_n,
            key_only_len,
            site_n,
            mib(run_bytes),
            sum_len as f64 / sum_span.max(1) as f64,
            self.gather_pool.delta_sum,
            mib(4.0 * self.gather_pool.delta_sum as f64 + 56.0 * self.gather_pool.news as f64),
            self.gather_pool.delta_hist,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{materialize, u32_elem_hash, SetKey, SetPool};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    /// 差分链语义（§4.20）：物化升序且等于并集；len / 哈希增量与全量重算一致；驻留同内容只此一份
    /// 节点（同 parent 快路径 + 异链流式兜底）；SetKey 判等与哈希口径一致；空 delta 短路回 prev
    #[test]
    fn setpool_intern_semantics() {
        let mut p = SetPool::default();
        let empty = p.empty();
        let dh = |xs: &[u32]| xs.iter().fold(0u64, |a, &x| a.wrapping_add(u32_elem_hash(x)));
        // 首链：∅ → {1,3,5} → {1,3,5,7,9}
        let (a, _ha) = p.intern(&empty, &[], &[1, 3, 5], dh(&[1, 3, 5]), 3);
        let mut m = Vec::new();
        materialize(&a, &mut m);
        assert_eq!(m, vec![1, 3, 5]);
        assert_eq!(a.len, 3);
        assert_eq!(a.hash, dh(&[1, 3, 5]));
        let (b, hb) = p.intern(&a, &[1, 3, 5], &[7, 9], dh(&[7, 9]), 5);
        materialize(&b, &mut m);
        assert_eq!(m, vec![1, 3, 5, 7, 9], "并集按新元素增长（delta 已过滤已接元素）");
        assert_eq!(b.len, 5);
        assert_eq!(hb, dh(&[1, 3, 5, 7, 9]), "哈希 = parent 哈希 + 新元素增量");
        // 异链同内容（流式兜底）：∅ → {7,9} → {1,3,5,7,9} 与 b 内容相同
        let (c, _) = p.intern(&empty, &[], &[7, 9], dh(&[7, 9]), 2);
        let (d, hd) = p.intern(&c, &[7, 9], &[1, 3, 5], dh(&[1, 3, 5]), 5);
        assert_eq!(hd, hb);
        assert!(Rc::ptr_eq(&b, &d), "异链同内容必须驻留到同一节点");
        // 同 parent 同 delta（快路径）
        let (e, _) = p.intern(&a, &[1, 3, 5], &[7, 9], dh(&[7, 9]), 5);
        assert!(Rc::ptr_eq(&b, &e));
        // 同 parent 异 delta 同 len：内容必不等（各自登记）
        let (f, hf) = p.intern(&a, &[1, 3, 5], &[6, 8], dh(&[6, 8]), 5);
        assert_ne!(hf, hb);
        assert!(!Rc::ptr_eq(&b, &f));
        // 空 delta：内容即 prev，短路复用
        let (g, hg) = p.intern(&b, &[1, 3, 5, 7, 9], &[], 0, 5);
        assert!(Rc::ptr_eq(&b, &g));
        assert_eq!(hg, hb);
        // SetKey：指针相等快速路径 + (len, hash) 物化兜底（哈希一致）
        let k1 = SetKey(b.clone(), hb);
        let k2 = SetKey(d.clone(), hd);
        let k3 = SetKey(f.clone(), hf);
        assert_eq!(k1, k2);
        assert_ne!(k1, k3);
        let h = |k: &SetKey| {
            let mut s = DefaultHasher::new();
            k.hash(&mut s);
            s.finish()
        };
        assert_eq!(h(&k1), h(&k2));
        // 死节点出桶：drop 后再 intern 同内容得到新节点（桶内清理）
        drop((f, k3));
        let (i, hi) = p.intern(&a, &[1, 3, 5], &[6, 8], dh(&[6, 8]), 5);
        assert_eq!(hi, hf);
        assert_ne!(p.live(), 0);
        let _ = (i, e, c, empty, a);
    }

    /// 深链物化：多级 delta 两两不相交、归并升序无重复（模拟站点逐批增长的链）
    #[test]
    fn gset_chain_materialize() {
        let mut p = SetPool::default();
        let empty = p.empty();
        let dh = |xs: &[u32]| xs.iter().fold(0u64, |a, &x| a.wrapping_add(u32_elem_hash(x)));
        let batches: [&[u32]; 6] = [&[10, 20], &[5, 15, 25], &[1], &[30, 40, 50], &[7, 33], &[2, 60]];
        let mut cur = empty;
        let mut expect: Vec<u32> = Vec::new();
        for b in batches {
            let delta: Vec<u32> = b.iter().copied().filter(|x| !expect.binary_search(x).is_ok()).collect();
            let prev_snapshot = expect.clone();
            expect.extend_from_slice(&delta);
            expect.sort_unstable();
            let (next, _) = p.intern(&cur, &prev_snapshot, &delta, dh(&delta), expect.len() as u32);
            let mut m = Vec::new();
            materialize(&next, &mut m);
            assert_eq!(m, expect, "链物化 = 逐批并集");
            cur = next;
        }
    }
}

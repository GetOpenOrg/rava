//! `IdSet`：u32 id 的有序集合，稀疏 / 稠密两种形态。
//!
//! - 稀疏：有序 `Vec<u32>`，多数集合只有几个到几十个元素，二分 / 归并最快；
//! - 稠密：只存位图（`words`）与非零字摘要（`sum`，每位对应一个非零字）。
//!   插入 O(1)（不再为保持有序向量逐个搬移），差 / 并 / 子集按字运算，遍历按摘要跳过零字。
//!
//! 形态由**元素数与跨度**共同决定（性能计划 §4.16 表征压缩）：位图按最大 id 分配（`max/8` 字节，
//! 与元素数无关），宽距低密度集合（元素 × 32 < 最大 id）的位图远大于有序数组（元素 × 4 字节），
//! 这类集合保持稀疏——逐克隆节点的类型集大量处于此区间（uniq 内容的平均 fill 见 census uniq 行）。
//! 升格在元素 × 32 ≥ 最大 id（平均每字 2 元素）、降格在元素数 < 字数（每字不足 1 元素，位图两倍于数组），
//! 其间滞回防振荡；遍历一律升序，形态不参与哈希 / 判等（内容相同的两个 `IdSet` 无论形态逐元素相等）。

/// 内联只占一个 `Vec` 宽（24 字节）：稠密形态的位图与计数装箱。
/// 流图每节点存两份类型集（值 + 待推增量）、每份两个 `IdSet`，内联宽度直接决定节点表的常驻内存
#[derive(Debug, Clone)]
pub struct IdSet(Repr);

#[derive(Debug, Clone)]
enum Repr {
    /// 有序元素
    Sparse(Vec<u32>),
    Dense(Box<Dense>),
}

/// 稀疏 → 稠密的元素数下限（形态还要求跨度足够小：元素 × 32 ≥ 最大 id，见模块说明）
pub(super) const DENSE_AT: usize = 64;

/// 形态判定：此元素数与跨度（最大元素 id）下稠密不劣于有序数组
/// （位图 ≈ `max/8` 字节、数组 `len×4` 字节：`len×32 ≥ max` 即平均每字 2 元素）。
/// 降格线 `元素数 < 字数`（平均每字不足 1 元素，位图已两倍于数组），其间留 2 倍滞回防振荡
#[inline]
fn dense_worth(len: usize, max: u32) -> bool {
    len >= DENSE_AT && len * 32 >= max as usize
}

/// 表征画像累加器（`--lambda-prof` 的 census uniq 行，只读诊断；性能计划 §4.16 表征压缩画像）。
///
/// 关键量：稠密形态的 `words` 按**最大 id** 分配（与元素数无关），画像给出
/// 真实驻留字节（capacity 口径，与计数分配器的活跃字节同口径）、非零字占比（fill）
/// 与按块压缩（4096 id = 64 字为一块，非零块存位图或有序 u16 低 12 位数组）的估计字节
/// ——`est_blocks` 为全部换块式、`est_adaptive` 为逐集合取两形态较小者。
#[derive(Default)]
pub(super) struct ReprProfile {
    /// 集合数
    pub(super) sets: u64,
    /// 深堆字节（capacity 口径）
    pub(super) bytes: u64,
    pub(super) sparse_n: u64,
    pub(super) sparse_bytes: u64,
    pub(super) dense_n: u64,
    /// words.len() 计 / capacity 字节 / sum 字节 / Box 头字节
    pub(super) words: u64,
    pub(super) words_cap: u64,
    pub(super) sum_cap: u64,
    pub(super) box_hdr: u64,
    /// 非零字数 / 非零块数（sum 的置位数；一位 = 64 字 = 4096 id）
    pub(super) nz_words: u64,
    pub(super) nz_blocks: u64,
    /// 元素数直方图：<8 / <32 / <64 / <256 / <1k / <8k / ≥8k（稀疏止于 63）
    pub(super) len_hist: [u64; 7],
    /// fill 直方图（非零字 / 字数）：<5% / <20% / <50% / <80% / ≥80%
    pub(super) fill_hist: [u64; 5],
    /// 块式估计 / 纯稀疏估计（len×4×1.25 容量余量）/ 逐集合自适应估计（字节）：
    /// `est_blocks`、`est_sparse` 为全部换该形态；`est_span` / `est_block` 为逐集合取「现状与该形态较小者」
    /// （span 感知形态 / 块式压缩各自的净收益），`est_adaptive` 为三者与现状取小
    pub(super) est_blocks: u64,
    pub(super) est_sparse: u64,
    pub(super) est_span: u64,
    pub(super) est_block: u64,
    pub(super) est_adaptive: u64,
}

impl IdSet {
    /// 并入画像（只读；稠密集合按块统计非零块数与块内元素数）
    pub(super) fn profile(&self, p: &mut ReprProfile) {
        p.sets += 1;
        match &self.0 {
            Repr::Sparse(v) => {
                p.sparse_n += 1;
                let b = v.capacity() as u64 * 4;
                p.sparse_bytes += b;
                p.bytes += b;
                let n = v.len();
                p.len_hist[if n < 8 {
                    0
                } else if n < 32 {
                    1
                } else if n < 64 {
                    2
                } else {
                    3
                }] += 1;
            }
            Repr::Dense(d) => {
                p.dense_n += 1;
                let (wb, sb) = (d.words.capacity() as u64 * 8, d.sum.capacity() as u64 * 8);
                let hdr = std::mem::size_of::<Dense>() as u64;
                let cur = wb + sb + hdr + std::mem::size_of::<Box<Dense>>() as u64;
                p.words += d.words.len() as u64;
                p.words_cap += wb;
                p.sum_cap += sb;
                p.box_hdr += hdr + std::mem::size_of::<Box<Dense>>() as u64;
                p.bytes += cur;
                let wl = d.words.len();
                let n = d.n;
                p.len_hist[if n < 256 {
                    3
                } else if n < 1024 {
                    4
                } else if n < 8192 {
                    5
                } else {
                    6
                }] += 1;
                // 逐 sum 字统计：sum 的每位 = 一个非零字；一个非零 sum 字 = 一个非零块（64 字）
                let mut est: u64 = 16 + 2; // 块索引（按非零块 ×2 字累计）+ 头
                for (si, &s) in d.sum.iter().enumerate() {
                    if s == 0 {
                        continue;
                    }
                    p.nz_blocks += 1;
                    p.nz_words += u64::from(s.count_ones());
                    let mut bits = s;
                    while bits != 0 {
                        let b = bits.trailing_zeros() as usize;
                        bits &= bits - 1;
                        let w0 = (si * 64 + b) * 64;
                        let mut e: u32 = 0;
                        for w in d.words.iter().take((w0 + 64).min(wl)).skip(w0) {
                            e += w.count_ones();
                        }
                        // 块容器：≤64 元素存有序 u16（低 12 位），否则 512B 位图
                        est += if e <= 64 { u64::from(e) * 2 + 8 } else { 512 + 8 };
                    }
                }
                p.est_blocks += est;
                // 纯稀疏估计：有序 u32 数组（容量按 1.25 倍摊还），宽距低密度集合的最小形态
                let sp = n as u64 * 5 + 24;
                p.est_sparse += sp;
                p.est_span += sp.min(cur);
                p.est_block += est.min(cur);
                p.est_adaptive += est.min(cur).min(sp);
                // fill 直方图按本集合的非零字占比
                let local_nz: u64 = d.sum.iter().map(|s| u64::from(s.count_ones())).sum();
                let r = if wl == 0 { 0 } else { local_nz * 100 / wl.max(1) as u64 };
                p.fill_hist[if r < 5 {
                    0
                } else if r < 20 {
                    1
                } else if r < 50 {
                    2
                } else if r < 80 {
                    3
                } else {
                    4
                }] += 1;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Dense {
    /// 位图
    words: Vec<u64>,
    /// 第 i 位 = `words[i]` 非零
    sum: Vec<u64>,
    /// 元素数
    n: usize,
}

impl Default for IdSet {
    fn default() -> IdSet {
        IdSet(Repr::Sparse(Vec::new()))
    }
}

impl PartialEq for IdSet {
    fn eq(&self, o: &IdSet) -> bool {
        self.len() == o.len() && self.iter().eq(o.iter())
    }
}
impl Eq for IdSet {}
impl std::hash::Hash for IdSet {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        // 与旧形态（`Vec<u32>` 的哈希）逐字节相同：内部表的遍历顺序不因形态改变
        match &self.0 {
            Repr::Dense(_) => self.iter().collect::<Vec<u32>>().hash(h),
            Repr::Sparse(v) => v.hash(h),
        }
    }
}

/// `Dense::set_bit` 的结果：新置 / 已置 / 跨度越出降格线（元素 × 64 < 字数，x 尚未置位，
/// 调用方换稀疏数组后自行插入——位图按最大 id 分配，宽距低密度集合数组小得多，见模块说明）
enum SetBit {
    New,
    No,
    Demote(u32),
}

impl Dense {
    /// 置位（不维护计数）
    #[inline]
    fn set_bit(&mut self, x: u32) -> SetBit {
        let w = x as usize / 64;
        if self.words.len() <= w {
            // 跨度增长越出降格线（元素数 < 字数，平均每字不足 1 元素）：不扩位图，交调用方降格
            if w > self.n {
                return SetBit::Demote(x);
            }
            self.words.resize(w + 1, 0);
            self.sum.resize(w / 64 + 1, 0);
        }
        let b = 1u64 << (x % 64);
        if self.words[w] & b != 0 {
            return SetBit::No;
        }
        self.words[w] |= b;
        self.sum[w / 64] |= 1 << (w % 64);
        SetBit::New
    }
    /// 收集为有序元素数组（降格用）
    fn to_sparse_vec(&self) -> Vec<u32> {
        let mut v = Vec::with_capacity(self.n);
        for w in self.nz_words() {
            let mut bits = self.words[w];
            while bits != 0 {
                let b = bits.trailing_zeros();
                bits &= bits - 1;
                v.push((w * 64) as u32 + b);
            }
        }
        v
    }
    #[inline]
    fn contains(&self, x: u32) -> bool {
        let w = x as usize / 64;
        w < self.words.len() && self.words[w] >> (x % 64) & 1 != 0
    }
    #[inline]
    fn word(&self, w: usize) -> u64 {
        self.words.get(w).copied().unwrap_or(0)
    }
    /// 跨度爆到降格线以下（元素数 < 字数，平均每字不足 1 元素）：换稀疏数组（调用方持有 `&mut IdSet`）
    #[inline]
    fn demote_if_span_escaped(&self) -> bool {
        self.n < self.words.len()
    }
    /// 非零字下标（升序）
    fn nz_words(&self) -> impl Iterator<Item = usize> + '_ {
        self.sum.iter().enumerate().flat_map(|(si, &s)| BitIter(s).map(move |b| si * 64 + b as usize))
    }
    fn from_sparse(v: &[u32]) -> Dense {
        let max = v.last().copied().unwrap_or(0);
        let words = vec![0u64; max as usize / 64 + 1];
        let sum = vec![0u64; words.len() / 64 + 1];
        let mut d = Dense { words, sum, n: v.len() };
        for &x in v {
            d.set_bit(x);
        }
        d
    }
}

impl IdSet {
    #[cfg(test)]
    fn dense(&self) -> bool {
        matches!(self.0, Repr::Dense(_))
    }
    /// 由升序去重的元素构造（形态按元素数与跨度：位图按最大 id 分配，宽距低密度保持稀疏）
    pub(super) fn from_sorted(v: Vec<u32>) -> IdSet {
        if dense_worth(v.len(), v.last().copied().unwrap_or(0)) {
            IdSet(Repr::Dense(Box::new(Dense::from_sparse(&v))))
        } else {
            IdSet(Repr::Sparse(v))
        }
    }
    pub fn insert(&mut self, x: u32) -> bool {
        let demote = match &mut self.0 {
            Repr::Dense(d) => match d.set_bit(x) {
                SetBit::New => {
                    d.n += 1;
                    return true;
                }
                SetBit::No => return false,
                SetBit::Demote(_) => true,
            },
            Repr::Sparse(v) => {
                if let Err(i) = v.binary_search(&x) {
                    v.insert(i, x);
                    if dense_worth(v.len(), v.last().copied().unwrap_or(0)) {
                        let d = Dense::from_sparse(v);
                        self.0 = Repr::Dense(Box::new(d));
                    }
                    return true;
                }
                return false;
            }
        };
        debug_assert!(demote);
        // 跨度越出降格线：换稀疏数组后插入 x
        let Repr::Dense(d) = &self.0 else { unreachable!() };
        let mut v = d.to_sparse_vec();
        if let Err(i) = v.binary_search(&x) {
            v.insert(i, x);
        }
        self.0 = Repr::Sparse(v);
        true
    }
    #[inline]
    pub fn contains(&self, x: &u32) -> bool {
        match &self.0 {
            Repr::Dense(d) => d.contains(*x),
            Repr::Sparse(v) => v.binary_search(x).is_ok(),
        }
    }
    /// 堆占用字节（估算，用于记忆表的内存预算）
    pub(super) fn heap_bytes(&self) -> usize {
        match &self.0 {
            Repr::Sparse(v) => v.capacity() * 4,
            Repr::Dense(d) => std::mem::size_of::<Dense>() + (d.words.capacity() + d.sum.capacity()) * 8,
        }
    }
    #[inline]
    pub fn len(&self) -> usize {
        match &self.0 {
            Repr::Dense(d) => d.n,
            Repr::Sparse(v) => v.len(),
        }
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// 升序遍历
    pub fn iter(&self) -> IdIter<'_> {
        match &self.0 {
            Repr::Dense(d) => IdIter::Dense { words: &d.words, sum: &d.sum, si: 0, sbits: 0, w: 0, bits: 0 },
            Repr::Sparse(v) => IdIter::Sparse(v.iter()),
        }
    }
    /// self \ o（差为空时不分配）
    pub(super) fn minus(&self, o: &IdSet) -> IdSet {
        if o.is_empty() {
            return self.clone();
        }
        match (&self.0, &o.0) {
            (Repr::Dense(a), Repr::Dense(b)) => {
                let mut out = Vec::new();
                for w in a.nz_words() {
                    let x = a.words[w] & !b.word(w);
                    if x != 0 {
                        out.extend(BitIter(x).map(|b| (w * 64) as u32 + b));
                    }
                }
                return IdSet::from_sorted(out);
            }
            (Repr::Sparse(a), Repr::Sparse(b)) if a.len() * 16 >= b.len() => {
                // 两侧有序归并
                let mut out: Vec<u32> = Vec::new();
                let mut j = 0;
                for (i, &x) in a.iter().enumerate() {
                    while j < b.len() && b[j] < x {
                        j += 1;
                    }
                    if j == b.len() || b[j] != x {
                        if out.capacity() == 0 {
                            out.reserve(a.len() - i);
                        }
                        out.push(x);
                    }
                }
                return IdSet::from_sorted(out);
            }
            _ => {}
        }
        // 逐个查成员
        let mut it = self.iter();
        let Some(first) = it.by_ref().find(|x| !o.contains(x)) else { return IdSet::default() };
        let mut out = vec![first];
        out.extend(it.filter(|x| !o.contains(x)));
        IdSet::from_sorted(out)
    }
    /// self ⊆ o（不分配）
    pub(super) fn is_subset(&self, o: &IdSet) -> bool {
        if self.len() > o.len() {
            return false;
        }
        match (&self.0, &o.0) {
            (Repr::Dense(a), Repr::Dense(b)) => a.nz_words().all(|w| a.words[w] & !b.word(w) == 0),
            // 形态含跨度（宽距低密度的稠密超集不存在，但稀疏超集的元素可多于宽距稠密 self）：逐元素查
            (Repr::Dense(_), Repr::Sparse(_)) => self.iter().all(|x| o.contains(&x)),
            (Repr::Sparse(a), Repr::Dense(b)) => a.iter().all(|&x| b.contains(x)),
            (Repr::Sparse(a), Repr::Sparse(b)) => {
                if a.len() * 16 < b.len() {
                    return a.iter().all(|x| b.binary_search(x).is_ok());
                }
                let mut j = 0;
                for &x in a {
                    while j < b.len() && b[j] < x {
                        j += 1;
                    }
                    if j == b.len() || b[j] != x {
                        return false;
                    }
                    j += 1;
                }
                true
            }
        }
    }
    /// 并入 o
    pub(super) fn union_with(&mut self, o: &IdSet) {
        if o.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = o.clone();
            return;
        }
        match (&mut self.0, &o.0) {
            (Repr::Sparse(a), Repr::Dense(b)) => {
                // o 稠密而 self 稀疏：并入后按（合并元素数, 合并跨度）重判形态——
                // 宽距下（合并仍稀疏划算）走有序归并，否则克隆位图置位
                let n2 = a.len() + b.n;
                let max2 = a.last().copied().unwrap_or(0).max(b.words.len() as u32 * 64 - 1);
                if !dense_worth(n2, max2) {
                    let out = merge_sorted(a, &b.to_sparse_vec());
                    self.0 = Repr::Sparse(out);
                    return;
                }
                let mut d = b.clone();
                for &x in a.iter() {
                    if let SetBit::New = d.set_bit(x) {
                        d.n += 1;
                    }
                }
                self.0 = Repr::Dense(d);
            }
            (Repr::Dense(a), Repr::Dense(b)) => {
                if a.words.len() < b.words.len() {
                    a.words.resize(b.words.len(), 0);
                    a.sum.resize(b.sum.len(), 0);
                }
                for w in b.nz_words() {
                    let add = b.words[w] & !a.words[w];
                    if add != 0 {
                        a.words[w] |= add;
                        a.sum[w / 64] |= 1 << (w % 64);
                        a.n += add.count_ones() as usize;
                    }
                }
                // 跨度被 o 拉爆（元素 × 64 < 字数）：降格稀疏
                if a.demote_if_span_escaped() {
                    let v = a.to_sparse_vec();
                    self.0 = Repr::Sparse(v);
                }
            }
            (Repr::Dense(a), Repr::Sparse(b)) => {
                let mut demote = None;
                for &x in b {
                    match a.set_bit(x) {
                        SetBit::New => a.n += 1,
                        SetBit::No => {}
                        SetBit::Demote(x) => {
                            demote = Some(x);
                            break;
                        }
                    }
                }
                if let Some(x) = demote {
                    let mut v = a.to_sparse_vec();
                    if let Err(i) = v.binary_search(&x) {
                        v.insert(i, x);
                    }
                    // b 中 x 之后的元素一并归并（x 之前已并入 a 的收集结果）
                    let rest: Vec<u32> = b.iter().copied().filter(|&y| y > x).collect();
                    self.0 = Repr::Sparse(merge_sorted(&v, &rest));
                }
            }
            (Repr::Sparse(a), Repr::Sparse(b)) => {
                // 两侧稀疏：有序归并（形态由 from_sorted 按合并结果重判）
                *self = IdSet::from_sorted(merge_sorted(a, b));
            }
        }
    }
}

/// u64 的置位位序号（升序）
struct BitIter(u64);

impl Iterator for BitIter {
    type Item = u32;
    #[inline]
    fn next(&mut self) -> Option<u32> {
        if self.0 == 0 {
            return None;
        }
        let b = self.0.trailing_zeros();
        self.0 &= self.0 - 1;
        Some(b)
    }
}

/// `IdSet` 的升序迭代器
pub enum IdIter<'a> {
    Sparse(std::slice::Iter<'a, u32>),
    Dense { words: &'a [u64], sum: &'a [u64], si: usize, sbits: u64, w: usize, bits: u64 },
}

impl Iterator for IdIter<'_> {
    type Item = u32;
    #[inline]
    fn next(&mut self) -> Option<u32> {
        match self {
            IdIter::Sparse(it) => it.next().copied(),
            IdIter::Dense { words, sum, si, sbits, w, bits } => loop {
                if *bits != 0 {
                    let b = bits.trailing_zeros();
                    *bits &= *bits - 1;
                    return Some((*w * 64) as u32 + b);
                }
                if *sbits != 0 {
                    let b = sbits.trailing_zeros() as usize;
                    *sbits &= *sbits - 1;
                    *w = (*si - 1) * 64 + b;
                    *bits = words[*w];
                    continue;
                }
                if *si >= sum.len() {
                    return None;
                }
                *sbits = sum[*si];
                *si += 1;
            },
        }
    }
}

impl<'a> IntoIterator for &'a IdSet {
    type Item = u32;
    type IntoIter = IdIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// 两个升序去重数组的归并（升序去重）
fn merge_sorted(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut out = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        match a[i].cmp(&b[j]) {
            std::cmp::Ordering::Less => {
                out.push(a[i]);
                i += 1;
            }
            std::cmp::Ordering::Greater => {
                out.push(b[j]);
                j += 1;
            }
            std::cmp::Ordering::Equal => {
                out.push(a[i]);
                i += 1;
                j += 1;
            }
        }
    }
    out.extend_from_slice(&a[i..]);
    out.extend_from_slice(&b[j..]);
    out
}

impl FromIterator<u32> for IdSet {
    fn from_iter<I: IntoIterator<Item = u32>>(it: I) -> Self {
        let mut v: Vec<u32> = it.into_iter().collect();
        v.sort_unstable();
        v.dedup();
        IdSet::from_sorted(v)
    }
}

impl Extend<u32> for IdSet {
    fn extend<I: IntoIterator<Item = u32>>(&mut self, it: I) {
        let o: IdSet = it.into_iter().collect();
        self.union_with(&o);
    }
}

impl std::convert::From<[u32; 1]> for IdSet {
    fn from(a: [u32; 1]) -> Self {
        IdSet(Repr::Sparse(a.to_vec()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(a: &[u32], b: &[u32]) -> Vec<u32> {
        let mut v: Vec<u32> = a.iter().chain(b).copied().collect();
        v.sort_unstable();
        v.dedup();
        v
    }

    /// 内联宽度：类型集（两个 IdSet）是流图节点表的主体
    #[test]
    fn inline_width() {
        assert_eq!(std::mem::size_of::<IdSet>(), 24);
    }

    /// 并 / 差 / 子集 / 插入与朴素实现一致（覆盖稀疏、稠密、宽距稀疏（跨形态含跨度判定）、
    /// 升格与降格各路径）
    #[test]
    fn ops_match_naive() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut rnd = |m: u32| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed % u64::from(m)) as u32
        };
        for round in 0..900 {
            let na = rnd(300) as usize;
            let nb = rnd(if round % 2 == 0 { 12 } else { 300 }) as usize;
            // 跨度三档：近距（多数集合稠密）、中距（升格线上下）、宽距（元素数百、id 空间数十万——
            // 逐上下文类型集的典型形态，覆盖保持稀疏 / 跨度爆炸降格）
            let span = 50 + rnd(match round % 4 {
                0 => 200,
                1 => 20000,
                2 => 600000,
                _ => 260000,
            });
            let av: Vec<u32> = (0..na).map(|_| rnd(span)).collect();
            let bv: Vec<u32> = (0..nb).map(|_| rnd(span)).collect();
            let a: IdSet = av.iter().copied().collect();
            let b: IdSet = bv.iter().copied().collect();
            let (sa, sb) = (naive(&av, &[]), naive(&bv, &[]));
            assert_eq!(a.iter().collect::<Vec<_>>(), sa);
            assert_eq!(a.len(), sa.len());
            let mut u = a.clone();
            u.union_with(&b);
            let want = naive(&av, &bv);
            assert_eq!(u.iter().collect::<Vec<_>>(), want);
            assert_eq!(u.len(), want.len());
            assert!(want.iter().all(|x| u.contains(x)));
            assert!(!u.contains(&(span + 1)));
            assert_eq!(u, IdSet::from_sorted(want.clone()));
            // 形态按元素数与跨度判定（宽距集合保持稀疏）
            assert_eq!(u.dense(), dense_worth(u.len(), *want.last().unwrap_or(&0)));
            let mut ins = a.clone();
            for &x in &bv {
                ins.insert(x);
            }
            assert_eq!(ins, u);
            // 跨度爆炸降格：近距稠密集合并入远距元素后仍与朴素实现一致
            let mut wide = a.clone();
            wide.insert(span + 7);
            let mut wv = sa.clone();
            wv.push(span + 7);
            wv.sort_unstable();
            wv.dedup();
            assert_eq!(wide.iter().collect::<Vec<_>>(), wv);
            assert_eq!(b.is_subset(&a), sb.iter().all(|x| sa.contains(x)));
            assert!(b.is_subset(&u) && a.is_subset(&u));
            let d = b.minus(&a);
            let dw: Vec<u32> = sb.iter().copied().filter(|x| !sa.contains(x)).collect();
            assert_eq!(d.iter().collect::<Vec<_>>(), dw);
            assert_eq!(d.len(), dw.len());
        }
    }
}

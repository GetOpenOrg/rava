//! `IdSet`：u32 id 的有序集合，稀疏 / 稠密两种形态。
//!
//! - 稀疏（元素 < `DENSE_AT`）：有序 `Vec<u32>`，多数集合只有几个到几十个元素，二分 / 归并最快；
//! - 稠密（元素 ≥ `DENSE_AT`）：只存位图（`words`）与非零字摘要（`sum`，每位对应一个非零字）。
//!   插入 O(1)（不再为保持有序向量逐个搬移），差 / 并 / 子集按字运算，遍历按摘要跳过零字。
//!
//! 形态只由元素数决定（没有删除操作），遍历一律升序——与旧的「有序向量 + 位图索引」形态逐元素同序。

/// 稀疏 → 稠密的元素数阈值
pub(super) const DENSE_AT: usize = 64;

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

impl Dense {
    /// 置位（不维护计数）；返回是否新置
    #[inline]
    fn set_bit(&mut self, x: u32) -> bool {
        let w = x as usize / 64;
        if self.words.len() <= w {
            self.words.resize(w + 1, 0);
            self.sum.resize(w / 64 + 1, 0);
        }
        let b = 1u64 << (x % 64);
        if self.words[w] & b != 0 {
            return false;
        }
        self.words[w] |= b;
        self.sum[w / 64] |= 1 << (w % 64);
        true
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
    /// 由升序去重的元素构造
    pub(super) fn from_sorted(v: Vec<u32>) -> IdSet {
        if v.len() >= DENSE_AT {
            IdSet(Repr::Dense(Box::new(Dense::from_sparse(&v))))
        } else {
            IdSet(Repr::Sparse(v))
        }
    }
    pub fn insert(&mut self, x: u32) -> bool {
        match &mut self.0 {
            Repr::Dense(d) => {
                let new = d.set_bit(x);
                d.n += usize::from(new);
                new
            }
            Repr::Sparse(v) => match v.binary_search(&x) {
                Ok(_) => false,
                Err(i) => {
                    v.insert(i, x);
                    if v.len() >= DENSE_AT {
                        let d = Dense::from_sparse(v);
                        self.0 = Repr::Dense(Box::new(d));
                    }
                    true
                }
            },
        }
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
            // o 元素不少于 self，故也是稠密形态
            (Repr::Dense(a), Repr::Dense(b)) => a.nz_words().all(|w| a.words[w] & !b.word(w) == 0),
            // 形态只由元素数决定，稠密集合的超集必为稠密；此臂仅作完备
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
                let mut d = b.clone();
                for &x in a.iter() {
                    let new = d.set_bit(x);
                    d.n += usize::from(new);
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
            }
            (Repr::Dense(a), Repr::Sparse(b)) => {
                for &x in b {
                    let new = a.set_bit(x);
                    a.n += usize::from(new);
                }
            }
            (Repr::Sparse(a), Repr::Sparse(b)) => {
                // 两侧稀疏：有序归并
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
                *self = IdSet::from_sorted(out);
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

    /// 并 / 差 / 子集 / 插入与朴素实现一致（覆盖稀疏、稠密、跨形态各路径）
    #[test]
    fn ops_match_naive() {
        let mut seed = 0x2545_f491_4f6c_dd1du64;
        let mut rnd = |m: u32| {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed % u64::from(m)) as u32
        };
        for round in 0..600 {
            let na = rnd(300) as usize;
            let nb = rnd(if round % 2 == 0 { 12 } else { 300 }) as usize;
            let span = 50 + rnd(if round % 3 == 0 { 200 } else { 20000 });
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
            assert_eq!(u.dense(), u.len() >= DENSE_AT);
            let mut ins = a.clone();
            for &x in &bv {
                ins.insert(x);
            }
            assert_eq!(ins, u);
            assert_eq!(b.is_subset(&a), sb.iter().all(|x| sa.contains(x)));
            assert!(b.is_subset(&u) && a.is_subset(&u));
            let d = b.minus(&a);
            let dw: Vec<u32> = sb.iter().copied().filter(|x| !sa.contains(x)).collect();
            assert_eq!(d.iter().collect::<Vec<_>>(), dw);
            assert_eq!(d.len(), dw.len());
        }
    }
}

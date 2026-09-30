//! 类型集与内部哈希：`TypeSet`（确定类型 + open）、`IdSet`（稀疏 / 位图双形态）、FxHash 表别名。

use super::*;

/// 整数键为主的内部表用的快速哈希（FxHash 乘法混合；遍历顺序不参与任何输出）
#[derive(Default, Clone, Copy)]
pub struct FxHasher(u64);

impl Hasher for FxHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.add(u64::from(b));
        }
    }
    fn write_u8(&mut self, i: u8) {
        self.add(u64::from(i));
    }
    fn write_u16(&mut self, i: u16) {
        self.add(u64::from(i));
    }
    fn write_u32(&mut self, i: u32) {
        self.add(u64::from(i));
    }
    fn write_u64(&mut self, i: u64) {
        self.add(i);
    }
    fn write_usize(&mut self, i: usize) {
        self.add(i as u64);
    }
    fn finish(&self) -> u64 {
        self.0
    }
}

impl FxHasher {
    #[inline]
    pub(super) fn add(&mut self, i: u64) {
        self.0 = (self.0.rotate_left(5) ^ i).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

pub type HashMap<K, V> = std::collections::HashMap<K, V, BuildHasherDefault<FxHasher>>;
pub type HashSet<K> = std::collections::HashSet<K, BuildHasherDefault<FxHasher>>;

// ── 类型集 ──────────────────────────────────────────────────────────────────

#[derive(Default, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TypeSet {
    pub classes: IdSet,
    pub open: IdSet,
}

/// 有序去重的 id 集合。多数集合只有几个到几十个元素，有序 Vec 最快；
/// 元素超过 `DENSE_AT` 后附带位图成员索引：差分传播里「小增量 \ 大集合」按增量逐个查位图，
/// 小增量并入大集合按位插入，均与大集合规模无关
#[derive(Debug, Clone, Default)]
pub struct IdSet(pub(super) Vec<u32>, pub(super) Vec<u64>);

pub(super) const DENSE_AT: usize = 64;

impl PartialEq for IdSet {
    fn eq(&self, o: &IdSet) -> bool {
        self.0 == o.0
    }
}
impl Eq for IdSet {}
impl std::hash::Hash for IdSet {
    fn hash<H: std::hash::Hasher>(&self, h: &mut H) {
        self.0.hash(h);
    }
}

impl IdSet {
    pub(super) fn from_sorted(v: Vec<u32>) -> IdSet {
        let mut s = IdSet(v, Vec::new());
        s.reindex();
        s
    }
    pub(super) fn reindex(&mut self) {
        if self.0.len() < DENSE_AT {
            self.1 = Vec::new();
            return;
        }
        let max = *self.0.last().unwrap() as usize;
        let mut b = vec![0u64; max / 64 + 1];
        for &x in &self.0 {
            b[x as usize / 64] |= 1 << (x % 64);
        }
        self.1 = b;
    }
    pub(super) fn set_bit(&mut self, x: u32) {
        let w = x as usize / 64;
        if self.1.len() <= w {
            self.1.resize(w + 1, 0);
        }
        self.1[w] |= 1 << (x % 64);
    }
    pub fn insert(&mut self, x: u32) -> bool {
        if !self.1.is_empty() && self.contains(&x) {
            return false;
        }
        match self.0.binary_search(&x) {
            Ok(_) => false,
            Err(i) => {
                self.0.insert(i, x);
                if !self.1.is_empty() {
                    self.set_bit(x);
                } else if self.0.len() >= DENSE_AT {
                    self.reindex();
                }
                true
            }
        }
    }
    pub fn contains(&self, x: &u32) -> bool {
        if !self.1.is_empty() {
            let w = *x as usize / 64;
            return w < self.1.len() && self.1[w] >> (x % 64) & 1 != 0;
        }
        self.0.binary_search(x).is_ok()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn iter(&self) -> std::slice::Iter<'_, u32> {
        self.0.iter()
    }
    /// 有序追加（调用方保证 x 大于现有全部元素）
    pub(super) fn push_max(&mut self, x: u32) {
        debug_assert!(self.0.last().is_none_or(|&l| l < x));
        self.0.push(x);
        if !self.1.is_empty() {
            self.set_bit(x);
        } else if self.0.len() >= DENSE_AT {
            self.reindex();
        }
    }
    /// self \ o
    pub(super) fn minus(&self, o: &IdSet) -> IdSet {
        if o.is_empty() {
            return self.clone();
        }
        // 两侧都有位图且字数少于元素数：按字求差（差为空时只扫字）
        if !self.1.is_empty() && !o.1.is_empty() && self.1.len() < self.0.len() {
            let word = |w: usize| self.1[w] & !o.1.get(w).copied().unwrap_or(0);
            let Some(k) = (0..self.1.len()).find(|&w| word(w) != 0) else { return IdSet::default() };
            let mut out = Vec::new();
            for w in k..self.1.len() {
                let mut x = word(w);
                while x != 0 {
                    out.push((w * 64) as u32 + x.trailing_zeros());
                    x &= x - 1;
                }
            }
            return IdSet::from_sorted(out);
        }
        let (a, b) = (&self.0, &o.0);
        // o 有位图或远大于 self：逐个查成员；否则两侧有序归并。差为空（重复并入的常态）时不分配
        if !o.1.is_empty() || a.len() * 16 < b.len() {
            let Some(k) = a.iter().position(|x| !o.contains(x)) else { return IdSet::default() };
            let mut out = Vec::with_capacity(a.len() - k);
            out.extend(a[k..].iter().copied().filter(|x| !o.contains(x)));
            return IdSet::from_sorted(out);
        }
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
        IdSet::from_sorted(out)
    }
    /// 并入 o（o 小时逐个插入，否则有序归并）
    pub(super) fn union_with(&mut self, o: &IdSet) {
        if o.is_empty() {
            return;
        }
        if self.is_empty() {
            *self = o.clone();
            return;
        }
        if o.len() * 16 < self.len() {
            for &x in &o.0 {
                self.insert(x);
            }
            return;
        }
        let (a, b) = (&self.0, &o.0);
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

impl<'a> IntoIterator for &'a IdSet {
    type Item = &'a u32;
    type IntoIter = std::slice::Iter<'a, u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl IntoIterator for IdSet {
    type Item = u32;
    type IntoIter = std::vec::IntoIter<u32>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
        IdSet(a.to_vec(), Vec::new())
    }
}

impl TypeSet {
    pub(super) fn exact(id: u32) -> TypeSet {
        TypeSet { classes: [id].into(), open: IdSet::default() }
    }
    pub(super) fn open(id: u32) -> TypeSet {
        TypeSet { classes: IdSet::default(), open: [id].into() }
    }
    pub(super) fn add_all(&mut self, o: &TypeSet) -> bool {
        let n = self.classes.len() + self.open.len();
        self.classes.union_with(&o.classes);
        self.open.union_with(&o.open);
        n != self.classes.len() + self.open.len()
    }
    pub(super) fn is_empty(&self) -> bool {
        self.classes.is_empty() && self.open.is_empty()
    }
}

//! 类型集与内部哈希：`TypeSet`（确定类型 + open）、`IdSet`（稀疏 / 位图双形态）、FxHash 表别名。

use super::*;

/// 整数键为主的内部表用的快速哈希（FxHash 乘法混合；遍历顺序不参与任何输出）
#[derive(Clone, Copy)]
pub struct FxHasher(u64);

static HASH_SEED: std::sync::OnceLock<u64> = std::sync::OnceLock::new();

/// 设置内部表哈希初值（`rava closure --hash-seed N`；缺省 0）：换种子即换各内部表的遍历顺序，
/// 用于检验分析结果与处理顺序无关（双种子集合对照）。须在首个内部表建立前调用，之后调用无效
pub fn set_hash_seed(seed: u64) {
    let _ = HASH_SEED.set(seed);
}

fn hash_seed() -> u64 {
    *HASH_SEED.get_or_init(|| 0)
}

impl Default for FxHasher {
    #[inline]
    fn default() -> Self {
        FxHasher(hash_seed())
    }
}

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
    #[inline]
    pub(super) fn is_subset_of(&self, o: &TypeSet) -> bool {
        self.classes.is_subset(&o.classes) && self.open.is_subset(&o.open)
    }
}

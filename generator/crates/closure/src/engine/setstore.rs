//! 类型流图节点类型集的存储：内容哈希驻留（计划 2026-09-30-closure-analyzer-performance.md P3「TypeSet 哈希驻留」）。
//!
//! 传播到不动点时大量节点持有相同的类型集（同一批值沿调用链 / 字段 / 数组元素层层传递：DeepCopy 17.7 万个非空
//! 节点集只有约 1.2 万种内容）。每种内容只存一份，节点按 `Rc` 共享；写入时复制，写后按新内容再驻留。
//!
//! 内容哈希取元素哈希之和（与插入顺序无关），增长时只加新增元素的哈希，不重扫整集合。
//! 驻留表只影响存储共享，不影响任何集合内容与遍历顺序。

use super::*;

/// 驻留表清理的表长下限；清理后的再清理阈值 = 清理后表长 × (1 + 1/`SWEEP_GAIN`)（§4.16：清理只回收
/// 引用数为 1 的死条目，旧翻倍阈值让死条目积到与活内容等量——uniq 计数 15万↔25万 波动即此；
/// 收紧到 +1/4 后死条目上界从 1.0× 降到 0.25×，retain O(表长) 在数十万条目级微秒到毫秒）
const SWEEP_AT: usize = 4096;
/// 清理后再清理的表长增额（1/`SWEEP_GAIN`）
const SWEEP_GAIN: usize = 4;

/// 元素哈希（open 与 classes 区分）
#[inline]
fn elem_hash(x: u32, open: bool) -> u64 {
    let v = (u64::from(x) << 1 | u64::from(open)).wrapping_add(1);
    let m = u128::from(v) * 0x9e37_79b9_7f4a_7c15;
    (m as u64) ^ (m >> 64) as u64
}

/// 内容哈希：元素哈希之和
fn content_hash(t: &TypeSet) -> u64 {
    let c = t.classes.iter().fold(0u64, |h, x| h.wrapping_add(elem_hash(x, false)));
    t.open.iter().fold(c, |h, x| h.wrapping_add(elem_hash(x, true)))
}

#[derive(Default)]
pub(super) struct SetStore {
    /// 按节点序号的类型集（空 = 尚无值，共享 `empty`）
    sets: Vec<Rc<TypeSet>>,
    /// 各类型集的内容哈希
    hashes: Vec<u64>,
    empty: Rc<TypeSet>,
    /// 驻留表：(内容哈希, 共享的类型集)；表自身持一份引用，引用数为 1 即无节点持有，清理时移除
    table: hashbrown::HashTable<(u64, Rc<TypeSet>)>,
    /// 下次清理的表长
    sweep_at: usize,
    /// 观测：写入次数 / 写后命中已有内容的次数 / 清理次数
    pub(super) stats: [u64; 3],
}

impl SetStore {
    pub(super) fn push_empty(&mut self) {
        self.sets.push(self.empty.clone());
        self.hashes.push(0);
    }
    #[inline]
    pub(super) fn get(&self, i: usize) -> &TypeSet {
        &self.sets[i]
    }
    #[inline]
    pub(super) fn get_rc(&self, i: usize) -> Rc<TypeSet> {
        self.sets[i].clone()
    }
    /// 第 i 个类型集并入 delta（delta 与原集合不相交）。增长后的内容已驻留时直接共享，不复制原集合
    pub(super) fn grow(&mut self, i: usize, delta: &TypeSet) {
        self.stats[0] += 1;
        let h = self.hashes[i];
        let h2 = h.wrapping_add(content_hash(delta));
        self.hashes[i] = h2;
        let cur = &self.sets[i];
        let (nc, no) = (cur.classes.len() + delta.classes.len(), cur.open.len() + delta.open.len());
        let hit = self.table.find(h2, |e| {
            e.1.classes.len() == nc && e.1.open.len() == no && delta.is_subset_of(&e.1) && cur.is_subset_of(&e.1)
        });
        if let Some(e) = hit {
            self.stats[1] += 1;
            self.sets[i] = e.1.clone();
            return;
        }
        // 只有本节点与驻留表持有：先移出表，原地增长免复制
        if Rc::strong_count(cur) == 2 {
            let p = Rc::as_ptr(cur);
            if let Ok(e) = self.table.find_entry(h, |e| Rc::as_ptr(&e.1) == p) {
                e.remove();
            }
        }
        Rc::make_mut(&mut self.sets[i]).add_all(delta);
        self.register(i);
    }
    /// 第 i 个类型集整体置为 s
    pub(super) fn put(&mut self, i: usize, s: TypeSet) {
        self.hashes[i] = content_hash(&s);
        self.sets[i] = Rc::new(s);
        self.intern(i);
    }
    /// 取出第 i 个类型集，原处置空
    pub(super) fn take(&mut self, i: usize) -> Rc<TypeSet> {
        self.hashes[i] = 0;
        std::mem::replace(&mut self.sets[i], self.empty.clone())
    }
    /// 第 i 个类型集按内容驻留：已有同内容的共享之，否则登记
    fn intern(&mut self, i: usize) {
        if self.sets[i].is_empty() {
            self.sets[i] = self.empty.clone();
            return;
        }
        let h = self.hashes[i];
        let s = &self.sets[i];
        if let Some(e) = self.table.find(h, |e| Rc::ptr_eq(&e.1, s) || *e.1 == **s) {
            if !Rc::ptr_eq(&e.1, s) {
                self.stats[1] += 1;
                self.sets[i] = e.1.clone();
            }
            return;
        }
        self.register(i);
    }
    /// 登记第 i 个类型集（调用方已确认表中无同内容）
    fn register(&mut self, i: usize) {
        if self.sets[i].is_empty() {
            self.sets[i] = self.empty.clone();
            return;
        }
        let h = self.hashes[i];
        self.table.insert_unique(h, (h, self.sets[i].clone()), |e| e.0);
        if self.table.len() >= self.sweep_at.max(SWEEP_AT) {
            self.stats[2] += 1;
            self.table.retain(|e| Rc::strong_count(&e.1) > 1);
            self.sweep_at = self.table.len() + self.table.len() / SWEEP_GAIN;
        }
    }
    /// 驻留表中的不同内容份数（观测）
    pub(super) fn unique(&self) -> usize {
        self.table.len()
    }
    /// 全部节点类型集的原始字节估计（观测，含共享的重复计数）：返回（字节和, 非空节点数）
    pub(super) fn census(&self) -> (usize, usize) {
        let mut b = 0usize;
        let mut n = 0usize;
        for s in &self.sets {
            if !s.is_empty() {
                n += 1;
                b += s.classes.heap_bytes() + s.open.heap_bytes() + std::mem::size_of::<TypeSet>();
            }
        }
        (b, n)
    }
    /// uniq 内容的表征画像（census uniq 行）：逐驻留表条目累计，返回（classes, open, Rc 头与表槽字节, uniq 数）。
    /// 真实驻留字节 = classes.bytes + open.bytes + 头字节（原始 `census` 按节点重复计数共享内容）
    pub(super) fn uniq_profile(&self) -> (super::idset::ReprProfile, super::idset::ReprProfile, u64, u64) {
        let (mut pc, mut po) = (super::idset::ReprProfile::default(), super::idset::ReprProfile::default());
        let mut ovh = 0u64;
        for e in self.table.iter() {
            e.1.classes.profile(&mut pc);
            e.1.open.profile(&mut po);
            // Rc 头（强弱计数 16B）+ TypeSet 内联 48B + 驻留表槽（(u64, Rc) 16B × 装载因子）
            ovh += (16 + std::mem::size_of::<TypeSet>() + 18) as u64;
        }
        (pc, po, ovh, self.table.len() as u64)
    }
}

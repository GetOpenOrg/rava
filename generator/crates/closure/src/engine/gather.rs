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

/// 对象集内容驻留池（§4.19）：同内容的 `Rc<[u32]>` 活分配只此一份——站点累计集（`gather_last` 族）与
/// 汇集节点键（`gather_ids`）共享同一分配（§4.17 归因 gathers 为 sb3 死前第一大项，驻留主体即对象集；
/// 各克隆上下文的同形站点以相同次序累计出相同集合——内容重复度高）。哈希为元素哈希之和，桶内
/// 内容比对（碰撞安全）；弱引用记账，死分配随桶清理出表。只影响存储共享，不影响任何集合内容
#[derive(Default)]
pub(super) struct SetPool {
    /// 内容哈希 → 活分配弱引用
    ix: HashMap<u64, Vec<std::rc::Weak<[u32]>>>,
    /// 观测：驻留命中（复用既有分配）/ 新登记
    pub(super) hits: u64,
    pub(super) news: u64,
}

impl SetPool {
    /// v 驻留：已有同内容活分配则复用，否则登记 v；返回（分配, 内容哈希）
    pub(super) fn intern(&mut self, v: Vec<u32>) -> (Rc<[u32]>, u64) {
        let h = v.iter().fold(0u64, |a, &x| a.wrapping_add(u32_elem_hash(x)));
        let bucket = self.ix.entry(h).or_default();
        bucket.retain(|w| w.strong_count() > 0);
        for w in bucket.iter() {
            let rc = w.upgrade().unwrap();
            if rc[..] == v[..] {
                self.hits += 1;
                return (rc, h);
            }
        }
        self.news += 1;
        let rc: Rc<[u32]> = v.into();
        bucket.push(Rc::downgrade(&rc));
        (rc, h)
    }
    /// 深堆字节（census deep3，只读）：索引桶
    pub(super) fn deep_bytes(&self) -> usize {
        self.ix.capacity() * 40 * 9 / 7 + self.ix.values().map(|v| 24 + v.len() * 16).sum::<usize>()
    }
    /// 活分配数（观测）
    pub(super) fn live(&self) -> usize {
        self.ix.values().map(|v| v.iter().filter(|w| w.strong_count() > 0).count()).sum()
    }
}

/// 汇集节点键的对象集项：按预存内容哈希入桶、内容判等（§4.19——`Rc<[u32]>` 的缺省 `Hash` 对整段
/// 走 SipHash，`gather_ids` 每次查找 O(n)；驻留池保证同内容至多一份活分配，指针先试后内容兜底）
#[derive(Debug)]
pub(super) struct SetKey(pub(super) Rc<[u32]>, u64);

impl PartialEq for SetKey {
    fn eq(&self, o: &Self) -> bool {
        self.1 == o.1 && (Rc::ptr_eq(&self.0, &o.0) || self.0[..] == o.0[..])
    }
}
impl Eq for SetKey {}
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

    /// 站点原有记录 last（汇集节点, 累计对象）并入 objs（升序，可含已接过的）后的新记录（不变时 None）与汇集节点；
    /// 槽位节点与汇集节点间按 f 过滤
    fn gather_step(&mut self, last: Option<(u32, Rc<[u32]>)>, slot: Slot, f: Filt, objs: &[u32], put: bool) -> (Option<(u32, Rc<[u32]>)>, Option<u32>) {
        let (g0, prev) = last.unwrap_or((NO_GATHER, Rc::from([])));
        // 两段均升序：归并
        let mut full: Vec<u32> = Vec::with_capacity(prev.len() + objs.len());
        let (mut i, mut j) = (0, 0);
        while i < prev.len() && j < objs.len() {
            let (p, o) = (prev[i], objs[j]);
            full.push(p.min(o));
            i += usize::from(p <= o);
            j += usize::from(o <= p);
        }
        full.extend_from_slice(&prev[i..]);
        full.extend_from_slice(&objs[j..]);
        // 无新增对象：沿用原汇集节点（重跑的常态）
        if full.len() == prev.len() && g0 != NO_GATHER {
            return (None, Some(g0));
        }
        if full.len() < GATHER_MIN {
            // 小集合也驻留：站点数远多于汇集节点数，同名小集合（未达阈值前的公共前缀）跨站点重复
            let (full, _) = self.gather_pool.intern(full);
            return (Some((NO_GATHER, full)), None);
        }
        // 内容驻留（§4.19）：同内容集合只此一份分配——站点累计集与汇集节点键共享
        let (full, kh) = self.gather_pool.intern(full);
        let g = match self.gather_ids.get(&(slot, put, SetKey(full.clone(), kh))) {
            Some(&g) => g,
            None => {
                let g = self.gathers.len() as u32;
                self.gathers.push((slot, full.len() as u32, put));
                self.gather_ids.insert((slot, put, SetKey(full.clone(), kh)), g);
                // 新集合 = 原汇集节点的集合（子集）∪ 其余对象；无原节点时全部对象逐个接边
                let rest: Vec<u32> = if g0 == NO_GATHER {
                    full.to_vec()
                } else {
                    let fc = self.filt_chain(f);
                    if put {
                        self.flow(Node::G(g), Node::G(g0), fc);
                    } else {
                        self.flow(Node::G(g0), Node::G(g), fc);
                    }
                    full.iter().copied().filter(|o| prev.binary_search(o).is_err()).collect()
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
}

#[cfg(test)]
mod tests {
    use super::{SetKey, SetPool};
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;

    /// 内容驻留语义：同内容复用同一分配、异内容各自登记；SetKey 判等与哈希口径一致
    #[test]
    fn setpool_intern_semantics() {
        let mut p = SetPool::default();
        let (a, ha) = p.intern(vec![1, 3, 5]);
        let (b, hb) = p.intern(vec![1, 3, 5]);
        assert_eq!(ha, hb);
        assert!(Rc::ptr_eq(&a, &b), "同内容必须复用同一分配");
        let (c, hc) = p.intern(vec![1, 3]);
        assert_ne!(ha, hc);
        assert!(!Rc::ptr_eq(&a, &c));
        // 内容哈希与顺序无关的等价集（升序输入为常态，同内容不同分配路径只在测试构造）
        let (d, _) = p.intern(vec![1, 3, 5]);
        assert!(Rc::ptr_eq(&a, &d));
        // SetKey：指针相等快速路径 + 内容兜底（哈希一致）
        let k1 = SetKey(a.clone(), ha);
        let k2 = SetKey(b.clone(), hb);
        let k3 = SetKey(c.clone(), hc);
        assert_eq!(k1, k2);
        assert_ne!(k1, k3);
        let h = |k: &SetKey| {
            let mut s = DefaultHasher::new();
            k.hash(&mut s);
            s.finish()
        };
        assert_eq!(h(&k1), h(&k2));
        // 死分配出桶：drop 后再 intern 同内容得到新分配（桶内清理）
        drop((c, k3));
        let (e, he) = p.intern(vec![1, 3]);
        assert_eq!(he, hc);
        assert_ne!(p.live(), 0);
        let _ = e;
    }
}

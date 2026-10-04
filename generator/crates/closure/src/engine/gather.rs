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

impl Slot {
    /// 分配点 o 的该槽位节点
    fn node(self, o: u32) -> Node {
        match self {
            Slot::Field(fi) => Node::O(o, fi),
            Slot::Elem(p) => Node::E(o, p),
        }
    }
}

impl Engine<'_> {
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

    /// 站点累计对象（原有 ∪ objs）对应的汇集节点（槽位节点与汇集节点间按 f 过滤）；累计不足 `GATHER_MIN` 时 None（调用方逐对象接边）
    fn gather_node(&mut self, m: usize, off: u32, slot: Slot, f: u32, objs: &[u32], put: bool) -> Option<u32> {
        let last = self.gather_last.get(&m).and_then(|s| s.get(&(off, slot))).cloned();
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
            return Some(g0);
        }
        if full.len() < GATHER_MIN {
            self.gather_last.entry(m).or_default().insert((off, slot), (NO_GATHER, full.into()));
            return None;
        }
        let full: Rc<[u32]> = full.into();
        let g = match self.gather_ids.get(&(slot, put, full.clone())) {
            Some(&g) => g,
            None => {
                let g = self.gathers.len() as u32;
                self.gathers.push((slot, full.len() as u32, put));
                self.gather_ids.insert((slot, put, full.clone()), g);
                // 新集合 = 原汇集节点的集合（子集）∪ 其余对象；无原节点时全部对象逐个接边
                let rest: Vec<u32> = if g0 == NO_GATHER {
                    full.to_vec()
                } else {
                    if put {
                        self.flow(Node::G(g), Node::G(g0), f);
                    } else {
                        self.flow(Node::G(g0), Node::G(g), f);
                    }
                    full.iter().copied().filter(|o| prev.binary_search(o).is_err()).collect()
                };
                for o in rest {
                    if put {
                        self.flow(Node::G(g), slot.node(o), f);
                    } else {
                        self.flow(slot.node(o), Node::G(g), f);
                    }
                }
                g
            }
        };
        self.gather_last.entry(m).or_default().insert((off, slot), (g, full));
        Some(g)
    }
}

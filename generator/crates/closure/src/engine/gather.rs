//! 引擎：字段读的汇集节点——同一字段、同一抽象对象集合的读取站点共用一个汇集节点。
//!
//! 字节码字段读站点按接收者抽象对象拆分：每个对象的字段节点 `O(o, f)` 各接一条边到站点结果。
//! 同一批对象（如某容器节点类的全部分配点）被大量站点（各克隆上下文里的同一读取）读取时，
//! 边数是站点数 × 对象数，对象字段每次增长都要向全部站点重复推送同一增量。
//!
//! 汇集节点 `G(g)` 按 (字段, 对象集合) 共享：各对象字段节点流入它，它再流向各站点结果；
//! 过滤类型都是字段类型，于是站点收到的集合与逐对象接边相同。站点的对象集合只增不减
//! （重跑只带来新增对象）：集合增长时换接新集合的汇集节点，新节点由原节点（子集）与新增对象的字段节点流入。
//! 对象少于 `GATHER_MIN` 的站点仍逐对象接边。

use super::*;

/// 站点累计对象数达到此数时改经汇集节点
const GATHER_MIN: usize = 4;

/// 无汇集节点（站点仍逐对象接边）
const NO_GATHER: u32 = u32::MAX;

impl Engine<'_> {
    /// 字节码字段读站点 (m, off) 新接上抽象对象 objs（升序、此前未接过）：字段 fi（类型 tid）的值流到 res
    pub(super) fn gather_read(&mut self, m: usize, off: u32, fi: usize, tid: u32, objs: &[u32], res: Node) {
        let last = self.gather_last.get(&m).and_then(|s| s.get(&off)).filter(|x| x.0 == fi).map(|x| (x.1, x.2.clone()));
        let (g0, prev) = last.unwrap_or((NO_GATHER, Rc::from([])));
        let mut full: Vec<u32> = Vec::with_capacity(prev.len() + objs.len());
        full.extend(prev.iter().copied());
        full.extend(objs.iter().copied());
        full.sort_unstable();
        full.dedup();
        if full.len() < GATHER_MIN {
            for &o in objs {
                self.flow(Node::O(o, fi), res, tid);
            }
            self.gather_last.entry(m).or_default().insert(off, (fi, NO_GATHER, full.into()));
            return;
        }
        let full: Rc<[u32]> = full.into();
        let g = match self.gather_ids.get(&(fi, full.clone())) {
            Some(&g) => g,
            None => {
                let g = self.gathers.len() as u32;
                self.gathers.push((fi, full.len() as u32));
                self.gather_ids.insert((fi, full.clone()), g);
                // 新集合 = 原汇集节点的集合（子集）∪ 其余对象；无原节点时全部对象逐个流入
                let rest: Vec<u32> = if g0 == NO_GATHER {
                    full.to_vec()
                } else {
                    self.flow(Node::G(g0), Node::G(g), tid);
                    full.iter().copied().filter(|o| prev.binary_search(o).is_err()).collect()
                };
                for o in rest {
                    self.flow(Node::O(o, fi), Node::G(g), tid);
                }
                g
            }
        };
        self.flow(Node::G(g), res, tid);
        self.gather_last.entry(m).or_default().insert(off, (fi, g, full));
    }
}

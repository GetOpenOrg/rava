//! 引擎：未建模值来源——`null_recv`（接收者恒为 null）判定的证据前提。
//!
//! 类型集为空只在值流完整建模时才等价于「只可能是 null」：字节码里的每个引用值都来自 `new` /
//! 常量 / 字段 / 数组 / 调用返回，流图把它们全部接成边，空集意味着流入的只有 `aconst_null`、
//! 字段 / 数组元素的缺省值或 null 实参。手写体则不同：它经语法不可见的途径（注册表工厂、函数
//! 指针、缓存）构造并交出对象，值池里看不到这些对象——值池喂出的空集是「值流缺失」，不是 null。
//!
//! 判定规则：接收者的每个来源节点都不可经流边从未建模来源到达，才允许按空集判 `null_recv`；
//! 接收者是 null 常量（抽象值 `Null`）恒可判；值未知（`Top`）恒不判。

use super::*;
use crate::absint::Src;

/// 未建模来源可达标记（按流图节点序号，经代表归并）
pub(super) struct Unmodeled {
    marked: Vec<bool>,
}

impl Engine<'_> {
    /// 未建模来源：手写方法的值池 / 产出（含按 locale / 清单种子补入的产出）、手写调用点写回实参数组的
    /// 元素来源，以及无字节码可分析（类缺失）的方法返回值。从这些节点沿流边（不按过滤类型收窄，
    /// 保守）可达的节点均视为「值流可能缺失」
    pub(super) fn unmodeled(&self) -> Unmodeled {
        let g = &self.graph;
        let n = g.node_count();
        let roots = (0..n as u32).filter(|&i| match *g.node_at(i) {
            Node::S(m, k) if k == POOL || k == PROD => matches!(self.methods[m].kind, Kind::Handwritten(_)),
            Node::W(..) => true,
            Node::R(m) => self.methods[m].kind == Kind::Missing,
            _ => false,
        });
        let marked = reach(&g.edges, |i| g.rep(i), roots);
        Unmodeled { marked }
    }

    /// 方法 m 内抽象值 v 作为接收者时，值流是否可能缺失（true = 不得据空集判恒 null）
    pub(super) fn recv_unmodeled(&self, m: usize, v: &V, um: &Unmodeled) -> bool {
        match recv_sources(m, v) {
            None => true,
            Some(ns) => ns.iter().any(|n| self.graph.lookup(n).is_some_and(|i| um.marked[self.graph.rep(i) as usize])),
        }
    }
}

/// 从根（序号）沿出边可达的节点标记（按代表归并；边不按过滤类型收窄）
fn reach(edges: &[Vec<(u32, u32)>], rep: impl Fn(u32) -> u32, roots: impl Iterator<Item = u32>) -> Vec<bool> {
    let mut marked = vec![false; edges.len()];
    let mut work: Vec<u32> = Vec::new();
    for r in roots.map(&rep) {
        if !std::mem::replace(&mut marked[r as usize], true) {
            work.push(r);
        }
    }
    while let Some(r) = work.pop() {
        for &(d, _) in &edges[r as usize] {
            let d = rep(d);
            if !std::mem::replace(&mut marked[d as usize], true) {
                work.push(d);
            }
        }
    }
    marked
}

/// 接收者抽象值的来源节点：null 常量 → 空（无来源，恒可判）；值未知 → None（不作证据）；
/// 字符串字面量不是对象来源节点
fn recv_sources(m: usize, v: &V) -> Option<Vec<Node>> {
    match v {
        V::Null => Some(vec![]),
        V::Ref { src, .. } => Some(
            src.iter()
                .filter_map(|s| match *s {
                    Src::Param(i) => Some(Node::P(m, i)),
                    Src::Site(o) => Some(Node::S(m, o)),
                    Src::Catch(o) => Some(Node::S(m, CATCH | o)),
                    Src::Str(_) => None,
                })
                .collect(),
        ),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reach_follows_edges_through_representatives() {
        // 0 → 1 → 2；3 并入代表 1（3 的出边存于代表）；4 孤立
        let edges = vec![vec![(1, 0)], vec![(2, 0)], vec![], vec![], vec![]];
        let rep = |i: u32| if i == 3 { 1 } else { i };
        assert_eq!(reach(&edges, rep, [0].into_iter()), vec![true, true, true, false, false]);
        assert_eq!(reach(&edges, rep, [3].into_iter()), vec![false, true, true, false, false]);
        assert_eq!(reach(&edges, rep, std::iter::empty()), vec![false; 5]);
    }

    #[test]
    fn recv_sources_by_value_kind() {
        assert_eq!(recv_sources(7, &V::Null), Some(vec![]));
        assert_eq!(recv_sources(7, &V::Top), None);
        let src: crate::absint::Srcs = std::rc::Rc::from([Src::Param(1), Src::Site(9), Src::Catch(4), Src::Str(0)].as_slice());
        let v = V::Ref { ty: None, nonnull: false, src, obj: None };
        assert_eq!(recv_sources(7, &v), Some(vec![Node::P(7, 1), Node::S(7, 9), Node::S(7, CATCH | 4)]));
    }
}

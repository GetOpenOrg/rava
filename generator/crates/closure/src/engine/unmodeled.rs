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

/// 未建模来源可达标记（按流图节点序号，经代表归并）。类型集始终为空的读取点不驻留流图：
/// 它们按虚序号（流图节点数之后）参与派生，无出边
pub(super) struct Unmodeled {
    marked: Vec<bool>,
    absent: HashMap<Node, u32>,
}

impl Engine<'_> {
    /// 未建模来源（手写 / native 值的全部出口）：
    /// - 手写方法（native / 边界 / 内建 / 手写实现对象的伪方法 / VM 钩子）的值池、产出与返回值——
    ///   返回值按 open(返回类型) 建模，Rust 侧构造的对象（静态单例、`T::default()` 填表）不在 G 中，
    ///   open 展开为空集并不说明只可能是 null；
    /// - 手写调用点写回实参数组的元素来源；无字节码可分析（类缺失）的方法返回值；
    /// - open 直接注入点：类型集不经流边并入，来源不可追溯；
    /// - 派生：以值流可能缺失的对象为基址 / 接收者的读取结果——数组元素（数组来自手写 / native，元素由
    ///   Rust 填入，流图里没有元素到读取点的边）、实例字段（Rust 侧构造的对象，字段由 Rust 写入）、
    ///   实例方法返回值（接收者不在 G 中，派发目标与其返回值都不在流图里）。
    /// 从这些节点沿流边（不按过滤类型收窄，保守）可达的节点均视为「值流可能缺失」；派生规则与可达
    /// 传播交替到不动点
    pub(super) fn unmodeled(&self) -> Unmodeled {
        let g = &self.graph;
        let n = g.node_count();
        let roots = (0..n as u32).filter(|&i| match *g.node_at(i) {
            Node::S(m, k) if k == POOL || k == PROD => matches!(self.methods[m].kind, Kind::Handwritten(_)),
            Node::W(..) => true,
            Node::R(m) => matches!(self.methods[m].kind, Kind::Missing | Kind::Handwritten(_)),
            _ => false,
        });
        // open 直接注入点（`add_to` 登记的 `open_inj`）：值以类型集而非流边并入（open 实参 / 接收者的非虚调用、
        // 未知值按声明类型……），其上游来源可能是未建模节点而流图里没有这条边——同样视为未建模来源
        let injected = self.open_inj.keys().filter_map(|x| g.lookup(x));
        let mut marked = vec![false; n];
        let rep = |i: u32| if (i as usize) < n { g.rep(i) } else { i };
        reach(&g.edges, rep, &mut marked, roots.chain(injected));
        // 派生读取点：(读取点代表, 基址 / 接收者值来源代表；None = 值未知)。类型集始终为空的读取点
        // （值流缺失时恰是这种情形：未建模的基址 / 接收者派发不到目标、读不出元素）不在流图里，
        // 取虚序号——否则它的派生标记无处存放，下游以它为接收者的调用会被误判恒 null
        let reads: Vec<(Node, Option<Vec<Node>>)> = self
            .methods
            .values()
            .enumerate()
            .filter_map(|(i, mn)| mn.analysis.as_ref().map(|a| (i, a)))
            .flat_map(|(i, a)| a.events.iter().filter_map(move |(pc, e)| Some((Node::S(i, *pc), recv_sources(i, derived_base(e)?)))))
            .collect();
        let mut absent: HashMap<Node, u32> = HashMap::default();
        for (at, _) in &reads {
            if g.lookup(at).is_none() {
                let k = (n + absent.len()) as u32;
                absent.entry(*at).or_insert(k);
            }
        }
        marked.resize(n + absent.len(), false);
        let idx = |x: &Node| g.lookup(x).map(|i| g.rep(i)).or_else(|| absent.get(x).copied());
        let loads: Vec<(u32, Option<Vec<u32>>)> =
            reads.iter().filter_map(|(at, src)| Some((idx(at)?, src.as_ref().map(|ns| ns.iter().filter_map(&idx).collect())))).collect();
        derive_sites(&g.edges, rep, &mut marked, &loads);
        Unmodeled { marked, absent }
    }

    /// 方法 m 内抽象值 v 作为接收者时，值流是否可能缺失（true = 不得据空集判恒 null）
    pub(super) fn recv_unmodeled(&self, m: usize, v: &V, um: &Unmodeled) -> bool {
        match recv_sources(m, v) {
            None => true,
            Some(ns) => ns.iter().any(|n| {
                let i = self.graph.lookup(n).map(|i| self.graph.rep(i)).or_else(|| um.absent.get(n).copied());
                i.is_some_and(|i| um.marked[i as usize])
            }),
        }
    }
}

/// 结果随基址 / 接收者派生污染的读取事件：数组读取、实例字段读取、实例方法调用（返回其基址 / 接收者值）
fn derived_base(e: &Event) -> Option<&V> {
    match e {
        Event::ArrayLoad { array, .. } => Some(array),
        Event::Field { recv: Some(r), value: None, .. } => Some(r),
        Event::Invoke { opcode, args, .. } if *opcode != classfile::op::INVOKESTATIC => args.first(),
        _ => None,
    }
}

/// 派生规则到不动点：基址 / 接收者值来源有已标记者（或值未知）的读取点并入标记，再沿流边传播
fn derive_sites(edges: &[Vec<(u32, u32)>], rep: impl Fn(u32) -> u32 + Copy, marked: &mut [bool], loads: &[(u32, Option<Vec<u32>>)]) {
    loop {
        let fresh: Vec<u32> = loads
            .iter()
            .filter(|(at, src)| !marked[*at as usize] && src.as_ref().is_none_or(|v| v.iter().any(|&x| marked[x as usize])))
            .map(|(at, _)| *at)
            .collect();
        if fresh.is_empty() {
            return;
        }
        reach(edges, rep, marked, fresh.into_iter());
    }
}

/// 从根（序号）沿出边可达的节点并入标记（按代表归并；边不按过滤类型收窄）
fn reach(edges: &[Vec<(u32, u32)>], rep: impl Fn(u32) -> u32, marked: &mut [bool], roots: impl Iterator<Item = u32>) {
    let mut work: Vec<u32> = Vec::new();
    for r in roots.map(&rep) {
        if !std::mem::replace(&mut marked[r as usize], true) {
            work.push(r);
        }
    }
    while let Some(r) = work.pop() {
        // 虚序号（不驻留流图的读取点）无出边
        for &(d, _) in edges.get(r as usize).into_iter().flatten() {
            let d = rep(d);
            if !std::mem::replace(&mut marked[d as usize], true) {
                work.push(d);
            }
        }
    }
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
        let run = |roots: &[u32]| {
            let mut m = vec![false; 5];
            reach(&edges, rep, &mut m, roots.iter().copied());
            m
        };
        assert_eq!(run(&[0]), vec![true, true, true, false, false]);
        assert_eq!(run(&[3]), vec![false, true, true, false, false]);
        assert_eq!(run(&[]), vec![false; 5]);
        // 增量：已标记的节点不再展开，新根只补未标记部分
        let mut m = vec![false, false, true, false, false];
        reach(&edges, rep, &mut m, [4].into_iter());
        assert_eq!(m, vec![false, false, true, false, true]);
    }

    /// 手写数组读出的元素：读取点并入未建模，并经流边带动下游读取点（链式到不动点）；来源未标记的读取点不动
    #[test]
    fn array_loads_derive_to_fixpoint() {
        // 0 = 手写返回（根）→ 1 = 调用点；读取点 2 读数组 1，2 → 3；读取点 4 读数组 3；读取点 5 读数组 6（未标记）
        let edges = vec![vec![(1, 0)], vec![], vec![(3, 0)], vec![], vec![], vec![], vec![]];
        let rep = |i: u32| i;
        let mut m = vec![false; 7];
        reach(&edges, rep, &mut m, [0].into_iter());
        let loads = vec![(4, Some(vec![3])), (2, Some(vec![1])), (5, Some(vec![6]))];
        derive_sites(&edges, rep, &mut m, &loads);
        assert_eq!(m, vec![true, true, true, true, true, false, false]);
        // 数组值未知：读取点恒并入
        let mut m = vec![false; 7];
        derive_sites(&edges, rep, &mut m, &[(5, None)]);
        assert!(m[5] && !m[6]);
    }

    /// 不驻留流图的读取点（虚序号，无出边）：基址值未知或已标记时并入标记，并作为下游读取点的来源继续派生
    #[test]
    fn absent_read_points_derive() {
        // 流图 0 → 1；虚序号 2 = 以 1 为接收者的调用结果（不在流图），3 = 以 2 为接收者的调用结果，4 = 值未知的接收者
        let edges = vec![vec![(1, 0)], vec![]];
        let rep = |i: u32| i;
        let mut m = vec![false; 5];
        reach(&edges, rep, &mut m, [0].into_iter());
        derive_sites(&edges, rep, &mut m, &[(3, Some(vec![2])), (2, Some(vec![1])), (4, None)]);
        assert_eq!(m, vec![true; 5]);
    }

    /// 派生读取事件：数组读取、实例字段读取、实例调用取基址 / 接收者；静态读取 / 静态调用 / 字段写入不派生
    #[test]
    fn derived_base_by_event_kind() {
        use classfile::op;
        let mref = MemberRef { owner: "p/C".into(), name: "f".into(), desc: "()Lp/C;".into() };
        let r = V::Null;
        let inv = |opcode| Event::Invoke { opcode, mref: mref.clone(), iface: false, args: vec![r.clone()] };
        let fld = |opcode, recv: Option<V>, value: Option<V>| Event::Field { opcode, mref: mref.clone(), recv, value };
        assert!(derived_base(&Event::ArrayLoad { array: r.clone(), index: V::Top }).is_some());
        assert!(derived_base(&inv(op::INVOKEVIRTUAL)).is_some());
        assert!(derived_base(&inv(op::INVOKEINTERFACE)).is_some());
        assert!(derived_base(&inv(op::INVOKESTATIC)).is_none());
        assert!(derived_base(&fld(op::GETFIELD, Some(r.clone()), None)).is_some());
        assert!(derived_base(&fld(op::PUTFIELD, Some(r.clone()), Some(V::Top))).is_none());
        assert!(derived_base(&fld(op::GETSTATIC, None, None)).is_none());
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

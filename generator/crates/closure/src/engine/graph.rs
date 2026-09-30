//! 类型流图的存储：节点驻留为稠密序号，类型集 / 出边 / 待推增量按序号索引。
//!
//! 传播热路径（`drain_flows` 逐边 `add_to`）只做数组索引，不再按 `Node` 哈希；
//! 驻留只改变存储形态，不改变任何遍历 / 入队顺序（输出逐字节不变）。

use super::*;

#[derive(Default)]
pub(super) struct FlowGraph {
    ids: HashMap<Node, u32>,
    nodes: Vec<Node>,
    /// 节点类型集（空 = 尚无值）
    sets: Vec<TypeSet>,
    /// 出边（目标序号, 过滤类型 id），按接边顺序
    pub(super) edges: Vec<Vec<(u32, u32)>>,
    /// 待沿出边推送的新增类型（差分传播；空 = 无待推）
    pub(super) delta: Vec<TypeSet>,
    /// 已在 `fwork` 中
    pub(super) queued: Vec<bool>,
    /// 去重的流边（源, 目标, 过滤类型）
    pub(super) seen: HashSet<(u32, u32, u32)>,
    /// 观测：`add_to` 调用次数 / 有增量的次数 / 并入的元素数
    pub(super) adds: [u64; 3],
}

impl FlowGraph {
    /// 节点序号（首次出现时驻留）
    #[inline]
    pub(super) fn id(&mut self, n: Node) -> u32 {
        if let Some(&i) = self.ids.get(&n) {
            return i;
        }
        let i = self.nodes.len() as u32;
        self.ids.insert(n, i);
        self.nodes.push(n);
        self.sets.push(TypeSet::default());
        self.edges.push(Vec::new());
        self.delta.push(TypeSet::default());
        self.queued.push(false);
        i
    }
    #[inline]
    pub(super) fn node(&self, i: u32) -> Node {
        self.nodes[i as usize]
    }
    #[inline]
    pub(super) fn set(&self, i: u32) -> &TypeSet {
        &self.sets[i as usize]
    }
    #[inline]
    pub(super) fn set_mut(&mut self, i: u32) -> &mut TypeSet {
        &mut self.sets[i as usize]
    }
    /// 节点的类型集（无值时 None）
    pub(super) fn get(&self, n: &Node) -> Option<&TypeSet> {
        let &i = self.ids.get(n)?;
        let s = &self.sets[i as usize];
        (!s.is_empty()).then_some(s)
    }
    /// 有值的节点（诊断；顺序不参与输出）
    pub(super) fn keys(&self) -> impl Iterator<Item = &Node> {
        self.nodes.iter().zip(&self.sets).filter(|(_, s)| !s.is_empty()).map(|(n, _)| n)
    }
    /// 有值的节点及类型集（诊断）
    pub(super) fn iter(&self) -> impl Iterator<Item = (&Node, &TypeSet)> {
        self.nodes.iter().zip(&self.sets).filter(|(_, s)| !s.is_empty())
    }
    /// 有值的节点数
    pub(super) fn len(&self) -> usize {
        self.sets.iter().filter(|s| !s.is_empty()).count()
    }
    /// 全部流边按源节点展开（诊断 / 观测）
    pub(super) fn flow_list(&self) -> Vec<(Node, Vec<(Node, u32)>)> {
        self.edges
            .iter()
            .enumerate()
            .filter(|(_, es)| !es.is_empty())
            .map(|(i, es)| (self.nodes[i], es.iter().map(|&(d, f)| (self.nodes[d as usize], f)).collect()))
            .collect()
    }
}

//! 类型流图的存储：节点驻留为稠密序号，类型集 / 出边 / 待推增量按序号索引。
//!
//! 传播热路径（`drain_flows` 逐边 `add_to`）只做数组索引，不再按 `Node` 哈希；
//! 环合并（`scc.rs`）：只经 Object 过滤边构成的强连通分量在不动点处类型集必然相等，合并为一个代表节点，
//! 类型集 / 出边 / 待推增量只存于代表；成员保留各自的节点身份（读者、钩子仍按成员登记）。

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
    /// 节点 → 所属代表（自身 = 代表）；合并时整组改指，恒为一跳
    rep: Vec<u32>,
    /// 代表 → 成员（仅合并过的代表；含代表自身，升序）
    pub(super) members: HashMap<u32, Vec<u32>>,
    /// 上次环检测以来新接的流边数（触发下次检测）
    pub(super) edges_since: usize,
    /// 观测：环检测次数 / 合并掉的节点数 / 检测耗时 ms
    pub(super) scc_stats: [u64; 3],
    /// 新接边整集合收窄的记忆：(源代表, 过滤类型) → (源集合元素数, 收窄结果)。
    /// 类型集只增不减，元素数相同即集合相同，故元素数即版本号（只记大集合，见 `flow.rs::flow`）
    pub(super) fmemo: HashMap<(u32, u32), (usize, TypeSet)>,
    /// 收窄记忆的堆占用（估算字节）；超预算整表清空
    pub(super) fmemo_bytes: usize,
    /// 观测：`drain_flows` 按（源种类, 目标种类）的推送次数 / 其中有增量的次数（种类序号见 `stats.rs::kind_ix`）
    pub(super) pushes: Vec<[u64; 2]>,
    /// 观测：收窄记忆命中 / 未命中 / 超预算清空次数
    pub(super) fmemo_stats: [u64; 3],
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
        self.rep.push(i);
        i
    }
    /// 节点所属代表
    #[inline]
    pub(super) fn rep(&self, i: u32) -> u32 {
        self.rep[i as usize]
    }
    /// 节点数（含已合并的成员）
    pub(super) fn node_count(&self) -> usize {
        self.nodes.len()
    }
    /// 把代表 b 并入代表 a（调用方负责类型集 / 增量 / 出边的合并）
    pub(super) fn union_into(&mut self, a: u32, b: u32) {
        let mb = self.members.remove(&b).unwrap_or_else(|| vec![b]);
        for &x in &mb {
            self.rep[x as usize] = a;
        }
        let ma = self.members.entry(a).or_insert_with(|| vec![a]);
        ma.extend(mb);
        ma.sort_unstable();
    }
    #[inline]
    pub(super) fn node(&self, i: u32) -> Node {
        self.nodes[i as usize]
    }
    /// 节点（按所属代表）的类型集
    #[inline]
    pub(super) fn set(&self, i: u32) -> &TypeSet {
        &self.sets[self.rep[i as usize] as usize]
    }
    #[inline]
    pub(super) fn set_mut(&mut self, i: u32) -> &mut TypeSet {
        let r = self.rep[i as usize] as usize;
        &mut self.sets[r]
    }
    /// 代表自身存储的类型集（合并用；成员的存储在合并后为空）
    pub(super) fn own_set_mut(&mut self, r: u32) -> &mut TypeSet {
        &mut self.sets[r as usize]
    }
    /// 节点的类型集（无值时 None）
    pub(super) fn get(&self, n: &Node) -> Option<&TypeSet> {
        let &i = self.ids.get(n)?;
        let s = self.set(i);
        (!s.is_empty()).then_some(s)
    }
    /// 有值的节点（诊断；顺序不参与输出）
    pub(super) fn keys(&self) -> impl Iterator<Item = &Node> {
        self.iter().map(|(n, _)| n)
    }
    /// 有值的节点及类型集（诊断）
    pub(super) fn iter(&self) -> impl Iterator<Item = (&Node, &TypeSet)> {
        self.nodes.iter().enumerate().map(|(i, n)| (n, self.set(i as u32))).filter(|(_, s)| !s.is_empty())
    }
    /// 有值的节点数
    pub(super) fn len(&self) -> usize {
        self.iter().count()
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

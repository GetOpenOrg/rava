//! 引擎：类型流——节点类型集、流边、实参来源、字段节点与逃逸。

use super::stats::{kind_ix, KINDS};
use super::*;

/// 新接边整集合收窄走记忆的源集合元素数下限
const FILTER_MEMO_AT: usize = 64;
/// 收窄记忆的内存预算（字节）；超出整表清空（只影响速度，不影响结果）
const FILTER_MEMO_BUDGET: usize = 64 << 20;

fn memo_bytes(s: &TypeSet) -> usize {
    s.classes.heap_bytes() + s.open.heap_bytes() + 64
}

impl<'a> Engine<'a> {
    // ── 类型流 ──────────────────────────────────────────────────────────────

    pub(super) fn set_of(&self, n: Node) -> TypeSet {
        self.graph.get(&n).cloned().unwrap_or_default()
    }

    pub(super) fn add_to(&mut self, n: Node, s: &TypeSet) {
        let i = self.graph.id(n);
        self.add_to_id(i, s);
    }

    /// 节点（序号）并入类型集；新增部分登记待沿流边推送。类型集存于所属代表（见 `scc.rs`）
    fn add_to_id(&mut self, i: u32, s: &TypeSet) {
        let src = std::mem::replace(&mut self.flow_src, diag::NO_SRC);
        let direct = src == diag::NO_SRC;
        self.graph.adds[0] += 1;
        if s.is_empty() {
            return;
        }
        // 无增量快速返回（流边推送的绝大多数）：不取节点、不查暂存。暂存中的空数组元素节点
        // 若已含 s，旧路径暂存 s、补回时同样是无增量，两者等价
        if s.is_subset_of(self.graph.set(i)) {
            return;
        }
        let n = self.graph.node(i);
        // 空数组元素节点不参与合并（`scc.rs`），恒为自身代表
        if let Node::E(x, _) = n {
            if let Some(held) = self.empty_arrays.get_mut(&x) {
                held.entry(n).or_default().add_all(s);
                return;
            }
        }
        let r = self.graph.rep(i);
        let cur = self.graph.set(r);
        let delta = TypeSet {
            classes: s.classes.minus(&cur.classes),
            open: s.open.minus(&cur.open),
        };
        if delta.is_empty() {
            return;
        }
        self.graph.grow(r, &delta);
        self.graph.adds[1] += 1;
        self.graph.adds[2] += (delta.classes.len() + delta.open.len()) as u64;
        if direct && !delta.open.is_empty() {
            self.open_inj.entry(n).or_default().extend(delta.open.iter());
        }
        if self.probes.is_some() {
            self.probe_grown(i, r, src, &delta);
        }
        self.grown(r, &delta);
        // 只沿流边推送新增部分（差分传播）
        self.queue_delta(r, &delta);
    }

    /// 代表 r 登记待推增量
    pub(super) fn queue_delta(&mut self, r: u32, delta: &TypeSet) {
        let ix = r as usize;
        self.graph.delta[ix].add_all(delta);
        if !self.graph.queued[ix] {
            self.graph.queued[ix] = true;
            self.fwork.push_back(r);
        }
    }

    /// 代表 r 的类型集新增 delta：逐成员触发节点钩子（逃逸、自身字段、成员枚举、手写调用点实参、读者重跑）
    pub(super) fn grown(&mut self, r: u32, delta: &TypeSet) {
        match self.graph.members.get(&r) {
            None => self.node_grown(self.graph.node(r), delta),
            Some(ms) => {
                let ms = ms.clone();
                for m in ms {
                    self.node_grown(self.graph.node(m), delta);
                }
            }
        }
    }

    pub(super) fn node_grown(&mut self, n: Node, delta: &TypeSet) {
        if n == Node::Esc {
            self.escape(&delta.classes);
        }
        if let Node::K(g) = n {
            self.kgate_grown(g, delta);
        }
        if let Node::NR(r) = n {
            self.name_write_objs(r, delta);
        }
        if self.self_fields.contains_key(&n) {
            self.self_field_objs(n, delta);
        }
        if self.name_reads.contains_key(&n) {
            self.name_read_objs(n, delta);
        }
        if let Some(&(k, e)) = self.enum_recv.get(&n) {
            self.rpending.push((k, e, delta.clone()));
        }
        // 手写方法调用点的实参新增数组分配点：接上该数组的元素读写
        if let Node::A(s, i) = n {
            let ys: Vec<u32> = delta.classes.iter().filter(|x| self.arrays.contains_key(x)).collect();
            if !ys.is_empty() {
                self.hw_site_arrays(s, i, &ys);
            }
            self.hw_site_fields(s, i, delta);
            if self.hw_reads.get(&s).is_some_and(|r| r.0 == i) {
                self.memory_read(s, delta);
            }
        }
        if let Some(ws) = self.watch.get(&n) {
            for &w in ws {
                if self.in_swork.insert(w) {
                    self.swork.push_back(w);
                }
            }
        }
        if self.mirror_watch.contains_key(&n) {
            self.mirror_grown(n, delta);
        }
        if let Some(cs) = self.call_watch.get(&n) {
            for &c in cs {
                if self.in_cwork.insert(c) {
                    self.cwork.push_back(c);
                }
            }
        }
    }

    /// 流边 src → dst（按 filter 收窄）；立即按当前集合推一次。边接在两端的代表之间；
    /// 同一代表内的 Object 边是空操作（合并只经 Object 边，见 `scc.rs`）
    pub(super) fn flow(&mut self, src: Node, dst: Node, filter: u32) {
        let (si, di) = (self.graph.id(src), self.graph.id(dst));
        let (rs, rd) = (self.graph.rep(si), self.graph.rep(di));
        let objf = filter & NOT_SUB == 0 && self.names[filter as usize].as_ref() == OBJECT;
        if rs == rd && objf {
            return;
        }
        if !self.graph.add_edge(rs, rd, filter) {
            return;
        }
        self.graph.edges_since += 1;
        if self.probes.is_some() {
            self.probe_edge(si, di, filter);
        }
        if self.graph.set(rs).is_empty() {
            return;
        }
        // Object 过滤且目标已含源集合：推送必为无增量，免去整集合克隆（大集合新接边的常态）
        if objf && self.graph.set(rs).is_subset_of(self.graph.set(rd)) {
            return;
        }
        let n = {
            let s = self.graph.set(rs);
            s.classes.len() + s.open.len()
        };
        if objf || n < FILTER_MEMO_AT {
            let s = self.graph.set_rc(rs);
            let out = self.filter(&s, filter);
            self.flow_src = si;
            self.add_to_id(di, &out);
            return;
        }
        // 大集合经同一过滤类型接出多条新边（手写调用点写入槽 → 各数组元素）：收窄结果按集合版本记忆
        let hit = self.graph.fmemo.remove(&(rs, filter));
        if let Some((_, o)) = &hit {
            self.graph.fmemo_bytes -= memo_bytes(o);
        }
        let out = match hit {
            Some((len, out)) if len == n => {
                self.graph.fmemo_stats[0] += 1;
                out
            }
            _ => {
                self.graph.fmemo_stats[1] += 1;
                let s = self.graph.set_rc(rs);
                let out = self.filter(&s, filter);
                out
            }
        };
        self.flow_src = si;
        self.add_to_id(di, &out);
        let b = memo_bytes(&out);
        if self.graph.fmemo_bytes + b > FILTER_MEMO_BUDGET {
            self.graph.fmemo.clear();
            self.graph.fmemo_bytes = 0;
            self.graph.fmemo_stats[2] += 1;
        }
        self.graph.fmemo_bytes += b;
        self.graph.fmemo.insert((rs, filter), (n, out));
    }

    /// 代表 src 沿一条出边推送增量 s；同一过滤类型只收窄一次（`narrowed`），Object 过滤直接推增量本身
    fn push_edge(&mut self, src: u32, dst: u32, f: u32, obj: Option<u32>, s: &TypeSet, narrowed: &mut Vec<(u32, TypeSet)>) {
        if Some(f) == obj {
            self.flow_src = src;
            self.add_to_id(dst, s);
            return;
        }
        let k = match narrowed.iter().position(|x| x.0 == f) {
            Some(k) => k,
            None => {
                let out = self.filter(s, f);
                narrowed.push((f, out));
                narrowed.len() - 1
            }
        };
        let out = std::mem::take(&mut narrowed[k].1);
        self.flow_src = src;
        self.add_to_id(dst, &out);
        narrowed[k].1 = out;
    }

    pub(super) fn drain_flows(&mut self) {
        let obj = self.ids.get(OBJECT).copied();
        if self.graph.pushes.is_empty() {
            self.graph.pushes = vec![[0; 2]; KINDS * KINDS];
        }
        loop {
            if self.scc_due() {
                self.collapse_cycles();
            }
            let Some(i) = self.fwork.pop_front() else { break };
            let ix = i as usize;
            self.graph.queued[ix] = false;
            let s = std::mem::take(&mut self.graph.delta[ix]);
            // 已并入其它代表的节点：增量已随合并转交
            if s.is_empty() || self.graph.rep(i) != i {
                continue;
            }
            // 按下标原地遍历推送前已有的出边（推送中新接的边追加在后，已由 `flow` 按当前集合推过；
            // 边表只增不改，环合并只在两次出队之间）；同一过滤类型只收窄一次，Object 过滤直接推增量本身
            let ne = self.graph.edges[ix].len();
            let mut narrowed: Vec<(u32, TypeSet)> = Vec::new();
            let sk = kind_ix(&self.graph.node(i)) * KINDS;
            let grew0 = self.graph.adds[1];
            for k in 0..ne {
                let (dst, f) = self.graph.edges[ix][k];
                let pk = sk + kind_ix(&self.graph.node(dst));
                let grew = self.graph.adds[1];
                self.push_edge(i, dst, f, obj, &s, &mut narrowed);
                let p = &mut self.graph.pushes[pk];
                p[0] += 1;
                p[1] += u64::from(self.graph.adds[1] != grew);
            }
            let ps = &mut self.graph.push_src[ix];
            ps[0] += 1;
            ps[1] += ne as u64;
            ps[2] += self.graph.adds[1] - grew0;
            let ms = self.graph.members.get(&i).cloned().unwrap_or_else(|| vec![i]);
            for m in ms {
                let n = self.graph.node(m);
                if let Some(ds) = self.mflows.get(&n).cloned() {
                    for (d, op) in ds {
                        self.mirror_into(op, &s, d);
                    }
                }
            }
        }
    }

    /// 镜像流边 src → dst（变换 op）；立即按当前集合推一次
    pub(super) fn mflow(&mut self, src: Node, dst: Node, op: MirrorOp) {
        if !self.mflow_seen.insert((src, dst, op)) {
            return;
        }
        self.mflows.entry(src).or_default().push((dst, op));
        let s = self.set_of(src);
        self.mirror_into(op, &s, dst);
    }

    /// 方法 m 内抽象值 v 的类型来源；未知值按声明类型 open
    pub(super) fn feeds(&mut self, m: usize, v: &V, decl: u32) -> Vec<Feed> {
        // 类镜像子类型判定成立一侧的收窄值：类型流取本方法该偏移处的收窄节点（来源不变，见 `absint/narrow.rs`）
        if let Some(at) = v.mirror_narrowed() {
            return vec![Feed::N(Node::S(m, at))];
        }
        match v {
            V::Str(_) => vec![Feed::S(TypeSet::exact(self.id(STRING)))],
            V::Class(c, _) => {
                let k = self.mirror(c);
                vec![Feed::S(TypeSet::exact(k))]
            }
            V::Top => vec![Feed::S(TypeSet::open(decl))],
            V::Ref { src, .. } => {
                // 字面量来源按序号区分，类型相同：只给一条 String 来源
                let mut lit = false;
                src.iter()
                    .filter_map(|s| match *s {
                        Src::Param(i) => Some(Feed::N(Node::P(m, i))),
                        Src::Site(o) => Some(Feed::N(Node::S(m, o))),
                        Src::Catch(o) => Some(Feed::N(Node::S(m, CATCH | o))),
                        Src::Str(_) => (!std::mem::replace(&mut lit, true)).then(|| Feed::S(TypeSet::exact(self.id(STRING)))),
                    })
                    .collect()
            }
            _ => vec![],
        }
    }

    pub(super) fn feed(&mut self, fs: &[Feed], dst: Node, filter: u32) {
        for f in fs {
            match f {
                Feed::N(n) => self.flow(*n, dst, filter),
                Feed::S(s) => {
                    let out = self.filter(s, filter);
                    self.add_to(dst, &out);
                }
            }
        }
    }

    /// 来源的当前类型集；处理字节码站点时登记该站点为来源节点的读者
    pub(super) fn value_set(&mut self, fs: &[Feed]) -> TypeSet {
        let mut out = TypeSet::default();
        for f in fs {
            match f {
                Feed::N(n) => {
                    if let Some(c) = self.cur_call {
                        self.call_watch.entry(*n).or_default().insert(c);
                    } else if let Some(w) = self.cur_site {
                        self.watch.entry(*n).or_default().insert(w);
                    }
                    if let Some(s) = self.graph.get(n) {
                        out.add_all(s);
                    }
                }
                Feed::S(s) => {
                    out.add_all(s);
                }
            }
        }
        out
    }

    /// 形参类型列表 → 全部取自同一来源的实参
    pub(super) fn args_from(&mut self, desc: &str, f: impl Fn(u32) -> Vec<Feed>) -> Args {
        let Some(md) = parse_method(desc) else { return vec![] };
        md.params.iter().map(|p| self.ptype(p).map(&f)).collect()
    }

    /// 字段节点；首次登记时接上「未知接收者写入 → 字段并集」
    /// 字段节点；首次登记时接上未知接收者视图，以及手写层按名读写的值池（之后登记的名字由 `hw_fields` 补接）
    pub(super) fn field_node(&mut self, key: MemberRef) -> usize {
        if let Some(fi) = self.fields.get_index_of(&key) {
            return fi;
        }
        let (fi, _) = self.fields.insert_full(key.clone(), ());
        if let Some(tid) = parse_field(&key.desc).and_then(|t| self.ptype(&t)) {
            self.flow(Node::U(fi), Node::F(fi), tid);
            if self.hw_written_names.contains(&key.name) {
                self.add_to(Node::U(fi), &TypeSet::open(tid));
            }
            for p in self.hw_read_names.get(&key.name).cloned().unwrap_or_default() {
                self.flow(Node::F(fi), p, tid);
            }
            for (p, t) in self.hw_copy_names.get(&key.name).cloned().unwrap_or_default() {
                self.flow(Node::F(fi), p, t);
            }
            if self.ctx.fopen_names.borrow().contains(&key.name) {
                self.open_static(&key);
            }
        }
        fi
    }

    /// 抽象对象 o 的字段节点。已逃逸的对象才与未知接收者视图相连（收 `U`、汇入 `F`）：
    /// 未逃逸的对象只经字节码可见的引用被访问，open / 非抽象接收者不可能指向它
    pub(super) fn obj_field(&mut self, o: u32, fi: usize, tid: u32) -> Node {
        let n = Node::O(o, fi);
        let fs = self.obj_fields.entry(o).or_default();
        if !fs.iter().any(|&(f, _)| f == fi) {
            fs.push((fi, tid));
            if self.escaped.contains(&o) {
                self.flow(Node::U(fi), n, tid);
                self.flow(n, Node::F(fi), tid);
            }
        }
        n
    }

    /// 值集新到达逃逸汇点：抽象对象接上未知接收者视图；数组分配点的元素随之逃逸（非建模代码可读出）
    pub(super) fn escape(&mut self, delta: &IdSet) {
        let obj = self.id(OBJECT);
        for x in delta.iter() {
            if self.objs.contains_key(&x) {
                if !self.escaped.insert(x) {
                    continue;
                }
                for (fi, tid) in self.obj_fields.get(&x).cloned().unwrap_or_default() {
                    self.flow(Node::U(fi), Node::O(x, fi), tid);
                    self.flow(Node::O(x, fi), Node::F(fi), tid);
                }
            } else if let Some(&t) = self.arrays.get(&x) {
                if !self.escaped.insert(x) {
                    continue;
                }
                // 逃逸数组：元素随之逃逸；未知数组（open）的写入只可能落在逃逸数组上
                let cid = absint::component(&self.names[t as usize].clone()).filter(|c| c.len() > 1).map(|c| self.id(&c));
                for p in PARITIES {
                    self.flow(Node::E(x, p), Node::Esc, obj);
                    if let Some(cid) = cid {
                        self.flow(Node::Array, Node::E(x, p), cid);
                    }
                }
                // open 数组值按逃逸数组分配点展开：新逃逸的数组要补给已展开过 open 的方法 / 站点与枢纽
                self.hubs_grow(x);
                self.reopen(x);
            }
        }
    }
}

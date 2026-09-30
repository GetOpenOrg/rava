//! 引擎：类型流——节点类型集、流边、实参来源、字段节点与逃逸。

use super::*;

impl<'a> Engine<'a> {
    // ── 类型流 ──────────────────────────────────────────────────────────────

    pub(super) fn set_of(&self, n: Node) -> TypeSet {
        self.sets.get(&n).cloned().unwrap_or_default()
    }

    pub(super) fn add_to(&mut self, n: Node, s: &TypeSet) {
        if s.is_empty() {
            return;
        }
        if let Node::E(x, _) = n {
            if let Some(held) = self.empty_arrays.get_mut(&x) {
                held.entry(n).or_default().add_all(s);
                return;
            }
        }
        let cur = self.sets.entry(n).or_default();
        let delta = TypeSet {
            classes: s.classes.minus(&cur.classes),
            open: s.open.minus(&cur.open),
        };
        if delta.is_empty() {
            return;
        }
        cur.add_all(&delta);
        if n == Node::Esc {
            self.escape(&delta.classes);
        }
        if self.self_fields.contains_key(&n) {
            self.self_field_objs(n, &delta);
        }
        if let Some(&(k, e)) = self.enum_recv.get(&n) {
            self.rpending.push((k, e, delta.clone()));
        }
        // 手写方法调用点的实参新增数组分配点：接上该数组的元素读写
        if let Node::A(s, i) = n {
            let ys: Vec<u32> = delta.classes.iter().copied().filter(|x| self.arrays.contains_key(x)).collect();
            if !ys.is_empty() {
                self.hw_site_arrays(s, i, &ys);
            }
            if self.hw_reads.get(&s).is_some_and(|r| r.0 == i) {
                self.memory_read(s, &delta);
            }
        }
        if let Some(ws) = self.watch.get(&n) {
            for &w in ws {
                if self.in_swork.insert(w) {
                    self.swork.push_back(w);
                }
            }
        }
        if let Some(cs) = self.call_watch.get(&n) {
            for &c in cs {
                if self.in_cwork.insert(c) {
                    self.cwork.push_back(c);
                }
            }
        }
        // 只沿流边推送新增部分（差分传播）
        self.fdelta.entry(n).or_default().add_all(&delta);
        if self.in_fwork.insert(n) {
            self.fwork.push_back(n);
        }
    }

    /// 流边 src → dst（按 filter 收窄）；立即按当前集合推一次
    pub(super) fn flow(&mut self, src: Node, dst: Node, filter: u32) {
        if !self.flow_seen.insert((src, dst, filter)) {
            return;
        }
        self.flows.entry(src).or_default().push((dst, filter));
        let Some(s) = self.sets.remove(&src) else { return };
        let out = self.filter(&s, filter);
        self.sets.insert(src, s);
        self.add_to(dst, &out);
    }

    pub(super) fn drain_flows(&mut self) {
        while let Some(n) = self.fwork.pop_front() {
            self.in_fwork.remove(&n);
            let Some(s) = self.fdelta.remove(&n) else { continue };
            // 边表借出（推送中新接的边已由 `flow` 按当前集合推过，归还时并在后面）；
            // 同一过滤类型只收窄一次，Object 过滤直接推增量本身
            let edges = self.flows.get_mut(&n).map(std::mem::take).unwrap_or_default();
            let mut narrowed: Vec<(u32, TypeSet)> = Vec::new();
            for &(dst, f) in &edges {
                if self.names[f as usize].as_ref() == OBJECT {
                    self.add_to(dst, &s);
                    continue;
                }
                let i = match narrowed.iter().position(|x| x.0 == f) {
                    Some(i) => i,
                    None => {
                        let out = self.filter(&s, f);
                        narrowed.push((f, out));
                        narrowed.len() - 1
                    }
                };
                let out = std::mem::take(&mut narrowed[i].1);
                self.add_to(dst, &out);
                narrowed[i].1 = out;
            }
            if !edges.is_empty() {
                let slot = self.flows.entry(n).or_default();
                let added = std::mem::replace(slot, edges);
                slot.extend(added);
            }
            if let Some(ds) = self.mflows.get(&n).cloned() {
                let k = self.mirror_set(&s);
                for d in ds {
                    self.add_to(d, &k);
                }
            }
        }
    }

    /// 镜像流边 src → dst；立即按当前集合推一次
    pub(super) fn mflow(&mut self, src: Node, dst: Node) {
        if !self.mflow_seen.insert((src, dst)) {
            return;
        }
        self.mflows.entry(src).or_default().push(dst);
        let s = self.set_of(src);
        let k = self.mirror_set(&s);
        self.add_to(dst, &k);
    }

    /// 方法 m 内抽象值 v 的类型来源；未知值按声明类型 open
    pub(super) fn feeds(&mut self, m: usize, v: &V, decl: u32) -> Vec<Feed> {
        match v {
            V::Str(_) => vec![Feed::S(TypeSet::exact(self.id(STRING)))],
            V::Class(c, _) => {
                let k = self.mirror(c);
                vec![Feed::S(TypeSet::exact(k))]
            }
            V::Top => vec![Feed::S(TypeSet::open(decl))],
            V::Ref { src, .. } => src
                .iter()
                .map(|s| match *s {
                    Src::Param(i) => Feed::N(Node::P(m, i)),
                    Src::Site(o) => Feed::N(Node::S(m, o)),
                    Src::Catch(o) => Feed::N(Node::S(m, CATCH | o)),
                    Src::Str => Feed::S(TypeSet::exact(self.id(STRING))),
                })
                .collect(),
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
                    if let Some(s) = self.sets.get(n) {
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
        for &x in delta.iter() {
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

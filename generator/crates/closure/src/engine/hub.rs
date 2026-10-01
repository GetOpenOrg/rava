//! 引擎：派发枢纽——精确接收者多时经集合枢纽派发。

use super::pstrs::PSlot;
use super::*;

impl<'a> Engine<'a> {
    // ── 派发枢纽 ────────────────────────────────────────────────────────────

    /// 取（或建立）枢纽。精确集合枢纽的父枢纽：调用点原枢纽，其集合须是本集合的子集
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hub(&mut self, mref: &MemberRef, iface: bool, owner: u32, set: HubSet, parent: Option<u32>, site: &resolve::MethodSite, md: &classfile::descriptor::MethodDesc, via: Via) -> u32 {
        let key = (mref.clone(), iface, set);
        if let Some(&h) = self.hub_ids.get(&key) {
            return h;
        }
        let h = self.hubs.len() as u32;
        let ptypes = md.params.iter().map(|p| self.ptype(p)).collect();
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        let (open, pending, parent) = match &key.2 {
            HubSet::Open(o) | HubSet::Vm(o) => (Some(*o), Vec::new(), None),
            HubSet::Exact(rs) => {
                let parent = parent.filter(|&p| {
                    let ph = &self.hubs[p as usize];
                    ph.open.is_none() && ph.recvs.iter().all(|x| rs.binary_search(x).is_ok())
                });
                let pending = match parent {
                    Some(p) => rs.iter().copied().filter(|x| !self.hubs[p as usize].recvs.contains(x)).collect(),
                    None => rs.to_vec(),
                };
                (None, pending, parent)
            }
        };
        let (lambdas, special) = parent.map(|p| (self.hubs[p as usize].lambdas.clone(), self.hubs[p as usize].special.clone())).unwrap_or_default();
        self.hubs.push(Hub {
            site: site.clone(),
            owner,
            open,
            parent,
            ptypes,
            ret,
            vals: None,
            via,
            expanded: false,
            pending,
            recvs: BTreeSet::new(),
            plain: BTreeSet::new(),
            lambdas,
            special,
            links: BTreeMap::new(),
            link_seq: 0,
            edged: HashSet::default(),
        });
        self.hub_ids.insert(key, h);
        if let Some(o) = open {
            self.hubs_by_open.entry(o).or_default().push(h);
        }
        if let Some(p) = parent {
            // 父枢纽承接集合的旧部分：实参下传、返回值上汇
            let (ptypes, ret) = (self.hubs[h as usize].ptypes.clone(), self.hubs[h as usize].ret);
            for j in 0..ptypes.len() {
                self.pstr_edge(PSlot::H(h, j), PSlot::H(p, j));
            }
            for (j, pt) in ptypes.iter().enumerate() {
                if let Some(pt) = pt {
                    self.flow(Node::HP(h, j as u16), Node::HP(p, j as u16), *pt);
                }
            }
            if let Some(rt) = ret {
                self.flow(Node::HR(p), Node::HR(h), rt);
            }
            let rs = self.hubs[p as usize].recvs.clone();
            self.hubs[h as usize].recvs.extend(rs);
        }
        h
    }

    /// 调用点接入枢纽：实参汇入 `HP`，`HR` 流向结果；逐调用点派发的接收者对本调用点派发
    pub(super) fn link_hub(&mut self, h: u32, m: usize, off: u32, a: &Args, res: Option<Node>) {
        // 同一分析结果下重跑调用点：实参来源与常量不变，已接入即完成（与 `dispatched` 同口径，分析重算时清空）。
        // 字节码调用点上的 lambda 调用由自身读者单元增量驱动；非字节码调用方按当前值重新接边
        if !self.hub_linked.entry(m).or_default().insert((off, h)) {
            if self.methods[m].kind != Kind::Bytecode {
                let hub = &self.hubs[h as usize];
                let (site, lambdas, ret) = (hub.site.clone(), hub.lambdas.clone(), hub.ret);
                for r in lambdas {
                    self.dispatch_one(m, off, r, &site, a, ret, res, NOCTX);
                }
            }
            return;
        }
        self.hub_sites.entry((m, off)).or_default().insert(h);
        let hub = &self.hubs[h as usize];
        let (ptypes, ret) = (hub.ptypes.clone(), hub.ret);
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(j)) {
                self.feed(fs, Node::HP(h, j as u16), *pt);
            }
        }
        if let (Some(rt), Some(res)) = (ret, res) {
            self.flow(Node::HR(h), res, rt);
        }
        let cv = self.call_vals.clone();
        if let Some(vs) = &cv {
            self.pstr_site(m, vs, |j| PSlot::H(h, j));
        }
        let mine: Vec<PV> = (0..ptypes.len()).map(|j| cv.as_ref().and_then(|vs| vs.get(j)).map_or(PV::Top, PV::of)).collect();
        self.hub_vals(h, &mine);
        let hub = &mut self.hubs[h as usize];
        let id = hub.link_seq;
        hub.link_seq += 1;
        hub.links.insert((m, off), Rc::new(Link { id, a: a.clone(), res, cv }));
        let (site, lambdas, special) = (hub.site.clone(), hub.lambdas.clone(), hub.special.clone());
        let replay = self.methods[m].kind == Kind::Bytecode;
        for r in lambdas {
            if replay && !self.hub_lsent.entry(m).or_default().insert((off, r)) {
                continue;
            }
            self.dispatch_one(m, off, r, &site, a, ret, res, NOCTX);
        }
        for (t, rs) in special {
            if replay && self.ancestor_sent(m, off, h, t, &rs) {
                continue;
            }
            let recv = TypeSet { classes: rs.into_iter().collect(), open: IdSet::default() };
            self.edge(m, off, t, Recv::Feeds(vec![Feed::S(recv)]), a, ret, res);
            self.hubs[h as usize].edged.insert((id, t));
        }
        // 首个调用点接入后展开（先并入实参常量，再按形参值分析目标）
        self.hub_expand(h);
    }

    /// 调用点 (m, off) 已接入枢纽 h 的某个祖先、且该祖先已把目标 t 的接收者 rs 全部送达：重放恒等。
    /// 精确集合枢纽在首个调用点接入时一次展开、其后接收者不再增长，子枢纽的接收者表以父枢纽的为前缀，
    /// 故已接入的祖先的接收者表即本调用点经它收到的全部接收者
    fn ancestor_sent(&self, m: usize, off: u32, h: u32, t: usize, rs: &[u32]) -> bool {
        let Some(linked) = self.hub_linked.get(&m) else { return false };
        let mut p = self.hubs[h as usize].parent;
        while let Some(a) = p {
            if linked.contains(&(off, a)) {
                let ps = self.hubs[a as usize].special.get(&t).map_or(&[][..], |v| &v[..]);
                return rs.len() <= ps.len() && (ps.starts_with(rs) || rs.iter().all(|r| ps.contains(r)));
            }
            p = self.hubs[a as usize].parent;
        }
        false
    }

    /// 实参常量并入枢纽（沿父链下传）；变化时重新并入各中转目标
    pub(super) fn hub_vals(&mut self, h: u32, mine: &[PV]) {
        let hub = &mut self.hubs[h as usize];
        let joined: Vec<PV> = match &hub.vals {
            None => mine.to_vec(),
            Some(cur) => cur.iter().zip(mine).map(|(c, v)| PV::join(Some(c), v)).collect(),
        };
        if hub.vals.as_ref() == Some(&joined) {
            return;
        }
        hub.vals = Some(joined.clone());
        let parent = hub.parent;
        for t in hub.plain.clone() {
            self.hub_bind(h, t);
        }
        if let Some(p) = parent {
            self.hub_vals(p, &joined);
        }
    }

    pub(super) fn hub_expand(&mut self, h: u32) {
        let hub = &mut self.hubs[h as usize];
        if std::mem::replace(&mut hub.expanded, true) {
            return;
        }
        let xs = match hub.open {
            Some(o) => self.g_of(o).to_vec(),
            None => std::mem::take(&mut hub.pending),
        };
        for x in xs {
            self.hub_recv(h, x);
        }
    }

    pub(super) fn hub_bind(&mut self, h: u32, t: usize) {
        let base = usize::from(!self.methods[t].is_static);
        for j in 0..self.hubs[h as usize].ptypes.len() {
            self.pstr_edge(PSlot::H(h, j), PSlot::M(t, base + j));
        }
        let Some(vals) = self.hubs[h as usize].vals.clone() else { return };
        let n = self.methods[t].ptypes.len();
        self.bind_pvs(t, base, n, Some(&vals));
    }

    /// 新成员 x 进入 G（或数组逃逸）：已展开、open 类型含 x 的枢纽展开之
    pub(super) fn hubs_grow(&mut self, x: u32) {
        let os: Vec<u32> = self.hubs_by_open.keys().copied().collect();
        for o in os {
            if !self.sub(x, o) {
                continue;
            }
            for h in self.hubs_by_open[&o].clone() {
                if self.hubs[h as usize].expanded {
                    self.hub_recv(h, x);
                }
            }
        }
    }

    /// 枢纽展开一个接收者：方法本体经枢纽中转，按调用点建模的目标逐调用点派发
    pub(super) fn hub_recv(&mut self, h: u32, r: u32) {
        let owner = self.hubs[h as usize].owner;
        if !self.sub(r, owner) {
            return;
        }
        // open 值只能是逃逸对象：数组分配点须已逃逸（逃逸时再展开）
        if self.hubs[h as usize].open.is_some() && self.arrays.contains_key(&r) && !self.escaped.contains(&r) {
            return;
        }
        if !self.hubs[h as usize].recvs.insert(r) {
            return;
        }
        let site = self.hubs[h as usize].site.clone();
        // 调用点表只在逐调用点派发时用到（经枢纽中转的目标不逐调用点接边）
        let links = |e: &Self| -> Vec<_> { e.hubs[h as usize].links.iter().map(|(k, v)| (*k, v.clone())).collect() };
        let ret = self.hubs[h as usize].ret;
        // lambda 与手写实现对象不是 Java 类：逐调用点派发（dispatch_one 按其 SAM / trait impl 选目标）
        if self.lambdas.contains_key(&r) || self.hwobjs.contains_key(&r) {
            self.hubs[h as usize].lambdas.push(r);
            let saved = self.call_vals.take();
            for ((m, off), l) in links(self) {
                if self.methods[m].kind == Kind::Bytecode && !self.hub_lsent.entry(m).or_default().insert((off, r)) {
                    continue;
                }
                self.call_vals = l.cv.clone();
                self.dispatch_one(m, off, r, &site, &l.a, ret, l.res, NOCTX);
            }
            self.call_vals = saved;
            return;
        }
        let rt = self.ty(r);
        let rname = self.names[rt as usize].to_string();
        let Some(sel) = self.h.select(&rname, &site) else {
            self.unresolved.insert(format!("select {rname} {}", site.method().name));
            return;
        };
        let (o, n, d) = sel.key();
        let via = self.hubs[h as usize].via.clone();
        let t = self.method_ctx(MemberRef { owner: o, name: n, desc: d }, self.recv_ctx(r), via);
        if self.vm_hubs.contains(&h) {
            // VM 反射虚调用：目标与 `expose` 的反射成员同口径（形参 open；返回值由反射调用点按声明类型给出）
            self.add_to(Node::P(t, 0), &TypeSet::exact(r));
            self.open_params(t);
            return;
        }
        // 先并入形参常量，再判定是否按调用点建模（透传摘要依赖分析）
        if self.methods[t].kind == Kind::Bytecode && !self.methods[t].is_static {
            self.hub_bind(h, t);
        }
        if !self.hub_plain(t) {
            self.hubs[h as usize].special.entry(t).or_default().push(r);
            let saved = self.call_vals.take();
            for ((m, off), l) in links(self) {
                // 同一接入记录已对 t 完整接边：实参、形参常量与调用关系不变，只补接收者相关部分
                self.call_vals = l.cv.clone();
                if self.hubs[h as usize].edged.contains(&(l.id, t)) {
                    self.edge_more(m, off, t, r, &l.a, ret, l.res);
                    continue;
                }
                self.edge(m, off, t, Recv::Exact(r), &l.a, ret, l.res);
                self.hubs[h as usize].edged.insert((l.id, t));
            }
            self.call_vals = saved;
            return;
        }
        self.add_to(Node::P(t, 0), &TypeSet::exact(r));
        if !self.hubs[h as usize].plain.insert(t) {
            return;
        }
        let ptypes = self.methods[t].ptypes.clone();
        for j in 0..self.hubs[h as usize].ptypes.len() {
            if let Some(Some(pt)) = ptypes.get(1 + j) {
                self.flow(Node::HP(h, j as u16), Node::P(t, (1 + j) as u16), *pt);
            }
        }
        if let Some(rt) = self.hubs[h as usize].ret {
            self.flow(Node::R(t), Node::HR(h), rt);
        }
    }

    /// 目标经枢纽中转：字节码方法本体、结果取其返回值节点（非按调用点建模）
    pub(super) fn hub_plain(&mut self, t: usize) -> bool {
        if self.methods[t].kind != Kind::Bytecode || self.methods[t].is_static {
            return false;
        }
        self.methods[t].ret_model == RetModel::Plain && self.passthrough(t).is_none()
    }

    /// 字节码方法的返回值只来自形参时，返回这些形参序号
    pub(super) fn passthrough(&mut self, t: usize) -> Option<Vec<u16>> {
        if self.methods[t].kind != Kind::Bytecode {
            return None;
        }
        self.analysis(t)?.returned_params()
    }

    /// VM 按反射对象虚调用 key（声明类 cls 上的实例方法）：经 open(cls) 的 VM 枢纽派发到各接收者的选中实现，
    /// G 增长时增量展开（反射取到基类方法、以子类实例调用时执行的是子类覆写）
    pub(super) fn vm_dispatch(&mut self, key: &MemberRef, iface: bool, via: Via) {
        let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else {
            self.unresolved.insert(key.to_string());
            return;
        };
        let Some(md) = parse_method(&key.desc) else { return };
        let owner = self.id(&key.owner);
        let before = self.hubs.len();
        let h = self.hub(key, iface, owner, HubSet::Vm(owner), None, &site, &md, via);
        if (h as usize) < before {
            return;
        }
        self.vm_hubs.insert(h);
        self.hubs[h as usize].vals = Some(vec![PV::Top; md.params.len()]);
        self.hub_expand(h);
    }
}

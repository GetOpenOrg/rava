//! 引擎：派发枢纽——精确接收者多时经集合枢纽派发。

use super::pstrs::PSlot;
use super::reflect_call::VmBind;
use super::*;

impl<'a> Engine<'a> {
    // ── 派发枢纽 ────────────────────────────────────────────────────────────

    /// 取（或建立）枢纽。精确集合枢纽的父枢纽：调用点原枢纽，其集合须是本集合的子集
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hub(&mut self, mref: &MemberRef, iface: bool, owner: u32, set: HubSet, parent: Option<u32>, site: &resolve::MethodSite, md: &classfile::descriptor::MethodDesc, via: Via) -> u32 {
        // 档位上下文的调用点各用一族枢纽（目标在档位上下文中克隆，见 `levels_boot.rs`）
        let lc = self.via_level(&via);
        let key = (mref.clone(), iface, set, lc);
        if let Some(&h) = self.hub_ids.get(&key) {
            return h;
        }
        let h = self.hubs.len() as u32;
        let ptypes = md.params.iter().map(|p| self.ptype(p)).collect();
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        let (open, pending, parent, set) = match &key.2 {
            HubSet::Open(o) | HubSet::Vm(o) => (Some(*o), Vec::new(), None, None),
            HubSet::Exact(rs) => {
                let parent = self.hub_parent(&key.0, iface, lc, rs, parent);
                let pending = match parent.and_then(|p| self.hubs[p as usize].set.clone()) {
                    Some(ps) => sorted_minus(rs, &ps),
                    None => rs.to_vec(),
                };
                (None, pending, parent, Some(rs.clone()))
            }
        };
        let (lambdas, special) = parent.map(|p| (self.hubs[p as usize].lambdas.clone(), self.hubs[p as usize].special.clone())).unwrap_or_default();
        self.hubs.push(Hub {
            site: site.clone(),
            owner,
            open,
            parent,
            set,
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
        if let HubSet::Exact(rs) = &key.2 {
            let fam = self.hub_family.entry((key.0.clone(), iface, lc)).or_default();
            let at = fam.partition_point(|x| x.1.len() <= rs.len());
            fam.insert(at, (h, rs.clone()));
        }
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
            cut::edge_plain(&format!("H:{h}"), &format!("H:{p}"));
        }
        h
    }

    /// 精确集合枢纽 rs 的父枢纽：同族（调用成员与接口标志相同）已展开的精确集合枢纽中、集合为 rs 子集的最大者
    /// （调用点原枢纽 last 也是候选）。父枢纽承接其集合部分，新枢纽只展开差集：同一成员在大体相同的接收者集合上
    /// 派发的各调用点（不同上下文的同一调用点、汇自同一值池的各调用点）共用已展开的部分，不各自展开全集。
    /// 目标、实参与返回值的汇合与无父时相同（见 [`Hub`]）。族内按大小降序试探，未命中的试探次数有上限
    fn hub_parent(&self, mref: &MemberRef, iface: bool, lc: u32, rs: &[u32], last: Option<u32>) -> Option<u32> {
        const TRIES: usize = 16;
        let usable = |p: u32| {
            let ph = &self.hubs[p as usize];
            ph.open.is_none() && ph.expanded && ph.pending.is_empty()
        };
        let set = |p: u32| self.hubs[p as usize].set.as_deref();
        let best = last.filter(|&p| set(p).is_some_and(|ps| sorted_subset(ps.iter().copied(), rs)));
        let best_len = best.and_then(set).map_or(0, <[u32]>::len);
        let Some(fam) = self.hub_family.get(&(mref.clone(), iface, lc)) else { return best };
        let end = fam.partition_point(|x| x.1.len() <= rs.len());
        let mut tries = 0;
        for (p, ps) in fam[..end].iter().rev() {
            if ps.len() <= best_len || tries >= TRIES {
                break;
            }
            if !usable(*p) {
                continue;
            }
            tries += 1;
            if sorted_subset(ps.iter().copied(), rs) {
                return Some(*p);
            }
        }
        best
    }

    /// 调用点接入枢纽：实参汇入 `HP`，`HR` 流向结果；逐调用点派发的接收者对本调用点派发
    pub(super) fn link_hub(&mut self, h: u32, m: usize, off: u32, a: &Args, res: Option<Node>) {
        // 同一分析结果下重跑调用点：实参来源与常量不变，已接入即完成（与 `dispatched` 同口径，分析重算时清空）。
        // 字节码调用点上的 lambda 调用由自身读者单元增量驱动；非字节码调用方按当前值重新接边
        if !self.hub_linked.entry(m).or_default().insert((off, h)) {
            if self.methods[m].kind != Kind::Bytecode {
                let hub = &self.hubs[h as usize];
                let (site, lambdas, ret) = (hub.site.clone(), hub.lambdas.clone(), hub.ret);
                for &r in lambdas.iter() {
                    self.dispatch_one(m, off, r, &site, a, ret, res, NOCTX);
                }
            }
            return;
        }
        self.hub_sites.entry((m, off)).or_default().insert(h);
        if cut::edges_on() {
            cut::edge_plain(&format!("M:{}", self.methods[m].key), &format!("H:{h}"));
        }
        self.prof_seg(site_prof::SEG_LINK_FEED);
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
        self.prof_seg(site_prof::SEG_LINK_VALS);
        let cv = self.call_vals.clone();
        if let Some(vs) = &cv {
            let string = self.id(STRING);
            self.pstr_site(m, off, vs, |j| Some(PSlot::H(h, j)), |j| ptypes.get(j).copied().flatten() == Some(string));
        } else {
            self.pstr_top_h(h);
        }
        let mine: Vec<PV> = (0..ptypes.len()).map(|j| cv.as_ref().and_then(|vs| vs.get(j)).map_or(PV::Top, PV::of)).collect();
        self.hub_vals(h, &mine);
        self.prof_seg(site_prof::SEG_LINK_REPLAY);
        let hub = &mut self.hubs[h as usize];
        let id = hub.link_seq;
        hub.link_seq += 1;
        hub.links.insert((m, off), Rc::new(Link { id, a: a.clone(), res, cv }));
        let (site, lambdas, special) = (hub.site.clone(), hub.lambdas.clone(), hub.special.clone());
        let replay = self.methods[m].kind == Kind::Bytecode;
        // 本调用点已接入的最近祖先：它的 lambda 接收者都已送达本调用点（`hub_lsent` 已登记，重放即跳过），
        // 故与它的 lambda 表相同的前缀直接跳过，结果与逐个查登记相同
        let anc = if replay { self.linked_ancestor(m, off, h) } else { None };
        let skip = anc.map_or(0, |p| common_prefix(&lambdas, &self.hubs[p as usize].lambdas));
        for &r in &lambdas[skip..] {
            if replay && !self.hub_lsent.entry(m).or_default().insert((off, r)) {
                continue;
            }
            self.dispatch_one(m, off, r, &site, a, ret, res, NOCTX);
        }
        for (t, rs) in special {
            if anc.is_some_and(|p| self.ancestor_sent(p, t, &rs)) {
                continue;
            }
            let recv = TypeSet { classes: rs.iter().copied().collect(), open: IdSet::default() };
            self.edge(m, off, t, Recv::Feeds(vec![Feed::S(recv)]), a, ret, res);
            self.hubs[h as usize].edged.insert((id, t));
        }
        // 首个调用点接入后展开（先并入实参常量，再按形参值分析目标）
        self.prof_seg(site_prof::SEG_LINK_EXPAND);
        self.hub_expand(h);
    }

    /// 枢纽 h 的祖先中调用点 (m, off) 已接入的最近者
    fn linked_ancestor(&self, m: usize, off: u32, h: u32) -> Option<u32> {
        let linked = self.hub_linked.get(&m)?;
        let mut p = self.hubs[h as usize].parent;
        while let Some(a) = p {
            if linked.contains(&(off, a)) {
                return Some(a);
            }
            p = self.hubs[a as usize].parent;
        }
        None
    }

    /// 调用点已接入的祖先 a 已把目标 t 的接收者 rs 全部送达：重放恒等。
    /// 精确集合枢纽在首个调用点接入时一次展开、其后接收者不再增长，子枢纽的接收者表以父枢纽的为前缀，
    /// 故已接入的祖先的接收者表即本调用点经它收到的全部接收者。与祖先共享同一张表（未追加过）时直接成立
    fn ancestor_sent(&self, a: u32, t: usize, rs: &Rc<Vec<u32>>) -> bool {
        let Some(ps) = self.hubs[a as usize].special.get(&t) else { return rs.is_empty() };
        Rc::ptr_eq(ps, rs) || rs.len() <= ps.len() && (ps.starts_with(rs) || rs.iter().all(|r| ps.contains(r)))
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
        if self.cuts.no_open_hub && self.hubs[h as usize].open.is_some() {
            return;
        }
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
            Rc::make_mut(&mut self.hubs[h as usize].lambdas).push(r);
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
        let cx = self.recv_ctx(r);
        let t = cut::with_ctx(Some(format!("H:{h}")), Some(format!("A:{rname}")), || self.method_ctx(MemberRef { owner: o, name: n, desc: d }, cx, via));
        if self.vm_hubs.contains(&h) {
            // VM 反射虚调用：目标与 `expose` 的反射成员同口径（形参按枢纽接法：实参池 / open；返回值由反射调用点给出）
            self.vm_targets.insert(t);
            self.add_to(Node::P(t, 0), &TypeSet::exact(r));
            self.vm_hub_target(h, t);
            return;
        }
        // 先并入形参常量，再判定是否按调用点建模（透传摘要依赖分析）
        if self.methods[t].kind == Kind::Bytecode && !self.methods[t].is_static {
            self.hub_bind(h, t);
        }
        if !self.hub_plain(t) {
            Rc::make_mut(self.hubs[h as usize].special.entry(t).or_default()).push(r);
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
    /// 目标形参按 bind 接（同一成员的枢纽共用，接法逐次并入，已选中的目标补接）
    pub(super) fn vm_dispatch(&mut self, key: &MemberRef, iface: bool, via: Via, bind: VmBind) {
        let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else {
            self.unresolved.insert(key.to_string());
            return;
        };
        let Some(md) = parse_method(&key.desc) else { return };
        let owner = self.id(&key.owner);
        let before = self.hubs.len();
        let h = self.hub(key, iface, owner, HubSet::Vm(owner), None, &site, &md, via);
        self.vm_hub_bind(h, bind);
        if (h as usize) < before {
            return;
        }
        self.vm_hubs.insert(h);
        self.hubs[h as usize].vals = Some(vec![PV::Top; md.params.len()]);
        self.hub_expand(h);
    }
}

/// a 与 b 的公共前缀长度（同一张表时即全长）
fn common_prefix(a: &Rc<Vec<u32>>, b: &Rc<Vec<u32>>) -> usize {
    if Rc::ptr_eq(a, b) {
        return a.len();
    }
    a.iter().zip(b.iter()).take_while(|(x, y)| x == y).count()
}

/// 升序序列 a 是否为升序切片 b 的子集（逐个前移，首个缺失即否）
fn sorted_subset(a: impl Iterator<Item = u32>, b: &[u32]) -> bool {
    let mut j = 0;
    for x in a {
        match b[j..].binary_search(&x) {
            Ok(k) => j += k + 1,
            Err(_) => return false,
        }
    }
    true
}

/// 升序切片之差 a \ b
fn sorted_minus(a: &[u32], b: &[u32]) -> Vec<u32> {
    let mut j = 0;
    let mut out = Vec::with_capacity(a.len().saturating_sub(b.len()));
    for &x in a {
        while j < b.len() && b[j] < x {
            j += 1;
        }
        if j >= b.len() || b[j] != x {
            out.push(x);
        }
    }
    out
}

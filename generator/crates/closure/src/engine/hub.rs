//! 引擎：派发枢纽——精确接收者多时经集合枢纽派发。

use super::pstrs::PSlot;
use super::reflect_call::VmBind;
use super::*;

impl<'a> Engine<'a> {
    // ── 派发枢纽 ────────────────────────────────────────────────────────────

    /// 取（或建立）枢纽
    #[allow(clippy::too_many_arguments)]
    pub(super) fn hub(&mut self, mref: &MemberRef, iface: bool, owner: u32, set: HubSet, site: &resolve::MethodSite, md: &classfile::descriptor::MethodDesc, via: Via) -> u32 {
        // 档位上下文的调用点各用一族枢纽（目标在档位上下文中克隆，见 `levels_boot.rs`）
        let lc = self.via_level(&via);
        let key = (mref.clone(), iface, set, lc);
        if let Some(&h) = self.hub_ids.get(&key) {
            return h;
        }
        let ptypes = md.params.iter().map(|p| self.ptype(p)).collect();
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        self.hub_new(key, site, owner, ptypes, ret, via)
    }

    /// 建立键为 key 的枢纽（调用方已确认不存在）
    pub(super) fn hub_new(&mut self, key: (MemberRef, bool, HubSet, u32), site: &resolve::MethodSite, owner: u32, ptypes: Vec<Option<u32>>, ret: Option<u32>, via: Via) -> u32 {
        let h = self.hubs.len() as u32;
        let open = match &key.2 {
            HubSet::Open(o) | HubSet::Vm(o) => Some(*o),
            HubSet::Grow(_) | HubSet::Shared(_) => None,
        };
        let share = match &key.2 {
            HubSet::Grow(src) => Some((key.0.clone(), key.1, matches!(src, GrowSrc::Nodes(_, true)))),
            _ => None,
        };
        self.hubs.push(Hub {
            site: site.clone(),
            owner,
            open,
            ptypes,
            ret,
            vals: None,
            via,
            expanded: false,
            recvs: BTreeSet::new(),
            plain: BTreeSet::new(),
            lambdas: Rc::default(),
            special: BTreeMap::new(),
            links: BTreeMap::new(),
            link_seq: 0,
            edged: HashSet::default(),
            anchor: None,
            ltargets: BTreeSet::new(),
            lhubs: BTreeSet::new(),
            lbind: BTreeSet::new(),
            share,
            fwd: None,
            skeys: HashSet::default(),
        });
        self.hub_ids.insert(key, h);
        if let Some(o) = open {
            self.hubs_by_open.entry(o).or_default().push(h);
        }
        h
    }

    /// 精确接收者 recv（升序）达 `HUB_MIN` 的调用点经增长枢纽派发：按接收值来源 fs 取枢纽（来源全为图节点时按来源共用，
    /// 否则按调用点；objs = 只收抽象对象的非虚调用点），接入并并入接收者（已并入的跳过）。
    /// 枢纽只随来源建立、不随接收者集合的增长新建：同一来源的值集在各调用点恒同，枢纽数 = 来源数
    #[allow(clippy::too_many_arguments)]
    pub(super) fn grow_hub(&mut self, m: usize, off: u32, mref: &MemberRef, iface: bool, owner: u32, fs: &[Feed], objs: bool, recv: &[u32], site: &resolve::MethodSite, md: &classfile::descriptor::MethodDesc, via: Via, a: &Args, res: Option<Node>) -> u32 {
        let src = match fs.iter().map(|f| if let Feed::N(n) = f { Some(*n) } else { None }).collect::<Option<Vec<Node>>>() {
            Some(mut ns) => {
                ns.sort_unstable();
                ns.dedup();
                GrowSrc::Nodes(ns.into(), objs)
            }
            None => GrowSrc::Site(m, off),
        };
        let h = self.hub(mref, iface, owner, HubSet::Grow(src), site, md, via);
        self.link_hub(h, m, off, a, res);
        for &r in recv {
            self.hub_recv(h, r);
        }
        h
    }

    /// 调用点接入枢纽：实参汇入 `HP`，`HR` 流向结果；逐调用点派发的接收者对本调用点派发
    pub(super) fn link_hub(&mut self, h: u32, m: usize, off: u32, a: &Args, res: Option<Node>) {
        // 已升级的增长枢纽：接入转发到共享枢纽
        let h = self.hub_fwd(h);
        // 枢纽上的接边属调用点（重接随调用方整方法重接），不归当前 lambda 调用读者
        // 枢纽 lambda 读者代接时，接入记录与其接边归该读者的枢纽（`hub_reader.rs`）
        let host = self.reader_hub();
        let outer = self.cur_lcall.take();
        self.with_reader(host, |e| e.link_hub_in(h, m, off, a, res));
        self.cur_lcall = outer;
    }

    pub(super) fn link_hub_in(&mut self, h: u32, m: usize, off: u32, a: &Args, res: Option<Node>) {
        // 同一分析结果下重跑调用点：实参来源与常量不变，已接入即完成（与 `dispatched` 同口径，分析重算时清空）。
        // 字节码调用点上的 lambda 调用由自身读者单元增量驱动；非字节码调用方按当前值重新接边
        if !self.hub_linked.entry(m).or_default().insert((off, h)) {
            // 锚定在同一调用点的另一枢纽读者再接入：接入关系同样记到该读者的枢纽
            if let Some(g) = self.reader_hub() {
                self.hubs[g as usize].lhubs.insert(h);
            }
            if self.methods[m].kind != Kind::Bytecode {
                let hub = &self.hubs[h as usize];
                let (site, lambdas, ret) = (hub.site.clone(), hub.lambdas.clone(), hub.ret);
                for &r in lambdas.iter() {
                    self.lprof_replay(h, false);
                    self.dispatch_one(m, off, r, &site, a, ret, res, NOCTX);
                }
                // lambda 上非 SAM 方法的目标（`hub_recv` 登记在逐调用点目标表）同样按当前值重新接边
                let special = self.hubs[h as usize].special.clone();
                for (t, rs) in special {
                    let ls: Vec<u32> = rs.iter().copied().filter(|r| self.lambdas.contains_key(r)).collect();
                    for r in ls {
                        self.edge(m, off, t, Recv::Exact(r), a, ret, res);
                    }
                }
            } else {
                self.link_more(h, m, off, a);
            }
            return;
        }
        let host = self.reader_hub();
        self.note_hub_site(host, h, m, off);
        if cut::edges_on() {
            cut::edge_plain(&self.site_node(m, off), &format!("H:{h}"));
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
        self.taint_hub_site(m, h, ptypes.len(), cv.as_deref());
        // 形参常量格与逐个接边同口径（`bind_params` 取 `PV::of_ret`：确定非空的无标签引用记为「非空引用」）
        let mine: Vec<PV> = (0..ptypes.len()).map(|j| cv.as_ref().and_then(|vs| vs.get(j)).map_or(PV::Top, PV::of_ret)).collect();
        self.hub_vals(h, &mine);
        self.prof_seg(site_prof::SEG_LINK_REPLAY);
        let replay = self.methods[m].kind == Kind::Bytecode;
        let hub = &mut self.hubs[h as usize];
        // 调用方重分析后以同一接入（实参来源、结果节点、实参常量）重接：沿用原接入记录；
        // 送达本调用点的 lambda 读者若在本次重分析中挂起，按同一调用直接复活，不再逐个派发
        let same = replay && hub.links.get(&(m, off)).is_some_and(|l| l.a == *a && l.res == res && l.cv == cv);
        let id = if same {
            hub.links[&(m, off)].id
        } else {
            let id = hub.link_seq;
            hub.link_seq += 1;
            hub.links.insert((m, off), Rc::new(Link { id, a: a.clone(), res, cv, host }));
            id
        };
        let (site, lambdas, special) = (hub.site.clone(), hub.lambdas.clone(), hub.special.clone());
        // 字节码接入点：lambda 读者只建在枢纽锚点，实参 / 结果取枢纽节点（见 `Hub::anchor`）
        let at_anchor = replay && *self.hubs[h as usize].anchor.get_or_insert((m, off)) == (m, off);
        let lambdas: &[u32] = if replay && !at_anchor { &[] } else { &lambdas[..] };
        let (la, lres) = if replay { self.hub_lambda_call(h) } else { (a.clone(), res) };
        let saved = if replay { self.call_vals.take() } else { None };
        // 锚点上的读者归本枢纽；非字节码接入点逐调用点派发，归属同接入记录
        let owner = if replay { Some(h) } else { host };
        let lambdas = lambdas.to_vec();
        self.with_reader(owner, |e| {
            for r in lambdas {
                if replay && !e.hub_lsent.entry(m).or_default().insert((off, r, h)) {
                    continue;
                }
                if replay {
                    let id = e.lambda_done.get(&m).and_then(|d| d.get(&off)).and_then(|at| at.get(&(r, ret, lres, Some(h)))).copied();
                    if let Some(id) = id.filter(|&id| e.lcalls[id as usize].suspended) {
                        e.lcall_revive(id);
                        continue;
                    }
                }
                e.lprof_replay(h, true);
                e.dispatch_one(m, off, r, &site, &la, ret, lres, NOCTX);
            }
        });
        if replay {
            self.call_vals = saved;
        }
        for (t, rs) in special {
            if matches!(self.methods[t].ret_model, RetModel::HandleAccess(_)) {
                // 句柄存取入口按接收者逐个接边（与 `hub_recv` 同口径）：合成值集会把来源标记一并归为
                // 「由标记给出身份」，本调用点不登记所指字段，结果随调用点经父枢纽还是直接收到接收者而变
                for &r in rs.iter() {
                    self.edge(m, off, t, Recv::Exact(r), a, ret, res);
                }
            } else {
                let recv = TypeSet { classes: rs.iter().copied().collect(), open: IdSet::default() };
                self.edge(m, off, t, Recv::Feeds(vec![Feed::S(recv)]), a, ret, res);
            }
            self.hubs[h as usize].edged.insert((id, t));
        }
        // 首个调用点接入后展开（先并入实参常量，再按形参值分析目标）
        self.prof_seg(site_prof::SEG_LINK_EXPAND);
        self.hub_expand(h);
    }

    /// 枢纽上 lambda 读者的实参与结果节点：枢纽形参节点 `HP`、返回节点 `HR`
    fn hub_lambda_call(&self, h: u32) -> (Args, Option<Node>) {
        let hub = &self.hubs[h as usize];
        let a = hub.ptypes.iter().enumerate().map(|(j, pt)| pt.map(|_| vec![Feed::N(Node::HP(h, j as u16))])).collect();
        (a, hub.ret.map(|_| Node::HR(h)))
    }

    /// 同一调用点以另一组实参再接入（同一调用点上不同方法引用 lambda 的转发调用，捕获拼接后实参不同）：
    /// 接入记录的实参并为两组之并，新增部分汇入 `HP`；逐调用点派发的接收者与按调用点建模的目标以并后的实参重接
    fn link_more(&mut self, h: u32, m: usize, off: u32, a: &Args) {
        let Some(l) = self.hubs[h as usize].links.get(&(m, off)).cloned() else { return };
        let merged: Args = l
            .a
            .iter()
            .zip(a.iter())
            .map(|(x, y)| match (x, y) {
                (Some(x), Some(y)) => {
                    let mut v = x.clone();
                    v.extend(y.iter().filter(|f| !x.contains(f)).cloned());
                    Some(v)
                }
                (x, y) => x.clone().or_else(|| y.clone()),
            })
            .collect();
        if merged == l.a {
            return;
        }
        let ptypes = self.hubs[h as usize].ptypes.clone();
        for (j, f) in a.iter().enumerate() {
            if let (Some(fs), Some(Some(pt))) = (f, ptypes.get(j)) {
                self.feed(fs, Node::HP(h, j as u16), *pt);
            }
        }
        let hub = &mut self.hubs[h as usize];
        hub.links.insert((m, off), Rc::new(Link { id: l.id, a: merged.clone(), res: l.res, cv: l.cv.clone(), host: l.host }));
        let (site, lambdas, special, ret) = (hub.site.clone(), hub.lambdas.clone(), hub.special.clone(), hub.ret);
        let saved = std::mem::replace(&mut self.call_vals, l.cv.clone());
        // lambda 接收者的读者在枢纽锚点、以枢纽节点为实参，新增实参已汇入 `HP`
        let _ = (lambdas, site);
        self.with_reader(l.host, |e| {
            for (t, rs) in special {
                for &r in rs.iter() {
                    e.edge(m, off, t, Recv::Exact(r), &merged, ret, l.res);
                }
            }
        });
        self.call_vals = saved;
    }

    /// 实参常量并入枢纽；变化时重新并入各中转目标
    pub(super) fn hub_vals(&mut self, h: u32, mine: &[PV]) {
        let hub = &mut self.hubs[h as usize];
        let joined: Vec<PV> = match &hub.vals {
            None => mine.to_vec(),
            Some(cur) => cur.iter().zip(mine).map(|(c, v)| PV::join(Some(c), v)).collect(),
        };
        if hub.vals.as_ref() == Some(&joined) {
            return;
        }
        hub.vals = Some(joined);
        for t in hub.plain.clone() {
            self.hub_bind(h, t);
        }
        self.hub_lrebind(h);
    }

    pub(super) fn hub_expand(&mut self, h: u32) {
        if self.cuts.no_open_hub && self.hubs[h as usize].open.is_some() {
            return;
        }
        let hub = &mut self.hubs[h as usize];
        if std::mem::replace(&mut hub.expanded, true) {
            return;
        }
        let Some(o) = hub.open else { return };
        for x in self.g_of(o).to_vec() {
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
        // 污染不看合流后的常量格：各接入点的污染已记在枢纽形参槽上，沿上面的子集边到达目标形参槽
        self.pstr_offsite(t);
        self.join_pvs(t, base, n, Some(&vals));
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
        // 枢纽上的接边属各接入调用点，不归当前 lambda 调用读者
        let outer = self.cur_lcall.take();
        self.hub_recv_in(h, r);
        self.cur_lcall = outer;
    }

    fn hub_recv_in(&mut self, h: u32, r: u32) {
        let h = self.hub_fwd(h);
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
        // 增长枢纽接收者超过阈值：永久升级为共享枢纽（已并入的接收者连同 r 一并转发）
        if self.hubs[h as usize].share.is_some() && self.hub_grown(h) {
            return;
        }
        let site = self.hubs[h as usize].site.clone();
        // 调用点表只在逐调用点派发时用到（经枢纽中转的目标不逐调用点接边）
        let links = |e: &Self| -> Vec<_> { e.hubs[h as usize].links.iter().map(|(k, v)| (*k, v.clone())).collect() };
        let ret = self.hubs[h as usize].ret;
        // lambda 上的非 SAM 方法（Object 方法 / 接口 default 方法）：目标与接收者无关地由所实现的接口选出，
        // 与 Java 类接收者同样按目标逐调用点接边并登记为按调用点建模的目标——调用点重接时每个目标一条边，
        // 不逐 lambda 重放（open 枢纽上 lambda 数以千计）
        let non_sam = self.lambdas.get(&r).is_some_and(|l| site.method().name != l.sam);
        if non_sam && !self.vm_hubs.contains(&h) {
            let via = self.hubs[h as usize].via.clone();
            if let Some(t) = self.lambda_member(r, &site, via) {
                self.hub_special(h, t, r);
            }
            return;
        }
        // lambda 的 SAM 与手写实现对象不是 Java 类：逐调用点派发（dispatch_one 按其 SAM / trait impl 选目标）
        if self.lambdas.contains_key(&r) || self.hwobjs.contains_key(&r) {
            Rc::make_mut(&mut self.hubs[h as usize].lambdas).push(r);
            let saved = self.call_vals.take();
            let anchor = self.hubs[h as usize].anchor;
            for ((m, off), l) in links(self) {
                if self.methods[m].kind == Kind::Bytecode {
                    // 字节码接入点只在锚点以枢纽节点建一个读者
                    if anchor != Some((m, off)) || !self.hub_lsent.entry(m).or_default().insert((off, r, h)) {
                        continue;
                    }
                    let (la, lres) = self.hub_lambda_call(h);
                    self.lprof_replay(h, true);
                    self.call_vals = None;
                    self.with_reader(Some(h), |e| e.dispatch_one(m, off, r, &site, &la, ret, lres, NOCTX));
                    continue;
                }
                self.lprof_replay(h, true);
                self.call_vals = l.cv.clone();
                self.with_reader(l.host, |e| e.dispatch_one(m, off, r, &site, &l.a, ret, l.res, NOCTX));
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
            self.hub_special(h, t, r);
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

    /// 接收者 r 的目标 t 按调用点建模：登记进枢纽的逐调用点目标表，对已接入的各调用点接边
    fn hub_special(&mut self, h: u32, t: usize, r: u32) {
        // 同效果键的接收者已有代表接过边：逐调用点效果相同，只把 r 注入被调方 this
        if let Some(k) = self.special_key(t, r) {
            if !self.hubs[h as usize].skeys.insert((t, k)) {
                self.add_to(Node::P(t, 0), &TypeSet::exact(r));
                return;
            }
        }
        Rc::make_mut(self.hubs[h as usize].special.entry(t).or_default()).push(r);
        let ret = self.hubs[h as usize].ret;
        let links: Vec<_> = self.hubs[h as usize].links.iter().map(|(k, v)| (*k, v.clone())).collect();
        let saved = self.call_vals.take();
        for ((m, off), l) in links {
            // 同一接入记录已对 t 完整接边：实参、形参常量与调用关系不变，只补接收者相关部分
            self.call_vals = l.cv.clone();
            if self.hubs[h as usize].edged.contains(&(l.id, t)) {
                self.with_reader(l.host, |e| e.edge_more(m, off, t, r, &l.a, ret, l.res));
                continue;
            }
            self.with_reader(l.host, |e| e.edge(m, off, t, Recv::Exact(r), &l.a, ret, l.res));
            self.hubs[h as usize].edged.insert((l.id, t));
        }
        self.call_vals = saved;
    }

    /// 目标经枢纽中转：字节码方法本体、结果取其返回值节点（非按调用点建模）。
    /// @CallerSensitive 目标的调用者镜像取自各调用点所在类（`caller_edge`），经枢纽中转会丢掉调用点，
    /// 故逐调用点接边（`edge` 按接入调用点并入调用者镜像）
    pub(super) fn hub_plain(&mut self, t: usize) -> bool {
        if self.methods[t].kind != Kind::Bytecode || self.methods[t].is_static || self.is_caller_sensitive(t) {
            return false;
        }
        self.methods[t].ret_model == RetModel::Plain && self.passthrough(t).is_none()
    }

    /// 字节码方法的返回值只来自形参时，返回这些形参序号
    pub(super) fn passthrough(&mut self, t: usize) -> Option<Vec<u16>> {
        if self.methods[t].kind != Kind::Bytecode {
            return None;
        }
        let a = self.analysis(t)?;
        self.returned_of(t, &a)
    }

    /// 分析 a 的透传摘要（[`Analysis::returned_params`]）。只对引用返回的方法有意义：无引用返回值的方法
    /// 不接返回值流，恒按非透传（尚无返回路径的空集只是引用返回值的 ⊥，不必为它们逐调用点接边）
    pub(super) fn returned_of(&self, t: usize, a: &Analysis) -> Option<Vec<u16>> {
        self.methods[t].rtype?;
        a.returned_params()
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
        let h = self.hub(key, iface, owner, HubSet::Vm(owner), &site, &md, via);
        self.vm_hub_bind(h, bind);
        if (h as usize) < before {
            return;
        }
        self.vm_hubs.insert(h);
        self.hubs[h as usize].vals = Some(vec![PV::Top; md.params.len()]);
        self.hub_expand(h);
    }
}

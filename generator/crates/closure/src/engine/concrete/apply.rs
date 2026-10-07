//! 具体求值结果并入闭包：轨迹方法的依赖登记、结果对象图物化、折叠用的轨迹可达性。

use classfile::{op, Operand};

use super::snap::{MObj, MV};
use super::vm::{Lam, Put};
use super::*;

/// 物化时不可按类型代表的结果（映像中的容器形态对象：其抽象值是按分配点区分的抽象对象）
pub(in crate::engine) fn image_types(o: &Outcome) -> impl Iterator<Item = &Rc<str>> {
    let all = o.rets.iter().chain(&o.thrown).chain(o.memo.iter().map(|(_, v)| v));
    let objs = o.objs.iter().flat_map(|x| x.fields.iter().map(|(_, v)| v).chain(&x.elems));
    all.chain(objs).filter_map(|v| match v {
        MV::Image(t) => Some(t),
        _ => None,
    })
}

/// 物化 lambda 的类型名：实现方法 + 具体求值调用点 + 快照序号
fn lam_name(l: &Lam, m: usize, off: u32, i: usize) -> String {
    format!("{}$$Lambda@concrete:{m}:{off}:{i}:{}", l.imp.member.owner, l.imp.member.name)
}

/// String 字段写入的字符串值：内容可读时为字面量，否则推不出（None）；null 不计
fn put_str(p: &Put) -> Option<Option<V>> {
    match p {
        Put::Null => None,
        Put::Str(s) => Some(Some(V::Str(s.clone(), Rc::from([Src::Str(crate::absint::lit_id(s))])))),
        _ => Some(None),
    }
}

fn pv_of(v: &MV) -> PV {
    match v {
        MV::Prim(Put::Int(x)) => PV::Const(V::Int(*x)),
        MV::Prim(Put::Long(x)) => PV::Const(V::Long(*x)),
        MV::Prim(Put::Null) => PV::Const(V::Null),
        _ => PV::Top,
    }
}

impl<'a> Engine<'a> {
    /// 一组实参的结果并入调用点 m@off（入口 entry）
    pub(super) fn concrete_apply(&mut self, m: usize, off: u32, entry: &MemberRef, md: &MethodDesc, o: &Outcome) {
        let via = Via::method("concrete", m, Some(off));
        let cctx = self.concrete.ctx;
        let mut grown: Vec<MemberRef> = Vec::new();
        for (k, pcs) in &o.pcs {
            let t = self.concrete.traces.entry(k.clone()).or_default();
            let n = t.0.len();
            t.0.extend(pcs.iter().copied());
            if t.0.len() > n {
                grown.push(k.clone());
            }
        }
        for ((k, pc), ts) in &o.calls {
            let t = self.concrete.traces.entry(k.clone()).or_default();
            let e = t.1.entry(*pc).or_default();
            let n = e.len();
            e.extend(ts.iter().cloned());
            if e.len() > n && !grown.contains(k) {
                grown.push(k.clone());
            }
        }
        for k in grown {
            let t = self.method_ctx(k, cctx, via.clone());
            self.push_m(t);
        }
        for c in &o.inited {
            self.init(c, via.clone());
        }
        for (f, ps) in &o.puts {
            if self.static_final(f) {
                continue;
            }
            for p in ps {
                self.field_put(f, pv_of(&MV::Prim(p.clone())));
            }
            self.concrete_str_puts(m, f, ps);
        }
        // 结果对象图
        let obj = self.id(OBJECT);
        let mut ids: Vec<Option<TypeSet>> = vec![None; o.objs.len()];
        for i in 0..o.objs.len() {
            self.mat_obj(m, off, &o.objs, i, &mut ids, &via);
        }
        for (i, x) in o.objs.iter().enumerate() {
            let me = ids[i].clone().unwrap_or_default();
            if let Some(l) = &x.lam {
                self.mat_lambda(m, off, l, &lam_name(l, m, off, i), &x.elems, &ids, &via);
            } else if x.arr {
                let id = me.classes.iter().next().expect("数组物化为分配点");
                for (j, e) in x.elems.iter().enumerate() {
                    if let Some(f) = self.mat_val(e, &ids, &via) {
                        self.feed(&[f], Node::E(id, (j % 2) as u8), obj);
                    }
                }
            } else {
                for (f, v) in &x.fields {
                    self.mat_field(f, v, &ids, &via);
                }
            }
        }
        for (f, v) in &o.memo {
            self.mat_field(f, v, &ids, &via);
        }
        for v in &o.thrown {
            self.mat_val(v, &ids, &via);
        }
        let ret_ref = md.ret.as_ref().and_then(|r| self.ptype(r));
        let mut rv: Option<PV> = None;
        for v in &o.rets {
            rv = Some(PV::join(rv.as_ref(), &pv_of(v)));
            if let (Some(_), Some(f)) = (ret_ref, self.mat_val(v, &ids, &via)) {
                self.feed(&[f], Node::S(m, off), obj);
            }
        }
        // 抛出的组合：调用点之后不可经此组合到达，返回常量格按值未知并入（保守）
        if !o.thrown.is_empty() || o.rets.is_empty() {
            rv = Some(PV::Top);
        }
        if let Some(rv) = rv {
            self.join_rval(entry, rv);
        }
    }

    fn static_final(&self, f: &MemberRef) -> bool {
        self.h.class(&f.owner).and_then(|c| c.field(&f.name, &f.desc).map(|x| x.is_static() && x.access & acc::FINAL != 0)).unwrap_or(false)
    }

    fn join_rval(&mut self, key: &MemberRef, r: PV) {
        let cur = self.ctx.rvals.borrow().get(key).cloned();
        let new = PV::join(cur.as_ref(), &r);
        if cur.as_ref() == Some(&new) {
            return;
        }
        self.ctx.rvals.borrow_mut().insert(key.clone(), new);
        let deps = self.ctx.rdeps.borrow().get(key).cloned();
        self.invalidate_all(deps, Why::RetConst);
    }

    fn mat_obj(&mut self, m: usize, off: u32, objs: &[MObj], i: usize, ids: &mut [Option<TypeSet>], via: &Via) {
        let x = &objs[i];
        let s = if let Some(l) = &x.lam {
            TypeSet::exact(self.id(&lam_name(l, m, off, i)))
        } else if x.arr {
            TypeSet::exact(self.array_site(m, off, &x.ty, false, via.clone()))
        } else {
            self.instantiate_type(&x.ty, via.clone());
            TypeSet::exact(self.id(&x.ty))
        };
        ids[i] = Some(s);
    }

    /// 具体求值中 String 字段的写入并入字段常量集与字段字符串槽（与字节码写入同一口径，见 bytecode.rs）：
    /// 按名读取的资源 / 类 / 资源束经字段取名时，具体求值写入的名字照样计入，内容不可读的写入使槽推不出
    fn concrete_str_puts(&mut self, m: usize, f: &MemberRef, ps: &[Put]) {
        if f.desc != format!("L{STRING};") {
            return;
        }
        let fi = self.field_node(f.clone());
        for v in ps.iter().filter_map(put_str) {
            self.field_strs_put(f, v.as_ref());
            self.pstr_field_put(m, fi, v.as_ref());
        }
    }

    fn mat_val(&mut self, v: &MV, ids: &[Option<TypeSet>], via: &Via) -> Option<Feed> {
        let s = match v {
            MV::Prim(_) => None,
            MV::Str => {
                self.instantiate(STRING, via.clone());
                Some(TypeSet::exact(self.id(STRING)))
            }
            MV::Mirror(c) => {
                if c.len() > 1 {
                    self.touch(c, Level::Type, via.clone());
                }
                self.instantiate(CLASS, via.clone());
                Some(TypeSet::exact(self.mirror(c)))
            }
            MV::Obj(i) => ids[*i].clone(),
            MV::Image(t) => {
                self.instantiate(t, via.clone());
                Some(TypeSet::exact(self.id(t)))
            }
            MV::Static(f) => {
                self.init(&f.owner, via.clone());
                let fi = self.field_node(f.clone());
                return Some(Feed::N(Node::F(fi)));
            }
        };
        s.map(Feed::S)
    }

    /// lambda 对象物化为调用点 m@off 名下的抽象 lambda（捕获来源取各次物化的并）
    #[allow(clippy::too_many_arguments)]
    fn mat_lambda(&mut self, m: usize, off: u32, l: &Lam, name: &str, caps: &[MV], ids: &[Option<TypeSet>], via: &Via) {
        let Some(md) = parse_method(&l.desc) else { return };
        let mut cap: Args = Vec::with_capacity(md.params.len());
        for (p, v) in md.params.iter().zip(caps) {
            let slot = self.ptype(p).map(|_| self.mat_val(v, ids, via).into_iter().collect::<Vec<_>>());
            cap.push(slot);
        }
        let lid = self.id(name);
        if let Some(old) = self.lambdas.get(&lid) {
            for (slot, prev) in cap.iter_mut().zip(&old.cap) {
                if let (Some(fs), Some(prev)) = (slot.as_mut(), prev) {
                    fs.extend(prev.iter().cloned());
                }
            }
        }
        let ctx = self.methods[m].ctx;
        let Some(Const::MethodHandle(imh)) = l.bargs.get(1) else { return };
        self.new_lambda(m, off, name, ctx, (l.iface.clone(), &l.sam), imh, (cap, None), &l.bargs);
    }

    /// 字段写入：常量格并入值集，引用值并入未知接收者视图（物化对象按类型代表，读者经字段并集取值）
    fn mat_field(&mut self, f: &MemberRef, v: &MV, ids: &[Option<TypeSet>], via: &Via) {
        if !self.static_final(f) {
            self.field_put(f, pv_of(v));
        }
        if parse_field(&f.desc).and_then(|t| self.ptype(&t)).is_none() {
            return;
        }
        let fi = self.field_node(f.clone());
        if let Some(f) = self.mat_val(v, ids, via) {
            let obj = self.id(OBJECT);
            self.feed(&[f], Node::U(fi), obj);
        }
    }

    /// 具体上下文的方法节点：按轨迹执行过的指令登记依赖（不做抽象分析）
    pub(in crate::engine) fn process_concrete(&mut self, m: usize) {
        let key = self.methods[m].key.clone();
        let Some((pcs, calls)) = self.concrete.traces.get(&key).cloned() else { return };
        let Some(cf) = self.h.class(&key.owner) else { return };
        let Some(code) = cf.method(&key.name, &key.desc).and_then(|x| x.code.as_ref()) else { return };
        let cctx = self.concrete.ctx;
        for x in code.insns.iter().filter(|x| pcs.contains(&x.offset)) {
            let off = x.offset;
            let via = Via::method("concrete", m, Some(off));
            match (&x.operand, x.opcode) {
                (Operand::Class(c), op::NEW) => {
                    self.instantiate_type(c, via.clone());
                    self.init(c, via);
                }
                (Operand::Class(c), op::ANEWARRAY) => self.touch_desc(&interp::array_of(c), &via),
                (Operand::Class(c), _) => {
                    if c.starts_with('[') {
                        self.touch_desc(c, &via);
                    } else {
                        self.touch(c, Level::Type, via);
                    }
                }
                (Operand::MultiANewArray(c, _), _) => self.touch_desc(c, &via),
                (Operand::Ldc(c), _) => {
                    let c = c.clone();
                    self.ldc(m, off, &c);
                }
                (Operand::Field(f), opc) => {
                    let Some(site) = self.h.resolve_field(&f.owner, &f.name, &f.desc) else { continue };
                    let decl = site.class.name.clone();
                    self.touch(&f.owner, Level::Layout, via.clone());
                    self.touch_desc(&f.desc, &via);
                    self.nest_access(m, off, &decl, site.field().access & acc::PRIVATE != 0);
                    if opc == op::GETSTATIC || opc == op::PUTSTATIC {
                        self.init(&decl, via.clone());
                    }
                    self.field_handwritten(&decl, &f.name, &f.desc, &via, None);
                }
                (Operand::Method(mref, iface), opc) => {
                    let lvl = if opc == op::INVOKESTATIC || opc == op::INVOKESPECIAL { Level::Layout } else { Level::Type };
                    self.touch(&mref.owner, lvl, via.clone());
                    self.note_ref(mref);
                    let Some(site) = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface) else { continue };
                    let (o, _, _) = site.key();
                    let rm = site.method();
                    self.nest_access(m, off, &o, rm.is_private());
                    if opc == op::INVOKESTATIC {
                        self.init(&o, via.clone());
                    } else if opc != op::INVOKESPECIAL {
                        let direct = rm.is_private()
                            || rm.is_static()
                            || rm.is_final()
                            || site.class.access & acc::FINAL != 0 && !site.class.is_interface()
                            || mref.owner.starts_with('[');
                        if !direct {
                            self.recv_sites.insert((m, off));
                        } else if !rm.is_private() {
                            self.direct_virtual_sites.insert((m, off));
                        }
                    }
                    for t in calls.get(&off).into_iter().flatten() {
                        let ti = self.method_ctx(t.clone(), cctx, via.clone());
                        self.dispatch.entry((m, off)).or_default().insert(ti);
                        self.callers.entry(ti).or_default().insert(m);
                    }
                }
                (Operand::InvokeDynamic { bsm, name, desc, .. }, _) => {
                    let n = interp::nparams(desc);
                    self.indy(m, off, &cf, *bsm, name, desc, &vec![V::Top; n], false);
                }
                _ => {}
            }
        }
        for h in &code.exception_table {
            if pcs.contains(&h.handler) {
                let c = h.catch_type.clone().unwrap_or_else(|| THROWABLE.to_string());
                self.touch(&c, Level::Type, Via::method("concrete-catch", m, Some(h.handler)));
            }
        }
    }

    /// 折叠用的轨迹可达性（具体上下文的克隆没有抽象分析结果）
    pub(in crate::engine) fn concrete_analysis(&self, key: &MemberRef) -> Option<Rc<Analysis>> {
        let cf = self.h.class(&key.owner)?;
        let code = cf.method(&key.name, &key.desc)?.code.as_ref()?;
        let pcs = self.concrete_pcs(key);
        Some(Rc::new(Analysis {
            reachable: code.insns.iter().map(|x| pcs.is_some_and(|p| p.contains(&x.offset))).collect(),
            events: Vec::new(),
            pending_types: Vec::new(),
            mirror_assumed: Vec::new(),
            mirror_field_assumed: Vec::new(),
            conservative: false,
            cfg: Rc::default(),
            selector_params: 0,
        }))
    }
}

//! 引擎：lambda / MethodHandle / indy。

use super::*;

/// `ObjectMethods` 调用点对应的 Object 方法描述符：indy 描述符去掉首个（record 接收者）形参
fn object_method_desc(md: &MethodDesc) -> String {
    let params: String = md.params.iter().skip(1).map(FieldType::descriptor).collect();
    format!("({params}){}", md.ret.as_ref().map_or_else(|| "V".to_string(), FieldType::descriptor))
}

/// `altMetafactory` 静态实参的附加接口（LambdaMetafactory 协议：`[samMT, impl, instMT, flags, (n, 标记类×n)?, (n, 桥接 MT×n)?]`；
/// flags 位 1 = 可序列化、2 = 带标记接口、4 = 带桥接）。可序列化时 lambda 类实现清单的序列化标记接口。
/// `metafactory` 只有前三项，结果为空
fn alt_markers(bargs: &[Const], serializable: &[String]) -> Vec<String> {
    const FLAG_SERIALIZABLE: i32 = 1;
    const FLAG_MARKERS: i32 = 2;
    let Some(Const::Int(flags)) = bargs.get(3) else { return vec![] };
    let mut out = Vec::new();
    if flags & FLAG_SERIALIZABLE != 0 {
        out.extend(serializable.iter().cloned());
    }
    if flags & FLAG_MARKERS != 0 {
        if let Some(Const::Int(n)) = bargs.get(4) {
            let n = usize::try_from(*n).unwrap_or(0);
            out.extend(bargs.iter().skip(5).take(n).filter_map(|c| match c {
                Const::Class(c) => Some(c.clone()),
                _ => None,
            }));
        }
    }
    out
}

/// 站点键：record 引用分量值的汇合节点（`ObjectMethods` 调用点偏移 | 本位）
const COMPONENTS: u32 = 1 << 30;

impl<'a> Engine<'a> {
    /// lambda 的 SAM 调用：捕获实参 ++ SAM 实参 → 实现方法
    ///
    /// 字节码调用点上的调用登记为读者单元（[`LCall`]）：接一次流边，此后只由其接收值（捕获 / 首个 SAM 实参）
    /// 的增长与 open 展开的 G 增长驱动增量重跑；调用点重跑、同一调用重入均不再进入
    pub(super) fn invoke_lambda(&mut self, m: usize, off: u32, lid: u32, a: &Args, ret: Option<u32>, res: Option<Node>) {
        let call: LambdaCall = (lid, a.clone(), ret, res);
        if self.methods[m].kind == Kind::Bytecode {
            let at = self.lambda_done.entry(m).or_default().entry(off).or_default();
            if at.contains_key(&call) {
                return;
            }
            let id = self.lcalls.len() as u32;
            at.insert(call.clone(), id);
            self.lcalls.push(LCall { m, off, call, done: TypeSet::default(), live: true });
            self.lambda_step(m, off, Some(id));
            return;
        }
        // 非字节码调用方：按当前值完整接边；同一 lambda 以相同实参重入即外层已接上全部流边
        if !self.lambda_stack.insert(call.clone()) {
            return;
        }
        self.lcalls.push(LCall { m, off, call: call.clone(), done: TypeSet::default(), live: false });
        self.lambda_step(m, off, None);
        let tmp = self.lcalls.pop();
        debug_assert!(tmp.is_some_and(|c| !c.live));
        self.lambda_stack.remove(&call);
    }

    /// lambda 调用读者的接收值增长 / open 展开的 G 增长：只处理增量
    pub(super) fn rerun_lcall(&mut self, id: u32) {
        let c = &self.lcalls[id as usize];
        if !c.live {
            return;
        }
        let (m, off) = (c.m, c.off);
        let site = self.cur_site.replace((m, off));
        let vals = self.call_vals.take();
        self.lambda_step(m, off, Some(id));
        self.call_vals = vals;
        self.cur_site = site;
    }

    /// 调用 id（None = 非字节码调用方的临时调用，位于 `lcalls` 末尾，每次完整接边）
    pub(super) fn lambda_step(&mut self, m: usize, off: u32, id: Option<u32>) {
        let at = id.map_or(self.lcalls.len() - 1, |i| i as usize);
        let (lid, a, ret, res) = self.lcalls[at].call.clone();
        let l = self.lambdas[&lid].clone();
        let k = &l.imh.member;
        let via = Via::method("lambda", m, Some(off));
        let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, l.imh.interface) else {
            self.unresolved.insert(k.to_string());
            return;
        };
        let (o, n, d) = site.key();
        let resolved = MemberRef { owner: o, name: n, desc: d };
        let mut all: Args = l.cap.clone();
        all.extend(a.iter().cloned());
        self.lambda_adapt(m, off, &l, &mut all, ret, res);
        let first = || all.first().cloned().flatten().unwrap_or_default();
        let rest = all.get(1..).unwrap_or(&[]).to_vec();
        match l.imh.kind {
            6 => {
                // 静态实现方法继承 lambda 创建时的克隆上下文；分派转发的实现方法按调用点克隆
                let cond = Some(format!("A:{}", self.names[lid as usize]));
                let ctx = self.static_ctx(m, off, &resolved, Call::Lambda(l.ctx));
                let t = super::cut::with_ctx(None, cond, || self.method_ctx(resolved, ctx, via));
                self.edge(m, off, t, Recv::None, &all, ret, res);
            }
            8 => {
                // 构造器引用：容器类在 lambda 创建点分配抽象对象
                let oid = if self.container(&k.owner) { self.obj_at(l.site.0, l.site.1, &k.owner) } else { self.id(&k.owner) };
                let cond = Some(format!("A:{}", self.names[lid as usize]));
                let cx = self.recv_ctx(oid);
                let t = super::cut::with_ctx(None, cond, || self.method_ctx(resolved, cx, via));
                self.edge(m, off, t, Recv::Exact(oid), &all, None, None);
                if let (Some(res), Some(rt)) = (res, ret) {
                    let s = self.filter(&TypeSet::exact(oid), rt);
                    self.add_to(res, &s);
                }
            }
            kind => {
                // 绑定接收者（捕获或首个 SAM 实参）：读值集时本调用登记为读者，只接增量
                let reader = std::mem::replace(&mut self.cur_call, id);
                let cur = self.value_set(&first());
                let done = std::mem::replace(&mut self.lcalls[at].done, cur.clone());
                let delta = TypeSet { classes: cur.classes.minus(&done.classes), open: cur.open.minus(&done.open) };
                if kind == 7 {
                    self.cur_call = reader;
                    if !delta.is_empty() {
                        self.edge_recv(m, off, resolved, via, vec![Feed::S(delta)], &rest, ret, res, false);
                    }
                    return;
                }
                // open 部分按 G 的当前成员展开（G 增长经 open 索引重跑本调用；已派发者由 `dispatched` 去重）
                let owner = self.id(&k.owner);
                let s = TypeSet { classes: delta.classes, open: cur.open };
                let recv = self.receivers(m, &s, owner);
                self.cur_call = reader;
                for r in recv {
                    self.dispatch_one(m, off, r, &site, &rest, ret, res, lid);
                }
            }
        }
    }

    /// 方法句柄（ldc MH / 引导方法及其静态实参）：由 VM / 库内部调用，实参按声明类型 open
    pub(super) fn invoke_mh(&mut self, m: usize, off: u32, mh: &MethodHandle) {
        let k = &mh.member;
        let via = Via::method("method-handle", m, Some(off));
        // 句柄按属主的成员（字段访问器 / 方法）发射，属主至少 L2
        self.touch(&k.owner, Level::Layout, via.clone());
        match mh.kind {
            // getField / getStatic / putField / putStatic
            1..=4 => {
                let opc = [0, classfile::op::GETFIELD, classfile::op::GETSTATIC, classfile::op::PUTFIELD, classfile::op::PUTSTATIC]
                    [mh.kind as usize];
                self.field(m, off, opc, k, None, None, Node::S(m, off));
            }
            _ => {
                let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, mh.interface) else {
                    self.unresolved.insert(k.to_string());
                    return;
                };
                let (o, n, d) = site.key();
                let resolved = MemberRef { owner: o, name: n, desc: d };
                let a = self.args_from(&k.desc, |t| vec![Feed::S(TypeSet::open(t))]);
                let owner = self.id(&k.owner);
                match mh.kind {
                    6 => {
                        self.init(&resolved.owner, via.clone());
                        let ctx = self.static_ctx(m, off, &resolved, Call::Handle);
                        let t = self.method_ctx(resolved, ctx, via);
                        self.edge(m, off, t, Recv::None, &a, None, None);
                    }
                    7 => {
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, Recv::Feeds(vec![Feed::S(TypeSet::open(owner))]), &a, None, None);
                    }
                    8 => {
                        self.instantiate(&k.owner, via.clone());
                        self.init(&k.owner, via.clone());
                        let t = self.method(resolved, via);
                        self.edge(m, off, t, Recv::Exact(owner), &a, None, None);
                    }
                    _ => {
                        let recv = self.receivers(m, &TypeSet::open(owner), owner);
                        for r in recv {
                            self.dispatch_one(m, off, r, &site, &a, None, None, NOCTX);
                        }
                    }
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn indy(&mut self, m: usize, off: u32, cf: &ClassFile, bsm: u16, name: &str, desc: &str, args: &[V]) {
        let via = Via::method("indy", m, Some(off));
        self.touch_desc(desc, &via);
        let Some(b) = cf.bootstrap_methods.get(bsm as usize).cloned() else { return };
        let Some(md) = parse_method(desc) else { return };
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
        let kind = self.man.indy_kind(&bkey);
        if let Some(k) = kind {
            // 运行模型替换的调用点：引导方法不执行（JVM 链接期对其所属类的加载不属翻译程序）
            self.indy_models.insert(format!("{}@{off}", self.methods[m].key), (b.handle.member.to_string(), k));
        }
        match kind {
            Some(IndyKind::Lambda) => {
                let (Some(Const::MethodHandle(imh)), Some(iface)) =
                    (b.args.get(1), md.ret.as_ref().and_then(|r| r.class_ref().map(String::from)))
                else {
                    return;
                };
                let mut cap: Args = Vec::with_capacity(md.params.len());
                for (p, v) in md.params.iter().zip(args.iter()) {
                    let f = self.ptype(p).map(|t| self.feeds(m, v, t));
                    cap.push(f);
                }
                let lname = format!("{}$$Lambda@{}:{}", cf.name, m, off);
                let lid = self.id(&lname);
                if super::cut::edges_on() {
                    super::cut::edge_plain(&format!("M:{}", self.methods[m].key), &format!("A:{lname}"));
                }
                let ctx = self.methods[m].ctx;
                let adapt = self.lambda_plan(&b.args, imh, cap.len());
                let markers = alt_markers(&b.args, self.man.serializable_markers());
                for x in &markers {
                    self.touch(x, Level::Type, via.clone());
                }
                self.lambdas.insert(lid, Lambda { site: (m, off), ctx, iface: iface.clone(), markers, sam: name.to_string(), imh: imh.clone(), cap, adapt });
                self.touch(&iface, Level::Alloc, via.clone());
                for a in &b.args {
                    if let Const::MethodType(d) = a {
                        self.touch_desc(d, &via);
                    }
                }
                self.add_to(Node::S(m, off), &TypeSet::exact(lid));
                if self.g.insert(lid) {
                    self.on_g_grow(lid);
                }
                // 静态 / 私有 / 构造实现在创建点即入链（实参随 SAM 调用接入）
                let k = &imh.member;
                if matches!(imh.kind, 6..=8) {
                    if let Some(site) = self.h.resolve_method(&k.owner, &k.name, &k.desc, imh.interface) {
                        let (o, n, d) = site.key();
                        if imh.kind == 6 {
                            self.init(&o, via.clone());
                        }
                        if imh.kind == 8 {
                            self.instantiate(&k.owner, via.clone());
                            self.init(&k.owner, via.clone());
                        }
                        let key = MemberRef { owner: o, name: n, desc: d };
                        let c = if imh.kind == 6 { self.static_ctx(m, off, &key, Call::Eager) } else { NOCTX };
                        self.method_ctx(key, c, via);
                    } else {
                        self.unresolved.insert(k.to_string());
                    }
                }
            }
            Some(IndyKind::Concat) => {
                self.instantiate(STRING, via.clone());
                let sid = self.id(STRING);
                self.add_to(Node::S(m, off), &TypeSet::exact(sid));
                let Some(site) = self.h.resolve_method(OBJECT, TO_STRING.0, TO_STRING.1, false) else { return };
                for (p, v) in md.params.iter().zip(args.iter()) {
                    let Some(t) = self.ptype(p) else { continue };
                    let fs = self.feeds(m, v, t);
                    let s = self.value_set(&fs);
                    let recv = self.receivers(m, &s, t);
                    for r in recv {
                        self.dispatch_one(m, off, r, &site, &vec![], None, None, NOCTX);
                    }
                }
            }
            Some(IndyKind::ObjectMethods) => self.object_methods(m, off, &b.args, name, &md, args),
            kind => {
                // 引导方法产出的调用点：结果按声明类型 open
                if let Some(rt) = ret {
                    self.add_to(Node::S(m, off), &TypeSet::open(rt));
                }
                if kind.is_none() {
                    let h = b.handle.clone();
                    self.invoke_mh(m, off, &h);
                }
                for a in &b.args {
                    match a {
                        Const::MethodHandle(x) => {
                            let x = x.clone();
                            self.invoke_mh(m, off, &x);
                        }
                        Const::Class(c) => {
                            self.touch(c, Level::Type, via.clone());
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    /// `ObjectMethods` 引导的 record equals / hashCode / toString：静态实参里的 getter 句柄读出各分量
    /// （接收者取 indy 实参，泛型 record 的抽象对象按对象读），引用分量的值集上派发同名 Object 方法。
    /// 描述符 = indy 描述符去掉首个 record 形参；equals 的实参取同一分量汇合节点（对方 record 的分量）
    fn object_methods(&mut self, m: usize, off: u32, bargs: &[Const], name: &str, md: &MethodDesc, args: &[V]) {
        let via = Via::method("indy", m, Some(off));
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        if let Some(rt) = ret {
            self.add_to(Node::S(m, off), &TypeSet::open(rt));
        }
        let node = Node::S(m, COMPONENTS | off);
        let mut any = false;
        for a in bargs {
            match a {
                Const::Class(c) => {
                    self.touch(c, Level::Type, via.clone());
                }
                Const::MethodHandle(x) if x.kind == 1 => {
                    let Some(ft) = parse_field(&x.member.desc) else { continue };
                    if !ft.is_reference() {
                        continue;
                    }
                    let k = x.member.clone();
                    for (p, v) in md.params.iter().zip(args.iter()) {
                        if p.is_reference() {
                            self.field(m, off, classfile::op::GETFIELD, &k, Some(v), None, node);
                        }
                    }
                    any = true;
                }
                Const::MethodHandle(x) => {
                    let x = x.clone();
                    self.invoke_mh(m, off, &x);
                }
                _ => {}
            }
        }
        if !any {
            return;
        }
        let odesc = object_method_desc(md);
        let Some(site) = self.h.resolve_method(OBJECT, name, &odesc, false) else {
            self.unresolved.insert(format!("{OBJECT}.{name}:{odesc}"));
            return;
        };
        let obj = self.id(OBJECT);
        let s = self.value_set(&[Feed::N(node)]);
        let a: Args = md.params.iter().skip(1).map(|p| p.is_reference().then(|| vec![Feed::N(node)])).collect();
        let recv = self.receivers(m, &s, obj);
        for r in recv {
            self.dispatch_one(m, off, r, &site, &a, ret, None, NOCTX);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn object_method_desc_drops_record_receiver() {
        let d = |s: &str| object_method_desc(&parse_method(s).unwrap());
        assert_eq!(d("(Lt/R;Lt/O;)Z"), "(Lt/O;)Z");
        assert_eq!(d("(Lt/R;)I"), "()I");
        assert_eq!(d("(Lt/R;)Lt/S;"), "()Lt/S;");
    }

    #[test]
    fn alt_markers_by_flags() {
        let ser = vec!["a/Ser".to_string()];
        let mt = || Const::MethodType("()V".into());
        let base = || vec![mt(), Const::Int(0), mt()];
        assert!(alt_markers(&base(), &ser).is_empty());
        let with = |extra: Vec<Const>| [base(), extra].concat();
        assert_eq!(alt_markers(&with(vec![Const::Int(1)]), &ser), ser);
        assert!(alt_markers(&with(vec![Const::Int(4), Const::Int(1), mt()]), &ser).is_empty());
        let m = alt_markers(&with(vec![Const::Int(3), Const::Int(1), Const::Class("a/M".into()), Const::Int(0)]), &ser);
        assert_eq!(m, vec!["a/Ser".to_string(), "a/M".to_string()]);
    }
}

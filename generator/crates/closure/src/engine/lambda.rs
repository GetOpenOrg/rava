//! 引擎：lambda / MethodHandle / indy。

use super::*;

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
    ///
    /// 方法引用到 @CallerSensitive 方法（`MethodHandles::lookup`）：进入实现方法的边以 lambda 类为调用方
    /// （生成器同判据以隐藏类名压栈）
    pub(super) fn lambda_step(&mut self, m: usize, off: u32, id: Option<u32>) {
        let at = id.map_or(self.lcalls.len() - 1, |i| i as usize);
        let lid = self.lcalls[at].call.0;
        let k = self.lambdas[&lid].imh.member.clone();
        let cs_site = self.ref_caller_sensitive(&k).then(|| (lid, k.name.to_string(), k.desc.to_string()));
        let outer = std::mem::replace(&mut self.cs.lambda_site, cs_site);
        self.lambda_connect(m, off, id);
        self.cs.lambda_site = outer;
    }

    /// 接边期间登记捕获值与接收者位置，实现方法的形参值按其对齐（`lambda_vals.rs`）
    fn lambda_connect(&mut self, m: usize, off: u32, id: Option<u32>) {
        let at = id.map_or(self.lcalls.len() - 1, |i| i as usize);
        let l = &self.lambdas[&self.lcalls[at].call.0];
        // 静态 / 构造实现的实参自首个起，其余实现的首个实参是接收者
        let skip = usize::from(!matches!(l.imh.kind, 6 | 8));
        let cap = super::lambda_vals::LambdaCap { site: l.site, vals: l.vals.clone(), skip };
        let outer = self.lambda_cap.replace(cap);
        self.lambda_dispatch(m, off, id);
        self.lambda_cap = outer;
    }

    /// 按实现句柄种类接边（静态 / 构造 / 特殊 / 虚分派）
    fn lambda_dispatch(&mut self, m: usize, off: u32, id: Option<u32>) {
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
                if matches!(mh.kind, 2 | 4) {
                    if let Some(site) = self.h.resolve_field(&k.owner, &k.name, &k.desc) {
                        let decl = site.class.name.clone();
                        self.static_mh_fields.insert((decl.clone(), k.name.clone()));
                        self.static_field_owner(&decl, via.clone());
                    }
                }
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

    /// 登记 lambda 对象（创建点 m@off 所在上下文 ctx；`bargs` 为 LambdaMetafactory 静态实参）：函数式接口入
    /// 实例化集，静态 / 私有 / 构造实现在创建点即入链（实参随 SAM 调用接入）。同名重复登记覆盖捕获来源
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new_lambda(
        &mut self,
        m: usize,
        off: u32,
        lname: &str,
        ctx: u32,
        (iface, sam): (String, &str),
        imh: &MethodHandle,
        (cap, vals): (Args, Option<Rc<[V]>>),
        bargs: &[Const],
    ) -> u32 {
        let via = Via::method("indy", m, Some(off));
        let lid = self.id(lname);
        if super::cut::edges_on() {
            super::cut::edge_plain(&self.site_node(m, off), &format!("A:{lname}"));
        }
        let adapt = self.lambda_plan(bargs, imh, cap.len());
        let markers = alt_markers(bargs, self.man.serializable_markers());
        for x in &markers {
            self.touch(x, Level::Type, via.clone());
        }
        self.lambdas.insert(lid, Lambda { site: (m, off), ctx, iface: iface.clone(), markers, sam: sam.to_string(), imh: imh.clone(), cap, vals, adapt });
        self.sysprops_lambda_new(imh);
        self.touch(&iface, Level::Alloc, via.clone());
        for a in bargs {
            if let Const::MethodType(d) = a {
                self.touch_desc(d, &via);
            }
        }
        if self.g.insert(lid) {
            self.on_g_grow(lid);
        }
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
        lid
    }

    /// indy 调用点；`known` = 实参值来自本方法的抽象分析帧（具体上下文的克隆为占位值，捕获值按未知处理）
    #[allow(clippy::too_many_arguments)]
    pub(super) fn indy(&mut self, m: usize, off: u32, cf: &ClassFile, bsm: u16, name: &str, desc: &str, args: &[V], known: bool) {
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
                let ctx = self.methods[m].ctx;
                let lid = self.new_lambda(m, off, &lname, ctx, (iface, name), imh, (cap, known.then(|| Rc::from(args))), &b.args);
                self.add_to(Node::S(m, off), &TypeSet::exact(lid));
            }
            Some(IndyKind::Concat) => {
                self.instantiate(STRING, via.clone());
                let sid = self.id(STRING);
                self.add_to(Node::S(m, off), &TypeSet::exact(sid));
                for (p, v) in md.params.iter().zip(args.iter()) {
                    let Some(t) = self.ptype(p) else { continue };
                    let fs = self.feeds(m, v, t);
                    let entry = self.man.indy_helpers.stringify.clone();
                    self.indy_helper(m, off, entry.as_deref(), std::slice::from_ref(v), vec![Some(fs)]);
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

    /// 拼接的引用实参 / record 的引用分量接入清单登记的 `[indy]` 分量处理入口（静态方法，
    /// 如 `String.valueOf(Object)` / `Objects.hashCode(Object)` / `Objects.equals(Object,Object)`）的形参：
    /// 生成器在调用点发射对它的静态调用。清单装载时已校验必填项；解析不到记入 unresolved，不回落
    fn indy_helper(&mut self, m: usize, off: u32, entry: Option<&str>, vals: &[V], fs: Args) {
        let Some(key) = entry.and_then(super::seeds::parse_member) else { return };
        let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, false) else {
            self.unresolved.insert(key.to_string());
            return;
        };
        let via = Via::method("indy", m, Some(off));
        let (o, n, d) = site.key();
        let resolved = MemberRef { owner: o, name: n, desc: d };
        self.init(&resolved.owner, via.clone());
        let heap = parse_method(&resolved.desc).is_some_and(|md| md.ret.iter().chain(&md.params).any(|r| r.is_reference()));
        let ctx = self.static_ctx(m, off, &resolved, Call::Invoke { heap, args: vals });
        let t = self.method_ctx(resolved, ctx, via);
        self.edge(m, off, t, Recv::None, &fs, None, None);
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
        // 引用分量接入对应的分量处理入口；equals 的两个实参（本方 / 对方分量）取同一汇合节点
        let h = &self.man.indy_helpers;
        let (entry, n) = match name {
            n if n == TO_STRING.0 => (h.stringify.clone(), 1),
            "hashCode" => (h.hash.clone(), 1),
            "equals" => (h.equals.clone(), 2),
            _ => return,
        };
        let fs: Args = vec![Some(vec![Feed::N(node)]); n];
        self.indy_helper(m, off, entry.as_deref(), &[], fs);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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

impl Engine<'_> {
    /// lambda 站点的函数式接口（samtype）集合：发射层据此为接口合成 `I__Lambda` 对象。
    /// 档案按入口取并（含用户方法里的站点）——JDK 侧的合成对象集合只依赖档案
    pub fn sam_types(&self) -> BTreeSet<String> {
        self.lambdas.values().map(|l| l.iface.clone()).collect()
    }
}

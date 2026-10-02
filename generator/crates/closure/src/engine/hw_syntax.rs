//! 引擎：手写体语法推断——类型引用解析、回调调用点与实参、字段访问器。

use super::*;

/// 手写体值的静态类型推断结果
pub(super) enum HwArg {
    /// 可实例化的类
    Exact(u32),
    /// 抽象类 / 接口：值是其某个已实例化子类的对象
    Open(u32),
    /// 推不出
    Unknown,
}

impl<'a> Engine<'a> {
    pub(super) fn resolve_tref(&self, host: &str, t: &TypeRef) -> Option<String> {
        self.hw.resolve_type(host, t).into_iter().find(|c| self.cp.contains(c))
    }

    /// 手写体调用点推断出的具体类型（须是可实例化的类）
    pub(super) fn hw_type(&mut self, host: &str, t: &Option<TypeRef>) -> Option<u32> {
        match self.hw_arg_type(host, t) {
            HwArg::Exact(id) => Some(id),
            _ => None,
        }
    }

    /// 手写体值的静态类型：可实例化类 → 精确；抽象类 / 接口 → 该类型（取其已实例化子类的 open 集）；推不出 → 未知
    pub(super) fn hw_arg_type(&mut self, host: &str, t: &Option<TypeRef>) -> HwArg {
        let Some(c) = t.as_ref().and_then(|t| self.resolve_tref(host, t)) else { return HwArg::Unknown };
        let Some(cf) = self.h.class(&c) else { return HwArg::Unknown };
        if cf.is_interface() || cf.access & acc::ABSTRACT != 0 {
            return HwArg::Open(self.id(&c));
        }
        HwArg::Exact(self.id(&c))
    }

    /// 回调 / 构造在手写体里的调用点：同名（Rust 名规则）、实参个数一致；
    /// 同名标识符出现在宏内（syn 不展开）或找不到调用点 → None（退回值池）
    pub(super) fn hw_sites<'b>(mh: &'b MemberHw, matches: impl Fn(&TypedCall) -> bool, java_name: &str, nargs: usize) -> Option<Vec<&'b TypedCall>> {
        if mh.opaque.iter().any(|i| member_matches(i, java_name)) {
            return None;
        }
        let v: Vec<&TypedCall> = mh.calls.iter().filter(|c| c.args.len() == nargs && matches(c)).collect();
        (!v.is_empty()).then_some(v)
    }

    /// 实参来源：各调用点该位置都推断出具体类型 → 精确类型集；否则值池。
    /// 静态类型为抽象类 / 接口的实参另并入 open(该类型)：手写体经语法不可见的途径（函数指针工厂、
    /// 注册表、缓存）构造的对象不在值池里，只以其静态类型的已实例化子类身份出现——不能因值池缺失推出空集
    pub(super) fn hw_args(&mut self, m: usize, host: &str, desc: &str, sites: &Option<Vec<&TypedCall>>) -> Args {
        let pool = Node::S(m, POOL);
        let Some(md) = parse_method(desc) else { return vec![] };
        let mut out = Vec::with_capacity(md.params.len());
        for (j, p) in md.params.iter().enumerate() {
            if self.ptype(p).is_none() {
                out.push(None);
                continue;
            }
            let mut exact = TypeSet::default();
            let mut open = TypeSet::default();
            let mut ok = sites.is_some();
            for c in sites.iter().flatten() {
                match self.hw_arg_type(host, &c.args[j]) {
                    HwArg::Exact(id) => {
                        exact.classes.insert(id);
                    }
                    HwArg::Open(id) => {
                        open.open.insert(id);
                        ok = false;
                    }
                    HwArg::Unknown => ok = false,
                }
            }
            let mut fs = if ok { vec![Feed::S(exact)] } else { vec![Feed::N(pool)] };
            if !open.is_empty() {
                fs.push(Feed::S(open));
            }
            out.push(Some(fs));
        }
        out
    }

    /// 类（含超类 / 超接口）中按名字找字段 → (声明类, 描述符)
    pub(super) fn field_by_name(&self, cls: &str, name: &str) -> Option<(String, String)> {
        let c = self.h.class(cls)?;
        if let Some(f) = c.fields.iter().find(|f| f.name == name) {
            return Some((c.name.clone(), f.desc.clone()));
        }
        c.super_name
            .iter()
            .chain(c.interfaces.iter())
            .find_map(|s| self.field_by_name(s, name))
    }

    /// 类（含超类型）中按名字找 static 字段 → (声明类, 描述符)
    pub(super) fn static_field(&self, cls: &str, name: &str) -> Option<(String, String)> {
        self.supertypes(cls)
            .iter()
            .find_map(|c| c.fields.iter().find(|f| f.name == name && f.is_static()).map(|f| (c.name.clone(), f.desc.clone())))
    }

    /// 手写体写入字段：只不折叠、读者失效。与按名取得（`open_field`）分开——手写体按 Rust 字段直接写，
    /// 不产出偏移，字段不因此可按偏移读写，也不接按名打开的静态字段写入
    fn hw_open_field(&mut self, key: &MemberRef) {
        if self.ctx.fhw.borrow_mut().insert(key.clone()) {
            let mut deps = self.ctx.ceval_drop_field(key);
            deps.extend(self.ctx.fdeps.borrow().get(key).into_iter().flatten().copied());
            self.invalidate_all(Some(deps), Why::FieldOpen);
        }
    }

    /// 手写体写入推不出所属类的字段名：同名字段只不折叠
    fn hw_open_field_name(&mut self, name: &str) {
        if self.ctx.fhw_names.borrow_mut().insert(name.to_string()) {
            let mut deps = self.ctx.ceval_drop_name(name);
            deps.extend(self.ctx.fdeps.borrow().iter().filter(|(k, _)| k.name == name).flat_map(|(_, v)| v.iter().copied()));
            self.invalidate_all(Some(deps), Why::FieldOpenName);
        }
    }

    /// 手写体字段访问器：写入值接进字段节点并登记「有手写写入」；读出值汇入值池。
    /// 接收者推不出 → 所有同名字段按 open 处理（安全回退）
    pub(super) fn hw_fields(&mut self, m: usize, host: &str, fields: &[FieldAccess]) {
        let pool = Node::S(m, POOL);
        let prod = Node::S(m, PROD);
        for fa in fields {
            let cls = fa.recv.as_ref().and_then(|r| self.stype_class(host, r));
            let site = cls.as_ref().and_then(|c| self.field_by_name(c, &fa.field));
            // static 写访问器形态的路径调用：类型上无此字段 → 同名手写辅助函数，不是字段写入
            if fa.path && cls.is_some() && site.is_none() {
                continue;
            }
            let Some((decl, desc)) = site else {
                if fa.write {
                    self.hw_open_field_name(&fa.field);
                }
                let fresh = if fa.write {
                    self.hw_written_names.insert(fa.field.clone())
                } else {
                    self.hw_read_names.entry(fa.field.clone()).or_default().insert(prod)
                };
                if !fresh {
                    continue;
                }
                let hit: Vec<(usize, String)> = self
                    .fields
                    .keys()
                    .enumerate()
                    .filter(|(_, k)| k.name == fa.field)
                    .map(|(i, k)| (i, k.desc.clone()))
                    .collect();
                for (i, d) in hit {
                    let Some(tid) = parse_field(&d).and_then(|t| self.ptype(&t)) else { continue };
                    if fa.write {
                        // 写入值取自值池、只以 open 出现在读者处：须逃逸
                        self.add_to(Node::U(i), &TypeSet::open(tid));
                        self.flow(pool, Node::Esc, tid);
                    } else {
                        self.flow(Node::F(i), prod, tid);
                    }
                }
                continue;
            };
            let key = MemberRef { owner: decl, name: fa.field.clone(), desc };
            let tid = parse_field(&key.desc).and_then(|t| self.ptype(&t));
            if fa.write {
                self.hw_written.insert(key.clone());
                self.hw_open_field(&key);
            }
            let Some(tid) = tid else { continue };
            let fi = self.field_node(key);
            if fa.on_self && !self.methods[m].is_static {
                let fs: Rc<[Feed]> = match (fa.write, self.hw_value(m, host, fa)) {
                    (false, _) => Rc::from([]),
                    (true, f) => Rc::from([f]),
                };
                let recv = Node::P(m, 0);
                self.self_fields.entry(recv).or_default().push((fi, tid, fa.write, fs, prod));
                let cur = self.graph.get(&recv).cloned().unwrap_or_default();
                self.self_field_objs(recv, &cur);
                continue;
            }
            if fa.write {
                let fs = vec![self.hw_value(m, host, fa)];
                self.feed(&fs, Node::U(fi), tid);
                // static 字段的手写写入值推不出类型：值未知，按字段声明类型的实例（open）
                if fa.path && matches!(fs[0], Feed::N(n) if n == pool) {
                    self.add_to(Node::U(fi), &TypeSet::open(tid));
                }
            } else {
                self.flow(Node::F(fi), prod, tid);
            }
        }
    }

    /// 手写字段写入的值来源：写入值是被调 Java 方法的接收者 → 接收者参数节点（值即其确定值集，
    /// 如 `__set_clazz(Clone::clone(self))` 的声明类镜像）；语法推得类型 → 该类型的确定实例；否则取自值池
    fn hw_value(&mut self, m: usize, host: &str, fa: &FieldAccess) -> Feed {
        if fa.value_self && !self.methods[m].is_static {
            return Feed::N(Node::P(m, 0));
        }
        match self.hw_type(host, &fa.value) {
            Some(id) => Feed::S(TypeSet::exact(id)),
            None => Feed::N(Node::S(m, POOL)),
        }
    }

    /// 手写体访问接收者自身字段：抽象对象接其字段节点，非抽象接收者（类本身 / open）经未知接收者视图
    pub(super) fn self_field_objs(&mut self, recv: Node, delta: &TypeSet) {
        let Some(acc) = self.self_fields.get(&recv).cloned() else { return };
        let raw = !delta.open.is_empty() || delta.classes.iter().any(|x| !self.objs.contains_key(&x));
        let objs: Vec<u32> = delta.classes.iter().filter(|x| self.objs.contains_key(x)).collect();
        for (fi, tid, write, fs, prod) in acc.iter() {
            let (fi, tid) = (*fi, *tid);
            for &o in &objs {
                let n = self.obj_field(o, fi, tid);
                if *write { self.feed(fs, n, tid) } else { self.flow(n, *prod, tid) }
            }
            if raw {
                if *write { self.feed(fs, Node::U(fi), tid) } else { self.flow(Node::F(fi), *prod, tid) }
            }
        }
    }
}

//! 引擎：直连反射调用（清单 `[facts.reflect.direct_invokers]`，见 `manifest/direct.rs`）。
//!
//! 反射调用入口（Method.invoke）的体对任意反射对象判 CS、取访问器、两路调用访问器，分支取舍取决于运行期的
//! 反射对象，经它的每个调用点都把注解解析与 CS 访问器链带进闭包。调用点满足下列条件时，反射对象只可能是
//! 解析出的非 CS 目标之一，原入口的执行与清单登记的特化入口（helper）逐句一致：
//! 1. 反射对象（接收者）唯一来自本方法内一个清单所列查找入口的调用点；
//! 2. 查找的名字是常量，或能按拼接段（`method_lookup.rs` 同口径）在查找口径可见范围内取出候选名（超集，逐个解析）；
//!    形参类型数组恒为空（常量长度 0 的数组或 null）；查找类集取类字面量 / Class 值集里字节码类
//!    镜像所指的类，类集为空时查找恒抛异常、不产生目标（名字无关）。值集里所指未知的部分（open、非镜像的 Class 对象、隐藏 / 代理类镜像）按反射缺口口径处理
//!    （同 `method_lookup.rs::recv_mirrors`：查找点自身已记缺口，分析器不做模糊扩展）——缺口类上的成员本就不经
//!    该查找点进入调用链，原入口与 helper 在缺口上同样落到调用链外（helper 的 native 与原入口的 native 访问器共用
//!    `reflect_dispatch::native_invoke`：非 CS 方法的实参检查、空接收者 NPE、装箱、按接收者虚派发逐句一致；CS 方法只在 JDK 内且
//!    名字可见，缺口不改变此前提），不因缺口回退；
//! 3. 按查找口径（public：公开成员沿超类链；declared：只查声明类）在各类上解析出的目标全是无参、非 @CallerSensitive
//!    的方法（解析不到 = 查找抛 NoSuchMethodException，不产生目标；有超接口同名无参方法参与选择时不直连）。
//!
//! 满足时调用点接到 helper（静态调用，按 @CallerSensitive 声明压栈调用方所在类）与各目标：静态目标直接接边，实例目标
//! 按 invoke 的接收者实参值集经枢纽虚派发（同字节码虚调用）；引用返回值流入调用点结果，基本类型返回值经装箱类
//! `valueOf` 流入（同本地访问器）；
//! 目标入反射分派面（`reflect_members`，helper 的 native 按声明键分派），不再接原入口——Method.invoke 体内的
//! CS 判定、注解解析、访问器链不经本调用点入链。条件随值集增长可由真变假（查找类集新增的类上解析出不可直连的目标等）：
//! 一旦不满足，调用点记入 `rdirect_fallback`，此后恒按原入口接边（已接的直连边保留，过近似），导出时不改写。
//! 导出见 `report.rs`（folds `direct_calls`：全部克隆在该点都按直连处理时才改写）。

use super::*;
use crate::manifest::{DirectInvoker, LookupScope};

/// 常量长度 0 的数组或 null（`Class.getMethod` 的形参类型实参：null 与空数组同义）
fn empty_array(v: &V) -> bool {
    match v {
        V::Null => true,
        V::Ref { obj: Some(o), .. } => matches!(**o, Obj::Len(0)),
        _ => false,
    }
}

/// 类上名为 name 的无参方法（`()` 描述符）；public = 只取公开方法
fn nullary<'c>(cf: &'c ClassFile, name: &str, public: bool) -> Vec<&'c classfile::Method> {
    cf.methods.iter().filter(|x| x.name == name && x.desc.starts_with("()") && (!public || x.access & acc::PUBLIC != 0)).collect()
}

impl<'a> Engine<'a> {
    /// 字节码调用点按直连处理：true = 已接 helper 与各目标，调用方不再按原入口接边
    pub(super) fn reflect_direct(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) -> bool {
        if opcode != classfile::op::INVOKEVIRTUAL || self.man.direct_invokers.is_empty() || self.rdirect_fallback.contains(&(m, off)) {
            return false;
        }
        let key = self.mref_key(mref);
        let man = self.man;
        let Some(spec) = man.direct_invokers.get(&key) else { return false };
        match self.direct_targets(m, off, spec, args) {
            Some(ts) => {
                self.direct_edges(m, off, spec, args, ts);
                true
            }
            None => {
                self.rdirect_fallback.insert((m, off));
                false
            }
        }
    }

    /// 直连条件成立时的目标集（查找类集尚空 = 空集）；None = 不满足
    fn direct_targets(&mut self, m: usize, off: u32, spec: &DirectInvoker, args: &[V]) -> Option<Vec<MemberRef>> {
        // 读本方法查找调用点的事件：登记为跨偏移读者（重分析时一并重跑）
        self.xreaders.entry(m).or_default().insert(off);
        let o = class_lookup::site_of(args.first()?)?;
        let a = self.analysis(m)?;
        if a.conservative {
            return None;
        }
        let Some(Event::Invoke { opcode, mref, args: largs, .. }) = class_lookup::event_at(&a, o, class_lookup::is_invoke) else { return None };
        let lk = self.mref_key(mref);
        let scope = *spec.lookups.get(&*lk)?;
        let md = parse_method(&mref.desc)?;
        let base = usize::from(*opcode != classfile::op::INVOKESTATIC);
        let class_arr = format!("[L{CLASS};");
        let mut name = None;
        let mut classes = BTreeSet::new();
        for (j, p) in md.params.iter().enumerate() {
            let v = largs.get(base + j)?;
            match p {
                FieldType::Object(c) if c == STRING => name = Some(v.clone()),
                FieldType::Object(c) if c == CLASS => self.lookup_classes(m, v, &mut classes),
                _ if p.descriptor() == class_arr => {
                    if !empty_array(v) {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        if base == 1 {
            if mref.owner != CLASS {
                return None;
            }
            self.lookup_classes(m, largs.first()?, &mut classes);
        }
        let name = name?;
        // 查找类集为空：查找恒抛异常（或落在反射缺口上），不产生目标，名字无关
        if classes.is_empty() {
            return Some(Vec::new());
        }
        let mut out = Vec::new();
        for c in &classes {
            for n in self.direct_names(m, &name, c, scope)? {
                out.extend(self.direct_resolve(c, &n, scope)?);
            }
        }
        Some(out)
    }

    /// 查找名字 v 在类 cls 上的候选：常量即其本身；否则按拼接段（`method_lookup.rs` 同口径）取查找口径可见范围
    /// （declared：声明类；public：类链与全部超接口）里能由拼接段拼出的方法名（超集，逐个解析）。None = 推不出
    fn direct_names(&mut self, m: usize, v: &V, cls: &str, scope: LookupScope) -> Option<BTreeSet<Rc<str>>> {
        if let V::Str(s, _) = v {
            return Some([s.clone()].into());
        }
        let parts = self.method_name_parts(m, v)?;
        if cls.starts_with('[') {
            return match scope {
                LookupScope::Declared => Some(BTreeSet::new()),
                LookupScope::Public => Some(self.declared_matching(OBJECT, &parts)),
            };
        }
        let mut out = BTreeSet::new();
        let scan: Vec<String> = match scope {
            LookupScope::Declared => vec![cls.to_string()],
            LookupScope::Public => self.type_closure(cls)?,
        };
        for c in &scan {
            out.extend(self.declared_matching(c, &parts));
        }
        Some(out)
    }

    /// cls 的类链与全部（传递）超接口；None = 有类不可得
    fn type_closure(&self, cls: &str) -> Option<Vec<String>> {
        let mut out = Vec::new();
        let mut stack: Vec<String> = Vec::new();
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            let cf = self.h.class(&c)?;
            stack.extend(cf.interfaces.iter().cloned());
            cur = cf.super_name.clone();
            out.push(c);
        }
        let mut seen = BTreeSet::new();
        while let Some(i) = stack.pop() {
            if !seen.insert(i.clone()) {
                continue;
            }
            let cf = self.h.class(&i)?;
            stack.extend(cf.interfaces.iter().cloned());
            out.push(i);
        }
        Some(out)
    }

    /// 按查找口径在类 cls 上解析无参方法 name：Some(None) = 查找抛 NoSuchMethodException；
    /// None = 解析出的不是可直连目标（实例 / CS / 基本类型返回 / 多义）或类不可得
    fn direct_resolve(&mut self, cls: &str, name: &str, scope: LookupScope) -> Option<Option<MemberRef>> {
        if cls.starts_with('[') {
            // 数组类没有声明方法；公开成员即根类的公开成员（JLS §10.7）
            return match scope {
                LookupScope::Declared => Some(None),
                LookupScope::Public => self.direct_resolve(OBJECT, name, scope),
            };
        }
        let cf = self.h.class(cls)?;
        let hit = match scope {
            LookupScope::Declared => {
                let hs = nullary(&cf, name, false);
                if hs.len() > 1 {
                    return None;
                }
                hs.first().map(|x| self.direct_target(&cf.name, x))
            }
            LookupScope::Public => {
                // 超接口的公开实例方法参与 getMethod 的选择（静态接口方法不被继承）：有同名无参者不直连
                if self.iface_nullary(cls, name)? {
                    return None;
                }
                let mut cur = Some(cf);
                let mut found = None;
                let mut hops = 0;
                while let Some(c) = cur {
                    let hs = nullary(&c, name, true);
                    if hs.len() > 1 {
                        return None;
                    }
                    if let Some(x) = hs.first() {
                        found = Some(self.direct_target(&c.name, x));
                        break;
                    }
                    hops += 1;
                    if hops > 64 {
                        return None;
                    }
                    cur = match &c.super_name {
                        Some(s) => Some(self.h.class(s)?),
                        None => None,
                    };
                }
                found
            }
        };
        match hit {
            // 查找抛 NoSuchMethodException：调用点不可达 invoke
            None => Some(None),
            Some(t) => Some(Some(t?)),
        }
    }

    /// 解析出的方法 x（声明于 owner）可直连时的成员键：非 @CallerSensitive（静态与实例方法都可；实例方法在调用点
    /// 按 invoke 的接收者实参虚派发，基本类型返回值经装箱流入结果，均同本地访问器）
    fn direct_target(&self, owner: &str, x: &classfile::Method) -> Option<MemberRef> {
        let cs = x.annotations.iter().any(|a| self.man.is_caller_sensitive_annotation(&a.type_desc));
        if cs {
            return None;
        }
        Some(MemberRef { owner: owner.to_string(), name: x.name.clone(), desc: x.desc.clone() })
    }

    /// 查找类集：Class 值 v 所指的字节码类并入 out（类字面量；值集里的类镜像，值集增长时本站点重跑）。
    /// 所指未知的部分（open、非镜像的 Class 对象、隐藏 / 代理类镜像）是反射缺口，不并入；基本类型类镜像没有方法，
    /// 查找恒抛异常，不并入
    fn lookup_classes(&mut self, m: usize, v: &V, out: &mut BTreeSet<String>) {
        let V::Ref { .. } = v else {
            self.class_values(m, v, out);
            return;
        };
        let class = self.id(CLASS);
        let fs = self.feeds(m, v, class);
        let s = self.value_set(&fs);
        for x in s.classes.iter() {
            if let Some(&c) = self.mirrors.get(&x) {
                out.insert(self.names[c as usize].to_string());
            }
        }
    }

    /// cls 的类链上全部超接口（传递）里有名为 name 的无参非静态方法；None = 有类不可得
    fn iface_nullary(&self, cls: &str, name: &str) -> Option<bool> {
        let mut stack: Vec<String> = Vec::new();
        let mut cur = Some(cls.to_string());
        while let Some(c) = cur {
            let cf = self.h.class(&c)?;
            stack.extend(cf.interfaces.iter().cloned());
            cur = cf.super_name.clone();
        }
        let mut seen = BTreeSet::new();
        while let Some(i) = stack.pop() {
            if !seen.insert(i.clone()) {
                continue;
            }
            let cf = self.h.class(&i)?;
            if nullary(&cf, name, false).iter().any(|x| x.access & acc::STATIC == 0) {
                return Some(true);
            }
            stack.extend(cf.interfaces.iter().cloned());
        }
        Some(false)
    }

    /// 直连接边：调用点 → helper（实参原样，结果不取 helper 返回值）；调用点 → 各目标（结果 = 目标返回值）
    fn direct_edges(&mut self, m: usize, off: u32, spec: &DirectInvoker, args: &[V], ts: Vec<MemberRef>) {
        let helper = spec.helper.clone();
        self.rdirect.insert((m, off), helper.clone());
        self.note_ref(&helper);
        let via = Via::method("invoke", m, Some(off));
        self.touch(&helper.owner, Level::Layout, via.clone());
        self.init(&helper.owner, via.clone());
        let Some(hmd) = parse_method(&helper.desc) else { return };
        let mut a: Args = Vec::with_capacity(hmd.params.len());
        for (p, v) in hmd.params.iter().zip(args.iter()) {
            let f = self.ptype(p).map(|t| self.feeds(m, v, t));
            a.push(f);
        }
        let hret = hmd.ret.as_ref().and_then(|r| self.ptype(r));
        // helper 标注 @CallerSensitive：生成器在改写后的调用点压入调用方所在类，分析同口径
        let wrapped = self.ref_caller_sensitive(&helper);
        let outer = std::mem::replace(&mut self.cs.site_wrapped, wrapped);
        let vals = self.call_vals.replace(Rc::from(args));
        let t = self.method(helper, via.clone());
        self.edge(m, off, t, Recv::None, &a, hret, None);
        self.cs.site_wrapped = false;
        self.call_vals = Some(Rc::from([].as_slice()));
        let obj = self.id(OBJECT);
        let res = Node::S(m, off);
        for key in ts {
            self.init(&key.owner, via.clone());
            self.reflect_members.insert((Members::Methods, key.clone()));
            let ret = key.desc.rsplit_once(')').map_or(b'V', |(_, r)| r.as_bytes()[0]);
            let ret_ref = matches!(ret, b'L' | b'[');
            // 基本类型返回值由本地访问器装箱（`装箱类.valueOf`），装箱结果流入调用点结果
            if !ret_ref && ret != b'V' {
                self.box_edge(m, off, ret, &via, None, Some(res));
            }
            let rt = ret_ref.then_some(obj);
            let rn = ret_ref.then_some(res);
            let Some(cf) = self.h.class(&key.owner) else { continue };
            let is_static = cf.method(&key.name, &key.desc).is_some_and(|x| x.is_static());
            if is_static {
                let t = self.method(key, via.clone());
                self.edge(m, off, t, Recv::None, &[], rt, rn);
            } else {
                // 实例目标：按 invoke 的接收者实参（声明类过滤）虚派发，同字节码 invokevirtual / invokeinterface
                let Some(recv) = args.get(1) else { continue };
                self.direct_virtual(m, off, &key, cf.is_interface(), recv, rn, &via);
            }
        }
        self.call_vals = vals;
        self.cs.site_wrapped = outer;
    }

    /// 实例目标 key 在调用点 (m, off) 上按接收者值 recv 派发：精确接收者与 open 部分各经枢纽（枢纽按成员与接收者集合
    /// 为键，同一调用点上的多个目标互不干扰）
    #[allow(clippy::too_many_arguments)]
    fn direct_virtual(&mut self, m: usize, off: u32, key: &MemberRef, iface: bool, recv: &V, res: Option<Node>, via: &Via) {
        let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else {
            self.unresolved.insert(key.to_string());
            return;
        };
        let Some(md) = parse_method(&key.desc) else { return };
        let owner = self.id(&key.owner);
        let fs = self.feeds(m, recv, owner);
        let s = self.value_set(&fs);
        let exact = TypeSet { classes: s.classes.clone(), open: IdSet::default() };
        let rs: Rc<[u32]> = self.receivers(m, &exact, owner).into();
        if !rs.is_empty() {
            let h = self.hub(key, iface, owner, HubSet::Exact(rs), None, &site, &md, via.clone());
            self.link_hub(h, m, off, &Vec::new(), res);
        }
        let opens: Vec<u32> = s.open.iter().collect();
        for o in opens {
            let h = self.hub(key, iface, owner, HubSet::Open(o), None, &site, &md, via.clone());
            self.link_hub(h, m, off, &Vec::new(), res);
        }
    }

    /// 方法 m 偏移 pc 的调用点按直连处理（从未回退）时的特化入口
    pub(super) fn direct_call_of(&self, m: usize, pc: u32) -> Option<&MemberRef> {
        if self.rdirect_fallback.contains(&(m, pc)) {
            return None;
        }
        self.rdirect.get(&(m, pc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_array_shapes() {
        assert!(empty_array(&V::Null));
        let len = |n| V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Len(n))) };
        assert!(empty_array(&len(0)));
        assert!(!empty_array(&len(1)));
        assert!(!empty_array(&V::Top));
        assert!(!empty_array(&V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: None }));
    }
}

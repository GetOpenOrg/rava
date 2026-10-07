//! 引擎：直连反射调用（清单 `[facts.reflect.direct_invokers]`，见 `manifest/direct.rs`）。
//!
//! 反射调用入口（Method.invoke）的体对任意反射对象判 CS、取访问器、两路调用访问器，分支取舍取决于运行期的
//! 反射对象，经它的每个调用点都把注解解析与 CS 访问器链带进闭包。调用点满足下列条件时，反射对象只可能是
//! 解析出的非 CS 静态目标之一，原入口的执行与清单登记的特化入口（helper）逐句一致：
//! 1. 反射对象（接收者）唯一来自本方法内一个清单所列查找入口的调用点；
//! 2. 查找的名字是常量，形参类型数组恒为空（常量长度 0 的数组或 null）；查找类集取类字面量 / Class 值集里字节码类
//!    镜像所指的类。值集里所指未知的部分（open、非镜像的 Class 对象、隐藏 / 代理类镜像）按反射缺口口径处理
//!    （同 `method_lookup.rs::recv_mirrors`：查找点自身已记缺口，分析器不做模糊扩展）——缺口类上的成员本就不经
//!    该查找点进入调用链，原入口与 helper 在缺口上同样落到调用链外（helper 的 native 与原入口的 native 访问器共用
//!    `reflect_dispatch::native_invoke`：非 CS 方法的实参检查、空接收者 NPE、装箱逐句一致；CS 方法只在 JDK 内且
//!    名字可见，缺口不改变此前提），不因缺口回退；
//! 3. 按查找口径（public：公开成员沿超类链；declared：只查声明类）在各类上解析出的目标全是无参、返回引用或 void、
//!    非 @CallerSensitive 的静态方法（解析不到 = 查找抛 NoSuchMethodException，不产生目标；有超接口同名无参方法
//!    参与选择时不直连）。
//!
//! 满足时调用点接到 helper（静态调用，按 @CallerSensitive 声明压栈调用方所在类）与各目标（结果 = 目标返回值），
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
                FieldType::Object(c) if c == STRING => match v {
                    V::Str(s, _) => name = Some(s.clone()),
                    _ => return None,
                },
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
        let mut out = Vec::new();
        for c in &classes {
            out.extend(self.direct_resolve(c, &name, scope)?);
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

    /// 解析出的方法 x（声明于 owner）可直连时的成员键：静态、非 @CallerSensitive、返回引用或 void
    fn direct_target(&self, owner: &str, x: &classfile::Method) -> Option<MemberRef> {
        let ret = &x.desc[2..];
        let cs = x.annotations.iter().any(|a| self.man.is_caller_sensitive_annotation(&a.type_desc));
        if x.access & acc::STATIC == 0 || cs || !(ret == "V" || ret.starts_with('L') || ret.starts_with('[')) {
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
        for key in ts {
            self.init(&key.owner, via.clone());
            self.reflect_members.insert((Members::Methods, key.clone()));
            let t = self.method(key, via.clone());
            self.edge(m, off, t, Recv::None, &[], Some(obj), Some(Node::S(m, off)));
        }
        self.call_vals = vals;
        self.cs.site_wrapped = outer;
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

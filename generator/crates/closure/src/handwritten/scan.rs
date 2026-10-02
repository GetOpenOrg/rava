//! 手写文件的 syn 扫描：逐 fn 收集调用、分配、字段访问，同文件 fn 调用的传递闭包，use 表与 prelude。

use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use syn::visit::Visit;

use super::generic_fns::*;
use super::stype::*;
use super::syntax::*;
use super::*;

pub(super) struct RawFn {
    pub(super) info: FnInfo,
    pub(super) calls: HashSet<String>,
}

/// 手写实现对象的扫描原料（trait impl 与固有 impl 的 fn，未闭包）
#[derive(Default)]
pub(super) struct RawObject {
    pub(super) supers: BTreeSet<TypeRef>,
    pub(super) fns: HashMap<String, FnInfo>,
    pub(super) calls: HashMap<String, HashSet<String>>,
}

/// 一个类的手写文件扫描结果（未闭包）
#[derive(Default)]
pub(super) struct FileFns {
    pub(super) fns: HashMap<String, FnInfo>,
    pub(super) calls: HashMap<String, HashSet<String>>,
    pub(super) objects: BTreeMap<String, RawObject>,
    /// 文件顶层 impl 块关联 fn 的返回类型（见 [`local_rets`]）
    pub(super) rets: LocalRets,
}

pub(super) struct FileScan<'a> {
    pub(super) uses: &'a HashMap<String, Vec<String>>,
    /// 本文件的手写实现对象 struct 名
    pub(super) local_objects: &'a HashSet<String>,
    pub(super) fns: Vec<(String, RawFn)>,
    /// 手写实现对象的 fn：(struct 名, fn 名, 原料)；实现的 Java 类型
    pub(super) object_fns: Vec<(String, String, RawFn)>,
    pub(super) supers: Vec<(String, TypeRef)>,
    /// 当前 impl 块的 self 类型
    pub(super) self_ty: Option<Vec<String>>,
    /// 当前 impl 块的 self 类型是本文件的手写实现对象
    pub(super) cur_obj: Option<String>,
    /// 本文件 impl 块关联 fn 的返回类型（见 [`local_rets`]）
    pub(super) rets: &'a LocalRets,
    /// 本文件构造器名形态的辅助 fn（见 [`local_helpers`]）
    pub(super) helpers: &'a HashSet<String>,
    /// 本文件带闭包形参的泛型辅助 fn（见 [`generic_fns`]）
    pub(super) generics: &'a GenericFns,
}

/// 标识符收集（宏外；宏内标识符另经 `macro_idents` 收集）
struct Idents(HashSet<String>);

impl<'ast> Visit<'ast> for Idents {
    fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
        self.0.insert(i.to_string());
    }
}

impl FileScan<'_> {
    fn add(&mut self, sig: &syn::Signature, is_pub: bool, block: &syn::Block) {
        let name = sig.ident.to_string();
        let helpers = helpers_in(self.helpers, self.self_ty.as_ref());
        let mut b = BodyScan { helpers: helpers.clone(), ..Default::default() };
        b.visit_block(block);
        let mut scope = HashMap::new();
        for a in &sig.inputs {
            match a {
                syn::FnArg::Receiver(_) => {
                    scope.insert("self".to_string(), Some(SType::Named(TypeRef(vec!["Self".into()]))));
                }
                syn::FnArg::Typed(pt) => {
                    if let syn::Pat::Ident(pi) = &*pt.pat {
                        scope.insert(pi.ident.to_string(), type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))));
                        scope.insert(elem_key(&pi.ident.to_string()), elem_type(&pt.ty).map(|p| SType::Named(TypeRef(p))));
                    }
                }
            }
        }
        let mut info = FnInfo { is_pub, ..Default::default() };
        for (var, ty) in &b.defaults {
            if b.inited.contains(var) {
                info.allocs.insert(TypeRef(expand(self.uses, ty.clone())));
                b.locals.insert(var.clone(), Some(ty.clone()));
            }
        }
        let mut cs = CallScan { locals: &b.locals, scope, fresh: HashMap::new(), calls: Vec::new(), fields: Vec::new(), opaque: HashSet::new(), helpers: &helpers, generics: self.generics, self_last: self.self_ty.as_ref().and_then(|t| t.last().cloned()) };
        cs.visit_block(block);
        for (field, write, recv, value, on_self, path) in cs.fields {
            info.fields.push(FieldAccess {
                on_self,
                path,
                field,
                write,
                recv: recv.and_then(|r| local_ret(expand_s(self.uses, r, &self.self_ty), self.rets)),
                value: value.map(|v| TypeRef(expand(self.uses, v))),
            });
        }
        let tr = |t: Option<Vec<String>>| t.map(|t| TypeRef(expand(self.uses, t)));
        for (mut name, mut ty, mut recv, mut args, fresh, mut srecv) in cs.calls {
            // vtable trait 的完全限定调用 `X__VTable::m(&*recv, …)`：首个实参是接收者，即 X 上的虚调用
            if recv.is_none() && !args.is_empty() {
                if let Some(t) = ty.as_ref().map(|t| expand(self.uses, t.clone())).as_ref().and_then(|t| Some((t, t.last()?.strip_suffix(VTABLE_SUFFIX)?))).filter(|(_, s)| !s.is_empty()).map(|(t, s)| [&t[..t.len() - 1], &[s.to_string()]].concat()) {
                    recv = Some(args.remove(0));
                    srecv = Some(SType::Named(TypeRef(t)));
                    ty = None;
                }
            }
            // 经 use 引入的自由 fn（`use crate::m::f;` 后的 `f(…)`）：路径取引入的全路径
            if ty.is_none() && recv.is_none() {
                if let Some((last, head)) = self.uses.get(&name).and_then(|full| full.split_last()).filter(|(_, h)| !h.is_empty()) {
                    (name, ty) = (last.clone(), Some(head.to_vec()));
                }
            }
            info.calls.push(TypedCall {
                name,
                path_ty: tr(ty),
                recv: recv.map(tr),
                args: args.into_iter().map(tr).collect(),
                fresh: tr(fresh),
                srecv: srecv.and_then(|r| local_ret(expand_s(self.uses, r, &self.self_ty), self.rets)),
            });
        }
        info.opaque = cs.opaque;
        let mut ids = ArrayIdents::default();
        ids.visit_signature(sig);
        ids.visit_block(block);
        info.array_access = ids.writes() || opaque_array_writes(&info.opaque);
        for (ty, ctor) in b.ctors {
            info.ctors.insert((TypeRef(expand(self.uses, ty)), ctor));
        }
        let mut ids = Idents(HashSet::new());
        ids.visit_block(block);
        ids.0.extend(info.opaque.iter().cloned());
        info.objects = ids.0.iter().filter_map(|i| self.object_ref(i)).collect();
        let raw = RawFn { info, calls: b.calls };
        match &self.cur_obj {
            Some(o) => self.object_fns.push((o.clone(), name, raw)),
            None => self.fns.push((name, raw)),
        }
    }

    /// 标识符指称的手写实现对象：本文件的实现对象 struct，或经 `use` 从共置手写模块（`…::<类>_impl::S`）引入的
    fn object_ref(&self, ident: &str) -> Option<TypeRef> {
        if self.local_objects.contains(ident) {
            return Some(TypeRef(vec![ident.to_string()]));
        }
        let full = self.uses.get(ident)?;
        let n = full.len();
        let module = full.get(n.checked_sub(2)?)?;
        let upper = ident.starts_with(|c: char| c.is_ascii_uppercase());
        (upper && MODULE_SUFFIXES.iter().any(|x| module.ends_with(x))).then(|| TypeRef(full.clone()))
    }
}

impl<'ast> Visit<'ast> for FileScan<'_> {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        let is_pub = matches!(f.vis, syn::Visibility::Public(_));
        // 自由 fn 内的 Self 无意义：暂离 impl 上下文
        let outer = self.self_ty.take();
        let outer_obj = self.cur_obj.take();
        self.add(&f.sig, is_pub, &f.block);
        syn::visit::visit_item_fn(self, f);
        self.self_ty = outer;
        self.cur_obj = outer_obj;
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let ty = type_path(&i.self_ty);
        let obj = ty.as_ref().filter(|t| t.len() == 1 && self.local_objects.contains(&t[0])).map(|t| t[0].clone());
        if let (Some(o), Some(tr)) = (&obj, vtable_trait(i)) {
            // trait 名可能经 use 引入：先展开，再去后缀得 Java 类型的 Rust 路径
            let mut full = expand(self.uses, tr);
            if let Some(last) = full.last_mut().and_then(|l| l.strip_suffix(VTABLE_SUFFIX).map(str::to_string)) {
                *full.last_mut().expect("非空路径") = last;
                self.supers.push((o.clone(), TypeRef(full)));
            }
        }
        let outer = std::mem::replace(&mut self.self_ty, ty);
        let outer_obj = std::mem::replace(&mut self.cur_obj, obj);
        syn::visit::visit_item_impl(self, i);
        self.self_ty = outer;
        self.cur_obj = outer_obj;
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let is_pub = matches!(f.vis, syn::Visibility::Public(_));
        self.add(&f.sig, is_pub, &f.block);
        syn::visit::visit_impl_item_fn(self, f);
    }
}

pub(super) fn collect_uses(tree: &syn::UseTree, prefix: &mut Vec<String>, out: &mut HashMap<String, Vec<String>>) {
    match tree {
        syn::UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            collect_uses(&p.tree, prefix, out);
            prefix.pop();
        }
        syn::UseTree::Name(n) => {
            let mut full = prefix.clone();
            full.push(n.ident.to_string());
            out.insert(n.ident.to_string(), full);
        }
        syn::UseTree::Rename(r) => {
            let mut full = prefix.clone();
            full.push(r.ident.to_string());
            out.insert(r.rename.to_string(), full);
        }
        syn::UseTree::Group(g) => {
            for t in &g.items {
                collect_uses(t, prefix, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

pub(super) struct UseScan(pub(super) HashMap<String, Vec<String>>);

impl<'ast> Visit<'ast> for UseScan {
    fn visit_item_use(&mut self, u: &'ast syn::ItemUse) {
        collect_uses(&u.tree, &mut Vec::new(), &mut self.0);
    }
}

/// `mod prelude { pub use super::… }` → 名字 → `crate::…` 路径
pub(super) fn prelude_uses(file: &syn::File) -> HashMap<String, Vec<String>> {
    let mut us = UseScan(HashMap::new());
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            if m.ident == "prelude" {
                if let Some((_, items)) = &m.content {
                    for i in items {
                        us.visit_item(i);
                    }
                }
            }
        }
    }
    us.0.into_iter()
        .map(|(k, mut v)| {
            if v.first().is_some_and(|f| f == "super") {
                v[0] = "crate".into();
            }
            (k, v)
        })
        .collect()
}

/// `impl X__VTable for S` 的 trait 路径（`X__VTable` 形态才算 Java 类型的 vtable trait）
pub(super) fn vtable_trait(i: &syn::ItemImpl) -> Option<Vec<String>> {
    let (_, path, _) = i.trait_.as_ref()?;
    let segs = path_segs(path);
    segs.last()?.strip_suffix(VTABLE_SUFFIX).filter(|s| !s.is_empty())?;
    Some(segs)
}

/// 同名 fn（多个 impl 块 / 文件）并入
fn merge_fn(fns: &mut HashMap<String, FnInfo>, calls: &mut HashMap<String, HashSet<String>>, name: String, raw: RawFn) {
    calls.entry(name.clone()).or_default().extend(raw.calls);
    let e = fns.entry(name).or_default();
    e.is_pub |= raw.info.is_pub;
    e.allocs.extend(raw.info.allocs);
    e.ctors.extend(raw.info.ctors);
    e.calls.extend(raw.info.calls);
    e.opaque.extend(raw.info.opaque);
    e.fields.extend(raw.info.fields);
    e.array_access |= raw.info.array_access;
    e.objects.extend(raw.info.objects);
}

pub(super) fn scan_file(file: &syn::File, prelude: &HashMap<String, Vec<String>>, out: &mut FileFns) {
    let mut us = UseScan(prelude.clone());
    us.visit_file(file);
    let local = super::objects::object_structs(file);
    let rets = local_rets(file, &us.0);
    out.rets.extend(rets.iter().map(|(k, v)| (k.clone(), v.clone())));
    let helpers = local_helpers(file);
    let generics = generic_fns(file);
    let mut fs = FileScan {
        rets: &rets,
        helpers: &helpers,
        generics: &generics,
        uses: &us.0,
        local_objects: &local,
        fns: Vec::new(),
        object_fns: Vec::new(),
        supers: Vec::new(),
        self_ty: None,
        cur_obj: None,
    };
    fs.visit_file(file);
    for (name, raw) in fs.fns {
        merge_fn(&mut out.fns, &mut out.calls, name, raw);
    }
    for (obj, name, raw) in fs.object_fns {
        let o = out.objects.entry(obj).or_default();
        merge_fn(&mut o.fns, &mut o.calls, name, raw);
    }
    for (obj, sup) in fs.supers {
        out.objects.entry(obj).or_default().supers.insert(sup);
    }
}

/// 分配 / 构造沿同文件 fn 调用传递（被调 fn 名须在同类手写文件内）
pub(super) fn close_transitive(fns: &mut HashMap<String, FnInfo>, calls: &HashMap<String, HashSet<String>>) {
    let names: Vec<String> = fns.keys().cloned().collect();
    let mut closed = Vec::new();
    for n in &names {
        let mut seen: HashSet<&str> = HashSet::from([n.as_str()]);
        let mut stack = vec![n.as_str()];
        let (mut allocs, mut ctors, mut objects) = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
        let (mut tcalls, mut opaque, mut fields) = (Vec::new(), HashSet::new(), Vec::new());
        let mut arr = false;
        while let Some(x) = stack.pop() {
            if let Some(f) = fns.get(x) {
                arr |= f.array_access;
                allocs.extend(f.allocs.iter().cloned());
                ctors.extend(f.ctors.iter().cloned());
                tcalls.extend(f.calls.iter().cloned());
                opaque.extend(f.opaque.iter().cloned());
                objects.extend(f.objects.iter().cloned());
                // 被调 fn 的 self 不一定是本方法的接收者
                let own = x == n.as_str();
                fields.extend(f.fields.iter().map(|fa| FieldAccess { on_self: fa.on_self && own, ..fa.clone() }));
            }
            for c in calls.get(x).into_iter().flatten() {
                if fns.contains_key(c) && seen.insert(c.as_str()) {
                    stack.push(c.as_str());
                }
            }
        }
        closed.push((n.clone(), allocs, ctors, tcalls, opaque, fields, arr, objects));
    }
    for (n, a, c, t, o, fl, arr, objs) in closed {
        let f = fns.get_mut(&n).expect("fn 名来自同一表");
        f.allocs = a;
        f.ctors = c;
        f.calls = t;
        f.opaque = o;
        f.fields = fl;
        f.array_access = arr;
        f.objects = objs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields_of(src: &str, f: &str) -> Vec<FieldAccess> {
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        out.fns.remove(f).map(|i| i.fields).unwrap_or_default()
    }

    fn named(p: &[&str]) -> SType {
        SType::Named(TypeRef(p.iter().map(|s| s.to_string()).collect()))
    }

    /// 只读视图与基本元素数组视图不改写引用元素；引用 / 未写明元素类型的视图上调用改写元素的方法、
    /// Object 上的引用元素存取、宏内同时出现视图与改写方法名的保守计入
    #[test]
    fn ref_array_access() {
        let src = r#"
            impl P {
                pub fn eq(&self, ob: Object) -> Result<bool> { Ok(to_u8(&self.__get_path()) == vec![]) }
                pub fn prim(&self, ob: Object) { let a = ob.try_cast_array::<i32>("[I"); let b = JArray::<u16>::new(1); b.set(0, 1); ob.array_store_byte(0, 1); }
                pub fn refs(&self, ob: Object) { let a = ob.try_cast_array::<Object>("[Ljava/lang/Object;"); let x = a.get(0); }
                pub fn refs_set(&self, ob: Object) { let a = ob.try_cast_array::<Object>("[Ljava/lang/Object;"); a.set(0, ob); }
                pub fn bare(&self) { let a = JArray::from_vec(vec![]); }
                pub fn store(&self, ob: Object) { ob.array_store_object(0, ob); }
                pub fn generic<T>(&self, a: &JArray<T>) -> usize { a.len() }
                pub fn generic_set<T>(&self, a: &JArray<T>, x: T) { a.set(0, x); }
                pub fn slice(&self, a: JArray<Class>) { a.with_vec(|v| v.reverse()); }
                pub fn nested(&self, a: &JArray<JArray<i8>>) {}
                pub fn mac(&self) { m!(JArray<i8>); }
                pub fn mac_set(&self, a: JArray<Object>) { m!(a.set(0, Object::default())); }
            }
            fn to_u8(a: &JArray<i8>) -> Vec<u8> { vec![] }
            fn put(a: &JArray<Object>) { a.set(0, Object::default()); }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let acc = |f: &str| out.fns.get(f).map(|i| i.array_access).expect("fn 存在");
        for f in ["eq", "prim", "to_u8", "refs", "bare", "generic", "nested", "mac"] {
            assert!(!acc(f), "{f} 不改写引用元素");
        }
        for f in ["refs_set", "store", "generic_set", "slice", "mac_set", "put"] {
            assert!(acc(f), "{f} 改写引用元素");
        }
    }

    #[test]
    fn field_access_receivers() {
        let src = r#"
            impl Decoder {
                pub fn open(&self, x: &Stream) {
                    self.__set_in_(Stream::new());
                    self.__get_fd().__set_fd(1);
                    let h = Holder::new(1).unwrap_or_else(|e| panic!());
                    h.__set_status(2);
                    { let h = other(); h.__set_status(3); }
                    h.__set_status(4);
                    for x in xs { x.__set_a(0); }
                    x.__set_b(0);
                    let v = x.__get_c();
                }
            }
        "#;
        let fs = fields_of(src, "open");
        let get = |i: usize| (fs[i].field.as_str(), fs[i].write, fs[i].recv.clone());
        assert_eq!(get(0), ("in", true, Some(named(&["Decoder"]))));
        assert_eq!(fs[0].value, Some(TypeRef(vec!["Stream".into()])));
        // 外层访问器先登记，再进接收者
        assert_eq!(get(1), ("fd", true, Some(SType::Field(Box::new(named(&["Decoder"])), "fd".into()))));
        assert_eq!(get(2), ("fd", false, Some(named(&["Decoder"]))));
        assert_eq!(get(3), ("status", true, Some(named(&["Holder"]))));
        assert_eq!(get(4), ("status", true, None));
        assert_eq!(get(5), ("status", true, Some(named(&["Holder"]))));
        // for 模式遮蔽形参 x；块外恢复
        assert_eq!(get(6), ("a", true, None));
        assert_eq!(get(7), ("b", true, Some(named(&["Stream"]))));
        assert_eq!(get(8), ("c", false, Some(named(&["Stream"]))));
    }

    #[test]
    fn static_field_setters() {
        let src = r#"
            impl System {
                pub fn registerNatives() {
                    let mut p = Properties::default();
                    p._init_not_null();
                    System::set_props(p);
                    Self::set_in_(x);
                    crate::java::lang::Runtime::set_current(1);
                    helper::set_mode(2);
                    System::set_pair(1, 2);
                }
            }
        "#;
        let fs = fields_of(src, "registerNatives");
        let w: Vec<(&str, bool, Option<SType>, bool)> = fs.iter().map(|f| (f.field.as_str(), f.write, f.recv.clone(), f.path)).collect();
        assert_eq!(
            w,
            vec![
                ("props", true, Some(named(&["System"])), true),
                ("in", true, Some(named(&["System"])), true),
                ("current", true, Some(named(&["crate", "java", "lang", "Runtime"])), true),
            ]
        );
        assert_eq!(fs[0].value, Some(TypeRef(vec!["Properties".into()])));
    }

    #[test]
    fn call_receivers() {
        let src = r#"
            impl Factory {
                pub fn open(options: Set<Object>) {
                    let it = options.iterator()?;
                    while it.hasNext()? { let o = it.next()?; }
                    Factory::make(1, 2);
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("open").map(|i| i.calls).unwrap_or_default();
        let site = |n: &str| cs.iter().find(|c| c.name == n).expect("调用点已登记");
        let set = named(&["Set"]);
        let iter = SType::Call(Box::new(set.clone()), "iterator".into());
        assert_eq!(site("iterator").srecv, Some(set));
        assert_eq!(site("hasNext").srecv, Some(iter.clone()));
        assert_eq!(site("next").srecv, Some(iter));
        let make = site("make");
        assert_eq!((make.path_ty.clone(), make.args.len(), make.srecv.clone()), (Some(TypeRef(vec!["Factory".into()])), 2, None));
    }

    /// 本文件辅助 fn 的返回值：`let df = Self::make(…)?` 的静态类型取 `make` 声明的返回类型
    #[test]
    fn local_helper_returns() {
        let src = r#"
            impl Provider {
                fn make(n: i32) -> Result<Format> { todo() }
                fn new_view(n: i32) -> Result<View> { todo() }
                pub fn get(&self) { let df = Self::make(0)?; df.setFlag(true)?; }
            }
            impl Provider__VTable for Provider {
                fn put(&self) { let v = Self::new_view(0)?; v.setMode(1)?; let w = Widget::new_i(1); w.show()?; }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("get").map(|i| i.calls).unwrap_or_default();
        let site = cs.iter().find(|c| c.name == "setFlag").expect("调用点已登记");
        assert_eq!(site.srecv, Some(named(&["Format"])));
        let cs = out.fns.remove("put").map(|i| i.calls).unwrap_or_default();
        let srecv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.srecv.clone());
        assert_eq!(srecv("setMode"), Some(named(&["View"])));
        assert_eq!(srecv("show"), Some(named(&["Widget"])));
        // 构造器名形态的本文件辅助 fn 不是构造：返回值动态类型推不出（不当成构造 Provider）
        let recv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.recv.clone());
        assert_eq!(recv("setMode"), Some(None));
        assert_eq!(recv("show"), Some(Some(TypeRef(vec!["Widget".into()]))));
    }

    #[test]
    fn container_elements() {
        let src = r#"
            impl Resources {
                fn chain(names: &[Name], m: HashMap<Key, Bundle>, d: Decoder) {
                    d.__get_in_().readAll()?;
                    let mut chain: Vec<Bundle> = Vec::new();
                    chain[0].setParent(Clone::clone(&chain[1]))?;
                    for n in names.iter() { n.intern()?; }
                    for b in &chain { b.close()?; }
                    let first = chain[0].clone();
                    first.reset()?;
                    for (k, v) in m.iter() { v.drop_all()?; }
                    let chain = 3; chain[0].stale()?;
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("chain").map(|i| i.calls).unwrap_or_default();
        let srecv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.srecv.clone());
        assert_eq!(srecv("setParent"), Some(named(&["Bundle"])));
        assert_eq!(srecv("intern"), Some(named(&["Name"])));
        assert_eq!(srecv("close"), Some(named(&["Bundle"])));
        assert_eq!(srecv("stale"), None);
        assert_eq!(srecv("readAll"), Some(SType::Field(Box::new(named(&["Decoder"])), "in".into())));
    }

    #[test]
    fn turbofish_casts() {
        let src = r#"
            impl Natives {
                pub fn resolve(type_: Object, e: Object) {
                    let Ok(mt) = Clone::clone(&type_).try_cast::<MethodType>("m") else { return; };
                    mt.toDesc()?;
                    if let Ok(c) = e.try_cast::<Class>("c") { c.getName()?; }
                    match e.try_cast::<Path>("p") { Ok(p) => { p.toUri()?; } Err(_) => {} }
                    let t: Throwable = e.catch_as::<Throwable>("t");
                    t.getCause()?;
                    e.try_cast::<Str>("s")?.length()?;
                    let name = format!("{}", mt.toMethodDescriptorString()?);
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("resolve").map(|i| i.calls).unwrap_or_default();
        let site = |n: &str| cs.iter().find(|c| c.name == n).expect("调用点已登记");
        let ty = |n: &str| Some(TypeRef(vec![n.to_string()]));
        assert_eq!(site("toDesc").recv, Some(ty("MethodType")));
        assert_eq!(site("toDesc").srecv, Some(named(&["MethodType"])));
        assert_eq!(site("getName").srecv, Some(named(&["Class"])));
        assert_eq!(site("toUri").srecv, Some(named(&["Path"])));
        assert_eq!(site("getCause").srecv, Some(named(&["Throwable"])));
        assert_eq!(site("length").recv, Some(ty("Str")));
        assert_eq!(site("toMethodDescriptorString").srecv, Some(named(&["MethodType"])));
    }
}

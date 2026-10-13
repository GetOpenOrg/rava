//! 手写实现对象（`impl X__VTable for S` 的本地 struct）的识别与方法体闭包。

use std::collections::{BTreeMap, HashMap, HashSet};

use super::scan::{vtable_trait, FileFns, RawObject};
use super::*;

/// 本文件里作为某个 Java vtable trait 实现体的 struct 名
pub(super) fn object_structs(file: &syn::File) -> HashSet<String> {
    let local: HashSet<String> = file
        .items
        .iter()
        .filter_map(|i| match i {
            syn::Item::Struct(s) => Some(s.ident.to_string()),
            _ => None,
        })
        .collect();
    file.items
        .iter()
        .filter_map(|i| match i {
            syn::Item::Impl(im) if vtable_trait(im).is_some() => stype::type_path(&im.self_ty),
            _ => None,
        })
        .filter_map(|t| (t.len() == 1 && local.contains(&t[0])).then(|| t[0].clone()))
        .collect()
}

/// 实现对象的各 fn 沿本对象的 fn 与本文件（已闭包的）fn 调用传递闭包。
/// 回调声明不传递（同 [`super::scan::close_transitive`]）；字段访问的 `self` 只对本 fn 成立。
/// 被调 fn 体内调用点的接收者归属（[`TypedCall::on_self`]）沿全程 `self.f(…)` 的调用链保持
/// （同 [`super::scan::close_transitive`]）
pub(super) fn close(raw: &FileFns) -> BTreeMap<String, HwObject> {
    raw.objects.iter().map(|(name, o)| (name.clone(), close_object(o, &raw.fns))).collect()
}

fn close_object(o: &RawObject, file_fns: &HashMap<String, FnInfo>) -> HwObject {
    let mut fns = HashMap::new();
    for (n, f) in &o.fns {
        let mut out = f.clone();
        let mut seen: HashSet<(&str, bool)> = HashSet::from([(n.as_str(), true)]);
        let mut stack = vec![(n.as_str(), true)];
        while let Some((x, keep)) = stack.pop() {
            let ns = o.nonself.get(x);
            for c in o.calls.get(x).into_iter().flatten() {
                // 链上的 self 仍是本 fn 的 self ⇔ 本边以 `self.c(…)` 调用（经非 `self` 接收者到达则取消）
                let self_recv = keep && !ns.is_some_and(|s| s.contains(c));
                if !seen.insert((c.as_str(), self_recv)) {
                    continue;
                }
                // 对象自身的 fn 优先（`Self::helper`），否则取同文件 fn（已闭包，不再展开）；
                // 入口 fn 自身重达时不重复并入（其效果即 `out` 的基础，同 [`close_transitive`] 对本 fn 的口径）
                if let Some(g) = o.fns.get(c) {
                    if c.as_str() != n.as_str() {
                        absorb_body(&mut out, g, self_recv);
                    }
                    stack.push((c.as_str(), self_recv));
                } else if let Some(g) = file_fns.get(c) {
                    absorb_body(&mut out, g, self_recv);
                }
            }
        }
        fns.insert(n.clone(), out);
    }
    HwObject { supers: o.supers.clone(), fns }
}

/// 被调 fn 的手写体效果并入；`self_recv`：被调 fn 是沿全程 `self.f(…)` 的调用链到达（其 `self`
/// 即本方法的接收者，体内调用点的 on_self 保持），否则被调 fn 的 self 不一定是本方法的接收者
fn absorb_body(out: &mut FnInfo, g: &FnInfo, self_recv: bool) {
    out.allocs.extend(g.allocs.iter().cloned());
    out.ctors.extend(g.ctors.iter().cloned());
    out.calls.extend(g.calls.iter().map(|c| TypedCall { on_self: c.on_self && self_recv, ..c.clone() }));
    out.opaque.extend(g.opaque.iter().cloned());
    out.fields.extend(g.fields.iter().map(|fa| FieldAccess { on_self: false, value_self: false, value_src_param: None, recv_src_param: None, ..fa.clone() }));
    out.array_access |= g.array_access;
    out.objects.extend(g.objects.iter().cloned());
}

#[cfg(test)]
mod tests {
    use super::super::scan::{close_transitive, scan_file};
    use super::*;

    fn tr(p: &[&str]) -> TypeRef {
        TypeRef(p.iter().map(|s| s.to_string()).collect())
    }

    /// 对象 fn 经 `self.f(…)` 调用的本对象辅助 fn：其体内 `self.X()` 上调仍是本方法的接收者
    /// （[`TypedCall::on_self`]，`hw-upcall-self-lost-in-helper`）；经非 `self` 接收者到达的链取消
    #[test]
    fn object_helper_upcall_on_self() {
        let src = r#"
            struct Adapter;
            impl Foo__VTable for Adapter {
                fn bar(&self, o: &Adapter) -> i32 { self.a() + o.b() }
                fn a(&self) -> i32 { self.inner() }
                fn b(&self) -> i32 { self.inner() }
            }
            impl Adapter {
                fn inner(&self) -> i32 { self.upcall() }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut raw = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut raw);
        close_transitive(&mut raw.fns, &raw.calls, &raw.nonself);
        let objs = close(&raw);
        let on = |f: &str| {
            let mut v: Vec<bool> = objs["Adapter"].fns[f].calls.iter().filter(|c| c.name == "upcall").map(|c| c.on_self).collect();
            v.sort();
            v
        };
        // bar 经 self.a() 到达 inner：保持归属；经 o.b() 到达（他对象接收者）：取消——两个变体并存
        assert_eq!(on("bar"), vec![false, true]);
        // inner 自身体内：本 fn 的 self
        assert_eq!(on("inner"), vec![true]);
    }

    #[test]
    fn vtable_impl_structs_become_objects() {
        let src = r#"
            use crate::java::util::stream::stream::Stream;
            use crate::sun::util::locale::provider::locale_provider_adapter::LocaleProviderAdapter__VTable;
            use super::shared_impl::Other;
            struct Adapter;
            struct Plain;
            impl LocaleProviderAdapter__VTable for Adapter {
                fn getProvider(&self) -> Object { helper(); Stream::new_empty() }
                fn name(&self) -> i32 { self.inner() }
            }
            impl Adapter {
                fn inner(&self) -> i32 { let _ = Rc::new(Provider); 0 }
            }
            impl Plain { fn f(&self) {} }
            fn helper() { let _ = Other; }
            impl LocaleProviderAdapter {
                pub fn getAdapter() -> Object { Object::from(Rc::new(Adapter)) }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        assert_eq!(object_structs(&file), HashSet::from(["Adapter".to_string()]));
        let mut raw = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut raw);
        close_transitive(&mut raw.fns, &raw.calls, &raw.nonself);
        let objs = close(&raw);
        let a = &objs["Adapter"];
        assert_eq!(
            a.supers,
            BTreeSet::from([tr(&["crate", "sun", "util", "locale", "provider", "locale_provider_adapter", "LocaleProviderAdapter"])])
        );
        // 对象 fn 不混入文件 fn；对象内与文件 fn 的调用传递闭包
        assert!(!raw.fns.contains_key("getProvider") && !raw.fns.contains_key("inner"));
        let names: BTreeSet<&str> = a.fns.keys().map(String::as_str).collect();
        assert_eq!(names, BTreeSet::from(["getProvider", "inner", "name"]));
        assert!(a.fns["getProvider"].objects.contains(&tr(&["super", "shared_impl", "Other"])));
        // 手写边界方法引用本地实现对象
        assert!(raw.fns["getAdapter"].objects.contains(&tr(&["Adapter"])));
    }
}

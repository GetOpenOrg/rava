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
/// 回调声明不传递（同 [`super::scan::close_transitive`]）；字段访问的 `self` 只对本 fn 成立
pub(super) fn close(raw: &FileFns) -> BTreeMap<String, HwObject> {
    raw.objects.iter().map(|(name, o)| (name.clone(), close_object(o, &raw.fns))).collect()
}

fn close_object(o: &RawObject, file_fns: &HashMap<String, FnInfo>) -> HwObject {
    let mut fns = HashMap::new();
    for (n, f) in &o.fns {
        let mut out = f.clone();
        let mut seen: HashSet<&str> = HashSet::from([n.as_str()]);
        let mut stack = vec![n.as_str()];
        while let Some(x) = stack.pop() {
            for c in o.calls.get(x).into_iter().flatten() {
                if !seen.insert(c.as_str()) {
                    continue;
                }
                // 对象自身的 fn 优先（`Self::helper`），否则取同文件 fn（已闭包，不再展开）
                if let Some(g) = o.fns.get(c) {
                    absorb_body(&mut out, g);
                    stack.push(c.as_str());
                } else if let Some(g) = file_fns.get(c) {
                    absorb_body(&mut out, g);
                }
            }
        }
        fns.insert(n.clone(), out);
    }
    HwObject { supers: o.supers.clone(), fns }
}

/// 被调 fn 的手写体效果并入（被调 fn 的 self 不一定是本方法的接收者）
fn absorb_body(out: &mut FnInfo, g: &FnInfo) {
    out.allocs.extend(g.allocs.iter().cloned());
    out.ctors.extend(g.ctors.iter().cloned());
    out.calls.extend(g.calls.iter().map(|c| TypedCall { on_self: false, ..c.clone() }));
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

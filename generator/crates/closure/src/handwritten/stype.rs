//! 手写体表达式的静态类型（[`SType`]）：类型注解路径、容器元素类型、访问器 / 转型 / 方法链推导。

use std::collections::{HashMap, HashSet};

use super::syntax::*;
use super::*;

/// 类型注解的路径（剥引用 / 括号；`Self` 原样保留，由解析时换成宿主类）
pub(super) fn type_path(t: &syn::Type) -> Option<Vec<String>> {
    match t {
        syn::Type::Reference(r) => type_path(&r.elem),
        syn::Type::Paren(p) => type_path(&p.elem),
        syn::Type::Group(g) => type_path(&g.elem),
        syn::Type::Path(p) if p.qself.is_none() => Some(path_segs(&p.path)),
        _ => None,
    }
}

/// 访问器名中的字段段 → Java 字段名：Java 字段名是 Rust 关键字时访问器带 `_` 后缀（`in` → `__get_in_`）
pub(super) fn java_field_name(f: &str) -> &str {
    f.strip_suffix('_').filter(|k| RUST_KEYWORDS.contains(k)).unwrap_or(f)
}

/// 容器类型的元素类型：`Vec<T>` / `HashMap<K, V>` 等取末个泛型实参，`[T]` / `[T; N]` 取元素（索引与 for 迭代的结果类型）
pub(super) fn elem_type(t: &syn::Type) -> Option<Vec<String>> {
    match t {
        syn::Type::Reference(r) => elem_type(&r.elem),
        syn::Type::Paren(p) => elem_type(&p.elem),
        syn::Type::Group(g) => elem_type(&g.elem),
        syn::Type::Slice(s) => type_path(&s.elem),
        syn::Type::Array(a) => type_path(&a.elem),
        syn::Type::Path(p) if p.qself.is_none() => {
            let syn::PathArguments::AngleBracketed(a) = &p.path.segments.last()?.arguments else { return None };
            a.args.iter().rev().find_map(|g| match g {
                syn::GenericArgument::Type(t) => Some(type_path(t)),
                _ => None,
            })?
        }
        _ => None,
    }
}

/// 作用域中容器变量的元素类型键：`<变量名>[]`（标识符不含 `[]`，与变量名不冲突）
pub(super) fn elem_key(var: &str) -> String {
    format!("{var}[]")
}

/// 容器表达式（`v` / `&v` / `v.iter()` / `v.into_iter()` / `v.iter_mut()`）的元素静态类型
pub(super) fn elem_stype(e: &syn::Expr, statics: &HashMap<String, Option<SType>>) -> Option<SType> {
    use syn::Expr;
    match e {
        Expr::Paren(p) => elem_stype(&p.expr, statics),
        Expr::Group(g) => elem_stype(&g.expr, statics),
        Expr::Reference(r) => elem_stype(&r.expr, statics),
        Expr::Path(p) => p.path.get_ident().and_then(|i| statics.get(&elem_key(&i.to_string())).cloned().flatten()),
        Expr::MethodCall(m) if m.args.is_empty() && matches!(m.method.to_string().as_str(), "iter" | "into_iter" | "iter_mut") => {
            elem_stype(&m.receiver, statics)
        }
        _ => None,
    }
}

/// 表达式的静态类型：形参 / let 注解、`T::default()` / `T::new*`、`T::m(…)` 的返回、`x.__get_f()` 的字段；
/// 其余退回动态类型推断
pub(super) fn stype(e: &syn::Expr, statics: &HashMap<String, Option<SType>>, locals: &HashMap<String, Option<Vec<String>>>) -> Option<SType> {
    use syn::Expr;
    let direct = match e {
        Expr::Paren(p) => stype(&p.expr, statics, locals),
        Expr::Group(g) => stype(&g.expr, statics, locals),
        Expr::Reference(r) => stype(&r.expr, statics, locals),
        Expr::Try(t) => stype(&t.expr, statics, locals),
        Expr::Path(p) => p.path.get_ident().and_then(|i| statics.get(&i.to_string()).cloned().flatten()),
        Expr::Index(ix) => elem_stype(&ix.expr, statics),
        Expr::MethodCall(m) if cast_target(m).is_some() => cast_target(m).map(|t| SType::Named(TypeRef(t))),
        Expr::MethodCall(m) => {
            let name = m.method.to_string();
            match name.strip_prefix(GET_PREFIX) {
                Some(f) if m.args.is_empty() => {
                    stype(&m.receiver, statics, locals).map(|r| SType::Field(Box::new(r), java_field_name(f).to_string()))
                }
                _ if matches!(name.as_str(), "clone" | "unwrap" | "expect" | "unwrap_or_else" | "unwrap_or") => {
                    stype(&m.receiver, statics, locals)
                }
                _ => stype(&m.receiver, statics, locals).map(|r| SType::Call(Box::new(r), name)),
            }
        }
        Expr::Call(c) => match &*c.func {
            Expr::Path(p) => {
                let segs = expr_path_segs(p);
                match segs.split_last() {
                    Some((last, head)) if head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase())) => {
                        let t = TypeRef(head.to_vec());
                        // `T::from(x)`：引用类型间转换即 checkcast，静态类型为 T。构造器形态的 `T::new*` 亦记为
                        // `Ret`：本文件同名辅助 fn（`Self::new_format`）以声明的返回类型为准，否则取 T（见 scan::local_ret）
                        Some(if last == "default" || last == "from" {
                            SType::Named(t)
                        } else {
                            SType::Ret(t, last.clone())
                        })
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    };
    direct.or_else(|| infer(e, locals, &HashSet::new()).map(|t| SType::Named(TypeRef(t))))
}

/// 构造器的 Rust 名形态：`new` / `new_<签名>`
pub(super) fn is_ctor_name(name: &str) -> bool {
    name == CTOR_RUST || name.starts_with("new_")
}

/// 路径调用 `T::f(…)` 是 Java 构造器：构造器名形态，且不是本文件 impl 块声明的同名辅助 fn
///（`helpers` 为 `<写出的类型路径>::<fn 名>`，见 [`local_helpers`]；`Self::new_format` 是辅助 fn，
/// 返回值的动态类型不是 `Self`）
pub(super) fn is_ctor_call(head: &[String], last: &str, helpers: &HashSet<String>) -> bool {
    is_ctor_name(last) && !helpers.contains(&format!("{}::{last}", head.join("::")))
}

/// 本文件 impl 块中构造器名形态的辅助 fn：`<impl 类型末段>::<fn 名>`
pub(super) fn local_helpers(file: &syn::File) -> HashSet<String> {
    let mut out = HashSet::new();
    for item in &file.items {
        let syn::Item::Impl(i) = item else { continue };
        let Some(last) = type_path(&i.self_ty).and_then(|t| t.last().cloned()) else { continue };
        for it in &i.items {
            if let syn::ImplItem::Fn(f) = it {
                let n = f.sig.ident.to_string();
                if is_ctor_name(&n) {
                    out.insert(format!("{last}::{n}"));
                }
            }
        }
    }
    out
}

/// 某 impl 块内的辅助 fn 写法：文件级登记，加上 impl 类型自身的辅助 fn 的 `Self::<fn 名>` 写法
pub(super) fn helpers_in(file: &HashSet<String>, self_ty: Option<&Vec<String>>) -> HashSet<String> {
    let mut out = file.clone();
    if let Some(prefix) = self_ty.and_then(|t| t.last()).map(|l| format!("{l}::")) {
        out.extend(file.iter().filter_map(|h| h.strip_prefix(&prefix)).map(|n| format!("Self::{n}")));
    }
    out
}

pub(super) fn expand_s(uses: &HashMap<String, Vec<String>>, s: SType, self_ty: &Option<Vec<String>>) -> SType {
    let tr = |t: TypeRef| match (t.0.as_slice(), self_ty) {
        ([one], Some(st)) if one == "Self" => TypeRef(expand(uses, st.clone())),
        _ => TypeRef(expand(uses, t.0)),
    };
    match s {
        SType::Named(t) => SType::Named(tr(t)),
        SType::Ret(t, m) => SType::Ret(tr(t), m),
        SType::Field(b, f) => SType::Field(Box::new(expand_s(uses, *b, self_ty)), f),
        SType::Call(b, m) => SType::Call(Box::new(expand_s(uses, *b, self_ty)), m),
    }
}

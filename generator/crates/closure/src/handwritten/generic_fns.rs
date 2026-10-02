//! 本文件辅助 fn（含泛型）的闭包形参类型：调用点按实参解出类型形参，再代入闭包形参。
//!
//! `fn each<T>(arr: &JArray<T>, f: impl FnMut(T) -> R)` 在 `each(&<JArray<Object> as From<Object>>::from(v), |x| x.toString())`
//! 处解出 `T = Object`，闭包形参 `x` 的静态类型即 `Object`。闭包形参类型来自 `impl Fn*(…)` 形参，或以
//! `Fn*(…)` 为约束（泛型列表 / where 子句）的类型形参；类型形参按实参的整体类型（`T` / `&T`）或元素类型
//! （`X<…, T>`，与容器元素口径相同，见 [`elem_type`]）解出。

use std::collections::{HashMap, HashSet};

use super::stype::*;
use super::*;

/// 泛型辅助 fn 签名
#[derive(Default)]
pub(super) struct GenericSig {
    /// 形参类型（不含 self）
    params: Vec<syn::Type>,
    /// 类型形参名
    tparams: HashSet<String>,
    /// 以 `Fn*(…)` 为约束的类型形参 → 闭包形参类型
    fn_bounds: HashMap<String, Vec<syn::Type>>,
}

/// 本文件的泛型辅助 fn：键为写出的调用路径（自由 fn `f`；impl 关联 fn `<impl 类型末段>::f`）
pub(super) type GenericFns = HashMap<String, GenericSig>;

/// `Fn(A…)` / `FnMut(A…)` / `FnOnce(A…)` 约束的实参类型
fn fn_args(b: &syn::TypeParamBound) -> Option<Vec<syn::Type>> {
    let syn::TypeParamBound::Trait(t) = b else { return None };
    let seg = t.path.segments.last()?;
    let syn::PathArguments::Parenthesized(p) = &seg.arguments else { return None };
    matches!(seg.ident.to_string().as_str(), "Fn" | "FnMut" | "FnOnce").then(|| p.inputs.iter().cloned().collect())
}

fn sig_of(sig: &syn::Signature) -> Option<GenericSig> {
    let mut g = GenericSig::default();
    for p in sig.generics.type_params() {
        let name = p.ident.to_string();
        if let Some(a) = p.bounds.iter().find_map(fn_args) {
            g.fn_bounds.insert(name.clone(), a);
        }
        g.tparams.insert(name);
    }
    for w in sig.generics.where_clause.iter().flat_map(|w| &w.predicates) {
        let syn::WherePredicate::Type(pt) = w else { continue };
        if let (Some([name]), Some(a)) = (type_path(&pt.bounded_ty).as_deref(), pt.bounds.iter().find_map(fn_args)) {
            g.fn_bounds.insert(name.clone(), a);
        }
    }
    g.params = sig
        .inputs
        .iter()
        .filter_map(|a| match a {
            syn::FnArg::Typed(pt) => Some((*pt.ty).clone()),
            syn::FnArg::Receiver(_) => None,
        })
        .collect();
    g.params.iter().any(|t| g.closure_params(t).is_some()).then_some(g)
}

/// 本文件顶层自由 fn 与 impl 块关联 fn 中带闭包形参的 fn
pub(super) fn generic_fns(file: &syn::File) -> GenericFns {
    let mut out = HashMap::new();
    for item in &file.items {
        match item {
            syn::Item::Fn(f) => {
                if let Some(g) = sig_of(&f.sig) {
                    out.insert(f.sig.ident.to_string(), g);
                }
            }
            syn::Item::Impl(i) => {
                let Some(last) = type_path(&i.self_ty).and_then(|t| t.last().cloned()) else { continue };
                for it in &i.items {
                    if let syn::ImplItem::Fn(f) = it {
                        if let Some(g) = sig_of(&f.sig) {
                            out.insert(format!("{last}::{}", f.sig.ident), g);
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// 类型形参在声明类型中的位置
enum Slot {
    /// `T` / `&T`：实参整体
    Whole,
    /// `X<…, T>`：实参的元素
    Elem,
}

fn strip_ref(t: &syn::Type) -> &syn::Type {
    match t {
        syn::Type::Reference(r) => strip_ref(&r.elem),
        syn::Type::Paren(p) => strip_ref(&p.elem),
        syn::Type::Group(g) => strip_ref(&g.elem),
        other => other,
    }
}

/// 末个泛型实参（容器元素口径）
fn last_type_arg(args: &syn::PathArguments) -> Option<&syn::Type> {
    let syn::PathArguments::AngleBracketed(a) = args else { return None };
    a.args.iter().rev().find_map(|g| match g {
        syn::GenericArgument::Type(t) => Some(t),
        _ => None,
    })
}

fn slot_of(decl: &syn::Type, t: &str) -> Option<Slot> {
    let syn::Type::Path(p) = strip_ref(decl) else { return None };
    if p.qself.is_none() && p.path.is_ident(t) {
        return Some(Slot::Whole);
    }
    let el = last_type_arg(&p.path.segments.last()?.arguments)?;
    matches!(strip_ref(el), syn::Type::Path(e) if e.qself.is_none() && e.path.is_ident(t)).then_some(Slot::Elem)
}

/// 实参表达式写明的元素类型：`<X<E> as Tr>::f(…)` / `X::<E>::f(…)` 的 `E`（可带 `&` / `?` / 括号）
fn written_elem(e: &syn::Expr) -> Option<Vec<String>> {
    match e {
        syn::Expr::Paren(p) => written_elem(&p.expr),
        syn::Expr::Group(g) => written_elem(&g.expr),
        syn::Expr::Reference(r) => written_elem(&r.expr),
        syn::Expr::Try(t) => written_elem(&t.expr),
        syn::Expr::Call(c) => {
            let syn::Expr::Path(p) = &*c.func else { return None };
            if let Some(q) = &p.qself {
                return elem_type(&q.ty);
            }
            let n = p.path.segments.len();
            let head = p.path.segments.iter().nth(n.checked_sub(2)?)?;
            last_type_arg(&head.arguments).and_then(type_path)
        }
        _ => None,
    }
}

impl GenericSig {
    /// 形参类型若是闭包（`impl Fn*(…)` 或 `Fn*` 约束的类型形参），返回闭包形参类型
    fn closure_params(&self, t: &syn::Type) -> Option<Vec<syn::Type>> {
        match strip_ref(t) {
            syn::Type::ImplTrait(it) => it.bounds.iter().find_map(fn_args),
            syn::Type::Path(p) if p.qself.is_none() => p.path.get_ident().and_then(|i| self.fn_bounds.get(&i.to_string()).cloned()),
            _ => None,
        }
    }

    /// 类型形参 `t` 按其余实参解出的静态类型
    fn solve(&self, t: &str, args: &[&syn::Expr], scope: &HashMap<String, Option<SType>>, locals: &HashMap<String, Option<Vec<String>>>) -> Option<SType> {
        self.params.iter().zip(args).find_map(|(decl, a)| match slot_of(decl, t)? {
            Slot::Whole => stype(a, scope, locals),
            Slot::Elem => elem_stype(a, scope).or_else(|| written_elem(a).map(|p| SType::Named(TypeRef(p)))),
        })
    }

    /// 第 `i` 个实参若是闭包，其各形参的静态类型（具体类型取注解路径，类型形参按实参解出）
    pub(super) fn closure_arg_types(
        &self,
        i: usize,
        args: &[&syn::Expr],
        scope: &HashMap<String, Option<SType>>,
        locals: &HashMap<String, Option<Vec<String>>>,
    ) -> Option<Vec<Option<SType>>> {
        let ps = self.closure_params(self.params.get(i)?)?;
        Some(
            ps.iter()
                .map(|p| match type_path(p).as_deref() {
                    Some([n]) if self.tparams.contains(n) => self.solve(n, args, scope, locals),
                    Some(path) => Some(SType::Named(TypeRef(path.to_vec()))),
                    None => None,
                })
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::scan::{scan_file, FileFns};
    use super::*;

    /// 闭包形参按调用点解出：`impl FnMut(T)` / 约束 `F: Fn(T)` / where 子句；元素口径（`&JArray<T>`）与
    /// 整体口径（`T`）；`Self::f` 关联 fn；注解类型优先；非泛型闭包形参仍无类型
    #[test]
    fn generic_helper_closure_params() {
        let src = r#"
            fn each<T: Clone>(arr: &JArray<T>, mut f: impl FnMut(T) -> Result<Str>) -> Result<Vec<Str>> { todo() }
            fn with<T, F: Fn(&T, i32)>(v: T, f: F) {}
            fn after<F, T>(f: F, v: &T) where F: FnOnce(T) {}
            fn plain(f: impl Fn(Path)) {}
            impl H {
                fn walk<E>(xs: Vec<E>, f: impl Fn(E)) {}
                pub fn run(value: Object, k: Klass, ns: JArray<Node>) -> Result<()> {
                    each(&<JArray<Object> as From<Object>>::from(value), |x| Ok(x.toString()?))?;
                    each(&ns, |n| n.render())?;
                    with(k, |a, i| a.describe());
                    after(|b| b.flush(), &k);
                    plain(|p| p.toUri());
                    Self::walk(JArray::<Site>::from(value), |s| s.visit());
                    each(&ns, |q: Other| q.open())?;
                    let u = |y| y.isDir();
                    Ok(())
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("run").map(|i| i.calls).unwrap_or_default();
        let srecv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.srecv.clone());
        let named = |p: &str| Some(SType::Named(TypeRef(vec![p.to_string()])));
        assert_eq!(srecv("toString"), named("Object"));
        assert_eq!(srecv("render"), named("Node"));
        assert_eq!(srecv("describe"), named("Klass"));
        assert_eq!(srecv("flush"), named("Klass"));
        assert_eq!(srecv("toUri"), named("Path"));
        assert_eq!(srecv("visit"), named("Site"));
        assert_eq!(srecv("open"), named("Other"));
        assert_eq!(srecv("isDir"), None);
    }
}

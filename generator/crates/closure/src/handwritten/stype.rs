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

/// match 分支 `Ok(x) => x` / `Some(x) => x`：解包被匹配的 `Result` / `Option`
fn unwrap_arm(a: &syn::Arm) -> bool {
    let syn::Pat::TupleStruct(ts) = &a.pat else { return false };
    let wrapper = ts.path.get_ident().is_some_and(|i| i == "Ok" || i == "Some");
    let bound = match ts.elems.first() {
        Some(syn::Pat::Ident(pi)) if ts.elems.len() == 1 => &pi.ident,
        _ => return false,
    };
    let body = match &*a.body {
        syn::Expr::Block(b) if b.block.stmts.len() == 1 => match &b.block.stmts[0] {
            syn::Stmt::Expr(e, None) => e,
            _ => return false,
        },
        e => e,
    };
    wrapper && matches!(body, syn::Expr::Path(p) if p.path.is_ident(bound))
}

/// 表达式的静态类型：形参 / let 注解、`T::default()` / `T::new*`、`T::m(…)` 的返回、`x.__get_f()` 的字段；
/// 其余退回动态类型推断
pub(super) fn stype(e: &syn::Expr, statics: &HashMap<String, Option<SType>>, locals: &HashMap<String, Option<Vec<String>>>) -> Option<SType> {
    use syn::Expr;
    let direct = match e {
        Expr::Paren(p) => stype(&p.expr, statics, locals),
        Expr::Group(g) => stype(&g.expr, statics, locals),
        Expr::Reference(r) => stype(&r.expr, statics, locals),
        // `x.ok()?`：`?` 取回 Result 内的值
        Expr::Try(t) => match &*t.expr {
            Expr::MethodCall(m) if m.method == "ok" && m.args.is_empty() => stype(&m.receiver, statics, locals),
            inner => stype(inner, statics, locals),
        },
        Expr::Path(p) => p.path.get_ident().and_then(|i| statics.get(&i.to_string()).cloned().flatten()),
        Expr::Index(ix) => elem_stype(&ix.expr, statics),
        // `match r { Ok(x) => x, Err(e) => … }`：解包分支原样产出被匹配值内的 Java 值
        Expr::Match(mt) if mt.arms.iter().any(unwrap_arm) => stype(&mt.expr, statics, locals),
        Expr::MethodCall(m) if cast_target(m).is_some() => cast_target(m).map(|t| SType::Named(TypeRef(t))),
        Expr::MethodCall(m) => {
            let name = m.method.to_string();
            match name.strip_prefix(GET_PREFIX) {
                Some(f) if m.args.is_empty() => {
                    stype(&m.receiver, statics, locals).map(|r| SType::Field(Box::new(r), java_field_name(f).to_string()))
                }
                _ if matches!(name.as_str(), "clone" | "unwrap" | "expect" | "unwrap_or_else" | "unwrap_or" | "unwrap_or_default") => {
                    stype(&m.receiver, statics, locals)
                }
                // 带元素类型注解的容器（`ptypes: JArray<Class>`）的元素访问器 `get(i)`
                _ if name == "get" && m.args.len() == 1 && elem_stype(&m.receiver, statics).is_some() => elem_stype(&m.receiver, statics),
                // 未经 `?` 的 `x.ok()` 是 Rust `Option` 包装，不是 Java 值
                _ if name == "ok" && m.args.is_empty() => return None,
                _ => stype(&m.receiver, statics, locals).map(|r| SType::Call(Box::new(r), name)),
            }
        }
        Expr::Call(c) => match &*c.func {
            Expr::Path(p) => {
                let segs = expr_path_segs(p);
                match segs.split_last() {
                    // `Clone::clone(&x)`：引用克隆，静态类型同 x
                    Some((last, head)) if last == "clone" && head == ["Clone"] && c.args.len() == 1 => stype(&c.args[0], statics, locals),
                    // 本文件自由 fn `f(…)`：返回类型由 scan::local_ret 按声明换上
                    Some((last, [])) if last.starts_with(|ch: char| ch.is_ascii_lowercase() || ch == '_') => Some(SType::Ret(TypeRef(Vec::new()), last.clone())),
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
                    // 模块路径上的自由 fn `super::m::f(…)` / `crate::a::m::f(…)`：返回类型由引擎按目标文件的声明解析
                    //（[`Handwritten::module_fn_ret`]）；动态类型推断得出的优先
                    Some((last, head)) if is_module_path(head) && last.starts_with(|ch: char| ch.is_ascii_lowercase() || ch == '_') => {
                        infer(e, locals, &HashSet::new()).map(|t| SType::Named(TypeRef(t))).or_else(|| Some(SType::Ret(TypeRef(head.to_vec()), last.clone())))
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
        SType::Java(c) => SType::Java(c),
    }
}

/// (impl self 类型全路径, fn 名) → 返回类型全路径（非类型路径的返回为空）
pub type LocalRets = HashMap<(Vec<String>, String), Vec<String>>;

/// 本文件顶层 impl 块关联 fn 与自由 fn 的返回类型（剥 `Result` / `Option`；`Self` 换成 impl 类型）：
/// 手写辅助 fn（`Self::new_format(…)`）返回值的静态类型
pub(super) fn local_rets(file: &syn::File, uses: &HashMap<String, Vec<String>>) -> LocalRets {
    let mut out = HashMap::new();
    for item in &file.items {
        // 文件顶层自由 fn：键的类型路径为空
        if let syn::Item::Fn(f) = item {
            if let syn::ReturnType::Type(_, t) = &f.sig.output {
                if let Some(r) = ret_path(t) {
                    out.insert((Vec::new(), f.sig.ident.to_string()), expand(uses, r));
                }
            }
            continue;
        }
        let syn::Item::Impl(i) = item else { continue };
        let Some(st) = type_path(&i.self_ty).map(|t| expand(uses, t)) else { continue };
        for it in &i.items {
            let syn::ImplItem::Fn(f) = it else { continue };
            // 返回类型不是类型路径（无返回、引用、元组…）记为空路径：解析不到 Java 类
            let r = match &f.sig.output {
                syn::ReturnType::Type(_, t) => ret_path(t).unwrap_or_default(),
                syn::ReturnType::Default => Vec::new(),
            };
            let r = if r == ["Self"] { st.clone() } else if r.is_empty() { r } else { expand(uses, r) };
            out.insert((st.clone(), f.sig.ident.to_string()), r);
        }
    }
    out
}

/// 模块路径（`super::m` / `crate::a::m` / `self::m`）：首段是路径关键字、末段是小写模块名
pub(super) fn is_module_path(p: &[String]) -> bool {
    matches!(p.first().map(String::as_str), Some("super" | "crate" | "self"))
        && p.last().is_some_and(|l| l.starts_with(|ch: char| ch.is_ascii_lowercase() || ch == '_'))
}

/// Rust 标量（基本类型与 `str`）：不承载 Java 引用的形参类型
const RUST_SCALARS: &[&str] = &["i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize", "f32", "f64", "bool", "char", "str"];

/// 形参类型是 Rust 标量（可经引用 / 切片 / 数组）：值里不可能有 Java 引用
fn is_scalar_type(t: &syn::Type) -> bool {
    match t {
        syn::Type::Reference(r) => is_scalar_type(&r.elem),
        syn::Type::Slice(s) => is_scalar_type(&s.elem),
        syn::Type::Array(a) => is_scalar_type(&a.elem),
        syn::Type::Paren(p) => is_scalar_type(&p.elem),
        syn::Type::Path(p) => p.qself.is_none() && p.path.get_ident().is_some_and(|i| RUST_SCALARS.contains(&i.to_string().as_str())),
        _ => false,
    }
}

/// 本文件只收标量形参的辅助 fn（顶层自由 fn 与 impl 块里无 `self` 的关联 fn；键同 [`LocalRets`]）：
/// 其返回值不可能是任何实参，只能来自 fn 体的产出（分配、回调结果、静态读取）
pub(super) fn scalar_arg_fns(file: &syn::File, uses: &HashMap<String, Vec<String>>) -> HashSet<(Vec<String>, String)> {
    let scalar = |sig: &syn::Signature| sig.inputs.iter().all(|a| matches!(a, syn::FnArg::Typed(pt) if is_scalar_type(&pt.ty)));
    let mut out = HashSet::new();
    for item in &file.items {
        match item {
            syn::Item::Fn(f) if scalar(&f.sig) => {
                out.insert((Vec::new(), f.sig.ident.to_string()));
            }
            syn::Item::Impl(i) => {
                let Some(st) = type_path(&i.self_ty).map(|t| expand(uses, t)) else { continue };
                for it in &i.items {
                    if let syn::ImplItem::Fn(f) = it {
                        if scalar(&f.sig) {
                            out.insert((st.clone(), f.sig.ident.to_string()));
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}

/// 返回类型路径：`Result<T>` / `Option<T>` 取 `T`
fn ret_path(t: &syn::Type) -> Option<Vec<String>> {
    if let syn::Type::Path(p) = t {
        let last = p.path.segments.last()?;
        if matches!(last.ident.to_string().as_str(), "Result" | "Option") {
            let syn::PathArguments::AngleBracketed(a) = &last.arguments else { return None };
            let Some(syn::GenericArgument::Type(inner)) = a.args.first() else { return None };
            return ret_path(inner);
        }
    }
    type_path(t)
}

/// 静态类型中本文件辅助 fn 的返回（`SType::Ret`）换成其声明的返回类型；非本文件声明的构造器形态
/// `T::new*` 取 `T`；非本文件的自由 fn（类型路径为空）推不出 → None
pub(super) fn local_ret(s: SType, rets: &LocalRets) -> Option<SType> {
    Some(match s {
        SType::Ret(t, m) => match rets.get(&(t.0.clone(), m.clone())).filter(|r| !r.is_empty()) {
            Some(r) => SType::Named(TypeRef(r.clone())),
            None if t.0.is_empty() => return None,
            None if is_module_path(&t.0) => SType::Ret(t, m),
            None if is_ctor_name(&m) => SType::Named(t),
            None => SType::Ret(t, m),
        },
        SType::Field(b, f) => SType::Field(Box::new(local_ret(*b, rets)?), f),
        SType::Call(b, m) => SType::Call(Box::new(local_ret(*b, rets)?), m),
        n => n,
    })
}

#[cfg(test)]
mod tests {
    use super::super::scan::{scan_file, FileFns};
    use super::*;

    fn named(p: &[&str]) -> SType {
        SType::Named(TypeRef(p.iter().map(|s| s.to_string()).collect()))
    }

    /// `x.ok()?` 取回 Java 值；未经 `?` 的 `x.ok()` 是 Rust `Option`，无 Java 静态类型。
    /// 非类型路径返回的辅助 fn 也登记（空路径）：审计据此判定其后的调用是 Rust 语义
    #[test]
    fn result_adapters_and_opaque_returns() {
        let src = r#"
            fn lookup(n: i32) -> Result<Site> { todo() }
            impl Ctor {
                fn rows(&self) -> &'static [Row] { todo() }
                fn meta(&self) -> Option<(i32, bool)> { todo() }
                pub fn key(c: &Ctor, a: Attrs, ps: JArray<Class>) -> Option<()> {
                    ps.get(0)?.descriptorString()?;
                    lookup(1)?.describe()?;
                    let p = c.__get_params().get(0).ok()?;
                    p.getName()?;
                    let z = c.parent().unwrap_or_default();
                    z.getSuper()?;
                    let o = a.read().ok();
                    o.isDir();
                    let m = match a.open() { Ok(f) => f, Err(e) => return None };
                    m.sync()?;
                    Clone::clone(&a).flush()?;
                    let k = |q: &Path| q.toUri();
                    None
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("key").map(|i| i.calls).unwrap_or_default();
        let srecv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.srecv.clone());
        let ctor = named(&["Ctor"]);
        let params = SType::Field(Box::new(ctor.clone()), "params".into());
        assert_eq!(srecv("getName"), Some(SType::Call(Box::new(params), "get".into())));
        assert_eq!(srecv("getSuper"), Some(SType::Call(Box::new(ctor), "parent".into())));
        assert_eq!(srecv("isDir"), None);
        assert_eq!(srecv("descriptorString"), Some(named(&["Class"])));
        assert_eq!(srecv("describe"), Some(named(&["Site"])));
        let attrs = named(&["Attrs"]);
        assert_eq!(srecv("sync"), Some(SType::Call(Box::new(attrs.clone()), "open".into())));
        assert_eq!(srecv("flush"), Some(attrs));
        assert_eq!(srecv("toUri"), Some(named(&["Path"])));
        let ty = vec!["Ctor".to_string()];
        assert_eq!(out.rets.get(&(ty.clone(), "rows".into())), Some(&vec![]));
        assert_eq!(out.rets.get(&(ty, "meta".into())), Some(&vec![]));
    }

    /// 模块路径上的自由 fn（`super::thread_impl::f()?`）记为以模块路径为宿主的返回，由引擎按目标文件声明解析；
    /// 外部 crate 路径（`std::mem::take`）不记
    #[test]
    fn module_path_free_fn_returns() {
        let src = r#"
            impl Loader {
                fn boot(scl: Loader) -> Result<()> {
                    super::thread_impl::initial()?.setContext(scl)?;
                    crate::java::lang::thread_impl::initial()?.start()?;
                    std::mem::take(&mut v).push(1);
                    Ok(())
                }
            }
        "#;
        let file = syn::parse_file(src).expect("测试源码可解析");
        let mut out = FileFns::default();
        scan_file(&file, &HashMap::new(), &mut out);
        let cs = out.fns.remove("boot").map(|i| i.calls).unwrap_or_default();
        let srecv = |n: &str| cs.iter().find(|c| c.name == n).and_then(|c| c.srecv.clone());
        let ret = |p: &[&str]| Some(SType::Ret(TypeRef(p.iter().map(|s| s.to_string()).collect()), "initial".into()));
        assert_eq!(srecv("setContext"), ret(&["super", "thread_impl"]));
        assert_eq!(srecv("start"), ret(&["crate", "java", "lang", "thread_impl"]));
        assert_eq!(srecv("push"), None);
    }
}

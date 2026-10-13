//! 每类方法 trait `X__Methods`（S7 计划 §9.9 K3）：wrapper 固有 impl 的可读层成员
//! （Java 方法外壳、继承转发、构造器、静态方法、字段访问器、`__class_init`、`_init_not_null`）
//! 移入与 wrapper 同类型形参的 trait，由 wrapper 实现。
//!
//! trait 方法签名与原固有方法逐一相同（只去可见性与声明处模式），trait 形参与 wrapper
//! 形参一一对应（`impl<G> X__Methods<G> for X<G>`）→ 静态调用点 `X::m(..)` 的类型形参推断
//! 与固有 impl 一致。调用方按需导入 `use …::X__Methods as _;`（生成器按字节码指令的 owner 登记）。
//!
//! 留在固有 impl 的成员：
//! - `const fn`（trait 方法不可为 const）；
//! - 与 wrapper 已实现的 std prelude trait（及其 blanket impl）同名的方法（[`STD_COLLISIONS`]）：
//!   trait 方法与 `Clone::clone` / `From::from` 等同名即 E0034 歧义，固有方法则优先于 trait 方法；
//! - 关联常量（`__STATICS` 等）。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{FnArg, ImplItem, ImplItemFn, Pat, TraitItemFn, Visibility};

use super::super::context::GenContext;

/// wrapper 实现的 std prelude trait（Clone / PartialEq / Default / From，及 blanket 的
/// Into / TryFrom / TryInto / ToOwned）的方法名：同名 Java 方法留在固有 impl
pub(crate) const STD_COLLISIONS: [&str; 11] = [
    "clone", "clone_from", "clone_into", "to_owned", "eq", "ne", "default", "from", "into", "try_from",
    "try_into",
];

/// 该固有成员是否移入方法 trait
///
/// 不移入（除 `const fn` 与 std 同名外，S7 计划 §9.9 K3 收尾）：字段访问器（`__get_*` / `__set_*`）、
/// `_init_not_null`、`__init_on`——它们的跨 crate 调用点（实现层体、子类构造器体、反射分派尾巴）
/// 都在实现层或对本类，固有 impl 即可解析；放进 trait 只会让声明层 crate 的展开 / 类型检查 /
/// 元数据为每个字段多付一份签名（§9.9 K3 峰值归因：trait 项是签名双份的主要来源）。
/// K6 搬 struct 进标记 crate 后固有失效，届时按 K6 前置项处理（同 std 同名成员，见下）。
///
/// 接口载体（§9.9 K6 前置项，经 [`split_parts`]）另有内部成员留固有：`is_jvm_null`（null 探测，
/// 与类 wrapper 的固有方法同形，K6 归标记层身份面）、`__default_*`（default 体的单一持有处，
/// 载体分派回退与合成对象 vtable 条目都直调——固有调用无需 trait 导入）
///
/// 装箱工厂族 `valueOf_*`（按描述符后缀命名的全部，包装类 `valueOf_z` / `valueOf_i` / …）同样
/// 留固有（b1013-fix，§9.9 协议面保留名单扩充）：运行时手写层（`reflect_dispatch::box_value`、
/// `reflect/Array.get` 装箱、`proxy_dyn` 重装箱）以 `X::valueOf_z(..)` 路径直调——它们是 VM 装箱
/// 协议面，与访问器同级，不应依赖调用方导入方法 trait。按前缀识别（不类名特判）：带描述符后缀的
/// 重载名只出现在工厂/解析工厂形态；无后缀 `valueOf`（枚举 `valueOf(String)`、Character 唯一
/// 重载等）仍移入 trait，由生成器登记导入或手写层 use 行对齐。
///
/// 构造器（`new` / `new_<描述符后缀>`，与 `util::classify_method` 的 Constructor 名字口径
/// 一致——`main` 除外，入口钩子按 b1015-fix 走 trait 导入）同样留固有（mark-ctor-fix，§9.9 协议面保留
/// 名单再扩充）：手写 native 层按 errno / IO 错误构造异常类族（random_access_file / net /
/// dispatcher / process_impl 等的 `IOException::new_str`、`ProtocolException::new_str` …），
/// 这些文件的伴生声明按闭包条件发生——哪个闭包拉入宿主类事先不可知，per-file 方法 trait 导入
/// 无法穷举对齐（b1013-fix 的 25 文件对齐即漏了这批，与 K6 合批互为盲区第三例）。构造器是
/// 手写层直调的协议面，与 `valueOf_*` / 访问器同级；按名字识别（不类名特判），Java 方法名不可能
/// 叫 `new`（关键字），碰巧叫 `new_<x>` 的真方法留固有亦无语义差（固有优先解析，生成侧调用
/// 点两种形态都可解析）。
fn moves_to_trait(f: &ImplItemFn) -> bool {
    let n = f.sig.ident.to_string();
    f.sig.constness.is_none()
        && !STD_COLLISIONS.contains(&n.as_str())
        && !matches!(n.as_str(), "_init_not_null" | "__init_on" | "is_jvm_null")
        && !n.starts_with("__get_")
        && !n.starts_with("__set_")
        && !n.starts_with("__default_")
        && !n.starts_with("valueOf_")
        && !(n == "new" || n.starts_with("new_"))
}

/// trait 中的方法声明：签名同实现（形参模式去 `mut` / 非标识符模式改 `_`，无体声明不接受模式），
/// 属性去 `#[inline]`（只对有体函数有意义）
fn declaration(f: &ImplItemFn) -> TraitItemFn {
    let mut sig = f.sig.clone();
    for arg in sig.inputs.iter_mut() {
        match arg {
            FnArg::Receiver(r) => {
                if r.reference.is_none() {
                    r.mutability = None;
                }
            }
            FnArg::Typed(pt) => {
                let plain = match &*pt.pat {
                    Pat::Ident(pi) if pi.subpat.is_none() => {
                        let mut pi = pi.clone();
                        pi.mutability = None;
                        pi.by_ref = None;
                        Pat::Ident(pi)
                    }
                    _ => syn::parse_quote!(_),
                };
                *pt.pat = plain;
            }
        }
    }
    let attrs = f.attrs.iter().filter(|a| !a.path().is_ident("inline")).cloned().collect();
    TraitItemFn { attrs, sig, default: None, semi_token: Some(Default::default()) }
}

/// wrapper 成员 → (方法 trait 声明 + wrapper 的 trait 实现, 留在固有 impl 的成员)
pub(super) fn split(ctx: &GenContext, members: TokenStream2) -> syn::Result<TokenStream2> {
    split_parts(&ctx.struct_ident, &ctx.impl_g, &ctx.ty_g, &ctx.where_c, members)
}

/// [`split`] 的载体形态（接口载体无 GenContext，§9.9 K6 前置项：载体成员同样移入
/// `I__Methods`——K6 搬载体 struct 进标记 crate 后固有 impl 跨 crate 失效）。
/// 三段泛型即 `Generics::split_for_impl` 的展开片段（TokenStream 形态，与 GenContext 同）
pub(crate) fn split_parts(
    struct_ident: &syn::Ident,
    impl_g: &TokenStream2,
    ty_g: &TokenStream2,
    where_c: &TokenStream2,
    members: TokenStream2,
) -> syn::Result<TokenStream2> {
    let trait_ident = format_ident!("{}__Methods", struct_ident);
    let all: syn::ItemImpl = syn::parse2(quote! {
        impl #impl_g #struct_ident #ty_g #where_c { #members }
    })?;

    let mut decls: Vec<TraitItemFn> = Vec::new();
    let mut impls: Vec<ImplItemFn> = Vec::new();
    let mut inherent: Vec<ImplItem> = Vec::new();
    for item in all.items {
        match item {
            ImplItem::Fn(f) if moves_to_trait(&f) => {
                decls.push(declaration(&f));
                let mut f = f;
                f.vis = Visibility::Inherited;
                impls.push(f);
            }
            other => inherent.push(other),
        }
    }

    let inherent_impl = if inherent.is_empty() {
        quote! {}
    } else {
        quote! { impl #impl_g #struct_ident #ty_g #where_c { #(#inherent)* } }
    };
    Ok(quote! {
        #[doc(hidden)]
        #[allow(non_camel_case_types, non_snake_case)]
        pub trait #trait_ident #impl_g : ::core::marker::Sized #where_c {
            #(#decls)*
        }
        #[allow(non_snake_case)]
        impl #impl_g #trait_ident #ty_g for #struct_ident #ty_g #where_c {
            #(#impls)*
        }
        #inherent_impl
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn decl_of(src: &str) -> String {
        let f: ImplItemFn = syn::parse_str(src).unwrap();
        let d = declaration(&f);
        quote!(#d).to_string()
    }

    #[test]
    fn declaration_strips_patterns_and_inline() {
        let d = decl_of("#[inline] #[doc(hidden)] pub fn m(mut self, mut x: i32, (a, b): (u8, u8)) -> i32 { x }");
        assert!(!d.contains("inline"), "{d}");
        assert!(d.contains("doc (hidden)"), "{d}");
        assert!(d.contains("fn m (self , x : i32 , _ : (u8 , u8)) -> i32 ;"), "{d}");
        let d = decl_of("pub fn n(&mut self) {}");
        assert!(d.contains("fn n (& mut self) ;"), "{d}");
    }

    #[test]
    fn collisions_and_const_fns_stay_inherent() {
        let keep: ImplItemFn = syn::parse_str("pub fn clone(&self) -> Result<Object> { todo() }").unwrap();
        assert!(!moves_to_trait(&keep));
        let keep: ImplItemFn = syn::parse_str("pub const fn k() -> u8 { 0 }").unwrap();
        assert!(!moves_to_trait(&keep));
        let mv: ImplItemFn = syn::parse_str("pub fn size(&self) -> Result<i32> { Ok(0) }").unwrap();
        assert!(moves_to_trait(&mv));
    }

    const GENERIC_CLASS: &str = r#"
    #[binary_name = "a/L"]
    pub struct L<E> { pub n: i32 }
    impl<E> L<E> {
        pub static COUNT: i32;
        #[java_method(name = "<init>", descriptor = "()V", access = "public")]
        pub fn new() -> Result<Self> {
            let mut this = Self::default();
            this._init_not_null();
            Self::__init_on(this)
        }
        #[doc(hidden)]
        pub fn __init_on(mut this: Self) -> Result<Self> { Ok(this) }
        #[java_method(name = "make", descriptor = "(I)La/L;", access = "public", modifiers = "static")]
        pub fn make(mut k: i32) -> Result<L<Object>> { k += 1; Ok(L::<Object>::default()) }
        #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public")]
        pub fn clone(&self) -> Result<Object> { Ok(Object::default()) }
    }
"#;

    fn expanded() -> String {
        use std::str::FromStr;
        let out = super::super::super::super::expand(TokenStream2::from_str(GENERIC_CLASS).unwrap()).to_string();
        assert!(!out.contains("compile_error"), "{out}");
        out
    }

    /// 方法 trait 形参与 wrapper 形参一一对应、实现头 `impl<E> L__Methods<E> for L<E>`：
    /// 静态调用点 `L::new()` / `L::make(..)` 的 `E` 由 Self 类型唯一确定，推断与固有 impl 一致
    #[test]
    fn trait_mirrors_wrapper_generics() {
        let out = expanded();
        // 形参带宏补的界（`E : Clone + Default + …`），trait 与实现头同界
        let bounds = out.split("pub struct L < ").nth(1).unwrap().split(" > {").next().unwrap();
        assert!(out.contains(&format!("pub trait L__Methods < {bounds} > : :: core :: marker :: Sized")), "{out}");
        assert!(out.contains(&format!("impl < {bounds} > L__Methods < E > for L < E >")), "{out}");
        // mark-ctor-fix：构造器 `new` 留固有 impl（手写层直调协议面），trait 声明不再含它
        assert!(out.contains("pub fn new () -> Result < Self > {"), "{out}");
        assert!(out.contains("fn make (k : i32) -> Result < L < Object > > ;"), "{out}");
        for m in ["fn COUNT () -> Result < i32 > ;", "fn __class_init () -> Result < () > ;"] {
            assert!(out.contains(m), "{m}: {out}");
        }
        // K3 收尾：访问器 / _init_not_null / __init_on 留固有 impl（trait 项少一份签名）；
        // mark-ctor-fix 起构造器 `new` 同列
        let tr = out.split("pub trait L__Methods").nth(1).unwrap().split("impl <").next().unwrap();
        for m in ["fn _init_not_null", "fn __get_n", "fn __init_on", "fn new ()"] {
            assert!(!tr.contains(m), "{m} 不应在 trait 声明里: {tr}");
        }
        assert!(out.contains("pub fn _init_not_null (& mut self)"), "{out}");
        assert!(out.contains("pub fn __get_n (& self) -> i32"), "{out}");
    }

    /// 与 std prelude trait 同名的 Java 方法（`clone`）留在固有 impl（trait 内同名即 E0034）。
    /// 生成器侧对这类成员已统一改 `__` 前缀内部名（fn 标识符 `__clone`，§9.9 K6 前置项）——
    /// 本测试守护宏对**未经改名**的块输入（手写块）的兜底行为
    #[test]
    fn std_named_method_stays_inherent() {
        let out = expanded();
        let tr = out.split("pub trait L__Methods").nth(1).unwrap().split("impl <").next().unwrap();
        assert!(!tr.contains("fn new ()"), "构造器留固有（mark-ctor-fix）: {tr}");
        assert!(!tr.contains("fn clone"), "{tr}");
        assert!(out.contains("pub fn new () -> Result < Self >"), "{out}");
        assert!(out.contains("pub fn clone (& self) -> Result < Object >"), "{out}");
    }

    /// 生成器改过名的 std 同名成员（`__clone`）不在 [`STD_COLLISIONS`]，正常移入方法 trait
    #[test]
    fn renamed_std_member_moves_to_trait() {
        use std::str::FromStr;
        let src = r#"
        #[binary_name = "a/C"]
        pub struct C { pub n: i32 }
        impl C {
            #[java_method(name = "clone", descriptor = "()Ljava/lang/Object;", access = "public")]
            pub fn __clone(&self) -> Result<Object> { Ok(Object::default()) }
        }
    "#;
        let out = super::super::super::super::expand(TokenStream2::from_str(src).unwrap()).to_string();
        assert!(!out.contains("compile_error"), "{out}");
        let tr = out.split("pub trait C__Methods").nth(1).unwrap().split("impl <").next().unwrap();
        assert!(tr.contains("fn __clone"), "{tr}");
        assert!(out.contains("impl C__Methods for C"), "{out}");
    }

    /// 装箱工厂族 `valueOf_*`（描述符后缀命名）留固有 impl（b1013-fix：运行时协议面，
    /// `reflect_dispatch::box_value` 等路径直调不导入方法 trait）；无后缀 `valueOf` 仍入 trait
    #[test]
    fn boxing_factory_stays_inherent() {
        use std::str::FromStr;
        let src = r#"
        #[binary_name = "java/lang/Integer"]
        pub struct Integer { pub value: i32 }
        impl Integer {
            #[java_method(name = "valueOf", descriptor = "(I)Ljava/lang/Integer;", access = "public", modifiers = "static")]
            pub fn valueOf_i(x: i32) -> Result<Integer> { Ok(Integer::default()) }
            #[java_method(name = "valueOf", descriptor = "(Ljava/lang/String;)Ljava/lang/Integer;", access = "public", modifiers = "static")]
            pub fn valueOf_ljava_lang_String(s: String) -> Result<Integer> { Ok(Integer::default()) }
        }
    "#;
        let out = super::super::super::super::expand(TokenStream2::from_str(src).unwrap()).to_string();
        assert!(!out.contains("compile_error"), "{out}");
        // trait 声明段：`pub trait Integer__Methods` 到其实现头（非泛型类无 `impl <`，按实现类名切）
        let tr = out.split("pub trait Integer__Methods").nth(1).unwrap().split("impl Integer__Methods").next().unwrap();
        assert!(!tr.contains("valueOf_"), "装箱工厂不应在 trait 声明里: {tr}");
        assert!(out.contains("pub fn valueOf_i (x : i32)"), "{out}");
        assert!(out.contains("pub fn valueOf_ljava_lang_String (s : String)"), "{out}");
    }

    /// 构造器（`new` / `new_<描述符后缀>`）留固有 impl（mark-ctor-fix：手写 native 层按 errno /
    /// IO 错误构造异常类族的协议面，伴生按闭包条件发生、per-file trait 导入无法穷举对齐——
    /// batch-1015 Log4j2ApiMain E0599 ×21 同构第三例）；`__init_on` 孪生与静态方法仍入 trait
    #[test]
    fn constructors_stay_inherent() {
        use std::str::FromStr;
        let src = r#"
        #[binary_name = "java/io/IOException"]
        #[superclass = "Exception"]
        pub struct IOException;
        impl IOException {
            #[java_method(name = "<init>", descriptor = "()V", access = "public")]
            pub fn new() -> Result<Self> { Ok(Self::default()) }
            #[doc(hidden)]
            pub fn __init_on(mut this: Self) -> Result<Self> { Ok(this) }
            #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;)V", access = "public")]
            pub fn new_str(message: String) -> Result<Self> { Ok(Self::default()) }
            #[doc(hidden)]
            pub fn __init_on_str(mut this: Self, mut message: String) -> Result<Self> { Ok(this) }
            #[java_method(name = "<init>", descriptor = "(Ljava/lang/String;Ljava/lang/Throwable;)V", access = "public")]
            pub fn new_str_throwable(message: String, cause: Throwable) -> Result<Self> { Ok(Self::default()) }
        }
    "#;
        let out = super::super::super::super::expand(TokenStream2::from_str(src).unwrap()).to_string();
        assert!(!out.contains("compile_error"), "{out}");
        let tr = out.split("pub trait IOException__Methods").nth(1).unwrap().split("impl IOException__Methods").next().unwrap();
        for m in ["fn new ()", "fn new_str (", "fn new_str_throwable ("] {
            assert!(!tr.contains(m), "构造器 {m} 不应在 trait 声明里: {tr}");
        }
        // 固有 impl 保留 pub（手写层 crate 内任意路径直调可解析）
        for m in ["pub fn new () -> Result < Self >", "pub fn new_str (message : String)", "pub fn new_str_throwable (message : String"] {
            assert!(out.contains(m), "构造器 {m} 应在固有 impl: {out}");
        }
        // 孪生入口与静态方法不受影响（生成侧调用点按账本导入）
        assert!(tr.contains("fn __init_on_str ("), "{tr}");
    }
}

//! 类初始化（JVMS §5.5）与 static 字段存储。
//!
//! codegen 在 `java_class!` 块内只声明事实：
//!   - `pub static NAME: Type;`        有存储的 static 字段
//!   - `pub const NAME: Type = expr;`  ConstantValue 编译期常量
//!   - `fn __clinit() -> Result<()>`   `<clinit>` 字节码的翻译
//!
//! 宏把它们展开为：
//!   - 进程级无锁存储：常量初始化的普通 `static`——基本类型 `__PrimCell<T>`（原子），引用类型
//!     `__RefField<Option<T>>`（内联自旋单元），无 `OnceLock` 惰性层、无 `static mut`
//!   - `NAME()` / `set_NAME(v)` 访问器：入口先触发 `__class_init()`
//!   - `__class_init()`：状态机保证 `<clinit>` 恰好执行一次；初始化进行中的同线程递归
//!     访问立即返回（JVMS §5.5 步骤 3）；先初始化父类（步骤 7）；`<clinit>` 抛异常后类
//!     进入 erroneous 状态，后续主动使用抛 NoClassDefFoundError（步骤 5 / 11）
//!   - static 方法 / 构造器入口注入 `Self::__class_init()?;`（invokestatic / new 触发点）
//!
//! 可读层（方法体）只看到 `Locale::defaultLocale()?`，初始化触发不可见。

use std::collections::HashSet;

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{Attribute, Block, Ident, ReturnType, Signature, Type};

use super::parse::StaticItem;
use super::util::{is_basic, java_field_is_volatile, strip_meta_attrs};

/// 可放入原子基本单元的 static 字段类型（JVM 基本类型的 Rust 映射；`__AtomicRepr` 实现集）
fn is_atomic_prim(ty: &Type) -> bool {
    is_basic(ty) && !matches!(quote!(#ty).to_string().as_str(), "i128" | "u128")
}

/// `<clinit>` 翻译函数在块内的固定名字。
pub(crate) const CLINIT_FN: &str = "__clinit";

/// 方法是否返回 `Result<..>`（只有可传播异常的入口才能注入初始化触发）。
pub(crate) fn returns_result(sig: &Signature) -> bool {
    if let ReturnType::Type(_, ty) = &sig.output {
        if let Type::Path(tp) = &**ty {
            return tp.path.segments.last().map_or(false, |s| s.ident == "Result");
        }
    }
    false
}

/// 该函数是否是类初始化触发点：无接收者（static 方法 / 构造器 / main）且不是 `<clinit>` 自身。
pub(crate) fn is_init_trigger(sig: &Signature) -> bool {
    sig.receiver().is_none() && sig.ident != CLINIT_FN && returns_result(sig)
}

/// 方法入口检查，只对可传播异常（返回 `Result`）的方法生成：
/// 1. 实例方法的空接收者检查（JVMS §6.5 invokevirtual / invokespecial / invokeinterface：
///    objectref 为 null 抛 NullPointerException，先于建帧）；
/// 2. 栈界检查 `__stack_check()?`（建帧：栈耗尽抛 StackOverflowError，a3-T1b，计划 §21.8.2）；
///    生成器从字节码判定为叶子的方法（`leaf = "true"`：无调用指令且足够短）省略——叶子帧之下
///    不再有 Java 帧，无界递归必经其调用者的检查点，叶子帧本身落在 `SHADOW` 余量内（a3-T1b-2）。
pub(crate) fn entry_checks(sig: &Signature, attrs: &[Attribute]) -> proc_macro2::TokenStream {
    if !returns_result(sig) {
        return quote::quote! {};
    }
    let has_recv = sig.receiver().is_some();
    if is_leaf(attrs) {
        return if has_recv {
            quote::quote! { if self.__r.is_none() { return Err(JvmError::null_pointer()); } }
        } else {
            quote::quote! {}
        };
    }
    // 实例方法：两项合为一次 runtime 调用 `__enter`（次序同上：先空接收者、后栈界），
    // 每个入口少一个分支块与一处 `?` 展开（声明层外壳数以万计，按条目计的前端内存随之下降）
    if has_recv {
        quote::quote! { __enter(self.__r.is_none())?; }
    } else {
        quote::quote! { __stack_check()?; }
    }
}

/// 转发外壳（虚分派 / 继承转发）的入口检查：只做空接收者检查，不建帧。
/// 外壳本身不是 Java 帧——分派到的目标方法体（`__jbm_*` 体函数）自带完整入口检查，
/// 栈界检查在那里做一次；空接收者必须在外壳判（null 引用无存储可分派，分派后
/// 目标看到的接收者不再带 null 标志；手写目标也不带检查）。形态同叶子方法的空检查
pub(crate) fn forward_checks(sig: &Signature) -> proc_macro2::TokenStream {
    if returns_result(sig) && sig.receiver().is_some() {
        quote::quote! { if self.__r.is_none() { return Err(JvmError::null_pointer()); } }
    } else {
        quote::quote! {}
    }
}

/// vtable-safe 方法体（`_base` 真实体，经 vtable 直连执行、不经 `__jbm_*`）的建帧检查：
/// 只做栈界检查（`this` 是 `&dyn` 存储视图，无 null 标志——空接收者已在转发外壳判过），
/// 叶子方法省略，规则同 [`entry_checks`]
pub(crate) fn frame_check(sig: &Signature, attrs: &[Attribute]) -> proc_macro2::TokenStream {
    if returns_result(sig) && !is_leaf(attrs) {
        quote::quote! { __stack_check()?; }
    } else {
        quote::quote! {}
    }
}

/// 生成器标注的叶子方法（`#[java_method(.., leaf = "true")]`）
pub(crate) fn is_leaf(attrs: &[Attribute]) -> bool {
    super::util::attr_str(attrs, "leaf").as_deref() == Some("true")
}

/// 在方法体入口注入 `Self::__class_init()?;`。
pub(crate) fn inject_init_trigger(block: &mut Block) {
    block.stmts.insert(0, syn::parse_quote! { Self::__class_init()?; });
}

/// 展开 static 字段声明。返回 (模块级存储项, impl 块成员)。
pub(crate) fn expand_statics(
    struct_ident: &Ident,
    statics: &[StaticItem],
    impl_methods: &HashSet<String>,
) -> (Vec<TokenStream2>, Vec<TokenStream2>) {
    let mut storage: Vec<TokenStream2> = Vec::new();
    let mut members: Vec<TokenStream2> = Vec::new();
    for st in statics {
        let keep_attrs = strip_meta_attrs(&st.attrs);
        let vis = &st.vis;
        let name = &st.name;
        let ty = &st.ty;
        let getter_handwritten = impl_methods.contains(&name.to_string());
        if let Some(value) = &st.const_value {
            if !getter_handwritten {
                members.push(quote! {
                    #(#keep_attrs)*
                    #[allow(non_snake_case)]
                    #[inline]
                    #vis fn #name() -> Result<#ty> { Ok(#value) }
                });
            }
            continue;
        }
        let cell = format_ident!("__STATIC_{}_{}", struct_ident, name);
        let setter = format_ident!("set_{}", name);
        let (raw_get, raw_set) = (raw_getter(name), raw_setter(name));
        // 无锁静态单元（R1 Q1(a)）：常量初始化的普通 `static`，不经 `OnceLock`。
        // 基本类型为原子单元（普通字段 relaxed、volatile 为 SeqCst）；引用类型为内联自旋单元
        // （读写经获取 / 释放序）。`<clinit>` 写入与读者之间的 happens-before 由初始化状态的
        // 发布（`clinit_exit`）/ 观察（`__class_init` 快路径）给出（JLS §12.4.2）
        let (cell_ty, load, store) = if is_atomic_prim(ty) {
            let (load, store) = if java_field_is_volatile(&st.attrs) {
                (quote! { #cell.get() }, quote! { #cell.set(v) })
            } else {
                (quote! { #cell.get_plain() }, quote! { #cell.set_plain(v) })
            };
            (quote! { __PrimCell<#ty> = __PrimCell::zeroed() }, load, store)
        } else {
            (
                quote! { __RefField<::std::option::Option<#ty>> = __RefField::new(::std::option::Option::None) },
                quote! { #cell.get_or_default() },
                quote! { #cell.set(::std::option::Option::Some(v)) },
            )
        };
        storage.push(quote! {
            #[allow(non_upper_case_globals)]
            static #cell: #cell_ty;
        });
        if !getter_handwritten {
            members.push(quote! {
                #(#keep_attrs)*
                #[allow(non_snake_case)]
                #vis fn #name() -> Result<#ty> {
                    Self::__class_init()?;
                    Self::#raw_get()
                }
            });
            members.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[inline]
                #vis fn #raw_get() -> Result<#ty> {
                    __safepoint();
                    Ok(#load)
                }
            });
        }
        if !impl_methods.contains(&setter.to_string()) {
            members.push(quote! {
                #[allow(non_snake_case)]
                #vis fn #setter(v: #ty) -> Result<()> {
                    Self::__class_init()?;
                    Self::#raw_set(v)
                }
            });
            members.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                #[inline]
                #vis fn #raw_set(v: #ty) -> Result<()> {
                    #store;
                    Ok(())
                }
            });
        }
    }
    members.push(statics_table(statics));
    (storage, members)
}

/// 带反射标记的静态字段的 Java 字段名：`cfg_attr(any(), java_field(name = "..", .., reflect = true))`
/// （生成器只给档案内可经按名反射 / 序列化协议访问的静态字段打标记）。
fn reflected_java_name(attrs: &[Attribute]) -> Option<String> {
    use syn::punctuated::Punctuated;
    use syn::{Expr, ExprLit, Lit, Meta, MetaNameValue, Token};
    // 按语法树解析属性实参：属性的字符串化形态随 proc_macro 后端（编译器 / 回退实现）不同，不作判据
    attrs.iter().filter(|a| a.path().is_ident("cfg_attr")).find_map(|a| {
        let metas = a.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated).ok()?;
        metas.iter().find_map(|m| {
            let Meta::List(list) = m else { return None };
            if !list.path.is_ident("java_field") {
                return None;
            }
            let pairs = list.parse_args_with(Punctuated::<MetaNameValue, Token![,]>::parse_terminated).ok()?;
            let (mut name, mut reflect) = (None, false);
            for nv in &pairs {
                match &nv.value {
                    Expr::Lit(ExprLit { lit: Lit::Str(s), .. }) if nv.path.is_ident("name") => name = Some(s.value()),
                    Expr::Lit(ExprLit { lit: Lit::Bool(b), .. }) if nv.path.is_ident("reflect") => reflect = b.value,
                    _ => {}
                }
            }
            name.filter(|_| reflect)
        })
    })
}

/// 静态字段表 `__STATICS`（S7-3b，按名字段访问的静态半边，见运行时 `field_reflect`）：每个带反射
/// 标记的 Java 静态字段一项，经 `__StaticFieldDesc::of_prim / of_ref` 记 Java 名、既有访问器
/// （getter / setter，手写同名访问器同样适用）与按值类型实例化的读写协议。关联常量只在被引用
/// （main 的登记）时求值与代码生成。
fn statics_table(statics: &[StaticItem]) -> TokenStream2 {
    const PRIMS: [&str; 8] = ["bool", "i8", "i16", "u16", "i32", "f32", "i64", "f64"];
    let entries = statics.iter().filter_map(|st| {
        let java = reflected_java_name(&st.attrs)?;
        let (name, ty) = (&st.name, &st.ty);
        let ctor = if is_basic(ty) {
            let Type::Path(tp) = ty else { return None };
            if !tp.path.get_ident().is_some_and(|i| PRIMS.contains(&i.to_string().as_str())) {
                return None;
            }
            quote! { of_prim }
        } else {
            quote! { of_ref }
        };
        let set = if st.const_value.is_some() {
            quote! { ::core::option::Option::None }
        } else {
            let setter = format_ident!("set_{}", name);
            quote! { ::core::option::Option::Some(Self::#setter) }
        };
        Some(quote! { __StaticFieldDesc::#ctor::<#ty>(#java, Self::#name, #set) })
    });
    quote! {
        #[doc(hidden)]
        pub const __STATICS: &'static [__StaticFieldDesc] = &[#(#entries),*];
    }
}

/// 免初始化触发的 static 访问器名（`__si_NAME` / `__si_set_NAME`）：只供本类初始化触发点之内
/// 的调用（见 [`OwnStatics`]）
fn raw_getter(name: &Ident) -> Ident {
    format_ident!("__si_{}", name)
}

fn raw_setter(name: &Ident) -> Ident {
    format_ident!("__si_set_{}", name)
}

/// 本类有宏生成访问器的 static 字段（R1 Q1(b)）。
///
/// 本类的 static 方法 / 构造器 / `<clinit>` 体内，入口已触发（或正由本线程执行）本类初始化
/// （JVMS §5.5：invokestatic / new 是触发点；初始化进行中的同线程访问立即返回），体内对本类
/// static 字段的读写不必再经 `__class_init()`：调用点 `Own::f()` / `Self::f()` / `Own::set_f(v)`
/// 改写为免触发访问器。闭包体不改写（可能在他线程、本类初始化完成前执行，须照常等待）。
/// 实例方法不改写：运行时手写层可不经构造器造出实例，实例存在不保证本类已初始化。
pub(crate) struct OwnStatics {
    ident: Ident,
    getters: HashSet<String>,
    setters: HashSet<String>,
}

impl OwnStatics {
    pub(crate) fn new(struct_ident: &Ident, statics: &[StaticItem], impl_methods: &HashSet<String>) -> Self {
        let mut getters = HashSet::new();
        let mut setters = HashSet::new();
        for st in statics.iter().filter(|st| st.const_value.is_none()) {
            let name = st.name.to_string();
            if !impl_methods.contains(&name) {
                getters.insert(name.clone());
            }
            if !impl_methods.contains(&format!("set_{name}")) {
                setters.insert(name);
            }
        }
        OwnStatics { ident: struct_ident.clone(), getters, setters }
    }

    /// 本类初始化触发点（static 方法 / 构造器）与 `<clinit>` 的体内改写本类 static 访问
    pub(crate) fn rewrite_in(&self, sig: &Signature, block: &mut Block) {
        if (self.getters.is_empty() && self.setters.is_empty())
            || !(is_init_trigger(sig) || sig.ident == CLINIT_FN)
        {
            return;
        }
        syn::visit_mut::VisitMut::visit_block_mut(&mut OwnStaticRewriter(self), block);
    }
}

struct OwnStaticRewriter<'a>(&'a OwnStatics);

impl syn::visit_mut::VisitMut for OwnStaticRewriter<'_> {
    fn visit_expr_closure_mut(&mut self, _: &mut syn::ExprClosure) {}

    fn visit_item_mut(&mut self, _: &mut syn::Item) {}

    fn visit_expr_call_mut(&mut self, call: &mut syn::ExprCall) {
        syn::visit_mut::visit_expr_call_mut(self, call);
        let syn::Expr::Path(p) = &mut *call.func else { return };
        if p.qself.is_some() || p.path.segments.len() != 2 {
            return;
        }
        let owner = &p.path.segments[0];
        if !owner.arguments.is_empty() || !(owner.ident == "Self" || owner.ident == self.0.ident) {
            return;
        }
        let last = &p.path.segments[1];
        if !last.arguments.is_empty() {
            return;
        }
        let m = last.ident.to_string();
        let raw = match call.args.len() {
            0 if self.0.getters.contains(&m) => raw_getter(&last.ident),
            1 => match m.strip_prefix("set_") {
                Some(f) if self.0.setters.contains(f) => raw_setter(&format_ident!("{}", f)),
                _ => return,
            },
            _ => return,
        };
        p.path.segments[0].ident = format_ident!("Self");
        p.path.segments[1].ident = raw;
    }
}

/// 生成 `__class_init()`。返回 (模块级状态项, impl 块成员)。
///
/// 状态：0 = 未初始化；1 = 初始化中；2 = erroneous（`<clinit>` 曾抛出异常）；3 = 已完成。
/// 多线程协议（他线程等待 / 同线程递归返回）由运行时 `gil::clinit_enter/exit` 承载。
///
pub(crate) fn expand_class_init(
    struct_ident: &Ident,
    binary_name: &str,
    superclass: Option<&Type>,
    init_interfaces: &[Type],
    has_clinit: bool,
) -> (TokenStream2, TokenStream2) {
    let state = format_ident!("__CLINIT_STATE_{}", struct_ident);
    let storage = quote! {
        #[allow(non_upper_case_globals)]
        static #state: __PrimCell<u8> = __PrimCell::zeroed();
    };
    let init_super = superclass.map(|sup| quote! { <#sup>::__class_init()?; });
    // JVMS §5.5 步骤 7：父类之后、本类 `<clinit>` 之前，初始化带 default 方法的超接口
    let init_ifaces = init_interfaces.iter().map(|t| quote! { <#t>::__class_init()?; });
    let clinit_ident = format_ident!("{}", CLINIT_FN);
    // 带 `?;` 的语句形态（不再作为尾表达式）：无 `<clinit>` 时不生成该语句
    // （独立的 `Ok(())?;` 缺少类型上下文，无法推断）。
    let run_clinit = if has_clinit {
        quote! { Self::#clinit_ident()?; }
    } else {
        quote! {}
    };
    let member = quote! {
        pub fn __class_init() -> Result<()> {
            let __state = &#state;
            if __state.get() == 3 {
                return Ok(());
            }
            // 慢路径（JVMS §5.5 的等待 / 递归 / 失败协议）全程序一份，按类只给初始化体
            __class_init_run(#binary_name, __state, &|| -> Result<()> {
                #init_super
                #(#init_ifaces)*
                #run_clinit
                Ok(())
            })
        }

        /// 构建期引导映像：本类在构建期已完成初始化（静态字段已由启动序列写入映像值），
        /// 登记后直接进入「已初始化」，不运行 `<clinit>`
        #[doc(hidden)]
        pub fn __boot_initialized() {
            __boot_initialized_run(#binary_name, &#state);
        }
    };
    (storage, member)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method(src: &str) -> syn::ImplItemFn {
        syn::parse_str(src).unwrap()
    }

    fn checks(src: &str) -> String {
        let f = method(src);
        entry_checks(&f.sig, &f.attrs).to_string()
    }

    #[test]
    fn non_leaf_gets_stack_check() {
        let s = checks(r#"#[java_method(name = "f", descriptor = "()I")] fn f() -> Result<i32> { Ok(1) }"#);
        assert!(s.contains("__stack_check"), "{s}");
        let s = checks(r#"#[java_method(name = "f", descriptor = "()I")] fn f(&self) -> Result<i32> { Ok(1) }"#);
        assert!(s.contains("__enter (self . __r . is_none ()) ?"), "{s}");
    }

    #[test]
    fn leaf_omits_stack_check_keeps_null_check() {
        let s = checks(r#"#[java_method(name = "g", descriptor = "()I", leaf = "true")] fn g(&self) -> Result<i32> { Ok(1) }"#);
        assert!(!s.contains("__stack_check"), "{s}");
        assert!(s.contains("null_pointer"), "{s}");
        let s = checks(r#"#[java_method(name = "h", descriptor = "()I", leaf = "true")] fn h() -> Result<i32> { Ok(1) }"#);
        assert!(s.is_empty(), "{s}");
    }

    #[test]
    fn own_static_rewrite_in_init_triggers_only() {
        let st = |n: &str| StaticItem {
            attrs: vec![], vis: syn::parse_quote!(pub), name: format_ident!("{}", n),
            ty: syn::parse_quote!(i32), const_value: None,
        };
        let own = OwnStatics::new(&format_ident!("Foo"), &[st("a"), st("b")], &HashSet::from(["b".to_string()]));
        let f = method("fn s() -> Result<()> { let x = Foo::a()?; Foo::set_a(x)?; Self::b()?; Bar::a()?; let c = || Foo::a(); Ok(()) }");
        let mut b = f.block.clone();
        own.rewrite_in(&f.sig, &mut b);
        let s = quote!(#b).to_string();
        assert!(s.contains("Self :: __si_a ()"), "{s}");
        assert!(s.contains("Self :: __si_set_a (x)"), "{s}");
        assert!(s.contains("Self :: b ()"), "{s}");
        assert!(s.contains("Bar :: a ()"), "{s}");
        assert!(s.contains("| | Foo :: a ()"), "{s}");
        let f = method("fn i(&self) -> Result<()> { Foo::a()?; Ok(()) }");
        let mut b = f.block.clone();
        own.rewrite_in(&f.sig, &mut b);
        assert!(quote!(#b).to_string().contains("Foo :: a ()"));
    }

    #[test]
    fn non_result_has_no_checks() {
        assert!(checks(r#"fn k(&self) -> i32 { 1 }"#).is_empty());
    }

    fn statics_of(src: &str) -> String {
        let (_, statics) = syn::parse::Parser::parse_str(super::super::parse::parse_impl_fns, src).unwrap();
        statics_table(&statics).to_string()
    }

    #[test]
    fn statics_table_only_reflected_fields() {
        let s = statics_of(r#"
            #[cfg_attr(any(), java_field(name = "serialVersionUID", descriptor = "J", access = "private", modifiers = "static final", is_static = true, constant_value = "1", reflect = true))]
            // static field: serialVersionUID:J
            pub const serialVersionUID: i64 = 1i64;
            #[cfg_attr(any(), java_field(name = "hidden", descriptor = "I", is_static = true))]
            pub static hidden: i32;
        "#);
        assert!(s.contains("\"serialVersionUID\""), "{s}");
        assert!(!s.contains("\"hidden\""), "{s}");
    }
}

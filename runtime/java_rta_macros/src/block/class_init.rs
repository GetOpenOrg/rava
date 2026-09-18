//! 类初始化（JVMS §5.5）与 static 字段存储。
//!
//! codegen 在 `java_class!` 块内只声明事实：
//!   - `pub static NAME: Type;`        有存储的 static 字段
//!   - `pub const NAME: Type = expr;`  ConstantValue 编译期常量
//!   - `fn __clinit() -> Result<()>`   `<clinit>` 字节码的翻译
//!
//! 宏把它们展开为：
//!   - 线程局部存储（`thread_local!` + `RefCell<Option<T>>`，无 `static mut` / unsafe）
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
use syn::{Block, Ident, ReturnType, Signature, Type};

use super::parse::StaticItem;
use super::util::strip_meta_attrs;

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

/// 实例方法入口的空接收者检查（JVMS §6.5 invokevirtual / invokespecial / invokeinterface：
/// objectref 为 null 抛 NullPointerException）。只对可传播异常的方法生成。
pub(crate) fn null_receiver_check(sig: &Signature) -> proc_macro2::TokenStream {
    if sig.receiver().is_some() && returns_result(sig) {
        quote::quote! {
            if self._jvm_null { return Err(JvmError::null_pointer()); }
        }
    } else {
        quote::quote! {}
    }
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
        storage.push(quote! {
            ::std::thread_local! {
                #[allow(non_upper_case_globals)]
                static #cell: ::std::cell::RefCell<::std::option::Option<#ty>> =
                    const { ::std::cell::RefCell::new(::std::option::Option::None) };
            }
        });
        if !getter_handwritten {
            members.push(quote! {
                #(#keep_attrs)*
                #[allow(non_snake_case)]
                #vis fn #name() -> Result<#ty> {
                    Self::__class_init()?;
                    Ok(#cell.with(|c| ::std::clone::Clone::clone(&*c.borrow())).unwrap_or_default())
                }
            });
        }
        if !impl_methods.contains(&setter.to_string()) {
            members.push(quote! {
                #[allow(non_snake_case)]
                #vis fn #setter(v: #ty) -> Result<()> {
                    Self::__class_init()?;
                    #cell.with(|c| *c.borrow_mut() = ::std::option::Option::Some(v));
                    Ok(())
                }
            });
        }
    }
    (storage, members)
}

/// 生成 `__class_init()`。返回 (模块级状态项, impl 块成员)。
///
/// 状态：0 = 未初始化；1 = 初始化中 / 已完成（同线程递归与后续访问都立即返回）；
///       2 = erroneous（`<clinit>` 曾抛出异常）。
pub(crate) fn expand_class_init(
    struct_ident: &Ident,
    binary_name: &str,
    superclass: Option<&Type>,
    has_clinit: bool,
) -> (TokenStream2, TokenStream2) {
    let state = format_ident!("__CLINIT_STATE_{}", struct_ident);
    let storage = quote! {
        ::std::thread_local! {
            #[allow(non_upper_case_globals)]
            static #state: ::std::cell::Cell<u8> = const { ::std::cell::Cell::new(0) };
        }
    };
    let init_super = superclass.map(|sup| quote! { <#sup>::__class_init()?; });
    let clinit_ident = format_ident!("{}", CLINIT_FN);
    let run_clinit = if has_clinit {
        quote! { Self::#clinit_ident() }
    } else {
        quote! { Ok(()) }
    };
    let member = quote! {
        #[doc(hidden)]
        pub fn __class_init() -> Result<()> {
            match #state.with(|s| s.replace(1)) {
                0 => {}
                2 => {
                    #state.with(|s| s.set(2));
                    return Err(JvmError::no_class_def_found(#binary_name));
                }
                _ => return Ok(()),
            }
            let run = || -> Result<()> {
                #init_super
                #run_clinit
            };
            match run() {
                Ok(()) => Ok(()),
                Err(e) => {
                    #state.with(|s| s.set(2));
                    Err(JvmError::in_initializer(e))
                }
            }
        }
    };
    (storage, member)
}

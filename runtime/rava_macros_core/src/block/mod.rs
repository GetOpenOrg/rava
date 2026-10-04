//! `java_class!` 块级宏实现 — vtable 两指针架构。
//!
//! 展开产物（生成逻辑按 Java 语义切分在 `gen/` 各模块，擦除辅助在 `erasure.rs`，
//! 接口载体在 `interface.rs`）：
//!   - `ClassName__VTable` trait（虚方法分派接口，含 default impl）
//!   - `ClassName__inner` 存储 struct（平铺字段，无 `_super` 嵌套）
//!   - `impl AncestorVTable for ClassName__inner`（字段访问器 + 覆盖方法）
//!   - `pub struct ClassName { vtable: Rc<dyn ClassName__VTable>, any: Rc<dyn Any> }`
//!   - `impl From<ClassName> for Object`（Object 直接持有存储，S7-2b）
//!   - 字段访问器委托 + 虚方法委托 + 构造器（on wrapper）
//!   - `ClassName__methodName_base` 自由函数（super() 调用路由）
//!   - `From<ClassName> for DirectParent`（vtable trait upcasting）
//!   - `From<Object> for ClassName`（downcast 路径）
//!
//! ## virtual_in 属性
//!
//! codegen 在 `#[java_method(virtual_in = "RustClassName")]` 中携带：
//!   - 等于当前类名 → VirtualDefine（新虚方法，进 trait default impl）
//!   - 不等于当前类名 → VirtualOverride（覆盖祖先，进 `impl AncestorVTable for __inner`）
//!   - 缺失 → Constructor（`new`/`new_*` 前缀）或 NonVirtual

mod class_init;
mod classify;
mod desc_types;
mod erasure;
mod gen;
mod generic_sig;
mod interface;
mod moved;
#[cfg(feature = "plan")]
pub mod plan;
mod parse;
mod rewrite;
mod util;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use gen::GenContext;
/// 类型参数约束补齐（`iface_upcasts!` 与类路径同源复用）
pub use gen::context::augment_generic_bounds;
use gen::layer::{place_fns, Layer};
use interface::expand_interface;
use parse::ClassInput;


pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<ClassInput>(input) {
        Ok(v) => expand_inner(v),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(input: ClassInput) -> TokenStream2 {
    match expand_class(&input) {
        Ok(ts) => ts,
        Err(e) => e.to_compile_error(),
    }
}

/// 类展开的调用序列：接口走载体路径；类经 GenContext 分派到 gen/ 各生成模块，
/// 最终按 §1-§11 的原始顺序拼装（token 流与拆分前逐字节一致）。
fn expand_class(input: &ClassInput) -> syn::Result<TokenStream2> {
    let meta = parse::ClassMeta::from_attrs(&input.attrs)?;
    // M3-c 纯位类型一致性断言（常开，失配 → compile_error）
    desc_types::audit_class(&meta.binary_name, &input.struct_ident.to_string(), &input.fns)
        .map_err(|msg| syn::Error::new(input.struct_ident.span(), msg))?;

    // ── 泛型参数补齐 Clone + Default + 'static + From<Object> + Into<Object> ──────────────────────────────
    let generics = gen::context::augment_generic_bounds(&input.generics);

    // 剥体标注只出现在非接口类的声明层（§7.5.2）
    if let Some(f) = input.fns.iter().find(|f| f.moved.is_some()) {
        if meta.layer != Layer::Decl || meta.is_interface {
            return Err(syn::Error::new_spanned(&f.sig.ident, "rava_moved 只用于非接口类的声明层"));
        }
    }

    // ── 接口：同名载体类型（接口引用 + 静态成员）────────────────────────────
    if meta.is_interface {
        // 接口载体整体属声明层（default / static 方法体进实现层是后续项，见拆 crate 文档 §7.5）
        if meta.layer == Layer::Body {
            return Ok(TokenStream2::new());
        }
        return Ok(expand_interface(
            &meta, &input.struct_ident, &generics, &input.fns, &input.statics,
        ));
    }

    let ctx = GenContext::build(input, meta, generics);

    let layer = ctx.meta.layer;
    let binary_name = ctx.meta.binary_name.clone();
    // 存储层与 vtable impl 只进实现层：声明模式不计算（已下沉的体不在声明层文本里）
    let (layout, dispatch_impls) = if layer == Layer::Decl {
        (TokenStream2::new(), TokenStream2::new())
    } else {
        (gen::struct_layout::generate(&ctx), gen::virtual_dispatch::vtable_impls(&ctx)?)
    };
    let hooks = gen::storage_hooks::generate(&ctx);
    let dispatch_trait = gen::virtual_dispatch::vtable_trait(&ctx);
    let (wrapper, body_fns) = gen::wrapper::generate(&ctx)?;
    let (conversions, inner_consts) = gen::type_conversions::generate(&ctx);
    let base_fns = gen::virtual_dispatch::base_fns(&ctx);
    // 非泛型 base 函数随存储层拆入实现层；泛型者（类 / 方法泛型）与存根留在声明层
    let (generic_base, split_base): (Vec<_>, Vec<_>) = base_fns.into_iter().partition(|b| b.decl_only);
    let generic_base: Vec<TokenStream2> = generic_base.into_iter().map(|b| b.item).collect();
    let split_base: Vec<TokenStream2> = split_base.into_iter().map(|b| b.item).collect();

    let hooks = place_fns(layer, &binary_name, &hooks)?;
    let body_fns = place_fns(layer, &binary_name, &body_fns)?;
    let split_base = place_fns(layer, &binary_name, &split_base)?;
    Ok(match layer {
        Layer::Full => quote! {
            #dispatch_trait
            #layout
            #hooks
            #dispatch_impls
            #wrapper
            #body_fns
            #conversions
            #inner_consts
            #(#generic_base)*
            #split_base
        },
        Layer::Decl => {
            let digest = moved::decl_digest(&ctx);
            quote! {
            #digest
            #dispatch_trait
            #hooks
            #wrapper
            #body_fns
            #conversions
            #(#generic_base)*
            #split_base
            }
        }
        Layer::Body => {
            let check = moved::body_assert(&ctx);
            quote! {
            #check
            #layout
            #hooks
            #dispatch_impls
            #body_fns
            #inner_consts
            #split_base
            }
        }
    })
}

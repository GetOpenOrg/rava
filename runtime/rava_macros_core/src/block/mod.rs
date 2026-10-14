//! `java_class!` 块级宏实现 — vtable 两指针架构。
//!
//! 展开产物（生成逻辑按 Java 语义切分在 `gen/` 各模块，擦除辅助在 `erasure.rs`，
//! 接口载体在 `interface.rs`）：
//!   - `ClassName__VTable` trait（虚方法分派接口，含 default impl）
//!   - `ClassName__inner` 存储 struct（平铺字段，无 `_super` 嵌套）
//!   - `impl AncestorVTable for ClassName__inner`（字段访问器 + 覆盖方法）
//!   - `pub struct ClassName { __r: __Handle }`（对象句柄，S7-2；视图经 `__vt()` 现取，§9.9 K1）
//!   - 接口：载体 `pub struct Iface { __ref: __IfaceRef }`（只持 Object 句柄，S7-2c；视图经 `enter` 现取，§9.9 K2）
//!   - `impl From<ClassName> for Object`（Object 直接持有存储，S7-2b）
//!   - 字段访问器委托 + 虚方法委托 + 构造器（on wrapper）
//!   - `ClassName__methodName_base` 自由函数（super() 调用路由）
//!   - `From<ClassName> for DirectParent`（上转只搬句柄）
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
mod ldc;
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
        Ok(mut v) => {
            ldc::cache_literals(&mut v);
            expand_inner(v)
        }
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

    // 剥体标注只出现在非接口类的声明层（§7.5.2）；标记层（§9.9 K6）复用同一份剥体后的块文本，
    // 标注原样带入（标记层不展开方法面，标注不生效）
    if let Some(f) = input.fns.iter().find(|f| f.moved.is_some()) {
        if !matches!(meta.layer, Layer::Decl | Layer::Marker) || meta.is_interface {
            return Err(syn::Error::new_spanned(&f.sig.ident, "rava_moved 只用于非接口类的声明层"));
        }
    }

    // ── 接口：同名载体类型（接口引用 + 静态成员）────────────────────────────
    if meta.is_interface {
        // 接口载体拆层（§9.9 K4）：标记层展开载体身份面（struct + 身份 impl，随标记文件入
        // `java_marker` crate）；`I__VTable` / `I__Methods` / statics 留声明层；实现层为空
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
    // 存储层与 vtable impl 只进实现层：声明 / 标记模式不计算（已下沉的体不在声明层文本里）
    let (layout, dispatch_impls) = if layer == Layer::Decl || layer == Layer::Marker {
        (TokenStream2::new(), TokenStream2::new())
    } else {
        (gen::struct_layout::generate(&ctx), gen::virtual_dispatch::vtable_impls(&ctx)?)
    };
    // 标记层（§9.9 K6）不展开方法面与分派：只算标记部分、转换与 alloc / offsets 钩子声明。
    // 实现层要方法面生成的体函数 / base 函数（定义），声明层要 vtable trait 与外壳，两者都要算
    let need_marker = matches!(layer, Layer::Full | Layer::Marker);
    let need_trait = matches!(layer, Layer::Full | Layer::Decl);
    let need_methods = matches!(layer, Layer::Full | Layer::Decl | Layer::Body);
    let hooks = gen::storage_hooks::generate(&ctx);
    let (dispatch_trait, base_fns): (TokenStream2, Vec<_>) = if need_methods {
        (
            if need_trait { gen::virtual_dispatch::vtable_trait(&ctx) } else { TokenStream2::new() },
            gen::virtual_dispatch::base_fns(&ctx),
        )
    } else {
        (TokenStream2::new(), Vec::new())
    };
    let wrapper_marker = if need_marker { gen::wrapper::generate_marker(&ctx) } else { TokenStream2::new() };
    let (wrapper_decl, body_fns): (TokenStream2, Vec<TokenStream2>) = if need_methods {
        let (d, b) = gen::wrapper::generate_decl(&ctx)?;
        (if need_trait { d } else { TokenStream2::new() }, b)
    } else {
        (TokenStream2::new(), Vec::new())
    };
    // 标记层（§9.9 K6）：转换（BINARY_NAME / 描述符 / From 三族）整体属标记面
    let (conversions, inner_consts) = gen::type_conversions::generate(&ctx);
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
            #wrapper_marker
            #wrapper_decl
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
            #wrapper_decl
            #body_fns
            #(#generic_base)*
            #split_base
            }
        }
        Layer::Marker => {
            // hooks 即 alloc / offsets 的外部声明块（place_fns 对标记层同声明层），
            // 供描述符 `X__DESC` 的分配 / 偏移闭包引用
            quote! {
            #wrapper_marker
            #conversions
            #hooks
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

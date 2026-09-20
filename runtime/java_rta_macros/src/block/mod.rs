//! `java_class!` 块级宏实现 — vtable 两指针架构。
//!
//! 展开产物（生成逻辑按 Java 语义切分在 `gen/` 各模块，擦除辅助在 `erasure.rs`，
//! 接口载体在 `interface.rs`）：
//!   - `ClassName__VTable` trait（虚方法分派接口，含 default impl）
//!   - `ClassName__inner` 存储 struct（平铺字段，无 `_super` 嵌套）
//!   - `impl AncestorVTable for ClassName__inner`（字段访问器 + 覆盖方法）
//!   - `pub struct ClassName { vtable: Rc<dyn ClassName__VTable>, any: Rc<dyn Any> }`
//!   - `impl ObjectVTable for ClassName`（委托到 vtable，R-1 blanket 需要）
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
mod erasure;
mod gen;
mod generic_sig;
mod interface;
mod parse;
mod rewrite;
mod util;

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use gen::GenContext;
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

    // ── 泛型参数补齐 Clone + Default + 'static + From<Object> + Into<Object> ──────────────────────────────
    let generics = gen::context::augment_generic_bounds(&input.generics);

    // ── 接口：同名载体类型（接口引用 + 静态成员）────────────────────────────
    if meta.is_interface {
        return Ok(expand_interface(
            &meta, &input.struct_ident, &generics, &input.fns, &input.statics,
        ));
    }

    let ctx = GenContext::build(input, meta, generics);

    let layout = gen::struct_layout::generate(&ctx);
    let dispatch_trait = gen::virtual_dispatch::vtable_trait(&ctx);
    let dispatch_impls = gen::virtual_dispatch::vtable_impls(&ctx)?;
    let wrapper = gen::wrapper::generate(&ctx)?;
    let conversions = gen::type_conversions::generate(&ctx);
    let base_fns = gen::virtual_dispatch::base_fns(&ctx);

    Ok(quote! {
        #dispatch_trait
        #layout
        #dispatch_impls
        #wrapper
        #conversions
        #base_fns
    })
}

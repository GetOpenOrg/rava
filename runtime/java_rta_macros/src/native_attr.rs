//! `#[jvm_native]`：手写 ACC_NATIVE 方法的标记属性 + JVMS §5.5 类初始化触发点。
//!
//! invokestatic 是类初始化触发点（JVMS §5.5）。java_class! 宏为块内静态方法 / 构造器
//! 入口注入 `Self::__class_init()?;`（block/class_init.rs）；手写 native 实现在块外的
//! `*_impl.rs` 伴生 `impl X { … }` 中，此前没有该触发点（S-10 子缺口 a）。本属性对**无
//! 接收者且返回 `Result` 的方法**在入口注入同一语句——手写调用方与生成调用方经同一入口
//! 触发，与 JVM 在被调方法侧完成初始化的语义一致。`__class_init` 的状态机对「初始化中」
//! 立即返回，`<clinit>` 内调用本类 native（registerNatives）不会递归。
//!
//! 参数：`upcalls = "…"`（native_upcalls.py 消费，宏忽略）；`no_class_init`——宿主类型
//! 不是 java_class! 生成类（无 `__class_init`，如手写根类 Object）时显式豁免。

use proc_macro2::TokenStream;
use quote::quote;

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    if attr.to_string().contains("no_class_init") {
        return item;
    }
    let mut f: syn::ImplItemFn = match syn::parse2(item.clone()) {
        Ok(f) => f,
        Err(_) => return item,
    };
    if f.sig.receiver().is_some() || !returns_result(&f.sig.output) {
        return quote! { #f };
    }
    f.block.stmts.insert(0, syn::parse_quote! { Self::__class_init()?; });
    quote! { #f }
}

fn returns_result(out: &syn::ReturnType) -> bool {
    match out {
        syn::ReturnType::Type(_, ty) => match &**ty {
            syn::Type::Path(tp) => tp.path.segments.last().map(|s| s.ident == "Result").unwrap_or(false),
            _ => false,
        },
        syn::ReturnType::Default => false,
    }
}

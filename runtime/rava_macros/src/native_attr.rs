//! `#[jvm_native]`：手写 ACC_NATIVE 方法的标记属性 + JVMS §5.5 类初始化触发点。
//!
//! invokestatic 是类初始化触发点（JVMS §5.5）。java_class! 宏为块内静态方法 / 构造器
//! 入口注入 `Self::__class_init()?;`（block/class_init.rs）；手写 native 实现在块外的
//! `*_impl.rs` 伴生 `impl X { … }` 中，此前没有该触发点（S-10 子缺口 a）。本属性对**无
//! 接收者且返回 `Result` 的方法**在入口注入同一语句——手写调用方与生成调用方经同一入口
//! 触发，与 JVM 在被调方法侧完成初始化的语义一致。`__class_init` 的状态机对「初始化中」
//! 立即返回，`<clinit>` 内调用本类 native（registerNatives）不会递归。
//!
//! 栈上 native 帧（a3-T2，计划 §21.8.3）：方法体包一个 `exec_context::NativeFrame` 守卫，进出时当前执行流的
//! native 计数 ±1——虚拟线程在 native 方法回调的 Java 代码里让出时判定为 NATIVE pinned（HotSpot 冻结遇到 native
//! 帧同义）。只计数，无其它开销。
//!
//! 参数（逗号分隔，可组合）：
//! - `no_class_init`——宿主类型不是 java_class! 生成类（无 `__class_init`，如手写根类 Object）时显式豁免；
//! - `unpinned`——不计入 native 帧：`Continuation` 自身的 VM 入口（`doYield` / `enterSpecial` / `pin` / `unpin` /
//!   `isPinned0`），HotSpot 冻结与 `is_pinned0` 判定同样跳过这些帧。

use proc_macro2::TokenStream;
use quote::quote;

pub fn expand(attr: TokenStream, item: TokenStream) -> TokenStream {
    let flags: Vec<std::string::String> = attr
        .to_string()
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let class_init = !flags.iter().any(|f| f == "no_class_init");
    let pinned = !flags.iter().any(|f| f == "unpinned");
    let mut f: syn::ImplItemFn = match syn::parse2(item.clone()) {
        Ok(f) => f,
        Err(_) => return item,
    };
    if class_init && f.sig.receiver().is_none() && returns_result(&f.sig.output) {
        f.block.stmts.insert(0, syn::parse_quote! { Self::__class_init()?; });
    }
    if pinned {
        // 守卫先于类初始化触发点：`<clinit>` 经本 native 触发时同样处在 native 帧内
        f.block.stmts.insert(0, syn::parse_quote! {
            let __native_frame = crate::exec_context::NativeFrame::enter();
        });
    }
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

use proc_macro::TokenStream;

mod block;

/// `java_class! { ... }` — 块级宏（方案 2026-09-16 的 `java_class!` 块级宏统一方案）。
///
/// 同时持有 struct 字段定义与 impl 块方法体，在一个宏调用里完成全部展开：
///   - Inner struct（flat layout）+ newtype 包装
///   - 字段访问器（基本类型 Cell / 引用类型 RefCell，borrow 窗口最小化）
///   - 方法体 token 重写（self.field → 访问器调用）
///   - JavaObject / ObjectVTable / Upcast / native 存根生成
#[proc_macro]
pub fn java_class(input: TokenStream) -> TokenStream {
    block::expand(input.into()).into()
}

/// 标记该方法实现了 Java 字节码中的 `ACC_NATIVE` 方法。
#[proc_macro_attribute]
pub fn jvm_native(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// 标记该方法属于内部边界类（`jdk/internal/`、`sun/`），BFS 截断后整体手写。
#[proc_macro_attribute]
pub fn jvm_boundary(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// 标记该方法是 Rust 侧人机工程学扩展，Java 规范中不存在。
#[proc_macro_attribute]
pub fn jvm_ext(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// `#[java_rta_macros::java_method(name = "...", descriptor = "...", ...)]`
///
/// 携带 Java 字节码方法元数据（名称、描述符、访问标志等），供宏和工具链读取。
/// 当前行为：透传（identity passthrough），未来扩展 SAM 检测、泛型签名校验等。
#[proc_macro_attribute]
pub fn java_method(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

/// `#[java_rta_macros::java_native(name = "...", descriptor = "...", ...)]`
///
/// 携带 `ACC_NATIVE` 方法的 Java 字节码元数据。当前行为：透传。
#[proc_macro_attribute]
pub fn java_native(_attr: TokenStream, item: TokenStream) -> TokenStream { item }

use proc_macro::TokenStream;

mod block;
mod enum_macro;
mod switch_macro;
mod synchronized;

/// `java_class! { ... }` — 块级宏，封装单个 Java 类的全部 Rust 复杂度。
#[proc_macro]
pub fn java_class(input: TokenStream) -> TokenStream {
    block::expand(input.into()).into()
}

/// `java_enum! { ... }` — 块级宏，封装 Java `enum`。
/// 自动生成 ordinal()/name()/values()/valueOf() + ObjectVTable impl。
#[proc_macro]
pub fn java_enum(input: TokenStream) -> TokenStream {
    enum_macro::expand(input.into()).into()
}

/// `java_switch! { expr; arm => body, ... }` — 封装 Java switch 语义。
/// 支持整数/enum（直接 match）、fallthrough（if-chain + __fall 标志）、String（equals 链）三种模式。
#[proc_macro]
pub fn java_switch(input: TokenStream) -> TokenStream {
    switch_macro::expand(input.into()).into()
}

/// `#[java_synchronized]` — 封装 Java `synchronized` 方法，函数级静态 Mutex 保证互斥。
#[proc_macro_attribute]
pub fn java_synchronized(attr: TokenStream, item: TokenStream) -> TokenStream {
    synchronized::expand(attr.into(), item.into()).into()
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

// 注：`java_method` / `java_native` 属性宏已删除。
// 块内 `#[java_method(...)]` / `#[java_native(...)]` 只是文本标签：
// block.rs 在展开时将其剥离，build.rs 按文本前缀扫描维护 native_status.toml，
// 二者均不依赖同名 proc-macro 的存在。

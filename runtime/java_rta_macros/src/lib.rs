use proc_macro::TokenStream;

mod block;
mod native_attr;
mod try_macro;

/// `java_class! { ... }` — 块级宏，封装单个 Java 类的全部 Rust 复杂度。
///
/// Java enum 在字节码层面就是普通类（继承 `java/lang/Enum`，常量为 static final
/// 字段），同样经此宏翻译；不再有独立的 enum 表示路径。
#[proc_macro]
pub fn java_class(input: TokenStream) -> TokenStream {
    block::expand(input.into()).into()
}

/// `java_try! { try { ... } catch (e: T) { ... } }` — 封装 Java try/catch：
/// 按异常对象的运行时类（含子类）匹配 catch 子句，未匹配则继续向外传播。
#[proc_macro]
pub fn java_try(input: TokenStream) -> TokenStream {
    try_macro::expand(input.into()).into()
}


/// 标记该方法实现了 Java 字节码中的 `ACC_NATIVE` 方法；静态 native 入口注入类初始化
/// 触发点（JVMS §5.5，见 native_attr.rs）。
#[proc_macro_attribute]
pub fn jvm_native(attr: TokenStream, item: TokenStream) -> TokenStream {
    native_attr::expand(attr.into(), item.into()).into()
}

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

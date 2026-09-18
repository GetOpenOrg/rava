//! `#[java_synchronized]` 属性宏实现。
//!
//! 将 Java `synchronized` 方法语义映射到 Rust：进入方法时获取锁，离开时自动释放。
//!
//! ## 当前阶段（过渡方案）
//!
//! vtable 架构尚未为所有生成 struct 提供 `__monitor` 字段，
//! 因此当前用函数级静态 `Mutex<()>` 模拟方法粒度的 monitor 锁。
//! 正确的对象级 monitor 需要 `java_class!` 在 `__inner` 上添加
//! `__monitor: Mutex<()>` 字段后再切换。
//!
//! ## 展开形式
//!
//! 输入：
//! ```ignore
//! #[java_synchronized]
//! pub fn increment(&self) -> Result<()> {
//!     self.count += 1;
//!     Ok(())
//! }
//! ```
//!
//! 展开后（实例方法与静态方法相同，均注入函数级静态锁）：
//! ```ignore
//! pub fn increment(&self) -> Result<()> {
//!     static __MONITOR: ::std::sync::Mutex<()> = ::std::sync::Mutex::new(());
//!     let _guard = __MONITOR.lock().map_err(|_| {
//!         java_runtime::error::JvmError::illegal_monitor_state("monitor lock poisoned")
//!     })?;
//!     self.count += 1;
//!     Ok(())
//! }
//! ```

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::ItemFn;

pub fn expand(_attr: TokenStream2, item: TokenStream2) -> TokenStream2 {
    let mut func: ItemFn = match syn::parse2(item) {
        Ok(f) => f,
        Err(e) => return e.to_compile_error(),
    };

    // 静态 Mutex：函数级作用域，程序生命周期内唯一初始化。
    // 所有对同一方法的调用共享此锁，实现方法粒度的互斥访问。
    // _guard 离开作用域时自动解锁（RAII），无论是否 panic 或提前 return。
    let stmt_static: syn::Stmt = syn::parse_quote! {
        static __MONITOR: ::std::sync::Mutex<()> = ::std::sync::Mutex::new(());
    };

    // ? 传播：若 Mutex 被 poison（持有线程 panic），转换为 JvmError 向上传递
    let stmt_lock: syn::Stmt = syn::parse_quote! {
        let _guard = __MONITOR.lock().map_err(|_| {
            java_runtime::error::JvmError::illegal_monitor_state("monitor lock poisoned")
        })?;
    };

    let mut new_stmts = vec![stmt_static, stmt_lock];
    new_stmts.extend(func.block.stmts.drain(..));
    func.block.stmts = new_stmts;

    quote! { #func }
}

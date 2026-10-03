//! 按 `target_arch` 选择切换汇编；其余平台编译期报错，不提供退化实现。

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
compile_error!("rava_coro 只支持 aarch64 与 x86_64（虚拟线程的有栈协程切换无退化实现）");

#[cfg(not(unix))]
compile_error!("rava_coro 只支持 unix（mmap 栈与 guard page）");

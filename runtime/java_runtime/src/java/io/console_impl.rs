//! `java/io/Console` 的 native 方法（与生成的 console.rs 共置）。
//!
//! 对标 HotSpot `Console_md.c`（Unix）：`istty` 为 stdin 与 stdout 均是终端；
//! `encoding` 在 Unix 恒返回 null（由 Java 侧回落 stdout.encoding / 本地编码）。

use crate::prelude::*;
use super::console::Console;
use crate::java::lang::String;

impl Console {
    /// `istty()`：stdin 与 stdout 均连接终端（`isatty(0) && isatty(1)`）。
    #[jvm_native]
    pub fn istty() -> Result<bool> {
        // SAFETY: isatty 只查询文件描述符属性，无内存副作用
        Ok(unsafe { libc::isatty(0) == 1 && libc::isatty(1) == 1 })
    }

    /// `encoding()`：Unix 实现恒返回 null。
    #[jvm_native]
    pub fn encoding() -> Result<String> {
        Ok(Default::default())
    }
}

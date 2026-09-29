//! `java/io/Console` 的 native 方法（与生成的 console.rs 共置）。
//!
//! 对标 HotSpot `Console_md.c`（Unix）：`istty` 为 stdin 与 stdout 均是终端；
//! `ttyStatus`（JDK 21 后续更新版本起取代 istty）按位返回 stdin / stdout / stderr 的终端状态；
//! `encoding` 在 Unix 恒返回 null（由 Java 侧回落 stdout.encoding / 本地编码）。
//! 两个 native 并存：不同 JDK 21 更新版本的 `Console.<clinit>` 分别调用其一。

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

    /// `ttyStatus()`：TTY_STDIN_MASK(1) | TTY_STDOUT_MASK(2) | TTY_STDERR_MASK(4)。
    #[jvm_native]
    pub fn ttyStatus() -> Result<i32> {
        // SAFETY: isatty 只查询文件描述符属性，无内存副作用
        let tty = |fd: i32| unsafe { libc::isatty(fd) == 1 };
        Ok((tty(0) as i32) | ((tty(1) as i32) << 1) | ((tty(2) as i32) << 2))
    }
}

//! `sun/nio/ch/IOUtil` 的 ACC_NATIVE（类 1，libnio `IOUtil.c`）。其余方法按字节码翻译。

use crate::prelude::*;
use super::io_util::IOUtil;
use crate::java::io::FileDescriptor;

impl IOUtil {
    /// native `initIDs()`：HotSpot 缓存 FileDescriptor.fd 的 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native `iovMax()`：`sysconf(_SC_IOV_MAX)`，不可得时 16（JNI 同款）。
    #[jvm_native]
    pub fn iovMax() -> Result<i32> {
        // SAFETY: sysconf 无副作用
        let v = unsafe { libc::sysconf(libc::_SC_IOV_MAX) };
        Ok(if v == -1 { 16 } else { v as i32 })
    }

    /// native `writevMax()`：Linux / macOS 为 INT_MAX（JNI 同款）。
    #[jvm_native]
    pub fn writevMax() -> Result<i64> {
        Ok(i32::MAX as i64)
    }

    /// native `fdVal(FileDescriptor)`：读取 fd 字段。
    #[jvm_native]
    pub fn fdVal(fdo: FileDescriptor) -> Result<i32> {
        Ok(fdo.__get_fd())
    }
}

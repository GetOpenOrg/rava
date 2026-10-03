//! `sun/nio/fs/UnixFileSystem` 手写伴生：仅 ACC_NATIVE 方法（其余由字节码翻译）。

use crate::prelude::*;

impl super::unix_file_system::UnixFileSystem {
    /// native `bufferedCopy0(int dst, int src, long address, int size, long addressToPollForCancel)`：
    /// 对标 JDK 21 `UnixFileSystem.c`——以 `address` 处 `size` 字节直接内存为缓冲循环
    /// read(src) / write(dst)（均 EINTR 重试，短写续写）；每读到一批先查取消字
    /// （`addressToPollForCancel` 非 0 时指向 int，非 0 即取消，抛 ECANCELED）。读到 EOF 返回。
    #[jvm_native]
    pub fn bufferedCopy0(dst: i32, src: i32, address: i64, size: i32, cancel: i64) -> Result<()> {
        use super::unix_native_dispatcher_impl::{restartable, unix_exception};
        let buf = address as *mut libc::c_void;
        loop {
            // SAFETY: address 指向至少 size 字节的直接缓冲（Java 侧 NativeBuffer 分配）
            let n = restartable(|| unsafe { libc::read(src, buf, size as usize) } as i32).map_err(unix_exception)?;
            if n == 0 {
                return Ok(());
            }
            // SAFETY: cancel 非 0 时指向 Cancellable 的轮询 int
            if cancel != 0 && unsafe { std::ptr::read_volatile(cancel as *const i32) } != 0 {
                return Err(unix_exception(libc::ECANCELED));
            }
            let (mut pos, mut len) = (0usize, n as usize);
            while len > 0 {
                // SAFETY: [pos, pos + len) 在本批读入的范围内
                let w = restartable(|| unsafe { libc::write(dst, (buf as *const u8).add(pos).cast(), len) } as i32)
                    .map_err(unix_exception)?;
                pos += w as usize;
                len -= w as usize;
            }
        }
    }
}

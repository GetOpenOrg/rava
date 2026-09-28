use crate::prelude::*;
use super::file_cleanable::FileCleanable;
use crate::jdk_resources;

// FileCleanable 的伴生手写：只剩 native `cleanupClose0`。register / unregister 走 JDK 字节码
// （FileDescriptor.registerCleanup + new FileCleanable(..)）；无 GC 下的 no-op 语义下沉到内部边界
// PhantomCleanable / CleanerFactory（jdk/internal/ref/，FS-G4）。
impl FileCleanable {
    /// static native cleanupClose0(int fd, long handle)：按 fd 关闭描述符
    ///（虚拟句柄注销光标 / OS fd close(2)；-1 幂等无操作）。与
    /// FileDescriptor.close0 同一资源协议；本运行时无清理线程，此入口
    /// 仅供伴生语义完整。
    #[jvm_native]
    pub fn cleanupClose0(fd: i32, _handle: i64) -> Result<()> {
        if fd <= -2 {
            jdk_resources::virtual_close(fd);
            return Ok(());
        }
        if fd >= 0 {
            use std::os::fd::FromRawFd;
            drop(unsafe { std::fs::File::from_raw_fd(fd) });
        }
        Ok(())
    }
}

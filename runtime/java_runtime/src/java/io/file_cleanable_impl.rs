use crate::prelude::*;
use super::file_cleanable::FileCleanable;
use crate::jdk_resources;

// FileCleanable 的伴生手写：FileDescriptor 的 phantom 清理注册。
//
// JDK 语义：FileCleanable.register 把 fd 包成 PhantomCleanable 挂到
// Common-Cleaner，FileDescriptor 幻象可达时由清理线程调 cleanupClose0
// 兜底关闭泄漏的描述符。本运行时无 GC/引用处理：对象经 Rc 常驻、
// phantom 引用永不入队、清理动作永不触发——注册的正确语义是无操作，
// 关闭的唯一路径是显式 close（FileDescriptor.close0，含虚拟句柄分派）。
// 对象图依赖（PhantomCleanable/CleanerImpl 工作线程）随之整体免装：
// 清理线程空转是纯开销，InnocuousThread 深链（AccessController/系统
// 线程组）无可观察行为。
impl FileCleanable {
    /// static register(FileDescriptor)：无操作（无 GC ⇒ 无 phantom 处理）。
    #[jvm_native]
    pub fn register(_fdo: super::file_descriptor::FileDescriptor) -> Result<()> {
        Ok(())
    }

    /// static unregister(FileDescriptor)：与 register 对称的无操作。
    #[jvm_native]
    pub fn unregister(_fdo: super::file_descriptor::FileDescriptor) -> Result<()> {
        Ok(())
    }

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

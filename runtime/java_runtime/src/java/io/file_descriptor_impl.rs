use crate::java::io::FileDescriptor__Methods as _;
use crate::java::io::IOException__Methods as _;
use crate::prelude::*;
use super::file_descriptor::FileDescriptor;

impl FileDescriptor {
    /// native initIDs：HotSpot 缓存 JNI 字段 ID；原生二进制无此需要。
    #[jvm_native]
    pub fn initIDs() -> Result<()> {
        Ok(())
    }

    /// native getHandle(int)：Unix 平台无 Windows HANDLE，恒为 -1。
    #[jvm_native]
    pub fn getHandle(_fd: i32) -> Result<i64> {
        Ok(-1)
    }

    /// native getAppend(int)：标准流不以 O_APPEND 打开。
    #[jvm_native]
    pub fn getAppend(_fd: i32) -> Result<bool> {
        Ok(false)
    }

    /// native close0()：关闭底层描述符。fd 符号分派（FileInputStream.open0
    /// 的资源协议）：负数虚拟句柄（≤ -2）注销嵌入资源光标；非负为 OS fd，
    /// 取回所有权并 drop（close(2)）。-1 为已关闭的无效值，幂等无操作。
    #[jvm_native]
    pub fn close0(&self) -> Result<()> {
        let fd = self.__get_fd();
        if fd <= -2 {
            crate::jdk_resources::virtual_close(fd);
            self.__set_fd(-1i32);
            return Ok(());
        }
        if fd >= 0 {
            use std::os::fd::FromRawFd;
            // 取回 open0 转入所有权的描述符；drop 即 close(2)
            drop(unsafe { std::fs::File::from_raw_fd(fd) });
            self.__set_fd(-1i32);
        }
        Ok(())
    }

    /// native sync0()：强制落盘（fsync）。失败抛 `SyncFailedException("sync failed")`
    /// （FileDescriptor_md.c 同：无效 fd 的 fsync 同样失败，消息不带 errno 文本）。
    #[jvm_native]
    pub fn sync0(&self) -> Result<()> {
        let fd = self.__get_fd();
        if fd <= -2 {
            // 嵌入资源位于二进制 .rodata，无落盘语义
            return Ok(());
        }
        use std::os::fd::FromRawFd;
        let ok = fd >= 0 && {
            let f = std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) });
            f.sync_all().is_ok()
        };
        if ok {
            return Ok(());
        }
        Err(JvmError::from(super::SyncFailedException::new(String::from("sync failed"))?))
    }
}

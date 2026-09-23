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

    /// native sync0()：强制落盘（fsync）。失败抛 IOException（JDK 精确类型为
    /// SyncFailedException——IOException 子类，仅当语料引用时才生成，按捕获
    /// 语义等价降型为 IOException，消息保留 "sync failed" 前缀）。
    #[jvm_native(upcalls = "java/io/IOException.<init>:(Ljava/lang/String;)V")]
    pub fn sync0(&self) -> Result<()> {
        let fd = self.__get_fd();
        if fd <= -2 {
            // 嵌入资源位于二进制 .rodata，无落盘语义
            return Ok(());
        }
        if fd < 0 {
            return Err(JvmError::from(super::IOException::new_str(
                String::from("sync failed"))?));
        }
        use std::os::fd::FromRawFd;
        let f = std::mem::ManuallyDrop::new(unsafe { std::fs::File::from_raw_fd(fd) });
        f.sync_all().map_err(|e| {
            JvmError::from(super::IOException::new_str(
                String::from(format!("sync failed: {}", e))).unwrap())
        })
    }
}

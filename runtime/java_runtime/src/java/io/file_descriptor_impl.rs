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
}

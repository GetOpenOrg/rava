//! Unsafe 的直接内存 native（malloc 承载）（宿主 unsafe__impl.rs 的私有辅助模块）

use super::*;

// ── 直接内存（malloc 承载；寻址约定见 crate::native_memory）─────────────────
// 只有 ACC_NATIVE 的 allocateMemory0 族在此手写（类 1）；公开包装（allocateMemory / setMemory /
// copyMemory / copySwapMemory / getByte(long) …）的参数检查、对齐与零长度短路按 JDK 字节码翻译（a3-U1）。

impl Unsafe {
    #[jvm_native]
    pub fn allocateMemory0(&self, bytes: i64) -> Result<i64> {
        Ok(crate::native_memory::allocate(bytes))
    }

    #[jvm_native]
    pub fn reallocateMemory0(&self, address: i64, bytes: i64) -> Result<i64> {
        Ok(crate::native_memory::reallocate(address, bytes))
    }

    #[jvm_native]
    pub fn freeMemory0(&self, address: i64) -> Result<()> {
        crate::native_memory::free(address);
        Ok(())
    }

    #[jvm_native]
    pub fn setMemory0(&self, o: Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
        crate::native_memory::fill(&o, offset, bytes, value)
    }

    #[jvm_native]
    pub fn copyMemory0(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64, bytes: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, 1)
    }

    #[jvm_native]
    pub fn copySwapMemory0(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                           bytes: i64, elem_size: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, elem_size as usize)
    }
}

//! Unsafe 的直接内存访问（malloc 承载）（宿主 unsafe__impl.rs 的私有辅助模块）

use super::*;

// ── 直接内存（malloc 承载；寻址约定见 crate::native_memory）─────────────────
// native（allocateMemory0 族）为 VM 契约；公开包装（allocateMemory / setMemory / copyMemory …）
// 的参数检查与零长度短路照 JDK 字节码语义手写（jdk/internal/misc 前缀截断期的过渡实现）。

/// JDK `alignToHeapWordSize`：按 8 字节向上取整。
fn _align_to_heap_word(bytes: i64) -> i64 {
    if bytes >= 0 { bytes.wrapping_add(7) & !7 } else { bytes }
}

fn _check_size(bytes: i64) -> Result<()> {
    if bytes < 0 {
        return Err(JvmError::illegal_argument("negative size"));
    }
    Ok(())
}

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

    /// `allocateMemory(long)`：按堆字宽对齐；0 字节返回 0；分配失败抛 OutOfMemoryError。
    #[jvm_boundary]
    pub fn allocateMemory(&self, bytes: i64) -> Result<i64> {
        let bytes = _align_to_heap_word(bytes);
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(0);
        }
        let p = self.allocateMemory0(bytes)?;
        if p == 0 {
            return Err(JvmError::out_of_memory(&format!("Unable to allocate {} bytes", bytes)));
        }
        Ok(p)
    }

    /// `reallocateMemory(long, long)`：0 字节时释放并返回 0。
    #[jvm_boundary]
    pub fn reallocateMemory(&self, address: i64, bytes: i64) -> Result<i64> {
        let bytes = _align_to_heap_word(bytes);
        _check_size(bytes)?;
        if bytes == 0 {
            self.freeMemory(address)?;
            return Ok(0);
        }
        let p = if address == 0 { self.allocateMemory0(bytes)? } else { self.reallocateMemory0(address, bytes)? };
        if p == 0 {
            return Err(JvmError::out_of_memory(&format!("Unable to allocate {} bytes", bytes)));
        }
        Ok(p)
    }

    #[jvm_boundary]
    pub fn freeMemory(&self, address: i64) -> Result<()> {
        if address == 0 {
            return Ok(());
        }
        self.freeMemory0(address)
    }

    #[jvm_boundary]
    pub fn setMemory_obj_l_l_b(&self, o: Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(());
        }
        self.setMemory0(o, offset, bytes, value)
    }

    #[jvm_boundary]
    pub fn setMemory_l_l_b(&self, address: i64, bytes: i64, value: i8) -> Result<()> {
        self.setMemory_obj_l_l_b(Object::default(), address, bytes, value)
    }

    #[jvm_boundary]
    pub fn copyMemory_obj_l_obj_l_l(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64, bytes: i64) -> Result<()> {
        _check_size(bytes)?;
        if bytes == 0 {
            return Ok(());
        }
        self.copyMemory0(src_base, src_offset, dst_base, dst_offset, bytes)
    }

    #[jvm_boundary]
    pub fn copyMemory_l_l_l(&self, src_address: i64, dst_address: i64, bytes: i64) -> Result<()> {
        self.copyMemory_obj_l_obj_l_l(Object::default(), src_address, Object::default(), dst_address, bytes)
    }

    /// `copySwapMemory(Object, long, Object, long, long, long elemSize)`：elemSize ∈ {2, 4, 8}，
    /// bytes 须为其整数倍（JDK copySwapMemoryChecks）。
    #[jvm_boundary]
    pub fn copySwapMemory_obj_l_obj_l_l_l(&self, src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                                          bytes: i64, elem_size: i64) -> Result<()> {
        _check_size(bytes)?;
        if !matches!(elem_size, 2 | 4 | 8) {
            return Err(JvmError::illegal_argument("Illegal element size"));
        }
        if bytes % elem_size != 0 {
            return Err(JvmError::illegal_argument("Size not a multiple of element size"));
        }
        if bytes == 0 {
            return Ok(());
        }
        self.copySwapMemory0(src_base, src_offset, dst_base, dst_offset, bytes, elem_size)
    }

    #[jvm_boundary]
    pub fn copySwapMemory_l_l_l_l(&self, src_address: i64, dst_address: i64, bytes: i64, elem_size: i64) -> Result<()> {
        self.copySwapMemory_obj_l_obj_l_l_l(Object::default(), src_address, Object::default(), dst_address, bytes, elem_size)
    }

    #[jvm_boundary]
    pub fn getByte_l(&self, address: i64) -> Result<i8> {
        self.getByte_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putByte_l_b(&self, address: i64, x: i8) -> Result<()> {
        self.putByte_obj_l_b(Object::default(), address, x)
    }

    #[jvm_boundary]
    pub fn getInt_l(&self, address: i64) -> Result<i32> {
        self.getInt_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putInt_l_i(&self, address: i64, x: i32) -> Result<()> {
        self.putInt_obj_l_i(Object::default(), address, x)
    }

    #[jvm_boundary]
    pub fn getLong_l(&self, address: i64) -> Result<i64> {
        self.getLong_obj_l(Object::default(), address)
    }

    #[jvm_boundary]
    pub fn putLong_l_l(&self, address: i64, x: i64) -> Result<()> {
        self.putLong_obj_l_l(Object::default(), address, x)
    }
}

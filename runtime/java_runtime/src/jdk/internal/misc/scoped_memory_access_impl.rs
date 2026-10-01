//! `jdk/internal/misc/ScopedMemoryAccess`（内部边界类，按调用链按需手写）。
//!
//! JDK 的 nio 缓冲（HeapByteBuffer / DirectByteBuffer 的 getShort / getInt …）经本类按
//! (base, offset) 读写多字节值，session 为 null（缓冲无内存段作用域）。寻址经
//! crate::native_memory：base 为 null 时 offset 是直接内存地址，否则是基本类型数组内偏移
//! （Unsafe.ARRAY_*_BASE_OFFSET + 下标 × 宽度），按 bigEndian 组装。
//! 消费方：AnnotationParser 经 ByteBuffer.wrap 解析注解原始字节（FS-R R4b）；
//! 写入族（putInt / putLong … 经 put*Unaligned）：用户代码的 ByteBuffer.putInt 等。其余方法保持存根。

use crate::prelude::*;
use super::scoped_memory_access::ScopedMemoryAccess;
use crate::jdk::internal::foreign::MemorySessionImpl;

fn read_bytes(base: &Object, offset: i64, n: usize) -> Result<Vec<u8>> {
    let mut out = vec![0u8; n];
    crate::native_memory::read(base, offset, &mut out)?;
    Ok(out)
}

fn write_bytes(base: &Object, offset: i64, v: u64, n: usize, big_endian: bool) -> Result<()> {
    // 大端：最高字节在前；小端：最低字节在前
    let bytes: Vec<u8> = (0..n)
        .map(|i| (v >> if big_endian { 8 * (n - 1 - i) } else { 8 * i }) as u8)
        .collect();
    crate::native_memory::write(base, offset, &bytes)
}

fn compose(bytes: &[u8], big_endian: bool) -> u64 {
    let mut v = 0u64;
    if big_endian {
        for b in bytes { v = (v << 8) | *b as u64; }
    } else {
        for b in bytes.iter().rev() { v = (v << 8) | *b as u64; }
    }
    v
}

impl ScopedMemoryAccess {
    /// 进程内唯一实例（对应静态字段 theScopedMemoryAccess）。
    #[jvm_boundary]
    pub fn getScopedMemoryAccess() -> Result<ScopedMemoryAccess> {
        crate::__process_static! {
            static THE_SMA: ScopedMemoryAccess = {
                let mut s = ScopedMemoryAccess::default();
                s._init_not_null();
                s
            };
        }
        Ok(THE_SMA.with(Clone::clone))
    }

    #[jvm_boundary]
    pub fn getByte(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<i8> {
        Ok(read_bytes(&base, offset, 1)?[0] as i8)
    }

    #[jvm_boundary]
    pub fn getShortUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, big_endian: bool) -> Result<i16> {
        Ok(compose(&read_bytes(&base, offset, 2)?, big_endian) as u16 as i16)
    }

    #[jvm_boundary]
    pub fn getCharUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, big_endian: bool) -> Result<u16> {
        Ok(compose(&read_bytes(&base, offset, 2)?, big_endian) as u16)
    }

    #[jvm_boundary]
    pub fn getIntUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, big_endian: bool) -> Result<i32> {
        Ok(compose(&read_bytes(&base, offset, 4)?, big_endian) as u32 as i32)
    }

    #[jvm_boundary]
    pub fn getLongUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, big_endian: bool) -> Result<i64> {
        Ok(compose(&read_bytes(&base, offset, 8)?, big_endian) as i64)
    }

    #[jvm_boundary]
    pub fn putByte(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i8) -> Result<()> {
        write_bytes(&base, offset, value as u8 as u64, 1, true)
    }

    #[jvm_boundary]
    pub fn putShortUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i16, big_endian: bool) -> Result<()> {
        write_bytes(&base, offset, value as u16 as u64, 2, big_endian)
    }

    #[jvm_boundary]
    pub fn putCharUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: u16, big_endian: bool) -> Result<()> {
        write_bytes(&base, offset, value as u64, 2, big_endian)
    }

    #[jvm_boundary]
    pub fn putIntUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i32, big_endian: bool) -> Result<()> {
        write_bytes(&base, offset, value as u32 as u64, 4, big_endian)
    }

    #[jvm_boundary]
    pub fn putLongUnaligned(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i64, big_endian: bool) -> Result<()> {
        write_bytes(&base, offset, value as u64, 8, big_endian)
    }

    // 对齐访问族（本机字节序；视图缓冲 DirectIntBufferS / DirectLongBufferU … 的 get / put 经此）。
    // JDK：`putXxx(session, base, offset, v)` → `UNSAFE.putXxx(base, offset, v)`，与 Unaligned 族同一寻址。
    #[jvm_boundary]
    pub fn getShort(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<i16> {
        let v = compose(&read_bytes(&base, offset, 2)?, cfg!(target_endian = "big"));
        Ok(v as u16 as i16)
    }

    #[jvm_boundary]
    pub fn putShort(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i16) -> Result<()> {
        write_bytes(&base, offset, value as u16 as u64, 2, cfg!(target_endian = "big"))
    }

    #[jvm_boundary]
    pub fn getChar(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<u16> {
        let v = compose(&read_bytes(&base, offset, 2)?, cfg!(target_endian = "big"));
        Ok(v as u16)
    }

    #[jvm_boundary]
    pub fn putChar(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: u16) -> Result<()> {
        write_bytes(&base, offset, value as u64, 2, cfg!(target_endian = "big"))
    }

    #[jvm_boundary]
    pub fn getInt(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<i32> {
        let v = compose(&read_bytes(&base, offset, 4)?, cfg!(target_endian = "big"));
        Ok(v as u32 as i32)
    }

    #[jvm_boundary]
    pub fn putInt(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i32) -> Result<()> {
        write_bytes(&base, offset, value as u32 as u64, 4, cfg!(target_endian = "big"))
    }

    #[jvm_boundary]
    pub fn getLong(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<i64> {
        let v = compose(&read_bytes(&base, offset, 8)?, cfg!(target_endian = "big"));
        Ok(v as i64)
    }

    #[jvm_boundary]
    pub fn putLong(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: i64) -> Result<()> {
        write_bytes(&base, offset, value as u64, 8, cfg!(target_endian = "big"))
    }

    #[jvm_boundary]
    pub fn getFloat(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<f32> {
        let v = compose(&read_bytes(&base, offset, 4)?, cfg!(target_endian = "big"));
        Ok(f32::from_bits(v as u32))
    }

    #[jvm_boundary]
    pub fn putFloat(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: f32) -> Result<()> {
        write_bytes(&base, offset, value.to_bits() as u64, 4, cfg!(target_endian = "big"))
    }

    #[jvm_boundary]
    pub fn getDouble(&self, _session: MemorySessionImpl, base: Object, offset: i64) -> Result<f64> {
        let v = compose(&read_bytes(&base, offset, 8)?, cfg!(target_endian = "big"));
        Ok(f64::from_bits(v))
    }

    #[jvm_boundary]
    pub fn putDouble(&self, _session: MemorySessionImpl, base: Object, offset: i64, value: f64) -> Result<()> {
        write_bytes(&base, offset, value.to_bits(), 8, cfg!(target_endian = "big"))
    }

    /// `copyMemory(srcSession, dstSession, srcBase, srcOffset, destBase, destOffset, bytes)`：
    /// 缓冲批量读写（DirectByteBuffer.get(byte[]) / put(byte[]) 等）。
    #[jvm_boundary]
    pub fn copyMemory(&self, _src_session: MemorySessionImpl, _dst_session: MemorySessionImpl,
                      src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                      bytes: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, 1)
    }

    /// `copySwapMemory(..., bytes, elemSize)`：按元素宽度翻转字节序的批量拷贝
    /// （非本机字节序视图缓冲的批量读写）。
    #[jvm_boundary]
    pub fn copySwapMemory(&self, _src_session: MemorySessionImpl, _dst_session: MemorySessionImpl,
                          src_base: Object, src_offset: i64, dst_base: Object, dst_offset: i64,
                          bytes: i64, elem_size: i64) -> Result<()> {
        crate::native_memory::copy(&src_base, src_offset, &dst_base, dst_offset, bytes, elem_size as usize)
    }

    /// `setMemory(session, o, offset, bytes, value)`：填充（缓冲清零等）。
    #[jvm_boundary]
    pub fn setMemory(&self, _session: MemorySessionImpl, o: Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
        crate::native_memory::fill(&o, offset, bytes, value)
    }
}

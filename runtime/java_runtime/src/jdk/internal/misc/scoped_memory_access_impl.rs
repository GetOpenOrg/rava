//! `jdk/internal/misc/ScopedMemoryAccess`（内部边界类，按调用链按需手写）。
//!
//! JDK 的 nio 堆缓冲（HeapByteBuffer.getShort / getInt …）经本类按 (base, offset) 读多字节值，
//! session 为 null（堆缓冲无内存段作用域）。原生二进制只承载堆 byte[]：offset 为
//! Unsafe.ARRAY_BYTE_BASE_OFFSET + 下标（与 unsafe__impl 同一约定），按 bigEndian 组装。
//! 消费方：AnnotationParser 经 ByteBuffer.wrap 解析注解原始字节（FS-R R4b）。其余方法保持存根。

use crate::prelude::*;
use super::scoped_memory_access::ScopedMemoryAccess;
use crate::jdk::internal::foreign::MemorySessionImpl;

/// Unsafe.ARRAY_BYTE_BASE_OFFSET（unsafe__impl.rs ARRAY_BASE_OFFSET 同值）。
const ARRAY_BASE_OFFSET: i64 = 16;

fn read_bytes(base: &Object, offset: i64, n: usize) -> Result<Vec<u8>> {
    let arr = <JArray<i8> as From<Object>>::from(Clone::clone(base));
    let start = (offset - ARRAY_BASE_OFFSET) as i32;
    let mut out = Vec::with_capacity(n);
    for i in 0..n as i32 {
        out.push(arr.get(start + i)? as u8);
    }
    Ok(out)
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
}

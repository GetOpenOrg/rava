//! `java/util/zip/CRC32` 的 native 方法（与生成的 crc32.rs 共置）。
//! 对标 zlib `crc32()`：反射多项式 0xEDB88320，入参 / 返回为已取反后的 CRC 值。

use crate::prelude::*;
use super::crc32::CRC32;

fn crc_table() -> &'static [u32; 256] {
    static TABLE: std::sync::OnceLock<[u32; 256]> = std::sync::OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [0u32; 256];
        for (n, slot) in t.iter_mut().enumerate() {
            let mut c = n as u32;
            for _ in 0..8 {
                c = if c & 1 != 0 { 0xEDB8_8320 ^ (c >> 1) } else { c >> 1 };
            }
            *slot = c;
        }
        t
    })
}

fn crc_update(crc: i32, bytes: impl Iterator<Item = u8>) -> i32 {
    let t = crc_table();
    let mut c = !(crc as u32);
    for b in bytes {
        c = t[((c ^ b as u32) & 0xff) as usize] ^ (c >> 8);
    }
    (!c) as i32
}

impl CRC32 {
    /// `update(int crc, int b)`：追加单字节（低 8 位）。
    #[jvm_native]
    pub fn update_i_i(crc: i32, b: i32) -> Result<i32> {
        Ok(crc_update(crc, std::iter::once(b as u8)))
    }

    /// `updateBytes0(int crc, byte[] b, int off, int len)`。
    #[jvm_native]
    pub fn updateBytes0(crc: i32, b: JArray<i8>, off: i32, len: i32) -> Result<i32> {
        let mut buf = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            buf.push(b.get(i)? as u8);
        }
        Ok(crc_update(crc, buf.into_iter()))
    }

    /// `updateByteBuffer0(int, long addr, int off, int len)`：直接缓冲区（直接内存地址 addr + off 起 len 字节）。
    #[jvm_native]
    pub fn updateByteBuffer0(v: i32, addr: i64, off: i32, len: i32) -> Result<i32> {
        // SAFETY: addr 为 DirectByteBuffer 的直接内存地址，Java 侧已做 off / len 界检查
        let bytes = unsafe { std::slice::from_raw_parts((addr + off as i64) as *const u8, len.max(0) as usize) };
        Ok(crc_update(v, bytes.iter().copied()))
    }
}

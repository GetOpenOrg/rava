//! `java/util/zip/Adler32` 的 native 方法（与生成的 adler32.rs 共置）。
//! 对标 zlib `adler32()`（RFC 1950）：s1 = 1 + Σbyte，s2 = Σs1，均 mod 65521。

use crate::prelude::*;
use super::adler32::Adler32;

const BASE: u32 = 65521;

fn adler_update(adler: i32, bytes: impl Iterator<Item = u8>) -> i32 {
    let mut s1 = (adler as u32) & 0xffff;
    let mut s2 = ((adler as u32) >> 16) & 0xffff;
    for b in bytes {
        s1 = (s1 + b as u32) % BASE;
        s2 = (s2 + s1) % BASE;
    }
    ((s2 << 16) | s1) as i32
}

impl Adler32 {
    /// `update(int adler, int b)`：追加单字节（低 8 位）。
    #[jvm_native]
    pub fn update_i_i(adler: i32, b: i32) -> Result<i32> {
        Ok(adler_update(adler, std::iter::once(b as u8)))
    }

    /// `updateBytes(int adler, byte[] b, int off, int len)`。
    #[jvm_native]
    pub fn updateBytes(adler: i32, b: JArray<i8>, off: i32, len: i32) -> Result<i32> {
        let mut buf = Vec::with_capacity(len.max(0) as usize);
        for i in off..off + len {
            buf.push(b.get(i)? as u8);
        }
        Ok(adler_update(adler, buf.into_iter()))
    }

    /// `updateByteBuffer(int, long addr, int off, int len)`：直接缓冲区（直接内存地址 addr + off 起 len 字节）。
    #[jvm_native]
    pub fn updateByteBuffer(v: i32, addr: i64, off: i32, len: i32) -> Result<i32> {
        // SAFETY: addr 为 DirectByteBuffer 的直接内存地址，Java 侧已做 off / len 界检查
        let bytes = unsafe { std::slice::from_raw_parts((addr + off as i64) as *const u8, len.max(0) as usize) };
        Ok(adler_update(v, bytes.iter().copied()))
    }
}

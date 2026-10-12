//! 直接内存（`Unsafe.allocateMemory` 族返回的绝对地址）的字粒度原子访问原语。
//!
//! ## 单一原子单元
//!
//! 直接内存可被 Java 代码以任意宽度混访：`copyMemory` / `setMemory` 批量读写、`getInt` / `putByte`
//! 普通读写、`compareAndSetLong` / `getAndAddInt` 原子读-改-写，且可来自不同线程（例如 VarHandle
//! CAS 与 `ByteBuffer.put(byte[])` 落在同一直接缓冲区）。Rust 内存模型下，原子与非原子访问并发、或
//! **不同宽度**的原子访问并发且其一为写，都是未定义行为。本 crate 把全部访问统一落在**包含该地址的
//! 8 字节对齐字**上的 `AtomicU64` 操作：
//!
//! - 整字覆盖的读 / 写：一次 `Relaxed` load / store；
//! - 字内部分字节的写：在该字上 `Relaxed` CAS 循环合并（相邻字节不受影响，不丢其他线程的更新）；
//! - 原子读-改-写（CAS / 交换 / getAndAdd）：目标区间须按自身宽度对齐（必在一个字内），在该字上
//!   `SeqCst` CAS 循环，只比较 / 替换自身字节——同字另一半的并发改动只引起重试，不改变结果。
//!
//! 于是同一地址上的任意两次并发访问都是同宽（8 字节）、同对齐的原子操作，没有数据竞争，也没有混宽
//! 原子访问。普通（非 volatile）读写取 `Relaxed`；volatile 语义由调用方以栅栏补足（与 HotSpot
//! `MemoryAccess::get_volatile` / `put_volatile` 同解）。
//!
//! ## 地址前提（各 `unsafe fn` 的安全契约）
//!
//! 访问区间 `[addr, addr + n)` 所覆盖的每个 8 字节对齐字整体可读写、在调用期间保持映射，且该内存
//! 只经本 crate（或其他 8 字节对齐的 `AtomicU64` 操作）访问。直接内存的来源全部满足：
//! `Unsafe.allocateMemory` / `reallocateMemory` 先按 HeapWordSize（8）向上取整再交给 malloc
//! （malloc 返回至少 16 字节对齐）；`FileChannel.map` 的 mmap 区间以页为粒度。

use std::sync::atomic::{AtomicU64, Ordering};

const WORD: usize = 8;

/// 访问参数不被支持。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// 长度为负（Unsafe 的 `size_t` 参数）。
    NegativeLength(i64),
    /// 原子读-改-写的地址未按宽度对齐。
    Misaligned { addr: usize, width: usize },
}

/// Java 侧 `long bytes` → 长度；负数为 [`Error::NegativeLength`]。
pub fn length(bytes: i64) -> Result<usize, Error> {
    usize::try_from(bytes).map_err(|_| Error::NegativeLength(bytes))
}

/// 包含 `addr` 的 8 字节对齐字。
///
/// # Safety
/// 该字满足模块头注的地址前提。
#[inline]
unsafe fn word<'a>(addr: usize) -> &'a AtomicU64 {
    // SAFETY: 地址 8 字节对齐（掩去低 3 位）；有效性与「只经原子访问」由调用方保证
    unsafe { AtomicU64::from_ptr((addr & !(WORD - 1)) as *mut u64) }
}

/// 宽度 `width`（1..=8）的低位掩码。
#[inline]
fn mask(width: usize) -> u64 {
    if width >= WORD { u64::MAX } else { (1u64 << (width * 8)) - 1 }
}

/// 字内偏移 `off`、宽度 `width` 的字节区间在字值（本机字节序）中的位移。
#[inline]
fn shift(off: usize, width: usize) -> u32 {
    (if cfg!(target_endian = "big") { (WORD - off - width) * 8 } else { off * 8 }) as u32
}

/// 字值 `w` 中 `[off, off + width)` 字节构成的本机字节序值（零扩展）。
#[inline]
fn extract(w: u64, off: usize, width: usize) -> u64 {
    (w >> shift(off, width)) & mask(width)
}

/// 字值 `w` 的 `[off, off + width)` 字节替换为 `v` 的低 `width` 字节。
#[inline]
fn insert(w: u64, off: usize, width: usize, v: u64) -> u64 {
    let s = shift(off, width);
    (w & !(mask(width) << s)) | ((v & mask(width)) << s)
}

/// 字内部分字节的普通写：Relaxed CAS 循环合并，只替换 `[off, off + width)`。
///
/// # Safety
/// 同 [`word`]。
#[inline]
unsafe fn merge(addr: usize, off: usize, width: usize, v: u64) {
    // SAFETY: 由调用方保证
    let a = unsafe { word(addr) };
    let mut cur = a.load(Ordering::Relaxed);
    while let Err(actual) = a.compare_exchange_weak(cur, insert(cur, off, width, v), Ordering::Relaxed, Ordering::Relaxed) {
        cur = actual;
    }
}

/// 字节串 → 本机字节序零扩展值（`bytes.len() <= 8`）。
#[inline]
fn bits_of(bytes: &[u8]) -> u64 {
    let mut b = [0u8; WORD];
    if cfg!(target_endian = "big") {
        b[WORD - bytes.len()..].copy_from_slice(bytes);
        u64::from_be_bytes(b)
    } else {
        b[..bytes.len()].copy_from_slice(bytes);
        u64::from_le_bytes(b)
    }
}

/// `addr` 起、总长 `n` 的区间在第一个对齐字边界之前的字节数（头段长）。
#[inline]
fn head_len(addr: usize, n: usize) -> usize {
    (addr.wrapping_neg() & (WORD - 1)).min(n)
}

/// 读 `dst.len()` 字节：头 / 尾不足一字的段各一次整字读取后截取，中间整字逐字 Relaxed 读。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn read(addr: usize, dst: &mut [u8]) {
    let n = dst.len();
    let h = head_len(addr, n);
    if h > 0 {
        let off = addr & (WORD - 1);
        // SAFETY: 由调用方保证
        let w = unsafe { word(addr) }.load(Ordering::Relaxed).to_ne_bytes();
        dst[..h].copy_from_slice(&w[off..off + h]);
    }
    let (body, tail) = dst[h..].split_at_mut((n - h) / WORD * WORD);
    let mut a = addr + h;
    for c in body.chunks_exact_mut(WORD) {
        // SAFETY: 由调用方保证
        c.copy_from_slice(&unsafe { word(a) }.load(Ordering::Relaxed).to_ne_bytes());
        a += WORD;
    }
    if !tail.is_empty() {
        // SAFETY: 由调用方保证
        let w = unsafe { word(a) }.load(Ordering::Relaxed).to_ne_bytes();
        let t = tail.len();
        tail.copy_from_slice(&w[..t]);
    }
}

/// 写 `src`：头 / 尾不足一字的段经同字 CAS 合并，中间整字逐字 Relaxed 写。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn write(addr: usize, src: &[u8]) {
    let n = src.len();
    let h = head_len(addr, n);
    if h > 0 {
        // SAFETY: 由调用方保证
        unsafe { merge(addr, addr & (WORD - 1), h, bits_of(&src[..h])) };
    }
    let (body, tail) = src[h..].split_at((n - h) / WORD * WORD);
    let mut a = addr + h;
    for c in body.chunks_exact(WORD) {
        let mut b = [0u8; WORD];
        b.copy_from_slice(c);
        // SAFETY: 由调用方保证
        unsafe { word(a) }.store(u64::from_ne_bytes(b), Ordering::Relaxed);
        a += WORD;
    }
    if !tail.is_empty() {
        // SAFETY: 由调用方保证
        unsafe { merge(a, 0, tail.len(), bits_of(tail)) };
    }
}

/// `[addr, addr + n)` 填 `value`（Unsafe.setMemory0）。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn fill(addr: usize, n: usize, value: u8) {
    let full = u64::from_ne_bytes([value; WORD]);
    let h = head_len(addr, n);
    if h > 0 {
        // SAFETY: 由调用方保证
        unsafe { merge(addr, addr & (WORD - 1), h, full) };
    }
    let mut a = addr + h;
    let end = addr + n;
    while end - a >= WORD {
        // SAFETY: 由调用方保证
        unsafe { word(a) }.store(full, Ordering::Relaxed);
        a += WORD;
    }
    if a < end {
        // SAFETY: 由调用方保证
        unsafe { merge(a, 0, end - a, full) };
    }
}

/// 拷贝 `n` 字节（memmove 语义，区间可重叠）。
///
/// 源与目标字内偏移相同时（`allocateMemory` 区间之间、缓冲区同偏移拷贝的常见情形）走逐字快路径：
/// 头 / 尾段先整段读出再写入，中间整字一读一写；目标在源之后且重叠时自尾向头。其余情形按块经栈缓冲
/// 读出再写入。
///
/// # Safety
/// 两个区间均满足模块头注的地址前提。
pub unsafe fn copy(src: usize, dst: usize, n: usize) {
    let backward = dst > src && dst < src.wrapping_add(n);
    if (src ^ dst) & (WORD - 1) == 0 {
        let h = head_len(dst, n);
        let words = (n - h) / WORD;
        let t = n - h - words * WORD;
        // SAFETY: 由调用方保证；小段整段读出再写入
        let small = |at: usize, len: usize| unsafe {
            let mut b = [0u8; WORD];
            read(src + at, &mut b[..len]);
            write(dst + at, &b[..len]);
        };
        // SAFETY: 由调用方保证；源与目标同偏移，第 i 字一读一写
        let step = |i: usize| unsafe {
            let at = h + i * WORD;
            word(dst + at).store(word(src + at).load(Ordering::Relaxed), Ordering::Relaxed);
        };
        if backward {
            if t > 0 {
                small(n - t, t);
            }
            (0..words).rev().for_each(step);
            if h > 0 {
                small(0, h);
            }
        } else {
            if h > 0 {
                small(0, h);
            }
            (0..words).for_each(step);
            if t > 0 {
                small(n - t, t);
            }
        }
        return;
    }
    const CHUNK: usize = 512;
    let mut buf = [0u8; CHUNK];
    let mut chunk = |at: usize, len: usize| {
        // SAFETY: 由调用方保证；每块先整体读出再写入，重叠方向由遍历次序保证不读到已改写的源
        unsafe {
            read(src + at, &mut buf[..len]);
            write(dst + at, &buf[..len]);
        }
    };
    if backward {
        let mut end = n;
        while end > 0 {
            let len = end.min(CHUNK);
            end -= len;
            chunk(end, len);
        }
    } else {
        let mut at = 0;
        while at < n {
            let len = (n - at).min(CHUNK);
            chunk(at, len);
            at += len;
        }
    }
}

#[inline] // 直接内存热路径：跨 crate（java_runtime ↔ 语料 crate）thin LTO 下显式导入内联（普通段 +25% 回退，分诊报告六）
/// 普通读：`addr` 处 `width`（1..=8）字节的本机字节序值（零扩展）。不跨字时为单次原子读。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn load(addr: usize, width: usize) -> u64 {
    let off = addr & (WORD - 1);
    if off + width <= WORD {
        // SAFETY: 由调用方保证
        return extract(unsafe { word(addr) }.load(Ordering::Relaxed), off, width);
    }
    let mut b = [0u8; WORD];
    // SAFETY: 由调用方保证
    unsafe { read(addr, &mut b[..width]) };
    bits_of(&b[..width])
}

#[inline] // 直接内存热路径：跨 crate（java_runtime ↔ 语料 crate）thin LTO 下显式导入内联（普通段 +25% 回退，分诊报告六）
/// 普通写：`v` 的低 `width`（1..=8）字节写到 `addr`。对齐的 8 字节为单次原子写，其余字内写为
/// 同字 CAS 合并；跨字写逐字完成（与 HotSpot 非对齐普通访问同为非整体原子）。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn store(addr: usize, width: usize, v: u64) {
    let off = addr & (WORD - 1);
    if off == 0 && width == WORD {
        // SAFETY: 由调用方保证
        unsafe { word(addr) }.store(v, Ordering::Relaxed);
    } else if off + width <= WORD {
        // SAFETY: 由调用方保证
        unsafe { merge(addr, off, width, v) };
    } else {
        let bytes = v.to_ne_bytes();
        let part = if cfg!(target_endian = "big") { &bytes[WORD - width..] } else { &bytes[..width] };
        // SAFETY: 由调用方保证
        unsafe { write(addr, part) };
    }
}

#[inline] // 直接内存热路径：跨 crate（java_runtime ↔ 语料 crate）thin LTO 下显式导入内联（普通段 +25% 回退，分诊报告六）
/// 原子读-改-写：`addr` 处 `width`（1 / 2 / 4 / 8）字节值（本机字节序零扩展）。`op(旧)` 给新值即
/// 原子替换，给 None 即不写；返回旧值。顺序一致（Java CAS 的 volatile 读写语义）。CAS 重试时 op
/// 重新求值（须为纯函数）。
///
/// 地址须按 `width` 对齐，否则 [`Error::Misaligned`]：未对齐区间可能跨字，无法以单条原子指令完成，
/// 退化为读-改-写会丢并发更新。
///
/// # Safety
/// 见模块头注的地址前提。
pub unsafe fn update(addr: usize, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Result<u64, Error> {
    if width == 0 || width > WORD || !width.is_power_of_two() || addr % width != 0 {
        return Err(Error::Misaligned { addr, width });
    }
    let off = addr & (WORD - 1);
    // SAFETY: 由调用方保证
    let a = unsafe { word(addr) };
    let mut cur = a.load(Ordering::SeqCst);
    loop {
        let old = extract(cur, off, width);
        let Some(n) = op(old) else { return Ok(old) };
        match a.compare_exchange_weak(cur, insert(cur, off, width, n), Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => return Ok(old),
            Err(actual) => cur = actual,
        }
    }
}

#[cfg(test)]
mod tests;

//! 原生内存访问（运行时基础设施）：Unsafe / ScopedMemoryAccess 的 `(base, offset)` 寻址。
//!
//! HotSpot 的寻址约定：`base == null` 时 `offset` 是绝对地址（直接内存，
//! `Unsafe.allocateMemory` 分配）；否则是堆对象内偏移——基本类型数组为
//! `ARRAY_BASE_OFFSET + 下标 × 元素宽度`（与 unsafe__impl / scoped_memory_access_impl
//! 同一组常量）。原生二进制没有 C 布局的堆数组，基本类型数组按其元素存储的本机字节
//! 视图寻址（小端机器上与 HotSpot 逐字节一致）。

use crate::array::JArray;
use crate::error::{JvmError, Result};
use crate::java::lang::Object;

/// 基本类型数组的首元素偏移（Unsafe.ARRAY_*_BASE_OFFSET，全部数组类同值）。
pub const ARRAY_BASE_OFFSET: i64 = 16;

/// 分配 `bytes` 字节直接内存（malloc）；失败返回 0。
pub fn allocate(bytes: i64) -> i64 {
    // SAFETY: malloc 只返回新块或 null
    unsafe { libc::malloc(bytes as usize) as i64 }
}

/// 重新分配（realloc）；`address == 0` 等价 allocate。失败返回 0。
pub fn reallocate(address: i64, bytes: i64) -> i64 {
    // SAFETY: address 为 allocate / reallocate 所得（或 0）
    unsafe { libc::realloc(address as *mut libc::c_void, bytes as usize) as i64 }
}

/// 释放直接内存（free）。
pub fn free(address: i64) {
    // SAFETY: address 为 allocate / reallocate 所得（或 0）
    unsafe { libc::free(address as *mut libc::c_void) }
}

/// 基本类型数组 `o` 按元素类型取数组对象 `__ArrayObj<T>`，对其求 `$e`（`$a` 绑定为数组引用）；非基本类型
/// 数组为 None。数组字节视图的访问全部经元素原子单元（见 array/store.rs），不持锁、无数据竞争。
macro_rules! with_prim_array {
    ($o:expr, |$a:ident| $e:expr) => {{
        let any = $o.0.as_any();
        with_prim_array!(@try any, $a, $e; bool, i8, u16, i16, i32, f32, i64, f64)
    }};
    (@try $any:ident, $a:ident, $e:expr; $($t:ty),*) => {{
        let mut r = None;
        $(
            if r.is_none() {
                if let Some($a) = $any.downcast_ref::<crate::array::__ArrayObj<$t>>() {
                    r = Some($e);
                }
            }
        )*
        r
    }};
}

/// `width` 字节的本机字节序位形 → 零扩展的 u64。
pub(crate) fn load_bits(src: &[u8]) -> u64 {
    let mut b = [0u8; 8];
    if cfg!(target_endian = "big") {
        b[8 - src.len()..].copy_from_slice(src);
        u64::from_be_bytes(b)
    } else {
        b[..src.len()].copy_from_slice(src);
        u64::from_le_bytes(b)
    }
}

/// u64 的低 `dst.len()` 字节按本机字节序写入 `dst`（截断）。
pub(crate) fn store_bits(dst: &mut [u8], v: u64) {
    let n = dst.len();
    if cfg!(target_endian = "big") {
        dst.copy_from_slice(&v.to_be_bytes()[8 - n..]);
    } else {
        dst.copy_from_slice(&v.to_le_bytes()[..n]);
    }
}

// ── 直接内存（base 为 null 的绝对地址）────────────────────────────────────────────
// 全部经 `rava_rawmem` 的字粒度原子原语：批量读写 / 填充 / 拷贝与 CAS 落在同一 8 字节原子单元上，
// 同地址的并发访问恒为同宽原子操作，无数据竞争、无混宽原子访问（单元设计与地址前提见该 crate 头注）。
// 地址前提：直接内存只来自 Unsafe.allocateMemory / reallocateMemory（Java 侧先按 HeapWordSize 向上
// 取整，malloc 至少 16 字节对齐）与 FileChannel.map（mmap，页粒度），覆盖区间的对齐字整体可访问。

/// `rava_rawmem` 的参数拒绝 → Java 异常。
fn raw_err(e: rava_rawmem::Error) -> JvmError {
    match e {
        rava_rawmem::Error::NegativeLength(n) => JvmError::illegal_argument(&format!("negative size: {n}")),
        // HotSpot 对 int / long CAS 不查对齐，结果随平台（x86 走 split lock 仍原子，aarch64 无 LSE2
        // 时对齐故障 → InternalError）；JDK 自身对无法以单字原子完成的更新的规定是 IAE
        //（Unsafe.compareAndExchangeShort：「Update spans the word, not supported」）。此处取后者，
        // 各平台一致，且不静默退化为会丢并发更新的非原子读-改-写。
        rava_rawmem::Error::Misaligned { addr, width } => JvmError::illegal_argument(&format!(
            "Unaligned atomic update of {width} bytes at address {addr:#x}, not supported")),
    }
}

/// 长度参数（Unsafe 的 size_t）：负数 → IllegalArgumentException（与 Java 侧 `Unsafe.checkSize`
/// 同一口径；公开包装已先查，这里守住 native 入口本身）。
fn len(bytes: i64) -> Result<usize> {
    rava_rawmem::length(bytes).map_err(raw_err)
}

/// `(base, offset)` 处 `width`（1 / 2 / 4 / 8）字节值的原子读-改-写（Unsafe / VarHandle 的 CAS、交换、
/// getAndAdd 族）。值是本机字节序零扩展的位形；`op(旧)` 给出新值则写入其低 `width` 字节，返回旧值，
/// 给 None 即不写。基本类型数组经元素原子单元（区间在单个元素内为该元素的 CAS，跨元素见
/// `__ArrayObj::__update_bytes`）；直接内存经包含该地址的对齐字上的 SeqCst CAS，地址须按宽度对齐，
/// 否则 IllegalArgumentException（见 [`raw_err`]）。CAS 重试时 op 重新求值（须为纯函数）。
pub fn update(base: &Object, offset: i64, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Result<u64> {
    if base.0.is_jvm_null() {
        // SAFETY: 直接内存地址前提见本节头注
        return unsafe { rava_rawmem::update(offset as usize, width, op) }.map_err(raw_err);
    }
    let start = usize::try_from(offset - ARRAY_BASE_OFFSET).unwrap_or(usize::MAX - 8);
    with_prim_array!(base, |a| a.__update_bytes(start, width, &mut *op)).flatten().ok_or_else(out_of_bounds)
}

/// 普通读（Unsafe get 族，volatile 由调用方加栅栏）：`(base, offset)` 处 `width` 字节值（零扩展）。
/// 直接内存不要求对齐（未对齐时逐字读取，与 HotSpot 非对齐普通访问同为非整体原子）。
pub fn load(base: &Object, offset: i64, width: usize) -> Result<u64> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 update
        return Ok(unsafe { rava_rawmem::load(offset as usize, width) });
    }
    update(base, offset, width, &mut |_| None)
}

/// 普通写（Unsafe put 族，volatile 由调用方加栅栏）：`v` 的低 `width` 字节写到 `(base, offset)`。
pub fn store(base: &Object, offset: i64, width: usize, v: u64) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 update
        unsafe { rava_rawmem::store(offset as usize, width, v) };
        return Ok(());
    }
    update(base, offset, width, &mut |_| Some(v)).map(|_| ())
}

/// 直接内存地址上的 volatile 读（Java 侧以 `Unsafe.putIntVolatile` 等写入的轮询标记，如 Files.copy
/// 的取消字）：与 Unsafe getXxxVolatile 同一原子单元与栅栏口径。
///
/// # Safety
/// `address` 为调用方持有、在调用期间保持有效的直接内存（满足本节头注的地址前提）。
pub unsafe fn load_volatile_direct(address: i64, width: usize) -> u64 {
    use std::sync::atomic::{fence, Ordering};
    fence(Ordering::SeqCst);
    // SAFETY: 由调用方保证
    let v = unsafe { rava_rawmem::load(address as usize, width) };
    fence(Ordering::Acquire);
    v
}

fn out_of_bounds() -> JvmError {
    JvmError::illegal_argument("Unsafe access out of array bounds")
}

/// 从 `(base, offset)` 读 `dst.len()` 字节。
pub fn read(base: &Object, offset: i64, dst: &mut [u8]) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 update；区间由调用方（DirectByteBuffer 等）做过界检查
        unsafe { rava_rawmem::read(offset as usize, dst) };
        return Ok(());
    }
    let start = usize::try_from(offset - ARRAY_BASE_OFFSET).map_err(|_| out_of_bounds())?;
    match with_prim_array!(base, |a| a.__read_bytes(start, &mut *dst)) {
        Some(true) => Ok(()),
        _ => Err(out_of_bounds()),
    }
}

/// 向 `(base, offset)` 写 `src`。
pub fn write(base: &Object, offset: i64, src: &[u8]) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 read
        unsafe { rava_rawmem::write(offset as usize, src) };
        return Ok(());
    }
    let start = usize::try_from(offset - ARRAY_BASE_OFFSET).map_err(|_| out_of_bounds())?;
    match with_prim_array!(base, |a| a.__write_bytes(start, src)) {
        Some(true) => Ok(()),
        _ => Err(out_of_bounds()),
    }
}

/// `(base, offset)` 起 `bytes` 字节填 `value`（Unsafe.setMemory0）；负 bytes → IllegalArgumentException。
pub fn fill(base: &Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
    let n = len(bytes)?;
    if base.0.is_jvm_null() {
        // SAFETY: 同 read
        unsafe { rava_rawmem::fill(offset as usize, n, value as u8) };
        return Ok(());
    }
    write(base, offset, &vec![value as u8; n])
}

/// 拷贝 `bytes` 字节（Unsafe.copyMemory0）；负 bytes → IllegalArgumentException。源与目标可重叠
///（memmove 语义）；`swap_width > 1` 时按该宽度逐元素翻转字节序（Unsafe.copySwapMemory0）。
pub fn copy(src_base: &Object, src_offset: i64, dst_base: &Object, dst_offset: i64,
            bytes: i64, swap_width: usize) -> Result<()> {
    let n = len(bytes)?;
    if src_base.0.is_jvm_null() && dst_base.0.is_jvm_null() && swap_width <= 1 {
        // SAFETY: 两端均为直接内存，同 read
        unsafe { rava_rawmem::copy(src_offset as usize, dst_offset as usize, n) };
        return Ok(());
    }
    let mut buf = vec![0u8; n];
    read(src_base, src_offset, &mut buf)?;
    if swap_width > 1 {
        for chunk in buf.chunks_mut(swap_width) {
            chunk.reverse();
        }
    }
    write(dst_base, dst_offset, &buf)
}

/// `(base, offset)` 是否按原生内存寻址：base 为 null（直接内存）或基本类型数组。
/// 其余对象（实例字段偏移、引用数组）由 Unsafe 的字段 / 引用访问器处理。
pub fn is_raw(base: &Object) -> bool {
    base.0.is_jvm_null() || with_prim_array!(base, |_a| ()).is_some()
}

/// 读 N 字节（本机字节序，供 `T::from_ne_bytes`）。
pub fn read_ne<const N: usize>(base: &Object, offset: i64) -> Result<[u8; N]> {
    let mut b = [0u8; N];
    read(base, offset, &mut b)?;
    Ok(b)
}

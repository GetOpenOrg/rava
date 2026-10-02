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

/// 基本类型数组 `o` 的本机字节视图上执行 `f`（按元素类型分派，数组存储写锁内）；非基本类型数组
/// 返回 None。boolean 数组在 `f` 之后把每个字节规范化为 0 / 1（`bool` 的合法位形；锁内完成，
/// 读者不会看到中间态）。
fn with_array_bytes<R>(o: &Object, f: impl FnOnce(&mut [u8]) -> R) -> Option<Result<R>> {
    fn bytes_of<T>(v: &mut [T]) -> &mut [u8] {
        let n = std::mem::size_of_val(v);
        // SAFETY: 基本类型元素的存储即连续字节，视图不越过 Vec 的初始化区间
        unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, n) }
    }
    if let Some(a) = o.0.as_any().downcast_ref::<JArray<bool>>() {
        return Some(a.with_vec(|v| {
            let bytes = bytes_of(v);
            let r = f(&mut *bytes);
            for b in bytes.iter_mut() {
                *b = (*b != 0) as u8;
            }
            r
        }));
    }
    macro_rules! try_elem {
        ($($t:ty),*) => {$(
            if let Some(a) = o.0.as_any().downcast_ref::<JArray<$t>>() {
                return Some(a.with_vec(|v| f(bytes_of(v))));
            }
        )*};
    }
    try_elem!(i8, u16, i16, i32, f32, i64, f64);
    None
}

/// `width` 字节的本机字节序位形 → 零扩展的 u64。
fn load_bits(src: &[u8]) -> u64 {
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
fn store_bits(dst: &mut [u8], v: u64) {
    let n = dst.len();
    if cfg!(target_endian = "big") {
        dst.copy_from_slice(&v.to_be_bytes()[8 - n..]);
    } else {
        dst.copy_from_slice(&v.to_le_bytes()[..n]);
    }
}

/// `(base, offset)` 处 `width`（1 / 2 / 4 / 8）字节值的原子读-改-写（Unsafe 基本类型访问器族的
/// 原生内存形态：读、写、CAS、getAndAdd 等统一经此）。值是本机字节序零扩展的位形；`op(旧)` 给出
/// 新值则写入其低 `width` 字节，返回旧值，给 None 即只读。基本类型数组在数组存储写锁内完成（与
/// 数组元素的直接读写同一把锁）；直接内存经该地址上的同宽原子指令（地址按宽度对齐时——JDK 对
/// 未对齐地址的原子访问不作保证，未对齐时退为普通读写）。CAS 重试时 op 重新求值（须为纯函数）。
pub fn update(base: &Object, offset: i64, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Result<u64> {
    if base.0.is_jvm_null() {
        use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering::SeqCst};
        macro_rules! atomic {
            ($a:ty, $t:ty) => {{
                let p = offset as *mut $t;
                if (p as usize) % std::mem::align_of::<$t>() == 0 {
                    // SAFETY: 绝对地址由调用方经 allocateMemory（或 VM 承载的静态字）取得，按宽度对齐
                    let a = unsafe { <$a>::from_ptr(p) };
                    let mut cur = a.load(SeqCst);
                    loop {
                        let old = cur as u64;
                        let Some(n) = op(old) else { return Ok(old) };
                        match a.compare_exchange_weak(cur, n as $t, SeqCst, SeqCst) {
                            Ok(_) => return Ok(old),
                            Err(actual) => cur = actual,
                        }
                    }
                }
            }};
        }
        match width {
            1 => atomic!(AtomicU8, u8),
            2 => atomic!(AtomicU16, u16),
            4 => atomic!(AtomicU32, u32),
            8 => atomic!(AtomicU64, u64),
            _ => {}
        }
        let mut b = [0u8; 8];
        read(base, offset, &mut b[..width])?;
        let old = load_bits(&b[..width]);
        if let Some(n) = op(old) {
            store_bits(&mut b[..width], n);
            write(base, offset, &b[..width])?;
        }
        return Ok(old);
    }
    let start = usize::try_from(offset - ARRAY_BASE_OFFSET).unwrap_or(usize::MAX - 8);
    with_array_bytes(base, |b| match b.get_mut(start..start + width) {
        Some(slot) => {
            let old = load_bits(slot);
            if let Some(n) = op(old) {
                store_bits(slot, n);
            }
            Ok(old)
        }
        None => Err(out_of_bounds()),
    }).unwrap_or_else(|| Err(out_of_bounds()))?
}

fn out_of_bounds() -> JvmError {
    JvmError::illegal_argument("Unsafe access out of array bounds")
}

/// 从 `(base, offset)` 读 `dst.len()` 字节。
pub fn read(base: &Object, offset: i64, dst: &mut [u8]) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 绝对地址由调用方（DirectByteBuffer 等）经 allocateMemory 取得并做过界检查
        unsafe { std::ptr::copy_nonoverlapping(offset as *const u8, dst.as_mut_ptr(), dst.len()) };
        return Ok(());
    }
    let start = (offset - ARRAY_BASE_OFFSET) as usize;
    with_array_bytes(base, |b| match b.get(start..start + dst.len()) {
        Some(src) => { dst.copy_from_slice(src); Ok(()) }
        None => Err(out_of_bounds()),
    }).unwrap_or_else(|| Err(out_of_bounds()))?
}

/// 向 `(base, offset)` 写 `src`。
pub fn write(base: &Object, offset: i64, src: &[u8]) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 read
        unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), offset as *mut u8, src.len()) };
        return Ok(());
    }
    let start = (offset - ARRAY_BASE_OFFSET) as usize;
    with_array_bytes(base, |b| match b.get_mut(start..start + src.len()) {
        Some(dst) => { dst.copy_from_slice(src); Ok(()) }
        None => Err(out_of_bounds()),
    }).unwrap_or_else(|| Err(out_of_bounds()))?
}

/// `(base, offset)` 起 `bytes` 字节填 `value`（Unsafe.setMemory0）。
pub fn fill(base: &Object, offset: i64, bytes: i64, value: i8) -> Result<()> {
    if base.0.is_jvm_null() {
        // SAFETY: 同 read
        unsafe { std::ptr::write_bytes(offset as *mut u8, value as u8, bytes as usize) };
        return Ok(());
    }
    write(base, offset, &vec![value as u8; bytes as usize])
}

/// 拷贝 `bytes` 字节（Unsafe.copyMemory0）。源与目标可为同一数组（先读出再写入）；
/// `swap_width > 1` 时按该宽度逐元素翻转字节序（Unsafe.copySwapMemory0）。
pub fn copy(src_base: &Object, src_offset: i64, dst_base: &Object, dst_offset: i64,
            bytes: i64, swap_width: usize) -> Result<()> {
    if src_base.0.is_jvm_null() && dst_base.0.is_jvm_null() && swap_width <= 1 {
        // SAFETY: 两端均为直接内存，copy（memmove 语义）允许重叠
        unsafe { std::ptr::copy(src_offset as *const u8, dst_offset as *mut u8, bytes as usize) };
        return Ok(());
    }
    let mut buf = vec![0u8; bytes as usize];
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
    base.0.is_jvm_null() || with_array_bytes(base, |_| ()).is_some()
}

/// 读 N 字节（本机字节序，供 `T::from_ne_bytes`）。
pub fn read_ne<const N: usize>(base: &Object, offset: i64) -> Result<[u8; N]> {
    let mut b = [0u8; N];
    read(base, offset, &mut b)?;
    Ok(b)
}

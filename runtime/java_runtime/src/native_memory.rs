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

/// 基本类型数组 `o` 的本机字节视图上执行 `f`（按元素类型分派）；非基本类型数组返回 None。
fn with_array_bytes<R>(o: &Object, f: impl FnOnce(&mut [u8]) -> R) -> Option<Result<R>> {
    macro_rules! try_elem {
        ($($t:ty),*) => {$(
            if let Some(a) = o.0.as_any().downcast_ref::<JArray<$t>>() {
                return Some(a.with_vec(|v| {
                    let n = std::mem::size_of_val(v);
                    // SAFETY: 基本类型元素的存储即连续字节，视图不越过 Vec 的初始化区间
                    let bytes = unsafe { std::slice::from_raw_parts_mut(v.as_mut_ptr() as *mut u8, n) };
                    f(bytes)
                }));
            }
        )*};
    }
    try_elem!(i8, bool, u16, i16, i32, f32, i64, f64);
    None
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

//! JArray 的元素存取：无锁（宿主 array.rs 的私有辅助模块）
//!
//! - 基本元素：元素本身就是同宽原子单元（`AtomicU8/16/32/64::from_ptr`），普通读写为 relaxed
//!   （JMM 对普通数组元素不要求互斥与顺序；同步动作的获取 / 释放序给出 happens-before），原子
//!   读-改-写（Unsafe / VarHandle CAS 族）为 SeqCst。元素占原生宽度，无额外开销。
//! - 引用元素：逐元素内联自旋单元 `__RefField<T>`（锁内只克隆 / 交换，旧值锁外释放）。
//!
//! 任何元素访问都经原子指令或单元锁，不存在非原子的并发访问（无数据竞争 UB）。

use super::*;
use std::cell::UnsafeCell;
use std::mem::{size_of, ManuallyDrop};
use std::sync::atomic::{AtomicU16, AtomicU32, AtomicU64, AtomicU8, Ordering};

use crate::sync_model::__RefField;

/// 基本元素槽：只经同宽原子指令访问（`repr(transparent)`：元素区可按 T 写入、按槽读取）
#[repr(transparent)]
pub(super) struct PrimSlot<T>(UnsafeCell<T>);

// 槽只经原子指令访问
unsafe impl<T: Send> Sync for PrimSlot<T> {}

/// 数组对象的元素区（紧随 `__ArrayObj`，见 obj.rs）
#[derive(Clone, Copy)]
pub(super) enum Store<'a, T> {
    Prim(&'a [PrimSlot<T>]),
    Ref(&'a [__RefField<T>]),
}

/// 按元素宽度选同宽原子类型执行 `$body`（`$a` 绑定为该原子的引用，`$u` 为其无符号整型）。
/// 槽地址直接按原子类型取引用（与 `Atomic*::from_ptr` 同一前提），不经 std 方法：元素存取链
/// 标 `#[inline(always)]`，在 opt-level 0 的调用方 crate 里也展开成少量指令（见 obj.rs 快路径）。
macro_rules! with_atomic {
    ($slot:expr, |$a:ident, $u:ident| $body:expr) => {{
        let p = $slot.0.get();
        // SAFETY: 基本元素类型的宽度与对齐等于同宽原子类型（64 位目标），槽只经原子指令访问
        unsafe {
            match size_of::<T>() {
                1 => { type $u = u8; let $a = &*(p as *const AtomicU8); $body }
                2 => { type $u = u16; let $a = &*(p as *const AtomicU16); $body }
                4 => { type $u = u32; let $a = &*(p as *const AtomicU32); $body }
                _ => { type $u = u64; let $a = &*(p as *const AtomicU64); $body }
            }
        }
    }};
}

impl<T> PrimSlot<T> {
    /// 元素位形（零扩展到 u64）
    #[inline(always)]
    pub(super) fn load_bits(&self, ord: Ordering) -> u64 {
        with_atomic!(self, |a, U| a.load(ord) as u64)
    }

    #[inline(always)]
    pub(super) fn store_bits(&self, bits: u64, ord: Ordering) {
        with_atomic!(self, |a, U| a.store(bits as U, ord))
    }

    /// 位形 CAS（SeqCst）
    #[inline]
    pub(super) fn cas_bits(&self, cur: u64, new: u64) -> Result<(), u64> {
        with_atomic!(self, |a, U| a
            .compare_exchange_weak(cur as U, new as U, Ordering::SeqCst, Ordering::SeqCst)
            .map(|_| ())
            .map_err(|v| v as u64))
    }
}

/// 位形 → 元素值（低 `size_of::<T>()` 字节）
#[inline(always)]
pub(super) fn from_bits<T>(bits: u64) -> T {
    // SAFETY: T 为基本元素类型，位形来自同类型元素（或经规范化的 boolean 字节）
    unsafe {
        match size_of::<T>() {
            1 => std::ptr::read(&(bits as u8) as *const u8 as *const T),
            2 => std::ptr::read(&(bits as u16) as *const u16 as *const T),
            4 => std::ptr::read(&(bits as u32) as *const u32 as *const T),
            _ => std::ptr::read(&bits as *const u64 as *const T),
        }
    }
}

/// 元素值 → 位形（零扩展）
#[inline(always)]
pub(super) fn to_bits<T>(v: T) -> u64 {
    let v = ManuallyDrop::new(v);
    let p = &*v as *const T;
    // SAFETY: T 为基本元素类型，按同宽无符号整型读出（基本整型可按值复制，不经 ptr::read）
    unsafe {
        match size_of::<T>() {
            1 => *(p as *const u8) as u64,
            2 => *(p as *const u16) as u64,
            4 => *(p as *const u32) as u64,
            _ => *(p as *const u64),
        }
    }
}

impl<T: 'static> Store<'_, T> {
    #[inline]
    pub(super) fn len(&self) -> usize {
        match self {
            Store::Prim(s) => s.len(),
            Store::Ref(s) => s.len(),
        }
    }

    #[inline]
    fn index(&self, i: i32) -> crate::error::Result<usize> {
        let len = self.len();
        if i < 0 || i as usize >= len {
            return Err(crate::error::JvmError::array_index_out_of_bounds(i, len as i32));
        }
        Ok(i as usize)
    }

    /// 普通读（*aload）
    #[inline]
    pub(super) fn get(&self, i: i32) -> crate::error::Result<T> where T: Clone {
        let i = self.index(i)?;
        Ok(match self {
            Store::Prim(s) => from_bits(s[i].load_bits(Ordering::Relaxed)),
            Store::Ref(s) => s[i].get(),
        })
    }

    /// 普通写（*astore）
    #[inline]
    pub(super) fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        let i = self.index(i)?;
        match self {
            Store::Prim(s) => s[i].store_bits(to_bits(v), Ordering::Relaxed),
            Store::Ref(s) => s[i].set(v),
        }
        Ok(())
    }

    /// 元素原子读-改-写：基本元素为位形 CAS 循环（重试时 `f` 重新求值），引用元素在单元锁内
    /// 完成「读 → 判定 → 写」，被替换的旧值锁外释放。返回旧值。消费方只有 Unsafe / VarHandle
    /// 的 CAS / 交换（volatile 访问模式）：基本元素 SeqCst CAS，引用元素取单元的 volatile 族
    /// （加锁 / 解锁都 SeqCst，无 GC 文档第四节小步 B）。
    pub(super) fn update(&self, i: i32, f: &mut dyn FnMut(T) -> Option<T>) -> crate::error::Result<T>
    where T: Clone {
        let i = self.index(i)?;
        match self {
            Store::Prim(s) => {
                let slot = &s[i];
                let mut cur = slot.load_bits(Ordering::SeqCst);
                loop {
                    let Some(n) = f(from_bits(cur)) else { return Ok(from_bits(cur)) };
                    match slot.cas_bits(cur, to_bits(n)) {
                        Ok(()) => return Ok(from_bits(cur)),
                        Err(actual) => cur = actual,
                    }
                }
            }
            Store::Ref(s) => {
                let (cur, replaced) = s[i].with_mut_volatile(|v| {
                    let cur = v.clone();
                    let replaced = f(cur.clone()).map(|n| std::mem::replace(v, n));
                    (cur, replaced)
                });
                drop(replaced);
                Ok(cur)
            }
        }
    }

    /// 元素快照（逐元素读）
    pub(super) fn to_vec(&self) -> Vec<T> where T: Clone {
        match self {
            Store::Prim(s) => s.iter().map(|e| from_bits(e.load_bits(Ordering::Relaxed))).collect(),
            Store::Ref(s) => s.iter().map(|e| e.get()).collect(),
        }
    }
}

// ── 基本元素数组的本机字节视图（Unsafe / ScopedMemoryAccess 的 `(base, offset)` 堆寻址）──

/// 元素位形在内存中的前 `n` 字节（本机字节序）
fn elem_bytes(bits: u64, n: usize) -> [u8; 8] {
    let mut out = [0u8; 8];
    crate::native_memory::store_bits(&mut out[..n], bits);
    out
}

/// 跨元素的字节读-改-写互斥（按数组存储地址分条）：只在一次访问跨越多个元素时使用
static STRIPES: [parking_lot::Mutex<()>; 16] = [const { parking_lot::const_mutex(()) }; 16];

impl<'a, T: 'static> Store<'a, T> {
    fn prim_slots(&self) -> Option<&'a [PrimSlot<T>]> {
        match *self {
            Store::Prim(s) => Some(s),
            Store::Ref(_) => None,
        }
    }

    /// 字节区间 `[start, start + n)` 落在存储内时，返回覆盖它的元素下标区间
    fn covering(&self, start: usize, n: usize) -> Option<(&'a [PrimSlot<T>], usize, usize)> {
        let slots = self.prim_slots()?;
        let w = size_of::<T>();
        let end = start.checked_add(n)?;
        if end > slots.len() * w || n == 0 {
            return (n == 0 && start <= slots.len() * w).then_some((slots, 0, 0));
        }
        Some((slots, start / w, (end - 1) / w + 1))
    }

    /// 读 `dst.len()` 字节（逐元素 relaxed 原子读）。区间越界或非基本元素数组返回 false。
    pub(super) fn read_bytes(&self, start: usize, dst: &mut [u8]) -> bool {
        let Some((slots, first, last)) = self.covering(start, dst.len()) else { return false };
        let w = size_of::<T>();
        for e in first..last {
            let bytes = elem_bytes(slots[e].load_bits(Ordering::Relaxed), w);
            for k in 0..w {
                let p = e * w + k;
                if p >= start && p < start + dst.len() {
                    dst[p - start] = bytes[k];
                }
            }
        }
        true
    }

    /// 写 `src`：整元素覆盖为 relaxed 原子写，部分覆盖的元素经位形 CAS 合并（不改动区间外字节）。
    /// boolean 元素按 JVM 规范化为 0 / 1。
    pub(super) fn write_bytes(&self, start: usize, src: &[u8]) -> bool {
        let Some((slots, first, last)) = self.covering(start, src.len()) else { return false };
        let w = size_of::<T>();
        let is_bool = std::any::TypeId::of::<T>() == std::any::TypeId::of::<bool>();
        let merge = |bits: u64, e: usize| -> u64 {
            let mut bytes = elem_bytes(bits, w);
            for k in 0..w {
                let p = e * w + k;
                if p >= start && p < start + src.len() {
                    bytes[k] = src[p - start];
                }
            }
            if is_bool {
                bytes[0] = (bytes[0] != 0) as u8;
            }
            crate::native_memory::load_bits(&bytes[..w])
        };
        for e in first..last {
            let slot = &slots[e];
            if e * w >= start && (e + 1) * w <= start + src.len() {
                slot.store_bits(merge(0, e), Ordering::Relaxed);
                continue;
            }
            let mut cur = slot.load_bits(Ordering::Relaxed);
            while let Err(actual) = slot.cas_bits(cur, merge(cur, e)) {
                cur = actual;
            }
        }
        true
    }

    /// `width` 字节值的原子读-改-写（值为本机字节序零扩展位形；`op` 给 None 即只读）。区间在单个
    /// 元素内时为该元素上的 CAS 循环（与元素的原子访问同一单元，真原子）；跨元素时（如 byte[] 上
    /// 的 int / long 访问）在分条锁内读 → 改 → 写，与其他跨元素读-改-写互斥。越界返回 None。
    pub(super) fn update_bytes(&self, start: usize, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Option<u64> {
        let (slots, first, last) = self.covering(start, width)?;
        let w = size_of::<T>();
        if last == first + 1 {
            let slot = &slots[first];
            let off = start - first * w;
            let mut cur = slot.load_bits(Ordering::SeqCst);
            loop {
                let mut bytes = elem_bytes(cur, w);
                let old = crate::native_memory::load_bits(&bytes[off..off + width]);
                let Some(n) = op(old) else { return Some(old) };
                crate::native_memory::store_bits(&mut bytes[off..off + width], n);
                if std::any::TypeId::of::<T>() == std::any::TypeId::of::<bool>() {
                    bytes[0] = (bytes[0] != 0) as u8;
                }
                match slot.cas_bits(cur, crate::native_memory::load_bits(&bytes[..w])) {
                    Ok(()) => return Some(old),
                    Err(actual) => cur = actual,
                }
            }
        }
        let _g = STRIPES[(slots.as_ptr() as usize >> 4) % STRIPES.len()].lock();
        let mut b = [0u8; 8];
        self.read_bytes(start, &mut b[..width]);
        let old = crate::native_memory::load_bits(&b[..width]);
        if let Some(n) = op(old) {
            crate::native_memory::store_bits(&mut b[..width], n);
            self.write_bytes(start, &b[..width]);
        }
        Some(old)
    }
}

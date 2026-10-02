//! Unsafe 基本类型访问的统一载体（`unsafe__impl` 的 get / put / CAS / getAndX 访问器族与
//! VarHandle 字段族共用）。
//!
//! 每次访问是「(基址, 偏移) 处 `width` 字节值的读-改-写」：值以零扩展的 u64 位形流转，
//! `op(旧)` 给新值即写入、给 None 即只读，返回旧值。按载体分派到同一存储的三种视图：
//! - 原生内存（基址 null 的绝对地址 / 基本类型数组，`arrayBaseOffset + i × arrayIndexScale`）：
//!   `native_memory::update` 的字节视图——数组在存储写锁内、直接内存经同宽原子指令；子字元素
//!   与按字对齐的 int 访问（JDK compareAndExchangeByte / Short 的 `offset & ~3` 掩码路径）落在
//!   同一字节序列上，相邻元素互不干扰；
//! - 静态字段（staticFieldBase + staticFieldOffset 的不透明 id）：`unsafe__impl::_static_rmw`（声明类字段闭包
//!   按装箱值、在静态读-改-写锁内原子完成；位形换算见 [`box_bits`]），与引用族静态臂同一载体；
//! - 实例字段（objectFieldOffset 的不透明 id）：ObjectVTable 的字 / 双字视图
//!   （`__unsafe_word` / `__unsafe_dword`，字段单元上的原子读-改-写）。
//!
//! 字段槽按 `FIELD_SLOT`（4 字节）对齐、各字段独占：宽度小于字的访问在字视图上按掩码合成
//! （只改低 `width` 字节，其余位原样），与原生内存形态同一口径。

use crate::prelude::*;

/// `width` 字节的位掩码。
fn mask(width: usize) -> u64 {
    if width >= 8 { u64::MAX } else { (1u64 << (width * 8)) - 1 }
}

/// 装箱字段值 → 位形（字段闭包读出的原生盒：boolean 0 / 1，整数零扩展，浮点原始位）。
fn box_bits(v: &Object) -> Option<u64> {
    let any = v.0.as_any();
    if let Some(b) = any.downcast_ref::<bool>() {
        return Some(*b as u64);
    }
    if let Some(b) = any.downcast_ref::<i8>() {
        return Some(*b as u8 as u64);
    }
    if let Some(b) = any.downcast_ref::<i16>() {
        return Some(*b as u16 as u64);
    }
    if let Some(b) = any.downcast_ref::<u16>() {
        return Some(*b as u64);
    }
    if let Some(b) = any.downcast_ref::<i32>() {
        return Some(*b as u32 as u64);
    }
    if let Some(b) = any.downcast_ref::<f32>() {
        return Some(b.to_bits() as u64);
    }
    if let Some(b) = any.downcast_ref::<i64>() {
        return Some(*b as u64);
    }
    if let Some(b) = any.downcast_ref::<f64>() {
        return Some(b.to_bits());
    }
    None
}

/// 位形写回装箱字段：按当前值的盒类型截断（只改该字段自身的槽）。
fn bits_boxed_like(cur: &Object, bits: u64) -> Object {
    let any = cur.0.as_any();
    if any.is::<bool>() {
        Object::from(bits & 0xFF != 0)
    } else if any.is::<i8>() {
        Object::from(bits as u8 as i8)
    } else if any.is::<i16>() {
        Object::from(bits as u16 as i16)
    } else if any.is::<u16>() {
        Object::from(bits as u16)
    } else if any.is::<f32>() {
        Object::from(f32::from_bits(bits as u32))
    } else if any.is::<i64>() {
        Object::from(bits as i64)
    } else if any.is::<f64>() {
        Object::from(f64::from_bits(bits))
    } else {
        Object::from(bits as u32 as i32)
    }
}

/// 宽度 `width` 的访问在字段视图（字 / 双字）上的掩码合成：op 看到低 `width` 字节，
/// 写回只替换这些字节。
fn masked(width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> impl FnMut(u64) -> Option<u64> + '_ {
    let m = mask(width);
    move |cur| op(cur & m).map(|n| (cur & !m) | (n & m))
}

/// 统一载体：`(o, offset)` 处 `width` 字节值的读-改-写（见模块头注），返回旧值（低 `width`
/// 字节，零扩展）。`what` 为访问器签名（缺口报文用）。
pub(super) fn prim(o: &Object, offset: i64, width: usize, what: &str,
                   op: &mut dyn FnMut(u64) -> Option<u64>) -> Result<u64> {
    if crate::native_memory::is_raw(o) {
        return crate::native_memory::update(o, offset, width, op);
    }
    let m = mask(width);
    {
        let mut op = masked(width, op);
        let mut boxed = |cur: Object| -> Option<Object> {
            let bits = box_bits(&cur)?;
            op(bits).map(|n| bits_boxed_like(&cur, n))
        };
        if let Some(r) = super::unsafe__impl::_static_rmw(offset, &mut boxed) {
            let bits = box_bits(&r?).ok_or_else(|| {
                JvmError::illegal_argument(&format!("Unsafe.{}: static field value type mismatch", what))
            })?;
            return Ok(bits & m);
        }
    }
    if let Some(field) = super::unsafe__impl::offset_field_name(offset) {
        let mut op = masked(width, op);
        let old = if width == 8 {
            o.0.__unsafe_dword(&field, &mut |c| op(c as u64).map(|n| n as i64)).map(|c| c as u64)
        } else {
            o.0.__unsafe_word(&field, &mut |c| op(c as u32 as u64).map(|n| n as u32 as i32)).map(|c| c as u32 as u64)
        };
        if let Some(old) = old {
            return Ok(old & m);
        }
    }
    panic!("stub: jdk/internal/misc/Unsafe.{} (offset={} 无共享存储：非基本类型数组 / 非静态字段 id / 运行时类无该平铺基本类型字段)", what, offset)
}

/// 读（`op` 恒 None）。
pub(super) fn get(o: &Object, offset: i64, width: usize, what: &str) -> Result<u64> {
    prim(o, offset, width, what, &mut |_| None)
}

/// 无条件写。
pub(super) fn put(o: &Object, offset: i64, width: usize, what: &str, v: u64) -> Result<()> {
    prim(o, offset, width, what, &mut |_| Some(v)).map(|_| ())
}

/// 比较交换：当前值（低 `width` 字节）等于 `expected` 则写 `x`；返回见证值（交换前的当前值）。
pub(super) fn cas(o: &Object, offset: i64, width: usize, what: &str, expected: u64, x: u64) -> Result<u64> {
    let e = expected & mask(width);
    prim(o, offset, width, what, &mut |c| (c == e).then_some(x))
}

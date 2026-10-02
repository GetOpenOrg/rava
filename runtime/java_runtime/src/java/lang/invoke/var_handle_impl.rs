//! `java/lang/invoke/VarHandle` 手写伴生：公开 API 类的 ACC_NATIVE 方法
//! （K-3a 共置形态）。`get` / `set` / `getAndSet` 等签名多态方法是 HotSpot
//! 内建（按 VarHandle 的具体 flavor 分派到内联访问器）；原生二进制无内建
//! ——按运行时类名（flavor）识别家族与元素族（`var_handle_ext.rs`）后路由：
//!   - Field 家族（$FieldInstance* / $FieldStatic*）：flavor 的 `fieldOffset`
//!     （出自 MethodHandleNatives.objectFieldOffset / staticFieldOffset 与 Unsafe
//!     同一登记表）经 ObjectVTable 的 `__unsafe_long_cell("fieldOffset")` 按名协议
//!     直读；静态 flavor 的基址取 `base` 字段（args 不含 holder）。读写按元素族
//!     落到 Unsafe 的同族访问器——plain 档走 `get/putX`，volatile / acquire /
//!     release / opaque 档走 `get/putXVolatile`（JDK `VarHandleXs$FieldInstance*`
//!     字节码的同一调用目标），存储即字段闭包的共享单元，写入对直接字段读取可见；
//!   - Array 家族（*Array*）：见 `var_handle_ext.rs`。
//!
//! CAS 族（compareAndSet/weakCompareAndSet*/compareAndExchange*/getAndSet*/getAndAdd*）
//! 的读-比-写在字段存储单元内原子完成（引用槽写锁 / int、long 原子单元）。

use crate::prelude::*;
use super::var_handle::VarHandle;
use super::var_handle_ext::*;
use crate::jdk::internal::misc::Unsafe;

/// 本 VarHandle 的字段偏移（不透明 id）。Field 家族 flavor 的 `fieldOffset` 是非擦除
/// 平铺 long 字段，经 `__unsafe_long_cell` 按名协议直读；非 Field flavor → None。
fn _field_offset(vh: &VarHandle) -> Option<i64> {
    Object::from(Clone::clone(vh))
        .0
        .__unsafe_long_cell("fieldOffset")
        .map(|c| c.get())
}

/// Field 家族的访问坐标：(基址, 偏移, 首个值实参下标)。实例 flavor 的基址是
/// args[0]；静态 flavor（`$FieldStatic*`）的基址是 flavor 的 `base` 字段，args 只有值。
fn _field_coords(vh: &VarHandle, args: &JArray<Object>) -> Result<(Object, i64, i32)> {
    let offset = _field_offset(vh).ok_or_else(_state_err)?;
    let o = Object::from(Clone::clone(vh));
    if o.0.__class_name().contains("$FieldStatic") {
        let base = o.0.__unsafe_ref_get("base").ok_or_else(_state_err)?;
        Ok((base, offset, 0))
    } else {
        Ok((args.get(0)?, offset, 1))
    }
}

fn _field_read(c: _Carrier, holder: &Object, off: i64, volatile: bool) -> Result<Object> {
    let u = Unsafe::getUnsafe()?;
    let h = Clone::clone(holder);
    Ok(match (c, volatile) {
        (_Carrier::Ref, false) => u.getReference(h, off)?,
        (_Carrier::Ref, true) => u.getReferenceVolatile(h, off)?,
        (_Carrier::Bool, false) => Object::from(u.getBoolean(h, off)?),
        (_Carrier::Bool, true) => Object::from(u.getBooleanVolatile(h, off)?),
        (_Carrier::Byte, false) => Object::from(u.getByte_obj_l(h, off)?),
        (_Carrier::Byte, true) => Object::from(u.getByteVolatile(h, off)?),
        (_Carrier::Short, false) => Object::from(u.getShort_obj_l(h, off)?),
        (_Carrier::Short, true) => Object::from(u.getShortVolatile(h, off)?),
        (_Carrier::Char, false) => Object::from(u.getChar_obj_l(h, off)?),
        (_Carrier::Char, true) => Object::from(u.getCharVolatile(h, off)?),
        (_Carrier::Int, false) => Object::from(u.getInt_obj_l(h, off)?),
        (_Carrier::Int, true) => Object::from(u.getIntVolatile(h, off)?),
        (_Carrier::Long, false) => Object::from(u.getLong_obj_l(h, off)?),
        (_Carrier::Long, true) => Object::from(u.getLongVolatile(h, off)?),
        (_Carrier::Float, false) => Object::from(u.getFloat_obj_l(h, off)?),
        (_Carrier::Float, true) => Object::from(u.getFloatVolatile(h, off)?),
        (_Carrier::Double, false) => Object::from(u.getDouble_obj_l(h, off)?),
        (_Carrier::Double, true) => Object::from(u.getDoubleVolatile(h, off)?),
    })
}

fn _field_write(c: _Carrier, holder: &Object, off: i64, v: &Object, volatile: bool) -> Result<()> {
    let u = Unsafe::getUnsafe()?;
    let h = Clone::clone(holder);
    if c == _Carrier::Ref {
        let v = Clone::clone(v);
        return if volatile { u.putReferenceVolatile(h, off, v) } else { u.putReference(h, off, v) };
    }
    let b = _bits(c, v).ok_or_else(|| _bad_arg("bad value form"))?;
    match (c, volatile) {
        (_Carrier::Bool, false) => u.putBoolean(h, off, b != 0),
        (_Carrier::Bool, true) => u.putBooleanVolatile(h, off, b != 0),
        (_Carrier::Byte, false) => u.putByte_obj_l_b(h, off, b as u8 as i8),
        (_Carrier::Byte, true) => u.putByteVolatile(h, off, b as u8 as i8),
        (_Carrier::Short, false) => u.putShort_obj_l_s(h, off, b as u16 as i16),
        (_Carrier::Short, true) => u.putShortVolatile(h, off, b as u16 as i16),
        (_Carrier::Char, false) => u.putChar_obj_l_c(h, off, b as u16),
        (_Carrier::Char, true) => u.putCharVolatile(h, off, b as u16),
        (_Carrier::Int, false) => u.putInt_obj_l_i(h, off, b as u32 as i32),
        (_Carrier::Int, true) => u.putIntVolatile(h, off, b as u32 as i32),
        (_Carrier::Long, false) => u.putLong_obj_l_l(h, off, b as i64),
        (_Carrier::Long, true) => u.putLongVolatile(h, off, b as i64),
        (_Carrier::Float, false) => u.putFloat_obj_l_f(h, off, f32::from_bits(b as u32)),
        (_Carrier::Float, true) => u.putFloatVolatile(h, off, f32::from_bits(b as u32)),
        (_Carrier::Double, false) => u.putDouble_obj_l_d(h, off, f64::from_bits(b)),
        (_Carrier::Double, true) => u.putDoubleVolatile(h, off, f64::from_bits(b)),
        (_Carrier::Ref, _) => unreachable!(),
    }
}

/// 交换语义（getAndSet / compareAndExchange 共用）：读旧值，可选比较（Some：不「相同」
/// 则不写，返回当前值——exchange 的见证形态；None：无条件换），写新值，返回旧值。
/// 读-比-写在字段存储单元内原子完成：引用槽写锁 / int、long 原子单元。
fn _field_exchange(c: _Carrier, holder: &Object, off: i64, expected: Option<&Object>, new: &Object) -> Result<Object> {
    let u = Unsafe::getUnsafe()?;
    let e = match expected {
        Some(e) => Some(_norm(c, e)?),
        None => None,
    };
    let nv = _norm(c, new)?;
    match c {
        _Carrier::Ref => {
            let mut nv = Some(nv);
            u.__vh_ref_update(holder, off, &mut |cur| match &e {
                Some(e) if !_same(c, &cur, e) => None,
                _ => nv.take(),
            }).ok_or_else(_state_err)
        }
        _Carrier::Long | _Carrier::Int => {
            let eb = e.as_ref().and_then(|e| _bits(c, e));
            let vb = _bits(c, &nv).ok_or_else(|| _bad_arg("bad value form"))?;
            let old = if c == _Carrier::Long {
                u.__vh_long_update(holder, off, |cur| match eb {
                    Some(e) if cur as u64 != e => cur,
                    _ => vb as i64,
                }).map(|o| o as u64)
            } else {
                u.__vh_int_update(holder, off, |cur| match eb {
                    Some(e) if cur as u32 as u64 != e => cur,
                    _ => vb as u32 as i32,
                }).map(|o| o as u32 as u64)
            };
            Ok(_box(c, old.ok_or_else(_state_err)?))
        }
        _ => panic!("stub: java/lang/invoke/VarHandle 字段读-比-写（{:?} 族无共享原子单元协议）", c),
    }
}

fn _field_cas(c: _Carrier, holder: &Object, off: i64, expected: &Object, new: &Object) -> Result<bool> {
    let witness = _field_exchange(c, holder, off, Some(expected), new)?;
    Ok(_same(c, &witness, &_norm(c, expected)?))
}

impl VarHandle {
    /// native `get(Object...)`：plain 读（单线程下与 volatile 同一存储单元）。
    pub fn get(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_get(self, &args);
        }
        let (h, off, _) = _field_coords(self, &args)?;
        _field_read(_carrier(self), &h, off, false)
    }

    /// native `set(Object...)`：plain 写。
    pub fn set(&self, args: JArray<Object>) -> Result<()> {
        if _is_array_flavor(self) {
            return _array_set(self, &args);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        _field_write(_carrier(self), &h, off, &args.get(vi)?, false)
    }

    /// native `getVolatile(Object...)`：volatile 读。
    pub fn getVolatile(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_get(self, &args);
        }
        let (h, off, _) = _field_coords(self, &args)?;
        _field_read(_carrier(self), &h, off, true)
    }

    /// native `setVolatile(Object...)`：volatile 写。
    pub fn setVolatile(&self, args: JArray<Object>) -> Result<()> {
        if _is_array_flavor(self) {
            return _array_set(self, &args);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        _field_write(_carrier(self), &h, off, &args.get(vi)?, true)
    }

    /// native `getAcquire(Object...)`：acquire 读（单线程档位与 plain 同单元）。
    pub fn getAcquire(&self, args: JArray<Object>) -> Result<Object> {
        self.getVolatile(args)
    }

    /// native `setRelease(Object...)`：release 写。
    pub fn setRelease(&self, args: JArray<Object>) -> Result<()> {
        self.setVolatile(args)
    }

    /// native `getOpaque(Object...)`：opaque 读。
    pub fn getOpaque(&self, args: JArray<Object>) -> Result<Object> {
        self.getVolatile(args)
    }

    /// native `setOpaque(Object...)`：opaque 写。
    pub fn setOpaque(&self, args: JArray<Object>) -> Result<()> {
        self.setVolatile(args)
    }

    /// native `compareAndSet(Object...)`：CAS（args = [holder, expected, new]），读-比-写在
    /// 存储单元内原子完成。
    pub fn compareAndSet(&self, args: JArray<Object>) -> Result<bool> {
        if _is_array_flavor(self) {
            return _array_cas(self, &args);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        _field_cas(_carrier(self), &h, off, &args.get(vi)?, &args.get(vi + 1)?)
    }

    /// native `weakCompareAndSet(Object...)`：weak CAS（无竞争下与非 weak 同义）。
    pub fn weakCompareAndSet(&self, args: JArray<Object>) -> Result<bool> {
        self.compareAndSet(args)
    }

    /// native `weakCompareAndSetPlain(Object...)`：weak plain CAS。
    pub fn weakCompareAndSetPlain(&self, args: JArray<Object>) -> Result<bool> {
        self.compareAndSet(args)
    }

    /// native `weakCompareAndSetAcquire(Object...)`：weak acquire CAS。
    pub fn weakCompareAndSetAcquire(&self, args: JArray<Object>) -> Result<bool> {
        self.compareAndSet(args)
    }

    /// native `weakCompareAndSetRelease(Object...)`：weak release CAS。
    pub fn weakCompareAndSetRelease(&self, args: JArray<Object>) -> Result<bool> {
        self.compareAndSet(args)
    }

    /// native `compareAndExchange(Object...)`：交换并返回旧值（见证）。
    pub fn compareAndExchange(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_exchange(self, &args, true);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        _field_exchange(_carrier(self), &h, off, Some(&args.get(vi)?), &args.get(vi + 1)?)
    }

    /// native `compareAndExchangeAcquire(Object...)`：acquire 交换。
    pub fn compareAndExchangeAcquire(&self, args: JArray<Object>) -> Result<Object> {
        self.compareAndExchange(args)
    }

    /// native `compareAndExchangeRelease(Object...)`：release 交换。
    pub fn compareAndExchangeRelease(&self, args: JArray<Object>) -> Result<Object> {
        self.compareAndExchange(args)
    }

    /// native `getAndSet(Object...)`：原子交换（旧值返回）。
    pub fn getAndSet(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_exchange(self, &args, false);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        _field_exchange(_carrier(self), &h, off, None, &args.get(vi)?)
    }

    /// native `getAndSetAcquire(Object...)`：acquire 交换。
    pub fn getAndSetAcquire(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndSet(args)
    }

    /// native `getAndSetRelease(Object...)`：release 交换。
    pub fn getAndSetRelease(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndSet(args)
    }

    /// native `getAndAdd(Object...)`：原子加（args = [holder?, delta] / [array, index, delta]），
    /// 返回旧值。数组元素在存储写锁内一次读-改-写；字段经读-CAS 循环（无竞争下一轮完成，
    /// 有竞争时重读重试——与 Unsafe.getAndAddInt 的字节码语义同）。引用 / 布尔族
    /// 无此操作（UnsupportedOperationException）。
    pub fn getAndAdd(&self, args: JArray<Object>) -> Result<Object> {
        let c = _carrier(self);
        if matches!(c, _Carrier::Ref | _Carrier::Bool) {
            return Err(_unsupported("getAndAdd"));
        }
        if _is_array_flavor(self) {
            return _array_get_and_add(self, &args);
        }
        let (h, off, vi) = _field_coords(self, &args)?;
        let delta = args.get(vi)?;
        loop {
            let cur = _field_read(c, &h, off, true)?;
            let new = _add(c, &cur, &delta)?;
            if _field_cas(c, &h, off, &cur, &new)? {
                return Ok(cur);
            }
        }
    }

    /// native `getAndAddAcquire(Object...)`：acquire 档位（单元内同 getAndAdd）。
    pub fn getAndAddAcquire(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndAdd(args)
    }

    /// native `getAndAddRelease(Object...)`：release 档位。
    pub fn getAndAddRelease(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndAdd(args)
    }
}

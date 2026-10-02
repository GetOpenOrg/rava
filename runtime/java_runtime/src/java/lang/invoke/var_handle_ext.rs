//! `java/lang/invoke/VarHandle` 手写伴生之二：元素族载体与 Array 家族
//! （`var_handle_impl.rs` 的 Field 家族与签名多态入口共用）。
//!
//! 元素族按 flavor 运行时类名识别（`VarHandle{References,Booleans,Bytes,Shorts,
//! Chars,Ints,Longs,Floats,Doubles}$…`，与 JDK `VarHandles.makeFieldHandle` /
//! `makeArrayElementHandle` 的 flavor 一一对应）。数值族的值在本层统一以「位形」
//! u64 承载：比较按位形（JDK 浮点 CAS 比较 `floatToRawIntBits` / `doubleToRawLongBits`），
//! 回装为原生盒（`Object::from(i8/i16/u16/i32/i64/f32/f64/bool)`）。

use crate::prelude::*;
use super::var_handle::VarHandle;
use crate::reflect_dispatch::{unbox_bool, unbox_char, unbox_f32, unbox_f64, unbox_i32, unbox_i64};

/// 元素族。
#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) enum _Carrier { Ref, Bool, Byte, Short, Char, Int, Long, Float, Double }

pub(super) fn _carrier(vh: &VarHandle) -> _Carrier {
    let o = Object::from(Clone::clone(vh));
    let cn = o.0.__class_name();
    const FAMILIES: [(&str, _Carrier); 9] = [
        ("VarHandleReferences$", _Carrier::Ref), ("VarHandleBooleans$", _Carrier::Bool),
        ("VarHandleBytes$", _Carrier::Byte), ("VarHandleShorts$", _Carrier::Short),
        ("VarHandleChars$", _Carrier::Char), ("VarHandleInts$", _Carrier::Int),
        ("VarHandleLongs$", _Carrier::Long), ("VarHandleFloats$", _Carrier::Float),
        ("VarHandleDoubles$", _Carrier::Double),
    ];
    FAMILIES.iter().find(|(p, _)| cn.contains(p)).map(|(_, c)| *c).unwrap_or(_Carrier::Int)
}

/// 包装值 → 位形（形态不符 / null → None）。引用族无位形。
pub(super) fn _bits(c: _Carrier, v: &Object) -> Option<u64> {
    Some(match c {
        _Carrier::Ref => return None,
        _Carrier::Bool => unbox_bool(v)? as u64,
        _Carrier::Byte => unbox_i32(v)? as i8 as u8 as u64,
        _Carrier::Short => unbox_i32(v)? as i16 as u16 as u64,
        _Carrier::Char => unbox_char(v)? as u64,
        _Carrier::Int => unbox_i32(v)? as u32 as u64,
        _Carrier::Long => unbox_i64(v)? as u64,
        _Carrier::Float => unbox_f32(v)?.to_bits() as u64,
        _Carrier::Double => unbox_f64(v)?.to_bits(),
    })
}

/// 位形 → 原生盒。
pub(super) fn _box(c: _Carrier, bits: u64) -> Object {
    match c {
        _Carrier::Ref | _Carrier::Int => Object::from(bits as u32 as i32),
        _Carrier::Bool => Object::from(bits != 0),
        _Carrier::Byte => Object::from(bits as u8 as i8),
        _Carrier::Short => Object::from(bits as u16 as i16),
        _Carrier::Char => Object::from(bits as u16),
        _Carrier::Long => Object::from(bits as i64),
        _Carrier::Float => Object::from(f32::from_bits(bits as u32)),
        _Carrier::Double => Object::from(f64::from_bits(bits)),
    }
}

/// 值规整：引用原样；数值按本族拆箱再装原生盒（形态不符 → IllegalArgumentException）。
pub(super) fn _norm(c: _Carrier, v: &Object) -> Result<Object> {
    if c == _Carrier::Ref {
        return Ok(Clone::clone(v));
    }
    _bits(c, v).map(|b| _box(c, b)).ok_or_else(|| _bad_arg("bad value form"))
}

/// CAS 的「相同」：引用按 Java `==`（对象身份 / null），数值按位形。
pub(super) fn _same(c: _Carrier, a: &Object, b: &Object) -> bool {
    match c {
        _Carrier::Ref => a == b,
        _ => _bits(c, a).is_some() && _bits(c, a) == _bits(c, b),
    }
}

/// getAndAdd 的新值：cur + delta（整数族按本族宽度回绕，浮点按 IEEE 加法）。
/// 引用 / 布尔族无此操作（JDK：UnsupportedOperationException）。
pub(super) fn _add(c: _Carrier, cur: &Object, delta: &Object) -> Result<Object> {
    let bad = || _bad_arg("bad numeric value form");
    Ok(match c {
        _Carrier::Ref | _Carrier::Bool => return Err(_unsupported("getAndAdd")),
        _Carrier::Byte => Object::from((unbox_i32(cur).ok_or_else(bad)? as i8).wrapping_add(unbox_i32(delta).ok_or_else(bad)? as i8)),
        _Carrier::Short => Object::from((unbox_i32(cur).ok_or_else(bad)? as i16).wrapping_add(unbox_i32(delta).ok_or_else(bad)? as i16)),
        _Carrier::Char => Object::from(unbox_char(cur).ok_or_else(bad)?.wrapping_add(unbox_char(delta).ok_or_else(bad)?)),
        _Carrier::Int => Object::from(unbox_i32(cur).ok_or_else(bad)?.wrapping_add(unbox_i32(delta).ok_or_else(bad)?)),
        _Carrier::Long => Object::from(unbox_i64(cur).ok_or_else(bad)?.wrapping_add(unbox_i64(delta).ok_or_else(bad)?)),
        _Carrier::Float => Object::from(unbox_f32(cur).ok_or_else(bad)? + unbox_f32(delta).ok_or_else(bad)?),
        _Carrier::Double => Object::from(unbox_f64(cur).ok_or_else(bad)? + unbox_f64(delta).ok_or_else(bad)?),
    })
}

pub(super) fn _bad_arg(what: &str) -> JvmError {
    JvmError::illegal_argument(what)
}

/// 访问模式不被本元素族支持（JDK `UnsupportedOperationException`）。
pub(super) fn _unsupported(mode: &str) -> JvmError {
    match crate::java::lang::UnsupportedOperationException::new_str(String::from(mode)) {
        Ok(e) => JvmError::from(e),
        Err(nested) => nested,
    }
}

/// 无字段偏移形态（非 Field flavor）的失败错误。
pub(super) fn _state_err() -> JvmError {
    match crate::java::lang::IllegalStateException::new() {
        Ok(e) => JvmError::from(e),
        Err(nested) => nested,
    }
}

/// args[i] → 下标。
pub(super) fn _arg_index(args: &JArray<Object>, i: i32) -> Result<i32> {
    unbox_i32(&args.get(i)?).ok_or_else(|| _bad_arg("bad array index form"))
}

// ── Array 家族（args = [数组, 下标, 值...]）──────────────────────────────────

/// flavor（运行时类名）是否 Array 家族（含字节数组视图族）。
pub(super) fn _is_array_flavor(vh: &VarHandle) -> bool {
    let o = Object::from(Clone::clone(vh));
    let cn = o.0.__class_name();
    cn.contains("$Array")
}

/// 元素的读-改-写：在数组存储写锁内读出旧值 `cur`，`f(cur)` 返回 `Some(new)` 时写入，
/// 返回旧值。数组按元素族还原具体元素类型（引用元素走协变视图）。
fn _rmw_typed<T>(arr: &Object, desc: &str, idx: i32,
                 f: &mut dyn FnMut(Object) -> Option<Object>) -> Result<Object>
where T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe {
    let a = arr.try_cast_array::<T>(desc)?;
    Ok(a.__update(idx, &mut |cur: T| f(cur.into()).map(T::from))?.into())
}

pub(super) fn _array_rmw(c: _Carrier, arr: &Object, idx: i32,
                         f: &mut dyn FnMut(Object) -> Option<Object>) -> Result<Object> {
    match c {
        _Carrier::Ref => _rmw_typed::<Object>(arr, "[Ljava/lang/Object;", idx, f),
        _Carrier::Bool => _rmw_typed::<bool>(arr, "[Z", idx, f),
        _Carrier::Byte => _rmw_typed::<i8>(arr, "[B", idx, f),
        _Carrier::Short => _rmw_typed::<i16>(arr, "[S", idx, f),
        _Carrier::Char => _rmw_typed::<u16>(arr, "[C", idx, f),
        _Carrier::Int => _rmw_typed::<i32>(arr, "[I", idx, f),
        _Carrier::Long => _rmw_typed::<i64>(arr, "[J", idx, f),
        _Carrier::Float => _rmw_typed::<f32>(arr, "[F", idx, f),
        _Carrier::Double => _rmw_typed::<f64>(arr, "[D", idx, f),
    }
}

pub(super) fn _array_get(vh: &VarHandle, args: &JArray<Object>) -> Result<Object> {
    if let Some(r) = _view_get(vh, args) {
        return r;
    }
    _array_rmw(_carrier(vh), &args.get(0)?, _arg_index(args, 1)?, &mut |_| None)
}

pub(super) fn _array_set(vh: &VarHandle, args: &JArray<Object>) -> Result<()> {
    if let Some(r) = _view_set(vh, args) {
        return r;
    }
    let c = _carrier(vh);
    let mut v = Some(_norm(c, &args.get(2)?)?);
    _array_rmw(c, &args.get(0)?, _arg_index(args, 1)?, &mut |_| v.take()).map(|_| ())
}

/// 交换语义：args = [数组, 下标, (expected,) 新值]；expected 给出且与当前值不「相同」→
/// 不写；返回旧值（见证）。
pub(super) fn _array_exchange(vh: &VarHandle, args: &JArray<Object>, expected: bool) -> Result<Object> {
    let c = _carrier(vh);
    let e = if expected { Some(_norm(c, &args.get(2)?)?) } else { None };
    let mut v = Some(_norm(c, &args.get(if expected { 3 } else { 2 })?)?);
    _array_rmw(c, &args.get(0)?, _arg_index(args, 1)?, &mut |cur| match &e {
        Some(e) if !_same(c, &cur, e) => None,
        _ => v.take(),
    })
}

pub(super) fn _array_cas(vh: &VarHandle, args: &JArray<Object>) -> Result<bool> {
    let c = _carrier(vh);
    let witness = _array_exchange(vh, args, true)?;
    Ok(_same(c, &witness, &_norm(c, &args.get(2)?)?))
}

/// 数组元素 getAndAdd：读-改-写在存储写锁内一次完成。
pub(super) fn _array_get_and_add(vh: &VarHandle, args: &JArray<Object>) -> Result<Object> {
    let c = _carrier(vh);
    let delta = args.get(2)?;
    let mut err = None;
    let old = _array_rmw(c, &args.get(0)?, _arg_index(args, 1)?, &mut |cur| match _add(c, &cur, &delta) {
        Ok(n) => Some(n),
        Err(e) => {
            err = Some(e);
            None
        }
    })?;
    match err {
        Some(e) => Err(e),
        None => Ok(old),
    }
}

// ── 字节数组视图族（MethodHandles.byteArrayViewVarHandle：VarHandleByteArrayAs*$ArrayHandle）──
// args = [byte[], 字节偏移, 值...]：在 byte[] 上按视图元素宽度、以 flavor 的 `be` 字段
// 给定的字节序读写一个 short/char/int/long/float/double（JDK 语义：越界检查
// `Preconditions.checkIndex(index, length - (width - 1))` → ArrayIndexOutOfBoundsException）。
// 消费方：sun.security.provider.ByteArrayAccess.LE/BE（MD5 / SHA 的字与字节块转换）。

fn _byte_view(vh: &VarHandle) -> Option<(_Carrier, usize, bool)> {
    let o = Object::from(Clone::clone(vh));
    let cn = o.0.__class_name();
    if !cn.contains("VarHandleByteArrayAs") || !cn.ends_with("$ArrayHandle") {
        return None;
    }
    let (kind, width) = if cn.contains("AsShorts") { (_Carrier::Short, 2) }
        else if cn.contains("AsChars") { (_Carrier::Char, 2) }
        else if cn.contains("AsInts") { (_Carrier::Int, 4) }
        else if cn.contains("AsLongs") { (_Carrier::Long, 8) }
        else if cn.contains("AsFloats") { (_Carrier::Float, 4) }
        else if cn.contains("AsDoubles") { (_Carrier::Double, 8) }
        else { return None };
    let be = o.0.__unsafe_bool_cell("be").map(|c| c.get()).unwrap_or(true);
    Some((kind, width, be))
}

/// 字节数组 + 偏移（越界 → AIOOBE，长度按 JDK 口径 `length - (width - 1)`）。
fn _view_target(args: &JArray<Object>, width: usize) -> Result<(JArray<i8>, usize)> {
    let ba = Clone::clone(&args.get(0)?).try_cast_array::<i8>("[B")?;
    let off = unbox_i32(&args.get(1)?).ok_or_else(|| _bad_arg("bad byte array view index form"))?;
    let limit = ba.len()? - (width as i32 - 1);
    if off < 0 || off >= limit {
        return Err(JvmError::array_index_out_of_bounds(off, limit.max(0)));
    }
    Ok((ba, off as usize))
}

fn _view_get(vh: &VarHandle, args: &JArray<Object>) -> Option<Result<Object>> {
    let (kind, width, be) = _byte_view(vh)?;
    Some((|| {
        let (ba, off) = _view_target(args, width)?;
        let mut bits: u64 = 0;
        for k in 0..width {
            let i = if be { k } else { width - 1 - k };
            bits = (bits << 8) | (ba.get((off + i) as i32)? as u8 as u64);
        }
        Ok(_box(kind, bits))
    })())
}

fn _view_set(vh: &VarHandle, args: &JArray<Object>) -> Option<Result<()>> {
    let (kind, width, be) = _byte_view(vh)?;
    Some((|| {
        let (ba, off) = _view_target(args, width)?;
        let bits = _bits(kind, &args.get(2)?).ok_or_else(|| _bad_arg("bad byte array view element form"))?;
        for k in 0..width {
            let shift = 8 * (width - 1 - k);
            let i = if be { k } else { width - 1 - k };
            ba.set((off + i) as i32, (bits >> shift) as u8 as i8)?;
        }
        Ok(())
    })())
}

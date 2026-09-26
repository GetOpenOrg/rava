//! `java/lang/invoke/VarHandle` 手写伴生：公开 API 类的 ACC_NATIVE 方法
//! （K-3a 共置形态）。`get` / `set` / `getAndSet` 等签名多态方法是 HotSpot
//! 内建（按 VarHandle 的具体 flavor 分派到内联访问器）；原生二进制无内建
//! ——按运行时类名（flavor）识别元素族后路由到既有擦除协议：
//!   - Field 家族（$FieldInstance*）：flavor 的 `fieldOffset`（实例字段的不
//!     透明 id，出自 MethodHandleNatives.objectFieldOffset 与 Unsafe 同一
//!     登记表）是非擦除平铺 long 字段——经 ObjectVTable 的
//!     `__unsafe_long_cell("fieldOffset")` 按名协议直读（inner 平铺持有全部
//!     继承字段，无需按元素族还原具体静态视图）；目标字段的访问按元素族
//!     分派：
//!       - 引用族（VarHandleReferences$）：经 Unsafe 的 `__vh_ref_get/set`
//!         （登记表反查字段名 + ObjectVTable 引用原子协议，存储即擦除载体
//!         的 `Rc<RefCell<Option<Box<T>>>>`，与直接字段读取同一单元）；
//!       - int 族（Ints$/Booleans$）经 Unsafe volatile int 访问器、long 族
//!         （Longs$）经 volatile long 访问器（`__unsafe_int/long_cell` 的
//!         共享单元——写入对直接字段读取可见）；
//!   - Array 家族（*Array*）：args[0] 是数组本体、args[1] 是下标，经
//!     try_cast_array 还元素类型后直接读写（引用元素数组走协变视图）。
//! 元素装箱形态：翻译层的 `i32/bool → Object` 装箱（Integer/Boolean 包装），
//! 回收按 toString 解析（数值/布尔字面量即十进制文本，解析无损）。
//!
//! volatile / acquire / release / opaque 与 plain 的访问序差异在
//! GIL 下无跨线程可见性区别——同一存储单元，锁的获取 / 释放建立 happens-before（#42）；CAS 族
//! （compareAndSet/weakCompareAndSet/compareAndExchange/getAndSet）为
//! 读-比-写三步，无并发穿插即不可分割（与 Unsafe.getAndAddInt、
//! compareAndSetReference 的语义承载同族）。weak 与非 weak 在无竞争下同义。

use crate::prelude::*;
use super::var_handle::VarHandle;
use crate::jdk::internal::misc::Unsafe;

/// 元素族（按运行时类名的 flavor 段识别）。
#[derive(Clone, Copy, PartialEq)]
enum _Carrier {
    /// 引用族（VarHandleReferences$）：值即 Object（含 null）。
    Ref,
    /// long 族（VarHandleLongs$）：值装箱为 Long 文本。
    Long,
    /// int 族（VarHandleInts$/VarHandleBooleans$，缺省）：值装箱为
    /// Integer/Boolean 文本。
    Int,
}

fn _carrier(vh: &VarHandle) -> _Carrier {
    let cn = Object::from(Clone::clone(vh)).0.__class_name();
    if cn.contains("VarHandleReferences$") {
        _Carrier::Ref
    } else if cn.contains("VarHandleLongs$") {
        _Carrier::Long
    } else {
        _Carrier::Int
    }
}

/// 包装值 → i32（Integer 十进制文本 / Boolean 字面量；其余 → None）。
fn _unbox_i32(v: &Object) -> Option<i32> {
    if v.0.is_jvm_null() {
        return None;
    }
    match v.0.__class_name() {
        "java/lang/Integer" => v.0.__obj_str().parse::<i32>().ok(),
        "java/lang/Boolean" => Some(if v.0.__obj_str() == "true" { 1 } else { 0 }),
        _ => None,
    }
}

/// 包装值 → i64（Long/Integer 十进制文本；其余 → None；int 位形无损放宽）。
fn _unbox_i64(v: &Object) -> Option<i64> {
    if v.0.is_jvm_null() {
        return None;
    }
    match v.0.__class_name() {
        "java/lang/Long" | "java/lang/Integer" => v.0.__obj_str().parse::<i64>().ok(),
        _ => None,
    }
}

fn _bad_arg(what: &str) -> JvmError {
    match crate::java::lang::IllegalArgumentException::new_str(String::from(what)) {
        Ok(e) => JvmError::from(e),
        Err(nested) => nested,
    }
}

/// args[i] → i32（形态不符 → IllegalArgumentException，与 JDK 传递异常同型）。
fn _arg_i32(args: &JArray<Object>, i: i32, what: &str) -> Result<i32> {
    match _unbox_i32(&args.get(i)?) {
        Some(v) => Ok(v),
        None => Err(_bad_arg(what)),
    }
}

/// args[i] → i64（`_arg_i32` 的 long 镜像）。
fn _arg_i64(args: &JArray<Object>, i: i32, what: &str) -> Result<i64> {
    match _unbox_i64(&args.get(i)?) {
        Some(v) => Ok(v),
        None => Err(_bad_arg(what)),
    }
}

/// 无字段偏移形态（Array 家族 / 非 Field flavor）的失败错误。
fn _state_err() -> JvmError {
    match crate::java::lang::IllegalStateException::new() {
        Ok(e) => JvmError::from(e),
        Err(nested) => nested,
    }
}

/// 本 VarHandle 的实例字段偏移（不透明 id）。Field 家族 flavor 的
/// `fieldOffset` 是非擦除平铺 long 字段（inner 平铺持有继承字段），经
/// `__unsafe_long_cell` 按名协议直读——与元素族无关，全部 Field flavor
/// （Ints/Longs/Booleans/References/...）统一命中。Array 家族 / 非 Field
/// flavor（无该字段臂）→ None。
fn _field_offset(vh: &VarHandle) -> Option<i64> {
    Object::from(Clone::clone(vh))
        .0
        .__unsafe_long_cell("fieldOffset")
        .map(|c| c.get())
}

/// flavor（运行时类名）是否 Array 家族。
fn _is_array_flavor(vh: &VarHandle) -> bool {
    Object::from(Clone::clone(vh)).0.__class_name().contains("$Array")
}

// ── Field 家族按元素族的读 / 写 / CAS 原语（共享存储单元，无并发穿插） ──────

fn _field_read(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, volatile: bool) -> Result<Object> {
    match carrier {
        _Carrier::Ref => match u.__vh_ref_get(holder, offset) {
            Some(v) => Ok(v),
            None => Err(_state_err()),
        },
        _Carrier::Long => Ok(Object::from(if volatile {
            u.getLongVolatile(Clone::clone(holder), offset)?
        } else {
            u.getLong_obj_l(Clone::clone(holder), offset)?
        })),
        _Carrier::Int => Ok(Object::from(if volatile {
            u.getIntVolatile(Clone::clone(holder), offset)?
        } else {
            u.getInt_obj_l(Clone::clone(holder), offset)?
        })),
    }
}

fn _field_write(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, v: &Object, volatile: bool) -> Result<()> {
    match carrier {
        _Carrier::Ref => {
            if u.__vh_ref_set(holder, offset, Clone::clone(v)) {
                Ok(())
            } else {
                Err(_state_err())
            }
        }
        _Carrier::Long => {
            let x = match _unbox_i64(v) {
                Some(x) => x,
                None => return Err(_bad_arg("bad value form")),
            };
            if volatile {
                u.putLongVolatile(Clone::clone(holder), offset, x)
            } else {
                u.putLong_obj_l_l(Clone::clone(holder), offset, x)
            }
        }
        _Carrier::Int => {
            let x = match _unbox_i32(v) {
                Some(x) => x,
                None => return Err(_bad_arg("bad value form")),
            };
            if volatile {
                u.putIntVolatile(Clone::clone(holder), offset, x)
            } else {
                u.putInt_obj_l_i(Clone::clone(holder), offset, x)
            }
        }
    }
}

/// 读-比-写（GIL 下（持锁线程独占执行，#42）不可分割）。引用比较按 Java `==`（null 与
/// 对象身份，`PartialEq for Object`）；数值族经 Unsafe 的 int/long CAS。
fn _field_cas(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, expected: &Object, new: &Object) -> Result<bool> {
    let witness = _field_exchange(u, carrier, holder, offset, Some(expected), new)?;
    Ok(match carrier {
        _Carrier::Ref => witness == Clone::clone(expected),
        _Carrier::Long => _unbox_i64(&witness) == _unbox_i64(expected),
        _Carrier::Int => _unbox_i32(&witness) == _unbox_i32(expected),
    })
}

/// 交换语义（getAndSet / compareAndExchange 共用）：读旧值，可选比较
/// （Some：不符则不写并返回当前值——exchange 的见证形态；None：无条件换），
/// 写新值，返回旧值。
fn _field_exchange(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, expected: Option<&Object>, new: &Object) -> Result<Object> {
    // 读-比-写在字段存储单元内原子完成（引用槽写锁 / 原子单元），返回旧值（见证）。
    match carrier {
        _Carrier::Ref => {
            let mut nv = Some(Clone::clone(new));
            u.__vh_ref_update(holder, offset, &mut |cur| match expected {
                Some(e) if cur != Clone::clone(e) => None,
                _ => nv.take(),
            }).ok_or_else(_state_err)
        }
        _Carrier::Long => {
            let v = _unbox_i64(new).ok_or_else(|| _bad_arg("bad value form"))?;
            let e = match expected {
                Some(e) => Some(_unbox_i64(e).ok_or_else(|| _bad_arg("bad expected form"))?),
                None => None,
            };
            let old = u.__vh_long_update(holder, offset, |cur| match e {
                Some(e) if cur != e => cur,
                _ => v,
            }).ok_or_else(_state_err)?;
            Ok(Object::from(old))
        }
        _Carrier::Int => {
            let v = _unbox_i32(new).ok_or_else(|| _bad_arg("bad value form"))?;
            let e = match expected {
                Some(e) => Some(_unbox_i32(e).ok_or_else(|| _bad_arg("bad expected form"))?),
                None => None,
            };
            let old = u.__vh_int_update(holder, offset, |cur| match e {
                Some(e) if cur != e => cur,
                _ => v,
            }).ok_or_else(_state_err)?;
            Ok(Object::from(old))
        }
    }
}

// ── Array 家族（args = [数组, 下标, 值...]）──────────────────────────────────

// ── 字节数组视图族（MethodHandles.byteArrayViewVarHandle：VarHandleByteArrayAs*$ArrayHandle）──
// args = [byte[], 字节偏移, 值...]：在 byte[] 上按视图元素宽度、以 flavor 的 `be` 字段
// 给定的字节序读写一个 short/char/int/long/float/double（JDK 语义：越界检查
// `Preconditions.checkIndex(index, length - (width - 1))` → ArrayIndexOutOfBoundsException）。
// 消费方：sun.security.provider.ByteArrayAccess.LE/BE（MD5 / SHA 的字与字节块转换）。

#[derive(Clone, Copy)]
enum _ViewKind { Short, Char, Int, Long, Float, Double }

fn _byte_view(vh: &VarHandle) -> Option<(_ViewKind, usize, bool)> {
    let o = Object::from(Clone::clone(vh));
    let cn = o.0.__class_name();
    if !cn.contains("VarHandleByteArrayAs") || !cn.ends_with("$ArrayHandle") {
        return None;
    }
    let (kind, width) = if cn.contains("AsShorts") { (_ViewKind::Short, 2) }
        else if cn.contains("AsChars") { (_ViewKind::Char, 2) }
        else if cn.contains("AsInts") { (_ViewKind::Int, 4) }
        else if cn.contains("AsLongs") { (_ViewKind::Long, 8) }
        else if cn.contains("AsFloats") { (_ViewKind::Float, 4) }
        else if cn.contains("AsDoubles") { (_ViewKind::Double, 8) }
        else { return None };
    let be = o.0.__unsafe_bool_cell("be").map(|c| c.get()).unwrap_or(true);
    Some((kind, width, be))
}

/// 字节数组 + 偏移（越界 → AIOOBE，长度按 JDK 口径 `length - (width - 1)`）。
fn _view_target(args: &JArray<Object>, width: usize) -> Result<(JArray<i8>, usize)> {
    let ba = Clone::clone(&args.get(0)?).try_cast_array::<i8>("[B")?;
    let off = _arg_i32(args, 1, "bad byte array view index form")?;
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
        Ok(match kind {
            _ViewKind::Short => Object::from(bits as u16 as i16),
            _ViewKind::Char => Object::from(bits as u16),
            _ViewKind::Int => Object::from(bits as u32 as i32),
            _ViewKind::Long => Object::from(bits as i64),
            _ViewKind::Float => Object::from(f32::from_bits(bits as u32)),
            _ViewKind::Double => Object::from(f64::from_bits(bits)),
        })
    })())
}

fn _view_set(vh: &VarHandle, args: &JArray<Object>) -> Option<Result<()>> {
    let (kind, width, be) = _byte_view(vh)?;
    Some((|| {
        let (ba, off) = _view_target(args, width)?;
        let v = args.get(2)?;
        let bad = || _bad_arg("bad byte array view element form");
        let bits: u64 = match kind {
            _ViewKind::Short => crate::reflect_dispatch::unbox_i32(&v).ok_or_else(bad)? as u16 as u64,
            _ViewKind::Char => crate::reflect_dispatch::unbox_char(&v).ok_or_else(bad)? as u64,
            _ViewKind::Int => crate::reflect_dispatch::unbox_i32(&v).ok_or_else(bad)? as u32 as u64,
            _ViewKind::Long => crate::reflect_dispatch::unbox_i64(&v).ok_or_else(bad)? as u64,
            _ViewKind::Float => crate::reflect_dispatch::unbox_f32(&v).ok_or_else(bad)?.to_bits() as u64,
            _ViewKind::Double => crate::reflect_dispatch::unbox_f64(&v).ok_or_else(bad)?.to_bits(),
        };
        for k in 0..width {
            let shift = 8 * (width - 1 - k);
            let i = if be { k } else { width - 1 - k };
            ba.set((off + i) as i32, (bits >> shift) as u8 as i8)?;
        }
        Ok(())
    })())
}


fn _array_get(vh: &VarHandle, args: &JArray<Object>) -> Result<Object> {
    if let Some(r) = _view_get(vh, args) {
        return r;
    }
    let carrier = _carrier(vh);
    let idx = _arg_i32(args, 1, "bad array index form")?;
    match carrier {
        _Carrier::Ref => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<Object>("[Ljava/lang/Object;")?;
            arr.get(idx)
        }
        _Carrier::Long => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i64>("[J")?;
            Ok(Object::from(arr.get(idx)?))
        }
        _Carrier::Int => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            Ok(Object::from(arr.get(idx)?))
        }
    }
}

fn _array_set(vh: &VarHandle, args: &JArray<Object>) -> Result<()> {
    if let Some(r) = _view_set(vh, args) {
        return r;
    }
    let carrier = _carrier(vh);
    let idx = _arg_i32(args, 1, "bad array index form")?;
    match carrier {
        _Carrier::Ref => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<Object>("[Ljava/lang/Object;")?;
            arr.set(idx, args.get(2)?)
        }
        _Carrier::Long => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i64>("[J")?;
            let v = _arg_i64(args, 2, "bad array element form")?;
            arr.set(idx, v)
        }
        _Carrier::Int => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let v = _arg_i32(args, 2, "bad array element form")?;
            arr.set(idx, v)
        }
    }
}

fn _array_cas(vh: &VarHandle, args: &JArray<Object>) -> Result<bool> {
    let witness = _array_exchange(vh, args, true)?;
    Ok(match _carrier(vh) {
        _Carrier::Ref => witness == args.get(2)?,
        _Carrier::Long => _unbox_i64(&witness) == Some(_arg_i64(args, 2, "bad array element form")?),
        _Carrier::Int => _unbox_i32(&witness) == Some(_arg_i32(args, 2, "bad array element form")?),
    })
}

fn _array_exchange(vh: &VarHandle, args: &JArray<Object>, expected: bool) -> Result<Object> {
    // 元素的读-比-写在数组存储写锁内原子完成（JArray::__update），返回旧值（见证）。
    let carrier = _carrier(vh);
    let idx = _arg_i32(args, 1, "bad array index form")?;
    let vi = if expected { 3 } else { 2 };
    match carrier {
        _Carrier::Ref => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<Object>("[Ljava/lang/Object;")?;
            let e = if expected { Some(args.get(2)?) } else { None };
            let mut nv = Some(args.get(vi)?);
            arr.__update(idx, &mut |cur| match &e {
                Some(e) if cur != Clone::clone(e) => None,
                _ => nv.take(),
            })
        }
        _Carrier::Long => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i64>("[J")?;
            let e = if expected { Some(_arg_i64(args, 2, "bad array element form")?) } else { None };
            let v = _arg_i64(args, vi, "bad array element form")?;
            let old = arr.__update(idx, &mut |cur| match e {
                Some(e) if cur != e => None,
                _ => Some(v),
            })?;
            Ok(Object::from(old))
        }
        _Carrier::Int => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let e = if expected { Some(_arg_i32(args, 2, "bad array element form")?) } else { None };
            let v = _arg_i32(args, vi, "bad array element form")?;
            let old = arr.__update(idx, &mut |cur| match e {
                Some(e) if cur != e => None,
                _ => Some(v),
            })?;
            Ok(Object::from(old))
        }
    }
}

impl VarHandle {
    /// native `get(Object...)`：plain 读（单线程下与 volatile 同一存储单元）。
    pub fn get(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_get(self, &args);
        }
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_read(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, false)
    }

    /// native `set(Object...)`：plain 写。
    pub fn set(&self, args: JArray<Object>) -> Result<()> {
        if _is_array_flavor(self) {
            return _array_set(self, &args);
        }
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_write(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, &args.get(1)?, false)
    }

    /// native `getVolatile(Object...)`：volatile 读。
    pub fn getVolatile(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            return _array_get(self, &args);
        }
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_read(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, true)
    }

    /// native `setVolatile(Object...)`：volatile 写。
    pub fn setVolatile(&self, args: JArray<Object>) -> Result<()> {
        if _is_array_flavor(self) {
            return _array_set(self, &args);
        }
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_write(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, &args.get(1)?, true)
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
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_cas(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, &args.get(1)?, &args.get(2)?)
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
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_exchange(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, Some(&args.get(1)?), &args.get(2)?)
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
        let holder = args.get(0)?;
        let offset = _field_offset(self).ok_or_else(_state_err)?;
        _field_exchange(&Unsafe::getUnsafe()?, _carrier(self), &holder, offset, None, &args.get(1)?)
    }

    /// native `getAndSetAcquire(Object...)`：acquire 交换。
    pub fn getAndSetAcquire(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndSet(args)
    }

    /// native `getAndSetRelease(Object...)`：release 交换。
    pub fn getAndSetRelease(&self, args: JArray<Object>) -> Result<Object> {
        self.getAndSet(args)
    }
}

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
//! volatile / acquire / release / opaque 与 plain 的访问序差异在单 OS 线程
//! 协作调度下无跨线程可见性区别——同一存储单元（S-11 档位等价）；CAS 族
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

/// 读-比-写（单 OS 线程协作调度下不可分割）。引用比较按 Java `==`（null 与
/// 对象身份，`PartialEq for Object`）；数值族经 Unsafe 的 int/long CAS。
fn _field_cas(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, expected: &Object, new: &Object) -> Result<bool> {
    match carrier {
        _Carrier::Ref => {
            let cur = match u.__vh_ref_get(holder, offset) {
                Some(v) => v,
                None => return Err(_state_err()),
            };
            if cur == Clone::clone(expected) {
                _field_write(u, carrier, holder, offset, new, true)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _Carrier::Long => Ok(u.compareAndSetLong(
            Clone::clone(holder),
            offset,
            _unbox_i64(expected).ok_or_else(|| _bad_arg("bad expected form"))?,
            _unbox_i64(new).ok_or_else(|| _bad_arg("bad value form"))?,
        )?),
        _Carrier::Int => Ok(u.compareAndSetInt(
            Clone::clone(holder),
            offset,
            _unbox_i32(expected).ok_or_else(|| _bad_arg("bad expected form"))?,
            _unbox_i32(new).ok_or_else(|| _bad_arg("bad value form"))?,
        )?),
    }
}

/// 交换语义（getAndSet / compareAndExchange 共用）：读旧值，可选比较
/// （Some：不符则不写并返回当前值——exchange 的见证形态；None：无条件换），
/// 写新值，返回旧值。
fn _field_exchange(u: &Unsafe, carrier: _Carrier, holder: &Object, offset: i64, expected: Option<&Object>, new: &Object) -> Result<Object> {
    let old = _field_read(u, carrier, holder, offset, true)?;
    match expected {
        Some(e) if old != Clone::clone(e) => return Ok(old),
        _ => {}
    }
    _field_write(u, carrier, holder, offset, new, true)?;
    Ok(old)
}

// ── Array 家族（args = [数组, 下标, 值...]）──────────────────────────────────

fn _array_get(vh: &VarHandle, args: &JArray<Object>) -> Result<Object> {
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
    let carrier = _carrier(vh);
    let idx = _arg_i32(args, 1, "bad array index form")?;
    match carrier {
        _Carrier::Ref => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<Object>("[Ljava/lang/Object;")?;
            let cur = arr.get(idx)?;
            if cur == args.get(2)? {
                arr.set(idx, args.get(3)?)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _Carrier::Long => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i64>("[J")?;
            let expected = _arg_i64(args, 2, "bad array element form")?;
            let v = _arg_i64(args, 3, "bad array element form")?;
            if arr.get(idx)? == expected {
                arr.set(idx, v)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        _Carrier::Int => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let expected = _arg_i32(args, 2, "bad array element form")?;
            let v = _arg_i32(args, 3, "bad array element form")?;
            if arr.get(idx)? == expected {
                arr.set(idx, v)?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

fn _array_exchange(vh: &VarHandle, args: &JArray<Object>, expected: bool) -> Result<Object> {
    let carrier = _carrier(vh);
    let idx = _arg_i32(args, 1, "bad array index form")?;
    match carrier {
        _Carrier::Ref => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<Object>("[Ljava/lang/Object;")?;
            let old = arr.get(idx)?;
            let should_write = !expected || old == args.get(2)?;
            if should_write {
                arr.set(idx, args.get(if expected { 3 } else { 2 })?)?;
            }
            Ok(old)
        }
        _Carrier::Long => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i64>("[J")?;
            let old = arr.get(idx)?;
            let (expected_v, v) = if expected {
                (_arg_i64(args, 2, "bad array element form")?, _arg_i64(args, 3, "bad array element form")?)
            } else {
                (old, _arg_i64(args, 2, "bad array element form")?)
            };
            if old == expected_v {
                arr.set(idx, v)?;
            }
            Ok(Object::from(old))
        }
        _Carrier::Int => {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let old = arr.get(idx)?;
            let (expected_v, v) = if expected {
                (_arg_i32(args, 2, "bad array element form")?, _arg_i32(args, 3, "bad array element form")?)
            } else {
                (old, _arg_i32(args, 2, "bad array element form")?)
            };
            if old == expected_v {
                arr.set(idx, v)?;
            }
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

    /// native `compareAndSet(Object...)`：CAS（args = [holder, expected, new]）。
    /// 单 OS 线程协作调度下读-比-写不可分割。
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
    /// 单线程 Rc 运行时下 volatile 读 + volatile 写与原子交换观察面等价
    /// （无并发线程穿插；与 Unsafe.getAndAddInt 同族语义承载）。
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

//! `java/lang/invoke/VarHandle` 手写伴生：公开 API 类的 ACC_NATIVE 方法
//! （K-3a 共置形态）。`get` / `set` / `getAndSet` 等签名多态方法是 HotSpot
//! 内建（按 VarHandle 的具体 flavor 分派到内联访问器）；原生二进制无内建
//! ——分派按运行时类名（flavor）路由到既有擦除协议：
//!   - Field 家族（$FieldInstance*）：`fieldOffset`（实例字段的不透明 id，
//!     出自 MethodHandleNatives.objectFieldOffset 与 Unsafe 同一登记表）经
//!     ObjectVTable 的 `__unsafe_long_cell` 按字段名协议自读，访问经
//!     Unsafe 的 volatile int/long 访问器族（同一存储单元——写入对直接
//!     字段读取可见）；
//!   - Array 家族（*Array*）：args[0] 是数组本体、args[1] 是下标，经
//!     try_cast_array 还元素类型后直接读写。
//! 元素装箱形态：翻译层的 `i32/bool → Object` 装箱（Integer/Boolean 包装），
//! 回收按 toString 解析（数值/布尔字面量即十进制文本，解析无损）。

use crate::prelude::*;
use super::var_handle::VarHandle;
use crate::jdk::internal::misc::Unsafe;
use super::var_handle_booleans_field_instance_read_only::VarHandleBooleans_FieldInstanceReadOnly;
use super::var_handle_ints_field_instance_read_only::VarHandleInts_FieldInstanceReadOnly;
use super::var_handle_longs_field_instance_read_only::VarHandleLongs_FieldInstanceReadOnly;

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

/// args[i] → i32（形态不符 → IllegalArgumentException，与 JDK 传递异常同型）。
fn _arg_i32(args: &JArray<Object>, i: i32, what: &str) -> Result<i32> {
    match _unbox_i32(&args.get(i)?) {
        Some(v) => Ok(v),
        None => Err(JvmError::from(crate::java::lang::IllegalArgumentException::new_str(
            String::from(what))?)),
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
/// `fieldOffset` 擦除字段按运行时元素族还原具体静态视图后经字段访问器
/// 读取（`__unsafe_long_cell` 按名协议臂只对非擦除平铺 long 字段生成，
/// VarHandle flavor 的字段全走擦除载体，无臂可命中）。Array 家族 /
/// 引用元素族（无 int/long 访问器协议，宏域缺口如实暴露）→ None。
fn _field_offset(vh: &VarHandle) -> Option<i64> {
    let obj = Object::from(Clone::clone(vh));
    let cn = obj.0.__class_name();
    if cn.contains("VarHandleBooleans$") || cn.contains("VarHandleInts$") {
        if cn.contains("FieldInstance") {
            let v = Clone::clone(&obj)
                .try_cast::<VarHandleInts_FieldInstanceReadOnly>("java/lang/invoke/VarHandleInts$FieldInstanceReadOnly")
                .ok()?;
            return Some(v.__get_fieldOffset());
        }
    }
    if cn.contains("VarHandleLongs$") && cn.contains("FieldInstance") {
        let v = Clone::clone(&obj)
            .try_cast::<VarHandleLongs_FieldInstanceReadOnly>("java/lang/invoke/VarHandleLongs$FieldInstanceReadOnly")
            .ok()?;
        return Some(v.__get_fieldOffset());
    }
    let _ = std::marker::PhantomData::<VarHandleBooleans_FieldInstanceReadOnly>;
    None
}

/// flavor（运行时类名）是否 Array 家族。
fn _is_array_flavor(vh: &VarHandle) -> bool {
    Object::from(Clone::clone(vh)).0.__class_name().contains("$Array")
}

impl VarHandle {
    /// native `getVolatile(Object...)`：volatile 读。
    pub fn getVolatile(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let idx = _arg_i32(&args, 1, "bad array index form")?;
            return Ok(Object::from(arr.get(idx)?));
        }
        // Field 家族：int 载体（Booleans/Ints）经 Unsafe volatile 读
        let holder = args.get(0)?;
        let offset = _field_offset(self)
            .ok_or_else(_state_err)?;
        Ok(Object::from(Unsafe::getUnsafe()?.getIntVolatile(holder, offset)?))
    }

    /// native `setVolatile(Object...)`：volatile 写。
    pub fn setVolatile(&self, args: JArray<Object>) -> Result<()> {
        if _is_array_flavor(self) {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let idx = _arg_i32(&args, 1, "bad array index form")?;
            let v = _arg_i32(&args, 2, "bad array element form")?;
            arr.set(idx, v)?;
            return Ok(());
        }
        let holder = args.get(0)?;
        let v = _arg_i32(&args, 1, "bad value form")?;
        let offset = _field_offset(self)
            .ok_or_else(_state_err)?;
        Unsafe::getUnsafe()?.putIntVolatile(holder, offset, v)?;
        Ok(())
    }

    /// native `getAndSet(Object...)`：原子交换（旧值返回）。
    /// 单线程 Rc 运行时下 volatile 读 + volatile 写与原子交换观察面等价
    /// （无并发线程穿插；与 Unsafe.getAndAddInt 同族语义承载）。
    pub fn getAndSet(&self, args: JArray<Object>) -> Result<Object> {
        if _is_array_flavor(self) {
            let arr = Clone::clone(&args.get(0)?).try_cast_array::<i32>("[I")?;
            let idx = _arg_i32(&args, 1, "bad array index form")?;
            let v = _arg_i32(&args, 2, "bad array element form")?;
            let old = arr.get(idx)?;
            arr.set(idx, v)?;
            return Ok(Object::from(old));
        }
        let holder = args.get(0)?;
        let v = _arg_i32(&args, 1, "bad value form")?;
        let offset = _field_offset(self)
            .ok_or_else(_state_err)?;
        let u = Unsafe::getUnsafe()?;
        let old = u.getIntVolatile(Clone::clone(&holder), offset)?;
        u.putIntVolatile(holder, offset, v)?;
        Ok(Object::from(old))
    }
}

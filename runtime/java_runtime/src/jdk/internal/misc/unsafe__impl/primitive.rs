//! Unsafe 的基本类型读写与 volatile 访问（boolean / byte / short / char / float / double）（宿主 unsafe__impl.rs 的私有辅助模块）

use super::*;

// ── 基本类型读写（boolean / byte / short / char / float / double 的对象偏移形态）──────────
//
// 与 int / long 访问器同经统一载体 `unsafe__ext::prim`：基本类型数组 / 直接内存走字节视图，
// 静态字段走字段闭包 Update 臂，实例字段（如 ObjectStreamClass.FieldReflector 以 objectFieldOffset
// 读写的序列化字段）走 ObjectVTable 字 / 双字视图。

impl Unsafe {
    #[jvm_boundary]
    pub fn getBoolean(&self, o: Object, offset: i64) -> Result<bool> {
        Ok(_ext::get(&o, offset, 1, "getBoolean:(Ljava/lang/Object;J)Z")? != 0)
    }

    #[jvm_boundary]
    pub fn putBoolean(&self, o: Object, offset: i64, x: bool) -> Result<()> {
        _ext::put(&o, offset, 1, "putBoolean:(Ljava/lang/Object;JZ)V", x as u64)
    }

    #[jvm_boundary]
    pub fn getByte_obj_l(&self, o: Object, offset: i64) -> Result<i8> {
        Ok(_ext::get(&o, offset, 1, "getByte:(Ljava/lang/Object;J)B")? as u8 as i8)
    }

    #[jvm_boundary]
    pub fn putByte_obj_l_b(&self, o: Object, offset: i64, x: i8) -> Result<()> {
        _ext::put(&o, offset, 1, "putByte:(Ljava/lang/Object;JB)V", x as u8 as u64)
    }

    #[jvm_boundary]
    pub fn getShort_obj_l(&self, o: Object, offset: i64) -> Result<i16> {
        Ok(_ext::get(&o, offset, 2, "getShort:(Ljava/lang/Object;J)S")? as u16 as i16)
    }

    #[jvm_boundary]
    pub fn putShort_obj_l_s(&self, o: Object, offset: i64, x: i16) -> Result<()> {
        _ext::put(&o, offset, 2, "putShort:(Ljava/lang/Object;JS)V", x as u16 as u64)
    }

    #[jvm_boundary]
    pub fn getChar_obj_l(&self, o: Object, offset: i64) -> Result<u16> {
        Ok(_ext::get(&o, offset, 2, "getChar:(Ljava/lang/Object;J)C")? as u16)
    }

    #[jvm_boundary]
    pub fn putChar_obj_l_c(&self, o: Object, offset: i64, x: u16) -> Result<()> {
        _ext::put(&o, offset, 2, "putChar:(Ljava/lang/Object;JC)V", x as u64)
    }

    #[jvm_boundary]
    pub fn getFloat_obj_l(&self, o: Object, offset: i64) -> Result<f32> {
        Ok(f32::from_bits(_ext::get(&o, offset, 4, "getFloat:(Ljava/lang/Object;J)F")? as u32))
    }

    #[jvm_boundary]
    pub fn putFloat_obj_l_f(&self, o: Object, offset: i64, x: f32) -> Result<()> {
        _ext::put(&o, offset, 4, "putFloat:(Ljava/lang/Object;JF)V", x.to_bits() as u64)
    }

    #[jvm_boundary]
    pub fn getDouble_obj_l(&self, o: Object, offset: i64) -> Result<f64> {
        Ok(f64::from_bits(_ext::get(&o, offset, 8, "getDouble:(Ljava/lang/Object;J)D")?))
    }

    #[jvm_boundary]
    pub fn putDouble_obj_l_d(&self, o: Object, offset: i64, x: f64) -> Result<()> {
        _ext::put(&o, offset, 8, "putDouble:(Ljava/lang/Object;JD)V", x.to_bits())
    }
}

// ── 基本类型 volatile 访问（native get/put{Boolean,Byte,Short,Char,Float,Double}Volatile）──────
// HotSpot `MemoryAccess::get_volatile` / `put_volatile`（unsafe.cpp）：
//   读：[IRIW 平台先 fence] load；acquire
//   写：release；store；fence
// 存储单元与 plain 形态同一（字段闭包 / 原生内存），内存序按 HotSpot 原样以栅栏落地：
// 读前 SeqCst 栅栏（IRIW 保守取法）+ 读后 Acquire，写前 Release + 写后 SeqCst——
// 与 Java volatile 的顺序一致性同解（全部 volatile 访问之间存在单一全序）。

pub(super) fn _volatile_load<T>(load: impl FnOnce() -> Result<T>) -> Result<T> {
    use std::sync::atomic::{fence, Ordering};
    fence(Ordering::SeqCst);
    let v = load();
    fence(Ordering::Acquire);
    v
}

pub(super) fn _volatile_store(store: impl FnOnce() -> Result<()>) -> Result<()> {
    use std::sync::atomic::{fence, Ordering};
    fence(Ordering::Release);
    let r = store();
    fence(Ordering::SeqCst);
    r
}

impl Unsafe {
    #[jvm_native]
    pub fn getBooleanVolatile(&self, o: Object, offset: i64) -> Result<bool> {
        _volatile_load(|| self.getBoolean(o, offset))
    }

    #[jvm_native]
    pub fn putBooleanVolatile(&self, o: Object, offset: i64, x: bool) -> Result<()> {
        _volatile_store(|| self.putBoolean(o, offset, x))
    }

    #[jvm_native]
    pub fn getByteVolatile(&self, o: Object, offset: i64) -> Result<i8> {
        _volatile_load(|| self.getByte_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putByteVolatile(&self, o: Object, offset: i64, x: i8) -> Result<()> {
        _volatile_store(|| self.putByte_obj_l_b(o, offset, x))
    }

    #[jvm_native]
    pub fn getShortVolatile(&self, o: Object, offset: i64) -> Result<i16> {
        _volatile_load(|| self.getShort_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putShortVolatile(&self, o: Object, offset: i64, x: i16) -> Result<()> {
        _volatile_store(|| self.putShort_obj_l_s(o, offset, x))
    }

    #[jvm_native]
    pub fn getCharVolatile(&self, o: Object, offset: i64) -> Result<u16> {
        _volatile_load(|| self.getChar_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putCharVolatile(&self, o: Object, offset: i64, x: u16) -> Result<()> {
        _volatile_store(|| self.putChar_obj_l_c(o, offset, x))
    }

    #[jvm_native]
    pub fn getFloatVolatile(&self, o: Object, offset: i64) -> Result<f32> {
        _volatile_load(|| self.getFloat_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putFloatVolatile(&self, o: Object, offset: i64, x: f32) -> Result<()> {
        _volatile_store(|| self.putFloat_obj_l_f(o, offset, x))
    }

    #[jvm_native]
    pub fn getDoubleVolatile(&self, o: Object, offset: i64) -> Result<f64> {
        _volatile_load(|| self.getDouble_obj_l(o, offset))
    }

    #[jvm_native]
    pub fn putDoubleVolatile(&self, o: Object, offset: i64, x: f64) -> Result<()> {
        _volatile_store(|| self.putDouble_obj_l_d(o, offset, x))
    }

    /// native `throwException(Throwable ee)`：原样抛出 ee（HotSpot `Unsafe_ThrowException`：
    /// `THROW_OOP(JNIHandles::resolve(thr))`，不包装、不重填栈）；null → NullPointerException
    /// （与 athrow 对 null 的 JVMS 语义同一载体 `JvmError::from`）。
    #[jvm_native]
    pub fn throwException(&self, ee: crate::java::lang::Throwable) -> Result<()> {
        Err(crate::error::JvmError::from(ee))
    }
}

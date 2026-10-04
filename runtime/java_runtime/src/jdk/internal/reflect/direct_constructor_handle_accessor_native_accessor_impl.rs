//! `jdk.internal.reflect.DirectConstructorHandleAccessor$NativeAccessor` 的 native 方法。
//!
//! HotSpot `Reflection::invoke_constructor` 的对应物：按声明键经 L3 分派闭包 `<alloc>` + `<init>`。

use crate::prelude::*;
use super::direct_constructor_handle_accessor_native_accessor::DirectConstructorHandleAccessor_NativeAccessor;
use crate::java::lang::reflect::Constructor;
use crate::java::lang::reflect::InvocationTargetException;

/// 目标抛出的异常包装为 InvocationTargetException（JVM 同）；实参拆箱失败的
/// IllegalArgumentException 直抛。
fn __wrap_target_exception(e: crate::error::JvmError) -> crate::error::JvmError {
    if crate::reflect_dispatch::take_bad_arg() {
        return e;
    }
    match InvocationTargetException::new_throwable(
        <crate::java::lang::Throwable as From<Object>>::from(Clone::clone(e.thrown()))) {
        Ok(ite) => crate::error::JvmError::from(ite),
        Err(nested) => nested,
    }
}

impl DirectConstructorHandleAccessor_NativeAccessor {
    #[jvm_native]
    pub fn newInstance0(c: Constructor<Object>, args: JArray<Object>) -> Result<Object> {
        let Some((cls, desc)) = c.__reflect_key() else {
            panic!("stub: NativeAccessor.newInstance0 无声明键（非表构造的 Constructor）");
        };
        crate::reflect_dispatch::native_invoke(&cls, "<init>", &desc, Object::default(), &args)
            .map_err(__wrap_target_exception)
    }
}

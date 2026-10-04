//! `jdk.internal.reflect.NativeMethodAccessorImpl` 的 native 方法。
//!
//! 旧式反射方法访问器（`-Djdk.reflect.useDirectMethodHandle=false` / 早期启动路径）；
//! 与 DirectMethodHandleAccessor$NativeAccessor.invoke0 同为 HotSpot
//! `Reflection::invoke_method` 的对应物：按声明键走 L3 分派闭包。

use crate::prelude::*;
use super::native_method_accessor_impl::NativeMethodAccessorImpl;
use crate::java::lang::reflect::Method;
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

impl NativeMethodAccessorImpl {
    #[jvm_native]
    pub fn invoke0(m: Method, obj: Object, args: JArray<Object>) -> Result<Object> {
        let Some((cls, name, desc)) = m.__reflect_key() else {
            panic!("stub: NativeMethodAccessorImpl.invoke0 无声明键（非表构造的 Method）");
        };
        crate::reflect_dispatch::native_invoke(&cls, &name, &desc, obj, &args)
            .map_err(__wrap_target_exception)
    }
}

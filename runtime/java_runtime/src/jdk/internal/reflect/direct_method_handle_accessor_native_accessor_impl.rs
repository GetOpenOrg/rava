//! `jdk.internal.reflect.DirectMethodHandleAccessor$NativeAccessor` 的 native 方法。
//!
//! HotSpot `Reflection::invoke_method` 的对应物：MethodHandleAccessorFactory.useNativeAccessor
//! 为真（native 目标、java.lang.invoke 自身成员等）时经此调用；按声明键走 L3 分派闭包。

use crate::prelude::*;
use super::direct_method_handle_accessor_native_accessor::DirectMethodHandleAccessor_NativeAccessor;
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

impl DirectMethodHandleAccessor_NativeAccessor {
    #[jvm_native(upcalls = "java/lang/reflect/InvocationTargetException.<init>:(Ljava/lang/Throwable;)V java/lang/Integer.toString:()Ljava/lang/String; java/lang/Long.toString:()Ljava/lang/String; java/lang/Short.toString:()Ljava/lang/String; java/lang/Byte.toString:()Ljava/lang/String; java/lang/Character.toString:()Ljava/lang/String; java/lang/Boolean.toString:()Ljava/lang/String; java/lang/Float.toString:()Ljava/lang/String; java/lang/Double.toString:()Ljava/lang/String;")]
    pub fn invoke0(m: Method, obj: Object, args: JArray<Object>) -> Result<Object> {
        let Some((cls, name, desc)) = m.__reflect_key() else {
            panic!("stub: NativeAccessor.invoke0 无声明键（非表构造的 Method）");
        };
        crate::reflect_dispatch::reflect_invoke(&cls, &name, &desc, obj, &args)
            .map_err(__wrap_target_exception)
    }
}

//! `jdk.internal.reflect.NativeConstructorAccessorImpl` 的 native 方法。
//!
//! 旧式反射构造访问器（`-Djdk.reflect.useDirectMethodHandle=false` / 早期启动路径）；
//! 与 DirectConstructorHandleAccessor$NativeAccessor.newInstance0 同为 HotSpot
//! `Reflection::invoke_constructor` 的对应物：按声明键经 L3 分派闭包 `<alloc>` + `<init>`。

use crate::prelude::*;
use super::native_constructor_accessor_impl::NativeConstructorAccessorImpl;
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

impl NativeConstructorAccessorImpl {
    #[jvm_native(upcalls = "java/lang/reflect/InvocationTargetException.<init>:(Ljava/lang/Throwable;)V java/lang/Integer.toString:()Ljava/lang/String; java/lang/Long.toString:()Ljava/lang/String; java/lang/Short.toString:()Ljava/lang/String; java/lang/Byte.toString:()Ljava/lang/String; java/lang/Character.toString:()Ljava/lang/String; java/lang/Boolean.toString:()Ljava/lang/String; java/lang/Float.toString:()Ljava/lang/String; java/lang/Double.toString:()Ljava/lang/String;")]
    pub fn newInstance0(c: Constructor<Object>, args: JArray<Object>) -> Result<Object> {
        let Some((cls, desc)) = c.__reflect_key() else {
            panic!("stub: NativeConstructorAccessorImpl.newInstance0 无声明键（非表构造的 Constructor）");
        };
        crate::reflect_dispatch::reflect_invoke(&cls, "<init>", &desc, Object::default(), &args)
            .map_err(__wrap_target_exception)
    }
}

//! VM 支持类 `java/lang/reflect/Method$Direct`（直连反射调用，java_support/）的 native 方法。
//!
//! `invoke0` 与 `DirectMethodHandleAccessor$NativeAccessor.invoke0` 同语义（HotSpot `Reflection::invoke_method`
//! 的对应物）：按 Method 的声明键走 L3 分派闭包，目标抛出的异常包装为 InvocationTargetException。
//! 两处各自持有包装函数：两个类不一定同在闭包内。

use crate::prelude::*;
use super::Method_Direct;
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

impl Method_Direct {
    #[jvm_native]
    pub fn invoke0(m: Method, obj: Object, args: JArray<Object>) -> Result<Object> {
        let Some((cls, name, desc)) = m.__reflect_key() else {
            panic!("stub: Method$Direct.invoke0 无声明键（非表构造的 Method）");
        };
        crate::reflect_dispatch::native_invoke(&cls, &name, &desc, obj, &args)
            .map_err(__wrap_target_exception)
    }
}

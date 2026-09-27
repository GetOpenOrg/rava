//! 动态代理的 VM 侧公共面（FS-R R4a，方案 `docs/plans/2026-09-27-reflection-metadata-table.md` §2.6）。
//!
//! 代理实例由 VM 支持类 `java/lang/reflect/Proxy$Dyn`（字节码翻译）承载；接口载体分派回退经
//! `ObjectVTable::__proxy_invoke` 转入其 `dispatch`（手写层 `proxy_dyn_impl.rs`）。本模块只放
//! 与具体类无关、所有接口载体共用的返回值转换：JDK 生成的代理方法体对 `h.invoke` 的结果做
//! `checkcast` 包装类 + 拆箱（null → NullPointerException，错型 → ClassCastException）。

use crate::error::{JvmError, Result};
use crate::java::lang::Object;

/// 代理返回值 → 接口方法的声明返回类型（基本类型 / void；引用类型经 `From<Object>`）。
pub trait __ProxyRet: Sized {
    fn __from_proxy(v: Object) -> Result<Self>;
}

impl __ProxyRet for () {
    fn __from_proxy(_v: Object) -> Result<()> {
        Ok(())
    }
}

/// 生成体的 `checkcast <包装类>` + `xxxValue()`：null → NPE，非该包装类 → CCE。
fn checked_unbox<T>(v: &Object, wrapper: &str, f: impl Fn(&Object) -> Option<T>) -> Result<T> {
    if v.0.is_jvm_null() {
        return Err(JvmError::null_pointer());
    }
    let actual = v.0.__class_name();
    if actual != wrapper {
        return Err(JvmError::class_cast(format!(
            "class {} cannot be cast to class {}",
            actual.replace('/', "."), wrapper.replace('/', "."))));
    }
    f(v).ok_or_else(|| JvmError::class_cast(format!("cannot unbox {}", wrapper.replace('/', "."))))
}

macro_rules! proxy_ret_prim {
    ($t:ty, $wrapper:literal, $unbox:expr) => {
        impl __ProxyRet for $t {
            fn __from_proxy(v: Object) -> Result<$t> {
                checked_unbox(&v, $wrapper, $unbox)
            }
        }
    };
}

proxy_ret_prim!(i32, "java/lang/Integer", crate::reflect_dispatch::unbox_i32);
proxy_ret_prim!(i64, "java/lang/Long", crate::reflect_dispatch::unbox_i64);
proxy_ret_prim!(bool, "java/lang/Boolean", crate::reflect_dispatch::unbox_bool);
proxy_ret_prim!(f64, "java/lang/Double", crate::reflect_dispatch::unbox_f64);
proxy_ret_prim!(f32, "java/lang/Float", crate::reflect_dispatch::unbox_f32);
proxy_ret_prim!(u16, "java/lang/Character", crate::reflect_dispatch::unbox_char);
proxy_ret_prim!(i16, "java/lang/Short", |v: &Object| crate::reflect_dispatch::unbox_i32(v).map(|x| x as i16));
proxy_ret_prim!(i8, "java/lang/Byte", |v: &Object| crate::reflect_dispatch::unbox_i32(v).map(|x| x as i8));

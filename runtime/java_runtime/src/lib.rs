#![allow(unused_imports)]
pub mod array;
pub mod error;
pub mod java;
pub mod jdk;
pub mod sun;

pub use array::JArray;
pub use error::{JvmError, Result};
pub use java::lang::Object;

/// Java 风格浮点数格式化：整数值显示 .0，其他同 Rust 默认格式
pub fn java_fmt_f64(v: f64) -> String {
    if v.is_infinite() {
        if v > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
    } else if v.is_nan() {
        "NaN".to_string()
    } else if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}
pub fn java_fmt_f32(v: f32) -> String {
    if v.is_infinite() {
        if v > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
    } else if v.is_nan() {
        "NaN".to_string()
    } else if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

/// JVM null 检查：ifnull/ifnonnull 字节码翻译辅助。
/// Object::default()（内部 vtable = ()）表示 Java null；其他类型始终返回 false。
#[inline(always)]
pub fn _is_jnull<T: 'static>(val: &T) -> bool {
    if let Some(obj) = (val as &dyn std::any::Any).downcast_ref::<Object>() {
        obj.0.as_any().downcast_ref::<()>().is_some()
    } else {
        false
    }
}


/// MutexHolder：包装 parking_lot::ReentrantMutex，为 InternalLock 等需要 PartialEq 的结构体使用
#[derive(Clone)]
pub struct MutexHolder(pub std::sync::Arc<parking_lot::ReentrantMutex<()>>);
impl Default for MutexHolder {
    fn default() -> Self { Self(std::sync::Arc::new(parking_lot::ReentrantMutex::new(()))) }
}
impl PartialEq for MutexHolder {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::array::JArray;
    pub use super::error::{JvmError, Result};
    pub use super::java::lang::Object;
    pub use super::java::lang::ObjectVTable;
    pub use super::java::lang::String;
    pub use super::_is_jnull;

    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use super::MutexHolder;
    pub use java_rta_macros::{jvm_native, jvm_boundary, jvm_ext};
}

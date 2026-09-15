#![allow(unused_imports)]
pub mod error;
pub mod types;
pub mod java;
pub mod jdk;
pub mod sun;

pub use error::{JvmError, Result};
pub use types::JField;
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

/// Printable trait：统一 println 派发（T38）
/// 实现此 trait 的类型可直接传给 PrintStream::println
pub trait Printable {
    fn to_print_string(&self) -> String;
}
impl Printable for i32   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i64   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for f32   { fn to_print_string(&self) -> String { java_fmt_f32(*self) } }
impl Printable for f64   { fn to_print_string(&self) -> String { java_fmt_f64(*self) } }
impl Printable for bool  { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i8    { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i16   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for u16   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for java::lang::Object {
    fn to_print_string(&self) -> String { format!("{}", self) }
}

/// JVM null 检查：ifnull/ifnonnull 字节码翻译辅助。
/// Rust 类型不可为 null，此函数始终返回 false。
/// Option<T> 类型单独通过 Option::is_none() 处理。
#[inline(always)]
pub fn _is_jnull<T>(_val: &T) -> bool { false }

/// JVM Enum 基类方法：为所有类型提供默认 ordinal/name stub，
/// 避免 E0599 "no method named `ordinal`"。
/// 具体 enum 类的 inherent 方法会优先于此 trait 方法。
pub trait JvmEnum {
    fn ordinal(&self) -> Result<i32> {
        panic!("stub: Enum.ordinal() - enum field not initialized")
    }
}
impl<T> JvmEnum for T {}

/// JVM Object 基类方法：为所有类型提供默认 stub，
/// 避免 E0599 "no method named `getClass`/`hashCode`"。
/// 具体类的 inherent 方法会优先于此 trait 方法。
pub trait JvmObjectBase {
    fn getClass(&self) -> Result<java::lang::Object> {
        panic!("stub: Object.getClass()")
    }
    fn hashCode(&self) -> Result<i32> { Ok(0) }
    fn equals(&self, _other: java::lang::Object) -> Result<bool> { Ok(false) }
    fn jvm_clone(&self) -> Result<java::lang::Object> {
        panic!("stub: Object.clone()")
    }
}
impl<T> JvmObjectBase for T {}

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
    pub use super::error::{JvmError, Result};
    pub use super::types::JField;
    pub use super::java::lang::Object;
    pub use super::Printable;
    pub use super::_is_jnull;
    pub use super::JvmEnum;
    pub use super::JvmObjectBase;
    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use super::MutexHolder;
    pub use java_rta_macros::{jvm_native, jvm_boundary, jvm_ext};
}

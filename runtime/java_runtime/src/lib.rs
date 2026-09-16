#![allow(unused_imports)]
pub mod error;
pub mod java;
pub mod jdk;
pub mod sun;

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

/// Printable trait：统一 println 派发（T38）
/// 实现此 trait 的类型可直接传给 PrintStream::println
/// 注：J-3 废弃计划 — Arch-1 完成后由 ObjectVTable::toString 替代，届时删除本 trait
pub trait Printable {
    fn to_print_string(&self) -> std::string::String;
}
impl Printable for i32   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for i64   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for f32   { fn to_print_string(&self) -> std::string::String { java_fmt_f32(*self) } }
impl Printable for f64   { fn to_print_string(&self) -> std::string::String { java_fmt_f64(*self) } }
impl Printable for bool  { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for i8    { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for i16   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for u16   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for java::lang::Object {
    // 通过 ObjectVTable::toString() 动态派发到具体类型（Arch-4）
    fn to_print_string(&self) -> std::string::String { self.0.toString() }
}

/// JVM null 检查：ifnull/ifnonnull 字节码翻译辅助。
/// Rust 类型不可为 null，此函数始终返回 false。
/// Option<T> 类型单独通过 Option::is_none() 处理。
#[inline(always)]
pub fn _is_jnull<T>(_val: &T) -> bool { false }


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
    pub use super::java::lang::Object;
    pub use super::java::lang::ObjectVTable;
    pub use super::Printable;
    pub use super::_is_jnull;

    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use super::MutexHolder;
    pub use java_rta_macros::{jvm_native, jvm_boundary, jvm_ext};
}

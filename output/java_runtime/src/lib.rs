#![allow(unused_imports)]
pub mod error;
pub mod types;
pub mod java;

pub use error::{JvmError, Result};
pub use types::JField;
pub use java::lang::Object;

/// Printable trait：统一 println 派发（T38）
/// 实现此 trait 的类型可直接传给 PrintStream::println
pub trait Printable {
    fn to_print_string(&self) -> String;
}
impl Printable for i32   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i64   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for f32   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for f64   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for bool  { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i8    { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for i16   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for u16   { fn to_print_string(&self) -> String { format!("{}", self) } }
impl Printable for java::lang::Object {
    fn to_print_string(&self) -> String { format!("{}", self) }
}

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::error::{JvmError, Result};
    pub use super::types::JField;
    pub use super::java::lang::Object;
    pub use super::Printable;
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
}

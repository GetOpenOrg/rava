//! java.lang.Object — 所有 Java 类的根类型

#[doc(hidden)]
pub mod raw {
    use std::rc::Rc;

    /// 存储层：内部路径 crate::java::lang::object::raw::Object
    pub struct Object(pub Rc<dyn std::any::Any>);

    impl Clone for Object {
        fn clone(&self) -> Self {
            Object(self.0.clone())
        }
    }

    impl Default for Object {
        fn default() -> Self {
            Object(Rc::new(()))
        }
    }
}

/// 公开 API：re-export raw::Object 为 java.lang.Object
pub use raw::Object;

use crate::error::Result;
use crate::java::lang::String;

impl Object {
    /// monitorenter — synchronized 块进入（stub，单线程环境无需真正加锁）
    #[allow(non_snake_case)]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    /// monitorexit — synchronized 块退出（stub）
    #[allow(non_snake_case)]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    /// java.lang.Object.getClass()
    #[allow(non_snake_case)]
    pub fn getClass(&self) -> Result<String> { Ok(String::from("Object")) }

    /// java.lang.Object.hashCode()
    #[allow(non_snake_case)]
    pub fn hashCode(&self) -> Result<i32> { Ok(0) }

    /// java.lang.Object.equals(Object)
    #[allow(non_snake_case)]
    pub fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }

    /// java.lang.Object.toString()
    #[allow(non_snake_case)]
    pub fn toString(&self) -> Result<String> { Ok(String::from("Object")) }

    /// java.io.PrintStream 内部缓冲刷新（stub）
    #[allow(non_snake_case)]
    pub fn flushBuffer(&self) -> Result<()> { Ok(()) }
}

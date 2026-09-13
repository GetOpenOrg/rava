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

impl Object {
    /// 将任意 'static 值装入 Object（upcasting 工厂方法）
    pub fn from_any<T: std::any::Any + 'static>(v: T) -> Self {
        Object(std::rc::Rc::new(v))
    }

    /// 从 Object 中取出 T 的引用（downcasting），类型不符则 panic（ClassCastException）
    pub fn downcast<T: std::any::Any + Clone + 'static>(&self) -> T {
        self.0.downcast_ref::<T>()
            .expect("ClassCastException")
            .clone()
    }

    /// monitorenter — synchronized 块进入（stub，单线程环境无需真正加锁）
    #[allow(non_snake_case)]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    /// monitorexit — synchronized 块退出（stub）
    #[allow(non_snake_case)]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    /// java.lang.Object.getClass()
    #[allow(non_snake_case)]
    pub fn getClass(&self) -> Result<Object> { Ok(self.clone()) }

    /// java.lang.Object.hashCode()
    #[allow(non_snake_case)]
    pub fn hashCode(&self) -> Result<i32> { Ok(0) }

    /// java.lang.Object.equals(Object)
    #[allow(non_snake_case)]
    pub fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }

    /// java.lang.Object.toString()
    #[allow(non_snake_case)]
    pub fn toString(&self) -> Result<Object> { Ok(self.clone()) }

    /// java.io.PrintStream 内部缓冲刷新（stub）
    #[allow(non_snake_case)]
    pub fn flushBuffer(&self) -> Result<()> { Ok(()) }

    /// null 检查：在转译模型中 Object 永远非 null，始终返回 false
    pub fn is_none(&self) -> bool { false }

    /// Option::get 兼容接口：Object 始终存在，返回 self 的克隆
    pub fn get(&self) -> Result<Object> { Ok(self.clone()) }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object")
    }
}

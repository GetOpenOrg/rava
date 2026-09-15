//! java.lang.Object — 所有 Java 类的根类型
//! struct 定义永久手写（Rc<dyn Any> 类型擦除是 Rust-specific，无法从字节码生成）

#[doc(hidden)]
pub mod raw {
    use std::rc::Rc;

    /// Object 存储两个字段：
    ///   .0  : Rc<dyn Any>        — 类型擦除的值（downcast_ref::<T>() 仍可用）
    ///   .1  : fn(&str) -> bool   — instanceof 检查函数指针（非 Java 类型为 |_| false）
    pub struct Object(pub Rc<dyn std::any::Any>, pub fn(&str) -> bool);

    impl Clone for Object {
        fn clone(&self) -> Self { Object(self.0.clone(), self.1) }
    }

    impl Default for Object {
        fn default() -> Self { Object(Rc::new(()), |_| false) }
    }
}

pub use raw::Object;

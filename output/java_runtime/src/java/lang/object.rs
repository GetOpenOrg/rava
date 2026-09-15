//! java.lang.Object — 所有 Java 类的根类型
//! struct 定义永久手写（Rc<dyn Any> 类型擦除是 Rust-specific，无法从字节码生成）

#[doc(hidden)]
pub mod raw {
    use std::rc::Rc;

    pub struct Object(pub Rc<dyn std::any::Any>);

    impl Clone for Object {
        fn clone(&self) -> Self { Object(self.0.clone()) }
    }

    impl Default for Object {
        fn default() -> Self { Object(Rc::new(())) }
    }
}

pub use raw::Object;

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

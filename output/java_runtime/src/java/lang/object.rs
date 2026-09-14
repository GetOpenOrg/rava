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

    /// java.lang.Class.getComponentType() - 数组类型的元素类型（stub，非数组返回 null）
    #[allow(non_snake_case)]
    pub fn getComponentType(&self) -> Result<Object> {
        panic!("stub: Class.getComponentType()")
    }

    /// java.lang.Class.getName()
    #[allow(non_snake_case)]
    pub fn getName(&self) -> Result<Object> {
        panic!("stub: Class.getName()")
    }

    /// java.lang.Class.isArray()
    #[allow(non_snake_case)]
    pub fn isArray(&self) -> Result<bool> {
        panic!("stub: Class.isArray()")
    }

    /// null 检查：在转译模型中 Object 永远非 null，始终返回 false
    pub fn is_none(&self) -> bool { false }

    /// Option::get 兼容接口：Object 始终存在，返回 self 的克隆
    pub fn get(&self) -> Result<Object> { Ok(self.clone()) }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(r) = self.fmt_primitive(f) { return r; }
        write!(f, "Object")
    }
}

// Java autoboxing: 基本类型自动装箱为 Object
impl From<i32>  for Object { fn from(v: i32)  -> Self { Object::from_any(v) } }
impl From<i64>  for Object { fn from(v: i64)  -> Self { Object::from_any(v) } }
impl From<f32>  for Object { fn from(v: f32)  -> Self { Object::from_any(v) } }
impl From<f64>  for Object { fn from(v: f64)  -> Self { Object::from_any(v) } }
impl From<bool> for Object { fn from(v: bool) -> Self { Object::from_any(v) } }
impl From<i8>   for Object { fn from(v: i8)   -> Self { Object::from_any(v) } }
impl From<i16>  for Object { fn from(v: i16)  -> Self { Object::from_any(v) } }
impl From<u16>  for Object { fn from(v: u16)  -> Self { Object::from_any(v) } }

// Java unboxing: Object 反向解包为基本类型（T36）
// 用于 ArrayList<i32>::get(0) → i32 等场景
impl From<Object> for i32   { fn from(o: Object) -> i32   { o.downcast::<i32>()   } }
impl From<Object> for i64   { fn from(o: Object) -> i64   { o.downcast::<i64>()   } }
impl From<Object> for f32   { fn from(o: Object) -> f32   { o.downcast::<f32>()   } }
impl From<Object> for f64   { fn from(o: Object) -> f64   { o.downcast::<f64>()   } }
impl From<Object> for bool  { fn from(o: Object) -> bool  { o.downcast::<bool>()  } }
impl From<Object> for i8    { fn from(o: Object) -> i8    { o.downcast::<i8>()    } }
impl From<Object> for i16   { fn from(o: Object) -> i16   { o.downcast::<i16>()   } }
impl From<Object> for u16   { fn from(o: Object) -> u16   { o.downcast::<u16>()   } }

// Object equality: 比较原始类型值，其他类型回退到指针相等（Java Object.equals 语义）
// 注：String 等引用类型的值比较由 jdk_classes 的 native 实现负责（它能引用 String 类型）
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        macro_rules! try_eq {
            ($t:ty) => {
                if let (Some(a), Some(b)) = (self.0.downcast_ref::<$t>(), other.0.downcast_ref::<$t>()) {
                    return a == b;
                }
            };
        }
        try_eq!(i32);
        try_eq!(i64);
        try_eq!(bool);
        try_eq!(f32);
        try_eq!(f64);
        try_eq!(i8);
        try_eq!(i16);
        try_eq!(u16);
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Object {}

impl Object {
    /// 格式化原始类型的 Object，返回 None 表示需要调用方处理
    pub fn fmt_primitive(&self, f: &mut std::fmt::Formatter<'_>) -> Option<std::fmt::Result> {
        macro_rules! try_fmt {
            ($t:ty) => {
                if let Some(v) = self.0.downcast_ref::<$t>() {
                    return Some(write!(f, "{}", v));
                }
            };
        }
        try_fmt!(i32);
        try_fmt!(i64);
        try_fmt!(bool);
        try_fmt!(f32);
        try_fmt!(f64);
        try_fmt!(i8);
        try_fmt!(i16);
        try_fmt!(u16);
        None
    }
}

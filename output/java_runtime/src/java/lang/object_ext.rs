use crate::prelude::*;
use super::object::raw::Object;

impl Object {
    /// 将任意 'static 值装入 Object（JVM upcasting）
    #[jvm_ext]
    pub fn from_any<T: std::any::Any + 'static>(v: T) -> Self {
        Object(std::rc::Rc::new(v))
    }

    /// 从 Object 中取出 T（JVM checkcast/downcasting），类型不符则 panic（ClassCastException）
    #[jvm_ext]
    pub fn downcast<T: std::any::Any + Clone + 'static>(&self) -> T {
        self.0.downcast_ref::<T>()
            .expect("ClassCastException")
            .clone()
    }

    /// null 检查：转译模型中 Object 永远非 null，始终返回 false
    #[jvm_ext]
    pub fn is_none(&self) -> bool { false }

    /// Option::get 兼容接口：Object 始终存在，返回 self 的克隆
    #[jvm_ext]
    pub fn get(&self) -> Result<Object> { Ok(self.clone()) }

    /// 格式化原始类型的 Object，供 Display impl 使用
    #[jvm_ext]
    pub fn fmt_primitive(&self, f: &mut std::fmt::Formatter<'_>) -> Option<std::fmt::Result> {
        macro_rules! try_fmt {
            ($t:ty) => {
                if let Some(v) = self.0.downcast_ref::<$t>() {
                    return Some(write!(f, "{}", v));
                }
            };
        }
        try_fmt!(i32); try_fmt!(i64); try_fmt!(bool);
        try_fmt!(f32); try_fmt!(f64); try_fmt!(i8);
        try_fmt!(i16); try_fmt!(u16);
        None
    }
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

// Java unboxing: Object 反向解包为基本类型
impl From<Object> for i32   { fn from(o: Object) -> i32   { o.downcast::<i32>()   } }
impl From<Object> for i64   { fn from(o: Object) -> i64   { o.downcast::<i64>()   } }
impl From<Object> for f32   { fn from(o: Object) -> f32   { o.downcast::<f32>()   } }
impl From<Object> for f64   { fn from(o: Object) -> f64   { o.downcast::<f64>()   } }
impl From<Object> for bool  { fn from(o: Object) -> bool  { o.downcast::<bool>()  } }
impl From<Object> for i8    { fn from(o: Object) -> i8    { o.downcast::<i8>()    } }
impl From<Object> for i16   { fn from(o: Object) -> i16   { o.downcast::<i16>()   } }
impl From<Object> for u16   { fn from(o: Object) -> u16   { o.downcast::<u16>()   } }

// Object equality: 比较原始类型值，其他类型回退到指针相等
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        macro_rules! try_eq {
            ($t:ty) => {
                if let (Some(a), Some(b)) = (self.0.downcast_ref::<$t>(), other.0.downcast_ref::<$t>()) {
                    return a == b;
                }
            };
        }
        try_eq!(i32); try_eq!(i64); try_eq!(bool);
        try_eq!(f32); try_eq!(f64); try_eq!(i8);
        try_eq!(i16); try_eq!(u16);
        std::rc::Rc::ptr_eq(&self.0, &other.0)
    }
}
impl Eq for Object {}

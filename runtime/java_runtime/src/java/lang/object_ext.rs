use crate::prelude::*;
use super::object::{Object, ObjectVTable, JvmRef};

impl Object {
    /// 将任意 `'static` 值包装进 Object。
    ///
    /// 对于有 `ObjectVTable` impl 的类型（具体类、基本类型），通过 `JvmRef<T>` 包装；
    /// `Into<Object>` 生成代码中的具体类路径使用 `Object(Rc::new(self))` 直接存储，
    /// 不需要 `from_any`。
    /// `from_any` 主要用于：泛型参数 E、接口存根、工具方法中的类型擦除场景。
    #[jvm_ext]
    pub fn from_any<T: 'static>(v: T) -> Self {
        // 若 T 已经是 Object，直接 clone 避免双层包装（Object::from_any(Object) 幂等）
        let any_val: &dyn std::any::Any = &v;
        if let Some(obj) = any_val.downcast_ref::<Object>() {
            return obj.clone();
        }
        Object(std::rc::Rc::new(JvmRef(v)))
    }

    /// instanceof 运行时检查：委托给 ObjectVTable::is_instance_of（Arch-2）
    #[jvm_ext]
    pub fn is_instance_of(&self, type_id: &str) -> bool {
        self.0.is_instance_of(type_id)
    }

    /// 从 Object 中取出 T（JVM checkcast/downcasting），类型不符则 panic（ClassCastException）
    ///
    /// 双路径检查：
    ///   1. 直接路径：T 实现 ObjectVTable，通过 as_any() 直接取出
    ///   2. JvmRef 路径：T 通过 from_any 包装，as_any() 返回 &T
    ///      (JvmRef<T>::as_any → &self.0: &dyn Any，downcast_ref::<T>() 成功)
    #[jvm_ext]
    pub fn downcast<T: std::any::Any + Clone + Default + 'static>(&self) -> T {
        // null 通过任何引用类型的 checkcast（引用类型的 Default 即 null）；基本类型位置是拆箱
        if self.0.is_jvm_null() {
            let tid = std::any::TypeId::of::<T>();
            let is_primitive = [
                std::any::TypeId::of::<i8>(), std::any::TypeId::of::<i16>(), std::any::TypeId::of::<u16>(),
                std::any::TypeId::of::<i32>(), std::any::TypeId::of::<i64>(), std::any::TypeId::of::<f32>(),
                std::any::TypeId::of::<f64>(), std::any::TypeId::of::<bool>(),
            ].contains(&tid);
            if is_primitive {
                panic!("NullPointerException: 对 null 拆箱为 {}", std::any::type_name::<T>());
            }
            return T::default();
        }
        // Clone::clone 而非 .clone()：T 可能是带 Java clone() 方法的类
        // （Reference/HashMap 等），方法语法会被遮蔽返回 Result<Object>
        // T 本身就是 Object（泛型类以 Object 实例化，如 HashMap<Object, Object> 中的 V）：
        // 值无需拆箱，直接返回自身别名
        if let Some(same) = (self as &dyn std::any::Any).downcast_ref::<T>() {
            return Clone::clone(same);
        }
        if let Some(same) = self.0.as_any().downcast_ref::<T>() {
            return Clone::clone(same);
        }
        // 运行时类是 T 的子类（引用以祖先 / 子类的静态类型流转）：按运行时类重建 T 视图
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let mut slot: Option<T> = None;
        if self.0.__view_into(unused, &mut slot) {
            if let Some(view) = slot {
                return view;
            }
        }
        panic!("ClassCastException: {} cannot be cast to {}",
               self.0.__class_name(), std::any::type_name::<T>())
    }

    /// checkcast 的非 panic 形式：运行时类是 `T` 或其子类 → Some(视图)，否则 None。
    #[jvm_ext]
    pub fn try_checkcast<T: std::any::Any + Clone + 'static>(&self) -> Option<T> {
        if let Some(same) = (self as &dyn std::any::Any).downcast_ref::<T>() {
            return Some(Clone::clone(same));
        }
        if let Some(same) = self.0.as_any().downcast_ref::<T>() {
            return Some(Clone::clone(same));
        }
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let mut slot: Option<T> = None;
        self.0.__view_into(unused, &mut slot);
        slot
    }

    /// JVM checkcast：把引用还原为类 `T`（binary name 为 `binary_name`）的视图。
    /// 运行时类就是 `T` → 直接取出；运行时类是 `T` 的子类 → 按运行时类重建 `T` 视图
    /// （vtable upcast，保留运行时类的覆盖实现）；否则 ClassCastException。
    #[jvm_ext]
    pub fn checkcast<T: std::any::Any + Clone + 'static>(&self, binary_name: &str) -> T {
        if let Some(same) = (self as &dyn std::any::Any).downcast_ref::<T>() {
            return Clone::clone(same);
        }
        if let Some(same) = self.0.as_any().downcast_ref::<T>() {
            return Clone::clone(same);
        }
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        match self.0.__view_as(unused, binary_name).and_then(|boxed| boxed.downcast::<T>().ok()) {
            Some(view) => *view,
            None => panic!("ClassCastException: {} cannot be cast to {}", self.0.__class_name(), binary_name),
        }
    }

    /// java.lang.Comparable.compareTo — 委托到 vtable（String/Integer 等实现类会覆盖）
    #[jvm_ext]
    pub fn compareTo(&self, other: Object) -> crate::error::Result<i32> {
        self.0.compareTo(other)
    }

    /// null 检查：转译模型中 Object 永远非 null，始终返回 false
    #[jvm_ext]
    pub fn is_none(&self) -> bool { false }

    /// Option::get 兼容接口：Object 始终存在，返回 self 的克隆
    #[jvm_ext]
    pub fn get(&self) -> Result<Object> { Ok(self.clone()) }

    /// 格式化原始类型的 Object，供 Display impl 使用（fallback 路径）
    #[jvm_ext]
    pub fn fmt_primitive(&self, f: &mut std::fmt::Formatter<'_>) -> Option<std::fmt::Result> {
        macro_rules! try_fmt {
            ($t:ty) => {
                if let Some(v) = self.0.as_any().downcast_ref::<$t>() {
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
        write!(f, "{}", self.0.__obj_str())
    }
}

impl std::fmt::Debug for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object({})", self.0.__obj_str())
    }
}

// R-1: blanket impl — 所有 ObjectVTable 实现类型（含基本类型、生成类）均可转为 Object。
// 替代原先为每个基本类型和每个生成类手写/宏生成的 Into<Object>。
// Object = Rc<dyn ObjectVTable>；Rc<dyn ObjectVTable> 本身不实现 ObjectVTable，
// 故与 std 的 From<T> for T 无冲突。
impl<T: ObjectVTable + 'static> From<T> for Object {
    fn from(val: T) -> Self { Object(std::rc::Rc::new(val)) }
}

// Java unboxing: Object 反向解包为基本类型
impl From<Object> for i32   { fn from(o: Object) -> i32   { o.downcast::<i32>()   } }
impl From<Object> for i64   { fn from(o: Object) -> i64   { o.downcast::<i64>()   } }
impl From<Object> for f32   { fn from(o: Object) -> f32   { o.downcast::<f32>()   } }
impl From<Object> for f64   { fn from(o: Object) -> f64   { o.downcast::<f64>()   } }
impl From<Object> for bool  { fn from(o: Object) -> bool  { o.downcast::<bool>()  } }
impl From<Object> for i8    { fn from(o: Object) -> i8    { o.downcast::<i8>()    } }
impl From<Object> for i16   { fn from(o: Object) -> i16   { o.downcast::<i16>()   } }
impl From<Object> for u16   { fn from(o: Object) -> u16   { o.downcast::<u16>()   } }

// Object equality: null == null，基本类型值相等，其他类型 Rc 指针相等
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        macro_rules! try_eq {
            ($t:ty) => {
                if let (Some(a), Some(b)) = (
                    self.0.as_any().downcast_ref::<$t>(),
                    other.0.as_any().downcast_ref::<$t>()
                ) {
                    return a == b;
                }
            };
        }
        try_eq!(());  // null == null
        try_eq!(i32); try_eq!(i64); try_eq!(bool);
        try_eq!(f32); try_eq!(f64); try_eq!(i8);
        try_eq!(i16); try_eq!(u16);
        match (self.0.is_jvm_null(), other.0.is_jvm_null()) {
            (true, true) => true,
            (false, false) => self.0.__identity() == other.0.__identity(),
            _ => false,
        }
    }
}
impl Eq for Object {}

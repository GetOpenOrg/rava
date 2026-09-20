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

    /// checkcast 的可失败形态（A-3 / S-1）：判定失败返回 `Err(JvmError::class_cast)`
    /// ——沿 `?` 传播、可被 `java_try!` 捕获，替代 `Object::downcast` / `checkcast`
    /// 的进程 panic。判定按序：
    ///   1. null 通过任何引用类型的 checkcast（JVMS §6.5），得到目标类型的 null 视图；
    ///   2. 同 TypeId / 类族视图（`try_checkcast` 的 `__view_into`：同实例化取回、
    ///      子类按超类实参映射的视图、数组同元素类型 / 协变视图）；
    ///   3. 类目标：运行时类是目标类族（`is_instance_of` 按静态超类型名单匹配，
    ///      含子类）→ `<T as From<Object>>` 的擦除路径（`__erased_vtable` +
    ///      `__erased_inner` 部件重建，共享存储与对象标识，A-1）；
    ///   4. 数组目标（binary_name 以 `[` 开头）：`From<Object> for JArray<T>` 的
    ///      元素类型驱动判定（协变探针 / 擦除数组逐元素兼容，与 array.rs 同规则）。
    #[jvm_ext]
    pub fn try_cast<T>(&self, binary_name: &str) -> Result<T>
    where T: Clone + Default + Into<Object> + From<Object> + 'static {
        if self.0.is_jvm_null() {
            return Ok(<T as From<Object>>::from(self.clone()));
        }
        if let Some(same) = self.try_checkcast::<T>() {
            return Ok(same);
        }
        if binary_name.starts_with('[') {
            if crate::array::erased_array_compatible::<T>(self) {
                return Ok(<T as From<Object>>::from(self.clone()));
            }
        } else if self.0.is_instance_of(binary_name) {
            return Ok(<T as From<Object>>::from(self.clone()));
        }
        Err(crate::error::JvmError::class_cast(format!(
            "class {} cannot be cast to class {}",
            self.0.__class_name().replace('/', "."),
            binary_name.replace('/', "."),
        )))
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

    // ── S-2.2：Object 上的数组访问 API ─────────────────────────────────────
    // 多维数组 / 擦除数组以 Object 流转后，xaload / xastore / arraylength 需要
    // 按元素类型读写内部 JArray。不引入泛型参数：转译期与运行时都不知道擦除数组的
    // 元素类型，方法名即元素类型（codegen 由指令操作码选择，如 iaload → array_load_int）。
    // 与 JArray::get/set 一致：越界抛 ArrayIndexOutOfBoundsException（内部转发），
    // null 数组抛 NullPointerException（JVMS §6.5 arraylength / *aload / *astore）。

    /// 数组访问前的 null 检查：接收者是 Java null → NullPointerException
    fn array_npe_check(&self) -> Result<()> {
        if self.0.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        Ok(())
    }

    /// 运行时类不是数组（或元素类型与指令不符）→ ClassCastException。
    /// 仅防御性触发：JVM 校验器保证 xaload/xastore 的静态类型合法，转译代码不应到达。
    fn array_type_mismatch(&self, op: &str, expect: &str) -> crate::error::JvmError {
        crate::error::JvmError::class_cast(std::string::String::from(
            format!("{}: 运行时类 {} 不是{}", op, self.0.__class_name(), expect)))
    }

    /// arraylength：数组长度。任意元素类型（长度与元素类型无关，经 vtable 钩子）。
    #[jvm_ext]
    pub fn array_length(&self) -> Result<i32> {
        self.array_npe_check()?;
        match self.0.__array_len() {
            Some(len) => len,
            None => Err(self.array_type_mismatch("arraylength", "数组")),
        }
    }

    /// aaload：读引用元素。引用元素数组（含 `Object[]`、`String[]`、任意类数组、
    /// 多维数组、协变视图）经 `__view_into` 取 `JArray<Object>` 视图转发；
    /// 基本元素数组按元素类型分支装入 Object（防御路径）。
    #[jvm_ext]
    pub fn array_load_object(&self, idx: i32) -> Result<Object> {
        self.array_npe_check()?;
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let mut slot: Option<JArray<Object>> = None;
        self.0.__view_into(unused, &mut slot);
        if let Some(view) = slot {
            return view.get(idx);
        }
        macro_rules! try_prim_load {
            ($t:ty) => {
                if let Some(arr) = self.0.as_any().downcast_ref::<JArray<$t>>() {
                    return arr.get(idx).map(|v| Object::from(v));
                }
            };
        }
        try_prim_load!(i32); try_prim_load!(i64); try_prim_load!(f32); try_prim_load!(f64);
        try_prim_load!(i8);  try_prim_load!(i16); try_prim_load!(u16); try_prim_load!(bool);
        Err(self.array_type_mismatch("aaload", "引用元素数组"))
    }

    /// aastore：写引用元素。值经协变视图按源元素类型转发（存储检查在源数组侧）；
    /// 基本元素数组按元素类型还原后写入，类型不符抛 ClassCastException（防御路径）。
    #[jvm_ext]
    pub fn array_store_object(&self, idx: i32, val: Object) -> Result<()> {
        self.array_npe_check()?;
        let unused: std::rc::Rc<dyn std::any::Any> = std::rc::Rc::new(());
        let mut slot: Option<JArray<Object>> = None;
        self.0.__view_into(unused, &mut slot);
        if let Some(view) = slot {
            return view.set(idx, val);
        }
        macro_rules! try_prim_store {
            ($t:ty) => {
                if let Some(arr) = self.0.as_any().downcast_ref::<JArray<$t>>() {
                    if let Some(v) = val.0.as_any().downcast_ref::<$t>() {
                        return arr.set(idx, Clone::clone(v));
                    }
                    return Err(self.array_type_mismatch("aastore", "基本元素数组"));
                }
            };
        }
        try_prim_store!(i32); try_prim_store!(i64); try_prim_store!(f32); try_prim_store!(f64);
        try_prim_store!(i8);  try_prim_store!(i16); try_prim_store!(u16); try_prim_store!(bool);
        Err(self.array_type_mismatch("aastore", "引用元素数组"))
    }
}

/// 数组元素访问（基本类型，S-2.2）：元素类型与 JVM 栈形态一致（int/long/float/double）。
/// 宏定义须在模块作用域（impl 内不允许 macro_rules 定义），展开进下方 impl Object。
macro_rules! array_elem_exact {
    ($load:ident, $store:ident, $elem:ty, $stack:ty, $desc:literal) => {
        #[jvm_ext]
        pub fn $load(&self, idx: i32) -> Result<$stack> {
            self.array_npe_check()?;
            match self.0.as_any().downcast_ref::<JArray<$elem>>() {
                Some(arr) => arr.get(idx),
                None => Err(self.array_type_mismatch(concat!($desc, "aload"), concat!(" ", $desc, "[]"))),
            }
        }
        #[jvm_ext]
        pub fn $store(&self, idx: i32, val: $stack) -> Result<()> {
            self.array_npe_check()?;
            match self.0.as_any().downcast_ref::<JArray<$elem>>() {
                Some(arr) => arr.set(idx, val),
                None => Err(self.array_type_mismatch(concat!($desc, "astore"), concat!(" ", $desc, "[]"))),
            }
        }
    };
}

/// 数组元素访问（窄类型，S-2.2）：JVM 操作数栈上 byte/char/short 都是 int（JVMS §2.11.1），
/// 装载符号/零扩展到 i32，存储显式收窄。
macro_rules! array_elem_narrow {
    ($load:ident, $store:ident, $elem:ty, $desc:literal) => {
        #[jvm_ext]
        pub fn $load(&self, idx: i32) -> Result<i32> {
            self.array_npe_check()?;
            match self.0.as_any().downcast_ref::<JArray<$elem>>() {
                Some(arr) => arr.get(idx).map(|v| v as i32),
                None => Err(self.array_type_mismatch(concat!($desc, "aload"), concat!(" ", $desc, "[]"))),
            }
        }
        #[jvm_ext]
        pub fn $store(&self, idx: i32, val: i32) -> Result<()> {
            self.array_npe_check()?;
            match self.0.as_any().downcast_ref::<JArray<$elem>>() {
                Some(arr) => arr.set(idx, val as $elem),
                None => Err(self.array_type_mismatch(concat!($desc, "astore"), concat!(" ", $desc, "[]"))),
            }
        }
    };
}

impl Object {
    array_elem_exact!(array_load_int,    array_store_int,    i32, i32, "i");
    array_elem_exact!(array_load_long,   array_store_long,   i64, i64, "l");
    array_elem_exact!(array_load_float,  array_store_float,  f32, f32, "f");
    array_elem_exact!(array_load_double, array_store_double, f64, f64, "d");
    array_elem_narrow!(array_load_char,  array_store_char,  u16, "c");
    array_elem_narrow!(array_load_short, array_store_short, i16, "s");

    /// baload/bastore：JVM 对 boolean[] 与 byte[] 共用同一指令，运行时按实际元素类型分派
    #[jvm_ext]
    pub fn array_load_byte(&self, idx: i32) -> Result<i32> {
        self.array_npe_check()?;
        if let Some(arr) = self.0.as_any().downcast_ref::<JArray<bool>>() {
            return arr.get(idx).map(|v| v as i32);
        }
        match self.0.as_any().downcast_ref::<JArray<i8>>() {
            Some(arr) => arr.get(idx).map(|v| v as i32),
            None => Err(self.array_type_mismatch("baload", " byte[] 或 boolean[]")),
        }
    }
    #[jvm_ext]
    pub fn array_store_byte(&self, idx: i32, val: i32) -> Result<()> {
        self.array_npe_check()?;
        if let Some(arr) = self.0.as_any().downcast_ref::<JArray<bool>>() {
            return arr.set(idx, val != 0);
        }
        match self.0.as_any().downcast_ref::<JArray<i8>>() {
            Some(arr) => arr.set(idx, val as i8),
            None => Err(self.array_type_mismatch("bastore", " byte[] 或 boolean[]")),
        }
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

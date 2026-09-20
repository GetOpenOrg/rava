//! java.lang.Object — 所有 Java 类的根类型
//! struct 定义永久手写（Rc<dyn ObjectVTable> 是 Rust-specific，无法从字节码生成）

use std::rc::Rc;

/// JVM Object vtable：方法名与 java.lang.Object 字节码方法一一对应。
///
/// `java_class` 宏为每个具体非接口类自动生成 impl：
///   - `is_instance_of`：静态展开 `all_supertypes` 列表（Arch-2）
///   - `as_any`：返回 `self as &dyn Any`，供 downcast 使用
///   - `toString`：返回 Rust 字符串用于 Display（Arch-4）
///
/// 接口类型（`is_interface = true`）不生成 ObjectVTable impl，
/// 其运行时实例通过 `JvmRef` 包装存储在 Object 中。
pub trait ObjectVTable: 'static {
    /// java.lang.Object.hashCode()I 默认实现
    fn hashCode(&self) -> i32 { 0 }

    /// java.lang.Object.equals(Object)Z 的覆盖入口：引用相等已由调用方（`Object::equals`）判定，
    /// 此处只承载运行时类的覆盖实现；未覆盖的类 → false。
    fn equals(&self, _other: Object) -> crate::error::Result<bool> { Ok(false) }

    /// 用于 Display/Debug 的 Rust 字符串（内部用途，避免与 Java toString() -> Result<String> 冲突）
    fn __obj_str(&self) -> std::string::String {
        std::any::type_name::<Self>().to_owned()
    }

    /// instanceof 运行时检查（java_class 宏从 all_supertypes 静态展开 matches! 模式）
    fn is_instance_of(&self, _type_id: &str) -> bool { false }

    /// 向下转型辅助：返回 self 作为 &dyn Any（供 Object::downcast 使用）
    fn as_any(&self) -> &dyn std::any::Any;

    /// java.lang.Object.getClass()Ljava/lang/Class; — 返回类型与字节码签名一致。
    /// 以 `__class_name()` 经 `Class::for_class` 取类对象（线程内按名缓存，身份语义）。
    /// null 检查语义：JVM 里对 null 引用调 getClass 抛 NPE，由调用侧的 null 守卫承载；
    /// 此处到达即 receiver 非 null。
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> {
        Ok(crate::java::lang::Class::for_class(
            crate::java::lang::String::from(self.__class_name()),
        ))
    }

    /// java.lang.Comparable.compareTo(Object)I — 接口方法，不实现 Comparable 的类调用时 panic
    fn compareTo(&self, _other: Object) -> crate::error::Result<i32> {
        panic!("stub: java/lang/Comparable.compareTo:(Ljava/lang/Object;)I")
    }

    /// JVM null 检查辅助：Default::default() 代表 null，构造后设为 false。
    /// java_class! 宏对生成类自动 override；基本类型 / 手写类默认 false（永不为 null）。
    fn is_jvm_null(&self) -> bool { false }

    /// 接口视图查询（invokeinterface 的运行时入口）：`slot` 是调用方提供的
    /// `Option<Rc<dyn I__VTable>>`（I 为被调用的 Java 接口）；对象的运行时类实现 I 时，
    /// 把自身以该接口的擦除 vtable 形态填入 `slot`。
    ///
    /// 按「擦除后的接口」选择——`slot` 的类型不含任何类型实参，与 JVM 的 itable 查找一致。
    /// `java_class!` 宏为每个类按其 `impl Iface for Class` 块生成实现；
    /// 默认（未实现任何接口的对象）不填 `slot`。
    #[doc(hidden)]
    fn __interface(self: Rc<Self>, _slot: &mut dyn std::any::Any) {}
    /// 运行时类的 binary name（如 `java/lang/NullPointerException`）。
    /// java_class! 宏对生成类自动 override；未捕获异常报告等 VM 级设施据此取得类名。
    fn __class_name(&self) -> &'static str { "java/lang/Object" }

    /// 按运行时类重建 `type_id`（本类或任一祖先类的 binary name）类型的引用视图。
    /// 对象常以静态类型（如 `Throwable`）流转，catch 需要按运行时类还原为 catch 声明类型。
    /// `any` 是对象存储的 `Rc<dyn Any>`；wrapper 侧 override 传入自身存储并委托 vtable。
    fn __view_as(
        &self,
        _any: Rc<dyn std::any::Any>,
        _type_id: &str,
    ) -> Option<Box<dyn std::any::Any>> { None }

    /// 对象标识（`==` / `!=` 引用比较的依据）：同一 Java 对象的所有引用视图（祖先类 wrapper、
    /// 接口载体、Object）返回同一值。java_class! 宏对生成类 override 为对象存储的标识单元。
    #[doc(hidden)]
    fn __identity(&self) -> *const () { self as *const Self as *const () }

    /// 以 Object 流转的数组（`Object::array_length`，S-2.2）：本运行时类是数组 → 长度；
    /// 非数组 → None。JArray 与数组载体（Rc<RefCell<Vec<T>>>）override；元素类型对长度
    /// 无关紧要，故不按元素类型分支，避免枚举所有实例化。
    #[doc(hidden)]
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { None }

    /// 擦除存储导出（A-1 存储层擦除）：`slot` 是 `Option<Rc<dyn Any>>`，类 wrapper 填入
    /// 自身持有的非泛型 `Rc<X__inner>`（其 TypeId 与类型实参无关）。`From<Object> for X<A>`
    /// 的擦除路径据此对任意类型实参重建视图（Java 泛型运行时本就擦除）。
    /// 其余对象（基本类型、闭包等）不填 `slot`。
    #[doc(hidden)]
    fn __erased_inner(self: Rc<Self>, _slot: &mut dyn std::any::Any) {}

    /// checkcast 的类型驱动形式：`slot` 是 `Option<T>`，`T` 为本类或任一祖先类的 wrapper 类型时
    /// 按运行时类重建该视图写入 `slot` 并返回 true（保留运行时类的覆盖实现）；否则返回 false。
    fn __view_into(
        &self,
        _any: Rc<dyn std::any::Any>,
        _slot: &mut dyn std::any::Any,
    ) -> bool { false }

    /// `Object.clone()` 的 native 语义：新建同运行时类的对象，逐字段拷贝（浅拷贝）。
    /// java_class! 宏对生成类自动 override；无字段存储的值（装箱基本类型等）返回 None。
    fn __shallow_copy(&self) -> Option<Object> { None }
}

/// `super.clone()`（invokespecial java/lang/Object.clone）的落点。
/// Object.clone 是 ACC_NATIVE：运行时类未实现 Cloneable 时抛 CloneNotSupportedException，
/// 否则返回逐字段浅拷贝。
#[allow(non_snake_case)]
pub fn Object__clone_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<Object> {
    if !this.is_instance_of("java/lang/Cloneable") {
        return Err(crate::error::JvmError::clone_not_supported(this.__class_name()));
    }
    match this.__shallow_copy() {
        Some(copy) => Ok(copy),
        None => Err(crate::error::JvmError::clone_not_supported(this.__class_name())),
    }
}

/// `super.hashCode()`（invokespecial java/lang/Object.hashCode）的落点。
/// Object.hashCode 是 ACC_NATIVE：身份哈希，取实例体的堆地址（与 `new Object()` 实例一致）。
#[allow(non_snake_case)]
pub fn Object__hashCode_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<i32> {
    Ok(this as *const T as *const () as usize as i32)
}

/// `super.equals(o)`（invokespecial java/lang/Object.equals）的落点：引用相等（`this == o`）。
/// `this` 是实例体引用，`other` 的 Rc 数据指针指向同一实例体时为同一对象。
#[allow(non_snake_case)]
pub fn Object__equals_base<T: ObjectVTable + ?Sized>(this: &T, other: Object) -> crate::error::Result<bool> {
    Ok(std::ptr::eq(this as *const T as *const (), Rc::as_ptr(&other.0) as *const ()))
}

/// `super.toString()`（invokespecial java/lang/Object.toString）的落点：
/// `getClass().getName() + "@" + Integer.toHexString(hashCode())`，hashCode 走虚派发。
#[allow(non_snake_case)]
pub fn Object__toString_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<crate::java::lang::String> {
    let text = format!("{}@{:x}", this.__class_name().replace('/', "."), this.hashCode());
    Ok(crate::java::lang::String::from(text))
}

// ── 基本类型 ObjectVTable impl（int 装箱进 Object 的场景）─────────────────────
//
// S-3.1 后装箱类型（Integer/Long/...）是字节码翻译出的真实类，`Integer.valueOf`
// 等工厂由翻译体承载（含缓存池语义）。原生值只在「未经 javac 装箱就流入 Object
// 位置」的角落出现（类型变量擦除边界、手写层的 Object::from_any(i32) 等）：
// 此时基本类型盒自身承载对应包装类的 binary name —— getClass()、instanceof、
// 异常消息对这些值给出与 JVM 一致的答案。这与翻译类路径（wrapper 的 vtable）
// 互不冲突：Rc<i32> 与 Rc<Integer> 的 TypeId 不同，downcast 各自精确命中。
macro_rules! impl_vtable_primitive {
    ($t:ty, $bin:literal) => {
        impl ObjectVTable for $t {
            fn __obj_str(&self) -> std::string::String { format!("{}", self) }
            fn __class_name(&self) -> &'static str { $bin }
            fn is_instance_of(&self, type_id: &str) -> bool { type_id == $bin }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
    ($t:ty, $bin:literal, $fmt:ident) => {
        impl ObjectVTable for $t {
            fn __obj_str(&self) -> std::string::String { crate::$fmt(*self) }
            fn __class_name(&self) -> &'static str { $bin }
            fn is_instance_of(&self, type_id: &str) -> bool { type_id == $bin }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
}
impl_vtable_primitive!(i32, "java/lang/Integer");
impl_vtable_primitive!(i64, "java/lang/Long");
impl_vtable_primitive!(bool, "java/lang/Boolean");
impl_vtable_primitive!(i8,  "java/lang/Byte");
impl_vtable_primitive!(i16, "java/lang/Short");
impl_vtable_primitive!(u16, "java/lang/Character");
impl_vtable_primitive!(f32, "java/lang/Float", java_fmt_f32);
impl_vtable_primitive!(f64, "java/lang/Double", java_fmt_f64);

/// null/default 值：存储 () 表示 Java null
impl ObjectVTable for () {
    fn __obj_str(&self) -> std::string::String { "null".to_owned() }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn is_jvm_null(&self) -> bool { true }
}

/// 数组类型（Rc<RefCell<Vec<T>>>）自动装入 Object
impl<T: 'static> ObjectVTable for Rc<std::cell::RefCell<Vec<T>>> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> {
        Some(Ok(self.borrow().len() as i32))
    }
}

/// `JvmRef<T>` — 将没有 `ObjectVTable` impl 的任意值（泛型参数、接口类型存根等）包装进 Object。
///
/// 使用场景：
///   - `Object::from_any(e)` where `e: E`（泛型参数，不确定是否实现 ObjectVTable）
///   - 接口 PhantomData 存根临时转为 Object（Arch-1 前的过渡状态）
///
/// `downcast::<T>()` 会同时检查直接路径（T implements ObjectVTable）和 JvmRef 包装路径。
pub struct JvmRef<T: 'static>(pub T);
impl<T: 'static> ObjectVTable for JvmRef<T> {
    fn as_any(&self) -> &dyn std::any::Any { &self.0 }
    fn __obj_str(&self) -> std::string::String {
        let v: &dyn std::any::Any = &self.0;
        macro_rules! try_fmt {
            ($t:ty) => { if let Some(x) = v.downcast_ref::<$t>() { return format!("{}", x); } };
        }
        try_fmt!(i32); try_fmt!(i64); try_fmt!(bool);
        try_fmt!(f32); try_fmt!(f64); try_fmt!(i8); try_fmt!(i16); try_fmt!(u16);
        std::any::type_name::<T>().to_owned()
    }
}

/// `Object` — 所有 Java 类的运行时表示。
///
/// 内部结构：`Rc<dyn ObjectVTable>`
///   - 具体类通过 `java_class` 宏的 `impl ObjectVTable` 直接存储
///   - 基本类型通过 primitive ObjectVTable impl 直接存储
///   - 泛型参数/接口类型通过 `JvmRef<T>` 包装存储
///   - 通过 `as_any()` + `downcast_ref` 实现类型还原
#[derive(Clone)]
pub struct Object(pub Rc<dyn ObjectVTable>);

/// `(void) obj` —— 丢弃引用；使 `()` 满足类型实参的 `From<Object>` 约束。
impl From<Object> for () {
    fn from(_: Object) {}
}

/// Java null 的唯一实例（S-3.2）：`Object::default()` 每次新建 `Rc::new(())` 时，
/// 两个 null 的 `__identity()` 不同，凡按身份比较的路径（`Object__equals_base` 的指针
/// 相等、协变视图的 `identity` 等）会把 null 误判为互不相等。null 用 thread_local
/// singleton 后所有 null 共享同一 `Rc` 指针，身份比较与 `PartialEq` 的 null 短路
///（object_ext.rs，先于本 singleton 存在的第二道防线）语义一致。
impl Default for Object {
    fn default() -> Self {
        thread_local! {
            static JVM_NULL: Object = Object(Rc::new(()));
        }
        JVM_NULL.with(|null| null.clone())
    }
}


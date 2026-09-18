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

    /// 用于 Display/Debug 的 Rust 字符串（内部用途，避免与 Java toString() -> Result<String> 冲突）
    fn __obj_str(&self) -> std::string::String {
        std::any::type_name::<Self>().to_owned()
    }

    /// instanceof 运行时检查（java_class 宏从 all_supertypes 静态展开 matches! 模式）
    fn is_instance_of(&self, _type_id: &str) -> bool { false }

    /// 向下转型辅助：返回 self 作为 &dyn Any（供 Object::downcast 使用）
    fn as_any(&self) -> &dyn std::any::Any;

    /// java.lang.Object.getClass()Ljava/lang/Class; — 返回类型与字节码签名一致。
    /// 简化实现：返回 null Class（Default），运行时类对象模型落地后在此处替换。
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> {
        Ok(Default::default())
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
}

// ── 基本类型 ObjectVTable impl（供自动装箱路径使用）────────────────────────────
macro_rules! impl_vtable_primitive {
    ($t:ty) => {
        impl ObjectVTable for $t {
            fn __obj_str(&self) -> std::string::String { format!("{}", self) }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
    ($t:ty, $fmt:ident) => {
        impl ObjectVTable for $t {
            fn __obj_str(&self) -> std::string::String { crate::$fmt(*self) }
            fn as_any(&self) -> &dyn std::any::Any { self }
        }
    };
}
impl_vtable_primitive!(i32);
impl_vtable_primitive!(i64);
impl_vtable_primitive!(bool);
impl_vtable_primitive!(i8);
impl_vtable_primitive!(i16);
impl_vtable_primitive!(u16);
impl_vtable_primitive!(f32, java_fmt_f32);
impl_vtable_primitive!(f64, java_fmt_f64);

/// null/default 值：存储 () 表示 Java null
impl ObjectVTable for () {
    fn __obj_str(&self) -> std::string::String { "null".to_owned() }
    fn as_any(&self) -> &dyn std::any::Any { self }
}

/// 数组类型（Rc<RefCell<Vec<T>>>）自动装入 Object
impl<T: 'static> ObjectVTable for Rc<std::cell::RefCell<Vec<T>>> {
    fn as_any(&self) -> &dyn std::any::Any { self }
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

impl Default for Object {
    fn default() -> Self { Object(Rc::new(())) }
}

/// `super.clone()` 的落点：`java/lang/Object.clone` 是 ACC_NATIVE 方法，invokespecial 的
/// `Object__clone_base(this)` 路由到这里（与宏为生成类产出的 `ClassName__method_base` 同形）。
/// 浅拷贝需要对象模型提供按运行时类型复制字段的入口，当前调用链尚未实际命中，保持精确存根。
#[allow(non_snake_case)]
pub fn Object__clone_base<T: ?Sized>(_this: &T) -> crate::error::Result<Object> {
    panic!("stub: java/lang/Object.clone:()Ljava/lang/Object;")
}

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

    // ── Object 监视器方法（S-20，JLS §17.2）────────────────────────────────
    //
    // 按对象身份（__identity）挂接 monitor.rs 的监视器侧表。以 trait 默认方法
    // 提供给全部实现类型（生成类 wrapper、基本类型盒、数组、JvmRef），具体类
    // 不感知；`Object` 包装器的固有方法（object_impl.rs）承载 bare-Object 接收者
    // 的调用（invokevirtual java/lang/Object.*）。重载命名与生成侧同源：
    // wait()V → wait、wait(J)V → wait_l、wait(JI)V → wait_l_i（描述符后缀 _PRIM_SUFFIX，J→l）。

    /// java.lang.Object.wait()V（等价 wait(0)）
    fn wait(&self) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.__identity() as usize, false, 0, 0)
    }

    /// java.lang.Object.wait(J)V：millis 为 0 表示无限等待，负值抛 IllegalArgumentException。
    fn wait_l(&self, millis: i64) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.__identity() as usize, false, millis, 0)
    }

    /// java.lang.Object.wait(JI)V：nanos 须在 0..=999999。
    fn wait_l_i(&self, millis: i64, nanos: i32) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::wait_timeout(self.__identity() as usize, false, millis, nanos)
    }

    /// java.lang.Object.notify()V：唤醒一个在该对象监视器上等待的线程，无等待者时静默。
    fn notify(&self) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::notify(self.__identity() as usize, false)
    }

    /// java.lang.Object.notifyAll()V
    fn notify_all(&self) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::notify_all(self.__identity() as usize, false)
    }

    /// monitorenter（指令侧）：进入本对象的监视器（可重入）。
    fn monitor_enter(&self) -> crate::error::Result<()> {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        crate::monitor::enter(self.__identity() as usize, false)
    }

    /// monitorexit（指令侧）：退出本对象的监视器一层。
    fn monitor_exit(&self) -> crate::error::Result<()> {
        crate::monitor::exit(self.__identity() as usize)
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

    /// 擦除 vtable 导出（A-1 部件形态，与 `__erased_inner` 配对）：`slot` 是调用方
    /// （`From<Object> for X<A>`，知道目标类 X）构造的 `Option<Rc<dyn X__VTable>>`。
    /// 对象的运行时类是 X 或 X 的子类时，把自身 vtable 以 X 的擦除 vtable 形态填入
    /// （类 vtable trait 非泛型、超类链是其 supertrait —— 子类 vtable 直接上转）。
    /// 与 `__erased_inner` 导出的存储合成 `X<A>` 的任意实例化视图——「运行时类是本类
    /// 子类 + 目标实例化非精确实参」的重建（如 `Enum::<Object>::from(枚举常量)`）。
    /// 其余对象（基本类型、闭包、接口载体等）不填 `slot`。
    #[doc(hidden)]
    fn __erased_vtable(self: Rc<Self>, _slot: &mut dyn std::any::Any) {}

    /// 数组协变的元素赋值兼容探针（S-4）：receiver 是引用元素数组（JArray），`slot` 是
    /// 调用方（`From<Object> for JArray<T>`，知道目标元素类型 T）构造的 `Option<T>`。
    /// 本钩子以「源元素类型的探针对象」view_into 该 slot——祖先名单静态生成（与元素值
    /// 无关），填充成功 ⇔ T 是源元素类型自身或其祖先（JLS §4.10.3 数组子类型条件）。
    /// 非数组对象不响应（默认 false，checkcast 由其余钩子判定）。
    #[doc(hidden)]
    fn __array_elem_assignable(&self, _slot: &mut dyn std::any::Any) -> bool { false }

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

    /// Unsafe 实例字段 long 原子协议（`Unsafe.getLongVolatile`/`putLongVolatile`/
    /// `compareAndSetLong`/`getAndAddLong` 的实例字段形态）：按字段名取共享的
    /// long 字存储单元。java_class! 宏为每个含非擦除 `long` 字段的生成类按
    /// 平铺字段名单生成臂（含继承字段）；其余（无该字段 / 数组 / 基本类型盒）
    /// 返回 None。返回的 `Rc<Cell<i64>>` 与该对象全部 wrapper 视图共享——
    /// Unsafe 经 Object 写入对直接字段读取（`__get_xxx`）可见，与 JVM 的字段
    /// 内存语义一致（Unsafe 与普通字段访问指向同一存储）。
    #[doc(hidden)]
    fn __unsafe_long_cell(&self, _field: &str) -> Option<Rc<std::cell::Cell<i64>>> { None }

    /// Unsafe 实例字段 int 原子协议（`Unsafe.getInt`/`putInt`/`compareAndSetInt`/
    /// `getAndAddInt` 的实例字段形态）：`__unsafe_long_cell` 的 int 镜像，
    /// 按字段名取共享的 int 存储单元（`Rc<Cell<i32>>`）。
    #[doc(hidden)]
    fn __unsafe_int_cell(&self, _field: &str) -> Option<Rc<std::cell::Cell<i32>>> { None }

    /// Unsafe/VarHandle 实例字段**引用**原子协议（引用族的
    /// `get/set/compareAndSet/getAndSet` 等实例字段形态）：按字段名读共享的
    /// 引用存储单元。引用字段（含擦除字段）的存储是
    /// `Rc<RefCell<Option<Box<T>>>>`——与 int/long 的 `Cell<i64/i32>` 不同，
    /// 载体类型随字段声明类型异构（`Box<Object>` / `Box<Completion>` / ...），
    /// 无法以统一 cell 类型导出，故以读/写双方法承载（值在边界经
    /// `From<Object>` / `Into<Object>` 转换，与字段访问器的边界协议一致）。
    /// java_class! 宏为每个含引用字段的生成类按平铺字段名单（含继承字段）
    /// 生成臂；其余返回 None（调用方归 stub）。读与 `__get_xxx` 同一存储，
    /// 经 Unsafe/VarHandle 写入对直接字段读取可见（JVM 字段内存语义）；
    /// `None`（未写入）与 `Some(Box<null>)` 均以 jvm-null Object 应答。
    #[doc(hidden)]
    fn __unsafe_ref_get(&self, _field: &str) -> Option<Object> { None }

    /// 引用原子协议的写形态：命中字段名单则写入并返回 true；未命中 → false
    /// （与 `__unsafe_ref_get` 的 None 同一未命中语义，bool 仅为区分「命中」）。
    /// 写入值经 `<T as From<Object>>::from` 还原字段声明类型的视图（null 直通，
    /// 类型不符按 checkcast 语义处理——与 Java 字段存储检查同型）。
    #[doc(hidden)]
    fn __unsafe_ref_set(&self, _field: &str, _v: Object) -> bool { false }
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


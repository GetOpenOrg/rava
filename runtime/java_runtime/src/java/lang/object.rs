//! java.lang.Object — 所有 Java 类的根类型
//! struct 定义永久手写（`__Obj<dyn ObjectVTable>` 是 Rust-specific，无法从字节码生成）

use crate::sync_model::__Shared as Rc;
use crate::obj_ref::__Obj;
// 类视图重建的共用部件（定义在 object_ext，宏生成的 `From<Object>` / `__virtual_view` 转交到这里）
pub use super::object_ext::{__class_from_object, __erased_view, __iface_missing, __PartsFn};

/// JVM Object vtable：方法名与 java.lang.Object 字节码方法一一对应。
///
/// `java_class` 宏为每个具体非接口类自动生成 impl：
///   - `is_instance_of`：静态展开 `all_supertypes` 列表（Arch-2）
///   - `as_any`：返回 `self as &dyn Any`，供 downcast 使用
///   - `toString`：返回 Rust 字符串用于 Display（Arch-4）
///
/// 接口类型（`is_interface = true`）不生成 ObjectVTable impl，
/// 其运行时实例通过 `JvmRef` 包装存储在 Object 中。
pub trait ObjectVTable: 'static + crate::sync_model::__ThreadSafe {
    /// java.lang.Object.hashCode()I 默认实现：身份哈希（实例体地址）——与
    /// `System.identityHashCode`、`Object__hashCode_base` 同一来源（`__identity`），
    /// 未覆盖 hashCode 的类满足 `hashCode() == identityHashCode()`（JLS 契约，S-6）。
    fn hashCode(&self) -> i32 {
        __identity_hash(self.__identity())
    }

    /// java.lang.Object.equals(Object)Z 的虚分派入口：覆盖的类由宏桥接到翻译体；未覆盖的运行时类
    /// 对象执行 `Object.equals` 的字节码翻译体（生成的根类方法体 `Object__equals_body`）。
    /// 非 Java 类载体（[`ObjectVTable::__object`] 为 None）按引用相等应答。
    fn equals(&self, other: Object) -> crate::error::Result<bool> {
        match self.__object() {
            Some(this) => super::object_body::Object__equals_body(&this, other),
            None => Ok(!other.0.is_jvm_null() && self.__identity() == other.0.__identity()),
        }
    }

    /// 本对象的 Object 句柄（根类字节码方法体的 `this`）：运行时类对象——生成类的存储、
    /// `new Object()` 实例、数组——应答 Some（同一对象，引用计数加一）；非 Java 类载体
    /// （基本类型盒、`JvmRef`、lambda 载体、null 哨兵）应答 None，根类方法由各自的载体语义承载。
    #[doc(hidden)]
    fn __object(&self) -> Option<Object> {
        None
    }

    /// 用于 Display/Debug 的 Rust 字符串（内部用途，避免与 Java toString() -> Result<String> 冲突）
    fn __obj_str(&self) -> std::string::String {
        std::any::type_name::<Self>().to_owned()
    }

    /// `Object.toString()` 的虚分派入口（可失败形态，FS-E3）：覆盖 toString 的类由宏桥接到
    /// 翻译体，toString 抛出的异常以 `Err` 传播（可被 catch）；未覆盖的运行时类对象执行
    /// `Object.toString` 的字节码翻译体（`getClass().getName() + "@" + Integer.toHexString(hashCode())`），
    /// 非 Java 类载体回落 `__obj_str`。`__obj_str` 只服务 Rust 侧 Display / Debug（不可失败）。
    fn __to_string(&self) -> crate::error::Result<std::string::String> {
        match self.__object() {
            Some(this) => super::object_body::Object__toString_body(&this).map(|s| s.to_string()),
            None => Ok(self.__obj_str()),
        }
    }

    /// 动态代理钩子（FS-R R4a）：接口载体分派 vtable 未命中时询问接收者。代理载体类
    /// （手写层提供 `__vm_proxy_invoke` 的类，宏据 impl_methods 识别）应答 `Some`——按
    /// (声明接口, 方法名, 描述符) 转发 InvocationHandler；其余对象 `None`（回落 default 体 /
    /// AbstractMethodError）。实参已按 JVM 装箱（基本类型 → 包装对象）。
    fn __proxy_invoke(&self, _iface: &str, _name: &str, _desc: &str, _args: Vec<Object>)
        -> Option<crate::error::Result<Object>> {
        None
    }

    /// instanceof 运行时检查：生成类按本类描述符的超类型名单（本类、父类链、
    /// 全部接口、`java/lang/Object`）判定（S7-1，不再按类展开 matches!）。代理载体另行覆盖。
    fn is_instance_of(&self, type_id: &str) -> bool {
        self.__desc().is_some_and(|d| d.is_subtype_name(type_id))
    }

    /// 运行时类的静态描述符（S7）：生成类返回本类 `X__DESC`；数组、基本类型装箱、
    /// 手写非类对象没有类描述符 → None。
    #[doc(hidden)]
    fn __desc(&self) -> Option<&'static crate::class_desc::__ClassDesc> {
        None
    }

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
    // 以 trait 默认方法提供给全部实现类型（生成类存储、基本类型盒、数组、JvmRef），具体类
    // 不感知；`Object` 包装器的固有方法（object_impl.rs）承载 bare-Object 接收者
    // 的调用（invokevirtual java/lang/Object.*）。wait 三个重载是字节码方法：运行时类对象
    // 转交生成的根类方法体（object_body，等待落到 native `wait0`）；notify / notifyAll 是
    // native，按对象身份（__identity）挂接 monitor.rs 的监视器侧表。重载命名与生成侧同源：
    // wait()V → wait、wait(J)V → wait_l、wait(JI)V → wait_l_i（描述符后缀 _PRIM_SUFFIX，J→l）。
    // 调用点恒为具体类型接收者（生成侧对 wrapper / 数组 / 盒类型静态调用，Object 走固有
    // 方法），故以 `where Self: Sized` 移出 vtable：只在被调用的类型上单态化，不再为每个
    // 实现类型各生成一份（emitter-performance §5.5 N4）。

    /// java.lang.Object.wait()V：运行时类对象转交 Object 的翻译体（final，非虚），载体直接等待
    fn wait(&self) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        match self.__object() {
            Some(this) => super::object_body::Object__wait_body(&this),
            None => crate::monitor::wait_timeout(self.__identity() as usize, 0, 0),
        }
    }

    /// java.lang.Object.wait(J)V：运行时类对象转交 Object 的翻译体（参数校验、虚拟线程中断处理、
    /// native `wait0`）；非 Java 类载体直接等待本对象监视器
    fn wait_l(&self, millis: i64) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        match self.__object() {
            Some(this) => super::object_body::Object__wait_l_body(&this, millis),
            None => crate::monitor::wait_timeout(self.__identity() as usize, millis, 0),
        }
    }

    /// java.lang.Object.wait(JI)V：运行时类对象转交 Object 的翻译体（nanos 校验后转 wait(J)）
    fn wait_l_i(&self, millis: i64, nanos: i32) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        if self.is_jvm_null() {
            return Err(crate::error::JvmError::null_pointer());
        }
        match self.__object() {
            Some(this) => super::object_body::Object__wait_l_i_body(&this, millis, nanos),
            None => crate::monitor::wait_timeout(self.__identity() as usize, millis, nanos),
        }
    }

    /// java.lang.Object.notify()V：唤醒一个在该对象监视器上等待的线程，无等待者时静默。
    fn notify(&self) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        crate::monitor::notify(self.__identity() as usize)
    }

    /// java.lang.Object.notifyAll()V
    fn notify_all(&self) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        crate::monitor::notify_all(self.__identity() as usize)
    }

    /// monitorenter（指令侧）：进入本对象的监视器（可重入）。
    fn monitor_enter(&self) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        crate::monitor::enter(self.__identity() as usize)
    }

    /// monitorexit（指令侧）：退出本对象的监视器一层。
    fn monitor_exit(&self) -> crate::error::Result<()>
    where
        Self: Sized,
    {
        crate::monitor::exit(self.__identity() as usize)
    }

    /// JVM null 检查辅助：Object 持有的是运行时类对象，只有 null 单例与类型化 null 应答 true
    /// （类 wrapper 的 null 判定是其固有方法 `is_jvm_null`，S7-2b）。
    fn is_jvm_null(&self) -> bool { false }

    /// 接口视图查询（接口引用 `__IfaceRef` 建立时的运行时入口，S7-2c）：`slot` 是调用方提供的
    /// `Option<NonNull<dyn I__VTable>>`（I 为目标 Java 接口）；对象的运行时类实现 I 时，
    /// 把自身以该接口的擦除 vtable 形态的指针填入 `slot`（与 `__erased_vtable` 同形，
    /// 调用方以持有本对象的 Object 为句柄，指针不持有）。
    ///
    /// 按「擦除后的接口」选择——`slot` 的类型不含任何类型实参，与 JVM 的 itable 查找一致。
    /// `java_class!` 宏为每个类按其 `impl Iface for Class` 块生成实现；
    /// 默认（未实现任何接口的对象）不填 `slot`。
    #[doc(hidden)]
    fn __interface(&self, _slot: &mut dyn std::any::Any) {}
    /// 运行时类的 binary name（如 `java/lang/NullPointerException`）：生成类取本类描述符（S7-1）；未捕获异常报告等 VM 级设施据此取得类名。
    fn __class_name(&self) -> &'static str {
        match self.__desc() {
            Some(d) => d.binary_name,
            None => "java/lang/Object",
        }
    }

    /// 对象标识（`==` / `!=` 引用比较的依据）：同一 Java 对象的所有引用视图（祖先类 wrapper、
    /// 接口载体、Object）返回同一值。java_class! 宏对生成类 override 为对象存储的标识单元。
    #[doc(hidden)]
    fn __identity(&self) -> *const () {
        self as *const Self as *const ()
    }

    /// 以 Object 流转的数组（`Object::array_length`，S-2.2）：本运行时类是数组 → 长度；
    /// 非数组 → None。JArray 与数组载体（Rc<RefCell<Vec<T>>>）override；元素类型对长度
    /// 无关紧要，故不按元素类型分支，避免枚举所有实例化。
    #[doc(hidden)]
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { None }

    /// 擦除视图导出（S7-2）：`slot` 是调用方（`From<Object> for X<A>` / `X::__virtual_view`，
    /// 知道目标类 X）构造的 `Option<NonNull<dyn X__VTable>>`。运行时类是 X 或 X 的子类时，
    /// 生成类的存储把自身以 X 的擦除 vtable 形态的指针填入（类 vtable trait 非泛型、超类链是其
    /// supertrait —— 子类 vtable 直接上转）；调用方与句柄合成 `__Ref`，对任意类型实参成立。
    /// 其余对象（基本类型、闭包、接口载体、wrapper 本身等）不填 `slot`。
    #[doc(hidden)]
    fn __erased_vtable(&self, _slot: &mut dyn std::any::Any) {}

    /// 数组协变的元素赋值兼容探针（S-4）：receiver 是引用元素数组（JArray），调用方
    /// （`From<Object> for JArray<T>`，知道目标元素类型 T）给出 T 的 binary name `target_elem`
    /// 与 `Option<T>` 的 `slot`。以「源元素类型的 null 探针」判定：探针是类 → 按其描述符的
    /// 超类型名单（与元素值无关）；探针是数组 → view_into 该 slot（嵌套数组的 `Object[]` 上转）。
    /// 成立 ⇔ T 是源元素类型自身或其超类型（JLS §4.10.3 数组子类型条件）。
    /// 非数组对象不响应（默认 false，checkcast 由其余钩子判定）。
    #[doc(hidden)]
    fn __array_elem_assignable(&self, _target_elem: &str, _slot: &mut dyn std::any::Any) -> bool { false }

    /// 多维数组的元素级 checkcast 探针：receiver 是**目标**数组类型 `JArray<U>` 的 null 探针
    /// （`T::default()`），`candidate` 是源数组的一个元素。数组探针按 `try_array_view::<U>`
    /// 判定（与 `From<Object> for JArray<U>` 同一决策点，递归覆盖任意维数与协变上转——
    /// `Set<String>[][] t = new HashSet[n][n]` 的内层 `HashSet[]` → `Set[]`）。
    /// 非数组探针不响应（默认 false，元素兼容由其余臂判定）。
    #[doc(hidden)]
    fn __array_accepts(&self, _candidate: &Object) -> bool { false }

    /// 数组的类型驱动视图：`slot` 是 `Option<JArray<E>>` 等数组形态时按本数组重建该视图写入
    /// `slot` 并返回 true；否则返回 false。类对象不响应——类目标的判定读描述符（S7-1），视图经
    /// `From<Object>` 的擦除重建取得。
    fn __view_into(
        &self,
        _any: crate::sync_model::__AnyRef,
        _slot: &mut dyn std::any::Any,
    ) -> bool { false }

    /// `Object.clone()` 的 native 语义：新建同运行时类的对象，逐字段拷贝（浅拷贝）。
    /// 类对象不覆盖——按描述符的 `fields` 拷贝（`field_desc::__clone_fields`）；数组覆盖；
    /// 无字段存储的值（装箱基本类型等）返回 None。
    fn __shallow_copy(&self) -> Option<Object> {
        None
    }
}

/// 引导映像中 `new Object()` 实例的值类型（映像模块按此发射对象值）
#[doc(hidden)]
pub use super::object_impl::Instance as __ObjectInstance;

/// 身份哈希（`Object.hashCode` / `System.identityHashCode` 的唯一来源，FS-M5）：
/// 实例体地址经 SplitMix64 混合取 31 位——非负、非零（HotSpot markWord 的 31 位 hash 域，
/// 0 保留为「未计算」，取 0 时换 0xBAD），低位分布均匀（地址对齐使低位恒 0，直接截断
/// 会让 HashMap 桶分布退化）。对同一实例恒定；实例存活期间地址唯一，故不同存活对象
/// 的哈希只在混合碰撞时相同（与 HotSpot 随机哈希同等概率级别）。
#[inline]
pub fn __identity_hash(id: *const ()) -> i32 {
    // 映像对象返回构建期取得的值：构建期建好的哈希表桶位在运行期仍然有效（引导映像 §3.6）
    if let Some(h) = crate::obj_ref::__image_hash(id) {
        return h;
    }
    let mut z = (id as usize as u64).wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^= z >> 31;
    let v = (z as u32) & 0x7FFF_FFFF;
    (if v == 0 { 0xBAD } else { v }) as i32
}

/// `super.clone()`（invokespecial java/lang/Object.clone）的落点。
/// Object.clone 是 ACC_NATIVE：运行时类未实现 Cloneable 时抛 CloneNotSupportedException，
/// 否则返回逐字段浅拷贝。
#[allow(non_snake_case)]
pub fn Object__clone_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<Object> {
    if !this.is_instance_of("java/lang/Cloneable") {
        return Err(crate::error::JvmError::clone_not_supported(this.__class_name()));
    }
    let copy = match this.__desc() {
        // 类对象：按运行时类描述符逐字段拷贝（S7-3，非泛型）
        Some(desc) if !this.is_jvm_null() =>
            // SAFETY: 报出描述符的非 null 对象即该类的存储（`X__inner`，`#[repr(C)]` 细指针字段）
            Some(unsafe { crate::field_desc::__clone_fields(this as *const T as *const (), desc) }),
        _ => this.__shallow_copy(),
    };
    match copy {
        Some(copy) => Ok(copy),
        None => Err(crate::error::JvmError::clone_not_supported(this.__class_name())),
    }
}

/// `super.hashCode()`（invokespecial java/lang/Object.hashCode）的落点。
/// Object.hashCode 是 ACC_NATIVE：身份哈希，取实例体的堆地址（与 `new Object()` 实例一致）。
#[allow(non_snake_case)]
pub fn Object__hashCode_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<i32> {
    Ok(__identity_hash(this.__identity()))
}

/// `super.finalize()`（invokespecial java/lang/Object.finalize）的落点：Object.finalize 的翻译体。
/// GC 触发的终结调用不建模（无 GC，见 compatibility.md）。
#[allow(non_snake_case)]
pub fn Object__finalize_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<()> {
    match this.__object() {
        Some(o) => super::object_body::Object__finalize_body(&o),
        None => Ok(()),
    }
}

/// `invokespecial java/lang/Object.getClass` 的落点（javac 对 `super.getClass()` 发射）：
/// Object.getClass 是 final ACC_NATIVE，非虚入口与虚入口同义——取运行时类。
#[allow(non_snake_case)]
pub fn Object__getClass_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<crate::java::lang::Class> {
    this.getClass()
}

/// `super.equals(o)`（invokespecial java/lang/Object.equals）的落点：Object.equals 的翻译体
/// （引用相等 `this == o`）；非 Java 类载体按对象标识比较。
#[allow(non_snake_case)]
pub fn Object__equals_base<T: ObjectVTable + ?Sized>(this: &T, other: Object) -> crate::error::Result<bool> {
    match this.__object() {
        Some(o) => super::object_body::Object__equals_body(&o, other),
        None => Ok(!other.0.is_jvm_null() && this.__identity() == other.0.__identity()),
    }
}

/// `super.toString()`（invokespecial java/lang/Object.toString）的落点：Object.toString 的翻译体
/// （`getClass().getName() + "@" + Integer.toHexString(hashCode())`，hashCode 走虚派发）。
#[allow(non_snake_case)]
pub fn Object__toString_base<T: ObjectVTable + ?Sized>(this: &T) -> crate::error::Result<crate::java::lang::String> {
    match this.__object() {
        Some(o) => super::object_body::Object__toString_body(&o),
        None => Ok(crate::java::lang::String::from(this.__obj_str())),
    }
}

// ── 基本类型 ObjectVTable impl（int 装箱进 Object 的场景）─────────────────────
//
// S-3.1 后装箱类型（Integer/Long/...）是字节码翻译出的真实类，`Integer.valueOf`
// 等工厂由翻译体承载（含缓存池语义）。原生值只在「未经 javac 装箱就流入 Object
// 位置」的角落出现（类型变量擦除边界、手写层的 Object::from_any(i32) 等）：
// 此时基本类型盒自身承载对应包装类的 binary name —— getClass()、instanceof、
// 异常消息对这些值给出与 JVM 一致的答案。这与翻译类路径（wrapper 的 vtable）
// 互不冲突：Rc<i32> 与 Rc<Integer> 的 TypeId 不同，downcast 各自精确命中。
/// 基本类型装箱（Integer / Long / ... 的原生值盒，反射 / VarHandle 等运行时路径产出）：
/// `hashCode` / `equals` 按包装类语义——值哈希（`Integer.hashCode` 等静态形态）；值等要求
/// 同一包装类且值文本相同（对方可为原生值盒或翻译出的包装类实例，二者的 `__obj_str` 均为
/// Java `toString` 文本，与 `reflect_dispatch::unbox_*` 同一识别约定）。浮点的 `toString`
/// 文本相等 ⇔ `doubleToLongBits` 相等（NaN 自等、0.0 与 -0.0 不等），与 `Double.equals` 一致。
macro_rules! impl_vtable_primitive {
    ($t:ty, $bin:literal, $fmt:expr, $hash:expr) => {
        impl ObjectVTable for $t {
            fn __obj_str(&self) -> std::string::String { ($fmt)(*self) }
            fn __class_name(&self) -> &'static str { $bin }
            fn is_instance_of(&self, type_id: &str) -> bool { type_id == $bin }
            fn as_any(&self) -> &dyn std::any::Any { self }
            fn hashCode(&self) -> i32 { ($hash)(*self) }
            fn equals(&self, other: Object) -> crate::error::Result<bool> {
                if other.0.is_jvm_null() || other.0.__class_name() != $bin {
                    return Ok(false);
                }
                // 同为原生值盒：按值位比较（浮点经规范化位：NaN 自等、0.0 与 -0.0 不等）
                if let Some(o) = other.0.as_any().downcast_ref::<$t>() {
                    return Ok(__prim_bits_eq(self, o));
                }
                Ok(other.0.__obj_str() == ($fmt)(*self))
            }
        }
    };
}

/// 原生值盒的值位相等（`Double.equals` / `Float.equals` 的 `doubleToLongBits` 比较；
/// 整型等价 `==`）。
fn __prim_bits_eq<T: __PrimBits>(a: &T, b: &T) -> bool { a.__bits() == b.__bits() }
trait __PrimBits { fn __bits(&self) -> u64; }
impl __PrimBits for i32 { fn __bits(&self) -> u64 { *self as u32 as u64 } }
impl __PrimBits for i64 { fn __bits(&self) -> u64 { *self as u64 } }
impl __PrimBits for bool { fn __bits(&self) -> u64 { *self as u64 } }
impl __PrimBits for i8 { fn __bits(&self) -> u64 { *self as u8 as u64 } }
impl __PrimBits for i16 { fn __bits(&self) -> u64 { *self as u16 as u64 } }
impl __PrimBits for u16 { fn __bits(&self) -> u64 { *self as u64 } }
impl __PrimBits for f32 { fn __bits(&self) -> u64 { __canon_f32_bits(*self) as u64 } }
impl __PrimBits for f64 { fn __bits(&self) -> u64 { __canon_f64_bits(*self) } }
fn __canon_f64_bits(v: f64) -> u64 { if v.is_nan() { 0x7ff8000000000000 } else { v.to_bits() } }
fn __canon_f32_bits(v: f32) -> u32 { if v.is_nan() { 0x7fc00000 } else { v.to_bits() } }
/// 基本类型值装入 Object（原生值盒，见上）：逐类型显式 `From`（S7-2b 删 blanket `From<T: ObjectVTable>`）
macro_rules! impl_from_primitive {
    ($($t:ty),*) => { $(impl From<$t> for Object { fn from(v: $t) -> Object { Object::__alloc(v) } })* };
}
impl_from_primitive!(i32, i64, bool, i8, i16, u16, f32, f64);
impl_vtable_primitive!(i32, "java/lang/Integer", |v: i32| format!("{}", v), |v: i32| v);
impl_vtable_primitive!(i64, "java/lang/Long", |v: i64| format!("{}", v),
    |v: i64| (v ^ ((v as u64) >> 32) as i64) as i32);
impl_vtable_primitive!(bool, "java/lang/Boolean", |v: bool| format!("{}", v),
    |v: bool| if v { 1231 } else { 1237 });
impl_vtable_primitive!(i8,  "java/lang/Byte", |v: i8| format!("{}", v), |v: i8| v as i32);
impl_vtable_primitive!(i16, "java/lang/Short", |v: i16| format!("{}", v), |v: i16| v as i32);
impl_vtable_primitive!(u16, "java/lang/Character", crate::java_fmt_char, |v: u16| v as i32);
impl_vtable_primitive!(f32, "java/lang/Float", crate::java_fmt_f32,
    |v: f32| __canon_f32_bits(v) as i32);
impl_vtable_primitive!(f64, "java/lang/Double", crate::java_fmt_f64,
    |v: f64| { let b = __canon_f64_bits(v); (b ^ (b >> 32)) as i32 });

/// Java null 的静态哨兵（不计数，见 `obj_ref`）：`Object::default()` 与无静态类型的 null 共用这一个值。
/// 全部 null（本值、各 `__TypedNull`、接口 `__TYPED_NULL`、`__ArrayNull`）的身份都是本值地址，
/// 监视器入口据此一次比较判 null（`monitor::null_identity`）。
pub(crate) static JVM_NULL: __TypedNull = __TypedNull::new("java/lang/Object", None);

/// 带静态类型的 null：接口载体（`java_class!` 接口块）与类 wrapper 的 null 装入 Object 的形态。
/// 值语义仍是 Java null（`is_jvm_null`，身份即 null 哨兵，与任意 null 引用相等），但
/// `__class_name` 报静态类型；类 wrapper 的 null 另带本类描述符（`__desc` / `is_instance_of`
/// 按静态类应答，与 S7-2b 前 null wrapper 装入 Object 的应答相同）。数组以元素类型的 null 探针
/// 取元素类（`new I[0].getClass()` 为 `[LI;`、aastore 存储检查的元素类名、checkcast 的目标元素类），
/// 接口元素数组据此得到 JVM 的数组类，而非退化为 `Object[]`。
///
/// 每个静态类型一个 `'static` 值（类与接口由 `java_class!` 生成 `static`），以不计数的哨兵装入
/// Object：克隆 / 释放 null 不触碰任何共享计数。
#[doc(hidden)]
pub struct __TypedNull(&'static str, Option<&'static crate::class_desc::__ClassDesc>);

impl __TypedNull {
    #[doc(hidden)]
    pub const fn new(binary_name: &'static str, desc: Option<&'static crate::class_desc::__ClassDesc>) -> Self {
        __TypedNull(binary_name, desc)
    }
}

impl ObjectVTable for __TypedNull {
    fn __obj_str(&self) -> std::string::String { "null".to_owned() }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn is_jvm_null(&self) -> bool { true }
    fn __class_name(&self) -> &'static str { self.0 }
    fn __desc(&self) -> Option<&'static crate::class_desc::__ClassDesc> { self.1 }
    fn __identity(&self) -> *const () { &raw const JVM_NULL as *const () }
}

crate::__process_static! {
    /// 类型化 null 的缓存：键为静态类 binary name（同名共享一个实例）
    static TYPED_NULLS: crate::sync_model::__RefSlot<std::collections::HashMap<&'static str, Object>> =
        crate::sync_model::__RefSlot::new(std::collections::HashMap::new());
}

impl Object {
    /// 静态类型为 `binary_name` 的 null（按名缓存，同名共享一个实例）。
    #[doc(hidden)]
    pub fn __typed_null(binary_name: &'static str) -> Object {
        Self::__typed_null_of(binary_name, None)
    }

    /// 类 wrapper 的 null 装入 Object（`From<X> for Object` 的 null 臂，S7-2b）：静态类型取自
    /// 本类描述符，取该类的类型化 null 哨兵（描述符的 `typed_null`）。
    #[doc(hidden)]
    #[inline]
    pub fn __typed_null_desc(desc: &'static crate::class_desc::__ClassDesc) -> Object {
        Object::__from_static(desc.typed_null)
    }

    /// 静态值以不计数的哨兵装入 Object（类型化 null）。
    #[doc(hidden)]
    #[inline]
    pub fn __from_static(value: &'static __TypedNull) -> Object {
        Object(__Obj::from_static(value as &'static dyn ObjectVTable))
    }

    /// 静态哨兵装入 Object（常量求值可用：引导映像中引用数组的 null 元素）
    #[doc(hidden)]
    pub const fn __const_static(value: &'static __TypedNull) -> Object {
        Object(__Obj::from_static_const(value as &'static dyn ObjectVTable))
    }

    /// 无静态类型的 null 常量（引导映像）
    #[doc(hidden)]
    pub const __NULL: Object = Object::__const_static(&JVM_NULL);

    fn __typed_null_of(binary_name: &'static str,
                       desc: Option<&'static crate::class_desc::__ClassDesc>) -> Object {
        if let Some(n) = TYPED_NULLS.with(|m| m.borrow().get(binary_name).cloned()) {
            if n.0.__desc().is_some() || desc.is_none() {
                return n;
            }
        }
        // 运行期按名建立的类型化 null：每名一个泄漏的 'static 值（名字集合有界：静态类型名）。
        // 同名的接口载体 null 与类 null 不会并存（类与接口不同名）；带描述符的覆盖无描述符的
        let n = Object::__from_static(Box::leak(Box::new(__TypedNull(binary_name, desc))));
        TYPED_NULLS.with(|m| m.put(binary_name, Clone::clone(&n)));
        n
    }

    /// 新分配一个运行时对象并装入 Object（基本类型盒、`new Object()`、lambda 对象等非类存储）。
    #[doc(hidden)]
    #[inline]
    pub fn __alloc<T: ObjectVTable>(value: T) -> Object {
        Object(__Obj::new(value).map_ptr(|p| p as *mut dyn ObjectVTable))
    }

    /// 以存储自身重建 Object（`ObjectVTable::__object`）：引用计数加一，同一对象。
    ///
    /// # Safety
    /// `value` 必须是某个 `__Obj` 所持的堆值或映像值（不得是静态哨兵或栈上的值）。
    #[doc(hidden)]
    #[inline]
    pub unsafe fn __from_storage<T: ObjectVTable>(value: &T) -> Object {
        // SAFETY: 调用方保证 value 位于 `__Obj` 分配（或带头部的映像对象）中
        Object(unsafe { __Obj::from_value(value) }.map_ptr(|p| p as *mut dyn ObjectVTable))
    }

    /// 存储（运行时类对象）装入 Object：句柄交出所持对象，或分配后直接装入（S7-2b）。
    #[doc(hidden)]
    #[inline]
    pub fn __from_shared(rc: __Obj<dyn ObjectVTable>) -> Object { Object(rc) }
}

/// 数组类型（Rc<RefCell<Vec<T>>>）自动装入 Object
impl<T: 'static + crate::sync_model::__ThreadSafe> From<Rc<crate::sync_model::__RefSlot<Vec<T>>>> for Object {
    fn from(v: Rc<crate::sync_model::__RefSlot<Vec<T>>>) -> Object { Object::__alloc(v) }
}
impl<T: 'static + crate::sync_model::__ThreadSafe> ObjectVTable for Rc<crate::sync_model::__RefSlot<Vec<T>>> {
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
impl<T: 'static + crate::sync_model::__ThreadSafe> From<JvmRef<T>> for Object {
    fn from(v: JvmRef<T>) -> Object { Object::__alloc(v) }
}
impl<T: 'static + crate::sync_model::__ThreadSafe> ObjectVTable for JvmRef<T> {
    fn as_any(&self) -> &dyn std::any::Any { &self.0 }
    fn __obj_str(&self) -> std::string::String {
        let v: &dyn std::any::Any = &self.0;
        macro_rules! try_fmt {
            ($t:ty) => { if let Some(x) = v.downcast_ref::<$t>() { return format!("{}", x); } };
        }
        try_fmt!(i32); try_fmt!(i64); try_fmt!(bool);
        try_fmt!(f32); try_fmt!(f64); try_fmt!(i8); try_fmt!(i16);
        if let Some(x) = v.downcast_ref::<u16>() { return crate::java_fmt_char(*x); }
        std::any::type_name::<T>().to_owned()
    }
}

/// Object 所持指针上的 null 判定（`obj.0.is_jvm_null()`）：Java null 恰为不计数的静态哨兵
/// （`JVM_NULL`、类 / 接口类型化 null、`__ArrayNull<T>`），堆对象恒非 null——只测指针标记位，
/// 不经 vtable。固有方法先于 `ObjectVTable::is_jvm_null` 解析；vtable 的应答与之一致。
impl __Obj<dyn ObjectVTable> {
    #[inline]
    pub fn is_jvm_null(&self) -> bool {
        debug_assert_eq!(self.is_static(), (**self).is_jvm_null(), "null 与静态哨兵不一致");
        self.is_static()
    }
}

/// `Object` — 所有 Java 类的运行时表示。
///
/// 内部结构：`__Obj<dyn ObjectVTable>`（引用计数指针，null 为不计数的静态哨兵，见 `obj_ref`）
///   - 具体类通过 `java_class` 宏的 `impl ObjectVTable` 直接存储
///   - 基本类型通过 primitive ObjectVTable impl 直接存储
///   - 泛型参数/接口类型通过 `JvmRef<T>` 包装存储
///   - 通过 `as_any()` + `downcast_ref` 实现类型还原
#[derive(Clone)]
pub struct Object(pub __Obj<dyn ObjectVTable>);

/// 释放：最后一个强引用经 `handle::__release` 计深释放（S7-3x 非递归释放），槽位换成 null 哨兵（不计数）。
impl Drop for Object {
    #[inline]
    fn drop(&mut self) {
        if !self.0.is_unique() {
            return;
        }
        let null = __Obj::from_static(&JVM_NULL as &'static dyn ObjectVTable);
        crate::handle::__release(std::mem::replace(&mut self.0, null));
    }
}

/// `(void) obj` —— 丢弃引用；使 `()` 满足类型实参的 `From<Object>` 约束。
impl From<Object> for () {
    fn from(_: Object) {}
}

/// `()` 装入 Object 即 Java null（使 `()` 满足类型实参的 `Into<Object>` 约束）。
impl From<()> for Object {
    fn from(_: ()) -> Object { Object::default() }
}

/// Java null 的唯一实例（S-3.2）：所有无静态类型的 null 共享 `JVM_NULL` 哨兵，按身份比较的路径
/// （`Object__equals_base` 的指针相等、协变视图的 `identity` 等）与 `PartialEq` 的 null 短路
/// （object_ext.rs）语义一致。哨兵不计数：取 null 不读写任何共享状态。
impl Default for Object {
    #[inline]
    fn default() -> Self {
        Object::__from_static(&JVM_NULL)
    }
}

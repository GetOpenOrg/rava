// Java 数组类型封装：可读层包装
// 调用方只需 array.get(i)、array.set(i, v)、array.len()；元素存储无锁（见 array/store.rs）

use crate::sync_model::__Shared as Rc;

use crate::java::lang::Object;

mod vtable;
use vtable::*;
mod view;
use view::*;
mod store;
use store::*;
mod obj;
pub use obj::{__ArrayNull, __ArrayObj, __ImageArr};
use obj::{Form, Repr};
use crate::obj_ref::__Obj;

/// Java 数组。Clone 共享底层存储（Java 数组是引用类型，赋值不复制内容）。
///
/// 数组协变（JLS §4.10.3：`Number[] na = new Integer[n]`）：引用元素数组以任意祖先
/// 元素静态类型流转时是 `Covariant` 视图——存储保持类型擦除（读写经源数组完成，
/// 元素类型转换发生在 `JArray<T>` 泛型边界：读出按目标元素类型重建视图，写入按源
/// 元素类型做存储检查，对应 aastore，失败抛 `ArrayStoreException`），对象标识与源
/// 数组相同；还原为源类型（`(Integer[]) na`）取回源数组本身。
///
/// Null（S-3.1）：数组引用可为 null（未初始化的静态字段、`long[] a = null`），表示为无存储
/// （`None`，不分配）。null 与 `JArray::new(0)`（真实存在的空数组）严格区分：null 上的
/// get/set/len 抛 NullPointerException（JVMS §6.5 arraylength/*aload/*astore），
/// `is_jvm_null()` 为 true；null 装入 Object 后以 vtable 的 is_jvm_null 呈现 null 语义
/// （null 通过任意 checkcast、与任何非 null 引用不等）。
///
/// 存储形态（R1）：非 null 数组是一个 `__ArrayObj<T>` 对象，元素紧随其后、同一分配；装入
/// Object 只换指针类型。null 数组装入 Object 为 `__ArrayNull<T>` 静态哨兵（见 array/obj.rs）。
pub struct JArray<T>(Option<__Obj<__ArrayObj<T>>>);

/// aastore 存储检查（JLS §10.5 / JVMS §6.5 aastore）：值与源元素类型赋值兼容才能写入，
/// 否则抛 `ArrayStoreException`。判定按序三条：
///   1. null 可存入任意引用元素数组；
///   2. 数组值的类型驱动视图（`__view_into` 填 `Option<T>` slot，T 为数组形态）——
///      多维数组的元素也是数组，视图经 Covariant 委托到源数组判定；
///   3. 按运行时类的超类型名单 `is_instance_of`（类值读描述符，S7-1）——覆盖本类、
///      子类与实现接口的值，与值以何种静态 wrapper 视图流转无关。
fn aastore_storable<T: Clone + From<Object> + 'static>(v: &Object, elem_name: &str) -> bool {
    if v.0.is_jvm_null() {
        return true;
    }
    let mut slot: Option<T> = None;
    let unused = crate::sync_model::__unused_any();
    if v.0.__view_into(unused, &mut slot) && slot.is_some() {
        return true;
    }
    v.0.is_instance_of(elem_name)
}

impl<T> JArray<T> {
    /// 映像数组的引用（常量求值可用，引导映像物化）
    #[doc(hidden)]
    pub const fn __image(value: &'static __ArrayObj<T>) -> Self { JArray(Some(__Obj::image(value))) }
}

impl<T> Clone for JArray<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        JArray(match &self.0 {
            Some(o) => Some(o.clone()),
            None => None,
        })
    }
}

impl<T> std::fmt::Debug for JArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(r) => write!(f, "JArray@{:p}", r.as_ptr()),
            None => f.write_str("JArray(null)"),
        }
    }
}

/// Java 数组引用的默认值是 null（局部变量 / 未初始化字段 / 静态字段单元格的
/// `.unwrap_or_default()`），与 `new T[0]`（真实空数组）不同。
impl<T: 'static> Default for JArray<T> {
    fn default() -> Self { JArray(None) }
}

impl<T: 'static> PartialEq for JArray<T> {
    fn eq(&self, other: &Self) -> bool {
        // null == null（Java 引用比较）；null 与任何真实数组不等（null 的标识为空指针）
        self.identity() == other.identity()
    }
}

impl<T: 'static> JArray<T> {
    pub(crate) fn identity(&self) -> *const () {
        match self.0.as_deref() {
            Some(a) => a.identity(),
            None => std::ptr::null(),
        }
    }

    /// 本引用是否为 Java null（ifnull/ifnonnull 的接收者）。
    #[inline]
    pub fn is_jvm_null(&self) -> bool {
        self.0.is_none()
    }

    /// 非 null 数组的数组对象；null 抛 NullPointerException（JVMS §6.5 *aload / *astore / arraylength）
    #[inline(always)]
    fn obj(&self) -> crate::error::Result<&__ArrayObj<T>> {
        match &self.0 {
            Some(o) => Ok(o),
            None => Err(crate::error::JvmError::null_pointer()),
        }
    }

    /// 非 null 数组的存取形态
    #[inline]
    fn form(&self) -> Option<Form<'_, T>> {
        self.0.as_deref().map(__ArrayObj::form)
    }

    /// 自有元素数组（一次分配，元素由 `elems` 逐个给出，恰好 `len` 个）
    fn own(len: usize, tag: Option<Rc<str>>, elems: impl IntoIterator<Item = T>) -> Self {
        JArray(Some(__ArrayObj::own(len, tag, elems)))
    }

    fn covariant(view: CovariantView) -> Self {
        JArray(Some(__ArrayObj::covariant(view)))
    }

    fn has_primitive_elements() -> bool {
        let element = std::any::TypeId::of::<T>();
        [
            std::any::TypeId::of::<i8>(), std::any::TypeId::of::<i16>(), std::any::TypeId::of::<u16>(),
            std::any::TypeId::of::<i32>(), std::any::TypeId::of::<i64>(), std::any::TypeId::of::<f32>(),
            std::any::TypeId::of::<f64>(), std::any::TypeId::of::<bool>(),
        ].contains(&element)
    }

    /// 基本元素类型的 JVM 描述符字符（静态分派，与 has_primitive_elements 同一
    /// TypeId 手法）；引用元素类型 → None。
    fn primitive_elem_descriptor() -> Option<&'static str> {
        let element = std::any::TypeId::of::<T>();
        Some(if element == std::any::TypeId::of::<i8>() { "B" }
            else if element == std::any::TypeId::of::<i16>() { "S" }
            else if element == std::any::TypeId::of::<u16>() { "C" }
            else if element == std::any::TypeId::of::<i32>() { "I" }
            else if element == std::any::TypeId::of::<i64>() { "J" }
            else if element == std::any::TypeId::of::<f32>() { "F" }
            else if element == std::any::TypeId::of::<f64>() { "D" }
            else if element == std::any::TypeId::of::<bool>() { "Z" }
            else { return None })
    }
}

impl<T: Clone + Default + 'static> JArray<T> {
    /// 创建长度为 len 的数组，元素初始化为类型默认值（对应 Java newarray/anewarray）。
    ///
    /// 负长度在此饱和为空数组（手写 native 内部路径的进程崩溃防线：`vec![T; len as usize]`
    /// 对负 len 是不可捕获的 `capacity overflow` panic）。Java 语义的创建点
    /// （newarray/anewarray 字节码翻译）必须走 [`Self::try_new`]——负长度抛
    /// `NegativeArraySizeException`（JVMS §6.5，Err 形态可被 java_try 捕获）。
    pub fn new(len: i32) -> Self {
        JArray(Some(__ArrayObj::own_default(len.max(0) as usize)))
    }

    /// `newarray`/`anewarray` 的可失败创建：负长度抛 `NegativeArraySizeException`
    /// （真实异常对象，与数组 get/set 的 Result 机制一致）。
    pub fn try_new(len: i32) -> crate::error::Result<Self> {
        if len < 0 {
            return Err(crate::error::JvmError::negative_array_size(len));
        }
        Ok(JArray(Some(__ArrayObj::own_default(len as usize))))
    }

    /// 创建长度为 len 的数组，每个元素由 init 独立构造（对应 Java multianewarray：
    /// 每一行是独立的数组对象，不能共享同一个默认值的引用）。负长度饱和为空数组
    /// （同 [`Self::new`] 的崩溃防线语义）。
    pub fn new_with(len: i32, init: impl Fn() -> T) -> Self {
        let len = len.max(0) as usize;
        JArray::own(len, None, std::iter::repeat_with(init))
    }

    /// `multianewarray` 的可失败创建：任一已给维度为负即抛 `NegativeArraySizeException`——
    /// 外层维先行检查（外层为 0 时内层闭包不执行，与 Java `new int[0][-1]` 不抛一致），
    /// 内层维的 Err 在逐行构造中传播。每行由 init 独立构造（行间不共享引用）。
    pub fn try_new_with(
        len: i32,
        init: impl Fn() -> crate::error::Result<T>,
    ) -> crate::error::Result<Self> {
        if len < 0 {
            return Err(crate::error::JvmError::negative_array_size(len));
        }
        let mut rows = Vec::with_capacity(len as usize);
        for _ in 0..len {
            rows.push(init()?);
        }
        Ok(JArray::from(rows))
    }
}

impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> JArray<T> {
    /// 读取下标 i 的元素（对应 Java iaload/aaload 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *aload）；
    /// null 引用抛 `NullPointerException`。
    #[inline(always)]
    pub fn get(&self, i: i32) -> crate::error::Result<T> {
        self.obj()?.get(i)
    }

    /// 元素的原子读-改-写（Unsafe / VarHandle 数组元素 CAS 族）：基本元素为元素原子单元上的
    /// CAS 循环（重试时 `f` 重新求值），引用元素在元素单元锁内读出 `cur`、`f(cur)` 给 `Some(new)`
    /// 时写入；返回 `cur`。协变视图经 `update` 闭包委托源数组的 `__update`。
    pub fn __update(&self, i: i32, f: &mut dyn FnMut(T) -> Option<T>) -> crate::error::Result<T> {
        self.obj()?.__update(i, f)
    }

    /// 写入下标 i 的元素（对应 Java iastore/aastore 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *astore）；
    /// null 引用抛 `NullPointerException`。
    #[inline(always)]
    pub fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        self.obj()?.set(i, v)
    }

    /// 元素快照（手写 VM 层批量读取用，逐元素读）
    pub fn to_vec(&self) -> Vec<T> {
        match self.0.as_deref() {
            Some(a) => a.to_vec(),
            None => panic!("NullPointerException: 对 null 数组做批量读取"),
        }
    }

    /// 数组长度（对应 Java arraylength 字节码）。null 引用抛 NullPointerException
    /// （JVMS §6.5 arraylength：objectref 为 null 时抛 NPE）。
    #[inline(always)]
    pub fn len(&self) -> crate::error::Result<i32> {
        Ok(self.obj()?.len())
    }

    /// 辅助谓词（非字节码语义）：null 视为无元素。
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }
}

impl<T: 'static> From<Vec<T>> for JArray<T> {
    /// 从 Vec<T> 构造，用于字面量数组初始化（对应 Java 数组初始化器）
    fn from(v: Vec<T>) -> Self {
        JArray::own(v.len(), None, v)
    }
}

impl JArray<crate::java::lang::String> {
    /// 字符串字面量数组初始化器（`new String[]{"a", "b"}`）的紧凑形态：元素表为静态
    /// 切片，逐元素建 Java String。与 `JArray::from(vec![String::from(..), ..])` 等价，
    /// 但数千元素的资源束字面量表（CLDR getContents）不再展开为数千个表达式节点。
    pub fn from_strs(v: &[&str]) -> Self {
        JArray::own(v.len(), None, v.iter().map(|s| crate::java::lang::String::from(*s)))
    }
}

impl JArray<Object> {
    /// 反射创建的引用数组（`Array.newInstance(componentType, len)`，FS-R6）：擦除载体
    /// + 组件类型标签（binary name，斜线形态：`java/lang/String`、`[I`）。元素初值 null。
    pub fn __new_component_tagged(len: i32, component: &str) -> Self {
        let tag: Option<Rc<str>> = if component == "java/lang/Object" { None } else { Some(Rc::from(component)) };
        JArray::own(len.max(0) as usize, tag, std::iter::repeat_with(Object::default))
    }

    /// 组件类型标签（仅反射创建的引用数组；静态类型数组与视图为 None）。
    pub(crate) fn __component_tag(&self) -> Option<std::string::String> {
        match self.form() {
            Some(Form::Own(_, Some(tag))) => Some(tag.to_string()),
            _ => None,
        }
    }

    /// `new Object[]{"k", "v"}`（元素全为字符串字面量）的紧凑形态，语义同上。
    pub fn objects_from_strs(v: &[&str]) -> Self {
        JArray::own(v.len(), None, v.iter().map(|s| Object::from(crate::java::lang::String::from(*s))))
    }
}

/// Java 数组是对象：可直接装入 Object（`Object o = arr;`）——数组对象本身即 Object 所持对象，
/// 只换指针类型、不分配；null 数组为本元素类型的静态哨兵（不分配、不计数）。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> From<JArray<T>> for Object {
    #[inline]
    fn from(a: JArray<T>) -> Object {
        use crate::java::lang::ObjectVTable;
        match a.0 {
            Some(o) => Object::__from_shared(o.map_ptr(|p| p as *mut dyn ObjectVTable)),
            None => Object::__from_shared(__Obj::from_static(
                const { &__ArrayNull::<T>::NULL } as &'static dyn ObjectVTable)),
        }
    }
}

/// 擦除数组的逐元素兼容判定（`From<Object> for JArray<T>` 的擦除还原臂，
/// A-3）：源数组可按 Object 级协变视图观察、且每个非 null 元素与目标元素
/// 类型 T 赋值兼容 → checkcast 到 `JArray<T>` 成立（泛型数组的擦除还原
/// 路径：`(String[]) objArr`，源静态元素类型是 T 的祖先形态）。空数组 /
/// 全 null 恒兼容。
///
/// 元素兼容按序三臂：`try_checkcast::<T>`（同类型；数组元素——`__view_into`
/// 驱动，元素本身是数组与数组协变视图由此臂与 `__array_accepts` 判定）；T 的
/// null 探针类名非退化（`java/lang/Object`——T 是接口别名时探针失义）时按
/// `is_instance_of`（类元素读描述符：本类、子类、实现接口；另覆盖未经 javac
/// 装箱、以原生值盒（`Rc<i32>` 带 Integer vtable）流入 Object 槽位的元素）。
pub(crate) fn erased_array_compatible<T: Clone + Default + Into<Object> + 'static>(obj: &Object) -> bool {
    let unused = crate::sync_model::__unused_any();
    let mut erased: Option<JArray<Object>> = None;
    obj.0.__view_into(unused, &mut erased);
    if let Some(view) = erased {
        let probe: Object = Into::<Object>::into(T::default());
        let t_name = probe.0.__class_name();
        let len = view.len().unwrap_or(0);
        return (0..len).all(|i| match view.get(i) {
            Ok(e) => e.0.is_jvm_null()
                || e.try_checkcast::<T>().is_some()
                // T 自身是数组：元素（内层数组）按 T 的数组视图规则递归判定
                || probe.0.__array_accepts(&e)
                || (t_name != "java/lang/Object" && e.0.is_instance_of(t_name)),
            Err(_) => false,
        });
    }
    false
}

/// checkcast 到 `JArray<T>` 的判定与视图构造（可失败形态，S-4 / A-1 的
/// 唯一决策点）：按序 null 还原 / 同形态取回（`__view_into`：同元素类型、
/// 视图还原、`Object[]` 上转）/ 协变上转（`__array_elem_assignable`：目标
/// 元素类型是源元素类型的祖先）/ 擦除还原（逐元素兼容）。均不满足 → None
/// （ClassCastException）。`From<Object>`（panic 形态，非法转换的进程级
/// 断言）与 `Object::try_cast`（Err 形态，可被 java_try 捕获，S-1）共用，
/// 两条 cast 路径对数组目标的语义完全一致。
pub(crate) fn try_array_view<T: Clone + Default + From<Object> + Into<Object> + 'static>(
    obj: &Object,
) -> Option<JArray<T>> {
    if obj.0.is_jvm_null() {
        return Some(JArray::default());
    }
    if let Some(same) = obj.try_checkcast::<JArray<T>>() {
        return Some(same);
    }
    // 反射创建的引用数组（FS-R6）：运行时数组类由组件标签确定，按 JVM checkcast
    // 精确判定（组件类型可赋值 → 视图；否则 ClassCastException），不走逐元素兼容。
    if let Some(tag) = obj.try_checkcast::<JArray<Object>>().and_then(|a| a.__component_tag()) {
        let target = format!("{}", Into::<Object>::into(T::default()).0.getClass().ok()?.__get_name())
            .replace('.', "/");
        return if crate::java::lang::Class::__name_assignable(&target, &tag) {
            Some(erased_object_view(Clone::clone(obj)))
        } else {
            None
        };
    }
    let mut elem_slot: Option<T> = None;
    let target_elem = Into::<Object>::into(T::default()).0.__class_name();
    if obj.0.__array_elem_assignable(target_elem, &mut elem_slot) {
        return Some(erased_object_view(Clone::clone(obj)));
    }
    if !JArray::<T>::has_primitive_elements() && erased_array_compatible::<T>(obj) {
        return Some(erased_object_view(Clone::clone(obj)));
    }
    None
}

/// `(T[]) obj` —— checkcast 到数组类型（见 `__view_into` / `__array_elem_assignable`）。
///
/// null 通过任意数组类型的 checkcast（以目标形态的 null 还原）；同元素类型 / 已有视图
/// 还原 / `Object[]` 上转走 `__view_into` 既有形态；其余引用元素数组先按「目标元素
/// 类型是源元素类型（运行时元素类型）自身或其祖先」判定（协变上转）。上转不满足时
/// 走泛型数组的擦除还原（对标 wrapper `From<Object>` 的 `is_instance_of` + 擦除重建
/// 路径）：javac 对 `[TK;`（K 为类型变量）插入的 checkcast 目标是 K 的擦除（界类型），
/// 运行时不做元素级检查——源静态元素类型是 T 的祖先形态时（如 `JArray<Enum<Object>>`
/// 还原为 `JArray<K>`，K extends Enum<K>），按运行时元素判定（全部与 T 的 binary name
/// 赋值兼容，或空数组 / 全 null）。两条路径都以源数组的 Object 级协变视图（存储擦除）
/// 重建 `JArray<T>`——读出按 T 重建视图、写入按源元素类型做存储检查。判定失败抛
/// ClassCastException（JVMS §6.5 checkcast）。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> From<Object> for JArray<T> {
    fn from(obj: Object) -> Self {
        try_array_view::<T>(&obj).unwrap_or_else(|| {
            panic!("ClassCastException: {} cannot be cast to {}",
                   obj.0.__class_name(), std::any::type_name::<Self>())
        })
    }
}

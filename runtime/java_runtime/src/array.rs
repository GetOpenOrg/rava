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
pub struct JArray<T>(Option<Rc<Repr<T>>>);

enum Repr<T> {
    /// 自有存储。第二域为反射创建数组的组件类型标签（FS-R6）：
    /// `Array.newInstance(String.class, n)` 在原生侧只有擦除载体 `JArray<Object>`，
    /// 标签记录运行时组件类型的 binary name（`java/lang/String`、`[I`），
    /// 使 getClass / instanceof / checkcast / aastore 检查按 JVM 的真实数组类判定。
    /// 静态类型数组（newarray / anewarray 翻译）元素类型即载体类型，恒为 None。
    Own(Store<T>, Option<Rc<str>>),
    Covariant(CovariantView),
}

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

impl<T> Clone for JArray<T> {
    fn clone(&self) -> Self { JArray(self.0.clone()) }
}

impl<T> std::fmt::Debug for JArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Some(r) => write!(f, "JArray@{:p}", Rc::as_ptr(r)),
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
    fn identity(&self) -> *const () {
        match self.0.as_deref() {
            Some(Repr::Own(..)) => self.0.as_ref().map_or(std::ptr::null(), |r| Rc::as_ptr(r) as *const ()),
            Some(Repr::Covariant(view)) => view.origin.0.__identity(),
            None => std::ptr::null(),
        }
    }

    /// 本引用是否为 Java null（ifnull/ifnonnull 的接收者）。
    #[inline]
    pub fn is_jvm_null(&self) -> bool {
        self.0.is_none()
    }

    /// 非 null 数组的存储形态；null 抛 NullPointerException（JVMS §6.5 *aload / *astore / arraylength）
    #[inline]
    fn repr(&self) -> crate::error::Result<&Repr<T>> {
        self.0.as_deref().ok_or_else(crate::error::JvmError::null_pointer)
    }

    fn own(repr: Repr<T>) -> Self {
        JArray(Some(Rc::new(repr)))
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
        JArray::from(vec![T::default(); len.max(0) as usize])
    }

    /// `newarray`/`anewarray` 的可失败创建：负长度抛 `NegativeArraySizeException`
    /// （真实异常对象，与数组 get/set 的 Result 机制一致）。
    pub fn try_new(len: i32) -> crate::error::Result<Self> {
        if len < 0 {
            return Err(crate::error::JvmError::negative_array_size(len));
        }
        Ok(JArray::from(vec![T::default(); len as usize]))
    }

    /// 创建长度为 len 的数组，每个元素由 init 独立构造（对应 Java multianewarray：
    /// 每一行是独立的数组对象，不能共享同一个默认值的引用）。负长度饱和为空数组
    /// （同 [`Self::new`] 的崩溃防线语义）。
    pub fn new_with(len: i32, init: impl Fn() -> T) -> Self {
        JArray::from((0..len.max(0)).map(|_| init()).collect::<Vec<T>>())
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
    #[inline]
    pub fn get(&self, i: i32) -> crate::error::Result<T> {
        crate::gil::safepoint(); // 安全点钩子（并行后端为空）
        match self.repr()? {
            Repr::Own(store, _) => store.get(i),
            Repr::Covariant(view) => Ok(T::from((view.get)(&view.origin, i)?)),
        }
    }

    /// 元素的原子读-改-写（Unsafe / VarHandle 数组元素 CAS 族）：基本元素为元素原子单元上的
    /// CAS 循环（重试时 `f` 重新求值），引用元素在元素单元锁内读出 `cur`、`f(cur)` 给 `Some(new)`
    /// 时写入；返回 `cur`。协变视图经 `update` 闭包委托源数组的 `__update`。
    pub fn __update(&self, i: i32, f: &mut dyn FnMut(T) -> Option<T>) -> crate::error::Result<T> {
        match self.repr()? {
            Repr::Own(store, _) => store.update(i, f),
            Repr::Covariant(view) => {
                let old = (view.update)(&view.origin, i, &mut |cur: Object| f(T::from(cur)).map(Into::into))?;
                Ok(T::from(old))
            }
        }
    }

    /// 写入下标 i 的元素（对应 Java iastore/aastore 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *astore）；
    /// null 引用抛 `NullPointerException`。
    #[inline]
    pub fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        match self.repr()? {
            Repr::Own(store, tag) => {
                if let Some(tag) = tag {
                    // 反射创建数组的 aastore 存储检查（JVMS §6.5 aastore）
                    let o: Object = Clone::clone(&v).into();
                    if !o.0.is_jvm_null() && !o.0.is_instance_of(tag) {
                        return Err(crate::error::JvmError::array_store(o.0.__class_name()));
                    }
                }
                store.set(i, v)
            }
            Repr::Covariant(view) => (view.set)(&view.origin, i, v.into()),
        }
    }

    /// 元素快照（手写 VM 层批量读取用，逐元素读）
    pub fn to_vec(&self) -> Vec<T> {
        match self.0.as_deref() {
            Some(Repr::Own(store, _)) => store.to_vec(),
            Some(Repr::Covariant(view)) => (0..view.len())
                .map(|i| T::from((view.get)(&view.origin, i).expect("index within length")))
                .collect(),
            None => panic!("NullPointerException: 对 null 数组做批量读取"),
        }
    }

    /// 数组长度（对应 Java arraylength 字节码）。null 引用抛 NullPointerException
    /// （JVMS §6.5 arraylength：objectref 为 null 时抛 NPE）。
    #[inline]
    pub fn len(&self) -> crate::error::Result<i32> {
        match self.repr()? {
            Repr::Own(store, _) => Ok(store.len() as i32),
            Repr::Covariant(view) => Ok(view.len()),
        }
    }

    /// 辅助谓词（非字节码语义）：null 视为无元素。
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }
}

impl<T: 'static> JArray<T> {
    /// 基本元素数组的本机字节视图读（`native_memory` 的堆寻址；逐元素原子读）。
    /// 非基本元素数组 / 视图 / null / 越界返回 false。
    pub(crate) fn __read_bytes(&self, start: usize, dst: &mut [u8]) -> bool {
        matches!(self.0.as_deref(), Some(Repr::Own(store, _)) if store.read_bytes(start, dst))
    }

    /// 基本元素数组的本机字节视图写（部分覆盖的元素按位形 CAS 合并）。失败条件同 `__read_bytes`。
    pub(crate) fn __write_bytes(&self, start: usize, src: &[u8]) -> bool {
        matches!(self.0.as_deref(), Some(Repr::Own(store, _)) if store.write_bytes(start, src))
    }

    /// 基本元素数组字节视图上 `width` 字节值的原子读-改-写（见 `Store::update_bytes`）。
    pub(crate) fn __update_bytes(&self, start: usize, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Option<u64> {
        match self.0.as_deref() {
            Some(Repr::Own(store, _)) => store.update_bytes(start, width, op),
            _ => None,
        }
    }
}

impl<T: 'static> From<Vec<T>> for JArray<T> {
    /// 从 Vec<T> 构造，用于字面量数组初始化（对应 Java 数组初始化器）
    fn from(v: Vec<T>) -> Self {
        JArray::own(Repr::Own(Store::from_vec(v), None))
    }
}

impl JArray<crate::java::lang::String> {
    /// 字符串字面量数组初始化器（`new String[]{"a", "b"}`）的紧凑形态：元素表为静态
    /// 切片，逐元素建 Java String。与 `JArray::from(vec![String::from(..), ..])` 等价，
    /// 但数千元素的资源束字面量表（CLDR getContents）不再展开为数千个表达式节点。
    pub fn from_strs(v: &[&str]) -> Self {
        JArray::from(v.iter().map(|s| crate::java::lang::String::from(*s)).collect::<Vec<_>>())
    }
}

impl JArray<Object> {
    /// 反射创建的引用数组（`Array.newInstance(componentType, len)`，FS-R6）：擦除载体
    /// + 组件类型标签（binary name，斜线形态：`java/lang/String`、`[I`）。元素初值 null。
    pub fn __new_component_tagged(len: i32, component: &str) -> Self {
        let tag: Option<Rc<str>> = if component == "java/lang/Object" { None } else { Some(Rc::from(component)) };
        JArray::own(Repr::Own(Store::from_vec(vec![Object::default(); len.max(0) as usize]), tag))
    }

    /// 组件类型标签（仅反射创建的引用数组；静态类型数组与视图为 None）。
    pub(crate) fn __component_tag(&self) -> Option<std::string::String> {
        match self.0.as_deref() {
            Some(Repr::Own(_, Some(tag))) => Some(tag.to_string()),
            _ => None,
        }
    }

    /// `new Object[]{"k", "v"}`（元素全为字符串字面量）的紧凑形态，语义同上。
    pub fn objects_from_strs(v: &[&str]) -> Self {
        JArray::from(v.iter()
            .map(|s| Object::from(crate::java::lang::String::from(*s)))
            .collect::<Vec<_>>())
    }
}

/// Java 数组是对象：可直接装入 Object（`Object o = arr;`）。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> From<JArray<T>> for Object {
    fn from(a: JArray<T>) -> Object { Object::__alloc(a) }
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

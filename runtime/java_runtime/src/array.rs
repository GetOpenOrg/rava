// Java 数组类型封装：可读层包装
// get/set/len 隐藏 borrow_mut()，让生成代码保持 Java 语义可读性
// 调用方只需 array.get(i)、array.set(i, v)、array.len()，无需接触 RefCell API

use std::cell::RefCell;
use std::rc::Rc;

use crate::java::lang::Object;

/// Java 数组。Clone 共享底层存储（Java 数组是引用类型，赋值不复制内容）。
///
/// 数组协变（JLS §4.10.3：`Number[] na = new Integer[n]`）：引用元素数组以任意祖先
/// 元素静态类型流转时是 `Covariant` 视图——存储保持类型擦除（读写经源数组完成，
/// 元素类型转换发生在 `JArray<T>` 泛型边界：读出按目标元素类型重建视图，写入按源
/// 元素类型做存储检查，对应 aastore，失败抛 `ArrayStoreException`），对象标识与源
/// 数组相同；还原为源类型（`(Integer[]) na`）取回源数组本身。
///
/// Null（S-3.1）：数组引用可为 null（未初始化的静态字段、`long[] a = null`）。
/// `Repr::Null` 与 `JArray::new(0)`（真实存在的空数组）严格区分：null 上的
/// get/set/len 抛 NullPointerException（JVMS §6.5 arraylength/*aload/*astore），
/// `is_jvm_null()` 为 true；null 装入 Object 后以 vtable 的 is_jvm_null 呈现 null 语义
/// （null 通过任意 checkcast、与任何非 null 引用不等）。
pub struct JArray<T>(Rc<Repr<T>>);

enum Repr<T> {
    Own(RefCell<Vec<T>>),
    Covariant(CovariantView),
    Null,
}

/// 以任意引用元素静态类型观察某个引用元素数组（存储擦除：边界是 Object）。
#[derive(Clone)]
struct CovariantView {
    origin: Object,
    len: Rc<dyn Fn() -> i32>,
    get: Rc<dyn Fn(i32) -> crate::error::Result<Object>>,
    set: Rc<dyn Fn(i32, Object) -> crate::error::Result<()>>,
}

/// aastore 存储检查（JLS §10.5 / JVMS §6.5 aastore）：值与源元素类型赋值兼容才能写入，
/// 否则抛 `ArrayStoreException`。判定按序三条：
///   1. null 可存入任意引用元素数组；
///   2. 值的 wrapper 静态祖先名单（`__view_into` 填 `Option<T>` slot）——覆盖以子类
///      wrapper 或数组视图形态流转的值（多维数组的元素也是数组，视图经 Covariant
///      委托到源数组判定）；
///   3. 按运行时类名 `is_instance_of`——覆盖以祖先 wrapper 视图流转的子类值
///      （如经 `Number` 视图流转的 `Integer`：wrapper 的 vtable 仍持运行时类）。
fn aastore_storable<T: Clone + From<Object> + 'static>(v: &Object, elem_name: &str) -> bool {
    if v.0.is_jvm_null() {
        return true;
    }
    let mut slot: Option<T> = None;
    let unused: Rc<dyn std::any::Any> = Rc::new(());
    if v.0.__view_into(unused, &mut slot) && slot.is_some() {
        return true;
    }
    v.0.is_instance_of(elem_name)
}

impl<T> Clone for JArray<T> {
    fn clone(&self) -> Self { JArray(Rc::clone(&self.0)) }
}

impl<T> std::fmt::Debug for JArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JArray@{:p}", Rc::as_ptr(&self.0))
    }
}

/// Java 数组引用的默认值是 null（局部变量 / 未初始化字段 / 静态字段单元格的
/// `.unwrap_or_default()`），与 `new T[0]`（真实空数组）不同。
impl<T: 'static> Default for JArray<T> {
    fn default() -> Self { JArray(Rc::new(Repr::Null)) }
}

impl<T: 'static> PartialEq for JArray<T> {
    fn eq(&self, other: &Self) -> bool {
        match (&*self.0, &*other.0) {
            // null == null（Java 引用比较）；null 与任何真实数组不等
            (Repr::Null, Repr::Null) => true,
            (Repr::Null, _) | (_, Repr::Null) => false,
            _ => self.identity() == other.identity(),
        }
    }
}

impl<T: 'static> JArray<T> {
    fn identity(&self) -> *const () {
        match &*self.0 {
            Repr::Own(_) => Rc::as_ptr(&self.0) as *const (),
            Repr::Covariant(view) => view.origin.0.__identity(),
            Repr::Null => std::ptr::null(),
        }
    }

    /// 本引用是否为 Java null（ifnull/ifnonnull 的接收者）。
    pub fn is_jvm_null(&self) -> bool {
        matches!(&*self.0, Repr::Null)
    }

    fn has_primitive_elements() -> bool {
        let element = std::any::TypeId::of::<T>();
        [
            std::any::TypeId::of::<i8>(), std::any::TypeId::of::<i16>(), std::any::TypeId::of::<u16>(),
            std::any::TypeId::of::<i32>(), std::any::TypeId::of::<i64>(), std::any::TypeId::of::<f32>(),
            std::any::TypeId::of::<f64>(), std::any::TypeId::of::<bool>(),
        ].contains(&element)
    }
}

impl<T: Clone + Default + 'static> JArray<T> {
    /// 创建长度为 len 的数组，元素初始化为类型默认值（对应 Java newarray/anewarray）
    pub fn new(len: i32) -> Self {
        JArray::from(vec![T::default(); len as usize])
    }

    /// 创建长度为 len 的数组，每个元素由 init 独立构造（对应 Java multianewarray：
    /// 每一行是独立的数组对象，不能共享同一个默认值的引用）
    pub fn new_with(len: i32, init: impl Fn() -> T) -> Self {
        JArray::from((0..len.max(0)).map(|_| init()).collect::<Vec<T>>())
    }
}

impl<T: Clone + Default + From<Object> + Into<Object> + 'static> JArray<T> {
    /// 读取下标 i 的元素（对应 Java iaload/aaload 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *aload）；
    /// null 引用抛 `NullPointerException`。
    pub fn get(&self, i: i32) -> crate::error::Result<T> {
        match &*self.0 {
            Repr::Own(cells) => {
                let data = cells.borrow();
                if i < 0 || i as usize >= data.len() {
                    return Err(crate::error::JvmError::array_index_out_of_bounds(i, data.len() as i32));
                }
                Ok(data[i as usize].clone())
            }
            Repr::Covariant(view) => Ok(T::from((view.get)(i)?)),
            Repr::Null => Err(crate::error::JvmError::null_pointer()),
        }
    }

    /// 写入下标 i 的元素（对应 Java iastore/aastore 等）。
    /// 越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5 *astore）；
    /// null 引用抛 `NullPointerException`。
    pub fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        match &*self.0 {
            Repr::Own(cells) => {
                let mut data = cells.borrow_mut();
                if i < 0 || i as usize >= data.len() {
                    return Err(crate::error::JvmError::array_index_out_of_bounds(i, data.len() as i32));
                }
                data[i as usize] = v;
                Ok(())
            }
            Repr::Covariant(view) => (view.set)(i, v.into()),
            Repr::Null => Err(crate::error::JvmError::null_pointer()),
        }
    }

    /// 元素快照（手写 VM 层批量读取用，不经过逐元素边界检查）
    pub fn to_vec(&self) -> Vec<T> {
        match &*self.0 {
            Repr::Own(cells) => cells.borrow().clone(),
            Repr::Covariant(view) => (0..(view.len)())
                .map(|i| T::from((view.get)(i).expect("index within length")))
                .collect(),
            Repr::Null => panic!("NullPointerException: 对 null 数组做批量读取"),
        }
    }

    /// 数组长度（对应 Java arraylength 字节码）。null 引用抛 NullPointerException
    /// （JVMS §6.5 arraylength：objectref 为 null 时抛 NPE）。
    pub fn len(&self) -> crate::error::Result<i32> {
        match &*self.0 {
            Repr::Own(cells) => Ok(cells.borrow().len() as i32),
            Repr::Covariant(view) => Ok((view.len)()),
            Repr::Null => Err(crate::error::JvmError::null_pointer()),
        }
    }

    /// 辅助谓词（非字节码语义）：null 视为无元素。
    pub fn is_empty(&self) -> bool {
        self.len().unwrap_or(0) == 0
    }

    /// 本数组的 Object 元素视图（协变上转）。已是视图的直接沿用其源。
    /// null 引用不会到达（__view_into 对 null 只回填 null，不构造视图）。
    fn covariant_view(&self) -> CovariantView {
        match &*self.0 {
            Repr::Covariant(view) => Clone::clone(view),
            Repr::Own(_) => {
                let (for_len, for_get, for_set) = (Clone::clone(self), Clone::clone(self), Clone::clone(self));
                // 存储检查的元素类型名：源元素类型的 null 探针经 vtable 取 binary name
                //（名单与元素值无关，null 探针等价于任意元素）
                let elem_name = Into::<Object>::into(T::default()).0.__class_name();
                CovariantView {
                    origin: Object::from(Clone::clone(self)),
                    len: Rc::new(move || for_len.len().expect("covariant view of non-null array")),
                    get: Rc::new(move |i| Ok(for_get.get(i)?.into())),
                    set: Rc::new(move |i, v| {
                        if !aastore_storable::<T>(&v, elem_name) {
                            return Err(crate::error::JvmError::array_store(v.0.__class_name()));
                        }
                        for_set.set(i, T::from(v))
                    }),
                }
            }
            Repr::Null => panic!("NullPointerException: 构造 null 数组的协变视图"),
        }
    }
}

impl<T> From<Vec<T>> for JArray<T> {
    /// 从 Vec<T> 构造，用于字面量数组初始化（对应 Java 数组初始化器）
    fn from(v: Vec<T>) -> Self {
        JArray(Rc::new(Repr::Own(RefCell::new(v))))
    }
}

/// Java 数组是对象：可直接装入 Object（`Object o = arr;`）。
/// null 数组装入后经 vtable 的 is_jvm_null 呈现 Java null 语义。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static> crate::java::lang::ObjectVTable for JArray<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __identity(&self) -> *const () { self.identity() }
    fn is_jvm_null(&self) -> bool { JArray::is_jvm_null(self) }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { Some(self.len()) }

    /// checkcast 到数组类型：同元素类型 → 自身；视图还原 → 交给源数组判定；
    /// 引用类型数组 → `Object[]`：协变视图。其余目标元素类型由
    /// `From<Object> for JArray<T>`（知道目标元素类型）经 `__array_elem_assignable`
    /// 判定。null 通过任意引用类型的 checkcast（JVMS §6.5 checkcast）。
    fn __view_into(&self, any: Rc<dyn std::any::Any>, slot: &mut dyn std::any::Any) -> bool {
        if let Some(same) = slot.downcast_mut::<Option<Self>>() {
            *same = Some(Clone::clone(self));
            return true;
        }
        if matches!(&*self.0, Repr::Null) {
            // null 通过任意数组类型的 checkcast：以目标形态的 null 还原
            if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
                *erased = Some(JArray::default());
                return true;
            }
            return false;
        }
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.__view_into(any, slot);
        }
        if let Some(erased) = slot.downcast_mut::<Option<JArray<Object>>>() {
            if !Self::has_primitive_elements() {
                *erased = Some(JArray(Rc::new(Repr::Covariant(self.covariant_view()))));
                return true;
            }
        }
        false
    }

    /// 数组协变的元素赋值兼容探针（S-4）：调用方（`From<Object> for JArray<T>`，
    /// 知道目标元素类型）构造 `Option<T>` slot；本钩子以「源元素类型的 null 探针」
    /// view_into 该 slot——wrapper 的祖先名单是静态生成的（与元素值无关），填充成功
    /// ⇔ 目标元素类型是源元素类型自身或其祖先（JLS §4.10.3 数组子类型条件）。
    /// 已是视图的数组按其源数组（运行时元素类型）判定；基本元素数组无协变视图。
    fn __array_elem_assignable(&self, slot: &mut dyn std::any::Any) -> bool {
        if Self::has_primitive_elements() {
            return false;
        }
        match &*self.0 {
            Repr::Covariant(view) => view.origin.0.__array_elem_assignable(slot),
            Repr::Own(_) => {
                let probe: Object = Into::<Object>::into(T::default());
                let unused: Rc<dyn std::any::Any> = Rc::new(());
                probe.0.__view_into(unused, slot)
            }
            Repr::Null => true,
        }
    }
}

/// 擦除数组的逐元素兼容判定（`From<Object> for JArray<T>` 与 `Object::try_cast`
/// 共用，A-3）：源数组可按 Object 级协变视图观察、且每个非 null 元素的运行时类与
/// T 的 binary name 赋值兼容 → checkcast 到 `JArray<T>` 成立（泛型数组的擦除还原
/// 路径：`(String[]) objArr`，源静态元素类型是 T 的祖先形态）。空数组 / 全 null 恒兼容。
pub(crate) fn erased_array_compatible<T: Clone + Default + Into<Object> + 'static>(obj: &Object) -> bool {
    let unused: Rc<dyn std::any::Any> = Rc::new(());
    let mut erased: Option<JArray<Object>> = None;
    obj.0.__view_into(unused, &mut erased);
    if let Some(view) = erased {
        let t_name = Into::<Object>::into(T::default()).0.__class_name();
        let len = view.len().unwrap_or(0);
        return (0..len).all(|i| match view.get(i) {
            Ok(e) => e.0.is_jvm_null() || e.0.is_instance_of(t_name),
            Err(_) => false,
        });
    }
    false
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
impl<T: Clone + Default + From<Object> + Into<Object> + 'static> From<Object> for JArray<T> {
    fn from(obj: Object) -> Self {
        if obj.0.is_jvm_null() {
            return Self::default();
        }
        if let Some(same) = obj.try_checkcast::<Self>() {
            return same;
        }
        let mut elem_slot: Option<T> = None;
        if obj.0.__array_elem_assignable(&mut elem_slot) && elem_slot.is_some() {
            return erased_object_view(obj);
        }
        if !Self::has_primitive_elements() && erased_array_compatible::<T>(&obj) {
            return erased_object_view(obj);
        }
        panic!("ClassCastException: {} cannot be cast to {}",
               obj.0.__class_name(), std::any::type_name::<Self>())
    }
}

/// 源数组的 Object 级协变视图：元素访问经 Object 的数组 API（array_length /
/// array_load_object / array_store_object）转发到源数组——读出在 `JArray<T>::get`
/// 边界按 T 重建（wrapper 经擦除路径，保持运行时类），写入在源数组的协变视图闭包
/// 做存储检查（ArrayStoreException）。对象标识与源数组相同。
fn erased_object_view<T: 'static>(origin: Object) -> JArray<T> {
    let (for_len, for_get, for_set) =
        (Clone::clone(&origin), Clone::clone(&origin), Clone::clone(&origin));
    JArray(Rc::new(Repr::Covariant(CovariantView {
        len: Rc::new(move || {
            for_len.array_length().expect("covariant view of non-null array")
        }),
        get: Rc::new(move |i| for_get.array_load_object(i)),
        set: Rc::new(move |i, v| for_set.array_store_object(i, v)),
        origin,
    })))
}

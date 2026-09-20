// Java 数组类型封装：可读层包装
// get/set/len 隐藏 borrow_mut()，让生成代码保持 Java 语义可读性
// 调用方只需 array.get(i)、array.set(i, v)、array.len()，无需接触 RefCell API

use std::cell::RefCell;
use std::rc::Rc;

use crate::java::lang::Object;

/// Java 数组。Clone 共享底层存储（Java 数组是引用类型，赋值不复制内容）。
///
/// 数组协变（`Object[] a = new String[n]`）：同一数组对象以 Object 元素静态类型流转时是
/// `Covariant` 视图——读写经源数组完成（写入按源元素类型 checkcast，对应 aastore 的存储检查），
/// 对象标识与源数组相同；还原为源类型（`(String[]) a`）取回源数组本身。
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

/// 以 Object 为元素静态类型观察某个引用类型数组。
#[derive(Clone)]
struct CovariantView {
    origin: Object,
    len: Rc<dyn Fn() -> i32>,
    get: Rc<dyn Fn(i32) -> crate::error::Result<Object>>,
    set: Rc<dyn Fn(i32, Object) -> crate::error::Result<()>>,
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

impl<T: Clone + From<Object> + Into<Object> + 'static> JArray<T> {
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
                CovariantView {
                    origin: Object::from(Clone::clone(self)),
                    len: Rc::new(move || for_len.len().expect("covariant view of non-null array")),
                    get: Rc::new(move |i| Ok(for_get.get(i)?.into())),
                    set: Rc::new(move |i, v| for_set.set(i, T::from(v))),
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
impl<T: Clone + From<Object> + Into<Object> + 'static> crate::java::lang::ObjectVTable for JArray<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __identity(&self) -> *const () { self.identity() }
    fn is_jvm_null(&self) -> bool { JArray::is_jvm_null(self) }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { Some(self.len()) }

    /// checkcast 到数组类型：同元素类型 → 自身；视图还原 → 交给源数组判定；
    /// 引用类型数组 → `Object[]`：协变视图。其余为 ClassCastException（返回 false）。
    /// null 通过任意引用类型的 checkcast（JVMS §6.5 checkcast）。
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
}

/// `(T[]) obj` —— checkcast 到数组类型（见 `__view_into`）。
impl<T: Clone + Default + 'static> From<Object> for JArray<T> {
    fn from(obj: Object) -> Self { obj.downcast::<Self>() }
}

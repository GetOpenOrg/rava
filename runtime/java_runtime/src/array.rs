// Java 数组类型封装：可读层包装
// get/set/len 隐藏 borrow_mut()，让生成代码保持 Java 语义可读性
// 调用方只需 array.get(i)、array.set(i, v)、array.len()，无需接触 RefCell API

use crate::sync_model::__RefSlot as RefCell;
use crate::sync_model::__Shared as Rc;

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
    /// 手写 native 的就地整段访问（Own 形态直取底层 Vec）。回调内多元素读写
    /// 免逐元素 Result；返回回调返回值。null 接收者抛 NPE；协变视图（元素类型
    /// 擦除）无原生切片可取——调用方（DecimalDigits 等接收 new byte[] 直造数组
    /// 的 native）当前不触达，触达时再经逐 get/set 适配。
    pub fn with_vec<R>(
        &self, f: impl FnOnce(&mut [T]) -> R,
    ) -> crate::error::Result<R> {
        match &*self.0 {
            Repr::Own(cells) => {
                let mut data = cells.borrow_mut();
                Ok(f(&mut data))
            }
            Repr::Covariant(_) => {
                let e = crate::java::lang::UnsupportedOperationException::new_str(
                    crate::java::lang::String::from("JArray::with_vec on covariant view"));
                Err(crate::error::JvmError::from(e?))
            }
            Repr::Null => Err(crate::error::JvmError::null_pointer()),
        }
    }

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

impl JArray<crate::java::lang::String> {
    /// 字符串字面量数组初始化器（`new String[]{"a", "b"}`）的紧凑形态：元素表为静态
    /// 切片，逐元素建 Java String。与 `JArray::from(vec![String::from(..), ..])` 等价，
    /// 但数千元素的资源束字面量表（CLDR getContents）不再展开为数千个表达式节点。
    pub fn from_strs(v: &[&str]) -> Self {
        JArray::from(v.iter().map(|s| crate::java::lang::String::from(*s)).collect::<Vec<_>>())
    }
}

impl JArray<Object> {
    /// `new Object[]{"k", "v"}`（元素全为字符串字面量）的紧凑形态，语义同上。
    pub fn objects_from_strs(v: &[&str]) -> Self {
        JArray::from(v.iter()
            .map(|s| Object::from(crate::java::lang::String::from(*s)))
            .collect::<Vec<_>>())
    }
}

/// Java 数组是对象：可直接装入 Object（`Object o = arr;`）。
/// null 数组装入后经 vtable 的 is_jvm_null 呈现 Java null 语义。
impl<T: Clone + Default + From<Object> + Into<Object> + 'static> crate::java::lang::ObjectVTable for JArray<T> {
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn __identity(&self) -> *const () { self.identity() }
    fn is_jvm_null(&self) -> bool { JArray::is_jvm_null(self) }
    fn __array_len(&self) -> Option<crate::error::Result<i32>> { Some(self.len()) }

    /// 数组类的 Class 对象（JLS §10.8：`new String[0].getClass()` 是
    /// `[Ljava.lang.String;`）。binary name 为 JVM 描述符形态、斜线键——与
    /// ldc 的 `X[].class`（`Class::for_class("[Ljava/lang/String;")`）落同一
    /// 缓存条目，`a.getClass() == X[].class` 的身份语义由此成立。协变视图
    /// 委托源数组（数组类由创建时的元素类型决定，与观察形态无关）；基本元素
    /// 静态取描述符字符；引用元素经元素 vtable 的 getClass 递归取得（嵌套
    /// 数组因此正确：`JArray<JArray<T>>` → `[[T`）。
    fn getClass(&self) -> crate::error::Result<crate::java::lang::Class> {
        // 协变视图：`String[]` 以 `Object[]` 形态流转时 getClass 仍是
        // `[Ljava.lang.String;`——委托源数组取运行时元素类型。
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.getClass();
        }
        if let Some(d) = Self::primitive_elem_descriptor() {
            return Ok(crate::java::lang::Class::for_class(
                crate::java::lang::String::from(format!("[{}", d).as_str())));
        }
        let elem_name = format!("{}", Into::<Object>::into(T::default())
            .0.getClass()?
            .__get_name())
            .replace('.', "/");
        let binary = if elem_name.starts_with('[') {
            format!("[{}", elem_name)
        } else {
            format!("[L{};", elem_name)
        };
        Ok(crate::java::lang::Class::for_class(
            crate::java::lang::String::from(binary.as_str())))
    }

    /// JLS §10.8 / §4.10.4：数组的直接超类型是 Object、Cloneable、Serializable。
    /// 有意不按 "java/lang/Object" 匹配——aastore_storable 的元素类型名探针对
    /// 多维数组退化为 "java/lang/Object"（数组的类名探针无元素信息），按名放行
    /// 会使异构数组元素的存储检查失效（元素是数组的情形走 `__view_into` 臂）。
    fn is_instance_of(&self, type_id: &str) -> bool {
        if matches!(type_id, "java/lang/Cloneable" | "java/io/Serializable") {
            return true;
        }
        if !type_id.starts_with('[') {
            return false;
        }
        // instanceof 数组目标（instanceof byte[] 等，描述符形态）：
        //   - null 数组通过任意 instanceof（JVMS aconst_null 语义）；
        //   - 协变视图按源数组（运行时元素类型）判定；
        //   - 同描述符成立；目标 "[Ljava/lang/Object;" 按数组协变恒成立
        //（JLS §4.10.4：任意引用元素数组是 Object[] 的子类型——基本元素
        //     数组不是，故只在引用元素分支放行）。
        if matches!(&*self.0, Repr::Null) {
            return true;
        }
        if let Repr::Covariant(view) = &*self.0 {
            return view.origin.0.is_instance_of(type_id);
        }
        if let Some(d) = Self::primitive_elem_descriptor() {
            return type_id == format!("[{}", d);
        }
        let target = type_id.replace('.', "/");
        if target == "[Ljava/lang/Object;" {
            return true;
        }
        // 引用元素 / 嵌套数组：与 getClass 的描述符形态比对
        match self.getClass() {
            Ok(c) => format!("{}", c.__get_name()).replace('.', "/") == target,
            Err(_) => false,
        }
    }

    /// `Object.clone()`（invokevirtual）在数组上的语义（JLS §10.7）：浅拷贝——
    /// 新数组对象、逐元素共享引用（元素自身的 Clone 即引用共享）。Own 形态按
    /// 当前元素类型重建；协变视图委托源数组（克隆保持运行时元素类型，Java 的
    /// clone 不改变数组的具体类型）。
    fn __shallow_copy(&self) -> Option<Object> {
        match &*self.0 {
            Repr::Own(cells) => {
                let data = cells.borrow();
                Some(Object::from(JArray::from(data.clone())))
            }
            Repr::Covariant(view) => view.origin.0.__shallow_copy(),
            Repr::Null => Some(Object::from(Clone::clone(self))),
        }
    }

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

/// 擦除数组的逐元素兼容判定（`From<Object> for JArray<T>` 的擦除还原臂，
/// A-3）：源数组可按 Object 级协变视图观察、且每个非 null 元素与目标元素
/// 类型 T 赋值兼容 → checkcast 到 `JArray<T>` 成立（泛型数组的擦除还原
/// 路径：`(String[]) objArr`，源静态元素类型是 T 的祖先形态）。空数组 /
/// 全 null 恒兼容。
///
/// 元素兼容按序两臂：`try_checkcast::<T>`（同类型 / 祖先 wrapper 视图 /
/// 数组元素——`__view_into` 驱动，元素本身是数组（`JArray` 不实现
/// `is_instance_of`）与数组协变视图只有此臂可判）；T 的 null 探针类名
/// 非退化（`java/lang/Object`——T 自身是数组或接口别名时探针失义）时补
/// `is_instance_of`（覆盖未经 javac 装箱、以原生值盒（`Rc<i32>` 带 Integer
/// vtable）流入 Object 槽位的元素）。
pub(crate) fn erased_array_compatible<T: Clone + Default + Into<Object> + 'static>(obj: &Object) -> bool {
    let unused: Rc<dyn std::any::Any> = Rc::new(());
    let mut erased: Option<JArray<Object>> = None;
    obj.0.__view_into(unused, &mut erased);
    if let Some(view) = erased {
        let t_name = Into::<Object>::into(T::default()).0.__class_name();
        let len = view.len().unwrap_or(0);
        return (0..len).all(|i| match view.get(i) {
            Ok(e) => e.0.is_jvm_null()
                || e.try_checkcast::<T>().is_some()
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
    let mut elem_slot: Option<T> = None;
    if obj.0.__array_elem_assignable(&mut elem_slot) && elem_slot.is_some() {
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
impl<T: Clone + Default + From<Object> + Into<Object> + 'static> From<Object> for JArray<T> {
    fn from(obj: Object) -> Self {
        try_array_view::<T>(&obj).unwrap_or_else(|| {
            panic!("ClassCastException: {} cannot be cast to {}",
                   obj.0.__class_name(), std::any::type_name::<Self>())
        })
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

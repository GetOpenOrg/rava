//! JArray 的协变视图：引用元素数组以祖先元素静态类型流转时的擦除视图（宿主 array.rs 的私有辅助模块）

use super::*;

// ── 数组协变视图（JLS §4.10.3）：引用元素数组以祖先元素静态类型流转时的擦除视图 ──

/// 以任意引用元素静态类型观察某个引用元素数组（存储擦除：边界是 Object）。
///
/// 元素访问按源数组形态单态化为函数指针（不捕获状态，源数组即 `origin`），构造视图只分配
/// 源数组的 Object 句柄与视图本身：Unsafe / VarHandle 引用访问器、aaload / aastore 的 Object
/// 形态每次经 `__view_into` 取视图，视图构造在这些路径的热循环上。长度经 `origin` 的数组长度钩子。
#[derive(Clone)]
pub(super) struct CovariantView {
    pub(super) origin: Object,
    pub(super) get: fn(&Object, i32) -> crate::error::Result<Object>,
    pub(super) set: fn(&Object, i32, Object) -> crate::error::Result<()>,
    /// 元素原子读-改-写：委托源数组的 `__update`，在源存储写锁内完成「读 → 判定 → 写」
    /// （Unsafe / VarHandle 数组元素 CAS 经擦除协变视图到达：ForkJoinPool.WorkQueue 的
    /// `ForkJoinTask[]`、ConcurrentHashMap 的 `Node[]`）。读后写两步在并行后端会丢更新 /
    /// 重复取任务。
    pub(super) update: fn(&Object, i32, &mut dyn FnMut(Object) -> Option<Object>) -> crate::error::Result<Object>,
}

impl CovariantView {
    pub(super) fn len(&self) -> i32 {
        self.origin.0.__array_len()
            .expect("covariant view origin is an array")
            .expect("covariant view of non-null array")
    }
}

/// 数组 `a` 的 Object 元素视图（协变上转）。已是视图的直接沿用其源。
/// null 引用不会到达（__view_into 对 null 只回填 null，不构造视图）。
pub(super) fn covariant_view<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(
    a: &JArray<T>,
) -> CovariantView {
    match a.0.as_deref() {
        Some(o) => covariant_view_of(o),
        None => panic!("NullPointerException: 构造 null 数组的协变视图"),
    }
}

/// 数组对象 `a` 的 Object 元素视图（见 `covariant_view`）
pub(super) fn covariant_view_of<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(
    a: &__ArrayObj<T>,
) -> CovariantView {
    match &a.repr {
        Repr::Covariant(view) => Clone::clone(view),
        Repr::Own { .. } => CovariantView {
            origin: Object::from(a.handle()),
            get: own_view_get::<T>,
            set: own_view_set::<T>,
            update: own_view_update::<T>,
        },
    }
}

/// 源数组的 Object 级协变视图：元素访问经 Object 的数组 API（array_length /
/// array_load_object / array_store_object）转发到源数组——读出在 `JArray<T>::get`
/// 边界按 T 重建（wrapper 经擦除路径，保持运行时类），写入在源数组的协变视图闭包
/// 做存储检查（ArrayStoreException）。对象标识与源数组相同。
pub(super) fn erased_object_view<T: 'static>(origin: Object) -> JArray<T> {
    JArray::covariant(erased_view(origin))
}

/// 源数组的 Object 级协变视图本体（常量求值可用：映像中形态不一致的数组引用即此视图的映像对象）
pub(super) const fn erased_view(origin: Object) -> CovariantView {
    CovariantView { origin, get: erased_get, set: erased_set, update: erased_update }
}

fn erased_get(o: &Object, i: i32) -> crate::error::Result<Object> {
    o.array_load_object(i)
}

fn erased_set(o: &Object, i: i32, v: Object) -> crate::error::Result<()> {
    o.array_store_object(i, v)
}

/// 原子读-改-写：源数组的 Object 元素视图（`__view_into` → 源 Own 存储的协变视图，
/// 其 update 在源存储写锁内完成）
fn erased_update(o: &Object, i: i32, f: &mut dyn FnMut(Object) -> Option<Object>) -> crate::error::Result<Object> {
    let unused = crate::sync_model::__unused_any();
    let mut slot: Option<JArray<Object>> = None;
    o.0.__view_into(unused, &mut slot);
    match slot {
        Some(view) => view.__update(i, f),
        None => Err(crate::error::JvmError::class_cast(format!("{} 不是引用元素数组", o.0.__class_name()))),
    }
}

// ── 自有存储源数组的协变视图元素访问（`covariant_view` 按元素类型 T 单态化的函数指针）──

/// 视图源：`covariant_view` 以 `Object::from(JArray<T>)` 构造 `origin`，运行时形态恒为 `__ArrayObj<T>`。
fn own_source<T: 'static>(origin: &Object) -> &__ArrayObj<T> {
    origin.0.as_any().downcast_ref::<__ArrayObj<T>>().expect("covariant view origin is __ArrayObj<T>")
}

/// 存储检查的元素类型名：源元素类型的 null 探针经 vtable 取 binary name
///（名单与元素值无关，null 探针等价于任意元素）
fn own_elem_name<T: Default + Into<Object>>() -> &'static str {
    Into::<Object>::into(T::default()).0.__class_name()
}

fn own_view_get<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(
    origin: &Object, i: i32,
) -> crate::error::Result<Object> {
    Ok(own_source::<T>(origin).get(i)?.into())
}

fn own_view_set<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(
    origin: &Object, i: i32, v: Object,
) -> crate::error::Result<()> {
    if !aastore_storable::<T>(&v, own_elem_name::<T>()) {
        return Err(crate::error::JvmError::array_store(v.0.__class_name()));
    }
    own_source::<T>(origin).set(i, T::from(v))
}

fn own_view_update<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe>(
    origin: &Object, i: i32, f: &mut dyn FnMut(Object) -> Option<Object>,
) -> crate::error::Result<Object> {
    // 存储检查在写锁内对候选新值执行（CAS 的新值同样是 aastore）；
    // 检查失败时不写入，错误在锁外抛出
    let mut store_err: Option<Object> = None;
    let old = own_source::<T>(origin).__update(i, &mut |cur: T| {
        let n = f(cur.into())?;
        if !aastore_storable::<T>(&n, own_elem_name::<T>()) {
            store_err = Some(n);
            return None;
        }
        Some(T::from(n))
    })?;
    if let Some(v) = store_err {
        return Err(crate::error::JvmError::array_store(v.0.__class_name()));
    }
    Ok(old.into())
}

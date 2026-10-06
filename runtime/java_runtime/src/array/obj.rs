//! 数组对象：`[对象头 | __ArrayObj<T> | 元素 × len]` 一次分配（宿主 array.rs 的私有辅助模块）
//!
//! `JArray<T>` 持 `__Obj<__ArrayObj<T>>`，装入 Object 只换指针类型（不分配）；元素紧随数组对象
//! 存放（`__Obj::new_trailing`），不再有独立的存储块。null 数组装入 Object 为 `__ArrayNull<T>`
//! 静态哨兵（不分配、不计数），以元素类型 T 应答 getClass / checkcast 等静态类型探针。

use super::*;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::mem::{align_of, size_of};
use std::sync::atomic::Ordering;

use crate::obj_ref::{__trailing, __Obj};
use crate::sync_model::__RefField;

/// 数组对象（元素紧随其后）。
///
/// `_cell`：元素区经本值的引用寻址并被原子写入 / 单元锁改写，类型须含内部可变性（非 `Freeze`），
/// `&__ArrayObj` 才不会被视为只读不别名的引用。
#[doc(hidden)]
pub struct __ArrayObj<T> {
    pub(super) repr: Repr,
    _cell: UnsafeCell<()>,
    _elem: PhantomData<T>,
}

// 元素只经原子指令或单元锁访问（与 `PrimSlot` / `__RefField` 同一条件）
unsafe impl<T: Send> Send for __ArrayObj<T> {}
unsafe impl<T: Send + Sync> Sync for __ArrayObj<T> {}

pub(super) enum Repr {
    /// 自有元素（紧随对象）。`prim`：元素为基本类型槽 `PrimSlot<T>`，否则为引用单元 `__RefField<T>`。
    /// `tag` 为反射创建数组的组件类型标签（FS-R6）：`Array.newInstance(String.class, n)` 在原生侧
    /// 只有擦除载体 `JArray<Object>`，标签记录运行时组件类型的 binary name（`java/lang/String`、
    /// `[I`），使 getClass / instanceof / checkcast / aastore 检查按 JVM 的真实数组类判定。
    /// 静态类型数组（newarray / anewarray 翻译）元素类型即载体类型，恒为 None。
    Own { len: usize, prim: bool, tag: Option<Rc<str>> },
    /// 协变视图（无自有元素）
    Covariant(CovariantView),
}

/// 数组对象的存取形态
pub(super) enum Form<'a, T> {
    Own(Store<'a, T>, Option<&'a Rc<str>>),
    Covariant(&'a CovariantView),
}

impl<T: 'static> __ArrayObj<T> {
    /// 自有元素数组：一次分配，元素逐个由 `elems` 写入（恰好 `len` 个）
    pub(super) fn own(len: usize, tag: Option<Rc<str>>, elems: impl IntoIterator<Item = T>) -> __Obj<Self> {
        const { assert!(align_of::<PrimSlot<T>>() <= align_of::<Self>() && align_of::<__RefField<T>>() <= align_of::<Self>()) };
        let prim = JArray::<T>::has_primitive_elements();
        let width = if prim { size_of::<PrimSlot<T>>() } else { size_of::<__RefField<T>>() };
        let bytes = len.checked_mul(width).expect("数组大小溢出");
        // SAFETY: 值与恰好 len 个元素在初始化闭包内写入；元素对齐 ≤ 值对齐，值大小是其对齐的倍数
        unsafe {
            __Obj::new_trailing(bytes, |p: *mut Self| {
                p.write(__ArrayObj { repr: Repr::Own { len, prim, tag }, _cell: UnsafeCell::new(()), _elem: PhantomData });
                let base = (p as *mut u8).add(size_of::<Self>());
                let mut n = 0;
                for e in elems.into_iter().take(len) {
                    if prim {
                        (base as *mut T).add(n).write(e);
                    } else {
                        (base as *mut __RefField<T>).add(n).write(__RefField::new(e));
                    }
                    n += 1;
                }
                assert_eq!(n, len, "数组元素个数与长度不符");
            })
        }
    }

    /// 元素取默认值的自有元素数组（newarray / anewarray）：基本元素的 Java 默认值（0 / false / +0.0）
    /// 位形全零，分配时清零即完成初始化，不逐元素写入；引用元素逐个写入默认值（类型化 null）。
    pub(super) fn own_default(len: usize) -> __Obj<Self>
    where
        T: Default,
    {
        if !JArray::<T>::has_primitive_elements() {
            return Self::own(len, None, std::iter::repeat_with(T::default));
        }
        let bytes = len.checked_mul(size_of::<PrimSlot<T>>()).expect("数组大小溢出");
        // SAFETY: 基本元素类型的默认值位形全零；值在初始化闭包内写入
        unsafe {
            __Obj::new_trailing_zeroed(bytes, |p: *mut Self| {
                p.write(__ArrayObj { repr: Repr::Own { len, prim: true, tag: None }, _cell: UnsafeCell::new(()), _elem: PhantomData });
            })
        }
    }

    /// 映像数组对象的值（常量求值可用）：`prim` 必须与 `has_primitive_elements` 一致
    pub const fn __image(len: usize, prim: bool) -> Self {
        __ArrayObj { repr: Repr::Own { len, prim, tag: None }, _cell: UnsafeCell::new(()), _elem: PhantomData }
    }

    /// 协变视图数组（无元素）
    pub(super) fn covariant(view: CovariantView) -> __Obj<Self> {
        __Obj::new(__ArrayObj { repr: Repr::Covariant(view), _cell: UnsafeCell::new(()), _elem: PhantomData })
    }

    #[inline]
    pub(super) fn form(&self) -> Form<'_, T> {
        match &self.repr {
            Repr::Own { len, prim, tag } => {
                // SAFETY: 自有元素数组经 `own` 分配，元素区紧随值、恰好 len 个
                let base = unsafe { __trailing::<Self, u8>(self) };
                let store = if *prim {
                    Store::Prim(unsafe { std::slice::from_raw_parts(base as *const PrimSlot<T>, *len) })
                } else {
                    Store::Ref(unsafe { std::slice::from_raw_parts(base as *const __RefField<T>, *len) })
                };
                Form::Own(store, tag.as_ref())
            }
            Repr::Covariant(view) => Form::Covariant(view),
        }
    }

    /// 对象标识：自有元素数组为对象地址；协变视图与源数组相同
    #[inline]
    pub(crate) fn identity(&self) -> *const () {
        match &self.repr {
            Repr::Own { .. } => self as *const Self as *const (),
            Repr::Covariant(view) => view.origin.0.__identity(),
        }
    }

    /// 本数组对象的引用（引用计数加一）：数组对象只存在于 `own` / `covariant` 的分配中
    #[inline]
    pub(super) fn handle(&self) -> JArray<T> {
        // SAFETY: 值位于 `__Obj` 堆分配内（构造入口只有 own / covariant）
        JArray(Some(unsafe { __Obj::from_value(self) }))
    }
}

impl<T> Drop for __ArrayObj<T> {
    fn drop(&mut self) {
        if let Repr::Own { len, prim: false, .. } = self.repr {
            // SAFETY: 引用元素区恰好 len 个已初始化单元，随数组对象一同释放
            unsafe {
                let base = __trailing::<Self, __RefField<T>>(self);
                std::ptr::drop_in_place(std::ptr::slice_from_raw_parts_mut(base, len));
            }
        }
    }
}

/// null 数组装入 Object 的静态哨兵：携带元素类型 T（数组类 / checkcast 的静态探针），不分配。
#[doc(hidden)]
#[repr(align(8))]
pub struct __ArrayNull<T>(PhantomData<fn() -> T>);

impl<T> __ArrayNull<T> {
    pub(super) const NULL: __ArrayNull<T> = __ArrayNull(PhantomData);
}

impl<T: 'static> __ArrayObj<T> {
    /// 基本元素数组的本机字节视图读（`native_memory` 的堆寻址；逐元素原子读）。
    /// 非基本元素数组 / 视图 / 越界返回 false。
    pub(crate) fn __read_bytes(&self, start: usize, dst: &mut [u8]) -> bool {
        matches!(Some(self.form()), Some(Form::Own(store, _)) if store.read_bytes(start, dst))
    }

    /// 基本元素数组的本机字节视图写（部分覆盖的元素按位形 CAS 合并）。失败条件同 `__read_bytes`。
    pub(crate) fn __write_bytes(&self, start: usize, src: &[u8]) -> bool {
        matches!(Some(self.form()), Some(Form::Own(store, _)) if store.write_bytes(start, src))
    }

    /// 基本元素数组字节视图上 `width` 字节值的原子读-改-写（见 `Store::update_bytes`）。
    pub(crate) fn __update_bytes(&self, start: usize, width: usize, op: &mut dyn FnMut(u64) -> Option<u64>) -> Option<u64> {
        match Some(self.form()) {
            Some(Form::Own(store, _)) => store.update_bytes(start, width, op),
            _ => None,
        }
    }
}

// ── 元素存取（JArray 与 Object 的数组访问转发到这里）──

impl<T: Clone + Default + From<Object> + Into<Object> + 'static + crate::sync_model::__ThreadSafe> __ArrayObj<T> {
    /// 读（*aload）。自有基本元素数组走快路径：界检 + 槽的同宽原子读，全程 `#[inline(always)]`
    /// 与整数地址运算，opt-level 0 的调用方（dev-opt 档的用户 crate）也不逐层调用存取链。
    #[inline(always)]
    pub fn get(&self, i: i32) -> crate::error::Result<T> {
        crate::gil::safepoint(); // 安全点钩子（并行后端为空）
        if let Repr::Own { len, prim: true, .. } = self.repr {
            return Ok(from_bits(self.prim_slot(i, len)?.load_bits(Ordering::Relaxed)));
        }
        self.get_general(i)
    }

    /// 自有基本元素数组下标 `i` 的槽；越界抛 `ArrayIndexOutOfBoundsException`（JVMS §6.5）
    #[inline(always)]
    fn prim_slot(&self, i: i32, len: usize) -> crate::error::Result<&PrimSlot<T>> {
        if i < 0 || i as usize >= len {
            return Err(crate::error::JvmError::array_index_out_of_bounds(i, len as i32));
        }
        // 元素区紧随值（`own` / `own_default` 分配）；整数地址运算，不经 std 指针方法
        let addr = self as *const Self as usize + size_of::<Self>() + i as usize * size_of::<PrimSlot<T>>();
        // SAFETY: 下标在界内，槽已初始化，随数组对象存活
        Ok(unsafe { &*(addr as *const PrimSlot<T>) })
    }

    fn get_general(&self, i: i32) -> crate::error::Result<T> {
        match self.form() {
            Form::Own(store, _) => store.get(i),
            Form::Covariant(view) => Ok(T::from((view.get)(&view.origin, i)?)),
        }
    }

    pub fn __update(&self, i: i32, f: &mut dyn FnMut(T) -> Option<T>) -> crate::error::Result<T> {
        match self.form() {
            Form::Own(store, _) => store.update(i, f),
            Form::Covariant(view) => {
                let old = (view.update)(&view.origin, i, &mut |cur: Object| f(T::from(cur)).map(Into::into))?;
                Ok(T::from(old))
            }
        }
    }

    /// 写（*astore）。自有基本元素数组走快路径（同 `get`）。
    #[inline(always)]
    pub fn set(&self, i: i32, v: T) -> crate::error::Result<()> {
        if let Repr::Own { len, prim: true, .. } = self.repr {
            self.prim_slot(i, len)?.store_bits(to_bits(v), Ordering::Relaxed);
            return Ok(());
        }
        self.set_general(i, v)
    }

    fn set_general(&self, i: i32, v: T) -> crate::error::Result<()> {
        match self.form() {
            Form::Own(store, tag) => {
                if let Some(tag) = tag {
                    // 反射创建数组的 aastore 存储检查（JVMS §6.5 aastore）
                    let o: Object = Clone::clone(&v).into();
                    if !o.0.is_jvm_null() && !o.0.is_instance_of(tag) {
                        return Err(crate::error::JvmError::array_store(o.0.__class_name()));
                    }
                }
                store.set(i, v)
            }
            Form::Covariant(view) => (view.set)(&view.origin, i, v.into()),
        }
    }

    pub fn to_vec(&self) -> Vec<T> {
        match self.form() {
            Form::Own(store, _) => store.to_vec(),
            Form::Covariant(view) => (0..view.len())
                .map(|i| T::from((view.get)(&view.origin, i).expect("index within length")))
                .collect(),
        }
    }

    #[inline(always)]
    pub fn len(&self) -> i32 {
        if let Repr::Own { len, .. } = self.repr {
            return len as i32;
        }
        match self.form() {
            Form::Own(store, _) => store.len() as i32,
            Form::Covariant(view) => view.len(),
        }
    }
}

/// 构建期引导映像中的数组：`[映像对象头 | __ArrayObj<T> | 元素区 S]`，与堆数组同一布局
/// （元素区紧随值，`__trailing`）。`S`：基本元素为同宽位形数组的 `UnsafeCell`（如 `byte[]` 取
/// `UnsafeCell<[u8; N]>`，与 `[PrimSlot<i8>; N]` 同布局），引用元素为 `[__RefField<T>; N]`。
#[repr(C)]
pub struct __ImageArr<T, S> {
    head: crate::obj_ref::Header,
    pub value: __ArrayObj<T>,
    elems: S,
}

// 元素只经原子指令或单元锁访问（与 `__ArrayObj` 同一条件）
unsafe impl<T: Send + Sync, S> Sync for __ImageArr<T, S> {}

impl<T: 'static, S> __ImageArr<T, S> {
    /// `len` 个元素；`prim` 为基本元素；`hash` 同 `__ImageObj::new`
    pub const fn new(hash: Option<i32>, len: usize, prim: bool, elems: S) -> Self {
        let width = if prim { size_of::<PrimSlot<T>>() } else { size_of::<__RefField<T>>() };
        assert!(size_of::<S>() == len * width, "映像数组元素区大小与长度不符");
        assert!(size_of::<__ArrayObj<T>>() % align_of::<S>() == 0, "映像数组元素区未紧随数组对象");
        __ImageArr { head: crate::obj_ref::Header::image(hash), value: __ArrayObj::__image(len, prim), elems }
    }
}

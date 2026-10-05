//! 统一对象句柄（S7-2，docs/plans/2026-10-04-s7-object-handle-descriptor.md §3.1 方案 B）。
//!
//! 类 wrapper 只有一个字段 `__r: __Ref<dyn X__VTable>`：
//! - `__Handle`：对象的唯一持有者 `Option<__Shared<dyn ObjectVTable>>`，`None` 即 Java null——
//!   null 不再分配一份缺省存储（`Default` 零分配）；
//! - `vt`：指向句柄所持对象的本类视图指针（不持有），虚分派直接经它取 `&dyn X__VTable`，
//!   开销与原 `__Shared<dyn X__VTable>` 相同（§4.3）；上转只换指针（trait upcasting），不动句柄。
//!
//! 名字带 `__` 前缀：不是 Java 类型，只出现在生成器封装层，不进入可读方法体。

use crate::java::lang::ObjectVTable;
use crate::sync_model::__Shared;
use crate::exec_context::release_slot;
use std::ptr::NonNull;

/// 对象句柄：持有对象（运行时类的存储）或为 null。
#[derive(Clone, Default)]
pub struct __Handle(Option<__Shared<dyn ObjectVTable>>);

impl __Handle {
    /// null 句柄。
    pub const NULL: __Handle = __Handle(None);

    #[inline]
    pub fn new(rc: __Shared<dyn ObjectVTable>) -> Self { __Handle(Some(rc)) }

    #[inline]
    pub fn is_none(&self) -> bool { self.0.is_none() }

    /// 句柄所持对象（运行时类的存储）；null → None。
    #[inline]
    pub fn target(&self) -> Option<&dyn ObjectVTable> { self.0.as_deref() }

    /// 装入 Object（`From<X> for Object`，S7-2b）：Object 直接持有句柄所持存储（运行时类对象），
    /// 不再包一层 wrapper；null → 带本类描述符的类型化 null（`__class_name` / `__desc` 报静态类）。
    #[inline]
    pub fn into_object(mut self, desc: &'static crate::class_desc::__ClassDesc) -> crate::java::lang::Object {
        match self.0.take() {
            Some(rc) => crate::java::lang::Object::__from_shared(rc),
            None => crate::java::lang::Object::__typed_null_desc(desc),
        }
    }

    /// 引用相等（`==`）：两个 null 相等；非 null 按对象标识（同一对象的各视图共享标识单元）。
    pub fn same(a: &__Handle, b: &__Handle) -> bool {
        match (a.target(), b.target()) {
            (None, None) => true,
            (Some(x), Some(y)) => x.__identity() == y.__identity(),
            _ => false,
        }
    }

    /// Debug 用的对象字符串：null → `null`。
    pub fn obj_str(&self) -> std::string::String {
        match self.target() {
            Some(t) => t.__obj_str(),
            None => "null".to_owned(),
        }
    }
}

impl Drop for __Handle {
    #[inline]
    fn drop(&mut self) {
        if let Some(rc) = self.0.take() {
            __release(rc);
        }
    }
}

// ── 非递归释放（S7-3x）────────────────────────────────────────────────────────
//
// 释放对象的最后一个强引用会经 drop glue 释放其字段，字段又持有对象：Arc → 字段单元 → 句柄 →
// Arc …，朴素递归的栈深等于引用链长（长链表节点、cause 链），协程栈（1 MiB）与主线程都会溢出。
// 句柄与 Object 释放最后一个强引用时经此计深：深度未超阈值就地释放；超阈值把该对象移入线程本地
// 待释放队列，由最外层释放循环清空——任意链长下释放栈深有界（阈值 × 单层 drop 帧）。
// 释放中不会让出（drop glue 不执行 Java 代码），载体线程的线程局部即当前栈的状态。

/// 就地释放的最大嵌套深度（超出的对象入待释放队列）
const RELEASE_MAX_DEPTH: u32 = 32;

/// 释放一个对象引用（`__Handle` / `Object` 的 Drop）：非最后一个强引用只减计数；最后一个按深度
/// 就地释放或入队，最外层释放清空队列。线程局部已销毁（线程退出期）时就地释放。
#[inline]
pub(crate) fn __release(rc: __Shared<dyn ObjectVTable>) {
    if __Shared::strong_count(&rc) != 1 {
        return;
    }
    release_last(rc);
}

#[inline(never)]
fn release_last(rc: __Shared<dyn ObjectVTable>) {
    // 释放槽不可用（线程退出期）时 rc 随闭包丢弃，就地释放
    let Some(depth) = release_slot(|s| s.depth.get()) else { return };
    if depth >= RELEASE_MAX_DEPTH {
        release_slot(move |s| s.pending.borrow_mut().push(rc));
        return;
    }
    release_slot(|s| s.depth.set(depth + 1));
    drop(rc);
    if depth == 0 {
        // 最外层：逐个释放队列中的对象（其字段再次超深的入队，循环至空）
        while let Some(Some(next)) = release_slot(|s| s.pending.borrow_mut().pop()) {
            drop(next);
        }
    }
    release_slot(|s| s.depth.set(depth));
}

/// 类型化引用：句柄 + 本类视图指针。`vt` 恒指向 `h` 所持对象（或二者同为空），
/// 句柄存活即指针有效；字段私有，只经下列安全构造入口建立该不变式。
pub struct __Ref<V: ?Sized> {
    h: __Handle,
    vt: Option<NonNull<V>>,
}

// `vt` 指向 `h` 所持（Send + Sync）对象，借用期不超过 `h`
unsafe impl<V: ?Sized + Send + Sync> Send for __Ref<V> {}
unsafe impl<V: ?Sized + Send + Sync> Sync for __Ref<V> {}

impl<V: ?Sized> Clone for __Ref<V> {
    #[inline]
    fn clone(&self) -> Self { __Ref { h: self.h.clone(), vt: self.vt } }
}

impl<V: ?Sized> Default for __Ref<V> {
    #[inline]
    fn default() -> Self { Self::NULL }
}

impl<V: ?Sized> __Ref<V> {
    /// null 引用（不分配）。
    pub const NULL: Self = __Ref { h: __Handle::NULL, vt: None };

    /// 以新存储建立引用：`view` 给出存储的本类视图（`|i| i as &dyn X__VTable`）。
    #[inline]
    pub fn new<T: ObjectVTable>(rc: __Shared<T>, view: impl for<'a> FnOnce(&'a T) -> &'a V) -> Self {
        let vt = NonNull::from(view(&*rc));
        __Ref { h: __Handle::new(rc), vt: Some(vt) }
    }

    /// 以存储自身建立引用（`impl X__VTable for X__inner` 的 wrapper 重建钩子）：存储位于分配钩子
    /// 建立的 `__Shared<T>` 中，引用计数加一即得同一对象的句柄——不复制存储，对象标识即存储地址。
    ///
    /// # Safety
    /// `this` 必须是某个以 `T` 分配的 `__Shared<T>` 所持的值。
    #[inline]
    pub unsafe fn from_storage<T: ObjectVTable>(this: &T, view: impl for<'a> FnOnce(&'a T) -> &'a V) -> Self {
        let p = this as *const T;
        // SAFETY: 调用方保证 p 来自 `__Shared::<T>`；先加计数，再收回一个强引用
        let rc = unsafe {
            __Shared::increment_strong_count(p);
            __Shared::from_raw(p)
        };
        Self::new(rc, view)
    }

    /// 由句柄与其所持对象上取得的视图指针合成（`__erased_vtable` 填入的指针）。
    ///
    /// # Safety
    /// `vt` 必须指向 `h` 所持对象（同一分配）。
    #[inline]
    pub unsafe fn from_raw(h: __Handle, vt: NonNull<V>) -> Self { __Ref { h, vt: Some(vt) } }

    #[inline]
    pub fn is_none(&self) -> bool { self.vt.is_none() }

    #[inline]
    pub fn handle(&self) -> &__Handle { &self.h }

    /// 装入 Object（见 `__Handle::into_object`）。
    #[inline]
    pub fn into_object(self, desc: &'static crate::class_desc::__ClassDesc) -> crate::java::lang::Object {
        self.h.into_object(desc)
    }

    /// 本类视图（虚分派 / 字段访问入口）。null 接收者在入口检查已抛 NPE，
    /// 到这里仍为 null 只可能是不经 `Result` 的内部路径——冷路径 panic。
    #[inline]
    pub fn vt(&self) -> &V {
        match self.vt {
            // SAFETY: 不变式——vt 指向 h 所持对象，h 与 self 同寿
            Some(p) => unsafe { p.as_ref() },
            None => __null_view(),
        }
    }

    /// 上转（子类引用 → 祖先类引用）：句柄不变，视图指针经 trait upcasting 换为祖先视图。
    #[inline]
    pub fn upcast<U: ?Sized>(self, view: impl for<'a> FnOnce(&'a V) -> &'a U) -> __Ref<U> {
        let vt = self.vt.map(|p| {
            // SAFETY: 同 `vt()`；新指针由同一对象上的引用转换得到
            NonNull::from(view(unsafe { p.as_ref() }))
        });
        __Ref { h: self.h, vt }
    }
}

#[cold]
#[inline(never)]
fn __null_view() -> ! {
    panic!("NullPointerException: 在 null 引用上分派")
}

/// 按 Object 所持存储重建 `V` 视图（`V = dyn X__VTable`）：运行时类是 X 或其子类时 Some。
/// `From<Object>` 擦除路径与 `__virtual_view` 共用；Object 直接持有运行时类存储（S7-2b），
/// 以它为句柄。
pub fn __ref_from_object<V: ?Sized + 'static>(obj: &crate::java::lang::Object) -> Option<__Ref<V>> {
    let mut slot: Option<NonNull<V>> = None;
    obj.0.__erased_vtable(&mut slot);
    // SAFETY: `__erased_vtable` 在 Object 所持存储自身上取视图指针，句柄持有同一存储
    slot.map(|vt| unsafe { __Ref::from_raw(__Handle::new(__Shared::clone(&obj.0)), vt) })
}

/// 接口引用（接口载体的唯一字段，S7-2c）：句柄是 `Object`（载体 `Deref<Target = Object>`，
/// null 带接口静态类型），另持接口视图指针 `vt`（不持有，指向 `obj` 所持对象）——在
/// `From<Object>` 时经 `ObjectVTable::__interface` 求出一次，invokeinterface 直接经它分派，
/// 不再每次克隆引用、查询。`vt` 为 None 而 `obj` 非 null：运行时类不实现该接口
/// （动态代理、只走 default 体的对象），分派落到代理 / default / AbstractMethodError 回退。
pub struct __IfaceRef<V: ?Sized> {
    obj: crate::java::lang::Object,
    vt: Option<NonNull<V>>,
}

// `vt` 指向 `obj` 所持（Send + Sync）对象，借用期不超过 `obj`
unsafe impl<V: ?Sized + Send + Sync> Send for __IfaceRef<V> {}
unsafe impl<V: ?Sized + Send + Sync> Sync for __IfaceRef<V> {}

impl<V: ?Sized> Clone for __IfaceRef<V> {
    #[inline]
    fn clone(&self) -> Self { __IfaceRef { obj: self.obj.clone(), vt: self.vt } }
}

impl<V: ?Sized> std::ops::Deref for __IfaceRef<V> {
    type Target = crate::java::lang::Object;
    #[inline]
    fn deref(&self) -> &crate::java::lang::Object { &self.obj }
}

impl<V: ?Sized + 'static> __IfaceRef<V> {
    /// 以 Object 建立接口引用：运行时类实现该接口时取得接口视图指针（`V = dyn I__VTable`）。
    pub fn new(obj: crate::java::lang::Object) -> Self {
        let mut slot: Option<NonNull<V>> = None;
        obj.0.__interface(&mut slot);
        __IfaceRef { obj, vt: slot }
    }

    /// 带接口静态类型的 null（不查询）。
    #[inline]
    pub fn null(obj: crate::java::lang::Object) -> Self { __IfaceRef { obj, vt: None } }

    /// 交出句柄（`From<Iface> for Object`）。
    #[inline]
    pub fn into_object(self) -> crate::java::lang::Object { self.obj }

    /// invokeinterface 载体分派的入口部分：接收者为 null 时抛 NullPointerException（先于方法
    /// 选择），建帧做栈界检查，再交出接口视图；接收者不实现该接口时为 `None`。
    #[inline]
    pub fn enter(&self) -> crate::error::Result<Option<&V>> {
        match self.vt {
            Some(p) => {
                crate::__stack_check()?;
                // SAFETY: 不变式——vt 指向 obj 所持对象，obj 与 self 同寿
                Ok(Some(unsafe { p.as_ref() }))
            }
            None => {
                if self.obj.0.is_jvm_null() {
                    return Err(crate::error::JvmError::null_pointer());
                }
                crate::__stack_check()?;
                Ok(None)
            }
        }
    }
}

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
    pub fn into_object(self, desc: &'static crate::class_desc::__ClassDesc) -> crate::java::lang::Object {
        match self.0 {
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

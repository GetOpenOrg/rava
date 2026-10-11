//! 统一对象句柄（S7-2；S7 计划 §3.1 取法 A，§9.9 K1）。
//!
//! 类 wrapper 只有一个字段 `__r: __Handle`（对象的唯一持有者 `Option<__Obj<dyn ObjectVTable>>`，
//! `None` 即 Java null——null 不分配存储）。wrapper 不再持有本类视图指针：类型与签名无关，
//! 是类型标记 crate 的前提（§9.9）。虚分派入口 `X::__vt()` 经 `__Handle::view` 按本类的
//! display 深度向运行时类存储取视图（`ObjectVTable::__erased_vtable`，一次间接调用 + 一次槽
//! 类型比较，常数级、与层次深度无关）；上转只搬移句柄。
//!
//! 名字带 `__` 前缀：不是 Java 类型，只出现在生成器封装层，不进入可读方法体。

use crate::java::lang::ObjectVTable;
use crate::obj_ref::__Obj;
use crate::exec_context::release_slot;
use std::ptr::NonNull;

/// 对象句柄：持有对象（运行时类的存储）或为 null。
#[derive(Clone, Default)]
pub struct __Handle(Option<__Obj<dyn ObjectVTable>>);

impl __Handle {
    /// null 句柄。
    pub const NULL: __Handle = __Handle(None);

    #[inline]
    pub fn new(rc: __Obj<dyn ObjectVTable>) -> Self { __Handle(Some(rc)) }

    /// 以新存储建立句柄（分配钩子 `__jb_X__alloc`）。
    #[inline]
    pub fn alloc<T: ObjectVTable>(rc: __Obj<T>) -> Self {
        __Handle::new(rc.map_ptr(|p| p as *mut dyn ObjectVTable))
    }

    /// 以存储自身建立句柄（`impl X__VTable for X__inner` 的 wrapper 重建钩子）：存储位于分配钩子
    /// 建立的 `__Obj<T>` 中，引用计数加一即得同一对象的句柄——不复制存储，对象标识即存储地址。
    ///
    /// # Safety
    /// `this` 必须是某个以 `T` 分配的 `__Obj<T>` 所持的值。
    #[inline]
    pub unsafe fn from_storage<T: ObjectVTable>(this: &T) -> Self {
        // SAFETY: 调用方保证 this 是堆对象的值
        Self::alloc(unsafe { __Obj::from_value(this) })
    }

    /// 本类视图（wrapper 的分派入口 `__vt()`，`V = dyn X__VTable`）：按本类在 display 表中的
    /// 深度问运行时类存储（`__erased_vtable`），存储按深度只比较一次槽类型。wrapper 的句柄
    /// 恒持本类或其子类的实例（构造点保证），未命中只可能是不经 `Result` 的内部路径上的 null——
    /// 冷路径 panic，与原视图指针为空时相同。
    #[inline]
    pub fn view<V: ?Sized + 'static>(&self, depth: u16) -> &V {
        let mut slot: Option<NonNull<V>> = None;
        if let Some(t) = self.target() {
            t.__erased_vtable(depth, &mut slot);
        }
        match slot {
            // SAFETY: 指针由句柄所持对象自身上的引用转换得到（同一分配），借用期不超过 self
            Some(p) => unsafe { p.as_ref() },
            None => __null_view(self.is_none()),
        }
    }

    /// 映像对象的句柄（常量求值可用，引导映像物化）
    pub const fn image(rc: __Obj<dyn ObjectVTable>) -> Self { __Handle(Some(rc)) }

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
pub(crate) fn __release(rc: __Obj<dyn ObjectVTable>) {
    if !rc.is_unique() {
        return;
    }
    release_last(rc);
}

#[inline(never)]
fn release_last(rc: __Obj<dyn ObjectVTable>) {
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

#[cold]
#[inline(never)]
fn __null_view(null: bool) -> ! {
    if null {
        panic!("NullPointerException: 在 null 引用上分派")
    }
    panic!("句柄所持对象不是 wrapper 静态类型的实例（构造点违反不变式）")
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

    /// 映像对象的接口引用（常量求值可用）：`vt` 是 `obj` 所持映像对象的接口视图
    pub const fn image(obj: crate::java::lang::Object, vt: &'static V) -> Self {
        // SAFETY: 引用非空
        __IfaceRef { obj, vt: Some(unsafe { NonNull::new_unchecked(vt as *const V as *mut V) }) }
    }

    /// 带接口静态类型的 null（不查询）。
    #[inline]
    pub const fn null(obj: crate::java::lang::Object) -> Self { __IfaceRef { obj, vt: None } }

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

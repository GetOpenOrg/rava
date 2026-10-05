//! Java 对象的引用计数指针 `__Obj<T>`（R1 余项：null 无计数哨兵；数组单一分配）。
//!
//! - **堆对象**：一次分配 `[头 16 字节 | 值 T | 尾随元素]`，头为强引用计数与分配字节数（无弱引用：
//!   对象模型不用 `Weak`）。指针存值地址，解引用不做偏移运算；头在值地址前固定 16 字节（值对齐 ≤ 16）。
//! - **静态哨兵**：指向不计数的 `'static` 值（null 与类型化 null），指针最低位置 1 作标记。克隆 / 释放
//!   只测一位，不读写任何共享计数——多线程间传递 null 不再争用同一缓存行。哨兵值的对齐 ≥ 2（低位空闲）。
//! - **尾随元素**：数组对象的元素紧随值存放（`new_trailing`），与对象同一分配；释放时由值的 `Drop`
//!   析构元素、按头记录的字节数归还。
//!
//! 名字带 `__` 前缀：对象模型实现细节，不出现在可读层（方法体）。

use std::alloc::Layout;
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::atomic::{fence, AtomicUsize, Ordering};

/// 对象头：值地址前 16 字节
#[repr(C, align(16))]
struct Header {
    strong: AtomicUsize,
    /// 整个分配的字节数（头 + 值 + 尾随元素）
    size: usize,
}

/// 头的大小，也是值相对分配起点的偏移
const HEAD: usize = std::mem::size_of::<Header>();
/// 分配的对齐（头的对齐；值的对齐不得超过它）
const ALIGN: usize = std::mem::align_of::<Header>();
/// 静态哨兵标记位
const STATIC_TAG: usize = 1;
/// 计数上限：超出即中止（与 `Arc` 同一防护，防止计数回绕后提前释放）
const MAX_REFCOUNT: usize = isize::MAX as usize;

/// Java 对象的共享指针：堆对象引用计数，静态哨兵不计数。
pub struct __Obj<T: ?Sized> {
    /// 值地址；静态哨兵的地址最低位置 1
    ptr: NonNull<T>,
    _owns: PhantomData<T>,
}

// 与 `Arc<T>` 同一条件
unsafe impl<T: ?Sized + Send + Sync> Send for __Obj<T> {}
unsafe impl<T: ?Sized + Send + Sync> Sync for __Obj<T> {}

impl<T> __Obj<T> {
    /// 分配一个堆对象
    #[inline]
    pub fn new(value: T) -> __Obj<T> {
        // SAFETY: 尾随 0 个元素，初始化闭包写入值
        unsafe { Self::alloc_with(0, false, |p| p.write(value)) }
    }

    /// 分配值之后紧随 `trailing` 字节的对象（数组元素）：`init` 收到值地址，负责写入值与尾随元素。
    ///
    /// # Safety
    /// `init` 必须完整初始化值；值的 `Drop` 负责析构尾随元素；尾随区相对值地址的偏移
    /// `size_of::<T>()` 须满足元素对齐（元素对齐 ≤ 16 且整除 `size_of::<T>()` 对齐后的位置）。
    #[inline]
    pub unsafe fn new_trailing(trailing: usize, init: impl FnOnce(*mut T)) -> __Obj<T> {
        unsafe { Self::alloc_with(trailing, false, init) }
    }

    /// 同 [`Self::new_trailing`]，但尾随区预先清零（`alloc_zeroed`：大块直接取自零页，不逐元素写入）。
    /// `init` 只需写入值；全零位形即元素的合法初值时使用（基本类型数组的 Java 默认值）。
    ///
    /// # Safety
    /// 同 [`Self::new_trailing`]；另须全零位形是尾随元素的合法值。
    #[inline]
    pub unsafe fn new_trailing_zeroed(trailing: usize, init: impl FnOnce(*mut T)) -> __Obj<T> {
        unsafe { Self::alloc_with(trailing, true, init) }
    }

    #[inline]
    unsafe fn alloc_with(trailing: usize, zeroed: bool, init: impl FnOnce(*mut T)) -> __Obj<T> {
        const { assert!(std::mem::align_of::<T>() <= ALIGN, "对象值的对齐超过 16") };
        let size = (HEAD + std::mem::size_of::<T>() + trailing).next_multiple_of(ALIGN);
        let layout = Layout::from_size_align(size, ALIGN).expect("对象大小溢出");
        // SAFETY: size ≥ HEAD > 0
        let base = unsafe { if zeroed { std::alloc::alloc_zeroed(layout) } else { std::alloc::alloc(layout) } };
        if base.is_null() {
            std::alloc::handle_alloc_error(layout);
        }
        // SAFETY: base 按 Header 对齐、空间足够
        unsafe {
            (base as *mut Header).write(Header { strong: AtomicUsize::new(1), size });
            let value = base.add(HEAD) as *mut T;
            init(value);
            __Obj { ptr: NonNull::new_unchecked(value), _owns: PhantomData }
        }
    }
}

impl<T: ?Sized> __Obj<T> {
    /// 静态哨兵（不计数）：`value` 的对齐须 ≥ 2
    #[inline]
    pub fn from_static(value: &'static T) -> __Obj<T> {
        let p = value as *const T as *mut T;
        debug_assert!(p as *const () as usize & STATIC_TAG == 0, "静态哨兵对齐不足");
        // SAFETY: 置标记位后仍非空
        __Obj { ptr: unsafe { NonNull::new_unchecked(p.map_addr(|a| a | STATIC_TAG)) }, _owns: PhantomData }
    }

    /// 是否为静态哨兵（null / 类型化 null）
    #[inline(always)]
    pub fn is_static(&self) -> bool {
        self.ptr.as_ptr() as *const () as usize & STATIC_TAG != 0
    }

    /// 值地址（去掉标记位）
    #[inline(always)]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr().map_addr(|a| a & !STATIC_TAG)
    }

    #[inline(always)]
    fn header(&self) -> &Header {
        // SAFETY: 只对堆对象调用；头在值地址前 HEAD 字节
        unsafe { &*((self.ptr.as_ptr() as *const u8).sub(HEAD) as *const Header) }
    }

    /// 唯一持有：堆对象且强引用计数为 1（释放时的计深判定）；静态哨兵恒为 false
    #[inline]
    pub fn is_unique(&self) -> bool {
        !self.is_static() && self.header().strong.load(Ordering::Acquire) == 1
    }

    /// 两个指针指向同一对象（静态哨兵按地址比较）
    #[inline]
    pub fn ptr_eq(a: &Self, b: &Self) -> bool {
        std::ptr::addr_eq(a.as_ptr(), b.as_ptr())
    }

    /// 以值引用重建一个持有者（引用计数加一）：wrapper 重建钩子、数组视图由存储自身取句柄。
    ///
    /// # Safety
    /// `value` 必须是某个 `__Obj` 所持的值（堆对象或静态哨兵均可，但须与原指针同标记：堆对象
    /// 的值地址前有头）。静态哨兵须经 `from_static` 取得，不经此入口。
    #[inline]
    pub unsafe fn from_value(value: &T) -> __Obj<T> {
        let p = value as *const T as *mut T;
        // SAFETY: 调用方保证 p 是堆对象的值地址
        let obj = __Obj { ptr: unsafe { NonNull::new_unchecked(p) }, _owns: PhantomData };
        obj.inc();
        obj
    }

    /// 改变指针的静态类型（unsize：`|p| p as *mut dyn Trait`）。`f` 必须保持地址不变。
    #[inline]
    pub fn map_ptr<U: ?Sized>(self, f: impl FnOnce(*mut T) -> *mut U) -> __Obj<U> {
        let me = std::mem::ManuallyDrop::new(self);
        let q = f(me.ptr.as_ptr());
        debug_assert!(std::ptr::addr_eq(q, me.ptr.as_ptr()));
        // SAFETY: 地址不变（含标记位），仍非空
        __Obj { ptr: unsafe { NonNull::new_unchecked(q) }, _owns: PhantomData }
    }

    #[inline(always)]
    fn inc(&self) {
        if self.is_static() {
            return;
        }
        let old = self.header().strong.fetch_add(1, Ordering::Relaxed);
        if old > MAX_REFCOUNT {
            std::process::abort();
        }
    }

    /// 最后一个引用释放：析构值（含尾随元素）并归还分配
    #[inline(never)]
    unsafe fn drop_slow(&mut self) {
        // SAFETY: 计数已归零，本线程独占；头记录分配大小
        unsafe {
            let size = self.header().size;
            let base = (self.ptr.as_ptr() as *mut u8).sub(HEAD);
            std::ptr::drop_in_place(self.ptr.as_ptr());
            std::alloc::dealloc(base, Layout::from_size_align_unchecked(size, ALIGN));
        }
    }
}

impl<T: ?Sized> Clone for __Obj<T> {
    #[inline]
    fn clone(&self) -> Self {
        self.inc();
        __Obj { ptr: self.ptr, _owns: PhantomData }
    }
}

impl<T: ?Sized> Drop for __Obj<T> {
    #[inline]
    fn drop(&mut self) {
        if self.is_static() {
            return;
        }
        if self.header().strong.fetch_sub(1, Ordering::Release) != 1 {
            return;
        }
        fence(Ordering::Acquire);
        // SAFETY: 最后一个引用
        unsafe { self.drop_slow() }
    }
}

impl<T: ?Sized> std::ops::Deref for __Obj<T> {
    type Target = T;
    #[inline(always)]
    fn deref(&self) -> &T {
        // SAFETY: 堆对象存活至最后一个引用释放；静态哨兵为 'static
        unsafe { &*self.as_ptr() }
    }
}

impl<T: ?Sized> AsRef<T> for __Obj<T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self
    }
}

/// 尾随元素区的起点（值地址之后 `size_of::<T>()` 字节）
///
/// # Safety
/// `value` 必须是经 `new_trailing` 分配的对象的值。
#[inline]
pub unsafe fn __trailing<T, E>(value: &T) -> *mut E {
    // SAFETY: 尾随区紧随值
    unsafe { (value as *const T as *mut u8).add(std::mem::size_of::<T>()) as *mut E }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    static DROPS: AtomicUsize = AtomicUsize::new(0);
    struct D(#[allow(dead_code)] u64);
    impl Drop for D {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::Relaxed);
        }
    }

    #[test]
    fn counted_and_static() {
        let a = __Obj::new(D(7));
        let b = a.clone();
        assert!(!a.is_unique());
        drop(b);
        assert!(a.is_unique());
        let before = DROPS.load(Ordering::Relaxed);
        drop(a);
        assert_eq!(DROPS.load(Ordering::Relaxed), before + 1);

        static S: u64 = 5;
        let s = __Obj::from_static(&S);
        let t = s.clone();
        assert!(s.is_static() && !s.is_unique());
        assert_eq!(*t, 5);
        assert!(__Obj::ptr_eq(&s, &t));
    }
}

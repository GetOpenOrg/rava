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
use std::sync::OnceLock;

/// 对象头：值地址前 16 字节
#[repr(C, align(16))]
pub(crate) struct Header {
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

/// 映像对象的常驻强引用计数：克隆 / 释放照常增减而永不归零（不释放、不需要静态标记位）
const IMMORTAL: usize = 1 << 62;
/// 映像对象头第二个字的标记：低 32 位为构建期身份哈希（堆对象该字为分配字节数，恒为 16 的倍数）
const IMAGE_HASHED: usize = 1 << 63;

impl Header {
    /// 映像对象头（计划 2026-10-05-boot-image-evaluator §5.5.2 D2 / D3）
    pub(crate) const fn image(hash: Option<i32>) -> Header {
        Header {
            strong: AtomicUsize::new(IMMORTAL),
            size: match hash {
                Some(h) => IMAGE_HASHED | (h as u32 as usize),
                None => 0,
            },
        }
    }
}

/// 构建期引导映像中的对象：与堆对象同一布局（16 字节头 + 值），作为映像静态结构的字段常驻。
/// 名字带 `__` 前缀：对象模型实现细节，只出现在生成的映像模块。
#[repr(C)]
pub struct __ImageObj<T> {
    head: Header,
    pub value: T,
}

impl<T> __ImageObj<T> {
    /// `hash`：构建期取过身份哈希的对象带上该值（运行期 `identityHashCode` 原样返回）
    pub const fn new(hash: Option<i32>, value: T) -> Self {
        const { assert!(std::mem::align_of::<T>() <= ALIGN, "对象值的对齐超过 16") };
        __ImageObj { head: Header::image(hash), value }
    }
}

// 映像区（按规模分段的静态结构）的地址区间：启动时登记一次，身份哈希据此判定映像对象。
// IMAGE_LO / IMAGE_HI 是全部段的包络（快速排除堆对象），IMAGE_SEGS 是按起址排序的各段 [起, 止)
static IMAGE_LO: AtomicUsize = AtomicUsize::new(0);
static IMAGE_HI: AtomicUsize = AtomicUsize::new(0);
static IMAGE_SEGS: OnceLock<Box<[(usize, usize)]>> = OnceLock::new();

/// 登记映像区各段（生成的启动序列第一步调用一次）：`(段起址, 段字节数)`
pub fn __image_register(segs: &[(*const u8, usize)]) {
    let mut v: Vec<(usize, usize)> = segs.iter().filter(|s| s.1 > 0).map(|&(p, n)| (p as usize, p as usize + n)).collect();
    v.sort_unstable();
    let (Some(lo), Some(hi)) = (v.first().map(|s| s.0), v.iter().map(|s| s.1).max()) else { return };
    if IMAGE_SEGS.set(v.into_boxed_slice()).is_ok() {
        IMAGE_LO.store(lo, Ordering::Relaxed);
        IMAGE_HI.store(hi, Ordering::Release);
    }
}

/// 映像对象的构建期身份哈希：`id` 为对象值地址；不在映像区或构建期未取哈希 → None
#[inline]
pub fn __image_hash(id: *const ()) -> Option<i32> {
    let a = id as usize;
    let hi = IMAGE_HI.load(Ordering::Acquire);
    if a >= hi || a < IMAGE_LO.load(Ordering::Relaxed) + HEAD {
        return None;
    }
    let segs = IMAGE_SEGS.get()?;
    let k = segs.partition_point(|s| s.0 + HEAD <= a);
    let &(start, end) = segs.get(k.checked_sub(1)?)?;
    if a < start + HEAD || a >= end {
        return None;
    }
    // SAFETY: 映像段内的对象值地址前 HEAD 字节是其对象头（映像段结构的每个字段都是 `__ImageObj` / 映像数组）
    let size = unsafe { (*((a - HEAD) as *const Header)).size };
    (size & IMAGE_HASHED != 0).then_some(size as u32 as i32)
}

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
    /// 映像对象的指针（常量求值可用）：`value` 必须是 `__ImageObj` / 映像数组的值（前有映像对象头）；
    /// 计数常驻，按堆对象处理
    pub const fn image(value: &'static T) -> __Obj<T> {
        // SAFETY: 引用非空
        __Obj { ptr: unsafe { NonNull::new_unchecked(value as *const T as *mut T) }, _owns: PhantomData }
    }

    /// 静态哨兵（不计数）：`value` 的对齐须 ≥ 2
    #[inline]
    pub fn from_static(value: &'static T) -> __Obj<T> {
        let p = value as *const T as *mut T;
        debug_assert!(p as *const () as usize & STATIC_TAG == 0, "静态哨兵对齐不足");
        // SAFETY: 置标记位后仍非空
        __Obj { ptr: unsafe { NonNull::new_unchecked(p.map_addr(|a| a | STATIC_TAG)) }, _owns: PhantomData }
    }

    /// 静态哨兵（常量求值可用，引导映像的 null 元素）：同 [`Self::from_static`]，对齐 ≥ 2 时
    /// 加 1 即置标记位（哨兵须非零大小，否则常量求值判其可能为空指针）
    pub const fn from_static_const(value: &'static T) -> __Obj<T> {
        let p = (value as *const T as *mut T).wrapping_byte_add(STATIC_TAG);
        // SAFETY: 非空地址加 1 后仍非空
        __Obj { ptr: unsafe { NonNull::new_unchecked(p) }, _owns: PhantomData }
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

    #[repr(C)]
    struct Img {
        a: __ImageObj<u64>,
        b: __ImageObj<u64>,
    }
    static IMG: Img = Img { a: __ImageObj::new(Some(0x1234), 7), b: __ImageObj::new(None, 9) };

    #[test]
    fn image_objects_are_immortal_and_hashed() {
        __image_register(&[(&IMG as *const Img as *const u8, std::mem::size_of::<Img>())]);
        let a = __Obj::image(&IMG.a.value);
        let a2 = a.clone();
        drop(a2);
        drop(a);
        assert_eq!(*__Obj::image(&IMG.a.value), 7);
        assert!(!__Obj::image(&IMG.b.value).is_unique());
        assert_eq!(__image_hash(&IMG.a.value as *const u64 as *const ()), Some(0x1234));
        assert_eq!(__image_hash(&IMG.b.value as *const u64 as *const ()), None);
        let heap = __Obj::new(5u64);
        assert_eq!(__image_hash(heap.as_ptr() as *const ()), None);
    }
}

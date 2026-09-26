//! 同步模型原语（#42 真多线程，docs/plans/2026-09-26-real-multithreading.md）。
//!
//! 对象模型的共享、字段单元、引用槽、进程级存储四种原语在此集中定义；java_class! 宏展开、
//! 生成代码与手写层一律经这些名字引用，不直接写 `Rc` / `Cell` / `RefCell` / `thread_local!`。
//! 两个后端同名同方法集：
//!
//! | 原语 | 单线程 + GIL（默认） | 并行（cargo feature `mt`，最终态） |
//! |---|---|---|
//! | `__Shared<T>` | `Rc<T>` | `Arc<T>` |
//! | `__PrimCell<T>` | `Cell<T>` | 原子单元（SeqCst，64 位位形；long/double 无撕裂） |
//! | `__RefSlot<T>` | `RefCell<T>` | 读写锁（`borrow` = 可重入读、`borrow_mut` = 写） |
//! | `__process_static!` | GIL 下全局单元 | 全局 `OnceLock` 单元 |
//! | `__ThreadSafe` | 空约束 | `Send + Sync` |
//! | `__DynFn!` | `dyn Fn(..) -> R` | `dyn Fn(..) -> R + Send + Sync` |
//!
//! 命名以 `__` 起始：对象模型的实现细节，不出现在可读层（方法体）。

/// 线程安全约束：`ObjectVTable` 与接口 vtable trait 的超 trait，使 `dyn` 对象在并行后端
/// 为 `Send + Sync`（单线程后端为空约束）。
#[cfg(not(feature = "mt"))]
pub trait __ThreadSafe {}
#[cfg(not(feature = "mt"))]
impl<T: ?Sized> __ThreadSafe for T {}
#[cfg(feature = "mt")]
pub trait __ThreadSafe: Send + Sync {}
#[cfg(feature = "mt")]
impl<T: ?Sized + Send + Sync> __ThreadSafe for T {}

/// 闭包对象类型（lambda / 方法引用载体、登记表回调）：`__DynFn!((A, B) -> R)`。
#[cfg(not(feature = "mt"))]
#[macro_export]
#[doc(hidden)]
macro_rules! __DynFn {
    ($($t:tt)*) => { dyn Fn $($t)* };
}
#[cfg(feature = "mt")]
#[macro_export]
#[doc(hidden)]
macro_rules! __DynFn {
    ($($t:tt)*) => { dyn Fn $($t)* + Send + Sync };
}

// ── 单线程 + GIL 后端 ──────────────────────────────────────────────────────────

/// 对象 / 字段单元的共享所有权。
#[cfg(not(feature = "mt"))]
pub type __Shared<T> = std::rc::Rc<T>;
/// 基本类型字段单元。
#[cfg(not(feature = "mt"))]
pub type __PrimCell<T> = std::cell::Cell<T>;
/// 引用字段 / 可变槽。
#[cfg(not(feature = "mt"))]
pub type __RefSlot<T> = std::cell::RefCell<T>;

// ── 并行后端（最终态）──────────────────────────────────────────────────────────

#[cfg(feature = "mt")]
pub type __Shared<T> = std::sync::Arc<T>;

/// 对象存储的类型擦除句柄（wrapper 的 `any` 字段、擦除视图导出）：单线程 `Rc<dyn Any>`，
/// 并行 `Arc<dyn Any + Send + Sync>`（`downcast` 同名可用）。
#[cfg(not(feature = "mt"))]
pub type __AnyRef = std::rc::Rc<dyn std::any::Any>;
#[cfg(feature = "mt")]
pub type __AnyRef = std::sync::Arc<dyn std::any::Any + Send + Sync>;
#[cfg(feature = "mt")]
pub use self::mt::{__AtomicRepr, __PrimCell, __RefSlot};

#[cfg(feature = "mt")]
mod mt {
    use std::marker::PhantomData;
    use std::sync::atomic::{AtomicU64, Ordering::SeqCst};

    /// 可放入原子单元的基本类型：与 u64 位形互转（JVM 基本类型 + 运行时计数用整型）。
    pub trait __AtomicRepr: Copy {
        fn __to_bits(self) -> u64;
        fn __from_bits(b: u64) -> Self;
    }
    macro_rules! int_repr {
        ($($t:ty),*) => {$(
            impl __AtomicRepr for $t {
                #[inline] fn __to_bits(self) -> u64 { self as u64 }
                #[inline] fn __from_bits(b: u64) -> Self { b as $t }
            }
        )*};
    }
    int_repr!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
    impl __AtomicRepr for bool {
        #[inline] fn __to_bits(self) -> u64 { self as u64 }
        #[inline] fn __from_bits(b: u64) -> Self { b != 0 }
    }
    impl __AtomicRepr for f32 {
        #[inline] fn __to_bits(self) -> u64 { self.to_bits() as u64 }
        #[inline] fn __from_bits(b: u64) -> Self { f32::from_bits(b as u32) }
    }
    impl __AtomicRepr for f64 {
        #[inline] fn __to_bits(self) -> u64 { self.to_bits() }
        #[inline] fn __from_bits(b: u64) -> Self { f64::from_bits(b) }
    }
    impl __AtomicRepr for char {
        #[inline] fn __to_bits(self) -> u64 { self as u64 }
        #[inline] fn __from_bits(b: u64) -> Self { char::from_u32(b as u32).unwrap_or('\0') }
    }

    /// 基本类型字段单元：`Cell` 同名方法集，全部 SeqCst（volatile 语义，强于普通字段要求；
    /// long / double 64 位原子读写，无 JLS §17.7 撕裂）。
    pub struct __PrimCell<T: __AtomicRepr> {
        bits: AtomicU64,
        _t: PhantomData<T>,
    }

    impl<T: __AtomicRepr> __PrimCell<T> {
        #[inline]
        pub fn new(v: T) -> Self { __PrimCell { bits: AtomicU64::new(v.__to_bits()), _t: PhantomData } }
        #[inline]
        pub fn get(&self) -> T { T::__from_bits(self.bits.load(SeqCst)) }
        #[inline]
        pub fn set(&self, v: T) { self.bits.store(v.__to_bits(), SeqCst) }
        #[inline]
        pub fn replace(&self, v: T) -> T { T::__from_bits(self.bits.swap(v.__to_bits(), SeqCst)) }
        #[inline]
        pub fn into_inner(self) -> T { T::__from_bits(self.bits.into_inner()) }
        #[inline]
        pub fn take(&self) -> T where T: Default { self.replace(T::default()) }
        /// 原子比较交换（按位形比较：浮点的 NaN / ±0 与 Unsafe.compareAndSet 的位比较一致）。
        #[inline]
        pub fn __cas(&self, expected: T, new: T) -> bool {
            self.bits.compare_exchange(expected.__to_bits(), new.__to_bits(), SeqCst, SeqCst).is_ok()
        }
        /// 原子读-改-写，返回旧值（getAndAdd / getAndBitwise* 族）。
        #[inline]
        pub fn __fetch_update(&self, f: impl Fn(T) -> T) -> T {
            let mut cur = self.bits.load(SeqCst);
            loop {
                let new = f(T::__from_bits(cur)).__to_bits();
                match self.bits.compare_exchange_weak(cur, new, SeqCst, SeqCst) {
                    Ok(old) => return T::__from_bits(old),
                    Err(actual) => cur = actual,
                }
            }
        }
    }
    impl<T: __AtomicRepr + Default> Default for __PrimCell<T> {
        fn default() -> Self { Self::new(T::default()) }
    }
    impl<T: __AtomicRepr> Clone for __PrimCell<T> {
        fn clone(&self) -> Self { Self::new(self.get()) }
    }
    impl<T: __AtomicRepr + PartialEq> PartialEq for __PrimCell<T> {
        fn eq(&self, o: &Self) -> bool { self.get() == o.get() }
    }
    impl<T: __AtomicRepr + std::fmt::Debug> std::fmt::Debug for __PrimCell<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("Cell").field("value", &self.get()).finish()
        }
    }

    /// 引用字段 / 可变槽：`RefCell` 同名方法集的读写锁。`borrow` 为可重入读（同线程嵌套读
    /// 不死锁），`borrow_mut` 为写。同线程读后写与 `RefCell` 的 panic 同属违例形态。
    pub struct __RefSlot<T> {
        lock: parking_lot::RwLock<T>,
    }

    impl<T> __RefSlot<T> {
        #[inline]
        pub const fn new(v: T) -> Self { __RefSlot { lock: parking_lot::RwLock::new(v) } }
        #[inline]
        pub fn borrow(&self) -> parking_lot::RwLockReadGuard<'_, T> { self.lock.read_recursive() }
        #[inline]
        pub fn borrow_mut(&self) -> parking_lot::RwLockWriteGuard<'_, T> { self.lock.write() }
        #[inline]
        pub fn try_borrow(&self) -> Result<parking_lot::RwLockReadGuard<'_, T>, ()> {
            self.lock.try_read_recursive().ok_or(())
        }
        #[inline]
        pub fn try_borrow_mut(&self) -> Result<parking_lot::RwLockWriteGuard<'_, T>, ()> {
            self.lock.try_write().ok_or(())
        }
        #[inline]
        pub fn replace(&self, v: T) -> T { std::mem::replace(&mut *self.lock.write(), v) }
        #[inline]
        pub fn take(&self) -> T where T: Default { std::mem::take(&mut *self.lock.write()) }
        #[inline]
        pub fn into_inner(self) -> T { self.lock.into_inner() }
        #[inline]
        pub fn get_mut(&mut self) -> &mut T { self.lock.get_mut() }
    }
    impl<T: Default> Default for __RefSlot<T> {
        fn default() -> Self { Self::new(T::default()) }
    }
    impl<T: Clone> Clone for __RefSlot<T> {
        fn clone(&self) -> Self { Self::new(self.borrow().clone()) }
    }
    impl<T: PartialEq> PartialEq for __RefSlot<T> {
        fn eq(&self, o: &Self) -> bool {
            if std::ptr::eq(self, o) {
                return true;
            }
            *self.borrow() == *o.borrow()
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for __RefSlot<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self.lock.try_read_recursive() {
                Some(v) => f.debug_struct("RefCell").field("value", &*v).finish(),
                None => f.write_str("RefCell { <locked> }"),
            }
        }
    }
}

// 单线程后端的原子读-改-写同名方法（GIL 下读-改-写之间无安全点，不可分割）。
#[cfg(not(feature = "mt"))]
pub trait __CellAtomicOps<T: Copy> {
    fn __cas(&self, expected: T, new: T) -> bool;
    fn __fetch_update(&self, f: impl Fn(T) -> T) -> T;
}
#[cfg(not(feature = "mt"))]
impl<T: Copy + __BitEq> __CellAtomicOps<T> for std::cell::Cell<T> {
    #[inline]
    fn __cas(&self, expected: T, new: T) -> bool {
        if self.get().__bit_eq(expected) {
            self.set(new);
            true
        } else {
            false
        }
    }
    #[inline]
    fn __fetch_update(&self, f: impl Fn(T) -> T) -> T {
        let old = self.get();
        self.set(f(old));
        old
    }
}
/// 按位相等（CAS 的比较语义：浮点按位形，与 Unsafe.compareAndSet 一致）。
#[cfg(not(feature = "mt"))]
pub trait __BitEq: Copy { fn __bit_eq(self, o: Self) -> bool; }
#[cfg(not(feature = "mt"))]
macro_rules! bit_eq_int { ($($t:ty),*) => {$( impl __BitEq for $t { #[inline] fn __bit_eq(self, o: Self) -> bool { self == o } } )*}; }
#[cfg(not(feature = "mt"))]
bit_eq_int!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize, bool, char);
#[cfg(not(feature = "mt"))]
impl __BitEq for f32 { #[inline] fn __bit_eq(self, o: Self) -> bool { self.to_bits() == o.to_bits() } }
#[cfg(not(feature = "mt"))]
impl __BitEq for f64 { #[inline] fn __bit_eq(self, o: Self) -> bool { self.to_bits() == o.to_bits() } }

/// 进程级存储：全进程一份，经 GIL 串行访问（`crate::gil` 模块注释）。
///
/// 语义为「全进程一份」：运行时登记表、驻留表、类的静态字段与初始化状态等。真正按线程
/// 区分的状态（当前线程、GIL 持有、InternalLock 守卫、拆箱失败标记）直写 `thread_local!`。
///
/// 语法与 `thread_local!` 相同（`static NAME: T = const { e };` / `= e;`，可带属性与可见性），
/// 访问面同 `LocalKey`：`with`，以及 `Cell` / `RefCell` 单元的 `get` / `set` / `take` /
/// `replace` / `with_borrow` / `with_borrow_mut`。
#[macro_export]
#[doc(hidden)]
macro_rules! __process_static {
    () => {};
    ($(#[$attr:meta])* $vis:vis static $name:ident : $t:ty = const { $init:expr } $(; $($rest:tt)*)?) => {
        $(#[$attr])* $vis static $name: $crate::sync_model::__GilStatic<$t> =
            $crate::sync_model::__GilStatic::new(|| $init);
        $($crate::__process_static!($($rest)*);)?
    };
    ($(#[$attr:meta])* $vis:vis static $name:ident : $t:ty = $init:expr $(; $($rest:tt)*)?) => {
        $(#[$attr])* $vis static $name: $crate::sync_model::__GilStatic<$t> =
            $crate::sync_model::__GilStatic::new(|| $init);
        $($crate::__process_static!($($rest)*);)?
    };
}

/// `__process_static!` 的存储单元：首次访问时惰性初始化的全局量。
///
/// 单线程 + GIL 后端：Java 代码与运行时手写层只在持有 GIL 时执行（GIL 启用前进程只有
/// 主线程），任一时刻至多一个线程访问单元内容，`Sync` 由此成立。并行后端：`OnceLock`
/// 惰性初始化，内容自身为线程安全单元（原子 / 读写锁），`Sync` 由类型系统保证。
pub struct __GilStatic<T> {
    init: fn() -> T,
    #[cfg(not(feature = "mt"))]
    cell: std::cell::OnceCell<T>,
    #[cfg(feature = "mt")]
    cell: std::sync::OnceLock<T>,
}

#[cfg(not(feature = "mt"))]
unsafe impl<T> Sync for __GilStatic<T> {}

impl<T> __GilStatic<T> {
    pub const fn new(init: fn() -> T) -> Self {
        #[cfg(not(feature = "mt"))]
        return __GilStatic { init, cell: std::cell::OnceCell::new() };
        #[cfg(feature = "mt")]
        return __GilStatic { init, cell: std::sync::OnceLock::new() };
    }

    #[inline]
    pub fn with<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        f(self.cell.get_or_init(self.init))
    }
}

#[cfg(not(feature = "mt"))]
impl<T: Copy> __GilStatic<__PrimCell<T>> {
    #[inline]
    pub fn get(&'static self) -> T { self.with(|c| c.get()) }
    #[inline]
    pub fn set(&'static self, v: T) { self.with(|c| c.set(v)) }
    #[inline]
    pub fn replace(&'static self, v: T) -> T { self.with(|c| c.replace(v)) }
}
#[cfg(feature = "mt")]
impl<T: __AtomicRepr> __GilStatic<__PrimCell<T>> {
    #[inline]
    pub fn get(&'static self) -> T { self.with(|c| c.get()) }
    #[inline]
    pub fn set(&'static self, v: T) { self.with(|c| c.set(v)) }
    #[inline]
    pub fn replace(&'static self, v: T) -> T { self.with(|c| c.replace(v)) }
}

impl<T> __GilStatic<__RefSlot<T>> {
    #[inline]
    pub fn with_borrow<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        self.with(|c| f(&c.borrow()))
    }
    #[inline]
    pub fn with_borrow_mut<R>(&'static self, f: impl FnOnce(&mut T) -> R) -> R {
        self.with(|c| f(&mut c.borrow_mut()))
    }
    #[inline]
    pub fn set(&'static self, v: T) { self.with(|c| *c.borrow_mut() = v) }
    #[inline]
    pub fn replace(&'static self, v: T) -> T { self.with(|c| c.replace(v)) }
    #[inline]
    pub fn take(&'static self) -> T where T: Default { self.with(|c| c.take()) }
}

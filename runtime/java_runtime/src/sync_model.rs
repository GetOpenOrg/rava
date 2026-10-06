//! 同步模型原语（#42 真多线程，docs/plans/2026-09-26-real-multithreading.md）。
//!
//! 对象模型的共享、字段单元、引用槽、进程级存储四种原语在此集中定义；java_class! 宏展开、
//! 生成代码与手写层一律经这些名字引用，不直接写 `Rc` / `Cell` / `RefCell` / `thread_local!`。
//! 并行后端（#42 最终态，默认且唯一；单线程 + GIL 后端已移除）：
//!
//! | 原语 | 实现 |
//! |---|---|
//! | `__Shared<T>` | `Arc<T>` |
//! | `__PrimCell<T>` | 原子单元（SeqCst，64 位位形；long/double 无撕裂） |
//! | `__RefSlot<T>` | 读写锁（`borrow` = 可重入读、`borrow_mut` = 写） |
//! | `__RefField<T>` | 引用字段内联单元：字节自旋锁 + 值（临界区只做克隆 / 交换） |
//! | `__process_static!` | 全局 `OnceLock` 单元 |
//! | `__ThreadSafe` | `Send + Sync` |
//! | `__DynFn!` | `dyn Fn(..) -> R + Send + Sync` |
//!
//! 命名以 `__` 起始：对象模型的实现细节，不出现在可读层（方法体）。

/// 线程安全约束：`ObjectVTable` 与接口 vtable trait 的超 trait，使 `dyn` 对象为 `Send + Sync`。
pub trait __ThreadSafe: Send + Sync {}
impl<T: ?Sized + Send + Sync> __ThreadSafe for T {}

/// 闭包对象类型（lambda / 方法引用载体、登记表回调）：`__DynFn!((A, B) -> R)`。
#[macro_export]
#[doc(hidden)]
macro_rules! __DynFn {
    ($($t:tt)*) => { dyn Fn $($t)* + Send + Sync };
}

/// 对象 / 字段单元的共享所有权。
pub type __Shared<T> = std::sync::Arc<T>;

/// 对象存储的类型擦除句柄（wrapper 的 `any` 字段、擦除视图导出）：`Arc<dyn Any + Send + Sync>`
/// （`downcast` 同名可用）。
pub type __AnyRef = std::sync::Arc<dyn std::any::Any + Send + Sync>;

/// 不承载存储的擦除句柄（`__view_into` 等只透传、不读取 `any` 的探针调用）：进程内共用一个单元，
/// 调用点只增引用计数，不逐次分配。
pub fn __unused_any() -> __AnyRef {
    static UNUSED: std::sync::OnceLock<__AnyRef> = std::sync::OnceLock::new();
    UNUSED.get_or_init(|| std::sync::Arc::new(())).clone()
}
pub use self::mt::{__AtomicRepr, __PrimCell, __RefField, __RefSlot};

mod mt {
    use std::cell::UnsafeCell;
    use std::marker::PhantomData;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::{Acquire, Relaxed, Release, SeqCst}};

    /// 可放入原子单元的基本类型：与 u64 位形互转（JVM 基本类型 + 运行时计数用整型）。
    pub trait __AtomicRepr: Copy {
        fn __to_bits(self) -> u64;
        fn __from_bits(b: u64) -> Self;
    }
    macro_rules! int_repr {
        ($($t:ty),*) => {$(
            impl __AtomicRepr for $t {
                #[inline(always)] fn __to_bits(self) -> u64 { self as u64 }
                #[inline(always)] fn __from_bits(b: u64) -> Self { b as $t }
            }
        )*};
    }
    int_repr!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);
    impl __AtomicRepr for bool {
        #[inline(always)] fn __to_bits(self) -> u64 { self as u64 }
        #[inline(always)] fn __from_bits(b: u64) -> Self { b != 0 }
    }
    impl __AtomicRepr for f32 {
        #[inline(always)] fn __to_bits(self) -> u64 { self.to_bits() as u64 }
        #[inline(always)] fn __from_bits(b: u64) -> Self { f32::from_bits(b as u32) }
    }
    impl __AtomicRepr for f64 {
        #[inline(always)] fn __to_bits(self) -> u64 { self.to_bits() }
        #[inline(always)] fn __from_bits(b: u64) -> Self { f64::from_bits(b) }
    }
    impl __AtomicRepr for char {
        #[inline(always)] fn __to_bits(self) -> u64 { self as u64 }
        #[inline(always)] fn __from_bits(b: u64) -> Self { char::from_u32(b as u32).unwrap_or('\0') }
    }

    /// 基本类型字段单元：`Cell` 同名方法集为 SeqCst（volatile 语义；long / double 64 位原子读写，
    /// 无 JLS §17.7 撕裂）；`get_plain` / `set_plain` 为 relaxed（普通字段：JMM 不要求互斥与
    /// 顺序，同步动作的获取 / 释放序给出 happens-before，无数据竞争 UB）。`repr(transparent)`：
    /// 各实例化布局相同，描述符驱动的浅拷贝按位拷贝任意基本单元（`field_desc`）。
    #[repr(transparent)]
    pub struct __PrimCell<T: __AtomicRepr> {
        bits: AtomicU64,
        _t: PhantomData<T>,
    }

    impl<T: __AtomicRepr> __PrimCell<T> {
        #[inline]
        pub fn new(v: T) -> Self { __PrimCell { bits: AtomicU64::new(v.__to_bits()), _t: PhantomData } }
        /// 全零位形（各基本类型的 JVM 缺省值）：常量求值可用（静态字段单元）
        #[inline]
        pub const fn zeroed() -> Self { __PrimCell { bits: AtomicU64::new(0), _t: PhantomData } }
        /// 给定位形（`__to_bits` 的结果）：常量求值可用（引导映像的字段与静态字段初值）
        #[inline]
        pub const fn from_bits(bits: u64) -> Self { __PrimCell { bits: AtomicU64::new(bits), _t: PhantomData } }
        #[inline(always)]
        pub fn get(&self) -> T { T::__from_bits(self.bits.load(SeqCst)) }
        /// 普通（非 volatile）字段读：relaxed
        #[inline(always)]
        pub fn get_plain(&self) -> T { T::__from_bits(self.bits.load(Relaxed)) }
        /// 普通（非 volatile）字段写：relaxed
        #[inline]
        pub fn set_plain(&self, v: T) { self.bits.store(v.__to_bits(), Relaxed) }
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
    // Unsafe int 字视图（`ObjectVTable::__unsafe_word`）：int、float 及子字（boolean / byte / short /
    // char）字段各占独享的 4 字节对齐槽（objectFieldOffset 的 id 恒为 4 的倍数），字段值位于槽的低位
    // （小端，JDK compareAndExchangeByte / Short 的 `offset & ~3` 与 shift 落在槽内且 shift = 0），
    // 其余位是恒为 0 的填充。to_word：字段值零扩展为字（JVM 字段内存形态）；from_word：取字的
    // 低位截断回字段（boolean 取低字节非 0，与 HotSpot 的 boolean 规范化一致）。
    macro_rules! word_view {
        ($method:ident: $w_ty:ty; $($t:ty => |$v:ident| $to:expr, |$w:ident| $from:expr;)*) => {$(
            impl __PrimCell<$t> {
                /// 字视图读-改-写：`op(旧字)` 给出新字则原子写入其低位截断，返回旧字；
                /// 给 None 即只读。CAS 重试时 op 重新求值（op 须为纯函数）。
                pub fn $method(&self, op: &mut dyn FnMut($w_ty) -> Option<$w_ty>) -> $w_ty {
                    let to = |$v: $t| -> $w_ty { $to };
                    let from = |$w: $w_ty| -> $t { $from };
                    let mut cur = self.bits.load(SeqCst);
                    loop {
                        let old = to(<$t as __AtomicRepr>::__from_bits(cur));
                        let Some(w) = op(old) else { return old };
                        match self.bits.compare_exchange_weak(cur, from(w).__to_bits(), SeqCst, SeqCst) {
                            Ok(_) => return old,
                            Err(actual) => cur = actual,
                        }
                    }
                }
            }
        )*};
    }
    // float 字段的字形态是其原始位（JDK compareAndSetFloat → compareAndSetInt(floatToRawIntBits)）
    word_view! { __word_update: i32;
        i32 => |v| v, |w| w;
        bool => |v| v as i32, |w| (w & 0xFF) != 0;
        i8 => |v| v as u8 as i32, |w| w as i8;
        i16 => |v| v as u16 as i32, |w| w as i16;
        u16 => |v| v as i32, |w| w as u16;
        f32 => |v| v.to_bits() as i32, |w| f32::from_bits(w as u32);
    }
    // 双字视图（`ObjectVTable::__unsafe_dword`）：long 与 double（原始位，JDK compareAndSetDouble →
    // compareAndSetLong(doubleToRawLongBits)）字段
    word_view! { __dword_update: i64;
        i64 => |v| v, |w| w;
        f64 => |v| v.to_bits() as i64, |w| f64::from_bits(w as u64);
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

    /// 引用字段内联单元：字节自旋锁 + 值，与对象存储同一分配（不再每字段一个 `Arc` + 读写锁）。
    ///
    /// 临界区只做值的克隆（引用计数加一）或交换，不执行 Java 代码、不让出，旧值在锁外释放；
    /// 不对外暴露守卫，同线程不会重入。读写都经获取 / 释放序，引用发布随之携带被引对象的
    /// 构造写入（final 字段语义）。`const fn new`：静态字段单元可常量初始化。
    pub struct __RefField<T> {
        locked: AtomicBool,
        val: UnsafeCell<T>,
    }

    // 值只在锁内访问
    unsafe impl<T: Send> Send for __RefField<T> {}
    unsafe impl<T: Send + Sync> Sync for __RefField<T> {}

    struct FieldGuard<'a>(&'a AtomicBool);
    impl Drop for FieldGuard<'_> {
        #[inline(always)]
        fn drop(&mut self) { self.0.store(false, Release) }
    }

    impl<T> __RefField<T> {
        #[inline]
        pub const fn new(v: T) -> Self { __RefField { locked: AtomicBool::new(false), val: UnsafeCell::new(v) } }

        #[inline(always)]
        fn lock(&self) -> FieldGuard<'_> {
            if self.locked.compare_exchange_weak(false, true, Acquire, Relaxed).is_err() {
                self.lock_slow();
            }
            FieldGuard(&self.locked)
        }

        #[cold]
        #[inline(never)]
        fn lock_slow(&self) {
            let mut spins = 0u32;
            loop {
                while self.locked.load(Relaxed) {
                    if spins < 64 {
                        spins += 1;
                        std::hint::spin_loop();
                    } else {
                        std::thread::yield_now();
                    }
                }
                if self.locked.compare_exchange_weak(false, true, Acquire, Relaxed).is_ok() {
                    return;
                }
            }
        }

        /// 锁内对值执行 `f`（`f` 不得访问同一单元）。
        #[inline]
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let _g = self.lock();
            // SAFETY: 持锁独占
            f(unsafe { &mut *self.val.get() })
        }

        /// 读：锁内克隆。不经 `with` 的闭包：读路径（静态 / 实例引用字段 getter、引用数组元素）
        /// 标 `#[inline(always)]`，opt-level 0 的调用方 crate 里只剩加锁 CAS、克隆与解锁。
        #[inline(always)]
        pub fn get(&self) -> T where T: Clone {
            let _g = self.lock();
            // SAFETY: 持锁独占
            unsafe { (*self.val.get()).clone() }
        }

        /// 写：锁内交换，旧值在锁外释放。
        #[inline]
        pub fn set(&self, v: T) {
            drop(self.replace(v));
        }

        #[inline]
        pub fn replace(&self, v: T) -> T {
            self.with(|cur| std::mem::replace(cur, v))
        }

        #[inline]
        pub fn take(&self) -> T where T: Default {
            self.replace(T::default())
        }

        #[inline]
        pub fn into_inner(self) -> T { self.val.into_inner() }

        #[inline]
        pub fn get_mut(&mut self) -> &mut T { self.val.get_mut() }
    }

    impl<T> __RefField<Option<T>> {
        /// 引用字段读（存储 `None` = 从未写入，按声明类型的缺省值应答）。
        #[inline(always)]
        pub fn get_or_default(&self) -> T where T: Clone + Default {
            let v = {
                let _g = self.lock();
                // SAFETY: 持锁独占
                match unsafe { &*self.val.get() } {
                    Some(v) => Some(v.clone()),
                    None => None,
                }
            };
            match v {
                Some(v) => v,
                None => T::default(),
            }
        }
    }

    impl<T: Default> Default for __RefField<T> {
        fn default() -> Self { Self::new(T::default()) }
    }
    // 不在锁内格式化值（值的 Debug 可能执行 Java toString，回到本单元）
    impl<T> std::fmt::Debug for __RefField<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("RefField { .. }")
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

/// 进程级存储：全进程一份，内容为线程安全单元（原子 / 读写锁）。
///
/// 语义为「全进程一份」：运行时登记表、驻留表、类的静态字段与初始化状态等。随执行流走的状态放
/// 执行上下文块（`exec_context::ExecState`）；OS 线程局部存储只承载载体槽（`thread_impl.rs`）。
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
/// `OnceLock` 惰性初始化，内容自身为线程安全单元（原子 / 读写锁），`Sync` 由类型系统保证。
pub struct __GilStatic<T> {
    init: fn() -> T,
    cell: std::sync::OnceLock<T>,
}

impl<T> __GilStatic<T> {
    pub const fn new(init: fn() -> T) -> Self {
        __GilStatic { init, cell: std::sync::OnceLock::new() }
    }

    /// 取（必要时惰性初始化）存储单元。只随 `T` 实例化——宏展开的静态字段 / 类初始化
    /// 状态访问经此直取单元，不为每个访问点的闭包各实例化一份 `with`。
    #[inline]
    pub fn force(&'static self) -> &'static T {
        self.cell.get_or_init(self.init)
    }

    #[inline]
    pub fn with<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        f(self.force())
    }
}

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

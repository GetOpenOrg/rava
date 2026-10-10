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
//! | `__RefField<T>` | 引用字段内联单元：字节自旋锁 + 值（临界区只做克隆 / 交换；普通族 Acquire / Release，volatile 族 SeqCst） |
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
pub use self::mt::{__AtomicRepr, __PrimCell, __RefField, __RefSlot, __SlotRead, __SlotWrite};

/// 持锁登记（无 GC 文档第四节小步 A）：当前线程持有的字段锁——`__RefField::with` / `with_mut` 的
/// 临界区与 `__RefSlot` 的读写守卫——计数，`__RefSlot` 另记槽位地址（自持有检查）。
/// 只在 debug 档生效：release 档 `Held` 是无 `Drop` 的零大小值，登记与断言整体消去。
mod held {
    #[cfg(debug_assertions)]
    use std::cell::{Cell, RefCell};

    /// 槽位登记容量：嵌套持有超过此数的槽位不再登记地址（计数照常），只漏检自持有
    #[cfg(debug_assertions)]
    const SLOTS: usize = 16;

    /// 一项槽位持有：(槽地址, 是否写锁, 加锁位置)；地址 0 为空位
    #[cfg(debug_assertions)]
    type SlotHold = (usize, bool, Option<&'static std::panic::Location<'static>>);

    #[cfg(debug_assertions)]
    std::thread_local! {
        static COUNT: Cell<usize> = const { Cell::new(0) };
        static HELD_SLOTS: RefCell<[SlotHold; SLOTS]> = const { RefCell::new([(0, false, None); SLOTS]) };
    }

    /// 持锁凭据：存活期间计入当前线程的持锁数（随守卫释放，含 panic 展开路径）
    pub(crate) struct Held {
        /// 登记的槽位地址（0 = 未登记：`__RefField` 临界区或登记表已满）
        #[cfg(debug_assertions)]
        slot: usize,
    }

    impl Held {
        /// `__RefField` 临界区
        #[inline(always)]
        pub(crate) fn field() -> Held {
            #[cfg(debug_assertions)]
            {
                let _ = COUNT.try_with(|c| c.set(c.get() + 1));
                Held { slot: 0 }
            }
            #[cfg(not(debug_assertions))]
            Held {}
        }

        /// `__RefSlot` 守卫（加锁之前调用）：`write` 时本线程已持有该槽（读或写）、读时本线程已持有
        /// 该槽的写锁即 panic 报两处加锁位置（读写锁不可重入升级，继续加锁即自死锁）
        #[inline(always)]
        #[cfg_attr(debug_assertions, track_caller)]
        pub(crate) fn slot(addr: usize, write: bool) -> Held {
            #[cfg(debug_assertions)]
            {
                let here = std::panic::Location::caller();
                let prior = HELD_SLOTS
                    .try_with(|s| s.borrow().iter().find(|h| h.0 == addr && (write || h.1)).copied())
                    .ok()
                    .flatten();
                if let Some((_, prior_write, at)) = prior {
                    panic!("字段锁自持有：槽位 {addr:#x} 在 {here} 加{}锁，本线程已于 {} 持有其{}锁",
                        if write { "写" } else { "读" },
                        at.map_or_else(|| "?".to_owned(), |l| l.to_string()),
                        if prior_write { "写" } else { "读" });
                }
                Self::record(addr, write, here)
            }
            #[cfg(not(debug_assertions))]
            {
                let _ = (addr, write);
                Held {}
            }
        }

        /// `try_borrow*` 成功后登记（不阻塞，不需自持有检查）
        #[inline(always)]
        #[cfg_attr(debug_assertions, track_caller)]
        pub(crate) fn slot_acquired(addr: usize, write: bool) -> Held {
            #[cfg(debug_assertions)]
            {
                Self::record(addr, write, std::panic::Location::caller())
            }
            #[cfg(not(debug_assertions))]
            {
                let _ = (addr, write);
                Held {}
            }
        }

        #[cfg(debug_assertions)]
        fn record(addr: usize, write: bool, at: &'static std::panic::Location<'static>) -> Held {
            let _ = COUNT.try_with(|c| c.set(c.get() + 1));
            let slot = HELD_SLOTS
                .try_with(|s| {
                    let mut s = s.borrow_mut();
                    s.iter_mut().find(|h| h.0 == 0).map(|h| {
                        *h = (addr, write, Some(at));
                        addr
                    })
                })
                .ok()
                .flatten()
                .unwrap_or(0);
            Held { slot }
        }
    }

    #[cfg(debug_assertions)]
    impl Drop for Held {
        fn drop(&mut self) {
            let _ = COUNT.try_with(|c| c.set(c.get().saturating_sub(1)));
            if self.slot != 0 {
                let slot = self.slot;
                let _ = HELD_SLOTS.try_with(|s| {
                    // 同槽多次读持有：去掉任一项即可（登记只用于判定「是否持有」）
                    if let Some(h) = s.borrow_mut().iter_mut().rev().find(|h| h.0 == slot) {
                        *h = (0, false, None);
                    }
                });
            }
        }
    }

    /// 当前线程持有的字段锁数（debug 档）
    #[cfg(debug_assertions)]
    pub(crate) fn count() -> usize {
        COUNT.try_with(Cell::get).unwrap_or(0)
    }
}

/// debug 档断言：当前线程不持有任何字段锁（`what` 为断言点说明）。release 档为空。
///
/// 断言点：对象释放（`drop_slow`：锁内释放对象会重入任意析构链）、安全点、类初始化入口。
#[inline(always)]
#[cfg_attr(debug_assertions, track_caller)]
pub fn __assert_no_field_lock(what: &str) {
    #[cfg(debug_assertions)]
    {
        let n = held::count();
        if n != 0 {
            panic!("{what}时持有 {n} 把字段锁（__RefField 临界区 / __RefSlot 守卫）");
        }
    }
    #[cfg(not(debug_assertions))]
    let _ = what;
}

mod mt {
    use std::cell::UnsafeCell;
    use std::marker::PhantomData;
    use std::sync::atomic::{AtomicBool, AtomicU64, Ordering::{Acquire, Relaxed, Release, SeqCst}};

    use super::held::Held;

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
    /// 不对外暴露守卫，同线程不会重入。`const fn new`：静态字段单元可常量初始化。
    ///
    /// 内存序按访问方式分两族（同一存储，由访问器选择，与 `__PrimCell` 的 `get` / `get_plain` 同构）：
    /// - 普通族（`get` / `set` / `with` / `with_mut` …）：加锁 CAS 取 Acquire、解锁 store 取 Release。
    ///   普通引用字段与 `*aload` / `*astore`；引用发布随之携带被引对象的构造写入（final 字段语义）。
    /// - volatile 族（`*_volatile`）：加锁 CAS 与解锁 store **都取 SeqCst**，每次访问的两条锁字
    ///   操作都进入 SeqCst 全序 S。volatile 字段（字节码 ACC_VOLATILE，宏按字段属性分流）的
    ///   getfield / putfield / getstatic / putstatic、Unsafe / VarHandle 的引用读-改-写走这一族。
    ///   论证见无 GC 文档（docs/plans/2026-10-07-no-gc-memory-model.md）第五节「小步 B」。
    pub struct __RefField<T> {
        locked: AtomicBool,
        val: UnsafeCell<T>,
    }

    // 值只在锁内访问
    unsafe impl<T: Send> Send for __RefField<T> {}
    unsafe impl<T: Send + Sync> Sync for __RefField<T> {}

    /// 加锁凭据：`SC` 选解锁 store 的序（SeqCst / Release），与加锁 CAS 的序成对
    struct FieldGuard<'a, const SC: bool>(&'a AtomicBool);
    impl<const SC: bool> Drop for FieldGuard<'_, SC> {
        #[inline(always)]
        fn drop(&mut self) { self.0.store(false, if SC { SeqCst } else { Release }) }
    }

    /// 加锁 CAS 成功序：volatile 族 SeqCst，普通族 Acquire（失败序恒 Relaxed：失败不建立任何序）
    #[inline(always)]
    const fn lock_order(sc: bool) -> std::sync::atomic::Ordering {
        if sc { SeqCst } else { Acquire }
    }

    impl<T> __RefField<T> {
        #[inline]
        pub const fn new(v: T) -> Self { __RefField { locked: AtomicBool::new(false), val: UnsafeCell::new(v) } }

        #[inline(always)]
        fn lock<const SC: bool>(&self) -> FieldGuard<'_, SC> {
            if self.locked.compare_exchange_weak(false, true, lock_order(SC), Relaxed).is_err() {
                self.lock_slow::<SC>();
            }
            FieldGuard(&self.locked)
        }

        #[cold]
        #[inline(never)]
        fn lock_slow<const SC: bool>(&self) {
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
                if self.locked.compare_exchange_weak(false, true, lock_order(SC), Relaxed).is_ok() {
                    return;
                }
            }
        }

        /// 锁内只读访问值（`f` 不得访问同一单元、不得执行 Java 代码）。
        #[inline]
        pub fn with<R>(&self, f: impl FnOnce(&T) -> R) -> R {
            let _g = self.lock::<false>();
            let _held = Held::field();
            // SAFETY: 持锁独占
            f(unsafe { &*self.val.get() })
        }

        /// 锁内改写值（`f` 不得访问同一单元、不得执行 Java 代码）。换下的旧值须从 `f` 返回、
        /// 放锁后再释放：锁内释放对象会重入任意析构链（debug 档 `drop_slow` 断言）。
        #[inline]
        pub fn with_mut<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            self.with_mut_ordered::<false, R>(f)
        }

        /// `with_mut` 的 volatile 族（加锁 / 解锁都 SeqCst）：Unsafe / VarHandle 的引用读-改-写
        /// （compareAndSet / compareAndExchange / getAndSet，volatile 访问模式）。
        #[inline]
        pub fn with_mut_volatile<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            self.with_mut_ordered::<true, R>(f)
        }

        #[inline(always)]
        fn with_mut_ordered<const SC: bool, R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            let _g = self.lock::<SC>();
            let _held = Held::field();
            // SAFETY: 持锁独占
            f(unsafe { &mut *self.val.get() })
        }

        /// 锁内克隆（`SC` 选加锁 / 解锁的序）
        #[inline(always)]
        fn get_ordered<const SC: bool>(&self) -> T where T: Clone {
            let _g = self.lock::<SC>();
            // SAFETY: 持锁独占
            unsafe { (*self.val.get()).clone() }
        }

        /// 读：锁内克隆。不经 `with` 的闭包：读路径（静态 / 实例引用字段 getter、引用数组元素）
        /// 标 `#[inline(always)]`，opt-level 0 的调用方 crate 里只剩加锁 CAS、克隆与解锁。
        #[inline(always)]
        pub fn get(&self) -> T where T: Clone {
            self.get_ordered::<false>()
        }

        /// volatile 读（加锁 / 解锁都 SeqCst）
        #[inline(always)]
        pub fn get_volatile(&self) -> T where T: Clone {
            self.get_ordered::<true>()
        }

        /// 写：锁内交换，旧值在锁外释放。
        #[inline]
        pub fn set(&self, v: T) {
            drop(self.replace(v));
        }

        /// volatile 写（加锁 / 解锁都 SeqCst），旧值在锁外释放。
        #[inline]
        pub fn set_volatile(&self, v: T) {
            drop(self.replace_volatile(v));
        }

        #[inline]
        pub fn replace(&self, v: T) -> T {
            self.with_mut(|cur| std::mem::replace(cur, v))
        }

        #[inline]
        pub fn replace_volatile(&self, v: T) -> T {
            self.with_mut_volatile(|cur| std::mem::replace(cur, v))
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
            self.get_or_default_ordered::<false>()
        }

        /// volatile 引用字段读（加锁 / 解锁都 SeqCst）。
        #[inline(always)]
        pub fn get_or_default_volatile(&self) -> T where T: Clone + Default {
            self.get_or_default_ordered::<true>()
        }

        #[inline(always)]
        fn get_or_default_ordered<const SC: bool>(&self) -> T where T: Clone + Default {
            let v = {
                let _g = self.lock::<SC>();
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
    /// 不死锁），`borrow_mut` 为写。同线程读后写与 `RefCell` 的 panic 同属违例形态：debug 档
    /// 加锁前查本线程的持有登记，自持有即 panic 报位置（release 档直接自死锁）。
    pub struct __RefSlot<T> {
        lock: parking_lot::RwLock<T>,
    }

    /// `__RefSlot` 读守卫（debug 档随守卫登记持锁）
    pub struct __SlotRead<'a, T> {
        g: parking_lot::RwLockReadGuard<'a, T>,
        _held: Held,
    }

    /// `__RefSlot` 写守卫（debug 档随守卫登记持锁）
    pub struct __SlotWrite<'a, T> {
        g: parking_lot::RwLockWriteGuard<'a, T>,
        _held: Held,
    }

    impl<T> std::ops::Deref for __SlotRead<'_, T> {
        type Target = T;
        #[inline(always)]
        fn deref(&self) -> &T { &self.g }
    }
    impl<T> std::ops::Deref for __SlotWrite<'_, T> {
        type Target = T;
        #[inline(always)]
        fn deref(&self) -> &T { &self.g }
    }
    impl<T> std::ops::DerefMut for __SlotWrite<'_, T> {
        #[inline(always)]
        fn deref_mut(&mut self) -> &mut T { &mut self.g }
    }

    impl<T> __RefSlot<T> {
        #[inline]
        pub const fn new(v: T) -> Self { __RefSlot { lock: parking_lot::RwLock::new(v) } }
        #[inline(always)]
        fn addr(&self) -> usize { self as *const Self as usize }
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn borrow(&self) -> __SlotRead<'_, T> {
            let _held = Held::slot(self.addr(), false);
            __SlotRead { g: self.lock.read_recursive(), _held }
        }
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn borrow_mut(&self) -> __SlotWrite<'_, T> {
            let _held = Held::slot(self.addr(), true);
            __SlotWrite { g: self.lock.write(), _held }
        }
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn try_borrow(&self) -> Result<__SlotRead<'_, T>, ()> {
            let g = self.lock.try_read_recursive().ok_or(())?;
            Ok(__SlotRead { g, _held: Held::slot_acquired(self.addr(), false) })
        }
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn try_borrow_mut(&self) -> Result<__SlotWrite<'_, T>, ()> {
            let g = self.lock.try_write().ok_or(())?;
            Ok(__SlotWrite { g, _held: Held::slot_acquired(self.addr(), true) })
        }
        /// 锁内换值，旧值返回给调用方在锁外释放
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn replace(&self, v: T) -> T { std::mem::replace(&mut *self.borrow_mut(), v) }
        #[inline]
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn take(&self) -> T where T: Default { std::mem::take(&mut *self.borrow_mut()) }
        #[inline]
        pub fn into_inner(self) -> T { self.lock.into_inner() }
        #[inline]
        pub fn get_mut(&mut self) -> &mut T { self.lock.get_mut() }
    }

    // 登记表的写入形态（无 GC 文档第四节小步 A）：锁内只换出值，被替换 / 落选的值放锁后再释放
    // （锁内释放对象会重入任意析构链，debug 档 `drop_slow` 断言）；值的构造在锁外完成。

    impl<T: Clone> __RefSlot<Option<T>> {
        /// 槽为空时写入 `v`，返回槽中的值（并发首次写入：先写入者胜出，保持单一身份）。落选的 `v` 在锁外释放。
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn get_or_insert(&self, v: T) -> T {
            let (cur, unused) = {
                let mut g = self.borrow_mut();
                match &*g {
                    Some(cur) => (cur.clone(), Some(v)),
                    None => {
                        *g = Some(v.clone());
                        (v, None)
                    }
                }
            };
            drop(unused);
            cur
        }
    }

    impl<K: Eq + std::hash::Hash, V> __RefSlot<std::collections::HashMap<K, V>> {
        /// 写入 `k → v`；被覆盖的旧值在锁外释放。
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn put(&self, k: K, v: V) {
            let old = self.borrow_mut().insert(k, v);
            drop(old);
        }

        /// 按键取规范值：已有则取表中的值，否则写入 `v`（先写入者胜出，保持单一身份）。落选的 `v` 在锁外释放。
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn intern(&self, k: K, v: V) -> V where V: Clone {
            use std::collections::hash_map::Entry;
            let (cur, unused) = match self.borrow_mut().entry(k) {
                Entry::Occupied(e) => (e.get().clone(), Some(v)),
                Entry::Vacant(e) => (e.insert(v).clone(), None),
            };
            drop(unused);
            cur
        }
    }

    impl<T> __RefSlot<Vec<T>> {
        /// 摘除满足 `pred` 的元素；摘下的元素在锁外释放。`pred` 不得执行 Java 代码。
        #[cfg_attr(debug_assertions, track_caller)]
        pub fn remove_where(&self, pred: impl FnMut(&T) -> bool) {
            let mut pred = pred;
            let gone: Vec<T> = self.borrow_mut().extract_if(.., |x| pred(x)).collect();
            drop(gone);
        }
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
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn with_borrow<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        f(&self.force().borrow())
    }
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn with_borrow_mut<R>(&'static self, f: impl FnOnce(&mut T) -> R) -> R {
        f(&mut self.force().borrow_mut())
    }
    /// 写入；旧值在锁外释放
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn set(&'static self, v: T) { drop(self.force().replace(v)) }
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn replace(&'static self, v: T) -> T { self.force().replace(v) }
    #[inline]
    #[cfg_attr(debug_assertions, track_caller)]
    pub fn take(&'static self) -> T where T: Default { self.force().take() }
}

//! 同步模型原语（#42 真多线程，docs/plans/2026-09-26-real-multithreading.md §三 第 1 步）。
//!
//! 对象模型的共享、字段单元、引用槽三种原语在此集中定义；java_class! 宏展开与手写层
//! 一律经这些名字引用，不直接写 `Rc` / `Cell` / `RefCell`。当前为单线程后端（类型别名，
//! 与原实现完全等价）；多线程后端（cargo feature `mt`）将以同名、同方法集的
//! `Arc` / 原子单元 / 锁单元替换，宏与生成代码无需再改。
//!
//! 命名以 `__` 起始：对象模型的实现细节，不出现在可读层（方法体）。

/// 对象 / 字段单元的共享所有权（单线程：`Rc`；多线程：`Arc`）。
pub type __Shared<T> = std::rc::Rc<T>;

/// 基本类型字段单元（单线程：`Cell`；多线程：原子单元，同名 `get` / `set` / `replace`）。
pub type __PrimCell<T> = std::cell::Cell<T>;

/// 引用字段 / 可变槽（单线程：`RefCell`；多线程：锁单元，同名 `borrow` / `borrow_mut` / `replace`）。
pub type __RefSlot<T> = std::cell::RefCell<T>;

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
/// 安全性：Java 代码与运行时手写层只在持有 GIL 时执行（GIL 启用前进程只有主线程），
/// 因此任一时刻至多一个线程访问单元内容；`Sync` 由此成立。
pub struct __GilStatic<T> {
    init: fn() -> T,
    cell: std::cell::OnceCell<T>,
}

unsafe impl<T> Sync for __GilStatic<T> {}

impl<T> __GilStatic<T> {
    pub const fn new(init: fn() -> T) -> Self {
        __GilStatic { init, cell: std::cell::OnceCell::new() }
    }

    #[inline]
    pub fn with<R>(&'static self, f: impl FnOnce(&T) -> R) -> R {
        f(self.cell.get_or_init(self.init))
    }
}

impl<T: Copy> __GilStatic<std::cell::Cell<T>> {
    #[inline]
    pub fn get(&'static self) -> T { self.with(|c| c.get()) }
    #[inline]
    pub fn set(&'static self, v: T) { self.with(|c| c.set(v)) }
    #[inline]
    pub fn replace(&'static self, v: T) -> T { self.with(|c| c.replace(v)) }
}

impl<T> __GilStatic<std::cell::RefCell<T>> {
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

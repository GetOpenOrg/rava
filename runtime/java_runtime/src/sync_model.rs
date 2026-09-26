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

/// 进程级存储（单线程：`thread_local!`；多线程：全局 `OnceLock` 单元，同名 `with` 访问）。
///
/// 语义为「全进程一份」：运行时登记表、类的静态字段与初始化状态、驻留表等。真正按线程
/// 区分的状态（当前线程、监视器持有、InternalLock 守卫、拆箱失败标记）仍直写
/// `thread_local!`，与本宏区分开来，第 2 步 mt 后端只替换本宏的展开。
#[macro_export]
#[doc(hidden)]
macro_rules! __process_static {
    ($($t:tt)*) => { ::std::thread_local! { $($t)* } };
}

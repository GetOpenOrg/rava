//! rava 有栈协程原语（a3-T，计划 §21.8.2）：`jdk.internal.vm.Continuation` 的 VM 方法
//! （`enterSpecial` / `doYield`）以此实现执行流切换。
//!
//! 不依赖 `java_runtime`，可单独 `cargo test`。对外入口：
//!
//! - [`Stack`]：独立栈（slab 预留多栈、按需提交、热 / 冷槽复用与 madvise 回收；映射数与存活栈数脱钩）；
//! - [`Context`]：一个执行流被挂起时的上下文（保存的栈指针 + 栈界：软件栈界与硬件 guard 区间）；
//! - [`switch`]：保存当前上下文、恢复另一个，并传递一个字；
//! - [`stack_exhausted`] / [`YellowZone`] / [`init_platform_thread`]：软件栈界检查（Java `StackOverflowError`）。
//!
//! 新栈由 [`Context::new`] 布置初始切换帧：第一次被 [`switch`] 切入时经入口蹦床调用
//! `entry(arg, data)`；入口函数不得返回，执行完毕时切回某个已保存的上下文且不再被切入，
//! 此后其 [`Stack`] 可以 drop（归池）。
//!
//! 切换在 aarch64（AAPCS64）与 x86_64（SysV）上以汇编实现，其余平台编译期报错。

#![allow(clippy::missing_safety_doc)]

mod arch;
mod guard;
mod limit;
pub mod stack;

pub use limit::{init_platform_thread, limit_for, stack_exhausted, stack_limit, YellowZone, SHADOW, YELLOW};
pub use stack::{Stack, StackError};

/// 入口函数：`arg` 为首次切入时 [`switch`] 传入的字，`data` 为 [`Context::new`] 给定的数据指针。
/// 不得返回；也不得以 unwind 离开（`extern "C"` 边界上的 panic 会 abort）。
pub type Entry = unsafe extern "C" fn(arg: usize, data: *mut u8) -> !;

/// 一个挂起的执行流（`repr(C)`：首字段为保存的栈指针，汇编按此布局存取）
#[derive(Debug)]
#[repr(C)]
pub struct Context {
    sp: usize,
    bounds: guard::Bounds,
}

// SAFETY: Context 只是挂起执行流的栈指针与栈界；由哪个线程恢复由调用方保证互斥
unsafe impl Send for Context {}
// SAFETY: 同上
unsafe impl Send for Stack {}
// SAFETY: Stack 的只读查询（top / guard）无内部可变性
unsafe impl Sync for Stack {}

impl Context {
    /// 空上下文：作为 [`switch`] 的保存目标（如载体线程在 `enterSpecial` 处的上下文）
    pub const fn empty() -> Context {
        Context { sp: 0, bounds: guard::Bounds::NONE }
    }

    /// 在 `stack` 上布置初始切换帧：首次切入时以 `entry(arg, data)` 开始执行
    ///
    /// # Safety
    /// `stack` 在执行流结束前必须保持存活且不被另一执行流使用；`data` 的有效性由调用方保证。
    pub unsafe fn new(stack: &Stack, entry: Entry, data: *mut u8) -> Context {
        let top = stack.top();
        debug_assert_eq!(top % 16, 0);
        let sp = top - arch::FRAME;
        let frame = sp as *mut u8;
        std::ptr::write_bytes(frame, 0, arch::FRAME);
        for &(off, v) in arch::CONTROL_SLOTS {
            (frame.add(off) as *mut u64).write(v);
        }
        (frame.add(arch::SLOT_DATA) as *mut usize).write(data as usize);
        (frame.add(arch::SLOT_ENTRY) as *mut usize).write(entry as usize);
        (frame.add(arch::SLOT_RET) as *mut usize).write(arch::trampoline as *const () as usize);
        Context { sp, bounds: guard::Bounds { guard: stack.guard(), limit: limit_for(stack.limit()) } }
    }

    /// 是否持有一个可恢复的执行流
    pub fn is_suspended(&self) -> bool {
        self.sp != 0
    }
}

/// 保存当前执行流到 `save`、恢复 `load`，返回日后切回 `save` 时对端传入的字。
///
/// 每次调用后返回时可能已位于另一个 OS 线程（另一载体）上：本函数 `#[inline(never)]`，且返回后
/// 不再访问线程局部——调用方对线程局部的访问须经不内联的取址函数重新读取（§21.8.3）。
///
/// # Safety
/// `load` 必须是挂起状态（[`Context::new`] 所得或先前作为 `save` 被写入且尚未恢复），且没有其他线程
/// 同时恢复它（切入即清除其挂起标记，重复切入 panic）；`save` 在被恢复前保持有效。
#[inline(never)]
pub unsafe fn switch(save: *mut Context, load: *mut Context, arg: usize) -> usize {
    let target = std::mem::replace(&mut (*load).sp, 0);
    assert!(target != 0, "rava_coro: 切入未挂起的上下文");
    (*save).bounds = guard::current();
    guard::enter((*load).bounds);
    arch::raw_switch(&raw mut (*save).sp, target, arg)
}

/// 裸切换（不维护栈界与挂起标记），仅供寄存器保存检查等测试直接调用汇编
#[doc(hidden)]
pub use arch::raw_switch;

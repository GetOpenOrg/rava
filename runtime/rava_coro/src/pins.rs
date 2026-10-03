//! 执行流的 pin 计数与判定（a3-T2，计划 §21.8.3）：`Continuation.doYield` / `isPinned0` 的依据。
//!
//! 每个执行流（平台线程或一条协程）一组 [`Pins`]，由运行时的执行上下文块持有；协程挂载时以 [`Pins::mount`]
//! 链到载体上的外层执行流，卸载时 [`Pins::unmount`] 断开。原因码与 `jdk.internal.vm.Continuation.pinnedReason`
//! 的 tableswitch 一致：
//!
//! - CRITICAL_SECTION（2）：`Continuation.pin()` / `unpin()`，按执行流（HotSpot `ContinuationEntry::_pin_count`）；
//! - MONITOR（4）：持有的监视器；判定取载体上整条执行流链的总和（HotSpot `JavaThread::held_monitor_count` 按线程计）；
//! - NATIVE（3）：栈上的 native 帧（含执行中的 `<clinit>`）。
//!
//! 计数 > 0 期间执行流被 pin、不会让出，因而不会换载体：计数随执行流的块走即可。

use std::cell::Cell;
use std::ptr;

/// 未被 pin
pub const PINNED_NONE: i32 = 0;
/// 临界区（`Continuation.pin`）
pub const PINNED_CRITICAL_SECTION: i32 = 2;
/// 栈上有 native 帧
pub const PINNED_NATIVE: i32 = 3;
/// 持有监视器
pub const PINNED_MONITOR: i32 = 4;

/// 一个执行流的 pin 计数
#[derive(Debug)]
pub struct Pins {
    critical: Cell<u32>,
    monitors: Cell<u32>,
    natives: Cell<u32>,
    /// 外层执行流（挂载时设定）；平台线程与未挂载的协程为 null
    parent: Cell<*const Pins>,
    /// 所属作用域的身份（平台线程为 0）
    scope: usize,
}

impl Pins {
    /// 平台线程（根执行流）
    pub const fn platform() -> Pins {
        Pins::with_scope(0)
    }

    /// 作用域身份为 `scope` 的协程执行流
    pub const fn with_scope(scope: usize) -> Pins {
        Pins {
            critical: Cell::new(0),
            monitors: Cell::new(0),
            natives: Cell::new(0),
            parent: Cell::new(ptr::null()),
            scope,
        }
    }

    /// 是否根执行流（未挂在任何外层执行流上）
    pub fn is_root(&self) -> bool {
        self.parent.get().is_null()
    }

    /// 挂到 `outer` 之内（`outer` 为载体上当前的执行流）
    ///
    /// # Safety
    /// `outer` 在 [`Pins::unmount`] 之前保持存活。
    pub unsafe fn mount(&self, outer: *const Pins) {
        debug_assert!(!outer.is_null());
        self.parent.set(outer);
    }

    /// 从外层执行流断开
    pub fn unmount(&self) {
        self.parent.set(ptr::null());
    }

    /// 临界区计数 +1（根执行流无操作）；已达上限返回 false（HotSpot「pin overflow」）
    pub fn pin(&self) -> bool {
        if self.is_root() {
            return true;
        }
        match self.critical.get().checked_add(1) {
            Some(n) => {
                self.critical.set(n);
                true
            }
            None => false,
        }
    }

    /// 临界区计数 −1（根执行流无操作）；已为 0 返回 false（HotSpot「pin underflow」）
    pub fn unpin(&self) -> bool {
        if self.is_root() {
            return true;
        }
        match self.critical.get().checked_sub(1) {
            Some(n) => {
                self.critical.set(n);
                true
            }
            None => false,
        }
    }

    /// 进入一层监视器
    pub fn monitor_entered(&self) {
        self.monitors.set(self.monitors.get() + 1);
    }

    /// 退出一层监视器
    pub fn monitor_exited(&self) {
        self.monitors.set(self.monitors.get().saturating_sub(1));
    }

    /// 进入一个 native 帧
    pub fn native_entered(&self) {
        self.natives.set(self.natives.get() + 1);
    }

    /// 离开一个 native 帧
    pub fn native_exited(&self) {
        self.natives.set(self.natives.get().saturating_sub(1));
    }

    fn outer(&self) -> Option<&Pins> {
        // SAFETY: 外层执行流在本执行流挂载期间存活（mount 的约定）
        unsafe { self.parent.get().as_ref() }
    }

    /// 本执行流及其外层（直到根）持有的监视器总数
    fn monitors_through_root(&self) -> u32 {
        let mut total = self.monitors.get();
        let mut p = self.outer();
        while let Some(b) = p {
            total += b.monitors.get();
            p = b.outer();
        }
        total
    }

    /// 让出判定（本执行流为最内层）：次序同 HotSpot `freeze_internal`——临界区 → 线程持锁 → native 帧。
    /// 根执行流（不在任何协程内）返回 None
    pub fn yield_reason(&self) -> Option<i32> {
        if self.is_root() {
            return None;
        }
        Some(if self.critical.get() > 0 {
            PINNED_CRITICAL_SECTION
        } else if self.monitors_through_root() > 0 {
            PINNED_MONITOR
        } else if self.natives.get() > 0 {
            PINNED_NATIVE
        } else {
            PINNED_NONE
        })
    }

    /// 作用域判定（本执行流为最内层）：自内向外直到第一个作用域身份为 `scope` 的执行流（含），任一层被 pin
    /// 即返回其原因码。同 HotSpot `is_pinned0`：最内层查临界区与线程持锁数，逐层查 native 帧，外层查其临界区
    /// 与它之外持有的监视器。不在任何协程内返回 0
    pub fn scope_reason(&self, scope: usize) -> i32 {
        if self.is_root() {
            return PINNED_NONE;
        }
        if self.critical.get() > 0 {
            return PINNED_CRITICAL_SECTION;
        }
        if self.monitors_through_root() > 0 {
            return PINNED_MONITOR;
        }
        let mut cur = self;
        loop {
            if cur.natives.get() > 0 {
                return PINNED_NATIVE;
            }
            if cur.scope == scope {
                return PINNED_NONE;
            }
            let outer = match cur.outer() {
                Some(o) if !o.is_root() => o,
                _ => return PINNED_NONE,
            };
            if outer.critical.get() > 0 {
                return PINNED_CRITICAL_SECTION;
            }
            if outer.monitors_through_root() > 0 {
                return PINNED_MONITOR;
            }
            cur = outer;
        }
    }
}

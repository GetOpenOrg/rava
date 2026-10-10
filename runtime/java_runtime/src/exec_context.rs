//! 执行上下文块（a3-T，计划 §21.8.3）：每个执行流一块——平台线程一块（线程局部内建），每个已启动的
//! `Continuation` 一块（由 `continuation_impl.rs` 的协程记录持有）。载体线程局部只存指向当前块的一个指针，
//! `Continuation.enterSpecial` 在切入 / 切回时换指针。
//!
//! 块内现有三类 pin 计数（[`rava_coro::pins::Pins`]，判定规则与单测在 `rava_coro`）：
//!
//! - CRITICAL_SECTION：`Continuation.pin()` / `unpin()`；
//! - MONITOR：`monitor.rs` 的 `monitorenter` / `monitorexit` 与同步方法守卫；
//! - NATIVE：`#[jvm_native]` 方法体进出（宏包裹 [`NativeFrame`]，见 `rava_macros` `native_attr.rs`）与类初始化协议
//!   执行 `<clinit>` 期间（`gil.rs`；HotSpot 由 VM 帧调用 `<clinit>`，冻结时同样判定为 native 帧）。
//!
//! **TLS 地址缓存**：LLVM 视线程局部地址在函数内不变；执行流可能在一次调用之后换到另一载体上继续，所以线程局部
//! 一律经本模块不内联的取址函数访问，每次重新读取。块本身的地址在执行流存活期间不变，可以缓存。

use rava_coro::pins::Pins;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr;

pub use rava_coro::pins::{PINNED_CRITICAL_SECTION, PINNED_MONITOR, PINNED_NATIVE, PINNED_NONE};

/// 一个执行流的执行级状态（repr(C)：协程记录以本块为首字段，由块地址还原记录）
#[repr(C)]
pub struct ExecContext {
    pins: Pins,
    state: ExecState,
}

/// 随执行流走、不随载体走的运行时状态（a3-T3）：这些状态的生存期跨越 Java 调用（反射目标、栈遍历回调、
/// 引导段），期间执行流可能让出并在另一载体上恢复，同一载体上也会有别的执行流穿插运行——放在载体线程局部会
/// 串到别的执行流上。与载体绑定的状态（当前线程 / 载体线程对象、ScopedValue 缓存槽）在 `thread_impl.rs`。
#[derive(Default)]
pub struct ExecState {
    /// 最近一次实参拆箱失败标记（`reflect_dispatch::bad_arg` / `take_bad_arg`）
    pub(crate) bad_arg: Cell<bool>,
    /// @CallerSensitive 调用者栈（`reflect_dispatch::__caller_sensitive`）
    pub(crate) cs_callers: RefCell<Vec<&'static str>>,
    /// 最近从反射目标逃逸的异常（`reflect_dispatch::thrown_by_target`）
    pub(crate) target_thrown: RefCell<Vec<crate::Object>>,
    /// StackWalker 锚定的帧流（`AbstractStackWalker.callStackWalk` / `fetchStackFrames`）
    pub(crate) walk_anchors: RefCell<HashMap<i64, crate::vm_stack::AnchoredWalk>>,
    pub(crate) next_walk_anchor: Cell<i64>,
}

impl ExecContext {
    /// 平台线程（OS 线程根执行流）的块
    fn platform() -> ExecContext {
        ExecContext { pins: Pins::platform(), state: ExecState::default() }
    }

    /// Continuation 执行流的块：`scope` 为其 `ContinuationScope` 的身份。该 scope 由 Continuation 持有，
    /// Continuation 在执行流存活期间不会释放（协程栈上的 `enter` 帧持有它）
    pub fn with_scope(scope: usize) -> ExecContext {
        ExecContext { pins: Pins::with_scope(scope), state: ExecState::default() }
    }

    pub fn pins(&self) -> &Pins {
        &self.pins
    }
}

/// 当前执行流的执行级状态。块在执行流存活期间地址不变，引用可跨让出点持有（让出后在另一载体上恢复时
/// 仍是同一执行流的块）；块内单元只由本执行流访问。
pub fn state() -> &'static ExecState {
    // SAFETY: 当前块在当前执行流存活期间存活（平台块随 OS 线程、协程块随 Continuation 登记）
    unsafe { &(*current()).state }
}

std::thread_local! {
    /// 平台线程块（OS 线程的根执行流）
    static PLATFORM: ExecContext = ExecContext::platform();
    /// 当前执行流的块；null = 平台线程块
    static CURRENT: Cell<*const ExecContext> = const { Cell::new(ptr::null()) };
}

/// 对象释放的载体槽（S7-3x，见 `handle::__release`）：释放深度与待释放队列。释放（Drop 链）中途没有
/// 让出点，一次最外层释放总在同一载体上开始并清空队列，所以与载体绑定而不随执行流。
pub(crate) struct ReleaseSlot {
    pub(crate) depth: Cell<u32>,
    pub(crate) pending: RefCell<Vec<crate::obj_ref::__Obj<dyn crate::java::lang::ObjectVTable>>>,
}

std::thread_local! {
    static RELEASE: ReleaseSlot = const {
        ReleaseSlot { depth: Cell::new(0), pending: RefCell::new(Vec::new()) }
    };
}

/// 访问本载体的释放槽（不内联）；线程局部已销毁（线程退出期）→ None，`f` 随之丢弃。
#[inline(never)]
pub(crate) fn release_slot<R>(f: impl FnOnce(&ReleaseSlot) -> R) -> Option<R> {
    RELEASE.try_with(f).ok()
}

/// 持锁登记的载体槽（无 GC 文档第四节小步 A，见 `sync_model::held`，仅 debug 档）：持有字段锁的计数与
/// `__RefSlot` 槽位登记表。持有字段锁期间不得让出（安全点断言持锁数为 0；`__RefSlot` 的 OS 读写锁守卫
/// 不能跨载体释放），一次持有的登记与注销总在同一载体上，所以与载体绑定而不随执行流。
#[cfg(debug_assertions)]
pub(crate) struct HoldSlot {
    pub(crate) count: Cell<usize>,
    /// (槽地址, 是否写锁, 加锁位置)；地址 0 为空位
    pub(crate) slots: RefCell<[(usize, bool, Option<&'static std::panic::Location<'static>>); HOLD_SLOTS]>,
}

/// 槽位登记容量：嵌套持有超过此数的槽位不再登记地址（计数照常），只漏检自持有
#[cfg(debug_assertions)]
pub(crate) const HOLD_SLOTS: usize = 16;

#[cfg(debug_assertions)]
std::thread_local! {
    static HOLD: HoldSlot = const {
        HoldSlot { count: Cell::new(0), slots: RefCell::new([(0, false, None); HOLD_SLOTS]) }
    };
}

/// 访问本载体的持锁登记槽（不内联，仅 debug 档）；线程局部已销毁（线程退出期）→ None。
#[cfg(debug_assertions)]
#[inline(never)]
pub(crate) fn hold_slot<R>(f: impl FnOnce(&HoldSlot) -> R) -> Option<R> {
    HOLD.try_with(f).ok()
}

/// 当前执行流的块（每次重新读取线程局部，不内联）
#[inline(never)]
pub fn current() -> *const ExecContext {
    let cur = CURRENT.with(Cell::get);
    if cur.is_null() {
        PLATFORM.with(|p| p as *const ExecContext)
    } else {
        cur
    }
}

/// 当前执行流的 pin 计数
fn pins() -> &'static Pins {
    // SAFETY: 当前块在当前执行流运行期间存活；调用方不跨让出点持有该引用
    unsafe { &(*current()).pins }
}

/// 把 `ctx` 挂为本载体的当前块（链到挂载前的当前块之内），返回挂载前的当前块，供 [`unmount`] 恢复。
///
/// # Safety
/// `ctx` 在 [`unmount`] 之前保持存活；挂载与卸载在同一 OS 线程上成对调用。
#[inline(never)]
pub unsafe fn mount(ctx: *const ExecContext) -> *const ExecContext {
    let outer = current();
    (*ctx).pins.mount(&(*outer).pins);
    CURRENT.with(|c| c.set(ctx));
    outer
}

/// 恢复 [`mount`] 之前的当前块
///
/// # Safety
/// `outer` 为配对的 [`mount`] 返回值，且在同一 OS 线程上调用。
#[inline(never)]
pub unsafe fn unmount(ctx: *const ExecContext, outer: *const ExecContext) {
    (*ctx).pins.unmount();
    let platform = PLATFORM.with(|p| p as *const ExecContext);
    CURRENT.with(|c| c.set(if outer == platform { ptr::null() } else { outer }));
}

/// `Continuation.pin()`：false = 计数溢出
pub fn pin_critical() -> bool {
    pins().pin()
}

/// `Continuation.unpin()`：false = 计数下溢
pub fn unpin_critical() -> bool {
    pins().unpin()
}

/// 当前执行流成功进入一层监视器
pub fn monitor_entered() {
    pins().monitor_entered();
}

/// 当前执行流成功退出一层监视器
pub fn monitor_exited() {
    pins().monitor_exited();
}

/// 类初始化协议开始执行 `<clinit>`（与 [`native_exit`] 在同一执行流上成对调用）
pub fn native_enter() {
    pins().native_entered();
}

/// `<clinit>` 执行结束
pub fn native_exit() {
    pins().native_exited();
}

/// `doYield` 的判定（当前执行流为最内层）；不在 Continuation 内返回 None
pub fn yield_pinned_reason() -> Option<i32> {
    pins().yield_reason()
}

/// `isPinned0(scope)` 的判定
pub fn scope_pinned_reason(scope: usize) -> i32 {
    pins().scope_reason(scope)
}

/// 栈上一个 native 帧（`#[jvm_native]` 方法体）：构造时当前块的 native 计数 +1，drop 时 −1。
/// 计数 > 0 期间执行流被 pin、不换载体，守卫缓存块地址即可
pub struct NativeFrame {
    ctx: *const ExecContext,
}

impl NativeFrame {
    #[inline]
    pub fn enter() -> NativeFrame {
        let ctx = current();
        // SAFETY: 块在执行流存活期间不变
        unsafe { (*ctx).pins.native_entered() };
        NativeFrame { ctx }
    }
}

impl Drop for NativeFrame {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: 同上；本帧所在执行流仍存活
        unsafe { (*self.ctx).pins.native_exited() };
    }
}

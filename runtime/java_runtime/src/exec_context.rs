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
use std::cell::Cell;
use std::ptr;

pub use rava_coro::pins::{PINNED_CRITICAL_SECTION, PINNED_MONITOR, PINNED_NATIVE, PINNED_NONE};

/// 一个执行流的执行级状态（repr(C)：协程记录以本块为首字段，由块地址还原记录）
#[repr(C)]
pub struct ExecContext {
    pins: Pins,
}

impl ExecContext {
    /// Continuation 执行流的块：`scope` 为其 `ContinuationScope` 的身份。该 scope 由 Continuation 持有，
    /// Continuation 在执行流存活期间不会释放（协程栈上的 `enter` 帧持有它）
    pub const fn with_scope(scope: usize) -> ExecContext {
        ExecContext { pins: Pins::with_scope(scope) }
    }

    pub fn pins(&self) -> &Pins {
        &self.pins
    }
}

std::thread_local! {
    /// 平台线程块（OS 线程的根执行流）
    static PLATFORM: ExecContext = const { ExecContext { pins: Pins::platform() } };
    /// 当前执行流的块；null = 平台线程块
    static CURRENT: Cell<*const ExecContext> = const { Cell::new(ptr::null()) };
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

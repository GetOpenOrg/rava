//! 线程运行时支撑（#42 真多线程，`docs/plans/2026-09-26-real-multithreading.md`）。
//!
//! ## 模型（并行后端，最终态）
//!
//! 每个 Java 线程（平台 / 虚拟）是一条真实 OS 线程，并行执行。对象模型自身线程安全
//! （`sync_model`：`Arc` + 原子字段单元 + 读写锁引用槽），不再有全局解释器锁（GIL）——
//! 第一档的单线程 + GIL 后端已移除。本模块保留与锁无关的线程运行时协议：
//!
//!   - `safepoint` / `blocking` / `yield_now`：生成代码与阻塞原语的调用点（安全点与阻塞
//!     包裹在并行后端下无需让锁，前者为空、后者直通；保留为钩子供将来的安全点语义使用）；
//!   - 非守护线程存活登记（DestroyJavaVM 等待）；
//!   - 类初始化协议（JVMS §5.5，跨线程 `<clinit>` 互斥与等待）。
//!
//! 模块名沿用 `gil`（生成代码经 prelude 引用其入口），后续随调用点收敛再更名。

use parking_lot::{Condvar, Mutex};
use crate::sync_model::__PrimCell;

/// 在阻塞状态下执行 `f`（sleep / wait / park / 竞争 monitorenter / join / 类初始化等待）。
/// 并行后端无全局锁可让，直接执行。
#[inline]
pub fn blocking<R>(f: impl FnOnce() -> R) -> R {
    f()
}

/// 安全点（字段 / 数组元素读取、监视器操作处的生成代码调用点）：并行后端无需让出。
#[inline(always)]
pub fn safepoint() {}

/// `Thread.yield`：让出当前 OS 线程时间片。
pub fn yield_now() {
    std::thread::yield_now();
}

// ── 存活线程登记（DestroyJavaVM：主线程结束后等待全部非守护平台线程）─────────

static LIVE_NON_DAEMON: Mutex<usize> = Mutex::new(0);
static LIVE_CV: Condvar = Condvar::new();

/// 非守护平台线程启动登记。
pub fn note_started(daemon: bool) {
    if !daemon {
        *LIVE_NON_DAEMON.lock() += 1;
    }
}

/// 非守护平台线程终结登记。
pub fn note_terminated(daemon: bool) {
    if !daemon {
        let mut n = LIVE_NON_DAEMON.lock();
        *n -= 1;
        if *n == 0 {
            LIVE_CV.notify_all();
        }
    }
}

/// 主线程结束：等待全部非守护平台线程终结（JVM `DestroyJavaVM` 语义）。
pub fn await_non_daemon_threads() {
    blocking(|| {
        let mut n = LIVE_NON_DAEMON.lock();
        while *n > 0 {
            LIVE_CV.wait(&mut n);
        }
    });
}

// ── 类初始化协议（JVMS §5.5）─────────────────────────────────────────────────
//
// 状态单元（进程级 `__PrimCell<u8>`）：0 = 未初始化，1 = 初始化中，2 = erroneous，
// 3 = 已完成。「初始化中」的持有线程记在 `CLINIT_OWNERS`（类名 → ThreadId）：同线程
// 递归立即返回（步骤 3），他线程等待至状态离开 1（步骤 2），失败后续访问
// NoClassDefFoundError（步骤 5）。

/// `__clinit_enter` 的判定结果。
pub enum ClinitEnter {
    /// 本线程负责执行初始化。
    Run,
    /// 已完成，或本线程递归进入（立即返回）。
    Done,
    /// 曾初始化失败。
    Erroneous,
}

static CLINIT_OWNERS: Mutex<Vec<(&'static str, std::thread::ThreadId)>> = Mutex::new(Vec::new());
static CLINIT_GEN: Mutex<u64> = Mutex::new(0);
/// 已成功完成初始化的类（binary name，`/` 分隔）：`Unsafe.shouldBeInitialized` 的查询面（JVMS §5.5 状态「已初始化」）
static CLINIT_DONE: Mutex<Option<std::collections::HashSet<&'static str>>> = Mutex::new(None);
static CLINIT_CV: Condvar = Condvar::new();

/// 类是否已成功完成初始化（binary name，`/` 分隔）
pub fn clinit_done(class: &str) -> bool {
    CLINIT_DONE.lock().as_ref().is_some_and(|s| s.contains(class))
}

/// 进入类初始化（`state` 为该类的状态单元）。非泛型：全部类共用一份实例。
pub fn clinit_enter(class: &'static str, state: &'static __PrimCell<u8>) -> ClinitEnter {
    // 持有者登记：<clinit> 内启动的线程据此等待而非越过未完成的初始化。状态转换在
    // CLINIT_OWNERS 锁内完成（两线程同时读到 0 时只有一个进入 <clinit>——JVMS §5.5 的 LC 锁）。
    let me = std::thread::current().id();
    loop {
        {
            let mut owners = CLINIT_OWNERS.lock();
            match state.get() {
                0 => {
                    state.set(1);
                    owners.push((class, me));
                    // 执行 <clinit> 期间栈上视为有 native 帧（HotSpot 由 VM 帧调用 <clinit>，虚拟线程在其中
                    // 让出判定为 NATIVE pinned，§21.8.3）；与 clinit_exit 在同一执行流上成对
                    crate::exec_context::native_enter();
                    return ClinitEnter::Run;
                }
                2 => return ClinitEnter::Erroneous,
                3 => return ClinitEnter::Done,
                _ => {
                    let owner = owners.iter().find(|(c, _)| *c == class).map(|(_, t)| *t);
                    if owner.map_or(true, |t| t == me) {
                        return ClinitEnter::Done;
                    }
                }
            }
        }
        // 他线程初始化中：在广播锁下复查后等待（clinit_exit 先改状态再取广播锁通知，不丢唤醒）
        let mut gen = CLINIT_GEN.lock();
        if state.get() == 1 {
            CLINIT_CV.wait(&mut gen);
        }
    }
}

/// 结束类初始化：`ok` → 已完成（3），否则 erroneous（2）；唤醒等待者。
pub fn clinit_exit(class: &'static str, ok: bool, state: &'static __PrimCell<u8>) {
    crate::exec_context::native_exit();
    if ok {
        CLINIT_DONE.lock().get_or_insert_with(Default::default).insert(class);
    }
    {
        let mut owners = CLINIT_OWNERS.lock();
        state.set(if ok { 3 } else { 2 });
        owners.retain(|(c, _)| *c != class);
    }
    let mut gen = CLINIT_GEN.lock();
    *gen += 1;
    CLINIT_CV.notify_all();
}

/// 构建期已初始化的类（引导映像，计划 2026-10-05-boot-image-evaluator D4）：启动序列在静态字段
/// 写入映像值之后调用，不运行 `<clinit>`，直接进入「已初始化」（同 `clinit_exit` 的成功分支）。
/// 启动序列单线程执行，无等待者。
pub fn boot_initialized(class: &'static str, state: &'static __PrimCell<u8>) {
    CLINIT_DONE.lock().get_or_insert_with(Default::default).insert(class);
    state.set(3);
}

// ── 跨线程移交 ──────────────────────────────────────────────────────────────

/// 把线程对象移交给新 OS 线程的载体（对象模型为 `Arc` + 线程安全单元，移交安全）。
pub struct Handoff<T>(pub T);
unsafe impl<T> Send for Handoff<T> {}

/// 类初始化骨架的慢路径（JVMS §5.5）：他线程初始化中则等待、同线程递归立即返回、失败后
/// NoClassDefFoundError；初始化体（父类 / 超接口初始化、`<clinit>`）由调用方给出。
/// 宏按类只生成快路径判定与一行转交，骨架全程序一份（拆 crate §7.5.4，S7-4 的类初始化部分）。
pub fn class_init(
    class: &'static str,
    state: &'static __PrimCell<u8>,
    body: &dyn Fn() -> crate::error::Result<()>,
) -> crate::error::Result<()> {
    match clinit_enter(class, state) {
        ClinitEnter::Run => {}
        ClinitEnter::Done => return Ok(()),
        ClinitEnter::Erroneous => return Err(crate::error::JvmError::no_class_def_found(class)),
    }
    let result = body();
    clinit_exit(class, result.is_ok(), state);
    result.map_err(crate::error::JvmError::in_initializer)
}

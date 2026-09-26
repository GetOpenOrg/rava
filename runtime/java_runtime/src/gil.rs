//! 真多线程第一档：OS 线程 + 全局解释器锁（GIL）（#42，
//! `docs/plans/2026-09-26-real-multithreading.md` §三-A）。
//!
//! ## 模型
//!
//! 每个 Java 线程（平台 / 虚拟）是一条真实 OS 线程；执行 Java 代码（翻译体、运行时
//! 手写层）前必须持有全局锁 `GIL`。对象模型仍为 `Rc` / `Cell` / `RefCell`——同一时刻
//! 只有持锁线程触碰对象图，锁的获取 / 释放建立 happens-before，因此：
//!
//!   - 无数据竞争（`Rc` 引用计数、`RefCell` 借用标记均在锁内修改）；
//!   - `volatile` / 原子类 / CAS 的可见性与原子性平凡成立（强于 JMM 要求）；
//!   - 进程级存储（`__process_static!`）是全进程一份的全局量，经锁串行访问。
//!
//! 阻塞原语（`sleep` / `wait` / `park` / 竞争中的 `monitorenter` / `join` / 类初始化
//! 等待）一律**先释放 GIL 再阻塞**，醒来后重获取（`blocking`）——真实挂钟驻留，其他
//! 线程在此期间运行。持锁线程在安全点（字段 / 数组元素读取、监视器操作、`Thread.yield`）
//! 检查是否有线程在等锁，时间片（`SLICE`）用尽则公平让出（`MutexGuard::bump`），
//! 自旋等待另一线程写入的程序（`while (!flag) {}`）因此能推进。
//!
//! 这与 JLS §17 的线程语义等价（JLS 不要求并行执行；绿色线程 JVM 同属合规实现）：
//! 交错只发生在安全点，是合法执行集合的子集。并行加速属第二档（Arc + 原子单元，
//! 方案 §三-B），不改变可观察语义。
//!
//! ## 激活
//!
//! 首个 `Thread.start` 之前进程只有主线程，GIL 不启用（零开销：安全点仅一次 Relaxed
//! 原子读）。`activate` 在首次派生线程前由主线程调用：主线程取得 GIL 后再派生。

use std::cell::{Cell, RefCell};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use parking_lot::{Condvar, Mutex, MutexGuard};

static ACTIVE: AtomicBool = AtomicBool::new(false);
static GIL: Mutex<()> = Mutex::new(());
/// 正在等待 GIL 的线程数（安全点据此决定是否让出）。
static WAITERS: AtomicUsize = AtomicUsize::new(0);

/// 时间片：持锁超过该时长且有等待者时，安全点让出。
const SLICE: Duration = Duration::from_millis(2);

std::thread_local! {
    static HELD: RefCell<Option<MutexGuard<'static, ()>>> = const { RefCell::new(None) };
    static SLICE_START: Cell<Option<Instant>> = const { Cell::new(None) };
}

/// GIL 是否已启用（进程内曾派生过 Java 线程）。
#[inline]
pub fn is_active() -> bool {
    ACTIVE.load(Ordering::Acquire)
}

/// 启用 GIL（主线程在首次派生线程前调用；幂等）。
pub fn activate() {
    if !ACTIVE.load(Ordering::Acquire) {
        acquire();
        ACTIVE.store(true, Ordering::Release);
    }
}

/// 当前线程取得 GIL（阻塞）。派生线程入口与 `blocking` 结束时调用。
pub fn acquire() {
    WAITERS.fetch_add(1, Ordering::SeqCst);
    let guard = GIL.lock();
    WAITERS.fetch_sub(1, Ordering::SeqCst);
    HELD.with(|h| *h.borrow_mut() = Some(guard));
    SLICE_START.with(|s| s.set(Some(Instant::now())));
}

/// 当前线程释放 GIL（派生线程出口与 `blocking` 开始时调用）。
pub fn release() {
    let guard = HELD.with(|h| h.borrow_mut().take());
    drop(guard);
}

/// 在释放 GIL 的状态下执行阻塞动作 `f`，返回前重获取 GIL。
///
/// `f` 内**不得**触碰 Java 对象图与进程级存储（只做 OS 级等待）；GIL 未启用时直接执行。
pub fn blocking<R>(f: impl FnOnce() -> R) -> R {
    if !is_active() {
        return f();
    }
    release();
    let r = f();
    acquire();
    r
}

/// 安全点：有线程等待 GIL 且本线程时间片用尽时公平让出。
#[inline]
pub fn safepoint() {
    if ACTIVE.load(Ordering::Relaxed) && WAITERS.load(Ordering::Relaxed) > 0 {
        yield_slice(false);
    }
}

/// `Thread.yield`：有等待者即让出（不看时间片）。
pub fn yield_now() {
    if is_active() {
        if WAITERS.load(Ordering::SeqCst) > 0 {
            yield_slice(true);
        } else {
            std::thread::yield_now();
        }
    }
}

#[cold]
fn yield_slice(force: bool) {
    let due = force || SLICE_START.with(|s| s.get().map_or(true, |t| t.elapsed() >= SLICE));
    if !due {
        return;
    }
    HELD.with(|h| {
        if let Some(g) = h.borrow_mut().as_mut() {
            MutexGuard::bump(g);
        }
    });
    SLICE_START.with(|s| s.set(Some(Instant::now())));
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
static CLINIT_CV: Condvar = Condvar::new();

/// 进入类初始化（`get` / `set` 读写该类的状态单元，GIL 下调用）。
pub fn clinit_enter(class: &'static str, get: impl Fn() -> u8, set: impl Fn(u8)) -> ClinitEnter {
    // 持有者登记在单线程阶段同样进行：初始化期间才启用 GIL（<clinit> 内启动线程）时，
    // 新线程据此等待而非越过未完成的初始化。
    let me = std::thread::current().id();
    loop {
        match get() {
            0 => {
                set(1);
                CLINIT_OWNERS.lock().push((class, me));
                return ClinitEnter::Run;
            }
            2 => return ClinitEnter::Erroneous,
            3 => return ClinitEnter::Done,
            _ => {
                let owner = CLINIT_OWNERS.lock().iter().find(|(c, _)| *c == class).map(|(_, t)| *t);
                if owner.map_or(true, |t| t == me) {
                    return ClinitEnter::Done;
                }
                // 他线程初始化中：释放 GIL 等待任一类初始化结束的广播，再重查
                let mut gen = CLINIT_GEN.lock();
                let seen = *gen;
                release();
                while *gen == seen {
                    CLINIT_CV.wait(&mut gen);
                }
                drop(gen);
                acquire();
            }
        }
    }
}

/// 结束类初始化：`ok` → 已完成（3），否则 erroneous（2）；唤醒等待者。
pub fn clinit_exit(class: &'static str, ok: bool, set: impl Fn(u8)) {
    set(if ok { 3 } else { 2 });
    CLINIT_OWNERS.lock().retain(|(c, _)| *c != class);
    let mut gen = CLINIT_GEN.lock();
    *gen += 1;
    CLINIT_CV.notify_all();
}

// ── 跨线程移交（仅 GIL 下成立）──────────────────────────────────────────────

/// 把 `Rc` 对象图移交给新 OS 线程的载体。安全性：移交方在持有 GIL 时构造，接收方
/// 取得 GIL 后才解包使用，对象图在任意时刻只被持锁线程触碰。
pub struct Handoff<T>(pub T);
unsafe impl<T> Send for Handoff<T> {}

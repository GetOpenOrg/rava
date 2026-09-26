//! JVM 对象监视器（S-20）：`monitorenter` / `monitorexit` / `Object.wait` / `notify`
//! / `notifyAll` 的运行时载体。
//!
//! ## 锁模型
//!
//! 每个对象按**对象身份**（`ObjectVTable::__identity`，生成类为存储里的 `__identity`
//! 单元指针）惰性挂接一个 `Monitor`——身份侧表（`identity → Arc<Monitor>`）承载。
//! 同一 Java 对象的任意引用视图（wrapper 克隆、祖先视图、`Object` 装箱）身份相同，
//! 因此监视器挂接点唯一，与 JVM 的 mark-word / ObjectMonitor 惰性膨胀同构。
//!
//! `Monitor` 内部是一把线程安全互斥量 + 两个条件队列：
//!   - `acquire_q`：竞争获取监视器的线程（enter 阻塞 / wait 结束后的重获取）
//!   - `wait_q`：`Object.wait` 的等待队列（notify / notifyAll 的唤醒目标）
//! 两队列分离保证 `notify` 只唤醒等待者、不与锁竞争者串扰（单队列会丢唤醒）。
//!
//! enter / exit 可重入（按线程持有计数）；`wait` 释放全部重入计数后阻塞，
//! 返回前按原计数重新获取——JLS §17.2 监视器语义。
//!
//! ## 与 InternalLock（S-11）的关系
//!
//! `jdk/internal/misc/InternalLock` 是 JDK I/O 层（PrintStream 等）的 Java 级锁设施，
//! 其字段是翻译出的 Java 对象（Rc 载体），承载不了跨线程互斥；对象监视器是 VM 级
//! 原语，落在本模块的 Rust 原生锁上。S-11 的终态（真线程模型）另行收敛。
//!
//! ## 与 GIL 的关系（#42，`crate::gil`）
//!
//! Java 线程是真实 OS 线程，执行 Java 代码须持有 GIL。监视器的一切阻塞（竞争中的
//! enter、`wait`、wait 结束后的重获取）都在**释放 GIL 之后**进行，醒来后先拿到监视器
//! 再重获取 GIL——任何线程都不会持有 GIL 去等监视器，因此两把锁之间无环。
//! 监视器内部状态锁（`state`）只在极短临界区内持有，且从不跨越 GIL 的获取。
//!
//! ## 遗留（随线程模型一并收敛）
//!
//!   - `wait` 的 `InterruptedException`（中断唤醒）：语料无中断等待，未实现
//!     （见 thread_impl.rs 模块注释的取舍说明）。
//!   - 侧表条目不回收（对象回收后指针身份可能复用）：监视器按身份惰性创建且
//!     数量以「曾被同步块锁定的对象」为界，测试负载下无泄漏压力；引入弱键
//!     回收需对象生命周期钩子，随对象模型收敛。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::OnceLock;
use std::time::{Duration, Instant};

use parking_lot::{Condvar, Mutex};

use crate::error::{JvmError, Result};
use crate::java::lang::Object;

// ── 监视器本体 ───────────────────────────────────────────────────────────────

struct MonitorState {
    owner: Option<std::thread::ThreadId>,
    count: u32,
}

struct Monitor {
    state: Mutex<MonitorState>,
    /// 锁竞争队列（enter 阻塞者 / wait 结束后的重获取）
    acquire_q: Condvar,
    /// Object.wait 等待队列（notify / notifyAll 的目标）
    wait_q: Condvar,
}

impl Monitor {
    fn new() -> Self {
        Monitor {
            state: Mutex::new(MonitorState { owner: None, count: 0 }),
            acquire_q: Condvar::new(),
            wait_q: Condvar::new(),
        }
    }

    /// 进入监视器（可重入）。被他线程持有时释放 GIL 后阻塞在竞争队列上。
    fn enter(&self) {
        crate::gil::safepoint();
        let me = std::thread::current().id();
        {
            let mut st = self.state.lock();
            if st.owner.is_none() {
                st.owner = Some(me);
                st.count = 1;
                return;
            }
            if st.owner == Some(me) {
                st.count += 1;
                return;
            }
        }
        crate::gil::blocking(|| {
            let mut st = self.state.lock();
            self.acquire_blocking(&mut st, me, 1);
        });
    }

    /// 阻塞获取（调用方已释放 GIL）：竞争队列排队至监视器空闲，按 `count` 设重入计数。
    fn acquire_blocking(&self, st: &mut parking_lot::MutexGuard<'_, MonitorState>,
                        me: std::thread::ThreadId, count: u32) {
        loop {
            if st.owner.is_none() {
                st.owner = Some(me);
                st.count = count;
                return;
            }
            if st.owner == Some(me) {
                st.count += count;
                return;
            }
            self.acquire_q.wait(st);
        }
    }

    /// 退出监视器一层（重入计数递减；归零时让出并唤醒竞争者）。
    /// 未持有监视器时抛 `IllegalMonitorStateException`（VM 级失衡报告）。
    fn exit(&self) -> Result<()> {
        let me = std::thread::current().id();
        let mut st = self.state.lock();
        match st.owner {
            Some(o) if o == me => {
                st.count -= 1;
                if st.count == 0 {
                    st.owner = None;
                    self.acquire_q.notify_one();
                }
                Ok(())
            }
            _ => Err(JvmError::illegal_monitor_state("current thread is not owner")),
        }
    }

    /// `Object.wait(millis, nanos)`：校验参数（HotSpot JVM_MonitorWait 同序：
    /// 参数异常先于持有检查）→ 释放全部重入计数 → 释放 GIL 等待至 notify / 超时 /
    /// 虚假唤醒 → 按原计数重新获取监视器 → 重获取 GIL。`millis == 0 && nanos == 0`
    /// 为无限等待。调用方以条件循环消费虚假唤醒。
    fn wait_timeout(&self, millis: i64, nanos: i32) -> Result<()> {
        if millis < 0 {
            return Err(JvmError::illegal_argument("timeout value is negative"));
        }
        if !(0..=999_999).contains(&nanos) {
            return Err(JvmError::illegal_argument("nanosecond timeout value out of range"));
        }
        let me = std::thread::current().id();
        let saved = {
            let st = self.state.lock();
            match st.owner {
                Some(o) if o == me => st.count,
                _ => return Err(JvmError::illegal_monitor_state("current thread is not owner")),
            }
        };
        crate::gil::blocking(|| {
            let mut st = self.state.lock();
            st.owner = None;
            st.count = 0;
            self.acquire_q.notify_one(); // 让出监视器：唤醒竞争者
            if millis == 0 && nanos == 0 {
                self.wait_q.wait(&mut st);
            } else {
                let deadline = Instant::now()
                    + Duration::from_millis(millis as u64)
                    + Duration::from_nanos(nanos as u64);
                self.wait_q.wait_until(&mut st, deadline);
            }
            self.acquire_blocking(&mut st, me, saved);
        });
        Ok(())
    }

    /// `Object.notify`：唤醒等待队列上的一个线程（无等待者时静默，与 JLS §17.2 一致）。
    fn notify_one(&self) -> Result<()> {
        let me = std::thread::current().id();
        {
            let st = self.state.lock();
            if st.owner != Some(me) {
                return Err(JvmError::illegal_monitor_state("current thread is not owner"));
            }
        }
        self.wait_q.notify_one();
        Ok(())
    }

    /// `Object.notifyAll`：唤醒等待队列上的全部线程。
    fn notify_waiters(&self) -> Result<()> {
        let me = std::thread::current().id();
        {
            let st = self.state.lock();
            if st.owner != Some(me) {
                return Err(JvmError::illegal_monitor_state("current thread is not owner"));
            }
        }
        self.wait_q.notify_all();
        Ok(())
    }
}

// ── park / unpark（LockSupport 的 VM 底座）──────────────────────────────────

/// 每线程一张许可（permit 语义：unpark 授予、park 消费；多次 unpark 不累加）。
struct Parker {
    permit: Mutex<bool>,
    cv: Condvar,
}

fn parker_for(thread_identity: usize) -> Arc<Parker> {
    static PARKERS: OnceLock<Mutex<HashMap<usize, Arc<Parker>>>> = OnceLock::new();
    let mut table = PARKERS.get_or_init(|| Mutex::new(HashMap::new())).lock();
    Clone::clone(table.entry(thread_identity).or_insert_with(|| Arc::new(Parker {
        permit: Mutex::new(false),
        cv: Condvar::new(),
    })))
}

/// `Unsafe.park(isAbsolute, time)`：消费许可；无许可时释放 GIL 阻塞至 unpark /
/// 超时（相对纳秒；绝对为 epoch 毫秒截止）/ 虚假唤醒。`time == 0` 且相对为无限期；
/// 相对 `time < 0` 或已过的绝对截止立即返回（HotSpot Parker::park 同判定）。
pub fn park(thread_identity: usize, is_absolute: bool, time: i64) {
    let deadline = if is_absolute {
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64;
        if time <= now_ms {
            None
        } else {
            Some(Instant::now() + Duration::from_millis((time - now_ms) as u64))
        }
    } else if time > 0 {
        Some(Instant::now() + Duration::from_nanos(time as u64))
    } else {
        None
    };
    let indefinite = !is_absolute && time == 0;
    if deadline.is_none() && !indefinite {
        // 立即返回的形态仍消费已有许可
        *parker_for(thread_identity).permit.lock() = false;
        return;
    }
    let p = parker_for(thread_identity);
    crate::gil::blocking(|| {
        let mut permit = p.permit.lock();
        if !*permit {
            match deadline {
                Some(d) => { p.cv.wait_until(&mut permit, d); }
                None => { p.cv.wait(&mut permit); }
            }
        }
        *permit = false;
    });
}

/// `Unsafe.unpark(thread)`：授予许可并唤醒（线程未 park 时许可留待下次 park 消费）。
pub fn unpark(thread_identity: usize) {
    let p = parker_for(thread_identity);
    let mut permit = p.permit.lock();
    *permit = true;
    p.cv.notify_one();
}

// ── 身份侧表 ─────────────────────────────────────────────────────────────────

fn side_table() -> &'static Mutex<HashMap<usize, Arc<Monitor>>> {
    static TABLE: OnceLock<Mutex<HashMap<usize, Arc<Monitor>>>> = OnceLock::new();
    TABLE.get_or_init(|| Mutex::new(HashMap::new()))
}

/// 按对象身份取（惰性创建）监视器。
fn monitor_for(identity: usize) -> Arc<Monitor> {
    let mut table = side_table().lock();
    Clone::clone(table.entry(identity).or_insert_with(|| Arc::new(Monitor::new())))
}

// ── VM 入口（monitorenter / monitorexit / Object 监视器方法）────────────────

/// `monitorenter`：进入 `identity` 对象的监视器。identity 为 null 单元 → NPE
/// （JVMS §6.5：objectref 为 null 时抛 NullPointerException）。
pub fn enter(identity: usize, is_null: bool) -> Result<()> {
    if is_null {
        return Err(JvmError::null_pointer());
    }
    monitor_for(identity).enter();
    Ok(())
}

/// `monitorexit`：退出 `identity` 对象的监视器一层。
pub fn exit(identity: usize) -> Result<()> {
    monitor_for(identity).exit()
}

/// `Object.wait(millis, nanos)`（wait() = wait(0,0)）。null 检查同上。
pub fn wait_timeout(identity: usize, is_null: bool, millis: i64, nanos: i32) -> Result<()> {
    if is_null {
        return Err(JvmError::null_pointer());
    }
    monitor_for(identity).wait_timeout(millis, nanos)
}

/// `Object.notify()`。
pub fn notify(identity: usize, is_null: bool) -> Result<()> {
    if is_null {
        return Err(JvmError::null_pointer());
    }
    monitor_for(identity).notify_one()
}

/// `Object.notifyAll()`。
pub fn notify_all(identity: usize, is_null: bool) -> Result<()> {
    if is_null {
        return Err(JvmError::null_pointer());
    }
    monitor_for(identity).notify_waiters()
}

// ── ACC_SYNCHRONIZED 方法的 RAII 守卫（codegen 前导发射）─────────────────────

/// 监视器守卫：`Drop` 时释放（覆盖 return / `?` 提前退出 / panic 展开全部路径，
/// 与 JVM 同步方法在异常完成时释放监视器的语义一致）。
pub struct MonitorGuard {
    monitor: Option<Arc<Monitor>>,
}

impl MonitorGuard {
    /// 进入 `obj` 的监视器并返回守卫。构造失败（NPE 等）时不产生需回滚的持有。
    pub fn acquire(obj: &Object) -> Result<Self> {
        if obj.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let monitor = monitor_for(obj.0.__identity() as usize);
        monitor.enter();
        Ok(MonitorGuard { monitor: Some(monitor) })
    }
}

impl Drop for MonitorGuard {
    fn drop(&mut self) {
        if let Some(monitor) = self.monitor.take() {
            let _ = monitor.exit();
        }
    }
}

/// 静态同步方法的监视器：声明类的 `Class` 对象（JVMS §2.11.10——静态同步方法
/// 锁 `java.lang.Class` 实例）。按名取类对象并装箱为 `Object`，供
/// `MonitorGuard::acquire` 使用；跨 crate（java_runtime 内部 / 用户 crate）统一
/// 经 prelude 调用，免逐文件 import `Class`。
pub fn class_monitor(binary_name: &str) -> Object {
    Object::from(crate::java::lang::Class::for_class(
        crate::java::lang::String::from(binary_name)))
}

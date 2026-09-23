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
//! ## 线程层档位（单线程协作调度，S-11 线程层）
//!
//! 生成代码的对象模型是 `Rc`（非 Send/Sync），真 OS 线程被禁用：全部模拟线程
//! 在唯一 OS 线程上以嵌套 Rust 调用运行。`wait` 因此不能真正阻塞在 OS 条件变量
//! 上（唯一线程阻塞即死锁）——首个 `Thread.start0` 之后（协作泵登记，
//! `java/lang/thread_impl.rs`），`wait_timeout` 改走协作路径：
//!
//!   - 等待者领取一张 ticket 登记进本监视器的 `wait_set`，随后**泵**运行就绪
//!     模拟线程（`Thread.run`），直到 ticket 被 `notify` 置位或无就绪线程；
//!   - `notify` / `notifyAll` 优先消费 `wait_set`（置位 ticket），无 ticket 时
//!     落回 OS 条件变量（协作档位下无 OS 等待者，等价 no-op）；
//!   - 泵返回时全部泵内栈帧已退（各 RAII 守卫释放了持有的监视器），等待者
//!     直接恢复原重入计数；未被唤醒而返回属 JLS §17.3 允许的虚假唤醒语义。
//!
//! `start0` 之前（从未有线程启动）保持 OS 路径：限时等待按时限自醒，无限等待
//! 阻塞——与 Java 在无 notifier 时的行为一致。
//!
//! ## 遗留（随线程模型一并收敛）
//!
//!   - `wait` 的 `InterruptedException`（中断唤醒）：语料无中断等待，未实现
//!     （见 thread_impl.rs 模块注释的取舍说明）。
//!   - 侧表条目不回收（对象回收后指针身份可能复用）：监视器按身份惰性创建且
//!     数量以「曾被同步块锁定的对象」为界，测试负载下无泄漏压力；引入弱键
//!     回收需对象生命周期钩子，随对象模型收敛。
//!   - 模拟线程的限时等待被唤醒后无法「到点自醒」（嵌套调用栈无法暂停再续）：
//!     泵内就绪队列耗尽即返回。语料无此形态，见 compatibility.md 线程行。

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
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
    /// 协作档位（单线程模拟线程层）的等待票据：notify 置位 ticket 而非唤醒 OS
    /// 线程（见模块注释「线程层档位」）。
    wait_set: Mutex<VecDeque<Arc<AtomicBool>>>,
}

impl Monitor {
    fn new() -> Self {
        Monitor {
            state: Mutex::new(MonitorState { owner: None, count: 0 }),
            acquire_q: Condvar::new(),
            wait_q: Condvar::new(),
            wait_set: Mutex::new(VecDeque::new()),
        }
    }

    /// 进入监视器（可重入；被他线程持有时阻塞在竞争队列上）。
    fn enter(&self) {
        let me = std::thread::current().id();
        let mut st = self.state.lock();
        loop {
            if st.owner.is_none() {
                st.owner = Some(me);
                st.count = 1;
                return;
            }
            if st.owner == Some(me) {
                st.count += 1;
                return;
            }
            self.acquire_q.wait(&mut st);
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
    /// 参数异常先于持有检查）→ 释放全部重入计数 → 等待至 notify / 超时 /
    /// 虚假唤醒 → 按原计数重新获取监视器。`millis == 0 && nanos == 0` 为无限
    /// 等待。调用方以条件循环消费虚假唤醒。
    ///
    /// 首个 `Thread.start0` 后进入协作档位（模块注释「线程层档位」）：ticket
    /// 登记进 `wait_set`，泵运行就绪模拟线程直至被置位或无进展；此前保持 OS
    /// 条件变量路径。
    fn wait_timeout(&self, millis: i64, nanos: i32) -> Result<()> {
        if millis < 0 {
            return Err(JvmError::illegal_argument("timeout value is negative"));
        }
        if !(0..=999_999).contains(&nanos) {
            return Err(JvmError::illegal_argument("nanosecond timeout value out of range"));
        }
        let me = std::thread::current().id();
        let mut st = self.state.lock();
        match st.owner {
            Some(o) if o == me => {}
            _ => return Err(JvmError::illegal_monitor_state("current thread is not owner")),
        }
        let saved = st.count;
        let pump = coop_pump();
        if pump.is_some() {
            // 协作档位：先放 state 锁再入泵——泵内模拟线程会重入本监视器
            st.owner = None;
            st.count = 0;
            drop(st);
            self.acquire_q.notify_one(); // 让出监视器：唤醒 OS 竞争者（无 OS 等待者时 no-op）
            let pump = pump.unwrap();
            let ticket: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
            self.wait_set.lock().push_back(Arc::clone(&ticket));
            let notified = pump(&ticket, millis)?;
            if !notified {
                // 超时/无通知源返回：ticket 出队（单 OS 线程，泵返回与出队之间
                // 无并发 notify）。返回本身是 JLS §17.3 允许的虚假唤醒。
                self.wait_set.lock().retain(|t| !Arc::ptr_eq(t, &ticket));
            }
            // 泵内栈帧已全部退回：本监视器空闲，直接恢复原重入计数
            let mut st = self.state.lock();
            st.owner = Some(me);
            st.count = saved;
            return Ok(());
        }
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
        // 重获取（竞争队列排队），成功后恢复重入计数
        loop {
            if st.owner.is_none() {
                st.owner = Some(me);
                st.count = saved;
                return Ok(());
            }
            if st.owner == Some(me) {
                st.count += saved;
                return Ok(());
            }
            self.acquire_q.wait(&mut st);
        }
    }

    /// `Object.notify`：唤醒等待队列上的一个线程（无等待者时静默，与 JLS §17.2 一致）。
    /// 协作档位优先消费 `wait_set`（置位一张 ticket）。
    fn notify_one(&self) -> Result<()> {
        let me = std::thread::current().id();
        {
            let st = self.state.lock();
            if st.owner != Some(me) {
                return Err(JvmError::illegal_monitor_state("current thread is not owner"));
            }
        }
        if let Some(ticket) = self.wait_set.lock().pop_front() {
            ticket.store(true, Ordering::SeqCst);
            return Ok(());
        }
        self.wait_q.notify_one();
        Ok(())
    }

    /// `Object.notifyAll`：唤醒等待队列上的全部线程。
    /// 协作档位置位 `wait_set` 内全部 ticket。
    fn notify_waiters(&self) -> Result<()> {
        let me = std::thread::current().id();
        {
            let st = self.state.lock();
            if st.owner != Some(me) {
                return Err(JvmError::illegal_monitor_state("current thread is not owner"));
            }
        }
        let tickets: Vec<_> = self.wait_set.lock().drain(..).collect();
        for ticket in tickets {
            ticket.store(true, Ordering::SeqCst);
        }
        self.wait_q.notify_all();
        Ok(())
    }
}

// ── 协作调度泵（线程层档位的 wait 侧入口）──────────────────────────────────

/// 泵签名：运行就绪模拟线程直至 `ticket` 被置位（返回 true）或无就绪线程
/// （返回 false）。由 `java/lang/thread_impl.rs` 在首个 `start0` 时登记。
pub type CoopPump = fn(&Arc<AtomicBool>, i64) -> Result<bool>;

thread_local! {
    static COOP_PUMP: RefCell<Option<CoopPump>> = RefCell::new(None);
}

/// 登记协作泵（`Thread.start0` 首次启动时调用；幂等）。
pub fn set_cooperative_pump(pump: CoopPump) {
    COOP_PUMP.with(|c| *c.borrow_mut() = Some(pump));
}

fn coop_pump() -> Option<CoopPump> {
    COOP_PUMP.with(|c| *c.borrow())
}

/// 协作档位的 park 底座（`Unsafe.park` 的消费面，与 `wait_timeout` 的泵路径
/// 同型但无监视器簿记）：泵运行就绪模拟线程后返回——返回本身是 JLS §17.3
/// 允许的虚假唤醒形态，调用方（LockSupport.park / CF waitingGet）的条件
/// 循环重查消费面兑现等价。未登记泵（从未有线程启动）→ 直接返回（无就绪
/// 线程可推进，阻塞不可达的形态与 sleep0 同一取舍）。
pub fn cooperative_park() -> Result<()> {
    if let Some(pump) = coop_pump() {
        let ticket: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        pump(&ticket, 0)?;
    }
    Ok(())
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

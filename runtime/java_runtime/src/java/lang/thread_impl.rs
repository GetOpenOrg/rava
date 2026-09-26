//! `java.lang.Thread` 的 native 层：真实 OS 线程（#42 第一档，GIL 模型，见 `crate::gil`）。
//!
//! ## 线程生命周期
//!
//!   - `start0`：`NEW→RUNNABLE`（eetop / threadStatus 翻位），启用 GIL（首次），派生 OS
//!     线程；新线程取得 GIL 后以 vtable 分派执行 `run()`；
//!   - 终结（JVM thread-exit 同序）：未捕获异常报告（`Exception in thread "<name>"`，
//!     只终结本线程）→ eetop 清零（`isAlive` 翻 false）、threadStatus → TERMINATED →
//!     持有线程对象监视器 `notifyAll`（唤醒 `join` 的 `while (isAlive()) wait(0)`）；
//!   - 主线程 `main` 返回后等待全部非守护平台线程（`destroy_java_vm`）。
//!
//! 虚拟线程（`VirtualThread.start`）同样派生 OS 线程（线程模型方案 A：虚拟线程 =
//! 平台线程，Continuation 不建模），不计入 DestroyJavaVM 的等待集（JVM 语义）。
//!
//! ## 时间
//!
//! `sleep0` 释放 GIL 后真实驻留（挂钟）；其他线程在此期间运行。
//!
//! ## 字段消费清单（golden 反推，不铺全量）
//!
//! `currentThread()` 平台对象：eetop（isAlive）、name、tid、
//! holder{group, priority, daemon}（Thread 构造器继承链 getThreadGroup /
//! getPriority / isDaemon）；contextClassLoader / inheritableThreadLocals /
//! inheritedAccessControlContext 保持 null（消费点均按 null 分支短路）。
//!
//! ## 取舍
//!
//! - 中断：`interrupt` 置 Java 字段 + VM 侧镜像并唤醒目标的 sleep / wait / park；sleep /
//!   wait 抛 InterruptedException 并清中断状态，park 返回且保留状态（JVM 语义）。
//! - `getState` 在阻塞中仍报 RUNNABLE（threadStatus 不随阻塞翻位）。

use crate::prelude::*;
use super::*;

use std::cell::RefCell;
use std::time::Duration;

// JVMTI 线程状态位（jdk.internal.misc.VM.toThreadState 的编码，getState 消费）：
// ALIVE = 0x1，TERMINATED = 0x2，RUNNABLE = 0x4。
const JVMTI_ALIVE: i32 = 0x1;
const JVMTI_TERMINATED: i32 = 0x2;
const JVMTI_RUNNABLE: i32 = 0x4;
/// Java 线程的 OS 线程栈（翻译代码递归深度与 JVM 默认线程栈 + 解释帧开销对齐的保守取值；
/// 仅保留虚拟地址，按需提交）。
const JAVA_THREAD_STACK: usize = 256 << 20;

std::thread_local! {
    /// 当前 OS 线程对应的 Java 线程对象（派生时设定；主线程首次 currentThread 时构造）。
    static CURRENT: RefCell<Option<Thread>> = const { RefCell::new(None) };
}

/// 派生 OS 线程执行 `t.run()`。`daemon` 为 true 的线程不计入 DestroyJavaVM 等待集。
pub(crate) fn spawn_java_thread(t: Thread, daemon: bool) -> Result<()> {
    crate::gil::activate();
    crate::gil::note_started(daemon);
    let name = format!("{}", t.__get_name());
    let handoff = crate::gil::Handoff(t);
    let spawned = std::thread::Builder::new()
        .name(name)
        .stack_size(JAVA_THREAD_STACK)
        .spawn(move || {
            let handoff = handoff;
            crate::gil::acquire();
            let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let t = handoff.0;
                CURRENT.with(|c| *c.borrow_mut() = Some(Clone::clone(&t)));
                run_java_thread(&t);
                // 线程局部与栈上的对象引用在持有 GIL 时释放（Rc 计数只在锁内修改）
                CURRENT.with(|c| c.borrow_mut().take());
                drop(t);
            }));
            if outcome.is_err() {
                // Rust panic（未覆盖存根等致命缺口）：与主线程 panic 同样终止进程
                std::process::exit(101);
            }
            crate::gil::note_terminated(daemon);
            crate::gil::release();
        });
    if spawned.is_err() {
        crate::gil::note_terminated(daemon);
        return Err(JvmError::out_of_memory("unable to create native thread"));
    }
    Ok(())
}

/// 执行线程体并做 JVM thread-exit 簿记（见模块注释「线程生命周期」）。
fn run_java_thread(t: &Thread) {
    // 虚分派（Thread__VTable::run）：子类覆盖（Worker.run）或 Thread.run 的
    // Runnable task 入口都在此一跳生效——与 JVM 以 virtual Thread.start 调
    // run() 同构。
    let result = Thread__VTable::run(&*t.vtable);
    if let Err(e) = result {
        e.report_uncaught_in(&format!("{}", t.__get_name()));
    }
    t.__set_eetop(0);
    let _ = t.__get_holder().__set_threadStatus(JVMTI_TERMINATED);
    let obj = Object::from(Clone::clone(t));
    if let Ok(guard) = crate::monitor::MonitorGuard::acquire(&obj) {
        let _ = crate::monitor::notify_all(obj.0.__identity() as usize, false);
        drop(guard);
    }
}

impl Thread {
    /// `Thread.registerNatives:()V`：JVM 内部的 JNI 方法注册钩子（HotSpot 在
    /// `<clinit>` 中调用）。转译运行时的 native 在编译期静态解析，注册动作无
    /// 对应物 → no-op（与 CDS.initializeFromArchive 同一处置）。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `start0`：`NEW→RUNNABLE`，派生 OS 线程执行 `run()`（模块注释「线程生命周期」）。
    /// eetop 取非零（JVM 中为 native 线程句柄，仅以非零承载 alive 语义）。
    ///
    /// upcalls：新线程以 vtable 分派调用 `Thread.run()`——这条 runtime→Java 调用边
    /// 不在任何字节码里，经 upcalls 声明使 BFS 翻译 run() 的方法体（Runnable task 的
    /// 转发入口）而非停留在存根。
    #[jvm_native(upcalls = "java/lang/Thread.run:()V")]
    pub fn start0(&self) -> Result<()> {
        self.__set_eetop(1);
        let holder = self.__get_holder();
        let _ = holder.__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
        let daemon = holder.__get_daemon();
        spawn_java_thread(Clone::clone(self), daemon)
    }

    /// native `sleep0(J nanos)`：释放 GIL 后按挂钟驻留 `nanos` 纳秒（其他线程期间运行）。
    /// 进入时或驻留中被中断：清中断状态并抛 `InterruptedException("sleep interrupted")`
    /// （HotSpot JVM_Sleep 同语义）。
    #[jvm_native]
    pub fn sleep0(nanos: i64) -> Result<()> {
        let me = crate::monitor::current_thread_identity()?;
        let t = Thread::currentThread()?;
        let interrupted = t.__get_interrupted() || {
            crate::monitor::enter_blocking_status(
                crate::monitor::STATE_WAITING_TIMED | crate::monitor::STATE_SLEEPING);
            let hit = crate::monitor::sleep_interruptibly(me, nanos);
            crate::monitor::leave_blocking_status();
            hit
        };
        if interrupted {
            crate::monitor::clear_interrupt(me);
            t.__set_interrupted(false);
            return Err(JvmError::interrupted(Some("sleep interrupted")));
        }
        Ok(())
    }

    /// native `yield0()`：让出 GIL 给等待中的线程（无等待者时 OS 级让出）。
    #[jvm_native]
    pub fn yield0() -> Result<()> {
        crate::gil::yield_now();
        Ok(())
    }

    /// native `sleepNanos0(long)`：JDK25 对 `sleep0(long)` 的改名（参数同为纳秒，
    /// 语义不变）——两版语料各经各自名面触达，同一实现。
    #[jvm_native]
    pub fn sleepNanos0(nanos: i64) -> Result<()> {
        Self::sleep0(nanos)
    }

    /// native `currentThread()`：返回当前 OS 线程对应的 Java 线程对象。派生线程在入口
    /// 设定；主线程首次调用构造一次并缓存——与 JVM 平台线程对象线程内唯一一致；字段按
    /// 语料消费清单填充（模块注释），构造路径绕开 `Thread.<init>` 的安全
    /// 管制分支（JVM 的主线程对象同样由 VM 原生构造，不经 Java 构造器）。
    ///
    /// upcalls：主线程对象的 holder 经 `Thread$FieldHolder.<init>` 构造
    /// （runtime→Java 构造边，字节码不可见）。经此声明，凡闭包触达
    /// currentThread（如 PrintStream 的 println 路径），FieldHolder 类一并
    /// 入闭包——Thread.holder 字段保持真实类型而非擦除 Object，本文件的
    /// holder 访问在任意闭包形态下可编译。
    #[jvm_native(upcalls = "java/lang/Thread$FieldHolder.<init>:(Ljava/lang/ThreadGroup;Ljava/lang/Runnable;JIZ)V")]
    pub fn currentThread() -> Result<Thread> {
        if let Some(t) = CURRENT.with(|c| c.borrow().as_ref().map(Clone::clone)) {
            return Ok(t);
        }
        let main = platform_main_thread();
        CURRENT.with(|c| *c.borrow_mut() = Some(Clone::clone(&main)));
        Ok(main)
    }

    /// native `ensureMaterializedForStackWalk(Object bindings)`：JVM 在
    /// `Thread.runWith` 里对 scoped-value 绑定做的栈遍历物化簿记（纯 VM 内部
    /// 动作，无可观察行为）→ no-op。
    #[jvm_native]
    pub fn ensureMaterializedForStackWalk(_bindings: Object) -> Result<()> {
        Ok(())
    }

    /// native `getNextThreadIdOffset()`：`Thread$ThreadIdentifiers` 的线程 id
    /// 计数静态字偏移。HotSpot 返回静态字的真实地址偏移；原生二进制按
    /// 「静态原子字」约定承载（`Unsafe.getAndAddLong` 的 null 基址 + offset 键），
    /// 返回固定哨兵键。
    #[jvm_native]
    pub fn getNextThreadIdOffset() -> Result<i64> {
        Ok(1)
    }

    #[jvm_native]
    pub fn isTerminated(&self) -> Result<bool> {
        Ok(false)
    }

    /// native `holdsLock(Object)`：当前线程是否持有 obj 的监视器（null → NPE）。
    #[jvm_native]
    pub fn holdsLock(obj: Object) -> Result<bool> {
        if obj.0.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        Ok(crate::monitor::holds_lock(obj.0.__identity() as usize))
    }

    /// `interrupt()`：置中断状态并唤醒目标线程的 sleep / wait / park（`monitor::interrupt`）。
    #[jvm_native]
    pub fn interrupt(&self) -> Result<()> {
        self.__set_interrupted(true);
        crate::monitor::interrupt(Object::from(Clone::clone(self)).0.__identity() as usize);
        Ok(())
    }

    /// native `interrupt0()`：JDK `interrupt()` 字节码置字段后通知 VM——唤醒同上。
    #[jvm_native]
    pub fn interrupt0(&self) -> Result<()> {
        crate::monitor::interrupt(Object::from(Clone::clone(self)).0.__identity() as usize);
        Ok(())
    }

    /// native `clearInterruptEvent()`：Java 侧清中断状态后同步清 VM 侧镜像。
    #[jvm_native]
    pub fn clearInterruptEvent() -> Result<()> {
        crate::monitor::clear_interrupt(crate::monitor::current_thread_identity()?);
        Ok(())
    }

    /// native `getThreadGroup()`：holder.group。JVM 在线程终结时会清组引用，
    /// 语料不消费终结线程的组，本实现保持构造时值。
    #[jvm_native]
    pub fn getThreadGroup(&self) -> Result<ThreadGroup> {
        Ok(self.__get_holder().__get_group())
    }
}

/// 构造主线程平台对象（`currentThread` 的缓存初始化，见其注释）。
fn platform_main_thread() -> Thread {
    let mut t = Thread::default();
    t._init_not_null();
    t.__set_eetop(1);
    t.__set_tid(1);
    t.__set_name(String::from("main"));
    let mut group = ThreadGroup::default();
    group._init_not_null();
    group.__set_name(String::from("main"));
    // JLS §17（Thread 规范）：NORM_PRIORITY = 5、MAX_PRIORITY = 10；常量访问器
    // 仅触发 Thread.<clinit>（no-op registerNatives + 两个静态字段），失败不可达。
    group.__set_maxPriority(Thread::MAX_PRIORITY().unwrap_or(10));
    let holder = Thread_FieldHolder::new(
        group,
        Object::default(), // task：主线程无 Runnable
        0,
        Thread::NORM_PRIORITY().unwrap_or(5),
        false,
    )
    .unwrap_or_else(|e| panic!("Thread$FieldHolder.<init> 构造失败: {:?}", e));
    let _ = holder.__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
    t.__set_holder(holder);
    t
}

//! `java.lang.Thread` 的 native 层 + 单线程协作调度器（线程层语义档位）。
//!
//! ## 语义档位：单线程模拟
//!
//! 生成代码的对象模型是 `Rc`（非 Send/Sync）——把翻译出的 `run()` 放上真实
//! OS 线程会跨线程移动 Rc，属未定义行为，禁用。「线程」在此档位是**调度簿记**：
//!
//!   - `start0`：`NEW→RUNNABLE`（eetop / threadStatus 翻位），Thread 引用入
//!     就绪队列，**不**立即执行；
//!   - 推进点（`pump`）：`Thread.join`（经 `isAlive → wait(0)` 条件循环）、
//!     `Object.wait`、`Thread.sleep0` 进入等待时，泵按 FIFO 运行就绪模拟线程
//!     的 `run()` 至完成、或该线程自身 `wait` 让出（嵌套泵）；
//!   - 终结：`eetop` 清零（`isAlive` 翻 false）、threadStatus → TERMINATED、
//!     线程对象监视器 `notifyAll`（唤醒 join 等待者，与 JVM thread-exit 一致）。
//!
//! 对**确定性输出**的程序（语料三例：join 后读结果 / 排序后打印），模拟序与
//! Java 的可重放序一致；依赖真实 interleaving 的程序语义不可达（真并发属对象
//! 模型 Send/Sync 化之后的远期档位，compatibility.md 线程行如实分档）。
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
//! - `wait` / `sleep` 的 `InterruptedException` 未实现：语料三例无中断等待
//!   （TestWaitNotify 的 catch 分支不被触达），引入需构造异常链，暂不做。
//! - `sleep0` 时长不驻留（无时钟模拟）：按「sleep 期间其他可运行线程得以
//!   推进」兑现等价——泵运行就绪线程后立即返回。

use crate::prelude::*;
use super::*;

use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// JVMTI 线程状态位（jdk.internal.misc.VM.toThreadState 的编码，getState 消费）：
// ALIVE = 0x1，TERMINATED = 0x2，RUNNABLE = 0x4。
const JVMTI_ALIVE: i32 = 0x1;
const JVMTI_TERMINATED: i32 = 0x2;
const JVMTI_RUNNABLE: i32 = 0x4;

// ── 单线程协作调度器 ─────────────────────────────────────────────────────────

thread_local! {
    /// 就绪队列（FIFO = start 序）：已启动未终结的模拟线程。
    static READY: RefCell<VecDeque<Thread>> = RefCell::new(VecDeque::new());
}

/// 泵：按 FIFO 运行就绪模拟线程，直至 `ticket` 被唤醒（notify 置位，true）
/// 或无就绪线程（false）。经 `monitor::set_cooperative_pump` 挂到所有
/// `Object.wait` / join 的等待路径上。
fn pump(ticket: &Arc<AtomicBool>, _millis: i64) -> Result<bool> {
    loop {
        if ticket.load(Ordering::SeqCst) {
            return Ok(true);
        }
        let next = READY.with(|q| q.borrow_mut().pop_front());
        match next {
            Some(t) => run_sim_thread(t)?,
            None => return Ok(false),
        }
    }
}

/// 运行一个模拟线程至其 `run()` 返回（或经 `wait` 让出后由嵌套泵推进至终结），
/// 随后做 JVM thread-exit 簿记。
fn run_sim_thread(t: Thread) -> Result<()> {
    // 虚分派（Thread__VTable::run）：子类覆盖（Worker.run）或 Thread.run 的
    // Runnable task 入口都在此一跳生效——与 JVM 以 virtual Thread.start 调
    // run() 同构。
    let result = Thread__VTable::run(&*t.vtable);
    // 终结：eetop 清零（isAlive/alive 的唯一判据）、状态位 TERMINATED、
    // 线程对象监视器 notifyAll 唤醒 join 等待者。
    t.__set_eetop(0);
    let _ = t.__get_holder().__set_threadStatus(JVMTI_TERMINATED);
    let identity = Object::from(Clone::clone(&t)).0.__identity() as usize;
    let _ = crate::monitor::notify_all(identity, false);
    if let Err(e) = result {
        // 未捕获异常只终结本线程（JVM 语义），报告到 stderr；golden 只比对 stdout。
        e.report_uncaught();
    }
    Ok(())
}

impl Thread {
    /// `Thread.registerNatives:()V`：JVM 内部的 JNI 方法注册钩子（HotSpot 在
    /// `<clinit>` 中调用）。转译运行时的 native 在编译期静态解析，注册动作无
    /// 对应物 → no-op（与 CDS.initializeFromArchive 同一处置）。
    #[jvm_native]
    pub fn registerNatives() -> Result<()> {
        Ok(())
    }

    /// native `start0`：`NEW→RUNNABLE`。真 OS 线程被对象模型禁用（Rc 非
    /// Send）——Thread 引用入就绪队列，`run()` 的实际执行发生在 join / wait /
    /// sleep 的协作泵（本模块 `pump`）。eetop 取非零（JVM 中为 native 线程
    /// 句柄，仅以非零承载 alive 语义）。
    ///
    /// upcalls：调度器以 vtable 分派调用 `Thread.run()`——这条 runtime→Java
    /// 调用边不在任何字节码里，经 upcalls 声明使 BFS 翻译 run() 的方法体
    /// （Runnable task 的转发入口）而非停留在存根。
    #[jvm_native(upcalls = "java/lang/Thread.run:()V")]
    pub fn start0(&self) -> Result<()> {
        self.__set_eetop(1);
        let _ = self.__get_holder().__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
        READY.with(|q| q.borrow_mut().push_back(Clone::clone(self)));
        crate::monitor::set_cooperative_pump(pump);
        Ok(())
    }

    /// native `sleep0(J nanos)`：限时休眠。时长不驻留（无时钟模拟）；等价性按
    /// 「sleep 期间其他可运行线程得以推进」兑现——泵运行就绪模拟线程后返回。
    /// 中断唤醒（InterruptedException）未实现，见模块注释取舍。
    #[jvm_native]
    pub fn sleep0(_nanos: i64) -> Result<()> {
        let ticket: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        pump(&ticket, 0)?;
        Ok(())
    }

    /// native `currentThread()`：返回当前（唯一）OS 线程的平台线程对象。
    /// 首次调用构造一次并缓存——与 JVM 平台线程对象线程内唯一一致；字段按
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
        thread_local! {
            static CURRENT: Thread = platform_main_thread();
        }
        Ok(CURRENT.with(Clone::clone))
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

    #[jvm_native]
    pub fn interrupt(&self) -> Result<()> {
        self.__set_interrupted(true);
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
    t.__set_holder(holder);
    t
}

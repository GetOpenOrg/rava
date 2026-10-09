//! `java.lang.Thread` 的 native 层：真实 OS 线程，并行执行（#42 并行后端，见 `crate::sync_model`）。
//!
//! ## 线程生命周期
//!
//!   - `start0`：`NEW→RUNNABLE`（eetop / threadStatus 翻位），派生 OS 线程；新线程以
//!     vtable 分派执行 `run()`；
//!   - 终结（JVM thread-exit 同序）：未捕获异常报告（`Exception in thread "<name>"`，
//!     只终结本线程）→ eetop 清零（`isAlive` 翻 false）、threadStatus → TERMINATED →
//!     持有线程对象监视器 `notifyAll`（唤醒 `join` 的 `while (isAlive()) wait(0)`）；
//!   - 主线程 `main` 返回后等待全部非守护平台线程（`destroy_java_vm`）。
//!
//! 虚拟线程由 `VirtualThread` 字节码在载体线程（ForkJoinPool 工作线程，即本层派生的平台线程）上
//! 经 `Continuation` 挂载执行，不计入 DestroyJavaVM 的等待集（JVM 语义）。本层只承载
//! `JavaThread` 的三个载体槽（当前线程 / 载体线程 / ScopedValue 缓存）与 `currentCarrierThread` /
//! `setCurrentThread` 等 native。
//!
//! ## 时间
//!
//! `sleep0` 真实驻留（挂钟）；其他线程并行运行。
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

// 载体槽（a3-T3）：HotSpot `JavaThread` 上与 OS 线程绑定的三个槽。虚拟线程挂载时由字节码
// （`VirtualThread.mount` → `setCurrentThread`；`Continuation.run` 存取 ScopedValue 缓存）换入换出，
// 随执行流走的状态在执行上下文块（`exec_context.rs`）。执行流可能在一次调用之后换到另一载体上继续，
// 槽一律经下方不内联的存取函数访问（LLVM 视线程局部地址在函数内不变，内联后会跨让出点缓存旧载体的地址）。
/// `Thread$ThreadIdentifiers` 的线程 id 计数字（VM 侧静态存储，`getNextThreadIdOffset` 返回其地址）；
/// 引导映像的 VM 单元 `next_thread_id` 给出初值
pub(crate) static NEXT_THREAD_ID: std::sync::atomic::AtomicI64 = std::sync::atomic::AtomicI64::new(0);

std::thread_local! {
    /// 当前线程（`JavaThread::_vthread`）：平台线程即载体自身，虚拟线程挂载期间为该虚拟线程
    static CURRENT: RefCell<Option<Thread>> = const { RefCell::new(None) };
    /// 载体线程（`JavaThread::_threadObj`）：派生时设定；主线程首次取当前线程时构造
    static CARRIER: RefCell<Option<Thread>> = const { RefCell::new(None) };
    /// `Thread.scopedValueCache` / `setScopedValueCache`（`JavaThread::_scopedValueCache`）
    static SCOPED_VALUE_CACHE: RefCell<Option<JArray<Object>>> = const { RefCell::new(None) };
}

#[inline(never)]
fn current_slot() -> Option<Thread> {
    CURRENT.with(|c| c.borrow().as_ref().map(Clone::clone))
}

#[inline(never)]
fn set_current_slot(t: Option<Thread>) {
    CURRENT.with(|c| *c.borrow_mut() = t);
}

#[inline(never)]
fn carrier_slot() -> Option<Thread> {
    CARRIER.with(|c| c.borrow().as_ref().map(Clone::clone))
}

#[inline(never)]
fn set_carrier_slot(t: Option<Thread>) {
    CARRIER.with(|c| *c.borrow_mut() = t);
}

#[inline(never)]
fn scoped_value_cache_slot() -> Option<JArray<Object>> {
    SCOPED_VALUE_CACHE.with(|c| c.borrow().as_ref().map(Clone::clone))
}

#[inline(never)]
fn set_scoped_value_cache_slot(cache: Option<JArray<Object>>) {
    SCOPED_VALUE_CACHE.with(|c| *c.borrow_mut() = cache);
}

crate::__process_static! {
    /// 存活平台线程表（HotSpot `Threads` 列表的对应物）：start0 登记、线程终结摘除，
    /// 主线程在首次 currentThread 构造时登记。消费方：`getThreads`（Thread.getAllThreads →
    /// getAllStackTraces / ThreadGroup.activeCount / enumerate）。
    static LIVE_THREADS: crate::sync_model::__RefSlot<Vec<Thread>> =
        crate::sync_model::__RefSlot::new(Vec::new());
    /// 初始线程（VM 创建的 main 线程对象，HotSpot `create_initial_thread`）。
    static INITIAL_THREAD: crate::sync_model::__RefSlot<Option<Thread>> =
        crate::sync_model::__RefSlot::new(None);
}

fn thread_identity(t: &Thread) -> usize {
    Object::from(Clone::clone(t)).0.__identity() as usize
}

/// 派生 OS 线程执行 `t.run()`。`daemon` 为 true 的线程不计入 DestroyJavaVM 等待集。
pub(crate) fn spawn_java_thread(t: Thread, daemon: bool) -> Result<()> {
    crate::gil::note_started(daemon);
    let name = format!("{}", t.__get_name());
    let handoff = crate::gil::Handoff(t);
    let spawned = std::thread::Builder::new()
        .name(name)
        .stack_size(JAVA_THREAD_STACK)
        .spawn(move || {
            // Rust panic（未覆盖存根等致命缺口）由 create_java_vm 登记的 panic 钩子
            // 以退出码 101 终止整个进程，与主线程 panic 同一出口
            let handoff = handoff; // 整体移入闭包（Handoff 承载跨线程移交），不按字段捕获
            let t = handoff.0;
            // 本线程的软件栈界（StackOverflowError 判定）
            rava_coro::init_platform_thread();
            set_carrier_slot(Some(Clone::clone(&t)));
            set_current_slot(Some(Clone::clone(&t)));
            run_java_thread(&t);
            set_current_slot(None);
            set_carrier_slot(None);
            drop(t);
            crate::gil::note_terminated(daemon);
        });
    if spawned.is_err() {
        crate::gil::note_terminated(daemon);
        return Err(JvmError::out_of_memory("unable to create native thread"));
    }
    Ok(())
}

/// 执行线程体并做 JVM thread-exit 簿记（见模块注释「线程生命周期」）。
fn run_java_thread(t: &Thread) {
    // 虚调用 `run()`：子类覆盖（Worker.run）或 Thread.run 的 Runnable task 入口都在此一跳生效——
    // 与 JVM 以 virtual Thread.start 调 run() 同构。走方法调用形态而非 vtable trait 完全限定路径：
    // 档案内无覆盖者时 run 的槽位被裁剪（plain），trait 上不存在该方法，方法调用形态两种情况都成立。
    let result = t.run();
    if let Err(e) = result {
        e.report_uncaught_in(&format!("{}", t.__get_name()));
    }
    t.__set_eetop(0);
    let _ = t.__get_holder().__set_threadStatus(JVMTI_TERMINATED);
    let id = thread_identity(t);
    LIVE_THREADS.with(|v| v.borrow_mut().retain(|x| thread_identity(x) != id));
    let obj = Object::from(Clone::clone(t));
    if let Ok(guard) = crate::monitor::MonitorGuard::acquire(&obj) {
        let _ = crate::monitor::notify_all(obj.0.__identity() as usize);
        drop(guard);
    }
}

impl Thread {
    /// 绑定初始线程（计划 2026-10-05-boot-image-evaluator §5.5.1 S3）：引导映像中的 main 线程（HotSpot
    /// `create_initial_thread` 的对象：system / main 线程组、initPhase3 设定的上下文类加载器）成为 OS 主线程的
    /// 当前线程。eetop / threadStatus 按 `create_initial_thread` 写入（存活、RUNNABLE）。启动序列在任何 Java
    /// 代码之前调用；此前若已按需构造过初始线程，以映像线程取代。
    #[doc(hidden)]
    pub fn __vm_bind_initial(&self) {
        let t = Clone::clone(self);
        t.__set_eetop(1);
        let _ = t.__get_holder().__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
        let old = INITIAL_THREAD.with(|m| m.borrow_mut().replace(Clone::clone(&t)));
        LIVE_THREADS.with(|v| {
            let mut v = v.borrow_mut();
            if let Some(old) = old {
                let id = thread_identity(&old);
                v.retain(|x| thread_identity(x) != id);
            }
            v.insert(0, Clone::clone(&t));
        });
        set_carrier_slot(Some(Clone::clone(&t)));
        set_current_slot(Some(t));
    }

    /// 线程 id 计数字的地址与初值（引导映像 VM 单元 `next_thread_id`；本伴生文件以私有 mod 挂入，
    /// 经类型挂载对包外可见）
    #[doc(hidden)]
    pub fn __next_thread_id_cell() -> &'static std::sync::atomic::AtomicI64 {
        &NEXT_THREAD_ID
    }

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
    /// 新线程以 vtable 分派调用 `Thread.run()`——这条 runtime→Java 调用边不在任何字节码里，
    /// 闭包分析沿 `spawn_java_thread` → `run_java_thread` 的手写体调用点（`t.run()`）
    /// 推断，run() 的方法体（Runnable task 的转发入口）照常翻译。
    #[jvm_native]
    pub fn start0(&self) -> Result<()> {
        self.__set_eetop(1);
        let holder = self.__get_holder();
        let _ = holder.__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
        let daemon = holder.__get_daemon();
        LIVE_THREADS.with(|v| v.borrow_mut().push(Clone::clone(self)));
        spawn_java_thread(Clone::clone(self), daemon)
    }

    /// native `sleep0(J nanos)`：按挂钟驻留 `nanos` 纳秒。
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

    /// native `yield0()`：OS 级让出当前线程时间片。
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
    /// 主线程对象的 holder 经 `Thread$FieldHolder.<init>` 构造（runtime→Java 构造边，
    /// 字节码不可见，由手写体的构造调用推断）：凡闭包触达 currentThread（如 PrintStream 的
    /// println 路径），FieldHolder 类一并入闭包——Thread.holder 字段保持真实类型而非擦除
    /// Object，本文件的 holder 访问在任意闭包形态下可编译。
    #[jvm_native]
    pub fn currentThread() -> Result<Thread> {
        if let Some(t) = current_slot() {
            return Ok(t);
        }
        let main = platform_main_thread();
        INITIAL_THREAD.with(|m| *m.borrow_mut() = Some(Clone::clone(&main)));
        set_carrier_slot(Some(Clone::clone(&main)));
        set_current_slot(Some(Clone::clone(&main)));
        LIVE_THREADS.with(|v| v.borrow_mut().insert(0, Clone::clone(&main)));
        Ok(main)
    }

    /// native `setPriority0(int)`：OS 线程优先级提示。优先级值本身由 `setPriority`
    /// 字节码写入 holder（getPriority 读取）；HotSpot 在 Linux 缺省策略
    /// （ThreadPriorityPolicy=0）下同样不改变调度，无可观察行为。
    #[jvm_native]
    pub fn setPriority0(&self, _newPriority: i32) -> Result<()> {
        Ok(())
    }

    /// native `setNativeName(String)`：设置 OS 层线程名（调试器 / top 可见），Java 侧
    /// 名字已由 `setName` 字节码写入 name 字段；OS 线程名在派生时已按创建时名设定。
    #[jvm_native]
    pub fn setNativeName(&self, _name: String) -> Result<()> {
        Ok(())
    }

    /// native `getThreads()`：全部存活平台线程（Thread.getAllThreads 的数据源）。
    #[jvm_native]
    pub fn getThreads() -> Result<JArray<Thread>> {
        Ok(JArray::from(LIVE_THREADS.with(|v| v.borrow().iter()
            .filter(|t| t.__get_eetop() != 0)
            .map(Clone::clone)
            .collect::<Vec<_>>())))
    }

    /// native `getStackTrace0()`：他线程的栈快照。Java 帧元数据不随原生栈保留（FS-E1），
    /// 返回 null——`getStackTrace` 按 JDK 语义给出空数组（与目标线程已终结同形）。
    #[jvm_native]
    pub fn getStackTrace0(&self) -> Result<Object> {
        Ok(Object::default())
    }

    /// native `dumpThreads(Thread[])`：getAllStackTraces 的批量快照，同上每线程为空栈
    /// （StackTraceElement 由共置手写层保证恒在闭包内）。
    #[jvm_native]
    pub fn dumpThreads(threads: JArray<Thread>) -> Result<JArray<JArray<crate::java::lang::StackTraceElement>>> {
        let n = threads.len()?;
        Ok(JArray::new_with(n, || JArray::new(0)))
    }

    /// native `currentCarrierThread()`：当前载体线程（`JavaThread::_threadObj`）。虚拟线程挂载期间
    /// 与 `currentThread()` 不同；平台线程上两者相同。
    #[jvm_native]
    pub fn currentCarrierThread() -> Result<Thread> {
        if let Some(t) = carrier_slot() {
            return Ok(t);
        }
        // 主线程尚未构造：currentThread 构造并同时设两槽
        Thread::currentThread()?;
        Ok(carrier_slot().expect("currentThread 已设载体槽"))
    }

    /// native `setCurrentThread(Thread)`：`this` 为载体，VirtualThread 挂载 / 卸载时切换本载体的
    /// 「当前线程」对象（HotSpot JavaThread::_vthread）。
    #[jvm_native]
    pub fn setCurrentThread(&self, thread: Thread) -> Result<()> {
        set_current_slot(Some(thread));
        Ok(())
    }

    /// native `findScopedValueBindings()`：栈上 `runWith` 帧携带的绑定快照。绑定经
    /// `Thread.scopedValueBindings` 字段随 `ScopedValue.Carrier` 设置 / 恢复；只有线程
    /// 处于初始哨兵态（新线程、从未进入 runWith）时才走本查找，此时栈上无 runWith 帧，
    /// 返回 null（调用方回落 `Snapshot.EMPTY_SNAPSHOT`）。
    #[jvm_native]
    pub fn findScopedValueBindings() -> Result<Object> {
        Ok(Object::default())
    }

    /// native `scopedValueCache()` / `setScopedValueCache(Object[])`：每线程缓存槽。
    #[jvm_native]
    pub fn scopedValueCache() -> Result<JArray<Object>> {
        Ok(scoped_value_cache_slot().unwrap_or_default())
    }

    #[jvm_native]
    pub fn setScopedValueCache(cache: JArray<Object>) -> Result<()> {
        set_scoped_value_cache_slot(if cache.is_jvm_null() { None } else { Some(cache) });
        Ok(())
    }

    /// native `ensureMaterializedForStackWalk(Object bindings)`：JVM 在
    /// `Thread.runWith` 里对 scoped-value 绑定做的栈遍历物化簿记（纯 VM 内部
    /// 动作，无可观察行为）→ no-op。
    #[jvm_native]
    pub fn ensureMaterializedForStackWalk(_bindings: Object) -> Result<()> {
        Ok(())
    }

    /// native `getNextThreadIdOffset()`：`Thread$ThreadIdentifiers` 的线程 id 计数字地址。
    /// 与 HotSpot 同形：计数字是 VM 侧静态存储（`ThreadIdentifier` 的 next 字），返回其
    /// 绝对地址，`Unsafe.getAndAddLong(null, 地址, 1)` 经原生内存的原子指令推进。
    #[jvm_native]
    pub fn getNextThreadIdOffset() -> Result<i64> {
        Ok(NEXT_THREAD_ID.as_ptr() as i64)
    }

    /// native `holdsLock(Object)`：当前线程是否持有 obj 的监视器（null → NPE）。
    #[jvm_native]
    pub fn holdsLock(obj: Object) -> Result<bool> {
        crate::monitor::holds_lock(obj.0.__identity() as usize)
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

}

/// 构造主线程平台对象（`currentThread` 的缓存初始化，见其注释）。
fn platform_main_thread() -> Thread {
    let mut t = Thread::default();
    t._init_not_null();
    t.__set_eetop(1);
    t.__set_tid(1);
    t.__set_name(String::from("main"));
    // 字段初始化器 `interruptLock = new Object()`：主线程不经构造器，补同一初值
    // （跨线程 interrupt() 的字节码在其上 synchronized）。
    t.__set_interruptLock(Object::new().unwrap_or_default());
    let mut group = ThreadGroup::default();
    group._init_not_null();
    group.__set_name(String::from("main"));
    // JLS §17（Thread 规范）：NORM_PRIORITY = 5、MAX_PRIORITY = 10；常量访问器
    // 仅触发 Thread.<clinit>（no-op registerNatives + 两个静态字段），失败不可达。
    group.__set_maxPriority(Thread::MAX_PRIORITY().unwrap_or(10));
    let holder = Thread_FieldHolder::new(
        group,
        Default::default(), // task：主线程无 Runnable（Object 或 Runnable 载体，T-2 两形态兼容）
        0,
        Thread::NORM_PRIORITY().unwrap_or(5),
        false,
    )
    .unwrap_or_else(|e| panic!("Thread$FieldHolder.<init> 构造失败: {:?}", e));
    let _ = holder.__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
    t.__set_holder(holder);
    t
}

/// 派生 VM 系统线程（HotSpot `JavaThread::create_system_thread_object` + `start_internal_daemon`，如
/// 「Signal Dispatcher」）：守护线程，属 system 线程组（初始线程所在组的父组），最高优先级；线程对象
/// 在调用线程上按与主线程相同的方式直接构造（不经 Java 构造器），线程号取同一计数。
fn spawn_vm_system_thread(name: &'static str, body: impl FnOnce() + Send + 'static) -> Result<()> {
    let t = vm_system_thread(name)?;
    LIVE_THREADS.with(|v| v.borrow_mut().push(Clone::clone(&t)));
    crate::gil::note_started(true);
    let handoff = crate::gil::Handoff(Clone::clone(&t));
    let spawned = std::thread::Builder::new()
        .name(name.to_string())
        .stack_size(JAVA_THREAD_STACK)
        .spawn(move || {
            let handoff = handoff;
            let t = handoff.0;
            rava_coro::init_platform_thread();
            set_carrier_slot(Some(Clone::clone(&t)));
            set_current_slot(Some(Clone::clone(&t)));
            body();
            t.__set_eetop(0);
            let _ = t.__get_holder().__set_threadStatus(JVMTI_TERMINATED);
            let id = thread_identity(&t);
            LIVE_THREADS.with(|v| v.borrow_mut().retain(|x| thread_identity(x) != id));
            set_current_slot(None);
            set_carrier_slot(None);
            drop(t);
            crate::gil::note_terminated(true);
        });
    if spawned.is_err() {
        let id = thread_identity(&t);
        LIVE_THREADS.with(|v| v.borrow_mut().retain(|x| thread_identity(x) != id));
        crate::gil::note_terminated(true);
        return Err(JvmError::out_of_memory("unable to create native thread"));
    }
    Ok(())
}

/// 系统线程对象：组取当前线程所在组的根（system 组），优先级 MAX、守护
fn vm_system_thread(name: &str) -> Result<Thread> {
    let mut group = Thread::currentThread()?.__get_holder().__get_group();
    loop {
        let parent = group.__get_parent();
        if Object::from(Clone::clone(&parent)).0.is_jvm_null() {
            break;
        }
        group = parent;
    }
    let mut t = Thread::default();
    t._init_not_null();
    t.__set_eetop(1);
    t.__set_tid(NEXT_THREAD_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst));
    t.__set_name(String::from(name));
    t.__set_interruptLock(Object::new()?);
    let holder = Thread_FieldHolder::new(group, Default::default(), 0, Thread::MAX_PRIORITY()?, true)?;
    let _ = holder.__set_threadStatus(JVMTI_ALIVE | JVMTI_RUNNABLE);
    t.__set_holder(holder);
    Ok(t)
}

impl Thread {
    /// 派生 VM 系统线程（见 [`spawn_vm_system_thread`]；本伴生文件以私有 mod 挂入，经类型挂载对包外可见）
    #[doc(hidden)]
    pub fn __vm_spawn_system(name: &'static str, body: impl FnOnce() + Send + 'static) -> Result<()> {
        spawn_vm_system_thread(name, body)
    }
}

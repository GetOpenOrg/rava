//! `java/lang/VirtualThread` 手写伴生（vm_boundary 边界类，按调用链按需实现）。
//!
//! ## 线程模型方案 A（2026-09-24 用户拍板）
//!
//! 生成代码的对象模型是 `Rc`（非 Send/Sync），真 OS 线程被禁用（见 thread_impl
//! 模块注释）；平台线程在本档位是单线程协作调度的**模拟线程**。虚拟线程直接
//! 映射为同一调度器上的模拟线程：
//!
//!   - 不建模 Continuation / mount / unmount / 载体线程（VirtualThread 仍留
//!     vm_boundary，BFS 不进入 ContinuationScope 等 VM 深耦合实现）；
//!   - `start` → 入就绪队列（与 `Thread.start0` 同一队列、FIFO 同序）；
//!   - 调度器以 vtable 分派 `run()` → 本类覆盖体：经 `Thread.runWith(bindings,
//!     task)` 执行任务（与 JDK `VirtualThread.run(Runnable)` 同一入口）；
//!   - `join` → `joinNanos`：推进就绪线程直至本线程终结。
//!
//! 对输出确定的程序（join 后读结果、Future.get 汇总），与 JVM 可观察行为一致；
//! 依赖真实交错 / 载体线程身份的语义不可达（与平台线程同一档位，compatibility
//! 如实分档）。状态值取 JDK 21 VirtualThread 常量（NEW=0 / STARTED=1 /
//! RUNNING=2 / TERMINATED=99）。

use crate::prelude::*;
use super::virtual_thread::VirtualThread;
use super::Thread;

const NEW: i32 = 0;
const STARTED: i32 = 1;
const RUNNING: i32 = 2;
const TERMINATED: i32 = 99;

impl VirtualThread {
    /// `<init>(Executor scheduler, String name, int characteristics, Runnable task)`：
    /// task 存入 runContinuation 字段（本档位不建 Continuation，该字段即任务本身）。
    #[jvm_boundary]
    pub fn new(scheduler: Object, name: String, _characteristics: i32, task: Object) -> Result<Self> {
        let mut this = Self::default();
        this._init_not_null();
        this.__set_scheduler(scheduler);
        this.__set_name(name);
        this.__set_runContinuation(task);
        this.__set_state(NEW);
        Ok(this)
    }

    /// `start()`：NEW → STARTED，入模拟线程就绪队列。
    #[jvm_boundary]
    pub fn __impl_start(&self) -> Result<()> {
        self.__set_state(STARTED);
        super::thread_impl::enqueue_sim_thread(Clone::clone(self).into());
        Ok(())
    }

    /// `start(ThreadContainer)`：容器登记不建模（ThreadPerTaskExecutor 的线程
    /// 计数由其自身 onStart/onExit 维护），同 `start()`。
    #[jvm_boundary]
    pub fn __impl_start_threadcontainer(&self, _container: crate::jdk::internal::vm::ThreadContainer) -> Result<()> {
        self.__impl_start()
    }

    /// `run()`：调度器运行本模拟线程时的入口——RUNNING → 执行任务 → TERMINATED。
    /// upcalls：runWith / scopedValueBindings 为 runtime→Java 调用边（字节码不可见）。
    #[jvm_boundary(upcalls = "java/lang/Thread.runWith:(Ljava/lang/Object;Ljava/lang/Runnable;)V java/lang/Thread.scopedValueBindings:()Ljava/lang/Object;")]
    pub fn __impl_run(&self) -> Result<()> {
        self.__set_state(RUNNING);
        let task = self.__get_runContinuation();
        let t: Thread = Clone::clone(self).into();
        let result = t.runWith(Thread::scopedValueBindings()?, task);
        self.__set_state(TERMINATED);
        result
    }

    /// `alive()`：已启动且未终结。
    #[jvm_boundary]
    pub fn __impl_alive(&self) -> Result<bool> {
        let s = self.__get_state();
        Ok(s != NEW && s != TERMINATED)
    }

    /// `isTerminated()`。
    #[jvm_boundary]
    pub fn __impl_isTerminated(&self) -> Result<bool> {
        Ok(self.__get_state() == TERMINATED)
    }

    /// `joinNanos(long)`：推进就绪模拟线程直至本线程终结（时长不驻留，与
    /// sleep0 同一取舍）；返回是否已终结。
    #[jvm_boundary]
    pub fn __impl_joinNanos(&self, _nanos: i64) -> Result<bool> {
        while self.__get_state() != TERMINATED {
            if !super::thread_impl::run_next_ready()? {
                break;
            }
        }
        Ok(self.__get_state() == TERMINATED)
    }
}

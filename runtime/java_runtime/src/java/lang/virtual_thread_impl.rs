//! `java/lang/VirtualThread` 手写伴生（vm_boundary 边界类，按调用链按需实现）。
//!
//! ## 线程模型方案 A（2026-09-24 用户拍板；#42 起为真实 OS 线程）
//!
//! 虚拟线程映射为平台线程（`thread_impl::spawn_java_thread`，GIL 模型）：
//!
//!   - 不建模 Continuation / mount / unmount / 载体线程（VirtualThread 仍留
//!     vm_boundary，BFS 不进入 ContinuationScope 等 VM 深耦合实现）；
//!   - `start` → 派生 OS 线程，不计入 DestroyJavaVM 等待集（虚拟线程恒为守护线程）；
//!   - 新线程以 vtable 分派 `run()` → 本类覆盖体：经 `Thread.runWith(bindings,
//!     task)` 执行任务（与 JDK `VirtualThread.run(Runnable)` 同一入口）；
//!   - `joinNanos` → 在线程对象监视器上等待至 TERMINATED（线程终结簿记 notifyAll）。
//!
//! 状态值取 JDK 21 VirtualThread 常量（NEW=0 / STARTED=1 / RUNNING=2 / TERMINATED=99）。

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

    /// `start()`：NEW → STARTED，派生 OS 线程（守护：不计入 DestroyJavaVM 等待集）。
    #[jvm_boundary]
    pub fn __impl_start(&self) -> Result<()> {
        self.__set_state(STARTED);
        super::thread_impl::spawn_java_thread(Clone::clone(self).into(), true)
    }

    /// `start(ThreadContainer)`：容器登记不建模（ThreadPerTaskExecutor 的线程
    /// 计数由其自身 onStart/onExit 维护），同 `start()`。
    #[jvm_boundary]
    pub fn __impl_start_threadcontainer(&self, _container: crate::jdk::internal::vm::ThreadContainer) -> Result<()> {
        self.__impl_start()
    }

    /// `run()`：新 OS 线程执行本虚拟线程时的入口——RUNNING → 执行任务 → TERMINATED。
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

    /// `joinNanos(long)`：在线程对象监视器上等待至 TERMINATED（`nanos == 0` 为无限期），
    /// 返回是否已终结。
    #[jvm_boundary]
    pub fn __impl_joinNanos(&self, nanos: i64) -> Result<bool> {
        let obj = Object::from(Clone::clone(self));
        let identity = obj.0.__identity() as usize;
        let deadline = (nanos > 0).then(|| std::time::Instant::now()
            + std::time::Duration::from_nanos(nanos as u64));
        let guard = crate::monitor::MonitorGuard::acquire(&obj)?;
        while self.__get_state() != TERMINATED {
            let (ms, ns) = match deadline {
                None => (0, 0),
                Some(d) => {
                    let left = d.saturating_duration_since(std::time::Instant::now());
                    if left.is_zero() {
                        break;
                    }
                    let ms = left.as_millis() as i64;
                    let ns = (left.as_nanos() % 1_000_000) as i32;
                    if ms == 0 && ns == 0 { (0, 1) } else { (ms, ns) }
                }
            };
            crate::monitor::wait_timeout(identity, false, ms, ns)?;
        }
        drop(guard);
        Ok(self.__get_state() == TERMINATED)
    }

    /// `park()`：LockSupport.park 在虚拟线程上的入口（JLA.parkVirtualThread）——虚拟线程
    /// 即 OS 线程，与平台线程同一许可设施（`monitor::park`，按线程对象身份）。
    #[jvm_boundary]
    pub fn __impl_park(&self) -> Result<()> {
        let identity = Object::from(Clone::clone(self)).0.__identity() as usize;
        crate::monitor::park(identity, false, 0);
        Ok(())
    }

    /// `parkNanos(long)`：限时 park（`nanos <= 0` 立即返回）。
    #[jvm_boundary]
    pub fn __impl_parkNanos(&self, nanos: i64) -> Result<()> {
        if nanos > 0 {
            let identity = Object::from(Clone::clone(self)).0.__identity() as usize;
            crate::monitor::park(identity, false, nanos);
        }
        Ok(())
    }

    /// `unpark()`：授予许可（JLA.unparkVirtualThread）。
    #[jvm_boundary]
    pub fn __impl_unpark(&self) -> Result<()> {
        crate::monitor::unpark(Object::from(Clone::clone(self)).0.__identity() as usize);
        Ok(())
    }
}

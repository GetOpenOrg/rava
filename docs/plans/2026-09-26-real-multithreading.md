# 真多线程：OS 线程 + JVM 等价的时间 / 同步语义（#42）

> 2026-09-26 用户决策：虚拟时钟（N12，`a8c027e`）偏离 JVM 真实时间语义，须建立任务——
> **支持真实多线程，行为上与 JVM 等价**。本文是方案，分两档实施：
>
> - **第一档（§三-A，已实施）**：OS 线程 + 全局解释器锁（GIL）。真实线程、真实挂钟时间、
>   真实阻塞与唤醒，语义与 JVM 等价；同一时刻只有一个线程执行 Java 代码（无并行加速）。
>   协作调度与虚拟时钟随之删除。
> - **第二档（§三-B，远期）**：对象模型 Arc + 原子单元，去掉 GIL 获得并行加速；可观察语义
>   不变，属性能档位。第 1 步抽象层（`__Shared` / `__PrimCell` / `__RefSlot` /
>   `__process_static!`）两档共用。

## 一、原状（第一档实施前）：单 OS 线程协作调度

| 层 | 现状 | 单线程依赖 |
|---|---|---|
| 对象模型 | `Object = Rc<dyn ObjectVTable>`；实例体 `Rc<…>` | `Rc` 非 Send/Sync |
| 字段存储 | 宏 `struct_layout.rs`：基本类型 `Rc<Cell<T>>`、引用 `Rc<RefCell<Option<Box<T>>>>` | `Cell/RefCell` 非 Sync |
| 数组 | `JArray<T>(Rc<Repr<T>>)`，`Repr::Own(RefCell<Vec<T>>)` | 同上 |
| 静态字段 / 类初始化 | 宏 `class_init.rs`：`thread_local!` 存储 + `Cell<u8>` 状态机 | 每线程一份静态字段（真线程下语义错误） |
| 运行时登记表 | 反射分派、字段闭包、Unsafe 偏移、字符串驻留、Class 缓存等 | `thread_local!`（运行时 + 宏 96 处） |
| 线程 | `Thread.start0` 入就绪队列，`join/wait/sleep/park` 处泵式运行 | 无并发、无抢占 |
| 时间 | `park/wait/sleep` 不驻留，虚拟时钟前移 | 与挂钟偏离 |
| 监视器 | `monitor.rs`：parking_lot Mutex + Condvar，**已有 OS 路径**（协作泵登记前） | 可直接复用 |

量化（2026-09-26）：运行时 + 宏 `Rc` 212 处、`Cell/RefCell` 125 处、`thread_local!` 96 处；
字段存储形态集中在宏 `struct_layout.rs` 一处。

## 二、目标语义（JLS §17 / JVMS §5.5）

1. `Thread.start` 在新 OS 线程上执行 `run()`；`join` / `isAlive` / 未捕获异常处理与 JVM 一致。
2. `synchronized` / `wait` / `notify` 真实互斥与阻塞（监视器 OS 路径）；`wait(ms)`、`sleep`、
   `LockSupport.parkNanos/parkUntil` 真实驻留，`nanoTime` / `currentTimeMillis` 为挂钟。
3. 字段与数组元素访问无数据竞争 UB（Rust 侧）：`volatile` / 原子类（`AtomicInteger` 等，经
   Unsafe / VarHandle CAS 族）的原子性与可见性成立；普通字段至少满足 JMM 的无撕裂
   （`long/double` 按原子 64 位实现，强于 JLS §17.7 的要求）。
4. 类初始化按 JVMS §5.5 的锁 + 条件等待协议：并发首次访问只有一个线程执行 `<clinit>`，
   其余线程阻塞至完成；同线程递归立即返回；初始化失败后续访问 `NoClassDefFoundError`。
5. 静态字段全进程一份。

## 三-A、第一档：OS 线程 + GIL（已实施）

运行时 `gil.rs`（模块注释为权威说明）：

| 机制 | 实现 |
|---|---|
| 线程 | `Thread.start0` / `VirtualThread.start` → `std::thread::spawn`（栈 256 MiB 虚拟保留）；`Rc` 对象图经 `gil::Handoff` 移交，新线程取得 GIL 后解包 |
| 互斥 | 全局 `parking_lot::Mutex<()>`；执行 Java 代码须持锁。锁的获取/释放建立 happens-before：`Rc` 计数、`RefCell` 借用标记、volatile / CAS 的可见性与原子性平凡成立 |
| 阻塞 | `sleep` / `wait` / `park` / 竞争中的 `monitorenter` / `InternalLock` / 类初始化等待：**先释放 GIL 再阻塞**（`gil::blocking`），醒来先取业务锁再取 GIL——任何线程不持 GIL 等其他锁，锁序无环 |
| 抢占 | 安全点 `gil::safepoint()`：字段读取（wrapper getter、静态字段 getter）、数组元素读取、监视器 enter；有等待者且时间片（2ms）用尽时 `MutexGuard::bump` 公平让出。自旋读他线程写入的程序因此推进 |
| 进程级存储 | `__process_static!` 展开为全局 `__GilStatic<T>`（惰性初始化，`unsafe impl Sync`，只在持 GIL 时访问）；真正按线程的状态（当前线程、GIL 持有、InternalLock 守卫、拆箱失败标记）保留 `thread_local!` |
| 类初始化 | JVMS §5.5：状态 0/1/2/3（未初始化 / 初始化中 / erroneous / 完成）+ 持有线程登记；他线程释放 GIL 等待广播，同线程递归立即返回，失败后 `NoClassDefFoundError` |
| 线程终结 | 未捕获异常报告（只终结本线程）→ eetop 清零、TERMINATED → 持线程对象监视器 `notifyAll`（join 唤醒） |
| 进程退出 | `main` 返回后 `destroy_java_vm` 等待全部非守护平台线程（虚拟线程恒为守护） |
| 时间 | `nanoTime` 单调时钟、`currentTimeMillis` 挂钟；`park` 每线程许可（permit 语义，绝对 / 相对截止） |
| GIL 启用 | 首次派生线程前不启用（单线程程序零开销：安全点仅一次 Relaxed 原子读） |

等价性论证：JLS §17 不要求并行执行；交错只发生在安全点，是 JMM 合法执行集合的子集
（绿色线程 JVM 同属合规实现）。已知差距：`wait` / `sleep` / `park` 的中断唤醒
（InterruptedException）未实现；`getState` 在阻塞中仍报 RUNNABLE。

验收 e2e：`tests/e2e/60_real_threads/`（7 例，期望由 JVM 生成）+ 既有线程族。

## 三-B、第二档：同步模型抽象层 + 并行后端（远期）

**第 1 步：抽象层（行为零变化，已完成）。** 运行时新增 `sync_model` 模块，定义对象模型原语的类型
别名与操作：`Shared<T>`（Rc / Arc）、`FieldPrim<T>`（Cell / 原子）、`FieldRef<T>`（RefCell /
Mutex）、`ArrayRepr<T>`、`ProcessStatic<T>`（thread_local / 全局 OnceLock）。宏与运行时
全部改经别名；单线程后端即现有实现——生成树与行为逐字节不变（compare_trees 验收）。

**第 2 步：多线程后端（cargo feature `mt`）。**
- `Shared = Arc`；`ObjectVTable: Send + Sync`；
- 基本类型字段 → `AtomicI32/AtomicI64/AtomicU16/AtomicI8/AtomicBool`，`float/double` 经
  `AtomicU32/AtomicU64` 位形（Relaxed 读写 = 普通字段；volatile / VarHandle 族取 SeqCst）；
- 引用字段与引用数组元素 → `parking_lot::Mutex<Object>`（短临界区，读写即克隆 Arc）；
- 基本类型数组 → `Arc<[AtomicX]>`；
- 静态字段 → 全局 `OnceLock` + 上述原子单元；
- 运行时登记表 → 全局 `RwLock` / `Mutex`。

**第 3 步：类初始化协议。** 宏 `class_init.rs` 的状态机换成 JVMS §5.5：每类
`Mutex<InitState{ Uninit, InProgress(ThreadId), Done, Erroneous }>` + `Condvar`。

**第 4 步：线程与时间。** `start0` → `std::thread::spawn`（Thread 对象 Arc 移入）；
`park/unpark` → 每线程 parker（permit 语义）；`sleep` / `wait(ms)` 真实驻留；删除虚拟时钟与
协作泵；监视器只走 OS 路径。

**第 5 步：切换默认档。** `mt` 后端通过 JDK21 / JDK25 全量 e2e 后设为默认，删除单线程后端
与协作调度代码（compatibility.md 线程行改为「等价」）。

## 四、验证

- 每步：compare_trees（第 1 步要求逐字节一致）+ 线程族 e2e（TestThreadJoin / TestWaitNotify /
  TestSynchronized / TestCompletableFuture / TestVirtualThread / TestAtomics / TestVirtualClockPark）。
- 新增真并发 e2e（期望由 JVM 生成且输出确定）：多线程计数器（synchronized / AtomicInteger /
  LongAdder）、生产者-消费者（BlockingQueue）、CountDownLatch / CyclicBarrier、并发类初始化
  （多线程同时触发 `<clinit>` 只执行一次）、`sleep` 挂钟下界、`parkNanos` 超时。
- 性能基线：原子字段访问的开销在 e2e 耗时上的变化（`[time]` 行对照）。

## 五、风险

- 宏生成体量：字段访问经原子 / 锁，二进制与编译时间上升——第 2 步起逐测试记录 bin 大小与
  build 时间。
- 手写层中依赖 `Rc` 身份 / `thread_local` 的隐含假设（驻留、Class 缓存身份）需逐项改为
  进程级；以 `rg thread_local` 清单逐项销账。
- 虚拟线程：JDK 的 VirtualThread 依赖 Continuation；本方案中虚拟线程 = 平台线程
  （与现方案 A 一致），语义等价、调度粒度不同。

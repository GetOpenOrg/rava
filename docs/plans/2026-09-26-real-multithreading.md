# 真多线程：OS 线程 + JVM 等价的时间 / 同步语义（#42）

> 2026-09-26 用户决策：虚拟时钟（N12，`a8c027e`）偏离 JVM 真实时间语义，须建立任务——
> **支持真实多线程，行为上与 JVM 等价**。本文是方案，分阶段实施；协作调度 + 虚拟时钟在
> 新模型通过全量回归前保留为默认档，之后删除。

## 一、现状：单 OS 线程协作调度

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

## 三、方案：同步模型抽象层 + 双后端，逐层切换

**第 1 步：抽象层（行为零变化）。** 运行时新增 `sync_model` 模块，定义对象模型原语的类型
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

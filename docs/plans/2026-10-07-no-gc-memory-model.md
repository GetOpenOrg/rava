# 无 GC 内存模型：编译期逃逸分析 + 所有权推断

> 日期：2026-10-07（用户定，本文仅登记决定，详细设计在 C4 之后另行展开）
> 关系：[transitional-state-inventory](2026-09-26-transitional-state-inventory.md) FS-G1（Rc 环永不释放）/ FS-G4（Cleaner no-op）的终态口径；[product-vision](2026-09-18-product-vision.md)「无 GC」定位。

## 一、决定

1. **坚持无 GC**：不做追踪式 GC，也不做运行期环回收（试探删除等）。
2. 回收只靠两项编译期机制：
   - **逃逸分析 + 作用域 / 区域整组释放**：方法内不逃逸的临时对象（含成环的，如 stream 管线）在作用域或区域结束时整组释放。判据用例：PartitionInteger（C4 全量运行超时 900 s，非死循环，引用环泄漏）。
   - **所有权推断生成弱引用**：可证明安全的回指字段生成 `Weak`，从源头不成强环。
3. 两者都覆盖不到的逃逸环登记为已知限制，不以运行期机制兜底。
4. **引用类语义**（`WeakReference` / `SoftReference` 清空、`ReferenceQueue` 入队，手写边界第 ③ 类）随本机制设计，由对象释放（`Rc` drop）触发，不引入任何 GC。对外表述不写「GC 引用处理」，避免误读。

## 二、排期

C4 收官之后实施。入队路径进闭包后，`ReferenceQueue.poll` 当前的折叠随之撤销。

## 三、补充约束（2026-10-09，外部评审多轮核对后定）

> 第 1–7 条随 C4 之后的无 GC 实施落地；第 8 条对象头不变量与第四节小步 A / B 现在就做（并发正确性，不依赖 C4）。

1. **弱引用推断必须可靠**：只在能证明回指字段不是唯一持有者时生成 `Weak`；证明不了就保持强引用，交给第 6 条环检测报告。宁可泄漏，不可悬空。
2. **`Weak` 读取计入性能预算**：每次读回指字段都是一次 `upgrade`（原子 CAS + 失败分支）；推断结果要附带读次数统计，热点字段的 `Weak` 化要在对标三测点上过目。
3. **SoftReference**：封闭世界下软引用持有的对象集合有上界（档案内可分配的类型有限），语义取「与强引用等价、随持有者释放」；可选接入内存压力信号（分配失败前回调清空软引用），作为可选项，不作为正确性前提。
4. **OOM 直接 abort**：`handle_alloc_error` 直接终止进程（`obj_ref.rs`），不抛 `OutOfMemoryError`，登记为已知语义偏差。（`StackOverflowError` 已由软件栈界 `__stack_check` 实现，不在偏差之列。）
5. **监视器身份侧表与 PARKERS 表回收**：
   - 身份侧表（identity → `Arc<Monitor>`）现在永不回收。终态：对象第一次进入监视器时在对象头置 `MONITOR_MARK`；`drop_slow` 发现标记时**先删侧表条目、再 dealloc**（否则地址复用后新对象会继承旧监视器）。没有标记的对象释放时不查侧表（快速路径）。
   - 永生对象（映像对象）与静态哨兵不置标记，其条目永不回收：对象永不释放，条目不会悬空，不算泄漏。
   - PARKERS（按线程身份）同样先删后释放，或者在线程退出时删除本线程条目。
6. **环检测诊断 `cycle_finder`**（新工具）：debug 构建在退出时（或按需）扫描仍存活的强引用环，报告环上的类与字段，作为第 1 条「证明不了」时的兜底诊断与推断规则的改进依据。
7. **逃逸分析不得破坏对象头不变量**：逃逸分析降级一个对象（放到栈上、`Box`、整组区域）之前，必须**同时**满足：
   - 若它的身份用于 `wait` / `notify`：不得降级，保留 `__Obj` 对象头（无超时 `wait` 永久挂起、超时 `wait` 可中断睡眠、未持锁抛 `IllegalMonitorStateException`，三种语义都必须保留）；
   - 若它被加锁：对它的 `monitorenter` / `monitorexit` 已全部消除（不逃逸对象消锁是 JMM 允许的，HotSpot 同做法）；
   - 若它用了 `identityHashCode`：降级后的位置在整个生命期内地址稳定（`Box` 可以，按值移动的栈值不行）；一旦以后身份哈希缓存进对象头，就改为不得降级。

   此外，任何没有 `__Obj` 对象头的值，它的身份都不得流进监视器。release 构建没有断言兜底，这条只能靠逃逸分析在编译期保证。
8. **对象头不变量**（小步 A 落地，第 5 条的前提）：
   - **对象头一律由 `identity - HEAD` 推导**。逐类核实（2026-10-09）：生成类的 `__identity()` 是 inner 结构即 `__Obj` 值的地址；自有数组 `__ArrayObj` 是自身地址；协变视图返回源数组身份（标记写在拥有身份的源数组上，不写在视图上，否则视图释放会删掉仍在使用的监视器）；`JArray` 作值时返回内层数组身份；映像对象是 `__ImageObj.value` 地址；基本类型盒、`JvmRef<T>`、`Rc<__RefSlot<Vec<T>>>` 是所在 `__Obj` 的值地址。
   - **null 没有对象头**：所有 null（`JVM_NULL`、各 `__TypedNull`、接口 `__TYPED_NULL`、`__ArrayNull`）的身份都是 `&raw const JVM_NULL`，监视器入口比较一次即抛 NPE，不得流进置位函数。
   - **第二个字的位布局集中定义**在 `obj_ref.rs` 一组常量中：映像对象 `IMAGE_HASHED`（第 63 位）、`IMAGE_OBJ`（第 62 位，`Header::image()` 一律置）、哈希占低 32 位；堆对象为 16 对齐的分配大小、`MONITOR_MARK`（第 0 位）、第 1–3 位保留。`size()`、`has_monitor()`、`__image_hash`、`Header::image()` 只经这组常量读写。置位前读第二个字，带 `IMAGE_OBJ` 即跳过，堆对象用 `fetch_or`；标记的读取在 `Drop` 的 Acquire 栅栏之后。
   - **映像对象计数会漂移**：构建期写好的映像内部引用与静态字段初值没有对应的加一，运行期覆盖时旧值多减一次，计数从 `IMMORTAL`（`1<<62`）往下漂。无害（无代码与 `IMMORTAL` 比较，降到 0 需净覆盖 2^62 次，`MAX_REFCOUNT = isize::MAX` 高于 `IMMORTAL`），但**计数不得作为永生判据**。
   - 运行期绝不写静态哨兵的「对象头」（它们没有）；映像对象头可写（含 Atomic 的 static 在可写段），但只做计数加减，不写第二个字。

## 四、并发正确性小步（2026-10-09 定，现在做，不等 C4）

- **小步 A（持锁与对象头断言）**，按序：
  1. 持锁线程局部计数，只加在 `__RefField::with` / `with_mut` 与 `__RefSlot` 守卫上；`drop_slow` 断言计数为 0（锁内释放对象可能重入任意析构链）；
  2. `__RefSlot` 写锁前做自持有检查（线程局部小数组记录持有的槽位地址），自持有即 panic 报位置；
  3. `__safepoint()` 与 `<clinit>` 入口加 debug 断言（不得持字段锁）；
  4. `__RefField::with(FnOnce(&mut T))` 拆成 `with(&T)` / `with_mut`，迁移调用方；需要在锁内换下的值从闭包返回，放锁后再 drop；
  5. 第三节第 8 条：位布局常量、`IMAGE_OBJ`、监视器入口 null 判定（删 `is_null` 参数）、置位函数与 debug 断言（地址不在映像段内；对象头合理性：第二个字带 `IMAGE_OBJ`，或为 ≥`HEAD` 的 16 的倍数且低位只含已定义标志，且 `strong > 0`）。侧表回收（第 5 条）本步只置标记，不删条目；删条目随第 5 条实施。
  6. 合批跑测试集，报告注明断言覆盖范围（debug 档才生效）。
- **小步 B（volatile / 监视器 SeqCst，单独提交）**：volatile 引用字段的加锁 CAS **与解锁 store** 都改为 SeqCst（`class_init.rs:140-154` 与实例字段布局按 `is_volatile` 分流，非 volatile 保持 Acquire / Release）；监视器进入纳入 SeqCst 全序（以监视器退出后加 SeqCst 栅栏为出发点）。提交说明逐条论证：volatile 之间（引用-引用、引用-基本类型）、volatile 与监视器、两个监视器之间、IRIW；写明 ARMv8 硬件上测不出差别，正确性只能靠模型推理。volatile 读改为 SeqCst 后的排队开销记为性能关注点。
- 监视器入口改为接收句柄：不作为本次正确性前提，登记为长期整理方向。

## 五、现状与交接（2026-10-10）

### 1. 已完成

- **文档**：第三节约束 1–8、第四节小步 A / B，提交 879bf139，已快进进集成分支 rust-closure-analyzer。约束经外部评审多轮核对后定稿：
  - 判据用 `IMAGE_OBJ` 位，不用计数，也不用指针标记；
  - 对象头由 `identity - HEAD` 推导；
  - 逃逸分析的三条约束必须同时满足。
- **小步 A 第 1–5 项**：分支 conc-step-a（基于 879bf139），工作区 `rava_concA`，已推送 origin 和 github，**未合入**。

| 提交 | 内容 |
|---|---|
| 78d2b9c6 | 第 1–2 项：持锁线程局部计数（只在 `debug_assertions` 下生效，挂在 RAII 持锁凭据上，panic 展开时也会减计数），覆盖 `with` / `with_mut` 与 `__RefSlot` 读写守卫（新守卫类型 `__SlotRead` / `__SlotWrite`）；`__RefSlot` 写锁自持有检查（16 项线程局部数组），并检查「已持写锁再加读锁」；`drop_slow` 断言释放时不持字段锁；`__GilStatic::set` 改为在锁外释放旧值 |
| 4400cc70 | 第 3 项：`gil::safepoint` 与 `gil::clinit_enter` 断言不持字段锁（`<clinit>` 唯一入口经 `clinit_enter`，生成器不改） |
| 2031ac5c | 第 4 项：`with(&T)` / `with_mut(&mut T)` 拆分。实际调用方只有 3 处（`sync_model::replace`、`field_desc::__ref_field` 的 Update 分支、`array/store.rs` 引用元素 `update`），全部改为 `with_mut`；`System.setOut0` / `setErr0` / `setIn0` 改为在锁外释放旧流 |
| f00af3cc | 第 5 项：第二个字改为私有 `AtomicUsize`，位布局常量集中定义；`Header::image` 一律置 `IMAGE_OBJ`，仍可常量构造，映像生成器不改；`__mark_monitor` 含两条 debug 断言（不在映像段内、对象头合理性）；监视器入口先比较 `&raw const JVM_NULL` 并抛 NPE，删除 `is_null` 参数；`holds_lock` 改为返回 `Result<bool>`，null 抛 NPE；`monitorexit(null)` 抛 NPE；`monitor_for` 新建条目时置标记，**不删条目** |

### 2. 未验证（合入前必须完成）

- **`java_runtime` 编译未确认**。它不能脱离生成层单独编译。子代理只在临时 crate 里检查了 `sync_model.rs` 与 `obj_ref.rs`（debug / release 都通过）。以下 8 个文件没有经过编译器：
  - `monitor.rs`、`gil.rs`、`field_desc.rs`、`system_impl.rs`、`array/store.rs`
  - `java/lang/object.rs`、`java/lang/object_impl.rs`、`java/lang/thread_impl.rs`
- generator 全工作区（`--release --tests`）与 `rava_macros_core` 的 cargo check 已通过。
- **预计 debug 档会触发的断言**：子代理有意没有预先清理，这些正是断言要找的违例：
  - `thread_impl` 中 `LIVE_THREADS ... retain`：在 `__RefSlot` 写锁内释放线程对象；
  - `System.in_()` 并发首读：`get_or_insert` 在锁内丢掉多余的流；
  - 各登记表 `insert` 覆盖旧值：旧值在写锁内释放；
  - 持 `__RefSlot` 守卫期间执行 Java 代码的地方，会被安全点断言抓到。

### 3. 下一步（按序）

1. **服务器 debug 档抽查**（第 6 项前半）：在 conc-step-a 上跑 HelloWorld，加几个多线程 / 监视器用例（如 `wait` / `notify`、`synchronized` 静态方法、`Thread.holdsLock`）。先确认 `java_runtime` 能编译，再收集断言报出的违例。抽查要用 debug 档（不加 `--release`），否则断言不生效。
2. **逐个修违例**：统一写法是在锁内换出旧值、放锁后再 drop。安全点违例改为放锁后再调 Java。每类违例单独提交。
3. **进合批**（第 6 项后半）：交给协调会话排批。报告要注明断言只在 debug 档覆盖，release 档的抽查只能验证功能。
4. **小步 B**（第四节）：A 合入后另派代理、单独提交。分支基于含 A 的集成分支。
5. 第三节第 5 条的侧表删条目（`drop_slow` 见 `MONITOR_MARK` 即先删后释放）、PARKERS 回收，以及第 1–4、6、7 条，按第二节排期在 C4 之后实施。

### 4. 注意

- 另有协调巡检会话（rava-b5）在管服务器作业、合批和 tasks.md。发抽查或合批前先和它对齐，避免同 tag 作业冲突，也避免子代理总数超过上限 5。
- 计数漂移的结论：无害，`Drop` 不改，但计数不得作为永生判据（第三节第 8 条）。后续代码不要引入 `strong >= IMMORTAL` 这类判断。

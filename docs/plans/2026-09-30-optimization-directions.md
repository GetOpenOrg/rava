# 优化方向总览：闭包精度、闭包分析效率、生成器效率

> 日期：2026-09-30
> 性质：本文记录用户拍板的优化方向、验收口径和各执行线的分工，是总纲，执行细节以各子计划为准。
> 关联：[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md)（跨测试编译复用，方案编写中）、[`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md)（rustc 内存与拆 crate，§7.5.4 终态达标账）、[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md)（C0–C5）、[`2026-09-29-boundary-narrowing.md`](2026-09-29-boundary-narrowing.md)（C1d 与精度线 §6.9）、[`2026-09-30-closure-analyzer-performance.md`](2026-09-30-closure-analyzer-performance.md)（闭包分析性能 P0–P8）、[`2026-09-30-rust-emitter.md`](2026-09-30-rust-emitter.md)（Rust 生成器）、[`2026-09-30-emitter-performance.md`](2026-09-30-emitter-performance.md)（生成器效率）。

---

## 〇、根本目标（用户 2026-10-01）

> 「最根本的是，提升效率，降低成本，降低内存，缩短时间消耗。但是要保证正确性，以及生成的代码可以正常运行，并且运行准确」

- **优化目标**：端到端（闭包分析 → 发射 → cargo 编译 → 运行）的耗时、内存、成本全部下降。
- **不可让步的约束**：正确性。生成的代码必须能编译、能运行，并且输出与 JVM 一致。任何优化在这三点上出现回归都不接受。
- **验收顺序**：先守正确性（动态对照漏覆盖 = 0、e2e 通过、输出与 JVM 一致），再比较效率。验收口径（D3 的集合一致、生成形态可以偏离 Python）只能放宽「形式」，不能放宽「行为」。

## 一、决策（用户 2026-09-30）

| # | 决策 | 原话 |
|---|---|---|
| D1 | **凡能提升闭包精度的优化都要做**：精度缺口直接列入实施，不以「只影响闭包大小、不影响正确性」为由挂起或推迟 | 「能够有利于提升精度的优化都要做」 |
| D2 | **闭包分析效率、生成器效率同样全部要做**，目标是终态效率，不做阶段性妥协 | 「包括提升闭包分析效率、生成器效率」 |
| D3 | **性能改造的验收口径是集合一致**：类集、方法集及其余集合型输出（dispatch / folds / facts 条目内容）必须一致；`via` 和条目顺序可以变化，前提是顺序不影响正常测试（生成树 / e2e） | 「只要最终的集合结果是一致的就可以了，不在意顺序，只要这个顺序不影响正常的测试」 |

健全性底线不变：动态对照（C5，`scripts/dyn_compare.py`）的翻译域漏覆盖 = 0。精度优化只能在这个前提下缩小闭包。

## 二、终态指标（量化）

| 方向 | 指标 | 现状（2026-09-30） | 终态 |
|---|---|---:|---:|
| 闭包精度 | 动态对照翻译域漏覆盖 | 0 | 0（底线） |
| 闭包精度 | 已登记的精度缺口（§三.1 清单） | 三期 9 项完成（G6 按实测修订目标），✅ 已合入 d8212bee（2026-10-01） | 0 |
| 闭包精度 | 无运行期依据的可达链（如 Digester 的 VarHandle 访问模式链，约 +1045 方法） | 存在 | 0 |
| 闭包分析效率 | e2e 任一用例的墙钟 | DeepCopy 冷算 5.1 s（P6 前）；P7 四项 + P5 一 + P8 一后分段 analyze 423 + aux 299 + sites ~890 + flows ~607 + process ~875 + setup ~165 ms（c9d0a0ca） | ≤ 10 s（HelloWorld ≤ 0.5 s）；DeepCopy 冷算 ≤ 2 s |
| 闭包分析效率 | e2e 任一用例的峰值 RSS | DeepCopy 652 MB（P4，✅ 已达标） | ≤ 1 GB |
| 闭包分析效率 | 同输入重跑（缓存命中）单测试墙钟 | P6 整体结果缓存：分析段 24–33 ms，DeepCopy `--no-run` 1.12 s（✅ 已达标） | ≤ 2 s |
| 生成器效率 | 发射阶段墙钟 / 峰值 RSS（任一用例） | P0 5.9 s / 342 MB → P5 1.99 s / 315 MB → 并行发射 1.02 s / 396 MB（DeepCopy 热写出；冷写出 1.08 s） | ≤ 2 s / ≤ 500 MB |
| 生成器效率 | 内容未变文件的重写次数（复用 scratch 时） | 0（P5） | 0 |
| 下游编译 | 单个 rustc 峰值 RSS | 约 14 GB（N8，单 crate）→ S4/S5 后 Digester 声明 crate 3.1–3.4 GB | ≤ 2 GB（路径见 [`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md)） |
| 下游编译 | HelloWorld 编译墙钟 | 23.9 s（主线）→ S5 后 cargo 16.9 s / 2 m 40 s（C1d） | ≤ 12 s；仅用户类变化时 JDK 部分零重编 |
| 测试效率 | JDK 21 全量 e2e（1083 例，7 台服务器） | 约 5 h 墙钟、约 35 机时；cargo 编译约占 70–75%，转译约 20%，超时空等约 4%，运行约 3%（2026-10-01，主要在 f00b6858） | 由跨测试编译复用方案（§三.5）量化给出；超时空等 0 |
| 测试效率 | 同一 JDK 类跨测试重复编译 | 每个测试各编一遍（生成文本随测试闭包变化，无法复用） | 同内容只编一次 |
| 生成代码运行性能 | 计算密集用例运行段（debug 构建，服务器；标杆 LynchBell） | > 300 s（`RUN_TIMEOUT` 超时，2026-10-01 分布式跑批） | ≤ 30 s，且不放宽 `RUN_TIMEOUT` |

## 三、执行线

### 1. 闭包精度（精度三期，分支 `closure-prec3`）

已完成部分见边界收窄计划 §6.9（精度线、精度二期）。三期清单：

| # | 项 | 来源 |
|---|---|---|
| 1 | VarHandle 可达性收窄：`Invokers.checkVarHandleGenericType` 被反射根带成可达，导致 `VarForm.resolveMemberName` 按访问模式名在约 25 个 VarHandle* 类里查方法（Digester 约 +1045 方法） | 精度二期实测 |
| 2 | G4：平台线程上 `instanceof CarrierThread` / 虚拟线程分支折叠 | §6.7 |
| 3 | G5：异常构造 / 错误消息冷路径触达 Formatter、stream 子系统（如 `getConstructor0 → methodToString → Arrays.stream` 带入约 60 个 stream 类） | §6.7 / §6.9 |
| 4 | G6：经字段传递的构造期标志常量（`Pattern.flags0` / `CANON_EQ`） | §6.9 设计 |
| 5 | ServiceLoader 服务目录事实（模块 provides 作事实） | §6.9 |
| 6 | `SystemJavaLangAccess` 选择失败（`decodeASCII` / `encodeASCII` / `inflateBytesToChars`） | §6.9 |
| 7 | 数组汇聚：DeepCopy 约 6200 个数组汇入同一 arraycopy 类手写调用点（`hw_mem.rs::hw_site_arrays`） | 性能 P0 剖析 |
| 8 | 类初始化事实：「初始化以参数传入的 Class」（`Unsafe.ensureClassInitialized` 等），供生成器消费（EasterRelatedHolidays） | 精度二期诊断 |
| 9 | 实施中发现的其余精度缺口 | — |

### 2. 闭包分析效率（性能结构改造，分支 `closure-perf2` → `closure-mono`）

- P0–P2 已合入（5b95675a），4 例 closure.json 逐字节一致，DeepCopy user 351 s → 130 s。
- 状态（2026-10-01，main c9d0a0ca）：P7 四项 ✅（含写站点经汇集节点分发）；P5 第一项 ✅（枢纽对按调用点建模目标增量接边，2c503775，DeepCopy flows 914 → ~600 ms）；P8 第一项 ✅（成员引用文本键缓存、按 Rust 名查方法先比前缀，93dcccb2，DeepCopy process 1050 → ~875 ms）；进行中：P8 余量、sites。
- 早先状态（main d842653d）：P0–P4、P6 ✅；P7 进行中（一 / 二 / 三已合入：站点重跑常数开销、记忆条目精确作废、站点只处理新增接收者 + 字段读汇集节点；剩写站点分发节点、hub 路径常数）；P5 flows、P8 process 与工程化待做。分段数据见性能计划 §4.8。
- 结构性改造（`closure-perf2`）与顺序依赖修复 + 批量排空（`closure-mono`，6e0849c6）✅ 已合入：集合结果与处理顺序无关（`--flow-batch 1/7/64 × --hash-seed` 矩阵一致），DeepCopy 59.5 s → 41.7 s。待做：P3、P4、W→E 扇出，P5–P8。
- P0 剖析推翻原假设：方法整体重分析不到 1%。真正的热点是类型流传播，手写调用点数组读写的双向扇出占全部边的 82%，其中 99.7% 的传播不带来新类型。
- 按 D3 放宽不变量后做结构性改造：扇出边经共享节点、按调用图 SCC 排序；然后按数据推进 P3 上下文共享、P4 内存；P5–P8（预算降级、跨测试缓存、并行、工程化）随后。
- 顺序影响的检查：确认 Python / Rust 两个生成器不依赖 closure.json 的条目顺序。如果依赖，由分析器按键排序输出，并用生成树对照确认不变。

### 3. 生成器效率（分支 `emitter-perf`）

- P0 观测：`rava build` 分阶段耗时、峰值内存、按类 / 方法的 Top-N。
- 生成器自身优化：热路径分配、类型 / 描述符驻留与缓存、按类并行发射（输出确定）、闭包结果进程内传递、写文件去重（内容不变不重写、保持 mtime，减少下游 cargo 重编译）。
- 降低下游编译成本：按 [`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md) §五 执行，依次为 rustc 分阶段测量 → 泛型擦除核心（先论证布局前提）→ decl / body 分层拆 crate。实测表明按包 / 按 SCC 直接拆不可行（95% 文件在同一个环里）。
- `emitter-perf2` 进展（详见 [`2026-09-30-emitter-performance.md`](2026-09-30-emitter-performance.md) §4.4–4.7）：
  - 已做：引用字段协议合并、clinit 去泛型、存根改调共享冷函数。HelloWorld java_runtime 的 mono size_est −24.2%，rustc −10%，峰值 RSS −14%。
  - 已做：`panic = "abort"`，panic 钩子保持退出码 101。mono −25.6%，二进制约 −25%，HelloWorld user 时间约 −10%。
  - `overflow-checks = false` 实测无收益，不做；手写层的 Java 整数运算已全部显式化。
  - 按类并行发射：✅ 已做（2a800b4b），DeepCopy 冷写出 2.10 → 1.08 s，峰值 RSS 314 → 396 MB，输出与串行逐字节一致。
  - 以上 ✅ 已合入 rust-closure-analyzer（f6d80103，e2e 抽查 7 例全过，二进制约 −40%）。待做：N6 剩余串行段（约 460 ms）、rustc 分阶段测量（拆 crate 方案 §五 第 1 步）。
- `emitter-final`（生成器补齐）✅ 已合入（ef1daf34，e2e 抽查 10 例全过，含 DeepCopy / CollectorsDemo 修复）：S1–S6 清零、CLI 选项与审计补齐、缺省路径不再导入 `codegen`（[`2026-10-01-codegen-dependency-inventory.md`](2026-10-01-codegen-dependency-inventory.md)）；删除清单见 [`2026-10-01-python-generator-deletion.md`](2026-10-01-python-generator-deletion.md)。
- 拆 crate S4（声明层 / 实现层，`emitter-perf2`）✅ 已合入（55c5dacc，e2e 抽查 11 例全过）；未达标项：Digester 声明 crate 峰值 4.51 GB（目标 ≤ 2 GB）、HelloWorld 构建 15.9 s（目标 ≤ 12 s），由 S5（声明层剥体）继续，见生成器效率计划 §7.5–7.7。
- S5（声明层剥体，生成器与宏共用判定 `runtime/rava_macros_core`）✅ 已合入（0f21a9da，e2e 抽查 13 例全过）：与 S4 等价（27 例声明层逐记号一致、实现层逐字节一致），宏展开 17.3 → 11.6 s，但峰值与总耗时在噪声内未变；Digester 声明 crate 3.1–3.4 GB、HelloWorld cargo 16.9 s，仍未达标。
- 终态达标账（生成器效率计划 §7.5.4，853b3cec）：峰值 ≈ 0.72 GB + 0.048 GB × 声明 crate 宏展开体量（MB）。峰值在前端各阶段按代码量逐级累积（Digester 宏展开后 1.40 GB → HIR 2.15 GB → 类型检查 3.17 GB → 单态化起点 3.59 GB），不是单态化收集独占。Digester 展开现为 56.5 MB，≤ 2 GB 须压到 ≤ 26.7 MB。候选手段：

  | 项 | 内容 | 预计削减（展开 MB） |
  |---|---|---:|
  | V1 | ObjectVTable 表驱动视图：`__view_into` / `__view_as` / `__erased_vtable` 改为每类只判自身、逐级转交父类；接口载体臂改共享泛型 helper（第 1 步前提扫描 ✅，29 棵树 5178 类 bad=0） | −2.9 |
  | V2 | `From<Object>` / checkcast / `new` 按类收敛 | −2.0 |
  | V3 | 分层 `use` 表 | −2.5 |
  | V4 | 字段访问器收敛 | −0.8 |
  | V5 | S6 泛型擦除 | −1.9 |
  | V6 | 外壳去一层：声明层 3.06 万个 `__jb_*` / `_base` 包装函数改为模块级 extern 直调（该类占 11.5 MB） | −5.5 |
  | V7 | 去掉生成条目上的 doc 属性 | −3.0 |
  | 合计 | V1–V7 | −18.6 → 37.9 MB，约 2.5–2.6 GB，**不达标** |

- 剩余大头的根因：每个 Java 类都有自己的一整套 Rust 类型。每类基础设施约 18 MB；`__Shared<dyn X__VTable>` 每类一个类型，带出 std 按类实例化约 25 万 size_est（如 `Weak::drop` / `Arc::drop_slow` 各 ×1087、`Box::new` ×3123）。结构性候选 **S7「统一对象句柄 + 每类静态描述符」**：wrapper 改为 `repr(transparent)` 包一个与类无关的句柄，每类只生成一份 static 描述符（祖先表、接口表、字段布局），由 runtime 的非泛型实现读取。预计 Digester 展开约 24 MB、峰值约 1.85 GB，HelloWorld cargo 约 11 s。S7 并入跨测试编译复用方案统一评估（§三.5），待用户决策（Q3）。V1 的链式委托与 S7 的祖先表一致，不是过渡形态；V1 后续步骤在复用方案定稿前暂停。
- 验收：生成器自身优化（不改输出的）要求 27 例生成树与改前逐字节一致；降低下游编译成本的生成形态改造（Q1 已允许）要求 e2e 通过，生成树差异只含预期改动。

### 4. 生成代码运行性能（任务 R1，待排期）

用户 2026-10-01 决定：运行超时的算法用例，只要 Java 写法合法，就不改测试文件，也不放宽 `RUN_TIMEOUT`（300 s）。超时按性能问题处理，归入本线，原用例作为验收用例。

**标杆用例**：`tests/e2e/23_algorithms/LynchBell.java`（期望输出 `Number found: 9867312`）。

- **负载**：`i` 从 98764321 递减到 9867312，共约 8890 万次迭代。每次迭代执行 `String.valueOf(i)`，新分配一个 8 位字符串，赋给静态字段 `s`；`uniqueDigits` 再在双重循环里反复调用 `s.length()` 和 `s.charAt()`。
- **JVM**：JIT 内联加逃逸分析后，每次迭代在几十纳秒量级，整程序几秒完成。
- **rava 现状**：分布式跑批以 debug 构建运行，没有内联；运行段超过 300 s 超时。按代码逻辑，热点有四处：
  1. `charAt` / `length` 逐层走字节码翻译链（`isLatin1` → `StringLatin1.charAt` → `checkIndex`），每层都是一次独立调用；
  2. 每次访问静态字段 `s` 都要做类初始化检查；
  3. `String.valueOf` 每次都分配 `byte[]` 和 `String`，带 Rc 引用计数；
  4. 实例调用都经过 vtable 分派。
- **方向**：在生成器侧降低热路径的调用与检查成本，例如对已证明完成初始化的类省去重复的初始化检查、final / 私有 / 静态调用直接调用并标注内联、热点小方法生成 `#[inline]`、减少临时对象分配。具体手段由实测剖析决定。约束：不手写替代 JDK 方法，性能替换不是手写理由（handwritten-boundary.md §一）。
- **验收**：LynchBell 在 debug 构建下运行段 ≤ 30 s，输出与 JVM 一致；同时 e2e 不回归。后续发现的同类超时用例并入本任务，作为附加验收用例。

**附加验收用例**（2026-10-01 分布式跑批运行段 > 300 s 超时；三例写法都合法，JVM 上都能正常结束）：

| 用例 | 负载（按代码逻辑） | 主要热点 |
|---|---|---|
| `23_algorithms/Factorion.java` | 4 种进制 × 149 万次迭代，约 600 万次 | 每次迭代：`String.valueOf` + `Integer.parseInt` + `fromDeci`（StringBuilder 追加、`reverse`、`new String`）；每位数字再调一次 `String.valueOf(char)` + `parseInt` + 最深 12 层的递归 `factorialRec`。合计约数亿次小调用和数千万次字符串分配 |
| `23_algorithms/FWord.java` | 第 37 个 Fibonacci 词长 2416 万字符，37 个词累计约 6300 万字符 | `entropy` 对每个字符执行 `HashMap<Character, Integer>` 的 `containsKey` / `get` / `put`，涉及装箱、哈希、equals；拼接也要复制同样量级的字符 |
| `23_algorithms/FibonacciMatrixExponentiation.java` | `fib(10^7)` 约 209 万位十进制（约 690 万比特） | 大数 `BigInteger.multiply`（Toom-Cook 路径）对 `int[]` 做大量逐元素运算；`toString` 走递归进制转换。debug 构建下每次数组访问都有越界检查、无向量化。JVM 上也要秒级 |
| `23_algorithms/IQPuzzle.java` | 15 孔三角跳棋，15 个起始空位逐一做全树深度优先搜索，遍历全部合法走法序列（千万级节点） | 每个节点 `new Puzzle`、复制 `boolean[16]`，并逐个 `new Move` 复制走法历史（最深 13 步）；`getValidMoves` 每次新建 `ArrayList`，并查 `HashMap<Integer, List<Move>>`（装箱）；`Stack` 进出。合计上亿次对象分配和虚调用 |
| `23_algorithms/FourIsTheNumberOfLetters.java` | 依次生成 201、10³ … 10⁷ 个词的自指句子，累计约 1111 万词 | 每个句段调用 `numToString` 递归拼接，`toOrdinal` 做 `split` / `HashMap` 查询 / `substring`；每个词做两次 `replace`、一次 `split`。合计数百万次字符串分配与拷贝 |

**2026-10-01 全量新增超时用例**（同样运行段 > 300 s，负载与热点待按代码逻辑分析后补入上表）：`PartitionInteger`、`PrimorialNumbers`、`RailwayCircuit`、`SelfNumbers`、`UnprimeableNumbers`、`WeirdNumbers`（均在 `23_algorithms/`）。

终态：以上十二例（含 LynchBell）在 debug 构建下运行段都 ≤ 30 s，且输出与 JVM 一致。

**外部参照终态（2026-10-04 用户采纳）**：十二例在 `rava build --release` 下的运行段 ≤ 同机 GraalVM native-image（无 PGO）的运行时间，例如 LynchBell ≤ 3.62 s、Factorion ≤ 1.59 s、FractionReduction ≤ 9.00 s。逐例阈值见 [`docs/reports/2026-10-04-graalvm-baseline.md`](../reports/2026-10-04-graalvm-baseline.md) §二，复现用 `scripts/graalvm_bench.sh`。上面 debug ≤ 30 s 的终态同时保留。起点基线是服务器作业 `timing-rel-10795076`（release）和抽查 `timing-dbg-10795076`（debug）记录的逐例耗时。

#### R1 实施记录（2026-10-05 起，分支 `r1-perf`）

**起点**（服务器 Linux x86；release 取作业 `timing-rel-10795076`，debug 取抽查 `timing-dbg-10795076`，GraalVM 取 `gvm-linux-bd52b537`；单位秒）：

| 用例 | rava release 运行段 | rava debug 运行段 | GVM Linux ni | 阈值（报告 §二） |
|---|---|---|---|---|
| LynchBell | 147.3（复跑 141.6 / 150.6） | > 300 | 失败（JVM 1.59） | 3.62 |
| Factorion | 54.5 | > 300 | 1.74 | 1.59 |
| FWord | 编译失败（OOM，11.9 GB） | > 300 | 1.60 | 1.37 |
| FibonacciMatrixExponentiation | 编译失败（OOM） | > 300 | 8.87 | 6.16 |
| IQPuzzle | 编译失败 | > 300 | 4.86 | 3.98 |
| FourIsTheNumberOfLetters | 编译失败 | > 300 | 3.18 | 2.77 |
| FractionReduction | 编译失败 | > 300 | 12.75 | 9.00 |
| PartitionInteger | 运行 OOM-kill（1m34s，峰值 19.8 GB） | > 300 | 0.85 | 1.36 |
| PrimorialNumbers | 编译失败 | > 300 | 失败（JVM 3.93） | 3.74 |
| RailwayCircuit | 编译失败 | > 300 | 失败（JVM 0.81） | 1.88 |
| SelfNumbers | 84.5 | > 300 | 2.54 | 2.27 |
| UnprimeableNumbers | 编译失败 | > 300 | 4.10 | 4.60 |

release 下 LynchBell 每次迭代约 1.7 µs，需提速约 40 倍；debug 需 10 倍以上。release 编译失败的 9 例属 release 构建资源问题（fat LTO 单 codegen-unit 峰值内存），不在本线的方法体翻译范围内，另记。

**剖析手段**：服务器 `perf` 存在但 `perf_event_paranoid=4`，无特权不可用（作业 `r1p-prof-rel-6f93f1c6` 产出空数据）；不改系统参数，改用 `scripts/runprof/`（LD_PRELOAD 的 SIGPROF 采样器 + addr2line 符号化，`prof.sh <Test> release|debug 秒数`）。

**热点分类（代码通读，LynchBell 一次 `s.charAt(l)` 的 release 路径）**：

| 类别 | 现形态 | 每次 `charAt` 的次数 | 归属 |
|---|---|---|---|
| 静态字段读 | `X::f()?` = 类初始化检查（OnceLock + 原子读）+ `__RefSlot<Option<T>>` 读锁 + 克隆；基本类型静态（`COMPACT_STRINGS`）同样走读锁 | 3（`s`、`COMPACT_STRINGS`、`SIOOBE_FORMATTER`） | 静态存储形态：S7-3 区 |
| 实例字段读 | wrapper `__get_f()` → `vt()` 动态调用 → 每字段一个 `Arc<RwLock<Option<Box<T>>>>` 读锁 + 克隆；基本类型字段 `Arc<AtomicU64>` SeqCst | 2–3（`value` ×2、`coder`） | 对象存储与访问器形态：S7-3 区 |
| 数组访问 | `JArray` = `Arc<Repr>`，`get` / `set` / `len` 每次取 `RwLock` 读锁 | 2（`len`、`get`） | 运行时 `array.rs` |
| 引用计数 | 实参 / 临时值 `Clone::clone(&..)`：调用结果、getter 结果也再克隆一次 | 3–4 对原子增减 | 方法体翻译（本线） |
| 类初始化检查 | 每个静态方法入口 `Self::__class_init()?`，静态字段 getter 内再查一次 | 4（`StringLatin1.charAt`、`String.checkIndex`、`Preconditions.checkIndex` 及 getter） | 宏 `class_init.rs`：S7-3 同文件 |
| 栈界检查 | 非叶子方法入口 `__stack_check()` → `rava_coro::stack_exhausted()`（`#[inline(never)]`，TLS） | 4 | T6 区 |
| 虚 / 接口分派 | final 类（String）实例方法仍经 wrapper → 下沉体函数；字段访问经 `dyn` vtable | 每次字段读 1 | S7-3 区（访问器） |
| 字符串构造 | `String.valueOf(int)`：`byte[]` + String 对象，对象每字段一个 Arc 分配（约 9 次分配） | 每次迭代 1 组 | 对象存储形态：S7-3 区 |
| 异常 / 溢出 | `checkIndex` 走完整 `Preconditions` 字节码；整数运算 `wrapping_*` 无额外开销 | — | 不需改 |

**剖析实测**（作业 `r1p-prof5-2ac19d8c`，Factorion release，jp1，250 Hz 采样 10000 个样本，自耗按类归并）：

| 类别 | 自耗占比 | 主要符号 |
|---|---|---|
| 分配 / 释放 | ≈ 24% | libc（malloc / free）15.4%、`Arc::drop_slow` 3.4%、drop_glue 3.9%、alloc / dealloc |
| 实例字段访问器 | ≈ 10% | `__get_buf` 6.6%（RwLock 读 + Arc 克隆）、`__get_coder` / `__get_value` / `__set_*` / `__as_*` |
| 数组访问 | ≈ 10% | `JArray::get` 4.4%、`set` 3.0%、`arraycopy` 2.2%（RwLock） |
| 静态字段读 + 类初始化检查 | ≈ 7.5% | `__class_init` 3.4%、`SIOOBE_FORMATTER` 3.2%、`DigitOnes` / `DigitTens` |
| 栈界检查 | ≈ 4.5% | `rava_coro::stack_exhausted` 3.6%（不内联，TLS）、`__stack_check` 1.0% |
| JDK 方法体本身 | 其余 | `StringLatin1.charAt` 5.2%、`String.isLatin1` 3.5%、`String.length`、`Integer.parseInt` 等 |

LynchBell debug（同作业 kr1，10000 样本，含子调用口径）：`uniqueDigits` 83.6%，其中 `String.charAt` 56.4%。静态字段 getter 合计约 34%（`s` 13.9%、`SIOOBE_FORMATTER` 10.2%、`COMPACT_STRINGS` 9.9%），内部是 `__class_init` 8.7% 和 OnceLock `force` 8.4%；`__RefSlot` 读锁 `borrow` / `read_recursive` 20%；栈界检查 8.8%（`guard::current` TLS 6.2%）；`String.valueOf` 13.4%。debug 下 parking_lot 的 `try_lock_shared_fast` / `deadlock_acquire` / `checked_add` 等以 opt-level 0 编译，单把读锁展开成十余层调用。

LynchBell release（同作业 jp1，10000 样本，自耗）：静态 getter `s` 9.9%、`SIOOBE_FORMATTER` 5.4%、`__class_init` 3.3%、`DigitTens` / `DigitOnes` 3.1%，合计约 22%；`__get_buf` 8.7%、`__get_value` / `__get_coder` 等字段访问器约 10%；`JArray::get` / `set` 9.3%；`Result` 的 `?`（`Try::branch`，result.rs:2176-2177）7.4%，几乎全部来自 `main` 循环（Throwable 结果按值搬运）；`stack_exhausted` + `__stack_check` 3.1%；`try_lock_shared_fast` 2.4%。含子调用：`uniqueDigits` 44.9%，其中 `String.charAt` 38.3%（`checkIndex` 7.2%、`isLatin1` 5.3%）；`Integer.toString` 19.7%。

结论：前四类（约 52%）都是对象 / 静态 / 数组存储的同步形态（每字段一个 `Arc`、每次读写一把 `RwLock`、每次读出克隆一份引用计数），属 S7-3 区与运行时 `array.rs`；方法体翻译侧能直接消除的是多余的引用计数增减（已做两项）。存储形态改造已向主会话申请协调（2026-10-05）。

**已做（本线范围内的方法体翻译）**：

1. `9b74cb28` 独占临时值不再克隆：Java 调用结果（`?`）、静态字段读、字段 getter 作实参 / checkcast 源时直接移交（`Expr::is_owned_temp`）。LynchBell 生成树 `Clone::clone(&` 6292 → 5974。
2. `e983141d` 单用临时值按值移交：`let _tN = e;` 后仅在紧随语句以 `Clone::clone(&_tN)` 出现一次时改为 `_tN`（循环头 / 闭包除外；同名重绑保守不改）。LynchBell 生成树 939 行受益。
3. `9bdd3095` 删除本类 static 字段的丢弃读：`let _ = Own::f()?;`（值已折叠、只为类初始化副作用保留）在本类代码中恒为空操作（JVMS §5.5，本类代码执行时本类已初始化或正由当前线程初始化），整行删去；他类读取不动。LynchBell 生成树删二十余行（`String.coder()` 的 `COMPACT_STRINGS` 读、各类 `$assertionsDisabled` 读）。
4. `79e4ccc4`（Q1(b)）本类静态方法 / 构造器 / `<clinit>` 内的本类 static 读写不再逐次查初始化状态：宏为每个 static 生成原始存取 `__si_<名>` / `__si_set_<名>`（只含安全点 + 存取），公开 getter / setter = `__class_init()?` + 原始存取；入口已注入 `Self::__class_init()?` 的方法体内，本类 `Own::f()` / `Own::set_f(v)` 改写为原始存取（闭包 / 嵌套项不改写；实例方法不改写——运行时手写可不经构造器造实例）。
5. `b4b6ccd1`（Q4）栈界检查快路径内联：§21.8.3 不允许编译器线程局部取址跨挂起点缓存，故不直接 `#[inline]` 原函数，改为 Linux x86_64 / aarch64 每次现读线程指针（非 `pure` 内联汇编，`fs:[0]` / `tpidr_el0`）+ 静态 TLS 偏移取栈界；偏移在线程入口与载体切入时核对，任一线程不一致即永久退回 `#[inline(never)]` 慢路径；其他平台只走慢路径。
6. （Q3）性能类测试构建档 `dev-opt`：生成的 workspace 增 `[profile.dev-opt]`（继承 dev，opt-level 1；`package.user` opt-level 0），`rava build / compile --dev-opt`（与 `--release` 互斥），e2e 用例以独占一行 `// rava-build-profile: dev-opt` 声明、`run_tests.py` 缺省档时按声明改走该档。dev 档保持 opt-level 0。档案 crate 跨测试共享缓存只付一次 opt 1 编译代价，用户 crate 每例重编保持 dev 速度。
7. `e9c8c5b9` 即上条 Q3 的提交。
8. `bc517e9e` 引用型常量（字符串 / 类字面量 / 拼接结果 / null）视为独占临时值：作实参与 checkcast 源时不再包 `Clone::clone(&..)`（`Clone::clone(&String::from("ha"))` → `String::from("ha")`）。
9. `a61f8dc0`（杠杆 ②）字符串常量逐调用点缓存：`java_class!` 展开时把方法体与 ConstantValue 初值中的 `String::from("…")` 改写为调用点私有 `OnceLock` 单元，首次经全局驻留表（按 `Vec<u16>` 哈希、持锁）取规范实例，此后只克隆同一实例（JVMS §5.4.3 常量池项解析一次）。不用 `get_or_init`（首次加载可能经类初始化重入同一调用点），并发 / 重入的各次加载取到同一驻留实例、先写入者留存。可读层源码不变。
10. `1697a9a8`（杠杆 ④）Unsafe 偏移与数组视图免查表：实例字段偏移反查改为按 id 稠密下标（`id / FIELD_SLOT - 1`）直取进程常驻登记项，`offset_slot` 返回 `&'static str`（原先每次访问哈希查表 + 两次 `String` 克隆 + `to_owned`），`field_of_offset` 不再线性扫描；数组协变视图（`Node[]` 等经 `__view_into` 擦除为 `Object[]`，Unsafe / VarHandle 引用访问器、`array_load_object` / `array_store_object` 每次都构造）的元素访问改为按元素类型单态化的函数指针，构造开销由 4 个闭包 + 视图 + 元素名探针降为源句柄 + 视图两次分配，长度经源数组钩子；`__view_into` 等只透传的擦除句柄改用进程共用单元，不再逐次 `Arc::new(())`。

**存储形态改造（分支 `r1-storage`，主会话协调后由本线承担）**：JMM 口径——普通字段只要求无撕裂、无数据竞争 UB（Relaxed 原子），volatile 字段 SeqCst，final 字段由构造完成后的发布（`Arc` 移交 / 类初始化状态发布）给出 happens-before；只用 `Atomic*`、原子指针与内联单一分配，不引入每字段锁。

11. `a58120d4` 对象字段内联单一分配：基本字段 `__PrimCell`（`AtomicU64` 位存储，普通字段 `get_plain` / `set_plain` 即 Relaxed、volatile SeqCst，祖先 volatile 经 `superclass_volatile_fields` 传递），引用字段 `__RefField<T>`（`AtomicBool` 自旋 + `UnsafeCell`，临界区只做克隆 / 替换，旧值在锁外释放）；去掉每字段 `Arc<RwLock<Option<Box<T>>>>`。wrapper 重建钩子经 `__Ref::from_storage` 共享同一存储（不再整份克隆），对象标识即存储地址；`__ClassDesc.offsets`（实现层 `offset_of!` 导出）供按名协议、反射、Unsafe 偏移、VarHandle、序列化与浅拷贝按偏移定位，手写层访问形态不变。
12. `28fd9be7`（Q1(a)）静态字段无锁：`__process_static!`（`OnceLock` + 读写锁单元）改为常量初始化的普通 `static`——基本类型 `__PrimCell::zeroed()`（`static volatile` 走 SeqCst，其余 Relaxed），引用类型 `__RefField<Option<T>>`；类初始化状态改为 `static __PrimCell<u8>`，已初始化快路径为一次原子读（`<clinit>` 写入对其他线程的可见性由状态发布的 SeqCst 写 / 读建立）。
13. `e63547cc`（Q2）数组无锁：`JArray<T>(Option<Arc<Repr<T>>>)`，null 数组为 `None` 不分配；`Store<T>` 基本元素为同宽原子单元（`AtomicU8/16/32/64::from_ptr`，get / set Relaxed，读改写 CAS SeqCst），引用元素逐元素 `__RefField` 内联；Unsafe / 原生内存按字节视图读写逐元素拼接（部分覆盖元素用 CAS 合并，单元素内的读改写为该元素 CAS，跨元素读改写取 16 路条带锁），bool 元素归一为 0 / 1。`RwLock<Vec<T>>` 与 `with_vec` 全部去除。
14. `31ebfe21`（杠杆 ③）类型化 null 每类单例：`Object::__typed_null_desc` 原先每次在 `RwLock<HashMap>`（`TYPED_NULLS`）下按名查表，现 `__ClassDesc` 增 `typed_null: fn() -> Object`，由生成的每类 `OnceLock` 承载（描述符本身仍是可 `const` 引用的常量，不加内部可变性）；接口载体的 `Default`（null 局部量 / 未写入接口槽）同样每接口一份 `OnceLock`。
15. `d28fd72e` `array/store.rs` 按手写层辅助目录约定书写（`use super::*;`、宿主 `use store::*;`，不写其他 `super::` 路径；`runtime_helpers_follow_convention` 守护）。11–15 经 gate-ec6cbe11 合入集成分支（`10314222`）。

**验证**：抽查 `r1s-spot-31ebfe21`（16 例，跑完 10 例时停止，gate 已覆盖）：通过 8 例（HelloWorld、DeepCopy、TestConcurrentClinit、TestReflectEnumOps、TestClassCastSubclass、ReflectionAPI、ConcurrentMapDemo 等）；失败 3 例都不是本线回归——LynchBell / Factorion 是 debug 档运行超时（> 300 s，起点 `10795076`、`e983141d` 两轮抽查同样超时），StockTrans 是 S7-3c `serialVersionUID 无字段闭包` 存根（集成抽查 `int-a04e67a0`、`s73d-1150fc04` 同样失败，另线修复中）。单测 `r1s-ut-31ebfe21` 的 closure 单测 1 例失败（辅助目录约定，即第 15 条），修后由 gate 覆盖。

**计时**（作业 `r1s-time2-d28fd72e`，sg2，`d28fd72e`；作业 60 min 超时，只跑完改造后的前 5 例，同机基线段未跑）：

| 用例 | 档 | 构建 | 运行 | 二进制 | 参照 |
|---|---|---|---|---|---|
| LynchBell | release | 2 m 22.7 s | 66.5 s | 7.2 MB | 起点 147.3 s；`1697a9a8` 116.4 s（不同作业） |
| Factorion | release | 2 m 20.2 s | 28.8 s | 7.2 MB | 起点 54.5 s；`1697a9a8` 42.1 s（不同作业） |
| TestVirtualThreadScale | dev-opt | 9 m 05 s | 7.41 s | 428.1 MB | 折合 (7.41 − 2) s / 1e5 ≈ 54 µs/VT，**未达 ≤ 20 µs 目标** |
| SelfNumbers | dev-opt | 8 m 55 s | 207.9 s | 421.4 MB | 起点 release 84.5 s（档不同，不可直接比） |
| FibonacciMatrixExponentiation | dev-opt | 8 m 45 s | 172.5 s | 421.4 MB | 起点 release 编译失败 |

同机前后对照：基线作业 `r1s-timebase-2602f409`（sg2，`2602f409`，与上表同 5 例同档，作业超时 5400 s）在途，由主会话接手判读。判读口径：同一服务器两段的运行段之比；release 两例（D8 合入前只计这两例）以运行段为准，dev-opt 例与 VT/µs 只作同机前后比较，不与 release 起点比。IQPuzzle、FourIsTheNumberOfLetters、PrimorialNumbers、RailwayCircuit、UnprimeableNumbers 五例的 dev-opt 前后对照没跑（作业超时）。

**同机基线对照**（`r1s-timebase-2602f409` 对 `r1s-time2-d28fd72e`，均在 sg2，rc=0；比值 = r1 / 基线，运行段）：

| 用例 | 档 | 基线 `2602f409` | r1 `d28fd72e` | 比值 |
|---|---|---|---|---|
| LynchBell | release | 118.9 s | 66.5 s | 0.56 |
| Factorion | release | 44.8 s | 28.8 s | 0.64 |
| TestVirtualThreadScale | dev-opt | 10.37 s（≈ 84 µs/VT） | 7.41 s（≈ 54 µs/VT） | 0.71 |
| SelfNumbers | dev-opt | 156.3 s | 207.9 s | **1.33** |
| FibonacciMatrixExponentiation | dev-opt | 245.1 s | 172.5 s | 0.70 |

- 基线上 TestVirtualThreadScale 输出有 2 行差异（基线自身问题，r1 输出一致）。
- 二进制：release 7.6 → 7.2 MB；dev-opt 428.0 / 421.1 → 428.1 / 421.4 MB（持平）。
- 构建耗时基本持平：release 2 m 19–21 s → 2 m 20–23 s；dev-opt 8 m 31–51 s → 8 m 45 s–9 m 05 s。
- SelfNumbers dev-opt 慢 33%，单次样本：转入 R1 续（`r1-next`）先同机复跑确认，属实则剖析（疑点：opt-level 1 下内联字段自旋单元 / 数组元素访问未内联），修到不慢于基线。

#### R1 续（分支 `r1-next`，2026-10-05）

**已做**（16–19 经 gate-a826aae7 合入集成分支）：

16. `fecb1a7a`（第 2 项）null 改为不计数的静态哨兵：新增对象指针 `__Obj`（16 字节头 + 值，静态哨兵以指针低位标记，克隆 / 释放只测一位）；`Object` / `__Handle` 改持 `__Obj`；`JVM_NULL` 与类 / 接口的类型化 null 均为 `static __TypedNull`，删去 `OnceLock` 单例。
17. `4d937120`（第 3 项）数组单一分配：`JArray<T>` 持 `__Obj<__ArrayObj<T>>`，元素紧随数组对象、同一分配（三次分配减为一次）；数组装入 `Object` 只换指针类型、不分配；null 数组装入 `Object` 为 `__ArrayNull<T>` 静态哨兵。
18. `4b06af56`（第 4 项）删 `with_vec` 残留：手写扫描的改写元素方法名单与单测随删。
19. `cad1b41c` 基本类型数组按默认值创建改为清零分配（`alloc_zeroed`）：`newarray` 不再逐元素经迭代器写入。SelfNumbers 的 10⁹ 元素 `boolean[]` 逐元素初始化原占运行段 15.5%。
20. `5f87b295` 基本元素数组存取与静态引用字段读的运行时链标 `#[inline(always)]`（见下文 SelfNumbers 剖析）：`__ArrayObj::get` / `set` / `len` 增自有基本元素快路径（界检 + 整数地址运算取槽 + 同宽原子读写）；`JArray::get` / `set` / `len` / `clone`、`__RefField` 读路径（不经 `with` 闭包）、`__PrimCell` 读、`__AtomicRepr` 位形转换、`__Obj` 内部小函数、`safepoint` 同改。

**SelfNumbers dev-opt 变慢：复核与剖析**

- 同机复跑确认属实：sg2 上基线 `2602f409` 复跑（`r1n-selfbase-2602f409`）运行段 153.1 s，与上轮 156.3 s 一致；`dbe90052`（第 2–4 项后）为 291.6 s（`r1n-time-dbe90052`），慢约 1.9 倍。
- 剖析（`r1n-snprof-dbe90052`，sg1，dev-opt）：含子调用口径数组 `get` 42.8%、`set` 27.2%、静态 getter `SV()` 20.9%（其中自旋锁 14.2%）、`<clinit>` 里 `try_new` 的逐元素初始化 15.5%。自耗分散在未优化的小调用上：`Take::next` 9.6%、`compare_exchange_weak` 13%、`from_raw_parts` 的前置检查（ub_checks）、`ok_or_else`、`form` / `own` 等。
- 根因：dev-opt 档的用户 crate 以 opt-level 0 编译（上文第 6 条）。运行时的存取函数都是 `#[inline]` 泛型，在用户 crate 里按 opt-level 0 单态化，既不内联也不消去 ub_checks，每次元素存取展开成二十余层调用。share-generics 只复用上游 crate 的非 `#[inline]` 实例，所以上游按 opt-level 1 编好的实例用不上。存储改造把单次存取拆成了更多层小函数（`obj` → `form` → `Store::get` → `index` → `load_bits` → `from_ptr` / `load` → `from_bits`），在 opt-level 0 下层数正是成本；JDK 侧（opt-level 1）不受影响，FibonacciMatrixExponentiation 同期 0.70 倍即为反例。
- 用户 crate opt-level 对照（`r1n-uopt-d5ef6f58`，kr1，同一 scratch 只改 `[profile.dev-opt.package.user]`，`scripts/runprof/user_opt_cmp.sh`）：

| 用例 | 用户 crate | 构建 | 运行 | 二进制 |
|---|---|---|---|---|
| SelfNumbers | opt 0 | 427.4 s | 268.8 s | 364.4 MB |
| SelfNumbers | opt 1 | 428.4 s | **23.5 s** | 363.9 MB |
| FibonacciMatrixExponentiation | opt 0 | 425.5 s | 214.3 s | 364.6 MB |
| FibonacciMatrixExponentiation | opt 1 | 427.5 s | 214.9 s | 364.1 MB |
| TestVirtualThreadScale | opt 0 | 432.2 s | 6.71 s | 369.6 MB |
| TestVirtualThreadScale | opt 1 | 435.0 s | 6.14 s | 369.1 MB |

  用户 crate 改 opt-level 1 后，SelfNumbers 运行段降为原来的 1/11，构建耗时与二进制大小持平：用户 crate 只有几个类，opt 1 的编译增量在秒级，被档案 crate 的编译量淹没。负载在 JDK 侧的两例不受影响。
- 运行时侧修复（第 20 条，用户 crate 保持 opt-level 0）：基本元素数组的读、写、取长，以及静态引用字段的读，在用户 crate 里展开为少量指令，只剩 std 的原子读写与 `ptr::read` 等几次调用。实测见下表。

第 20 条的实测作业已发起，结果出来后补表：
- `r1n-sn-5f87b295`（sg2，SelfNumbers dev-opt）：与同机基线 153.1 s、`dbe90052` 291.6 s 对照。
- `r1n-time4-5f87b295`（sg2）：其余 4 例 dev-opt。
- `r1n-time-5f87b295`（kr1，5 例 dev-opt）：与 kr1 上 `d5ef6f58` 用户 opt 0 的 268.8 s 对照。
- `r1n-uoptvt-5f87b295`（sg1）：TestVirtualThreadScale 与 SelfNumbers 在用户 crate opt 0 / 1 下对照，并打印输出。`r1n-uopt-d5ef6f58` 里 TestVirtualThreadScale 的 opt 0 / 1 输出 md5 不同，要据此判断差异来自时序（`all sleeping at once` 取决于 10⁵ 个虚拟线程能否在 2 s 内全部起跑），还是 opt 1 下行为不同。

**前后对照**（`d28fd72e` 为 r1-next 之前，sg2，作业 `r1s-time2-d28fd72e`；`dbe90052` 为第 16–18 条之后，sg2，作业 `r1n-time-dbe90052`；运行段，dev-opt）：

| 用例 | `d28fd72e` | `dbe90052` | 构建（`dbe90052`） | 二进制（`dbe90052`） |
|---|---|---|---|---|
| SelfNumbers | 207.9 s | 291.6 s | 7 m 31 s | 344.5 MB |
| TestVirtualThreadScale | 7.41 s | 7.34 s | 7 m 36 s | 349.5 MB |
| FibonacciMatrixExponentiation | 172.5 s | 221.0 s | 7 m 30 s | 344.7 MB |
| LynchBell | —（上轮为 release） | 235.6 s | 1 m 07 s | 50.1 MB |
| Factorion | —（上轮为 release） | 50.3 s | 1 m 06 s | 50.1 MB |

- 二进制 421 MB → 345 MB（−18%）。
- 构建 8 m 45 s–9 m 05 s → 7 m 30 s 左右（−15%）。
- FibonacciMatrixExponentiation 慢 28%。同例在 kr1 的 `d5ef6f58` 上为 214 s，在 sg2 的基线 `2602f409` 上为 245.1 s，单次样本，待 `r1n-time4-5f87b295` 复核。
- 原定的「前」作业 `r1n-time-6a3c051e` 在 sg2 排队约 1.5 小时未启动，已撤下，让位给 `r1n-sn-5f87b295`。

**待决**（交主会话 / 用户）：dev-opt 档用户 crate 的 opt-level 由 0 改为 1。上文第 6 条 Q3 定为 0，理由是「用户 crate 每例重编保持 dev 速度」；上表实测构建耗时与二进制持平，SelfNumbers 运行段降为 1/11。运行时侧（第 20 条）只能压缩运行时自身的调用层，用户方法体里的 `?`、`wrapping_add`、引用计数增减等在 opt-level 0 下仍逐个是调用。改用 opt-level 1 是一行档位配置（生成的 workspace `Cargo.toml`），不涉及生成器逻辑。

**TestVirtualThreadScale 剖析**（`r1n-vtprof2-dbe90052`，kr1，8 核；运行 6.23 s，折合约 42 µs/VT，目标 ≤ 20 µs）：

- 串行瓶颈有两处：
  - 主线程逐个创建虚拟线程，约占 CPU 的 18%（`VirtualThreadBuilder.start` 12.4%）；
  - 唯一的 unparker 线程处理延迟队列 `DelayedWorkQueue` 的 take / finishPoll / siftDown，约 20%。
- siftDown 里的 `ScheduledFutureTask.compareTo` 占 9.5%。这一项来自生成代码的按名类型判定：`is_instance_of("…/ScheduledThreadPoolExecutor$ScheduledFutureTask")` 和 `try_cast` 在 `__ClassDesc.supertypes` 上二分查找名字字符串（`memcmp` 2.9%），再加上 `Object::from(Clone::clone(..))` 的引用计数增减和 `type_id` 比较。
- libc 占 24%：释放 6%、parking_lot `unlock_slow` 5.5%、futex 等待 4.3%。
- 杠杆（未实施）：
  - 生成器对类目标的 instanceof / checkcast 改发描述符判定 `__ClassDesc::is_subclass_of`（O(1) display 表比较，不比名字字符串）；
  - 删去 compareTo 等热点里的多余克隆。
  
  前者要同步改闭包分析器的手写守卫识别（`handwritten/guards.rs` 按 `is_instance_of("…")` 形态识别），单独成步。剩余两处串行段属于 JDK 字节码本身的结构（单 unparker、主线程串行创建），只能压低每步的常数。预计上述两项合计约 10–15%，离 ≤ 20 µs 仍差约 2 倍。

**余项**：
- TestVirtualThreadScale 约 42 µs/VT（kr1，`dbe90052`），离 ≤ 20 µs 还差约 2 倍。杠杆见上，其余是 `Thread` / Continuation 对象构造的分配与释放。
- 闭包分析器 `handwritten/syntax.rs` 的 `ELEMENT_MUTATORS` 已随第 18 条删去 `with_vec`。

### 5. 测试流程效率（用户 2026-10-01）

2026-10-01 分布式全量：1083 例、7 台服务器、约 5 h 墙钟、约 35 机时。按本地抽查比例（13 例：编译 7 m 08 s / 共 9 m 44 s）推算：cargo 编译约 70–75%，转译约 20%，超时空等约 1.5 机时（约 4%），运行约 3%。

| # | 方向 | 收益面 | 状态 |
|---|---|---|---|
| T1 | **跨测试编译复用**（最关键）：同一 JDK 类在上千个测试里各编一遍，是机时的主要去处。终态要让同内容只编一次：(a) 类的翻译与测试无关，可达性只在链接 / 分派层面决定；或 (b) 按内容寻址的 crate / 产物缓存，含 7 台服务器之间的缓存共享。须与 CLAUDE.md 第 2 条（调用链外方法生成存根）和第 1 条（手写边界）协调，原则若需调整交用户决策 | 总机时中的编译部分 | perf2 编写方案中（[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md)），与 S7 统一评估 |
| T2 | 降低单例编译成本：S 系列拆层、V1–V7、S7（§三.3） | 每例编译时间与峰值 | S4、S5 已合入；其余见 §三.3 |
| T3 | 超时用例修复：12 例每例空等 5 min | 约 1.5 机时 | R1（§三.4） |
| T4 | 服务器拉新提交后首例转译约 2 min（推断为重编 `rava` 生成器本身，待服务器核实） | 每台每提交约 2 min | 待核实；可考虑每个提交只构建一次生成器再分发 |
| T5 | 派发按历史耗时降序（LPT）：只缩短收尾长尾（约 10 min，约 3%），不减总机时 | 收尾阶段 | 用户 2026-10-01 决定不作为优化重点，暂不做 |


## 四、待用户决策

| # | 事项 | 现状 |
|---|---|---|
| Q1 | 为了降低下游 cargo 编译成本，是否允许 Rust 生成器的生成形态偏离 Python 基线 | ✅ 允许（2026-10-01）：降编译成本的生成形态改造纳入生成器效率线实施，验收改为 e2e 通过 + 生成树差异只含预期改动 |
| Q2 | 缺省生成器切换为 rust 的时机 | ✅ 已切换（2026-10-01）。Python 生成器保留为对照基线（只读，不再投入），删除条件：① S1–S6 占位清零、CLI 选项与审计补齐、Python 依赖（jimage 提取、golden 转储等）迁完（✅ 2026-10-01，ef1daf34）；② rust 缺省下 JDK 21 全量 e2e 通过集合 ⊇ 冻结的 Python 基线（1029 例，`2026-10-01-python-baseline-jdk21.txt`；JDK 25 不设 Python 基线，2026-10-01 用户决定）。满足后由用户确认删除 |
| Q3 | 跨测试编译复用路线与 S7（统一对象句柄 + 每类静态描述符）是否实施、取哪条路线 | 待 perf2 复用方案定稿后由用户一次决策（§三.3、§三.5） |

## 五、执行约束（所有执行线共用）

- 各线使用独立 worktree 与分支（自 rust-closure-analyzer），每完成一步合并主线、自行解决冲突；由主会话审查后合入。
- 小步同步（用户 2026-10-01）：每完成一个可验证小步就提交、报告，主会话尽快合入并广播「集成分支已更新到 X」；开始新步前先合并集成分支最新版；拆文件 / 搬模块等结构性改动单独成提交、优先合入，期间其他执行线暂停合并。
- `CARGO_BUILD_JOBS=2`；执行者不跑 e2e（由主会话串行抽查）；同一时间只跑一个重进程；禁止 pkill / killall 按名杀进程。
- 生成器 / 分析器代码不得出现 JDK 类名字面量；精度改进不得新增手写代码或清单中的类名特判掩盖。

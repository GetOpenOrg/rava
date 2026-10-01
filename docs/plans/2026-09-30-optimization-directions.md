# 优化方向总览：闭包精度、闭包分析效率、生成器效率

> 日期：2026-09-30
> 性质：本文记录用户拍板的优化方向、验收口径和各执行线的分工，是总纲，执行细节以各子计划为准。
> 关联：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md)（C0–C5）、[`2026-09-29-boundary-narrowing.md`](2026-09-29-boundary-narrowing.md)（C1d 与精度线 §6.9）、[`2026-09-30-closure-analyzer-performance.md`](2026-09-30-closure-analyzer-performance.md)（闭包分析性能 P0–P8）、[`2026-09-30-rust-emitter.md`](2026-09-30-rust-emitter.md)（Rust 生成器）、[`2026-09-30-emitter-performance.md`](2026-09-30-emitter-performance.md)（生成器效率）。

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
| 闭包分析效率 | e2e 任一用例的墙钟 | DeepCopy 6.8 s（批量排空 + 精度三期，均已合入） | ≤ 10 s（HelloWorld ≤ 0.5 s） |
| 闭包分析效率 | e2e 任一用例的峰值 RSS | DeepCopy 2.3–2.8 GB | ≤ 1 GB |
| 闭包分析效率 | 跨测试缓存命中时单测试墙钟 | 无缓存 | ≤ 2 s |
| 生成器效率 | 发射阶段墙钟 / 峰值 RSS（任一用例） | P0 5.9 s / 342 MB → P5 1.99 s / 315 MB → 并行发射 1.02 s / 396 MB（DeepCopy 热写出；冷写出 1.08 s） | ≤ 2 s / ≤ 500 MB |
| 生成器效率 | 内容未变文件的重写次数（复用 scratch 时） | 0（P5） | 0 |
| 下游编译 | 单个 rustc 峰值 RSS | 约 14 GB（N8 实测，单 crate） | ≤ 2 GB（路径见 [`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md)） |
| 下游编译 | HelloWorld 编译墙钟 | 23.9 s（主线）/ 2 m 40 s（C1d） | ≤ 20 s；仅用户类变化时 JDK 部分零重编 |
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

终态：六例（含 LynchBell）在 debug 构建下运行段都 ≤ 30 s，且输出与 JVM 一致。

## 四、待用户决策

| # | 事项 | 现状 |
|---|---|---|
| Q1 | 为了降低下游 cargo 编译成本，是否允许 Rust 生成器的生成形态偏离 Python 基线 | ✅ 允许（2026-10-01）：降编译成本的生成形态改造纳入生成器效率线实施，验收改为 e2e 通过 + 生成树差异只含预期改动 |
| Q2 | 缺省生成器切换为 rust 的时机 | ✅ 已切换（2026-10-01）。Python 生成器保留为对照基线（只读，不再投入），删除条件：① S1–S6 占位清零、CLI 选项与审计补齐、Python 依赖（jimage 提取、golden 转储等）迁完（✅ 2026-10-01，ef1daf34）；② rust 缺省下 JDK 21 全量 e2e 通过集合 ⊇ 冻结的 Python 基线（1029 例，`2026-10-01-python-baseline-jdk21.txt`；JDK 25 不设 Python 基线，2026-10-01 用户决定）。满足后由用户确认删除 |

## 五、执行约束（所有执行线共用）

- 各线使用独立 worktree 与分支（自 rust-closure-analyzer），每完成一步合并主线、自行解决冲突；由主会话审查后合入。
- `CARGO_BUILD_JOBS=2`；执行者不跑 e2e（由主会话串行抽查）；同一时间只跑一个重进程；禁止 pkill / killall 按名杀进程。
- 生成器 / 分析器代码不得出现 JDK 类名字面量；精度改进不得新增手写代码或清单中的类名特判掩盖。

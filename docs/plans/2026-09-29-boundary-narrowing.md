# C1d 边界收窄：手写只留 VM 契约层

> 关联：`docs/plans/2026-09-29-rust-closure-analyzer.md`（§6.1 分工原则、C1c 精确分析）、`CLAUDE.md` 原则 0 / 3b、
> `runtime/java_runtime/closure.toml`、`docs/reports/2026-09-14-impl-strategy.md`（截断的原始规模数据）。
> 状态：⏳ 未开始。2026-09-29 用户确认方向；实测数据在 C1c 第 3 步（CPA）完成后采集。

## 一、目标

**默认字节码翻译，手写只留字节码表达不了的 VM 契约层。**

终态（量化）：

| 指标 | 现状 | 终态 |
|---|---:|---:|
| `closure.toml [boundary].packages` 前缀条目 | 5（`sun/` `jdk/` `com/sun/` `com/oracle/` `java/security/`） | 0，改为逐类的 VM 契约清单 |
| 边界手写 fn 中非 VM 契约者（`#[jvm_boundary]` 标注、运行时本可执行字节码的成员） | 435 个 fn 的待分类集合 | 0 |
| 放行包的动态对照翻译域漏覆盖 | — | 0 |
| 全量 e2e（JDK 21 + 25） | 现行通过集 | 不减少 |
| 验收集闭包规模（C1c 7 例） | C1c 终值 | 每例增量逐包记录；总类仍 ≤ 400、translate∩code ≤ 250 |

VM 契约层的定义（与 GraalVM native-image 只对 VM 层做 `@Substitute` 同思路）：

1. `ACC_NATIVE` 方法：`Unsafe`、`Class` 元数据、`Thread` / monitor、文件与系统调用；
2. VM 注入的状态与对象：类元数据表、反射对象构造（`getRecordComponents0` 等），以及 VM 直接写入的字段；
3. 运行模型替换：lambda / indy 引导、MethodHandle / LambdaForm，rava 用自有模型替代 JVM 实现。

不属于以上三类的手写代码，都是在复刻本可翻译的 Java 逻辑。

## 二、现状

内部包截断的原始理由是规模：跟随内部包调用链，类数 111 → 635（+470%）。但这个数字是 Python BFS 口径，
膨胀主要来自过近似（分析器计划第二节 M1–M9：CHA 接口分派、全局 RTA、根虚方法广播、签名类型递归……），
并不说明这些包的代码本身必须手写。精确分析落地后，截断收益需要按包重测。

手写层现状（2026-09-29，`runtime/java_runtime/src`，不含 mod.rs）：

| 范围 | 文件 | 行数 | `#[jvm_native]` | `#[jvm_boundary]` |
|---|---:|---:|---:|---:|
| `sun/` | 50 | 5404 | — | — |
| `jdk/` | 51 | 6291 | — | — |
| `java/security/` | 2 | 82 | — | — |
| 合计 | 103 | 11777 | 42 | 435 |

按包（文件数前 10）：

| 包 | 文件 | 行数 | native | boundary |
|---|---:|---:|---:|---:|
| `sun/nio/cs` | 12 | 772 | 0 | 27 |
| `jdk/internal/util` | 12 | 1222 | 12 | 84 |
| `sun/nio/fs` | 11 | 1848 | 10 | 74 |
| `jdk/internal/misc` | 8 | 1356 | 3 | 87 |
| `jdk/internal/reflect` | 7 | 487 | 12 | 2 |
| `sun/util/locale` | 6 | 177 | 0 | 36 |
| `sun/security/jca` | 5 | 264 | 0 | 10 |
| `jdk/internal/event` | 5 | 132 | 0 | 14 |
| `sun/nio/ch` | 4 | 370 | 0 | 18 |
| `jdk/internal/vm` | 4 | 154 | 0 | 14 |

435 个 `#[jvm_boundary]` fn 是手写复刻 Java 逻辑的主体，也是本阶段要分类的对象。

## 三、手写的代价（收窄的动机）

- 正确性：每个手写 fn 逐个复刻 JDK 语义，偏差只能靠 e2e 撞出。
- 分析可见性：分析器要靠 syn 推断 `__set_` 接收者类型、依赖人工声明的 upcalls；
  C1c 中 TestRecordComponents 的 `getAccessor` 漏报即出于手写体写入对分析不可见。
- JDK 升级：字节码翻译自动跟随，手写要逐个核对（JDK 21 → 25 的改名 / 签名变化）。
- 覆盖节奏：未手写即 panic 存根，覆盖面每扩大一步都要人工补。

## 四、方法

### 4.1 前置条件

- C1c 第 3 步（CPA）完成：放行实测必须在精确分析下做，否则增量被过近似放大，结论偏向保留截断。
- 分析器已按方法划分 `[vm_boundary]` 类（C1c 起执行）：手写按精确名提供者取手写效果，其余按字节码建模。

### 4.2 放行实测工具

`rava closure` 增加 `--release <前缀>`（可多次）：分析期把该前缀视同 `[release]` 放行（Domain::Translate），
不改清单文件。同一前缀下仍有手写提供的成员按 `provides` 取手写（与翻译类的共置手写同一规则）。

每个前缀在验收集（C1c 7 例 + FileIOTest）上各跑一次，记录：

- 总类 / translate∩code / 方法数 / 耗时相对基线的增量；
- 新增类的分布：前缀内 / 前缀外，前缀外的新增按所在包归类（判断是否引出新的边界穿透）；
- 放行后变为「按字节码执行」的原手写 fn 清单（即可删除候选）；
- 放行后触达的 native 方法清单（即必须保留或新增的手写）。

### 4.3 判定

每个包按实测归入三类：

| 结论 | 条件 | 处理 |
|---|---|---|
| 放行 | 增量在预算内（8 例合计新增类 ≤ 该包现有手写文件数 × 3，且不引出新的边界穿透） | 从前缀截断中移出，按字节码翻译；非 native 手写 fn 进入删除清单 |
| 部分放行 | 包内只有少数类承载 VM 契约 | 包整体放行，VM 契约类逐类列入 `[vm_boundary]` 式的类清单，按方法划分 |
| 保留 | 包内逻辑本身就是 VM 契约（native 密集、VM 直接驱动） | 逐类列入 VM 契约清单 |

预期保留：`jdk/internal/misc`（Unsafe / VM / Signal）、`jdk/internal/vm`、`jdk/internal/reflect` 中的访问器生成、
MethodHandle / LambdaForm 相关。预期放行候选：`sun/nio/cs`、`sun/util/locale`、`jdk/internal/util`、`sun/security/jca`、
`jdk/internal/event`。以实测为准。

### 4.4 清单形态变化

- `[boundary].packages` 前缀截断整体取消，改为 `[vm_contract].classes`：逐类列出的 VM 契约类（含嵌套类）。
  与现 `[vm_boundary]` 合并为同一语义：**按方法划分**（native / VM 内建 / 手写提供 → 手写，其余字节码）。
- `[release]` 放行清单随之删除（不再有前缀截断需要例外）。
- 生成器代码仍不出现类名（原则 4），全部边界知识只在清单里。

### 4.5 发射层与手写层

- 放行包的类由生成器按闭包翻译；同名手写 fn 中非 native 的删除（逐项经用户确认，不批量删）。
- 放行后首次触达、原来被截断的 native 方法：补 `_impl.rs` 手写（这是 VM 契约层的正当手写）。
- 每放行一个包独立提交：清单改动 + 删除的手写 + 补的 native + 该包实测数据。

## 五、步骤

0. 前置：C1c 第 3 步完成，采集 C1c 终值作为基线。
1. 实现 `--release <前缀>`；在验收集上对 §二 所列各包逐一放行实测，结果写入本文档 §六。
2. 按 §4.3 给出每个包的结论与删除候选清单，提交用户逐项确认。
3. 逐包实施（按增量从小到大）：清单改动 → 删除确认过的手写 → 补 native → 全量 e2e（JDK 21 + 25）→ 提交。
4. 所有包处理完后：`[boundary].packages` 删除、`[release]` 删除，清单改为 `[vm_contract]`；
   `CLAUDE.md` 原则 3b 与「内部包边界截断规则」表同步改写为按方法划分的 VM 契约语义。

## 六、实测数据

（C1c 第 3 步完成后采集。）

## 七、验收

- §一 终态表各项达标。
- 每个放行包：验收集动态对照翻译域漏覆盖 = 0；全量 e2e 不减少；raw-audit `non_native_overrides` = 0。
- `[raw-audit] vm_boundary_methods` 与新清单口径对齐：只统计 VM 契约类的手写成员。

## 八、风险与对策

| 风险 | 对策 |
|---|---|
| 放行后翻译面扩大，暴露发射层对内部类形态的翻译缺陷 | 逐包放行、独立提交；缺陷修生成器（原则 0），不回退为手写 |
| 放行包引出新的边界穿透（链式展开到其它截断包） | 4.2 记录前缀外新增分布；穿透目标包先于本包处理或同批处理 |
| 手写体承载了 JVM 语义之外的 rava 专有行为（如 StreamDecoder 以 Charset 重载直连解码） | 删除清单逐项确认时标注；属运行模型替换的归入 VM 契约第 3 类保留 |
| 精确分析的漏报在放行后被放大 | 放行实测同时跑动态对照；漏报先修分析器 |

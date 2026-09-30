# C1d 边界收窄：手写只留 VM 契约层

> 关联：`docs/plans/2026-09-29-rust-closure-analyzer.md`（§6.1 分工原则、C1c 精确分析）、`CLAUDE.md` 原则 0 / 3b、
> `runtime/java_runtime/closure.toml`、`docs/reports/2026-09-14-impl-strategy.md`（截断的原始规模数据）。
> 状态：🔄 第 1 步完成（`--release` 实测，§六）；第 2 步分类见 §6.4，删除候选待用户逐项确认。

## 一、目标

**默认字节码翻译，手写只留字节码表达不了的 VM 契约层。**

终态（量化）：

| 指标 | 现状 | 终态 |
|---|---:|---:|
| `closure.toml [boundary].packages` 前缀条目 | 5（`sun/` `jdk/` `com/sun/` `com/oracle/` `java/security/`） | 0，改为逐类的 VM 契约清单 |
| 边界手写 fn 中非 VM 契约者（`#[jvm_boundary]` 标注、运行时本可执行字节码的成员） | 435 个 fn 的待分类集合 | 0 |
| 放行包的动态对照翻译域漏覆盖 | — | 0 |
| 全量 e2e（JDK 21 + 25） | 现行通过集 | 不减少 |
| 验收集闭包规模（C1c 7 例） | C1c 终值 | 每例增量逐包记录；translate∩code ≤ 运行时下限 × 1.3（C1c 验收口径） |

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

- C1c 第 3b 步（流水线对象敏感）完成：放行实测必须在精确分析下做，否则增量被过近似放大，结论偏向保留截断。
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

0. 前置：C1c 第 3b 步完成，采集 C1c 终值作为基线。
1. 实现 `--release <前缀>`；在验收集上对 §二 所列各包逐一放行实测，结果写入本文档 §六。
2. 按 §4.3 给出每个包的结论与删除候选清单，提交用户逐项确认。
3. 逐包实施（按增量从小到大）：清单改动 → 删除确认过的手写 → 补 native → 全量 e2e（JDK 21 + 25）→ 提交。
4. 所有包处理完后：`[boundary].packages` 删除、`[release]` 删除，清单改为 `[vm_contract]`；
   `CLAUDE.md` 原则 3b 与「内部包边界截断规则」表同步改写为按方法划分的 VM 契约语义。

## 六、实测数据

2026-09-30 采集。基线 = C1c 终值（`build/closure/<t>.json`，C1c 耗时步提交 `1e09a9a4`）；每个前缀单独
`rava closure --release <前缀>`，验收集 8 例（HelloWorld / TestSwitchString / PatternSwitchTest / FileIOTest /
ChineseRemainderTheorem / TestStreamBasic / TestRecordComponents / CollectorsDemo，逐例增量按此顺序）。
嵌套前缀（`sun/util/` ⊃ `sun/util/locale/` ⊃ `sun/util/locale/provider/`）含其子包。

列说明：预算 = 手写文件数 × 3（§4.3）；Δ 为 8 例合计；前缀内 = 新增类中位于前缀内的去重数；
穿透 = 新增且仍属边界域（其它截断包 / `[vm_boundary]`）的类去重数；候选 = 基线 `handwritten:boundary`、
放行后不再按边界手写的方法（有手写 fn / 全部，差额是基线里无手写体的存根）；native = 放行后触达的
`ACC_NATIVE`；漏覆盖 = 动态对照（`-Xlog:class+load`，main 之后加载）中属前缀、却不在闭包的类；
耗时 = 8 例中最大 `elapsed_ms`（CollectorsDemo，机器负载 ≈ 4，只作量级参考）。

### 6.1 增量总表

| 前缀 | 文件 | 预算 | Δ类 | Δtranslate∩code | Δ方法 | 逐例 Δ类 | 前缀内 | 穿透 | 候选 | native | 漏覆盖 | 耗时 ms |
|---|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|---:|---:|
| `jdk/internal/access/` | 3 | 9 | +0 | +4 | +4 | 0/0/0/0/0/0/0/0 | 0 | 0 | 11/14 | 0 | 1 | 3198 |
| `jdk/internal/math/` | 3 | 9 | +0 | +0 | +0 | 0/0/0/0/0/0/0/0 | 0 | 0 | 8/8 | 0 | 1 | 2723 |
| `sun/security/action/` | 2 | 6 | +0 | +0 | +0 | 0/0/0/0/0/0/0/0 | 0 | 0 | 7/7 | 0 | 1 | 3081 |
| `jdk/internal/module/` | 1 | 3 | +1 | +2 | +4 | 0/0/0/0/0/0/0/1 | 1 | 0 | 0/2 | 0 | 0 | 3408 |
| `jdk/internal/perf/` | 1 | 3 | +2 | +3 | +8 | 0/0/0/0/0/0/0/2 | 2 | 0 | 6/8 | 1 | 0 | 3259 |
| `sun/security/util/` | 3 | 9 | +3 | +2 | +27 | 0/0/0/0/0/0/0/3 | 0 | 1 | 1/2 | 0 | 0 | 2782 |
| `sun/text/` | 0 | 0 | +1 | +1 | +1 | 0/0/0/0/0/0/0/1 | 0 | 1 | 0/1 | 0 | 0 | 2829 |
| `sun/invoke/util/` | 3 | 9 | +6 | +10 | +56 | 0/0/0/0/0/4/1/1 | 1 | 2 | 21/26 | 0 | 7 | 2684 |
| `sun/nio/cs/` | 12 | 36 | +27 | +56 | +164 | 1/1/1/1/0/0/20/3 | 3 | 4 | 21/28 | 0 | 1 | 2815 |
| `sun/util/locale/provider/` | 4 | 12 | +20 | +16 | +69 | 0/0/0/0/0/0/0/20 | 3 | 2 | 15/22 | 0 | 0 | 2970 |
| `sun/util/locale/` | 5 | 15 | +20 | +17 | +72 | 0/0/0/0/0/0/0/20 | 3 | 2 | 23/32 | 0 | 0 | 2885 |
| `sun/util/logging/` | 0 | 0 | +20 | +13 | +65 | 0/0/0/0/13/0/0/7 | 3 | 3 | 0/3 | 0 | 0 | 2850 |
| `sun/util/` | 6 | 18 | +37 | +30 | +135 | 0/0/0/0/13/0/0/24 | 7 | 3 | 23/44 | 0 | 0 | 3262 |
| `jdk/internal/vm/` | 4 | 12 | +31 | +38 | +119 | 5/5/5/4/3/3/3/3 | 3 | 1 | 6/13 | 2 | 0 | 2684 |
| `jdk/internal/ref/` | 3 | 9 | +50 | +45 | +235 | 0/0/0/40/0/0/0/10 | 5 | 8 | 5/6 | 0 | 0 | 2730 |
| `jdk/internal/org/objectweb/asm/` | 0 | 0 | +52 | +49 | +412 | 0/0/0/0/0/0/19/33 | 30 | 0 | 0/5 | 0 | 20 | 2844 |
| `jdk/internal/icu/` | 0 | 0 | +57 | +56 | +281 | 0/0/0/0/0/0/0/57 | 50 | 0 | 0/2 | 0 | 0 | 2825 |
| `jdk/internal/util/` | 11 | 33 | +135 | +90 | +323 | 5/5/5/4/4/3/56/53 | 5 | 16 | 21/34 | 0 | 5 | 2778 |
| `java/security/` | 2 | 6 | +188 | +137 | +1120 | 0/0/0/0/0/−4/−1/193 | 18 | 48 | 0/12 | 3 | 0 | 4177 |
| `jdk/internal/loader/` | 3 | 9 | +208 | +143 | +1326 | 0/0/0/0/0/0/20/188 | 22 | 45 | 4/9 | 1 | 0 | 3963 |
| `sun/reflect/generics/` | 1 | 3 | +6587 | +5504 | +45839 | 1007/1007/1005/966/963/903/688/48 | 39 | 74 | 0/6 | 0 | 0 | 3170 |
| `jdk/internal/misc/` | 9 | 27 | −128 | −68 | −900 | 2/2/2/2/1/1/−33/−105 | 1 | 1 | 55/74 | 26 | 2 | 2487 |

验收集未触达（8 例 Δ 全 0、候选 0）：`sun/nio/ch/`（4 文件）、`sun/nio/fs/`（11）、`jdk/internal/event/`（6）、
`sun/security/jca/`（5）、`jdk/internal/logger/`（1）、`jdk/internal/invoke/`（1）、`sun/reflect/misc/`（1）、
`sun/util/resources/`（1）、`jdk/internal/foreign/`、`sun/invoke/empty/`、`sun/util/spi/`、`sun/util/cldr/`、
`sun/security/provider/`（0）。验收集口径下无数据，需在全量 e2e 语料上重测（§6.5）。

### 6.2 增量成因

- **`jdk/internal/misc/` 负增量**：基线 open(Object) 的 5 个引入点里有 2 个是该包手写返回
  （`InternalLock.newLockOr`、`Unsafe.allocateUninitializedArray`）。手写返回对分析不透明，只能取 open(返回类型)；
  放行后按字节码建模更精确，CollectorsDemo −105 类（去掉的 `ArrayDeque` / `IdentityHashMap` / 流水线类在动态对照中
  均不加载）。这是「手写的代价」（§三 分析可见性）的直接实测。
- **大增量来自分析器精度缺口，不是包自身规模**。按 `--why` / `--flows @path` / `@openstat` 追到的三类：
  - **G1 静态转发不分调用点**：`AccessController.doPrivileged → executePrivileged → action.run()` 的 action 形参
    跨调用点合并，`AccessibleObject.<clinit>` 的特权块分派到 `ClassFileDumper$1.run` 等全部动作
    （`jdk/internal/util/` 的 `java/nio/file` / `sun/nio/fs` 穿透、`jdk/internal/loader/` 的 `URLClassPath` → `JarFile`）。
  - **G2 全局汇合点上的 open(Object) 广播**：`StringBuilder.append(Object)` / `String.valueOf` / `Objects.equals`
    的形参无上下文区分，open(Object) 一旦流入即对全部逃逸对象分派 `toString` / `equals`，逃逸集随之扩大
    （`sun/reflect/generics/`：`Reifier → ParameterizedTypeImpl.validateConstructorArguments → String.format` 起，
    HelloWorld open(Object) 节点 44 → 2266、展开 568 → 2950 类；`java/security/`：`Set12.contains → SocketPermission.equals`
    → `InetAddress`）。
  - **G3 native 返回数组的元素取 open**：`Class.getEnclosingMethod0` 的 `Object[]` 元素按 open(Object) 进入
    `Class$EnclosingMethodInfo.<init>`，是 G2 的一个引入源；VM 实际只写入 `{Class, String, String}`。
  - 真实路径（非精度问题）：`jdk/internal/ref/` 在 FileIOTest 的 +40 来自 `FileCleanable.register → CleanerFactory
    → Cleaner`；`jdk/internal/icu/` 来自 `Pattern` 的 `\N{}` 分支 → `CharacterName` → 归一化数据。

### 6.3 漏覆盖

| 前缀 | 漏覆盖类 | 性质 |
|---|---|---|
| `sun/nio/cs/` | `UTF_8$Decoder` | `StreamDecoder` 仍按 provides 取手写（以 Charset 重载直连解码），删除该手写后复测 |
| `jdk/internal/math/` | `MathUtils` | `DoubleToDecimal` / `FloatToDecimal` 仍按 provides 取手写，删除后复测 |
| `sun/security/action/` | `GetBooleanAction` | 同上（provides 取手写） |
| `jdk/internal/access/` | `JavaLangInvokeAccess` | JVM 的 MethodHandle 引导加载；rava 以运行模型替换，待核对后入白名单 |
| `sun/invoke/util/` | `Wrapper` / `ValueConversions` 等 7 个 | 同上（indy / MethodHandle 引导） |
| `jdk/internal/util/` | `ReferencedKeyMap` / `ReferencedKeySet` 等 5 个 | `MethodType` 驻留表（同上） |
| `jdk/internal/org/objectweb/asm/` | 20 个 | `InvokerBytecodeGenerator` 运行期字节码生成（运行模型替换，VM 契约第 3 类） |
| `jdk/internal/misc/` | `MainMethodFinder` / `PreviewFeatures` | JVM 启动器找 main（rava 启动模型替换） |

「provides 取手写」一类是实测工具的口径所致（放行后同名手写仍优先），删除候选落地后按字节码执行即应消失。

### 6.4 判定（§4.3）

| 结论 | 前缀 | 依据 |
|---|---|---|
| 放行 | `jdk/internal/access/`、`jdk/internal/math/`、`sun/security/action/`、`jdk/internal/module/`、`jdk/internal/perf/`、`sun/invoke/util/`、`sun/security/util/`、`sun/text/` | 增量 ≤ 预算；穿透目标在同批（`sun/invoke/util` → `jdk/internal/math`）或为 VM 契约接口 |
| 放行（附条件） | `sun/nio/cs/` | +27 ≤ 36；穿透 `jdk/internal/access`（同批放行）、`jdk/internal/misc/ScopedMemoryAccess` / `jdk/internal/foreign/MemorySessionImpl`（VM 契约）；`StreamDecoder` 的 Charset 直连解码需用户判定是否属运行模型替换 |
| 部分放行 | `jdk/internal/misc/` | 负增量；VM 契约类逐类列清单：`Unsafe`、`VM`、`CDS`、`ScopedMemoryAccess`、`PreviewFeatures`（放行后触达的 26 个 native 全部落在这 5 类），其余按字节码 |
| 部分放行 | `jdk/internal/vm/` | +31 > 12；`Continuation` / `ContinuationSupport` 承载 native（虚拟线程，VM 契约），其余类待 G1–G3 修正后重测 |
| 待重测（分析器先修） | `sun/reflect/generics/`、`java/security/`、`jdk/internal/loader/`、`jdk/internal/util/`、`jdk/internal/ref/`、`sun/util/`（含 `locale`、`locale/provider`、`logging`）、`jdk/internal/icu/`、`jdk/internal/org/objectweb/asm/` | 超预算，主因 G1–G3；按 §八「精确分析的问题先修分析器」处理，不据此保留截断 |
| 无数据 | §6.1 末段 13 个前缀 | 验收集未触达，全量 e2e 语料重测 |

`jdk/internal/org/objectweb/asm/` 属运行期字节码生成（VM 契约第 3 类），终态由 `InvokerBytecodeGenerator` 的承载点截断，
无需放行；此处只记录数据。

### 6.5 后续

1. 分析器：G1 静态转发按调用点克隆（形参流入分派接收者的静态方法）、G2 汇合点上下文（`append(Object)` /
   `valueOf` / `equals` 按调用点）、G3 `[facts]` 增加 native 返回数组元素类型（`Class.getEnclosingMethod0` 等）；
   修正后对「待重测」行复测。
2. 「无数据」行在全量 e2e 语料上重测（`rava closure --release` 批量）。
3. 放行行的删除候选经用户逐项确认后，按 §五 第 3 步逐包实施并复测漏覆盖。

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

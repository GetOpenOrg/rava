# C1d 边界收窄：手写只留 VM 契约层

> 关联：`docs/plans/2026-09-29-rust-closure-analyzer.md`（§6.1 分工原则、C1c 精确分析）、`CLAUDE.md` 原则 1、[`docs/reference/handwritten-boundary.md`](../reference/handwritten-boundary.md)（手写边界规范，准入与审计的权威定义）、
> `runtime/java_runtime/closure.toml`、`docs/reports/2026-09-14-impl-strategy.md`（截断的原始规模数据）。
> 状态（2026-10-01）：🔄 第 1 步完成（`--release` 实测，§六）；删除方式按用户决定改为**一次性删到终态再统一验证**（不再逐包，§6.8 的包序仅作参考）：c1d-p6 已并入 `c1d-final`，全部非 VM 契约过渡手写删除与 `[boundary]` 前缀取消已提交（1e623cec）；`c1d-final` 已并入 `c1d-prec`，删除后暴露缺口的修复进行中（Digester E0433、枚举反射 values、System.in 已修；UnsafeConstants / 直接内存、反射 signature 待修），另一会话的 VM 注入状态修复（dc9fd946 / d8a0b082：UnsafeConstants、VM.directMemory、System.in、jca 别名、ScopedMemoryAccess 等）已经 b1ac5b7c 并入主干，由 `c1d-prec` 合并时按手写边界规范取舍；内容感知精度实测可靠收益为 0、不实施（19cf2b5b），编译成本改由 rustc 拆 crate 方案解决（`2026-10-01-rustc-memory-and-crate-split.md`）；精度线已收尾（§6.9：G2′ ✅、系统属性折叠 ✅、类镜像静态字段 ✅、按名取类 ✅），精度二期已合入（a6b4c6d5：方法引用装箱适配、record ObjectMethods、按名方法查找，§6.10）；精度三期（`closure-prec3`：VarHandle 可达性收窄、G4–G6、ServiceLoader、SystemJavaLangAccess、数组汇聚、选择子克隆、类初始化事实）9 项完成，TestCharsetForName 回归已修（98bc2e68 / 17fc2e7e，§6.11），✅ 已合入 d8212bee。优化方向总纲见 [`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md)（用户决策：凡能提升精度的优化都要做）。

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

VM 契约层的定义与准入类别见 [手写边界规范](../reference/handwritten-boundary.md) §一–§二（原则：方法的语义以它自己的
字节码为准；只有字节码无法表达时才手写，性能替换不算）。本阶段处理的是规范 §三 的**过渡类**（规模策略截断），
终态计数 0。

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
另需「忽略前缀内 provides」模式，模拟手写删除后的闭包：删除候选落地前以此实测真实增量（如 `sun/nio/cs` 的
`StandardCharsets` 查找链），§六 中依赖 provides 的结论以该模式复测为准。

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
- `whole_class` 删除；其中 `InvokerBytecodeGenerator` 的生成入口已按方法登记为运行模型替换（规范类 2），其余成员按字节码翻译。
- 每个非 native 手写方法登记类别（运行模型 / VM 行为 / 策略截断），raw-audit 按类计数（规范 §六）。
- 内部边界类的 struct 改由字节码生成，VM 注入的隐藏字段由清单声明、生成器追加（规范 §四）。
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
   手写边界规范 §七 过渡表收敛为终态（CLAUDE.md 原则 1 已于 2026-09-30 改写为按方法划分的准入原则）。

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
| 放行（附条件） | `sun/nio/cs/` | +27 ≤ 36；穿透 `jdk/internal/access`（同批放行）、`jdk/internal/misc/ScopedMemoryAccess` / `jdk/internal/foreign/MemorySessionImpl`（VM 契约）；`StreamDecoder` / `StreamEncoder` 属过渡类（规范 §三），删除前以「忽略 provides」模式复测 `StandardCharsets` 查找链增量 |
| 部分放行 | `jdk/internal/misc/` | 负增量；VM 契约类逐类列清单：`Unsafe`、`VM`、`CDS`、`ScopedMemoryAccess`、`PreviewFeatures`（放行后触达的 26 个 native 全部落在这 5 类），其余按字节码 |
| 部分放行 | `jdk/internal/vm/` | +31 > 12；`Continuation` / `ContinuationSupport` 承载 native（虚拟线程，VM 契约），其余类待 G1–G3 修正后重测 |
| 待重测（分析器先修） | `sun/reflect/generics/`、`java/security/`、`jdk/internal/loader/`、`jdk/internal/util/`、`jdk/internal/ref/`、`sun/util/`（含 `locale`、`locale/provider`、`logging`）、`jdk/internal/icu/`、`jdk/internal/org/objectweb/asm/` | 超预算，主因 G1–G3；按 §八「精确分析的问题先修分析器」处理，不据此保留截断 |
| 无数据 | §6.1 末段 13 个前缀 | 验收集未触达，全量 e2e 语料重测 |

`jdk/internal/org/objectweb/asm/` 属运行期字节码生成（VM 契约第 3 类），终态由 `InvokerBytecodeGenerator` 的承载点截断，
无需放行；此处只记录数据。

### 6.5 后续

1. 分析器：G1 静态转发按调用点克隆（形参流入分派接收者的静态方法）、G2 汇合点上下文（`append(Object)` /
   `valueOf` / `equals` 按调用点）、G3 `[facts]` 增加 native 返回数组元素类型（`Class.getEnclosingMethod0` 等）；
   修正后对「待重测」行复测。✅ G3 / G1 已实施，G2 实测否决，复测与逐包建议见 §6.7。
2. 「无数据」行在全量 e2e 语料上重测（`rava closure --release` 批量）。
3. ✅ `--release-bytecode`（放行且忽略前缀内同名手写）已实现，复测结果见 §6.6。
4. 放行行的删除候选经用户逐项确认后，按 §五 第 3 步逐包实施并复测漏覆盖。`StreamDecoder` / `StreamEncoder`
   属过渡类（规范 §三），终态删除，随 `sun/nio/cs` 放行一起走。

### 6.6 删除手写后复测（`--release-bytecode`）

`rava closure --release-bytecode <前缀>`：放行前缀，且前缀内手写不再按精确名提供（`provides`）——模拟删除候选落地后的闭包。
native 仍取手写。11 个前缀 × 8 例（2026-09-30）。「JVM 加载」= 新增类中出现在该例 `-Xlog:class+load` 全量记录里的比例
（含启动期加载），衡量增量中有多少是 JVM 实际执行到的工作。

| 前缀 | `--release` Δ类 | `--release-bytecode` Δ类 | 逐例 Δ类 | JVM 加载 | 漏覆盖 | 成因 |
|---|---:|---:|---|---|---|---|
| `sun/nio/cs/` | +27 | +460 | 14/14/14/361/13/15/14/15 | 小例 12–16/22；FileIOTest 152/364 | 1 → 0 | 见下 ① |
| `jdk/internal/access/` | +0 | +141 | 26/26/26/23/26/14/0/0 | 20–22/26 | 1 → 1 | 见下 ② |
| `jdk/internal/misc/` | −128 | +121 | 18/18/18/18/17/17/11/4 | 10–11/18 | 2 → 2 | 见下 ③ |
| `sun/security/util/` | +3 | +86 | 0/…/0/86 | 11/86 | 0 | 见下 ④ |
| `sun/invoke/util/` | +6 | +25 | 0/…/4/11/10 | 1–2/例 | 7 → 7 | `Wrapper` / `VerifyAccess` 按字节码的真实工作；漏覆盖为 MethodHandle 引导（运行模型替换） |
| `jdk/internal/perf/` | +2 | +6 | 0/…/0/6 | 0/6 | 0 | 小 |
| `jdk/internal/math/` | +0 | +2 | 0/0/2/0/0/0/3/1 | 2/6 | 1 → 0 | `MathUtils` 漏覆盖消失（`DoubleToDecimal` 按字节码） |
| `jdk/internal/module/` | +1 | +1 | 同 `--release` | — | 0 | 无手写 provides |
| `sun/text/` | +1 | +1 | 同 `--release` | — | 0 | 无手写 provides |
| `sun/security/action/` | +0 | −39 | −10/−10/−10/−9/0/0/0/0 | — | 1 → 1 | 手写不透明返回导致的 `Character*` 过近似消失；`GetBooleanAction` 仍漏，待查 |
| `sun/util/` | +37 | −213 | 0/0/0/0/13/0/0/−226 | — | 0 | 见下 ⑤ |

成因：

1. **`sun/nio/cs`**：小例 +14 是 `StreamEncoder` / `UTF_8` 按字节码走的 `ByteBuffer` / `CharBuffer` / `CharsetEncoder` 等，
   大部分 JVM 实际加载——此前由手写（Charset 直连编解码）承担，被隐藏的真实工作。FileIOTest +361 来自
   `Charset.defaultCharset` → `StandardCharsets.lookup` → `Class.newInstance`（按类名表反射实例化非内建字符集）
   → `ReflectionFactory.newConstructorAccessor` → `MethodHandleAccessorFactory` → MethodHandle 体系；该路径在 rava 中按字节码执行
   （`jdk/internal/reflect/` 已放行，同 `Field.get` / `Method.invoke` 管线），静态上不可排除（字符集名来自属性）。CollectorsDemo
   基线已含这套管线，故只 +15——属一次性基础设施成本。另有 G2（`StringBuilder.append(Object)` → `ProtectionDomain.toString`）的少量噪声。
   手写版本多出的 8 个类（`UTF_16*` / `UTF_32*` 等，JVM 未加载）在按字节码时消失。
2. **`jdk/internal/access`**：`SharedSecrets.getJavaXxxAccess` 的字节码先 `ensureClassInitialized(目标类)` 再读静态字段，
   把目标类的 `<clinit>` 拉入闭包；新增类 85% 为 JVM 实际加载。真实工作。
3. **`jdk/internal/misc`**：`VM` / `InternalLock` / `Blocker` / `VirtualThreads` 按字节码；JVM 未加载的 7 个
   （`ForkJoinPool`、`Blocker$ForkJoinPools`、`JavaUtilConcurrentFJPAccess` 等）来自 `Blocker.begin` 中
   `currentCarrierThread() instanceof CarrierThread` 分支——分析器不知道平台线程永不是 `CarrierThread`。
   与 `--release` 的 −128 对比：−128 是「放行但 VM 契约类仍取手写」口径；`--release-bytecode` 把 `Unsafe` 等 VM 契约类的非 native
   方法也按字节码走，终态口径以 §6.4「部分放行」（5 个 VM 契约类的 native 手写）为准，二者之间的差距不影响判定。
4. **`sun/security/util`**：`Debug.getInstance` 按字节码后，`Debug.println` → `formatCaller` → `StackWalker` 进入闭包；到达路径是
   G1（`AccessibleObject.<clinit>` 的 `doPrivileged` 与 `ForkJoinPool` 工厂共用 `executePrivileged`）→ `AccessController.getContext`
   → `AccessControlContext.optimize` → `Debug.println`（运行时 `debug == null`，不执行）。`TzdbZoneRulesProvider` 等来自 G2
   （`println(Object)` 广播）。精度问题，G1/G2 修正后应基本消失。
5. **`sun/util`**：CollectorsDemo −226。手写的返回对分析不透明、只能取 open(返回类型)，经 G2 广播放大；按字节码后精度提高。
   移除的 256 类中 9 个是 JVM 加载的（`PublicMethods$Key`、`InfoFromMemberName`、`FinalReference`、`WeakHashMap$KeySet` 等），
   已查的均来自 lambda 引导的 `revealDirect`（运行模型替换）、finalizer（VM 驱动）或 G1 路径，其余待逐个核对后才能判定无漏。

结论：

- 删除手写的真实代价显著高于 `--release` 口径（`sun/nio/cs` +460 vs +27，`jdk/internal/access` +141 vs +0），
  但**超出部分大多是 JVM 实际执行的工作**（小例 60–85% 为 JVM 加载类）——此前被手写近似承担，不是分析器噪声。
  §4.3 的预算（文件数 × 3）是按「手写保留」设计的精度告警线，不适用于删除口径；删除按规范 §三「过渡类终态 0」执行，
  超出部分只追查精度问题（G1/G2 与 ③ 的线程类型判定），不据此保留手写。
- 精度问题新增一项：**G4** 平台线程上 `instanceof CarrierThread` / 虚拟线程分支（`Blocker`、`VirtualThreads`）——
  由清单事实声明「无虚拟线程载体」或按 `Thread.currentCarrierThread` 的返回事实收窄。
- `sun/security/action`、`sun/util` 删除手写使闭包**变小**，是「手写对分析不透明」的直接证据（规范 §六）。

### 6.7 G1–G3 修正后复测

口径：验收集 8 例（HelloWorld、TestSwitchString、PatternSwitchTest、FileIOTest、ChineseRemainderTheorem、TestStreamBasic、
TestRecordComponents、CollectorsDemo）× 9 配置（基线 + 下列 8 个前缀各自 `--release`），JDK 21。
三个版本：修正前（2026-09-29 C1d 基线）、G3、G3+G1（终版）。动态对照 = JVM `-Xlog:class+load` 在主类之后加载、
不在闭包内的类（隐藏类 / lambda 类除外），逐配置比较漏覆盖集合。

**实现**

- **G3**（2208ddda）：`vm_intrinsics.toml` 新增 `[facts.array_returns]`（`"<成员>" = { elements = [...] }`，成员须返回引用数组），
  分析器把该手写返回建模为一个数组分配点，元素 = 所列类型的 open；首条：`Class.getEnclosingMethod0` → `{Class, String}`。
- **G1**（69c2e14d）：**静态分派转发方法按调用点克隆，k = 1**。判定纯看字节码（`engine/forward.rs`）：静态方法的引用形参
  经 checkcast、经下游非虚调用的转发槽（递归）、或作为字符串拼接动态实参，流到虚 / 接口分派接收者。上下文无关的调用方
  调用它时按调用点克隆，调用方已在上下文中（容器对象 / 调用点）则继承——`doPrivileged → executePrivileged` 整条链随
  最外层调用点分开（实测各克隆的 `action` 形参均为单一类）。无类名、无清单项。
  实现要点：上下文须在**建方法节点前**判定——先建上下文无关本体再改道到克隆，本体成为无调用方的孤立节点，以 Top 形参被分析，
  常量折叠失效（`ZipFile$Source.<init>` 的 `toDelete` 分支 → `OperatingSystem`）且耗时 ×7。
- **G2 否决**：同一规则扩展到实例方法（`append(Object)` / `valueOf(Object)` / `Objects.equals` 等汇合点，含分派中心按调用点接边）
  后，CollectorsDemo 等 4 个配置的闭包类集与只做 G1 **逐一相同**（CollectorsDemo 1256 = 1256），上下文 22838 → 77231、
  耗时 5.0 s → 30.3 s（放行 `java/security/` 时 70 s）。原因：汇合点上的 open(Object) 在调用点处已经是 open——
  例 `LambdaForm$Name.exprString` 把 `Object[] arguments` 的元素传给 `append(Object)`，元素集本身是 open(Object)，
  调用点上下文分不开。收窄要落在 open(Object) 的**引入点**（`Object[]` 元素读、手写返回、`Permissions.getUnresolvedPermissions`、
  `MemberName.getMethodType` 等，`--flows @opens:java/lang/Object` 可列），记为 **G2′**。

**基线配置（不放行）**

| 用例 | 类数 修正前 → G3 → G1 | open(Object) 节点 | 方法上下文 | 耗时 ms | 动态漏覆盖 |
|---|---|---|---|---|---|
| HelloWorld | 251 → 251 → 251 | 44 → 44 → 42 | 854 → 854 → 878 | 134 → 165 → 181 | 76 → 76 → 76 |
| TestStreamBasic | 400 → 400 → 400 | 399 → 399 → 556 | 2369 → 2369 → 2479 | 226 → 242 → 286 | 92 → 92 → 92 |
| FileIOTest | 293 → 293 → 293 | 62 → 62 → 60 | 1027 → 1027 → 1057 | 169 → 165 → 164 | 80 → 80 → 80 |
| CollectorsDemo | 1261 → 1261 → 1256 | 9606 → 9600 → 11140 | 18081 → 18081 → 22838 | 3982 → 4030 → 4964 | 25 → 25 → 25 |

open(Object) 节点按方法克隆计数：G3 使 `getEnclosingMethod0` 的引入点消失（−3~−6）；G1 克隆把同一 open 值复制到各调用点副本，
节点数随上下文增加而增加，不代表更宽。G1 删去的类：`ReduceOps$5` / `CountingSink*`（`MethodHandleImpl.loop` 的
`Object...` 形参跨调用点汇合）、`MethodHandleImpl$CasesHolder` / `$LoopClauses`、`VarHandles`（TestRecordComponents）。

**`--release` 增量（相对同版本基线配置；修正前 → G3 → G1）**

| 前缀 | HelloWorld | TestStreamBasic | FileIOTest | CollectorsDemo |
|---|---|---|---|---|
| `sun/reflect/generics/` | +1036 → +1036 → +1031 | +939 → +939 → +937 | +995 → +995 → +990 | +48 → +48 → +48 |
| `java/security/` | 0 → 0 → 0 | −5 → −5 → −5 | 0 → 0 → 0 | +227 → +227 → +227 |
| `jdk/internal/loader/` | 0 → 0 → 0 | 0 → 0 → 0 | 0 → 0 → 0 | +238 → +238 → +238 |
| `jdk/internal/util/` | +4 → +4 → +4 | +3 → +3 → +3 | +3 → +3 → +3 | +59 → +59 → +59 |
| `jdk/internal/ref/` | 0 → 0 → 0 | 0 → 0 → 0 | +39 → +39 → +39 | +9 → +9 → +9 |
| `sun/util/` | 0 → 0 → 0 | 0 → 0 → 0 | 0 → 0 → 0 | +21 → +21 → +21 |
| `jdk/internal/icu/` | 0 → 0 → 0 | 0 → 0 → 0 | 0 → 0 → 0 | +55 → +55 → +55 |
| `jdk/internal/org/objectweb/asm/` | 0 → 0 → 0 | 0 → 0 → 0 | 0 → 0 → 0 | +17 → +17 → +17 |

**健全性**：8 例 × 9 配置共 72 个闭包，G1 相对 G3 类集只减不增；漏覆盖合计 4791 → 4793，新增 2 个均为 PatternSwitchTest
放行 `sun/reflect/generics/` 时的 `MethodHandleImpl$CasesHolder` / `$LoopClauses`——二者在该用例其余 8 个配置（含基线）
修正前后都是漏报（`typeSwitch` 引导的 MethodHandle 组合子，§6.3 「indy / MethodHandle 引导」类，运行模型替换），
G3 版本在 generics 配置下只是经 `MethodHandleImpl.createFunction` 的 `ldc` 顺带覆盖。其余 70 个配置漏覆盖集合逐一相同。
`cargo test -p closure` 通过（含 `[facts.array_returns]` 解析 / 拒绝两例）。

**结论：增量主因不是 G1–G3**。G1–G3 修正后 8 个前缀的增量基本不变；§6.2 按 `--why` 首条路径归因 G1/G2 不成立——
`--why` 按方法本体合并上下文，去掉一条路径后同一批类经其他真实路径仍可达。逐包追到的主路径（G1 版本）：

| 前缀 | 增量中 JVM 实际加载 | 主路径 | 性质 | 建议 |
|---|---|---|---|---|
| `sun/reflect/generics/` | HelloWorld 193 / 1031；CollectorsDemo 1 / 48 | `ConcurrentHashMap.comparableClassFor → Class.getGenericInterfaces → Reifier → ParameterizedTypeImpl.validateConstructorArguments → String.format`（异常消息） | 包自身成本 +48（39 类属本包）；小例的 ~1000 是首次触达 `Formatter` 子系统（→ `Pattern` → 大小写 / 断词 / locale → 流水线 / MethodHandle），CollectorsDemo 经 `Collectors.duplicateKeyException` 已含 | **放行**；`Formatter` 冷路径另立 G5（异常构造分支的冷路径成本），不据此保留截断 |
| `java/security/` | 18 / 227 | `String.format → Formatter → Calendar → JapaneseImperialCalendar → CalendarSystem.forName → Class.newInstance → MethodHandle → LambdaForm.toString → append(Object)`（open 元素）`→ ProtectionDomain.toString → Policy → PolicyFile → URL / InetAddress / SocketPermission` | 精度（G2′：`Object[]` 元素 open） | G2′ 修正后复测再放行；终态放行 |
| `jdk/internal/loader/` | 37 / 238 | `Pattern \N{}` → `CharacterName` 读 `uniName.dat` → `Class.getResourceAsStream → BuiltinClassLoader.findResource → URLClassPath → JarFile / ZipFile` | 真实路径（类路径资源分支静态不可排除） | **放行** |
| `jdk/internal/util/` | CollectorsDemo 5 / 59（小例 +3~+4） | `BindCaller.makeInjectedInvoker → Lookup$ClassDefiner.defineClass → ClassFileDumper.dumpClass → doPrivileged → Files` | 穿过运行期类定义点（规范准入②），该点承载后截断 | **放行** |
| `jdk/internal/ref/` | FileIOTest 30 / 39；CollectorsDemo 8 / 9 | 文件流的 `Cleaner` 注册路径 | 真实路径 | **放行** |
| `sun/util/` | 1 / 21 | `Currency$1` 错误路径取日志器 →（手写 `LazyLoggers.getLazyLogger` 返回 open(`System$Logger`)）→ 各实现类 | 手写不透明（§6.6 按字节码后 −226） | **放行**（连同 `jdk/internal/logger` 手写按字节码） |
| `jdk/internal/icu/` | 0 / 55 | `Pattern.<init>` 的 `CANON_EQ` 分支 `→ Pattern.normalize → Normalizer → NormalizerBase` | 精度（G6：`CANON_EQ` 标志经字段 / 汇合形参（`Pattern(p, 0)`）传递，分支不折叠） | **放行**；G6 修正后增量应归零 |
| `jdk/internal/org/objectweb/asm/` | 7 / 17 | `InvokerBytecodeGenerator` 运行期字节码生成 | 运行模型替换（规范准入②） | 不放行，由承载点截断（同 §6.4） |

新增精度项：**G2′** open(Object) 引入点收窄（`Object[]` 元素、手写返回）；**G5** 异常构造分支触达 `Formatter` 子系统的冷路径成本；
**G6** 经字段传递的构造期标志常量。

### 6.8 逐包删除手写的顺序（2026-09-30 确认）

用户确认「直接删除这些不要的手写方法」。逐包独立提交，每包：删除手写 → `[boundary]` 去前缀 → 抽查
（`master_passed_jdk21.txt` 中触达该包的用例）通过数不降、动态对照漏覆盖不增。顺序按依赖与风险由低到高：

1. `jdk/internal/math` ✅ 705d54f5（抽查 10 例 4 过；6 例失败经闭包对照与本包无关：3+1 例为手写 LocaleProviderAdapter 返回流未建模的既有精度缺口，已转闭包精度子任务；2 例为并发编译资源争抢）
2. `sun/security/action` ✅ 2322925c（删 GetPropertyAction / GetBooleanAction 9 个过渡方法；e2e 抽查待跑）
3. `jdk/internal/module` ✅ 5a72714c（放行；`ServicesCatalog` 3 个过渡手写暂留：依赖手写 JLA 不可见 + `jdk/internal/loader` 截断 + `ServicesCatalog.create` 未入链致 map 折叠为 null；e2e 抽查待跑）
4. `jdk/internal/perf` ✅ 29447fa0（删 PerfCounter 14 个过渡方法；新增 `perf_impl.rs` 仅含 2 个 `ACC_NATIVE`；e2e 抽查待跑）
5. `sun/security/util` ✅ d6101c07（删 SecurityConstants / CryptoAlgorithmConstraints；`Debug` 4 个过渡手写暂留，待分析器折叠未设置的系统属性为 null 后删除）
   - 包 2–5 抽查（c1d25 / c1d25b / c1d25c）暴露的回归均为分析器看不见手写层所致，已在分析器侧修正：System.props 折叠为 null（5f40dcb0）、`MethodType.toMethodDescriptorString` 误为存根（8078f2b1）、DeepCopy / ListFields / RecordsSerializationTest 经 checkContext 拉入 `DomainName$Rules`（02bf00ff + c731a473）；另修手写 NetworkInterface 在 macOS 上的 `ifa_ifu` 编译错误（8c898456）；修正后的复测进行中
6. `sun/invoke/util` 🔄 a6c836e6（分支 `c1d-p6`：删 Wrapper / BytecodeDescriptor / VerifyAccess 过渡手写 576 行；抽查通过后合入）
7. `jdk/internal/access`
8. `sun/nio/cs`
9. `jdk/internal/misc` 中非 VM 契约部分（`Unsafe` / `VM` / `Signal` 等 VM 契约类保留）

精度项 G4–G6 与删除并行推进（G2′ 已完成，见 §6.9），不作为删除的前置条件（精度只影响闭包大小，不影响正确性）。

### 6.9 精度线进展（`closure-precision`，2026-09-30）

| 提交 | 内容 | 实测 |
|---|---|---|
| 5f40dcb0 / c501c499 / bfae1dee | 系统属性表折叠（`[facts.system_properties]`）、手写 static setter 识别、属性表作返回值不判逃逸 | 默认为 null 的属性读点折叠，其守卫分支成为死代码（`jdk.security.defaultKeySize` 等） |
| 02bf00ff / c731a473 | **G2′ ✅**：字段枚举须与句柄写入口（按调用边）同时可达才放开字段 | `checkContext` 等随 SecurityManager 常量折叠 |
| 8078f2b1 | 手写扫描：turbofish 转换、let-else / if let / match 绑定、宏实参调用点 | — |
| b7f76452 | folds v2：逐条目 `dead_catches` | Python 与 Rust input 两侧消费 |
| b643a597 | 按名查找的名字来自形参时取各调用点字符串常量；`MemberName` 按名构造登记为方法查找 | DMH / Invokers 具名函数进入反射分派 |
| 0333ede2 | 类镜像作静态字段基址：`Unsafe` 按「镜像 + 偏移」读写接到该类静态引用字段；镜像所指类未知时按名开放静态字段 | RecordsSerializationTest：`SpeciesData.transformHelper` 入链，null_recv 691 → 503，missing 0 |
| f1d00887 | **按名取类**：`Class.forName` 的名字为「常量前缀 + 常量表取值」拼接时解析成具体类（清单 `class_lookups` / `instantiators` / `constant_tables`、`[facts.string_concat]`），经实例化再 checkcast 到 T 时只留 T 的子类型 | `--release-bytecode sun/nio/cs/`：FileIOTest 661 → 478 类、反射缺口 1 → 0；HelloWorld 缺口 0 |

**`MethodHandleAccessorFactory` 实况**：`StandardCharsets.lookup → Class.newInstance → ReflectionFactory.newConstructorAccessor`
按字节码进入该类，但只有 `<clinit>` / `newConstructorAccessor` / `useNativeAccessor` 三个方法；`useNativeAccessor`
折叠为 true，MethodHandle 生成分支（`newConstructorAccessor` dead_pcs [12, 84]）为死代码，实际只走
`DirectConstructorHandleAccessor$NativeAccessor`。这是 `Class.newInstance` 字节码的如实结果，不据此手写。
同一路径上 `getConstructor0` 的错误消息分支（`methodToString → Arrays.stream`）带入约 60 个 stream 类，属 G5 同类冷路径。

**剩余缺口**：

- `TestCharsetForName` / `TestStreamEncoderCharsets` 的 `getDeclaredConstructors0 <- open(Class)` 来自
  `Charset$ExtendedProviderHolder → ServiceLoader → LazyClassPathLookupIterator.nextProviderClass` 的 `forName`：名字读自
  `META-INF/services`，非常量，按设计记为缺口；终态由「服务目录」事实（模块描述符 `provides` + 类路径服务文件，
  分析期可枚举）给出候选类，不做模糊扩展。
- `SystemJavaLangAccess`：`JavaLangAccess.decodeASCII` / `encodeASCII` / `inflateBytesToChars` 选择失败（FileIOTest
  unresolved 3），`SharedSecrets` 注入的实现对象（`System$2`）未建模到接口调用点。
- **G6**（未实施，设计）：`Pattern.flags0` 被构造器形参与 `addFlag` 的常量位或写入，`has(CANON_EQ)` 读字段后与实参
  按位与。终态：整型字段「可能置位掩码」域——字段掩码 = 全部写入值掩码之并（常量取其值；`x | C` 取 `x ∪ C`；
  `x & C` 取 `x ∩ C`；形参取各调用点实参掩码之并；其余为全 1）；`(field & C) != 0` 在掩码与 C 不交时折叠为 false，
  布尔返回的小方法（`has`）按调用点实参常量求值。验收：`jdk/internal/icu` 增量归零。

**下游须知**：

- **emitter 须消费 `null_recv`**：接收者值集为空的调用点目前仍生成对存根的调用，运行时若走到即 panic；终态
  按 `null_recv` 生成 NullPointerException 抛出（与 JVM 在该点的行为一致），不再引用存根。
- **运行时须加载 `[facts.system_properties]`**：分析器按清单初值折叠属性读点，运行时初始属性表必须来自同一张表，
  否则折叠不可靠（死分支在运行时实际可达）。
- **`Debug` 过渡手写可删**：`java.security.debug` 等属性默认 null，折叠后 `Debug.getInstance` 结果为 null，
  各 `Debug.println` 调用点只以 null_recv 形式出现（TestNetworkInterface 中 KnownOIDs / Provider 各点均如此），
  按字节码翻译不引入新类；前提是上面两条落地。
- **TestNetworkInterface `SHA-1 not available`**（非分析器问题）：`GetInstance` 手写边界按 `crate::jca::providers_for`
  精确匹配算法名，服务表只登记标准名（`SHA-1`），JDK 内部以别名 `SHA` 查询时返回空表。终态：provider 选择与 JDK
  `ProviderList` 同构——遍历已登记 provider 逐个调翻译字节码的 `Provider.getService`（别名由 serviceMap 解析），服务表只决定
  构造哪些 provider。`getInstance("SHA","SUN")` 同样失败，需运行期定位 `Sun.getService` 的别名查找。

### 6.10 精度二期（`closure-prec2`，2026-09-30，已合入 a6b4c6d5）

| 提交 | 内容 | 实测（类 / 方法） |
|---|---|---|
| dd2737ad | lambda 方法引用装箱适配（C 组） | TestMethodRef / TestMethodRefKinds / TestOptional / TestRecordHashCode 闭包正常 |
| c111e64f | record `ObjectMethods` 引导（K 组） | 同上 |
| b06fb0b6 / 66e176b2 | 按名方法查找（findStatic / findVirtual / resolveOrFail / MemberName.&lt;init&gt;）复用「常量前缀 + 常量表 / 枚举值」拼接解析，只保留目标类确实声明的方法；跟进单目标 String 返回辅助方法（≤ 2 层）、枚举 final String 字段候选值集；`[facts.string_concat]` 增 `StringBuilder.append(C/I/J/Z)` | TestMethodHandleCombinators 537/2741 → 540/2968（补 ValueConversions box/unbox 等）；HelloWorld 不变；Digester +9 / +1045（见下） |

- **Digester +1045 方法**：几乎全部来自 `VarForm.resolveMemberName` 按访问模式名在约 25 个 VarHandle* 类里查方法。按现有可达性结果健全，但上游 `Invokers.checkVarHandleGenericType` 经反射根可达，而 VarHandle 访问模式实际走手写运行时——列入精度三期第 1 项收窄。
- **EasterRelatedHolidays 诊断**：不是「静态字段缺默认 null」，是边界截断——`ZipUtils.<clinit>` 调 `SharedSecrets.getJavaNioAccess`（`jdk/` 边界手写），`ensureClassInitialized` 成为存根；且 `Unsafe.ensureClassInitialized` 手写为空操作，`CLASS_INIT_HOOKS` 只覆盖用户类与注解枚举。终态：C1d 取消截断后 SharedSecrets 按字节码翻译；分析器输出「初始化以参数传入的 Class」事实（精度三期第 8 项）；生成器按事实登记 JDK 类初始化钩子，`Unsafe.ensureClassInitialized` 接 `crate::ensure_class_initialized`（C3）。

### 6.11 精度三期（`closure-prec3`，2026-09-30 起）

清单与验收见 [`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md) §三.1。执行者按项把前后类 / 方法数与漏覆盖检查结果记录在本节。

基线（合入 c6ae80bb 后，类 / 方法 / 上下文，动态对照漏 / 多）：

| 测试 | 基线 | 常量实参求值后 |
|---|---|---|
| HelloWorld | 251 / 620 / 882，漏 0 | 251 / 620 / 882，漏 0 |
| Digester | 1440 / 9737 / 28634，漏 1 | **1297 / 7510 / 21078**，漏 1 |
| DeepCopy | 1649 / 10969 / 40942，漏 1 | 1649 / 10959 / 40294，漏 1 |
| FileIOTest | 293 / 795 / 1062，漏 0 | 293 / 795 / 1062，漏 0 |
| CollectorsDemo | 1166 / 8010 / 21319，漏 1 | **1034 / 5951 / 14375**，漏 1 |
| TestMethodHandleCombinators | 540 / 2968 / 5954，漏 0 | 540 / 2968 / 5954，漏 0 |

漏 1 均为 `LambdaMetafactory`（indy 引导，手写准入第 2 类，预先存在），漏集不变。

**项 1 VarHandle 可达性（常量实参求值）**。`--why` 链：`MessageDigest.getInstance → CryptoAlgorithmConstraints.<init> → ReferencePipeline.anyMatch
→ AbstractPipeline.evaluate`（未折 `isParallel()`，见下 (A)）`→ MatchTask → AtomicReference.<clinit> findVarHandle → VarHandles.makeFieldHandle@56
→ maybeAdapt → filterValue → MethodHandleImpl.getConstantHandle → … → Invokers` 按名暴露 NF 方法 → `checkVarHandleGenericType → VarForm.resolveMemberName`（≈1045 方法）。
`maybeAdapt` 的守卫 `MethodHandleStatics.VAR_HANDLE_IDENTITY_ADAPT = Boolean.parseBoolean(getProperty(k, "false"))`：属性读取已折为 `"false"`，
但 `parseBoolean` 的返回常量格在全部调用点上汇合为 Top，`<clinit>` 常量求值（辅助分析）又不查被调方法。
修法（分析层通用规则，无类名特判）：
- `engine/consteval.rs` 常量实参求值：唯一字节码目标、实参含常量（int / long / 字符串 / null）、返回常量格为 Top 或缺席（及辅助分析中）时，
  以常量实参绑定形参对被调方法做一次辅助分析，全部返回路径汇成同一常量即为该调用的结果。深度 ≤ 3、被调方法 ≤ 256 条指令、递归保护；
  按 (目标, 常量实参) 记忆；求值中读过的字段随结果登记给外层方法（字段转不折叠 / 系统属性转不稳定时记忆清空、外层失效）。
- `[facts.string_ops]`（清单）：`String.equalsIgnoreCase` / `length` / `isEmpty` 在常量实参上按 JDK 规范语义求值（非 ASCII 大小写比较不求值）。
  `parseBoolean("false") = "true".equalsIgnoreCase("false") = false` 由此经字节码求出。
结果：Digester 中 `filterValue` / `checkVarHandleGenericType` / `resolveMemberName` 全部移出闭包；CollectorsDemo 同链受益（−132 类 / −2059 方法）。
辅助分析次数 Digester 5966 → 38027，总耗时持平（6.4 s → 5.4–6.4 s）。
剩余同类缺口：(A) `AbstractPipeline.parallel` 由构造器从常量实参写入，`isParallel()` 未折（字段值域，并入项 4）；
(C) `Invokers.createFunction(byte)` 的 tableswitch 未按常量实参剪枝（形参常量格合流为 Top；需按常量实参克隆上下文，见后续）。

**健全性 S1：反射虚调用漏覆写（`DirectMethodHandle$Interface.checkReceiver` 存根命中）**。
`--why`：`DirectMethodHandle.createFunction` 以 `new MemberName(DirectMethodHandle.class, "checkReceiver", OBJ_OBJ_TYPE, REF_invokeVirtual)`
取 NF，按名暴露只补入了声明类上的 `DirectMethodHandle.checkReceiver`；LambdaForm 解释执行时该 NF 以 DMH 实例为接收者**虚调用**，
`Interface` / `Special` 的覆写不在闭包（`Interface` 类本身经 `DirectMethodHandle.make@156` 已实例化入闭包）。同一缺口也吃掉了用户侧
`findVirtual(Shape, "area")` 的 `Sq.area` / `Circle.area`（`Method.invoke` 同理）。
修法：`reflect.rs::expose` 对可被覆写的实例方法（非 static / private / final、类非 final）经新的 VM 枢纽 `HubSet::Vm(open(声明类))`
派发——G 中每个接收者选中的实现入链、形参 open（与反射成员同口径），G 增长时增量展开（`hub.rs::vm_dispatch`）。
代价：TestMethodHandleDirect 552 / 3003 → 1149 / 7829，TestMethodHandleCombinators 540 / 2968 → 1140 / 7808——`--why` 均为
`Special.checkReceiver@41` 的异常消息 `String.format`（→ Formatter / Locale / regex），属 G5 冷路径（项 3 处理），不是本修复的误差。
其余 5 例不变。

**动态对照为何没发现 S1**：`dyn_compare` 以**类加载**为粒度——只有程序期加载了闭包外的类才检查栈。`Interface` 已在闭包内（被实例化），
执行其漏掉的方法 `checkReceiver` 没有触发任何新类加载（`refc.isInstance` 成功路径不加载类），于是没有事件可供归因；
LambdaForm 解释路径的隐藏帧被剔除不是主因（若 `checkReceiver` 内发生加载，其帧不在闭包仍会报漏）。
结论：类粒度对照对「已在闭包内的类上漏掉的方法」结构性失明。补救方向（记为项 9 候选）：load_trace agent 增加方法粒度轨迹
（JVMTI `MethodEntry` 或采样），对照闭包 `methods` 的翻译域漏方法。

**健全性 S2：Digester / CollectorsDemo / DeepCopy 漏 1 = `LambdaMetafactory`**。首次加载栈顶即 `Security.<clinit>@9`
（invokedynamic，引导 `LambdaMetafactory.metafactory`）：JVM 链接 indy 调用点时解析引导方法句柄而加载其所属类，全栈已建模 → 报为类引用边漏覆盖。
终态语义论证：清单 `[indy]` 列出的引导（lambda / 字符串拼接 / record 方法 / native）属手写准入第 ② 类「运行模型替换」，原生程序不执行
引导方法，`LambdaMetafactory` 不属翻译程序——在分析器补 Ref 级类边会让生成器为一个永不执行的类出类型，是错误的闭包语义。
修法：分析器导出事实、对照按事实归因（无类名特判）：
- closure.json 新增 `indy_models: [{site: "方法@偏移", bootstrap, kind}]`（`lambda.rs::indy` 记录被运行模型替换的调用点）；
- `dyn_compare` 新分类 `indy-model`：帧停在这些调用点、其上方全是模型外帧（JVM 链接期 / 引导执行）时的加载归此类；模型再次进入的
  已建模方法（拼接 `toString`、lambda 实现）从该帧起照常归因，不被掩盖（单测 `test_indy_model`）。
结果：Digester / TestStreamBasic / CollectorsDemo 漏 0（原先经 `MethodHandleNatives.linkCallSite` 归 `vm-upcall` 的链接期加载一并改归 `indy-model`）。

**项 3 G5：异常构造 / 错误消息冷路径**。测量工具（已提交）：`cold.rs::doomed` 按指令给出「冷」标记——从该指令出发的每条正常路径都在本方法内
以 `athrow` 结束、且覆盖它的处理器同为冷（最小不动点，循环不冷，单测 2 例）；诊断选项 `rava closure --cold-cut` 丢弃冷指令上的事件，
得到「只经热路径可达」的闭包（**不健全，只用于测量**）。实测（类 / 方法，正常 → 截冷；「冷独占类」= 两者之差）：

| 测试 | 正常 | 截冷 | 冷独占类 | 冷独占主要包 |
|---|---|---|---|---|
| HelloWorld | 251 / 620 | 236 / 508 | 15 | `java/lang` 14 |
| Digester | 1297 / 7510 | 1245 / 7328 | 52 | `java/util` 15、`java/lang` 8、`java/io` 5 |
| DeepCopy | 1649 / 10961 | 1599 / 10729 | 50 | 同上 |
| FileIOTest | 293 / 795 | 286 / 748 | 7 | — |
| CollectorsDemo | 1034 / 5951 | **349 / 1070** | **685** | `java/util` 113、`java/util/stream` 65、`java/util/regex` 63、`java/lang/invoke` 56、`java/time/temporal` 28 |
| TestMethodHandleCombinators | 1140 / 7808 | **531 / 2887** | **609** | `java/util` 101、`java/util/regex` 63、`java/util/stream` 61 |
| TestMethodHandleDirect | 1149 / 7829 | 539 / 2901 | 610 | 同上 |
| TestStreamBasic | 400 / 1449 | 394 / 1427 | 6 | — |

`--why`（冷独占的两大例）：CollectorsDemo 为 `Collectors.toMap → uniqKeysMapAccumulator` 的 lambda`@44`（`v != null` 时）`→ duplicateKeyException@22
→ String.format → new Formatter → Formatter.<clinit>@7 Pattern.compile(<常量正则>)`；MH 两例为 `DirectMethodHandle$Interface.checkReceiver@41` /
`$Special.checkReceiver@41`（`!refc.isInstance(recv)` 时）`→ String.format`。`Special` 只因 `DirectMethodHandle.make` 的 `refKind` 在
`unreflect`（`MemberName.getReferenceKind()` 取自 flags）路径上为 Top 才入闭包，但 `Interface.checkReceiver`（运行期真实可达，S1）有同一条
`String.format` 冷路径，因此给 `refKind` 加整数值集合域剪掉 `Special` 对类集无收益，不做。

健全的分析层收窄不存在：抛出条件取决于运行期数据（重复键、接收者类型），异常消息是可观察语义（`getMessage`、未捕获异常打印），
冷路径不能剪。冷独占规模的终态分两部分解决，都不降低健全性：
1. **精度（分析层，需求并入项 8 类初始化事实）**：冷独占的大头不是 `format` 本身，而是**常量输入上的库初始化**——`Formatter.<clinit>`
   以常量正则调 `Pattern.compile`（`java/util/regex` 63 类 + 字符属性表），以及调用点的常量格式串（`"Duplicate key %s (attempted merging values %s and %s)"`
   只含 `%s`，`java/time/temporal` 等日期 / 数值转换分支不可达）。终态做**构建期类初始化**：对输入全为常量、无外部副作用的 `<clinit>`
   在分析期解释执行得到堆快照，快照里的对象图（Pattern 节点实例）成为闭包事实，正则编译器代码不再入链；格式串按常量实参求值 `Formatter.parse`
   （项 1 的常量实参求值扩展到对象返回）。前者需要带堆的字节码解释器，超出现有辅助分析（只求标量常量），单独立项。
2. **成本（生成层分层，C3）**：冷独占方法仍须翻译（健全），但可按低成本形态生成。分析器导出 `cold_methods`（闭包内、只经冷指令可达的方法），
   生成器把它们放入冷层：不做泛型 / 上下文特化、`opt-level = "s"`、JDK 冷层按内容哈希跨测试复用编译产物。
   导出口径：单次不动点内给方法节点加「热 / 冷」两级可达标签——热方法中非冷指令上的事件传热，其余只传冷；类型流不截断（闭包不变），
   冷标签只是分层依据，误标只影响成本、不影响正确性。

**给 C3 的接口（G5）**：`closure.json` 新增 `cold_methods: ["类.方法:描述符", …]`（上条口径；未实现，按上表 CollectorsDemo 约 4900 方法、MH 约 4900 方法），
生成器按之分层；`build_time_init`（构建期初始化快照）与项 8 的类初始化事实同一通道设计。

**项 2 G4：平台线程上的 `instanceof CarrierThread` / 虚拟线程分支**。`--why`（HelloWorld）：`ConcurrentHashMap.initTable@23 → Thread.yield`
里 `currentThread() instanceof VirtualThread` 未折，`VirtualThread.tryYield → yieldContinuation → Continuation`；`LockSupport.unpark@12`
同型分支带入 `VirtualThreads`；`Blocker.begin` 的 `currentCarrierThread() instanceof CarrierThread`。`currentThread()` 是 native，值为 open(Thread)，
值层无法判定。修法（通用规则，无类名）：**instanceof 目标类型（非数组）在 G 中无已实例化子类型时恒为 false**——与 catch 类型存活判定同一口径
（Oracle `catch_live` 更名 `type_live`，`Analysis.pending_catch` 更名 `pending_types`，可达 instanceof 的死目标类型一并登记，类型进入 G 时方法重分析）；
只对 `V::Ref` 值折叠（字符串 / 类字面量不折）。依据：G 已是虚分派的实例化事实（VM 产生的 `Thread` / `String` / `Class` / VM 抛出异常均由清单种入），
虚拟线程与载体线程只能由字节码 `new` 产生。单测 `absint/tests.rs` 2 例。结果（类 / 方法 / 上下文，动态对照漏）：

| 测试 | 项 3 后 | 项 2 后 |
|---|---|---|
| HelloWorld | 251 / 620 / 882 | 246 / 605 / 867，漏 0 |
| Digester | 1297 / 7510 / 21078 | 1289 / 7481 / 21045，漏 0 |
| DeepCopy | 1649 / 10961 / 40300 | 1641 / 10934 / 40269，漏 0（S2 修复后首次复测，漏 1 → 0） |
| FileIOTest | 293 / 795 / 1062 | 288 / 780 / 1047，漏 0 |
| CollectorsDemo | 1034 / 5951 / 14375 | 1014 / 5835 / 14147，漏 0 |
| TestMethodHandleCombinators | 1140 / 7808 / 17539 | 1132 / 7780 / 17509，漏 0 |
| TestMethodHandleDirect | 1149 / 7829 / 18172 | 1141 / 7801 / 18142，漏 0 |
| TestStreamBasic | 400 / 1449 / 2481 | 370 / 1361 / 2145，漏 0 |

移出：`VirtualThreads`、`Continuation`、`ContinuationScope`（各例），及其下游（`Thread$Constants`、`TimeUnit`、CollectorsDemo 的反射构造访问器链等）。
`VirtualThread` / `BaseVirtualThread` 降为 type 级（instanceof 指令仍在，只需类型名）；`Blocker` 留 init 级（`FileOutputStream.write` 真实调用 `begin/end`，
其内 `CarrierThread` 分支已折，`CarrierThread` 不在闭包）。
**给 C3 的接口**：恒 false 的 instanceof 已体现在 `folds.dead_pcs`（真分支死区）；若再把该点作为常量导出（`consts` 增 `instanceof` 指令、类型 `Z`），
type 级的 `VirtualThread` 也可移出——需生成器消费 instanceof 常量，列为 C3 待定项。

**项 9-a（项 1 遗留 (A) 的真因）：反序列化放开字段的范围**。`AbstractPipeline.parallel` 未折的原因不是字段值域：调试确认其写入只有常量 `false`，
但 `ObjectInputStream.readObject` 可达（清单 `deserializers`）后**全部**非 static / 非 transient 字段按不折叠处理。反序列化只写可序列化类声明的字段
（`ObjectStreamClass` 按类描述逐个写；首个不可序列化超类及其以上的字段由该超类无参构造器初始化，走字节码），`AbstractPipeline` 不可序列化。
修法：清单 `[facts.field_writes] serializable_markers = ["java/io/Serializable"]`，字段声明类是其子类型时才因反序列化放开（`facts.rs::deser_writes`，
单测 `deser_writes_rule` / `serializable_markers_parse`）。结果（类 / 方法 / 上下文，漏）：

| 测试 | 项 2 后 | 本项后 |
|---|---|---|
| HelloWorld | 246 / 605 / 867 | 不变，漏 0 |
| Digester | 1289 / 7481 / 21045 | **1165 / 6630 / 17271**，漏 0 |
| DeepCopy | 1641 / 10934 / 40269 | 1623 / 10845 / 39770，漏 0 |
| FileIOTest | 288 / 780 / 1047 | 不变，漏 0 |
| CollectorsDemo | 1014 / 5835 / 14147 | **911 / 5283 / 12363**，漏 0 |
| TestMethodHandleCombinators | 1132 / 7780 / 17509 | 1057 / 6729 / 15599，漏 0 |
| TestMethodHandleDirect | 1141 / 7801 / 18142 | 1066 / 6748 / 15795，漏 0 |
| TestStreamBasic | 370 / 1361 / 2145 | 不变，漏 0 |

Digester / CollectorsDemo 的 `AbstractPipeline.evaluate` 并行分支（`dead_pcs` 增 `[56,76)`）折叠，移出 `java/util/stream` 60 / 26 类、`java/lang/invoke` 38 / 56 类。

**项 4 G6：`Pattern.flags0` / `has(CANON_EQ)`——按原设计不可达成，未实施**。`jdk/internal/icu` 现为 3 类（`NormalizerBase`、`$Mode`、type 级 `UCharacter`；
早期 +55 的大头已随 G1–G3 与本期各项消失）。`--why`：`Formatter.<clinit>@7 Pattern.compile(<常量正则>) → Pattern.<init>@108 → compile@24 has(CANON_EQ)
→ normalize → Normalizer.normalize → NormalizerBase`；另 `sequence@175/285`、`atom@219` 在解析中途再查 `has(CANON_EQ)`。实测反例：`Pattern.addFlag`
（内联标志 `(?c)`，`case 'c': flags0 |= 128`）可达——任何正则解析都经 `group0 → addFlag`。按原设计的流不敏感「可能置位掩码」，`flags0` 的掩码含 128，
`has(CANON_EQ)` 不可折叠；即使做到对象 / 流敏感（`compile@24` 时 `addFlag` 尚未执行），`sequence` / `atom` 的解析期检查仍依赖「正则里没有 `(?c)`」，
这只有对常量正则执行解析才能得知。结论：G6 并入项 3 第 1 部分（构建期类初始化：`Formatter.<clinit>` 在分析期执行，`Pattern` 对象图成为快照事实，
解析器与 `CANON_EQ` 分支整体不入链）。`UCharacter` 另经手写 `sun/text/Normalizer.getCombiningClass`（`String.toUpperCase(Locale)` → `ConditionalSpecialCasing`）
以 type 级入闭包，与 G6 无关，属手写层对分析不透明（规范 §六），随该手写按字节码翻译消失。
**G6 修订目标（2026-10-01 用户决策：按实测修订，不删项）**。原目标「流不敏感可能置位掩码使 `jdk/internal/icu` 增量归零」不可达的原因：
① `addFlag` 的 `flags0 |= 128` 由解析器对内联标志 `(?c)` 执行，任何正则解析都可达，掩码必含 `CANON_EQ`；② 即便流敏感，`sequence` / `atom` 在解析中途
再查标志，是否置位取决于正则文本本身——这是「对常量输入执行解析器」才能回答的问题，任何不执行代码的抽象域都给不出。
修订后的量化目标（两段，均以 `--why` 与 dyn 对照验收）：
- **G6-a（C3 构建期初始化落地时达成）**：常量正则（`ldc` 字符串直接传入 `Pattern.compile`，且位于构建期执行的 `<clinit>` 内）经 `Pattern.has(CANON_EQ)`
  入链的类 = **0**；9 例中经 `Pattern.normalize` 入链的 `jdk/internal/icu` 类由现 2 类（`NormalizerBase`、`$Mode`）降到 **0**。依赖项 3 的 `build_time_init`
  快照（`Pattern` 对象图成为事实，解析器不入链）。
- **G6-b（手写层随 C1d 翻译）**：`UCharacter` 经手写 `sun/text/Normalizer.getCombiningClass` 的 type 级入口 = **0**（该手写按字节码翻译后其依赖可见，
  不再以不透明方式引入）。
- 分析器侧本期不再为 G6 增加抽象域：成本与收益均为零（两段目标都不由分析器规则达成）。

**项 6 SystemJavaLangAccess 选择失败——分析器侧已无缺口，残余随 C1d 删除手写消失，无需新规则**。原记录的 FileIOTest 3 条
（`decodeASCII` / `encodeASCII` / `inflateBytesToChars`）在本期基线已为 0。残余 unresolved 全部是 `hwobj_target` 在手写对象上找不到方法：
`SharedSecrets.getJavaLangAccess` 的手写实现返回手写 `SystemJavaLangAccess`（`jvm_boundary`，过渡类），它只实现了部分接口方法——
Digester 1 条（`getDeclaredPublicMethods`）、DeepCopy 11 条（`addExports` / `addOpens` / `addReads` / `defineClass` …）、
TestMethodHandleCombinators 2 条（`defineClass` / `protectionDomain`）；另 `NativeLocaleAdapter.getBreakIteratorProvider` 同属手写对象残缺。
这是手写层不完整，不是分析精度问题：给分析器补「选择失败时回落到接口全部实现」只会把缺失手写掩盖成过近似。
实测：本分支分析器 + c1d-final（80c5c1df）runtime（已删 `java_lang_access_impl.rs`，`SharedSecrets` 走 `System.<clinit>` →
`setJavaLangAccess(new System$2)` 字节码），Digester / TestMethodHandleCombinators / CollectorsDemo / FileIOTest / HelloWorld 的 unresolved 全部为 0
（类数 471 / 725 / 518 / 228 / 194，含 c1d-final 清单收窄的影响，不与本表直接对照）。终态：c1d-final 合入即清零，分析器不改。

**项 5 ServiceLoader：模块 `provides` / 类路径服务文件作为事实**。原先 `ServiceLoader` 的 provider 构造（`ProviderImpl.newInstance` 反射构造、
`findStaticProviderMethod` 反射调用）在闭包里只有 open 反射，provider 类本身不入链；TestCharsetForName 动态对照漏 1 = `CopyOnWriteArrayList$COWIterator`
（JVM 引导层为 `CharsetProvider` 绑定了 jdk.charsets 的 `ExtendedCharsets`，`Charset$ExtendedProviderHolder` 迭代 provider 时加载）。
修法（分析层通用规则 + 清单锚点，无类名）：
- `classfile/module.rs` 解析 `module-info.class`（`Module` / `ModuleResolution` 属性：requires（去 `requires static`）、无限定 exports 有无、uses、provides、
  `DO_NOT_RESOLVE_BY_DEFAULT`），`resolve::ClassPath::module_views` 给出各档案的模块描述符与 `META-INF/services/*`（单测 `module_info_parse`）。
- `seeds/services.rs::catalog`：与 JVM 默认启动（类路径应用、无 `--add-modules`）同构的引导层——根 = 有无限定 exports 且未标 `DO_NOT_RESOLVE_BY_DEFAULT`
  的系统模块，沿运行期 requires 闭包，再按已解析模块的 `uses` 绑定 provider 模块到不动点（JDK 21 实测 69 个 jmod 解析 62 个，与
  `java --show-module-resolution` 一致）；模块 provider 在前，其后是用户 / 库档案的 `META-INF/services` 条目（单测 `boot_layer_binds_used_services` /
  `service_file_lines`）。
- 清单 `seeds.toml [services]`：`lookups` = `ServiceLoader` 三个构造器及服务 Class 形参序号；`population` = 模块服务目录的装填方法
  （`ServicesCatalog.create` / `addProvider`）。`engine/services.rs::service_lookup`：lookups 调用点上服务 Class 实参值集里的类镜像即被查找的服务
  （值集增长时站点重跑）；含所指未知的 Class 时按全部服务处理（健全回退，`services_unknown = true`）。入选 provider 按 `ServiceLoader.loadProvider`
  的构造途径入链：命名模块中声明了 `public static provider()` 的取该方法，否则实例化 + 公开无参构造器（类初始化、反射名登记、形参 open、返回交 VM）。
  有模块 provider 入选时 `population` 作根。

结果（类 / 方法 / 上下文，漏；「无本项」= 同分析器、清单去掉 `[services]`）：

| 测试 | 无本项 | 本项后 | 查找到的服务 |
|---|---|---|---|
| HelloWorld / FileIOTest / CollectorsDemo / TestMethodHandleCombinators / TestMethodHandleDirect / TestStreamBasic | 同上表「本项后」列 | 不变，漏 0 | 无 |
| Digester / DeepCopy | 1165 / 6630 / 17271、1623 / 10845 / 39770 | 不变，漏 0 | `InetAddressResolverProvider` / `URLStreamHandlerProvider`，引导层无 provider |
| TestCharsetForName | 1157 / 7293 / 16810，**漏 1** | **429 / 1492 / 2405，漏 0**（多 127） | `CharsetProvider` → jdk.charsets `ExtendedCharsets` |

TestCharsetForName 新增 12 类（`ExtendedCharsets`、`AbstractCharsetProvider`、`CopyOnWriteArrayList` / `$COWIterator`、`WeakPairMap*` 等），漏覆盖清零。
同时移出 740 类不是本项的直接效果：调试确认「无本项」一侧 `ReflectionFactory.loadConfig` 的系统属性读取未折（`fold_props = 0`），
`jdk.reflect.useNativeAccessorOnly = "true"`（清单事实）没有生效，反射构造走进 `MethodHandleAccessorFactory → unreflectConstructor` 与
`String.format → Formatter → regex`。原因是乐观阶段收尾后的顺序敏感：第一次排空时（1445 个方法节点）乐观假设关闭，其后新入链的
`GetPropertyAction.privilegedGetProperties` 先于其被调方法 `System.getProperties` 分析，被调方法「尚无返回」按值未知答复，返回常量格记为 Top；
被调方法随后给出带属性表标签的常量，并入 Top 仍为 Top → 系统属性全部转不稳定。本项改变了补种时机，该方法恰好在被调方法之后分析，所以没有触发。
这是引擎缺口，已按项 9-b 修复（见下）。修复后本项的净效果：TestCharsetForName 407 / 1299 / 2055、漏 1 → 429 / 1492 / 2405、漏 0（+22 类为 provider 及其实例化链，健全性修复）。

**给 C3 的接口（项 5）**：`closure.json` `seeds.services: [{service, providers: [{module, class}]}]`（`module = null` 为类路径 provider）与 `seeds.services_unknown`。
生成器 / 运行时需要：① 引导期按事实建立模块服务目录——对每个模块 provider 以 `ServicesCatalog.create` + `addProvider(模块, 服务, provider 类)`
装填所属加载器的目录（模块 → 加载器按 JDK 模块加载器映射：jdk.charsets 等在平台加载器），`BootLoader.getServicesCatalog` / `ClassLoader` 的
`ServicesCatalog` 返回它；现行手写 `ServicesCatalog.findServices` 恒返回空、`BootLoader.getServicesCatalog` 返回 null，与 JVM 行为不一致
（运行时拿不到 `ExtendedCharsets`，`Charset.forName` 的扩展字符集查不到），属手写层终态待改项；② 顺序按事实给出的顺序（模块声明序）；
③ 类路径 provider 需要把 `META-INF/services/<服务>` 作为资源嵌入，`ClassLoader.getResources` 可读，provider 类名已登记在 `reflect` 按名表中。

**项 9-b 收尾阶段「尚无返回」答复的顺序依赖（引擎缺口）**。现象与 `--why` 见项 5：第一次排空后乐观假设全局关闭，其后入链的方法遇到**尚未建节点 /
尚未分析**的被调方法时按值未知答复，Top 并入调用方的返回常量格，被调方法随后给出常量也撤不回（格只升不降）。TestCharsetForName 中
`privilegedGetProperties` 因此记为 Top，系统属性全部转不稳定，`jdk.reflect.useNativeAccessorOnly` 事实失效，带入 `MethodHandleAccessorFactory`、
`Formatter`、regex 等 700+ 类；结果取决于处理次序（项 5 改变补种时机就让它消失）。调试数据：收尾前 `never` 549 条答复中只有 14 条的被调方法确实没有返回路径，
其余在收尾时都已有返回常量。
修法（`engine/noreturn.rs`，通用规则）：收尾阶段区分两种缺席——被调方法**还没有节点、有节点未分析、或其当前分析自身含未定论的「不返回」答复**时
仍答「不返回」并记入 `never`，排空后重算；全部节点已分析且没有返回路径才是定论，按值未知求值（folds 规则 7 不变）。`never` 改为「当前分析含该答复的节点」
（重分析时先移出）。每次排空重算 `never`，为空即结束；两次排空的 `never` 相同（互等的环、接收者恒空永不建节点的目标）或满 32 轮时转为全部按值未知，
保证终止。单测 `closing_answers_and_stall`。
结果：TestCharsetForName（无项 5 清单时）1157 / 7293 / 16810 → **407 / 1299 / 2055**；其余 8 例类 / 方法 / 上下文与项 5 后完全相同，漏均为 0。
同类剩余（性能线 §4.5 记录）：`CalendarSystem.forName` 的按名查找结果依赖求值次序——真因是重分析的依赖登记缺失，已按项 9-c 修复（见下）。

**项 7 DeepCopy 数组汇聚：真正的汇聚源不是 arraycopy，而是基本元素数组视图被当作可改写引用元素**。
上界实验（临时去掉全部 `W→E` 边，不健全）：DeepCopy 1623 → 1619、Digester 1165 → 1158、CollectorsDemo 911 → 907、MH Combinators 1057 → 1051。
少掉的类（`StringUTF16$CharsSpliterator`、`ReduceOps$5ReducingSink` / `$6`、`IntBinaryOperator` 等）经 `--why` 与折叠对照全部来自**真实流的丢失**：
`Currency$CurrencyProperty.getValidEntry@32` 变成 `null_recv`，因为 `Properties` 值经 `ConcurrentHashMap` 表数组的 Unsafe 写入（`casTabAt`）/ 读取（`tabAt`）
在同一数组上往返，砍掉 `W→E` 即砍掉真实流——**任何健全的拆分都不会减少类**。按分配点拆分 arraycopy 站点也不改变集合：站点的 `W` 已是按调用点的枢纽，
拆成 (src 数组, dst 数组) 对边得到同一个完全二部连接；只有让源 / 目标按调用上下文相关联（克隆上下文）才会更精，那属于上下文敏感度问题而非汇聚点拆分。
按站点计数（临时插桩，每站点新增数组）：DeepCopy 共 5 477 256 条，其中 `Object.equals` 各调用点派发到手写 `sun/nio/fs/UnixPath.equals` 形成的站点占绝大多数
（单站点 3 030 个数组；`ConcurrentHashMap$Node.equals` 内的站点合计 150 万条，`Objects.equals` 73 万条，`HashMap.getNode` / `TreeNode.find` 等同类）。
原因：`UnixPath.equals` 手写体经 `to_u8(&JArray<i8>)` 读自身 `byte[]` 路径，扫描器只认 `JArray` 标识符即标 `array_access`，`hw_writes` 保守规则于是把
`Object` 形参当写入目标、来源为全部实参数组元素——凡传给 `equals` 的数组元素两两并成一个集合。
修法（`handwritten/syntax.rs::ArrayIdents`，通用规则，无类名）：`array_access` 改为「取得**引用元素**数组视图」：`JArray<P>` / `try_cast_array::<P>` 的 P
为基本元素类型（i8 / u16 / i16 / i32 / i64 / f32 / f64 / bool）不计，`array_store_byte` 不计；引用 / 泛型 / 未写明元素类型的 `JArray`、`__view_into`、
`array_store_object`、宏内标识符照旧保守计入。基本元素数组的元素不携带类型，不可能写入引用，健全。单测 `ref_array_access`。
结果：DeepCopy 类 / 方法 / 上下文 1623 / 10845 / 39770 → **1623 / 10806 / 37735**，耗时 45 s → 13 s，站点新增数组 5 477 256 → 39 403（剩余写入扇入是
`System.arraycopy` 按清单语义的逐站点扇入，如 `Arrays.copyOf` 单上下文 2 714 个源数组，属上下文敏感度问题）；其余 8 例类 / 方法 / 上下文不变；9 例动态对照漏均为 0。

**项 8 类初始化事实（`Unsafe.ensureClassInitialized` 等）**。原状：`ensureClassInitialized` 是整方法手写边界（空操作），分析器不建模其效果——
`ldc X.class` 只让 X 进 type 层（JVMS §5.5 类字面量不触发初始化），X 的 `<clinit>` 不入链。例：DeepCopy `DirectMethodHandle.<clinit>@85`
的 `DirectMethodHandle$Holder` 原为 type 层；`SharedSecrets.getJavaXxxAccess` 的「先 ensureClassInitialized(目标类) 再读静态字段」形状下，目标类
`<clinit>` 缺席会让静态字段值域缺少其写入（健全性缺口，C1d 后 SharedSecrets 按字节码翻译即暴露）。
实现（通用规则，类名只在清单）：`vm_intrinsics.toml [facts.class_init.initializers]` 登记「初始化以实参传入的类」的成员 → Class 形参序号
（`Unsafe.ensureClassInitialized` / `ensureClassInitialized0`）；`engine/class_init.rs` 在调用点取 Class 实参值集里的类镜像（常量 / 值集，增长时站点重跑），
所指类（数组类除外）进入初始化层，按调用点记录；值集含 open / 非镜像值记 `unknown`。输出 closure.json：
`class_init: { targets: [类…], sites: [{ site: "调用方成员@偏移", classes: [类…] }], unknown: bool }`。单测 `class_initializers_parse`、`targets_union_sites`。
结果（类 / 方法 / 上下文，漏）：HelloWorld、Digester、FileIOTest、CollectorsDemo、TestStreamBasic、TestCharsetForName 类 / 方法 / 上下文不变
（Digester：`VarHandleGuards` type → init）；DeepCopy 1623 / 10806 / 37735 → 1623 / **10808 / 37737**（新入 `CallSite.<clinit>`、`InetAddressResolverProvider.<clinit>`；
4 个 `$Holder` 等 type → init；10 个站点 723 个目标，其中 `MethodHandleAccessorFactory.ensureClassInitialized@14` 718 个——反射字段访问器的
`field.getDeclaringClass()` 值集即全部类镜像，另有 unknown）；MH Combinators 1057 → **1058**、MH Direct 1066 → **1067**（+`VarHandleGuards`，
+`VarHandle.<clinit>`）；9 例漏均为 0。
MH 两例的 +1 经 `--why` 为不精确：`VarHandle` 由 `DirectMethodHandle.shouldBeInitialized@104` 初始化，该处 `member.getDeclaringClass()` 的值集是
`MemberName.clazz` 字段汇合的 18 个镜像；动态对照中 `VarHandleGuards` 属 extra（JVM 未加载），即 `VarHandle.<clinit>` 实际未执行。
不精确的来源是 MemberName 声明类的值集（字段按对象敏感不足），不是本事实本身；列为项 9 候选。
**C3 生成器接口**：生成器按 `class_init.targets` 为每个带 `<clinit>` 的目标类登记初始化钩子（与用户类 / 注解枚举的 `CLASS_INIT_HOOKS` 同一张表，
键为类的 binary name）；手写 `Unsafe.ensureClassInitialized(0)` 由空操作改为 `crate::ensure_class_initialized(c)`——按类镜像名查钩子执行，
查不到即无 `<clinit>` 或未入闭包（此时语义为空操作）。`unknown = true` 时钩子表覆盖闭包内全部带 `<clinit>` 的类（`clinit` 列表），
保证所指未知的实参同样可初始化。`sites` 仅供溯源 / 审计。`build_time_init`（§6.11 项 3）与本事实共用这张钩子表。

**项 9-c 按名取类 / 按名查方法站点的重分析遗漏（健全性，引擎缺口）**。现象：CollectorsDemo `CalendarSystem.forName@81`（`Class.newInstance`）折叠为
`null_recv`，JapaneseImperialCalendar → `forName("julian")` 这条反射实例化链被整体切掉。调试数据（临时插桩）：`@78 Class.forName` 的按名取类只求值了一次——
当时字段 `names` 的值格仍是初值 `Const(Null)`（`initNames` 尚未处理），`@45 ConcurrentMap.get` 的接收者为 `Null`、无读者登记可挂，常量表读取给出空集，
结果 `Some([])` 不接返回边；`initNames` 的 `putstatic names` 随后使值格升为 Top、`forName` 重分析，但重分析只重跑**事件变化了的偏移**：`@45` 变了，
`@78` 的事件（实参仍是 `Src::Site(50)`）没变，而它的求值恰恰读的是 `@45` / `@50` 的事件内容——于是永远停在空集。
这不是 `class_lookup` 自身的非单调：值集只增、`table_read` 一旦因非常量表类型 / open 给出 None 就保持 None，返回边一经接上不撤；缺的是依赖登记。
修法（`engine/bytecode.rs`，通用规则）：求值会回溯同一分析里其他偏移事件的站点（按名取类、按名查方法）登记为「整体依赖站点」（`whole_sites`）；
重分析只要有事件变化，这些站点连同变化的偏移一起重跑（仍在新分析里的偏移才跑）。单测 `whole_rerun_keeps_live_offsets`。
结果（类 / 方法 / 上下文，均相对项 8）：HelloWorld、FileIOTest、TestStreamBasic、TestCharsetForName 不变；Digester 1165 / 6630 / 17271 → 1166 / 6638 / 17300、
DeepCopy 1623 / 10808 / 37737 → 1624 / 10816 / 37751（均 +`Class$1`）；CollectorsDemo 911 / 5283 / 12363 → **933 / 5372 / 12560**；
MH Combinators 1058 / 6730 / 15600 → **1070 / 6803 / 15752**；MH Direct 1067 / 6749 / 15796 → **1078 / 6818 / 15903**；9 例漏均为 0。
增加的类经 `--why` 全部经 `CalendarSystem.forName@81 → Class.newInstance → ReflectionFactory.newInstance → Constructor.acquireConstructorAccessor`
（`MethodHandleAccessorFactory`、`DirectConstructorHandleAccessor`、`ConstructorAccessorImpl` 等），是先前不健全折叠漏掉的可达内容（本次运行 JVM 未走
`forName("julian")`，所以动态对照看不出）。其中 CollectorsDemo 11 个、MH 两例各 3 个 JVM 实际加载了，但经另一入口（`getEnumConstantsShared → Method.invoke`
的边界派发，动态对照归 `boundary-dispatch`），与本链无关。
精度余量：`names` 的值来自 `"sun.util.calendar." + namePairs[奇数]` 写入 `ConcurrentHashMap`，精确答案只有 `Gregorian` / `LocalGregorianCalendar` /
`JulianCalendar` 三类；需要「映射值的字符串来源」建模（按容器对象的值槽追踪常量拼接），不在本期范围，记为后续候选。性能线 §4.5 所记
「批量排空时 CollectorsDemo 丢 11 类 / 69 方法」即本缺口的另一次序表现，修复后与次序无关。

**合并 rust-closure-analyzer 6e0849c6（closure-mono 顺序依赖修复）**。上游以「跨偏移读者」`xreaders`（按名取类 / 按名查方法，重分析即重跑，事件全同也跑）
加按调用点粘滞的 top（`lookup_top`）修了同一缺口，覆盖项 9-c 的 `whole_sites`；合并时取上游实现、删去本分支的重复登记（语义等价且更强）。
合并后（类 / 方法 / 上下文）：HelloWorld 246 / 605 / 867、Digester 1166 / 6638 / 17300、DeepCopy 1624 / **10821** / 37756、FileIOTest 288 / 780 / 1047、
CollectorsDemo 933 / 5372 / 12560、MH Combinators 1070 / **6807** / 15756、MH Direct 1078 / **6822** / 15907、TestStreamBasic 370 / 1361 / 2145、
TestCharsetForName 429 / 1492 / 2405；9 例漏均为 0（DeepCopy / MH 两例各 +4～5 方法来自上游的顺序修复）。
顺序矩阵（`--flow-batch 1 / 64` × `--hash-seed 0 / 12345`）：HelloWorld、CollectorsDemo、DeepCopy 四种组合的类集与方法集逐一相同。

**构建期类初始化（2026-10-01 用户决策）**：保留项 3 第 1 部分的设计（`build_time_init`：`<clinit>` 在分析期执行、对象图成为快照事实），
归 C3 实施，本期不做；G6-a 随之达成。

**运行期服务目录按分析器的服务事实装填（决策 4，只改 runtime/）**。项 5 让分析器导出 `closure.json` 的 `seeds.services`，但运行期
`BootLoader.getServicesCatalog` 恒 null、`ServicesCatalog.findServices` 手写恒返回空 `ArrayList`——分析按事实建模 provider、运行期却找不到它，两侧不一致。
改动：
- `runtime/java_runtime/build.rs` 读 `../closure_input/closure.json`（两条流水线的 scratch 布局相同；缺席即空表），把模块 provider（`module` 非 null）按事实序
  写成 `OUT_DIR/services_table.rs`。build 依赖只有 std，就地极简 JSON 解析（8 MB 的 DeepCopy closure.json 未优化构建 66 ms）。
- `ServicesCatalog::__boot_catalog()`：进程唯一，首次请求时 `create()` + 逐条 `addProvider(provider.getModule(), 服务, provider)`（字节码翻译体）；
  `BootLoader.getServicesCatalog()` 与 `JavaLangAccess.getServicesCatalog(ModuleLayer)`（boot 层目录）都返回它，二者各声明 `create` 回调边；
  `addProvider` 仍由 `seeds.toml [services] population` 在有模块 provider 时作根，与运行期只在表非空时调用一致。
- 删手写 `findServices`（改按字节码 `map.getOrDefault(service, List.of())`）与 `__empty_layer_catalog`；`getServicesCatalogOrNull` 保留（过渡：`CLV.get`
  经边界存根）。运行期全部类由 boot 定义，模块 provider 在 boot 一步命中（JDK 上 `jdk.charsets` 在 platform 一步命中），迭代结果相同。
- 补手写 `BootLoader.loadClass(Module, String)`（原为存根，`Class.forName(Module, String)` 在模块加载器为 null 时进入；照字节码：`loadClassOrNull` 后比对
  `getModule()`）。
- 未覆盖：类路径 provider（`META-INF/services`，`module` 为 null）仍无运行期资源表（FS-C2）；命名模块的静态 `provider()` 工厂路径——运行期模块恒未命名，
  `inExplicitModule` 为 false，走构造器，与 JDK 对 `ExtendedCharsets` 的行为一致（其无 `provider()`）。
结果（类 / 方法，相对合并后）：Digester 1166 / 6638 → 1166 / 6640（+`ServicesCatalog.create` / `<init>`、`ConcurrentHashMap.getOrDefault`、`BootLoader.loadClassOrNull`，
−`ArrayList$SubList.toArray` / `Collections$EmptyList.toArray`）；DeepCopy 10821 → 10824；TestCharsetForName 429 / 1492 → **428 / 1486**（手写 `findServices`
返回 `ArrayList` 带入的 `Collections$EmptyIterator` 等 8 方法消失）；其余 6 例不变；9 例漏均为 0。
**COWIterator 结论**：`--why` 为 `CopyOnWriteArrayList$COWIterator.<init> ← CopyOnWriteArrayList.iterator@9 ← ModuleServicesLookupIterator.iteratorFor@55
← … Charset.providers`。基线的漏由手写 `findServices` 引起：分析跟随手写返回的 `ArrayList`，JVM 迭代的是目录里的 `CopyOnWriteArrayList`；项 5 的
population 根已在分析侧覆盖（此后漏 0），本项让运行期与之一致。编译验证：TestCharsetForName scratch `cargo check` 通过（`services_table.rs` 含
`CharsetProvider → ExtendedCharsets`）；e2e 待协调方安排。

**顺带修复的两处生成失败（本分支精度项引入，编译 TestCharsetForName / HelloWorld 时暴露）**：
- **折叠常量类型不符**：项 1 的字符串纯函数事实让字符串值经声明为 `Object` 的返回（`Objects.requireNonNull(s, …)`、`System.getLogger` 内等）到达，
  `fold.rs` 把它按调用点导出为 `{"type": "Ljava/lang/Object;", "value": "UTF-16BE"}`，发射层输入（`input/norm.rs`）拒收，**全部 Rust 流水线构建失败**
  （9 例里 8 例有此类条目）。修法：`exportable(值, 读取点类型)`——整型族 ↔ 整数、J ↔ long、String 槽 ↔ 字符串、引用槽 ↔ null，其余不导出
  （同时挡住 `V::Class` / `V::Par` 被 `const_json` 编成 null 的潜在错误）。单测 `exportable_matches_slot_type`。类 / 方法集不变。
- **手写文件的派生类型需求**：`method_handle_ext.rs` 的 `interpret` 写 `mh.__get_form().__get_names()`；精度收窄后 TestCharsetForName 的
  `MethodHandle` 只在 type 级（`AccessibleObject.checkCanSetAccessible` 的 `ldc`），`LambdaForm` 不入闭包，`form` 字段被发射为 `Object`，编译失败。
  规则（`touch_hw_types`，通用）：类入生成范围即编译其手写文件，文件里经访问器 / 方法返回推得的接收者静态类型（`SType::Field` / `Call` / `Ret`
  链上各级）与类型路径同为 L1 需求，按 type 级入闭包。单测 `derived_nodes_walk_accessor_chain`。结果：HelloWorld / FileIOTest / TestStreamBasic
  +1 类（`Enumeration`，经 `ClassLoader` 手写），Digester / CollectorsDemo / TestCharsetForName +1 类（`LambdaForm`），方法数不变；9 例漏均为 0。
  修后 TestCharsetForName、HelloWorld 的 scratch `cargo check` 通过。
本段之后（类 / 方法 / 上下文）：HelloWorld 247 / 605 / 867、Digester 1167 / 6640 / 17288、DeepCopy 1624 / 10824 / 37760、FileIOTest 289 / 780 / 1047、
CollectorsDemo 934 / 5372 / 12560、MH Combinators 1070 / 6807 / 15756、MH Direct 1078 / 6822 / 15907、TestStreamBasic 371 / 1361 / 2145、
TestCharsetForName 429 / 1486 / 2375。

**合并 rust-closure-analyzer f6d80103（closure-mono + emitter-perf2，共享类数据 Rc → Arc）**：无文本冲突；唯一适配为项 5 的
`resolve/classpath.rs::module_views`——上游 `archives` 改为 `Mutex<Vec<Archive>>`、来源移到并列的只读 `origins`，改为 `origins.zip(lock(archives))`，
失败记录走 `lock(&self.failures)`。合并后 9 例类 / 方法 / 上下文与上表逐一相同，漏均为 0；顺序矩阵（HelloWorld / CollectorsDemo / DeepCopy ×
`--flow-batch 1 / 64` × `--hash-seed 0 / 12345`）类集与方法集四组一致（仅 via 不同）。

**项 9 候选：封存的静态映射值 / 常量字符串数组（`CalendarSystem.names`）**。按名取类拆段新增两种来源（`engine/sealed.rs`，通用规则，类名只在清单）：
- **值映射字段**：private static 字段，嵌套（宿主 + 全部 `NestMembers`）内每次 putstatic 的值都是「新建 `[facts.reflect.value_maps] classes` 的实现类 →
  无引用实参的构造 → 若干次 `writers` 写入 → putstatic 本字段」（或 null），映射对象别无他用；每次 getstatic 的值只作 `readers` 的接收者。映射不逃逸，
  读取结果只能是某次写入的值或 null：候选 = 各写入值按拆段规则（`name_parts`）求得字符串集的并。
- **常量字符串数组字段**：private static `String[]`，每次写入是新建数组、元素只存字符串常量 / null、除 putstatic 外只被元素读取；每次读取只作数组元素读取。
  读取下标奇偶已知（`V::Par`）时只取同奇偶下标处存入的常量——`namePairs` 为「名, 类名」交错数组，`initNames` 以 `i += 2` / `i + 1` 访问。
- 嵌套内访问方法以不读事实的 Oracle 分析（结果只取决于字节码，与处理次序无关）；private 字段在嵌套外无字节码访问，反射 / Unsafe 写 private static
  不建模（与常量折叠同一前提）。任一条件不满足即不给候选。
- **构建器复用的健全性**（`builder_head` + 新增 `absint/cfg.rs`）：`initNames` 在循环里复用一个 `StringBuilder`，每轮先 `setLength(0)`。链首判定改为：
  新建构建器的使用只能是构造器、实参常量 0 的清空（`[facts.string_concat] resets`）与链上第一次使用 p；且从 p 出发、不经构造器 / 清空点回不到 p
  （基本块 CFG 可达性，异常边从块首保守进入处理器）。有清空时丢弃构造器实参内容。原规则只要求「构造 + 一次使用」，对「循环外构造、循环内追加且不清空」
  的写法会漏掉上一轮残留的内容——本次一并堵上。
- 单测：`cfg::loop_append_needs_reset_in_body`、`sealed::tests::{map_writes_collects_values, map_writes_rejects_escape_and_unlisted,
  array_writes_constants_only, builder_head_requires_reset_in_loop}`（虚构类名）；清单解析测试补 `value_maps` / `resets`。
结果（类 / 方法 / 上下文，相对合并后）：HelloWorld、FileIOTest、TestStreamBasic、TestCharsetForName 不变；Digester 1167 / 6640 / 17288 → **1168 / 6657 / 17305**、
DeepCopy 1624 / 10824 / 37760 → **1625 / 10841 / 37777**、CollectorsDemo 934 / 5372 / 12560 → **935 / 5389 / 12577**、MH Combinators 1070 / 6807 / 15756 →
**1071 / 6824 / 15773**、MH Direct 1078 / 6822 / 15907 → **1079 / 6839 / 15924**；9 例漏均为 0，耗时不变（DeepCopy 11.9 s）。
增量逐一核对（5 例相同）：+1 类 `JulianCalendar$Date`，`JulianCalendar` 由 type 级升 code 级，+17 方法全部属于这两个类（`<clinit>` / `<init>` /
`getCalendarDate` 等）；无类 / 方法减少。原因：此前 `forName@81` 的 `Class.forName(className)` 为 top，`newInstance` 的构造器只能记 `open(Class)`
反射缺口而不展开——`JulianCalendar` 的构造器与方法体是**漏掉的可达内容**（JVM 在 `forName("julian")` 时会走到）；现在候选精确为
`Gregorian` / `LocalGregorianCalendar` / `JulianCalendar` 三类，按候选展开。反射缺口：CollectorsDemo、MH 两例的
`getDeclaredConstructors0 <- open(java/lang/Class)` 消失；Digester / DeepCopy 仍有，临时插桩确认 Digester 的 top 取类点只剩
`Provider$Service.getImplClass@64`、`ServiceLoader$LazyClassPathLookupIterator.nextProviderClass@207`、`URL$DefaultFactory.createURLStreamHandler@162`，
与本链无关（`forName@81` 已不再 top）。

**项 9 候选：`MemberName.clazz` 对象敏感 → 落为「类字面量与 Class 形参的引用比较」**。`--flows @srcs:VarHandle 镜像` / `@path` 实测：
`VarHandle` 镜像经 `MethodHandles$Lookup.findVirtual` 的 `refc == VarHandle.class` 分支（→ `findVirtualForVH` → `varHandleInvoker` 一族）进入，
而该方法 `refc` 形参值集里并无 `VarHandle` 镜像——`if_acmp` 两侧一边是 `ldc` 类字面量、一边是 Class 形参，原先一律按未知处理。
字段按对象敏感需要堆抽象，收益集中在这一条比较上，于是改做通用的比较规则：
- absint：`Oracle::param_mirror(i, cls)`（缺省 None）；`if_acmp` 比较 `Class(c)` 与值源恰为 `[Param(i)]` 的引用时，形参值集已知且不含 c 的镜像 → 恒不等，
  记入 `Analysis::mirror_assumed`（`Class(x)` 对 `Class(y)` 直接按类名比较）。
- 引擎（`engine/mirror_eq.rs`）：`param_mirror_sets` 取 Class 类型形参节点的值集，仅当全部元素都是所指已知的镜像伪类型时给出类集（含普通 `Class`
  对象或未知来源即 None）。分析采用的每条假设登记到 `mirror_watch[P(m, i)]`；节点增长（`node_grown`）到可能含该镜像时以 `Why::Mirror` 失效重分析，
  命中的登记随之撤销。单测 `absint::tests::class_literal_eq_folds_by_param_mirrors`、`mirror_eq::tests::{mirror_sets_known_only_for_pure_mirrors,
  growth_invalidates_only_when_answer_may_change}`（虚构类名）。
- 诊断：`--flows @srcs:<类>` 列出持有某类镜像 / 对象的全部源节点（与 `@opens` 共用代码）；`@path` 回溯在第一个无前驱持有该类的节点（源）处截止。
结果（类 / 方法 / 上下文，相对上一项）：DeepCopy 1625 / 10841 / 37777 → 1625 / **10827** / 37757、MH Combinators 1071 / 6824 / 15773 → 1071 / **6809** / 15753、
MH Direct 1079 / 6839 / 15924 → 1079 / **6824** / 15904；其余 6 例不变；9 例漏均为 0。减少的方法：`findVirtualForVH`、`MethodHandles.varHandleInvoker`、
`Invokers.{varHandleMethodInvoker, makeVarHandleMethodInvoker, cachedVHInvoker, setCachedVHInvoker, varHandleMethodInvokerHandleForm}`、
`MemberName.makeVarHandleMethodInvoke` ×2、`VarHandle$AccessDescriptor.<init>`、`LambdaForm.{basicTypeSignature, shortenSignature}`、`BasicType.basicTypeChar`、
`String.valueOf([C)`（MH 两例另有 `AccessMode.methodName`）。余下的 `VarHandle` 镜像源含 `Invokers.createFunction@134/159/182`（VH 的 NamedFunction 分支），
`VarHandleGuards` init 级与 `VarHandle.<clinit>` 仍在——后续「createFunction 按调用点常量克隆」项的收益因此变大。
耗时：DeepCopy 16–20 s（上项 11.9 s），但工作量计数相同，测时机器负载 6.75，不计为回归。
顺序矩阵：MH Combinators / CollectorsDemo 三组与缺省一致；DeepCopy 在 `--flow-batch 64 --hash-seed 12345` 下多 5 方法（`VarHandles.filterCoordinates`
一族，via `VarHandles.maybeAdapt@62`）。其终态折叠为 `maybeAdapt` 8–74 死代码，插桩确认：`MethodHandleStatics` 的 `<clinit>` 常量首次在常量实参
求值深度 3 处计算，`parseBoolean(getProperty(…))` 被深度上限截断，`VAR_HANDLE_IDENTITY_ADAPT` 以「非常量」写入缓存；系统属性不稳定集增长清缓存后才在
深度 0 重算为 false，其间 `maybeAdapt` 的调用边已加入。这是辅助分析记忆与计算上下文相关的既有缺陷（本规则改变处理次序而暴露），下一步单独修复。

**辅助分析记忆与上下文无关（`engine/memo.rs`，修上条的顺序依赖）**。`<clinit>` 常量（`consts`）、构造器摘要（`objs`）、属性读取摘要（`psums`）、
常量实参求值（`cevals`）按键记忆，但计算时受两种截断影响：递归保护（键已在进行中 → 未知）与常量实参求值深度上限 3。截断取决于外层正在算什么，
截断后的答复被记下后就随处理次序变化。规则：
- 四处记忆化计算统一为帧（`Guards`：进行中的键 → 层号）。递归保护命中键 K 时记下 K 所在帧的层号；一帧子树里的截断层都不低于本帧层号
  （截断只来自本帧自身的递归）才写入记忆，否则答复只供本次使用。
- 深度：`<clinit>` 常量 / 构造器摘要 / 属性读取摘要从深度 0 开始算（外层深度不影响），常量实参求值的记忆键带起始深度。
  终止性不变：每层新帧都占用一个不同的进行中键。
- 于是记忆里的每个答复都等于在空上下文中计算该键的答复。单测 `memo::tests::{cut_below_frame_blocks_memo, self_recursion_keeps_memo}`。
实测：修前**所有顺序**下 `maybeAdapt` 都先按 `VAR_HANDLE_IDENTITY_ADAPT` 未知分析过（`MethodHandleStatics.<clinit>` 首次在深度 3 处计算，
`parseBoolean(getProperty(…))` 被截断），`filterValue@23` 等调用边永久留下，终态折叠却报 8–74 死代码；64/12345 只是多走了一步 `filterCoordinates`。
修后该字段从一开始即为 false（`MemberName.<init>` 的 `$assertionsDisabled` 同理由未知变为 true）。
结果（类 / 方法 / 上下文，相对上一项）：DeepCopy 1625 / 10827 / 37757 → **1597 / 9817 / 33686**（−28 类 / −1010 方法：`VarHandles.filterValue` 起的
VarHandle 适配一族、`VarHandleByteArrayAs*`、`IndirectVarHandle`、`NativeMethodHandle`、`InfoFromMemberName`、`UnsafeConstants`、`BitSet`、
`PrimitiveIterator` / `Spliterators$*Adapter` 等，按包 `java/lang/invoke` 754、`jdk/internal/misc` 199），无增加；其余 8 例不变；9 例漏均为 0。
DeepCopy 辅助分析次数 45635 → 53640（带截断的帧不记忆、重算），总耗时 13.6 s。
顺序矩阵（`--flow-batch 1 / 64` × `--hash-seed 0 / 12345`）：DeepCopy、HelloWorld、CollectorsDemo、MH Combinators 类集与方法集四组一致。

**项 9 候选：`createFunction` 按调用点常量克隆 → 落为「选择子形参的调用点上下文」（`engine/selector.rs`）**。`--why VarHandle$AccessDescriptor`（MH Direct）：
`ldc @ Invokers.createFunction:(B)@142 ← getFunction:(B)@17 ← invokeHandleForm@500`——`createFunction` 对 byte 形参 tableswitch 0–6，
各调用点传的编号不同，形参常量在共享节点汇合为 Top 后 VH 分支（case 2、4–6）都按可达处理。`MethodHandleImpl` / `DirectMethodHandle` 的
`getFunction → createFunction` 同构。改为通用规则：
- absint：int 族形参的入口值记为 `V::Arg(i)`（语义同 Top，只多一个来源标记，`PV::of` / 可导出 / 常量判定都按 Top）；直接作 switch 键或条件跳转操作数的
  形参记入 `Analysis::selector_params`。
- 选择子形参（`Ctx::selector_slots`）：静态字节码方法的上述形参，加上经 invokestatic 原样转给被调方选择子形参的形参（`getFunction` 转给 `createFunction`）。
  用不读事实的 Oracle 分析，按成员记忆；递归走 `memo.rs` 帧规则（成环截断的答复不记忆），与处理次序无关。
- 调用点（INVOKESTATIC，调用方为字节码）：选择子形参上传常量 → 被调方按调用点克隆，链尾接调用方上下文（`site_ctx_in`，截断到 HEAP_DEPTH）；
  否则调用方已在上下文中时继承之（常量可能来自调用方克隆的形参常量）。优先于「返回基本类型 / void 不克隆」规则。
  初版在调用方已有上下文时直接继承：MH Combinators 的 `invokeHandleForm` 在上下文中，其 @474 / @500 / @538 三个 `getFunction` 调用点的编号又汇合，
  `AccessDescriptor` 仍在；改为链尾接调用方上下文后消失。
- 单测 `selector::tests::{forwarding_maps_callee_slots_to_own_params, switch_on_param_marks_selector}`（虚构类名）。
结果（类 / 方法 / 上下文，相对上一项）：DeepCopy 1597 / 9817 / 33686 → **1579 / 9669** / 30595、MH Combinators 1071 / 6809 / 15753 → **1069 / 6801** / 18192、
MH Direct 1079 / 6824 / 15904 → **1070 / 6758** / 17878；其余 6 例类 / 方法不变，上下文 +2%–+10%（HelloWorld 867 → 901、Digester 17305 → 18728、
CollectorsDemo 12577 → 13844）；无类 / 方法增加；9 例漏均为 0。MH 两例减少 `VarHandle$AccessDescriptor`、`CallSite`（Direct 另有 `Invokers$Lazy`、
`MethodHandleImpl$CasesHolder` / `$LoopClauses`、`MethodHandle$1`、`ReduceOps$5` / `CountingSink` 一族）。
DeepCopy 的 −18 类 / −148 方法来自条件跳转判据：临时插桩逐个跳过被调方的克隆（`SELSKIP`，未提交），只有跳过 `Arrays.fill([Object;IILObject;)V`
时恢复为 1597 / 9817，跳过 `Formatter` / `BreakIterator` / `MethodType` / `Math` / `BigDecimal` / `Arrays.{sort, copyOf, spliterator, …}` 均不影响。
`fill` 以 from / to 作循环判定，原先返回 void 按本体共享，`Collections$CopiesList.toArray` 填入的元素与 `ObjectInputStream$HandleTable.clear`
等的数组汇合；按调用点克隆后各数组的元素流分开，减少的类为 `URLConnection` 一族（`FileURLConnection` / `JarURLConnection` / `JavaRuntimeURLConnection`、
`ParseUtil`、`Proxy`、`FileNameMap`、`Collator`）与 `IdentityHashMap` / `WeakHashMap` / `ArrayList$SubList` / `Collections$2` / `RangeIntSpliterator` 等
Spliterator。只用 switch 判据的对照：MH 两例收益相同、DeepCopy 无收益且上下文 34107，故保留条件跳转判据。
耗时：DeepCopy 13.6 s → 6.8 s（上下文减少），其余持平。顺序矩阵：HelloWorld、CollectorsDemo、DeepCopy、MH Combinators 四组一致。

**项 9 候选：noreturn 折叠标记（导出给 C3，不改变闭包）**。定论阶段对「全部节点已分析且没有返回路径」的被调方法按值未知答复，
其后的代码照常分析（folds 规则 7：死区只从跳转 / switch / return / athrow 之后开始），所以永不返回的调用之后仍按可达导出、照常翻译。
新增按条目的两个字段（folds_version 仍为 2，与 `null_recv` 同为附加字段，现有消费方忽略即保持原行为）：
- `noreturn_calls`：定论不返回的活调用点——唯一目标为字节码方法（无清单返回事实、不走值相等 / 字符串运算 / 属性读取的派生结果），
  目标有节点且全部节点已分析，返回常量格缺席（没有任何克隆含返回点）。判定只看分析终态，与处理次序无关。
- `noreturn_dead_pcs`：把这些调用点当作控制流终点时另外不可达的区间（与 `dead_pcs` 不相交，格式同 `dead_pcs`）：只沿原本可达的指令走，
  覆盖指令的处理器原本可达即计入（保守）。C3 发射层可在调用后终止控制流（如 `unreachable!()`）并删去这些区间；两字段并用时 `dead_pcs ∪
  noreturn_dead_pcs` 满足「死区只从跳转 / switch / return / athrow 或 noreturn 调用之后开始」。
- summary 增加 `fold_noreturn_calls` / `fold_noreturn_dead_bytes`。单测 `fold::tests::cut_after_stops_fallthrough_keeps_handlers`（虚构类名）。
实测（调用点 / 另外不可达字节）：HelloWorld、FileIOTest、TestStreamBasic 0 / 0；Digester 60 / 15；DeepCopy 79 / 56；CollectorsDemo 22 / 5；
MH Combinators 37 / 20；MH Direct 36 / 19；TestCharsetForName 21 / 6。多数调用点其后紧跟 `athrow`（如 `throw uncaughtException(ex)`）或跳转，
另外可达的只是其后的 `goto` / 返回。抽查 `ObjectStreamClass.invokeReadObject@68`（`throwMiscException` → `goto 92`）、
`BoundMethodHandle.arg@146`（`uncaughtException` → `athrow`）、`BigInteger.checkRange@29`（`reportOverflow`）字节码均符合。
9 例类集 / 方法集与上一项完全相同；DeepCopy、CollectorsDemo 在 `--flow-batch 64 --hash-seed 12345` 下标记逐项相同。

**e2e 回归修复：边界包内的服务 provider 按普通构造建模（TestCharsetForName）**。1b87995d 的 e2e 中 TestCharsetForName 命中存根
`sun/nio/cs/ext/AbstractCharsetProvider.<init>:(Ljava/lang/String;)V`（主干通过）。根因（closure.json 实测）：服务目录装填后，
运行期 `ServiceLoader` 真的实例化了 jdk.charsets 模块的 provider `ExtendedCharsets`；该类与父类 `AbstractCharsetProvider` 位于
`[boundary]` 前缀 `sun/`、不在放行清单，分析器把 `ExtendedCharsets.<init>`（经 `service-provider` 入链）与 `AbstractCharsetProvider.charsetForName`
（经 `Charset.lookupExtendedCharset@35`）登记为 `handwritten:boundary`、不展开方法体；发射层却对无手写承载的边界方法照常翻译字节码，
体内调用的父类构造器、`charset` / `init` / `canonicalize` / `lookup` 不在闭包 → panic 存根。主干服务目录为空，provider 从未被实例化，
所以没有暴露。provider 入链本身（`instantiate` + `init` + 构造器）已是普通 `new` 语义，缺的是边界截断把构造链与继承方法体切掉。
- 修法（通用规则，无类名）：`Catalog::provider_lines` 取服务目录全部 provider 及其超类型闭包（超类链 + 超接口），这些类即便落在边界前缀
  也归翻译域（`Ctx::domain`，与纯数据资源束同一位置；`[vm_boundary]` 类仍按方法划分）。ServiceLoader 按普通构造实例化 provider，构造链、
  字段初始化、`<clinit>` 与继承的实例方法运行期执行的都是字节码，按字节码分析与发射层口径一致；共置手写体提供的方法仍取手写效果。
  判定只依赖类路径与模块声明（静态），与处理次序无关。服务目录改由 `Ctx` 持有（`OnceCell`），引擎的服务查找与域判定共用一份。
- 单测 `seeds::services::tests::provider_lines_cover_super_chain`（虚构类名）。
- 结果：TestCharsetForName 429 / 1486 / 2451 → **437 / 1546 / 2622**（类 / 方法 / 上下文），`ExtendedCharsets`、`AbstractCharsetProvider`
  域 boundary → translate，其 9 个方法（含 `<init>(String)`、`charset`、`init`、`canonicalize`、`lookup`、两个 `<clinit>`）全部 bytecode；
  新增类为构造器里的 `TreeMap` 一族（`TreeMap` / `$Entry` / `$EntrySet` / `$EntryIterator` / `$PrivateEntryIterator`、`NavigableMap`、`SortedMap`）
  与 `lookup` 的 `Class$1`。其余 8 例类 / 方法 / 上下文完全不变；9 例漏均为 0；反射缺口不变（1 条）。
  顺序矩阵（`--flow-batch 1/64` × `--hash-seed 0/12345`）：TestCharsetForName、DeepCopy 类集 / 方法集（含 kind 与 cut 标记）四组一致。
- 已知局限：`lookup` 对 map 值做 `Class.forName(className).newInstance()`；本例查的是不存在的字符集名，走不到。若测试查扩展字符集
  （如按别名取 jdk.charsets 里的编码），被取的类需要按名取类的值集覆盖（`charset(name, className, aliases)` 的常量实参进 map 值），届时实测。

**dyn_compare 为何没报出，及补强（方法粒度对照）**。类粒度对照在本例结构性失明，原因有二：一是 `ExtendedCharsets` / `AbstractCharsetProvider`
已在闭包内（alloc / init 级），「闭包内类上漏掉的方法」按类对照看不见（前文已记）；二是它们属边界域，程序期加载的类与经过边界帧的加载一律
归 `boundary` / `boundary-code`，前提是「边界方法由手写层承载，运行期不执行其字节码」——而对无手写承载的边界方法这个前提不成立（发射层翻译其
字节码）。实测 1b87995d 闭包的对照，程序期没有任何加载事件的调用栈含 `sun/nio/cs/ext` 帧（`TreeMap` 等早在 main 之前已加载）。补强：
- 分析器在 closure.json 的方法条目上附加 `"cut": true`（`Ctx::boundary_cut`）：内部边界类上无手写承载（非 native、有体、非 `<clinit>`、
  非 VM 内建、无共置手写体按精确名提供、无伴生核心 `core_<名>`）的方法——发射层翻译其字节码、分析器不展开其体。附加字段，现有消费方忽略。
- `scripts/dyn_agent/load_trace.c` 新增 `methods=<主类>` 模式：开 MethodEntry 事件，按 jmethodID 去重记录主类 main 首次进入之后每个方法的
  首次进入及其调用方帧（`M` 行）。`dyn_compare.py --methods` 逐条对照：调用方是翻译体（bytecode 或 cut）而被调方不在闭包 → `mmiss`
  （indy 模型调用点、`vm_upcall_classes`、隐藏帧除外；调用方为手写 / native / 不在闭包的不可比）；结果行附 `mmiss N cut K`。
  单测 `tests/unit/test_dyn_compare.py::MethodCompareTest`（虚构类名）。
- 验证：修复前的闭包（同一代码去掉 provider 执行线）对照，TestCharsetForName 报出 `cut` 调用方的 mmiss 6 条，首条即
  `AbstractCharsetProvider.<init>:(Ljava/lang/String;)V ← ExtendedCharsets.<init>@3（边界截断体）`，其后 `charset` / `init` / `canonicalize` / `lookup`；
  修复后只剩 `BuiltinClassLoader.findResources ← BootLoader.findResources@4`。
- 9 例 mmiss（修复后）：HelloWorld 7、FileIOTest 6、TestStreamBasic 11、CollectorsDemo 20、Digester 28、MH Combinators 37、MH Direct 47、
  DeepCopy 81、TestCharsetForName 39。字节码调用方的条目多为 JVM 与原生运行期执行模型不同：VM 自建对象上的虚派发（`ConcurrentHashMap.get`
  对 `StrongReferenceKey` / `MemberName` 取 hashCode）、手写层替换的子系统（反射访问器工厂、类加载器、`AccessController.executePrivileged`
  的 action、`InternalLock`）。cut 调用方条目：MH / DeepCopy 的 `jdk/internal/org/objectweb/asm` 一族（JVM 编译 LambdaForm / 生成类，原生
  由运行模型替换）、Digester 的 `jdk/internal/event/Event.<init>`、MH Direct 的 `Unsafe.bool2byte` / `compareAndSetByte`、
  TestCharsetForName / DeepCopy 的 `BootLoader.findResources` → `BuiltinClassLoader.findResources`（JVM 平台加载器有类路径；原生
  `BootLoader.hasClassPath` 为手写，`LazyClassPathLookupIterator` 走空枚举）。因此 `mmiss` 暂作诊断输出、不作门槛；cut 调用方的条目
  是高信号子集，e2e 命中存根时先查它。耗时：methods 模式 HelloWorld + TestCharsetForName 两例（含闭包分析）合计 3.3 s。

**TestStreamEncoderCharsets：跨写入拆开的代理对输出 U+FFFD（归属：Rust 生成器，非闭包精度）**。主干 c7a9d7b4 同样失败（协调方对照）。
闭包侧排查：`StreamEncoder` 的写入 / 关闭全部为手写（`stream_encoder_impl.rs::encode_units` 按 `haveLeftoverChar` / `leftoverChar` 跨写入
配对），翻译体 `CharsetEncoder.encode` 的折叠（`dead_pcs [8,9]`、`replacement` null）不在本例执行路径上。决定性证据是 UTF-8 行：手写层对
孤立代理项输出 `?`，而实测输出 `ef bf bd`（U+FFFD 的正常编码），说明进入 `StreamEncoder` 的码元已经是 U+FFFD——字面量 `"a\uD83D"` 在
生成器里被替换。根因：`classfile::reader::decode_mutf8` 解码到 Rust `String`，孤立代理项按 `from_utf16_lossy` 变成 U+FFFD
（`instr/GOLDEN_DIFF.md` 已知差异 1；Python 侧保值发射 UTF-16 码元数组，故 Python 基线通过）。修复：
- `decode_mutf8` 在含孤立代理项时另携无损码元（`CpEntry::Utf8(text, Option<units>)`），常量池字符串常量产出 `Const::StringUtf16(units)`；
  文本侧（成员名 / 类名匹配）不变。
- 生成器：ldc 发射 `Lit::JStringUtf16`（`String::from_utf16_lit`，驻留语义同 `String::from`）；ConstantValue 字段常量同形。
- 闭包分析：`StringUtf16` 按非空 String 站点值入栈（不进常量格），ldc 事件照常实例化 String。
- 单测：`classfile reader::mutf8_tests::lone_surrogate_keeps_units`、`instr sim::consts::tests::lone_surrogate_string_keeps_units`、
  `emit class_writer::fields::tests::names_and_literals`。单例验证 `scripts/main.py` 输出三行 split 与 JVM 一致。
- ~~已知局限：字符串拼接与注解字符串常量按 Rust 文本~~ → 已在 UTF-16 层构造（2026-10-01，bc7eb54b）：
  - 拼接 IR 改为 `Lit::JStringConcat(Vec<ConcatPart>)`（`Text` / `Units` / `Arg`），渲染 `(String::of("首段") + &s + c + n)`，运行时
    `String` 的 `Add<&str | &[u16] | &String | u16 | bool | i8..i64 | f32 | f64>` 逐段追加码元（`string_ext.rs`）。配方模板与常量位值按码元
    切分（含 `StringUtf16`），`char` 实参以 `u16` 码元追加，String 实参按引用追加码元（null → `"null"`），不再经 `format!` / Display。
  - 注解稀疏常量池：含孤立代理项的 Utf8 → `AnnoConst::Utf16` → `idx:W:<码元 hex>` → `CpVal::W`，`getUTF8At0` 以 `from_utf16_lit` 构造。
  - 单测：`instr concat::tests::{recipe_keeps_lone_surrogates, recipe_text_segments_and_arity, prim_operands_are_typed}`、
    `ir render::lit::tests::concat_parts`、`classfile extras::tests::anno_const_keeps_lone_surrogate_units`、`emit attrs::tests::anno_cpool_strings`。
  - e2e：`17_string_advanced/TestConcatLoneSurrogate`（配方常量段 / 常量位值 / char 实参 / String 实参 / 半代理拼成代理对 / 注解字符串常量，
    期望输出由 JDK 21 生成），`scripts/main.py` 单例 11 行与 JVM 逐字一致。

**TestNetworkInterface：命中 `UnixNativeDispatcher.openatSupported:()Z` 存根（归属：边界截断 + 手写层缺口，非精度改动；主干同样失败）**。
`dyn_compare --methods` 的 cut 条目直接给出链：`SHA1PRNG` → `SeedGenerator$1.run`（JCA 放行，翻译体）列举临时目录 →
`Files.newDirectoryStream` → `UnixFileSystemProvider.newDirectoryStream`（`sun/` 边界类，无手写承载 → `cut`：发射层翻译其体，分析器不展开）
→ 体内的 `toUnixPath` / `checkRead` / `openatSupported` / `opendir` / `UnixDirectoryStream.<init>` 5 条 mmiss 均标 `caller_cut`；
其后 `UnixDirectoryStream.iterator` / `hasNext` / `next` / `close` 由翻译体 `SeedGenerator$1.run` 调用，同样不在闭包（返回值来自截断体，类型集为空）。
- 方案 A（分析器把全部边界截断体按字节码展开，与发射层一致）实测不可行：HelloWorld 605 → 19292 方法、3226 类，DeepCopy 9669 → 22614
  （截断体 `ClassRepository.make` 等经 `sun/reflect/generics` 展开全族），已撤回。
- 方案 B（清单放行 `sun/nio/fs/`，按字节码建模该执行线）实测增量小：TestFilesApi 866 → 889 方法、FileIODemo 935 → 957、
  TestFileAccessSpace 6939 → 7099、TestNetworkInterface 6820 → 6989，HelloWorld / FileIOTest 不变；`openatSupported` / `opendir` /
  `readdir` / `UnixDirectoryStream` 族入闭包。但放行后 `UnixNativeDispatcher.<clinit>` 转为翻译体，需要的 native 在手写层缺 6 个
  （`close0` / `closedir` / `dup` / `fdopendir` / `opendir0` / `readdir0`），且现有手写 `init(int[])` 与 JDK 21 的 `init()I` 签名不符——
  属 POSIX 原生族档 B 的手写层工作，放行须与这批 native 同批落地并跑 `TestFilesApi` / `TestFileAccessSpace` / `FileIODemo` 回归。未提交，待排期。
- 边界截断体（`cut`）整体仍是分析与发射不一致的来源：各例 cut 数 HelloWorld 3、FileIOTest 4、CollectorsDemo 34、Digester 62、
  MH 59、DeepCopy 106、TestNetworkInterface 78。终态随 `[boundary]` 前缀清零消解；过渡期 e2e 命中存根先查 `mmiss … cut`。

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

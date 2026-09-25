# jnc / ruva Java 测试语料导入 e2e 方案

> 日期：2026-09-25
> 来源：`~/dev/workspace/jnc/tests`（java-sources + expected-output）、`~/dev/workspace/ruva/tests/e2e`
> 去向：`tests/e2e/<NN_category>/` + `tests/expected/<类名>.txt`
> 定位：实施计划。经全量单文件编译审计后定稿，所有数字为实测。
>
> **范围变更（2026-09-25 执行时用户裁定）**：本次仅执行 §四-§五（文件导入 + 内容修正）；
> §六（期望输出生成与确定性闸门）、§七中的 e2e 跑批基线不在范围内——期望输出文件一并不生成
> （导入测试在补齐期望前会被 run_tests.py 以 SKIP 处理），§六.4 的 run_tests.py cwd 修正暂缓。
> 另：`features/InstanceofPattern/Main.java` 核对内容后定名 `InstanceofCastDemo`（内容为经典
> instanceof+cast，非 JDK16 模式匹配），落 `21_casting` 而非 `41_patterns_advanced`。

---

## 一、目标

把两个姊妹项目的 Java 测试语料并入本项目 e2e 语料库，终态：

- 每个导入测试满足框架契约：**单文件自含、public 类名 = 文件名、有 `main`、无 stdin/外部进程/机器相关输出、期望输出由项目钉住的 JDK 生成且跨次运行稳定**。
- 全部落位到既有编号类目目录（语料原目录不保留）；期望输出统一进 `tests/expected/`。
- 导入后跑不通的测试不是包袱而是**覆盖缺口清单**——进 `--skip-failed` 基线，作为后续修复的选材池。

## 二、框架约束（方案的推导起点）

`scripts/run_tests.py` 的约定决定了导入形态：

| 约定 | 代码位置 | 含义 |
|---|---|---|
| 发现 `tests/e2e/**/*.java`，`_class_name` = 文件 stem | run_tests.py:132 | 文件名即测试键，public 类必须与文件名一致 |
| 期望输出 = `tests/expected/<类名>.txt`，缺失则 SKIP | run_tests.py:741,979 | 导入必须配齐期望，否则静默不跑 |
| 期望生成：`javac` 单文件编译 + `java -cp <classes> <类名>`，**cwd=ROOT** | run_tests.py:759-776 | 跨文件依赖/包声明/读 stdin 的测试无法生成期望；写相对路径文件的测试会污染仓库根 |
| 每测试独立 scratch，转译后 `cargo run --bin <class>` | 文档头注释 | 多文件工程不可行，一切必须单文件化 |

## 三、审计结果（实测，2026-09-25）

对 825 个候选文件（jnc 120 + ruva 706 − Greeter.java 无 main）逐一做了隔离目录 `javac` 单文件编译：

| 指标 | 数值 |
|---|---|
| 单文件可编译 | **823 / 825（99.76%）** |
| 编译失败 | 2：`jnc/ComprehensiveTestV1.java`（public 类名与文件名不符）、`ruva MultiFileDemo.java`（引用独立文件 Greeter） |
| 类名跨库撞名 | 3：`HelloWorld`（jnc/ruva 各一份，本项目已有同名测试）、`InterfaceTest`、`StreamTest`（jnc 与 ruva 各一份，内容不同） |
| `package` 声明 | 22 个（ruva：algorithms 9、features 10、collections 1、string 2，均为 `com.github.wesleyegberto.*`，文件本身自含可独立编译） |
| jnc 自带期望输出 | 121 份（`expected-output/`）；`bootstrap/` 3 个类无 main 无期望（运行时 bootstrap 实验类，非测试） |
| 非确定性/环境敏感命中 | 线程并发 19、时间戳 4、`Math.random` 2、UUID 1、stdin 2、外部进程 1、`System.getProperty` 2、文件 IO 9 |

## 四、逐文件处置清单

### 4.1 剔除（9 + 3，剔除理由均为违反框架契约，非质量问题）

| 文件 | 理由 |
|---|---|
| jnc `HelloWorld.java`、ruva `basic/HelloWorld.java` | 与既有 `01_basics/HelloWorld` 同名；内容均为一行 "Hello, World!" 打印，语义已被完全覆盖 |
| jnc `ComprehensiveTestV1.java` | 与 `ComprehensiveTest.java` **逐字节相同**（diff 为空），纯重复 |
| jnc `bootstrap/`（Wrapper、SimpleNode、SimpleContainer） | 无 main 的支持类（Optional 式包装实验），非独立测试 |
| ruva `io/ScannerBasic.java` | `Scanner(System.in).nextInt()` 读 stdin，headless 必 EOF 失败 |
| ruva `io/ProcessBuilderDemo.java` | 派生外部进程且输出含 `$HOME`，机器相关 |
| ruva `misc/UUIDDemo.java` | `UUID.randomUUID` 输出不可复现 |
| ruva `algorithms/KnightsTour.java`、`BalancedBrackets.java` | `Math.random` 驱动输出 |
| ruva `algorithms/VersCheck.java` | 打印 `java.version`，JDK 版本相关（本项目双 JDK 交替） |
| ruva `algorithms/ScriptName.java` | 读 `sun.java.command`，调 `java` 与调 `cargo run` 输出必然不同 |

保留观察（不由人工预判，交确定性闸门裁决，见 §6）：线程/并发 19 个、时间戳类 4 个（MianChowlaSequence、PartitionFunction、PrimeTriangle、RamanujanPrimes）——多数只在计时不打印时间，预计大部分可通过。

### 4.2 内容修正（导入时一次性完成，修正后单文件编译必须通过）

| 文件 | 修正 |
|---|---|
| 22 个带 `package` 的文件 | 删除 `package` 行（文件自含，已验证删除后无编译影响） |
| ruva `basic/MultiFileDemo/MultiFileDemo.java` | 把 `Greeter.java` 并入同文件作为非 public 第二类，单文件化 |
| ruva `features/InterfaceTest.java` | package 剥离 + 类/文件改名 `InterfaceDefaultChainTest`（撞 jnc `InterfaceTest`；内容为 A→B→C default/static 方法继承链） |
| ruva `features/StreamTest.java` | package 剥离 + 类/文件改名 `StreamOfNullableFilteringTest`（撞 jnc `StreamTest`；内容为 `Stream.ofNullable` + `Collectors.filtering`） |
| ruva `features/InstanceofPattern/Main.java` | 类名 `Main` 语义过弱，按内容改为 `InstanceofPatternDemo`（执行时核对内容后定名） |

改名共 3 个，新名在既有 179 + 导入语料内均无冲突。jnc 侧保持原名不动。

## 五、目录映射

原则：**落位到既有编号类目，语料原目录不保留**；`-Advanced` 变体进对应 advanced 目录；无法归类者进新目录 `58_misc`（编号续接 57）。导入脚本产出 `build/import-manifest.tsv`（源路径 → 目标路径 → 处置动作），拷贝前人工过目一遍。

### ruva（目录 → 类目，692 个导入）

| ruva 目录 | 数量 | 去向 |
|---|---|---|
| algorithms | 528 | `23_algorithms`（剔除 4：KnightsTour、BalancedBrackets、VersCheck、ScriptName） |
| collections | 29 | `04_collections`（其中 ExceptionHierarchy、ExceptionPropagation 按名改投 `06_exceptions`） |
| features | 29 | 按名分投（Base64/UrlEncoder→`53_io_api`，Shadowing→`52_lang_features`，Diamond/InterfaceDefaultChain→`26_interface_advanced`，PipeStream/RecordsSerialization→`35_io`，SimpleReductions/ExampleConsumers→`14_functional`…），余量→`58_misc` |
| misc | 38 | 按名分投（Enum→`09_enum`、Switch→`15_switch`、Pattern→`41_patterns_advanced`、StringBuilder→`17_string_advanced`、Nested→`11_inner_classes`、WeakRef→`48_refs`、Big→`36_bignum`、Strictfp→`52_lang_features`…），余量→`58_misc` |
| basic | 9 | `01_basics`（剔除 HelloWorld；MultiFileDemo+Greeter 合并计 1） |
| exceptions / string / oop / inheritance | 10/10/8/8 | `06_exceptions` / `05_strings` / `02_oop` / `02_oop` |
| io | 5 | `35_io`（剔除 2） |
| generics / lambda / streams / time / stdlib | 5/5/3/1/6 | `03_generics` / `07_lambdas` / `30_streams` / `37_datetime` / 按名分投（BigDecimal/BigInteger→`36_bignum`，Regex→`27_string_regex`，Atomic→`34_concurrency`，JavaTime→`37_datetime`，RandomLCG→`56_random_format`） |

### jnc（118 个导入，关键词规则 + 人工余量）

规则序匹配文件名，首轮分桶：collections 9、strings 9、exceptions 10、lambdas 10、maps 6、interface/poly 7、switch 5、io 4、algorithms 4、其余小桶若干；余量 29 个人工归类（ControlFlow/ForEach/EdgeCase/Comprehensive→`01_basics`，Regex→`27_string_regex`，StaticField/StaticInit→`10_static`，Checkcast→`21_casting`，DivByZero→`40_numeric_edge`，SetOperations→`44_collection_api`，Package→`52_lang_features`，JavaBase* 家族按各自主题，java_base_e2e_reflect_invoke→反射相关目录）。完整映射以 manifest 为准。

## 六、期望输出：统一再生成 + 确定性闸门

**不直接采信 jnc 的 121 份期望**（生成环境/JDK 未知），全量用本项目 JDK（`.jdk-version` 钉住的 21）重新生成，单一基准源：

1. 拷贝导入 → `python3 scripts/run_tests.py --update-expected`（约 810 个 javac+java，并行数分钟）。
2. **确定性闸门**：生成后快照 `tests/expected/` 增量文件 → 再跑一次 `--update-expected` → diff。两次不一致者出列（预期命中线程/时间类中的少数），出列者逐个人工裁定：改写为确定性或剔除，不带病入库。
3. jnc 原始 121 份期望与本轮再生成结果 diff 一遍：不一致处记录（JDK 版本漂移或 jnc 运行时缺陷线索），仅作情报不阻塞。
4. 文件 IO 类 9 个测试会在期望生成时于 cwd 写文件——先给 `_update_expected` 的 `java` 子进程把 cwd 从 `ROOT` 换成每测试 scratch 目录（顺手修掉现状污染仓库根的隐患，对既有 179 测试无影响：现语料无读预置文件的用例），生成后核对仓库根无残留。

## 七、执行步骤（三批提交，失败可定位）

| 批次 | 内容 | 规模 | 说明 |
|---|---|---|---|
| B1 | jnc 118 + 全部内容修正机制跑通（package 剥离、改名、合并的样例都在 B1 前验证） | ~118 | 小批先验证管线；含 jnc 自带期望的交叉 diff |
| B2 | ruva 除 algorithms 外全部 | ~164 | features/misc 按名分投在这里定型 |
| B3 | ruva algorithms 528 → `23_algorithms` | 528 | 纯增量，`23_algorithms` 2 → ~530 |

每批流程：manifest 生成过目 → 拷贝+修正 → `--update-expected` → 确定性闸门 → `run_tests.py --filter <新目录>` 全跑 → 转译失败入 `failed_tests.txt` 基线（`--skip-failed` 后续迭代用）→ 提交推送。

## 八、验证与风险

- **管线验证**：B1 先行，全流程走通再放量。
- **终态核对**：`find tests/e2e -name '*.java' | wc -l` ≈ 179 + 810 − 后续闸门出列；`tests/expected/` 同步增长；仓库根无生成残留；`.class`/`.DS_Store` 不入库。
- **风险 1 — 转译失败放量**：810 个新测试预计大部分首跑失败（覆盖缺口本就是导入目的）。基线入 `failed_tests.txt`，不阻塞合入；后续按 `--failed` 定向消化。
- **风险 2 — e2e 总时长上涨**：每测试独立 cargo 编译，全量时长显著增加；共享 target 缓存 + `CARGO_INCREMENTAL=0` 已就位，服务器 OOM 防线沿用既有脚本。
- **风险 3 — 跨机可复现**：期望由 JDK21 生成，双 JDK 交替下若个别测试输出随 JDK 漂移，与既有语料同策略（真漂移的按 JDK 分层或修正测试），确定性闸门只保同机同 JDK 稳定。
- **风险 4 — 多类单文件转译**：合并后的 MultiFileDemo 与 jnc 语料内含内联 interface/第二类，B1 用 ComprehensiveTest（多类）做转译金丝雀，确认转译器对多 top-level 类文件的行为后再放量。

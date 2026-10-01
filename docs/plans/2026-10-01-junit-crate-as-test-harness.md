# JUnit 依赖包作为测试框架：现状盘点与继续路线

> 日期：2026-10-01
> 来源：用户命题「怎么用 JUnit 的依赖包作为测试？之前有相关的测试，但是没有做完，分析一下要怎么继续做」。
> 定位：现状盘点 + 继续路线（A/B/C 三步，每步独立止损）。事实跟踪归 [tasks.md](../../docs/tasks.md)；本文是对 [2026-09-23-junit-crate-pilot](2026-09-23-junit-crate-pilot.md) 的续篇——那个计划回答「JUnit 怎么成 crate」，本文回答「crate 怎么被用来测试」。

---

## 一、现状：试点已交付什么（M1–M5 全部 GOLDEN OK，2026-09-23~25）

| 里程碑 | 交付 | 验证 |
|---|---|---|
| M1 | hamcrest crate（CoreMatchers + TypeSafeMatcher 族，12 检查含失败路径消息逐字等价） | `lib_pilot_golden.sh m1` |
| M2 | junit4 crate 的 `org.junit.Assert` 全族（含 ComparisonFailure 消息格式） | `m2` |
| M3 | Runner 路径：注解元数据表跨 crate 扩表 + 反射 L3 分派 → `JUnitCore.runClasses` 跑通 `@Test` 发现/实例化/调用 | `m3` |
| M4 | `@Test(timeout=)`：FailOnTimeout（ThreadGroup + 模拟线程 + FutureTask.get 限时）；边界 = 真实超时不可达（已入 compatibility.md） | `m4` |
| M5 | workspace 一键构建（java_runtime + java_meta + hamcrest + junit4 + user），跨 crate 分派链（user 实现 lib 类型被 lib 回调，三 crate 往返） | `m5` |

> **注意**：上表 GOLDEN OK 全部是 **Python 生成器路径**验证的；Rust 路径（`rava build --lib`）未复跑——复跑即步骤 0，是 A 的入场券。

机制事实（2026-10-01 核实，含 Python→Rust 迁移状态）：

- 入口：`rava build --lib NAME=JAR[:seed=FQN[,FQN...]]`。Python 生成器已删除（fc801d83，2026-10-01 已进 main），`--lib` 只有 Rust 侧实现（0d650638，driver/src/build_libs.rs 与 emit/src/project/lib_crates.rs）。scripts/main.py 目前只是转发到 `rava build` 的薄层，将在 [scripts-into-rava](2026-10-01-scripts-into-rava.md) S5 删除。
- 发射：lib crate 为 scratch workspace 成员，pub 面 access_flags 驱动；**lib crate 对 JDK 类的引用定向到共享 `java_runtime`**（emit/src/project/lib_crates.rs 与 imports/cross.rs）——lib crate 只装自己的类，JDK 闭包不双份，发射内容在 jar+种子+生成器版本固定时确定（G-4 指纹已交付）。
- 消费：`--lib` 为单 bin 消费形态（拒 `--batch`）；run_tests.py 逐测试调转译入口恰是单文件模式，**形态上兼容，只是 harness 没接线**。
- 资产：junit-4.13.2.jar / hamcrest-3.0.jar 经 `fetch_pilot_deps.sh --no-scan` 导出到 `tests/lib_pilot/deps/target/pilot-libs/`（gitignore，不入库；**本机实测当前缺失**，复跑前先取包），清单在 tests/lib_pilot/deps/pom.xml。

## 二、缺口：「用依赖包作为测试」为什么还没成立

试点与「测试框架」之间隔三层，全部实测核实：

1. **harness 没接线（字面缺口的主体）**。run_tests.py（1085 例 e2e 的 harness）没有任何 lib/junit/hamcrest 概念（grep 零命中）：JVM 侧 javac/java 不带 junit jar 的 `-cp`，转译侧不传 `--lib`。今天写一个 `import org.junit.Test` 的 e2e 测试，两侧都跑不起来。junit4/hamcrest crate 只被 tests/lib_pilot 的 5 个手写 main 经独立脚本消费，lib 模式不在 e2e 回归面内。
2. **依赖包不是「包」**。每次 `--lib` 都在测试 scratch 内现场发射 + 编译 junit4/hamcrest（lib 自有类约 458 个，JDK 引用走共享 java_runtime）。没有预构建、跨测试复用的 crate 缓存——语义上是「每测试现场生产」，不是「依赖消费」。因发射内容确定性（G-4），预构建指纹缓存在架构上是可达的，属工程问题不是架构问题。
3. **种子面停在最小集**。junit4 crate 种子 = JUnitCore + Assert + Test(+Before)（golden 脚本 m3/m5 行）。`@BeforeClass/@AfterClass/@After/@Ignore/@Test(expected=)/assertThrows/Assume` 均未验证；Parameterized/Enclosed/@Rule 族未评估。作为「被翻译测试消费的框架」，注解生命周期族是基本盘。

远期项（不属本轮「继续做」）：M5 终态（java-runtime-core 版本化 + 可 publish）挂重写 R9；licensing 口径在 publish 前过。

另：导入语料的 815 例 SKIP 问题已消化（2026-10-01 实测 1085 java / 1085 expected 已对齐），「用 JUnit 免生成期望输出」不再是从零主动机；JUnit 形态的价值在于**自断言测试不需要逐字 golden**——导入审计出列的线程/时间/非确定性类测试由此获得出路。

## 三、继续路线（0 → A → B → C，每步独立验收）

### 步骤 0：Rust 路径复跑试点 golden（前置；现在即可开工）

M1–M5 的 GOLDEN OK 全部是 Python 生成器验证的，Rust 路径（`rava build --lib`）未复跑；本机 jar 资产实测缺失。py-delete 已进 main，`lib_pilot_golden.sh` 现经 main.py 转发到 `rava build`，跑的就是 Rust 路径（S5 后改为直接调 `rava build`，语义不变）。此步与 C1d/C6、scripts-into-rava 均无依赖。

1. `scripts/fetch_pilot_deps.sh --no-scan` 取包（PILOT_LIBS 导出位不变，jar 不入库）。
2. `scripts/lib_pilot_golden.sh m1..m5` 全量复跑，输出与 tests/lib_pilot/golden/ 库存档逐字 diff——不一致项即 Rust 路径对 lib 模式的回归清单，逐项归类后清零。
3. 复跑全绿是步骤 A 接线的入场券；期间 run_tests.py / dyn_compare.py 不动。

### 步骤 A：JUnit 测试形态接入 e2e harness（本机可开工，纯增量工程）

**形态裁决（独立脚本 vs 融入 run_tests.py——议定：融入，不建第二 runner）**：

- 「逻辑不一致」的实况：差异不在测试语义，在 harness 输入旋钮。e2e 契约 = 单文件自含、public 类名=文件名、有 main、确定性输出、expected diff——JUnit 形态测试全部满足（JunitRunnerMain 本身就是单文件有 main 输出确定的程序）；它只是多了一组**固定统一**的依赖（全 flavor 同一组 jar + 同一固定种子集，非每测试自定义依赖解析）。「目录即形态」恰好表达这个。
- run_tests.py 每测试管线形态中立：discover → 转译（`rava build`，含 javac）→ dyn 对照 → cargo run → diff expected → 台账（failed_tests 棘轮 / --record-passed / --batch / 内存纪律 / 产物清理 / jdk_select / 跨机一致性）。JUnit 形态只有三个接线点，其余全部继承：① 转译调用追加 `--lib`（`scripts/e2e/rava.py` 拼 `rava build` 参数时追加；转译内部本就为 lib 模式给 javac 上 classpath）；② 期望输出生成的 javac/java 加 `-cp` jar（`scripts/e2e/expected.py`；run_tests 拆为 `scripts/e2e/` 后是 scripts-into-rava 终态下**保留的 Python 编排层**，此改动持久）；③ dyn 对照的 JVM 运行加 `-cp` jar——**落在 rava 侧 dyn 判定实现上**（scripts-into-rava 把 dyn_compare.py 的判定逻辑并入 rava，不在 Python 侧投改以免做抛弃式工作），并顺带验证 lib 域类在域规则的归类。
- 排期：步骤 0 现在即可做；接线（①②③）依赖 scripts-into-rava 的 S6（dyn 判定并入 rava）与 S7（run_tests 拆分、预留形态接口），**不必等 C1d/C6**；受 C1d 制约的只有成本测量与缓存决策（见下第 4 点）。
- 独立脚本的代价 = 第二套 runner：期望管理 / 失败基线 / 批次 / 内存纪律 / 产物清理 / JDK 选择 / 跨机确定性全要重造且永远追不上主 harness 演进；两个台账 = 状态分叉 + 陈旧文件类问题。lib_pilot_golden.sh 即前车之鉴——它是刻意的单件（无台账无基线），只该做 crate 验收复现，不该往上面长测试。
- 职责切分因此清晰：lib_pilot_golden.sh m1–m5 = **crate 验收**（crate 自身忠实性，golden 文本入库存档，保持不动）；run_tests.py `63_junit` = **语料回归**（crate 作为依赖的测试进 1085 例台账）。将来 P1 试点（commons-lang3）是独立的对账 harness（测量仪器，非回归套件），同样不冲突。
- 唯一需单独权衡的是成本：JUnit 形态 scratch 多编译 junit4+hamcrest 两 crate（约 458 类，JDK 引用共享 java_runtime 不双份）。flavor 目录 opt-in，既有 62 类目零影响；先实测一条的增量再决定是否上预构建缓存。

**终态目标**：run_tests.py 原生支持 JUnit 消费形态——测试文件 import org.junit，JVM 侧真 jar 跑、翻译侧 crate 跑，结果逐字对账；新形态测试数从 0 到一批（首判 10–20 条）。

1. **形态标记**：新目录 `tests/e2e/63_junit/`，目录下放 `form.toml` 即为特殊形态（scripts-into-rava §六）。jar 名与固定种子集作为数据写在 form.toml 里（框架基本盘，非每测试自定义，保证闭包稳定可缓存），Python 中不写库类名；`scripts/e2e/select.py` 的 `form_of` 读取。
2. **harness 接线**（三个接线点）：
   - 转译侧：`scripts/e2e/rava.py` 追加 `form.lib_args`，即 `--lib hamcrest=... --lib junit4=...:seed=<form.toml 种子集>`。
   - 期望输出：`scripts/e2e/expected.py` 的 javac/java 加 `-cp <classes>:<jars>`。
   - dyn 对照：rava `--dyn-compare` 给 JVM 加 jar classpath，并把 lib 类归入 lib 域。
   - jar 目录由 run_tests `--pilot-libs DIR` 指定（缺省 `tests/lib_pilot/deps/target/pilot-libs`，不新增环境变量）；缺失时自动取包，取不到即失败，不静默跳过。
3. **输出契约不变**：JUnit 形态测试的 main = `JUnitCore.runClasses(...)` 打印确定性摘要（Tests run/Failures 计数 + Failure 消息，无栈迹无身份哈希——试点 m3 已验证此形态逐字可比），期望文件照旧走 tests/expected，框架契约零改动。
4. **成本闸门（时点受 C1d 制约）**：测量须在 C1d 边界收窄（闭包回落至 ≤250 类量级）落地后做——收窄前膨胀的 java_runtime 会掩盖 lib crate 的真实增量。测量口径：一条 JUnit 形态测试的 per-test 构建时长/磁盘增量（junit4+hamcrest 约 458 类编译叠加；JDK 引用共享 java_runtime，预期中等增量而非翻倍）。
5. **预构建缓存（终态目标，一次写死）**：「依赖包就是包」——终态 = 预构建 lib crate 落 `build/libcrates/<jar指纹+种子表+生成器版本>/`，scratch 以 path 依赖外引（workspace members 改可选）。**依赖链必须写明**：lib crate 的 rlib 链接的是 per-test 裁剪的 java_runtime，跨测试复用 rlib 的前提是版本化 java-runtime-core（= pilot 计划条件③终态，R9 域）；此前可达的近期增量是指纹键控的**生成源缓存**（省转译、不省 rustc）。A 内是否先落源缓存，由第 4 点测量裁决。
6. **首批用例**：双来源——(a) Assert/Matcher 断言面扩展（m1/m2 用例风格搬进 e2e 形态）；(b) 导入审计出列的非确定性测试改写为 JUnit 断言形态（线程 join 限时、时间戳容差、随机种子固定后断言），变「输出对账」为「断言对账」。
7. **验收**：首批全绿 + `lib_pilot_golden.sh m1..m5` 零回归 + 既有 e2e 抽样（≥10 例）零回归 + `python3 -m compileall -q scripts`（run_tests 侧）与 `cargo check`（rava 接线侧）。

### 步骤 B：junit4 crate 种子面补全（随 A 的用例需求拉动）

- 种子扩到 `@BeforeClass/@AfterClass/@After/@Ignore/@Test(expected=)/assertThrows/Assume`；每扩一批以 m2/m3 golden 扩展用例对账一次。
- 显式不进种子的（compatibility.md 挂账，避免半支持假象）：Parameterized/Enclosed Runner 变体、@Rule 规则族（运行时代理倾向，压静态翻译模型死穴）。

### 步骤 C：P1 真实试点——commons-lang3（路线图既定阶梯的兑现）

- commons-lang3 jar `--lib` 输入 + 其自带 JUnit 测试套件选子集翻译；同一套测试两侧跑，结果计数 diff = 0 或逐项归类。
- 产出 = 「新错误族频率」的实测标定（R0 信号 c）+ P0 地基价值（「每个 jar pilot 的验证 harness 就是它自带的 JUnit 测试套件」）的兑现。启动时机挂 roadmap §四-3 决策点 3（语料扩张形态），**属拍板项**。

### 优先级与边界

- 步骤 0 现在即可做；A 的接线排在 scripts-into-rava S6/S7 之后，直接落在终态架构（`rava build --lib` + rava 侧 dyn 判定 + `scripts/e2e/` 形态接口），不对将删的 Python 面投入；
- B 跟随 A 的用例需求，不独立排期；
- C 需用户拍板后另立任务书，时点 = A 首批全绿之后；
- 不动：build.rs / java_meta 元数据表语义（A/B 只消费 M3 已扩的跨 crate 表）、runtime/rava_macros、var_handle*。

## 四、开放问题（裁定状态）

1. ~~jar 资产策略~~ → **已裁定：不入库**，缺失时自动取包，取不到即失败。
2. ~~形态标记用目录还是文件头注释~~ → **已议定：目录即形态**（`tests/e2e/63_junit/`），独立脚本 vs 融入的取舍一并议定为融入，理由见步骤 A「形态裁决」。
3. ~~预构建缓存是否在 A 一并做~~ → **已裁定：预构建 lib crate 写为 A 的终态目标**（跨测试 rlib 复用依赖 java-runtime-core 版本化，见步骤 A 第 5 点）；A 内是否先落生成源缓存，由成本测量（C1d 后）裁决。
4. 开工时点：步骤 0 = 现在；接线 = scripts-into-rava S6/S7 之后；成本测量与缓存决策 = C1d 落地后；C 试点拍板 = A 首批全绿后。

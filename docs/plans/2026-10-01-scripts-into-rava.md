# Python 脚本并入 rava：产品路径 Python 归零

> 状态：方案已认可（2026-10-01），S1–S4 实施中，S5 起按 §4.1 闸门等协调者通知。第一步「Python 生成器删除」已完成（fc801d83，main 1ca2ab76），
> 本文 B 类随之结清；本次修订刷新现状、细化 A 类为可逐步提交的步骤，给出与 c1d-prec / c4-regfix 的
> 合并顺序，并为 JUnit 形态（[`2026-10-01-junit-crate-as-test-harness.md`](2026-10-01-junit-crate-as-test-harness.md)）
> 在 e2e 编排中预留 `--lib` 接口。

## 一、终态

- **产品路径 Python 代码为 0**：`.java` / `.class` → 可运行二进制的全过程（JDK 定位、javac、闭包、生成、
  cargo 编译与运行、动态对照）只经 `rava` 单二进制。`scripts/main.py` 删除，不保留转发层。
- 与转译结果相关的判定逻辑（JDK 解析、重型编译判定、动态对照与域规则、缺口审计、crate 划分）只在 rava 内
  有一份实现；Python 侧重复实现为 0。
- 保留的 Python 仅限**测试编排与语料运维**（跑批、结果对照、语料导入、编译测量），只调用 rava 的公开 CLI，
  不复制 rava 的任何判定规则，不 import 任何产品路径模块。
- 本项目不新增自有环境变量，新增能力一律为 CLI 选项；`JAVA_HOME`、`CARGO_BUILD_JOBS`、`CC` 等标准变量照常尊重。

## 二、现状（main 1ca2ab76）

| 脚本 | 行数 | 职责 | 调用方 | 与 rava 的重复 / 终态归属 |
|---|---:|---|---|---|
| main.py | 151 | 纯转发：`apply_jdk` → `rava build --no-run` → `cargo run --bin`（heavy jobs） | 用户、run_tests、gap_scan、gen_trees / seed_check / lib_pilot_golden.sh | rava build 自带 cargo run；main.py 只多了 JDK 选择与 heavy 判定 → **A1 删除** |
| rava_cli.py | 58 | 定位 / 构建 rava，拼 `rava build` 参数 | main.py、gap_scan | 随 main.py 与 gap_scan 一起 **删除** |
| cargo_env.py | 53 | 重型工作区（≥1700 类）单作业编译 | main.py、run_tests | `build_cmd.rs::HEAVY_CLASSES` 同值两份 → **A2 并入** |
| jdk_select.py | 170 | `--jdk` > JAVA_HOME > `.jdk-version` > 最新已装；brew 双前缀 / Linux / `java_home` 扫描 | main.py、run_tests、gap_scan、fetch_pilot_deps.sh、lib_pilot_golden.sh | `resolve::jdk::find_java_home`（64 行）优先级不同、扫描面更窄 → **A3 并入** |
| dyn_compare.py | 591 | JVMTI agent 跑真实 JVM，类 / 方法加载集对照闭包，provenance 归因（含 0c251b4b 的 `sigpoly_sites`） | run_tests | 读回 closure.json 重判；域规则从清单重解析 → **A4 并入** |
| dyn_agent/load_trace.c | — | JVMTI agent 源 | dyn_compare.py 运行期 cc 编译 | **A4 迁入** dyn crate |
| gap_scan.py | 194 | API / 语料两模式缺口扫描（`main.py --precheck-only` 产物 grep `panic!`） | 手工 | **A5 并入** `rava audit` |
| native_gap_scan.py | 77 | 逐例 `rava closure`，native / boundary 方法与手写 fn 名交叉比对 | 手工 | 复制了手写 fn 名规则 → **A5 并入** |
| dep_scc.py | 107 | 类依赖 SCC（拆 crate 依据） | 手工；crate-split 文档 §三「复现」 | S4 物理拆层已合入，划分在 emit 内 → **A6 删除** |
| run_tests.py | 1683 | e2e 编排：选例、并行、期望输出、转译、cargo build、动态对照、产物清理、台账、汇总 | 用户、run_bg.sh | 自调 `cargo build` + 解析 JSON 产物；import jdk_select / cargo_env / dyn_compare → **C 类拆分改接口** |
| baseline_diff.py | 175 | 全量结果与冻结基线对照 | 手工 | 保留（编排） |
| dep_scan.py | 229 | lib pilot 依赖透视 | fetch_pilot_deps.sh | 保留（语料运维） |
| import_e2e_corpus.py | 374 | 语料导入 | 手工 | 保留（语料运维） |
| mono_stats.py | 59 | `-Z dump-mono-stats` 归类 | 手工 | 保留（编译测量，不在产品路径） |

调用 main.py / jdk_select 的 shell：gen_trees.sh、seed_check.sh、lib_pilot_golden.sh（`main.py --lib`）、
fetch_pilot_deps.sh（`jdk_select.py` 取 JAVA_HOME）。文档：README.md、docs/environment-variables.md、
docs/compatibility.md、docs/tasks.md、CLAUDE.md（**CLAUDE.md 由用户改**，本计划只给改写稿）。

## 三、终态设计

### A1 入口：`rava build` 取代 main.py

- `rava build <Test.java>` 是唯一入口；main.py 的选项全部已在 build_opts（`--jdk` / `--out` / `--clean` /
  `--no-run` / `--batch` / `--debug` / `--strict` / `--trace-class` / `--raw-sites` / `--lib` / `--locale` /
  `--closure-json` / `--precheck-only`；c1d-prec 另加 `--cut` / `--cut-file` / `--dump-edges`）。
- 阶段选择统一为 `--stop-after <javac|closure|emit|compile|run>`（缺省 `run`），删除 `--no-run` /
  `--precheck-only` 两个布尔开关：`--no-run` ≡ `--stop-after emit`；`--precheck-only` 更名 `--full-precheck`
  （仅 `--stop-after emit` 下可用；预检明细全量打印，不出审计行；S2 已实施）。run_tests 用 `--stop-after compile`（运行与输出比对由编排做）。
- `--build-timeout SECS` 进 rava：超时终止整个 cargo 进程组，状态写入 build_status.json（A2）。
- 现 rava_cli 显式传的 `--runtime` / `--closure-cache` / `--java-home` 改为 rava 缺省派生：仓库根由可执行文件向上
  查找 `runtime/java_runtime/closure.toml` 得到，`--runtime` 缺省 `<repo>/runtime/java_runtime`、`--closure-cache` 缺省
  `<repo>/build/closure_cache`、JDK 走 A3。显式选项仍可覆盖。
- rava 二进制位置沿用 `build/analyzer-target/release/rava`。编排在**每批开头**执行一次
  `cargo build --release -p driver --target-dir build/analyzer-target`（新鲜时空操作，经 heavy_lock 口径），保证不用陈旧
  生成器；shell 脚本同样先构建再调用。逐例调用直接执行二进制，不再每例 `cargo run`。

### A2 cargo 编排：cargo_env.py 并入 `driver/src/cargo.rs`

- rava 内唯一的 cargo 调用点：`CARGO_TARGET_DIR=<repo>/build/target`、`CARGO_INCREMENTAL=0`、
  `--message-format=json-render-diagnostics`，`compile` 阶段 `cargo build --bin`，`run` 阶段直接执行产物。
- 重型判定按**最大单 crate 规模**（S4 拆层后 emit 已知每个 crate 的类数 / 方法体数），阈值是 cargo.rs 内
  一个常量；调用方显式设置 `CARGO_BUILD_JOBS` 时尊重。Python 侧阈值 0 份。
- `<scratch>/build_artifacts.json`：本工作区 manifest 下的 compiler-artifact / build-script-executed（现
  run_tests `_record_build_artifacts` 的归属规则原样搬入），另含可执行文件绝对路径。
- `<scratch>/build_status.json`：`{stage, ok, exit, signal, timeout, first_error, log, jdk:{home,major,source}, heavy:{crate,classes,jobs}}`；
  rustc 全文日志落 `<scratch>/logs/build.log`。run_tests 的 OOM / 超时 / 首错行提取全部删除，改读该文件。

### A3 JDK 解析：jdk_select.py 并入 `resolve::jdk`

- 优先级以 jdk_select 为终态口径：`--jdk N` > `--java-home` > 有效 `JAVA_HOME` > 仓库根 `.jdk-version` >
  已装最新版。`--jdk N` 无精确主版本时**报错**（不再静默取 ≥N，语料与 javac 同源的前提）。
- 扫描面：`HOMEBREW_PREFIX`、`/opt/homebrew`、`/usr/local` 的 Cellar（`openjdk@NN` 与裸 `openjdk`）、
  `/usr/lib/jvm`、`/usr/libexec/java_home -v N` 回退。选中结果按 jdk_select 现格式打印来源。
- 新子命令 `rava jdk [--jdk N] [--list] [--home-only]`：打印 home 与来源 / 列出已装版本 / 只打印 home
  （shell 脚本取 JAVA_HOME 用，取代 `python3 scripts/jdk_select.py N`）。编排取 `java` / `javac` 也经它。

### A4 动态对照：dyn_compare.py 并入 `rava build --dyn-compare`

- 新 crate `generator/crates/dyn`（`agent.rs` / `trace.rs`（xlog + agent 轨迹解析）/ `domain.rs` / `attribute.rs` /
  `methods.rs` / `report.rs`，每文件 ≤ 600 行）。`--dyn-compare` 与 `--stop-after` 正交：closure 阶段后用同一 JDK
  跑真实 JVM（`-Xshare:off` + 原生系统属性 `-D` + `-agentpath` + `-Xlog:class+load,class+init`），在**内存中的闭包结果**
  上对照与归因，不读回 closure.json。`--dyn-methods` 开方法粒度对照（诊断项）。
- 域规则直接用分析器已加载的 `RuntimeManifest`（closure.toml / vm_intrinsics.toml），Python 的 `DomainRules`
  重解析删除；模型调用点表（`indy_models` ∪ `sigpoly_sites`）取分析器内存结构。
- JVMTI agent 保留为 C：源移到 `generator/crates/dyn/agent/load_trace.c`，`include_str!` 嵌入 rava；**运行期**用
  `$CC`/cc 按 (源哈希, JDK home) 编译并缓存于 `build/dyn_agent/`（JDK 头文件依赖运行期选中的 JDK，不能放 build.rs），
  临时文件 + 原子改名保证并发安全（现 `ensure_agent` 语义）。
- `--lib` 输入时 JVM `-cp` = 用户 classes + 各 lib jar（顺序同 `--lib`），lib 自有类按「lib 翻译域」归类
  （JUnit 计划步骤 A 的接线点③在此一次解决，编排侧零接线）。
- 结果写 `<scratch>/dyn_compare.json`（字段与现 Python 结果字典一致），一行摘要（现 `summary_tag` 格式）打印到 stdout；
  run_tests 汇总只读 JSON（`print_summary` 留在编排 report 模块）。
- `tests/unit/test_dyn_compare.py` 的用例逐条迁为 dyn crate 单元测试后删除。

### A5 审计：gap_scan.py、native_gap_scan.py 并入 `rava audit`

- `rava audit api <package>... [--recursive]`、`rava audit corpus [--filter S]... [-j N]`、`rava audit native [--filter S]...`：
  缺口判定即发射层预检（`emit::precheck`）：闭包方法节点 ∩ 生成体为方法体存根者，不再 grep stdout，不复制手写 fn 名规则
  （native 模式 = 同一缺口集合按闭包种类 `handwritten:native` / `handwritten:boundary` 过滤）。存根文本统一经
  `emit::precheck::stub_call` 生成，预检按同一形态识别。
- 预检扫描点在第二阶段收尾之后、物理拆层之前（`write_project` 产出 `ProjectReport.precheck`；审计用只发射不落盘的
  `emit::project::scan_gaps`）。S3 实施时查出两处既有回归，均随本步修复：① 84fc5581 存根改调 `__stub(…)` 后预检正则
  仍只认 `panic!(…)`，`[precheck]` 恒为 0；② S4 物理拆层后声明层方法体已省略、体在 `java_body_k`，预检只扫
  `java_runtime` 文本，漏掉实现层的全部存根。
- 预检链事实不含 abstract 方法：abstract 方法无方法体，派发落到子类实现，其存根体不可达（边界类上的 abstract 方法由
  手写子类经 vtable 实现，闭包种类记为 `handwritten:boundary`，但不是缺口）。
- native 模式与旧 fn 名启发式的差异（Cal / AStar / Date 19 例子集实测）：旧 72 行，新 14 行；新集合 ⊂ 旧集合，
  共有行触达数全同。旧独有 58 行均为误报：56 行闭包种类为 `handwritten:boundary`，但发射层已按字节码翻译出方法体
  （如 `Policy.isSet`、`VM.initLevel`），旧规则只看手写层有无同名 fn；2 行为 abstract 方法
  （`LocaleProviderAdapter.getBreakIteratorProvider`、`PhantomCleanable.performCleanup`），按上条排除。
- 只认声明类自身文本里的存根：子类的继承转发副本以声明者签名作标签，声明者方法 `slot_stub`（槽未被派发）时副本体为
  存根，但声明者自身的方法体已翻译，不是缺口（如 `LinkedHashMap` 里的 `HashMap.compute`、`Stack` 里的 `Vector.clear`）。
- API 模式验收（`rava audit api java/lang java/util`，JDK 21）：已提交报告出自 09-29 Python 流水线（261 类 / 3630 入口，
  23 native-missing / 193 boundary-stub），新审计 267 类 / 3828 入口，35 / 0（排除继承副本前为 35 / 12）。旧报告对比：
  - 旧 193 个 boundary-stub 全部消失：134 个已按字节码翻译出方法体（C1d 边界收窄，如 `ClassLoader.<init>`、`Policy.isSet`），
    45 个仍为存根但已不在调用链上（Rust 闭包分析器精度，如 `InvokerBytecodeGenerator.<init>`），14 个所在类不再入闭包
    （如 `jdk/internal/module/Modules`）。
  - 旧 23 个 native-missing 中 18 个消失：14 个已有手写实现（09-29 之后的缺口批次，如 `ProcessImpl.forkAndExec`、
    `NetworkInterface.*`、`Class.getGenericSignature0`），3 个不在调用链上，1 个所在类不再入闭包。
  - 新增 30 个 native-missing 为真实缺口：闭包变化后入链的 native（`Module.*0`、`ClassLoader.defineClass1/2`、
    `Class.getDeclaredClasses0` 等）与无手写实现的 `Unsafe.{get,put}*Volatile`、`Unsafe.throwException`。
- 输出 `docs/reports/gap-scan-<模式>.md` 与 `native-gap-scan.md`，格式不变。逐例只做 javac + 闭包 + 内存发射，
  不写 scratch（javac 产物落 `build/audit/` 临时目录，用完即删）；corpus / native 逐例起 `rava audit test` 子进程并行。

### A6 crate 划分：dep_scc.py 删除

- S4 物理拆层已合入，划分在 emit 内；查看划分用 `rava build --stop-after emit --perf`（perf 报告列各 crate 类数 / 体规模，
  A2 的重型判定同源）。crate-split 文档 §三 的「复现」行改指向历史提交 + 该命令。
- **已完成（S4）**：`scripts/dep_scc.py` 删除；`Perf.crates`（`emit::perf::CrateStat`）在拆层后按
  java_runtime → lib → java_body_k → user 记各 crate 类数与生成文本字节（手写真源不计字节），
  java_runtime 行类数取 JDK 布局类数，与 `Heavy::decide(r.jdk_classes)` 同源；driver 的 `--perf` 输出末尾补一行
  重型判定（同一函数，阈值 `HEAVY_CLASSES`）。crate-split 文档复现行改指 `git show 4bae5479:scripts/dep_scc.py` + 该命令。

### C 类：e2e 编排拆为 `scripts/e2e/`

`scripts/run_tests.py` 保留为 ≤ 40 行入口（`from e2e.cli import main`，命令行与现在一致，run_bg.sh / 文档命令不变），
实现拆为：

| 模块 | 职责 | 估计行数 |
|---|---|---:|
| `e2e/cli.py` | 参数、全局配置、入口分派 | ~150 |
| `e2e/select.py` | 发现、`--filter`、`--batch K/N`、失败过滤、**形态解析（§六）** | ~150 |
| `e2e/rava.py` | rava 定位、`rava jdk`、拼 `rava build` 参数（含形态 `--lib`）、读 build_status / build_artifacts / dyn_compare | ~150 |
| `e2e/runner.py` | 顺序 / 并行两种调度，每例 rava build → 运行二进制 → 比对 | ~450 |
| `e2e/expected.py` | 期望输出生成（javac / java，形态 classpath）、读取与 diff | ~150 |
| `e2e/ledger.py` | failed / passed 棘轮、`--record-passed`、prune | ~250 |
| `e2e/artifacts.py` | 通过测试生成物清理（读 build_artifacts.json） | ~120 |
| `e2e/report.py` | 审计行解析与汇总（readability / equiv / fallback / raw / run 子族 / dyn 汇总 / deny） | ~450 |

删除对 jdk_select / cargo_env / dyn_compare 的 import；`"cargo"` 子进程调用为 0（`cargo --version` 环境记录除外）。

## 四、分步方案

### 4.1 合并门与冲突面

| 在途分支 | 与本计划的冲突面 | 处理 |
|---|---|---|
| c4-regfix | sigpoly_sites 归因（0c251b4b）**已在 main**；工作区未提交改动只在 closure crate（vmrules / rtfn / units 等），不碰 scripts / driver / resolve。在跑的「C4 基线回归 7 例」可能再动 dyn_compare 归因 | A4 前必须收口：其 dyn_compare 改动合入集成分支后才开 A4 |
| c1d-prec | **dyn_compare.py +52 行未合**：DomainRules 去 `boundary_packages` / `PUBLIC_API` 截断、放行改 `[vm_boundary] translate_nested`、方法粒度去 `cut`；test_dyn_compare.py 同步改；系统属性 `-D` 注入。**main.py**：`--cut` / `--cut-file` / `--dump-edges` 透传；**driver build_opts / closure_cmd / api_roots** 加同名选项。且该分支尚未合入 py-delete（仍有 generator_select / golden 改动） | A1、A4 都在 c1d-prec 合入集成分支之后开工；A2 / A3 碰 build_opts 的选项表，与其 `VALUED` / `BUILD_ONLY` 增项是文本冲突，合并时两边并集即可 |
| emitter-s5（暂停） | 只动 emit / 宏 | 无 |

**dyn_compare.py 冻结规则**：A4 开工时刻起 dyn_compare.py / test_dyn_compare.py 冻结，此后任何归因修正只改 Rust
dyn crate；A4 开工前在冻结版本上跑 11 例取 Python 基准 JSON（存 `build/scripts-into-rava/dyn-py/`），A4 以「冻结时的
Python 语义」为参照逐项移植，验收为与基准逐例相等。

### 4.2 步骤（每步一个或数个小提交，提交即并入集成分支）

| 步 | 内容 | 前置 | 单步验证（不跑 e2e 批次） |
|---|---|---|---|
| S1 A3 | `resolve::jdk` 改优先级与扫描面；`rava jdk` 子命令；`rava build` / `rava closure` 走新解析；build_status.json 的 jdk 段 | 无 | `cargo test -p resolve -p driver`；`rava jdk --list` 与 `jdk_select.py` 列表对照；`--jdk 99` 报错 |
| S2 A2 | `driver/src/cargo.rs`：cargo build / run、per-crate 重型判定、build_artifacts.json / build_status.json、`--build-timeout`；`--stop-after` 选项落地（A1 的 CLI 部分在此先做，旧布尔开关同提交删除，main.py / gap_scan 的调用同提交改用新选项） | S1 | HelloWorld `--stop-after compile` 产物清单与 run_tests `_record_build_artifacts` 结果集合相等；Digester 级重型例 jobs=1 判定 |
| S3 A5 | `rava audit api / corpus / native`；删 gap_scan.py、native_gap_scan.py | S2 | `rava audit api java/lang java/util` 缺口集合 = 删前 gap_scan 报告；native 报告集合相等 |
| S4 A6 | 删 dep_scc.py；perf 报告补 crate 规模表；crate-split 文档改复现说明 | S2 | `rava build --stop-after emit --perf` 输出含 crate 表 |
| S5 A1 | 删 main.py、rava_cli.py、cargo_env.py、jdk_select.py；run_tests 改调 `rava build --stop-after compile` + 读 JSON；gen_trees / seed_check / lib_pilot_golden / fetch_pilot_deps 改 `rava build` / `rava jdk`；README / environment-variables / compatibility / tasks 改写；CLAUDE.md 改写稿交用户 | S2；**c1d-prec 已合入** | gen_trees 27 例与 S5 前逐字节一致；HelloWorld / `--lib` m3 单例跑通 |
| S6 A4 | dyn crate；`--dyn-compare` / `--dyn-methods`；`--lib` 的 JVM classpath 与 lib 域；run_tests 改读 dyn_compare.json；删 dyn_compare.py、dyn_agent/、test_dyn_compare.py | S5；**c1d-prec 与 c4-regfix 的 dyn_compare 改动已合入**，冻结 | 11 例 dyn_compare.json 与 Python 基准逐例相等（类 / 方法漏覆盖、provenance 分布、unattributed） |
| S7 C | run_tests 拆 `scripts/e2e/`（含 §六 形态接口，注册表为空即无行为变化） | S6 | `run_tests.py --no-run --filter HelloWorld` 与拆前同输出；`compileall` |
| S8 | 收尾：Python 归零 grep、文档残留清理、计划状态改「已完成」 | S7 | §五 全部 |

顺序取舍：A5 / A6 排在 A1 之前——gap_scan 是 main.py 的调用方，先并入则 A1 删除 main.py 时少改一个调用方；
A4 排在 A1 之后、编排拆分之前——A4 的门最晚（两个在途分支），且 run_tests 拆分要基于 A4 后的接口一次拆到位。
S1–S4 与 c1d-prec / c4-regfix 无语义冲突，可在门开之前先做。

## 五、验收

- **生成树逐字节一致**：`scripts/gen_trees.sh` 验收集 27 例，改造前后 `compare_trees.sh` 0 差异（closure.json 的
  `/summary` 计时与 perf 计数除外）。
- **动态对照一致**：11 例（perf2 抽查集）`dyn_compare.json` 与 §4.1 冻结的 Python 基准逐例相等。
- **审计一致**：`rava audit` 三种模式与改造前报告的缺口集合相等。
- **e2e**：JDK 21 抽查（perf2 抽查集 + `--no-dyn` 2 例 + scratch 复用 1 例 + lib_pilot m3）全过；全量按分布式跑批口径累计。
- **Python 归零核对**：
  - `ls scripts/*.py` = run_tests.py、baseline_diff.py、dep_scan.py、import_e2e_corpus.py、mono_stats.py，另 `scripts/e2e/`；
  - `grep -rn "main\.py\|jdk_select\|cargo_env\|dyn_compare\|rava_cli\|gap_scan\|dep_scc" scripts docs/environment-variables.md README.md` 为 0；
  - `scripts/e2e/` 中 `"cargo"` 子进程调用为 0（版本记录除外）。
- 编译检查：`cd generator && cargo check --workspace`、`cargo test --workspace`；`python3 -m unittest discover tests/unit` 全过。

## 六、JUnit 形态接口（为 junit-crate-as-test-harness 步骤 A 预留）

S7 拆分时落地接口，注册表为空，不改变现有行为；JUnit 步骤 A 只加数据与用例。

- **形态 = 目录**：`tests/e2e/<目录>/form.toml` 存在即该目录为特殊形态，内容为数据（Python 中不写任何库类名）：
  ```toml
  [[lib]]
  name = "hamcrest"
  jar  = "hamcrest-3.0.jar"
  [[lib]]
  name = "junit4"
  jar  = "junit-4.13.2.jar"
  seed = ["org.junit.Assert", "org.junit.runner.JUnitCore"]   # 由步骤 A 定稿的固定种子集
  ```
- `e2e/select.py`：`form_of(java_file) -> Form | None`（读所在目录的 form.toml，缓存）。`Form.lib_args(libs_dir)` →
  `["--lib", "hamcrest=<abs>", "--lib", "junit4=<abs>:seed=A,B"]`（rava 侧 `--lib NAME=JAR[:seed=FQN,…]` 已支持）；
  `Form.classpath(libs_dir)` → jar 列表。
- 接线点①（转译）：`e2e/rava.py` 拼 `rava build` 参数时追加 `form.lib_args`。`--lib` 只支持单 bin，e2e 每例单独转译，形态兼容。
- 接线点②（期望输出）：`e2e/expected.py` 的 javac / java 加 `-cp <classes>:<jars>`。
- 接线点③（动态对照）：编排侧零接线——A4 已让 `rava build --dyn-compare` 按 `--lib` 自动扩 JVM classpath 并归类 lib 域。
- jar 目录：run_tests 新选项 `--pilot-libs DIR`（缺省 `tests/lib_pilot/deps/target/pilot-libs`，不新增环境变量）。
  jar 缺失时**不跳过**（静默跳过会掩盖回归）：批次开头（选例后、调度前）对选中的形态统一自动取包，逻辑与
  `scripts/fetch_pilot_deps.sh --no-scan` 相同（`mvn -B -q -f tests/lib_pilot/deps/pom.xml package` 导出到缺省目录），
  只调一次；取包后仍缺 jar（mvn 不在 PATH、网络失败等）则该形态全部用例记失败（`deps-fetch-fail`，进失败棘轮），
  批次结束报错退出码非 0。
- 台账、并行、内存纪律、产物清理全部继承；lib crate 的产物由 build_artifacts.json 一并纳入清理。

附带给用户的勘误（junit 计划文档由用户维护，本计划不改）：其 §现状「`main.py --lib`（scripts/main.py:230,349）」
与「run_tests.py:488」在 S5 / S7 后失效，入口变为 `rava build --lib` 与 `scripts/e2e/rava.py`；步骤 A 验收里的
`compileall -q codegen scripts` 中 codegen 已删除，应为 `compileall -q scripts`。

## 七、风险

- **c1d-prec 合并晚于预期**：S5 / S6 阻塞，S1–S4 可先完成并入；若 c1d-prec 先于 S5 合入后仍有 main.py 改动，
  合并时取「删除 main.py + rava 原生选项」。
- **动态对照移植偏差**：以冻结基准逐例对照兜底；差异只允许是 Python 读回 JSON 与内存结构之间的序列化差异（如排序），
  需逐项列出说明。
- **per-crate 重型阈值标定**：以 S4 测量记录（crate-split §7.7）中的单 crate 峰值标定常量，验收只看重型例与 HelloWorld
  两端判定正确；跑批内存纪律仍由 run_bg.sh / heavy_lock 兜底。
- **跑批口径中断**：run_tests 命令行不变，分布式累计续跑不受影响；rava 构建由编排每批开头完成，服务器侧无额外步骤。
- **CLAUDE.md 同步**：「常用命令」「工作区结构」中的 main.py 用法在 S5 后失效；S5 提交时附改写稿交用户落笔。

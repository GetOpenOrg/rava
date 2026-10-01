# Python 脚本并入 rava：产品路径 Python 归零

> 状态：计划（2026-10-01）。实施时机：emitter-perf2（拆 crate）合入后，与 Python 生成器删除
> （[`2026-10-01-python-generator-deletion.md`](2026-10-01-python-generator-deletion.md)）合为一轮收尾。

## 一、终态

- **产品路径 Python 代码为 0**：`.java` / `.class` → 可运行二进制的全过程（JDK 定位、javac、闭包、生成、
  cargo 编译与运行）只经 `rava` 单二进制。
- 与转译结果相关的判定逻辑（动态对照、缺口审计、crate 划分）只在 rava 内有一份实现；Python 侧
  重复实现为 0（现有 3 处：JDK 解析、重型编译阈值、dyn 对照读 closure.json）。
- 保留的 Python 仅限**测试编排与语料运维**（跑批、结果对照、语料导入），它们只调用 rava 的公开 CLI，
  不复制 rava 的任何判定规则。
- 本项目不新增自有环境变量，新增能力一律为 CLI 选项；`JAVA_HOME`、`CARGO_BUILD_JOBS` 等标准变量照常尊重。

## 二、现状清单

| 脚本 | 行数 | 职责 | 调用方 | 与 rava 的重复 |
|---|---:|---|---|---|
| main.py | 406 | 入口：JDK 选择 → `rava build`（rust）或 Python codegen → `cargo run` | 用户、run_tests | rava build 已含 javac / overlay / cargo run，main.py 的 rust 路径只剩转发 |
| cargo_env.py | 53 | 重型工作区（≥1700 类）单作业编译 | main.py、run_tests | `build_cmd.rs::HEAVY_CLASSES` 同值两份 |
| jdk_select.py | 170 | `--jdk` > JAVA_HOME > `.jdk-version` > 最新已装；brew 双前缀 / Linux / `java_home` 扫描 | main.py、run_tests、golden | `resolve::jdk::find_java_home` 优先级不同（无 `.jdk-version`，`--jdk N` 缺失时静默取 ≥N 版本），扫描面更窄（无 `/usr/local`、`HOMEBREW_PREFIX`、`java_home` 回退） |
| dyn_compare.py | 552 | JVMTI agent 跑真实 JVM，对照类 / 方法加载集与闭包，归因 provenance | run_tests | 读回 closure.json 再判定；域规则（DomainRules）从清单重新解析一遍 |
| gap_scan.py | 194 | API / 语料两模式缺口扫描（`--precheck-only` 产物里找 `panic!`） | 手工 | 靠 grep 生成文本判定缺口 |
| native_gap_scan.py | 77 | 语料逐例 `rava closure`，native / boundary 方法与手写 fn 名交叉比对 | 手工 | 手写 fn 名匹配规则（`__impl_` / `core_` 前缀）是 emit 规则的复制 |
| dep_scc.py | 107 | 类依赖 SCC，拆 crate 的划分依据 | 手工 | 拆 crate 实施后由 emit 内部完成 |
| generator_select.py | 76 | `--generator` / `RAVA_GENERATOR` 选择 | main.py、run_tests | 随 Python 生成器删除 |
| classfile_golden.py、golden/dump_*.py | 158 + 1751 | Python 生成器各层 golden | 手工 | 随 Python 生成器删除 |
| run_tests.py | 1668 | e2e 编排：选例、并行、期望输出、cargo build、产物清理、汇总 | 用户 | 自行调 `cargo build` 并解析 JSON 产物清单 |
| baseline_diff.py | 102 | 全量结果与冻结基线对照 | 手工 | 无 |
| dep_scan.py | 229 | lib pilot 依赖透视 | fetch_pilot_deps.sh | 无（语料运维） |
| import_e2e_corpus.py | 374 | 语料导入（一次性） | 手工 | 无 |

## 三、分类与终态设计

### A 类：并入 rava

**A1 入口：`rava build` 取代 main.py**

- main.py 删除；`rava build <Test.java>` 是唯一入口，选项已覆盖 main.py 的全部 rust 路径选项
  （`--jdk` / `--out` / `--clean` / `--no-run` / `--debug` / `--strict` / `--trace-class` / `--raw-sites` / `--lib` / `--locale`）。
- 阶段选择改为单一选项 `--stop-after <javac|closure|emit|compile|run>`（缺省 `run`），取代 `--no-run` /
  `--precheck-only` 两个布尔开关；`--no-run` 对应 `--stop-after emit`。run_tests 需要「编译但不运行」
  （输出比对由它自己做），即 `--stop-after compile`。
- `--build-timeout` 与 cargo 超时进 rava：超时后终止整个 cargo 进程组，退出码区分（见 A2）。
- CLAUDE.md「常用命令」与 docs/environment-variables.md 改写为 `rava build`；仓库根提供
  `generator/target/release/rava` 的构建说明（`cargo build --release -p driver`），不提供 Python 包装。

**A2 cargo 编排：cargo_env.py 并入**

- rava 内唯一的 cargo 调用模块（`driver/src/cargo.rs`）：`CARGO_TARGET_DIR=<repo>/build/target`、
  `CARGO_INCREMENTAL=0`、`--message-format=json-render-diagnostics`。
- 并发按拆 crate 后的**最大单 crate 规模**决定（不再按总类数）：emit 已知每个 crate 的类数与方法体规模，
  超过内存阈值的 crate 所在构建单作业；阈值是 rava 内一个常量，Python 侧 0 份。调用方显式设置
  `CARGO_BUILD_JOBS` 时尊重。
- 产物清单：rava 解析 cargo JSON 消息，把本工作区的 compiler-artifact / build-script-executed 写入
  `<scratch>/build_artifacts.json`；run_tests 的「通过测试产物清理」只读该文件，不再自己调 cargo。
- 失败分类统一由 rava 给出：退出码 + `<scratch>/build_status.json`（阶段、首错行、信号、超时、
  rustc 全文日志路径）；run_tests 的 OOM / 超时 / 首错行提取全部删掉，改读该文件。

**A3 JDK 解析：jdk_select.py 并入 `resolve::jdk`**

- 优先级取 jdk_select 的口径为终态：`--jdk N` > `--java-home` > 有效 `JAVA_HOME` > 仓库根 `.jdk-version` >
  已装最新版。`--jdk N` 找不到精确主版本时**报错**，不再静默取 ≥N 的版本（语料与 javac 同源的前提）。
- 扫描面合并：`HOMEBREW_PREFIX`、`/opt/homebrew`、`/usr/local` 的 Cellar（`openjdk@NN` 与裸 `openjdk`）、
  `/usr/lib/jvm`、`/usr/libexec/java_home -v N` 回退。
- 选中结果打印来源（与现 jdk_select 同格式），并写入 `<scratch>/build_status.json`。
- 新增 `rava jdk [--jdk N]`：打印解析出的 home 与来源；`rava jdk --list` 列出已装版本。
  run_tests 通过它取得 `java` / `javac` 路径（期望输出生成、动态对照都要跑真实 JVM），自身不再扫描。

**A4 动态对照：dyn_compare.py 并入 `rava build --dyn-compare`**

- 在 `--stop-after` 之外独立开关：javac 与闭包完成后，rava 用同一 JDK 跑真实 JVM（`-Xlog:class+load`
  + JVMTI agent），在**内存中的闭包结果**上做类 / 方法对照与 provenance 归因，不读回 closure.json。
- 域规则直接用分析器已加载的清单模型（closure.toml / vm_intrinsics.toml），不再在 Python 里重解析。
- JVMTI agent 源（`scripts/dyn_agent/load_trace.c`）保留为 C，移到 `generator/crates/dyn/agent/`；
  由该 crate 的 build.rs 用系统 cc 编译并按 JDK home 缓存（现 `ensure_agent` 的逻辑）。
- 结果写 `<scratch>/dyn_compare.json`，摘要一行打印（现 `summary_tag` 格式）；run_tests 的汇总只读 JSON
  （`print_summary` 保留在 run_tests，属编排）。方法粒度对照 `--methods` 一并迁入，仍为诊断项。
- `--closure-json` 只剩调试用途（N2 的全量比对开关保留）。
- `tests/unit/test_dyn_compare.py` 的用例迁为 dyn crate 的 Rust 单元测试，Python 版删除。

**A5 审计：gap_scan.py、native_gap_scan.py 并入 `rava audit`**

- `rava audit api <package>... [--recursive]` 与 `rava audit corpus [--filter ...] [-j N]`：
  缺口判定直接用 emit 的方法归类结果（native-missing / boundary-stub / 手写已覆盖），不再 grep 生成文本，
  不复制手写 fn 名匹配规则。
- 输出 `docs/reports/gap-scan-<模式>.md` 与 `native-gap-scan.md`（格式不变）。
- corpus 模式逐例只做闭包 + 归类（不写生成文件），语料枚举是 rava 内 `tests/e2e/**/*.java` 遍历。

**A6 crate 划分：dep_scc.py 淘汰**

- 拆 crate（`2026-10-01-rustc-memory-and-crate-split.md`）实施后，SCC 计算与划分在 emit 内部完成；
  需要查看划分时用 `rava build --stop-after emit --perf`，在 perf 报告中列出每个 crate 的类数 / 体规模。

### B 类：随 Python 生成器删除

generator_select.py、classfile_golden.py、`scripts/golden/`（dump_* 共 8 个）、main.py 的 Python 分支
（`prepare_scratch` / `_python_codegen` / `_parse_lib_specs`）。按删除计划 §二、§三 一次删净，同时删
`--generator` 选项与 `RAVA_GENERATOR`。

### C 类：保留 Python（测试编排与语料运维）

| 脚本 | 终态 |
|---|---|
| run_tests.py | 只做编排：选例、并行、期望输出比对、产物清理、汇总。每例一次 `rava build --stop-after compile [--dyn-compare]` + 运行二进制比对输出。按职责拆为 `scripts/e2e/`（cli / select / runner / expected / artifacts / report），单文件 ≤ 600 行；删除对 jdk_select / cargo_env / dyn_compare / generator_select 的导入 |
| baseline_diff.py | 保留（删除判据工具，删除完成后随基线文件一起归档删除） |
| dep_scan.py、import_e2e_corpus.py | 保留（语料运维，不在产品路径） |
| shell 脚本 | 保留；gen_trees / seed_check / closure_bench / emit_bench / rustc_profile 中调用 main.py 的改为 `rava build` |

## 四、时序

1. **前置**：emitter-perf2（拆 crate、N4）合入 rust-closure-analyzer；Python 生成器删除条件满足并经用户确认。
2. 一个子代理一轮实施，顺序：A3（JDK）→ A2（cargo）→ A1（入口 + `--stop-after`）→ A4（dyn）→ A5（audit）
   → A6 → B 类删除 → run_tests 拆分与改接口 → shell 脚本与文档改写。
   A 与 B 同轮完成，中间不保留 main.py 转发层。
3. 每步小步提交；全部完成后统一验证（§五），不逐项跑 e2e。

## 五、验收

- **生成树逐字节一致**：`scripts/gen_trees.sh` 验收集 27 例，改造前后 `compare_trees.sh` 对照为 0 差异
  （入口迁移不得改变生成物）。
- **动态对照一致**：抽 11 例（perf2 抽查集）比较 `dyn_compare.json` 与改造前 Python 结果：
  类 / 方法漏覆盖数、provenance 归因逐例一致。
- **审计一致**：`rava audit api java/lang java/util` 与改造前 gap_scan 报告的缺口集合一致。
- **e2e**：JDK 21 抽查（perf2 抽查集 + `--no-dyn` 2 例 + scratch 复用 1 例）全过；全量由删除计划 §四 的
  JDK 21 全量 + `baseline_diff.py` 判定。
- **Python 归零核对**：
  - `ls scripts/*.py` 只剩 run_tests 入口、baseline_diff.py、dep_scan.py、import_e2e_corpus.py 与 `scripts/e2e/`；
  - `grep -rn "main.py\|jdk_select\|cargo_env\|dyn_compare\|generator_select\|RAVA_GENERATOR" scripts docs/environment-variables.md CLAUDE.md` 为 0；
  - run_tests 中 `"cargo"` 调用为 0（`cargo --version` 环境记录除外）。
- 编译检查：`cd generator && cargo check --workspace`；生成器单元测试与 `tests/unit` 全过。

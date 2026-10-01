# Python 生成器删除清单

> 状态（2026-10-01）：本文只是清单，**当前不删除任何文件**。Python 生成器（`codegen/`）作为对照基线保留，
> `--generator python` 开关继续保留并可用。迁移项见 [`2026-10-01-codegen-dependency-inventory.md`](2026-10-01-codegen-dependency-inventory.md)（已全部完成）。

## 一、删除条件

**rust 缺省下 JDK 21 全量 e2e 的通过集合 ⊇ 冻结的 Python 基线，且无转译失败。**

基线口径（2026-10-01 用户决定）：Python 基线冻结，不再用 Python 生成器重跑；JDK 25 不设 Python 基线，只比较 JDK 21。

- **P21（冻结）**：[`2026-10-01-python-baseline-jdk21.txt`](2026-10-01-python-baseline-jdk21.txt)，1029 例。
  - 来源：服务器 `distribute_tests.py --jdk 21` 在 master 上的通过记录（`test_results/master_passed_jdk21.txt`），文件时间 2026-09-30 23:36 (+0800)。
  - 当时的缺省生成器是 Python：缺省切到 rust 的提交是 49a7684d（2026-10-01 00:02），而且只在工作分支上。
  - sha256 `c69f200bb30a1fd834a9d1a8bf24a92c41846b4ee55fb18119544f8c8493b78c`。
- **R21**：JDK 21 全量 `run_tests.py`，rust 缺省（或服务器 `distribute_tests.py --jdk 21`）。
- **判定**：
  - 按测试名，`P21 ⊆ R21 通过`。只看数量不够：rust 多通过的测试不能抵消 Python 通过而 rust 失败的测试。
  - R21 没有 `TRANSPILE-FAIL`。
  - 差集非空时，逐例说明原因（例如用例已删除或改名）；无法说明的视为回归。
- **记录**：R21 的提交号、通过数、差集为空的证明、日志路径，填在本文 §五，作为删除依据。

## 二、删除清单（条件满足后执行）

### A. Python 生成器本体

| 路径 | 规模 | 说明 |
|---|---|---|
| `codegen/`（整包，78 个 .py，约 2.7 万行） | 全部 | 含 `closure_input.py`、`jdk_resolver.py`、`runtime_manifest.py`、各审计模块 |

### B. 只服务 Python 或 Python↔Rust 对等验证的设施

| 路径 | 说明 |
|---|---|
| `scripts/golden/dump_{input,ty,ir,sim,cfg,instr,method,emit}.py` | 逐层 golden 采集（Python 真源） |
| `generator/crates/{input,ty,ir,sim,cfg,instr,method,emit}/tests/golden.rs` | 逐层 golden 对照 |
| `generator/crates/ir/tests/golden_support/`，以及 `{cfg,sim,instr}/tests/support/` 中只供 golden 使用的部分 | golden 反序列化 / 重放支持（删除前核对单测是否共用） |
| `generator/crates/*/GOLDEN_DIFF.md`、`generator/crates/emit/P5B_NOTES.md` 中的 golden 章节 | 对照差异记录 |
| `scripts/classfile_golden.py` + `rava dump-classes` 子命令（`generator/crates/driver/src/dump.rs`） | 类文件解析对照 |
| `tests/unit/test_{cfg_structuring,closure_folds,data_bundle,erased_queries,fold_array_literals,jca_services,jvm_type,try_expr,typeir_sites,upcast_expr}.py` | 测试对象是 Python 模块（Rust 对应测试见依赖清单 §三）。`test_dyn_compare.py` 保留 |

### C. 脚本内 Python 分支

| 位置 | 删除内容 |
|---|---|
| `scripts/main.py` | `_python_codegen`、`_parse_lib_specs`，以及 Python 分支的 overlay（`prepare_scratch` 及其文件复制辅助函数；rust 路径由 `rava build` 做 overlay） |
| `scripts/generator_select.py` | 生成器选择整体移除（见 §三）：删 `DEFAULT_GENERATOR` / `CHOICES` / `ENV_VAR`、`add_argument`、`resolve`；`rava_cmd` / `run_rust` 迁入 `scripts/rava_cli.py`（`main.py`、`run_tests.py`、`gap_scan.py` 改为从此导入），原文件删除 |
| `scripts/main.py`（开关） | `--generator` 选项、`resolve_generator` 调用与按生成器分派的分支，转译段直接调用 `rava build --no-run` |
| `scripts/run_tests.py`（开关） | `generator_select.add_argument`、`--generator` 透传（`MAIN_FLAGS += ["--generator", …]`） |
| `scripts/run_tests.py` | `FALLBACK_IDS` 中的 Python B 组十项（`sig-parse-*`、`type-map-params`、`vars-*`、`sam-*`）；`EQUIV_IDS` 注释改指 Rust 发射点 |
| `scripts/gap_scan.py` | 无（api 模式已走 `rava build --api-package`） |
| `scripts/seed_check.sh` | `PYTHONHASHSEED` 双种子说法改为「两次生成逐字节一致」 |

### D. 文档与注释（只改指向，不删文件）

- `docs/environment-variables.md`：删除 `main.py` / `run_tests.py` 两处 `--generator` 行、第三节 `RAVA_GENERATOR` 行与开头「唯一例外」说明（删除后项目没有自有环境变量）；`--image` 行中 `main.py --generator rust` 改为 `main.py`；`--debug` / `--raw-sites` 中 python 口径的说明。
- `runtime/java_runtime/*.toml` / `*.txt` 与 `runtime/java_support/*.java` 注释里的 `codegen/…` 路径，改指 Rust 读取端（`input::manifest`、`closure::manifest`、`resolve::image`）。
- `docs/rules.md`、`docs/java-bytecode-transpiler-design.md`、`docs/compatibility.md` 中的 `codegen/` 路径。
- `CLAUDE.md` 的生成器相关条目（原则 4 的 `instr.py` / `type_map.py` / `emitter.py`、常用命令里的 `python3 -m unittest tests.unit.<模块>`）：由用户改定。

## 三、`--generator` 开关

已决定（2026-10-01）：按终态处理。删除 Python 生成器时，`--generator` 选项与 `RAVA_GENERATOR` 环境变量**一并删除**，不保留只剩 `rust` 一个取值的开关。具体改动列在 §二 C（`generator_select.py`、`main.py`、`run_tests.py`）与 §二 D（`docs/environment-variables.md`）。

删除前的现状不变：`--generator python` 继续保留、可用。删除本身仍须用户确认后执行。

删除后的核对：`grep -rn "generator_select\|RAVA_GENERATOR\|--generator" scripts tests docs` 只允许命中本清单与历史计划文档。

## 四、执行顺序（条件满足后）

1. 按 §一 跑 R21 全量（`run_tests.py --jdk 21 --record-passed`，或服务器分布式跑批），用 `scripts/baseline_diff.py --passed <通过清单…> --log <跑批日志…> --commit <sha>` 与冻结的 P21 对照，报告（Markdown，退出码 0 = 满足）贴入 §五。
2. 一次性删除 A、B，并修改 C（按「一次删到终态再验证」原则，不逐文件分批）。
3. 验证：
   - 生成器各 crate 通过 `cargo build` / `cargo test`；
   - `python3 -m unittest tests.unit.test_dyn_compare`；
   - `scripts/gen_trees.sh` 与删除前的 27 例树逐字节一致；
   - HelloWorld 与 Digester 通过 `cargo check`；
   - 全量 e2e 复跑，通过集合与 §五 的 R 组一致。
4. 修改 §二 D 中的文档注释，单独提交。

## 五、删除依据记录

（条件满足时填写：R21 提交号、通过数、`P21 − R21` 差集及说明、日志路径。）

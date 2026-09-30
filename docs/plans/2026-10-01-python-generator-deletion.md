# Python 生成器删除清单

> 状态（2026-10-01）：本文只是清单，**当前不删除任何文件**。Python 生成器（`codegen/`）作为对照基线保留，
> `--generator python` 开关继续保留并可用。迁移项见 [`2026-10-01-codegen-dependency-inventory.md`](2026-10-01-codegen-dependency-inventory.md)（已全部完成）。

## 一、删除条件

**rust 缺省下全量 e2e（JDK 21 + 25）通过数不少于 Python 基线且无回归。**

判定方法：同一提交上跑四组 `run_tests.py` 全量结果，按以下口径比较：

| 组 | 命令 |
|---|---|
| R21 | `JAVA_HOME=<JDK 21> python3 scripts/run_tests.py -j N` |
| P21 | `JAVA_HOME=<JDK 21> python3 scripts/run_tests.py -j N --generator python` |
| R25 / P25 | 同上，JDK 25 |

- **通过数**：`|R21 通过| ≥ |P21 通过|`，且 `|R25 通过| ≥ |P25 通过|`。
- **无回归**：
  - 按测试名，`P21 通过 ⊆ R21 通过`，`P25 通过 ⊆ R25 通过`。只看数量不够：rust 多过的测试不能抵消 Python 通过而 rust 失败的测试。
  - 同时 rust 两组都没有 `TRANSPILE-FAIL`。
- **记录**：四组结果的 `build/logs` 汇总连同提交号落在本文 §五，作为删除依据。

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
| `scripts/generator_select.py` | `'python'` 取值与 `resolve` 中的 Python 分支（开关本身的去留见 §三） |
| `scripts/run_tests.py` | `FALLBACK_IDS` 中的 Python B 组十项（`sig-parse-*`、`type-map-params`、`vars-*`、`sam-*`）；`EQUIV_IDS` 注释改指 Rust 发射点 |
| `scripts/gap_scan.py` | 无（api 模式已走 `rava build --api-package`） |
| `scripts/seed_check.sh` | `PYTHONHASHSEED` 双种子说法改为「两次生成逐字节一致」 |

### D. 文档与注释（只改指向，不删文件）

- `docs/environment-variables.md`：`--generator`、`RAVA_GENERATOR` 条目；`--debug` / `--raw-sites` 中 python 口径的说明。
- `runtime/java_runtime/*.toml` / `*.txt` 与 `runtime/java_support/*.java` 注释里的 `codegen/…` 路径，改指 Rust 读取端（`input::manifest`、`closure::manifest`、`resolve::image`）。
- `docs/rules.md`、`docs/java-bytecode-transpiler-design.md`、`docs/compatibility.md` 中的 `codegen/` 路径。
- `CLAUDE.md` 的生成器相关条目（原则 4 的 `instr.py` / `type_map.py` / `emitter.py`、常用命令里的 `python3 -m unittest tests.unit.<模块>`）：由用户改定。

## 三、`--generator` 开关

按用户决定，现在保留 `--generator python`。删除 A–C 之后，`python` 取值没有实现可指，届时开关和 `RAVA_GENERATOR` 是一起移除，还是只保留 `rust` 一个取值，由用户在执行删除时决定。本清单不预设。

## 四、执行顺序（条件满足后）

1. 按 §一 跑四组 e2e，把结果记入 §五。
2. 一次性删除 A、B，并修改 C（按「一次删到终态再验证」原则，不逐文件分批）。
3. 验证：
   - 生成器各 crate 通过 `cargo build` / `cargo test`；
   - `python3 -m unittest tests.unit.test_dyn_compare`；
   - `scripts/gen_trees.sh` 与删除前的 27 例树逐字节一致；
   - HelloWorld 与 Digester 通过 `cargo check`；
   - 全量 e2e 复跑，通过集合与 §五 的 R 组一致。
4. 修改 §二 D 中的文档注释，单独提交。

## 五、删除依据记录

（条件满足时填写：提交号、四组通过数、差集为空的证明、日志路径。）

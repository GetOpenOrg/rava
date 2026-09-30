# codegen/ 依赖清单与迁移（Rust 生成器全替换的前置）

> 状态（2026-10-01）：全部依赖已逐项定性。必须迁移的项已迁移，缺省 rust 路径上不再 import `codegen`。
> 余下依赖方都是 Python 生成器自身的对照 / 测试设施，随 Python 生成器一并删除。
> 删除清单与删除条件见 [`2026-10-01-python-generator-deletion.md`](2026-10-01-python-generator-deletion.md)。

## 一、口径

- **依赖方**：仓库内 `codegen/` 以外、import 或调用 `codegen` 的文件（`grep -E "from codegen|import codegen|codegen[./]"`，文档不计）。
- **处置**只有两种：
  - **迁移**：该功能在 rust 缺省路径上仍然需要。改为由 Rust 或脚本自身实现，终态不 import `codegen`。
  - **随删**：只服务 Python 生成器，或是 Python↔Rust 对等验证的设施。Python 删除后没有对象可比，与 Python 一同删除。
- **约束**：
  - 迁移后 `--generator python` 仍完整可用（Python 路径不删、不破坏）。
  - Python 专用的导入一律收进 Python 分支，按需导入。

## 二、清单

| # | 依赖方 | 用到的 codegen 模块 | rust 缺省路径是否经过 | 处置 | 落点 / 理由 | 状态 |
|---|---|---|---|---|---|---|
| 1 | `scripts/generator_select.py` `run_rust` | `jdk_resolver.JdkResolver.image_class_dirs()` | 是 | 迁移 | 由 `rava build` / `rava emit` 在未给 `--image` 时自行派生（`generator/crates/resolve/src/image.rs`：`jimage list` 与 jmod 求差集，`jimage extract` 提取到缓存，外加 `runtime/java_support` 的 `javac --patch-module` 编译缓存）；`run_rust` 不再传 `--image` | ✅ a610c296 |
| 2 | `scripts/closure_bench.sh` | 同上 | 是（基准脚本） | 迁移 | 新增 `rava image-dirs`，每行输出一个目录（与第 1 项同一实现） | ✅ a610c296 |
| 3 | `scripts/main.py` | 模块级 import：`transpile`、`cfg.STATS`、`equiv_audit`、`fallback_audit`、`raw_audit`、`options`、`constants`（`RUNTIME_*`、`scratch_pkg_version`）、`emitter.to_snake`、`transpile.LibSpec`、`type_map._PRELUDE_DISAMBIGUATED` | 是（模块级 import 在 rust 路径上同样执行） | 迁移 | codegen 导入全部收进 `_python_codegen` 与 Python 分支的 overlay。rust 分支：overlay 由 `rava build` 完成（此前两边各做一次，属重复）；bin 名读 scratch 的 `user/Cargo.toml` `[[bin]]`（生成器是唯一真源）；缺省 scratch 目录名与 `run_tests.py` 同一套驼峰转蛇形规则 | ✅ a610c296 |
| 4 | `scripts/dyn_compare.py` | `runtime_manifest`（边界包 / VM 边界类 / 放行 / VM 上行入口） | 是（`run_tests.py` 动态对照） | 迁移 | 直接用 `tomllib` 读 `closure.toml` / `seeds.toml`，口径与 Python 逐项一致；单测 `ManifestTest` | ✅ 7182039f |
| 5 | `scripts/gap_scan.py` api 模式 | `JdkResolver`、`classfile`、`callchain._is_boundary_class`、`options.JDK_SEEDS`、`transpile` | 否（直接驱动 Python 生成器） | 迁移 | `rava build --api-package P [--api-recursive] --precheck-only`：Rust 侧枚举包内 public 类的 public / protected 方法作为根（边界类跳过），预检行格式与语料模式相同。语料模式本就经 `main.py --precheck-only`，缺省即 rust。`--modules` 选项去掉（包唯一确定模块） | ✅ 3fb1aea1 |
| 6 | `codegen/closure_input.py` | （Python 包内）`rava closure` → ClassInfo 注册表 | 否 | 随删 | Python 路径专用的闭包接入；rust 路径的闭包由 `rava build` 在进程内完成（`build_cmd::analyze`） | 保留至删除 |
| 7 | `scripts/golden/dump_{input,ty,ir,sim,cfg,instr,method,emit}.py` + `generator/crates/*/tests/golden.rs`（及 `ir/tests/golden_support`、`{cfg,sim,instr}/tests/support` 中读 golden 的部分） | 各层 Python 模块 | 否 | 随删 | 逐层 Python↔Rust 对等验证：以 Python 产物为真源，比 Rust 各层结果。切换后 Rust 自身即真源，没有对照对象。回归保障改由：① 27 例生成树前后对照（`gen_trees.sh` + `compare_trees.sh`，每次生成器改动的验收）；② 各 crate 单元测试；③ `driver/tests/build_cli.rs` 端到端集成测试；④ e2e。Rust `golden.rs` 缺 golden 时本就跳过，删除前不影响 `cargo test` | 保留至删除 |
| 8 | `scripts/classfile_golden.py` | `classfile`、`jdk_resolver`、`vm_constants` | 否 | 随删 | Python 类文件解析与 `rava dump-classes` 的对照。`dump-classes` 子命令只为这项对照而设，一并删除 | 保留至删除 |
| 9 | `tests/unit/test_{cfg_structuring,closure_folds,data_bundle,erased_queries,fold_array_literals,jca_services,jvm_type,try_expr,typeir_sites,upcast_expr}.py` | 对应 Python 模块 | 否 | 随删 | 测试对象是 Python 模块本身。Rust 对应测试见 §三 | 保留至删除 |
| 10 | `scripts/run_tests.py` | 仅注释引用；`EQUIV_IDS` / `FALLBACK_IDS` 为脚本内常量 | 是 | 保留 | 不 import codegen。`FALLBACK_IDS` 已收录 Rust 降级点（`class-extras` / `lvt-substitute` / `sam-ctor-path`）；删除时去掉 Python B 组十项 | — |
| 11 | `scripts/jdk_select.py` | 仅注释引用 | 是 | 保留 | 不 import codegen；删除时改注释 | — |
| 12 | `scripts/lib_pilot_golden.sh`、`scripts/fetch_pilot_deps.sh` | 经 `main.py --lib` | 是 | 无需迁移 | 经 `main.py`，缺省即 rust。`--lib` 在 rust 路径已支持 | — |
| 13 | `scripts/seed_check.sh` | 经 `main.py`（`PYTHONHASHSEED` 双种子） | 是 | 无需迁移 | 在 rust 下是一般的确定性检查（两次生成逐字节一致）；删除 Python 后可去掉 `PYTHONHASHSEED` 的说法 | — |
| 14 | `scripts/gen_trees.sh` / `compare_trees.sh` | 经 `main.py` | 是 | 无需迁移 | `compare_trees.sh` 已去掉 closure.json 计时噪声（0ef2ebe1） | — |
| 15 | `runtime/java_runtime/*.toml` / `*.txt`、`runtime/java_support/*.java` | 注释中引用 codegen 路径 | — | 保留 | 清单是手写层真源，Rust 读同一文件。删除时统一改注释指向 Rust 读取端 | — |
| 16 | `scripts/emit_bench.sh`（合入 rust-closure-analyzer 带入） | `jdk_resolver.JdkResolver.image_class_dirs()` | 是（基准脚本） | 迁移 | 去掉 `--image` 拼装，由 `rava build` / `rava emit` 自行派生（同第 1 项） | ✅ 合并提交 |

## 三、Python 单测 → Rust 对应测试

| Python 单测 | 覆盖面 | Rust 对应 |
|---|---|---|
| `test_cfg_structuring` | 控制流结构化 / 条件代数 / try 计划 | `generator/crates/cfg`（`src/tests.rs`、`tests/golden.rs`）、`method/src/try_plan.rs` 测试 |
| `test_closure_folds` | closure.json 折叠消费 | `generator/crates/input/src/norm.rs` 测试；闭包侧 `closure/src/engine/fold.rs` |
| `test_data_bundle` | 纯数据资源束判定 | `closure/src/seeds/data_bundle.rs` 测试 |
| `test_erased_queries` / `test_typeir_sites` / `test_jvm_type` | 擦除查询 / 类型代数 / 短名索引 | `generator/crates/ty`、`sim/src/types.rs`、`sim/tests/unit.rs` |
| `test_fold_array_literals` | 数组初始化器折叠 | `method/src/fold_array.rs` 测试 |
| `test_jca_services` | JCA 服务清单 | `closure/src/seeds/jca.rs` 测试 |
| `test_try_expr` / `test_upcast_expr` | IR 渲染（`?` 传播 / 上转） | `ir/src/render/tests_expr.rs`、`tests_stmt.rs`、`render/cast.rs` |
| `test_dyn_compare` | 动态对照（非 codegen） | 保留（已不依赖 codegen） |

## 四、验证

- **第 1、2 项**：
  - `resolve::image` 单测：`jimage list` 解析、Java 正则转义、指纹稳定；真 JDK 测试：镜像独有类目录中的类均不在 jmod 中，VM 支持类目录非空。
  - `driver/tests/build_cli.rs` `image_dirs_lists_existing_class_dirs`：`rava image-dirs` 每行为存在的目录，VM 支持类目录数等于 `runtime/java_support` 模块数。
  - 与 Python `JdkResolver.image_class_dirs()` 逐类对照：JDK 21 两个目录 21 个类、JDK 25 一个目录 16 个类，类集合相同（JDK 25 上两边都报同一个 VM 支持类编译失败）。
- **第 3 项**：
  - rust 路径 `python3 -X importtime scripts/main.py HelloWorld.java --no-run`：导入表中 `codegen` 出现 0 次。
  - Python 分支照常可用：`--generator python --no-run` 生成 HelloWorld；两条路径的 `_bin_name` 都读出 `hello_world`。
  - 缺省 scratch 名：新规则与 `codegen` `to_snake` 在全部 1087 个 `tests/**/*.java` 主类名上结果相同。
- **第 5 项**：
  - `api_roots::tests::real_jdk_roots`：只取 public 类的 public / protected 非 `<clinit>` 方法；边界域包整体跳过；VM 耦合边界类照常纳入；`--api-recursive` 含子包。
  - `build_opts` 单测：选项解析与互斥校验。
  - `build_cli.rs` `api_package_precheck`：端到端出 `[api]` 行与预检行。
  - `gap_scan.py api java/util/function`：43 类 / 79 入口方法，与 Python 口径的枚举结果相同。
- **生成树**：以 31dba6a0（迁移前，脚本经 Python 解析器取 `--image`、main.py 做 overlay）为基线，对照 3fb1aea1（rava 自行派生镜像目录、自行 overlay）。27 例逐字节 0 差异，raw-audit 相同。

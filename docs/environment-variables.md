# 命令行选项与环境变量

> rava **不设项目自有环境变量**（2026-09-28 清理：原 `RAVA_*` 共 14 个全部改为命令行参数、生成文件或标准变量）。
> 新增开关一律做成命令行参数；需要传给 cargo 构建的内部数据写入 scratch 的生成文件，由 `build.rs` 读取。
> 唯一例外：生成器切换期的 `RAVA_GENERATOR`（见第三节），Python 生成器删除时随 `--generator` 一并删除。

## 一、命令行选项

### `scripts/main.py`（转译 + 运行单个程序）

| 选项 | 用途 |
|---|---|
| `--jdk N` | 指定 JDK 主版本。缺省按 `JAVA_HOME` → 仓库根 `.jdk-version`（当前 21）→ 已安装最新版 |
| `--no-run` / `--clean` / `--out DIR` | 只生成不编译运行 / 清空 scratch 重建 / 指定 scratch 目录 |
| `--debug` | 诊断明细：兜底点 traceback 与逐条触发、闭包分析未解析调用、cfg 结构化逐块判定 |
| `--strict` | 严格模式：转译兜底改为硬失败；缺手写实现的 native 方法编译报错（写入 scratch 的 `java_runtime/strict.txt`，`build.rs` 读取） |
| `--trace-class CLASS` | 打印该类或方法（斜线形态，如 `java/net/InetAddress`、`类.方法:描述符`）入闭包的最短 provenance 链，回答“为什么被拉进闭包”（转交 `rava closure --why`） |
| `--raw-sites FILE` | Raw 逃生舱构造位点剖面追加写入 FILE（FS-Q1 热点排序），不影响生成代码。行格式 `{次数}\t{种类}\t{位点}`，次数降序；`python` 位点为 Python 调用栈，`rust` 位点为构造调用处 `文件:行:列`（`#[track_caller]`），种类 `raw_expr` / `raw_stmt` / `raw_item` |
| `--generator {python,rust}` | 生成器实现：`python` = `codegen/`；`rust` = `generator/` 的 `rava build --no-run`（只替换转译段，overlay 与 cargo 流程共用）。缺省取 `RAVA_GENERATOR`，再缺省 `rust`（2026-10-01 起；缺省值唯一定义在 `scripts/generator_select.py`）。两路径支持同一组选项（`rust` 下原样转交 `rava build`）；`python` 保留为对照基线 |

```bash
python3 scripts/main.py Foo.java --no-run --trace-class java/security/Provider
python3 scripts/main.py tests/e2e/04_collections/TestArrayList.java --no-run --raw-sites /tmp/raw.txt
python3 scripts/main.py tests/e2e/01_basics/BubbleSort.java --strict
```

### `scripts/run_tests.py`（e2e 跑批）

| 选项 | 用途 |
|---|---|
| `--jdk N` | 同上 |
| `--build-timeout SEC` | 单测试 cargo 构建超时。缺省按闭包规模自动：重型闭包 3000 秒，其余 600 秒 |
| `--debug` / `--strict` | 透传给每个测试的 `main.py` |
| `--deny SPEC` | 审计计数非零升级为整体失败（`equiv` / `fallback` / `stub-hit` 等，见 `--help`） |
| `--no-dyn` | 关闭动态对照（缺省开，见下） |
| `--generator {python,rust}` | 同 `main.py`；非缺省取值显式透传给每个测试的 `main.py`（`[meta]` 的 options 可见） |

**动态对照**（闭包计划 C5，`scripts/dyn_compare.py`）：每个测试转译成功后（cargo 之前，`--no-run` 下同样执行）
用 JVMTI agent + `-Xlog:class+load,class+init` 跑一次原始 Java 程序，把 JVM 实际加载的类与 `closure.json` 对照。
结果行附 `dyn miss N / extra M prov P%`（翻译域漏覆盖 / 静态多出 / 多出类 provenance 覆盖率；有无调用栈的加载时再附
`unattr K`），明细落盘 `build/jdk<N>/logs/dyn/<test>.json`，汇总行 `[dyn-compare]` 列出全部漏覆盖。
每测试一次 java 运行（实测约 0.1 秒，相对 5–70 秒的转译可忽略），故缺省开启。agent 首次使用时以 `$CC`（缺省 `cc`）
编译，按源码 + JDK 缓存于 `build/dyn_agent/`。单独运行：`python3 scripts/dyn_compare.py build/jdk21/<test> [-o out.json]`。

启动时的 `[meta]` 行打印 `PYTHONHASHSEED`、`CARGO_INCREMENTAL`、`CARGO_BUILD_JOBS` 与透传选项，便于事后解读结果。

### `rava build` / `rava emit`（Rust 生成器外壳，`generator/crates/driver`）

```bash
cd generator && cargo run --release -q -- build ../tests/e2e/01_basics/HelloWorld.java --jdk 21 --no-run
cargo run --release -q -- emit ../build/hello_world/closure_input/closure.json --jdk 21 --java ../tests/e2e/01_basics/HelloWorld.java
```

| 选项 | 用途 |
|---|---|
| `build <A.java>…` | javac（`-g`，JDK ≥ 14 加 `--enable-preview --release N`）→ 闭包分析（同 `rava closure`）→ 发射 scratch →（缺省）`cargo run --bin <入口 snake 名>`（`CARGO_TARGET_DIR=<仓库>/build/target`、`CARGO_INCREMENTAL=0`）。用户类与 `closure.json` 落 `<scratch>/closure_input/` |
| `emit <closure.json>` | 由既有 `closure.json` 与用户类目录重建输入后发射（不编译运行）。`--classes DIR` 缺省 `closure.json` 同目录的 `classes/`；`--java A.java`（可多次）给出源文件，决定用户类包布局与入口序 |
| `--jdk N` / `--java-home P` | JDK 选择（互斥；缺省同 `rava closure`：`JAVA_HOME` → 已安装最新版） |
| `--runtime R` | 手写层真源 `runtime/java_runtime`（缺省自当前目录向上查找） |
| `--out DIR` | scratch 目录。build 缺省 `<仓库>/build/<入口 snake 名>`；emit 缺省为 `closure.json` 所在 `closure_input/` 的上级 |
| `--image D` | 镜像独有 / VM 支持类目录（可多次；Rust 侧尚无 jimage 提取，`main.py --generator rust` 取 `JdkResolver.image_class_dirs()` 传入） |
| `--main 类` / `--locale L` / `--root 类.方法:描述符` | 仅 build：入口类（缺省首个源文件的同名类，否则首个带 static main 的用户类）/ locale 种子 / 外部种子方法（均可多次） |
| `--clean` | 发射前清空 scratch（emit 的输入位于 scratch 内时拒绝） |
| `--no-run` | 仅 build：只生成不编译运行 |
| `--strict` | 同 `main.py --strict`（写入 scratch 的 `java_runtime/strict.txt`） |
| `--lib NAME=JAR[:seed=FQN,…]` | 仅 build：jar 输入模式（可多次，声明序即 crate 依赖序）。jar 上 javac `-cp` 与类路径；无 seed = 整包（jar 全部类进 lib crate，种子 = 全部类的 public 方法），有 seed = 子集（种子类须在 jar 内，只收闭包触达的 jar 类）。每个 lib 一个 `crate-type = ["lib"]` 的 crate：public / protected → `pub`，其余 → `pub(crate)`；user 依赖全部 lib。与 `--batch` 互斥 |
| `--batch` | 仅 build：入口写 `user/src/bin/<bin>.rs`（`#[path]` 引用同级类文件），向 `user/Cargo.toml` 追加 `[[bin]]`（已有同名 bin 跳过） |
| `--trace-class 类` | 仅 build：打印该类或方法（`类.方法:描述符`）入闭包的最短 provenance 链（`      [why] …`，同 `rava closure --why`） |
| `--debug` | 闭包未解析调用（`[closure] unresolved: …`）与存根兜底逐条（`[cfg-audit] stub fallback (位点): 方法: 原因`） |
| `--precheck-only` | 发射后只输出完整预检明细（`[precheck]` 不截断），不出审计行、不编译运行。缺省时预检每类明细封顶 40 行 |

### 重型闭包的自动处理（无需配置）

`scripts/cargo_env.py` 统计工作区内带生成标记的类文件数，≥ 1700 视为重型闭包（16G 机器上单 rustc 峰值约 14G，
双作业会被 OOM 杀）：

- 所有编译缺省 `CARGO_PROFILE_DEV_DEBUG=line-tables-only`（调试信息减量，rustc 内存明显下降）；
- 自动加 `CARGO_BUILD_JOBS=1`，打印 `[cargo-env]` 一行；显式设置 `CARGO_BUILD_JOBS` 时尊重显式值；
- `run_tests.py` 构建超时自动放宽到 3000 秒（`--build-timeout` 显式值优先）。

## 二、生成程序的运行期诊断

生成的二进制读取 Rust 标准变量 `RUST_BACKTRACE`：

| 取值 | 效果 |
|---|---|
| `1` | main 线程因未捕获异常退出时，除 Java 格式异常栈外附打印 Rust 回溯 |
| `full` | 另在每个 VM 抛异常点（NPE / 数组越界 / ClassCastException …）打印 `[vm-throw]` 与 Rust 回溯（输出量大） |

```bash
RUST_BACKTRACE=1 python3 scripts/main.py Foo.java
```

## 三、会读取 / 设置的标准变量

| 变量 | 读取 / 设置方 | 说明 |
|---|---|---|
| `CC` | `dyn_compare.py` | 编译动态对照 JVMTI agent 的 C 编译器（缺省 `cc`；需要能找到 `$JAVA_HOME/include` 下的 jvmti.h） |
| `JAVA_HOME` | `jdk_select.py`、`jdk_resolver.py`、各脚本 | JDK 位置。选中 JDK 后由脚本写回，javac / java / jmods 均经它取得 |
| `CARGO_BUILD_JOBS` | 用户 / `cargo_env.py` | rustc 并行作业数，显式设置时优先于重型闭包自动判定 |
| `RUST_BACKTRACE` | 生成程序 | 见第二节 |
| `CARGO_TARGET_DIR` | 脚本自动设置 | 共享编译缓存（`run_tests.py` 为 `build/jdk<N>/target`，`main.py` 为 `build/target`），无需手动设置 |
| `CARGO_INCREMENTAL` | 脚本自动设为 `0` | 关闭增量编译：宽闭包下增量元数据是 OOM 的主要诱因 |
| `CARGO_PROFILE_DEV_DEBUG` | 脚本缺省 `line-tables-only`（`cargo_env.py`，显式设置时尊重） | 减少调试信息：debuginfo=2 下大闭包 rustc 峰值约 13.8G 会被 OOM 杀；减量后二进制约减半、保留行号回溯 |
| `LANG` / `LC_ALL` | `run_bg.sh` 设为 `C.UTF-8` | 保证非 ASCII 输出一致 |
| `PYTHONHASHSEED` | `seed_check.sh` | 双种子确定性检查（1 / 2 各转译一次，生成树必须一致） |
| `XDG_CACHE_HOME` | `jdk_resolver.py` | JDK 解包缓存根目录（缺省 `~/.cache`，缓存在 `<根>/rava/`） |
| `HOMEBREW_PREFIX` | `jdk_select.py` | macOS 自定义 brew 前缀，优先于 `/opt/homebrew`、`/usr/local` 扫描 |
| `PILOT_LIBS` | `lib_pilot_golden.sh` | lib pilot 依赖 jar 目录（缺省 `tests/lib_pilot/deps/target/pilot-libs`，一般不需要设置） |
| `RAVA_GENERATOR` | `generator_select.py`（`main.py` / `run_tests.py`） | 生成器切换期的缺省生成器（`python` / `rust`），`--generator` 显式值优先；项目唯一自有变量，随 Python 生成器删除 |
| `OUT_DIR` | cargo → `build.rs` | cargo 标准变量：`build.rs` 在此生成 `jdk_feature.rs`（语料 JDK 版本常量） |

## 四、常用组合

```bash
# 服务器 / 16G 机器后台跑批（低内存编译环境 + 自动单作业 + 自动放宽超时）
scripts/run_bg.sh j21 python3 scripts/run_tests.py --filter TestCipherDesModes

# JDK 25 全量
python3 scripts/run_tests.py --jdk 25 -j 4

# 严格模式回归（兜底即失败）
python3 scripts/run_tests.py --strict --deny fallback

# 排查运行期异常的抛出位置
RUST_BACKTRACE=full python3 scripts/main.py Foo.java
```

## 五、旧环境变量对照（2026-09-28 删除）

| 旧变量 | 现在的做法 |
|---|---|
| `RAVA_JDK` | `--jdk N`（或 `JAVA_HOME` / `.jdk-version`） |
| `RAVA_BUILD_TIMEOUT` | `run_tests.py --build-timeout SEC`；重型闭包自动 3000 秒 |
| `RAVA_HEAVY_CLASSES` | 阈值固定 1700；需要时显式设 `CARGO_BUILD_JOBS` |
| `RAVA_STRICT` | `--strict` |
| `RAVA_DEBUG` / `RAVA_BFS_EDGE_AUDIT` / `RAVA_CFG_DEBUG` | `--debug` |
| `RAVA_BFS_TRACE` | `--trace-class CLASS` |
| `RAVA_RAW_SITES` | `--raw-sites FILE` |
| `RAVA_THROW_BT` / `RAVA_UNCAUGHT_BT` | `RUST_BACKTRACE=full` / `RUST_BACKTRACE=1` |
| `RAVA_JDK_FEATURE` | `build.rs` 在 `OUT_DIR` 生成 `jdk_feature.rs` 常量 |
| `RAVA_M3_ASSERT` / `RAVA_M3_AUDIT` | 删除：M3 纯位类型断言常开，差分审计已完成 |
| `RAVA_MT` | 2026-09-27 起已无作用（并行后端唯一），删除 |
| `JAVA_RTA_*` | 项目改名为 rava 时统一改名，随后全部删除 |

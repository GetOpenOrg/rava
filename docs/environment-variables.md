# 命令行选项与环境变量

> rava **不设项目自有环境变量**（2026-09-28 清理：原 `RAVA_*` 共 14 个全部改为命令行参数、生成文件或标准变量）。
> 新增开关一律做成命令行参数；需要传给 cargo 构建的内部数据写入 scratch 的生成文件，由 `build.rs` 读取。

## 一、命令行选项

### `scripts/main.py`（转译 + 运行单个程序）

| 选项 | 用途 |
|---|---|
| `--jdk N` | 指定 JDK 主版本。缺省按 `JAVA_HOME` → 仓库根 `.jdk-version`（当前 21）→ 已安装最新版 |
| `--no-run` / `--clean` / `--out DIR` | 只生成不编译运行 / 清空 scratch 重建 / 指定 scratch 目录 |
| `--debug` | 诊断明细：兜底点 traceback 与逐条触发、未解析调用、BFS 迟到 static 边、cfg 结构化逐块判定 |
| `--strict` | 严格模式：转译兜底改为硬失败；缺手写实现的 native 方法编译报错（写入 scratch 的 `java_runtime/strict.txt`，`build.rs` 读取） |
| `--trace-class CLASS` | 打印该类（斜线形态，如 `java/net/InetAddress`）各方法进入调用链的路径，回答“为什么被拉进闭包” |
| `--raw-sites FILE` | Raw 逃生舱构造位点剖面追加写入 FILE（FS-Q1 热点排序），不影响生成代码 |

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

**动态对照**（闭包计划 C5，`scripts/dyn_compare.py`）：每个测试转译成功后（cargo 之前，`--no-run` 下同样执行）
用 JVMTI agent + `-Xlog:class+load,class+init` 跑一次原始 Java 程序，把 JVM 实际加载的类与 `closure.json` 对照。
结果行附 `dyn miss N / extra M prov P%`（翻译域漏覆盖 / 静态多出 / 多出类 provenance 覆盖率；有无调用栈的加载时再附
`unattr K`），明细落盘 `build/jdk<N>/logs/dyn/<test>.json`，汇总行 `[dyn-compare]` 列出全部漏覆盖。
每测试一次 java 运行（实测约 0.1 秒，相对 5–70 秒的转译可忽略），故缺省开启。agent 首次使用时以 `$CC`（缺省 `cc`）
编译，按源码 + JDK 缓存于 `build/dyn_agent/`。单独运行：`python3 scripts/dyn_compare.py build/jdk21/<test> [-o out.json]`。

启动时的 `[meta]` 行打印 `PYTHONHASHSEED`、`CARGO_INCREMENTAL`、`CARGO_BUILD_JOBS` 与透传选项，便于事后解读结果。

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

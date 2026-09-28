# 环境变量说明

> 本文列出 rava 转译 / 编译 / 运行 / 测试各环节读取的全部环境变量。新增或删除环境变量时同步更新本文。
> 所有项目自有变量统一以 `RAVA_` 为前缀。

## 一、速查表

| 变量 | 作用环节 | 取值 | 缺省 | 用途 |
|---|---|---|---|---|
| `RAVA_JDK` | 转译 / 测试 | JDK 主版本号（如 `21`、`25`） | 未设 | 选择所用 JDK |
| `RAVA_BUILD_TIMEOUT` | 测试（`run_tests.py`） | 秒 | `600` | 单测试 cargo 构建超时 |
| `RAVA_HEAVY_CLASSES` | 测试 / `main.py` 编译 | 生成类数 | `1700` | 重型闭包自动单作业编译的阈值 |
| `RAVA_STRICT` | 转译 + 运行时 crate 构建 | `1` | 未设 | 严格模式：兜底路径改为硬失败 |
| `RAVA_DEBUG` | 转译 | 任意非空值 | 未设 | 输出兜底 / 未解析调用的逐条明细 |
| `RAVA_BFS_TRACE` | 转译 | 类 binary name（斜线形态） | 未设 | 打印该类方法进入调用链的路径 |
| `RAVA_BFS_EDGE_AUDIT` | 转译 | 任意非空值 | 未设 | 打印迟到的 static 调用边 |
| `RAVA_CFG_DEBUG` | 转译 | 任意非空值 | 未设 | 打印控制流结构化的判定过程 |
| `RAVA_RAW_SITES` | 转译 | 输出文件路径 | 未设 | Raw 逃生舱构造位点剖面 |
| `RAVA_M3_ASSERT` | 宏展开（cargo 构建） | `off` | 常开 | 关闭纯位类型一致性断言 |
| `RAVA_M3_AUDIT` | 宏展开（cargo 构建） | 输出文件路径 | 未设 | 纯位类型差分审计文件 |
| `RAVA_THROW_BT` | 生成程序运行时 | 任意值 | 未设 | VM 抛异常处打印 Rust 回溯 |
| `RAVA_UNCAUGHT_BT` | 生成程序运行时 | 任意值 | 未设 | 未捕获异常退出时打印 Rust 回溯 |
| `RAVA_JDK_FEATURE` | 运行时 crate 编译期（内部） | — | — | 由 build.rs 设置，**不要手动设置** |

标准 / 第三方变量（非项目自有，但 rava 会读取或设置）见第六节。

## 二、JDK 选择与构建控制

### `RAVA_JDK`

选择转译所用的 JDK。javac、java 以及 JDK 类库语料全部取自同一个 JDK，保证版本同源。
读取方：`scripts/jdk_select.py`（被 `main.py`、`run_tests.py`、`lib_pilot_golden.sh` 共用）。

选择优先级（高 → 低）：

1. 命令行 `--jdk N`
2. `RAVA_JDK=N`
3. 已设置且有效的 `JAVA_HOME`
4. 仓库根 `.jdk-version` 固定的默认版本（当前 21）
5. 已安装的最新版

```bash
RAVA_JDK=25 python3 scripts/run_tests.py --filter BubbleSort
```

### `RAVA_BUILD_TIMEOUT`

`run_tests.py` 中单个测试的 `cargo build` 超时（秒），缺省 600。低内存、单作业编译大闭包时需调大。

```bash
RAVA_BUILD_TIMEOUT=3000 python3 scripts/run_tests.py --filter TestCipherDesModes
```

### `RAVA_HEAVY_CLASSES`

重型闭包判定阈值（`scripts/cargo_env.py`）。工作区内带生成标记的类文件数 ≥ 阈值、且调用方没有显式设置
`CARGO_BUILD_JOBS` 时，自动加 `CARGO_BUILD_JOBS=1`，并打印 `[cargo-env]` 一行。
原因：单个 rustc 峰值约 14G，16G 机器上约 1750 类的闭包在双作业下会被 OOM 杀。
内存充足的机器可以调高阈值，或直接显式设置 `CARGO_BUILD_JOBS` 覆盖自动判定。

```bash
RAVA_HEAVY_CLASSES=99999 python3 scripts/run_tests.py      # 关闭自动单作业
CARGO_BUILD_JOBS=4 python3 scripts/run_tests.py            # 显式指定，自动判定不生效
```

### `RAVA_STRICT`

严格模式，取值 `1` 时生效，作用于两处：

- **转译期**（`codegen/fallback_audit.py`）：转译器各兜底点不再把失败方法降级为 `panic!("stub: ..")`
  存根，异常直接穿透、转译失败；变量提升等处的渲染失败也改为硬失败。
- **运行时 crate 构建期**（`runtime/java_runtime/build.rs`）：调用链上仍缺手写实现的 native 方法
  由 cargo warning 升级为 cargo error。

用于确认某次改动没有引入新的静默兜底。

```bash
RAVA_STRICT=1 python3 scripts/main.py tests/e2e/01_basics/BubbleSort.java
```

## 三、转译期诊断开关

这几项只影响诊断输出，不改变生成代码。

### `RAVA_DEBUG`

设为任意非空值时：

- 打印每个 cfg 兜底点的方法与原因（`[cfg-audit] stub fallback ...`）；
- 打印全部未解析调用（`[bfs-audit] unresolved: ...`）；
- 兜底审计的九个兜底点全部打印 traceback，并逐条输出 B 组触发明细（缺省只给计数和前 20 条警告）。

```bash
RAVA_DEBUG=1 python3 scripts/main.py Foo.java --no-run
```

### `RAVA_BFS_TRACE`

值为类的 binary name（斜线形态）。转译结束时，对该类每个进入调用链的方法打印入链路径，
回答“这个类 / 方法为什么被拉进闭包”。

```bash
RAVA_BFS_TRACE=java/net/InetAddress python3 scripts/main.py Foo.java --no-run
```

### `RAVA_BFS_EDGE_AUDIT`

设为非空值时，打印 BFS 后期补入的 static 调用边（`[bfs-audit] late-static-edge: ...`），
用于排查调用链遗漏。

### `RAVA_CFG_DEBUG`

设为非空值时，打印控制流结构化（try 区域 / 循环嵌套判定）的逐块判定过程（`[cfg-dbg] ...`），
用于排查 `CfgError`。

### `RAVA_RAW_SITES`

值为输出文件路径。按构造调用位点（文件:行:函数）累计 RawExpr / RawStmt 的构造次数，进程退出时写入该文件。
用于 FS-Q1（Raw 逃生舱收敛）按热点排序，缺省关闭，不影响生成代码。

```bash
RAVA_RAW_SITES=/tmp/raw_sites.txt python3 scripts/main.py tests/e2e/04_collections/TestArrayList.java --no-run
```

## 四、宏展开期（cargo 构建时）

由 proc-macro crate `rava_macros` 在展开 `java_class!` 时读取，需要在执行 `cargo build` / `cargo run`
的环境中设置。

### `RAVA_M3_ASSERT`

纯位类型一致性断言：生成器发射的签名类型与描述符推导的类型不一致时报 `compile_error!`，缺省常开。
设为 `off` 可临时关闭，仅作应急手段。

### `RAVA_M3_AUDIT`

值为输出文件路径。展开期对每个被审计方法追加一行 `OK|MISMATCH <类> <方法><描述符> …`，
与断言相互独立。未设置时对生成产物没有任何影响。

## 五、生成程序运行时

由生成的二进制在运行时读取，用于定位翻译体里的抛出点。

### `RAVA_THROW_BT`

设置后，每个 VM 抛异常点（NPE、数组越界、ClassCastException 等）都向 stderr 打印 `[vm-throw]`
和 Rust 回溯。输出量大，建议配合单个用例使用。

### `RAVA_UNCAUGHT_BT`

设置后，main 线程因未捕获异常退出时，除 Java 格式的异常栈外，额外打印 Rust 回溯。

```bash
RAVA_UNCAUGHT_BT=1 build/target/debug/test_foo
```

### `RAVA_JDK_FEATURE`（内部）

生成器把语料 JDK 版本写入 `java_runtime/jdk_feature.txt`，`build.rs` 将其转为编译期环境变量，
手写层经 `crate::jdk_feature()` 读取（缺省 21），同时设置 `cfg(jdk_ge_25)`。**不要手动设置。**

## 六、标准 / 第三方变量

| 变量 | 读取 / 设置方 | 说明 |
|---|---|---|
| `JAVA_HOME` | `jdk_select.py`、`jdk_resolver.py`、各脚本 | JDK 位置。选中 JDK 后由脚本写回，javac / java / jmods 均经它取得 |
| `HOMEBREW_PREFIX` | `jdk_select.py` | macOS 自定义 brew 前缀，优先于 `/opt/homebrew`、`/usr/local` 扫描 |
| `XDG_CACHE_HOME` | `jdk_resolver.py` | JDK 解包缓存根目录（缺省 `~/.cache`，缓存在 `<根>/rava/`） |
| `CARGO_TARGET_DIR` | 脚本自动设置 | 共享编译缓存目录（`run_tests.py` 为 `build/jdk<N>/target`，`main.py` 为 `build/target`），一般无需手动设置 |
| `CARGO_INCREMENTAL` | 脚本自动设为 `0` | 关闭增量编译：宽闭包下增量元数据是 OOM 的主要诱因 |
| `CARGO_BUILD_JOBS` | 用户 / `cargo_env.py` | rustc 并行作业数。显式设置时优先于重型闭包自动判定 |
| `CARGO_PROFILE_DEV_DEBUG` | `run_bg.sh` 设为 `line-tables-only` | 减少调试信息（二进制约减半，rustc 内存下降） |
| `PYTHONHASHSEED` | `seed_check.sh` | 双种子确定性检查（1 / 2 各转译一次，生成树必须一致） |
| `LANG` / `LC_ALL` | `run_bg.sh` 设为 `C.UTF-8` | 保证非 ASCII 输出一致 |
| `PILOT_LIBS` | `lib_pilot_golden.sh` | lib pilot 依赖 jar 目录（缺省 `tests/lib_pilot/deps/target/pilot-libs`） |

`run_tests.py` 启动时的 `[meta]` 行会打印 `PYTHONHASHSEED`、`CARGO_INCREMENTAL`、`CARGO_BUILD_JOBS`、
`RAVA_DEBUG`、`RAVA_STRICT`、`RAVA_BFS_EDGE_AUDIT` 的当前值，便于事后解读跑批结果。

## 七、常用组合

```bash
# 服务器 / 16G 机器跑重型用例（后台 + 低内存编译 + 放宽构建超时）
RAVA_BUILD_TIMEOUT=3000 scripts/run_bg.sh j21 python3 scripts/run_tests.py --filter TestCipherDesModes

# 指定 JDK 25 全量
RAVA_JDK=25 python3 scripts/run_tests.py -j 4

# 排查“某类为什么进了闭包”
RAVA_BFS_TRACE=java/security/Provider python3 scripts/main.py Foo.java --no-run

# 排查运行期异常的抛出位置
RAVA_UNCAUGHT_BT=1 RAVA_THROW_BT=1 python3 scripts/main.py Foo.java
```

## 八、已删除的变量

| 变量 | 删除时间 | 说明 |
|---|---|---|
| `RAVA_MT`（原 `JAVA_RTA_MT`） | 2026-09-27（#42） | 并行后端成为唯一后端，开关已无作用 |
| `JAVA_RTA_*` 全部 | 2026-09-28 | 项目改名，统一改为 `RAVA_*`，不保留旧名兼容 |
| `JAVA_RTA_G5_AUDIT` 等双算审计开关 | 2026-09-28 | N4 G5 切换完成后随审计插桩删除（`c4472b0`） |

# 参考 JDK 21：语料两侧（expected 生成与转译）同源化

> 日期：2026-10-03。来源：e2enew-da8abee1 归因报告中 TestLocaleCurrency 的 CLDR 漂移
> （本机 Homebrew 21.0.11 = `¥`，服务器 Ubuntu apt openjdk-21 21.0.12.1 = `CN¥`）。
> 用户裁定：**合法测试不改**，终态为「语料全部环节使用同一个 JDK 构建」。
> 本文为落地后的最终方案（初稿 171ce833 的「最新 GA + 再生成」已按用户 2026-10-03 决定改为下述固定版本）。

## 一、问题与终态

CLDR / localedata / tzdata 等随 JDK 小版本（update release）漂移。expected 由本机 JDK 生成，转译语料 jmod 与
golden JVM 取自各服务器的系统 JDK（apt 自动升级，持续漂移），两个构建不同即产生伪差异。

**终态**：语料（e2e 全集、抽查、作业、expected 生成）在所有机器上使用同一个固定 JDK 构建——
**Eclipse Temurin `jdk-21.0.11+10`**。javac / java / jmods / golden JVM / 动态对照全部同源。

- 选 21.0.11：它是现有 `tests/expected` 的生成版本，固定在这个版本，现有 golden 天然有效，**expected 改动量为 0**
  （本机实测：Temurin 21.0.11 跑 TestLocaleCurrency 与现有 expected 逐字节一致）。
- **实测更正（2026-10-03 抽查 jdkpin-efc13cd0）**：TestLocaleCurrency 的 `CN¥` 并非 JDK 漂移。sg2 上参考构建
  Temurin 21.0.11 与系统 apt 21.0.12.1 的 **JVM** 均输出 `¥`（`LANG=C.UTF-8`）；在参考构建下，转译产物仍输出 `CN¥` /
  `CN¥ 9,999.50`（`[meta]` 显示 `jdk=jdk-21.0.11+10(参考构建)`）。因此这是转译侧的行为差异（Linux 服务器上
  转译程序的 CNY 本地化符号取值与 JVM 不一致），归生成器 / 闭包修复，不归本方案。参考构建的价值不变：
  语料各环节同源，排除 JDK 构建这一变量。
- 不选「最新 GA」：换版本后只再生成 65_locale_data / 64_charsets_ext 不可靠——其他目录未验证，可能藏着漂移；
  要可靠就得全量再生成，代价大、收益为 0。

## 二、机制（零 rava 改动、零系统配置改动）

不动 `resolve::jdk`（rava 面向开发者的 JDK 选择与回落链保持现状）、不改系统安装的 JDK——**在语料编排层注入 JAVA_HOME**。

### 1. 固定信息入库：`tools/refjdk.toml`

被跟踪的清单登记 tag、版本与每个平台的下载 URL + sha256（`linux-x64` / `linux-aarch64` / `macos-aarch64` /
`macos-x64`，来源 Adoptium API `/v3/assets/release_name/eclipse/jdk-21.0.11+10`，与 GitHub release 的
`.sha256.txt` 核对一致）。清单在仓库里，任何一侧都可复现、可核对；生成器 crate 不读它。

### 2. 取包与定位：`scripts/fetch_reference_jdk.sh`

参考构建的定位只有这一处实现：

- 缺省：确保当前平台（`uname` 判定）的参考 JDK 就位——下载、校验 sha256、解压、落位，幂等；stdout 打印 JAVA_HOME；
- `--check`：只检查不下载，未就位退出码 1 并提示取包命令；`--tag` 打印清单 tag；
- 落位 `<JDK 根>/<tag>/` 即 JAVA_HOME（macOS 包的 `Contents/Home` 提升为该目录），`.rava-refjdk` 印记记录
  tag / 平台 / sha256，与清单不符（换版本 / 换包 / 别的平台）即视为未就位；
- JDK 根：`--root` > `RAVA_REFJDK_ROOT` > 主检出的 `tools/refjdk/`（git worktree 共用主检出那一份；
  `.gitignore` 登记 `/tools/refjdk/`，根目录另自带 `*` 忽略文件）；
- 并发：同一根目录多进程同时取包以 `<根>/.lock-<tag>` 互斥，安装经临时目录整体改名落位。

### 3. 消费：语料缺省参考构建，不静默回退

- `scripts/run_tests.py`（e2e、`--update-expected`、动态对照）：缺省经 `fetch_reference_jdk.sh --check` 取参考
  构建，以 `rava jdk --json --java-home <home>` 注入（压过环境 `JAVA_HOME`）。**未就位即报错并给出取包命令，不回退系统 JDK。**
  显式 `--jdk N` / `--java-home P` 仍可覆盖，仅供实验，`[jdk]` / `[meta]` 行标记「非参考构建」。`--show-jdk` 干跑打印选择结果。
- 语料 shell 脚本（`gen_trees.sh`、`seed_check.sh`、`profile_closure.sh`、`lib_pilot_golden.sh`、`closure_bench.sh`、
  `emit_bench.sh`）经 `scripts/corpus_jdk.sh` 同口径选择；环境变量 `JDK=N` 为实验覆盖。
- 「回落链保持现状」只适用于 rava 本身面向开发者的 JDK 选择（`rava build` 直接调用）；语料编排层不回退。

### 4. 落位

| 位置 | JDK 根 | 方式 |
|---|---|---|
| 本机 | 主检出 `tools/refjdk/`（gitignore） | `scripts/fetch_reference_jdk.sh` 一次 |
| 服务器（`/data/rava` 检出） | `/data/rava-jdk/` | 分发脚本按服务器配置 `refjdk_root` 传 `RAVA_REFJDK_ROOT` |
| ubuntu（WSL，`/mnt/d/workspace/java_rta`） | `/mnt/d/workspace/rava-jdk/` | 同上，按 remote_dir 所在盘配置 |

服务器侧（server_maintenance `rava/`）：每台服务器在检出（main / 抽查 / 作业）之后幂等执行该检出里的
`scripts/fetch_reference_jdk.sh`（已就位即空操作，不受 `--skip-setup` 影响——清单随被测提交走），
e2e 与作业命令导出 `RAVA_REFJDK_ROOT`；e2e 缺省 JDK（21）不再传 `--jdk`，即用参考构建。只写数据目录，
不动 apt 包、`/usr/lib/jvm`、update-alternatives 与用户 shell 配置。

## 三、边界

- **升级参考 JDK 是一次显式的基线事件**，单独立项：改 `tools/refjdk.toml`（tag / URL / sha256）→ 全量
  `--update-expected` 再生成 → 双跑确定性闸门 → expected 变更一次入库。不做局部目录再生成。
- 双 JDK 目标（21/25）：参考构建只钉死 21 基线；JDK 25 适配轮另按其基线处理（当前 `--jdk 25` 属实验覆盖）。
- `.jdk-version` 语义不变（仍表达"21"，供 rava 直接调用时的主版本固定）。

## 四、执行（2026-10-03 全部由本助手完成，用户侧不做）

| 步骤 | 内容 | 状态 |
|---|---|---|
| 1 | `tools/refjdk.toml` + `scripts/fetch_reference_jdk.sh`（下载 / 校验 / 解压 / 定位）+ 单测 | 完成 |
| 2 | `run_tests.py`（含 expected 生成路径）与语料 shell 脚本切换到参考构建；不重新生成任何 expected（版本未变） | 完成 |
| 3 | server_maintenance：服务器数据目录落位 + 检出后幂等确保 + `RAVA_REFJDK_ROOT` 注入（e2e / 抽查 / 作业） | 完成（`jdk-pin` 分支，待审查合并） |

验证：抽查 jdkpin-efc13cd0 中，kr1 / kr2 / sg1 / sg2 / jp1 / jp2 / us1 落位 `/data/rava-jdk/jdk-21.0.11+10`，
ubuntu 落位 `/mnt/d/workspace/rava-jdk/jdk-21.0.11+10`；kr1 `java -version` = Temurin 21.0.11+10。
HelloWorld、TestDateTimeFormat 通过；TestLocaleCurrency 仍失败，原因见 §一「实测更正」，属转译侧问题。

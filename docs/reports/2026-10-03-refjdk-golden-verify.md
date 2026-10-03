# 参考 JDK 全量 golden 核验（Temurin 21.0.11 vs tests/expected）

> 日期：2026-10-03（任务 #8）。起点 main @ 37b11dfb。
> 方法：`scripts/fetch_reference_jdk.sh` 按仓库清单（tools/refjdk.toml）取 Temurin
> jdk-21.0.11+10；对 e2e 全集 1074 例逐例 javac + java **双跑**（golden 环境：
> DisableIntrinsic + en_US + UTC，cwd=仓库根），与 tests/expected/ 逐字比对；
> 逐例串行、每例经 heavy_lock.py；只跑 JVM 侧，不跑转译。总耗时 1448s + 复测。

## 结论

**Temurin 21.0.11 与生成现有 expected 的 Homebrew 21.0.11 输出一致：构建差异 0 例，
「零重生成」验收通过。**

| 分类 | 例数 | 说明 |
|---|---:|---|
| 一致 | **1070 / 1074** | 含 63_junit 10 例（裸 javac 因缺 junit/hamcrest classpath 失败，带 cp 复测 10/10 逐字一致——非构建差异） |
| 构建差异 | **0** | — |
| 环境依赖 | 1 | TestVmPlatformNatives |
| 不确定（JVM 固有非确定） | 2 | ListMethods、TestSocketLoopbackPair |
| javac-fail（其他） | 1 | TestUnnamedVar（既有基线已知失败例，与本次无关） |

## 不一致清单与定性

| 用例 | 定性 | 证据 |
|---|---|---|
| TestVmPlatformNatives | **环境依赖（平台，非构建）** | expected 含 Linux 痕迹（`libzip.so`），本机 Temurin 稳定输出 `libzip.dylib`；**对照实验：本机 Homebrew 21.0.11 与 Temurin 输出逐字一致**——差异纯来自 expected 的生成平台（Linux 服务器），与参考构建无关。建议列入跨平台不可移植基线（与 CLDR 同类处置思路） |
| ListMethods | **不确定** | 五跑两种输出（`Class.getMethods()` 枚举顺序 JVM 不作保证）——JVM 固有非确定，与构建无关 |
| TestSocketLoopbackPair | **不确定** | 五跑两种输出：半关闭读取的字节取决于 worker/main 线程竞速（`after-half-close` 读到回显尾字节 33 或 EOF -1）——JVM 固有非确定。该例建议后续改写为输出序确定（把 worker 输出收集到 join 后统一打印），归语料维护项 |

## 附注

- **TestLocaleCurrency 背景佐证**：本例本次在一致集内（Temurin 21.0.11 输出 ¥ 与
  expected 一致）——支持 tasks.md 的改判：转译侧在 Linux 输出 CN¥ 是**翻译数据通路
  缺陷**（JVM 侧同机两版本均 ¥），非 JDK 版本漂移；此前分诊报告的 CLDR 归因撤回。
- 复现命令：参照本报告方法段（脚本按 heavy_lock 包裹 `javac + java×2`，golden
  环境复刻 run_tests 的 GOLDEN_JVM_FLAGS 与 _fixed_env）；63_junit 例需带
  pilot-libs 的 junit/hamcrest classpath。

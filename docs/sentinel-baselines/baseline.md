# 16G 哨兵回归集与闭包病理档——基线存档（T11）

- 采集日期：2026-10-14 · 基线版本 **bdb9bad4**（origin/main，batch-1015 合入后）· 机器 dev（120G / 32 核）
- 来源与立项：`docs/reports/2026-10-13-scan32g-authoritative-list.md` §八（T6 权威清单建议——
  "16G tripwire 哨兵例建议进常规回归，作为内存上界监视；超 32G 六库 12 例不进常规回归集，
  单列闭包病理档"）。本目录把该建议落成常设机制：G1（内存主项）每轮优化后重扫水位。
- 统一入口：`scripts/diag/sentinel16g.sh [sentinel|pathology|all] [--full]`；
  采样器 `scripts/diag/sentinel_sampler.py`（机制承 scan32g watchdog，/proc 树求和、cwd 门控
  只盯本 worktree 的 `rava build` 树，JVM golden / dyn 对照 java 不进曲线）。
- 运行输出落 `build/sentinel/<日期>/`；本目录（docs/sentinel-baselines/）为**随仓库存档的基线**，
  由 `scripts/diag/sentinel16g.sh promote [日期]` 从某轮全绿结果提升而来。
- 断点续跑（长任务纪律）：两档 TSV 逐例 append 落盘，中断后重跑同一命令即从缺例续起
  （TSV 已有行自动跳过，汇总行从全表聚合）；env `SENTINEL_OUT` 可覆盖输出目录（自测用）。
  采样器 quiet-grace（e2e 1200s / pilot·病理 900s）须覆盖 heavy_lock 排队与 JVM golden 前置段，
  防止命令未进转译段时误判 no-run、使该例脱离 tripwire。

## 一、常规哨兵档（sentinel，10 例）

名单 = T6 双基线 top 峰值非病理例（"距 16G 护栏最近的健康例"）：引擎若引入内存回归，它们先于
全量越线报警。

| 面 | 例 | 基线峰值（GiB） |
|---|---|---|
| e2e | 52_lang_features/TestSystemStableProps | 7.80 |
| e2e | 71_xml/TestXmlTransform | 8.05 |
| e2e | 71_xml/TestSaxNamespaceCallbacks | 5.72 |
| e2e | 72_http/TestSystemProxySelector | 7.90 |
| e2e | 23_algorithms/DataEncryptionStandard | 5.50 |
| e2e | 23_algorithms/TestAesGcmRound | 5.34 |
| e2e | 23_algorithms/TestCipherDesModes | 5.53 |
| e2e | 23_algorithms/TestCipherAlgorithmParameters | 5.28 |
| e2e | 73_jndi_script/TestJndiNoProvider | 5.62 |
| pilot | HoconParseMain | 5.51 |

首轮采集（2026-10-14，`SENTINEL ALL-GREEN 10/10`）：10/10 完成发射、16G tripwire 零触发、
rc 全 0；与 T6 双基线历史参考值逐一对照差 ±3% 内（详见 `sentinel-baseline.tsv`）。
运行产物 `build/sentinel/2026-10-14/`（sentinel.tsv / curves/ / logs/ / summary.txt）。

**护栏**：逐例转译（e2e 走 `run_tests.py --no-run --filter /<例>`、pilot 走 `lib_pilot_golden.sh
<Main> --emit-only`，均经全机重进程锁 heavy_lock 包装、顺序执行一次一例）；RSS tripwire 16G
（越线 SIGKILL 全树记 FAIL）；`--transpile-timeout 2400`。

**绿转红警报口径**（任一触发即汇总行打 `SENTINEL ALERT`，退出码 1；全绿打 `SENTINEL ALL-GREEN`）：

1. 任一例转译 rc≠0（`FAIL-RC`）；
2. 任一例被 16G tripwire 杀（`FAIL-LIMIT`）或采样器全程未见转译进程（`FAIL-NO-RUN`）；
3. 任一例峰值 > 其基线 × 1.25（`FAIL-REGRESS`）。

无基线的新例记 `BASELINE-SET`，不作警报（首轮采集口径）。

## 二、闭包病理档（pathology，12 例）

名单 = T6 §四超 32G 权威清单（六库 12 例，转译闭包病态单调增长不收敛）：
ognl×3（OgnlCollectionMain/OgnlContextMain/OgnlEvalMain）、snakeyaml×2（YamlRoundTripMain/
YamlTypesMain）、fastjson2×2（FastjsonPathMain/FastjsonStreamMain）、jackson databind×3
（DatabindAnnotationMain/DatabindDeserMain/DatabindSerMain）、SpringProxyJdkMain、
esper 项目面（esper/com.example.esper.EsperLengthWindowDemo）。

**护栏（固定窗口采样，不跑死）**：每例 240 秒窗口 / 16G 上限，RSS 曲线 10s 粒度落 TSV
（`pathology-curves/<例>.tsv`，两列 `t_offset_s` / `rss_mib`）；到窗或越线即 SIGKILL 转译树。
JVM golden 前置段（javac+java 真实 jar）不计入窗口与曲线。

**指标**：速率 rate_mib_s = 曲线最小二乘斜率（MiB/s）；终值 final_gib；曲线指纹 curve_sha256
（TSV 全文哈希，仅作存档指纹——采样粒度内噪声使其不可逐位复现，比对一律走速率）。

**对比规则**：|当前速率 / 基线速率 − 1| > 20% → 该例打 `DRIFT`，汇总行 `PATHOLOGY DRIFT`
（退出码 1）；全部在带内 → `PATHOLOGY STABLE`。采样器全程未见转译进程（`NO-RUN`，依赖缺失/
脚本早退）按运行事故计 drift。注意：**G1 修复后病理例行为改变（增速显著变化、甚至窗口内完成
发射 completed）同样表现为 DRIFT**——这是修复信号而非回归，此时应重跑 `--full` 认证并用新一轮
全绿结果 `promote` 刷新基线。

首轮基线速率（2026-10-14，240s 窗口 / 16G 上限，`PATHOLOGY BASELINE-SET 12/12`）——
12/12 全部 window-kill（240s 窗口内单调增长不出发射，病理不收敛逐一在档）：

| 例 | 速率（MiB/s） | 终值（GiB） | | 例 | 速率（MiB/s） | 终值（GiB） |
|---|---|---|---|---|---|---|
| OgnlCollectionMain | 13.47 | 4.38 | | DatabindAnnotationMain | 11.05 | 3.47 |
| OgnlContextMain | 12.98 | 4.26 | | DatabindDeserMain | 12.61 | 3.81 |
| OgnlEvalMain | 12.80 | 4.18 | | DatabindSerMain | 23.49 | 6.38 |
| YamlRoundTripMain | 6.33 | 2.93 | | SpringProxyJdkMain | 14.21 | 4.38 |
| YamlTypesMain | 16.61 | 4.83 | | esper LengthWindowDemo | 19.26 | 5.59 |
| FastjsonPathMain | 17.77 | 5.01 | | FastjsonStreamMain | 12.23 | 4.27 |

（速率为 240s 窗口最小二乘斜率，与复现卡早期段增速 16–52 MiB/s 同带偏低端——窗口均化了
增速递减段；逐例明细与曲线见 `pathology-baseline.tsv` / `pathology-curves/`。）

**--full 终局验收档**：上限 16G→32G、窗口取消（跑完为止：发射完成 = 认证不超限，越 32G 线杀 =
仍超限）。仅在 G1 修复后使用。

## 三、基线文件

| 文件 | 内容 |
|---|---|
| `sentinel-baseline.tsv` | face · case · 峰值 GiB · rc · outcome · 版本 · 日期（10 行） |
| `pathology-baseline.tsv` | case · rate_mib_s · final_gib · outcome · 版本 · 日期 · window_s（12 行） |
| `sentinel-curves/<例>.tsv` | 哨兵例转译 RSS 曲线（5s 粒度，峰值佐证） |
| `pathology-curves/<例>.tsv` | 病理例 240s 窗口 RSS 曲线（10s 粒度） |

基线刷新纪律：只在"当前基线版本上全绿/全稳"的健康轮提升（`promote`）；版本列记录采集时
HEAD，跨引擎版本对比时先看该列。峰值对比规则带 ±25% 容差、速率 ±20% 容差，均为吸收采样
粒度（5s/10s）与运行噪声的经验带——T6 双基线实测同例峰值涨落在 ±5% 内（Hocon 7.42→5.62G
属引擎改进），top 例一致下降，未见内存上漂。

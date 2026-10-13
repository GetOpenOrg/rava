# 超 32G 用例权威清单（全语料转译 32G 达标扫描）

- 日期：2026-10-13 起（跨日完成） · 分支 `libtests-scan32g` · worktree `/data/workspace/rava-wt/scan32g`
- 范围：JDK21 参考构建（jdk-21.0.11+10）缺省口径转译段（`rava build --stop-after emit`，含 `--train-predefined`）。
  JDK25 语料不属本扫（j25 线自管）；junit 族切片（m1–m5/sc/sb/jc）已另行剖面，不在本扫。
- 双基线：预扫基线 **9268501e**（origin/main 当时的 HEAD）全量推进；中途按协调方 T6 纠偏重锚
  **bdb9bad4**（batch-1015 合入后 main），决定性子集复测。本清单的权威口径 = bdb9bad4。

## 一、结论（TL;DR）

| 项 | 数字 |
|---|---|
| e2e 总例数 | 1144（全部有 expected，无 SKIP） |
| e2e 双基线覆盖 | 1144/1144（预扫 975 + bdb 393，重叠 224，并集 1144，未覆盖 0） |
| pilot 总例数 | 47 单文件（另有 esper 项目面） |
| pilot 实测 | 36（11 例已知 >32G 免测引证）+ bdb 复测样本 10 |
| **超 32G 用例总数** | **12 例**——全部为病理报告已知户（六库），**新发现超限户 = 0** |
| 16G tripwire 触发 | 0 次（两基线、全部 1368 例次转译） |
| 非免测例最高 RSS 峰值 | 8.19 GiB（71_xml/TestXmlTransform，bdb）——32G 限值的 1/4 |
| 32G 逐例认证段 | 零候选，未触发 |
| 顺带发现（非内存） | ① 预扫 2 例折叠点自检违约在 bdb9bad4 已修复；② batch-1015 转译时长 +28%（内存持平）——移交引擎线；③ KotlinStdlibPilotMain 标识符 bug 两基线均在 |

## 二、口径与机制

### 2.1 内部限流机制发现

`__RAVA_OOM_KILL__` 标记定义于 `scripts/cluster/dist_remote.py`（`OOM_MARK`）：**远端集群专用**的
cgroup v2 限流——`systemd-run --user --scope -p MemoryMax=<N>M -p MemorySwapMax=0` 包执行，事后读
scope 的 `memory.events`（oom_kill 计数）与 `memory.peak`，越限打 `__RAVA_OOM_KILL__ limit=…M peak=…M` 行。
它不是 generator 内部开关，也没有可被 run_tests.py 转译子进程继承的环境变量（RAVA_LIM/RAVA_TOTAL
是包装 shell 局部量）。本地扫描按任务预案退回**监控杀法**，并按协调方 T6 改单实现两段式。

### 2.2 两段式 RSS tripwire（本轮实施形态）

- watchdog（5s 粒度轮询 /proc）：识别 cwd 位于本 worktree 的全部 `rava build` 进程树（根 + 子进程
  RSS 求和，含训练 java 子进程），逐例记录峰值/时长（JSONL 事件流，断点续跑安全）。
  - 第一段（全量扫描）：RSS 越 **16G** SIGKILL 全树 → `gt16g-candidate`；机器占用 ≈ 单例 ≤16G。
  - 第二段（认证）：候选例逐例单独跑、上限 **32G**：越线杀 → 确认超限；完成发射 → 不超限（记峰值）。
  - 只作用于本 worktree（cwd 门控），不波及主检出与其他 worktree。
- 转译超时护栏：e2e `--transpile-timeout 2400`；pilot 外层 `timeout 3000`。
- 执行序（重尾优先）：e2e 重尾 85（63_junit 全部 + 各目录体量最大例 + 64_charsets_ext 全部）→
  47 pilot 中 36 例 → e2e 轻尾 12 批（--batch K/12 发现序切片）。
- 进度逐例落盘：`/tmp/scan32g-progress.tsv`（base/face/case/rc/peak_rss_gib/dur_s/outcome），
  预扫全表存档 `/tmp/scan32g/prescan-9268501e.tsv`。

### 2.3 双基线方法（T6 纠偏）

1. 预扫（9268501e）：重尾 85 + 36 pilot + 轻尾 batch 1–10（至 batch10 第 82/95 例按指令中断）。
2. 重锚：`git reset --hard origin/main`（bdb9bad4）→ 重建 driver → **closure_cache 挪存**（`.prescan`）、
   全部新跑走全新 out-dir `build/scan32g-bdb`——与预扫同冷启动口径，杜绝暖缓存假平价。
3. 决定性子集（bdb9bad4）：重尾全量 85 + 每 12 例抽样 67 + batch10–12 全量 287 + 补钉 6
   （预扫 top 峰值例中子集未覆盖者 + 2 个预扫错误例复查）+ pilot 样本 10（含最高峰值 HoconParseMain）。
4. 平价判定（§五）：**内存口径**峰值中位 −3.0%（±10% 内）→ 预扫全表按内存口径有效、基线重锚 bdb9bad4；
   时长口径超 ±10%（§七顺带发现），不推翻内存结论（见 §五论证）。

## 三、总表

### 3.1 e2e 面（tests/e2e 全量 1144 例）

| 基线 | 覆盖 | 转译 OK | 转译错误 | 说明 |
|---|---|---|---|---|
| 预扫 9268501e | 975 | 973 | 2 | TestListOf、TestCollectionsFactories：folds 自检违约（折叠点不一致，秒级失败非内存）——**bdb 复测均 PASS，batch-1015 已修** |
| bdb9bad4 | 393 | 393 | 0 | 决定性子集 + 补钉，含全部重尾与预扫 top 峰值例 |
| 并集 | **1144** | — | — | 未覆盖 0 |

- 16G 触发：**0**。峰值分布：预扫 839 例有峰值数据（>4G 共 7 例，除 1 例被中断的部分运行 10.37G 外
  最高 7.62G）；bdb 392 例（>4G 共 22 例——xml/http 目录吃重，最高 8.19G）。
- bdb top 峰值（全部完成发射，单位 GiB）：TestXmlTransform 8.19、TestSystemProxySelector 7.85、
  TestSystemStableProps 7.82、TestDomResultNode 6.63、TestXmlXPath 6.46、TestXmlDomParse 6.31、
  TestXmlFactoryConfigs 6.27、TestHttpLoopbackAsync 6.20。

### 3.2 pilot 面（47 单文件 pilot + esper 项目面）

| 结果 | 数 | 例 |
|---|---|---|
| 转译通过（预扫） | 35 | 36 实测中除 KotlinStdlibPilotMain 外全部；AsmCommonsMain 为语料缺陷（§四.2，JVM 侧 golden 硬读缺失文件中止）后**转译面单独直跑 PASS**（峰值 1.53G） |
| 转译失败（两基线一致） | 1 | KotlinStdlibPilotMain：`kotlin/collections/CollectionsKt___CollectionsKt.sorted` 方法体生成"非法 Rust 标识符 `_i_a_-apply-…`"（Kotlin 名字修饰带 `-`；峰值 0.74G，非内存） |
| 已知 >32G 免测 | 11 | §四 |
| bdb 复测样本 | 10 | 9 PASS + KotlinStdlib（同错）；HoconParseMain 7.42→5.62G，其余 ±0.2G 内 |

logback×2（E0624 编译死）非内存已知户：实测转译段通过（LogbackConfigMain 0.40G/5s、
LogbackPatternMain 1.36G），按口径记 transpile-pass，**不算超 32G**。

## 四、超 32G 权威清单（六库 12 例，全部免测引证）

依据：`docs/reports/2026-10-12-lib-tests-merge-verification.md` §二/§五.1（原始长跑：单核 25–82 分钟、
RSS 26.8–66G 不出发射即被杀）+ `docs/reports/2026-10-13-closure-pathology-repro.md`（六库复现卡：
720s 时限 + 16–20G RSS 护栏下的受控复现、增速与栈签名）。两报告共同证明：这些例在受控上限内
**单调增长不收敛**（增速 16.3–52 MiB/s 或 1.0–1.5 GiB/min），给定时间必越 32G——认证方式为引证，
本轮未重跑（T6 指令 ②）。

| # | 例 | 库（类数） | 复现卡实测（kill/RSS/增速） | 病理帧 |
|---|---|---|---|---|
| 1 | OgnlCollectionMain | ognl 3.4.14 + javassist（168） | 750s 时限 / 14.47G / 16.3 MiB/s 单调 | `is_subset←flow::flow←link_hub_in` |
| 2 | OgnlContextMain | 同上 | 同族（merge-verification：37min 26.8G） | 同上 |
| 3 | OgnlEvalMain | 同上 | 同族 | 同上 |
| 4 | YamlRoundTripMain | snakeyaml 2.7（230） | 749s / 18.76G / 前期 52 MiB/s | `pstr_edge` insert←`hub_bind`；`is_subset←add_to_id` |
| 5 | YamlTypesMain | 同上 | 同族（51min 58.1G） | 同上 |
| 6 | FastjsonPathMain | fastjson2 2.0.65（772） | 559s（available 护栏）/ 11.30G / 18.0 MiB/s | `worklist::run` 自旋（memset 内联） |
| 7 | FastjsonStreamMain | 同上 | 同族（82min 66G） | 同上 |
| 8 | DatabindAnnotationMain | jackson 三件（2.22.3） | 729s / 14.11G / ~1.0 GiB/min 线性 | `is_subset←add_to_id←run`；`node_tau` |
| 9 | DatabindDeserMain | 同上 | 同族（37min 36.3G） | 同上 |
| 10 | DatabindSerMain | 同上 | 同族 | 同上 |
| 11 | SpringProxyJdkMain | spring-aop/core/jcl 6.2.19（1503） | 744s / 16.49G / 前期 1.4–1.5 GiB/min | `rcall_dispatch` memmove←`run` |
| 12 | EsperLengthWindowDemo（esper 项目面，多文件模式，非 47 单文件集） | esper 7.1.0（5455，六例最大） | 749s / 14.70G / 31.5→12.2 MiB/s 递减仍不收敛 | 流集层+hub 层混合档（与 yaml/databind 同型） |

记账脱钩共性：闭包自报峰值 815–2518 MiB vs 实际 RSS 11.3–18.8 GiB（esper 14.6 倍）——失控分配在
自报统计之后的 flow/worklist 循环（复现卡 §三）。

## 五、第二段认证与平价判定

- **>16G 候选：0**（两基线全部例次零触发）→ 32G 逐例认证段未触发；超限认定全部走 §四引证。
- 平价（224 双基线例 + pilot 样本 10）：
  - **峰值 RSS：中位 −3.0%，目录级涨跌互现（小例亚 GiB 峰值受 5s 采样粒度影响），top 例一致下降**
    （Sax 7.62→5.87、Jndi 7.09→5.70、Hocon 7.42→5.62、SystemStableProps 完成 7.82）。
    无任何目录出现内存上漂 → 预扫表按内存口径在 bdb9bad4 上有效。
  - 时长：≥20s 例比例差中位 **+27.7%**（P90 +38.1%），目录级普遍 +26–38%（batch-1015 的转译耗时
    回归，§七②）。**不推翻内存结论的论证**：转译时长与 RSS 峰值在本扫描内不相关（时长涨的例峰值
    反降）；32G 达标是内存命题，峰值平价即证据有效；时长回归单独立项移交引擎线，不在本清单口径内。

## 六、与 closure-pathology 报告六库对照

| 病理报告结论 | 本扫描对照 |
|---|---|
| 六库 12 例转译闭包病态（>32G） | 全部在本清单 §四，无遗漏、无新增 |
| "5 库 11 例"（merge-verification）+ esper 第六例（复现卡，已归引擎线） | 一致：11 单文件 + esper 项目面 |
| 记账脱钩（自报 vs 实际 RSS 数倍） | 本扫描非病理例无双例复现（top 例自报与 RSS 关系未测——不在本扫口径） |
| logback×2 编译死（E0624）非内存户 | 一致：转译段通过（0.40G/1.36G），不算超 32G |
| 病理纯在闭包引擎、与被译库无关 | 本扫描新增旁证：47 pilot 其余 36 例 + e2e 1144 例（含重反射/序列化/xml/jndi 面）无一越 16G |

## 七、顺带发现（非内存，移交对应线）

1. **折叠点自检违约 2 例已修**：TestListOf、TestCollectionsFactories（13_collections_advanced /
   44_collection_api）在 9268501e 上 `folds 自检违约 …活指令 pc 落入 dead_pcs`（秒级失败）；
   bdb9bad4 复测均 PASS（2.24s / 5.22s）。batch-1015 的引擎/发射改动顺带根治，可从 known_failures
   核对出列（若在列）。
2. **batch-1015 转译时长回归**：≥20s 例中位 +27.7%（P90 +38.1%），小例固定增量约 +1.2–1.5s；
   最大例（30s 级）+6–11%。内存峰值持平或降。建议引擎线对 batch-1015 引入的转译段耗时做一次
   归因（候选：方法 trait 导入路径统一后的闭包扩张）。明细 `/tmp/scan32g/parity.tsv`（224 例）。
3. **KotlinStdlibPilotMain 发射 bug**（两基线一致）：Kotlin 名字修饰 `_i_a_-apply-…` 含 `-`，
   生成非法 Rust 标识符（`kotlin/collections/CollectionsKt___CollectionsKt.sorted`）。待标识符
   清洗修复；非内存问题。
4. AsmCommonsMain 语料缺陷复现（§四.2 既有登记）：JVM 侧 golden 硬读上游工程产物
   `target/classes/com/example/cglib/User.class`——自足化漏网，按同法改语料即可。

## 八、G1 验收口径建议（回归集收录）

1. **超 32G 六库 12 例不进常规回归集**：转译段必须跳过或单列"闭包病理档"（独立限额 + 限时，
   只在引擎线改闭包引擎时跑）。混入常规批会吃满机器内存拖垮整批。
2. **16G tripwire 哨兵例**（建议进常规回归，作为内存上界监视）：预扫/bdb 双基线 top 峰值例——
   TestSystemStableProps（7.8–10.4G）、TestXmlTransform（8.2G）、TestSaxNamespaceCallbacks（5.9–7.6G）、
   TestSystemProxySelector（7.9G）、23_algorithms crypto 四例（7.1–7.4G）、TestJndiNoProvider（5.7–7.1G）、
   HoconParseMain（5.6–7.4G）。这些例是"距 16G 护栏最近的健康例"：引擎若引入内存回归，它们先于
   全量越线报警。
3. 常规 e2e 批建议挂 watchdog 口径：-j 1 单列 + 16G tripwire（本轮验证 1368 例次零误杀）；
   重反射/序列化新库入库前先按"重尾 85"名单试跑。
4. 71_xml 目录转译最重（5–17 分钟/例、5–8G）：CI 抽查此目录时预留时长预算。

## 九、工件与复现

- 进度表：`/tmp/scan32g-progress.tsv`（bdb，带 base 列）与 `/tmp/scan32g/prescan-9268501e.tsv`（预扫存档）
- watchdog 事件流：`/tmp/scan32g/watchdog.jsonl`（预扫）`watchdog-bdb.jsonl`；平价明细 `parity.tsv`
- 编排器/watchdog/认证脚本：`/tmp/scan32g/{run_scan.sh,run_scan_bdb.sh,watchdog.py,merge.py,parity.py,cert32g.sh}`
- 复现口径：worktree + `scripts/fetch_pilot_deps.sh --no-scan` + 参考 JDK21（tools/refjdk.toml）+
  `uv run python3 scripts/run_tests.py --no-run -j 1 --transpile-timeout 2400 --out-dir build/<dir>`，
  watchdog 5s 粒度 16G/32G 两档

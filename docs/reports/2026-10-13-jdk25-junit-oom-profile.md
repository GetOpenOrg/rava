# JDK25 junit 大槽案例极限剖面：TestJunitFailureMessages 单分析器受控复现（2026-10-13）

> 来源：JDK25 语料 138 例语义分诊（2026-10-12-jdk25-sem-triage.md §2.1）——63_junit 8 例在
> 22G/40G 槽峰值 39.6–39.9G 仍转译 OOM，名单待大槽重跑。本报告对其中 TestJunitFailureMessages
> 做单分析器受控极限剖面：允许 65G RSS（90 分钟硬顶），看它到底怎么长、能否收敛、长在哪。
> 基线：batch-1014 @22409c66；worktree rava-wt/j25prof（分支 libtests-j25prof）；纯诊断，无代码变更。
> 同格式参考：2026-10-13-closure-pathology-repro.md（五库病理卡）。

## 一、结论摘要

**65G 护栏 kill，43.5 分钟，不收敛。** 单线程（Threads=1 全程）RSS 从 3.1 GiB 单调爬到
65.03 GiB，末段恒定 ~20 MiB/s 无平台；栈采样先后落入五库病理卡的**两档回路**（600s 在
hub 层 `grow_hub→hub_recv_in`，1800s 在流集层 `add_to_id→node_cut`）——同一台发动机、
同一回路，junit@JDK25 是其更重的档位。lambda-prof 副剖面确认早期内存主项是方法上下文
克隆（~40 KiB/个，与 G1 记录一致），后期转为增长枢纽数量线性膨胀，枢纽重放计数本身不是主项。

| 项 | 值 |
|---|---|
| kill 方式 | kill -9（显式 PID），**RSS>65G 护栏**，et=2610s |
| 终值 RSS | 65.03 GiB（available 30G，未触 available<20G / 5400s 护栏） |
| 平均增速 | ~24 MiB/s（Phase A 44 → Phase D 20 MiB/s） |
| 22G 槽外推 | et≈445s（7.4 分钟）即死，死时仍 44 MiB/s 无放缓 |
| 40G 槽外推 | et≈985s（16.4 分钟）即死，与 triage 记录的 40G 槽峰值 39.6–39.9G 吻合 |
| 最深 rava 帧 | stack1：`hub_recv_in←grow_hub`；stack2：`node_cut←flow::add_to_id`（均在 worklist::run 之下） |
| lambda-prof | 已接线（类目录输入），16 段打点曲线见 §五 |

## 二、复现规程

```bash
git worktree add /data/workspace/rava-wt/j25prof -b libtests-j25prof batch-1014 && cd $_
scripts/fetch_pilot_deps.sh --no-scan
python3 /data/workspace/heavy_lock.py cargo build --release -p driver \
  --manifest-path generator/Cargo.toml --target-dir build/analyzer-target
# 主剖面（run_tests 只转译不运行）
python3 /data/workspace/heavy_lock.py uv run python3 scripts/run_tests.py --jdk 25 \
  --filter /TestJunitFailureMessages.java --no-run \
  --transpile-timeout 5400 --out-dir build/j25prof
```

- 分析器展开命令（run_tests 内，PID 331076）：
  `rava build tests/e2e/63_junit/TestJunitFailureMessages.java --stop-after emit --out build/j25prof/jdk25/test_junit_failure_messages --java-home /usr/lib/jvm/java-1.25.0-openjdk-amd64 --train-predefined --deps tests/lib_pilot/deps/target/deps.lock.toml --cp hamcrest,junit --closure-json`
  （junit:junit:4.13.2 + org.hamcrest:hamcrest[-core]:3.0）
- JDK 25 版本：OpenJDK 25.0.4.1+1-1-26.04.4-Ubuntu（`--jdk 25` 解析至 java-1.25.0-openjdk-amd64）
- 分析器档位：release + debuginfo（build/analyzer-target/release/rava）
- 护栏（监控循环 30s 粒度，/tmp/j25prof/monitor.py，只显式 PID kill）：
  ① RSS > 65 GiB；② MemAvailable < 20 G；③ 已过 5400s。**触发①**：et=2610s、RSS 65.03 GiB、
  available 30G（②③未触；期间闸门作业瞬时用内存曾把 available 压到 22G，未破线）。
- run_tests 记账：`[FAIL] transpile error (44m55.9s)`（含 rava 启动前 ~64s 的 javac/JDK 解析）；
  本机 JVM 侧该例正常（triage 已述，病理纯在转译侧）。

## 三、RSS 时间线（/tmp/j25prof/rss.tsv，30s 粒度）

| 阶段 | et（s） | RSS 轨迹 | 特征 |
|---|---|---|---|
| A 快涨 | 0–480 | 3.1 → 23.9 GiB | ~44 MiB/s，越 22G 于 et≈445s |
| B 减速 | 480–960 | → 38.7 GiB | ~27 MiB/s |
| C 锯齿平台 | 960–1380 | 37 ↔ 41.5 GiB | 对应引擎日志两批按对象物化（128 对象/峰值 2025 MiB → 256 对象/峰值 45485 MiB），批间释放-复长 |
| D 稳态单调 | 1380–2610 | 40.7 → 65.0 GiB | ~20 MiB/s 恒定，无收敛迹象 → 护栏杀 |

- 起点 et=0（监控接入时 rava RSS 3.1 GiB）；终点 65.03 GiB；全程平均 ~24 MiB/s；Threads=1。
- **22G 槽对照**：越 22G 在 et≈445s（Phase A 中段，首 7.4 分钟）——22G 槽的 OOM kill 发生在
  最陡段且增速无任何放缓，与「junit 8 例 22G 槽全灭」一致。
- **40G 槽对照**：首次触 39.6–40G 于 et≈985s（Phase C 入口）；triage 记录 40G 槽峰值
  39.6–39.9G 与 Phase C 锯齿带（37–41.5G）吻合——40G 槽死在物化批次锯齿段（~16.4 分钟）。
- **能否收敛 / 何时爆**：Phase D 无平台、无减速拐点；kill 后外推按 20 MiB/s，若把整机余量
  （kill 时 available 30G）全部给它，约 et≈4100s 触顶——90 分钟硬顶内必死，65G 只是护栏不是需求边界。

## 四、栈采样（sudo eu-stack；stack3/4 计划点 et=3600/5100s 未达——进程 2610s 已被杀）

- **stack1**（et=600s，RSS 27.4 GiB，/tmp/j25prof/stack1.txt）最深 rava 帧：
  ```
  #0  closure::engine::hub::Engine::hub_recv_in
  #1  closure::engine::hub::Engine::grow_hub
  #2  closure::engine::invoke::Engine::invoke_inner → invoke
  #4  closure::engine::bytecode::Engine::event_inner → event → process_bytecode
  #7  closure::engine::worklist::Engine::process → run
  #9  closure::analyze → rava::closure_run::analyze → rava::build_cmd::build_stages → main
  ```
- **stack2**（et=1800s，RSS ≈48 GiB，/tmp/j25prof/stack2.txt）最深 rava 帧：
  ```
  #0  closure::engine::flow::Engine::node_cut
  #1  closure::engine::flow::Engine::add_to_id
  #2  closure::engine::worklist::Engine::run
  #3  closure::analyze → rava::closure_run::analyze → build_stages → main
  ```
- **与五库病理卡对照（同一回路）**：入口链逐帧一致（`worklist::run` 主回路）；档位覆盖其两档——
  边/hub 层（`worklist→bytecode::event→invoke→grow_hub`，五库 snakeyaml 的 `pstr_edge`/hub 档）
  与流集层（`worklist→flow::add_to_id`，ognl/databind 的 `is_subset`/`node_grown` 同族，本例
  内层为 `node_cut`）。junit 例特有：**一例内先后经历两档**（600s hub 层 → 1800s 流集层），
  说明随图长大工作热点从枢纽增长迁移到流集运算，与五库「集合膨胀后超线性退化」同一病程。
- 引擎日志同样零进展：全程仅 6 行 `[closure]`，末行「按对象物化：256 个对象，回退后应用
  194 组，峰值 45485 MiB」后直至 kill 静默 20+ 分钟（病理卡共性发现 1 再现）。
- 记账差距：物化段自报峰值 45485 MiB vs kill 时 RSS 65.03 GiB——差距（~1.4 倍）比五库
  （3–5 倍）小，因本例失控在物化段末批已现、且后续 flow/worklist 段继续增长。

## 五、lambda-prof 副剖面（已接线，--lambda-prof 60）

接线方式：`rava closure` 的 .java 输入经 javac 编译时**不带依赖类路径**（closure_cmd.rs
`user_classes` 只 `-g -d`），直接传 .java 报 `package org.junit does not exist`；改为 javac
预编译到类目录再喂 `rava closure`（与 scripts/api_surface_job.sh 同形态）：

```bash
javac -g -d /tmp/j25prof/classes -cp "pilot-libs/junit-4.13.2.jar:hamcrest-core-3.0.jar:hamcrest-3.0.jar" \
  tests/e2e/63_junit/TestJunitFailureMessages.java
python3 /data/workspace/heavy_lock.py build/analyzer-target/release/rava closure /tmp/j25prof/classes \
  --java-home /usr/lib/jvm/java-25-openjdk-amd64 \
  --deps tests/lib_pilot/deps/target/deps.lock.toml --cp hamcrest,junit --lambda-prof 60
# 护栏同主剖面（65G / available<20G / 显式 PID kill -9），硬顶缩至 1800s
```

结果：同护栏 30 分钟顶，**et=1800s 时限杀，RSS 47.59 GiB**（曲线与主剖面吻合：et=600s 时
26.8G vs 主剖面 27.4G，复现稳定）。16 段打点（/tmp/j25prof/lambda-prof.txt，含曲线整理表）：

| et(s) | rss_peak(MiB) | methods(上下文克隆) | hubs | replay_total | recv_total |
|---|---|---|---|---|---|
| 62 | 2523 | 123032 | 9971 | 20741 | 219035 |
| 278 | 12954 | 384986 | 59410 | 62745 | 248539 |
| 618 | 25223 | 431755 | 100997 | 79495 | 269666 |
| 1166 | 41326 | 469347 | 130819 | 109010 | 318805 |
| 1709 | 53044 | 517307 | 160426 | 125908 | 335164 |

读数：

1. **早期（62–376s）方法上下文克隆风暴**：methods 123K→402K（~73K/min），同期 RSS
   2.5→18.5 GiB——**~40 KiB/上下文**，与 G1 线记录的「方法上下文 ~35 KB/个」一致。
2. **中后期主项切换**：methods 减速至 +3–8K/min，但 hubs 线性 +5–8K/min（10K→160K），
   RSS 恒定 ~20 MiB/s——后期内存跟踪的是**增长枢纽的持续复制**而非重放次数。
3. **枢纽重放/接收者展开计数不是体量主项**：replay_total 28 分钟仅 20.7K→125.9K、
   recv_total 219K→335K（增速 ~3–5K/min）——G1 关注的「重放 × 展开计数」在 junit 例中
   已被 s0-g1 系列压住，剩下的爆点是**枢纽与上下文对象的净增长**。
4. 上下文画像：ctxc 头部为 `@level:2`(11171) + `@level:0`(11159) 档位上下文，concrete 仅
   325；ctxm 头部 `Objects.requireNonNull` 9176、`ConcurrentHashMap` 家族 ~5.7K、
   `Integer.valueOf` 3.3K、`MethodType` 家族 ~3.0K（`java.lang.invoke` 分配点，G1 记录同项）。
5. **同一站点跨上下文复制成多个增长枢纽**：`Pattern$CharPredicate.is`（regex
   `BmpCharPropertyGreedy.match@26`/`CharPropertyGreedy.match@64` 等站点）在 `@level:0/2`
   等上下文各克隆一个增长枢纽、各挂 828 lambda——上下文克隆直接放大枢纽数。
6. 末段工作表：`swork=281790 watch=1371876 site_nodes=444659`。

## 六、结论与 G1 基线定位

1. **不收敛**：全程单调（Phase C 锯齿为物化批内释放-复长），末段 20 MiB/s 无平台；
   65G/43.5min 是护栏值而非需求边界，给满整机余量也会在 ~68 分钟内触顶。
2. **外推**：22G 槽死于 et≈445s（7.4 分钟，最陡段）；40G 槽死于 et≈985s（16.4 分钟，
   物化锯齿段，与 triage 峰值记录吻合）。
3. **病理同源**：与五库病理卡同一发动机、同一入口链、同一两档回路（hub 层 + 流集层）；
   junit@JDK25 更重（65G@43.5min；五库在 20G 上限 12–12.5min）。
4. **给 G1 线的一句话基线定位**：TestJunitFailureMessages@JDK25 在 batch-1014 主线上是
   比 S0-A（2971s/34.9G）更重更快的单例——单线程 ~20 MiB/s 单调增长、43.5 分钟 65G 无收敛，
   同时压中 G1 两个已立项主项（`@level` 档位方法上下文克隆、宽接收者增长枢纽跨上下文复制）
   并在后期落入流集层 `add_to_id` 重入；G1 修复验收应把本例纳入回归集（当前 22G/40G 槽
   分别在 7.4/16.4 分钟死亡，任何槽位 <90 分钟都过不去）。

## 附录：产物清单（/tmp/j25prof/，原始数据）

- rss.tsv（主+副两次运行 RSS 30s 采样）、guardrail.txt（两次 kill 记录）
- stack1.txt / stack2.txt（eu-stack 栈采样）、transpile.log（run_tests 全输出）
- lambda-prof.txt（16 段打点全文 + 曲线整理表）、lambda-prof.out（stdout，无 summary——被杀）
- monitor.py（护栏监控脚本）；classes/（副剖面预编译类目录）

## 七、余 7 例严重度表（2026-10-13 补扫：32G 护栏串行剖面）

> 对象：§2.1 triage 名单 63_junit 大槽 8 例中除 TestJunitFailureMessages 外的余 7 例（目录中另有的
> TestJunitAssertFamily / TestJunitHamcrestBridge 不在大槽名单，不在本扫范围）。方法与 §二 同构、护栏收紧：
> 逐例串行 `heavy_lock uv run python3 scripts/run_tests.py --jdk 25 --filter /<Test>.java --no-run
> --transpile-timeout 2400 --out-dir build/j25sweep`，30s 粒度监控分析器（只显式 PID kill -9，禁 pkill），
> 护栏任一即杀：① 分析器 RSS > 32 GiB；② MemAvailable < 20 G；③ 分析器已过 2400s；例间隔确认
> available ≥ 25G 再起下一例。结果逐例即时落盘 /tmp/j25sweep-progress.tsv（可中断恢复：中途宿主侧
> 后台任务被杀一次，按进度文件恢复续跑，已完 4 例结果保留；RSS 原始采样 /tmp/j25sweep/*.rss.tsv）。

| # | 用例 | kill 原因 | 终值 RSS | 死亡时刻 et | 越线 et（22G / 32G / 40G 外推） | 速率特征（早段→全程→末段，MiB/s） | 物化锯齿 |
|---|---|---|---|---|---|---|---|
| 1 | TestJunitIgnoreAndExpected | RSS>32G | 34.03 GiB | 780s | 462 / 750 / ≈966 | 52 → 43.9 → 38（末段仍最陡） | 1×2.01G |
| 2 | TestJunitNewInstancePerTest | AVAIL<20G* | 30.93 GiB | 750s | 457 / ≈784（外推*）/ ≈1040 | 55 → 41.4 → 32（起步最陡） | 1×1.70G |
| 3 | TestJunitBeforeClassState | RSS>32G | 32.22 GiB | 810s | 491 / 805 / ≈1069 | 49 → 40.0 → 31 | 1×1.84G |
| 4 | TestJunitLifecycleOrder | RSS>32G | 32.84 GiB | 840s | 481 / 818 / ≈1122 | 53 → 39.4 → 27 | 1×1.79G |
| 5 | TestJunitAssertThrowsAssume | RSS>32G | 32.61 GiB | 900s | 526 / 883 / ≈1198 | 47 → 36.5 → 26 | 1×0.68G |
| 6 | TestJunitRunnerResult | RSS>32G | 32.11 GiB | 900s | 530 / 897 / ≈1238 | 46 → 35.9 → 24 | 1×2.14G |
| 7 | TestJunitTimeoutFast | RSS>32G | 33.37 GiB | 930s | 544 / 903 / ≈1231 | 42 → 36.1 → 25（720–750s 有 1.7G 回落再越线） | 1×1.74G |

- 早段 = et 60–300s 斜率；全程 = 首末样本均值；末段 = 死前最后 300s 斜率；越线 et 为相邻采样线性内插；
  40G 外推 = 32G 越线 + 8 GiB ÷ 末段斜率。全部 Threads=1 单线程，run_tests 记账均 transpile 1 failed
  （run 0.00s，`--no-run`），无一完成转译——病理在转译侧，与 §二 FailureMessages 口径一致。
- *TestJunitNewInstancePerTest：机器侧并发作业把 MemAvailable 压破 20G 触发护栏②（死时自身 RSS
  30.93 GiB、末段 32 MiB/s，按该斜率 34s 后即越 32G，外推 ≈784s）——严重度判读不受实质影响。

**严重度排序（重 → 轻，按 22G/32G 越线时刻与斜率）**：IgnoreAndExpected ≳ NewInstancePerTest（最重档，
22G ≈460s、32G 750–784s）> BeforeClassState ≈ LifecycleOrder（中档，22G 480–490s、32G 805–818s）
> AssertThrowsAssume ≈ RunnerResult ≈ TimeoutFast（相对轻档，22G 526–544s、32G 883–903s）。
榜首两例差距在测量噪声内（32G 相差 ≈34s 且一为外推）；档内排序按 32G 越线时刻。

**与 FailureMessages 外推模型的一致性核对**：7 例 22G 越线 457–544s 落在模型锚点 445s 的 +3%~+22%、
40G 越线外推 966–1238s 落在模型锚点 985s 的 −2%~+26%，且全部复现「早段陡升（42–55 MiB/s ≈ 其 Phase A
44）→ 一次物化锯齿（0.7–2.1G ≈ 其 Phase C）→ 末段减速不收敛（24–38 ≈ 其 B/D 20–27）」的同构曲线
——junit 8 例同处一个严重度带（FailureMessages 22G/32G 越线 445s/~745s 居带首），§三的外推模型
（22G 槽 ~7.4min 死、40G 槽 ~16.4min 死）对余 7 例整体成立，与 triage 记录的 40G 槽峰值 39.6–39.9G 吻合。

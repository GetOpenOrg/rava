# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。
> 历史任务：T01–T81 见 `docs/tasks-history.md`（冻结），2026-09-16 起的完成项见 `docs/tasks-history-2026-09.md`；R5 轮集成后的完整遗留清单见 `docs/plans/2026-09-19-remaining-issues.md`。
> 格式：一行一个任务。大任务行只写总体状态与关键提交；拆出的子任务各占一行，编号 `<父任务>-<子项>`。下文「历史 §X」指 `docs/tasks-history-2026-09.md`「2026-10-04 tasks.md 一行一任务整理时移出的过程记录」节的 X 小节。

---

## ⚠️ 执行约束（最高优先级，不可绕过）

1. **架构问题优先**：先做架构改造，测试错误待架构完成后自然消解，禁止因为测试失败而中断架构工作转去修 Bug。
2. **架构完成前禁止全量测试**：定向验证（红线集 + 金丝雀）除外，全量 run_tests.py 只在架构节点合入后由主会话统一执行。
3. **子代理上限 5，子代理不得再派代理**（2026-10-05 起；重命令经全机锁，测试走合批）。
4. **任务执行顺序（2026-09-23 同步）**：已过时，当前顺序以下方「任务依赖树」为准（原文见历史 §A）。
5. **当前执行顺序（2026-09-24 晚，用户确认）**：已过时，当前顺序以下方「任务依赖树」为准（原文见历史 §A）。

7. **当前执行顺序（2026-09-29 刷新）**：已过时，当前顺序以下方「任务依赖树」为准（原文见历史 §A；其中 T-2 余项见 P3 表、N14 见开放项总表、#7 / #10 见原 20 项清单）。

6. **任务验收 = 端到端 Java 测试（2026-09-25 用户确认）**：每个任务以对应的 e2e Java 测试跑通为判定依据——已有测试直接用；没有就补（`tests/e2e/<类别>/TestXxx.java` + `tests/expected/TestXxx.txt` 由 JVM 生成）。构造测试须覆盖该任务涉及的**全部逻辑分支**，一个测试覆盖不全就拆成多个。定向测试跑通即视为正确、继续推进，**不等用户全量结果**；用户全量有问题反馈再处理。

8. **子代理派发规范（2026-10-04 用户采纳）**：
   - 一次派发只给一个可验收小步（写明退出条件与文件归属），达成即推送、汇报、停止；新阶段一律新开代理，不对已完成代理续派；
   - 硬上限 6 小时或上下文压缩 10 次，到点推送可编译状态 + 写计划文档交接节后停止；
   - 计划文档维护「现状」节（结论 / 失败路线 / 实测数字 / 下一步）；
   - 2 小时无可推送提交或同一问题两轮抽查未修好即停下报根因；改平台 cfg 分支的提交推送前做 Linux target cargo check。
9. **提速四项（2026-10-04 用户采纳，按序落地）**：
   - ① 已知失败清单 + 子代理自发抽查 + 合入守护脚本；
   - ② 服务器通用作业模式（closure / emit / compile 下放；现行本机只跑 cargo check，代理上限见第 3 条）；
   - ③ 按关键路径分配资源，下游等待期预排独立小步（当前关键路径首项：清 StockTrans 已知失败）；
   - ④ T1 档案化（跨测试复用编译）提前单独派发。

---

**关联追踪文档**：
- `docs/plans/2026-09-19-remaining-issues.md` — **当前主线**：A/S/G/P/V/R 六类遗留问题全清单（架构缺口/JVM 语义/生成器质量/原则违规/验证/仓库）
- `docs/plans/2026-09-21-codegen-type-convergence.md` — **生成器类型系统收敛路线图**：擦除-恢复/字符串手术/特例 if 链的统一诊断、五层调整清单（TypeIR/M-3/A-4/转换 IR 化/downcast 链）、量化终态
- `docs/plans/java-rust-translation-reference.md` — 翻译对照（宏家族 §16）
- `docs/tasks-history.md` — T01-T81 历史全记录

- `docs/tasks-history-2026-09.md` — 2026-09-16 起完成项归档（按批次追加）

**当前基线**：
- JDK21 冲刺收官预期 168/169（2026-09-24，见归档）；新语料 65 例抽样 51/65（S-65）。
- **2026-09-28 用户 JDK21 全量轮**：失败 22 例——rustc OOM 3 例（sg1，已改所有编译缺省调试信息减量 `f900436`）、日志为空 19 例（mac 11 例疑 Python < 3.11 缺 tomllib，已加 tomli 回落 `76639f7`；其余 5 例多机同刻失败，疑跑批框架层，待单例重跑取转译输出）。

---

## 📋 开放项总表（2026-09-27 刷新）

> 分支 `claude/jolly-dijkstra-diftum`（2026-09-28 已合 main 至 `486a162`；其后提交在分支上待合）。只列开放 / 待确认项；完成即移入归档文档。

### 原 20 项清单（开放部分）

| # | 任务 | 状态 | 证据 / 下一步 |
|---|---|---|---|
| 1 | 服务器 JDK21 收官轮 | ⏳ **用户执行中** | 本分支预期仅剩 TestVirtualThread（挂线程模型）；TestAnnotations 已随 M3 分支转绿。服务器单 rustc 峰值 ~14G，需 `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only` |
| 2 | JDK25 全量轮 | ⏳ 用户 `--failed` 轮 @8bf6baf：**23/34 出列、余 11**（2026-09-25） | 余 11 例本机已修（`197dc82` + `bacf062`）；本机 JDK25 定向 14 例 12 PASS，TestCompletableFuture 已修待复跑。**下轮 `--failed` 预期余 1（TestLocaleConstants，L-1 数据缺口）**。修复清单详见历史 §B |
| 7 | 窗口 3（G-1/G-2 迁 rs_ir） | ✅ **全部实施完成，待用户 JDK21 全量对账（G-2）** | ⑧⑨⑩、G-1（G1-a/b/c，终态达成）、G-2（`let mut x: T;`）全部实施；G-2 改变生成树，待用户 JDK21 全量对账。计划 `docs/plans/2026-09-25-window3-g1-structured-hoist.md`，实施证据详见历史 §B |
| 10 | Rust 重写 R0 启动 | ⏳ **门槛②③实施完成，待 G-2 全量对账确认** | 门槛③ P-1 ✅；门槛② 窗口 3 全部实施（⑧⑨⑩ + G1-a/b/c + G-2）；门槛① 口径见 roadmap §四-1。用户 JDK21 全量通过即可启动 |
| 15 | libc（posix 档 B） | 📝 **已定：保持按需** | 真实用例触达目录遍历 / 文件属性 / socket 时逐 native 补，不全量手写 |
| 16 | JDK25 第四失配（`sun/security/action` E0432）及后续 | ✅ **本机冒烟全绿，待用户 JDK25 全量确认** | 修复链 ①–④（`9df7959` / `5ec92fe` / `3a3312e` 等）；本机 JDK25 冒烟 6 例与 JDK21 受影响回归 8 例全 PASS。JDK25 须显式 `--jdk 25`（`9439d46`）。修复链详见历史 §B |
| 18 | hashCodeOfUTF16（j25-edge 下一层） | 📝 用户全量未触达 | 34 例 `--failed` 轮与本机 14 例均未命中；按需原则不预实现，触达时补 |

### 新增 / 遗留

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| N4 | TypeIR 完全体能力 G1–G5 | 🔄 G1/G2/G3/G5 ✅（2026-09-28）；G4 归 M-3 | 扩大口径 22→0 已完成；进度表见 `docs/reports/2026-09-24-typeir-remaining-survey.md` 开头；09-28 各项实施详见历史 §C |
| N4-G4 | TypeIR G4 | ⏳ 归 M-3 | 随 M-3 推进 |
| N4-余 | 其余「render_type → 串 → 解析」位点迁移 | ⏳ | 随 FS-Q1 Raw 收敛迁移 |
| S-65 | 新语料 65 例分层抽样（JDK21）：51/65（2026-09-25） | 🔄 修复已推送，复跑排队 | 14 例失败全部归因并修复，每类配 e2e；待复跑。归因、修复提交与新增用例详见历史 §C |
| N8 | 服务器编译资源约束 | 📝 已记录 | 单 rustc ~14G，约 1750 类贴线；已做 line-tables-only 缺省（`f900436`）、≥1700 类自动单作业 + 超时 3000 s（`driver/src/cargo.rs`）、`scripts/prune.sh` / `run_bg.sh`。根治靠生成代码体量下降。实测记录详见历史 §C |
| N14 | 闭包膨胀：几行 println 的用例闭包 ~1800 类（单测试 build 10–20 分钟、rustc 峰值贴 14G） | 🔄 2026-09-28 | 子项见下；过程详见历史 §C |
| N14-① | VM 常量守卫死分支剪除 | ✅ `0e261b6` | TestFieldEvalOrder 1827 → 1813（收益小） |
| N14-② | 虚 / 接口分派的 RTA 扇出（主因） | ⏳ | 下一步：先做「移除该边的闭包差」剖面定位大头，再出分派收窄方案 |
| N14-③ | 接口分派 CHA → 仅已实例化类 | ❌ 已撤回 | 1813 → 1812，非主因 |
| N14-④ | 拆 crate 可行性（2026-09-29 现状形态） | ❌ 现状形态不可行 | HelloWorld 最大强连通分量 97% 类；S7 后形态的复验见 V5（`docs/plans/2026-10-04-archive-crate-layering-decisions.md`） |

### 过渡态 → 最终态（2026-09-26 用户要求建立记录）

> 全量清单 116 项（无记录 50 项；2026-09-28 核对：开放 76 行 / 81 项，K1..K6 合一行）见 **`docs/plans/2026-09-26-transitional-state-inventory.md`**（编号 FS-xx）。下表只列优先项；其余按清单推进，完成即在清单中划除。

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| FS-P1..P3 / C4 | 系统属性全集、`System.exit`、`getenv`、ServiceLoader 静态服务表 | ✅ P1 `b938ea5`（同批附带 FS-Q9 修复 `83a4ac2`）、P2/P3 `098d5e9`（用户验证 TestSystemPropsSpec / TestShutdownHooks / TestSystemExitEnv PASS，后者含 `f4d0351`+`e2f78ef`）；C4 方案已出未实施 | 验证：TestSystemPropsSpec TestShutdownHooks TestSystemExitEnv |

## 📌 现状（2026-10-09）

> 依赖树仍为 10-08 版本，以本节与活跃任务表为准。

- **集成分支与 main**：rust-closure-analyzer = main，已含 batch-1013（669be8c6 起）。batch-1012（fcabee4a，含 1009–1011）与 batch-1013 均已放行。改名 rava 于 10-09 完成（目录、origin `yw/rava.git`、脚本路径）；dev 检出的 origin / 目录待 dev 恢复后改。
- **batch-1012 放行记录**：单测仅已知失败；抽查 53/55（TestUrlParsingFaces 已知，TestJndiNoProvider 转译超时、放宽重跑通过）。java_base OOM 根因与修复：s6（c18e8fc4）后启动映像对象 7702 → 19347 且集中在单个 static 与单个启动函数，charset-ext 将映像分 24 段、启动函数拆 42 个（b3a860ac）、重定位先于回放（160789c7）、`ImageData::writes_statics_of` 统一判定（14917aed），java_base 峰值 11.8 GB → 1.6 GB。
- **batch-1013 放行记录**：fix-1011 后续（3f78902a）+ reflect-marker（d51837e4，DeepCopy 3553 → 3532，HelloWorld / CollectorsDemo 3304 → 3263）+ logger-chain（65ab8a99，含 seed-chain c7fbaf8c，HelloWorld / CollectorsDemo → 3233，DeepCopy 3511，LogManager 0）。单测 A 组仅已知失败、B 组全过（`profile_union_key_and_coverage` 本次通过，第四根因仍在）。抽查 50/55：TestUrlParsingFaces 已知；DeepCopy、TestJndiNoProvider、TestSerialDefaultSuid、TestSerialUserGenericCallbacks 转译超 600 s，放宽超时重跑全部通过。
- **转译耗时回归**（进行中，perf-regress）：转译秒数 DeepCopy 527 → 912、TestSerialDefaultSuid 538 → 862、TestSerialUserGenericCallbacks 514 → 907（batch-1012 → 1013），TestJndiNoProvider 324 → 570 → 1006（batch-1009 → 1012 → 1013）；疑点 78b744fe 与 batch-1012 区间。目标：4 例回到 batch-1012 水平以下（JNDI ≤350 s），DeepCopy 峰值 ≤6 GB，闭包类集不变大。
- **fix-1011 第四根因**：具体求值站点（`Class.getGenericInterfaces`）回退普通分析后，已写入映像的缓存组没有撤回；终态做法是分析结束时删除只由回退站点贡献的缓存组（引导映像计划 §5.8.6）。
- **日志链缺口 ③**：HelloWorld 3233 未达 537 / 583；剩余持有者 `logRuntimeExit@74` 的 `log(DEBUG)`，需把 `isLoggable(DEBUG)` 按映像值折叠为 false（§5.9.7）。
- **进行中（10-09 派）**：缓存组回退撤回（bootcache，§5.8.6）；日志链缺口 ③（logchain3）；annot-sig 续作（并入 a6dca5c0 后先修 `reflect_new_array_element_precision`）。**待派（按序）**：C1d 闭包收窄余项（以 `rava closure --gates` 排名为准）；引导映像零拷贝（§8.3）；regress2 遗留。
- **已知单测失败**：`param_string_constants_fold_switch`。在缺少相应修复的分支上还会出现：`container_elements_per_object` / `known_gate_ranks_first`（缺 fix-1010）、`profile_union_key_and_coverage`（第四根因修复前）。
- **known_failures**：batch-1008 新增 TestBootLayer、删除 TestXmlSaxEvents；抽查已知失败 TestUrlParsingFaces。
- **派发规则**：子代理上限 5，不得再派代理。协调巡检自动攒批、空闲即测、放行合入与清理。
- **暂缓**：
  - build-memsafe；纯优化线（10-06 分级）；U1 重议；
  - 引用类语义（无 GC，C4 之后）；声明层底段收窄（C4 之后）；
  - 等 dev 恢复：S0 Spring Boot 闭包、确定性单测（`closure_independent_of_*`）、重例。
- **C4 全量**：尚未开始。前置：转译耗时回归修复、dev 恢复（换内存条 + BIOS + 内存自检）。
- **测试资源**：dev 内存坏，禁止投作业，等换内存条（BIOS 散热调整同一次停机做）。现用云服务器 jp1、jp2、kr1、kr2、sg1、sg2、us1；本机只跑 cargo check。合批全量单测拆 A（`-p driver --test closure_cli`）和 B（其余）两组并行，各约 1 小时。工作流见 `docs/reference/cluster-testing.md` 十二。

## 🌳 任务依赖树（2026-10-08，集成分支 rust-closure-analyzer = main = 7ed2154f）

> 图例：✅ 已完成（提交，日期）　🔄 进行中（分支）　🧪 待合批验证（分支，所在 batch）　⏳ 已立项待启　⏸ 暂缓（原因）　◇ 待用户决策
> `A ──▶ B` 表示 A 是 B 的前置。同一层内无箭头相连的任务互不依赖，可以并行。
> 一个节点一行，括号写分支 / 提交 / 计划文档；10-04 及以前的节点与过程见历史 §D–§L 与各计划文档。
> 派发口径：10-06 任务分级（下节）——只从「继续」项选；本机只跑 cargo check，测试走合批（`docs/reference/cluster-testing.md` 十二）。

### 一、全景树

```
rava 终态：Java 的新编译后端（开发者只写 Java，构建产出原生二进制）
│  量化：JDK 21 全量 e2e ⊇ 1029 例基线；`#[jvm_boundary]` 0（现 14）；手写只剩准入三类；产品路径 Python 0；JDK 25 适配
│
├─ 【已完成】（只列与剩余任务有依赖的；10-04 以前见历史）
│   ├─ ✅ 闭包引擎：engine-order V9（6aeedfe4，10-05）、V11（ab8dcfee，10-05）、V12 调用点重分析（8bc0c7aa，10-06）、D1 处理顺序无关（d46bb23a，10-06）
│   ├─ ✅ 引导映像求值器第 1–2 步（39dc3e02 / c5812d89，10-05 / 10-06）
│   ├─ ✅ C1d-a a3 第一批（b98a40f2，10-06）；Class 接收者镜像求值 c1d-clsfact（140ef55e，10-06）；容器元素第 1 项 c1d-elem（b15b81a3，10-06）
│   ├─ ✅ C4 预检与首轮全量分诊：c4-preflight（3f59a8e9，10-06）、c4-regress / c4-misc / c4-runfix / c1d-jca / fix-sam-default-lambda（10-07）、c4-resbundle（74837098）、c4-beans-precision（35b535a0）
│   ├─ ✅ JCA 服务按类型过滤 fix-jca-subset（b6ed3950，10-07）；JNDI 属性表逃逸 fix-jndi-bloat（f75c32a4，10-07）
│   ├─ ✅ 生成器命名：gen-lc-naming（84440f29）、gen-overload-hier 跨层重载命名（d7b490db，10-07）
│   ├─ ✅ junit-deps J0–J2 与 §3.6 删除 lib_runtime（ea2627ec，10-07）；S0 Spring Boot API 面第 1 步（c76c800e，10-07）
│   ├─ ✅ S7-0～S7-3、M1 / M2、D8 声明层 SCC 分段、R1-next、二进制体积 B1–B3（10-04～10-06）
│   └─ ✅ 测试分发与合批流程（cluster-testing.md 十二；dev 关机期间用云服务器）
│
├─ 【当前】阶段 C 收官：闭包分析器主线（正确、确定、最小，过 C4）
│   │
│   ├─ 🧪 合批 batch-1008（6934dc93 → 修复 b558e0c2；HelloWorld emit 内存超限定位中，c1d §30.18）
│   │     ├─ 🧪 boot-image-s3 / s4：引导映像第 3–4 步，`#[jvm_boundary]` 23 → 14，去除无映像回退（用户 10-08 确认保留）
│   │     ├─ 🧪 c1d-url-b2：URL 来源精度 B1–B6（c1d §30.17）
│   │     ├─ 🧪 user-unreach-stubs：链外方法存根、use 行扫描按调用链门控
│   │     ├─ 🧪 enum-values-direct：直连反射调用（类数 3011 未降，余 4 个调用点）
│   │     └─ 🧪 closure-composition：闭包构成报告与脚本
│   │           └─▶ 合批通过即合入集成分支、快进 main
│   │
│   ├─ 🔄 引导映像第 5 步（boot-image-s5）：jimage 嵌入 + getNativeMap，ClassLoader 6 / BootLoader 2 / JceSecurity 6，`#[jvm_boundary]` 14 → 0 ◀── batch-1008
│   │     ├─ U14 `java.home` 构建期钉值随本分支：JceSecurity 策略文件改为构建期事实
│   │     ├─ ⏳ 零拷贝永久区（§8.3，10-07 定；先服务器跑通现实现，再改零拷贝，再测体积 ≤+5% / 启动装载 ≤1 ms）
│   │     └─▶ 第 6 步 非引导类构建期初始化（C3 build_time_init）──▶ 第 7 步 语料全量
│   │
│   ├─ 🔄 u12-props（基于 b558e0c2）：U12 机制 ①③（构建期定 LoggerFinder 提供者、日志级别折叠）+ U14 `line.separator`（按目标三元组）/ `file.encoding`（UTF-8）钉值；逐项实测闭包类数，无收益不钉
│   ├─ ⏳ 待派：S2 `Class.genericInfo` 入映像（U13 已定 10-08），有空名额即派
│   │
│   ├─ 🔄 闭包门自动排名 `rava closure --gates`（closure-gates）──▶ 解释 HelloWorld 468 与约 3011 类的落差（引导映像 §8.4 待核对）
│   │
│   ├─ ⏳ C1d-a 收窄余项（c1d 计划）：a5-4 终态 DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468；§29 能力③ RB 未知 Class（随引导映像）、格式串常量求值、a5-4e / a5-4f；precheck 按目标平台 jmod
│   ├─ ⏳ C1d-b 余项：b2（◀── why2-93e0f28e 取证）、b3 余 URL$DefaultFactory 扇出
│   │
│   └─ ⏳ C4 收官：验收轮全量 e2e（JDK 21）⊇ 1029 例基线——尚未开始
│         ◀── 合批（batch-1008 起）合入集成分支、改名 rava、dev 恢复（换内存条 + BIOS）
│
├─ 【近期】依赖 C4 收官
│   ├─ ⏳ scripts-into-rava S6 → S7 → S8（产品路径 Python 归零）
│   ├─ ⏳ JUnit 依赖包测试 J3 形态接线 → J4 10 例跑通（2026-10-05-junit-e2e-deps-task.md）
│   ├─ ⏳ 框架驱动 API 覆盖（2026-10-07-framework-driven-api-coverage.md）：S0 闭包面复算 ◀── dev 恢复；再扩其他流行库
│   ├─ ⏳ jmod 覆盖第 1–4 步（2026-10-03-jmod-coverage.md）
│   ├─ ⏳ 引用类语义：无 GC 模型（编译期逃逸分析整组释放 + 所有权推断弱引用，2026-10-07-no-gc-memory-model.md）◀── C4
│   └─ ⏳ JDK 25 适配轮
│
├─ 【暂缓】
│   ├─ ⏸ build-memsafe 内存友好缺省构建档（10-08 暂缓）
│   ├─ ⏸ 纯优化线（10-06 分级）：二进制 ≤3 MB、S7-4 / S7-5、D2 / D3 引擎结构改造、IR 收敛 / TypeIR G4
│   ├─ ⏸ 声明层底段收窄：S7-4 / S7-5 收 SCC → D8 自动切段，每个声明 crate ≤1.3 GB ◀── C4（10-08 用户定）
│   ├─ ⏸ 等 dev 恢复（换内存条，数天）：S0 Spring Boot 闭包、确定性单测（`closure_independent_of_*`）、重例
│   ├─ ⏸ 不实施 / 挂起（10-06 用户定）：方法句柄对象化、T2 余 4b、b1 序列化收窄、a5 关系型边界推理
│   └─ ⏸ 缓：虚拟线程余项（T6 规模、pinned）、T1-M3、第三方库通用机制（JNI 层 / 构建期捕获运行期生成类）
│
└─ 【远期】
    ├─ ◇ 线程模型终态（2026-09-26-real-multithreading.md）
    ├─ ⏳ 真实项目 pilot ──▶ Spring Boot 等完整转译
    └─ ⏳ 产品化：构建流程集成 ──▶ 公开 Demo 与性能对比（2026-09-18-product-vision.md）
```

### 二、关键路径

```
batch-1008 合批验证（emit 内存超限修复）──▶ 合入集成分支 ──▶ 引导映像第 5 步（14 → 0）──┐
u12-props（U12 ①③ + U14 钉值）/ U13 genericInfo（待派）─────────────────────────────────────┤
closure-gates ──▶ 闭包落差解释 / C1d 收窄余项 ─────────────────────────────────────┼──▶ C4 验收全量 ──▶ S6–S8 / JUnit J3–J4 / API 覆盖 ──▶ pilot ──▶ 产品化
改名 rava ／ dev 恢复（换内存 + BIOS）──────────────────────────────┘
```

---

## 🧭 任务分级（2026-10-06 用户采纳）

用户定：功能优化先放一边，先完成闭包分析器主线（正确、确定、最小，过 C4）。派发只从「继续」项选，空出名额优先 C3 / C5。

| 类 | 项 | 处理 |
|---|---|---|
| A 正确性缺陷 | A1 TestModuleLayerDefine NPE（引导映像第 3 步）、A2 TestClassModuleFace 具名模块（第 4 步）、A4 JNDI / HTTP 转译超时、A5 C4 全量 e2e | 继续 |
| A 正确性缺陷 | A3 URL 协议可靠口径（c1d-urlhost） | 依赖提速，随 D2 搁置 |
| B 架构终态 | B1 a3 `#[jvm_boundary]` 归零、B2 引导映像求值器第 2–7 步 | 继续 |
| B 架构终态 | B3 虚拟线程余项（T6 规模 / T1b 审计 / pinned 偶发）、B4 T1-M3 按模块登记 | 缓 |
| C 闭包精度与规模 | C1 c1d-elem、C2 clsfact、C3 a5-4 收窄（DeepCopy ≤2803，§29 三项能力优先）、C5 C1d-b（T2 余 / b1 / b2 / b3 余） | 继续 |
| C 闭包精度与规模 | C4 a5 关系型边界推理（HelloWorld ≤371） | 缓 |
| C 闭包精度与规模 | C6 jar 签名收窄 / jar·URL class-path（并入 §29 能力①）、§29 能力② JCA 提供者序求值 | 继续，优先（10-06 用户定） |
| C 闭包精度与规模 | 方法句柄对象化（C1d-b-mhobj）、T2 余 4b、b1 | 不实施 / 挂起（10-06 用户定，上界 −5~−6 类） |
| C 闭包精度与规模 | 共享汇点（c1d-sink） | 已收口（§28.10），≤569 由引导映像达成 |
| D 分析性能 | D1 处理顺序无关（分支 closure-order-free，定性为正确性） | 继续 |
| D 分析性能 | D2 枢纽翻新 / 延迟站点重跑等结构改造、D3 在线节点合并 | 暂停（V12 后提速线暂停） |
| E 编译资源 | E1 B4 内存友好缺省构建档（分支 build-memsafe，16 GB 机器全部可构建为硬约束） | 暂缓（2026-10-08） |
| E 编译资源 | E2 D8 声明层分段 | 机制已合入（b51f9531 / b093069f）；底段收窄随 S7-4 / S7-5，C4 之后（见活跃任务「声明层底段收窄」） |
| B 架构终态 | B5 第三方库通用机制：JNI ABI 层（库自带 native 原样调用）、构建期捕获运行期生成类（三方依赖分层 §3.6；rava 仓库不放任何第三方库专属内容，库配置归用户项目） | 缓（10-06 用户定） |
| F 纯优化 | 二进制 ≤3 MB、S7-4～5（S7-3 已合入）、VT `instanceof` / `checkcast` 走 `__ClassDesc`、IR 结构化收敛 / TypeIR G4 | 暂停 |

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途 / 待验证 / 待启项（2026-10-09）；已完成与过时行已删，见历史 §J 与各计划文档。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| 缓存组回退撤回 | 🔄 bootcache（a6dca5c0） | 具体求值站点回退普通分析时，撤回只由该站点贡献的映像缓存组；修复后 `profile_union_key_and_coverage` 应通过。引导映像计划 §5.8.6 |
| reflect-marker 耗时回归 | ⏳ 改名后派 | DeepCopy 分析 502 → 889 s、峰值 5.9 → 7.7 GB，疑为 78b744fe；续作 enum-values-direct §9.6 |
| 日志链缺口 ③ | 🔄 logchain3（a6dca5c0） | 把 `isLoggable(DEBUG)` 按映像值折叠为 false，去掉 `logRuntimeExit@74` 持有者；HelloWorld 3233 → 目标 537 / 583。§5.9.7 |
| 转译耗时回归 | 🔄 进行中（perf-regress，基于 5090505f） | DeepCopy 527 → 912 s、序列化两例 ≈520 → ≈890 s、TestJndiNoProvider 324 → 1006 s；目标回到 batch-1012 水平以下（JNDI ≤350 s）、DeepCopy 峰值 ≤6 GB、闭包类集不变大 |
| 引导映像零拷贝 | ⏳（§8.3，10-07 定） | 映像落为 Rust 常量；体积 ≤+5%、启动装载 ≤1 ms |
| C1d-a-a5-4 | ⏳ | 终态 DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468；余 §29 能力③（随引导映像）、格式串常量求值、a5-4e ICU、a5-4f 日志后端 |
| C1d-a-precheck | ⏳ | 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3） |
| C1d-b-b2 | ⏳ ◀── why2-93e0f28e 取证 | 任务 2 |
| C1d-b-b3余 | ⏳ | URL$DefaultFactory 反射构造器扇出收窄 |
| regress2 遗留 | ⏳ 待用户定（regress2-rest） | 过渡 `<init>` 帧 ✅ 已随过渡手写删除消失；Object.wait 帧仍错（单帧 -1，JDK 为 `wait0` native + `wait` 行号帧）——根因是根类 `wait` 三重载有字节码却整体手写（还跳过 Blocker 载体补偿），终态为根类非 native 方法按字节码翻译，待用户定。边界用例 TestObjectWaitFrames（作业 r2-wait-a79e2b60）。regress2 文档 §10.1b |
| C4 收官 · 全量 e2e | ⏳ 尚未开始 | JDK 21 ⊇ 1029 例基线；前置：合批（batch-1008 起）合入集成分支，以及改名 rava 与 dev BIOS 维护窗口。10-06／10-07 的首轮全量分诊修复已合入（c4-preflight / c4-regress / c4-misc / c4-runfix 等） |
| JUnit 依赖包测试 | ⏳ J3 / J4 ◀── C4 | J0–J2 ✅（f9298933 / ea2627ec）；任务书 `docs/plans/2026-10-05-junit-e2e-deps-task.md` |
| 框架驱动 API 覆盖 | ⏸ 暂缓（等 dev 恢复） | S0 第 1 步 ✅ c76c800e；闭包两变体在 15G 云服务器上未产出，dev 恢复后复算 |
| build-memsafe | ⏸ 暂缓（2026-10-08） | 内存友好缺省构建档（16 GB 机器全部可构建为硬约束） |
| 声明层底段收窄 | ⏸ C4 之后（10-08 用户定，按现有顺序） | D8 分段已合入：上段每段约 330 类、约 1.27 GB；底段 `java_base_decl` 是含 INFRA 的签名 SCC（约 76% 类），现状形态即下限，峰值 7.9 GB（D8 时）→ 4.9 GB（10-08 CollectorsDemo，sg2）。终态：S7-4 / S7-5 把最大 SCC 收到约 22%，D8 机制自动切段，每个声明 crate ≤1.3 GB，D8 无需改。计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` §九（§9.5 / §9.7） |
| 引用类语义 | ⏸ 暂缓（C4 之后） | 无 GC 模型，`docs/plans/2026-10-07-no-gc-memory-model.md` |
| 测试分发 | ✅ 2026-10-02 起 | 全部 e2e 与重命令作业经 `scripts/cluster/distribute_tests.py` 在服务器执行；dev 关机期间用云服务器（jp1、jp2、kr1、kr2、sg1、sg2、us1）；本机只跑 cargo check；合批测试见 `docs/reference/cluster-testing.md` 十二 |

---

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| JDK25 边界 stubs 遗留 | P-3 轮 | `DoubleToDecimal.split`（Formatter `%f/%e/%g`）、`FloatToDecimal`、`Random__nextInt_i_base`（E0432）——随 JDK25 用例按需补 |

## P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| mut 标注递归覆盖 | T54 | 嵌套块 |
| 括号优化 | T60 | 表达式优先级 |
| 布尔压缩 | T70 | `if c {1i32} else {0i32}` → bool（R5 已修部分：lxor/比较产物） |
| 语义桩计数 | T72 | 指令级统一标记未做 |
| 类级并行解析 | T65 | 测试级并行已实现（run_tests.py），类级未做 |
| IR 结构化收敛 | T05/T06/T07/T50/T58/T61/T67 | RawExpr/RawStmt 消除，架构级 |

## P3 · 长期重构（不阻塞主线）

> 详细设计见 `docs/plans/2026-09-17-java-rust-type-1to1.md` 与 `docs/plans/2026-09-18-erased-runtime-identity.md`。

| 任务 | 启动条件 / 状态 | 说明 |
|------|---------|------|
| **T-2 接口泛型进类型位置** | 🔄 2026-09-29：7a / 7b / 标记接口 / 7d 兼容部分 ✅ | 擦除阶段 1 已落地（非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`）；方案 `2026-09-28-t2-interface-carriers.md`；原行见历史 §K |
| T-2-7c | 🔄 验证中 | 桥方法签名代入修复验证 → 名单置 None |
| T-2-余 | ⏳ | 接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类 / 枚举 / 手写类的 `__interface` 覆盖 |
| T-2-收尾 | ⏳ ◀── 接口载体全面铺开 | 删除 `carrier_type_positions.txt` 与 `signature_erased_interfaces.txt`（FS-M2 最终态，不迁移进 TOML，`d449f77` 决定）；`overload_abbrev.txt` 保留，不另建 `codegen_types.toml` |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long` 等从字节码生成（String 已走生成） |
| **R-3 `#[derive(Debug)]` 替换宏生成 Debug** | 无依赖 | 前提：生成类字段类型均满足 `Debug` |

---

## 维护规则

1. 完成一项 → 从本表删除，整行移入 `docs/tasks-history-2026-09.md` 当期归档批次（保留提交号与证据）
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新基线数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log
5. 一行一任务：父任务状态栏只写总体状态与关键提交，子任务各占一行；证据 / 下一步每格约 200 字以内，过程史写进计划文档或历史文档

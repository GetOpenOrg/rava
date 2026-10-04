# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。
> 历史任务：T01–T81 见 `docs/tasks-history.md`（冻结），2026-09-16 起的完成项见 `docs/tasks-history-2026-09.md`；R5 轮集成后的完整遗留清单见 `docs/plans/2026-09-19-remaining-issues.md`。
> 格式：一行一个任务。大任务行只写总体状态与关键提交；拆出的子任务各占一行，编号 `<父任务>-<子项>`。下文「历史 §X」指 `docs/tasks-history-2026-09.md`「2026-10-04 tasks.md 一行一任务整理时移出的过程记录」节的 X 小节。

---

## ⚠️ 执行约束（最高优先级，不可绕过）

1. **架构问题优先**：先做架构改造，测试错误待架构完成后自然消解，禁止因为测试失败而中断架构工作转去修 Bug。
2. **架构完成前禁止全量测试**：定向验证（红线集 + 金丝雀）除外，全量 run_tests.py 只在架构节点合入后由主会话统一执行。
3. **子代理串行执行**：一次只运行一个子代理（用户指定，内存约束）；前一个完成并合入验证后再启动下一个。
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
   - ② 服务器通用作业模式（closure / emit / compile 下放，本机只留 driver 构建与单测，之后代理上限提至约 8）；
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

## 🌳 任务依赖树（2026-10-04，集成分支 rust-closure-analyzer 本地 21fc601b，未推送）

> 图例：✅ 已完成　🔄 进行中　⏳ 已立项待启　⏸ 按用户决定暂停　◇ 待用户决策
> `A ──▶ B` 表示 A 是 B 的前置。同一层内无箭头相连的任务互不依赖，可以并行。
> 一个节点一行，括号写分支 / 提交 / 计划文档；过程与实测史见历史 §D–§H、§L 与各计划文档。
> 分层决策与待验证清单：`docs/plans/2026-10-04-archive-crate-layering-decisions.md`（D1–D8、V1–V12）。

### 一、全景树

```
rava 终态：Java 的新编译后端（开发者只写 Java，构建产出原生二进制）
│  量化：JDK 21 全量 e2e ⊇ 1029 例基线；HelloWorld ≤250 类、闭包 ≤3 s；`[boundary]` 前缀 0、`#[jvm_boundary]` 0；
│        手写只剩准入三类；产品路径 Python 0；JDK 25 适配
│
├─ 【已完成的大分支】（只列与剩余任务有依赖的）
│   ├─ ✅ Rust 生成器取代 Python 生成器，Python 生成器已删（py-delete，2026-10-01-python-generator-deletion.md）
│   ├─ ✅ C1 / C1c / C2 / C3：Rust 闭包分析器、精确分析、seeds、反射数据流（2026-09-29-rust-closure-analyzer.md）
│   ├─ ✅ C4 接入、第五节 Python 机制删除（closure-c4-cleanup）
│   ├─ ✅ 闭包精度一 / 二 / 三期，闭包性能 closure-perf2 / closure-mono（d8212bee / 6e0849c6）
│   ├─ ✅ C6 主体：upcalls 声明与解析机制清零（59dbedc1），c4-regfix 合入 1ba0d9aa
│   ├─ ✅ C6 后续（c6-generic-closure 7fdbfa8c，合入 3f9d4cc9）
│   ├─ ✅ C1d-b 第一段 c1d-pick（4b73ea61）
│   ├─ ✅ regress2 第二轮基线回归 + FS-E1（6c7eb831）
│   ├─ ✅ regress2 续：栈帧来源统一（frames-unify bf91f075，合入 be1b97be）
│   ├─ ✅ native-gaps（bdc4cd64，合入 417a6594）
│   ├─ ✅ FS-C2 应用类加载器（fs-c2 9c737f03，合入 4a98f5e3；vm_boundary_methods 30→27）
│   ├─ ✅ scripts-into-rava S1–S5：Python 脚本并入 rava、名字作用域统一、m3 编译错误 0（bb0b7736）
│   ├─ ✅ run-tests-prune：逐例清理产物、rava prune（ad9e938d）
│   └─ ✅ 测试分发：全部 e2e 与重命令作业走 8 台服务器（server_maintenance/rava/distribute_tests.py）
│
├─ 【推送阻塞】集成分支本地已合 crate-split / module-m1 / s7-desc，未推送
│   └─ 🔄 engine-order 闭包结果与哈希顺序无关（V9，ca2488f0）：门禁 closure_independent_of_hash_seed 在 TestSerialLookupPairing 失败
│
├─ 【当前】阶段 C 收官：闭包分析器（rust-closure-analyzer）── 用户 2026-10-01 决定先做完本阶段
│   │
│   ├─ 🔄 闭包引擎提速（用户 2026-10-04 定先做）
│   │     ├─ ✅ http-perf A/B/C（c5741dfe，合入 a068b87a）：TestHttpLoopbackSync 本机 404→285 s、服务器 630 s
│   │     ├─ 🔄 engine-order（V9）── 见【推送阻塞】
│   │     ├─ ⏳ 逃逸对象上下文收拢（V10）：TestHttpLoopbackSync 服务器闭包 ≤60 s ◀── engine-order
│   │     └─ ⏳ URL 协议可靠口径（c1d-urlhost 9087cf1c）◀── 引擎提速达标
│   │
│   ├─ 🔄 C1d-a 去截断（c1d-p0，2026-10-01-c1d-closure-bloat.md；原节点见历史 §D）
│   │     ├─ ✅ a1 具体求值器 engine/concrete/：正式 HelloWorld 423 类 / 2–3 s（≤360 余量转 a5）
│   │     ├─ ✅ a2 合入 62f46bb2（b4669206，抽查 9/9）
│   │     ├─ 🔄 a2 续：initPhase2 膨胀定位 → 早退检查按分析期事实求值 → [[boot_init.phases]] → boot layer 步骤 2–5
│   │     ├─ ✅ a4 TestCharsetNamedStreams（b124e5ac，抽查 c1da-f2bdcf6e 通过）
│   │     ├─ ✅ JCA 请求点值流 / 种子修复（51a4d8c5）：StockTrans 3283 / DeepCopy 3278 类
│   │     ├─ ⏳ jar 签名校验路径收窄（JCA 后 +134 类的来源）
│   │     ├─ ✅ jar/URL 来源精度乙（fca1643b，闭包不变，根因 §22.10）
│   │     ├─ ⏳ jar/URL 来源甲 class-path（计划 §22.2）
│   │     ├─ ⏳ a5-4 闭包膨胀收窄：终态 DeepCopy ≤1640（计划 §21.5）
│   │     │     ├─ ✅ s1 构造器查找只在 Class 值集齐全时点名
│   │     │     ├─ ✅ s2 instanceof 否定分支收窄 + 钩子字段不按 open
│   │     │     ├─ ⏳ a5-4b 引导加载器类路径查找 → JarVerifier / pkcs11
│   │     │     ├─ ⏳ a5-4e ICU 归一化入口 / Latin-1 语言折叠
│   │     │     ├─ ⏳ a5-4f 日志后端探测
│   │     │     ├─ ⏳ 容器元素 Object 方法（归通用 open 值精度另立项）
│   │     │     └─ ⏸ macOS 专有 MacOSXFileSystemProvider 多级协变桥缺失（暂不修，复现见历史 §D）
│   │     ├─ ⏳ a5 OOB 关系型边界推理（a5-1 → a5-2 → a5-3），HelloWorld 目标 ≤371
│   │     ├─ ⏳ a3 #[jvm_boundary] 归零：审计数 86→0，拆 U0–U3 / V / T / L1 / L2 / C / X1 / X2 / Z（计划 §21、§21.7）◀── a2
│   │     │     └─ 🔄 a3-T 虚拟线程终态（a3t-vthread，§21.8）：T1–T5 ✅ a78cccef；余 T6、T1b 审计
│   │     └─ ⏳ precheck 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3）
│   │
│   ├─ 🔄 C1d-b 反射与过近似收窄（c1d-pick，2026-10-02-c1d-reflect-narrow.md；原节点见历史 §E）
│   │     ├─ ✅ b0 合入 e90a592d（eb6571ba）
│   │     ├─ 🔄 b1′ ArrayList.writeObject 分派臂（计划 §4.6）：T3–T7 ✅，T2 ✅ 35c5f0ee
│   │     │     └─ ⏳ T2 余项：getDefaultSerialFields 收窄、4b
│   │     ├─ 🔄 b1 序列化收窄：S2 ✅ 0d7dd2a5；DeepCopy ≤1640、fold_props ≥42
│   │     ├─ ⏳ b2 任务 2 ◀── why2-93e0f28e 取证
│   │     ├─ ✅ b3 class_init.unknown 归零（d46d9b06，扇出收窄 6294755d / ee52c596）
│   │     │     └─ ⏳ b3 余：URL$DefaultFactory 反射构造器扇出（未见完成记录）
│   │     ├─ ✅ lambda 隐藏类（0060fa77，合入 94d2ff90）+ from_any 归零（631bb78b）
│   │     ├─ ✅ gen-fixes 生成器独立缺陷五步（52cde496 / 0e1a5d1f / a2e2a7e5）
│   │     ├─ ✅ non_native_overrides 清零（2816cfa8 / 601b0e3b）
│   │     └─ 🔄 TestJndiNoProvider 冷闭包转译性能（2026-10-03-jndi-transpile-perf.md）：第 1 步 ✅；余修法 B
│   │
│   ├─ 🔄 boot layer：ModuleBootstrap.boot 引导期建层、System.bootLayer 按字节码读取、系统模块描述符承载（FS-H12）
│   │     ├─ ✅ 第 0 步 27dfb419：镜像与 jmod 字节不同的类以镜像为准
│   │     └─ ⏳ 第 1 步起 ◀── C1d-a a2 续；验收 TestModuleLayerDefine 原样通过
│   │
│   ├─ ⏳ regress2 遗留：Object.wait(J/JI) 手写帧行号 -1、过渡类手写 <init> 不成帧 ◀── C1d-a a2
│   │
│   └─ ⏳ C4 收官：全量 e2e（JDK 21）⊇ 1029 例基线
│         ◀── C1d-a、C1d-b、boot layer 全部合入
│
├─ 【近期】阶段 C 之后，依赖 C4 收官
│   │
│   ├─ ⏳ scripts-into-rava S6–S8（2026-10-01-scripts-into-rava.md）
│   │     S6 dyn 对照并入 rava（dyn crate，删 dyn_compare.py / dyn_agent）◀── C4 收官（dyn_compare 改动冻结）
│   │      └─▶ S7 run_tests 拆 scripts/e2e/，预留形态接口 ──▶ S8 产品路径 Python 归零、计划结项
│   │
│   ├─ ⏳ JUnit 依赖包作为测试（2026-10-01-junit-crate-as-test-harness.md）
│   │     步骤 0 m1..m5 Rust 路径复跑 golden，清零回归（可提前；m3 编译 0 ✅）
│   │      └─▶ 步骤 A：63_junit 形态接入 run_tests ◀── S7 ──▶ 步骤 B ──▶ 步骤 C
│   │
│   ├─ ⏳ R1 运行性能：超时用例（标杆 LynchBell 等 12 例）不改测试、不放宽时限
│   │     （2026-09-30-optimization-directions.md §三.4）◀── C4 收官后排期
│   │
│   └─ ⏳ e2e 扩展到 java.base 之外的 JDK 模块（2026-10-03-jmod-coverage.md；64_–74_ 共 43 例已入 4939f290；原节点见历史 §G）
│         ├─ ✅ 63_junit expected 10/10、新增用例查重、6 例 expected 复核（d4efc8d6 / ffbebc0d）
│         ├─ ✅ e2enew-da8abee1 失败 69 例归因：13 根因族（docs/reports/e2enew-da8abee1-triage.md）
│         ├─ ⏳ R1 xml lambda 存根 SecuritySupport.lambda$getSystemProperty$0 可达性（10 例）◀── jmod 第 1 步
│         ├─ ⏳ R2 泛型反射 scope 构造器存根 + E0432（6 例）◀── T2 队列
│         ├─ ⏳ R5 Method.invoke 实参数量 / 类型不符抛 IAE ◀── T2 队列
│         ├─ ⏳ R11 compareTo 桥分派闭包 ◀── T2 队列
│         ├─ ⏳ R3 JCA ProviderList / GetInstance 存根（5 例）◀── C1d-a JCA 收窄线
│         ├─ ✅ R4 java.logging E0433（服务器复跑 4/4，归环境侧）
│         ├─ ⏳ R6 模块元数据 + 强封装边界（2 例）◀── boot layer
│         ├─ ⏳ R7 系统资源装载（3 例）◀── boot layer
│         ├─ ⏳ R8 beans finder 构造存根（3 例）◀── jmod 第 1 步前置
│         ├─ ⏳ R9 charset / zipfs 提供者构造（5 例）◀── jmod 第 1 步前置
│         ├─ ✅ R10 Array.get/set 系 native、R12 E0308 三例、数组协变 TestClassCastSubclass（gen-fixes 0e1a5d1f）
│         ├─ ⏳ R13 http async 转译错误
│         ├─ ✅ 参考 JDK 固定构建（jdk-pin 75a15d93）+ 全量 golden 核验（c3ad4c4e）
│         ├─ 🔄 TestLocaleCurrency 生成器缺陷（CN¥ vs ¥）：rava 侧闭包检查待做（docs/reports/2026-10-03-locale-currency-cn.md）
│         ├─ ⏳ TestSocketLoopbackPair 改写后 rava 产物 main 线程 NPE，待归因
│         ├─ ✅ 「档案调用链」口径文档同步（ad62f2c1）
│         └─ ⏳ 第 0 步 A 档用例预审（rava audit，可提前）
│              └─▶ 第 1 步 A 档 7 模块 ◀── C4 收官、boot layer、b3 CallerSensitive
│                   └─▶ 第 2 步 java.xml ──▶ 第 3 步 HTTP 回环 + 空提供者 ──▶ 第 4 步 beans / geom 子集
│
├─ 【中期】优化线（用户 2026-10-01 决定暂停、C 阶段收官后恢复；10-04 起 crate-split / S7 / T1 已派发；原节点见历史 §H）
│   │
│   ├─ ⏸ 闭包分析效率 P8 余量、sites（optimization-directions §三.2）
│   ├─ 🔄 生成器 / 下游编译成本：V1–V7、S 系列余项（emitter-performance、rustc-memory-and-crate-split）
│   │     ├─ ✅ unsafe-rmw 合入 5f759708
│   │     ├─ ✅ crate-split 声明层拆分（db45a65c）：抽查 14/14，本地合入 3e739189，待推送 ◀── engine-order
│   │     └─ ⏳ SCC 拆分（D8）◀── S7
│   ├─ 🔄 S7 统一对象句柄 + 每类静态描述符（D7 批准取法 B；2026-10-04-s7-object-handle-descriptor.md）
│   │     ├─ ✅ S7-0 / S7-1 s7-desc（b0166702）：抽查 16/16，本地合入 8c218a72，待推送
│   │     └─ ⏳ S7-2 … S7-5
│   ├─ 🔄 T1 跨测试编译复用（决策 ✅ 2026-10-03 四项全采纳，2026-10-01-cross-test-compile-reuse.md）
│   │     ├─ ✅ T1-1a 档案化分析（90398dc8）、T1-1b 按档案生成 JDK crate（10cfb657）
│   │     ├─ ✅ T1-2 方案 t1-link（6aa69280，合入 53f32664；2026-10-04-t1-step2-direct-rustc-link.md）
│   │     ├─ ✅ M1 jmod 模块映射与 import 过滤（module-m1 188d67ac）：抽查 12/12，本地合入 9d587416，待推送
│   │     ├─ ⏳ M2 按模块切 crate ──▶ M3 按模块登记
│   │     ├─ ⏳ L2–L4（同计划）
│   │     └─ ⏳ T4 生成器只构建一次再分发（待服务器核实）
│   └─ ⏳ JDK 25 适配轮 ◀── C4 收官（JDK 25 不设 Python 基线；8 台服务器 JDK 25 已就绪，env_setup --check-only 2026-10-03）
│
└─ 【远期】
    ├─ ◇ 线程模型终态：单线程协作调度深化，或改真并发（2026-09-26-real-multithreading.md）
    │     ◀── VirtualThread 调查 + 真实语料需求（long-term-roadmap §四 决策 2）
    ├─ ⏳ 真实项目 pilot：P1 commons-lang3 对账 harness ◀── JUnit 步骤 C、T1 / S7 决策
    │     └─▶ Spring Boot 等知名项目完整转译 ◀── 线程模型终态、反射 / 动态代理完备
    └─ ⏳ 产品化：构建流程集成（开发者写 Java、构建自动出原生二进制）──▶ 公开 Demo 与性能对比
          ◀── 真实项目 pilot、R1 运行性能（2026-09-18-product-vision.md）
```

### 二、关键路径

```
engine-order（V9）──▶ 推送集成分支（crate-split / module-m1 / s7-desc）──▶ V10 ──▶ URL 可靠口径
C1d-a a2 ✅ ──▶ a2 续 / a3 jvm_boundary 归零 / a5-4 收窄 ──────────┐
C1d-b b1′ T2 余项 / b1 序列化收窄 ─────────────────────────────┤
boot layer 第 1 步起（◀── a2 续）─────────────────────────────┼──▶ C4 全量 e2e ──▶ S6 ──▶ S7 ──▶ JUnit A ──▶ B/C ──▶ 真实项目 pilot ──▶ 产品化
regress2 遗留（◀── a2）───────────────────────────────────────┘        └──▶ 优化线（P8 / V / S7·T1 / R1 / JDK 25）
```

当前阻塞：engine-order 门禁失败阻塞集成分支推送；C4 的前置剩 C1d-a（a2 续 / a3 / a5-4）、C1d-b（T2 余项 / b1）、boot layer 第 1 步起。2026-10-02 版说明见历史 §I。

---

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途分支的当前状态。原表长单元格见历史 §J。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| engine-order 闭包结果与哈希顺序无关（V9） | 🔄 ca2488f0 | 集成分支门禁 `closure_independent_of_hash_seed` 在 TestSerialLookupPairing 失败（种子 1 多出 com/sun/crypto/provider/AESCipher 一族），由该线修复；**推送集成分支的阻塞项** |
| 逃逸对象上下文收拢（V10） | ⏳ ◀── engine-order | TestHttpLoopbackSync 服务器闭包 ≤60 s，HTTP 两例通过，闭包集合变化逐项论证 |
| URL 协议可靠口径 | ⏳ c1d-urlhost 9087cf1c ◀── 引擎提速达标 | 证 file URL host 为 "" / localhost 以杀 ftp 分支；引擎提速期间口径偏差暂容忍 |
| crate-split 声明层拆分 | ✅ db45a65c，本地合入 3e739189 | 抽查 14/14；待种子门禁（engine-order）修复后推送 |
| C1d-a 去截断（c1d-p0） | 🔄 2026-10-03 | 闭包闸门 P2/P3；StockTrans 3283 / DeepCopy 3278 类，目标 DeepCopy ≤1640；子项见下，过程见历史 §D / §J |
| C1d-a-a1 | ✅ | 正式 HelloWorld 423 类 / 2–3 s（≤360 余量转 a5） |
| C1d-a-a2 | ✅ 62f46bb2 | b4669206 合入，抽查 c1da-b4669206 9/9 |
| C1d-a-a2续 | 🔄 | initPhase2 膨胀用真实 `--cut` 定位 → 早退检查按分析期事实求值 → `[[boot_init.phases]]` → boot layer 步骤 2–5；闸门以档案规模计（基线 3609） |
| C1d-a-a4 | ✅ b124e5ac | TestCharsetNamedStreams，抽查 c1da-f2bdcf6e 通过（计划 §21.4） |
| C1d-a-JCA | ✅ 51a4d8c5 | JCA 种子修复（抽查 9/10 + 6/6）；代价 StockTrans 3141→3283，+134 来自 jar 签名校验路径 |
| C1d-a-jar签名 | ⏳ | jar 签名校验路径收窄（4 个算法名不可定的请求点） |
| C1d-a-乙 | ✅ fca1643b | URL.getURLStreamHandler 按键闸门；闭包不变，根因 URL host 无逐对象精度（§22.10），后续归「URL 协议可靠口径」 |
| C1d-a-甲 | ⏳ | jar/URL 来源甲 class-path（计划 §22.2） |
| C1d-a-a5-4 | ⏳ | 闭包膨胀收窄，终态 DeepCopy ≤1640，pkcs11 / smartcardio / defineClass0 所在类不入闭包（计划 §21.5）；s1 / s2 ✅，余 a5-4b / a5-4e / a5-4f |
| C1d-a-a5 | ⏳ | OOB 关系型边界推理 a5-1 → a5-2 → a5-3，HelloWorld 目标 ≤371 |
| C1d-a-a3 | ⏳ ◀── a2 | `#[jvm_boundary]` 归零，审计数 86→0；拆分与验收见计划 §21 / §21.7 |
| C1d-a-precheck | ⏳ | 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3） |
| a3-T 虚拟线程终态（a3t-vthread） | 🔄 T1–T5 ✅ a78cccef | VirtualThread 字节码翻译 + Continuation 有栈协程；交接见计划 §21.8.5 |
| a3-T-T6 | ⏳ | 规模指标：10 万虚拟线程 ≤10 s / ≤2 GiB（现 14.6 s / 2.66 GB，草稿未提交） |
| a3-T-T1b审计 | ⏳ | 手写运行时无界递归审计（T1b 栈检查注入 65b6ecb3、T1b-2 叶方法豁免 40193ed3 已合入） |
| a3-T-pinned | ⏳ | TestContinuationPinned parkNanos 早返偶发需查 |
| C1d-b 反射收窄（c1d-pick） | 🔄 2026-10-02 | b0 / T3–T7 / T2 / b3 已合入；余 T2 余项、b1、b2；过程见历史 §E / §J |
| C1d-b-b1′ | 🔄 | ArrayList.writeObject 分派臂（计划 §4.6）：T2 ✅ 35c5f0ee（抽查 13/14，TestFieldHandleProvenance 为 OOM 归声明层拆分线） |
| C1d-b-T2余 | ⏳ 待派新代理 | getDefaultSerialFields 收窄、4b；StockTrans 目标按 JCA 后基线重定（计划 5.7）；StockTrans 既有失败 writeObject 反射臂待 T2 2e1d3355 |
| C1d-b-b1 | 🔄 S2 ✅ 0d7dd2a5 | 序列化收窄：目标 DeepCopy ≤1640、fold_props ≥42；大值集来自未知接收者字段视图 |
| C1d-b-b3余 | ⏳ | URL$DefaultFactory 反射构造器扇出收窄（b3 原「余」项之一；registerNatives 开放接收者 toString 已由 6294755d / ee52c596 收窄，此项未见完成记录） |
| C1d-b-b2 | ⏳ ◀── why2-93e0f28e 取证 | 任务 2 |
| C1d-b-jndi | 🔄 第 1 步 ✅ 26720aff | TestJndiNoProvider 冷闭包 198.7 s→126 s（600 s 上限不放宽）；余修法 B（按调用点配对 + Const 形参保留 Src::Param + flow-batch×seed 集合不变性守护），计划 `docs/plans/2026-10-03-jndi-transpile-perf.md` |
| T1 档案化（t1-profile） | 🔄 | 1a / 1b / 第 2 步方案 ✅；M1 本地合入待推送；下一步 M2 / M3 |
| T1-1a | ✅ 90398dc8 | 多根开放世界分析 + 档案键 / 内容摘要 + rava profile（27 例档案 3244 类，抽查 10/10） |
| T1-1b | ✅ 10cfb657 | 按档案生成 JDK crate、java_meta 拆 JDK 表 + 用户登记（抽查 14/14） |
| T1-2 方案 | ✅ 6aa69280，合入 53f32664 | 直接 rustc 链接档案；语料模式 panic=unwind、生产 --release fat LTO、档案 crate 按 jmod 模块切分 |
| T1-M1 | ✅ 188d67ac，本地合入 9d587416 | jmod 模块映射与 import 过滤，抽查 12/12；待推送 |
| T1-M2 | ⏳ | 按模块切 crate（计划 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md`） |
| T1-M3 | ⏳ ◀── M2 | 按模块登记（同计划） |
| S7 统一对象句柄 + 每类静态描述符 | 🔄 | D7 批准取法 B；计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` |
| S7-0/S7-1 | ✅ b0166702，本地合入 8c218a72 | 描述符 / 子类型判定改读描述符，抽查 16/16；待推送 |
| S7-2…S7-5 | ⏳ | 见同计划 |
| boot layer | 🔄 第 0 步 ✅ 27dfb419 | ModuleBootstrap 引导建层；第 1 步起 ◀── C1d-a a2 续（FS-C2 ✅）；验收 TestModuleLayerDefine |
| regress2 遗留 | ⏳ ◀── C1d-a a2 | Object.wait 帧行号、过渡 <init> 帧 |
| C4 收官 · 全量 e2e | ⏳ | JDK 21 ⊇ 1029 例基线；以上全部合入后 |
| 测试分发 | ✅ 2026-10-02 起 | 全部 e2e 与重命令作业经 `server_maintenance/rava/distribute_tests.py`（`--spot` / `--job`）在 8 台服务器执行；本机只做编译 / 构建 / 单测 |

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

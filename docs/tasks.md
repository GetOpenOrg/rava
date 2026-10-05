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

## 🌳 任务依赖树（2026-10-04，集成分支 rust-closure-analyzer 21fc601b，已推送；main 已快进到 21fc601b）

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
├─ 【当前】阶段 C 收官：闭包分析器（rust-closure-analyzer）── 用户 2026-10-01 决定先做完本阶段
│   │
│   ├─ 🔄 闭包引擎提速（用户 2026-10-04 定先做）
│   │     ├─ ✅ http-perf A/B/C（c5741dfe，合入 a068b87a）：TestHttpLoopbackSync 本机 404→285 s、服务器 630 s
│   │     ├─ 🔄 engine-order 闭包结果与哈希顺序无关（V9，e435ace5）：种子 0/1/2 一致、类数 ≤ main ✅；第三阶段把 JNDI / HTTP 闭包耗时（+33% / +17%）压回 main 以下
│   │     ├─ ⏳ 逃逸对象上下文收拢（V10）：TestHttpLoopbackSync 服务器闭包 ≤60 s ◀── engine-order
│   │     └─ ⏳ URL 协议可靠口径（c1d-urlhost 9087cf1c）◀── 引擎提速达标
│   │
│   ├─ 🔄 C1d-a 去截断（c1d-p0，2026-10-01-c1d-closure-bloat.md；原节点见历史 §D）
│   │     ├─ ✅ a1 具体求值器 engine/concrete/：正式 HelloWorld 423 类 / 2–3 s（≤360 余量转 a5）
│   │     ├─ ✅ a2 合入 62f46bb2（b4669206，抽查 9/9）
│   │     ├─ ✅ a2 续：早退检查按分析期事实求值（F1/F2）、[[boot_init.phases]]（6666c19b，锚点留空）；锚点代价实测见 c1d §25（档案 +约 20 类，单例 HelloWorld 469→3190）
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
│   │     ├─ ✅ 第 1 步：[[boot_init.phases]] 清单与按锚点作根（6666c19b）
│   │     └─ ⏳ 第 2–5 步 ◀── Class 实例方法按接收者镜像求 classLoader/module、容器元素类型、实例汇合点（c1d §25.4，判据 HelloWorld ≤569 类）；验收 TestModuleLayerDefine 原样通过
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
│   │     2026-10-05 用户提前开工、走 V12 终态：任务书 2026-10-05-junit-e2e-deps-task.md（J0 基线 → J1 = V12-0 → J2 = V12-1 删 --lib → J3 形态接线 → J4 10 例跑通；V12 决策 3 / 4 / 5 取 A）
│   │
│   ├─ ⏳ R1 运行性能：超时用例（标杆 LynchBell 等 12 例）不改测试、不放宽时限
│   │     （2026-09-30-optimization-directions.md §三.4）◀── C4 收官后排期
│   │     ├─ ✅ GraalVM 参照基线入库：报告 docs/reports/2026-10-04-graalvm-baseline.md、scripts/graalvm_bench.sh、release 终态 ≤ native 逐例阈值
│   │     └─ 🔄 逐例耗时起点：服务器 timing-dbg / timing-rel-10795076 与 Linux GraalVM gvm-linux-bd52b537 已完成待汇总；本机 mac rava-release 跑批进行中（运行 / 构建 / 二进制三项），汇总后写报告 §六
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
│   │     ├─ ✅ crate-split 声明层拆分（db45a65c）：抽查 14/14，合入 3e739189（已推送）
│   │     └─ 🔄 SCC 拆分（D8）与 S7-4 并行，实施中：b51f9531 / b093069f 已合入；9 个 release OOM 例声明层 ≤ 8.1 GB，余下 OOM 是用户 bin 的 fat LTO 全程序链接（计划 S7 §9.7）
│   ├─ 🔄 S7 统一对象句柄 + 每类静态描述符（D7 批准取法 B；2026-10-04-s7-object-handle-descriptor.md）
│   │     ├─ ✅ S7-0 / S7-1 s7-desc（b0166702）：抽查 16/16，合入 8c218a72（已推送）
│   │     ├─ ✅ S7-2a 类 wrapper 单字段句柄（合入 cfe37f90；TSDS 声明层 7597 MB）
│   │     ├─ ✅ S7-2b Object 直接持对象 / 删 blanket From（6a784b50 合入）
│   │     ├─ ✅ S7-2c 接口载体收为句柄（63bc9213）
│   │     └─ ⏳ S7-3 … S7-5 ◀── S7-2
│   ├─ 🔄 T1 跨测试编译复用（决策 ✅ 2026-10-03 四项全采纳，2026-10-01-cross-test-compile-reuse.md）
│   │     ├─ ✅ T1-1a 档案化分析（90398dc8）、T1-1b 按档案生成 JDK crate（10cfb657）
│   │     ├─ ✅ T1-2 方案 t1-link（6aa69280，合入 53f32664；2026-10-04-t1-step2-direct-rustc-link.md）
│   │     ├─ ✅ M1 jmod 模块映射与 import 过滤（module-m1 188d67ac）：抽查 12/12，合入 9d587416（已推送）
│   │     ├─ ✅ M2 按模块切 crate（a62793fd 合入）──▶ M3 按模块登记
│   │     ├─ ✅ V12 第三方依赖分层方案（a2fdfbca，合入 5c995939；7 项待用户决策）──▶ V12-0 模块归属修正
│   │     ├─ ⏳ L2–L4（同计划）
│   │     └─ ⏳ T4 生成器只构建一次再分发（待服务器核实）
│   ├─ 🔄 二进制体积（2026-10-04 用户交主会话推进；2026-10-04-binary-size.md；HelloWorld 14.4 MB → ≤3 MB）
│   │     ├─ ✅ B0 基线重测 + B1 元数据按档案按类裁剪、字符串池编码（2682139c）
│   │     ├─ ⏳ B2 栈还原按地址查表 + strip=symbols ◀── B1
│   │     └─ ⏳ B3 体积档位评估（opt=s / z 性能对照，交用户决定）◀── B2
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
engine-order（V9）──▶ V10 ──▶ URL 可靠口径
C1d-a a2 ✅ ──▶ a2 续 / a3 jvm_boundary 归零 / a5-4 收窄 ──────────┐
C1d-b b1′ T2 余项 / b1 序列化收窄 ─────────────────────────────┤
boot layer 第 1 步起（◀── a2 续）─────────────────────────────┼──▶ C4 全量 e2e ──▶ S6 ──▶ S7 ──▶ JUnit A ──▶ B/C ──▶ 真实项目 pilot ──▶ 产品化
regress2 遗留（◀── a2）───────────────────────────────────────┘        └──▶ 优化线（P8 / V / S7·T1 / R1 / JDK 25）
```

当前（2026-10-04 晚）：M2 / S7-2a·2b·2c / B1 已合入 main；在途 engine-order 第三阶段（闭包耗时）、C1d-a a2 续、B2；C4 的前置剩 C1d-a（a2 续 / a3 / a5-4）、C1d-b（T2 余项 / b1）、boot layer 第 1 步起。2026-10-02 版说明见历史 §I。

---

## 🧭 任务分级（2026-10-06 用户采纳）

用户定：功能优化先放一边，先完成闭包分析器主线（正确、确定、最小，过 C4）。派发只从「继续」项选，空出名额优先 C3 / C5。

| 类 | 项 | 处理 |
|---|---|---|
| A 正确性缺陷 | A1 TestModuleLayerDefine NPE（引导映像第 3 步）、A2 TestClassModuleFace 具名模块（第 4 步）、A4 JNDI / HTTP 转译超时、A5 C4 全量 e2e | 继续 |
| A 正确性缺陷 | A3 URL 协议可靠口径（c1d-urlhost） | 依赖提速，随 D2 搁置 |
| B 架构终态 | B1 a3 `#[jvm_boundary]` 归零、B2 引导映像求值器第 2–7 步 | 继续 |
| B 架构终态 | B3 虚拟线程余项（T6 规模 / T1b 审计 / pinned 偶发）、B4 T1-M3 按模块登记 | 缓 |
| C 闭包精度与规模 | C1 c1d-elem、C2 clsfact、C3 a5-4 收窄（DeepCopy ≤1640）、C5 C1d-b（T2 余 / b1 / b2 / b3 余） | 继续 |
| C 闭包精度与规模 | C4 a5 关系型边界推理（HelloWorld ≤371）、C6 jar 签名收窄 / jar·URL class-path | 缓 |
| C 闭包精度与规模 | 共享汇点（c1d-sink） | 已收口（§28.10），≤569 由引导映像达成 |
| D 分析性能 | D1 处理顺序无关（分支 closure-order-free，定性为正确性） | 继续 |
| D 分析性能 | D2 枢纽翻新 / 延迟站点重跑等结构改造、D3 在线节点合并 | 暂停（V12 后提速线暂停） |
| E 编译资源 | E1 B4 内存友好缺省构建档（分支 build-memsafe，16 GB 机器全部可构建为硬约束） | 继续 |
| E 编译资源 | E2 D8 声明层分段 | 缓（视 B4 结果） |
| B 架构终态 | B5 第三方库通用机制：JNI ABI 层（库自带 native 原样调用）、构建期捕获运行期生成类（三方依赖分层 §3.6；rava 仓库不放任何第三方库专属内容，库配置归用户项目） | 缓（10-06 用户定） |
| F 纯优化 | 二进制 ≤3 MB、S7-3～5、VT `instanceof` / `checkcast` 走 `__ClassDesc`、IR 结构化收敛 / TypeIR G4 | 暂停 |

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途分支的当前状态。原表长单元格见历史 §J。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| engine-order 闭包结果与哈希顺序无关（V9） | 🔄 e435ace5（修复本体 7426e583） | 非字面量 `Class.forName` 收敛后再判、名集完整才加载；`Provider$Service.getImplClass` 由 seeds.toml 声明不按名加载；字段名配对只认 String 形参。6 例种子 0/1/2 闭包完全一致（TestSerialUserGenericCallbacks 恒 3391，main 种子 0 为 4167），`closure_independent_of_hash_seed` 稳定；类数 JNDI 3841→3841、HTTP 5443→5442。未达标：`rava closure` 耗时 JNDI 184→281 s、HTTP 293→342 s，第三阶段 profile 后压回 main 以下再合入；抽查 order-e435ace5 进行中 |
| 逃逸对象上下文收拢（V10） | ⏳ ◀── engine-order | TestHttpLoopbackSync 服务器闭包 ≤60 s，HTTP 两例通过，闭包集合变化逐项论证 |
| URL 协议可靠口径 | ⏳ c1d-urlhost 9087cf1c（本地 worktree 已删，分支保留在 origin / github） ◀── 引擎提速达标 | 证 file URL host 为 "" / localhost 以杀 ftp 分支；引擎提速期间口径偏差暂容忍 |
| crate-split 声明层拆分 | ✅ db45a65c，合入 3e739189 | 抽查 14/14；已推送 |
| D8 声明层按签名 SCC 分段 | 🔄 与 S7-4 并行，实施中：b51f9531（合入 2602f409）、b093069f（合入 10314222） | 镜像链多 crate、上限 650 类/段；release 9 例底段 7.9–8.1 GB、上段约 1.27 GB（原约 11.9 GB OOM）；单测 scc-ut-b093069f rc=0；9 例 rava compile 仍 rc=1：OOM 是用户 bin 的 release fat LTO + cgu=1 链接（scc-rel4 定位），不在声明层；ubuntu 前后对照 FWord 声明层 9618→7909 MB（见 S7 计划 §9.7）；≤1.3 GB/crate 依赖 S7 |
| C1d-a 去截断（c1d-p0） | ⏸ 未派（2026-10-04 优化线优先期间暂停）· 2026-10-03 | 闭包闸门 P2/P3；StockTrans 3283 / DeepCopy 3278 类，目标 DeepCopy ≤1640；子项见下，过程见历史 §D / §J |
| C1d-a-a1 | ✅ | 正式 HelloWorld 423 类 / 2–3 s（≤360 余量转 a5） |
| C1d-a-a2 | ✅ 62f46bb2 | b4669206 合入，抽查 c1da-b4669206 9/9 |
| C1d-a-a2续 | ✅ | 早退检查按分析期事实求值（F1 / F2，§23）、`[[boot_init.phases]]` 清单与按锚点作根（6666c19b，锚点留空）。锚点代价实测（c1d §25，分支 c1d-boot）：档案并集 7886 类只增加约 20 个 `jdk/internal/module` 类，但 `Class.module` 锚点使所有用例都作根，单例 HelloWorld 469→3190；具体求值器执行 initPhase2 不可行（§25.3）。boot layer 第 2–5 步转为依赖 §25.4 的三项精度 |
| C1d-a-a4 | ✅ b124e5ac | TestCharsetNamedStreams，抽查 c1da-f2bdcf6e 通过（计划 §21.4） |
| C1d-a-JCA | ✅ 51a4d8c5 | JCA 种子修复（抽查 9/10 + 6/6）；代价 StockTrans 3141→3283，+134 来自 jar 签名校验路径 |
| C1d-a-jar签名 | ⏳ | jar 签名校验路径收窄（4 个算法名不可定的请求点） |
| C1d-a-乙 | ✅ fca1643b | URL.getURLStreamHandler 按键闸门；闭包不变，根因 URL host 无逐对象精度（§22.10），后续归「URL 协议可靠口径」 |
| C1d-a-甲 | ⏳ | jar/URL 来源甲 class-path（计划 §22.2） |
| C1d-a-a5-4 | ⏳ | 闭包膨胀收窄，终态 DeepCopy ≤1640，pkcs11 / smartcardio / defineClass0 所在类不入闭包（计划 §21.5）；s1 / s2 ✅，余 a5-4b / a5-4e / a5-4f |
| C1d-a-a5 | ⏳ | OOB 关系型边界推理 a5-1 → a5-2 → a5-3，HelloWorld 目标 ≤371 |
| C1d-a-a3 | 🔄 U0–U3 / L1（4 项）/ X1 / X2（CDS、FileSystems）✅ 分支 c1d-a3 e336f8ef | `#[jvm_boundary]` 归零。HelloWorld 审计 77→29，全仓属性 123→33，各项闭包类数持平或下降（L1 +10 类为本地库装载路径本身）。余项阻塞：C ◀ 反射调用精度；L2、L1 余 2、SecurityManager ◀ boot layer 第 2–3 步；V 待定字段钩子或急切引导；JceSecurity ◀ java.home NIO 虚拟层。见计划 §21.9 |
| C1d-a-precheck | ⏳ | 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3） |
| a3-T 虚拟线程终态（a3t-vthread） | ⏸ 未派（2026-10-04 优化线优先期间暂停）· T1–T5 ✅ a78cccef | VirtualThread 字节码翻译 + Continuation 有栈协程；交接见计划 §21.8.5 |
| a3-T-T6 | ⏳ | 规模指标：10 万虚拟线程 ≤10 s / ≤2 GiB（现 14.6 s / 2.66 GB，草稿未提交） |
| a3-T-T1b审计 | ⏳ | 手写运行时无界递归审计（T1b 栈检查注入 65b6ecb3、T1b-2 叶方法豁免 40193ed3 已合入） |
| a3-T-pinned | ⏳ | TestContinuationPinned parkNanos 早返偶发需查 |
| C1d-b 反射收窄（c1d-pick） | ⏸ 未派（2026-10-04 优化线优先期间暂停）· 2026-10-02 | b0 / T3–T7 / T2 / b3 已合入；余 T2 余项、b1、b2；过程见历史 §E / §J |
| C1d-b-b1′ | ⏸ 未派（2026-10-04 优化线优先期间暂停）· | ArrayList.writeObject 分派臂（计划 §4.6）：T2 ✅ 35c5f0ee（抽查 13/14，TestFieldHandleProvenance 为 OOM 归声明层拆分线） |
| C1d-b-T2余 | 🔧 c1d-b-t2 | 类镜像子类型判定收窄（计划 §七）：StockTrans 目标按 6f93f1c6 重定为 ≤3380 / ≤20813，实测 3386/20860→3380/20813；可序列化字段可读面贡献 0（§6.2）；StockTrans 已通过，writeObject 反射臂一项消解；余 4b |
| C1d-b-b1 | ⏸ 未派（2026-10-04 优化线优先期间暂停）· S2 ✅ 0d7dd2a5 | 序列化收窄：目标 DeepCopy ≤1640、fold_props ≥42；大值集来自未知接收者字段视图 |
| C1d-b-b3余 | ⏳ | URL$DefaultFactory 反射构造器扇出收窄（b3 原「余」项之一；registerNatives 开放接收者 toString 已由 6294755d / ee52c596 收窄，此项未见完成记录） |
| C1d-b-b2 | ⏳ ◀── why2-93e0f28e 取证 | 任务 2 |
| C1d-b-jndi | ⏸ 未派（2026-10-04 优化线优先期间暂停）· 第 1 步 ✅ 26720aff | TestJndiNoProvider 冷闭包 198.7 s→126 s（600 s 上限不放宽）；余修法 B（按调用点配对 + Const 形参保留 Src::Param + flow-batch×seed 集合不变性守护），计划 `docs/plans/2026-10-03-jndi-transpile-perf.md` |
| T1 档案化（t1-profile） | 🔄 | 1a / 1b / 第 2 步方案 / M1 / V12 方案 / M2 ✅；M3 |
| T1-1a | ✅ 90398dc8 | 多根开放世界分析 + 档案键 / 内容摘要 + rava profile（27 例档案 3244 类，抽查 10/10） |
| T1-1b | ✅ 10cfb657 | 按档案生成 JDK crate、java_meta 拆 JDK 表 + 用户登记（抽查 14/14） |
| T1-2 方案 | ✅ 6aa69280，合入 53f32664 | 直接 rustc 链接档案；语料模式 panic=unwind、生产 --release fat LTO、档案 crate 按 jmod 模块切分 |
| T1-M1 | ✅ 188d67ac，合入 9d587416 | jmod 模块映射与 import 过滤，抽查 12/12；已推送，分支 module-m1 已删 |
| T1-M2 | ✅ a62793fd | 按模块切 crate（计划 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md`）。服务器声明层峰值 DeepCopy 8214→7618 MB、TestSerialDefaultSuid 8160→7572 MB，端到端 11:30→10:28、11:22→10:48，HelloWorld 持平（1389→1404 MB 噪声内）；抽查 13 过 / 5 败均为 main 既有。未做：第三方 SCC 合并（V12）、java_meta 按模块（M3）、build.rs 只留 java_base_decl |
| T1-M3 | ⏳ ◀── M2 | 按模块登记（同计划） |
| T1-V12 方案 | ✅ a2fdfbca，合入 5c995939 | 第三方依赖分层（`docs/plans/2026-10-04-third-party-dependency-layering.md`）；7 项决策待用户确认；V12-0 模块归属修正（多版本 jar module-info、JDK 包遮蔽）可先做 |
| S7 统一对象句柄 + 每类静态描述符 | 🔄 | D7 批准取法 B；计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` |
| S7-0/S7-1 | ✅ b0166702，合入 8c218a72 | 描述符 / 子类型判定改读描述符，抽查 16/16；已推送，分支 s7-desc 已删 |
| S7-2a | ✅ 707fde3c，合入 cfe37f90 | 类 wrapper 单字段 `__r`（句柄 + 视图指针）、Default 不分配、null 判 `is_none()`；服务器 HW 声明层 1408→1359 MB、TSDS 8192→7597 MB；抽查 16/16 + 13/13 |
| S7-2b | ✅ 6a784b50 | `Object` 直接持对象、删 blanket `From` 改逐类 `From<X> for Object`、null 为带描述符的 typed null、downcast 审计 64 处修 3 处；服务器声明层峰值 HelloWorld 1359→1233 MB、TestSerialDefaultSuid 7597→6654 MB；抽查 16/16，与 M2 合并后 8/8 |
| S7-2c | ✅ 63bc9213 | 接口载体改持 `__IfaceRef<dyn I__VTable>`（Object 句柄 + 构造时一次算定的接口视图指针；不用 `__Ref` 因载体须 Deref 到 Object 且 null 带接口静态类型），`__interface` 改 `&self` 填视图槽与 `__erased_vtable` 同形，删 `__iface_vtable` 与全部 `__Shared<dyn I__VTable>`；lambda 闭包存储 `__Shared<__DynFn>` 按擦除签名而非接口，保留；手写层 `from_any` 仅 Throwable 栈帧一处，无需改。服务器声明层峰值 TSDS 6654→6153 MB（−7.5%）、HelloWorld 1233→1242 MB 持平；展开体量 12.78 / 80.63 MB 基本不变；抽查 20/20 |
| S7-3…S7-5 | ⏳ ◀── S7-2 | 见同计划 |
| 二进制体积 | 🔄 | 用户 2026-10-04 交主会话推进；计划 `docs/plans/2026-10-04-binary-size.md`；HelloWorld release 14.4 MB → ≤3 MB |
| BS-B1 | ✅ 2682139c | HelloWorld release 元数据 3,506,260→255,720 B（B0 的 7.3%），二进制 15.1→11.8 MB；抽查 18/18（含注解数组 / 嵌套注解 / CallerSensitive 回归修复：L1 用户类与注解类型保留类级注解，注解解析可达时闭包内注解类型带方法表）。 |
| BS-B2 | ✅ 8f5ad0c5（合并 6f9b189b） | 栈还原按地址查表（rava-link 链接期 pcmap），release 加 strip=symbols；release 验证 b2-8f5ad0c5-rel1 5/5，HelloWorld release 7,410,488 B（ubuntu）。 |
| BS-B3 | ✅ 38c17d97（合并 5bc31469） | 可选体积档 `--release-small`（opt s，不设 z）。对照（ubuntu b3-bench2-b064f315）：s 档二进制 −19~24%、构建 −25~32%，计算用例运行 +13%（ARM +22~40%），按 5%/15% 规则维持 opt 3 缺省。16 GB 机器上 opt 3 构建大闭包用例 OOM（峰值 14.5–15.7 GB；b3-mem16-b064f315 4/8 OOM），s 档 8/8 可构建，交用户决定缺省档（binary-size §五）。 |
| boot layer | 🔄 第 0 / 1 步 ✅ 27dfb419 / 6666c19b | ModuleBootstrap 引导期建层。第 2–5 步依赖：`Class` 实例方法按接收者镜像求值（✅ c1d-clsfact 69d1c73d，c1d §26：classLoader 逐镜像、`Class.module` 锚点按接收者；锚点口径 HelloWorld 仍为 3190，膨胀是 `boot2` 内共享汇点饱和，`arraycopy` / `append(Object)` / Unsafe 引用写）、容器元素类型、实例汇合点（c1d §25.4 / §26.3，判据 HelloWorld ≤569 类，差 2621）；验收 TestModuleLayerDefine 原样通过，TestProtectionDomainFaces / TestClassModuleFace / TestSetAccessibleBoundary 随第 2–3 步解决 |
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

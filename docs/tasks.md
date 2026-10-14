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

### 10-11 晚收拢：暂停派发后的剩余工作（用户 10-11 令：暂停派新子代理，本批完成后剩余工作落文档）

本批子代理已全部交回，下列各线的剩余项按优先级排列。恢复派发后从第 1 条起取。各条的细节以括注文档为准。

**批次状态**
- batch-1011h 已放行（main = rust-closure-analyzer = c6b126cc）。
- batch-1011i（0b91182b）：抽查 36/36，单测 A / B / P 通过，**单测 S 失败**（`closure_independent_of_hash_seed` 断言 closure_cli.rs:150，种子 0 与 1/2 闭包集合不同；作业时 ref dd1445e6）。重跑作业 b1011i-seed 不过滤输出，用于定位差异类。
- batch-1011j（74919e58，叠 1011i）：单测四作业全过、抽查 57/57，已放行（10-11 晚，main = rust-closure-analyzer = 74919e58），1011i 内容随之并入。
- batch-1011k（叠 1011j，含 review-soundness + resolve 单测临时目录同名互删修复 fda1c09b）：单测四作业全过、抽查 58/58，已放行（10-12，main = rust-closure-analyzer = fda1c09b）。

**剩余工作**
1. **种子无关性回归（fix-seed-reg，10-12 已修复）**：`closure_independent_of_hash_seed` 在 batch-1011i（0b91182b）失败——TestSerialLookupPairing 种子 0 闭包比种子 1/2 多 `ProtectionDomain$JavaSecurityAccessImpl` 与 `JavaSecurityAccess` 两类（2711 vs 2709）。九轮诊断（seedreg-diag1..9）+ 修复轮插桩（seedfix-diag1）收窄出**终局根因**：`ObjectStreamClass$2.run@336` 的 `putfield domains` 写入 `@333 getProtectionDomains(cons, cl)` 返回值，而 `getProtectionDomains@13 System.getSecurityManager/@16 ifnull` 守卫**无返回事实不折**，其 rvals 在两种子下于 Const(Null)/Top 分叉，种子 0 某轮 Unknown 答复经 putfield 以 `PV::Top` 并入全局 `fvals[domains]`（格只升不降）→ `newInstance@15 ifnull` 展开 → `@38 getJavaSecurityAccess` 入事件集 → 字段节点首建、映像值 `Image(impl)` 并入 → `@65` 牵出两类。domains 真程序恒 null（原生二进制无启动器、无人调 setSecurityManager），种子 1（2709）是正确侧。前轮「domains 只被无人调用私有方法写入」说法有误（调用方在内部类 $2.run）；按对象 ⊥（Never）读两子均为 0 次（非根因）；deser 放开被空 serialPersistentFields 排除。**修法（b898b399）**：清单 `[facts.returns]` 增 `System.getSecurityManager = "null"`——原生二进制安全管理器恒缺位；调用点事实先于汇合格答复，@16 守卫恒折、getProtectionDomains 恒返 Null、domains 恒写 Null，与种子及轮次无关（清单即边界，非类名特判、不依赖固定种子 / nogc 掩盖）。**验收**：seedfix-accept4 集合口径 PASS（TestSerial 2709×3 且等于 0b91182b 种子 1；HelloWorld 641×3 / CollectorsDemo 662×3 / DeepCopy 2707×3 与基线逐种子一致）；单测 seedfix-unit4（closure_independent_of_hash_seed / closure_independent_of_order）；generator cargo check 过。详见 `docs/plans/2026-10-11-seed-regression-fix.md` §五；失败模式 `seed-order-osc-newinstance-domains` 已置 fixed。分支 fix-seed-reg 已推 origin/github。遗留：全局字段格被暂时 Unknown 污染为 Top 后不可撤回的一般现象（见交接文档 §六）。
2. **G1 闭包不收敛**（S0 A 34.9 GB、sb3 30.6 GB 被杀，主因是方法上下文）：**10-11 晚 g1-ctx-sel 交付（未合入，性能计划 §4.13）**：实现选择性上下文 ctx_flat（结构判据、位开关：继承 / 接收者 / lambda / 反射池静态 / 宽档）+ 反射池静态成员短路 + ctxflat/wip 诊断。实证：继承与接收者两路**单独**即令 DeepCopy 超时 / 18 GiB OOM、lambda 路 `boot_image_data.live` +1——逐上下文常量格（pvals/pstr/sret）是语料精度的主通道，节点合并必失精度；**只有池静态短路过四例集合口径（缺省启用）**，但语料仅 −46 上下文。全开上限曲线：sb3 / S0 A 的 RSS 与方法增长速率压慢约 3.5–4 倍（sb3 1107 s 31 GiB 被杀 → 全开 3868 s 才到同位；S0 A 4201 s 看门狗超时于 30 GB）——**仍线性不收敛，≤20 万上下文 / ≤16 GiB 未达成**。240 s 尖峰线索：与 fwork（字段工作表）突发同步（3880→3949 s fwork 12→20838、RSS +4 GB），watch 表 64.5 万条。下一步（按序）：常量保持的合并（节点身份与常量格身份分离）、堆上下文选择性（宽分配点停止堆链，需先扫语料站点变体上界）；确定性两项在缺省 8 下通过（g1ctx-det8，3370 s）。补跑扫描缺的 28 例（11 例类路径问题，17 例需 ≥16 GiB 槽）仍未动。矩阵 #12 spring-beans sb1–sb3 与 S0 Boot 受此阻塞。见性能计划 §4.12 / §4.13。
3. **S7 K3 方法 trait（s7-k3，未合入）**：**10-12 用户裁决：峰值阈值放宽至 ±10%，K3 已随 batch-1013 合入放行**（f7b69314，含 K6 前置项两提交 8efa6444+5c3c8bdc 与 b1013-fix 回归修复 abdd523b）。原任务（成因分离 / DeepCopy 复测 / 全量单测与抽查 / K6 前置项）已由代理 C 完成，见 S7 计划 §9.9。**K6 本体（s7-k6 @ fb938aa3，10-12 交回待批）**：`java_class!` 块拆类型标记层（`#[rava_layer = "marker"]`）——struct/身份 impl/`From` 三族/描述符进同 crate 标记文件 `<stem>_m.rs`（K4 未做故不独立成 crate，过渡形态，偏差记录在案）；声明文件块文本与 D8 分段输入不变（切段逐类一致已实证：三例含 DeepCopy 全同）；K6pre 遗留按新形态归位（`is_jvm_null` 随标记层，访问器/`__init_on`/`valueOf_*` 留声明层固有，接口载体不动）。已过：宏单测 41/35（s7k6b-macro2）、emit 99、check 0 警告、确定性两项（本机 release 3655 s）、HW 三种子闭包全同、五例链接运行逐字一致、峰值 A/B +2.8%~+8.5%（±10% 内）；peak ✅（sg1：全部 crate ≤+9.0%，判定项 HW decl +3.0% / CD decl +1.9%）；det 本机已过、服务器作业在跑作重复确认；seed 三败均为作业基础设施（非集合差异），续作入口见 S7 计划 §9.9「K6 交接」节。4. **运行时性能（三测点对照 JVM）**：**10-11 已深剖**（分支 rt-perf-prof，报告 [`2026-10-11-rt-perf-profiling.md`](reports/2026-10-11-rt-perf-profiling.md)，作业 rtpp-prof1（基线）/ rtpp-verify3 / rtpp-dbg2（回归与 A/B），基线 kr1 7b0bf81a）：
   - `Method.invoke`：invoke 段 **1.76 µs/调用**（JVM 31 ns，56 倍）；热点前三 = `__member_key` 每次重建分派键的 String 簇 ~17%（`method_impl.rs:36`）、分配器簇 ~16%、分派协议簇 ~10%（`is_static_descriptor` 2.27% 等）。微修复已入（9157a73e → 57eda4ee）：`__name_assignable` 二分化 + `rd::unbox_i32` 的 Integer/Short/Byte 结构化取值（P-B 收窄定稿：Float 的翻译 hashCode 对 3.0f 应答 0 被实测否决、Character/Boolean 无实参级见证，维持文本；A/B invoke 段 −11.6%（跨机约 −20%）、整例 4.3 → 3.66 s）。**10-12 分派键免重建已实施（invoke-key-opt，c0713ed8，报告 §二.3）**：表构造成员对象 slot 即方法表行号，(clazz 身份, slot) 首用经表行直取 `&'static` 键 + 进程级缓存（类镜像永生为回收论证），非表形态回落原重建（驻留）；同机 A/B（sg1，ikopt-base/after）invoke 段 1.515 → 0.646 µs/调用（−57.4%）、idRef 1636 → 587 ns、分配器簇占比 ~15.3% → ~7.3%、debug 档整例 42.9 → 25.2 s，键缓存哈希自身代价 ≈ 2.4%；回归 9 例（rtpp-verify4 清单 + TestWidenFacesInvoke / TestWrapperValueFaces）PASS（ikopt-verify）；§二 43 µs vs 4.3 µs 的 10 倍差 = debug（perf_hotpath）vs release（perf_rt_bench）口径差，已闭合。下一步小项：键缓存轻量哈希器、`is_static_descriptor` 行内线性 find 二分化；再下一层 = 分派协议簇（双重 static 判定）。**10-13 两小项已实施（ikopt2，f2232b29，报告 §二.4）**：METHOD_KEYS 换 FxHash 风格乘法散列（原 RandomState+Sip13 ≈2.4%）；is_static_descriptor 行内查找改进程级 (name,descriptor) 有序索引（行序=slot 冻结不可重排，索引自冻结表派生、不动发射，≤8 行小类仍顺序扫紧凑三元组）；编译验证 ik2-compile PASS（jp1，端到端 ok=True）；A/B（ik2-base/after，RldcSegments 口径）与 9 例回归（ik2-verify）作业已派发——派发时段七台云服务器全被并行分发器占满，按退让循环轮候中，结果落 `cluster_results/job/ik2-*/`，同 tag 续接。
   - `rd::unbox_*`：文本往返（翻译 toString + UTF-16→UTF-8 + parse）≈ 全样本 4–5%；P-A 终态（ObjectVTable 原生值视图臂）归 S7 K 系设计。**10-12 fix-widen 补记**：P-B（hashCode 取值）与文本路径实测双双落在闭包外存根上（`stub.rs stub_body`：hashCode→Ok(0)、toString→二进制名；运行时经 ObjectVTable 桥的调用对闭包分析器不可见）——TestWidenFacesInvoke 的 widen-local-short 0/-299、widen-char IAE，TestUnboxWrapperFaces 第 40 行 IAE，rtpp §三.4 的 Float.hashCode(3.0f)=0 同根因（Integer 当时通过系 invoke 站点装箱原生盒先命中 downcast，Short/Byte 无实参级见证）。修法 = **P-C 承载字段直读**：`unbox_*` 对翻译包装类经实例字段协议（`__instance_field`，Field.get 同一数据面）直读 final 基本字段 `value`，不依赖方法链，Long/Double 按位精确（NaN/-0.0 位形保留），种类不符回落文本；无分配收益扩展到全部八包装类。分层见证 TestWrapperValueFaces（直呼翻译体 / 擦除 Object 桥 / 反射拆箱三层一致）。残余暴露面（未修，单列）：翻译类根虚方法槽在「类在闭包、方法不在链」时一律存根——经擦除 Object 接收者的虚分派落到其上（hashCode 恒 0、equals 存根 panic、toString 恒二进制名），根治面 = 闭包种子（seeds.toml 值视图族）或 P-A，随反射建模 / S7 K 系排期；失败模式 `unbox-root-virtual-stub` 已登记。
   - PartitionInteger release **48 s**（JVM 0.65 s，72 倍）：大头 = 每叶子管线构建 25–30% + 每元素约 5 重接口分派（`__IfaceRef::enter` ×2，`handle.rs:187`）；装箱 <3%；self 弥散无单点可修——生成器侧按调用点单态化 / PGO 或 S7 分派形态是正解，运行时侧不动。
   - jackson-core jc1 分派视图约 19% 与 String 跨 crate 不内联约 4% 仍归 S7 线口径（本步未复测）。
   - 遗留：43f37f63（us1）43 µs/迭代 与 本测 4.3 µs/迭代 差 10 倍未闭合（疑 lineage / 机差，见报告 §二）。
   - 见分诊报告 C 节余项（已更新）、无 GC 文档 §六。
5. **JDK 25 适配**：问题 A（StackFrameInfo 反射元素流，`FrameBuffer.at → ClassFrameInfo.declaringClass` 存根，归反射建模）；DeepCopy / DataEncryptionStandard 转译 OOM；合批后 JDK 25 全量复跑（`--reset-failed --jdk 25`），按其余失败分类处理。见 `docs/reports/2026-10-11-fix-jdk25-handoff.md`。**10-12 语义分诊（j25-sem）**：138 例三类分类与首簇修复已落 [`2026-10-12-jdk25-sem-triage.md`](reports/2026-10-12-jdk25-sem-triage.md)——主检出 error_logs 被 junit 复跑覆盖，改在 j25-sem 服务器重建（作业 j25s-rA/rB 宽超时复跑 127 例中）；先并 b1013-fix（valueOf E0599 批级回归，否则探测全污）；新根因 `escape-lambda-virtual-cycle`（逃逸分析 call_virtual↔call_lambda 无限互递归，JDK25 首触，304edfb1 已修待 j25s-fix1 验收）；release 口径下 debug-profile-too-slow 族实测转过（FractionReduction PASS）；OOM 类（junit 8 + DeepCopy/DES/Digester/HttpLoopback×2 + 14G 槽 ~62 例）名单待 rA/rB 的 oom 字段出齐交主会话大槽重跑。
6. **闭包分析器手写扫描器**：`&self` 辅助 fn 内 `self.X()` 保留 on_self（`hw-upcall-self-lost-in-helper`，现由手写侧规避）。
7. **无 GC 后续**：所有权推断 `Weak`（逃逸到旧对象但成环）；引用类语义随 `Rc` 释放触发；JDK 方法体内区域（需开放世界事实）；`thread_local_lint` 文档头登记 `region_slot` 理由。见无 GC 文档 §六-5。**10-13 设计轮已收口**（分支 nogc-design，设计先行未做大实施）：四子项实施设计成稿 [`2026-10-13-nogc-phase2-design.md`](plans/2026-10-13-nogc-phase2-design.md)——公共运行时原语（`__Obj` 弱计数，R1 内联 32B 头 / R2 侧表两路线）+ 推断证明义务 INV 与 W1 链接窗规则（路线 B v1 + C 清单 + A 终态）+ 观察弱 referent / 登记侧表（`REF_MARK` 位，与 MONITORS/PARKERS 同纪律）/ pending 链 + handler 线程 + poll 折叠撤销 + 开放世界五查询保守重解读（`user_extensible` 同源移植）与首批 `[escape]` 事实清单草案，逐子项配判据与实施分解；判据探针 `tests/perf/NogcWeakCycle.java` 入库，作业 nogcd-probe1（jp2）已实测：cyclic 400k×8 节点 **357 MB、≈110 B/节点线性泄漏**，acyclic 对照恒定 15.1 MB，checksum 与 JDK 逐字一致——「逃逸到旧对象但成环 → 泄漏」的归因坐实（数字在文档 §五）；`thread_local_lint` 文档头 region_slot 登记已顺手完成。下轮按文档 §六实施序（P0 → 三 → 一 → 二）派实施代理。8. **健全性审查余项（sound-rem，10-11 交付）**：① rawmem 批量快路径重做 fc2e979d（整字段 8 字 ×8 展开、同偏移拷贝 8 字块先读后写按重叠方向排序；Miri 2 种子 ×10 例过[作业 srem-miri1]；kr1 同机三作业对照 srem-bench4/5/6：批量 4–6（中位 5，判据 ≤5 ✓，57564806 为 7）；CAS 417–428 vs 控制组 5328072a（无 sound-rem）446（改善 4–6.5%）；普通 79–80 vs 控制组 74（+8%，布局漂移，#[inline] 复测无改善）；普通/CAS 对 96fc1cf4 基线的回退主体是 batch-1011i/j/k 自身（控制组即 74/446），转主会话）；② `update` 回调泛型内联 5bb96f9d（rava_rawmem→native_memory→unsafe__ext::prim 全链单态化）；③ 堆数组跨元素 RMW ed4244b1 + 8cf51774（同条双锁死锁修复）：经核旧分条按数组基址取条、同数组本就互斥，原「丢普通元素写」修正为元素内 / 跨元素原子性边界（逐元素原子 + 逐位置一致，无整区间一步原子）；字粒度原子单元因**混宽原子访问 UB**（Miri weak_memory 文档明言）不可行——边界与论证写进 array/store.rs 头注；分条改锁区间覆盖全部对齐字、update_bytes 泛型内联；e2e TestHeapArrayAtomicView；④ byteBufferView ISE 根因已查清未修：flavor 类 `VarHandleByteArrayAsInts$ByteBufferHandle` 不含 `"$Array"` 判定子串 → 落 Field 族 → 无 fieldOffset → `var_handle_ext.rs:138` 无消息 ISE 首次访问即抛（堆 / 直接缓冲同症，byteArrayView 不受影响）；修复（新增 $ByteBufferHandle 视图族）留专项。抽查 srem-spot1 8/9 + srem-spot2 1/1（新测例初版同条双锁死锁挂起，8cf51774 修复后过）。见分诊报告五「余项处置」。
9. **生成器审查余项**：vendor 迁移 `cp -a` / `mv` 保留 mtime 复验（sg1 上三次中断，需重发）。见分诊报告「生成器修复」节。
10. **反射强转同类风险**：✅ 10-12 核查收口，**无失配、见证入库**（coerce-check 分支，仅测例 + 文档无代码改动）：`to_object_text` Prim 分支的 `.into()` 原生盒只出现在转译器内部擦除边界（块级合并 / 条件原子 / lambda 体 / sigpoly 实参），不与用户可见 checkcast 相接——源级装箱 javac 已显式发 valueOf，checkcast 翻译（try_cast / box_first / From<Object> 视图）收到的是翻译包装类实例；MH `invoke__site` 返回侧三形态各得其所（基本返回经 `From<Object> for i32` downcast 原生盒、适配装箱经翻译 asType 字节码 valueOf、等型引用直取），与 84d9468f 的「原生盒只作 VM 内部载体」口径自洽。拒收面语义按 JVM 实测校准：`invoke` 引用上下文不可转换（`(Long)` 套 int 句柄）是 asType 适配时 WMTE 非 CCE、`invokeWithArguments` 返回后强转错误包装类是 CCE、`invokeExact` 调用点类型（含返回）必须全等；另见证 sigpoly 返回只在显式强转上下文细化（赋值上下文按 Object 发描述符）。新增 TestCoerceBoxCast（21_casting，checkcast/coerce 装箱族全分支 + 四种错误包装类 CCE）与 TestMhBoxCast（59_method_handles，八种基本类型引用上下文 / 子字实参三入口 / invokeWithArguments / 拒收面），期望 JDK 21 本地生成；抽查 ccx-new2（fb2825d85b5a，kr2/sg2/us1/jp1）15/15 PASS 含 TestReflectBoxCast、wrapper/widen faces、MH 族、autobox 族回归；generator 本机 cargo check（--release --tests，heavy_lock）过，java_runtime 由 15 例端到端构建覆盖。见 [`2026-10-12-coerce-boxcast-audit.md`](reports/2026-10-12-coerce-boxcast-audit.md)。
11. **GSSNameSpi 大闭包复查**：E0599 已修（fix-iface-obj-overload），闭包已回落到 3001 类不再触发；下次闭包纳入 jgss 时复验。
12. **main 普通段（putInt/getInt）+25% 回退（10-12 立条，用户裁决下轮二分）**：kr1 direct_mem_bench 普通段 64→74 ms（96fc1cf4 → batch-1011k，控制组复现，回退主体在 1011i/j/k 区间，嫌疑 nogc-escape 5df7e9ba 或闭包变化；sound-rem 其上另 +8% 为 java_runtime 布局/LTO 漂移）。下轮派代理在 1011h↔1011k 区间二分定位。见分诊报告五（sound-rem 带入）。

- **spring-core-slice（矩阵 #11，10-10）**：spring-core 6.2.19 切片 sc1–sc4 全部 GOLDEN OK（60e68cc7，dev）。通用修复五项（lib crate 依赖序、类路径外类折叠、继承协变桥、代理接口方法元数据、开放枚举入 L3 分派），无 Spring 特判、runtime 无第三方内容。闭包 sc1 2333 / sc2 2165 / sc3 2479 / sc4 2166 个 JDK 类。回归 TestDynamicProxy / TestAnnoReflect / TestSerialProxyForm 3/3 PASS；单测仅 thread_local_lint 失败（main 既有）。详见矩阵 #11 行。
- **c4-url（TestUrlParsingFaces，10-10）**：根因是 `URL$DefaultFactory.createURLStreamHandler@162` 按 `"sun.net.www.protocol." + protocol + ".Handler"` 按名取类，protocol 推不出，旧口径含任意串的候选只匹配已在闭包中的类，https 处理器不入闭包。修法：首段为字面量的任意串候选按类路径匹配（开放世界，`class_lookup.rs::anchored`），不用补种、无类名特判。类数：HelloWorld 580 不变、DeepCopy 3028 → 3033（+5 个处理器类）、TestUrlParsingFaces 3404 → 5123（https.Handler 经 `URL.openConnection` 分派牵出 SSL / JCA / certpath-LDAP / JNDI-RMI 栈，属 https 支持的固有代价）。抽查 8/8 通过（TestUrlParsingFaces、HelloWorld、DeepCopy、TestBuiltinUrlProtocol、TestUrlProtocolOpen、TestEmbeddedUrlRebuild、TestUriRawParts、TestSecurityPermissions）；全量单测仅 thread_local_lint 失败（main 既有，与本改动无关）；known_failures 条目已删。详见 `docs/plans/2026-10-07-c4-run-failures.md` §三。
- **转译耗时回归（perf-regress4，第四轮达标，待合入）**：见 [`docs/plans/2026-10-09-transpile-time-regression.md`](plans/2026-10-09-transpile-time-regression.md)。CHM 表合并的真因是 `SerialCallbackContext.obj` 按类共用：序列化写方的全部对象图成了反序列化 `setObjFieldValues` 的写入目标，引起整堆字段互灌（8ce97959 让 `defaultReadObject` 路径入档案后触发）。c3ec480d 让值持有者（final `Object` 字段由构造器从实参写入）按对象分开。us1 同机对照（p4-t2）：DeepCopy 527 → 306 s / 4.1 GB，JNDI 606 → 348 s，序列化两例约 520 → 约 310 s；类集合对 batch-1010c 无新增（减少 17–40 个）。

- **集成分支与 main**：rust-closure-analyzer = main，已含 batch-1013（669be8c6 起）。batch-1012（fcabee4a，含 1009–1011）与 batch-1013 均已放行。改名 rava 于 10-09 完成（目录、origin `yw/rava.git`、脚本路径）；dev 检出的 origin / 目录待 dev 恢复后改。
- **batch-1012 放行记录**：单测仅已知失败；抽查 53/55（TestUrlParsingFaces 已知，TestJndiNoProvider 转译超时、放宽重跑通过）。java_base OOM 根因与修复：s6（c18e8fc4）后启动映像对象 7702 → 19347 且集中在单个 static 与单个启动函数，charset-ext 将映像分 24 段、启动函数拆 42 个（b3a860ac）、重定位先于回放（160789c7）、`ImageData::writes_statics_of` 统一判定（14917aed），java_base 峰值 11.8 GB → 1.6 GB。
- **batch-1013 放行记录**：fix-1011 后续（3f78902a）+ reflect-marker（d51837e4，DeepCopy 3553 → 3532，HelloWorld / CollectorsDemo 3304 → 3263）+ logger-chain（65ab8a99，含 seed-chain c7fbaf8c，HelloWorld / CollectorsDemo → 3233，DeepCopy 3511，LogManager 0）。单测 A 组仅已知失败、B 组全过（`profile_union_key_and_coverage` 本次通过，第四根因仍在）。抽查 50/55：TestUrlParsingFaces 已知；DeepCopy、TestJndiNoProvider、TestSerialDefaultSuid、TestSerialUserGenericCallbacks 转译超 600 s，放宽超时重跑全部通过。
- **reflect-marker 耗时回归（已消失，10-11 关闭）**：main 30bd666f 在 dev 上复测（closure 模式，作业 rp-cc-30bd666f）：DeepCopy 99 s / 2.9 GB / 3033 类，同机 a88d7075 为 232 s / 6.3 GB / 3553 类；TestSerialDefaultSuid 98 s / 2.9 GB；TestJndiNoProvider 415 s / 7.0 GB / 5116 类（含 JCA 提供者，属已知可达）。消失的原因是 perf-regress4 的 c3ec480d，78b744fe 只是触发者，其口径不变。详见 enum-values-direct §9.7。
- **转译耗时回归**（第四轮达标，perf-regress4 待合入）：转译秒数 DeepCopy 527 → 912、TestSerialDefaultSuid 538 → 862、TestSerialUserGenericCallbacks 514 → 907（batch-1012 → 1013），TestJndiNoProvider 324 → 570 → 1006（batch-1009 → 1012 → 1013）；疑点 78b744fe 与 batch-1012 区间。目标：4 例回到 batch-1012 水平以下（JNDI ≤350 s），DeepCopy 峰值 ≤6 GB，闭包类集不变大。
- **fix-1011 第四根因**：✅ 已随 bootcache 入 batch-1009c 合入（缓存组 = 未回退站点贡献的组，引导映像计划 §5.8.6）。
- **日志链缺口 ③**：✅ d1df66a6 兑现（HelloWorld 3456 → 575 / 1875、CollectorsDemo 614 / 2000，闭包 112 s → 6 s；537 / 583 是 seed-chain 时的旧基线），详见引导映像计划 §5.9.7；`useSurrogateLoggers` 未折叠的终态解法（路径 A：折叠 `isSystem(module)`）记于同节。
- **lc3-fix（4c42ac44，基于 batch-1009d / logchain3）**：修复 ReflectionAPI 运行超时。根因是 `Class$Atomic.<clinit>` 经 static_const 折叠，按名取偏移（`objectFieldOffset`）从未放开 `Class.reflectionData`。Unsafe CAS 只写流图、不进常量格，getfield 被折叠成 null，`newReflectionData` 的 CAS 因此无限重试；收窄后的闭包暴露了这个潜伏缺陷。修复在 `hw_offset.rs::site_offset`：消费符号偏移即 `open_field`，恢复「按偏移访问 ⇒ 字段不折叠」。dev 验证：抽查 6/6（ReflectionAPI 运行 0.02 s），单测 A 11/11、B 全过。
- **注解反射 NPE 回归（TestAnnoReflect / TestAnnoDeepAccess）同由 lc3-fix 修复**：二分定位到 reflect-marker 78b744fe（边界类字段改为「手写层提及才开放」，`Class.annotationType` 只经 Unsafe CAS 写、偏移在构建期初始化的 `Class$Atomic.<clinit>` 按名取得，故被折叠成 null）。与 reflectionData 同一缺陷；lc3-fix 在符号偏移消费点放开字段，dev 抽查 anno-on-7d261f96 两例通过。另一修法 anno-fix（fe1ad3ab：读取映像偏移槽即放开字段）放开范围更宽且暴露 `reflect_new_array_element_precision` 的顺序依赖（须随 order-findops），弃用、分支已删。
- **batch-1009c 已合入（10-09，按用户指示先合入，单测与 57 例抽查转合入后验证）**：含 bootcache（缓存组 = 未回退站点贡献的组，§5.8.6）、c1d-rest（C1d 收窄余项 0 类；c1d 计划 §31 记下一步能力：① 每对象 URI 跟踪约 285 类；② 引导区未知名字调 `Charset.isSupported` 放开全部扩展字符集约 390 类，charset 线续作；③ Formatter 常量格式串构建期求值）、regress2-rest（② ✅，① 转 object-bytecode）、annot-sig（SignatureParser 出闭包，HelloWorld 3456→3407、DeepCopy 3757→3727，`rava closure` 口径；e2e 口径另计）。
- **进行中（10-09 派）**：根类非 native 方法按字节码翻译（object-bytecode）；日志链缺口 ③（logchain3）；转译耗时回归第四轮已达标（perf-regress4，c3ec480d，待合入；根因与实测见 2026-10-09-transpile-time-regression.md「第四轮」）。**待派（按序）**：并发小步 A → B（无 GC 文档 §四，10-09 定）；C1d §31 三项能力。
- **C1d §31 能力 ② 字符集（c1d-charset，10-10，c1d 计划 §34）**：新增动态属性受理性质 `[facts.system_properties.accepted]`。`sun.jnu.encoding` 在 initPhase1 经 `isSupported` 校验，自档位 1 起作标准提供者 `charsetForName` 实参恒非空；受理调用按映像对象接收者的运行期类选目标，映像对象类表随映像追加补齐。jnu 来源已折叠，取值仍按 U14 取运行期宿主值。类数（同基线 44e49b2e）DeepCopy 3038 → 3038（ext 281）、HelloWorld 577 → 577（ext 0），类集合逐类相同。扩展字符集全有或全无，余下来源 2–4 任一在就仍是 281：XML 声明编码 `Parser.enc`（Decompressor 派发不精确，归 c1d-fmt2）、`String` 按名字符集构造器反射面、x509 残差 `DerValue.string2bytes` 常量 `"UTF_32BE"`。三者出闭包后的上界为 DeepCopy 2743（−295），ext 只剩 2 个类型级类。常量名构建期求值需要闭包期具体求值的占位语义（§34.5）。
- **C1d §34.5 来源 4 占位语义（c1d-placeholder，10-11，c1d 计划 §36）**：已落地两项能力。一是闭包期具体求值读不可建模类的 final 引用静态得占位对象，可存放、可传递，身份运算或内容访问即回退，快照按来源静态字段的抽象值代表。二是构造器本地访问器 `newInstance0` → `reflect_new`。另把 `Charset.forName` 列为具体求值入口。类数 HelloWorld 580 / ext 0、DeepCopy 3033 / ext 281，与基线 c02fbf35 逐类相同，种子 0 / 1 / 2 一致。来源 4 仍成立：`"UTF_32BE"` 的求值停在 `Class.reflectionFactory`。余项是 SharedSecrets 一次写入的登记表静态（清单 `registry_statics`）、`reflectionFactory` 缓存登记、访问器选择钉值（§36.5）。
- **C1d §36.5 反射工厂链（c1d-registry，10-11，c1d 计划 §37）**：落地登记表静态（`registry_statics`）、映像值静态（`image_statics`：`ReflectionFactory.config` 钉本地访问器、`VM.javaLangInvokeInited`）、`Class.reflectionFactory` 缓存、`getClassAccessFlags` 操作。`DerValue.string2bytes [Str("UTF_32BE")]` 转为具体求值成功，**来源 4 出闭包**：DeepCopy 切除来源 2、3 后 3033 / 281 → 2738 / 2（逐类只减）。HelloWorld 580 / 0、DeepCopy 3033 / 281 与基线 30bd666f 逐类相同，种子 0 / 1 / 2 一致。下一步是来源 2、3，以及 `ExtendedProviderHolder.<clinit>` 在闭包期抛异常的根因。
- **派发点顺序依赖（order-findops，10-10 完工入 batch-1010b）**：FindOps 根因为 `absint/oracle.rs` `returned_params` 在尚无返回路径（⊥）时答 None（汇合），改答空集（§5.8.6），`closure_independent_of_order` 通过，HelloWorld / DeepCopy 方法各 −1（`ForEachOp$OfRef.get`）。`closure_independent_of_hash_seed` 仍失败（TestSerialDefaultSuid 种子 0 多 `Nodes$CollectionNode.forEach`）：`engine/ctxsel.rs` `selector_ctx` 读尚未定论的常量格选上下文（`ArrayDeque.grow` → `Arrays.copyOf` 常量阶段按调用点克隆，撤不回），终态修法为选择子掩码非空即一律按调用点克隆（续作 6，见 2026-10-08-annotation-signature-closure.md），10-10 派 ctxsel-mono。
- **C1d a5-4 反射构造器按描述符收窄（c1d-a54-reflect，10-11，c1d 计划 §38.4）**：非用户类构造器只暴露被请求的描述符。请求来自三处：查找点由 Class 数组抽象值推出（null / 空数组 / 类字面量定长数组）；record 规范构造器查找按分量类型（`record_constructor_lookups`）；公开枚举在调用者处判定，包装方法与只读元数据的调用者（`computeDefaultSUID` / `superHasAccessibleConstructor`）不请求。DeepCopy 3001 → 2659（−342，`sun/nio/cs` −291），StockTrans 2999 → 2657，TestSerialDefaultSuid 3006 → 2664，HelloWorld 580 不变；三集合都是基线子集，DeepCopy 种子 0 / 1 / 2 一致；抽查 12 / 12，closure lib 244、closure_cli 12 / 12 通过。遗留：按 Constructor 对象来源请求（取代调用者清单）、基本类型类入描述符。
- **c1d-fmt2（10-10 完工待合批，分支 c1d-fmt2）**：C1d 能力③，详见 c1d 计划 §33.7。
- **expected 版本分目录（10-13 完工待合批，分支 expected-dirs，基 batch-1014，用户裁决口径）**：tests/expected 期望按 JDK 版本分目录（jdk21/ 全集 git mv 1144 例；jdk25/ 只放版本差异 4 例，triage §3.7 清单，OpenJDK 25.0.4.1 生成）；run_tests 期望读取/生成路径按语料 JDK 主版本推导（EXPECTED_LAYER，不写死版本值），--update-expected 写入 jdk<N>/，实验 JDK 运行遇缺失按 golden JVM 补生成入本版本目录后照常 diff（参考构建运行维持缺省 SKIP）；运行期 fixture 数据文件（sqrt.dat 等 Java 源字面引用）保持平铺；适配 opt_profile_bench / vt_repeat / direct_mem_bench×3 / refjdk.toml / README。验收：单测全绿（test_known_failures 一败为基线既有）；exd-gen/exd-regen 作业 rc=0（jdk25 4 例差异与报告逐一对应；两版本删件重生成字节一致）；spot exd-a-j21 JDK21 5 例混合全 PASS、exd-b-j25 JDK25 7 例全 PASS（4 差异读 jdk25/，3 常规经缺失补生成路径）。清单与语义：`docs/plans/2026-10-12-expected-jdk-version-dirs.md`
- **s7k6 回收（10-13 完工待合批，分支 s7k6-recover，基 31e4762d，与 batch-1014 无文件重叠）**：s7k6-uta 本地重跑收口（13/0/1 rc=0）+ s7k6_peak.sh sccache 防护（强制 `RUSTC_WRAPPER=""`）与 `PEAK_OUT` 覆盖 + marker_crate_probe 用法防坑提示 + §9.9/tasks.md 回填；纯文档与诊断脚本，无生成器/运行时行为变更，合批免抽查。
- **libtests-j25prof（10-13 完工待合批，分支 libtests-j25prof，基 batch-1014 @22409c66，docs-only）**：JDK25 junit 大槽 8 例剖面收口——TestJunitFailureMessages 65G 极限剖面（499e2bf8）+ 余 7 例 32G 护栏串行严重度补扫（报告 §七：全部不收敛，22G 越线 457–544s、32G 越线 750–903s，与 FailureMessages 外推模型同带吻合）；见 2026-10-13-jdk25-junit-oom-profile.md；纯文档无行为变更，作 G1 验收回归集输入，合批免抽查。
  - 机制 B（`[concrete] object_results` = `Formatter.parse`）已恢复。内存受控靠结果对象按内容合并，用户 printf 不再污染 Formatter.format 靠枢纽形参槽污染。
  - 修复具体上下文的映像静态活性缺口（LahNumbers NPE，failure_patterns `concrete-ctx-image-static-dead`）。  - 对 main b1dae15c 实测：

    | 用例 | 基线（类 / 方法，峰值） | 本分支 b04de767（类 / 方法，峰值） |
    | --- | --- | --- |
    | DeepCopy | 3038 / 16715，3.14 GiB | 3028 / 16587，2.80 GiB |
    | LahNumbers | 2158 / 11077 | 675 / 2431 |
    | HelloWorld | 577 / 1896 | 580 / 1896 |

    HelloWorld 的 +3 类是健全性代价（`ClassRepository.NONE`）。
  - 单测 A / B 0 失败，hash_seed 通过，抽查 30/30。
  - **main b1dae15c 的 `closure_independent_of_order` 失败 → 已修（fix-order-live，aad074f8）**：DeepCopy batch 1 / seed 1，只有 `boot_image_data.live` 不同（各类 `serialPersistentFields` 静态值）。二分到 batch-1010g（c3ec480d 使缺陷显现）。根因是 2c503775 起的 `edge_more` / 父枢纽重放：句柄存取入口只对先到的接收者按来源标记建模，后到标记所指字段节点不登记。修法是句柄存取入口按接收者逐个完整接边（`engine/invoke.rs` `edge_more`、`engine/hub.rs` `link_hub`）。DeepCopy 类 / 方法 3075 / 16838 不变，活对象 20229 → 20315，耗时持平；详见 2026-10-09-transpile-time-regression.md「顺序无关回归」节，失败模式 image-live-handle-order。`closure_independent_of_hash_seed`：1010k 932a4ba4 曾失败、l 751e0d6f 通过。
  - 余项：DeepCopy 的 Calendar 仍经日志链（`ObjectInputFilter$Config.<clinit>` → System.Logger → MessageFormat）与真正未知格式串（`SimpleConsoleLogger.format` 属性、`toGMTFormat` 资源束串）可达；TestStringFormat 有机制 C 的 owild 污染；「仅身份」映像活性层未做。
- **sentinel16g（10-14 完工待合批，分支 libtests-sentinel16g，基 origin/main @bdb9bad4，纯工具+基线存档，batch-1016 候选免抽查）**：T11 落地 T6 §八建议的 16G 哨兵回归集常设机制——`scripts/diag/sentinel16g.sh`（sentinel/pathology/all/--full/promote）+ `scripts/diag/sentinel_sampler.py`（/proc 树 RSS 采样 + 越限杀 tripwire，机制承 scan32g watchdog）+ `docs/sentinel-baselines/`（首轮基线：哨兵 10 例峰值 / 病理 12 例速率 + 22 条 RSS 曲线 + baseline.md 口径）。首轮采集全绿：**SENTINEL ALL-GREEN 10/10**（峰值 5.28–8.05G，16G tripwire 零触发，与 T6 双基线参考 ±3% 内）；**PATHOLOGY BASELINE-SET 12/12** 全 window-kill（240s 窗口不收敛逐一在档，速率 6.3–23.5 MiB/s）。TSV 逐例 append 断点可续跑；绿转红 = rc≠0 / 越线 / 峰值超基线 +25%，病理漂移 = 速率差 >±20%，--full 终局 32G 认证（G1 修复后用）。无生成器/运行时行为变更。
- **引导映像零拷贝（boot-zerocopy，10-10 完工入 batch-1010b）**：映像表改链接期符号、驻留查找回填运行期表；整进程墙钟 HelloWorld 195 → 178 ms、DeepCopy 314 → 292 ms，二进制 −0.9%。未达标：`__boot_image_start` 156 / 176 ms（目标 ≤1 ms，热点采样作业 zc-prof-669365cf）、DeepCopy 门面峰值 1779 MB（目标约 1.6 GB，需映像静态按块分 crate）；release 档待大内存机器；D5 残差区、S6 标准流未做。续作入口：引导映像计划 §5.10.5（属纯优化，按 10-06 分级暂缓）。
- **fix-e0283-nd（10-10，C4 全量失败 NormalDistribution）**：根因——null 存入 / 汇合 / 实参等路径先把 null 落成无类型 `Default::default()`，随后再经 Object 边界或 `From` / `Into` / checkcast 转换（`<T as From<Object>>::from(Object::from(Default::default()))`），源类型不可推断（E0283，`AbstractPipeline` 局部 `p` 的跨实例化重建）。修法：在转换构造的公共入口统一处理——`sim::exprs` 的 `object_from` 对无类型缺省值取 Object 的 null，`qualified_from` / `into_call` / `instr::coerce::cast_node` 取目标类型的 `<T as Default>::default()`，`from_call` 原样返回；文本层 `to_object_text` 与 `unify` 的 `from_object` / `from_common` 同口径；存储重建 `rebuild_via_object` 对 null 直接取声明类型缺省值。验证：抽查 fixnd2（jp2）NormalDistribution 通过；单测作业 fixnd-ut2（dev，generator 除 driver）全过。
- **fix-clone（10-10，C4 全量失败 TestJucSync / TestLocaleCurrency / TestLocaleDateCjk / TestParallelCapable / TestRandomFactoryAll / TestProcessBuilder）**：三个根因——① `offset-field-folded`（JucSync / RandomFactoryAll / ProcessBuilder）：只经 Unsafe 按偏移写的字段在构建期初始化类 `<clinit>` 按名取偏移、未放开而被折叠，5177e182 在符号偏移产生处（`field_offset`）即 `open_field` 收口；② 类镜像身份哈希构建期与运行期不一致（ParallelCapable）：映像内以 Class 为键的 WeakHashMap / HashMap（`ParallelLoaders.loaderTypes`、`Reflection.fieldFilterMap` 等）在运行期按地址哈希查不中，96143cf3 令镜像身份哈希按所指类型名确定（求值器 `fnv32("m:"+名)`，运行期 `Class::__pin_mirror_hash` 同值）；③ 地区补种漏别名常量（LocaleCurrency / LocaleDateCjk）：`Locale.CHINA / PRC / TAIWAN` 在 `<clinit>` 以另一常量赋值，溯源器只认字面量 invokestatic，zh 资源束类不进闭包而回落 root，222fd151 补别名溯源与中文候选链补文字 / 地区。验证（dev，222fd151）：抽查 9/9（6 例 + HelloWorld / DeepCopy / CollectorsDemo）；单测 fixclone-ut222c（closure_cli 11/11）/ fixclone-ut222r（其余全过，0 failed）。known_failures 删 TestLocaleCurrency（原误记为 expected 基线）/ TestLocaleDateCjk。
- **已知单测失败**：`param_string_constants_fold_switch`。在缺少相应修复的分支上还会出现：`container_elements_per_object` / `known_gate_ranks_first`（缺 fix-1010）、`profile_union_key_and_coverage`（第四根因修复前）。
- **known_failures**：batch-1008 新增 TestBootLayer、删除 TestXmlSaxEvents；抽查已知失败 TestUrlParsingFaces。
- **框架推进顺序（10-10 用户定）**：① 反射组修复（c4-reflect：反射调子类覆盖方法命中存根、反射帧可见；JUnit 与 Spring 共同前提）→ ② JUnit：J3 形态接线 + J4 注解驱动反射入口通用建模（`2026-10-05-junit-e2e-deps-task.md`），让 63_junit 的 10 例 e2e 跑通（**10-10 junit-j3j4 交付**：J3 形态接线完成；J4 无补种 10/10 逐字通过（抽查 junit-5a0993c1，dev），通用注解反射建模由既有机制覆盖、未改分析器；AssertFamily / FailureMessages 期望含 JVM 身份哈希（违反方法论 §五），经用户批准改为定值 toString 对象并重生成期望；新增 TestAnnotationDrivenRunner 通过；见任务书 §七 第 14–16 条。**遗留（不在 J4 范围）**：TestJunitFailureMessages 动态对照漏覆盖 5（StreamOpFlag.<clinit> → StreamOpFlag$Type / $MaskBuilder / EnumMap / EnumMap$1，VarHandle.<clinit> → VarHandleGuards），属构建期初始化类 `<clinit>` 运行期引用的通用口径问题，与 JUnit 无关；**10-10 dyncmp-bootinit 已排**：dev 实测两类均在 `boot_image_data.build_time`、生成类头带 `#[boot_initialized = true]`，5 个被加载类只在 JVM 执行这两个 `<clinit>` 时加载、运行期不需要（亦在 build_time 而不在闭包，映像活对象不含其实例），非闭包缺口；dyn_compare 新增 `build-time-init` 归因（栈上遇构建期初始化类的 `<clinit>` 即整段归之，数据取 closure.json，无类名特判），漏覆盖 5 → 0；dev 验收：抽查 18 例（63_junit 10 + HelloWorld + DeepCopy + 6 例 stream / VarHandle）全过，同 18 例动态对照漏覆盖 0、未归因 0（build-time-init 78 处），见 failure_patterns `dyncmp-build-time-clinit`；m5 `@Before` 字段写回 W0-4 已于 10-05 J2A 5/5 GOLDEN 收口）→ ③ spring-core / beans 切片（矩阵 #11，可与 ② 并行）→ ④ S0 最小 Boot 应用。S0 API 面计算（api-surface-s0，只分析不改生成器）可与 JUnit 并行。**10-10 S0 API 面续作（api-surface-s0b）**：dev 复算闭包仍未产出，④ 的前置闸门 G1 未解除。A 在 58451 MiB / 2251 s、B 在 31822 MiB / 1831 s 被看门狗终止，均为单线程线性增长，栈落在 `link_hub` 重放 lambda × `lambda_dispatch` 方法引用接收者展开（hub.rs:170 / lambda.rs:157）。解除需要分析器任务：加诊断，再把重放改为按枢纽 × lambda 只连接一次、接收者展开增量传播，目标峰值 ≤ 16 GiB、耗时 ≤ 30 min。分层规则 3 改为面外占比阈值，阈值由标定集定为 0.4476（14 例零分错）。重出 tiers_s0：主力 989、暂缓 121、补测 1370。known_failures 仅补 TestVirtualThreadScale 的依据。见 `docs/reports/api-surface-s0.md` §5.1 / §5.5 / §六 / §7.0。依据：JUnit 的注解扫描 / 反射实例化 / Method.invoke 是 Spring IoC 核心路径的子集。②③ 已交付（junit-j3j4 batch-1010p；spring-core-slice sc1–sc4 GOLDEN，batch-1010t）。**10-11 G1 分析器任务（`s0-g1`，未合入）**：已加 `--lambda-prof` 诊断。改造后枢纽 lambda 按「枢纽 × lambda」只建一个读者，方法引用接收者经增长枢纽增量并入，读者从 291 万降到 6.9 万，种子 0/1/2 下 HelloWorld / DeepCopy / TestSerialLookupPairing 的集合与 main 一致。但 S0 仍不收敛：A 在 615 s 时 26.7 GB、61 万方法上下文，仍在增长。新主项是方法上下文克隆（`@level` 档位与 `java.lang.invoke` 分配点上下文、`Object.<init>` 一类叶方法上万份），G1 未解除。**10-11 G1 续作（`s0-g1-hub`，未合入，性能计划 §4.11）**：完成两项改造。一是枢纽按接收者来源建立，删去精确集合父链，lambda 读者形参改取 `hub.vals`。二是移植上下文无关方法不克隆（ctx_free）。四例在种子 0/1/2 下与 main 集合一致；TSLP 的 `MemorySessionImpl` 档位落在 main 自身两种种子状态之一，属基线既有。S0 A 同一时刻 RSS 约降一半（250 s：11.8 → 5.5 GB），但仍线性增长：5ae32b76 在 1020 s 达 25.4 GB，被看门狗终止。按分配类归并接收者上下文的试验使四例方法 +1 到 +5，未提交；该试验 1846 s 时 23.2 GB，增长项转为增长枢纽的接收者（2.8 亿）。新主项是 `Object.equals` / `hashCode` / `getClass` / `toString` 的宽接收者增长枢纽：每成员约 7 千个调用点克隆 × 1.4 万个接收者。**10-11 G1 共享（`s0-g1-share`，未合入，性能计划 §4.12）**：已实现两项。一是宽接收者枢纽跨克隆共享：增长枢纽的接收者超过阈值后永久升级为共享枢纽。二是按调用点目标按效果键去重。语料关闭共享实测，增长枢纽最大接收者数 16391（TestSystemProxySelector），阈值据此取 32768。四例在种子 0/1/2 下与 main 集合一致（T=4096 与 32768 两档），确定性两项通过；去重使方法上下文降 4–7%。S0 A 以 T=4096 强制共享：接收者 9000 万 → 5000 万，但仍不收敛，2971 s 达 34.9 GB 被杀，方法上下文 60.2 万且持续增长。**主项是方法上下文**（约 35 KB / 个），不是接收者；另有每约 240 s 一次 +6 GB 的瞬时峰。sb3（515b656e）在 1270 s 前后达 30 GiB 被杀，方法上下文 45.1 万，不收敛。**未完成**：阈值扫描缺 28 例（11 例 rc=1 为作业类路径问题；3 例 OOM rc=137 加 14 例批次丢失，需单例补测）。**待派**：方法上下文选择性（Zipper 类预分析，宽对象不按对象克隆，目标 S0 上下文 ≤20 万、峰值 ≤16 GiB）；定位瞬时峰；完成后复测 S0，再续 ④。
- **C4 全量第一轮（10-10，dev）**：1109 例，复跑后通过 1090、失败 19。已登记 13（JUnit 10、VirtualThreadScale、HttpLoopback 2、XmlTransform）；新登记 known_failures 5 例（f680bab4）并派修：TestThreadNatives → conc-step-b；TestReflectFieldMethod / TestReflectInvokePerReceiver / TestSecurityManagerContext → c4-reflect；TestUrlParsingFaces → c4-url。TestHttpLoopbackAsync 本轮编译 OOM（14 GB），资源类。
- **审查生成器修复（review-generator，10-11）**：待办「生成器 / 宏」七项已落地（4f2fb0a5…e11a3191，见 `docs/reports/2026-10-11-code-review-triage.md`「生成器修复」节）；单测 rc=0、27 例生成 `.rs` 零差异、seed_check 4/4、DeepCopy emit 无回归；抽查 rg-spot-e11a3191 15/15 全过，vendor `cp -a`/`mv` 复核待重发；未合入 main。
- **10-11 用户裁决与派发**：S7-4 类型标记 crate 选路线 A（每类方法 trait 属生成层内部细节，同 `ObjectVTable`；接受常数级分派开销；标记 crate 按档案生成；语料档案目标 ⑤），实施由 s7-marker 推进（S7 计划 §9.8.3 / §9.9）；reflect-marker 耗时回归改名后派出（reflect-perf，先复测）；G1 续作（枢纽建立口径，性能计划 §4.10）待 batch-1011b 放行后派。巡检 cron 保留；迁往 dev 的切换窗口定在「在跑代理全部交回、无在测批次」的第一个空档（交接文档 /data/workspace/handoff）。
- **S7-4 K3 方法 trait（s7-k3，10-11 交回；同晚收尾 15276d82，未合入）**：wrapper 可读层成员移入 `X__Methods`，生成器按 owner 精确导入，runtime 只加 `use` 行。收尾已把 +24–31% 底段峰值分解归因（匿名导入全部落声明层→D8 边级联底段吸收上段类 342→476 / 1895→2458，+trait 成员签名双份全管线抬升）并修复（15276d82：声明层按词剪枝匿名导入、留体 fn 按发射差分账本精确保留、访问器/`__init_on`/`_init_not_null` 留固有、模拟器复现一致）：切段与基线逐类一致，本机底段 +7.8% / 上段 +8.0%（body ±5% 内）；剩余为 java 方法外壳 trait 展开的下限，压到 ±5% 需选择性 trait 或放宽阈值，待用户裁决。服务器作业 s7k3-fix1-*（复测/单测 A/B/DeepCopy release 二进制）跑批中；待验证清单见计划 §9.9「K3 收尾」节。
- **S7-4 K6 前置项（s7-k3 8efa6444+5c3c8bdc，10-11 完成，未合入）**：与 std prelude trait 同名的方法（clone/eq/from 等 11 名单，实测命中 clone/eq/from 三名）改 `__` 前缀内部名后移入方法 trait（定义/调用侧单一规则 `ty::ident::escape_std_trait_method`）；接口载体成员（static 方法、static 访问器、`__class_init`、实例分派与继承转发）移入 `I__Methods`，`is_jvm_null`/`__default_*`/const 成员留固有；登记面补齐（record_member 接口、phase2 转发体、init_interfaces、getstatic 声明接口、boot_image、main 钩子）；D8 权重不计接口方法 trait 匿名导入行（否则 DeepCopy 一类跨段）。本机：单测全绿（emit 95 等）、三例切段与 70dd51f8 逐类一致、四例运行输出与 JVM 逐字一致（DeepCopy 80 处改名调用点）；服务器：宏单测 ✅（23/28）、四例种子 0/1/2 闭包集合对基线全同 ✅（s7k6-seed rc=0）；**两在跑作业 10-13 测试机回收**：s7k6-uta（closure_cli 全量）服务器死档后本地重跑 **13 passed / 0 failed / 1 ignored / rc=0**（5504.84s）✅；s7k6-peak4 服务器死档、本地首跑被本机 sccache 污染（touch 重编命中内容缓存不跑 rustc、真编译在守护进程内 `/usr/bin/time` 只见客户端——假峰值），禁用 `RUSTC_WRAPPER` 后 HelloWorld decl 新侧 1 137 484 KB 对 peak3 服务器 +1.2% 口径一致，随后分支合入（f7b69314）清理 worktree 中断，全表不补（±10% 判定已有 s7k3-fix2 服务器 +7.1–9.5% 支撑）；`s7k6_peak.sh` 已加 sccache 防护与 `PEAK_OUT` 覆盖。回收详情见计划 §9.9「K6 前置项完成」节。
- **派发规则**：子代理上限 5，不得再派代理。协调巡检自动攒批、空闲即测、放行合入与清理。
- **暂缓**：
  - 纯优化线（10-06 分级）；U1 重议；
  - 引用类语义（无 GC，C4 之后）；声明层底段收窄（C4 之后）；
  - 等 dev 恢复：确定性单测（`closure_independent_of_*`）、重例。S0 Spring Boot 闭包已于 10-10 在 dev 复算，仍未产出，转为闸门 G1（分析器任务，见上方框架推进顺序）。
- **fix-domstack（b4da5107，2026-10-10）**：C4 全量新失败 TestDomResultNode 与 main 上同样失败的 TestResourceBundleFaces 同源（转译期主线程栈溢出）。根因：常量实参求值（`consteval.rs`）穿过静态分派转发方法不计深度（ef6a1249 引入），`ResourceBundle.findBundle` 自递归且整数实参逐层变化，每层求值键不同、`memo_enter` 同键截断失效，`const_eval → aux_analyze → invoke_result` 嵌套约 1200 层。修法：嵌套位置改为 `EvalDepth { nest, pass }`，穿过次数上限 `MAX_PASS = 8`，超出按普通调用计深度，两者都进记忆键（记忆仍与处理次序无关）。验证：dev 抽查 TestDomResultNode、TestResourceBundleFaces / GetKeys、71_xml 其余 9 例、HelloWorld、DeepCopy 共 14 例通过；TestXmlTransform 在 main 上转译 30 min 超时，本修复后转译 2m49s、编译通过，运行期止于 XSLTC translet 实例化（已改 known_failures 签名、登记 failure_patterns `xsltc-translet-create`）；单测 A（closure_cli 11/11）、B 全过（`param_string_constants_fold_switch` 本次通过）。诊断法：栈溢出无 RUST_BACKTRACE 输出，dev 上用 `gdb -batch -ex run -ex 'bt 80' -ex 'bt -60'` 取栈。
- **c4-reflect（07e849f6 + 677c07d4，2026-10-10）**：C4 复跑稳定失败的 3 例反射用例（TestReflectFieldMethod / TestReflectInvokePerReceiver 运行期命中子类覆盖方法存根，TestSecurityManagerContext 多出 `Method$Direct` 帧）同源于直连反射调用改写（29f54f85 / 75bead2f）。根因 ①：`engine/reflect_direct.rs` `direct_send` 逐接收者连边，所选实现进了 methods，却未计入 dispatched，所以生成器对 vtable 槽条目发存根，而帮助方法的 native `invoke0` 正是经槽虚调用。修法：非 private 实现记入 `vm_targets`（同 VM 反射虚调用）。根因 ②：帮助方法帧不被栈遍历识别为反射帧。修法：清单 `direct_invokers` 的 helper → invoker 经 input `frame_aliases` 进发射层行表（`apply_aliases`），帧呈现为 `Method.invoke`，getClassContext / getCallerClass / StackWalker 统一生效。验证：抽查 c4rf-677c07d4（sg1 / sg2）5/5（3 例 + HelloWorld / DeepCopy）。known_failures 删这 3 条，failure_patterns 登记 `direct-invoke-override-not-dispatched` / `reflect-helper-frame-visible`。
- **fix-xsltc（2026-10-10，fix-xsltc2 已合入 batch-1010r）**：TestXmlTransform 运行期 `Translet class loaded, but unable to create translet instance.` 的根因是运行期定义类没有终态承载（`ClassLoader.defineClass1` 手写对合法类文件一律抛 `LinkageError`）。按用户决定（X1 = S2 训练运行，X2 = 语料 `rava build` 按需自动训练 / 生产显式 `rava trace`，X3 = 同字节多次定义共用一个类）实施按内容寻址的预定义类：JVMTI 记录代理 → 预定义类目录输入（`Origin::Predefined`）→ 闭包把 `defineClass0/1/2`（`vm_intrinsics.toml predefined_definers`）返回值建模为预定义类镜像之并 → user meta `PREDEFINED_CLASSES` → 运行期按 SHA-256 查表（未命中报期望 / 实际哈希）；同时补 `defineClass0`，未捕获异常报告改为虚调 `getCause()`。X3 语义差异登记于 java-rust-translation-reference.md §14.3。闭包单测 `reflect_new_array_element_precision` 曾稳定失败（TestModuleLayerDefine 种子 0 / 1 闭包差 10 类）。根因是引擎的调用点答复直接取不动点途中的返回常量格（`booleanValue` 先见 FALSE 接收者，`isSystem` 暂得 0），已改为先按本调用点常量实参求值、格只作退路（7646c9ba，`facts/oracle.rs` / `deval.rs`，xsltc 计划 §8.4、c1d 计划 §35），阻塞已解除。合入 main a8fbebb6 后的 ef8bc9da 已验证：`closure_cli` 11 例全过；抽查 xs3-m-spot3（dev）4/4 通过（TestXmlTransform / TestModuleLayerDefine / HelloWorld / DeepCopy）；四例种子 0 / 1 / 2 闭包一致，与 main 同规模。顺序单测与组 B（xs3-m-ut2 第 3 / 4 项）在 jp1 上被中途停止、未出结果，合入批次时随批在 dev 重跑 `/tmp/ut-seq.cmd`、`/tmp/ut-rest.cmd`（修法提交 9e7fa2b0 上 hash_seed 通过，只有 main 既有的 order / carrier-slot 两例失败）。遗留（同名多内容改名、档案模式种子、隐藏类记录、classData 等）见 [`docs/plans/2026-10-10-xsltc-translet.md`](plans/2026-10-10-xsltc-translet.md) §8.3。
- **C4 全量**：10-09 在 dev（128G / 16 槽，Memtest86+ 四轮 0 错误、BIOS 风扇曲线已调）上 `--reset` 开跑，基于 main c249cdec，放宽超时（转译 1800 / 运行 900 / 构建 ×2，单例 9000 s）。转译耗时回归未修完即开跑，修复合入后续用例自动受益；全量期间 main 冻结语义改动，只合修复全量失败的提交。
- **C4 运行超时 4 例分诊（c4-runtimeout，10-10）**：AmicablePairs、RamanujanPrimes、SelfReferentialSequence、TestParallelArrayCas（另 TestCommonPool 同症）在 c249cdec 上运行超 900 s，均为语义缺陷（并行流 / ForkJoinPool 卡死），不是 debug 档性能：10-06 的 3f59a8e9 上运行 13.3 / 1.3 / 42 / 0.71 s（CommonPool 0.1 s）。根因属 `offset-field-folded`（只经 Unsafe 按偏移写的字段在构建期初始化类 `<clinit>` 按名取偏移、未放开而被折叠）；c249cdec 不含 lc3-fix 4c42ac44，集成分支头 589052eb 抽查 c4rt-a 5/5 通过（运行 2.53 / 0.84 / 6.97 / 1.13 s，CommonPool 0.17 s）。无需新修复；fix-clone 5177e182（符号偏移产生处即放开）是同类终态收口。分诊中重复实现的 dead3574 已撤回（5743e146）。已登记 `docs/failure_patterns.toml`。
- **测试资源**：dev 已恢复（10-09），测试优先放 dev（缺省 24 槽），云服务器 jp1、jp2、kr1、kr2、sg1、sg2、us1 只作溢出；本机只跑 cargo check。合批全量单测拆 A（`-p driver --test closure_cli`）和 B（其余）两组并行，各约 1 小时。工作流见 `docs/reference/cluster-testing.md` 十二。

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
│   ├─ ✅ S2 `Class.genericInfo` 入映像（U13，u13-generic，见引导映像计划 §5.6.9；闭包规模不变，收益限于运行期）
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
│   ├─ ⏳ JUnit 依赖包测试 J3 形态接线 → J4 10 例跑通（2026-10-05-junit-e2e-deps-task.md；10-10 junit-j3j4 待合批）
│   ├─ ⏳ 框架驱动 API 覆盖（2026-10-07-framework-driven-api-coverage.md）：S0 闭包面复算 ◀── dev 恢复；再扩其他流行库
│   ├─ ⏳ jmod 覆盖第 1–4 步（2026-10-03-jmod-coverage.md）
│   ├─ ⏳ 引用类语义：无 GC 模型（编译期逃逸分析整组释放 + 所有权推断弱引用，2026-10-07-no-gc-memory-model.md）◀── C4
│   └─ ⏳ JDK 25 适配轮（分支 fix-jdk25，交接 docs/reports/2026-10-11-fix-jdk25-handoff.md）：java_base_decl 三组编译问题已修；JDK 21 两处回归已修（C 闭包翻倍 d279b821 扫描器求值 cfg，DeepCopy 闭包 = 集成头 3001 类；B StackWalkerFrames 725249ef 上调留 native 体内）；待合批。
│       下一步：① 问题 A，JDK 25 StackFrameInfo 反射元素流（两个 StackWalker 测试存根失败，归 G1 / 反射建模）；② ✅ 扫描器 `&self` 辅助内 `self.X()` 上调丢 on_self（10-12 修，df026eed，规避已解除——见「剩余工作」第 6 条）；③ `GSSNameSpi.equals_obj` 潜伏命名缺陷；④ JDK 25 全量剩余分类（版本输出差异 3、fold / dead_pcs 2、condy switch 标签 1、超时 1、DeepCopy / DES OOM）
│
├─ 【暂缓】
│   ├─ ✅ build-memsafe 内存友好缺省构建档（10-11 交付，待合入；2026-10-11-build-memsafe.md）
│   ├─ ⏸ 纯优化线（10-06 分级）：二进制 ≤3 MB、S7-4 / S7-5、D2 / D3 引擎结构改造、IR 收敛 / TypeIR G4
│   ├─ ◐ 声明层底段收窄：模拟分析完成（S7 计划 §9.8）；10-11 用户裁决类型标记 crate 路线 A（每类方法 trait，属内部细节，同 `ObjectVTable`；常数级分派开销；标记 crate 按档案生成；语料档案目标 ⑤），实施分解 K1–K8 见 §9.9（分支 s7-marker；K1 wrapper 只持句柄 10-11 完成，单测与抽查 12/12 过；K2 接口载体只持 Object 10-11 完成（s7-k2 847493cb，dev 抽查 5/5，§4.3 计时运行无变化、二进制 −0.83%，待合批单测）；下一步 K4，K3 / K5 可并行）；D8 上段已改按上游感知峰值预算切分（d8-mb，§9.8.4，待攒批合入）；目标所有声明 crate ≤1.3 GB
│   ├─ ⏸ 等 dev 恢复（换内存条，数天）：S0 Spring Boot 闭包、确定性单测（`closure_independent_of_*`）、重例
│   ├─ ⏸ 不实施 / 挂起（10-06 用户定）：方法句柄对象化、T2 余 4b、b1 序列化收窄、a5 关系型边界推理
│   └─ ⏸ 缓：虚拟线程余项（T6 规模；pinned 已消解 0/260，vt-rest）、T1-M3、第三方库通用机制（JNI 层 / 构建期捕获运行期生成类）
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
| B 架构终态 | B3 虚拟线程余项（T6 规模 / T1b 审计；pinned 偶发 10-11 已消解：rava 0/260、PinnedRace 与 JDK 同量级，见 c1d §21.8.5）、B4 T1-M3 按模块登记 | 缓 |
| C 闭包精度与规模 | C1 c1d-elem、C2 clsfact、C3 a5-4 收窄（DeepCopy ≤2803，§29 三项能力优先）、C5 C1d-b（T2 余 / b1 / b2 / b3 余） | 继续 |
| C 闭包精度与规模 | C4 a5 关系型边界推理（HelloWorld ≤371） | 缓 |
| C 闭包精度与规模 | C6 jar 签名收窄 / jar·URL class-path（并入 §29 能力①）、§29 能力② JCA 提供者序求值 | 继续，优先（10-06 用户定） |
| C 闭包精度与规模 | 方法句柄对象化（C1d-b-mhobj）、T2 余 4b、b1 | 不实施 / 挂起（10-06 用户定，上界 −5~−6 类） |
| C 闭包精度与规模 | 共享汇点（c1d-sink） | 已收口（§28.10），≤569 由引导映像达成 |
| D 分析性能 | D1 处理顺序无关（分支 closure-order-free，定性为正确性） | 继续 |
| D 分析性能 | D2 枢纽翻新 / 延迟站点重跑等结构改造、D3 在线节点合并 | 暂停（V12 后提速线暂停） |
| E 编译资源 | E1 B4 内存友好缺省构建档（分支 build-memsafe，16 GB 机器全部可构建为硬约束） | 10-11 交付：15 GB 云服务器缺省档四例无 OOM（`2026-10-11-build-memsafe.md` §四） |
| E 编译资源 | E2 D8 声明层分段 | 机制已合入（b51f9531 / b093069f）。10-11 段上限由 650 类改为按源码体量（分支 d8-mb，a1771258，待攒批合入）：不分段判定 5.5 MB；上段按上游感知峰值预算（−9 + 240×本段 MB + 13×上游 MB ≤ 1236 MB）逐段装满。HelloWorld / CollectorsDemo / DeepCopy / FWord 全部上段实测 ≤ 1221 MB；探针压力（DeepCopy 5.52 MB 上段原 1597 MB）切为 1221 + 725 MB。抽查 5/5、emit 单测 93 通过。遗留：上游系数在档案量级（40–50 MB）待复测（S7 计划 §9.8.4）。底段收窄随 S7-4 / S7-5（见活跃任务「声明层底段收窄」） |
| B 架构终态 | B5 第三方库通用机制：JNI ABI 层（库自带 native 原样调用）、构建期捕获运行期生成类（三方依赖分层 §3.6；rava 仓库不放任何第三方库专属内容，库配置归用户项目） | 缓（10-06 用户定） |
| F 纯优化 | 二进制 ≤3 MB、S7-5（S7-3 已合入，S7-4 10-10 提前开工）、VT `instanceof` / `checkcast` 走 `__ClassDesc`、IR 结构化收敛 / TypeIR G4、PGO（前置：perf 剖析拆分开销，含 volatile 读 SeqCst 排队） | 暂停 |

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途 / 待验证 / 待启项（2026-10-09）；已完成与过时行已删，见历史 §J 与各计划文档。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| 缓存组回退撤回 | ✅ bootcache 9a029858 → batch-1009（待测） | 具体求值站点回退普通分析时，撤回只由该站点贡献的映像缓存组；修复后 `profile_union_key_and_coverage` 应通过。引导映像计划 §5.8.6 |
| reflect-marker 耗时回归 | ✅ 已消失（reflect-perf 7caab3ef） | dev 复测 DeepCopy 分析 99 s / 2.9 GB，无需代码改动；结论见 enum-values-direct §9.6 |
| 日志链缺口 ③ | ✅ d1df66a6 | HelloWorld 575 / 1875（切除上界兑现）；余项路径 A 见 §5.9.7 |
| 转译耗时回归 | ✅ 第四轮达标（perf-regress4，c3ec480d，待合入） | 四例比 batch-1012 快约 40%（DeepCopy 306 s / 4.1 GB、JNDI 348 s、序列化两例约 310 s），类集合对 batch-1010c 无新增 |
| 引导映像零拷贝 | 🔄 boot-zerocopy（基于 batch-1009，U4 / U11 §5.5.6） | 映像落为 Rust 常量；体积 ≤+5%、启动装载 ≤1 ms |
| C1d-a-a5-4 | ⏳ | 终态 DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468；10-11 c1d-a54 dea3057d jimage 资源恒未压缩，DeepCopy 3033→3001、StockTrans 3031→2999、TSDS 3038→3006、HelloWorld 580（计划 §38）；余：来源 3 描述符精确构造器模型（−295）、a5-4f 日志后端（−44）、§29 能力③（−30，随引导映像）、a5-4e ICU、格式串常量求值 |
| C1d-a-precheck | ⏳ | 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3） |
| C1d-b-b2 | ⏳ ◀── why2-93e0f28e 取证 | 任务 2 |
| C1d-b-b3余 | ⏳ | URL$DefaultFactory 反射构造器扇出收窄 |
| regress2 遗留 | 🔄 ② ✅；① 转 object-bytecode | 过渡 `<init>` 帧 ✅ 已随过渡手写删除消失；Object.wait 帧仍错（单帧 -1，JDK 为 `wait0` native + `wait` 行号帧）——根因是根类 `wait` 三重载有字节码却整体手写（还跳过 Blocker 载体补偿）。按手写边界规则（有字节码即翻译，非用户待定项）派 object-bytecode：根类非 native 方法按字节码翻译。边界用例 TestObjectWaitFrames（作业 r2-wait-a79e2b60，修前为已知失败）。regress2 文档 §10.1b |
| C4 收官 · 全量 e2e | 🔄 10-09 dev 上开跑 | JDK 21 ⊇ 1029 例基线；前置：合批（batch-1008 起）合入集成分支，以及改名 rava 与 dev BIOS 维护窗口。10-06／10-07 的首轮全量分诊修复已合入（c4-preflight / c4-regress / c4-misc / c4-runfix 等） |
| JUnit 依赖包测试 | ✅ J3 / J4 已合入（batch-1010p，2226e47f） | J0–J2 ✅（f9298933 / ea2627ec）；J3 ✅、J4 ✅（身份哈希 2 例经批准改测试源）；任务书 `docs/plans/2026-10-05-junit-e2e-deps-task.md` §七 14–16 |
| 框架驱动 API 覆盖 | 🔄 dev 上复算 S0 闭包面（10-10） | S0 第 1 步 ✅ c76c800e；闭包两变体在 15G 云服务器上未产出，dev 恢复后复算 |
| build-memsafe | ✅ 10-11 交付，待合入 | 内存友好缺省构建档（16 GB 机器全部可构建为硬约束） |
| 声明层底段收窄 | 🔄 S7-4 10-10 提前开工（用户定） | **10-11 用户裁决路线 A**（类型标记 crate；每类方法 trait 属生成层内部细节，同 `ObjectVTable`；按 §3.1 取法 A 接受常数级分派开销；标记 crate 按档案生成；语料档案目标形态 ⑤），实施分解 K1–K8 见 S7 计划 §9.9，s7-marker 分支推进。先做依赖图模拟验证「最大 SCC 降到 22%」，且 22% 段本身 ≤1.3 GB；模拟同时对比底段可达集 / 全集、逐入口统计扇出咽喉、在现形态底段实测阶段剖析（time-passes / timings），类型标记 crate 评估纳入预编译 java.base 维度；结论汇总进 S7 计划 §9.8。D8 分段已合入：上段每段约 330 类、约 1.27 GB（10-11 起上段按上游感知峰值预算切段，§9.8.4）；底段 `java_base_decl` 是含 INFRA 的签名 SCC（约 76% 类），现状形态即下限，峰值 7.9 GB（D8 时）→ 4.9 GB（10-08 CollectorsDemo，sg2）。终态：S7-4 / S7-5 把最大 SCC 收到约 22%，D8 机制自动切段，每个声明 crate ≤1.3 GB，D8 无需改。计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` §九（§9.5 / §9.7） |
| 引用类语义 | ⏸ 暂缓（C4 之后） | 无 GC 模型，`docs/plans/2026-10-07-no-gc-memory-model.md`；10-09 补第三节约束 1–8（Weak 可靠、SoftReference、OOM 偏差、侧表 / PARKERS 回收、cycle_finder、逃逸分析与对象头不变量） |
| 无 GC 第一步：逃逸分析 + 区域整组释放 | ⏳ 合批测试中（batch-1011j） | PartitionInteger 内存恒定 57 MB（原 19.8 GB OOM）、输出一致；release 运行 46 s（JVM 0.71 s，stream 管线执行开销，转运行时性能线）；dev 抽查 8/9（PartitionInteger 300 s 运行超时）；全量单测 nogc-unit3 630/0。现状、实测与下一步见无 GC 文档 §六 |
| 并发小步 A | ✅ 已合入（batch-1010l，583edccd） | 第 1–5 项 78d2b9c6 / 4400cc70 / 2031ac5c / f00af3cc；违例修复 ad7f59f6（登记表锁内释放对象）/ ff1caecf（持锁执行 Java 代码）。debug 档抽查 20 例 0 断言违例，失败 2 例（TestConcurrentClinit / TestJucSync）基线同败。断言只在 debug 档生效。无 GC 文档 §四、§五-4 |
| 并发小步 B | ⏳ 合批测试中（batch-1010r，与 fix-order-live、fix-xsltc2 同测） | 小步 A 遗留 3b95ceeb（持锁登记移入载体槽，thread_local_lint）；B 本体 c955162e（volatile 引用字段加锁 / 解锁 SeqCst、监视器让出后 SeqCst 栅栏，提交说明逐条论证含 IRIW）；efafe2cb（Thread.getThreads 锁内安全点，TestThreadNatives）。debug 抽查 25/25 + 5/5，单测 630/0。无 GC 文档 §四、§五-5 |
| 接口重声明根类方法重载命名 | ✅ fix-iface-obj-overload 5b6aacf0 + d4cb7a55（抽查 iobj-d4cb7a55 6/6；单测 iobj-ut-d4cb7a55 三作业全过，已并入 batch-1011i） | 接口重声明 equals(Object) 且有 equals(Self) 重载时调用点误取 `equals_obj`（GSSNameSpi E0599）；调用点改按根类命名（`owner::is_iface_root_member` 单一判定）；类侧 ObjectVTable 桥接带重载槽名（`#[equals_vtable = "Owner::equals_obj"]`）；剩余：大闭包（jgss 纳入）下 GSSNameSpi 实测待扫描器 cfg 修复后复验，见 fix-jdk25 交接 C 节，测例 TestIfaceObjectOverload；failure_patterns `iface-root-member-overload-name` |
| 审查健全性修复 | ✅ 实施完成，待合批（review-soundness 57564806 起；余项 sound-rem cd4de4d4，见「10-11 晚收拢」第 8 条） | 代码审查分诊二「待办」中健全性各项已修，结论见 `docs/reports/2026-10-11-code-review-triage.md` 五（含「余项处置」节）：Handoff Send 收窄、直接内存字粒度原子单元（rava_rawmem）、未对齐 CAS 与负长度抛 IAE、rava_coro Drop 不 panic、描述符维数 / 注解深度上限、runtime_impl 按平台读常驻集。全量单测 rs-ut2 rc=0；抽查 rs-spot2 11/11、rs-spot3 1/1 通过。sound-rem 交付四余项：批量快路径重做（fc2e979d，Miri 2 种子过，kr1 批量段中位 5 ms 达标）、update 泛型内联（5bb96f9d，CAS 比无 sound-rem 控制组快 4–6.5%）、堆数组跨元素 RMW 边界修正与回归测例（ed4244b1 + 8cf51774 同条双锁死锁修复，TestHeapArrayAtomicView，抽查 9/9）、byteBufferView ISE 根因（flavor 类 `$ByteBufferHandle` 不入数组族判定，修复留专项）；cd4de4d4 普通读写 #[inline]。普通段对基线 +25% 回退主体在 batch-1011i/j/k 自身（控制组复现），转主会话 |
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

### 10-12 普通段（putInt/getInt）+25% 回退：二分完成、已修复（分支 plainseg-bisect，7977eca4）

main 第 12 条（10-12 立条）处置完毕。**根因不是 nogc-escape 也不是闭包变化**：kr1 同作业四 sha 二分（psg-bis1）显示回退在 96fc1cf4..c6b126cc（1011h）区间已全量出现（普通段 64–65 → 87–90，+37%），fe923a2f（nogc-escape 合并前一刻）87–88 与 5df7e9ba（合并后）87–90 同水位——nogc-escape 与 1011i/j 内容对普通段零贡献；生成树对照（psg-gen1）证明直接路径生成物逐字节一致、唯一代码生成差异是 build-memsafe（0e3cd83d，用户 10-06 定）把 `[profile.release]` 的 lto 从 fat 改 thin。机制：Unsafe 直接内存每次调用经 4–6 层跨 4 crate 包装，fat LTO 整链内联、thin 按模块导入决策不再稳定内联；用户看到的 64→74 是「thin +24 ms、1011k rava_rawmem 改善 −14 ms」的净额，sound-rem 的 +8% 是同因的 thin 摘要漂移。**修复（7977eca4）**：driver 编译段 `--release` 时按内存余量决定 lto——fat 链接估计（mem_budget ReleaseMax 模型）≤ 探测可用内存（含 cgroup）× 50% 就地把工作区 Cargo.toml 改回 fat，不足保持 thin（10-06 约束原样成立）；`--release-lto fat|thin|auto` 钉死（rava build/compile 与 run_tests 同名透传）；auto fat 链接 OOM 先降级 thin 再退半作业数；选择记 build_status.json。driver 单测 4 例。验收：psg-verify1（kr1 三段 vs 基线 96fc1cf4）、psg-spot1（直接内存 6 测 + PartitionInteger dev 超时为已知既有，release 复跑 psg-part1）、psg-utdrv（driver 单测）。工具入库：`scripts/direct_mem_bench_multi.sh`（多 sha 同作业二分）、`scripts/direct_mem_bench_profiles.sh`（同检出 thin/fat 两档对照）、`scripts/gen_diff_bench.sh`（多 sha 生成物对照）。详见分诊报告「六」与 build-memsafe 计划「六」。

（psg-verify2 补记，b46fdb7d）kr1 验收：普通段 **63–65（中位 63，判据 ≤66 ✓，回到 fat 基线水位）**、CAS **357–368（判据与 446 持平 ✓，且优于 fat 基线 404–413——inline 后 CAS 链 getIntVolatile/compareAndSetInt 也收入）**、批量 7–8 ✗（判据 ≤5；branch 无 sound-rem 的批量段后续改进，thin 下批量链未内联）。7a84d579 补批量链（native_memory / rava_rawmem 的 read/write/fill/copy 与 memory.rs setMemory0/copyMemory0）#[inline]，psg-verify3 复测中；psg-spot2（b46fdb7d）直接内存五测复跑通过中。合并者请以 verify3 / spot2 结果关闭或续派批量段。

### 10-12 batch-1013 全量编译回归（装箱工厂 vs 方法 trait 跨分支缝）：已修（分支 b1013-fix）

抽查 33/33 同根因 E0599（`Boolean::valueOf_z` 等不在 `boolean::Boolean` 上）：s7-k3 的方法 trait 化与主 lineage 后续给手写层新增的生成类成员直调（装箱 switch 84d9468f、dispatchUncaughtException 9aed6cd6、SyncFailedException 1b8728ec 等）互为盲区，合批 f7b69314 才相撞。修法两支：装箱工厂族 `valueOf_*` 进 `moves_to_trait` 留固有名单（宏 `boxing_factory_stays_inherent` 守护）；手写层 25 文件补 `X__Methods as _` 对齐（含 `#[jvm_native]` 静态注入 `Self::__class_init` 的系统性缝——非 glob 伴生全量补齐）。闭包零变化（TypeRef 同类同路径）。排查清单与 K6 遗留注意见 S7 计划 §9.9「协议面保留名单扩充」；failure_patterns 登记 method-trait-handwritten-seam。验收（abdd523b，全过）：bfx-acc（us1，rc=0）5/5 PASS——ArraysStream/HelloWorld/TestJunitAssertFamily + TestDirectBuffer(Paging)（覆盖 zip 伴生修复）；bfx-seed（jp1，rc=0）四例种子 0/1/2 闭包对 adf4cc1f 逐项全同（ALL-IDENTICAL，HelloWorld 641/2108/61、CollectorsDemo 662/2206/62、DeepCopy 2660/15390/801、TestSerialLookupPairing 2662/15380/795）。

### 10-13 batch-1015 批内项目模式 rs 编译回归（用户 crate 方法 trait 导入走包路径）：已修（分支 b1015-fix）

`guava/GuavaCollectionsDemo` 项目模式 golden 链 JVM 侧 ✅ → 转译 ✅ → rs 编译 **E0432**（`unresolved import com::example::guava::GuavaCollectionsDemo__Methods`，main.rs 匿名导入行；连带 E0599——`main` 在 trait 上，导入失败两错同报）。**层数记录：修复后直达 GOLDEN OK**，用户验证轮（35263131）已知的 E0308 缺口未再露出（j25-r2-fix 的 SAM vtable 桥接已收掉，E0432 是最后一层）。根因：K3（430f6bd8）给 main.rs 主类方法 trait 匿名导入复用了类型导入的包路径形——JDK / lib crate 的包 mod.rs 有 glob 再导出所以成立，而用户 crate 包 mod.rs 是按名再导出（P5a(a) 既有设计），trait（宏在类模块内的展开项）不在包路径上；语料 e2e 全部无包声明（两形同路），多文件项目模式（com.example.* 包 + 独立 lib crate）从未验过 rs 编译，潜伏至今。嫌疑面排除：与 s7-k6 标记层无关（用户类不拆层、无标记文件），类文件侧 fill.rs 的同 crate 定向本就正确。修法（emit/project/entry.rs）：主类与钩子类的用户类路径统一走 `phase2::uses::class_use_path` 的模块层定向（`crate::<包>::<模块>::<类>`，与 fill.rs / 钩子路径同一口径，不特判；顺带收编 class_init_hooks 手拼路径——包段关键字转义同源）；JDK / lib 类的包路径形与用户 crate mod.rs 按名再导出设计均不动。`METHODS_SUFFIX` 全部发射点复核（fill / boot_image / entry）无同族残留。验收：guava GOLDEN OK（dev 本机，transpile 43.1s / compile 41.9s）；templates/ThymeleafDemo 同链验在跑（32G 硬上限 systemd 单元 b1015-thymeleaf2，闭包阶段 18 GiB 未见增长失控；结果与 THY_RC 见 /tmp/b1015_thymeleaf.log，通过则 golden rs 产物入库）；**esper 段如实记录到已知层**：两入口共用闭包输入在 32G 上限内不收敛（PatternDemo 转译 RSS 33.1G 被杀，LengthWindow 同症不重复消耗）——新发现病态闭包（此前被双版本失配挡住、未入五库清单），**归引擎线**，阻断层在 E0432 之前（闭包期未到 rs 编译）、与本修复无关；四例种子 0/1/2 闭包集合对 batch-1015（63617c4d）逐项全同（s7k6_seed_sets 口径 ALL-IDENTICAL）+ seed_check 字节确定性 0 diff（HelloWorld / TestSerialLookupPairing）+ 确定性两项（closure_independent_of_order / of_hash_seed，b1015-det 作业 32G 上限内在跑，DET_RC 见 /tmp/b1015_det_tests.log）+ generator `cargo check --tests` 与 emit 单测 99+2 全过。failure_patterns 登记 `user-crate-methods-trait-package-path`；修法与层数详录 S7 计划 §9.9「用户 crate 方法 trait 导入路径统一」。

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

- **转译耗时回归（perf-regress4，第四轮达标，待合入）**：见 [`docs/plans/2026-10-09-transpile-time-regression.md`](plans/2026-10-09-transpile-time-regression.md)。CHM 表合并的真因是 `SerialCallbackContext.obj` 按类共用：序列化写方的全部对象图成了反序列化 `setObjFieldValues` 的写入目标，引起整堆字段互灌（8ce97959 让 `defaultReadObject` 路径入档案后触发）。c3ec480d 让值持有者（final `Object` 字段由构造器从实参写入）按对象分开。us1 同机对照（p4-t2）：DeepCopy 527 → 306 s / 4.1 GB，JNDI 606 → 348 s，序列化两例约 520 → 约 310 s；类集合对 batch-1010c 无新增（减少 17–40 个）。

- **集成分支与 main**：rust-closure-analyzer = main，已含 batch-1013（669be8c6 起）。batch-1012（fcabee4a，含 1009–1011）与 batch-1013 均已放行。改名 rava 于 10-09 完成（目录、origin `yw/rava.git`、脚本路径）；dev 检出的 origin / 目录待 dev 恢复后改。
- **batch-1012 放行记录**：单测仅已知失败；抽查 53/55（TestUrlParsingFaces 已知，TestJndiNoProvider 转译超时、放宽重跑通过）。java_base OOM 根因与修复：s6（c18e8fc4）后启动映像对象 7702 → 19347 且集中在单个 static 与单个启动函数，charset-ext 将映像分 24 段、启动函数拆 42 个（b3a860ac）、重定位先于回放（160789c7）、`ImageData::writes_statics_of` 统一判定（14917aed），java_base 峰值 11.8 GB → 1.6 GB。
- **batch-1013 放行记录**：fix-1011 后续（3f78902a）+ reflect-marker（d51837e4，DeepCopy 3553 → 3532，HelloWorld / CollectorsDemo 3304 → 3263）+ logger-chain（65ab8a99，含 seed-chain c7fbaf8c，HelloWorld / CollectorsDemo → 3233，DeepCopy 3511，LogManager 0）。单测 A 组仅已知失败、B 组全过（`profile_union_key_and_coverage` 本次通过，第四根因仍在）。抽查 50/55：TestUrlParsingFaces 已知；DeepCopy、TestJndiNoProvider、TestSerialDefaultSuid、TestSerialUserGenericCallbacks 转译超 600 s，放宽超时重跑全部通过。
- **转译耗时回归**（第四轮达标，perf-regress4 待合入）：转译秒数 DeepCopy 527 → 912、TestSerialDefaultSuid 538 → 862、TestSerialUserGenericCallbacks 514 → 907（batch-1012 → 1013），TestJndiNoProvider 324 → 570 → 1006（batch-1009 → 1012 → 1013）；疑点 78b744fe 与 batch-1012 区间。目标：4 例回到 batch-1012 水平以下（JNDI ≤350 s），DeepCopy 峰值 ≤6 GB，闭包类集不变大。
- **fix-1011 第四根因**：具体求值站点（`Class.getGenericInterfaces`）回退普通分析后，已写入映像的缓存组没有撤回；终态做法是分析结束时删除只由回退站点贡献的缓存组（引导映像计划 §5.8.6）。
- **日志链缺口 ③**：HelloWorld 3233 未达 537 / 583；剩余持有者 `logRuntimeExit@74` 的 `log(DEBUG)`，需把 `isLoggable(DEBUG)` 按映像值折叠为 false（§5.9.7）。
- **lc3-fix（4c42ac44，基于 batch-1009d / logchain3）**：修复 ReflectionAPI 运行超时。根因是 `Class$Atomic.<clinit>` 经 static_const 折叠，按名取偏移（`objectFieldOffset`）从未放开 `Class.reflectionData`。Unsafe CAS 只写流图、不进常量格，getfield 被折叠成 null，`newReflectionData` 的 CAS 因此无限重试；收窄后的闭包暴露了这个潜伏缺陷。修复在 `hw_offset.rs::site_offset`：消费符号偏移即 `open_field`，恢复「按偏移访问 ⇒ 字段不折叠」。dev 验证：抽查 6/6（ReflectionAPI 运行 0.02 s），单测 A 11/11、B 全过。
- **注解反射 NPE 回归（TestAnnoReflect / TestAnnoDeepAccess）同由 lc3-fix 修复**：二分定位到 reflect-marker 78b744fe（边界类字段改为「手写层提及才开放」，`Class.annotationType` 只经 Unsafe CAS 写、偏移在构建期初始化的 `Class$Atomic.<clinit>` 按名取得，故被折叠成 null）。与 reflectionData 同一缺陷；lc3-fix 在符号偏移消费点放开字段，dev 抽查 anno-on-7d261f96 两例通过。另一修法 anno-fix（fe1ad3ab：读取映像偏移槽即放开字段）放开范围更宽且暴露 `reflect_new_array_element_precision` 的顺序依赖（须随 order-findops），弃用、分支已删。
- **batch-1009c 已合入（10-09，按用户指示先合入，单测与 57 例抽查转合入后验证）**：含 bootcache（缓存组 = 未回退站点贡献的组，§5.8.6）、c1d-rest（C1d 收窄余项 0 类；c1d 计划 §31 记下一步能力：① 每对象 URI 跟踪约 285 类；② 引导区未知名字调 `Charset.isSupported` 放开全部扩展字符集约 390 类，charset 线续作；③ Formatter 常量格式串构建期求值）、regress2-rest（② ✅，① 转 object-bytecode）、annot-sig（SignatureParser 出闭包，HelloWorld 3456→3407、DeepCopy 3757→3727，`rava closure` 口径；e2e 口径另计）。
- **进行中（10-09 派）**：根类非 native 方法按字节码翻译（object-bytecode）；日志链缺口 ③（logchain3）；转译耗时回归第四轮已达标（perf-regress4，c3ec480d，待合入；根因与实测见 2026-10-09-transpile-time-regression.md「第四轮」）。**待派（按序）**：并发小步 A → B（无 GC 文档 §四，10-09 定）；C1d §31 三项能力。
- **派发点顺序依赖（order-findops，10-10 完工入 batch-1010b）**：FindOps 根因为 `absint/oracle.rs` `returned_params` 在尚无返回路径（⊥）时答 None（汇合），改答空集（§5.8.6），`closure_independent_of_order` 通过，HelloWorld / DeepCopy 方法各 −1（`ForEachOp$OfRef.get`）。`closure_independent_of_hash_seed` 仍失败（TestSerialDefaultSuid 种子 0 多 `Nodes$CollectionNode.forEach`）：`engine/ctxsel.rs` `selector_ctx` 读尚未定论的常量格选上下文（`ArrayDeque.grow` → `Arrays.copyOf` 常量阶段按调用点克隆，撤不回），终态修法为选择子掩码非空即一律按调用点克隆（续作 6，见 2026-10-08-annotation-signature-closure.md），10-10 派 ctxsel-mono。
- **c1d-fmt2（10-10 完工待合批，分支 c1d-fmt2）**：C1d 能力③，详见 c1d 计划 §33.7。
  - 机制 B（`[concrete] object_results` = `Formatter.parse`）已恢复。内存受控靠结果对象按内容合并，用户 printf 不再污染 Formatter.format 靠枢纽形参槽污染。
  - 修复具体上下文的映像静态活性缺口（LahNumbers NPE，failure_patterns `concrete-ctx-image-static-dead`）。
  - 对 main b1dae15c 实测：

    | 用例 | 基线（类 / 方法，峰值） | 本分支 b04de767（类 / 方法，峰值） |
    | --- | --- | --- |
    | DeepCopy | 3038 / 16715，3.14 GiB | 3028 / 16587，2.80 GiB |
    | LahNumbers | 2158 / 11077 | 675 / 2431 |
    | HelloWorld | 577 / 1896 | 580 / 1896 |

    HelloWorld 的 +3 类是健全性代价（`ClassRepository.NONE`）。
  - 单测 A / B 0 失败，hash_seed 通过，抽查 30/30。
  - **main b1dae15c 的 `closure_independent_of_order` 失败**：DeepCopy batch 1 / seed 1，差异在 `boot_image_data.live`，fmt2-ov-b1dae15c，batch-1010g/h 引入。
  - 余项：DeepCopy 的 Calendar 仍经日志链（`ObjectInputFilter$Config.<clinit>` → System.Logger → MessageFormat）与真正未知格式串（`SimpleConsoleLogger.format` 属性、`toGMTFormat` 资源束串）可达；TestStringFormat 有机制 C 的 owild 污染；「仅身份」映像活性层未做。
- **引导映像零拷贝（boot-zerocopy，10-10 完工入 batch-1010b）**：映像表改链接期符号、驻留查找回填运行期表；整进程墙钟 HelloWorld 195 → 178 ms、DeepCopy 314 → 292 ms，二进制 −0.9%。未达标：`__boot_image_start` 156 / 176 ms（目标 ≤1 ms，热点采样作业 zc-prof-669365cf）、DeepCopy 门面峰值 1779 MB（目标约 1.6 GB，需映像静态按块分 crate）；release 档待大内存机器；D5 残差区、S6 标准流未做。续作入口：引导映像计划 §5.10.5（属纯优化，按 10-06 分级暂缓）。
- **fix-e0283-nd（10-10，C4 全量失败 NormalDistribution）**：根因——null 存入 / 汇合 / 实参等路径先把 null 落成无类型 `Default::default()`，随后再经 Object 边界或 `From` / `Into` / checkcast 转换（`<T as From<Object>>::from(Object::from(Default::default()))`），源类型不可推断（E0283，`AbstractPipeline` 局部 `p` 的跨实例化重建）。修法：在转换构造的公共入口统一处理——`sim::exprs` 的 `object_from` 对无类型缺省值取 Object 的 null，`qualified_from` / `into_call` / `instr::coerce::cast_node` 取目标类型的 `<T as Default>::default()`，`from_call` 原样返回；文本层 `to_object_text` 与 `unify` 的 `from_object` / `from_common` 同口径；存储重建 `rebuild_via_object` 对 null 直接取声明类型缺省值。验证：抽查 fixnd2（jp2）NormalDistribution 通过；单测作业 fixnd-ut2（dev，generator 除 driver）全过。
- **fix-clone（10-10，C4 全量失败 TestJucSync / TestLocaleCurrency / TestLocaleDateCjk / TestParallelCapable / TestRandomFactoryAll / TestProcessBuilder）**：三个根因——① `offset-field-folded`（JucSync / RandomFactoryAll / ProcessBuilder）：只经 Unsafe 按偏移写的字段在构建期初始化类 `<clinit>` 按名取偏移、未放开而被折叠，5177e182 在符号偏移产生处（`field_offset`）即 `open_field` 收口；② 类镜像身份哈希构建期与运行期不一致（ParallelCapable）：映像内以 Class 为键的 WeakHashMap / HashMap（`ParallelLoaders.loaderTypes`、`Reflection.fieldFilterMap` 等）在运行期按地址哈希查不中，96143cf3 令镜像身份哈希按所指类型名确定（求值器 `fnv32("m:"+名)`，运行期 `Class::__pin_mirror_hash` 同值）；③ 地区补种漏别名常量（LocaleCurrency / LocaleDateCjk）：`Locale.CHINA / PRC / TAIWAN` 在 `<clinit>` 以另一常量赋值，溯源器只认字面量 invokestatic，zh 资源束类不进闭包而回落 root，222fd151 补别名溯源与中文候选链补文字 / 地区。验证（dev，222fd151）：抽查 9/9（6 例 + HelloWorld / DeepCopy / CollectorsDemo）；单测 fixclone-ut222c（closure_cli 11/11）/ fixclone-ut222r（其余全过，0 failed）。known_failures 删 TestLocaleCurrency（原误记为 expected 基线）/ TestLocaleDateCjk。
- **已知单测失败**：`param_string_constants_fold_switch`。在缺少相应修复的分支上还会出现：`container_elements_per_object` / `known_gate_ranks_first`（缺 fix-1010）、`profile_union_key_and_coverage`（第四根因修复前）。
- **known_failures**：batch-1008 新增 TestBootLayer、删除 TestXmlSaxEvents；抽查已知失败 TestUrlParsingFaces。
- **派发规则**：子代理上限 5，不得再派代理。协调巡检自动攒批、空闲即测、放行合入与清理。
- **暂缓**：
  - build-memsafe；纯优化线（10-06 分级）；U1 重议；
  - 引用类语义（无 GC，C4 之后）；声明层底段收窄（C4 之后）；
  - 等 dev 恢复：S0 Spring Boot 闭包、确定性单测（`closure_independent_of_*`）、重例。
- **fix-domstack（b4da5107，2026-10-10）**：C4 全量新失败 TestDomResultNode 与 main 上同样失败的 TestResourceBundleFaces 同源（转译期主线程栈溢出）。根因：常量实参求值（`consteval.rs`）穿过静态分派转发方法不计深度（ef6a1249 引入），`ResourceBundle.findBundle` 自递归且整数实参逐层变化，每层求值键不同、`memo_enter` 同键截断失效，`const_eval → aux_analyze → invoke_result` 嵌套约 1200 层。修法：嵌套位置改为 `EvalDepth { nest, pass }`，穿过次数上限 `MAX_PASS = 8`，超出按普通调用计深度，两者都进记忆键（记忆仍与处理次序无关）。验证：dev 抽查 TestDomResultNode、TestResourceBundleFaces / GetKeys、71_xml 其余 9 例、HelloWorld、DeepCopy 共 14 例通过；TestXmlTransform 在 main 上转译 30 min 超时，本修复后转译 2m49s、编译通过，运行期止于 XSLTC translet 实例化（已改 known_failures 签名、登记 failure_patterns `xsltc-translet-create`）；单测 A（closure_cli 11/11）、B 全过（`param_string_constants_fold_switch` 本次通过）。诊断法：栈溢出无 RUST_BACKTRACE 输出，dev 上用 `gdb -batch -ex run -ex 'bt 80' -ex 'bt -60'` 取栈。
- **fix-xsltc（2026-10-10，方案待决）**：TestXmlTransform 运行期 `Translet class loaded, but unable to create translet instance.` 分诊：运行期定义类没有终态承载——`TemplatesImpl.defineTransletClasses` → `ClassLoader.defineClass1`（手写）对格式合法的类文件一律抛 `LinkageError`，被包成 TRANSLET_OBJECT_ERR；不是生成器 / 闭包 / 反射缺陷。终态方案：按内容寻址的预定义类（构建期取得运行期会定义的类文件、按用户类翻译进用户 crate，运行期 defineClass 按 SHA-256 查表，未命中维持 LinkageError）；字节来源 S1 构建期求值 / S2 训练运行（建议）/ 两者并用，以及训练触发方式、同字节多次定义语义，待用户定（X1–X3）。见 [`docs/plans/2026-10-10-xsltc-translet.md`](plans/2026-10-10-xsltc-translet.md)。旁证遗留：未捕获异常报告只读 `cause` 字段，不打印经 `getCause()` 覆盖给出的 cause。
- **C4 全量**：10-09 在 dev（128G / 16 槽，Memtest86+ 四轮 0 错误、BIOS 风扇曲线已调）上 `--reset` 开跑，基于 main c249cdec，放宽超时（转译 1800 / 运行 900 / 构建 ×2，单例 9000 s）。转译耗时回归未修完即开跑，修复合入后续用例自动受益；全量期间 main 冻结语义改动，只合修复全量失败的提交。
- **C4 运行超时 4 例分诊（c4-runtimeout，10-10）**：AmicablePairs、RamanujanPrimes、SelfReferentialSequence、TestParallelArrayCas（另 TestCommonPool 同症）在 c249cdec 上运行超 900 s，均为语义缺陷（并行流 / ForkJoinPool 卡死），不是 debug 档性能：10-06 的 3f59a8e9 上运行 13.3 / 1.3 / 42 / 0.71 s（CommonPool 0.1 s）。根因属 `offset-field-folded`（只经 Unsafe 按偏移写的字段在构建期初始化类 `<clinit>` 按名取偏移、未放开而被折叠）；c249cdec 不含 lc3-fix 4c42ac44，集成分支头 589052eb 抽查 c4rt-a 5/5 通过（运行 2.53 / 0.84 / 6.97 / 1.13 s，CommonPool 0.17 s）。无需新修复；fix-clone 5177e182（符号偏移产生处即放开）是同类终态收口。分诊中重复实现的 dead3574 已撤回（5743e146）。已登记 `docs/failure_patterns.toml`。
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
| F 纯优化 | 二进制 ≤3 MB、S7-4～5（S7-3 已合入）、VT `instanceof` / `checkcast` 走 `__ClassDesc`、IR 结构化收敛 / TypeIR G4、PGO（前置：perf 剖析拆分开销，含 volatile 读 SeqCst 排队） | 暂停 |

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途 / 待验证 / 待启项（2026-10-09）；已完成与过时行已删，见历史 §J 与各计划文档。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| 缓存组回退撤回 | ✅ bootcache 9a029858 → batch-1009（待测） | 具体求值站点回退普通分析时，撤回只由该站点贡献的映像缓存组；修复后 `profile_union_key_and_coverage` 应通过。引导映像计划 §5.8.6 |
| reflect-marker 耗时回归 | ⏳ 改名后派 | DeepCopy 分析 502 → 889 s、峰值 5.9 → 7.7 GB，疑为 78b744fe；续作 enum-values-direct §9.6 |
| 日志链缺口 ③ | 🔄 logchain3（a6dca5c0） | 把 `isLoggable(DEBUG)` 按映像值折叠为 false，去掉 `logRuntimeExit@74` 持有者；HelloWorld 3233 → 目标 537 / 583。§5.9.7 |
| 转译耗时回归 | ✅ 第四轮达标（perf-regress4，c3ec480d，待合入） | 四例比 batch-1012 快约 40%（DeepCopy 306 s / 4.1 GB、JNDI 348 s、序列化两例约 310 s），类集合对 batch-1010c 无新增 |
| 引导映像零拷贝 | 🔄 boot-zerocopy（基于 batch-1009，U4 / U11 §5.5.6） | 映像落为 Rust 常量；体积 ≤+5%、启动装载 ≤1 ms |
| C1d-a-a5-4 | ⏳ | 终态 DeepCopy ≤2803 / StockTrans ≤2807 / TSDS ≤2809 / HelloWorld 468；余 §29 能力③（随引导映像）、格式串常量求值、a5-4e ICU、a5-4f 日志后端 |
| C1d-a-precheck | ⏳ | 按目标平台 jmod 扫描（清单落盘已做 8ed3a5e3） |
| C1d-b-b2 | ⏳ ◀── why2-93e0f28e 取证 | 任务 2 |
| C1d-b-b3余 | ⏳ | URL$DefaultFactory 反射构造器扇出收窄 |
| regress2 遗留 | 🔄 ② ✅；① 转 object-bytecode | 过渡 `<init>` 帧 ✅ 已随过渡手写删除消失；Object.wait 帧仍错（单帧 -1，JDK 为 `wait0` native + `wait` 行号帧）——根因是根类 `wait` 三重载有字节码却整体手写（还跳过 Blocker 载体补偿）。按手写边界规则（有字节码即翻译，非用户待定项）派 object-bytecode：根类非 native 方法按字节码翻译。边界用例 TestObjectWaitFrames（作业 r2-wait-a79e2b60，修前为已知失败）。regress2 文档 §10.1b |
| C4 收官 · 全量 e2e | 🔄 10-09 dev 上开跑 | JDK 21 ⊇ 1029 例基线；前置：合批（batch-1008 起）合入集成分支，以及改名 rava 与 dev BIOS 维护窗口。10-06／10-07 的首轮全量分诊修复已合入（c4-preflight / c4-regress / c4-misc / c4-runfix 等） |
| JUnit 依赖包测试 | ⏳ J3 / J4 ◀── C4 | J0–J2 ✅（f9298933 / ea2627ec）；任务书 `docs/plans/2026-10-05-junit-e2e-deps-task.md` |
| 框架驱动 API 覆盖 | ⏸ 暂缓（等 dev 恢复） | S0 第 1 步 ✅ c76c800e；闭包两变体在 15G 云服务器上未产出，dev 恢复后复算 |
| build-memsafe | ⏸ 暂缓（2026-10-08） | 内存友好缺省构建档（16 GB 机器全部可构建为硬约束） |
| 声明层底段收窄 | ⏸ C4 之后（10-08 用户定，按现有顺序） | D8 分段已合入：上段每段约 330 类、约 1.27 GB；底段 `java_base_decl` 是含 INFRA 的签名 SCC（约 76% 类），现状形态即下限，峰值 7.9 GB（D8 时）→ 4.9 GB（10-08 CollectorsDemo，sg2）。终态：S7-4 / S7-5 把最大 SCC 收到约 22%，D8 机制自动切段，每个声明 crate ≤1.3 GB，D8 无需改。计划 `docs/plans/2026-10-04-s7-object-handle-descriptor.md` §九（§9.5 / §9.7） |
| 引用类语义 | ⏸ 暂缓（C4 之后） | 无 GC 模型，`docs/plans/2026-10-07-no-gc-memory-model.md`；10-09 补第三节约束 1–8（Weak 可靠、SoftReference、OOM 偏差、侧表 / PARKERS 回收、cycle_finder、逃逸分析与对象头不变量） |
| 并发小步 A | ✅ 已完工待合批（分支 conc-step-a） | 第 1–5 项 78d2b9c6 / 4400cc70 / 2031ac5c / f00af3cc；违例修复 ad7f59f6（登记表锁内释放对象）/ ff1caecf（持锁执行 Java 代码）。debug 档抽查 20 例 0 断言违例，失败 2 例（TestConcurrentClinit / TestJucSync）基线同败。断言只在 debug 档生效。无 GC 文档 §四、§五-4 |
| 并发小步 B | ⏳ ◀── 小步 A | volatile 引用字段加锁 / 解锁 SeqCst、监视器进入纳入 SeqCst 全序，单独提交逐条论证（含 IRIW）。无 GC 文档 §四 |
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

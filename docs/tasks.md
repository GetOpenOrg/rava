# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。
> 历史任务：T01–T81 见 `docs/tasks-history.md`（冻结），2026-09-16 起的完成项见 `docs/tasks-history-2026-09.md`；R5 轮集成后的完整遗留清单见 `docs/plans/2026-09-19-remaining-issues.md`。

---

## ⚠️ 执行约束（最高优先级，不可绕过）

1. **架构问题优先**：先做架构改造，测试错误待架构完成后自然消解，禁止因为测试失败而中断架构工作转去修 Bug。
2. **架构完成前禁止全量测试**：定向验证（红线集 + 金丝雀）除外，全量 run_tests.py 只在架构节点合入后由主会话统一执行。
3. **子代理串行执行**：一次只运行一个子代理（用户指定，内存约束）；前一个完成并合入验证后再启动下一个。
4. **任务执行顺序（2026-09-23 同步）**：~~陈旧树筛→A-8/S-20/数组视图→downcast 链→S-19→A-5/A-4~~ **全部完成**。当前：ice 修复①② + `__unsafe_int_cell` 委托 + TestSealed `.0`（在途双开）→ K-6b 双侧一致（6 例）→ G-3 三重槽（窗口 3 前置）→ TypeIR 批次 3（invoke 域 9 处）→ M-3 试点 → 窗口 3 → P-1/Rust 重写 R0（三信号中"发现频率月级"仍差）。**子代理并行纪律：本机 ≤2（内存）+ 另一机 2；本地禁全量（定向 ≤10 例），全量归用户服务器**。
5. **当前执行顺序（2026-09-24 晚，用户确认）**：清单 3/11（M3）→ 4+20 → 5（TypeIR）→ 6（R-2′）**均已完成** → **7 窗口 3（进行中）** → 8 P-1 → 10 R0（门槛②③达成后）；13 M5、19 equiv 探针、N 系列按依赖穿插。1/2 收官轮待你执行，数据回来后 16–18 插队。挂决策：12/14/15。

6. **任务验收 = 端到端 Java 测试（2026-09-25 用户确认）**：每个任务以对应的 e2e Java 测试跑通为判定依据——已有测试直接用；没有就补（`tests/e2e/<类别>/TestXxx.java` + `tests/expected/TestXxx.txt` 由 JVM 生成）。构造测试须覆盖该任务涉及的**全部逻辑分支**，一个测试覆盖不全就拆成多个。定向测试跑通即视为正确、继续推进，**不等用户全量结果**；用户全量有问题反馈再处理。

---

**关联追踪文档**：
- `docs/plans/2026-09-19-remaining-issues.md` — **当前主线**：A/S/G/P/V/R 六类遗留问题全清单（架构缺口/JVM 语义/生成器质量/原则违规/验证/仓库）
- `docs/plans/2026-09-21-codegen-type-convergence.md` — **生成器类型系统收敛路线图**：擦除-恢复/字符串手术/特例 if 链的统一诊断、五层调整清单（TypeIR/M-3/A-4/转换 IR 化/downcast 链）、量化终态
- `docs/plans/java-rust-translation-reference.md` — 翻译对照（宏家族 §16）
- `docs/tasks-history.md` — T01-T81 历史全记录

- `docs/tasks-history-2026-09.md` — 2026-09-16 起完成项归档（按批次追加）

**当前基线**：JDK21 冲刺收官预期 168/169（2026-09-24，见归档）；新语料 65 例抽样 51/65（S-65，修复已推送复跑中）。历史基线段落已移入归档。

---

## 📋 开放项总表（2026-09-26 刷新）

> 分支 `claude/jolly-dijkstra-diftum`（未合 main）。只列开放 / 待确认项；完成即移入归档文档。

### 原 20 项清单（开放部分）

| # | 任务 | 状态 | 证据 / 下一步 |
|---|---|---|---|
| 1 | 服务器 JDK21 收官轮 | ⏳ **用户执行中** | 本分支预期仅剩 TestVirtualThread（挂线程模型）；TestAnnotations 已随 M3 分支转绿。服务器单 rustc 峰值 ~14G，需 `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only` |
| 2 | JDK25 全量轮 | ⏳ 用户 `--failed` 轮 @8bf6baf：**23/34 出列、余 11**（2026-09-25） | 余 11 例本机逐一复现并修复（`197dc82` + `bacf062`）：Unsafe 数组常量族（静态字段 core_ 适配）、JLA currentCarrierThread / uncheckedCountPositives、MhUtil 4 参、putDecimal（#17）、DateTimeHelper.formatTo、接口接收者超接口重载命名（E0061）、BaseLocale 常量表按版本（Locale US/DE 对调）、FJP 见证值 CAS 族。本机 JDK25 定向 14 例：12 PASS；TestCompletableFuture 推进至 FJP 层已修待复跑；TestLocaleConstants 余 fr/it 格式化（L-1 数据缺口）。**下轮 `--failed` 预期余 1（TestLocaleConstants）** |
| 7 | 窗口 3（G-1/G-2 迁 rs_ir） | ✅ **全部实施完成，待用户 JDK21 全量对账（G-2）** | 拆分方案窗口 3 结构步骤全部完成：⑧ blocks.py→unify/fusion（`5a14b30`，973→552 行）、⑨ `_store_local` 7 阶段（`9d4b007`，391→≤102）、⑩ `_hoist_if_vars` 6 阶段（`e97e49c`，375→≤106）；G-1 结构化 ✅：G1-a 块节点 BlockStmt + flatten（`cf457e3`）、G1-b 块结构判定改读 emitter 标注、删除花括号计数与文本特征判定（`49692a0`，双算 330 万次 0 分歧）、G1-c 引用检测 IR 化（双算 6.4 万决策 0 差异）。每步 23 例生成树逐字节一致——**G-1 终态（结构与引用文本匹配 = 0，Raw 节点除外）达成**。**G-2 ✅**：提升前置声明去 Default 占位 → `let mut x: T;`（Rust 确定赋值分析验证未初始化即使用）；18 例零 E0381 全 PASS（含 8 例大闭包）+ 13 例回归 + m1/m2 golden OK。**改变生成树，待用户 JDK21 全量对账**。计划 `docs/plans/2026-09-25-window3-g1-structured-hoist.md` |
| 9 | M-3 宏拆库试点 | ⏸ **待用户决策 M3-c** | M3-a/b 完成：宏侧纯位映射 + 差分审计（6 测试 120,567 处零分歧，`docs/plans/2026-09-25-m3-signature-types-in-macro.md` §五）。M3-c 取舍见 §六：建议显式类型 + 宏一致性断言，映射归属随 R0；不做 `_` 占位（可读性） |
| 10 | Rust 重写 R0 启动 | ⏳ **门槛②③实施完成，待 G-2 全量对账确认** | 门槛③ P-1 ✅；门槛② 窗口 3 全部实施（⑧⑨⑩ + G1-a/b/c + G-2）；门槛① 口径见 roadmap §四-1。用户 JDK21 全量通过即可启动 |
| 15 | libc（posix 档 B） | 📝 **已定：保持按需** | 真实用例触达目录遍历 / 文件属性 / socket 时逐 native 补，不全量手写 |
| 16 | JDK25 第四失配（`sun/security/action` E0432）及后续 | ✅ **本机冒烟全绿，待用户 JDK25 全量确认** | 用户 JDK25 全量 @4ccd3ff：81/172（compile 85 中 81 例同一 E0432，run 5 例 stub）。本机装 OpenJDK 25.0.2 逐层推进，修复链：①E0432 = JEP 486 移除 SecurityManager 后 JDK25 整包删除 sun/security/action，手写 UnixFileSystem 改调 System.getProperty（`9df7959`）；②E0599 VarHandle 签名多态方法 = project_writer 对 var_handle_impl 的过期依赖登记（`9df7959`）；③stub：Unsafe.isBigEndian、ThreadSleepEvent.<init>+Event.isEnabled、UTF_32 三件套 <init>、Thread.sleepNanos0（sleep0 改名）、JavaLangAccess unchecked*（改名）、HexDigits.digitPair（`9df7959`/`5ec92fe`/`3a3312e`/本提交）；④9df7959 引入的 JDK21 TestFilesApi 回退已修（FileSystems 手写链逐跳 upcall，`3a3312e`）。**本机 JDK25 冒烟**：HelloWorld / TestTernary（81 例大闭包形态代表）/ TestConstructorChain / TestThreadJoin / TestHexFormat / TestFilesApi 全 PASS；JDK21 受影响回归（FilesApi/PrintStreamApi/ThreadJoin/StringEdge/Atomics/CompletableFuture/HexFormat/HelloWorld）全 PASS。3 例 cargo 依赖拉取失败属用户环境（已重试）。JDK 选择：未指定 --jdk 固定走 .jdk-version=21（`9439d46`+后续），JDK25 须显式 `--jdk 25` |
| 18 | hashCodeOfUTF16（j25-edge 下一层） | 📝 用户全量未触达 | 34 例 `--failed` 轮与本机 14 例均未命中；按需原则不预实现，触达时补 |

### 新增 / 遗留

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| N2 | 序列化构造器只返元数据 | ⬜ 观察 | 反序列化实例化语义未建模，待真实用例 |
| N4 | TypeIR 扩大口径 22 处 + 完全体能力 G1–G5 | ⬜ 待做 | G1 RsType→JvmType 桥与窗口 3 同步；G4 归 M-3 |
| N6 | 手写 `_impl.rs` 分配的对象不进 RTA | 🔄 实现完成（本地 `wip/n6`，未推送），编译验证排队 jcaN6 | native_upcalls 按 fn 识别 `T::default()`+`_init_not_null()` 分配（同文件辅助 fn 传递、use/super/crate 路径解析），触达成员时登记 RTA 已实例化（仅翻译类）；生成树对照：TestArrayList +5 文件（Thread 覆盖方法，正是缺口本体） |
| S-65 | **新语料 65 例分层抽样（JDK21）：51/65**（2026-09-25） | 🔄 修复已推送，复跑排队 | 14 例失败全部归因：拼接模板含 `\n`（WordWrap/SwitchExpressionTests/PatternMatchingForInstanceOf，`cd2cf2c`）、模板含控制字符（BWT，`6e7dae3`）、大写开头参数名（WordWrap，`cd2cf2c`）、`FloatingDecimal.parseDouble` 存根（IntegerMethodsDemo/TypeCastingTest/RPN，`afe7481`）、合成槽跨分支异型（RecordPatternsTest，`3e61885`+`798c78c`）、大闭包四族（DataEncryptionStandard：兄弟分支合并 / 类型变量形参 / Object 声明首绑定 / derive(Debug) 遮蔽，`6b76885`+`e87e9b5`）、Class 未入 RTA（SumDataType，`079b9e7`，N6 同族）、环境（ExceptionPropagation OOM / NicePrimes 磁盘满 → prune.sh `8a70bc4`）、对象序列化（RecordsSerializationTest/SerializableDemo：FileOutputStream 原生层 `e5f5ba0`，本体见 S-66）。每类配 e2e（TestUpperCaseLocalNames、TestConcatTemplateWhitespace、TestParseDoubleEdge、TestPatternSlotReuse、TestBranchLocalMerge、TestObjectLocalWidening、TestTypeVarBoundArg、TestClassToString、TestFileOutputStream） |
| S-66 | Java 对象序列化（ObjectOutputStream / ObjectInputStream，含 record） | 🔄 进行中 | SerializableDemo 已推进至反序列化 resolveClass（forName0 已补，待复跑）；**record 路径阻塞于 N11 MH-native**（写侧 canonicalRecordCtr → unreflectConstructor，读侧 MethodHandle 组合子） |
| K-JCA | **JCA 服务注册**：Cipher / MessageDigest / SecureRandom | 🔄 验证中（jcaB3，17 例） | `docs/plans/2026-09-25-jca-service-registry.md`。服务表 + 种子 + 放行 + 手写边界已完成；验证链揭出并修复 10 余项通用缺口（见归档 2026-09-25/26 批次）。进展：DES 过 getInstance/init 至 doFinal、MD5/SHA-1/SHA-256 摘要输出与 JVM 一致、SecureRandom.nextBytes 正确 |
| N8 | 服务器编译资源约束 | 📝 已记录 | 单 rustc ~14G 内存；共享 target 每测试残留 0.5–1G，跑批间需清理（`scripts/prune.sh`；后台跑批用 `scripts/run_bg.sh`，自带低内存编译环境）。2026-09-25 本机（16G 容器）JDK25 TestVirtualThread（76+1699 类）debuginfo=2 下 rustc 峰值 13.8G 被 cgroup OOM 杀；`CARGO_PROFILE_DEV_DEBUG=line-tables-only` 下通过（二进制 507M→270M） |
| N11 | MH-native：MethodHandle 原生调用模型 | ⬜ 暂缓（覆盖面仅 1 例） | record 序列化前置。全部跑批日志仅 RecordsSerializationTest 需要；JDK 组合子靠 ClassSpecializer 运行期生成字节码不可翻译，须原生化。窄切口：RecordSupport 入 vm_boundary + 直接句柄经 reflect_invoke |
| N12 | JDK25 TestCompletableFuture 运行 4 分钟（JDK21 0.3s） | ⬜ 已诊断，待线程模型决策 | 协作调度下 Unsafe.park 立即返回，FJP worker 在 awaitWork 限时 park 循环空转至 keepAlive 真实截止（60s × 4）。修复需虚拟时钟或模拟线程挂起 |

## 🔴 活跃任务

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| JDK25 语料适配（原 8 例 E0308 的硬阻塞） | 数组视图族判定 | **调查报告已入档（`2026-09-23-jdk25-corpus-survey.md`，双 JDK 实测 javap 对比）**：主因=JDK22 引入 ToDecimal 抽象基类重构（实例字段→显式传参）——math 双件手写 overlay 失配（访问器 9+6 处）；DecimalDigits 迁 jdk.internal.util 全新静态类（需新增手写）；**Random 零字节码差异（此前 E0432 归因误判，需重定位）**；LVT 间隙 1 处。方案定案 **L1 兼容改写**（显式传参形态天然双版本兼容，G 表零动，~300-430 行）+ decimal_digits 新增手写（~150-250 行）；L2 版本分治否决、L3 对 DoubleToDecimal 否决（会拖 JLA 大接口）。工作量与 JDK25 全量 tee 落盘待实测清单见报告 §五 |
| **异常兜底收窄（观察项→正式，审计报告已入档 `2026-09-23-fallback-audit.md`）** | K-6b 教训跟进 | 43 吞点三分组：A 组九大 stub 吞点（实测全部触发=CfgError，固定两方法 `AbstractMap.equals`/`OIS$BlockDataInputStream.readBlockHeader`——**语义限制伪装的质量缺口活案例，建议单独跟进**）；B 组 15 处静默吞点（**全部样本零触发=死代码，收窄零损失**，未来触发必是 bug）；C 组合法保留。方案：A 组只 catch CfgError+并入 cfg-audit 位点标记、B 组新增 [fallback-audit] 审计线、JAVA_RTA_STRICT 分级（复用 build.rs 钩子）。注入实验复现 K-6b（ImportError→exit=0+stub） |

---

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| record `hashCode` 恒为 `Ok(0)` | R5 遗留 | `toString`/`equals` 已真实化，`hashCode` 未实现（S-7） |
| 手写静态 native 不触发类初始化；带 default 方法的接口自身不初始化 | S-10 剩余 | 见 remaining-issues S-10 |
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

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-1 bounds 进宏** | 无依赖 | `impl ArrayList<E>` 的 Rust bounds 由 `java_class!` 从 `generic_signature` 注入，codegen 只写裸参数名 |
| **T-2 接口泛型进类型位置**（擦除阶段 1 已落地：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`） | T-1 完成后 | 剩余：接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类/枚举/手写类的 `__interface` 覆盖 |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long` 等从字节码生成（String 已走生成） |
| **R-3 `#[derive(Debug)]` 替换宏生成 Debug** | 无依赖 | 前提：生成类字段类型均满足 `Debug` |

---

## 维护规则

1. 完成一项 → 从本表删除，整行移入 `docs/tasks-history-2026-09.md` 当期归档批次（保留提交号与证据）
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新基线数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log

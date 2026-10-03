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

7. **当前执行顺序（2026-09-29 刷新）**：FS-H0 ✅、N12 ✅、N13（改名 rava + 环境变量清零）✅、T-1 ✅（已归档）；**FS-Q1 Raw 收敛**：Q1-a..d ✅、Q1-e 十余批（单测试 raw 102.6K → 23.4K，生成树逐字节一致 + 编译验证），余值叶子收益递减、暂停；**T-2 接口载体铺开 🔄**（7a 访问器接口 ✅、7b Path ✅、标记接口 upcast ✅、7d 兼容部分 ✅、7c 桥方法签名代入修复验证中 → 名单置 None + 删两个过渡 txt）→ FS-M 组；N14 闭包收窄（按入口剖面）；#7 G-2 全量对账（用户测试滚动）→ #10 R0 启动。

6. **任务验收 = 端到端 Java 测试（2026-09-25 用户确认）**：每个任务以对应的 e2e Java 测试跑通为判定依据——已有测试直接用；没有就补（`tests/e2e/<类别>/TestXxx.java` + `tests/expected/TestXxx.txt` 由 JVM 生成）。构造测试须覆盖该任务涉及的**全部逻辑分支**，一个测试覆盖不全就拆成多个。定向测试跑通即视为正确、继续推进，**不等用户全量结果**；用户全量有问题反馈再处理。

---

**关联追踪文档**：
- `docs/plans/2026-09-19-remaining-issues.md` — **当前主线**：A/S/G/P/V/R 六类遗留问题全清单（架构缺口/JVM 语义/生成器质量/原则违规/验证/仓库）
- `docs/plans/2026-09-21-codegen-type-convergence.md` — **生成器类型系统收敛路线图**：擦除-恢复/字符串手术/特例 if 链的统一诊断、五层调整清单（TypeIR/M-3/A-4/转换 IR 化/downcast 链）、量化终态
- `docs/plans/java-rust-translation-reference.md` — 翻译对照（宏家族 §16）
- `docs/tasks-history.md` — T01-T81 历史全记录

- `docs/tasks-history-2026-09.md` — 2026-09-16 起完成项归档（按批次追加）

**当前基线**：JDK21 冲刺收官预期 168/169（2026-09-24，见归档）；新语料 65 例抽样 51/65（S-65）。**2026-09-28 用户 JDK21 全量轮**：失败 22 例——rustc OOM 3 例（sg1，已改所有编译缺省调试信息减量 `f900436`）、日志为空 19 例（mac 11 例疑 Python < 3.11 缺 tomllib，已加 tomli 回落 `76639f7`；其余 5 例多机同刻失败，疑跑批框架层，待单例重跑取转译输出）。

---

## 📋 开放项总表（2026-09-27 刷新）

> 分支 `claude/jolly-dijkstra-diftum`（2026-09-28 已合 main 至 `486a162`；其后提交在分支上待合）。只列开放 / 待确认项；完成即移入归档文档。

### 原 20 项清单（开放部分）

| # | 任务 | 状态 | 证据 / 下一步 |
|---|---|---|---|
| 1 | 服务器 JDK21 收官轮 | ⏳ **用户执行中** | 本分支预期仅剩 TestVirtualThread（挂线程模型）；TestAnnotations 已随 M3 分支转绿。服务器单 rustc 峰值 ~14G，需 `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only` |
| 2 | JDK25 全量轮 | ⏳ 用户 `--failed` 轮 @8bf6baf：**23/34 出列、余 11**（2026-09-25） | 余 11 例本机逐一复现并修复（`197dc82` + `bacf062`）：Unsafe 数组常量族（静态字段 core_ 适配）、JLA currentCarrierThread / uncheckedCountPositives、MhUtil 4 参、putDecimal（#17）、DateTimeHelper.formatTo、接口接收者超接口重载命名（E0061）、BaseLocale 常量表按版本（Locale US/DE 对调）、FJP 见证值 CAS 族。本机 JDK25 定向 14 例：12 PASS；TestCompletableFuture 推进至 FJP 层已修待复跑；TestLocaleConstants 余 fr/it 格式化（L-1 数据缺口）。**下轮 `--failed` 预期余 1（TestLocaleConstants）** |
| 7 | 窗口 3（G-1/G-2 迁 rs_ir） | ✅ **全部实施完成，待用户 JDK21 全量对账（G-2）** | 拆分方案窗口 3 结构步骤全部完成：⑧ blocks.py→unify/fusion（`5a14b30`，973→552 行）、⑨ `_store_local` 7 阶段（`9d4b007`，391→≤102）、⑩ `_hoist_if_vars` 6 阶段（`e97e49c`，375→≤106）；G-1 结构化 ✅：G1-a 块节点 BlockStmt + flatten（`cf457e3`）、G1-b 块结构判定改读 emitter 标注、删除花括号计数与文本特征判定（`49692a0`，双算 330 万次 0 分歧）、G1-c 引用检测 IR 化（双算 6.4 万决策 0 差异）。每步 23 例生成树逐字节一致——**G-1 终态（结构与引用文本匹配 = 0，Raw 节点除外）达成**。**G-2 ✅**：提升前置声明去 Default 占位 → `let mut x: T;`（Rust 确定赋值分析验证未初始化即使用）；18 例零 E0381 全 PASS（含 8 例大闭包）+ 13 例回归 + m1/m2 golden OK。**改变生成树，待用户 JDK21 全量对账**。计划 `docs/plans/2026-09-25-window3-g1-structured-hoist.md` |
| 10 | Rust 重写 R0 启动 | ⏳ **门槛②③实施完成，待 G-2 全量对账确认** | 门槛③ P-1 ✅；门槛② 窗口 3 全部实施（⑧⑨⑩ + G1-a/b/c + G-2）；门槛① 口径见 roadmap §四-1。用户 JDK21 全量通过即可启动 |
| 15 | libc（posix 档 B） | 📝 **已定：保持按需** | 真实用例触达目录遍历 / 文件属性 / socket 时逐 native 补，不全量手写 |
| 16 | JDK25 第四失配（`sun/security/action` E0432）及后续 | ✅ **本机冒烟全绿，待用户 JDK25 全量确认** | 用户 JDK25 全量 @4ccd3ff：81/172（compile 85 中 81 例同一 E0432，run 5 例 stub）。本机装 OpenJDK 25.0.2 逐层推进，修复链：①E0432 = JEP 486 移除 SecurityManager 后 JDK25 整包删除 sun/security/action，手写 UnixFileSystem 改调 System.getProperty（`9df7959`）；②E0599 VarHandle 签名多态方法 = project_writer 对 var_handle_impl 的过期依赖登记（`9df7959`）；③stub：Unsafe.isBigEndian、ThreadSleepEvent.<init>+Event.isEnabled、UTF_32 三件套 <init>、Thread.sleepNanos0（sleep0 改名）、JavaLangAccess unchecked*（改名）、HexDigits.digitPair（`9df7959`/`5ec92fe`/`3a3312e`/本提交）；④9df7959 引入的 JDK21 TestFilesApi 回退已修（FileSystems 手写链逐跳 upcall，`3a3312e`）。**本机 JDK25 冒烟**：HelloWorld / TestTernary（81 例大闭包形态代表）/ TestConstructorChain / TestThreadJoin / TestHexFormat / TestFilesApi 全 PASS；JDK21 受影响回归（FilesApi/PrintStreamApi/ThreadJoin/StringEdge/Atomics/CompletableFuture/HexFormat/HelloWorld）全 PASS。3 例 cargo 依赖拉取失败属用户环境（已重试）。JDK 选择：未指定 --jdk 固定走 .jdk-version=21（`9439d46`+后续），JDK25 须显式 `--jdk 25` |
| 18 | hashCodeOfUTF16（j25-edge 下一层） | 📝 用户全量未触达 | 34 例 `--failed` 轮与本机 14 例均未命中；按需原则不预实现，触达时补 |

### 新增 / 遗留

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| N4 | TypeIR 完全体能力 G1–G5 | 🔄 G1/G2/G3/G5 ✅（2026-09-28）；G4 归 M-3 | 扩大口径 22→0 已完成（见归档批次（五））。2026-09-28：G5 双查询面合一（stack 查询改 jvm_type 薄转发）、G2 `from_rust_type(tparams=)` → TypeVar（coerce 装箱分类 + `_coerce_arg` 7 处形参判定）、G3 `HostPrim` 变体（`_forms_alignable`）、G1 `from_rs_type` 节点桥（S3 / S8 已迁）；每步双算插桩零差异或生成树逐字节一致。进度表见 `docs/reports/2026-09-24-typeir-remaining-survey.md` 开头。余：其余「render_type → 串 → 解析」位点随 FS-Q1 Raw 收敛迁移 |
| S-65 | **新语料 65 例分层抽样（JDK21）：51/65**（2026-09-25） | 🔄 修复已推送，复跑排队 | 14 例失败全部归因：拼接模板含 `\n`（WordWrap/SwitchExpressionTests/PatternMatchingForInstanceOf，`cd2cf2c`）、模板含控制字符（BWT，`6e7dae3`）、大写开头参数名（WordWrap，`cd2cf2c`）、`FloatingDecimal.parseDouble` 存根（IntegerMethodsDemo/TypeCastingTest/RPN，`afe7481`）、合成槽跨分支异型（RecordPatternsTest，`3e61885`+`798c78c`）、大闭包四族（DataEncryptionStandard：兄弟分支合并 / 类型变量形参 / Object 声明首绑定 / derive(Debug) 遮蔽，`6b76885`+`e87e9b5`）、Class 未入 RTA（SumDataType，`079b9e7`，N6 同族）、环境（ExceptionPropagation OOM / NicePrimes 磁盘满 → prune.sh `8a70bc4`）、对象序列化（RecordsSerializationTest/SerializableDemo：FileOutputStream 原生层 `e5f5ba0`，本体见 S-66）。每类配 e2e（TestUpperCaseLocalNames、TestConcatTemplateWhitespace、TestParseDoubleEdge、TestPatternSlotReuse、TestBranchLocalMerge、TestObjectLocalWidening、TestTypeVarBoundArg、TestClassToString、TestFileOutputStream） |
| N8 | 服务器编译资源约束 | 📝 已记录 | 单 rustc ~14G 内存；共享 target 每测试残留 0.5–1G，跑批间需清理（`scripts/prune.sh`；后台跑批用 `scripts/run_bg.sh`，自带低内存编译环境）。2026-09-25 本机（16G 容器）JDK25 TestVirtualThread（76+1699 类）debuginfo=2 下 rustc 峰值 13.8G 被 cgroup OOM 杀；`CARGO_PROFILE_DEV_DEBUG=line-tables-only` 下通过（二进制 507M→270M）；**2026-09-28 实测上限**：line-tables-only + `CARGO_BUILD_JOBS=2` 下 1777 类（TestEnumAdvanced）、`JOBS=1` 下 1807 类（TestCipherDesModes，rustc 13.9G）被 OOM 杀——约 1750 类即贴线。已做：InetAddress 名字服务栈截断（closure.toml [vm_boundary]，Cipher 闭包 −1.3MB）；重型用例自动 `CARGO_BUILD_JOBS=1`（rava 编译阶段 `driver/src/cargo.rs`：声明层 `java_runtime` 类 ≥ 1700 且未显式设置时，`rava build` / `rava compile`（run_tests 经后者）均生效；显式设置 `CARGO_BUILD_JOBS` 即覆盖自动判定，重型闭包构建超时自动放宽至 3000 秒）。根治靠生成代码体量下降（N4 / FS-Q1 raw 逃生舱收敛、超大 `<clinit>` 如 KnownOIDs 的发射形态）；2026-09-28/29：所有编译缺省 `CARGO_PROFILE_DEV_DEBUG=line-tables-only`（`f900436`）、生成类 ≥ 1700 自动单作业 + 构建超时 3000 秒（2026-10-02 起在 rava `driver/src/cargo.rs`） |
| N14 | 闭包膨胀：几行 println 的用例闭包 ~1800 类（单测试 build 10–20 分钟、rustc 峰值贴 14G） | 🔄 2026-09-28 | ① VM 常量守卫死分支剪除 ✅ `0e261b6`：`vm_intrinsics.toml [vm_constants]` + `generator/crates/input/src/prune.rs`（classfile 解码后单点剪除，调用链与生成代码同源）；`ThreadLocalRandom.<clinit>` 的 `java.util.secureRandomSeed` 分支不再拉入 SecureRandom / NativePRNG 族，TestFieldEvalOrder 1827 → 1813（收益小）。② **主因是虚 / 接口分派的 RTA 扇出**：`--trace-class` 实证 `MethodHandleImpl$CountingWrapper.updateForm` 经 `Function.apply` 落到闭包内全部已实例化实现类，`PrivilegedAction.run` 同理拉入 `ObjectStreamClass$1` → 序列化 → MessageDigest / SUN provider。下一步：出分派收窄方案（候选：调用点接收者静态类型 + 实例化点可达性的 XTA / 按字段 / 按方法实例化集合；lambda / 匿名类实例化点按所在方法可达性登记），先做剖面（各扇出点贡献类数）再定。③ 2026-09-28 实验：接口分派从 CHA（全部已加载实现类）收窄到仅已实例化类，闭包 1813 → 1812——**非主因**，已撤回。剖面：1812 类 / 约 3.4 万个真实方法体 + 1.4 万 stub / 生成代码 30MB，调用链确实触达约 3 万方法；需按入口逐个做「移除该边的闭包差」剖面定位大头。④ 2026-09-29 拆 crate 可行性实测：HelloWorld 闭包 1813 个生成类的类型引用图中最大强连通分量 1761 类（97% 类 / 99% 代码量），其余 52 个均为单类分量——Rust crate 不允许循环依赖，拆分只能剥离约 1% 代码，**不可行**（打断环路需跨 crate 全部改动态分派，违背可读性目标）。降 rustc 峰值的方向：闭包收窄（按入口剖面）+ `java_class!` 宏展开体积剖面（逐祖先 From / 接口 upcast / vtable / 反射元数据中「生成未用」的部分）+ 机器侧 swap 兜底 |

### 过渡态 → 最终态（2026-09-26 用户要求建立记录）

> 全量清单 116 项（无记录 50 项；2026-09-28 核对：开放 76 行 / 81 项，K1..K6 合一行）见 **`docs/plans/2026-09-26-transitional-state-inventory.md`**（编号 FS-xx）。下表只列优先项；其余按清单推进，完成即在清单中划除。

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| FS-P1..P3 / C4 | 系统属性全集、`System.exit`、`getenv`、ServiceLoader 静态服务表 | ✅ P1 `b938ea5`（同批附带 FS-Q9 修复 `83a4ac2`）、P2/P3 `098d5e9`（用户验证 TestSystemPropsSpec / TestShutdownHooks / TestSystemExitEnv PASS，后者含 `f4d0351`+`e2f78ef`）；C4 方案已出未实施 | 验证：TestSystemPropsSpec TestShutdownHooks TestSystemExitEnv |

## 🌳 任务依赖树（2026-10-02，集成分支 rust-closure-analyzer 417a6594）

> 图例：✅ 已完成　🔄 进行中　⏳ 已立项待启　⏸ 按用户决定暂停　◇ 待用户决策
> `A ──▶ B` 表示 A 是 B 的前置。同一层内无箭头相连的任务互不依赖，可以并行。
> 每项后面的括号写分支 / 提交 / 计划文档。完成后改图例，整枝完成后折叠为一行。

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
│   ├─ ✅ regress2 第二轮基线回归 + FS-E1（6c7eb831）
│   ├─ ✅ C1d-b 第一段 c1d-pick（4b73ea61）
│   ├─ ✅ scripts-into-rava S1–S5：Python 脚本并入 rava、名字作用域统一、m3 编译错误 0（bb0b7736）
│   ├─ ✅ run-tests-prune：逐例清理产物、rava prune（ad9e938d）
│   └─ ✅ 测试分发：全部 e2e 与重命令作业走 8 台服务器（server_maintenance/rava/distribute_tests.py）
│
├─ 【当前】阶段 C 收官：闭包分析器（rust-closure-analyzer）── 用户 2026-10-01 决定先做完本阶段
│   │
│   ├─ ✅ C6 后续（c6-generic-closure 7fdbfa8c，合入 3f9d4cc9）：泛型辅助 fn 闭包形参、TestAnnoNestedArray null_recv、用户注解类型补种
│   │
│   ├─ 🔄 C1d-a 去截断（c1d-p0，2026-10-01-c1d-closure-bloat.md）
│   │     ├─ ✅ a1 具体求值器 engine/concrete/：GGI / PTI 闸门关闭，正式 HelloWorld ≈3091 类 / ≈600 s → 423 类 / 2–3 s
│   │     │       （≤360 不可达：OOB 约 52 类为用户代码真实可达、fullAddCount 约 8 类为 CAS 竞争分支，放行转 a5）
│   │     ├─ ✅ a2 合入 62f46bb2（c1d-p0 b4669206，抽查 c1da-b4669206 9/9，含接口分派宏补 null 检查 + TestInstanceofElseDispatch；合并时 name_eval 同名私有函数改名 frame_mirror_classes）；
│   │     │       🔄 续：initPhase2 膨胀用真实 --cut 定位 → 早退检查按分析期事实求值 → [[boot_init.phases]] → boot layer 步骤 2–5；闸门以档案规模计（基线 3609）
│   │     │       原记录：代码 f2bdcf6e；抽查 c1da-f2bdcf6e 7/8（StockTrans 为已知基线）：
│   │     │       TestUnixFileNatives ✅、TestCharsetNamedStreams ✅（ModuleLayer 移出 clinit_carried，新增 TestServiceLoaderLayers）、
│   │     │       FileDispatcherImpl.init0 ✅；TestFileStoreMountLookup 的 MapMode 反射构造分派缺席已修（构造器查找建模 22eb9e72，
│   │     │       新增 TestJdkConstructorLookup）；c1da-2c478e2f 的 TestDateTimeFormat 回归（缺 JRE FormatData 束）已修（d1b1b2ba，新增 TestLocaleBundleFamilies）
│   │     ├─ ⏳ 后续项 precheck 按目标平台扫描：本机只扫宿主 JDK 的 jmod，看不到 Linux 专有 native。已做：precheck 清单落盘
│   │     │       build_status.json emit.precheck、run_tests 失败详情附清单（8ed3a5e3）。待做：按目标平台 jmod 扫描
│   │     ├─ ⏳ a3 #[jvm_boundary] 归零，验收为审计数 vm_boundary_methods 归零（5c6dd98f 口径 86：Unsafe 44、VM 10、VirtualThread 10、
│   │     │       ClassLoader 6、BootLoader 5、Class 2、Module/ModuleLayer 9 归 boot layer）；拆为 U0–U3 / V / T / L1 / L2 / C / X1 / X2 / Z，
│   │     │       见计划 §21（§21.7 各项验收数字；§21.8 a3-T 终态：VirtualThread / ForkJoinPool 字节码翻译 + Continuation 有栈协程，
│   │     │       2026-10-03 用户定，方案 A 作废，细分 T1–T6，目标百万级虚拟线程）◀── a2
│   │     ├─ ⏳ 生成器 bug：`hierarchy_overloaded_names` 只查超类链、不查接口——类自有 `m(String[])` 与接口继承的抽象 `m()` 同名时
│   │     │       不 mangle，`this.m()` 解析到一参方法（E0061；`AbstractBasicFileAttributeView.readAttributes`，
│   │     │       `Files.getAttribute(p, "unix:nlink")` 触发）。修生成器 + 补边界用例，c1d-p0 合入后另开步骤
│   │     ├─ ⏳ a5-4 闭包膨胀：TestUnixFileNatives Linux 闭包剩余 18 个与文件 API 无关的缺失 native（pkcs11 10、smartcardio 2、
│   │     │       jimage 1、NativeLibraries 3、BootLoader 1、defineClass0 1）作为膨胀指纹；终态：pkcs11 / smartcardio / defineClass0
│   │     │       13 个所在类不入闭包（不补手写），NativeLibraries / getSystemPackageLocation / getNativeMap 5 个归 a3-L1 ① native；
│   │     │       计划 §21.5 a5-4，c1d-p0 合入后另开步骤
│   │     │       引入链已归因（TestFileStoreMountLookup 2885 类 / r4 extra 2156）：a5-4a doPrivileged 动作合流（1092）、
│   │     │       a5-4b 引导加载器类路径查找 → JarVerifier → Signature / pkcs11（684）、a5-4c jrt 随 b 消失、a5-4d Formatter → ICU 归 a5-3；
│   │     │       目标该例 ≤900 类、transpile ≤60 s（计划 §21.5）
│   │     │       DeepCopy 实测 3139 类（集成分支 1820，目标 ≤1640）：a5-4a 单独回收约 0（动作分配点均在合法路径，只改归属），
│   │     │       a5-4b 回收 160..401；另立 a5-4e ICU 归一化入口（248）、a5-4f 日志后端探测（231）
│   │     │       合入门槛（用户 10-03 定，先收窄再合）：DeepCopy / DTF / FSML 闭包类数与分析时间不高于集成分支
│   │     │       （约 1820 类 / 23s、1476、1611），终态 DeepCopy ≤1640；顺序 a5-4b → a5-4e → a5-4f，不足再查余下 342
│   │     │       ⚠ 10-03 实况：62f46bb2 合入 b4669206 时连带 1e623cec 去截断进入集成分支，门槛被跨过（StockTrans 1841→3107、DeepCopy ≈3139）；
│   │     │         不回退（回退即恢复 80 个过渡手写），改以收窄兑现：JCA / jar 签名簇（≈−377）→ 容器元素 Object 方法 → Latin-1 语言折叠（a5-4e）
│   │     │       s1 构造器查找只在 Class 值集齐全时点名：DeepCopy 3139→3069、3m03s→66s（暴露构造器 3498→105）
│   │     │       s2 instanceof 否定分支收窄 + 钩子字段不按 open：DeepCopy 3065 / DTF 2884 / FSML 2886；首次发现子树重排后最大三支
│   │     │       （getLoggerFromFinder 1163、toLowerCase→CLDR 783、URLClassPath$3→JarVerifier 493）均需值层面建模，原定手段不足，见 §21.5
│   │     │       抽查 ① allocateInstance 抽象类 / 接口 → InstantiationException（db4f8a48）；② LocaleBundleFamilies：EnableNativeAccess 嵌套翻译 + 无扩展名 / 拼接模板资源（本地编译运行通过）
│   │     │       ⏳ 暂不修（10-03 登记）macOS 专有：MacOSXFileSystemProvider 多级协变桥缺失，Linux 不受影响。
│   │     │         最小复现：macOS 上 rava build tests/e2e/62_reflection/TestJdkConstructorLookup.java，运行时命中
│   │     │         stub: sun/nio/fs/UnixFileSystemProvider.newFileSystem:(Ljava/lang/String;)Lsun/nio/fs/UnixFileSystem;
│   │     │         该类文件里 newFileSystem(String) 有三个返回类型版本：MacOSXFileSystem 为本体，BsdFileSystem / UnixFileSystem
│   │     │         为 javac 桥（Bsd 层的同形态是一体一桥，单级）；经 UnixFileSystemProvider 形参分派时落到存根，即 Unix 级桥没有接上
│   │     │         （疑为同名同形参、仅返回类型不同的两级桥在发射 / 分派表合并时丢失）
│   │     ├─ ⏳ a4 TestCharsetNamedStreams（自 c4-regfix 移交）◀── a2
│   │     └─ ⏳ a5 OOB 关系型边界推理（a5-1 差分约束域 → a5-2 类不变量 → a5-3 检查点判定，计划 §21.5），HelloWorld 目标 ≤371；fullAddCount 仅记录
│   │
│   ├─ 🔄 C1d-b 反射与过近似收窄（c1d-pick，2026-10-02-c1d-reflect-narrow.md）
│   │     ├─ ✅ b0 阶段合入 e90a592d（eb6571ba）：m3 serialVersionUID、同一数组自拷贝、反射字段按值流点名（TestReflectProbe ✅）
│   │     ├─ 🔄 b1′ ArrayList.writeObject 分派臂：2026-10-02 交接拆分（c1d-pick 2c16454b，计划 §4.6；WIP 快照 c1d-pick-wip 63d90e86）
│   │     │       T0 基线抽查 c1db-2c16454b → 并行 ✅ T6 getSuperclass 返回模型（7a0188e6）、✅ T5 MH→putReference 清单精确化（cc6b59e0）、
│   │     │       ✅ T7 探针转正 --flows（1518612d）→ ✅ T4 未知 Class 字段枚举收窄（2311459e；余 2 处字段全开源头归 b1） → ✅ T3 反射回调按接收者克隆上下文（4c3a614e 合入，抽查 9/10；6 s 回收，StockTrans 分派 22949→1645；MH 按句柄路由实测上界 −1 类，不做） → 🔄 T2（c1d-t2：writeObject0 对象池 / 名字×镜像 / forName0 回退 / Serializable 有界镜像） 名字×镜像交叉（最后合）
│   │     │       验收：StockTrans / TestSerialDefaultSuid / TestSerialProxyForm 通过、fold_props ≥42、m3 golden
│   │     ├─ 🔄 b1 序列化收窄：✅ S2 ReflectUtil 放行 / 静态 CAS（含子字宽）/ 返回模型收窄合入（c1d-b1 0d7dd2a5；StockTrans 1803→1794、DeepCopy 1804→1789）；余：DeepCopy 距 ≤1640 目标，归 T2
│   │     │       验收：DeepCopy ≤1640 类、fold_props ≥42、StockTrans / TestSerialDefaultSuid 回调保留
│   │     ├─ ⏳ b2 任务 2 ◀── why2-93e0f28e 取证
│   │     └─ 🔄 b3 任务 3：class_init.unknown 归 false——✅ 第一段合入 1721f701（aeff784b）：class_init 钩子、截断体同类调用登记（bool2byte）、
│   │             跳过 Class#<synthetic>、子字字段 CAS（每字段 4 字节槽 + __unsafe_word）；第二段 01c88c11 抽查 c1db3-01c88c11 10/10：
│   │             基本类型数组元素 Unsafe 访问、S3 getCallerClass（CallerSensitive 记字节码所在类）、sun/misc/Unsafe 放行、静态字段钩子每次访问连边
│   │             （修 ThreadTest）；01572ce6 协议名常量分支折叠收窄加载器链（ThreadTest 1445→346，新增 TestBuiltinUrlProtocol）；
│   │             ✅ 第二段合入 b1983313（84c92245，抽查 c1db3-84c92245 10/10；含 setContextClassLoader 存根修复 + TestThreadContextLoaderInit）；
│   │             余：扇出收窄（registerNatives 开放接收者 toString、URL$DefaultFactory 反射构造器）、✅ CallerSensitive 经方法引用 / MH（382cf3e1 合入，抽查 10/10：MN_CALLER_SENSITIVE、BindCaller 注入调用器 InjectedInvokerDyn、Method.invoke 适配；方法句柄类用例闭包 −22）；✅ S5 手写值池合并拆分（fd76553d 合入，抽查 8/8：FieldAccess.value_fresh、getDeclaringClass0 按接收者、TestDeclaringClassInit；7 例测量集类 / 方法集合不变）；✅ S7 Class.forName 拼接类名字符串值流建模（3553df09 合入，抽查 c1db3-3553df09 9/9：拼接各段可确定时折叠为常量串集合，新增 TestForNameComputedName）；✅ DMH checkInitialized / shouldBeInitialized 未知站点归零（2eb9403a 合入 3473d696，抽查 c1db3-2eb9403a 9/9：DMH 未知点建模、按名查找静态字段初始化声明类、TestMethodHandleStaticInit；合并适配 875afe99）
│   │     ├─ ✅ lambda 隐藏类（c1d-lambda-class 0060fa77，合入 94d2ff90）：每调用点 Host$$Lambda/0x… 隐藏类、超类 Object、接口 + 标记接口、
│   │     │       isHidden / isSynthetic 按类元数据、实例判定按超类型集合；TestLambdaHiddenClass；from_any 归零（2026-10-03-from-any-zero.md）：
│   │     │       ✅ ① 审计按类计数含 java_body_*（56506adb，合入 ec714d98；真实基线 2–78）；✅ ② A+B 超接口 / 接口视图类型实参（b820c8aa 合入；27 例 from_any 2–78→1–4，闭包不变）；✅ ③ C+D 方法级类型变量 / super.m()（44b3a3b3 合入；27 例中 26 例 from_any=0，StockTrans 11→0）；✅ ④ 余下发射点统一 Object::from / Into<Object>、void 入 Object 改内部错误、from_any=0 守护测试（631bb78b 合入；27 例 + StockTrans / LambdaHiddenClass 全部 from_any=0，闭包不变）——**from_any 归零达成**
│   │
│   ├─ ✅ native-gaps（bdc4cd64，合入 417a6594）：sun/nio/fs native 21 个、loop_hoist 合流变量、栈帧按声明类归属（declared_by）
│   │     已知失败（集成分支原本即失败，不是回归）：TestUnixFileNatives ◀── C1d-a a2（c1d-p0 已修）；TestModuleLayerDefine ◀── boot layer；
│   │     ThreadTest（FS-C2 A 4896b5a7 起 initSystemClassLoader 存根：静态字段钩子漏连边）◀── C1d-b b3（01c88c11 已修）；
│   │     TestClassNestNatives ✅ FS-C2；TestReflectProbe ✅ C1d-b
│   │
│   ├─ ✅ FS-C2 应用类加载器（fs-c2 9c737f03，合入 4a98f5e3）：定义加载器读取钩子、ClassLoaders 整类字节码、initPhase3（scl + 上下文加载器）、
│   │     desiredAssertionStatus 特判删除；vm_boundary_methods 30→27；TestClassNestNatives ✅、TestAppClassLoader / TestParallelCapable
│   │
│   ├─ ⏳ boot layer：ModuleBootstrap.boot 引导期建层、System.bootLayer 按字节码读取、系统模块描述符承载（FS-H12）
│   │     第 0 步 ✅ 27dfb419：镜像与 jmod 字节不同的类以镜像为准（java.base 5 类：SystemModulesMap + 4 个 MH $Holder），spot bl0-27dfb419 8/9（TestModuleLayerDefine 为原有失败）
│   │     ◀── C1d-a a2（1e623cec 的 [boot_init] 与 jdk/ 去截断）、FS-C2　验收：TestModuleLayerDefine 原样通过
│   │
│   ├─ ✅ regress2 续：栈帧来源统一（frames-unify bf91f075，合入 be1b97be）：行表单一帧源、删符号解析、手写 native / Object 成帧、StackWalker 行号
│   │     遗留：Object.wait(J/JI) 手写帧行号 -1、过渡类手写 <init> 不成帧 ◀── C1d-a a2
│   │
│   └─ ⏳ C4 收官：全量 e2e（JDK 21）⊇ 1029 例基线
│         ◀── C1d-a（a1–a4）、C1d-b、FS-C2、boot layer 全部合入
│
├─ 【近期】阶段 C 之后，依赖 C4 收官
│   │
│   ├─ ⏳ scripts-into-rava S6–S8（2026-10-01-scripts-into-rava.md）
│   │     S6 dyn 对照并入 rava（dyn crate，删 dyn_compare.py / dyn_agent）◀── C4 收官（dyn_compare 改动冻结）
│   │      └─▶ S7 run_tests 拆 scripts/e2e/，预留形态接口 ──▶ S8 产品路径 Python 归零、计划结项
│   │
│   ├─ ⏳ JUnit 依赖包作为测试（2026-10-01-junit-crate-as-test-harness.md）
│   │     步骤 0 m1..m5 Rust 路径复跑 golden，清零回归（可提前；m3 编译 0 ✅，运行期存根由 C1d-b b0 处理）
│   │      └─▶ 步骤 A：63_junit 形态接入 run_tests ◀── S7 ──▶ 步骤 B ──▶ 步骤 C
│   │
│   ├─ ⏳ R1 运行性能：超时用例（标杆 LynchBell 等 12 例）不改测试、不放宽时限
│   │     （2026-09-30-optimization-directions.md §三.4）◀── C4 收官后排期
│   │
│   └─ ⏳ e2e 扩展到 java.base 之外的 JDK 模块（2026-10-03-jmod-coverage.md）
│         用例已写入（feat/framework-pilot-matrix 4939f290 合入：64_–74_ 共 43 例，期望由 JDK 21 生成；71_xml 11 例）
│         ✅【2026-10-03 完成，feat/junit-expected-redundancy d4efc8d6】63_junit expected 10/10（junit+hamcrest cp、JDK21 实跑、双跑确定性全过；顺修 3 处源码错误：assertTrue 静态导入缺失、遮蔽 helper、Sample 构造器非 public 致 initializationError）
│         ✅【2026-10-03 完成，同分支】新增用例查重：133 例 ∩ 冗余候选 = 5、相似对交集 0，逐条论证全部保留（定向回归网/独有边界/算法族/jmod 档设计），无删除建议；报告 docs/reports/e2e-redundancy-newtests.md
│         ✅【2026-10-03 用户侧完成，feat/e2e-failure-triage ffbebc0d】6 例输出不符 expected 复核：expected 纠正 0；生成器缺陷 5（本机 6c0adc44 复现 5/5）；环境 1（TestLocaleCurrency：CLDR 随 JDK 21 小版本漂移）
│         ✅【2026-10-03 用户侧完成，同分支】e2enew-da8abee1 失败 69 例归因：13 根因族，报告 docs/reports/e2enew-da8abee1-triage.md，jmod §七 实测列已回填；派生条目：
│           ├─ ⏳ R1 xml lambda 存根 SecuritySupport.lambda$getSystemProperty$0 可达性（10 例，71_xml）◀── jmod 第 1 步
│           ├─ ⏳ R2 泛型反射 scope 构造器存根 + E0432（6 例）、R5 Method.invoke 实参数量 / 类型不符抛 IAE、R11 compareTo 桥分派闭包 ◀── 并入 T2 队列
│           ├─ ⏳ R3 JCA ProviderList / GetInstance 存根（5 例）◀── C1d-a JCA 收窄线
│           ├─ ⏳ R4 跨模块 import 断链 java.logging E0433（4 例）
│           ├─ ⏳ R6 模块元数据（isNamed / getName / isExported）+ 强封装边界（2 例）、R7 系统资源装载 getSystemResourceAsStream / findBootstrapClassOrNull（3 例）◀── boot layer（C1d-a 步骤 2–5：命名模块 + jimage）
│           ├─ ⏳ R8 beans finder 构造存根（3 例）、R9 charset / zipfs 提供者构造（5 例）◀── jmod 第 1 步前置
│           ├─ ⏳ R10 Array.set native 准入、R12 E0308 三例、R13 http async 转译错误；数组协变 Object[].class.isAssignableFrom(Integer[].class)（TestClassCastSubclass）
│           └─ ✅ 参考 JDK 固定构建（jdk-pin 75a15d93 已合入；server_maintenance b595193）：Temurin 21.0.11+10，清单 tools/refjdk.toml（四平台 URL+sha256），scripts/fetch_reference_jdk.sh；本机 tools/refjdk/，服务器 /data/rava-jdk/（ubuntu /mnt/d/workspace/rava-jdk）；run_tests 缺省参考构建、缺失即报错，--jdk/--java-home 标「非参考构建」
│              ⏳ 生成器缺陷（原误判为环境）：TestLocaleCurrency 转译产物在 Linux 输出 CN¥，JVM（21.0.11 与 21.0.12.1）均输出 ¥——locale/CLDR 取值经转译后不同，待归因（jdkpin-efc13cd0 sg2 复现）；🔄 用户侧 D（#10）归因中：JVM 侧取证完成（zh_CN 链 [zh_CN_#Hans, zh__#Hans, zh_CN, zh, root]、载 cldr/ext/CurrencyNames_zh / FormatData_zh），rava 侧闭包检查待做，报告 docs/reports/2026-10-03-locale-currency-cn.md
│             ✅【2026-10-03 用户侧完成，c3ad4c4e 已合入】参考 JDK 全量 golden 核验（docs/reports/2026-10-03-refjdk-golden-verify.md）：1074 例构建差异 0、一致 1070（63_junit 带 cp 10/10）——固定 21.0.11 零重生成验收通过；TestLocaleCurrency 在一致集内，佐证生成器缺陷改判
│                ✅（95962474 合入：改为 libzip-name-ok= 断言，期望输出与平台无关）派生：TestVmPlatformNatives expected 含 libzip.so（Linux 生成），macOS 本机输出 .dylib——平台依赖，登记跨平台基线
│                ✅【2026-10-03 用户侧完成，95962474 合入；抽查 nondet-95962474 2/3】派生：ListMethods（getMethods 枚举序）、TestSocketLoopbackPair（半关闭竞速）JVM 侧输出非确定——语料自身缺陷，后者为第八轮新写，改为确定序形态（join 后统一打印）；前者按输出排序形态复核；⏳ 转译侧缺陷：TestSocketLoopbackPair 改写后 rava 产物启动即 main 线程 NullPointerException、无任何输出（jp1，参考 JDK 21.0.11），待归因（疑 readNBytes / 线程缓冲路径）
│         ✅【2026-10-03 用户侧完成，ad62f2c1 已合入】「档案调用链」口径文档同步：handwritten-boundary 原则一、java-rust-translation-reference §8.4、environment-variables、java-bytecode-transpiler-design、compatibility 五处改为档案口径并加「当前仍单测试」现状注；[boundary] 过渡期表述、行为现状表与历史文档按原样保留
│         第 0 步 A 档用例预审（rava audit，登记闭包规模与缺口，可提前）
│          └─▶ 第 1 步 A 档 7 模块（charsets / localedata / logging / sql / random / zipfs / crypto.ec）◀── C4 收官、boot layer、b3 CallerSensitive
│               └─▶ 第 2 步 java.xml ──▶ 第 3 步 HTTP 回环 + 空提供者 ──▶ 第 4 步 beans / geom 子集
│
├─ 【中期】优化线（用户 2026-10-01 决定暂停，C 阶段收官后恢复；精度 / 效率优化都要做）
│   │
│   ├─ ⏸ 闭包分析效率 P8 余量、sites（optimization-directions §三.2）
│   ├─ ⏸ 生成器 / 下游编译成本：V1–V7、S 系列余项（emitter-performance、rustc-memory-and-crate-split）
│   ├─ ◇ S7 统一对象句柄 + 每类静态描述符 ─┐
│   ├─ ✅ T1 跨测试编译复用决策（2026-10-01-cross-test-compile-reuse.md，99dc658f 实测：档案 3609 类，全量 ≈35→≤11 机时）—— 用户 2026-10-03 四项全采纳：档案化 + 分发层、CLAUDE.md 第 2 条改写（已改）、开放世界折叠、语料动态 / 生产静态链接；C1d / C4 收官后按 §5.3 实施
│   │     └─▶ T4 生成器只构建一次再分发（待服务器核实）
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
C1d-a a1 ✅ ──▶ a2 f2bdcf6e 验收（余 1 例）──▶ a3 jvm_boundary 归零 ─────────┐
C1d-b b1′ writeObject 分派臂 / b1 序列化收窄 ───────────────────────┤
C6 ✅ 3f9d4cc9 ──────────────────────────────────────────────────────┼──▶ C4 全量 e2e ──▶ S6 ──▶ S7 ──▶ JUnit A ──▶ B/C ──▶ 真实项目 pilot ──▶ 产品化
native-gaps ✅ ──▶ FS-C2 ✅ ──▶ boot layer（另需 C1d-a a2）───────────────┤        │
               └─▶ regress2 栈帧来源统一 ✅ be1b97be ─────────────────┘        └──▶ 优化线恢复（P8 / V / S7·T1 决策 / R1 / JDK 25）
```

工期瓶颈是 C1d-a a2：闭包规模已达标（正式 HelloWorld 423 类 / 2–3 s，余量转 a5），只剩 TestFileStoreMountLookup 重跑验收（TestUnixFileNatives 已过）；a2 合入前 boot layer 第 1 步起、regress2 遗留、a3 都不能启动，C4 全量 e2e 随之顺延。

---

## 🔴 活跃任务

> 依赖关系见上方「任务依赖树」。本表只列在途分支的当前状态。

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| C1d-a 去截断（c1d-p0） | 🔄 2026-10-02 | a1 ✅ 正式 HelloWorld 423 类 / 2–3 s；a2 抽查 c1da-f2bdcf6e 7/8（StockTrans 基线），余 TestFileStoreMountLookup 重跑；之后 a3 审计数 86→0、a5 OOB 关系推理；后续项 precheck 按目标平台扫描 |
| C1d-b 反射收窄（c1d-pick） | 🔄 2026-10-02 | b0 阶段合入 e90a592d（TestReflectProbe ✅）；ArrayList.writeObject 分派臂已拆为 T2–T7（计划 §4.6），T4 / T5 / T6 / T7 已合入，b3 第一段已合入（1721f701），b1 已合入（0d7dd2a5），T3 已合入（4c3a614e），b3 第二段与 CallerSensitive 续段已合入（84c92245、382cf3e1）/ b3 DMH 段已合入（3473d696）/ b3 扇出收窄、T2 进行中；新派 A gen-fixes（生成器独立缺陷合集，计划 docs/plans/2026-10-03-gen-fixes.md）、B a3t-vthread（VirtualThread 翻译 + Continuation 有栈协程）；序列化收窄 WIP：大值集来自未知接收者字段视图，目标 DeepCopy ≤1640、fold_props ≥42 |
| native-gaps · native 缺口补齐 | ✅ 417a6594 | 已知失败 TestUnixFileNatives（待 C1d-a）、TestModuleLayerDefine（待 boot layer）、TestClassNestNatives（待 FS-C2） |
| FS-C2 应用类加载器 | ✅ 4a98f5e3 | TestClassNestNatives 通过；vm_boundary_methods 30→27；交接 C1d-a：ServicesCatalog / JLA 补丁随过渡手写删除 |
| regress2 续 · 栈帧来源统一 | ✅ be1b97be | 遗留 Object.wait 帧行号、过渡 <init> 帧 ◀── C1d-a a2 |
| boot layer | 🔄 第 0 步 ✅ 27dfb419 | ModuleBootstrap 引导建层；第 1 步起 ◀── C1d-a a2（FS-C2 ✅） |
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

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-2 接口泛型进类型位置**（擦除阶段 1 已落地：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`） | 🔄 2026-09-29：7a / 7b / 标记接口 / 7d 兼容部分 ✅，7c 验证中（方案 `2026-09-28-t2-interface-carriers.md`） | 剩余：接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类/枚举/手写类的 `__interface` 覆盖。**收尾交付**（清单整合第 4 项，`d449f77` 决定）：接口载体全面铺开后删除 `runtime/java_runtime/carrier_type_positions.txt` 与 `signature_erased_interfaces.txt`（FS-M2 最终态，不迁移进 TOML）；`overload_abbrev.txt` 保留，不另建 `codegen_types.toml` |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long` 等从字节码生成（String 已走生成） |
| **R-3 `#[derive(Debug)]` 替换宏生成 Debug** | 无依赖 | 前提：生成类字段类型均满足 `Debug` |

---

## 维护规则

1. 完成一项 → 从本表删除，整行移入 `docs/tasks-history-2026-09.md` 当期归档批次（保留提交号与证据）
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新基线数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log

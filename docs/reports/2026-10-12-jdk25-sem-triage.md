# JDK 25 语义失败分诊（j25-sem，2026-10-12）

分支 `j25-sem`（基 batch-1013 @ adf4cc1f → 并入 `b1013-fix` @ abdd523b → 本分支修复 304edfb1），
worktree `/data/workspace/rava-wt/j25-sem`，作业结果目录（worktree 本地，gitignore）
`cluster_results/job/j25s-*`。

## 0. 背景与输入完整性

- 任务输入：2026-10-12 主会话 JDK25 失败集复跑（by_commit/c7cfb932684b 归档 468 例，跑在 main@99de3268）
  → **987 过 / 138 败**。138 清单在 `cluster_results/by_commit/99de32683615/failed_jdk25.txt`。
- **主检出的 `cluster_results/failed_tests_jdk25.txt` 与 `error_logs/` 已被 17:28 的 63_junit 大槽复跑覆盖**
  （只剩 3 条 junit OOM 日志），138 例的逐例错误日志不可恢复（dev 槽目录 e2e 清理亦无残留）。
  本轮分类据此改为**在 j25-sem 分支服务器复跑重建**（作业 tag `j25s-*`）。
- 99de3268→adf4cc1f 区间无语义修复（全为 S7 K3/K6、invoke-key-opt、plainseg-bisect、coerce-check 性能线），
  138 例中的语义失败在 adf4cc1f 上应仍失败，除非属 dev 档性能类（release 下转过，见 §3）。

## 1. 前置：批级编译回归（非本任务语义类，但污染一切探测）

batch-1013（adf4cc1f）有 K6 前置项装箱工厂 trait 化的批级 E0599 回归（`valueOf_z` 在
`boolean::Boolean` 上不存在，成片编译失败）——协调者预警并给出修复分支 `b1013-fix`（abdd523b）。
本分支已按指示先并入（`j25-sem = b1013-fix + 修复`）。污染证据：作业 `j25s-rerun` G3
（adf4cc1f，15 例）中 IQPuzzle / TestReflectProbe / TestMethodHandleCombinators /
TestSecurityManagerContext 4 例同签 E0599，其余 11 例为我方缺省 600s 转译超时（配置问题，
复跑已改 `--transpile-timeout 2400`）。**对 138 的归因须剔除这两族。**

## 2. 三类分类（138 例）

### 2.1 OOM 类（75 例 @14G 槽；只列不修，G1/主会话大槽线处理）

按任务书口径列出（逐例错误日志已失，名字级来源：任务书 + failure_patterns + 交接文档）：

- **63_junit 8 例**（22G/40G 槽峰值 39.6–39.9G 仍 OOM，闭包分析规模，G1 线在攻）：
  TestJunitAssertThrowsAssume、TestJunitFailureMessages、TestJunitBeforeClassState、
  TestJunitLifecycleOrder、TestJunitIgnoreAndExpected、TestJunitNewInstancePerTest、
  TestJunitTimeoutFast、TestJunitRunnerResult。
- **已知转译 OOM（transpile-oom-14g / 交接文档）**：DeepCopy、DataEncryptionStandard、Digester、
  TestHttpLoopbackSync、TestHttpLoopbackAsync。
- 其余 ~62 例为 14G 槽上限下的 OOM（含转译与编译段），逐例名不可恢复；本分支复跑（11G 作业槽上限，
  更紧）将继续把这些暴露为 OOM，作业 `j25s-rA/rB` 的 summary.json `oom` 字段即现名单（交接见 §5）。

### 2.2 已知已修 / 已知未修类（对照 failure_patterns 与交接文档认领，非本轮新修）

已修（patterns status=fixed/pending，复跑预期转过；rA/rB 正在出数）：
- unbox-root-virtual-stub（748e0987）：TestReflectBoxCast；
- direct-invoke-override-not-dispatched（677c07d4）：TestReflectFieldMethod；
- module-unnamed-handwritten（e50d4ba3）：TestModuleLayerDefine、TestSetAccessibleBoundary；
- transpile-stack-overflow（b4da5107）：TestDomResultNode；
- canonicalize-requires-existing-path（4308ba57）：TestFileCanonicalPaths；
- url-protocol-handler-by-name（c4-url）：TestUrlParsingFaces、TestUrlProtocolOpen、TestEmbeddedUrlRebuild；
- transitional-handwritten / carrier-thread-local-stub：TestRandomAccessFile、TestUnixFileNatives；
- offset-field-folded：AmicablePairs、RamanujanPrimes、TestCommonPool、TestParallelArrayCas；
- l3-dispatch-serial-callback / static-field-table：StockTrans 族；
- root-object-handwritten-frames（78af2fa8）：TestObjectWaitFrames（注意交接文档另有「JDK 版本输出差异」口径）。

已知未修（open，登记在案、非本轮范围）：
- `jdk25-stackframeinfo-reflective-element-flow`（G1/反射建模）：TestStackWalkerFrames、TestStackWalkerLines；
- `resource-bundle-by-name`：TestXmlSaxEvents（TestXmlTransform 混见 xsltc 已修与该项）；
- `native-unimplemented`（IOUtil.fdLimit 等）：TestHttpLoopbackSync/Async（与 OOM 叠加）；
- `debug-profile-too-slow`：FibonacciMatrixExponentiation、IQPuzzle、RailwayCircuit、TestVirtualThreadScale——
  **release 下 FractionReduction 实测已 PASS（j25s-rB G8）**，此族在 release 口径大量转过；
- 运行超时：TestArraysSetAll（j25s-rB G10 实测 run timeout >10m，同交接文档「运行超时」类）。

### 2.3 新语义类（本轮修复对象）

- **逃逸分析 call_virtual↔call_lambda 无限互递归（已修 304edfb1）**：
  症状 = JDK25 转译在闭包完成后 abort（`fatal runtime error: stack overflow`，RUST_MIN_STACK=128M 仍溢出）。
  实证：FunctionalCompositionTest（sg1，gdb 栈 `escape::analysis::Analysis::call_virtual(analysis.rs:485) ↔
  call_lambda(analysis.rs:549)` 数千层）。根因：OLD/FRESH_UNKNOWN 接收者上的 call_virtual 按 SAM 名遍历
  全部 lambda 站点，call_lambda 对 5|9 实现句柄直呼 call_virtual（cap=OLD 回落）——SAM 名与实现名互为对方
  （或单站点 sam==实现名）即无限环；JDK21 档案无此形态，JDK25 首次触达。修法：在途 lambda 站点键
  （owner,name,desc）判环，命中按未知被调保守返回（同 `Callee::Native` 口径），无环链不变。
  影响面：所有触发该环的 JDK25 用例（已证 FunctionalCompositionTest；rA/rB 复跑将继续认领同类）。
- **其余新语义簇：待 rA/rB 复跑日志聚类**（见 §5 交接）。

## 3. 复跑口径更正（重要）

主会话 138 复跑若为 dev 档（e2e 分发器缺省不传 `--release`），则 debug-profile-too-slow 族
（≥5 例）与部分超时例在 release 口径下会转过——本轮复跑统一 `--release`（任务书配方），
FractionReduction 已实证转过。分类与验收以 release 为准；dev 档差异在报告中单列。

## 4. 修复与验收状态（截至本报告落笔）

| 项 | 状态 |
|---|---|
| b1013-fix 并入（E0599 批级回归） | ✅ 已并入 j25-sem（abdd523b），双远端核对 |
| escape 互递归环判定（304edfb1） | ✅ 代码落分支；generator 本机 cargo check（--release --tests，heavy_lock）过；**JDK25 代表例 FunctionalCompositionTest 与 JDK21 回归（FunctionalCompositionTest/TestChmTransfer/TestHoistShadow，参考构建 21）验收作业 `j25s-fix1` 在跑**（2 命令，7 服务器池） |
| 138 例现状重建 | 🔄 `j25s-rA`（组 1–7）/`j25s-rB`（组 8–14）复跑中（127 例，宽超时 + keep-artifacts + 流式回显 + 周期打包）；已出：FractionReduction PASS、TestArraysSetAll run-timeout、FunctionalCompositionTest 栈溢出（已修） |

## 5. 交接（下一棒从这里继续）

1. 盯 `/data/workspace/rava-wt/j25-sem/cluster_results/job/{j25s-rA,j25s-rB,j25s-fix2}/summary.json`
   （勿裸 pgrep）。rA/rB 完成后：解包各 `NN/j25s2-g*-logs.tgz`，用逐例 run.log/build.log 聚类
   （聚合脚本雏形 `/tmp/j25s_collect.py`），把 §2.2 的「预期已修」坐实、新簇落 §2.3。
2. OOM 类现名单 = rA/rB summary 的 `oom` 条目 + §2.1 已列名字，交主会话大槽批量重跑。
3. j25s-fix2 两作业 rc=0 即代表例验收完成；若 FunctionalCompositionTest 仍败，读
   `build/j25sfix-a.out`（服务器检出 `/data/rava-spot-job-j25s-fix2`）。
4. 派发器均为 setsid 脱离进程，本子代理结束后继续运行；如需停止：`pkill -f "job j25s-rA"` 等。
5. 未竟事项：junit 8 例 OOM（G1 线）、HttpLoopback native 缺口、resource-bundle-by-name、
   StackFrameInfo 反射流（G1 线）均维持 open 登记，不在本分支修复。

---

# 第二轮（j25-r2，2026-10-13）

分支 `j25-r2`（基 batch-1014 @ 1b644fbc，含 j25-sem escape 修复与 lib-tests 全部内容），
修复提交落在 `j25-r2-fix`（e05dcd0a + 979b759a）。作业 tag 前缀 `j25r-`。

## 0. 口径

- **输入**：2026-10-13 主会话扫尾重跑（release + 22G 槽，基线 e44f78d6「生成器审查余项收口」）
  ——1127 例总盘子 994 过 / 133 败。133 = junit 族 OOM 8（闭包规模，G1 线，不属本轮）+ **语义类 125（本轮对象）**。
- **重要口径修正**：主会话复跑基线 e44f78d6 **不含 batch-1013 合批内容**（adf4cc1f 起：coerce-check、
  plainseg-bisect、invoke-key-opt、b1013-fix）也不含 batch-1014（lib-tests、j25-sem）。即 125 例中的一部分
  在 batch-1014 上应已消解；TestCoerceBoxCast / TestMhBoxCast 在 e44f78d6 上是幽灵失败（测试文件不存在，
  日志为 "No test files found"，coerce-check 才入库这两例）。
- **expected 口径**：expected 全部由参考构建 21.0.11 生成；被测为 JDK 25 转译体。凡两版 JDK 自身行为
  差异属版本差异口径（§3.4），不算转译缺陷。
- 复跑配方：`--release --jdk 25 --transpile-timeout 2400 --run-timeout 600 --build-timeout-scale 2`，
  服务器 kr2/sg2/us1/jp1（dev 批闸门避让），tag `j25r-run1`（14 作业 × 125 例）。

## 1. 输入 125 例本地聚类（error_logs 全量挖掘）

| 簇 | 例数 | 阶段 | 签名 |
|---|---|---|---|
| E0308 symbol_lookup | 65 | compile | `java/lang/foreign/symbol_lookup.rs:190:71` expected `Result<Optional<MemorySegment>>` found `Result<Optional<Object>>` |
| Unsafe.getX 解析失败（MH 族） | ~25 | run | InternalError: NoSuchMethodException `jdk.internal.misc.Unsafe.get{Int,Double,Reference}(Object,long)…/invokeVirtual`，栈 DirectMethodHandle.makePreparedFieldLambdaForm → MethodHandleNatives.resolve |
| 版本差异（输出不一致） | 4 | output | TestParseDoubleEdge（hex 浮点错误消息）/ TestNativeFrameTrace（sleepNanos 链）/ TestObjectWaitFrames（wait 行号）/ TestSimpleDateFormatRound（时区短名） |
| 运行超时 | 5 | run timeout | AmicablePairs、RamanujanPrimes、TestArraysSetAll、TestCommonPool、TestParallelArrayCas（均 >300s） |
| folds 违约 | 2 | transpile | 活指令 pc=NNN invokeinterface 顺序落入 dead_pcs（TestCollectionsFactories@122,300、TestListOf@635） |
| condy switch 标签 | 1 | transpile | switch 标签形态 Dynamic(17,"I","Ljava/lang/Class;")（TestPatternSlotReuse） |
| 幽灵（文件不存在） | 2 | — | TestCoerceBoxCast、TestMhBoxCast（coerce-check 入库后为真实新例，首次跑 JDK25） |
| getClassContext 多一帧 | 2 | output | TestClassContextDispatch、TestSecurityManagerContext 行首多 `java.lang.SecurityManager` |
| Console 编码 NPE | 1 | run | CheckOutputDeviceIsATerminal：Console.<clinit> → Charset.forName(null) IAE |
| native 缺失 | 3 | run | Class.getClassFileVersion0（TestClassNestNatives）、UnixFileDispatcherImpl.available0（TestFilesApi、FileIODemo） |
| stub（链外方法） | 1 | run | TestModuleLayerDefine：java/util/AbstractCollection.spliterator 存根 |
| VarHandle ISE | 3 | run | TestVolatilePrimitiveAccess / TestCallerSensitiveLookup / TestSecurityProperties：VarHandle.setVolatile/getAndAdd/compareAndSet 无消息 IllegalStateException |
| 其余单例 run error | ~15 | run | 逐例见 §5 清单（TestEcSignVerify 等安全族多数预期属 MH 族——首错即 Unsafe.getX） |

（例数为日志签名口径；MH 族实际认领以 j25r-run1 复跑为准——凡经 Field/VarHandle/MH 的用例首访即挂同一错。）

## 2. 复跑两组（j25r-run1 @ 1b644fbc，服务器拥挤排队中——完成后回填）

- **基线升级已消解**（预期至少：escape 修复的 7 例栈溢度族 FunctionalCompositionTest/RealNumberSet/
  TestFunctionalInterface/TestHttpLoopback×2/TestPrimitiveSpinedBuffer/TestStreamMore；两幽灵例转为真实新例）
- **仍失败**：（回填）

## 3. 逐簇根因与修复（j25-r2-fix）

### 3.1 SAM 合成对象 vtable 条目类型分歧（E0308，65 例）— 已修 e05dcd0a

- 根因：`sam_objects.rs` 的 SAM 条目签名取接口发射擦除声明（`erased_declaration`——具体参数化类型按
  Signature 恢复），闭包 fn 类型取 SAM 描述符擦除视图（`SamSpec::erased_sig`）。JDK25
  `java.lang.foreign.SymbolLookup.find` 返回 `Optional<MemorySegment>`：描述符只到裸 `Optional`（擦除视图
  `Optional<Object>`），两视图在具体参数化类型位置分歧，条目体 `(self.0)(arg0)` 直呼即 E0308。JDK21 档案
  无此形态（SAM 返回类型为泛型实例化的接口），JDK25 首触。该文件在 java_base_decl——一切含 JDK 档案的
  构建全挂，故 65 例一簇。
- 修法：条目签名与擦除签名逐位置（参数 + 返回）比对，分歧位置经 Object 边界桥接（装箱再按目标类型
  `From<Object>` 擦除重建——只搬句柄，与宏 `result="checkcast"` 同口径）。无分歧时输出零变化。

### 3.2 手写边界类方法表随反射事实裁剪（MH 族 ~25 例）— 已修 979b759a

- 根因（证据链）：JDK25 `DirectMethodHandle.makePreparedFieldLambdaForm` 改经
  `unsafeMethodName(isGetter,isVolatile,wrapper)` 辅助取 Unsafe 访问器名（switch-map 数组 + StringBuilder，
  JDK21 为 `LambdaForm$Kind.methodName` 枚举常量字段读）→ 分析器推不出按名反射事实 → `Unsafe` 不入
  `meta_method_classes` → 档案侧 `Keep` 裁剪（`rava_meta_tables::render` retain）把方法表行清零 → 运行期
  `MethodHandleNatives.resolve` → `__declared_method_meta` 落空 → NoSuchMethodError。行本身在壳文件里
  一直在（本机 slot11 JDK21 残留构建实证 `unsafe_.rs` 的 `// [meta] #[java_native(name="getDouble"…)]` 与
  meta_tables 行；JDK21 缓存 closure.json 的 reflect.members 含全 getX 族）。
- 修法：`write_archive` 扫描壳文本的 `#[impl_methods]` 标记（手写边界类，76 类），其类名并进 Keep 的
  methods 集——手写层成员由人维护、MemberName 可在运行期以计算名触达是这类类的固有形态。
  JDK21 侧对照（slot11 残留构建 closure.json）：反射重的用例事实集本含 Unsafe/Class/Object，但
  Console/TimeZone/MethodHandleNatives 等不在——这些类此前 0 行（按名解析必失败、枚举为空，是裁剪伪影），
  修复后按真实声明应答，方向严格贴近 JVM 语义；二进制代价 76 类方法行（千行级）。

### 3.3 folds 活调用死顺序（TestCollectionsFactories / TestListOf）— 已修 e05dcd0a

- 根因：`Set.of().add("x")` 恒抛 UnsupportedOperationException——调用点活、顺序后继（pop/goto）全克隆
  不可达。report.rs 已有「按不返回导出」缓解但门控在 `is_concrete`（须有具体求值克隆）；本形态知识来自
  派发面定论（无具体克隆），违约原样导出 → 发射层 apply_fold 拒绝。语义论证：常量折叠的分支指令本身
  可达，造不出「调用活而顺序后继死」——该形态必为不返回调用，门控可撤。
- 修法：Operand::Method 形态的 folds 违约一律转 noreturn_calls（撤 is_concrete 门控）。

### 3.4 typeSwitch 基本类型 Class 标签（TestPatternSlotReuse）— 已修 e05dcd0a

- 根因：对象 switch 的基本类型无条件模式（`case int` 等）以 `ConstantBootstraps.primitiveClass` condy
  （`Dynamic(bsm,"I","Ljava/lang/Class;")`）装载类字面量；`labels()` 未识别该形态（JDK21+ javac 同形态，
  语料首触）。typeSwitch 语义：基本类型标签匹配对应包装类实例。
- 修法：bsm 为 primitiveClass 且描述符 `Ljava/lang/Class;` 的 Dynamic 标签按名（基本类型描述符）经
  `wrapper_of` 归一为装箱类 `Label::Class`（与 Class 标签同一出口）。

### 3.5 标准流编码属性 null（CheckOutputDeviceIsATerminal）— 已修 e05dcd0a

- 根因：`system_props_raw_impl::tty_encoding` 只在流为终端时给值；e2e 管道下 `stdout.encoding` null。
  JDK25 `Console.<clinit>` 读 stdin/stdout.encoding 作 `Charset.forName` 实参 → IAE "Null charset name" →
  ExceptionInInitializerError。本机实测真机 JDK21/25 管道下 `stdout.encoding=UTF-8`、JDK25
  `stdin.encoding=UTF-8`（JEP 400 缺省字符集兜底）。
- 修法：非终端流按 "UTF-8"（终端仍本地编码），JDK21/25 与真机一致。

### 3.6 两缺口 native — 已修 979b759a

- `Class.getClassFileVersion0()`（JDK24+ API）：= `jdk_feature() + 44`（语料类同源同版本；TestClassNestNatives
  断言状态路径触达）。
- `UnixFileDispatcherImpl.available0(FileDescriptor)`（FileChannelImpl.available 后端）：字符设备/FIFO/socket
  取 FIONREAD、其余 fstat size − lseek(cur)，虚拟句柄走 jdk_resources（TestFilesApi、FileIODemo）。

### 3.7 未修——根因已写清

- **版本差异 4 例**（TestParseDoubleEdge/TestNativeFrameTrace/TestObjectWaitFrames/TestSimpleDateFormatRound）：
  本机真机 JDK21/25 双跑实证——转译体输出与真机 JDK25 **逐字一致**，差异全部来自两版 JDK 自身行为
  （FloatingDecimal 错误分类、Thread.sleep→sleepNanos 链、Object.java 行号、时区短名策略）。属 expected
  口径（21.0.11 生成）问题，归主会话裁决：refjdk 25 重生成 expected 或按 JDK 版本分档。登记
  `jdk25-version-output-diff`（open）。
- **getClassContext 多一帧 2 例**（TestClassContextDispatch/TestSecurityManagerContext）：JDK25 该方法改
  StackWalker 实现（`walk(s -> s.map(declaringClass).skip(1)…)`，真机流头恰为 SecurityManager.getClassContext
  被 skip(1) 吃掉）；我们的帧枚举头部多一帧（疑 walk 回调帧 lambda$getClassContext$0——声明类同为
  SecurityManager——或 walker 机制帧过滤口径差一）。JDK21 该方法是手写 native（过滤自洽）故不显现。需
  运行期帧列表取证（j25r-run1 产物或 gdb）；候选修法：`__anchor_stream` 过滤口径对齐 HotSpot（从 walk
  调用者帧起取）。登记 `jdk25-getclasscontext-extra-frame`（open）。
- **VarHandle 无消息 ISE 3 例**（TestVolatilePrimitiveAccess/TestCallerSensitiveLookup/TestSecurityProperties）：
  `_field_coords` → `_field_offset`（`__unsafe_long_cell("fieldOffset")` 按名协议）落空 → `_state_err()`。
  JDK21/25 的 Field 族 flavor 类逐字段对比完全一致（`final long fieldOffset` 都在），疑点在运行期 flavor
  实例的按名字段面（构造路径 JDK25 改 `makeFieldHandle(MemberName,Class,boolean)` 3 参 / findVarHandle 走
  getFieldVarHandle）——需 j25r-run1 复跑产物进一步定位（与 §3.2 修复可能相关：EventHelper 静态 VH 的
  compareAndSet 栈即此簇）。
- **运行超时 5 例**：JDK21 侧同例已过（lc3-fix 后）；JDK25 挂起机制未取证（复跑 600s 档 + 产物后再定，
  候选：ForkJoin/并行流等待面在 JDK25 新形态）。本轮未修。
- **TestModuleLayerDefine（AbstractCollection.spliterator 存根）**：链外方法存根，JDK25 闭包形状差一轮
  取证；第一轮报告的 module-unnamed-handwritten（e50d4ba3）不覆盖本形态。未修。

## 4. 验收（截至本报告收笔 2026-10-13 07:30，作业部分仍在跑）

| 项 | 状态 | 证据 |
|---|---|---|
| JDK21 回归 7 例 | ✅ **6/7 PASS**（j25r-fixv2/02 @ 856297e7，kr2） | HelloWorld PASS；**FunctionalCompositionTest PASS**（escape 修复例叠 SAM 桥接不回归）；TestPatternSlotReuse PASS（condy 标签修复）；TestGetFields PASS（16m59 构建，Keep 修复下反射面不回归）；TestCollectionsFactories PASS（folds 修复）；TestReflectInvokePerReceiver PASS |
| JDK21 唯一失败 | ⚠️ CheckOutputDeviceIsATerminal `E0599: new_str not found for IllegalArgumentException`（编译期）——归属待定：纯净基线探针（rr-compile-…@1b644fbc）在跑，若纯基线同败则为既有缝（手写层 IAE(String) 构造器请求缺位，同 method-trait-handwritten-seam 一类），非本轮回归 | fixv2/02 第 4 例；修不修归下一棒按探针结果 |
| JDK25 代表例 13 例 | 🔄 j25r-fixv/01 + j25r-fixv2/01 排队/在跑（服务器拥挤） | 先行信号：j25r-fixv/02（979b759a，含 S_ISSOCK 笔误）上 TestPatternSlotReuse / TestCollectionsFactories 已 PASS——condy 与 folds 修复在 JDK21 侧实证有效 |
| S_ISSOCK 笔误（中间态回归） | ✅ 已修（856297e7） | j25r-fixv/02 上 4 例 E0425（libc 无 S_ISSOCK 函数，java_runtime 手写文件按闭包裁剪编译，含 UnixFileDispatcherImpl 的用例全挂）→ 改掩码比较后 fixv2/02 同 4 例全 PASS |
| generator cargo check（--release --tests，heavy_lock） | ✅ 全部修复提交过 | e05dcd0a / 979b759a / 856297e7 |
| java_runtime | 服务器构建覆盖 | 本机无 closure_input 不可检；多例端到端构建实编（fixv/fixv2） |

### 4.1 已知中间过程事件

- j25r-fixv/01（JDK25 13 例代表）与 j25r-fixv2/01 在 979b759a/856297e7 上排队（kr2/sg2/us1/jp1 长期被其他线作业占满，本轮 17:40—次日 06:00 间累计空转约 3 小时）；两作业完成后由下一棒按 §6 流程回收。
- j25r-run1 复跑（纯净 1b644fbc）首例 DataEncryptionStandard 转译错误 34m52s（11G 作业槽，与第一轮「DES 转译 OOM」同族，资源类非语义类）。

## 5. j25r-run1 两组清单（14 作业 @ 1b644fbc，截至收笔仅零星起跑——未完）

复跑被服务器拥挤严重拖延（其余线作业长期占满 kr2/sg2/us1/jp1）。收笔时状态：作业 01（11 例组）
在 jp1 起跑（首例 DataEncryptionStandard 转译错误 34m52s，资源类），其余 13 作业排队/在跑。
**两组清单的终值由下一棒回收**（流程见 §6）；本地聚类（§1）+ 第一轮经验给预期：
已消解至少 = escape 修复 7 例（栈溢度族）+ folds 2 例中的可析部分 + 2 幽灵例转真实新例；
仍失败预期主体 = E0308 65 例（纯净基线上必仍在——修复在 j25-r2-fix）+ MH 族 + 超时族 + 版本差异族。

## 6. 交接（下一棒从这里继续）

1. **回收复跑与验收作业**（盯 summary.json，勿裸 pgrep）：
   `cluster_results/job/{j25r-run1,j25r-fixv,j25r-fixv2}/summary.json`；产物 `j25r2-g*-logs.tgz` /
   `j25rf2-*-logs.tgz` 解包后用 `/tmp/j25r_collect.py`（已备）聚合成两组清单，回填报告 §2/§5。
   服务器检出按 tag 自动清理（checkouts.json）。
2. **分支状态**：`j25-r2`（远端）= 1b644fbc 纯净基线（勿在 run1 完成前推——作业检出追分支头）；
   本地 j25-r2 = j25-r2-fix = **856297e7**（双远端已推核对）= 全部修复 + 文档。run1 完成后把远端
   j25-r2 快进到 856297e7（或合批直接采用 j25-r2-fix）。
3. **CheckOutputDeviceIsATerminal 的 E0599 归属**：探针 rr-compile-…@1b644fbc（纯净）与
   @856297e7（修复）各一在跑；同败 = 既有手写层/构造器请求缝（登记 failure_patterns 的话归
   method-trait-handwritten-seam 邻类），只败修复 = 本轮回归（查 Keep/folds 改动对闭包的间接影响）。
   该例 JDK25 侧的流编码修复验收也被此编译错挡住——先定归属再修。
4. **未修簇**（§3.7）：版本差异 4 例（等主会话裁 expected 口径）、getClassContext 多帧 2 例、
   VarHandle ISE 3 例（用 run1 产物取 flavor 实例的按名字段面证据）、超时 5 例（run1 600s 档 +
   gdb 挂点）、AbstractCollection.spliterator 存根 1 例。
5. **junit 8 例 OOM** 归 G1 线不变；DES/DeepCopy 类 11G 槽转译 OOM 在 run1 将再现，归大槽重跑。

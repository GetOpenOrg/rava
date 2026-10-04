# 任务历史归档（2026-09-16 起）

> 接续 `docs/tasks-history.md`（T01–T81，2026-09-16 冻结）。本文件收纳 `docs/tasks.md` 中已完成关闭的条目，
> 按归档批次追加，原表格行原样保留（含提交号与证据）。当前开放项见 `docs/tasks.md`。

---

## 2026-09-26 归档批次

### 历史基线记录（原 tasks.md 头部）

**基线（2026-09-22 凌晨，用户 Ubuntu 全量 @ ~afa3890 树，双进程与定向批并发）**：**113 PASS / 53 FAIL**（较初基线 87 **+26**）——compile 13 / run 33 / output 6 / transpile 1，run 族自动分类：stub-hit 16、runtime-panic 9、s8-crash 1、unclassified 7。**失败清单双进程合并实战通过**（定向批写 22 条保留 32 → 全量合并 53，无丢失）。**`--failed` 回归已实证线程层三例转绿出列（a705fd9 树：TestSynchronized/ThreadJoin/WaitNotify PASS，清单 53→50）+ `--skip-failed` 干净面 113/113 全绿——有效基线 **152/169**（151 + TestBigInteger[参数重绑定+getLong 补链]）。**Ubuntu 2026-09-23 凌晨双轮实证（树 548c08b，pull 到 cdc6422 前的最后旧基线）**：--failed 37 例 2 出列（NestedTry/Suppressed=macros 修实证）→35；--skip-failed 干净面 **133/133 全绿**（含 FieldDemo/ReflectProbe/CollectionFactory/ListOf/LambdaVar/StringEdge 等全部近期修复用例）——**该树全量口径 135/168**。清单剩余 35 与 main 最新（cdc6422）已合未验差≈compile14 八例+S-8→下轮 pull 后清单预期 35→26**。棘轮全循环验证：全量播种 → --failed 回归出列 → --skip-failed 零失败快速面。TestCollectionsUtil 层进（getDeclaredField 随线程层落地，下一卡点 Unsafe.objectFieldOffset）；新见 native：Reflection.getCallerClass（TestAtomics 下一层）。**2026-09-22 13:48-16:12 `--failed` 轮（树=f4636d0，时间戳对照定位）**：7 出列对账——5 stubs 绿 + TestCustomException（S-19 #5）+ **TestLambdaVar 意外转绿（本地 e4abf67 实证归因：栈帧轮顺带修复**，其 ImmutableCollections AME 链经 throwable 构造路径变化消除**）**；清单 48→41。**reflect 已合入（08c60c3）：下轮 pull 后 TestCollectionFactory 出列→40**。新形态记录：TestSequencedCollections 推进至 output（NullableKeyValueHolder 类型名 toString=已归类桥接项）；TestAtomics→ExceptionInInitializerError；新 stub：RandomSupport$AbstractSplittableGenerator.<init>（TestRandomSeed）。TestRefKindsFull PASS→FAIL 复现定性：`stub: Integer.valueOf`（与 TestOptional 同族，装箱旁路 from_any 被对象化收编后揭开，非行为回归）。此前 compile 族多项推进到下一层（HashSetOps→UOE remove、GenericBoundsCombo→collection.rs panic、DateTimeFormat/ZonedDateTime→E0308）。
**基线（2026-09-23 服务器 `--failed` 轮 @ b0f5460 + 主会话三修）**：21 例跑 6 出列（CollectorsMore/Pecs/ArrayBounds/AutoboxEdge/Atomics/SequencedCollections——近期合并全部兑现）→ 保留 15。**第二轮 @ 85f0ac0**：再出列 2（StreamNumeric=K-6b、BigInteger=param-rebind）→ 保留 13（三修未到服务器，UnnamedVar/EnumSetMap/StringEdge 复现修复前行为）。**`1075f11` 推送后（三修 + output3 两件 + merge）下轮 `--failed` 预期 13→8**：StringCompare(ArrayCovariance 同批)/UnnamedVar/EnumSetMap/StringEdge 五例出列。**`cf4b8c2`（unify3 两修）再推送后预期 13→6**——StringNewMethods/StreamMore 同批出列。**`1a5f0eb`（record-stub + runner 小修）后预期 6→5**（RecordPattern 出列）。G-3 三例（DateTimeFormat/ZonedDateTime/FilesApi）合入后再 →2，cf-anewarray 成功后 →**1（仅 VirtualThread）**。剩余队列：G-3 release 验证批（主会话接管，静态验收过）+ CompletableFuture（cf-anewarray 在途）+ VirtualThread（ContinuationSupport stub，深浅未知）。

**基线（2026-09-21）**：
- **166 全量（用户 Ubuntu，JDK21）**：87 PASS / 79 FAIL；**跑批树落后 main，混有陈旧污染**——fcb04ce 干净树复核 18 例，TestStringBuilder / TestOptionalFull / TestIncDec / TestShortCircuit 已 PASS（假象），真实失败面待复验收敛。归类全记录：`2026-09-21-e2e-baseline-classification.md`（新立项 A-8 / S-19 / S-20；S-8 / S-9 实测升级）。
- 红线 19/19 全绿（a20a31f 实测）；TestStringBuilder 编译 0 错误、输出 22 行与 Java 逐字一致（fcb04ce 复核 PASS，含金丝雀）。
- **streams 三测全绿（2026-09-21 午后，P-3 合入后主会话 --clean 6/6）**：TestStreamBasic / TestStreamAdvanced / TestStreamCollectors + TestStringBuilderOps 全 PASS，输出逐字一致。
- 架构里程碑链：vtable 双指针多态 → CFG 支配树结构化（含 try/catch `java_try!`、`<clinit>` 语义、异常对象）→ println 字节码化 → K-6 槽位签名模型 → 擦除运行时身份阶段 1（非泛型 `I__VTable` + `__interface`）→ S-18 接口分派 BFS（fcb04ce）→ P-3 边界补全（7b64afa，join + DoubleToDecimal）。
- 教训：**全量跑批必须 tee 落盘**（本轮 224 分钟 stdout-only，38 个 run 族无 stderr 只能二次定向）。


### 原 20 项清单

| # | 任务 | 状态 | 证据 / 下一步 |
|---|---|---|---|
| 3 | M3 反射 L3（注解元数据 + Method.invoke 分派） | ✅ | `d3e02dc` + 本分支 M3 链（`82bd0d7` 等），`GOLDEN OK (m3)` |
| 4 | 抽象槽位需求登记缺口 | ✅ | `f358124` |
| 5 | TypeIR 余 10 处 | ✅ | `2f5d3c3`，type_surgery 10→0；扩大口径余 22 另记（见新增 N4） |
| 6 | R-2 Deref（`__into_super` 25 处） | ✅ 关闭 / R-2′ 第一步 ✅ | `4d0b4c3`；第二步见新增 N3 |
| 8 | P-1 JDK 类名字面量清零 | ✅ `a330f78`（**重写门槛③达成**） | 库知识表迁 runtime/java_runtime/ 清单（carrier_type_positions / overload_abbrev / boundary_prefixes / signature_erased_interfaces，经 codegen/runtime_manifest 读取；迁移前后逐项相等）；JLS/JVMS 语言层类（Object/String/Class/包装类/Record/Annotation/Cloneable 与 main/toString/equals 描述符）集中 constants.py 作唯一引用点。新增审计 `[raw-audit] jdk_literals`（AST 字符串常量中的完整类名 / 嵌入描述符 / 二级包前缀，constants.py 豁免）**= 0**。23 例生成树逐字节一致。口径说明：原则 4 按「禁止按类名枚举的库知识」执行，语言层类集中而非删除；如需连 Object/String 也清零另议 |
| 11 | JUnit M3 | ✅ | 同 #3，`GOLDEN OK (m3)` |
| 12 | JUnit M4（timeout/join 边界） | ✅ **GOLDEN OK (m4)**（2026-09-25） | `tests/lib_pilot/JunitTimeoutMain.java` + `lib_pilot_golden.sh m4`：@Test(timeout=) 经 FailOnTimeout（ThreadGroup + 模拟线程 + CountDownLatch + FutureTask.get 限时）逐字一致。补链：Unsafe 原子族 9 个（getAndBitwiseAnd/OrInt、getAndSetInt/Reference、weakCAS Int/Reference、putIntOpaque/Release、storeStoreFence）。边界：无抢占 → 真实超时不可达，已入 compatibility.md |
| 13 | JUnit M5（workspace 打包 + 跨 crate 分派链） | ✅ **过渡形态 GOLDEN OK (m5)**（2026-09-25）；终态挂 R9 | 探针 `tests/lib_pilot/JunitCrossCrateMain.java` + `lib_pilot_golden.sh m5`：user 类继承/实现 lib 类型（BaseMatcher 子类、TypeSafeMatcher 模板方法、匿名 Matcher）被 lib 回调 + Runner→用户测试(@Before/@Test)→hamcrest→用户 Matcher 三 crate 往返，16 行逐字一致。唯一缺口 = lib 可见性映射 protected→pub(crate) 致下游子类调父类 protected `<init>` E0624，改为 protected→pub（JLS §6.6.2）。workspace 一键构建（java_runtime+hamcrest+junit4+user）M1–M5 共用。**终态**（java-runtime-core 版本化 + 可 publish）= 条件③，挂重写 R9 |
| 14 | 线程模型终态（VT/Continuation） | ✅ **方案 A 已实现**（`4821738` + 本提交） | **口径更正**：原描述「虚拟线程映射 OS 线程」在 Rc 对象模型（非 Send/Sync）下不可行——落地为**虚拟线程 = 模拟平台线程**：与平台线程共用 `thread_impl.rs` 的单线程协作调度器（READY 队列 + join/wait/sleep 泵），Continuation 不建模（VirtualThread.start/run/joinNanos 手写直驱 `runWith(task)`）。真并发仍属对象模型 Send/Sync 化之后的远期档位。TestVirtualThread **JDK21 PASS（`4821738`）/ JDK25 PASS（本提交：MhUtil.findVarHandle、VerifyAccess.ensureTypeVisible、ReferencedKeySet.create 2 参、Wrapper.forPrimitiveType 四处 JDK25 改名/改签适配）**。方案 B / C 不采纳 |
| 17 | JDK25 putDecimal 入口（ASB.append 链） | ✅ `197dc82` | 用户全量确认触达（TestStringBuilderOps）。DoubleToDecimal/FloatToDecimal 的 LATIN1/UTF16 单例 + putDecimal；新增 e2e TestAppendDecimal（Latin1/UTF16 × double/float × 常规/整数/科学计数/特殊值/极值 + StringBuffer/insert/valueOf）JDK25 PASS |
| 19 | equiv 探针四件（identityHashCode/finalize/引用类型/clone） | ✅ 完成（2026-09-25） | TestIdentityHash / TestFinalizeProbe / TestReferenceTypes / TestObjectClone JDK21 全 PASS，分档入账 compatibility.md；探针揭出的 clone 覆盖体绕过已由 C-1 修复 |
| 20 | 缺席直接接口宽化（原记「11 个」） | ✅ | `ce79244`（实测 20 个，已全部物化） |

### 本会话新增 / 遗留

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| N1 | Object 无参构造在反射里不可见 | ✅ 2026-09-25 | 根因是 `getConstructors` 错误地沿父类链上溯（构造器不继承，JLS §8.8）——修正为只取本类后，Object 补行不再影响其他类枚举。build.rs 给手写 Object 补 `<init>()V` public 行；reflect_dispatch 加 Object 构造臂；新增手写 `getConstructor(Class...)`（原经字节码落 native 存根）。验收：新增 e2e `TestCtorReflect`（构造器不继承 / 非 public 可见性 / 带参 newInstance / 抽象类 / 接口 / Object public+declared 面 / 未命中 NSME，19 行）PASS；反射回归 TestReflectProbe/TestAnnoReflect/TestAnnotations + JUnit m3 GOLDEN OK |
| N3 | R-2′ 第二步：上转 IR 化（方案 C `UpcastExpr` 节点） | ✅ 2026-09-25 | rs_ir 新增 `UpcastExpr(expr, wrap)`，render 分派到 `upcast_expr` 唯一形态决策点；IR 管线 4 处（stack 两个上转分支、vars 合并点 / 降级对齐）改直接构造节点，字符串管线 4 处随 L5 迁移。**形态决策**：保留 `.into()`、不做 UFCS 统一——`animal = dog.into()` 对 Java 开发者可读性优于 `<Animal as From<_>>::from(dog)`（CLAUDE.md 可读层目标），方案 C 的结构化收益不依赖形态。验收：27 例生成树**逐字节一致**，raw_expr 净降 698、无测试上升。顺带修复长期失效单测 test_cfg_structuring（81/81） |
| N5 | invoke_virtual `this` 路径子类登记与第 4 项重复 | ✅ 2026-09-25 | 调用侧 this 路径删除，定义侧单一来源。**揭出并修复定义侧缺口**：JDK 链抽象槽位 + 本类桥（SpinedBuffer.OfInt/OfLong/OfDouble 的 arrayForEach/arrayLength/arrayForOne）此前只靠调用侧兜住，改为定义侧照登记。剩余生成树差异仅「最近声明者 == 槽位 trait」与接口 default 两类（行为等价）。**顺带修复**用户链叶子继承祖先桥时丢 vtable_name/vtable_erasure（E0407）。验收：新增 e2e `TestPrimitiveSpinedBuffer`（int/long/double × sorted/builder/toArray/iterator）+ `TestInheritedSlots`（用户层次四形态 + JDK 继承槽位面）PASS；**反证**：去掉修复后 TestPrimitiveSpinedBuffer 命中 `stub: SpinedBuffer$OfPrimitive.arrayLength`；定向回归 12/12 PASS |
| S-67 | synchronized 块内的模式 switch：CFG「try 区域与控制流不成嵌套结构」→ 方法体落存根 | ✅ 完成（`7e4f690`） | 仅经出口到达的 handler 子树（外层 try 之外）补齐缺失的 ctx 组。TestSyncPatternSwitch、TestPatternSlotReuse PASS |
| N10 | Python 3.11 兼容 | ✅ 本提交 | `project_writer.py` 一处 f-string 内同种引号嵌套（3.12+ 语法）在 3.11 下 SyntaxError，改为字符串拼接；全仓 `ast.parse` 扫描仅此一处 |
| L-1 | **Locale 数据改由 CLDR 资源束字节码翻译供给** | ✅ 完成（2026-09-25，`docs/plans/2026-09-25-l1-cldr-locale-data.md` L1-a..d） | **L1-a** 纯数据资源束结构判定（载体清单 `data_bundle_carriers.txt` + 操作码白名单）+ `_is_boundary_class` 惰性放行；**L1-b** locale 种子（`locale_seeds.txt`：常量字段沿 `<clinit>` 溯源到字面量——JDK21/25 常量表下标顺序不同而种子集一致；factory / tag 字面量；`--locales`；父链）；**L1-c** trigger（数据消费入口触达才入选束，不消费本地化数据的程序零代价）+ main 登记束注册表（`data_bundles.rs`）+ `LocaleResources` 手写边界（候选链实例化翻译束 + setParent，`getNumberStrings` 按字节码语义）+ DFS / NumberFormatProvider 走翻译字节码，**删除手写 en/de 两表**；**L1-d** 数组初始化器折叠（`JArray::from(vec![..])`），全字符串字面量表紧凑形态 `JArray::from_strs(&[..])`（TestDateTimeFormat 编译峰值 >13.6G OOM → 10.5G）。**验收**：TestLocaleConstants（fr_FR / it_IT 与 JVM 逐字一致）、TestFormatLocale、TestStringFormat、TestDateTimeFormat 4/4 PASS；单测 test_data_bundle / test_locale_seed / test_fold_array_literals 共 50 例。**偏差**：货币（Currency / CurrencyNames 束）不在范围——`initializeCurrency` 按地区表给代码与符号，`getCurrency()` 为 null（compatibility.md） |
| C-1 | ~~类自带 clone 覆盖体被绕过~~（#19 探针揭出） | **✅ 完成（`5aaf9da`，已授权改宏）** | 宏为 `__inner` 生成 `__shallow_copy`（按运行时类逐字段新建存储单元 + 新 `__identity`），wrapper `__shallow_copy` 先委托 inner；调用点仅在链上无 clone 声明时走浅拷贝特判，否则虚分派（`_chain_declares`）。修复了 `44323f9` 回退时的 `super.clone()` CloneNotSupported 问题（vtable 体内 `this` 是 `__inner`）。**验收**：JDK21 TestObjectClone（`deep.independent` / `deep.viaBase` 等全部行）、TestArrayCopy、TestArrayBounds、TestArrayList、TestCollections、TestCollectionsUtil、TestHashMapOps 7/7 PASS；JDK25 MemberName.clone 链由 TestCompletableFuture 复验（排队中） |
| N9 | 仓库清理：`stash@{0}` 与 /tmp/wt-* 残留 | ✅ 服务器侧 stash 已删；用户本机 /tmp/wt-* 由用户清理（`git worktree prune` 后删目录） | worktree 均为已合入分支 |

### 活跃任务（原当前队列）

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| ~~陈旧树筛 + run 族定向复验~~ | **✅ 完成（2026-09-21 午后）** | 30 例复验 + 26 run 族 stderr 定性 + 4 output 族复验全记录：归类文档 §4.2/§五。假失败第 6 例（TestMethodRef）；数组视图族升 6 例；CDS/isBigEndian native 双件 8 例 |
| ~~A-8 同文件辅助类未进闭包~~ | **✅ 完成（`1be8caa`）** | 15/15 E0433/E0425 清零，4 例全过（FieldShadow/EnumAdvanced/InnerClass/InstanceOfChain），闭包指纹 20/20 一致；连带修复 P-3 潜伏回归 Appendable E0432（`0b22604`，四红线干净树恢复全绿） |
| ~~A-8 下一层（11 例 compile 清零后暴露，已归类）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestBridgeMethod PASS（本轮回归多次）；中文双重编码已随 S-19 关闭 |
| ~~S-20 `Object.wait/notify/notifyAll`~~ | **✅ 完成（`23fe881`）** | `monitor.rs` 双条件队列监视器 + monitorenter/同步方法/同步块四发射面真实化；12/12 编译清零、TestStringSearch 全绿、红线 23/23 含 streams。**未接 InternalLock（论证见 monitor.rs），S-11 终态随线程模型** |
| ~~native 双件：CDS + isBigEndian~~ | **✅ 完成（`9873095`）** | CDS 恒 0（HotSpot 语义）+ isBigEndian `cfg!` + Thread.registerNatives no-op；6+2 用例编译/运行推进，TestStringSearch 全绿。**下一层已归类**：线程层总闸 3 例、ImmutableCollections 载体分派 3 例（A-4 邻域）、SharedSecrets.getJavaUtilCollectionAccess 2 例（P-3 邻域）、Charset clinit、toString 装箱分派 1 行 |
| ~~TypeIR 最小层~~ | **✅ 完成（`e488ec5`+`92e3db5`）** | `jvm_type.py` 六变体代数（erasure/substitute/is_subtype_of + registry 闭包缓存）+ 35 单测；试点 hierarchy._is_subtype 委托——204 万对对拍零分歧、双种子零 diff、五审计线一致。**下一批接入点已排**：control.py(7)/stack.py(5)/returns.py(5)/invoke_sig.py(5)/fields.py(3)/blocks.py(2)/arrays.py(2)——A-5 合入后可续（invoke_sig 在其域） |
| ~~K-6b 双侧一致~~ | **✅ 完成（`73dbc57`+`8c1f50d`，主会话复核）** | 单一来源=发射签名+ACC_BRIDGE 槽位同一性见证，四消费方全派生（定义侧签名/定义侧体 Safe 直挂参数还原/调用侧名字经 interface_member_local_name/调用侧返回位实例化代入+bridge 见证逐子类放行）。**TestStreamNumeric 全绿**（基线 10 编译错）；TestStringNewMethods 编译过推进 run（残余=Nodes.builder 合并局部 CCE 归 unify_pair 域——**与 ice-fix 修的是同函数不同分支，下一增量**）。曾试桥接槽位直挂被 E0201 证伪整体回退（inherited_gen 桥成员已占槽），改登记放行路线。K-6 教训红线守住：只认 bridge 见证不做形态猜测 |
| ~~TypeIR 批次 2：六消费点迁移~~ | **✅ 完成（`c440b73`，7 提交）** | control(12 含 A-4 债务2)/stack(6+共享擦除查询入口)/returns(10)/fields(5)/blocks(3)/arrays(6)——**type_surgery 69→27**；行为零变化四重验证；+10 单测。余量 27 大头=invoke_sig 5+invoke_virtual 4（批次 3，排 compile 族后） |
| ~~TypeIR 余项：type_surgery 9 处清零（清单第 5 项）~~ | **✅ 完成（`e0e74eb` 单测 + `2f5d3c3`，分支 `claude/jolly-dijkstra-diftum`；调研 `docs/reports/2026-09-24-typeir-remaining-survey.md`）** | **`type_surgery_sites` 10→0**：原 10 中 1 处为 raw_audit 自身 docstring 误报（扫描豁免本模块），真实 9 处——coerce×2 / vars×3（含 G-3 承重 _forms_alignable）/ hierarchy / invoke / member_owner / render——统一经批次 2 擦除边界（erased_base / erased_class_of / is_jvm_array）与语法拆分器 split_rust_type_args。选 erased_base 族而非 from_rust_type：对 `&X` / `()` 等非标识符形态逐点等价（from_rust_type 剥 `&` 是调研点名的分歧面）。**验收**：33 例（A/B/C/D 四批回归集并集）生成树修改前后**逐字节一致**、raw_expr/raw_stmt 逐测试相同、fallback none；新增逐形对拍单测 `tests/unit/test_typeir_sites.py`（58/58 含既有 TypeIR 单测）；双种子 diff 0。**扩大口径 `type_surgery_ext` 基线 22**（maxsplit 头部切分 / partition / find|index('<') / 无尖括号 JArray 前缀 / Vec|Rc 前缀；解析层与擦除边界白名单豁免）= L1 下一轮余量，`[raw-audit]` 与 run_tests `[raw]` 汇总同步输出。TypeIR 完全体剩余能力缺口 G1–G5（RsType→JvmType 桥、作用域类型变量、宿主基本类型、带实参回渲染[归 M-3]、双查询面合一）见调研 §三 |
| ~~A-4 批次 3-6：类型位置载体化~~ | **✅ 主战役收官（批次 6=`0526bb4`，第二任零修复纯验证收官）** | CharSequence/Appendable + decimal 双侧对齐 + iface_carrier_views 接口位；记分牌 TSB 1010→913（−100=CharSequence 残余兑现）；主会话 5/5 定向复核全绿；Spliterator 沿暂缓（证据 §9：阻塞面在特化桥接非类型位置）。**累计 Into<I> −49%** |
| ~~A-4 批次 3-5：类型位置载体化~~ | **✅ 完成（四任接力，`2bb2255` 合入）** | **`Into<I>` 32931→17106（−48%）**：批次 3 Iterator 穿透（carrier_type 单一决策点）→ 批次 4 集合族 → 批次 5 函数式族 39 接口 + 载体 instanceof 运行时化（抓修 streams 红线破口）。遗留：CharSequence/Appendable 手写层阻塞（证据 §8）、Spliterator 沿、TSB 残余 1010 分布、type_surgery +7 债务（TypeIR 批次 2 消化）。主会话修 stubs×a4b3 语义冲突（impl 签名对齐） |
| ~~A-4 批次 6：CharSequence/Appendable 载体化~~ | **✅ 完成（`fix/a4b6-charsequence`，前任 WIP+本任验证收官）** | §8 遗留 1 双侧落地：铺设两接口 + sig_parse 早返回（越过 `_CLASSNAME_MAP` 旧擦除）+ `#[iface_carrier_views]` 接口视图臂（JLS 4.10.3 数组协变/checkcast 的接口位）+ decimal 三文件 appendTo 载体签名。九例定向全绿（全量待服务器）；TSB 1010→913 / TSC 1009→912 / TPM 975→878；bfs-audit、双种子、from_any 持平。Spliterator 沿评估维持暂缓（证据 §9） |
| ~~A-5 lambda 对象化~~ | **✅ 完成（`4fa0f6d`+`54e2026`+`50eede0`）** | `<Iface>__Lambda` 合成对象（证据驱动 27 接口+共置伴生）+ `__interface` 应答 + SAM/default 单一路径；**downcast_ref 70→0、from_any −60.5%**；主会话 7 测试复核吻合（6/7，唯一失败=子串带入基线项）。**from_any 残余 1588 全归 A-4——A-4 已解锁（A-5 合入）** |
| ~~线程层（S-20 下一层总闸，3 例）~~ | **✅ 完成（`8eca47b`）** | 单线程协作调度（start0 就绪队列 + join/wait/sleep 嵌套泵 + monitor ticket 等待集联动）；currentThread 字段按 golden 反推；**三例全绿**（主会话 4/4 复核含 TestExceptions）。条件等价边界六条已入 compatibility.md。遗留：TestVirtualThread 卡 ContinuationSupport（下一层）；getDeclaredField not-found 分支无字段元数据表 |
| ~~数组视图 coerce 族~~ | **✅ 完成（`fix/array-view-coerce` 4 提交）** | 根因两层：anewarray 数组类组件拼出非法描述符 `L[I;`（类型保真缺口）+ `try_cast` 数组分支探针退化恒真落 panic。修法：`try_array_view` 唯一决策点合一（可捕获 Err）、实参/返回位发射（areturn 曾静默 `Default::default()` 错值）、`Object.clone` 真浅拷贝、`System.arraycopy` 规范化。**3 例全绿**（ArrayCopy/BigDecimal/DurationPeriod）+ ArrayCovariance CCE 消除可捕获 + BigInteger compile 清零；JDK25 判定=部分同超族（载体返回位已补），硬阻塞为语料适配。闭包指纹逐字一致，红线 21/23（2 失败=基线既有同型同点） |
| ~~数组 getClass 命名（1 例）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestArrayCovariance PASS（S-5 数组 getClass 动态分派 `1075f11` 兑现） |
| ~~runtime 散点三件包~~ | **✅ 完成（`9b58672`+`928328f`）** | **3 例全绿**（TestOptional/TestRefKindsFull/TestRandomSeed，主会话 5/6 复核含回归）：Integer.valueOf（[-128,127] 缓存保身份 + toString 顺带收层）+ RandomSupport 族（upcalls 接口级回调边让 BFS 自动翻译 SplittableRandom 实现类——零手写零枚举）。**第 3 件改判归 macros 域**（见下） |
| ~~异常 upcast 中间型 catch 缺口~~ | **✅ 完成（`548c08b`，主会话直做）** | inner `__erased_vtable` 运行时类覆盖 + wrapper 委托；**TestNestedTry/TestSuppressed 双绿**，13 测试回归全过。踩坑：`Rc::clone` 泛型推断方向（inner 侧显式 `as` 上转） |
| ~~S-8 负数组崩溃~~ | **✅ 件 1 合入（`030b7ab`，另一台机器代理交付，主会话复核确认）** | 三创建指令经 `JArray::try_new/try_new_with` 抛真实 NegativeArraySizeException（Err 可捕获）；不可失败形态饱和为空数组作手写层崩溃防线；Arrays.copyOf 同接入。**TestArrayBounds s8-crash 转绿**（我首次复跑遇陈旧 scratch 假失败，fresh 后 PASS——个中差异已核实为生成物未刷新）。等效审计口径同步更新（neg-array 转为创建点总量观测） |
| ~~ImmutableCollections 分派族（ice-dispatch 调查）~~ | **✅ 调查报告交付（零改码，另一台机器代理）** | 7 例根因表：①default 注入扫父类链（_emit_interface_default_inheritance 只查本类——10-15 行，**建议最先做**，HashSetOps/MapIteration 2 例）②unify_pair 灭真值（TreeMap_KeySet 三元真臂→Default::default()，TreeMapSet 1 例）③~~ListIterator 未进语料闭包（AutoboxEdge/LinkedHash 2 例）~~ **已由 `c23a4b0` 顺带修复，零 codegen 改动收官（fix/ice-listiterator-closure，2026-09-23）**——compile14 的 Pecs 修复（BFS 接口闭包 stub 通道）即同链 ListItr→ListIterator→Iterator，根因表交付时未与合并序对账（调查基线树早于 c23a4b0 进 main）；main @ a73a7e3 fresh scratch `--jdk 21` 双例全绿，生成物实证：ListItr 的 all_supertypes 含 Iterator、`impl Iterator for ImmutableCollections_ListItr` 在位（c23a4b0~1 同口径再生成则断链复现：闭包止于 ListIterator、无该 impl）；bfs-audit 三计数前后一致（sig-poly-native=0/root-inherited=152/unresolved=0），闭包 +3 接口存根（ListIterator/RandomAccess/SequencedCollection——接口进闭包合法变化）；双种子（PYTHONHASHSEED=1/2）生成树 diff 归零；回归 TestArrayList/TestIterator/TestLinkedList/TestPecs/TestOptional 5/5 全绿；本域残留（同类缺口、非失败路径、未铺设载体）：TestAutoboxEdge 闭包内 11 个缺席直接接口（Readable/AnnotatedElement/SortedSet/Lock 等）均不在 CARRIER_TYPE_POSITIONS，无 itable 分派路径，如需收口走 c23a4b0 stub 通道宽化另立项 ④string Display null 守卫（HashMapOps 1 例）⑤GenericBoundsCombo 本机不复现需对账。行号纠正：iterator.rs:12/17 是宏 span 非真 bug 位点 |
| ~~反射族 Method 元数据表（membername）~~ | **✅ 完成（`d8f17b8`+`376a6cd`+`6564a04`，主会话 5/5 复核）** | L1 方法表（build.rs，(name,descriptor) 键+exceptions 列）+ L2 `getDeclaredMethod/getDeclaredMethods` 转正 + MethodHandleNatives.resolve 内核 + 12 层伴生放行——**TestAtomics FAIL→4/7 行**（ai/ar/al/ab 四行正确）；**MethodDemo 转正**（语料 168→169）。**下一层归类**：`Unsafe.compareAndSetInt` 经 AQS 基类视图无共享 int 单元——宏域 `__unsafe_int_cell` 臂委托缺口（与 `__erased_vtable` 同型的 inner/wrapper 不对称，A-4 域，修法≈wrapper 臂失败后委托 self.vtable 或 inner 补臂） |
| ~~compile 14 族攻坚~~ | **✅ 8/14 全绿（10 提交，主会话 10/10 复核含红线）** | 全绿：RecordAdvanced(E0369)/SwitchNull(S-17 完成：String/Integer 常量标签判定桥)/InterfacePrivate(Java9+ 私有方法落载体)/MethodRefKinds(绑定接收者 coerce)/BridgeMethod(**桥方法体落槽统一机制**+toString 恒虚)/PriorityQueue(接口闭包二段解析 JLS 5.4.3.3)/CollectorsMore(return this 擦除重建)/Pecs(BFS 接口闭包 stub 通道)；StreamNumeric 10错→2。**遗留 6 例归类**：DateTimeFormat/ZonedDateTime/FilesApi=**G-3 三重复用槽**（try 暂存+监视对象+catch 形参同槽，需窗口 3）；StreamNumeric×2/StringNewMethods×4=**K-6b 双侧一致**（宏 __impl_ 体改写）；CompletableFuture=anewarray `_` 落 arrays.py（另一机 scatter4 域，诊断已移交） |
| ~~缺席直接接口宽化（清单第 20 项）+ 抽象槽位需求登记缺口（清单第 4 项）~~ | **✅ 完成（第 4 项 `f358124` + 第 20 项 `ce79244`，分支 `claude/jolly-dijkstra-diftum`；调研 `docs/reports/2026-09-24-demand-registration-survey.md`）** | 同类不同源（需求已入账、消费端未闭合），同分支两提交。**第 4 项**：typed 调用入链键为槽位声明类（FileSystemProvider.isSameFile），定义侧填槽门控只按实现者/本类键查 → 中间祖先实现 + 叶子继承的槽位落 abstract stub。修 = `class_writer._slot_demanded_on_chain`（整条超类链任一层 (m,参数描述符) 在调用链即放行；K-6b 过滤以被索要的声明层为起点，实现者 final 覆盖合法）；删止血手写 `file_system_provider_impl.rs`（**TestFilesApi 删后仍全绿 = 见证**）；FileSystems 的 vm_boundary 条目保留、理由改写为策略截断。生成树：TestFilesApi 仅 +3 转发成员（LinuxFileSystemProvider.isSameFile、LinuxFileSystem.getPath/provider）删 0 行，TestAutoboxEdge 零变化，闭包零扰动。回归 17/17（两轮）+ 双种子 diff 0。**第 20 项**：c23a4b0 stub 通道的接口登记只写账本，物化循环消费入口快照 → 循环内 / 父类补全 / 迟至补扫三处登记的接口从未物化。修 = 父类补全补登记 + 三通道沉降后按账本（sorted）只物化接口本体（不回灌 stub 通道，不重开 4b776b6 类型闭环）；新增类恒 ⊆ 账本且全部 is_interface。闭包：TestAutoboxEdge 398→422（+24 全接口，原记「11 个」口径过时——实测缺席 20 个直接接口 + 超接口闭包；Deque 在 CARRIER_TYPE_POSITIONS 内仍缺席，原「不在载体门控」归因不准），TestFilesApi 484→505（+21 全接口）；调用链类数与 bfs-audit 不变。回归 24/24（含大闭包 StreamBasic/TreeMapSet/PrintStreamApi/Atomics/CompletableFuture/LambdaVar 与小闭包抽样）+ m1/m2/m3 GOLDEN OK + 双种子 diff 0，编译面零新增错误。**后续清理项**：invoke_virtual `this` 路径子类登记与定义侧修复重复（暂留，K-6b 口径在彼验证过）；手写 `_impl.rs` 构造的公开包对象不进 RTA（实现体未入链时槽位落实现者 stub）另评估；A-4 终态（CARRIER_TYPE_POSITIONS 置 None）的前置已就位 |
| ~~codegen 参数重绑定第三分支（TestBigInteger，~10-15 行）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestBigInteger PASS |
| ~~反射族 Field 元数据表~~ | **✅ 完成（`18f1fa2`+`2803715`+`417c315`，主会话 5/5 复核）** | build.rs FIELD_TABLE + getDeclaredField not-found 抛 NoSuchFieldException（可捕获）+ getComponentType/Array.newArray/Field.get/set 最小实现；**TestCollectionFactory 转绿**；探针转正 FieldDemo/TestReflectProbe（**语料 166→168**，golden 以 java 实跑生成）；MemberName 方法表评估入档（`2026-09-22-method-metadata-table-eval.md`：方法属性 4 空格对齐坑、L3 分派协议须与 A-4 合流） |
| ~~ice 修复①+②（default 父类链 + unify_pair）~~ | **✅ 完成（`33a63e9`+`15565bf`，主会话 5/5 复核）** | **3 例全绿**（HashSetOps/MapIteration/TreeMapSet）：default 注入覆盖判定沿父类链累积（+21 行，_anc_ifaces 防线不动）+ unify_pair 不灭真值（具体臂经 Object 边界重建，86bf63a 同型；null 臂合法路径未误伤）。**顺带恢复 CompletableFuture.postComplete 三元语义**（基线隐性灭 null，因编译未过从未暴露）。调查表修复③仍在队（ListIterator 断链→另一机）；④见下 |
| ~~ice 修复④（string Display null 守卫）~~ | **✅ 完成（`c70ddc2`，wt 代理交付，基线 cab5a63）** | Display impl 加 vtable `is_jvm_null()` 守卫（S-3.1 统一入口，+8 行）：null 载体输出字面 `"null"`（JLS §5.1.11/§15.18.1），不再对 Repr::Null 数组 `to_vec` panic（array.rs 批量读取 NPE 链）；与 Object 侧 null 呈现（`() vtable __obj_str`）同语义。**TestHashMapOps 全绿**；StringBuilder/StringOps/StringFormat/ArrayList/Collections 零回归（7/7 含 filter 顺带 StringBuilderOps/CollectionsUtil） |
| ~~TestBigInteger 运行期 NPE 再定性（ice ④伴调查，零改码）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestBigInteger PASS |
| ~~unsafe-cell 委托 + record toString `.0`~~ | **✅ 完成（`4916401`+`574a5ff`，主会话 6/6 复核含 TestRecordPattern 基线项区分）** | **TestAtomics 7/7 全绿**（inner cell 臂运行时类应答+wrapper 委托，与 __erased_vtable 同型第三例——inner/wrapper 不对称缺口家族就此三修三验）+ **TestSealed 全绿**（record toString 浮点分量走 java_fmt_f64/f32）。TestRecordPattern=既有 describe 转译 stub 非回归（归类：模式匹配用户类方法 stub，G-10 邻域待查） |
| ~~toString 桥接（scatter4 件 2，另一台机器）~~ | **✅ 完成（`e955e7d` 合入，主会话 3/3 复核）** | 根因精化：摘除机制只针对 inherent 形态手写，项目已有 `__impl_toString` 第二形态不触发摘除（Double/Integer 先例）——修法只补伴生手写体，**codegen/宏零改动**；**TestSequencedCollections 全绿**（mfirst=p=1/mlast=q=2）。附带情报：TestArrayCovariance 6 行 diff=既有（JArray getClass 协变视图+getSimpleName 数组形态，归数组 getClass 族） |
| ~~stub-hit 散点大礼包（约 10 处）~~ | **✅ 完成（`fix/stub-scatter-pack` 7 提交，+1704）** | **5 例全绿**（CollectionsUtil/LocalDate/FormatLocale/HexFormat/PrintStreamApi，主会话 7/7 复核含回归）；Unsafe 对象布局+实例字段原子族（ObjectVTable `__unsafe_long/int_cell` 钩子）、getCallerClass、getAdapter（德语数据）、US_ASCII 族、FloatToDecimal（**顺带修两处 Schubfach 移植 bug**：int 回绕缺失/float 平局括号，千例对拍零差异）。**下一层归类**：TestBigInteger=codegen 参数重绑定（ifnull 分支 astore 死局部）；TestCollectionFactory=`Class.getComponentType`（反射域）；TestAtomics=MemberName 元数据表（reflect 域）；TestSequencedCollections=手写 toString 桥接（emitter `_root_method_vtable_owner`） |
| ~~native 散点双件：availableProcessors + mismatch int 版~~ | **✅ 完成（`fix/native-scatter-2`）** | **TestArraysUtil 全绿**；TestLocalDate 卡点解除（NCPU 判定：仅作批次切分阈值不进输出，无需归一）。mismatch 语义核正：ArraysSupport 层恒 -1（「较小剩余长度」是公开 API Arrays.mismatch 行为，由翻译字节码承担） |
| ~~Unsafe 对象布局族散点（TestLocalDate 下一层）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestLocalDate PASS（arrayBaseOffset 等已在 `unsafe__impl.rs`）；**JDK25 侧 `Unsafe.isBigEndian` 另见开放项总表 #16** |
| ~~native 双件：`CDS.getRandomSeedForDumping` + `StringUTF16.isBigEndian`~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | JDK21：CDS 六例（ListOf/LinkedHash/CollectionFactory/StreamMore/AutoboxEdge/LambdaVar）全 PASS，`cds_impl.rs`/`string_utf16_impl.rs` 已落地；**JDK25 侧 `Unsafe.isBigEndian` 另见 #16** |
| ~~S-19 输出一致性缺陷群~~ | **✅ 全六件收官** | rint/NaN/expm1（`0078906`+`73a6c54`+`444ad9a`）+ UTF-16/科学计数（`16296bf`+`2c27f14`）+ 栈帧（`2e35b9e`，std::backtrace 帧数真实内容近似）；TestMathRound/TestNaN/TestFloatBits/TestStringCodePoints/TestMathExact/TestCustomException 全绿 |
| ~~downcast 链移除（859 处）~~ | **✅ 完成（2026-09-21，fix/downcast-chain-removal）** | 方案 §6 步骤 4：根方法根 vtable 直调 + 类虚方法 `__virtual_view` 部件重建分派（宏新增）+ SAM 回退保留；`downcast_ref` 方法体清零（TestStringBuilder 1104→6，余全 SAM）；红线/streams/金丝雀 18/18，7 例失败经基线对照确认既有；指纹不变、双种子归零（链顺序不确定性源消失）；`from_any` 残余归 A-4/A-5（清单见分支报告） |
| ~~服务器 15 例主会话三修（2026-09-23）~~ | **✅ 完成（`57b0dac`+`2741344`+`aca9acc`）** | ①TestUnnamedVar：javac `--enable-preview --release N`（N=当前 javac 版本）——预览语法编译门，transpile 错误清零；②TestEnumSetMap 两层：反射按名路径（getEnumConstantsShared/Enum.valueOf）强制类初始化钩子表（lib.rs `ensure_class_initialized`，生成侧 main 按枚举形态类登记）+ build.rs 元数据四表扫描域扩至 `../user/src`（Class.getSuperclass 对用户类断链修复）——反射回归 4/4（ArrayList/CollectionFactory/ReflectProbe/Atomics）；③TestStringEdge E0432=376a6cd 回归（var_handle_impl 硬 import 三 flavor 语料条件类）——companion 声明加 `_IMPL_FILE_DEPS` 依赖闭包门槛，不齐时 impl 整体不参与编译；Atomics 回归零影响 |
| ~~output3 三件包（另一台机器，`fix/output-intern-classname`）~~ | **✅ 合入（`755ad7e`+`0fcbb7f`，merge `1075f11`，主会话 5/5 复核）** | 件 1 S-6 intern 驻留表（`755ad7e`）：thread_local 按 UTF-16 code units 内容键，`From<&str>`（ldc）同表、`from_owned`（拼接）不入表保 `lit==heap=false`——TestStringCompare 全绿；件 2 S-5 数组 getClass（`0fcbb7f`）：根因比任务书浅=invoke_virtual.py:116 早路径拦截喂静态描述符（未走到 vtable）——动态分派 `Object::from(recv).0.getClass()` + JArray Repr::Covariant 委托 + getSimpleName 按维度剥 `[`——TestArrayCovariance 全绿；K-6b 同文件改动零冲突（`_jarray_type_desc` 删除零残留引用、K-6b 符号存活）。**合流点：TestStringEdge 首次全绿**（E0432 门 aca9acc + internSame 755ad7e）。件 3 InterfaceConflict **核销**：分支基 a73a7e3 早于 ice①（33a63e9 父类链），当前 main 实测 PASS，诊断陈旧 |
| ~~unify3：unify_pair 第三增量 + ldc 常量割尾换行~~ | **✅ 合入（`6273242`+`ea1c4e1`，merge `cf4b8c2`，代理 OOM 静默终止后主会话接管收官，10/10 验证）** | ①具体类型臂与擦除 Object 臂汇合不伪造 checkcast：合并点按擦除 Object 落定+具体臂 `_coerce_to_object` 上转（保 vtable 动态分派）——异构兄弟类（Nodes.builder 的 FixedNodeBuilder vs SpinedNodeBuilder）CCE 消除，**TestStreamMore/TestStringNewMethods 双双全绿**；②consts.py `.rstrip('\n')` 早期 javap 残留剥常量真实尾换行——`"\n"` 常量（joining delimiter）曾发射成空串、文本块尾行换行丢失——stripIndent 换行逐字恢复。代理死亡时段回归清单的 TestBigInteger FAIL 定性为 OOM 假信号（worktree 复跑 PASS）。教训：**代理回归批与主会话验证批双大闭包 cargo 并发=OOM 杀代理**，主会话发起验证批前先看代理是否在跑批 |
| ~~record pattern 访问器卫兵 try 结构化（record-stub 代理）~~ | **✅ 合入（`325d7da`，merge `8086bf2`，主会话交集复核 5/5）** | 根因换轴：非调用链（用户类本就全量翻译）——gen_method_body 抛 CfgError 走异常回退 panic 存根：javac 21 对 record pattern 解构访问器逐个发 3 指令卫兵 try 区间共享同一 MatchException 处理器，`_install_try_nodes` 只为首区间装节点→「词法组==控制流组」不变量炸。修法=`_split_disjoint_try_ranges`（每区间补装合成 try 节点+处理器私有子图克隆挂载，语义与异常表逐区间一致）+ `parent_try` 改「同词法层支配链最深者」。**TestRecordPattern 全绿**（cfg-audit stub_fallback 3→0）；代理自验 25 例全绿（含首版曾打破后修正的 TestTryInLoop/TestMultiCatch 且与基线逐字节一致）；双种子归零、闭包指纹一致。顺带修 run_tests `--failed-file` 外部路径 relative_to 崩溃（`1a5f0eb`） |
| ~~G-3 三重复用槽（WIP 接管收官）~~ | **✅ 合入（`7f94f65`+`4c395ea`+`0699952`，merge `e4dae52`）** | `_forms_alignable` 分型框架（可对齐并入提升绑定/不可对齐独立绑定）——**ZonedDateTime/FilesApi 的三重槽 E0308 根除**（cargo check 零错误+release 编译过）；VM.isModuleSystemInited 恒真（FilesApi 推进一层）；DateTimeFormat 判定=**不同根因**（泛型集合实参 coerce，另立项）；前任"已出列"系中断前中间态。run 层验收：ArrayList/OptionalFull（在列项定论=已修）/TestSuppressed（中断项）PASS；DateTimeFormat 双种子归零 |
| ~~anewarray 组件类型 E0283（cf-anewarray 代理）~~ | **✅ 合入（3 提交，merge `cc3c8e7`；代理自验回归 10/10、双种子归零、审计持平）** | 根因=sim/arrays.py 对泛型组件把 Object 替换成 `_`（0e6ff4f 手法）——varargs 数组只经 Object 边界流转无推断锚。修法=**anewarray 发射即终态**（删 `_` 分支，直接发射 `C<Object>`/载体接口形态；异实参存储由 aastore 既有 Object 边界检查兜住）+ CF 运行时补链四件（SharedThreadContainer/FJPAccess/getIntOpaque/AtomicInteger.\<init\>）+ availableProcessors=1（单线程协作档位，等价 -XX:ActiveProcessorCount=1）。**TestCompletableFuture 编译过、run 推进 5 卡点后停在 `native: VarHandle.set`** |
| ~~G-3 下一层三件包（next3+unify4 两棒接力）~~ | **✅ 合入（next3 `6858233` + unify4 `c8e131d`；回归 10/10、双种子归零、审计成比例解释）** | 三族编译缺口根除：①unify_pair 第五增量（`_common_ref_type` 含 `<` 返回 None → 泛型 widening 分支）+②`_store_local` 绑定点改 LVT 区间语义判定（两字节 astore 覆盖）+③invokespecial 基类调用实参重建（turbofish 实例化）。**doPrivileged 回调边激活**（接口级 upcalls + callchain 手写体回调链补全 + System.initPhase1 props 空表）。**清单 4 例全推进到架构决策层**：FilesApi→`sun/nio/fs` POSIX 原生族（系统性手写域）、ZonedDateTime→tzdb.dat 数据供给（TzdbZoneRulesProvider 要 `$JAVA_HOME/lib/tzdb.dat`，嵌入/随行是架构决策，与 locale 数据链同族）、VT→vm_boundary.txt 策略清单（解除后构造器完整翻译但暴露 executors 族 3 编译错：DelayedWorkQueue E0407/E0782+ForkJoinTask E0425）、DateTimeFormat→CalendarDataUtility locale 链。**四例的下一层全部是「数据文件供给/原生族手写/策略决策」级，非 bug 修复——进入路线图决策域** |
| ~~unify4：blocks 两族+激活+VT 构造器~~ | **✅ 已并入上行（5 提交 `e83c45f`..`4b776b6`）** | 详见上；callchain 补全为 next3 禁改域的最小例外（单独提交 `4b776b6`，接口级翻转丢失枚举形态递归回调链的必要补全，TestHashMapOps 实证 valueOf 存根后修） |
| ~~TestVirtualThread 终态定性（2026-09-24 试探裁定）~~ | **✅ 关闭（2026-09-25）** | 线程模型方案 A 落地后 JDK21/JDK25 双 PASS（见总表 #14）。原记卡点 VirtualThread.<init> / ContinuationScope 均由手写 VirtualThread 伴生绕开（Continuation 不建模） |
| ~~TestDateTimeFormat：locale 数据（locale-embed 代理）~~ | **✅ 合入（`20c5252`，merge `5171851`；冲刺清单收官）** 形态证伪=ResourceBundle 类非数据文件（84 束类反射加载永不进闭包）→ 首边界 CalendarDataUtility 手写 297 行（en 全表 6 style+114 区域周参数+fw/rg，金本位与数据同源）；javatime/classic 双口径忠实；非 en 语言 ROOT 回落按需补表。回归 4/4 双种子 0 审计恒等体积 +0.06% |
| **冲刺收官：JDK21 预期 168/169** | 2026-09-24 | 清单 8 例全部处理（CF/FilesApi/ZonedDateTime/DateTimeFormat 修复 + StringNewMethods/StreamMore/RecordPattern 出列 + VT 挂线程模型决策）。**服务器收官轮可跑**：pull 5171851 后 `--jdk 21` 全量（新版本化清单首次播种 failed_tests_jdk21.txt） |
| ~~JDK25 编译墙双错（jdk25-wall 代理）~~ | **✅ 合入（`19373bb`+`ff291e0`，merge `f75598d`；JDK25 HelloWorld 端到端 PASS+JDK21 回归 4/4+双种子 0+审计恒等）** 代理修正主会话两处初判：E0407 真改名 uncheckedNewStringNoRepl（_nf_covered 实为空集——伴生是裸 fn 扫不出）→ 伴生 trait impl 签名提取+接口恒发声明+模型缺席方法补发；E0308=arrayBaseOffset I→J → `core_` 伴生核心约定。**JDK25 全量可重跑** |
| 原 E0407 条目（已修） | JDK25 全量首炮 | `error[E0407]: method 'newStringNoRepl' is not a member of trait 'JavaLangAccess__VTable'`——**非接口方法集漂移**（javap 双侧确认两版接口均有该方法），真因**修正（本地复现定位）**：JDK25 对该族**改名演化**（newStringNoRepl→newStringUTF8NoRepl），新名有调用边进生成 trait、旧名没有；手写伴生实现旧名 → trait 缺成员。另有第二错 E0308（CHM.set_ABASE i32→i64，JDK25 的 ABASE 字段类型演化）。**修法定案**：class_writer:293 的 `_nf_covered` 命中对接口场景仍发 abstract 声明（trait 恒含伴生方法集）+ ABASE widening 按证据修。**JDK25 全量暂停等此修**（169×20 分钟全撞同一墙）；math 双件（已修）在此墙之后才可见 |
| ~~在途派发批次（2026-09-23 深夜 ~ 09-24，8 行）~~ | **✅ 全部收官（2026-09-24 核对：逐项有合入提交，无残留在途）** | ①typeir-b3 → `8c8ba84`（TypeIR 批次 3，169 生成树 diff=0）；②executors-fix → `d3c02e9`（VT 解锁三错）；③posix-survey → 报告 `2026-09-23-posix-survey.md`，实施 `d8c431c`（档 A）+`0f46fb8`（Linux 侧）；④jdk25-wall → `f75598d`（编译墙双错）；⑤vars-hoist（v1/v2/v3 三棒）→ `6da2a14`（CHM transfer 吞节点根因）；⑥fallback-narrow → `466f513`（兜底收窄 + [fallback-audit]）；⑦tzdb-embed → `b0964cc`（TestZonedDateTime 全绿）；⑧reflect-l3 → `d3e02dc`；⑨atomics-regression → `7495d3b`（真凶 _is_jnull 误用）；⑩jnull 批量修 → `4ccd3ff`（缺陷家族清零）。其下棒 JUnit M3 已收官（见 JUnit M3 行）。/tmp/wt-* worktree 均为已合入分支的残留，可删 |
| ~~JDK25 第三失配：BFS 静态边时序丢失（2026-09-24 实测）~~ **✅ 已修（`f30998d`：非迟到窗口，而是 boundary 静态边被 _propagate_virtual_targets 确定性丢弃；迟至静态边补扫三道门，JDK25 HelloWorld PASS；下一层 hashCodeOfUTF16 并入 JDK25 边界 stubs 遗留）** | HelloWorld --jdk 25 复现 | 编译墙修复实证生效（E0407/E0308 消失，推进 run 层）；新卡点 `stub: ArraysSupport.hashCodeOfUnsigned:([BIII)I`。**证据链**：生成 string_latin1.rs:211 的调用边存在（发射侧正常）、RAVA_DEBUG 零 fallback 记录（=in_call_chain false 链外 stub，非翻译回退）→ **调用侧发射与 BFS 方法键入队不一致**（迟到的静态调用边未补扫——next3「迟至实现者」的静态边变体；ArraysSupport 经别的通道进闭包后，其方法键未被 StringLatin1.hashCode 的边补上）。排队 locale 之后，修法方向=callchain 边时序对齐（发射侧见到的方法引用与入队集合同源） |
| **悬案结案：streams 族服务器-only compile error = OOM（2026-09-24 实证）** | 用户贴完整输出 | `rustc ... (signal: 9, SIGKILL: kill)` 无任何 error[E]——**内存不足被 OOM Killer 杀**，非代码缺陷（本地同树 PASS=mac 内存充裕）。服务器 1500+ 类大 crate 编译卡在内存阈值附近（Zoned 同量级过/StreamAdvanced 挂）；main.py 直接跑走 debug+incremental 更吃内存。**缓解：CARGO_INCREMENTAL=0 + 全量走 run_tests（release）+ WSL2 加 swap（.wslconfig memory/swap）**。25m/29m 慢=/mnt/d 慢 IO 叠加。main.py 的 `[codegen] 继承成员重名未声明` 日志=信息性（覆盖判定常规决策），可选降噪 |
| **五项决策定案（2026-09-23 深夜，用户拍板按建议执行）** | — | ①**数据文件供给=编译期嵌入**（tzdb.dat/locale 数据 include 进 runtime，零依赖单二进制，golden 可比性最强）——解锁 ZonedDateTime（tzdb 先行）与 DateTimeFormat（locale 后）；②**libc 暂不破**（posix 档 B 缓做，档 A 纯 std 足够）；③VT 的 vm_boundary 策略等 executors-fix 结果再定；④**下一波排期修订**：E0407 修复（JDK25 通杀墙，域与 executors-fix 相邻故等其收工）→ **tzdb 嵌入**（ZonedDateTime，挟两例收益最高）→ FilesApi 档 A（posix-survey §6 切入序）→ locale 嵌入（DateTimeFormat）→ M3 反射 L3；⑤服务器 JDK21 全量等三代理合入后一次跑 |
| ~~FilesApi 解锁（posix 档 A，posix-tier-a 代理）~~ | **✅ 合入（3 提交，merge `d8c431c`；TestFilesApi 收官全绿）** 手写 ~2345 行按 7 步切入序；BFS 前沿推进链含报告外坑（getBytesNoRepl/getModule/FileChannel clinit upcall/invoke.py makeConcat Z 参[JLS 5.1.11 布尔拼接]/implref 稳定别名[K-4 必要件——冲突改名类跨闭包恒定路径；2026-10-02 类在定义处恒用定义名后删除]）；FileSystems 边界收编（bin 反降 143M，削反射子系统 ~1000 类）；Linux 侧 cfg 双形态**待服务器轮验证**。**新立项线索**：①typed 调用对抽象槽位（静态类型接收者）的继承成员需求登记缺口（FileSystems 根因，codegen demand 登记扩展——**✅ 清单第 4 项 `f358124` 已根治**）②档 B 落差注释在案（R/W/X access/jnu 编码/socket 通道/UTF_8$Decoder 机器） |
| 原 FilesApi 调查条目（已兑现） | 65 native 全集盘点（49 Unix+6 Linux+8 Bsd+1 Mac+1）；**用例面只需 4 native（open0/stat0/unlink0/strerror）+init**；先行依赖 StaticProperty.USER_DIR（10 行）。**档 A**（解锁 TestFilesApi 双平台）≈2300-2700 行/13 文件（含邻接 sun/nio/ch 的 FileChannelImpl/FileDispatcherImpl 700-800）；**档 B**（全集）≈5000-6000 行且需引入 **libc 依赖**（架构决策——runtime 目前零 libc）。7 步切入序见报告 §6，每步 BFS 前沿推进刷新 stub |
| ~~JDK25 L1 math 双件改写（jdk25-l1 代理）~~ | **✅ 合入（`08ddbf7`+`7caefcc`+`ddb1672`，merge `a63df7c`；JDK21 8/8+双种子归零+审计逐字节一致+javap 方法集对账）** | math 双件显式传参改写（15 处字段访问器依赖删除，G 表 md5 核对零动）；decimal_digits **对账补齐**（基线已部分存在——补三缺口 + 修 UTF16 字节序真 bug：BE 误写在 LE 宿主读回 0x3000 乱码）；自审计抓 float toChars2 尾部调用漏写。**JDK25 侧待服务器实测 5 项**：E0599 根除/stringSize 存根消除/putDecimal 入口（ASB.append 链，未手写）/ToDecimal UTF16 通道（JLA 依赖）/BigDecimal getChars 路径。**JDK25 全量轮现在值得跑**（tee 落盘对账调查报告清单） |
| ~~JUnit pilot M1+M2（libpilot 代理）~~ | **✅ 合入（`a7593cd`+`3fa8726`+`5b76a99`，merge `5f00970`）** | **hamcrest/junit4 双 crate golden 逐字一致**（M1：12 检查含失败路径 StringDescription；M2：Assert 全族含 ComparisonFailure `expected:<x> but was:<y>` 消息逐字等价）。两机制：`--lib NAME=JAR[:seed=FQN]` jar 输入（**lib 类走可达性通道**非用户通道——RTA/接口传播/存根补全统一生效+class_cache 落置）+ lib 发射（可见性 access_flags 驱动 + crate_prefix_resolver 跨 crate 定向，依赖方向按声明序）。闭包 1651 类/188MB 达标（≈OptionalChain）；审计级零回归（TestOptionalChain 基线逐字节一致）+双种子归零+回归 6 例；顺带修 6 项既有发射缺陷（StringBuilder 签名别名等，见提交信息）。golden 脚本 `scripts/lib_pilot_golden.sh m1|m2` 可复现。**M3 前置=反射 L3（TypeSafeMatcher 族查 build.rs 元数据表，表不覆盖 lib crate→getSuperclass 死循环）+ reflect.Array/isInstance native**；M5=跨 crate 分派链（lib 接口的用户匿名实现类不可从 lib crate 引用） |
| ~~JUnit pilot M3（清单 3/11 合流：注解元数据+反射 L3 → Runner 路径）~~ | **✅ 收官（分支 `claude/jolly-dijkstra-diftum`，续 `fix/junit-m3-result` 子代理 WIP；未合 main）** | **`lib_pilot_golden.sh m3` GOLDEN OK**——JUnitCore.runClasses 翻译侧与 JVM 逐字一致（SamplePass 2 过 / SampleFail 1 败 + `expected:<[expected-x]> but was:<[actual-y]>`）。**缺口 1 prelude 短名消歧**：决策点 `type_map.configure_short_names`（第二冲突域 `_PRELUDE_CONFLICT_NAMES`，java.lang 本主 String/Object 豁免），触发 = junit 闭包 1 处（org/junit/runner/Result），TestArrayList 闭包 0 处（`[shortname-audit]` 审计线）。**逐层打通的下一层（全部实测定位）**：①classfile Record 属性未 skip 载荷（WIP 引入，record 类解析错位移出闭包，`630c6f3`）②`Reflection.getCallerClass` 帧符号解析依赖 rustc 版本打印形态（1.94 的 `Type<T>::m` / `mod::<impl T>::m` 解析失败→lookup 抛 IllegalCallerException→findVarHandle 类 EIIE；**即 TestAtomics 服务器 FAIL/用户机 PASS 的差异根因**，`e737df1` 三段归一，离线单测 14 形态）③`Result.<clinit>` 的 `ObjectStreamClass.lookup(SerializedForm).getFields()` 序列化元数据链：Field.getLong/getInt、Class.isInterface/descriptorString/getClassLoader/getPackageName/isInstance/getDeclaredConstructor、ReflectionFactory.newConstructorForSerialization、Reference.refersTo0/clear0（`d9f4e2d`/`5dd045c`/`82bd0d7`；build.rs 修饰符表补 interface/annotation 词——接口 getModifiers 缺 INTERFACE 位潜伏缺陷）④void 方法 getReturnType 为 null（class_for_descriptor 缺 V）→ JUnit validatePublicVoid 全败 InvalidTestClassError；isPrimitive 补 void ⑤getDeclaredMethod/getMethod/getDeclaredConstructor 的 null 参数类型数组 ≡ 空数组 ⑥dispatch_gen 静态 void 分派体漏 `?` 静默吞异常。**验收**：回归 15/15（ArrayList/Atomics/EnumSetMap/BridgeMethod/PatternMatch/Annotations/Record×2/RecordPattern/ReflectProbe/MethodDemo/FieldDemo/CompletableFuture/ClassLiteral + TestAnnoReflect 手工对账 7 行一致——该例无 expected 被 run_tests SKIP，待补 expected）；m1/m2 GOLDEN OK；双种子 TestArrayList 生成树 diff 0；`[fallback]` none。**TestAnnotations 随本批转绿**（WIP 注解代理 `__VTable` impl 恒发射）。**已知偏差**：java/lang/Object 手写类无方法表 `<init>` 行→反射取不到无参构造（序列化构造器因此 null，仅反序列化可观测；补合成行会经 getConstructors 父类上溯改变全部类枚举，需反射面整体复核）；序列化构造器只返元数据副本（反序列化实例化语义未建模）。**服务器编译内存**：M3 闭包 java_runtime 单 rustc 峰值 ~14G 超 15G cgroup，需 `CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=line-tables-only`；golden 脚本已相对路径化（`../pilot-deps`、JAVA_HOME 自动发现、stdout UTF-8） |
| ~~VarHandle 引用族访问模式（TestCompletableFuture 下一层，run 层新发现）~~ | **✅ 合入（`85e051d`，merge `01e4be7`；代理自验 CF PASS + Atomics/ArrayList/StringEdge 全绿 + 双种子归零）** | 根因：引用字段存储=擦除载体 `Rc<RefCell<Option<Box<T>>>>` **载体类型异构**（Box<Object>/Box<Completion>），与 int/long 统一 Cell 根本不同。修法四层（纯 runtime/macros，零 codegen）：①宏域新协议 `__unsafe_ref_get/set`（inner/wrapper 家族**第四例**，照 cell 模板）；②Unsafe `get/putReference{,Volatile,Opaque}` 载体驱动；③var_handle_impl 重写——`_field_offset` 泛化为 `__unsafe_long_cell("fieldOffset")` 直读，**三条 flavor import 移除**（语料条件依赖自我消解）；④`Unsafe.park/unpark` 协作泵（cooperative_park，JLS §17.3 虚假唤醒+重查循环）。卡点链：VarHandle.set→putReferenceOpaque→Unsafe.park→PASS。**park/unpark 落地为 VirtualThread 铺路** |

### P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| ~~`monitorenter`/`monitorexit` 为 no-op~~ | **✅ 随 S-20 真实化（`4e6a2a2`）** | 监视器经 `monitor.rs` 侧表真实 acquire/release，单线程语义不变（可重入）；`[equiv-audit] monitor-mt` 已埋点观测。剩余：多线程调度语义随线程层立项 |
| ~~stub-hit 分流：包装类边界层（2 例）~~ | **✅ 随散点三件包（`9b58672`）** | Integer.valueOf [-128,127] 缓存保身份 + toString；TestOptional/TestRefKindsFull 双绿。T-4（包装类走生成）仍是终态方向，缓存实现为过渡 |
| ~~等价告警基建 `[equiv-audit]`~~ | **✅ 完成（ruva 方案 ③ 落地）** | 9 ID 发射点计数 + runner `[equiv]` 汇总 + run 族失败自动分类（`[run-classify]`：stub-hit/native-hit/s8-crash/runtime-panic）+ `--deny equiv[::id]/stub-hit`；生成代码零变化实证；monitor-mt 待 S-20 合入后补埋（一处计数器） |
| e2e 差分补缺（等价探针） | ruva 吸收方案 ② | identityHashCode / finalize / 弱软虚引用 / Object.clone——166 实测零覆盖，探针用例随对应 S 条目修复排队 |

### P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| ~~生成输出非确定性~~ | **✅ 已修（`b6ab58d`，G-4）** | 两处源：vars.py 提升集合迭代 + downcast 链 registry 插入序（新发现的第二源）→ 均排序遍历；双种子生成树 diff 归零。附带 `[raw-audit]` 仪表落地（`90d2e93`） |
| ~~catch 变量作用域（2 例）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestDateTimeFormat / TestZonedDateTime PASS |
| ~~接口槽位成员缺失（2 例）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestStreamNumeric / TestPriorityQueue PASS |
| ~~一次性编译错（4 例）~~ | **✅ 已消解（2026-09-24 晚分支 `claude/jolly-dijkstra-diftum` 实测 PASS）** | TestOverload / TestStringSearch / TestSwitchNull / TestCollectorsMore 全 PASS |
| ~~用户类 import 生成缺口~~ | **✅ 随 A-8 消失（2026-09-22 核验）** | TestCustomException 干净树编译通过（Ubuntu 2224f02+ 与 macOS 双确认）——E0425 被 A-8 的 `collect_referenced` 字节码引用集驱动导入扩展顺带修复；**S-19 #5 栈帧 diff 现在直达**（仅 `has stack frames` 1 行） |
| ~~异常层次 upcast CCE（2 例）~~ | **✅ 随 macros 修（`548c08b`）** | 同上条 |

### P3 · 长期重构

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| ~~R-2 `Deref<Target=Parent>` 替换 From 继承链~~ → ~~**R-2′ 统一上转写法**~~ | **R-2 关闭（前提已过期）；R-2′ 第一步 ✅ 完成（分支 `claude/jolly-dijkstra-diftum`）** | R-2 原前提不存在：`From<Child> for Parent` 链早已删除，`__into_super` 零处；wrapper 对象模型下子类不持有父类实例，Deref 无 `&Parent` 可借，且上转全为按值，接口载体已占 Deref<Target=Object>，泛型父类 Deref 目标唯一——**不可行**。上转真源在宏 type_conversions.rs §10（禁改域，保留）。**R-2′ 第一步**：类祖先按值上转的发射形态唯一决策点 = `render.upcast_expr(src, wrap)`（none/clone/paren/auto，形态恒为后缀 `.into()`；`is_atomic_rs` 随之下沉 render），8 处调用点（fields/arrays/returns/invoke_sig/stack×2/vars×2）统一改调；删除死代码 `_into_super_chain`（恒返回 `.into()` 的存根）/ `_super_prefix_to_expr` / `_super_path_to_class`；修正 class_writer/invoke_sig/jvm_type 过期注释；翻译参考 §5 按现行模型重写（字段平铺 / `.into()` 上转 / 继承转发与 `Owner__m_base` super 调用 / 为何不用 Deref），5 份历史计划加过期标注。**验收**：19 例（继承/上转主干 + 合并点/泛型父类/大 JDK 链）生成树修改前后**逐字节一致**、raw-audit 逐测试相同、fallback none；单测 `tests/unit/test_upcast_expr.py` 逐字对拍旧写法。**第二步（未做，需全量对账）**：形态统一为 `<T as From<_>>::from`（改 upcast_expr 一处即可，但改变每个 j.u.c 大工作区约 277 行生成树）；之后可升级为 `UpcastExpr` IR 节点（方案 C，随 IR 结构化）。调研 `docs/reports/2026-09-24-r2-deref-survey.md` |
| **M-3 方法签名类型决策进宏** | IR 结构化完成后 | 宏从 `#[descriptor]`/`#[generic_signature]` 自行做 JVM→Rust 类型选择，Python 只传原始字符串 |

### 2026-09-25/26 完成（K-JCA / S-66 验证链揭出的通用修复）

| 任务 | 状态 | 证据 |
|---|---|---|
| S-67 synchronized 内模式 switch 的 CFG try 区域嵌套 | ✅ | `7e4f690`；TestSyncPatternSwitch、TestPatternSlotReuse PASS |
| record 分量反射 `Class.getRecordComponents0`（含无分量 record → 空数组） | ✅ 代码完成，验证排队 | `37f0e71` + `c82f8bd`；e2e TestRecordComponents |
| Method.invoke 形态补全（char 装箱 / 拓宽拆箱 / 数组与泛型载体 / 实参不符不包装） | ✅ 代码完成，验证排队 | `f47071d`；e2e TestReflectInvokeShapes |
| 反射访问检查按声明类归属近似调用方 | ✅ 代码完成，验证排队 | `f70fd96`；e2e TestReflectAccessCheck |
| 协变返回桥承接抽象祖先槽位（codegen） | ✅ 回归批 jcaB2 7/7 PASS | `fe11203`；VarHandle.withInvokeExactBehavior 实证 |
| VarHandle 字节数组视图 get/set + 宏 `__unsafe_bool_cell` | ✅ 代码完成，验证排队 | `06eece1`；e2e TestByteArrayViewVarHandle |
| 边界类常量池引用沿超类链解析到翻译祖先（JVMS §5.4.3.3） | ✅ 代码完成，验证排队 | `f096054`；SecureRandom.nextInt → Random.nextInt |
| instanceof 开放层次不折叠（接口 × 非 final 类改运行时判定） | ✅ 代码完成，验证排队 | `bdfd80d`；e2e TestInstanceofOpenHierarchy |
| `Class.forName0` + `VM.latestUserDefinedLoader` | ✅ 代码完成，验证排队 | `35344a6`；e2e TestClassForName；偏差入 compatibility.md |
| `Charset.forName`（sun/nio/cs/StandardCharsets 边界）+ `BootLoader.hasClassPath` | ✅ 代码完成，验证排队 | `40779ab` + `73584f4`；e2e TestCharsetForName |
| `StreamEncoder.close`、SilentLogger log 重载、`SecureRandom.getProvider` | ✅ 代码完成，验证排队 | `d267714` / `9ec606c` / `070e779` |

## 2026-09-26 归档批次（二）

| # | 任务 | 状态 | 证据 |
|---|---|---|---|
| N7 | 第 4 项 macOS 侧验证 | ✅ 用户 macOS（arm64，JDK21.0.11）@2f8ba87 实测 | TestFilesApi / TestFileOutputStream PASS（provider 链 MacOSX→Bsd→Unix 通）；TestCharsetForName 揭出 getCallerClass 帧不可解析退回 null → `ServiceConfigurationError: no caller to check`，已修（退回可信类 Object） |
| — | JDK25 语料适配（原 8 例 E0308 硬阻塞，L1 兼容改写 + decimal_digits 手写） | ✅ 已合入（2026-09-26 核对活跃表时发现过期） | `08ddbf7`+`7caefcc`+`ddb1672`（merge `a63df7c`），见本文 JDK25 L1 math 双件行 |
| — | 异常兜底收窄（A 组只兜 CfgError、B 组 [fallback-audit]、RAVA_STRICT 分级） | ✅ 已合入（2026-09-26 核对活跃表时发现过期） | `466f513`；`codegen/fallback_audit.py`，转译输出 `[fallback-audit]` 行 |

## 2026-09-26 归档批次（三）

| # | 任务 | 状态 | 证据 |
|---|---|---|---|
| N12 / #38 | JDK25 TestCompletableFuture 运行 4 分钟（JDK21 0.3s） | ✅ 本机 JDK25 实测 **0.36s**（j25a，`5b54dd5`） | 根因：协作调度下限时 park 立即返回，FJP awaitWork 空转至 keepAlive 真实截止。修复：虚拟时钟 `a8c027e`（限时 park / wait / sleep 无线程可推进时时钟跳至截止）；e2e TestVirtualClockPark JDK21 / JDK25 PASS、TestWaitNotify / TestCompletableFuture JDK21 PASS。**真实时间语义的终态由 #42 真多线程取代**（用户决策 2026-09-26，方案 `docs/plans/2026-09-26-real-multithreading.md`） |
| K-JCA / #39 | JCA 服务注册（算法实现类字节码翻译 + 静态服务表） | ✅ 本机 JDK21 + JDK25 实测 | JDK21（tgt21）：Digester / TestCipherDesModes / TestMessageDigestApi / TestSecureRandomApi / SecurityDemo PASS，DES 经三元合并修复 `e7fbbab` PASS（mh2）；JDK25（j25a）：DES / Digester / TestCipherDesModes PASS。收尾修复：CryptoAlgorithmConstraints.permits（新版 JDK21 / JDK25，`1e9da59`+`1425bec` 重载名）、JDK25 GetInstance.getServices 返回 Iterator（编译期 cfg `jdk_ge_25`，`5b54dd5`）、athrow 克隆（`180ab8c`，MH 播种揭出的 E0382） |


## 2026-09-26 归档批次（四）

| # | 任务 | 状态 | 证据 |
|---|---|---|---|
| S-10 / #43 | 手写静态 native 不触发类初始化；带 default 方法的接口自身不初始化 | ✅ 本机 JDK21 dev1 实测 | `jvm_native` 属性宏向返回 Result 的静态 native 注入 `Self::__class_init()?`（`no_class_init` 豁免）；codegen `#[init_interfaces]`（JVMS §5.5 step 7）由宏消费（`ee3de51`）。ClinitOrder / StaticInitTest / TestInitOrder / TestInterfaceInitOrder / TestStaticInit PASS |
| S-7 | record `hashCode` 恒为 `Ok(0)` | ✅ 本机 JDK21 dev1 实测 | 按 ObjectMethods 31 多项式生成（`1d2af2f`）；TestRecordHashCode / TestRecord / TestRecordAdvanced / TestRecordPattern PASS |

## 2026-09-26 归档批次（五）

| # | 任务 | 状态 | 证据 |
|---|---|---|---|
| N4（扩大口径） | TypeIR 扩大口径 type_surgery_ext 21→0 | ✅ 生成树逐字节验收 | `d6822f0`：调用点文本解剖收口到解析层 type_args（rust_type_head / rust_type_partition / rust_type_arg_text / is_array_carrier / is_vec_type，语义逐点相同）。验收集 27 例基线 `d6c7960` 对照逐字节一致、raw_expr/raw_stmt 逐测试相同、唯一差异 type_surgery_ext 21→0；单元测试 140/140 |
| N6 / #41 | 手写 `_impl.rs` 分配的对象不进 RTA | ✅ 本机 jcaN6 10/10 | `04999fe`：native_upcalls 识别手写分配并登记 RTA 已实例化。ListFields（Field.toString 字节码翻译）/ HelloWorld / TestArrayList / TestRecordComponents / TestReflectAccessCheck / TestClassToString / TestZonedDateTime / TestVirtualThread（真 OS 线程）/ DES / TestCipherDesModes PASS |


## 2026-09-29 归档批次（tasks.md 已完成行整体迁出）

| # | 任务 | 状态 | 证据 / 说明 |
|---|---|---|---|
| 9 | M-3 宏拆库试点 | ✅ **M3-c 已实施（`7c27493`，用户决策方案 1+3）** | M3-a/b：宏侧纯位映射 + 差分审计（6 测试 120,567 处零分歧）；M3-c：宏侧纯位类型一致性断言常开（失配 compile_error），生成器保留显式类型（可读性）；映射最终归属随 R0。方案 `docs/plans/2026-09-25-m3-signature-types-in-macro.md` §六 |
| N2 | 反序列化实例化语义 | ✅ 用户验证 TestSerializationPrimitives / SerializableDemo PASS（`f71f288`） | 分派伪成员 `<alloc>` / `<init_on>` + 序列化构造器旁表 + Unsafe 基本类型 get/put 经 reflect_field；e2e TestSerializationPrimitives / SerializableDemo |
| N12 | 运行时清单整合（`runtime/java_runtime/*.txt`） | ✅ `d449f77`（用户 2026-09-28 决定「先做 1、2、3」） | 11 个 txt → 3 个 TOML：`closure.toml`（[boundary] / [vm_boundary] / [release]）、`seeds.toml`（[annotation] / [locale] / [jca] / [data_bundle] / [boot_init]）、`vm_intrinsics.toml`（[[intrinsic]] member/kind/reason、[caller_sensitive]、[sigpoly]）；读取统一 `codegen/runtime_manifest.py`（tomllib + 形态校验：packages 须 `/` 结尾，classes 覆盖自身与 `$` 嵌套类、不作字符串前缀）。验收：12 例生成树逐字节一致 + raw-audit 一致。第 4 项：`carrier_type_positions` / `signature_erased_interfaces` 随 T-2 删除（FS-M2），`overload_abbrev.txt` 保留 |
| S-66 | Java 对象序列化（ObjectOutputStream / ObjectInputStream，含 record） | ✅ 普通对象（SerializableDemo / TestSerializationPrimitives）+ record（RecordsSerializationTest 本机与用户 macOS PASS） | RecordsSerializationTest 已越过写侧与反序列化 doPrivileged / Unsafe 静态字段 / 字段访问器解析，当前层 `Wrapper.convert`（`0b87fe2` 待验证）；依赖 N11 |
| N13 | 项目改名 rava + 项目自有环境变量清零 | ✅ 2026-09-28 `93aa6e7` / `dbbcb05` / `f72ce0b` | `java_rta` / `java-rta` / `JAVA_RTA_*` 全部改名（宏 crate `rava_macros`）；`RAVA_*` 14 个环境变量改为命令行参数（`--debug` / `--strict` / `--trace-class` / `--raw-sites` / `--build-timeout`）、生成文件（`strict.txt`、`OUT_DIR/jdk_feature.rs`）与标准变量（`RUST_BACKTRACE`、`CARGO_BUILD_JOBS`），见 `docs/environment-variables.md`；重型闭包自动单作业编译（`scripts/cargo_env.py`） |
| N11 / #40 | MH-native：MethodHandle 原生调用模型 | ✅ MH-native 主体 + BMH 动态物种（`BoundMethodHandle$Species_Dyn` VM 支持类，方案 `docs/plans/2026-09-27-bmh-dynamic-species.md`）；TestMethodHandleDirect / TestMethodHandleCombinators / TestBmhDynamicSpecies / RecordsSerializationTest 全 PASS（2026-09-27） | 方案 `docs/plans/2026-09-26-mh-native.md`：InvokerBytecodeGenerator 入 vm_boundary、invokeBasic 原生 LambdaForm 解释器、linkTo*/成员调用经 reflect_invoke、常量反射引用播种、签名多态调用点 `__site`、字段句柄经 reflect_field。e2e TestMethodHandleDirect / TestMethodHandleCombinators（`tests/e2e/59_method_handles/`）。后续：组合子全集 + RecordsSerializationTest |
| #42 | 真多线程（OS 线程 + JVM 等价时间 / 同步语义） | ✅ 并行后端为默认且唯一（2026-09-27：`mt` feature、单线程 + GIL 分支删除）；阻塞项 TestCommonPool / TestParallelArrayCas mt 用户 Linux PASS @b0bb6c1（根因 `de6aa98` 数组协变 CAS 原子化）；全量回归随用户测试滚动覆盖 | 第一档 `a5476f3`+`cca3e92`+`c9931b4`+`fed7032`：gil2 @fed7032 TestAtomics / TestCompletableFuture / TestVirtualThread / TestWaitNotify / TestConcurrentClinit / TestSleepParkClock / TestSpinVolatile / TestThreadCounters / TestThreadInterrupt / TestThreadStates / TestThreadUncaught / TestWaitNotifyQueue PASS（TestJucSync 为磁盘满编译失败，复跑排队 cs1）。第二档（用户 2026-09-26 要求按最终态实施）：`0ca915a` 抽象层 mt 后端 + 原子 CAS 族；mt 编译错误 3169→408→0（HelloWorld mt 运行 PASS，用户验证）；`RAVA_MT=1` 以并行后端跑 e2e。方案 `docs/plans/2026-09-26-real-multithreading.md` |
| FS-H0 | 手写覆盖只许 ACC_NATIVE（审计线 + 越界覆盖清零） | ✅ 2026-09-28 `non_native_overrides=0`：JCA J1/J2（Provider / Service / SecureRandom 回到字节码）、Enum.valueOf（FS-H8）、StackTraceElement.computeFormat、FileCleanable（no-op 下沉内部边界 PhantomCleanable / CleanerFactory）、DecimalFormatSymbols.initializeCurrency（L-2 货币数据层）；边界类（closure.toml [vm_boundary] 与 [boundary] 内未放行类）的手写方法单独计数 `vm_boundary_methods`（用户决定），随子系统复核。验证：TestCipherDesModes / Digester / SecurityDemo / TestSecureRandomApi / TestEnumBasic / TestEnumAdvanced / TestCustomException / FileIOTest / TestFilesApi / TestFormatLocale / TestCurrencyApi PASS | 方案：`2026-09-28-jca-faithful-provider.md`、`2026-09-28-l2-currency-data.md`；审计 `[override-audit]` / `[vm-boundary-audit]` / `[upcall-audit]` |
| FS-N1..N5 | Math 手写覆盖删除（`random` 恒 0.5、`IEEEremainder` 用 round、`pow` NaN 规格、libm ulp、双下划线死代码） | ✅ `5c30b8c`（用户 mt 验证 TestMathSpec / TestMathExact / TestMathRound / MathEnhancedTest / Heron PASS；ComprehensiveTest 未报） | 全走 StrictMath / FdLibm 字节码链（`wide iinc` 修复 `8242bbe` 为前提；StrictMath.sqrt 入内建清单）。验证：TestMathSpec TestMathExact TestMathRound MathEnhancedTest Heron ComprehensiveTest |
| FS-H1 / H4 / H6 | `Character.digit` 非 ASCII、`Arrays.copyOf` 组件类型、`Properties` defaults 链 | ✅ `936c6a9` / `5b980a3` / `803aa05`（用户验证 TestArrayComponentType / TestCollectionFactory / TestArrayList / TestPropertiesDefaults PASS） | 验证：TestThreadOverridesSpec ✅ TestArrayComponentType TestCollectionFactory TestArrayList TestPropertiesDefaults |
| FS-T3 | `availableProcessors` 恒 1 | ✅ `18da936` + `2f64669`（TestCommonPool 本机 + 用户 Linux PASS、TestCompletableFuture PASS；mt 下 TestCommonPool 见 #42） | 宿主真实并行度；公共池工作线程所需安全类边界实现。验证：TestCommonPool TestCompletableFuture |
| FS-E3 / M7 / M8 | 不可捕获 panic（checkcast / NPE / toString 路径）、null 接收者字段访问不抛 NPE、`_is_jnull` 非 Object 载体恒假 | ✅ M7 `0442777`（TestFieldNullReceiver ✅）；E3 `2f64669`、M8 `5bf2286`（用户验证 TestToStringThrows / TestGenericNullCheck PASS） | 验证：TestToStringThrows TestGenericNullCheck |
| FS-M5 / C5 / T6 | 身份哈希截断、forName 不立即初始化、Thread native 缺失 | ✅ `b443134` / `6d885a2`+`c6933ca` / `8dba713` | 验证：TestIdentityHashSpec ✅ TestThreadNatives ✅ TestForNameInit ✅ TestThreadStates ✅（mt，含 `427e2ff`） |
| FS-T5 / R4 / IO1 / IO2 / M4 / L11 | InternalLock 按实例、反射访问检查按调用方、access / statvfs、装箱 instanceof 与 UTF16 hash 回归测试 | ✅ `3209e52` / `0d79cd1`+`625ba4a` / `9af1c05` / `ebbf6f8` / `619bd59`（用户验证 TestThreadCounters / TestJucSync / TestReflectAccessCheck / TestFilesApi / TestBoxInstanceof PASS，TestFileAccessSpace 本机 PASS；TestMethodHandleDirect 归 N11；L11 ✅） | 验证：TestThreadCounters TestWaitNotifyQueue TestJucSync / TestReflectAccessCheck TestMethodHandleDirect / TestFileAccessSpace TestFilesApi / TestBoxInstanceof |
| 链路修复（09-26 下半） | S-16 用户类虚槽填充入链、签名多态方法 resolve、`supportedFileAttributeViews`、字段读写求值顺序 | ✅ `aaedd95` / `fa1de49` / `86039f1` / `3a8ff90`（TestCommonPool / TestFieldEvalOrder / TestFileAccessSpace PASS） | 用户类继承 JDK 虚方法（ForkJoinTask.exec 等）用 JDK 调用链填槽；MethodHandle.linkTo* 按任意描述符解析；UnixFileSystem 视图集合；`f(x, x = …)` 在 putfield/putstatic 前物化同字段惰性读取（并行流 RangeLongSpliterator.trySplit 左半段为空的根因）。验证：TestCommonPool TestFieldEvalOrder RecordsSerializationTest TestFileAccessSpace |
| 链路修复（09-27） | 边界类手写覆盖继承虚方法入槽、static 与祖先实例方法同名 mangle、手写覆盖方法的元数据行、形参擦除+返回收窄桥填槽、StaticProperty 编码族、ProcessImpl.init、Array.getLength、UnixPath 路径运算 / createDirectory、mt 监视器 BLOCKED | ✅ `9b43d14`+`b26f4da` / `6e531fe` / `46fae8b` / `2532b19` / `f4d0351` / `e2f78ef` / `9b43d14` / `2e063db`+`b1c505f` / `427e2ff` | 已验证：TestSystemExitEnv（本机 + 用户 mt）、TestThreadStates（用户 mt）、TestArrayComponentType / TestVirtualThread / TestPropertiesDefaults / TestShutdownHooks / TestSystemPropsSpec（用户）。⏳ RecordsSerializationTest、TestFileAccessSpace、TestMethodHandleCombinators、TestMethodHandleDirect |
| 链路修复（09-27 下半） | MH 族：LambdaForm 解释入口非空、Wrapper Enum 身份 / wrapperType / isConvertibleFrom / convert / cast、MethodHandleNatives 静态字段偏移；安全：doPrivileged(PrivilegedExceptionAction)；Unsafe 静态字段臂 + 字符串常量命名静态字段的字段臂；调用链：边界类手写体 upcalls 经虚分派入链、边界祖先手写覆盖遮蔽远祖存根；nio：createDirectory / isReadable 族 | ✅ `cd4d558` `ba2984a` `400df89` `bbf0cec` `0b87fe2` `20e0ed6` / `9008dd2` / `9af5f58` / `7fb5045` `0b9a434` / `b1c505f` `728ca75` | 已验证：TestFileAccessSpace（本机 Linux）、TestCommonPool（本机 + 用户 Linux，非 mt）。⏳ RecordsSerializationTest、TestMethodHandleDirect、TestMethodHandleCombinators（逐层推进中）；mt TestCommonPool 超时（本机复现中） |
| FS-O1..O3 | compatibility.md 过期行、失真注释、过渡宏（java_synchronized / java_switch!）文档 | ✅ `1e670f0` / `24e8c81` | 文档真实性 |
| ~~**T-1 bounds 进宏**~~ ✅ 2026-09-28 `63b4eca` | — | 类路径早已由 `java_class!` 注入约束；最后一处 Python 拼写的约束（宏块外协变 upcast impl）改由新过程宏 `iface_upcasts!` 与 `java_class!` 同源补齐，生成器只写裸参数名；TestArrayList / TestStreamBasic PASS |
| 链路修复（09-29，用户 JDK21 跑批反馈） | DecimalFormat HALF_EVEN（FS-L8）、ThreadLocal.get 类型变量取回（FS-Q1 回归）、多维接口数组 checkcast、旧日历 API、ByteBuffer 写入、线程容器关闭、Console、zip 校验和、紧凑数字格式、泛型主类 main、构建警告清零 | 🔄 `3020d8c` `0049b7c` `a3eddb5` `68d1ca1` `5d430b1` `fe0e08a` | FloatingDecimal / FDBigInteger 放行字节码翻译（删手写 Schubfach 近似）；stack 存入类型变量局部的 From::from 分支对 MethodCall 等节点生效；ObjectVTable `__array_accepts` 递归判定内层数组；closure.toml 放行 `sun/util/calendar/` + TimeZone native；ScopedMemoryAccess 写入族；SharedThreadContainer.close；SharedSecrets.getJavaIOAccess + Console native；BootLoader.loadLibrary no-op + Adler32 / CRC32 native；NumberFormatProvider.getCompactNumberInstance + LocaleResources.getCNPatterns / getRules；泛型主类 main 擦除实例化；宏前文档注释移入宏体 + 生成代码放行 unused_comparisons。已验证：AnglesNormalizationAndConversion、CYKParser。⏳ TestDecimalFormatHalfEven、CalendarTask、ByteBufferDemo、VogelsApproximationMethod、CheckOutputDeviceIsATerminal、ChecksumTest、CompactNumberFormatExample、Currier |

## 2026-10-04 tasks.md 一行一任务整理时移出的过程记录

> 2026-10-04 将 `docs/tasks.md` 整理为「一行一任务」格式时，从表格单元格、依赖树续行与执行约束中移出的过程性内容（实测数字史、抽查记录、原记录、已完成子项证据），按原文搬入，按任务分小节。tasks.md 中「历史 §X」即指本节下的 X 小节。原文中的状态以搬出时（集成分支 rust-closure-analyzer 21fc601b）为准，较新的状态以 tasks.md 为准。

### A. 执行约束第 4、5、7 条原文

4. **任务执行顺序（2026-09-23 同步）**：~~陈旧树筛→A-8/S-20/数组视图→downcast 链→S-19→A-5/A-4~~ **全部完成**。当前：ice 修复①② + `__unsafe_int_cell` 委托 + TestSealed `.0`（在途双开）→ K-6b 双侧一致（6 例）→ G-3 三重槽（窗口 3 前置）→ TypeIR 批次 3（invoke 域 9 处）→ M-3 试点 → 窗口 3 → P-1/Rust 重写 R0（三信号中"发现频率月级"仍差）。**子代理并行纪律：本机 ≤2（内存）+ 另一机 2；本地禁全量（定向 ≤10 例），全量归用户服务器**。
5. **当前执行顺序（2026-09-24 晚，用户确认）**：清单 3/11（M3）→ 4+20 → 5（TypeIR）→ 6（R-2′）**均已完成** → **7 窗口 3（进行中）** → 8 P-1 → 10 R0（门槛②③达成后）；13 M5、19 equiv 探针、N 系列按依赖穿插。1/2 收官轮待你执行，数据回来后 16–18 插队。挂决策：12/14/15。
7. **当前执行顺序（2026-09-29 刷新）**：FS-H0 ✅、N12 ✅、N13（改名 rava + 环境变量清零）✅、T-1 ✅（已归档）；**FS-Q1 Raw 收敛**：Q1-a..d ✅、Q1-e 十余批（单测试 raw 102.6K → 23.4K，生成树逐字节一致 + 编译验证），余值叶子收益递减、暂停；**T-2 接口载体铺开 🔄**（7a 访问器接口 ✅、7b Path ✅、标记接口 upcast ✅、7d 兼容部分 ✅、7c 桥方法签名代入修复验证中 → 名单置 None + 删两个过渡 txt）→ FS-M 组；N14 闭包收窄（按入口剖面）；#7 G-2 全量对账（用户测试滚动）→ #10 R0 启动。

### B. 原 20 项清单 #2、#7、#16 原行

| # | 任务 | 状态 | 证据 / 下一步 |
|---|---|---|---|
| 2 | JDK25 全量轮 | ⏳ 用户 `--failed` 轮 @8bf6baf：**23/34 出列、余 11**（2026-09-25） | 余 11 例本机逐一复现并修复（`197dc82` + `bacf062`）：Unsafe 数组常量族（静态字段 core_ 适配）、JLA currentCarrierThread / uncheckedCountPositives、MhUtil 4 参、putDecimal（#17）、DateTimeHelper.formatTo、接口接收者超接口重载命名（E0061）、BaseLocale 常量表按版本（Locale US/DE 对调）、FJP 见证值 CAS 族。本机 JDK25 定向 14 例：12 PASS；TestCompletableFuture 推进至 FJP 层已修待复跑；TestLocaleConstants 余 fr/it 格式化（L-1 数据缺口）。**下轮 `--failed` 预期余 1（TestLocaleConstants）** |
| 7 | 窗口 3（G-1/G-2 迁 rs_ir） | ✅ **全部实施完成，待用户 JDK21 全量对账（G-2）** | 拆分方案窗口 3 结构步骤全部完成：⑧ blocks.py→unify/fusion（`5a14b30`，973→552 行）、⑨ `_store_local` 7 阶段（`9d4b007`，391→≤102）、⑩ `_hoist_if_vars` 6 阶段（`e97e49c`，375→≤106）；G-1 结构化 ✅：G1-a 块节点 BlockStmt + flatten（`cf457e3`）、G1-b 块结构判定改读 emitter 标注、删除花括号计数与文本特征判定（`49692a0`，双算 330 万次 0 分歧）、G1-c 引用检测 IR 化（双算 6.4 万决策 0 差异）。每步 23 例生成树逐字节一致——**G-1 终态（结构与引用文本匹配 = 0，Raw 节点除外）达成**。**G-2 ✅**：提升前置声明去 Default 占位 → `let mut x: T;`（Rust 确定赋值分析验证未初始化即使用）；18 例零 E0381 全 PASS（含 8 例大闭包）+ 13 例回归 + m1/m2 golden OK。**改变生成树，待用户 JDK21 全量对账**。计划 `docs/plans/2026-09-25-window3-g1-structured-hoist.md` |
| 16 | JDK25 第四失配（`sun/security/action` E0432）及后续 | ✅ **本机冒烟全绿，待用户 JDK25 全量确认** | 用户 JDK25 全量 @4ccd3ff：81/172（compile 85 中 81 例同一 E0432，run 5 例 stub）。本机装 OpenJDK 25.0.2 逐层推进，修复链：①E0432 = JEP 486 移除 SecurityManager 后 JDK25 整包删除 sun/security/action，手写 UnixFileSystem 改调 System.getProperty（`9df7959`）；②E0599 VarHandle 签名多态方法 = project_writer 对 var_handle_impl 的过期依赖登记（`9df7959`）；③stub：Unsafe.isBigEndian、ThreadSleepEvent.<init>+Event.isEnabled、UTF_32 三件套 <init>、Thread.sleepNanos0（sleep0 改名）、JavaLangAccess unchecked*（改名）、HexDigits.digitPair（`9df7959`/`5ec92fe`/`3a3312e`/本提交）；④9df7959 引入的 JDK21 TestFilesApi 回退已修（FileSystems 手写链逐跳 upcall，`3a3312e`）。**本机 JDK25 冒烟**：HelloWorld / TestTernary（81 例大闭包形态代表）/ TestConstructorChain / TestThreadJoin / TestHexFormat / TestFilesApi 全 PASS；JDK21 受影响回归（FilesApi/PrintStreamApi/ThreadJoin/StringEdge/Atomics/CompletableFuture/HexFormat/HelloWorld）全 PASS。3 例 cargo 依赖拉取失败属用户环境（已重试）。JDK 选择：未指定 --jdk 固定走 .jdk-version=21（`9439d46`+后续），JDK25 须显式 `--jdk 25` |

### C. 新增 / 遗留 N4、S-65、N8、N14 原行

| # | 任务 | 状态 | 说明 |
|---|---|---|---|
| N4 | TypeIR 完全体能力 G1–G5 | 🔄 G1/G2/G3/G5 ✅（2026-09-28）；G4 归 M-3 | 扩大口径 22→0 已完成（见归档批次（五））。2026-09-28：G5 双查询面合一（stack 查询改 jvm_type 薄转发）、G2 `from_rust_type(tparams=)` → TypeVar（coerce 装箱分类 + `_coerce_arg` 7 处形参判定）、G3 `HostPrim` 变体（`_forms_alignable`）、G1 `from_rs_type` 节点桥（S3 / S8 已迁）；每步双算插桩零差异或生成树逐字节一致。进度表见 `docs/reports/2026-09-24-typeir-remaining-survey.md` 开头。余：其余「render_type → 串 → 解析」位点随 FS-Q1 Raw 收敛迁移 |
| S-65 | **新语料 65 例分层抽样（JDK21）：51/65**（2026-09-25） | 🔄 修复已推送，复跑排队 | 14 例失败全部归因：拼接模板含 `\n`（WordWrap/SwitchExpressionTests/PatternMatchingForInstanceOf，`cd2cf2c`）、模板含控制字符（BWT，`6e7dae3`）、大写开头参数名（WordWrap，`cd2cf2c`）、`FloatingDecimal.parseDouble` 存根（IntegerMethodsDemo/TypeCastingTest/RPN，`afe7481`）、合成槽跨分支异型（RecordPatternsTest，`3e61885`+`798c78c`）、大闭包四族（DataEncryptionStandard：兄弟分支合并 / 类型变量形参 / Object 声明首绑定 / derive(Debug) 遮蔽，`6b76885`+`e87e9b5`）、Class 未入 RTA（SumDataType，`079b9e7`，N6 同族）、环境（ExceptionPropagation OOM / NicePrimes 磁盘满 → prune.sh `8a70bc4`）、对象序列化（RecordsSerializationTest/SerializableDemo：FileOutputStream 原生层 `e5f5ba0`，本体见 S-66）。每类配 e2e（TestUpperCaseLocalNames、TestConcatTemplateWhitespace、TestParseDoubleEdge、TestPatternSlotReuse、TestBranchLocalMerge、TestObjectLocalWidening、TestTypeVarBoundArg、TestClassToString、TestFileOutputStream） |
| N8 | 服务器编译资源约束 | 📝 已记录 | 单 rustc ~14G 内存；共享 target 每测试残留 0.5–1G，跑批间需清理（`scripts/prune.sh`；后台跑批用 `scripts/run_bg.sh`，自带低内存编译环境）。2026-09-25 本机（16G 容器）JDK25 TestVirtualThread（76+1699 类）debuginfo=2 下 rustc 峰值 13.8G 被 cgroup OOM 杀；`CARGO_PROFILE_DEV_DEBUG=line-tables-only` 下通过（二进制 507M→270M）；**2026-09-28 实测上限**：line-tables-only + `CARGO_BUILD_JOBS=2` 下 1777 类（TestEnumAdvanced）、`JOBS=1` 下 1807 类（TestCipherDesModes，rustc 13.9G）被 OOM 杀——约 1750 类即贴线。已做：InetAddress 名字服务栈截断（closure.toml [vm_boundary]，Cipher 闭包 −1.3MB）；重型用例自动 `CARGO_BUILD_JOBS=1`（rava 编译阶段 `driver/src/cargo.rs`：声明层 `java_runtime` 类 ≥ 1700 且未显式设置时，`rava build` / `rava compile`（run_tests 经后者）均生效；显式设置 `CARGO_BUILD_JOBS` 即覆盖自动判定，重型闭包构建超时自动放宽至 3000 秒）。根治靠生成代码体量下降（N4 / FS-Q1 raw 逃生舱收敛、超大 `<clinit>` 如 KnownOIDs 的发射形态）；2026-09-28/29：所有编译缺省 `CARGO_PROFILE_DEV_DEBUG=line-tables-only`（`f900436`）、生成类 ≥ 1700 自动单作业 + 构建超时 3000 秒（2026-10-02 起在 rava `driver/src/cargo.rs`） |
| N14 | 闭包膨胀：几行 println 的用例闭包 ~1800 类（单测试 build 10–20 分钟、rustc 峰值贴 14G） | 🔄 2026-09-28 | ① VM 常量守卫死分支剪除 ✅ `0e261b6`：`vm_intrinsics.toml [vm_constants]` + `generator/crates/input/src/prune.rs`（classfile 解码后单点剪除，调用链与生成代码同源）；`ThreadLocalRandom.<clinit>` 的 `java.util.secureRandomSeed` 分支不再拉入 SecureRandom / NativePRNG 族，TestFieldEvalOrder 1827 → 1813（收益小）。② **主因是虚 / 接口分派的 RTA 扇出**：`--trace-class` 实证 `MethodHandleImpl$CountingWrapper.updateForm` 经 `Function.apply` 落到闭包内全部已实例化实现类，`PrivilegedAction.run` 同理拉入 `ObjectStreamClass$1` → 序列化 → MessageDigest / SUN provider。下一步：出分派收窄方案（候选：调用点接收者静态类型 + 实例化点可达性的 XTA / 按字段 / 按方法实例化集合；lambda / 匿名类实例化点按所在方法可达性登记），先做剖面（各扇出点贡献类数）再定。③ 2026-09-28 实验：接口分派从 CHA（全部已加载实现类）收窄到仅已实例化类，闭包 1813 → 1812——**非主因**，已撤回。剖面：1812 类 / 约 3.4 万个真实方法体 + 1.4 万 stub / 生成代码 30MB，调用链确实触达约 3 万方法；需按入口逐个做「移除该边的闭包差」剖面定位大头。④ 2026-09-29 拆 crate 可行性实测：HelloWorld 闭包 1813 个生成类的类型引用图中最大强连通分量 1761 类（97% 类 / 99% 代码量），其余 52 个均为单类分量——Rust crate 不允许循环依赖，拆分只能剥离约 1% 代码，**不可行**（打断环路需跨 crate 全部改动态分派，违背可读性目标）。降 rustc 峰值的方向：闭包收窄（按入口剖面）+ `java_class!` 宏展开体积剖面（逐祖先 From / 接口 upcast / vtable / 反射元数据中「生成未用」的部分）+ 机器侧 swap 兜底 |

### D. 依赖树（2026-10-02 版）C1d-a 节点原文

```
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
```

### E. 依赖树 C1d-b 节点原文（含 b1′ T2–T7、b3 两段、lambda 隐藏类 / from_any 归零）

```
│   ├─ 🔄 C1d-b 反射与过近似收窄（c1d-pick，2026-10-02-c1d-reflect-narrow.md）
│   │     ├─ ✅ b0 阶段合入 e90a592d（eb6571ba）：m3 serialVersionUID、同一数组自拷贝、反射字段按值流点名（TestReflectProbe ✅）
│   │     ├─ 🔄 b1′ ArrayList.writeObject 分派臂：2026-10-02 交接拆分（c1d-pick 2c16454b，计划 §4.6；WIP 快照 c1d-pick-wip 63d90e86）
│   │     │       T0 基线抽查 c1db-2c16454b → 并行 ✅ T6 getSuperclass 返回模型（7a0188e6）、✅ T5 MH→putReference 清单精确化（cc6b59e0）、
│   │     │       ✅ T7 探针转正 --flows（1518612d）→ ✅ T4 未知 Class 字段枚举收窄（2311459e；余 2 处字段全开源头归 b1） → ✅ T3 反射回调按接收者克隆上下文（4c3a614e 合入，抽查 9/10；6 s 回收，StockTrans 分派 22949→1645；MH 按句柄路由实测上界 −1 类，不做） → 🔄 T2（c1d-t2：writeObject0 对象池 / 名字×镜像 / forName0 回退 / Serializable 有界镜像） 名字×镜像交叉（最后合）
│   │     │       验收：StockTrans / TestSerialDefaultSuid / TestSerialProxyForm 通过、fold_props ≥42、m3 golden
│   │     ├─ 🔄 b1 序列化收窄：✅ S2 ReflectUtil 放行 / 静态 CAS（含子字宽）/ 返回模型收窄合入（c1d-b1 0d7dd2a5；StockTrans 1803→1794、DeepCopy 1804→1789）；余：DeepCopy 距 ≤1640 目标，归 T2
│   │     │       验收：DeepCopy ≤1640 类、fold_props ≥42、StockTrans / TestSerialDefaultSuid 回调保留
│   │     ├─ ⏳ b2 任务 2 ◀── why2-93e0f28e 取证
│   │     └─ ✅ b3 任务 3（2026-10-03 收官，d46d9b06：12 例测量集类初始化缺口全 0，HelloWorld 467 / StockTrans 3107 / DeepCopy 3102 类）：class_init.unknown 归 false——✅ 第一段合入 1721f701（aeff784b）：class_init 钩子、截断体同类调用登记（bool2byte）、
│   │             跳过 Class#<synthetic>、子字字段 CAS（每字段 4 字节槽 + __unsafe_word）；第二段 01c88c11 抽查 c1db3-01c88c11 10/10：
│   │             基本类型数组元素 Unsafe 访问、S3 getCallerClass（CallerSensitive 记字节码所在类）、sun/misc/Unsafe 放行、静态字段钩子每次访问连边
│   │             （修 ThreadTest）；01572ce6 协议名常量分支折叠收窄加载器链（ThreadTest 1445→346，新增 TestBuiltinUrlProtocol）；
│   │             ✅ 第二段合入 b1983313（84c92245，抽查 c1db3-84c92245 10/10；含 setContextClassLoader 存根修复 + TestThreadContextLoaderInit）；
│   │             余：扇出收窄（registerNatives 开放接收者 toString、URL$DefaultFactory 反射构造器）、✅ CallerSensitive 经方法引用 / MH（382cf3e1 合入，抽查 10/10：MN_CALLER_SENSITIVE、BindCaller 注入调用器 InjectedInvokerDyn、Method.invoke 适配；方法句柄类用例闭包 −22）；✅ S5 手写值池合并拆分（fd76553d 合入，抽查 8/8：FieldAccess.value_fresh、getDeclaringClass0 按接收者、TestDeclaringClassInit；7 例测量集类 / 方法集合不变）；✅ S7 Class.forName 拼接类名字符串值流建模（3553df09 合入，抽查 c1db3-3553df09 9/9：拼接各段可确定时折叠为常量串集合，新增 TestForNameComputedName）；✅ DMH checkInitialized / shouldBeInitialized 未知站点归零（2eb9403a 合入 3473d696，抽查 c1db3-2eb9403a 9/9：DMH 未知点建模、按名查找静态字段初始化声明类、TestMethodHandleStaticInit；合并适配 875afe99）
│   │     ├─ ✅ lambda 隐藏类（c1d-lambda-class 0060fa77，合入 94d2ff90）：每调用点 Host$$Lambda/0x… 隐藏类、超类 Object、接口 + 标记接口、
│   │     │       isHidden / isSynthetic 按类元数据、实例判定按超类型集合；TestLambdaHiddenClass；from_any 归零（2026-10-03-from-any-zero.md）：
│   │     │       ✅ ① 审计按类计数含 java_body_*（56506adb，合入 ec714d98；真实基线 2–78）；✅ ② A+B 超接口 / 接口视图类型实参（b820c8aa 合入；27 例 from_any 2–78→1–4，闭包不变）；✅ ③ C+D 方法级类型变量 / super.m()（44b3a3b3 合入；27 例中 26 例 from_any=0，StockTrans 11→0）；✅ ④ 余下发射点统一 Object::from / Into<Object>、void 入 Object 改内部错误、from_any=0 守护测试（631bb78b 合入；27 例 + StockTrans / LambdaHiddenClass 全部 from_any=0，闭包不变）——**from_any 归零达成**
```

### F. 依赖树 native-gaps / FS-C2 / boot layer / regress2 续节点原文

```
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
```

### G. 依赖树「e2e 扩展到 java.base 之外的 JDK 模块」节点原文（含 R1–R13、参考 JDK、TestLocaleCurrency 等）

```
│   └─ ⏳ e2e 扩展到 java.base 之外的 JDK 模块（2026-10-03-jmod-coverage.md）
│         用例已写入（feat/framework-pilot-matrix 4939f290 合入：64_–74_ 共 43 例，期望由 JDK 21 生成；71_xml 11 例）
│         ✅【2026-10-03 完成，feat/junit-expected-redundancy d4efc8d6】63_junit expected 10/10（junit+hamcrest cp、JDK21 实跑、双跑确定性全过；顺修 3 处源码错误：assertTrue 静态导入缺失、遮蔽 helper、Sample 构造器非 public 致 initializationError）
│         ✅【2026-10-03 完成，同分支】新增用例查重：133 例 ∩ 冗余候选 = 5、相似对交集 0，逐条论证全部保留（定向回归网/独有边界/算法族/jmod 档设计），无删除建议；报告 docs/reports/e2e-redundancy-newtests.md
│         ✅【2026-10-03 用户侧完成，feat/e2e-failure-triage ffbebc0d】6 例输出不符 expected 复核：expected 纠正 0；生成器缺陷 5（本机 6c0adc44 复现 5/5）；环境 1（TestLocaleCurrency：CLDR 随 JDK 21 小版本漂移）
│         ✅【2026-10-03 用户侧完成，同分支】e2enew-da8abee1 失败 69 例归因：13 根因族，报告 docs/reports/e2enew-da8abee1-triage.md，jmod §七 实测列已回填；派生条目：
│           ├─ ⏳ R1 xml lambda 存根 SecuritySupport.lambda$getSystemProperty$0 可达性（10 例，71_xml）◀── jmod 第 1 步
│           ├─ ⏳ R2 泛型反射 scope 构造器存根 + E0432（6 例）、R5 Method.invoke 实参数量 / 类型不符抛 IAE、R11 compareTo 桥分派闭包 ◀── 并入 T2 队列
│           ├─ ⏳ R3 JCA ProviderList / GetInstance 存根（5 例）◀── C1d-a JCA 收窄线
│           ├─ ✅ R4 跨模块 import 断链 java.logging E0433（4 例；2026-10-03 服务器复跑 genfix-r4-a2e2a7e5 4/4 通过，归环境侧，无代码改动）
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
```

### H. 依赖树优化线节点原文

```
│   ├─ ⏸ 闭包分析效率 P8 余量、sites（optimization-directions §三.2）
│   ├─ 🔄 生成器 / 下游编译成本：V1–V7、S 系列余项（emitter-performance、rustc-memory-and-crate-split；10-04 分层决策与待验证清单 V1–V12 见 docs/plans/2026-10-04-archive-crate-layering-decisions.md）；2026-10-04 派 crate-split（声明层拆分，先服务器实测 OOM 三例再落 §7.5.4 收益最大项，S7 只出方案待批）；unsafe-rmw 合入 5f759708（字段槽按 Rust 名登记，Unsafe 引用 RMW 两例出已知失败清单）
│   ├─ ◇ S7 统一对象句柄 + 每类静态描述符 ─┐
│   ├─ ✅ T1 跨测试编译复用决策（2026-10-01-cross-test-compile-reuse.md，99dc658f 实测：档案 3609 类，全量 ≈35→≤11 机时）—— 用户 2026-10-03 四项全采纳：档案化 + 分发层、CLAUDE.md 第 2 条改写（已改）、开放世界折叠、语料动态 / 生产静态链接；C1d / C4 收官后按 §5.3 实施
│   │     └─▶ T4 生成器只构建一次再分发（待服务器核实）
│   └─ ⏳ JDK 25 适配轮 ◀── C4 收官（JDK 25 不设 Python 基线；8 台服务器 JDK 25 已就绪，env_setup --check-only 2026-10-03）
```

### I. 关键路径图与说明原文

```
C1d-a a1 ✅ ──▶ a2 f2bdcf6e 验收（余 1 例）──▶ a3 jvm_boundary 归零 ─────────┐
C1d-b b1′ writeObject 分派臂 / b1 序列化收窄 ───────────────────────┤
C6 ✅ 3f9d4cc9 ──────────────────────────────────────────────────────┼──▶ C4 全量 e2e ──▶ S6 ──▶ S7 ──▶ JUnit A ──▶ B/C ──▶ 真实项目 pilot ──▶ 产品化
native-gaps ✅ ──▶ FS-C2 ✅ ──▶ boot layer（另需 C1d-a a2）───────────────┤        │
               └─▶ regress2 栈帧来源统一 ✅ be1b97be ─────────────────┘        └──▶ 优化线恢复（P8 / V / S7·T1 决策 / R1 / JDK 25）
```

工期瓶颈是 C1d-a a2：闭包规模已达标（正式 HelloWorld 423 类 / 2–3 s，余量转 a5），只剩 TestFileStoreMountLookup 重跑验收（TestUnixFileNatives 已过）；a2 合入前 boot layer 第 1 步起、regress2 遗留、a3 都不能启动，C4 全量 e2e 随之顺延。

### J. 活跃任务表原行（C1d-a、C1d-b、a3-T、T1、native-gaps、FS-C2、regress2 续、boot layer）

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| C1d-a 去截断（c1d-p0） | 🔄 2026-10-03 | 现阶段：闭包闸门 P2/P3，最近合入 51a4d8c5（JCA 请求点值流）；StockTrans 3283 / DeepCopy 3278 类，目标 DeepCopy ≤1640；jar/URL 来源精度乙 ✅ 合入 fca1643b（URL.getURLStreamHandler 按键闸门；闭包不变，根因 §22.10：URL host 无逐对象精度），下一步待派 URL host 逐对象字段精度（证 file URL host 为 "" / localhost 杀 ftp 分支）+ 甲 class-path。以下为早期记录：a1 ✅ 正式 HelloWorld 423 类 / 2–3 s；a2 抽查 c1da-f2bdcf6e 7/8（StockTrans 基线），余 TestFileStoreMountLookup 重跑；之后 a3 审计数 86→0、a5 OOB 关系推理；后续项 precheck 按目标平台扫描 |
| C1d-b 反射收窄（c1d-pick） | 🔄 2026-10-02 | b0 阶段合入 e90a592d（TestReflectProbe ✅）；ArrayList.writeObject 分派臂已拆为 T2–T7（计划 §4.6），T4 / T5 / T6 / T7 已合入，b3 第一段已合入（1721f701），b1 已合入（0d7dd2a5），T3 已合入（4c3a614e），b3 第二段与 CallerSensitive 续段已合入（84c92245、382cf3e1）/ b3 DMH 段已合入（3473d696）/ b3 扇出收窄已合入（6294755d / ee52c596：手写 Object.toString 开放接收者收窄，HelloWorld 499→467；rava closure 参数严格校验）/ T2 ✅ 合入 35c5f0ee（E0599 / E0592 / jline、构造器查找点名种子无关化、同名同类型别槽局部合并修 EUC_TW byte2；抽查 13/14，TestFieldHandleProvenance 为 rustc cgroup OOM，归声明层拆分线）；T2 余项待派新代理：getDefaultSerialFields 收窄、4b，StockTrans 目标按 JCA 后基线重定（计划 5.7）；StockTrans 既有基线失败 ArrayList.writeObject 反射臂待 T2 2e1d3355；新派 A gen-fixes（生成器独立缺陷合集，计划 docs/plans/2026-10-03-gen-fixes.md；第 1 步层次重载计入接口未实现成员 E0061 已合入 52cde496，抽查 6/6；第 2–4 步（LMF 返回值拆箱 / 拓宽、catch 变量重赋值、java_try! 内层 try 带标签 break、isAssignableFrom 数组协变、reflect.Array 18 native）已合入（0e1a5d1f，抽查 13/14）；第 5 步 R4 服务器复跑 4/4 通过、归环境侧（a2e2a7e5，仅文档），gen-fixes 五步收官；A 续派：TestJndiNoProvider 冷闭包转译性能（计划 docs/plans/2026-10-03-jndi-transpile-perf.md；600s 上限不放宽）：第 1 步去重复计算 / 增量化已合入（d314fda7 / 26720aff，198.7s→126s，集合逐项一致，抽查 5/7 余为既有失败）；⚠️ 查实闭包顺序依赖（形参常量格 Const 窗口内名字被当站点字面量与接收者镜像相乘，--flow-batch 7 vs 64 得 3589 vs 3540 类）——修法 B（在 c1d-t2 lookup_pair 之上推广按调用点配对 + Const 形参保留 Src::Param + Class 形参形状 + flow-batch×seed 含缺口集合不变性守护）待 c1d-t2 合入后由 A 做，基准随之变更；等待期 A 清零 non_native_overrides=2 已合入（2816cfa8 / 601b0e3b，抽查 4/6，JCA 两例为既有失败）；C1d-a JCA 种子修复已合入（0db8060f / 8855d5c8 → 51a4d8c5，抽查 9/10 + 6/6；代价 StockTrans 3141→3283，+134 来自 jar 签名校验路径 4 个算法名不可定的请求点），下一步 jar 签名校验路径收窄，原 ② 容器元素 Object 方法归入通用 open 值精度另立项，③ 排其后）、B a3t-vthread（VirtualThread 翻译 + Continuation 有栈协程；T1 rava_coro 已合入（4a1b008c / 80bdb64e）：slab 多栈预留使映射数与存活协程脱钩、软件栈界，kr1 x86_64 复测切换 14.96ns、10⁵ 挂起映射 37→38；T1b-2 叶方法豁免已合入（d71fe910 / 40193ed3，抽查 11/12）；T3 执行上下文迁移已合入（5face54b / 488b5811，抽查 8/9）；T4 删方案 A + 接通 Continuation 进行中；T1b 方法入口栈检查注入 + stack-check VM 规则已合入（322e72a8 / 65b6ecb3，抽查 18/19，StockTrans 失败为 T2 族）：全体闭包 +1 类 StackOverflowError，__TEXT +3.7–4.7%，release 调用开销约 1.3ns / 入口；⏳ T1b-2 叶子方法省检查、运行时手写代码无界递归审计；T2 Continuation 6 native + 执行上下文块 + 三类 pin 计数已合入（39759ab8 / 1f76c606，抽查 13/13；⚠️ 既有回归：non_native_overrides=2——EventHelper.isLoggingSecurity、GetInstance$Instance.toArray，集成 67e14d7e 上即存在，待清零）；⏳ T4 接通（验收口径：CRITICAL_SECTION 公开 API 不可达，由 rava_coro 单测与审阅保证；TestContinuationPinned 两情形随 T4 验收））；序列化收窄 WIP：大值集来自未知接收者字段视图，目标 DeepCopy ≤1640、fold_props ≥42 |
| a3-T 虚拟线程终态（a3t-vthread） | 🔄 T1–T5 ✅ a78cccef | T4 VirtualThread 字节码翻译 + Continuation 有栈协程、T5 跨栈 panic 已合入（抽查 11/12 + 复查 3/3）；待派 T6 规模指标（10 万虚拟线程 ≤10 s / ≤2 GiB，现 14.6 s / 2.66 GB，草稿未提交）、T1b 手写运行时无界递归审计；TestContinuationPinned parkNanos 早返偶发需查。交接见计划 §21.8.5 |
| T1 档案化（t1-profile） | 🔄 1a ✅ 90398dc8 | 多根开放世界分析 + 档案键 / 内容摘要 + rava profile（27 例档案 3244 类 = 单例非用户侧之并，178 s / 2.0 GB；抽查 10/10）；1b ✅ 10cfb657（按档案生成 JDK crate、java_meta 拆 JDK 表 + 用户登记；修 meta.rs 宏化访问器丢手写边致种子差异，加 macro_fn_lint；抽查 14/14）；第 2 步方案 t1-link（f1f9d78c，直接 rustc 链接档案 dylib）用户 2026-10-04 定：语料模式 panic=unwind、生产开发无 LTO / --release fat LTO、档案 crate 按 jmod 模块命名（按模块切分论证中） |
| native-gaps · native 缺口补齐 | ✅ 417a6594 | 已知失败 TestUnixFileNatives（待 C1d-a）、TestModuleLayerDefine（待 boot layer）、TestClassNestNatives（待 FS-C2） |
| FS-C2 应用类加载器 | ✅ 4a98f5e3 | TestClassNestNatives 通过；vm_boundary_methods 30→27；交接 C1d-a：ServicesCatalog / JLA 补丁随过渡手写删除 |
| regress2 续 · 栈帧来源统一 | ✅ be1b97be | 遗留 Object.wait 帧行号、过渡 <init> 帧 ◀── C1d-a a2 |
| boot layer | 🔄 第 0 步 ✅ 27dfb419 | ModuleBootstrap 引导建层；第 1 步起 ◀── C1d-a a2（FS-C2 ✅） |

### K. P3 T-2 接口泛型进类型位置原行

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-2 接口泛型进类型位置**（擦除阶段 1 已落地：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`） | 🔄 2026-09-29：7a / 7b / 标记接口 / 7d 兼容部分 ✅，7c 验证中（方案 `2026-09-28-t2-interface-carriers.md`） | 剩余：接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类/枚举/手写类的 `__interface` 覆盖。**收尾交付**（清单整合第 4 项，`d449f77` 决定）：接口载体全面铺开后删除 `runtime/java_runtime/carrier_type_positions.txt` 与 `signature_erased_interfaces.txt`（FS-M2 最终态，不迁移进 TOML）；`overload_abbrev.txt` 保留，不另建 `codegen_types.toml` |

### L. 依赖树其余精简节点原文（已完成大分支 C1d-b 第一段、C6 后续、JUnit 步骤 0）

```
│   ├─ ✅ C1d-b 第一段 c1d-pick（4b73ea61）
│   ├─ ✅ C6 后续（c6-generic-closure 7fdbfa8c，合入 3f9d4cc9）：泛型辅助 fn 闭包形参、TestAnnoNestedArray null_recv、用户注解类型补种
│   │     步骤 0 m1..m5 Rust 路径复跑 golden，清零回归（可提前；m3 编译 0 ✅，运行期存根由 C1d-b b0 处理）
```

# 任务管理（当前活跃）

> 创建：2026-09-16（取代已归档的 `docs/tasks-history.md`）
> 定位：**只保留开放的问题与任务**。完成即关闭删除，不累积历史。
> 历史任务（T01-T81 全记录）见 `docs/tasks-history.md`；R5 轮集成后的完整遗留清单见 `docs/plans/2026-09-19-remaining-issues.md`。

---

## ⚠️ 执行约束（最高优先级，不可绕过）

1. **架构问题优先**：先做架构改造，测试错误待架构完成后自然消解，禁止因为测试失败而中断架构工作转去修 Bug。
2. **架构完成前禁止全量测试**：定向验证（红线集 + 金丝雀）除外，全量 run_tests.py 只在架构节点合入后由主会话统一执行。
3. **子代理串行执行**：一次只运行一个子代理（用户指定，内存约束）；前一个完成并合入验证后再启动下一个。
4. **任务执行顺序（2026-09-23 同步）**：~~陈旧树筛→A-8/S-20/数组视图→downcast 链→S-19→A-5/A-4~~ **全部完成**。当前：ice 修复①② + `__unsafe_int_cell` 委托 + TestSealed `.0`（在途双开）→ K-6b 双侧一致（6 例）→ G-3 三重槽（窗口 3 前置）→ TypeIR 批次 3（invoke 域 9 处）→ M-3 试点 → 窗口 3 → P-1/Rust 重写 R0（三信号中"发现频率月级"仍差）。**子代理并行纪律：本机 ≤2（内存）+ 另一机 2；本地禁全量（定向 ≤10 例），全量归用户服务器**。

---

**关联追踪文档**：
- `docs/plans/2026-09-19-remaining-issues.md` — **当前主线**：A/S/G/P/V/R 六类遗留问题全清单（架构缺口/JVM 语义/生成器质量/原则违规/验证/仓库）
- `docs/plans/2026-09-21-codegen-type-convergence.md` — **生成器类型系统收敛路线图**：擦除-恢复/字符串手术/特例 if 链的统一诊断、五层调整清单（TypeIR/M-3/A-4/转换 IR 化/downcast 链）、量化终态
- `docs/plans/java-rust-translation-reference.md` — 翻译对照（宏家族 §16）
- `docs/tasks-history.md` — T01-T81 历史全记录

**基线（2026-09-22 凌晨，用户 Ubuntu 全量 @ ~afa3890 树，双进程与定向批并发）**：**113 PASS / 53 FAIL**（较初基线 87 **+26**）——compile 13 / run 33 / output 6 / transpile 1，run 族自动分类：stub-hit 16、runtime-panic 9、s8-crash 1、unclassified 7。**失败清单双进程合并实战通过**（定向批写 22 条保留 32 → 全量合并 53，无丢失）。**`--failed` 回归已实证线程层三例转绿出列（a705fd9 树：TestSynchronized/ThreadJoin/WaitNotify PASS，清单 53→50）+ `--skip-failed` 干净面 113/113 全绿——有效基线 **147/169**（144 + HashSetOps/MapIteration/TreeMapSet）。**Ubuntu 2026-09-23 凌晨双轮实证（树 548c08b，pull 到 cdc6422 前的最后旧基线）**：--failed 37 例 2 出列（NestedTry/Suppressed=macros 修实证）→35；--skip-failed 干净面 **133/133 全绿**（含 FieldDemo/ReflectProbe/CollectionFactory/ListOf/LambdaVar/StringEdge 等全部近期修复用例）——**该树全量口径 135/168**。清单剩余 35 与 main 最新（cdc6422）已合未验差≈compile14 八例+S-8→下轮 pull 后清单预期 35→26**。棘轮全循环验证：全量播种 → --failed 回归出列 → --skip-failed 零失败快速面。TestCollectionsUtil 层进（getDeclaredField 随线程层落地，下一卡点 Unsafe.objectFieldOffset）；新见 native：Reflection.getCallerClass（TestAtomics 下一层）。**2026-09-22 13:48-16:12 `--failed` 轮（树=f4636d0，时间戳对照定位）**：7 出列对账——5 stubs 绿 + TestCustomException（S-19 #5）+ **TestLambdaVar 意外转绿（本地 e4abf67 实证归因：栈帧轮顺带修复**，其 ImmutableCollections AME 链经 throwable 构造路径变化消除**）**；清单 48→41。**reflect 已合入（08c60c3）：下轮 pull 后 TestCollectionFactory 出列→40**。新形态记录：TestSequencedCollections 推进至 output（NullableKeyValueHolder 类型名 toString=已归类桥接项）；TestAtomics→ExceptionInInitializerError；新 stub：RandomSupport$AbstractSplittableGenerator.<init>（TestRandomSeed）。TestRefKindsFull PASS→FAIL 复现定性：`stub: Integer.valueOf`（与 TestOptional 同族，装箱旁路 from_any 被对象化收编后揭开，非行为回归）。此前 compile 族多项推进到下一层（HashSetOps→UOE remove、GenericBoundsCombo→collection.rs panic、DateTimeFormat/ZonedDateTime→E0308）。

**基线（2026-09-21）**：
- **166 全量（用户 Ubuntu，JDK21）**：87 PASS / 79 FAIL；**跑批树落后 main，混有陈旧污染**——fcb04ce 干净树复核 18 例，TestStringBuilder / TestOptionalFull / TestIncDec / TestShortCircuit 已 PASS（假象），真实失败面待复验收敛。归类全记录：`2026-09-21-e2e-baseline-classification.md`（新立项 A-8 / S-19 / S-20；S-8 / S-9 实测升级）。
- 红线 19/19 全绿（a20a31f 实测）；TestStringBuilder 编译 0 错误、输出 22 行与 Java 逐字一致（fcb04ce 复核 PASS，含金丝雀）。
- **streams 三测全绿（2026-09-21 午后，P-3 合入后主会话 --clean 6/6）**：TestStreamBasic / TestStreamAdvanced / TestStreamCollectors + TestStringBuilderOps 全 PASS，输出逐字一致。
- 架构里程碑链：vtable 双指针多态 → CFG 支配树结构化（含 try/catch `java_try!`、`<clinit>` 语义、异常对象）→ println 字节码化 → K-6 槽位签名模型 → 擦除运行时身份阶段 1（非泛型 `I__VTable` + `__interface`）→ S-18 接口分派 BFS（fcb04ce）→ P-3 边界补全（7b64afa，join + DoubleToDecimal）。
- 教训：**全量跑批必须 tee 落盘**（本轮 224 分钟 stdout-only，38 个 run 族无 stderr 只能二次定向）。

---

## 🔴 活跃任务（当前队列）

| 任务 | 状态 | 目标 / 说明 |
|------|------|------------|
| ~~陈旧树筛 + run 族定向复验~~ | **✅ 完成（2026-09-21 午后）** | 30 例复验 + 26 run 族 stderr 定性 + 4 output 族复验全记录：归类文档 §4.2/§五。假失败第 6 例（TestMethodRef）；数组视图族升 6 例；CDS/isBigEndian native 双件 8 例 |
| ~~A-8 同文件辅助类未进闭包~~ | **✅ 完成（`1be8caa`）** | 15/15 E0433/E0425 清零，4 例全过（FieldShadow/EnumAdvanced/InnerClass/InstanceOfChain），闭包指纹 20/20 一致；连带修复 P-3 潜伏回归 Appendable E0432（`0b22604`，四红线干净树恢复全绿） |
| **A-8 下一层（11 例 compile 清零后暴露，已归类）** | 166 归类/A-8 报告 | 泛型用户父类**参数位擦除**（TestBridgeMethod，A-1 参数位延伸）；~~中文双重编码（2 例）~~ **✅ 随 S-19 #3 MUTF-8 修复顺带关闭**（TestConstructorChain/TestInitOrder Ubuntu+macOS 双确认）；**record toString 的 double `.0` 渲染**（TestSealed `Square[side=2]`→`2.0`，java_fmt_f64 的 record 组件路径，S-19 #4 邻域补件）；接口冲突 default 分派（TestInterfaceConflict）；接口 private 方法载体（TestInterfacePrivate）；方法引用接收者 coerce（TestMethodRefKinds）；泛型 record `==`（TestRecordAdvanced）；`HashSet.remove` stub（TestHashSetOps）；TestGenericBoundsCombo/TestVarContext 的 Appendable 缺口已随 `0b22604` 消失待复验 |
| ~~S-20 `Object.wait/notify/notifyAll`~~ | **✅ 完成（`23fe881`）** | `monitor.rs` 双条件队列监视器 + monitorenter/同步方法/同步块四发射面真实化；12/12 编译清零、TestStringSearch 全绿、红线 23/23 含 streams。**未接 InternalLock（论证见 monitor.rs），S-11 终态随线程模型** |
| ~~native 双件：CDS + isBigEndian~~ | **✅ 完成（`9873095`）** | CDS 恒 0（HotSpot 语义）+ isBigEndian `cfg!` + Thread.registerNatives no-op；6+2 用例编译/运行推进，TestStringSearch 全绿。**下一层已归类**：线程层总闸 3 例、ImmutableCollections 载体分派 3 例（A-4 邻域）、SharedSecrets.getJavaUtilCollectionAccess 2 例（P-3 邻域）、Charset clinit、toString 装箱分派 1 行 |
| ~~TypeIR 最小层~~ | **✅ 完成（`e488ec5`+`92e3db5`）** | `jvm_type.py` 六变体代数（erasure/substitute/is_subtype_of + registry 闭包缓存）+ 35 单测；试点 hierarchy._is_subtype 委托——204 万对对拍零分歧、双种子零 diff、五审计线一致。**下一批接入点已排**：control.py(7)/stack.py(5)/returns.py(5)/invoke_sig.py(5)/fields.py(3)/blocks.py(2)/arrays.py(2)——A-5 合入后可续（invoke_sig 在其域） |
| ~~TypeIR 批次 2：六消费点迁移~~ | **✅ 完成（`c440b73`，7 提交）** | control(12 含 A-4 债务2)/stack(6+共享擦除查询入口)/returns(10)/fields(5)/blocks(3)/arrays(6)——**type_surgery 69→27**；行为零变化四重验证；+10 单测。余量 27 大头=invoke_sig 5+invoke_virtual 4（批次 3，排 compile 族后） |
| ~~A-4 批次 3-6：类型位置载体化~~ | **✅ 主战役收官（批次 6=`0526bb4`，第二任零修复纯验证收官）** | CharSequence/Appendable + decimal 双侧对齐 + iface_carrier_views 接口位；记分牌 TSB 1010→913（−100=CharSequence 残余兑现）；主会话 5/5 定向复核全绿；Spliterator 沿暂缓（证据 §9：阻塞面在特化桥接非类型位置）。**累计 Into<I> −49%** |
| ~~A-4 批次 3-5：类型位置载体化~~ | **✅ 完成（四任接力，`2bb2255` 合入）** | **`Into<I>` 32931→17106（−48%）**：批次 3 Iterator 穿透（carrier_type 单一决策点）→ 批次 4 集合族 → 批次 5 函数式族 39 接口 + 载体 instanceof 运行时化（抓修 streams 红线破口）。遗留：CharSequence/Appendable 手写层阻塞（证据 §8）、Spliterator 沿、TSB 残余 1010 分布、type_surgery +7 债务（TypeIR 批次 2 消化）。主会话修 stubs×a4b3 语义冲突（impl 签名对齐） |
| ~~A-4 批次 6：CharSequence/Appendable 载体化~~ | **✅ 完成（`fix/a4b6-charsequence`，前任 WIP+本任验证收官）** | §8 遗留 1 双侧落地：铺设两接口 + sig_parse 早返回（越过 `_CLASSNAME_MAP` 旧擦除）+ `#[iface_carrier_views]` 接口视图臂（JLS 4.10.3 数组协变/checkcast 的接口位）+ decimal 三文件 appendTo 载体签名。九例定向全绿（全量待服务器）；TSB 1010→913 / TSC 1009→912 / TPM 975→878；bfs-audit、双种子、from_any 持平。Spliterator 沿评估维持暂缓（证据 §9） |
| ~~A-5 lambda 对象化~~ | **✅ 完成（`4fa0f6d`+`54e2026`+`50eede0`）** | `<Iface>__Lambda` 合成对象（证据驱动 27 接口+共置伴生）+ `__interface` 应答 + SAM/default 单一路径；**downcast_ref 70→0、from_any −60.5%**；主会话 7 测试复核吻合（6/7，唯一失败=子串带入基线项）。**from_any 残余 1588 全归 A-4——A-4 已解锁（A-5 合入）** |
| ~~线程层（S-20 下一层总闸，3 例）~~ | **✅ 完成（`8eca47b`）** | 单线程协作调度（start0 就绪队列 + join/wait/sleep 嵌套泵 + monitor ticket 等待集联动）；currentThread 字段按 golden 反推；**三例全绿**（主会话 4/4 复核含 TestExceptions）。条件等价边界六条已入 compatibility.md。遗留：TestVirtualThread 卡 ContinuationSupport（下一层）；getDeclaredField not-found 分支无字段元数据表 |
| ~~数组视图 coerce 族~~ | **✅ 完成（`fix/array-view-coerce` 4 提交）** | 根因两层：anewarray 数组类组件拼出非法描述符 `L[I;`（类型保真缺口）+ `try_cast` 数组分支探针退化恒真落 panic。修法：`try_array_view` 唯一决策点合一（可捕获 Err）、实参/返回位发射（areturn 曾静默 `Default::default()` 错值）、`Object.clone` 真浅拷贝、`System.arraycopy` 规范化。**3 例全绿**（ArrayCopy/BigDecimal/DurationPeriod）+ ArrayCovariance CCE 消除可捕获 + BigInteger compile 清零；JDK25 判定=部分同超族（载体返回位已补），硬阻塞为语料适配。闭包指纹逐字一致，红线 21/23（2 失败=基线既有同型同点） |
| 数组 getClass 命名（1 例） | 数组视图族遗留 | TestArrayCovariance 唯一剩余 diff：`type=String[]`→`Object;`——vtable `__class_name` 返回 `&'static str` 无法携带元素类型，独立族（签名限制级） |
| JDK25 语料适配（原 8 例 E0308 的硬阻塞） | 数组视图族判定 | 17/23 错误=JDK25 语料 DoubleToDecimal 结构变化（手写 overlay 字段失配）+ concurrent 成员解析；另有 LVT 间隙声明一处反向装箱。JDK25 定向任务，随 166 全量 JDK25 轮立项 |
| ~~runtime 散点三件包~~ | **✅ 完成（`9b58672`+`928328f`）** | **3 例全绿**（TestOptional/TestRefKindsFull/TestRandomSeed，主会话 5/6 复核含回归）：Integer.valueOf（[-128,127] 缓存保身份 + toString 顺带收层）+ RandomSupport 族（upcalls 接口级回调边让 BFS 自动翻译 SplittableRandom 实现类——零手写零枚举）。**第 3 件改判归 macros 域**（见下） |
| ~~异常 upcast 中间型 catch 缺口~~ | **✅ 完成（`548c08b`，主会话直做）** | inner `__erased_vtable` 运行时类覆盖 + wrapper 委托；**TestNestedTry/TestSuppressed 双绿**，13 测试回归全过。踩坑：`Rc::clone` 泛型推断方向（inner 侧显式 `as` 上转） |
| ~~S-8 负数组崩溃~~ | **✅ 件 1 合入（`030b7ab`，另一台机器代理交付，主会话复核确认）** | 三创建指令经 `JArray::try_new/try_new_with` 抛真实 NegativeArraySizeException（Err 可捕获）；不可失败形态饱和为空数组作手写层崩溃防线；Arrays.copyOf 同接入。**TestArrayBounds s8-crash 转绿**（我首次复跑遇陈旧 scratch 假失败，fresh 后 PASS——个中差异已核实为生成物未刷新）。等效审计口径同步更新（neg-array 转为创建点总量观测） |
| ~~ImmutableCollections 分派族（ice-dispatch 调查）~~ | **✅ 调查报告交付（零改码，另一台机器代理）** | 7 例根因表：①default 注入扫父类链（_emit_interface_default_inheritance 只查本类——10-15 行，**建议最先做**，HashSetOps/MapIteration 2 例）②unify_pair 灭真值（TreeMap_KeySet 三元真臂→Default::default()，TreeMapSet 1 例）③ListIterator 未进语料闭包（attrs.py _compute_all_supertypes 断链，AutoboxEdge/LinkedHash 2 例，随 A-4 排期）④string Display null 守卫（HashMapOps 1 例）⑤GenericBoundsCombo 本机不复现需对账。行号纠正：iterator.rs:12/17 是宏 span 非真 bug 位点 |
| ~~反射族 Method 元数据表（membername）~~ | **✅ 完成（`d8f17b8`+`376a6cd`+`6564a04`，主会话 5/5 复核）** | L1 方法表（build.rs，(name,descriptor) 键+exceptions 列）+ L2 `getDeclaredMethod/getDeclaredMethods` 转正 + MethodHandleNatives.resolve 内核 + 12 层伴生放行——**TestAtomics FAIL→4/7 行**（ai/ar/al/ab 四行正确）；**MethodDemo 转正**（语料 168→169）。**下一层归类**：`Unsafe.compareAndSetInt` 经 AQS 基类视图无共享 int 单元——宏域 `__unsafe_int_cell` 臂委托缺口（与 `__erased_vtable` 同型的 inner/wrapper 不对称，A-4 域，修法≈wrapper 臂失败后委托 self.vtable 或 inner 补臂） |
| ~~compile 14 族攻坚~~ | **✅ 8/14 全绿（10 提交，主会话 10/10 复核含红线）** | 全绿：RecordAdvanced(E0369)/SwitchNull(S-17 完成：String/Integer 常量标签判定桥)/InterfacePrivate(Java9+ 私有方法落载体)/MethodRefKinds(绑定接收者 coerce)/BridgeMethod(**桥方法体落槽统一机制**+toString 恒虚)/PriorityQueue(接口闭包二段解析 JLS 5.4.3.3)/CollectorsMore(return this 擦除重建)/Pecs(BFS 接口闭包 stub 通道)；StreamNumeric 10错→2。**遗留 6 例归类**：DateTimeFormat/ZonedDateTime/FilesApi=**G-3 三重复用槽**（try 暂存+监视对象+catch 形参同槽，需窗口 3）；StreamNumeric×2/StringNewMethods×4=**K-6b 双侧一致**（宏 __impl_ 体改写）；CompletableFuture=anewarray `_` 落 arrays.py（另一机 scatter4 域，诊断已移交） |
| ~~反射族 Field 元数据表~~ | **✅ 完成（`18f1fa2`+`2803715`+`417c315`，主会话 5/5 复核）** | build.rs FIELD_TABLE + getDeclaredField not-found 抛 NoSuchFieldException（可捕获）+ getComponentType/Array.newArray/Field.get/set 最小实现；**TestCollectionFactory 转绿**；探针转正 FieldDemo/TestReflectProbe（**语料 166→168**，golden 以 java 实跑生成）；MemberName 方法表评估入档（`2026-09-22-method-metadata-table-eval.md`：方法属性 4 空格对齐坑、L3 分派协议须与 A-4 合流） |
| ~~ice 修复①+②（default 父类链 + unify_pair）~~ | **✅ 完成（`33a63e9`+`15565bf`，主会话 5/5 复核）** | **3 例全绿**（HashSetOps/MapIteration/TreeMapSet）：default 注入覆盖判定沿父类链累积（+21 行，_anc_ifaces 防线不动）+ unify_pair 不灭真值（具体臂经 Object 边界重建，86bf63a 同型；null 臂合法路径未误伤）。**顺带恢复 CompletableFuture.postComplete 三元语义**（基线隐性灭 null，因编译未过从未暴露）。调查表修复③仍在队（ListIterator 断链→另一机）；④见下 |
| ~~ice 修复④（string Display null 守卫）~~ | **✅ 完成（`c70ddc2`，wt 代理交付，基线 cab5a63）** | Display impl 加 vtable `is_jvm_null()` 守卫（S-3.1 统一入口，+8 行）：null 载体输出字面 `"null"`（JLS §5.1.11/§15.18.1），不再对 Repr::Null 数组 `to_vec` panic（array.rs 批量读取 NPE 链）；与 Object 侧 null 呈现（`() vtable __obj_str`）同语义。**TestHashMapOps 全绿**；StringBuilder/StringOps/StringFormat/ArrayList/Collections 零回归（7/7 含 filter 顺带 StringBuilderOps/CollectionsUtil） |
| **TestBigInteger 运行期 NPE 再定性（ice ④伴调查，零改码）** | 调查报告（codegen 域，未跨域） | 症状 `Exception in thread "main" java.lang.NullPointerException`（unclassified，非 Rust panic）实证链：isProbablePrime→primeToCertainty(certainty, null)（bitLength=7<100）→passesMillerRabin 内 `if(rnd==null) rnd=ThreadLocalRandom.current()`（字节码 27-34 `aload_2/ifnonnull/invokestatic current/astore_2`，LVT 槽 2 `rnd:Random` 覆盖 0..166）的赋值落**死局部 local_2**，形参 rnd 未重绑定→`new BigInteger(7, null)`→randomBits→null Random 上 `nextBytes` NPE。**stubs 轮「codegen 参数重绑定」归类复核成立，形态从编译错变为运行期 NPE**。位点（stack.py `_store_local`）：①:387 形参槽强制 `decl=None`→decl_ty 子类上转分支（:446 段）不达，值保持子类型；②既有两形参分支（:622 null 字面量 / :630 Object 形参）不命中具体类形参+异型值；③:677 槽位复用改名按「名字全域匹配」误命中——形参自身 LVT 条目恒同名→恒改 local_N 死局部（其注释本意是「存储点不在任何声明区间内」的 javac 合成变量）。修法≈10-15 行：补第三形参分支（具体引用类型形参+异型引用值→`From<Object>` 按形参声明类型重建赋回，与 :647 段同变量分支同源）；可选收紧 :677 为偏移区间命中。对照组：BigDecimal.toString 局部变量重绑定正常（`sc = Clone::clone(&_t0)`），坏组合专属「形参槽+异型值」。修复后下一层风险：ThreadLocalRandom.current→localInit/U/instance 链（已实现非 stub，可运行验证） |
| ~~unsafe-cell 委托 + record toString `.0`~~ | **✅ 完成（`4916401`+`574a5ff`，主会话 6/6 复核含 TestRecordPattern 基线项区分）** | **TestAtomics 7/7 全绿**（inner cell 臂运行时类应答+wrapper 委托，与 __erased_vtable 同型第三例——inner/wrapper 不对称缺口家族就此三修三验）+ **TestSealed 全绿**（record toString 浮点分量走 java_fmt_f64/f32）。TestRecordPattern=既有 describe 转译 stub 非回归（归类：模式匹配用户类方法 stub，G-10 邻域待查） |
| ~~toString 桥接（scatter4 件 2，另一台机器）~~ | **✅ 完成（`e955e7d` 合入，主会话 3/3 复核）** | 根因精化：摘除机制只针对 inherent 形态手写，项目已有 `__impl_toString` 第二形态不触发摘除（Double/Integer 先例）——修法只补伴生手写体，**codegen/宏零改动**；**TestSequencedCollections 全绿**（mfirst=p=1/mlast=q=2）。附带情报：TestArrayCovariance 6 行 diff=既有（JArray getClass 协变视图+getSimpleName 数组形态，归数组 getClass 族） |
| ~~stub-hit 散点大礼包（约 10 处）~~ | **✅ 完成（`fix/stub-scatter-pack` 7 提交，+1704）** | **5 例全绿**（CollectionsUtil/LocalDate/FormatLocale/HexFormat/PrintStreamApi，主会话 7/7 复核含回归）；Unsafe 对象布局+实例字段原子族（ObjectVTable `__unsafe_long/int_cell` 钩子）、getCallerClass、getAdapter（德语数据）、US_ASCII 族、FloatToDecimal（**顺带修两处 Schubfach 移植 bug**：int 回绕缺失/float 平局括号，千例对拍零差异）。**下一层归类**：TestBigInteger=codegen 参数重绑定（ifnull 分支 astore 死局部）；TestCollectionFactory=`Class.getComponentType`（反射域）；TestAtomics=MemberName 元数据表（reflect 域）；TestSequencedCollections=手写 toString 桥接（emitter `_root_method_vtable_owner`） |
| ~~native 散点双件：availableProcessors + mismatch int 版~~ | **✅ 完成（`fix/native-scatter-2`）** | **TestArraysUtil 全绿**；TestLocalDate 卡点解除（NCPU 判定：仅作批次切分阈值不进输出，无需归一）。mismatch 语义核正：ArraysSupport 层恒 -1（「较小剩余长度」是公开 API Arrays.mismatch 行为，由翻译字节码承担） |
| **Unsafe 对象布局族散点（TestLocalDate 下一层）** | native 双件报告 | `Unsafe.arrayBaseOffset`（卡点：ConcurrentHashMap `<clinit>` ABASE 行）+ 同链排队 `arrayIndexScale`/`objectFieldOffset`——Unsafe 族按需补 |
| **native 双件：`CDS.getRandomSeedForDumping` + `StringUTF16.isBigEndian`** | 4.2 复验新增（8 例） | 两处 runtime 手写（数行级）：CDS 压 6 用例（ListOf/LinkedHash/CollectionFactory/StreamMore/AutoboxEdge/LambdaVar），isBigEndian 压 2（StringSearch/StringEdge）——收益密度最高 |
| ~~S-19 输出一致性缺陷群~~ | **✅ 全六件收官** | rint/NaN/expm1（`0078906`+`73a6c54`+`444ad9a`）+ UTF-16/科学计数（`16296bf`+`2c27f14`）+ 栈帧（`2e35b9e`，std::backtrace 帧数真实内容近似）；TestMathRound/TestNaN/TestFloatBits/TestStringCodePoints/TestMathExact/TestCustomException 全绿 |
| ~~downcast 链移除（859 处）~~ | **✅ 完成（2026-09-21，fix/downcast-chain-removal）** | 方案 §6 步骤 4：根方法根 vtable 直调 + 类虚方法 `__virtual_view` 部件重建分派（宏新增）+ SAM 回退保留；`downcast_ref` 方法体清零（TestStringBuilder 1104→6，余全 SAM）；红线/streams/金丝雀 18/18，7 例失败经基线对照确认既有；指纹不变、双种子归零（链顺序不确定性源消失）；`from_any` 残余归 A-4/A-5（清单见分支报告） |

## P1 · 功能缺口

| 任务 | 来源 | 说明 |
|------|------|------|
| record `hashCode` 恒为 `Ok(0)` | R5 遗留 | `toString`/`equals` 已真实化，`hashCode` 未实现（S-7） |
| ~~`monitorenter`/`monitorexit` 为 no-op~~ | **✅ 随 S-20 真实化（`4e6a2a2`）** | 监视器经 `monitor.rs` 侧表真实 acquire/release，单线程语义不变（可重入）；`[equiv-audit] monitor-mt` 已埋点观测。剩余：多线程调度语义随线程层立项 |
| 手写静态 native 不触发类初始化；带 default 方法的接口自身不初始化 | S-10 剩余 | 见 remaining-issues S-10 |
| ~~stub-hit 分流：包装类边界层（2 例）~~ | **✅ 随散点三件包（`9b58672`）** | Integer.valueOf [-128,127] 缓存保身份 + toString；TestOptional/TestRefKindsFull 双绿。T-4（包装类走生成）仍是终态方向，缓存实现为过渡 |
| JDK25 边界 stubs 遗留 | P-3 轮 | `DoubleToDecimal.split`（Formatter `%f/%e/%g`）、`FloatToDecimal`、`Random__nextInt_i_base`（E0432）——随 JDK25 用例按需补 |
| ~~等价告警基建 `[equiv-audit]`~~ | **✅ 完成（ruva 方案 ③ 落地）** | 9 ID 发射点计数 + runner `[equiv]` 汇总 + run 族失败自动分类（`[run-classify]`：stub-hit/native-hit/s8-crash/runtime-panic）+ `--deny equiv[::id]/stub-hit`；生成代码零变化实证；monitor-mt 待 S-20 合入后补埋（一处计数器） |
| e2e 差分补缺（等价探针） | ruva 吸收方案 ② | identityHashCode / finalize / 弱软虚引用 / Object.clone——166 实测零覆盖，探针用例随对应 S 条目修复排队 |

## P2 · 翻译质量

| 任务 | 来源 | 说明 |
|------|------|------|
| mut 标注递归覆盖 | T54 | 嵌套块 |
| 括号优化 | T60 | 表达式优先级 |
| 布尔压缩 | T70 | `if c {1i32} else {0i32}` → bool（R5 已修部分：lxor/比较产物） |
| 语义桩计数 | T72 | 指令级统一标记未做 |
| 类级并行解析 | T65 | 测试级并行已实现（run_tests.py），类级未做 |
| ~~生成输出非确定性~~ | **✅ 已修（`b6ab58d`，G-4）** | 两处源：vars.py 提升集合迭代 + downcast 链 registry 插入序（新发现的第二源）→ 均排序遍历；双种子生成树 diff 归零。附带 `[raw-audit]` 仪表落地（`90d2e93`） |
| catch 变量作用域（2 例） | 166 归类 | TestDateTimeFormat / TestZonedDateTime `E0425 ex`——catch 形参在后续引用点不可见，G-1/G-3 邻域 |
| 接口槽位成员缺失（2 例） | 166 归类 | TestStreamNumeric E0407 / TestPriorityQueue E0599——槽位沿继承层次的签名/成员解析，K-6/S-18 后续增量 |
| 一次性编译错（4 例） | 166 归类 | TestOverload 生成语法、TestStringSearch（escape 已修、转 run 族）、TestSwitchNull E0605（S-17 已归类 Integer/String 常量标签）、TestCollectorsMore E0061 |
| ~~用户类 import 生成缺口~~ | **✅ 随 A-8 消失（2026-09-22 核验）** | TestCustomException 干净树编译通过（Ubuntu 2224f02+ 与 macOS 双确认）——E0425 被 A-8 的 `collect_referenced` 字节码引用集驱动导入扩展顺带修复；**S-19 #5 栈帧 diff 现在直达**（仅 `has stack frames` 1 行） |
| ~~异常层次 upcast CCE（2 例）~~ | **✅ 随 macros 修（`548c08b`）** | 同上条 |
| IR 结构化收敛 | T05/T06/T07/T50/T58/T61/T67 | RawExpr/RawStmt 消除，架构级 |

## P3 · 长期重构（不阻塞主线）

> 详细设计见 `docs/plans/2026-09-17-java-rust-type-1to1.md` 与 `docs/plans/2026-09-18-erased-runtime-identity.md`。

| 任务 | 启动条件 | 说明 |
|------|---------|------|
| **T-1 bounds 进宏** | 无依赖 | `impl ArrayList<E>` 的 Rust bounds 由 `java_class!` 从 `generic_signature` 注入，codegen 只写裸参数名 |
| **T-2 接口泛型进类型位置**（擦除阶段 1 已落地：非泛型 `I__VTable` + 载体 struct + `ObjectVTable::__interface`） | T-1 完成后 | 剩余：接口名在类型位置仍擦除为 `Object`（调用点读 `Into::<Iterator<E>>::into(..).hasNext()` 而非 `it.hasNext()`）；lambda 对象实现 `I__VTable`；抽象类/枚举/手写类的 `__interface` 覆盖 |
| **T-4 包装类走生成** | T-3 完成后 | `Integer`/`Long` 等从字节码生成（String 已走生成） |
| **R-2 `Deref<Target=Parent>` 替换 From 继承链** | R-1 ✅ 已落地 | 删除 `class_writer.py` 的 `From<Child> for Parent` 链生成；Rust deref coercion 自动处理多层向上引用 |
| **R-3 `#[derive(Debug)]` 替换宏生成 Debug** | 无依赖 | 前提：生成类字段类型均满足 `Debug` |
| **M-3 方法签名类型决策进宏** | IR 结构化完成后 | 宏从 `#[descriptor]`/`#[generic_signature]` 自行做 JVM→Rust 类型选择，Python 只传原始字符串 |

---

## 维护规则

1. 完成一项 → 从本表删除，证据（commit / 测试结果）记入对应追踪文档
2. 新发现问题 → 入表并标注来源（e2e 普查 / 代码审查 / 实验发现）
3. 每 e2e 全量运行后刷新基线数字
4. 本文档不记历史——需要查完成记录去历史文档或 git log

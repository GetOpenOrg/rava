# 166 基线归类规程与首次归类（2026-09-21）

> 数据来源：用户 Ubuntu 全量 `run_tests.py --jdk 21`（166/166，**87 PASS / 79 FAIL**，224m25s，stdout-only 未落盘日志）；JDK25 跑批中断（仅前 30 用例）。
> 本机复核：fcb04ce 干净树定向复现 18 用例（清单见 §二）。
> 关联：等价承诺与边界判据 = [`docs/compatibility.md`](../compatibility.md)；条目承接 = `remaining-issues.md`（本轮新增 A-8 / S-19 / S-20，更新 S-8 / S-9 / V-1）；队列刷新 = `tasks.md`。

---

## 一、归类规程（durable）

### 1.1 错误族分类学

在 runner 现有四族（transpile / compile / run / output）之上细分：

| 族 | 判定 | 子族 |
|---|---|---|
| transpile | 转译器崩溃或 javac 拒绝 | `infra`（javac 层问题：预览特性、JDK 版本特性，非转译器缺陷） |
| compile | cargo E0xxx | `类型缺失`（E0433/E0425）｜`类型不匹配`（E0308）｜`成员缺失`（E0599/E0407）｜`cast`（E0605）｜`语法/转义`｜其他 |
| run | 二进制非零退出 | **`stub-hit`**（stderr 含 `stub: `）｜`npe-uncaught`（未捕获 NPE 逃逸顶层）｜`runtime-panic`（java_runtime 内部 panic）｜`codegen-panic`（生成代码 panic，如 capacity overflow） |
| output | stdout diff | `规格破坏`（数值/格式/编码违反 JLS）｜`边界偏差`｜`语义错`（求值顺序/类型渲染） |
| **equiv-boundary** | 命中 `compatibility.md` §3 特性表 | 引用表行，记「边界偏差」而非 bug |
| **java-fail / infra** | golden 生成失败 / JDK 版本依赖 | — |

注：`stub-hit` 与 `equiv-boundary` 当前**人工判定**（stderr 手抓、查表）；runner 自动化（run 失败自动抓 stderr 分类、`[equiv-audit]` 审计行、`--deny equiv`）为归类完成后的已排期事项。

### 1.2 判定流程

1. 失败 → 先查 `compatibility.md` §3：命中 → equiv-boundary（挂条目链接），不立项；
2. 未命中 → 按 §1.1 归族；
3. compile 族补全诊断：`cd build/<test_dir> && cargo build --bin <bin>`（runner 摘要只有一行）；
4. run 族补全证据：`./build/target/debug/<bin>` 抓 stderr——`stub: ` → stub-hit；NPE/panic 按子族；
5. 同根因 ≥3 用例 → 提升 remaining-issues 条目（现状/根因/终态/验收 四段）；单个 → 并入既有条目。

### 1.3 日志纪律（本轮教训）

- **全量跑批必须 `tee` 落盘**：本轮 224 分钟 stdout-only，38 个 run 族无 stderr，全部需要二次定向才能定性；
- 基线可信度先验证再归类：金丝雀（TestStringBuilder）+ 抽样失败在当前 HEAD 复现，**先排除陈旧树**；
- 全量只在架构节点合入后跑（执行约束 2）；日常按族 `--filter` 定向。

---

## 二、基线可信度：陈旧树污染（fcb04ce 复核）

用户跑批树落后于 main（fcb04ce）。本机定向复验（12 个失败代表 + 金丝雀）：

| 结果 | 用例 |
|---|---|
| **本机 PASS（陈旧树假象）** | TestStringBuilder（E0308→PASS）、TestOptionalFull（run error→PASS）、TestIncDec（r 值 diff→PASS）、TestShortCircuit（bool 渲染 diff→PASS） |
| **本机复现 FAIL（真缺陷）** | TestConstructorChain（E0433）、TestArrayCopy（E0308）、TestOptional（stub）、TestOptionalChain（NPE）、TestHashMapOps（panic）、TestArrayBounds（panic）、TestStringBuilderOps（stub）、TestMathRound（rint）、TestNaN（isNaN/nanEq）、TestStringCodePoints（UTF-16） |
| **用户基线失败、未复验** | 其余（含 TestMathExact、TestFloatBits、TestCustomException、TestStringCompare 等 output 族与全部 run 族待定向清单） |

**结论：79 个失败须先过「陈旧树筛」**——用户侧 `git pull` 到 fcb04ce 后按 §1.2 定向复验失败清单（`--filter` 分域，预计半小时级），真实失败面预计明显小于 79。本机已复现的条目以下按 fcb04ce 证据归类。

---

## 三、compile 族（31）→ 六个家族

### 3.1 辅助类未生成族（15）——**最大单一根因，已立项 A-8**

同 `.java` 文件里的非 public 顶层类 / 嵌套辅助类没有进入转译产物。**证据**：TestConstructorChain 的 scratch（`build/test_constructor_chain/user/src/`）只有 `main.rs` + `test_constructor_chain.rs`——`ShapeBase` 文件根本不存在（不是 mod 树漏挂，是没生成）。

| 用例 | 缺失类型 | | 用例 | 缺失类型 |
|---|---|---|---|---|
| TestBridgeMethod | Crate (E0425) | | TestInstanceOfChain | Puppy (E0425) |
| TestConstructorChain ✓复现 | ShapeBase (E0433) | | TestRecordAdvanced | PersonRecord (E0425) |
| TestFieldShadow | ShadowChild (E0425) | | TestSealed | Add (E0433) |
| TestMethodRefKinds | Person (E0433) | | TestVarContext | DataHolder (E0433) |
| TestEnumAdvanced | Level (E0433) | | TestInterfaceConflict | Impl (E0425) |
| TestInitOrder | DerivedInit (E0433) | | TestInterfacePrivate | SimplePipeline (E0433) |
| TestInnerClass | TestInnerClass (E0425) | | TestHashSetOps | SimpleKey (E0433) |
| TestGenericBoundsCombo | Registry (E0433) | | | |

E0425（cannot find value）与 E0433（cannot find type）同族，差别只在引用位置。根因方向：类装载/BFS 闭包只从 public 入口类出发，同文件兄弟类虽在常量池被引用（`new ShapeBase`）但未触发装载或未发射（与 S-15「BFS 对用户类不可见」同域，层级更深）。**一次修复预期清零 15 个用例。**

### 3.2 `Object.wait/notify/notifyAll` 缺失（4）——已立项 S-20

TestSynchronized、TestThreadJoin、TestVirtualThread、TestWaitNotify，统一报 `E0599: no method named wait found for struct object::Object`。grep 证实 `runtime/java_runtime/src/java/lang/object.rs`/`object_ext.rs` 无 `fn wait`/`fn notify`。runtime 补三个方法（接 InternalLock/Condvar 模型）即可一次修 4 个用例。

### 3.3 数组视图转换缺失族（JDK21: 2 + 待定性 2；JDK25 广泛命中）

**完整诊断证据**（TestArrayCopy，本机复现）：

```text
error[E0308]: `?` operator has incompatible types
  --> user/src/test_array_copy.rs:129
  let _t22: JArray<i32> = Arrays::copyOf_arr_i_i(Clone::clone(&matrix.get(0i32)?), 3i32)?;
                                                    expected `JArray<i32>`, found `Object`
  = note: `?` operator cannot convert from `java_runtime::Object` to `java_runtime::JArray<i32>`
```

数组实参从 `Object` 槽位流向类型化数组形参（`JArray<i32>`）时，`From<Object> for JArray<T>` 视图转换（R9 已在 runtime 提供）**未在实参位置发射**——codegen coerce 缺口。JDK21 命中：TestArrayCopy ✓复现、TestBigInteger；TestFilesApi、TestCompletableFuture 报 E0308 mismatched types，是否同根因待抓完整诊断。

**JDK25 关联**：用户 JDK25 前 30 用例中 8 个报同型 E0308 `?`（TestTernary / TestBridgeMethod / TestOverload / TestArrayList / TestCollections / TestStringBuilder / TestCustomException / TestLambda——JDK21 全 PASS），是否同一根因待 JDK25 定向复验；若是，则该族实为**当前最大修复杠杆**（wt-fix01 之后的新一代 JDK25 定向任务）。

### 3.4 catch 变量作用域（2）

TestDateTimeFormat、TestZonedDateTime：`error[E0425]: cannot find value ex`——catch 形参在后续引用点不可见（变量提升/作用域，G-1/G-3 邻域）。

### 3.5 接口槽位成员缺失（2）——K-6 后续

TestStreamNumeric（`E0407: trySplit is not a member of trait WhileOps_UnorderedWhileSpliterator_OfInt__VTable`）、TestPriorityQueue（`E0599: no method named poll found for reference &(dyn AbstractQueue__VTable)`）。接口槽位沿继承层次的签名/成员解析，K-6 槽位签名模型与 S-18 机制的后续增量。

### 3.6 一次性（4）

TestOverload（生成语法错误 `")={}""`）、TestStringSearch（`unknown character escape: d` 正则转义未翻倍）、TestSwitchNull（E0605——**S-17 已归类**「Integer/String 常量标签」，boundary 性质）、TestCollectorsMore（E0061 形参数量不匹配）。

---

## 四、run 族（38）→ 已定性 7 + 待定向 30 + 陈旧 1

### 4.1 已定性样本（stderr 实证）

| 用例 | stderr 证据 | 子族 | 承接 |
|---|---|---|---|
| TestStringBuilderOps ✓本机 | `stub: jdk/internal/math/DoubleToDecimal.appendTo` | stub-hit | P-3（已知） |
| TestStreamCollectors | JavaLangAccess.join stub（S-18 注记在案） | stub-hit | P-3（已知） |
| TestOptional ✓本机 | `stub: java/lang/Integer.valueOf:(I)`（integer.rs:229） | stub-hit | 包装类边界层（T-4/P-3 扩充） |
| TestOptionalChain ✓本机 | `of(null) -> NullPointerException` 打印后 NPE 逃逸顶层——catch 未捕获 | npe-uncaught | S-9 注记（NPE 可捕获性） |
| TestHashMapOps ✓本机 | `NullPointerException: 对 null 数组做批量读取`（array.rs:188） | runtime-panic | 待提升（HashMap 内部 null 处理） |
| TestArrayBounds ✓本机 | **`capacity overflow`（raw_vec）——负长度直通 Vec 分配，进程崩溃** | codegen-panic | S-8（现状升级，见条目） |
| TestInheritedMethod | K-6 后应 PASS，用户基线 run error | 待复验（疑陈旧树） | — |

### 4.2 run 族定向复验（30 例，2026-09-21 午后完成，main @ 28474b1 顺序跑 + 二进制 stderr 定性）

**陈旧树筛结论**：30 例中仅 TestMethodRef 转绿（第 6 个假失败实证，前 5 = TSB/TSBO/TestOptionalFull/TestIncDec/TestShortCircuit）；TestMethodRefKinds 经子串误命中带入，属 A-8 辅助类族。**26 个 run 错误全部定性**：

| 族 | 用例 | 数量 | stderr 证据 / 根因 | 承接 |
|---|---|---|---|---|
| **`CDS.getRandomSeedForDumping` native 缺失** | TestListOf / TestLinkedHash / TestCollectionFactory / TestStreamMore / TestAutoboxEdge / TestLambdaVar | **6** | `native: jdk/internal/misc/CDS.getRandomSeedForDumping:()J` | runtime 手写一处（收益密度最高） |
| **数组视图 CCE（运行期）** | TestArrayCovariance / TestBigDecimal / TestDurationPeriod / TestLocalDate | **4** | `ClassCastException: java/lang/Object cannot be cast to JArray<JArray<BigInteger>>`（返回值/多维位置 `Object`→`JArray<T>` 未发射） | **数组视图 coerce 族**——与 §3.3 编译期 2 例（TestArrayCopy E0308 实证）同根因候选，族规模 2→6，JDK25 或 +8 |
| `StringUTF16.isBigEndian` native 缺失 | TestStringSearch / TestStringEdge | 2 | `native: java/lang/StringUTF16.isBigEndian:()Z` | runtime 一行（`cfg!(target_endian)`） |
| 反射族 `getDeclaredField` | TestCollectionsUtil / TestRandomSeed | 2 | `stub: java/lang/Class.getDeclaredField` | 反射族（静态注册表终态方向已定） |
| 异常层次 CCE | TestNestedTry / TestSuppressed | 2 | `IllegalStateException cannot be cast to RuntimeException`、`RuntimeException cannot be cast to Exception` | 新归类：异常类继承 upcast 转换缺失 |
| 散点 stub（边界按需） | TestArraysUtil（`ArraysSupport.mismatch([I[II)` int 版）/ TestPrintStreamApi（`FloatToDecimal.toString`——P-3 已知遗留）/ TestAtomics（`Unsafe.getAndAddInt`）/ TestSequencedCollections（`NullableKeyValueHolder.<init>`）/ TestFormatLocale（`LocaleProviderAdapter.getAdapter`）/ TestHexFormat（`US_ASCII.INSTANCE` 静态字段） | 6 | 各自 `stub: ` | P-3 清单扩充 |
| NPE/异常逃逸 | TestTreeMapSet（toString NPE 变 panic 逃逸）/ TestMapIteration（`UnsupportedOperationException: remove` 逃逸顶层，Java 侧应可捕获） | 2 | panic / `Exception in thread` | S-9 邻域（可捕获性） |
| enum 反射 | TestEnumSetMap | 1 | `ClassCastException: java/lang/Class not an enum` | 反射族邻域 |
| **用户类方法被 stub（异常信号）** | TestRecordPattern | 1 | `stub: TestRecordPattern.describe:(Ljava/lang/Object;)Ljava/lang/String;` | **待查**：用户类方法不应被 stub，疑 record/合成方法生成路径跳过（G-10 同型：私有合成实例方法曾被跳过） |

**compile 3 例**：TestMethodRefKinds（E0433 Person → A-8 族）、TestStringNewMethods（E0599 `exactOutputSizeIfKnown` 于 Object → 擦除接收者成员解析族，与 TestStreamNumeric 同型）、TestPecs（E0432 unresolved import `java::lang::Appendable` → 新形态，接口/mod 树邻域，原基线为 run error——树更新后失败形态已变）。

**队列影响**：数组视图 coerce 族升级为 6 例（编译 2 + 运行 4）并列第二大杠杆；`CDS.getRandomSeedForDumping` + `isBigEndian` 两个 native 合计 8 例、runtime 数行——快速收益包。

---

## 五、output 族（9）→ 规格破坏 5 + 边界 1 + 陈旧 2 + 待复验 1

| 用例 | diff 证据 | 归类 | 承接 |
|---|---|---|---|
| TestMathRound ✓复现 | `rint=2.0,4.0`→`3.0,4.0`：**Math.rint 应 HALF_EVEN**（rint(2.5)=2.0），当前 half-away | 规格破坏 | S-19 |
| TestNaN ✓复现 | `nanEq=false`→`true`（**JLS 15.21.1：NaN≠NaN**）；`isNaN=true`→`false` | 规格破坏 | S-19 |
| TestStringCodePoints ✓复现 | `length=4`→`20`、`codePointAt1=128512`→`239`：**增补字符字面量未走 UTF-16 表示**（compact strings 的 Latin1 分支吞掉代理对） | 规格破坏 | S-19 |
| TestFloatBits ✓复现（2026-09-21 午后） | `isNaN=true`→`false`（与 TestNaN isNaN 同根） | 规格破坏 | S-19 |
| TestMathExact ✓复现（2026-09-21 午后） | `ulp=2.220446049250313E-16`→`0.000…313`（科学计数规则缺失）＋**新增第二处**：`expm1=1.718281828459045`→`…453`（尾数值不同——expm1 算法精度或最短表示格式化，并入 S-19） | 规格破坏 | S-19（java_fmt_f64 / expm1 native） |
| TestStringCompare ✓复现（2026-09-21 午后） | `interned==lit=true`→`false`：intern 同一性 | **equiv-boundary**（compatibility.md §3 已标） | S-6 |
| TestCustomException **改判 compile 族**（2026-09-21 午后） | `E0425: cannot find type PrintStream / Throwable_PrintStreamOrWriter / ObjectInputStream / Class`——用户内部类 `SubFineException` 的签名/体内 JDK 类型**未生成 import**（7 处）；疑 S-18 扩大闭包后方法体入译暴露的 import 生成缺口。原「栈帧未填充」症状被挡在编译错后面，修完 import 后需回到该症状验证 S-19 #5 | 规格破坏＋import 生成缺口（新） | S-19 #5 ＋ S-18 后续增量 |
| TestIncDec / TestShortCircuit | r 值 / bool 渲染 diff | **陈旧树假象**（本机 PASS） | 无需立项 |

## 六、transpile 族（1）

TestUnnamedVar：javac 21 拒绝 `_` 未命名变量（预览特性，JDK 22 定稿）→ **infra**，非转译器缺陷。处理：该用例标记 JDK25-only，或 runner 支持 `--enable-preview`；用户 JDK25 批次中 javac 层应可通过。

## 七、JDK25 部分数据附注

前 30 用例 10 失败：8×E0308 `?`（见 §3.3）+ E0432 `Random__nextInt_i_base`（JDK25 的 Random 内部结构变化，边界类未跟上）+ E0433/E0425（与 JDK21 辅助类族重合）。数据不全；补全后按本规程归类。**补跑务必 tee 落盘。**

## 八、下一波任务建议（按投入产出排序）

1. **陈旧树筛**：用户 pull 后按 §4.2 清单定向复验（半小时级），把 run/output 族的真实失败面定下来；
2. **A-8 辅助类族**（15 用例）：证据已足（§3.1），是最大单一杠杆；
3. **S-20 Object.wait/notify**（4 用例）：runtime 小改；
4. **3.3 数组视图族**（JDK21 2 个 + JDK25 可能 8 个）：若 JDK25 同根因，收益最大；
5. **S-19 规格破坏五件**（rint/NaN/isNaN/ulp/UTF-16 字面量）：math 与 string native 补全；
6. stub-hit 分流（TestOptional 的 Integer.valueOf 等）并入 P-3 清单；S-8 现状升级已记。

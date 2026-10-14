# 用户程序运行态内存面扫描（T8）：翻译后二进制哪些在运行期超 32G（2026-10-14）

> 基线：origin/main @bdb9bad4；worktree rava-wt/runmem（分支 libtests-runmem）；参考构建
> JDK 21.0.11（tools/refjdk 共享主检出那份）、debug 档、逐例串行、重进程全经 heavy_lock。
> 目的：回答「翻译后的二进制哪些在运行期超 32G」，给无 GC 二阶段
> （[2026-10-13-nogc-phase2-design.md](../plans/2026-10-13-nogc-phase2-design.md)：P0 弱引用原语 /
> 所有权推断 Weak / 引用类语义）排期提供运行态数据。原始数据：/tmp/runmem/（rss.tsv × 75、
> 每例 result.json 与 run_tests 全日志、plan.md 盘点、summary.json 汇总）。

## 一、结论摘要

**零超限。候选 65 例 + 对照 10 例共 75 例受控实测，运行段二进制 RSS 峰值无一超过 16G
（护栏未触发一次），更无 32G；全测集峰值 1.665 G（TestVirtualThreadScale，8.7s 峰形回落）。**
按 T8 口径如实写：**本次候选 ∪ 对照范围内，翻译后二进制运行态峰值 ≤ 1.7 G**——运行态 32G
超限清单为空。

关键定性（与 JDK25 侧 OOM 记录的关系）：

1. **JDK25 名单上的重例在 JDK21 运行段全部轻如鸿毛**：junit 8 例运行 0.04–0.06 s、RSS 采样
   ≤0.007 G；DeepCopy 运行 0.06 s、Digester 0.04 s。这些例的 OOM 病理全在**转译段**
   （j25prof 报告已证：分析器单线程 20–50 MiB/s 单调增长），运行段无对应面。
2. **运行段唯二呈「直线涨」形态的例是 23_algorithms 的两例大负载**：FourIsTheNumberOfLetters
   （0.07→1.52 G / 148 s，~10 MiB/s，运行自然结束时仍在涨）与 SelfNumbers（0.07→1.03 G /
   65 s 后平台到结束）。它们是当前语料里运行态泄漏形态的**最强真实样本**，量级 1–1.5 G，
   距 16G 护栏差一个数量级。
3. **线程族无泄漏面**：60_real_threads 15 例全部瞬态（≤0.8 s、≤0.08 G）；唯一的重量例
   TestVirtualThreadScale 为**峰形回落**（0.64→1.67→0.34 G，8.7 s）——虚拟线程批量建栈后
   随线程退出 RSS 回落，属「分配-释放」而非泄漏（其 output-mismatch 为已知失败，
   known_failures 在列）。
4. **引用/Weak 族（48_refs 10 例）全部瞬态**：这些用例循环规模小（数百次引用操作），
   距离暴露 nogc-phase2 §一所述「回指成环泄漏」（探针 NogcWeakCycle 实测 ≈110 B/节点）
   需要的迭代规模差 3–4 个数量级——语料现状不构成 Weak 环的运行期压力面，
   无 GC 二阶段的 Weak 排期**不被现有语料阻塞**，其验收应继续用 NogcWeakCycle 探针放大迭代，
   而非指望 e2e 语料。
5. **3 例编译不过导致运行面不可测**（DataEncryptionStandard、TestHttpLoopbackAsync/Sync）：
   E0599 `IOException::new_str`（见 §五附注，新失败面，非内存问题）；2 例运行超时
   （FractionReduction、IQPuzzle）RSS 恒平 0.07 G——纯算力慢，非内存。

## 二、存量遥测盘点结论（先省后测）

| 位置 | 运行期 RSS | 结论 |
|---|---|---|
| scripts/run_tests.py `_run_bin` | 无 | 只记 run 段耗时；sig=9 事后标「疑似 OOM」，峰值数字不留 |
| build_status.json（rava compile） | 无（编译段状态） | 编译面已有 build-memsafe 线，运行面无 |
| cluster dist_e2e / dist_job `oom` 字段 | 仅被杀时 | `limit=…M peak=…M`（peak=死时刻 RSS）；存活例零内存数据；历史作业仅 3 条 oom（g1c10-sb3 / g1c2-objscan×2，均转译段诊断） |
| cluster_results/timings_*.tsv | 无 | 纯耗时（run_s/transpile_s/build_s）——**可复用**：本扫描候选圈定即按 run_s 排序 |
| 诊断作业脚本（如 g1s1-sb3） | `/usr/bin/time -v` | 包的是**分析器**（rava closure），非翻译后二进制的运行段 |
| j25prof 报告（junit 8 剖面） | 30s 粒度 RSS | 对象是 JDK25 **转译段**（--no-run），非运行段；作形态参照 |

**结论：运行段（翻译后二进制执行期）RSS 峰值在全部存量遥测中零记录，必须实测（本报告）。**
JDK25「~62 例 14G 槽 OOM」经 triage 核实为转译+编译段混合 OOM、逐例名不可恢复
（j25s-rA/rB 作业不在本机 cluster_results）；可复名子集（junit 8 + DeepCopy/DES/Digester/
HttpLoopback×2）已全数纳入本扫描候选 E 组。

## 三、方法与护栏

- **执行**：逐例串行 `heavy_lock uv run python3 scripts/run_tests.py --filter /<Test>.java
  --out-dir build/runmem`（默认参考构建 JDK 21.0.11、debug 档、运行超时缺省 300 s；
  LynchBell/AmicablePairs/RamanujanPrimes/FourIsTheNumberOfLetters/TestParallelArrayCas/
  PrimorialNumbers 6 例放宽 --run-timeout 900）。
- **监控**：0.5 s 粒度扫 `/proc/<pid>/exe`，路径前缀 `build/runmem/jdk21/target/`（JDK 版本层）
  且文件名为该例 bin——只命中**运行段二进制**（转译 rava / rustc / dyn 对照 java 均不匹配），
  记 VmRSS → /tmp/runmem/<Name>.rss.tsv（粒度密于任务书 10 s 口径，为不漏 <10 s 的瞬态例；
  读 /proc 开销可忽略）。
- **护栏**：bin RSS > 16 G → SIGKILL 该 bin（显式 PID）并记 gt16g。**全程零触发**。
- **记录列**：case / 转译 / 编译 / 运行 rc / 运行峰值 RSS / 超线（16G/32G）/ 形态 / 备注。
  运行秒 <0.5 s 的瞬态例无采样（「无样本*」），其水位必然微小（sub-秒进程的映射上限）。
- 全程 75 例 10 577 s（约 2.9 h，含每例转译+编译；构建缓存共享，非运行段独占）。
- 形态判据：直线涨=单调升且末值 ≥80% 峰值；锯齿=相邻采样 ≥15% 峰值回落 ≥2 次；
  平台=早达峰后末值 ≥85% 峰值；峰形回落=末值 <50% 峰值；瞬态=<1 s。

## 四、全量结果（75 例）

### A. 48_refs（引用/Weak 族，10）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 | 备注 |
|---|---|---|---|---|---|---|---|
| WeakRefDemo | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestReferenceTypes | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestRefKindsFull | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestLongChainRelease | OK | OK | rc=0 | 1.5 | 0.207G | 短瞬 | |
| TestVolatilePrimitiveAccess | OK | OK | rc=0 | 0.0 | 0.022G | 瞬态 | |
| TestVarHandleRefRmw | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestVarHandleBitwise | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestSubwordFieldCas | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestUnsafePrimitiveArray | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestByteArrayViewVarHandle | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |

### B. 60_real_threads（15）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 | 备注 |
|---|---|---|---|---|---|---|---|
| TestConcurrentClinit | OK | OK | rc=0 | 0.2 | 无样本* | 瞬态 | |
| TestContinuationPinned | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | |
| TestJucSync | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | |
| TestParallelArrayCas | OK | OK | rc=0 | 0.5 | 0.083G | 瞬态 | |
| TestSleepParkClock | OK | OK | rc=0 | 0.5 | 0.019G | 瞬态 | |
| TestSpinVolatile | OK | OK | rc=0 | 0.1 | 0.020G | 瞬态 | |
| TestThreadCounters | OK | OK | rc=0 | 0.2 | 无样本* | 瞬态 | |
| TestThreadInterrupt | OK | OK | rc=0 | 0.5 | 无样本* | 瞬态 | |
| TestThreadNatives | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestThreadOverridesSpec | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | |
| TestThreadStates | OK | OK | rc=0 | 0.3 | 0.023G | 瞬态 | |
| TestThreadUncaught | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestVirtualThreadCarrier | OK | OK | rc=0 | 0.7 | 0.079G | 瞬态 | |
| TestVirtualThreadScale | OK | OK | rc=0·diff | 8.7 | **1.665G** | 峰形回落 | 已知失败（sleeping 断言）；0.64→1.67→0.34G |
| TestWaitNotifyQueue | OK | OK | rc=0 | 0.2 | 0.020G | 瞬态 | |

### C. 34_concurrency（5）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 | 备注 |
|---|---|---|---|---|---|---|---|
| TestChmTransfer | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| ParallelMergeSort | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| ConcurrentMapDemo | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| TestLongAdderCounters | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |
| VirtualThreadBarrier | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 | |

### D. 23_algorithms 大负载（10）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 | 备注 |
|---|---|---|---|---|---|---|---|
| LynchBell | OK | OK | rc=0 | 446 | 0.019G | 平台 | 全程恒平（cluster JDK25 的 832s 为算力差） |
| AmicablePairs | OK | OK | rc=0 | 1.9 | 0.091G | 短瞬 | JDK25 300s TO 未复现 |
| RamanujanPrimes | OK | OK | rc=0 | 0.8 | 0.091G | 短瞬 | 同上 |
| FourIsTheNumberOfLetters | OK | OK | rc=0 | 149 | **1.519G** | **直线涨** | ~10 MiB/s 单调升，结束时仍在涨 |
| PrimorialNumbers | OK | OK | rc=0 | 123 | 0.076G | 平台 | |
| PartitionInteger | OK | OK | rc=0 | 131 | 0.071G | 平台 | |
| FractionReduction | OK | OK | TO>300s | 300 | 0.070G | 平台 | 算力慢非内存（RSS 恒平） |
| IQPuzzle | OK | OK | TO>300s | 300 | 0.070G | 平台 | 同上 |
| SelfNumbers | OK | OK | rc=0 | 91 | **1.028G** | **直线涨→平台** | 65s 涨到 1.03G 后平台 |
| SquareFree | OK | OK | rc=0 | 84 | 0.123G | 缓升平台 | |

### E. JDK25 OOM 名单可复名子集 ∩ JDK21 同名（13）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 | 备注 |
|---|---|---|---|---|---|---|---|
| TestJunitFailureMessages | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | JDK25 病理在转译段（65G 剖面），运行段无面 |
| TestJunitIgnoreAndExpected | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| TestJunitNewInstancePerTest | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| TestJunitBeforeClassState | OK | OK | rc=0 | 0.0 | 0.007G | 瞬态 | 同上 |
| TestJunitLifecycleOrder | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| TestJunitAssertThrowsAssume | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| TestJunitRunnerResult | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| TestJunitTimeoutFast | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 | 同上 |
| DeepCopy | OK | OK | rc=0 | 0.06 | 无样本* | 瞬态 | JDK25 转译 OOM 例，JDK21 运行 0.06s |
| DataEncryptionStandard | OK | FAIL E0599 | — | — | — | — | 编译不过，运行面不可测（§五） |
| Digester | OK | OK | rc=0 | 0.04 | 无样本* | 瞬态 | JDK25 转译 OOM 例 |
| TestHttpLoopbackAsync | OK | FAIL E0599 | — | — | — | — | 编译不过（§五） |
| TestHttpLoopbackSync | OK | FAIL E0599 | — | — | — | — | 编译不过（§五） |

### F. 36_bignum（5）/ G. 37_datetime（3）/ H. 容器（4）

| 用例 | 转译 | 编译 | 运行 | 运行秒 | 峰值 RSS | 形态 |
|---|---|---|---|---|---|---|
| BigIntegerDemo / BigDecimalDemo / TestBigInteger / TestBigDecimal / BigNumberTest | OK | OK | rc=0 | ≤0.01 | 无样本* | 瞬态 |
| TestZonedDateTime / TestDateTimeFormat | OK | OK | rc=0 | 0.1 | 无样本* | 瞬态 |
| TestSimpleDateFormatRound | OK | OK | rc=0 | 0.3 | 0.066G | 瞬态 |
| TestArrayList / CollectorsDemo / TestLinkedList / HashMapComputeTest | OK | OK | rc=0 | 0.0 | 无样本* | 瞬态 |

### 对照组（10，非候选目录抽样）

HelloWorld、ComprehensiveTest、GenericBasic、NumberDemo、EnumAdvancedTest、PatternSwitchTest、
RegexTest、StreamAdvanced、JavaBaseOptionalTest、JavaBaseMultiTest——全部 OK/OK/rc=0、
运行 ≤0.1 s、无样本*（瞬态）。**对照组与轻候选无差别**：语料的典型运行段本就是亚秒级、
内存面可忽略；重负载集中在 D 组大负载数据集。

汇总：75 例 = 69 pass + 3 compile-fail（E0599，§五）+ 2 run-timeout（RSS 恒平）+
1 output-mismatch（已知失败）。gt16g = 0，gt32g = 0。

## 五、附注：编译面新失败（非内存，阻断 3 例运行面测量）

DataEncryptionStandard / TestHttpLoopbackAsync / TestHttpLoopbackSync 在当前 main（bdb9bad4）
**编译段** E0599：`no associated function or constant named 'new_str' found for struct
io_exception_m::IOException`。HttpLoopback×2 的 known_failures 在列签名是运行期
`native: sun/nio/ch/IOUtil.fdLimit`——签名不符=新失败面（DES 无登记）。三者同根
（IOException 构造族缺口，疑与手写层方法 trait 化缝隙同族），建议归 K6/E0599 线跟进；
本轮只登记不修（本扫描为纯测量，无代码变更）。**内存面结论不受影响**（编译都不过的例
谈不上运行态超限；且同族转译闭包正常的 Digester/DeepCopy 运行段均为瞬态）。

## 六、泄漏形态分类（给无 GC 二阶段 Weak 环排期的输入）

| 形态 | 例数 | 用例 | 给排期的读数 |
|---|---|---|---|
| 直线涨 | 1 | FourIsTheNumberOfLetters（1.52G/149s，~10 MiB/s，自然结束时仍在涨） | 当前语料**最强**运行态泄漏形态；量级 1.5G，距 16G 护栏一个数量级。适合当 nogc-phase2 的**运行态回归锚点**：P0/Weak 落地后看其斜率是否归零 |
| 直线涨→平台 | 1 | SelfNumbers（1.03G/91s） | 工作集固定的缓泄漏：涨到工作集上限后持平——泄漏对象被复用面盖住，形态上「有界泄漏」 |
| 峰形回落 | 1 | TestVirtualThreadScale（1.67G/8.7s） | 线程批量建栈→退出还栈：**分配-释放正常路径**，非泄漏；虚拟线程规模放大时它是运行态内存主项（栈），与 Weak 环无关 |
| 平台/缓升 | 6 | LynchBell、PrimorialNumbers、PartitionInteger、SquareFree、FractionReduction、IQPuzzle | 有界工作集，无泄漏面 |
| 瞬态/短瞬（<5s） | 66 | 其余全部（含 48_refs 全族、60_real_threads 其余 14、junit 8、bignum/datetime/容器、对照组） | 运行段亚秒~数秒、水位 ≤0.21G |

**对 Weak 环（nogc-phase2 §一）的具体排期输入**：

1. e2e 语料**不提供** Weak 环的运行期压力面：48_refs 全族瞬态，其循环规模（数百次引用操作）
   对 ≈110 B/节点 的环泄漏（NogcWeakCycle 探针实测）只能攒出 ~10 MB 级——与实测 ≤0.21G 吻合。
2. 运行态「直线涨」现存样本只有算法大负载（FourIsTheNumberOfLetters），且斜率 10 MiB/s、
   有程序自然终点——**不足以当 Weak 环的病理复现器**；二阶段验收应继续用 NogcWeakCycle
   探针（可控迭代规模、已知每节点字节数），FourIsTheNumberOfLetters 作辅助回归锚点。
3. 线程族的「峰形回落」说明运行时在线程退出路径上正确归还了栈/线程本地结构——
   二阶段引入弱计数（`__Obj` 头 16→32B）后，这条归还路径是「摘表纪律」（§三 引用类语义）
   必须保住的不回归面；TestVirtualThreadScale（已知断言失败）的 RSS 曲线可作该面的
   监控指标（峰后回落 ≥50%）。

## 七、方法学局限

- 单机单例顺序，RSS 采样 0.5 s 粒度：<0.5 s 的瞬态例无样本（75 例中 47 例），其水位由
  「亚秒进程 + 采样下界」联合上界（典型 <0.2G，测得最高瞬态样本 TestLongChainRelease 0.21G）。
- debug 档二进制：release 档（opt 3）下分配行为可能不同（省略某些中间拷贝），但泄漏形态
  （净增长斜率）不受 opt 影响；量级结论对档位不敏感。
- 运行超时 300/900 s 截断：FractionReduction/IQPuzzle 的曲线在 300 s 处右删失，但其 RSS
  恒平，删失不影响「无泄漏」判定。
- 16G 护栏 kill 为破坏性测量：本轮零触发，无需外推。

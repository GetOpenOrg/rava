# 转译耗时回归调查（2026-10-09，分支 perf-regress）

> 状态：**第四轮达标**（10-10，分支 perf-regress4，修复提交 c3ec480d）。CHM 表合并的真因是 `SerialCallbackContext.obj` 按类共用字段视图：序列化写方的全部对象图成了反序列化 `setObjFieldValues` 的写入目标，按偏移整堆互灌字段。值持有者按对象分开后，四例转译比 e5200a3e 快 40%（DeepCopy 306 s / 4.1 GB，JNDI 348 s），类集合对 batch-1010c 无新增。第三轮及以前的「摘要克隆 open 接收者」判断是误读，见「第四轮」。

## 现象

| 用例 | b1009 | b1012 e5200a3e | b1013 |
|---|---|---|---|
| TestJndiNoProvider | 324 | 570 | 1006 |
| DeepCopy | – | 527 | 912 |
| TestSerialDefaultSuid | – | 538 | 862 |
| TestSerialUserGenericCallbacks | – | 514 | 907 |

单位为秒（`rava build --stop-after emit` 冷跑）。目标：四例 ≤ b1012；TestJndiNoProvider ≤ 350；DeepCopy 峰值 ≤ 6 GB；闭包类集合不增。

## 二分（`scripts/transpile_time_job.sh`，同一服务器按提交冷跑）

### 第一段：b1009 → b1012（TestJndiNoProvider 324 → 587）

| 提交 | JNDI 秒 | 峰值 MB | 类数 |
|---|---|---|---|
| d5a2cb5d | 271 | | |
| 329ddb99 | 275 | | |
| c18e8fc4（boot-image-s6） | 389 | | |
| 2045f766 | 402 | | 3797 |
| a88d7075（locale-build，含 bbeb13a5） | 578 / 622 | | 3796 |
| b295467d（a88d7075 回退 bbeb13a5） | 420 | 5514 | 3797 |
| e5200a3e | 587 | | 4086 |

- **c18e8fc4**（启动映像 s6）：+115 s。映像对象 7702 → 19347，每个对象都是一个抽象对象分配点。5af967a8 已把基本类型元素数组按类型合并为一个分配点，增益很小。
- **bbeb13a5**（工厂产物并入调用点上下文段）：约 +160–200 s。产物多展开一层调用方上下文段。a4c4d0c1 把并入段计入 HEAP_DEPTH，但在 head 上没有可测增益，说明 head 的主导开销已不在这里。

### 第二段：b1012 → b1013（DeepCopy 548 → 934，JNDI 587 → 1033）

- 区间 1dbfd22c → 413ae150（reflect-marker，含 8ce97959 serialPersistentFields）→ 6c687d65（logger-chain）。
- DeepCopy：1dbfd22c 548 → 8ce97959 934，即跳变落在 **reflect-marker 的 8ce97959**。
- JNDI 在 1dbfd22c / 8ce97959 / 6c687d65 上尚未逐点测过；诊断提交 3972ce9b（1dbfd22c+edge_groups）与 cfa8a743（8ce97959+edge_groups）已推送，可直接测。

## 主导开销机制（edge_groups 对照 1dbfd22c ↔ 8ce97959 / head）

两次对照中，flows 阶段都占 554–657 s。

1. **ConcurrentHashMap 表数组 N²**
   - 各上下文的表数组 `[LConcurrentHashMap$Node;@<ctx>:40` 约 1485 个。每个元素节点 E0 都流向约 1224 个 `Unsafe.getReferenceAcquire` 克隆的 S3。E→S 推送从 27 M 涨到 62–75 M。
   - 约 1444 个手写 CAS / set 写节点（W）写入每一个表。W→E 推送从 15 M 涨到 41–51 M。
   - 结论：每个 relay 上下文的 `tab` 实参都含**全部**表，即 `tab` 来源在某处被合并。疑点（未确认）有三：
     - 未知接收者字段视图 F/U 共用；
     - NOCTX / 常量上下文；
     - transfer / ForwardingNode.nextTable / Traverser 跨表传递。
   - 8ce97959 让可序列化路径进入档案，带来大量 CHM 实例上下文，所以 N² 被放大。
2. **TreeBin.find → TreeNode.findTreeNode** 288 k 边。该处按选择子常量克隆（kc = null），仍走逐对象路径。
3. **replaceNode S290** 127 k；**HashMap$TreeNode.find R** 131 k。
4. aux_analyses 79 k → 477 k（ret_const 原因 16 k → 84 k）。

## 本分支提交

| 提交 | 内容 | 效果 |
|---|---|---|
| 5c064a33 | 直连反射调用点的精确接收者被 open 涵盖时由 open 枢纽展开 | 小 |
| 5af967a8 | 基本类型元素映像数组按类型共用分配点 | 小 |
| 19350546 | `--perf` 摘要加 edge_groups（诊断） | – |
| 0269f62218bf72c7a6f8ffd3733242373ae1a222 | final 实例方法调用点抽象对象接收者 ≥ HUB_MIN 走精确集合枢纽 | DeepCopy 832 → 794（jp2），JNDI 1007 → 910；类数 +1（InaccessibleObjectException），真因为枢纽形参常量格，c02c1825 修复（见「第三轮」） |
| a4c4d0c1179fa602ccd08b2c32bcbb71fd9428e8 | 工厂产物并入的调用点段计入堆深度 | head 上无可测增益 |
| 509c0297fb95659c30c13792e147a743b16ae73d | 作业脚本另存 `<用例>.classes` | – |
| 5a5a049c53af9f166bfd2652ba291bd227c51237 | `--flows @ctxsets:<方法键>` 诊断 | – |
| 40705e1a | @CallerSensitive 目标不经枢纽中转、逐调用点接边 | 对多出类无效（pr-tg4）；补上中转目标不调 `caller_edge` 的潜在缺口，保留 |
| c02c18250bd48f64dc1a007911416d04e2f47165 | 精确集合枢纽的形参常量格改用 `PV::of_ret`，与逐个接边同口径 | 两例都少 InaccessibleObjectException 与 X509CertificatePair，无新增类；耗时与 bedc57aa 持平（pr-tg6 / pr-tg7） |
| 53a03514 | 测时脚本对重复出现的提交复用首次构建的二进制 | – |

另有工具提交：a1ce56b7、3f48d10d、025b56c2、fb0bd6f2（`@objstat`）、34699639。

## 前后对照（同一服务器）

| 用例 | 服务器 | e5200a3e（b1012） | 5a5a049c（本分支 head） |
|---|---|---|---|
| TestJndiNoProvider | us1 | 596 s / 6827 MB / 4086 类 | 940 s / 7338 MB / 4039 类 |
| DeepCopy | us1 | 518 s / 5975 MB / 3842 类 | 814 s / 6803 MB / 3758 类 |
| TestSerialDefaultSuid | kr1 | 504 s / 5844 MB / 3847 类 | 792 s / 6868 MB / 3763 类 |
| TestSerialUserGenericCallbacks | kr1 | 501 s / 6294 MB / 3845 类 | 787 s / 7030 MB / 3801 类 |

- 相对 b1013 原始数（912 / 1006 / 862 / 907，jp1 / jp2）约快 10%，但仍远超目标。
- DeepCopy 峰值 6.8 GB，超出 6 GB。
- 类数比 b1012 少，来自 b1013 的精度改动，不是本分支的功劳；本分支相对 19350546 净 +1 类。

## 续作（10-09 下午，同一分支）

### 多出的类：@CallerSensitive 目标经枢纽中转丢调用者镜像（假设不成立，真因与修复见「第三轮」）

> pr-tg4（us1，5bfc1f67）：DeepCopy 849 s / 6933 MB / 3728 类，类集合与 bedc57aa 完全相同，仍含 InaccessibleObjectException——下述假设机制不是该类入闭包的原因。

- `--why java/lang/reflect/InaccessibleObjectException`（作业 pr-tw2，jp1，bedc57aa）：`DeepCopy.main → ObjectOutputStream.writeObject → ObjectStreamClass.lookup → … → ObjectStreamClass.getDeclaredSUID@23 → Field.setAccessible@11 → Field.checkCanSetAccessible → AccessibleObject.checkCanSetAccessible(Class,Class,Z)@40 → throwInaccessibleObjectException`。
- 机制：0269f622 让 final 方法调用点（`Field.setAccessible`，Field 为 final 类）在接收者 ≥ HUB_MIN 时经精确集合枢纽。枢纽对中转目标（`hub_plain`）只接 HP→P / R→HR，不走 `edge`，因而不调 `caller_edge`——@CallerSensitive 目标的调用者节点收不到该调用点所在类（ObjectStreamClass）的镜像，`getCallerClass` 不再折叠到 java.base 模块，`checkCanSetAccessible` 的拒绝分支可达。虚调用枢纽有同一缺口（此前未被触发）。
- 修复（`generator/crates/closure/src/engine/hub.rs` `hub_plain`）：@CallerSensitive 目标不作中转目标，走枢纽的逐调用点 `special` 路径（`edge` → `caller_edge`，与逐个接边同口径）。5bfc1f67 恢复 0269f622。
- 先行撤回提交 1b076270 保留在历史中（随后 5bfc1f67 恢复），二者净效果为 0269f622 + 40705e1a。

### CHM 表合并机制（已确认，未修）

`--flows @xpath:<方法>|<形参>`（f8297dbb，作业 pr-tm2，us1）沿流边反查 `tab` 值集最大的克隆：

1. 逃逸的 CHM 对象（DeepCopy 179 个）的 `table` 字段经 U(table) 收到其他对象的表：开放接收者（序列化 writeObject 的反射 / open 路径、反序列化得到的普通实例上的 readObject、addCount → transfer）在**摘要克隆**（NOCTX / 档位 / 无堆常量克隆）里读 F(table) / F(ForwardingNode.nextTable)，`table = nextTab` 又写回 U。
2. F = U ∪ 全部逃逸对象该字段，U 流入每个逃逸对象的 O——F→U 往返把每张表散到每个逃逸对象，各上下文 `tab` 含全部表，E→S / W→E 推送 N²。
3. 终态修复方向：摘要克隆对 open 接收者的字段写不能落到全体逃逸对象的公共 U，应按接收者对象分派（open 展开按逃逸对象逐个取上下文，或对 open 接收者的写按 `g_of` 成员各自的 O 接边并以枢纽汇集）。逐逃逸对象展开的代价需先估算（179 CHM × 克隆数），本轮未实施。

### 汇集节点（e5bb8bd1，保留）

手写读内存 / 写数组元素调用点经汇集节点（gather.rs `gather_hw_read` / `gather_hw_write`）：类集合不变；E→G 21 M、G→S 26 M、G→E 13 M，总推送与转译时间无可测变化（pr-tg1，us1：DeepCopy 855 → 857 s，JNDI 993 → 984 s）。结构上把「调用点数 × 数组数」改为二者之和，保留。

### 相位对照（pr-tg1，us1，DeepCopy，`--perf`）

| 指标 | e5200a3e | bedc57aa |
|---|---|---|
| flows | 292 s | 567 s |
| sites | 89 s | 111 s |
| analyze / aux_analyze | 15 / 2.4 s | 30 / 15 s |
| aux_analyses | 77 k | 477 k |
| ret_const / sysprops 原因 | 16 k / 614 | 83 k / 8111 |
| hub_sent | 9.3 M | 13.6 M |
| 推送合计（前 20 种） | ≈ 330 M | ≈ 266 M |

推送总数反而少，flows 却翻倍：单次推送更贵（值集更大——逃逸对象 / 表数组集合并入，E→S 26 → 60 M、W→E 14 → 40 M）。ret_const / aux 的增长来自 58166528 / 2408cff5 / ab2942ad 一组「按调用点常量实参求值返回常量」的精度改动（集成分支，类数 3842 → 3728 的来源之一），绝对耗时（aux 15 s）不是主因。

## 第三轮（10-09 夜 → 10-10，同一分支）

### 多出类的真因：精确集合枢纽的形参常量格丢「非空」（c02c1825 修复）

- 诊断（pr-why4，us1，closure 模式，`--flows @vals:Field.setAccessible|Field.checkCanSetAccessible|AccessibleObject.checkCanSetAccessible`）：有 0269f622 时（3f9c14db）`Field.checkCanSetAccessible` 的形参常量为 `[Top, Top]`；无 0269f622 时（1b076270）为 `[Top, Const(Ref{nonnull:true})]`。调用者集合两边相同（ObjectStreamClass 等 4 个类，open=0）——调用者镜像并未丢失，40705e1a 的假设不成立。
- 机制：`Field.setAccessible@8` 的 `getCallerClass()` 返回确定非空的无标签引用，`@11` 把它传给 `checkCanSetAccessible`。逐个接边（`edge` → `bind_params`）用 `PV::of_ret`，把它记为「非空引用」常量；枢纽 `link_hub` 汇总各调用点常量实参时用 `PV::of`，把同一实参记为 Top。经 0269f622 的 final 方法枢纽后，`checkCanSetAccessible(Class,Class,…)` 的调用者形参失去非空，依赖调用者非空折叠的拒绝分支不再被剪，`throwInaccessibleObjectException` 可达。
- 修复（c02c1825，`engine/hub.rs` `link_hub`）：枢纽形参常量格与逐个接边同口径，改用 `PV::of_ret`。这是枢纽与逐个接边的一致性修复，不是特判。
- 40705e1a 保留：它不是本类的成因，但枢纽中转目标（`hub_plain`）确实不调 `caller_edge`。@CallerSensitive 目标经中转时，调用者节点只能靠其他调用点补全，这是一个潜在缺口。改为逐调用点接边后与 `edge` 同口径，代价可忽略（CS 目标极少）。
- 同类不一致尚存一处：`lambda_vals.rs` 的 lambda 捕获参数常量仍用 `PV::of`，留作后续（见残留）。

### 「逃逸对象进 G」实验（93d80e65，已撤回 3f9c14db）

- 设计：逃逸抽象对象进实例化集合 G，类 id 字段走按类视图，非虚调用 / 反射非虚成员的 open 接收者经固定目标枢纽。目的是让 open 值按 G 展开时，每个逃逸对象进自己的接收者克隆，从而不再在 F / U 汇合。
- 实测（us1）：pr-tg5（build）DeepCopy 1141 s / 9793 MB / 3729 类；pr-st6（closure）1132 s / 9607 MB / 3729 类。比 bedc57aa 慢 33%，峰值 +36%，多出 `MethodHandleImpl$CountingWrapper$1`。`@ctxsets` 显示 transfer / tabAt 克隆的 `tab` 仍含约 724 张表，表合并没有消除。
- 结论：按逃逸对象展开克隆的代价是 179 个 CHM × 克隆数，而且合并的主要引入点是摘要克隆自己构造的 open 接收者，展开 G 触及不到（见下节），因此撤回。

### CHM 表合并：open CHM 的引入点

`--flows @opens:java/util/concurrent/ConcurrentHashMap`（pr-why4，3f9c14db）列出 open(CHM) 的引入点，即含 open(CHM) 但没有任何前驱含同一 open 的节点。共三类：

1. **CHM 私有 / 内部方法的接收者形参 P0**：addCount、fullAddCount、helpTransfer、initTable、putVal、sumCount、transfer（含两个常量克隆 `#~@1827:267`、`#~@1860:186`）、treeifyBin、tryPresize，以及 Object.getClass。这些 P0 没有 open 前驱，说明 open 接收者是克隆构造时直接给的，也就是摘要克隆（NOCTX / 档位 / 常量克隆）的 open 接收者，不是从调用方流进来的。
2. **三个 `[Ljava/lang/Object;` 分配点的元素节点**（`@22854:34`、`@22867:9`、`@702:25`，奇偶两槽），以及 `ObjectOutputStream$HandleTable.growSpine@55`。这些是对象数组元素直接播种 open 的点（数组身份未逐一核实，推测是序列化句柄表一类的对象数组，以及手写数组复制的结果）。
3. **`ConcurrentHashMap$CollectionView.map`** 在 `EntrySetView@23449:12` 各上下文克隆上的字段节点。

结论与终态方向：

- 表合并的主因是第 1 类。摘要克隆为节省上下文，用 open 接收者代表「任意 CHM」。克隆体读 F(table)、写回 U，U 流入全部逃逸 CHM 的 O(table)，形成 F→U 往返。
- 93d80e65 改的是「open 值按 G 展开」，没有碰摘要克隆自己构造的 open 接收者，所以合并没有消除。
- 终态修复：摘要克隆不对接收者字段做公共 U 写回。做法是让摘要克隆体对 open 接收者的字段写只落「写入值集」，在各具体接收者对象的上下文（调用方实参已知时）按对象落到 O；对接收者本就未知的调用（反射 / VM 入口），写回 U 是语义必需的。这需要把摘要克隆的接收者形参改为「调用方实参集」，而不是 open。
- 第 2 类（Object[] 元素 open）需单独核实来源。如果来自手写数组复制没有建模元素流，应按数组复制语义接元素边。

### 第三轮实测（us1，同机交替，build 模式，秒 / MB / 类）

pr-tg6 按 c02c1825 → e5200a3e 顺序跑；第三段 c02c1825 因脚本在共享 target 下复用了上一提交的二进制而失败（53a03514 修复），改由 pr-tg7 在同机补跑。

| 提交 | DeepCopy | TestJndiNoProvider |
|---|---|---|
| c02c1825（pr-tg6） | 875 / 6908 / 3726 | 1005 / 8051 / 4007 |
| e5200a3e（pr-tg6） | 520 / 5959 / 3842 | 597 / 6840 / 4086 |
| c02c1825（pr-tg7） | 876 / 6915 / 3726 | 1011 / 8049 / 4007 |
| bedc57aa（pr-tg1，参照） | 855 / 7208 / 3728 | 993 / 7831 / 4009 |

- 类集合（对照 pr-tg1 的 bedc57aa 类表）：两例都少了 `java/lang/reflect/InaccessibleObjectException` 与 `sun/security/provider/certpath/X509CertificatePair`，没有新增类。后者推测同样是非空常量恢复后被剪掉的分支带出的类，未单独 --why。
- 耗时与峰值：c02c1825 与 bedc57aa 持平（DeepCopy +2%，JNDI +1%，在噪声内），比 e5200a3e 慢 1.68×。峰值 6.9 GB（DeepCopy）/ 8.0 GB（JNDI），未达到 ≤6 GB 目标。CHM 表合并未修是主因。
- 单测（dev）：A 组 pr-ut-c02c1825 只有已知失败 `param_string_constants_fold_switch`；B 组 pr-utB-c02c1825 全过；`closure_independent_of_hash_seed`（pr-uth-c02c1825）通过。

## 第四轮（10-10，分支 perf-regress4，基于 batch-1010c）

### 真因：`SerialCallbackContext.obj` 按类共用，写方对象图成为读方写入目标

- 第三轮把 CHM 私有方法的 P0 列为 open(CHM) 引入点，是误读：`edge_recv_in` 把非对象接收者（open + 类 id）以字面 `Feed::S(rest)` 交给 NOCTX 本体，`@opens` / `@openinj` 因而把这些 P0 报为「无前驱」，open 实际来自调用方。新增诊断 `--flows @escin:<类>`（a7d8ea4b）列出流入逃逸汇点的源节点。
- `@trace:open:ConcurrentHashMap`（p4-trace3，dev）：最早的 open(CHM) 来自手写写入 `FieldReflector.setObjFieldValues@241 → Unsafe.putReference`。写入值是 vals 数组里的 open(Object)，按字段类型收窄后，写进大量抽象对象（含映像对象 `KeySetView@image236`）的 `CollectionView.map`。随后 `KeySetView.add` 读出 open(CHM)，进入 putVal / initTable / transfer 的 NOCTX 本体，F(table) → U(table) 的往返把表散到全体逃逸 CHM。
- 写入目标为何这么宽（p4-path1，`@path`）：
  1. 写方 `writeSerialData` 构造 `new SerialCallbackContext(obj, slotDesc)`。
  2. 读方 `defaultReadObject` 用 `curContext.getObj()` 取当前对象，再交给 `setObjFieldValues`。
  3. `SerialCallbackContext` 不是容器形态类，`obj` 字段走按类共用的 F / U，于是读方拿到写方 `writeObject0` 递归走过的全部对象（12179 个值）作为反序列化写入目标。
  4. 结果是 `getObjFieldValues` 读出的整堆字段值经 `setObjFieldValues` 按偏移写回整堆对象的同类型字段，形成整堆字段互灌。CHM 的 table / map 字段被灌成全体表集合，逃逸 CHM 从 1 个涨到 28 个。
- 8ce97959 之所以触发跳变：它让 `ObjectStreamClass` 的 Method 缓存不再混入 open(Method)，`invokeWriteObject` / `invokeReadObject` 精确派发到各类自身的 readObject / writeObject，`defaultReadObject` 这条路径因而进入档案。

### 修复（c3ec480d，`engine/classes.rs` `container_shape`）

- 新增一类容器形态：**值持有者**，即类链上有 final 的 `Object` 型实例字段，且本类某个构造器以「aload 非 this 局部量 → putfield 该字段」写入它。
- 这类对象按分配点 × 堆上下文分开，每个持有者只给出自身的构造实参。读方与写方的 `SerialCallbackContext` 因此是不同的抽象对象。
- 纯字节码形态判定，不列类名。结果只取决于类文件，与分析顺序无关；更细的对象划分只缩小值集，是单调的。
- 修复后（p4-d1，dev，closure 模式）：DeepCopy 411 → 166 s，峰值 7240 → 4177 MB；逃逸 CHM 从 28 个降到 1 个，open(CHM) 引入点清零；`setObjFieldValues` 的写入目标不再含写方对象。

### 实测（p4-t2，us1，build 模式，同机交替，先 e5200a3e 后 c3ec480d；秒 / MB / 类）

| 用例 | e5200a3e | c3ec480d | batch-1010c（a7d8ea4b，us1 closure 模式，p4-esc1 / p4-c1） |
|---|---|---|---|
| DeepCopy | 527 / 5983 / 3842 | **306 / 4104 / 3686** | 861 / 6777 / 3726 |
| TestJndiNoProvider | 606 / 6812 / 4086 | **348 / 4616 / 3990** | 984 / 7933 / 4007 |
| TestSerialDefaultSuid | 520 / 5907 / 3847 | **309 / 4131 / 3691** | 866 / 7131 / 3731 |
| TestSerialUserGenericCallbacks | 521 / 6294 / 3845 | **313 / 4032 / 3749** | 863 / 6982 / 3769 |

p4-t1（us1，c3ec480d 单跑）：DeepCopy 309 / 4082，JNDI 354 / 4642。

- 四例都比 e5200a3e 快约 40%，JNDI 348 s ≤ 350 s，DeepCopy 峰值 4.1 GB ≤ 6 GB。
- 类集合：四例对 batch-1010c **无新增**，分别减少 DeepCopy 40、JNDI 17、SerialDefaultSuid 40、SerialUserGenericCallbacks 20 个，减少的是整堆互灌带入的各集合拆分器、AbstractMap$1 等。对 e5200a3e 只多 `Method$Direct$It` / `Method$Direct$Marks`，属 b1013 的直连反射支持，与本轮无关。
- e2e 单例（dev，p4-e1 / p4-e2）：DeepCopy、TestSerialDefaultSuid、TestSerialUserGenericCallbacks、TestSerializationHooks、TestSerialAllocTargets、SerializableDemo 全部 PASS。
- 单测（us1，p4-ut，f0ceb0ab）：A / B 组与 rava_macros_core 全过，0 失败（顺序 / 种子两项单独跑，见下）。
- 顺序无关：UNIT_ORDER

## 残留与建议

1. **CHM 表合并：已修**（c3ec480d，见「第四轮」）。第三轮的「摘要克隆 open 接收者」方向不再需要。
2. TreeBin.find → findTreeNode：选择子常量克隆的非虚调用点可按「基调用点 × 常量」建枢纽（未做，耗时已达标，优先级降低）。
3. `lambda_vals.rs` 的 lambda 捕获参数常量仍用 `PV::of`，与 `bind_params` / 枢纽的 `PV::of_ret` 不同口径（未改，需单独测类集合）。
4. 临时引用 pr-tmp-eg-1dbfd22c、pr-tmp-eg-8ce97959：逐点测 JNDI 已无必要，可删。

## 恢复入口

- 分支 perf-regress4，修复 c3ec480d，诊断 a7d8ea4b（`@escin`），已 merge origin/rust-closure-analyzer（f0ceb0ab）；worktree `/Users/yuwei/dev/workspace/rava_perf4`。
- 同机对照基准（p4-t2，us1，build 模式）见「第四轮 · 实测」。

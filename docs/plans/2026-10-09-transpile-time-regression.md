# 转译耗时回归调查（2026-10-09，分支 perf-regress）

> 状态：**未完成，到时收尾**（第二轮 6 h 上限）。四例转译仍比 b1012 慢约 1.6×，耗时目标未达到。0269f622 多出的类未修（40705e1a 无效，见 pr-tg4）；CHM 表合并机制已确认，终态修复未实施。恢复入口见文末。

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
| 0269f62218bf72c7a6f8ffd3733242373ae1a222 | final 实例方法调用点抽象对象接收者 ≥ HUB_MIN 走精确集合枢纽 | DeepCopy 832 → 794（jp2），JNDI 1007 → 910；类数 +1（InaccessibleObjectException），未修，见 pr-tg4（40705e1a 无效） |
| a4c4d0c1179fa602ccd08b2c32bcbb71fd9428e8 | 工厂产物并入的调用点段计入堆深度 | head 上无可测增益 |
| 509c0297fb95659c30c13792e147a743b16ae73d | 作业脚本另存 `<用例>.classes` | – |
| 5a5a049c53af9f166bfd2652ba291bd227c51237 | `--flows @ctxsets:<方法键>` 诊断 | – |

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

### 多出的类：@CallerSensitive 目标经枢纽中转丢调用者镜像（未修，见 pr-tg4）

> pr-tg4（us1，5bfc1f67）：DeepCopy 849 s / 6933 MB / 3728 类，类集合与 bedc57aa 完全相同，仍含 InaccessibleObjectException——下述假设机制不成立，40705e1a 无效。

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

## 残留与建议

1. **CHM 表合并（最高优先，未修）**：机制见「续作 · CHM 表合并机制」。终态方向：摘要克隆对 open 接收者的字段写按接收者对象分派，不落公共 U；先用 `@objstat` / `@xpath` 估算逐逃逸对象展开的克隆规模再实施。不截断入口、不关精度。
2. **40705e1a 的验证**：作业 pr-tg3（jp1，DeepCopy + JNDI）与 pr-tg4（us1，DeepCopy）已投 5bfc1f67，收尾时两台服务器都被其他作业占用、尚在排队，结果落 `cluster_results/job/pr-tg3|pr-tg4/01/build/ttime/`。验收：`DeepCopy.classes` / `TestJndiNoProvider.classes` 与 pr-tg1 的 bedc57aa 类表相比不含 InaccessibleObjectException、无新增类；us1 上 DeepCopy 与 pr-tg1 的 bedc57aa 855 s / 7208 MB 对照。闭包单测（`closure_independent_of_hash_seed` 等）未跑，须在服务器上补跑。
3. TreeBin.find → findTreeNode：选择子常量克隆的非虚调用点可按「基调用点 × 常量」建枢纽（未做）。
4. 在 3972ce9b（pr-tmp-eg-1dbfd22c）/ cfa8a743（pr-tmp-eg-8ce97959）/ 6c687d65 上逐点测 JNDI（未做）。
5. 序列化两例（TestSerialDefaultSuid / TestSerialUserGenericCallbacks）尚未在 us1 / jp1 上与 e5200a3e 同机对照。
6. 临时引用：pr-tmp-r78、pr-tmp-rbb、pr-tmp-os-*、pr-tmp-egfix、pr-tmp-bb1、pr-tmp-bb2 与 worktree rava_prtmp 已删；**保留** pr-tmp-eg-1dbfd22c、pr-tmp-eg-8ce97959（第 4 项要用，origin 与 github 均在）。

## 恢复入口

- 分支 perf-regress，代码 head 5bfc1f67（集成分支 c249cdec 已 merge 进来，即 bedc57aa）；worktree `/Users/yuwei/dev/workspace/rava_perfregress`。
- 先取第 2 项结果。之后做第 1 项：在一台空闲服务器上跑
  `TTIME_MODE=closure bash scripts/transpile_time_job.sh --extra '--flows @objstat --flows @ctxsets:ConcurrentHashMap.transfer --flows @ctxsets:ConcurrentHashMap.tabAt' tests/e2e/23_algorithms/DeepCopy.java <head>`（`--extra` 内不得含 `$`，含 `|` 须加引号；经 `distribute_tests.py --job <tag> --fetch 'build/ttime/**'` 下发）。
- 同机对照基准（pr-tg1，us1，build 模式，秒 / MB / 类）：e5200a3e DeepCopy 522 / 5959 / 3842、JNDI 600 / 6810 / 4086；bedc57aa DeepCopy 855 / 7208 / 3728、JNDI 993 / 7831 / 4009。

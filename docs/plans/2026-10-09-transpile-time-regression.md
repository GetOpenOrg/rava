# 转译耗时回归调查（2026-10-09，分支 perf-regress）

> 状态：**未完成，到时收尾**（6 h 上限）。四例转译仍比 b1012 慢约 1.55–1.6×，目标都没达到；已定位主导开销，但还没有终态修复。恢复入口见文末。

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
| 0269f62218bf72c7a6f8ffd3733242373ae1a222 | final 实例方法调用点抽象对象接收者 ≥ HUB_MIN 走精确集合枢纽 | DeepCopy 832 → 794（jp2），JNDI 1007 → 910；**类数 +1（3757 → 3758 / 4038 → 4039），违反「类集合不增」，多出的类未查明** |
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

## 残留与建议

1. **CHM 表合并（最高优先）**：查清 `tab` 为何在各上下文都含全部表，按机制修复，不截断入口、不关精度。可能的方向有两个：
   - 让 F/U 视图与 relay 上下文按对象分离；
   - 让手写 CAS 写节点（W）按接收数组对象建边，不对全部表做笛卡尔积。
2. **0269f622 的 +1 类**：先对 19350546 跑 `.classes`（旧作业没有该产物），与 5a5a049c 的类表 diff。若无法消除，则回退 0269f622，或把枢纽只限于不影响可达性的情形。**合批前必须解决。**
3. TreeBin.find → findTreeNode：选择子常量克隆的非虚调用点可按「基调用点 × 常量」建枢纽。
4. 在 3972ce9b / cfa8a743 / 6c687d65 上逐点测 JNDI，确认 b1013 跳变同样来自 8ce97959。
5. 收尾待办：删临时引用（origin 与 github 都要删）：
   - pr-tmp-r78、pr-tmp-rbb
   - pr-tmp-os-{329ddb99,c18e8fc4,1dbfd22c,a88d7075}
   - pr-tmp-eg-1dbfd22c、pr-tmp-eg-8ce97959、pr-tmp-egfix
   - pr-tmp-bb1、pr-tmp-bb2

   以及本机 worktree `/Users/yuwei/dev/workspace/rava_prtmp`。这几个引用续查还会用到，所以本次未删。

## 恢复入口

- 分支 perf-regress，head 为本文所在提交（代码 head 5a5a049c）；worktree `/Users/yuwei/dev/workspace/rava_perfregress`。
- 下一步：在一台空闲服务器上跑 CHM 诊断，命令如下（`--extra` 内不得含 `$`，脚本 eval 在 `set -u` 下）：

  ```bash
  TTIME_MODE=closure bash scripts/transpile_time_job.sh --extra '--flows @ctxsets:ConcurrentHashMap.tabAt --flows @ctxsets:Unsafe.getReferenceAcquire --flows @ctxsets:ConcurrentHashMap.transfer --flows @ctxsets:TreeNode.findTreeNode --flows @ctxsets:ConcurrentHashMap.putVal --flows @ctxsets:ConcurrentHashMap.initTable' tests/e2e/23_algorithms/DeepCopy.java 5a5a049c53af9f166bfd2652ba291bd227c51237 19350546b874e0265d35904f1d52239e4e5ceeee
  ```

  - 经 `distribute_tests.py --job <tag> --fetch 'build/ttime/**'` 下发。
  - 该命令同时产出两提交的 `DeepCopy.classes`，可用于 +1 类的 diff。
  - 本次 pr-tk1 已投 jp1，但排队中被我停掉，未跑。

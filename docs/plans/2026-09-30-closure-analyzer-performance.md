# 闭包分析器性能、内存与工程化（rava closure）

> 日期：2026-09-30
> 上级计划：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md)（§七 终态指标「闭包计算耗时 ≤ 3s」只按 HelloWorld 定义，本文扩展到全量语料并补内存、健壮性指标）
> 状态（2026-09-30 深夜）：P0 ✅、P1 ✅（按数据改为图节点驻留，见 §4.4）、P2 ✅（保序常数优化，4 例 closure.json 逐字节一致；DeepCopy user 351 s → 130 s、RSS 4.1 GB → 2.3–2.8 GB），已合入 rust-closure-analyzer。结构性改造（手写调用点数组扇出经共享节点 / 按调用图 SCC 排序）会改变 `via` 与条目顺序，用户已同意把不变量放宽为「集合一致、`via` / 顺序可变」（2026-09-30，见 §二），结构性改造与 P3 起由 `closure-perf2` 推进。精度二期已合入（a6b4c6d5）。优化方向总纲见 [`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md)。

---

## 一、实测（2026-09-30，JDK 21，`rust-closure-analyzer` 3c3c3129 的 release 二进制，`/usr/bin/time -l`）

| 用例 | 墙钟 | user / sys | 峰值 RSS | 类 / 方法 | 方法上下文 |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 0.43 s | 0.25 / 0.07 s | 75 MB | — | — |
| Digester | 11.2 s | 9.0 / 1.3 s | 1.1 GB | 1446 / 8684 | 27 424 |
| DeepCopy | 173.6 s | 150.0 / 16.8 s | 5.6 GB | 1758 / 10 520 | 40 107 |

**结论**：DeepCopy 的上下文只比 Digester 多 1.5 倍，耗时多 16 倍、内存多 5 倍——**超线性失控在算法层**（重分析次数 / 值集膨胀），不是常数开销；sys 16.8 s 说明分配与缺页压力大。
（c731a473 修正后 DeepCopy 类降到 1677，但耗时仍在 125–175 s 量级，性能问题独立存在。）

### 1.1 已有的优化（不重复做）

- 类型流图：`TypeSet` 用 `IdSet`（有序 Vec，超过 64 元素附位图索引），**差分传播**（`engine/flow.rs` 只传 delta）；
- 整数键内部表已用 FxHash（`engine/sets.rs`）；
- release：`lto = "fat"`、`codegen-units = 1`、`debug = 1`（可直接做剖析）。

### 1.2 可疑热点（待 P0 剖析确认，不预设结论）

1. **方法整体重分析**：字段值集变化（`field_put`）、字段放开（`open_field`）、乐观阶段收尾（`never` 全体）都会让读者方法**整方法**重跑抽象解释（`invalidate` → `push_m` → `process`），在字段读者多、值集反复增长时呈级联放大；
2. **`MemberRef` 为三个 `String` 的键**：`facts.rs` 等 13 张表以它为键、走标准库 SipHash，另有大量 `key.clone()`——每次查表都哈希三段字符串、每次克隆都分配；
3. **上下文克隆的抽象状态不共享**：4 万上下文 × 每上下文独立的节点值集与 `Analysis`，平均约 140 KB / 上下文；
4. **`Analysis` 常驻**：方法摘要产出后中间状态（逐指令帧）仍保留。

## 二、终态（量化）

| 指标 | 现状 | 终态 |
|---|---:|---:|
| e2e 语料任一测试闭包分析墙钟 | 最坏 174 s | **≤ 10 s**（HelloWorld ≤ 0.5 s） |
| e2e 语料任一测试峰值 RSS | 最坏 5.6 GB | **≤ 1 GB** |
| 跨测试缓存命中时单测试墙钟 | 无缓存 | **≤ 2 s** |
| 超预算时被系统杀进程 | 可能 | **0**（预算内降级或带诊断退出） |
| 输出确定性 | 双种子脚本人工跑 | 双种子一致纳入 CI 必过 |
| 库代码 `unwrap` / `expect` / `panic!` | 24 处（classfile / resolve / closure） | **0**（`Result` + 上下文） |
| 分析规则单元测试 | 3 个 crate 合计 13 个 `#[test]` | 每条规则 ≥ 1 个；27 例闭包快照测试 |
| 清单（TOML）未知字段 | 静默忽略 | schema 校验，未知字段报错 |
| `closure` crate 单文件行数 | `engine.rs` 1067 行 | ≤ ~600 行 |

**不变量**（2026-09-30 用户决策修订）：性能步骤只要求**最终集合结果一致**——类集、方法集及其余集合型输出（dispatch / folds / facts 条目内容）与改前相同；`via` 与条目顺序可以变化，但顺序不得影响正常测试（生成器若依赖条目顺序，由分析器按键排序输出，生成树对照不变）；输出须确定（双种子一致）。P0–P2 实际达到逐字节一致。改变精度形态的步骤（P5 降级）动态对照翻译域漏覆盖保持 0、默认预算下输出不变。

## 三、步骤

| 步 | 内容 | 验收 |
|---|---|---|
| **P0 观测** | ① summary 增加分阶段耗时、峰值内存、每方法重分析次数 Top-N 与失效原因计数（字段写 / 字段放开 / 乐观收尾 / 摘要变化）；② `samply` 剖析 DeepCopy / Digester，结论落 §四；③ `scripts/closure_bench.sh`：基准集（HelloWorld、Digester、DeepCopy、CollectorsDemo + 验收集 27 例）串行跑，记录墙钟 / RSS / 上下文数，产出对照表 | 热点定位写回本文；基准脚本可复跑 |
| **P1 成员驻留** | `MemberRef`（三段 `String`）在引擎边界驻留为 `MethodId` / `FieldId`（`u32`），`facts.rs` 等表改为 Fx 表或按 id 索引的 `Vec`；消除热路径 `clone` | 输出逐字节一致；基准墙钟与 RSS 记录 |
| **P2 增量重分析** | 按 P0 数据选定：失效按「方法内依赖该字段 / 返回值的站点」精确重跑，替代整方法重分析；同一轮内的失效合并后一次处理；工作队列按调用图 SCC 逆拓扑序 | 输出逐字节一致；DeepCopy 重分析总次数与墙钟下降到 Digester 同量级（按上下文数线性） |
| **P3 上下文共享** | 入口抽象状态相同的上下文合并复用摘要；`TypeSet` / `PV` 哈希驻留（相同值集共享一份）；上下文克隆只在容器 / 工厂 / 转发等需要处发生（现有判定收拢为单一策略点） | 输出逐字节一致（若合并规则改变形态：动态对照漏覆盖 0、类集不增） |
| **P4 内存** | 方法摘要产出后释放逐指令帧（需要时重算）；阶段性 arena；评估 mimalloc 作为全局分配器 | 基准集峰值 RSS ≤ 1 GB |
| **P5 预算与降级** | `--max-mem <MB>` / `--time-budget <s>`：计数型全局分配器；逼近上限时剩余方法降为上下文不敏感（结果仍健全，只是更粗），summary 标记 `degraded` 与触发点；硬上限处带诊断退出（非系统 OOM）。`rava build` 缺省预算按机器内存推导 | 人为设低预算：输出 ⊇ 无预算输出、动态对照漏覆盖 0、进程不被杀 |
| **P6 跨测试缓存** | 缓存键 =（jmod 集合哈希、`runtime/` 清单哈希、分析器版本）；第一层缓存已解码类与方法 CFG；第二层缓存 JDK 方法摘要，用户类增量分析；缓存目录在 `build/`，损坏即重建 | 命中时输出与冷启动逐字节一致；命中墙钟 ≤ 2 s |
| **P7 并行** | 类解码 / CFG 预构建并行；不动点主循环保持单线程（确定性优先） | 输出逐字节一致；双种子一致 |
| **P8 工程化** | `unwrap` / `panic` 清零；规则单测补齐；27 例闭包快照测试（变化须审查）；清单 schema 校验；`engine.rs` 等超长文件拆分；classfile 解析器 `cargo-fuzz`；双种子确定性与 Linux / macOS 双平台纳入 CI | §二 对应指标达成 |

顺序：P0 必须先做（后续取舍依赖数据）；P1 → P2 → P3 → P4 按收益；P5 在 P4 之后（先把常态压到预算内，降级只兜底）；P6 / P7 / P8 可在 P2 之后穿插。

## 四、剖析结论

> 2026-09-30，`closure-perf` 分支（自 b3ccb0e9），release 二进制。观测字段在 closure.json `summary.perf`（不属于分析结果，`scripts/closure_bench.sh --diff` 对照时剔除）。
> DeepCopy 两次均在主会话 e2e 批次并行时跑，内存压力下墙钟 454–652 s（计划 §一 无压力为 174 s），**绝对秒数偏大，比例可信**。

### 4.1 分阶段耗时（ms，自耗时，`summary.perf.phases_ms`）

| 用例 | 上下文 | 分析次数 | flows（类型流传播） | sites（调用点重跑） | process（事件应用） | analyze（absint） | setup |
|---|---:|---:|---:|---:|---:|---:|---:|
| HelloWorld | 882 | 1 681 | 2 | 2 | 62 | 61 | 44 |
| CollectorsDemo | 20 380 | 38 651 | 1 159 | 844 | 981 | 1 053 | 163 |
| Digester | 24 442 | 41 271 | 1 890 | 1 348 | 1 166 | 1 007 | 221 |
| DeepCopy | 39 640 | 60 282 | 352 k–443 k | 68 k–151 k | 23 k–32 k | **1.6 k–3.6 k** | — |

### 4.2 结论

1. **§1.2 假设 1（整方法重分析是热点）被否定**：DeepCopy 抽象解释总计 < 1 % 墙钟；分析次数 60 282 / 上下文 39 640 = 1.5 倍，与 Digester（1.7 倍）同量级，没有级联放大。失效原因（Digester）：ret_const 6 728、never 4 060（其中 3 925 次事件不变）、sysprops 2 555（2 553 次不变）、param_const 1 482、field_put 1 137——多数重分析是「事件不变」的空转，但成本本身小。
2. **§1.2 假设 2（`MemberRef` 字符串键）收益可忽略**：`sample` 采样中 MemberRef 哈希 / 克隆 < 1 % 样本。
3. **超线性在类型流图**：DeepCopy 流边 24.66 M（Digester 2.17 M，11 倍），类型节点 25 万；`add_to` 调用 **7 804 761 719 次，其中仅 23 430 375 次（0.3 %）使目标增长**——99.7 % 是空传播。边种类分布：
   - `E→W` 10.9 M、`W→E` 9.3 M（合计 82 %）：手写调用点数组写（`engine/hw_mem.rs::hw_site_arrays`）把每个逃逸值 E 与每个手写站点数组槽 W 两两相连，二部图全连接；单个 `W(site,1)` 入度 6 201；
   - `O→S` 1.48 M、`E→S` 628 k、`S→O` 496 k、`S→HP` 403 k、`HR→S` 311 k；
   - 入度 Top：`ObjectStreamClass$FieldReflector.getObjFieldValues` 的 S74（13 785）、`Esc`（7 041）；出度 Top：数组总节点、`SpinedBuffer.accept` P1 的多个上下文、LambdaForm `Name` 节点（各 5 458）。
4. **sites 次之**：DeepCopy 调用点重跑 254 565 次（Digester 156 981），每次重跑按整个接收者值集重新枚举目标。
5. `sample` 热点（DeepCopy）：`run` 内 drain 循环自耗时、`add_to`、`IdSet::minus / insert / union_with` 与 memmove、`sets.entry` 的 Node 哈希 / 比较、`flow_seen` 插入、`ClassPath::get` 与 resolve 层标准库 SipHash 表。

### 4.3 对后续步骤的约束

- **输出依赖处理顺序**：closure.json 的 `classes` / `methods` 按首次发现顺序（IndexMap 插入序）并带 `via`，上下文标签内嵌方法序号（`@10772:0`）。凡改变处理顺序的结构性改造（E↔W 改汇聚节点、调用图 SCC 逆拓扑序、失效合并批处理）都会改变 `via` / 顺序，与「逐字节一致」不变量冲突。P1 / P2 只做**保序**优化；结构性改造（E↔W 汇聚节点化、SCC 排序）需要把不变量放宽为「集合一致 + via 可变」，由决策方拍板后作为独立步骤。
- 据此 P1 调整为「**图节点驻留**」（Node → 稠密 `u32`，`sets` / `fdelta` / `flows` 改 `Vec` 索引、`flow_seen` 紧凑键），MemberRef 驻留不做（无收益）；P2 调整为「空传播 / 空重跑削减」（保序）。

### 4.4 进展实测（release，`scripts/closure_bench.sh`；逐字节对照 `--diff` 全部 SAME）

| 步骤 | 提交 | HelloWorld | Digester 墙钟 / RSS | CollectorsDemo 墙钟 / RSS | DeepCopy 墙钟（user）/ RSS | DeepCopy flows / sites ms |
|---|---|---:|---:|---:|---:|---:|
| 基线 b3ccb0e9 | — | 0.80 s | 6.10 s / 1124 MB | 4.55 s / 764 MB | 454 s（351 s）/ 4074 MB ¹ | 352 k–443 k / 68 k–151 k |
| P1 图节点驻留 | 见 git log | 0.72 s | 5.43 s / 954 MB | 4.17 s / 710 MB | 256 s（231 s）/ 3606 MB ² | 170 k / 59 k |
| P2a 无增量快路径 + `IdSet` 稠密形态 | 见 git log | 0.91 s | 6.05 s / 705 MB | 4.60 s / 586 MB | **154 s（150 s）/ 2380 MB** | 92 k / 43 k |
| P2b 接收者集合改向量 | 见 git log | 0.20 s（user）| 5.15–5.89 s / 796 MB | 4.59 s / 656 MB | **131 s（130 s）/ 2252–2819 MB** ³ | 86 k / 29 k |

¹ 与主会话 e2e 批次并行、内存压力下测得（计划 §一 的 173.6 s 为无压力值，基线二进制未在无压力下复测）；比较以 user 时间为准。

² P1 首版（图节点驻留，未含 open 收窄缓存 / 原地归并）测得；Digester / CollectorsDemo 列为 P1 完整版。

³ 引擎自报峰值 2252 MB，`/usr/bin/time` 最大驻留 2819 MB（含释放前的分配器保留）；两次 P2a/P2b DeepCopy 的 time 读数在 2.4–2.8 GB 间波动。Digester 同机 A/B（P1 vs P2b 各两次）：user 5.20/5.29 s vs 5.20/5.01 s，RSS 912/954 vs 796/796 MB，小用例无回退。

**P1 调整说明**：按 §4.2 数据，`MemberRef` 驻留收益 < 1 %，改为驻留类型流图节点——`engine/graph.rs::FlowGraph`：`Node` → 稠密 `u32`，类型集 / 出边 / 待推增量 / 入队标记改为按序号索引的 `Vec`，流边去重键从 40 字节降到 12 字节，`drain_flows` 逐边只做数组索引，Object 过滤判定由逐边字符串比较改为 id 比较；另加 open 收窄结果按 (o, t) 缓存（去掉逐次 `ClassPath::get` 的 SipHash）、`IdSet` 小并大改为原地自尾归并（一次搬移）。

**P2a 说明**（保序、纯常数）：`add_to` 先做不分配的子集判定，无增量直接返回（不取节点、不查空数组暂存）；新接 Object 过滤边且目标已含源集合时不再克隆整集合；`IdSet` 拆到 `engine/idset.rs`，≥ 64 元素改为「位图 + 非零字摘要」单一稠密形态（插入 O(1)、差 / 并 / 子集按字、升序遍历跳零字），取代「有序向量 + 位图索引」双存（大集合逐个插入的 memmove 与双份内存一并消除）；哈希与旧形态逐字节相同。小用例墙钟在噪声内（主会话并行 e2e），DeepCopy user 时间 231 → 150 s、峰值 RSS 3.6 → 2.4 GB。

**P2b 说明**：`receivers` 的 exact 部分直接收集为 `Vec<u32>`，仅在存在 open 类型时才建 `IdSet` 并入 G 展开，调用点重跑（sites）43 k → 29 k ms。

**P2 结论与遗留**：保序前提下 DeepCopy user 351 → 130 s（基线受内存压力放大；对无压力的 150 s 级 user 仍降约 2.7×），但未达「与 Digester 同量级」。剩余热点仍是 E↔W 二部扇出上的无增量传播（`add_to` 调用 99.7 % 无增量）与站点重跑，需按 §4.3 的结构性改造（汇聚节点化 / SCC 序）才能再降一个量级，这要求把不变量放宽为「集合一致 + via 可变」，待决策。另记精度观察（未改）：DeepCopy 中单个 arraycopy 类手写站点的 W 入度约 6200 个数组，是潜在的不精确来源。

### 4.5 结构性改造与后续（`closure-perf2`，2026-10-01；不变量为「集合一致」，`--diff` 四例全部 SAME）

**口径**：机器 16 GB、swap 常驻约 9.6 GB 且有其它代理并行，墙钟与 RSS 抖动大（同一二进制 DeepCopy 墙钟 33–41 s、HelloWorld 0.24–2.4 s，后者 user 恒为 0.16–0.17 s，墙钟差是缺页 I/O）。
因此：CPU 以 `/usr/bin/time -l` 的 **instructions retired** 为准（同一二进制重复跑差 < 1 %），内存以 **peak memory footprint** 为准（RSS 受换出影响偏小或偏大）；`closure_bench.sh` 表已加「峰值 footprint MB」列。
基线 = `build/rava-base`（f86695ac，与 P2b 同逻辑）。

| 步骤 | 提交 | HelloWorld 墙钟 | Digester 墙钟 | CollectorsDemo 墙钟 | DeepCopy 墙钟（user）| DeepCopy footprint | DeepCopy 指令数 |
|---|---|---:|---:|---:|---:|---:|---:|
| 基线 | f86695ac | 0.30 s | 6.65 s | 4.44 s | 169 s（146 s）| 2886 MB | 2535 G |
| S1 Object 边环合并 | 60a1b85f | 1.16 s ¹ | 7.87 s | 5.21 s | 79.2 s（74.1 s）| 1919 MB | 1012 G |
| dispatch_sites 按规范成员聚合 | a92ebced | 1.35 s ¹ | 7.78 s | 5.54 s | 78.7 s（68.9 s）| — | — |
| 热路径去字符串（镜像 id 记忆、句柄写入判定免分配）| e55f3bd2 | 0.95 s ¹ | 6.37 s | 4.64 s | 56.6 s（53.4 s）| 1714 MB | 895 G |
| 新接边收窄记忆（fmemo，64 MB 预算）| 1e162d0a | 0.24 s | 6.35 s | 4.66 s | 41–50 s（40–45 s）| 1.8–1.9 GB | ≈ 600 G |
| 枢纽重放去重（hub_lsent / hub_ssent）| 82eb2889 | 0.72 s ¹ | 7.14 s | 5.02 s | 33.1 s（32.4 s）| 2279 MB | 483 G |
| 观测：pushes_by_kind / hubs | bfa815b4 | 1.33 s ¹ | 5.56 s | 4.07 s | 31.8 s（31.3 s）| 2382 MB | 483 G |

¹ user 0.16–0.17 s，墙钟差为内存压力下的缺页 I/O。

- **S1 环合并**（`engine/scc.rs`）：只经 Object 过滤边构成的强连通分量在不动点处类型集必然相等，合并为代表节点（成员保留身份，钩子逐成员触发）。DeepCopy 21 轮检测合并 12.5 k 节点，E→W→E 大环消失，指令数 2535 G → 1012 G。
- **fmemo**：新接非 Object 过滤边时，大源集合（≥ 64 元素）的整集合收窄按 (源代表, 过滤) 记忆，源集合元素数作版本号。命中 3.25 M / 未命中 0.22 M。不设上限时 footprint 到 3.2 GB，故按 64 MB 预算整表清空（DeepCopy 清空 20 次）。
- **枢纽重放去重**：同一分析结果下，调用点换接子枢纽时继承的 lambda / 按调用点建模接收者是恒等重放，按 (偏移, 接收者) 记已送达。代价是记录 2.14 M + 16.5 M 条（约 150–200 MB），是 footprint 回升的主因，留给 P4 收紧（例如改为按枢纽父链判定而非逐接收者记录）。

**DeepCopy 现状剖析**（82eb2889 / bfa815b4）：

- 分阶段：flows 15.7–16.3 s、sites 11.0 s、process 3.4 s、analyze 1.2 s。
- `add_to` 5.45 亿次，其中 1580 万次有增量。
- 推送按边种类（次数 / 有增量）：S→HP 211 M / 0.49 M，W→E 110 M / 85 k，P→HP 66 M / 263 k，HP→P 35 M / 919 k，O→S 28 M / 540 k，E→W 16 M / 47 k。
- S→HP 边 38.3 万条，平均每条边被推约 450 次：源节点的集合在「处理一个方法 → 排空传播」的交替中被零碎增量反复推送，枢纽代数不是主因（11.6 k 枢纽、1.1 M 调用点接入、单调用点最多 244 次）。
- 另有约 3 s 耗在内存压力下从 jmod 读类（`Archive::read_class` 阻塞），属环境因素，P6 缓存后消失。

**试过未采用**：

1. **拓扑序工作表**：`fwork` 改为按凝聚图拓扑序号出队（每轮环检测重算）。推送 −13 %，指令数 483 G → 473 G（−2 %），额外开销是堆与每节点 4 字节。收益不值，未提交。
2. **换接后代枢纽时摘除直连祖先枢纽的冗余边**：可证健全，可达关系与过滤类型不变。实测只摘 2.8 万条，推送与指令数无变化，说明 S→HP 冗余不来自枢纽链。未提交。
3. **批量排空（每处理 64 个方法 / 站点排空一次传播）**：DeepCopy 指令数 483 G → 421 G（−13 %），flows 16.1 → 11.7 s，`add_to` 次数减半。但 **CollectorsDemo 集合不一致**：类 1166 → 1155，方法 8010 → 7941，丢了 `Class.newInstance` 及反射构造一族。
   - 原因是现有引擎存在**依赖处理顺序的非单调判定**。`CalendarSystem.forName@78`（`Class.forName(names.get(name))`）按名取类时，`class_lookup` 返回 `Some` 就不接 `R(Class.forName)` → 结果的边，返回 `None` 就接边，而边一经接上永不撤回。
   - 基线顺序下该站点某次求值走了 `None` 路径，边留了下来，@81 `newInstance` 有接收者；批量顺序下每次求值都走 `Some`，@81 判为 `null_recv`。
   - 所以基线结果本身是处理历史的产物，不是良定义的不动点。批量排空要落地，先得把这类判定改成单调：「`Some` → `None`」时接边，「`None` → `Some`」不撤边且对称处理，或者按名结果与常规返回值取并集。这会改变集合结果，属精度线的职责，待决策。
   - 同理，任何改变方法 / 站点处理次序的改造（按调用图 SCC 排序、失效合并批处理）都可能触发它。

**终态差距**：
- DeepCopy 墙钟约 32 s（目标 ≤ 10 s）。
- DeepCopy footprint 约 2.3 GB（目标 ≤ 1 GB）；Digester / CollectorsDemo footprint 0.69–0.94 GB，已达标。
- HelloWorld user 0.16 s，达标。
- 下一步收益最大的是批量排空（先解决上述顺序依赖）；其次是 W→E 扇出（手写 arraycopy 类站点 W 节点入度 2608，约 3110 个数组进入约 1800 个手写站点）和 P4 的去重记录 / fmemo 内存。

**精度观察**（只记录，未改）：
- `ConcurrentHashMap.put` P1 入度 11 796、出度 11 079。
- arraycopy 类手写站点把全部逃逸数组两两相连（W 入度 2608）。
- `CalendarSystem.forName` 的按名查找结果依赖求值次序（见上）。

**生成器与顺序**：Python 与 Rust 生成器在 TestHashMapOps / TestStreamBasic / TestCompletableFuture / TestSwitchString 上对照 S1 前后的生成树不变，条目顺序变化不影响生成。

## 五、内存上限的系统层手段（运维参考，不替代 P5）

- Linux：`ulimit -v`、`prlimit --as=`、`systemd-run --scope -p MemoryMax=4G`、容器 `--memory`；
- macOS：内核不强制 `RLIMIT_AS`，无可靠系统层上限——这是 P5 在程序内实现预算的原因。

## 六、执行约束

- 执行者用独立 worktree + 分支（自 `rust-closure-analyzer`）；`CARGO_BUILD_JOBS=2`；不派子代理 / fork；不跑 e2e（抽查由主会话串行执行）；同一时间只跑一个 `rava closure` / 基准进程；小步提交（单步 ≤ ~40 分钟），阅读结论落盘笔记。
- 与精度线同改 `closure` crate：精度线收尾后再开工，或由同一执行者在精度队列之后串行接手。

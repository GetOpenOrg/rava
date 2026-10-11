# 闭包分析器性能、内存与工程化（rava closure）

> 日期：2026-09-30
> 上级计划：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md)（§七 终态指标「闭包计算耗时 ≤ 3s」只按 HelloWorld 定义，本文扩展到全量语料并补内存、健壮性指标）
> 状态（2026-10-01）：P0 ✅、P1 ✅（按数据改为图节点驻留，见 §4.4）、P2 ✅（保序常数优化，DeepCopy user 351 s → 130 s）；结构性改造（`closure-perf2`，§4.5）✅ 已合入；顺序依赖修复与批量排空（`closure-mono`，§4.6，集合结果与处理顺序无关，`--flow-batch` / `--hash-seed` 矩阵验收）✅ 已合入 6e0849c6，DeepCopy 59.5 s → 41.7 s / 2.3 GB；精度三期（`closure-prec3`）的数组汇聚与选择子克隆另把 DeepCopy 降到 6.8 s（✅ 已合入 d8212bee）。P3 上下文共享 / P4 内存 / W→E 扇出（`closure-mono` 第二轮，§4.7）✅，DeepCopy 物理占用 965 → 652 MB；P6 整体结果缓存（`closure-p6`，§4.8）✅，同输入重跑分析段 24–29 ms；P7 冷路径一 / 二 / 三 ✅ 已合入（d842653d，§4.8；DeepCopy sites 1460 → 947 ms、analyze + aux 1147 → 713 ms、流边 2.22 M → 1.32 M），P7 余项进行中；待做：P5 / P8（预算降级、工程化，另分担冷路径 DeepCopy ≤ 2 s）。S0 闸门 G1（`s0-g1`，§4.9，2026-10-11）：枢纽 lambda 重放 × 方法引用接收者展开的乘积已消除（读者 291 万 → 6.9 万），S0 仍不收敛，新主项为方法上下文克隆，未解除。不变量：集合一致、`via` / 顺序可变（用户 2026-09-30 同意）。优化方向总纲见 [`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md)。

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
| 同输入重跑（缓存命中）分析段墙钟 | 无缓存 | **≤ 0.3 s**（P6） |
| DeepCopy 冷启动分析墙钟 | 5.1 s | **≤ 2 s**（P5 / P7 / P8 分担，§4.8） |
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
| **P5 预算与降级** | 另承担冷路径的流传播段：DeepCopy `flows` 1.21 s → **≤ 0.4 s**（差量传播与汇聚点批量，§4.8）；`--max-mem <MB>` / `--time-budget <s>`：计数型全局分配器；逼近上限时剩余方法降为上下文不敏感（结果仍健全，只是更粗），summary 标记 `degraded` 与触发点；硬上限处带诊断退出（非系统 OOM）。`rava build` 缺省预算按机器内存推导 | 人为设低预算：输出 ⊇ 无预算输出、动态对照漏覆盖 0、进程不被杀 |
| **P6 整体结果缓存** | 键 = 分析的全部输入（分析器可执行文件内容、JDK jmods 与镜像目录、`runtime/java_runtime` 全部文件、用户类与 `--lib` 内容、全部影响结果的参数、条目格式与 closure.json 版本）；值 = closure.json 值 + 分析期诊断行；`--closure-cache <目录>`，`main.py` 缺省 `build/closure_cache/`。原计划的两层缓存（已解码类、JDK 方法摘要）按实测「不实施」，见 §4.8 | 任一输入变化不命中；损坏删除重建；写入中断可恢复；命中 closure.json 与冷算逐字节一致（计时字段除外，27 例）；同测试第二次分析段 ≤ 0.3 s |
| **P7 并行** | 另承担冷路径的站点重跑与方法体分析：DeepCopy `sites` 1.54 s → **≤ 0.5 s**、`analyze` + `aux_analyze` 1.14 s → **≤ 0.4 s**（按方法分片并行、确定性合并，§4.8）；类解码 / CFG 预构建并行；不动点主循环保持单线程（确定性优先） | 输出逐字节一致；双种子一致 |
| **P8 工程化** | 另承担冷路径的建图段：DeepCopy `process` 1.18 s → **≤ 0.4 s**（§4.8）；`unwrap` / `panic` 清零；规则单测补齐；27 例闭包快照测试（变化须审查）；清单 schema 校验；`engine.rs` 等超长文件拆分；classfile 解析器 `cargo-fuzz`；双种子确定性与 Linux / macOS 双平台纳入 CI | §二 对应指标达成 |

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

### 4.6 顺序依赖修复与批量排空（`closure-mono`，2026-10-01；8162d43e / 4b283217 / 4f98da55）

**根因**：按名查找的结果依赖求值先后，是健全性问题，不是精度取舍。有两处。

1. **按名取类**：`CalendarSystem.forName@78` 首次分析时，常量表读取的接收者还没有来源，返回空集（底）。
   - 该站点没有登记读者，后续重分析只重跑增量站点，@78 于是永久为空。
   - @81 `newInstance` 因此拿不到接收者，被折叠为 `null_recv`，这是不健全的。
   - 基线能拿到 `newInstance`，只是某次求值碰巧走了 `None` 路径，边留了下来。
2. **形参字符串常量**：常量取自形参常量汇合格 `pvals`，而 `pvals` 对「取常量集」不单调，先到的常量会被后到的调用点抬为 Top。
   - 另外，switch 合流的多个字面量在 `Src::Str` 上合成一个无值来源，字面量整体丢失。

**修复**（8162d43e）：

- **跨偏移读者 `xreaders`**：按名查找站点在本方法每次重分析时一并重跑。
- **按名取类单调**：
  - 常量表读取遇到非常量表接收者（open / lambda / 手写对象）时标 `partial`，候选照取，结果另接所指未知的 `Class`。
  - 一旦推不出，即粘滞为 Top（`lookup_top`），不再回落到名字集。
  - 按名求值读取被调方字段时，登记字段依赖。
- **形参字符串常量集 `engine/pstrs.rs`**：
  - 只并不减，不走汇合格。
  - 实参字面量并入被调形参槽；透传的形参建子集边；派发枢纽同样有形参槽，并流向父枢纽和各目标。
  - 槽增长时，读者站点进入站点队列重跑。
- **字面量编号**：`Src::Str(u32)` 携带字面量编号（`absint/lit.rs`，线程内驻留），合流后仍可经 `V::lits` 取回。

**顺序无关验收**：

- 配置矩阵为批量 1 / 7 / 64 × 哈希种子 0 / 12345 / 99（`rava closure --flow-batch N --hash-seed N`）。
- 在 HelloWorld / Digester / CollectorsDemo / DeepCopy 上，类、方法、派发、折叠、事实集合逐项一致（去掉 `via` 与统计后比较）。

**相对 3e944bc9 的集合变化**（全部是原先漏掉的可达内容）：

| 用例 | 变化 | 原因 |
|---|---|---|
| Digester / DeepCopy | 新增 `Class$1`（类 / clinit / 实例化，`<init>`、`run`）、`Class.newInstance`、`ReflectAccess.newInstance`、`ReflectionFactory.newInstance`、`InvocationTargetException.getTargetException`、`Unsafe.throwException`；去掉 `CalendarSystem.forName` 的 `null_recv` 折叠 | @78 的接收者是 `ConcurrentHashMap`（非常量表），正确结果是 Top，@81 由此可达 |
| Digester / DeepCopy / CollectorsDemo | 新增 `MethodHandle.linkToVirtual` / `linkToStatic` / `linkToSpecial` / `linkToInterface`（方法、派发、反射成员）；`invokeExact` 成为反射成员 | `DirectMethodHandle.makePreparedLambdaForm@313` 的链接器名是 switch 合流的多个字面量，原先丢失 |
| DeepCopy | 新增 `MethodHandle.invoke` | `makeExactOrGeneralInvoker(boolean)` 有两个调用方，名字合流为 `invoke` 或 `invokeExact` |

**动态对照**（`scripts/dyn_compare.py`，mono 批量 64 的产物）：

- HelloWorld、TestClassForName、TestForNameInit：漏覆盖 0。
- Digester、CollectorsDemo、DeepCopy：各漏覆盖 1，均为 `LambdaMetafactory`。
  - 它在 indy 引导方法句柄解析时由 JVM 装载，栈顶帧是 `invokedynamic`。
  - 基线 3e944bc9 同样存在这一漏覆盖，与本改造无关。
  - `closure-prec3` 06fd3f31 已修复（`indy_models` 导出，加 `indy-model` 分类），合并后归零。
- 静态多出的 provenance 均为 100%。

**批量排空**（4b283217，缺省 64）：

| 用例 | 墙钟 s | 峰值 footprint MB | 类 | 方法 | 分析次数 |
|---|---:|---:|---:|---:|---:|
| HelloWorld | 0.21 | 78 | 251 | 620 | 1685 |
| Digester | 7.03 | 924 | 1441 | 9749 | 46717 |
| DeepCopy | 41.70 | 2322 | 1650 | 10982 | 63491 |
| CollectorsDemo | 4.77 | 702 | 1166 | 8014 | 40620 |

- 同机基线 3e944bc9 实测：DeepCopy 墙钟 59.5 s，Digester 7.7 s。
- 机器同时承载其它代理，墙钟噪声大（DeepCopy 在 36–59 s 之间波动），以指令数为准：§4.5 的实测为 483 G → 421 G。

**遗留（理论）**：

- 常量表签名读取在尚无常量表接收者时会落到其它策略。若这时经被调方常量推出名字，读取之后再变成 `partial` 时，这些名字不会撤回。
  - 名字只增不减，所以结果仍健全，但可能不是最小。
- 其它依赖 `pvals` 乐观值的判定不在本次范围。

### 4.7 P3 上下文共享、P4 内存、W→E 扇出（`closure-mono` 第二轮，2026-10-01）

**计量口径**（584a8916）：

- 内存一律取物理占用 footprint（`summary.perf.peak_mem_mb`；`/usr/bin/time -l` 的 peak memory footprint），CPU 取 instructions retired。墙钟只作参考。
- 此前 DeepCopy 的 maxrss 在 1276–2097 MB 之间抖动。根因是 macOS 的 maxrss 不计被内存压缩器收走的页，内存压力越大读数越低；同一时期 footprint 稳定在 2128–2183 MB。
- 本节数字都在全机锁（`heavy_lock.py`）内测得，期间没有其它重进程争用。DeepCopy footprint 复测落在 649–685 MB，读数稳定。

**逐步实测**（DeepCopy，release，批量 64 / 种子 0）：

| 步骤 | 提交 | footprint MB | 指令 G | 说明 |
|---|---|---:|---:|---|
| 起点（合入 prec3 后） | 46fe88a1 | 965 | — | |
| 子类型判定缓存改为两位位图 | 23a78e07 | 902 | — | |
| `IdSet` 内联压到 24 B | 1dc941c1 | 871 | — | |
| 流边去重只收高出度源 | 9103382f | 860 | — | |
| 高出度去重改为出边下标哈希表 | 4e47ca01 | 843 | — | |
| P3 上下文共享摘要 | 3ffe1eed | 764–775 | 79.8 | 摘要复用 23 960 次；analyze 阶段 1156 → 723 ms |
| P3 克隆上下文选择收拢到 `ctxsel.rs` | ea5beac2 | 775 | 79.7 | 纯结构调整，统计逐项不变 |
| P3 TypeSet 按内容哈希驻留（`setstore.rs`） | d882bccd | 685 | 80.4 | |
| P4 枢纽重放按祖先链判定，去掉 `hub_ssent` | 43b20741 | 669 | 79.6 | |
| P4 `recv_done` 改为按站点的升序表 | 59f8385b | 652 | 80.5 | |

同期其它用例（d882bccd）：Digester 440 MB，CollectorsDemo 351 MB。终态目标「峰值 ≤ 1 GB」已达成。

**各步验收**：

- 每一步都在 HelloWorld / Digester / CollectorsDemo / DeepCopy / TestClassForName / TestForNameInit 上跑批量 64 × 种子 0 与批量 1 × 种子 99 两组配置。两组都与合并基线比对：类、方法、派发、折叠、事实集合全部 SAME。
- ea5beac2、43b20741、59f8385b 三步的 `flow_edges`、`pushes_by_kind`、`shared_analyses` 等统计与上一步逐项相同，只有计时字段不同。
- `dyn_compare` 在 d882bccd 上 6 例漏覆盖均为 0。

**P3 上下文共享**（3ffe1eed，`engine/share.rs`）：

- 方法体的抽象解释只取决于三样东西：
  - 方法本身；
  - 入口形参常量 `pvals`；
  - Class 形参的镜像集。
- 全局事实的读取都已按被分析上下文登记依赖（`fdeps` / `rdeps` / `never` / `pdeps`）。
- 因此入口状态相同、并且摘要仍被某个上下文有效持有时，新上下文直接共享这份摘要（`Rc`），同时原样重放依赖。之后任何一条依赖触发，全部持有者一起失效。
- 收尾阶段（`NoReturn::closing`）的「尚无返回」答复有时效性，所以这一阶段既不复用摘要，也不登记依赖。
- 实测：DeepCopy 共 29 923 个上下文，不同的入口状态只有 9 866 种。

**P3 克隆上下文选择单点化**（ea5beac2，`engine/ctxsel.rs`）：

- 接收者上下文 `recv_ctx` 和静态 / lambda / 句柄调用上下文 `static_ctx(Call::{Invoke, Lambda, Handle, Eager})` 收拢到同一个文件。
- 此前这些逻辑分散在 `classes.rs`、`selector.rs`、`forward.rs`、`engine.rs` 四处。

**P3 TypeSet 驻留**（d882bccd，`engine/setstore.rs`）：

- 每个节点持有 `Rc<TypeSet>`，另维护一个加法内容哈希（元素哈希之和，增量更新）。
- 增长时先在驻留表里查找增长后的内容，比对元素数与子集关系，查找本身不需要克隆：
  - 命中则共享表里的集合；
  - 未命中、且只有本节点和表持有（引用计数为 2）时，原地增长。
- 表长每翻倍一次，清扫只剩表自身引用的项。
- 实测 DeepCopy：写入 1.52 M 次，命中 1.45 M 次，不同集合约 15 k 个；`graph.sets` 从 83 MB 降到 9.8 MB。
- 先后试过两种更简单的做法，都放弃了：
  - 周期性压缩：压缩后写时又各自克隆，占用不降；
  - 每次写入都驻留、命中前先克隆：指令 +10%。

**P4**：

- **按指令的帧**：分析器本来就只保留合流点的帧，逐条指令的帧用完即丢，「释放逐指令帧」已经成立，无需改动。
- **枢纽重放去重**（43b20741）：
  - 精确枢纽展开后，其目标列表即为终值；子枢纽的 `special` 列表以父枢纽的同名列表为前缀。
  - 站点若已链接过某个祖先枢纽，且该祖先的 `special[t]` 覆盖待重放的接收者集，就跳过重放。
  - 由此删去按站点记录的 `hub_ssent`（DeepCopy 约 21 MB）。
- **`recv_done`**（59f8385b）：
  - 由「站点 → 哈希集」改为「站点 → 升序 `Vec<u32>`」，两个哨兵值 `FIELD_STATIC`、`FIELD_OTHER` 同在表内。
  - 实测 2.24 M 项分布在 106 k 个站点上，其中 2.1 M 项集中在对象数 ≥ 64 的站点。按升序表二分插入，指令约 +1%。
- **mimalloc**：保留。系统分配器 A/B 实测 footprint +100 MB、指令 +25%。
- **剩余占用**（d882bccd 后按「逐项 drop」测量，DeepCopy 存活约 486 MB）：

  | 项 | MB |
  |---|---:|
  | classpath.cache | 113 |
  | methods | 66 |
  | edges | 32 |
  | recv_done | 31 |
  | seen | 30 |
  | hubs | 23 |
  | pstr | 22 |
  | hub_ssent（已删） | 21 |
  | dispatch | 15 |
  | watch | 13 |
  | fmemo | 12.5 |
  | delta | 12 |

  - classpath.cache 是解析后的类文件，其中 `Insn` 每条 96 B，属于 input crate 的表示问题，不在本节范围。
  - 单项 ≤ 15 MB 的条目已到收益递减区。

**PV 哈希驻留**：`pvals` 总计约 4.8 MB，驻留的收益上限即为此数，只作记录，不实施。

**W→E 扇出**（结构化 `hw_site_arrays`）：DeepCopy 实测只有 1906 条边、61 k 次推送，在总推送量中占比可以忽略，价值低，不实施。

### 4.8 P6 整体结果缓存（`closure-p6`，2026-10-01）

**原计划两层缓存的实测（持锁，release）——均「不实施」**：

- 第一层（已解码类）：类解码只占总耗时 1–3%。HelloWorld 488 类 48 ms / 总 0.24 s，Digester 1434 类 97 ms / 3.3 s，
  CollectorsDemo 1194 类 42 ms / 2.2 s，DeepCopy 1832 类 72 ms / 5.1 s。class 格式本身已接近反序列化格式，
  换成缓存的解码结果读回也是同一量级；分析器不建 CFG（cfg crate 只在生成器用）。
- 第二层（JDK 方法摘要）：收益上限约 22%。DeepCopy 5.1 s 中方法体分析（`analyze` 0.56 + `aux_analyze` 0.58）合计 1.14 s，
  其余约 4 s 是整程序不动点（`process` 1.18、`sites` 1.54、`flows` 1.21 s），JDK 节点的内容取决于用户代码流入什么，
  没有跨测试可复用的单元；摘要全部命中 DeepCopy 仍约 4 s。各测试共有的前缀状态（种子部分）规模约一个 HelloWorld，热启动只省约 0.15 s。
- 跨测试复用不再追求；跨测试摘要命中率不测。

**终态：整体结果缓存**（`closure::cache`，驱动侧 `driver/src/closure_run.rs`）：

- 键：128 位分帧流式指纹，覆盖条目格式版本、`FOLDS_VERSION`、分析器可执行文件内容、类路径全部档案（加入序、来源角色、路径与内容）、
  `runtime_dir` 全部文件、`Input` 每个字段（解构穷举，新增字段不进键即编译失败）、内部表哈希初值、命令行放行项。
  分析器不读环境变量，JDK 位置等环境项都经命令行化为档案路径。
- 条目：`<键>.entry`，头部带格式、键、载荷长度与摘要；先写临时文件再改名；任一校验不过即删除并冷算重写；遗留超过 1 h 的临时文件下次写入时清掉；
  总量超过 `--closure-cache-max-mb`（缺省 4096）按修改时间淘汰。
- 命中：诊断行原样重放，事实经 `ClosureFacts::from_json` 交给发射；冷算且启用缓存时总校验 `from_json` 与 `from_closure` 一致，
  保证命中与冷算交给发射的事实相同。`--trace-class`（`rava closure` 的 `--why` / `--flows` / `--report`）需引擎本体，不读缓存。
- 单测（`closure/src/cache/tests.rs`）：任一输入变化不命中、损坏（截断 / 翻位 / 空 / 缺头 / 错键 / 错格式）删除重建、写入中断恢复、往返逐字节一致、淘汰。

**冷路径不因缓存放弃**：未跑过的测试、改过的用户代码、改过的分析器都走冷分析。DeepCopy 冷启动终态 **≤ 2 s**，分担为
P5 `flows` 1.21 → ≤ 0.4 s、P7 `sites` 1.54 → ≤ 0.5 s 与 `analyze` + `aux_analyze` 1.14 → ≤ 0.4 s、P8 `process` 1.18 → ≤ 0.4 s，其余阶段合计 ≤ 0.3 s。

**验收**（持锁，release，并入 rust-closure-analyzer 3c7ca8e6 后复跑，基线为 3c7ca8e6 的 27 例生成树）：

- 27 例：清空缓存后冷算树与基线 `compare_trees` 全 0、raw-audit 一致；再跑一遍全部命中，命中树与冷算树全 0；
  closure.json 剔除 `summary.elapsed_ms` / `summary.perf` 后 27 例逐字节一致（`elapsed_ms` 之前的原文前缀亦逐字节一致）。
- 命中分析段（键计算 + 读条目 + 校验 + 解析）24–29 ms，27 例全部命中，缓存目录 28 条共 23 MB。
- DeepCopy（`rava build --perf`）：`closure` 阶段冷 5110 ms / 命中 33 ms；整次 `--no-run` 墙钟 6.34 → 1.12 s，最大 RSS 687 → 372 MB。
- 单测：`cargo test -p closure -p input -p emit -p driver` 全过（cache 6 项，build_opts 新增 `closure_cache_options`）。

**P7 冷路径进展（`closure-p6` 续，2026-10-01；release，DeepCopy 分段 ms，单次运行波动约 ±5%）**：

计量口径：各版本 `git archive` 到 `build/verify/<提交>`，**各用独立 target-dir 全新构建**，对照前核对 md5 互异。
曾出现的计量事故：快照目录与工作区共用 target 时，cargo 指纹按工作区相对路径 + mtime 判定，快照源文件 mtime 早于上次构建即被当作最新，
两个「不同版本」实为同一二进制（只影响未提交的 p7e2 中间结论，已作废重测）；已提交版本按上述口径复核。

| 版本 | analyze | aux_analyze | sites | flows | process | 流边 | 集合（对 29 例：验收 27 + DeepCopy + Digester） |
|---|---|---|---|---|---|---|---|
| d41b3006（基线） | 558 | 589 | 1460 | 1130 | 1128 | — | — |
| 6c8d48f6（P7 一、二） | 423 | 295 | 1153 | 1116 | 1100 | — | 对 d41b3006 全部 SAME |
| f67d1de8（prec3 嵌套宿主边，集成分支） | 550 | 575 | 1396 | 1141 | 1115 | — | 精度改动，对 d41b3006 有差异 |
| 48dcfc01（合并） | 418 | 294 | 1083 | 1130 | 1088 | 2.22 M | 对 f67d1de8 全部 SAME |
| P7 三（站点只处理新增接收者 + 字段读汇集节点） | 420 | 293 | 947 | 997 | 1057 | 1.32 M | 对 48dcfc01 全部 SAME |
| P7 四（字段写站点经汇集节点分发；合并 d842653d 后为 m39） | 426 | 303 | 884 | 914 | 1051 | 0.77 M | 对 48dcfc01 全部 SAME |
| P5 一（枢纽对按调用点建模目标的增量接边） | 426 | 303 | 894 | 606 | 1047 | 0.77 M | 对 m39 全部 SAME |
| 4b80d77c（合并 c3 后的集成分支，新基线） | 428 | 303 | 910 | 632 | 1050 | 0.77 M | — |
| P8 一（成员引用文本键缓存、按 Rust 名找方法先比前缀） | 423 | 299 | 891 | 607 | 875 | 0.77 M | 对 4b80d77c 全部 SAME |
| P8 二（内部表哈希按字并入、汇集节点成员归并、其余清单查询用缓存键；基线 = P8 一，c9d0a0ca 的 closure crate 与之相同） | 357 | 269 | 727 | 603 | 757 | 0.77 M | 对 P8 一全部 SAME |

- P7 一：站点重跑常数开销、辅助分析去重（aux_analyses 54 121 → 25 130）；P7 二：记忆条目按条目号精确作废（`Dep::Memo`）。
- P7 三：字节码站点重跑只处理新增接收者值（含 open 值，`recv_delta`）；字段读站点按 (字段, 对象集合) 共用汇集节点 `G`，
  O→S 推送 1436 万次（有效 31 万）→ G→S 74 万 + O→G 62 万；Digester sites 516 → 431、flows 694 → 551。
- P7 四：字段写站点同样经汇集节点（写向「写入值 → G → 各对象字段」）；推送 2760 万 → 850 万次。
  试验（未提交）：数组元素读写也经汇集节点（槽位 = 分配点 × 下标奇偶，写向出边按各分配点分量类型过滤），
  29 例 SAME、流边 77.3 万 → 66.2 万、推送 848 万 → 749 万，但耗时无可测收益——数组站点不是瓶颈，不并入。
- P5 剖面（m39，drain_flows 852 采样）：约 55% 在 G 增长 → `hubs_grow` → `hub_recv`，其中 open 枢纽每新增一个接收者，
  对按调用点建模的目标（手写 / 静态 / 特判返回 / 透传）在全部接入调用点上各重做一次完整 `edge`（调用关系登记、
  形参常量并入、字符串常量、全部实参边），且每次克隆整张调用点表；纯推送约 45%（其中 `is_subset` 无增量判定约 120）。
- P5 一：调用点接入记录 `Link` 带序号、以 `Rc` 共享；枢纽记下「(接入记录, 目标) 已完整接边」，同一记录再派发该目标的
  新接收者时只接接收者相关部分（this 注入、手写调用点、依赖接收者的返回值模型）——其余部分对同一实参 / 实参值幂等
  （形参常量只升不降、集合只并不减）。重接入（分析变化）换新记录，自然回到完整接边。DeepCopy flows 914 → ~600、
  Digester 491 → 307。
- P8 一：process 剖面（m39）里格式化约 70 采样来自每个调用事件反复 `mref.to_string()`（`refs` 登记、清单反射 / 系统属性 /
  按名取类查询），按引用缓存文本键；`methods_by_rust_name`（手写体回调推断）对类型层次上每个方法都算 Rust 名（含重载计数与
  后缀格式化，约 155 采样），改为先比「方法名 / 方法名_」前缀。DeepCopy process 1050 → ~875、Digester 672 → 532。
- P8 二：内部表哈希器 `FxHasher::write` 原为逐字节并入，字符串键（`MemberRef` 等）与整数切片键（枢纽精确接收者集合
  `HubSet::Exact`、汇集节点成员集合）每字节一次乘法；改为按 8 / 4 字节成字并入（只改哈希值、不改相等判定，表的遍历顺序随之变化，
  结果与处理顺序无关——双种子对照口径）。sites 剖面中枢纽键哈希约 63 采样。另：汇集节点的累计成员由「拼接 + 排序」改为两段升序归并；
  类初始化 / 服务查找 / 系统属性持有者查询改用成员引用缓存键。DeepCopy analyze 420 → ~357、aux 296 → ~269、sites 893 → ~727、
  process 872 → ~757；Digester sites 400 → 326、process 532 → 424。
- **暂停点（2026-10-01，用户决定先完成 C1d、C4 回归修复，性能线暂停）**：DeepCopy 各段（ms）analyze ~357、aux ~269、
  sites ~727、flows ~603、process ~757、setup ~149、seeds ~46，合计约 2.9 s（目标 ≤ 2 s）。剩余目标差距：sites（≤ 0.5 s）、
  flows（≤ 0.4 s）、process（≤ 0.4 s）。恢复后的候选：
  - sites：站点重跑每次重算完整值集再与已接集合求差（`recv_delta` / `recv_mark_all` 约 60 采样自耗时），可改为按读者节点的增量驱动；
    精确枢纽集合增长时新枢纽的建立（`receivers` 全量重算、父链接收者表复制）；
  - flows：纯推送约 380 采样（`is_subset` 无增量判定约 120）；
  - process：剖面平坦，`static_ctx → dispatch_slots`（辅助分析，已按成员缓存）、类文件惰性读取约 114、分配器约 280。
- 原「下一步」：P8 process 余量（剖面平坦：调用点分派 / 槽位上下文 `dispatch_slots`、类读取、分配器），之后 analyze 常量求值。

### 4.9 S0 闸门 G1：枢纽 lambda 重放 × 方法引用接收者展开（`s0-g1`，2026-10-11，未解除）

背景：S0 最小 Spring Boot 应用的档案闭包在 dev 上不收敛（A 58451 MiB / 2251 s、B 31822 MiB / 1831 s 被看门狗终止），
栈落在 `link_hub` 重放 lambda × `lambda_dispatch` 方法引用接收者展开（`docs/reports/api-surface-s0.md` §5.1）。
目标：A / B 两变体在 dev 上产出，峰值 ≤ 16 GiB、耗时 ≤ 30 min；既有用例的类 / 方法集合与 main 一致（种子 0/1/2）。

**诊断**（cd5c226b / f5fb5aef / b69882be）：`rava closure --lambda-prof <秒>`（`API_SURFACE_LAMBDA_PROF`）周期向 stderr 打
枢纽重放、lambda 调用读者、方法引用接收者展开三类前列，附 RSS 峰值、读者复活、同调用点多读者分量（实参 / 返回类型 / 结果节点）。
定位结果（S0 A）：

- 主体是 `StreamSpliterators$WrappingSpliterator` 的 `buffer::accept` 一类 k9 方法引用（Sink / Consumer，另有 Int / Long / Double 变体），
  经 open Sink 枢纽（`WrappingSpliterator.tryAdvance` 处 `Consumer.accept`）送达约 2200–2700 个调用点；
- 每个 lambda 约 50–150 个绑定接收者，每次接收者集合增长都在每个调用点重算接收者、新建一个精确集合枢纽（父链），
  新枢纽各自接入全部调用点——单个 lambda 约 33 万次枢纽接入，规模 ≈ 调用点 × 接收者增长次数；
- 枢纽上登记的 lambda 逐调用点派发（`hub_lsent` 键 = 调用点 × lambda）：Consumer.accept 一类枢纽 2196 个接入 × 50 个 lambda，
  单枢纽重放 30–50 万次；读者（`LCall`）数随之到百万级（A 260 s 时 108 万）；
- 调用方重分析（偏移重跑 / 被调方摘要变化）使读者整体作废后逐个重建，重放次数在 76d83cc1 时达 3800 万。

**改动**（均已推 `s0-g1`，种子 0/1/2 下 HelloWorld / DeepCopy / TestSerialLookupPairing 类 / 方法集合与 main 一致，见下表）：

| 提交 | 内容 | S0 A（dev） | S0 B（dev） |
|---|---|---|---|
| 7ceca8e6 / 484c16ff | 方法引用 open 接收者改经 open 枢纽、精确接收者达 `HUB_MIN` 经集合枢纽（与字节码虚调用同口径），删 g_log 增量展开 | — | — |
| 1f34a68f / 68c7230f / 4e20f348 | lambda 读者按作废方式复活：偏移重跑保留已接记录只接增量，摘要变化清空后完整重接 | 547 s：17.4 GB，44.8 万方法，未收敛 | 603 s 21.3 GB 终止 |
| 76d83cc1 | 挂起读者以同一调用再登记时直接复活；枢纽同一接入重接时按调用键复活 | 546 s：17.8 GB；重放 3800 万（摘要变化引起的整方法重接占主） | 582 s：20.7 GB |
| e3d1aded | 被调方摘要变化只让经该读者接边的读者各自重接，不整方法重接（`dcallers` / `lcallers`） | 446 s：42 GB（读者 450 万、枢纽接入 5200 万）——更差 | 462 s：17.6 GB |
| e3cafda3 | 读者按（lambda、返回类型、结果节点）登记，不同实参并入同一读者（`merge_args`） | 322 s：18.6 GB；520 s：30.4 GB，58 万方法，读者 274 万 | 525 s：21.9 GB，67 万方法 |
| e6384922 | 方法引用绑定接收者达 `HUB_MIN` 后经增长枢纽（`HubSet::Grow`，按接收值来源节点共用）增量并入，不再逐次建精确集合枢纽链 | 257 s：14.3 GB；445 s：23.7 GB，55 万方法，读者 291 万 | 458 s：19.5 GB，63 万方法 |
| **74ef5f3f** | **枢纽上的 lambda 只在锚点（首个字节码接入点）以枢纽节点 `HP` / `HR` 为实参 / 结果建一个读者**，其余字节码接入点不逐调用点派发 | 210 s：7.9 GB，读者 2.9 万；615 s：26.7 GB，61 万方法，读者 6.9 万、重放 18 万（停止） | 563 s：19.1 GB，64 万方法，读者 3.2 万 |

**结论**：
- 去掉精确集合枢纽链（e6384922）后峰值与 e3cafda3 同量级，剩余主项是**枢纽 lambda 的逐调用点派发**：
  枢纽每到一个 lambda，就在全部 2196 个接入点各建一个读者并各自展开，读者数 = 接入点 × lambda；
- 74ef5f3f 把枢纽 lambda 改为「枢纽 × lambda」一个读者（锚定首个字节码接入点，实参取 `HP`、结果取 `HR`，常量 / 污染按缺省口径），
  与枢纽上普通目标的合流口径一致。lambda 路径的乘积项随之消失：读者 291 万 → 6.9 万，枢纽重放 2065 万 → 18 万，
  open Sink 枢纽接入点 2196 → 8（接入点本身也由逐调用点的 lambda 克隆产生）。种子 0/1/2 集合与 main 一致；
- **G1 仍未解除**：S0 A 在 615 s 时 26.7 GB、方法上下文 61 万且仍以约 40 MB/s、3 万方法 / 分钟增长。lambda 剖析已不在前列，
  新的主项是**方法上下文克隆**（对象敏感上下文：`@level:N` 档位上下文与按分配点的抽象对象上下文）。142a1c54 的上下文前列见
  `docs/reports/api-surface-s0.md` §5.1：`Object.<init>` 1 万份、`Objects.requireNonNull` 3300、`ConcurrentHashMap` 节点 / 查找 2000+、
  `MethodType` / `LambdaForm` 一族 1600，上下文对象以 `@level:0` / `@level:2`（各约 5800）与 `java.lang.invoke` 的
  BoundMethodHandle Species / DirectMethodHandle 分配点为主。

**失败方案**：
- 只在偏移重跑时保留读者（4e20f348 / 76d83cc1）：主项是摘要变化（`reset_sites`）引起的整方法重接，快路径命中极少；
- 摘要变化按读者重接（e3d1aded）：读者各自清空重接，没有了整方法重分析时的批量去重，读者与接入次数翻倍，A 峰值 42 GB；
- 增长枢纽（e6384922）：精确枢纽链消失，但未降低峰值（主项在上一条）。
- 方法上下文克隆上限（929de3b8，已回退）：同一方法的抽象对象上下文克隆超过 64 个后并入无上下文本体。S0 的方法上下文增长放缓
  （A 375 s 时 19.6 万、8.2 GB），但到 963 s 仍不收敛（A 峰值 16.2 GB、27.9 万上下文；B 峰值 16.9 GB，枢纽 78 万）。
  DeepCopy（种子 0）闭包从约 7 s 变为 16 分钟、14.6 GB 被杀：并入本体后各接收者的值在本体内汇合，精度崩塌。
  因此上下文预算不能简单回落无上下文本体，需要保精度的合并方式（见下一步）。

**下一步**：
- 锚点读者的常量 / 污染当前取缺省口径（`call_vals = None`）。终态应取枢纽合流值（`hub.vals`，与 `hub_bind` 同口径），
  并把读者挂在枢纽而非锚点方法上（锚点方法重分析时不作废）；
- 方法上下文克隆的预算与合并（性能计划 P5「预算降级」）：按方法限制上下文数。超出时不能回落无上下文本体（929de3b8 已证明精度崩塌），应按上下文对象的类型或分配类合并；
  对只做转发 / 校验的叶方法（`Object.<init>`、`Objects.requireNonNull`、`Math.min` 一类，不读写对象字段、无分配）不克隆。
  需按种子集合对照确认既有用例不变。

### 4.10 S0 闸门 G1：方法上下文预算与合并（`s0-g1-ctx`，2026-10-11，未解除）

承 §4.9：lambda 乘积消除后，S0 的主项是方法上下文克隆。本节做 P5「预算降级」的保精度形态：超预算时按类型 / 分配类合并，不回落无上下文本体。

**实现**（`engine/ctx_free.rs`、`engine/ctx_budget.rs`、`engine/ctx_prof.rs`）：
- **上下文无关方法不克隆**（6cc3cf6f）：返回 void，方法体只做局部变量 / 常量 / 算术 / 比较 / 分支，异常表为空，无选择子形参，调用只有
  `invokespecial` / `invokestatic` 且目标同样上下文无关（`Object.<init>` 与只调超类无参构造器的构造器链）。这类方法的克隆与本体效果
  相同，一律进本体。判定按成员缓存，是方法字节码的静态性质，与处理顺序无关。DeepCopy / TSLP 每次闭包命中约 9600 次、38 个成员；
- **对象上下文克隆预算 2048，超出按「类型 + 堆上下文尾」合并**（9fef627e，af835822 改为只计实际新建的字节码克隆）：同一成员按抽象对象
  上下文建出的克隆达 2048 后，新到的对象上下文落到类型上下文 `^类|尾`。接收者仍逐个以 `Recv::Exact` 注入，分派按确切类型；
  类型上下文作堆上下文时按类型与属主分开。档位 / 具体求值 / 调用点 / 形参常量上下文不参与合并。预算高于 e2e 语料每成员对象上下文
  克隆数的上界（1736，`ConcurrentHashMap$Node.<init>`），语料不触发；
- **诊断**：`--lambda-prof` 周期打印 `ctxfree` / `ctxbudget` / `ctxkind`（按上下文种类的节点数）/ `ctxhist`（每成员克隆数分布）/
  `ctxmerge`（克隆最多的成员按分配点 / 类归并后剩几个）/ `objsite` / `objcls`。

**集合对照**（dev，种子 0/1/2，与 main 基线 f79f8758 逐项比较；时间 / 峰值为三种子并行时的单次值）：

| 用例 | 类 / 方法 | 基线 | 本分支 |
|---|---|---|---|
| HelloWorld | 580 / 1897，相同 | 2.2–2.3 s / 437–449 MB | 2.4 s / 440–445 MB |
| DeepCopy | 3080 / 16856，相同 | 1:46 / 2898–3002 MB | 1:44–1:46 / 2896–2971 MB |
| TestSerialLookupPairing | 3082 / 16841，相同 | 1:52–1:56 / 3443–3509 MB | 1:46–1:48 / 3450–3458 MB |
| CollectorsDemo | 619 / 2022，相同 | 2.2–2.4 s / 451–456 MB | 2.4 s / 453–457 MB |

本分支头 7a2d9946（含锚点顺序修复）：四例三种子的类数 / 方法数与基线相同；HelloWorld / CollectorsDemo 闭包 JSON 逐项相同；
DeepCopy 三种子与（batch 1, seed 1）（4096, 0）逐项相同，与基线只差派发表 −63/+66（锚点读者目标计入枢纽全部接入点，见下文）；
TSLP 与基线同差派发表，另有 `MemorySessionImpl` 的类 / folds / refs 种子差异（基线既有）。ctx_free 每次闭包命中 DeepCopy / TSLP
约 1 万次、38 个成员；预算在语料上不触发（`ctxbudget merged=0`）。

**S0 实测**（dev，看门狗上限 18–20 GiB）：

| 提交 | 变体 A | 变体 B |
|---|---|---|
| 88ae279e（预算前，剖析） | 245 s：7.8 GB，38.6 万方法上下文 | — |
| 9fef627e（ctx_free + 预算 2048） | 273 s：5.6 GB，26.5 万 | 246 s：5.3 GB，24.1 万 |
| 1de06faf（+ 锚点顺序修复） | 478 s：15.2 GB，51 万（Obj 35 万）；600 s：20.2 GB 被杀 | 900 s：23.7 GB 被杀 |
| 7a2d9946（本分支头：+ 预算只计新建克隆） | 484 s：18.6 GB，51 万（合并 2.9 万次、类型上下文 2428），枢纽 6.1 万；540 s 达 18 GB 上限被杀 | 503 s：16.8 GB，51 万（合并 5.7 万次），枢纽 9.1 万；570 s 被杀 |
| 8f16844a（试验：节点总数 20 万后一律按类合并） | 1059 s：18.5 GB，节点 33 万、**枢纽 78 万**；1141 s 被杀 | 870 s：19.4 GB 被杀 |

**结论**：
- 预算与 ctx_free 让同一时刻的方法上下文少约 30%，但不改变线性增长。1de06faf 的 `ctxhist` 显示节点主要分布在每成员 64–1024 个克隆的
  数百个成员上（≤1024 档 240 个成员 12.9 万节点、>1024 档 113 个成员 18.8 万节点），单成员预算压不住总量；抽象对象数本身（3.1 万，
  `HashMap$Node` 4126 个变体）随档案扩大持续增长；
- 试验 8f16844a 把节点总数钉在约 33 万后，内存仍以约 15 MB/s 增长，增长项换成**枢纽**：1.8 万（139 s）→ 78 万（1059 s），
  `recv_by hub_link` 142 万。前列是 `Function.apply` / `Supplier.get` / `PrivilegedAction.run` / `CharPredicate.is` 的 open 枢纽
  （各 400–950 个 lambda）。下一主项是枢纽的建立口径（逐调用点、逐接收者增长建精确集合枢纽），不是方法上下文。

**锚点读者的顺序无关**（g1-anchor-order：d65e86e9 + 988fd219，batch-1011b `closure_independent_of_order` 失败的修复）：
- 根因一：枢纽 lambda 读者建在锚点（首个字节码接入点）上，读者接边的目标记在锚点调用点的派发表，锚点随接入先后而定；
- 根因二：多个枢纽可锚定在同一调用点（精确集合枢纽与父链），锚点上的去重记录（读者键、`hub_lsent`、方法引用的 `dispatched`、
  `hub_linked`）不分枢纽，后到枢纽的读者被先到者挡掉，效果记到先到者名下；
- 修复：读者的一切按调用点的效果归枢纽。接边目标记入枢纽的 `ltargets`，读者接入的枢纽记入 `lhubs`，报告时计入该枢纽的每个接入点
  （与 `plain` 同口径，经父链与 `lhubs` 传递）；读者接边不取锚点方法的克隆上下文与调用点字面常量；锚点上的去重记录一律带所属枢纽
  （`LambdaKey` 含枢纽读者、`hub_lsent` 按（偏移, 接收者, 枢纽）、方法引用逐接收者派发按读者单元去重、再接入已接入枢纽时同样记 `lhubs`）。
  实现见 `engine/hub_reader.rs`；
- 验证（dev）：DeepCopy 缺省与（batch 1, seed 1）（7, 2）（512, 12345）（4096, 0）四种组合闭包 JSON 逐项相同；TSLP 派发表同样逐项相同
  （剩余 `MemorySessionImpl` 档位 / folds / refs 的种子差异是基线既有的，与本修复无关）。与基线相比，DeepCopy 的类、方法、
  `dispatched`、`instantiated`、`folds`、`refs` 相同，派发表只增不减（64 个调用点多了目标，新增 3 个调用点）：读者目标改为计入
  枢纽的全部接入点。

**失败方案**：
- 预算 256（26525eb6）：DeepCopy 多 8 个类，闭包慢约 50%：合并后的类型上下文汇合了同类不同属主的值；
- 推广 ctx_free 到返回基本类型 / 读字段 / 虚调用的方法（1a0111c1）：调用点上下文与对象上下文的克隆携带逐调用点返回常量与逐对象
  字段常量，并入本体后常量汇合，DeepCopy 多 2085 个类；
- 超预算回落无上下文本体（929de3b8，§4.9）：精度崩塌，DeepCopy 16 分钟、14.6 GB 被杀；
- 节点总数上限后一律按类合并（8f16844a，未合入）：节点数钉住但枢纽接替增长（见结论）。

**遗留**：
- 预算触发后按先到先得分配细克隆名额，哪些上下文拿到细克隆随处理顺序变化。语料不触发，S0 规模触发；终态须改为与顺序无关的判定
  （例如触发时把该成员已有的细克隆一并改接类型上下文）；
- 锚点读者的常量 / 污染取缺省口径（`call_vals = None`），终态应取枢纽合流值 `hub.vals`；读者模式下 `caller_edge` 的
  CallerSensitive 归属仍取锚点方法；
- G1 下一步：枢纽的建立口径（open / 精确集合枢纽按调用点 × 接收者增长的乘积），见上表 8f16844a 的剖析。

## 五、内存上限的系统层手段（运维参考，不替代 P5）

- Linux：`ulimit -v`、`prlimit --as=`、`systemd-run --scope -p MemoryMax=4G`、容器 `--memory`；
- macOS：内核不强制 `RLIMIT_AS`，无可靠系统层上限——这是 P5 在程序内实现预算的原因。

## 六、执行约束

- 执行者用独立 worktree + 分支（自 `rust-closure-analyzer`）；`CARGO_BUILD_JOBS=2`；不派子代理 / fork；不跑 e2e（抽查由主会话串行执行）；同一时间只跑一个 `rava closure` / 基准进程；小步提交（单步 ≤ ~40 分钟），阅读结论落盘笔记。
- 与精度线同改 `closure` crate：精度线收尾后再开工，或由同一执行者在精度队列之后串行接手。

# 闭包分析器性能、内存与工程化（rava closure）

> 日期：2026-09-30
> 上级计划：[`2026-09-29-rust-closure-analyzer.md`](2026-09-29-rust-closure-analyzer.md)（§七 终态指标「闭包计算耗时 ≤ 3s」只按 HelloWorld 定义，本文扩展到全量语料并补内存、健壮性指标）
> 状态：P0 ✅（见 §四）；P1 / P2 进行中（`closure-perf` 分支）。原排期：排期：闭包精度线手上的回归（DeepCopy / DES / Digester / TestFileAccessSpace 一组）修完后接手；与精度线同改 `closure` crate，**不并行**，由同一执行者串行推进或精度线收尾后交接。

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

**不变量**：纯性能步骤（P1 / P2 / P4 / P6 / P7）的 closure.json 与改前**逐字节一致**（`elapsed_ms` 除外）；改变精度形态的步骤（P3 / P5 降级）动态对照翻译域漏覆盖保持 0、默认预算下输出不变。

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

## 五、内存上限的系统层手段（运维参考，不替代 P5）

- Linux：`ulimit -v`、`prlimit --as=`、`systemd-run --scope -p MemoryMax=4G`、容器 `--memory`；
- macOS：内核不强制 `RLIMIT_AS`，无可靠系统层上限——这是 P5 在程序内实现预算的原因。

## 六、执行约束

- 执行者用独立 worktree + 分支（自 `rust-closure-analyzer`）；`CARGO_BUILD_JOBS=2`；不派子代理 / fork；不跑 e2e（抽查由主会话串行执行）；同一时间只跑一个 `rava closure` / 基准进程；小步提交（单步 ≤ ~40 分钟），阅读结论落盘笔记。
- 与精度线同改 `closure` crate：精度线收尾后再开工，或由同一执行者在精度队列之后串行接手。

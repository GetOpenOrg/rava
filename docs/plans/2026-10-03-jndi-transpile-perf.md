# TestJndiNoProvider 冷闭包转译性能（2026-10-03）

## 目标与约束

- TestJndiNoProvider 冷转译（无闭包缓存）当前 5–6 分钟（服务器），600 秒上限不放宽，要求明显低于上限。
- 只做去重复计算 / 增量 / 缓存与结构性去冗余；**不得靠缩小闭包提速**，类集 / 方法集与修前逐项相同。
- 测量：本机 `rava closure`（冷，独立 `--closure-cache`），经 `heavy_lock.py bash capped.sh`（600s / 8G 封顶）串行；
  集合对照忽略 via / perf / elapsed，其余字段逐项相同。

## 基线（0fad1f6b，本机）

- 闭包 195–205 s（`summary.elapsed_ms` 198.7 s），3540 类 / 22361 方法；HelloWorld 468 / 1822，StockTrans 3107 / 18814。
- 阶段：sites 71 s、flows 67 s、process 51 s。
- 采样剖析（≈ 99k 样本）热点：
  - `drain_flows` / `add_to_id`（≈ 31%，其中 `is_subset` 自身 12.6k）——推送绝大多数无增量；
  - `hubs_grow → hub_recv`（≈ 17%），`link_hub`（13k；其中 `special` 接收者表收集成 `TypeSet` 排序 4.4k）；
  - `dispatch_one`（16k），`apply_hw → dispatch_one`（10.5k）；
  - `reflective_writes → class_values`（8.4k，每次重跑把整个 Class 值集重新映射成类名）；
  - `Hierarchy::select → superclasses`（3.3k，同一（类, 方法）反复解析）；
  - `edge_ret → declares_memory / memory_modeled`（≈ 2.9k，每次格式化成员键查清单）；
  - `method_ctx → sysprops` 的 `BTreeSet<MemberRef>` 克隆插入（1.9k）。

## 第 1 步：重复计算去除（已完成）

| 改动 | 位置 |
|---|---|
| `declares_memory` 按方法缓存（键不变）；`memory_modeled` 并入（返回值模型 Read 即 memory_read 登记） | `hw_offset.rs` / `hw_mem.rs` / `invoke.rs` |
| 字段枚举 / 字段查找调用点的 Class 值集按站点增量处理（只映射新增部分；名字 / 模式变化时整体重算；与 `recv_done` 同口径作废） | `field_lookup.rs` / `invoke.rs` / `worklist.rs` |
| `IdSet` 大集合 `FromIterator` 直接置位，不再排序 | `idset.rs` |
| `sysprops` 未跟踪成员表改哈希集合，先查后插免克隆 | `sysprops.rs` / `sysprops_write.rs` |
| `Hierarchy::select` 按（解析类, 方法）记忆 | `resolve/hierarchy.rs` |
| 观测：按源节点的推送计数（`top_push_sources` / `push_src_by_kind`），枢纽节点标签带集合 / 调用点 / 目标数 | `graph.rs` / `flow.rs` / `stats.rs` |

结果：冷闭包 198.7 s → 126.0 s（本机，−37%），集合与基线逐项相同；HelloWorld / StockTrans 不变。

## 推送剖析（第 1 步后）

- 推送 ≈ 1.19 G 次，有增量 ≈ 4%。按边种类：`S→HP` 322 M、`HP→P` 242 M、`E→S` 152 M、`P→HP` 130 M。
- 枢纽 43 176 个，（调用点, 枢纽）接入 4.33 M，单调用点最多 343 个枢纽：
  调用点实参直接接到它接入过的每个枢纽（精确集合每增长一次换接一个新枢纽、open 集合每个 open 类型一个枢纽），
  实参每次增量都向全部历史枢纽推送。
- 推送最多的 `HP` 节点是约 200 个 `Object.equals` 精确集合枢纽（各 3–4k 接收者、≈ 1100 个中转目标、无父枢纽），
  每个出队 ≈ 1000 次、每次推 ≈ 1100 条边，几乎全部无增量。

## 发现：闭包结果依赖传播顺序（既有问题）

基线二进制（0fad1f6b）仅把 `--flow-batch` 由缺省 64 改为 7，结果变为 3589 类 / 23016 方法
（多出 `ObjectStreamClass.getInheritableMethod / getPrivateMethod <- recv(java/lang/Class)` 缺口及其放开的序列化成员：
`*$SerializationProxy`、`KeyRep`、`java/time/Ser` 等）。单调系统的不动点与求值顺序无关，
说明引擎中存在按当时状态一次性决定、之后不再重新评估的判定（非单调或漏登记读者）。

影响：任何改变推送顺序的结构性优化（如下文实参端口）都会使结果在 3540 与 3589 两个口径之间摆动，
「集合逐项相同」作为验收在顺序敏感存在时不可靠。终态应先消除顺序依赖（定位该判定并改为可重算 / 正确登记读者），
使不动点唯一，再做改变推送顺序的结构优化。

### 根因（已查实）

判定位置：`engine/invoke.rs::reflective_writes` 的按名查方法分支。名字实参是 `V::Str` 时记为「本调用点字面量」
（`site_names`），与 Class 接收者值集里的类镜像相乘（`recv_mirrors` → `reflect_name`），非镜像 Class 值记缺口；
名字来自形参时（`Src::Param`）按设计**不**与接收者镜像相乘，只记 `recv(param-name)` 缺口。

非单调来源：形参常量格 `pvals`（`PV::join`，两个不同字符串合流即 Top）在分析时把 Const 形参直接代成 `V::Str`，
与 ldc 字面量无法区分。`ObjectStreamClass$2.run` 依次以 `"writeObject"` / `"readObject"` / `"readObjectNoData"`
调 `getPrivateMethod(cl, name, …)`、以 `"writeReplace"` / `"readResolve"` 调 `getInheritableMethod`：

1. 首个调用点接入后 `pvals[name] = Const("writeObject")`，被调方按此分析，`@3 getDeclaredMethod` 的名字是 `V::Str`，
   按「本调用点字面量」与**当时**的接收者镜像集相乘，放开各镜像类的 `writeObject` 并记缺口；
2. 后续调用点接入，`pvals` 抬为 Top，被调方失效重分析，名字变为形参来源，站点不再满足相乘条件；
3. 第 1 步已放开的成员与缺口保留（效果只增不撤），之后接收者值集的增长不再触发相乘。

于是结果取决于第 1 步发生时接收者值集已长到多大：flow-batch 7 时已含 `java/lang/Class` / `Class#<synthetic>` 及
更多镜像（3081 个类），64 时较少（3040 个类）。插桩实测两次运行均只有 `writeObject`、`writeReplace` 两个名字
被相乘（各 1 次，即只在 Const 窗口内）；同一机制另见 `Proxy$Dyn.dispatchObject @5 "toString"`（仅 batch 7 出现）。
`pstrs.rs` 模块注释已指出 `pvals` 汇合格的顺序依赖并为名字集另建了单调的形参字符串集，但这条
「Const 形参被当成站点字面量」的路径仍走 `pvals`。

终态修法（待协调方确认，涉及序列化反射面）：名字与类都来自形参时，按**调用点逐个配对**——
每个（直接或经透传链的）调用点上该名字形参的字面量，与同一调用点上 Class 实参的镜像集相乘；
调用点新增、调用点的 Class 实参值集增长时重跑。这样 `writeObject` / `readObject` / `readObjectNoData` /
`writeReplace` / `readResolve` 五个名字都与 `cl` 的镜像集配对，与求值顺序无关；
`pvals` 代入的 Const 字符串不再作为站点字面量（站点字面量只认本方法内的 ldc 来源）。

## 后续步骤（规划）

1. 定位顺序依赖的判定：对 flow-batch 64 / 7 两次运行逐阶段对照，找出首个分歧的值集 / 判定，改为单调可重算；
   修正后的唯一不动点作为新的集合基准（需与协调方确认基准变更）。
2. 调用点实参端口：字节码调用点实参汇入单个端口节点，端口流向当前枢纽；换接子枢纽后撤去端口到祖先枢纽的冗余边
   （祖先经子 → 父实参链收到相同值）。已实现原型，集合在顺序敏感问题解决前无法做逐项验收。
3. open 枢纽去冗余：调用点 open 类型集里被其它 open 类型涵盖（子类型）的枢纽不再接入。
4. 精确集合枢纽的父枢纽从「本调用点原枢纽」推广为同成员族内最大的子集枢纽，消除平行的大扇出枢纽。
5. `hub_recv` 逐接收者克隆调用点表、`link_hub` 的 `special` 表收集排序。

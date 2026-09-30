# 生成器效率：发射层与下游编译成本

> 分支 `emitter-perf`。上位文档：[`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md) §三.3。
> 根本原则（用户 2026-10-01）：目标是降低耗时、内存和成本，但正确性不可让步——生成代码必须能编译、能运行，输出与 JVM 一致。
> 改生成形态的每一步都要论证语义等价，并列出需要主会话 e2e 抽查的用例；不确定的形态改动先不做，写进本方案。

## 一、终态目标（量化）

| 指标 | 终态 | 现状（P5 后） |
|---|---:|---|
| 发射阶段墙钟（`rava emit`，任一 e2e 用例，热写出） | ≤ 2 s | ✅ 最重的 DeepCopy 1.99 s（冷写出 2.13 s，见 §五） |
| 发射阶段峰值 RSS（任一用例） | ≤ 500 MB | ✅ 最大 315 MB（DeepCopy） |
| 复用 scratch 时内容未变文件的重写数 | 0 | ✅ 0 / 1727（Digester） |
| 发射的输出确定性 | 27 例生成树逐字节一致 | ✅ 每步验收 |
| 下游 cargo 编译成本 | 见 §四（由 runtime / 宏样板主导，终态需跨线协调） | D1 已做：HelloWorld user 32.4 → 26.9 s |

计量方法：`scripts/emit_bench.sh <out> [Test…]`（`SKIP_BUILD=1` 复用 closure 输入），记录 `/usr/bin/time -l` 的墙钟、user、sys、峰值 RSS、**指令数**（instructions retired，不受机器负载影响，作 A/B 主指标）和周期数，以及 `--perf` 分阶段毫秒。机器为共享机，负载约 8，墙钟有 ±10% 噪声。

## 二、P0 基线与剖析结论

### 2.1 基线（P0，`rava build --perf` 与 `rava emit`）

| 用例 | 模式 | 墙钟 s | user s | RSS MB | closure ms | classes ms | phase2 ms | write ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| HelloWorld | build | 1.08 | 1.19 | 103 | 173 | 242 | 94 | 20 |
| Digester | build | 17.73 | 15.90 | 661 | 8798 | 4110 | 2743 | 190 |
| DeepCopy | build | 159.75 | 145.29 | 2642 | 145995 | 3036 | 2456 | 110 |
| CollectorsDemo | build | 8.85 | 9.13 | 666 | 3995 | 2364 | 1467 | 81 |
| TestCompletableFuture | build | 8.79 | 9.02 | 696 | 3495 | 2453 | 1851 | 110 |
| Digester | emit-warm | 5.69 | 5.35 | 281 | — | 2978 | 2246 | 37 |
| DeepCopy | emit-warm | 5.88 | 5.71 | 341 | — | 3051 | 2431 | 36 |

结论：`rava build` 的耗时和峰值内存由闭包分析主导（归闭包性能线）。发射层自身 4–6 s，集中在 `classes`（逐类生成）和 `phase2`（成员补全 / 导入）两段。

### 2.2 剖析（macOS `sample`，Digester / DeepCopy）

| 热点（P0） | 占比 | 处理 |
|---|---:|---|
| 逐次 `Regex::new`（手写属性扫描、struct 泛型、`fn` 探测等，每类每方法重编译） | 约 55% | P1 |
| `code()` 规范化指令整份克隆（引用收集、跨包导入、inherit / sam / bridge 各扫一遍） | 约 8% | P2 |
| use 索引逐类重建、impl 名集克隆、`indent` 逐行分配 | 约 6% | P2 |
| 系统分配器（malloc / free 小对象） | 约 12% | P3 |
| 变量提升重复渲染条目取引用名；重载名改写逐调用重算 | 约 6% | P4 |

P4 之后的剖析是平的（DeepCopy 约 1790 样本）：方法体翻译（`gen_method_body`，块结构化 + 栈模拟）占 44%，phase2 占 12%，`gen_cross_imports` 占 7.5%，`collect_referenced` 占 6%（其中 `EmitCtx::extras` 按类二次解析 2.7%），手写层 `syn` 解析约 3%，写盘约 6%（冷写出）。已经没有单点能再省 10% 以上。

## 三、已完成步骤（发射层，输出逐字节不变）

每一步的验收：`scripts/gen_trees.sh` 生成 27 例生成树，`scripts/compare_trees.sh` 与改前逐字节对照（另含 Cargo.toml 的 `diff -rq` 全量对照），并且 raw-audit 一致。

| 步骤 | 提交 | 内容 |
|---|---|---|
| P0 | 56927f31 | `rava build/emit --perf` 分阶段耗时、峰值 RSS、逐类 / 逐方法 Top-N；`scripts/emit_bench.sh` |
| P1 | 4249cbf5 | 动态正则改为定形扫描（`emit/src/scan.rs`，与原正则语义逐字等价，单测以 regex 对拍） |
| P2 | 441272f5 | 只读指令视图 `CodeOps`（免克隆）、`UseIndex` 缓存、`impl_fns_contain` 免克隆、`indent` 单次预分配 |
| P3 | d59a2239 | rava 二进制改用 mimalloc 全局分配器 |
| P4 | f7e6bf3d | 变量提升只读扫描缓存条目引用名（`RefCache`，Pass 4 改写条目处不用缓存）；`mangle_if_overloaded` 按（类、方法、描述符）缓存 |
| P5 | 0af99c2a | 热写出零重写：根 `lib.rs` 由 mod 树阶段按「手写真源 + 顶层包补全」经 `Writer` 整体写出；陈旧手写清扫从 overlay 移到本轮写出之后（此前模块资源表等无标记生成文件被先删后写） |

### 3.1 逐步对照（emit，指令 G / 墙钟 s / 峰值 RSS MB）

| 用例 | 模式 | P0 | P1 | P2 | P3 | P4 | P5 |
|---|---|---|---|---|---|---|---|
| HelloWorld | warm | — / 0.50 / 83 | — / 0.35 / 83 | 4.9 / 0.33 / 84 | 3.6 / 0.30 / 78 | — | 3.5 / 0.27 / 78 |
| Digester | cold | 96.9\* / 7.69 / 277 | 41.3\* / 2.78 / 315 | 36.3 / 2.52 / 317 | 25.4 / 2.16 / 282 | 24.0 / 3.07† / 283 | 23.8 / 1.90 / 283 |
| Digester | warm | — / 5.69 / 281 | — / 2.82 / 315 | 35.6 / 2.40 / 317 | 24.8 / 2.01 / 282 | 23.5 / 2.09 / 284 | 22.7 / 1.76 / 284 |
| DeepCopy | cold | 106.9\* / 5.92 / 342 | 46.6\* / 3.44 / 333 | 40.6 / 2.76 / 347 | 28.4 / 2.32 / 313 | 27.0 / 2.53 / 315 | 26.6 / 2.13 / 315 |
| DeepCopy | warm | — / 5.88 / 341 | — / 2.95 / 337 | 40.0 / 2.68 / 349 | 27.8 / 2.26 / 313 | 26.1 / 2.49 / 315 | 25.3 / 1.99 / 315 |
| CollectorsDemo | warm | — / 4.10 / 270 | — / 2.22 / 277 | 29.7 / 1.93 / 271 | 20.6 / 1.62 / 248 | — | 18.7 / 1.44 / 248 |
| TestCompletableFuture | warm | — / 4.69 / 288 | — / 2.46 / 291 | 31.7 / 2.08 / 293 | 22.1 / 1.75 / 260 | — | 20.2 / 1.57 / 263 |

\* P0 / P1 的指令数是 P2 加入指令列之前用同一二进制补测的值。† 该次测量受机器负载峰值影响，指令数正常。

合计：发射指令数降到基线的约 24%（Digester 96.9 → 22.7 G），墙钟 5.7–5.9 s 降到 1.8–2.0 s，峰值 RSS 不升（DeepCopy 341 → 315 MB）。

### 3.2 写出去重

- `Writer::write` / `write_bytes`、overlay 的 `copy_if_changed` 均做「内容相同不写」，保留 mtime，下游 cargo 不会因发射而重编未变的 crate。
- P5 前，Digester 热写出仍重写 2 个文件：
  - 根 `lib.rs`：overlay 先复制手写原文，补全阶段再追加顶层包。
  - `jdk_resources/module_resources.rs`：overlay 按「无生成标记且不在手写真源」判为陈旧，先删后写。
- P5 后重写数为 0 / 1727（`/tmp` 下的双次 emit + `find -newer` 检查）。

## 四、下游编译成本

### 4.1 量化（HelloWorld，249 类；`-Z time-passes`，java_runtime crate）

| 阶段 | 秒 |
|---|---:|
| 合计 | 22.6 |
| LLVM passes | 5.5 |
| codegen_crate | 5.4 |
| type_check | 4.5 |
| borrowck | 4.3 |
| metadata | 3.2 |
| macro_expand | 3.1 |
| monomorphization | 2.3 |

- 源码规模：宏展开前 4.7 MB，展开后 24.6 MB（约 5.2 倍）。
- LLVM IR：约 420 万行，7.4 万个函数。

### 4.2 IR 构成（按函数归类）

| 类别 | 占 IR 行 | 归属 |
|---|---:|---|
| 宏生成的逐类 `__` 样板（`__unsafe_ref_update` 299k 行、`__unsafe_ref_set` 235k、`__shallow_copy` 130k、`__unsafe_ref_get` 92k、`__erased_vtable`、`__view_*` 等） | 39.8% | `runtime/rava_macros`（本线禁改） |
| 方法体、手写代码、From impl | 36.2% | 生成器 / runtime |
| std / core 泛型实例化 | 17% | — |
| runtime 同步 / gil 泛型（`gil::clinit_enter::<T>` 按类单态化，其中 `Vec<(&str,ThreadId)>::retain_mut` ×243 = 5.4 万行） | 4.5% | `runtime/`（本线禁改） |

其中 `panic!("stub: …")` 存根函数占 IR 的 7.1%：6150 个函数，IR 行数中位数 49。存根本身只有一行，IR 大是因为按值传入的参数在 unwind 路径要 drop，生成 landing pad。Digester 共有 10,657 个存根：普通方法 9,327、`new` 665、`__init_on` 665。

结论：下游 rustc 成本主要来自 runtime 与宏拥有的逐类样板（约 44%）。生成器侧能直接改的方法体部分，在不改语义的前提下没有可以大幅削减的形态。

### 4.3 已实施：D1 Cargo profile（提交 6d9fe0c6）

- 根 Cargo.toml 写入：
  ```toml
  [profile.dev]
  debug = "line-tables-only"
  incremental = false
  ```
- `rava build` 在生成类数 ≥ 1700 且未显式设置 `CARGO_BUILD_JOBS` 时设为 1，与 `scripts/cargo_env.py` 同阈值，防止重型工作区内存峰值过高。
- 语义：**不影响运行期语义**。
  - `debug` 只改调试信息级别，panic 回溯仍带文件行号。
  - `incremental` 只影响编译缓存。
  - 作业数只影响并行度。
  - 没有改动 `opt-level`、`overflow-checks`、`panic`、`debug-assertions`，生成代码的算术溢出检查、unwind 行为、断言与改前一致。
  - 此前 `scripts/main.py` 已经以环境变量设置这两项。写进生成物后，`rava build` 与手工 cargo 也一致。
- 生成树差异类别：27 例都只有根 Cargo.toml 增加 `[profile.dev]` 段，其余逐字节一致，raw-audit 一致。
- 编译耗时（cargo build dev，`CARGO_BUILD_JOBS=2`，重编 java_runtime + user，user 秒）：

| 用例（类数） | debug=2（rava build 原状） | line-tables-only |
|---|---|---|
| HelloWorld（249） | 32.4 | 26.9 |
| TestStreamBasic（398） | 55.6 / 42.6 / 49.2 | 43.1 / 39.0 / 36.2 |

- 其余 profile 变体（HelloWorld，user 秒）：
  - `debug=0`：27.7，与 line-tables-only 持平，但失去回溯行号，不取。
  - `codegen-units=16`：24.8。
  - `codegen-units=4`：24.0，但峰值 RSS 1.9 GB，超出内存预算，不取。
- TestStreamBasic 新树 `cargo check` 通过（22.6 s user）。

### 4.4 未做（不确定，或归属其他线），列入待办

| # | 事项 | 预期收益 | 为何未做 / 归属 |
|---|---|---|---|
| X1 | `overflow-checks = false` / `panic = "abort"` | LLVM 阶段 10–20% | **语义不确定**：生成代码或手写层可能依赖 debug 溢出 panic、依赖 unwind 实现 Java 异常 / `catch_unwind`。未做，需逐项论证后由用户决定 |
| X2 | 宏样板瘦身（`__unsafe_ref_*` / `__shallow_copy` 改为泛型共享实现，或按需生成） | IR 约 40% 中的大部分 | `runtime/rava_macros` 属 runtime 线 |
| X3 | `gil::clinit_enter::<T>` 去泛型（按 `&'static str` / TypeId 调非泛型内核） | IR 约 4% | `runtime/` 属 runtime 线 |
| X4 | 存根瘦身：参数改借用、共享 `#[cold]` 非泛型 panic 函数，或不生成未被引用的存根 | IR 约 7% | 删除存根与 CLAUDE.md 存根原则冲突，属**用户决策**；改参数传递方式会改 vtable 签名形态，**不确定**，未做 |
| X5 | `codegen-units` 调整 | user 约 −8% | 峰值内存与收益随用例变化，需全量实测后再定 |

## 五、未完成与后续

| # | 事项 | 说明 |
|---|---|---|
| N1 | 按类并行发射（输出确定） | 被阻塞：`resolve`（闭包 crate 依赖，本线禁改）、ty `Registry`、`EmitCtx` 使用 `Rc` / `RefCell`；`ProjectState.seen_simples` 跨类累积，与发射顺序相关。终态做法：共享只读上下文改 `Arc` + 线程安全只读缓存（`OnceLock` / 分片），`seen_simples` 改为先并行生成、后按原序做一次确定性的导入冲突裁决。需要与闭包线协调 `resolve` 的 `Rc → Arc` |
| N2 | 闭包结果进程内传递 | `rava build` 现在由闭包写出 closure.json，发射再读入并二次解析手写层（`syn` 约 3%）、按类二次解析补充属性（`extras` 约 2.7%）。终态由闭包直接交出内存结构，手写层解析结果复用。需要闭包 crate 暴露接口（闭包线） |
| N3 | 冷写出 DeepCopy 2.13 s 仍略高于 2 s | 剩余热点是平的。可以做的小项：`Acc::precise` 按类名缓存包路径 / 短名（约 2%）、`collect_referenced` 免重复插入、phase2 `fill_slot` 分配。N1 落地后（2 线程）会有充分余量 |

## 六、需要主会话 e2e 抽查的用例

- **D1**（改动所有用例的根 Cargo.toml，并在重型工作区单作业）：
  - `HelloWorld`、`TestStreamBasic`：普通工作区的 profile 生效。
  - `DeepCopy`：≥ 1700 类，经 `rava build` 走 `CARGO_BUILD_JOBS=1` 分支。
  - 任选一个依赖 panic 回溯 / 异常路径的用例（如 `TestSuppressed`、`TestNestedTry`）：确认 unwind 语义未变。
- **P3**（全局分配器）与 **P5**（lib.rs / 陈旧清扫时机）：影响所有生成器运行，生成树已逐字节一致。
  - 抽查一例复用 scratch 的连续两次运行（不加 `--clean`），确认第二次 cargo 不重编 java_runtime。
  - 抽查一例在 runtime/ 删除手写文件后的复用 scratch 运行（陈旧手写清扫）。

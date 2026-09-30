# 生成器效率：发射层与下游编译成本

> 分支 `emitter-perf`（发射层、D1），`emitter-perf2`（下游编译成本 §4.4–4.7）。上位文档：[`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md) §三.3。
> 根本原则（用户 2026-10-01）：目标是降低耗时、内存和成本，但正确性不可让步——生成代码必须能编译、能运行，输出与 JVM 一致。
> 改生成形态的每一步都要论证语义等价，并列出需要主会话 e2e 抽查的用例；不确定的形态改动先不做，写进本方案。

## 一、终态目标（量化）

| 指标 | 终态 | 现状（P5 后） |
|---|---:|---|
| 发射阶段墙钟（`rava emit`，任一 e2e 用例，热写出） | ≤ 2 s | ✅ 最重的 DeepCopy：按类并行后热写出 1.02 s、冷写出 1.08 s（串行时 1.99 s / 2.13 s，见 §5.1） |
| 发射阶段峰值 RSS（任一用例） | ≤ 500 MB | ✅ 最大 396 MB（DeepCopy，并行；`--emit-jobs 1` 时 327 MB） |
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

### 4.4 事项状态

| # | 事项 | 状态 |
|---|---|---|
| X1a | `panic = "abort"` | ✅ 已做（§4.5 第 5 步），论证见 §4.6 |
| X1b | `overflow-checks = false` | **不做**：实测无收益（§4.6），且保留它可以检出手写层 usize 运算缺陷 |
| X2 | 宏样板瘦身 | ✅ 引用字段协议已做（§4.5 第 1 步）；`__shallow_copy` / `From<Object>` / `__erased_vtable` / `__view_*` 未做，见 §五 N4 |
| X3 | `gil::clinit_enter::<T>` 去泛型 | ✅ 已做（§4.5 第 2 步） |
| X4 | 存根瘦身 | ✅ 改为调用共享冷函数（§4.5 第 3 步）。存根仍然生成、仍 panic、消息不变；参数传递方式不改 |
| X5 | `codegen-units` 调整 | 未做：峰值内存与收益随用例变化，需全量实测后再定 |
| N1 | 按类并行发射 | ✅ 已做（提交 2a800b4b），输出与串行逐字节一致，见 §5.1 |

### 4.5 分支 `emitter-perf2`：下游编译成本的逐步实施

计量对象：HelloWorld 的 java_runtime crate。
- 确定性指标：`-Z dump-mono-stats` 的 size_est 合计。
- 耗时 / RSS：与基线 worktree（d145f609）交替跑（ABAB）。
- 共享机上其他线同时编译，墙钟与 RSS 只看同批对照。

| 步 | 提交 | 改动 | mono size_est | 条目数 |
|---|---|---|---:|---:|
| 0 | d145f609 | 基线 | 990,227 | 31,051 |
| 1 | 8f022dd5 | 引用字段 `__unsafe_ref_get/set/update` 三个按名协议合为单一分派 `__unsafe_ref_access(field, &mut __RefAccess)`，逐字段分支调非泛型化的 `__ref_slot_access` | 854,413（−13.7%） | — |
| 2 | 394619fd | `clinit_enter` / `clinit_exit` 以 `&'static __PrimCell<u8>` 取状态，不再按类单态化；静态字段读写经 `__GilStatic::force` | 758,805（−11.2%） | — |
| 3 | 84fc5581 | 存根 `panic!("…")` → `__stub("…")`（`#[cold] #[inline(never)] #[track_caller]`，入参 `&'static str`）；`checkcast` 失败出口改非泛型的 `checkcast_fail` | 750,474 | 26,214 |
| 4 | 2d6af212、0844b55b | 手写层正确性修复（§4.6），不以性能为目的 | — | — |
| 5 | ee90ef5f | 两个 profile 都设 `panic = "abort"`，并由 `create_java_vm` 登记退出码 101 的 panic 钩子 | −25.6%（同批对照 434,653 → 323,540） | — |

注：第 5 步的 size_est 取自重建后的计量区，统计口径与第 0–3 步不同，只看同批相对值。

步 1–3 合计：size_est −24.2%，条目数 −15.6%。

步 1–3 的 ABAB 编译对照（HelloWorld java_runtime，`cargo rustc` dev）：

| 指标 | 基线 | 步 1–3 后 |
|---|---:|---:|
| rustc 合计 | 18.3 s / 17.4 s | 15.6 s / 16.5 s（约 −10%） |
| 峰值 RSS | 2,336 MB / 2,338 MB | 2,010 MB / 2,009 MB（−14%） |

步 1–3 后各阶段耗时：type_check 3.8 s、LLVM 3.6 s、codegen 3.5 s、borrowck 3.4 s、expand 2.6 s。

步 5 的 ABAB 编译对照（cargo build dev，重编 java_runtime + user，`CARGO_PROFILE_DEV_PANIC` 切换，其余相同）：

| 用例 | unwind（user s / 峰值 RSS） | abort（user s / 峰值 RSS） | 二进制 |
|---|---|---|---|
| HelloWorld（5 组） | 20.73 / 19.93 / 22.80 / 20.45 / 20.56 s；1.4–1.9 GB | 18.84 / 18.01 / 18.42 / 18.13 / 18.46 s；1.4–1.7 GB | 24.2 MB → 18.3 MB（−24%） |
| TestCompletableFuture（2 组） | 139.5 / 138.8 s；5.6–6.6 GB | 111.5 / 141.8 s；4.8–7.1 GB | 175.3 MB → 129.5 MB（−26%） |

- HelloWorld：user 时间稳定下降约 10%。
- TestCompletableFuture：第二组 abort 与其他线的编译重叠，噪声大于差值。确定性指标（mono size_est −25.6%、二进制 −26%）方向一致。

验证（步 1–3）：
- 27 例生成树与基线相比，runtime overlay 以外只有 `panic!("…")` → `__stub("…")` 一类差异：单行形态 169,881 处，缩进形态 345 对。
- runtime 文件差异只在 `object.rs`、`lib.rs`、`sync_model.rs`、`gil.rs`、`object_ext.rs`。
- raw-audit 逐行一致。
- `cargo check` 全部通过：TestAtomics（144 s，3.09 GB）、TestCompletableFuture（231 s，1.98 GB）、TestStreamCollectors（154 s，2.73 GB）。

验证（步 4、5）：TestCompletableFuture 树 `cargo check` 通过，HelloWorld 与 TestCompletableFuture 树 dev `cargo build` 通过。27 例树对照见 §4.7。

语义等价论证：
- **步 1**：协议只改分派形态，不改语义。
  - 取值用读锁；设值先在锁外完成类型转换，再取写锁；更新在锁内转换。三者的原子性与原 `__unsafe_ref_get/set/update` 相同。
  - 命中字段返回 `Some`，未命中返回 `None`。wrapper 先问内层存储，未命中再委托 vtable，与原先的逐方法委托顺序相同。
  - 调用方经 `impl dyn ObjectVTable` 上的同名包装方法调用，调用点源码不变。
- **步 2**：`clinit` 状态机的判定、等待、异常包装逻辑不变，只把状态单元改为经 `&'static` 引用传入。`force` 与原 `with` 的惰性初始化是同一 `OnceLock` 语义。
- **步 3**：`__stub` 与原 `panic!` 输出同一消息（`stub: 类.方法:描述符`）。`#[track_caller]` 使 panic 位置仍报存根所在行。`checkcast_fail` 抛出的 ClassCastException 消息构造与原内联代码逐字相同。
- **步 5**：见 §4.6。

### 4.6 `overflow-checks` / `panic = "abort"` 论证

**整数运算审计**

生成器侧已全部显式：
- `iadd/isub/imul/ineg/ladd/lsub/lmul/lneg` 与 `iinc` 用 `wrapping_*`。
- 移位先掩码（`&0x1f` / `&0x3f`）；long 移位用 `wrapping_shl/shr`。
- `idiv/irem/ldiv/lrem` 经运行时函数（`wrapping_div/rem` + 除零 ArithmeticException）。只有正的非零字面量除数保留裸运算符，这种情况不会溢出。
- 窄化与类型转换用 `as`，与 JVM 的截断语义一致。
- 浮点运算为裸运算，不涉及溢出检查。
- `rava_macros` 不发射整数算术。

手写层：共 176 个文件，筛出 311 行含裸 `+ - *`，逐行分类如下。
- 绝大多数是 Rust 内部的 usize / 迭代下标运算，或 OS 取值的换算。这类运算溢出即是运行时缺陷，与 Java 语义无关。
- 以 Java 实参为操作数、可能溢出的有 12 个文件，已在 0844b55b 改为显式形态：
  - `off..off + len` 循环区间改 `saturating_add`（涉及 StreamEncoder.write、FileOutputStream.writeBytes、Adler32 / CRC32.updateBytes、Deflater / Inflater.copy_in、vectorizedHashCode）。溢出时的行为与未溢出的越界区间相同：逐元素访问到数组末端时抛 AIOOBE。
  - 下标偏移 `base + i` 改 `wrapping_add` / `wrapping_mul`（涉及 System.arraycopy、countPositives、ArraysSupport 各 mismatch、ByteArray、Deflater / Inflater 输出）。回绕后的负下标由 `JArray::get/set` 抛 AIOOBE，与 JVM 的下标检查一致。
  - `StreamDecoder.read` 的 `offset + length > cbuf.length` 改为 i64 拓宽比较。溢出的区间现在抛 IOOBE；原先在 overflow-checks 下会 panic，与 JDK 的 `(off + len) < 0` 判定对齐。
  - `Unsafe` 的 `alignToHeapWordSize` 改为 `bytes.wrapping_add(7) & !7`，与 Java 的 long 回绕逐位一致。
  - `decimal_digits_impl` 采用 JDK 的负数形式，对 `MIN_VALUE` 也不会溢出，无需改动。`Preconditions` 的 `length - fromIndex` 在两个操作数都非负后才计算，不会溢出。`VM.getNanoTimeAdjustment` 先按 ±2^32 秒截断再乘，也不会溢出。
- 审计中发现一处与溢出检查无关的真缺陷，已在 2d6af212 修复：`Object.wait` / `Unsafe.park` / `Thread.sleep0` / `VirtualThread.joinNanos` 以 `Instant::now() + Duration` 计算截止时刻。Java 超时可取到 `Long.MAX_VALUE`，此时加法越界，任何 profile 下都会 panic。现在统一经 `monitor::deadline_after` 截到 2^32 秒（约 136 年，观测上等同无限期）。

**`overflow-checks = false`：不做**

- 经上述修正后，生成代码与手写层的 Java 语义运算都不再依赖这个开关，关掉它在语义上是安全的。
- 实测没有收益：mono size_est 434,653 → 434,066（−0.13%），耗时差落在噪声内。原因是生成代码本身已全部使用 `wrapping_*`，没有溢出检查可省。
- 保持开启还能在 dev 构建里检出手写层 usize 运算的缺陷。终态保持默认值。

**`panic = "abort"`：已做**

依赖 unwind 的出现处，全部列出：
- `catch_unwind` 全仓只有一处：`thread_impl.rs` 的 Java 线程入口。捕获后只判断 `is_err()`，然后 `exit(101)`。
- 没有 `set_hook`、`resume_unwind`、panic payload downcast、`thread::scope`，也没有用 `JoinHandle::join` 取 panic。
- 没有 StackOverflowError 映射。Rust 栈溢出由 SIGSEGV 处理器报告后 abort，这一点与 profile 无关。
- Java 异常全部经 `Result<_, JvmError>` 传播，不走 unwind。

等价设计：
- 生成的 `main` 第一条语句调用 `java_runtime::create_java_vm()`，它登记 panic 钩子：先调用默认钩子（输出 panic 位置、消息与回溯提示），再 `process::exit(101)`。
- 线程入口的 `catch_unwind` 随之删除，任何线程的 panic 走同一出口。

与改前 unwind 形态逐项对照：

| 项目 | 改前 | 改后 |
|---|---|---|
| stderr 文本 | 默认钩子输出 | 同一默认钩子，文本相同 |
| 主线程 panic 退出码 | lang_start 返回 101 | 101 |
| 子线程 panic 退出码 | catch_unwind 后 `exit(101)` | 101 |
| 标准输出冲刷 | 进程退出时冲刷 | 由 `process::exit` 的 `rt::cleanup` 完成 |
| unwind 期间运行的 Drop | 会运行 | 不再运行；Java 对象没有析构语义，进程随即退出，无可观测差别 |

- 以上各项已用独立探针 crate 验证：panic=abort 加同形钩子，主线程与子线程 panic 都以退出码 101 结束，panic 前未换行的 `print!` 输出已冲刷，stderr 文本与默认形态相同。
- `run_tests.py` 的 run.log 落盘与 `panicked at` / `stub:` 首行提取依赖非零退出码和 stderr 文本，两者都不变。
- cargo 对测试、构建脚本与 proc-macro 忽略 `panic` 设置，`rava_macros` 与 `build.rs` 不受影响。

### 4.7 27 例生成树对照（基线 d145f609 → ee90ef5f）

`scripts/gen_trees.sh`：基线用 `java_rta_emitter_perf2_base` worktree，新树用本分支。27 例都转译成功，raw-audit 逐行一致。`closure.json` 按惯例排除。差异共 10,831 个文件，逐类列出：

| 类别 | 出现处 | 来源 |
|---|---|---|
| `panic!("…")` → `__stub("…")`，其余逐字节相同 | 169,881 处调用 | 步 3 |
| runtime overlay 文件（手写真源的拷贝） | 20 个文件 × 27：`gil.rs`、`lib.rs`、`monitor.rs`、`sync_model.rs`、`java/lang/{object,object_ext,system_impl,thread_impl,virtual_thread_impl}.rs`、`java/io/file_output_stream_impl.rs`、`java/util/zip/{adler32,crc32,deflater,inflater}_impl.rs`、`jdk/internal/access/java_lang_access_impl.rs`、`jdk/internal/misc/unsafe__impl.rs`、`jdk/internal/util/{arrays_support,byte_array}_impl.rs`、`sun/nio/cs/stream_{decoder,encoder}_impl.rs` | 步 1–5 |
| 根 Cargo.toml 两个 profile 各加一行 `panic = "abort"` | 27 × 2 | 步 5 |
| `user/src/main.rs` 首行加 `java_runtime::create_java_vm();` | 27 | 步 5 |
| `rava_macros` 绝对路径与 scratch 包版本号 | 每例 2 个 Cargo.toml | 由 worktree 路径派生，不是本线改动 |

除以上类别外没有其他差异。

## 五、未完成与后续

| # | 事项 | 说明 |
|---|---|---|
| N1 | 按类并行发射（输出确定） | ✅ 已做，见 §5.1 |
| N2 | 闭包结果进程内传递 | `rava build` 现在由闭包写出 closure.json，发射再读入并二次解析手写层（`syn` 约 3%）、按类二次解析补充属性（`extras` 约 2.7%）。终态由闭包直接交出内存结构，手写层解析结果复用。需要闭包 crate 暴露接口（闭包线） |
| N3 | 冷写出 DeepCopy ≤ 2 s | ✅ N1 落地后达成（1.08 s）。剩余串行段见 §5.1 N6 |
| N4 | 剩余 mono 热点（步 1–3 后） | `drop`（逐 T 的 Weak / Arc 析构）、`__shallow_copy`（内层 12.8k、wrapper 回退 10.8k；回退路径被 `NativeNumberFormatProvider` 等手写 `X__VTable` impl 用到，保留）、`From<Object>`、`__clinit`、`__erased_vtable`、`__view_into`、`__virtual_view`。这些与泛型擦除布局相关，并入拆 crate（泛型擦除为其前提）一并处理 |
| N5 | 按类并行发射（N1）的实施时机 | 协调决定：等 closure-perf2 合入主线后，本线先合主线，再把 `resolve` 的 `Rc` 改为 `Arc`，改完闭包 4 例集合必须一致 |
| N6 | 并行后剩余串行段 | DeepCopy 冷写出 1.08 s 中：输入重建约 100 ms、跨类导入裁决约 120 ms、phase2 约 240 ms 仍串行。跨类导入是「先引入者得短名」语义，必须按发射序；终态可改为并行收集各类候选短名、再按序一次裁决（纯计算约数十 ms）。phase2 按 crate 分片可并行。输入重建并入 N2 |


### 5.1 N1 按类并行发射（emitter-perf2，提交 2a800b4b）

**设计**
- 共享只读上下文线程安全化：`resolve` / `closure` / `input` / `ty` 的 `Rc<ClassFile>` → `Arc`。`ClassPath::get` 读锁快路径；未命中时持写锁复查后加载，保证每类只解析一次、失败只记一次。ty `Registry` 的各缓存改为 `Cache<V>`（`RwLock<BTreeMap<String, Arc<V>>>`，首写者胜）。instr 的 `impl_fn_cache` / `mangle_cache`、emit 的 `extras` / `use_index` 同样处理。这些都是纯记忆化，读到谁写入的值结果都相同。
- 逐类三段：
  1. 并行 `class_prep`：引用收集、手写覆盖、用户同包导入；
  2. 串行 `class_cross_imports`：`seen_simples` 按「先引入者得短名」裁决，与发射序相关，所以保持原序；
  3. 并行 `class_text`：每类使用独立的 `ProjectState` 增量，结束后按发射序 `ProjectState::merge`。
- 方法体审计账本改为按类事件日志 `BodyLog`（`BodyEvent::Method` / `StubFallback`），合并后由 `BodyAudit::from_log` 按序重放。去重、计数语义与串行完全相同。`MethodBodyEmitter` 由此变为无状态（`&self` + `Sync`）。
- `par_map`：原子游标领取下标，结果按下标归位（保序）。工作线程栈 16 MiB（方法体生成有深递归）；子线程 panic 原样上抛。
- 并行度 `--emit-jobs N`（缺省 0 = 可用核数，1 = 串行）。

**验证**
- 闭包 4 例（HelloWorld / Digester / DeepCopy / CollectorsDemo，基线 4c461708）：`closure_bench.sh --diff` 全部 SAME。指令数变化 ≤ 0.3%（DeepCopy 412.26 → 412.65 G）。
- 27 例生成树：`compare_trees.sh` 0 差异，raw-audit 一致。`closure.json` 除 `elapsed_ms` / `perf` 外一致。
- 并行（缺省）、串行（`--emit-jobs 1`）与基线三者的发射输出逐字节一致。审计行除 `[perf]` 外一致。

**实测**（10 核共享机，`emit_bench.sh`，冷写出；ms 为 `--perf` 分阶段）

| 用例 | 指标 | 基线 | 并行（缺省） | 串行 `--emit-jobs 1` |
|---|---|---:|---:|---:|
| DeepCopy | 冷写出墙钟 | 2.10 s | **1.08 s** | 2.03 s |
| DeepCopy | 热写出墙钟 | 2.01 s | **1.02 s** | — |
| DeepCopy | 类阶段 | 1392 ms | 42 + 122 + 238 = 402 ms | 107 + 123 + 1142 ms |
| DeepCopy | 指令数 | 26.7 G | 27.2 G | — |
| DeepCopy | 峰值 RSS | 314 MB | 396 MB | 327 MB |
| Digester | 冷写出墙钟 | 1.84 s | **0.99 s** | 1.82 s |
| Digester | 热写出墙钟 | 1.74 s | **0.86 s** | — |
| Digester | 类阶段 | 1252 ms | 33 + 101 + 236 ms | — |
| Digester | 峰值 RSS | 287 MB | 364 MB | — |

类阶段三项依次为 prep / imports / text。RSS 增加约 80 MB，来自各工作线程并存的类体缓冲与线程栈，仍低于 500 MB 目标。总指令数增加约 2%，为锁与合并开销。

## 六、需要主会话 e2e 抽查的用例

- **N1**（按类并行发射）：27 例生成树与串行逐字节一致，生成形态没有变化，抽查可选。建议正常跑一次 `DeepCopy`（最多类，走并行）和 `TestCompletableFuture`，确认生成器在多线程下无 panic、结果与此前一致。
- **emitter-perf2 步 1**（引用字段协议）：`TestAtomics`、`TestCompletableFuture`、`TestChmTransfer`。这三例覆盖 Unsafe / VarHandle / AtomicReference 的引用字段 get / set / CAS / getAndUpdate。
- **步 2**（clinit / 静态字段）：`TestSynchronized`、`TestCompletableFuture`（多线程下的类初始化），`TestSwitchString`、`TestZonedDateTime`（静态表与枚举初始化）。
- **步 3**（存根 / checkcast）：`TestCasting`（ClassCastException 消息）。另任选一个当前命中存根而失败的用例，确认 stderr 仍含 `stub: 类.方法:描述符`，run.log 首行提取正常。
- **步 4**（手写层）：`TestFilesApi`、`TestDateTimeFormat`（StreamDecoder / Encoder、arraycopy、ByteArray 路径），`TestSynchronized`（wait / sleep / park 截止时刻）。
- **步 5**（panic = "abort"）：
  - 同上任一命中存根的用例：退出码仍为 101，run.log 仍落盘。
  - 一个有子线程的用例（`TestCompletableFuture`）：正常退出码 0，输出完整。
  - 全量跑一次，确认没有用例由「失败」变为 `signal 6`。


- **D1**（改动所有用例的根 Cargo.toml，并在重型工作区单作业）：
  - `HelloWorld`、`TestStreamBasic`：普通工作区的 profile 生效。
  - `DeepCopy`：≥ 1700 类，经 `rava build` 走 `CARGO_BUILD_JOBS=1` 分支。
  - 任选一个依赖 panic 回溯 / 异常路径的用例（如 `TestSuppressed`、`TestNestedTry`）：确认 unwind 语义未变。
- **P3**（全局分配器）与 **P5**（lib.rs / 陈旧清扫时机）：影响所有生成器运行，生成树已逐字节一致。
  - 抽查一例复用 scratch 的连续两次运行（不加 `--clean`），确认第二次 cargo 不重编 java_runtime。
  - 抽查一例在 runtime/ 删除手写文件后的复用 scratch 运行（陈旧手写清扫）。

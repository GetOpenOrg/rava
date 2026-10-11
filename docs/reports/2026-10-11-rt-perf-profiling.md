# 运行时性能深剖（2026-10-11，分支 rt-perf-prof）

> 对象：docs/tasks.md「10-11 晚收拢」第 4 条的三项——`Method.invoke` 微基准、`rd::unbox_*` 的
> `__obj_str` 文本解析、无 GC 第一步后 PartitionInteger 的 stream 管线开销。任务书以「热点定位
> 到行 + 占比数字」为验收；本报告给出同机（kr1）rava 与 JVM 对照、热点函数与行号、占比，
> 以及随附微修复与 unbox 承载方案评估。基线提交 7b0bf81a（batch-1011k + 剖析脚本），
> 修复提交 9157a73e。作业：`cluster_results/job/rtpp-prof1/`（基线剖析）、
> `cluster_results/job/rtpp-verify1/`（修复后验证与 A/B）。
>
> 注：jackson jc1 分派视图约 19% 已归 S7 线（tasks.md 第 4 条），不在本报告范围。

## 一、方法

- 剖析：`scripts/perf_line_profile.sh`——release 档带符号
  （`CARGO_PROFILE_RELEASE_DEBUG=line-tables-only`、strip 关），每例两次采样：F997 无调用栈
  （自采样，`<case>.self.txt`）+ F300 dwarf 调用链（children 视图，`<case>.children.txt`）。
  `perf report -l` 在该构建形态下未产出 srcline 行（符号级行为主）；「定位到行」以符号 →
  本仓库源码位置的逐一映射给出（运行时侧函数行号取自当期源码，翻译体符号标注其 JDK
  方法语义），LLVM 内联使被调方开销部分落在调用方符号内，聚簇解读时已计及。
- 对照微基准：`scripts/perf_rt_bench.sh`——rava 与参考 JVM（SerialGC）同机同口径：
  1. `BenchReflectArg`：`idInt(int)`（拆箱+装箱）vs `idRef(Integer)`（引用直传）vs
     `idInt2(int,int)`——同一分派协议下差值即 unbox/box 边界开销；
  2. `RldcSegments`：TestReflectLdcHotLoop 三段（ldc / 类字面量 / Method.invoke）逐段墙钟；
  3. `BenchStreamLoop`：`Arrays.stream(combo).map(...).sum()` vs 手写 for 循环（每元素 ns）。
- JVM 计时均为参考 JDK 21（SerialGC），5 次取中位数。

## 二、对象 1：Method.invoke 微基准（TestReflectLdcHotLoop）

### 1. 一次 invoke 的完整开销清单（代码侧盘点，基线 7b0bf81a）

`step.invoke(o, i & 255)` 每次迭代经过（源码位置）：

| 步骤 | 位置 | 开销性质 |
|---|---|---|
| `Integer.valueOf(i & 255)`（现场装箱） | 翻译体 | 128..255 缓存外，每迭代新分配翻译 Integer |
| varargs `Object[]` 装配 | 翻译体 | 1 次数组分配（JVM 同） |
| Method.invoke / 访问器链 | 翻译体（Method.invoke → accessor.invoke → invoke0） | 若干次虚分派 + 访问检查 |
| **`__member_key` 描述符重建** | `method_impl.rs:36-58` | **每次 invoke 重建 (类名, 方法名, 描述符) 三个 String：4 次 Java String→Rust String 转换 + 约 5 次分配**（参数 / 返回类型逐个 format + replace + push） |
| `is_static_descriptor` | `reflect_dispatch.rs:279` | 元数据表二分 + **类方法表内线性 find（逐方法 String 比较）**；native_invoke 与 reflect_invoke 各查一次 |
| 引用实参可赋值检查 | `reflect_dispatch.rs:191-203` → `class_impl.rs:304` | `is_instance_of`（描述符超类型表）失败后 `__name_assignable` **线性扫 class_hierarchy 全表**（数千行；已修，§六） |
| 接收者检查 | `reflect_dispatch.rs:141` | `is_instance_of` + `superclass_chain().any()` 沿父类链走 |
| 分派表查找 | `reflect_dispatch.rs:80-83` | 冻结表二分（82f55952 后无锁无克隆） |
| 分派臂 → 目标 | 用户 crate `__reflect_dispatch` | `try_cast` 视图 + `unbox_i32(实参)` |
| **`unbox_i32` 文本往返** | `reflect_dispatch.rs:303`（基线） | **翻译 Integer.toString 体（构建 Java String）+ UTF-16→UTF-8 转换 + `parse::<i32>()`**（P-B 目标，§三） |
| 返回装箱 | `reflect_dispatch.rs:217-233` | 原生盒 downcast + `Integer.valueOf_i`（缓存外新分配） |
| `take_bad_arg` | `reflect_dispatch.rs:294` | TLS Cell 读写 |

### 2. 同机对照与热点占比（kr1，基线 7b0bf81a，作业 rtpp-prof1）

整例墙钟（3 次）：rava 4.25 / 4.31 / 4.31 s，JVM 0.11 / 0.11 / 0.12 s（约 39 倍）。
逐段（RldcSegments 计时副本，n = 1×10⁶，中位）：

| 段 | rava | JVM | 倍数 |
|---|---:|---:|---:|
| ldc（字符串常量驻留 + length） | 31.3 ms（31 ns/迭代） | 7.9 ms | 4.0× |
| 类字面量 / getClass / isInstance | 496 ms（496 ns/迭代） | 3.2 ms | 155× |
| Method.invoke（每次迭代 2 调用） | 3529 ms（**1.76 µs/调用**） | 62.9 ms（31 ns/调用） | 56× |

分诊 C 节所记「约 43 µs/迭代」（perf_hotpath，us1，43f37f63，整例 43.197 s / 1M 迭代；
彼时测试尚用 `hashCode()` 取值绕开反射装箱强转缺陷，6333f82d 已恢复直接强转）与本测
kr1 上 4.25 s 差约 10 倍：分段计时显示三段合计 + 启动即构成整例，kr1 无单段能达到 40 s
量级，疑为彼时提交 lineage（43f37f63 → batch-1011k 之间并入 fix-invoke-box-cast 等）或
机差所致，**未闭合**——如需精确归因，可在 kr1 补一次 43f37f63 对照（遗留，非本步必需）。
以本报告 kr1 同机数字为准：**1.76 µs/调用**。剩余 56 倍的构成（受控对照
BenchReflectArg，ns/调用，中位）：

| 测点 | rava | JVM | 差值含义 |
|---|---:|---:|---|
| idRef(Integer)（引用直传） | 1732 | 20 | 协议机底座：member_key 重建 + 双重 static 判定 + 实参校验 + 上溯分派 |
| idInt(int)（拆箱+装箱） | 1910 | 34 | 边界开销 = idInt−idRef = **178 ns**（valueOf 新分配 + unbox + box_value） |
| idInt2(int,int) | 2465 | 29 | 每加一个 int 实参 +555 ns（第二个 valueOf 新分配 + unbox + 边界） |

自采样前三热点（F997，全二进制约 82% 样本在 invoke 段；行号为本仓库源）：

1. **String 构建 / 转换簇 ≈ 17%**：`String` 的 `Display::fmt` 7.05%（`string_ext`）+
   `String::units` 3.48% + `core::fmt::write` 2.27% + `String__inner` vtable 胶水 2.16% +
   `String.__init_on_arr` 1.23% + `write_str`/`Display` 0.91%——主要来自
   **`__member_key` 每次 invoke 重建 (类名, 方法名, 描述符)**（`method_impl.rs:36-58`，
   经 `__reflect_key`（`:62`）← `invoke0`（`native_accessor_impl.rs:27`）每调用执行：
   3–4 次 Java String→Rust String 转换 + 约 5 次堆分配）与 unbox 文本路径的转换份额。
2. **分配器簇 ≈ 16%**：malloc 6.46% + cfree 3.91% + realloc 1.46% + raw_vec
   reserve/finish_grow 2.95% + format_inner 1.0% + calloc 0.41%（varargs 数组
   `Store::to_vec` 1.52% + `JArray::to_vec` 0.54% 为其需求方之一）。
3. **分派协议簇 ≈ 10%**：`is_static_descriptor` 2.27%（`reflect_dispatch.rs:279`，
   每调用两次：`native_invoke:180` 与 `reflect_invoke:116`）+ `lookup` 1.09% +
   `native_invoke` 1.05% + `reflect_invoke` 0.73% + `descriptor_params` 胶水 0.80% +
   用户 `__reflect_dispatch` 臂 0.71% + `try_cast` 0.52% + `box_value` 0.57% +
   `handle::release_last` 0.91%。

其余可辨识簇：`invoke0` 本体 6.42%（含 member_key 内联）、Integer 装箱簇 ≈ 6%
（`valueOf_i` 1.23 + `getChars` 1.05 + `toString_i` 0.89 + `__init_on_i` 0.84 +
`AbstractStringBuilder` 1.23）、`Method_Direct.invoke` 翻译体 1.34%、
`__caller_sensitive` 压栈 0.68%、`injected_invoker::registered` 0.59%、
`image_rt::image_mirror` 0.68%、`exec_context::current` 0.39%。

**结论**：56 倍中约四分之一在「每次 invoke 重建分派键」（member_key），约六分之一在
其引发的分配，约十分之一在分派协议本身（含两遍 static 判定）；装箱本身（valueOf）约 6%。
类字面量段 155×（496 ns/迭代：两次镜像比较 + isInstance）与 ldc 段 4× 相对次要。

## 三、对象 2：rd::unbox_* 解析 __obj_str 文本

### 1. 开销形态（基线）

翻译包装类实例（`Integer.valueOf` 产出的翻译 Integer，非原生值盒 `Rc<i32>`）在 `unbox_i32`
等入口的前四个 downcast 全部落空，落入 `v.0.__obj_str().parse()`：

1. `__obj_str` 桥接（`rava_macros_core/src/block/gen/struct_layout.rs:166`）调**翻译体
   `Integer.toString`**——按值逐位构建 Java String（UTF-16 缓冲分配）；
2. `ToString::to_string(&s)`——Java String → Rust String 的 **UTF-16→UTF-8 转换 + 分配**；
3. `str::parse::<i32>()` 文本解析。

即一次拆箱 = 一次完整的「值→文本→值」往返（≥3 次分配 + 数位格式化 + 解析）。

### 2. 方案评估

- **P-A「ObjectVTable 原生值视图臂」（结构化承载，终态方案）**：`ObjectVTable` 增
  `__prim(&self) -> Option<PrimVal>`（默认 None），rava_macros 对包装类桥接到其 `intValue()`
  等翻译体，`rd::unbox_*` 先问视图。优点：直取值、无契约耦合、覆盖 Long/Double；代价：
  vtable 增一臂（全体类付槽位税）、宏与包装类桥接属协议层改动、需与 S7 K 系（方法 trait /
  vtable 形态演进）协调。**建议随 S7 线排期，不单独做。**
- **P-B「hashCode 结构化取值」（本步落地的可回退原型，`reflect_dispatch.rs:298` 起）**：
  包装类均 final 且 hashCode 契约与值一一对应（Integer/Byte/Short 恒等、Character=码元、
  Boolean 1231/1237、Float=floatToIntBits），经 hashCode 虚分派一次取回值位；Long/Double
  折叠到 int 不可逆，仍走文本。改动集中在 unbox 六函数内，接收类名集合与原路径一致
  （不新增可拆箱类型），单 commit 可整体回退。
- **缓存方案（否决）**：对 `(对象身份 → 已解析值)` 做侧表缓存——侧表键为地址，地址复用
  需 pin 对象防假命中（泄漏随 distinct 包装实例无界增长）；包装类不可变但实例海量
  （循环装箱即海量），缓存收益不如 P-B 且引入生命周期风险，不取。

### 3. 量化（kr1，基线 7b0bf81a，作业 rtpp-prof1）

- **边界开销**：`idInt − idRef = 178 ns/调用`（含 `Integer.valueOf` 缓存外新分配与
  `box_value`，unbox 文本往返为其中可去除的主要部分）。
- **self 归属**：unbox 文本路径的可辨识片段 = `Integer.toString_i` 0.89% +
  `Integer.getChars` 1.05% + `AbstractStringBuilder` 1.23% + `String.__init_on_arr`
  1.23% + String `Display::fmt`/`units` 中 `__obj_str` 的 to_string 份额，
  合计约 4–5% 全样本（invoke 段占 82% ⇒ 折合每次调用约 80–100 ns）。

### 4. A/B 结果（P-B 落地后，作业 rtpp-verify1）

（待填：idInt / idInt2 / RldcSegments invoke 段修复前后对照）

## 四、对象 3：PartitionInteger release 48 s（JVM 0.65–0.70 s）

### 1. 同机对照（kr1，基线 7b0bf81a，作业 rtpp-prof1）

- 整例墙钟（3 次）：rava 48.34 / 48.50 / 50.08 s，JVM 0.65 / 0.68 / 0.70 s
  （**约 72 倍**；无 GC 文档 §六在 812b763f 记 46.2–47.0 s，batch-1011k 增量 +2~3 s
  属 review-soundness 等并入后的机况差异，结构未变）。
- 受控对照（BenchStreamLoop，24 元素 `Arrays.stream(combo).map(i -> primes[i]).sum()`，
  每次调用 ns，中位）：rava **stream 6435（268 ns/元素）vs 手写循环 193（8 ns/元素）**；
  JVM stream 39（约 1 ns/元素）vs 循环 8。即 stream 管线在 rava 侧每元素附加
  ~260 ns、是同二进制手写循环的 33 倍、是 JVM stream 的 268 倍——慢不在数组遍历本身，
  在管线协议。

### 2. children 分解（F300 dwarf，占比有重叠，看结构）

| 聚簇 | children 占比 | 构成（符号 → 语义） |
|---|---|---|
| 终端求值 `IntStream.sum` | 71.7% | `copyInto` 33.8 + `Spliterator.forEachRemaining` 22.4（接口入口；`IntConsumer.accept` 17.6）+ `Sink.accept_i` 9.25 + `ReducingSink.accept_i` 7.42 + `IntBinaryOperator.applyAsInt` 2.33 |
| 管线构建（每叶子一次） | ~25–30% | `Head` 构造 10.2 + `StatelessOp` 构造 8.28 + `AbstractPipeline` 构造 7.91+2.42 + spliterator 构造 2.97+3.52 + `makeInt` 3.43 + `ReduceOps_6::new` 2.60 + `StreamSupport.intStream` 3.40 |
| 接口分派基础设施 | ~8% | `__IfaceRef::enter` 3.22（`handle.rs:187`，含每次接口调用的 `__stack_check`，self 1.10）+ `Handle::view` 2.18 + `try_cast::<Sink>` 2.60 |
| 区域整组释放 | ~6% | `___Region::exit` 4.65（`region.rs`，日志 Vec 追加 + 撤销）+ `on_alloc` 胶水 1.12 |
| 装箱 | <3% | `Integer.valueOf_i` 2.19 children / `__init_on_i` 0.47 self（仅静态初始化与个别角落） |
| 分配 / 释放 | ~3% | malloc 2.24 self + cfree 0.50 |

self 视图呈弥散形态：最大单点 `ReducingSink.accept_i` 3.08%（内层真实工作），其余
0.4–3% 碎片分布在数十个翻译体方法与 vtable 胶水（`IntPipeline`/`AbstractPipeline` 各
构造、`estimateSize` 1.42、`__IfaceRef::enter` 三形态 1.10+0.80+0.71 …）——协议税
无单点可修。

### 3. 结论（接口分派 / 装箱 / 迭代器协议之辨）

- **大头是迭代器协议 + 接口分派**：每个叶子整条管线重建（~10 个堆对象与 vtable 构造，
  占 25–30%），每元素走 `forEachRemaining → IntConsumer.accept →（map 的 ChainedInt
  accept）→ ReducingSink.accept → IntBinaryOperator.applyAsInt` 约 5 重间接分派，
  其中 2 重经 `__IfaceRef::enter`（视图查询 + 栈界检查，`handle.rs:187-204`）。
- **装箱不是大头**（<3%）：IntStream 全程基本类型，`Integer.valueOf` 只在静态初始化
  的流构造等角落出现。
- JVM 0.65 s 的构成是 JIT 的标量替换与内联把管线构建与逐元素分派全部消解
  （BenchStreamLoop 里 JVM stream ≈ 1 ns/元素、与手写循环 0.33 ns/元素同量级）；
  rava 无 JIT，等价收益要靠生成器侧（按调用点单态化 / 内联）或 PGO，非运行时单点可修。

## 五、已有符号级基线（nogc-perf3，kr1，812b763f）

`perf record -F 300 --call-graph dwarf`，children 视图（占比有重叠，仅作结构参考）：

| 聚簇 | 占比 | 构成 |
|---|---|---|
| `IntStream.sum` 终端求值 | 68.8% | copyInto 27.7% + forEachRemaining 17.8% + accept 链（IntConsumer.accept 13.6% / Sink.accept_i 6.8% / ReducingSink.accept_i 6.1% 自耗） |
| 管线构建（每叶子） | ~25–30% | AbstractPipeline 构造 8.4% + wrapSink 8.4% + spliterator 构造 ~6% + makeInt 4.3% + source 2.2% |
| 对象释放 | ~5% | Object drop_glue 4.4%（自耗第一）+ malloc 1.5% |
| 装箱 | <3% | Integer::from 0.76% 等（IntStream 全程基本类型，无装箱热点） |

## 六、微修复（提交 9157a73e）

1. `Class.__name_assignable`（`runtime/java_runtime/src/java/lang/class_impl.rs:324`）：
   层次表与直接超接口表由线性 `iter().find` 改 `meta::row` 二分（表按名有序，同取首个同名行，
   与 meta::row 既有口径一致）。调用面：反射实参校验（`reflect_dispatch.rs:199`）、数组
   checkcast 视图（`array.rs:366`）、`isAssignableFrom`、代理与 Array.set 分量检查。
2. `rd::unbox_*`（`reflect_dispatch.rs:298` 起）：翻译包装类实例取值从 `__obj_str().parse()`
   文本往返改为 hashCode 结构化取值（P-B 原型，见 §三）；Long/Double 不可逆仍走文本。

回归测例（expected 由服务器 JVM 生成）：`tests/e2e/62_reflection/TestIfaceAssignWalk.java`
（两条查表路径 + 反射实参校验 + 拒收面）、`TestUnboxWrapperFaces.java`（全值域 + 拓宽 +
Float 位形 + 拒收面）。

## 七、下一步建议

1. **Method.invoke 分派键重建（最大单项，String 簇 ~17% + 其分配 ~16% 的一部分）**：
   `__member_key`（`method_impl.rs:36`）每次 invoke 从 clazz/name/parameterTypes 重建
   (类名, 方法名, 描述符)。两条候选路线，均超出「低风险微修复」边界，建议随反射建模线排期：
   - 运行时侧：Method 对象身份键侧表缓存（`__identity` 为键、值 pin 一份 Method 防
     地址复用，容量上限 + 超限不再插入；Method 元数据字段为 final，JDK 自身的
     `methodAccessor` 缓存同假设）；
   - 发射层（生成器域）：`getDeclaredMethod` 族构造 Method 时一次性预计算分派键。
2. **unbox 结构化承载终态（P-A）**：随 S7 K 系 vtable / 方法 trait 形态演进一并设计
   （`ObjectVTable` 原生值视图臂），Long/Double 也能直取值；P-B 已收割无协议改动的部分。
3. **PartitionInteger / stream 管线**：72 倍的主干是「每叶子管线构建（25–30%）+ 每元素
   约 5 重接口分派（含 2 次 `__IfaceRef::enter` 的视图查询 + 栈界检查，`handle.rs:187`）」，
   运行时侧无单点可修（self 弥散在数十个翻译体方法）。可行路线：
   - S7 线分派视图约 19%（jackson jc1 已归 S7）同源；
   - 生成器侧按调用点对 stream 管线单态化 / 内联（等价 JVM JIT 的标量替换，BenchStreamLoop
     实测 JVM 把管线消解到 1 ns/元素）；
   - 小项：`__IfaceRef::enter` 的 `__stack_check` 与所在方法体的入口检查合并（发射层）；
     `___Region::exit` 4.65% 里日志 Vec 追加的可合并性（区域线）。
4. **类字面量段 155×**（496 ns/迭代：镜像比较 + isInstance + 周期性 getName）单列观察，
   未深剖；量级小于 invoke 段。
5. String 跨 crate 不内联（jackson jc1 约 4%）：属 S7 线口径，本步未复测。

## 八、验证与检查状态

- 本机 cargo check：generator `--release --tests` 通过；runtime 子 crate
  （rava_macros / rava_macros_core / rava_coro）`--release --tests` 通过。
  `java_runtime` 单独不可 cargo check（`src/java` 等目录的 mod.rs 与伴生类模块由发射层
  在 scratch 内生成，裸检出 E0583 为既有形态，pristine 5328072a 同败）——其编译验证经
  rtpp 作业的端到端构建承载。
- 回归与 A/B：作业 rtpp-verify1（expected 生成、7 例回归、BenchReflectArg /
  RldcSegments 复测、修复后行级剖析），结果见 `cluster_results/job/rtpp-verify1/`。

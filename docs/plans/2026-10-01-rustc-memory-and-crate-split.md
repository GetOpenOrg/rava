# rustc 编译内存：限制手段、依赖图实测与拆 crate 方案

> 日期：2026-10-01
> 性质：设计分析与执行顺序。结论先行：**按包 / 按 SCC 直接拆 crate 不可行**（实测 95% 文件在同一个环里）；可行路径是「声明层 / 方法体层」分离，让跨 crate 调用在链接期解析。该方案的收益尚无实测证据，执行顺序为先测量、再收窄闭包、再擦除泛型、最后才拆 crate。
> 关联：[`2026-09-30-optimization-directions.md`](2026-09-30-optimization-directions.md)（总纲 §二「下游编译」、§三.3 生成器效率）、`2026-09-30-emitter-performance.md`（生成器效率线，执行者维护）、`scripts/cargo_env.py`（N8）、`scripts/run_bg.sh`。
> 复现：`python3 scripts/dep_scc.py build/<test>/java_runtime/src [--dump 清单]`

---

## 一、问题

- 生成代码全部落在一个 `java_runtime` crate 中。16G 机器上，单个 rustc 的峰值约 14G（N8 实测：`CARGO_BUILD_JOBS=2` 时约 1750 类的闭包就会被 OOM 杀）。
- 现有手段只能控制**同时跑几个 rustc**，压不低**单个 rustc 的峰值**。
- C1d 删掉过渡手写后，闭包显著变大（见下表），编译成本问题会更突出：

| 用例（缺省 rust 生成器，JDK 21） | 当前主线（a484fbac） | C1d（c1d-final 80c5c1df） |
|---|---:|---:|
| HelloWorld 生成类 | 249 | 1981 |
| HelloWorld 等价性观测发射点（eq） | 2412 | 30875 |
| HelloWorld 编译 | 23.9 s | 2 m 40 s（且编译失败） |
| DeepCopy 生成类 | — | 2774，编译 9 m 06 s |

## 二、限制 rustc 内存的手段

### 2.1 系统层硬上限

| 平台 | 做法 | 说明 |
|---|---|---|
| macOS（本机） | 无 | `ulimit -v` / `-m` 内核不强制，没有 cgroups |
| macOS 替代 | `RUSTC_WRAPPER` 包装脚本：启动 rustc 后轮询 `ps -o rss=`，超过阈值就 kill 并报错；或在限内存的 Docker / Linux 虚拟机中编译 | 目的是尽早失败，避免拖垮整机或进入 swap |
| Linux | `systemd-run --user --scope -p MemoryMax=8G cargo build`、`docker run --memory=8g`、cgroup v2 | 真正的硬上限 |

硬上限只保护机器，不能让编译成功：rustc 需要 14G 却只给 8G，结果就是编译失败。

### 2.2 控制同时跑几个 rustc（已在用）

- `scripts/cargo_env.py`（N8）：生成类 ≥ 1700 时自动 `CARGO_BUILD_JOBS=1`。
- 同一时间只跑一个 e2e 批次。

### 2.3 编译参数（压低单个 rustc 的峰值）

| 参数 | 作用 | 现状 |
|---|---|---|
| `CARGO_INCREMENTAL=0` | 关闭增量编译，省掉增量元数据的双份内存 | `run_bg.sh` 已设 |
| `CARGO_PROFILE_DEV_DEBUG=line-tables-only`（或 `0`） | 减少调试信息；设为 `0` 时 panic 回溯没有行号 | 已设 `line-tables-only` |
| `CARGO_PROFILE_DEV_CODEGEN_UNITS` | 调大后每个 LLVM 模块更小，效果待实测 | 默认值 |
| `-Z threads=N`（nightly） | 前端并行度，调低能省内存 | 未用 |
| `-Z self-profile` / `-Z time-passes`（nightly） | 测量内存耗在哪个阶段 | **待做（§五 第 1 步）** |

这些参数通常只能降几成，降不了一个数量级。要给单个 rustc 设上限，只能把单个 crate 变小。

## 三、依赖图实测：JDK 类几乎全部互相依赖

方法：依赖边取生成文件的 `use crate::…::<类型>;`，解析到定义该 struct 的生成文件；只统计带 `rava_macros::java_class` 生成标记的文件（`scripts/dep_scc.py`，基于 rust-closure-analyzer a484fbac 的生成树）。

| 用例 | 生成文件 | 行 | 依赖边 | 最大 SCC（文件 / 行） | 包数 | 最大包级 SCC（包 / 覆盖文件） |
|---|---:|---:|---:|---|---:|---|
| HelloWorld | 249 | 57 062 | 4 798 | 225（90%）/ 96% | 31 | 29 / 97% |
| TestStreamBasic | 398 | 89 024 | 11 047 | 371（93%）/ 97% | 34 | 33 / 98% |
| Digester | 1 438 | 416 030 | 73 532 | 1 368（95%）/ 98% | 82 | 79 / 99% |

- `Object`、`String`、`Class`、异常体系、集合、`System` 之间彼此引用，几乎任何类都能绕回来。闭包越大，环覆盖的比例越高。
- Rust 要求 crate 之间不能循环依赖，所以**按包或按 SCC 切分，切出来仍是一个装下 96–98% 代码的大 crate**，这条路不可行。

## 四、可行方案：声明层 / 方法体层分离

### 4.1 依据

环存在于**类型与签名**层面：编译 A 调用 B 的代码，只需要 B 的签名，不需要 B 的方法体。Rust 禁止的是 crate 之间循环依赖；链接阶段符号互相引用是允许的。所以把环留在声明层，把方法体拆出去，跨块调用交给链接器解析。

```
decl crate（底层）：全部 struct、字段布局、vtable trait、方法外壳
      ↑            ↑            ↑
body crate 1   body crate 2 … body crate N   （只依赖 decl，彼此不依赖）
      └── 链接器把外壳里的 extern 声明和 body 中的 #[no_mangle] 实现接上 ──┘
```

```rust
// decl：外壳，可读层调用处仍然是 map.get(k)
impl HashMap { pub fn get(&self, k: Object) -> Result<Object> { unsafe { __jb_java_util_HashMap_get(self, k) } } }
extern "Rust" { fn __jb_java_util_HashMap_get(this: &HashMap, k: Object) -> Result<Object>; }

// body_k：方法体
#[no_mangle] fn __jb_java_util_HashMap_get(this: &HashMap, k: Object) -> Result<Object> { … }
```

- body crate 按代码量均匀切块（目标规模由 §五 的测量结果确定）。块与块之间不 `use`，结构上不可能成环。
- 可读层不变：外壳与 extern 由生成器（及 `java_class!` 宏）封装，方法体里不出现底层调用，符合 CLAUDE.md「转译等价性原则」。
- 附带收益：各块可以并行编译、单独复用缓存；某个测试只改了用户类时，不重编 JDK 部分。

### 4.2 难点：泛型

extern 函数只能是单态的。实测泛型 struct 占比：HelloWorld 56 / 254，TestStreamBasic 150 / 409，Digester 329 / 1464。它们的方法都在 `impl<K, V> X<K, V>` 里，由各使用方单态化，没法直接放到 extern 后面。（生成文件中自带泛型参数的 `fn` 为 0，问题只出在泛型 struct 的方法上。）

方向：**擦除核心 + 泛型外壳**。
- 方法体只针对擦除后的类型实现一份（相当于 `X<Object, …>`），放在 body crate。
- decl 里的泛型外壳 `impl<K, V> X<K, V>` 负责在入口和出口做零成本转换。
- Java 语义与类型实参无关（擦除），这一点有保证。同时能消掉重复单态化的编译成本，这一项本身可能就是 rustc 内存的大头。

**待验证的前提：内存布局与类型实参无关。** 实测 Digester 中有 58 个字段直接以类型参数作为字段类型（例如 `HashMap_Node { key: K, value: V }`、`JArray<T>`）。如果实参为具体类（如 `String`）与实参为 `Object`（`Rc<dyn ObjectVTable>`）时表示不同（瘦指针与胖指针），就不能零成本转换，需要改成「类型参数位统一存 `Object`，外壳负责类型化」。这会触及命名原则 3（泛型 `T` 不强制擦除为 `Object`）的实现方式，但不改变它的可读层要求：签名和调用处仍然是泛型 `T`，只是存储层统一。这一条要先论证，再实施。

#### 4.2.1 布局前提论证（2026-10-01，生成器效率线）

结论：**除数组外，前提成立**。类型实参只存在于类型系统里，不影响句柄布局和对象存储；只有 `JArray<T>` 的自有存储随 `T` 变化。依据取自主线 `java_class!` 宏展开与 `runtime/java_runtime/src/array.rs`。

| 形态 | 表示 | 与类型实参的关系 | 擦除核心 ↔ 泛型外壳的转换 |
|---|---|---|---|
| 类句柄 `X<K, V>` | `{ vtable: Rc<dyn X__VTable>, any: AnyRef, _jvm_null: bool, __phantom: PhantomData<fn() -> K> … }` | 无关（只差零尺寸的 phantom） | `__from_parts(vtable, any, null)` 逐域搬移；零成本，对象标识不变 |
| 接口句柄 `I<T>` | `{ __ref: Object, __phantom }` | 无关 | 同上，零成本 |
| 对象存储 `X__inner` | 以类型参数为类型的字段，宏已擦除为 `Object`（例：`KeyValueHolder__inner.key: Rc<RefSlot<Option<Box<Object>>>>`）；泛型元素数组字段也存为 `JArray<Object>`（例：`HashMap__inner.table`） | 无关 | 无需转换。§4.2 所说的 58 个 `key: K` 字段只是源码层声明，存储层早已统一为 `Object` |
| 类型参数位的值 `K` | 实参为具体类时是该类句柄（约 40 B：vtable + any + null 标志），为 `Object` 时是胖指针（16 B） | **不同** | 入口 `Into<Object>`、出口 `From<Object>`。这正是现有字段读写已经走的转换（约束 `K: From<Object> + Into<Object>` 已在全部泛型类上），不是新增成本，语义与现状同源 |
| 数组 `JArray<T>`（`T` 为类型参数） | `Rc<Repr<T>>`，`Repr::Own(RefCell<Vec<T>>, …)` | **不同**：元素布局随 `T` 变化 | 不能零成本重解释。可用运行时已有的 `Covariant` 视图（保持对象标识，读写经源数组，aastore 检查不变）；代价是每次转换分配一个视图，每次元素访问多一次 dyn 调用和一次元素转换 |

实施约束：
1. 擦除核心的签名按描述符（`T[]` → `[Ljava/lang/Object;` → `JArray<Object>`）生成。外壳里把 `JArray<T>` 转为 `JArray<Object>` 走 `Covariant` 视图，从 `JArray<Object>` 还原走视图回源（运行时已有语义：「还原为源类型取回源数组本身」）。
   - 涉及面：DeepCopy 树中签名含 `JArray<类型参数>` 的函数共 52 个（全部函数 42,310 个），集中在 `TimSort` / `ComparableTimSort` / `Arrays` 一类排序与拷贝代码。
   - 运行期影响集中在这些热循环，需要单独做性能验收。
2. 命名原则 3 的可读层不变：外壳签名保持泛型 `T`，擦除只发生在 decl 与 body 之间的 extern 边界。
3. 等价性：Java 语义本来就在擦除后执行，擦除核心与 JVM 字节码一一对应。转换点从「方法体内的字段读写」前移到「外壳入口 / 出口」，`From<Object>` 失败的时机可能随之改变（例如堆污染时，由读字段时报错变为返回时报错）。这正对应 javac 在调用方插入 checkcast 的位置，但仍需逐一核对 `From<Object>` 在类型不符时的行为（返回 null 句柄还是抛 ClassCastException），再给出 e2e 抽查清单。

### 4.3 代价与风险

| 项 | 说明 | 对策 |
|---|---|---|
| 跨 crate 调用不能内联 | dev 构建本来就内联得少；release 有性能影响 | release 用 thin-LTO（链接阶段也占内存，要实测） |
| extern 两侧签名一致性 | 不一致就是 UB | 两侧由生成器从同一份描述生成；加链接期签名校验（符号名中编码描述符哈希） |
| 外壳数量 | 约等于函数数（Digester 3.76 万个 `fn`） | decl 体量要实测；外壳只转发，没有方法体 |
| 宏展开位置 | `java_class!` 生成的 impl 必须与 struct 同在 decl | 宏拆出「声明展开」和「方法体展开」两种模式 |

## 五、执行顺序与量化目标

| 步 | 内容 | 验收 / 目标 | 归属 |
|---|---|---|---|
| 1 | **测量**：nightly `-Z self-profile` / `-Z time-passes` 编译 HelloWorld、DeepCopy（主线与 C1d 各一次），拿到分阶段耗时与内存（类型检查、MIR、单态化、LLVM） | 得出内存主要耗在哪个阶段，写回本文 §六 | 生成器效率线 |
| 2 | **收窄闭包**：C1d 下 HelloWorld 的 1981 类逐项用 `--trace-class` 归因，精度不够带进来的列入精度三期 | 类数回到有运行期依据的范围；C1d 在编译成本回到合理范围前不合入 | 主会话 + 精度三期 |
| 3 | **泛型擦除核心**：先论证 §4.2 的布局前提，再实施 | 单态化实例数 → 每个泛型方法 1 份；e2e 通过、输出与 JVM 一致 | 生成器效率线 |
| 4 | **decl / body 分层拆 crate**：第 1–3 步之后单个 rustc 仍然超出目标时实施 | 单个 rustc 峰值 ≤ 2 GB；HelloWorld 编译 ≤ 20 s；仅用户类变化时 JDK 部分零重编 | 生成器效率线 |
| — | 附：`RUSTC_WRAPPER` RSS 看门狗（阈值 12 GB） | 超限时编译明确报错，不拖垮整机 | 可随时加，独立于上述步骤 |

第 3、4 步是生成形态改造，属于总纲 Q1 已允许的「降低编译成本的形态改造」。验收口径是 e2e 通过，且生成树差异只含预期改动。

## 六、测量记录

### 6.1 第 1 步：rustc 分阶段测量（2026-10-01，生成器效率线）

**测量方法**
- 生成树：主线生成器（`emitter-perf` 与主线逐字节一致；唯一差别是根 Cargo.toml 的 `[profile.dev] debug = "line-tables-only"`、`incremental = false`，与 e2e 的环境变量设置相同），JDK 21，`rava build --no-run`。
- 编译命令：
  ```
  cargo +nightly rustc -p java_runtime --lib -- -Z time-passes -Z dump-mono-stats=<dir> -Z dump-mono-stats-format=json
  ```
  dev profile，`CARGO_BUILD_JOBS=2`，依赖预先编好，一次只跑一个；外层 `/usr/bin/time -l` 取峰值 RSS，另每 2 s 采样 rustc RSS。
- 共享机负载 3–19，其他代理并行占用内存。macOS 在内存压力下会压缩页面，使 RSS 下降：Digester 运行时出现 sys 68 s 和逐阶段 RSS 回落。**各阶段 RSS 是下限，真实工作集以峰值 RSS 为准**；耗时同样偏高。

**规模**

| 用例 | 生成类 | 生成源码（java_runtime） | `fn` 数 | 泛型 struct |
|---|---:|---:|---:|---:|
| HelloWorld | 249 | 4.7 MB | 8,761 | 62 |
| Digester | 1,438 | 27.9 MB | 38,691 | 335 |
| DeepCopy | 1,645 | 31.2 MB | 42,310 | 355 |

**java_runtime 单个 rustc：耗时（s）/ 阶段结束时 RSS（MB）**

| 阶段 | HelloWorld | Digester | DeepCopy |
|---|---|---|---|
| 宏展开 `expand_crate` | 3.1 / 45 → 504 | 24.8 / 45 → 1590 | 22.0 / 45 → 2944 |
| 名称解析 `resolve_crate` | 0.2 / 598 | 3.4 / 1707 | 2.0 / 3321 |
| AST → HIR 降级（未单列计时，按前后阶段 RSS 推算） | +212 MB → 811 | —（压力下不可读） | +1.8 GB → 5114 |
| coherence | 0.4 / 933 | 9.4 / — | 3.1 / **6067** |
| 类型检查 `type_check_crate` | 3.6 / 1414 | 97.2 / — | 29.2 / — |
| MIR 借用检查 | 3.7 / 1880 | 60.9 / 2360 | 42.1 / —（此处失败，见下） |
| lints / misc_checking_3 | 0.2 | 6.9 | — |
| 单态化收集 | 1.1 / 2113 | 18.4 / 3039 | — |
| crate 元数据 | 1.7 / 2151 | 28.8 | — |
| codegen → LLVM IR | 1.6 / 2320 | 55.9 | — |
| LLVM passes（与 codegen 重叠） | 3.6 | 86.4 | — |
| 链接 rlib | 0.03 | 4.9 | — |
| **合计墙钟** | **17.1** | **332.6** | 103.7（失败退出） |
| **峰值 RSS（time -l）** | **2.35 GB** | **4.35 GB**（压力下偏低） | **≥ 6.14 GB**（只到借用检查） |

**DeepCopy 在主线上编译失败**
- 错误：`java/io/object_input_stream.rs:977` E0381（`ObjectInputStream.readObject0` 的 `local_5` 在 try / finally 复制出的 `return Ok(local_5)` 路径上未初始化）。
- 这是方法体结构化翻译的缺陷，属生成器 bug，已转交主会话。
- 因此 DeepCopy 的后端阶段（单态化、LLVM）没有数据，后端用规模相近的 Digester 代替。

**单态化实例（`-Z dump-mono-stats`，size 为 rustc 的 MIR 规模估计）**

| 类别 | HelloWorld 实例 / size 占比 | Digester 实例 / size 占比 |
|---|---|---|
| 合计 | 68,387 / 99 万 | 400,360 / 677 万 |
| 生成代码的非泛型项（方法体、存根、From 等） | 50.8% / 44.2% | 54.9% / 54.9% |
| std / core 泛型实例 | 27.0% / 26.8% | 23.8% / 20.2% |
| 宏 `__` 样板（`__unsafe_ref_*` / `__shallow_copy` / `__view_*` …） | 10.0% / 19.1% | 10.2% / 17.2% |
| runtime `gil` / `sync_model` 泛型（`clinit_enter::<T>`、`__GilStatic::<T>::with` 按类单态化） | 5.5% / 5.1% | 5.9% / 5.0% |
| 泛型 Java 类的方法（`X::<K, V>::m`） | 4.9% / 6.2%，平均每项 1.15 份实例 | 6.6% / 7.9%，平均每项 1.37 份实例 |
| 其中擦除成单份可省的部分 | 436 份实例 / 0.8% | 7,144 份实例 / **1.8%** |
| `ObjectVTable` 缺省方法（按实现类型单态化） | 8,276 份实例 / 4.3% | 50,135 份实例 / 3.9% |
| `object_ext` 泛型辅助（`checkcast::<T>` 等） | 782 份实例 / 2.0% | 5,032 份实例 / 2.0% |

**结论**
1. **内存峰值在前端，不在后端。**
   - HelloWorld 的 RSS 随阶段单调上升，到 codegen 时约 2.3 GB。
   - DeepCopy 在 coherence / 类型检查阶段已达 6.1 GB。
   - DeepCopy 的峰值中，宏展开（+2.9 GB）和 HIR 降级（约 +1.8 GB）两项合计约占 3/4。
   - HelloWorld 的峰值出现在 codegen，这两项（+0.46 GB、+0.21 GB）约占 30%，其余在类型检查和借用检查中逐步累积。
   - 两者都与展开后的代码体量成正比（HelloWorld 展开前 4.7 MB，展开后 24.6 MB），瓶颈在 `java_class!` 的逐类展开量。
2. **耗时也在前端。**
   - Digester 的前端（宏展开、解析、coherence、类型检查、借用检查、misc checks）约 205 s，占总耗时 62%。
   - 后端的单态化、元数据、codegen 和链接约 139 s（LLVM passes 与 codegen 重叠）。
   - 类型检查和借用检查按函数定义计费，与单态化实例数无关。
3. **第 3 步「泛型擦除核心」对编译成本的直接收益很小。**
   - 泛型 Java 类方法平均只有 1.15–1.37 份实例，擦除成单份只省 0.8–1.8% 的单态化规模；类型检查和借用检查的成本本来就按定义只算一次。
   - 这一步的必要性只来自第 4 步：extern 边界必须是单态的。不能把它当作独立的降本手段。
4. **有效杠杆按规模排序：**
   - ① 宏展开体量：逐类 `__` 样板占单态化 size 17–19%，还同时推高宏展开和 HIR 的内存。
   - ② 闭包规模（第 2 步）：所有阶段都随类数线性增长。
   - ③ 按类单态化的 runtime 泛型（`gil` / `sync_model` / `ObjectVTable` 缺省方法，合计约 9%），改成非泛型内核 + 薄泛型入口。
   - ④ 拆 crate（第 4 步）：降低单个 rustc 峰值的唯一结构性手段；前端内存按 crate 体量线性分摊。
   - ①③ 归 runtime / 宏的属主。


### 6.2 第 1 步复测：emitter-perf2 合入后（2026-10-01，生成器效率线）

**测量方法**
- 生成树：`rust-closure-analyzer` @ f6d80103（含 emitter-perf2：宏样板收窄、二进制约小 40%），JDK 21，`rava build --no-run`。
- 脚本化：`scripts/rustc_profile.sh <out> <scratch>...`（nightly 1.100.0-nightly 2026-09-04；依赖先编好；`-Z time-passes` + `-Z dump-mono-stats`；`/usr/bin/time -l` 取峰值；每 2 s 采样 RSS）。单态化归类：`scripts/mono_stats.py`。
- `-Z self-profile` 可产出 `.mm_profdata`，但本机没有 measureme 的 `summarize`，无法汇总，本次只用 time-passes。
- 测量前等待本机无 rustc 空闲 30 s 再启动；Digester 测量期间有其他工作树的 `cargo build`（closure / emit crate，约 0.6 GB）插入，耗时略偏高，峰值 RSS 不受影响（按进程计）。

**规模**

| 用例 | 生成类 | 生成源码（java_runtime） | `fn` 数 |
|---|---:|---:|---:|
| HelloWorld | 249 | 4.7 MB | 9,120 |
| Digester | 1,439 | 27.9 MB | 39,290 |
| DeepCopy | 1,646 | 31.2 MB | 42,970 |

**java_runtime 单个 rustc：耗时（s）/ 阶段结束时 RSS（MB）**

| 阶段 | HelloWorld | Digester | DeepCopy |
|---|---|---|---|
| 宏展开 `expand_crate` | 2.9 / 45 → 430 | 19.2 / 45 → 2443 | 20.6 / 45 → 2722 |
| 名称解析 `resolve_crate` | 0.2 / 530 | 1.4 / 2891 | 1.5 / 3203 |
| AST → HIR 降级（按前后阶段 RSS 推算） | +193 MB → 723 | +1.27 GB → 4157 | +1.35 GB → 4553 |
| coherence | 0.5 / 831 | 2.9 / 4733 | 3.0 / 4525 |
| 类型检查 `type_check_crate` | 3.3 / 1264 | 31.8 / —（压缩回落） | 23.9 / 4822 |
| MIR 借用检查 | 3.2 / 1605 | 28.4 / — | 25.6 / —（此处失败） |
| lints / misc_checking_3 | 0.2 | 3.8 | — |
| 单态化收集 | 1.0 / 1817 | 10.0 / 3813 | — |
| crate 元数据 | 1.4 | 16.5 | — |
| codegen（含 → LLVM IR） | 2.4 / 1932 | 28.6 | — |
| LLVM passes（与 codegen 重叠） | 2.4 | 27.9 | — |
| 链接 rlib | 0.06 | 1.2 | — |
| **rustc 合计** | **14.4**（墙钟 16.0） | **138.3**（墙钟 144.9） | 77.4（失败退出，墙钟 83.4） |
| **峰值 RSS（time -l）** | **1.97 GB** | **4.45 GB** | **≥ 6.11 GB**（只到借用检查） |

与 §6.1 对照：
- HelloWorld：墙钟 17.1 → 16.0 s，峰值 2.35 → 1.97 GB（−16%）；宏展开后 RSS 504 → 430 MB。
- Digester：rustc 合计 332.6 → 138.3 s。§6.1 那次处在内存压力下（sys 68 s），这次 sys 20 s，两次耗时不可直接比；各阶段比例可比：前端（宏展开到 misc_checking_3）约 88 s（64%），后端约 50 s。峰值 4.35 → 4.45 GB（§6.1 在压力下偏低，按持平看）。
- DeepCopy：宏展开后 RSS 2944 → 2722 MB，coherence 时 6067 → 4553 MB（阶段 RSS，受压缩影响只作下限），峰值 6.14 → 6.11 GB，基本持平——前端峰值出现在类型检查 / 借用检查阶段，这部分按函数定义计费，宏样板收窄对它影响小。
- **DeepCopy 仍编译失败**：同一位置 `java/io/object_input_stream.rs:977` E0381（`local_5`），属主会话在修的方法体结构化缺陷，后端数据仍以 Digester 代替。

**单态化实例**

| | HelloWorld | Digester |
|---|---|---|
| 条目 / 实例 / size_est | 26,224 / 57,077 / 62.1 万 | 155,522 / 329,431 / 419.5 万 |
| 对 §6.1 | 实例 −17%，size −37% | 实例 −18%，size −38% |

按 `scripts/mono_stats.py` 归类（启发式与 §6.1 的手工归类口径不同：`gen` 只含单份实例项，多份实例的非 std 项归 `gen-generic`；`__clinit` 不算宏样板），实例 / size 占比：

| 类别 | HelloWorld | Digester |
|---|---|---|
| 生成代码单份项 `gen`（方法体、存根、`__clinit`、From 等） | 21.4% / 27.0% | 22.3% / 33.1% |
| 宏 / runtime `__` 样板 `macro` | 25.3% / 23.3% | 28.2% / 27.8% |
| std / core 泛型实例 | 27.6% / 31.2% | 23.3% / 21.9% |
| 多份实例的生成项 `gen-generic` | 11.1% / 7.8% | 10.8% / 7.6% |
| `ObjectVTable` 缺省方法 | 11.8% / 7.8% | 12.9% / 6.9% |
| `object_ext` 泛型辅助 | 1.4% / 2.4% | 1.5% / 2.4% |
| `gil` / `sync_model` | 1.4% / 0.4% | 1.0% / 0.2% |

size_est 前列（Digester）：`Weak::drop` 4,799 份（18.7 万，占 4.5%）、`Box::new` 7,882 份、`checkcast::<T>` 1,253 份、`Arc::drop_slow` / `Arc::new` / `Arc::drop` 各约 4,800–5,600 份、`dyn Any::downcast_mut` 3,268 份。`Weak` / `Arc` 一族按持有类型单态化，来源是每类一份的弱自引用 / 共享载体，合计约 10% size。

**结论（在 §6.1 基础上更新）**
1. 结论 1、2 不变：内存与耗时都在前端。emitter-perf2 让宏展开与 HIR 体量下降约 10–15%，单态化规模下降 37–38%，HelloWorld 峰值降到 1.97 GB；大用例的峰值由类型检查 / 借用检查阶段决定，基本没动。
2. 后端（单态化 + codegen + LLVM）的剩余热点从宏样板转向 std 智能指针按类型的实例（`Weak` / `Arc` / `Box` / `downcast_*`，合计约 20% size），与 `ObjectVTable` 缺省方法（约 7%）。这两项的收窄方向是 runtime 非泛型内核 + 薄泛型入口（§6.1 结论 4 ③），归 runtime / 宏属主。
3. 单个 rustc 峰值 ≤ 2 GB 的目标（第 4 步）只有 HelloWorld 达到；Digester / DeepCopy 仍需第 2 步收窄闭包和第 4 步拆 crate。

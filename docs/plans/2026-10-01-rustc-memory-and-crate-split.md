# rustc 编译内存：限制手段、依赖图实测与拆 crate 方案

> 日期：2026-10-01
> 性质：设计分析与执行顺序。结论先行：**按包 / 按 SCC 直接拆 crate 不可行**（实测 95% 文件在同一个环里）。可行路径是「声明层 / 方法体层」分离，跨 crate 调用在链接期解析。**2026-10-01 实测修订（§七）**：只剥方法体压不低峰值——声明层本身（逐类 wrapper / vtable / 对象存储样板 + 反射数据表）才是峰值主体；终态因此是「元数据 crate + 声明 crate + N 个实现 crate」三层，对象存储 `X__inner` 及其全部 trait impl 与方法体一起下沉到实现 crate，并收窄声明层样板。实施步骤见 §七.5。
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

## 七、拆 crate 终态设计（2026-10-01 修订，生成器效率线）

### 7.1 修订依据：只剥方法体压不低峰值

**实验 1：源码层剥方法体（Digester）。** 把全部 `java_class!` 块里的方法体换成 `{ __stub("decl") }`，其余不动，相当于 §四 设想的 decl crate（`build/exp/strip_bodies.py`）。

| | 完整 | 剥方法体 |
|---|---:|---:|
| 墙钟 | 204.6 s | 169.8 s |
| 峰值 RSS | 3.90 GB | 3.85 GB |
| 宏展开后 RSS | 2474 MB | 2039 MB |
| 类型检查 / 借用检查 | 37.4 s / 40.0 s | 25.0 s / 34.4 s |
| 单态化实例 / size_est | 320,315 / 402 万 | 285,194 / 279 万 |

方法体只占源码 9.08 MB / 30 MB（29,527 个 fn、1,439 个文件）。剥掉之后峰值只降 1%：**§四 的「decl crate 只剩签名、体量很小」前提不成立**，峰值主体是 `java_class!` 为每个类展开的声明层样板。

**实验 2：宏展开层裁剪（直接编译 `-Z unpretty=expanded` 的产物）。**
- 工具：`build/exp/fix_expanded.py`（补 feature 门）、`build/exp/rustc_expanded.sh`（nightly rustc 直接编译展开结果，`/usr/bin/time -l` 取峰值）、`build/exp/core_cut.py`（按类别裁剪）。
- 校准：HelloWorld 展开结果直接编译 13.8 s / 1.96 GB，与 §6.2 经宏编译的 16.0 s / 1.97 GB 一致，可以当作测量代理。
- 裁剪类别：
  - `tables`：build.rs 生成的反射数据表（`MethodMeta` / `FieldMeta` 等）初值换成 `&[]`；
  - `inner`：删除 `X__inner` 结构体及其全部 impl，其余函数体凡提到 `__inner` 的换成 `loop {}`；
  - `bodies`：wrapper 固有 impl 里的方法体（`__impl_*` / `__init_on_*` / 构造器 / 静态方法）与 `_base` 函数体换成 `loop {}`。

| HelloWorld（展开后源码 / 峰值 / 墙钟） | 结果 |
|---|---|
| 完整 | 21.4 MB / 1.96 GB / 13.8 s |
| 去 tables | 15.0 MB / 1.76 GB / 13.7 s |
| 去 inner | 17.1 MB / 1.52 GB / 11.0 s |
| 去 bodies | 18.8 MB / 1.77 GB / 11.3 s |
| 去 inner + tables | 10.7 MB / 1.34 GB / 8.0 s |
| **三项全去（≈ 声明 crate）** | **8.4 MB / 1.12 GB / 6.1 s** |

| Digester | 结果 |
|---|---|
| 完整（机器内存压力下，sys 155 s，峰值偏高） | 130.7 MB / 4.99 GB / 390 s |
| **三项全去（≈ 声明 crate）** | **52.1 MB / 3.21 GB / 59 s** |

Digester 声明 crate 的剩余构成（展开后字节）：

| 类别 | 占比 |
|---|---:|
| wrapper 固有 impl（字段访问器、虚分派入口、各方法外壳） | 33.5% |
| `impl ObjectVTable for X`（wrapper；`__view_into` / `__view_as` / `__erased_*` / `__shallow_copy` / `__unsafe_*` 逐祖先展开） | 13.9% |
| `use` 列表与属性（按方法体需要导入，声明层用不到大半） | 13.8% |
| 泛型类的各类 impl（`impl<K, …>`，方法体暂留声明层） | 约 16% |
| 自由函数（`_base` 外壳、手写 runtime） | 7.5% |
| vtable trait 声明（含缺省方法） | 6.3% |
| From / derive 类 impl / 静态存储 / struct | 约 9% |

**结论**
1. 声明 crate（去掉存储层、方法体、数据表）已经把 HelloWorld 降到 1.12 GB / 6.1 s，Digester 降到 3.21 GB / 59 s（原 3.9–5.0 GB / 205–390 s）。
2. 对象存储层 `X__inner`（结构体 + `ObjectVTable` / 各级 vtable trait 的实现 + derive）是单项最大的可下沉部分（HelloWorld −0.44 GB），**必须和方法体一起下沉到实现 crate**；只下沉方法体（§四原设计）不够。
3. 反射数据表是纯数据，与类型环无关，单独成 crate（HelloWorld −0.20 GB）。
4. Digester 声明 crate 仍超 2 GB，还要收窄声明层样板（§7.4）。

### 7.2 终态结构

```
java_runtime（声明层）     runtime 基础设施（手写，含元数据元素类型 `meta.rs`）+ 每类：wrapper struct、
   ↑   ↑   ↑   ↑           vtable trait 声明、wrapper 固有 API（虚分派入口、字段访问器、静态字段、
   │   │   │   │           方法外壳）、From / Clone / PartialEq / Debug、ObjectVTable for wrapper、静态存储
   │   │   │   java_meta   反射 / 元数据表（构建脚本扫描整个 workspace 生成的纯数据），表 static 以
   │   │   │   │           `__java_meta_<表名>` 符号导出，java_runtime::meta 以 extern 声明读取
java_body_1 … java_body_N（实现层，彼此不依赖）
   │   │   │   │           每类：X__inner 存储、ObjectVTable / 各级 vtable trait for X__inner、
   │   │   │   │           方法体（#[no_mangle]）、_base 体、__clinit 体、存储钩子
user                       用户类（声明 + 实现同 crate，full 模式）；`use java_meta as _;`、
                           `use java_body_k as _;` 保证链接
```

- 元数据表覆盖用户类，所以 `java_meta` 必须位于 `java_runtime` 之上（依赖它取元素类型），而不是之下：若 `java_runtime` 依赖 `java_meta`，任何用户类改动都会经 `java_meta` 连带重编 `java_runtime`。表与读取方之间走导出符号，与实现层同一机制。
- 链接次序：rustc 把依赖方排在被依赖方之前传给链接器，`java_runtime` 对 `java_meta` / `java_body_k` 符号的引用出现在定义之后。macOS ld64、`rust-lld`（x86_64-unknown-linux-gnu 自 Rust 1.90 起缺省）按全部输入解析，不受次序影响；GNU ld / gold 按次序单遍扫描归档，会报未定义符号——这类目标须在 `.cargo/config.toml` 里指定 `-C link-arg=-fuse-ld=lld`。

- 类型环全部留在 `java_runtime` 内部；实现层只依赖声明层，结构上不可能成环。
- 链接器把声明层外壳里的 `extern "Rust"` 声明和实现层的 `#[no_mangle]` 定义接上。
- 实现 crate 的划分：类按 binary name 排序，按展开体量（以生成源码字节近似）贪心装箱；单箱上限由 §7.6 的测量确定（初值：单箱展开体量 ≤ HelloWorld 完整展开的 1.5 倍，约 30 MB）。按名排序装箱保证同一闭包的划分确定，缓存可复用。
- 可读层不变：调用处仍是 `map.get(k)`、`X::m(a)`、`this.__get_f()`；外壳、extern、钩子全部由 `java_class!` 宏生成，生成的方法体里不出现底层调用（CLAUDE.md 转译等价性原则）。
- 手写 `*_impl.rs`（`impl X { … }` native 方法）是 wrapper 的固有 impl，留在声明层。它们不引用 `X__inner`（实测手写与生成源码中 `__inner` 出现 0 次），不受存储层下沉影响。
- 用户 crate 位于依赖图顶端，用户类不拆层（宏 full 模式），与现状相同。

### 7.3 `java_class!` 的三种展开模式

同一个 `java_class!` 块由生成器写两份：声明层文件（`java_runtime/src/<pkg>/<x>.rs`，方法体不写出）与实现层文件（`java_body_k/src/<pkg>/<x>.rs`，完整块）。宏按块属性 `#[rava_layer = "decl" | "body"]` 选择展开内容，缺省 `full`（= 两者之和，与今天相同，用户 crate 用）。

| 项 | decl | body |
|---|---|---|
| wrapper struct、vtable trait 声明、`__from_parts` | ✓ | |
| 虚分派入口 `pub fn m(&self)`（null 检查 + `self.vtable.m()`） | ✓ | |
| 字段访问器 `__get_f` / `__set_f`、静态字段访问器与存储、clinit 状态 | ✓ | |
| 方法外壳：`__impl_m` / `__init_on_*` / 构造器 / 静态方法 → `unsafe { __jb_<符号>(…) }` | ✓ | |
| 方法体：`#[no_mangle] pub fn __jb_<符号>(this: &X, …) -> …` | | ✓ |
| `X__inner` 及 derive、`ObjectVTable for X__inner`、各级 `Y__VTable for X__inner` | | ✓ |
| 存储钩子（§7.3.1） | 外壳 | 定义 |
| From / Clone / PartialEq / Debug for wrapper、`ObjectVTable for X` | ✓ | |
| `_base` 体 | 外壳 | 定义 |
| `__stub(...)` 存根体（一行 panic，无依赖） | ✓（直接内联，不走 extern） | |

#### 7.3.1 声明层对存储层的依赖只经钩子

声明层今天引用 `X__inner` 的位置只有 4 处（宏源码逐一核对）：

| 位置 | 用途 | 终态 |
|---|---|---|
| `wrapper/mod.rs` `Default for X` | 分配一个默认存储（null 引用与 `new_*` 构造器共用） | 钩子 `__jb_<X>__alloc() -> (__Shared<dyn X__VTable>, __AnyRef)` |
| `wrapper/object_vtable.rs` `__shallow_copy` | 同上，分配后逐字段拷贝 | 同一钩子 |
| `wrapper/object_vtable.rs` `__unsafe_*_cell` / `__unsafe_ref_access` | 把 `any` 精确 downcast 成本类存储，取字段单元 | 钩子 `__jb_<X>__cells(any: &__AnyRef) -> Option<&dyn ObjectVTable>`（精确类型命中时返回存储的 ObjectVTable 视图，调用方随后调同名方法；未命中回落 vtable，与现状同序） |
| `type_conversions.rs` `From<Object>` 部件路径 B | `any` 精确 downcast 成本类存储，重建 wrapper | 钩子 `__jb_<X>__from_any(any: __AnyRef) -> Result<(__Shared<dyn X__VTable>, __AnyRef), __AnyRef>`（返回部件而非 `X`：wrapper 可能带类型参数，钩子保持非泛型） |

- 等价性：钩子体就是今天内联在原位置的代码，原样搬进实现层，调用点语义、求值顺序、失败回落顺序都不变。
- 钩子是与存储布局同模块的非泛型 `pub fn`（`#[doc(hidden)]`），不挂在 wrapper 上：S4 拆层后定义随存储层进实现 crate，声明层以同签名 extern 声明调用，调用点文本不变。
- `impl X__inner { pub const BINARY_NAME }`（让 vtable 上下文的方法体里 `Self::BINARY_NAME` 可解析）随存储层下沉。

#### 7.3.2 extern 边界

- 符号名：`__jb_` + 类 binary name 编码 + Rust 方法名 + 签名哈希（宏对「参数类型 + 返回类型」的 token 文本取 FNV-1a 64 位）。两侧签名来自同一个 `java_class!` 块；若生成器或宏的缺陷让两侧不一致，哈希不同，结果是链接错误（未定义符号），不会是未定义行为。
- 参数与返回值按今天的 Rust 类型原样传递（同一 rustc、同一 target、`extern "Rust"` ABI）。`self` 改名为 `this`：方法体开头本来就是 `let this = self;`，宏改写成参数名，体内文本不变。
- 链接：实现 crate 只被 `user` 以 `use java_body_k as _;` 引用；`#[no_mangle]` 符号是导出符号，不会被当作死代码剥掉。release 的 `lto = true` 仍跨 crate 内联。

#### 7.3.3 `_base` 与重复方法体（N4 剩余项并入）

- 今天每个虚方法的方法体最多出现 3 份：wrapper 的 `__impl_m`、`X__VTable for X__inner` 里的私有方法直展开、`X__m_base::<__BT>` 泛型体（被 vtable trait 缺省方法按实现类型单态化）。
- 终态每个方法体只保留一份定义（实现层的 `__jb_` 函数）。另外两处改为调用它：
  - `_base` 收 `this: &dyn X__VTable`（非泛型；类泛型 K、V 留到 S6 擦除）。调用方三种形态：`&X__inner`（unsize）、`&dyn Sub__VTable`（trait upcasting，rustc ≥ 1.86 稳定）、wrapper 的 `&*w.vtable`。
  - vtable trait 缺省方法的 `Self` 可能非 Sized，不能直接 unsize：每个有 Safe 缺省方法的类加一个视图 trait `X__AsVTable { fn __dyn_X(&self) -> &dyn X__VTable; }`，一揽子 impl 覆盖全部 Sized 实现类型，并作为 `X__VTable` 的 supertrait。生成的与手写的实现类型（如 locale provider 的 `_impl.rs`）都自动具备，`dyn` 对象经 supertrait 槽位取得同一视图。
  - 等价性：Safe 体不含 `this.<非 __ 方法>(`、裸 `Clone::clone(this)`、`Self::`（classify.rs 判定），只经 `__` 前缀访问器和本类 vtable trait 链上的方法访问 `this`；原先在 `__BT: X__VTable + ?Sized` 约束下即可编译，换成 `&dyn X__VTable` 调的是同一组 trait 方法、命中同一实现（`dyn` 的具体类型就是原先的 `__BT`）。
  - 体只一份：`X__VTable for X__inner` 里 Safe VirtualDefine 的私有直展开删除，沿用 trait 缺省方法（其体就是 `X__m_base(视图, ..)`）；Safe VirtualOverride 中作为 `_base` 所有者的条目（同名首个），覆盖体改为 `X__m_base(self, ..)` 转发，形参擦除还原（K-6b）与返回装箱（K-6a）保留在外壳。
  - 生成器侧：继承成员转发体与 `super.m()` 的 turbofish 去掉末位接收者类型（`Self` / `_`），非泛型 owner 不带 turbofish。
- N4 剩余项：
  - `__shallow_copy` / `__unsafe_*`：改走 §7.3.1 的钩子，随本方案实施。
  - `__erased_vtable` / `__view_into` / `__view_as`：逐祖先展开，留在声明层，属于 §7.4 的样板收窄。
  - `__clinit`：体下沉实现层，状态机留在声明层。
  - From impl：留在声明层，属于 §7.4。

### 7.4 声明层样板收窄（Digester 声明 crate 3.21 GB → ≤ 2 GB 的手段）

| 项 | 占声明层 | 做法 |
|---|---:|---|
| `use` 列表 | 13.8% | 声明层文件只导入签名与字段类型用到的类（生成器按层计算导入）；方法体的导入随方法体进实现层 |
| `ObjectVTable for X`（wrapper） | 13.9% | `__view_into` / `__view_as` / `__erased_vtable` 逐祖先的 if 链改为按祖先表驱动的单个泛型 helper（每类一行调用） |
| 字段访问器 | 约 4% | 继承字段的访问器改为经超类 vtable trait 的统一入口，只为本类声明字段生成（等价性待论证） |
| 泛型类方法体 | 约 16% | §4.2 擦除核心：方法体按擦除实例化只编一份，经外壳进入实现层（等价性见 §4.2.1；`JArray<类型参数>` 签名的 52 个函数走 Covariant 视图，需单独做性能验收） |

每项单独提交，逐项测 Digester 声明 crate 的峰值。

### 7.5 实施步骤

每步都是终态的一部分，不引入之后要拆掉的过渡形态。每步验收：
- 生成器单测；
- 27 例生成树对照：差异只含本步预期改动；
- HelloWorld / Digester 的宏展开对照或单态化统计；
- 抽查用例 `cargo build` 通过；
- 列出需要主会话 e2e 抽查的用例。

| 步 | 内容 | 生成形态改动 | 验收重点 |
|---|---|---|---|
| S1 ✅ | **`java_meta` crate**：反射数据表与造表构建脚本从 `java_runtime` 移出；元素类型留在 `java_runtime::meta`（手写），表 static 以导出符号交给 `java_runtime` 读取（依赖方向见 §7.2） | 仅 workspace 结构；表数据逐字节不变 | 表文件逐字节对照；HelloWorld / Digester `java_runtime` 峰值（§7.7） |
| S2 | **存储钩子**：§7.3.1 的 4 处改走钩子（同 crate 内落地为与存储布局同模块的非泛型自由函数，签名即终态 extern 签名） | 宏展开：4 处调用点换钩子调用 | 展开对照只在 4 处变化；抽查 |
| S3 | **方法体函数化**：方法体移入 `__jb_<符号>` 自由函数，wrapper 方法与 `_base` 变外壳；`_base` 去泛型；vtable-for-inner 私有方法直展开改调同一函数 | 宏展开：方法体搬家；单态化 `_base` 实例归一 | 单态化统计；抽查（覆盖私有方法、super 调用、`_base` 缺省分派） |
| S4 ✅ | **物理拆层**：宏 `rava_layer` 模式；生成器写 decl / body 两份文件、实现 crate 装箱、各 crate 的 Cargo.toml 与模块树；可见性收口。细化见 §7.5.1 | 生成树结构变化（新增实现 crate 目录；声明层文件块首多一行属性） | Digester / DeepCopy 各 crate 峰值与总墙钟；user 变化时 JDK 部分零重编 |
| S5 | **声明层样板收窄**（§7.4 前三项）；声明层文本不带下沉方法体（§7.7 S4 记录：先把下沉判据改为生成器显式标注，再剥体） | 宏展开与导入列表 | 逐项测声明 crate 峰值 |
| S6 | **泛型类擦除核心**（§4.2 / §7.4 第四项） | 泛型类方法体进实现层 | 性能验收（排序 / 拷贝热循环）+ 抽查 |

#### 7.5.1 S4 细化（2026-10-01）

**宏（`rava_macros`，`block/gen/layer.rs`）**
- 块属性 `#[rava_layer = "decl" | "body"]`，缺省完整展开（用户 crate、lib crate、Python 生成器不受影响）。
- 实现层（body）：`X__inner` 及其 derive、`ObjectVTable for X__inner`、`impl X__inner { BINARY_NAME }`、全部 `Y__VTable for X__inner` 与接口 impl、三个存储钩子、`__jbm_` 方法体函数、非泛型且非存根的 `_base` 函数。
- 声明层（decl）：其余全部（vtable trait 与 `AsVTable`、wrapper 及其 trait impl、固有方法与外壳、静态存储与类初始化、From / Into、泛型或存根 `_base`）。
- 可拆的三类自由函数（钩子 / 体函数 / 非泛型 `_base`）由同一个拆分函数处理：实现层定义加 `#[export_name = SYM]`；声明层生成同名外壳 `#[inline] pub fn f(..) -> R { extern "Rust" { #[link_name = SYM] fn f(..) -> R; } unsafe { f(..) } }`。外壳形参与返回类型取自同一函数项，调用点文本不变。
- `SYM = __rava_<binary name 转义>__<函数名>_<FNV-1a 64(binary name, 函数名, 签名记号文本)>`：两侧签名不一致只会是链接错误。
- 实现层文件 glob 导入声明层本类模块；同名的实现层定义遮蔽 glob 导入的外壳，所以实现 crate 内对本类钩子 / 体函数 / `_base` 的调用直达定义，别的类的经声明层外壳转调。
- 存根 `_base` 留在声明层直接定义（一行 panic，无依赖，不值得走 extern）。
- 接口块整体属声明层（body 模式展开为空，生成器也不为接口写实现层文件）。接口 default / static 方法体下沉是后续项，与 S6 一并做。
- 泛型类：`X__inner` 与 vtable impl 本就非泛型，随存储层下沉；wrapper 方法体与泛型 `_base` 留在声明层，到 S6 擦除核心再下沉。
- 可见性：wrapper 的 `vtable` / `any` / `__phantom` 字段由 `pub(crate)` / 私有改为 `pub` + `#[doc(hidden)]`。实现层的 `__as_X` 钩子与体函数按部件构造、读取 wrapper。

**生成器（Rust 生成器，`emit/src/project/layers.rs`）**
- 第二阶段收尾之后、落盘之前拆分：JDK 生成类（非手写、非接口）的块首加 `#[rava_layer = "decl"]`，原位落盘。
- 实现层文本 = 原文件头（allow 属性 + use 列表）+ `use java_runtime::<本类模块路径>::*;` + 同一块（`#[rava_layer = "body"]`）。块后的 `iface_upcasts!` / `implref` / 反射字段闭包属声明层，不进实现层。
- 装箱：类按 binary name 排序，均衡装箱。箱数 = max(⌈总字节 / `BODY_CRATE_BYTES`（5 MiB）⌉, 2)，每类按其字节中点落入的区间归箱。上限按 §7.6 的峰值目标实测校准，见 §7.7 S4 记录。
- 实现 crate `java_body_k`：
  - `src/lib.rs` 是 allow 属性 + 私有 `use java_runtime::*;` + `mod body;`；
  - 类文件放在 `src/body/<原相对路径>`，mod.rs 只写 `mod x;`，不再导出。
  - 这样类文件头的 `crate::java::…` / `crate::prelude` 经根 glob 解析到声明层，不被本 crate 模块遮蔽（取代原设想的 `_body` 后缀）。
  - Cargo 依赖只有 `java_runtime` 与 `rava_macros`。
- workspace 成员加实现 crate。`user` 依赖全部实现 crate，`main.rs` 写 `use java_body_k as _;` 纳入链接。批量模式的既有 user 清单补齐依赖行。
- `java_meta` 构建脚本扫描兄弟 crate 时排除 `java_body_` 前缀：实现层是声明层块的副本，类宇宙已由 `java_runtime` 覆盖。
- 陈旧实现层类文件（带生成标记、本轮未写）删除。装箱数减少时多余的 `java_body_k` 目录不在 workspace 成员内，不参与编译。

**等价性**
- 拆分只改变函数定义所在的 crate 和调用经过的一层 `#[inline]` 转发：形参按值原样转交，返回值原样交回，求值顺序与副作用不变。
- trait impl（`ObjectVTable` / `Y__VTable for X__inner`）在哪个 crate 定义不影响分派：vtable 由实现层的 `alloc` 钩子在 unsize 时取用。
- `#[export_name]` 项恒被代码生成且作为导出符号保留。dev 下跨 crate 调用不内联；release 的 `lto = true` 仍跨 crate 内联。

#### 7.5.2 S5 剥体：声明层文本不带下沉方法体（2026-10-01）

**问题**：S4 声明层收到整块文本，宏在声明模式下仍要词法分析、解析、改写全部方法体，再丢弃实现层部分。声明模式只读方法体的这几个结论：

| 读体位置（声明模式） | 读出的结论 |
|---|---|
| `wrapper/methods.rs` 虚方法（定义 / 覆盖） | gated 分类是否 Safe；非 Safe 时 `__impl_` 能否函数化（不能则体内联在 wrapper） |
| `wrapper/methods.rs` 构造器 / 非虚方法 | 能否函数化（不能则体内联） |
| `virtual_dispatch/trait_decl.rs` | 定义方法是否 Safe（缺省方法走 base 还是钩子；是否要 `AsVTable` 视图） |
| `virtual_dispatch/base_fns.rs` | 是否 Safe、Safe 定义方法的 base 是真实体还是存根；真实体的 base 非泛型时拆入实现层，声明层只用外壳 |

外壳（wrapper 外壳、体函数外壳、base 外壳）只取签名，不取体。

**做法**
- **分析逻辑独立成库 crate `runtime/rava_macros_core`**：`block/` 与 `try_macro.rs` 原样移入，`rava_macros` 只剩 proc-macro 入口。生成器（emit crate）依赖同一个库，用与宏相同的代码对同一份块文本做判定，不另写一份移植。
- **判定函数 `moved_fact`（库内，宏与生成器共用）**：对非泛型、非接口类里有体的方法，体在声明模式下不被消费时给出标注，否则 None（体留在声明层）：
  - 虚方法：Safe 且方法无泛型形参 / where 子句 → `safe`（base 真实体）或 `safe_stub`（base 存根）；非 Safe 且 `__impl_` 可函数化 → `wrapper`；
  - 构造器 / 非虚方法：可函数化 → `plain`；
  - 继承成员声明不下沉。
- **生成器**：拆层时用 `rava_macros_core` 解析块（`proc-macro2` 回落实现 + `span-locations` 取字节区间），对有标注的方法：方法项前插 `#[rava_moved = "<标注>"] `，体 `{ … }` 换成 `;`。实现层文本不变。
- **宏声明模式**：`rava_moved` 方法按标注走与有体时相同的分支：分类结论取标注；函数化外壳与 base 外壳只用签名生成；base 存根消息只用描述符。声明模式不再计算实现层的 vtable impl。完整 / 实现模式下出现 `rava_moved` 即 `compile_error`。
- **一致性断言（每次构建都编译）**：声明模式把本类全部标注按方法序拼成文本，取 FNV-1a 64 写成 `pub const __RAVA_MOVED_<X>: u64`；实现模式对完整块重算 `moved_fact`，生成 `const _: () = assert!(__RAVA_MOVED_<X> == <重算值>)`。生成器在回落实现下算出的标注与编译器记号流下宏的结论若有出入，实现 crate 编译失败，不会静默错配。

**等价性**
- 声明模式展开只依赖上表各结论与签名。标注 = 同一函数对同一块文本的结论（断言保证在编译器记号流下也成立），所以声明模式展开与 S4 逐项相同，只多一个 `u64` 常量。
- 实现层文本逐字节不变；实现模式展开只多一个常量断言。
- 完整模式（用户 crate、lib crate、Python 生成器）不受影响。

**验收**
- 27 例生成树对照：实现层逐字节一致；声明层差异只是「插标注 + 体换 `;`」，还原后与 S4 逐字节一致（对照脚本做还原比对）。
- 宏展开对照：用库对 27 例每个可拆类分别展开「S4 声明文本（带体）」与「S5 声明文本（剥体）」，记号文本一致（只差常量）。
- 测量：Digester 声明 crate 峰值（目标 ≤ 2 GB）、HelloWorld `cargo build` 墙钟（目标 ≤ 12 s）。
- 主会话 e2e 抽查：覆盖 Safe 定义 / 覆盖、`safe_stub`、`__impl_` 函数化、构造器与静态方法（含类初始化触发）、super 调用。

### 7.6 量化目标

| 指标 | 现状 | 终态 |
|---|---|---|
| 单个 rustc 峰值：实现 crate | —（今天只有一个 crate） | 每个 ≤ 1.5 GB |
| 单个 rustc 峰值：声明 crate | HelloWorld 1.97 GB；Digester 3.9 GB | HelloWorld ≤ 1.2 GB；Digester ≤ 2 GB |
| HelloWorld `cargo build` 墙钟（`CARGO_BUILD_JOBS=2`） | 16.0 s | ≤ 12 s |
| 仅用户类变化时 JDK 部分重编 | 全量 | 0 个 crate |

### 7.7 测量记录（生成器效率线）

#### S1：`java_meta` 拆出（2026-10-01）

条件：stable、`CARGO_BUILD_JOBS=2`、`CARGO_INCREMENTAL=0`、全新 target；每 crate 墙钟与峰值由 RUSTC_WRAPPER（`/usr/bin/time -l`）记录。「前」= 同一 scratch 换回 S1 之前的 `build.rs` / `class_impl.rs` / `anno_pool.rs`、去掉 `java_meta`。

表数据等价：HelloWorld、Digester 两例的 10 个表文件，去掉类型定义与导出属性两处机械差异后逐字节相同（Digester `method_table.rs` 7,984,036 B）。

HelloWorld：

| | 前 | 后 |
|---|---|---|
| `java_runtime` rustc | 21.4 s / 1671 MB | 19.9 s / 1319 MB（峰值 −21%） |
| `java_meta` rustc | — | 1.1 s / 396 MB |
| 仅改 user 源文件后的增量构建 | 重编 `java_runtime`：19.0 s / 1688 MB，总 20.7 s | 只重编 `java_meta`：1.0 s / 407 MB，总 2.9 s |

Digester（全机锁内测量，机器上无其他重进程；两侧用同一份冻结宏快照）：

| | 前 | 后 |
|---|---|---|
| `java_runtime` rustc | 122.2 s / 5047 MB | 116.7 s / 4979 MB（峰值 −1.3%） |
| `java_meta` rustc | — | 3.9 s / 1638 MB（与 `java_runtime` 后段流水并行） |
| 总墙钟 | 131.2 s | 124.7 s |

Digester 的峰值落在方法体与存储层（类型检查 / 借用检查 / 代码生成），表在其中占比小，S1 对它的峰值几乎无影响；收益在增量构建（user 变化不再重编 `java_runtime`）。峰值目标靠 S3–S6 的拆层。

链接：ld64 下 `hello_world` 正常链接，用到的 `__java_meta_*` 符号已解析，未用到的表被死代码剥除。

27 例生成树对照：每例差异 1196 行且完全相同，全部来自三类预期改动：
- 根 `Cargo.toml` 的 members 加 `java_meta`；`user/Cargo.toml` 加 `java_meta` 依赖；`user/src/main.rs` 加 `use java_meta as _;`；
- 新增 `java_meta/` 目录（`runtime/java_meta` 手写镜像）；
- `java_runtime` 手写 overlay：`build.rs`、新增 `src/meta.rs`、`class_impl.rs` / `anno_pool.rs` 改读 `crate::meta`，另有 5 个文件只改注释。

生成的类文件零差异，raw-audit 一致。

#### S2：存储钩子（2026-10-01）

- 宏新增 `gen/storage_hooks.rs`：每类在存储布局旁生成 3 个非泛型 `#[doc(hidden)] pub fn`（`__jb_<X>__alloc` / `__jb_<X>__cells` / `__jb_<X>__from_any`），§7.3.1 的 4 处调用点改调钩子。
- 等价性逐处：
  - `Default` / `__shallow_copy`：钩子体与原内联代码同为「`__Shared::new(Default)` → 先克隆出 vtable 视图、再转 `__AnyRef`」，求值顺序相同；`_jvm_null` 取值不变。
  - `__unsafe_*_cell` / `__unsafe_ref_access`：原为对 `&X__inner` 静态调用 `ObjectVTable` 方法，现为对同一对象的 `&dyn ObjectVTable` 调用，命中同一 impl；未命中时回落 `self.vtable` 的顺序不变。
  - `From<Object>` 路径 B：`downcast` 成功 → 部件同原式构造；失败 → 原样交还 `__AnyRef`，随后 `drop`，与原 `Err(__other) => drop(__other)` 相同。
- HelloWorld `java_runtime` 宏展开对照（nightly `-Z unpretty=expanded`，前后同一 scratch，仅宏 crate 不同）：差异 1543 处，逐类只有「新增 3 个钩子 + 4 处调用点替换」两类（190 类 × 3 钩子；删除行全部是上述 4 处的原内联代码）。`cargo build` 通过。
- 生成树：宏不进 scratch，生成树不变。

#### S3a：`_base` 去泛型 + 体只一份（2026-10-01）

- 宏：`_base` 接收者 `this: &__BT`（按调用方类型单态化）→ `this: &dyn X__VTable`；有 Safe 缺省方法的类加视图 trait `X__AsVTable`（§7.3.3）；`X__VTable for X__inner` 里 Safe VirtualDefine 的私有直展开删除、沿用缺省方法；Safe VirtualOverride 的 `_base` 所有者条目改为转发外壳。`virtual_dispatch/base_fns.rs` 首句 `let this = self;` 的剥离改为去空白后精确匹配（rustc 记号流文本化为 `let this = self;`，与 proc_macro2 回退实现的 `self ;` 不同，按原文比较会漏剥，导致 E0424）。
- 生成器（Rust / Python 两套）：继承成员转发体与 `super.m()` 去掉 turbofish 末位接收者类型。
- 27 例生成树对照（基线 `trees_s1`）：除基线早于 `build_script` 改名的目录名与 closure.json 计时字段外，代码差异全部是 `_base::<…, Self>(` / `_base::<…, _>(` → `_base::<…>(`、`_base::<Self>(` / `_base::<_>(` → `_base(`（新旧各 52,309 行，归一后多重集合相等）；closure.json 差异全部位于 `/summary/perf` 与 `elapsed_ms`（计时 / RSS 统计）；raw-audit 一致。
- HelloWorld `cargo build` 通过（java_runtime + user + 链接）。nightly 分阶段测量（`scripts/rustc_profile.sh`，仅 java_runtime）：

| | 前（S2） | 后（S3a） |
|---|---|---|
| 墙钟 / 峰值 RSS | 14.08 s / 1797 MB | 13.73 s / 1801 MB |
| 单态化条目 / 实例 / size_est | 27,357 / 53,603 / 576,201 | 29,016 / 54,628 / 582,466 |
| 其中 `_base` 条目 / 实例 | 420 / 1,444 | 2,268 / 2,280 |

- 解读：`_base` 去泛型后每个定义恰好编一份（之前只编被调用到的实例化，但一个体可按调用方类型编多份，如 `Throwable__toString_base` 32 份）；未被调用的 `_base`（多为 NeedsWrapper 的钩子桥与 stub）现在也会编出，size_est 合计 +1.1%，峰值与墙钟持平。这是拆层的前提形态：实现层的体函数必须是非泛型的单一定义，才能经 extern 边界被声明层调用，并在多个实现 crate 中并行编译。
- 运行期：Safe 体内对 `this` 的访问器调用由静态分派变为经 `&dyn` 分派（转发外壳与 `_base` 同 crate，LLVM 内联后 vtable 为常量可去虚化）。属性能观察项，随 S6 性能验收一并测。

#### S3b：wrapper 固有方法体函数化（2026-10-01）

- 宏（`wrapper/body_fns.rs`）：NeedsWrapper 的 `__impl_m`（VirtualDefine / VirtualOverride）与构造器 / 静态 / 私有实例方法，体移入模块级非泛型自由函数 `__jbm_<类>__<方法>(this: &X, ..)`，wrapper 方法变为外壳 `{ 空接收者检查; __jbm_X__m(self, ..) }`。S4 拆层时体函数改名为带签名哈希的 `__jb_` 导出符号、随存储层进实现 crate，外壳改经 extern 声明调用。
- 等价性：体函数语句就是宏改写（类初始化触发注入、字段 / base 调用改写、虚调用改写）完成后的原方法体，只做两处纯改名：接收者 `self` → 形参 `this`（首句 `let this = self;` 去掉；`self::` 路径前缀不改），`Self` → `X`（非泛型类二者同一类型）。外壳先做原位于体首的空接收者检查，再按原实参顺序转发、原样返回；类初始化触发仍在体首，求值顺序与副作用不变。
- 适用范围与保守回落（保持内联、形态不变）：泛型类（随 S6 擦除核心处理）、方法自身带泛型或 where 子句、接收者不是无生命周期的 `&self`、形参含非标识符模式、体内含 `impl`（嵌套项里 `Self` 另有所指）、`&self` 方法体内另有 `this` 与 `self` 同时出现。
- 生成器、生成树不变（只改宏展开）。HelloWorld `cargo build` 通过，无新增告警。nightly 分阶段测量（java_runtime）：

| | S3a | S3b |
|---|---|---|
| 墙钟 / 峰值 RSS | 13.73 s / 1801 MB | 14.13 s / 1826 MB |
| 单态化条目 / 实例 / size_est | 29,016 / 54,628 / 582,466 | 31,614 / 57,222 / 587,668 |
| 其中 `__jbm_` 体函数 | — | 2,603 条目，size_est 57,362 |

- 解读：体函数从 wrapper 方法（统计归 `gen`，−40,489）搬到 `__jbm_`（归 `macro`），外壳每个多一次调用，size_est 合计 +0.9%，峰值 +1.4%，墙钟 +2.9%（单次测量，含噪声）。单 crate 内这是纯搬移的固定开销；收益在 S4：体函数整体离开声明层，声明 crate 只剩外壳与样板。

#### S4：物理拆层（2026-10-01）

- 宏与生成器形态见 §7.5.1。测量工具 `scripts/crate_profile.sh`：对已生成的 scratch 预编依赖后，强制重编 workspace 自有 crate，经 RUSTC_WRAPPER（`/usr/bin/time -l`）记录每 crate 的墙钟和峰值 RSS，以及整次 `cargo build` 的墙钟（`CARGO_BUILD_JOBS=2`、`CARGO_INCREMENTAL=0`）。`PROFILE_TOUCH=user` 只 touch `user/src/main.rs`，测「仅用户类变化」时的重编集合。
- 27 例生成树对照（基线是 f00b6858 的临时 worktree；`/tmp` 下的分类脚本按 `split_text` 从基线文件重建期望文本后逐字节比较）：
  - 每个 `java_runtime/src` 生成类文件都等于「基线 + 块首 `#[rava_layer = "decl"]`」或与基线相同（手写、接口）；
  - 每个 `java_body_k/src/body/<rel>` 都等于从基线同路径文件重建的实现层文本，两个集合完全相同；
  - 其余差异只有以下几类：
    - 根 `Cargo.toml` 的 members 加 `java_body_k`（21 例 1 个、2 例 2 个、4 例 3 个）；
    - `user/Cargo.toml` 加 body 依赖行，`user/src/main.rs` 加 `use java_body_k as _;`；
    - `java_body_k` 脚手架（lib.rs / mod.rs / Cargo.toml）；
    - `java_meta/build_script/main.rs` 排除 `java_body_` 前缀；
    - 仓库路径与由路径派生的包版本号；
    - closure.json `/summary/perf` 与 `elapsed_ms` 的计时字段。
  - raw-audit 与 fallback-audit 27 例一致。
- 装箱校准：
  - 初版按 6 MiB 贪心装箱，Digester 实现 crate 为 1.44–1.55 GB，另有 0.06 MiB 的尾箱；
  - 改为均衡装箱：箱数 = max(⌈总字节 / 5 MiB⌉, 2)，各箱均分。至少两箱是为了声明层元数据就绪后让实现层两箱并行。
  - 均衡装箱只改变类文件归哪个 `java_body_k`，每个类文件的文本不变。上面的 27 例分类脚本不依赖箱的划分。
- 构建与运行：HelloWorld、DeepCopy、Digester 三例 `cargo build` 均 0 错误。DeepCopy 在 §6.2 时编译失败，现在能编过。HelloWorld 二进制运行输出正确，`nm` 可见 2417 个 `__rava_` 导出符号。
- 测量条件：stable，`CARGO_BUILD_JOBS=2`，`CARGO_INCREMENTAL=0`，依赖预编，其余为全新 target，全机锁内测量。

| HelloWorld | 前（§6.2 / §7.6） | S4 |
|---|---|---|
| `java_runtime`（声明层） | 1.97 GB | 11.0 s / 1246 MB |
| `java_body_1` / `java_body_2` | — | 4.7 s / 502 MB；4.1 s / 512 MB |
| `java_meta` | — | 0.6 s / 405 MB |
| bin | — | 0.2 s / 184 MB |
| `cargo build` 墙钟 | 16.0 s | 15.9 s（单箱时 18.1 s） |
| 仅改 user 源文件 | — | 2.0 s：只重编 `java_meta`（含用户类元数据，S1 设计）与 bin，`java_runtime` / `java_body_*` 0 个 |

| Digester | 前（§7.7 S1，stable） | S4 |
|---|---|---|
| `java_runtime`（声明层） | 116.7 s / 4979 MB | 53.8 s / 4509 MB |
| `java_body_1..4` | — | 11.4–15.0 s / 1196–1261 MB |
| `java_meta` | 3.9 s / 1638 MB | 2.7 s / 1291 MB |
| `cargo build` 墙钟 | 124.7 s | 82.7 s |

| DeepCopy | 前（§6.2，借用检查处失败） | S4 |
|---|---|---|
| `java_runtime`（声明层） | ≥ 6.11 GB（失败退出） | 76.3 s / 5437 MB |
| `java_body_1..6` | — | 11.7–13.8 s / 1112–1244 MB |
| `java_meta` | — | 3.7 s / 1691 MB |
| `cargo build` 墙钟 | — | 116.6 s |

对照 §7.6：

| 指标 | 终态目标 | S4 实测 | 结论 |
|---|---|---|---|
| 实现 crate 峰值 | 每个 ≤ 1.5 GB | 最大 1.26 GB（Digester / DeepCopy） | 达成 |
| HelloWorld 声明 crate 峰值 | ≤ 1.2 GB | 1.25 GB | 差 0.05 GB |
| Digester 声明 crate 峰值 | ≤ 2 GB | 4.51 GB | 未达 |
| HelloWorld 墙钟 | ≤ 12 s | 15.9 s | 未达 |
| 仅用户类变化时 JDK 重编 | 0 个 crate | 0 个（`java_meta` 属元数据，含用户类） | 达成 |

**声明层为何高于 §7.1 的 3.21 GB 预测**

Digester 声明 crate 的 nightly 分阶段测量（`scripts/rustc_profile.sh`）：49.4 s，峰值 4.65 GB。

| 阶段 | 耗时 | RSS |
|---|---:|---|
| 宏展开 | 17.3 s | 45 → 1650 MB |
| 名称解析 + HIR 降级 | — | → 2496 MB |
| coherence | 1.8 s | → 2886 MB |
| 类型检查 | 8.6 s | → 3765 MB |
| 借用检查 | 9.9 s | → 4214 MB |
| 单态化收集 / 元数据 / codegen | 2.0 / 3.6 / 4.6 s | → 4.7–4.97 GB |

- 后端已大幅缩小：单态化 size_est 从 419.5 万降到 114.5 万。剩余部分都是声明层样板，前列为：
  - `From` 8.1 万；
  - `ObjectVTable for X` 的 `__view_into` / `__erased_vtable` / `__virtual_view` / `__shallow_copy` / `__view_as` / `__erased_inner` / `__unsafe_*`，合计约 15 万；
  - `__class_init` 2.3 万；
  - `__reflect_field` 2.3 万。
  
  这与 §7.4 的样板构成一致。
- 峰值逐阶段累积，不集中在某一阶段。§7.1 实验测的是「去掉方法体后的展开结果」直接编译，S4 的声明层比它多出三类：
  1. **声明层源码仍含整块文本（含方法体）**：
     - Digester 声明 crate 源码 19.9 MB，其中 17.8 MB 与实现层重复；
     - 宏在声明模式下仍要词法分析、解析、改写全部方法体，再丢弃实现层部分；
     - 宏展开 17.3 s，占声明 crate 的 35%；
     - §7.1 实验 1 测得方法体记号使展开后 RSS 多约 0.43 GB。
  2. **泛型类方法体与接口 default / static 方法体仍在声明层**：它们随 S6 下沉。
  3. **每个下沉函数在声明层多一个 extern 外壳**：Digester 有 10,508 个 `__jb` 外壳，size_est 2.1 万，前端按函数定义计费。
- 因此声明层降到 2 GB 需要 S5 / S6，并新增一项（列入 S5）：
  - **声明层文本不带下沉方法体**：生成器写声明层时，把已下沉方法的方法体换成空体标记，宏的声明模式不再接收、解析这些记号。
  - 前提：宏决定「是否下沉」的判据不能读方法体。今天的判据里，回落条件（体内含 `impl`、`this` 与 `self` 同时出现）和存根判定（体为 `panic!("stub: …")`）都要读体。需要先把这些判据改为由生成器在块属性里显式标注，两层读同一标注，再做剥体。
  - 等价性要逐项论证，属不确定的形态改动，本步不做。
- 墙钟：HelloWorld 的关键路径是「声明层到元数据（约 9 s）→ 实现层两箱并行（约 4.7 s）→ 链接」。≤ 12 s 需要声明层降到 §7.1 预测的约 6 s 量级，取决于 S5（含上面的剥体项）。实现层再细分不能缩短关键路径（`CARGO_BUILD_JOBS=2`）。

#### S5 剥体（2026-10-01，§7.5.2）

- 范围：只做 §7.5.2 的剥体（声明层文本不带下沉方法体）。§7.4 前三项（`use` 列表、`ObjectVTable` if 链、字段访问器）未做，仍列在 S5。
- 27 例生成树对照：
  - 基线是 S4 的 8cbcfc57，S5 树来自 3504dd5f，工具为 `runtime/rava_macros_core/examples/decl_check.rs` 加上实现层 `diff -r`；
  - 声明层：每个声明层类文件都等于「S4 同名文件按剥体计划插标注、体换 `;`」，块外文本逐字节一致；
  - 声明模式展开（带体 vs 剥体）逐记号一致，归一项只有摘要常量值和 `java_try` 标签序号：
    - 标签序号来自进程级计数器，只要求函数内唯一；
    - 对照进程里 S4 侧多解析了下沉体，后续序号整体平移；
    - 按首次出现重编号后一致；
  - 结果：27 例 bad = 0，单例下沉方法体 4,648–13,284 个，声明层文本 −21% 到 −31%（Digester 未在 27 例内；DateTimeFormat 为 16.9 MB → 11.7 MB）；
  - 实现层 `java_body_k/src` 逐字节一致，箱数一致；
  - 声明层之外的 `java_runtime/src` 文件无差异；
  - raw-audit / fallback-audit 27 例一致；
  - 其余差异只有两类：
    - Cargo.toml 的仓库路径与由路径派生的包版本号；
    - closure.json 的 `/summary/perf` 与 `elapsed_ms`。
- 构建与运行：HelloWorld、Digester `cargo build` 0 错误（实现层常量断言全部通过），二进制输出与 JVM 一致。
- 测量（S4 = 8cbcfc57，S5 = 3504dd5f，同一次持锁背靠背，`scripts/crate_profile.sh`）：

| | S4 | S5 |
|---|---|---|
| HelloWorld `java_runtime`（声明层） | 11.97 s / 1095 MB | 11.04 s / 1210 MB |
| HelloWorld `cargo build` 墙钟 | 17.2 s | 16.9 s |
| Digester `java_runtime`（声明层） | 59.7 s / 3085 MB | 52.7 s / 3421 MB |
| Digester `java_body_1..4` | 13.4–15.4 s / 1126–1252 MB | 12.3–17.9 s / 1067–1245 MB |
| Digester `cargo build` 墙钟 | 91.3 s | 86.5 s |

  - 同一 S5 树隔一次持锁再测，Digester 声明层为 61.1 s / 3167 MB，实现层逐字节相同的 `java_body_1` 为 21.1 s。可见单次测量的噪声：峰值约 ±10%，墙钟约 ±20%。
  - 基线更正：§7.7 S4 记录的 Digester 声明层 4509 MB 测于并入闭包精度三期（prec3 / mono）之前。同一 S4 生成器在 8cbcfc57 上测得 3085 MB，下降来自闭包缩小，与剥体无关。
- Digester 声明 crate nightly 分阶段（S5，`scripts/rustc_profile.sh`）：48.0 s，峰值 3.11 GB。

| 阶段 | 耗时 | RSS |
|---|---:|---|
| 宏展开 | 11.6 s | 45 → 1399 MB |
| 名称解析 | 0.9 s | → 1680 MB |
| coherence / 类型检查 | 2.2 / 10.0 s | 2410 → 2139 MB |
| 借用检查 | 10.5 s | → 2418 MB |
| misc_checking_3 | 1.2 s | → 2740 MB |
| 单态化收集 | 2.8 s | 峰值 3122 MB |
| 元数据 / codegen / LLVM | 5.0 / 5.2 / 4.9 s | 2.2–2.75 GB |

  - 单态化 size_est 114.6 万（72,104 项）。前列：`Drop` 7.9 万；`From<Object>` 6.9 万；`new` 4.9 万；`checkcast` 3.5 万；`ObjectVTable` 的 `__view_into` / `__erased_vtable` / `__shallow_copy` / `__view_as` 合计 10.0 万，另有 `__virtual_view` 2.4 万；`__class_init` 2.3 万；`__reflect_field` 2.3 万。
- 结论：
  - 剥体只缩短宏展开阶段：宏展开从 S4 记录的 17.3 s 降到 11.6 s，但两次测量的闭包不同，只作量级参考。
  - 峰值不在宏展开：峰值落在单态化收集，且由类型检查起各阶段逐步累积；剥体对峰值和总墙钟没有可测改善。
  - 剥体的价值在结构：声明层不再携带实现层文本，JDK 方法体变化不再使声明 crate 失效。§7.6 的两项仍未达成：
    - Digester 声明层 3.1–3.4 GB，目标 ≤ 2 GB；
    - HelloWorld 墙钟 16.9 s，目标 ≤ 12 s。
  - 剩余手段按单态化 / 前端占比排列：
    - §7.4 的 `ObjectVTable` 表驱动（约 12.4 万 size_est）；
    - `From<Object>` / `checkcast` / `new` 的每类单态化（约 15 万，S5 新增调查项）；
    - `use` 列表按层计算；
    - S6 泛型类擦除核心；
    - 字段访问器。
  - 逐项测量仍按 §7.4。

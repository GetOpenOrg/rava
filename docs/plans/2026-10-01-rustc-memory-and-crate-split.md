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

（第 1 步完成后填写：各阶段内存峰值、单态化实例数、decl 与 body 的体量估算。）

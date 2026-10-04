# T1 第 2 步：驱动直接调用 rustc 链接档案（方案 + 最小实测）

> 状态（2026-10-04）：方案与本机最小实测完成；用户已答复三个决策点（§七），本文已按答复修订，档案按 JDK 模块切分 crate。
> 未改生成器 / 驱动 / runtime 主线逻辑。
> 上游：[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md) §4.4、§5.3 第 2 步、§6（1a / 1b）；
> crate 分层：[`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md) §7。
> 本文只写终态，不写过渡方案。

## 一、结论摘要

1. **档案按 JDK 模块切分 crate，切分成立。**
   - 每个出现在档案中的 JDK 模块对应一个模块 crate，以模块名命名：`java.base` → `java_base`，`jdk.crypto.ec` → `jdk_crypto_ec`。
   - 类与模块的归属、模块之间的 requires 关系都从所用 JDK 的 jimage / jmod 模块描述动态读取，生成器里不出现字面量。
   - 实测 s2 档案（3077 类，15 个模块）的生成代码：
     - 正文里实际使用的跨模块引用共 3,424 条，**全部沿 requires 方向**；
     - 反向与互不可达的引用为 **0**；
     - 宏属性中的类名引用同样为 0 反向；
     - 手写代码只引用 java.base。
   - 唯一的反向来源是**未使用的 `use` 导入**：
     - 反向 3,248 + 3,171 条，互不可达 335 + 330 条（声明层 + 实现层）；
     - 都来自生成器 `dispatch_subtype_refs`，它按调用 owner 收集全部 JDK 子类型作为导入，正文并不使用（§4.7）。
2. **31,048 个未定义符号的环，在按模块切分后不再跨越模块边界。**

   | 符号 | 数量 | 切分后的归宿 |
   |---|---:|---|
   | `__java_meta_*` | 22 | 改为每个模块在自己的 crate 中登记数据（§3.3），环消除 |
   | 非 java.base 类的 `__rava_*` | 2,645 | 这些模块单 crate 即可，无需拆层，环消除 |
   | java.base 类的 `__rava_*` | 28,381 | 保留，是 java.base 内部「声明层 ↔ 实现层」拆分的链接机制，闭合在 `java_base` 模块产物内部 |

   最小模型实测：
   - 跨模块的 `__rava_*` 引用只出现在内联外壳中，方向是下游引用上游，绑定到上游模块的 dylib；
   - static / thread_local / `OnceLock` / `Rc<dyn ObjectVTable>` 跨模块 dylib 时唯一且正确。
3. **语料模式每个模块一个 dylib，没有全局门面。**
   - 模块 crate 本身就是该模块的 dylib（生产模式下为 rlib），dylib 之间的依赖与 requires DAG 一致；
   - 被拆成多层的模块（实际上只有 `java_base`）由模块 crate `pub use` 自己的声明层，并静态吸收自己的实现层；
   - 下游模块与用户 crate 只引用模块 crate，例如写 `java_base::java::lang::String`。
4. **按模块切分不能替代 crate-split。**

   | 项 | java.base 占比（s2） |
   |---|---:|
   | 类 | 2726 / 3077 |
   | 档案源码 | 91.5% |
   | 门面导出符号 | 95.6% |

   因此：
   - 单例链接要读入的导出符号总量基本不变，dylib 个数不影响单例耗时；
   - 档案构建峰值的瓶颈仍是 java.base 的声明层，按比例估算从 6.33 GB 降到约 5.8 GB；
   - 模块 dylib 链接峰值的瓶颈是 `java_base`，按比例估算约 3.3 GB，单 dylib 时为 3.48 GB。

   建议 crate-split 的切分轴改为「先按模块，再在超阈值的模块内按体量切实现层」。实际只有 java.base 需要拆层；协调接口见 §4.8。
5. **语料模式用 `panic = "unwind"`（用户已批准）**：Rust dylib 与 `-C prefer-dynamic` 只能链接工具链自带的 std dylib，它内置 `panic_unwind`。`create_java_vm` 的钩子在 unwind 开始前就 `exit(101)`，可观测行为与今天相同。生产模式保持 abort。
6. **单例数字（s2 档案，本机 arm64 macOS，dylib）**：
   - 用户 crate 直接调用 rustc：1.8–2.5 s，峰值 625–686 MB；
   - 把档案侧登记表移出用户 `main.rs` 后：**0.58–0.96 s，峰值 455–507 MB**，二进制 0.18–0.70 MB，输出与 `tests/expected` 一致；
   - 静态链接对照：峰值 2.3–2.6 GB，超出目标。
   - 登记表移出后的归属：每个模块的行登记在该模块 crate 中，用户 `main.rs` 只按拓扑序为每个模块调用一行 `<模块>::__rava_register_module()`。s2 档案约 1,370 行变为 15 行。
7. **生产模式（用户已定）**：静态链接 rlib，开发构建不开 LTO（只改用户代码 0.58 s），`--release` 用 fat LTO（同场景 47 s，体积 −20%）。

## 二、现状（实验记录）

**实验环境**
- 工作树 `java_rta_t1link`，分支 t1-link，从 10cfb657 起（1b 的提交）。rava 二进制用 `java_rta_t1b` 下同一提交的构建。
- rustc 1.98.1，aarch64-apple-darwin，ld `ld-27037.1`，16 GB 内存，JDK 21.0.11（Homebrew）。
- 所有重编译都经 `heavy_lock`，`CARGO_BUILD_JOBS=2`。
- 实验目录 `build/t1link/`（gitignore）。

**实验脚本（`scripts/diag/`）**

| 脚本 | 作用 |
|---|---|
| `t1link_emit_set.sh` | 建档案，逐个程序 `--profile` 发射 |
| `t1link_facade.sh` | 把档案 rlib 打成单个门面 dylib（实验对照，终态不采用） |
| `t1link_user_rustc.sh` | 直接调用 rustc 编译用户 crate，rlib / dylib 两种链接，记录时间、峰值、运行输出 |
| `t1link_release.sh` | 生产模式首构，与「只改用户 crate」后的重建 |
| `t1link_module_edges.py` | 按 JDK 模块统计生成代码的跨模块引用边、每模块体量、用户 `main.rs` 登记行的归属 |
| `t1link_exports_by_module.py` | 门面 dylib 的导出符号按模块归类 |
| `t1link_module_toy.sh` | 两模块最小模型：每模块一个 dylib，与单 dylib 对照 |
| `t1link_module_toy_reexport.sh` | 终态形态最小模型：模块名 crate `pub use` 声明层，导出 `__rava_register_module`；dylib / rlib 两种模式 |

### 2.1 档案

| 档案 | 内容 | 程序 | 档案 crate 是否逐字节相同 |
|---|---|---|---|
| s1 | 465 类、1809 方法（1b 的 s1） | HelloWorld、ControlFlowTest | 是 |
| s2 | 3077 类、17754 方法，6 个入口，15 个 JDK 模块 | HelloWorld、TestBridgeMethod、LambdaBasic（另有 AnonClassTest、ReflectionBasic、ObjectMethods 只入档案，不发射） | 是 |

- s2 的 `rava profile`：18.8 s，峰值 1453 MB。
- s2 规模：`java_runtime` 源码 42 MB，`java_body_1..10` 每个约 5.2 MB。

### 2.2 实测数字

**档案构建（每个键一次）**

| 项 | s1 | s2 |
|---|---:|---:|
| cargo 构建档案 + HelloWorld（dev，abort） | 34.5 s / 2.08 GB | — |
| 同上（dev，unwind） | 34.6 s / 2.19 GB | 248 s / **6.33 GB**（峰值来自 `java_runtime`） |
| 档案 rlib 体积 abort → unwind | `java_runtime` 112 → 125 MB，body +17% | 合计 1.67 GB |
| 单门面 dylib 链接（unwind + prefer-dynamic） | 1.07 s / 568 MB / 76 MB | **8.75 s / 3.48 GB / 510 MB**，导出 650,965 个符号 |
| 语料包：rmeta + dylib | — | 442 + 510 MB，`zstd -3` 后 **202 MB** |

**单例：用户 crate 直接调用 rustc，每格 2–3 次取范围**

| 程序（档案） | 链接 | `main.rs` | 墙钟 | 峰值 RSS | 二进制 | 输出 |
|---|---|---|---:|---:|---:|---|
| HelloWorld（s1） | 静态 rlib | 今天形态 | 0.88 s | 338 MB | 42 MB | 一致 |
| HelloWorld（s1） | dylib | 今天形态 | 0.25–0.31 s | 195 MB | 0.37 MB | 一致 |
| ControlFlowTest（s1，链接 HelloWorld 的档案产物） | dylib | 今天形态 | 0.24 s | 192 MB | 0.35 MB | 一致 |
| HelloWorld（s2） | 静态 rlib | 今天形态 | 3.1–4.3 s | 2.55 GB | 440 MB | 一致 |
| HelloWorld（s2） | 静态 rlib | 档案行移出 | 1.64–1.68 s | 2.29 GB | 346 MB | 一致 |
| HelloWorld（s2） | dylib | 今天形态 | 1.82–2.54 s | 625 MB | 2.9 MB | 一致 |
| TestBridgeMethod（s2） | dylib | 今天形态 | 2.02–2.13 s | 658–686 MB | 3.4 MB | 一致 |
| LambdaBasic（s2） | dylib | 今天形态 | 1.80–1.82 s | 624 MB | 2.9 MB | 一致 |
| HelloWorld（s2） | dylib | 档案行移出 | **0.55–0.59 s** | **455 MB** | 0.18 MB | 一致 |
| TestBridgeMethod（s2） | dylib | 档案行移出 | **0.77–0.96 s** | **458–507 MB** | 0.70 MB | 一致 |
| LambdaBasic（s2） | dylib | 档案行移出 | 1.28 s（单次，user 0.72 s，rmeta 冷缓存） | 464 MB | 0.15 MB | 一致 |

- 「档案行移出」：从 `main.rs` 删去档案类的类初始化钩子、反射方法 / 字段分派、VM 引导初始化各行，模拟 §3.3 的终态，删后 `main.rs` 剩 23–38 行。这是实验近似：这三个用例运行时不经由被删除的 JDK 类反射分派。
- ControlFlowTest 一行用另一程序的档案产物链接，结果正确，直接验证了「同一档案、不同程序共用一份产物」。

**单例 rustc 耗时构成**（s2 HelloWorld，今天的 `main.rs`，`-Ztime-passes`，借 `RUSTC_BOOTSTRAP` 仅作诊断）

| 阶段 | 耗时 |
|---|---:|
| 总计 | 1.80 s |
| 类型检查 | 0.50 s |
| 借用检查 | 0.27 s |
| 单态化收集 | 0.23 s |
| codegen | 0.41 s |
| 链接 | 0.41 s |

- 档案行移出后墙钟为 0.58 s，其中链接仍约 0.4 s。链接耗时与 `main.rs` 内容无关，与读入的导出符号量有关，是剩余的大头。

**参数扫描**（s2，档案行移出，dylib）

| 参数 | HelloWorld | TestBridgeMethod |
|---|---:|---:|
| 基线（opt 0，cgu 缺省，line-tables-only） | 0.59 s | 0.80 s |
| `-C debuginfo=0` | 0.57 s | 0.77 s |
| `-C codegen-units=1` | 0.61 s | 0.86 s |

- ld64.lld 不可用：`--ld-path=<sysroot>/.../gcc-ld/ld64.lld` 链接失败，报 `rust-lld: could not load TAPI file ... MacOSX27.0.sdk/usr/lib/libiconv.tbd: malformed file`，rust-lld 读不了本机新版 SDK 的 tbd。未计入。Linux 缺省就是 rust-lld，不受影响。

**运行期**：dylib 二进制启动加 HelloWorld 运行 0.02 s，与静态二进制相同。

**单态化落点**（`-Zprint-mono-items`，档案行移出）

| 程序 | 单态化项 | 其中 std | 其中定义在 java_runtime |
|---|---:|---:|---:|
| LambdaBasic | 182 | 66 | 13 |
| TestBridgeMethod | 1264 | 296 | 55 |

**生产模式**（s1 HelloWorld，cargo release：opt 3，cgu 1）

| LTO | 首构 | 只改用户 crate | 二进制 |
|---|---:|---:|---:|
| fat（今天的 release 配置） | 102.5 s / 3.27 GB | **47.1 s** / 3.29 GB | 16.5 MB |
| off | 77.3 s / 2.95 GB | **0.58 s** / 245 MB | 20.8 MB |

### 2.3 按模块切分的实测（s2 档案，`t1link_module_edges.py` / `t1link_exports_by_module.py`）

**每模块体量**（声明层 + 实现层 + 手写；手写基础设施与 VM 支持类按包归 java.base）

| 模块 | 类 | 源码（MB） | 导出符号 |
|---|---:|---:|---:|
| java.base（含手写基础设施 0.59 MB） | 2726 | 75.9 | 约 622k（95.6%，含 `__rava_*`、std 实例化） |
| java.xml.crypto | 135 | 1.92 | 8,278 |
| jdk.crypto.ec | 84 | 1.53 | 7,799 |
| jdk.crypto.cryptoki | 26 | 0.89 | 3,053 |
| java.logging | 31 | 0.80 | 3,438 |
| jdk.random | 10 | 0.64 | 1,036 |
| 其余 9 个模块 | 合计 62 | 合计 1.30 | 合计 5,272 |

- 「其余 9 个模块」：java.security.jgss、java.xml、java.smartcardio、java.security.sasl、jdk.security.jgss、java.naming、jdk.charsets、jdk.localedata、jdk.zipfs。
- 非 java.base 合计：类约 350 个，源码 7.1 MB（8.5%），导出符号 28,876 个（4.4%）。

**跨模块引用边**（类级去重；只计正文中实际使用的名字与全路径，已去掉字符串字面量和注释）

| 来源 | 同模块 | 沿 requires（fwd） | 反向 / 互不可达 |
|---|---:|---:|---:|
| 声明层（生成） | 18,448 | 1,434 | **0** |
| 实现层（生成） | 25,268 | 1,990 | **0** |
| 手写 `<x>_impl.rs` / `_ext.rs` | 108 | 0 | **0** |
| 手写基础设施（error.rs 等） | — | 56（全部指向 java.base） | **0** |
| 宏属性中的类名（`super_class` / `interfaces` / `all_supertypes` / `inner_classes` / `nest_members` / `permitted_subclasses` / `generic_signature` / `enclosing_method` 等） | 60,709 | 2,735 | **0** |
| **`use` 导入行（含未使用）** | — | — | **声明层：反向 3,248、互不可达 335；实现层：反向 3,171、互不可达 330** |

- fwd 边中最多的模块对：jdk.crypto.ec → java.base（1,044），java.xml.crypto → java.base（728），java.logging → java.base（361），java.xml.crypto → java.xml（136）；java.base 以外的模块之间只有 java.xml.crypto → java.xml / java.logging、jdk.security.jgss → java.security.sasl / java.security.jgss。
- `use` 导入行反向的例子：`java/io/print_stream.rs` 与 `java/lang/runtime_exception.rs` 中有 `use crate::sun::security::pkcs11::ConfigurationException;`，而正文从未使用（文件有 `#![allow(unused_imports)]`）。487 个文件导入了该类。来源见 §4.7。

**`java_runtime` rlib 的未定义符号**（s2，共 31,048）

| 符号 | 所属模块 | 数量 |
|---|---|---:|
| `__java_meta_*` | （档案元数据表） | 22 |
| `__rava_*` | java.base | 28,381 |
| `__rava_*` | 其余 14 个模块 | 2,645（java.xml.crypto 723、jdk.crypto.ec 568、jdk.crypto.cryptoki 365、java.logging 365 等） |

- 链接符号 `__rava_<二进制名转义>__<函数>_<指纹>` 只用于类自己的声明层外壳调用自己的实现层定义（`rava_macros_core/src/block/gen/layer.rs`）。因此「声明层 ↔ 实现层」的符号环是同类、同模块的。

**用户 `main.rs` 中引用档案类的行，按目标模块计**（s2 HelloWorld）
- java.base 1,290 行；
- 其余 14 个模块合计 70 行（jdk.crypto.cryptoki 13、jdk.random 10、java.xml.crypto 8 等）；
- VM 支持类 2 行。

**最小模型**（`t1link_module_toy*.sh`；两个模块 a ← b，各含声明层 / 实现层与 `export_name` 环）

| 形态 | 结果 |
|---|---|
| 每模块一个 dylib：liba 含 a_decl + a_body；libb 含 b_decl + b_body，依赖 liba | rustc 接受「上游 dylib 已静态包含 a_decl、下游 rlib 依赖 a_decl」，a_decl 不重复链接；libb 的唯一未定义 `__rava_` 符号是内联外壳带来的 `__rava_a_A__hello`，lazy-bind 到 `liba`（下游 → 上游）；输出正确，计数器、`OnceLock`、thread_local 跨 dylib 唯一 |
| 单 dylib 含全部模块 crate（对照） | 同样正确 |
| 终态形态：模块名 crate `a` / `b` 以 `pub use` 导出各自声明层并导出 `__rava_register_module()`；下游与 bin 只经 `a::` / `b::` 引用；bin 按拓扑序登记 | dylib 模式（unwind + prefer-dynamic）与 rlib 模式（abort，静态）都通过；`Rc<dyn a::ObjectVTable>` 装 a::A 与 b::B 的动态分派正确；`otool -L` 显示 libb → liba → libstd，main → liba、libb、libstd |

### 2.4 失败与被取代的路线

**F1 同一模块的声明层、实现层各编一个 dylib**
- 声明层通过 `extern "Rust"` 引用实现层 `#[export_name]` 定义的符号，被引用方又依赖声明层，构成符号环（java.base 部分 28,381 个）。
- macOS 链接 dylib 不允许未定义符号，除非加 `-undefined dynamic_lookup`；那样会把签名不匹配推迟到运行期，违背 crate-split §7.3.2「签名不一致即链接失败」。
- Linux 下可执行文件不直接引用实现层，`--as-needed` 会丢掉它。
- 结论：环必须闭合在同一个链接产物里，即模块 dylib 内部。

**F2 panic=abort + dylib 链接 std dylib（`-C prefer-dynamic`）**
- rustc 报 `the linked panic runtime panic_unwind is not compiled with this crate's panic strategy abort`。
- sysroot 的 `libstd-*.dylib` 内置 `panic_unwind`，稳定版无法重编 std。

**F3 panic=abort + dylib 静态吸收 std**
- dylib 能构建（1.63 s，59 MB），但用户 crate 链接时报 `cannot satisfy dependencies so std only shows up once`。
- 绕开它要改工具链布局，排除。

**F4 语料模式静态链接 rlib**：单例链接峰值 2.3–2.6 GB，超出 ≤ 1 GB；二进制 346–440 MB。只留给生产模式。

**F5（被取代）单个全局门面 dylib `java_profile`**
- 技术上可行（§2.2 的单 dylib 数字即由它测得）。
- 被用户「按 jmod 模块命名」的决定和本文的模块切分取代：模块 crate 自身就是 dylib，不再需要一个不对应 JDK 命名空间的自造顶层 crate。
- 单例耗时与之相同：导出符号总量相同。

## 三、终态设计

### 3.1 crate 布局：一个 JDK 模块一个模块 crate

```
java_base（模块 crate；语料 dylib / 生产 rlib）
  ├─ pub use java_base_decl::*            ← 声明层（结构体、外壳、泛型方法体、trait、ObjectVTable、手写基础设施）
  ├─ 静态吸收 java_base_body_1..N         ← 实现层（export_name 定义），环闭合在 java_base 产物内
  └─ pub fn __rava_register_module()      ← 本模块的元数据表 / 反射分派 / 类初始化钩子 / 模块资源 / 行表
java_logging、java_xml、jdk_crypto_ec …（模块 crate；体量在阈值内 → 单 crate，不拆层，无 export_name 环）
  ├─ 依赖：本模块实际引用到的上游模块 crate（⊆ requires 传递闭包）
  └─ pub fn __rava_register_module()
user（bin）
  ├─ 依赖全部档案模块 crate
  └─ main：按模块拓扑序逐个调用 <模块>::__rava_register_module()，再 register_user，再 create_java_vm
```

**命名**
- 模块 crate 名 = 模块名中的 `.` 换成 `_`。
- 被拆层的模块，内部 crate 名由模块 crate 名加层后缀派生：`<m>_decl`、`<m>_body_<k>`。
- 所有名字都由 JDK 模块描述（jimage / jmod）在生成时得出。生成器不写模块名字面量，`no_jdk_literals` 守护范围不变。

**类到模块的归属**
- JDK 类：按 jimage 中的所属模块。
- lambda / 隐藏类：按宿主类。
- `runtime/java_support` 的 VM 支持类（Proxy$Dyn、Species_Dyn、InjectedInvoker$Dyn 等）与 `runtime/java_runtime` 手写基础设施（gil、meta、reflect_dispatch、error、exec_context、array 等）：按包归属，实测全部是 java.base 的包，所以归 `java_base_decl`。
- 手写 `<x>_impl.rs` 与类 x 同 crate（共置原则不变）。

**模块依赖**
- 模块 crate 的依赖 = 本模块生成代码实际引用到的模块。
- 生成器断言这些依赖 ⊆ requires 传递闭包，越界即生成失败；由守护测试与 `t1link_module_edges.py` 同口径检查。
- JPMS 保证 requires 无环，所以 crate 图无环；拓扑序也就是 VM 登记序。

**全局对象的归属**

| 对象 | 归属 | 说明 |
|---|---|---|
| `ObjectVTable` / `Object` / `JArray` / GIL / 异常与 `Result` 基础设施 | `java_base_decl` | 其他模块的类在各自 crate 中 `impl ObjectVTable`；类型是本 crate 的，不违反孤儿规则 |
| 元数据表（类层次、字段、方法、修饰符、record、嵌套、注解常量池、行表；今天的 `java_meta` 与 `closure_input/*_tables.rs`） | 各模块 crate 中本模块的行 | `__rava_register_module()` 把切片交给 `java_base` 的 meta 查询面合并；`__java_meta_*` extern 取消 |
| 反射方法 / 字段分派、类初始化钩子（今天在用户 `main.rs`） | 各模块 crate 中本模块的行 | 写成静态 fn 指针表，不再逐行 `__Shared::new(闭包)` |
| 模块服务表（provides） | 提供方模块 | `uses` 方（多为 java.base）在运行期查表，没有链接期边 |
| 模块资源（seeds `[module_resources]`） | 资源所属模块 | |
| VM 引导初始化序列、VM 系统属性 | `java_base` | |
| 用户类的元数据与反射行 | 用户 crate | `register_user` 不变 |

### 3.2 两种模式

| | 语料模式（`rava build --profile`） | 生产模式（单项目，档案即项目闭包） |
|---|---|---|
| 模块 crate 类型 | dylib（每模块一个，依赖与 requires DAG 一致） | rlib |
| 档案产物 | 模块 dylib，声明层 rmeta，传递依赖 rmeta，std dylib 副本，`rava_macros` | 全部 rlib，`rava_macros` |
| panic | unwind（档案与用户 crate 一致；钩子 `exit(101)`） | abort |
| 用户 crate 链接 | `-C prefer-dynamic`，rpath 指向档案 `lib/` | 静态；开发构建不开 LTO，`--release` 用 fat LTO |
| 单例产物 | 只含用户 crate 的小二进制（实测 0.15–0.7 MB） | 完整原生二进制 |

两种模式下用户 crate 源码相同，只是 rustc 参数不同。

### 3.3 登记：用户 `main.rs` 的终态形态

```rust
fn main() {
    java_base::__rava_register_module();          // 按模块拓扑序，每个档案模块一行
    java_logging::__rava_register_module();
    java_xml::__rava_register_module();
    // …
    java_runtime_entry(&USER_META, user_dispatch_rows, || Main::main(args));   // 用户行 + 启动（形态沿用今天）
}
```

- 语料模式下每个程序登记档案的全部模块：档案是全体入口的并集，反射 / ServiceLoader / JCA 可能用到程序自己不直接引用的模块。
- 登记只是把静态切片指针并入表，s2 共 15 次调用。
- 量化：用户 `main.rs` 中档案类登记行 **0**（今天约 1,370 行），档案相关行数 = 档案模块数。

### 3.4 档案构建（每个 `B` 一次，构建者见 cross-test §4.3）

1. 生成档案树：各模块 crate（含 java_base 的声明层 / 实现层）与 `closure_input/`，只依赖 `profile.json`。
2. cargo 构建全部 crate 的 rlib：`--message-format=json`，panic 取模式值，`--remap-path-prefix=<树>=/rava/profile`。
3. 语料模式：rava 按模块拓扑序直接调用 rustc，把每个模块 crate 链成 dylib。不经 cargo，因为 `-C prefer-dynamic` 若写进 RUSTFLAGS 会波及 proc-macro 与构建脚本。参数：
   - `--crate-type dylib -C prefer-dynamic -C panic=unwind`；
   - `--extern` 本模块的 rlib 与上游模块的 dylib；
   - macOS：`-Wl,-install_name,@rpath/lib<m>.dylib -Wl,-rpath,@loader_path`；
   - Linux：`-Wl,-rpath,$ORIGIN`（`DT_RUNPATH` 不作用于传递依赖，每个模块 `.so` 都要能找到同目录的上游与 std）。
4. 落盘 `~/.cache/rava/archive/jdk<M>/<B>/`：
   - `lib/`：模块 dylib（生产模式为 rlib），`<m>_decl` rmeta，传递依赖 rmeta，std dylib 副本，`rava_macros`；
   - `manifest.json` 最后写入，写入后改名生效，flock 互斥。

### 3.5 `manifest.json` 与键（补充 cross-test §4.1 的 L2 键 `B`）

`B` 在 §4.1 的输入之外加入：模式（corpus / production）、panic 策略、crate-type、模块拓扑与链接参数、rpath 规则、LTO / opt。

`manifest.json` 记录：
- `P`、`S`、`B`；
- `rustc -vV` 全文、target triple、JDK 版本；
- 模块表（模块名 → crate 名、产物文件、上游模块）；
- 文件表（相对路径、sha256、大小）；
- `user_rustc` 参数模板（§4.4）。

### 3.6 `rava compile`（单例）

1. 读 `build_status.json` 的 emit 段，得到 bin 名与 `P`；在本机缓存查 `P → B`，校验 `manifest.json`。
2. 按模板拼 rustc 参数直接执行，`--error-format=json`。首错提取、超时、进程组处理沿用 `cargo.rs` 的逻辑，改为解析 rustc 的诊断 JSON。
3. 写 `build_status.json`，新增 `archive: {key, mode, hit|pulled|built|fallback}`、`rustc_ms`、`peak_mb`；运行照旧。
4. 单例产物写在 scratch 下，不经共享 target，`prune` 不再涉及档案。

## 四、问题结论

### 4.1 Rust dylib 可行性

**版本 / 参数一致性**
- rustc 自身保证：
  - 不同编译器的元数据加载时报 E0514；
  - rmeta 与 dylib 不是同一次构建时 SVH 不符，报 E0460；
  - panic 策略不一致时报链接期错误（F2）。
- 所以档案必须原样携带同一次构建的 rmeta 与 dylib，按 sha256 校验。
- 键 `B` 必须包含：
  - `rustc -vV` 全文（std dylib 的文件名哈希随它变）；
  - target；
  - 模式与 panic；
  - opt-level / debuginfo / codegen-units / crate-type / LTO；
  - 模块拓扑；
  - 链接参数与 rpath 规则；
  - `--remap-path-prefix` 规则。
- 用户 crate 的 opt-level / debuginfo 不必与档案一致；panic 必须一致。

**`-C prefer-dynamic` 与 std**
- 语料模式下 std 以 dylib 共享，这是稳定版下唯一能构建通过的组合（F2 / F3），代价是 panic=unwind（用户已批准）。
- std dylib 复制进档案 `lib/`，运行期不依赖工具链路径。

**rpath**
- 可执行文件：`-Wl,-rpath,<本机档案>/lib`。
- 模块 dylib：macOS 用 `install_name @rpath/lib<m>.dylib` 加 `@loader_path`；Linux 用 `$ORIGIN`。

**macOS 与 Linux 的差异**
- macOS：
  - `@rpath` 依赖按加载链上全部镜像的 LC_RPATH 搜索；
  - SIP 剥离 `DYLD_*`，只能靠 rpath；
  - 两级命名空间让下游模块对上游符号的引用绑定到具体的上游 dylib（最小模型中 `liba/___rava_a_A__hello`）。
- Linux：
  - `DT_RUNPATH` 不传递，每个模块 `.so` 自带 `$ORIGIN`；
  - 可执行文件直接引用每个模块的 `__rava_register_module`，`--as-needed` 不会丢掉 `DT_NEEDED`；
  - java_base 内部声明层 → 实现层的引用在 rlib 次序中出现在定义之后，需要 lld。x86_64-unknown-linux-gnu 自 1.90 起缺省 rust-lld，同 crate-split §7.2。
  - 这些都**待服务器验证**（§5.2）。

### 4.2 泛型与内联

- 非泛型方法体在实现层，经 `export_name` 符号调用；泛型方法体留在声明层，由使用方单态化。
- 用户 crate 中定义在 java_runtime 的单态化项只有 13–55 个：`#[inline]` 外壳与 `Clone` / `From` / `_is_jnull::<Object>` 一类，不构成编译成本。
- 档案侧**不需要**预先实例化。opt 0 下 rustc 缺省开启 share-generics，复用上游已有的同参实例。
- 跨模块内联外壳会在下游模块中引用上游模块的 `__rava_*` 符号。方向是下游到上游，绑定到上游 dylib（最小模型已验证）。
- 生成器要避免的形态只有一种：**把同一档案下逐例相同的内容生成进用户 crate**。今天约 1,370 行档案类登记就是这种形态，移出后单例从 1.8–2.5 s 降到 0.58–0.96 s；终态归属见 §3.3。

### 4.3 vtable、static、类初始化状态的跨 dylib 正确性

- 每个档案 crate 只静态链接进一个模块 dylib：`<m>_decl` / `<m>_body_k` 进 `<m>`。可执行文件与下游模块都经 dylib 依赖使用上游，不含副本；rustc 的「同一 crate 只出现一次」检查在构建期保证这一点（最小模型中 libb 没有重复链接 a_decl）。
- 实测（s2 单 dylib 形态）：用户二进制里属于 java_runtime 的已定义数据符号为 **0**。
- 最小模型（多 dylib 形态）：上游模块的 static、`OnceLock`、thread_local 被下游模块与 bin 共享，且只有一份。
- vtable：Rust 不保证 vtable 地址唯一，运行时也没有依赖 vtable 地址的判同；runtime 中 `ptr::eq` 用法都先转成瘦指针，`Rc::ptr_eq` 自 1.72 起忽略元数据。`TypeId` 与所在镜像无关。最小模型中 `Rc<dyn ObjectVTable>` 跨模块分派正确。
- s1 / s2 共 5 个程序在 dylib 下输出与 `tests/expected` 一致。

### 4.4 直接调用 rustc，还是走 cargo

**用户 crate 参数模板**（`manifest.json.user_rustc`，语料模式）

```
rustc --crate-name <bin> --edition=2021 <scratch>/user/src/main.rs --crate-type bin --emit=link
  -C panic=unwind -C prefer-dynamic -C opt-level=0 -C debuginfo=line-tables-only
  -C embed-bitcode=no -C metadata=<用户 crate 摘要> --cap-lints allow
  --error-format=json --json=diagnostic-rendered-ansi
  -L dependency=<archive>/lib
  --extern <m>=<archive>/lib/lib<m>.<dylib|so>          （每个档案模块一项）
  --extern rava_macros=<archive>/lib/librava_macros-*.<dylib|so>
  -C link-arg=-Wl,-rpath,<archive>/lib
  --out-dir <scratch>/bin
```

- 生产模式：`--extern` 改指 rlib，去掉 `prefer-dynamic` 与 rpath，`-C panic=abort`。
- 取值对 p50 的影响：`debuginfo=0` −0.02–0.03 s，`codegen-units=1` +0.02–0.06 s，都在噪声量级；维持 line-tables-only，codegen-units 取缺省，不开增量。
- 不走 cargo：一是 cross-test §4.4 的指纹问题（绑定路径，大 rlib 的检查约 7 s）；二是实测剩余成本全部在 rustc 内部（0.58–0.96 s）。

### 4.5 生产模式静态链接（用户已定）

- 模块 crate 为 rlib，用户 crate 直接调用 rustc 静态链接；`-dead_strip` / `--gc-sections` 由 rustc 缺省开启。
- `rava build`（开发）不开 LTO：465 类档案首构 77 s，只改用户代码 0.58 s。
- `rava build --release` 用 fat LTO、cgu 1、opt 3：首构 102 s，只改用户代码 47 s，体积 −20%。
- 两者 `B` 不同，各自缓存。

### 4.6 失败回退

逐级判定，每级记入 `build_status.json.archive`：
1. **本机命中**：manifest、sha256 与本机 `rustc -vV` 都校验通过。
2. **拉取**（语料模式）：cross-test §4.5，带分块续传与拉取预算。
3. **本机构建同一个 `S`**：
   - 触发条件：未命中且拉取预算用尽；工具链不一致（记为漂移、上报）；校验失败；
   - 链接时报 E0460 / E0514 / 未定义符号，视为档案损坏：作废后重建一次，第二次仍失败按真实编译错误报告；
   - 构建期间持 flock，同机其他进程等锁后复用。
4. **档案不覆盖本程序**：语料作业由调度方并入档案（`P` 变）；单独运行时退化为生产模式，以本程序闭包为档案、静态链接。

**不做**：不回落到 cargo 编译整个 scratch；不允许「部分链接档案、部分本地编译 JDK 类」的混合形态，它会破坏 §4.3 的唯一性。

### 4.7 按模块切分的反向边清单与消除手段

| # | 来源 | 实测规模（s2） | 消除手段 | 结果 |
|---|---|---|---|---|
| R1 | 生成器导入：`generator/crates/emit/src/imports/referenced.rs` 的 `dispatch_subtype_refs`，按方法调用 owner 收集全部 JDK 子类型并预认领为导入（如 Throwable → `sun.security.pkcs11.ConfigurationException`）；正文不使用 | 反向 6,419 条，互不可达 665 条（声明层 + 实现层，全部在 `use` 行） | 子类型集合按模块可读性过滤：只收本模块与上游模块的子类型。正文实测 0 条跨模块的子类型使用，过滤不改变生成正文 | 0 |
| R2 | 档案元数据：声明层 `meta` 以 `extern` 读 `java_meta` 的 `__java_meta_*`（22 张表，含全部模块的行） | 22 个未定义符号 | 表按模块拆到各模块 crate，经 `__rava_register_module()` 登记；查询面合并已登记的切片（与 `register_user` 同一机制） | 0 |
| R3 | 档案侧登记表（反射分派、类初始化钩子、VM 引导；今天在用户 `main.rs`，java_runtime 里没有反向链接边，但阻止「全局表放 java_base」） | 约 1,370 行 / 例（java.base 1,290，其余 70） | 每个模块的行进该模块 crate；用户 `main.rs` 按拓扑序调用 | 0 |
| R4 | 声明层 ↔ 实现层 `export_name` 环 | 31,026 个 `__rava_*`：java.base 28,381，其余 2,645 | 非 java.base 模块单 crate，环消失；java.base 拆层，环闭合在 `java_base` 产物内，不跨模块 | 跨模块 0 |
| R5 | 宏属性、手写代码、正文路径 | 0 | — | 0 |

- 未发现需要把下游模块的类上提到 java_base 的情形。
- ServiceLoader、JCA provider、`SharedSecrets` 这类「上游使用下游实现」的场景，今天已经走运行期表（反射分派、模块服务），没有链接期边，按 R2 / R3 归属后依旧没有。

### 4.8 与 crate-split（§7 `java_body_*`）的关系与协调接口

**结论**
- 切分轴改为两级：
  1. 先按 JDK 模块切；
  2. 模块内只在超出体量阈值时，切成 `<m>_decl` + `<m>_body_1..N`。
- s2 中只有 java.base（75.9 MB 源码）需要第二级；第二大的 java.xml.crypto 只有 1.92 MB。
- 内存瓶颈（java.base 声明层）不因按模块切分而消失。crate-split 的声明层收窄、实现层切分策略仍然适用，作用对象从「整个 java_runtime」收窄为 java.base。

**需要与 crate-split 代理协调的接口**
1. **分区函数的输入**：实现层分区的输入由「全部档案类」改为「某模块的类」；输出 `<m>_body_<k>` 的类集合。分区规则（体量均衡 / 按包）由 crate-split 定。
2. **拆层判据**：模块是否拆层用 crate-split 的同一体量口径与阈值，实测 s2 只有 java.base 越过。不拆层的模块不生成 `rava_layer`，没有 `export_name` 外壳。
3. **crate 名与路径前缀**：
   - 生成文本中的跨类路径前缀：同模块内用 `crate::`；跨模块用 `<上游模块 crate>::`；java.base 实现层引用声明层用 `java_base_decl::`；
   - 对应今天发射层的 `site.prefix()` / `import_site()`，需要由类所属模块决定。
4. **Cargo 依赖**：模块 crate 依赖 = 实际引用的模块（⊆ requires 闭包，越界即失败）；`<m>_body_k` 依赖 `<m>_decl` 与上游模块 crate。
5. **导入过滤**：R1 的 `dispatch_subtype_refs` 可读性过滤。
6. **元数据与登记表**：`java_meta` 拆成各模块行，R2 / R3。这与 crate-split 的「元数据 crate」设计重叠，以本文为终态：不再有独立的全档案元数据 crate。

## 五、实施分步（每步可独立验收）

### 5.1 步骤

| 步 | 内容 | 验收 |
|---|---|---|
| M1 | 类 → 模块映射与模块依赖（从 JDK jimage / jmod 模块描述读取）；导入的子类型集合按模块可读性过滤（R1）；引用越界的断言 | `t1link_module_edges.py` 对 s2 报告：正文与 `use` 行的反向 / 互不可达边都为 0；生成正文与今天逐字节相同（只少了 `use` 行）；`no_jdk_literals` 通过 |
| M2 | 按模块切 crate：模块 crate 命名、依赖、拆层判据；java.base 拆为 `java_base_decl` + `java_base_body_k`，其余模块单 crate（与 crate-split 合并实施，§4.8） | s2 生成 15 个模块 crate，cargo 构建通过；`<m>` 以外的模块 crate 没有未定义 `__rava_*` 符号；单例模式与档案模式输出不变（服务器抽查） |
| M3 | 元数据表、反射分派、类初始化钩子、模块资源、行表按模块归属，`__rava_register_module()`；`meta` 查询面改为合并登记切片；用户 `main.rs` 按拓扑序登记（R2 / R3） | 用户 `main.rs` 中档案类登记行为 0，档案相关行数 = 模块数；`__java_meta_` 未定义符号为 0；档案 crate 两程序逐字节相同（`archive_emit_cli` 扩展）；输出不变 |
| L2 | `rava archive build <profile.json> [--mode corpus\|production]`：cargo 编 rlib，按拓扑序直接调用 rustc 链接模块 dylib，复制 std dylib，落盘 `lib/` 与 `manifest.json`，flock | 同一 `S` 两次构建 `B` 相同；manifest 校验通过；Linux（服务器）上每个模块 `.so` 的 `RUNPATH=$ORIGIN`，`NEEDED` 与 requires 一致，可执行文件 `NEEDED` 含全部模块；语料包 zstd 后 ≤ 0.25 GB |
| L3 | `rava compile` 直接调用 rustc：读 manifest，按模板拼参数，解析诊断 JSON，写 `build_status`；生产模式静态链接 | 1061 例结果与第 1 步一致；单例 p50 ≤ 1 s、p99 ≤ 3 s、峰值 ≤ 0.8 GB；语料模式单例二进制 ≤ 2 MB |
| L4 | 回退链 §4.6；本机 GC 与引用登记同 cross-test §4.6 | 人为删档案文件、改 `rustc -vV` 记录、给未覆盖程序，各自走到对应级别且结果正确 |

- M1 只动导入与映射，可以先于 crate-split 合入。
- M2 与 crate-split 代理合并实施：分区函数由它提供，模块轴由本方案提供。
- M3 依赖 M2。L2 / L3 只动 driver。

### 5.2 待服务器验证（本机无法覆盖）

`remote_rava` 只能运行 rava 子命令，Linux 差异放进 L2 / L3 的服务器验收：
1. 模块 `.so` 链接：rust-lld 缺省，GNU ld 的次序问题不出现；`java_base` 的链接峰值（按比例估算约 3.3 GB，在构建者 7 GB 预算内）。
2. `readelf -d`：每个模块 `RUNPATH=$ORIGIN`；`NEEDED` 为上游模块与 `libstd-<hash>.so`；可执行文件 `NEEDED` 含全部档案模块。
3. 全集档案（约 3609 类）上 1061 例的 p50 / p99 / 峰值；dyld / ld.so 加载约 20 个模块 `.so` 的启动开销。
4. panic：stub panic 发生在子线程与 Continuation 载体栈上时，unwind 下仍以退出码 101 结束，stderr 首行与今天相同。

## 六、量化目标（终态）

| 项 | 目标 | 依据 |
|---|---:|---|
| 单例用户 crate 编译 + 链接，p50 | **≤ 1 s** | s2 实测 0.58–0.96 s；模块 dylib 个数不改变导出符号总量 |
| 单例 p99 | **≤ 3 s** | 用户类多的程序由类型检查 / codegen 线性增长决定；服务器全集验收 |
| 单例峰值 RSS | **≤ 0.8 GB** | 实测 455–507 MB |
| 跨模块反向 / 互不可达引用（正文 + 导入 + 宏属性 + 链接符号） | **0** | 实测正文、宏属性、手写为 0；导入 7,084 条与 22 张表由 R1 / R2 消除 |
| 用户 `main.rs` 档案类登记行 | **0**（只剩每模块一行登记） | 今天约 1,370 行 / 例 |
| 非 java.base 模块的未定义 `__rava_*` 符号 | **0** | 今天 2,645 个 |
| 语料模式单例二进制 | ≤ 2 MB | 实测 0.15–0.70 MB |
| 语料包 | ≤ 0.25 GB / 键（zstd） | s2 实测 202 MB；全集按类数外推约 0.24 GB |
| `java_base` dylib 链接 | ≤ 15 s、≤ 4.5 GB，每键一次 | s2 单 dylib 实测 8.75 s / 3.48 GB，java_base 占 95.6% 的符号 |
| 生产模式只改用户代码（不开 LTO） | ≤ 2 s | 实测 0.58 s |

cross-test §5.2 的「p50 ≤ 2 s、p99 ≤ 5 s、峰值 ≤ 1 GB」按上表收紧。

## 七、用户决策（2026-10-04 已答复）与风险

**决策记录**
1. **语料模式 `panic = "unwind"`：批准。** 推翻 X1a 中语料模式的 abort 部分，生产模式不变。
2. **生产 LTO：开发构建不开 LTO，`--release` 用 fat LTO。**
3. **crate 命名：按 JDK 的 jmod 模块名命名**（`java.base` → `java_base`，`java.net.http` → `java_net_http`，`jdk.crypto.ec` → `jdk_crypto_ec`），从 JDK 的 jmod / 模块描述动态取得，生成器里不写字面量。不用 `java_profile` / `java_archive`。据此重论证后得出 §三 的按模块切分。

**仍需确认的点**
- 被拆层模块的内部 crate 命名 `<m>_decl` / `<m>_body_<k>`：由模块名加层后缀派生，下游与用户代码不可见。若希望内部 crate 也不出现后缀，需改为单 crate（与 java.base 的内存目标冲突），本文不建议。

**风险**
- **java.base 体量**：92% 的源码和 96% 的导出符号在 java.base，按模块切分对内存与单例链接的帮助有限（档案构建峰值按比例约 5.8 GB）。内存目标依赖 crate-split 在 java.base 内的切分。
- **导出符号规模**：`java_base` 约 62 万导出符号，单例链接固定约 0.4 s。若 Linux 上更慢，可在模块 dylib 链接时只导出下游可见的符号（version script / `-exported_symbols_list`，由 rmeta 可见项生成），作为 L3 的可选优化，以实测为准。
- **第三方 lib crate**：jar 有 module-info 时按其模块名；自动模块按 `Automatic-Module-Name` 或 JPMS 的 jar 名推导规则命名。类路径 jar 之间可能存在循环引用；遇到时合并该强连通分量为一个 crate，名字取分量内按名序最小的模块。lib pilot 语料验收时确认。
- **模块 dylib 个数**：s2 为 15 个，全集预计约 20 个；动态加载开销在 L3 服务器验收中实测。
- **macOS `split-debuginfo=unpacked`**：拉取方的 Rust 回溯缺档案帧的文件 / 行号；Java 栈帧走自己的行表，不受影响。服务器为 Linux，不受影响。

## 八、交接

- 实验目录：`build/t1link/`（s1 / s2 档案树、`target-*`、`out/`、`mod/`、`modtoy*/`），可随时删除。
- 下一步：M1（可先行）→ M2（与 crate-split 合并）→ M3 → L2 → L3 → L4。

# T1 第 2 步：驱动直接调用 rustc 链接档案（方案 + 最小实测）

> 状态（2026-10-04）：方案与本机最小实测完成，交用户决策（§七）。未改生成器 / 驱动 / runtime 主线逻辑。
> 上游：[`2026-10-01-cross-test-compile-reuse.md`](2026-10-01-cross-test-compile-reuse.md) §4.4、§5.3 第 2 步、§6（1a / 1b）；
> crate 分层：[`2026-10-01-rustc-memory-and-crate-split.md`](2026-10-01-rustc-memory-and-crate-split.md) §7。
> 本文只写终态，不写过渡方案。

## 一、结论摘要

1. **档案以单个门面 dylib 交付**：新增档案 crate `java_profile`，静态吸收 `java_runtime`、`java_body_*`、`java_meta`，在 dylib 内闭合声明层与实现层 / 元数据层之间的 `export_name` 符号环。逐 crate 各编一个 dylib 不可行：s2 档案的 `java_runtime` rlib 有 31,048 个未定义的 `__rava_*` / `__java_meta_*` 符号，定义都在下游 crate 里。
2. **语料模式必须用 `panic = "unwind"`**：Rust dylib 与 `-C prefer-dynamic` 只能链接工具链自带的 std dylib，而它内置 `panic_unwind`。panic=abort 下的两条路线（门面链 std dylib、门面静态吸收 std）都被 rustc 拒绝，见 §5.2。`create_java_vm` 的钩子在 unwind 开始前就 `exit(101)`，可观测行为与今天相同。档案构建开销实测差别在噪声内。**需用户决策**，因为这推翻了 X1a 中语料模式部分的结论。
3. **单例数字（3077 类档案 s2，本机 arm64 macOS）**：
   - 用户 crate 直接调用 rustc 链接 dylib：1.8–2.0 s，峰值 625–686 MB；
   - 再把档案侧登记表移出用户 `main.rs`：**0.58–0.96 s，峰值 455–507 MB**，二进制 0.18–0.70 MB，输出与 `tests/expected` 一致；
   - 对照静态链接 rlib：1.6–4.3 s，峰值 **2.3–2.6 GB**，二进制 346–440 MB。超出 1 GB 的目标，所以语料模式只能走 dylib。
4. **用户 crate 的主要成本是档案侧登记表，不是 JDK 泛型**：
   - 今天的 `main.rs` 有 1391–1408 行，除约 24–38 行用户相关内容外，都是档案类的类初始化钩子、反射分派闭包与引导初始化，同一档案下逐例相同。把它们移进档案后，单例编译时间降到约三分之一；
   - 用户 crate 中在 java_runtime 定义的单态化项只有 13–55 个，都是 `#[inline]` 外壳和 `Clone` / `From` 一类；
   - 档案侧不需要预先实例化任何东西。
5. **跨 dylib 的唯一性**：档案的 static / thread_local / `OnceLock` 类初始化状态只存在于门面 dylib 中。用户二进制里属于 java_runtime 的数据符号实测为 0。vtable 可能有多份，但运行时不以 vtable 地址判同一性（§4.3）。
6. **参数复现**：用户 crate 的 rustc 参数全部来自档案清单 `manifest.json`。opt-level、codegen-units、debuginfo 对 p50 的影响都 ≤ 0.04 s，链接约 0.4 s 是剩余的大头（§4.4）。
7. **生产模式**：静态链接 rlib。465 类档案上：
   - 不开 LTO：首构 77 s，只改用户 crate 后重建 0.58 s；
   - fat LTO：首构 102 s，只改用户 crate 后重建 47 s，峰值 3.3 GB，二进制小 20%；
   - 开发迭代用不开 LTO，`--release` 用 fat LTO（**决策点**）。

## 二、现状（实验记录）

**实验环境**
- 工作树 `java_rta_t1link`，分支 t1-link，从 10cfb657 起，即 1b 的提交。rava 二进制用的是 `java_rta_t1b` 下同一提交的构建。
- rustc 1.98.1，aarch64-apple-darwin，ld `ld-27037.1`，16 GB 内存。
- 所有编译都经 `heavy_lock`，`CARGO_BUILD_JOBS=2`。
- 实验目录 `build/t1link/`（gitignore）。

**实验脚本（`scripts/diag/`）**
- `t1link_emit_set.sh`：建档案，再逐个程序 `--profile` 发射；
- `t1link_facade.sh`：把档案 rlib 打成门面 dylib；
- `t1link_user_rustc.sh`：直接调用 rustc 编译用户 crate，`rlib` / `dylib` 两种链接方式，记录时间、峰值、运行输出；
- `t1link_release.sh`：生产模式首构与「只改用户 crate」重建。

### 2.1 档案

| 档案 | 内容 | 程序 | 档案 crate 是否逐字节相同 |
|---|---|---|---|
| s1 | 465 类、1809 方法（1b 的 s1） | HelloWorld、ControlFlowTest | 是 |
| s2 | 3077 类、17754 方法，6 个入口 | HelloWorld、TestBridgeMethod、LambdaBasic（另有 AnonClassTest、ReflectionBasic、ObjectMethods 只入档案，不发射） | 是（`java_runtime` / `java_meta` 逐目录对照） |

- s2 的 `rava profile` 耗时 18.8 s，峰值 1453 MB。
- s2 规模：`java_runtime` 源码 42 MB，`java_body_1..10` 每个约 5.2 MB。

### 2.2 实测数字

**档案构建（每个键一次）**

| 项 | s1 | s2 |
|---|---:|---:|
| cargo 构建档案 + HelloWorld（dev，abort） | 34.5 s / 2.08 GB | — |
| 同上（dev，unwind） | 34.6 s / 2.19 GB | 248 s / **6.33 GB**（`java_runtime` 决定峰值） |
| 档案 rlib 体积 abort → unwind | `java_runtime` 112 → 125 MB，body +17% | 合计 1.67 GB |
| 门面 dylib 链接（unwind + prefer-dynamic） | 1.07 s / 568 MB / 76 MB | **8.75 s / 3.48 GB / 510 MB**，导出符号 650,965 个 |
| 语料包：各 crate rmeta + 门面 dylib | — | 442 + 510 MB，`zstd -3` 后 **202 MB** |

**单例：用户 crate 直接调用 rustc，每格 2–3 次取范围**

| 程序（档案） | 链接方式 | `main.rs` | 墙钟 | 峰值 RSS | 二进制 | 输出 |
|---|---|---|---:|---:|---:|---|
| HelloWorld（s1） | 静态 rlib | 今天形态 | 0.88 s | 338 MB | 42 MB | 一致 |
| HelloWorld（s1） | dylib | 今天形态 | 0.25–0.31 s | 195 MB | 0.37 MB | 一致 |
| ControlFlowTest（s1，链接 HelloWorld 档案目录的产物） | dylib | 今天形态 | 0.24 s | 192 MB | 0.35 MB | 一致 |
| HelloWorld（s2） | 静态 rlib | 今天形态 | 3.1–4.3 s | 2.55 GB | 440 MB | 一致 |
| HelloWorld（s2） | 静态 rlib | 档案行移出 | 1.64–1.68 s | 2.29 GB | 346 MB | 一致 |
| HelloWorld（s2） | dylib | 今天形态 | 1.82–2.54 s | 625 MB | 2.9 MB | 一致 |
| TestBridgeMethod（s2） | dylib | 今天形态 | 2.02–2.13 s | 658–686 MB | 3.4 MB | 一致 |
| LambdaBasic（s2） | dylib | 今天形态 | 1.80–1.82 s | 624 MB | 2.9 MB | 一致 |
| HelloWorld（s2） | dylib | 档案行移出 | **0.55–0.59 s** | **455 MB** | 0.18 MB | 一致 |
| TestBridgeMethod（s2） | dylib | 档案行移出 | **0.77–0.96 s** | **458–507 MB** | 0.70 MB | 一致 |
| LambdaBasic（s2） | dylib | 档案行移出 | 1.28 s 墙钟（单次，user 0.72 s，含冷缓存读 rmeta） | 464 MB | 0.15 MB | 一致 |

- 「档案行移出」：从 `main.rs` 删去档案类的 `register_class_init_hooks` / `register_method_dispatch` / `register_field_dispatch` / `vm_boot_init` 行，模拟 §3.3 L1 的终态。删去后 `main.rs` 只剩 24 行。这是实验近似：这三个用例运行时不经由被删除的 JDK 类反射分派。
- ControlFlowTest 一行：用另一个程序的档案目录产物链接，结果正确。这直接验证了「同一档案、不同用户程序共用一份产物」。

**单例 rustc 耗时构成**（s2 HelloWorld，`-Ztime-passes`，借 `RUSTC_BOOTSTRAP` 只作诊断）

| 阶段（今天的 `main.rs`） | 耗时 |
|---|---:|
| 总计 | 1.80 s |
| 类型检查 | 0.50 s |
| 借用检查 | 0.27 s |
| 单态化收集 | 0.23 s |
| codegen | 0.41 s |
| 链接 | 0.41 s |

- 档案行移出后墙钟 0.58 s，其中链接仍约 0.4 s（链接与 `main.rs` 内容无关），是剩余的大头。

**参数扫描**（s2，档案行移出，dylib）

| 参数 | HelloWorld | TestBridgeMethod |
|---|---:|---:|
| 基线（opt 0，cgu 缺省，line-tables-only） | 0.59 s | 0.80 s |
| `-C debuginfo=0` | 0.57 s | 0.77 s |
| `-C codegen-units=1` | 0.61 s | 0.86 s |

- ld64.lld（`-C link-arg=--ld-path=<sysroot>/.../gcc-ld/ld64.lld`）：链接失败，`rust-lld: could not load TAPI file ... MacOSX27.0.sdk/usr/lib/libiconv.tbd: malformed file`（rust-lld 读不了本机新版 SDK 的 tbd）。macOS 上不可用，未计入；Linux 缺省即 rust-lld，不受影响。

**运行期**
- dylib 二进制启动 + HelloWorld 运行 0.02 s，与静态二进制相同。档案 dylib 的加载开销可忽略。

**单态化落点**（`-Zprint-mono-items`，档案行移出）

| 程序 | 单态化项 | 其中 std | 其中定义在 java_runtime |
|---|---:|---:|---:|
| LambdaBasic | 182 | 66 | 13 |
| TestBridgeMethod | 1264 | 296 | 55 |

- TestBridgeMethod 的其余项是用户泛型类，例如 `Store<String>`，以及用户类对 JDK trait 的实现。这是用户代码固有的，不随档案变化。

**生产模式**（s1 HelloWorld，cargo release：opt 3，cgu 1）

| LTO | 首构 | 只改用户 crate | 峰值 | 二进制 |
|---|---:|---:|---:|---:|
| fat（今天的 release 配置） | 102.5 s / 3.27 GB | **47.1 s** / 3.29 GB | 3.29 GB | 16.5 MB |
| off | 77.3 s / 2.95 GB | **0.58 s** / 245 MB | 2.95 GB | 20.8 MB |

### 2.3 失败路线与原因

**F1 逐 crate 各编 dylib**
- 声明层通过 `extern "Rust"` 引用实现层 `#[export_name]` 定义的 `__rava_*` 符号，并引用 `java_meta` 的 `__java_meta_*`；被引用方又依赖声明层，构成符号环（s2 有 31,048 个未定义符号）。
- macOS 链接 dylib 时不允许未定义符号，除非加 `-undefined dynamic_lookup`。那样会把链接期的签名不匹配推迟到运行期，违背 crate-split §7.3.2「签名不一致即链接失败」的设计。
- Linux 允许未定义符号，但可执行文件不直接引用 `java_body_k`，`--as-needed` 会丢掉它的 `DT_NEEDED`，运行期解析失败。
- 排除。

**F2 panic=abort + 门面 dylib 链接 std dylib（`-C prefer-dynamic`）**
- rustc 报错：`the linked panic runtime panic_unwind is not compiled with this crate's panic strategy abort`。
- sysroot 里的 `libstd-*.dylib` 内置 `panic_unwind`。稳定版无法重编 std（`-Zbuild-std` 只在 nightly 可用）。

**F3 panic=abort + 门面静态吸收 std，可执行文件不用 prefer-dynamic**
- 门面 dylib 本身能构建（1.63 s，59 MB，无未定义符号）。
- 用户 crate 链接时报错：`cannot satisfy dependencies so std only shows up once`。sysroot 同时提供 std 的 rlib 与 dylib，rustc 的依赖格式计算把找得到 dylib 的 crate 一律记为动态，与门面「已静态包含 std」冲突。
- 绕开它要换一份不含 std dylib 的 sysroot，属于改动工具链布局，排除。

**F4 语料模式静态链接 rlib**
- 链接峰值 2.3–2.6 GB，超出单例 ≤ 1 GB 的目标；二进制 346–440 MB。
- 1061 例逐例链接时，链接墙钟与磁盘写入都按档案规模增长。只保留给生产模式。

## 三、终态设计

### 3.1 crate 布局（与 crate-split §7.2 一致，新增门面）

```
java_runtime（声明层）  java_body_1..N（实现层）  java_meta（档案侧元数据表）
        └──────────────┬──────────────┘
                java_profile（档案门面，生成）
                  · 语料模式 crate-type = dylib；生产模式 rlib
                  · `pub extern crate` 全部档案 crate，链接期闭合 export_name 环
                  · 档案侧登记表：JDK 类的类初始化钩子、反射方法 / 字段分派、VM 引导初始化序列
                    （今天在用户 main.rs 里，见 §3.3）
user（bin）
  · `use java_profile as _;`（取代今天的 `use java_meta as _;` / `use java_body_k as _;`）
  · 只登记用户行：`register_user(&USER_META)` 与用户类的反射分派
```

- `java_profile` 的源码由档案事实决定，同一 `P` 下逐字节相同。它纳入档案 crate 的包版本摘要（1b `stamp_versions`）与 `archive_emit_cli` 守护测试。
- 档案侧登记表的形态：
  - 写成静态表 `&[(&str, fn(...) -> ...)]`，以 `#[export_name = "__java_profile_<表名>"]` 导出；
  - `java_runtime` 用 `extern` 声明读取，与 `__java_meta_*` 同一机制，`create_java_vm` 首次查询时合并；
  - 静态表代替今天逐行 `__Shared::new(闭包)` 的堆分配，用户 crate 不再为 800 个 JDK 类做类型检查和单态化。
- 生成器不出现 JDK 类名字面量：表行由档案事实遍历生成，与今天写进 `main.rs` 的来源相同，只是换了落点。

### 3.2 两种模式

| | 语料模式（`rava build --profile`） | 生产模式（单项目，档案即项目闭包） |
|---|---|---|
| 档案产物 | 各档案 crate 的 rmeta + `libjava_profile.{dylib,so}` + std dylib 副本 + `rava_macros` proc-macro | 各档案 crate 的 rlib（含 rmeta）+ `rava_macros` |
| panic | `unwind`（档案与用户 crate 一致；钩子 `exit(101)`） | `abort` |
| 用户 crate 链接 | `-C prefer-dynamic`，rpath 指向档案 `lib/` | 静态；开发迭代不开 LTO，`--release` 用 fat LTO |
| 单例产物 | 只含用户 crate 的小二进制（实测 0.15–0.7 MB） | 完整原生二进制 |

两种模式下，用户 crate 源码相同，只是 rustc 参数不同。

### 3.3 档案构建（每个 `B` 一次，构建者见 cross-test §4.3）

1. 生成档案树：`java_runtime`、`java_body_*`、`java_meta`、`java_profile`、`closure_input/`。只依赖 `profile.json`，不需要任何用户程序。
2. cargo 构建 rlib crate（`--message-format=json`，profile 的 panic 取模式值，`--remap-path-prefix=<树>=/rava/profile`）。从 artifact 消息得到各 crate 的 rlib / rmeta 路径。
3. 语料模式：rava 直接调用 rustc 链接门面 dylib。不经 cargo，因为 `-C prefer-dynamic` 若写进 RUSTFLAGS 会波及 proc-macro 与构建脚本：
   - `--crate-type dylib -C prefer-dynamic -C panic=unwind`，`--extern` 各档案 rlib；
   - macOS 加 `-Wl,-install_name,@rpath/libjava_profile.dylib` 与 `-Wl,-rpath,@loader_path`；
   - Linux 加 `-Wl,-rpath,$ORIGIN`。原因：`DT_RUNPATH` 不作用于传递依赖，门面依赖的 std `.so` 必须由门面自己的 RUNPATH 找到。
4. 落盘 `~/.cache/rava/archive/jdk<M>/<B>/`：
   - `lib/`：档案 crate 的 rmeta（生产模式为 rlib）、传递依赖（parking_lot / libc / rava_coro 等）的 rmeta、门面 dylib、std dylib 副本（`rustc --print target-libdir` 下的 `libstd-<hash>.*`）、`rava_macros` proc-macro；
   - `manifest.json`：见 §3.4。最后写入，写入后改名生效；flock 互斥（cross-test §4.3）。

### 3.4 `manifest.json` 与键（补充 cross-test §4.1 的 L2 键 `B`）

`B` 的输入在 §4.1 之外再加：
- 模式（corpus / production）；
- panic 策略；
- 门面 crate-type 与链接参数；
- rpath 规则。

`manifest.json` 记录：
- `P`、`S`、`B`；
- `rustc -vV` 全文与 target triple；
- 文件表（相对路径、sha256、大小）；
- `crates`：crate 名 → 文件（rmeta / rlib / dylib）；
- `user_rustc`：用户 crate 的参数模板，见 §4.4。

### 3.5 `rava compile`（单例）

1. 读 `build_status.json` 的 emit 段，得到 bin 名与档案键 `P`；在本机缓存查 `P → B`，再校验 `manifest.json`。
2. 按模板拼 rustc 参数并直接执行，保留 `--error-format=json`。今天 `cargo.rs` 的首错提取与超时 / 进程组处理沿用，改为解析 rustc 的诊断 JSON。
3. 写 `build_status.json`，新增 `archive: {key, mode, hit|pulled|built|fallback}` 和 `rustc_ms` / `peak_mb`；运行照旧。
4. 单例产物直接写在 scratch 下，不经共享 target，所以 `prune` 不再涉及档案。

## 四、六个问题的结论

### 4.1 Rust dylib 可行性

**版本 / 参数一致性**
- rustc 自身保证：
  - 不同编译器编出的 crate 元数据，加载时报 E0514；
  - 门面 dylib 内含的 crate 与 `--extern` 给出的 rmeta 不是同一次构建时，SVH 不符，报 E0460；
  - panic 策略不一致时报链接期错误（F2 即此类）。
  所以档案必须原样携带「同一次构建」的 rmeta 与 dylib，产物清单按文件 sha256 校验。
- 键 `B` 必须包含：
  - `rustc -vV` 全文（std dylib 的文件名哈希随它变）；
  - target；
  - 模式与 panic；
  - opt-level / debuginfo / codegen-units / crate-type；
  - 门面链接参数与 rpath 规则；
  - `--remap-path-prefix` 规则。
- 用户 crate 的 opt-level / debuginfo 不必与档案一致；panic 必须一致。

**`-C prefer-dynamic` 与 std**
- 语料模式下 std 以 dylib 形式共享，F2 / F3 已证明这是稳定版下唯一能构建通过的组合，代价是 panic=unwind。
- std dylib 复制进档案 `lib/`，运行期不依赖工具链路径。各服务器的 `~/.rustup` 路径不同，这一点必要。

**rpath**
- 可执行文件：`-Wl,-rpath,<本机档案>/lib`（绝对路径，服务器侧固定在家目录下的缓存，cross-test §4.2）。
- 门面：macOS 用 `install_name @rpath/...` 加 `@loader_path`；Linux 用 `$ORIGIN`。

**macOS 与 Linux 的差异**
- macOS：
  - `@rpath` 依赖按加载链上全部镜像的 LC_RPATH 搜索（实测可执行文件的 rpath 能为门面找到 std）；
  - SIP 会剥离 `DYLD_*` 环境变量，所以只能靠 rpath；
  - ld64 对 650k 导出符号的 dylib 链接约 0.4 s。
- Linux：
  - `DT_RUNPATH` 不传递，门面必须自带 `$ORIGIN`；
  - 可执行文件直接引用门面里的 java_runtime 符号，`--as-needed` 不会丢掉门面的 `DT_NEEDED`；
  - 门面链接时 rlib 次序是依赖方在前，声明层对实现层的引用出现在定义之后，必须用 lld。x86_64-unknown-linux-gnu 自 1.90 起缺省 rust-lld，满足要求，同 crate-split §7.2；
  - **待服务器验证**，见 §5.2。

### 4.2 泛型与内联

- 拆层后，非泛型方法体在实现层，经 `export_name` 符号调用；泛型方法体留在声明层，由使用方单态化。
- 用户 crate 中定义在 java_runtime 的单态化项实测只有 13–55 个：
  - `#[inline]` 外壳（例如 `PrintStream::println_i` 转发到实现层符号）；
  - `Clone` / `From` / `_is_jnull::<Object>` 一类。
  它们体量很小，不构成编译成本。
- 用户泛型类对 JDK 类型的实例化（例如 `Store<String>`）是用户代码固有的，任何形态下都要编译。
- 档案侧**不需要**预先实例化。debug（opt 0）下 rustc 缺省 share-generics，上游已有的同参实例会被复用。
- 生成器侧要避免的形态只有一种：**把同一档案下逐例相同的内容生成进用户 crate**。今天 `main.rs` 中约 1370 行档案类登记就是这种形态，移出后单例 rustc 从 1.8–2.5 s 降到 0.58–0.96 s。§3.1 把它移进门面。
- 闭包登记改为静态 fn 指针表后，还消除了每行 `__Shared::new` 的单态化与堆分配。

### 4.3 vtable、static、类初始化状态的跨 dylib 正确性

- 档案的全部 crate 静态链接进同一个门面 dylib，可执行文件只链接门面，不含任何档案 crate 的副本。rustc 的「同一 crate 只出现一次」检查（F3 的报错正是它）在构建期保证这一点。
- 实测（s2 TestBridgeMethod 二进制）：定义的数据符号（d / s / b）中属于 java_runtime 的为 **0**。所以 static、`thread_local!` 键、`OnceLock` 形式的类初始化状态、GIL、`register_user` 合并表都只有门面内的一份。
- vtable：
  - Rust 不保证 vtable 地址唯一，同一 crate 内也不保证，所以跨 dylib 多一份不改变语义；
  - 运行时没有依赖 vtable 地址的判同：runtime 中的 `ptr::eq` 用法都先转成瘦指针或作用于 sized 类型，`Rc::ptr_eq` / `Arc::ptr_eq` 自 Rust 1.72 起忽略元数据；
  - `TypeId` 是类型哈希，与所在镜像无关，`downcast` 正确。
- 实测 s1 / s2 共 5 个程序在 dylib 下输出与 `tests/expected` 一致，覆盖了 GIL、类初始化、`println`、lambda、桥方法与协变返回。

### 4.4 直接调用 rustc，还是走 cargo

**用户 crate 参数模板**（`manifest.json.user_rustc`，语料模式）

```
rustc --crate-name <bin> --edition=2021 <scratch>/user/src/main.rs --crate-type bin --emit=link
  -C panic=unwind -C prefer-dynamic -C opt-level=0 -C debuginfo=line-tables-only
  -C embed-bitcode=no -C metadata=<用户 crate 摘要> --cap-lints allow
  --error-format=json --json=diagnostic-rendered-ansi
  -L dependency=<archive>/lib
  --extern java_runtime=<archive>/lib/libjava_runtime-*.rmeta   （java_meta / java_body_k 同理）
  --extern java_profile=<archive>/lib/libjava_profile.<dylib|so>
  --extern rava_macros=<archive>/lib/librava_macros-*.<dylib|so>
  -C link-arg=-Wl,-rpath,<archive>/lib
  --out-dir <scratch>/bin
```

- edition、cfg（今天无）、features（今天无）都是常量。lints 用 `--cap-lints allow` 代替今天逐条 allow。
- `-C metadata` 防止用户 crate 名与档案 crate 名偶合时符号冲突。
- 生产模式：`--extern` 改指 rlib，去掉 `prefer-dynamic` / rpath，`-C panic=abort`。

**取值对 p50 的影响**
- opt-level 0：`-C debuginfo=0` −0.02–0.03 s，`-C codegen-units=1` +0.02–0.06 s，都在噪声量级。
- 维持 line-tables-only，保证 Rust 回溯可用；codegen-units 取缺省。
- 不开增量编译：单例是一次性编译。

**不走 cargo 的原因**
- 一是 cross-test §4.4 已列出的原因：指纹绑定路径，大 rlib 的检查约 7 s。
- 二是实测：剩余成本全部在 rustc 内部，0.58–0.96 s。

### 4.5 生产模式静态链接

- 档案为 rlib，用户 crate 直接调用 rustc 静态链接；macOS 用 `-dead_strip`，Linux 用 `--gc-sections`，rustc 缺省已开。
- 首构 = 档案构建 + 用户 crate。465 类档案上，不开 LTO 77 s，fat LTO 102 s。按 cross-test §1.6 的线性拟合外推，3000 类以上的项目档案约 4–5 min；档案体量收窄见 crate-split §7.5.4 与 S7。
- 只改用户代码：不开 LTO 0.58 s；fat LTO 47 s，每次都要重做全程序 LTO。
- 终态取值（**决策点**）：
  - `rava build`（开发迭代）不开 LTO；
  - `rava build --release` 用 fat LTO、cgu 1、opt 3，得到可交付二进制，体积 −20%；
  - 两者的档案键不同（`B` 含 LTO / opt），各自缓存。

### 4.6 失败回退

逐级判定如下，每一级都记入 `build_status.json` 的 `archive` 字段。任一级成功即为「链接档案」，与命中等价。

1. **本机命中**：`~/.cache/rava/archive/jdk<M>/<B>/manifest.json` 存在，文件 sha256 与本机 `rustc -vV` 都校验通过，直接链接。
2. **拉取**（语料模式）：按 cross-test §4.5 从调度方拉取，带分块续传与拉取预算。
3. **本机构建同一个 `S`**：
   - 以下任一情况转为本机构建：档案未命中且拉取预算用尽；工具链不一致（记为漂移、上报）；文件校验失败；
   - 用户 crate 链接时报 E0460 / E0514 / 未定义符号，视为档案损坏：作废后重建一次。第二次仍失败，按真实编译错误报告。
   - 构建期间持 flock，同机其他进程等锁后直接复用。
4. **档案不覆盖本程序**：
   - `rava build --profile` 已对未覆盖的非用户类 / 方法报错（1b）。
   - 语料作业由调度方把该例并入档案，`P` 随之改变（cross-test §4.1）。
   - 单独运行时，退化为生产模式：以本程序闭包为档案，静态链接，结果正确，只是更慢。这条路径就是生产模式本身，不另设单例编译路径。

**不做的回退**
- 不回落到 cargo 编译整个 scratch。
- 不允许「部分链接档案、部分本地编译 JDK 类」的混合形态：会破坏 §4.3 的唯一性。

## 五、实施分步（每步可独立验收）

### 5.1 步骤

| 步 | 内容 | 验收 |
|---|---|---|
| L1 | 生成器：新增门面 crate `java_profile`（两种模式同源）；把档案侧登记表（类初始化钩子、反射方法 / 字段分派、VM 引导初始化序列）从用户 `main.rs` 移入门面，改为静态 fn 指针表加 `export_name` 导出，`create_java_vm` 读取；用户 `main.rs` 只登记用户行，`use java_profile as _;` | 用户 `main.rs` 中档案类登记行 = 0；`archive_emit_cli` 扩到门面，两程序逐字节相同；单例模式与档案模式输出不变（服务器抽查）；s2 上 HelloWorld / TestBridgeMethod 的用户 crate cargo 编译不慢于今天 |
| L2 | `rava archive build <profile.json> [--mode corpus\|production]`：生成档案树，cargo 编 rlib，rava 直接调用 rustc 链接门面 dylib，复制 std dylib，落盘 `lib/` 与 `manifest.json`，flock | 同一 `S` 两次构建 `B` 相同；`manifest` 校验通过；Linux（服务器）上 `readelf -d` 显示门面 RUNPATH 为 `$ORIGIN`，可执行文件 NEEDED 为门面文件名；语料包 zstd 后 ≤ 全集外推的 0.25 GB |
| L3 | `rava compile` 直接调用 rustc：读 `manifest`，按模板拼参数，解析 rustc 诊断 JSON，写 `build_status`；生产模式静态链接 | 1061 例结果与第 1 步一致；单例 p50 ≤ 1 s、p99 ≤ 3 s、峰值 ≤ 0.8 GB（按本文实测收紧，§6）；语料模式单例二进制 ≤ 2 MB |
| L4 | 回退链 §4.6：工具链漂移、校验失败、E0460 / E0514 作废重建、未覆盖时退生产模式；本机 GC 与引用登记同 cross-test §4.6 | 人为删档案文件、改 `rustc -vV` 记录、给未覆盖程序，各自走到对应级别且结果正确 |

- L1 与 crate-split 代理的布局改动有交集：门面依赖实现层 crate 的命名与个数。L1 在 crate-split 合入后开始。
- L2 / L3 只动 driver，与生成器解耦。

### 5.2 待服务器验证（本机无法覆盖）

`remote_rava` 只能运行 rava 子命令，跑不了本文的手工 rustc 实验，所以 Linux 差异放进 L2 / L3 的服务器验收：
1. 门面 `.so` 链接：rust-lld 缺省，GNU ld 次序问题是否不出现；导出符号数与链接峰值（预计与 macOS 的 3.5 GB 同量级，在构建者 7 GB 预算内）。
2. `readelf -d`：门面 `RUNPATH=$ORIGIN`、`NEEDED libstd-<hash>.so`；可执行文件 `RUNPATH=<archive>/lib`、`NEEDED libjava_profile.so`。
3. 单例 rustc 墙钟与峰值：全集档案（约 3609 类）上 1061 例的 p50 / p99 / 峰值。
4. panic：stub panic 发生在子线程与 Continuation 载体栈上时，在 unwind 下仍以退出码 101 结束，stderr 首行与今天相同（`run_tests` 的 `panicked at` / `stub:` 提取不变）。

## 六、量化目标（终态）

| 项 | 目标 | 依据 |
|---|---:|---|
| 单例用户 crate 编译 + 链接，p50 | **≤ 1 s** | s2 实测 0.58–0.96 s；全集档案约多 17% 的类，链接项随导出符号略增 |
| 单例 p99 | **≤ 3 s** | 用户类多的程序由类型检查 / codegen 线性增长决定；服务器全集验收 |
| 单例峰值 RSS | **≤ 0.8 GB** | 实测 455–507 MB |
| 用户 `main.rs` 中档案类登记行 | **0** | 今天约 1370 行 / 例 |
| 语料模式单例二进制 | ≤ 2 MB | 实测 0.15–0.70 MB；今天逐例 debug 二进制 19.5 MB 起 |
| 语料包（rmeta + 门面 + std） | ≤ 0.25 GB / 键（zstd） | s2 实测 202 MB；全集按类数外推约 0.24 GB，优于 cross-test §4.2 估的 0.4 GB |
| 门面 dylib 链接 | ≤ 15 s、≤ 4.5 GB，每键一次 | s2 实测 8.75 s / 3.48 GB |
| 生产模式只改用户代码（不开 LTO） | ≤ 2 s | 实测 0.58 s |

cross-test §5.2 的「p50 ≤ 2 s、p99 ≤ 5 s、峰值 ≤ 1 GB」按上表收紧。

## 七、风险与需用户决策的点

**决策点**
1. **语料模式改用 `panic = "unwind"`**：
   - 稳定版 Rust 下动态链接档案的唯一组合，F2 / F3 已排除其余路线；
   - 可观测行为不变：钩子在 unwind 前 `exit(101)`；
   - 档案构建墙钟 +0.4%、峰值 +5%、rlib 体积 +12–17%，每键一次；
   - 生产模式保持 abort；
   - 否决此点，语料模式只能静态链接，单例峰值 2.3–2.6 GB，不满足 ≤ 1 GB。
2. **生产模式 LTO**：开发迭代不开 LTO（只改用户代码 0.58 s），`--release` 用 fat LTO（同场景 47 s，体积 −20%）。另一选项是 `--release` 也不开 LTO，换迭代速度。
3. **门面 crate 名 `java_profile`**：
   - 它是 crate 名，不是 Java 命名空间里的 trait，不违反 CLAUDE.md 的命名原则；
   - 但它是新的自造顶层名，请确认；备选 `java_archive`。

**风险**
- **导出符号规模**：s2 门面导出 650,965 个符号，单例链接固定约 0.4 s；全集按类数约 75 万。若 Linux 上链接项明显更慢，可在门面链接时只导出用户 crate 可见的符号（version script / `-exported_symbols_list`，由 rmeta 可见项生成），列为 L3 的可选优化，以实测为准。
- **macOS `split-debuginfo=unpacked`**：门面的调试信息留在构建者的 `.o` 中，拉取方的 Rust 回溯缺档案帧的文件 / 行号。Java 栈帧走自己的 LINE_TABLES，不受影响。服务器为 Linux，调试信息内嵌在 `.so`，同样不受影响。
- **与 crate-split 并行**：实现层 crate 的个数与命名由 crate-split 决定，门面只遍历档案事实，不写死个数。
- **全集档案构建峰值**：s2（3077 类）cargo 构建峰值 6.33 GB，来自 `java_runtime`，与 cross-test §1.6 的外推（约 6.0 GB）一致。构建者须满足 ≥ 7 GB，不变。

## 八、交接

- 实验目录：`build/t1link/`（s1、s2 档案树，`target-*`，`out/`）。体积较大，随时可删。
- 下一步：用户决策 §七后，按 §5.1 的 L1 → L4 实施。L1 等 crate-split 合入后开始。

# rava 项目指南

## 项目定位

Java → Rust 转译器。将 Java `.class` 字节码翻译为等价的 Rust 源码，生成一个可直接编译运行的 Cargo workspace。

---

## 优先级原则

> **架构问题优先于 Bug 修复。架构问题解决后，测试错误自然消解。**

- 任务执行顺序：架构设计与实现 → 编译错误归零 → 测试修复
- **禁止在架构改造完成之前运行测试**（运行结果无意义，且分散注意力）
- Bug 修复不是最高优先级；深远的架构缺陷（如多态分派缺失）是最高优先级
- 评估任务价值时，优先选择覆盖面最广、影响最深远的改造

---

## 转译等价性原则

> 详细的 Java → Rust 对照规则见 **[`docs/plans/java-rust-translation-reference.md`](docs/plans/java-rust-translation-reference.md)**。  
> 产品定位与市场策略见 **[`docs/plans/2026-09-18-product-vision.md`](docs/plans/2026-09-18-product-vision.md)**。

**核心目标**：rava 是 Java 语言的新编译后端，不是迁移工具。开发者继续写 Java，构建流程自动生成原生二进制。生成的 Rust 是可读的中间层——Java 开发者能直接对应原始逻辑，无需学习 Rust 的所有权/生命周期/trait dispatch。

```java
// Java（开发者写的）
Animal animal = new Dog();
animal.speak();
```
```rust
// 生成的 Rust（开发者能读懂，不需要自己写）
let animal: Animal = Dog::new();
animal.speak();
// vtable dispatch、Rc<dyn Trait>、borrow 全部由生成器封装，不出现在这里
```

**判定标准**：方法体中若出现 `borrow()`、`downcast::<T>()`、`Rc::new`、`Object::from_any` 等 Rust 底层调用，即封装不足，需修复生成器。禁止列表见 [`java-rust-translation-reference.md §16`](docs/plans/java-rust-translation-reference.md#16-禁止出现在可读层的调用列表)。

---

## 核心架构原则（实现时必须遵守）

### 0. 代码生成优先：能生成的都走生成器

**能通过字节码翻译、codegen、或 proc-macro 宏实现的功能，禁止通过手写覆盖生成 Rust 文件解决。**

- 遇到编译错误，优先修复生成器逻辑或宏实现，而非给生成文件打补丁
- 生成器 + 宏建好后，编译错误自然消解；先打补丁会造成技术债务积累

**检验方式**：若某个问题的解决方案会修改 `build/` 下 scratch 里的生成文件（而非 `runtime/` 手写文件），则该方案违反本原则，需改用生成器或宏方案。

---

### 1. 手写边界：方法的语义以它自己的字节码为准

> **只有字节码无法表达该语义时才手写——没有字节码，或依赖运行期生成与加载字节码，或由 VM 直接驱动。性能替换不算。**

- 判定单位是**方法**，不是类或包；`@IntrinsicCandidate` 之类的性能内建照样翻译字节码
- 准入三类：① `ACC_NATIVE`；② 运行模型替换（lambda / indy 引导、LambdaForm 编译、动态代理等运行期类定义点）；③ VM 注入的状态与 VM 驱动行为的落地语义（引用类语义随 `Rc` 释放触发、不引入 GC，JVMTI、栈遍历等）
- 过渡类（策略截断）：`[boundary]` 前缀已删除（规范 §三 / §七）；整方法手写的 `#[jvm_boundary]` 已归零（引导映像第 5 步与 JceSecurity 构建期求值，2026-10-09 核实 runtime/ 下 0 处），`closure.toml [vm_boundary]` 只剩 `java/lang/Class`（其 `<init>` 只由 VM 调用）；新增即回归
- 终态下 Java 类的 struct 一律由字节码生成；VM 注入的隐藏字段由清单声明、生成器追加
- 每个非 native 手写方法在清单登记类别，raw-audit 按类计数（`non_native_overrides` 2026-09-28 已清零，新增即回归）
- `System.out.println`、`String`、`ArrayList` 等的 Rust 实现必须来自 JDK `.class` 字节码翻译，不得手写近似实现

完整规范（类别细则、struct 归属、文件布局、登记与审计、过渡期现状）：**[`docs/reference/handwritten-boundary.md`](docs/reference/handwritten-boundary.md)**。
收窄的实测与实施：[`docs/plans/2026-09-29-boundary-narrowing.md`](docs/plans/2026-09-29-boundary-narrowing.md)。

### 2. 只分析档案调用链上的方法内部依赖

档案（profile）是一个构建单元全体入口的调用链并集，在开放世界下计算：
- 生产构建：构建单元是一个用户项目，入口是该项目的全部 `main` 与种子；
- 语料构建：构建单元是 e2e 全集，入口是全体测试的 `main` 与种子。

```
全体入口 → A() → B() → C()（native，手写）
D()  ← 不在任何入口的调用链上，panic!("stub: ...") 存根，其依赖的类不被分析
```

- 不在档案调用链上的方法 → 生成 `panic!("stub: ClassName.method:descriptor")` 存根，不分析其内部引用的类
- 在档案调用链上有字节码的方法 → 翻译字节码；满足手写准入的方法 → 共置手写
- 档案内的 JDK 类只翻译、编译一次，生成结果与具体程序无关；单个程序可以含有它自己不可达、但档案可达的已翻译方法（语料模式动态链接档案，生产模式静态链接）
- 依赖「实例化集合」的折叠（null 接收者等）只对用户无法扩展的类型做（final / sealed / 非公开 / 私有与静态目标），档案只依赖 JDK 侧事实
- 闭包「正确且最小」以档案规模衡量
- `[boundary]` 前缀截断已删除，JDK 全部类按字节码翻译；`[vm_boundary]` 类按方法划分，残留过渡手写只剩 `#[jvm_boundary]` 方法（见规范 §三 / §七）

存根**不用 `todo!()`**：运行时命中存根时，panic 消息精确报出类名+方法名+描述符，方便定位哪条规则或翻译路径没有覆盖到。

**禁止以"保证编译通过"为由**将不在档案调用链上的类加入生成范围。

方案、实测与分发协议见 [`docs/plans/2026-10-01-cross-test-compile-reuse.md`](docs/plans/2026-10-01-cross-test-compile-reuse.md)（2026-10-03 用户采纳）。

### 3. 手写与生成代码共置，清单即边界

- 手写与生成放在同一目录层次，不存在独立的 `native_impls/` 目录：类 `X` 的手写方法放 `<x>_impl.rs`，与生成的 `<x>.rs` 并列，只写 `impl X { ... }`，由生成的 `mod.rs` 包含
- **禁止** `/// @field name: Type` 注释注入与 `#[path = "..."] mod _impl;` 远程引用
- 边界、放行、补种、VM 承载、手写登记全部集中在 `runtime/java_runtime/` 下的 TOML 清单（`closure.toml` / `seeds.toml` / `vm_intrinsics.toml`），生成器代码里不写类名特判

### 4. 生成器 crate 中不得出现任何 JDK 类名字面量

`generator/crates/` 下的闭包分析器与生成器 crate 中，不允许以字面量形式出现 `ArrayList`、`HashMap`、`String`、`System` 等 JDK 类名（由 `no_jdk_literals` / `jdk_literal_lint` 测试守护）。所有类型信息必须从 `.class` 文件与 `runtime/java_runtime/` 下的清单动态获得。

---

## 工作区结构（per-test scratch workspace）

> 2026-09-16 起实施，方案与动机见 `docs/plans/2026-09-16-per-test-scratch-workspace.md`

```
runtime/                            # 提交到 git：手写代码唯一真源
├── java_runtime/
│   ├── Cargo.toml                  # 宏依赖为 path = "<repo>/runtime/rava_macros"
│   ├── build.rs                    # 维护 native_status.toml；反射 / 注解元数据表
│   ├── closure.toml                # VM 边界类（[vm_boundary]，按方法划分）/ 动态对照清单
│   ├── seeds.toml                  # 补种（注解 / locale / JCA / 服务与资源束查找 / 按名资源）
│   ├── vm_intrinsics.toml          # VM 承载方法、调用点特判、VM 常量、引导映像求值（[concrete.boot]）
│   └── src/
│       ├── lib.rs / error.rs       # VM 基础设施（java/jdk/sun 顶层 mod 声明）
│       ├── java/lang/
│       │   ├── object.rs           # 手写（ObjectVTable，Arch-4）
│       │   ├── object_impl.rs      # 手写 native impl（co-located）
│       │   ├── string_ext.rs       # 手写扩展
│       │   └── ...
│       └── jdk/internal/...        # 共置手写（native 与 VM 契约方法，<x>_impl.rs）
├── java_support/                   # VM 支持类的 Java 源（Proxy$Dyn、BMH Species_Dyn 等）
└── rava_macros/                    # proc-macro crate（java_class! 块级宏）

build/                              # gitignore：每测试一次性 scratch
├── analyzer-target/                # rava 二进制
└── jdk<N>/                         # 按参考 JDK 主版本分根（jdk21 / jdk25）
    ├── target/                     # 共享编译缓存（CARGO_TARGET_DIR）
    └── <test_snake>/               # 每测试独立工作区
        ├── Cargo.toml              # workspace 根（emitter 生成）
        ├── java_base_decl/src/     # runtime/ 手写 overlay + 生成的 java.base 声明层
        ├── java_base_body_<k>/     # java.base 方法体（按规模切分）
        ├── java_base/              # java.base 门面 crate
        ├── <jmod 模块>/            # 其他 JDK 模块各一 crate（java_xml、jdk_zipfs …，按 jmod 命名）
        ├── java_meta/              # 反射 / 注解 / 行表元数据
        └── user/src/               # 该测试的用户类翻译
```

**规则**：
- 生成代码**永不提交**；仓库里只有 `runtime/` 手写真源
- 每次转译（`rava build`）流程：清空或复用 scratch → overlay `runtime/` → javac → 闭包 → 发射 → cargo build → 运行；rava 二进制在 `build/analyzer-target/release/rava`，run_tests 与各 shell 脚本每批开头自动构建
- 手写文件靠「无 `rava_macros::java_class` 生成标记」识别，`rava build` 不会覆盖它们
- `rava_macros` 不复制进 scratch，以绝对 path 依赖参与编译（共享 target 下缓存命中）

## 常用命令

```bash
# ── 本机：只跑 cargo check 与分布式派发 ──────────────────────────────────────────
(cd generator && CARGO_BUILD_JOBS=2 python3 ../../heavy_lock.py cargo check --release --tests --target-dir ../build/check-target)   # 本机唯一的编译检查
uv run --group cluster python scripts/cluster/distribute_tests.py --no-monitor --skip-setup --spot <tag> --ref <sha> --per-dir 0 --tests A B   # 集群抽查（--job 作业 / --reset 全量；结果 cluster_results/，服务器清单 ~/.config/rava/cluster.toml；dev 关机期间只有云服务器可用，见 docs/reference/cluster-testing.md）
uv run --group cluster python -m unittest discover -s tests/unit/cluster   # 集群分发脚本单元测试

# ── 仅服务器执行（经 distribute_tests.py --job / --spot 或 remote_rava.py 下发；本机只跑 cargo check）──────
cargo build --release -p driver --manifest-path generator/Cargo.toml --target-dir build/analyzer-target   # 构建 rava（新鲜时为空操作）
build/analyzer-target/release/rava build <Test.java>                     # 转译 + 编译 + 运行（scratch = build/<test>）
build/analyzer-target/release/rava build <Test.java> --stop-after emit   # 只生成
build/analyzer-target/release/rava build <Test.java> --clean             # 清空 scratch 重建
build/analyzer-target/release/rava compile build/<test>                  # 编译已生成的 scratch（结果见 build_status.json）
python3 scripts/run_tests.py                    # 全量 e2e（顺序）
python3 scripts/run_tests.py -j 4               # 并行
python3 scripts/run_tests.py --filter TestXxx   # 单测试
build/analyzer-target/release/rava build <Test.java> --stop-after emit --trace-class <类>   # 查某类为何入闭包（同 rava closure --why；另有 --debug / --strict / --raw-sites）
build/analyzer-target/release/rava audit api|corpus|native ...          # 编译前缺口审计（报告写 docs/reports/）
(cd generator && cargo test --release)         # 生成器 / 闭包分析器单元测试
python3 -m unittest tests.unit.<模块>            # 脚本单元测试（test_dyn_compare / test_baseline_diff）
# 手写层改动的验证：直接重跑相关测试（scratch 每次重新 overlay）
scripts/prune.sh                                # 清共享 target 过期产物（跑批间调用，防磁盘满）
scripts/run_bg.sh <tag> <cmd...>                # 后台跑批：低内存编译环境 + prune + 落盘 build/logs/bg/
scripts/gen_trees.sh <out> [Test...]            # 生成转译树（缺省=验收集 27 例）
scripts/compare_trees.sh <base> <new>           # 生成树逐字节对照 + raw-audit 对照（重构验收）
scripts/seed_check.sh <Test.java>               # 两次生成确定性检查
scripts/fetch_pilot_deps.sh [--no-scan]         # lib pilot 语料取包（清单 tests/lib_pilot/deps/pom.xml）+ dep_scan 透视
scripts/lib_pilot_golden.sh m1..m5              # JUnit/hamcrest crate golden 对账（前置：上一条）
```

命令行选项（`--debug` / `--strict` / `--trace-class` / `--raw-sites` / `--build-timeout` / `--release`）与会读取的标准环境变量见 **[`docs/environment-variables.md`](docs/environment-variables.md)**。

---

## 代码生成命名原则

**生成的 Rust 代码应与 Java 命名空间自然一致，是类层次的自然产物，而非人为插入的基础设施。**

具体规则：

1. **禁止在 `java_runtime` 中引入游离于 Java 命名空间之外的自造 trait 名称。**  
   例如：不得新增 `JvmObject`、`JvmIterable` 等带 `Jvm` 前缀或无对应 Java 类的 trait。  
   判断标准：如果 Java 标准库中不存在对应的 `java.lang.Xxx` 接口，就不应在 `java_runtime` 里手写同语义的 Rust trait。  
   例外（2026-10-11 用户裁决）：宏按类生成的 `X__VTable` 与每类方法 trait `X__Methods`（类型标记 crate 路线，S7 计划 §9.9）同 `ObjectVTable`，属生成层内部细节，不作公开 API，不出现在可读层方法体文本中。

2. **动态派发机制必须从 `java/lang/Object.class` 字节码翻译得到，方法名与 Java 完全一致。**  
   `Object = Rc<dyn ObjectVTable>` 的内部 vtable trait 名为 `ObjectVTable`，其方法名为 `hashCode`、`equals`、`toString`、`getClass`（与 Java 字节码中的 method name 一致），而非 `jvm_hash_code` 等带前缀的自造名称。  
   `ObjectVTable` 是 `java/lang/object.rs` 的实现细节，不应作为 `java_runtime` 的公开 API 对外暴露。

3. **`Object` 仅出现在真正的多态边界，不得将泛型参数强制擦除为 `Object`。**  
   - 参数在 Java 中声明为 `Object` → 生成 `Object`（真实多态边界）  
   - 参数在 Java 中是泛型 `T`，仅因类型擦除在 descriptor 中变为 `Object` → 生成 Rust 泛型参数 `T`  
   区分依据：读取 `generic_signature` 属性（注解中始终存在），而非 `descriptor`。

**检验方式**：在 Rust 生成代码中搜索任何 `jvm_` 前缀的方法名或不在 `java.*` 命名空间下的 trait，若存在即违反本原则。

> 命名原则和所有 Java → Rust 等价形式的完整对照表见 **[`docs/plans/java-rust-translation-reference.md`](docs/plans/java-rust-translation-reference.md)**。

---

## 代码规范

- 标识符、函数名、变量名用英文；注释、提交信息、文档用中文
- 计划文档保存在 `docs/plans/YYYY-MM-DD-description.md`
- 手写层（runtime/）改动后重跑相关 e2e 验证（scratch 每次重新 overlay，无需手动同步）

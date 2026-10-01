# rava

**Java 语言的新编译后端**：把 Java `.class` 字节码翻译为等价、可读的 Rust 源码，生成可直接 `cargo build`
的 workspace，产出原生二进制。开发者继续写 Java，构建流程自动得到原生程序——rava 不是一次性迁移工具。

- 产品定位：[`docs/plans/2026-09-18-product-vision.md`](docs/plans/2026-09-18-product-vision.md)
- Java → Rust 对照规则：[`docs/plans/java-rust-translation-reference.md`](docs/plans/java-rust-translation-reference.md)
- 语义等价性现状：[`docs/compatibility.md`](docs/compatibility.md)

```java
// Java（开发者写的）
Animal animal = new Dog();
animal.speak();
```
```rust
// 生成的 Rust（可读的中间层；vtable 分派、所有权细节由生成器与宏封装）
let animal: Animal = Dog::new()?;
animal.speak()?;
```

---

## 核心思路

- **JDK 也来自字节码翻译**：`String`、`ArrayList`、`System.out` 等的 Rust 实现由 JDK 自身的 `.class`
  翻译得到，而不是手写近似。手写只限两类：公开 API 的 `ACC_NATIVE` 方法（`*_impl.rs`）与 VM 边界类。
- **调用链闭包**：从 `main` 出发做 BFS，只翻译调用链上的方法；链外方法生成
  `panic!("stub: 类.方法:描述符")` 存根，命中即精确报出缺口。
- **边界截断**：调用链进入 `jdk/internal/`、`sun/` 等内部包即停止展开，由手写边界类承接；
  边界与放行清单集中在 `runtime/java_runtime/closure.toml`。
- **宏承载对象模型**：生成代码以 `java_class!` 块级宏描述类，由 proc-macro crate `rava_macros`
  展开为 struct、vtable、字段访问器、类型转换与反射元数据。
- **VM 语义对齐**：真多线程（OS 线程 + 原子 / 读写锁对象模型）、异常与栈回溯、反射 / 注解 /
  动态代理 / MethodHandle、类初始化时序、本地化与货币数据均按 JVM 行为实现，并有 e2e 对照。

## 流水线

```
.java ──javac──▶ .class（用户类）      JDK jmods（所选 JDK 版本的类库）
                      │                        │
                      └──────────┬─────────────┘
                                 ▼   rava build（generator/ 的 Rust 生成器，各阶段为同名 crate）
             classfile      二进制解析（常量池 / BootstrapMethods / LVT / 注解 / 异常表）
                                 ▼
             closure        精确闭包分析：可达方法 + 类初始化 + 分派目标 + 折叠点
                            （closure.toml 边界 / seeds.toml 补种 / vm_intrinsics.toml）
             input          闭包事实 → 发射范围；边界判定 / 种子全局 / 预检
                                 ▼
             cfg            控制流结构化（循环 / try 区域 / 条件）
             sim / instr    操作数栈模拟 → ir（Rust IR）
             method         变量提升 / 可变性 / 融合 / 后处理 → ir::render 渲染
                                 ▼
             emit           java_class! 宏块、模块树、Cargo workspace、main 引导
                                 ▼
             build/<测试>/   scratch workspace（runtime/ 手写 overlay + 生成代码）
                                 ▼  cargo build（rava_macros 展开）
                            原生二进制
```

## 快速开始

**环境要求**：Python 3.11+、JDK 21（语料基线；JDK 25 同样支持）、Rust stable。
大闭包单个 rustc 峰值约 14G 内存，16G 机器上重型用例自动单作业编译。

```bash
# 转译 + 编译 + 运行（scratch = build/<主类 snake 名>）
python3 scripts/main.py tests/e2e/01_basics/HelloWorld.java

python3 scripts/main.py Foo.java --no-run          # 只生成
python3 scripts/main.py Foo.java --clean           # 清空 scratch 重建
python3 scripts/main.py Foo.java --jdk 25          # 指定 JDK

# e2e 测试（期望输出由 JVM 生成，逐字比对）
python3 scripts/run_tests.py                       # 全量
python3 scripts/run_tests.py --filter TestXxx      # 单测试
scripts/run_bg.sh <tag> python3 scripts/run_tests.py --filter TestXxx   # 后台低内存跑批
```

诊断与构建选项（`--debug` / `--strict` / `--trace-class` / `--raw-sites` / `--build-timeout`）见
[`docs/environment-variables.md`](docs/environment-variables.md)。项目不设自有环境变量。

## 目录结构

```
rava/
├── generator/                     # 生成器（Rust workspace，rava CLI）
│   └── crates/
│       ├── classfile / resolve    # .class 解析 / JDK 与镜像定位
│       ├── closure                # 闭包分析（rava closure）
│       ├── input                  # 闭包事实 → 发射输入、运行时清单读取
│       ├── ty / ir                # 类型层 / Rust IR 与渲染
│       ├── cfg / sim / instr      # 控制流结构化 / 栈模拟 / 指令翻译
│       ├── method / emit          # 方法体生成 / 类、模块、workspace 发射与审计行
│       └── driver                 # rava 命令行（build / emit / closure）
├── runtime/                       # 手写代码唯一真源（提交 git）
│   ├── java_runtime/              # 运行时 crate：native 方法 *_impl.rs、VM 边界类、build.rs
│   │   ├── closure.toml           # 调用链边界 / VM 边界类 / 放行清单
│   │   ├── seeds.toml             # 补种（注解 / locale 资源束 / JCA 服务 / 模块资源 / 引导初始化）
│   │   └── vm_intrinsics.toml     # VM 承载方法、调用点特判、VM 常量
│   ├── java_support/              # VM 支持类的 Java 源（动态代理、BMH 物种等载体）
│   └── rava_macros/               # proc-macro crate（java_class! 块级宏）
├── scripts/                       # main.py / run_tests.py / 跑批与对照工具（rava_cli.py 调用 rava）
├── tests/
│   ├── e2e/                       # e2e 语料（61 个类别，1066 例）
│   ├── expected/                  # JVM 生成的期望输出
│   ├── unit/                      # 脚本单元测试（python3 -m unittest tests.unit.<模块>）；生成器单测在 generator/ 下 cargo test
│   └── lib_pilot/                 # jar 输入模式（JUnit / hamcrest crate）试点
├── docs/                          # 任务、兼容性、方案与报告
└── build/                         # 每测试一次性 scratch 与共享编译缓存（gitignore）
```

## 项目状态

- 任务与进展：[`docs/tasks.md`](docs/tasks.md)（只列开放项），完成项归档在 `docs/tasks-history-2026-09.md`
- 过渡态总清单（距最终态的全部差距，编号 FS-xx）：
  [`docs/plans/2026-09-26-transitional-state-inventory.md`](docs/plans/2026-09-26-transitional-state-inventory.md)
- 长期路线（含 Rust 单二进制重写 R0；Python 生成器已于 2026-10-01 删除）：
  [`docs/plans/2026-09-23-long-term-roadmap.md`](docs/plans/2026-09-23-long-term-roadmap.md)
- 开发约定（架构原则、手写层规则、命名原则）：[`CLAUDE.md`](CLAUDE.md)

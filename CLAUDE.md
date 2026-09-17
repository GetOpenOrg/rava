# java_rta 项目指南

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

**核心目标**：java_rta 是 Java 语言的新编译后端，不是迁移工具。开发者继续写 Java，构建流程自动生成原生二进制。生成的 Rust 是可读的中间层——Java 开发者能直接对应原始逻辑，无需学习 Rust 的所有权/生命周期/trait dispatch。

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

- 手写代码仅限两类：① native 方法（`*_impl.rs`）；② 内部边界类（`jdk/internal/`、`sun/`）整体手写
- 遇到编译错误，优先修复生成器逻辑或宏实现，而非给生成文件打补丁
- 生成器 + 宏建好后，编译错误自然消解；先打补丁会造成技术债务积累

**检验方式**：若某个问题的解决方案会修改 `build/` 下 scratch 里的生成文件（而非 `runtime/` 手写文件），则该方案违反本原则，需改用生成器或宏方案。

---

### 1. HelloWorld 必须运行在 JDK 字节码翻译出的 Rust 代码上

`System.out.println`、`String`、`ArrayList` 等的 Rust 实现必须来自对 JDK `.class` 文件的字节码翻译，**不得**来自手写 Rust 字符串（如 `runtime.py` / `java_runtime` crate 中的硬编码实现）。

`runtime.py` 中的手写 Rust 内容以及 `java_runtime` crate 中的 `String`/`ArrayList`/`System` 等实现是**临时绕行方案**，新功能不得依赖这些手写实现，已有的应逐步用字节码翻译替换。

### 2. 只分析调用链上的方法内部依赖

```
main() → A() → B() → C()（native，手写）
D()  ← 不在调用链上，panic!("stub: ...") 存根，其依赖的类不被分析
```

- 不在调用链上的方法 → 生成 `panic!("stub: ClassName.method:descriptor")` 存根，不分析其内部引用的类
- 在调用链上有字节码的方法（公开 API 类）→ 翻译字节码
- 在调用链上的 `ACC_NATIVE` 方法（公开 API 类）→ 在同目录的 `<classname>_impl.rs` 中手写
- 调用链进入内部包（`jdk/internal/`、`sun/`）→ 停止 BFS，该类视为内部边界类，整体手写（见规则 3b）

不在调用链上的方法生成 `panic!("stub: ClassName.method:descriptor")` 存根，**不用 `todo!()`**。原因：运行时命中存根时，panic 消息精确报出类名+方法名+描述符，方便定位哪条规则或翻译路径没有覆盖到。

**禁止以"保证编译通过"为由**将不在调用链上的类加入生成范围。

### 3. 手写实现与生成代码共置，按需增量实现

**手写代码与生成代码放在同一目录层次下，不存在独立的 `native_impls/` 目录。**

手写文件分两类，处理方式不同：

#### 3a. 公开 API 类的 native 方法（与生成文件共置）

公开 API 类（`java/`、`javax/`）的 `ACC_NATIVE` 方法，以 `<classname>_impl.rs` 的形式与生成的 `<classname>.rs` 并列存放。Struct 定义由 codegen 从字节码生成，`_impl.rs` 只添加方法 `impl` 块：

```
output/src/java/lang/
├── string.rs          ← codegen 生成（struct + 字节码翻译方法）
├── string_impl.rs     ← 手写（native 方法 impl，co-located）
└── mod.rs             ← codegen 生成，同时 pub mod string; mod string_impl;
```

`string_impl.rs` 中直接写 `impl String { ... }`，无需任何注解或注入机制，由 `mod.rs` 自然包含。

**禁止使用 `/// @field name: Type` 注释注入机制**，也禁止 `#[path = "..."] mod _impl;` 的远程引用方式。

#### 3b. 内部边界类（完整手写，BFS 在此截断）

当调用链从公开 API 进入 `jdk/internal/` 或 `sun/` 包时，**停止 BFS 展开**，该类视为「内部边界类」：

- **struct 和全部方法由手写文件完整定义**，codegen 不生成任何 struct
- 手写文件放在与公开 API 一致的对应目录层次下
- 因为手写文件完全拥有 struct 定义，字段可自由声明，无需 `@field` 注入

```
output/src/jdk/internal/misc/
├── internal_lock.rs   ← 手写（完整 struct + 被调用的方法）
├── unsafe.rs          ← 手写（完整 struct + ACC_NATIVE 方法实现）
└── mod.rs             ← codegen 或手写，pub use 两个类
```

**内部边界类的实现节奏：按调用链按需推进，不一次性全量手写。**  
当前调用链用到哪些方法，就实现哪些；其余保持 `panic!("stub: ...")` 存根。随着测试覆盖扩大，逐步补全。

```rust
// internal_lock.rs 示例：只实现当前被调用的 lock/unlock
pub struct InternalLock {
    _mutex: MutexHolder,    // 自有字段，无需注入
}
impl InternalLock {
    pub fn lock(&self) -> Result<()> { ... }    // 已实现
    pub fn unlock(&self) -> Result<()> { ... }  // 已实现
    pub fn tryLock(&self) -> Result<bool> {     // 当前未被调用
        panic!("stub: jdk/internal/misc/InternalLock.tryLock:()Z")
    }
}
```

#### 内部包边界截断规则

BFS 调用链分析规则：

| 调用目标所在包 | 处理方式 |
|--------------|---------|
| `java/`、`javax/`（公开 API） | 继续 BFS，翻译字节码 |
| `jdk/internal/`、`sun/`（内部包） | 停止 BFS，视为内部边界类，整体手写 |

**截断的意义**：报告数据（`docs/reports/2026-09-14-impl-strategy.md`）显示，跟随内部包调用链会使类数从 111 膨胀到 635（+470%）。边界截断将翻译规模压缩 83%，是策略性收益而非渐进优化。

**实现优先级**（按被引用次数，阶段 1 叶子层先做）：
- `jdk/internal/misc/InternalLock`：184 次引用，0 个 ACC_NATIVE，手写 7 个方法
- `jdk/internal/util/Preconditions`：537 次引用，0 个 ACC_NATIVE，手写 15 个方法
- `jdk/internal/misc/Unsafe`：1379 次引用，84 个 ACC_NATIVE，阶段 2 实现

### 4. Python 代码中不得出现任何 JDK 类名常量

`instr.py`、`type_map.py`、`emitter.py` 等生成器模块中，不允许以字面量形式出现 `ArrayList`、`HashMap`、`String`、`System` 等 JDK 类名。所有类型信息必须从 `.class` 文件的字节码注释中动态解析。

---

## 工作区结构（per-test scratch workspace）

> 2026-09-16 起实施，方案与动机见 `docs/plans/2026-09-16-per-test-scratch-workspace.md`

```
runtime/                            # 提交到 git：手写代码唯一真源
├── java_runtime/
│   ├── Cargo.toml                  # 宏依赖为 path = "<repo>/runtime/java_rta_macros"
│   ├── build.rs                    # 维护 native_status.toml
│   └── src/
│       ├── lib.rs / error.rs       # VM 基础设施（java/jdk/sun 顶层 mod 声明）
│       ├── java/lang/
│       │   ├── object.rs           # 手写（ObjectVTable，Arch-4）
│       │   ├── object_impl.rs      # 手写 native impl（co-located）
│       │   ├── string_ext.rs       # 手写扩展
│       │   └── ...
│       ├── java/util/function/     # Arch-1 接口存根（4 个）
│       └── jdk/internal/...        # 内部边界类（完整手写）
└── java_rta_macros/                # proc-macro crate（java_class! 块级宏）

build/                              # gitignore：每测试一次性 scratch
├── target/                         # 共享编译缓存（CARGO_TARGET_DIR）
└── <test_name>/                    # 每测试独立工作区
    ├── Cargo.toml                  # workspace 根（emitter 生成）
    ├── java_runtime/src/           # runtime/ 手写 overlay + 该测试生成的 JDK 类
    └── user/src/                   # 该测试的用户类翻译
```

**规则**：
- 生成代码**永不提交**；仓库里只有 `runtime/` 手写真源
- 每次转译（`scripts/main.py`）流程：清空或复用 scratch → overlay `runtime/` → codegen → cargo
- 手写文件靠「无 `java_rta_macros::java_class` 生成标记」识别，codegen 不会覆盖它们
- `java_rta_macros` 不复制进 scratch，以绝对 path 依赖参与编译（共享 target 下缓存命中）

## 常用命令

```bash
python3 scripts/main.py <Test.java>            # 转译 + 运行（scratch = build/<test>）
python3 scripts/main.py <Test.java> --no-run    # 只生成
python3 scripts/main.py <Test.java> --clean     # 清空 scratch 重建
python3 scripts/run_tests.py                    # 全量 e2e（顺序）
python3 scripts/run_tests.py -j 4               # 并行
python3 scripts/run_tests.py --filter TestXxx   # 单测试
# 手写层改动的验证：直接重跑相关测试（scratch 每次重新 overlay）
```

---

## 代码生成命名原则

**生成的 Rust 代码应与 Java 命名空间自然一致，是类层次的自然产物，而非人为插入的基础设施。**

具体规则：

1. **禁止在 `java_runtime` 中引入游离于 Java 命名空间之外的自造 trait 名称。**  
   例如：不得新增 `JvmObject`、`JvmIterable` 等带 `Jvm` 前缀或无对应 Java 类的 trait。  
   判断标准：如果 Java 标准库中不存在对应的 `java.lang.Xxx` 接口，就不应在 `java_runtime` 里手写同语义的 Rust trait。

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

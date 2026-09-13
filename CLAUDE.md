# java_rta 项目指南

## 项目定位

Java → Rust 转译器。将 Java `.class` 字节码翻译为等价的 Rust 源码，生成一个可直接编译运行的 Cargo workspace。

---

## 核心架构原则（实现时必须遵守）

### 1. HelloWorld 必须运行在 JDK 字节码翻译出的 Rust 代码上

`System.out.println`、`String`、`ArrayList` 等的 Rust 实现必须来自对 JDK `.class` 文件的字节码翻译，**不得**来自手写 Rust 字符串（如 `runtime.py` / `java_runtime` crate 中的硬编码实现）。

`runtime.py` 中的手写 Rust 内容以及 `java_runtime` crate 中的 `String`/`ArrayList`/`System` 等实现是**临时绕行方案**，新功能不得依赖这些手写实现，已有的应逐步用字节码翻译替换。

### 2. 只分析调用链上的方法内部依赖

```
main() → A() → B() → C()（native，手写）
D()  ← 不在调用链上，panic!("stub: ...") 存根，其依赖的类不被分析
```

- 不在调用链上的方法 → 生成 `panic!("stub: ClassName.method:descriptor")` 存根，不分析其内部引用的类
- 在调用链上有字节码的方法 → 翻译字节码
- 在调用链上的 native 方法 → 在 `native_impls/` 中手写 Rust 实现

不在调用链上的方法生成 `panic!("stub: ClassName.method:descriptor")` 存根，**不用 `todo!()`**。原因：运行时命中存根时，panic 消息精确报出类名+方法名+描述符，方便定位哪条规则或翻译路径没有覆盖到。

**禁止以"保证编译通过"为由**将不在调用链上的类加入生成范围。

### 3. 唯一需要手写 Rust 的是 native 方法实现

`ACC_NATIVE` 方法没有字节码，无法翻译，必须手写。这些手写文件放在 `output/native_impls/` 目录中。所有其他代码均由转译器自动从字节码生成。

### 4. Python 代码中不得出现任何 JDK 类名常量

`instr.py`、`type_map.py`、`emitter.py` 等生成器模块中，不允许以字面量形式出现 `ArrayList`、`HashMap`、`String`、`System` 等 JDK 类名。所有类型信息必须从 `.class` 文件的字节码注释中动态解析。

---

## 输出结构

### 当前过渡态（workspace 多 crate）

```
output/
├── Cargo.toml                  # workspace（成员：java_runtime, jdk_classes, user）
├── java_runtime/               ← 提交到 git（手写，VM 基础设施层）
│   └── src/
│       ├── error.rs            ← JvmError / Result<T>（永久保留）
│       ├── types.rs            ← Field<T>（永久保留）
│       └── java/               ← 临时：手写 Java 类实现，待替换为字节码翻译
├── jdk_classes/                ← gitignore（由 codegen 生成，不提交）
│   └── src/java/...            ← JDK 字节码 → Rust 存根
├── user/                       ← gitignore（由 codegen 生成，不提交）
│   └── src/                    ← 用户 Java 翻译
└── native_impls/               ← 提交到 git（手写：仅 native 方法实现）
    └── java/lang/system.rs     ← System.arraycopy 等
```

### 最终目标态（单 crate）

```
output/
├── Cargo.toml                  # workspace，成员：user
├── src/
│   ├── main.rs
│   ├── hello_world.rs          ← 用户 Java 翻译（字节码 → Rust）
│   └── java/                   ← JDK 字节码翻译（非手写）
└── native_impls/               ← 手写：仅 native 方法实现
    └── java/lang/system.rs
```

`jdk_classes` crate 和 `user` crate 是**代码生成产物**，不提交到 git（已加 `.gitignore`）。  
`java_runtime` crate 的 **`error.rs` / `types.rs` 是 VM 基础设施层，永久保留**；`java/` 子目录是临时手写实现，目标态由字节码翻译替换后删除。  
`runtime.py` 中的手写 Rust 字符串是临时绕行方案，新功能不得依赖。

---

## 当前状态（2026-09-13）

- `java_runtime/src/error.rs`、`types.rs`：VM 基础设施，**永久保留**
- `java_runtime/src/java/`：手写 String/ArrayList/System 等，**临时，待字节码翻译后删除**
- `runtime.py`：向 `java_runtime/src/java/` 写入手写内容，**临时使用**
- `jdk_classes` crate：BFS 生成的 `panic!("stub: ...")` 存根，**gitignored，过渡方案**

完整目标架构见：`docs/plans/2026-09-13-target-architecture.md`  
实现路径见：`docs/plans/2026-09-12-architecture-redesign.md`

---

## 代码规范

- 标识符、函数名、变量名用英文；注释、提交信息、文档用中文
- 修改后运行 `cd output && cargo check` 确保零错误再提交
- 计划文档保存在 `docs/plans/YYYY-MM-DD-description.md`

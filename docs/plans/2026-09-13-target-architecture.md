# 目标架构：基于调用链的 JDK 字节码翻译

**日期**：2026-09-13  
**状态**：架构定稿（指导所有实现）  
**优先级**：本文档的原则优先于所有其他计划文档

---

## 核心原则

### 原则一：HelloWorld 必须使用 JDK 字节码翻译出来的 Rust 代码运行

测试 HelloWorld 时，`System.out.println`、`String` 等的 Rust 实现**不得**来自 `runtime.py` 里的手写字符串。它们必须由转译器从 JDK 的 `.class` 文件翻译产生。

`runtime.py` / `java_runtime` 中手写的 `String`、`ArrayList`、`System` 等实现是**临时绕行方案**，目标态必须删除。

### 原则二：只分析调用链上的方法依赖

```
main()
 └─ 调用 A()         ← 有字节码 → 翻译字节码，分析其依赖
      └─ 调用 B()    ← 有字节码 → 翻译字节码，分析其依赖
           └─ 调用 C() ← native  → 只有这里需要手写 Rust

D()                  ← 不在调用链 → panic!("stub: ...") 存根，其依赖的类不分析
```

**关键推论**：未被调用的方法生成 `panic!("stub: ClassName.method:descriptor")` 存根。这些存根不会执行，所以它们内部引用的其他 Java 类**不需要被分析或生成代码**，从而阻断依赖爆炸。运行时一旦命中存根，panic 消息精确报出是哪个方法没有被翻译覆盖，便于定位缺口。

### 原则三：唯一需要手写 Rust 的是调用链上的 native 方法

JVM 的 `ACC_NATIVE` 方法没有字节码，无法翻译。对于调用链上到达的 native 方法，提供手写 Rust 实现（放在 `native_impls/` 目录）。

对于不在调用链上的 native 方法，同样生成 `panic!("native: ClassName.method:descriptor")` 存根，无需手写实现。

---

## 三层代码结构（目标态）

```
output/
├── src/
│   ├── main.rs
│   ├── hello_world.rs            ← 用户 Java 翻译（字节码 → Rust）
│   └── java/                     ← JDK 类翻译（字节码 → Rust）
│       ├── lang/
│       │   ├── string.rs         ← java/lang/String.class 翻译
│       │   ├── system.rs         ← java/lang/System.class 翻译
│       │   └── object.rs         ← java/lang/Object.class 翻译
│       └── util/
│           ├── array_list.rs     ← java/util/ArrayList.class 翻译
│           └── ...
│
└── native_impls/                 ← 手写：仅 native 方法实现
    └── java/
        ├── lang/
        │   ├── system.rs         ← System.currentTimeMillis, arraycopy 等
        │   └── string.rs         ← String.charAt, String.length 等
        └── io/
            └── print_stream.rs   ← PrintStream.write 等
```

**不存在** `java_runtime` crate（或其中的 `String`/`ArrayList` 等手写实现）。`runtime.py` 中的 Rust 字符串全部删除。

---

## 调用链分析流程

### BFS 扫描（当前已实现，保留）

从用户类的方法调用指令出发，BFS 收集所有可达 JDK 类的 binary name：

```python
# transpile.py - _discover_jdk_classes() 已实现此逻辑
初始集合 = 用户类指令中引用的 JDK 类
BFS 展开：每个类的方法指令 → 新引用的 JDK 类
截断规则（_CUTOFF_PREFIXES / _CUTOFF_CLASSES）阻止依赖爆炸
```

### 方法级别的细化（目标态扩展）

当前 BFS 以**类**为粒度。目标态精化到**方法**粒度：

1. 从 `main()` 出发，追踪实际被 `invokevirtual/invokestatic/invokespecial` 调用的具体方法
2. 只翻译可达方法的字节码；不可达方法生成 `panic!("stub: ...")` 存根
3. 不可达方法引用的类不加入 BFS 队列

这样 `ArrayList` 的 `sort()`、`subList()` 等 HelloWorld 不调用的方法全部存根化，其内部依赖的 `Comparator`、`AbstractList` 等类也不需要翻译。

---

## 各类方法的处理规则

| 方法类型 | 在调用链上 | 不在调用链上 |
|---------|-----------|------------|
| 有字节码的普通方法 | 翻译字节码 → Rust | `panic!("stub: ClassName.method:descriptor")` |
| `ACC_NATIVE` 方法 | `native_impls/` 手写 Rust | `panic!("native: ClassName.method:descriptor")` |
| `ACC_ABSTRACT` 方法 | 翻译具体实现类的覆写方法 | `panic!("stub: ClassName.method:descriptor")` |

---

## 现状与目标的差距

| 层次 | 当前状态 | 目标状态 |
|-----|---------|--------|
| `java_runtime` crate | 手写 `String`/`ArrayList`/`System` 等完整实现 | **删除** |
| `runtime.py` | 包含手写 Rust 字符串 | **删除** |
| `jdk_classes` crate | 所有方法一律 `panic!("stub: ...")`（类粒度存根） | 调用链上的方法翻译字节码，不在调用链的方法 `panic!("stub: ...")` |
| `native_impls/` | 不存在 | 新建，只包含调用链上的 native 方法实现 |

---

## 实现路径

详细实现步骤见：

- `2026-09-12-architecture-redesign.md`：阶段 A（IR 全量迁移）→ B（数据驱动分派）→ C（属性格式）→ **D（完整 JDK 字节码翻译）** → E（native_impls）
- `2026-09-12-jdk-bytecode-translation.md`：P0-P4 原始计划

**阶段 D（完整 JDK 字节码翻译）是核心里程碑**：完成后，`java_runtime` / `runtime.py` 可以删除，HelloWorld 将完全运行在字节码翻译出的 Rust 代码上。

---

## 验收标准

以下测试通过即为目标架构达成：

```bash
# 1. 生成代码不包含对 java_runtime crate 的依赖
grep -r "java_runtime" output/user/Cargo.toml  # 应无输出

# 2. HelloWorld 能够运行
cd output && cargo run --release
# 输出：Hello, World

# 3. 所有实际执行的代码来自 JDK 字节码翻译
# output/src/java/lang/system.rs 存在，且方法有真实实现（非 panic! 存根）
```

# Java 字节码转译器：可达性分析与增量生成技术设计文档

**版本** 0.2 · **日期** 2026-09-12 · **状态** 进行中（P0–P3 已验证通过）

---

## 目录

1. [背景与问题陈述](#1-背景与问题陈述)
2. [核心概念：调用图分析算法谱系](#2-核心概念调用图分析算法谱系)
3. [为什么 Hello World 会产生 5000 个类依赖](#3-为什么-hello-world-会产生-5000-个类依赖)
4. [四层过滤漏斗架构](#4-四层过滤漏斗架构)
5. [第一层：API Surface 入口点定义](#5-第一层api-surface-入口点定义)
6. [第二层：改进的 RTA 分析](#6-第二层改进的-rta-分析)
7. [第三层：方法分类器](#7-第三层方法分类器)
8. [第四层：清单与空壳生成](#8-第四层清单与空壳生成)
9. [完整流水线设计](#9-完整流水线设计)
10. [数字预估](#10-数字预估)
11. [验证方法（Python 脚本）](#11-验证方法python-脚本)
12. [开放问题](#12-开放问题)

---

## 1 背景与问题陈述

### 1.1 目标

将 Java 字节码（`.class` / `.jar`）转译为目标语言源码。转译对象包括：

- 用户编写的 Java 代码
- JDK 标准库（`java.base` 等模块，~7500 个类，~85000 个方法）
- 第三方依赖包

### 1.2 核心矛盾

直接转译所有可见类会导致产物规模不可接受。以 `System.out.println("Hello World")` 为例，朴素的静态分析会拉入 5000 个以上的类，而其语义上真正需要的调用链只涉及约 30 个类、150 个方法。

```
朴素分析：5000+ 类   →   编译失败 / 产物过大
精确分析：~30 类     →   可编译，行为等价
```

**解决思路**：不转译所有代码，而是从明确声明的 API 入口点出发，通过可达性分析精确确定需要转译的最小方法集合，再对这个集合按类型分类处理。

---

## 2 核心概念：调用图分析算法谱系

从精度最低到最高，有五个层次。选型时在精度和速度之间权衡。

### 2.1 CHA — Class Hierarchy Analysis

对虚调用 `obj.foo()`，把 `obj` 静态声明类型的整个继承子树下所有有 `foo()` 的类都视为候选，不管这些类有没有被实例化。

```
Animal.speak()  →  Dog.speak(), Cat.speak(), Fish.speak()
                   （不管有没有 new，全部保留）
```

- **精度**：最低，大量误报
- **速度**：最快，O(继承树大小)
- **适用**：IDE"查找实现"、增量编译依赖图

### 2.2 RTA — Rapid Type Analysis

在 CHA 基础上加约束：**只有被 `new` 实例化过的类，其方法覆写才视为可达**。

```
instantiated = {Dog}          ← 全程序 NEW 指令收集
Animal.speak()  →  Dog.speak()  ✓（new 过）
                   Cat.speak()  ✗（从未 new）
```

全局一次性分析，不需要区分调用上下文。这是工程上的甜点：精度足够，速度可接受。

- **精度**：中等
- **速度**：O(程序规模)
- **适用**：TeaVM、ProGuard shrinking、Android R8

### 2.3 VTA — Variable Type Analysis

RTA 的升级版，精度更高。不只问"这个类有没有被 new 过"，而是问"流到这个特定调用点的 receiver 变量上，实际能有哪些类型"。

在全程序构建**赋值流图**（assignment flow graph），节点是变量，边是赋值/传参/返回，在图上传播类型集合直到不动点。

```
o = new Dog()   →  o 的类型集 = {Dog}
o 作为参数传入  →  参数 p 的类型集 = {Dog}
p.speak()       →  候选 = {Dog}，而不是"Animal 所有子类"
```

TeaVM 文档里引用的 Sundaresan 论文（*Practical Techniques for Virtual Call Resolution in Java*，OOPSLA 2000）描述的就是这一层算法。

- **精度**：高（流不敏感，不区分 if/else 分支，但已显著优于 RTA）
- **速度**：O(程序规模 × 类型集大小)
- **适用**：Wala `ZeroOneCFA`、Soot `CHA+VTA`

### 2.4 k-CFA — k 层上下文敏感分析

同一个函数从不同调用点进入时，维护独立的分析状态（上下文由最近 k 层调用栈确定）。

```
// k=1
call serialize(obj) from pathA  →  obj 类型集 = {JsonData}
call serialize(obj) from pathB  →  obj 类型集 = {XmlData}

// k=0（退化为上下文不敏感）
call serialize(obj)             →  obj 类型集 = {JsonData, XmlData}
```

- **精度**：k 越大越精确，理论上无上限
- **速度**：指数级，k≥2 在大型程序上不可用
- **适用**：小程序安全分析、学术研究

### 2.5 Points-to Analysis — 指针分析

不止追踪类型，而是追踪**对象身份**（哪个 `new` 出来的对象能流到哪里）。

```
o1 = new Dog()   → allocation site 1
o2 = new Dog()   → allocation site 2
a  = o1
a.speak()        → 只有 o1（site 1）这个对象
```

主流实现：Andersen's analysis（O(n³)，精度最高）、Steensgaard's（线性，精度略低）、Soot Spark/Paddle。

- **精度**：最高，可区分同一类的不同实例
- **速度**：重，大型代码库需要分钟级
- **适用**：编译器深度优化、安全漏洞分析

### 2.6 选型建议

对字节码转译场景，推荐以 **RTA 为基础 + 局部 VTA 补充**：

| 场景 | 推荐算法 |
|------|---------|
| 初次分析、确定大致范围 | RTA |
| JDK 内部虚调用链，需要更精确的候选集 | VTA（对 JDK 子集运行） |
| 需要区分同一类不同实例的行为 | Points-to（按需，局部运行） |
| 生产环境全量分析 | RTA + cutoff 截断规则（本文方案） |

---

## 3 为什么 Hello World 会产生 5000 个类依赖

朴素 RTA 的数字失控，根源在三个地方：

### 3.1 `<clinit>` 链污染

JVM 规范规定类在首次被引用时执行静态初始化块（`<clinit>`）。如果把 `<clinit>` 当普通方法处理，调用 `java.lang.String` 会触发：

```
String.<clinit>
  → StringCoding.<clinit>
    → Charset.<clinit>
      → CharsetProvider（SPI）
        → ServiceLoader
          → 整个 SPI 体系（数百个类）
```

Hello World 根本不做编码转换，这条链是完全的假性依赖。

**根因**：`<clinit>` 不应与普通方法等同对待。类的静态初始化只在该类实际被实例化或其静态成员被访问时才有意义；在可达性分析阶段，应当将 `<clinit>` 的跟随设置为条件触发，而非无条件跟随。

### 3.2 接口宽化

调用 `List.add()` 时，如果分析器在 `instantiated` 集合中找到了任何实现了 `List` 接口的类（包括 JDK 内部类），就会将其方法纳入候选。JDK 内部工厂方法会默默 `new` 出许多内部实现类（`Arrays$ArrayList`、`Collections$UnmodifiableList`、`Collections$SingletonList` 等），这些类会通过工厂方法间接进入 `instantiated` 集合，进而拉入大量 `List` 接口的实现。

### 3.3 工厂方法创建的对象逃过 NEW 指令追踪

标准 RTA 只追踪 `NEW` 字节码指令。但 JDK 内部大量使用工厂模式：

```java
// 这行代码在字节码层面没有 NEW 指令
List<String> list = List.of("a", "b");
// 实际执行的是 ImmutableCollections$List2.<init>()
// 只有追踪 return new XXX() 的返回类型才能发现
```

这导致 `instantiated` 集合不完整，或者分析器为了保守而扩大候选范围，两种情况都会导致数字失真。

---

## 4 四层过滤漏斗架构

```
原始字节码（java.base ~7500 类，~85000 方法）
              │
              ▼  Layer 1：API Surface 限定
         明确声明的入口点集合
              │
              ▼  Layer 2：改进的 RTA（隔离 <clinit>，截断规则）
         可达方法集（预计 200-500 个 for Hello World）
              │
              ▼  Layer 3：方法分类器
     ┌────────────────┬───────────────┬───────────────┐
     │  AutoTranspile │  NativeStub   │  Unsupported  │
     │  可自动转译    │  native 边界  │  暂不支持模式 │
     └────────────────┴───────────────┴───────────────┘
              │
              ▼  Layer 4：清单 + 空壳生成
         manifest.toml  +  stubs/
```

---

## 5 第一层：API Surface 入口点定义

### 5.1 核心认识

**RTA 的起点不应该是用户的 `main()`，而应该是转译器承诺对外暴露的公开 API 集合。**

从 `main()` 出发会把分析范围交给用户代码，导致不同用户代码触发不同的分析结果，无法建立稳定的转译范围基线。从 API Surface 出发，则由转译器主动声明支持边界，范围可控、可重复。

### 5.2 API Surface 文件格式

```toml
# api-surface.toml
# 转译器明确支持的 API 入口点
# RTA 从这些方法出发向下追踪

[[entry]]
class      = "java/lang/System"
field      = "out"            # 字段访问，特殊处理
note       = "System.out → PrintStream"

[[entry]]
class      = "java/io/PrintStream"
method     = "println"
descriptor = "(Ljava/lang/String;)V"

[[entry]]
class      = "java/io/PrintStream"
method     = "println"
descriptor = "(I)V"

[[entry]]
class      = "java/lang/String"
method     = "<init>"
descriptor = "([C)V"

[[entry]]
class      = "java/lang/String"
method     = "valueOf"
descriptor = "(I)Ljava/lang/String;"

[[entry]]
class      = "java/util/ArrayList"
method     = "<init>"
descriptor = "()V"

[[entry]]
class      = "java/util/ArrayList"
method     = "add"
descriptor = "(Ljava/lang/Object;)Z"

[[entry]]
class      = "java/util/ArrayList"
method     = "get"
descriptor = "(I)Ljava/lang/Object;"
```

### 5.3 扩展策略

初期只添加验证过的入口点。随着手动实现层覆盖范围扩大，逐步增加条目。每新增一个入口点，重新运行分析，观察新增可达方法数，评估实现成本后再决定是否纳入。

---

## 6 第二层：改进的 RTA 分析

### 6.1 `<clinit>` 隔离

```python
class ReachabilityAnalyzer:

    def should_follow_edge(self, caller: MethodRef, callee: MethodRef) -> bool:
        # <clinit> 只在该类确实被 new 过时才跟随
        if callee.method_name == "<clinit>":
            return callee.class_name in self.instantiated

        # native 方法：到边界就停，记录到 native_frontier，不继续追踪
        if self.is_native(callee):
            self.native_frontier.add(callee)
            return False

        return True
```

这一条规则单独就能把 Hello World 的依赖从 5000+ 降到 1000 以内。

### 6.2 工厂方法返回类型追踪

标准 RTA 只追踪 `NEW` 指令。扩展到追踪工厂方法的返回类型：

```python
def collect_instantiated(self, method_body: MethodNode):
    for insn in method_body.instructions:
        # 标准：NEW 字节码
        if insn.opcode == NEW:
            self.instantiated.add(insn.operand)

        # 扩展：return new Foo() 中的 Foo
        # 通过分析 ARETURN 前的栈顶类型推断
        if insn.opcode == ARETURN:
            return_type = self.infer_stack_top_type(method_body, insn)
            if return_type:
                self.instantiated.add(return_type)
```

### 6.3 截断规则表

对已知会导致依赖爆炸的类，手动设置边界，不追踪其内部调用：

```toml
# cutoff.toml
[cutoff]
# 进入这些类的调用到此为止，记录为 NativeStub 或 Unsupported
classes = [
    "java/util/ServiceLoader",            # SPI，运行时动态加载
    "java/lang/reflect/Method",           # 反射
    "java/lang/reflect/Constructor",      # 反射
    "java/security/AccessController",     # 安全框架
    "sun/misc/Unsafe",                    # unsafe 操作
    "java/lang/ClassLoader",              # 类加载器
    "java/lang/invoke/MethodHandle",      # MethodHandle
    "java/lang/invoke/MethodHandles",     # MethodHandles.Lookup
    "sun/nio/cs/StreamEncoder",           # 字符集编码（触发 SPI）
]

# 这些方法体内的调用不追踪（只追踪到这一层）
methods = [
    "java/lang/Object#clone#()Ljava/lang/Object;",
    "java/lang/Runtime#exec#(Ljava/lang/String;)Ljava/lang/Process;",
]
```

### 6.4 完整分析流程（伪代码）

```python
def analyze(entry_points, jdk_classpath, cutoff_rules):
    instantiated = set()
    reachable    = {}       # key → MethodRef
    native_frontier = set()
    queue = list(entry_points)

    # 预扫描：收集全程序 NEW 指令（第一遍）
    for class_file in jdk_classpath:
        for method in class_file.methods:
            for insn in method.instructions:
                if insn.opcode == NEW:
                    instantiated.add(insn.operand)

    # BFS 可达性
    while queue:
        curr = queue.pop(0)

        if curr.key in reachable:
            continue
        reachable[curr.key] = curr

        # 截断规则检查
        if curr.class_name in cutoff_rules.classes:
            native_frontier.add(curr)
            continue

        method_body = load_method(curr, jdk_classpath)
        if method_body is None:
            continue

        for callee in extract_callees(method_body):
            if not should_follow_edge(curr, callee, instantiated):
                continue

            # 虚方法：RTA 过滤
            if callee.is_virtual:
                candidates = resolve_virtual_rta(callee, instantiated)
            else:
                candidates = [callee]

            for c in candidates:
                if c.key not in reachable:
                    queue.append(c)

    return reachable, native_frontier, instantiated
```

---

## 7 第三层：方法分类器

RTA 跑完后，对可达方法集中的每个方法打上标签，决定后续如何处理。

### 7.1 分类枚举

```python
class MethodCategory(Enum):
    AUTO_TRANSPILE      = "auto_transpile"
    # 字节码规整，可直接转译
    # 条件：无 native 调用、无反射、无 invokedynamic/MethodHandle、
    #       所有调用的方法均在可达集内或已有手动实现

    NATIVE_STUB         = "native_stub"
    # ACC_NATIVE flag，需手写目标语言实现
    # 附带：JNI 签名、建议的目标语言对应实现路径

    UNSUPPORTED_PATTERN = "unsupported"
    # 字节码存在但转译器当前不支持
    # 常见原因：invokedynamic（lambda）、MethodHandle、复杂异常表
    # 生成空壳，记录原因，等待转译器能力提升

    MANUAL_OVERRIDE     = "manual_override"
    # 已在手写运行时层有实现，跳过转译，直接链接
```

### 7.2 分类逻辑

```python
def classify(method: MethodNode, manual_registry: dict) -> MethodCategory:

    # 优先级 1：手写层已覆盖
    if method.id in manual_registry:
        return MethodCategory.MANUAL_OVERRIDE

    # 优先级 2：native flag
    if ACC_NATIVE in method.access_flags:
        return MethodCategory.NATIVE_STUB

    # 优先级 3：扫描字节码，寻找不支持的指令
    for insn in method.instructions:
        if insn.opcode == INVOKEDYNAMIC:
            return MethodCategory.UNSUPPORTED_PATTERN  # reason: "invokedynamic"
        if insn.opcode in METHOD_HANDLE_OPS:
            return MethodCategory.UNSUPPORTED_PATTERN  # reason: "MethodHandle"

    # 默认：可自动转译
    return MethodCategory.AUTO_TRANSPILE
```

### 7.3 各分类的数量预估（Hello World 场景）

| 分类 | 预估数量 | 说明 |
|------|---------|------|
| `AUTO_TRANSPILE` | ~200 | PrintStream 格式化逻辑、ArrayList 数组操作等 |
| `NATIVE_STUB` | ~13 | arraycopy、hashCode、currentTimeMillis 等 |
| `UNSUPPORTED_PATTERN` | ~15–30 | 不使用 Stream API 则更少 |
| `MANUAL_OVERRIDE` | 视手写层覆盖情况 | 随时间增长 |

---

## 8 第四层：清单与空壳生成

### 8.1 清单文件格式（`manifest.toml`）

清单是后续所有工作的**单一事实来源**。由分析阶段自动生成，人工可编辑。编辑后可跳过分析阶段直接进入生成阶段。

```toml
# manifest.toml — 由分析器生成，人工可维护

[meta]
entry_points     = ["api-surface.toml"]
jdk_version      = 21
generated_at     = "2026-09-12T10:00:00Z"
total_reachable  = 228
auto_transpile   = 200
native_stub      = 13
unsupported      = 15

# ── 可自动转译 ──────────────────────────────────────────
[[method]]
id          = "java/io/PrintStream#println#(Ljava/lang/String;)V"
category    = "auto_transpile"
class_file  = "java/io/PrintStream.class"

[[method]]
id          = "java/util/ArrayList#add#(Ljava/lang/Object;)Z"
category    = "auto_transpile"
class_file  = "java/util/ArrayList.class"

# ── Native 边界，需手填 ────────────────────────────────
[[method]]
id              = "java/lang/System#arraycopy#(Ljava/lang/Object;ILjava/lang/Object;II)V"
category        = "native_stub"
jni_name        = "Java_java_lang_System_arraycopy"
suggested_impl  = "std::ptr::copy"
status          = "todo"     # todo | done | wont_implement

[[method]]
id              = "java/lang/Object#hashCode#()I"
category        = "native_stub"
jni_name        = "Java_java_lang_Object_hashCode"
suggested_impl  = "pointer address as hash"
status          = "todo"

[[method]]
id              = "java/lang/System#currentTimeMillis#()J"
category        = "native_stub"
jni_name        = "Java_java_lang_System_currentTimeMillis"
suggested_impl  = "std::time::SystemTime"
status          = "done"     # 手写层已实现

# ── 转译器暂不支持的模式 ────────────────────────────────
[[method]]
id       = "java/util/stream/Stream#of#([Ljava/lang/Object;)Ljava/util/stream/Stream;"
category = "unsupported"
reason   = "invokedynamic"
status   = "todo"
```

### 8.2 `status` 字段语义

| 值 | 含义 | 生成代码行为 |
|----|------|------------|
| `todo` | 尚未实现 | 生成空壳 + 编译期错误，确保不会静默通过 |
| `done` | 手写层已实现 | 生成到手写层的链接/转发，不生成逻辑代码 |
| `wont_implement` | 明确不支持 | 生成运行时 panic，可通过 feature gate 隐藏 |

### 8.3 空壳生成规则

**`NATIVE_STUB`，`status = "todo"`**

生成带编译期错误的空壳，保证 cargo check / 等价工具的 CI 在未实现时报错，而不是静默产生错误行为：

```rust
// AUTO-GENERATED — DO NOT EDIT
// Source: java/lang/System#arraycopy
// JNI:    Java_java_lang_System_arraycopy
// Hint:   std::ptr::copy
// Status: TODO — implement in runtime/src/natives/system.rs

pub unsafe fn arraycopy(
    src: JRef<Object>, src_pos: i32,
    dest: JRef<Object>, dest_pos: i32,
    length: i32,
) -> Result<(), Thrown> {
    compile_error!(
        "native stub not implemented: java/lang/System#arraycopy"
    )
}
```

**`NATIVE_STUB`，`status = "done"`**

生成到手写实现的转发：

```rust
// AUTO-GENERATED — links to manual implementation
pub use runtime::natives::system::current_time_millis as currentTimeMillis;
```

**`UNSUPPORTED_PATTERN`，`status = "todo"`**

```rust
// AUTO-GENERATED — DO NOT EDIT
// Source: java/util/stream/Stream#of
// Reason: invokedynamic — transpiler does not support lambda desugaring

pub fn of<T>(values: JArray<T>) -> JRef<Stream<T>> {
    todo!(
        "invokedynamic not yet supported: java/util/stream/Stream#of"
    )
}
```

---

## 9 完整流水线设计

### 9.1 阶段划分

```
阶段 1  analyze     字节码 + api-surface + cutoff → manifest.toml
阶段 2  stub        manifest.toml → 空壳源码文件（含 compile_error!）
阶段 3  transpile   manifest.toml + 字节码 → 自动转译的源码（覆盖空壳）
阶段 4  check       编译检查，输出未实现的 native stub 清单
阶段 5  implement   手工填写 native stub（运行时手写层）
阶段 6  verify      端到端测试（Hello World → 运行输出正确）
```

阶段 3 完成后，`AUTO_TRANSPILE` 的方法已有实际代码，`NATIVE_STUB/todo` 和 `UNSUPPORTED/todo` 仍为 `compile_error!`，阶段 4 的 check 输出就是精确的 **TODO 清单**，按依赖深度排序即为优先级顺序。

### 9.2 JDK 类文件定位与反编译流程

RTA 分析完成后，需要把可达的 JDK 方法对应的 `.class` 文件找出来并转译成 Rust。这是 Hello World → Rust 可运行程序的核心环节。

#### 9.2.1 JDK 类文件定位

JDK 8 与 JDK 9+ 的类文件存放方式不同：

| JDK 版本 | 存放位置 | 提取方式 |
|----------|---------|---------|
| JDK ≤ 8 | `$JAVA_HOME/jre/lib/rt.jar` | `jar xf rt.jar java/io/PrintStream.class` |
| JDK 9+ | `$JAVA_HOME/lib/modules`（jimage 格式） | `jimage extract --dir=extracted $JAVA_HOME/lib/modules` |

实践上直接用 `javap` 命令即可，JDK 会自动从模块系统中定位类文件：

```bash
# 无需先解包，javap 直接读取 JDK 内部类
javap -verbose java.io.PrintStream
```

#### 9.2.2 反编译流程

从 manifest.toml 中读取 `AUTO_TRANSPILE` 类别的 JDK 方法，按类分组，每个类调用 `javap -verbose` 一次：

```python
def decompile_jdk_class(class_name: str) -> str:
    """
    class_name: JVM 内部名，如 "java/io/PrintStream"
    返回: javap -verbose 的文本输出
    """
    dot_name = class_name.replace('/', '.')  # java.io.PrintStream
    result = subprocess.run(
        ['javap', '-verbose', '-p', dot_name],
        capture_output=True, text=True
    )
    if result.returncode != 0:
        raise RuntimeError(f"javap failed for {dot_name}: {result.stderr}")
    return result.stdout
```

#### 9.2.3 javap 文本解析

`scripts/codegen/javap.py` 中的 `parse_javap(text, class_name) → ClassInfo` 解析 `javap -verbose` 的输出，提取：

- **类头**：类名、父类、实现的接口、access flags
- **字段列表**：字段名、描述符、access flags
- **方法列表**：每个方法的
  - 签名（名称 + 描述符 + 参数类型）
  - locals 数量、args_size
  - 字节码指令序列（pc → `Instr(opcode, operand, comment)`）

解析的核心是逐行扫描，识别三类标记行：

```
public void println(java.lang.String);   ← 方法签名行
  descriptor: (Ljava/lang/String;)V       ← 描述符行
  Code:
     0: aload_0                           ← 指令行（pc: opcode [operand] [// comment]）
     1: aload_1
     2: invokevirtual #7  // Method java/io/PrintStream.write:(Ljava/lang/String;)V
```

#### 9.2.4 Rust 代码生成

解析得到 `ClassInfo` 后，进入与用户代码相同的生成流程：

```
ClassInfo
  └─ gen_method_body(method, class_info)   ← scripts/codegen/method.py
       └─ sim_instr(ins, sim, ...)          ← scripts/codegen/instr.py（StackSim 模拟）
            └─ emit(rust_stmt)
  └─ write_cargo_project(out_dir, ...)     ← scripts/codegen/emitter.py
       └─ 按包路径写入 src/<pkg>/<class>.rs
```

JDK 类的生成结果放在 `output/src/java_runtime/java/` 下，按原始包路径组织：

```
java_runtime/java/io/print_stream.rs      ← java.io.PrintStream 的 Rust 实现
java_runtime/java/lang/system.rs          ← java.lang.System
java_runtime/java/util/array_list.rs      ← java.util.ArrayList
```

凡是分类为 `NATIVE_STUB` 或 `UNSUPPORTED_PATTERN` 的方法，不走字节码解析，而是直接生成带 `compile_error!` / `todo!()` 的空壳（见第 8 章），待手工实现后替换。

#### 9.2.5 Hello World 完整调用路径示例

```
HelloWorld.main()
  └─ System.out.println("Hello World")
       └─ [JDK] PrintStream.println(String)      ← AUTO_TRANSPILE → 反编译并生成 Rust
            └─ [JDK] PrintStream.write(String)   ← AUTO_TRANSPILE → 同上
                 └─ [JDK] System.arraycopy(...)  ← NATIVE_STUB   → compile_error! 空壳
```

用户只需实现约 13 个 native stub（见第 10 章），其余 ~200 个方法由转译器自动从 JDK 字节码生成 Rust。

### 9.3 命令行接口设计（参考）

```bash
# 阶段 1：分析，生成清单
transpiler analyze \
  --jdk     /path/to/jdk21 \
  --surface api-surface.toml \
  --cutoff  cutoff.toml \
  --out     manifest.toml

# 终端输出摘要：
# Reachable : 228 methods across 31 classes
# breakdown : auto=200  native=13  unsupported=15
# manual TODO: 13 native stubs  (5 already in runtime layer)

# 阶段 2：生成空壳
transpiler stub \
  --manifest manifest.toml \
  --out      src/generated/

# 阶段 3：自动转译（在空壳上覆盖）
transpiler transpile \
  --manifest manifest.toml \
  --jdk      /path/to/jdk21 \
  --out      src/generated/

# 阶段 4：编译检查，输出 TODO 清单
cargo check 2>&1 | grep "native stub not implemented" | sort

# 示例输出（即优先级清单）：
# error: native stub not implemented: java/lang/System#arraycopy
# error: native stub not implemented: java/lang/Object#hashCode
# error: native stub not implemented: java/lang/Object#getClass
# ... (10 more)

# 阶段 5：手写实现（以 arraycopy 为例）
# 在 runtime/src/natives/system.rs 中实现后：
# 将 manifest.toml 中对应条目的 status 改为 "done"
# 重跑阶段 3，自动替换为 pub use 转发

# 阶段 6：验证
cargo run --example hello_world
# 期望输出：Hello, World!
```

### 9.4 增量更新策略

清单文件一旦生成，人工编辑（修改 `status`、添加 `suggested_impl` 注释、标记 `wont_implement`）是常态操作，分析器不应无条件覆盖它。

建议策略：

- 分析器生成 `manifest.toml.new`，不直接覆盖 `manifest.toml`
- 工具提供 `merge` 命令，将新分析结果与现有手工编辑合并
- 合并规则：新分析发现的方法追加，已有条目的 `status` 和 `suggested_impl` 保留，只更新 `category`（如转译器升级后某方法从 `unsupported` 变为 `auto_transpile`）

---

## 10 数字预估

以 `System.out.println("Hello World")` 为例，各过滤层的效果：

| 过滤层 | 操作 | 剩余方法数（估） |
|--------|------|----------------|
| 原始 java.base | 无 | ~85,000 |
| 朴素 RTA（无 `<clinit>` 隔离） | — | ~5,000 类 / ~20,000 方法 |
| `<clinit>` 隔离 | 不追踪静态初始化副作用 | ~2,000 |
| API Surface 限定入口 | 只从声明的入口点出发 | ~800 |
| cutoff.toml 截断 | 砍掉 SPI/反射/安全框架 | ~350 |
| native 边界停止 | arraycopy 等不继续往里追 | ~228 |
| **最终可达集** | | **~200–250 方法，~30 类** |

分类分布（最终可达 228 个方法）：

| 分类 | 数量 | 手工工作量 |
|------|------|-----------|
| `AUTO_TRANSPILE` | ~200 | 零（转译器处理） |
| `NATIVE_STUB` | ~13 | 13 个函数体，每个 5–30 行 |
| `UNSUPPORTED_PATTERN` | ~15 | 暂时 `todo!()`，不阻塞 Hello World |

**Hello World 端到端跑通所需手工实现约 13 个 native stub。** 常见列表：

```
java/lang/System#arraycopy
java/lang/Object#hashCode
java/lang/Object#getClass
java/lang/Object#clone
java/lang/System#currentTimeMillis
java/lang/System#nanoTime
java/lang/Float#floatToRawIntBits
java/lang/Double#doubleToRawLongBits
java/lang/Double#longBitsToDouble
java/lang/String#intern
java/lang/Thread#currentThread
java/lang/Runtime#availableProcessors
java/io/FileDescriptor#sync
```

---

## 11 实现现状与验证方法

### 11.1 依赖管理

项目使用 `uv` 管理依赖，`pyproject.toml` 无第三方 Python 依赖：

```toml
[project]
requires-python = ">=3.12"
dependencies = []
```

- `javalang`（Java 源码 AST 解析库）已 vendor 到 `scripts/javalang/`，可在此基础上二次改造，无需 pip 安装
- `six`（Python 2 兼容层）已完全移除，`scripts/javalang/` 已改为纯 Python 3
- `jawa`（字节码解析库）未引入；转译器通过 `javap -verbose` 文本输出解析字节码，无需额外依赖

安装环境：`uv sync`，运行：`uv run python scripts/main.py <file.java>`

### 11.2 当前脚本结构（v0.2 实际状态）

```
scripts/
  main.py               ← CLI 入口：python scripts/main.py <file.java>
  java_rta.py           ← RTA 分析器（基于 javalang 源码 AST）✅ 已实现
  codegen/              ← Java → Rust 转译器包 ✅ P0–P3 已实现
    __init__.py         ←   对外暴露 transpile()
    types.py            ←   数据结构（Instr/FieldInfo/ParsedMethod/ClassInfo）
    type_map.py         ←   JVM→Rust 类型映射、装箱拆箱表
    javap.py            ←   javap -verbose 文本解析器
    cfg.py              ←   控制流分析（while 循环 back-edge 检测）
    stack.py            ←   StackSim JVM 操作数栈模拟
    instr.py            ←   字节码指令 → Rust 语句翻译
    method.py           ←   gen_method_body（方法体生成）
    emitter.py          ←   Cargo 项目输出、按包路径组织文件
    runtime.py          ←   java_runtime Rust 存根（JDK 包路径层级）
    transpile.py        ←   流水线：javac → javap → parse → codegen

  # 以下脚本规划中，尚未实现：
  classify.py           ☐ 方法分类器（AUTO_TRANSPILE / NATIVE_STUB / UNSUPPORTED）
  gen_manifest.py       ☐ 生成 manifest.toml
  gen_stubs.py          ☐ 从清单生成空壳（compile_error! / todo!）
  validate.py           ☐ 验证清单完整性
```

### 11.3 输出项目结构：Cargo Workspace（v0.3）

转译器生成一个 **Cargo workspace**，包含三个独立子 crate：

```
output/
  Cargo.toml                    ← [workspace] members = ["java_runtime", "jdk_classes", "user"]

  java_runtime/                 ← 【手写，永不自动生成】内部运行时封装
    Cargo.toml                  ← [lib]，无外部依赖
    src/
      lib.rs                    ← pub mod error; pub mod types; pub mod java; + prelude
      error.rs                  ← JvmError enum, Result<T> 类型别名
      types.rs                  ← Field<T>（内部用 Rc<RefCell<T>>，对外不可见）
      java/
        lang/
          string.rs             ← java.lang.String
          system.rs             ← java.lang.System / PrintStream
          math.rs               ← java.lang.Math native stubs
        util/
          array_list.rs         ← java.util.ArrayList<T>
          hash_map.rs           ← java.util.HashMap<K,V>
          hash_set.rs           ← java.util.HashSet<T>

  jdk_classes/                  ← 【自动生成】JDK .class 字节码翻译
    Cargo.toml                  ← [lib]，depends on java_runtime = { path = "../java_runtime" }
    src/
      lib.rs                    ← pub mod java;
      java/
        util/
          array_list.rs         ← ArrayList.class 字节码翻译（含泛型 <E>）
          hash_map.rs           ← HashMap.class 字节码翻译（含泛型 <K,V>）
          ...
        lang/
          object.rs             ← Object.class 字节码翻译

  user/                         ← 【自动生成】用户 Java 代码翻译
    Cargo.toml                  ← [[bin]]，depends on java_runtime + jdk_classes
    src/
      main.rs                   ← fn main() { HelloWorld::main()... }
      hello_world.rs            ← HelloWorld.java 翻译
```

**三层隔离原则**：
- `java_runtime`：内部实现（`Rc<RefCell<>>`、`dyn Any`）完全封装，对外只暴露 prelude
- `jdk_classes`：生成代码只用 `use java_runtime::prelude::*;`，不见任何内部结构
- `user`：生成代码只用 `use java_runtime::prelude::*;`，通过 `jdk_classes` 调用 JDK 类

**运行命令**：
```bash
python3 scripts/main.py tests/HelloWorld.java
cd output && cargo run --release -p user
```

新增 JDK 运行时存根：在 `scripts/codegen/runtime.py` 的 `RUNTIME_FILES` 字典中添加路径和内容，内容写入 `java_runtime/src/` 下对应位置。

### 11.4 已验证阶段（P0–P3）

| 阶段 | 内容 | Java 输出 | Rust 输出 | 状态 |
|------|------|-----------|-----------|------|
| P0 | 整数运算、while 循环、静态方法调用 | `15 21 720 2` | `15 21 720 2` | ✅ 通过 |
| P1 | 类实例化、字段读写、实例方法 | `7 14 10` | `7 14 10` | ✅ 通过 |
| P2 | 数组（newarray/iaload/iastore/arraylength） | `5 150 30 15` | `5 150 30 15` | ✅ 通过 |
| P3 | ArrayList、HashMap、HashSet，自动装箱 | `3 20 3 2 true 2 true` | `3 20 3 2 true 2 true` | ✅ 通过 |

### 11.5 快速运行验证

```bash
# RTA 可达性分析（基于源码 AST）
uv run python scripts/java_rta.py tests/TestP1.java

# Java → Rust 转译（基于字节码）
uv run python scripts/main.py tests/TestP0.java
cd output && cargo run --release

# 全量回归对比（Java vs Rust 输出）
for f in TestP0 TestP1 TestP2 TestP3; do
  java_out=$(java -cp tests/classes $f)
  uv run python scripts/main.py tests/$f.java > /dev/null 2>&1
  rust_out=$(cd output && cargo run --release 2>/dev/null)
  [ "$java_out" = "$rust_out" ] && echo "✓ $f" || echo "✗ $f"
done
```

---

## 12 开放问题

| 编号 | 问题 | 当前状态 | 优先级 |
|------|------|---------|--------|
| Q1 | `invokedynamic`（lambda/Stream）如何处理？是否纳入第一阶段支持范围 | 暂标记 `unsupported`，不阻塞 Hello World | 中 |
| Q2 | MethodHandle / VarHandle 是否需要支持 | 暂截断，标记为 `cutoff` | 低 |
| Q3 | 工厂方法返回类型追踪的准确性如何验证 | 待设计测试用例 | 高 |
| Q4 | cutoff.toml 的维护成本：随 JDK 版本升级是否需要更新 | 未评估 | 中 |
| Q5 | `<clinit>` 隔离是否会导致漏掉真正必要的初始化逻辑 | 需要反例验证 | 高 |
| Q6 | manifest.toml 的合并策略（新分析 vs 已有手工编辑）的边界情况 | 未设计 | 中 |

---

*文档由对话整理生成，描述的是转译器中可达性分析与增量生成子系统的设计思路。所有数字为估算值，以实际分析结果为准。*

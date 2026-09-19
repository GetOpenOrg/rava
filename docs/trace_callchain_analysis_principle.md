# `trace_callchain.py` 完整分析原理

> **2026-09-19 完整版修订**：本文描述 `scripts/trace_callchain.py` 的**完整版实现**（2026-09-19 重写）。
> 历史版本 `trace_callchain0917.py` / `trace_callchain0919.py` 保留作对照，不再演进：
> 基线版在字段指令处提取字段描述符类型（0917 起被移除），0919 补了 ldc/catch/字段声明三条通道
> 但丢失了字段描述符通道，且两者的层次索引不随 `shared_cache` 重建（见 3.4 注）。
> 完整版统一实现下述全部机制。

---

## 0. 文档定位与适用范围

**本文描述的是 `scripts/trace_callchain.py` 这一独立分析工具的算法原理，不是转译主流程的实现。**

两者是**两套独立实现**：

| | `scripts/trace_callchain.py` | `codegen/transpile.py::_discover_jdk_classes_method_level` |
|---|---|---|
| 定位 | 独立的链路分析 / 算法验证工具 | 转译流水线的一部分，产出真正参与 codegen 的类闭包 |
| 是否被主流程调用 | **否**（`scripts/main.py` 无引用；`codegen/transpile.py:301` 仅注释提及） | 是，`transpile()` 第 3 步调用 |
| 输出 | 调用链 + 引用集合 + 统计报告 | `jdk_class_infos` / `visited_methods` / `field_stubs` |

因此本文描述的算法细节**不能**直接当作转译器的行为依据。两者的覆盖差异见 **第九节**，
主流程侧独有的机制（边界截断、native upcall 等）见 **第 9.2 节**。

---

## 一、整体架构

脚本的核心任务是：**从用户编写的 Java 类出发，静态模拟 JVM 的类加载与方法分派过程，把运行时会真正触达的每一个类、每一个方法、每一条初始化路径都找出来**，不依赖实际运行，纯粹分析字节码。

整个流程分为三个阶段：

```
.java 文件
    ↓ javac 编译
.class 文件（用户类）
    ↓ _parse_class 解析
常量池 + 方法表 + 字段表 + 异常表
    ↓ BFS 展开
JDK jmods（zip 格式的 .class 库）
    ↓ 多轮分析
调用链 + 引用集合 + 统计报告
```

---

## 二、字节码信息提取清单

要完整还原调用链，必须从每个 `.class` 文件中提取以下 **7 类信息**，缺一不可。

---

### 2.1 常量池（Constant Pool）

`.class` 文件的核心索引结构，所有引用都通过常量池间接寻址。需要识别的条目类型：

| 标签值 | 类型 | 用途 |
|--------|------|------|
| 1 | `CONSTANT_Utf8` | 类名、方法名、描述符的字符串 |
| 7 | `CONSTANT_Class` | 类/接口引用（new、checkcast、父类等） |
| 8 | `CONSTANT_String` | 字符串字面量（间接持有 Utf8） |
| 9 | `CONSTANT_Fieldref` | 字段引用（getstatic/getfield 等） |
| 10 | `CONSTANT_Methodref` | 普通方法引用（invokevirtual 等） |
| 11 | `CONSTANT_InterfaceMethodref` | 接口方法引用（invokeinterface） |
| 12 | `CONSTANT_NameAndType` | 名字+描述符对，被 9/10/11 引用 |
| 15 | `CONSTANT_MethodHandle` | 方法句柄（invokedynamic 基础） |
| 16 | `CONSTANT_MethodType` | 方法类型描述符 |
| 17/18 | `CONSTANT_Dynamic/InvokeDynamic` | 动态调用（lambda/字符串拼接） |

解析规则：Long 和 Double 占两个槽位（索引 i 和 i+1），解析时需跳过。

---

### 2.2 字段表（Fields）

每个字段有：`access_flags`、`name_index`、`descriptor_index`、属性列表。

**必须提取：字段描述符中的类型引用。**

描述符格式：`Ljava/util/List;` 表示引用类型，`[Ljava/lang/String;` 表示数组。规则：
- `L类名;` → 提取类名
- `[L类名;` / `[[L类名;` → 剥掉所有 `[` 后提取类名
- `B C D F I J S Z V` → 基本类型，忽略

**原因**：字段声明了类型，即使方法体里没有任何对该字段的 get/put 操作，编译后仍然存在该类型依赖，Rust/C++ codegen 生成结构体时需要知道字段类型对应的类定义。

> **主流程对照（2026-09-19 核对）**：`codegen/transpile.py` **只**在遇到 `getstatic/putstatic/getfield/putfield` 指令时收集该字段描述符里的类型（`_collect_method_refs` → `_add_type_refs`），
> **没有**在首次解析类时遍历 `ci.fields` 统一收集声明类型。若某个类仅作为类型存根进入闭包、且从未被字段指令触达，其字段类型会缺失。见 9.3 缺口 B。

---

### 2.3 方法表（Methods）

每个方法有：`access_flags`、`name_index`、`descriptor_index`、属性列表（含 `Code` 属性）。
脚本将每个方法解析为 7 元组 `(name, descriptor, bytecode, is_native, is_abstract, is_static, catch_types)`。

**必须提取的内容：**

**① `access_flags`**：判断方法类型
- `ACC_NATIVE (0x0100)`：native 方法，无字节码，是调用链的边界
- `ACC_ABSTRACT (0x0400)`：抽象方法，无字节码，但描述符里的类型仍是依赖
- `ACC_STATIC (0x0008)`：静态方法，invokestatic 调用，无接收者

**② 方法描述符**：格式 `(参数类型列表)返回类型`，例如 `(Ljava/util/List;I)Ljava/lang/String;`。**必须从描述符中提取所有 `L类名;` 形式的类名**，无论方法有没有字节码。原因：
- abstract/native 方法无 `Code` 属性，只有描述符
- 这些类型会出现在生成代码的函数签名里，必须有对应的类型定义

**③ 特殊方法名**：
- `<init>`：构造方法，对应 `new` 指令后的 `invokespecial`
- `<clinit>`：静态初始化块，JVM 在类首次被使用时自动触发，不会出现在任何 invoke 指令中，必须特殊处理

**④ Code 属性**：包含字节码和异常表，见 2.4 和 2.5

---

### 2.4 字节码指令（Bytecode Instructions）

字节码是最核心的信息来源。需要扫描的指令分为 5 类：

**类型一：方法调用指令**

| 操作码 | 指令 | 含义 | 处理方式 |
|--------|------|------|---------|
| 0xB6 | `invokevirtual` | 虚方法调用（实例方法，按运行时类型分派） | 加入虚调用集合，RTA/VTA 过滤 |
| 0xB7 | `invokespecial` | 直接调用（构造方法、private、super） | 直接跟随展开 |
| 0xB8 | `invokestatic` | 静态方法调用 | 直接跟随展开 |
| 0xB9 | `invokeinterface` | 接口方法调用 | 加入虚调用集合，接口传播处理 |
| 0xBA | `invokedynamic` | 动态调用（lambda/方法引用/字符串拼接） | 当前版本跳过，需单独处理 |

每条调用指令后跟 2 字节常量池索引（invokeinterface 后跟 4 字节），通过索引查 `CONSTANT_Methodref` 或 `CONSTANT_InterfaceMethodref`，拿到 `(类名, 方法名, 描述符)` 三元组。

**类型二：对象实例化指令**

| 操作码 | 指令 | 含义 | 处理方式 |
|--------|------|------|---------|
| 0xBB | `new` | 创建对象实例 | 加入 `instantiated` 集合，触发 `<clinit>` |

`new` 指令是 RTA（快速类型分析）的关键输入：只有出现在 `instantiated` 里的类，其具体方法实现才会在 `invokevirtual`/`invokeinterface` 分派时被解析。

**类型三：字段访问指令**

| 操作码 | 指令 | 含义 | 处理方式 |
|--------|------|------|---------|
| 0xB2 | `getstatic` | 读取静态字段 | 提取字段所属类 |
| 0xB3 | `putstatic` | 写入静态字段 | 提取字段所属类 |
| 0xB4 | `getfield` | 读取实例字段 | 提取字段所属类 |
| 0xB5 | `putfield` | 写入实例字段 | 提取字段所属类 |

每条指令后跟 2 字节索引指向 `CONSTANT_Fieldref`，从中拿到字段所属类名，加入引用集合（不展开方法体，只记录类型依赖）。还需额外提取字段描述符里的类型（如 `putstatic System.out:Ljava/io/PrintStream;` 里的 `PrintStream`）。

**类型四：类型检查与数组指令**

| 操作码 | 指令 | 含义 | 处理方式 |
|--------|------|------|---------|
| 0xBD | `anewarray` | 创建引用类型数组 | 提取元素类型，加入引用集合 |
| 0xC0 | `checkcast` | 类型强制转换 | 提取目标类型，加入引用集合 |
| 0xC1 | `instanceof` | 类型判断 | 提取判断类型，加入引用集合 |
| 0xC5 | `multianewarray` | 创建多维引用数组 | 操作数是数组类名（如 `[[Ljava/lang/String;`），剥掉全部 `[` 前缀后提取元素类名，基本类型元素忽略（完整版新增） |

**类型五：常量加载指令**（0919 引入，完整版保留）

| 操作码 | 指令 | 含义 | 处理方式 |
|--------|------|------|---------|
| 0x12 | `ldc` | 加载常量（1字节索引） | 若常量池条目是 `CONSTANT_Class`，提取类名 |
| 0x13 | `ldc_w` | 加载常量（2字节宽索引） | 同上 |

Java 代码 `Class<?> c = ArrayList.class;` 或 `getClass().isAssignableFrom(List.class)` 会生成 `ldc Class java/util/List` 指令。这是类引用进入链路的又一个入口，原来完全被忽略。

> **主流程对照（2026-09-19 核对）**：**未覆盖**。`codegen/classfile.py::_ldc_str` 对 `CONSTANT_Class` 生成的注释形如 `class java/util/List`（带 `class ` 前缀），
> 而 `codegen/transpile.py::_collect_method_refs` 的末条分支只匹配 `comment.startswith(_JDK_PREFIXES)`，因此该通道被静默丢弃。见 9.3 缺口 A。

**变长指令的特殊处理：**

| 指令 | 处理规则 |
|------|---------|
| `tableswitch` (0xAA) | 需要 4 字节对齐填充 + low/high 边界计算跳转表长度 |
| `lookupswitch` (0xAB) | 需要 4 字节对齐填充 + npairs 数量计算 |
| `wide` (0xC4) | 后跟 1 字节子操作码，若子操作码是 `iinc (0x84)` 则总长 6 字节，否则 4 字节 |
| `multianewarray` (0xC5) | 3字节索引 + 1字节维度数，共 4 字节 |

> **主流程对照**：`multianewarray` 已正确解码，注释为元素类型名；但多维**引用**数组的注释形如 `[[Ljava/lang/String;`，
> 含 `[` 字符，被 `_collect_method_refs` 的 `'[' not in c` 守卫过滤掉 → 元素类型不进闭包。见 9.3 缺口 C。

---

### 2.5 异常表（Exception Table）

位于 `Code` 属性内，字节码之后。格式：每条 8 字节：

```
from_pc (u2) | to_pc (u2) | handler_pc (u2) | catch_type (u2)
```

`catch_type` 是常量池索引，指向 `CONSTANT_Class`，即 `catch (IOException e)` 里的 `IOException`。`catch_type = 0` 表示 `finally`，不对应具体类型。

**必须提取**，因为：
- `catch` 子句声明的异常类型不出现在任何 invoke 指令里
- 但 Rust/C++ codegen 生成异常处理代码时需要知道这个类型的定义

---

### 2.6 类继承结构

解析 `.class` 文件头部：

```
access_flags (u2)
this_class (u2)    → 常量池 CONSTANT_Class → 本类名
super_class (u2)   → 常量池 CONSTANT_Class → 父类名（0 表示 java/lang/Object）
interfaces_count (u2)
interfaces[i] (u2) → 常量池 CONSTANT_Class → 接口名
```

用途：
- **父类链**：构建继承层次，用于 `is_subtype` 判断（RTA 虚分派时需要知道哪些具体类是某接口/抽象类的子类型）
- **接口列表**：用于接口传播——`invokeinterface List.add()` 时，需要找到所有实现了 `List` 的已实例化类（如 `ArrayList`），把它们的 `add()` 实现也纳入调用链

---

### 2.7 `invokedynamic` 的 BootstrapMethods 属性（`trace_callchain.py` 当前版本跳过）

Lambda 表达式和字符串拼接（`+` 运算符在 Java 9+ 由 `StringConcatFactory` 处理）会生成 `invokedynamic` 指令。完整处理需要解析 `BootstrapMethods` 属性，找到对应的 `MethodHandle`，追踪到实际调用的方法。当前版本跳过此指令，是已知的局限。

> **主流程对照**：**已覆盖，且比本节描述更完整**。`codegen/classfile.py::_parse_bootstrap_methods` 解析 `BootstrapMethods`，
> 对 `LambdaMetafactory` 提取 `sam_type`（SAM 方法类型）与 `impl_method`（实现方法 `Cls.name:desc`），对 `makeConcatWithConstants` 提取模板串，
> 一并写入指令注释；`transpile.py` 再从 `impl:` 令牌把实现方法入队（`impl:Cls.method:desc`）。
> 差异：`samtype:` 令牌**未被** `_collect_method_refs` 使用，SAM 函数式接口类型不由此进闭包（通常已由方法描述符通道覆盖，故未暴露为故障）。

---

## 三、核心算法：BFS + 三层优化

### 3.1 基础 BFS

从用户类的所有方法出发，把方法调用三元组 `(类名, 方法名, 描述符)` 放入队列，依次展开每个方法的字节码，收集新的调用引用，循环直到队列为空。

```
初始种子 = 用户类所有方法的字节码扫描结果
队列 = 所有 JDK 类方法调用

while 队列非空:
    (cls, method, desc) = 取出队首
    解析 cls 的字节码（从 jmods 读取）
    找到匹配方法
    扫描其字节码 → 新的调用 → 入队
    收集描述符类型、字段类型、catch类型 → refs 集合
```

---

### 3.2 L1：Native 边界

`ACC_NATIVE` 的方法没有字节码，无法展开。发现后记入 `native_stubs` 集合，作为调用链的叶节点，不再递归。

---

### 3.3 L2：`<clinit>` 门控

`<clinit>` 永远不会出现在任何 invoke 指令中，JVM 在类被首次主动使用时自动调用。脚本用 `instantiated` 集合模拟这个行为：

```
规则：
  - new 指令发现一个类 → 加入 instantiated
  - 展开 JDK 类的某个方法时，若该类在 instantiated 中 → 同时展开其 <clinit>
  - 若该类不在 instantiated 中 → 跳过 <clinit>（该类还没被实例化）
```

这避免了把所有 JDK 类的静态初始化块都无差别展开（会引入大量无关依赖）。

---

### 3.4 L3：RTA 虚调用过滤

`invokevirtual` 和 `invokeinterface` 在运行时按接收者的实际类型分派，静态分析无法直接知道接收者是什么。RTA（Rapid Type Analysis）用一个近似：

```
规则：
  遇到 invokevirtual A.foo() 时：
    - 在 instantiated 集合里找所有 A 的子类型 {B, C, D}
    - 若有 → 分别展开 B.foo()、C.foo()、D.foo()
    - 若无 → 保守地展开 A.foo()（可能是 abstract，无方法体，但描述符类型仍收集）
```

Two-pass 变体：先做一遍无过滤 BFS 收集全量 `instantiated`，再用完整集合重做一遍，精度更高。

**层次索引的生命周期（完整版修复，历史版本缺陷）**：RTA 依赖的 `hier_super` / `hier_ifaces` 是每次
BFS 调用的局部状态，而类解析结果放在跨调用共享的 `shared_cache` 里。历史版本 `get()` 在缓存命中时
提前返回、不重建层次索引，导致 two-pass 第二程（必然热缓存）层次索引为空，`is_subtype` 退化为
`sub == sup`，接口分派目标整体丢失；受限域实验（闭包只含 ArrayList/List 等类）证实了这一点。
完整版的两条规则：
1. **缓存命中即重建**：`get()` 命中 `shared_cache` 时同步回填 `hier_super` / `hier_ifaces`；
2. **惰性解析**：`is_subtype` 沿父类链上溯时，中间类若尚未解析则先 `get()` 再查，
   避免链在未解析的中间类处提前断裂。

---

### 3.5 VTA（可选）：操作数栈类型追踪

在方法内部模拟操作数栈，追踪每个引用类型槽位持有的具体类型：

```
new ArrayList → 栈顶 = "java/util/ArrayList"
astore_1      → locs[1] = "java/util/ArrayList"
aload_1       → 栈顶 = "java/util/ArrayList"
invokevirtual List.add() 但栈顶是 ArrayList → 精确分派到 ArrayList.add()
```

接收者类型追踪同时覆盖 `invokevirtual` 与 `invokeinterface`（完整版起，历史版本仅前者）。
减少因声明类型为接口/抽象类而引入的保守展开，降低调用链规模。

---

### 3.6 方法解析：沿父类链与接口闭包定位声明者（完整版新增）

字节码里 `(类名, 方法名, 描述符)` 三元组的"类名"是**调用点的静态类型**，不一定是声明者：
`new ArrayList<>().toString()` 的 Methodref 是 `ArrayList.toString`，而 `toString` 声明在
`AbstractCollection`。历史版本出队时只在本类方法表里精确匹配，继承方法与 default 方法直接断链。

完整版按 JVMS §5.4.3.3 / §5.4.3.4 近似实现 `find_declaring`，出队目标未在本类命中时：

1. 沿父类链自下而上找 `(name, desc)` 精确声明者，优先带方法体/native 的声明；
   声明为 abstract 时继续上溯找具体实现，保留首个 abstract 声明者兜底（收集描述符类型）；
2. 类链整条找不到时，沿「父类链 + 接口」传递闭包找 default 方法（必带方法体）声明者；
3. 仍找不到时，退化为**同名 ACC_NATIVE 声明**——覆盖签名多态方法
   （`MethodHandle.invokeBasic` / `invoke` / `VarHandle.get` 等）：javac 按实际参数签名生成调用点
   描述符，与声明描述符必然不同，精确匹配永远失败；这类目标本质是 native/intrinsic 边界，
   记入 `native_stubs`。

`<init>` / `<clinit>` 不参与继承解析（构造器与静态初始化块不可继承）。
完全找不到声明者的目标记入 `unresolved` 集合并随报告输出（历史版本为静默丢弃）。

---

## 四、五条引用发现通道

信息进入最终结果集 (`refs` 或 `visited`) 的路径共 5 条：

```
通道 1：调用链展开
  invoke* 指令 → (cls, name, desc) → BFS 队列 → visited

通道 2：方法描述符类型
  任意方法（含 abstract/native）的参数类型和返回类型 → refs
  时机：匹配到目标方法时 + 用户类种子扫描时

通道 3：字段声明类型
  字段表的 descriptor 解析 → refs
  时机：首次解析该类时统一收集

通道 4：字节码中的直接类引用
  new / checkcast / instanceof / anewarray / multianewarray → new_cls 或 other_refs → refs
  getstatic / getfield 等字段指令 → 字段所属类 + 字段描述符类型 → refs
  ldc / ldc_w（CONSTANT_Class 条目）→ refs

通道 5：异常表 catch 类型
  Code 属性内异常表每条的 catch_type → refs  ← 0919 引入，完整版保留
```

---

## 五、`<clinit>` 的完整触发链路

这是最容易被遗漏的部分，详细说明触发条件：

```
触发条件（满足任一）：
  1. new 指令创建该类实例
  2. invokestatic 调用该类的静态方法（隐含类初始化）
  3. getstatic/putstatic 访问该类的静态字段

脚本的实现（完整版起与 JVM 语义对齐，历史版本仅实现条件 1 的近似）：

  - 维护独立的 `initialized` 集合（与 RTA 的 `instantiated` 分开）；
  - 三类触发点各自调用 `_init_class`：
      new 指令                → 实例化类进入 instantiated，同时初始化
      invokestatic 目标类      → 初始化
      getstatic/putstatic 属主 → 初始化（字段指令扫描时单独归类 `sfield_cls`）
  - `_init_class` 沿父类链自底向上逐类触发（JVM §5.5：初始化子类前必须先初始化父类），幂等；
  - 触发时**直接把 `(cls, '<clinit>', '()V') 入队**，而不是等该类的其它方法出队时顺带检查——
    历史版本的顺带检查存在时序缺口：若某类的全部方法出队之后该类才被实例化/初始化，
    其 `<clinit>` 永远不会再展开（two-pass 预置 instantiated 可部分缓解，单程无法自愈）。
```

> **主流程对照**：`codegen/transpile.py::_enqueue_class_init` 在 **4 个**触发点生效——`new`、`<init>` 展开、
> `invokestatic`（`m.is_static`）、`getstatic`/`putstatic`（`_drain_static_fields` → `_static_field_owner`），
> 并沿**父类链**递归入队，比上面的近似处理更完整。

HelloWorld 实际触发的 `<clinit>` 共 **54 个**（主流程口径，含 `ArrayList.<clinit>` 空数组常量、
`Pattern.<clinit>` 正则引擎注册、`HashMap$TreeNode.<clinit>` 红黑树常量等）；
完整版脚本按相同触发语义运行，实际数量以 `docs/reports/trace-HelloWorld.md` 报告为准。

---

## 六、JDK 类库的读取方式

JDK 的类库以 `.jmod` 格式存储，本质是 ZIP 文件：

```
java.base.jmod
  └── classes/
        ├── java/lang/String.class
        ├── java/util/ArrayList.class
        └── ...
```

`JdkResolver` 在初始化时扫描所有 `.jmod` 文件，建立 `类名 → jmod路径` 的索引，按需读取字节数据。所有解析结果缓存在 `shared_cache` 中，多次 BFS 共享，避免重复解析。

---

## 七、集成到程序时需要实现的模块清单

| 模块 | 职责 | 关键接口 |
|------|------|---------|
| **ClassParser** | 解析 `.class` 二进制，提取常量池/字段/方法/异常表 | `parse(bytes) → ClassInfo` |
| **DescriptorParser** | 从方法/字段描述符提取所有 `L类名;` 引用 | `extract_classes(desc) → [str]` |
| **BytecodeScanner** | 扫描字节码指令，识别 5 类引用 | `scan(bytecode, pool) → ScanResult` |
| **VtaAnalyzer** | 操作数栈模拟，推断 invokevirtual/invokeinterface 接收者类型 | `analyze(bytecode, pool) → {pc: type}` |
| **JdkResolver** | 从 jmods 目录按类名读取字节码 | `resolve(class_name) → bytes?` |
| **HierarchyIndex** | 维护类继承/接口实现关系，支持子类型查询（随解析缓存重建、查询时惰性解析） | `is_subtype(sub, sup) → bool` |
| **MethodResolver** | JVMS §5.4.3.3/§5.4.3.4 近似：沿父类链/接口闭包定位声明者，签名多态方法归 native 边界 | `find_declaring(cls, name, desc) → (cls, method)?` |
| **BfsEngine** | 核心 BFS 调度，管理队列/visited/instantiated/refs | `run(seeds) → BfsResult` |
| **ClinitTracker** | 三类触发点（new/invokestatic/getstatic·putstatic）+ 父类链递归，触发即入队 | `init_class(cls)` |
| **StatsCollector** | 按类型分类统计所有触达的类和方法 | `summarize(visited, refs) → Report` |

---

## 八、已知局限（需要后续补充）

> **2026-09-19 完整版修订**：完整版脚本已修复第 2、3、6 条；第 1 条转译主流程已修复、脚本仍未实现；
> 第 4、5 条两者都仍未处理。

| # | 局限 | 影响 | 补充方向 | 完整版脚本 | 主流程 |
|---|------|------|---------|-----------|--------|
| 1 | `invokedynamic` 跳过 | Lambda 调用链断开，字符串拼接底层类丢失 | 解析 `BootstrapMethods` 属性，追踪 `MethodHandle` | 未修复 | **已修复**（见 2.7 对照） |
| 2 | invokestatic 不触发 `<clinit>` | 部分静态工厂类的初始化路径丢失 | 见第五节 | **已修复**（三触发 + 父类链） | **已修复**（`_enqueue_class_init`） |
| 3 | 层次索引不随缓存重建 / 接口传播断链 | 热缓存（含 two-pass 第二程）下 `is_subtype` 退化为 `sub==sup`，接口分派目标丢失；间接接口实现的具体方法可能丢失 | 见 3.4 注 | **已修复**（缓存命中重建 + 惰性解析） | 不适用（实现不同） |
| 4 | 反射调用完全不追踪 | `Class.forName`/`Method.invoke` 的动态目标丢失 | 需要常量字符串传播分析（超出静态分析能力范围） | 未修复 | 否 |
| 5 | 注解处理器 | `@Override`/`@FunctionalInterface` 等元信息引用 | 解析 `RuntimeVisibleAnnotations` 属性 | 未修复 | 否 |
| 6 | 方法解析不沿父类链（历史版本） | 继承方法与 default 方法断链，且静默丢弃 | 见 3.6 | **已实现**（含签名多态 native 兜底） | **已实现**（`_enqueue_declaring_method`） |

---

## 九、与转译主流程的覆盖对照

核对日期：2026-09-19。对照对象：`codegen/transpile.py::_discover_jdk_classes_method_level`
+ `codegen/classfile.py`，即真正决定「哪些类与方法被转译」的那套实现。

### 9.1 逐条覆盖情况

| 本文条目 | 主流程实现 | 状态 |
|---|---|---|
| 2.1 常量池（Utf8/Class/String/Fieldref/Methodref/InterfaceMethodref/NameAndType/MethodHandle/MethodType/Dynamic/InvokeDynamic） | `_parse_constant_pool` 全部覆盖，Long/Double 双槽已跳过 | 完全覆盖 |
| 2.2 字段表描述符类型 | 仅字段**指令**处收集（`_add_type_refs`），未遍历 `ci.fields` 统一收集 | 部分（缺口 B） |
| 2.3 方法表：native / abstract / static 标志、描述符类型、`<init>`、`<clinit>`、Code 属性 | 全覆盖；abstract/native 无 `instrs` 自然不展开，但描述符类型仍收集 | 完全覆盖 |
| 2.4 类型一 invokevirtual / invokespecial / invokestatic / invokeinterface / invokedynamic | 全覆盖，invokedynamic 解析到实现方法并入队 | 完全覆盖 |
| 2.4 类型二 `new` | 进入 `instantiated_classes`（RTA 实例化集合） | 完全覆盖 |
| 2.4 类型三 getstatic / putstatic / getfield / putfield | 全覆盖，并额外解析字段声明类（`_static_field_owner`，JVMS §5.4.3.2） | 超越 |
| 2.4 类型四 `anewarray` / `checkcast` / `instanceof` | 覆盖 | 完全覆盖 |
| 2.4 类型四 `multianewarray` | 已解码，但多维引用数组的 `[` 前缀被守卫过滤 | 部分（缺口 C） |
| 2.4 类型五 `ldc` / `ldc_w` 加载 Class | **未覆盖** | 缺口（见 A） |
| 2.5 异常表 `catch_type` | 覆盖（`enqueue_refs` 内遍历 `exception_table`） | 完全覆盖 |
| 2.6 类继承结构（super_class / interfaces） | 覆盖，并以 `_supertypes` 做传递闭包 | 超越 |
| 2.7 BootstrapMethods | 覆盖（见 2.7 对照） | 超越 |
| 3.2 L1 Native 边界 | 覆盖：native 无字节码，展开即终止；描述符类型仍进闭包 | 完全覆盖 |
| 3.3 L2 `<clinit>` 门控 | 覆盖且更严：`new` / `invokestatic` / `getstatic` / `putstatic` 四处触发，沿父类链递归 | 超越 |
| 3.4 L3 RTA 虚调用过滤 | 覆盖：RTA + **不动点迭代**，另含根类虚目标（`Object.toString` 等）与边界虚目标两类传播 | 超越 |
| 3.5 VTA 操作数栈类型追踪 | **未实现**，以保守 RTA 替代（闭包偏大，但不漏） | 未实现（可接受） |
| 3.6 方法解析（JVMS §5.4.3.3/§5.4.3.4 近似） | `_enqueue_declaring_method`：沿父类链找最近声明者，未找到再沿父接口找 default 方法并补进 registry | 覆盖（脚本另含签名多态 native 兜底） |
| 通道 1 调用链展开 | 覆盖 | 完全覆盖 |
| 通道 2 方法描述符类型 | 覆盖（`_enqueue_desc_types`，含用户类种子方法自身描述符） | 完全覆盖 |
| 通道 3 字段声明类型 | 未覆盖 | 缺口（见 B） |
| 通道 4 字节码直接类引用 | 覆盖，缺 `ldc` 与 `multianewarray` | 部分（缺口 A / C） |
| 通道 5 异常表 catch 类型 | 覆盖 | 完全覆盖 |
| 七、模块清单 10 项 | 9 项以内联函数 / 闭包形式存在于 `transpile.py`（`_load_class`、`_enqueue_declaring_method`、`_propagate_virtual_targets`、`_enqueue_class_init`、`JdkResolver` 等）；**VtaAnalyzer 无对应物**（见 3.5 行） | 9/10 |

### 9.2 主流程有、本文工具未实现/不必关心的机制

- **内部包边界截断**：`sun/`、`jdk/`、`com/sun/`、`com/oracle/`、`java/security/` 以及 `runtime/java_runtime/vm_boundary.txt` 清单内的类，
  只生成类型占位符、方法体为 `panic!` stub，BFS 在此截断。**这是策略取舍，不是缺陷**（脚本侧的同名机制是 `bfs_internal_boundary` 分析模式）。
- **native upcall 反向边**：手写 runtime 的 `_impl.rs` 可声明「native 方法回调 Java」的目标，反向注入调用链。
- **VM 根方法清单**：`runtime/java_runtime/vm_roots.txt` 声明手写运行时直接调用的已翻译方法，作为 BFS 的额外种子。
- **用户类父类链初始化**：用户类的 JDK 父类先初始化。
- **不动点收敛**：`queue` 排空 → 传播虚调用目标 → 有新方法则继续，直到不再增长。

> JVMS §5.4.3.3 方法解析原列于此；完整版脚本已实现（见 3.6）并更新 9.1 对照行，不再是"本文未描述"。

### 9.3 主流程侧的实际缺口

| 缺口 | 位置 | 影响 | 建议改法 |
|---|---|---|---|
| **A. `ldc` 加载 Class 字面量不进闭包** | `transpile.py::_collect_method_refs` 末条分支只匹配 `comment.startswith(_JDK_PREFIXES)`，而 `_ldc_str` 产出 `class java/util/List` | `X.class` 字面量引入的类不进闭包 → 该类缺失或退化为无初始化存根。与本文 2.4 类型五同源 | 新增分支：`comment` 以 `class ` 开头时取 `comment.split()[1]`，按现有 `field_classes` / `field_discover_classes` 通道处理 |
| **B. 字段声明类型未统一收集** | `transpile.py::_discover_jdk_classes_method_level` 未遍历 `ci.fields` | 仅作类型存根、又从未被字段指令触达的类，其字段引用的 JDK 类不进闭包 → 生成代码引用不存在的类型 | 解析类时统一 `_enqueue_desc_types(f.descriptor)`（可先只对 `field_discover_classes` 内的类生效，控制闭包膨胀） |
| **C. `multianewarray` 多维引用数组元素类型被过滤** | 同上，`'[' not in c` 守卫 | `new String[2][3]` 的 `String` 不由此进闭包（通常被其它通道覆盖，故尚未暴露为故障） | 对 `multianewarray` 的注释先剥掉前导 `[` 再取类名 |
| D. `samtype:` 未使用 | `transpile.py::_collect_method_refs` 的 `InvokeDynamic` 分支只取 `impl:` | SAM 函数式接口类型不由此进闭包 | 需要时按 `_add_type_refs(sam)` 处理 |

> **注意**：A、B、C 都会**扩大**闭包（更多类进入转译范围）。改动后必须跑全量回归——
> 闭包变大可能引入新的编译错误，也可能让构建变慢。建议逐项改、逐项验。

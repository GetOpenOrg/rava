# Java → Rust 类型 1:1 对应完整化方案

> 创建日期：2026-09-17  
> 来源：两轮讨论——类型映射现状分析 + 1:1 对应目标设定  
> 关联文档：`2026-09-16-java-class-macro-unified.md`（`java_class!` 宏设计）

---

## 目录

1. [目标：生成代码与 Java 源码 1:1 对应](#1-目标)
2. [现状分析：三类类型的处理方式](#2-现状分析)
3. [三个 Gap 与对应任务](#3-三个-gap)
4. [任务详情](#4-任务详情)
   - [T-1 bounds 进宏](#t-1-bounds-进宏)
   - [T-2 接口泛型透明](#t-2-接口泛型透明)
   - [T-3 String 走生成](#t-3-string-走生成)
   - [T-4 包装类走生成（延后）](#t-4-包装类走生成延后)
5. [任务依赖关系](#5-任务依赖关系)
6. [不变项](#6-不变项)
7. [生成器→Rust 原生机制迁移（R/M 系列）](#7-生成器rust-原生机制迁移rm-系列)
   - [R-1 blanket Into&lt;Object&gt;](#r-1-blanket-intoobject)
   - [R-2 Deref&lt;Target=Parent&gt; 替换 From 继承链](#r-2-dereftargetparent-替换-from-继承链)
   - [R-3 derive(Debug)](#r-3-derivedebug-替换宏生成-debug)
   - [M-1 From&lt;Child&gt; for Parent 进宏](#m-1-fromchild-for-parent-进宏r-2-备选方案)
   - [M-3 方法签名类型决策进宏](#m-3-方法签名类型决策进宏长期)

---

## 1 目标

**生成的 Rust 代码与 Java 源码，在泛型参数、命名空间路径、字段声明、方法签名和方法体写法上完全 1:1 对应。所有封装细节（`RefCell`/`Box`/`Rc` 嵌套、borrow 窗口、Rust trait bounds）由 `java_class!` 宏和 codegen 管线统一吸收，对读代码的人不可见。**

### 1:1 对照示例

```java
// Java 源码
package java.util;
public class ArrayList<E> extends AbstractList<E>
        implements List<E> {
    Object[] elementData;
    int size;

    public boolean add(E e) {
        elementData[size] = e;
        size++;
        return true;
    }
    public E get(int index) {
        return (E) elementData[index];
    }
    public Iterator<E> iterator() { ... }
}
```

```rust
// 目标：生成的 Rust（java/util/array_list.rs）
java_class! {
    #[binary_name = "java/util/ArrayList"]

    pub struct ArrayList<E> {          // 泛型参数写法与 Java 完全一致
        elementData: Vec<E>,           // 字段名和类型与 Java 一致
        size: i32,
    }

    impl ArrayList<E> {                // 无 Rust bounds 污染

        pub fn add(&self, e: E) -> Result<bool> {
            self.elementData.push(e);  // Java 风格，宏负责 borrow 重写
            self.size += 1;
            Ok(true)
        }

        pub fn get(&self, index: i32) -> Result<E> {
            Ok(self.elementData[index as usize])
        }

        pub fn iterator(&self) -> Result<Iterator<E>> {  // 接口泛型参数可见
            ...
        }
    }
}
```

---

## 2 现状分析

### 2.1 原生类型（`int`, `long`, `boolean` 等）

| 类型 | 现状 | 评价 |
|------|------|------|
| `int` | `i32` | **正确，保留**。位宽固定、无对象身份，语义双射 |
| `long` | `i64` | 同上 |
| `boolean` | `bool` | 同上 |
| `byte/short/char/float/double` | `i8/i16/u16/f32/f64` | 同上 |

原生类型的直接映射不是"别扭的绕行"，是数学意义上的一一对应，**不需要改**。

### 2.2 包装类型（`Integer`, `Long`, `Boolean` 等）

| 类型 | 现状 | 问题 |
|------|------|------|
| `Integer` | 透明映射为 `i32` | Java `Integer` 可为 `null`；`parseInt`/`MAX_VALUE` 等缺失 |
| `Long` | 透明映射为 `i64` | 同上 |
| `Boolean` | 透明映射为 `bool` | 同上 |

**当前处理基本可用**，autoboxing/unboxing 通过 codegen 跳过 `valueOf`/`intValue` 调用透明处理。  
真实漏洞场景：泛型集合 `List<Integer>` 中的 null、`Integer.parseInt` 静态调用。  
**优先级低，列为 T-4（延后）**，等 String 生成完成后再推进。

### 2.3 引用类型的多层嵌套

| 层次 | 现状 |
|------|------|
| 方法体 | **已屏蔽**：`java_class!` 宏 token 重写，`self.size += 1` → `__set_size` 调用 |
| 字段访问器 | **已屏蔽**：`__get_xxx`/`__set_xxx` 抽象层 |
| 字段存储类型 | **未完全屏蔽**：宏展开后仍可见 `Box<RefCell<T>>` |
| impl 块 bounds | **未屏蔽**：codegen 生成 `E: Clone + Default + 'static`，出现在 `java_class!` 输入 |

### 2.4 接口类型

| 场景 | 现状 | 问题 |
|------|------|------|
| 方法参数/返回值 | `crate::java::util::Iterator`（全路径） | 泛型参数丢失：`Iterator<E>` → `crate::java::util::Iterator` |
| 字段类型 | 同上 | 同上 |
| 接口本身 | `pub type Iterator = Object;` | Rust 不允许 `type Iterator<E> = Object`（E 未使用） |

### 2.5 String

| 现状 | 问题 |
|------|------|
| `String` 映射 Rust `std::string::String` | 语义不同：Java String 不可变、UTF-16、有 `charAt`/`substring` 等方法 |
| 手写 `System`/`PrintStream` 绕过了 String 的真实方法调用 | HelloWorld 能跑，但 String 操作语义不完整 |

---

## 3 三个 Gap

| Gap | 影响 | 难度 | 阻塞什么 |
|-----|------|------|---------|
| **G-1** impl bounds 出现在 `java_class!` 输入里 | 代码可读性 | 低 | 不阻塞，纯清洁 |
| **G-2** 接口泛型参数（`List<E>`）在 `java_class!` 输入里被擦除 | 1:1 对应不完整 | 中 | 不阻塞，增强可读性 |
| **G-3** `String` 语义漏洞 | **正确性** | 高 | **阻塞完整 HelloWorld 语义** |

---

## 4 任务详情

---

### T-1 bounds 进宏

**目标**：codegen 生成的 `java_class!` 块内 `impl ArrayList<E>` 中不出现 Rust trait bounds，由宏在展开时自动注入。

#### 现状

```rust
// 现在 codegen 生成的
java_class! {
    pub struct ArrayList<E: Clone + Default + 'static> {
        ...
    }
    impl<E: Clone + Default + 'static> ArrayList<E> {
        ...
    }
}
```

#### 目标

```rust
// T-1 完成后
java_class! {
    pub struct ArrayList<E> {   // 无 bounds
        ...
    }
    impl ArrayList<E> {         // 无 bounds
        ...
    }
}
```

Rust bounds 仅在宏展开产物中出现，对读者不可见。

#### 实现方案

**宏侧（`runtime/java_rta_macros/src/block.rs`）**：
- 解析 `pub struct Foo<E>` 时记录类型参数名列表（如 `['E']`）
- 展开时为每个类型参数附加 `Clone + Default + 'static`
- 同样处理 `impl Foo<E>` —— 宏添加 bounds，生成 `impl<E: Clone + Default + 'static> Foo<E>`

**codegen 侧（`codegen/emitter/class_writer.py`）**：
- struct 声明中类型参数只写裸参数名（`<E>`），不写 bounds
- `impl` 块头部同样只写 `impl ArrayList<E>`（codegen 当前写完整 bounds 的逻辑删除）

#### 启动条件

- 无前置任务依赖
- 宏当前已能处理 struct + impl 的完整展开（`java_class!` Step 1–7 已落地）
- 可独立开始

#### 验证方式

```bash
# 运行 HelloWorld e2e，全程无编译错误
python3 scripts/main.py HelloWorld.java
# cargo expand 确认展开产物中 bounds 存在
```

---

### T-2 接口泛型透明

**目标**：`java_class!` 输入中的接口类型可以携带泛型参数（如 `List<E>`、`Iterator<E>`），宏在展开时将其擦除为 `Object`（Arch-1 语义），读代码的人看到的是 Java 风格的完整类型。

#### 现状

```rust
// 现在：接口泛型参数丢失
pub fn iterator(&self) -> Result<crate::java::util::Iterator> { ... }
// 字段类型若是接口，也只保留全路径，无泛型
```

#### 目标

```rust
// T-2 完成后：java_class! 输入里可以写
pub fn iterator(&self) -> Result<Iterator<E>> { ... }

// 字段也可以写
someField: Comparator<E>,
```

宏在展开时：
- `Iterator<E>` → 识别为接口 → 擦除为 `Object`
- 生成的字段存储和方法签名中用 `Object`

#### 实现方案

**接口感知机制**：宏需要知道哪些类型是接口。有两种选择：

**方案 A（推荐）**：codegen 对每个 `use` 引入的接口类型加 `#[is_interface]` 注解，宏维护接口名集合：

```rust
java_class! {
    #[binary_name = "java/util/ArrayList"]
    #[interfaces(List, RandomAccess, Cloneable, Serializable)]  // codegen 注入

    pub struct ArrayList<E> { ... }
    impl ArrayList<E> {
        pub fn iterator(&self) -> Result<Iterator<E>> { ... }
    }
}
```

宏收到 `#[interfaces(...)]` 后建立接口名集合，在解析字段类型和方法签名时，对集合内的类型名实施泛型擦除。

**方案 B**：通过 `use` 声明 + 特定属性标记接口类型别名（侵入性更小，但实现复杂度更高）。

**宏侧类型解析扩展**：
- 字段类型解析：遇到 `InterfaceName<Args>` → 存储类型用 `Object`，访问器类型也用 `Object`
- 方法参数/返回类型解析：同上
- 保留原始写法在注释或 `#[doc]` 属性，便于工具读取

**codegen 侧**：
- 生成 `#[interfaces(...)]` 属性（从 `ci.interfaces` 字段读取）
- 方法签名里的接口参数/返回类型按原始泛型签名写（如 `Iterator<E>`），不做提前擦除
- 不再调用 `_iface_full_path`（接口类型擦除职责转移到宏）

#### 启动条件

- **T-1 必须先完成**：宏已能接受无 bounds 的 `impl Foo<E>`，才能继续扩展类型解析范围
- codegen 的 `ci.interfaces` 字段已在 `ClassInfo` 中可用（已有）
- 接口列表生成（`#[interfaces(...)]` 属性）需 codegen 侧新增逻辑

#### 验证方式

```bash
# 运行包含接口类型参数场景的测试
python3 scripts/run_tests.py --filter ArrayList
# 检查生成代码中接口类型参数可见
grep "Iterator<E>" build/*/java_runtime/src/java/util/array_list.rs
```

---

### T-3 String 走生成

**目标**：`java.lang.String` 的 Rust 实现从 `java/lang/String.class` 字节码翻译，替换掉 `JVM_RUST` 中 `'Ljava/lang/String;' → 'String'` 的硬编码映射。生成 `String` 类型后，方法体里的 `String` 就是 `java::lang::String`，与 Java 语义一致。

#### 现状

```python
# type_map.py JVM_RUST（当前硬编码）
'Ljava/lang/String;': 'String',   # → Rust std::string::String
```

存在的语义漏洞：
- Rust `String` 是 UTF-8 可变字符串；Java `String` 是 UTF-16 不可变对象
- `charAt`、`substring`、`indexOf`、`length` 等方法目前全缺（HelloWorld 靠手写 System 绕过）
- `String.format`、`String.valueOf` 等静态方法无实现

#### 目标

```rust
// java/lang/string.rs（由字节码生成）
java_class! {
    #[binary_name = "java/lang/String"]
    pub struct String {
        value: Vec<u16>,   // Java String 的实际存储：UTF-16 char 数组
        ...
    }
    impl String {
        pub fn charAt(&self, index: i32) -> Result<u16> { ... }
        pub fn length(&self) -> Result<i32> { ... }
        pub fn substring(&self, begin: i32, end: i32) -> Result<String> { ... }
        // ... native 方法在 string_impl.rs 中实现
    }
}
```

`JVM_RUST` 中移除 `String` 硬编码，由 registry 路径自动解析为 `String`（短类名）。

#### 实现分解（子任务）

**T-3a**：BFS 收录 `java/lang/String` 完整依赖  
- `trace_callchain.py` 已能提取字段类型依赖（本 session 已修复）
- 需确认 String 的所有被调用方法都在 BFS 分析范围内

**T-3b**：`String` native 方法手写实现（`java/lang/string_impl.rs`）  
- `charAt`、`length`、`indexOf` 等常用方法：基于 `value: Vec<u16>` 实现
- `String(byte[])` 构造器：UTF-8 → UTF-16 转换
- `toString()`：返回 self clone

**T-3c**：移除 `JVM_RUST` 中 `String` 硬编码  
- 删除 `'Ljava/lang/String;': 'String'`
- 确认 registry 路径能正确把 `java/lang/String` 解析为短名 `String`

**T-3d**：`system_impl.rs` 的 `println` 适配  
- 目前 `println` 直接对 Rust `String` 调用 `.as_str()`；改为对 `java::lang::String` 调用 `.toString()` 后得到显示用字符串

**T-3e**：全量 e2e 验证

#### 启动条件

- T-3 **不依赖 T-1/T-2**，可独立启动
- 必须先确认 `java/lang/String.class` 字节码分析完整（运行 `trace_callchain.py` 看 String 是否在输出类集合里）
- `String` 的 native 方法（如 `intern`、`getBytes`）需要有占位存根（panic!），不需要全量实现，只需 HelloWorld 路径上的方法
- **这是三个任务中最高语义价值的，也是工作量最大的**

#### 验证方式

```bash
# 基本 HelloWorld 路径
python3 scripts/main.py HelloWorld.java
# 含字符串操作的测试
python3 scripts/run_tests.py --filter String
# 确认 JVM_RUST 中无 String 硬编码
grep "'String'" codegen/type_map.py  # 应无输出
```

---

### T-4 包装类走生成（延后）

**目标**：`Integer`、`Long`、`Boolean` 等包装类从 `java/lang/Integer.class` 等字节码生成，移除 `JVM_RUST` 中的透明映射。

#### 延后原因

- 当前透明映射（`Integer → i32`）在 HelloWorld 路径上无语义漏洞
- autoboxing/unboxing 需要 codegen 特判 `valueOf`/`intValue` 指令序列，工作量不小
- `T-3 String 走生成`完成后，类似的模式可以复用，T-4 的实施成本会大幅降低

#### 启动条件（未来）

- **T-3 完成后**：`String` 生成管线走通，模式可复用
- codegen 能识别 autoboxing bytecode pattern（`invokestatic Integer.valueOf` + 紧接使用）
- `Integer`/`Long` 的 native 方法（`parseInt`、`toString` 等）有手写实现

---

## 5 任务依赖关系

```
T-1（bounds 进宏）
  │
  └──→ T-2（接口泛型透明）
            │
            └──→ [可读性目标完成]

T-3（String 走生成）          ← 独立启动，不依赖 T-1/T-2
  │
  └──→ T-4（包装类走生成）    ← T-3 完成后启动
            │
            └──→ [语义正确性目标完成]
```

T-1 → T-2 是串行依赖（T-2 扩展宏的类型解析，需要 T-1 先稳定宏的基础结构）。  
T-3 与 T-1/T-2 完全独立，可并行推进。

---

## 6 不变项

以下映射在最终架构中**保持直接映射，不走生成路径**：

| Java 类型 | Rust 类型 | 原因 |
|-----------|-----------|------|
| `int` | `i32` | 语义双射，无对象身份，无 null |
| `long` | `i64` | 同上 |
| `boolean` | `bool` | 同上 |
| `byte/short/char/float/double` | `i8/i16/u16/f32/f64` | 同上 |
| `void` | `()` | 语义双射 |
| `Object` | `Rc<dyn ObjectVTable>` | 动态派发机制，从 Object.class 字节码翻译得到 |

多层嵌套（`Box<RefCell<T>>`、`Rc<RefCell<Vec<E>>>`）已通过 `java_class!` 宏的字段访问器模式屏蔽，是实现细节，不是 API 面。在 `java_class!` 块内的代码里，这些细节对读者不可见。

---

---

## 7 生成器→Rust 原生机制迁移（R/M 系列）

> 与 T 系列目标互补：T 系列减少 **可见类型标注** 的噪声，R/M 系列减少 **生成代码量** 本身，让 codegen 和宏只做 Java→Rust 信息传递，机械变换交给 Rust 类型系统自动完成。

### 边界原则

| 场景 | 应该怎么做 |
|------|----------|
| 信息已在 `java_class!` 块内可读、变换纯机械 | 进宏（M 系列） |
| 对所有生成类通用、只需满足一个 trait bound | 写 `java_runtime` blanket impl（R 系列） |
| 需要字节码解析、class 级上下文 | 留 codegen |

---

### R-1 blanket `Into<Object>`

**目标**：在 `java_runtime` 写一次 blanket impl，覆盖所有生成类的 `Into<Object>` 转换，宏内删除 per-class 生成代码。

#### 现状

宏为每个类展开约 8 行：

```rust
impl<E: Clone + Default + 'static> Into<Object> for ArrayList<E> {
    fn into(self) -> Object { Rc::new(self) }
}
```

n 个类 = n × 8 行重复代码，全部内容相同。

#### 目标

`java_runtime/src/lib.rs`（永久基础设施）：

```rust
impl<T> From<T> for Object
where
    T: ObjectVTable + Clone + Default + 'static,
{
    fn from(val: T) -> Object {
        Rc::new(val) as Rc<dyn ObjectVTable>
    }
}
```

宏内 `Into<Object>` 生成块全部删除。

#### 冲突分析

- `Object = Rc<dyn ObjectVTable>`，`Rc` 不实现 `ObjectVTable`（`ObjectVTable` 只由具体生成 struct 实现）
- std 的 `From<T> for T` blanket 不产生冲突：`Object` 不实现 `ObjectVTable`
- 结论：**无孤儿规则违反，无重叠 impl**

#### 删除量

宏中约 8 行/类 × 当前 ~100 类 = ~800 行展开代码消失；宏 `block.rs` 删除对应生成块。

#### 启动条件

- 无依赖，可立即启动
- 需确认 `ObjectVTable` 的定义位置（`java/lang/object.rs`），blanket impl 与其在同一 crate

---

### R-2 `Deref<Target=Parent>` 替换 From 继承链

**目标**：宏为每个有直接父类的生成类生成一个 `Deref` impl，通过 Rust deref coercion 自动处理多层继承链；删除 codegen 中 T55 `From<Child> for Parent` 生成循环。

#### 现状

codegen（`class_writer.py`）为每个类沿继承链生成一整条 `From` impl：

```rust
impl From<ArrayList<E>> for AbstractList<E> { ... }
impl From<ArrayList<E>> for AbstractCollection<E> { ... }
impl From<ArrayList<E>> for Iterable<E> { ... }
impl From<ArrayList<E>> for Object { ... }  // R-1 完成后此条消失
```

层数深（ArrayList 继承链 5 层）× 类数多 = 数千行纯样板代码。

#### 目标

宏根据 `#[superclass = "AbstractList"]` 属性（T-1/T-2 阶段 codegen 已写入），只生成一个 `Deref`：

```rust
impl<E: Clone + Default + 'static> Deref for ArrayList<E> {
    type Target = AbstractList<E>;
    fn deref(&self) -> &AbstractList<E> {
        // 访问 inner 中的 _super 字段
        &*self.0.borrow()._super
    }
}
```

Rust deref coercion 自动处理：
- `array_list.modCount()` → 找不到 → deref → `AbstractList.modCount()` → 找不到 → deref → ...
- 跨层方法调用无需生成逐层 From

codegen `class_writer.py` T55 循环（~40 行）整体删除。

#### 注意事项

- `Deref` 用于 immutable 路径；`DerefMut` 需要另外考虑（或走 `__set_xxx` 路径）
- 父类本身也是 `java_class!` 生成的 struct，不是 `Rc<dyn Trait>`，Deref 可直接持有值
- `_super` 字段已由 codegen 生成（继承展平方案中存在）

#### 启动条件

- **R-1 必须先完成**：`Into<Object>` 由 blanket impl 覆盖后，R-2 才能安全删掉 `From` 链中的 `→ Object` 那条
- 宏已能读取 `#[superclass = "..."]` 属性

---

### R-3 `#[derive(Debug)]` 替换宏生成 Debug

**目标**：生成的 struct 加 `#[derive(Debug)]`，`block.rs` 删除手工展开的 `fmt::Debug` 实现（约 12 行）。

#### 实现

codegen 在 `java_class!` struct 前加 `#[derive(Debug)]`（同时加 `#[derive(Clone)]` 对齐已有行为），宏不再显式生成 Debug。

#### 前提

所有 struct 字段的类型必须实现 `Debug`：
- `RefCell<T>` 中 `T: Debug` ← `T` 是生成类型，也加 `#[derive(Debug)]`，满足
- `Box<RefCell<T>>`、`Vec<T>` 均在 `T: Debug` 时满足
- `Object = Rc<dyn ObjectVTable>` — `ObjectVTable` 需要 `fn fmt_debug(...)`，或者 `Object` 实现 `Debug`（当前已有）

#### 启动条件

- 无强依赖，可独立启动
- 建议与 R-1 同批提交

---

### M-1 `From<Child> for Parent` 进宏（R-2 备选方案）

**目标**：若不做 Deref 方案，退而求其次：宏从 `#[superclass = "..."]` 属性读取直接父类名，生成 `From<Self> for DirectParent` 一跳 impl，多级链不生成。

- codegen 只写直接父类 `#[superclass = "AbstractList"]`，不再生成祖先 From 链
- 宏生成一条 `impl From<ArrayList<E>> for AbstractList<E>`
- 跨多级转型场景由调用方显式写（`let a: AbstractCollection = al.into(); let b: Object = a.into()`）

#### 与 R-2 的关系

R-2（Deref）更优雅，且能支持方法调用的自动 deref；M-1 是保守备选。优先做 R-2；若 R-2 遇到无法解决的所有权问题，回退到 M-1。

---

### M-3 方法签名类型决策进宏（长期）

**目标**：宏从 `#[descriptor("(I)Ljava/lang/Object;")]` 和 `#[generic_signature("<T:...>(I)TT;")]` 属性中自行做 JVM→Rust 类型选择，Python 只传原始签名字符串，不做类型判断。

#### 现状

`method_gen.py` 中 `_sig_param_valid` / `_param_rust_type` / `_is_perm_iface_param` 共 ~60 行：Python 侧做 JVM 泛型签名解析、接口降级决策、registry 合法性校验。

#### 目标

宏侧实现 JVM 签名解析器（`syn` 做词法分析），Python 只传 descriptor 和 generic_signature 字符串，宏自行解析并生成 Rust 类型注解。

#### 工作量评估

- 需要在 `block.rs` 中实现 JVM 泛型签名语法解析（`(TT;I)Ljava/util/List<TE;>;` 格式）
- 这是 M 系列中工作量最大的，但能彻底消除 Python 侧的类型系统知识
- **依赖 IR 结构化完成**（Rust 宏需要结构化的方法 AST，而非 RawExpr 字符串）

---

### R/M 系列依赖图

```
R-1（blanket Into<Object>）← 独立，最高收益
  │
  └──→ R-2（Deref 继承链）← 工程量最大，删代码最多
            │
            └──→ [From 链 codegen 全删]

R-3（derive Debug）← 独立，最低成本

M-1（直接父类 From 进宏）← R-2 备选，R-2 若推进则跳过
M-3（签名类型决策进宏）← 最后做，依赖 IR 结构化
```

---

## 附：本 session 已完成的相关工作

| 变更 | commit | 说明 |
|------|--------|------|
| `trace_callchain.py` 补全字段类型 BFS 依赖 | `c529835` | 字段类型描述符现在被提取入分析集 |
| 去掉 Python 生成器中 JDK 类名硬编码 | `c529835` | `invoke.py`、`method_gen.py` 改为动态接口集合 |
| 接口类型改用全路径（`crate::java::util::Iterator`） | `e7f6a91` | 避免与 Rust prelude 冲突，`_iface_full_path()` |
| 删除手写 function 存根（4 个） | `e7f6a91` | Principle 0 违规清理，待 codegen 生成替代 |
| `_validate_field_type` 修复误判 | `e6e30d5` | 全路径类型通过字段类型校验 |
| 文档 §5 接口全路径描述更新 | `e6e30d5` | `java-class-macro-unified.md` 同步 |

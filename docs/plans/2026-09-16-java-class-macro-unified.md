# `java_class!` 块级宏统一方案

> 创建日期：2026-09-16  
> 状态：**已实施**（Step 1–7 全量落地，commit 5e24e26；§16 记录实施期决策与偏差）  
> 关联问题：JField 消除、泛型保留、封装透明化  

---

## 目录

1. [背景：三个问题，同一根源](#1-背景三个问题同一根源)
2. [Java ↔ Rust 对象模型的三个根本冲突](#2-java--rust-对象模型的三个根本冲突)
3. [核心设计：块级宏](#3-核心设计块级宏)
4. [输入语法](#4-输入语法)
5. [字段类型重写规则（generic_signature 驱动）](#5-字段类型重写规则)
6. [继承字段展平（superclass_fields）](#6-继承字段展平)
7. [字段访问器生成](#7-字段访问器生成)
8. [方法体 token 重写](#8-方法体-token-重写)
9. [borrow 窗口规则](#9-borrow-窗口规则)
10. [_impl.rs 手写层的兼容性](#10-_implrs-手写层的兼容性)
11. [职责边界总表](#11-职责边界总表)
12. [实施步骤](#12-实施步骤)
13. [迁移期兼容策略](#13-迁移期兼容策略)
14. [已知风险](#14-已知风险)
15. [设计原则记录](#15-设计原则记录)
16. [`_super` 语义边界（实施期决策记录）](#16-_super-语义边界实施期决策记录)

---

## 1 背景：三个问题，同一根源

字节码转译为 Rust 过程中，有三个表面独立的问题：

| 问题 | 现状 | 目标 |
|------|------|------|
| **泛型擦除** | 字段/方法签名走 `descriptor`，`E` 被擦除为 `Object` | 走 `generic_signature`，保留 `E`、`K`、`V` 等类型变量 |
| **JField 污染** | 字段类型是 `JField<RefCell<Vec<E>>>`，与 Java 源码完全不同 | 字段声明写 `Vec<E>`，封装细节不可见 |
| **内部可变性暴露** | 方法体里充斥 `.borrow_mut()`、`.get()`、`.set()` | 方法体写 `self.size = v`，与 Java 源码 1:1 对应 |

三个问题的**同一根源**：codegen Python 侧持有全量信息（字段类型、继承关系、泛型签名），但没有把这些信息有效传递给 Rust 的类型系统和宏系统，导致封装细节被迫散落在生成代码的各处。

**统一解法**：`java_class!` 块级宏作为信息传递的桥梁。codegen 把 Python 侧知道的一切写进宏参数，宏在 Rust 编译期展开，两侧信息完全对齐，封装细节集中在宏内部，对读者不可见。

---

## 2 Java ↔ Rust 对象模型的三个根本冲突

当前所有"不优雅"都来自以下三个本质冲突：

```
冲突 1：Java this 是隐式可变引用
         Rust &self 是共享只读引用
         → 现在用 JField<Cell/RefCell> 绕过，但污染了字段类型

冲突 2：Java 泛型在运行时擦除为 Object
         原始信息保存在 generic_signature 字节码属性里
         → 现在走 descriptor（擦除后），泛型参数丢失

冲突 3：Java 继承是字段+方法的整体继承
         Rust 没有字段继承
         → 现在每个子类重复父类字段，或嵌套结构体
```

任何完整方案必须同时解决这三个冲突。

---

## 3 核心设计：块级宏

### 为什么用块级宏而非属性宏

属性宏（`#[java_class]`）有一个根本限制：**proc-macro 无法跨 item 边界访问信息**。`#[java_method]` 在某个函数上无法看到同一个 struct 的字段定义，因此无法判断 `self.size` 里的 `size` 是字段还是局部变量，无法做方法体内的字段访问重写。

块级宏（`java_class! { struct ... impl ... }`）同时持有 struct 字段定义和 impl 块中所有方法体，从而在同一个宏调用里完成全部展开，绕开跨 item 限制。

### 信息流向

```
Python codegen（全量信息）
  │
  │  写入宏参数：
  │  - binary_name
  │  - generic_signature（类级别）
  │  - superclass_fields（展平后的祖先字段列表）
  │  - 字段声明（裸类型，由 generic_sig 驱动）+ 每字段 field_sig
  │  - 方法体（字节码翻译结果，Java 风格）
  │
  ▼
java_class! { ... }         ← Rust 编译期展开
  │
  ├── Inner struct 生成（flat layout）
  ├── RefCell<Inner> newtype 包装
  ├── 字段访问器生成（borrow 窗口最小化）
  ├── 字段类型重写（field_sig / generic_sig → 裸 Rust 类型）
  ├── 方法签名泛型重写（Object → E）
  ├── 方法体 token 重写（self.field → 访问器调用）
  ├── native 方法空存根生成
  └── JavaObject / vtable / Upcast impl 生成
```

---

## 4 输入语法

codegen 生成的 `.rs` 文件，读者看到的形式：

```rust
java_class! {
    // ── 类级别属性 ──────────────────────────────────────
    #[binary_name = "java/util/ArrayList"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;"]
    #[superclass = "AbstractList<E>"]

    // ── 继承字段展平（由 codegen Python 展平整条继承链）──
    // 格式：父类字段在前，按 JVM 内存布局顺序
    #[superclass_fields(modCount: i32)]

    // ── struct 声明（字段用 generic_sig 驱动的裸类型）──
    pub struct ArrayList<E> {
        #[field_sig = "[TE;"]          // 字段级 generic_signature
        elementData: Vec<E>,

        #[field_sig = "I"]
        size: i32,
    }

    // ── impl 块 ──────────────────────────────────────────
    // 泛型参数的 bound 是纯粹的 Rust 能力声明，故意不引入 `JavaType` 这类游离于
    // Java 命名空间之外的 trait 名（CLAUDE.md 命名原则）
    impl<E: Clone + Default + 'static> ArrayList<E> {

        #[descriptor        = "(Ljava/lang/Object;)Z"]
        #[generic_signature = "(TE;)Z"]
        pub fn add(&self, e: E) -> Result<bool> {
            // 写 Java 风格，宏负责重写
            self.elementData.push(e);
            self.size = self.size + 1;
            Ok(true)
        }

        #[descriptor        = "(I)Ljava/lang/Object;"]
        #[generic_signature = "(I)TE;"]
        pub fn get(&self, index: i32) -> Result<E> {
            Ok(self.elementData[index as usize].clone())
        }

        // native 存根（宏生成 unimplemented! 占位，_impl.rs 提供真实实现）
        #[descriptor        = "([Ljava/lang/Object;Ljava/util/Comparator;)V"]
        #[native]
        pub fn sort(&self, c: Object) -> Result<()>;
    }
}
```

### 属性语义说明

| 属性 | 位置 | 来源 | 含义 |
|------|------|------|------|
| `binary_name` | 类 | codegen | JVM 二进制类名，用于 vtable 注册 |
| `generic_signature`（类级别） | 类 | 字节码 Signature 属性 | 类型参数声明，用于提取类型变量映射 |
| `superclass` | 类 | codegen | 直接父类名，用于 Upcast impl 生成 |
| `superclass_fields` | 类 | codegen 展平 | 祖先字段列表，已按 JVM 顺序排列 |
| `field_sig` | 字段 | 字节码字段 Signature 属性 | 字段的原始泛型签名，覆盖 descriptor |
| `descriptor` | 方法 | 字节码方法 descriptor | 擦除后的方法签名 |
| `generic_signature`（方法级别） | 方法 | 字节码方法 Signature 属性 | 方法的原始泛型签名，用于签名重写 |
| `native` | 方法 | ACC_NATIVE flag | 标记 native 方法，宏生成 unimplemented! 存根 |

---

## 5 字段类型重写规则

宏解析每个字段上的 `field_sig` 属性（优先），类型变量从类级别 `generic_signature` 中提取。

### 解析规则表

| JVM generic signature | 解析结果 | 说明 |
|-----------------------|---------|------|
| `TE;` | `E` | 类型变量，直接保留 |
| `TK;` | `K` | 类型变量，直接保留 |
| `I` | `i32` | 基本类型 |
| `J` | `i64` | 基本类型 |
| `Z` | `bool` | 基本类型 |
| `D` | `f64` | 基本类型 |
| `F` | `f32` | 基本类型 |
| `[TE;` | `Vec<E>` | 数组 + 类型变量 |
| `[I` | `Vec<i32>` | 基本类型数组 |
| `Ljava/lang/String;` | `String` | 具体类（直接对应） |
| `Ljava/util/ArrayList<TE;>;` | `ArrayList<E>` | 具体类，保留泛型参数 |
| `Ljava/util/List<TE;>;` | `Object` | **接口**：泛型参数擦除（Arch-1 语义） |
| `Ljava/util/Comparator<TE;>;` | `Object` | 接口，擦除 |

### 接口擦除规则

接口的泛型参数在 Rust 层擦除为 `Object`（通过 vtable 分派），这是 Arch-1 的正确语义：接口引用在运行时只保证 vtable 派发，不保证具体类型。具体类（class）的泛型参数保留。

区分具体类和接口：codegen 传入 `#[is_interface]` 显式标记，宏不依赖推断。

### 优先级

```
信息来源优先级：field_sig / generic_signature > descriptor > 默认推断
```

---

## 6 继承字段展平

> **实施期修订（见 §16）**：本节描述「展平」的**责任分工与顺序约定**，这两点成立且已实现。
> 但展平产物是**转发访问器**，不是子类 Inner 里的副本字段——父类字段的实际存储唯一地留在
> `_super` 里。原因见 §16.2：复制字段会立刻产生两份分叉的状态。

### 责任分工

**codegen Python 负责展平**，宏侧零 registry 依赖。原因：

- Python 侧已有完整 `ClassInfo` registry，继承链在生成子类时完全可见
- Rust 宏无法跨 crate 读取其他类型的字段定义
- 展平逻辑在 Python 里实现比在宏里实现简单一个数量级

### 展平算法（Python 伪代码）

```python
def collect_superclass_fields(class_info: ClassInfo,
                               registry: ClassRegistry) -> list[FieldInfo]:
    """
    按 JVM 内存布局顺序收集整条继承链的实例字段。
    父类字段在前，子类字段在后。Object 本身无用户字段，跳过。
    """
    chain = []
    cursor = class_info.superclass
    while cursor and cursor != "java/lang/Object":
        parent = registry.get(cursor)
        if parent:
            chain.append(parent)
            cursor = parent.superclass
        else:
            break

    # 从顶向下：最远祖先的字段最先
    result = []
    for ancestor in reversed(chain):
        result.extend(ancestor.instance_fields)  # 仅实例字段，不含 static

    return result
```

### JVM 内存布局一致性

父字段在前的顺序与 HotSpot 的对象内存布局一致。对两个场景有实际意义：

- **JNI 互操作**：通过字段偏移量访问字段时，偏移量必须与 JVM 一致
- **序列化/反序列化**：对象的二进制表示依赖字段顺序

codegen 在生成 `#[superclass_fields(...)]` 时必须保证此顺序。

---

## 7 字段访问器生成

宏为每个字段（含继承字段）生成访问器，可见性为 `pub(crate)`，供 `_impl.rs` 手写层使用。

公开 struct 是 newtype：`pub struct ArrayList<E: Clone + Default + 'static>(ArrayList__inner<E>)`，内部用 `self.0` 访问。

### 基本类型（`Copy` 类型：`i32`、`i64`、`bool`、`f64` 等）

```rust
// 字段：size: i32
#[inline]
pub(crate) fn __get_size(&self) -> i32 {
    self.0.borrow().size
    // Ref 在表达式求值后立即 drop，borrow 窗口最小
}

#[inline]
pub(crate) fn __set_size(&self, v: i32) {
    self.0.borrow_mut().size = v;
    // RefMut 在语句结束后立即 drop
}
```

### 引用类型（`Vec<E>`、`Object`、`String` 等）

```rust
// 字段：elementData: Vec<E>

// 只读借用
#[inline]
pub(crate) fn __borrow_elementData(&self) -> ::std::cell::Ref<Vec<E>> {
    ::std::cell::Ref::map(self.0.borrow(), |i| &i.elementData)
}

// 可变借用（大多数情况下使用此版本）
#[inline]
pub(crate) fn __borrow_mut_elementData(&self) -> ::std::cell::RefMut<Vec<E>> {
    ::std::cell::RefMut::map(self.0.borrow_mut(), |i| &mut i.elementData)
}

// 整体替换（对应 PUTFIELD 写整个字段）
#[inline]
pub(crate) fn __set_elementData(&self, v: Vec<E>) {
    self.0.borrow_mut().elementData = v;
}
```

继承字段（如 `modCount`）也生成同样的访问器，命名规则一致。

---

## 8 方法体 token 重写

宏在已知字段集合的前提下，对 impl 块内所有方法体做 token 树遍历，识别字段访问模式并重写。

### 字段集合的建立

宏在处理 struct 声明时，收集所有字段名（含 `superclass_fields` 里的继承字段）并按类型分类：

```
field_set = {
    "modCount":    BasicType(i32),      // 继承字段
    "elementData": RefType(Vec<E>),     // 自有字段
    "size":        BasicType(i32),      // 自有字段
}
```

宏在方法体重写时以此集合为依据，只重写集合内的字段访问，不触碰局部变量。

### 重写规则（7 种模式）

**模式 1：引用类型方法调用（最常见）**

```rust
// 原始（Java 风格）：
self.elementData.push(e)

// 重写为（block-scoped borrow，见第 9 节）：
{ self.0.borrow_mut().elementData.push(e) }
```

**模式 2：基本类型读取**

```rust
// 原始：
self.size

// 重写为：
self.__get_size()
```

**模式 3：基本类型赋值**

```rust
// 原始：
self.size = self.size + 1

// 重写为（先求值右侧，再赋值，避免 borrow 冲突）：
self.__set_size(self.__get_size() + 1)
```

**模式 4：基本类型复合赋值**

```rust
// 原始：
self.size += 1

// 重写为：
self.__set_size(self.__get_size() + 1)
```

**模式 5：引用类型整体赋值（较少见）**

```rust
// 原始：
self.elementData = new_vec

// 重写为：
self.__set_elementData(new_vec)
```

**模式 6：引用类型下标访问（只读）**

```rust
// 原始：
self.elementData[i]

// 重写为：
{ self.0.borrow().elementData[i] }
```

**模式 7：引用类型下标赋值**

```rust
// 原始：
self.elementData[i] = val

// 重写为：
{ self.0.borrow_mut().elementData[i] = val }
```

---

## 9 borrow 窗口规则

### 核心原则

**每个字段访问的 borrow 窗口必须是最小的**，即 `Ref`/`RefMut` 在产生后尽快 drop，不跨越其他字段的 borrow 操作。违反此原则会导致运行时 `BorrowError`。

### 实现方式

宏对所有引用类型字段访问，展开为 block-scoped 形式（`{ ... }`），使 `Ref`/`RefMut` 的生命周期局限在 block 内：

```rust
// ❌ 错误展开（两个 borrow 重叠）：
let x = self.0.borrow_mut().elementData.len();  // RefMut 持续到语句结束
let y = self.__get_size();                      // 再借 borrow → panic

// ✅ 正确展开（block-scoped）：
let x = { self.0.borrow_mut().elementData.len() };  // RefMut 在 } 处 drop
let y = self.__get_size();                           // 新借，无冲突
```

### 为什么一律走 `borrow_mut()`

宏在 token 树遍历阶段无法做类型推断，不知道 `Vec::push` 需要 `&mut self` 而 `Vec::len` 只需要 `&self`。因此对引用类型的方法调用一律走 `borrow_mut()`。

在单线程 `RefCell` 场景下，只读方法调用获取可变借用没有副作用。

### `#[readonly]` 精确优化（可选）

如果需要精确区分（性能敏感路径），可在字段访问处添加显式 hint：

```rust
// codegen 确认某处只读时，可显式标记：
#[readonly] self.elementData.len()
// 宏看到 #[readonly] 则走 __borrow_elementData() 而非 __borrow_mut_elementData()
```

默认不加 `#[readonly]`，一律走 `borrow_mut()`；需要时由 codegen 按访问语义显式标注。

---

## 10 `_impl.rs` 手写层的兼容性

### 结构关系

```
java/util/
  array_list.rs        ← java_class! 宏生成（codegen 输出）
  array_list_impl.rs   ← 手写 native 实现
  mod.rs               ← pub mod array_list; mod array_list_impl;
```

### 手写层使用访问器

`java_class!` 宏生成的代码在同一 crate 内，访问器可见性为 `pub(crate)`。`_impl.rs` 里的 impl 块在宏外部，无法享受 token 重写，但可以直接调用访问器：

```rust
// array_list_impl.rs — 手写层，用访问器写，不伪装 Java 风格
impl<E: Clone + Default + 'static> ArrayList<E> {
    pub fn sort(&self, c: Object) -> Result<()> {
        // 用访问器访问字段——手写者知道自己在写 Rust，这是合理要求
        let mut data = self.__borrow_mut_elementData();
        // ... 排序实现 ...
        Ok(())
    }
}
```

### 边界原则

- **生成代码（`array_list.rs`）**：在 `java_class!` 宏内，宏保证 Java 风格，可直接与 Java 源码对照
- **手写代码（`array_list_impl.rs`）**：在宏外，使用访问器，Rust 风格，对手写开发者是合理要求

这个边界是清晰且稳定的。

---

## 11 职责边界总表

| 职责 | 承担者 | 依据 |
|------|--------|------|
| 继承链字段展平（superclass_fields） | codegen Python | 已有完整 ClassInfo registry，宏无法跨 crate 读取 |
| 字段类型重写（field_sig / generic_sig → 裸 Rust 类型） | `java_class!` 宏 | 宏看得到字段声明和 field_sig 属性 |
| Inner struct + RefCell 包装生成 | `java_class!` 宏 | 宏看得到完整 struct 定义 |
| 字段访问器生成（borrow 窗口最小化） | `java_class!` 宏 | 宏已知字段集合和类型分类，生成 get/set/borrow/borrow_mut 四类 |
| 方法签名泛型重写（Object → E） | `java_class!` 宏 | 宏同时看到字段和 impl 块 |
| 方法体字段访问 token 重写（7 种模式） | `java_class!` 宏 | 块级宏同时拥有字段集合和所有方法体 |
| native 方法 unimplemented! 存根生成 | `java_class!` 宏 | 识别 `#[native]` 属性，生成空实现占位 |
| JavaObject / vtable / Upcast impl 生成 | `java_class!` 宏 | 已有逻辑，从 `#[java_class]` 迁移进来 |
| native 真实实现 | `_impl.rs` 手写 | 使用 `pub(crate)` 访问器访问字段 |
| 字节码翻译逻辑（方法体指令翻译） | codegen Python | 不变，继续走 ILOAD/ALOAD/INVOKEVIRTUAL 等翻译流程 |

---

## 12 实施步骤

```
Step 1  定义输入语法，手写 ArrayList 验证
        ├── 确定所有属性格式（binary_name / generic_signature /
        │   superclass_fields / field_sig / descriptor / native）
        ├── 手写展开结果（不写宏，直接写目标 Rust 代码）
        └── cargo check 通过 → 确认目标形态可编译

Step 2  实现 struct 层展开
        ├── 解析 superclass_fields，生成 Inner struct（父字段在前）
        ├── 生成 RefCell<Inner> newtype 包装（self.0 语义）
        ├── 解析 field_sig，重写字段类型（TE; → E，[TE; → Vec<E>）
        └── 生成字段访问器：
              基本类型 → __get_xxx / __set_xxx
              引用类型 → __borrow_xxx / __borrow_mut_xxx / __set_xxx

Step 3  实现方法签名重写
        ├── 解析方法 generic_signature 属性
        ├── 识别类型变量（TE; → E，TK; → K）
        ├── 应用接口擦除规则（接口泛型参数 → Object）
        └── 重写方法参数类型和返回类型

        ── ArrayList 单测（struct 层 + 签名层）通过后继续 ──

Step 4  实现方法体 token 重写
        ├── 建立字段集合（含继承字段，按名称和类型分类）
        ├── token 树遍历，识别 self.field 访问模式（7 种模式）
        ├── 按规则展开为访问器调用（borrow 窗口 block-scoped 最小化）
        └── 边缘情况处理（if let、match arm、嵌套表达式）

Step 5  实现 JavaObject / vtable / Upcast / native 存根生成
        ├── 迁移现有 #[java_class] 逻辑（all_supertypes、is_instance_of）
        ├── toString / hashCode 条件转发迁移
        └── #[native] 方法生成 unimplemented!("native: ...") 存根

        ── Step 4-5 可在 Step 2-3 稳定后并行开发 ──

Step 6  修改 codegen Python
        ├── 输出 java_class! { ... } 块而非 #[java_class] struct
        ├── 字段加 field_sig 属性，类型直接输出 generic_sig 驱动的裸类型
        ├── superclass_fields 由 Python 展平整条继承链后传入
        ├── native 方法生成 #[native] 属性的空签名（无方法体）
        └── 方法体继续走字节码翻译逻辑（Java 风格输出，宏做 token 重写）

Step 7  删除 JField<T>
        └── e2e 测试全绿后执行，作为独立 commit
```

---

## 13 迁移期兼容策略

`java_class!` 块级宏和现有 `#[java_class]` 属性宏可以共存：

- Step 1-5 期间：新宏在独立测试 crate 验证，不影响现有生成代码
- Step 6 期间：codegen 切换输出格式，`#[java_class]` 属性宏暂时保留
- Step 7 之后：属性宏废弃，仅保留块级宏

两个宏在同一个 `java_rta_macros` crate 里维护，不需要额外依赖。

---

## 14 已知风险

| 风险 | 等级 | 说明 | 缓解策略 |
|------|------|------|---------|
| Step 4 token 重写的边缘情况 | 高 | Rust 表达式树复杂，`if let`、`match`、`&mut self.field` 引用传递等场景需逐一处理 | Step 4 独立实施，先覆盖 7 种核心模式，边缘情况允许 fallback 到手写访问器调用 |
| 接口/具体类判断准确性 | 中 | 字段类型是接口时泛型参数需要擦除，判断错误会导致类型不匹配 | codegen 传入 `#[is_interface]` 显式标记，宏不依赖推断 |
| borrow 窗口遗漏 | 中 | token 重写生成的 block 边界不正确时，可能产生运行时 borrow panic（不是编译错误） | 建立 borrow panic 测试集，覆盖同对象多字段同时访问的场景 |
| JVM 字段顺序与 codegen 不一致 | 低 | superclass_fields 展平顺序错误会导致 JNI 互操作字段偏移量错误 | codegen 展平后写入测试，与 `javap -verbose` 输出对比验证 |
| 宏编译时间增加 | 低 | 块级宏展开复杂，可能增加 `cargo build` 耗时 | 基准测试，必要时拆分为多个独立宏减少单次展开规模 |

---

## 15 设计原则记录

这次讨论的演进路径本身值得记录：从"如何去掉 JField"到"泛型如何保留"到"如何让宏做封装"，三个问题最终收敛到同一个机制（`java_class!` 块级宏），而非分别解决。

**根本原因**：codegen Python 侧已经拥有所有需要的信息（字段类型、泛型签名、继承关系），但这些信息没有有效传递给 Rust 的类型系统。块级宏是这个"信息传递桥梁"——codegen 把 Python 侧知道的一切写进宏参数，宏在 Rust 编译期展开，两侧信息完全对齐。

**推论**：遇到生成代码"看起来不像 Java"的地方，首先问"Python 侧是否已有这个信息"，如果有，解法是把它传给宏，而不是在生成代码里用替代方案。

---

## 16 `_super` 语义边界（实施期决策记录）

> 实施日期：2026-09-16
> 状态：已实现并验证（`java_class!` 宏 + codegen 全链路切换完成）

### 16.1 决策来源

继承模型选 **Option C：展平字段 + 保留 `_super` 仅供分派**。理由是字段访问（读写、token 重写）走统一路径，Upcast / `instanceof` 与字段布局解耦。

### 16.2 实施时发现的硬约束

字面形态的 Option C 不可实现。两条候选路径都走不通：

**（a）「展平字段 + `_super` 降级为 PhantomData」→ 同一字段两份状态。**

`_super: Parent` 内部的父类字段，和子类 Inner 里展平出来的父类字段，是两块独立存储。于是

```rust
obj.__super().__get_modCount()   // 读的是 _super 里那份
obj.__get_modCount()             // 读的是展平出来那份
```

第一次写入就分叉，之后永远不一致。这是静默错误，比编译错误危险得多。

**（b）「展平字段 + unsafe 前缀转换做 upcast」→ 内存布局不成立。**

`Parent` 是 newtype `Parent(Parent__inner)`，其首字段是 `RefCell<...>`；子类 Inner 展平后首字段是父类第一个字段。两者前缀类型根本不同，`&*(self as *const _ as *const Parent)` 既不安全也不正确。`RefCell` 没有 `repr(transparent)`，加 `#[repr(C)]` 也救不回来。

### 16.3 实际落地的语义

**`_super` 是父类状态的唯一所有者，是 Inner struct 里的真实字段。「展平字段」是视图，不是存储。**

子类不再复制父类字段，而是由宏为每个祖先字段生成一个转发访问器：

```rust
pub struct ArrayList__inner<E> {
    // 父类状态的唯一所有者；ArrayList__inner 里没有 modCount
    pub(crate) _super: AbstractList<E>,
    pub(crate) elementData: Box<RefCell<Rc<RefCell<Vec<Object>>>>>,
    pub(crate) size: Cell<i32>,
}

// 转发访问器（内部字段 vs 继承字段，对外签名完全一致）
pub fn __get_modCount(&self) -> i32          { self.0._super.__get_modCount() }
pub fn __set_modCount(&self, v: i32)         { self.0._super.__set_modCount(v); }
pub fn __get_size(&self) -> i32              { self.0.size.get() }
pub fn __set_size(&self, v: i32)             { self.0.size.set(v); }
```

转发访问器把「深度差异」抹平成「签名差异为零」，这就是 Option C 想要的统一视图——只是统一发生在**访问器层**，而不是内存布局层。

### 16.4 边界规则（三条，均为强制）

| # | 位置 | 允许的写法 | 禁止的写法 |
|---|------|-----------|-----------|
| 1 | `java_class! { impl ... }` 内的生成方法体 | `self.field`（读写、下标、方法调用） | 任何形式的 `self._super` |
| 2 | 超类方法派发 | `self.__super().method(args)` | `self._super.method(args)` |
| 3 | 宏外代码（`_impl.rs` / `_ext.rs`） | `__get_xxx()` / `__set_xxx()` / `__borrow_xxx()` / `__borrow_mut_xxx()` | `self.0.<field>` / `self._super` |

规则 1 是「展平视图」的全部意义：方法体里 `self.modCount`（继承字段）与 `self.size`（自有字段）写法完全一致，宏在重写阶段决定该调哪个访问器，写代码的人不需要知道字段定义在哪一级。

宏对外只暴露三个合法入口：`__super()` / `__into_super()` / `__new_with_super()`。

### 16.5 codegen 侧的连带简化

转发访问器抹平了深度差异之后，codegen 不再需要按接收者静态类型拼 `_super._super.` 路径：

- 删除 `_find_field_super_prefix` / `_find_field_super_prefix_for_type`
- `getfield` → `{recv}.__get_{name}()`，`putfield` → `{recv}.__set_{name}(v)`
- invoke 侧的前缀翻译收敛到一处：`_super_prefix_to_expr(recv, "_super._super.")` → `recv.__super().__super()`

「按接收者静态类型拼路径」是 T76 方案里最脆弱的一环（正确性依赖类型推断），现在整体消失。

### 16.6 与文档前面章节的两处偏差

| 章节 | 文档写法 | 实际实现 | 原因 |
|------|---------|---------|------|
| §7 | `self.0.borrow().size`（单一 `RefCell<Inner>`） | 每字段独立 `Cell`/`RefCell` | 单一 `RefCell<Inner>` 下，同一条语句里访问两个字段必然 panic（`{ self.0.borrow_mut().a.push(x) } + self.b.size` 第二个借用失败）。§9 想用 block 缩小窗口，但窗口无法缩到「同一表达式的两个子表达式之间」 |
| §7 | 引用类型字段 `RefCell<T>` | `Box<RefCell<T>>` | `Throwable.cause`、`Class.classData` 这类自引用/互引用字段在无间接层时触发 `E0072: recursive type has infinite size`。`Box` 提供间接层，同时保持「字段槽可变 + 对象在堆上」的 Java 语义（与旧 `JField<T> = Box<RefCell<T>>` 一致） |

### 16.7 `#[repr(C)]` 的决定

**不加。** JVM 内存布局一致性只在「按偏移量访问字段」时才有意义，当前架构没有 JNI 字段偏移的读取方；引入 `Cell`/`RefCell`/`Box` 包装后，`#[repr(C)]` 也无法复现 HotSpot 布局。

字段顺序仍按 JVM 顺序排列（§6），一旦出现真实的 JNI 偏移需求，正确的做法是生成一张偏移表（`field_name → offset`），而不是依赖 `#[repr(C)]`。

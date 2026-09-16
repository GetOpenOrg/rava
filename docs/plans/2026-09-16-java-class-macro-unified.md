# `java_class!` 块级宏统一方案

> 创建日期：2026-09-16  
> 状态：设计定稿，待实施  
> 关联问题：JField 消除、泛型保留、封装透明化  

---

## 背景：三个独立讨论收敛到同一根源

以下三个独立问题看起来各不相关，但最终方案统一在同一机制里：

| 讨论主题 | 具体问题 | 层次 |
|---------|---------|------|
| 泛型保留 | 方法参数/返回值/字段类型被擦除为 `Object`，无法与 Java 源码 1:1 对应 | 签名层 |
| JField 消除 | 字段类型 `JField<Rc<RefCell<Vec<E>>>>` 污染 struct 定义，与 Java 完全不同 | 存储层 |
| 封装透明化 | `Rc<RefCell<>>` 包装细节暴露在生成代码里，读者需要了解 Rust 内存模型才能理解 | 表达层 |

三个问题的共同根源：**codegen Python 侧拥有全量信息（字段类型、泛型签名、继承关系），但没有有效机制把这些信息传递给 Rust 的类型系统，导致在生成代码里用粗糙的替代方案（JField、Object 擦除、显式 RefCell）填补这个信息缺口。**

---

## Java ↔ Rust 对象模型的三个根本冲突

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

## 核心方案：`java_class!` 块级宏

### 设计思路

将 `#[java_class]` 属性宏升级为 `java_class! { ... }` 函数式宏（接受 token 树），把 struct 定义和 impl 块一起传入。宏同时拥有字段信息和方法体，从根本上绕开 proc-macro 跨 item 的上下文限制。

**信息流向**：
```
Python codegen（全量信息）
    → 宏参数（superclass_fields、generic_signature、binary_name 等）
        → Rust 编译期展开（生成真实内存布局、访问器、impl）
```

### 用户/codegen 书写层（`.rs` 文件内容）

```rust
java_class! {
    #[binary_name = "java/util/ArrayList"]
    #[generic_signature = "<E:Ljava/lang/Object;>Ljava/util/AbstractList<TE;>;"]
    #[superclass_fields(modCount: i32)]   // codegen 展平的祖先字段，父类字段在前
    pub struct ArrayList<E> {
        elementData: Vec<E>,   // generic_sig: [TE; → Vec<E>，不是 Object[]
        size: i32,
    }

    impl<E: JavaType> ArrayList<E> {
        #[descriptor = "(Ljava/lang/Object;)Z"]
        #[generic_signature = "(TE;)Z"]
        pub fn add(&self, e: E) -> Result<bool> {
            // Java 风格：直接字段操作
            self.elementData.push(e);
            self.size = self.size + 1;
            Ok(true)
        }

        #[descriptor = "(I)Ljava/lang/Object;"]
        #[generic_signature = "(I)TE;"]
        pub fn get(&self, index: i32) -> Result<E> {
            Ok(self.elementData[index as usize].clone())
        }
    }
}
```

### 宏展开产物（`cargo expand` 可见，正常阅读不可见）

**产物 1：真实内存布局**

```rust
// 继承字段 flat layout：父类字段在前，与 JVM 内存布局一致（JNI 互操作依赖）
struct ArrayList__inner<E: JavaType> {
    modCount: i32,        // ← 来自 superclass_fields（AbstractList）
    elementData: Vec<E>,  // ← ArrayList 自有字段
    size: i32,
}

pub struct ArrayList<E: JavaType>(::std::cell::RefCell<ArrayList__inner<E>>);
```

**产物 2：字段访问器（borrow 窗口最小化）**

```rust
impl<E: JavaType> ArrayList<E> {
    // 引用类型（Vec、Object 等）：返回 Ref / RefMut
    #[inline]
    fn __field_elementData(&self) -> ::std::cell::Ref<Vec<E>> {
        ::std::cell::Ref::map(self.0.borrow(), |i| &i.elementData)
    }
    #[inline]
    fn __field_elementData_mut(&self) -> ::std::cell::RefMut<Vec<E>> {
        ::std::cell::RefMut::map(self.0.borrow_mut(), |i| &mut i.elementData)
    }

    // 基本类型（i32/i64/bool 等 Copy 类型）：直接取值，borrow 立即释放
    #[inline] fn __get_size(&self) -> i32       { self.0.borrow().size }
    #[inline] fn __set_size(&self, v: i32)      { self.0.borrow_mut().size = v; }
    #[inline] fn __get_modCount(&self) -> i32   { self.0.borrow().modCount }
    #[inline] fn __set_modCount(&self, v: i32)  { self.0.borrow_mut().modCount = v; }
}
```

**产物 3：方法体 token 重写结果**

宏对已知字段集合 `{elementData: Vec<E>, size: i32, modCount: i32}` 做 token 遍历，重写三种访问模式：

| 源码写法（Java 风格）| 重写为（Rust 正确语义）| 说明 |
|--------------------|-----------------------|------|
| `self.field.method(args)` | `{ let mut r = self.__field_xxx_mut(); r.method(args) }` | 引用类型方法调用，borrow 窗口 block-scoped |
| `self.field = expr` | `self.__set_xxx(expr)` | 基本类型赋值 |
| `self.field op= expr` | `self.__set_xxx(self.__get_xxx() op expr)` | 基本类型复合赋值 |
| `self.field`（只读） | `self.__get_xxx()` | 基本类型读取，copy |

重写后 `add` 方法实际展开为：
```rust
pub fn add(&self, e: E) -> Result<bool> {
    { let mut r = self.__field_elementData_mut(); r.push(e); }
    self.__set_size(self.__get_size() + 1);
    Ok(true)
}
```

**产物 4：JavaObject / vtable / Upcast impl**（与现有 `#[java_class]` 逻辑一致，迁移进来）

```rust
impl<E: JavaType> ObjectVTable for ArrayList<E> { ... }
impl<E: JavaType> Into<Object> for ArrayList<E> { ... }
impl<E: JavaType> From<Object> for ArrayList<E> { ... }
impl<E: JavaType> Debug for ArrayList<E> { ... }
```

---

## 关键设计决策

### borrow 窗口规则

**所有字段访问均使用最小 borrow 窗口**。规则：

- 基本类型（`i32`/`i64`/`bool`/`f32`/`f64`/`u16`/`i8`）：直接 copy，`borrow()` 在表达式结束后立即释放
- 引用类型：用 block 包裹，`RefMut` 在 block 结束时 drop

此规则保证同一方法体内对同一对象的多个字段访问不会 double borrow panic：

```rust
// 安全：每次 borrow 独立，不交叠
let size = { self.0.borrow().size };         // borrow 释放
let val  = { self.0.borrow().elementData[i as usize].clone() }; // borrow 释放
{ self.0.borrow_mut().elementData.push(val) };  // borrow 释放
```

### superclass_fields 展平规则

codegen Python 侧在生成子类时，沿继承链从顶层祖先到直接父类顺序收集所有字段，展平为单层列表传入宏：

```
Object（无字段）
  └── AbstractCollection（无字段）
        └── AbstractList（modCount: i32）
              └── ArrayList（elementData: Vec<E>, size: i32）

superclass_fields = (modCount: i32)   ← 所有祖先字段，按继承顺序
```

宏展开 `__inner` 时父类字段在前，与 JVM 堆布局一致，支持未来 JNI unsafe 内存访问。

### generic_signature 优先级

```
信息来源优先级：generic_signature > descriptor > 默认推断

字段类型解析：
  [TE;              → Vec<E>
  TE;               → E
  Ljava/lang/String; 且 generic_sig 缺失 → Object（擦除语义）

方法签名解析：
  (TE;)Z            → fn(e: E) -> Result<bool>
  (Ljava/lang/Object;)Z 且 generic_sig 缺失 → fn(e: Object) -> Result<bool>

类型变量映射：
  TE; / TK; / TV;   → 当前类的类型参数 E/K/V（直接使用）
  接口泛型（Arch-1）→ Object（擦除，不保留）
  具体类泛型        → 保留，ArrayList<E>、HashMap<K,V> 等
```

---

## 职责边界（最终版）

| 职责 | 承担者 | 原因 |
|------|--------|------|
| 父类字段展平（superclass_fields） | codegen Python | 已维护完整 ClassInfo registry，展平逻辑在 Python 更简单 |
| generic_signature 解析 → 裸 Rust 类型 | `java_class!` 宏 | 宏看得到字段声明和属性，解析发生在编译期 |
| Inner struct + RefCell 包装生成 | `java_class!` 宏 | 宏看得到完整 struct 定义 |
| 字段访问器生成（borrow 窗口最小化） | `java_class!` 宏 | 宏已知字段集合和类型，分类生成 Copy/非Copy 访问器 |
| 方法签名泛型重写（Object → E） | `java_class!` 宏 | 块级宏同时看到字段类型参数和方法签名 |
| 方法体字段访问 token 重写 | `java_class!` 宏 | 块级宏同时拥有字段集合和方法体，唯一能做 token 重写的位置 |
| JavaObject / vtable / Upcast impl | `java_class!` 宏 | 已有逻辑从 `#[java_class]` 迁移进来 |
| 字节码翻译逻辑（方法体指令） | codegen Python | 不变，继续走 ILOAD/ALOAD/INVOKEVIRTUAL 等翻译流程 |

---

## `_impl.rs` 兼容性

native 方法的手写实现在 `java_class!` 宏外部（`_impl.rs` 通过 `mod.rs` 包含）。这些文件写的是展开后的类型，需使用宏生成的访问器：

```rust
// string_impl.rs（手写 native 方法）
impl String {
    pub fn intern(&self) -> Result<Object> {
        // 宏已展开，此处 String 是 newtype 包装
        // 用 pub(crate) 访问器，不用裸字段
        let value = self.get_value();  // 访问器，pub(crate)
        // ...
    }
}
```

**规则**：宏生成的字段访问器设为 `pub(crate)`，`_impl.rs` 通过访问器访问字段。手写代码的作者知道自己在写 Rust，使用访问器是合理要求，不需要伪装成 Java 风格。

---

## 实施步骤

```
Step 1  设计 java_class! 输入语法并手写 ArrayList 验证
        ├── 确定宏 token 树的解析结构（struct 部分 + impl 部分）
        ├── 确定 superclass_fields 参数格式
        ├── 确定 generic_signature 字符串格式（复用现有 codegen 生成的格式）
        └── 手写展开目标，确认 cargo check 通过
            输出：verified ArrayList expand target

Step 2  实现 java_class! struct 层展开
        ├── 解析 struct 字段（名称、类型）
        ├── 解析 superclass_fields 参数
        ├── 生成 __inner struct（祖先字段在前 + 自有字段）
        ├── 生成 RefCell<Inner> newtype 包装
        ├── generic_sig 字段类型重写（TE; → E，[TE; → Vec<E>）
        └── 生成字段访问器（Copy → 值语义，非Copy → Ref/RefMut，窗口最小化）
            输出：struct 层单元测试通过

Step 3  实现 java_class! vtable / Into / From / Debug 迁移
        ├── 从 #[java_class] 属性宏迁移现有逻辑（all_supertypes、is_instance_of）
        ├── toString / hashCode 条件转发迁移
        └── has_to_string_method / has_hash_code_method 标志迁移
            输出：与现有 #[java_class] 行为等价

        ── 在 ArrayList 单测通过（Step 1-3）后再继续 ──

Step 4  实现 java_class! 方法签名泛型重写
        ├── 解析方法上的 generic_signature 属性
        ├── 类型变量替换（TE; → E，接口泛型 → Object）
        └── 参数类型 + 返回值类型重写
            输出：方法签名与 Java 源码 1:1 对应

Step 5  实现 java_class! 方法体 token 重写
        ├── 基于字段集合识别 self.field 访问模式
        ├── 展开三种模式：方法调用 / 赋值 / 复合赋值
        ├── borrow 窗口 block-scoped 展开
        └── 边缘情况处理：self.field[i]、let x = &self.field、match self.field
            输出：add/get/remove 等方法体 token 重写正确

        ── Step 4-5 可在 Step 2-3 稳定后并行开发 ──

Step 6  修改 codegen Python
        ├── 输出 java_class! { ... } 块而非 #[java_class] struct
        ├── 字段直接输出 generic_sig 驱动的裸类型（不再包 JField）
        ├── superclass_fields 由 Python 沿继承链展平后传入
        └── 方法体继续走字节码翻译，不生成访问器调用（宏做 token 重写）
            输出：jdk_classes 全量重新生成，cargo check 通过

Step 7  删除 JField<T>
        └── e2e 测试全绿后，从 java_runtime/src/types.rs 删除 JField
            输出：无 JField 引用，cargo check 通过
```

---

## 迁移期兼容策略

`java_class!` 块级宏和现有 `#[java_class]` 属性宏可以共存：

- Step 1-5 期间：新宏在独立测试 crate 验证，不影响现有生成代码
- Step 6 期间：codegen 切换输出格式，`#[java_class]` 属性宏暂时保留
- Step 7 之后：属性宏废弃，仅保留块级宏

两个宏在同一个 `java_rta_macros` crate 里维护，不需要额外依赖。

---

## 设计原则记录

这次讨论的演进路径本身是值得记录的：从"如何去掉 JField"到"泛型如何保留"到"如何让宏做封装"，三个问题最终收敛到同一个机制（`java_class!` 块级宏）。

**根本原因**：codegen Python 侧已经拥有所有需要的信息（字段类型、泛型签名、继承关系），但这些信息没有有效传递给 Rust 的类型系统。块级宏是这个"信息传递桥梁"——codegen 把 Python 侧知道的一切写进宏参数，宏在 Rust 编译期展开，两侧信息完全对齐。

**推论**：遇到生成代码"看起来不像 Java"的地方，首先问"Python 侧是否已有这个信息"，如果有，解法是把它传给宏，而不是在生成代码里用替代方案。

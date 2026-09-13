# 设计文档：Java 风格 API 完整路线图

**日期**：2026-09-13  
**目标**：让用户用 Rust 写代码时，API 使用方式与 Java 代码高度相似；差异由宏/VM 层吸收，复杂性对用户透明

---

## 一、目标状态对比

### Java 原版代码
```java
// 泛型集合
ArrayList<String> list = new ArrayList<>();
list.add("Alice");
list.add("Bob");
String first = list.get(0);
System.out.println(first);
System.out.println(list.size());

// 装箱整数
ArrayList<Integer> nums = new ArrayList<>();
nums.add(10);
nums.add(20);
int n = nums.get(0);

// HashMap
HashMap<String, Integer> map = new HashMap<>();
map.put("key", 42);
int val = map.get("key");
System.out.println(map.containsKey("key"));

// 用户自定义类
class Person { String name; int age; }
ArrayList<Person> persons = new ArrayList<>();
persons.add(new Person("Alice", 30));
Person p = persons.get(0);
```

### 目标 Rust 代码（与 Java 对齐后）
```rust
use jdk_classes::java::util::*;
use jdk_classes::java::lang::*;
use java_runtime::prelude::*;

fn main() -> Result<()> {
    // 泛型集合 — 与 Java 几乎一致
    let mut list: ArrayList<String> = ArrayList::new()?;
    list.add(String::from("Alice"))?;          // Java: list.add("Alice")
    list.add(String::from("Bob"))?;
    let first: String = list.get(0)?;          // Java: String first = list.get(0)
    System::out().println(first)?;             // Java: System.out.println(first)
    System::out().println(list.size()?)?;      // Java: System.out.println(list.size())

    // 装箱整数 — Rust 无隐式装箱，差异被吸收
    let mut nums: ArrayList<i32> = ArrayList::new()?;
    nums.add(10)?;                             // Java: nums.add(10)
    nums.add(20)?;
    let n: i32 = nums.get(0)?;                // Java: int n = nums.get(0)

    // HashMap — 与 Java 几乎一致
    let mut map: HashMap<String, i32> = HashMap::new()?;
    map.put(String::from("key"), 42)?;         // Java: map.put("key", 42)
    let val: i32 = map.get(String::from("key"))?;  // Java: int val = map.get("key")
    System::out().println(map.containsKey(String::from("key"))?)?;

    // 用户自定义类
    let mut persons: ArrayList<Person> = ArrayList::new()?;
    persons.add(Person::new(String::from("Alice"), 30)?)?;
    let p: Person = persons.get(0)?;
    Ok(())
}
```

### 对比：当前状态 vs 目标状态

| 操作 | 当前 Rust | 目标 Rust | 差距根因 |
|------|-----------|-----------|---------|
| 构造 ArrayList | `ArrayList::<Object>::new_default()?` | `ArrayList::new()?` | 命名+泛型 |
| 添加 String | `list.add__obj(x.into())?` | `list.add(x)?` | 方法名+隐式转换 |
| 获取元素 | `list.get(0i32)?.downcast::<String>()` | `list.get(0)?` | 方法名+类型恢复 |
| 添加 int | `list.add__obj(10i32.into())?` | `nums.add(10)?` | 泛型+自动装箱 |
| println | `System::out().println__i(42)?` | `System::out().println(42)?` | 方法名 |
| HashMap.put | `map.put(k.into(), v.into())?` | `map.put(k, v)?` | 隐式转换 |

---

## 二、技术根因分析

### 2.1 核心问题：类型参数 E 是 phantom

当前 `ArrayList<E>` 的 `E` 没有实际类型约束作用：

```rust
pub struct ArrayList<E: Clone + 'static> {
    pub elementData: JField<Rc<RefCell<Vec<Object>>>>,  // 存储 Object，E 未参与
    pub size:        JField<i32>,
    pub _phantom:    std::marker::PhantomData<E>,        // E 只是占位符
}

// 方法签名也不使用 E：
pub fn add__obj(&self, e: Object) -> Result<bool>   // 应该是 add(&self, e: E)
pub fn get(&self, index: i32) -> Result<Object>     // 应该是 get(&self, i: i32) -> Result<E>
```

**根因**：JVM 字节码经类型擦除，`ArrayList.add:(TE;)Z` 变成 `ArrayList.add:(Ljava/lang/Object;)Z`。转译器直接翻译了擦除后的字节码，`E` 信息丢失。

### 2.2 信息已存在：Signature 属性

`ArrayList.add` 的 Java **Signature** 属性保存了 `(TE;)Z`——参数类型是类型变量 `E`，不是 `Object`。这个信息已经被解析（T34 相关工作），只是没用于方法签名生成。

### 2.3 缺失的 unboxing（Object → primitive）

`From<Object> for i32` 等不存在。当 `ArrayList<i32>` 的 `get(0)` 返回 `i32` 时，需要从 `Object` 中取出 `i32`。目前只有 boxing（`From<i32> for Object`），没有 unboxing。

---

## 三、完整技术方案

### 方案架构：两层方法 + proc-macro

```
用户代码层：  list.add(x)    list.get(0)    map.put(k, v)
              ↓              ↓               ↓
ergonomic层： add(e: E)      get(i) -> E    put(k: K, v: V)
              ↓              ↓               ↓
JVM原始层：   add_obj(Object) get_raw(i)->Object  put_obj(K,V)->Object
              （用于字节码翻译生成代码，用户不接触）
```

### 3.1 Unboxing：为 Object → primitive 添加 From 实现

在 `java_runtime/src/java/lang/object.rs` 中添加：

```rust
// T35 已有 boxing（From<i32> for Object），补充 unboxing：
impl From<Object> for i32   { fn from(o: Object) -> i32   { o.downcast::<i32>()   } }
impl From<Object> for i64   { fn from(o: Object) -> i64   { o.downcast::<i64>()   } }
impl From<Object> for f32   { fn from(o: Object) -> f32   { o.downcast::<f32>()   } }
impl From<Object> for f64   { fn from(o: Object) -> f64   { o.downcast::<f64>()   } }
impl From<Object> for bool  { fn from(o: Object) -> bool  { o.downcast::<bool>()  } }
```

完成后，`ArrayList<i32>` 的 `get(0) -> i32` 就可以工作：`i32::from(obj)`。

### 3.2 集合 ergonomic 层：泛型方法覆盖

在 `native_impls` 中通过 `@synthetic` 注入额外 `impl` 块，提供使用类型参数 `E` 的方法：

```rust
// native_impls/java/util/array_list.rs 中新增 @synthetic ergonomic block：

/// @synthetic_impl<E: Clone + Into<Object> + From<Object> + 'static>
/// ArrayList ergonomic API: add/get using type parameter E directly
pub fn add_typed<E: Clone + Into<Object> + From<Object> + 'static>(
    _this: &ArrayList<E>, e: E
) -> Result<bool> {
    _this.add__obj(e.into())
}

pub fn get_typed<E: Clone + Into<Object> + From<Object> + 'static>(
    _this: &ArrayList<E>, index: i32
) -> Result<E> {
    let obj = _this.get(index)?;
    Ok(E::from(obj))
}
```

然后在 emitter 中，对 `ArrayList<E>` 生成额外的 `impl` 块（需要 proc-macro 或直接代码生成），提供 `add(e: E)` 和 `get_typed(i) -> E`。

**重命名策略**（关键）：

| 当前名称 | 新 JVM-exact 名称 | 新 ergonomic 名称 |
|---------|-----------------|-----------------|
| `add__obj(Object)` | `add_obj(Object)` | `add(E)` |
| `get(i32) -> Object` | `get_obj(i32) -> Object` | `get(i32) -> E` |
| `put(Object, Object)` | `put_obj(Object, Object)` | `put(K, V)` |
| `add__obj` (HashMap set) | `add_obj` | `add(E)` |

字节码翻译生成的代码调用 `_obj` 后缀版本，用户代码调用干净版本。

### 3.3 proc-macro crate `java_rta_macros`

新建 proc-macro crate，替换 `cfg_attr(any(), java_class(...))` 为真正激活的 attribute macro。

#### `#[java_class(...)]` 展开内容

输入（struct 定义 + 属性）：
```rust
#[java_class(
    binary_name = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,java/util/RandomAccess,...",
    access       = "public",
)]
#[derive(Clone, Default)]
pub struct ArrayList<E: Clone + 'static> { ... }
```

宏展开自动生成：

```rust
// 1. Object 转换（已由 emitter 生成，改为宏负责）
impl<E: Clone + 'static> Into<Object> for ArrayList<E> {
    fn into(self) -> Object { Object::from_any(self) }
}
impl<E: Clone + 'static> From<Object> for ArrayList<E> {
    fn from(o: Object) -> Self { o.downcast::<Self>() }
}

// 2. 标准 Rust trait
impl<E: Clone + 'static> std::fmt::Display for ArrayList<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayList(size={})", self.size().unwrap_or(0))
    }
}
impl<E: Clone + 'static> std::fmt::Debug for ArrayList<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayList<E>@{:p}", self as *const _)
    }
}

// 3. 当 E 满足更强约束时，激活 ergonomic 泛型方法
// （由宏根据 interfaces 信息判断是否为 Collection，自动注入 add/get 包装）
impl<E: Clone + Into<Object> + From<Object> + 'static> ArrayList<E> {
    /// Java: list.add(e)  ← 不需要 .into()
    pub fn add(&self, e: E) -> Result<bool> { self.add_obj(e.into()) }
    
    /// Java: E e = list.get(i)  ← 不需要 .downcast::<E>()
    pub fn get(&self, index: i32) -> Result<E> {
        Ok(E::from(self.get_obj(index)?))
    }
    
    /// Java: for (E e : list)  ← Rust iterator
    pub fn iter(&self) -> impl Iterator<Item = E> {
        let data = self.elementData.get();
        let vec = data.borrow().clone();
        vec.into_iter().map(|o| E::from(o))
    }
}
```

#### 用户自定义 `#[java_class]` struct

用户只需写这些，宏生成其余所有样板：

```rust
use java_rta_macros::java_class;
use java_runtime::prelude::*;

#[java_class(binary_name = "com/example/Person")]
#[derive(Clone, Default)]
pub struct Person {
    pub name: JField<String>,
    pub age:  JField<i32>,
}

impl Person {
    pub fn new(name: String, age: i32) -> Result<Self> {
        let p = Self::default();
        p.name.set(name);
        p.age.set(age);
        Ok(p)
    }
}

// 宏自动生成了：
// impl Into<Object> for Person { ... }
// impl From<Object> for Person { ... }
// impl Display for Person { ... }
```

用户代码与 JDK API 的整合：

```rust
let persons: ArrayList<Person> = ArrayList::new()?;
persons.add(Person::new(String::from("Alice"), 30)?)?;  // ← 不需要 .into()
let p: Person = persons.get(0)?;                         // ← 不需要 .downcast()
```

#### `#[java_field(...)]` 展开内容

```rust
#[java_field(name = "name", descriptor = "Ljava/lang/String;", access = "private")]
pub name: JField<String>,

// 宏生成访问器文档（IDE hover 时显示 Java 原始签名）：
/// Java field: `private String name`
/// Descriptor: `Ljava/lang/String;`
pub name: JField<String>,
// 宏可以选择性生成命名访问器，减少 .get()/.set() 调用：
pub fn get_name(&self) -> String { self.name.get() }
pub fn set_name(&self, v: String) { self.name.set(v) }
```

### 3.4 `println` 统一 trait 派发

核心：定义 `Printable` trait，所有"可打印"类型实现它。

```rust
// 在 java_runtime/src/java/lang/object.rs 中定义：
pub trait Printable {
    fn to_print_string(&self) -> std::string::String;
}

impl Printable for i32   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for i64   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for bool  { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for f32   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for f64   { fn to_print_string(&self) -> std::string::String { format!("{}", self) } }
impl Printable for Object {
    fn to_print_string(&self) -> std::string::String { format!("{}", self) }
}
// String 实现在 jdk_classes 上下文（因为 String 类型在 jdk_classes 里）

// PrintStream 的统一 println：
impl PrintStream {
    pub fn println<T: Printable>(&self, v: T) -> Result<()> {
        let s = v.to_print_string();
        println!("{}", s);
        Ok(())
    }
}
```

这样：
```rust
System::out().println(42)?;                    // ← Java: System.out.println(42)
System::out().println(String::from("hi"))?;    // ← Java: System.out.println("hi")
System::out().println(true)?;                  // ← Java: System.out.println(true)
System::out().println(list.size()?)?;          // ← Java: System.out.println(list.size())
```

代码生成器中，`invokevirtual PrintStream.println:(*` 系列调用统一生成 `println(x)`，不再分 `println__i / println__str / println__obj`。

---

## 四、用户真正需要理解的最小差异

完成上述方案后，Java → Rust 的**不可消除的差异**（语言本质差异，不能用宏消除）：

| Java | Rust | 无法消除的原因 |
|------|------|-------------|
| `"Alice"` | `String::from("Alice")` | Rust 有自己的 `&str`/`String`，必须显式构造 Java String |
| `list.add("Alice")` | `list.add(String::from("Alice"))?` | 字符串构造 + `?` |
| `String first = list.get(0)` | `let first: String = list.get(0)?` | 类型声明语法 + `?` |
| `catch (Exception e)` | `?` 传播 | Rust 不支持 checked exception |
| `list.get(0) == null` | `if result == Object::default()` | Rust 无 null |
| `new Foo()` | `Foo::new()?` | Rust 构造函数语法 + `?` |

**不需要用户理解的差异**（被宏/VM层吸收）：

- `.into()` — 自动装箱，被泛型 `E: Into<Object>` 吸收
- `.downcast::<T>()` — 类型恢复，被泛型 `E: From<Object>` 吸收
- `Into<Object> impl` — `#[java_class]` 宏自动生成
- `Rc<RefCell<...>>` — 封装在 VM 层，用户完全看不到
- `JField<T>` 的 `.get()/.set()` — 被 `#[java_field]` 生成的访问器隐藏（可选）

---

## 五、实现路线图

### Phase A：基础修复（无架构改动）

**A-1：添加 Unboxing（Object → primitive）**  
- 文件：`output/java_runtime/src/java/lang/object.rs`
- 添加 `impl From<Object> for i32/i64/bool/f32/f64`
- 复杂度：低（10 行代码）

**A-2：方法名去 mangle 优化**  
- 文件：`scripts/codegen/emitter.py`  
- 规则：每组重载中参数最少的版本保留原名（`add` 代替 `add__obj`）；同时 JVM-exact 版本改名为 `add_obj` 供字节码翻译使用
- 复杂度：中（需同步更新 `instr.py` 的调用生成）

**A-3：构造器统一为 `new()`**  
- 文件：`scripts/codegen/emitter.py`
- 无参 `<init>` 的 synthetic 构造器命名为 `new()` 而非 `new_default()`
- 复杂度：低

**A-4：`println` 统一 trait**  
- 文件：`output/java_runtime/src/java/lang/object.rs`（定义 Printable）、`output/native_impls/java/io/print_stream.rs`（统一 println）、`scripts/codegen/instr.py`（统一生成 println(x)）
- 复杂度：中

### Phase B：泛型方法层（ergonomic API 核心）

**B-1：集合 ergonomic impl 生成**  
- 目标：`ArrayList<String>` 的 `add(String)` 和 `get(i32) -> String` 工作
- 路径 1（快）：在 `native_impls/java/util/array_list.rs` 中添加 `@synthetic_impl` 注解，由 emitter 识别并生成额外 impl 块
- 路径 2（正确）：代码生成器解析方法 Signature，当参数类型是类型变量时直接生成 `E` 而非 `Object`
- 推荐路径 1 先行，路径 2 作为长期目标
- 复杂度：中

**B-2：HashMap/HashSet 同步**  
- 同 B-1，为 `HashMap<K,V>` 和 `HashSet<E>` 生成泛型方法
- 复杂度：低（复用 B-1 框架）

### Phase C：proc-macro crate（架构性改造）

**C-1：新建 `java_rta_macros` crate**  
- 新建 proc-macro crate，加入 workspace
- 复杂度：低（框架搭建）

**C-2：实现 `#[java_class]` 宏**  
- 解析 `binary_name`、`interfaces` 等属性
- 生成 `Into<Object>`、`From<Object>`、`Display`、`Debug`
- 当 struct 实现了 Collection 接口时，生成 ergonomic 泛型方法
- 复杂度：中

**C-3：替换 `cfg_attr(any(), ...)` 为直接 `#[java_class]`**  
- 修改 emitter.py 生成真实宏调用
- 同步更新 build.rs（不再需要文本扫描 native 方法）
- 复杂度：中

**C-4：用户侧验证**  
- 用 `#[java_class]` 写用户自定义类，验证整合效果
- 复杂度：低

### 实现优先级

```
A-1 (Unboxing)         → 立即可做，解锁 ArrayList<i32>
A-2 (方法名去 mangle)  → 高优先，最大影响面
A-3 (new() 构造器)     → 顺手，一起做
A-4 (println 统一)     → 中优先
B-1 (集合 ergonomic)   → 高优先，核心用户体验
B-2 (HashMap/HashSet)  → B-1 之后顺手
C-1~C-4 (proc-macro)  → 架构完善，中长期目标
```

---

## 六、验收标准

### 最终验收：以下 Rust 代码必须编译且正确运行

```rust
use jdk_classes::java::util::*;
use jdk_classes::java::lang::*;
use java_runtime::prelude::*;
use java_rta_macros::java_class;

#[java_class(binary_name = "com/example/Person")]
#[derive(Clone, Default)]
struct Person {
    pub name: JField<String>,
}
impl Person {
    pub fn new(name: String) -> Result<Self> {
        let p = Self::default();
        p.name.set(name);
        Ok(p)
    }
}

fn test() -> Result<()> {
    // Phase A 验收：方法名 + 构造器
    let list: ArrayList<String> = ArrayList::new()?;  // not new_default
    list.add(String::from("Alice"))?;                  // not add__obj + .into()
    list.add(String::from("Bob"))?;
    
    // Phase B 验收：泛型类型恢复
    let first: String = list.get(0)?;                 // not .downcast::<String>()
    System::out().println(first)?;                    // not println__str
    System::out().println(list.size()?)?;             // not println__i
    
    // Phase A-1 验收：integer unboxing
    let nums: ArrayList<i32> = ArrayList::new()?;
    nums.add(10)?;                                    // not 10i32.into()
    let n: i32 = nums.get(0)?;                       // not .downcast::<i32>()
    System::out().println(n)?;
    
    // Phase B 验收：HashMap 泛型
    let map: HashMap<String, i32> = HashMap::new()?;
    map.put(String::from("key"), 42)?;                // not .into() on both sides
    let val: i32 = map.get(String::from("key"))?;    // not .downcast::<i32>()
    System::out().println(map.containsKey(String::from("key"))?)?;
    
    // Phase C 验收：用户自定义类
    let persons: ArrayList<Person> = ArrayList::new()?;
    persons.add(Person::new(String::from("Alice"))?)?;  // not .into()
    let p: Person = persons.get(0)?;                    // not .downcast::<Person>()
    System::out().println(p.name.get())?;
    
    Ok(())
}
```

---

## 七、约束和不变量

以下约束**不得**因追求 Java 相似性而破坏：

1. **`?` 操作符**：所有 Java 方法返回 `Result<T>`，调用处必须有 `?`。这是 Rust 错误处理的核心，不得省略
2. **`String::from("...")`**：Java String 遮蔽了 Rust `&str`，用户必须显式构造。可用辅助宏 `s!("...")` 简化，但不能消除
3. **`let` 声明**：Rust 变量必须显式声明，不能像 Java 一样省略
4. **方法名保持 Java 语义**：`add`/`get`/`put`/`size` 等 Java API 名称保留，不改为 `push`/`at`/`insert`/`len` 等 Rust 风格
5. **生成代码与手写代码等价**：转译器生成的代码，与开发者手工翻译同一 Java 代码所写的文件，必须可以达到等价的质量

# 设计文档：生成 API 人体工程学改造

**日期**：2026-09-13  
**状态**：设计稿，待实现  
**目标**：让 java-rta 生成的 JDK API 对 Rust 用户直接可用、友好，无需了解底层翻译细节

---

## 一、问题分析

### 1.1 现状与目标规范的差距

`docs/plans/2026-09-12-codegen-java-api-rules.md` 定义了目标 API 规范，当前实现与之存在系统性差距：

| 规范要求 | 当前实际 | 问题根因 |
|---------|---------|---------|
| `ArrayList::<String>::new()?` | `ArrayList::<Object>::new_default()?` | synthetic 构造器命名 + 类型擦除 |
| `list.add(x)?` | `list.add__obj(x.into())?` | 重载 mangle 策略过保守 |
| `list.get(0)?` 返回 `String` | 返回 `Object`，需手动 `.downcast::<String>()` | 泛型擦除，无类型恢复 |
| `System::out().println(42)?` | `System::out().println__i(42)?` | println 每个重载单独 mangle |
| `map.put("key", 42)?` | `map.put("key".into(), 42i32.into())?` | 无隐式 Into<Object> |

**核心症结**：所有方法生成逻辑都是"机械翻译 JVM 字节码"，没有在 Rust 层面做 ergonomic 封装。属性（`#[cfg_attr(any(), java_class(...))]` 等）已经嵌入了全量元数据，但仅作为死代码存在，未被利用。

### 1.2 属性系统现状

生成代码中已存在三级属性，全部以 `cfg_attr(any(), ...)` 包裹（始终不激活，仅供 `build.rs` 文本扫描）：

```rust
// 类级别
#[cfg_attr(any(), java_class(
    binary_name = "java/util/ArrayList",
    super_class  = "java/util/AbstractList",
    interfaces   = "java/util/List,...",
    access       = "public",
    source       = "ArrayList.java",
))]
pub struct ArrayList<E: Clone + 'static> {

    // 字段级别
    #[cfg_attr(any(), java_field(name = "elementData", descriptor = "[Ljava/lang/Object;", access = ""))]
    pub elementData: JField<Rc<RefCell<Vec<Object>>>>,
}

impl ArrayList<E> {
    // 方法级别（native 方法）
    #[cfg_attr(any(), java_native(name = "add", descriptor = "(Ljava/lang/Object;)Z", access = "public"))]
    pub fn add__obj(&self, e: Object) -> Result<bool> { ... }

    // 普通方法以注释形式存储（避免 cfg_attr 内关键字引发词法错误）
    // java: size()I
    pub fn size(&self) -> Result<i32> { ... }
}
```

**关键洞察**：这些属性包含了 Java 原始签名的全部信息。如果把 `cfg_attr(any(), ...)` 变成真正的 proc-macro，可以在编译期自动生成 ergonomic 封装层。

---

## 二、目标架构：三层 API 模型

```
┌─────────────────────────────────────────────────────────────┐
│  Layer 3：用户层 (user crate)                                │
│  用户写 Rust 代码，使用清洁 API                              │
│  list.add("hello")?   map.get("key")?   System::out()...    │
├─────────────────────────────────────────────────────────────┤
│  Layer 2：ergonomic 层 (proc-macro 自动生成)                 │
│  #[java_class] 宏展开产生：                                  │
│  - Into<Object> / From<Object>                               │
│  - 泛型友好的方法别名（add, get, put 等）                    │
│  - println 统一 Trait 派发                                   │
│  - 标准 Rust trait (Display, Debug, IntoIterator)           │
├─────────────────────────────────────────────────────────────┤
│  Layer 1：生成层 (jdk_classes crate)                        │
│  字节码翻译产物，方法名保留 JVM 原始签名（add__obj 等）      │
│  对用户透明，由 ergonomic 层代理                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 三、技术方案

### 3.1 方案 A：proc-macro crate `java_rta_macros`

**新增 crate**：`output/java_rta_macros/`（proc-macro crate，加入 workspace）

将 `cfg_attr(any(), java_class(...))` 变为真正激活的 `#[java_class(...)]` 属性宏，在编译期展开生成 ergonomic 代码。

#### `#[java_class(...)]` 宏展开内容

```rust
// 输入（用户写或生成器产生）：
#[java_class(binary_name = "java/util/ArrayList", ...)]
#[derive(Clone, Default)]
pub struct ArrayList<E: Clone + 'static> { ... }

// 宏展开自动生成：
impl<E: Clone + 'static> Into<Object> for ArrayList<E> {
    fn into(self) -> Object { Object::from_any(self) }
}

impl<E: Clone + 'static> From<Object> for ArrayList<E> {
    fn from(obj: Object) -> Self { obj.downcast::<Self>() }
}

impl<E: Clone + 'static> std::fmt::Display for ArrayList<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}@{:p}", "ArrayList", self as *const _)
    }
}

impl<E: Clone + 'static> std::fmt::Debug for ArrayList<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArrayList[size={}]", self.size().unwrap_or(0))
    }
}
```

#### `#[java_field(...)]` 宏展开内容

```rust
// 输入：
#[java_field(name = "size", descriptor = "I", access = "private")]
pub size: JField<i32>,

// 宏可以生成文档注释，标注原始 Java 字段信息：
/// Java field: `private int size` (descriptor: `I`)
pub size: JField<i32>,
```

字段级宏目前主要用于文档和 IDE 提示，不做代码生成（JField<T> 的 get/set 已够用）。

#### 用户侧使用方式

用户可以直接用宏定义自己的 Java 兼容类，参与 JDK API 调用：

```rust
use java_rta_macros::java_class;
use java_runtime::prelude::*;

// 用户定义一个"Java 风格"的 Rust 结构
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

// 宏自动生成了 Into<Object>/From<Object>，可直接放入 ArrayList：
fn example() -> Result<()> {
    let list: ArrayList<Object> = ArrayList::new_default()?;
    let person = Person::new(String::from("Alice"), 30)?;
    list.add__obj(person.into())?;  // person 可以装箱为 Object
    
    let p: Person = list.get(0i32)?.downcast::<Person>();
    Ok(())
}
```

---

### 3.2 方案 B：ergonomic 方法生成（代码生成器改进）

不依赖 proc-macro，直接在 emitter/native_impls 层生成更友好的方法。

#### B-1：方法名去 mangle 策略

**现在**：只要 Java 类里有重载，全部加后缀。
**改进**：只对 Rust 层面有歧义的重载加后缀，优先保留"最常用"版本的原始名。

**规则**：
1. 若某方法名只有一个版本（不论参数），保留原名
2. 若有多个重载，按参数数量从少到多排序，参数最少的一个保留原名
3. 其他重载加描述符后缀

```rust
// 现在：
pub fn add__obj(&self, e: Object) -> Result<bool>     // 最常用，应为 add
pub fn add__i_obj(&self, i: i32, e: Object) -> Result<()>

// 改进后：
pub fn add(&self, e: Object) -> Result<bool>          // ← 无后缀
pub fn add__i_obj(&self, i: i32, e: Object) -> Result<()>
```

**影响**：`ArrayList.add(Object)`, `HashMap.put(K,V)`, `HashSet.add(E)` 等高频 API 直接可用原始名。

#### B-2：构造器统一为 `new()`

现在 synthetic 构造器叫 `new_default()`，与规范文档要求的 `new()` 不符。

**改进**：
- 若 Java 类有无参 `<init>` → synthetic 生成 `new()` 而非 `new_default()`
- 若有多个构造器重载 → 保持 mangle（`new__i`, `new__i_f` 等）

```rust
// 现在：
ArrayList::<Object>::new_default()?

// 改进后：
ArrayList::<Object>::new()?
```

#### B-3：`println` 统一派发

`PrintStream.println` 的所有重载通过一个泛型 native 方法统一：

```rust
// 在 native_impls/java/io/print_stream.rs 中新增：

/// @synthetic
pub fn println<T: std::fmt::Display>(&self, v: T) -> Result<()> {
    println!("{}", v);
    Ok(())
}
```

配合代码生成器中对 `invokevirtual PrintStream.println:(*` 的统一识别，生成 `println(x)` 而非 `println__i(x)` / `println__str(x)` / `println__obj(x)`。

实现方式：在 `instr.py` 的 `_gen_invokevirtual` 中，识别 `println` 系列调用，统一生成 `println(x)` + 必要的 Display 转换。

---

### 3.3 方案 C：集合 ergonomic 扩展 trait

为集合类型实现 Rust 标准 trait，让用户写更 Rust 风格的代码：

```rust
// 在 native_impls 中通过 @synthetic 注入：

// ArrayList 支持 for 循环（IntoIterator）
/// @synthetic
pub fn iter<E: Clone + 'static>(this: &ArrayList<E>) -> impl Iterator<Item = Object> {
    let data = this.elementData.get();
    let vec = data.borrow().clone();
    vec.into_iter()
}

// 这样用户可以：
for item in list.iter() {
    let s: String = item.downcast::<String>();
    System::out().println__str(s)?;
}
```

---

## 四、用户体验目标

### 4.1 手写 Rust 调用 JDK API 的理想代码

完成三个方案后，用户写 Rust 代码应该是：

```rust
use jdk_classes::java::util::*;
use jdk_classes::java::lang::*;
use java_runtime::prelude::*;
use java_rta_macros::java_class;  // 新增

// 用户自己的 Java 兼容类
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
    pub fn get_name(&self) -> String { self.name.get() }
}

fn main_example() -> Result<()> {
    // ArrayList - 泛型参数仍是 Object（类型擦除），但构造器和方法名干净
    let list: ArrayList<Object> = ArrayList::new()?;          // ← new() 不再是 new_default()
    list.add(String::from("Alice").into())?;                  // ← add 不再是 add__obj
    list.add(String::from("Bob").into())?;
    let first: String = list.get(0i32)?.downcast::<String>();
    
    // HashMap - 同上
    let map: HashMap<Object, Object> = HashMap::new()?;
    map.put(String::from("key").into(), 42i32.into())?;       // ← put 不再有歧义
    let val: i32 = map.get(String::from("key").into())?.downcast::<i32>();
    
    // System.out.println - 统一调用
    System::out().println(42)?;                               // ← 不再是 println__i
    System::out().println(String::from("hello"))?;            // ← 不再是 println__str
    
    // 用户自定义类参与集合
    let persons: ArrayList<Object> = ArrayList::new()?;
    persons.add(Person::new(String::from("Charlie"))?.into())?;
    let p: Person = persons.get(0i32)?.downcast::<Person>();
    System::out().println(p.get_name())?;
    
    Ok(())
}
```

### 4.2 与当前代码的对比

| 操作 | 当前 | 目标 |
|------|------|------|
| 创建 ArrayList | `ArrayList::<Object>::new_default()?` | `ArrayList::new()?` |
| 添加元素 | `list.add__obj(x.into())?` | `list.add(x.into())?` |
| 创建 HashMap | `HashMap::<Object,Object>::new_default()?` | `HashMap::new()?` |
| 打印 int | `System::out().println__i(n)?` | `System::out().println(n)?` |
| 打印 String | `System::out().println__str(s.clone())?` | `System::out().println(s)?` |
| 用户类 → Object | 手写 `impl Into<Object>` | `#[java_class]` 自动生成 |

`.into()` 和 `.downcast::<T>()` 因为 Java 类型擦除是本质复杂度，**不应该消除**，只是让其他部分更干净。

---

## 五、实现路线图

### T36 · 方法名去 mangle 优化（B-1 + B-2）

**范围**：修改 `scripts/codegen/emitter.py` 的重载命名策略

**规则**：
- 对每个类，按 Java 方法名分组
- 每组中，参数数量最少的版本保留原名（无后缀）
- 其他版本按现有规则 mangle
- `<init>` 无参版本生成 `new()`，有参版本生成 `new__<suffix>()`

**验收**：
- `ArrayList.add(Object)` → `pub fn add(&self, e: Object)`
- `HashMap.put(K, V)` → `pub fn put(&self, key: Object, value: Object)`
- `ArrayList::<Object>::new()?` 可用
- TestP3、TestGenerics 继续正确运行

**风险**：mangle 策略改变会影响 `instr.py` 中的方法调用生成（_mangle_if_overloaded），需同步更新。

---

### T37 · `println` 统一派发（B-3）

**范围**：
1. `native_impls/java/io/print_stream.rs` 新增泛型 `println<T: Display>`
2. `instr.py` 识别 `println` 系列 invokevirtual，统一生成 `println(x)`（x 需确保实现 Display）

**验收**：
- `System::out().println(42)?` 可用
- `System::out().println(String::from("hi"))?` 可用
- `System::out().println(true)?` 可用

---

### T38 · proc-macro crate `java_rta_macros`（方案 A）

**范围**：新建 `output/java_rta_macros/` proc-macro crate

**功能**：
1. `#[java_class(...)]` — 生成 `Into<Object>`, `From<Object>`, `Display`, `Debug`
2. `#[java_field(...)]` — 生成字段文档注释（暂不做代码生成）
3. 替换所有 `cfg_attr(any(), java_class(...))` 为直接 `#[java_class(...)]`

**工作量估计**：
- 新建 proc-macro crate：1 天
- 实现 `#[java_class]` 展开：1-2 天
- 更新 emitter.py 去掉 `cfg_attr(any(), ...)` 包裹：半天
- 测试：半天

**验收**：
- 用户自定义 `#[java_class]` struct 可直接 `.into()` 到 `Object`
- 现有测试（TestP2、TestP3、TestGenerics）继续正确运行
- build.rs 的 native 方法扫描改为用 `#[java_native]` 属性（不再需要文本扫描）

---

### T39 · 集合 ergonomic 扩展（方案 C，可选）

**范围**：为 ArrayList/HashMap/HashSet 在 native_impls 中注入 `@synthetic` 扩展方法

**功能**：
- `ArrayList::iter()` 返回 Rust Iterator
- `ArrayList::len()` 作为 `size()` 的 Rust 风格别名
- `HashMap::iter()` 返回 `(Object, Object)` 迭代器

**依赖**：T38（proc-macro crate）完成后更易实现

---

## 六、优先级建议

```
T36 (方法名去 mangle)   → 最高优先级，影响最广，代价最小
T37 (println 统一派发)  → 高优先级，改善日常使用体验
T38 (proc-macro crate)  → 中优先级，使用户扩展成为可能
T39 (集合 ergonomic)    → 低优先级，按需实现
```

T36 和 T37 是纯代码生成器改进，不需要新建 crate，风险低，收益大。T38 是架构性改进，让整个系统具备"用户可扩展"的能力。

**执行序**：T36 → T37 → T38 → T39（可与 T33 单 crate 迁移并行）

---

## 七、设计约束

以下约束来自 `codegen-java-api-rules.md` 和 `target-architecture.md`，实现时必须遵守：

1. **生成代码不使用 Rust 标准库类型**（`Vec`、`HashMap`、`Rc` 等）——这些只出现在 native_impls 和 java_runtime 内部
2. **方法名保持 Java 风格**（`add` 不改成 `push`，`size` 不改成 `len`）——保持 Java 语义映射
3. **所有方法返回 `Result<T>`** — ergonomic 封装不得省略 `?`
4. **类型擦除是本质复杂度**——`ArrayList<Object>` 是正确的，不强求 `ArrayList<String>`（那需要 Java 泛型信息，T34 已做了局部改进）
5. **`Rc<RefCell<...>>` 不出现在业务代码**——应封装在 JField<T> 或 native_impls 内

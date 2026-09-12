# 代码生成规则：ruva API 规范

**日期**：2026-09-12  
**适用文件**：`scripts/codegen/` 所有生成器模块  
**优先级**：本文档的规则优先于 `2026-09-12-java-to-rust-transpiler.md` 中的旧类型映射

---

## 核心原则

### R-01：生成代码与手写代码编译产物等价

转译器生成的 Rust 文件，必须与一名有经验的开发者**手动**将同一 Java 代码翻译为 Rust 所写的文件完全一致。

- 生成代码不得使用任何"生成器专属"的内部 API 或运行时钩子
- 生成代码不得引用运行时基础设施的内部实现（如 `raw::` 子模块、`_alloc()`、`Field::new()` 等）
- 所有使用的 API 必须是 `java_base` 库向外公开的稳定接口

### R-02：运行时基础设施对用户完全不可见

`Rc<RefCell<>>`、`raw::` 子模块、内存分配细节、`Field::new()` 等实现细节：

- 只出现在 `java_base` 库的内部实现中，**不得**出现在生成的业务代码里
- 生成代码通过公共 trait、宏、方法调用与运行时交互

### R-03：命名空间与 Java 完全同构

Java 类的名字和包路径，在 Rust 中必须完全对应：

| Java | Rust 文件路径 | Rust 模块路径 |
|------|--------------|--------------|
| `HelloWorld`（默认包） | `src/hello_world.rs` | `crate::HelloWorld` |
| `com.example.Foo` | `src/com/example/foo.rs` | `crate::com::example::Foo` |
| `java.lang.String` | （在 `java_base` 内） | `java_base::java::lang::String` |
| `java.util.ArrayList` | （在 `java_base` 内） | `java_base::java::util::ArrayList` |

**自动生成中间 `mod.rs`**：emitter 须为每一层包路径生成 `mod.rs`，声明子模块并 re-export 类型。

### R-04：类型名称不使用 J 前缀

禁止使用 `JString`、`JvmRef`、`JArrayList` 等带 J 前缀的类型名。

所有 Java 类型在生成代码中以其 Java 名称出现（通过 `use` 引入作用域后直接使用）：

```rust
// 正确
use java_base::prelude::*;
let s: String = String::from("hello");
let list: ArrayList<String> = ArrayList::new()?;

// 错误
let s: JString = JString::from("hello");  // 不用 J 前缀
let list: Vec<String> = Vec::new();        // 不用 Rust 标准库类型
```

### R-05：运行时由 `java_rta` 自己生成，不依赖外部项目

`java_rta` 生成的 Rust 项目包含两层：

1. **运行时层**（`src/java_runtime/`）：由转译器生成，提供 `String`、`ArrayList`、`Field`、`System` 等类型。此层可以在内部使用 Rust 标准库（`Rc<RefCell<>>`、`Vec` 等）实现细节，对外只暴露公共 API。
2. **业务层**（如 `src/hello_world.rs`）：由转译器从 Java 字节码翻译生成，只通过 `use crate::java_runtime::prelude::*;` 使用运行时层的公共 API。

**不依赖 `ruva` 或 `java_base`**（ruva 是已废弃的旧项目）。`Cargo.toml` 中只添加真正需要的第三方 crate（如 `rand`、`indexmap`），不添加任何 Java 平台相关的外部依赖。

```rust
// 业务层文件（hello_world.rs）的 use
use crate::java_runtime::prelude::*;
```

---

## 类型映射规则

### R-06：Java 引用类型 → java_base 封装类型

| Java 类型 | 生成代码中的 Rust 类型 | 说明 |
|-----------|----------------------|------|
| `String` | `String`（来自 `java_base::java::lang::String`） | 遮蔽 Rust 的 `std::string::String` |
| `StringBuilder` | `StringBuilder`（来自 `java_base::java::lang`） | |
| `ArrayList<E>` | `ArrayList<E>` | |
| `HashMap<K,V>` | `HashMap<K,V>` | |
| `HashSet<E>` | `HashSet<E>` | |
| `int[]` | `IntArray` 或 `Array<i32>` | |
| 用户定义类 `Foo` | `Foo`（同名 struct） | |
| `null` | `Option<T>` | |

### R-07：基本类型保持 Rust 原生类型

| Java | Rust |
|------|------|
| `int` | `i32` |
| `long` | `i64` |
| `double` | `f64` |
| `float` | `f32` |
| `boolean` | `bool` |
| `void` | `()` |

基本类型**不**封装，直接以 Rust 原生类型传递。

---

## 方法签名规则

### R-08：所有 Java 方法返回 `Result<T>`

```rust
// Java: void greet()  →
pub fn greet(&self) -> Result<()>

// Java: String repeat(String s, int n)  →
pub fn repeat(s: String, n: i32) -> Result<String>

// Java: public static void main(String[] args)  →
pub fn main() -> Result<()>
```

例外：`main.rs` 的 Rust 入口 `fn main()` 调用 Java main 并展开错误：

```rust
fn main() {
    ClassName::main().unwrap_or_else(|e| eprintln!("Error: {:?}", e));
}
```

### R-09：实例方法使用 `&self`，不暴露 `this: &Rc<RefCell<Self>>`

```rust
// 正确
pub fn greet(&self) -> Result<()>

// 错误（旧设计，不再使用）
pub fn greet(this: &Rc<RefCell<Self>>) -> Result<()>
```

### R-10：所有方法调用后加 `?`

```rust
// Java: hw.greet();  →
hw.greet()?;

// Java: items.add("foo");  →
items.add(String::from("foo"))?;

// Java: System.out.println(x);  →
System::out().println(x)?;
```

---

## 输出语句规则

### R-11：`System.out.println` → `System::out().println()`

```rust
// 正确
System::out().println(format!("Hello, {}", name))?;
System::out().println(value)?;

// 错误（不使用 Rust 宏）
println!("Hello, {}", name);
```

---

## 字段访问规则

### R-12：实例字段通过 `self.field` 访问（Field<T> 内部透明）

`Field<T>` 提供 `get()` / `set()` 接口，生成代码按此调用：

```rust
// 读字段
let msg = self.message.get();

// 写字段  
self.message.set(new_value);
```

字段在 struct 声明中的类型为 `Field<T>`，但用户通过稳定的公共方法与之交互，无需了解 `Field` 的实现细节。

---

## 集合操作规则

### R-13：集合操作使用 Java 方法名

```rust
// ArrayList
let items = ArrayList::<String>::new()?;
items.add(String::from("foo"))?;       // 不用 push
let v = items.get(0)?;                  // 不用 items[0]
let n = items.size();                   // 不用 .len()
items.remove_at(0);                     // 不用 .remove(0)

// HashMap
let map = HashMap::<String, i32>::new()?;
map.put(String::from("key"), 42);       // 不用 insert
let v = map.get(&key);                  // 返回 Option<V>
let n = map.size();                     // 不用 .len()
```

---

## 字符串操作规则

### R-14：字符串字面量通过 `String::from` 构造

```rust
// 正确
String::from("hello")

// 错误
"hello".to_string()
"hello".to_owned()
String::new() + "hello"
```

### R-15：字符串拼接通过 `format!` 后包装，或 `StringBuilder`

```rust
// 简单插值（invokedynamic makeConcatWithConstants）
let result: String = String::from(format!("Hello, {}", name));

// StringBuilder 模式
let mut sb = StringBuilder::new()?;
sb.append(&s)?;
sb.append_str(" world")?;
let result: String = sb.to_string()?;
```

---

## Native 方法规则

### R-16：只有 native 方法实现可使用 Rust 标准库

`java_base` crate 内部实现（native stubs）可以自由使用 Rust 标准库和第三方 crate。

**生成的业务代码**禁止直接使用：
- `std::rc::Rc`
- `std::cell::RefCell`
- `std::collections::Vec`、`HashMap`、`HashSet`
- `println!`、`eprintln!`、`format!`（除包装在 `String::from(format!(...))` 时）
- 任何 `use std::` 导入

---

## 文件结构规则

### R-17：中间包路径层自动生成 `mod.rs`

对于 Java 类 `com.example.foo.Bar`，emitter 需生成：

```
src/
  com/
    mod.rs            → pub mod example;
    example/
      mod.rs          → pub mod foo;
      foo/
        mod.rs        → pub mod bar; pub use bar::Bar;
        bar.rs        → 实际类实现
  main.rs             → mod com; fn main() { ... }
```

### R-18：`main.rs` 只做入口，不含业务逻辑

```rust
// main.rs
mod hello_world;   // 或 mod com; 等包路径顶层模块

fn main() {
    hello_world::HelloWorld::main()
        .unwrap_or_else(|e| eprintln!("Error: {:?}", e));
}
```

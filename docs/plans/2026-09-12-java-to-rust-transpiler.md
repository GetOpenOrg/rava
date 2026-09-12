# Java → Rust 转译器实现方案

**日期**：2026-09-12  
**目标**：基于 RTA（Reaching Type Analysis）的裁剪结果，将 Java 字节码转译为 Rust 源码并编译为原生二进制。

---

## 1. 总体架构

```
.java
  │  javac
  ▼
.class (字节码)
  │  javap -verbose / ClassFileParser
  ▼
JVM 栈式 IR（指令序列 + 常量池）
  │  RTA 裁剪（java_rta.py 已实现）
  ▼
可达方法集合（classes + methods + instantiated）
  │  Stack → SSA 转换
  ▼
寄存器 IR（具名变量 + 基本块）
  │  控制流重建（CFG → structured）
  ▼
结构化 IR（if / loop / match）
  │  Rust 代码生成
  ▼
.rs 文件
  │  rustc / cargo build
  ▼
native binary
```

---

## 2. 类型系统映射

### 2.1 基本类型

| Java 类型   | Rust 类型 | 说明 |
|-------------|-----------|------|
| `int`       | `i32`     | 直接映射，JVM int 为 32 位有符号 |
| `long`      | `i64`     | |
| `float`     | `f32`     | |
| `double`    | `f64`     | |
| `boolean`   | `bool`    | JVM 内部用 int 存储，转译时还原 |
| `byte`      | `i8`      | |
| `short`     | `i16`     | |
| `char`      | `u16`     | Java char 是 UTF-16 码元 |
| `void`      | `()`      | |

### 2.2 引用类型

| Java 类型        | Rust 类型                     | 说明 |
|------------------|-------------------------------|------|
| `String`         | `String`                      | 堆字符串 |
| `null`           | `Option<T>`                   | 所有引用类型包 Option |
| 类实例           | `Rc<RefCell<ClassName>>`      | 共享引用 + 内部可变 |
| 接口引用         | `Rc<RefCell<InterfaceEnum>>`  | RTA 封闭世界 → enum |
| `T[]`            | `Vec<T>`                      | 见 §4 数组 |
| `List<T>`        | `Vec<T>`                      | 见 §5 集合 |
| `Map<K,V>`       | `HashMap<K,V>`                | 见 §6 Map |
| 异常             | `Result<T, JvmError>`         | 无展开开销 |

---

## 3. 数学运算

### 3.1 整数运算指令

| JVM 指令         | Rust 输出              | 注意事项 |
|------------------|------------------------|----------|
| `iadd`           | `a.wrapping_add(b)`    | Java 整数溢出是未定义行为，用 wrapping |
| `isub`           | `a.wrapping_sub(b)`    | |
| `imul`           | `a.wrapping_mul(b)`    | |
| `idiv`           | `a / b`                | 除零 → `return Err(ArithmeticException)` |
| `irem`           | `a % b`                | |
| `ineg`           | `a.wrapping_neg()`     | |
| `ishl`           | `a << (b & 0x1f)`      | Java 只取低 5 位 |
| `ishr`           | `a >> (b & 0x1f)`      | 算术右移 |
| `iushr`          | `((a as u32) >> (b & 0x1f)) as i32` | 逻辑右移 |
| `iand`           | `a & b`                | |
| `ior`            | `a \| b`               | |
| `ixor`           | `a ^ b`                | |

`l`（long）版本同理，wrapping 操作换 `i64`，移位掩码改 `0x3f`。

### 3.2 浮点运算指令

| JVM 指令 | Rust 输出    | 注意事项 |
|----------|--------------|----------|
| `fadd`   | `a + b`      | f32，NaN 传播行为与 Java 一致 |
| `fsub`   | `a - b`      | |
| `fmul`   | `a * b`      | |
| `fdiv`   | `a / b`      | 除零 → `f32::INFINITY`，不抛异常 |
| `frem`   | `a % b`      | |
| `fneg`   | `-a`         | |
| `f2i`    | `a as i32`   | NaN → 0，超范围截断（Java 规范） |
| `i2f`    | `a as f32`   | |
| `d2f`    | `a as f32`   | |

### 3.3 Math 类常用方法

```rust
// java_runtime/math.rs — JDK Math 的 Rust 存根
pub fn math_abs_int(a: i32) -> i32     { a.abs() }
pub fn math_abs_long(a: i64) -> i64    { a.abs() }
pub fn math_abs_double(a: f64) -> f64  { a.abs() }
pub fn math_max_int(a: i32, b: i32) -> i32   { a.max(b) }
pub fn math_min_int(a: i32, b: i32) -> i32   { a.min(b) }
pub fn math_sqrt(a: f64) -> f64        { a.sqrt() }
pub fn math_pow(a: f64, b: f64) -> f64 { a.powf(b) }
pub fn math_floor(a: f64) -> f64       { a.floor() }
pub fn math_ceil(a: f64) -> f64        { a.ceil() }
pub fn math_round_float(a: f32) -> i32 { a.round() as i32 }
pub fn math_round_double(a: f64) -> i64{ a.round() as i64 }
pub fn math_random() -> f64            { /* 接入 rand crate */ }
pub fn math_log(a: f64) -> f64         { a.ln() }
pub fn math_log10(a: f64) -> f64       { a.log10() }
```

---

## 4. 数组

### 4.1 类型映射

| Java            | Rust                  |
|-----------------|-----------------------|
| `int[]`         | `Vec<i32>`            |
| `long[]`        | `Vec<i64>`            |
| `double[]`      | `Vec<f64>`            |
| `boolean[]`     | `Vec<bool>`           |
| `String[]`      | `Vec<String>`         |
| `T[]`           | `Vec<Rc<RefCell<T>>>` |
| `int[][]`       | `Vec<Vec<i32>>`       |

### 4.2 字节码指令翻译

| JVM 指令            | Rust 输出                                              |
|---------------------|--------------------------------------------------------|
| `newarray int`      | `Vec::<i32>::with_capacity(len as usize)`              |
| `anewarray T`       | `Vec::<Rc<RefCell<T>>>::with_capacity(len as usize)`   |
| `iastore`           | `arr[idx as usize] = val`                              |
| `iaload`            | `arr[idx as usize]`（越界 → panic 或 Result）          |
| `arraylength`       | `arr.len() as i32`                                     |
| `multianewarray`    | 嵌套 `vec![vec![...]; rows]`                           |

### 4.3 越界处理策略

Java `ArrayIndexOutOfBoundsException` 映射为 Rust `Result`：

```rust
fn array_get<T: Clone>(arr: &[T], idx: i32) -> Result<T, JvmError> {
    arr.get(idx as usize)
       .cloned()
       .ok_or(JvmError::ArrayIndexOutOfBounds(idx))
}

fn array_set<T>(arr: &mut Vec<T>, idx: i32, val: T) -> Result<(), JvmError> {
    let i = idx as usize;
    if i >= arr.len() { return Err(JvmError::ArrayIndexOutOfBounds(idx)); }
    arr[i] = val;
    Ok(())
}
```

### 4.4 Arrays 工具类存根

```rust
// java_runtime/arrays.rs
pub fn arrays_sort_int(arr: &mut Vec<i32>)     { arr.sort(); }
pub fn arrays_sort_double(arr: &mut Vec<f64>)  { arr.sort_by(f64::total_cmp); }
pub fn arrays_fill_int(arr: &mut Vec<i32>, v: i32)     { arr.fill(v); }
pub fn arrays_copy_of_int(arr: &[i32], new_len: i32) -> Vec<i32> {
    let n = new_len as usize;
    let mut result = arr[..arr.len().min(n)].to_vec();
    result.resize(n, 0);
    result
}
pub fn arrays_as_list<T: Clone>(arr: &[T]) -> Vec<T> { arr.to_vec() }
pub fn arrays_binary_search_int(arr: &[i32], key: i32) -> i32 {
    arr.binary_search(&key).map(|i| i as i32).unwrap_or_else(|i| -(i as i32) - 1)
}
```

---

## 5. 集合（List / Set）

### 5.1 ArrayList → Vec\<T\>

RTA 如果确认只有 `ArrayList` 被实例化，直接退化为 `Vec`，无需 trait 对象：

```rust
// Java: List<String> list = new ArrayList<>();
let mut list: Vec<String> = Vec::new();

// Java: list.add("foo")
list.push("foo".to_string());

// Java: list.get(0)
list[0].clone()    // 或 array_get(&list, 0)?

// Java: list.size()
list.len() as i32

// Java: list.remove(0) — 按 index
list.remove(0);

// Java: list.contains("foo")
list.contains(&"foo".to_string())

// Java: list.isEmpty()
list.is_empty()

// Java: for (String s : list)
for s in &list { ... }

// Java: list.clear()
list.clear();

// Java: list.set(0, "bar")
list[0] = "bar".to_string();

// Java: Collections.sort(list)
list.sort();
```

### 5.2 多态 List（RTA 有多个实现时）

当 RTA 发现 `ArrayList` 和 `LinkedList` 都被实例化时，生成 enum dispatch：

```rust
pub enum ListImpl<T> {
    ArrayList(Vec<T>),
    LinkedList(std::collections::VecDeque<T>),
}

impl<T: Clone + PartialEq> ListImpl<T> {
    pub fn add(&mut self, val: T) {
        match self {
            ListImpl::ArrayList(v)  => v.push(val),
            ListImpl::LinkedList(d) => d.push_back(val),
        }
    }
    pub fn get(&self, idx: i32) -> &T {
        match self {
            ListImpl::ArrayList(v)  => &v[idx as usize],
            ListImpl::LinkedList(d) => &d[idx as usize],
        }
    }
    pub fn size(&self) -> i32 {
        match self {
            ListImpl::ArrayList(v)  => v.len() as i32,
            ListImpl::LinkedList(d) => d.len() as i32,
        }
    }
    // ... 其他方法
}
```

### 5.3 HashSet → HashSet\<T\>

```rust
use std::collections::HashSet;

// Java: Set<String> s = new HashSet<>();
let mut s: HashSet<String> = HashSet::new();

// Java: s.add("x")
s.insert("x".to_string());

// Java: s.contains("x")
s.contains("x");

// Java: s.remove("x")
s.remove("x");

// Java: s.size()
s.len() as i32

// Java: for (String x : s)
for x in &s { ... }
```

### 5.4 Collections 工具类存根

```rust
// java_runtime/collections.rs
pub fn collections_sort<T: Ord>(list: &mut Vec<T>) { list.sort(); }

pub fn collections_reverse<T>(list: &mut Vec<T>) { list.reverse(); }

pub fn collections_shuffle<T>(list: &mut Vec<T>) {
    // 接入 rand crate
}

pub fn collections_singleton_list<T>(val: T) -> Vec<T> { vec![val] }

pub fn collections_empty_list<T>() -> Vec<T> { Vec::new() }

pub fn collections_unmodifiable_list<T: Clone>(list: &[T]) -> Vec<T> {
    list.to_vec()  // 简化：返回副本（运行时不强制只读）
}

pub fn collections_frequency<T: PartialEq>(list: &[T], obj: &T) -> i32 {
    list.iter().filter(|x| *x == obj).count() as i32
}
```

---

## 6. Map（HashMap / LinkedHashMap）

### 6.1 HashMap → std::collections::HashMap

```rust
use std::collections::HashMap;

// Java: Map<String, Integer> map = new HashMap<>();
let mut map: HashMap<String, i32> = HashMap::new();

// Java: map.put("key", 42)
map.insert("key".to_string(), 42);

// Java: map.get("key")  → Integer（可为 null）
let val: Option<&i32> = map.get("key");

// Java: map.getOrDefault("key", 0)
let val = map.get("key").copied().unwrap_or(0);

// Java: map.containsKey("key")
map.contains_key("key");

// Java: map.remove("key")
map.remove("key");

// Java: map.size()
map.len() as i32

// Java: map.isEmpty()
map.is_empty()

// Java: map.keySet()
map.keys().cloned().collect::<Vec<_>>()

// Java: map.values()
map.values().cloned().collect::<Vec<_>>()

// Java: map.entrySet() + for loop
for (k, v) in &map { ... }

// Java: map.putIfAbsent("key", 0)
map.entry("key".to_string()).or_insert(0);

// Java: map.computeIfAbsent("key", k -> new ArrayList<>())
map.entry("key".to_string()).or_insert_with(Vec::new);
```

### 6.2 LinkedHashMap → IndexMap

插入顺序有语义时，使用 `indexmap` crate：

```toml
# Cargo.toml
[dependencies]
indexmap = "2"
```

```rust
use indexmap::IndexMap;

let mut map: IndexMap<String, i32> = IndexMap::new();
// API 与 HashMap 完全一致，但迭代按插入顺序
```

### 6.3 TreeMap → BTreeMap

```rust
use std::collections::BTreeMap;

// Java: TreeMap<String, Integer> 按 key 排序
let mut map: BTreeMap<String, i32> = BTreeMap::new();
// 迭代时按 key 的自然序
for (k, v) in &map { ... }
```

### 6.4 多态 Map（RTA 有多个实现时）

```rust
pub enum MapImpl<K, V> {
    HashMap(std::collections::HashMap<K, V>),
    TreeMap(std::collections::BTreeMap<K, V>),
}

impl<K, V> MapImpl<K, V>
where K: std::hash::Hash + Eq + Ord + Clone, V: Clone
{
    pub fn put(&mut self, k: K, v: V) -> Option<V> {
        match self {
            MapImpl::HashMap(m) => m.insert(k, v),
            MapImpl::TreeMap(m) => m.insert(k, v),
        }
    }
    pub fn get(&self, k: &K) -> Option<&V> {
        match self {
            MapImpl::HashMap(m) => m.get(k),
            MapImpl::TreeMap(m) => m.get(k),
        }
    }
    pub fn size(&self) -> i32 {
        match self {
            MapImpl::HashMap(m) => m.len() as i32,
            MapImpl::TreeMap(m) => m.len() as i32,
        }
    }
}
```

---

## 7. 虚方法分派（RTA 核心优化）

### 7.1 生成策略

| RTA 实例化集合大小 | 生成策略 |
|--------------------|----------|
| 0 个具体类         | 调用点不可达，直接删除 |
| 1 个具体类         | 直接静态调用，无分派 |
| 2–8 个具体类       | `match` enum 分派 |
| > 8 个具体类       | `dyn Trait` 动态分派（退化） |

### 7.2 Codegen 示例

```python
# codegen.py 中的虚方法生成逻辑
def gen_virtual_dispatch(interface: str, implementors: list[str], method: MethodSig) -> str:
    n = len(implementors)
    if n == 0:
        return "unreachable!()"
    if n == 1:
        cls = implementors[0]
        return f"{cls}::{method.name}({', '.join(method.params)})"
    
    # 生成 enum
    variants = "\n    ".join(f"{c}({c})" for c in implementors)
    arms = "\n            ".join(
        f"{interface}Impl::{c}(obj) => obj.{method.name}({', '.join(method.params)}),"
        for c in implementors
    )
    return f"""
pub enum {interface}Impl {{
    {variants}
}}
impl {interface}Impl {{
    pub fn {method.name}(&mut self, {method.rust_params()}) -> {method.rust_return()} {{
        match self {{
            {arms}
        }}
    }}
}}"""
```

---

## 8. 运行时存根库结构

```
java_runtime/
├── mod.rs            # pub use 各模块
├── error.rs          # JvmError enum（所有异常类型）
├── object.rs         # Object 基础方法（toString, equals, hashCode）
├── string.rs         # String / StringBuilder 操作
├── math.rs           # java.lang.Math
├── arrays.rs         # java.util.Arrays + 数组操作
├── collections.rs    # java.util.Collections
├── list.rs           # List / ArrayList / LinkedList
├── map.rs            # Map / HashMap / LinkedHashMap / TreeMap
├── set.rs            # Set / HashSet / TreeSet
├── io.rs             # System.out / System.err / PrintStream
└── boxed.rs          # Integer, Long, Double, Boolean 装箱
```

```rust
// error.rs
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ClassCastException(String),
    ArithmeticException(String),  // 除零
    StackOverflowError,
    OutOfMemoryError,
    Custom(String),
}
```

---

## 9. Cargo.toml 依赖

```toml
[package]
name = "java_transpiled"
version = "0.1.0"
edition = "2021"

[dependencies]
indexmap = "2"          # LinkedHashMap 有序 Map
rand     = "0.8"        # Math.random() / Collections.shuffle()

[profile.release]
opt-level = 3
lto       = true        # 链接期优化，进一步裁剪死代码
codegen-units = 1
strip     = "symbols"   # 去除符号表，缩小二进制
```

---

## 10. 实现阶段规划

| 阶段 | 目标 | 关键输出 |
|------|------|----------|
| **P0** | 整数运算 + 静态方法调用 + `println` | `HelloWorld` 能跑 |
| **P1** | 类实例化 + 字段读写 + 构造器 | 简单 OOP 能跑 |
| **P2** | 数组（一维/二维）+ `Arrays` 工具类 | 数组程序能跑 |
| **P3** | `ArrayList` + `HashMap` + `HashSet` | 集合程序能跑 |
| **P4** | 接口 + 虚方法 RTA enum 分派 | 多态程序能跑 |
| **P5** | `try/catch/finally` → `Result` 传播 | 异常处理 |
| **P6** | 浮点 + `Math` 类 | 数学密集型程序 |
| **P7** | 字符串操作 + `StringBuilder` | 字符串处理 |
| **P8** | `LinkedHashMap` / `TreeMap` / `TreeSet` | 有序集合 |
| **P9** | `for-each` / `Iterator` / `Stream` 基础 | 函数式风格 |

---

## 11. 验证方法

每阶段用对应的 Java 测试程序验证：

```bash
# 编译 Java
javac TestMath.java

# RTA 分析
uv run python java_rta.py TestMath.java --json > rta.json

# 生成 Rust（待实现）
uv run python codegen.py rta.json > src/main.rs

# 编译并运行
cargo run --release

# 对比输出
java TestMath > expected.txt
cargo run --release > actual.txt
diff expected.txt actual.txt
```

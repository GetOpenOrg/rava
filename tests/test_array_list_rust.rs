//! Rust vs Java 对比测试 — ArrayList / HashMap / HashSet
//!
//! 对应 Java 文件：tests/TestArrayList.java
//!
//! 本文件同时展示：
//!   [当前写法] — 现在可编译运行的代码
//!   [目标写法] — T37/T38/T39/T40 完成后的 ergonomic 写法（注释状态）
//!
//! 运行方式：
//!   cd output && cargo run --bin test_array_list_rust
//!
//! 与 Java 的不可消除差异（语言本质）：
//!   🔒  "Alice"  →  String::from("Alice")     （Rust 字符串构造语法）
//!   🔒  method() →  method()?                  （Rust 显式错误传播）
//!   🔒  Type x = →  let x: Type =              （Rust 类型声明语法）

#![allow(unused_variables, dead_code, non_snake_case, unused_imports)]
use java_runtime::prelude::*;
use jdk_classes::java::lang::*;
use jdk_classes::java::util::*;
use jdk_classes::java::io::*;

fn main() {
    run().unwrap_or_else(|e| eprintln!("Error: {:?}", e));
}

fn run() -> Result<()> {

    // ════════════════════════════════════════════════════════════════
    // 1. ArrayList<String>
    // ════════════════════════════════════════════════════════════════

    // Java:  ArrayList<String> names = new ArrayList<>();
    //
    // [当前写法]
    let names = ArrayList::<Object>::new_default()?;
    // [目标写法 — T37 后]
    // let names: ArrayList<String> = ArrayList::new()?;

    // Java:  names.add("Alice");
    //
    // [当前写法]
    names.add__obj(String::from("Alice").into())?;
    names.add__obj(String::from("Bob").into())?;
    names.add__obj(String::from("Charlie").into())?;
    // [目标写法 — T37+T39 后]
    // names.add(String::from("Alice"))?;
    // names.add(String::from("Bob"))?;
    // names.add(String::from("Charlie"))?;

    // Java:  System.out.println(names.size());   // 3
    //
    // [当前写法]
    System::out().println__i(names.size()?)?;
    // [目标写法 — T38 后]
    // System::out().println(names.size()?)?;

    // Java:  System.out.println(names.get(0));   // Alice
    //
    // [当前写法]
    System::out().println__obj(names.get(0i32)?)?;
    System::out().println__obj(names.get(1i32)?)?;
    System::out().println__obj(names.get(2i32)?)?;
    // [目标写法 — T37+T38+T39 后]
    // let s: String = names.get(0)?;          // get_item 改名为 get (T37)
    // System::out().println(s)?;              // 统一 println (T38)

    // ════════════════════════════════════════════════════════════════
    // 2. ArrayList<Integer>  autoboxing / unboxing
    // ════════════════════════════════════════════════════════════════

    // Java:  ArrayList<Integer> scores = new ArrayList<>();
    //
    // [当前写法]
    let scores = ArrayList::<Object>::new_default()?;
    // [目标写法 — T37 后]
    // let scores: ArrayList<i32> = ArrayList::new()?;

    // Java:  scores.add(100);   // autoboxing: int → Integer
    //
    // [当前写法]
    scores.add__obj(100i32.into())?;
    scores.add__obj(95i32.into())?;
    scores.add__obj(87i32.into())?;
    // [目标写法 — T37+T39 后]
    // scores.add(100)?;
    // scores.add(95)?;
    // scores.add(87)?;

    // Java:  System.out.println(scores.size());   // 3
    System::out().println__i(scores.size()?)?;

    // Java:  int first = scores.get(0);   // unboxing: Integer → int
    //
    // [当前写法]
    let first: i32 = scores.get(0i32)?.downcast::<i32>();
    System::out().println__i(first)?;
    // [目标写法 — T36+T37+T39 后]
    // let first: i32 = scores.get(0)?;     // get_item 改名为 get (T37)
    // System::out().println(first)?;

    // Java:  System.out.println(scores.get(1));   // 95
    System::out().println__obj(scores.get(1i32)?)?;
    // [目标写法]
    // System::out().println(scores.get(1)?)?;

    // ════════════════════════════════════════════════════════════════
    // 3. HashMap<String, Integer>
    // ════════════════════════════════════════════════════════════════

    // Java:  HashMap<String, Integer> ages = new HashMap<>();
    //
    // [当前写法]
    let ages = HashMap::<Object, Object>::new_default()?;
    // [目标写法 — T37 后]
    // let ages: HashMap<String, i32> = HashMap::new()?;

    // Java:  ages.put("Alice", 30);
    //
    // [当前写法]
    ages.put(String::from("Alice").into(), 30i32.into())?;
    ages.put(String::from("Bob").into(), 25i32.into())?;
    ages.put(String::from("Charlie").into(), 35i32.into())?;
    // [目标写法 — T37+T39 后]
    // ages.put_kv(String::from("Alice"), 30)?;
    // ages.put_kv(String::from("Bob"), 25)?;
    // ages.put_kv(String::from("Charlie"), 35)?;

    // Java:  System.out.println(ages.size());   // 3
    System::out().println__i(ages.size()?)?;

    // Java:  System.out.println(ages.get("Alice"));   // 30
    //
    // [当前写法]
    System::out().println__obj(ages.get(String::from("Alice").into())?)?;
    System::out().println__obj(ages.get(String::from("Bob").into())?)?;
    // [目标写法 — T37+T38+T39 后]
    // let v: i32 = ages.get_value(String::from("Alice"))?;
    // System::out().println(v)?;

    // Java:  System.out.println(ages.containsKey("Charlie"));   // true
    //
    // [当前写法]
    System::out().println__z(ages.containsKey(String::from("Charlie").into())?)?;
    System::out().println__z(ages.containsKey(String::from("Dave").into())?)?;
    // [目标写法 — T37+T38+T39 后]
    // System::out().println(ages.contains_key_e(String::from("Charlie"))?)?;

    // ════════════════════════════════════════════════════════════════
    // 4. HashSet<String>  去重
    // ════════════════════════════════════════════════════════════════

    // Java:  HashSet<String> unique = new HashSet<>();
    //
    // [当前写法]
    let unique = HashSet::<Object>::new_default()?;
    // [目标写法 — T37 后]
    // let unique: HashSet<String> = HashSet::new()?;

    // Java:  unique.add("apple");
    //
    // [当前写法]
    unique.add(String::from("apple").into())?;
    unique.add(String::from("banana").into())?;
    unique.add(String::from("apple").into())?;   // 重复，不加入
    // [目标写法 — T37+T39 后]
    // unique.add_e(String::from("apple"))?;
    // unique.add_e(String::from("banana"))?;
    // unique.add_e(String::from("apple"))?;

    // Java:  System.out.println(unique.size());   // 2
    System::out().println__i(unique.size()?)?;

    // Java:  System.out.println(unique.contains("apple"));   // true
    //
    // [当前写法]
    System::out().println__z(unique.contains(String::from("apple").into())?)?;
    System::out().println__z(unique.contains(String::from("grape").into())?)?;
    // [目标写法 — T37+T38+T39 后]
    // System::out().println(unique.contains_e(String::from("apple"))?)?;

    Ok(())
}

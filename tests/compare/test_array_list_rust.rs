//! Rust vs Java 对比测试 — ArrayList / HashMap / HashSet
//!
//! 对应 Java 文件：tests/TestArrayList.java
//!
//! 本文件同时展示：
//!   [底层写法]   — 生成代码使用的 Object-typed API（*_obj 后缀方法）
//!   [ergonomic] — T39 完成后的类型安全写法（直接使用类型参数，与 Java 高度相似）
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
    // [底层写法]  Object-typed（生成代码使用）
    let names = ArrayList::<Object>::new()?;
    // [ergonomic — T39 完成]  类型安全，与 Java 几乎完全相同
    // let names: ArrayList<String> = ArrayList::new()?;

    // Java:  names.add("Alice");
    //
    // [底层写法]
    names.add_obj(String::from("Alice").into())?;
    names.add_obj(String::from("Bob").into())?;
    names.add_obj(String::from("Charlie").into())?;
    // [ergonomic]  与 Java 一致，无需 .into()
    // let names: ArrayList<String> = ArrayList::new()?;
    // names.add(String::from("Alice"))?;
    // names.add(String::from("Bob"))?;
    // names.add(String::from("Charlie"))?;

    // Java:  System.out.println(names.size());   // 3
    System::out().println_v(names.size()?)?;

    // Java:  System.out.println(names.get(0));   // Alice
    //
    // [底层写法]  get_obj 返回 Object，需要 downcast
    System::out().println_v(names.get_obj(0i32)?.downcast::<String>().clone())?;
    System::out().println_v(names.get_obj(1i32)?.downcast::<String>().clone())?;
    System::out().println_v(names.get_obj(2i32)?.downcast::<String>().clone())?;
    // [ergonomic]  get 直接返回 String，与 Java 完全一致
    // let s: String = names.get(0)?;
    // System::out().println_v(s)?;

    // ════════════════════════════════════════════════════════════════
    // 2. ArrayList<Integer>  autoboxing / unboxing
    // ════════════════════════════════════════════════════════════════

    // Java:  ArrayList<Integer> scores = new ArrayList<>();
    let scores = ArrayList::<Object>::new()?;
    // [ergonomic]
    // let scores: ArrayList<i32> = ArrayList::new()?;

    // Java:  scores.add(100);
    //
    // [底层写法]  T36 boxing：i32 → Object
    scores.add_obj(100i32.into())?;
    scores.add_obj(95i32.into())?;
    scores.add_obj(87i32.into())?;
    // [ergonomic]  与 Java 完全一致
    // scores.add(100)?;
    // scores.add(95)?;
    // scores.add(87)?;

    // Java:  System.out.println(scores.size());   // 3
    System::out().println_v(scores.size()?)?;

    // Java:  int first = scores.get(0);
    //
    // [底层写法]  get_obj + T36 unboxing
    let first: i32 = scores.get_obj(0i32)?.downcast::<i32>();
    System::out().println_v(first)?;
    // [ergonomic]  与 Java 完全一致
    // let first: i32 = scores.get(0)?;
    // System::out().println_v(first)?;

    // Java:  System.out.println(scores.get(1));   // 95
    System::out().println_v(scores.get_obj(1i32)?)?;

    // ════════════════════════════════════════════════════════════════
    // 3. HashMap<String, Integer>
    // ════════════════════════════════════════════════════════════════

    // Java:  HashMap<String, Integer> ages = new HashMap<>();
    let ages = HashMap::<Object, Object>::new()?;
    // [ergonomic]
    // let ages: HashMap<String, i32> = HashMap::new()?;

    // Java:  ages.put("Alice", 30);
    //
    // [底层写法]
    ages.put_obj(String::from("Alice").into(), 30i32.into())?;
    ages.put_obj(String::from("Bob").into(), 25i32.into())?;
    ages.put_obj(String::from("Charlie").into(), 35i32.into())?;
    // [ergonomic]  与 Java 完全一致
    // ages.put(String::from("Alice"), 30)?;
    // ages.put(String::from("Bob"), 25)?;
    // ages.put(String::from("Charlie"), 35)?;

    // Java:  System.out.println(ages.size());   // 3
    System::out().println_v(ages.size()?)?;

    // Java:  System.out.println(ages.get("Alice"));   // 30
    //
    // [底层写法]
    System::out().println_v(ages.get_obj(String::from("Alice").into())?)?;
    System::out().println_v(ages.get_obj(String::from("Bob").into())?)?;
    // [ergonomic]  与 Java 完全一致
    // let v: i32 = ages.get(String::from("Alice"))?;
    // System::out().println_v(v)?;

    // Java:  System.out.println(ages.containsKey("Charlie"));   // true
    //
    // [底层写法]  containsKey 保持 camelCase（JDK 方法名，无重命名）
    System::out().println_v(ages.containsKey(String::from("Charlie").into())?)?;
    System::out().println_v(ages.containsKey(String::from("Dave").into())?)?;
    // [ergonomic]  contains_key（snake_case）与 Java containsKey 对应
    // System::out().println_v(ages.contains_key(String::from("Charlie"))?)?;

    // ════════════════════════════════════════════════════════════════
    // 4. HashSet<String>  去重
    // ════════════════════════════════════════════════════════════════

    // Java:  HashSet<String> unique = new HashSet<>();
    let unique = HashSet::<Object>::new()?;
    // [ergonomic]
    // let unique: HashSet<String> = HashSet::new()?;

    // Java:  unique.add("apple");
    //
    // [底层写法]
    unique.add_obj(String::from("apple").into())?;
    unique.add_obj(String::from("banana").into())?;
    unique.add_obj(String::from("apple").into())?;   // 重复，不加入
    // [ergonomic]  与 Java 完全一致
    // unique.add(String::from("apple"))?;
    // unique.add(String::from("banana"))?;
    // unique.add(String::from("apple"))?;

    // Java:  System.out.println(unique.size());   // 2
    System::out().println_v(unique.size()?)?;

    // Java:  System.out.println(unique.contains("apple"));   // true
    //
    // [底层写法]
    System::out().println_v(unique.contains_obj(String::from("apple").into())?)?;
    System::out().println_v(unique.contains_obj(String::from("grape").into())?)?;
    // [ergonomic]  与 Java 完全一致
    // System::out().println_v(unique.contains(String::from("apple"))?)?;

    Ok(())
}

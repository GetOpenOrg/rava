"""
java_runtime 运行时存根的 Rust 源码常量。
文件树结构镜像 JDK 包路径：
  java_runtime/
  ├── mod.rs
  ├── error.rs
  └── java/
      ├── mod.rs
      ├── lang/
      │   ├── mod.rs
      │   ├── math.rs       ← java.lang.Math
      │   ├── system.rs     ← java.lang.System (println 等)
      │   └── object.rs     ← java.lang.Object
      ├── util/
      │   ├── mod.rs
      │   ├── array_list.rs ← java.util.ArrayList
      │   ├── hash_map.rs   ← java.util.HashMap
      │   └── hash_set.rs   ← java.util.HashSet
      └── io/
          ├── mod.rs
          └── print_stream.rs ← java.io.PrintStream
"""

# key = 相对于 java_runtime/ 的路径，value = 文件内容
RUNTIME_FILES: dict[str, str] = {

    "mod.rs": """\
pub mod error;
pub mod java;
""",

    "error.rs": """\
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBounds(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    Custom(String),
}
""",

    # ── java/ ──────────────────────────────────────────────────
    "java/mod.rs": """\
pub mod lang;
pub mod util;
pub mod io;
""",

    # ── java/lang/ ─────────────────────────────────────────────
    "java/lang/mod.rs": """\
pub mod math;
pub mod system;
pub mod object;
""",

    "java/lang/math.rs": """\
// java.lang.Math native stubs
pub fn abs_i32(a: i32) -> i32   { a.abs() }
pub fn abs_i64(a: i64) -> i64   { a.abs() }
pub fn abs_f64(a: f64) -> f64   { a.abs() }
pub fn max_i32(a: i32, b: i32) -> i32 { a.max(b) }
pub fn min_i32(a: i32, b: i32) -> i32 { a.min(b) }
pub fn max_f64(a: f64, b: f64) -> f64 { a.max(b) }
pub fn min_f64(a: f64, b: f64) -> f64 { a.min(b) }
pub fn sqrt(a: f64) -> f64      { a.sqrt() }
pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
pub fn floor(a: f64) -> f64     { a.floor() }
pub fn ceil(a: f64) -> f64      { a.ceil() }
pub fn round(a: f64) -> i64     { a.round() as i64 }
pub fn log(a: f64) -> f64       { a.ln() }
pub fn log10(a: f64) -> f64     { a.log10() }
pub fn sin(a: f64) -> f64       { a.sin() }
pub fn cos(a: f64) -> f64       { a.cos() }
pub fn tan(a: f64) -> f64       { a.tan() }
""",

    "java/lang/system.rs": """\
// java.lang.System.out / System.err 输出存根
pub fn println_i32(v: i32)    { println!("{}", v); }
pub fn println_i64(v: i64)    { println!("{}", v); }
pub fn println_f64(v: f64)    { println!("{}", v); }
pub fn println_bool(v: bool)  { println!("{}", v); }
pub fn println_str(v: &str)   { println!("{}", v); }
pub fn println_empty()        { println!(); }

pub fn eprintln_i32(v: i32)   { eprintln!("{}", v); }
pub fn eprintln_str(v: &str)  { eprintln!("{}", v); }

pub fn exit(code: i32) -> ! { std::process::exit(code); }
pub fn current_time_millis() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as i64
}
""",

    "java/lang/object.rs": """\
// java.lang.Object 基础行为
pub fn hash_code<T: std::hash::Hash>(v: &T) -> i32 {
    use std::hash::{Hash, Hasher};
    use std::collections::hash_map::DefaultHasher;
    let mut h = DefaultHasher::new();
    v.hash(&mut h);
    h.finish() as i32
}
pub fn equals<T: PartialEq>(a: &T, b: &T) -> bool { a == b }
pub fn to_string_i32(v: i32) -> String { v.to_string() }
pub fn to_string_i64(v: i64) -> String { v.to_string() }
pub fn to_string_f64(v: f64) -> String { v.to_string() }
pub fn to_string_bool(v: bool) -> String { v.to_string() }
""",

    # ── java/util/ ─────────────────────────────────────────────
    "java/util/mod.rs": """\
pub mod array_list;
pub mod hash_map;
pub mod hash_set;
""",

    "java/util/array_list.rs": """\
// java.util.ArrayList<E> 存根（元素类型泛化为 i32）
pub fn new() -> Vec<i32> { Vec::new() }
pub fn add(list: &mut Vec<i32>, v: i32)        { list.push(v); }
pub fn get(list: &[i32], idx: i32) -> i32       { list[idx as usize] }
pub fn set(list: &mut Vec<i32>, idx: i32, v: i32) { list[idx as usize] = v; }
pub fn size(list: &[i32]) -> i32                 { list.len() as i32 }
pub fn remove(list: &mut Vec<i32>, idx: i32)    { list.remove(idx as usize); }
pub fn contains(list: &[i32], v: i32) -> bool   { list.contains(&v) }
pub fn clear(list: &mut Vec<i32>)               { list.clear(); }
pub fn is_empty(list: &[i32]) -> bool           { list.is_empty() }
""",

    "java/util/hash_map.rs": """\
// java.util.HashMap<K,V> 存根（K=String, V=i32）
use std::collections::HashMap;
pub fn new() -> HashMap<String, i32> { HashMap::new() }
pub fn put(map: &mut HashMap<String, i32>, k: String, v: i32) -> Option<i32> {
    map.insert(k, v)
}
pub fn get(map: &HashMap<String, i32>, k: &str) -> i32 {
    map.get(k).copied().unwrap_or(0)
}
pub fn contains_key(map: &HashMap<String, i32>, k: &str) -> bool { map.contains_key(k) }
pub fn remove(map: &mut HashMap<String, i32>, k: &str)           { map.remove(k); }
pub fn size(map: &HashMap<String, i32>) -> i32                    { map.len() as i32 }
pub fn is_empty(map: &HashMap<String, i32>) -> bool               { map.is_empty() }
""",

    "java/util/hash_set.rs": """\
// java.util.HashSet<E> 存根（E=i32）
use std::collections::HashSet;
pub fn new() -> HashSet<i32> { HashSet::new() }
pub fn add(set: &mut HashSet<i32>, v: i32) -> bool  { set.insert(v) }
pub fn contains(set: &HashSet<i32>, v: i32) -> bool { set.contains(&v) }
pub fn remove(set: &mut HashSet<i32>, v: i32) -> bool { set.remove(&v) }
pub fn size(set: &HashSet<i32>) -> i32               { set.len() as i32 }
pub fn is_empty(set: &HashSet<i32>) -> bool          { set.is_empty() }
""",

    # ── java/io/ ───────────────────────────────────────────────
    "java/io/mod.rs": """\
pub mod print_stream;
""",

    "java/io/print_stream.rs": """\
// java.io.PrintStream 存根（对应 System.out / System.err）
pub struct PrintStream { pub is_err: bool }

impl PrintStream {
    pub fn println_i32(&self, v: i32)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_i64(&self, v: i64)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_f64(&self, v: f64)   { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_bool(&self, v: bool) { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println_str(&self, v: &str)  { if self.is_err { eprintln!("{}", v); } else { println!("{}", v); } }
    pub fn println(&self)               { if self.is_err { eprintln!(); }        else { println!(); } }
}
""",
}

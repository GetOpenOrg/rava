"""
java_runtime 运行时存根 — 新模块结构，Java 命名空间同构。

目录结构：
  java_runtime/
  ├── mod.rs           ← re-exports + prelude
  ├── error.rs         ← JvmError, Result
  ├── types.rs         ← Field<T>（内部基础设施）
  └── java/
      ├── mod.rs
      ├── lang/
      │   ├── mod.rs
      │   ├── string.rs   ← java.lang.String
      │   ├── system.rs   ← java.lang.System / PrintStream
      │   └── math.rs     ← java.lang.Math native stubs
      └── util/
          ├── mod.rs
          ├── array_list.rs  ← java.util.ArrayList<T>
          ├── hash_map.rs    ← java.util.HashMap<K,V>
          └── hash_set.rs    ← java.util.HashSet<T>
"""

# key = 相对于 java_runtime/ 的路径，value = 文件内容
RUNTIME_FILES: dict[str, str] = {

    "mod.rs": """\
#![allow(unused_imports)]
pub mod error;
pub mod types;
pub mod java;

pub use error::{JvmError, Result};
pub use types::Field;
pub use java::lang::{String, System};
pub use java::util::{ArrayList, HashMap, HashSet};

/// prelude：生成代码用 `use crate::java_runtime::prelude::*;` 引入所有必要符号。
/// 这会遮蔽 Rust 的 std::string::String——这是预期行为，
/// 用户看到的 String 即 Java 的 String。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::error::{JvmError, Result};
    pub use super::types::Field;
    pub use super::java::lang::{String, System};
    pub use super::java::util::{ArrayList, HashMap, HashSet};
}
""",

    "error.rs": """\
#[derive(Debug)]
pub enum JvmError {
    NullPointerException,
    ArrayIndexOutOfBoundsException(i32),
    ArithmeticException(&'static str),
    ClassCastException,
    StackOverflowError,
    Custom(std::string::String),
}

impl std::fmt::Display for JvmError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, JvmError>;
""",

    "types.rs": """\
/// Field<T>：实例字段封装，支持通过不可变引用修改（Java 字段语义）。
pub struct Field<T>(std::cell::RefCell<T>);

impl<T: Clone> Field<T> {
    pub fn new(v: T) -> Self { Field(std::cell::RefCell::new(v)) }
    pub fn get(&self) -> T  { self.0.borrow().clone() }
    pub fn set(&self, v: T) { *self.0.borrow_mut() = v; }
}

impl<T: Default + Clone> Default for Field<T> {
    fn default() -> Self { Field::new(T::default()) }
}
""",

    # ── java/ ──────────────────────────────────────────────────────────
    "java/mod.rs": """\
pub mod lang;
pub mod util;
""",

    # ── java/lang/ ─────────────────────────────────────────────────────
    "java/lang/mod.rs": """\
pub mod string;
pub mod system;
pub mod math;

pub use string::String;
pub use system::System;
""",

    "java/lang/string.rs": """\
//! java.lang.String 同构类型。
//! 遮蔽 Rust 的 std::string::String，是预期行为。
#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct String(std::string::String);

impl String {
    pub fn new() -> Self { String(std::string::String::new()) }
    pub fn from(s: &str) -> Self { String(s.to_owned()) }
    pub fn from_owned(s: std::string::String) -> Self { String(s) }
    pub fn append(&mut self, s: &String) { self.0.push_str(&s.0); }
    pub fn append_str(&mut self, s: &str)  { self.0.push_str(s); }
    pub fn length(&self) -> i32  { self.0.len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.is_empty() }
    pub fn to_std(&self) -> &str  { &self.0 }
    pub fn concat(&self, other: &String) -> String {
        String(format!("{}{}", self.0, other.0))
    }
    pub fn trim(&self) -> String { String(self.0.trim().to_owned()) }
    pub fn to_upper_case(&self) -> String { String(self.0.to_uppercase()) }
    pub fn to_lower_case(&self) -> String { String(self.0.to_lowercase()) }
    pub fn contains_str(&self, s: &str) -> bool { self.0.contains(s) }
    pub fn char_at(&self, i: i32) -> u16 {
        self.0.chars().nth(i as usize).unwrap_or('\\0') as u16
    }
    pub fn substring(&self, start: i32) -> String {
        String(self.0.chars().skip(start as usize).collect())
    }
    pub fn substring_end(&self, start: i32, end: i32) -> String {
        String(self.0.chars().skip(start as usize).take((end - start) as usize).collect())
    }
    pub fn index_of_str(&self, s: &str) -> i32 {
        self.0.find(s).map(|i| i as i32).unwrap_or(-1)
    }
    pub fn replace_str(&self, old: &str, new: &str) -> String {
        String(self.0.replace(old, new))
    }
    pub fn value_of_i32(v: i32)  -> String { String(v.to_string()) }
    pub fn value_of_i64(v: i64)  -> String { String(v.to_string()) }
    pub fn value_of_f64(v: f64)  -> String { String(v.to_string()) }
    pub fn value_of_bool(v: bool) -> String { String(v.to_string()) }
    pub fn parse_int(&self)  -> i32 { self.0.parse::<i32>().unwrap_or(0) }
    pub fn parse_long(&self) -> i64 { self.0.parse::<i64>().unwrap_or(0) }
}

impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for String {
    fn from(s: &str) -> Self { String(s.to_owned()) }
}

impl From<std::string::String> for String {
    fn from(s: std::string::String) -> Self { String(s) }
}

impl From<i32>  for String { fn from(v: i32)  -> Self { String(v.to_string()) } }
impl From<i64>  for String { fn from(v: i64)  -> Self { String(v.to_string()) } }
impl From<f64>  for String { fn from(v: f64)  -> Self { String(v.to_string()) } }
impl From<bool> for String { fn from(v: bool) -> Self { String(v.to_string()) } }
""",

    "java/lang/system.rs": """\
//! java.lang.System 同构类型
use crate::java_runtime::error::Result;

pub struct System;
pub struct PrintStream { pub is_err: bool }

impl System {
    pub fn out() -> PrintStream { PrintStream { is_err: false } }
    pub fn err() -> PrintStream { PrintStream { is_err: true } }
    pub fn exit(code: i32) -> ! { std::process::exit(code); }
    pub fn current_time_millis() -> i64 {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as i64
    }
}

impl PrintStream {
    pub fn println<T: std::fmt::Display>(&self, v: T) -> Result<()> {
        if self.is_err { eprintln!("{}", v); } else { println!("{}", v); }
        Ok(())
    }
    pub fn println_empty(&self) -> Result<()> {
        if self.is_err { eprintln!(); } else { println!(); }
        Ok(())
    }
    pub fn print<T: std::fmt::Display>(&self, v: T) -> Result<()> {
        if self.is_err { eprint!("{}", v); } else { print!("{}", v); }
        Ok(())
    }
    pub fn flush(&self) -> Result<()> { Ok(()) }
}
""",

    "java/lang/math.rs": """\
//! java.lang.Math native stubs — Rust stdlib OK here
pub fn abs_i32(a: i32) -> i32   { a.wrapping_abs() }
pub fn abs_i64(a: i64) -> i64   { a.wrapping_abs() }
pub fn abs_f64(a: f64) -> f64   { a.abs() }
pub fn sqrt(a: f64) -> f64      { a.sqrt() }
pub fn pow(a: f64, b: f64) -> f64 { a.powf(b) }
pub fn max_i32(a: i32, b: i32) -> i32 { if a > b { a } else { b } }
pub fn min_i32(a: i32, b: i32) -> i32 { if a < b { a } else { b } }
pub fn max_i64(a: i64, b: i64) -> i64 { if a > b { a } else { b } }
pub fn min_i64(a: i64, b: i64) -> i64 { if a < b { a } else { b } }
pub fn max_f64(a: f64, b: f64) -> f64 { if a > b { a } else { b } }
pub fn min_f64(a: f64, b: f64) -> f64 { if a < b { a } else { b } }
pub fn floor(a: f64) -> f64     { a.floor() }
pub fn ceil(a: f64) -> f64      { a.ceil() }
pub fn round(a: f64) -> i64     { a.round() as i64 }
pub fn sin(a: f64) -> f64       { a.sin() }
pub fn cos(a: f64) -> f64       { a.cos() }
pub fn tan(a: f64) -> f64       { a.tan() }
pub fn log(a: f64) -> f64       { a.ln() }
pub fn log10(a: f64) -> f64     { a.log10() }
""",

    # ── java/util/ ─────────────────────────────────────────────────────
    "java/util/mod.rs": """\
pub mod array_list;
pub mod hash_map;
pub mod hash_set;

pub use array_list::ArrayList;
pub use hash_map::HashMap;
pub use hash_set::HashSet;
""",

    "java/util/array_list.rs": """\
//! java.util.ArrayList<T> 同构类型（内部用 Rc<RefCell<Vec<T>>>）
use crate::java_runtime::error::{JvmError, Result};
use std::rc::Rc;
use std::cell::RefCell;

pub struct ArrayList<T>(Rc<RefCell<Vec<T>>>);

impl<T: Clone + 'static> ArrayList<T> {
    pub fn new() -> Result<Self> {
        Ok(ArrayList(Rc::new(RefCell::new(Vec::new()))))
    }
    pub fn add(&self, v: T) -> Result<bool> {
        self.0.borrow_mut().push(v);
        Ok(true)
    }
    pub fn get(&self, i: i32) -> Result<T> {
        self.0.borrow().get(i as usize).cloned()
            .ok_or(JvmError::ArrayIndexOutOfBoundsException(i))
    }
    pub fn set_at(&self, i: i32, v: T) -> Result<T> {
        let mut b = self.0.borrow_mut();
        let old = b.get(i as usize).cloned()
            .ok_or(JvmError::ArrayIndexOutOfBoundsException(i))?;
        b[i as usize] = v;
        Ok(old)
    }
    pub fn size(&self) -> i32 { self.0.borrow().len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.borrow().is_empty() }
    pub fn remove_at(&self, i: i32) { self.0.borrow_mut().remove(i as usize); }
    pub fn clear(&self) { self.0.borrow_mut().clear(); }
    pub fn contains(&self, v: &T) -> bool where T: PartialEq {
        self.0.borrow().contains(v)
    }
    pub fn for_each<F: FnMut(&T)>(&self, mut f: F) {
        for item in self.0.borrow().iter() { f(item); }
    }
}

impl<T> Clone for ArrayList<T> {
    fn clone(&self) -> Self { ArrayList(self.0.clone()) }
}
""",

    "java/util/hash_map.rs": """\
//! java.util.HashMap<K,V> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashMap<K, V>(Rc<RefCell<std::collections::HashMap<K, V>>>);

impl<K: Clone + Eq + std::hash::Hash + 'static, V: Clone + 'static> HashMap<K, V> {
    pub fn new() -> Result<Self> {
        Ok(HashMap(Rc::new(RefCell::new(std::collections::HashMap::new()))))
    }
    pub fn put(&self, k: K, v: V) -> Option<V> {
        self.0.borrow_mut().insert(k, v)
    }
    pub fn get(&self, k: &K) -> Option<V> {
        self.0.borrow().get(k).cloned()
    }
    pub fn get_or_default(&self, k: &K, d: V) -> V {
        self.0.borrow().get(k).cloned().unwrap_or(d)
    }
    pub fn contains_key(&self, k: &K) -> bool { self.0.borrow().contains_key(k) }
    pub fn remove(&self, k: &K) -> Option<V>  { self.0.borrow_mut().remove(k) }
    pub fn size(&self) -> i32  { self.0.borrow().len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.borrow().is_empty() }
}

impl<K, V> Clone for HashMap<K, V> {
    fn clone(&self) -> Self { HashMap(self.0.clone()) }
}
""",

    "java/util/hash_set.rs": """\
//! java.util.HashSet<T> 同构类型
use crate::java_runtime::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashSet<T>(Rc<RefCell<std::collections::HashSet<T>>>);

impl<T: Clone + Eq + std::hash::Hash + 'static> HashSet<T> {
    pub fn new() -> Result<Self> {
        Ok(HashSet(Rc::new(RefCell::new(std::collections::HashSet::new()))))
    }
    pub fn add(&self, v: T) -> bool  { self.0.borrow_mut().insert(v) }
    pub fn contains(&self, v: &T) -> bool { self.0.borrow().contains(v) }
    pub fn remove(&self, v: &T) -> bool   { self.0.borrow_mut().remove(v) }
    pub fn size(&self) -> i32  { self.0.borrow().len() as i32 }
    pub fn is_empty(&self) -> bool { self.0.borrow().is_empty() }
}

impl<T> Clone for HashSet<T> {
    fn clone(&self) -> Self { HashSet(self.0.clone()) }
}
""",
}

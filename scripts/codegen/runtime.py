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
pub use java::lang::Object;
pub use java::util::{ArrayList, HashMap, HashSet};

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::error::{JvmError, Result};
    pub use super::types::Field;
    pub use super::java::lang::Object;
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

impl<T: Clone> Clone for Field<T> {
    fn clone(&self) -> Self { Field::new(self.get()) }
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
pub mod math;
pub mod object;

pub use object::Object;
""",

    "java/lang/object.rs": """\
//! java.lang.Object — 所有 Java 类的根类型

#[doc(hidden)]
pub mod raw {
    use std::rc::Rc;

    /// 存储层：内部路径 crate::java::lang::object::raw::Object
    pub struct Object(pub Rc<dyn std::any::Any>);

    impl Clone for Object {
        fn clone(&self) -> Self {
            Object(self.0.clone())
        }
    }

    impl Default for Object {
        fn default() -> Self {
            Object(Rc::new(()))
        }
    }
}

/// 公开 API：re-export raw::Object 为 java.lang.Object
pub use raw::Object;

use crate::error::Result;

impl Object {
    /// monitorenter — synchronized 块进入（stub，单线程环境无需真正加锁）
    #[allow(non_snake_case)]
    pub fn lock(&self) -> Result<()> { Ok(()) }

    /// monitorexit — synchronized 块退出（stub）
    #[allow(non_snake_case)]
    pub fn unlock(&self) -> Result<()> { Ok(()) }

    /// java.lang.Object.getClass()
    #[allow(non_snake_case)]
    pub fn getClass(&self) -> Result<Object> { Ok(self.clone()) }

    /// java.lang.Object.hashCode()
    #[allow(non_snake_case)]
    pub fn hashCode(&self) -> Result<i32> { Ok(0) }

    /// java.lang.Object.equals(Object)
    #[allow(non_snake_case)]
    pub fn equals(&self, _other: Object) -> Result<bool> { Ok(false) }

    /// java.lang.Object.toString()
    #[allow(non_snake_case)]
    pub fn toString(&self) -> Result<Object> { Ok(self.clone()) }

    /// java.io.PrintStream 内部缓冲刷新（stub）
    #[allow(non_snake_case)]
    pub fn flushBuffer(&self) -> Result<()> { Ok(()) }

    /// null 检查：在转译模型中 Object 永远非 null，始终返回 false
    pub fn is_none(&self) -> bool { false }

    /// Option::get 兼容接口：Object 始终存在，返回 self 的克隆
    pub fn get(&self) -> Result<Object> { Ok(self.clone()) }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Object")
    }
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
use crate::error::{JvmError, Result};
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
    pub fn size(&self) -> Result<i32> { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
    pub fn remove_at(&self, i: i32) -> Result<()> {
        self.0.borrow_mut().remove(i as usize);
        Ok(())
    }
    pub fn clear(&self) -> Result<()> { self.0.borrow_mut().clear(); Ok(()) }
    pub fn contains(&self, v: T) -> Result<bool> where T: PartialEq {
        Ok(self.0.borrow().contains(&v))
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
use crate::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashMap<K, V>(Rc<RefCell<std::collections::HashMap<K, V>>>);

impl<K: Clone + Eq + std::hash::Hash + 'static, V: Clone + 'static> HashMap<K, V> {
    pub fn new() -> Result<Self> {
        Ok(HashMap(Rc::new(RefCell::new(std::collections::HashMap::new()))))
    }
    pub fn put(&self, k: K, v: V) -> Result<Option<V>> {
        Ok(self.0.borrow_mut().insert(k, v))
    }
    pub fn get(&self, k: K) -> Result<V> where V: Default {
        Ok(self.0.borrow().get(&k).cloned().unwrap_or_default())
    }
    pub fn get_or_default(&self, k: K, d: V) -> Result<V> {
        Ok(self.0.borrow().get(&k).cloned().unwrap_or(d))
    }
    pub fn contains_key(&self, k: K) -> Result<bool> {
        Ok(self.0.borrow().contains_key(&k))
    }
    /// Java camelCase alias for contains_key
    #[allow(non_snake_case)]
    pub fn containsKey(&self, k: K) -> Result<bool> {
        self.contains_key(k)
    }
    pub fn remove(&self, k: K) -> Result<Option<V>> {
        Ok(self.0.borrow_mut().remove(&k))
    }
    pub fn size(&self) -> Result<i32>  { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
}

impl<K, V> Clone for HashMap<K, V> {
    fn clone(&self) -> Self { HashMap(self.0.clone()) }
}
""",

    "java/util/hash_set.rs": """\
//! java.util.HashSet<T> 同构类型
use crate::error::Result;
use std::rc::Rc;
use std::cell::RefCell;

pub struct HashSet<T>(Rc<RefCell<std::collections::HashSet<T>>>);

impl<T: Clone + Eq + std::hash::Hash + 'static> HashSet<T> {
    pub fn new() -> Result<Self> {
        Ok(HashSet(Rc::new(RefCell::new(std::collections::HashSet::new()))))
    }
    pub fn add(&self, v: T) -> Result<bool> { Ok(self.0.borrow_mut().insert(v)) }
    pub fn contains(&self, v: T) -> Result<bool> {
        Ok(self.0.borrow().contains(&v))
    }
    pub fn remove(&self, v: T) -> Result<bool> {
        Ok(self.0.borrow_mut().remove(&v))
    }
    pub fn size(&self) -> Result<i32>  { Ok(self.0.borrow().len() as i32) }
    pub fn is_empty(&self) -> Result<bool> { Ok(self.0.borrow().is_empty()) }
    pub fn isEmpty(&self) -> Result<bool> { self.is_empty() }
}

impl<T> Clone for HashSet<T> {
    fn clone(&self) -> Self { HashSet(self.0.clone()) }
}
""",
}

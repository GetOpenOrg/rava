#![allow(unused_imports)]
pub mod array;
pub mod error;
pub mod java;
pub mod jdk;
pub mod sun;

pub use array::JArray;
pub use error::{JvmError, Result};
pub use java::lang::Object;

/// Java 风格浮点数格式化：整数值显示 .0，其他同 Rust 默认格式
pub fn java_fmt_f64(v: f64) -> String {
    if v.is_infinite() {
        if v > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
    } else if v.is_nan() {
        "NaN".to_string()
    } else if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}
pub fn java_fmt_f32(v: f32) -> String {
    if v.is_infinite() {
        if v > 0.0 { "Infinity".to_string() } else { "-Infinity".to_string() }
    } else if v.is_nan() {
        "NaN".to_string()
    } else if v.fract() == 0.0 && v.abs() < 1e15 {
        format!("{:.1}", v)
    } else {
        format!("{}", v)
    }
}

/// 整数除法/取余（JVMS §6.5 idiv / irem / ldiv / lrem）：
/// 除数为 0 抛 `ArithmeticException("/ by zero")`；`MIN / -1` 按二进制补码回绕。
pub fn idiv(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn irem(a: i32, b: i32) -> error::Result<i32> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}
pub fn ldiv(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_div(b))
}
pub fn lrem(a: i64, b: i64) -> error::Result<i64> {
    if b == 0 { return Err(error::JvmError::arithmetic("/ by zero")); }
    Ok(a.wrapping_rem(b))
}

/// JVM null 检查：ifnull/ifnonnull 字节码翻译辅助。
/// Object::default()（内部 vtable = ()）表示 Java null；其他类型始终返回 false。
#[inline(always)]
pub fn _is_jnull<T: 'static>(val: &T) -> bool {
    if let Some(obj) = (val as &dyn std::any::Any).downcast_ref::<Object>() {
        obj.0.as_any().downcast_ref::<()>().is_some()
    } else {
        false
    }
}


/// MutexHolder：包装 parking_lot::ReentrantMutex，为 InternalLock 等需要 PartialEq 的结构体使用
#[derive(Clone)]
pub struct MutexHolder(pub std::sync::Arc<parking_lot::ReentrantMutex<()>>);
impl Default for MutexHolder {
    fn default() -> Self { Self(std::sync::Arc::new(parking_lot::ReentrantMutex::new(()))) }
}
impl PartialEq for MutexHolder {
    fn eq(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
}

// ── 常量目录（JDK `Class.enumConstantDirectory` 的数据面）────────────────────
//
// `java_class!` 宏在类初始化（JVMS §5.5）完成后，为声明了「自身类型 static 字段」
// 的类登记（常量名 → 取值闭包）——Java 枚举常量即该形态（`static final Day MONDAY`，
// 值由 `<clinit>` 写入线程局部存储）。查询侧（手写 `Enum.valueOf`）按类对象的
// binary name（点分）取常量。登记只看结构形态，不感知枚举语义；非枚举类的
// 同形态 static 字段一并登记，无副作用（目录仅被常量名查找消费）。

type ConstantGetter = std::rc::Rc<dyn Fn() -> Result<Object>>;

std::thread_local! {
    static CONSTANT_DIRECTORY: std::cell::RefCell<
        std::collections::HashMap<std::string::String, Vec<(std::string::String, ConstantGetter)>>
    > = std::cell::RefCell::new(std::collections::HashMap::new());
}

/// 登记一个类的常量目录项。`binary_name` 为 JVM binary name（斜线 / $ 形态），
/// 内部归一为点分形态（与 `Class` 对象承载的名字一致）。
pub fn register_constant_directory(binary_name: &str, entries: Vec<(std::string::String, ConstantGetter)>) {
    CONSTANT_DIRECTORY.with(|dir| {
        dir.borrow_mut().insert(binary_name.replace('/', "."), entries);
    });
}

/// 按类名 + 常量名取常量。类未登记（无该形态 static 字段 / 尚未初始化）或
/// 常量不存在 → None；常量取值闭包失败（如 erroneous 类初始化后置访问）→ None。
pub fn lookup_constant(binary_name: &str, constant_name: &str) -> Option<Object> {
    CONSTANT_DIRECTORY.with(|dir| {
        let dir = dir.borrow();
        dir.get(binary_name)?
            .iter()
            .find(|(name, _)| name == constant_name)
            .and_then(|(_, get)| get().ok())
    })
}

/// prelude：生成代码用 `use java_runtime::prelude::*;` 引入所有必要符号。
pub mod prelude {
    #![allow(unused_imports)]
    pub use super::array::JArray;
    pub use super::error::{JvmError, Result};
    pub use super::java::lang::Object;
    pub use super::java::lang::ObjectVTable;
    pub use super::java::lang::Object__clone_base;
    pub use super::java::lang::String;
    pub use super::_is_jnull;
    pub use super::{idiv, irem, ldiv, lrem};

    pub use super::java_fmt_f64;
    pub use super::java_fmt_f32;
    pub use super::{register_constant_directory, lookup_constant};
    pub use std::rc::Rc;
    pub use std::cell::RefCell;
    pub use super::MutexHolder;
    pub use java_rta_macros::{jvm_native, jvm_boundary, jvm_ext};
    pub use java_rta_macros::java_try;
}

//! 渲染器依赖的运行时锚点名（集中登记，供规则 4 的 lint 白名单引用）。
//!
//! 这些名字是 `java_runtime` 手写层暴露的 Rust 载体 / 入口，而非从字节码推导的类名：
//! 空引用、类字面量、字符串常量、数组载体与 checkcast / instanceof 入口的调用形态由
//! 运行时 API 决定。渲染器源码中其余位置不得出现 JDK 类名字面量。

/// 擦除载体（`Object = Rc<dyn ObjectVTable>`）：空引用 `Object::default()`、
/// 装箱 `Object::from(..)`、视图转换 `<T as From<Object>>::from(..)`。
pub const OBJECT: &str = "Object";
/// 字符串常量载体：`String::from("..")` / `String::from_utf16_lit(&[..])`。
pub const STRING: &str = "String";
/// 类字面量载体：`Class::for_class(String::from("<binary>"))`。
pub const CLASS: &str = "Class";
/// JVM 数组载体 `JArray<E>`：checkcast 到数组时按元素类型分派（`try_cast_array::<E>`）。
pub const ARRAY: &str = "JArray";

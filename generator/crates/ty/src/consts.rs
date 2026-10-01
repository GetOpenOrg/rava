//! 语言规范点名的类锚点（本 crate 唯一允许出现 JDK 类名字面量的位置）。
//!
//! CLAUDE.md 原则 4 禁止生成器按类名枚举**库知识**（那部分在
//! `runtime/java_runtime/*.txt` 清单里，由 [`crate::manifest`] 读取）。下列类是
//! JLS / JVMS 本身点名的语言语义，不属于库知识，集中于此作唯一引用点：
//!
//! - [`OBJECT`]：根类（JLS 4.3.2）——Rust 侧直映射为 `Object`，也是一切引用类型的上界；
//! - [`STRING`]：字符串字面量类型（JLS 3.10.5）——`java_runtime::prelude` 的同名本主；
//! - [`CLASS`]：类字面量类型（JLS 15.8.2）——其类型参数是纯 phantom，类级签名被置空、
//!   签名解析直映射为裸 `Class`；
//! - [`CLONEABLE`] / [`SERIALIZABLE`]：数组类型的固定超接口（JLS 4.10.3）；
//! - [`THROWABLE`]：异常类层次的根（JLS 11.1.1）——`athrow` 操作数与 catch-any 处理器
//!   绑定的静态类型（JVMS §4.7.3 catch_type 为 0）；
//! - [`BOXED_BY_DESC`]：基本类型的装箱类（JLS 5.1.7 装箱转换点名的 8 个包装类）。
//!
//! 其余模块不得再写这些名字的字面量。

/// 根类
pub const OBJECT: &str = "java/lang/Object";
/// 字符串类
pub const STRING: &str = "java/lang/String";
/// 类字面量类
pub const CLASS: &str = "java/lang/Class";
/// 数组超接口之一
pub const CLONEABLE: &str = "java/lang/Cloneable";
/// 数组超接口之一
pub const SERIALIZABLE: &str = "java/io/Serializable";

/// 异常类层次的根（catch-any 绑定类型）
pub const THROWABLE: &str = "java/lang/Throwable";

/// 数组类型的全部固定超类型（JLS 4.10.3）
pub const ARRAY_SUPERTYPES: [&str; 3] = [OBJECT, CLONEABLE, SERIALIZABLE];

/// MethodParameters 的 ACC_MANDATED（隐式声明的形参，如内部类构造器的外部实例）
pub const ACC_MANDATED: u16 = 0x8000;

/// 基本类型描述符字符 → 装箱类（JLS 5.1.7；按描述符字符排序）
pub const BOXED_BY_DESC: [(u8, &str); 8] = [
    (b'B', "java/lang/Byte"),
    (b'C', "java/lang/Character"),
    (b'D', "java/lang/Double"),
    (b'F', "java/lang/Float"),
    (b'I', "java/lang/Integer"),
    (b'J', "java/lang/Long"),
    (b'S', "java/lang/Short"),
    (b'Z', "java/lang/Boolean"),
];

/// 基本类型描述符字符的装箱类（非基本类型描述符 → None）
pub fn boxed_class(desc: u8) -> Option<&'static str> {
    BOXED_BY_DESC.iter().find(|(d, _)| *d == desc).map(|(_, c)| *c)
}

//! 发射层的语言锚点（本 crate 唯一允许出现 JDK 类名 / 包名字面量的位置）。
//!
//! 与 [`ty::consts`] 同一口径：只收 JLS / JVMS 点名的语言语义（异常根类、基本类型装箱类、
//! JDK 命名空间前缀），不收库知识。`emit` 其余模块经本模块引用，JDK 类名字面量 lint
//! （`tests/lint.rs`）豁免本文件。后续统一收归 `ty::consts`（本轮不改其它 crate）。

pub use ty::consts::{CLASS, OBJECT, SERIALIZABLE, STRING};

/// 异常根类（JLS 11.1.1）：catch-any 处理器的绑定类型
pub const THROWABLE: &str = "java/lang/Throwable";

/// record 类的隐式超类（JLS 8.10）
pub const RECORD: &str = "java/lang/Record";

/// 基本类型描述符 → 装箱类（JLS 5.1.7）
pub const BOXED_CLASS_BY_DESC: [(&str, &str); 8] = [
    ("B", "java/lang/Byte"),
    ("C", "java/lang/Character"),
    ("D", "java/lang/Double"),
    ("F", "java/lang/Float"),
    ("I", "java/lang/Integer"),
    ("J", "java/lang/Long"),
    ("S", "java/lang/Short"),
    ("Z", "java/lang/Boolean"),
];

/// 基本类型描述符的装箱类
pub fn boxed_class(desc: &str) -> Option<&'static str> {
    BOXED_CLASS_BY_DESC.iter().find(|(d, _)| *d == desc).map(|(_, c)| *c)
}

/// JDK 命名空间前缀（真前缀判定；lib crate 类不属于 JDK）
pub const JDK_NAMESPACES: [&str; 9] =
    ["java/", "javax/", "jdk/", "sun/", "com/sun/", "com/oracle/", "org/xml/", "org/w3c/", "org/ietf/"];

/// 类是否位于 JDK 命名空间
pub fn in_jdk_namespace(binary: &str) -> bool {
    JDK_NAMESPACES.iter().any(|p| binary.starts_with(p))
}

/// 公开 API 命名空间（FS-H0：此处手写只许 native）
pub const PUBLIC_API_NAMESPACES: [&str; 2] = ["java/", "javax/"];

/// 类是否位于公开 API 命名空间
pub fn in_public_api(binary: &str) -> bool {
    PUBLIC_API_NAMESPACES.iter().any(|p| binary.starts_with(p))
}

/// `Object.toString()` 描述符（存根给 vtable 可安全调用的默认值）
pub const TO_STRING_DESC: &str = "()Ljava/lang/String;";

/// 程序入口 `main(String[])` 描述符（JLS 12.1.4）
pub const MAIN_DESC: &str = "([Ljava/lang/String;)V";

//! `.class` 解析（JVMS 第 4 章）与类档案读取（jmod / jar / 目录）。
//!
//! 指令操作数结构化（符号引用、常量、分支目标
//! 解码期解出），不产出 javap 风格注释串——下游分析按结构匹配，不做字符串解析。

pub mod archive;
pub mod class;
pub mod constant;
pub mod descriptor;
pub mod extras;
pub mod insn;
pub mod module;
pub mod names;
pub mod reader;
pub mod sha256;

pub use class::{acc, parse, Annotation, BootstrapMethod, ClassFile, Code, ElementValue, ExceptionEntry, Field, Method};
pub use constant::{Const, MemberRef, MethodHandle};
pub use insn::{op, Insn, Operand};

/// 注解（`@` 元素值）与数组元素值的嵌套深度上限。JVMS 未规定上限；注解类型不能直接或间接以自身为
/// 成员类型（JLS §9.6.1），合法嵌套深度不超过注解类型链长度，实际远小于此值。超限视为畸形输入，
/// 防止不可信 .class 以深嵌套耗尽解析栈。
pub const MAX_ANNOTATION_DEPTH: usize = 256;

#[derive(Debug)]
pub enum Error {
    BadMagic,
    Truncated(usize),
    BadConstantTag(u8, usize),
    BadIndex(u16),
    BadOpcode(u8, u32),
    BadCode(u32),
    BadAnnotationTag(u8),
    /// 注解 / 元素值嵌套超过 [`MAX_ANNOTATION_DEPTH`]
    AnnotationTooDeep(usize),
    Io(String, String),
    /// 档案（路径）内的类名不能安全用作路径分量（见 [`names::is_valid_binary_name`]）
    BadClassName(String, String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::BadMagic => write!(f, "魔数不是 0xCAFEBABE"),
            Error::Truncated(p) => write!(f, "数据在偏移 {p} 处截断"),
            Error::BadConstantTag(t, i) => write!(f, "常量池 #{i} 的 tag {t} 非法"),
            Error::BadIndex(i) => write!(f, "常量池索引 #{i} 类型不符"),
            Error::BadOpcode(o, pc) => write!(f, "pc {pc} 处非法 opcode 0x{o:02x}"),
            Error::BadCode(pc) => write!(f, "pc {pc} 处 switch 表非法"),
            Error::BadAnnotationTag(t) => write!(f, "注解元素 tag {} 非法", *t as char),
            Error::AnnotationTooDeep(d) => write!(f, "注解嵌套深度超过 {d}"),
            Error::Io(p, e) => write!(f, "{p}: {e}"),
            Error::BadClassName(p, n) => write!(f, "{p}: 类名 `{n}` 的段须为合法 Java 标识符（不得为空 / . / ..）"),
        }
    }
}

impl std::error::Error for Error {}

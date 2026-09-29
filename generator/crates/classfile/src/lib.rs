//! `.class` 解析（JVMS 第 4 章）与类档案读取（jmod / jar / 目录）。
//!
//! 与 `codegen/classfile.py` 的区别：指令操作数结构化（符号引用、常量、分支目标
//! 解码期解出），不产出 javap 风格注释串——下游分析按结构匹配，不做字符串解析。

pub mod archive;
pub mod class;
pub mod constant;
pub mod descriptor;
pub mod insn;
pub mod reader;

pub use class::{acc, parse, Annotation, BootstrapMethod, ClassFile, Code, ElementValue, ExceptionEntry, Field, Method};
pub use constant::{Const, MemberRef, MethodHandle};
pub use insn::{op, Insn, Operand};

#[derive(Debug)]
pub enum Error {
    BadMagic,
    Truncated(usize),
    BadConstantTag(u8, usize),
    BadIndex(u16),
    BadOpcode(u8, u32),
    BadCode(u32),
    BadAnnotationTag(u8),
    Io(String, String),
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
            Error::Io(p, e) => write!(f, "{p}: {e}"),
        }
    }
}

impl std::error::Error for Error {}

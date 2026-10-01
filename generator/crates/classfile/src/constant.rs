//! 常量池（JVMS §4.4）：解析后保持索引寻址，符号引用按需解出。

use crate::reader::{decode_mutf8, Reader};
use crate::Error;

#[derive(Debug, Clone)]
pub enum CpEntry {
    /// 索引 0 与 long/double 的第二槽
    Unusable,
    /// 文本 + 含孤立代理项时的无损 UTF-16 码元（见 `decode_mutf8`）
    Utf8(String, Option<Box<[u16]>>),
    Integer(i32),
    Float(u32),
    Long(i64),
    Double(u64),
    Class(u16),
    String(u16),
    FieldRef(u16, u16),
    MethodRef(u16, u16),
    InterfaceMethodRef(u16, u16),
    NameAndType(u16, u16),
    MethodHandle(u8, u16),
    MethodType(u16),
    Dynamic(u16, u16),
    InvokeDynamic(u16, u16),
    Module(u16),
    Package(u16),
}

/// 字段 / 方法符号引用（常量池类 + 名字 + 描述符）。
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MemberRef {
    pub owner: String,
    pub name: String,
    pub desc: String,
}

impl std::fmt::Display for MemberRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}:{}", self.owner, self.name, self.desc)
    }
}

/// 方法句柄（JVMS §4.4.8）：reference_kind 1–9。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MethodHandle {
    pub kind: u8,
    pub member: MemberRef,
    /// 引用项为 InterfaceMethodref
    pub interface: bool,
}

/// 可装载常量（ldc 操作数 / bootstrap 静态实参 / ConstantValue）。
#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(i32),
    Float(u32),
    Long(i64),
    Double(u64),
    String(String),
    /// 含孤立代理项的字符串常量：UTF-16 码元（Rust String 无法表示；成员名匹配等按名用途不匹配它）
    StringUtf16(Vec<u16>),
    /// 类字面量：binary name 或数组描述符
    Class(String),
    MethodType(String),
    MethodHandle(MethodHandle),
    /// CONSTANT_Dynamic：(bootstrap 索引, 名字, 描述符)
    Dynamic(u16, String, String),
}

pub struct ConstantPool {
    entries: Vec<CpEntry>,
}

impl ConstantPool {
    pub fn parse(r: &mut Reader) -> Result<Self, Error> {
        let count = r.u2()? as usize;
        let mut entries = Vec::with_capacity(count);
        entries.push(CpEntry::Unusable);
        let mut i = 1;
        while i < count {
            let tag = r.u1()?;
            let e = match tag {
                1 => {
                    let len = r.u2()? as usize;
                    let (text, units) = decode_mutf8(r.bytes(len)?);
                    CpEntry::Utf8(text, units)
                }
                3 => CpEntry::Integer(r.i4()?),
                4 => CpEntry::Float(r.u4()?),
                5 => CpEntry::Long(r.u8_()? as i64),
                6 => CpEntry::Double(r.u8_()?),
                7 => CpEntry::Class(r.u2()?),
                8 => CpEntry::String(r.u2()?),
                9 => CpEntry::FieldRef(r.u2()?, r.u2()?),
                10 => CpEntry::MethodRef(r.u2()?, r.u2()?),
                11 => CpEntry::InterfaceMethodRef(r.u2()?, r.u2()?),
                12 => CpEntry::NameAndType(r.u2()?, r.u2()?),
                15 => CpEntry::MethodHandle(r.u1()?, r.u2()?),
                16 => CpEntry::MethodType(r.u2()?),
                17 => CpEntry::Dynamic(r.u2()?, r.u2()?),
                18 => CpEntry::InvokeDynamic(r.u2()?, r.u2()?),
                19 => CpEntry::Module(r.u2()?),
                20 => CpEntry::Package(r.u2()?),
                t => return Err(Error::BadConstantTag(t, i)),
            };
            let wide = matches!(e, CpEntry::Long(_) | CpEntry::Double(_));
            entries.push(e);
            i += 1;
            if wide {
                entries.push(CpEntry::Unusable);
                i += 1;
            }
        }
        Ok(ConstantPool { entries })
    }

    pub fn get(&self, idx: u16) -> Result<&CpEntry, Error> {
        self.entries.get(idx as usize).ok_or(Error::BadIndex(idx))
    }

    pub fn utf8(&self, idx: u16) -> Result<&str, Error> {
        match self.get(idx)? {
            CpEntry::Utf8(s, _) => Ok(s),
            _ => Err(Error::BadIndex(idx)),
        }
    }

    /// CONSTANT_Class → 名字（binary name 或数组描述符）
    pub fn class_name(&self, idx: u16) -> Result<&str, Error> {
        match self.get(idx)? {
            CpEntry::Class(n) => self.utf8(*n),
            _ => Err(Error::BadIndex(idx)),
        }
    }

    pub fn name_and_type(&self, idx: u16) -> Result<(&str, &str), Error> {
        match self.get(idx)? {
            CpEntry::NameAndType(n, d) => Ok((self.utf8(*n)?, self.utf8(*d)?)),
            _ => Err(Error::BadIndex(idx)),
        }
    }

    /// Fieldref / Methodref / InterfaceMethodref → (引用, 是否接口方法引用)
    pub fn member_ref(&self, idx: u16) -> Result<(MemberRef, bool), Error> {
        let (c, nt, iface) = match self.get(idx)? {
            CpEntry::FieldRef(c, nt) | CpEntry::MethodRef(c, nt) => (*c, *nt, false),
            CpEntry::InterfaceMethodRef(c, nt) => (*c, *nt, true),
            _ => return Err(Error::BadIndex(idx)),
        };
        let owner = self.class_name(c)?.to_string();
        let (name, desc) = self.name_and_type(nt)?;
        Ok((MemberRef { owner, name: name.to_string(), desc: desc.to_string() }, iface))
    }

    pub fn method_handle(&self, idx: u16) -> Result<MethodHandle, Error> {
        match self.get(idx)? {
            CpEntry::MethodHandle(kind, r) => {
                let (member, interface) = self.member_ref(*r)?;
                Ok(MethodHandle { kind: *kind, member, interface })
            }
            _ => Err(Error::BadIndex(idx)),
        }
    }

    /// 字符串常量（ldc / ConstantValue）：含孤立代理项时取无损码元
    pub fn string_const(&self, idx: u16) -> Result<Const, Error> {
        match self.get(idx)? {
            CpEntry::Utf8(_, Some(units)) => Ok(Const::StringUtf16(units.to_vec())),
            CpEntry::Utf8(s, None) => Ok(Const::String(s.clone())),
            _ => Err(Error::BadIndex(idx)),
        }
    }

    /// 可装载常量（JVMS §4.4 表 4.4-C）
    pub fn loadable(&self, idx: u16) -> Result<Const, Error> {
        Ok(match self.get(idx)? {
            CpEntry::Integer(v) => Const::Int(*v),
            CpEntry::Float(v) => Const::Float(*v),
            CpEntry::Long(v) => Const::Long(*v),
            CpEntry::Double(v) => Const::Double(*v),
            CpEntry::String(s) => self.string_const(*s)?,
            CpEntry::Class(n) => Const::Class(self.utf8(*n)?.to_string()),
            CpEntry::MethodType(d) => Const::MethodType(self.utf8(*d)?.to_string()),
            CpEntry::MethodHandle(..) => Const::MethodHandle(self.method_handle(idx)?),
            CpEntry::Dynamic(bsm, nt) => {
                let (n, d) = self.name_and_type(*nt)?;
                Const::Dynamic(*bsm, n.to_string(), d.to_string())
            }
            _ => return Err(Error::BadIndex(idx)),
        })
    }
}

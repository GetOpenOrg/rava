//! ClassFile 结构（JVMS §4.1）及分析所需属性。

use crate::constant::{Const, ConstantPool, MethodHandle};
use crate::insn::{decode, Insn};
use crate::reader::Reader;
use crate::Error;

pub mod acc {
    pub const PUBLIC: u16 = 0x0001;
    pub const PRIVATE: u16 = 0x0002;
    pub const PROTECTED: u16 = 0x0004;
    pub const STATIC: u16 = 0x0008;
    pub const FINAL: u16 = 0x0010;
    pub const SUPER: u16 = 0x0020;
    pub const BRIDGE: u16 = 0x0040;
    pub const VARARGS: u16 = 0x0080;
    pub const NATIVE: u16 = 0x0100;
    pub const INTERFACE: u16 = 0x0200;
    pub const ABSTRACT: u16 = 0x0400;
    pub const SYNTHETIC: u16 = 0x1000;
    pub const ANNOTATION: u16 = 0x2000;
    pub const ENUM: u16 = 0x4000;
}

#[derive(Debug, Clone)]
pub struct ExceptionEntry {
    pub start: u32,
    pub end: u32,
    pub handler: u32,
    /// None = catch-all（finally）
    pub catch_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub max_stack: u16,
    pub max_locals: u16,
    pub insns: Vec<Insn>,
    pub exception_table: Vec<ExceptionEntry>,
}

/// 注解元素值（JVMS §4.7.16.1）
#[derive(Debug, Clone, PartialEq)]
pub enum ElementValue {
    /// 基本类型 / String 常量：tag 为 B C D F I J S Z s
    Const(u8, Const),
    Enum { type_desc: String, name: String },
    /// 类字面量描述符（`V` / `Ljava/lang/String;` / 数组）
    Class(String),
    Annotation(Annotation),
    Array(Vec<ElementValue>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    /// 字段描述符形态（`Ljava/lang/Deprecated;`）
    pub type_desc: String,
    pub elements: Vec<(String, ElementValue)>,
}

#[derive(Debug, Clone)]
pub struct Field {
    pub access: u16,
    pub name: String,
    pub desc: String,
    pub signature: Option<String>,
    pub constant_value: Option<Const>,
    pub annotations: Vec<Annotation>,
}

impl Field {
    pub fn is_static(&self) -> bool {
        self.access & acc::STATIC != 0
    }
}

#[derive(Debug, Clone)]
pub struct Method {
    pub access: u16,
    pub name: String,
    pub desc: String,
    pub signature: Option<String>,
    pub code: Option<Code>,
    pub exceptions: Vec<String>,
    pub annotations: Vec<Annotation>,
    pub annotation_default: Option<ElementValue>,
}

impl Method {
    pub fn is_static(&self) -> bool {
        self.access & acc::STATIC != 0
    }
    pub fn is_abstract(&self) -> bool {
        self.access & acc::ABSTRACT != 0
    }
    pub fn is_native(&self) -> bool {
        self.access & acc::NATIVE != 0
    }
    pub fn is_private(&self) -> bool {
        self.access & acc::PRIVATE != 0
    }
    pub fn is_final(&self) -> bool {
        self.access & acc::FINAL != 0
    }
    pub fn is_init(&self) -> bool {
        self.name == "<init>"
    }
    pub fn is_clinit(&self) -> bool {
        self.name == "<clinit>"
    }
}

#[derive(Debug, Clone)]
pub struct BootstrapMethod {
    pub handle: MethodHandle,
    pub args: Vec<Const>,
}

#[derive(Debug, Clone)]
pub struct InnerClass {
    pub inner: String,
    pub outer: Option<String>,
    pub simple_name: Option<String>,
    pub access: u16,
}

#[derive(Debug, Clone)]
pub struct ClassFile {
    pub minor: u16,
    pub major: u16,
    pub access: u16,
    pub name: String,
    /// java/lang/Object 与 module-info 为 None
    pub super_name: Option<String>,
    pub interfaces: Vec<String>,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub signature: Option<String>,
    pub source_file: Option<String>,
    pub bootstrap_methods: Vec<BootstrapMethod>,
    pub inner_classes: Vec<InnerClass>,
    pub enclosing_method: Option<(String, Option<(String, String)>)>,
    pub nest_host: Option<String>,
    pub nest_members: Vec<String>,
    pub permitted_subclasses: Vec<String>,
    /// Record 组件：(名字, 描述符)
    pub record_components: Option<Vec<(String, String)>>,
    pub annotations: Vec<Annotation>,
}

impl ClassFile {
    pub fn is_interface(&self) -> bool {
        self.access & acc::INTERFACE != 0
    }
    pub fn is_abstract(&self) -> bool {
        self.access & acc::ABSTRACT != 0
    }
    pub fn method(&self, name: &str, desc: &str) -> Option<&Method> {
        self.methods.iter().find(|m| m.name == name && m.desc == desc)
    }
    pub fn field(&self, name: &str, desc: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name && f.desc == desc)
    }
    pub fn field_named(&self, name: &str) -> Option<&Field> {
        self.fields.iter().find(|f| f.name == name)
    }
}

pub fn parse(data: &[u8]) -> Result<ClassFile, Error> {
    let mut r = Reader::new(data);
    if r.u4()? != 0xCAFE_BABE {
        return Err(Error::BadMagic);
    }
    let minor = r.u2()?;
    let major = r.u2()?;
    let pool = ConstantPool::parse(&mut r)?;
    let access = r.u2()?;
    let name = pool.class_name(r.u2()?)?.to_string();
    let super_idx = r.u2()?;
    let super_name = if super_idx == 0 { None } else { Some(pool.class_name(super_idx)?.to_string()) };
    let n_if = r.u2()?;
    let mut interfaces = Vec::with_capacity(n_if as usize);
    for _ in 0..n_if {
        interfaces.push(pool.class_name(r.u2()?)?.to_string());
    }

    let n_fields = r.u2()?;
    let mut fields = Vec::with_capacity(n_fields as usize);
    for _ in 0..n_fields {
        let access = r.u2()?;
        let name = pool.utf8(r.u2()?)?.to_string();
        let desc = pool.utf8(r.u2()?)?.to_string();
        let mut f = Field { access, name, desc, signature: None, constant_value: None, annotations: vec![] };
        let n_attr = r.u2()?;
        for _ in 0..n_attr {
            let (aname, body) = attribute(&mut r, &pool)?;
            let mut ar = Reader::new(body);
            match aname {
                "ConstantValue" => f.constant_value = Some(pool.loadable(ar.u2()?)?),
                "Signature" => f.signature = Some(pool.utf8(ar.u2()?)?.to_string()),
                "RuntimeVisibleAnnotations" => f.annotations = annotations(&mut ar, &pool)?,
                _ => {}
            }
        }
        fields.push(f);
    }

    // Code 属性依赖 BootstrapMethods 之外的全部信息已就绪；BootstrapMethods 在类属性里，
    // 指令里只保留 bootstrap 索引，不需要先行解析
    let n_methods = r.u2()?;
    let mut methods = Vec::with_capacity(n_methods as usize);
    for _ in 0..n_methods {
        let access = r.u2()?;
        let name = pool.utf8(r.u2()?)?.to_string();
        let desc = pool.utf8(r.u2()?)?.to_string();
        let mut m = Method {
            access,
            name,
            desc,
            signature: None,
            code: None,
            exceptions: vec![],
            annotations: vec![],
            annotation_default: None,
        };
        let n_attr = r.u2()?;
        for _ in 0..n_attr {
            let (aname, body) = attribute(&mut r, &pool)?;
            let mut ar = Reader::new(body);
            match aname {
                "Code" => m.code = Some(code(&mut ar, &pool)?),
                "Signature" => m.signature = Some(pool.utf8(ar.u2()?)?.to_string()),
                "Exceptions" => {
                    let n = ar.u2()?;
                    for _ in 0..n {
                        m.exceptions.push(pool.class_name(ar.u2()?)?.to_string());
                    }
                }
                "RuntimeVisibleAnnotations" => m.annotations = annotations(&mut ar, &pool)?,
                "AnnotationDefault" => m.annotation_default = Some(element_value(&mut ar, &pool)?),
                _ => {}
            }
        }
        methods.push(m);
    }

    let mut cf = ClassFile {
        minor,
        major,
        access,
        name,
        super_name,
        interfaces,
        fields,
        methods,
        signature: None,
        source_file: None,
        bootstrap_methods: vec![],
        inner_classes: vec![],
        enclosing_method: None,
        nest_host: None,
        nest_members: vec![],
        permitted_subclasses: vec![],
        record_components: None,
        annotations: vec![],
    };
    let n_attr = r.u2()?;
    for _ in 0..n_attr {
        let (aname, body) = attribute(&mut r, &pool)?;
        let mut ar = Reader::new(body);
        match aname {
            "Signature" => cf.signature = Some(pool.utf8(ar.u2()?)?.to_string()),
            "SourceFile" => cf.source_file = Some(pool.utf8(ar.u2()?)?.to_string()),
            "BootstrapMethods" => {
                let n = ar.u2()?;
                for _ in 0..n {
                    let handle = pool.method_handle(ar.u2()?)?;
                    let na = ar.u2()?;
                    let mut args = Vec::with_capacity(na as usize);
                    for _ in 0..na {
                        args.push(pool.loadable(ar.u2()?)?);
                    }
                    cf.bootstrap_methods.push(BootstrapMethod { handle, args });
                }
            }
            "InnerClasses" => {
                let n = ar.u2()?;
                for _ in 0..n {
                    let inner = pool.class_name(ar.u2()?)?.to_string();
                    let o = ar.u2()?;
                    let s = ar.u2()?;
                    let access = ar.u2()?;
                    cf.inner_classes.push(InnerClass {
                        inner,
                        outer: if o == 0 { None } else { Some(pool.class_name(o)?.to_string()) },
                        simple_name: if s == 0 { None } else { Some(pool.utf8(s)?.to_string()) },
                        access,
                    });
                }
            }
            "EnclosingMethod" => {
                let c = pool.class_name(ar.u2()?)?.to_string();
                let nt = ar.u2()?;
                let m = if nt == 0 {
                    None
                } else {
                    let (n, d) = pool.name_and_type(nt)?;
                    Some((n.to_string(), d.to_string()))
                };
                cf.enclosing_method = Some((c, m));
            }
            "NestHost" => cf.nest_host = Some(pool.class_name(ar.u2()?)?.to_string()),
            "NestMembers" => cf.nest_members = class_list(&mut ar, &pool)?,
            "PermittedSubclasses" => cf.permitted_subclasses = class_list(&mut ar, &pool)?,
            "Record" => {
                let n = ar.u2()?;
                let mut comps = Vec::with_capacity(n as usize);
                for _ in 0..n {
                    let cn = pool.utf8(ar.u2()?)?.to_string();
                    let cd = pool.utf8(ar.u2()?)?.to_string();
                    let na = ar.u2()?;
                    for _ in 0..na {
                        attribute(&mut ar, &pool)?;
                    }
                    comps.push((cn, cd));
                }
                cf.record_components = Some(comps);
            }
            "RuntimeVisibleAnnotations" => cf.annotations = annotations(&mut ar, &pool)?,
            _ => {}
        }
    }
    Ok(cf)
}

fn attribute<'a>(r: &mut Reader<'a>, pool: &'a ConstantPool) -> Result<(&'a str, &'a [u8]), Error> {
    let name = pool.utf8(r.u2()?)?;
    let len = r.u4()? as usize;
    Ok((name, r.bytes(len)?))
}

fn class_list(r: &mut Reader, pool: &ConstantPool) -> Result<Vec<String>, Error> {
    let n = r.u2()?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(pool.class_name(r.u2()?)?.to_string());
    }
    Ok(v)
}

fn code(r: &mut Reader, pool: &ConstantPool) -> Result<Code, Error> {
    let max_stack = r.u2()?;
    let max_locals = r.u2()?;
    let len = r.u4()? as usize;
    let insns = decode(r.bytes(len)?, pool)?;
    let n = r.u2()?;
    let mut exception_table = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let start = r.u2()? as u32;
        let end = r.u2()? as u32;
        let handler = r.u2()? as u32;
        let ct = r.u2()?;
        exception_table.push(ExceptionEntry {
            start,
            end,
            handler,
            catch_type: if ct == 0 { None } else { Some(pool.class_name(ct)?.to_string()) },
        });
    }
    // Code 的子属性（行号表、局部变量表、StackMapTable）闭包分析不需要
    Ok(Code { max_stack, max_locals, insns, exception_table })
}

fn annotations(r: &mut Reader, pool: &ConstantPool) -> Result<Vec<Annotation>, Error> {
    let n = r.u2()?;
    let mut v = Vec::with_capacity(n as usize);
    for _ in 0..n {
        v.push(annotation(r, pool)?);
    }
    Ok(v)
}

fn annotation(r: &mut Reader, pool: &ConstantPool) -> Result<Annotation, Error> {
    let type_desc = pool.utf8(r.u2()?)?.to_string();
    let n = r.u2()?;
    let mut elements = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let name = pool.utf8(r.u2()?)?.to_string();
        elements.push((name, element_value(r, pool)?));
    }
    Ok(Annotation { type_desc, elements })
}

fn element_value(r: &mut Reader, pool: &ConstantPool) -> Result<ElementValue, Error> {
    let tag = r.u1()?;
    Ok(match tag {
        b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' => ElementValue::Const(tag, pool.loadable(r.u2()?)?),
        b's' => ElementValue::Const(tag, Const::String(pool.utf8(r.u2()?)?.to_string())),
        b'e' => {
            let type_desc = pool.utf8(r.u2()?)?.to_string();
            let name = pool.utf8(r.u2()?)?.to_string();
            ElementValue::Enum { type_desc, name }
        }
        b'c' => ElementValue::Class(pool.utf8(r.u2()?)?.to_string()),
        b'@' => ElementValue::Annotation(annotation(r, pool)?),
        b'[' => {
            let n = r.u2()?;
            let mut v = Vec::with_capacity(n as usize);
            for _ in 0..n {
                v.push(element_value(r, pool)?);
            }
            ElementValue::Array(v)
        }
        t => return Err(Error::BadAnnotationTag(t)),
    })
}

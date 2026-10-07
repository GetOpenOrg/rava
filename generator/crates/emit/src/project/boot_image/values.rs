//! 映像区：`__BootImage` 结构与 `BOOT_IMAGE` 常量初值。
//!
//! 引用的常量形态按槽的静态类型：`Object` → 句柄；类 wrapper → `__from_image(__Ref::image(..))`
//! （类 vtable 由 `X__inner` 经 supertrait 链实现）；同形数组 → `JArray::__image`。接口视图、类镜像、
//! 占位对象与形态不一致的引用不在常量中构造：槽取 null，由启动序列链接（[`Link`]）。

use std::fmt::Write as _;

use closure::image::{IBody, IVal};

use super::layout::{squash, Cell, SlotTy};
use super::Plan;
use crate::error::Result;

/// 启动序列的链接位置：对象字段（对象下标, 对象类, 存储字段 Rust 名）/ 数组元素
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum LinkLoc {
    Field(u32, String, String),
    Elem(u32, u32),
}

/// 启动序列写入的引用：位置 ← 映像对象
#[derive(Clone, Debug)]
pub(crate) struct Link {
    pub loc: LinkLoc,
    pub target: u32,
}

pub(crate) struct ImageText {
    pub text: String,
    pub links: Vec<Link>,
}

/// 映像对象的 `Object` 句柄（常量求值可用）
pub(crate) fn obj_ref(i: u32) -> String {
    format!("Object(__Obj::image(&BOOT_IMAGE.o{i}.value as &dyn ObjectVTable))")
}

/// 基本类型描述符字母的 Rust 类型
pub(crate) fn prim_rust(c: u8) -> &'static str {
    match c {
        b'Z' => "bool",
        b'B' => "i8",
        b'C' => "u16",
        b'S' => "i16",
        b'I' => "i32",
        b'J' => "i64",
        b'F' => "f32",
        _ => "f64",
    }
}

/// 基本元素的同宽位形（映像数组元素区）
fn bits_rust(c: u8) -> &'static str {
    match c {
        b'Z' | b'B' => "u8",
        b'C' | b'S' => "u16",
        b'I' | b'F' => "u32",
        _ => "u64",
    }
}

/// 槽值的位模式（整数符号扩展）
pub(crate) fn bits(v: IVal) -> u64 {
    match v {
        IVal::I(x) => x as i64 as u64,
        IVal::J(x) => x as u64,
        IVal::F(b) => u64::from(b),
        IVal::D(b) => b,
        IVal::N | IVal::R(_) | IVal::T(..) => 0,
    }
}

impl Plan<'_, '_> {
    /// 数组元素描述符的 Rust 类型（类型位置，完整路径）
    pub fn elem_rust(&self, desc: &str) -> String {
        let b = desc.as_bytes()[0];
        match b {
            b'[' => format!("JArray<{}>", self.elem_rust(&desc[1..])),
            b'L' => {
                let c = &desc[1..desc.len() - 1];
                if c != ty::consts::OBJECT && self.generated(c).is_some() {
                    self.full_ty(c)
                } else {
                    "Object".to_string()
                }
            }
            _ => prim_rust(b).to_string(),
        }
    }

    /// 数组元素描述符的短形态（与字段类型文本比对：类取定义名）
    fn elem_short(&self, desc: &str) -> String {
        let b = desc.as_bytes()[0];
        match b {
            b'[' => format!("JArray<{}>", self.elem_short(&desc[1..])),
            b'L' => {
                let c = &desc[1..desc.len() - 1];
                if c != ty::consts::OBJECT && self.generated(c).is_some() {
                    format!("{}{}", self.ctx.declared(c), self.targs(c))
                } else {
                    "Object".to_string()
                }
            }
            _ => prim_rust(b).to_string(),
        }
    }

    /// 数组元素的槽类型
    pub fn elem_slot(&self, elem: &str) -> SlotTy {
        match elem.as_bytes()[0] {
            b'[' => SlotTy::Array(squash(&self.elem_short(elem))),
            b'L' => {
                let c = &elem[1..elem.len() - 1];
                match self.generated(c) {
                    Some(ci) if c != ty::consts::OBJECT => {
                        if ci.is_interface() {
                            SlotTy::Iface(c.to_string())
                        } else {
                            SlotTy::Class(c.to_string())
                        }
                    }
                    _ => SlotTy::Object,
                }
            }
            _ => SlotTy::Other,
        }
    }

    /// 物化对象 `t` 作为 `st` 类型槽值的常量形态；常量不可表达 → None（启动链接）
    pub fn const_ref(&self, st: &SlotTy, t: u32) -> Option<String> {
        if !self.mat.contains(&t) {
            return None;
        }
        let ty = &self.obj(t).ty;
        match st {
            SlotTy::Object => Some(obj_ref(t)),
            SlotTy::Class(c) if !ty.starts_with('[') => {
                let p = self.path(c);
                Some(format!(
                    "{p}::__from_image(__Ref::image(__Handle::image(__Obj::image(&BOOT_IMAGE.o{t}.value as &dyn ObjectVTable)), \
                     &BOOT_IMAGE.o{t}.value as &dyn {p}__VTable))"
                ))
            }
            SlotTy::Array(view) if ty.starts_with('[') && squash(&self.elem_short(ty)) == *view => {
                Some(format!("JArray::__image(&BOOT_IMAGE.o{t}.value)"))
            }
            _ => None,
        }
    }

    /// 引用数组元素的类型化 null
    fn typed_null(&self, st: &SlotTy) -> String {
        match st {
            SlotTy::Class(c) => format!("{}::__from_image(__Ref::NULL)", self.expr_path(c)),
            SlotTy::Iface(c) => format!("{}::__IMAGE_NULL", self.expr_path(c)),
            SlotTy::Array(_) => "JArray::__IMAGE_NULL".to_string(),
            SlotTy::Object | SlotTy::Other => "Object::__NULL".to_string(),
        }
    }

    /// 映像中可直接引用的对象（链接与重放的目标；占位对象在其步骤之后另行回填）
    pub fn linkable(&self, t: u32) -> bool {
        self.mat.contains(&t) || (self.live.contains(&t) && self.obj(t).mirror.is_some())
    }
}

/// 构建映像区结构与初值
pub(crate) fn image_struct(p: &Plan<'_, '_>) -> Result<ImageText> {
    let mut decl = String::from("#[repr(C)]\npub struct __BootImage {\n");
    let mut init = String::from("pub static BOOT_IMAGE: __BootImage = __BootImage {\n");
    let mut links = Vec::new();
    let object_instance = {
        let o = p.path(ty::consts::OBJECT);
        format!("{}__ObjectInstance", o.strip_suffix(p.ctx.declared(ty::consts::OBJECT).as_str()).unwrap_or(&o))
    };
    for &i in &p.mat {
        let o = p.obj(i);
        let hash = o.hash.map_or("None".to_string(), |h| format!("Some({h})"));
        let (ty, val) = match &o.body {
            IBody::Arr(es) => array_value(p, i, &o.ty, es, &hash, &mut links),
            IBody::Inst(_) if o.ty == ty::consts::OBJECT => {
                (format!("__ImageObj<{object_instance}>"), format!("__ImageObj::new({hash}, {object_instance}::IMAGE)"))
            }
            IBody::Inst(_) => {
                let inner = p.inner(&o.ty);
                let body = inst_value(p, i, &mut links)?;
                (format!("__ImageObj<{inner}>"), format!("__ImageObj::new({hash}, {inner} {{{body}}})"))
            }
        };
        let _ = writeln!(decl, "    pub o{i}: {ty},");
        let _ = writeln!(init, "    o{i}: {val},");
    }
    decl.push_str("}\n\n// 映像对象只经原子单元 / 引用单元访问（与堆对象同一条件）\nunsafe impl Sync for __BootImage {}\n\n");
    init.push_str("};\n");
    Ok(ImageText { text: decl + &init, links })
}

/// 实例对象的 `X__inner { .. }` 字段表（布局全部字段显式给出）
fn inst_value(p: &Plan<'_, '_>, i: u32, links: &mut Vec<Link>) -> Result<String> {
    let ty = &p.obj(i).ty;
    let layout = &p.layouts[ty];
    let mut vals: Vec<Option<IVal>> = vec![None; layout.len()];
    for (decl, name, v) in p.fields(i) {
        let s = p.slot(i, decl, name)?;
        let k = layout.iter().position(|x| std::ptr::eq(x, s)).expect("布局槽");
        vals[k] = Some(*v);
    }
    let mut out = String::new();
    for (s, v) in layout.iter().zip(vals) {
        let v = v.unwrap_or(IVal::N);
        let text = match &s.cell {
            Cell::Prim(_) => match bits(v) {
                0 => "__PrimCell::zeroed()".to_string(),
                b => format!("__PrimCell::from_bits({b:#x})"),
            },
            Cell::Ref(st) => match v {
                IVal::R(t) => match p.const_ref(st, t) {
                    Some(r) => format!("__RefField::new(Some({r}))"),
                    None => {
                        if p.linkable(t) {
                            links.push(Link { loc: LinkLoc::Field(i, ty.clone(), s.rust.clone()), target: t });
                        }
                        "__RefField::new(None)".to_string()
                    }
                },
                _ => "__RefField::new(None)".to_string(),
            },
        };
        let _ = write!(out, " {}: {text},", s.rust);
    }
    Ok(out)
}

/// 数组对象：`__ImageArr<T, S>` 类型与初值
fn array_value(p: &Plan<'_, '_>, i: u32, desc: &str, es: &[IVal], hash: &str, links: &mut Vec<Link>) -> (String, String) {
    let elem = &desc[1..];
    let n = es.len();
    let t = p.elem_rust(elem);
    let c = elem.as_bytes()[0];
    if c != b'L' && c != b'[' {
        let w = bits_rust(c);
        let lit = if w == "u8" {
            let mut s = String::from("*b\"");
            for v in es {
                let b = bits(*v) as u8;
                if b.is_ascii_alphanumeric() || b == b' ' || (b.is_ascii_punctuation() && b != b'"' && b != b'\\') {
                    s.push(b as char);
                } else {
                    let _ = write!(s, "\\x{b:02x}");
                }
            }
            s.push('"');
            s
        } else {
            let mask = match w {
                "u16" => 0xffff,
                "u32" => 0xffff_ffff,
                _ => u64::MAX,
            };
            format!("[{}]", es.iter().map(|v| format!("{:#x}", bits(*v) & mask)).collect::<Vec<_>>().join(", "))
        };
        let ty = format!("__ImageArr<{t}, UnsafeCell<[{w}; {n}]>>");
        return (ty, format!("__ImageArr::new({hash}, {n}, true, UnsafeCell::new({lit}))"));
    }
    let st = p.elem_slot(elem);
    let mut items = Vec::with_capacity(n);
    for (j, v) in es.iter().enumerate() {
        let r = match v {
            IVal::R(target) => match p.const_ref(&st, *target) {
                Some(r) => r,
                None => {
                    if p.linkable(*target) {
                        links.push(Link { loc: LinkLoc::Elem(i, j as u32), target: *target });
                    }
                    p.typed_null(&st)
                }
            },
            _ => p.typed_null(&st),
        };
        items.push(format!("__RefField::new({r})"));
    }
    let ty = format!("__ImageArr<{t}, [__RefField<{t}>; {n}]>");
    (ty, format!("__ImageArr::new({hash}, {n}, false, [{}])", items.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_bits() {
        assert_eq!(bits(IVal::I(-1)), u64::MAX);
        assert_eq!(bits(IVal::F(0x3f80_0000)), 0x3f80_0000);
        assert_eq!(bits(IVal::N), 0);
        assert_eq!(obj_ref(3), "Object(__Obj::image(&BOOT_IMAGE.o3.value as &dyn ObjectVTable))");
    }
}

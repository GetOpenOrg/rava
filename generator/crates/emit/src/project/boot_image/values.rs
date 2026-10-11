//! 映像区：分段的 `__BootImage{k}` 结构与 `BOOT_IMAGE_{k}` 常量初值。
//!
//! 引用的常量形态按槽的静态类型：`Object` → 句柄；类 wrapper → `__from_image(__Handle::image(..))`
//! （类 vtable 由 `X__inner` 经 supertrait 链实现）；接口载体 → `__from_image(__IfaceRef::image(..))`
//! （对象的类有 `impl I for X` 块时取接口视图，否则与运行期 `__IfaceRef::new` 同为无视图）；同形数组 →
//! `JArray::__image`，形态不一致的数组 → 映像中的擦除协变视图（`JArray::__image_view`）。类镜像是映像对象。
//! 占位对象的槽取 null，由启动序列在其步骤处回填；槽类型无常量形态（[`SlotTy::Other`]）的引用是残差写入
//! （[`Residual`]），由启动序列写入。

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

/// 残差写入：槽类型无常量形态的映像内引用（位置 ← 映像对象），由启动序列写入
#[derive(Clone, Debug)]
pub(crate) struct Residual {
    pub loc: LinkLoc,
    pub target: u32,
}

pub(crate) struct ImageText {
    pub text: String,
    pub residual: Vec<Residual>,
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

    /// 物化对象 `t` 作为 `st` 类型槽值的常量形态；未物化（占位 / 不活）或槽类型无常量形态 → None
    pub fn const_ref(&self, st: &SlotTy, t: u32) -> Option<String> {
        if !self.mat.contains(&t) {
            return None;
        }
        let ty = &self.obj(t).ty;
        let arr = ty.starts_with('[');
        match st {
            SlotTy::Object => Some(self.obj_ref(t)),
            SlotTy::Class(c) if !arr => {
                let (p, o) = (self.path(c), self.img(t));
                Some(format!(
                    "{p}::__from_image(__Handle::image(__Obj::image(&{o}.value as &dyn ObjectVTable)))"
                ))
            }
            SlotTy::Iface(c) => {
                let p = self.path(c);
                let view = !arr && self.ems.get(ty).is_some_and(|em| em.iface_views.iter().any(|v| v == c));
                let r = if view {
                    format!("__IfaceRef::image({}, &{}.value as &dyn {p}__VTable)", self.obj_ref(t), self.img(t))
                } else {
                    format!("__IfaceRef::null({})", self.obj_ref(t))
                };
                Some(format!("{p}::__from_image({r})"))
            }
            SlotTy::Array(view) if arr => {
                if squash(&self.elem_short(ty)) == *view {
                    Some(format!("JArray::__image(&{}.value)", self.img(t)))
                } else {
                    self.views.borrow_mut().insert(t);
                    Some(format!("JArray::__image_view(&BOOT_VIEW_{t}.value)"))
                }
            }
            _ => None,
        }
    }

    /// 引用数组元素的类型化 null
    fn typed_null(&self, st: &SlotTy) -> String {
        match st {
            SlotTy::Class(c) => format!("{}::__from_image(__Handle::NULL)", self.expr_path(c)),
            SlotTy::Iface(c) => format!("{}::__IMAGE_NULL", self.expr_path(c)),
            SlotTy::Array(_) => "JArray::__IMAGE_NULL".to_string(),
            SlotTy::Object | SlotTy::Other => "Object::__NULL".to_string(),
        }
    }

}

/// 构建映像区各段的结构与初值
pub(crate) fn image_struct(p: &Plan<'_, '_>) -> Result<ImageText> {
    let mut decls = vec![String::new(); p.nseg];
    let mut inits = vec![String::new(); p.nseg];
    let mut residual = Vec::new();
    let object_instance = {
        let o = p.path(ty::consts::OBJECT);
        format!("{}__ObjectInstance", o.strip_suffix(p.ctx.declared(ty::consts::OBJECT).as_str()).unwrap_or(&o))
    };
    for &i in &p.mat {
        let o = p.obj(i);
        let hash = o.hash.map_or("None".to_string(), |h| format!("Some({h})"));
        let (ty, val) = match &o.body {
            IBody::Arr(es) => array_value(p, i, &o.ty, es, &hash, &mut residual),
            IBody::Inst(_) if o.ty == ty::consts::OBJECT => {
                (format!("__ImageObj<{object_instance}>"), format!("__ImageObj::new({hash}, {object_instance}::IMAGE)"))
            }
            IBody::Inst(_) => {
                let inner = p.inner(&o.ty);
                let body = inst_value(p, i, &mut residual)?;
                (format!("__ImageObj<{inner}>"), format!("__ImageObj::new({hash}, {inner} {{{body}}})"))
            }
        };
        let k = p.seg[&i];
        let _ = writeln!(decls[k], "    pub o{i}: {ty},");
        let _ = writeln!(inits[k], "    o{i}: {val},");
    }
    let mut text = String::new();
    for (k, (decl, init)) in decls.iter().zip(&inits).enumerate() {
        let _ = write!(
            text,
            "#[repr(C)]\npub struct __BootImage{k} {{\n{decl}}}\n\n\
             // 映像对象只经原子单元 / 引用单元访问（与堆对象同一条件）\n\
             unsafe impl Sync for __BootImage{k} {{}}\n\n\
             pub static BOOT_IMAGE_{k}: __BootImage{k} = __BootImage{k} {{\n{init}}};\n\n"
        );
    }
    Ok(ImageText { text, residual })
}

/// 擦除协变视图：每个被异形引用的映像数组一个（独立静态，初值只有源数组句柄与擦除存取函数）。
/// 须在全部常量引用生成之后调用
pub(crate) fn views_text(p: &Plan<'_, '_>) -> String {
    let mut text = String::new();
    for &t in p.views.borrow().iter() {
        let _ = writeln!(text, "pub static BOOT_VIEW_{t}: __ImageArr<Object, ()> = __ImageArr::view({});", p.obj_ref(t));
    }
    text
}

/// 实例对象的 `X__inner { .. }` 字段表（布局全部字段显式给出）
fn inst_value(p: &Plan<'_, '_>, i: u32, residual: &mut Vec<Residual>) -> Result<String> {
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
                        if p.mat.contains(&t) {
                            residual.push(Residual { loc: LinkLoc::Field(i, ty.clone(), s.rust.clone()), target: t });
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
fn array_value(p: &Plan<'_, '_>, i: u32, desc: &str, es: &[IVal], hash: &str, residual: &mut Vec<Residual>) -> (String, String) {
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
                    if p.mat.contains(target) {
                        residual.push(Residual { loc: LinkLoc::Elem(i, j as u32), target: *target });
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
    }
}

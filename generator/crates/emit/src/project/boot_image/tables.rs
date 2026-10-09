//! 映像表与静态字段存储（计划 2026-10-05-boot-image-evaluator §5.10 零拷贝）。
//!
//! - **静态字段存储**：带映像常量值的静态字段，其存储单元在此以常量初值定义（外部符号 [`static_symbol`]，
//!   声明层 `#[image_static]` 只声明同名外部静态）；每个字段一个静态，初值规模有界；
//! - **映像表** `IMAGE_TABLES`：类镜像（按键）、驻留串（按 UTF-16 内容）、构建期初始化类、VM 模块表初值。
//!   大表按块（[`CHUNK`]）各成一个静态，块间与块内整体有序，运行期分块二分查找。

use std::fmt::Write as _;

use closure::image::{IBody, IVal};

use super::layout::{static_cell, Cell};
use super::values::bits;
use super::{image_static_accessor, is_image_constant, static_symbol, Plan};
use crate::error::{EmitError, Result};

/// 每块表项数
const CHUNK: usize = 1024;

/// 残差静态写入：槽类型无常量形态的静态字段（类, 访问器名, 映像对象）
pub(crate) struct StaticResidual {
    pub cls: String,
    pub acc: String,
    pub target: u32,
}

pub(crate) struct TablesText {
    pub text: String,
    /// 静态存储保活块数（`IMAGE_STATICS_{k}`）
    pub keep: usize,
    pub statics: Vec<StaticResidual>,
}

impl Plan<'_, '_> {
    /// 映像对象的 `&dyn ObjectVTable`（常量求值可用）
    pub fn vt(&self, i: u32) -> String {
        format!("&{}.value as &dyn ObjectVTable", self.img(i))
    }

    /// 类所在模块的路径（类路径去掉定义名）
    fn module_of(&self, cls: &str) -> String {
        let p = self.path(cls);
        let d = self.ctx.declared(cls);
        p.strip_suffix(d.as_str()).unwrap_or(&p).to_string()
    }
}

/// 基本类型描述符字符 → 类镜像表键（Java 关键字，不与类名冲突）
fn primitive_key(c: &str) -> Option<&'static str> {
    Some(match c {
        "Z" => "boolean",
        "B" => "byte",
        "C" => "char",
        "S" => "short",
        "I" => "int",
        "J" => "long",
        "F" => "float",
        "D" => "double",
        "V" => "void",
        _ => return None,
    })
}

/// 映像字符串的内容（UTF-16）：存储布局中唯一的 `[B` 字段为内容数组、唯一的 `B` 字段为编码
/// （0 = Latin-1，否则 UTF-16 小端）。内容随宿主改写的串 / 结构不符 → None（不入驻留表，运行期首次驻留时登记）
fn string_units(p: &Plan<'_, '_>, s: u32) -> Option<Vec<u16>> {
    let ty = &p.obj(s).ty;
    let layout = p.layouts.get(ty)?;
    let one = |desc: &str| {
        let mut it = layout.iter().filter(|x| x.desc == desc);
        let a = it.next()?;
        it.next().is_none().then_some(a)
    };
    let (value, coder) = (one("[B")?, one("B")?);
    let get = |slot: &super::layout::Slot| p.fields(s).filter(|(d, n, _)| *d == slot.decl && *n == slot.java).last().map(|f| f.2);
    let IVal::R(a) = get(value)? else { return None };
    let arr = p.obj(a);
    if arr.host.is_some() || arr.deferred.is_some() {
        return None;
    }
    let IBody::Arr(es) = &arr.body else { return None };
    let bytes: Vec<u8> = es.iter().map(|v| bits(*v) as u8).collect();
    Some(match get(coder).map_or(0, bits) {
        0 => bytes.iter().map(|&b| u16::from(b)).collect(),
        _ => bytes.chunks(2).map(|c| u16::from(c[0]) | (u16::from(*c.get(1).unwrap_or(&0)) << 8)).collect(),
    })
}

/// 分块静态：`static {name}_{k}: [{ty}; n] = [..];`，返回各块引用（`&{name}_{k} as &[_]`）
fn chunks(out: &mut String, name: &str, ty: &str, items: &[String]) -> Vec<String> {
    let mut refs = Vec::new();
    for (k, c) in items.chunks(CHUNK).enumerate() {
        let _ = writeln!(out, "static {name}_{k}: [{ty}; {}] = [\n    {},\n];", c.len(), c.join(",\n    "));
        refs.push(format!("&{name}_{k} as &[_]"));
    }
    refs
}

/// 静态字段存储与映像表的文本
pub(crate) fn tables(p: &Plan<'_, '_>) -> Result<TablesText> {
    let d = p.d;
    let mut out = String::new();
    // ── 静态字段存储 ──
    let mut keep_items = Vec::new();
    let mut statics = Vec::new();
    for (n, (cls, name, v)) in d.statics.iter().enumerate() {
        if !is_image_constant(d, *v) || p.generated(cls).is_none() {
            continue;
        }
        let Some(acc) = image_static_accessor(p.ctx, cls, name) else { continue };
        let ci = p.generated(cls).expect("生成类");
        let f = ci.fields().iter().find(|f| f.is_static() && f.name == *name).expect("静态字段");
        let init = match (static_cell(p.ctx, ci, f), *v) {
            (Cell::Prim(_), v) => format!("__PrimCell::from_bits({:#x})", bits(v)),
            (Cell::Ref(st), IVal::R(t)) => match p.const_ref(&st, t) {
                Some(r) => format!("__RefField::new(Some({r}))"),
                None => {
                    if p.mat.contains(&t) {
                        statics.push(StaticResidual { cls: cls.clone(), acc: acc.clone(), target: t });
                    }
                    "__RefField::new(None)".to_string()
                }
            },
            (Cell::Ref(_), v) => return Err(EmitError::Input(format!("引导映像静态字段 {cls}.{name} 是引用槽但值为 {v:?}"))),
        };
        let ty = format!("{}__STATIC_TY_{}_{acc}", p.module_of(cls), p.ctx.declared(cls));
        let _ = writeln!(out, "#[unsafe(export_name = {:?})]\npub static __IS_{n}: {ty} = {init};", static_symbol(cls, name));
        keep_items.push(format!("&__IS_{n}"));
    }
    // 外部符号的定义须随启动序列一同链接（声明层经外部静态引用它们）：启动函数经保活块引用全部定义
    let keep = keep_items.chunks(CHUNK).len();
    for (k, c) in keep_items.chunks(CHUNK).enumerate() {
        let _ = writeln!(out, "pub static IMAGE_STATICS_{k}: [&(dyn ::std::marker::Sync); {}] = [{}];", c.len(), c.join(", "));
    }
    // ── 类镜像 ──
    let mut mirrors: Vec<(String, u32)> = Vec::new();
    for &i in &p.mat {
        if let Some(m) = &p.obj(i).mirror {
            mirrors.push((primitive_key(m).map_or_else(|| m.clone(), str::to_string), i));
        }
    }
    mirrors.sort();
    mirrors.dedup_by(|a, b| a.0 == b.0);
    let items: Vec<String> = mirrors.iter().map(|(k, i)| format!("({k:?}, {})", p.vt(*i))).collect();
    let mirror_refs = chunks(&mut out, "IMG_MIRRORS", "(&str, &dyn ObjectVTable)", &items);
    // ── 驻留串 ──
    let mut strings: Vec<(Vec<u16>, u32)> = d.strings.iter().filter(|s| p.mat.contains(s)).filter_map(|&s| Some((string_units(p, s)?, s))).collect();
    strings.sort();
    strings.dedup_by(|a, b| a.0 == b.0);
    let items: Vec<String> = strings.iter().map(|(_, s)| p.vt(*s)).collect();
    let string_refs = chunks(&mut out, "IMG_STRINGS", "&dyn ObjectVTable", &items);
    // ── 构建期初始化类 ──
    let mut bt: Vec<&str> = d.build_time.iter().filter(|c| p.generated(c).is_some()).map(String::as_str).collect();
    bt.sort_unstable();
    bt.dedup();
    let bt: Vec<String> = bt.iter().map(|c| format!("{c:?}")).collect();
    // ── VM 模块表 ──
    let mut modules = Vec::new();
    for m in &d.modules {
        if !p.mat.contains(&m.obj) {
            continue;
        }
        let loader = match m.loader {
            IVal::R(l) if p.mat.contains(&l) => format!("Some({})", p.vt(l)),
            IVal::R(l) => return Err(EmitError::Input(format!("引导映像模块 #{} 的定义加载器 #{l} 未物化", m.obj))),
            _ => "None".to_string(),
        };
        let pkgs: Vec<String> = m.packages.iter().map(|x| format!("{x:?}")).collect();
        modules.push(format!(
            "rt::ImageModule {{ module: {}, loader: {loader}, open: {}, location: {:?}, packages: &[{}] }}",
            p.vt(m.obj),
            m.open,
            m.location,
            pkgs.join(", ")
        ));
    }
    let _ = write!(
        out,
        "\n/// 映像表（启动时登记一次，运行期首次查询时直接查表）\npub static IMAGE_TABLES: rt::ImageTables = rt::ImageTables {{\n    \
         mirrors: &[{}],\n    strings: &[{}],\n    build_time: &[{}],\n    modules: &[\n        {}\n    ],\n}};\n\n",
        mirror_refs.join(", "),
        string_refs.join(", "),
        bt.join(", "),
        modules.join(",\n        ")
    );
    Ok(TablesText { text: out, keep, statics })
}


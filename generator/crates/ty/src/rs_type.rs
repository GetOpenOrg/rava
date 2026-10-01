//! 发射侧 Rust 类型节点（Python 侧的 Rust 类型串在这里是结构化值）。
//!
//! 类型身份用 binary name 承载；文本只在 [`RsType::render`] 出口经 [`ShortNames`]
//! 产生（显示 / 比对用）。

use crate::short_names::ShortNames;

/// JVM 基本类型在 Rust 侧的拼写
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Prim {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    Bool,
    U16,
}

impl Prim {
    /// JVM 描述符字符 → 基本类型（`V` 不在内，见 [`RsType::from_prim_desc`]）
    pub fn from_desc(c: u8) -> Option<Prim> {
        Some(match c {
            b'B' => Prim::I8,
            b'S' => Prim::I16,
            b'I' => Prim::I32,
            b'J' => Prim::I64,
            b'F' => Prim::F32,
            b'D' => Prim::F64,
            b'Z' => Prim::Bool,
            b'C' => Prim::U16,
            _ => return None,
        })
    }

    pub fn rust_name(self) -> &'static str {
        match self {
            Prim::I8 => "i8",
            Prim::I16 => "i16",
            Prim::I32 => "i32",
            Prim::I64 => "i64",
            Prim::F32 => "f32",
            Prim::F64 => "f64",
            Prim::Bool => "bool",
            Prim::U16 => "u16",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum RsType {
    Prim(Prim),
    /// `()`（void）
    Unit,
    /// 根类 / 擦除载体 `Object`
    Object,
    /// 注册表内的类（或接口载体）；按短名渲染，实参非空时带 `<..>`
    Class {
        binary: String,
        args: Vec<RsType>,
    },
    /// 签名解析直映射的锚点类：按简单名渲染、不经短名消歧（`_CLASSNAME_MAP` 路径）
    Bare {
        binary: String,
    },
    /// `JArray<elem>`
    Array(Box<RsType>),
    /// 作用域内类型形参
    Param(String),
}

impl RsType {
    /// 描述符基本类型字符（含 `V`）→ 类型
    pub fn from_prim_desc(c: u8) -> Option<RsType> {
        if c == b'V' {
            return Some(RsType::Unit);
        }
        Prim::from_desc(c).map(RsType::Prim)
    }

    pub fn class(binary: impl Into<String>, args: Vec<RsType>) -> RsType {
        RsType::Class {
            binary: binary.into(),
            args,
        }
    }

    pub fn array(elem: RsType) -> RsType {
        RsType::Array(Box::new(elem))
    }

    /// n 个 `Object`（擦除实参）
    pub fn objects(n: usize) -> Vec<RsType> {
        vec![RsType::Object; n]
    }

    /// 顶层类型实参（`split_rust_type_args` 的结构化等价物；数组的元素算一个实参）
    pub fn type_args(&self) -> &[RsType] {
        match self {
            RsType::Class { args, .. } => args,
            RsType::Array(e) => std::slice::from_ref(e),
            _ => &[],
        }
    }

    /// 头名（`^(\w+)` 口径）：`()` 无头名
    pub fn head_name(&self, names: &ShortNames) -> Option<String> {
        match self {
            RsType::Prim(p) => Some(p.rust_name().to_string()),
            RsType::Unit => None,
            RsType::Object => Some("Object".to_string()),
            RsType::Class { binary, .. } => Some(names.short(binary).into_owned()),
            RsType::Bare { binary } => Some(bare_name(binary)),
            RsType::Array(_) => Some("JArray".to_string()),
            RsType::Param(n) => Some(n.clone()),
        }
    }

    /// 类型形参代入（`substitute_type_params` 的结构化等价物：单遍，命中值不再代入）
    pub fn substitute(&self, mapping: &dyn Fn(&str) -> Option<RsType>) -> RsType {
        match self {
            RsType::Param(n) => mapping(n).unwrap_or_else(|| self.clone()),
            RsType::Class { binary, args } => RsType::Class {
                binary: binary.clone(),
                args: args.iter().map(|a| a.substitute(mapping)).collect(),
            },
            RsType::Array(e) => RsType::array(e.substitute(mapping)),
            _ => self.clone(),
        }
    }

    /// 引用的全部注册表类（binary，渲染序，含重复；`Bare` 锚点不经短名、不计入）
    pub fn collect_classes<'s>(&'s self, out: &mut Vec<&'s str>) {
        match self {
            RsType::Class { binary, args } => {
                out.push(binary);
                for a in args {
                    a.collect_classes(out);
                }
            }
            RsType::Array(e) => e.collect_classes(out),
            _ => {}
        }
    }

    /// 渲染为 Rust 类型文本
    pub fn render(&self, names: &ShortNames) -> String {
        let mut s = String::new();
        self.render_into(names, &mut s);
        s
    }

    fn render_into(&self, names: &ShortNames, out: &mut String) {
        match self {
            RsType::Prim(p) => out.push_str(p.rust_name()),
            RsType::Unit => out.push_str("()"),
            RsType::Object => out.push_str("Object"),
            RsType::Class { binary, args } => {
                out.push_str(&names.short(binary));
                render_args(args, names, out);
            }
            RsType::Bare { binary } => out.push_str(&bare_name(binary)),
            RsType::Array(e) => {
                out.push_str("JArray<");
                e.render_into(names, out);
                out.push('>');
            }
            RsType::Param(n) => out.push_str(n),
        }
    }
}

/// 实参表 `<A, B>`（空表不输出）
pub fn render_args(args: &[RsType], names: &ShortNames, out: &mut String) {
    if args.is_empty() {
        return;
    }
    out.push('<');
    for (i, a) in args.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        a.render_into(names, out);
    }
    out.push('>');
}

/// 渲染实参表为独立文本（`<A, B>` / 空串）
pub fn render_arg_list(args: &[RsType], names: &ShortNames) -> String {
    let mut s = String::new();
    render_args(args, names, &mut s);
    s
}

fn bare_name(binary: &str) -> String {
    binary
        .rsplit('/')
        .next()
        .unwrap_or(binary)
        .replace('$', "_")
}

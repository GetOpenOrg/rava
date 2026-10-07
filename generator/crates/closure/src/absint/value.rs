//! 抽象值：值来源（[`Src`]）、槽位值（[`V`]）及其按描述符构造的辅助函数。

use super::*;

/// 引用值来源
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Src {
    /// 形参序号（实例方法 0 = this）
    Param(u16),
    /// 产生该值的指令偏移（new / 调用返回 / 字段读 / 数组读 / indy / ldc 句柄等）
    Site(u32),
    /// 异常处理器入口（处理器偏移）
    Catch(u32),
    /// 字符串字面量（与其它值合流后；携带字面量序号，见 [`lit_id`]）
    Str(u32),
}

pub type Srcs = Rc<[Src]>;

pub(super) fn src1(s: Src) -> Srcs {
    Rc::from([s].as_slice())
}

fn src_union(a: &[Src], b: &[Src]) -> Srcs {
    let mut v: Vec<Src> = a.iter().chain(b.iter()).copied().collect();
    v.sort();
    v.dedup();
    Rc::from(v)
}

#[derive(Clone, Debug, PartialEq)]
pub enum V {
    Top,
    /// long / double 的第二槽
    Hi,
    Int(i32),
    /// int 族常量的小集合（≥ 2 个，见 [`ints`]）
    Ints(Rc<[i32]>),
    /// 奇偶已知的 int（true = 奇）：数组下标奇偶敏感（键值交错数组等）
    Par(bool),
    /// 值未知、但恒等于入口处 int 族形参 i 的值（只经复制保持；参与运算 / 合流即为 Top）：判定形参是否选择分支
    Arg(u16),
    Long(i64),
    Null,
    /// 引用：静态类型（binary name 或数组描述符）+ 是否确定非空 + 来源集合 + 对象身份标签（见 [`Obj`]）
    Ref { ty: Option<Rc<str>>, nonnull: bool, src: Srcs, obj: Option<Rc<Obj>> },
    /// 字符串常量 + 来源集合：本方法 ldc 字面量的来源为 `Src::Str`（本点字面量）；常量格给出的值落到本方法
    /// （形参 / 字段读 / 调用返回的常量）的来源为该读取点，即常量格推不出时同一值的来源；同值合流取来源并。
    /// 值用于折叠；来源决定它是否算本点字面量（[`V::site_lits`]）及合流后的来源——
    /// 常量格的中间态常量与其终态（Top，来源同上）给出同样的来源，引擎的点名与处理顺序无关。
    /// 常量格存储形态（[`V::stripped`]）来源为空
    Str(Rc<str>, Srcs),
    /// 类字面量（ldc class）：值是 Class 对象，携带所指类与 ldc 偏移（合流后以该偏移为来源，引擎在此处给出类镜像）
    Class(Rc<str>, u32),
    /// 符号字段偏移（long）：按名取得的实例字段偏移即字段身份（声明类上的字段键），只经复制 / 字段传递保持，
    /// 参与运算即为 Top；按偏移读写的手写调用点据此只触及该字段。不作为折叠常量导出
    Offset(Rc<MemberRef>),
}

pub const STRING: &str = "java/lang/String";
pub const CLASS: &str = "java/lang/Class";

impl V {
    /// 可空性：Some(true) 恒非 null（实例方法的 this、`new` 结果、catch 值、字面量）；Some(false) 恒 null
    pub(crate) fn nonnull(&self) -> Option<bool> {
        match self {
            V::Null => Some(false),
            V::Ref { nonnull: true, .. } | V::Str(..) | V::Class(..) => Some(true),
            _ => None,
        }
    }

    /// 引用值的静态类型
    pub fn static_type(&self) -> Option<&str> {
        match self {
            V::Ref { ty, .. } => ty.as_deref(),
            V::Str(..) => Some(STRING),
            V::Class(..) => Some(CLASS),
            _ => None,
        }
    }

    /// 值恰为入口处的引用形参 i（来源只有该形参，可空性未知；≥ 64 不计）：判定形参是否按可空性选择分支
    pub(crate) fn param_ref(&self) -> Option<u16> {
        match self {
            V::Ref { nonnull: false, src, .. } => match &src[..] {
                [Src::Param(i)] if *i < 64 => Some(*i),
                _ => None,
            },
            _ => None,
        }
    }

    pub(super) fn is_ref(&self) -> bool {
        matches!(self, V::Null | V::Ref { .. } | V::Str(..) | V::Class(..))
    }

    /// 引用值的来源集合（Null 无来源）
    pub fn srcs(&self) -> Srcs {
        match self {
            V::Ref { src, .. } => src.clone(),
            V::Str(_, src) => src.clone(),
            V::Class(_, off) => src1(Src::Site(*off)),
            _ => Rc::from([].as_slice()),
        }
    }

    /// 值可能是的字符串字面量：字面量本身，或合流引用来源里的各个字面量
    pub fn lits(&self) -> Vec<Rc<str>> {
        match self {
            V::Str(s, _) => vec![s.clone()],
            V::Ref { src, .. } => src
                .iter()
                .filter_map(|s| match *s {
                    Src::Str(id) => Some(lit_str(id)),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
    }

    /// 本方法的字面量（ldc，或合流来源里的字面量）：不含常量格给出的值（见 [`V::Str`]）
    pub fn site_lits(&self) -> Vec<Rc<str>> {
        match self {
            V::Str(..) if self.derived_str() => vec![],
            _ => self.lits(),
        }
    }

    /// 本方法的 ldc 字符串字面量（来源只有字面量本身）
    pub fn lit(s: impl Into<Rc<str>>) -> V {
        let s: Rc<str> = s.into();
        let id = lit_id(&s);
        V::Str(s, src1(Src::Str(id)))
    }

    /// 来源不全是本方法字面量的字符串常量（常量格给出的值，或与之同值合流）：按其来源处理，不算本点字面量
    pub fn derived_str(&self) -> bool {
        matches!(self, V::Str(_, src) if src.is_empty() || src.iter().any(|s| !matches!(s, Src::Str(_))))
    }

    /// 同 [`V::lits`]，取字面量序号（见 `lit.rs`）
    pub fn lit_ids(&self) -> Vec<u32> {
        match self {
            V::Str(s, _) => vec![lit_id(s)],
            V::Ref { src, .. } => src
                .iter()
                .filter_map(|s| match *s {
                    Src::Str(id) => Some(id),
                    _ => None,
                })
                .collect(),
            _ => vec![],
        }
    }

    /// int 值的奇偶（true = 奇）
    pub fn parity(&self) -> Option<bool> {
        match self {
            V::Int(x) => Some(x & 1 != 0),
            V::Ints(s) => s.iter().all(|x| x & 1 == s[0] & 1).then(|| s[0] & 1 != 0),
            V::Par(p) => Some(*p),
            _ => None,
        }
    }

    pub fn join(&self, o: &V) -> V {
        if self == o {
            return self.clone();
        }
        if let (V::Str(a, sa), V::Str(b, sb)) = (self, o) {
            if a == b {
                return V::Str(a.clone(), src_union(sa, sb));
            }
        }
        if self.is_ref() && o.is_ref() {
            let (a, b) = (self.static_type(), o.static_type());
            let ty = match (self, o) {
                (V::Null, _) => b,
                (_, V::Null) => a,
                _ if a == b => a,
                _ => None,
            };
            let nonnull = self.nonnull() == Some(true) && o.nonnull() == Some(true);
            return V::Ref { ty: ty.map(Rc::from), nonnull, src: src_union(&self.srcs(), &o.srcs()), obj: obj::join_obj(self, o) };
        }
        match (self.parity(), o.parity()) {
            (Some(a), Some(b)) if a == b => V::Par(a),
            _ => V::Top,
        }
    }
}

pub(super) fn ft_name(ft: &FieldType) -> Rc<str> {
    match ft {
        FieldType::Object(c) => Rc::from(c.as_str()),
        other => Rc::from(other.descriptor().as_str()),
    }
}

/// 描述符类型的抽象值（引用 → 带类型、来源为 s 的可空引用；基本类型 → Top）
pub(super) fn value_of(ft: &FieldType, s: Src) -> V {
    if ft.is_reference() {
        V::Ref { ty: Some(ft_name(ft)), nonnull: false, src: src1(s), obj: None }
    } else {
        match (ft, s) {
            (FieldType::Prim(b'B' | b'C' | b'I' | b'S' | b'Z'), Src::Param(i)) => V::Arg(i),
            _ => V::Top,
        }
    }
}

pub(super) fn site_ref(ty: &str, nonnull: bool, off: u32) -> V {
    V::Ref { ty: Some(Rc::from(ty)), nonnull, src: src1(Src::Site(off)), obj: None }
}

pub(super) fn push_typed(stack: &mut Vec<V>, ft: &FieldType, v: V) {
    stack.push(v);
    if ft.slots() == 2 {
        stack.push(V::Hi);
    }
}

/// 数组类型（描述符）的元素类型
pub fn component(arr: &str) -> Option<Rc<str>> {
    let e = arr.strip_prefix('[')?;
    if let Some(c) = e.strip_prefix('L').and_then(|c| c.strip_suffix(';')) {
        Some(Rc::from(c))
    } else if e.starts_with('[') {
        Some(Rc::from(e))
    } else {
        None
    }
}

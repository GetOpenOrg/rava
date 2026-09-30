//! 引用值的对象身份标签（[`V::Ref`] 的 `obj`）：字段值分析与 VM 对象识别。
//!
//! - `Uninit`：`new` 之后、构造器返回之前的对象；`invokespecial <init>` 返回时按构造器摘要换成 `Fields`
//! - `Fields`：构造完成的对象，携带 final 实例字段的常量（构造器以常量实参 / 常量写入给出）。
//!   final 实例字段只在本类构造器内写入（JVMS putfield 约束），反射 / Unsafe / 反序列化写入由字段
//!   开放判定（`field_open`）在读取时排除——于是「对象的 final 字段值」在它的整个生命期内不变，
//!   getfield 可按标签折叠，无论对象经局部变量、静态字段还是返回值传到读取点
//! - `SysProps`：清单声明的系统属性表对象（`[facts.system_properties]` 的持有锚点给出）
//! - `MaybeSysProps`：可能是系统属性表对象（属性表与其它对象合流）：不按属性表读取折叠，
//!   但它上面的改写 / 逃逸照样计入属性表的改写判定——属性表的别名不会因合流而从判定中消失
//!
//! 标签只随值传播：两个值合流时标签相同才保留（null 与对象合流保留对象标签，可空性另记）；
//! 属性表（或可能的属性表）与其它值合流得 `MaybeSysProps`。

use std::rc::Rc;

use classfile::MemberRef;

use super::{Src, V};

#[derive(Clone, Debug, PartialEq)]
pub enum Obj {
    Uninit,
    /// final 实例字段常量（声明类字段键 → 值，按键排序；未列出 = 未知）
    Fields(Vec<(MemberRef, V)>),
    SysProps,
    MaybeSysProps,
}

impl Obj {
    /// 构造完成的对象上 final 字段 f（声明类字段键）的常量
    /// 可能是系统属性表对象（改写 / 逃逸判定用）
    pub fn may_be_sysprops(&self) -> bool {
        matches!(self, Obj::SysProps | Obj::MaybeSysProps)
    }

    pub fn field(&self, f: &MemberRef) -> Option<&V> {
        match self {
            Obj::Fields(fs) => fs.iter().find(|(k, _)| k == f).map(|(_, v)| v),
            _ => None,
        }
    }
}

impl V {
    /// 值的对象标签（`Uninit` 不算身份）
    pub fn obj(&self) -> Option<&Rc<Obj>> {
        match self {
            V::Ref { obj: Some(o), .. } if **o != Obj::Uninit => Some(o),
            _ => None,
        }
    }

    /// 带对象标签的引用去掉来源（常量格存储形态：跨方法传递的只是类型 + 可空性 + 标签）
    pub fn stripped(&self) -> V {
        match self {
            V::Ref { ty, nonnull, obj, .. } => V::Ref { ty: ty.clone(), nonnull: *nonnull, src: Rc::from([].as_slice()), obj: obj.clone() },
            other => other.clone(),
        }
    }

    /// 常量格给出的带标签引用落到本方法：来源换成读取点（字段读 / 调用返回 / 形参）
    pub fn rebased(self, s: Src) -> V {
        match self {
            V::Ref { ty, nonnull, obj, .. } => V::Ref { ty, nonnull, src: Rc::from([s].as_slice()), obj },
            other => other,
        }
    }
}

/// 合流后的对象标签：相同标签保留；一侧为 null 时取另一侧；属性表与其它值合流为「可能是属性表」
pub(super) fn join_obj(a: &V, b: &V) -> Option<Rc<Obj>> {
    let tag = |v: &V| match v {
        V::Ref { obj, .. } => obj.clone(),
        _ => None,
    };
    match (a, b) {
        (V::Null, V::Ref { obj, .. }) | (V::Ref { obj, .. }, V::Null) => obj.clone(),
        (V::Ref { obj: Some(x), .. }, V::Ref { obj: Some(y), .. }) if x == y => Some(x.clone()),
        _ if [tag(a), tag(b)].iter().flatten().any(|o| o.may_be_sysprops()) => Some(Rc::new(Obj::MaybeSysProps)),
        _ => None,
    }
}

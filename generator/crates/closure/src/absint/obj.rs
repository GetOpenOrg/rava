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
//! - `Empty`：空的不可修改集合（清单 `[facts.empty_collections]` 的工厂结果）：按 JDK 规范不含元素、
//!   不可改写，其上的查询（`isEmpty` / `size` / `get` …）按清单给出的值折叠
//! - `Len(n)`：以常量长度 n 分配的数组（`newarray` / `anewarray` 的长度操作数为常量）。数组长度在其生命期内不变
//!   （JVMS §2.7，没有任何指令、反射或 Unsafe 操作能改变已分配数组的长度），`arraylength` 按标签折叠为 n，
//!   无论数组经局部变量、形参、字段还是返回值传到读取点（形参 / 字段 / 返回常量格按标签汇合，见 `engine/facts.rs`）
//! - `BootSingleton(m)`：清单 `[vm_state] boot_singletons` 的方法 m 在引导类镜像上的结果——运行时恒为同一个进程内
//!   对象（如引导类共用的模块单例），两个同标签的值引用相等（`if_acmp` 折叠为相等）
//!
//! - `MirrorSub(o)`：类镜像子类型判定（`K.class.isAssignableFrom(x)`，见 `narrow.rs`）成立一侧的 x：值本身与来源
//!   不变（按来源上溯的名字 / 类求值照旧），只有类型流改取本方法偏移 o（条件跳转）处的收窄节点——输入值集中所指类
//!   ⊂ K 的类镜像。偏移只在本方法内有意义：不算对象身份（[`V::obj`] 不给出），不进常量格、不跨方法传递
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
    Empty,
    /// 常量长度的数组（见模块文档）
    Len(i32),
    /// 引导类镜像上恒返回同一对象的方法（`类.方法:描述符`）的结果（见模块文档）
    BootSingleton(Rc<str>),
    /// 类镜像子类型判定成立一侧的收窄值（本方法内条件跳转偏移，见模块文档）
    MirrorSub(u32),
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
    /// 值的对象标签（`Uninit` 与类镜像收窄标记不算身份）
    pub fn obj(&self) -> Option<&Rc<Obj>> {
        match self {
            V::Ref { obj: Some(o), .. } if !matches!(**o, Obj::Uninit | Obj::MirrorSub(_)) => Some(o),
            _ => None,
        }
    }

    /// 类镜像子类型判定成立一侧的收窄值：其类型流取本方法该偏移处的收窄节点
    pub fn mirror_narrowed(&self) -> Option<u32> {
        match self {
            V::Ref { obj: Some(o), .. } => match **o {
                Obj::MirrorSub(at) => Some(at),
                _ => None,
            },
            _ => None,
        }
    }

    /// 带对象标签的引用 / 字符串常量去掉来源（常量格存储形态：跨方法传递的只是类型 + 可空性 + 标签）
    pub fn stripped(&self) -> V {
        match self {
            V::Ref { ty, nonnull, obj, .. } => V::Ref { ty: ty.clone(), nonnull: *nonnull, src: Rc::from([].as_slice()), obj: obj.clone() },
            V::Str(t, _) => V::Str(t.clone(), Rc::from([].as_slice())),
            other => other.clone(),
        }
    }

    /// 常量格给出的带标签引用 / 字符串常量落到本方法：来源换成读取点（字段读 / 调用返回 / 形参）
    pub fn rebased(self, s: Src) -> V {
        match self {
            V::Ref { ty, nonnull, obj, .. } => V::Ref { ty, nonnull, src: Rc::from([s].as_slice()), obj },
            V::Str(t, _) => V::Str(t, Rc::from([s].as_slice())),
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

//! 接口层次的重载命名（翻译对照 §13.4）。
//!
//! 接口方法的 Rust 名只由**声明接口**及其超接口（传递）决定：声明接口自有的方法名 n，
//! 若自有方法与超接口上未被自有方法覆盖的同名实例成员合计有两种以上参数段，则按描述符
//! 区分。名字不受子接口、实现类、调用点影响，故同一方法在声明、实现、调用处一致。
//!
//! 子接口视图（接口接收者经继承看到的超接口成员）沿用声明名；只有来自不同声明者、
//! 声明名相同而参数段不同、互不覆盖的成员（兄弟超接口各自声明的同名异参方法）在该视图里
//! 按描述符区分。
//!
//! 「覆盖」按 Java 语义判定：参数段相同，或代入接收者视角的类型实参后参数擦除相同
//! （`interface P extends Comparable<P> { int compareTo(P) }` 覆盖 `compareTo(T)`，不构成重载）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::Method;

use super::{param_section, ParamSets};
use crate::consts::OBJECT;
use crate::jvm_type::subtype::super_closure;
use crate::registry::ClassInfo;
use crate::rs_type::RsType;
use crate::type_map::mangle_name;
use crate::TyCtx;

/// 视图里的一条接口成员：声明接口、其形参在视图接收者视角的代入、方法
struct ViewMember<'r> {
    owner: &'r ClassInfo,
    mapping: &'r BTreeMap<String, RsType>,
    m: &'r Method,
}

/// 经继承对子接口可见的接口实例成员
fn inheritable(m: &Method) -> bool {
    !m.is_static() && !m.is_synthetic() && !m.is_private() && !m.name.starts_with('<')
}

/// 擦除：去类型实参，根类统一为 `Object`
fn erase(t: &RsType) -> RsType {
    match t {
        RsType::Class { binary, .. } | RsType::Bare { binary } if binary == OBJECT => RsType::Object,
        RsType::Class { binary, .. } | RsType::Bare { binary } => RsType::class(binary.clone(), Vec::new()),
        RsType::Array(e) => RsType::Array(Box::new(erase(e))),
        other => other.clone(),
    }
}

impl<'a> TyCtx<'a> {
    /// 接口 ci 的超接口（传递，不含自身）及各自形参在 ci 视角下的代入，近者在前
    fn superinterface_mappings(&self, ci: &ClassInfo) -> Vec<(&'a ClassInfo, BTreeMap<String, RsType>)> {
        self.implemented_interface_views(ci)
            .into_iter()
            .filter_map(|(bin, args)| {
                let sup = self.reg.get(&bin)?;
                let mapping = self.effective_class_type_params(sup).iter().cloned().zip(args).collect();
                Some((sup, mapping))
            })
            .collect()
    }

    /// 方法参数在视图里的擦除类型（声明者形参经 mapping 代入）；签名不可解析 → None
    fn view_param_erasures(&self, v: &ViewMember<'_>) -> Option<Vec<RsType>> {
        let tps = self.effective_class_type_params(v.owner);
        let sig = v.m.signature.as_deref().unwrap_or(&v.m.desc);
        let parsed = self.parse_method_param_types(sig, &tps, false)?;
        Some(parsed.params.iter().map(|t| erase(&t.substitute(&|n| v.mapping.get(n).cloned()))).collect())
    }

    /// 两条同名成员在视图里是否同一覆盖槽（参数段相同，或代入后参数擦除相同）
    fn same_override_slot(&self, a: &ViewMember<'_>, b: &ViewMember<'_>) -> bool {
        if a.m.name != b.m.name {
            return false;
        }
        if param_section(&a.m.desc) == param_section(&b.m.desc) {
            return true;
        }
        match (self.view_param_erasures(a), self.view_param_erasures(b)) {
            (Some(x), Some(y)) => x == y,
            _ => false,
        }
    }

    /// 接口 ci 中须按描述符区分的方法名：自有方法名 n 的自有参数段 ∪ 超接口（传递）上
    /// 未被 ci 自有实例方法覆盖的同名实例成员参数段，合计两种以上
    pub(super) fn interface_overloaded_names(&self, ci: &ClassInfo, own_all: &ParamSets) -> BTreeSet<String> {
        let mut result: BTreeSet<String> =
            own_all.iter().filter(|(_, ps)| ps.len() > 1).map(|(n, _)| n.clone()).collect();
        if self.reg.is_empty() || ci.interfaces().is_empty() {
            return result;
        }
        let identity = BTreeMap::new();
        let own: Vec<ViewMember<'_>> = ci
            .methods()
            .iter()
            .filter(|m| !m.is_static() && !m.is_synthetic() && !m.name.starts_with('<'))
            .map(|m| ViewMember { owner: ci, mapping: &identity, m })
            .collect();
        let supers = self.superinterface_mappings(ci);
        for (n, ps) in own_all.iter().filter(|(_, ps)| ps.len() == 1) {
            let mut sets = ps.clone();
            for (sup, mapping) in &supers {
                for sm in sup.methods().iter().filter(|m| m.name == *n && inheritable(m)) {
                    let s = ViewMember { owner: sup, mapping, m: sm };
                    if !own.iter().any(|o| self.same_override_slot(o, &s)) {
                        sets.insert(param_section(&sm.desc));
                    }
                }
            }
            if sets.len() > 1 {
                result.insert(n.clone());
            }
        }
        result
    }

    /// 方法在其声明接口 decl 上的 Rust 名（未转义）
    fn interface_declared_name(&self, decl: &ClassInfo, name: &str, desc: &str) -> String {
        if self.hierarchy_overloaded_names(decl).contains(name) {
            mangle_name(self.manifest, name, desc)
        } else {
            name.to_string()
        }
    }

    /// 接口接收者 recv 视角下、声明于超接口 decl 的成员 `name:desc` 的 Rust 名（未转义）。
    /// 通常即声明名；recv 经不同声明者继承到声明名相同、参数段不同、互不覆盖的同名成员时，
    /// 该视图里按描述符区分。定义侧（子接口继承成员声明）与调用侧同用本判定
    pub fn interface_view_member_name(&self, recv: &ClassInfo, decl: &ClassInfo, name: &str, desc: &str) -> String {
        let declared = self.interface_declared_name(decl, name, desc);
        if recv.name() == decl.name() || !recv.is_interface() || !decl.is_interface() {
            return declared;
        }
        let identity = BTreeMap::new();
        let supers = self.superinterface_mappings(recv);
        let mut members: Vec<ViewMember<'_>> = recv
            .methods()
            .iter()
            .filter(|m| m.name == name && !m.is_static() && !m.is_synthetic())
            .map(|m| ViewMember { owner: recv, mapping: &identity, m })
            .collect();
        for (sup, mapping) in &supers {
            members.extend(
                sup.methods()
                    .iter()
                    .filter(|m| m.name == name && inheritable(m))
                    .map(|m| ViewMember { owner: sup, mapping, m }),
            );
        }
        // 被更具体声明者（子接口或 recv 自身）同槽覆盖的成员在视图中不可见
        let hidden = |i: usize| {
            let e = &members[i];
            members.iter().enumerate().any(|(j, o)| {
                j != i
                    && o.owner.name() != e.owner.name()
                    && super_closure(o.owner.name(), self.reg).contains(e.owner.name())
                    && self.same_override_slot(o, e)
            })
        };
        let pdesc = param_section(desc);
        let Some(ti) = members
            .iter()
            .position(|e| e.owner.name() == decl.name() && param_section(&e.m.desc) == pdesc)
        else {
            return declared;
        };
        if hidden(ti) {
            return declared;
        }
        let collides = (0..members.len()).any(|i| {
            let e = &members[i];
            i != ti
                && e.owner.name() != decl.name()
                && param_section(&e.m.desc) != pdesc
                && !hidden(i)
                && self.interface_declared_name(e.owner, name, &e.m.desc) == declared
        });
        if collides {
            mangle_name(self.manifest, name, desc)
        } else {
            declared
        }
    }
}

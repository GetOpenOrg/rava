//! 子类型判定与祖先闭包（`is_subtype_of` / `_super_closure` / `_contained` /
//! `strict_erased_subtype`）。
//!
//! Python 的闭包短名回退 `_closure_hit`（注册表域外祖先按 Rust 短名比较）不移植：
//! binary name 即身份。可观察差异仅在「目标是短名占位 / 与域外祖先同短名」时出现。

use std::collections::{BTreeSet, VecDeque};
use std::rc::Rc;

use super::{JvmType, WildKind};
use crate::consts;
use crate::registry::Registry;

/// binary 的全部祖先 binary（含自身）：super_class 链 + interfaces 闭包（BFS）；
/// 未注册节点是叶子。按注册表实例缓存
pub fn super_closure(binary: &str, reg: &Registry) -> Rc<BTreeSet<String>> {
    if let Some(hit) = reg.caches.super_closure.borrow().get(binary) {
        return hit.clone();
    }
    let mut seen = BTreeSet::from([binary.to_string()]);
    let mut queue = VecDeque::from([binary.to_string()]);
    while let Some(cur) = queue.pop_front() {
        let Some(ci) = reg.get(&cur) else {
            continue;
        };
        let sup = ci.super_class();
        let edges = std::iter::once(sup)
            .filter(|s| !s.is_empty())
            .chain(ci.interfaces().iter().map(String::as_str));
        for nxt in edges {
            if seen.insert(nxt.to_string()) {
                queue.push_back(nxt.to_string());
            }
        }
    }
    let result = Rc::new(seen);
    reg.caches
        .super_closure
        .borrow_mut()
        .insert(binary.to_string(), result.clone());
    result
}

fn bound_or_object(b: &Option<Box<JvmType>>) -> JvmType {
    b.as_deref().cloned().unwrap_or_else(JvmType::object_type)
}

/// 泛型实参包含关系（JLS 4.5.1 最小实现）
fn contained(actual: &JvmType, formal: &JvmType, reg: &Registry) -> bool {
    match formal {
        JvmType::Wildcard {
            kind: WildKind::Unbounded,
            ..
        } => true,
        JvmType::Wildcard {
            kind: WildKind::Extends,
            bound,
        } => actual.is_subtype_of(&bound_or_object(bound), reg),
        JvmType::Wildcard {
            kind: WildKind::Super,
            bound,
        } => bound_or_object(bound).is_subtype_of(actual, reg),
        _ => actual == formal,
    }
}

impl JvmType {
    /// 子类型判定（自反 + 传递；规则见 Python 同名方法文档）
    pub fn is_subtype_of(&self, other: &JvmType, reg: &Registry) -> bool {
        if self == other {
            return true;
        }
        match self {
            JvmType::Null => {
                return matches!(
                    other,
                    JvmType::Class { .. }
                        | JvmType::Array(_)
                        | JvmType::TypeVar { .. }
                        | JvmType::Wildcard { .. }
                )
            }
            JvmType::TypeVar { bound, .. } => {
                return bound_or_object(bound).is_subtype_of(other, reg)
            }
            JvmType::Wildcard {
                kind: WildKind::Extends,
                bound,
            } => return bound_or_object(bound).is_subtype_of(other, reg),
            JvmType::Wildcard { .. } => return JvmType::object_type().is_subtype_of(other, reg),
            _ => {}
        }
        match other {
            JvmType::Wildcard {
                kind: WildKind::Unbounded,
                ..
            } => return true,
            JvmType::Wildcard {
                kind: WildKind::Extends,
                bound,
            } => return self.is_subtype_of(&bound_or_object(bound), reg),
            JvmType::Wildcard {
                kind: WildKind::Super,
                bound,
            } => return bound_or_object(bound).is_subtype_of(self, reg),
            JvmType::TypeVar { bound, .. } => {
                return self.is_subtype_of(&bound_or_object(bound), reg)
            }
            _ => {}
        }
        match (self, other) {
            (JvmType::Array(a), JvmType::Array(b)) => a.is_subtype_of(b, reg),
            (JvmType::Array(_), JvmType::Class { binary, .. }) => {
                consts::ARRAY_SUPERTYPES.contains(&binary.as_str())
            }
            (
                JvmType::Class {
                    binary: sb,
                    args: sa,
                    ..
                },
                JvmType::Class {
                    binary: ob,
                    args: oa,
                    ..
                },
            ) => {
                if !super_closure(sb, reg).contains(ob) {
                    return false;
                }
                if oa.is_empty() {
                    return true; // raw 目标：unchecked 接受
                }
                sa.len() == oa.len() && sa.iter().zip(oa).all(|(a, b)| contained(a, b, reg))
            }
            _ => false,
        }
    }
}

/// erasure 基名级的严格子类型（不自反；根类恒不作目标；actual 须在注册表内）
pub fn strict_erased_subtype(actual: &JvmType, expected: &JvmType, reg: &Registry) -> bool {
    let (a, e) = (actual.erasure(), expected.erasure());
    let (JvmType::Class { binary: ab, .. }, JvmType::Class { binary: eb, .. }) = (&a, &e) else {
        return false;
    };
    if ab == eb || eb == consts::OBJECT || !reg.contains(ab) {
        return false;
    }
    a.is_subtype_of(&e, reg)
}

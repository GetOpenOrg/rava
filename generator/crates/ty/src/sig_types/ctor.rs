//! 构造器与覆盖方法的签名类型（`constructor_sig_types` / `method_sig_types`）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::insn::Operand;
use classfile::{acc, Method};

use crate::class_params::outer_instance_class;
use crate::consts::ACC_MANDATED;
use crate::registry::{method_signature, ClassInfo};
use crate::rs_type::RsType;
use crate::type_map::parse_descriptor_params;
use crate::TyCtx;

/// 方法在 Rust 侧的泛型签名类型；`ret = None` 表示退回描述符擦除形态
/// （Python 的 `([], '')`）。构造器的 `ret` 恒为 `()`，`params` 为空表示退回描述符形态
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigTypes {
    pub params: Vec<RsType>,
    pub ret: Option<RsType>,
}

impl SigTypes {
    fn erased() -> SigTypes {
        SigTypes {
            params: Vec::new(),
            ret: None,
        }
    }
}

/// 匿名类：带 EnclosingMethod，且 InnerClasses 中自身条目无简单名
pub fn is_anonymous_class(ci: &ClassInfo) -> bool {
    if ci.enclosing_class().is_empty() {
        return false;
    }
    ci.inner_classes()
        .iter()
        .any(|ic| ic.inner == ci.name() && ic.simple_name.as_deref().unwrap_or("").is_empty())
}

/// xload / xload_n 读取的局部槽（Python `^[ailfd]load(?:_(\d))?$` 口径）
fn load_slot(opcode: u8, operand: &Operand) -> Option<i64> {
    match opcode {
        // iload / lload / fload / dload / aload（含 wide 形态）
        0x15..=0x19 => match operand {
            // Python `int(operand or -1)`：操作数串非空，槽 0 也按 0 取
            Operand::Local(n) => Some(i64::from(*n)),
            _ => Some(-1),
        },
        // iload_0 .. aload_3
        0x1a..=0x2d => Some(i64::from((opcode - 0x1a) % 4)),
        _ => None,
    }
}

/// 构造器体内 super(...) 调用的 (父类构造器描述符, {父类形参下标 → 本构造器形参下标})，
/// 只收录实参是本构造器形参直接转发（xload slot）的位置
fn forwarded_super_ctor_params(ci: &ClassInfo, m: &Method) -> (String, BTreeMap<usize, usize>) {
    let params = parse_descriptor_params(&m.desc);
    let mut slot_to_param = BTreeMap::new();
    let mut slot: i64 = 1;
    for (idx, p) in params.iter().enumerate() {
        slot_to_param.insert(slot, idx);
        slot += if p == "J" || p == "D" { 2 } else { 1 };
    }
    let Some(code) = &m.code else {
        return (String::new(), BTreeMap::new());
    };
    let insns = &code.insns;
    for (pos, ins) in insns.iter().enumerate() {
        let Operand::Method(r, _) = &ins.operand else {
            continue;
        };
        if ins.opcode != 0xb7 || r.owner != ci.super_class() || r.name != "<init>" {
            continue;
        }
        let n_super = parse_descriptor_params(&r.desc).len();
        let mut forwarded = BTreeMap::new();
        for k in 0..n_super {
            let Some(src_pos) = (pos + k).checked_sub(n_super) else {
                break;
            };
            let src = &insns[src_pos];
            if let Some(idx) =
                load_slot(src.opcode, &src.operand).and_then(|s| slot_to_param.get(&s))
            {
                forwarded.insert(k, *idx);
            }
        }
        return (r.desc.clone(), forwarded);
    }
    (String::new(), BTreeMap::new())
}

impl TyCtx<'_> {
    /// 构造器的形参类型表；空表示退回描述符擦除形态（隐式形参恢复规则见 Python 同名函数）
    pub fn constructor_sig_types(
        &self,
        ci: &ClassInfo,
        m: &Method,
        class_type_params: &[String],
    ) -> Vec<RsType> {
        let params = parse_descriptor_params(&m.desc);
        if params.is_empty() {
            return Vec::new();
        }
        let mut types: Vec<RsType> = params.iter().map(|p| self.jvm_to_rust(p)).collect();
        let mut recovered = false;
        let gsig = method_signature(m);
        if !gsig.is_empty() {
            let declared = self
                .parse_method_param_types(gsig, class_type_params, false)
                .map(|s| s.params)
                .unwrap_or_default();
            if declared.len() == params.len() {
                types = declared;
                recovered = true;
            } else if !declared.is_empty() && m.parameters.len() == params.len() {
                let explicit: Vec<usize> = m
                    .parameters
                    .iter()
                    .enumerate()
                    .filter(|(_, (_, flags))| flags & (acc::SYNTHETIC | ACC_MANDATED) == 0)
                    .map(|(i, _)| i)
                    .collect();
                if explicit.len() == declared.len() {
                    for (i, t) in explicit.into_iter().zip(declared) {
                        types[i] = t;
                    }
                    recovered = true;
                }
            }
        }
        let outer_bin = outer_instance_class(ci);
        if !outer_bin.is_empty() && params[0] == format!("L{outer_bin};") && !self.reg.is_empty() {
            if let Some(t) = self.outer_instance_rust_type(&outer_bin, class_type_params) {
                types[0] = t;
                recovered = true;
            }
        }
        if gsig.is_empty() && is_anonymous_class(ci) {
            if let Some(parent) = self.reg.get(ci.super_class()) {
                recovered |= self.recover_anonymous_forwarded(ci, m, parent, &mut types);
            }
        }
        if recovered {
            types
        } else {
            Vec::new()
        }
    }

    /// 匿名类构造器：转发给 super(...) 的形参取父类构造器的泛型形参类型
    /// （父类形参按本类 SuperclassSignature 实参替换）
    fn recover_anonymous_forwarded(
        &self,
        ci: &ClassInfo,
        m: &Method,
        parent: &ClassInfo,
        types: &mut [RsType],
    ) -> bool {
        let (super_desc, forwarded) = forwarded_super_ctor_params(ci, m);
        let Some(parent_m) = parent
            .methods()
            .iter()
            .find(|pm| pm.name == "<init>" && pm.desc == super_desc)
        else {
            return false;
        };
        if forwarded.is_empty() {
            return false;
        }
        let parent_params = self.effective_class_type_params(parent);
        let parent_types = self.constructor_sig_types(parent, parent_m, &parent_params);
        let mut parent_args = self.superclass_type_args(ci);
        if parent_args.len() != parent_params.len() {
            parent_args = RsType::objects(parent_params.len());
        }
        let mapping: BTreeMap<&str, &RsType> = parent_params
            .iter()
            .map(String::as_str)
            .zip(&parent_args)
            .collect();
        let mut recovered = false;
        for (super_idx, own_idx) in forwarded {
            if let Some(pt) = parent_types.get(super_idx) {
                types[own_idx] = pt.substitute(&|n| mapping.get(n).map(|t| (*t).clone()));
                recovered = true;
            }
        }
        recovered
    }

    /// 超类链祖先经接口 default 继承的同名同描述符槽位：phase2 把未被类方法覆盖的 default 注入
    /// 首个（最远）实现该接口的祖先类（`declared_by` = 接口）。返回（该祖先在 `chain` 中的深度,
    /// (声明接口, default 方法, 接口在本类视角下的实参)）；本类自己引入的接口不算（本类声明即槽位）
    fn ancestor_default_slot<'s>(
        &'s self,
        ci: &ClassInfo,
        m: &Method,
        chain: &[(String, Vec<RsType>)],
    ) -> Option<(usize, (&'s ClassInfo, &'s Method, Vec<RsType>))> {
        let is_default = |am: &Method| {
            am.name == m.name
                && am.desc == m.desc
                && !am.is_abstract()
                && !am.is_static()
                && !am.is_synthetic()
                && am.access & acc::PRIVATE == 0
        };
        let (depth, iface, dm) = chain.iter().enumerate().rev().find_map(|(depth, (anc_bin, _))| {
            let anc = self.reg.get(anc_bin)?;
            let mut queue: Vec<&str> = anc.interfaces().iter().map(String::as_str).collect();
            let mut seen = BTreeSet::new();
            while let Some(i) = queue.pop() {
                if !seen.insert(i) {
                    continue;
                }
                let Some(ici) = self.reg.get(i) else {
                    continue;
                };
                if let Some(dm) = ici.methods().iter().find(|am| is_default(am)) {
                    return Some((depth, ici, dm));
                }
                queue.extend(ici.interfaces().iter().map(String::as_str));
            }
            None
        })?;
        let args = self
            .implemented_interface_views(ci)
            .into_iter()
            .find(|(b, _)| b == iface.name())
            .map(|(_, a)| a)?;
        Some((depth, (iface, dm, args)))
    }

    /// 方法在 Rust 侧的泛型签名类型。覆盖方法（超类链上同名同描述符的非私有实例声明，
    /// 以及祖先类经接口 default 继承的槽位）取**最远祖先**的声明，按祖先形参 → 本类视角实参替换
    pub fn method_sig_types(
        &self,
        ci: &ClassInfo,
        m: &Method,
        class_type_params: &[String],
    ) -> SigTypes {
        if m.name == "<init>" {
            return SigTypes {
                params: self.constructor_sig_types(ci, m, class_type_params),
                ret: Some(RsType::Unit),
            };
        }
        let mut root: Option<(&ClassInfo, &Method, Vec<RsType>)> = None;
        if !self.reg.is_empty() && !m.is_static() && !ci.is_constructor(m) && !m.is_private() {
            let chain = self.ancestor_type_args(ci, None);
            let mut root_depth = None;
            for (depth, (anc_bin, args)) in chain.iter().enumerate() {
                let Some(anc) = self.reg.get(anc_bin) else {
                    break;
                };
                let hit = anc.methods().iter().find(|am| {
                    am.name == m.name && am.desc == m.desc && !am.is_static() && !am.is_private()
                });
                if let Some(am) = hit {
                    root = Some((anc, am, args.clone()));
                    root_depth = Some(depth);
                }
            }
            // 祖先类经接口 default 承载的槽位比所有类声明更远时，以该 default 为根
            if let Some((depth, slot)) = self.ancestor_default_slot(ci, m, &chain) {
                if root_depth.is_none_or(|d| depth > d) {
                    root = Some(slot);
                }
            }
        }
        let to_sig = |s: Option<crate::MethodSigTypes>| match s {
            Some(s) => SigTypes {
                params: s.params,
                ret: Some(s.ret),
            },
            None => SigTypes::erased(),
        };
        let Some((root_ci, root_m, root_args)) = root else {
            return to_sig(self.parse_method_param_types(
                method_signature(m),
                class_type_params,
                m.is_static(),
            ));
        };
        let root_tparams = self.effective_class_type_params(root_ci);
        let Some(parsed) =
            self.parse_method_param_types(method_signature(root_m), &root_tparams, false)
        else {
            return SigTypes::erased();
        };
        let root_args = if root_args.len() == root_tparams.len() {
            root_args
        } else {
            RsType::objects(root_tparams.len())
        };
        let mapping: BTreeMap<&str, &RsType> = root_tparams
            .iter()
            .map(String::as_str)
            .zip(&root_args)
            .collect();
        let sub = |t: &RsType| t.substitute(&|n| mapping.get(n).map(|t| (*t).clone()));
        SigTypes {
            params: parsed.params.iter().map(sub).collect(),
            ret: Some(sub(&parsed.ret)),
        }
    }
}

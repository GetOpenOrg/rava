//! 接口实现视图：`implemented_interface_views`（Rust 类型实参）与
//! `interface_signature_views`（JVM 签名文本）。

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::skip_type_params_prefix;
use crate::class_params::skip_field_type_sig;
use crate::consts;
use crate::registry::ClassInfo;
use crate::rs_type::RsType;
use crate::sig_parse::substitute_signature_type_vars;
use crate::TyCtx;

/// 接口形参 → 签名文本的代入表；None 表示祖先以原始类型出现（类型变量不可代入）
type SigMapping = Option<BTreeMap<String, String>>;

/// 类签名里的直接超类型（超类 + 超接口）→ (binary name, 各段实参签名，外层在前)。
/// 通配实参：`+X` 取上界 X，`-X` / `*` 取根类。
pub fn supertype_signature_args(ci: &ClassInfo) -> Vec<(String, Vec<String>)> {
    let sig = ci.generic_signature();
    let b = sig.as_bytes();
    let root = format!("L{};", consts::OBJECT);
    let mut i = skip_type_params_prefix(sig);
    let mut out = Vec::new();
    while b.get(i) == Some(&b'L') {
        let end = skip_field_type_sig(sig, i);
        let mut name = String::new();
        let mut args = Vec::new();
        let mut j = i + 1;
        let mut seg_start = j;
        while j + 1 < end {
            match b[j] {
                b'<' => {
                    name.push_str(&sig[seg_start..j]);
                    j += 1;
                    while j < end && b[j] != b'>' {
                        let k = skip_field_type_sig(sig, j);
                        let arg = &sig[j..k];
                        args.push(match b[j] {
                            b'+' => arg[1..].to_string(),
                            b'-' | b'*' => root.clone(),
                            _ => arg.to_string(),
                        });
                        j = k;
                    }
                    j += 1;
                    seg_start = j;
                }
                b'.' => {
                    name.push_str(&sig[seg_start..j]);
                    name.push('$');
                    j += 1;
                    seg_start = j;
                }
                _ => j += 1,
            }
        }
        if seg_start < j {
            name.push_str(&sig[seg_start..j]);
        }
        out.push((name, args));
        i = end;
    }
    out
}

impl TyCtx<'_> {
    /// 类实现的全部接口（自身 + 祖先类 + 超接口闭包）及其在 recv 视角下的类型实参，
    /// 广度优先、近者在前
    pub fn implemented_interface_views(&self, recv: &ClassInfo) -> Vec<(String, Vec<RsType>)> {
        let anc_args: BTreeMap<String, Vec<RsType>> =
            self.ancestor_type_args(recv, None).into_iter().collect();
        let mut queue: VecDeque<(&ClassInfo, BTreeMap<String, RsType>)> = VecDeque::new();
        queue.push_back((recv, BTreeMap::new()));
        let mut seen_cls = BTreeSet::from([recv.name().to_string()]);
        let mut cur = recv.super_class();
        while let Some(cur_ci) = self.reg.get(cur).filter(|_| !seen_cls.contains(cur)) {
            seen_cls.insert(cur.to_string());
            let params = self.effective_class_type_params(cur_ci);
            let args = anc_args.get(cur).map(Vec::as_slice).unwrap_or(&[]);
            let mapping = params
                .iter()
                .enumerate()
                .map(|(i, p)| (p.clone(), args.get(i).cloned().unwrap_or(RsType::Object)))
                .collect();
            queue.push_back((cur_ci, mapping));
            cur = cur_ci.super_class();
        }
        let mut out = Vec::new();
        let mut seen = BTreeSet::new();
        while let Some((cur_ci, mapping)) = queue.pop_front() {
            for (sup_bin, sup_args) in self.superinterface_type_args(cur_ci) {
                if !seen.insert(sup_bin.clone()) {
                    continue;
                }
                let args: Vec<RsType> = sup_args
                    .iter()
                    .map(|a| a.substitute(&|n| mapping.get(n).cloned()))
                    .collect();
                if let Some(sup_ci) = self.reg.get(&sup_bin) {
                    let m = self
                        .effective_class_type_params(sup_ci)
                        .iter()
                        .cloned()
                        .zip(args.iter().cloned())
                        .collect();
                    queue.push_back((sup_ci, m));
                }
                out.push((sup_bin, args));
            }
        }
        out
    }

    /// 类实现的全部接口 → 接口类型形参在 ci 视角下的类型签名（原始类型出现的接口不在结果里）；
    /// 按 BFS 发现序
    pub fn interface_signature_views(
        &self,
        ci: &ClassInfo,
    ) -> Vec<(String, BTreeMap<String, String>)> {
        let mut views = Vec::new();
        let mut queue: VecDeque<(&ClassInfo, SigMapping)> = VecDeque::new();
        queue.push_back((ci, Some(BTreeMap::new())));
        let mut seen = BTreeSet::from([ci.name().to_string()]);
        while let Some((cur_ci, mapping)) = queue.pop_front() {
            // Python dict(...)：同名重复时后者覆盖
            let declared: BTreeMap<String, Vec<String>> =
                supertype_signature_args(cur_ci).into_iter().collect();
            let supers = std::iter::once(cur_ci.super_class())
                .filter(|s| !s.is_empty())
                .chain(cur_ci.interfaces().iter().map(String::as_str));
            for sup_bin in supers {
                let Some(sup_ci) = self.reg.get(sup_bin) else {
                    continue;
                };
                if !seen.insert(sup_bin.to_string()) {
                    continue;
                }
                let params = self.effective_class_type_params(sup_ci);
                let args = declared.get(sup_bin).map(Vec::as_slice).unwrap_or(&[]);
                let mut sup_map = BTreeMap::new();
                if let Some(m) = mapping
                    .as_ref()
                    .filter(|_| !params.is_empty() && args.len() == params.len())
                {
                    for (p, a) in params.iter().zip(args) {
                        // 实参签名不以 `<` 开头，代入不会走形参段解析，不会失败
                        let v = substitute_signature_type_vars(a, m).unwrap_or_else(|| a.clone());
                        sup_map.insert(p.clone(), v);
                    }
                }
                if sup_ci.is_interface() && !sup_map.is_empty() {
                    views.push((sup_bin.to_string(), sup_map.clone()));
                }
                let next = if !sup_map.is_empty() || params.is_empty() {
                    Some(sup_map)
                } else {
                    None
                };
                queue.push_back((sup_ci, next));
            }
        }
        views
    }
}

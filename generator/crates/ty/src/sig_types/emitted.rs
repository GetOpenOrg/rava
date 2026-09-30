//! 发射签名单一来源：方法最终写进 Rust impl 块的参数 / 返回类型
//! （`emitted_method_sig_types` 及其有效性判定），与菱形推断 `infer_type_args_from_declared`。

use std::collections::BTreeMap;

use classfile::Method;

use crate::registry::ClassInfo;
use crate::rs_type::RsType;
use crate::type_args::collect_idents;
use crate::type_map::{parse_descriptor_params, parse_descriptor_return};
use crate::TyCtx;

/// 发射签名里恒有效的名字（`_SIG_TYPE_BUILTIN`）
const SIG_TYPE_BUILTIN: [&str; 20] = [
    "Object", "String", "i32", "i64", "f32", "f64", "bool", "u16", "i8", "i16", "u32", "u64", "()",
    "Rc", "__Shared", "Vec", "RefCell", "usize", "u8", "JArray",
];

/// 发射签名：参数类型表 + 返回类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmittedSig {
    pub params: Vec<RsType>,
    pub ret: RsType,
}

impl TyCtx<'_> {
    /// 签名派生类型是否可直接用于发射签名：整体是类级形参，或所有类型名均为
    /// 内建 / 类级形参 / 注册表短名
    pub fn sig_type_valid(&self, t: &RsType, class_tparams: &[String]) -> bool {
        if class_tparams.contains(&t.render(self.names)) {
            return true;
        }
        let mut idents = Vec::new();
        collect_idents(t, self, &mut idents);
        idents.iter().all(|(name, _)| {
            SIG_TYPE_BUILTIN.contains(&name.as_str())
                || class_tparams.contains(name)
                || self.names.is_registry_short(name)
        })
    }

    /// 类型头名是否为注册表内接口（方法签名中接口类型擦除为描述符形态）
    pub fn sig_type_is_iface(&self, t: &RsType) -> bool {
        t.head_name(self.names)
            .is_some_and(|h| self.names.is_iface_short(&h))
    }

    fn sig_type_usable(&self, t: &RsType, class_tparams: &[String]) -> bool {
        self.sig_type_valid(t, class_tparams) && !self.sig_type_is_iface(t)
    }

    /// 方法发射到 Rust impl 块的最终参数 / 返回类型：签名类型逐位过滤（无效或接口 →
    /// 该位描述符形态）；签名缺失 / 位数不符 → 整体描述符形态
    pub fn emitted_method_sig_types(
        &self,
        ci: &ClassInfo,
        m: &Method,
        class_tparams: &[String],
    ) -> EmittedSig {
        let sig = self.method_sig_types(ci, m, class_tparams);
        let desc_params = parse_descriptor_params(&m.desc);
        let jps: Vec<RsType> = desc_params.iter().map(|t| self.jvm_to_rust(t)).collect();
        let params = if !sig.params.is_empty() && sig.params.len() == desc_params.len() {
            sig.params
                .into_iter()
                .zip(jps)
                .map(|(sp, jp)| {
                    if self.sig_type_usable(&sp, class_tparams) {
                        sp
                    } else {
                        jp
                    }
                })
                .collect()
        } else {
            jps
        };
        let ret = match sig.ret {
            Some(r) if self.sig_type_usable(&r, class_tparams) => r,
            _ => self.jvm_to_rust(parse_descriptor_return(&m.desc)),
        };
        EmittedSig { params, ret }
    }

    /// 描述符或（非空时优先）字段级泛型签名 → 类型
    pub fn jvm_to_rs_type(
        &self,
        desc: &str,
        generic_sig: &str,
        class_tparams: &[String],
    ) -> RsType {
        if generic_sig.is_empty() {
            self.jvm_to_rust(desc)
        } else {
            self.parse_one_type(generic_sig, 0, class_tparams, None).0
        }
    }

    /// 菱形构造：由声明类型的泛型签名反推被构造类的类型实参；任一形参无解 → None
    pub fn infer_type_args_from_declared(
        &self,
        actual_bin: &str,
        declared_sig: &str,
        class_tparams: &[String],
    ) -> Option<Vec<RsType>> {
        if self.reg.is_empty() || !declared_sig.starts_with('L') {
            return None;
        }
        let lt = declared_sig.find('<')?;
        let declared_bin = &declared_sig[1..lt];
        let ci = self.reg.get(actual_bin)?;
        if !self.reg.contains(declared_bin) {
            return None;
        }
        let declared_args = self
            .parse_type_args(declared_sig, lt, class_tparams, None)
            .0;
        let params = self.effective_class_type_params(ci);
        if params.is_empty() {
            return None;
        }
        let mut views: BTreeMap<String, Vec<RsType>> = BTreeMap::new();
        views.insert(
            actual_bin.to_string(),
            params.iter().map(|p| RsType::Param(p.clone())).collect(),
        );
        for (b, args) in self
            .ancestor_type_args(ci, None)
            .into_iter()
            .chain(self.implemented_interface_views(ci))
        {
            views.entry(b).or_insert(args);
        }
        let view = views
            .get(declared_bin)
            .filter(|v| v.len() == declared_args.len())?;
        let mut solved: BTreeMap<String, RsType> = BTreeMap::new();
        for (view_arg, declared_arg) in view.iter().zip(declared_args) {
            // 以渲染文本判定「实参就是形参名」（与 Python 串比较同口径）
            let text = view_arg.render(self.names);
            if params.contains(&text) {
                solved.entry(text).or_insert(declared_arg);
            }
        }
        params.iter().map(|p| solved.get(p).cloned()).collect()
    }
}

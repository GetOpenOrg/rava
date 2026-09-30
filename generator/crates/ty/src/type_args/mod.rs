//! 层次类型实参：父类 / 超接口 / 祖先链的类型实参、外围作用域实参
//! （`codegen/type_args.py` 的移植；接口视图见 [`views`]）。

mod views;

use std::collections::{BTreeMap, BTreeSet};

pub use views::supertype_signature_args;

use crate::class_params::{is_outer_this_field, outer_instance_class, parse_class_type_params, skip_field_type_sig};
use crate::registry::{method_signature, ClassInfo};
use crate::rs_type::RsType;
use crate::TyCtx;

/// `_CLASSNAME_MAP` 值 ∪ 基本类型 Rust 名 ∪ `JArray`：实参里恒可解析的名字
const ALWAYS_RESOLVABLE: [&str; 13] =
    ["String", "Object", "Class", "()", "i32", "i64", "f32", "f64", "bool", "i8", "i16", "u16", "JArray"];

/// 跳过类签名开头的类型形参段 `<..>`，返回其后位置（无形参段 → 0）
pub(crate) fn skip_type_params_prefix(sig: &str) -> usize {
    let b = sig.as_bytes();
    if b.first() != Some(&b'<') {
        return 0;
    }
    let mut depth = 0;
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'<' => depth += 1,
            b'>' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
        i += 1;
    }
    i
}

/// 类型中出现的每个类型名及其是否带实参（Python 对渲染文本
/// `([A-Za-z_]\w*)\s*(<)?` 扫描的结构化等价物）
fn collect_idents(t: &RsType, ctx: &TyCtx<'_>, out: &mut Vec<(String, bool)>) {
    match t {
        RsType::Prim(p) => out.push((p.rust_name().to_string(), false)),
        RsType::Unit => {}
        RsType::Object | RsType::Bare { .. } | RsType::Param(_) => {
            if let Some(h) = t.head_name(ctx.names) {
                out.push((h, false));
            }
        }
        RsType::Class { binary, args } => {
            out.push((ctx.names.short(binary).into_owned(), !args.is_empty()));
            for a in args {
                collect_idents(a, ctx, out);
            }
        }
        RsType::Array(e) => {
            out.push(("JArray".to_string(), true));
            collect_idents(e, ctx, out);
        }
    }
}

impl TyCtx<'_> {
    /// 类型实参中出现的每个类型名都可解析（类型形参 / 内建 / 注册表内的类且元数匹配）
    pub fn type_arg_is_resolvable(&self, t: &RsType, class_type_params: &[String]) -> bool {
        let mut idents = Vec::new();
        collect_idents(t, self, &mut idents);
        idents.iter().all(|(name, has_args)| {
            if class_type_params.contains(name) || ALWAYS_RESOLVABLE.contains(&name.as_str()) {
                return true;
            }
            match self.names.binary_of(name).and_then(|b| self.reg.get(b)) {
                None => false,
                // 裸用泛型类（raw type）在 Rust 中缺实参 → 不可用
                Some(ref_ci) => self.effective_class_type_params(ref_ci).is_empty() != *has_args,
            }
        })
    }

    /// 在构造点实例化 ci 的有效类型形参：作用域内同名变量原样传递，
    /// 其余（外围方法的方法级变量）取上界 / `Object`
    pub fn enclosing_scope_type_args(&self, ci: &ClassInfo, scope_type_params: &[String]) -> Vec<RsType> {
        let params = self.effective_class_type_params(ci);
        let bounds = match self.enclosing_method_info(ci) {
            Some(em) => self.extract_method_tparam_bounds(method_signature(em), &[], None),
            None => BTreeMap::new(),
        };
        params
            .iter()
            .map(|p| {
                if scope_type_params.contains(p) {
                    RsType::Param(p.clone())
                } else {
                    bounds.get(p).cloned().unwrap_or(RsType::Object)
                }
            })
            .collect()
    }

    /// 类级类型变量 → (上界 Rust 类型, 上界类 binary)；只保留注册表内具体类上界，
    /// 内部类继承外部类形参的上界（被自身同名形参遮蔽者除外）
    pub fn class_type_param_bounds(&self, ci: &ClassInfo) -> BTreeMap<String, (RsType, String)> {
        let mut visiting = BTreeSet::new();
        self.class_type_param_bounds_rec(ci, &mut visiting)
    }

    fn class_type_param_bounds_rec(
        &self,
        ci: &ClassInfo,
        visiting: &mut BTreeSet<String>,
    ) -> BTreeMap<String, (RsType, String)> {
        let mut result = BTreeMap::new();
        if self.reg.is_empty() {
            return result;
        }
        visiting.insert(ci.name().to_string());
        let own = parse_class_type_params(ci.generic_signature());
        if let Some(outer) = self.reg.get(&outer_instance_class(ci)) {
            // 外部类链成环时截断（Python 侧无此防护，合法字节码不成环）
            if !std::ptr::eq(outer, ci) && !visiting.contains(outer.name()) {
                result = self.class_type_param_bounds_rec(outer, visiting);
                result.retain(|name, _| !own.contains(name));
            }
        }
        if !own.is_empty() {
            let mut binaries = BTreeMap::new();
            let bounds = self.extract_method_tparam_bounds(ci.generic_signature(), &own, Some(&mut binaries));
            for (name, t) in bounds {
                let b_ci = binaries.get(&name).and_then(|b| self.reg.get(b));
                if let Some(b_ci) = b_ci.filter(|c| own.contains(&name) && !c.is_interface()) {
                    result.insert(name, (t, b_ci.name().to_string()));
                }
            }
        }
        visiting.remove(ci.name());
        result
    }

    /// 直接父类的类型实参（以 ci 的有效形参表达；取自 SuperclassSignature，
    /// 元数不符 / 不可解析 → `Object`）
    pub fn superclass_type_args(&self, ci: &ClassInfo) -> Vec<RsType> {
        let Some(parent) = self.reg.get(ci.super_class()) else {
            return Vec::new();
        };
        let parent_params = self.effective_class_type_params(parent);
        if parent_params.is_empty() {
            return Vec::new();
        }
        let own = self.effective_class_type_params(ci);
        let sig = ci.generic_signature();
        let mut args = Vec::new();
        if !sig.is_empty() {
            let i = skip_type_params_prefix(sig);
            if sig.as_bytes().get(i) == Some(&b'L') {
                args = self.parse_one_type(sig, i, &own, None).0.type_args().to_vec();
            }
        }
        if args.len() != parent_params.len() {
            args = RsType::objects(parent_params.len());
        }
        args.into_iter().map(|a| if self.type_arg_is_resolvable(&a, &own) { a } else { RsType::Object }).collect()
    }

    /// 直接超接口（注册表内）的类型实参，按 `ci.interfaces` 声明序
    pub fn superinterface_type_args(&self, ci: &ClassInfo) -> Vec<(String, Vec<RsType>)> {
        let mut out = Vec::new();
        if self.reg.is_empty() {
            return out;
        }
        let own = self.effective_class_type_params(ci);
        let mut parsed: BTreeMap<&str, Vec<RsType>> = BTreeMap::new();
        let sig = ci.generic_signature();
        if !sig.is_empty() {
            let b = sig.as_bytes();
            let mut i = skip_field_type_sig(sig, skip_type_params_prefix(sig));
            while b.get(i) == Some(&b'L') {
                let end = skip_field_type_sig(sig, i);
                let mut j = i + 1;
                while j < end && !matches!(b[j], b'<' | b';' | b'.') {
                    j += 1;
                }
                let mut args = Vec::new();
                if j < end && b[j] == b'<' {
                    args = self.parse_type_args(sig, j, &own, None).0;
                }
                // Python dict 赋值：同名重复时后者覆盖
                parsed.insert(&sig[i + 1..j], args);
                i = end;
            }
        }
        for iface in ci.interfaces() {
            let Some(iface_ci) = self.reg.get(iface) else {
                continue;
            };
            let params = self.effective_class_type_params(iface_ci);
            let mut args = parsed.get(iface.as_str()).cloned().unwrap_or_default();
            if args.len() != params.len() {
                args = RsType::objects(params.len());
            }
            let args =
                args.into_iter().map(|a| if self.type_arg_is_resolvable(&a, &own) { a } else { RsType::Object }).collect();
            out.push((iface.clone(), args));
        }
        out
    }

    /// 外部实例在内部类视角下的类型：外部类 + 被内部类继承的同名类型变量（其余 `Object`）；
    /// 外部类非泛型 / 不在注册表 → None
    pub fn outer_instance_rust_type(&self, outer_bin: &str, decl_params: &[String]) -> Option<RsType> {
        let outer = self.reg.get(outer_bin)?;
        let tp = self.effective_class_type_params(outer);
        if tp.is_empty() {
            return None;
        }
        let args =
            tp.iter().map(|p| if decl_params.contains(p) { RsType::Param(p.clone()) } else { RsType::Object }).collect();
        Some(RsType::class(outer_bin, args))
    }

    /// 内部类外部引用字段（`this$N`）的类型；非 `this$N` / 外部类非泛型 → None
    pub fn outer_ref_field_type(&self, field_name: &str, field_desc: &str, decl_params: &[String]) -> Option<RsType> {
        if !is_outer_this_field(field_name) || decl_params.is_empty() || self.reg.is_empty() {
            return None;
        }
        let rest = field_desc.strip_prefix('L')?;
        let end = rest.find(';').filter(|&e| e > 0)?;
        self.outer_instance_rust_type(&rest[..end], decl_params)
    }

    /// 沿超类链每个祖先的类型实参（直接父类在前；不含根类，链在注册表外截断）。
    ///
    /// `self_args`：ci 自身形参的实参（调用点静态类型）；None → 以形参自身表达。
    pub fn ancestor_type_args(&self, ci: &ClassInfo, self_args: Option<&[RsType]>) -> Vec<(String, Vec<RsType>)> {
        let mut chain = Vec::new();
        if self.reg.is_empty() {
            return chain;
        }
        let own = self.effective_class_type_params(ci);
        let mut mapping: BTreeMap<String, RsType> = match self_args {
            Some(a) if a.len() == own.len() => own.iter().cloned().zip(a.iter().cloned()).collect(),
            Some(_) => own.iter().map(|p| (p.clone(), RsType::Object)).collect(),
            None => BTreeMap::new(),
        };
        let mut cur = ci;
        let mut seen = BTreeSet::new();
        loop {
            let sup = cur.super_class();
            if sup.is_empty() || sup == crate::consts::OBJECT || seen.contains(sup) {
                break;
            }
            let Some(parent) = self.reg.get(sup) else {
                break;
            };
            seen.insert(sup.to_string());
            let args: Vec<RsType> =
                self.superclass_type_args(cur).iter().map(|a| a.substitute(&|n| mapping.get(n).cloned())).collect();
            mapping = self.effective_class_type_params(parent).iter().cloned().zip(args.iter().cloned()).collect();
            chain.push((sup.to_string(), args));
            cur = parent;
        }
        chain
    }

    /// 静态类型为 `recv_ty`（ci 的实例化）的接收者，各祖先 VTable 的类型实参：
    /// {祖先短名 → 实参表}（渲染为 `<A, B>` / 空串见 [`crate::rs_type::render_arg_list`]）
    pub fn ancestor_vtable_args_by_short(&self, ci: &ClassInfo, recv_ty: &RsType) -> BTreeMap<String, Vec<RsType>> {
        self.ancestor_type_args(ci, Some(recv_ty.type_args()))
            .into_iter()
            .map(|(b, args)| (self.names.short(&b).into_owned(), args))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::consts;
    use crate::testutil::{class, Fixture};

    const OBJ_TP: &str = "Ljava/lang/Object;";

    fn fixture() -> Fixture {
        Fixture::new(vec![
            class(consts::OBJECT).sup(""),
            class(consts::STRING),
            class("p/Coll").iface().sig(&format!("<E:{OBJ_TP}>{OBJ_TP}")),
            class("p/List").iface().ifaces(&["p/Coll"]).sig(&format!("<E:{OBJ_TP}>{OBJ_TP}Lp/Coll<TE;>;")),
            class("p/Box").sig(&format!("<T:{OBJ_TP}>{OBJ_TP}")),
            class("p/Base").sig(&format!("<T:{OBJ_TP}>{OBJ_TP}")),
            class("p/Mid").sup("p/Base").sig(&format!("<U:{OBJ_TP}>Lp/Base<Lp/Box<TU;>;>;")),
            class("p/Leaf")
                .sup("p/Mid")
                .ifaces(&["p/List"])
                .sig("Lp/Mid<Ljava/lang/String;>;Lp/List<Ljava/lang/String;>;"),
            class("p/Raw").sup("p/Base"),
        ])
    }

    #[test]
    fn ancestor_chain() {
        let f = fixture();
        let c = f.ctx();
        let leaf = f.reg.get("p/Leaf").unwrap();
        let chain: Vec<_> = c
            .ancestor_type_args(leaf, None)
            .into_iter()
            .map(|(b, a)| (b, crate::rs_type::render_arg_list(&a, &f.names)))
            .collect();
        assert_eq!(chain, [("p/Mid".to_string(), "<String>".to_string()), ("p/Base".into(), "<Box<String>>".into())]);
        let raw = f.reg.get("p/Raw").unwrap();
        assert_eq!(c.superclass_type_args(raw).len(), 1);
        assert_eq!(c.superclass_type_args(raw)[0], crate::RsType::Object);
    }

    #[test]
    fn interface_views() {
        let f = fixture();
        let c = f.ctx();
        let leaf = f.reg.get("p/Leaf").unwrap();
        let views: Vec<_> = c
            .implemented_interface_views(leaf)
            .into_iter()
            .map(|(b, a)| (b, crate::rs_type::render_arg_list(&a, &f.names)))
            .collect();
        assert_eq!(views, [("p/List".to_string(), "<String>".to_string()), ("p/Coll".into(), "<String>".into())]);
        let sv = c.interface_signature_views(leaf);
        assert_eq!(sv.len(), 2);
        assert_eq!(sv[1].0, "p/Coll");
        assert_eq!(sv[1].1["E"], "Ljava/lang/String;");
    }
}

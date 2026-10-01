//! 泛型签名（JVMS §4.7.9）递归下降解析 → [`RsType`]。
//!
//! 解析位置以字节下标推进：签名的分隔符（`<>;.:[+-*^()`）全是 ASCII，UTF-8 多字节
//! 序列不含 ASCII 字节，按字节切片总落在字符边界上。

mod bounds;
mod substitute;

use std::collections::BTreeMap;

pub use substitute::substitute_signature_type_vars;

use crate::class_params::{is_prim_char, parse_class_type_params};
use crate::consts;
use crate::registry::ClassInfo;
use crate::rs_type::RsType;
use crate::TyCtx;

/// 方法级类型变量 → 上界类型
pub type Bounds = BTreeMap<String, RsType>;

/// 方法签名解析结果：参数类型 + 返回类型（`^` 抛出段忽略；void → `()`）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodSigTypes {
    pub params: Vec<RsType>,
    pub ret: RsType,
}

/// 签名里的类名结束符
fn is_name_end(c: u8) -> bool {
    matches!(c, b'<' | b';' | b'.')
}

impl TyCtx<'_> {
    /// `_CLASSNAME_MAP`：签名解析直映射的类
    fn mapped_class(&self, binary: &str) -> Option<RsType> {
        if binary == consts::STRING {
            return Some(RsType::class(consts::STRING, Vec::new()));
        }
        if binary == consts::OBJECT || self.manifest.erased_interfaces.contains(binary) {
            return Some(RsType::Object);
        }
        if binary == consts::CLASS {
            return Some(RsType::Bare {
                binary: binary.to_string(),
            });
        }
        None
    }

    /// 解析 `<TypeArgument*>`（`i` 指向 `<`），返回 (实参表, `>` 之后位置)
    pub(crate) fn parse_type_args(
        &self,
        sig: &str,
        mut i: usize,
        tps: &[String],
        bounds: Option<&Bounds>,
    ) -> (Vec<RsType>, usize) {
        let b = sig.as_bytes();
        i += 1;
        let mut args = Vec::new();
        while i < b.len() && b[i] != b'>' {
            let (t, next) = match b[i] {
                b'*' => (RsType::Object, i + 1),
                b'+' | b'-' => self.parse_one_type(sig, i + 1, tps, bounds),
                _ => self.parse_one_type(sig, i, tps, bounds),
            };
            args.push(t);
            i = next;
        }
        if i < b.len() && b[i] == b'>' {
            i += 1;
        }
        (args, i)
    }

    /// 从 `sig[i]` 起解析一个类型，返回 (类型, 其后位置)
    pub(crate) fn parse_one_type(
        &self,
        sig: &str,
        i: usize,
        tps: &[String],
        bounds: Option<&Bounds>,
    ) -> (RsType, usize) {
        let b = sig.as_bytes();
        let Some(&c) = b.get(i) else {
            return (RsType::Object, i);
        };
        if is_prim_char(c) {
            return (RsType::from_prim_desc(c).unwrap_or(RsType::Object), i + 1);
        }
        match c {
            b'T' => {
                let Some(end) = sig[i + 1..].find(';').map(|p| i + 1 + p) else {
                    return (RsType::Object, b.len());
                };
                let name = &sig[i + 1..end];
                if tps.iter().any(|p| p == name) {
                    return (RsType::Param(name.to_string()), end + 1);
                }
                if let Some(bt) = bounds.and_then(|m| m.get(name)) {
                    return (bt.clone(), end + 1);
                }
                (RsType::Object, end + 1)
            }
            b'[' => {
                let (elem, next) = self.parse_one_type(sig, i + 1, tps, bounds);
                (RsType::array(elem), next)
            }
            b'+' | b'-' => self.parse_one_type(sig, i + 1, tps, bounds),
            b'*' => (RsType::Object, i + 1),
            b'L' => self.parse_class_type(sig, i, tps, bounds),
            _ => (RsType::Object, i + 1),
        }
    }

    /// ClassTypeSignature：`L<name>(<args>)?(.<Inner>(<args>)?)*;`
    fn parse_class_type(
        &self,
        sig: &str,
        i: usize,
        tps: &[String],
        bounds: Option<&Bounds>,
    ) -> (RsType, usize) {
        let b = sig.as_bytes();
        let mut j = i + 1;
        while j < b.len() && !is_name_end(b[j]) {
            j += 1;
        }
        let mut class_name = sig[i + 1..j].to_string();
        let mut type_args: Vec<RsType> = Vec::new();
        let mut has_type_args = b.get(j) == Some(&b'<');
        if has_type_args {
            (type_args, j) = self.parse_type_args(sig, j, tps, bounds);
        }
        if b.get(j) == Some(&b'.') {
            // ClassTypeSigSuffix：类型是内部类 Outer$Inner，实参按内部类的有效形参选取
            let mut outer_ci = self.reg.get(&class_name);
            while b.get(j) == Some(&b'.') {
                let mut k = j + 1;
                while k < b.len() && !is_name_end(b[k]) {
                    k += 1;
                }
                class_name = format!("{class_name}${}", &sig[j + 1..k]);
                j = k;
                let mut seg_args = Vec::new();
                if b.get(j) == Some(&b'<') {
                    (seg_args, j) = self.parse_type_args(sig, j, tps, bounds);
                }
                let inner_ci = self.reg.get(&class_name);
                type_args = match inner_ci {
                    Some(inner) => {
                        self.inner_class_type_args(outer_ci, &type_args, inner, seg_args, tps)
                    }
                    None => seg_args,
                };
                outer_ci = inner_ci;
            }
            has_type_args = !type_args.is_empty();
        } else if has_type_args {
            if let Some(ci) = self.reg.get(&class_name) {
                // 局部类 `LOuter$1Local<TX;>;`：继承自外围作用域的形参按当前作用域补齐
                type_args = self.inner_class_type_args(None, &[], ci, type_args, tps);
            }
        }
        if !has_type_args {
            if let Some(raw_ci) = self.reg.get(&class_name) {
                if !raw_ci.is_interface() && self.mapped_class(&class_name).is_none() {
                    let raw_eff = self.effective_class_type_params(raw_ci);
                    if !raw_eff.is_empty() {
                        let raw_own = parse_class_type_params(raw_ci.generic_signature());
                        type_args = if raw_own.is_empty() && raw_eff.iter().all(|p| tps.contains(p))
                        {
                            raw_eff.iter().map(|p| RsType::Param(p.clone())).collect()
                        } else {
                            RsType::objects(raw_eff.len())
                        };
                        has_type_args = true;
                    }
                }
            }
        }
        if b.get(j) == Some(&b';') {
            j += 1;
        }
        (
            self.class_type_node(&class_name, type_args, has_type_args),
            j,
        )
    }

    /// 类名 + 已解析实参 → 类型节点（接口载体 / 直映射 / 闭包外擦除 / 短名）
    fn class_type_node(
        &self,
        class_name: &str,
        type_args: Vec<RsType>,
        has_type_args: bool,
    ) -> RsType {
        if let Some(carrier) = self.carrier_type(class_name) {
            return carrier;
        }
        if let Some(mapped) = self.mapped_class(class_name) {
            return mapped;
        }
        match self.reg.get(class_name) {
            None => RsType::Object,
            Some(_) if has_type_args => RsType::class(class_name, type_args),
            Some(_) => RsType::class(class_name, Vec::new()),
        }
    }

    /// `Outer<A..>.Inner<B..>` 中 Inner 的有效实参：继承自外围作用域的形参取外层实参
    /// （外层无对应实参：作用域内同名类型变量原样传递，否则 `Object`）+ 本段实参
    pub(crate) fn inner_class_type_args(
        &self,
        outer_ci: Option<&ClassInfo>,
        outer_args: &[RsType],
        inner_ci: &ClassInfo,
        seg_args: Vec<RsType>,
        scope: &[String],
    ) -> Vec<RsType> {
        let own = parse_class_type_params(inner_ci.generic_signature());
        let eff = self.effective_class_type_params(inner_ci);
        let inherited = if own.is_empty() {
            &eff[..]
        } else {
            &eff[..eff.len().saturating_sub(own.len())]
        };
        let own_args = if seg_args.len() == own.len() {
            seg_args
        } else {
            RsType::objects(own.len())
        };
        let outer_eff = outer_ci
            .map(|o| self.effective_class_type_params(o))
            .unwrap_or_default();
        let outer_map: BTreeMap<&str, &RsType> = if outer_eff.len() == outer_args.len() {
            outer_eff
                .iter()
                .map(String::as_str)
                .zip(outer_args)
                .collect()
        } else {
            BTreeMap::new()
        };
        let mut out: Vec<RsType> = inherited
            .iter()
            .map(|p| match outer_map.get(p.as_str()) {
                Some(t) => (*t).clone(),
                None if scope.contains(p) => RsType::Param(p.clone()),
                None => RsType::Object,
            })
            .collect();
        out.extend(own_args);
        out
    }

    /// 字段级 Signature → Rust 类型；空签名 → None
    pub fn parse_field_type(&self, sig: &str, tps: &[String]) -> Option<RsType> {
        if sig.is_empty() {
            return None;
        }
        Some(self.parse_one_type(sig, 0, tps, None).0)
    }

    /// 方法 Signature → (参数类型, 返回类型)；空签名或不以 `(` 开始参数段 → None。
    ///
    /// `is_static = false` 时方法级形参遮蔽同名类级形参（从 `tps` 剔除，按方法级变量——
    /// 上界 / `Object`——处理）。
    pub fn parse_method_param_types(
        &self,
        sig: &str,
        tps: &[String],
        is_static: bool,
    ) -> Option<MethodSigTypes> {
        if sig.is_empty() {
            return None;
        }
        let b = sig.as_bytes();
        let mut i = 0;
        let mut bounds = Bounds::new();
        let mut scope: Vec<String> = tps.to_vec();
        if b[0] == b'<' {
            bounds = self.extract_method_tparam_bounds(sig, &[], None);
            if !is_static {
                let shadowed = parse_class_type_params(sig);
                if shadowed.iter().any(|s| scope.contains(s)) {
                    scope.retain(|p| !shadowed.contains(p));
                }
            }
            let mut depth = 1;
            i = 1;
            while i < b.len() && depth > 0 {
                match b[i] {
                    b'<' => depth += 1,
                    b'>' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
        }
        if b.get(i) != Some(&b'(') {
            return None;
        }
        i += 1;
        let bref = (!bounds.is_empty()).then_some(&bounds);
        let mut params = Vec::new();
        while i < b.len() && b[i] != b')' {
            let (t, next) = self.parse_one_type(sig, i, &scope, bref);
            params.push(t);
            i = next;
        }
        if b.get(i) == Some(&b')') {
            i += 1;
        }
        let ret = if i < b.len() && b[i] != b'^' {
            self.parse_one_type(sig, i, &scope, bref).0
        } else {
            RsType::Unit
        };
        Some(MethodSigTypes { params, ret })
    }
}

#[cfg(test)]
mod tests {
    use crate::consts;
    use crate::testutil::{class, Fixture};

    fn fixture() -> Fixture {
        Fixture::new(vec![
            class(consts::OBJECT).sup(""),
            class(consts::STRING),
            class("p/List")
                .iface()
                .sig("<E:Ljava/lang/Object;>Ljava/lang/Object;"),
            class("p/Box").sig("<T:Ljava/lang/Object;>Ljava/lang/Object;"),
            class("p/Num"),
            class("p/Outer").sig("<A:Ljava/lang/Object;>Ljava/lang/Object;"),
            class("p/Outer$In")
                .sig("<B:Ljava/lang/Object;>Ljava/lang/Object;")
                .field(crate::testutil::field(0x1010, "this$0", "Lp/Outer;", None)),
        ])
    }

    fn tps(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn field_types() {
        let f = fixture();
        let c = f.ctx();
        let r = |s: &str, t: &[&str]| c.parse_field_type(s, &tps(t)).map(|x| x.render(&f.names));
        assert_eq!(r("Lp/Box<TT;>;", &["T"]).as_deref(), Some("Box<T>"));
        assert_eq!(r("Lp/Box;", &[]).as_deref(), Some("Box<Object>"));
        assert_eq!(
            r("Lp/List<Ljava/lang/String;>;", &[]).as_deref(),
            Some("List<Object>")
        );
        assert_eq!(r("[Lp/Missing;", &[]).as_deref(), Some("JArray<Object>"));
        assert_eq!(
            r("Lp/Outer<Lp/Num;>.In<TX;>;", &["X"]).as_deref(),
            Some("Outer_In<Num, X>")
        );
        assert_eq!(r("", &[]), None);
    }

    #[test]
    fn method_types_with_bounds() {
        let f = fixture();
        let c = f.ctx();
        let m = c
            .parse_method_param_types("<T:Lp/Num;>(TT;TE;I)[TT;", &tps(&["E"]), true)
            .unwrap();
        let params: Vec<_> = m.params.iter().map(|t| t.render(&f.names)).collect();
        assert_eq!(params, ["Num", "E", "i32"]);
        assert_eq!(m.ret.render(&f.names), "JArray<Num>");
        // 实例方法：方法级 E 遮蔽类级 E
        let m = c
            .parse_method_param_types("<E:Ljava/lang/Object;>(TE;)V", &tps(&["E"]), false)
            .unwrap();
        assert_eq!(m.params[0].render(&f.names), "Object");
        assert_eq!(m.ret.render(&f.names), "()");
        assert!(c.parse_method_param_types("I", &[], true).is_none());
    }
}

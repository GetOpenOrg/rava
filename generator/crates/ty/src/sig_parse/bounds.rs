//! 类型形参段 `<T:Lp/Bound;..>` 的上界提取（`sig_parse._extract_method_tparam_bounds`）。

use std::collections::BTreeMap;

use super::Bounds;
use crate::rs_type::RsType;
use crate::TyCtx;

/// Python `str.isalpha() or == '_'` 的字节口径：非 ASCII 字节一律视为字母
/// （类型变量名的首字符；签名分隔符全是 ASCII）
fn is_name_start(c: u8) -> bool {
    c.is_ascii_alphabetic() || c == b'_' || c >= 0x80
}

impl TyCtx<'_> {
    /// 类型变量 → 首个具体类上界（`Object` 与接口载体上界被过滤）。
    ///
    /// - `type_params`：上界内可见的类型变量（类级形参段传类形参表；方法级传空 → 擦除为 `Object`）
    /// - `bound_binaries`：若提供，回填 类型变量 → 上界类 binary name
    pub fn extract_method_tparam_bounds(
        &self,
        sig: &str,
        type_params: &[String],
        mut bound_binaries: Option<&mut BTreeMap<String, String>>,
    ) -> Bounds {
        let b = sig.as_bytes();
        let mut bounds = Bounds::new();
        if b.first() != Some(&b'<') {
            return bounds;
        }
        let mut i = 1;
        let mut depth = 1;
        while i < b.len() && depth > 0 {
            let c = b[i];
            if c == b'<' {
                depth += 1;
                i += 1;
            } else if c == b'>' {
                depth -= 1;
                i += 1;
            } else if is_name_start(c) {
                let mut j = i;
                while j < b.len() && !matches!(b[j], b':' | b'<' | b'>') {
                    j += 1;
                }
                let name = &sig[i..j];
                i = j;
                let mut first: Option<RsType> = None;
                while i < b.len() && b[i] == b':' {
                    i += 1;
                    match b.get(i) {
                        Some(b'L' | b'[') => {
                            let start = i;
                            let (t, next) = self.parse_one_type(sig, i, type_params, None);
                            i = next;
                            if first.is_none() && t.render(self.names) != "Object" && !self.is_carrier(&t) {
                                if let (Some(bb), b'L') = (bound_binaries.as_deref_mut(), b[start]) {
                                    let rest = &sig[start + 1..];
                                    let end = rest.find(['<', ';', '.']).unwrap_or(rest.len());
                                    bb.insert(name.to_string(), rest[..end].to_string());
                                }
                                first = Some(t);
                            }
                        }
                        Some(b'T') => i = self.parse_one_type(sig, i, &[], None).1,
                        _ => {}
                    }
                }
                if let (false, Some(t)) = (name.is_empty(), first) {
                    bounds.insert(name.to_string(), t);
                }
            } else {
                i += 1;
            }
        }
        bounds
    }
}

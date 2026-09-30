//! JVM 泛型签名文本内的类型变量代入（`sig_parse.substitute_signature_type_vars`）。

use std::collections::BTreeMap;

use crate::class_params::parse_class_type_params;

/// 把 `sig[i..]` 处的一个类型签名写入 `out`（类型变量按 mapping 代入），返回其后位置
fn substitute_type_sig(
    sig: &str,
    i: usize,
    out: &mut String,
    mapping: &BTreeMap<String, String>,
) -> usize {
    let b = sig.as_bytes();
    let Some(&c) = b.get(i) else {
        return i;
    };
    match c {
        b'[' | b'+' | b'-' => {
            out.push(c as char);
            substitute_type_sig(sig, i + 1, out, mapping)
        }
        b'T' => match sig[i..].find(';').map(|p| i + p) {
            None => {
                out.push_str(&sig[i..]);
                b.len()
            }
            Some(end) => {
                match mapping.get(&sig[i + 1..end]) {
                    Some(v) => out.push_str(v),
                    None => out.push_str(&sig[i..=end]),
                }
                end + 1
            }
        },
        b'L' => {
            let mut i = i;
            while i < b.len() {
                match b[i] {
                    b';' => {
                        out.push(';');
                        return i + 1;
                    }
                    b'<' => {
                        out.push('<');
                        i += 1;
                        while i < b.len() && b[i] != b'>' {
                            i = substitute_type_sig(sig, i, out, mapping);
                        }
                    }
                    _ => {
                        // 名字段按字符推进（保持 UTF-8 完整）
                        let ch = sig[i..].chars().next().map_or(1, char::len_utf8);
                        out.push_str(&sig[i..i + ch]);
                        i += ch;
                    }
                }
            }
            i
        }
        _ => {
            let ch = sig[i..].chars().next().map_or(1, char::len_utf8);
            out.push_str(&sig[i..i + ch]);
            i + ch
        }
    }
}

/// 签名里的类型变量按 mapping（名字 → 类型签名）代入；方法自身声明的类型形参
/// 遮蔽同名外层变量。形参段缺 `:` 的残缺签名 → None（Python 抛 ValueError）。
pub fn substitute_signature_type_vars(
    sig: &str,
    mapping: &BTreeMap<String, String>,
) -> Option<String> {
    if sig.is_empty() || mapping.is_empty() {
        return Some(sig.to_string());
    }
    let b = sig.as_bytes();
    let mut out = String::with_capacity(sig.len());
    let mut i = 0;
    let shadowed;
    let mut mapping = mapping;
    if b[0] == b'<' {
        let own = parse_class_type_params(sig);
        shadowed = mapping
            .iter()
            .filter(|(k, _)| !own.contains(k))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        mapping = &shadowed;
        if mapping.is_empty() {
            return Some(sig.to_string());
        }
        out.push('<');
        i = 1;
        while i < b.len() && b[i] != b'>' {
            let colon = i + sig[i..].find(':')?;
            out.push_str(&sig[i..colon]);
            i = colon;
            while i < b.len() && b[i] == b':' {
                out.push(':');
                i += 1;
                if i < b.len() && matches!(b[i], b'L' | b'T' | b'[') {
                    i = substitute_type_sig(sig, i, &mut out, mapping);
                }
            }
        }
        out.push('>');
        i += 1;
    }
    while i < b.len() {
        if matches!(b[i], b'(' | b')' | b'^') {
            out.push(b[i] as char);
            i += 1;
        } else {
            i = substitute_type_sig(sig, i, &mut out, mapping);
        }
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map(pairs: &[(&str, &str)]) -> BTreeMap<String, String> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect()
    }

    #[test]
    fn substitutes_and_shadows() {
        let m = map(&[("E", "Lp/S;"), ("T", "Lp/X;")]);
        assert_eq!(
            substitute_signature_type_vars("(TE;Lp/L<TE;>;)[TE;", &m).as_deref(),
            Some("(Lp/S;Lp/L<Lp/S;>;)[Lp/S;")
        );
        assert_eq!(
            substitute_signature_type_vars("<T:Ljava/lang/Object;>(TT;TE;)TT;", &m).as_deref(),
            Some("<T:Ljava/lang/Object;>(TT;Lp/S;)TT;")
        );
        assert_eq!(
            substitute_signature_type_vars("Lp/A<+TE;*>;", &m).as_deref(),
            Some("Lp/A<+Lp/S;*>;")
        );
    }
}

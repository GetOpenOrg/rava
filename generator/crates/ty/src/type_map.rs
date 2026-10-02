//! JVM 描述符 → Rust 类型与重载后缀（`type_map.jvm_to_rust` / `parse_descriptor_*` /
//! `descriptor_to_suffix` / `mangle_name`）。

use crate::consts;
use crate::manifest::Manifest;
use crate::rs_type::RsType;
use crate::TyCtx;

/// 方法描述符的参数段 `(..)` 内文；不以 `(` 开头或缺 `)` → None
fn param_section(desc: &str) -> Option<&str> {
    let rest = desc.strip_prefix('(')?;
    rest.find(')').map(|p| &rest[..p])
}

/// 方法描述符 → 参数描述符表（`(I[JLp/A;)V` → `[I, [J, Lp/A;]`）
pub fn parse_descriptor_params(desc: &str) -> Vec<String> {
    parse_type_list(param_section(desc).unwrap_or(""))
}

/// 方法描述符 → 返回描述符（格式不符 → `V`）
pub fn parse_descriptor_return(desc: &str) -> &str {
    match desc
        .strip_prefix('(')
        .and_then(|r| r.find(')').map(|p| &r[p + 1..]))
    {
        Some(r) => r,
        None => "V",
    }
}

fn parse_type_list(s: &str) -> Vec<String> {
    let b = s.as_bytes();
    let mut types = Vec::new();
    let mut i = 0;
    while i < b.len() {
        match b[i] {
            b'B' | b'C' | b'D' | b'F' | b'I' | b'J' | b'S' | b'Z' | b'V' => {
                types.push(s[i..i + 1].to_string());
                i += 1;
            }
            b'L' => {
                let end = s[i..].find(';').map(|p| i + p).unwrap_or(b.len() - 1);
                types.push(s[i..=end].to_string());
                i = end + 1;
            }
            b'[' => {
                let mut j = i + 1;
                while j < b.len() && b[j] == b'[' {
                    j += 1;
                }
                if b.get(j) == Some(&b'L') {
                    let end = s[j..].find(';').map(|p| j + p).unwrap_or(b.len() - 1);
                    types.push(s[i..=end].to_string());
                    i = end + 1;
                } else {
                    let end = (j + 1).min(b.len());
                    types.push(s[i..end].to_string());
                    i = end;
                }
            }
            _ => i += 1,
        }
    }
    types
}

fn prim_suffix(c: u8) -> Option<&'static str> {
    Some(match c {
        b'I' => "i",
        b'J' => "l",
        b'Z' => "z",
        b'B' => "b",
        b'S' => "s",
        b'F' => "f",
        b'D' => "d",
        b'C' => "c",
        _ => return None,
    })
}

fn class_suffix(manifest: &Manifest, binary: &str) -> String {
    let short = binary
        .rsplit('/')
        .next()
        .unwrap_or(binary)
        .to_lowercase()
        .replace('$', "_");
    manifest
        .overload_abbrev
        .get(&short)
        .cloned()
        .unwrap_or(short)
}

/// 描述符参数部分 → 重载后缀（不含分隔符），如 `(ITE;)V` → `i_e`
pub fn descriptor_to_suffix(manifest: &Manifest, descriptor: &str) -> String {
    let Some(s) = param_section(descriptor) else {
        return String::new();
    };
    let b = s.as_bytes();
    let find_semi = |from: usize| s.get(from..).and_then(|t| t.find(';')).map(|p| from + p);
    let mut parts: Vec<String> = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let c = b[i];
        if let Some(p) = prim_suffix(c) {
            parts.push(p.to_string());
            i += 1;
        } else if c == b'T' || c == b'L' {
            match find_semi(i + 1) {
                Some(end) if c == b'T' => {
                    parts.push(s[i + 1..end].to_lowercase());
                    i = end + 1;
                }
                Some(end) => {
                    parts.push(class_suffix(manifest, &s[i + 1..end]));
                    i = end + 1;
                }
                None => i += 1,
            }
        } else if c == b'[' {
            let mut j = i + 1;
            while j < b.len() && b[j] == b'[' {
                j += 1;
            }
            match b.get(j) {
                Some(&k @ (b'L' | b'T')) => match find_semi(j + 1) {
                    Some(end) => {
                        let body = &s[j + 1..end];
                        let name = if k == b'L' {
                            class_suffix(manifest, body)
                        } else {
                            body.to_lowercase()
                        };
                        parts.push(format!("arr_{name}"));
                        i = end + 1;
                    }
                    None => i = j + 1,
                },
                Some(&p) => {
                    parts.push(format!("arr_{}", prim_suffix(p).unwrap_or("x")));
                    i = j + 1;
                }
                None => i += 1,
            }
        } else {
            i += 1;
        }
    }
    parts.join("_")
}

/// 方法名 + 描述符 → 含后缀的唯一 Rust 名；无参数时为原名
pub fn mangle_name(manifest: &Manifest, name: &str, descriptor: &str) -> String {
    let suffix = descriptor_to_suffix(manifest, descriptor);
    if suffix.is_empty() {
        name.to_string()
    } else {
        format!("{name}_{suffix}")
    }
}

impl TyCtx<'_> {
    /// JVM 字段描述符 → Rust 类型：注册表内的类取短名（接口 → 载体；泛型类实参全取
    /// `Object`），注册表外 → `Object`
    pub fn jvm_to_rust(&self, t: &str) -> RsType {
        let b = t.as_bytes();
        if b.len() == 1 {
            if let Some(p) = RsType::from_prim_desc(b[0]) {
                return p;
            }
        }
        if let Some(inner) = t.strip_prefix('L').and_then(|r| r.strip_suffix(';')) {
            if inner == consts::OBJECT {
                return RsType::Object;
            }
            let Some(ci) = self.reg.get(inner) else {
                return RsType::Object;
            };
            if self.global_names().short(inner).is_empty() {
                return RsType::Object;
            }
            if ci.is_interface() {
                return self.carrier_type(inner).unwrap_or(RsType::Object);
            }
            let n = self.effective_class_type_params(ci).len();
            return RsType::class(inner, RsType::objects(n));
        }
        if t.len() == 2 && b[0] == b'[' {
            if let Some(p) = RsType::from_prim_desc(b[1]).filter(|p| *p != RsType::Unit) {
                return RsType::array(p);
            }
        }
        if t == format!("[L{};", consts::STRING) {
            return RsType::array(RsType::class(consts::STRING, Vec::new()));
        }
        if let Some(rest) = t.strip_prefix('[') {
            return RsType::array(self.jvm_to_rust(rest));
        }
        RsType::Object
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn manifest() -> Manifest {
        let mut m = Manifest::default();
        for (k, v) in [("object", "obj"), ("string", "str"), ("integer", "int")] {
            m.overload_abbrev.insert(k.into(), v.into());
        }
        m
    }

    #[test]
    fn suffix_and_mangle() {
        let m = manifest();
        assert_eq!(descriptor_to_suffix(&m, "(ITE;)V"), "i_e");
        assert_eq!(
            descriptor_to_suffix(&m, "(Ljava/lang/Object;[I[[Ljava/lang/String;)V"),
            "obj_arr_i_arr_str"
        );
        assert_eq!(
            descriptor_to_suffix(&m, "(Ljava/util/Map$Entry;[TT;)V"),
            "map_entry_arr_t"
        );
        assert_eq!(mangle_name(&m, "add", "()V"), "add");
        assert_eq!(mangle_name(&m, "add", "(Ljava/lang/Integer;)Z"), "add_int");
    }

    #[test]
    fn descriptor_params() {
        assert_eq!(
            parse_descriptor_params("(I[JLp/A;[[Lq/B;)V"),
            ["I", "[J", "Lp/A;", "[[Lq/B;"]
        );
        assert_eq!(parse_descriptor_return("(I)Lp/A;"), "Lp/A;");
        assert!(parse_descriptor_params("()V").is_empty());
    }
}

//! JVM 描述符 → Rust 类型（M-3 纯位映射，`docs/plans/2026-09-25-m3-signature-types-in-macro.md`）。
//!
//! 「纯位」= 只凭描述符字符串即可确定 Rust 类型、与生成闭包 / registry 无关的位置：
//! 基本类型、基本类型数组（含多维）、`String` / `Object` 及其（多维）数组。口径与生成器
//! `codegen/type_map.py` 的 `JVM_RUST` / `jvm_to_rust` 逐项一致（`C` → `u16`、
//! 数组 → `JArray<T>`；装箱类是引用类型，不属纯位——S-3.1）。
//! 其余引用类型（是否在闭包、接口载体、同名消歧、形参个数）依赖 registry，返回 `None`。

const OBJECT_DESC: &str = "Ljava/lang/Object;";
const STRING_DESC: &str = "Ljava/lang/String;";

/// 单个字段描述符 → 纯位 Rust 类型；非纯位或非法 → `None`。
pub(crate) fn pure_rust_type(desc: &str) -> Option<String> {
    let prim = match desc {
        "I" => Some("i32"),
        "J" => Some("i64"),
        "F" => Some("f32"),
        "D" => Some("f64"),
        "Z" => Some("bool"),
        "B" => Some("i8"),
        "S" => Some("i16"),
        "C" => Some("u16"),
        "V" => Some("()"),
        OBJECT_DESC => Some("Object"),
        STRING_DESC => Some("String"),
        _ => None,
    };
    if let Some(p) = prim {
        return Some(p.to_owned());
    }
    let elem = desc.strip_prefix('[')?;
    if elem == "V" {
        return None;
    }
    Some(format!("JArray<{}>", pure_rust_type(elem)?))
}

/// 方法描述符 → (参数字段描述符列表, 返回字段描述符)；非法 → `None`。
pub(crate) fn split_method_descriptor(desc: &str) -> Option<(Vec<String>, String)> {
    let body = desc.strip_prefix('(')?;
    let close = body.find(')')?;
    let (params_str, ret) = (&body[..close], &body[close + 1..]);
    let bytes = params_str.as_bytes();
    let mut params = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let start = i;
        while i < bytes.len() && bytes[i] == b'[' {
            i += 1;
        }
        match bytes.get(i)? {
            b'L' => i += params_str[i..].find(';')? + 1,
            b'I' | b'J' | b'F' | b'D' | b'Z' | b'B' | b'S' | b'C' => i += 1,
            _ => return None,
        }
        params.push(params_str[start..i].to_owned());
    }
    if ret.is_empty() {
        return None;
    }
    Some((params, ret.to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primitives() {
        for (d, r) in [("I", "i32"), ("J", "i64"), ("F", "f32"), ("D", "f64"), ("Z", "bool"),
                       ("B", "i8"), ("S", "i16"), ("C", "u16"), ("V", "()")] {
            assert_eq!(pure_rust_type(d).as_deref(), Some(r), "{d}");
        }
    }

    #[test]
    fn arrays() {
        assert_eq!(pure_rust_type("[I").as_deref(), Some("JArray<i32>"));
        assert_eq!(pure_rust_type("[C").as_deref(), Some("JArray<u16>"));
        assert_eq!(pure_rust_type("[[D").as_deref(), Some("JArray<JArray<f64>>"));
        assert_eq!(pure_rust_type("[Ljava/lang/String;").as_deref(), Some("JArray<String>"));
        assert_eq!(pure_rust_type("[[Ljava/lang/Object;").as_deref(), Some("JArray<JArray<Object>>"));
    }

    #[test]
    fn references() {
        assert_eq!(pure_rust_type("Ljava/lang/Object;").as_deref(), Some("Object"));
        assert_eq!(pure_rust_type("Ljava/lang/String;").as_deref(), Some("String"));
        // registry 依赖位（含装箱类、接口、数组元素为普通类）→ None
        for d in ["Ljava/lang/Integer;", "Ljava/util/List;", "[Ljava/util/Map;", "[V", "X", "", "["] {
            assert_eq!(pure_rust_type(d), None, "{d}");
        }
    }

    #[test]
    fn method_descriptor() {
        let (p, r) = split_method_descriptor("(I[JLjava/lang/String;[[Ljava/util/List;DZ)V").unwrap();
        assert_eq!(p, ["I", "[J", "Ljava/lang/String;", "[[Ljava/util/List;", "D", "Z"]);
        assert_eq!(r, "V");
        let (p, r) = split_method_descriptor("()[Ljava/lang/Object;").unwrap();
        assert!(p.is_empty());
        assert_eq!(r, "[Ljava/lang/Object;");
        for bad in ["I)V", "(I", "(Q)V", "(Ljava/lang/String)V", "(I)", "([)V"] {
            assert_eq!(split_method_descriptor(bad), None, "{bad}");
        }
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// M3-b 差分审计：纯位上「生成器已发射类型」vs「描述符推导类型」
// ══════════════════════════════════════════════════════════════════════════════

/// 审计开关：环境变量 `JAVA_RTA_M3_AUDIT=<输出文件>`（展开期读取；未设置 = 不审计，
/// 零产物变化）。每个被审计方法追加一行：`OK|MISMATCH <类> <方法><描述符> …`。
pub(crate) const AUDIT_ENV: &str = "JAVA_RTA_M3_AUDIT";

/// `Result<T>` → `T` 的 token 文本；非 Result → 原样。
fn result_inner(ty: &syn::Type) -> String {
    if let syn::Type::Path(p) = ty {
        if let Some(seg) = p.path.segments.last() {
            if seg.ident == "Result" {
                if let syn::PathArguments::AngleBracketed(a) = &seg.arguments {
                    if let Some(syn::GenericArgument::Type(t)) = a.args.first() {
                        return norm(&quote::quote!(#t).to_string());
                    }
                }
            }
        }
    }
    norm(&quote::quote!(#ty).to_string())
}

fn norm(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

/// 审计一个方法：返回 (已比对位数, 不一致明细)。跳过条件：无 descriptor、构造器族
/// （隐式外部实例形参）、参数个数与描述符不符（合成形参）。含 `Object` 的位置在
/// 方法带 generic_signature 或为覆盖 / 继承声明时跳过（祖先泛型签名可给出 `T`——
/// 那是形参位，不是纯位）。
pub(crate) fn audit_fn(f: &super::parse::FnItem, self_name: &str) -> Option<(usize, Vec<String>)> {
    let desc = super::util::attr_str(&f.attrs, "descriptor")?;
    let name = f.sig.ident.to_string();
    if name == "new" || name.starts_with("new_") || name.starts_with("__") {
        return None;
    }
    let (params, ret) = split_method_descriptor(&desc)?;
    let emitted: Vec<&syn::Type> = f.sig.inputs.iter().filter_map(|a| match a {
        syn::FnArg::Typed(pt) => Some(&*pt.ty),
        syn::FnArg::Receiver(_) => None,
    }).collect();
    if emitted.len() != params.len() {
        return None;
    }
    let generic = super::util::attr_str(&f.attrs, "generic_signature").is_some_and(|s| !s.is_empty());
    let virtual_in = super::util::attr_str(&f.attrs, "virtual_in");
    let inherited = super::util::attr_str(&f.attrs, "inherited_from").is_some();
    let foreign = inherited || virtual_in.as_deref().is_some_and(|v| v != self_name);
    let object_ok = !(generic || foreign);
    let mut checked = 0usize;
    let mut diffs = Vec::new();
    let mut check = |pos: &str, d: &str, got: String| {
        if d.contains("Ljava/lang/Object;") && !object_ok {
            return;
        }
        if let Some(want) = pure_rust_type(d) {
            checked += 1;
            if norm(&want) != got {
                diffs.push(format!("{pos} {d} want={want} got={got}"));
            }
        }
    };
    for (i, (d, t)) in params.iter().zip(emitted).enumerate() {
        check(&format!("p{i}"), d, norm(&quote::quote!(#t).to_string()));
    }
    if let syn::ReturnType::Type(_, t) = &f.sig.output {
        check("ret", &ret, result_inner(t));
    }
    Some((checked, diffs))
}

/// 审计整个类块的方法，结果追加到审计文件（按类一行汇总 + 每个不一致一行）。
pub(crate) fn audit_class(binary_name: &str, self_name: &str, fns: &[super::parse::FnItem]) {
    let Ok(path) = std::env::var(AUDIT_ENV) else { return };
    let (mut checked, mut lines) = (0usize, Vec::new());
    for f in fns {
        if let Some((n, diffs)) = audit_fn(f, self_name) {
            checked += n;
            let desc = super::util::attr_str(&f.attrs, "descriptor").unwrap_or_default();
            for d in diffs {
                lines.push(format!("MISMATCH {binary_name} {}{desc} {d}", f.sig.ident));
            }
        }
    }
    lines.insert(0, format!("CLASS {binary_name} checked={checked} mismatches={}", lines.len()));
    use std::io::Write;
    if let Ok(mut out) = std::fs::OpenOptions::new().create(true).append(true).open(path) {
        let _ = out.write_all((lines.join("\n") + "\n").as_bytes());
    }
}

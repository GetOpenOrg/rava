//! JVM generic_signature 解析 → Rust 泛型类型。

use syn::Signature;

// ══════════════════════════════════════════════════════════════════════════════
// JVM generic_signature 解析 → Rust 泛型类型
// ══════════════════════════════════════════════════════════════════════════════

/// 解析 JVM generic method signature，提取参数和返回类型。
/// 输入：`"(TK;TV;)TV;"` 或 `"(Ljava/util/List<TE;>;)V"`
/// 返回：`(params, return_type)` 每个元素是 Rust 类型字符串（`"K"`, `"V"`, `"Object"` 等）
pub(crate) fn parse_generic_method_sig(sig: &str) -> Option<(Vec<String>, String)> {
    if !sig.starts_with('(') { return None; }
    let close = sig.find(')')?;
    let params_str = &sig[1..close];
    let ret_str = &sig[close + 1..];
    let params = parse_jvm_type_sequence(params_str);
    let ret = parse_jvm_single_type(ret_str).unwrap_or_else(|| "()".to_owned());
    Some((params, ret))
}

fn parse_jvm_type_sequence(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < s.len() {
        let (ty, consumed) = parse_jvm_type_at(s, pos);
        if consumed == 0 { break; }
        result.push(ty);
        pos += consumed;
    }
    result
}

fn parse_jvm_single_type(s: &str) -> Option<String> {
    if s.is_empty() { return None; }
    let (ty, _) = parse_jvm_type_at(s, 0);
    Some(ty)
}

/// 在位置 pos 解析一个 JVM 泛型类型描述符，返回 (Rust 类型字符串, 消耗字节数)。
fn parse_jvm_type_at(s: &str, pos: usize) -> (String, usize) {
    let bytes = s.as_bytes();
    if pos >= bytes.len() { return ("Object".to_owned(), 0); }
    match bytes[pos] {
        b'T' => {
            let end = s[pos + 1..].find(';').map(|i| pos + 1 + i).unwrap_or(s.len());
            (s[pos + 1..end].to_owned(), end - pos + 1)
        }
        b'L' => {
            let mut depth = 0usize;
            let mut i = pos + 1;
            while i < bytes.len() {
                match bytes[i] {
                    b'<' => depth += 1,
                    b'>' => { if depth > 0 { depth -= 1; } }
                    b';' if depth == 0 => { i += 1; break; }
                    _ => {}
                }
                i += 1;
            }
            let inner = &s[pos + 1..i.saturating_sub(1)];
            let simple = inner.split('/').last().unwrap_or(inner);
            let base = simple.split('<').next().unwrap_or(simple);
            let rust_ty = match base {
                "String" => "String", "Object" => "Object",
                "Integer" => "i32", "Long" => "i64", "Boolean" => "bool",
                "Void" => "()", other => other,
            };
            (rust_ty.to_owned(), i - pos)
        }
        b'[' => {
            let (inner, consumed) = parse_jvm_type_at(s, pos + 1);
            (format!("Vec<{}>", inner), consumed + 1)
        }
        b'V' => ("()".to_owned(), 1),
        b'Z' => ("bool".to_owned(), 1),
        b'I' => ("i32".to_owned(), 1),
        b'J' => ("i64".to_owned(), 1),
        b'D' => ("f64".to_owned(), 1),
        b'F' => ("f32".to_owned(), 1),
        b'C' => ("char".to_owned(), 1),
        b'B' => ("i8".to_owned(), 1),
        b'S' => ("i16".to_owned(), 1),
        _ => ("Object".to_owned(), 1),
    }
}

/// 用 generic_signature 的类型变量重建方法的 Rust Signature。
/// 只替换 descriptor 擦除为 Object 的位置中匹配 class_type_params 的类型变量。
/// 返回 None 表示无需替换。
pub(crate) fn rebuild_sig_with_generics(
    sig: &Signature,
    generic_sig_str: &str,
    class_type_params: &[String],
) -> Option<Signature> {
    let (g_params, g_ret) = parse_generic_method_sig(generic_sig_str)?;
    let has_type_var = g_params.iter().any(|p| class_type_params.contains(p))
        || class_type_params.contains(&g_ret);
    if !has_type_var { return None; }

    let mut new_sig = sig.clone();

    let mut g_iter = g_params.iter();
    for arg in new_sig.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = arg {
            if let Some(g_ty) = g_iter.next() {
                if class_type_params.contains(g_ty) {
                    if let Ok(ty) = syn::parse_str::<syn::Type>(g_ty) {
                        *pt.ty = ty;
                    }
                }
            }
        }
    }

    if class_type_params.contains(&g_ret) {
        if let Ok(ty) = syn::parse_str::<syn::Type>(&format!("crate::error::Result<{}>", g_ret)) {
            new_sig.output = syn::ReturnType::Type(Default::default(), Box::new(ty));
        }
    }

    Some(new_sig)
}

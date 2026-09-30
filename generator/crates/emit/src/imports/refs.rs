//! 描述符 / 泛型签名文本中的类引用抽取（`import_gen._CLS_RE_NARROW` / `_CLS_RE_WIDE` /
//! `_superclass_arg_refs` 的移植；正则语义逐字符复现）。

use std::collections::BTreeSet;

use ty::ClassInfo;

use crate::ctx::EmitCtx;

/// `rust_type_head`：首个 `<` 之前
pub fn strip_generic(cls: &str) -> &str {
    cls.split('<').next().unwrap_or(cls)
}

fn wide_stop(c: char) -> bool {
    matches!(c, ';' | '<' | '>' | '[' | '(' | ')') || c.is_whitespace()
}

/// 宽松匹配 `L([^;<>\[()\s]+)`（泛型签名：嵌套 `<TT;>` 不截断）
pub fn wide_class_refs(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut it = text.char_indices().peekable();
    while let Some((i, c)) = it.next() {
        if c != 'L' {
            continue;
        }
        let start = i + 1;
        let mut end = start;
        while let Some(&(j, d)) = it.peek() {
            if wide_stop(d) {
                break;
            }
            end = j + d.len_utf8();
            it.next();
        }
        if end > start {
            out.push(&text[start..end]);
        }
    }
    out
}

/// 平铺描述符匹配 `L([^;]+);`
pub fn narrow_class_refs(text: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut i = 0;
    while let Some(off) = text[i..].find('L') {
        let start = i + off + 1;
        match text[start..].find(';') {
            Some(0) => i = start,
            Some(len) => {
                out.push(&text[start..start + len]);
                i = start + len + 1;
            }
            None => break,
        }
    }
    out
}

/// `_add_desc_refs`：宽松匹配 + 去泛型后非空者入集
pub fn add_desc_refs(text: &str, out: &mut BTreeSet<String>) {
    for c in wide_class_refs(text) {
        let c = strip_generic(c);
        if !c.is_empty() {
            out.insert(c.to_string());
        }
    }
}

/// 平铺描述符引用入集
pub fn add_narrow_refs(text: &str, out: &mut BTreeSet<String>) {
    for c in narrow_class_refs(text) {
        let c = strip_generic(c);
        if !c.is_empty() {
            out.insert(c.to_string());
        }
    }
}

/// `re.findall(r'\[*(?:L[^;]+;|[BCDFIJSZV])', desc)`：描述符的类型记号序列
pub fn desc_tokens(desc: &str) -> Vec<&str> {
    let b = desc.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < b.len() {
        let s = i;
        let mut j = i;
        while j < b.len() && b[j] == b'[' {
            j += 1;
        }
        if j < b.len() && b[j] == b'L' {
            if let Some(off) = desc[j + 1..].find(';') {
                if off > 0 {
                    let e = j + 1 + off + 1;
                    out.push(&desc[s..e]);
                    i = e;
                    continue;
                }
            }
        } else if j < b.len() && b"BCDFIJSZV".contains(&b[j]) {
            out.push(&desc[s..=j]);
            i = j + 1;
            continue;
        }
        i += 1;
    }
    out
}

/// 签名开头的类型形参段 `<...>` 之后的位置
fn after_type_params(sig: &str) -> usize {
    let b = sig.as_bytes();
    if b.first() != Some(&b'<') {
        return 0;
    }
    let mut depth = 0i32;
    for (i, &c) in b.iter().enumerate() {
        match c {
            b'<' => depth += 1,
            b'>' => {
                depth -= 1;
                if depth == 0 {
                    return i + 1;
                }
            }
            _ => {}
        }
    }
    b.len()
}

/// 超类段顶层实参文本（`LParent<Args>;` 的 `Args`）；无则 None
fn superclass_arg_text(sig: &str) -> Option<&str> {
    let i = after_type_params(sig);
    let b = sig.as_bytes();
    if b.get(i) != Some(&b'L') {
        return None;
    }
    let mut depth = 0i32;
    let mut sup_end = None;
    for (j, &c) in b.iter().enumerate().skip(i) {
        match c {
            b'<' => depth += 1,
            b'>' => depth -= 1,
            b';' if depth == 0 => {
                sup_end = Some(j);
                break;
            }
            _ => {}
        }
    }
    let sup_end = sup_end.filter(|&e| e > 0)?;
    let lt = i + sig[i..sup_end].find('<')?;
    let mut depth = 0i32;
    for (j, &c) in b.iter().enumerate().take(sup_end).skip(lt + 1) {
        match c {
            b'<' => depth += 1,
            b'>' => {
                depth -= 1;
                if depth < 0 {
                    return Some(&sig[lt + 1..j]);
                }
            }
            _ => {}
        }
    }
    None
}

/// `_superclass_arg_refs`：类操作数超类链各节点签名的超类顶层实参里引用的类
pub fn superclass_arg_refs(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> BTreeSet<String> {
    let reg = ctx.ty.reg;
    let mut out = BTreeSet::new();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut cur = ci;
    loop {
        let sup = cur.super_class();
        if seen.contains(cur.name()) || cur.name() == ty::consts::OBJECT || sup.is_empty() || sup == ty::consts::OBJECT {
            break;
        }
        let Some(next) = reg.get(sup) else { break };
        seen.insert(cur.name());
        if let Some(args) = superclass_arg_text(cur.generic_signature()) {
            add_desc_refs(args, &mut out);
        }
        cur = next;
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn class_ref_scanners() {
        assert_eq!(wide_class_refs("Ljava/util/Map<TK;Ljava/util/List<TV;>;>;"), vec!["java/util/Map", "java/util/List"]);
        assert_eq!(wide_class_refs("TLIST;"), vec!["IST"]);
        assert_eq!(narrow_class_refs("(ILa/B;[Lc/D;)V"), vec!["a/B", "c/D"]);
        assert_eq!(narrow_class_refs("(L;La/B;)"), vec!["a/B"]);
        assert_eq!(desc_tokens("(I[JLa/B;)V"), vec!["I", "[J", "La/B;", "V"]);
    }

    #[test]
    fn superclass_args() {
        assert_eq!(superclass_arg_text("<T:Ljava/lang/Object;>La/P<Lb/V;>;Lc/I;"), Some("Lb/V;"));
        assert_eq!(superclass_arg_text("La/P;La/I<Lx/Y;>;"), None);
        assert_eq!(superclass_arg_text("La/P<La/Q<Lb/V;>;>;"), Some("La/Q<Lb/V;>;"));
    }
}

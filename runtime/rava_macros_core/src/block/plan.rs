//! 生成器侧剥体计划（`plan` 特性，§7.5.2）：对 `java_class!` 块文本跑与宏相同的解析与
//! [`moved_fact`](super::moved::moved_fact) 判定，给出每个可下沉方法体的字节区间与标注。
//!
//! 记号取自 proc_macro2 回落实现（`span-locations`），字节区间相对传入文本起点。每次调用后
//! 清空本线程的回落源码表，长批次不累积内存。

use std::ops::Range;
use std::str::FromStr;

use proc_macro2::TokenStream as TokenStream2;

use super::gen::context::augment_generic_bounds;
use super::gen::GenContext;
use super::moved::plan_facts;
use super::parse::{ClassInput, ClassMeta, FnItem};

/// 一处剥体：在 `attr_at` 插入 `#[rava_moved = "<fact>"] `，`body`（`{ … }` 含花括号）换成 `;`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elision {
    pub attr_at: usize,
    pub body: Range<usize>,
    pub fact: &'static str,
}

/// 方法项起点：首个属性的 `#`，否则可见性 / 签名首记号
fn item_start(f: &FnItem) -> usize {
    if let Some(a) = f.attrs.first() {
        return a.pound_token.span.byte_range().start;
    }
    if let syn::Visibility::Public(p) = &f.vis {
        return p.span.byte_range().start;
    }
    let sig = &f.sig;
    let first = sig.constness.map(|t| t.span)
        .or(sig.asyncness.map(|t| t.span))
        .or(sig.unsafety.map(|t| t.span))
        .unwrap_or(sig.fn_token.span);
    first.byte_range().start
}

/// `inner` 是 `java_class! { … }` 花括号内的文本（不含花括号）。
/// 接口、泛型类与无可下沉体的类返回空表。
pub fn decl_elisions(inner: &str) -> Result<Vec<Elision>, String> {
    let out = compute(inner);
    proc_macro2::extra::invalidate_current_thread_spans();
    out
}

fn compute(inner: &str) -> Result<Vec<Elision>, String> {
    let ts = TokenStream2::from_str(inner).map_err(|e| format!("词法：{e}"))?;
    let input: ClassInput = syn::parse2(ts).map_err(|e| format!("解析：{e}"))?;
    let meta = ClassMeta::from_attrs(&input.attrs).map_err(|e| format!("类属性：{e}"))?;
    if meta.is_interface {
        return Ok(Vec::new());
    }
    let generics = augment_generic_bounds(&input.generics);
    let ctx = GenContext::build(&input, meta, generics);
    let mut out = Vec::new();
    for (i, m) in plan_facts(&ctx) {
        let f = &input.fns[i];
        let Some(block) = &f.block else { continue };
        let open = block.brace_token.span.open().byte_range().start;
        let close = block.brace_token.span.close().byte_range().end;
        out.push(Elision { attr_at: item_start(f), body: open..close, fact: m.as_str() });
    }
    out.sort_by_key(|e| e.attr_at);
    Ok(out)
}

/// 按计划改写块内文本：插标注、体（连同其前空白）换 `;`（计划按 `attr_at` 升序、互不重叠）
pub fn elide(inner: &str, plan: &[Elision]) -> String {
    let mut out = String::with_capacity(inner.len());
    let mut pos = 0;
    for e in plan {
        out.push_str(&inner[pos..e.attr_at]);
        out.push_str("#[rava_moved = \"");
        out.push_str(e.fact);
        out.push_str("\"] ");
        // 体前空白随体去掉：`fn f() -> R { … }` → `fn f() -> R;`
        out.push_str(inner[e.attr_at..e.body.start].trim_end());
        out.push(';');
        pos = e.body.end;
    }
    out.push_str(&inner[pos..]);
    out
}

/// 声明模式展开文本，摘要常量值与 try 标签序号归一（对照用：剥体前后展开只应差这两处）
pub fn decl_expansion_normalized(inner: &str) -> Result<String, String> {
    let ts = TokenStream2::from_str(inner).map_err(|e| format!("词法：{e}"))?;
    let s = super::expand(ts).to_string();
    proc_macro2::extra::invalidate_current_thread_spans();
    // `pub const __RAVA_MOVED_X : u64 = <n> ;` → 值换成 0
    let mut out = String::with_capacity(s.len());
    let mut rest = s.as_str();
    while let Some(p) = rest.find("__RAVA_MOVED_") {
        let Some(eq) = rest[p..].find("= ") else { break };
        let v0 = p + eq + 2;
        let v1 = v0 + rest[v0..].find(|c: char| !c.is_ascii_alphanumeric()).unwrap_or(rest.len() - v0);
        out.push_str(&rest[..v0]);
        out.push('0');
        rest = &rest[v1..];
    }
    out.push_str(rest);
    Ok(renumber_try_labels(&out))
}

/// `java_try!` 标签序号取自进程级计数器（序号只需在函数内唯一），剥体前后展开的体数不同会整体
/// 平移后续序号：按首次出现顺序重编号（`'java_try_N` 与 `'java_try_end_N` 共用 N），归一这种平移。
fn renumber_try_labels(s: &str) -> String {
    const PREFIXES: [&str; 2] = ["'java_try_end_", "'java_try_"];
    let mut map: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    let mut out = String::with_capacity(s.len());
    let mut rest = s;
    while let Some(p) = rest.find("'java_try_") {
        let prefix = PREFIXES.iter().find(|x| rest[p..].starts_with(**x)).copied().unwrap_or("'java_try_");
        let d0 = p + prefix.len();
        let d1 = d0 + rest[d0..].find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len() - d0);
        if d1 == d0 {
            out.push_str(&rest[..d0]);
            rest = &rest[d0..];
            continue;
        }
        let next = map.len();
        let k = *map.entry(&rest[d0..d1]).or_insert(next);
        out.push_str(&rest[..d0]);
        out.push_str(&k.to_string());
        rest = &rest[d1..];
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLASS: &str = r#"
    #[rava_layer = "decl"]
    #[binary_name = "a/B"]
    pub struct B { pub x: i32 }
    impl B {
        #[java_method(virtual_in = "B")]
        pub fn m(&self) -> Result<i32> { let this = self; Ok(this.__get_x()) }
        #[java_method(virtual_in = "B")]
        pub fn w(&self) -> Result<i32> { let this = self; this.s2(1) }
        pub fn s2(&self, a: i32) -> Result<i32> { Ok(a) }
        pub fn abs(&self) -> Result<i32>;
    }
"#;

    #[test]
    fn plan_marks_bodies_and_decl_expansion_is_unchanged() {
        let plan = decl_elisions(CLASS).unwrap();
        let facts: Vec<&str> = plan.iter().map(|e| e.fact).collect();
        assert_eq!(facts, vec!["safe", "wrapper", "plain"]);
        let elided = elide(CLASS, &plan);
        assert!(elided.contains("#[rava_moved = \"safe\"] #[java_method(virtual_in = \"B\")]\n        pub fn m(&self) -> Result<i32>;"));
        assert!(!elided.contains("__get_x"));
        let a = decl_expansion_normalized(CLASS).unwrap();
        let b = decl_expansion_normalized(&elided).unwrap();
        assert_eq!(a, b);
        assert!(!a.contains("compile_error"), "{a}");
    }

    #[test]
    fn try_labels_renumbered_by_first_use() {
        let a = renumber_try_labels("'java_try_end_7 : { 'java_try_7 : { break 'java_try_end_7 ; } } 'java_try_9 x");
        assert_eq!(a, "'java_try_end_0 : { 'java_try_0 : { break 'java_try_end_0 ; } } 'java_try_1 x");
    }

    #[test]
    fn body_assert_matches_decl_digest() {
        let plan = decl_elisions(CLASS).unwrap();
        let elided = elide(CLASS, &plan);
        let decl = super::super::expand(TokenStream2::from_str(&elided).unwrap()).to_string();
        let body_src = CLASS.replace("\"decl\"", "\"body\"");
        let body = super::super::expand(TokenStream2::from_str(&body_src).unwrap()).to_string();
        proc_macro2::extra::invalidate_current_thread_spans();
        let dv = decl.split("__RAVA_MOVED_B : u64 = ").nth(1).unwrap().split(' ').next().unwrap().to_string();
        let bv = body.split("__RAVA_MOVED_B == ").nth(1).unwrap().split(' ').next().unwrap().to_string();
        assert_eq!(dv, bv);
    }
}

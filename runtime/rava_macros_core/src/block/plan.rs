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
use super::util::attr_str;
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
    decl_plan(inner).map(|(e, _)| e)
}

/// 声明层留有的方法体（§9.9 K3 收尾）
#[derive(Debug, PartialEq, Eq)]
pub enum StayBodies {
    /// 无：全部方法体可下沉
    None,
    /// 留在声明层的 fn（java_method name, descriptor）：生成器按此差分出精确的方法 trait
    /// 导入需求（`ProjectState::method_traits`），替代整文件保留全部匿名导入
    Fns(Vec<(String, String)>),
    /// 某条留体 fn 缺 java_method 属性、无法按名差分（保守：整文件保留全部匿名导入）
    Unknown,
}

/// 剥体计划 + [`StayBodies`]：有体且不可下沉（wrapper 上下文 / 不可函数化，`moved_fact`
/// 为 None）的 fn 原样留在声明层。这类体的方法调用按推断类型解析，接收者类名可以不在文本
/// 中出现，文件头的方法 trait 匿名导入（§9.9 K3）不能按词剪枝（拆层 §7.5.4 #3）。
pub fn decl_plan(inner: &str) -> std::result::Result<(Vec<Elision>, StayBodies), String> {
    let out = compute(inner);
    proc_macro2::extra::invalidate_current_thread_spans();
    out
}

fn compute(inner: &str) -> std::result::Result<(Vec<Elision>, StayBodies), String> {
    let ts = TokenStream2::from_str(inner).map_err(|e| format!("词法：{e}"))?;
    let input: ClassInput = syn::parse2(ts).map_err(|e| format!("解析：{e}"))?;
    let meta = ClassMeta::from_attrs(&input.attrs).map_err(|e| format!("类属性：{e}"))?;
    if meta.is_interface {
        return Ok((Vec::new(), StayBodies::None));
    }
    let generics = augment_generic_bounds(&input.generics);
    let ctx = GenContext::build(&input, meta, generics);
    let facts = plan_facts(&ctx);
    let sink: std::collections::HashSet<usize> = facts.iter().map(|(i, _)| *i).collect();
    // 不可下沉的有体 fn（wrapper 上下文 / 不可函数化）原样留在声明层
    let mut stay = Vec::new();
    let mut unknown = false;
    for (i, f) in input.fns.iter().enumerate() {
        if f.block.is_none() || sink.contains(&i) {
            continue;
        }
        match (attr_str(&f.attrs, "name"), attr_str(&f.attrs, "descriptor")) {
            (Some(n), Some(d)) => stay.push((n, d)),
            _ => unknown = true,
        }
    }
    let mut out = Vec::new();
    for (i, m) in facts {
        let f = &input.fns[i];
        let Some(block) = &f.block else { continue };
        let open = block.brace_token.span.open().byte_range().start;
        let close = block.brace_token.span.close().byte_range().end;
        out.push(Elision { attr_at: item_start(f), body: open..close, fact: m.as_str() });
    }
    out.sort_by_key(|e| e.attr_at);
    let stay = if unknown {
        StayBodies::Unknown
    } else if stay.is_empty() {
        StayBodies::None
    } else {
        StayBodies::Fns(stay)
    };
    Ok((out, stay))
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

/// 模块级非泛型自由函数文本的跨层链接（生成器直接发射的自由函数，如根类方法体）：与宏拆层同一规则
/// 求 (链接符号, 声明层外部声明块文本)。实现层在原文本函数项前插入 `#[export_name = "<符号>"]`
/// 即可（保留原文本行布局），声明层放外部声明块
pub fn free_fn_link(binary_name: &str, item: &str) -> Result<(String, String), String> {
    let ts = TokenStream2::from_str(item).map_err(|e| format!("词法：{e}"));
    let split = ts.and_then(|ts| super::gen::layer::split_free_fn(binary_name, &ts).map_err(|e| e.to_string()));
    proc_macro2::extra::invalidate_current_thread_spans();
    let s = split?;
    let decl = s.decl;
    let block = quote::quote! {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        unsafe extern "Rust" { #decl }
    };
    Ok((s.sym, block.to_string()))
}

/// 声明模式展开文本，摘要常量值归一（对照用：剥体前后展开只应差这一处；`java_try!` 标签按词法嵌套
/// 深度编号，与体数无关，无需归一）
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
    Ok(out)
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

    /// 剥体计划同时报告「留在声明层的有体 fn」（按 java_method 名 + 描述符差分导入需求）：
    /// 可全下沉的类为 None；泛型类不剥体（有体留在声明层，方法调用按推断类型解析）→ Some(键集)
    #[test]
    fn decl_plan_flags_bodies_left_in_decl() {
        let (plan, stay) = decl_plan(CLASS).unwrap();
        assert!(matches!(stay, StayBodies::None));
        assert_eq!(plan.len(), 3);
        let generic = r#"
    #[binary_name = "a/L"]
    pub struct L<E> { pub n: i32 }
    impl L<E> {
        #[java_method(name = "m", descriptor = "()I", access = "public")]
        pub fn m(&self) -> Result<i32> { Ok(0) }
        #[java_method(name = "sib", descriptor = "()I", access = "public")]
        pub fn sib(&self) -> Result<i32>;
    }
"#;
        let (plan, stay) = decl_plan(generic).unwrap();
        assert!(plan.is_empty());
        assert_eq!(stay, StayBodies::Fns(vec![("m".to_string(), "()I".to_string())]));
        // 有体 fn 缺 java_method 属性 → 无法按名差分 → Unknown（调用方保守保留全部匿名导入）
        let attr_m = "#[java_method(name = \"m\", descriptor = \"()I\", access = \"public\")]\n        ";
        let noattr = generic.replace(attr_m, "");
        let (_, stay) = decl_plan(&noattr).unwrap();
        assert!(matches!(stay, StayBodies::Unknown));
    }

    /// S7 计划 §9.9 K1：wrapper 只持对象句柄，不含本类视图指针；分派经 `__vt()` 按深度现取
    #[test]
    fn wrapper_holds_handle_only() {
        let out = super::super::expand(TokenStream2::from_str(CLASS).unwrap()).to_string();
        proc_macro2::extra::invalidate_current_thread_spans();
        assert!(out.contains("pub __r : __Handle ,"), "{out}");
        assert!(!out.contains("__Ref <") && !out.contains("__Ref ::"), "{out}");
        assert!(out.contains("self . __r . view :: < dyn B__VTable > (0u16)"), "{out}");
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

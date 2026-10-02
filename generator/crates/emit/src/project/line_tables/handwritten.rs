//! 手写方法的行表项（栈帧来源统一）：手写方法体在共置 `<stem>_impl.rs`，不经 `java_class!`
//! 宏，回溯帧的 `at` 落在该伴生文件。生成文件以三种形态登记手写方法——`// [meta]` 注释对
//! （声明跳过）、`#[java_native(..)]` 声明、`body = "handwritten"` 声明——本模块据此收集
//! (Rust 类型, fn 名) → Java 方法，再以 syn（span-locations）解析伴生文件的 `impl` 块，
//! 为每个登记方法的 fn 区间（含属性行：`#[jvm_native]` 插入的类初始化以属性位置为 span）
//! 写出行表项：native 方法记 [`LINE_NATIVE`]，其余记 [`LINE_UNKNOWN`]（手写体无 Java 行号）。

use proc_macro2::Span;
use syn::spanned::Spanned;

use super::{java_method_attr, FileLines, Method, NO_METHOD};

/// Java 行哨兵：native 方法帧（运行时 `StackTraceElement.lineNumber = -2`）
pub const LINE_NATIVE: u32 = u32::MAX;
/// Java 行哨兵：无行号的非 native 手写方法帧（运行时 `-1`）
pub const LINE_UNKNOWN: u32 = u32::MAX - 1;

/// 生成文件中登记为手写的方法
#[derive(Debug, PartialEq, Eq)]
pub struct HwMethod {
    /// 所在 `impl X` 块的 Rust 类型名
    pub self_ty: String,
    /// Rust fn 名
    pub fn_name: String,
    pub method: Method,
    pub native: bool,
}

/// 属性行是否为手写方法的 `// [meta]` 注释（声明跳过，体在伴生文件）
pub fn is_meta_comment(line: &str) -> bool {
    line.trim_start().starts_with("// [meta]")
}

/// 是否为手写登记形态的方法属性行
fn is_handwritten_attr(line: &str) -> bool {
    is_meta_comment(line) || line.contains("#[java_native(") || line.contains("body = \"handwritten\"")
}

/// 签名行 `pub fn name(` / `fn name<T>(` 的 fn 名
fn fn_name(line: &str) -> Option<&str> {
    let at = line.find("fn ")?;
    let rest = &line[at + 3..];
    let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_'))?;
    (end > 0).then(|| &rest[..end])
}

/// 块内 `    impl X {` / `    impl<T> X<T> {` 的类型名
fn impl_type(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("    impl")?;
    let rest = if let Some(g) = rest.strip_prefix('<') {
        // 跳过泛型参数组（可嵌套）
        let mut depth = 1usize;
        let end = g.char_indices().find_map(|(i, c)| {
            match c {
                '<' => depth += 1,
                '>' => depth -= 1,
                _ => {}
            }
            (depth == 0).then_some(i)
        })?;
        &g[end + 1..]
    } else {
        rest
    };
    let rest = rest.strip_prefix(' ')?;
    let end = rest.find(|c: char| !(c.is_alphanumeric() || c == '_')).unwrap_or(rest.len());
    (end > 0).then(|| &rest[..end])
}

/// 收集生成文件中登记的手写方法（`class` / `source` 由块头部属性给出）
pub fn collect(text: &str) -> Vec<HwMethod> {
    let mut out = Vec::new();
    let (mut class, mut source, mut self_ty) = (String::new(), String::new(), String::new());
    let mut pending: Option<(Method, bool)> = None;
    for line in text.lines() {
        if let Some(v) = super::attr_value(line, "binary_name") {
            class = v.to_string();
            continue;
        }
        if let Some(v) = super::attr_value(line, "source") {
            source = v.to_string();
            continue;
        }
        if let Some(t) = impl_type(line) {
            self_ty = t.to_string();
            continue;
        }
        if let Some((native, name, descriptor)) = java_method_attr(line) {
            pending = is_handwritten_attr(line).then(|| {
                let method = Method::new(&class, name, descriptor, &source, &class);
                (method, native)
            });
            continue;
        }
        if let Some(f) = fn_name(line) {
            if let Some((method, native)) = pending.take() {
                out.push(HwMethod { self_ty: self_ty.clone(), fn_name: f.to_string(), method, native });
            }
        }
    }
    out
}

/// 伴生文件 `impl` 块中各 fn 的 (类型名, fn 名, 起始行, 结束行)
fn companion_fns(text: &str) -> Vec<(String, String, u32, u32)> {
    let Ok(file) = syn::parse_file(text) else { return Vec::new() };
    let line = |s: Span| s.start().line as u32;
    let mut out = Vec::new();
    for item in &file.items {
        let syn::Item::Impl(imp) = item else { continue };
        if imp.trait_.is_some() {
            continue;
        }
        let syn::Type::Path(p) = &*imp.self_ty else { continue };
        let Some(ty) = p.path.segments.last().map(|s| s.ident.to_string()) else { continue };
        for it in &imp.items {
            let syn::ImplItem::Fn(f) = it else { continue };
            let start = f.attrs.iter().map(|a| line(a.span())).chain([line(f.sig.span())]).min().unwrap_or(0);
            let end = f.block.brace_token.span.close().end().line as u32;
            out.push((ty.clone(), f.sig.ident.to_string(), start, end));
        }
    }
    out
}

/// 伴生文件的行表（无登记方法命中 → None）
pub fn companion_table(rel: &str, text: &str, hws: &[HwMethod]) -> Option<FileLines> {
    let mut out = FileLines { rel: rel.to_string(), ..Default::default() };
    let mut spans: Vec<(u32, u32, u32, u32)> = Vec::new();
    for (ty, name, start, end) in companion_fns(text) {
        let Some(hw) = hws.iter().find(|h| h.self_ty == ty && h.fn_name == name) else { continue };
        let idx = match out.methods.iter().position(|m| *m == hw.method) {
            Some(p) => p as u32,
            None => {
                out.methods.push(hw.method.clone());
                out.methods.len() as u32 - 1
            }
        };
        spans.push((start, end, idx, if hw.native { LINE_NATIVE } else { LINE_UNKNOWN }));
    }
    if spans.is_empty() {
        return None;
    }
    spans.sort();
    for (start, end, idx, java_line) in spans {
        out.rows.push((start, idx, java_line));
        out.rows.push((end + 1, NO_METHOD, 0));
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED: &str = "rava_macros::java_class! {
    #[binary_name       = \"p/A\"]
    #[source            = \"A.java\"]
    pub struct A;
    impl A {
        // [meta] #[java_native(name = \"n\", descriptor = \"()I\", access = \"public\", is_native    = true)]
        // [meta] pub fn n() -> Result<i32>;

        #[java_method(name = \"h\", descriptor = \"(I)V\", access = \"public\", body = \"handwritten\")]
        pub fn h_i(&self, a: i32) -> Result<()>;

        #[java_method(name = \"g\", descriptor = \"()V\")]
        pub fn g() -> Result<()> {
            x()?; // line 3
        }
    }
}";

    const COMPANION: &str = "use super::*;

impl A {
    /// 文档
    #[jvm_native]
    pub fn n() -> Result<i32> {
        Ok(1)
    }

    fn helper() {}

    pub fn h_i(&self, a: i32) -> Result<()> {
        Ok(())
    }
}
";

    #[test]
    fn collects_registered_handwritten_methods() {
        let hws = collect(GENERATED);
        let names: Vec<(&str, &str, bool)> =
            hws.iter().map(|h| (h.self_ty.as_str(), h.fn_name.as_str(), h.native)).collect();
        assert_eq!(names, vec![("A", "n", true), ("A", "h_i", false)]);
        assert_eq!(hws[1].method, Method::new("p/A", "h", "(I)V", "A.java", "p/A"));
    }

    #[test]
    fn companion_rows_cover_fn_spans() {
        let hws = collect(GENERATED);
        let t = companion_table("java_runtime/src/p/a_impl.rs", COMPANION, &hws).expect("命中");
        assert_eq!(t.methods.len(), 2);
        assert_eq!(t.rows, vec![(4, 0, LINE_NATIVE), (9, NO_METHOD, 0), (12, 1, LINE_UNKNOWN), (15, NO_METHOD, 0)]);
    }

    #[test]
    fn impl_type_skips_generics() {
        assert_eq!(impl_type("    impl<T: Clone> B<T> {"), Some("B"));
        assert_eq!(impl_type("    impl A {"), Some("A"));
        assert_eq!(impl_type("        impl A {"), None);
    }
}

//! 手写层「宏展开出自由函数」的守卫。
//!
//! 闭包分析器按 syn 扫描手写文件的 `fn` 项建立模块函数单元（rt-fn 节点），跨文件调用
//! `crate::<mod>::<fn>()` 靠它连边、传播流值。`macro_rules!` 在条目位置展开出的自由函数对扫描器不可见：
//! 调用边静默丢失，被调方的值流断开，闭包结果随之失真，并可能随哈希种子变化（2026-10-04
//! `meta.rs` 的 `merged_table!` 访问器即此回归）。故手写层的模块级函数一律写成普通 `fn`；
//! 条目位置的宏调用只允许展开出 trait impl、static 等非自由函数条目。

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use proc_macro2::{Delimiter, TokenStream, TokenTree};

    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }

    /// 展开体顶层（不进入花括号块，即不进 impl / fn 体）是否出现 `fn`；`$( ... )*` 重复组与属性组照常深入
    fn expands_free_fn(body: TokenStream) -> bool {
        body.into_iter().any(|t| match t {
            TokenTree::Ident(i) => i == "fn",
            TokenTree::Group(g) if g.delimiter() != Delimiter::Brace => expands_free_fn(g.stream()),
            _ => false,
        })
    }

    /// `macro_rules!` 体（`(规则) => {展开};` 序列）中是否有规则展开出自由函数
    fn rules_define_free_fn(rules: TokenStream) -> bool {
        let toks: Vec<TokenTree> = rules.into_iter().collect();
        toks.windows(3).any(|w| match (&w[0], &w[1], &w[2]) {
            (TokenTree::Punct(a), TokenTree::Punct(b), TokenTree::Group(g)) if a.as_char() == '=' && b.as_char() == '>' => {
                expands_free_fn(g.stream())
            }
            _ => false,
        })
    }

    /// 收集宏定义（名 → 是否展开自由函数）与条目位置的宏调用（名，位置）
    fn collect(items: &[syn::Item], rel: &str, defs: &mut BTreeMap<String, bool>, calls: &mut Vec<(String, String)>) {
        for item in items {
            match item {
                syn::Item::Macro(m) => {
                    let name = m.mac.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default();
                    if name == "macro_rules" {
                        if let Some(id) = &m.ident {
                            *defs.entry(id.to_string()).or_default() |= rules_define_free_fn(m.mac.tokens.clone());
                        }
                    } else {
                        calls.push((name, rel.to_string()));
                    }
                }
                syn::Item::Mod(m) => {
                    if let Some((_, inner)) = &m.content {
                        collect(inner, rel, defs, calls);
                    }
                }
                _ => {}
            }
        }
    }

    #[test]
    fn detects_macro_generated_free_fn() {
        let src = "macro_rules! acc { ($v:vis fn $n:ident) => { $v fn $n() -> i32 { 0 } }; }\n\
                   macro_rules! tr { ($t:ty) => { impl Foo for $t { fn f() {} } }; }\n\
                   macro_rules! rep { ($($t:ty),*) => {$( impl Foo for $t { fn f() {} } )*}; }";
        let file = syn::parse_file(src).unwrap();
        let (mut defs, mut calls) = (BTreeMap::new(), Vec::new());
        collect(&file.items, "x.rs", &mut defs, &mut calls);
        assert_eq!(defs.get("acc"), Some(&true));
        assert_eq!(defs.get("tr"), Some(&false));
        assert_eq!(defs.get("rep"), Some(&false));
    }

    #[test]
    fn runtime_module_fns_are_not_macro_generated() {
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime/src");
        let mut files = Vec::new();
        walk(&src, &mut files);
        assert!(files.len() > 10, "未找到手写层源文件：{}", src.display());
        let (mut defs, mut calls) = (BTreeMap::new(), Vec::new());
        for f in &files {
            let rel = f.strip_prefix(&src).unwrap().to_string_lossy().replace('\\', "/");
            let text = std::fs::read_to_string(f).unwrap();
            let file = syn::parse_file(&text).unwrap_or_else(|e| panic!("{rel} 解析失败：{e}"));
            collect(&file.items, &rel, &mut defs, &mut calls);
        }
        let bad: Vec<String> = calls
            .iter()
            .filter(|(name, _)| defs.get(name).copied().unwrap_or(false))
            .map(|(name, rel)| format!("{rel}: {name}!"))
            .collect();
        assert!(
            bad.is_empty(),
            "手写层条目位置的宏调用展开出自由函数，闭包分析器扫描不到（跨文件调用边丢失），改写为普通 fn：\n{}",
            bad.join("\n")
        );
    }
}

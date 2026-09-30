//! 共置手写文件扫描（`codegen/emitter/method_gen._scan_impl_files` 的移植）。
//!
//! 遍历 `src/**/*_impl.rs` / `*_ext.rs`（目录与文件名排序，与 `os.walk` 同序：先本目录文件、
//! 后子目录），按文本形态提取：
//! - `pub fn` 名（手写提供的方法，发射层跳过对应翻译）；
//! - 伴生核心 `fn core_<名>`（名 → (核心 fn 名, 返回类型文本)）；
//! - `impl <X>__VTable for T { … }` 块内的接口方法签名（登记到接口 X 的条目）。
//!
//! 口径是**文本级**（与 Python 正则逐项一致），不用语法树：发射层按原文拼接签名，
//! 语法树重打印会改变空白与写法。含生成标记的文件跳过。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use closure::handwritten::{GENERATED_MARK, MODULE_SUFFIXES};
use ty::Registry;

use crate::scan_text::{balanced, is_word, match_fn_head, skip_ws, starts_word, strip_self};

/// ObjectVTable 协议成员（不是 Java 契约方法）
const PROTO_FNS: [&str; 7] = ["as_any", "__class_name", "__obj_str", "__interface", "__to_string", "__hash_code", "__equals"];
const VTABLE_SUFFIX: &str = "__VTable";
const CORE_PREFIX: &str = "core_";
/// 同目录文件头中 binary_name 的读取长度（字符）
const HEAD_CHARS: usize = 2000;

/// 一个类的手写事实
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HwEntry {
    /// `pub fn` 名
    pub methods: BTreeSet<String>,
    /// rust 方法名 → (核心 fn 名, 返回类型文本)
    pub method_cores: BTreeMap<String, (String, String)>,
    /// 接口伴生实现的方法：rust fn 名 → (参数文本（不含接收者）, 返回类型文本)
    pub iface_method_sigs: BTreeMap<String, (String, String)>,
}

/// 类 binary name → 手写事实
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HandwrittenMap {
    pub classes: BTreeMap<String, HwEntry>,
}

/// `(?<=[a-z0-9])(?=[A-Z])` 与 `(?<=[A-Z])(?=[A-Z][a-z])` 处插 `_` 后转小写（不加关键字后缀）
fn camel_to_snake(name: &str) -> String {
    let c: Vec<char> = name.chars().collect();
    let mut s1 = Vec::with_capacity(c.len() + 4);
    for (i, &ch) in c.iter().enumerate() {
        if i > 0 && ch.is_ascii_uppercase() && (c[i - 1].is_ascii_lowercase() || c[i - 1].is_ascii_digit()) {
            s1.push('_');
        }
        s1.push(ch);
    }
    let mut out = String::with_capacity(s1.len() + 4);
    for (i, &ch) in s1.iter().enumerate() {
        let next_lower = s1.get(i + 1).is_some_and(char::is_ascii_lowercase);
        if i > 0 && s1[i - 1].is_ascii_uppercase() && ch.is_ascii_uppercase() && next_lower {
            out.push('_');
        }
        out.push(ch);
    }
    out.to_lowercase()
}

/// Python `str.capitalize`：首字符大写、其余小写
fn capitalize(w: &str) -> String {
    let mut it = w.chars();
    match it.next() {
        Some(f) => f.to_uppercase().chain(it.flat_map(char::to_lowercase)).collect(),
        None => String::new(),
    }
}

/// registry 插入序上的 snake 路径 → binary name（setdefault）
fn snake_index(reg: &Registry) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    for ci in reg.iter_insertion() {
        let bn = ci.name();
        let (pkg, cls) = match bn.rsplit_once('/') {
            Some((p, c)) => (Some(p), c),
            None => (None, bn),
        };
        let snake = cls.split('$').map(camel_to_snake).collect::<Vec<_>>().join("_");
        let alt = snake.replace("__", "_");
        for sv in [snake, alt] {
            let key = match pkg {
                Some(p) => format!("{p}/{sv}"),
                None => sv,
            };
            out.entry(key).or_insert_with(|| bn.to_string());
        }
    }
    out
}

/// `binary_name\s*=\s*"([^"]+)"`（文件头 2000 字符内）
fn head_binary_name(path: &Path) -> Option<String> {
    let text = std::fs::read_to_string(path).ok()?;
    let head: String = text.chars().take(HEAD_CHARS).collect();
    let mut from = 0;
    while let Some(p) = head[from..].find("binary_name") {
        let at = from + p + "binary_name".len();
        from = at;
        let i = skip_ws(&head, at);
        if !head[i..].starts_with('=') {
            continue;
        }
        let i = skip_ws(&head, i + 1);
        if let Some(rest) = head[i..].strip_prefix('"') {
            if let Some(end) = rest.find('"').filter(|e| *e > 0) {
                return Some(rest[..end].to_string());
            }
        }
    }
    None
}

/// `\bpub fn\s+(\w+)\s*[(<]`
fn pub_fns(content: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for (p, _) in content.match_indices("pub fn") {
        if !starts_word(content, p) {
            continue;
        }
        let after = p + "pub fn".len();
        let i = skip_ws(content, after);
        if i == after {
            continue;
        }
        let end = i + content[i..].find(|c: char| !is_word(c)).unwrap_or(content.len() - i);
        if end == i {
            continue;
        }
        let j = skip_ws(content, end);
        if content[j..].starts_with(['(', '<']) {
            out.insert(content[i..end].to_string());
        }
    }
    out
}

/// `\bfn\s+(core_\w+)…\(` → 名 → (核心名, 返回类型)
fn method_cores(content: &str) -> Vec<(String, (String, String))> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some((name, open)) = match_fn_head(content, from) {
        from = open;
        if let Some(core) = name.strip_prefix(CORE_PREFIX) {
            let (_, ret) = fn_sig(content, open);
            out.push((core.to_string(), (name.to_string(), ret)));
        }
    }
    out
}

/// 从 `(` 之后提取 (参数文本（剥接收者）, 返回类型文本)
fn fn_sig(text: &str, open: usize) -> (String, String) {
    let close = balanced(text, open, '(', ')');
    let params = &text[open..close.saturating_sub(1).max(open)];
    let params = strip_self(params).trim().to_string();
    let i = skip_ws(text, close);
    // `\s*->\s*([^{;]+)`：箭头后到 `{` / `;` 之前的文本非空即匹配（纯空白时 strip 为空串）
    let ret = match text[i..].strip_prefix("->") {
        Some(rest) => {
            let r = &rest[..rest.find(['{', ';']).unwrap_or(rest.len())];
            if r.is_empty() {
                "()".to_string()
            } else {
                r.trim().to_string()
            }
        }
        None => "()".to_string(),
    };
    (params, ret)
}

/// `\bimpl\s+(\w+)__VTable\s+for\s+[^{]*\{` 块内的方法签名，按 trait 标识分组
fn vtable_sigs(content: &str) -> BTreeMap<String, BTreeMap<String, (String, String)>> {
    let mut out: BTreeMap<String, BTreeMap<String, (String, String)>> = BTreeMap::new();
    let mut from = 0;
    while let Some(p) = content[from..].find("impl").map(|p| p + from) {
        from = p + "impl".len();
        let Some((ident, body_start)) = vtable_head(content, p) else { continue };
        let close = balanced(content, body_start, '{', '}');
        let body = &content[body_start..close.saturating_sub(1).max(body_start)];
        let mut bf = 0;
        while let Some((name, open)) = match_fn_head(body, bf) {
            bf = open;
            if name.starts_with('_') || PROTO_FNS.contains(&name) {
                continue;
            }
            let sig = fn_sig(body, open);
            out.entry(ident.to_string()).or_default().insert(name.to_string(), sig);
        }
        from = body_start;
    }
    out
}

/// 在 `impl` 位置匹配 vtable 头，返回 (trait 标识, `{` 之后的位置)
fn vtable_head(content: &str, p: usize) -> Option<(&str, usize)> {
    if !starts_word(content, p) {
        return None;
    }
    let after = p + "impl".len();
    let i = skip_ws(content, after);
    if i == after {
        return None;
    }
    let end = i + content[i..].find(|c: char| !is_word(c)).unwrap_or(content.len() - i);
    let ident = content[i..end].strip_suffix(VTABLE_SUFFIX).filter(|s| !s.is_empty())?;
    let j = skip_ws(content, end);
    if j == end || !content[j..].starts_with("for") {
        return None;
    }
    let k = skip_ws(content, j + "for".len());
    if k == j + "for".len() {
        return None;
    }
    let brace = k + content[k..].find('{')?;
    Some((ident, brace + 1))
}

/// os.walk 同序收集手写共置文件
fn walk(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            dirs.push(p);
        } else {
            files.push(p);
        }
    }
    files.sort();
    dirs.sort();
    out.extend(files);
    for d in dirs {
        walk(&d, out);
    }
}

impl HandwrittenMap {
    /// 扫描 `src_dir`（= runtime/java_runtime/src）；`reg` 用于文件 → 类与 vtable 标识 → 接口的解析
    pub fn scan(src_dir: &Path, reg: &Registry) -> HandwrittenMap {
        let mut map = HandwrittenMap::default();
        let snake = snake_index(reg);
        let mut files = Vec::new();
        walk(src_dir, &mut files);
        for path in files {
            let Some(fname) = path.file_name().and_then(|f| f.to_str()) else { continue };
            let Some(base) = MODULE_SUFFIXES.iter().find_map(|s| fname.strip_suffix(&format!("{s}.rs"))) else {
                continue;
            };
            let Ok(rel) = path.parent().unwrap_or(src_dir).strip_prefix(src_dir) else { continue };
            let pkg: Vec<String> = rel.iter().map(|s| s.to_string_lossy().into_owned()).collect();
            let with_pkg = |leaf: String| if pkg.is_empty() { leaf } else { format!("{}/{leaf}", pkg.join("/")) };
            let base_stem = with_pkg(base.to_string());
            let class_binary = match snake.get(&base_stem) {
                Some(b) => b.clone(),
                None => head_binary_name(&path.with_file_name(format!("{base}.rs")))
                    .unwrap_or_else(|| with_pkg(base.split('_').map(capitalize).collect())),
            };
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            if content.contains(GENERATED_MARK) {
                continue;
            }
            map.absorb(&content, &class_binary, &pkg.join("/"), reg);
        }
        map
    }

    fn absorb(&mut self, content: &str, class_binary: &str, file_pkg: &str, reg: &Registry) {
        let names = pub_fns(content);
        if !names.is_empty() {
            self.classes.entry(class_binary.to_string()).or_default().methods.extend(names);
        }
        for (name, core) in method_cores(content) {
            self.classes.entry(class_binary.to_string()).or_default().method_cores.insert(name, core);
        }
        for (ident, methods) in vtable_sigs(content) {
            let cands: Vec<&str> = reg
                .iter()
                .map(|ci| ci.name())
                .filter(|bn| bn.rsplit_once('/').is_some_and(|(_, s)| s.replace('$', "_") == ident))
                .collect();
            let same_pkg = cands.iter().find(|c| c.rsplit_once('/').is_some_and(|(p, _)| p == file_pkg));
            let target = same_pkg.or(cands.first()).copied().unwrap_or(class_binary);
            if reg.get(target).is_some_and(|ci| !ci.is_interface()) {
                continue;
            }
            self.classes.entry(target.to_string()).or_default().iface_method_sigs.extend(methods);
        }
    }

    pub fn get(&self, cls: &str) -> Option<&HwEntry> {
        self.classes.get(cls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_rules() {
        assert_eq!(camel_to_snake("HashMap"), "hash_map");
        assert_eq!(camel_to_snake("URLDecoder"), "url_decoder");
        assert_eq!(camel_to_snake("aBCd"), "a_b_cd");
        assert_eq!(capitalize("hash"), "Hash");
    }

    #[test]
    fn scans_text_forms() {
        let src = "impl X {\n  #[inline] pub fn foo(&self) {}\n  pub fn bar<T>(x: T) {}\n  fn core_len(&self) -> i64 { 0 }\n}\n\
                   impl Iter__VTable for X {\n  fn next(&self,\n a: i32) -> Object { todo() }\n  fn as_any(&self) {}\n  fn _h(&self) {}\n}\n";
        assert_eq!(pub_fns(src), ["bar", "foo"].iter().map(|s| s.to_string()).collect());
        assert_eq!(method_cores(src), vec![("len".to_string(), ("core_len".to_string(), "i64".to_string()))]);
        let vt = vtable_sigs(src);
        assert_eq!(vt["Iter"]["next"], ("a: i32".to_string(), "Object".to_string()));
        assert_eq!(vt["Iter"].len(), 1);
    }
}

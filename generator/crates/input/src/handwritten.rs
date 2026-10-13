//! 共置手写文件扫描。
//!
//! 遍历 `src/**/*_impl.rs` / `*_ext.rs`（目录与文件名排序，与 `os.walk` 同序：先本目录文件、
//! 后子目录；文本取手写单元合并文本，私有辅助目录内的文件并入宿主，见 `closure::handwritten::layout`），
//! 按文本形态提取：
//! - `pub fn` 名（手写提供的方法，发射层跳过对应翻译）；
//! - 伴生核心 `fn core_<名>`（名 → (核心 fn 名, 返回类型文本)）；
//! - `impl <X>__VTable for T { … }` 块内的接口方法签名（登记到接口 X 的条目）。
//!
//! 口径是**文本级**（与 Python 正则逐项一致），不用语法树：发射层按原文拼接签名，
//! 语法树重打印会改变空白与写法。含生成标记的文件跳过。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use closure::handwritten::{file_uses, layout, resolve_type, TypeRef, GENERATED_MARK, MODULE_SUFFIXES};
use ty::Registry;

use crate::par::par_map;
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
    /// 伴生签名文本里的类名（按所在手写文件的 use 表解析）：文本中的名字 → binary
    pub iface_sig_refs: BTreeMap<String, String>,
    /// 手写实现对象为该（非接口）类的 vtable trait 提供的 fn 名（`impl X__VTable for S`）：
    /// 这些槽由手写对象实现，发射层不得按分派结果裁剪
    pub class_vtable_fns: BTreeSet<String>,
    /// `impl __XHw{N} for X` 扩展 trait 实现块（§9.9 K4-2a，`scripts/diag/k4_2a_rewrite_hw_impls.py`
    /// 改写后的形态；序号 = 块序）。发射层据此镜像 trait 声明，调用点按名登记导入。
    pub hw_blocks: Vec<HwBlock>,
    /// hw 块签名 / 泛型界里出现、经伴生文件 use 表解析的名字 → 完整路径段
    /// （声明文件按此补导入——镜像出的 trait 声明须在本文件作用域可解析）
    pub hw_sig_uses: BTreeMap<String, Vec<String>>,
}

/// 一个手写扩展 trait 实现块的机械镜像（声明侧发射用）
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HwBlock {
    /// impl 头泛型组原文（`<T: Clone + …>`；空 = 非泛型）
    pub generics: String,
    /// 自类型实参原文（`<T, K, S>`；空 = 非泛型）——supertrait（`X__Methods`）按它应用
    pub self_args: String,
    /// impl 头 where 子句（规范化空白；空 = 无）——trait 声明侧照搬
    pub where_c: String,
    /// 条目（发射序）
    pub items: Vec<HwItem>,
}

/// 块内一个条目
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HwItem {
    pub name: String,
    /// 属性行原样（cfg 等须在声明侧保持一致；文档注释不采）
    pub attrs: Vec<String>,
    /// 声明签名（无可见性、无体；`fn f(&self, x: i32) -> R` 或 `const N: T`）
    pub sig: String,
    /// 改写前是 `pub`（改写脚本以 `#[cfg_attr(any(), rava_hw_pub)]` 原位标记）：这类条目按
    /// 「手写提供的成员 / vm 钩子」计；原 `pub(crate)` / `pub(super)` 的私有辅助不计
    /// （CollectorsDemo 种子漂移的根因——S7 计划 §9.9 K4-2a）
    pub is_pub: bool,
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
/// vtable 标识（简单名，`$` → `_`）→ 有包名的 registry 类（registry 序）
fn vtable_ident_index(reg: &Registry) -> BTreeMap<String, Vec<&str>> {
    let mut out: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for bn in reg.iter().map(|ci| ci.name()) {
        if let Some((_, s)) = bn.rsplit_once('/') {
            out.entry(s.replace('$', "_")).or_default().push(bn);
        }
    }
    out
}

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

// ── §9.9 K4-2a：`impl __XHw{N} for X` 块的机械镜像扫描 ─────────────────────────

/// 平衡尖括号组：`head[i] == '<'` 起，返回组内原文（含尖括号）与结束位置；
/// 泛型约束可嵌套（`<T: From<Object>>`）
fn angle_group(head: &str, mut i: usize) -> Option<(String, usize)> {
    if head[i..].starts_with('<') {
        let mut depth = 0usize;
        let start = i;
        let b = head.as_bytes();
        while i < b.len() {
            match b[i] {
                b'<' => depth += 1,
                b'>' => {
                    depth -= 1;
                    if depth == 0 {
                        return Some((head[start..=i].to_string(), i + 1));
                    }
                }
                b';' => break,
                _ => {}
            }
            i += 1;
        }
    }
    None
}

fn skip_sp(head: &str, mut i: usize) -> usize {
    while head[i..].chars().next().is_some_and(char::is_whitespace) {
        i += head[i..].chars().next().map_or(1, char::len_utf8);
    }
    i
}

fn ident_at(head: &str, i: usize) -> Option<(&str, usize)> {
    let end = i + head[i..].find(|c: char| !is_word(c)).unwrap_or(head.len() - i);
    (end > i).then(|| (&head[i..end], end))
}

/// `impl<GEN> __ClsHw[N] for Cls<APP> [where …] {` 的头解析（改写脚本的产出形态）：
/// (泛型组, 自类型实参, where 子句（首尾带空格；空 = 无))
fn hw_head(head: &str) -> Option<(String, String, String)> {
    let mut i = skip_sp(head, "impl".len());
    let generics = match angle_group(head, i) {
        Some((g, j)) => {
            i = skip_sp(head, j);
            g
        }
        None => String::new(),
    };
    let (trait_name, j) = ident_at(head, i)?;
    let stem = trait_name.strip_prefix("__")?;
    // 尾序号（`__XHw2`）剥去后须以 `Hw` 收尾
    let base = stem.trim_end_matches(|c: char| c.is_ascii_digit());
    let cls = base.strip_suffix("Hw").filter(|s| !s.is_empty())?;
    if base.len() != cls.len() + "Hw".len() {
        return None;
    }
    i = skip_sp(head, j);
    // trait 名后的自类型实参组（`impl<T: …> __XHw<T> for X<T>`——泛型 trait 的 impl 应用形参）
    if head[i..].starts_with('<') {
        let (_, j2) = angle_group(head, i)?;
        i = skip_sp(head, j2);
    }
    if !head[i..].starts_with("for") {
        return None;
    }
    i = skip_sp(head, i + "for".len());
    // 自类型（可路径限定，取末段核对）
    let (self_ty, j) = ident_at(head, i)?;
    let last = self_ty.rsplit("::").next().unwrap_or(self_ty);
    if last != cls {
        return None;
    }
    i = skip_sp(head, j);
    let self_args = match angle_group(head, i) {
        Some((a, j)) => {
            i = skip_sp(head, j);
            a
        }
        None => String::new(),
    };
    if !head.ends_with('{') {
        return None;
    }
    // 尾随 where 子句（trait 声明侧照搬）
    let tail = &head[i..].trim_end()[..head[i..].trim_end().len().saturating_sub(1)];
    let where_c = match tail.trim().strip_prefix("where ") {
        Some(w) => format!(" where {} ", w.split_whitespace().collect::<Vec<_>>().join(" ")),
        None if tail.trim().is_empty() => String::new(),
        None => return None,
    };
    Some((generics, self_args, where_c))
}

/// 形参文本规范化（trait 无体声明不接受模式）：接收者 `mut self` → `self`（引用接收者
/// `&mut self` 保持）、`mut x: T` → `x: T`、复合模式 → `_`
fn decl_params(params: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in params.chars() {
        match c {
            '(' | '[' | '<' => {
                depth += 1;
                cur.push(c)
            }
            ')' | ']' | '>' => {
                depth -= 1;
                cur.push(c)
            }
            ',' if depth == 0 => {
                out.push(cur.clone());
                cur.clear();
            }
            _ => cur.push(c),
        }
    }
    out.push(cur);
    out.iter()
        .map(|p| {
            let t = p.trim();
            if t == "mut self" {
                return "self".to_string();
            }
            match t.split_once(':') {
                Some((pat, ty)) => {
                    let pat = pat.trim().strip_prefix("mut ").unwrap_or(pat.trim());
                    let name = if !pat.is_empty() && pat.chars().all(|c| is_word(c)) { pat } else { "_" };
                    format!("{name}: {}", ty.trim())
                }
                None => t.to_string(),
            }
        })
        .collect::<Vec<_>>()
        .join(", ")
}

/// `fn 名(参数) [-> 返回]` 的声明签名：参数规范化（无体声明不接受模式），其余原样
fn decl_sig(sig: &str) -> String {
    let Some(open) = sig.find('(') else { return sig.trim().to_string() };
    // `balanced` 的 start 语义 = 开括号之后（初始深度 1 = 已开一层）
    let close = balanced(sig, open + 1, '(', ')');
    let params = decl_params(&sig[open + 1..close.saturating_sub(1).max(open + 1)]);
    let head = sig[..open].trim().to_string();
    let tail = sig[close..].trim().to_string();
    format!("{head}({params}) {tail}")
}

/// 单个手写文件的 `impl __XHw{N} for X` 块（发射序；行级扫描——改写脚本保证第 0 列头、
/// 条目缩进 4 列，属性单行）
pub fn hw_impl_blocks(content: &str) -> Vec<HwBlock> {
    let lines: Vec<&str> = content.lines().collect();
    let mut out = Vec::new();
    let mut i = 0usize;
    while i < lines.len() {
        let l = lines[i].trim_end();
        if !l.starts_with("impl") || !starts_word(l, 0) {
            i += 1;
            continue;
        }
        // 头累积到 `{` 结尾的行（where 跨行形态）
        let mut head = l.to_string();
        let mut j = i;
        while !head.ends_with('{') && j + 1 < lines.len() {
            j += 1;
            head.push(' ');
            head.push_str(lines[j].trim());
        }
        let Some((generics, self_args, where_c)) = hw_head(&head) else {
            // 其余 impl（trait impl / 本地类型固有 impl）：跳过整个块
            let mut depth = head.chars().filter(|&c| c == '{').count() as i32
                - head.chars().filter(|&c| c == '}').count() as i32;
            let mut k = j + 1;
            while k < lines.len() && depth > 0 {
                depth += lines[k].matches('{').count() as i32 - lines[k].matches('}').count() as i32;
                k += 1;
            }
            i = k;
            continue;
        };
        // 体内顶层条目
        let mut items: Vec<HwItem> = Vec::new();
        let mut attrs: Vec<String> = Vec::new();
        let mut hw_pub = false;
        let mut depth = 1i32;
        let mut k = j + 1;
        while k < lines.len() && depth > 0 {
            let t = lines[k].trim();
            let line_depth = depth;
            depth += lines[k].matches('{').count() as i32 - lines[k].matches('}').count() as i32;
            k += 1;
            if line_depth != 1 || depth == 0 {
                continue;
            }
            if t.is_empty() || t.starts_with("//") {
                continue;
            }
            if t.starts_with('#') {
                if t.contains("rava_hw_pub") {
                    hw_pub = true;
                }
                attrs.push(t.to_string());
                continue;
            }
            // 条目头：`fn …{` / `const …;`（跨行签名累积；签名止于首个 `{` / `;`）
            if t.starts_with("fn ") || t.starts_with("unsafe fn ") || t.starts_with("const ") || t.starts_with("unsafe const ") {
                let is_const = t.starts_with("const") || t.starts_with("unsafe const");
                let mut sig = t.to_string();
                let mut m = k;
                let mut inner_braces = 0i32;
                let done = |s: &str| if is_const { s.contains(';') } else { s.contains('{') };
                while m < lines.len() && !done(&sig) {
                    sig.push(' ');
                    sig.push_str(lines[m].trim());
                    // 被吞的续行（含尾 `{`）同样计入块深——否则块边界提前收
                    inner_braces += lines[m].matches('{').count() as i32 - lines[m].matches('}').count() as i32;
                    m += 1;
                }
                let end = if is_const {
                    sig.find(';').unwrap_or(sig.len())
                } else {
                    sig.find('{').unwrap_or(sig.len())
                };
                let sig = sig[..end].to_string();
                k = m;
                depth += inner_braces;
                let name = sig
                    .split(|c: char| c == '(' || c == '<' || c == ':')
                    .next()
                    .unwrap_or("")
                    .split_whitespace()
                    .last()
                    .unwrap_or("")
                    .to_string();
                let decl = if is_const {
                    let cut = sig.find(" =").unwrap_or(sig.len());
                    sig[..cut].trim().to_string()
                } else {
                    decl_sig(&sig)
                };
                if !name.is_empty() {
                    items.push(HwItem { name, attrs: std::mem::take(&mut attrs), sig: decl, is_pub: hw_pub });
                }
            }
            attrs.clear();
            hw_pub = false;
        }
        out.push(HwBlock { generics, self_args, where_c, items });
        i = k;
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

/// 签名文本里独立出现（不在 `::` 之后）、大写开头的标识符
pub fn type_words(text: &str) -> impl Iterator<Item = &str> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < text.len() {
        let rest = &text[i..];
        let Some(c) = rest.chars().next() else { break };
        if !is_word(c) {
            i += c.len_utf8();
            continue;
        }
        let len = rest.find(|c: char| !is_word(c)).unwrap_or(rest.len());
        let w = &rest[..len];
        if w.starts_with(|c: char| c.is_ascii_uppercase()) && !text[..i].ends_with("::") {
            out.push(w);
        }
        i += len;
    }
    out.into_iter()
}

/// 泛型界词（std prelude 界，不需要导入）
const BOUND_WORDS: [&str; 8] = ["Clone", "Default", "Sized", "Send", "Sync", "Into", "From", "Ord"];

/// 签名 / 泛型界作用域内出现的词（`type_words` 已跳 `::` 路径尾段——全限定界可解析；
/// std 界名不需导入）
fn type_words_scope(chars: impl Iterator<Item = char>) -> Vec<String> {
    let text: String = chars.collect();
    type_words(&text).filter(|w| !BOUND_WORDS.contains(w)).map(str::to_string).collect()
}

/// 签名里出现、经文件 use 表导入的类型名 → 完整路径段
fn sig_uses(content: &str, vtables: &BTreeMap<String, BTreeMap<String, (String, String)>>) -> BTreeMap<String, Vec<String>> {
    if vtables.is_empty() {
        return BTreeMap::new();
    }
    let uses = file_uses(content);
    let mut out = BTreeMap::new();
    for (params, ret) in vtables.values().flat_map(|m| m.values()) {
        for w in type_words(params).chain(type_words(ret)) {
            if let Some(segs) = uses.get(w) {
                out.insert(w.to_string(), segs.clone());
            }
        }
    }
    out
}

fn is_word_str(s: &str) -> bool {
    s.chars().all(is_word)
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
    // 路径限定的 trait（`crate::a::b::X__VTable`）取末段
    let end = i + content[i..].find(|c: char| !(is_word(c) || c == ':')).unwrap_or(content.len() - i);
    let path = &content[i..end];
    let last = path.rsplit("::").next().unwrap_or(path);
    let ident = last.strip_suffix(VTABLE_SUFFIX).filter(|s| !s.is_empty() && is_word_str(s))?;
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

/// 单个手写文件的文本提取结果（与注册表无关的部分）
struct FileFacts {
    class_binary: String,
    file_pkg: String,
    names: BTreeSet<String>,
    cores: Vec<(String, (String, String))>,
    vtables: BTreeMap<String, BTreeMap<String, (String, String)>>,
    /// 签名里出现、经本文件 use 表导入的类型名 → 完整路径段
    sig_uses: BTreeMap<String, Vec<String>>,
    /// `impl __XHw{N} for X` 扩展 trait 实现块（§9.9 K4-2a）
    hw_blocks: Vec<HwBlock>,
    /// hw 块签名 / 泛型界里的词 → 本文件 use 表路径
    hw_sig_uses: BTreeMap<String, Vec<String>>,
}

/// 读取并提取一个手写文件；非手写模块文件、不可读或含生成标记的文件返回 `None`
fn file_facts(src_dir: &Path, path: &Path, snake: &BTreeMap<String, String>) -> Option<FileFacts> {
    // 私有辅助目录内的文件并入宿主单元（随宿主读取），自身不单独提取
    if layout::helper_root(src_dir, path).is_some() {
        return None;
    }
    let fname = path.file_name().and_then(|f| f.to_str())?;
    let base = MODULE_SUFFIXES.iter().find_map(|s| fname.strip_suffix(&format!("{s}.rs")))?;
    let rel = path.parent().unwrap_or(src_dir).strip_prefix(src_dir).ok()?;
    let pkg: Vec<String> = rel.iter().map(|s| s.to_string_lossy().into_owned()).collect();
    let with_pkg = |leaf: String| if pkg.is_empty() { leaf } else { format!("{}/{leaf}", pkg.join("/")) };
    let base_stem = with_pkg(base.to_string());
    let class_binary = match snake.get(&base_stem) {
        Some(b) => b.clone(),
        None => head_binary_name(&path.with_file_name(format!("{base}.rs")))
            .unwrap_or_else(|| with_pkg(base.split('_').map(capitalize).collect())),
    };
    let content = layout::read_unit(src_dir, path).ok()?;
    if content.contains(GENERATED_MARK) {
        return None;
    }
    let vtables = vtable_sigs(&content);
    let sig_uses = sig_uses(&content, &vtables);
    let hw_blocks = hw_impl_blocks(&content);
    let hw_sig_uses = if hw_blocks.is_empty() {
        BTreeMap::new()
    } else {
        let uses = file_uses(&content);
        let mut out = BTreeMap::new();
        for blk in &hw_blocks {
            let words = blk
                .generics
                .chars()
                .chain(blk.where_c.chars())
                .chain(blk.items.iter().flat_map(|i| i.sig.chars()));
            for w in type_words_scope(words) {
                if let Some(segs) = uses.get(w.as_str()) {
                    out.entry(w).or_insert_with(|| segs.clone());
                }
            }
        }
        out
    };
    Some(FileFacts {
        class_binary,
        file_pkg: pkg.join("/"),
        names: pub_fns(&content),
        cores: method_cores(&content),
        vtables,
        sig_uses,
        hw_blocks,
        hw_sig_uses,
    })
}

impl HandwrittenMap {
    /// 扫描 `src_dir`（= runtime/java_runtime/src）；`reg` 用于文件 → 类与 vtable 标识 → 接口的解析。
    ///
    /// 逐文件读取与文本提取并行（`jobs`：0 = 可用核数），归并按文件遍历序串行进行，
    /// 结果与并行度无关。
    pub fn scan(src_dir: &Path, reg: &Registry, jobs: usize) -> HandwrittenMap {
        let mut map = HandwrittenMap::default();
        let snake = snake_index(reg);
        let idents = vtable_ident_index(reg);
        let mut files = Vec::new();
        walk(src_dir, &mut files);
        let facts = par_map(jobs, &files, |path| file_facts(src_dir, path, &snake));
        for f in facts.into_iter().flatten() {
            map.absorb(f, reg, &idents);
        }
        map
    }

    fn absorb(&mut self, f: FileFacts, reg: &Registry, idents: &BTreeMap<String, Vec<&str>>) {
        let FileFacts { class_binary, file_pkg, names, cores, vtables, sig_uses, hw_blocks, hw_sig_uses } = f;
        let refs: BTreeMap<String, String> = sig_uses
            .into_iter()
            .filter_map(|(w, segs)| {
                let hit = resolve_type(&class_binary, &TypeRef(segs)).into_iter().find(|c| reg.contains(c))?;
                Some((w, hit))
            })
            .collect();
        if !names.is_empty() || !hw_blocks.is_empty() {
            let e = self.classes.entry(class_binary.clone()).or_default();
            e.methods.extend(names);
            // K4-2a：改写后的 hw 块条目并入 impl_methods——只计改写前 `pub` 的条目
            // （`rava_hw_pub` 标记），恢复改写前 `pub fn` 正则的名单口径；宏与判定层不变
            e.methods.extend(
                hw_blocks.iter().flat_map(|b| b.items.iter().filter(|i| i.is_pub).map(|i| i.name.clone())),
            );
            e.hw_sig_uses.extend(hw_sig_uses);
            e.hw_blocks.extend(hw_blocks);
        }
        for (name, core) in cores {
            self.classes.entry(class_binary.clone()).or_default().method_cores.insert(name, core);
        }
        for (ident, methods) in vtables {
            let cands: &[&str] = idents.get(&ident).map_or(&[], Vec::as_slice);
            let same_pkg = cands.iter().find(|c| c.rsplit_once('/').is_some_and(|(p, _)| *p == file_pkg));
            let target = same_pkg.or(cands.first()).copied().unwrap_or(&class_binary);
            if reg.get(target).is_some_and(|ci| !ci.is_interface()) {
                self.classes.entry(target.to_string()).or_default().class_vtable_fns.extend(methods.into_keys());
                continue;
            }
            let e = self.classes.entry(target.to_string()).or_default();
            for (params, ret) in methods.values() {
                for w in type_words(params).chain(type_words(ret)) {
                    if let Some(b) = refs.get(w) {
                        e.iface_sig_refs.insert(w.to_string(), b.clone());
                    }
                }
            }
            e.iface_method_sigs.extend(methods);
        }
    }

    pub fn get(&self, cls: &str) -> Option<&HwEntry> {
        self.classes.get(cls)
    }

    /// K4-2a 手写扩展 trait 名单（binary → 各块条目名集；发射层据此做调用点按名登记）
    pub fn hw_names(&self) -> ty::HwNames {
        ty::HwNames::build(self.classes.iter().map(|(b, e)| {
            (
                b.clone(),
                e.hw_blocks
                    .iter()
                    .map(|blk| blk.items.iter().filter(|i| i.is_pub).map(|i| i.name.clone()).collect())
                    .collect(),
            )
        }))
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


#[test]
fn scans_real_class_loader_impl() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime/src/java/lang/class_loader_impl.rs");
    let text = std::fs::read_to_string(&p).unwrap();
    let blocks = hw_impl_blocks(&text);
    for (i, b) in blocks.iter().enumerate() {
        let names: Vec<&str> = b.items.iter().map(|x| x.name.as_str()).collect();
        println!("block {}: gen={:?} args={:?} items={:?}", i + 1, b.generics, b.self_args, names);
    }
    assert!(blocks.len() >= 2);
    assert!(blocks[1].items.iter().any(|x| x.name == "defineClass1"), "{:?}", blocks[1].items.iter().map(|x| x.name.clone()).collect::<Vec<_>>());
}

#[test]
fn scans_real_phantom_reference() {
    let p = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_runtime/src/java/lang/ref/phantom_reference_impl.rs");
    let text = std::fs::read_to_string(&p).unwrap();
    let blocks = hw_impl_blocks(&text);
    for (i, b) in blocks.iter().enumerate() {
        println!("block {}: gen={:?} args={:?} where={:?} items={:?}", i + 1, b.generics, b.self_args, b.where_c, b.items.iter().map(|x| x.name.as_str()).collect::<Vec<_>>());
    }
    assert_eq!(blocks.len(), 1);
    assert!(blocks[0].generics.starts_with("<T:"), "{:?}", blocks[0].generics);
    assert_eq!(blocks[0].self_args, "<T>");
}

    /// K4-2a：`impl __XHw{N} for X` 块的机械镜像（泛型 / where / 条目签名 / cfg 属性 / 多行签名）
    #[test]
    fn scans_hw_impl_blocks() {
        let src = "use super::*;\n\
                   impl __StringHw for String {\n    fn intern(&self) -> Result<String> { todo() }\n    #[cfg(jdk_ge_25)]\n    fn fetch(&mut self,\n             n: i32) -> Result<Vec<u16>> { todo() }\n    fn by_val(mut self, mut x: i32, (a, b): (u8, u8)) -> Self { self }\n}\n\
                   impl<T: Clone + Default> __ReferenceHw2 for Reference<T> where T: 'static {\n    const K: i32 = 3;\n}\n\
                   impl __NotHw for Y { fn f() {} }\nimpl Plain { fn g() {} }\n";
        let blocks = hw_impl_blocks(src);
        assert_eq!(blocks.len(), 2, "{blocks:?}");
        assert_eq!(blocks[0].generics, "");
        assert_eq!(blocks[0].self_args, "");
        let names: Vec<&str> = blocks[0].items.iter().map(|i| i.name.as_str()).collect();
        assert_eq!(names, vec!["intern", "fetch", "by_val"]);
        assert!(!blocks[0].items.iter().any(|i| i.is_pub), "无标记条目不计 pub");
        assert_eq!(blocks[0].items[0].sig, "fn intern(&self) -> Result<String>");
        // 跨行签名 + cfg 属性随行
        assert_eq!(blocks[0].items[1].attrs, vec!["#[cfg(jdk_ge_25)]"]);
        assert_eq!(blocks[0].items[1].sig, "fn fetch(&mut self, n: i32) -> Result<Vec<u16>>");
        // 模式规范化：mut self → self、mut x → x、复合模式 → _
        assert_eq!(blocks[0].items[2].sig, "fn by_val(self, x: i32, _: (u8, u8)) -> Self");
        assert_eq!(blocks[1].generics, "<T: Clone + Default>");
        assert_eq!(blocks[1].self_args, "<T>");
        assert_eq!(blocks[1].where_c, " where T: 'static ");
        assert_eq!(blocks[1].items[0].sig, "const K: i32");
    }
}

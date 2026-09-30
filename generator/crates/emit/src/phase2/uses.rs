//! 继承成员 / 接口实现引用类型的 use 行推导（← `inherited_gen.class_use_path` /
//! `type_arg_uses` / `_imports_for`）。

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::OnceLock;

use regex::Regex;
use ty::ident::is_rust_keyword;
use ty::ClassInfo;

use super::sig::idents;
use super::Emissions;
use crate::ctx::EmitCtx;
use crate::emission::ClassEmission;
use crate::text::to_snake;

const JAVA_RUNTIME: &str = "java_runtime";

/// use 行导入的末段名：`^use\s+(.+)::([A-Za-z_][A-Za-z0-9_]*);\s*$` 的第 2 组。
/// `(.+)` 贪婪且末段不含 `:`，唯一候选是最后一个 `::`
fn use_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("use")?;
    let ws = rest.chars().next().filter(|c| c.is_whitespace())?;
    let p = line.rfind("::")?;
    if p < 3 + ws.len_utf8() + 1 {
        return None;
    }
    let tail = &line[p + 2..];
    let semi = tail.find(';')?;
    let (name, after) = (&tail[..semi], &tail[semi + 1..]);
    let mut cs = name.chars();
    let head_ok = cs.next().is_some_and(|c| c == '_' || c.is_ascii_alphabetic());
    (head_ok && cs.all(|c| c == '_' || c.is_ascii_alphanumeric()) && after.chars().all(char::is_whitespace)).then_some(name)
}

fn sig_class_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"L([A-Za-z_$][\w$]*(?:/[A-Za-z_$][\w$]*)+)[<;]").expect("静态正则"))
}

/// 文件 use 行导入的末段名（`_USE_RE` 的 group 2），全文扫描
pub fn imported_names(text: &str) -> BTreeSet<String> {
    text.split('\n').filter_map(|l| use_name(l).map(str::to_string)).collect()
}

/// 类文件头（`rava_macros::java_class!` 行之前）的 use 行索引：末段名 → 首条 use 行（trim）
pub struct UseIndex {
    header: String,
    map: BTreeMap<String, String>,
}

const CLASS_MACRO_LINE: &str = "rava_macros::java_class!";

/// 文本头部：首个以类宏起始的行之前的部分（无该行 → 全文）
fn header_of(text: &str) -> &str {
    if text.starts_with(CLASS_MACRO_LINE) {
        return "";
    }
    let marker = format!("\n{CLASS_MACRO_LINE}");
    text.find(&marker).map_or(text, |i| &text[..i + 1])
}

/// owner 的 use 行索引（缓存；头部文本变化——继承 use 插入位填充后——即重建）
fn use_index(ctx: &EmitCtx<'_>, owner: &ClassEmission) -> std::sync::Arc<UseIndex> {
    let header = header_of(&owner.text);
    if let Some(ix) = ctx.use_index.lock().unwrap_or_else(|e| e.into_inner()).get(&owner.binary_name).filter(|ix| ix.header == header) {
        return ix.clone();
    }
    let mut map = BTreeMap::new();
    for ln in header.split('\n') {
        if let Some(name) = use_name(ln) {
            map.entry(name.to_string()).or_insert_with(|| ln.trim().to_string());
        }
    }
    let ix = std::sync::Arc::new(UseIndex { header: header.to_string(), map });
    ctx.use_index.lock().unwrap_or_else(|e| e.into_inner()).insert(owner.binary_name.clone(), ix.clone());
    ix
}

/// 类在 Rust 中的完整引用路径（不含 `use` 与 `;`）。
///
/// - JDK 类（java_runtime，包 mod.rs 再导出）：`<crate 前缀>::java::lang::String`
/// - 默认包用户类：main.rs 只声明 mod，路径写到模块层 `crate::<snake>::<Short>`
/// - 标记了 crate 名的非 java_runtime 类：接收者同 crate 用 `crate::`，否则用其 crate 名
///
/// `ems` 为 None 时不查目标类的发射记录（Python 部分调用点不传 emissions，照搬）。
pub fn class_use_path(ctx: &EmitCtx<'_>, binary: &str, crate_prefix: &str, ems: Option<&Emissions>, recv_crate: &str) -> String {
    let target = ems.and_then(|e| e.get(binary)).map(|e| (e.crate_name.as_str(), e.crate_prefix.as_str()));
    use_path(ctx, binary, crate_prefix, target, recv_crate)
}

/// [`class_use_path`] 的核心：`target` 为目标类的 (crate 名, crate 前缀) 视图（None = 无发射记录）
pub fn use_path(ctx: &EmitCtx<'_>, binary: &str, crate_prefix: &str, target: Option<(&str, &str)>, recv_crate: &str) -> String {
    let short = ctx.short(binary);
    let segs: Vec<&str> = binary.split('/').collect();
    let pkg = segs[..segs.len() - 1]
        .iter()
        .map(|p| if is_rust_keyword(p) { format!("r#{p}") } else { (*p).to_string() })
        .collect::<Vec<_>>()
        .join("::");
    let lib_crate = target.map_or("", |t| t.0);
    if !lib_crate.is_empty() && lib_crate != JAVA_RUNTIME {
        let prefix = if lib_crate == recv_crate { "crate" } else { lib_crate };
        if pkg.is_empty() {
            return format!("{prefix}::{}::{short}", to_snake(binary));
        }
        return format!("{prefix}::{pkg}::{short}");
    }
    if pkg.is_empty() || target.is_some_and(|t| t.1 != "crate") {
        return format!("crate::{}::{short}", to_snake(binary));
    }
    format!("{crate_prefix}::{pkg}::{short}")
}

/// 接收者视角下类型实参引用的类：短名 → use 行（接收者及全部超类型的类级泛型签名中
/// 出现、且已生成的类）
pub fn type_arg_uses(
    ctx: &EmitCtx<'_>,
    recv_ci: &ClassInfo,
    ems: &Emissions,
    crate_prefix: &str,
    recv_crate: &str,
) -> BTreeMap<String, String> {
    let reg = ctx.ty.reg;
    let mut uses = BTreeMap::new();
    let mut queue: VecDeque<&ClassInfo> = VecDeque::from([recv_ci]);
    let mut seen = BTreeSet::new();
    while let Some(cur) = queue.pop_front() {
        if !seen.insert(cur.name().to_string()) {
            continue;
        }
        let sig = cur.generic_signature();
        for c in sig_class_re().captures_iter(sig) {
            let bin = &c[1];
            if !ems.contains_key(bin) {
                continue;
            }
            uses.entry(ctx.short(bin))
                .or_insert_with(|| format!("use {};", class_use_path(ctx, bin, crate_prefix, Some(ems), recv_crate)));
        }
        let sups = std::iter::once(cur.super_class()).chain(cur.interfaces().iter().map(String::as_str));
        for s in sups {
            if let Some(sci) = (!s.is_empty()).then(|| reg.get(s)).flatten() {
                queue.push_back(sci);
            }
        }
    }
    uses
}

/// 签名文本引用类型的 use 行：沿用祖先文件里的精确 use（跨 crate 时按目标 crate 重定向）；
/// 代入的类型实参不在祖先文件中，按 `arg_uses` 解析。`already` 为接收者已导入名（随之更新）
pub fn imports_for(
    ctx: &EmitCtx<'_>,
    signature: &str,
    owner: &ClassEmission,
    recv: &ClassEmission,
    already: &mut BTreeSet<String>,
    arg_uses: Option<&BTreeMap<String, String>>,
    ems: Option<&Emissions>,
) -> Vec<String> {
    let index = use_index(ctx, owner);
    let self_short = owner.binary_name.contains('/').then(|| ctx.short(&owner.binary_name));
    let self_use = |ident: &str| -> Option<String> {
        (self_short.as_deref() == Some(ident))
            .then(|| format!("use {};", class_use_path(ctx, &owner.binary_name, &owner.crate_prefix, ems, &recv.crate_name)))
    };
    let owner_crate = if owner.crate_name.is_empty() { JAVA_RUNTIME } else { owner.crate_name.as_str() };
    let mut out = Vec::new();
    let mut seen_ident = BTreeSet::new();
    for ident in idents(signature) {
        if !seen_ident.insert(ident) || already.contains(ident) {
            continue;
        }
        let Some(use_line) = index.map.get(ident).cloned().or_else(|| self_use(ident)) else {
            if let Some(u) = arg_uses.and_then(|a| a.get(ident)) {
                already.insert(ident.to_string());
                out.push(u.clone());
            }
            continue;
        };
        let line = match use_line.strip_prefix("use crate::") {
            Some(rest) if owner_crate != recv.crate_name => format!("use {owner_crate}::{rest}"),
            _ => use_line,
        };
        already.insert(ident.to_string());
        out.push(line);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::use_name;

    #[test]
    fn use_name_matches_regex() {
        let rx = regex::Regex::new(r"^use\s+(.+)::([A-Za-z_][A-Za-z0-9_]*);\s*$").unwrap();
        for l in [
            "use crate::java::lang::String;",
            "use crate::java::lang::String;  ",
            "use  a::B;",
            "use a::B; x",
            "use a:::B;",
            "use ::B;",
            "use  ::B;",
            "use\u{3000}x::_B9;",
            "usex::B;",
            "use a::B::{C};",
            "use a::1B;",
            "use a::B;\u{a0}",
            "  use a::B;",
        ] {
            assert_eq!(use_name(l), rx.captures(l).map(|c| c.get(2).unwrap().as_str()), "{l}");
        }
    }
}

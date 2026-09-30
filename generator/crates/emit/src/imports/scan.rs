//! 按发射文本补导（← `import_gen.scan_used_vtable_imports` /
//! `scan_supplementary_iface_imports`）：方法体 UFCS 用到的 `X__VTable`，以及接口伴生契约
//! 补发声明里出现的注册表短名。

use std::collections::{BTreeMap, BTreeSet};

use super::cross::{rust_pkg_of, Prefix};
use crate::ctx::EmitCtx;

/// prelude 由生成文件头统一引入的名字
const PRELUDE_NAMES: [&str; 15] = [
    "JArray",
    "JvmError",
    "Result",
    "Object",
    "ObjectVTable",
    "String",
    "Rc",
    "__Shared",
    "RefCell",
    "Vec",
    "Box",
    "Option",
    "MonitorGuard",
    "_is_jnull",
    "_is_jnull_ref",
];
/// prelude 里的根类 `clone` base 函数
const PRELUDE_ROOT_CLONE_BASE: &str = "Object__clone_base";

fn is_word(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}

/// 文本中的 `\w+` 词（按出现序）
fn words(text: &str) -> impl Iterator<Item = &str> {
    text.split(|c: char| !is_word(c)).filter(|w| !w.is_empty())
}

/// `use (.+)::(\w+);$` → (路径, 末段)
fn parse_use(line: &str) -> Option<(&str, &str)> {
    let body = line.trim().strip_prefix("use ")?.strip_suffix(';')?;
    let (path, last) = body.rsplit_once("::")?;
    (!path.is_empty() && !last.is_empty() && last.chars().all(is_word)).then_some((path, last))
}

/// `scan_used_vtable_imports`
pub fn used_vtable_imports(
    ctx: &EmitCtx<'_>,
    blocks: &[String],
    cross_imports: &[String],
    struct_name: &str,
    prefix: Prefix<'_>,
) -> Vec<String> {
    let used: BTreeSet<&str> = blocks
        .iter()
        .flat_map(|b| words(b))
        .filter(|w| w.len() > "__VTable".len() && w.ends_with("__VTable"))
        .collect();
    let mut out = Vec::new();
    if used.is_empty() {
        return out;
    }
    let mut simple_to_pkg: BTreeMap<&str, &str> = BTreeMap::new();
    for l in cross_imports {
        if let Some((p, s)) = parse_use(l) {
            simple_to_pkg.insert(s, p);
        }
    }
    for vt in used {
        let already = cross_imports.iter().any(|l| l.contains(&format!("{vt};")) || l.contains(&format!("{vt}::")));
        if already {
            continue;
        }
        let base = &vt[..vt.len() - "__VTable".len()];
        if base == struct_name {
            continue;
        }
        let mut pkg = simple_to_pkg.get(base).map(|p| (*p).to_string());
        if pkg.is_none() {
            let norm = base.replace('_', "$");
            let hit = ctx.ty.reg.iter_insertion().find(|c| {
                let raw_short = c.name().rsplit('/').next().unwrap_or(c.name());
                ctx.ty.names.short(c.name()) == base || raw_short == norm
            });
            if let Some(c) = hit {
                pkg = rust_pkg_of(c.name()).map(|p| format!("{}::{p}", prefix.of(c.name())));
            }
        }
        if let Some(p) = pkg {
            out.push(format!("use {p}::{vt};"));
        }
    }
    out
}

/// `scan_supplementary_iface_imports`
pub fn supplementary_iface_imports(
    ctx: &EmitCtx<'_>,
    supp_blocks: &[String],
    cross_imports: &[String],
    struct_name: &str,
    prefix: Prefix<'_>,
) -> Vec<String> {
    let mut out = Vec::new();
    if supp_blocks.is_empty() || ctx.ty.reg.is_empty() {
        return out;
    }
    let used: BTreeSet<&str> = supp_blocks
        .iter()
        .flat_map(|b| words(b))
        .filter(|w| w.len() >= 2 && w.starts_with(|c: char| c.is_ascii_uppercase()))
        .filter(|w| !PRELUDE_NAMES.contains(w) && *w != PRELUDE_ROOT_CLONE_BASE)
        .collect();
    let mut short_to_bin: BTreeMap<String, &str> = BTreeMap::new();
    for c in ctx.ty.reg.iter().filter(|c| c.name().contains('/')) {
        let s = c.name().rsplit('/').next().unwrap_or(c.name()).replace('$', "_");
        short_to_bin.entry(s).or_insert(c.name());
    }
    let mut imported: BTreeSet<&str> = BTreeSet::new();
    let mut simple_to_pkg: BTreeMap<&str, &str> = BTreeMap::new();
    for l in cross_imports {
        if let Some((p, s)) = parse_use(l) {
            imported.insert(s);
            simple_to_pkg.entry(s).or_insert(p);
        }
    }
    for name in used {
        if name == struct_name || imported.contains(name) {
            continue;
        }
        let Some(bin) = short_to_bin.get(name) else { continue };
        let pkg = match simple_to_pkg.get(name) {
            Some(p) => (*p).to_string(),
            None => format!("{}::{}", prefix.of(bin), rust_pkg_of(bin).unwrap_or_default()),
        };
        out.push(format!("use {pkg}::{name};"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn use_lines() {
        assert_eq!(parse_use("use crate::java::util::Map;"), Some(("crate::java::util", "Map")));
        assert_eq!(parse_use("use x;"), None);
        assert_eq!(words("A__VTable::f(x.y)").collect::<Vec<_>>(), vec!["A__VTable", "f", "x", "y"]);
    }
}

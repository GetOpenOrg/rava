//! 继承成员 / 接口实现引用类型的 use 行：按引用类 binary 生成路径（不读其它类的文件头）。

use std::collections::BTreeSet;

use ty::ident::is_rust_keyword;

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

/// 文件 use 行导入的末段名（`_USE_RE` 的 group 2），全文扫描
pub fn imported_names(text: &str) -> BTreeSet<String> {
    text.split('\n').filter_map(|l| use_name(l).map(str::to_string)).collect()
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

/// 引用类（binary，按出现序）在接收者文件里的 use 行：只导入有发射记录的类；
/// `already` 为接收者已导入的名字（随之更新）
pub fn class_uses<S: AsRef<str>>(ctx: &EmitCtx<'_>, classes: &[S], recv: &ClassEmission, already: &mut BTreeSet<String>, ems: &Emissions) -> Vec<String> {
    let mut out = Vec::new();
    for b in classes {
        let b = b.as_ref();
        if !ems.contains_key(b) || !already.insert(ctx.short(b)) {
            continue;
        }
        out.push(format!("use {};", class_use_path(ctx, b, &recv.crate_prefix, Some(ems), &recv.crate_name)));
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

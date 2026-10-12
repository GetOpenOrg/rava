//! VM 基础设施文件（runtime/java_runtime/src 下非共置的手写文件，如 monitor.rs）对 Java 字段的写入。
//!
//! 共置文件（`<类>_impl.rs` / `_ext.rs`）的写入随所属手写方法可达才计入（引擎按方法扫描）；基础设施文件
//! 不属于任何 Java 方法，其中的 helper 由任意手写方法跨文件调用（`monitor::clear_current_interrupted` 写
//! `Thread.interrupted`），按方法无法归属。按「基础设施恒可达」处理：出现的字段写访问器名一律视为写入来源，
//! 同名字段不折叠。只按名字取（接收者类型需语法推断，保守起见不收窄）。

use std::collections::BTreeSet;
use std::path::Path;

use super::{Handwritten, GENERATED_MARK, SET_PREFIX, STATIC_SET_PREFIX, SUFFIXES};

impl Handwritten {
    /// 基础设施文件里写访问器（`x.__set_<字段>(…)` / `T::set_<字段>(…)`）点名的字段名
    pub fn vm_field_writes(&self) -> BTreeSet<String> {
        let mut out = BTreeSet::new();
        walk(&self.src, &mut |path, content| {
            // 私有辅助目录内的文件按其宿主归类（共置手写的辅助文件随宿主方法扫描）
            let host = super::layout::host_of(&self.src, path);
            let name = host.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if SUFFIXES.iter().any(|s| name.ends_with(s)) || content.contains(GENERATED_MARK) {
                return;
            }
            out.extend(setter_names(content));
        });
        out
    }
}

fn walk(dir: &Path, f: &mut dyn FnMut(&Path, &str)) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<_> = rd.flatten().map(|e| e.path()).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, f);
        } else if p.extension().is_some_and(|e| e == "rs") {
            if let Ok(c) = std::fs::read_to_string(&p) {
                f(&p, &c);
            }
        }
    }
}

/// 文本中的写访问器点名的字段名：实例 `__set_<名>`、静态 `::set_<名>(`
fn setter_names(content: &str) -> Vec<String> {
    let ident = |s: &str| -> String { s.chars().take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '$').collect() };
    let mut out = Vec::new();
    for (i, _) in content.match_indices(SET_PREFIX) {
        let n = ident(&content[i + SET_PREFIX.len()..]);
        if !n.is_empty() {
            out.push(n);
        }
    }
    let stat = format!("::{STATIC_SET_PREFIX}");
    for (i, _) in content.match_indices(&stat) {
        let rest = &content[i + stat.len()..];
        let n = ident(rest);
        if !n.is_empty() && rest[n.len()..].starts_with('(') {
            out.push(n);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_and_static_setters() {
        let src = "t.__set_interrupted(false); t.__get_holder().__set_threadStatus(3); Foo::set_count(1); x.set_y(2);";
        assert_eq!(setter_names(src), vec!["interrupted", "threadStatus", "count"]);
    }

    #[test]
    fn static_setter_needs_call() {
        assert!(setter_names("use a::set_up;").is_empty());
    }
}

//! 发射层的边界判定。
//!
//! C1d 终态：包前缀截断已取消，边界只剩 VM 契约类——最外层类列在 closure.toml
//! `[vm_boundary] classes`，且不属于 `translate_nested`（按字节码翻译的纯 Java 嵌套类）。

use crate::manifest::RuntimeManifest;

fn outer_of(cls: &str) -> &str {
    cls.split('$').next().unwrap_or(cls)
}

/// 类条目匹配类自身及其嵌套类
fn listed(entries: &[String], cls: &str) -> bool {
    entries
        .iter()
        .any(|r| cls == r || cls.strip_prefix(r.as_str()).is_some_and(|t| t.starts_with('$')))
}

/// 边界判定上下文（清单）
pub struct Boundary<'a> {
    manifest: &'a RuntimeManifest,
}

impl<'a> Boundary<'a> {
    pub fn new(manifest: &'a RuntimeManifest) -> Boundary<'a> {
        Boundary { manifest }
    }

    /// VM 耦合边界类：按方法划分
    pub fn is_vm_boundary_class(&self, cls: &str) -> bool {
        let m = self.manifest;
        m.vm_boundary_classes.contains(outer_of(cls)) && !listed(&m.release, cls)
    }

    /// VM 边界类中 `<clinit>` 由手写层承载、不翻译的类（`[vm_boundary] clinit_carried`）
    pub fn is_vm_clinit_carried(&self, cls: &str) -> bool {
        self.is_vm_boundary_class(cls) && self.manifest.vm_clinit_carried.contains(outer_of(cls))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_matching() {
        let rel = vec!["x/Y".to_string()];
        assert!(listed(&rel, "x/Y"));
        assert!(listed(&rel, "x/Y$Z"));
        assert!(!listed(&rel, "x/YZ"));
    }
}

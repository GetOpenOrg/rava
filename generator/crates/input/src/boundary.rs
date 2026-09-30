//! 发射层的边界判定（`codegen/callchain.py` `_is_boundary_class` / `_is_vm_boundary_class` 的移植）。
//!
//! - 内部边界类：类名落在 closure.toml `[boundary]` 包前缀内，且不属于 K-JCA 放行、通用放行、
//!   纯数据资源束三者之一。
//! - VM 耦合边界类：最外层类列在 `[vm_boundary] classes`、未通用放行。
//!
//! 纯数据资源束判定复用闭包分析器的 [`Carriers`]（同一结构判据），结果按类缓存在实例上。

use std::sync::RwLock;
use std::collections::BTreeMap;

use closure::seeds::data_bundle::Carriers;
use resolve::classpath::{ClassPath, Origin};

use crate::manifest::RuntimeManifest;

fn outer_of(cls: &str) -> &str {
    cls.split('$').next().unwrap_or(cls)
}

/// 包前缀（`/` 结尾）作前缀匹配；类条目匹配类自身及其嵌套类
fn released_general(release: &[String], cls: &str) -> bool {
    release.iter().any(|r| {
        if r.ends_with('/') {
            cls.starts_with(r.as_str())
        } else {
            cls == r || cls.strip_prefix(r.as_str()).is_some_and(|t| t.starts_with('$'))
        }
    })
}

/// K-JCA 放行：包前缀，或最外层类等于类条目
fn jca_released(jca: &[String], cls: &str) -> bool {
    let outer = outer_of(cls);
    jca.iter().any(|r| (r.ends_with('/') && cls.starts_with(r.as_str())) || outer == r)
}

/// 边界判定上下文（清单 + 类路径 + 资源束判定缓存）
pub struct Boundary<'a> {
    manifest: &'a RuntimeManifest,
    cp: &'a ClassPath,
    carriers: Carriers,
    bundle_cache: RwLock<BTreeMap<String, bool>>,
}

impl<'a> Boundary<'a> {
    pub fn new(manifest: &'a RuntimeManifest, cp: &'a ClassPath) -> Boundary<'a> {
        Boundary {
            manifest,
            cp,
            carriers: Carriers::new(&manifest.data_bundle_carriers),
            bundle_cache: RwLock::new(BTreeMap::new()),
        }
    }

    /// 发射层可装载的类（用户档案除外：用户类只来自本编译单元）
    fn loadable(&self, cls: &str) -> Option<std::sync::Arc<classfile::ClassFile>> {
        if self.cp.origin(cls) == Some(Origin::User) {
            return None;
        }
        self.cp.get(cls)
    }

    fn is_data_bundle(&self, cls: &str) -> bool {
        if cls.contains('[') {
            return false;
        }
        if let Some(v) = self.bundle_cache.read().unwrap_or_else(|e| e.into_inner()).get(cls) {
            return *v;
        }
        let v = self
            .loadable(cls)
            .is_some_and(|cf| self.carriers.is_pure_data_bundle(self.cp, &cf));
        self.bundle_cache.write().unwrap_or_else(|e| e.into_inner()).insert(cls.to_string(), v);
        v
    }

    fn released_general(&self, cls: &str) -> bool {
        released_general(&self.manifest.release, cls)
    }

    /// 内部边界类：包前缀截断、手写承载（前缀内的放行类 / 纯数据资源束除外）
    pub fn is_boundary_class(&self, cls: &str) -> bool {
        let m = self.manifest;
        if m.boundary_packages.iter().any(|p| cls.starts_with(p.as_str())) {
            return !(jca_released(&m.jca_release, cls) || self.released_general(cls) || self.is_data_bundle(cls));
        }
        false
    }

    /// VM 耦合边界类：按方法划分，`<clinit>` 不翻译
    pub fn is_vm_boundary_class(&self, cls: &str) -> bool {
        let m = self.manifest;
        m.vm_boundary_classes.contains(outer_of(cls)) && !self.released_general(cls)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn release_matching() {
        let rel = vec!["a/b/".to_string(), "x/Y".to_string()];
        assert!(released_general(&rel, "a/b/C"));
        assert!(released_general(&rel, "x/Y"));
        assert!(released_general(&rel, "x/Y$Z"));
        assert!(!released_general(&rel, "x/YZ"));
        let jca = vec!["p/".to_string(), "q/R".to_string()];
        assert!(jca_released(&jca, "p/S"));
        assert!(jca_released(&jca, "q/R$1"));
        assert!(!jca_released(&jca, "q/RS"));
    }
}

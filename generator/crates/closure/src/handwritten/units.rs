//! 运行时模块单元：`runtime/java_runtime/src` 下不属于任何 Java 类的手写 Rust 模块（VM 基础设施：
//! 异常构造、数组、监视器、反射分派、crate 根……）。
//!
//! 它们与类的共置手写文件同样按 fn 扫描（调用、分配、构造、字段访问，同文件传递闭包），宿主名取
//! 模块路径（`a/b.rs` / `a/b/mod.rs` → `a/b`，crate 根 → [`CRATE_ROOT`]），供引擎把跨文件调用建成
//! 调用图（见 `engine/rtfn.rs`）。识别只看文件形态（非 `_impl` / `_ext`、无生成标记），不按文件名特判。

use std::collections::BTreeMap;

use super::scan::{close_transitive, scan_file, FileFns};
use super::*;

/// crate 根模块（`lib.rs`）的宿主名
pub const CRATE_ROOT: &str = "lib";
/// 目录模块文件（`a/mod.rs`）的路径后缀
const MOD_SUFFIX: &str = "/mod";

/// 文件里定义的类型名（struct / enum / trait / type / union，含内联模块）
fn defined_types(items: &[syn::Item], out: &mut BTreeSet<String>) {
    for i in items {
        match i {
            syn::Item::Struct(s) => {
                out.insert(s.ident.to_string());
            }
            syn::Item::Enum(e) => {
                out.insert(e.ident.to_string());
            }
            syn::Item::Trait(t) => {
                out.insert(t.ident.to_string());
            }
            syn::Item::Type(t) => {
                out.insert(t.ident.to_string());
            }
            syn::Item::Union(u) => {
                out.insert(u.ident.to_string());
            }
            syn::Item::Mod(m) => {
                if let Some((_, items)) = &m.content {
                    defined_types(items, out);
                }
            }
            _ => {}
        }
    }
}

/// 源文件 → 模块单元宿主名（共置手写文件与生成文件不是单元）
fn unit_host(src: &Path, path: &Path) -> Option<String> {
    let rel = path.strip_prefix(src).ok()?.to_str()?.replace('\\', "/");
    if SUFFIXES.iter().any(|s| rel.ends_with(s)) {
        return None;
    }
    let stem = rel.strip_suffix(".rs")?;
    if stem == CRATE_ROOT {
        return Some(CRATE_ROOT.to_string());
    }
    Some(stem.strip_suffix(MOD_SUFFIX).unwrap_or(stem).to_string())
}

fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    let mut entries: Vec<PathBuf> = rd.filter_map(|e| e.ok().map(|e| e.path())).collect();
    entries.sort();
    for p in entries {
        if p.is_dir() {
            walk(&p, out);
        } else if p.extension().is_some_and(|x| x == "rs") {
            out.push(p);
        }
    }
}

impl Handwritten {
    /// 全部模块单元（宿主名 → 扫描结果；首次使用时整体载入）
    pub fn units(&self) -> Rc<BTreeMap<String, Rc<ClassHw>>> {
        if let Some(u) = self.units.borrow().as_ref() {
            return u.clone();
        }
        let mut files = Vec::new();
        walk(&self.src, &mut files);
        let mut out = BTreeMap::new();
        for path in files {
            let Some(host) = unit_host(&self.src, &path) else { continue };
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            if content.contains(GENERATED_MARK) {
                continue;
            }
            let file = match syn::parse_file(&content) {
                Ok(f) => f,
                Err(e) => {
                    self.errors.borrow_mut().push(format!("{}：{e}", path.display()));
                    continue;
                }
            };
            let mut raw = FileFns::default();
            scan_file(&file, &self.prelude, &mut raw);
            close_transitive(&mut raw.fns, &raw.calls);
            let mut hw = ClassHw { files: vec![path], ..Default::default() };
            defined_types(&file.items, &mut hw.types);
            hw.objects = objects::close(&raw);
            hw.fns = raw.fns;
            out.insert(host, Rc::new(hw));
        }
        let out = Rc::new(out);
        *self.units.borrow_mut() = Some(out.clone());
        out
    }

    /// 宿主（类或模块单元）的手写体
    pub fn host(&self, host: &str) -> Rc<ClassHw> {
        match self.units().get(host) {
            Some(u) => u.clone(),
            None => self.class(host),
        }
    }

    /// 绝对路径调用 `crate::m1::…::[T::]f` 的目标单元：模块路径是某个单元，余下至多一个该单元定义的类型段，
    /// 且单元里有 fn `f`
    pub fn unit_fn(&self, path: &[String], f: &str) -> Option<String> {
        let segs = path.strip_prefix(&["crate".to_string()])?;
        let units = self.units();
        for k in (0..=segs.len()).rev() {
            let host = if k == 0 { CRATE_ROOT.to_string() } else { segs[..k].join("/") };
            let Some(u) = units.get(&host) else { continue };
            let typed = match &segs[k..] {
                [] => true,
                [t] => u.types.contains(t),
                _ => false,
            };
            return (typed && u.fns.contains_key(f)).then_some(host);
        }
        None
    }

    /// 定义了 fn `f` 的单元（VM 规则按 fn 名定位运行时实现）
    pub fn units_with_fn(&self, f: &str) -> Vec<String> {
        self.units().iter().filter(|(_, u)| u.fns.contains_key(f)).map(|(h, _)| h.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_hosts_by_path_shape() {
        let src = Path::new("/r/src");
        let h = |p: &str| unit_host(src, &src.join(p));
        assert_eq!(h("lib.rs").as_deref(), Some(CRATE_ROOT));
        assert_eq!(h("error.rs").as_deref(), Some("error"));
        assert_eq!(h("java/lang/object.rs").as_deref(), Some("java/lang/object"));
        assert_eq!(h("jdk_resources/mod.rs").as_deref(), Some("jdk_resources"));
        assert_eq!(h("java/lang/system_impl.rs"), None);
        assert_eq!(h("java/lang/string_ext.rs"), None);
    }

    #[test]
    fn types_include_inline_modules() {
        let file = syn::parse_file("struct A; mod m { pub enum B {} pub trait C {} } type D = A;").expect("可解析");
        let mut out = BTreeSet::new();
        defined_types(&file.items, &mut out);
        assert_eq!(out, BTreeSet::from(["A", "B", "C", "D"].map(String::from)));
    }
}

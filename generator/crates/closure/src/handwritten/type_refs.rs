//! 手写文件的编译期类型引用：文件随其类进入生成范围即被编译，其中出现的每个类型路径
//! （签名、方法体、`use`）都要求对应的类存在——与执行语义无关的 L1（类型级）需求。

use std::collections::{BTreeSet, HashMap};

use syn::visit::Visit;

use super::{collect_uses, path_segs, TypeRef, GENERATED_MARK};

/// 共置手写模块后缀：`use super::<类>_impl::…` 要求该类在生成范围内（其 mod.rs 才声明该模块）
pub const MODULE_SUFFIXES: [&str; 2] = ["_impl", "_ext"];

struct TypePaths {
    uses: HashMap<String, Vec<String>>,
    out: BTreeSet<TypeRef>,
}

impl TypePaths {
    fn record(&mut self, segs: Vec<String>) {
        let segs = match segs.first().and_then(|h| self.uses.get(h)) {
            Some(full) if segs.len() > 1 || full.len() > 1 => {
                let mut v = full.clone();
                v.extend(segs.into_iter().skip(1));
                v
            }
            _ => segs,
        };
        // 共置手写模块段（`super::x_impl::T` 的 `x_impl`）与首个类型段（大写开头）各记一条
        let ty = segs.iter().position(|s| s.starts_with(|c: char| c.is_ascii_uppercase()));
        let module = segs.iter().position(|s| MODULE_SUFFIXES.iter().any(|x| s.ends_with(x)));
        if let Some(i) = module.filter(|&m| ty.map_or(true, |t| m < t)) {
            self.out.insert(TypeRef(segs[..=i].to_vec()));
        }
        if let Some(i) = ty.filter(|&i| segs[i] != "Self") {
            self.out.insert(TypeRef(segs[..=i].to_vec()));
        }
    }
}

impl<'ast> Visit<'ast> for TypePaths {
    fn visit_path(&mut self, p: &'ast syn::Path) {
        self.record(path_segs(p));
        syn::visit::visit_path(self, p);
    }
    fn visit_item_use(&mut self, u: &'ast syn::ItemUse) {
        let mut m = HashMap::new();
        collect_uses(&u.tree, &mut Vec::new(), &mut m);
        let mut paths: Vec<Vec<String>> = m.into_values().collect();
        paths.sort();
        for p in paths {
            self.record(p);
        }
    }
}

/// 一个手写文件（无生成标记）中的类型路径；解析失败返回 Err
pub fn scan(content: &str, prelude: &HashMap<String, Vec<String>>) -> Result<BTreeSet<TypeRef>, String> {
    if content.contains(GENERATED_MARK) {
        return Ok(BTreeSet::new());
    }
    let file = syn::parse_file(content).map_err(|e| e.to_string())?;
    let mut uses = prelude.clone();
    for item in &file.items {
        if let syn::Item::Use(u) = item {
            collect_uses(&u.tree, &mut Vec::new(), &mut uses);
        }
    }
    let mut tp = TypePaths { uses, out: BTreeSet::new() };
    tp.visit_file(&file);
    Ok(tp.out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn refs(src: &str) -> Vec<String> {
        scan(src, &HashMap::new()).unwrap().into_iter().map(|t| t.0.join("::")).collect()
    }

    #[test]
    fn module_and_type_paths() {
        let r = refs(
            "use super::java_lang_access_impl::SystemJavaLangAccess;\n\
             use crate::java::util::HashMap;\n\
             impl X { fn f(&self) -> Result<Self> { let m = HashMap::new(); Ok(Self) } }",
        );
        assert!(r.contains(&"super::java_lang_access_impl".to_string()));
        assert!(r.contains(&"super::java_lang_access_impl::SystemJavaLangAccess".to_string()));
        assert!(r.contains(&"crate::java::util::HashMap".to_string()));
        assert!(!r.iter().any(|s| s.ends_with("Self")));
    }

    #[test]
    fn generated_file_skipped() {
        assert!(refs("rava_macros::java_class! { struct A; }").is_empty());
    }
}

//! 注解类型种子（seeds.toml `[annotation]`）：注解实例由 JDK AnnotationParser 按原始字节解析，
//! 注解类型、元注解、枚举 / Class 元素类型只出现在注解属性体里，无静态调用边。
//!
//! 按用户类（类 / 字段 / 方法挂载点）的 RuntimeVisibleAnnotations 传递收集（含元注解与嵌套注解）。

use std::collections::{BTreeSet, HashSet};

use classfile::{Annotation, ClassFile, ElementValue};
use resolve::ClassPath;

#[derive(Debug, Default)]
pub struct AnnoCfg {
    pub triggers: Vec<String>,
    /// 触发后一并入链的精确方法（`类.方法:描述符`）
    pub seeds: Vec<String>,
}

impl AnnoCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let strs = |k: &str| -> Vec<String> {
            sec.and_then(|s| s.get(k)).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
        };
        AnnoCfg { triggers: strs("triggers"), seeds: strs("seeds") }
    }
}

/// 收集结果（不含用户类）
#[derive(Debug, Default)]
pub struct AnnoSeeds {
    /// 注解类型（方法表 = 元素面，全部入链）
    pub annos: BTreeSet<String>,
    /// 枚举元素类型（`<clinit>` 入链 + 类初始化钩子）
    pub enums: BTreeSet<String>,
    /// Class 元素类型（类型级）
    pub types: BTreeSet<String>,
}

fn desc_class(d: &str) -> Option<&str> {
    let d = d.trim_start_matches('[');
    d.strip_prefix('L').and_then(|x| x.strip_suffix(';'))
}

fn value_types(v: &ElementValue, enums: &mut BTreeSet<String>, types: &mut BTreeSet<String>, nested: &mut Vec<Annotation>) {
    match v {
        ElementValue::Enum { type_desc, .. } => {
            if let Some(c) = desc_class(type_desc) {
                enums.insert(c.to_string());
            }
        }
        ElementValue::Class(d) => {
            if let Some(c) = desc_class(d) {
                types.insert(c.to_string());
            }
        }
        ElementValue::Annotation(a) => nested.push(a.clone()),
        ElementValue::Array(xs) => {
            for x in xs {
                value_types(x, enums, types, nested);
            }
        }
        ElementValue::Const(..) => {}
    }
}

fn mounted(cf: &ClassFile) -> impl Iterator<Item = &Annotation> {
    cf.annotations.iter().chain(cf.fields.iter().flat_map(|f| f.annotations.iter())).chain(cf.methods.iter().flat_map(|m| m.annotations.iter()))
}

pub fn collect(cp: &ClassPath, users: &[std::rc::Rc<ClassFile>]) -> AnnoSeeds {
    let user_names: HashSet<&str> = users.iter().map(|c| c.name.as_str()).collect();
    let mut pending: Vec<Annotation> = users.iter().flat_map(|c| mounted(c).cloned().collect::<Vec<_>>()).collect();
    let mut seen: HashSet<String> = HashSet::new();
    let mut out = AnnoSeeds::default();
    let (mut enums, mut types) = (BTreeSet::new(), BTreeSet::new());
    while let Some(a) = pending.pop() {
        let mut nested = Vec::new();
        for (_, v) in &a.elements {
            value_types(v, &mut enums, &mut types, &mut nested);
        }
        let mut tys: Vec<String> = desc_class(&a.type_desc).map(String::from).into_iter().collect();
        for n in &nested {
            tys.extend(desc_class(&n.type_desc).map(String::from));
        }
        pending.extend(nested.iter().cloned());
        for t in tys {
            if !seen.insert(t.clone()) {
                continue;
            }
            let Some(acf) = cp.get(&t) else { continue };
            pending.extend(mounted(&acf).cloned()); // 元注解传递
            if !user_names.contains(t.as_str()) {
                out.annos.insert(t);
            }
        }
    }
    for e in enums {
        if seen.insert(e.clone()) && !user_names.contains(e.as_str()) && cp.contains(&e) {
            out.enums.insert(e);
        }
    }
    for t in types {
        if seen.insert(t.clone()) && !user_names.contains(t.as_str()) {
            out.types.insert(t);
        }
    }
    out
}

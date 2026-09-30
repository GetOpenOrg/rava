//! 类注册表：发射范围内的全部类（binary name → [`ClassInfo`]）。
//!
//! 与 Python `registry: dict` 的对应：
//! - 查找走 `BTreeMap`（确定性迭代）；
//! - Python dict 的**插入序**是可观察语义（`_registry_short_index` 同短名时后插入者胜出），
//!   这里以显式的 `order` 向量建模，只供 [`crate::ShortNames`] 构建短名反查索引使用；
//! - 插入语义 = `setdefault`（先到者占位），与 `write_cargo_project` 的
//!   「用户类 → lib 类 → JDK 类」构建顺序一致；
//! - 派生查询的缓存（有效类型形参、重载名、祖先闭包）挂在注册表实例上
//!   （`RefCell`，不是全局状态）；注册表构建后不可变，缓存无需失效。

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use classfile::class::InnerClass;
use classfile::{acc, ClassFile, Field, Method};

use crate::consts;

/// 类视图：`Rc<ClassFile>` + Python `ClassInfo` 口径的规范化
#[derive(Debug, Clone)]
pub struct ClassInfo {
    cf: Rc<ClassFile>,
    /// 类级泛型签名；类字面量类的签名置空（见 [`consts::CLASS`]）
    signature: String,
}

impl ClassInfo {
    pub fn new(cf: Rc<ClassFile>) -> ClassInfo {
        let signature = if cf.name == consts::CLASS { String::new() } else { cf.signature.clone().unwrap_or_default() };
        ClassInfo { cf, signature }
    }

    pub fn class_file(&self) -> &ClassFile {
        &self.cf
    }
    pub fn name(&self) -> &str {
        &self.cf.name
    }
    /// 父类 binary name；根类 / module-info 为空串
    pub fn super_class(&self) -> &str {
        self.cf.super_name.as_deref().unwrap_or("")
    }
    pub fn interfaces(&self) -> &[String] {
        &self.cf.interfaces
    }
    pub fn is_interface(&self) -> bool {
        self.cf.access & acc::INTERFACE != 0
    }
    /// 类级泛型签名（无 → 空串）
    pub fn generic_signature(&self) -> &str {
        &self.signature
    }
    pub fn fields(&self) -> &[Field] {
        &self.cf.fields
    }
    pub fn methods(&self) -> &[Method] {
        &self.cf.methods
    }
    pub fn inner_classes(&self) -> &[InnerClass] {
        &self.cf.inner_classes
    }
    /// EnclosingMethod 的直接外围类（局部 / 匿名类才有；否则空串）
    pub fn enclosing_class(&self) -> &str {
        self.cf.enclosing_method.as_ref().map(|(c, _)| c.as_str()).unwrap_or("")
    }
    /// EnclosingMethod 的外围方法 (name, descriptor)；位于初始化器中时为 None
    pub fn enclosing_method(&self) -> Option<(&str, &str)> {
        self.cf.enclosing_method.as_ref().and_then(|(_, m)| m.as_ref()).map(|(n, d)| (n.as_str(), d.as_str()))
    }
    /// ParsedMethod.is_constructor 口径：`<init>` 或与类名同名
    pub fn is_constructor(&self, m: &Method) -> bool {
        m.name == "<init>" || m.name == self.cf.name
    }
}

/// 方法的泛型签名（无 → 空串）
pub fn method_signature(m: &Method) -> &str {
    m.signature.as_deref().unwrap_or("")
}

/// 字段的泛型签名（无 → 空串）
pub fn field_signature(f: &Field) -> &str {
    f.signature.as_deref().unwrap_or("")
}

/// 注册表实例上的派生查询缓存
#[derive(Debug, Default)]
pub(crate) struct Caches {
    pub effective_params: RefCell<BTreeMap<String, Rc<Vec<String>>>>,
    pub overloaded: RefCell<BTreeMap<String, Rc<BTreeSet<String>>>>,
    pub super_closure: RefCell<BTreeMap<String, Rc<BTreeSet<String>>>>,
}

#[derive(Debug, Default)]
pub struct Registry {
    classes: BTreeMap<String, ClassInfo>,
    /// 插入序（Python dict 迭代序）
    order: Vec<String>,
    pub(crate) caches: Caches,
}

impl Registry {
    pub fn new() -> Registry {
        Registry::default()
    }

    /// `setdefault` 语义：已存在同名类时忽略并返回 false
    pub fn insert(&mut self, cf: Rc<ClassFile>) -> bool {
        if self.classes.contains_key(&cf.name) {
            return false;
        }
        self.order.push(cf.name.clone());
        self.classes.insert(cf.name.clone(), ClassInfo::new(cf));
        self.caches = Caches::default();
        true
    }

    pub fn get(&self, name: &str) -> Option<&ClassInfo> {
        self.classes.get(name)
    }
    pub fn contains(&self, name: &str) -> bool {
        self.classes.contains_key(name)
    }
    pub fn len(&self) -> usize {
        self.classes.len()
    }
    pub fn is_empty(&self) -> bool {
        self.classes.is_empty()
    }
    /// 按 binary name 字典序迭代
    pub fn iter(&self) -> impl Iterator<Item = &ClassInfo> {
        self.classes.values()
    }
    /// 按插入序迭代（仅短名索引等依赖插入序的语义使用）
    pub fn iter_insertion(&self) -> impl Iterator<Item = &ClassInfo> {
        self.order.iter().filter_map(|n| self.classes.get(n))
    }
}

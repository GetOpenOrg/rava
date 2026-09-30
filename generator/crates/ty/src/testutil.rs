//! 单元测试夹具：以 `ClassFile` 结构字面量构造注册表（不经 .class 解析）。

use std::sync::Arc;

use classfile::{acc, ClassFile, Field, Method};

use crate::{Manifest, Registry, ShortNames, TyCtx};

pub(crate) struct ClassSpec {
    pub cf: ClassFile,
}

pub(crate) fn class(name: &str) -> ClassSpec {
    ClassSpec {
        cf: ClassFile {
            minor: 0,
            major: 65,
            access: acc::PUBLIC,
            name: name.to_string(),
            super_name: Some(crate::consts::OBJECT.to_string()),
            interfaces: vec![],
            fields: vec![],
            methods: vec![],
            signature: None,
            source_file: None,
            bootstrap_methods: vec![],
            inner_classes: vec![],
            enclosing_method: None,
            nest_host: None,
            nest_members: vec![],
            permitted_subclasses: vec![],
            record_components: None,
            annotations: vec![],
        },
    }
}

pub(crate) fn method(access: u16, name: &str, desc: &str, sig: Option<&str>) -> Method {
    Method {
        access,
        name: name.to_string(),
        desc: desc.to_string(),
        signature: sig.map(str::to_string),
        code: None,
        exceptions: vec![],
        annotations: vec![],
        annotation_default: None,
        parameters: vec![],
        synthetic_attr: false,
    }
}

pub(crate) fn field(access: u16, name: &str, desc: &str, sig: Option<&str>) -> Field {
    Field {
        access,
        name: name.to_string(),
        desc: desc.to_string(),
        signature: sig.map(str::to_string),
        constant_value: None,
        annotations: vec![],
    }
}

impl ClassSpec {
    pub fn iface(mut self) -> Self {
        self.cf.access |= acc::INTERFACE | acc::ABSTRACT;
        self
    }
    pub fn sup(mut self, s: &str) -> Self {
        self.cf.super_name = if s.is_empty() {
            None
        } else {
            Some(s.to_string())
        };
        self
    }
    pub fn ifaces(mut self, list: &[&str]) -> Self {
        self.cf.interfaces = list.iter().map(|s| s.to_string()).collect();
        self
    }
    pub fn sig(mut self, s: &str) -> Self {
        self.cf.signature = Some(s.to_string());
        self
    }
    pub fn method(mut self, m: Method) -> Self {
        self.cf.methods.push(m);
        self
    }
    pub fn field(mut self, f: Field) -> Self {
        self.cf.fields.push(f);
        self
    }
    pub fn enclosing(mut self, outer: &str, m: Option<(&str, &str)>) -> Self {
        self.cf.enclosing_method = Some((
            outer.to_string(),
            m.map(|(n, d)| (n.to_string(), d.to_string())),
        ));
        self
    }
}

/// 注册表 + 短名 + 清单的自持夹具
pub(crate) struct Fixture {
    pub reg: Registry,
    pub names: ShortNames,
    pub manifest: Manifest,
}

impl Fixture {
    pub fn new(specs: Vec<ClassSpec>) -> Fixture {
        let mut reg = Registry::new();
        for s in specs {
            reg.insert(Arc::new(s.cf));
        }
        let names = ShortNames::build(&reg);
        Fixture {
            reg,
            names,
            manifest: Manifest::default(),
        }
    }
    pub fn ctx(&self) -> TyCtx<'_> {
        TyCtx::new(&self.reg, &self.names, &self.manifest)
    }
}

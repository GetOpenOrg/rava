//! 方法种类判定（字节码 / 手写 / 抽象）与类的分析域。

use super::*;

impl Ctx<'_> {
    pub(in crate::engine) fn kind_of(&self, cf: &ClassFile, m: &classfile::Method) -> Kind {
        let member = format!("{}.{}:{}", cf.name, m.name, m.desc);
        match self.domain(&cf.name) {
            // VM 契约边界（`[vm_boundary]`）按方法划分：手写承载（native / VM 内建 / 共置手写体
            // 按精确名提供 / 类初始化器）的取手写效果，其余被调用到的方法运行时执行的就是其字节码
            // （发射层同样翻译），按字节码建模——否则其体内的调用与写入（如经 native 手写体
            // 写入的字段）从分析中消失，成为漏报。类初始化器由清单逐类决定（`translate_clinit`）
            Domain::Boundary => {
                let hw = self.boundary_carried(cf, m, &member);
                return if hw { Kind::Handwritten("boundary") } else { Kind::Bytecode };
            }
            Domain::Root => return Kind::Handwritten("root"),
            _ => {}
        }
        if m.is_native() {
            return Kind::Handwritten("native");
        }
        if self.man.is_intrinsic(&member) {
            return Kind::Handwritten("intrinsic");
        }
        if m.name != "<clinit>" && self.provided(cf, &m.name, &m.desc) {
            return Kind::Handwritten("provides");
        }
        if m.code.is_none() {
            return Kind::Abstract;
        }
        Kind::Bytecode
    }

    /// 边界方法由手写层承载（native / 无体 / 内部边界类与 `clinit_carried` 所列 VM 边界类的 `<clinit>` /
    /// VM 内建 / 共置手写体提供）。其余 VM 边界类的 `<clinit>` 是纯 Java 静态状态，按字节码翻译
    fn boundary_carried(&self, cf: &ClassFile, m: &classfile::Method, member: &str) -> bool {
        m.is_native()
            || m.code.is_none()
            || (m.name == "<clinit>" && (!self.man.is_vm_boundary(&cf.name) || self.man.is_vm_clinit_carried(&cf.name)))
            || self.man.is_intrinsic(member)
            || self.provided(cf, &m.name, &m.desc)
    }

    /// 共置手写体按精确 Rust 名提供该成员（与发射侧 `_nf_covered` 同口径：mangle 名，或类内无重载时的裸名）
    pub(in crate::engine) fn provided(&self, cf: &ClassFile, name: &str, desc: &str) -> bool {
        let hw = self.hw.class(&cf.name);
        if hw.fns.is_empty() || self.man.hw_dropped(&cf.name) {
            return false;
        }
        let (rust, mangled) = self.rust_names(cf, name, desc);
        hw.fns.iter().any(|(f, i)| {
            let f = f.strip_prefix("__impl_").unwrap_or(f);
            i.is_pub && (f == mangled || rust.as_deref() == Some(f))
        })
    }

    /// (类内无重载时的裸名, mangle 名)
    pub(in crate::engine) fn rust_names(&self, cf: &ClassFile, name: &str, desc: &str) -> (Option<String>, String) {
        let base = if name == "<init>" { "new" } else { name };
        let suffix = self.hw.descriptor_suffix(desc);
        let mangled = if suffix.is_empty() { base.to_string() } else { format!("{base}_{suffix}") };
        let overloaded = cf.methods.iter().filter(|m| m.name == name).count() > 1;
        (if overloaded { None } else { Some(base.to_string()) }, mangled)
    }

    pub(in crate::engine) fn domain(&self, cls: &str) -> Domain {
        if let Some(&d) = self.domains.borrow().get(cls) {
            return d;
        }
        let origin = self.cp.origin(cls);
        let d = match self.man.domain(cls, origin == Some(Origin::User)) {
            // 依赖库类（类路径 jar，Origin::Lib）：库自身不属 JDK 边界，一律按字节码翻译
            Domain::Boundary if origin == Some(Origin::Lib) => Domain::Translate,
            d => d,
        };
        self.domains.borrow_mut().insert(cls.to_string(), d);
        d
    }
}

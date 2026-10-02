//! 单类发射记录（`inherited_gen.ClassEmission` / `EmittedMethod` 的移植）：两阶段生成的第一阶段产物。
//!
//! 第一阶段生成全部类文本（期间方法体登记继承成员需求 / SAM 站点 / lambda 名），并把
//! **实际输出的实例方法声明**记入 [`ClassEmission::methods`]（定义侧真源：由产出方法段的
//! 生成点直接给出结构化记录，签名以 [`FnSig`] 承载类型身份）；第二阶段（接口实现、继承成员、SAM 合成、反射分派，
//! 见 [`crate::phase2`]）按记录补齐文本后统一落盘。

use std::path::PathBuf;

use classfile::Method;
use ty::FnSig;

use crate::class_writer::attrs::{access_str, MethodAttrExtra};

/// 类 `java_class!` 块里的一条实例方法声明
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EmittedMethod {
    /// Java 方法名
    pub name: String,
    /// JVM 描述符
    pub descriptor: String,
    /// Rust 方法名（含重载改名，本类重载态）
    pub rust_name: String,
    /// 结构化签名（接收方按自身命名重渲染）
    pub sig: FnSig,
    /// public / protected / private / ''（package）
    pub access: String,
    /// 声明该虚方法的 VTable 所属类 binary；非虚方法为空
    pub virtual_in: String,
    /// wrapper 名 ≠ 槽位 trait 成员名时的 trait 成员名
    pub vtable_name: String,
    /// `body = "handwritten"`：体在共置 `_impl.rs` 的 `__impl_<m>`
    pub handwritten: bool,
    /// abstract 声明（接口契约的抽象方法）
    pub is_abstract: bool,
    /// 本轮翻译出了方法体
    pub has_body: bool,
    /// 共置手写提供体、声明以 `// [meta]` 注释行承载（手写 native 等非槽位方法）：只作继承成员的
    /// 声明者（子类接收者经上转直调），不参与槽位 / 桥接 / 接口实现的定位
    pub meta: bool,
}

impl EmittedMethod {
    /// 签名行文本（按给定命名渲染）
    pub fn signature(&self, names: &ty::ShortNames) -> String {
        self.sig.render(names)
    }

    /// 定义侧声明记录：实例方法（非 `<init>` / `<clinit>`、有结构化签名）才记录
    pub fn declared(m: &Method, extra: &MethodAttrExtra, sig: Option<&FnSig>, has_body: bool) -> Option<EmittedMethod> {
        let sig = sig.filter(|_| !m.is_static() && !m.name.starts_with('<'))?;
        let esc = |s: &str| s.replace('"', "\\\"");
        let access = match m.access {
            0 => String::new(),
            a => Some(access_str(a)).filter(|a| *a != "package").unwrap_or_default().to_string(),
        };
        let virtual_in = extra.virtual_in.clone();
        let vtable_name = if virtual_in.is_empty() { String::new() } else { extra.vtable_name.clone() };
        Some(EmittedMethod {
            name: esc(&m.name),
            descriptor: esc(&m.desc),
            rust_name: sig.name.clone(),
            sig: sig.clone(),
            access,
            virtual_in,
            vtable_name,
            handwritten: extra.handwritten_body,
            is_abstract: m.is_abstract(),
            has_body,
            meta: false,
        })
    }
}

/// 宏块 impl 内的一个方法段：文本 + 定义侧声明记录（非实例方法段为 None）
#[derive(Debug, Clone, Default)]
pub struct MethodBlock {
    pub text: String,
    pub decl: Option<EmittedMethod>,
}

impl MethodBlock {
    /// 无声明记录的段（静态字段 / `<clinit>` / 手写元数据注释等）
    pub fn plain(text: String) -> MethodBlock {
        MethodBlock { text, decl: None }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClassEmission {
    pub binary_name: String,
    /// 引用本 crate 外类型的前缀（`crate` / `java_runtime`）
    pub crate_prefix: String,
    pub path: PathBuf,
    /// 目标路径是 runtime/ 手写真源（落盘时跳过；记录不可信）
    pub handwritten: bool,
    /// 所属 crate 名（`java_runtime` / `user` / lib crate 名）
    pub crate_name: String,
    pub text: String,
    /// 实际输出的实例方法声明（发射序）
    pub methods: Vec<EmittedMethod>,
}

impl ClassEmission {
    /// 名字相同、描述符以 `param_desc` 为前缀的首条声明（返回类型不参与：协变）
    pub fn find(&self, name: &str, param_desc: &str) -> Option<&EmittedMethod> {
        self.slotted().find(|m| m.name == name && m.descriptor.starts_with(param_desc))
    }

    /// 同 [`Self::find`]，含 `[meta]` 声明（继承成员的声明者定位）
    pub fn find_declared(&self, name: &str, param_desc: &str) -> Option<&EmittedMethod> {
        self.methods.iter().find(|m| m.name == name && m.descriptor.starts_with(param_desc))
    }

    /// 文本中实际声明的实例方法（不含 `[meta]` 注释声明）
    pub fn slotted(&self) -> impl Iterator<Item = &EmittedMethod> {
        self.methods.iter().filter(|m| !m.meta)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn meta_declaration_only_found_as_declarer() {
        let m = |name: &str, meta| EmittedMethod { name: name.into(), descriptor: "()V".into(), meta, ..EmittedMethod::default() };
        let em = ClassEmission { methods: vec![m("a", false), m("ctx", true)], ..ClassEmission::default() };
        assert!(em.find("ctx", "()").is_none());
        assert!(em.find_declared("ctx", "()").is_some_and(|d| d.meta));
        assert_eq!(em.slotted().count(), 1);
    }
}

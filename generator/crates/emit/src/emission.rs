//! 单类发射记录（`inherited_gen.ClassEmission` / `EmittedMethod` 的移植）：两阶段生成的第一阶段产物。
//!
//! 第一阶段生成全部类文本（期间方法体登记继承成员需求 / SAM 站点 / lambda 名），并把
//! **实际输出的实例方法声明**记入 [`ClassEmission::methods`]（定义侧真源：签名、Rust 名、
//! `virtual_in` 全部取自已生成文本）；第二阶段（接口实现、继承成员、SAM 合成、反射分派，
//! 见 [`crate::phase2`]）按记录补齐文本后统一落盘。

use std::path::PathBuf;
use std::sync::OnceLock;

use regex::Regex;

/// 类 `java_class!` 块里的一条实例方法声明
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EmittedMethod {
    /// Java 方法名
    pub name: String,
    /// JVM 描述符
    pub descriptor: String,
    /// Rust 方法名（含重载改名，本类重载态）
    pub rust_name: String,
    /// `pub fn name(&self, ..) -> Result<..>`（不含方法体、参数不带 mut）
    pub signature: String,
    /// public / protected / private / ''（package）
    pub access: String,
    /// 声明该虚方法的 VTable 所属类（Rust 短名）；非虚方法为空
    pub virtual_in: String,
    /// wrapper 名 ≠ 槽位 trait 成员名时的 trait 成员名
    pub vtable_name: String,
    /// `body = "handwritten"`：体在共置 `_impl.rs` 的 `__impl_<m>`
    pub handwritten: bool,
    /// abstract 声明（接口契约的抽象方法）
    pub is_abstract: bool,
    /// 本轮翻译出了方法体
    pub has_body: bool,
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

struct RecordRes {
    attr: Regex,
    access: Regex,
    virtual_in: Regex,
    vtable_name: Regex,
    fn_name: Regex,
    is_static: Regex,
    handwritten: Regex,
    is_abstract: Regex,
    mut_binding: Regex,
}

fn res() -> &'static RecordRes {
    static R: OnceLock<RecordRes> = OnceLock::new();
    R.get_or_init(|| {
        let re = |p: &str| Regex::new(p).expect("静态正则");
        RecordRes {
            attr: re(r#"#\[java_(?:method|native)\(name = "((?:[^"\\]|\\.)*)", descriptor = "((?:[^"\\]|\\.)*)"([^\n]*)"#),
            access: re(r#"\baccess = "([^"]*)""#),
            virtual_in: re(r#"\bvirtual_in = "([^"]*)""#),
            vtable_name: re(r#"\bvtable_name = "([^"]*)""#),
            fn_name: re(r"^pub fn\s+([A-Za-z_][A-Za-z0-9_]*)"),
            is_static: re(r"\bis_static\s*=\s*true"),
            handwritten: re(r#"\bbody\s*=\s*"handwritten""#),
            is_abstract: re(r"\bis_abstract\s*=\s*true"),
            mut_binding: re(r"\bmut\s+([A-Za-z_][A-Za-z0-9_]*\s*:)"),
        }
    })
}

/// 方法块 → 声明记录（非实例方法 / 无 `&self` 签名的块返回 None）
fn parse_block(block: &str) -> Option<EmittedMethod> {
    let r = res();
    let cap = r.attr.captures(block)?;
    let (name, descriptor, rest) = (&cap[1], &cap[2], &cap[3]);
    if name.starts_with('<') || r.is_static.is_match(rest) {
        return None;
    }
    let sig_line = block.split('\n').find(|l| l.trim_start().starts_with("pub fn "))?.trim();
    let fn_name = r.fn_name.captures(sig_line)?[1].to_string();
    if !sig_line.contains("&self") {
        return None;
    }
    let mut signature = sig_line.trim_end();
    let has_body = !block.trim_end().ends_with(';');
    if signature.ends_with(';') || signature.ends_with('{') {
        signature = signature[..signature.len() - 1].trim_end();
    }
    let signature = r.mut_binding.replace_all(signature, "$1").into_owned();
    let get = |re: &Regex| re.captures(rest).map(|c| c[1].to_string()).unwrap_or_default();
    Some(EmittedMethod {
        name: name.to_string(),
        descriptor: descriptor.to_string(),
        rust_name: fn_name,
        signature,
        access: get(&r.access),
        virtual_in: get(&r.virtual_in),
        vtable_name: get(&r.vtable_name),
        handwritten: r.handwritten.is_match(rest),
        is_abstract: r.is_abstract.is_match(rest),
        has_body,
    })
}

/// 方法块表 → 实例方法声明记录（`ClassEmission.record_methods`）
pub fn record_methods(method_blocks: &[String]) -> Vec<EmittedMethod> {
    method_blocks.iter().filter_map(|b| parse_block(b)).collect()
}

impl ClassEmission {
    /// 名字相同、描述符以 `param_desc` 为前缀的首条声明（返回类型不参与：协变）
    pub fn find(&self, name: &str, param_desc: &str) -> Option<&EmittedMethod> {
        self.methods.iter().find(|m| m.name == name && m.descriptor.starts_with(param_desc))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_instance_method() {
        let block = "#[java_method(name = \"put\", descriptor = \"(Ljava/lang/Object;)V\", access = \"public\", virtual_in = \"AbstractMap\")]\npub fn put(&self, mut a: Object) -> Result<()> {\n    Ok(())\n}";
        let m = record_methods(&[block.to_string()]);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].signature, "pub fn put(&self, a: Object) -> Result<()>");
        assert_eq!(m[0].virtual_in, "AbstractMap");
        assert!(m[0].has_body);
        let stat = "#[java_method(name = \"f\", descriptor = \"()V\", is_static = true)]\npub fn f() -> Result<()>;";
        assert!(record_methods(&[stat.to_string()]).is_empty());
    }
}

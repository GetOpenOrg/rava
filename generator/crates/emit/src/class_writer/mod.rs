//! 单类文件文本（← `emitter/class_writer._gen_class_rs`）。
//!
//! 组装顺序与 Python 一致：文件头 + cross_imports + 继承导入插入位 → `java_class! { 类块头 /
//! struct / impl { static 字段 · 方法块 · 继承成员插入位 } / 接口实现插入位 }` → 协变 upcast
//! 插入位 → implref 稳定别名 → 接口 lambda 擦除 impl → 按文本补导的 use。
//! 方法块在步骤 (c)、vtable / 继承段在步骤 (d) 接入 [`gen_class_rs`] 的方法块表。

pub mod attrs;
pub mod fields;
pub mod head;
pub mod hw_overrides;
pub mod methods;
pub mod slot;
pub mod stub;

use std::collections::BTreeSet;

use ty::rs_type::render_arg_list;
use ty::ClassInfo;

use crate::body::MethodBodyEmitter;
use crate::ctx::{EmitCtx, ProjectState};
use crate::error::Result;
use crate::imports::refs::add_desc_refs;
use crate::imports::{collect_referenced, gen_cross_imports, supplementary_iface_imports, used_vtable_imports, CrossInput};
use crate::lang;
use crate::project::layout::JdkLayout;
use crate::text::indent;

/// 继承成员 use 插入位（`inherited_gen.IMPORTS_SLOT`）
pub const INHERITED_IMPORTS_SLOT: &str = "//@@rava:inherited-imports@@";
/// 继承成员声明插入位（`inherited_gen.MEMBERS_SLOT`）
pub const INHERITED_MEMBERS_SLOT: &str = "//@@rava:inherited-members@@";
/// 接口实现插入位（`interface_gen.IMPLS_SLOT`）
pub const INTERFACE_IMPLS_SLOT: &str = "//@@rava:interface-impls@@";
/// 接口协变 upcast 插入位（`interface_gen.UPCASTS_SLOT`）
pub const INTERFACE_UPCASTS_SLOT: &str = "//@@rava:interface-upcasts@@";

const FILE_ALLOW: &str = "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types, static_mut_refs, unused_comparisons)]";

/// 类所在 crate 的发射参数
pub struct ClassSite<'l> {
    /// java_runtime 布局（包路径 / 同名消歧 / 跳过包 / 生成集）
    pub jdk: &'l JdkLayout,
    /// user crate 类：同 crate 兄弟类导入行；None = java_runtime 内的 JDK 类
    pub user_sibling_imports: Option<Vec<String>>,
}

impl ClassSite<'_> {
    fn is_user(&self) -> bool {
        self.user_sibling_imports.is_some()
    }

    /// 引用 crate 外类型的前缀
    pub fn prefix(&self) -> &'static str {
        if self.is_user() {
            "java_runtime"
        } else {
            "crate"
        }
    }

    /// cross_imports 参数（用户类在无 JDK 类时不做包集合导入与生成集过滤）
    fn cross_input(&self) -> CrossInput<'_> {
        let jdk_on = !self.is_user() || !self.jdk.files.is_empty();
        CrossInput {
            pkg_paths: jdk_on.then_some(self.jdk.pkg_paths.as_slice()),
            generated: jdk_on.then_some(&self.jdk.generated),
            conflict_map: jdk_on.then_some(&self.jdk.conflict_map),
            skipped: jdk_on.then_some(&self.jdk.skipped_classes),
            sibling_imports: self.user_sibling_imports.as_deref().unwrap_or(&[]),
            prefix: self.prefix(),
            all_in_chain: self.is_user(),
        }
    }
}

/// Rust struct 名：含包或内部类分隔符时取短名
pub fn struct_name(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> String {
    if ci.name().contains(['/', '$']) {
        ctx.short(ci.name())
    } else {
        ci.name().to_string()
    }
}

/// 类型存根：JDK 类且无任何方法在调用链上（static 字段发 panic 存根访问器）
pub fn is_type_only(ctx: &EmitCtx<'_>, ci: &ClassInfo, site: &ClassSite<'_>) -> bool {
    !site.is_user() && !ci.methods().iter().any(|m| ctx.in_chain(ci.name(), &m.name, &m.desc))
}

/// 父类 Rust 类型（含本类视角的祖先实参）；父类为根类时为空
fn parent_rust(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> String {
    let sup = ci.super_class();
    if sup.is_empty() || sup == lang::OBJECT {
        return String::new();
    }
    let args = ctx
        .ty
        .ancestor_type_args(ci, None)
        .into_iter()
        .find(|(n, _)| n == sup)
        .map(|(_, a)| render_arg_list(&a, ctx.ty.names))
        .unwrap_or_default();
    format!("{}{args}", ctx.short(sup))
}

/// 生成单类文件文本
pub fn gen_class_rs(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
) -> Result<String> {
    let cross = site.cross_input();
    let mut referenced = collect_referenced(ctx, ci, cross.generated);
    let visible: Vec<&classfile::Method> = ci.methods().iter().filter(|m| !m.is_synthetic()).collect();
    let overrides = hw_overrides::handwritten_inherited_overrides(ctx, ci, &visible);
    for o in &overrides {
        add_desc_refs(&o.method.desc, &mut referenced);
        add_desc_refs(o.method.signature.as_deref().unwrap_or(""), &mut referenced);
    }
    let cross_imports = gen_cross_imports(ctx, state, ci, &cross, &referenced)?;
    let is_iface = ci.is_interface();
    let mut parts: Vec<String> = vec![FILE_ALLOW.to_string(), format!("use {}::prelude::*;", site.prefix())];
    parts.extend(cross_imports.iter().cloned());
    parts.push(INHERITED_IMPORTS_SLOT.to_string());
    parts.push(String::new());

    let sname = struct_name(ctx, ci);
    let tps: Vec<String> = ctx.ty.effective_class_type_params(ci).to_vec();
    let (struct_generic, impl_header) = if tps.is_empty() {
        (String::new(), format!("impl {sname}"))
    } else {
        let t = tps.join(", ");
        (format!("<{t}>"), format!("impl<{t}> {sname}<{t}>"))
    };
    let sup = fields::flatten_super_fields(ctx, ci);
    let struct_lines = fields::struct_lines(ctx, ci, &tps, &sup);

    // G-10 账本：本类方法由本轮生成
    state.generated_classes.insert(ci.name().to_string());
    let mut method_blocks = fields::static_field_blocks(ctx, ci, &tps, is_type_only(ctx, ci, site));
    let mb = methods::emit_method_blocks(ctx, state, bodies, ci, &tps, &overrides)?;
    method_blocks.extend(mb.method_blocks);
    // 接口 default / special / 超类虚方法继承段（步骤 (d)）在此接入
    let (iface_lambda_blocks, iface_supp_blocks) = (mb.iface_lambda_blocks, mb.iface_supp_blocks);

    let parent = parent_rust(ctx, ci);
    let empty = BTreeSet::new();
    let impl_methods = ctx.input.handwritten.get(ci.name()).map_or(&empty, |h| &h.methods);
    let head_in = head::HeadInput {
        superclass_rust: &parent,
        superclass_fields: &sup.fields,
        superclass_reference_fields: &sup.reference,
        superclass_erased_fields: &sup.erased,
        impl_methods: Some(impl_methods),
    };
    let mut block = head::block_head(ctx, ci, &head_in);
    block.push(String::new());
    if struct_lines.is_empty() {
        block.push(format!("pub struct {sname}{struct_generic};"));
    } else {
        block.push(format!("pub struct {sname}{struct_generic} {{"));
        block.extend(struct_lines);
        block.push("}".into());
    }
    block.push(String::new());
    block.push(format!("{impl_header} {{"));
    let impl_body = method_blocks.iter().map(|b| indent(b, "    ")).collect::<Vec<_>>().join("\n\n");
    if !impl_body.is_empty() {
        block.push(impl_body);
    }
    block.push(INHERITED_MEMBERS_SLOT.into());
    block.push("}".into());
    if !is_iface {
        block.push(INTERFACE_IMPLS_SLOT.into());
    }

    parts.push("rava_macros::java_class! {".into());
    parts.extend(block.iter().map(|l| if l.is_empty() { String::new() } else { indent(l, "    ") }));
    parts.push("}".into());
    parts.push(INTERFACE_UPCASTS_SLOT.into());
    parts.push(String::new());

    let java_simple = ci.name().rsplit('/').next().unwrap_or(ci.name()).replace('$', "_");
    parts.push("#[doc(hidden)]".into());
    parts.push(format!("pub mod implref {{ pub use super::{sname} as {java_simple}; }}"));
    parts.push(String::new());

    if !iface_lambda_blocks.is_empty() {
        let recv = if tps.is_empty() { sname.clone() } else { format!("{sname}<{}>", vec!["Object"; tps.len()].join(", ")) };
        parts.push("// G-10: 接口私有实例 lambda body —— 载体擦除实例化上的固有方法".into());
        parts.push(format!("impl {recv} {{"));
        parts.extend(iface_lambda_blocks.iter().map(|b| indent(b, "    ")));
        parts.push("}".into());
        parts.push(String::new());
    }

    let scanned: Vec<String> = method_blocks.iter().chain(&iface_lambda_blocks).cloned().collect();
    parts.extend(used_vtable_imports(ctx, &scanned, &cross_imports, &sname, site.prefix()));
    if !iface_supp_blocks.is_empty() {
        parts.extend(supplementary_iface_imports(ctx, &iface_supp_blocks, &cross_imports, &sname, site.prefix()));
    }
    Ok(parts.join("\n"))
}

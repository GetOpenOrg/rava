//! 单类文件文本（← `emitter/class_writer._gen_class_rs`）。
//!
//! 组装顺序与 Python 一致：文件头 + cross_imports + 继承导入插入位 → `java_class! { 类块头 /
//! struct / impl { static 字段 · 方法块 · 继承成员插入位 } / 接口实现插入位 }` → 协变 upcast
//! 插入位 → implref 稳定别名 → 接口 lambda 擦除 impl → 按文本补导的 use。
//! 方法块在步骤 (c)、vtable / 继承段在步骤 (d) 接入 [`gen_class_rs`] 的方法块表。

pub mod attrs;
pub mod fields;
pub mod head;
pub(crate) mod inherit;
pub mod methods;
pub mod opaque;
pub mod slot;
pub mod stub;
mod super_inherit;
pub mod visibility;

use std::collections::BTreeSet;

use ty::rs_type::render_arg_list;
use ty::ClassInfo;

use crate::body::MethodBodyEmitter;
use crate::ctx::{EmitCtx, ProjectState};
use crate::error::Result;
use crate::imports::{collect_referenced, plan_cross_imports, supplementary_iface_imports, used_vtable_imports, CrateRoute, CrossInput, CrossPlan, Prefix};
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

/// lib 模式（jar 输入）下类所在 crate 的定向参数
#[derive(Clone, Copy)]
pub struct LibSite<'l> {
    /// 引用按目标类归属 crate 定向（`current` = 本类所在 lib；None = user crate）
    pub route: CrateRoute<'l>,
    /// 可导入的生成集：JDK 生成类 ∪ 全部 lib crate 类
    pub generated: &'l BTreeSet<String>,
}

/// 类所在 crate 的发射参数
pub struct ClassSite<'l> {
    /// java_runtime 布局（包路径 / 同名消歧 / 跳过包 / 生成集）
    pub jdk: &'l JdkLayout,
    /// user crate 类：同 crate 兄弟类导入行；None = java_runtime / lib crate 内的类
    pub user_sibling_imports: Option<Vec<String>>,
    /// lib 模式定向；None = 单 crate 发射（无 --lib）
    pub lib: Option<LibSite<'l>>,
}

impl ClassSite<'_> {
    fn is_user(&self) -> bool {
        self.user_sibling_imports.is_some()
    }

    /// lib crate 内的类：Java 可见性映射生效
    fn in_lib_crate(&self) -> bool {
        self.lib.is_some_and(|l| l.route.current.is_some())
    }

    /// 引用 crate 外类型的前缀（lib 模式按目标 crate 定向前的缺省值）
    pub fn prefix(&self) -> &'static str {
        if self.is_user() || self.lib.is_some() {
            "java_runtime"
        } else {
            "crate"
        }
    }

    fn scan_prefix(&self) -> Prefix<'_> {
        Prefix { base: self.prefix(), route: self.lib.map(|l| l.route) }
    }

    /// cross_imports 参数（用户类在无 JDK 类时不做包集合导入与生成集过滤；lib 模式只按生成集定向）
    fn cross_input(&self) -> CrossInput<'_> {
        if let Some(l) = self.lib {
            return CrossInput {
                pkg_paths: None,
                generated: Some(l.generated),
                conflict_map: None,
                skipped: None,
                sibling_imports: self.user_sibling_imports.as_deref().unwrap_or(&[]),
                prefix: self.prefix(),
                all_in_chain: self.is_user(),
                route: Some(l.route),
            };
        }
        let jdk_on = !self.is_user() || !self.jdk.files.is_empty();
        CrossInput {
            pkg_paths: jdk_on.then_some(self.jdk.pkg_paths.as_slice()),
            generated: jdk_on.then_some(&self.jdk.generated),
            conflict_map: jdk_on.then_some(&self.jdk.conflict_map),
            skipped: jdk_on.then_some(&self.jdk.skipped_classes),
            sibling_imports: self.user_sibling_imports.as_deref().unwrap_or(&[]),
            prefix: self.prefix(),
            all_in_chain: self.is_user(),
            route: None,
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

/// 继承展开段：接口 default 继承 → `Iface.super` / 接口私有方法展开 → 超类虚方法继承
#[allow(clippy::too_many_arguments)]
fn inherited_segments<'c>(
    ctx: &EmitCtx<'c>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    ci: &'c ClassInfo,
    tps: &[String],
    visible: &[&'c classfile::Method],
    out: &mut Vec<String>,
) -> Result<()> {
    let overloaded = ctx.ty.hierarchy_overloaded_names(ci);
    let cx = methods::Cx { ctx, ci, tps, overloaded: &overloaded };
    let all: Vec<&classfile::Method> = visible.to_vec();
    let translated = inherit::interface_default_inheritance(&cx, state, bodies, &all, out)?;
    let sources: Vec<(&ClassInfo, &classfile::Method)> = visible.iter().map(|m| (ci, *m)).collect();
    inherit::interface_special_members(&cx, state, bodies, &sources, translated, out)?;
    super_inherit::superclass_virtual_inheritance(&cx, state, bodies, &all, out)
}

/// 单类发射结果：文件文本 + 实际输出的实例方法声明记录
pub struct ClassText {
    pub text: String,
    pub methods: Vec<crate::emission::EmittedMethod>,
}

/// 单类发射的前置事实（与发射序无关，可并行求得）：非 synthetic 方法、跨类导入规划
pub struct ClassPrep<'c> {
    visible: Vec<&'c classfile::Method>,
    cross: CrossPlan,
}

/// 前置事实：引用集 → 跨类导入规划
pub fn class_prep<'c>(ctx: &EmitCtx<'c>, ci: &'c ClassInfo, site: &ClassSite<'_>) -> Result<ClassPrep<'c>> {
    let cross = site.cross_input();
    if ctx.is_opaque(ci.name()) {
        // 不透明形态只引用全部传递超类型（upcast 目标）
        let referenced: BTreeSet<String> = opaque::opaque_supers(ctx, ci).into_iter().collect();
        let cross = plan_cross_imports(ctx, ci, &cross, &referenced)?;
        return Ok(ClassPrep { visible: Vec::new(), cross });
    }
    let referenced = collect_referenced(ctx, ci, cross.generated);
    let visible: Vec<&classfile::Method> = ci.methods().iter().filter(|m| !m.is_synthetic()).collect();
    let cross = plan_cross_imports(ctx, ci, &cross, &referenced)?;
    Ok(ClassPrep { visible, cross })
}

/// 跨类导入行：`seen_simples`（跨包同名冲突守卫）跨类累积，必须按发射序串行调用（只做裁决）
pub fn class_cross_imports(state: &mut ProjectState, prep: &ClassPrep<'_>) -> Vec<String> {
    prep.cross.resolve(&mut state.seen_simples)
}

/// 生成单类文件文本（串行形态：前置 → 跨类导入 → 类体）
pub fn gen_class_rs(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
) -> Result<ClassText> {
    let prep = class_prep(ctx, ci, site)?;
    let cross_imports = class_cross_imports(state, &prep);
    class_text(ctx, state, bodies, ci, site, &prep, cross_imports)
}

/// 类体文本：除跨类导入外只读共享上下文、只向 `state` 追加账本（可按类并行，
/// `state` 为本类增量，由调用方按发射序并入）
pub fn class_text(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
    prep: &ClassPrep<'_>,
    cross_imports: Vec<String>,
) -> Result<ClassText> {
    if ctx.is_opaque(ci.name()) {
        return Ok(opaque::opaque_text(ctx, state, ci, site, cross_imports));
    }
    let visible = &prep.visible;
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
    let java_vis = site.in_lib_crate();
    let struct_lines = fields::struct_lines(ctx, ci, &tps, &sup, java_vis);

    // G-10 账本：本类方法由本轮生成
    state.generated_classes.insert(ci.name().to_string());
    let mut method_blocks = fields::static_field_blocks(ctx, ci, &tps, is_type_only(ctx, ci, site));
    let mb = methods::emit_method_blocks(ctx, state, bodies, ci, &tps)?;
    method_blocks.extend(mb.method_blocks);
    let (iface_lambda_blocks, iface_supp_blocks) = (mb.iface_lambda_blocks, mb.iface_supp_blocks);
    inherited_segments(ctx, state, bodies, ci, &tps, visible, &mut method_blocks)?;
    // 先按 `pub fn` 形态记录方法声明，再做可见性降级（记录与文本最终形态解耦）
    let methods = crate::emission::record_methods(&method_blocks);
    if java_vis {
        visibility::downgrade_non_public_blocks(&mut method_blocks);
    }
    let struct_vis = if java_vis { visibility::java_member_vis(ci.class_file().access) } else { "pub" };

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
        block.push(format!("{struct_vis} struct {sname}{struct_generic};"));
    } else {
        block.push(format!("{struct_vis} struct {sname}{struct_generic} {{"));
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
    parts.extend(used_vtable_imports(ctx, &scanned, &cross_imports, &sname, site.scan_prefix()));
    if !iface_supp_blocks.is_empty() {
        parts.extend(supplementary_iface_imports(ctx, &iface_supp_blocks, &cross_imports, &sname, site.scan_prefix()));
    }
    Ok(ClassText { text: parts.join("\n"), methods })
}

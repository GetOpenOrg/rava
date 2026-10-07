//! 单类文件文本（← `emitter/class_writer._gen_class_rs`）。
//!
//! 组装顺序：文件头 + 导入块插入位（第二阶段后由文件作用域记录填充）→ `java_class! { 类块头 /
//! struct / impl { static 字段 · 方法块 · 继承成员插入位 } / 接口实现插入位 }` → 协变 upcast
//! 插入位 → 接口 lambda 擦除 impl。
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
use crate::emission::{EmittedMethod, MethodBlock};
use crate::error::Result;
use crate::imports::{claim_structural, collect_referenced, CrateRoute, CrossInput, ImportSite};
use crate::lang;
use crate::module_crates::ModuleCrates;
use crate::project::layout::{JdkLayout, UserLayout};
use crate::text::indent;

/// 导入块插入位（第二阶段后由文件作用域记录填充）
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
    /// JDK 布局（包路径 / 同名消歧 / 跳过包 / 生成集）
    pub jdk: &'l JdkLayout,
    /// user crate 布局：本类属 user crate 时；None = JDK 模块 crate / lib crate 内的类
    pub user: Option<&'l UserLayout>,
    /// lib 模式定向；None = 单 crate 发射（无 --lib）
    pub lib: Option<LibSite<'l>>,
    /// 本类所在 crate 名
    pub here: &'l str,
    /// JDK 模块 crate 表
    pub crates: &'l ModuleCrates,
}

impl<'l> ClassSite<'l> {
    fn is_user(&self) -> bool {
        self.user.is_some()
    }

    /// lib crate 内的类：Java 可见性映射生效
    fn in_lib_crate(&self) -> bool {
        self.lib.is_some_and(|l| l.route.current.is_some())
    }

    /// 引用运行时基础设施（prelude / error …）的路径首段：JDK 模块 crate 内 `crate`（非根 crate 经
    /// lib.rs 的根门面 glob 解析），user / lib crate 写根门面名
    pub fn prefix(&self) -> &'l str {
        if self.is_user() || self.in_lib_crate() {
            self.crates.root()
        } else {
            "crate"
        }
    }

    /// 导入定向参数
    pub fn import_site(&self) -> ImportSite<'l> {
        ImportSite { cross: self.cross_input(), user: self.user }
    }

    /// 跨类引用参数（用户类在无 JDK 类时不导入生成类；lib 模式按生成集定向）
    fn cross_input(&self) -> CrossInput<'l> {
        if let Some(l) = self.lib {
            return CrossInput {
                generated: Some(l.generated),
                here: self.here,
                crates: self.crates,
                route: Some(l.route),
            };
        }
        let jdk_on = !self.is_user() || !self.jdk.files.is_empty();
        CrossInput {
            generated: jdk_on.then_some(&self.jdk.generated),
            here: self.here,
            crates: self.crates,
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

/// 类型存根：无任何方法在调用链上且类不在初始化集合（static 字段发 panic 存根访问器）；
/// 用户类与非用户类同一口径（[`input::EmitInput::type_only`]，与判定层 `ClassPlan::type_only` 同源）。
/// 引导映像给出静态字段初值的类除外：静态字段须有真实存储
pub fn is_type_only(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> bool {
    ctx.input.type_only(ci) && !has_image_statics(ctx, ci.name())
}

/// 引导映像给出静态字段初值的类：构建期初始化后 `<clinit>` 不入链，静态字段仍由启动序列写入、
/// 被闭包内的方法读取，须有真实存储（不是类型存根）
fn has_image_statics(ctx: &EmitCtx<'_>, cls: &str) -> bool {
    ctx.input.boot_image.statics.iter().any(|(c, _, _)| c == cls)
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
        .map(|(_, a)| render_arg_list(&a, &ctx.ty))
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
    out: &mut Vec<MethodBlock>,
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
    pub methods: Vec<EmittedMethod>,
}

/// 单类发射的前置事实（与发射序无关，可并行求得）：非 synthetic 方法
pub struct ClassPrep<'c> {
    visible: Vec<&'c classfile::Method>,
}

/// 前置事实：结构化引用集（含 user crate 兄弟类）在本文件作用域
/// 按 binary 序预认领（先于任何文本渲染）→ 派生名登记
pub fn class_prep<'c>(ctx: &EmitCtx<'c>, ci: &'c ClassInfo, site: &ClassSite<'_>) -> Result<ClassPrep<'c>> {
    let cross = site.cross_input();
    // 引用集只是 binary 集合，在无作用域视图上求得，不产生认领
    let plain = ctx.unscoped();
    if ctx.is_opaque(ci.name()) {
        // 不透明形态只引用全部传递超类型（upcast 目标）
        let referenced: BTreeSet<String> = opaque::opaque_supers(&plain, ci).into_iter().collect();
        claim_structural(ctx, ci, &cross, &referenced)?;
        return Ok(ClassPrep { visible: Vec::new() });
    }
    let mut referenced = collect_referenced(&plain, ci, cross.generated);
    ctx.module_audit.check(&plain, ci, &referenced);
    let visible: Vec<&classfile::Method> = ci.methods().iter().filter(|m| !m.is_synthetic()).collect();
    if let Some(user) = site.user {
        // 用户类兄弟按未过滤引用集（生成集过滤只作用于 JDK 引用）
        referenced.extend(user.sibling_set(&plain, ci.name(), &collect_referenced(&plain, ci, None)));
    }
    claim_structural(ctx, ci, &cross, &referenced)?;
    Ok(ClassPrep { visible })
}

/// 生成单类文件文本（串行形态：前置 → 类体）
pub fn gen_class_rs(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
) -> Result<ClassText> {
    let prep = class_prep(ctx, ci, site)?;
    class_text(ctx, state, bodies, ci, site, &prep)
}

/// 类体文本：只读共享上下文、只向 `state` 追加账本（可按类并行，
/// `state` 为本类增量，由调用方按发射序并入）
pub fn class_text(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    ci: &ClassInfo,
    site: &ClassSite<'_>,
    prep: &ClassPrep<'_>,
) -> Result<ClassText> {
    if ctx.is_opaque(ci.name()) {
        return Ok(opaque::opaque_text(ctx, state, ci, site));
    }
    let visible = &prep.visible;
    let is_iface = ci.is_interface();
    let mut parts: Vec<String> = vec![FILE_ALLOW.to_string(), format!("use {}::prelude::*;", site.prefix())];
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
    let mut method_blocks: Vec<MethodBlock> =
        fields::static_field_blocks(ctx, ci, &tps, is_type_only(ctx, ci)).into_iter().map(MethodBlock::plain).collect();
    let mb = methods::emit_method_blocks(ctx, state, bodies, ci, &tps)?;
    method_blocks.extend(mb.method_blocks);
    let iface_lambda_blocks = mb.iface_lambda_blocks;
    inherited_segments(ctx, state, bodies, ci, &tps, visible, &mut method_blocks)?;
    // 方法声明记录由各方法段生成点给出（与文本最终形态解耦）
    let methods: Vec<EmittedMethod> = method_blocks.iter().filter_map(|b| b.decl.clone()).collect();
    let mut method_blocks: Vec<String> = method_blocks.into_iter().map(|b| b.text).collect();
    if java_vis {
        visibility::downgrade_non_public_blocks(&mut method_blocks);
    }
    let struct_vis = if java_vis { visibility::java_member_vis(ci.class_file().access) } else { "pub" };

    let parent = parent_rust(ctx, ci);
    let empty = BTreeSet::new();
    let impl_methods = ctx.input.handwritten.get(ci.name()).map_or(&empty, |h| &h.methods);
    let field_slots: Vec<String> = sup.slots.iter().cloned().chain(fields::own_field_slots(ctx, ci)).collect();
    let head_in = head::HeadInput {
        superclass_rust: &parent,
        superclass_fields: &sup.fields,
        superclass_reference_fields: &sup.reference,
        superclass_erased_fields: &sup.erased,
        superclass_volatile_fields: &sup.volatile,
        field_slots: &field_slots,
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

    if !iface_lambda_blocks.is_empty() {
        let recv = if tps.is_empty() { sname.clone() } else { format!("{sname}<{}>", vec!["Object"; tps.len()].join(", ")) };
        parts.push("// G-10: 接口私有实例 lambda body —— 载体擦除实例化上的固有方法".into());
        parts.push(format!("impl {recv} {{"));
        parts.extend(iface_lambda_blocks.iter().map(|b| indent(b, "    ")));
        parts.push("}".into());
        parts.push(String::new());
    }

    Ok(ClassText { text: parts.join("\n"), methods })
}

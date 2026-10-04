//! `java_class! { ... }` 类块头（← `attrs._java_class_block_head` 及其辅助计算）。
//!
//! 两段：字节码元数据（缺省即默认，缺省键整行不写）与宏展开输入。

use std::collections::{BTreeSet, VecDeque};

use ty::class_params::parse_class_type_params;
use ty::consts::{OBJECT, STRING};
use ty::ident::safe_ident;
use ty::rs_type::render_arg_list;
use ty::ClassInfo;

use super::attrs::{access_str, anno_cpool_str, class_modifiers_str, q};
use crate::ctx::EmitCtx;
use crate::text::hex;

/// 类块头的调用方输入（class_writer 展平继承链得到）
#[derive(Debug, Default)]
pub struct HeadInput<'s> {
    pub superclass_rust: &'s str,
    pub superclass_fields: &'s [(String, String)],
    pub superclass_reference_fields: &'s [String],
    pub superclass_erased_fields: &'s [String],
    /// Rust 名与 Java 名不同的平铺实例字段（继承 + 本类）：`声明类.Java 名=Rust 名`
    pub field_slots: &'s [String],
    /// 共置手写 impl 提供的方法名
    pub impl_methods: Option<&'s BTreeSet<String>>,
}

fn rule(title: &str) -> String {
    format!("// ── {title} {}", "─".repeat(46))
}

/// 类修饰符（HotSpot `InstanceKlass::compute_modifier_flags`）：有 InnerClasses 自引用条目时取该条目的
/// inner_class_access_flags（含 private / protected / static，取代顶层 access），否则取类文件 access
fn effective_class_flags(ci: &ClassInfo) -> u16 {
    let cf = ci.class_file();
    cf.inner_classes.iter().find(|ic| ic.inner == cf.name).map_or(cf.access, |ic| ic.access)
}

/// 生成类块头行
pub fn block_head(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &HeadInput<'_>) -> Vec<String> {
    let mut lines = metadata_lines(ctx, ci);
    lines.push(String::new());
    lines.push(rule("宏展开输入"));
    macro_input_lines(ctx, ci, inp, &mut lines);
    lines
}

/// 不透明（L1）类的类级元数据行：类镜像仍是非 null 的 Class 对象，`getSimpleName` / `isMemberClass` /
/// `getDeclaringClass` / `isRecord` / `isAssignableFrom` 等经 java_meta 表读这些属性（与实例层级无关）。
/// 不含类初始化（L1 不初始化）。类级注解（`getAnnotation` / `getAnnotations` 经 `getRawAnnotations` 读注解表）
/// 同属类镜像元数据，两类 L1 照发：
/// - 用户类：其注解类型由分析器的注解补种（遍历全部用户类）带入闭包（只作类字面量的注解持有类）；
/// - 注解类型自身（用户与 JDK 同）：`AnnotationType` 经 `getRawClassAnnotations` 读其 `@Retention` /
///   `@Inherited` 判定保留策略——缺行即退为 CLASS，方法 / 字段上该注解被解析器丢弃（如
///   `Reflection.isCallerSensitive` 读不到 `@CallerSensitive`，反射调用 CS 方法不经注入调用器）。
/// 其余 JDK L1 类的注解类型不随其进入闭包，不发
pub fn opaque_metadata_lines(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    const ANNOS: [&str; 2] = ["#[raw_annotations", "#[anno_cpool"];
    let keep_annos = ctx.is_user(ci.name()) || ci.class_file().access & super::attrs::ACC_ANNOTATION != 0;
    let mut lines: Vec<String> = metadata_lines(ctx, ci)
        .into_iter()
        .filter(|l| {
            l.starts_with("#[")
                && !l.starts_with("#[has_clinit")
                && (keep_annos || !ANNOS.iter().any(|k| l.starts_with(k)))
        })
        .collect();
    lines.push(format!("#[all_supertypes    = \"{}\"]", all_supertypes(ctx, ci).join(";")));
    lines
}

fn metadata_lines(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let cf = ci.class_file();
    let ex = ctx.extras(ci.name());
    let mut lines = vec![rule("字节码元数据")];
    lines.push(format!("#[binary_name       = \"{}\"]", q(&cf.name)));
    if !ci.super_class().is_empty() {
        lines.push(format!("#[super_class       = \"{}\"]", q(ci.super_class())));
    }
    if !cf.interfaces.is_empty() {
        lines.push(format!("#[interfaces        = \"{}\"]", q(&cf.interfaces.join(","))));
    }
    if cf.access != 0 {
        let a = access_str(cf.access);
        if a != "package" {
            lines.push(format!("#[access            = \"{a}\"]"));
        }
        let mods = class_modifiers_str(effective_class_flags(ci));
        if !mods.is_empty() {
            lines.push(format!("#[modifiers         = \"{mods}\"]"));
        }
    }
    if !ci.generic_signature().is_empty() {
        lines.push(format!("#[generic_signature = \"{}\"]", q(ci.generic_signature())));
    }
    if cf.access & super::attrs::ACC_ABSTRACT != 0 {
        lines.push("#[is_abstract       = true]".into());
    }
    if cf.access & super::attrs::ACC_ENUM != 0 {
        lines.push("#[is_enum           = true]".into());
    }
    if cf.record_components.is_some() {
        lines.push("#[is_record         = true]".into());
        if !ex.record_components.is_empty() {
            let rc: Vec<String> = ex
                .record_components
                .iter()
                .map(|r| format!("{}:{}:{}", r.name, r.desc, r.signature))
                .collect();
            lines.push(format!("#[record_components = \"{}\"]", q(&rc.join("|"))));
        }
    }
    if !cf.permitted_subclasses.is_empty() {
        lines.push(format!("#[permitted_subclasses = \"{}\"]", q(&cf.permitted_subclasses.join(","))));
    }
    if !cf.nest_members.is_empty() {
        lines.push(format!("#[nest_members      = \"{}\"]", q(&cf.nest_members.join(","))));
    }
    // 类文件 access_flags 原值（JVM_ACC_WRITTEN_FLAGS 掩码内，含 ACC_SUPER / ACC_SYNTHETIC 等）：
    // Class.getClassAccessFlagsRaw0 的数据源（HotSpot JVM_GetClassAccessFlags 同源）
    lines.push(format!("#[class_access_flags = \"{}\"]", cf.access & 0x7FFF));
    // 定义加载器（VM 建镜像时写入 Class.classLoader）：镜像的读取钩子按 java_meta 汇总表填充；引导加载器不写
    if let Some(l) = ctx.defining_loader(&cf.name) {
        lines.push(format!("#[defining_loader   = \"{l}\"]"));
    }
    if ci.methods().iter().any(|m| m.name == "<clinit>") {
        lines.push("#[has_clinit        = true]".into());
    }
    if ex.deprecated {
        lines.push("#[is_deprecated     = true]".into());
    }
    if let Some(src) = cf.source_file.as_deref().filter(|s| !s.is_empty()) {
        lines.push(format!("#[source            = \"{}\"]", q(src)));
    }
    if !cf.inner_classes.is_empty() {
        let ics: Vec<String> = cf
            .inner_classes
            .iter()
            .map(|ic| {
                format!(
                    "{}:{}:{}:{}",
                    ic.inner,
                    ic.outer.as_deref().unwrap_or(""),
                    ic.simple_name.as_deref().unwrap_or(""),
                    ic.access
                )
            })
            .collect();
        lines.push(format!("#[inner_classes     = \"{}\"]", q(&ics.join(";"))));
    }
    if let Some((ec, em)) = cf.enclosing_method.as_ref().filter(|(c, _)| !c.is_empty()) {
        let (n, d) = em.as_ref().map_or(("", ""), |(n, d)| (n.as_str(), d.as_str()));
        lines.push(format!("#[enclosing_method  = \"{}:{}:{}\"]", q(ec), q(n), q(d)));
    }
    if !ex.raw_annotations.is_empty() {
        lines.push(format!("#[raw_annotations   = \"{}\"]", hex(&ex.raw_annotations)));
    }
    if !ex.anno_cpool.is_empty() {
        lines.push(format!("#[anno_cpool        = \"{}\"]", anno_cpool_str(&ex.anno_cpool)));
    }
    lines
}

fn macro_input_lines(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &HeadInput<'_>, lines: &mut Vec<String>) {
    if ci.is_interface() {
        lines.push("#[is_interface      = true]".into());
    }
    if !inp.superclass_rust.is_empty() {
        lines.push(format!("#[superclass        = \"{}\"]", inp.superclass_rust));
    }
    let init_ifaces = default_init_interfaces(ctx, ci);
    if !init_ifaces.is_empty() {
        lines.push(format!("#[init_interfaces   = \"{}\"]", init_ifaces.join(";")));
    }
    if !inp.superclass_fields.is_empty() {
        let items: Vec<String> = inp.superclass_fields.iter().map(|(n, t)| format!("{n}: {t}")).collect();
        lines.push(format!("#[superclass_fields({})]", items.join(", ")));
    }
    if !inp.superclass_reference_fields.is_empty() {
        lines.push(format!("#[superclass_reference_fields = \"{}\"]", inp.superclass_reference_fields.join(";")));
    }
    if !inp.superclass_erased_fields.is_empty() {
        lines.push(format!("#[superclass_erased_fields = \"{}\"]", inp.superclass_erased_fields.join(";")));
    }
    if !inp.field_slots.is_empty() {
        lines.push(format!("#[field_slots       = \"{}\"]", inp.field_slots.join(";")));
    }
    if ci.is_interface() {
        return;
    }
    let supers = all_superclasses(ctx, ci);
    if !supers.is_empty() {
        lines.push(format!("#[all_superclasses  = \"{}\"]", supers.join(";")));
    }
    let hooks = ancestor_hooks(ctx, ci);
    if !hooks.is_empty() {
        lines.push(format!("#[ancestor_hooks    = \"{}\"]", hooks.join(";")));
    }
    let layout = ancestor_fields_layout(ctx, ci);
    if !layout.is_empty() {
        let parts: Vec<String> = layout.iter().map(|(n, fs)| format!("{n}:{}", fs.join(","))).collect();
        lines.push(format!("#[ancestor_fields_layout = \"{}\"]", parts.join(";")));
    }
    let supertypes = all_supertypes(ctx, ci);
    lines.push(format!("#[all_supertypes    = \"{}\"]", supertypes.join(";")));
    let root_sigs = [
        ("to_string_vtable  ", "toString", format!("()L{STRING};")),
        ("hash_code_vtable  ", "hashCode", "()I".to_string()),
        ("equals_vtable     ", "equals", format!("(L{OBJECT};)Z")),
    ];
    for (key, name, desc) in &root_sigs {
        if let Some(owner) = root_method_vtable_owner(ctx, ci, name, desc) {
            lines.push(format!("#[{key}= \"{owner}\"]"));
        }
    }
    if let Some(im) = inp.impl_methods.filter(|s| !s.is_empty()) {
        lines.push(format!("#[impl_methods      = \"{}\"]", im.iter().cloned().collect::<Vec<_>>().join(";")));
    }
}

/// 类型文本 `Short<args>`（← `type_args.rust_type_with_args`）
fn with_args(short: &str, args: &str) -> String {
    format!("{short}{args}")
}

/// 线性超类链 Rust 类型（最深祖先在前；链出注册表时保留尾部裸短名）
pub fn all_superclasses(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let resolved = ctx.ty.ancestor_type_args(ci, None);
    let mut chain: Vec<String> = resolved
        .iter()
        .map(|(b, args)| with_args(&ctx.short(b), &render_arg_list(args, &ctx.ty)))
        .collect();
    let tail = match resolved.last() {
        Some((b, _)) => ctx.class(b).map_or("", |c| c.super_class()),
        None => ci.super_class(),
    };
    if !tail.is_empty() && tail != OBJECT {
        chain.push(ctx.short(tail));
    }
    chain.reverse();
    chain
}

/// 本文件以别名引用的祖先：`本地名=定义名`。祖先 vtable 的 `__as_<名>` 钩子按祖先定义名
/// 声明，宏据此取钩子名（类型名仍用本地名）
fn ancestor_hooks(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let mut bins: Vec<String> = superclass_chain(ctx, ci).iter().map(|c| c.name().to_string()).collect();
    let tail = bins.last().map_or(ci.super_class(), |b| ctx.class(b).map_or("", |c| c.super_class())).to_string();
    if !tail.is_empty() && tail != OBJECT {
        bins.push(tail);
    }
    bins.iter()
        .filter_map(|b| {
            let (local, declared) = (ctx.short(b), ctx.declared(b));
            (local != declared).then(|| format!("{local}={declared}"))
        })
        .collect()
}

/// 超类链上的祖先类名（直接父类在前；止于根类 / 注册表外 / 环）
pub fn superclass_chain<'a>(ctx: &EmitCtx<'a>, ci: &ClassInfo) -> Vec<&'a ClassInfo> {
    let mut chain = Vec::new();
    let mut seen = BTreeSet::new();
    let mut cur = ci.super_class().to_string();
    while !cur.is_empty() && cur != OBJECT && seen.insert(cur.clone()) {
        let Some(c) = ctx.class(&cur) else { break };
        chain.push(c);
        cur = c.super_class().to_string();
    }
    chain
}

/// 各祖先自己声明的非静态字段（最深祖先在前；无字段的祖先省略）
fn ancestor_fields_layout(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<(String, Vec<String>)> {
    let mut declared = BTreeSet::new();
    let mut out = Vec::new();
    for anc in superclass_chain(ctx, ci).into_iter().rev() {
        let own: Vec<String> = anc
            .fields()
            .iter()
            .filter(|f| !f.is_static())
            .map(|f| ctx.ty.instance_field_rust_name(anc.name(), &safe_ident(&f.name)))
            .filter(|n| declared.insert(n.clone()))
            .collect();
        if !own.is_empty() {
            out.push((ctx.short(anc.name()), own));
        }
    }
    out
}

/// 全部超类型（含自身，binary 名，排序）
pub fn all_supertypes(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let mut out: BTreeSet<String> = BTreeSet::from([ci.name().to_string()]);
    let mut q: VecDeque<String> = VecDeque::new();
    if !ci.super_class().is_empty() {
        q.push_back(ci.super_class().to_string());
    }
    q.extend(ci.interfaces().iter().cloned());
    let mut visited = BTreeSet::new();
    while let Some(n) = q.pop_front() {
        if !visited.insert(n.clone()) {
            continue;
        }
        if let Some(p) = ctx.class(&n) {
            if !p.super_class().is_empty() {
                q.push_back(p.super_class().to_string());
            }
            q.extend(p.interfaces().iter().cloned());
        }
        out.insert(n);
    }
    out.into_iter().collect()
}

/// 类初始化随之初始化的超接口（有 default 方法且有 `<clinit>`；后序；泛型实参取 Object）
fn default_init_interfaces(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    fn visit(ctx: &EmitCtx<'_>, name: &str, seen: &mut BTreeSet<String>, out: &mut Vec<String>) {
        if !seen.insert(name.to_string()) {
            return;
        }
        let Some(ic) = ctx.class(name) else { return };
        for sup in ic.interfaces() {
            visit(ctx, sup, seen, out);
        }
        let has_default = ic
            .methods()
            .iter()
            .any(|m| !m.is_abstract() && !m.is_static() && m.name != "<init>" && m.name != "<clinit>");
        let has_clinit = ic.methods().iter().any(|m| m.name == "<clinit>");
        if has_default && has_clinit {
            let n = parse_class_type_params(ic.generic_signature()).len();
            let args = if n > 0 { format!("<{}>", vec!["Object"; n].join(", ")) } else { String::new() };
            out.push(with_args(&ctx.short(name), &args));
        }
    }
    let mut out = Vec::new();
    if ci.is_interface() {
        return out;
    }
    let mut seen = BTreeSet::new();
    for i in ci.interfaces() {
        visit(ctx, i, &mut seen, &mut out);
    }
    out
}

/// 根类虚方法（toString / hashCode / equals）在本类视角的 vtable 所属类 Rust 名
fn root_method_vtable_owner(ctx: &EmitCtx<'_>, ci: &ClassInfo, name: &str, desc: &str) -> Option<String> {
    let mut seen = BTreeSet::new();
    let mut cur = Some(ci);
    while let Some(c) = cur {
        if !seen.insert(c.name().to_string()) {
            break;
        }
        if let Some(decl) = c.methods().iter().find(|m| m.name == name && m.desc == desc && !m.is_static()) {
            let hw = ctx.input.handwritten.get(c.name()).is_some_and(|h| h.methods.contains(&safe_ident(&decl.name)));
            if hw || ctx.member_rust_name(c, decl) != decl.name {
                return None;
            }
            return Some(ctx.find_virtual_in(decl, c)).filter(|s| !s.is_empty()).map(|b| ctx.short(&b));
        }
        let sc = c.super_class();
        cur = if !sc.is_empty() && sc != OBJECT { ctx.class(sc) } else { None };
    }
    None
}

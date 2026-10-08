//! 本类文件需引用的类型集合（← `import_gen.collect_referenced`）。
//!
//! 指令扫描面是折叠后的规范化方法体（[`input::EmitInput::code`]）：Python 的指令注释
//! 在此按结构化操作数直接取（`Method` / `Field` 引用、invokedynamic 实现方法句柄、
//! 类操作数、类字面量），语义与注释文本解析一致。

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use classfile::{Const, ExceptionEntry, Insn, Method, Operand};
use input::manifest::IndyKind;
use instr::owner::{resolve_static_field_owner, resolve_static_method_owner};
use ty::ClassInfo;

use super::refs::{add_desc_refs, add_narrow_refs, desc_tokens, strip_generic, superclass_arg_refs};
use crate::ctx::EmitCtx;
use crate::lang;

/// 参与引用扫描的方法（声明类 + 在声明类方法表中的下标）
#[derive(Clone, Copy)]
pub struct ScanMethod<'a> {
    pub owner: &'a ClassInfo,
    pub index: usize,
    pub method: &'a Method,
}

fn scan_all<'a>(owner: &'a ClassInfo) -> impl Iterator<Item = ScanMethod<'a>> {
    owner.methods().iter().enumerate().map(move |(index, method)| ScanMethod { owner, index, method })
}

/// 扫描面：自身方法 + 注入本类的接口 default 方法 + 用户类超类链的方法（签名全量扫描；
/// 方法体只扫调用链上的，见 [`collect_referenced`]）
fn scan_methods<'a>(ctx: &EmitCtx<'a>, ci: &'a ClassInfo) -> Vec<ScanMethod<'a>> {
    let reg = ctx.ty.reg;
    let mut out: Vec<ScanMethod<'a>> = scan_all(ci).collect();
    if ci.is_interface() {
        return out;
    }
    let mut q: VecDeque<&str> = ci.interfaces().iter().map(String::as_str).collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    while let Some(n) = q.pop_front() {
        if !seen.insert(n) {
            continue;
        }
        let Some(ici) = reg.get(n) else { continue };
        q.extend(ici.interfaces().iter().map(String::as_str));
        out.extend(scan_all(ici).filter(|s| !s.method.is_abstract() && !s.method.is_static()));
    }
    let mut sup = ci.super_class();
    while !sup.is_empty() && sup != lang::OBJECT && !sup.contains('/') {
        let Some(sci) = reg.get(sup) else { break };
        out.extend(scan_all(sci));
        sup = sci.super_class();
    }
    out
}

/// invokedynamic 的实现方法句柄（lambda 类引导、≥2 个静态实参、第 2 个为方法句柄）
pub fn indy_impl_ref<'c>(ctx: &EmitCtx<'_>, owner: &'c ClassInfo, bsm: u16) -> Option<&'c classfile::MemberRef> {
    let b = owner.class_file().bootstrap_methods.get(usize::from(bsm))?;
    let key = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
    if ctx.manifest.indy_kind(&key) != Some(IndyKind::Lambda) || b.args.len() < 2 {
        return None;
    }
    match &b.args[1] {
        Const::MethodHandle(h) => Some(&h.member),
        _ => None,
    }
}

const GETSTATIC: u8 = 0xb2;
const PUTSTATIC: u8 = 0xb3;
const INVOKESTATIC: u8 = 0xb8;

/// 拼接 / ObjectMethods 调用点发射的清单分量处理入口（`[indy] concat_stringify` / `component_hash` /
/// `component_equals`）：生成器在调用点静态调用它们，声明类与描述符类型须在作用域内
fn indy_helper_refs(ctx: &EmitCtx<'_>, owner: &ClassInfo, bsm: u16, out: &mut BTreeSet<String>) {
    let Some(b) = owner.class_file().bootstrap_methods.get(usize::from(bsm)) else { return };
    let key = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
    let keys: &[&str] = match ctx.manifest.indy_kind(&key) {
        Some(IndyKind::Concat) => &["concat_stringify"],
        Some(IndyKind::ObjectMethods) => &["concat_stringify", "component_hash", "component_equals"],
        _ => &[],
    };
    for k in keys {
        if let Ok((o, _, d)) = ctx.manifest.indy_helper(k) {
            out.insert(o.to_string());
            add_desc_refs(d, out);
        }
    }
}

/// 单条指令贡献的引用
fn scan_insn(ctx: &EmitCtx<'_>, owner: &ClassInfo, insn: &Insn, out: &mut BTreeSet<String>) {
    match &insn.operand {
        Operand::Field(r) | Operand::Method(r, _) => {
            out.insert(r.owner.clone());
            add_desc_refs(&r.desc, out);
            // 常量池类可以是子类：static 成员的访问点落在实际声明类上（与 instr 的
            // getstatic / putstatic / invokestatic 解析同源），声明类须在作用域内
            let reg = ctx.ty.reg;
            let declared = match insn.opcode {
                GETSTATIC | PUTSTATIC => resolve_static_field_owner(reg, &r.owner, &r.name),
                INVOKESTATIC => resolve_static_method_owner(reg, &r.owner, &r.name, &r.desc),
                _ => None,
            };
            if let Some(d) = declared.filter(|d| *d != r.owner) {
                out.insert(d);
            }
        }
        Operand::InvokeDynamic { bsm, .. } => {
            indy_helper_refs(ctx, owner, *bsm, out);
            let Some(r) = indy_impl_ref(ctx, owner, *bsm) else { return };
            out.insert(r.owner.clone());
            add_desc_refs(&r.desc, out);
            for tok in desc_tokens(&r.desc) {
                if let Some(b) = lang::boxed_class(tok) {
                    out.insert(b.to_string());
                }
            }
        }
        Operand::Class(c) | Operand::MultiANewArray(c, _) => {
            if c.starts_with('[') {
                add_desc_refs(c, out);
            } else {
                let c = strip_generic(c);
                out.insert(c.to_string());
                if let Some(op_ci) = ctx.ty.reg.get(c) {
                    out.extend(superclass_arg_refs(ctx, op_ci));
                }
            }
        }
        Operand::Ldc(Const::Class(_)) => {
            out.insert(lang::CLASS.to_string());
        }
        _ => {}
    }
}

/// multi-catch：各处理器 ≥2 个 catch 类型时取父类链的最近公共祖先
fn multi_catch_refs(ctx: &EmitCtx<'_>, table: &[ExceptionEntry], out: &mut BTreeSet<String>) {
    let mut by_handler: BTreeMap<u32, Vec<&str>> = BTreeMap::new();
    for e in table {
        match &e.catch_type {
            Some(t) => {
                out.insert(t.clone());
                let v = by_handler.entry(e.handler).or_default();
                if !v.contains(&t.as_str()) {
                    v.push(t);
                }
            }
            None => {
                out.insert(lang::THROWABLE.to_string());
            }
        }
    }
    for types in by_handler.values().filter(|v| v.len() >= 2) {
        let mut common: Option<Vec<&str>> = None;
        for ct in types {
            let mut chain: Vec<&str> = Vec::new();
            let mut cc: &str = ct;
            while !cc.is_empty() && cc != lang::OBJECT && !chain.contains(&cc) {
                chain.push(cc);
                cc = ctx.ty.reg.get(cc).map_or("", ClassInfo::super_class);
            }
            common = Some(match common {
                None => chain,
                Some(c) => c.into_iter().filter(|x| chain.contains(x)).collect(),
            });
        }
        if let Some(first) = common.and_then(|c| c.first().copied()) {
            out.insert(first.to_string());
        }
    }
}

/// 方法体（指令 / 局部变量表 / 异常表）引用
fn scan_method_body(ctx: &EmitCtx<'_>, s: ScanMethod<'_>, out: &mut BTreeSet<String>) {
    if let Some(code) = ctx.input.code_ops(s.owner.name(), s.method) {
        for insn in code.ops() {
            scan_insn(ctx, s.owner, insn, out);
        }
        let ex = ctx.extras(s.owner.name());
        if let Some(mx) = ex.methods.get(s.index) {
            for lv in &mx.local_vars {
                add_desc_refs(&lv.desc, out);
                add_desc_refs(&lv.signature, out);
            }
        }
        multi_catch_refs(ctx, code.exception_table(), out);
    }
}

/// 分支汇合值的公共超类（`method::unify` 取两臂超类链上的最近公共类）只出现在合流文本里，
/// 字节码不直接引用：已引用类的超类链全部入集
fn add_join_supers(ctx: &EmitCtx<'_>, out: &mut BTreeSet<String>) {
    let reg = ctx.ty.reg;
    let mut extra: Vec<String> = Vec::new();
    for c in out.iter() {
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut cur = reg.get(c.as_str()).map_or("", |ci| ci.super_class());
        while !cur.is_empty() && cur != lang::OBJECT && seen.insert(cur) {
            extra.push(cur.to_string());
            cur = reg.get(cur).map_or("", |ci| ci.super_class());
        }
    }
    out.extend(extra);
}

/// 字段描述符 / 签名（本类全部字段 + 祖先未遮蔽的实例字段）
fn scan_fields(ctx: &EmitCtx<'_>, ci: &ClassInfo, out: &mut BTreeSet<String>) {
    // 祖先字段不按名字去重：本类同名字段隐藏祖先字段时，祖先字段仍以独立槽位名进入
    // superclass_fields 宏属性，其类型须在作用域内
    let mut fields: Vec<&classfile::Field> = ci.fields().iter().collect();
    let mut seen_classes: BTreeSet<&str> = BTreeSet::new();
    let mut sup = ci.super_class();
    while !sup.is_empty() && sup != lang::OBJECT && seen_classes.insert(sup) {
        let Some(sci) = ctx.ty.reg.get(sup) else { break };
        fields.extend(sci.fields().iter().filter(|f| !f.is_static()));
        sup = sci.super_class();
    }
    for f in fields {
        add_narrow_refs(&f.desc, out);
        add_desc_refs(f.signature.as_deref().unwrap_or(""), out);
    }
}

/// 分派子类型（BFS，兄弟按 binary 排序，叶节点优先）
pub fn subtypes_ordered(ctx: &EmitCtx<'_>, root: &str) -> Vec<String> {
    let children = ctx.subtype_children();
    let mut all = Vec::new();
    let mut visited: BTreeSet<&str> = BTreeSet::from([root]);
    let mut q: VecDeque<&str> = VecDeque::from([root]);
    while let Some(cur) = q.pop_front() {
        for c in children.get(cur).into_iter().flatten() {
            if visited.insert(c) {
                all.push(c.clone());
                q.push_back(c);
            }
        }
    }
    all.reverse();
    all
}

/// 虚分派链 downcast 目标子类型（JDK 命名空间接收者只收 JDK 命名空间子类型）。
///
/// 只收本类所在模块可读的子类型（本模块及其 requires 闭包，[`resolve::ModuleGraph::reads`]）：
/// 下游模块的子类型不会出现在本类正文里，收进来只会形成反向 / 互不可达的模块边
/// （T1 第 2 步 R1）；无名模块（用户类）读全部模块，不受影响
fn dispatch_subtype_refs(ctx: &EmitCtx<'_>, ci: &ClassInfo, methods: &[ScanMethod<'_>], out: &mut BTreeSet<String>) {
    let modules = ctx.modules();
    let here = modules.module_of(ci.name());
    let mut owners: BTreeSet<&str> = BTreeSet::new();
    for s in methods {
        let Some(code) = ctx.input.code_ops(s.owner.name(), s.method) else { continue };
        for insn in code.ops() {
            if let Insn { operand: Operand::Method(r, _), .. } = insn {
                if let Some(ci) = ctx.ty.reg.get(&r.owner) {
                    owners.insert(ci.name());
                }
            }
        }
    }
    for owner in owners {
        let jdk = lang::in_jdk_namespace(owner);
        for sub in subtypes_ordered(ctx, owner) {
            let sub = strip_generic(&sub);
            if (!jdk || lang::in_jdk_namespace(sub)) && modules.reads(here, modules.module_of(sub)) {
                out.insert(sub.to_string());
            }
        }
    }
}

/// `collect_referenced`：`generated` 为 None 时不做生成集过滤
pub fn collect_referenced(ctx: &EmitCtx<'_>, ci: &ClassInfo, generated: Option<&BTreeSet<String>>) -> BTreeSet<String> {
    let reg = ctx.ty.reg;
    let mut out = BTreeSet::new();
    let mut sup = ci.super_class();
    while !sup.is_empty() && sup != lang::OBJECT {
        out.insert(sup.to_string());
        match reg.get(sup) {
            Some(s) => sup = s.super_class(),
            None => break,
        }
    }
    out.extend(ci.interfaces().iter().filter(|i| *i != lang::OBJECT).cloned());
    out.extend(ctx.ty.class_type_param_bounds(ci).into_values().map(|(_, b)| b));
    let methods = scan_methods(ctx, ci);
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut cur = Some(ci);
    while let Some(c) = cur.filter(|c| seen.insert(c.name())) {
        add_desc_refs(c.generic_signature(), &mut out);
        cur = reg.get(c.super_class());
    }
    // 方法体扫描面与方法体发射同一口径：本类方法按自身键、注入的接口 default / 祖先方法按声明者键或本类符号键
    // （inherit / super_inherit 的 in_cc）；链外方法只发存根，其体内引用的类型不进引用集
    let live: Vec<ScanMethod<'_>> = methods
        .iter()
        .copied()
        .filter(|s| ctx.in_chain(s.owner.name(), &s.method.name, &s.method.desc) || ctx.in_chain(ci.name(), &s.method.name, &s.method.desc))
        .collect();
    for s in &live {
        scan_method_body(ctx, *s, &mut out);
    }
    add_join_supers(ctx, &mut out);
    scan_fields(ctx, ci, &mut out);
    for s in &methods {
        add_narrow_refs(&s.method.desc, &mut out);
        add_desc_refs(s.method.signature.as_deref().unwrap_or(""), &mut out);
    }
    dispatch_subtype_refs(ctx, ci, &live, &mut out);
    match generated {
        Some(g) => out.into_iter().filter(|c| g.contains(c)).collect(),
        None => out,
    }
}

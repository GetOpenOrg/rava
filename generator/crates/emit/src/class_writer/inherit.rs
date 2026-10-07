//! 接口方法展开到实现类（← `class_writer._emit_interface_default_inheritance` /
//! `_emit_interface_special_members` / `_adapt_interface_method`）。
//!
//! - **default 继承**：类实现接口而未覆盖其 default 方法时，把 default 方法体展开到本类
//!   （`virtual_in` = 本类，VirtualDefine；祖先类已由别的接口 default 注入同一槽位时覆盖该祖先
//!   槽位）；祖先类已实现的接口不重复注入（E0034）。
//! - **`Iface.super.m()` / 接口私有方法**：invokespecial InterfaceMethod 的落点以非虚成员
//!   `Iface_super_m` 展开到本类；展开出的方法体自身的同类调用递归处理。

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use classfile::{acc, op, Method, Operand};
use ty::ident::safe_ident;
use ty::sig_parse::substitute_signature_type_vars;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use super::attrs::MethodAttrExtra;
use super::methods::{slot_extra, BodySpec, Cx, Emitted};
use crate::body::MethodBodyEmitter;
use crate::ctx::{EmitCtx, ProjectState};
use crate::emission::MethodBlock;
use crate::error::{EmitError, Result};
use crate::vtable::param_part;

fn is_ctor_name(n: &str) -> bool {
    n == "<init>" || n == "<clinit>"
}

/// 接口方法展开到实现类：泛型签名里的接口类型变量换成实现类视角的类型实参。
/// 返回改写后的方法与代换表（方法体侧据此代换局部变量表签名）
pub(super) fn adapt_interface_method(
    ctx: &EmitCtx<'_>,
    ci: &ClassInfo,
    iface: &str,
    m: &Method,
) -> Result<(Method, Option<BTreeMap<String, String>>)> {
    let mut adapted = m.clone();
    let view = ctx.ty.interface_signature_views(ci).into_iter().find(|(n, _)| n == iface).map(|(_, v)| v);
    let Some(view) = view.filter(|v| !v.is_empty()) else { return Ok((adapted, None)) };
    if let Some(sig) = &m.signature {
        let sub = substitute_signature_type_vars(sig, &view)
            .ok_or_else(|| EmitError::Input(format!("残缺泛型签名：{iface}.{}{}：{sig}", m.name, m.desc)))?;
        adapted.signature = Some(sub);
    }
    Ok((adapted, Some(view)))
}

/// 类是否按全量翻译（用户类；Python `call_chain is None`）
pub(super) fn chain_all(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> bool {
    ctx.is_user(ci.name())
}

/// 接口方法展开到实现类时，字节码出处接口 `owner` 的方法体是否免调用链门控全量翻译：
/// 全量翻译只覆盖用户字节码（实现类与出处接口都是用户类）。JDK 接口的 default / 私有方法
/// 展开到用户类时仍是 JDK 字节码，按调用链判定——不在链上即存根：分析器不分析链外方法的
/// 内部依赖（引用类、lambda 函数式接口），翻译其体会引用闭包外的类与未合成的 `I__Lambda`
fn iface_body_all(ctx: &EmitCtx<'_>, ci: &ClassInfo, owner: &ClassInfo) -> bool {
    chain_all(ctx, ci) && ctx.is_user(owner.name())
}

fn method_index(ci: &ClassInfo, m: &Method) -> usize {
    ci.methods().iter().position(|x| std::ptr::eq(x, m)).unwrap_or(0)
}

/// 祖先类（不含本类）实现的全部接口（传递闭包）
fn ancestor_interfaces(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> BTreeSet<String> {
    let reg = ctx.ty.reg;
    let mut out = BTreeSet::new();
    let mut seen = BTreeSet::new();
    let mut cur = ci.super_class();
    while let Some(c) = reg.get(cur).filter(|_| !cur.is_empty() && seen.insert(cur.to_string())) {
        let mut q: VecDeque<String> = c.interfaces().iter().cloned().collect();
        while let Some(n) = q.pop_front() {
            if !out.insert(n.clone()) {
                continue;
            }
            if let Some(ic) = reg.get(&n) {
                q.extend(ic.interfaces().iter().cloned());
            }
        }
        cur = c.super_class();
    }
    out
}

/// 接口广度遍历（跳过 `visited` 初值），逐接口回调
fn walk_interfaces<'c>(ctx: &EmitCtx<'c>, ci: &ClassInfo, mut visited: BTreeSet<String>, mut f: impl FnMut(&'c ClassInfo)) {
    let mut q: VecDeque<String> = ci.interfaces().iter().cloned().collect();
    while let Some(n) = q.pop_front() {
        if !visited.insert(n.clone()) {
            continue;
        }
        let Some(ici) = ctx.ty.reg.get(&n) else { continue };
        q.extend(ici.interfaces().iter().cloned());
        f(ici);
    }
}

/// 实例桥方法（ACC_BRIDGE，非静态）
fn is_bridge(m: &Method) -> bool {
    m.access & acc::BRIDGE != 0 && !m.is_static()
}

fn is_inheritable_default(m: &Method) -> bool {
    !m.is_abstract() && !m.is_static() && !m.is_synthetic() && m.access & acc::PRIVATE == 0 && !is_ctor_name(&m.name)
}

/// 接口 default 方法继承；返回已翻译体的 (声明接口, 方法)（`Iface.super` 展开段的种子）
pub(super) fn interface_default_inheritance<'c>(
    cx: &Cx<'_, 'c>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    visible: &[&Method],
    out: &mut Vec<MethodBlock>,
) -> Result<Vec<(&'c ClassInfo, &'c Method)>> {
    let (ctx, ci) = (cx.ctx, cx.ci);
    let mut translated = Vec::new();
    if ci.interfaces().is_empty() || ci.is_interface() {
        return Ok(translated);
    }
    let mut existing: BTreeSet<(String, String)> = visible.iter().map(|m| (m.name.clone(), m.desc.clone())).collect();
    let mut existing_pp: BTreeSet<(String, String)> =
        visible.iter().map(|m| (m.name.clone(), param_part(&m.desc).to_string())).collect();
    // 桥方法与 default 同名同描述符即构成覆盖（JVMS §5.4.6 类方法优先于接口 default）：
    // 该槽位经桥转发到被桥接的类方法，不注入 default 体
    for b in ci.methods().iter().filter(|m| is_bridge(m)) {
        existing.insert((b.name.clone(), b.desc.clone()));
    }
    // 覆盖判定沿父类链（祖先类自身的非私有实例方法同样构成覆盖）
    let mut seen = BTreeSet::new();
    let mut cur = ci.super_class();
    while let Some(sci) = ctx.ty.reg.get(cur).filter(|_| !cur.is_empty() && seen.insert(cur.to_string())) {
        for sm in sci.methods() {
            if is_bridge(sm) {
                existing.insert((sm.name.clone(), sm.desc.clone()));
                continue;
            }
            if sm.is_static() || sm.is_synthetic() || is_ctor_name(&sm.name) || sm.access & acc::PRIVATE != 0 {
                continue;
            }
            existing.insert((sm.name.clone(), sm.desc.clone()));
            existing_pp.insert((sm.name.clone(), param_part(&sm.desc).to_string()));
        }
        cur = sci.super_class();
    }
    let mt = &ctx.manifest.ty;
    let mut used_rust: BTreeSet<String> = visible
        .iter()
        .filter(|m| !is_ctor_name(&m.name))
        .map(|m| if cx.overloaded.contains(&m.name) { mangle_name(mt, &m.name, &m.desc) } else { m.name.clone() })
        .collect();
    let anc = ancestor_interfaces(ctx, ci);
    // 预扫描：待继承 default 方法按 (名, 参数签名) 去重计数（default 之间重名则 mangle）
    let mut counts: BTreeMap<String, u32> = BTreeMap::new();
    let mut counted: BTreeSet<(String, String)> = BTreeSet::new();
    walk_interfaces(ctx, ci, anc.clone(), |ici| {
        for dm in ici.methods().iter().filter(|m| is_inheritable_default(m)) {
            let pp = (dm.name.clone(), param_part(&dm.desc).to_string());
            if !existing.contains(&(dm.name.clone(), dm.desc.clone())) && !existing_pp.contains(&pp) && counted.insert(pp) {
                *counts.entry(dm.name.clone()).or_default() += 1;
            }
        }
    });
    let mut pending: Vec<(&'c ClassInfo, &'c Method)> = Vec::new();
    walk_interfaces(ctx, ci, anc, |ici| {
        for dm in ici.methods().iter().filter(|m| is_inheritable_default(m)) {
            let pp = (dm.name.clone(), param_part(&dm.desc).to_string());
            if existing.contains(&(dm.name.clone(), dm.desc.clone())) || existing_pp.contains(&pp) {
                continue;
            }
            existing.insert((dm.name.clone(), dm.desc.clone()));
            existing_pp.insert(pp);
            pending.push((ici, dm));
        }
    });
    for (ici, dm) in pending {
        let mangle = cx.overloaded.contains(&dm.name)
            || used_rust.contains(&dm.name)
            || counts.get(&dm.name).copied().unwrap_or(0) > 1;
        let rust = if mangle { mangle_name(mt, &dm.name, &dm.desc) } else { dm.name.clone() };
        used_rust.insert(rust.clone());
        let (adapted, view) = adapt_interface_method(ctx, ci, ici.name(), dm)?;
        let e = Emitted { method: Cow::Owned(adapted), owner: ici, index: method_index(ici, dm) };
        // 槽位归属：祖先类已经由（别的接口的）default 注入同一槽位时覆盖那一槽位（JVM 选最具体
        // default 的结果须经祖先 vtable 派发）；否则本类新开槽位
        let mut extra = slot_extra(cx, &e.method, &rust);
        extra.declared_by = ici.name().to_string();
        let in_cc = iface_body_all(ctx, ci, ici) || ctx.in_chain(ci.name(), &dm.name, &dm.desc) || ctx.in_chain(ici.name(), &dm.name, &dm.desc);
        let mut text = None;
        if in_cc {
            let spec = BodySpec { ctparams: cx.tps, rust_name: Some(&rust), in_vtable_body: true, view: view.as_ref(), site: "iface-inherit" };
            text = cx.body_with(state, bodies, &e, &spec)?;
            if text.is_some() {
                translated.push((ici, dm));
            }
        }
        out.push(cx.body_block(&e, &extra, text, &rust, cx.tps));
    }
    Ok(translated)
}

/// invokespecial InterfaceMethod 的方法体声明者：常量池类为 registry 接口时自身优先、
/// 广度遍历父接口找最近的有体声明（描述符精确匹配）
pub(super) fn resolve_interface_special_target<'c>(ctx: &EmitCtx<'c>, iface: &str, name: &str, desc: &str) -> Option<&'c ClassInfo> {
    if !ctx.ty.reg.get(iface).is_some_and(ClassInfo::is_interface) {
        return None;
    }
    let mut q = VecDeque::from([iface.to_string()]);
    let mut seen = BTreeSet::new();
    while let Some(cur) = q.pop_front() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        let Some(c) = ctx.ty.reg.get(&cur) else { continue };
        if c.methods().iter().any(|m| !m.is_static() && !m.is_abstract() && m.name == name && m.desc == desc) {
            return Some(c);
        }
        q.extend(c.interfaces().iter().cloned());
    }
    None
}

/// `Iface.super.m(...)` 在实现类中的落点成员名 `Iface_super_m`（m 在接口内重载时带描述符后缀）
pub(crate) fn interface_special_member_name(ctx: &EmitCtx<'_>, owner: &ClassInfo, name: &str, desc: &str) -> String {
    let m = if ctx.ty.hierarchy_overloaded_names(owner).contains(name) {
        mangle_name(&ctx.manifest.ty, name, desc)
    } else {
        name.to_string()
    };
    format!("{}_super_{m}", ctx.short(owner.name()))
}

/// 方法体中 invokespecial（非构造器）的方法引用 (常量池类, 名, 描述符)
fn special_refs(ctx: &EmitCtx<'_>, owner: &ClassInfo, m: &Method) -> Vec<(String, String, String)> {
    let Some(code) = ctx.input.code_ops(owner.name(), m) else { return Vec::new() };
    code.ops()
        .filter(|i| i.opcode == op::INVOKESPECIAL)
        .filter_map(|i| match &i.operand {
            Operand::Method(r, _) if r.name != "<init>" => Some((r.owner.clone(), r.name.clone(), r.desc.clone())),
            _ => None,
        })
        .collect()
}

/// `Iface.super.m()` / 接口私有方法的非虚展开成员
pub(super) fn interface_special_members<'c>(
    cx: &Cx<'_, 'c>,
    state: &mut ProjectState,
    bodies: &dyn MethodBodyEmitter,
    visible: &[(&'c ClassInfo, &Method)],
    translated: Vec<(&'c ClassInfo, &'c Method)>,
    out: &mut Vec<MethodBlock>,
) -> Result<()> {
    let (ctx, ci) = (cx.ctx, cx.ci);
    if ci.is_interface() {
        return Ok(());
    }
    let all = chain_all(ctx, ci);
    let mut sources: VecDeque<(&ClassInfo, Cow<'_, Method>)> = visible
        .iter()
        .filter(|(_, m)| all || ctx.in_chain(ci.name(), &m.name, &m.desc))
        .map(|(o, m)| (*o, Cow::Borrowed(*m)))
        .collect();
    sources.extend(translated.into_iter().map(|(o, m)| (o, Cow::Borrowed(m))));
    let mut done: BTreeSet<(String, String, String)> = BTreeSet::new();
    while let Some((src_owner, src)) = sources.pop_front() {
        for (cls, name, desc) in special_refs(ctx, src_owner, &src) {
            let Some(owner) = resolve_interface_special_target(ctx, &cls, &name, &desc) else { continue };
            if !done.insert((owner.name().to_string(), name.clone(), desc.clone())) {
                continue;
            }
            let Some(idx) = owner
                .methods()
                .iter()
                .position(|m| m.name == name && m.desc == desc && !m.is_static() && !m.is_abstract())
            else {
                continue;
            };
            let sp_m = &owner.methods()[idx];
            let rust = safe_ident(&interface_special_member_name(ctx, owner, &name, &desc));
            let (adapted, view) = adapt_interface_method(ctx, ci, owner.name(), sp_m)?;
            let e = Emitted { method: Cow::Owned(adapted), owner, index: idx };
            let extra = MethodAttrExtra::default();
            let mut text = None;
            if iface_body_all(ctx, ci, owner) || ctx.in_chain(owner.name(), &name, &desc) {
                let spec = BodySpec { ctparams: cx.tps, rust_name: Some(&rust), in_vtable_body: false, view: view.as_ref(), site: "iface-special" };
                text = cx.body_with(state, bodies, &e, &spec)?;
                if text.is_some() {
                    sources.push_back((owner, Cow::Borrowed(sp_m)));
                }
            }
            out.push(cx.body_block(&e, &extra, text, &rust, cx.tps));
        }
    }
    Ok(())
}

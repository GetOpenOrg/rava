//! 接口实现声明与接口接收者的继承成员（← `emitter/interface_gen.py`）。
//!
//! 具体类对它实现的每个接口（含经超类 / 超接口传递得到的）输出擦除签名的
//! `impl Iface for Class { fn m(&self, ..) -> Result<..>; }`，宏据此为 `Class__inner` 实现
//! `Iface__VTable`；实现方法的定位完全来自定义侧记录：本类声明 → 共置 `_impl.rs` 成员 →
//! 超类链最近声明（同时登记继承成员需求）→ 桥接方法指向的真实方法。定位不到的方法不输出，
//! `Iface__VTable` 的缺省体 `panic!("stub: ...")` 精确报出缺口。

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::OnceLock;

use classfile::acc;
use indexmap::{IndexMap, IndexSet};
use regex::Regex;
use ty::{ClassInfo, RsType};

use super::bridge::{resolve_bridge_member, resolve_bridge_target};
use super::sig::{idents, param_mapping, param_part, rust_type, rust_type_head, split_top_level_trimmed, substitute_type_params};
use super::uses::claim_classes;
use super::{class_params, fill_slot, provided_methods, render_args, requests_by_recv, substituted_classes, type_classes, Emissions};
use crate::class_writer::{INHERITED_MEMBERS_SLOT, INTERFACE_IMPLS_SLOT, INTERFACE_UPCASTS_SLOT};
use crate::ctx::{EmitCtx, ProjectState};
use crate::emission::{ClassEmission, EmittedMethod};
use crate::lang;

fn sig_re() -> &'static Regex {
    static R: OnceLock<Regex> = OnceLock::new();
    R.get_or_init(|| Regex::new(r"^pub fn\s+([A-Za-z_][A-Za-z0-9_]*)\s*\((.*)\)\s*->\s*Result<(.*)>\s*$").expect("静态正则"))
}

/// 提及接口类型变量的类型位置在运行时是 Object 引用（与宏的 erase_type 同一规则）
fn erase(ty: &str, type_params: &BTreeSet<String>) -> String {
    if idents(ty).any(|i| type_params.contains(i)) {
        "Object".into()
    } else {
        ty.to_string()
    }
}

/// 接口方法声明 → 擦除签名的 trait 方法声明（`fn m(&self, a: Object) -> Result<Object>`）
pub fn erased_declaration(names: &dyn ty::Names, m: &EmittedMethod, type_params: &BTreeSet<String>) -> Option<String> {
    let sig = m.signature(names);
    let c = sig_re().captures(&sig)?;
    let params = split_top_level_trimmed(&c[2]);
    if params.first().map(String::as_str) != Some("&self") {
        return None;
    }
    let mut out = vec!["&self".to_string()];
    for p in &params[1..] {
        let (pname, pty) = p.split_once(':').unwrap_or((p.as_str(), ""));
        out.push(format!("{}: {}", pname.trim(), erase(pty.trim(), type_params)));
    }
    Some(format!("fn {}({}) -> Result<{}>", &c[1], out.join(", "), erase(c[3].trim(), type_params)))
}

/// 擦除声明引用的类：提及类型变量而擦成 Object 的位置不引用类（与 [`erase`] 同一判定）
fn erased_classes(names: &dyn ty::Names, m: &EmittedMethod, type_params: &BTreeSet<String>) -> Vec<String> {
    let mut out: Vec<&str> = Vec::new();
    for t in m.sig.params.iter().map(|(_, t)| t).chain(std::iter::once(&m.sig.ret)) {
        if !idents(&t.render(names)).any(|i| type_params.contains(i)) {
            t.collect_classes(&mut out);
        }
    }
    out.into_iter().map(str::to_string).collect()
}

/// 成员的返回类型与接口擦除声明的返回类型是同一泛型类的不同实例化
fn result_reinstantiated(names: &dyn ty::Names, erased: &str, member: Option<&EmittedMethod>) -> bool {
    let Some(member) = member else { return false };
    let wanted = format!("pub {erased}");
    let got_sig = member.signature(names);
    let (Some(e), Some(m)) = (sig_re().captures(&wanted), sig_re().captures(&got_sig)) else { return false };
    let (want, got) = (e[3].trim(), m[3].trim());
    want.contains('<') && got.contains('<') && want != got && rust_type_head(want) == rust_type_head(got)
}

/// 类实现的全部接口（自身 + 超类链 + 超接口传递闭包），按发现序去重
fn all_interfaces(ctx: &EmitCtx<'_>, ci: &ClassInfo) -> Vec<String> {
    let reg = ctx.ty.reg;
    let mut found: IndexSet<String> = IndexSet::new();
    let mut pending: VecDeque<String> = VecDeque::new();
    let mut seen = BTreeSet::new();
    let mut cur = Some(ci);
    while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
        seen.insert(c.name().to_string());
        pending.extend(c.interfaces().iter().cloned());
        cur = (!c.super_class().is_empty()).then(|| reg.get(c.super_class())).flatten();
    }
    while let Some(i) = pending.pop_front() {
        if !found.insert(i.clone()) {
            continue;
        }
        if let Some(ici) = reg.get(&i) {
            pending.extend(ici.interfaces().iter().cloned());
        }
    }
    found.into_iter().collect()
}

/// 沿超类链找 (name, descriptor 形参) 命中的桥接方法 → (声明类, 真实方法描述符)
fn bridge_real_descriptor(ctx: &EmitCtx<'_>, ci: &ClassInfo, name: &str, desc: &str) -> Option<(String, String)> {
    let reg = ctx.ty.reg;
    let ci = reg.get(ci.name())?;
    let mut hit = resolve_bridge_target(ctx, ci, name, desc);
    if hit.is_none() {
        let want = param_part(desc);
        let mut seen = BTreeSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur.filter(|c| !seen.contains(c.name())) {
            seen.insert(c.name().to_string());
            if let Some(m) = c.methods().iter().find(|m| m.access & acc::BRIDGE != 0 && m.name == name && param_part(&m.desc) == want) {
                hit = resolve_bridge_target(ctx, c, name, &m.desc);
            }
            if hit.is_some() {
                break;
            }
            cur = (!c.super_class().is_empty()).then(|| reg.get(c.super_class())).flatten();
        }
    }
    hit.map(|(c, d)| (c.name().to_string(), d))
}

/// 沿 recv 的超类链找最近的方法声明 → (声明类, 方法)
fn locate<'e>(ctx: &EmitCtx<'_>, recv_bin: &str, name: &str, pdesc: &str, ems: &'e Emissions) -> Option<(String, &'e EmittedMethod)> {
    let reg = ctx.ty.reg;
    let mut cur = recv_bin.to_string();
    let mut seen = BTreeSet::new();
    while !cur.is_empty() && cur != lang::OBJECT && reg.contains(&cur) && seen.insert(cur.clone()) {
        if let Some(m) = ems.get(&cur).filter(|e| !e.handwritten).and_then(|e| e.find(name, pdesc)) {
            return Some((cur, m));
        }
        cur = reg.get(&cur).map(|c| c.super_class().to_string()).unwrap_or_default();
    }
    None
}

fn object_args(n: usize) -> Vec<String> {
    vec!["Object".to_string(); n]
}

/// 单接收者的接口实现段落
#[derive(Default)]
struct ImplTexts {
    blocks: Vec<String>,
    /// (泛型头, 源类型, 目标擦除接口载体)
    upcasts: Vec<(String, String, String)>,
}

/// 具体类对单个接口的 impl 成员声明（找不到落点的成员跳过）
fn iface_decls(
    ctx: &EmitCtx<'_>,
    reqs: &mut Vec<(String, String, String)>,
    ems: &Emissions,
    recv: &ClassEmission,
    recv_ci: &ClassInfo,
    iface: &ClassEmission,
    type_params: &BTreeSet<String>,
) -> Vec<String> {
    let recv_bin = recv.binary_name.as_str();
    let own_names: BTreeSet<&str> = recv.slotted().map(|m| m.rust_name.as_str()).collect();
    let provided = provided_methods(ctx, recv_bin);
    let mut decls = Vec::new();
    for im in iface.slotted() {
        let Some(erased) = erased_declaration(&ctx.ty, im, type_params) else { continue };
        let mut pdesc = param_part(&im.descriptor).to_string();
        let own = recv.find(&im.name, &pdesc);
        let mut located = None;
        let target = if let Some(o) = own {
            o.rust_name.clone()
        } else if provided.contains(&im.rust_name) {
            im.rust_name.clone()
        } else if provided.contains(&im.name) {
            im.name.clone()
        } else {
            let mut hit = locate(ctx, recv_bin, &im.name, &pdesc, ems);
            if hit.is_none() {
                if let Some((_, real_desc)) = bridge_real_descriptor(ctx, recv_ci, &im.name, &im.descriptor) {
                    pdesc = param_part(&real_desc).to_string();
                    hit = locate(ctx, recv_bin, &im.name, &pdesc, ems);
                }
            }
            let Some((owner_bin, method)) = hit else { continue };
            located = Some(method);
            if owner_bin != recv_bin {
                let recv_ci_reg = ctx.ty.reg.get(recv_bin).expect("接收者在注册表");
                let recv_target = match resolve_bridge_member(ctx, recv_ci_reg, &im.name, &pdesc, ems, Some(recv), true) {
                    Some(b) => b.member_name,
                    None => ctx.ty.receiver_member_name(&method.name, &method.descriptor, recv_ci),
                };
                if own_names.contains(recv_target.as_str()) || provided.contains(&recv_target) {
                    continue;
                }
                reqs.push((recv_bin.to_string(), im.name.clone(), pdesc.clone()));
                recv_target
            } else {
                method.rust_name.clone()
            }
        };
        let mut attr_parts = Vec::new();
        if target != im.rust_name {
            attr_parts.push(format!("target = \"{target}\""));
        }
        if result_reinstantiated(&ctx.ty, &erased, own.or(located)) {
            attr_parts.push("result = \"checkcast\"".into());
        }
        let attr = if attr_parts.is_empty() { String::new() } else { format!("#[java_method({})]\n", attr_parts.join(", ")) };
        decls.push(format!("{attr}{erased};"));
        claim_classes(ctx, &erased_classes(&ctx.ty, im, type_params), ems);
    }
    decls
}

/// 单接收者：全部接口的 impl 块 / 协变 upcast（名字在接收者作用域认领）；需要的继承成员需求追加到 `reqs`
fn recv_impls(
    ctx: &EmitCtx<'_>,
    reqs: &mut Vec<(String, String, String)>,
    ems: &Emissions,
    recv: &ClassEmission,
    recv_ci: &ClassInfo,
) -> ImplTexts {
    let sctx = ctx.scoped(&recv.scope);
    let ctx = &sctx;
    let mut out = ImplTexts::default();
    let recv_bin = recv.binary_name.as_str();
    let recv_abstract = recv_ci.class_file().access & acc::ABSTRACT != 0;
    let recv_is_iface = recv_ci.is_interface();
    let recv_params = class_params(ctx, recv_ci);
    let recv_short = ctx.short(recv_bin);
    let recv_generics = if recv_params.is_empty() { String::new() } else { format!("<{}>", recv_params.join(", ")) };
    let recv_ty = format!("{recv_short}{recv_generics}");
    let up_src = if recv_is_iface { rust_type(&recv_short, &object_args(recv_params.len())) } else { recv_ty.clone() };
    for iface_bin in all_interfaces(ctx, recv_ci) {
        let (Some(iface), Some(iface_ci)) = (ems.get(&iface_bin), ctx.ty.reg.get(&iface_bin)) else { continue };
        if iface.handwritten {
            continue;
        }
        let iface_short = ctx.short(&iface_bin);
        let marker = iface.slotted().next().is_none();
        let iface_params = class_params(ctx, iface_ci);
        let type_params: BTreeSet<String> = iface_params.iter().cloned().collect();
        let non_concrete = recv_abstract || recv_is_iface || marker;
        let decls = if non_concrete {
            Vec::new()
        } else {
            iface_decls(ctx, reqs, ems, recv, recv_ci, iface, &type_params)
        };
        if decls.is_empty() && !non_concrete {
            continue;
        }
        if !decls.is_empty() {
            // 宏为 impl 块生成 `impl <Iface>__VTable for ..`：派生名登记进本文件作用域
            ctx.ty.derived(&iface_bin, "__VTable");
            let body: Vec<String> = decls.iter().flat_map(|d| d.split('\n')).map(|l| format!("    {l}")).collect();
            out.blocks.push(format!("impl{recv_generics} {iface_short} for {recv_ty} {{\n{}\n}}", body.join("\n")));
        }
        let erased_iface_ty = rust_type(&iface_short, &object_args(iface_params.len()));
        let up_g = if recv_is_iface || recv_params.is_empty() { String::new() } else { format!("<{}>", recv_params.join(", ")) };
        out.upcasts.push((up_g, up_src.clone(), erased_iface_ty));
    }
    out
}

fn apply_impls(text: &str, t: &ImplTexts) -> String {
    let impl_text = if t.blocks.is_empty() {
        String::new()
    } else {
        let joined = t.blocks.join("\n\n");
        let lines: Vec<String> = joined.split('\n').map(|l| if l.is_empty() { String::new() } else { format!("    {l}") }).collect();
        format!("\n{}\n", lines.join("\n"))
    };
    let text = fill_slot(text, INTERFACE_IMPLS_SLOT, &impl_text);
    let upcast_text = if t.upcasts.is_empty() {
        String::new()
    } else {
        let mut groups: IndexMap<(&str, &str), Vec<&str>> = IndexMap::new();
        for (g, src, tgt) in &t.upcasts {
            groups.entry((g, src)).or_default().push(tgt);
        }
        let mut s = String::from("// 协变 upcast：类实例 → 擦除接口载体视图（A-4，接口类型实参在运行时不存在）\n");
        for ((g, src), tgts) in groups {
            s.push_str(&format!("rava_macros::iface_upcasts! {{ impl{g} {src} => {} }}\n", tgts.join(", ")));
        }
        s
    };
    fill_slot(&text, INTERFACE_UPCASTS_SLOT, &upcast_text)
}

/// 接收者是否生成接口实现段（其余类只清空插入位）
fn wants_impls(ctx: &EmitCtx<'_>, recv_bin: &str, recv: &ClassEmission) -> bool {
    ctx.ty.reg.get(recv_bin).is_some_and(|ci| !recv.handwritten && (recv.text.contains(INTERFACE_IMPLS_SLOT) || ci.is_interface()))
}

/// 为每个具体类生成接口实现声明（填充 `IMPLS_SLOT` / `UPCASTS_SLOT`；引用名在接收者作用域认领）；
/// 需要的继承成员登记到需求账本。须先于继承成员解析。
///
/// 各接收者只读别的类的结构化记录（方法签名、binary），导入按引用的 binary 生成，不读别的类
/// 文件头，故全部接收者一轮并行；继承成员需求按发射序并入账本
pub fn resolve_interface_impls(ctx: &EmitCtx<'_>, state: &mut ProjectState, ems: &mut Emissions) {
    let idx: Vec<usize> = (0..ems.len()).collect();
    let results = crate::par::par_map(crate::par::resolve_jobs(ctx.opts.jobs), &idx, |&i| {
        let (recv_bin, recv) = ems.get_index(i).expect("下标");
        let mut reqs = Vec::new();
        let texts = match ctx.ty.reg.get(recv_bin) {
            Some(ci) if wants_impls(ctx, recv_bin, recv) => recv_impls(ctx, &mut reqs, ems, recv, ci),
            _ => ImplTexts::default(),
        };
        (apply_impls(&recv.text, &texts), reqs)
    });
    for (i, (text, reqs)) in results.into_iter().enumerate() {
        ems[i].text = text;
        state.inherited_requests.extend(reqs);
    }
}

/// 接口 recv 的全部超接口及其在 recv 视角下的类型实参（广度优先，近者在前）
fn superinterface_views(ctx: &EmitCtx<'_>, recv_ci: &ClassInfo) -> Vec<(String, Vec<RsType>)> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::from([recv_ci.name().to_string()]);
    let mut queue: VecDeque<(&ClassInfo, BTreeMap<String, RsType>)> = VecDeque::from([(recv_ci, BTreeMap::new())]);
    while let Some((cur, mapping)) = queue.pop_front() {
        for (sup_bin, sup_args) in ctx.ty.superinterface_type_args(cur) {
            if !seen.insert(sup_bin.clone()) {
                continue;
            }
            let args: Vec<RsType> = sup_args.iter().map(|a| a.substitute(&|n| mapping.get(n).cloned())).collect();
            let Some(sup_ci) = ctx.ty.reg.get(&sup_bin) else { continue };
            out.push((sup_bin, args.clone()));
            let m = class_params(ctx, sup_ci).into_iter().zip(args).collect();
            queue.push_back((sup_ci, m));
        }
    }
    out
}

/// 接口接收者单个需求的超接口成员声明（引用名在接收者作用域认领）
fn iface_recv_member(
    ctx: &EmitCtx<'_>,
    ems: &Emissions,
    views: &[(String, Vec<RsType>)],
    name: &str,
    pdesc: &str,
    taken: &mut BTreeSet<String>,
) -> Option<String> {
    for (owner_bin, owner_args_ty) in views {
        let owner_args = &render_args(ctx, owner_args_ty);
        let Some(owner) = ems.get(owner_bin).filter(|e| !e.handwritten) else { continue };
        let Some(method) = owner.find(name, pdesc) else { continue };
        if !taken.insert(method.rust_name.clone()) {
            return None;
        }
        let owner_params = ctx.ty.reg.get(owner_bin).map(|c| class_params(ctx, c)).unwrap_or_default();
        let signature = substitute_type_params(&method.signature(&ctx.ty), &param_mapping(&owner_params, owner_args));
        let owner_ty = rust_type(&ctx.short(owner_bin), owner_args);
        let mut attr = vec![format!("name = \"{}\"", method.name), format!("descriptor = \"{}\"", method.descriptor)];
        if !method.access.is_empty() {
            attr.push(format!("access = \"{}\"", method.access));
        }
        attr.push(format!("inherited_from = \"{owner_ty}\""));
        let decl = format!("#[java_method({})]\n{signature};", attr.join(", "));
        let mut classes = substituted_classes(ctx, &method.sig, owner_bin, owner_args_ty);
        classes.extend(type_classes(owner_bin, owner_args_ty));
        claim_classes(ctx, &classes, ems);
        return Some(decl);
    }
    None
}

/// 接口接收者的继承成员（`list.forEach(..)` → List 接口块里声明 `inherited_from = "Iterable<E>"`）。
/// 须先于类接收者的继承成员解析
pub fn resolve_interface_inherited_members(ctx: &EmitCtx<'_>, state: &ProjectState, ems: &mut Emissions) {
    for (recv_bin, wanted) in requests_by_recv(state) {
        let Some(idx) = ems.get_index_of(&recv_bin) else { continue };
        let recv = &ems[idx];
        let Some(recv_ci) = ctx.ty.reg.get(&recv_bin) else { continue };
        if !recv_ci.is_interface() || recv.handwritten || !recv.text.contains(INHERITED_MEMBERS_SLOT) {
            continue;
        }
        let sctx = ctx.scoped(&recv.scope);
        let ctx = &sctx;
        let mut taken: BTreeSet<String> = recv.methods.iter().map(|m| m.rust_name.clone()).collect();
        let views = superinterface_views(ctx, recv_ci);
        let mut decls = Vec::new();
        for (name, pdesc) in &wanted {
            if recv.find(name, pdesc).is_some() {
                continue;
            }
            if let Some(d) = iface_recv_member(ctx, ems, &views, name, pdesc, &mut taken) {
                decls.push(d);
            }
        }
        let mut text = ems[idx].text.clone();
        if !decls.is_empty() {
            let pad = " ".repeat(8);
            let body = decls.join("\n\n");
            let lines: Vec<String> = body.split('\n').map(|l| if l.is_empty() { String::new() } else { format!("{pad}{l}") }).collect();
            let member_text = format!("\n{pad}// ── 继承成员（超接口声明）──────────────────\n{}\n", lines.join("\n"));
            text = fill_slot(&text, INHERITED_MEMBERS_SLOT, &member_text);
        }
        ems[idx].text = text;
    }
}

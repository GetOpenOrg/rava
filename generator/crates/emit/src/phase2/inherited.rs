//! 类接收者的继承成员声明（← `inherited_gen._forward_body` / 擦除名单 / `_member_declaration` /
//! `_interface_member_declaration` / `resolve_inherited_members`）。
//!
//! 方法体登记「接收者需要某继承方法」后，本模块沿接收者超类链找到最近的声明者，把祖先签名
//! 中的类型变量代入为接收者视角的实参，生成带转发体的继承成员声明：
//!
//! ```text
//! #[java_method(name = "speak", descriptor = "()I", access = "public",
//!               inherited_from = "Animal", vtable_owner = "Animal")]
//! pub fn speak(&self) -> Result<i32> { Animal__speak_base::<Self>(self) }
//! ```

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use indexmap::IndexMap;
use ty::ClassInfo;

use super::bridge::{bridge_override_member, covariant_bridge_pending};
use super::sig::{param_idents, param_mapping, param_part, result_inner, rust_type, sig_param_types, substitute_type_params};
use super::uses::{class_use_path, imported_names, imports_for, type_arg_uses};
use super::{anc_args, class_params, provided_methods, requests_by_recv, Emissions};
use crate::class_writer::{INHERITED_IMPORTS_SLOT, INHERITED_MEMBERS_SLOT};
use crate::ctx::{EmitCtx, ProjectState};
use crate::emission::{ClassEmission, EmittedMethod};
use crate::lang;
use crate::text::contains_word;

const RUST_PRIMS: [&str; 10] = ["i8", "i16", "i32", "i64", "f32", "f64", "bool", "u16", "char", "()"];

fn is_prim(t: &str) -> bool {
    RUST_PRIMS.contains(&t)
}

/// 签名首行之后的部分（`decl.split('\n', 1)[1]`）
fn after_attr(decl: &str) -> &str {
    decl.split_once('\n').map_or("", |(_, r)| r)
}

/// `pub fn <name>` 的名字
fn fn_name(line: &str) -> Option<&str> {
    let rest = line.strip_prefix("pub fn")?;
    let rest = rest.trim_start();
    if rest.len() == line.len() - "pub fn".len() {
        return None;
    }
    let end = rest.find(|c: char| !(c.is_ascii_alphanumeric() || c == '_')).unwrap_or(rest.len());
    let n = &rest[..end];
    n.chars().next().filter(|c| c.is_ascii_alphabetic() || *c == '_').map(|_| n)
}

/// 继承成员的转发体：精确执行 owner 祖先的实现（super 调用同源，不经 vtable 分派）
fn forward_body(ctx: &EmitCtx<'_>, method: &EmittedMethod, owner_bin: &str, owner_args: &[String]) -> String {
    let mut args = param_idents(&method.signature);
    let owner_short = ctx.short(owner_bin);
    if !method.handwritten {
        let mut turbo: Vec<&str> = owner_args.iter().map(String::as_str).collect();
        turbo.push("Self");
        let call = if args.is_empty() { "self".to_string() } else { format!("self, {}", args.join(", ")) };
        return format!("{owner_short}__{}_base::<{}>({call})", method.rust_name, turbo.join(", "));
    }
    let (ptypes, ret) = sig_param_types(&method.signature);
    let wrap = |a: &String, ty: &String| {
        if is_prim(ty) {
            a.clone()
        } else if owner_args.is_empty() {
            format!("Into::into({a})")
        } else {
            format!("From::from(Into::<Object>::into({a}))")
        }
    };
    let n = ptypes.len().min(args.len());
    let mut conv: Vec<String> = args.iter().zip(&ptypes).map(|(a, t)| wrap(a, t)).collect();
    conv.extend(args.drain(n..));
    let mut call = format!("<Self as {owner_short}__VTable>::__as_{owner_short}(self).__impl_{}({})", method.rust_name, conv.join(", "));
    if let Some(inner) = result_inner(&ret) {
        if !inner.is_empty() && !is_prim(inner) {
            call.push_str(if owner_args.is_empty() {
                ".map(Into::into)"
            } else {
                ".map(|__r| From::from(Into::<Object>::into(__r)))"
            });
        }
    }
    call
}

/// 擦除条目位置
#[derive(Clone, Copy, PartialEq, Eq)]
enum Pos {
    Param(usize),
    Ret,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Pos::Param(i) => write!(f, "{i}"),
            Pos::Ret => f.write_str("r"),
        }
    }
}

fn subst_inner(ret: &str) -> String {
    if ret.is_empty() {
        return String::new();
    }
    result_inner(ret).filter(|s| !s.is_empty()).unwrap_or(ret).to_string()
}

/// owner 签名中提及自身类型形参的位置 → 接收者视角下代入后的类型串
fn owner_erasure_entries(owner_sig: &str, substituted: &str, owner_params: &[String]) -> Vec<(Pos, String)> {
    if owner_params.is_empty() {
        return Vec::new();
    }
    let (owner_types, owner_ret) = sig_param_types(owner_sig);
    let (subst_types, subst_ret) = sig_param_types(substituted);
    let mentions = |t: &str| owner_params.iter().any(|p| contains_word(t, p));
    let mut out = Vec::new();
    for (i, t) in owner_types.iter().enumerate() {
        if mentions(t) && i < subst_types.len() {
            out.push((Pos::Param(i), subst_types[i].clone()));
        }
    }
    if !owner_ret.is_empty() {
        match result_inner(&owner_ret) {
            Some(inner) => {
                if mentions(inner) && !subst_ret.is_empty() {
                    if let Some(s) = result_inner(&subst_ret) {
                        out.push((Pos::Ret, s.to_string()));
                    }
                }
            }
            None if mentions(&owner_ret) => out.push((Pos::Ret, subst_ret.clone())),
            None => {}
        }
    }
    out.retain(|(_, t)| !t.is_empty() && t != "Object");
    out
}

/// 继承成员的槽位擦除名单（K-6）：按槽位声明（vtable_owner）的发射签名计算
fn slot_erasure_entries(ctx: &EmitCtx<'_>, vt_bin: &str, method: &EmittedMethod, substituted: &str) -> Vec<(Pos, String)> {
    let Some(vt_ci) = ctx.ty.reg.get(vt_bin) else { return Vec::new() };
    let pd = param_part(&method.descriptor);
    let Some(vt_m) = vt_ci
        .methods()
        .iter()
        .find(|x| x.name == method.name && !x.is_synthetic() && !x.is_static() && param_part(&x.desc) == pd)
    else {
        return Vec::new();
    };
    let names = ctx.ty.names;
    let vt_params = class_params(ctx, vt_ci);
    let es = ctx.ty.emitted_method_sig_types(vt_ci, vt_m, &vt_params);
    let slot_types: Vec<String> = es.params.iter().map(|t| t.render(names)).collect();
    let slot_ret = es.ret.render(names);
    let (subst_types, subst_ret) = sig_param_types(substituted);
    let s_inner = subst_inner(&subst_ret);
    let slot_is_object =
        |t: &str| t.is_empty() || t == "Object" || t == "()" || vt_params.iter().any(|p| contains_word(t, p));
    let mut out = Vec::new();
    for (i, st) in slot_types.iter().enumerate() {
        if slot_is_object(st) {
            if let Some(s) = subst_types.get(i).filter(|s| !s.is_empty() && *s != "Object") {
                out.push((Pos::Param(i), s.clone()));
            }
        }
    }
    let target = result_inner(&slot_ret).unwrap_or(&slot_ret);
    if !target.is_empty() && slot_is_object(target) && !s_inner.is_empty() && s_inner != "Object" && s_inner != "()" {
        out.push((Pos::Ret, s_inner));
    }
    out
}

/// 擦除条目 → `vtable_erasure` 属性值（名集；不擦除位置与名集同形时改用位置标记 `@i` / `@r`）
fn erasure_attr(entries: &[(Pos, String)], substituted: &str) -> String {
    if entries.is_empty() {
        return String::new();
    }
    let mut names: Vec<&str> = Vec::new();
    for (_, t) in entries {
        if !names.contains(&t.as_str()) {
            names.push(t);
        }
    }
    let erased: Vec<Pos> = entries.iter().map(|(p, _)| *p).collect();
    let (subst_types, subst_ret) = sig_param_types(substituted);
    let s_inner = subst_inner(&subst_ret);
    let mut kept: Vec<&str> =
        subst_types.iter().enumerate().filter(|(i, _)| !erased.contains(&Pos::Param(*i))).map(|(_, t)| t.as_str()).collect();
    if !erased.contains(&Pos::Ret) && !s_inner.is_empty() {
        kept.push(&s_inner);
    }
    if !kept.iter().any(|t| names.contains(t)) {
        return names.join(";");
    }
    entries.iter().map(|(p, _)| format!("@{p}")).collect::<Vec<_>>().join(";")
}

/// 祖先方法声明 → 接收者类视角下的继承成员声明（声明 + 转发体）
fn member_declaration(ctx: &EmitCtx<'_>, method: &EmittedMethod, owner_bin: &str, recv_ci: &ClassInfo) -> String {
    let anc = anc_args(ctx, recv_ci);
    let get_args = |b: &str| anc.iter().find(|(x, _)| x == b).map(|(_, a)| a.clone()).unwrap_or_default();
    let owner_params = ctx.ty.reg.get(owner_bin).map(|c| class_params(ctx, c)).unwrap_or_default();
    let owner_args = get_args(owner_bin);
    let mut signature = substitute_type_params(&method.signature, &param_mapping(&owner_params, &owner_args));
    let recv_name = ctx.ty.receiver_member_name(&method.name, &method.descriptor, recv_ci);
    if recv_name != method.rust_name {
        let old_head = format!("pub fn {}(", method.rust_name);
        if let Some(rest) = signature.strip_prefix(&old_head) {
            signature = format!("pub fn {recv_name}({rest}");
        }
    }
    let (mut vt_bin, mut vt_args) = (owner_bin.to_string(), owner_args.clone());
    if !method.virtual_in.is_empty() {
        vt_bin = anc.iter().find(|(b, _)| ctx.short(b) == method.virtual_in).map_or_else(|| owner_bin.to_string(), |(b, _)| b.clone());
        vt_args = get_args(&vt_bin);
    }
    let slot_name = if method.vtable_name.is_empty() { &method.rust_name } else { &method.vtable_name };
    let mut parts = vec![format!("name = \"{}\"", method.name), format!("descriptor = \"{}\"", method.descriptor)];
    if !method.access.is_empty() {
        parts.push(format!("access = \"{}\"", method.access));
    }
    parts.push(format!("inherited_from = \"{}\"", rust_type(&ctx.short(owner_bin), &owner_args)));
    if !method.virtual_in.is_empty() {
        parts.push(format!("vtable_owner = \"{}\"", rust_type(&ctx.short(&vt_bin), &vt_args)));
        if &recv_name != slot_name {
            parts.push(format!("vtable_name = \"{slot_name}\""));
        }
    }
    let mut erasure = slot_erasure_entries(ctx, &vt_bin, method, &signature);
    if erasure.is_empty() {
        erasure = owner_erasure_entries(&method.signature, &signature, &owner_params);
    }
    let ev = erasure_attr(&erasure, &signature);
    if !ev.is_empty() {
        parts.push(format!("vtable_erasure = \"{ev}\""));
    }
    let body = forward_body(ctx, method, owner_bin, &owner_args);
    format!("#[java_method({})]\n{signature} {{ {body} }}", parts.join(", "))
}

/// 接口方法声明 → 类接收者视角下的继承成员声明；返回 (声明文本, 本类视角的 Rust 方法名)
fn interface_member_declaration(
    ctx: &EmitCtx<'_>,
    method: &EmittedMethod,
    owner_bin: &str,
    owner_args: &[String],
    recv_ci: &ClassInfo,
) -> (String, String) {
    let owner_params = ctx.ty.reg.get(owner_bin).map(|c| class_params(ctx, c)).unwrap_or_default();
    let mut signature = substitute_type_params(&method.signature, &param_mapping(&owner_params, owner_args));
    let local = ctx.ty.interface_member_local_name(recv_ci, &method.name, &method.descriptor);
    let mut parts = vec![format!("name = \"{}\"", method.name), format!("descriptor = \"{}\"", method.descriptor)];
    if !method.access.is_empty() {
        parts.push(format!("access = \"{}\"", method.access));
    }
    parts.push(format!("inherited_from = \"{}\"", rust_type(&ctx.short(owner_bin), owner_args)));
    parts.push("owner_kind = \"interface\"".into());
    if local != method.rust_name {
        parts.push(format!("target = \"{}\"", method.rust_name));
        if fn_name(&signature) == Some(method.rust_name.as_str()) {
            let after_kw = &signature["pub fn".len()..];
            let name_at = "pub fn".len() + (after_kw.len() - after_kw.trim_start().len());
            signature = format!("pub fn {local}{}", &signature[name_at + method.rust_name.len()..]);
        }
    }
    (format!("#[java_method({})]\n{signature};", parts.join(", ")), local)
}

/// 单接收者的继承成员生成（成员声明 / use 行累积到 `out`）
struct RecvPass<'a, 'c> {
    ctx: &'a EmitCtx<'c>,
    ems: &'a Emissions,
    recv: &'a ClassEmission,
    recv_ci: &'c ClassInfo,
    taken: BTreeSet<String>,
    imported: BTreeSet<String>,
    arg_uses: BTreeMap<String, String>,
    members: Vec<String>,
    imports: Vec<String>,
}

impl<'a> RecvPass<'a, '_> {
    /// 超类链上 (mname, pdesc) 的最近声明者
    fn super_decl(&self, mname: &str, pdesc: &str) -> Option<(String, &'a EmittedMethod)> {
        let reg = self.ctx.ty.reg;
        let mut cur = self.recv_ci.super_class().to_string();
        let mut seen = BTreeSet::new();
        while !cur.is_empty() && cur != lang::OBJECT && reg.contains(&cur) && seen.insert(cur.clone()) {
            if let Some(found) = self.ems.get(&cur).filter(|e| !e.handwritten).and_then(|e| e.find(mname, pdesc)) {
                return Some((cur, found));
            }
            cur = reg.get(&cur).map(|c| c.super_class().to_string()).unwrap_or_default();
        }
        None
    }

    /// 转发体调用的 owner `__base` 自由函数的导入
    fn base_fn_import(&mut self, owner_bin: &str, method: &EmittedMethod) {
        if method.handwritten {
            return;
        }
        let base_short = format!("{}__{}_base", self.ctx.short(owner_bin), method.rust_name);
        if self.imported.insert(base_short) {
            let path = class_use_path(self.ctx, owner_bin, &self.recv.crate_prefix, Some(self.ems), &self.recv.crate_name);
            self.imports.push(format!("use {path}__{}_base;", method.rust_name));
        }
    }

    /// 桥接成员（成功返回 true；名字缺失 / 重名回落普通继承）
    fn try_bridge(&mut self, name: &str, pdesc: &str) -> bool {
        let Some(b) = bridge_override_member(self.ctx, name, pdesc, self.recv, self.recv_ci, self.ems) else { return false };
        let fname = fn_name(after_attr(&b.sig_text).trim()).map(str::to_string);
        let Some(fname) = fname.filter(|n| !self.taken.contains(n)) else { return false };
        self.taken.insert(fname);
        self.members.push(b.decl);
        let u = imports_for(self.ctx, &b.sig_text, self.recv, self.recv, &mut self.imported, Some(&self.arg_uses), Some(self.ems));
        self.imports.extend(u);
        let Some((rn, rp)) = b.real_want else { return true };
        if self.recv.find(&rn, &rp).is_some() {
            return true;
        }
        let Some((real_owner, real_m)) = self.super_decl(&rn, &rp) else { return true };
        let real_recv_name = self.ctx.ty.receiver_member_name(&real_m.name, &real_m.descriptor, self.recv_ci);
        if self.taken.contains(&real_recv_name) {
            return true;
        }
        self.taken.insert(real_recv_name);
        let real_decl = member_declaration(self.ctx, real_m, &real_owner, self.recv_ci);
        let owner_em = &self.ems[&real_owner];
        let u = imports_for(self.ctx, after_attr(&real_decl), owner_em, self.recv, &mut self.imported, Some(&self.arg_uses), None);
        self.members.push(real_decl);
        self.imports.extend(u);
        self.base_fn_import(&real_owner, real_m);
        true
    }

    /// 超类链无声明 → 接口方法（抽象类未实现的接口抽象方法 / 未注入本类的 default）
    fn interface_member(&mut self, name: &str, pdesc: &str) {
        let names = self.ctx.ty.names;
        for (iface_bin, iface_args) in self.ctx.ty.implemented_interface_views(self.recv_ci) {
            let Some(iface) = self.ems.get(&iface_bin).filter(|e| !e.handwritten) else { continue };
            let Some(im) = iface.find(name, pdesc) else { continue };
            let iface_args: Vec<String> = iface_args.iter().map(|t| t.render(names)).collect();
            let (decl, local) = interface_member_declaration(self.ctx, im, &iface_bin, &iface_args, self.recv_ci);
            if !self.taken.insert(local) {
                break;
            }
            let owner_ty = rust_type(&self.ctx.short(&iface_bin), &iface_args);
            let scan = format!("{} {owner_ty}", after_attr(&decl));
            self.members.push(decl);
            let mut uses = imports_for(self.ctx, &scan, iface, self.recv, &mut self.imported, None, Some(self.ems));
            if self.imported.insert(self.ctx.short(&iface_bin)) {
                let p = class_use_path(self.ctx, &iface_bin, &self.recv.crate_prefix, Some(self.ems), &self.recv.crate_name);
                uses.push(format!("use {p};"));
            }
            self.imports.extend(uses);
            break;
        }
    }

    fn want(&mut self, name: &str, pdesc: &str) {
        let recv_bin = self.recv.binary_name.as_str();
        if let Some(own) = self.recv.find(name, pdesc) {
            if !covariant_bridge_pending(self.ctx, self.recv_ci, recv_bin, own, name, pdesc) {
                return;
            }
        }
        if self.try_bridge(name, pdesc) {
            return;
        }
        let Some((owner_bin, method)) = self.super_decl(name, pdesc) else {
            self.interface_member(name, pdesc);
            return;
        };
        let recv_name = self.ctx.ty.receiver_member_name(&method.name, &method.descriptor, self.recv_ci);
        if !self.taken.insert(recv_name) {
            return;
        }
        let decl = member_declaration(self.ctx, method, &owner_bin, self.recv_ci);
        let owner_em = &self.ems[&owner_bin];
        let u = imports_for(self.ctx, after_attr(&decl), owner_em, self.recv, &mut self.imported, Some(&self.arg_uses), Some(self.ems));
        self.members.push(decl);
        self.imports.extend(u);
        self.base_fn_import(&owner_bin, method);
    }
}

/// 按登记的需求为各接收者类生成继承成员声明，并填充所有类文本的两个插入位
pub fn resolve_inherited_members(ctx: &EmitCtx<'_>, state: &ProjectState, ems: &mut Emissions) {
    let mut members: IndexMap<String, Vec<String>> = IndexMap::new();
    let mut imports: IndexMap<String, Vec<String>> = IndexMap::new();
    for (recv_bin, wanted) in requests_by_recv(state) {
        let (Some(recv), Some(recv_ci)) = (ems.get(&recv_bin), ctx.ty.reg.get(&recv_bin)) else { continue };
        if recv.handwritten || !recv.text.contains(INHERITED_MEMBERS_SLOT) {
            continue;
        }
        let mut taken: BTreeSet<String> = recv.methods.iter().map(|m| m.rust_name.clone()).collect();
        taken.extend(provided_methods(ctx, &recv_bin));
        let mut imported = imported_names(&recv.text);
        imported.insert(ctx.short(&recv_bin));
        let arg_uses = type_arg_uses(ctx, recv_ci, ems, &recv.crate_prefix, &recv.crate_name);
        let mut pass = RecvPass {
            ctx,
            ems,
            recv,
            recv_ci,
            taken,
            imported,
            arg_uses,
            members: Vec::new(),
            imports: Vec::new(),
        };
        for (name, pdesc) in &wanted {
            pass.want(name, pdesc);
        }
        members.entry(recv_bin.clone()).or_default().extend(pass.members);
        imports.entry(recv_bin).or_default().extend(pass.imports);
    }
    for (bin, em) in ems.iter_mut() {
        let member_text = match members.get(bin).filter(|d| !d.is_empty()) {
            Some(decls) => {
                let body = decls.join("\n\n");
                let pad = " ".repeat(8);
                let indented: Vec<String> =
                    body.split('\n').map(|l| if l.is_empty() { String::new() } else { format!("{pad}{l}") }).collect();
                format!("\n{pad}// ── 继承成员（祖先声明，本类未覆盖）──────────────────\n{}\n", indented.join("\n"))
            }
            None => String::new(),
        };
        let use_text: String = imports.get(bin).map(|v| v.iter().map(|l| format!("{l}\n")).collect()).unwrap_or_default();
        em.text = super::fill_slot(&em.text, INHERITED_MEMBERS_SLOT, &member_text);
        em.text = super::fill_slot(&em.text, INHERITED_IMPORTS_SLOT, &use_text);
    }
}

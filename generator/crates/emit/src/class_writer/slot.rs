//! 覆盖条目的槽位签名擦除（← `class_writer._owner_slot_declaration` / `_override_vtable_erasure`）。
//!
//! 槽位（声明祖先 A 的 `A__VTable`）签名 = A 的发射签名经 A 的类型形参 Object 化；覆盖者在这些
//! 位置保持的具体形态（祖先形参代入后的实参 K-6b、协变返回 K-6a）按 token 全等交给宏擦除。

use std::collections::{BTreeSet, VecDeque};

use classfile::Method;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::text::contains_word;
use crate::vtable::param_part;

/// owner 类链上的槽位声明：精确描述符优先，其次同名同参数（非私有、非 static）；
/// 类未声明时按接口 default 解析（广度）
fn owner_slot_declaration<'c>(ctx: &EmitCtx<'c>, owner: &'c ClassInfo, m: &Method) -> Option<(&'c ClassInfo, &'c Method)> {
    let real = |x: &&Method| !x.is_static() && !x.is_synthetic();
    let hit = owner
        .methods()
        .iter()
        .filter(real)
        .find(|x| x.name == m.name && x.desc == m.desc)
        .or_else(|| {
            owner
                .methods()
                .iter()
                .filter(real)
                .find(|x| x.name == m.name && param_part(&x.desc) == param_part(&m.desc) && !x.is_private())
        });
    if let Some(h) = hit {
        return Some((owner, h));
    }
    let mut q: VecDeque<&str> = owner.interfaces().iter().map(String::as_str).collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    while let Some(n) = q.pop_front() {
        if n.is_empty() || !seen.insert(n) {
            continue;
        }
        let Some(ici) = ctx.ty.reg.get(n) else { continue };
        let found = ici.methods().iter().find(|x| {
            x.name == m.name
                && !x.is_synthetic()
                && param_part(&x.desc) == param_part(&m.desc)
                && !x.is_static()
                && !x.is_abstract()
        });
        if let Some(x) = found {
            return Some((ici, x));
        }
        q.extend(ici.interfaces().iter().map(String::as_str));
    }
    None
}

fn result_inner(t: &str) -> Option<&str> {
    t.strip_prefix("Result<").and_then(|r| r.strip_suffix('>')).map(str::trim)
}

/// 覆盖方法在声明祖先（槽位类短名 `virtual_in`）vtable 上的擦除名单（去重保序）
pub fn override_vtable_erasure(ctx: &EmitCtx<'_>, ci: &ClassInfo, m: &Method, virtual_in: &str) -> Vec<String> {
    let reg = ctx.ty.reg;
    let mut owner = None;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut cur = ci.super_class();
    while !cur.is_empty() && seen.insert(cur) {
        let Some(c) = reg.get(cur) else { break };
        if ctx.short(cur) == virtual_in {
            owner = Some(c);
            break;
        }
        cur = c.super_class();
    }
    let Some(owner) = owner else { return Vec::new() };
    let Some((decl_ci, owner_m)) = owner_slot_declaration(ctx, owner, m) else { return Vec::new() };
    let owner_params = ctx.ty.effective_class_type_params(decl_ci).to_vec();
    let names = ctx.ty.names;
    let anc = ctx.ty.emitted_method_sig_types(decl_ci, owner_m, &owner_params);
    let own_params = ctx.ty.effective_class_type_params(ci).to_vec();
    let own = ctx.ty.emitted_method_sig_types(ci, m, &own_params);
    let own_types: Vec<String> = own.params.iter().map(|t| t.render(names)).collect();
    let own_ret = own.ret.render(names);
    let slot_is_object =
        |t: &str| t.is_empty() || t == "Object" || t == "()" || owner_params.iter().any(|p| contains_word(t, p));
    let mut out: Vec<String> = Vec::new();
    for (i, a) in anc.params.iter().enumerate() {
        if slot_is_object(&a.render(names)) {
            if let Some(o) = own_types.get(i).filter(|o| !o.is_empty() && *o != "Object") {
                out.push(o.clone());
            }
        }
    }
    let anc_ret = anc.ret.render(names);
    let slot_ret = result_inner(&anc_ret).unwrap_or(&anc_ret);
    if slot_is_object(slot_ret) {
        let own_inner = result_inner(&own_ret).unwrap_or(&own_ret);
        if !own_inner.is_empty() && own_inner != "Object" && own_inner != "()" {
            out.push(own_inner.to_string());
        }
    }
    let mut dedup = BTreeSet::new();
    out.retain(|t| dedup.insert(t.clone()));
    out
}

//! `__base` 自由函数的命名解析（invokespecial super 调用的落点）。
//!
//! 私有最小移植：`instr/member_owner` 的 `_resolve_special_method_owner` /
//! `class_inherits_default_method` / `_resolve_method_owner`，与 `instr/member_naming.
//! _mangle_if_overloaded` 在「invokespecial 已解析到声明类」这一调用面上的分支。
//! 方法体翻译（P4c `method` crate）接入后统一到其成员命名模块，本文件随之删除。
//! 桥接重定向分支（调用描述符只命中 synthetic bridge）未移植，显式报错。

use std::collections::{BTreeSet, VecDeque};

use ty::type_map::mangle_name;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::error::{EmitError, Result};
use crate::lang;
use crate::vtable::param_part;

/// 类直接实现的接口闭包（含父接口）中是否有 (mname, desc) 的 default 方法体
pub fn class_inherits_default_method(ctx: &EmitCtx<'_>, cls: &str, mname: &str, desc: &str) -> bool {
    let reg = ctx.ty.reg;
    let Some(ci) = reg.get(cls) else { return false };
    let mut q: VecDeque<&str> = ci.interfaces().iter().map(String::as_str).collect();
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    while let Some(n) = q.pop_front() {
        if !seen.insert(n) {
            continue;
        }
        let Some(ici) = reg.get(n) else { continue };
        if ici
            .methods()
            .iter()
            .any(|m| !m.is_static() && !m.is_abstract() && m.name == mname && m.desc == desc)
        {
            return true;
        }
        q.extend(ici.interfaces().iter().map(String::as_str));
    }
    false
}

/// invokespecial 的 JVM 方法解析：父类链上最近的声明类（描述符精确）；链上无声明时取
/// 链上最早实现该 default 方法所在接口的类；都没有则常量池类本身
pub fn resolve_special_owner(ctx: &EmitCtx<'_>, cls: &str, mname: &str, desc: &str) -> String {
    let reg = ctx.ty.reg;
    let mut seen: Vec<&str> = Vec::new();
    let mut cur = cls;
    while !cur.is_empty() && !seen.contains(&cur) {
        seen.push(cur);
        let Some(ci) = reg.get(cur) else { break };
        if ci.methods().iter().any(|m| !m.is_static() && m.name == mname && m.desc == desc) {
            return cur.to_string();
        }
        cur = ci.super_class();
    }
    let injected = seen.iter().rev().find(|c| class_inherits_default_method(ctx, c, mname, desc));
    injected.map_or_else(|| cls.to_string(), |c| (*c).to_string())
}

/// `_resolve_method_owner`：沿父类链找非 synthetic 的同名、参数前缀一致的声明类
fn resolve_method_owner<'c>(ctx: &EmitCtx<'c>, ci: &'c ClassInfo, mname: &str, desc: &str) -> Option<&'c ClassInfo> {
    let pp = param_part(desc);
    let mut cur = Some(ci);
    while let Some(c) = cur {
        if c.methods().iter().any(|m| !m.is_synthetic() && m.name == mname && m.desc.starts_with(pp)) {
            return Some(c);
        }
        let sc = c.super_class();
        if sc.is_empty() || sc == lang::OBJECT {
            break;
        }
        cur = ctx.ty.reg.get(sc);
    }
    None
}

/// 父类链与接口闭包上是否存在描述符精确命中的 synthetic bridge
fn has_bridge(ctx: &EmitCtx<'_>, ci: &ClassInfo, mname: &str, desc: &str) -> bool {
    let reg = ctx.ty.reg;
    let hit = |c: &ClassInfo| c.methods().iter().any(|m| m.is_synthetic() && m.name == mname && m.desc == desc);
    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut pending: VecDeque<String> = VecDeque::new();
    let mut cur = Some(ci);
    while let Some(c) = cur.filter(|c| seen.insert(c.name().to_string())) {
        if hit(c) {
            return true;
        }
        pending.extend(c.interfaces().iter().cloned());
        cur = reg.get(c.super_class());
    }
    while let Some(n) = pending.pop_front() {
        let Some(ic) = reg.get(&n) else { continue };
        if !seen.insert(ic.name().to_string()) {
            continue;
        }
        if hit(ic) {
            return true;
        }
        pending.extend(ic.interfaces().iter().cloned());
    }
    false
}

/// 按短名定位注册表类（直接命中，或按插入序首个短名相同者）
fn lookup_by_short<'c>(ctx: &EmitCtx<'c>, cls_short: &str) -> Option<&'c ClassInfo> {
    if let Some(c) = ctx.ty.reg.get(cls_short) {
        return Some(c);
    }
    if cls_short.contains('/') {
        return None;
    }
    let norm = cls_short.replace('$', "_");
    ctx.ty.reg.iter_insertion().find(|c| ctx.ty.names.short(c.name()) == norm)
}

/// `_mangle_if_overloaded`（invokespecial 调用面）：`cls_short` 为声明类短名
pub fn base_member_name(ctx: &EmitCtx<'_>, cls_short: &str, mname: &str, desc: &str) -> Result<String> {
    let mt = &ctx.manifest.ty;
    if mname.is_empty() || (mname.starts_with('<') && mname != "<init>") {
        return Ok(mname.to_string());
    }
    let root_short = lang::OBJECT.rsplit('/').next().unwrap_or(lang::OBJECT);
    let mangled = mangle_name(mt, mname, desc);
    if mangled != mname && ctx.root_api().contains(&mangled) {
        return Ok(mangled);
    }
    if cls_short.rsplit('/').next() == Some(root_short) {
        return Ok(mname.to_string());
    }
    let Some(recv) = lookup_by_short(ctx, cls_short) else {
        return Ok(mname.to_string());
    };
    let is_root = |c: &ClassInfo| c.name().rsplit('/').next() == Some(root_short);
    if is_root(recv) {
        return Ok(mname.to_string());
    }
    if mname != "<init>" {
        match resolve_method_owner(ctx, recv, mname, desc) {
            Some(owner) if is_root(owner) => return Ok(mname.to_string()),
            Some(_) => {}
            None if has_bridge(ctx, recv, mname, desc) => {
                return Err(EmitError::Unported(format!(
                    "__base 命名：{}.{mname}:{desc} 的调用描述符只命中 synthetic bridge（桥接重定向未移植）",
                    recv.name()
                )));
            }
            None if !recv.is_interface() => return Ok(ctx.ty.interface_member_local_name(recv, mname, desc)),
            None => {
                return Err(EmitError::Unported(format!(
                    "__base 命名：接口接收者 {}.{mname}:{desc}（声明接口定位未移植）",
                    recv.name()
                )));
            }
        }
    }
    if ctx.ty.hierarchy_overloaded_names(recv).contains(mname) {
        Ok(mangled)
    } else {
        Ok(mname.to_string())
    }
}

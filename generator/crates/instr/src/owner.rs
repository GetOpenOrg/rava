//! 成员归属解析（← `instr/member_owner.py` 的注册表查询部分）：JVM 方法 / 字段解析
//! （JVMS §5.4.3）——沿继承链 / 接口闭包找到成员的声明类，bridge 目标解析。
//!
//! 依赖栈状态的部分（`_close_open_type_args` / `_resolve_virtual_sig_params`）在
//! [`crate::invoke`] 中。

use std::collections::{BTreeSet, VecDeque};

use classfile::{Method, Operand};
use ty::{ClassInfo, Registry};

use crate::ctx::InstrCtx;

const ACC_PRIVATE: u16 = 0x0002;

/// 描述符的参数部分 `(..)`（协变返回不参与匹配）
pub fn param_part(desc: &str) -> &str {
    match desc.find(')') {
        Some(i) => &desc[..=i],
        None => desc,
    }
}

fn is_root(c: &str) -> bool {
    c.is_empty() || c == ty::consts::OBJECT
}

/// 类及其继承链上实例字段 `safe_fname` 的泛型签名（`_get_field_generic_signature`）
pub fn field_generic_signature(reg: &Registry, class: &str, safe_fname: &str) -> String {
    let mut ci = reg.get(class);
    while let Some(c) = ci {
        if let Some(f) = c.fields().iter().find(|f| !f.is_static() && ty::ident::safe_ident(&f.name) == safe_fname) {
            return ty::registry::field_signature(f).to_string();
        }
        if is_root(c.super_class()) {
            break;
        }
        ci = reg.get(c.super_class());
    }
    String::new()
}

/// 非合成方法中是否有 `mname`（给出描述符时按参数部分前缀匹配）
fn declares_real(ci: &ClassInfo, mname: &str, pp: &str) -> bool {
    ci.methods().iter().any(|m| !m.is_synthetic() && m.name == mname && (pp.is_empty() || m.desc.starts_with(pp)))
}

/// T76：方法只在祖先声明时的 `_super.` 访问前缀段数（`_find_method_super_prefix`）；
/// 当前类声明 / 查不到 → 0
pub fn method_super_depth(reg: &Registry, class: &str, mname: &str, desc: &str) -> usize {
    let Some(ci) = reg.get(class) else {
        return 0;
    };
    let pp = if desc.is_empty() { "" } else { param_part(desc) };
    if declares_real(ci, mname, pp) {
        return 0;
    }
    let mut depth = 0;
    let mut sc = ci.super_class().to_string();
    while !is_root(&sc) {
        let Some(p) = reg.get(&sc) else {
            break;
        };
        depth += 1;
        if declares_real(p, mname, pp) {
            return depth;
        }
        sc = p.super_class().to_string();
    }
    0
}

/// 沿继承链解析 `mname` 的声明类（`_resolve_method_owner`；描述符按参数部分匹配，
/// 排除合成 / 桥方法）→ (owner, 层数)；整条链没有 → None
pub fn resolve_method_owner(reg: &Registry, class: &str, mname: &str, desc: &str) -> Option<(String, usize)> {
    if class.is_empty() {
        return None;
    }
    let pp = if desc.is_empty() { "" } else { param_part(desc) };
    let mut ci = reg.get(class);
    let mut lvl = 0;
    while let Some(c) = ci {
        if declares_real(c, mname, pp) {
            return Some((c.name().to_string(), lvl));
        }
        if is_root(c.super_class()) {
            break;
        }
        ci = reg.get(c.super_class());
        lvl += 1;
    }
    None
}

/// 实例方法 `mname:desc`（完整描述符精确匹配）的声明者（JVMS §5.4.3.3 / §5.4.3.4）：
/// 先沿超类链（含自身）找首个声明；没有 → 超接口闭包中的最具体（maximally-specific）声明，
/// 多个互不为子接口时取广度序首个；都没有 → None
pub fn resolve_method_declarer(reg: &Registry, class: &str, mname: &str, desc: &str) -> Option<String> {
    let hit = |c: &ClassInfo| c.methods().iter().any(|m| !m.is_static() && m.name == mname && m.desc == desc);
    let mut chain = Vec::new();
    let mut cur = reg.get(class);
    let mut seen = BTreeSet::new();
    while let Some(c) = cur.filter(|c| seen.insert(c.name().to_string())) {
        if hit(c) {
            return Some(c.name().to_string());
        }
        chain.push(c);
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    let mut queue: VecDeque<&str> = chain.iter().flat_map(|c| c.interfaces().iter().map(String::as_str)).collect();
    let mut seen_if = BTreeSet::new();
    let mut found: Vec<&ClassInfo> = Vec::new();
    while let Some(i) = queue.pop_front() {
        let Some(ic) = reg.get(i).filter(|_| seen_if.insert(i.to_string())) else {
            continue;
        };
        let declares = ic.methods().iter().any(|m| {
            !m.is_static() && !m.is_synthetic() && m.access & ACC_PRIVATE == 0 && m.name == mname && m.desc == desc
        });
        if declares {
            found.push(ic);
        }
        queue.extend(ic.interfaces().iter().map(String::as_str));
    }
    let specific = found.iter().find(|x| !found.iter().any(|y| y.name() != x.name() && is_subinterface(reg, y, x.name())))?;
    Some(specific.name().to_string())
}

/// 接口 `sub` 是否（传递地）继承接口 `sup`
fn is_subinterface(reg: &Registry, sub: &ClassInfo, sup: &str) -> bool {
    let mut queue: VecDeque<&str> = sub.interfaces().iter().map(String::as_str).collect();
    let mut seen = BTreeSet::new();
    while let Some(i) = queue.pop_front() {
        if i == sup {
            return true;
        }
        if seen.insert(i.to_string()) {
            queue.extend(reg.get(i).into_iter().flat_map(|c| c.interfaces().iter().map(String::as_str)));
        }
    }
    false
}

/// invokespecial（`super.m()`）的方法解析（`_resolve_special_method_owner`）：父类链上最近的
/// 精确声明者；链上无声明 → 链上最早实现带该 default 方法体接口的类；都没有 → 常量池类
pub fn resolve_special_method_owner(reg: &Registry, class: &str, mname: &str, desc: &str) -> String {
    let mut cur = class.to_string();
    let mut seen: Vec<String> = Vec::new();
    while !cur.is_empty() && !seen.contains(&cur) {
        seen.push(cur.clone());
        let Some(ci) = reg.get(&cur) else {
            break;
        };
        if ci.methods().iter().any(|m| !m.is_static() && m.name == mname && m.desc == desc) {
            return cur;
        }
        cur = ci.super_class().to_string();
    }
    let mut injected = String::new();
    for c in &seen {
        if class_inherits_default_method(reg, c, mname, desc) {
            injected = c.clone();
        }
    }
    if injected.is_empty() {
        class.to_string()
    } else {
        injected
    }
}

/// 类直接实现的接口闭包中是否有 (mname, desc) 的 default 方法体
pub fn class_inherits_default_method(reg: &Registry, class: &str, mname: &str, desc: &str) -> bool {
    let Some(ci) = reg.get(class) else {
        return false;
    };
    let mut queue: VecDeque<String> = ci.interfaces().iter().cloned().collect();
    let mut visited = BTreeSet::new();
    while let Some(i) = queue.pop_front() {
        if !visited.insert(i.clone()) {
            continue;
        }
        let Some(ici) = reg.get(&i) else {
            continue;
        };
        if ici.methods().iter().any(|m| is_default_body(m, mname, desc)) {
            return true;
        }
        queue.extend(ici.interfaces().iter().cloned());
    }
    false
}

fn is_default_body(m: &Method, mname: &str, desc: &str) -> bool {
    !m.is_static() && !m.is_abstract() && m.name == mname && m.desc == desc
}

/// 从接口自身起广度遍历父接口，找首个满足 `pred` 的声明接口
fn bfs_interfaces(reg: &Registry, iface: &str, pred: impl Fn(&Method) -> bool) -> Option<String> {
    let root = reg.get(iface)?;
    if !root.is_interface() {
        return None;
    }
    let mut queue = VecDeque::from([iface.to_string()]);
    let mut seen = BTreeSet::new();
    while let Some(cur) = queue.pop_front() {
        if !seen.insert(cur.clone()) {
            continue;
        }
        let Some(ci) = reg.get(&cur) else {
            continue;
        };
        if ci.methods().iter().any(&pred) {
            return Some(cur);
        }
        queue.extend(ci.interfaces().iter().cloned());
    }
    None
}

/// invokespecial InterfaceMethod（`Iface.super.m()` / 接口私有方法）的解析
/// （`_resolve_interface_special_target`）：最近的有方法体声明接口
pub fn resolve_interface_special_target(reg: &Registry, iface: &str, mname: &str, desc: &str) -> Option<String> {
    bfs_interfaces(reg, iface, |m| is_default_body(m, mname, desc))
}

/// 常量池类为接口且目标为私有实例方法时的声明接口（`private_interface_method_target`）
pub fn private_interface_method_target(reg: &Registry, iface: &str, mname: &str, desc: &str) -> Option<String> {
    bfs_interfaces(reg, iface, |m| is_default_body(m, mname, desc) && m.access & ACC_PRIVATE != 0)
}

/// `Iface.super.m(..)` 在实现类中的落点成员名 `Iface_super_m`（重载时带描述符后缀）
pub fn interface_special_member_name(ctx: &InstrCtx, owner: &str, mname: &str, desc: &str) -> String {
    let owner_short = ctx.short(owner);
    let rust_m = match ctx.reg().get(owner) {
        Some(ci) if ctx.ty.hierarchy_overloaded_names(ci).contains(mname) => {
            ty::type_map::mangle_name(ctx.ty.manifest, mname, desc)
        }
        _ => mname.to_string(),
    };
    format!("{owner_short}_super_{rust_m}")
}

/// invokestatic 的方法解析（`_resolve_static_method_owner`）：沿父类链的精确静态声明者
pub fn resolve_static_method_owner(reg: &Registry, class: &str, mname: &str, desc: &str) -> Option<String> {
    let mut cur = class.to_string();
    let mut seen = BTreeSet::new();
    while !cur.is_empty() && seen.insert(cur.clone()) {
        let ci = reg.get(&cur)?;
        if ci.methods().iter().any(|m| m.is_static() && m.name == mname && m.desc == desc) {
            return Some(cur);
        }
        cur = ci.super_class().to_string();
    }
    None
}

/// getstatic / putstatic 的字段解析（JVMS §5.4.3.2，`_resolve_static_field_owner`）：
/// 自身 → 父接口（深度优先）→ 父类
pub fn resolve_static_field_owner(reg: &Registry, class: &str, field: &str) -> Option<String> {
    fn lookup(reg: &Registry, cur: &str, field: &str, seen: &mut BTreeSet<String>) -> Option<String> {
        if cur.is_empty() || !seen.insert(cur.to_string()) {
            return None;
        }
        let ci = reg.get(cur)?;
        if ci.fields().iter().any(|f| f.is_static() && f.name == field) {
            return Some(cur.to_string());
        }
        for i in ci.interfaces() {
            if let Some(found) = lookup(reg, i, field, seen) {
                return Some(found);
            }
        }
        lookup(reg, ci.super_class(), field, seen)
    }
    lookup(reg, class, field, &mut BTreeSet::new())
}

/// 被桥接的真实方法：(声明类, 真实描述符)（`_bridge_call_target`：bridge 体里最后一条同名调用）
fn bridge_call_target(reg: &Registry, bridge: &Method, mname: &str) -> Option<(String, String)> {
    let code = bridge.code.as_ref()?;
    for ins in code.insns.iter().rev() {
        let Operand::Method(mref, _) = &ins.operand else {
            continue;
        };
        if mref.name != mname {
            continue;
        }
        let real_pp = param_part(&mref.desc);
        let declared = |ci: &ClassInfo| -> Option<String> {
            ci.methods().iter().find(|m| !m.is_synthetic() && m.name == mname && m.desc.starts_with(real_pp)).map(|m| m.desc.clone())
        };
        if let Some(d) = reg.get(&mref.owner).and_then(declared) {
            return Some((mref.owner.clone(), d));
        }
        let (owner, _) = resolve_method_owner(reg, &mref.owner, mname, &mref.desc)?;
        let d = reg.get(&owner).and_then(declared)?;
        return Some((owner, d));
    }
    None
}

/// 精确命中 `desc` 的合成 bridge 所桥接的真实方法（`_resolve_bridge_target`）：
/// 先父类链、再接口闭包（广度）；找不到 / 不可解析 → None
pub fn resolve_bridge_target(reg: &Registry, ci: &ClassInfo, mname: &str, desc: &str) -> Option<(String, String)> {
    let bridge_in = |c: &ClassInfo| -> Option<Method> {
        c.methods().iter().find(|m| m.is_synthetic() && m.name == mname && m.desc == desc).cloned()
    };
    let mut seen = BTreeSet::new();
    let mut pending: VecDeque<String> = VecDeque::new();
    let mut cur = Some(ci);
    while let Some(c) = cur {
        if !seen.insert(c.name().to_string()) {
            break;
        }
        if let Some(b) = bridge_in(c) {
            return bridge_call_target(reg, &b, mname);
        }
        pending.extend(c.interfaces().iter().cloned());
        cur = if c.super_class().is_empty() { None } else { reg.get(c.super_class()) };
    }
    while let Some(i) = pending.pop_front() {
        let Some(ic) = reg.get(&i) else {
            continue;
        };
        if !seen.insert(ic.name().to_string()) {
            continue;
        }
        if let Some(b) = bridge_in(ic) {
            return bridge_call_target(reg, &b, mname);
        }
        pending.extend(ic.interfaces().iter().cloned());
    }
    None
}

/// 接口（或其超接口，广度）中声明实例方法 `mname:desc` 的接口（`_declaring_interface`）；
/// 根类虚方法的重声明、私有 / 合成方法不算 → None
pub fn declaring_interface(ctx: &InstrCtx, iface: &ClassInfo, mname: &str, desc: &str) -> Option<String> {
    if ctx.facts.root_virtual.contains(&(mname.to_string(), param_part(desc).to_string())) {
        return None;
    }
    let mut queue: VecDeque<&ClassInfo> = VecDeque::from([iface]);
    let mut seen = BTreeSet::new();
    while let Some(cur) = queue.pop_front() {
        if !seen.insert(cur.name().to_string()) {
            continue;
        }
        let hit = cur.methods().iter().any(|m| {
            m.name == mname && m.desc == desc && !m.is_static() && !m.is_synthetic() && m.access & ACC_PRIVATE == 0
        });
        if hit {
            return Some(cur.name().to_string());
        }
        queue.extend(cur.interfaces().iter().filter_map(|i| ctx.reg().get(i)));
    }
    None
}

/// 接收者类链上 (mname, desc) 的首个声明是否为合成桥方法（`_first_decl_is_bridge`）；
/// 类链无声明时按超接口（广度）的首个实例声明判定
pub fn first_decl_is_bridge(reg: &Registry, ci: &ClassInfo, mname: &str, desc: &str) -> bool {
    let mut seen = BTreeSet::new();
    let mut walk = Some(ci);
    while let Some(w) = walk {
        if !seen.insert(w.name().to_string()) {
            break;
        }
        if let Some(m) = w.methods().iter().find(|m| m.name == mname && m.desc == desc) {
            return m.is_synthetic();
        }
        walk = if w.super_class().is_empty() { None } else { reg.get(w.super_class()) };
    }
    let mut pending: VecDeque<String> = ci.interfaces().iter().cloned().collect();
    let mut cur = Some(ci);
    while let Some(c) = cur {
        if c.super_class().is_empty() {
            break;
        }
        cur = reg.get(c.super_class());
        if let Some(p) = cur {
            pending.extend(p.interfaces().iter().cloned());
        }
    }
    while let Some(i) = pending.pop_front() {
        let Some(ic) = reg.get(&i) else {
            continue;
        };
        if !seen.insert(ic.name().to_string()) {
            continue;
        }
        if let Some(m) = ic.methods().iter().find(|m| m.name == mname && m.desc == desc && !m.is_static()) {
            return m.is_synthetic();
        }
        pending.extend(ic.interfaces().iter().cloned());
    }
    false
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use classfile::{acc, ClassFile, Method};
    use ty::Registry;

    use super::resolve_method_declarer;

    fn method(access: u16, name: &str, desc: &str) -> Method {
        Method {
            access,
            name: name.to_string(),
            desc: desc.to_string(),
            signature: None,
            code: None,
            exceptions: vec![],
            annotations: vec![],
            annotation_default: None,
            parameters: vec![],
            synthetic_attr: false,
        }
    }

    fn class(name: &str, sup: Option<&str>, ifaces: &[&str], iface: bool, methods: Vec<Method>) -> Arc<ClassFile> {
        Arc::new(ClassFile {
            minor: 0,
            major: 65,
            access: acc::PUBLIC | if iface { acc::INTERFACE | acc::ABSTRACT } else { 0 },
            name: name.to_string(),
            super_name: sup.map(str::to_string),
            interfaces: ifaces.iter().map(|s| s.to_string()).collect(),
            fields: vec![],
            methods,
            signature: None,
            source_file: None,
            bootstrap_methods: vec![],
            inner_classes: vec![],
            enclosing_method: None,
            nest_host: None,
            nest_members: vec![],
            permitted_subclasses: vec![],
            record_components: None,
            annotations: vec![],
        })
    }

    /// 声明者解析：超类链优先；接口继承的方法取最具体的超接口声明
    #[test]
    fn declarer_through_superinterfaces() {
        const D: &str = "(Lp/K;)Lp/V;";
        let abs = acc::PUBLIC | acc::ABSTRACT;
        let mut reg = Registry::new();
        for c in [
            class(ty::consts::OBJECT, None, &[], false, vec![]),
            class("p/Map", Some(ty::consts::OBJECT), &[], true, vec![method(abs, "get", D)]),
            class("p/SortedMap", Some(ty::consts::OBJECT), &["p/Map"], true, vec![]),
            class("p/NavMap", Some(ty::consts::OBJECT), &["p/SortedMap"], true, vec![]),
            class("p/CMap", Some(ty::consts::OBJECT), &["p/Map"], true, vec![method(abs, "get", D)]),
            class("p/Base", Some(ty::consts::OBJECT), &["p/CMap", "p/Map"], false, vec![]),
            class("p/Impl", Some("p/Base"), &[], false, vec![method(acc::PUBLIC, "get", D)]),
            class("p/Leaf", Some("p/Impl"), &["p/NavMap"], false, vec![]),
        ] {
            reg.insert(c);
        }
        assert_eq!(resolve_method_declarer(&reg, "p/NavMap", "get", D).as_deref(), Some("p/Map"));
        assert_eq!(resolve_method_declarer(&reg, "p/Base", "get", D).as_deref(), Some("p/CMap"));
        assert_eq!(resolve_method_declarer(&reg, "p/Leaf", "get", D).as_deref(), Some("p/Impl"));
        assert_eq!(resolve_method_declarer(&reg, "p/NavMap", "put", D), None);
    }
}

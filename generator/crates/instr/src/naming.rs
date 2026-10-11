//! 成员命名（← `instr/member_naming.py`）：重载 mangle、lambda 实现方法名的单一来源、
//! 类已知性判定。
//!
//! Python 的 `_LambdaNameLedger` 全局账在这里是 [`crate::log::Effect::LambdaRef`] 日志项，
//! 由调用方汇总断言；`_JAVA_RUST_RENAME` 在 Python 侧为空表，不移植。

use ty::ClassInfo;

use crate::ctx::InstrCtx;
use crate::error::InstrResult;
use crate::hierarchy;
use crate::owner;

/// binary / 短名的末段
fn simple(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

/// java_runtime 手写类的短名集（Python `JAVA_RUNTIME_SHORT_NAMES`：只有根类）
fn is_runtime_short(short: &str) -> bool {
    short == simple(ty::consts::OBJECT)
}

/// 短类名是否已知（`_class_known`：注册表内 binary、手写根类，或可反查到注册表的 Rust 名）
pub fn class_known(ctx: &InstrCtx, cls_short: &str) -> bool {
    if ctx.reg().is_empty() || cls_short.is_empty() || is_runtime_short(cls_short) || ctx.reg().contains(cls_short) {
        return true;
    }
    hierarchy::short_binary(ctx, cls_short).is_some()
}

/// 根类同名重载（wait 族）的描述符后缀名：后缀名在手写根类 API 名面时采用
/// （`_handwritten_root_overload_name`）
fn root_overload_name(ctx: &InstrCtx, mname: &str, desc: &str) -> Option<String> {
    let mangled = ty::type_map::mangle_name(ctx.ty.manifest, mname, desc);
    (mangled != mname && ctx.facts.root_api.contains(&mangled)).then_some(mangled)
}

/// 调用目标 `cls.mname:desc` 的 Rust 方法名（`_mangle_if_overloaded`）：重载 → 描述符后缀名，
/// 否则原名。`cls` 为 binary 名或短名；`desc` 为调用描述符（None → 不做声明者解析）
pub fn mangle_if_overloaded(ctx: &InstrCtx, cls: &str, mname: &str, desc: Option<&str>) -> InstrResult<String> {
    // 结果只取决于注册表与全局事实（整次生成不变），按调用目标缓存
    let key = format!("{cls}\0{mname}\0{}", desc.unwrap_or("\u{1}"));
    if let Some(hit) = ctx.facts.mangle_cache.read().unwrap_or_else(|e| e.into_inner()).get(&key) {
        return Ok(hit.clone());
    }
    let name = mangle_uncached(ctx, cls, mname, desc)?;
    ctx.facts.mangle_cache.write().unwrap_or_else(|e| e.into_inner()).insert(key, name.clone());
    Ok(name)
}

fn mangle_uncached(ctx: &InstrCtx, cls: &str, mname: &str, desc: Option<&str>) -> InstrResult<String> {
    let reg = ctx.reg();
    if reg.is_empty() || mname.is_empty() || (mname.starts_with('<') && mname != "<init>") {
        return Ok(mname.to_string());
    }
    if let Some(root) = desc.and_then(|d| root_overload_name(ctx, mname, d)) {
        return Ok(root);
    }
    if is_runtime_short(simple(cls)) {
        return Ok(mname.to_string());
    }
    let mut target: Option<&ClassInfo> = reg.get(cls);
    if target.is_none() && !cls.contains('/') {
        target = hierarchy::short_binary(ctx, cls).and_then(|b| reg.get(&b));
    }
    let Some(mut target) = target else {
        return Ok(mname.to_string());
    };
    if is_runtime_short(simple(target.name())) {
        return Ok(mname.to_string());
    }
    // 接口重声明的根类公开方法：命名同根类（接口不发射该成员，其同名重载不改变它）
    if desc.is_some_and(|d| owner::is_iface_root_member(ctx, target, mname, d)) {
        return mangle_uncached(ctx, ty::consts::OBJECT, mname, desc);
    }
    let recv = target;
    let mut call_desc = desc.unwrap_or("").to_string();
    let mut bridged_iface_desc: Option<String> = None;
    if let (true, Some(d)) = (mname != "<init>", desc) {
        match owner::resolve_method_owner(reg, target.name(), mname, d).and_then(|(o, _)| reg.get(&o)) {
            Some(o) => {
                target = o;
                if is_runtime_short(simple(target.name())) {
                    return Ok(mname.to_string());
                }
            }
            None => match owner::resolve_bridge_target(reg, target, mname, d) {
                Some((b_owner, b_desc)) => {
                    match reg.get(&b_owner) {
                        Some(b) if !b.is_interface() => target = b,
                        _ => bridged_iface_desc = Some(b_desc.clone()),
                    }
                    call_desc = b_desc;
                }
                None if !target.is_interface() => {
                    return Ok(ctx.ty.interface_member_local_name(target, mname, d));
                }
                None => {
                    if let Some(decl) = owner::declaring_interface(ctx, target, mname, d) {
                        if decl != target.name() {
                            if let Some(ci) = reg.get(&decl) {
                                target = ci;
                            }
                        }
                    }
                }
            },
        }
    }
    // 接口接收者经继承看到的超接口成员：声明名，兄弟超接口同名异参时视图内按描述符区分
    if recv.is_interface() && target.is_interface() && target.name() != recv.name() && bridged_iface_desc.is_none() && !call_desc.is_empty() {
        return Ok(ctx.ty.interface_view_member_name(recv, target, mname, &call_desc));
    }
    let name_ci = if recv.is_interface() { target } else { recv };
    if !ctx.ty.hierarchy_overloaded_names(name_ci).contains(mname) {
        if let (Some(bd), false) = (&bridged_iface_desc, recv.is_interface()) {
            return Ok(ctx.ty.interface_member_local_name(recv, mname, bd));
        }
        return Ok(mname.to_string());
    }
    if call_desc.is_empty() {
        return Ok(mname.to_string());
    }
    Ok(ty::type_map::mangle_name(ctx.ty.manifest, mname, &call_desc))
}

/// invokedynamic 实现方法（lambda body / 方法引用目标）的 Rust 名（`lambda_impl_rust_name`）：
/// 重载 mangle、`<init>` → `new`、安全标识符三步收拢
pub fn lambda_impl_rust_name(ctx: &InstrCtx, impl_cls: &str, impl_mname: &str, impl_desc: &str) -> InstrResult<String> {
    let mangled = mangle_if_overloaded(ctx, impl_cls, impl_mname, Some(impl_desc))?;
    Ok(ty::ident::safe_ident(&mangled.replace("<init>", "new")))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use classfile::{acc, ClassFile, Method};
    use input::RuntimeManifest;
    use ty::consts::OBJECT;
    use ty::{Registry, ShortNames, TyCtx};

    use super::mangle_if_overloaded;
    use crate::ctx::{InstrCtx, InstrFacts, NoHooks};

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

    fn class(name: &str, ifaces: &[&str], iface: bool, methods: Vec<Method>) -> Arc<ClassFile> {
        Arc::new(ClassFile {
            minor: 0,
            major: 65,
            access: acc::PUBLIC | if iface { acc::INTERFACE | acc::ABSTRACT } else { 0 },
            name: name.to_string(),
            super_name: (name != OBJECT).then(|| OBJECT.to_string()),
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

    /// 接口重声明根类 equals(Object) 且带 equals(自身) 重载：接口不发射根类成员，
    /// 经接口接收者调用 equals(Object) / hashCode 取根类名；自身重载与实现类仍按重载态后缀
    #[test]
    fn iface_redeclared_root_member_keeps_root_name() {
        let eq_obj = format!("(L{OBJECT};)Z");
        let eq_self = "(Lp/Named;)Z".to_string();
        let pub_abs = acc::PUBLIC | acc::ABSTRACT;
        let object = class(
            OBJECT,
            &[],
            false,
            vec![method(acc::PUBLIC, "equals", &eq_obj), method(acc::PUBLIC | acc::NATIVE, "hashCode", "()I")],
        );
        let mut reg = Registry::new();
        reg.insert(object.clone());
        reg.insert(class(
            "p/Named",
            &[],
            true,
            vec![method(pub_abs, "equals", &eq_self), method(pub_abs, "equals", &eq_obj), method(pub_abs, "hashCode", "()I")],
        ));
        reg.insert(class(
            "p/Impl",
            &["p/Named"],
            false,
            vec![method(acc::PUBLIC, "equals", &eq_self), method(acc::PUBLIC, "equals", &eq_obj), method(acc::PUBLIC, "hashCode", "()I")],
        ));
        let names = ShortNames::build(&reg);
        let rt = RuntimeManifest::default();
        let rt_src = std::env::temp_dir().join(format!("rava_naming_iface_root_{}", std::process::id()));
        let facts = InstrFacts::build(&reg, Some(object.as_ref()), &rt_src);
        let tctx = TyCtx::new(&reg, &names, &rt.ty);
        let ctx = InstrCtx::new(tctx, &rt, &facts, &NoHooks, "p/Caller");
        let mangled = |d: &str| ty::type_map::mangle_name(&rt.ty, "equals", d);

        assert_eq!(mangle_if_overloaded(&ctx, "p/Named", "equals", Some(&eq_obj)).unwrap(), "equals");
        assert_eq!(mangle_if_overloaded(&ctx, "p/Named", "hashCode", Some("()I")).unwrap(), "hashCode");
        assert_eq!(mangle_if_overloaded(&ctx, "p/Named", "equals", Some(&eq_self)).unwrap(), mangled(&eq_self));
        assert_ne!(mangled(&eq_self), "equals");
        assert_eq!(mangle_if_overloaded(&ctx, "p/Impl", "equals", Some(&eq_obj)).unwrap(), mangled(&eq_obj));
        assert_eq!(mangle_if_overloaded(&ctx, "p/Impl", "equals", Some(&eq_self)).unwrap(), mangled(&eq_self));
    }
}

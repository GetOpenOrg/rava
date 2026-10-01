//! 调用点类型实参推断与 @CallerSensitive 包装（← `instr/invoke.py` 的 `_bind_type_args` /
//! `_resolve_ctor_turbofish_args` / `_ctor_outer_ref_base` / `_super_ctor_view_args` /
//! `caller_sensitive_wrap`）。
//!
//! Python 以类型串合一；这里以 [`RsType`] 结构合一，比较口径仍按渲染文本（与 Python 串等价）。

use sim::StackSim;
use ty::RsType;

use crate::build::ty_text;
use crate::env::InstrEnv;
use crate::invoke::sig::TargMap;

/// 类型是否为菱形推断占位 `_`
pub fn is_infer(t: &RsType) -> bool {
    matches!(t, RsType::Param(n) if n == sim::INFER_PARAM)
}

/// 形参签名类型与实参静态类型结构化合一，得到类型变量 → 实参的绑定（`_bind_type_args`：
/// Java 钻石 / 泛型方法推断的静态近似）。
///
/// 裸类型变量直接绑定实参类型；同一泛型类的实例化（`G<T>` ← `G<X>`）逐类型实参递归。
/// 基本类型 / `()` / `_` / 顶层 Object 不产生绑定；同一变量得到互相矛盾的绑定时取 Object。
pub fn bind_type_args(env: &InstrEnv, sig_types: &[RsType], arg_tys: &[RsType], tparams: &[String]) -> TargMap {
    let mut bound = TargMap::new();
    for (s, a) in sig_types.iter().zip(arg_tys) {
        unify(env, s, a, false, tparams, &mut bound);
    }
    bound
}

fn unify(env: &InstrEnv, sig_t: &RsType, arg_t: &RsType, nested: bool, tparams: &[String], bound: &mut TargMap) {
    if let RsType::Param(n) = sig_t {
        if tparams.contains(n) {
            if matches!(arg_t, RsType::Prim(_) | RsType::Unit) || is_infer(arg_t) {
                return;
            }
            // 顶层 Object 实参可经 From<Object> 进入任意类型变量，不构成约束；
            // 类型实参位置（JArray<E> ← JArray<Object>）不变，E 只能是 Object
            if matches!(arg_t, RsType::Object) && !nested {
                return;
            }
            match bound.get(n) {
                None => {
                    bound.insert(n.clone(), arg_t.clone());
                }
                Some(prev) if ty_text(env, prev) != ty_text(env, arg_t) => {
                    bound.insert(n.clone(), RsType::Object);
                }
                Some(_) => {}
            }
            return;
        }
    }
    let (s_args, a_args) = (sig_t.type_args(), arg_t.type_args());
    if s_args.is_empty() || a_args.is_empty() || s_args.len() != a_args.len() {
        return;
    }
    if sig_t.head_name(env.ctx.ty.names) != arg_t.head_name(env.ctx.ty.names) {
        return;
    }
    for (s, a) in s_args.iter().zip(a_args) {
        unify(env, s, a, true, tparams, bound);
    }
}

/// `binary` 的顶层类名（`$` 之前）
fn top_level(binary: &str) -> &str {
    binary.split('$').next().unwrap_or(binary)
}

/// 泛型类构造器的 turbofish 实参（`_resolve_ctor_turbofish_args`）；类非泛型 / 注册表外 → None。
///
/// 1. 构造器签名形参引用类型变量 → 由实参类型合一，全部绑定时直接采用；
/// 2. 构造类是当前类 / 其内部类（同一顶层类）且类型形参名集合一致 → 当前 impl 的形参；
/// 3. 逐变量：已绑定取绑定，调用方同名形参取该形参，否则 Object（A-1 存储擦除后恒可行）。
pub fn resolve_ctor_turbofish_args(
    env: &InstrEnv,
    sim: &StackSim,
    full_cls: &str,
    ctor_params: &[String],
    arg_tys: &[RsType],
) -> Option<Vec<RsType>> {
    let ctx = &env.ctx;
    let ci = ctx.reg().get(full_cls)?;
    let cls_tparams = ctx.ty.effective_class_type_params(ci);
    if cls_tparams.is_empty() {
        return None;
    }
    // 规则 1：与定义侧同一张形参类型表（含编译器注入的外部实例形参 Outer<E>）
    let full_desc = format!("({})V", ctor_params.concat());
    let ctor_sig = ci
        .methods()
        .iter()
        .find(|m| m.name == "<init>" && m.desc == full_desc)
        .map(|m| ctx.ty.method_sig_types(ci, m, &cls_tparams).params)
        .filter(|sp| !sp.is_empty() && sp.len() == ctor_params.len());
    let mut subst = TargMap::new();
    if let Some(sp) = ctor_sig.filter(|_| !arg_tys.is_empty()) {
        subst = bind_type_args(env, &sp, arg_tys, &cls_tparams);
        // 裸类型变量形参收到顶层 Object 实参（擦除值：方法级类型变量 / 真 Object）且别处未绑定：取 Object。
        // 取调用方同名形参会把实参经 From<Object> 转成该形参的当前实例化——Java 不做的检查转换
        // （Box<T>.map 内 new Box<R>(f.apply(value))：R ≠ T）
        for (s, a) in sp.iter().zip(arg_tys) {
            if let RsType::Param(n) = s {
                if cls_tparams.contains(n) && matches!(a, RsType::Object) {
                    subst.entry(n.clone()).or_insert(RsType::Object);
                }
            }
        }
        if subst.len() == cls_tparams.len() {
            return Some(cls_tparams.iter().map(|t| subst.get(t).cloned().unwrap_or(RsType::Object)).collect());
        }
    }
    let caller = ctx.class_name;
    let caller_tps = &sim.cfg.class_type_params;
    // 规则 2：同一顶层类之下的内部类共享外部类的类型变量（内部类经 this$N 继承）
    if !caller.is_empty() && !caller_tps.is_empty() {
        let related = full_cls == caller
            || full_cls.starts_with(&format!("{caller}$"))
            || top_level(full_cls) == top_level(caller);
        let same_set = cls_tparams.iter().all(|t| caller_tps.contains(t)) && caller_tps.iter().all(|t| cls_tparams.contains(t));
        if related && same_set {
            return Some(cls_tparams.iter().map(|t| RsType::Param(t.clone())).collect());
        }
    }
    // 规则 3：未确定的变量优先取调用方同名类型变量，否则 Object
    let scoped = !caller.is_empty() && !caller_tps.is_empty();
    Some(
        cls_tparams
            .iter()
            .map(|t| match subst.get(t) {
                Some(b) => b.clone(),
                None if scoped && caller_tps.contains(t) => RsType::Param(t.clone()),
                None => RsType::Object,
            })
            .collect(),
    )
}

/// 内部类构造器首个形参是编译器注入的外部类引用、且定义侧按「外部类 + 继承的类型形参」
/// 生成（`Outer<E>`）时，返回该外部实例类型（`_ctor_outer_ref_base`；调用侧按头名比较）
pub fn ctor_outer_ref_base(env: &InstrEnv, cls_bin: &str, params: &[String]) -> Option<RsType> {
    let ctx = &env.ctx;
    let first = params.first()?;
    let ci = ctx.reg().get(cls_bin)?;
    let outer_bin = ty::class_params::outer_instance_class(ci);
    if outer_bin.is_empty() || *first != format!("L{outer_bin};") {
        return None;
    }
    ctx.ty.outer_instance_rust_type(&outer_bin, &ctx.ty.effective_class_type_params(ci))
}

/// `super(..)` 目标父类在本类定义内部视角下的类型实参（`_super_ctor_view_args`，K-5）：
/// 祖先链上命中取链上实参；不在链上 → 全 Object；非泛型父类 → 空
pub fn super_ctor_view_args(env: &InstrEnv, target_bin: &str) -> Vec<RsType> {
    let ctx = &env.ctx;
    let Some(ci) = ctx.reg().get(ctx.class_name) else {
        return Vec::new();
    };
    if target_bin.is_empty() {
        return Vec::new();
    }
    if let Some((_, args)) = ctx.ty.ancestor_type_args(ci, None).into_iter().find(|(b, _)| b == target_bin) {
        return args;
    }
    match ctx.reg().get(target_bin) {
        Some(t) => RsType::objects(ctx.ty.effective_class_type_params(t).len()),
        None => Vec::new(),
    }
}

/// 注解描述符 `Lx/Y;` → binary 名
fn anno_binary(type_desc: &str) -> &str {
    type_desc.strip_prefix('L').and_then(|s| s.strip_suffix(';')).unwrap_or(type_desc)
}

/// 被调方法（沿超类链解析声明处）的声明：(是否 @CallerSensitive, 是否 native)
pub(crate) fn caller_sensitive_decl(env: &InstrEnv, owner_bin: &str, mname: &str, desc: &str) -> (bool, bool) {
    let annos = &env.ctx.rt.caller_sensitive_annotations;
    if env.ctx.reg().is_empty() || annos.is_empty() {
        return (false, false);
    }
    let mut cur = owner_bin.to_string();
    let mut seen = std::collections::BTreeSet::new();
    while !cur.is_empty() && seen.insert(cur.clone()) {
        let Some(ci) = env.ctx.reg().get(&cur) else {
            break;
        };
        if let Some(m) = ci.methods().iter().find(|m| m.name == mname && m.desc == desc) {
            let cs = m.annotations.iter().any(|a| annos.contains(anno_binary(&a.type_desc)));
            return (cs, m.is_native());
        }
        cur = ci.super_class().to_string();
    }
    (false, false)
}

/// @CallerSensitive 调用：`__caller_sensitive("调用处类", || call)`；其余 → None（原样）。
///
/// 例外：`getCallerClass` 自身（native 的 CS 方法）不包装——它返回的是「调用它的 CS 方法」
/// 的调用方，即外层调用点已压入的栈顶；若在 CS 方法体内再压入所在类，`lookup()` 等会把
/// lookup 类解析成 CS 方法所在类自己。Python 以声明类名后缀判定，这里以
/// 「同名且声明为 native」判定（不写类名字面量）。
pub fn caller_sensitive_wrap(env: &InstrEnv, call_text: &str, owner_bin: &str, mname: &str, desc: &str) -> Option<String> {
    let (cs, native) = caller_sensitive_decl(env, owner_bin, mname, desc);
    if !cs || (native && mname == "getCallerClass") {
        return None;
    }
    Some(format!("__caller_sensitive(\"{}\", || {call_text})", env.ctx.class_name))
}

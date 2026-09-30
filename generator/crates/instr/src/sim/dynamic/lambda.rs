//! lambda / 方法引用调用点（Arch-3，`dynamic.sim_dynamic` 的 invokedynamic 主分支）：
//! 捕获值按声明序弹出并克隆为 `__lam_cap{idx}_{i}`，以实现方法（`impl:Cls.name:desc`）与
//! SAM 描述符（`samtype:`）合成真实 Rust 闭包，经 SAM 合成对象或不透明闭包装箱为 Object。
//!
//! 实参适配见 [`super::lambda_args`]，闭包体与返回值适配见 [`super::lambda_body`]。

use classfile::Const;
use sim::StackSim;
use ty::type_map::{parse_descriptor_params, parse_descriptor_return};
use ty::{ClassInfo, RsType};

use super::{raw, raw_stmt, IndySite};
use crate::build::{text, ty_text};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::log::{Effect, InstrLog};
use crate::naming::lambda_impl_rust_name;

/// 一个 lambda 站点的翻译事实（Python 同名局部变量的汇总）
pub(super) struct Lam<'r> {
    pub impl_cls: String,
    pub impl_mname: String,
    pub impl_desc: String,
    /// 实现方法 Rust 名（`lambda_impl_rust_name` 单一来源）
    pub impl_rust_name: String,
    /// 实现类（注册表外 → None：运行时手写层类）
    pub ci: Option<&'r ClassInfo>,
    pub is_ctor: bool,
    pub is_instance: bool,
    pub has_generic_sig: bool,
    /// 实现类的有效类型形参
    pub tparams: Vec<String>,
    /// 实现方法形参的泛型签名类型（无签名 → 空）
    pub sig_types: Vec<RsType>,
    /// 实现方法描述符形参
    pub impl_params: Vec<String>,
    /// SAM 描述符
    pub sam_desc: String,
    pub sam_params: Vec<String>,
    pub sam_ret: String,
    /// SAM 形参名 `_la{i}`
    pub sam_names: Vec<String>,
    /// 捕获变量名 `__lam_cap{idx}_{i}` 与捕获值的栈类型
    pub cap_names: Vec<String>,
    pub cap_tys: Vec<RsType>,
}

impl Lam<'_> {
    /// 描述符类型在 Rust 侧是否表现为擦除的 Object（接口别名 / 未翻译类）
    pub fn is_erased_ref(&self, env: &InstrEnv, d: &str) -> bool {
        let Some(bin) = d.strip_prefix('L').and_then(|r| r.strip_suffix(';')) else {
            return false;
        };
        match env.ctx.reg().get(bin) {
            None => true,
            Some(ci) => ci.is_interface() || rust_text(env, d) == ir::anchors::OBJECT,
        }
    }
}

/// 描述符 → Rust 类型文本（Python `jvm_to_rust` 的字符串结果）
pub(super) fn rust_text(env: &InstrEnv, d: &str) -> String {
    ty_text(env, &env.ctx.ty.jvm_to_rust(d))
}

/// lambda 类引导方法的 (samtype, impl)：静态实参 ≥ 2，arg0 为 MethodType、arg1 为 MethodHandle
fn lambda_args(site: &IndySite, is_lambda: bool) -> (String, String) {
    let Some(args) = site.bsm.map(|b| &b.args).filter(|a| is_lambda && a.len() >= 2) else {
        return (String::new(), String::new());
    };
    let sam = match &args[0] {
        Const::MethodType(d) => d.clone(),
        _ => String::new(),
    };
    let imp = match &args[1] {
        Const::MethodHandle(h) => h.member.to_string(),
        _ => String::new(),
    };
    (sam, imp)
}

/// `Cls.method:desc`（或 `pkg/Cls.method:desc`）→ (类, 方法名, 描述符)
fn split_impl(r: &str) -> Option<(&str, &str, &str)> {
    let dot = r.rfind('.')?;
    let colon = dot + r[dot..].find(':')?;
    (colon > dot).then(|| (&r[..dot], &r[dot + 1..colon], &r[colon + 1..]))
}

/// 按动态描述符弹出捕获值（声明序）；栈空时以 `Object::default()` 占位
fn pop_captures(sim: &mut StackSim, dyn_desc: &str) -> InstrResult<Vec<sim::StackEntry>> {
    let mut caps = Vec::new();
    if dyn_desc.starts_with('(') {
        for _ in parse_descriptor_params(dyn_desc) {
            let e = if sim.state.stack.is_empty() {
                let id = sim.state.next_id;
                sim.state.next_id += 1;
                sim::StackEntry { expr: raw(format!("{}::default()", ir::anchors::OBJECT)), ty: RsType::Object, id }
            } else {
                sim.pop()?
            };
            caps.push(e);
        }
    }
    caps.reverse();
    Ok(caps)
}

/// 实现方法在注册表中的形态：(是否实例方法, 有无泛型签名, 类有效类型形参, 形参签名类型)
fn impl_shape(env: &InstrEnv, ci: Option<&ClassInfo>, mname: &str, desc: &str, is_ctor: bool) -> (bool, bool, Vec<String>, Vec<RsType>) {
    let Some(ci) = ci else {
        return (false, false, Vec::new(), Vec::new());
    };
    let Some(m) = ci.methods().iter().find(|m| m.name == mname && m.desc == desc) else {
        return (false, false, Vec::new(), Vec::new());
    };
    let tparams = env.ctx.ty.effective_class_type_params(ci).to_vec();
    let sig = env.ctx.ty.method_sig_types(ci, m, &tparams).params;
    (!m.is_static() && !is_ctor, !ty::registry::method_signature(m).is_empty(), tparams, sig)
}

/// invokedynamic 的 lambda / 其余形态分支；`is_lambda`：引导方法属 `[indy] lambda` 类
pub(super) fn gen_lambda(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, site: &IndySite, is_lambda: bool) -> InstrResult<()> {
    let idx = site.cp_index(env)?;
    let caps = pop_captures(sim, site.desc)?;
    let (sam_desc, impl_ref) = lambda_args(site, is_lambda);
    if impl_ref.is_empty() || sam_desc.is_empty() {
        // 短期占位（无实现方法信息，如 native 类引导方法）
        sim.emit(raw_stmt(format!("/* TODO: invokedynamic {idx} */")));
        if parse_descriptor_return(site.desc) != "V" {
            sim.push(raw(format!("{}::default()", ir::anchors::OBJECT)), RsType::Object);
        }
        return Ok(());
    }
    let Some((impl_cls, impl_mname, impl_desc)) = split_impl(&impl_ref) else {
        sim.emit(raw_stmt(format!("/* TODO: invokedynamic {idx} (impl parse failed) */")));
        if parse_descriptor_return(&sam_desc) != "V" {
            sim.push(raw(format!("{}::default()", ir::anchors::OBJECT)), RsType::Object);
        }
        return Ok(());
    };
    // 接口重声明的根类公开方法（`handle::equals` → ProcessHandle.equals）：接口不发射这类
    // 成员（经根类 vtable 分派），实现句柄按 JVM 方法解析落到根类
    let impl_is_iface = env.ctx.reg().get(impl_cls).is_some_and(|c| c.is_interface());
    let impl_cls = if impl_is_iface
        && env.ctx.facts.root_virtual.contains(&(impl_mname.to_string(), crate::owner::param_part(impl_desc).to_string()))
    {
        ty::consts::OBJECT
    } else {
        impl_cls
    };
    // G-10：实现方法名取 lambda_impl_rust_name 单一来源（定义侧同源取名），调用点引用名在此
    // 登记，生成收尾由账本断言两侧恒等
    let impl_rust_name = lambda_impl_rust_name(&env.ctx, impl_cls, impl_mname, impl_desc)?;
    log.push(Effect::LambdaRef {
        class: impl_cls.to_string(),
        method: impl_mname.to_string(),
        rust_name: impl_rust_name.clone(),
    });
    let is_ctor = impl_mname == "<init>";
    let ci = env.ctx.reg().get(impl_cls);
    let (is_instance, has_generic_sig, tparams, sig_types) = impl_shape(env, ci, impl_mname, impl_desc, is_ctor);
    let sam_params = parse_descriptor_params(&sam_desc);
    let mut lam = Lam {
        impl_cls: impl_cls.to_string(),
        impl_mname: impl_mname.to_string(),
        impl_desc: impl_desc.to_string(),
        impl_rust_name,
        ci,
        is_ctor,
        is_instance,
        has_generic_sig,
        tparams,
        sig_types,
        impl_params: parse_descriptor_params(impl_desc),
        sam_ret: parse_descriptor_return(&sam_desc).to_string(),
        sam_names: (0..sam_params.len()).map(|i| format!("_la{i}")).collect(),
        sam_params,
        sam_desc,
        cap_names: (0..caps.len()).map(|i| format!("__lam_cap{idx}_{i}")).collect(),
        cap_tys: caps.iter().map(|c| c.ty.clone()).collect(),
    };
    // Java 捕获的是引用副本：被捕获的局部变量在闭包创建后仍可使用，按 Clone 捕获而非 move
    let cap_stmts: Vec<String> = caps
        .iter()
        .zip(&lam.cap_names)
        .map(|(c, n)| format!("let {n} = Clone::clone(&{});", text(env, &c.expr)))
        .collect();
    if lam.ci.is_none() && !lam.is_ctor {
        // 实现类不在注册表（运行时手写层类）：其实例方法是 &self 形态（描述符不含接收者）。
        // 接收者来自 SAM 首参（未绑定 X::m）或捕获首值（绑定 recv::m / this::m，`handle::equals`）；
        // 两者合计：捕获数 + SAM 实参数恰比描述符形参多一个（接收者）
        if lam.cap_names.len() + lam.sam_params.len() == lam.impl_params.len() + 1 {
            lam.is_instance = true;
        }
    }
    let call = super::lambda_args::call_args(env, &lam)?;
    for s in cap_stmts {
        sim.emit(raw_stmt(s));
    }
    let body = super::lambda_body::closure_body(env, sim, &lam, &call)?;
    let value = super::lambda_body::boxed_closure(env, log, site, &lam, body)?;
    let var = format!("__lam_{idx}");
    sim.emit(crate::build::let_typed(crate::build::id(&var)?, Some(sim::exprs::object_type()?), value));
    sim.push(crate::build::var(&var)?, RsType::Object);
    Ok(())
}

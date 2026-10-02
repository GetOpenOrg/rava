//! invokedynamic / monitor / athrow / nop / wide（← `instr/sim/dynamic.py`）。
//!
//! invokedynamic 按引导方法分类（`vm_intrinsics.toml [indy]`，[`IndyKind`]）分派：
//! concat → [`concat`]；type_switch / enum_switch → [`type_switch`]；object_methods →
//! [`object_methods`]；lambda → [`lambda`]。清单外的引导方法（其 Java 体由闭包分析器照常分析，
//! 生成侧无运行期链接）→ 精确存根 `panic!("stub: 引导类.方法:描述符")`，命中即报出引导方法。Python 在类文件解析期拼进指令注释的
//! 引导信息（`indy:` / `template:` / `tconsts:` / `impl:` / `samtype:` / `tslabels:` 令牌）
//! 在这里直接从当前类的 BootstrapMethods 表读取，取值口径与 `classfile._parse_bootstrap_methods`
//! 一致。

mod boxing;
mod concat;
mod lambda;
mod lambda_args;
mod lambda_body;
mod object_methods;
mod type_switch;

use classfile::{BootstrapMethod, Const, Insn, Operand};
use input::manifest::IndyKind;
use ir::{Expr, Stmt};
use sim::StackSim;
use type_switch::SwitchKind;

use crate::build::{call, text, ty_text};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::log::{Audit, InstrLog};
use crate::text::py_float_repr;

/// 运行时错误载体（`return Err(JvmError::from(t))`）
const JVM_ERROR: &str = "JvmError";

/// 一个 invokedynamic 调用点
pub(super) struct IndySite<'a> {
    /// 调用点名（`makeConcatWithConstants` / SAM 方法名 / `typeSwitch`）
    pub name: &'a str,
    /// 动态描述符（捕获值 → 函数式接口 / 拼接实参 → String / (selector, restart) → int）
    pub desc: &'a str,
    /// 调用点的常量池下标（指令操作数携带）
    pub cp_index: u16,
    /// 调用点指令偏移（JVM 每条 invokedynamic 指令各自链接：lambda 隐藏类身份按指令区分）
    pub pc: u32,
    pub bsm: Option<&'a BootstrapMethod>,
}

/// 数值常量的 Python `str(value)` 文本（Integer / Long / Float / Double；其余 → None）
pub(super) fn numeric_const_text(c: &Const) -> Option<String> {
    match c {
        Const::Int(i) => Some(i.to_string()),
        Const::Long(l) => Some(l.to_string()),
        Const::Float(bits) => Some(py_float_repr(f64::from(f32::from_bits(*bits)))),
        Const::Double(bits) => Some(py_float_repr(f64::from_bits(*bits))),
        _ => None,
    }
}

/// 基本类型描述符 → 装箱类 binary 名（`classfile._PRIM_CLASS_TO_WRAPPER`）
pub(super) fn wrapper_of(desc: &str) -> Option<&'static str> {
    match desc.as_bytes() {
        [b] => ty::consts::boxed_class(*b),
        _ => None,
    }
}

#[track_caller]
pub(super) fn raw_stmt(s: String) -> Stmt {
    Stmt::raw(s)
}

#[track_caller]
pub(super) fn raw(s: impl Into<String>) -> Expr {
    Expr::raw(s.into())
}

/// monitorenter / monitorexit 的操作数 → 保持对象身份的 Object 表达式（`_monitor_operand`）：
/// - `this`（&Self / owned Self）：`Object::from(Clone::clone(this))`——vtable 上下文按
///   `Clone::clone(this)` 子串路由到 wrapper 重建；
/// - 其余：`Into::<Object>::into(Clone::clone(&(expr)))`（Object 自反 / wrapper / String /
///   Class / JArray 均满足 blanket From）。
///
/// 同一对象的多次装箱共享存储身份（`__identity` 单元），监视器挂接点唯一。
fn monitor_operand(env: &InstrEnv, e: &Expr) -> String {
    let obj = ir::anchors::OBJECT;
    if e.is_var_named("this") {
        return format!("{obj}::from(Clone::clone(this))");
    }
    format!("Into::<{obj}>::into(Clone::clone(&({})))", text(env, e))
}

/// athrow：被抛出的就是栈顶对象本身，`return Err(JvmError::from(t));`
fn athrow(sim: &mut StackSim) -> InstrResult<()> {
    let e = sim.pop()?.expr;
    // 局部变量被抛出：按值克隆（Java 引用无 move 语义）。java_try! 的 catch 体改写 return
    // 的控制流后，借用检查不再视 `return Err(from(t))` 为路径终点——同一 catch 体中之后
    // 对 t 的使用会报 E0382
    let thrown = match &e {
        Expr::Var(v) if v.as_str() == "this" => sim::exprs::clone_plain(e)?,
        Expr::Var(_) => sim::exprs::clone_ref(e)?,
        _ => e,
    };
    let err = call(&[JVM_ERROR, "from"], vec![thrown])?;
    sim.emit(Stmt::Return(Some(call(&["Err"], vec![err])?)))?;
    Ok(())
}

fn invokedynamic(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<()> {
    let Operand::InvokeDynamic { index, bsm, name, desc } = &ins.operand else {
        return Err(InstrError::BadInsn("invokedynamic 缺调用点操作数".to_string()));
    };
    let bm = env.ctx.class_file().and_then(|cf| cf.bootstrap_methods.get(usize::from(*bsm)));
    // 引导方法分类键 `类.方法`（[indy] 清单；type_switch 是 native 的细分，清单装载时已优先）
    let kind = bm.and_then(|b| env.ctx.rt.indy_kind(&format!("{}.{}", b.handle.member.owner, b.handle.member.name)));
    let site = IndySite { name, desc, cp_index: *index, pc: ins.offset, bsm: bm };
    match kind {
        Some(IndyKind::Concat) => concat::string_concat(env, sim, log, &site),
        Some(IndyKind::TypeSwitch) => type_switch::gen_type_switch(env, sim, &site, SwitchKind::Type),
        Some(IndyKind::EnumSwitch) => type_switch::gen_type_switch(env, sim, &site, SwitchKind::Enum),
        Some(IndyKind::ObjectMethods) => object_methods::gen_object_methods(env, sim, log, &site),
        Some(IndyKind::Lambda) => lambda::gen_lambda(env, sim, log, &site),
        Some(IndyKind::Native) | None => bootstrap_stub(env, sim, &site),
    }
}

/// 无生成侧翻译的引导方法调用点：弹出动态实参，以精确存根占据调用点结果
/// （`panic!` 的 `!` 类型强转为调用点返回类型）
fn bootstrap_stub(env: &InstrEnv, sim: &mut StackSim, site: &IndySite) -> InstrResult<()> {
    let Some(bm) = site.bsm else {
        return Err(InstrError::BadInsn(format!("invokedynamic {}{} 缺引导方法", site.name, site.desc)));
    };
    for _ in ty::type_map::parse_descriptor_params(site.desc) {
        sim.pop()?;
    }
    let m = &bm.handle.member;
    let stub = format!("panic!(\"stub: {}.{}:{}\")", m.owner, m.name, m.desc);
    let ret = ty::type_map::parse_descriptor_return(site.desc);
    if ret == "V" {
        sim.emit(raw_stmt(format!("{stub};")))?;
        return Ok(());
    }
    let t = env.ctx.ty.jvm_to_rust(ret);
    let v = sim.fresh(&format!("__indy{}_", site.cp_index))?;
    sim.emit(raw_stmt(format!("let {v}: {} = {stub};", ty_text(env, &t))))?;
    sim.push(Expr::Var(v), t);
    Ok(())
}

/// 本组指令；非本组 → Ok(false)
pub fn sim_dynamic(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    match ins.name() {
        "invokedynamic" => invokedynamic(env, sim, log, ins)?,
        // 同步（S-20 真实化：可重入监视器，单线程语义不变）。JVMS §6.5：弹出 objectref，
        // 进入其监视器（null → NPE 由运行时承载）
        "monitorenter" => {
            // [equiv-audit] monitor-mt：条件等价——多线程互斥语义（S-11）
            log.audit(Audit::MonitorMt);
            let e = sim.pop()?;
            let op = monitor_operand(env, &e.expr);
            sim.emit(raw_stmt(format!("{op}.monitor_enter()?;")))?;
        }
        "monitorexit" => {
            let e = sim.pop()?;
            let op = monitor_operand(env, &e.expr);
            sim.emit(raw_stmt(format!("{op}.monitor_exit()?;")))?;
        }
        "nop" | "wide" => {}
        "athrow" => athrow(sim)?,
        _ => return Ok(false),
    }
    Ok(true)
}

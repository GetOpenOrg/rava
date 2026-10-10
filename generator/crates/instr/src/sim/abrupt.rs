//! 控制流终点（folds `null_recv` / `noreturn_calls`）：分析器定论不会正常返回的调用点。
//!
//! - [`null_recv`]：接收者恒为 null 的虚调用——不翻译调用（被调方不在调用链上），弹出实参与接收者
//!   （副作用按求值序保留）后 `return Err(__null_recv(&recv, "被调方"));`。运行时核对接收者：
//!   确为 null → NullPointerException；否则 panic 报出违约点（分析器缺写入来源），不静默当作 NPE。
//! - [`noreturn`]：调用照常翻译，返回值丢弃，其后 `__noreturn("被调方");`（返回 `!`）——被调方若违反
//!   分析结论而正常返回，panic 报出违约点，不会继续执行已删去的代码。
//! - [`no_class_def`]：解析的类不在类路径上的指令（folds `no_class`）——不翻译，弹出其操作数（副作用按求值序
//!   保留）后 `return Err(__no_class_def("类内部名"));`，与 JVM 解析失败同样抛 NoClassDefFoundError，
//!   覆盖它的处理器照常捕获。

use classfile::{Insn, Operand};
use ir::{Expr, Lit, Stmt};
use sim::StackSim;

use crate::build::{call, let_discard, text};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::log::InstrLog;
use crate::text::looks_effectful;

fn callee(call: &Insn) -> InstrResult<(String, usize)> {
    let Operand::Method(m, _) = &call.operand else {
        return Err(InstrError::BadInsn(format!("控制流终点 pc={} 不是调用指令", call.offset)));
    };
    Ok((m.to_string(), ty::type_map::parse_descriptor_params(&m.desc).len()))
}

/// 接收者恒为 null 的虚调用点：抛 NullPointerException
pub fn null_recv(env: &InstrEnv, sim: &mut StackSim, call_insn: &Insn) -> InstrResult<()> {
    let (member, nargs) = callee(call_insn)?;
    let mut args = Vec::with_capacity(nargs);
    for _ in 0..nargs.min(sim.state.stack.len()) {
        args.push(sim.pop()?);
    }
    args.reverse();
    let recv = sim.pop()?;
    // 实例方法自身的接收者绑定按 JVMS 恒非 null，分析器不会把它折叠为 null_recv（closure `fold.rs`）；
    // 它也是方法体里唯一的借用形态绑定（`&Self` / `&dyn …VTable`）——到达这里即违反分析结论
    if !sim.cfg.is_static && recv.expr.is_var_named("this") {
        return Err(InstrError::BadInsn(format!("null_recv pc={} 的接收者是实例方法自身的 this（恒非 null）", call_insn.offset)));
    }
    let effectful = |e: &Expr| looks_effectful(&text(env, e));
    // 接收者先于实参求值：两者都有副作用时先把接收者绑定到局部变量
    let recv_expr = if effectful(&recv.expr) && args.iter().any(|a| effectful(&a.expr)) {
        sim.fresh_let("recv", recv.expr, &recv.ty)?
    } else {
        recv.expr
    };
    for a in args {
        if effectful(&a.expr) {
            sim.emit(let_discard(a.expr))?;
        }
    }
    // 其余栈值（局部变量、形参、字段 / 数组读取、调用结果）都按其 RsType 持有，统一取引用交给 `__null_recv(&T)`
    let err = call(&["__null_recv"], vec![Expr::reference(recv_expr), Expr::Lit(Lit::Str(member))])?;
    sim.emit(Stmt::Return(Some(call(&["Err"], vec![err])?)))?;
    Ok(())
}

/// 不返回的调用点：调用照常翻译，其后终止控制流
pub fn noreturn(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, call_insn: &Insn) -> InstrResult<()> {
    let (member, nargs) = callee(call_insn)?;
    let is_static = call_insn.name() == "invokestatic";
    let depth = sim.state.stack.len() as i64 - nargs as i64 - i64::from(!is_static);
    super::sim_insn(env, sim, log, call_insn)?;
    if sim.state.stack.len() as i64 > depth {
        let e = sim.pop()?;
        sim.emit(let_discard(e.expr))?;
    }
    sim.emit(Stmt::Expr(call(&["__noreturn"], vec![Expr::Lit(Lit::Str(member))])?))?;
    Ok(())
}

/// 解析失败点：抛 NoClassDefFoundError（JVMS §5.4.3；消息为类的内部名，与 HotSpot 一致）
pub fn no_class_def(env: &InstrEnv, sim: &mut StackSim, at: &Insn, class: &str) -> InstrResult<()> {
    use classfile::op;
    let operands = match (&at.operand, at.opcode) {
        (Operand::Field(_), op::GETSTATIC) => 0,
        (Operand::Field(_), op::PUTSTATIC | op::GETFIELD) => 1,
        (Operand::Field(_), _) => 2,
        (Operand::Method(m, _), o) => ty::type_map::parse_descriptor_params(&m.desc).len() + usize::from(o != op::INVOKESTATIC),
        (Operand::Class(_), op::ANEWARRAY) => 1,
        (Operand::MultiANewArray(_, dims), _) => usize::from(*dims),
        _ => 0,
    };
    let mut popped = Vec::with_capacity(operands);
    for _ in 0..operands.min(sim.state.stack.len()) {
        popped.push(sim.pop()?);
    }
    for v in popped.into_iter().rev() {
        if looks_effectful(&text(env, &v.expr)) {
            sim.emit(let_discard(v.expr))?;
        }
    }
    let err = call(&["__no_class_def"], vec![Expr::Lit(Lit::Str(class.to_string()))])?;
    sim.emit(Stmt::Return(Some(call(&["Err"], vec![err])?)))?;
    Ok(())
}

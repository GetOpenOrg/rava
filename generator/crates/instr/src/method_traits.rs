//! 方法 trait 使用登记（S7 计划 §9.9 K3）：宏把 wrapper 的可读层成员（方法外壳、构造器、
//! 静态方法、字段访问器、类初始化）放在每类方法 trait `X__Methods` 上，调用点所在文件须
//! 匿名导入该 trait（`use …::X__Methods as _;`）。
//!
//! 登记来源是字节码成员引用的 owner：invoke / 字段指令的 owner，invokedynamic 引导实参中的
//! 方法句柄 owner；沿 owner 的超类链登记到声明该成员的类为止（继承成员在 owner 的 wrapper 上
//! 有转发外壳，调用点也可能落在声明类的 wrapper 上）。接口载体成员同样在 `I__Methods` 上
//!（§9.9 K6 前置项）：接收者载体持有自身声明与继承转发的全部外壳，只登记 owner 接口，
//! 不沿超接口链走。根类（手写）不登记。

use classfile::{Const, Insn, Operand};

use crate::ctx::InstrCtx;

/// 一条指令引用的方法 trait（登记进当前文件作用域；无作用域时无操作）
pub fn record_insn(ctx: &InstrCtx, ins: &Insn) {
    match &ins.operand {
        Operand::Method(m, _) => record_member(ctx, &m.owner, &m.name, Some(&m.desc)),
        Operand::Field(f) => record_member(ctx, &f.owner, &f.name, None),
        Operand::InvokeDynamic { bsm, .. } => {
            let Some(cf) = ctx.class_file() else { return };
            let Some(b) = cf.bootstrap_methods.get(usize::from(*bsm)) else { return };
            for a in &b.args {
                if let Const::MethodHandle(h) = a {
                    let field = h.member.desc.find('(').is_none();
                    record_member(ctx, &h.member.owner, &h.member.name, (!field).then_some(h.member.desc.as_str()));
                }
            }
        }
        _ => {}
    }
}

/// 成员引用 `owner.name[desc]`：owner 起沿超类链到声明类（含）登记方法 trait。
/// `desc` = None 表示字段引用。接口（载体成员在 `I__Methods` 上，K6 前置项）只登记
/// owner 自身——继承成员的转发外壳由宏在 owner 载体上生成，调用点落在 owner 载体上。
/// 手写伴生块（§9.9 K4-2a）按成员名一并登记 `__XHw{N}`（命中声明块即止，同链遍历口径）
pub fn record_member(ctx: &InstrCtx, owner: &str, name: &str, desc: Option<&str>) {
    let reg = ctx.reg();
    let mut cur = owner;
    while let Some(ci) = reg.get(cur) {
        if cur == ty::consts::OBJECT {
            return;
        }
        ctx.ty.use_methods(cur);
        if ci.is_interface() {
            ctx.ty.use_hw(cur, name);
            return;
        }
        let declares = match desc {
            Some(d) => ci.methods().iter().any(|m| m.name == name && m.desc == d),
            None => ci.fields().iter().any(|f| f.name == name),
        };
        if declares {
            ctx.ty.use_hw(cur, name);
            return;
        }
        cur = ci.super_class();
    }
}

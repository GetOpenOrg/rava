//! 字段读写与 `new` / `invokespecial`（← `instr/sim/fields.py`）。
//!
//! 字段读写走宏生成的访问器（`__get_x()` / `__set_x(v)` / `Cls::x()` / `Cls::set_x(v)`）；
//! 继承字段由子类的转发访问器统一暴露，无需 `_super` 路径。声明类型恢复见 [`types`]，
//! 写入值转换见 [`store`]。
//!
//! Python 的 getstatic `$assertionsDisabled` 常量分支不可达（比较对象是已 `safe_ident` 的
//! 字段名 `_assertionsDisabled`），按实际行为走普通 static 读取，不移植该死分支。

mod store;
mod types;

use classfile::{Insn, MemberRef, Operand};
use ir::{Expr, Ident, Path, StaticFieldRef};
use sim::StackSim;
use ty::RsType;

use crate::build::{call_path, expr_stmt, ir_ty, mcall, seg, text, try_};
use crate::env::InstrEnv;
use crate::error::{InstrError, InstrResult};
use crate::invoke::special::{class_segs, gen_invokespecial};
use crate::invoke::{sig, CallRef};
use crate::log::InstrLog;
use crate::owner::field_generic_signature;

use store::{coerce_stored_value, StoreCtx};
use types::{resolve_static_field, restore_field_declared_type};

/// 本组指令；非本组 → Ok(false)
pub fn sim_fields(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    match (ins.name(), &ins.operand) {
        ("new", Operand::Class(c)) => {
            let raw = c.strip_prefix("class ").unwrap_or(c).trim();
            sim.push(Expr::NewPending { class: raw.to_string() }, RsType::class(raw, Vec::new()));
        }
        ("invokespecial", Operand::Method(m, _)) => gen_invokespecial(env, sim, log, &CallRef::new(m))?,
        ("getfield", Operand::Field(f)) => getfield(env, sim, f)?,
        ("putfield", Operand::Field(f)) => putfield(env, sim, log, f)?,
        ("getstatic", Operand::Field(f)) => getstatic(env, sim, f)?,
        ("putstatic", Operand::Field(f)) => putstatic(env, sim, log, f)?,
        (name @ ("new" | "invokespecial" | "getfield" | "putfield" | "getstatic" | "putstatic"), op) => {
            return Err(InstrError::BadInsn(format!("{name} 的操作数形态：{op:?}")));
        }
        _ => return Ok(false),
    }
    Ok(true)
}

/// getfield / putfield 接收者判空（JVMS §6.5）：`this` 恒非 null 保持原形，其余经 `__nn()?`
fn null_checked(env: &InstrEnv, recv: Expr) -> InstrResult<Expr> {
    let t = text(env, &recv);
    if t == "this" || t == "self" {
        return Ok(recv);
    }
    Ok(try_(mcall(recv, "__nn", Vec::new())?))
}

/// 字段接收者视图：访问器定义在声明类上。
/// - 不透明（L1）类接收者 → 上转到声明类视图（见 [`crate::opaque`]）
/// - 空引用字面量接收者（分析器折叠出的 null 常量只剩 `Object` 类型）→ 声明类的 null
///   （`<View>::default()`），随后的 `__nn()?` 按 JVM 语义抛 NullPointerException
fn field_receiver_view(env: &InstrEnv, obj_e: Expr, obj_ty: RsType, owner: &str, name: &str) -> (Expr, RsType) {
    let declarer = crate::opaque::field_declarer(env, owner, name);
    if let Some(view) = crate::opaque::receiver_view(env, &obj_ty, &declarer) {
        return (Expr::raw(crate::opaque::upcast_text(env, &text(env, &obj_e), &view)), view);
    }
    if matches!(obj_ty, RsType::Object) && sim::exprs::is_null(&obj_e) {
        let view = crate::opaque::class_view(env, &declarer);
        return (Expr::raw(format!("<{}>::default()", crate::build::ty_text(env, &view))), view);
    }
    (obj_e, obj_ty)
}

/// 存储值节点：未经转换时直接携带栈上节点，经转换的值为 Raw 叶子
fn value_node(env: &InstrEnv, val: Expr, val_str: String) -> Expr {
    if text(env, &val) == val_str {
        val
    } else {
        Expr::raw(val_str)
    }
}

pub(crate) fn getfield(env: &InstrEnv, sim: &mut StackSim, f: &MemberRef) -> InstrResult<()> {
    let ctx = &env.ctx;
    let obj = sim.pop()?;
    // 装箱类擦除特例：接收者已是基本类型时字段访问就是值本身
    if sim::types::is_scalar(&obj.ty) {
        sim.push(obj.expr, obj.ty);
        return Ok(());
    }
    let fname = ty::ident::safe_ident(&f.name);
    let ftype = if f.desc.is_empty() { RsType::Object } else { ctx.ty.jvm_to_rust(&f.desc) };
    // 类型变量接收者：访问器定义在上界类上，经上界视图读字段
    let (obj_e, obj_ty) = sig::type_var_receiver_bound_view(env, sim, obj.expr, obj.ty)?;
    let owner = if f.owner.is_empty() { ctx.class_name } else { &f.owner };
    let (obj_e, obj_ty) = field_receiver_view(env, obj_e, obj_ty, owner, &f.name);
    let ftype = restore_field_declared_type(env, sim, &f.owner, &fname, ftype, Some(&obj_ty));
    let slot = ctx.ty.instance_field_rust_name(owner, &fname);
    let get = mcall(null_checked(env, obj_e)?, &format!("__get_{slot}"), Vec::new())?;
    sim.push(get, ftype);
    Ok(())
}

fn putfield(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, f: &MemberRef) -> InstrResult<()> {
    let ctx = &env.ctx;
    let val = sim.pop()?;
    let obj = sim.pop()?;
    let fname = ty::ident::safe_ident(&f.name);
    let ftype = if f.desc.is_empty() { RsType::Prim(ty::Prim::I32) } else { ctx.ty.jvm_to_rust(&f.desc) };
    let (obj_e, obj_ty) = sig::type_var_receiver_bound_view(env, sim, obj.expr, obj.ty)?;
    let owner = if f.owner.is_empty() { ctx.class_name } else { &f.owner };
    let (obj_e, obj_ty) = field_receiver_view(env, obj_e, obj_ty, owner, &f.name);
    let ftype = restore_field_declared_type(env, sim, &f.owner, &fname, ftype, Some(&obj_ty));
    let slot_gsig = field_generic_signature(ctx.reg(), owner, &fname);
    let obj_text = text(env, &obj_e);
    let class_tps = sim.cfg.class_type_params.clone();
    let sc = StoreCtx { obj_text: &obj_text, slot_is_type_var: slot_gsig.starts_with('T'), class_tps: &class_tps };
    let val_str = coerce_stored_value(env, log, &val.expr, &val.ty, &ftype, &sc)?;
    let slot = ctx.ty.instance_field_rust_name(owner, &fname);
    let set = mcall(null_checked(env, obj_e)?, &format!("__set_{slot}"), vec![value_node(env, val.expr, val_str)])?;
    sim.emit(expr_stmt(set))?;
    Ok(())
}

/// static 字段读取表达式与其类型（getstatic 与引导方法标签的枚举常量共用）
pub(crate) fn static_field_read(env: &InstrEnv, owner: &str, name: &str, desc: &str) -> InstrResult<(Expr, RsType)> {
    let sf = resolve_static_field(env, owner, name, desc);
    let turbofish = sf.turbofish.iter().map(|t| ir_ty(env, t)).collect::<InstrResult<Vec<_>>>()?;
    let r = StaticFieldRef { class: sf.class, field: Ident::new(sf.accessor)?, ty: ir_ty(env, &sf.ty)?, turbofish };
    Ok((Expr::StaticField(r), sf.ty))
}

fn getstatic(env: &InstrEnv, sim: &mut StackSim, f: &MemberRef) -> InstrResult<()> {
    let (e, t) = static_field_read(env, &f.owner, &f.name, &f.desc)?;
    sim.push(e, t);
    Ok(())
}

fn putstatic(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, f: &MemberRef) -> InstrResult<()> {
    let val = sim.pop()?;
    // Java 引用赋值无 move 语义：源局部变量仍可被使用 → setter 实参包 Clone::clone 保活（E0382）
    let val_e = sim::exprs::clone_moved_var(val.expr, &val.ty)?;
    let sf = resolve_static_field(env, &f.owner, &f.name, &f.desc);
    let node = if sim::types::is_scalar(&sf.ty) {
        // JVM 操作数栈上窄整型都是 int，写入字段时按字段描述符还原
        crate::coerce::value(val_e, &val.ty, &sf.ty)
    } else {
        let class_tps = sim.cfg.class_type_params.clone();
        let sc = StoreCtx { obj_text: "", slot_is_type_var: false, class_tps: &class_tps };
        let val_str = coerce_stored_value(env, log, &val_e, &val.ty, &sf.ty, &sc)?;
        value_node(env, val_e, val_str)
    };
    // static 写入 → 宏生成的 set_xxx 访问器（入口触发类初始化，JVMS §5.5）
    let turbofish = sf.turbofish.iter().map(|t| ir_ty(env, t)).collect::<InstrResult<Vec<_>>>()?;
    let mut segs = class_segs(&env.ctx.short(&sf.class), turbofish)?;
    segs.push(seg(&format!("set_{}", sf.accessor))?);
    sim.emit(expr_stmt(try_(call_path(Path::new(segs), vec![node]))))?;
    Ok(())
}


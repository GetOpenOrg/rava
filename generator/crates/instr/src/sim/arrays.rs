//! 数组创建 / 访问（← `instr/sim/arrays.py`）。
//!
//! 接收者擦除为 Object（多维数组元素、Object[] 持有数组引用）走 Object 的数组访问 API
//! （`array_load_*` / `array_store_*` / `array_length`，元素类型由操作码决定）；
//! `JArray` 接收者走类型化 `get` / `set` / `len`。Python 以 RawExpr / RawStmt 拼接的形态
//! 这里同样以 [`ir::Raw`] 承载（文本逐字一致），Python 已节点化的形态以节点构造。

use classfile::{Insn, Operand};
use ir::anchors::OBJECT;
use ir::{Expr, Ident, Stmt};
use sim::exprs::{clone_ref, from_call, object_from};
use sim::{StackEntry, StackSim};
use ty::{consts, Prim, RsType};

use crate::build::{call_path, expr_stmt, id, ir_ty, let_mut, mcall, seg, seg_g, text, try_, ty_text};
use crate::coerce::{to_i32, to_object};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy::is_subtype;
use crate::log::{Audit, InstrLog};
use crate::sim::control::{erased_base_ty, raw};

/// 数组创建指令（neg-array 口径：创建点全部经 `try_new` / `try_new_with` 的负长度检查）
const CREATE_OPS: [&str; 3] = ["newarray", "anewarray", "multianewarray"];

/// 数组访问指令（null-array 口径：对 null 数组引用抛 NPE 的发射点，JVMS §6.5）
const ACCESS_OPS: [&str; 17] = [
    "arraylength", "iaload", "laload", "faload", "daload", "aaload", "baload", "saload", "caload", "iastore",
    "lastore", "fastore", "dastore", "aastore", "bastore", "sastore", "castore",
];

/// newarray 基本类型代码（JVMS §6.5 newarray）→ 元素类型；未知代码按 int
fn newarray_prim(code: u8) -> Prim {
    match code {
        4 => Prim::Bool,
        5 => Prim::U16,
        6 => Prim::F32,
        7 => Prim::F64,
        8 => Prim::I8,
        9 => Prim::I16,
        11 => Prim::I64,
        _ => Prim::I32,
    }
}

/// 弹出数组下标：JVM 下标恒为 int，窄类型提升为 i32
fn pop_index(env: &InstrEnv, sim: &mut StackSim) -> InstrResult<Expr> {
    let e = sim.pop()?;
    Ok(to_i32(env, e.expr, &e.ty))
}

/// 数组指令接收者是否擦除为 Object（非 `JArray`）
fn is_object_receiver(t: &RsType) -> bool {
    !sim::types::is_jvm_array(t)
}

/// 数组元素类型（非数组 → Object）
fn elem_of(t: &RsType) -> RsType {
    match t {
        RsType::Array(e) => (**e).clone(),
        _ => RsType::Object,
    }
}

/// `let mut v: JArray<E> = JArray::<E>::try_new(n)?;`（try_new 负长度抛
/// NegativeArraySizeException，S-8）
fn new_array_let(env: &InstrEnv, v: Ident, elem: &RsType, count: Expr) -> InstrResult<Stmt> {
    let e = ir_ty(env, elem)?;
    let ctor = ir::Path::new(vec![seg_g(ir::anchors::ARRAY, vec![e])?, seg("try_new")?]);
    let value = try_(call_path(ctor, vec![count]));
    Ok(let_mut(v, Some(ir_ty(env, &RsType::array(elem.clone()))?), value))
}

/// 本组指令；非本组 → Ok(false)
pub fn sim_arrays(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    let op = ins.name();
    // [equiv-audit] 只读计数，不改发射内容
    if CREATE_OPS.contains(&op) {
        log.audit(Audit::NegArray);
    } else if ACCESS_OPS.contains(&op) {
        log.audit(Audit::NullArray);
    } else {
        return Ok(false);
    }
    match op {
        "newarray" => {
            let count = pop_index(env, sim)?;
            let code = match ins.operand {
                Operand::NewArray(c) => c,
                _ => 10,
            };
            new_array(env, sim, RsType::Prim(newarray_prim(code)), count)?;
        }
        "anewarray" => {
            let count = pop_index(env, sim)?;
            let comment = match &ins.operand {
                Operand::Class(c) => c.as_str(),
                _ => "",
            };
            // 组件类型按描述符映射（组件本身是数组类时操作数已是元素描述符）；泛型类填
            // Object 实参、接口发载体（擦除运行时形态），异实参值由 aastore 存储检查兜住
            let elem = if comment.is_empty() || comment == consts::OBJECT {
                RsType::Object
            } else if comment.starts_with('[') {
                env.ctx.ty.jvm_to_rust(comment)
            } else {
                env.ctx.ty.jvm_to_rust(&format!("L{comment};"))
            };
            new_array(env, sim, elem, count)?;
        }
        "multianewarray" => multi_new_array(env, sim, ins)?,
        "iastore" | "lastore" | "fastore" | "dastore" => prim_store(env, sim, op)?,
        "aastore" => aastore(env, sim)?,
        "bastore" | "sastore" | "castore" => narrow_store(env, sim, op)?,
        "arraylength" => {
            let arr = sim.pop()?;
            let a = text(env, &arr.expr);
            // Object::array_length / JArray::len 均返回 Result（null 数组引用抛 NPE）
            let api = if is_object_receiver(&arr.ty) { "array_length" } else { "len" };
            sim.push(raw(format!("({a}.{api}()?)")), RsType::Prim(Prim::I32));
        }
        _ => load(env, sim, op)?,
    }
    Ok(true)
}

fn new_array(env: &InstrEnv, sim: &mut StackSim, elem: RsType, count: Expr) -> InstrResult<()> {
    let v = sim.fresh("_arr")?;
    let stmt = new_array_let(env, v.clone(), &elem, count)?;
    sim.emit(stmt)?;
    sim.push(Expr::Var(v), RsType::array(elem));
    Ok(())
}

/// multianewarray：给出长度的各维逐层构造（每行独立数组对象），未给长度的内层维保持 null；
/// 任一维为负抛 NegativeArraySizeException（内层 Err 经逐行闭包返回值传播）
fn multi_new_array(env: &InstrEnv, sim: &mut StackSim, ins: &Insn) -> InstrResult<()> {
    let (desc, dims) = match &ins.operand {
        Operand::MultiANewArray(d, n) => (d.as_str(), usize::from(*n)),
        _ => ("", 2),
    };
    let mut sizes = Vec::with_capacity(dims);
    for _ in 0..dims {
        let e = pop_index(env, sim)?;
        sizes.push(text(env, &e));
    }
    sizes.reverse();
    let arr_t = if desc.starts_with('[') {
        env.ctx.ty.jvm_to_rust(desc)
    } else {
        RsType::array(RsType::array(RsType::Prim(Prim::I32)))
    };
    let mut levels = Vec::with_capacity(dims);
    let mut cur = arr_t.clone();
    for _ in 0..dims {
        let next = elem_of(&cur);
        levels.push(ty_text(env, &cur));
        cur = next;
    }
    let turbofish = |t: &str| t.replacen("JArray<", "JArray::<", 1);
    let (Some(last_t), Some(last_n)) = (levels.last(), sizes.last()) else {
        return Err(crate::error::InstrError::BadInsn("multianewarray 维数为 0".to_string()));
    };
    let mut init = format!("{}::try_new({last_n})", turbofish(last_t));
    for lv in (0..dims.saturating_sub(1)).rev() {
        init = format!("{}::try_new_with({}, || {init})", turbofish(&levels[lv]), sizes[lv]);
    }
    let v = sim.fresh("_arr")?;
    sim.emit(Stmt::raw(format!("let mut {v}: {} = {init}?;", ty_text(env, &arr_t))))?;
    sim.push(Expr::Var(v), arr_t);
    Ok(())
}

/// 弹出 (值, 下标, 数组)
fn pop_store(env: &InstrEnv, sim: &mut StackSim) -> InstrResult<(StackEntry, Expr, StackEntry)> {
    let val = sim.pop()?;
    let idx = pop_index(env, sim)?;
    let arr = sim.pop()?;
    Ok((val, idx, arr))
}

/// `recv.method(idx, val)?;`
fn emit_call(sim: &mut StackSim, recv: Expr, method: &str, idx: Expr, val: Expr) -> InstrResult<()> {
    sim.emit(expr_stmt(try_(mcall(recv, method, vec![idx, val])?)))?;
    Ok(())
}

/// iastore / lastore / fastore / dastore：窄类型局部存入宽数组时显式加宽
fn prim_store(env: &InstrEnv, sim: &mut StackSim, op: &str) -> InstrResult<()> {
    let (val, idx, arr) = pop_store(env, sim)?;
    let (elem_prim, api) = match op {
        "iastore" => ("i32", "array_store_int"),
        "lastore" => ("i64", "array_store_long"),
        "fastore" => ("f32", "array_store_float"),
        _ => ("f64", "array_store_double"),
    };
    let val_prim = ty_text(env, &val.ty);
    let v = if sim::types::is_scalar(&val.ty) && val_prim != elem_prim {
        raw(format!("(({}) as {elem_prim})", text(env, &val.expr)))
    } else {
        val.expr
    };
    let method = if is_object_receiver(&arr.ty) { api } else { "set" };
    emit_call(sim, arr.expr, method, idx, v)
}

/// bastore / sastore / castore（boolean[] 与 byte[] 共用 bastore）
fn narrow_store(env: &InstrEnv, sim: &mut StackSim, op: &str) -> InstrResult<()> {
    let (val, idx, arr) = pop_store(env, sim)?;
    let (a, i, v) = (text(env, &arr.expr), text(env, &idx), text(env, &val.expr));
    let code = if is_object_receiver(&arr.ty) {
        // 擦除接收者：由 Object 的数组 API 按运行时元素类型分派
        let api = match op {
            "bastore" => "array_store_byte",
            "sastore" => "array_store_short",
            _ => "array_store_char",
        };
        format!("{a}.{api}({i}, ({v}) as i32)?;")
    } else if op == "bastore" && ty_text(env, &arr.ty) == "JArray<bool>" {
        let coerced = if matches!(val.ty, RsType::Prim(Prim::Bool)) { v } else { format!("(({v}) as i8 != 0)") };
        format!("{a}.set({i}, {coerced})?;")
    } else {
        let t = match op {
            "bastore" => "i8",
            "sastore" => "i16",
            _ => "u16",
        };
        format!("{a}.set({i}, ({v}) as {t})?;")
    };
    sim.emit(Stmt::raw(code))?;
    Ok(())
}

/// xaload
fn load(env: &InstrEnv, sim: &mut StackSim, op: &str) -> InstrResult<()> {
    let idx = pop_index(env, sim)?;
    let arr = sim.pop()?;
    let (a, i) = (text(env, &arr.expr), text(env, &idx));
    let obj = is_object_receiver(&arr.ty);
    let (api, t) = match op {
        "iaload" => ("array_load_int", RsType::Prim(Prim::I32)),
        "baload" => ("array_load_byte", RsType::Prim(Prim::I32)),
        "saload" => ("array_load_short", RsType::Prim(Prim::I32)),
        "caload" => ("array_load_char", RsType::Prim(Prim::I32)),
        "laload" => ("array_load_long", RsType::Prim(Prim::I64)),
        "faload" => ("array_load_float", RsType::Prim(Prim::F32)),
        "daload" => ("array_load_double", RsType::Prim(Prim::F64)),
        // aaload：JArray::get 内部已 clone，直接使用返回值
        _ => ("array_load_object", if obj { RsType::Object } else { elem_of(&arr.ty) }),
    };
    let e = if obj {
        // Object API 已按 JVM 栈形态返回符号 / 零扩展后的 i32
        format!("{a}.{api}({i})?")
    } else if matches!(op, "baload" | "saload" | "caload") {
        format!("({a}.get({i})? as i32)")
    } else {
        format!("{a}.get({i})?")
    };
    sim.push(raw(e), t);
    Ok(())
}

/// aastore
fn aastore(env: &InstrEnv, sim: &mut StackSim) -> InstrResult<()> {
    let (val, idx, arr) = pop_store(env, sim)?;
    let val_s = ty_text(env, &val.ty);
    if is_object_receiver(&arr.ty) {
        // 擦除为 Object 的数组接收者：值统一装箱后经 array_store_object 转发（按源元素类型做存储检查）
        let v = if matches!(val_s.as_str(), OBJECT | "()") { val.expr } else { to_object(env, val.expr, &val.ty, true)? };
        return emit_call(sim, arr.expr, "array_store_object", idx, v);
    }
    let elem = elem_of(&arr.ty);
    let elem_s = ty_text(env, &elem);
    let mut val_ty = val.ty;
    if matches!(&val_ty, RsType::Array(e) if **e == RsType::Object) && sim::types::is_jvm_array(&elem) && elem_s != val_s {
        // `spine[i] = (E[]) new Object[n]`：新建数组在创建处即按槽位元素类型实例化
        if retype_fresh_array(env, sim, &val.expr, &elem)? {
            val_ty = elem.clone();
        }
    }
    let val_s = ty_text(env, &val_ty);
    let v = if elem_s == OBJECT && !matches!(val_s.as_str(), OBJECT | "()") {
        to_object(env, val.expr, &val_ty, true)?
    } else if elem_s != OBJECT && val_s == OBJECT {
        // 元素静态类型比值更具体（checkcast 被验证器省略的位置）：按对象标识还原
        from_call(clone_ref(val.expr)?)?
    } else if !sim::types::is_scalar(&val_ty) {
        if elem_s != val_s && is_subtype(&env.ctx, &erased_base_ty(&val_ty), &erased_base_ty(&elem)) {
            if matches!(val.expr, Expr::Var(_)) {
                Expr::upcast(val.expr, ir::UpcastWrap::Owned)
            } else {
                mcall(clone_ref(val.expr)?, "into", Vec::new())?
            }
        } else if elem_s != val_s {
            // 值静态类型与元素类型无子型关系：经 Object 边界走 aastore 存储检查（ArrayStoreException，S-4）
            let val_obj = to_object(env, val.expr, &val_ty, true)?;
            let arr_obj = object_from(clone_ref(arr.expr)?)?;
            return emit_call(sim, arr_obj, "array_store_object", idx, val_obj);
        } else {
            clone_ref(val.expr)?
        }
    } else {
        val.expr
    };
    emit_call(sim, arr.expr, "set", idx, v)
}

/// 向前找值变量的创建处 `let mut v: JArray<Object> = JArray::<Object>::try_new(..`，
/// 把元素类型改为槽位元素类型（节点 / Raw 两种形态）；找到 → true
fn retype_fresh_array(env: &InstrEnv, sim: &mut StackSim, val: &Expr, elem: &RsType) -> InstrResult<bool> {
    let name = text(env, val);
    let fresh_decl = format!("let mut {name}: JArray<Object> = JArray::<Object>::try_new(");
    let inner = elem_of(elem);
    let names = sim::TyNames(env);
    let renderer = ir::Renderer::new(&names);
    for si in (0..sim.state.stmts.len()).rev() {
        let replacement = match &sim.state.stmts[si] {
            Stmt::Let(l) if l.name.as_str() == name && renderer.stmt(&sim.state.stmts[si], 0).starts_with(&fresh_decl) => {
                let Some(count) = try_new_count(l.value.as_ref()) else {
                    continue;
                };
                new_array_let(env, id(&name)?, &inner, count)?
            }
            Stmt::Raw(r) if r.as_str().starts_with(&fresh_decl) => Stmt::raw(format!(
                "let mut {name}: {} = JArray::<{}>::try_new({}",
                ty_text(env, elem),
                ty_text(env, &inner),
                &r.as_str()[fresh_decl.len()..]
            )),
            _ => continue,
        };
        sim.state.stmts[si] = replacement;
        return Ok(true);
    }
    Ok(false)
}

/// `JArray::<E>::try_new(n)?` 的长度实参
fn try_new_count(value: Option<&Expr>) -> Option<Expr> {
    match value? {
        Expr::Try(inner) => match &**inner {
            Expr::Call { args, .. } => args.first().cloned(),
            _ => None,
        },
        _ => None,
    }
}

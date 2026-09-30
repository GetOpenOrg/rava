//! checkcast / instanceof（← `instr/sim/control.py`）。跳转指令（if* / goto / switch）由块层
//! 作为块终结解释，不经过本分发。
//!
//! Python 以渲染文本比较类型（`src_name == 'Object'`、`_erased_shape(..) == ..`）；这里的判定
//! 同样作用于 [`ty_text`] 文本（与 Python 同口径），发射一律为结构节点。

use classfile::{Insn, Operand};
use ir::anchors::OBJECT;
use ir::{CastExpr, CastMode, Expr, Lit};
use sim::StackSim;
use ty::{consts, Prim, RsType};

use crate::build::{ir_ty, str_leaf, text, ty_text};
use crate::coerce::{cast_node, to_object};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::hierarchy::{is_subtype, type_binary};
use crate::log::{Effect, InstrLog};

const ACC_FINAL: u16 = 0x0010;

#[track_caller]
pub(crate) fn raw(s: String) -> Expr {
    Expr::raw(s)
}

/// 擦除基名的类型形态（`erased_base` 的类型版）：类清空实参；数组 → 裸名 `JArray`
/// （Python 以基名串 `JArray` 再解析，注册表外 → 不透明装箱）；其余不变
pub(crate) fn erased_base_ty(t: &RsType) -> RsType {
    match t {
        RsType::Array(_) => RsType::Bare { binary: ir::anchors::ARRAY.to_string() },
        other => sim::erase(other),
    }
}

/// 类型文本中的标识符（`re.findall(r'\w+', ..)` / `[A-Za-z_][A-Za-z0-9_]*` 口径）
pub(crate) fn words(s: &str) -> Vec<&str> {
    s.split(|c: char| !(c.is_alphanumeric() || c == '_')).filter(|w| !w.is_empty()).collect()
}

/// 去掉类类型实参后的形态（数组保留元素形态）：`JArray<Entry<K, V>>` → `JArray<Entry>`
/// （`_erased_shape`：反复以 `\b(?!JArray\b)(\w+)<[^<>]*>` → `\1` 替换到不动点）
pub(crate) fn erased_shape(s: &str) -> String {
    let mut cur = s.to_string();
    while let Some((start, lt, end)) = innermost_group(&cur) {
        let word = cur[start..lt].to_string();
        cur.replace_range(start..end, &word);
    }
    cur
}

/// 第一个 `word<无尖括号内容>` 片段（word ≠ `JArray`）：(起点, `<` 位置, 终点)
fn innermost_group(s: &str) -> Option<(usize, usize, usize)> {
    let b = s.as_bytes();
    let is_w = |c: u8| c.is_ascii_alphanumeric() || c == b'_' || c >= 0x80;
    let mut i = 0;
    while i < b.len() {
        if !is_w(b[i]) || (i > 0 && is_w(b[i - 1])) {
            i += 1;
            continue;
        }
        let mut j = i;
        while j < b.len() && is_w(b[j]) {
            j += 1;
        }
        if j < b.len() && b[j] == b'<' && &s[i..j] != ir::anchors::ARRAY {
            if let Some(k) = s[j + 1..].find(['<', '>']) {
                if b[j + 1 + k] == b'>' {
                    return Some((i, j, j + 2 + k));
                }
            }
        }
        i = j;
    }
    None
}

/// 本组指令；非本组 → Ok(false)
pub fn sim_control(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, ins: &Insn) -> InstrResult<bool> {
    match ins.name() {
        "checkcast" => checkcast(env, sim, class_operand(ins))?,
        "instanceof" => instanceof(env, sim, log, class_operand(ins))?,
        _ => return Ok(false),
    }
    Ok(true)
}

fn class_operand(ins: &Insn) -> &str {
    match &ins.operand {
        Operand::Class(c) => c,
        _ => "",
    }
}

/// 指令操作数（binary 名或数组描述符）→ 擦除 Rust 类型
fn operand_type(env: &InstrEnv, comment: &str) -> RsType {
    if comment.starts_with('[') {
        env.ctx.ty.jvm_to_rust(comment)
    } else {
        env.ctx.ty.jvm_to_rust(&format!("L{comment};"))
    }
}

/// 目标类与当前类共享类型变量作用域（内部类沿用外层类的类型变量，其有效类型形参全部是
/// 当前类的类型形参）→ 擦除实例化 `X<Object, ..>` 按目标类自身的形参顺序还原为 `X<K, V>`
fn reinstantiate_shared(env: &InstrEnv, sim: &StackSim, comment: &str, cast: RsType) -> RsType {
    let ctps = &sim.cfg.class_type_params;
    if !ty_text(env, &cast).contains('<') || ctps.is_empty() || env.ctx.reg().is_empty() {
        return cast;
    }
    let stripped = comment.trim_start_matches('[');
    let elem = if stripped == comment {
        stripped
    } else if stripped.starts_with('L') && stripped.len() >= 2 {
        &stripped[1..stripped.len() - 1]
    } else {
        ""
    };
    let Some(ci) = env.ctx.reg().get(elem) else {
        return cast;
    };
    let tps = env.ctx.ty.effective_class_type_params(ci);
    // 接口载体不参与：载体是擦除运行时形态（I<Object> 即 itable 视图）
    let is_carrier = ci.is_interface() && env.ctx.ty.carrier_type(elem).is_some();
    if tps.is_empty() || is_carrier || !tps.iter().all(|tp| ctps.contains(tp)) {
        return cast;
    }
    let erased = RsType::class(elem, RsType::objects(tps.len()));
    let shared = RsType::class(elem, tps.iter().map(|tp| RsType::Param(tp.clone())).collect());
    replace_type(&cast, &erased, &shared)
}

/// 结构内逐处替换（Python 对类型串的 `str.replace`）
fn replace_type(t: &RsType, from: &RsType, to: &RsType) -> RsType {
    if t == from {
        return to.clone();
    }
    match t {
        RsType::Class { binary, args } => {
            RsType::class(binary.clone(), args.iter().map(|a| replace_type(a, from, to)).collect())
        }
        RsType::Array(e) => RsType::array(replace_type(e, from, to)),
        other => other.clone(),
    }
}

fn is_object_or_unit(s: &str) -> bool {
    s == OBJECT || s == "()"
}

fn checkcast(env: &InstrEnv, sim: &mut StackSim, comment: &str) -> InstrResult<()> {
    if comment.is_empty() || sim.state.stack.is_empty() {
        return Ok(());
    }
    let cast_rust = reinstantiate_shared(env, sim, comment, operand_type(env, comment));
    let cast_s = ty_text(env, &cast_rust);
    let entry = sim.pop()?;
    let (mut expr, src_ty) = (entry.expr, entry.ty);
    let src_name = ty_text(env, &src_ty);
    let tgt_ci = if comment.starts_with('[') { None } else { env.ctx.reg().get(comment) };
    // A-4 批次 1：checkcast 到未载体化的接口（目标擦除为 Object）→ try_cast_iface，
    // 栈类型保持擦除记录 Object
    if src_name == OBJECT && tgt_ci.is_some_and(|ci| ci.is_interface()) && env.ctx.ty.carrier_type(comment).is_none() {
        let target = ir_ty(env, &RsType::Object)?;
        let mode = CastMode::CheckedInterface { binary: comment.to_string() };
        sim.push(Expr::CheckCast(CastExpr { expr: Box::new(expr), target, mode, box_first: false }), RsType::Object);
        return Ok(());
    }
    if src_name == OBJECT && !is_object_or_unit(&cast_s) {
        // 源是 Object（擦除边界）：checkcast 语义由 try_cast 按序判定
        expr = cast_node(expr, ir_ty(env, &cast_rust)?, comment, true, false);
    } else if src_name != OBJECT && !is_object_or_unit(&cast_s) && cast_s != src_name
        && erased_shape(&src_name) == erased_shape(&cast_s)
    {
        // 同一擦除类型、仅类型实参不同：值不变；源侧待推断的 `_` 由目标类型给出
        let t = if words(&src_name).contains(&"_") { cast_rust } else { src_ty };
        sim.push(expr, t);
        return Ok(());
    } else if src_name != OBJECT && !is_object_or_unit(&cast_s) && bound_matches(env, sim, &src_name, &cast_rust) {
        // 类型变量值转型到其上界的擦除类：恒成立，值与静态类型都不变
        sim.push(expr, src_ty);
        return Ok(());
    } else if src_name != OBJECT && !is_object_or_unit(&cast_s) && cast_s != src_name {
        if is_subtype(&env.ctx, &erased_base_ty(&cast_rust), &erased_base_ty(&src_ty)) {
            // 合法向下转型：经 Object 边界按目标类取回子类视图（失败 Err，S-1）
            expr = cast_node(expr, ir_ty(env, &cast_rust)?, comment, true, true);
        } else {
            // 静态类型互不为子类型：经 Object 边界按目标类型取回（运行时校验）
            let boxed = to_object(env, str_leaf(expr), &src_ty, true)?;
            expr = cast_node(boxed, ir_ty(env, &cast_rust)?, comment, true, false);
        }
    }
    if cast_s == OBJECT && !is_object_or_unit(&src_name) {
        // 目标擦除为 Object 而值有更精确的静态类型：记录源类型；源是具体类且静态上并未
        // 实现目标接口（交叉转型）→ 接口视图只能经对象身份取得 → 装箱为 Object
        let src_base = erased_base_ty(&src_ty);
        let cross = comment != consts::OBJECT
            && !comment.starts_with('[')
            && type_binary(&env.ctx, &src_base).is_some()
            && sim::erased_base(&src_ty, env) != env.ctx.short(comment)
            && !is_subtype(&env.ctx, &src_base, &RsType::class(comment, Vec::new()));
        if cross {
            let se = text(env, &expr);
            let se = if se.starts_with("Clone::clone(") { se } else { format!("Clone::clone(&{se})") };
            sim.push(raw(format!("Into::<Object>::into({se})")), RsType::Object);
        } else {
            sim.push(expr, src_ty);
        }
    } else {
        sim.push(expr, cast_rust);
    }
    Ok(())
}

/// 源类型名是否为上界擦除到目标的类型变量（`erased_base(type_var_bounds[src]) == erased_base(cast)`）
fn bound_matches(env: &InstrEnv, sim: &StackSim, src_name: &str, cast: &RsType) -> bool {
    sim.state
        .type_var_bounds
        .get(src_name)
        .is_some_and(|b| sim::erased_base(b, env) == sim::erased_base(cast, env))
}

fn instanceof(env: &InstrEnv, sim: &mut StackSim, log: &mut InstrLog, comment: &str) -> InstrResult<()> {
    let popped = if sim.state.stack.is_empty() { None } else { Some(sim.pop()?) };
    let bool_t = RsType::Prim(Prim::Bool);
    let Some(val) = popped.filter(|_| !comment.is_empty()) else {
        log.push(Effect::InstanceofFold);
        sim.push(Expr::Lit(Lit::Bool(false)), bool_t);
        return Ok(());
    };
    let target_rust = operand_type(env, comment);
    // jvm_to_rust 对未载体化接口返回 Object；子类型判定需要接口的 Rust 短名
    let target = if ty_text(env, &target_rust) == OBJECT && !comment.starts_with('[') && comment != consts::OBJECT {
        RsType::class(comment, Vec::new())
    } else {
        target_rust
    };
    let obj_s = ty_text(env, &val.ty);
    let (obj_base, tgt_base) = (erased_base_ty(&val.ty), erased_base_ty(&target));
    let runtime = |sim: &mut StackSim, e: Expr| sim.push(Expr::InstanceOf { expr: Box::new(e), binary: comment.to_string() }, RsType::Prim(Prim::Bool));
    if obj_s == OBJECT {
        // 运行时多态：ObjectVTable 按运行时类的继承链判定
        runtime(sim, val.expr);
    } else if obj_s == ty_text(env, &target) || is_subtype(&env.ctx, &obj_base, &tgt_base) {
        sim.push(Expr::Lit(Lit::Bool(true)), bool_t);
    } else if is_subtype(&env.ctx, &tgt_base, &obj_base)
        || env.ctx.ty.carrier_type_for_ident(&obj_base).is_some()
        || open_hierarchy(env, &obj_base, comment)
    {
        // 静态类型是目标的超类 / 接口载体（实现者开放）/ 开放层次（接口 vs 非 final 类）：
        // 装箱（保持运行时类）后按运行时判定
        let boxed = to_object(env, str_leaf(val.expr), &obj_base, true)?;
        runtime(sim, boxed);
    } else {
        log.push(Effect::InstanceofFold);
        sim.push(Expr::Lit(Lit::Bool(false)), bool_t);
    }
    Ok(())
}

/// `obj instanceof T` 在两者互不为子类型时是否仍可能为真（JLS §5.5）：一侧是接口、另一侧
/// 是非 final 类 → 子类可同时实现该接口。类型不可解析时保守视为可能（永不误折叠）
fn open_hierarchy(env: &InstrEnv, obj_base: &RsType, target_bin: &str) -> bool {
    let reg = env.ctx.reg();
    if reg.is_empty() || target_bin.starts_with('[') {
        return false;
    }
    let tgt = reg.get(target_bin);
    let obj = type_binary(&env.ctx, obj_base).and_then(|b| reg.get(&b));
    let (Some(tgt), Some(obj)) = (tgt, obj) else {
        return true;
    };
    let is_final = |ci: &ty::ClassInfo| ci.class_file().access & ACC_FINAL != 0;
    if tgt.is_interface() && !obj.is_interface() {
        return !is_final(obj);
    }
    if obj.is_interface() && !tgt.is_interface() {
        return !is_final(tgt);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn erased_shape_strips_class_args() {
        assert_eq!(erased_shape("JArray<Entry<K, V>>"), "JArray<Entry>");
        assert_eq!(erased_shape("Map<K, JArray<V>>"), "Map<K, JArray<V>>");
        assert_eq!(erased_shape("A<B<C>, D>"), "A");
        assert_eq!(erased_shape("i32"), "i32");
        assert_eq!(erased_shape("XJArray<T>"), "XJArray");
    }

    #[test]
    fn word_scan() {
        assert_eq!(words("Foo<_, Bar<i32>>"), vec!["Foo", "_", "Bar", "i32"]);
    }
}

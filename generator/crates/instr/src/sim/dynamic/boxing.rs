//! 文本层的装箱 / 拆箱适配（`coerce._coerce_to_object` 的字符串入口、
//! `dynamic._unbox_object_arg` / `_box_prim_via_valueof`）。
//!
//! lambda 闭包体与拼接实参在 Python 侧以文本拼接（整体落 RawExpr / RawStmt），叶子是
//! 任意已渲染文本；这里按同一口径直接产出文本。

use ty::RsType;

use super::wrapper_of;
use crate::build::ty_text;
use crate::coerce::{object_kind, ObjectKind};
use crate::env::InstrEnv;
use crate::error::InstrResult;
use crate::naming::mangle_if_overloaded;

/// Rust 原生标量的类型文本（`constants.PRIMITIVE_RUST_TYPES`）
const PRIMITIVE_RUST_TYPES: [&str; 13] =
    ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize", "()"];

pub(super) fn is_prim_text(t: &str) -> bool {
    PRIMITIVE_RUST_TYPES.contains(&t)
}

/// 值文本 → Object 引用文本（`_coerce_to_object(val_str, ty)`，clone = True）：
/// 基本类型 `.into()`；类型形参 `Into::<Object>::into(..)`；数组 / 注册表内类 / 接口载体
/// `Object::from(..)`；其余 `Object::from_any(..)`。叶子按原文承载（`this` 亦为 `&this`）
pub(super) fn obj_text(env: &InstrEnv, val: &str, t: &RsType) -> String {
    let obj = ir::anchors::OBJECT;
    let kind = object_kind(env, t);
    if kind == ObjectKind::Prim {
        // 负数字面量补外层括号：`-1i32.into()` 解析为 `-(1i32.into())`
        return if val.trim_start().starts_with('-') { format!("({val}).into()") } else { format!("{val}.into()") };
    }
    let src = format!("Clone::clone(&{val})");
    match kind {
        ObjectKind::TypeVar => format!("Into::<{obj}>::into({src})"),
        ObjectKind::Ref => format!("{obj}::from({src})"),
        _ => format!("{obj}::from_any({src})"),
    }
}

/// 装箱类在注册表中的 (binary, Rust 类型文本)；类不在注册表 / 映射为 Object → None
fn wrapper_class<'r>(env: &InstrEnv<'r>, prim_desc: &str) -> Option<(&'r ty::ClassInfo, String)> {
    let wci = env.ctx.reg().get(wrapper_of(prim_desc)?)?;
    let rust = ty_text(env, &env.ctx.ty.jvm_to_rust(&format!("L{};", wci.name())));
    (rust != ir::anchors::OBJECT).then_some((wci, rust))
}

/// Object 形态的 SAM 实参 → 实现方法基本类型形参（LambdaMetafactory 装箱适配的拆箱侧，
/// S-3.1）：`x.try_cast::<Integer>("java/lang/Integer")?.intValue()?`。
///
/// 拆箱方法与 Rust 类型名从注册表的装箱类动态解析（实例方法、零实参、名以 `Value` 结尾、
/// 返回该基本类型）。解析失败 → None，调用方退化为直接传参（编译期 E0308 暴露）
pub(super) fn unbox_object_arg(env: &InstrEnv, val: &str, prim_desc: &str) -> InstrResult<Option<String>> {
    let Some((wci, wrapper_rust)) = wrapper_class(env, prim_desc) else {
        return Ok(None);
    };
    let hit = wci.methods().iter().find(|m| {
        !m.is_static()
            && m.name.ends_with("Value")
            && ty::type_map::parse_descriptor_params(&m.desc).is_empty()
            && ty::type_map::parse_descriptor_return(&m.desc) == prim_desc
    });
    let Some(m) = hit else {
        return Ok(None);
    };
    let mname = ty::ident::safe_ident(&mangle_if_overloaded(&env.ctx, wci.name(), &m.name, Some(&m.desc))?);
    Ok(Some(format!("{val}.try_cast::<{wrapper_rust}>(\"{}\")?.{mname}()?", wci.name())))
}

/// 基本类型值文本 → Object 形态的 SAM 返回值（装箱侧，S-3.1）：
/// `Object::from(Integer::valueOf_i(x)?)`。
///
/// 须经装箱类的 valueOf 工厂装成真实包装对象（含缓存池语义）：`.into()` 的原生盒
/// 在折叠状态再入 SAM 时 `try_cast::<Integer>` 无法命中。valueOf 从注册表动态解析
/// （static、单实参为该基本类型、返回该装箱类）。解析失败 → None，调用方退回 `.into()`
pub(super) fn box_prim_via_valueof(env: &InstrEnv, val: &str, prim_desc: &str) -> InstrResult<Option<String>> {
    let Some((wci, wrapper_rust)) = wrapper_class(env, prim_desc) else {
        return Ok(None);
    };
    let ret = format!("L{};", wci.name());
    let hit = wci.methods().iter().find(|m| {
        m.is_static()
            && m.name == "valueOf"
            && ty::type_map::parse_descriptor_params(&m.desc) == [prim_desc]
            && ty::type_map::parse_descriptor_return(&m.desc) == ret
    });
    let Some(m) = hit else {
        return Ok(None);
    };
    let mname = ty::ident::safe_ident(&mangle_if_overloaded(&env.ctx, wci.name(), &m.name, Some(&m.desc))?);
    Ok(Some(format!("{}::from({wrapper_rust}::{mname}({val})?)", ir::anchors::OBJECT)))
}

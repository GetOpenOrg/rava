//! 栈模拟的外部查询面（← `StackSim.__init__` 注入的 is_subtype / is_interface /
//! box_object / infer_type_args 回调 + `jvm_type.carrier_type_for_ident` + `short_cls`），
//! 以及 [`RsType`] → [`ir::Type`] 桥接。
//!
//! Python 回调以短名 / 类型串为实参；这里一律传结构化类型（擦除查询传擦除后的
//! [`RsType`]：`Class` 的实参清空），实现方自行映射到注册表。

use crate::error::{SimError, SimResult};
use ir::{Ident, Prim, Type};
use ty::RsType;

/// 栈模拟需要的类型层 / 发射层查询（由 P4b 方法体生成入口实现）。
pub trait SimEnv {
    /// JVM binary 名 → Rust 类型短名（`ty::ShortNames::short`）
    fn short_name(&self, binary: &str) -> String;
    /// 擦除类型 `sub` 是否为 `sup` 的子类型（`_is_subtype(child, parent, registry)`）
    fn is_subtype(&self, sub: &RsType, sup: &RsType) -> bool;
    /// 擦除类型是否为注册表内接口（`hierarchy._is_interface`）
    fn is_interface(&self, ty: &RsType) -> bool;
    /// 类型头名命中注册表内接口 → 擦除载体（`carrier_type_for_ident`）；否则 None
    fn carrier_type(&self, ty: &RsType) -> Option<RsType>;
    /// 已取得所有权的 `value`（静态类型 `ty`）→ 保持对象身份的 `Object` 引用
    /// （`coerce._coerce_to_object(.., clone=False)`）
    fn box_object(&self, value: ir::Expr, ty: &RsType) -> SimResult<ir::Expr>;
    /// 菱形构造 `ty`（实参待推断）按局部变量声明签名解出类型实参；无解 → None
    /// （`sig_types.infer_type_args_from_declared` + 作用域外实参擦除为 Object）
    fn infer_type_args(&self, ty: &RsType, declared_sig: &str) -> Option<Vec<RsType>>;
}

/// [`SimEnv`] → [`ir::ShortNames`] 适配（渲染器注入用）。
pub struct TyNames<'e>(pub &'e dyn SimEnv);

impl ir::ShortNames for TyNames<'_> {
    fn short_cls(&self, binary: &str) -> String {
        self.0.short_name(binary)
    }
}

/// 类型推断占位（菱形构造 `X<_>` 的实参），以类型形参 `_` 承载。
pub const INFER_PARAM: &str = "_";

pub(crate) fn ident(s: &str) -> SimResult<Ident> {
    Ident::new(s).map_err(SimError::from)
}

/// `$` → `_` 的末段简单名（`RsType::Bare` 的渲染口径）
fn bare_name(binary: &str) -> String {
    binary.rsplit('/').next().unwrap_or(binary).replace('$', "_")
}

pub(crate) fn prim_ir(p: ty::Prim) -> Prim {
    match p {
        ty::Prim::I8 => Prim::I8,
        ty::Prim::I16 => Prim::I16,
        ty::Prim::I32 => Prim::I32,
        ty::Prim::I64 => Prim::I64,
        ty::Prim::F32 => Prim::F32,
        ty::Prim::F64 => Prim::F64,
        ty::Prim::Bool => Prim::Bool,
        ty::Prim::U16 => Prim::U16,
    }
}

/// [`RsType`] → [`ir::Type`]（`Param("_")` → `_`）。
pub fn to_ir_type(t: &RsType, env: &dyn SimEnv) -> SimResult<Type> {
    Ok(match t {
        RsType::Prim(p) => Type::Prim(prim_ir(*p)),
        RsType::Unit => Type::UNIT,
        RsType::Object => Type::named(ident(ir::anchors::OBJECT)?, Vec::new()),
        RsType::Class { binary, args } => {
            let args = args.iter().map(|a| to_ir_type(a, env)).collect::<SimResult<Vec<_>>>()?;
            Type::named(ident(&env.short_name(binary))?, args)
        }
        RsType::Bare { binary } => Type::named(ident(&bare_name(binary))?, Vec::new()),
        RsType::Array(e) => Type::named(ident(ir::anchors::ARRAY)?, vec![to_ir_type(e, env)?]),
        RsType::Param(n) if n == INFER_PARAM => Type::Infer,
        RsType::Param(n) => Type::named(ident(n)?, Vec::new()),
    })
}

/// 渲染文本（Python `render_type` 同口径，用于「渲染不同」判定与合成槽分名）。
pub fn type_text(t: &RsType, env: &dyn SimEnv) -> String {
    match to_ir_type(t, env) {
        Ok(ty) => ir::render::render_type(&ty),
        // 非法标识符的类型不会进入栈（构造期已报错）；兜底按结构文本区分
        Err(_) => format!("{t:?}"),
    }
}

/// 擦除基名（`erased_base`：`X<A>` → `X`、`JArray<E>` → `JArray`、`()` → `()`）。
pub fn erased_base(t: &RsType, env: &dyn SimEnv) -> String {
    match t {
        RsType::Unit => "()".to_string(),
        RsType::Class { binary, .. } => env.short_name(binary),
        RsType::Bare { binary } => bare_name(binary),
        RsType::Prim(p) => p.rust_name().to_string(),
        RsType::Object => ir::anchors::OBJECT.to_string(),
        RsType::Array(_) => ir::anchors::ARRAY.to_string(),
        RsType::Param(n) => n.clone(),
    }
}

/// 擦除到裸类身份（`Class` 清空实参；其余不变）——擦除查询钩子的实参。
pub fn erase(t: &RsType) -> RsType {
    match t {
        RsType::Class { binary, .. } => RsType::class(binary.clone(), Vec::new()),
        other => other.clone(),
    }
}

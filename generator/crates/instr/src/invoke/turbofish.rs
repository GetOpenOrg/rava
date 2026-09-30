//! 泛型类静态调用路径的 turbofish（← `invoke._static_call_turbofish`）。

use std::collections::BTreeMap;

use sim::StackSim;
use ty::RsType;

use crate::ctx::InstrCtx;
use crate::hierarchy::type_binary;

/// `Cls::<..>::method` 所需的类型实参（空 = 不加 turbofish）。
///
/// 静态方法不使用类的类型参数，调用点无上下文可推断（E0283），须显式给出：
/// - 同类调用且当前 impl 有类型形参：用当前 impl 的形参（`type_bindings` 按实参绑定的
///   类级类型变量优先）；
/// - 跨类调用：一律 Object（擦除）。
///
/// `cls` 为 JVM binary 名；注册表外 / 无有效类型形参 → 空。
pub fn static_call_turbofish(
    ctx: &InstrCtx,
    sim: &StackSim,
    cls: &RsType,
    type_bindings: Option<&BTreeMap<String, RsType>>,
) -> Vec<RsType> {
    let Some(bin) = type_binary(ctx, cls) else {
        return Vec::new();
    };
    let Some(ci) = ctx.reg().get(&bin) else {
        return Vec::new();
    };
    let tparams = ctx.ty.effective_class_type_params(ci);
    if tparams.is_empty() {
        return Vec::new();
    }
    if bin == ctx.class_name && !sim.cfg.class_type_params.is_empty() {
        return tparams
            .iter()
            .map(|tp| type_bindings.and_then(|b| b.get(tp)).cloned().unwrap_or_else(|| RsType::Param(tp.clone())))
            .collect();
    }
    tparams.iter().map(|_| RsType::Object).collect()
}

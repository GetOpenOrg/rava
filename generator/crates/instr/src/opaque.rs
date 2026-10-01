//! L1（名字级）不透明类型的接收者上转（C3 第 6 项）。
//!
//! 不透明类（`java_class_opaque!`）只有类型身份，无字段访问器、无方法；它对每个传递超类型有 upcast。
//! 接收者的 Rust 静态类型是不透明类、而成员（调用属主 / 字段声明类）在其有布局的祖先上时，先经
//! `Into::<祖先>::into` 上转再访问。按分析器的级别规则这样的值恒为 null（非 null 值的类及其全部超类型
//! 至少 L2），上转保持 null，随后的访问按 JVM 语义抛 NullPointerException。

use ty::{JvmType, RsType};

use crate::build::ty_text;
use crate::env::InstrEnv;

/// 接收者擦除类型的 binary 名（类 / 接口）
fn erased_binary(env: &InstrEnv, t: &RsType) -> Option<String> {
    match env.ctx.ty.from_rs_type(t, &Default::default()).erasure() {
        JvmType::Class { binary, .. } => Some(binary),
        _ => None,
    }
}

/// 接收者静态类型是不透明类且 `target` 是它的（有布局的）真祖先：返回上转后的视图类型
/// （根类 → `Object`，其余 → 类型实参全取 Object 的擦除实例）
pub fn receiver_view(env: &InstrEnv, recv_ty: &RsType, target: &str) -> Option<RsType> {
    let bin = erased_binary(env, recv_ty)?;
    if bin == target || !env.ctx.hooks.is_opaque(&bin) || env.ctx.hooks.is_opaque(target) {
        return None;
    }
    Some(class_view(env, target))
}

/// 类 `binary` 的擦除视图类型：根类 → `Object`，其余 → 类型实参全取 Object
pub fn class_view(env: &InstrEnv, binary: &str) -> RsType {
    if binary == ty::consts::OBJECT {
        return RsType::Object;
    }
    let n = env.ctx.reg().get(binary).map_or(0, |ci| env.ctx.ty.effective_class_type_params(ci).len());
    RsType::class(binary.to_string(), vec![RsType::Object; n])
}

/// 上转表达式文本：`Into::<View>::into(Clone::clone(&recv))`
pub fn upcast_text(env: &InstrEnv, recv: &str, view: &RsType) -> String {
    let inner = recv.strip_prefix('&').map_or_else(|| format!("&{recv}"), str::to_string);
    format!("Into::<{}>::into(Clone::clone({inner}))", ty_text(env, view))
}

/// 实例字段 `name` 的声明类：自 `owner` 沿父类链找首个声明它的类（JVMS §5.4.3.2 的类链部分；
/// 实例字段不在接口上）。找不到 → `owner`
pub fn field_declarer(env: &InstrEnv, owner: &str, name: &str) -> String {
    let reg = env.ctx.reg();
    let mut cur = owner.to_string();
    while let Some(ci) = reg.get(&cur) {
        if ci.fields().iter().any(|f| f.name == name && !f.is_static()) {
            return cur;
        }
        let sup = ci.super_class();
        if sup.is_empty() {
            break;
        }
        cur = sup.to_string();
    }
    owner.to_string()
}

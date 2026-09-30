//! 成员命名（← `instr/member_naming.py`）：重载 mangle、lambda 实现方法名的单一来源。

use crate::ctx::InstrCtx;
use crate::error::{unported, InstrResult};

/// 调用目标 `cls.mname:desc` 的 Rust 方法名（`_mangle_if_overloaded`）：重载 → 描述符后缀名，
/// 否则原名。`cls` 为 binary 名或短名；`desc` 为调用描述符（无则按原名处理重载判定）
pub fn mangle_if_overloaded(_ctx: &InstrCtx, _cls: &str, mname: &str, _desc: Option<&str>) -> InstrResult<String> {
    unported(format!("mangle_if_overloaded({mname})"))
}

/// invokedynamic 实现方法（lambda body / 方法引用目标）的 Rust 名（`lambda_impl_rust_name`）：
/// 重载 mangle、`<init>` → `new`、安全标识符三步收拢
pub fn lambda_impl_rust_name(ctx: &InstrCtx, impl_cls: &str, impl_mname: &str, impl_desc: &str) -> InstrResult<String> {
    let mangled = mangle_if_overloaded(ctx, impl_cls, impl_mname, Some(impl_desc))?;
    Ok(ty::ident::safe_ident(&mangled.replace("<init>", "new")))
}

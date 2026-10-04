//! 继承成员 / 接口实现引用类型：在接收者作用域认领名字；跨文件引用路径按引用类 binary 生成。

use ty::ident::is_rust_keyword;

use super::Emissions;
use crate::ctx::{EmitCtx, USER_CRATE};
use crate::text::to_snake;

/// 类在 Rust 中的完整引用路径（不含 `use` 与 `;`），首段按目标类所在 crate 定向：
///
/// - JDK 类（模块 crate，包 mod.rs 再导出）：同 crate `crate::java::lang::String`，跨 crate `<模块 crate>::java::…`
/// - 用户类 / 默认包类：main.rs 只声明 mod，路径写到模块层 `crate::<snake>::<Short>`
/// - lib crate 类：接收者同 crate 用 `crate::`，否则用其 crate 名
///
/// `recv_crate` 为引用所在文件的 crate 名
pub fn class_use_path(ctx: &EmitCtx<'_>, binary: &str, recv_crate: &str) -> String {
    use_path(ctx, binary, recv_crate)
}

/// [`class_use_path`] 的核心
pub fn use_path(ctx: &EmitCtx<'_>, binary: &str, recv_crate: &str) -> String {
    // 全路径末段 = 定义处的名字（不在调用方作用域认领）
    let short = ctx.declared(binary);
    let segs: Vec<&str> = binary.split('/').collect();
    let pkg = segs[..segs.len() - 1]
        .iter()
        .map(|p| if is_rust_keyword(p) { format!("r#{p}") } else { (*p).to_string() })
        .collect::<Vec<_>>()
        .join("::");
    let target = ctx.crate_of(binary);
    let head = if target == recv_crate { "crate" } else { target };
    if pkg.is_empty() || target == USER_CRATE {
        return format!("{head}::{}::{short}", to_snake(binary));
    }
    format!("{head}::{pkg}::{short}")
}

/// 引用类（binary，按出现序）在接收者文件作用域认领名字：只认领有发射记录的类
/// （导入行由作用域记录生成）
pub fn claim_classes<S: AsRef<str>>(ctx: &EmitCtx<'_>, classes: &[S], ems: &Emissions) {
    for b in classes {
        let b = b.as_ref();
        if ems.contains_key(b) {
            ctx.short(b);
        }
    }
}

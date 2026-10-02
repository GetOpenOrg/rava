//! 继承成员 / 接口实现引用类型：在接收者作用域认领名字；跨文件引用路径按引用类 binary 生成。

use ty::ident::is_rust_keyword;

use super::Emissions;
use crate::ctx::EmitCtx;
use crate::text::to_snake;

const JAVA_RUNTIME: &str = "java_runtime";

/// 类在 Rust 中的完整引用路径（不含 `use` 与 `;`）。
///
/// - JDK 类（java_runtime，包 mod.rs 再导出）：`<crate 前缀>::java::lang::String`
/// - 默认包用户类：main.rs 只声明 mod，路径写到模块层 `crate::<snake>::<Short>`
/// - 标记了 crate 名的非 java_runtime 类：接收者同 crate 用 `crate::`，否则用其 crate 名
///
/// `ems` 为 None 时不查目标类的发射记录（Python 部分调用点不传 emissions，照搬）。
pub fn class_use_path(ctx: &EmitCtx<'_>, binary: &str, crate_prefix: &str, ems: Option<&Emissions>, recv_crate: &str) -> String {
    let target = ems.and_then(|e| e.get(binary)).map(|e| (e.crate_name.as_str(), e.crate_prefix.as_str()));
    use_path(ctx, binary, crate_prefix, target, recv_crate)
}

/// [`class_use_path`] 的核心：`target` 为目标类的 (crate 名, crate 前缀) 视图（None = 无发射记录）
pub fn use_path(ctx: &EmitCtx<'_>, binary: &str, crate_prefix: &str, target: Option<(&str, &str)>, recv_crate: &str) -> String {
    // 全路径末段 = 定义处的名字（不在调用方作用域认领）
    let short = ctx.ty.global_names().short(binary);
    let segs: Vec<&str> = binary.split('/').collect();
    let pkg = segs[..segs.len() - 1]
        .iter()
        .map(|p| if is_rust_keyword(p) { format!("r#{p}") } else { (*p).to_string() })
        .collect::<Vec<_>>()
        .join("::");
    let lib_crate = target.map_or("", |t| t.0);
    if !lib_crate.is_empty() && lib_crate != JAVA_RUNTIME {
        let prefix = if lib_crate == recv_crate { "crate" } else { lib_crate };
        if pkg.is_empty() {
            return format!("{prefix}::{}::{short}", to_snake(binary));
        }
        return format!("{prefix}::{pkg}::{short}");
    }
    if pkg.is_empty() || target.is_some_and(|t| t.1 != "crate") {
        return format!("crate::{}::{short}", to_snake(binary));
    }
    format!("{crate_prefix}::{pkg}::{short}")
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

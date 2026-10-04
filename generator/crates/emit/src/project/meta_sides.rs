//! 反射元数据两侧（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）。
//!
//! - 档案侧：`java_runtime` 声明层与 lib crate 的落盘文件 → `closure_input/meta_tables.rs`（JDK `java_meta`
//!   以 `include!` 引入，表 static 以 `__java_meta_<表名>` 导出）。同一档案下与用户程序无关。
//! - 用户侧：用户类发射文本 + 用户模块服务 + 用户文件的栈帧行表 → `user/src/<USER_META_MOD>.rs`
//!   （`const` 表 + `USER_META` 聚合），入口 `main` 首句 `java_runtime::meta::register_user` 登记。
//!
//! 表的扫描与渲染在 runtime 侧库 `rava_meta_tables`（与原 java_meta 构建脚本同一份逻辑）。

use std::path::Path;

use super::fs::Writer;
use crate::ctx::EmitCtx;
use crate::error::{EmitError, Result};

/// 用户元数据行模块名（`user/src/<名>.rs`）
pub const USER_META_MOD: &str = "rava_user_meta";
/// 档案侧反射元数据表文件（scratch 相对路径）：java_meta 的 `lib.rs` 以 `include!` 引入
pub const META_TABLES: &str = "closure_input/meta_tables.rs";

/// 档案侧表：扫描各 crate 根（`java_runtime/src` 与 lib crate 的 `src`）下的落盘 `.rs` 文件。
/// 成员表按档案的反射事实裁剪（`ReflectFacts::meta_methods` / `meta_fields`，见 `rava_meta_tables::Keep`）
pub(super) fn write_archive(ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, roots: &[&Path]) -> Result<()> {
    let mut texts: Vec<String> = Vec::new();
    for root in roots {
        for p in rava_meta_tables::rs_files(root) {
            let t = std::fs::read_to_string(&p).map_err(|e| EmitError::Io(format!("{}：{e}", p.display())))?;
            texts.push(t);
        }
    }
    let refs: Vec<&str> = texts.iter().map(String::as_str).collect();
    let r = &ctx.input.reflect;
    let keep = rava_meta_tables::Keep { methods: &r.meta_methods, fields: &r.meta_fields };
    w.write(&out_dir.join(META_TABLES), &rava_meta_tables::render(&refs, rava_meta_tables::Side::Archive(keep)))
}

/// 用户侧行：`files` 为用户 crate 的 (路径, 文本)，`lines` 为用户文件的行表源文本（`line_tables::write` 返回）
pub(super) fn write_user(ctx: &EmitCtx<'_>, w: &mut Writer, user_src: &Path, files: &[(&Path, &str)], lines: &str) -> Result<()> {
    let texts: Vec<&str> = files.iter().filter(|(p, _)| p.starts_with(user_src)).map(|(_, t)| *t).collect();
    let mut src = String::from("use java_runtime::meta::{CpVal, FieldMeta, MethodMeta, NestMeta};\n\n");
    src += &rava_meta_tables::render(&texts, rava_meta_tables::Side::User);
    src += "\n// 用户模块服务 (服务, provider)：闭包事实 seeds.module_services 中涉及用户类者，事实序\n";
    src += "pub const MODULE_SERVICES: &[(&str, &str)] = &[\n";
    for (s, p) in user_services(ctx) {
        src += &format!("    ({s:?}, {p:?}),\n");
    }
    src += "];\n\n";
    src += &rava_meta_tables::localize(lines);
    w.write(&user_src.join(format!("{USER_META_MOD}.rs")), &src)
}

/// 模块服务是否属于用户侧（服务或 provider 是用户类）
pub(super) fn is_user_service(ctx: &EmitCtx<'_>, svc: &str, prov: &str) -> bool {
    ctx.is_user(svc) || ctx.is_user(prov)
}

fn user_services<'c>(ctx: &'c EmitCtx<'_>) -> impl Iterator<Item = &'c (String, String)> {
    ctx.input.module_services.iter().filter(move |(s, p)| is_user_service(ctx, s, p))
}

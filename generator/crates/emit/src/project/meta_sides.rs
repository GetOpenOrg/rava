//! 反射元数据两侧（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）。
//!
//! - 档案侧：JDK 模块 crate（根为声明层）与 lib crate 的落盘文件 → `closure_input/meta_tables.rs`（JDK `java_meta`
//!   以 `include!` 引入，表 static 以 `__java_meta_<表名>` 导出）。同一档案下与用户程序无关。
//! - 用户侧：用户类发射文本 + 用户文件的栈帧行表 + 类路径资源表 + 本程序 jimage → `user/src/<USER_META_MOD>.rs`
//!   （`const` 表 + `USER_META` 聚合），入口 `main` 首句 `<根门面>::meta::register_user` 登记。
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

/// 档案侧表：扫描各 crate 根（JDK 模块 crate 与 lib crate 的 `src`）下的落盘 `.rs` 文件。
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
    let mut src = format!("use {}::meta::UserMeta;\n\n", ctx.crates().root());
    src += &rava_meta_tables::render(&texts, rava_meta_tables::Side::User);
    src += "\n";
    src += &rava_meta_tables::localize(lines);
    src += &resource_tables(ctx, w, user_src)?;
    w.write(&user_src.join(format!("{USER_META_MOD}.rs")), &src)
}

/// 构建期嵌入资源的落盘目录（`user/src/` 下）：类路径表第 i 行的字节存为 `<目录>/class_path/<i>`，jimage 存为 `<目录>/modules`
const RESOURCE_DIR: &str = "rava_resources";

/// 用户侧嵌入资源（计划 c1d §30.15、boot-image §5.7）：类路径资源 `CLASS_PATH_RESOURCES`（应用类路径的全部文件，
/// 按名有序、同名按类路径序）与本程序 jimage `MODULE_IMAGE`（闭包读取的 JDK 模块资源，`NativeImageBuffer.getNativeMap`
/// 映射为 `${java.home}/lib/modules`；8 字节对齐，读侧经 IntBuffer 视图读索引）
fn resource_tables(ctx: &EmitCtx<'_>, w: &mut Writer, user_src: &Path) -> Result<String> {
    let rows = &ctx.input.class_path_resources;
    let mut src = "\n// 类路径资源：应用类路径（用户与库档案）的全部文件，按名有序、同名按类路径序（计划 c1d §30.15）\n\
                   pub const CLASS_PATH_RESOURCES: &[(&str, &[u8])] = &[\n".to_string();
    for (i, (name, bytes)) in rows.iter().enumerate() {
        w.write_bytes(&user_src.join(RESOURCE_DIR).join("class_path").join(i.to_string()), bytes)?;
        src += &format!("    ({name:?}, include_bytes!(\"{RESOURCE_DIR}/class_path/{i}\")),\n");
    }
    src += "];\n";
    w.write_bytes(&user_src.join(RESOURCE_DIR).join("modules"), &super::jimage::write(&ctx.input.module_resources))?;
    src += &format!(
        "\n// 本程序 jimage（{} 份模块资源；格式见生成器 `project/jimage.rs`）\n\
         #[repr(C, align(8))]\nstruct ImageAligned<B: ?Sized>(B);\n\
         const MODULE_IMAGE_ALIGNED: &ImageAligned<[u8]> = &ImageAligned(*include_bytes!(\"{RESOURCE_DIR}/modules\"));\n\
         pub const MODULE_IMAGE: &[u8] = &MODULE_IMAGE_ALIGNED.0;\n",
        ctx.input.module_resources.len()
    );
    Ok(src)
}

//! 模块 crate 发射（T1 第 2 步 M2，方案 `docs/plans/2026-10-04-t1-step2-direct-rustc-link.md` §2.6）。
//!
//! - 根模块：声明层 `<根>_decl`（overlay 的运行时 crate 改名）+ 实现层 `<根>_body_k`，门面 crate `<根>`
//!   再导出声明层并链接实现层；
//! - 其余模块：一个模块一个完整 crate（Full 模式，不拆层），lib.rs 私有 glob 导入根，
//!   依赖 = 根声明层（以根名改名引入）+ requires 闭包内的上游模块 crate。
//!
//! 只有链接者（user crate）依赖门面：其余 crate 以根名改名依赖声明层，编译不等实现层。

use std::path::Path;

use super::entry::lints_section;
use super::fs::Writer;
use crate::ctx::EmitCtx;
use crate::error::{io_err, Result};
use crate::module_crates::ModuleCrates;
use crate::text::{safe_pkg_part, scratch_pkg_version};

/// 非根模块 crate 的 lib.rs 属性行
const MODULE_ALLOW: &str = "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, \
                            non_camel_case_types, non_upper_case_globals, static_mut_refs, ambiguous_glob_reexports, \
                            unused_comparisons)]";

/// `{name:<15} = { path = "../dir" }`
pub fn path_dep(name: &str, dir: &str) -> String {
    if name == dir {
        format!("{name:<15} = {{ path = \"../{dir}\" }}")
    } else {
        format!("{name:<15} = {{ package = \"{dir}\", path = \"../{dir}\" }}")
    }
}

/// 以根名引入根声明层的完整视图（`top`：声明层末段 crate；非链接者 crate 用）
pub fn root_decl_dep(crates: &ModuleCrates, top: &str) -> String {
    path_dep(crates.root(), top)
}

/// crate 清单头：`[package]` + `[lib]` + `[dependencies]` 行 + lints
pub fn lib_manifest(dir: &Path, name: &str, deps: &[String]) -> String {
    let mut l = vec![
        "[package]".to_string(),
        format!("name = \"{name}\""),
        format!("version = \"{}\"", scratch_pkg_version(dir)),
        "edition = \"2021\"".into(),
        String::new(),
        "[lib]".into(),
        format!("name = \"{name}\""),
        "path = \"src/lib.rs\"".into(),
        "crate-type = [\"lib\"]".into(),
        String::new(),
        "[dependencies]".into(),
    ];
    l.extend(deps.iter().cloned());
    l.push(String::new());
    l.extend(lints_section());
    l.join("\n")
}

/// 源码树下含 mod.rs 的顶层目录（名字序）
fn top_mods(src: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(src) else { return Ok(out) };
    for e in rd {
        let e = e.map_err(|e| io_err(&src.display().to_string(), e))?;
        if e.path().join("mod.rs").is_file() {
            out.push(e.file_name().to_string_lossy().into_owned());
        }
    }
    out.sort();
    Ok(out)
}

/// 非根模块 crate 的 lib.rs 与 Cargo.toml（包 mod 树须已写出）
pub fn write_module_crates(ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, top: &str) -> Result<()> {
    let crates = ctx.crates();
    for c in crates.others() {
        let dir = out_dir.join(&c.name);
        let src = dir.join("src");
        let mut lib_rs = vec![
            MODULE_ALLOW.to_string(),
            "// 根模块全部公开项（运行时基础设施与根模块类；本 crate 顶层包模块优先）".to_string(),
            format!("use {}::*;", crates.root()),
        ];
        lib_rs.extend(top_mods(&src)?.iter().map(|t| format!("pub mod {};", safe_pkg_part(t))));
        w.write(&src.join("lib.rs"), &(lib_rs.join("\n") + "\n"))?;
        let mut deps = vec![root_decl_dep(crates, top)];
        deps.extend(c.deps.iter().filter(|d| d.as_str() != crates.root()).map(|d| path_dep(d, d)));
        deps.push(format!("rava_macros     = {{ path = \"{}\" }}", ctx.macros_crate.display()));
        deps.push("libc            = \"0.2\"".into());
        w.write(&dir.join("Cargo.toml"), &lib_manifest(&dir, &c.name, &deps))?;
    }
    Ok(())
}

/// 根门面 crate：再导出声明层（以声明层名改名引入末段完整视图），链接实现层
/// `image`：根门面带构建期引导映像模块（`boot_image.rs`，见 [`super::boot_image`]）
pub fn write_facade(w: &mut Writer, out_dir: &Path, crates: &ModuleCrates, top: &str, bodies: &[&str], image: bool) -> Result<()> {
    let root = crates.root();
    let decl = crates.decl();
    let dir = out_dir.join(root);
    let mut lib_rs = vec![
        "#![allow(unused_imports)]".to_string(),
        "// 根模块门面：声明层全部公开项 + 实现层（只为链接）".to_string(),
        format!("pub use {decl}::*;"),
    ];
    lib_rs.extend(bodies.iter().map(|b| format!("use {b} as _;")));
    if image {
        lib_rs.push(format!("pub mod {};", super::boot_image::MODULE));
    }
    w.write(&dir.join("src").join("lib.rs"), &(lib_rs.join("\n") + "\n"))?;
    let mut deps = vec![path_dep(&decl, top)];
    deps.extend(bodies.iter().map(|b| path_dep(b, b)));
    w.write(&dir.join("Cargo.toml"), &lib_manifest(&dir, root, &deps))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dep_lines() {
        assert_eq!(path_dep("m", "m"), "m               = { path = \"../m\" }");
        assert_eq!(path_dep("rt", "rt_decl"), "rt              = { package = \"rt_decl\", path = \"../rt_decl\" }");
        assert_eq!(root_decl_dep(&ModuleCrates::single("rt"), "rt_decl"), path_dep("rt", "rt_decl"));
        assert_eq!(root_decl_dep(&ModuleCrates::single("rt"), "rt_decl_2"), path_dep("rt", "rt_decl_2"));
    }
}

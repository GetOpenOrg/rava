//! user crate 入口与工程文件：main.rs（启动钩子登记）、user / 根 Cargo.toml、用户子包 mod.rs、
//! 模块资源表、strict.txt / jdk_feature.txt。

use std::path::Path;

use super::fs::Writer;
use super::layout::{JdkLayout, UserLayout};
use crate::ctx::EmitCtx;
use crate::error::Result;
use crate::text::{safe_pkg_part, scratch_pkg_version, to_snake};

/// 反射分派登记行（dispatch_gen 产出；步骤 (d) 接入）
#[derive(Debug, Default, Clone)]
pub struct DispatchReg {
    pub methods: Vec<String>,
    pub fields: Vec<String>,
}

/// 模块声明行：非 ASCII 名补 `#[path]`（rustc 不按非 ASCII 名推导文件路径，E0754）
pub fn user_mod_decl(parent: &Path, name: &str, vis: &str) -> String {
    if name.is_ascii() {
        return format!("{vis}mod {name};");
    }
    let target = if parent.join(name).is_dir() { format!("{name}/mod.rs") } else { format!("{name}.rs") };
    format!("#[path = \"{target}\"]\n{vis}mod {name};")
}

/// JDK 类在 user crate 中的完整路径（`java_runtime::java::util::X`）
fn jrt_path(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let mut parts = vec!["java_runtime".to_string()];
    let mut segs: Vec<&str> = bin.split('/').collect();
    segs.pop();
    parts.extend(segs.iter().map(|p| safe_pkg_part(p)));
    parts.push(ctx.short(bin));
    parts.join("::")
}

fn block(head: &str, lines: &[String], tail: &str) -> String {
    format!("{head}\n{}\n{tail}\n", lines.join("\n"))
}

/// 类初始化钩子：枚举形态 / 有 `<clinit>` 的用户类 + 注解枚举种子（JDK）
fn class_init_hooks(ctx: &EmitCtx<'_>, user: &UserLayout, jdk: &JdkLayout) -> Vec<String> {
    let mut out = Vec::new();
    for (c, e) in &user.entries {
        let Some(ci) = ctx.class(c) else { continue };
        let self_desc = format!("L{c};");
        let enum_shaped = ci.fields().iter().any(|f| f.is_static() && f.desc == self_desc);
        let has_clinit = ci.methods().iter().any(|m| m.name == "<clinit>");
        if !(enum_shaped || has_clinit) {
            continue;
        }
        let mut p = vec!["crate".to_string()];
        p.extend(e.pkg_parts.iter().cloned());
        p.push(e.mod_name.clone());
        p.push(ctx.short(c));
        out.push(format!(
            "    (\"{c}\", java_runtime::sync_model::__Shared::new(|| {}::__class_init())),",
            p.join("::")
        ));
    }
    for en in &ctx.input.annotation_enum_seeds {
        if !jdk.generated.contains(en) {
            continue;
        }
        out.push(format!(
            "    (\"{en}\", java_runtime::sync_model::__Shared::new(|| {}::__class_init())),",
            jrt_path(ctx, en)
        ));
    }
    out
}

/// main 启动段（钩子登记全部段落）
fn hook_block(ctx: &EmitCtx<'_>, user: &UserLayout, jdk: &JdkLayout, disp: &DispatchReg) -> String {
    let mut hb = String::new();
    let hooks = class_init_hooks(ctx, user, jdk);
    if !hooks.is_empty() {
        hb += &block("    java_runtime::register_class_init_hooks(&[", &hooks, "    ]);");
    }
    if !disp.methods.is_empty() {
        hb += &block("    java_runtime::reflect_dispatch::register_method_dispatch(&[", &disp.methods, "    ]);");
    }
    if !disp.fields.is_empty() {
        hb += &block("    java_runtime::reflect_dispatch::register_field_dispatch(&[", &disp.fields, "    ]);");
    }
    let obj_ctor = |path: String| format!("(|| Ok(java_runtime::java::lang::Object::from({path}::new()?)))");
    if !ctx.input.data_bundle_seeds.is_empty() {
        let lines: Vec<String> = ctx
            .input
            .data_bundle_seeds
            .iter()
            .map(|b| format!("        (\"{b}\", {} as java_runtime::data_bundles::BundleCtor),", obj_ctor(jrt_path(ctx, b))))
            .collect();
        hb += &block("    java_runtime::data_bundles::register_data_bundles(&[", &lines, "    ]);");
    }
    let jca = &ctx.input.jca_seeds;
    if !jca.is_empty() {
        let svc: Vec<String> = jca
            .iter()
            .map(|s| format!("        (\"{}\", \"{}\", \"{}\", \"{}\"),", s.ty, s.algorithm, s.imp, s.provider))
            .collect();
        let seeded: std::collections::BTreeSet<&str> = jca.iter().map(|s| s.provider.as_str()).collect();
        // 清单序 = provider 优先序（JDK security.provider.N）
        let provs: Vec<String> = ctx
            .seeds
            .jca
            .providers
            .iter()
            .filter(|p| seeded.contains(p.0.as_str()))
            .filter_map(|p| {
                let pc = ctx.seeds.jca.provider_class(&p.0)?;
                Some(format!("        (\"{}\", {} as java_runtime::jca::ProviderCtor),", p.0, obj_ctor(jrt_path(ctx, pc))))
            })
            .collect();
        hb += &block("    java_runtime::jca::register_services(&[", &svc, "    ]);");
        hb += &block("    java_runtime::jca::register_providers(&[", &provs, "    ]);");
    }
    let boot: Vec<String> = ctx
        .manifest
        .boot_init_classes
        .iter()
        .filter(|b| jdk.generated.contains(*b))
        .map(|b| format!("        (\"{b}\", {}::__class_init as fn() -> java_runtime::error::Result<()>),", jrt_path(ctx, b)))
        .collect();
    if !boot.is_empty() {
        hb += &block("    java_runtime::vm_boot_init(&[", &boot, "    ]);");
    }
    hb
}

const MAIN_ALLOW: &str =
    "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]";

/// user/src/main.rs；返回 bin 名
pub fn write_main(
    ctx: &EmitCtx<'_>,
    w: &mut Writer,
    user_src: &Path,
    user: &UserLayout,
    jdk: &JdkLayout,
    disp: &DispatchReg,
) -> Result<String> {
    let Some((main_bin, main_e)) = user.entries.first() else {
        return Err(crate::error::EmitError::Input("无用户类：无法确定入口".into()));
    };
    let main_short = ctx.short(main_bin);
    let use_path = if main_e.pkg_parts.is_empty() {
        format!("{}::{main_short}", main_e.mod_name)
    } else {
        let mut p = main_e.pkg_parts.clone();
        p.push(main_short.clone());
        p.join("::")
    };
    let bin_name = to_snake(main_bin.rsplit('/').next().unwrap_or(main_bin));
    // 主类自身泛型：静态 main 的路径表达式无推断上下文（E0283），类型实参按擦除取 Object
    let tps = ctx.class(main_bin).map(|ci| ctx.ty.effective_class_type_params(ci).len()).unwrap_or(0);
    let main_call = if tps > 0 {
        let args = vec!["java_runtime::prelude::Object"; tps].join(", ");
        format!("{main_short}::<{args}>::main()")
    } else {
        format!("{main_short}::main()")
    };
    let mut lines = vec![MAIN_ALLOW.to_string()];
    if let Some(top) = user.mod_tree.get(user_src) {
        lines.extend(top.iter().map(|m| user_mod_decl(user_src, m, "")));
    }
    lines.push(format!("use {use_path};"));
    lines.push(String::new());
    lines.push("fn main() {".into());
    let hb = hook_block(ctx, user, jdk, disp);
    if !hb.is_empty() {
        lines.push(hb);
    }
    lines.push(format!("    java_runtime::destroy_java_vm({main_call});"));
    lines.push("}".into());
    lines.push(String::new());
    w.write(&user_src.join("main.rs"), &lines.join("\n"))?;
    Ok(bin_name)
}

/// 用户子包 mod.rs
pub fn write_user_mods(w: &mut Writer, user_src: &Path, user: &UserLayout) -> Result<()> {
    for (dir, children) in &user.mod_tree {
        if dir == user_src {
            continue;
        }
        let mut lines: Vec<String> = children.iter().map(|c| user_mod_decl(dir, c, "pub ")).collect();
        for (m, c) in user.reexport.get(dir).into_iter().flatten() {
            lines.push(format!("pub use {m}::{c};"));
        }
        w.write(&dir.join("mod.rs"), &(lines.join("\n") + "\n"))?;
    }
    Ok(())
}

const USER_LINTS: &[&str] = &[
    "unused_parens",
    "unused_braces",
    "dead_code",
    "unused_assignments",
    "unused_variables",
    "unused_mut",
    "unused_imports",
    "non_snake_case",
    "non_camel_case_types",
    "non_upper_case_globals",
    "unreachable_code",
    "unreachable_patterns",
];

/// user/Cargo.toml、根 Cargo.toml、strict.txt、jdk_feature.txt
pub fn write_cargo_files(ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, bin_name: &str) -> Result<()> {
    let user_dir = out_dir.join("user");
    let mut l = vec![
        "[package]".to_string(),
        "name = \"user\"".into(),
        format!("version = \"{}\"", scratch_pkg_version(&user_dir)),
        "edition = \"2021\"".into(),
        String::new(),
        "[[bin]]".into(),
        format!("name = \"{bin_name}\""),
        "path = \"src/main.rs\"".into(),
        String::new(),
        "[dependencies]".into(),
        "java_runtime    = { path = \"../java_runtime\" }".into(),
        format!("rava_macros = {{ path = \"{}\" }}", ctx.macros_crate.display()),
        String::new(),
        "[lints.rust]".into(),
    ];
    l.extend(USER_LINTS.iter().map(|n| format!("{n} = \"allow\"")));
    l.push(String::new());
    w.write(&user_dir.join("Cargo.toml"), &l.join("\n"))?;
    let root = "[workspace]\nmembers = [\"java_runtime\", \"user\"]\nresolver = \"2\"\n\n[profile.release]\n\
                opt-level = 3\nlto       = true\ncodegen-units = 1\nstrip     = \"symbols\"\n";
    w.write(&out_dir.join("Cargo.toml"), root)?;
    let jrt = out_dir.join("java_runtime");
    w.write(&jrt.join("strict.txt"), if ctx.opts.strict { "1\n" } else { "0\n" })?;
    if let Some(v) = ctx.opts.jdk_major {
        w.write(&jrt.join("jdk_feature.txt"), &format!("{v}\n"))?;
    }
    Ok(())
}

/// 模块资源嵌入表（恒生成；资源缺席时为空表）
pub fn write_module_resources(ctx: &EmitCtx<'_>, w: &mut Writer, jrt_src: &Path) -> Result<()> {
    let res_dir = jrt_src.join("jdk_resources");
    let mut items: Vec<&(String, Vec<u8>)> = ctx.input.module_resources.iter().collect();
    items.sort_by(|a, b| a.0.cmp(&b.0));
    let mut arms = String::new();
    for (p, bytes) in items {
        let f = p.split('/').fold(res_dir.join("module"), |d, s| d.join(s));
        w.write_bytes(&f, bytes)?;
        arms += &format!("        \"{p}\" => Some(include_bytes!(\"module/{p}\")),\n");
    }
    let text = format!(
        "//! 生成：模块资源嵌入表（seeds.toml [module_resources]；字节取自本轮所用 JDK 的 jmod）。\n\
         //! 键为 jmod `classes/` 下的相对路径（`Class.getResourceAsStream(\"/\" + 路径)` 的资源名）。\n\n\
         pub fn lookup(name: &str) -> Option<&'static [u8]> {{\n    match name.trim_start_matches('/') {{\n\
         {arms}        _ => None,\n    }}\n}}\n"
    );
    w.write(&res_dir.join("module_resources.rs"), &text)
}

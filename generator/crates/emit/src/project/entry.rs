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
    parts.push(ctx.declared(bin));
    parts.join("::")
}

/// 泛型类静态路径的擦除 turbofish（`::<Object, ..>`；非泛型为空）：main 里的静态调用没有推断上下文（E0283）
fn erased_turbofish(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let n = ctx.class(bin).map(|ci| ctx.ty.effective_class_type_params(ci).len()).unwrap_or(0);
    if n == 0 {
        return String::new();
    }
    format!("::<{}>", vec!["java_runtime::prelude::Object"; n].join(", "))
}

fn block(head: &str, lines: &[String], tail: &str) -> String {
    format!("{head}\n{}\n{tail}\n", lines.join("\n"))
}

/// 类初始化钩子（`ensure_class_initialized` 按名查表）：枚举形态 / 有 `<clinit>` 的用户类 +
/// 注解枚举种子与分析器 class_init 目标（JDK）
fn class_init_hooks(ctx: &EmitCtx<'_>, user: &UserLayout, jdk: &JdkLayout) -> Vec<String> {
    let mut out = Vec::new();
    for (c, e) in &user.entries {
        // 不透明（L1）类只有类型身份、不初始化，没有 `__class_init`
        let Some(ci) = ctx.class(c).filter(|_| !ctx.is_opaque(c)) else { continue };
        let self_desc = format!("L{c};");
        let enum_shaped = ci.fields().iter().any(|f| f.is_static() && f.desc == self_desc);
        let has_clinit = ci.methods().iter().any(|m| m.name == "<clinit>");
        if !(enum_shaped || has_clinit) {
            continue;
        }
        let mut p = vec!["crate".to_string()];
        p.extend(e.pkg_parts.iter().cloned());
        p.push(e.mod_name.clone());
        p.push(ctx.declared(c));
        out.push(format!(
            "    (\"{c}\", java_runtime::sync_model::__Shared::new(|| {}{}::__class_init())),",
            p.join("::"),
            erased_turbofish(ctx, c)
        ));
    }
    // JDK 侧：注解枚举种子 + 分析器 class_init 事实（Unsafe.ensureClassInitialized 等按名初始化的
    // 目标；`unknown` = 目标类不可定论 → 链上全部有 `<clinit>` 的类）。只登记本轮生成的类
    let jdk_targets: std::collections::BTreeSet<&String> = ctx
        .input
        .annotation_enum_seeds
        .iter()
        .chain(&ctx.input.class_init_targets)
        .filter(|c| jdk.generated.contains(*c) && !ctx.is_opaque(c))
        .collect();
    for c in jdk_targets {
        out.push(format!(
            "    (\"{c}\", java_runtime::sync_model::__Shared::new(|| {}{}::__class_init())),",
            jrt_path(ctx, c),
            erased_turbofish(ctx, c)
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
        .map(|b| {
            let path = format!("{}{}", jrt_path(ctx, b), erased_turbofish(ctx, b));
            format!("        (\"{b}\", {path}::__class_init as fn() -> java_runtime::error::Result<()>),")
        })
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
    bodies: &[&str],
) -> Result<String> {
    let Some((main_bin, main_e)) = user.entries.first() else {
        return Err(crate::error::EmitError::Input("无用户类：无法确定入口".into()));
    };
    let main_short = ctx.declared(main_bin);
    let use_path = if main_e.pkg_parts.is_empty() {
        format!("{}::{main_short}", main_e.mod_name)
    } else {
        let mut p = main_e.pkg_parts.clone();
        p.push(main_short.clone());
        p.join("::")
    };
    let bin_name = to_snake(main_bin.rsplit('/').next().unwrap_or(main_bin));
    // 主类自身泛型：静态 main 的路径表达式无推断上下文（E0283），类型实参按擦除取 Object
    let main_call = format!("{main_short}{}::main()", erased_turbofish(ctx, main_bin));
    let mut lines = vec![MAIN_ALLOW.to_string()];
    let top = user.mod_tree.get(user_src).into_iter().flatten();
    if ctx.opts.batch {
        // 批量模式：每个 bin 以 #[path] 独立包含自己的类文件（不共享 crate 根），单测编译失败互不影响
        for m in top {
            lines.push(format!("#[path = \"../{m}.rs\"]"));
            lines.push(format!("mod {m};"));
        }
    } else {
        lines.extend(top.map(|m| user_mod_decl(user_src, m, "")));
    }
    lines.push(format!("use {use_path};"));
    // 反射元数据表 crate：java_runtime 以导出符号读取其表，此处把它纳入链接
    lines.push("use java_meta as _;".into());
    // 实现层 crate：声明层外壳经导出符号调用其定义，此处把它们纳入链接
    lines.extend(bodies.iter().map(|b| format!("use {b} as _;")));
    lines.push(String::new());
    lines.push("fn main() {".into());
    // 进程级终止约定（panic 钩子）先于一切登记就位：此后任何 panic 同一出口
    lines.push("    java_runtime::create_java_vm();".into());
    let hb = hook_block(ctx, user, jdk, disp);
    if !hb.is_empty() {
        lines.push(hb);
    }
    lines.push(format!("    java_runtime::destroy_java_vm({main_call});"));
    lines.push("}".into());
    lines.push(String::new());
    let file = if ctx.opts.batch { user_src.join("bin").join(format!("{bin_name}.rs")) } else { user_src.join("main.rs") };
    w.write(&file, &lines.join("\n"))?;
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

/// `[lints.rust]` 段（user / lib crate 同一口径）
pub fn lints_section() -> Vec<String> {
    let mut l = vec!["[lints.rust]".to_string()];
    l.extend(USER_LINTS.iter().map(|n| format!("{n} = \"allow\"")));
    l.push(String::new());
    l
}

/// user/Cargo.toml 的依赖行：java_runtime、java_meta、宏 crate、全部 lib crate（声明序）、
/// 全部实现层 crate
fn user_deps(ctx: &EmitCtx<'_>, libs: &[&str], bodies: &[&str]) -> Vec<String> {
    let mut d = vec![
        "java_runtime    = { path = \"../java_runtime\" }".to_string(),
        "java_meta       = { path = \"../java_meta\" }".to_string(),
        format!("rava_macros = {{ path = \"{}\" }}", ctx.macros_crate.display()),
    ];
    d.extend(libs.iter().map(|l| super::lib_crates::dep_line(l)));
    d.extend(bodies.iter().map(|b| super::lib_crates::dep_line(b)));
    d
}

/// 批量模式：向 user/Cargo.toml 追加 `[[bin]]`（同名已在则跳过；插在 `[dependencies]` 前）；
/// 文件缺席时先建最小清单
fn append_cargo_bin(ctx: &EmitCtx<'_>, w: &mut Writer, user_dir: &Path, bin_name: &str, bodies: &[&str]) -> Result<()> {
    let path = user_dir.join("Cargo.toml");
    let new_bin = format!("\n[[bin]]\nname = \"{bin_name}\"\npath = \"src/bin/{bin_name}.rs\"\n");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => {
            // 实现层 crate 依赖补齐（清单先于拆层建立、或本轮装箱数增加）
            let c = with_dep_lines(c, bodies);
            if c.contains(&format!("name = \"{bin_name}\"")) {
                return w.write(&path, &c);
            }
            c
        }
        Err(_) => {
            let mut base = vec![
                "[package]".to_string(),
                "name = \"user\"".into(),
                format!("version = \"{}\"", scratch_pkg_version(user_dir)),
                "edition = \"2021\"".into(),
                String::new(),
                "[dependencies]".into(),
            ];
            base.extend(user_deps(ctx, &[], bodies));
            base.push(String::new());
            base.join("\n")
        }
    };
    let out = match content.find("[dependencies]") {
        Some(i) => format!("{}{new_bin}\n{}", &content[..i], &content[i..]),
        None => content + &new_bin,
    };
    w.write(&path, &out)
}

/// 清单缺席的依赖行插到 `[dependencies]` 段首
fn with_dep_lines(content: String, crates: &[&str]) -> String {
    let missing: Vec<String> = crates
        .iter()
        .filter(|c| !content.lines().any(|l| l.split_whitespace().next() == Some(**c)))
        .map(|c| super::lib_crates::dep_line(c) + "\n")
        .collect();
    match content.find("[dependencies]\n") {
        Some(i) if !missing.is_empty() => {
            let at = i + "[dependencies]\n".len();
            format!("{}{}{}", &content[..at], missing.concat(), &content[at..])
        }
        _ => content,
    }
}

/// user/Cargo.toml（批量模式为追加 `[[bin]]`）、根 Cargo.toml、strict.txt、jdk_feature.txt
pub fn write_cargo_files(
    ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, bin_name: &str, libs: &[&str], bodies: &[&str],
) -> Result<()> {
    let user_dir = out_dir.join("user");
    if ctx.opts.batch {
        append_cargo_bin(ctx, w, &user_dir, bin_name, bodies)?;
    } else {
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
        ];
        l.extend(user_deps(ctx, libs, bodies));
        l.push(String::new());
        l.extend(lints_section());
        w.write(&user_dir.join("Cargo.toml"), &l.join("\n"))?;
    }
    let members: Vec<String> = ["java_runtime", "java_meta"]
        .into_iter()
        .chain(libs.iter().copied())
        .chain(bodies.iter().copied())
        .chain(["user"])
        .map(|m| format!("\"{m}\""))
        .collect();
    // dev 构建：只保留行号表（回溯仍带文件行号；完整调试信息使大闭包 rustc 峰值内存翻倍、
    // 编译耗时约 +20%），关闭增量（scratch 每轮重生成，增量元数据只占内存与磁盘）。
    // 两项只影响调试信息与编译缓存，不影响程序语义。
    // 两个 profile 都 panic = "abort"：Java 异常经 Result 传播，不依赖 unwind；panic 只来自存根 /
    // 运行时缺陷，由 create_java_vm 的钩子以退出码 101 终止（与 unwind 形态退出码、stderr 一致），
    // 免除全部 unwind 清理路径（landing pad）
    let root = format!(
        "[workspace]\nmembers = [{}]\nresolver = \"2\"\n\n[profile.release]\n\
         opt-level = 3\nlto       = true\ncodegen-units = 1\ndebug     = \"line-tables-only\"\npanic     = \"abort\"\n\n\
         [profile.dev]\ndebug = \"line-tables-only\"\nincremental = false\npanic = \"abort\"\n",
        members.join(", ")
    );
    w.write(&out_dir.join("Cargo.toml"), &root)?;
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

/// 闭包派生表文件（scratch 相对路径）：java_meta 的 `lib.rs` 以 `include!` 引入
pub const CLOSURE_TABLES: &str = "closure_input/closure_tables.rs";

/// 闭包派生表：模块服务表（`__java_meta_MODULE_SERVICES`，BootLoader.getServicesCatalog 装填引导服务目录）
/// 与 VM 初始系统属性表（`__java_meta_VM_CONST_PROPERTIES` / `__java_meta_VM_DYNAMIC_PROPERTIES`，
/// System.registerNatives 写入）。java_runtime::meta 以同名 extern 声明读取；事实随闭包（即用户代码）变化，
/// 放在 java_meta 才不连带重编 java_runtime。每次构建写入（内容相同不重写）
pub fn write_closure_tables(ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path) -> Result<()> {
    let input = &ctx.input;
    let mut src = String::from(
        "// 生成：闭包派生表（模块服务表 / VM 初始系统属性表），由 java_meta 的 lib.rs 引入。\n\n\
         #[export_name = \"__java_meta_MODULE_SERVICES\"] pub static MODULE_SERVICES: &[(&str, &str)] = &[\n",
    );
    for (s, p) in &input.module_services {
        src += &format!("    ({s:?}, {p:?}),\n");
    }
    src += "];\n#[export_name = \"__java_meta_VM_CONST_PROPERTIES\"] pub static VM_CONST_PROPERTIES: &[(&str, &str)] = &[\n";
    for (k, v) in &input.system_properties.values {
        src += &format!("    ({k:?}, {v:?}),\n");
    }
    src += "];\n#[export_name = \"__java_meta_VM_DYNAMIC_PROPERTIES\"] pub static VM_DYNAMIC_PROPERTIES: &[&str] = &[\n";
    for k in &input.system_properties.dynamic {
        src += &format!("    {k:?},\n");
    }
    src += "];\n";
    w.write(&out_dir.join(CLOSURE_TABLES), &src)
}

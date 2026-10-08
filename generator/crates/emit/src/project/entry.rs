//! user crate 入口与工程文件：main.rs（启动钩子登记）、user / 根 Cargo.toml、用户子包 mod.rs、
//! 模块资源表、strict.txt / jdk_feature.txt。

use std::path::Path;

use super::fs::Writer;
use super::layout::{JdkLayout, UserLayout};
use super::module_side::path_dep;
use crate::ctx::{EmitCtx, USER_CRATE};
use crate::phase2::dispatch::registration_turbofish;
use crate::phase2::Emissions;
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

/// JDK 类在 user crate 中的完整路径（`<模块 crate>::java::util::X`）
fn jrt_path(ctx: &EmitCtx<'_>, bin: &str) -> String {
    let mut parts = vec![ctx.crate_of(bin).to_string()];
    let mut segs: Vec<&str> = bin.split('/').collect();
    segs.pop();
    parts.extend(segs.iter().map(|p| safe_pkg_part(p)));
    parts.push(ctx.declared(bin));
    parts.join("::")
}

fn block(head: &str, lines: &[String], tail: &str) -> String {
    format!("{head}\n{}\n{tail}\n", lines.join("\n"))
}

/// 类初始化钩子：枚举形态 / 有 `<clinit>` 的用户类 + 注解枚举种子与按镜像初始化目标（JDK）。
/// 泛型类路径的类型实参按登记约定取 Object（钩子闭包无推断上下文，E0283）
fn class_init_hooks(ctx: &EmitCtx<'_>, user: &UserLayout, jdk: &JdkLayout, ems: &Emissions) -> Vec<String> {
    let rt = ctx.crates().root();
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
            "    (\"{c}\", {rt}::sync_model::__Shared::new(|| {}{}::__class_init())),",
            p.join("::"),
            registration_turbofish(ctx, ems, c)
        ));
    }
    // 注解枚举元素类型、按类镜像强制初始化的目标类：运行期按名触发 `<clinit>`；
    // 不透明（L1）类只有类型身份、不初始化，不登记
    let mut seen = std::collections::BTreeSet::new();
    for en in ctx.input.annotation_enum_seeds.iter().chain(&ctx.input.mirror_init_classes) {
        if !jdk.generated.contains(en) || ctx.is_opaque(en) || !seen.insert(en) {
            continue;
        }
        out.push(format!(
            "    (\"{en}\", {rt}::sync_model::__Shared::new(|| {}{}::__class_init())),",
            jrt_path(ctx, en),
            registration_turbofish(ctx, ems, en)
        ));
    }
    out
}

/// main 启动段（钩子登记全部段落）
fn hook_block(ctx: &EmitCtx<'_>, user: &UserLayout, jdk: &JdkLayout, ems: &Emissions, disp: &DispatchReg) -> String {
    let rt = ctx.crates().root();
    let mut hb = String::new();
    let hooks = class_init_hooks(ctx, user, jdk, ems);
    if !hooks.is_empty() {
        hb += &block(&format!("    {rt}::register_class_init_hooks(&["), &hooks, "    ]);");
    }
    if !disp.methods.is_empty() {
        hb += &block(&format!("    {rt}::reflect_dispatch::register_method_dispatch(&["), &disp.methods, "    ]);");
    }
    if !disp.fields.is_empty() {
        hb += &block(&format!("    {rt}::reflect_dispatch::register_field_dispatch(&["), &disp.fields, "    ]);");
    }
    // 构建期引导映像（HotSpot initPhase1–3 的构建期求值结果）：登记钩子之后装入，main 之前完成引导的残差部分
    hb += &format!("    {rt}::{}::{}();\n", super::boot_image::MODULE, super::boot_image::START_FN);
    hb
}

/// 性能类测试的构建档位（cargo 自定义 profile，产物在 `<target>/dev-opt/`）：dev 语义、全部 crate opt-level 1。
/// 档案 crate 跨测试共享编译缓存，只付一次优化代价；用户 crate 只有本例几个类，opt 1 的编译增量在秒级。
/// 用户 crate 若取 opt 0，运行时的 `#[inline]` 泛型存取在用户 crate 内单态化后逐层成调用（SelfNumbers 慢 11 倍）
pub const DEV_OPT_PROFILE: &str = "dev-opt";

/// 可选体积档（cargo 自定义 profile，产物在 `<target>/release-small/`）：继承 release（fat LTO、codegen-units 1、
/// strip），只把 opt-level 换成 "s"。档位取舍见 docs/plans/2026-10-04-binary-size.md §B3
pub const RELEASE_SMALL_PROFILE: &str = "release-small";

const MAIN_ALLOW: &str =
    "#![allow(unused_variables, unused_mut, dead_code, non_snake_case, unused_imports, non_camel_case_types)]";

/// user/src/main.rs；返回 bin 名
pub fn write_main(
    ctx: &EmitCtx<'_>,
    w: &mut Writer,
    user_src: &Path,
    user: &UserLayout,
    jdk: &JdkLayout,
    ems: &Emissions,
    disp: &DispatchReg,
) -> Result<String> {
    let crates = ctx.crates();
    let rt = crates.root();
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
    let main_call = format!("{main_short}{}::main()", registration_turbofish(ctx, ems, main_bin));
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
    // 反射元数据表 crate：运行时以导出符号读取其表，此处把它纳入链接
    lines.push("use java_meta as _;".into());
    // 用户类的反射元数据行（`meta_sides`）：启动时登记，与档案侧表合并查询
    let meta_mod = super::meta_sides::USER_META_MOD;
    if ctx.opts.batch {
        lines.push(format!("#[path = \"../{meta_mod}.rs\"]"));
    }
    lines.push(format!("mod {meta_mod};"));
    // 全部模块 crate（根门面连带链接实现层：声明层外壳经导出符号调用其定义）；
    // 只经反射 / 服务加载触达的模块也要纳入链接
    lines.extend(crates.all().iter().map(|c| format!("use {} as _;", c.name)));
    lines.push(String::new());
    lines.push("fn main() {".into());
    lines.push(format!("    {rt}::meta::register_user(&{meta_mod}::USER_META);"));
    // 进程级终止约定（panic 钩子）先于一切登记就位：此后任何 panic 同一出口
    lines.push(format!("    {rt}::create_java_vm();"));
    let hb = hook_block(ctx, user, jdk, ems, disp);
    if !hb.is_empty() {
        lines.push(hb);
    }
    lines.push(format!("    {rt}::destroy_java_vm({main_call});"));
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

/// user crate 直接依赖的 crate 名：全部模块 crate（根为门面，链接者经它连带实现层）、java_meta
fn linked_crates(ctx: &EmitCtx<'_>) -> Vec<String> {
    let mut v: Vec<String> = ctx.crates().all().iter().map(|c| c.name.clone()).collect();
    v.push(META_CRATE.to_string());
    v
}

/// 反射元数据表 crate 名
pub const META_CRATE: &str = "java_meta";

/// user/Cargo.toml 的依赖行：全部模块 crate、java_meta、宏 crate、全部 lib crate（声明序）
fn user_deps(ctx: &EmitCtx<'_>, libs: &[&str]) -> Vec<String> {
    let mut d: Vec<String> = linked_crates(ctx).iter().map(|c| path_dep(c, c)).collect();
    d.push(format!("rava_macros     = {{ path = \"{}\" }}", ctx.macros_crate.display()));
    d.extend(libs.iter().map(|l| path_dep(l, l)));
    d
}

/// 批量模式：向 user/Cargo.toml 追加 `[[bin]]`（同名已在则跳过；插在 `[dependencies]` 前）；
/// 文件缺席时先建最小清单
fn append_cargo_bin(ctx: &EmitCtx<'_>, w: &mut Writer, user_dir: &Path, bin_name: &str) -> Result<()> {
    let path = user_dir.join("Cargo.toml");
    let new_bin = format!("\n[[bin]]\nname = \"{bin_name}\"\npath = \"src/bin/{bin_name}.rs\"\n");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => {
            // 模块 crate 依赖补齐（本轮闭包涉及的模块增加）
            let c = with_dep_lines(c, &linked_crates(ctx));
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
            base.extend(user_deps(ctx, &[]));
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
fn with_dep_lines(content: String, crates: &[String]) -> String {
    let missing: Vec<String> = crates
        .iter()
        .filter(|c| !content.lines().any(|l| l.split_whitespace().next() == Some(c.as_str())))
        .map(|c| path_dep(c, c) + "\n")
        .collect();
    match content.find("[dependencies]\n") {
        Some(i) if !missing.is_empty() => {
            let at = i + "[dependencies]\n".len();
            format!("{}{}{}", &content[..at], missing.concat(), &content[at..])
        }
        _ => content,
    }
}

/// user/Cargo.toml（批量模式为追加 `[[bin]]`）、根 Cargo.toml、strict.txt、jdk_feature.txt。
/// `decl_uppers`：根声明层的上层段 crate（链序，不分段时为空）
pub fn write_cargo_files(
    ctx: &EmitCtx<'_>, w: &mut Writer, out_dir: &Path, bin_name: &str, libs: &[&str], bodies: &[&str], decl_uppers: &[&str],
) -> Result<()> {
    let user_dir = out_dir.join("user");
    if ctx.opts.batch {
        append_cargo_bin(ctx, w, &user_dir, bin_name)?;
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
        l.extend(user_deps(ctx, libs));
        l.push(String::new());
        l.extend(lints_section());
        w.write(&user_dir.join("Cargo.toml"), &l.join("\n"))?;
    }
    let crates = ctx.crates();
    let decl = crates.decl();
    let members: Vec<String> = std::iter::once(decl.as_str())
        .chain(decl_uppers.iter().copied())
        .chain(bodies.iter().copied())
        .chain(crates.all().iter().map(|c| c.name.as_str()))
        .chain([META_CRATE])
        .chain(libs.iter().copied())
        .chain([USER_CRATE])
        .map(|m| format!("\"{m}\""))
        .collect();
    // dev 构建：只保留行号表（回溯仍带文件行号；完整调试信息使大闭包 rustc 峰值内存翻倍、
    // 编译耗时约 +20%），关闭增量（scratch 每轮重生成，增量元数据只占内存与磁盘）。
    // 两项只影响调试信息与编译缓存，不影响程序语义。
    // dev-opt 档（性能类测试，见 DEV_OPT_PROFILE）：继承 dev，全部 crate opt-level 1——
    // 语义检查（溢出 / debug 断言）与 dev 相同，只换优化级
    // 各 profile 都 panic = "abort"：Java 异常经 Result 传播，不依赖 unwind；panic 只来自存根 /
    // 运行时缺陷，由 create_java_vm 的钩子以退出码 101 终止（与 unwind 形态退出码、stderr 一致），
    // 免除全部 unwind 清理路径（landing pad）。
    // release 剥符号表（strip = "symbols"）：取栈按链接期地址表（driver `rava-link`，运行时
    // `pc_map`），不读符号与 DWARF；行号表仍由 rava-link 在剥离前读取。
    // release-small 档（见 RELEASE_SMALL_PROFILE）：继承 release，opt-level "s"（不设 "z" 档，用户 10-05 定）
    let root = format!(
        "[workspace]\nmembers = [{}]\nresolver = \"2\"\n\n[profile.release]\n\
         opt-level = 3\nlto       = true\ncodegen-units = 1\ndebug     = \"line-tables-only\"\npanic     = \"abort\"\nstrip     = \"symbols\"\n\n\
         [profile.{RELEASE_SMALL_PROFILE}]\ninherits = \"release\"\nopt-level = \"s\"\n\n\
         [profile.dev]\ndebug = \"line-tables-only\"\nincremental = false\npanic = \"abort\"\n\n\
         [profile.{DEV_OPT_PROFILE}]\ninherits = \"dev\"\nopt-level = 1\n",
        members.join(", ")
    );
    w.write(&out_dir.join("Cargo.toml"), &root)?;
    let jrt = out_dir.join(&decl);
    w.write(&jrt.join("strict.txt"), if ctx.opts.strict { "1\n" } else { "0\n" })?;
    if let Some(v) = ctx.opts.jdk_major {
        w.write(&jrt.join("jdk_feature.txt"), &format!("{v}\n"))?;
    }
    Ok(())
}


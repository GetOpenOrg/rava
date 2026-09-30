//! 工程发射（← `emitter/project_writer.write_cargo_project`）。
//!
//! 流程：布局 → 逐类文本（JDK 闭包序，再用户类序）→ 第二阶段收尾（继承补声明、SAM 合成、
//! 反射分派；步骤 (d)）→ 落盘 → 包 mod 树 → lib.rs 补全 → 用户子包 mod.rs → main.rs →
//! Cargo.toml / strict.txt / jdk_feature.txt。

pub mod entry;
pub mod fs;
pub mod layout;
pub mod mod_tree;
pub mod overlay;
#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::path::Path;

use indexmap::IndexMap;

pub use overlay::prepare_scratch;

use crate::body::MethodBodyEmitter;
use crate::class_writer::{gen_class_rs, ClassSite};
use crate::ctx::{EmitCtx, ProjectState};
use crate::emission::ClassEmission;
use crate::error::{EmitError, Result};
use entry::DispatchReg;
use fs::Writer;
use layout::{JdkLayout, UserLayout};

/// 发射结果摘要
#[derive(Debug, Default)]
pub struct ProjectReport {
    pub jdk_classes: usize,
    pub user_classes: usize,
    /// 类发射记录（发射序）
    pub emissions: Vec<ClassEmission>,
}

/// 逐类生成文本（尚未落盘）
fn emit_classes(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    w: &Writer,
    jdk: &JdkLayout,
    user: &UserLayout,
) -> Result<IndexMap<String, ClassEmission>> {
    let mut ems = IndexMap::new();
    for (c, path) in &jdk.files {
        let Some(ci) = ctx.class(c) else { continue };
        let text = gen_class_rs(ctx, state, bodies, ci, &ClassSite::Jdk(jdk))?;
        let em = ClassEmission {
            binary_name: c.clone(),
            crate_prefix: "crate".into(),
            path: path.clone(),
            handwritten: w.is_handwritten(path),
            crate_name: "java_runtime".into(),
            text,
        };
        ems.insert(c.clone(), em);
    }
    for (c, e) in &user.entries {
        let Some(ci) = ctx.class(c) else { continue };
        // 字节码引用集（import_gen.collect_referenced）步骤 (b) 接入
        let referenced = BTreeSet::new();
        let site = ClassSite::User { sibling_imports: user.sibling_imports(ctx, c, &referenced) };
        let text = gen_class_rs(ctx, state, bodies, ci, &site)?;
        let em = ClassEmission {
            binary_name: c.clone(),
            crate_prefix: "java_runtime".into(),
            path: e.path.clone(),
            handwritten: false,
            crate_name: "user".into(),
            text,
        };
        ems.insert(c.clone(), em);
    }
    Ok(ems)
}

/// 发射完整 scratch workspace（overlay 需先完成：mod 树按磁盘实际内容重建）
pub fn write_project(ctx: &EmitCtx<'_>, out_dir: &Path, bodies: &mut dyn MethodBodyEmitter) -> Result<ProjectReport> {
    if !ctx.input.lib_crates.is_empty() {
        return Err(EmitError::Unported("lib crate 模式（jar 输入）未移植".into()));
    }
    let jrt_src = out_dir.join("java_runtime").join("src");
    let user_src = out_dir.join("user").join("src");
    let runtime_src = ctx.runtime_src();
    let mut w = Writer::new(out_dir, &runtime_src);
    let jdk = JdkLayout::build(ctx, &jrt_src);
    let user = UserLayout::build(ctx, &user_src);
    let mut state = ProjectState::default();
    let ems = emit_classes(ctx, &mut state, bodies, &w, &jdk, &user)?;
    // 第二阶段收尾（LAMBDA 账本断言 / 继承补声明 / SAM 合成 / 反射分派）：步骤 (d)
    let disp = DispatchReg::default();
    for em in ems.values() {
        w.write(&em.path, &em.text)?;
    }
    entry::write_module_resources(ctx, &mut w, &jrt_src)?;
    mod_tree::write_mod_tree(&jrt_src, Some(&runtime_src), &mut w)?;
    mod_tree::complete_lib_rs(&jrt_src)?;
    entry::write_user_mods(&mut w, &user_src, &user)?;
    let bin = entry::write_main(ctx, &mut w, &user_src, &user, &jdk, &disp)?;
    entry::write_cargo_files(ctx, &mut w, out_dir, &bin)?;
    Ok(ProjectReport {
        jdk_classes: jdk.files.len(),
        user_classes: user.entries.len(),
        emissions: ems.into_values().collect(),
    })
}

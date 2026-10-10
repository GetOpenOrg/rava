//! 档案键与内容摘要（计划 §4.1 第一级键 P）。
//!
//! `P = H(档案格式, folds 版本, 生成器源码树, runtime 树, JDK 主版本, 非用户归档内容, 内容摘要)`：
//! - 内容摘要：档案事实的规范序列化（去掉溯源 via / entry、summary、profile 段；数组已按序排列、
//!   对象键有序），只依赖档案内容，与入口给出顺序、分析计时、溯源选择无关；
//! - 入口集合经内容摘要进入 P：入口变化改变档案内容时 P 随之改变；只改用户侧（不影响 JDK 侧事实）的
//!   入口变化不改变 P，档案可原样复用（开放世界 §3.3）。入口输入摘要（[`entries_digest`]）另记在
//!   `profile.inputs.entries`，供「输入未变则跳过分析」使用，不入 P；
//! - 各路径无关：归档按相对路径与内容取摘要，runtime 树同理（跳过点文件与 `target/`）。
//!
//! 摘要用 [`crate::cache::hash::Fp`]（128 位非密码学哈希，离线环境无密码学摘要 crate；只用于缓存键，
//! 不抗恶意构造）。

use std::path::{Path, PathBuf};

use resolve::Origin;
use serde_json::Value;

use crate::cache::hash::Fp;

/// 档案键的输入（`profile.inputs`）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyInputs {
    /// 生成器源码树摘要（driver 构建期算出）
    pub generator: String,
    /// runtime 树摘要（[`runtime_digest`]）
    pub runtime: String,
    pub jdk_major: u32,
    /// 非用户归档（JDK jmods / 镜像目录 / 依赖库）摘要（[`archives_digest`]）
    pub archives: String,
    /// 入口输入摘要（[`entries_digest`]；记录用，不入 P）
    pub entries: String,
    /// 依赖锁摘要（`--deps`；锁序是构建单元输入的一部分——顺序变化即 P 变化，入 P）
    pub deps_lock: String,
}

/// 内容摘要：去掉溯源与统计后的规范序列化
pub fn content_digest(profile: &Value) -> String {
    let mut v = profile.clone();
    if let Some(o) = v.as_object_mut() {
        o.remove("summary");
        o.remove("profile");
        for key in ["classes", "methods", "missing"] {
            for x in o.get_mut(key).and_then(Value::as_array_mut).into_iter().flatten() {
                if let Some(x) = x.as_object_mut() {
                    x.remove("via");
                    x.remove("entry");
                }
            }
        }
        // modules 行的 §4.2 富化字段是档案层元数据（模块图单一来源），不是闭包事实：
        // 不入内容摘要——覆盖判定（`--covers`）重并时无富化上下文，两侧须一致；
        // jar 身份变化经 profile.inputs.deps_lock（含 sha256）进入 P
        for x in o.get_mut("modules").and_then(Value::as_array_mut).into_iter().flatten() {
            if let Some(x) = x.as_object_mut() {
                x.remove("kind");
                x.remove("crate");
                x.remove("jars");
                x.remove("release");
            }
        }
    }
    let mut f = Fp::default();
    f.field("profile-content", v.to_string().as_bytes());
    f.hex()
}

/// 档案键 P
pub fn profile_key(k: &KeyInputs, content: &str) -> String {
    let mut f = Fp::default();
    f.field("profile_format", &super::PROFILE_FORMAT.to_le_bytes());
    f.field("folds_version", &crate::FOLDS_VERSION.to_le_bytes());
    f.field("generator", k.generator.as_bytes());
    f.field("runtime", k.runtime.as_bytes());
    f.field("jdk_major", &k.jdk_major.to_le_bytes());
    f.field("archives", k.archives.as_bytes());
    f.field("deps_lock", k.deps_lock.as_bytes());
    f.field("content", content.as_bytes());
    f.hex()
}

fn files_under(root: &Path, skip: &dyn Fn(&Path) -> bool) -> Result<Vec<PathBuf>, String> {
    let err = |p: &Path, e: std::io::Error| format!("{}：{e}", p.display());
    if root.is_file() {
        return Ok(vec![root.to_path_buf()]);
    }
    let mut stack = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).map_err(|e| err(&d, e))? {
            let p = e.map_err(|e| err(&d, e))?.path();
            if skip(&p) {
                continue;
            }
            if p.is_dir() {
                stack.push(p);
            } else {
                files.push(p);
            }
        }
    }
    files.sort();
    Ok(files)
}

/// 文件或目录树的摘要：相对路径 + 内容（与所在位置无关）
fn tree_digest(root: &Path, skip: &dyn Fn(&Path) -> bool) -> Result<String, String> {
    let mut f = Fp::default();
    for p in files_under(root, skip)? {
        let rel = p.strip_prefix(root).unwrap_or(Path::new(""));
        f.field("path", rel.to_string_lossy().as_bytes());
        f.field("data", &std::fs::read(&p).map_err(|e| format!("{}：{e}", p.display()))?);
    }
    Ok(f.hex())
}

fn hidden_or_target(p: &Path) -> bool {
    p.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with('.') || n == "target")
}

/// runtime 树（仓库 `runtime/`：java_runtime 清单与手写层、java_support、rava_macros）的摘要
pub fn runtime_digest(runtime_root: &Path) -> Result<String, String> {
    tree_digest(runtime_root, &hidden_or_target)
}

/// 非用户归档的摘要：每个归档按来源与树摘要计入，排序去重（与路径、给出顺序无关）
pub fn archives_digest(archives: &[(Origin, PathBuf)]) -> Result<String, String> {
    let mut items: Vec<String> = Vec::new();
    for (o, p) in archives.iter().filter(|(o, _)| !o.is_program()) {
        items.push(format!("{o:?} {}", tree_digest(p, &|_| false)?));
    }
    items.sort();
    items.dedup();
    let mut f = Fp::default();
    for i in items {
        f.field("archive", i.as_bytes());
    }
    Ok(f.hex())
}

/// 入口输入摘要：(入口名, 该入口输入的摘要) 按名排序后计入
pub fn entries_digest(items: &[(String, String)]) -> String {
    let mut v: Vec<&(String, String)> = items.iter().collect();
    v.sort();
    let mut f = Fp::default();
    for (n, d) in v {
        f.field("entry", n.as_bytes());
        f.field("input", d.as_bytes());
    }
    f.hex()
}

/// 一组文件内容的摘要（入口输入：源文件 / 依赖库 / 选项串），按给出顺序计入
pub fn files_digest(parts: &[(String, PathBuf)]) -> Result<String, String> {
    let mut f = Fp::default();
    for (label, p) in parts {
        f.field("label", label.as_bytes());
        if !p.as_os_str().is_empty() {
            f.field("tree", tree_digest(p, &hidden_or_target)?.as_bytes());
        }
    }
    Ok(f.hex())
}

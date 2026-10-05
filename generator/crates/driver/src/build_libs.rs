//! 依赖锁驱动的 lib crate 装配（V12 §3.2；`--lib NAME=JAR[:seed=]` 已删除）。
//!
//! `--deps deps.lock.toml` + `--cp <锁条目名>[,…]` 选出入口类路径（锁序）；crate 名 =
//! jar 模块的 crate 名（`resolve::modules` 按模块名映射，冲突加 FNV 后缀）；种子类由
//! `--seed-class` 显式给出（整包翻译 = 种子覆盖全部 jar 类），不再挂在 lib 声明上。

use std::path::PathBuf;

use input::LibCrate;
use resolve::classpath::{ClassPath, LibMeta};
use resolve::modules::ModuleFacts;

use crate::build_opts::BuildOpts;
use crate::deps_lock::DepsLock;

/// 入口类路径上的一个库条目（锁序）：路径 + 依赖锁元数据（喂给命名兜底链）
#[derive(Debug, Clone)]
pub struct LibEntry {
    pub path: PathBuf,
    pub meta: LibMeta,
}

/// 装配结果（锁序）
#[derive(Debug, Default)]
pub struct Libs {
    pub crates: Vec<LibCrate>,
    /// jar 绝对路径（javac `-cp` 与类路径 `Origin::Lib`）
    pub jars: Vec<PathBuf>,
}

/// 解析 `--deps` + `--cp`：读锁、按条目名取子集（锁序）。无 `--deps` 即无库
pub fn select_entries(o: &BuildOpts) -> Result<Vec<LibEntry>, String> {
    let Some(d) = &o.deps else { return Ok(Vec::new()) };
    let lock = DepsLock::load(d)?;
    Ok(lock
        .select(&o.cp)?
        .iter()
        .map(|j| LibEntry {
            path: j.path.clone(),
            meta: LibMeta { coordinate: j.coordinate.clone(), module: j.module.clone() },
        })
        .collect())
}

/// lib crate 装配：crate 名取 jar 模块的 crate 名（模块图来自类路径——`set_lib_meta` 与
/// 档案加入须已完成、`resolve::modules::check` 已过，无名 jar 在那里已报错）
pub fn from_lock(entries: &[LibEntry], cp: &ClassPath, facts: &ModuleFacts) -> Result<Libs, String> {
    let g = facts.graph(cp);
    let archives = cp.archives();
    let mut out = Libs::default();
    for e in entries {
        let idx = archives
            .iter()
            .position(|(_, p)| p == &e.path)
            .ok_or_else(|| format!("{}：不在类路径上（先经 class_path 加入）", e.path.display()))?;
        let module = facts.archive_module(idx);
        let name = module
            .and_then(|m| g.crate_name(m))
            .ok_or_else(|| format!("{}：无法确定模块 crate 名（模块图缺失）", e.path.display()))?
            .to_string();
        let lc = LibCrate::from_jar(&name, &e.path, cp.release()).map_err(|e| e.to_string())?;
        if lc.jar_classes.is_empty() {
            return Err(format!("jar 中未找到类条目：{}", e.path.display()));
        }
        let base = e.path.file_name().map(|f| f.to_string_lossy().to_string()).unwrap_or_default();
        println!("[jar] {name} ← {base}（{} 类）", lc.jar_classes.len());
        out.jars.push(e.path.clone());
        out.crates.push(lc);
    }
    Ok(out)
}

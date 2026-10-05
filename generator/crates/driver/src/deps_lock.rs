//! 依赖锁（`deps.lock.toml`，V12 §3.2）：构建单元的第三方 jar 清单（类路径序）。
//!
//! rava 只读不写；由 `scripts/fetch_pilot_deps.sh`（或构建工具插件）生成。
//! 锁条目名 = 坐标 artifactId（无坐标 jar 用文件名去 `.jar`），条目名须唯一
//! （同一构建单元同模块单版本的裁定落在锁生成侧）。

use std::collections::HashSet;
use std::path::{Path, PathBuf};

/// 锁内一个 jar 条目
#[derive(Debug, Clone)]
pub struct LockJar {
    /// 锁条目名（`--cp` 与 form.toml `cp[]` 引用）
    pub name: String,
    /// Maven 坐标 `group:artifact:version`（可缺省）
    pub coordinate: Option<String>,
    /// jar 路径（load 后为绝对路径）
    pub path: PathBuf,
    pub sha256: String,
    /// 显式模块名（模块命名兜底链的最后一环之前）
    pub module: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DepsLock {
    pub release: u32,
    /// 类路径序（锁内声明序即构建单元输入的一部分）
    pub jars: Vec<LockJar>,
}

impl DepsLock {
    /// 读锁并校验：条目名唯一、条目字段齐全、jar 文件在位
    pub fn load(path: &Path) -> Result<DepsLock, String> {
        let text = std::fs::read_to_string(path).map_err(|e| format!("{}：{e}", path.display()))?;
        let t: toml::Table = text.parse().map_err(|e| format!("{}：{e}", path.display()))?;
        let release = t.get("release").and_then(|v| v.as_integer()).filter(|r| *r > 0).ok_or("deps.lock 缺 release")?;
        let base = path.parent().unwrap_or(Path::new("."));
        let entries = t.get("jar").and_then(|v| v.as_array()).ok_or("deps.lock 缺 [[jar]]")?;
        let mut jars = Vec::with_capacity(entries.len());
        for (i, e) in entries.iter().enumerate() {
            let tb = e.as_table().ok_or_else(|| format!("[[jar]] 第 {i} 项须为表"))?;
            let coordinate = tb.get("coordinate").and_then(|v| v.as_str()).map(str::to_string);
            let rel = tb.get("path").and_then(|v| v.as_str()).ok_or_else(|| format!("[[jar]] 第 {i} 项缺 path"))?;
            let path = base.join(rel);
            let sha256 = tb.get("sha256").and_then(|v| v.as_str()).ok_or_else(|| format!("[[jar]] 第 {i} 项缺 sha256"))?.to_string();
            let module = tb.get("module").and_then(|v| v.as_str()).map(str::to_string);
            let name = coordinate
                .as_deref()
                .and_then(|c| c.split(':').nth(1))
                .filter(|a| !a.is_empty())
                .map(str::to_string)
                .unwrap_or_else(|| rel.rsplit('/').next().unwrap_or(rel).trim_end_matches(".jar").to_string());
            if name.is_empty() {
                return Err(format!("[[jar]] 第 {i} 项条目名为空"));
            }
            jars.push(LockJar { name, coordinate, path, sha256, module });
        }
        let mut seen = HashSet::new();
        for j in &jars {
            if !seen.insert(j.name.clone()) {
                return Err(format!("锁条目名重复：{}（同一模块单版本，锁内只能有一条）", j.name));
            }
            if !j.path.is_file() {
                return Err(format!("锁内 jar 不存在：{}", j.path.display()));
            }
        }
        Ok(DepsLock { release: release as u32, jars })
    }

    /// 按条目名取子集：**顺序取锁序**（入口给出的顺序不影响类路径，`P` 由此不随入口序变化）；
    /// 未知名报错
    pub fn select(&self, names: &[String]) -> Result<Vec<LockJar>, String> {
        for n in names {
            if !self.jars.iter().any(|j| &j.name == n) {
                return Err(format!("--cp 条目 {n} 不在依赖锁中"));
            }
        }
        let want: HashSet<&str> = names.iter().map(String::as_str).collect();
        Ok(self.jars.iter().filter(|j| want.contains(j.name.as_str())).cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_lock(dir: &Path, body: &str) -> PathBuf {
        let p = dir.join("deps.lock.toml");
        std::fs::write(&p, body).unwrap();
        p
    }

    fn fixture_dir(tag: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("rava_deps_lock_{tag}_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(d.join("libs")).unwrap();
        std::fs::write(d.join("libs/a-1.jar"), b"x").unwrap();
        std::fs::write(d.join("libs/b-2.jar"), b"y").unwrap();
        d
    }

    #[test]
    fn load_parses_and_derives_entry_names() {
        let d = fixture_dir("load");
        let p = write_lock(
            &d,
            "release = 21\n[[jar]]\ncoordinate = \"org.hamcrest:hamcrest:3.0\"\npath = \"libs/a-1.jar\"\nsha256 = \"aa\"\n\
             [[jar]]\npath = \"libs/b-2.jar\"\nsha256 = \"bb\"\n",
        );
        let l = DepsLock::load(&p).unwrap();
        assert_eq!(l.release, 21);
        assert_eq!(l.jars.len(), 2);
        assert_eq!(l.jars[0].name, "hamcrest", "条目名取坐标 artifactId");
        assert_eq!(l.jars[1].name, "b-2", "无坐标时用文件名");
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn duplicate_entry_name_and_missing_jar_are_errors() {
        let d = fixture_dir("err");
        let p = write_lock(
            &d,
            "release = 21\n[[jar]]\ncoordinate = \"g:a:1\"\npath = \"libs/a-1.jar\"\nsha256 = \"aa\"\n\
             [[jar]]\ncoordinate = \"h:a:2\"\npath = \"libs/b-2.jar\"\nsha256 = \"bb\"\n",
        );
        assert!(DepsLock::load(&p).unwrap_err().contains("条目名重复"));
        let p = write_lock(&d, "release = 21\n[[jar]]\npath = \"libs/none.jar\"\nsha256 = \"aa\"\n");
        assert!(DepsLock::load(&p).unwrap_err().contains("不存在"));
        let _ = std::fs::remove_dir_all(&d);
    }

    #[test]
    fn select_takes_lock_order_not_given_order() {
        let d = fixture_dir("select");
        let p = write_lock(
            &d,
            "release = 21\n[[jar]]\ncoordinate = \"org.hamcrest:hamcrest:3.0\"\npath = \"libs/a-1.jar\"\nsha256 = \"aa\"\n\
             [[jar]]\ncoordinate = \"junit:junit:4.13.2\"\npath = \"libs/b-2.jar\"\nsha256 = \"bb\"\n",
        );
        let l = DepsLock::load(&p).unwrap();
        let sel = l.select(&["junit".into(), "hamcrest".into()]).unwrap();
        assert_eq!(sel.iter().map(|j| j.name.as_str()).collect::<Vec<_>>(), vec!["hamcrest", "junit"]);
        assert!(l.select(&["nope".into()]).is_err());
        let _ = std::fs::remove_dir_all(&d);
    }
}

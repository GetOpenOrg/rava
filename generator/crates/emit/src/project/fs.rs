//! 落盘（`project_writer._write` / `_is_handwritten` 的移植）：手写真源同路径文件永不覆盖。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::error::{io_err, Result};

/// 生成标记：块级宏调用（手写文件不含）
pub const GEN_MARKER: &str = "rava_macros::java_class";

/// scratch 写出器：记录本轮写出（或内容相同跳过）的路径
pub struct Writer {
    /// scratch 的 `java_runtime/src`
    jrt_src: PathBuf,
    /// 手写真源 `runtime/java_runtime/src`
    runtime_src: PathBuf,
    written: BTreeSet<PathBuf>,
}

impl Writer {
    pub fn new(out_dir: &Path, runtime_src: &Path) -> Writer {
        Writer {
            jrt_src: out_dir.join("java_runtime").join("src"),
            runtime_src: runtime_src.to_path_buf(),
            written: BTreeSet::new(),
        }
    }

    /// `java_runtime/src/` 下的 .rs 在手写真源同相对路径存在（mod.rs / lib.rs 除外）
    pub fn is_handwritten(&self, path: &Path) -> bool {
        if path.extension().is_none_or(|e| e != "rs") {
            return false;
        }
        let base = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if base == "mod.rs" || base == "lib.rs" {
            return false;
        }
        match path.strip_prefix(&self.jrt_src) {
            Ok(rel) => self.runtime_src.join(rel).exists(),
            Err(_) => false,
        }
    }

    /// 写文件：手写文件跳过；内容相同不重写（保留 mtime）
    pub fn write(&mut self, path: &Path, content: &str) -> Result<()> {
        if self.is_handwritten(path) {
            return Ok(());
        }
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| io_err(&dir.display().to_string(), e))?;
        }
        self.written.insert(path.to_path_buf());
        if std::fs::read(path).is_ok_and(|old| old == content.as_bytes()) {
            return Ok(());
        }
        std::fs::write(path, content).map_err(|e| io_err(&path.display().to_string(), e))
    }

    /// 写二进制（内容相同不重写）
    pub fn write_bytes(&mut self, path: &Path, data: &[u8]) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| io_err(&dir.display().to_string(), e))?;
        }
        if std::fs::read(path).is_ok_and(|old| old == data) {
            return Ok(());
        }
        std::fs::write(path, data).map_err(|e| io_err(&path.display().to_string(), e))
    }

    pub fn written_this_run(&self, path: &Path) -> bool {
        self.written.contains(path)
    }
}

/// 文件含生成标记（读取失败按 None）
pub fn has_marker(path: &Path) -> Option<bool> {
    std::fs::read(path)
        .ok()
        .map(|b| String::from_utf8_lossy(&b).contains(GEN_MARKER))
}

/// os.walk 形态的递归列举：(目录, 子目录名, 文件名)，目录名 / 文件名按字典序
pub fn walk(root: &Path) -> Vec<(PathBuf, Vec<String>, Vec<String>)> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(rd) = std::fs::read_dir(&dir) else { continue };
        let mut dirs = Vec::new();
        let mut files = Vec::new();
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            match e.file_type() {
                Ok(t) if t.is_dir() => dirs.push(name),
                Ok(_) => files.push(name),
                Err(_) => {}
            }
        }
        dirs.sort();
        files.sort();
        for d in dirs.iter().rev() {
            stack.push(dir.join(d));
        }
        out.push((dir, dirs, files));
    }
    out
}

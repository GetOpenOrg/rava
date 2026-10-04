//! 落盘（`project_writer._write` / `_is_handwritten` 的移植）：手写真源同路径文件永不覆盖。

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::error::{io_err, Result};

/// 生成标记：块级宏调用（手写文件不含）
pub const GEN_MARKER: &str = "rava_macros::java_class";

/// scratch 写出器：记录本轮写出（或内容相同跳过）的路径
pub struct Writer {
    /// scratch 的各 JDK 模块 crate 源码树（手写伴随文件可落入其中任一）
    jdk_srcs: Vec<PathBuf>,
    /// 手写真源 `runtime/java_runtime/src`
    runtime_src: PathBuf,
    written: BTreeSet<PathBuf>,
}

impl Writer {
    pub fn new(jdk_srcs: &[PathBuf], runtime_src: &Path) -> Writer {
        Writer {
            jdk_srcs: jdk_srcs.to_vec(),
            runtime_src: runtime_src.to_path_buf(),
            written: BTreeSet::new(),
        }
    }

    /// JDK 源码树下的 .rs 在手写真源同相对路径存在（mod.rs / lib.rs 除外）
    pub fn is_handwritten(&self, path: &Path) -> bool {
        if path.extension().is_none_or(|e| e != "rs") {
            return false;
        }
        let base = path.file_name().and_then(|n| n.to_str()).unwrap_or_default();
        if base == "mod.rs" || base == "lib.rs" {
            return false;
        }
        self.jdk_srcs
            .iter()
            .find_map(|src| path.strip_prefix(src).ok())
            .is_some_and(|rel| self.runtime_src.join(rel).exists())
    }

    /// 写文件：手写文件跳过；内容相同不重写（保留 mtime）
    pub fn write(&mut self, path: &Path, content: &str) -> Result<()> {
        if self.put(path, content)? {
            self.written.insert(path.to_path_buf());
        }
        Ok(())
    }

    /// 批量写文件（语义同逐个 [`Writer::write`]）：各文件互不相干，按 `jobs` 并行落盘；
    /// 出错时报发射序上第一个错误
    pub fn write_all(&mut self, jobs: usize, files: &[(&Path, &str)]) -> Result<()> {
        let this: &Writer = self;
        let done = crate::par::par_map(jobs, files, |(p, c)| this.put(p, c));
        for ((p, _), r) in files.iter().zip(done) {
            if r? {
                self.written.insert(p.to_path_buf());
            }
        }
        Ok(())
    }

    /// 单文件落盘；返回是否计入本轮已写（手写文件不计）
    fn put(&self, path: &Path, content: &str) -> Result<bool> {
        if self.is_handwritten(path) {
            return Ok(false);
        }
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir).map_err(|e| io_err(&dir.display().to_string(), e))?;
        }
        if std::fs::read(path).is_ok_and(|old| old == content.as_bytes()) {
            return Ok(true);
        }
        std::fs::write(path, content).map_err(|e| io_err(&path.display().to_string(), e))?;
        Ok(true)
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

//! 类档案：jmod（`JM\x01\x00` 头 + zip，类在 `classes/` 下）、jar（zip）、类目录。

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use crate::Error;

enum Kind {
    Zip(zip::ZipArchive<BufReader<File>>),
    Dir(PathBuf),
}

pub struct Archive {
    pub path: PathBuf,
    kind: Kind,
    /// binary name → zip 条目下标（目录档案为空，按路径直接读）
    index: HashMap<String, usize>,
    /// 非类资源：档案内路径（去 `classes/` 前缀）→ 条目下标
    resources: HashMap<String, usize>,
    /// 根目录的 `module-info.class` 条目下标
    module_info: Option<usize>,
}

impl Archive {
    pub fn open(path: &Path) -> Result<Self, Error> {
        if path.is_dir() {
            return Ok(Archive {
                path: path.to_path_buf(),
                kind: Kind::Dir(path.to_path_buf()),
                index: HashMap::new(),
                resources: HashMap::new(),
                module_info: None,
            });
        }
        let io = |e: std::io::Error| Error::Io(path.display().to_string(), e.to_string());
        let file = File::open(path).map_err(io)?;
        let prefix = if path.extension().is_some_and(|e| e == "jmod") { "classes/" } else { "" };
        // jmod 的 4 字节头位于 zip 数据之前：zip 读取按中央目录自动识别前置数据偏移
        let zip = zip::ZipArchive::new(BufReader::new(file))
            .map_err(|e| Error::Io(path.display().to_string(), e.to_string()))?;
        let mut index = HashMap::new();
        let mut resources = HashMap::new();
        let mut module_info = None;
        for i in 0..zip.len() {
            let Some(name) = zip.name_for_index(i) else { continue };
            let Some(rest) = name.strip_prefix(prefix) else { continue };
            if rest == "module-info.class" {
                module_info = Some(i);
            } else if let Some(bin) = rest.strip_suffix(".class") {
                if !bin.ends_with("module-info") {
                    index.insert(bin.to_string(), i);
                }
            } else if !rest.ends_with('/') {
                resources.insert(rest.to_string(), i);
            }
        }
        Ok(Archive { path: path.to_path_buf(), kind: Kind::Zip(zip), index, resources, module_info })
    }

    /// 档案内全部类的 binary name（目录档案递归枚举）
    pub fn class_names(&self) -> Vec<String> {
        match &self.kind {
            Kind::Zip(_) => {
                let mut v: Vec<String> = self.index.keys().cloned().collect();
                v.sort();
                v
            }
            Kind::Dir(root) => {
                let mut v = Vec::new();
                walk(root, root, &mut v);
                v.sort();
                v
            }
        }
    }

    pub fn contains(&self, bin: &str) -> bool {
        match &self.kind {
            Kind::Zip(_) => self.index.contains_key(bin),
            Kind::Dir(root) => root.join(format!("{bin}.class")).is_file(),
        }
    }

    pub fn read_class(&mut self, bin: &str) -> Result<Option<Vec<u8>>, Error> {
        match &mut self.kind {
            Kind::Zip(zip) => {
                let Some(&i) = self.index.get(bin) else { return Ok(None) };
                read_entry(zip, i, &self.path).map(Some)
            }
            Kind::Dir(root) => {
                let p = root.join(format!("{bin}.class"));
                if !p.is_file() {
                    return Ok(None);
                }
                std::fs::read(&p).map(Some).map_err(|e| Error::Io(p.display().to_string(), e.to_string()))
            }
        }
    }

    pub fn read_resource(&mut self, path: &str) -> Result<Option<Vec<u8>>, Error> {
        match &mut self.kind {
            Kind::Zip(zip) => {
                let Some(&i) = self.resources.get(path) else { return Ok(None) };
                read_entry(zip, i, &self.path).map(Some)
            }
            Kind::Dir(root) => {
                let p = root.join(path);
                Ok(std::fs::read(p).ok())
            }
        }
    }
}

impl Archive {
    /// 根目录的 `module-info.class` 字节（jmod 为 `classes/module-info.class`）
    pub fn read_module_info(&mut self) -> Result<Option<Vec<u8>>, Error> {
        match &mut self.kind {
            Kind::Zip(zip) => match self.module_info {
                Some(i) => read_entry(zip, i, &self.path).map(Some),
                None => Ok(None),
            },
            Kind::Dir(root) => Ok(std::fs::read(root.join("module-info.class")).ok()),
        }
    }

    /// 类路径服务配置 `META-INF/services/<服务二进制名>`：(服务二进制名, 文件字节)，按名排序
    pub fn service_files(&mut self) -> Vec<(String, Vec<u8>)> {
        const DIR: &str = "META-INF/services/";
        let names: Vec<String> = match &self.kind {
            Kind::Zip(_) => self.resources.keys().filter_map(|k| k.strip_prefix(DIR)).map(String::from).collect(),
            Kind::Dir(root) => std::fs::read_dir(root.join(DIR))
                .into_iter()
                .flatten()
                .flatten()
                .filter(|e| e.path().is_file())
                .filter_map(|e| e.file_name().into_string().ok())
                .collect(),
        };
        let mut out: Vec<(String, Vec<u8>)> = names
            .into_iter()
            .filter(|n| !n.is_empty() && !n.contains('/'))
            .filter_map(|n| {
                let b = self.read_resource(&format!("{DIR}{n}")).ok().flatten()?;
                Some((n, b))
            })
            .collect();
        out.sort();
        out
    }
}

fn read_entry(zip: &mut zip::ZipArchive<BufReader<File>>, i: usize, path: &Path) -> Result<Vec<u8>, Error> {
    let err = |e: String| Error::Io(path.display().to_string(), e);
    let mut f = zip.by_index(i).map_err(|e| err(e.to_string()))?;
    let mut buf = Vec::with_capacity(f.size() as usize);
    f.read_to_end(&mut buf).map_err(|e| err(e.to_string()))?;
    Ok(buf)
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            walk(root, &p, out);
        } else if p.extension().is_some_and(|x| x == "class") {
            if let Ok(rel) = p.strip_prefix(root) {
                let s = rel.to_string_lossy().replace('\\', "/");
                let bin = s.trim_end_matches(".class").to_string();
                if !bin.ends_with("module-info") {
                    out.push(bin);
                }
            }
        }
    }
}

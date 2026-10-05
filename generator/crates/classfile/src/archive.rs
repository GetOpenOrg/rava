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
    /// `release`：多版本 jar（清单 `Multi-Release: true`）中 `META-INF/versions/N/`（N ≤ release，
    /// 取最大 N）的条目覆盖基础条目——对类、资源与 `module-info.class` 同样适用；
    /// `META-INF/versions/` 下的路径不以任何形态进入类索引；非多版本 jar 的版本化条目完全不可见
    pub fn open(path: &Path, release: u32) -> Result<Self, Error> {
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
        let mut zip = zip::ZipArchive::new(BufReader::new(file))
            .map_err(|e| Error::Io(path.display().to_string(), e.to_string()))?;
        let mut index = HashMap::new();
        let mut resources = HashMap::new();
        let mut module_info = None;
        let mut versioned: HashMap<String, (u32, usize)> = HashMap::new();
        for i in 0..zip.len() {
            let Some(name) = zip.name_for_index(i) else { continue };
            let Some(rest) = name.strip_prefix(prefix) else { continue };
            if let Some(v) = rest.strip_prefix(VERSIONS_DIR) {
                // 版本化条目：首段须为 ≥ 9 的整数且 ≤ release，取最大 N；N 超界或格式非法则不可见
                let Some((n, tail)) = v.split_once('/') else { continue };
                let Ok(n) = n.parse::<u32>() else { continue };
                if n < 9 || n > release || tail.is_empty() {
                    continue;
                }
                let e = versioned.entry(tail.to_string()).or_insert((n, i));
                if n > e.0 {
                    *e = (n, i);
                }
                continue;
            }
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
        if !versioned.is_empty() && is_multi_release(&mut zip, &resources, path)? {
            for (rest, (_, i)) in versioned {
                if rest == "module-info.class" {
                    module_info = Some(i);
                } else if let Some(bin) = rest.strip_suffix(".class") {
                    if !bin.ends_with("module-info") {
                        index.insert(bin.to_string(), i);
                    }
                } else if !rest.ends_with('/') {
                    resources.insert(rest, i);
                }
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

const VERSIONS_DIR: &str = "META-INF/versions/";
pub const MANIFEST_PATH: &str = "META-INF/MANIFEST.MF";
const MULTI_RELEASE_ATTR: &str = "Multi-Release";

/// 清单主段的属性值（续行以单个空格起首；主段以首个空行结束）
pub fn manifest_attr(text: &str, key: &str) -> Option<String> {
    let mut lines: Vec<String> = Vec::new();
    for ln in text.lines() {
        let ln = ln.trim_end_matches('\r');
        if ln.is_empty() {
            break;
        }
        match (ln.strip_prefix(' '), lines.last_mut()) {
            (Some(cont), Some(last)) => last.push_str(cont),
            _ => lines.push(ln.to_string()),
        }
    }
    lines.iter().find_map(|l| {
        let (k, v) = l.split_once(':')?;
        (k.trim().eq_ignore_ascii_case(key)).then(|| v.trim().to_string()).filter(|v| !v.is_empty())
    })
}

/// 多版本 jar 判定：清单 `Multi-Release: true`（大小写不敏感）
fn is_multi_release(
    zip: &mut zip::ZipArchive<BufReader<File>>,
    resources: &HashMap<String, usize>,
    path: &Path,
) -> Result<bool, Error> {
    let Some(&i) = resources.get(MANIFEST_PATH) else { return Ok(false) };
    let text = read_entry(zip, i, path)?;
    let text = String::from_utf8_lossy(&text);
    Ok(manifest_attr(&text, MULTI_RELEASE_ATTR).is_some_and(|v| v.eq_ignore_ascii_case("true")))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    /// 在临时目录写一个多版本 jar 夹具；返回其路径
    fn mr_jar(tag: &str, manifest: Option<&str>, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("rava_archive_test_{}_{}", tag, std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let p = dir.join(format!("{tag}.jar"));
        let f = File::create(&p).unwrap();
        let mut w = zip::ZipWriter::new(f);
        let opt: zip::write::SimpleFileOptions = Default::default();
        if let Some(m) = manifest {
            w.start_file(MANIFEST_PATH, opt).unwrap();
            w.write_all(m.as_bytes()).unwrap();
        }
        for (name, data) in entries {
            w.start_file(name.to_string(), opt).unwrap();
            w.write_all(data).unwrap();
        }
        w.finish().unwrap();
        p
    }

    /// 多版本 jar：versions/11 的类与 versions/9 的 module-info 覆盖基础条目；
    /// META-INF/versions 路径不入索引；release 低于版本号时版本化条目不可见
    #[test]
    fn multi_release_view() {
        let mi = crate::module::tests::sample();
        let p = mr_jar(
            "mrv",
            Some("Manifest-Version: 1.0\nMulti-Release: true\n"),
            &[
                ("p/Base.class", &[1u8]),
                ("p/Old.class", &[1u8]),
                ("META-INF/versions/9/module-info.class", &mi),
                ("META-INF/versions/11/p/Base.class", &[2u8]),
                ("META-INF/versions/17/q/New.class", &[3u8]),
                ("META-INF/versions/8/p/Hidden.class", &[4u8]),
                ("META-INF/versions/abc/p/Bad.class", &[5u8]),
            ],
        );
        let mut a = Archive::open(&p, 21).unwrap();
        assert_eq!(a.class_names(), vec!["p/Base".to_string(), "p/Old".to_string(), "q/New".to_string()]);
        assert!(a.class_names().iter().all(|n| !n.starts_with("META-INF")));
        assert_eq!(a.read_class("p/Base").unwrap().unwrap(), vec![2u8]); // 11 覆盖基础条目
        assert!(a.read_resource("p/Old.class").unwrap().is_none()); // 类不走资源表
        assert!(a.read_module_info().unwrap().is_some()); // versions/9 的描述符
        // release 10：11 / 17 的条目不可见，9 的描述符仍生效
        let mut a = Archive::open(&p, 10).unwrap();
        assert_eq!(a.class_names(), vec!["p/Base".to_string(), "p/Old".to_string()]);
        assert_eq!(a.read_class("p/Base").unwrap().unwrap(), vec![1u8]);
        assert!(a.read_module_info().unwrap().is_some());
        // 非 release 视角的关闭面（release 8）：全部版本化条目不可见
        let mut a = Archive::open(&p, 8).unwrap();
        assert_eq!(a.class_names(), vec!["p/Base".to_string(), "p/Old".to_string()]);
        assert!(a.read_module_info().unwrap().is_none());
        std::fs::remove_dir_all(p.parent().unwrap()).ok();
    }

    /// 清单没有 Multi-Release 属性时，版本化条目对任何 release 都不可见（与 JVM 一致）
    #[test]
    fn versioned_entries_need_manifest_flag() {
        let p = mr_jar(
            "mrn",
            Some("Manifest-Version: 1.0\n"),
            &[("p/Base.class", &[1u8]), ("META-INF/versions/11/p/Base.class", &[2u8])],
        );
        let mut a = Archive::open(&p, 21).unwrap();
        assert_eq!(a.read_class("p/Base").unwrap().unwrap(), vec![1u8]);
        std::fs::remove_dir_all(p.parent().unwrap()).ok();
    }

    /// manifest_attr：主段续行拼接、主段外（Name: 段）不取值
    #[test]
    fn manifest_attr_main_section() {
        let mf = "Manifest-Version: 1.0\r\nAutomatic-Module-Name: org.ex\r\n ample.lib\r\n\r\nName: x\r\nAutomatic-Module-Name: no\r\n";
        assert_eq!(manifest_attr(mf, "Automatic-Module-Name").as_deref(), Some("org.example.lib"));
        assert_eq!(manifest_attr("Name: x\n", "Automatic-Module-Name"), None);
        assert_eq!(manifest_attr("Manifest-Version: 1.0\n", "Multi-Release"), None);
    }
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

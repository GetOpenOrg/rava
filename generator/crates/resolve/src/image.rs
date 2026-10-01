//! 镜像独有类与 VM 支持类的类目录：`--image` 的缺省来源。
//!
//! - **镜像独有类**：jlink 插件在链接期写进运行时镜像（`lib/modules`）、jmod 中不存在的类
//!   （BoundMethodHandle 物种类、`SystemModules$*` 等）。JDK 运行期按名加载它们，原生二进制的类宇宙在
//!   生成期静态确定，故视为 JDK 的一部分：`jimage list` 求差集，`jimage extract` 一次提取到缓存，
//!   按模块给出类目录。jimage 缺席或 JDK 无 jmod → 空。
//! - **VM 支持类**：`runtime/java_support/<module>/…java`，以当前 JDK 的 `javac --patch-module` 编入对应包，
//!   按「JDK 路径 + 源码相对路径与内容」指纹缓存编译产物。编译失败告警并跳过该模块。
//!
//! 缓存根 `$XDG_CACHE_HOME`（缺省 `~/.cache`）下 `rava/jimage/<指纹>`、`rava/vmsupport/<指纹>/<模块>`；
//! 指纹为 FNV-1a 64 位（跨工具链版本稳定），与 Python 解析器的 SHA-1 目录互不干扰。

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use classfile::archive::Archive;

/// 镜像独有类目录（按模块名序）+ VM 支持类目录（按路径序）
pub fn image_class_dirs(java_home: &Path, support_root: &Path) -> Vec<PathBuf> {
    let mut dirs = image_only_dirs(java_home);
    dirs.extend(vm_support_dirs(java_home, support_root));
    dirs
}

fn exe(home: &Path, name: &str) -> PathBuf {
    home.join("bin").join(if cfg!(windows) { format!("{name}.exe") } else { name.to_string() })
}

fn cache_base() -> PathBuf {
    match std::env::var_os("XDG_CACHE_HOME").filter(|v| !v.is_empty()) {
        Some(v) => PathBuf::from(v),
        None => PathBuf::from(std::env::var_os("HOME").unwrap_or_default()).join(".cache"),
    }
}

/// FNV-1a 64 位
#[derive(Clone, Copy)]
struct Fnv(u64);

impl Fnv {
    fn new() -> Fnv {
        Fnv(0xcbf2_9ce4_8422_2325)
    }
    fn update(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= u64::from(*b);
            self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
        }
        // 段分隔：避免 ("ab","c") 与 ("a","bc") 同指纹
        self.0 ^= 0xff;
        self.0 = self.0.wrapping_mul(0x0100_0000_01b3);
    }
    fn hex(self) -> String {
        format!("{:016x}", self.0)
    }
}

/// `jimage list` 输出 → binary name → 模块（`Module: m` 分节；跳过 META-INF 与 module-info）
fn parse_jimage_list(text: &str) -> BTreeMap<String, String> {
    let mut out = BTreeMap::new();
    let mut module = "";
    for ln in text.lines() {
        let t = ln.trim();
        if let Some(m) = t.strip_prefix("Module: ") {
            module = m;
        } else if let Some(n) = t.strip_suffix(".class") {
            if !module.is_empty() && !t.starts_with("META-INF/") && !t.starts_with("module-info") {
                out.insert(n.to_string(), module.to_string());
            }
        }
    }
    out
}

/// Java 正则字面量转义（`jimage extract --include regex:`）
fn regex_escape(s: &str) -> String {
    let mut o = String::with_capacity(s.len() + 8);
    for c in s.chars() {
        if "\\.^$|?*+()[]{}".contains(c) {
            o.push('\\');
        }
        o.push(c);
    }
    o
}

fn jmod_class_names(home: &Path) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    let Ok(rd) = std::fs::read_dir(home.join("jmods")) else { return names };
    for e in rd.flatten() {
        let p = e.path();
        if p.extension().is_some_and(|x| x == "jmod") {
            if let Ok(a) = Archive::open(&p) {
                names.extend(a.class_names());
            }
        }
    }
    names
}

fn image_only_dirs(home: &Path) -> Vec<PathBuf> {
    let (tool, image) = (exe(home, "jimage"), home.join("lib").join("modules"));
    if !tool.exists() || !image.exists() {
        return Vec::new();
    }
    let Ok(o) = Command::new(&tool).arg("list").arg(&image).output() else { return Vec::new() };
    let listed = parse_jimage_list(&String::from_utf8_lossy(&o.stdout));
    let in_jmods = jmod_class_names(home);
    if in_jmods.is_empty() {
        // 无 jmod 的 JDK 布局：镜像即唯一来源，不区分「镜像独有」
        return Vec::new();
    }
    let only: Vec<(&String, &String)> = listed.iter().filter(|(n, _)| !in_jmods.contains(*n)).collect();
    if only.is_empty() {
        return Vec::new();
    }
    let Ok(meta) = std::fs::metadata(&image) else { return Vec::new() };
    let mtime_ns = meta.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos());
    let mut h = Fnv::new();
    h.update(format!("{}:{mtime_ns}:{}", home.canonicalize().unwrap_or(home.to_path_buf()).display(), meta.len()).as_bytes());
    let cache = cache_base().join("rava").join("jimage").join(h.hex());
    let missing = only.iter().any(|(n, m)| !cache.join(m).join(format!("{n}.class")).exists());
    if missing {
        let pattern = only.iter().map(|(n, m)| regex_escape(&format!("/{m}/{n}.class"))).collect::<Vec<_>>().join("|");
        let _ = std::fs::create_dir_all(&cache);
        let st = Command::new(&tool)
            .arg("extract")
            .arg("--dir")
            .arg(&cache)
            .arg("--include")
            .arg(format!("regex:{pattern}"))
            .arg(&image)
            .output();
        if let Err(e) = st {
            eprintln!("      警告：jimage extract 失败：{e}");
        }
    }
    let modules: BTreeSet<&String> = only.iter().map(|(_, m)| *m).collect();
    modules.into_iter().map(|m| cache.join(m)).filter(|d| d.is_dir()).collect()
}

/// 目录下全部某扩展名文件（路径序）
fn files_with_ext(dir: &Path, ext: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        for e in std::fs::read_dir(&d).into_iter().flatten().flatten() {
            let p = e.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.extension().is_some_and(|x| x == ext) {
                out.push(p);
            }
        }
    }
    out.sort();
    out
}

fn vm_support_dirs(home: &Path, root: &Path) -> Vec<PathBuf> {
    let javac = exe(home, "javac");
    if !root.is_dir() || !javac.exists() {
        return Vec::new();
    }
    let mut mods: Vec<PathBuf> = std::fs::read_dir(root).into_iter().flatten().flatten().map(|e| e.path()).filter(|p| p.is_dir()).collect();
    mods.sort();
    let mut dirs = Vec::new();
    for mod_dir in mods {
        let sources = files_with_ext(&mod_dir, "java");
        let Some(module) = mod_dir.file_name().and_then(|n| n.to_str()) else { continue };
        if sources.is_empty() {
            continue;
        }
        let mut h = Fnv::new();
        h.update(home.canonicalize().unwrap_or(home.to_path_buf()).to_string_lossy().as_bytes());
        for s in &sources {
            h.update(s.strip_prefix(&mod_dir).unwrap_or(s).to_string_lossy().as_bytes());
            h.update(&std::fs::read(s).unwrap_or_default());
        }
        let out = cache_base().join("rava").join("vmsupport").join(h.hex()).join(module);
        if files_with_ext(&out, "class").is_empty() {
            let _ = std::fs::create_dir_all(&out);
            let r = Command::new(&javac)
                .arg("--patch-module")
                .arg(format!("{module}={}", mod_dir.display()))
                .arg("-nowarn")
                .arg("-d")
                .arg(&out)
                .args(&sources)
                .output();
            match r {
                Ok(o) if o.status.success() => {}
                Ok(o) => {
                    let err = String::from_utf8_lossy(&o.stderr);
                    eprintln!("      警告：VM 支持类编译失败（{module}）：{}", err.trim().chars().take(400).collect::<String>());
                    continue;
                }
                Err(e) => {
                    eprintln!("      警告：VM 支持类编译失败（{module}）：{e}");
                    continue;
                }
            }
        }
        dirs.push(out);
    }
    dirs.sort();
    dirs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_jimage_listing() {
        let text = "jimage: modules\n\nModule: java.base\n    META-INF/x.class\n    java/lang/Object.class\n    java/lang/invoke/BoundMethodHandle$Species_LL.class\n    module-info.class\n\nModule: jdk.foo\n    a/B.class\n    a/res.txt\n";
        let m = parse_jimage_list(text);
        assert_eq!(m.len(), 3);
        assert_eq!(m["java/lang/invoke/BoundMethodHandle$Species_LL"], "java.base");
        assert_eq!(m["a/B"], "jdk.foo");
    }

    #[test]
    fn escapes_java_regex() {
        assert_eq!(regex_escape("/java.base/a/B$C.class"), "/java\\.base/a/B\\$C\\.class");
    }

    #[test]
    fn fingerprint_is_stable_and_segmented() {
        let f = |parts: &[&str]| {
            let mut h = Fnv::new();
            for p in parts {
                h.update(p.as_bytes());
            }
            h.hex()
        };
        assert_eq!(f(&["ab", "c"]), f(&["ab", "c"]));
        assert_ne!(f(&["ab", "c"]), f(&["a", "bc"]));
        assert_eq!(f(&[]), "cbf29ce484222325");
    }

    /// 真 JDK：镜像独有类目录下有类文件，且都不在 jmod 里；VM 支持类目录非空
    #[test]
    fn real_jdk_image_and_support_dirs() {
        let Some(home) = crate::jdk::find_major(21) else { return };
        let support = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../runtime/java_support");
        let dirs = image_class_dirs(&home, &support);
        let jmods = jmod_class_names(&home);
        let image = image_only_dirs(&home);
        for d in &image {
            let classes = files_with_ext(d, "class");
            assert!(!classes.is_empty(), "{}", d.display());
            for c in classes {
                let n = c.strip_prefix(d).unwrap().to_string_lossy().trim_end_matches(".class").replace('\\', "/");
                assert!(!jmods.contains(&n), "{n} 在 jmod 中");
            }
        }
        assert!(dirs.len() > image.len(), "VM 支持类目录缺失：{dirs:?}");
        assert!(dirs.iter().skip(image.len()).all(|d| !files_with_ext(d, "class").is_empty()));
    }
}

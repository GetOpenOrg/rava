//! 镜像独有类与 VM 支持类的类目录：`--image` 的缺省来源。
//!
//! - **镜像独有类**：jlink 插件在链接期写进运行时镜像（`lib/modules`）、jmod 中不存在的类
//!   （BoundMethodHandle 物种类、`SystemModules$*` 等）。JDK 运行期按名加载它们，原生二进制的类宇宙在
//!   生成期静态确定，故视为 JDK 的一部分，按模块给出类目录。
//! - **镜像改写类**：jmod 中有、jlink 插件改写过字节的类。JVM 执行的是镜像里的版本，JDK 语料以它覆盖 jmod
//!   （[`rewritten_dirs`]，由 [`crate::classpath::ClassPath::add_jdk`] 装入，来源角色仍是 JDK）。
//! - 两者都由逐字节比对求得（不按类名），结果按 JDK 指纹缓存；jimage 缺席或 JDK 无 jmod → 空。
//! - **VM 支持类**：`runtime/java_support/<module>/…java`，以当前 JDK 的 `javac --patch-module` 编入对应包，
//!   按「JDK 路径 + 源码相对路径与内容」指纹缓存编译产物。编译失败告警并跳过该模块。
//!
//! 缓存根 `$XDG_CACHE_HOME`（缺省 `~/.cache`）下 `rava/jimage/<指纹>/{only,rewritten}/<模块>`、`rava/vmsupport/<指纹>/<模块>`；
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

/// `<java_home>/jmods/*.jmod`：(模块名, 档案)，按文件名序
fn jmod_archives(home: &Path) -> Vec<(String, Archive)> {
    // jmod 无版本化条目，release 只对 jar 有意义；按该 JDK 自身主版本取视图即可
    let release = crate::jdk::major_of(home).unwrap_or(0);
    let mut paths: Vec<PathBuf> = std::fs::read_dir(home.join("jmods"))
        .into_iter()
        .flatten()
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|x| x == "jmod"))
        .collect();
    paths.sort();
    paths
        .into_iter()
        .filter_map(|p| {
            let module = p.file_stem()?.to_str()?.to_string();
            Archive::open(&p, release).ok().map(|a| (module, a))
        })
        .collect()
}

#[cfg(test)]
fn jmod_class_names(home: &Path) -> BTreeSet<String> {
    jmod_archives(home).iter().flat_map(|(_, a)| a.class_names()).collect()
}

/// 缓存布局版本：布局变化时换指纹，旧缓存不被误读
const LAYOUT: &str = "image-diff-v2";
/// 索引文件：每行 `only <模块>` / `rewritten <模块>`；它存在即缓存完整
const INDEX: &str = "index.txt";

/// 运行时镜像相对 jmod 的差异：(镜像独有类目录, 镜像改写类目录)，均为 (模块名, 目录)，按模块名序。
///
/// 两类都以镜像为准：镜像独有类是 jlink 插件新增的类，镜像改写类是 jlink 插件改写过、与 jmod 字节不同的类
/// （系统模块描述符映射、预生成 LambdaForm 持有类等）。判定逐字节比对，不按类名。
/// 首次对某个 JDK 求差时整体 `jimage extract` 到暂存目录，比对后只保留差异类，按 JDK 路径、镜像 mtime 与大小
/// 取指纹缓存；以后读索引文件即得，不再打开 jmod。jimage 缺席或 JDK 无 jmod → 空。
fn image_diff(home: &Path) -> (Vec<(String, PathBuf)>, Vec<(String, PathBuf)>) {
    let (tool, image) = (exe(home, "jimage"), home.join("lib").join("modules"));
    if !tool.exists() || !image.exists() || !home.join("jmods").is_dir() {
        return (Vec::new(), Vec::new());
    }
    let Ok(meta) = std::fs::metadata(&image) else { return (Vec::new(), Vec::new()) };
    let mtime_ns = meta.modified().ok().and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok()).map_or(0, |d| d.as_nanos());
    let mut h = Fnv::new();
    h.update(LAYOUT.as_bytes());
    h.update(format!("{}:{mtime_ns}:{}", home.canonicalize().unwrap_or(home.to_path_buf()).display(), meta.len()).as_bytes());
    let base = cache_base().join("rava").join("jimage");
    let cache = base.join(h.hex());
    if !cache.join(INDEX).exists() {
        if let Err(e) = build_image_diff(&tool, &image, home, &base, &h.hex()) {
            eprintln!("      警告：运行时镜像求差失败：{e}");
        }
    }
    read_index(&cache)
}

fn read_index(cache: &Path) -> (Vec<(String, PathBuf)>, Vec<(String, PathBuf)>) {
    let (mut only, mut rewritten) = (Vec::new(), Vec::new());
    let text = std::fs::read_to_string(cache.join(INDEX)).unwrap_or_default();
    for ln in text.lines() {
        let Some((kind, module)) = ln.split_once(' ') else { continue };
        let dir = cache.join(kind).join(module);
        if !dir.is_dir() {
            continue;
        }
        match kind {
            "only" => only.push((module.to_string(), dir)),
            "rewritten" => rewritten.push((module.to_string(), dir)),
            _ => {}
        }
    }
    (only, rewritten)
}

/// 在 `<base>/<fp>.tmp.<pid>` 中整体提取、比对、只留差异类并写索引，最后改名为 `<base>/<fp>`
/// （并发构建同一 JDK 时先改名者胜出，其余丢弃自己的结果）
fn build_image_diff(tool: &Path, image: &Path, home: &Path, base: &Path, fp: &str) -> Result<(), String> {
    let work = base.join(format!("{fp}.tmp.{}", std::process::id()));
    let staging = work.join("staging");
    let _ = std::fs::remove_dir_all(&work);
    std::fs::create_dir_all(&staging).map_err(|e| e.to_string())?;
    let o = Command::new(tool).arg("extract").arg("--dir").arg(&staging).arg(image).output().map_err(|e| format!("jimage extract：{e}"))?;
    if !o.status.success() {
        let _ = std::fs::remove_dir_all(&work);
        return Err(format!("jimage extract：{}", String::from_utf8_lossy(&o.stderr).trim()));
    }
    let mut jmods = jmod_archives(home);
    let in_jmods: BTreeSet<String> = jmods.iter().flat_map(|(_, a)| a.class_names()).collect();
    let mut kinds: BTreeSet<(&str, String)> = BTreeSet::new();
    let mut keep = |kind: &'static str, module: &str, name: &str| -> Result<(), String> {
        let from = staging.join(module).join(format!("{name}.class"));
        let to = work.join(kind).join(module).join(format!("{name}.class"));
        std::fs::create_dir_all(to.parent().unwrap_or(&work)).map_err(|e| e.to_string())?;
        std::fs::rename(&from, &to).map_err(|e| format!("{}：{e}", from.display()))?;
        kinds.insert((kind, module.to_string()));
        Ok(())
    };
    // 镜像改写类：jmod 中有、镜像中字节不同
    for (module, a) in &mut jmods {
        for n in a.class_names() {
            let Ok(img) = std::fs::read(staging.join(module.as_str()).join(format!("{n}.class"))) else { continue };
            if a.read_class(&n).ok().flatten().is_some_and(|b| b != img) {
                keep("rewritten", module, &n)?;
            }
        }
    }
    // 镜像独有类：任何 jmod 中都没有
    let o = Command::new(tool).arg("list").arg(image).output().map_err(|e| format!("jimage list：{e}"))?;
    for (n, module) in parse_jimage_list(&String::from_utf8_lossy(&o.stdout)) {
        if !in_jmods.contains(&n) {
            keep("only", &module, &n)?;
        }
    }
    drop(keep);
    let _ = std::fs::remove_dir_all(&staging);
    let index: String = kinds.iter().map(|(k, m)| format!("{k} {m}\n")).collect();
    std::fs::write(work.join(INDEX), index).map_err(|e| e.to_string())?;
    if std::fs::rename(&work, base.join(fp)).is_err() {
        let _ = std::fs::remove_dir_all(&work);
    }
    Ok(())
}

/// 镜像独有类目录（按模块名序）
fn image_only_dirs(home: &Path) -> Vec<PathBuf> {
    image_diff(home).0.into_iter().map(|(_, d)| d).collect()
}

/// 镜像改写类目录：(模块名, 目录)，按模块名序。JDK 语料以它们覆盖 jmod 中的同名类（[`crate::classpath::ClassPath::add_jdk`]）
pub fn rewritten_dirs(home: &Path) -> Vec<(String, PathBuf)> {
    image_diff(home).1
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

    /// 当前工作区的包目录：取运行期 `CARGO_MANIFEST_DIR`（cargo 按本次调用设置）。编译期 `env!` 在全机共享的
    /// CARGO_TARGET_DIR 下可能指向另一工作区——cargo 对路径包按工作区相对路径算 metadata，源码相同时不重编，
    /// 测试二进制里嵌的就是首次编译它的（可能已删除的）worktree
    fn manifest_dir() -> PathBuf {
        std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
    }

    /// 真 JDK：镜像独有类目录下有类文件，且都不在 jmod 里；VM 支持类目录非空
    #[test]
    fn real_jdk_image_and_support_dirs() {
        let Some(home) = crate::jdk::find_major(21) else { return };
        let support = manifest_dir().join("../../../runtime/java_support");
        assert!(support.is_dir(), "VM 支持类源码根不存在：{}", support.display());
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

    /// 真 JDK：镜像改写类与 jmod 字节不同；JDK 语料装入后同名类取镜像字节，来源角色与模块不变
    #[test]
    fn real_jdk_rewritten_classes_override_jmods() {
        let Some(home) = crate::jdk::find_major(21) else { return };
        let rewritten = rewritten_dirs(&home);
        assert!(!rewritten.is_empty(), "JDK 21 镜像应有 jlink 改写类");
        let mut jmods = jmod_archives(&home);
        let mut cp = crate::classpath::ClassPath::new(21);
        cp.add_jdk(&home).unwrap();
        for (module, d) in &rewritten {
            let (_, jmod) = jmods.iter_mut().find(|(m, _)| m == module).unwrap();
            for c in files_with_ext(d, "class") {
                let n = c.strip_prefix(d).unwrap().to_string_lossy().trim_end_matches(".class").replace('\\', "/");
                let img = std::fs::read(&c).unwrap();
                assert_ne!(jmod.read_class(&n).unwrap().as_deref(), Some(img.as_slice()), "{n} 与 jmod 相同");
                assert_eq!(cp.bytes(&n).as_deref(), Some(img.as_slice()), "{n} 未取镜像字节");
                assert_eq!(cp.origin(&n), Some(crate::classpath::Origin::Jdk), "{n}");
                assert_eq!(cp.module_of(&n).as_deref(), Some(module.as_str()), "{n}");
            }
        }
    }
}

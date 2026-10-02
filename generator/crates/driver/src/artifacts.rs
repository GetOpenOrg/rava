//! 单例编译产物清理：按 `<scratch>/build_artifacts.json` 登记的产物路径删除本工作区专属产物。
//!
//! 清单只登记 manifest 位于本 scratch 内的包（`java_runtime` / 实现层 / 用户 bin / lib crate），
//! 跨例共享的依赖（`rava_macros`、syn 等）不在清单内，永不触及。每个登记路径按 cargo 的产物布局
//! 扩展到同一 `<crate>-<hash>` 单元的全部文件：`deps/` 下同哈希兄弟（`.rlib` / `.rmeta` / `.d` /
//! 可执行原件）、`.fingerprint/<包>-<hash>` 指纹目录、`build/<crate>-<hash>/` 构建脚本目录、
//! profile 目录下可执行文件及其 `.d`。
//!
//! 两个时点：`rava compile` 链接成功后删除除可执行文件外的全部产物（中间产物只服务链接）；
//! 运行完成后 `rava prune <scratch>`（或 `rava build` 的 run 段之后）删除剩余的可执行文件。
//! `--keep-artifacts` 关闭两者，供需要复用编译缓存的场景使用。

use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::cargo::ARTIFACTS_FILE;

/// `<crate>-<hash>` 单元的哈希（文件名去 `lib` 前缀、去扩展名后最后一个 `-` 之后）
fn unit_hash(name: &str) -> Option<&str> {
    let name = name.strip_prefix("lib").unwrap_or(name);
    let stem = name.split('.').next().unwrap_or(name);
    stem.rsplit_once('-').map(|(_, h)| h).filter(|h| !h.is_empty())
}

fn unit_stem(name: &str) -> &str {
    let name = name.strip_prefix("lib").unwrap_or(name);
    name.split('.').next().unwrap_or(name)
}

fn name_of(p: &Path) -> &str {
    p.file_name().and_then(|n| n.to_str()).unwrap_or_default()
}

/// 目录下名字满足 `f` 的条目
fn entries(dir: &Path, f: impl Fn(&str) -> bool) -> Vec<PathBuf> {
    let Ok(rd) = std::fs::read_dir(dir) else { return Vec::new() };
    rd.filter_map(Result::ok).map(|e| e.path()).filter(|p| f(name_of(p))).collect()
}

/// profile 目录（`<target>/<profile>`）下 `.fingerprint` 中以 `-<hash>` 结尾的指纹目录
fn fingerprints(profile: &Path, hash: &str) -> Vec<PathBuf> {
    let suffix = format!("-{hash}");
    entries(&profile.join(".fingerprint"), |n| n.ends_with(&suffix))
}

/// 登记路径 → 同一编译单元的全部文件 / 目录
pub fn expand(p: &Path) -> Vec<PathBuf> {
    let mut out = vec![p.to_path_buf()];
    let Some(parent) = p.parent() else { return out };
    let grand = parent.parent();
    let name = name_of(p);
    if name_of(parent) == "deps" {
        // deps/<crate>-<hash>.{rlib,rmeta,d,…}
        let Some(profile) = grand else { return out };
        let stem = unit_stem(name).to_string();
        out.extend(entries(parent, |n| unit_stem(n) == stem));
        if let Some(h) = unit_hash(name) {
            out.extend(fingerprints(profile, h));
        }
    } else if grand.is_some_and(|g| name_of(g) == "build") {
        // build/<crate>-<hash>/build-script-build（构建脚本编译单元）
        out.push(parent.to_path_buf());
        if let (Some(h), Some(profile)) = (unit_hash(name_of(parent)), grand.and_then(Path::parent)) {
            out.extend(fingerprints(profile, h));
        }
    } else if name_of(parent) == "build" {
        // build/<crate>-<hash>/（构建脚本执行输出）
        if let (Some(h), Some(profile)) = (unit_hash(name), grand) {
            out.extend(fingerprints(profile, h));
        }
    } else {
        // <profile>/<bin>（deps 内原件的硬链接）及其 `.d`、macOS 调试符号链接 `.dSYM`
        out.push(p.with_file_name(format!("{name}.d")));
        out.push(p.with_file_name(format!("{name}.dSYM")));
    }
    out
}

fn remove(p: &Path) -> u64 {
    let Ok(meta) = std::fs::symlink_metadata(p) else { return 0 };
    if meta.is_dir() {
        let size = dir_size(p);
        if std::fs::remove_dir_all(p).is_ok() {
            return size;
        }
        0
    } else if std::fs::remove_file(p).is_ok() {
        meta.len()
    } else {
        0
    }
}

fn dir_size(d: &Path) -> u64 {
    let Ok(rd) = std::fs::read_dir(d) else { return 0 };
    rd.filter_map(Result::ok)
        .map(|e| match e.file_type() {
            Ok(t) if t.is_dir() => dir_size(&e.path()),
            _ => e.metadata().map(|m| m.len()).unwrap_or(0),
        })
        .sum()
}

/// 删除 `paths` 的编译单元（`keep` 中的路径及其 `.d` 保留）；返回释放字节数
pub fn prune(paths: &[PathBuf], keep: &[PathBuf]) -> u64 {
    let kept = |p: &Path| keep.iter().any(|k| p == k || p == k.with_file_name(format!("{}.d", name_of(k))));
    let mut freed = 0;
    for p in paths.iter().filter(|p| !keep.contains(p)) {
        for t in expand(p) {
            if !kept(&t) {
                freed += remove(&t);
            }
        }
    }
    freed
}

/// 读产物清单 → (全部登记路径, 可执行文件)
fn read_manifest(out: &Path) -> Result<(Value, Vec<PathBuf>, Option<PathBuf>), String> {
    let f = out.join(ARTIFACTS_FILE);
    let text = std::fs::read_to_string(&f).map_err(|e| format!("{}：{e}", f.display()))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| format!("{}：{e}", f.display()))?;
    let paths = v["paths"].as_array().into_iter().flatten().filter_map(Value::as_str).map(PathBuf::from).collect();
    let exe = v["executable"].as_str().map(PathBuf::from);
    Ok((v, paths, exe))
}

/// 清理本 scratch 的登记产物：`keep_exe` 时保留可执行文件（链接后、运行前），否则全部删除；
/// 清单改写为剩余产物。返回释放字节数
pub fn prune_scratch(out: &Path, keep_exe: bool) -> Result<u64, String> {
    let (mut v, paths, exe) = read_manifest(out)?;
    let keep: Vec<PathBuf> = if keep_exe { exe.into_iter().collect() } else { Vec::new() };
    let freed = prune(&paths, &keep);
    v["paths"] = keep.iter().map(|p| Value::from(p.to_string_lossy().into_owned())).collect();
    let f = out.join(ARTIFACTS_FILE);
    std::fs::write(&f, serde_json::to_string_pretty(&v).unwrap_or_default()).map_err(|e| format!("{}：{e}", f.display()))?;
    Ok(freed)
}

/// `rava prune <scratch>…`：删除各 scratch 登记的剩余编译产物（运行完成后调用）
pub fn run_prune(rest: &[String]) -> Result<(), String> {
    if rest.is_empty() {
        return Err("用法：rava prune <scratch>…".into());
    }
    for s in rest {
        if s.starts_with('-') {
            return Err(format!("未知选项：{s}"));
        }
        let out = Path::new(s);
        if !out.join(ARTIFACTS_FILE).exists() {
            continue;
        }
        let freed = prune_scratch(out, false)?;
        println!("[prune] {}：释放 {:.1} MiB", out.display(), freed as f64 / 1048576.0);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(p: &Path) {
        std::fs::create_dir_all(p.parent().unwrap()).unwrap();
        std::fs::write(p, b"x").unwrap();
    }

    #[test]
    fn prunes_own_units_keeps_shared_and_exe() {
        let t = std::env::temp_dir().join(format!("rava_prune_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&t);
        let prof = t.join("debug");
        let own = [
            "deps/libjava_runtime-aaaa1111.rlib",
            "deps/libjava_runtime-aaaa1111.rmeta",
            "deps/java_runtime-aaaa1111.d",
            "deps/t1-bbbb2222",
            "deps/t1-bbbb2222.d",
            ".fingerprint/java_runtime-aaaa1111/lib",
            ".fingerprint/user-bbbb2222/bin",
            "build/java_runtime-cccc3333/build-script-build",
            ".fingerprint/java_runtime-cccc3333/x",
            "build/java_runtime-dddd4444/out/meta.rs",
            ".fingerprint/java_runtime-dddd4444/y",
            "deps/t1-bbbb2222.dSYM/Contents/Info.plist",
            "t1",
            "t1.d",
        ];
        let shared = ["deps/librava_macros-eeee5555.dylib", ".fingerprint/rava_macros-eeee5555/z", "deps/java_runtime-aaaa11119.d"];
        for f in own.iter().chain(&shared) {
            touch(&prof.join(f));
        }
        let listed: Vec<PathBuf> = [
            "deps/libjava_runtime-aaaa1111.rlib",
            "deps/libjava_runtime-aaaa1111.rmeta",
            "deps/t1-bbbb2222",
            "build/java_runtime-cccc3333/build-script-build",
            "build/java_runtime-dddd4444",
            "t1",
        ]
        .iter()
        .map(|f| prof.join(f))
        .collect();
        // 链接后：保留可执行文件
        prune(&listed, &[prof.join("t1")]);
        assert!(prof.join("t1").exists() && prof.join("t1.d").exists());
        for f in own.iter().filter(|f| !f.starts_with("t1")) {
            assert!(!prof.join(f).exists(), "{f} 应已删除");
        }
        for f in shared {
            assert!(prof.join(f).exists(), "{f} 应保留");
        }
        // 运行后：全部删除
        prune(&listed, &[]);
        assert!(!prof.join("t1").exists() && !prof.join("t1.d").exists());
        let _ = std::fs::remove_dir_all(&t);
    }
}

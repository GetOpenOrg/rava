/// build.rs — 维护 native_status.toml、JDK 特性版本常量与严格模式
///
/// 职责：
///   1. 扫描 src/ 下所有 .rs 文件，提取 #[java_class] / #[java_native] 属性
///   2. 扫描 src/ 下 *_impl.rs 文件，对照已实现的方法（K-4 共置结构）
///   3. 更新 native_status.toml：implemented / needed / stub / not-needed
///   4. 打印 "needed" 状态的 native 方法清单（警告，不阻断构建；strict 模式报错）
///   5. jdk_feature.txt → OUT_DIR/jdk_feature.rs 常量与 `jdk_ge_25` cfg
///
/// 反射元数据表（类层次 / 字段 / 方法 / 注解等）由 java_meta crate 的构建脚本生成：
/// 表覆盖用户类，放在本 crate 会使任何用户改动都重编 java_runtime。

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let src_dir     = Path::new("src");
    let status_file = Path::new("../native_status.toml");

    println!("cargo:rerun-if-changed=src/");
    // 严格模式标记（生成侧按 rava build --strict 写入 strict.txt：1 / 0）
    println!("cargo:rerun-if-changed=strict.txt");
    // 语料 JDK 特性版本（生成侧写入 jdk_feature.txt）→ OUT_DIR/jdk_feature.rs 常量，
    // 手写层经 crate::jdk_feature() 读取（缺省 21）
    println!("cargo:rerun-if-changed=jdk_feature.txt");
    // 编译期 cfg `jdk_ge_25`：手写边界类中**签名**随 JDK 版本变化的成员按此分叉
    //（运行期数据差异用 crate::jdk_feature()；类型差异只能编译期选择）
    println!("cargo::rustc-check-cfg=cfg(jdk_ge_25)");
    let jdk_feature: u32 = fs::read_to_string("jdk_feature.txt").ok()
        .and_then(|v| v.trim().parse().ok()).unwrap_or(21);
    if jdk_feature >= 25 {
        println!("cargo:rustc-cfg=jdk_ge_25");
    }
    let feature_rs = Path::new(&std::env::var("OUT_DIR").unwrap()).join("jdk_feature.rs");
    let feature_src = format!("pub const JDK_FEATURE: u32 = {jdk_feature};\n");
    if fs::read_to_string(&feature_rs).ok().as_deref() != Some(feature_src.as_str()) {
        fs::write(&feature_rs, feature_src).unwrap();
    }

    let native_methods = scan_native_methods(src_dir);
    let status: BTreeMap<String, BTreeMap<String, String>> = load_status(status_file);
    // K-4: _impl.rs 文件与生成 stub 共置于 src/，scan_impls 只处理 *_impl.rs
    let implemented = scan_impls(src_dir);

    let mut new_status: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    for nm in &native_methods {
        let class_key  = nm.class.replace('/', ".");
        let method_key = nm.method.clone();
        if status.get(&class_key)
            .and_then(|c| c.get(&method_key))
            .map(|s| s.as_str()) == Some("not-needed")
        {
            new_status.entry(class_key).or_default()
                .insert(method_key, "not-needed".to_owned());
            continue;
        }
        let impl_key   = format!("{}.{}", nm.class, nm.method);
        let entry_stat = if implemented.contains(&impl_key) { "implemented" } else { "needed" };
        new_status.entry(class_key).or_default().insert(method_key, entry_stat.to_owned());
    }

    write_status(status_file, &new_status);

    let strict = fs::read_to_string("strict.txt").map(|v| v.trim() == "1").unwrap_or(false);
    let needed: Vec<_> = new_status.iter()
        .flat_map(|(cls, methods)| {
            methods.iter()
                .filter(|(_, s)| s.as_str() == "needed")
                .map(move |(m, _)| format!("  → {}.{}", cls, m))
        })
        .collect();

    if !needed.is_empty() {
        if strict {
            for line in &needed {
                let method = line.trim_start_matches("  → ");
                println!("cargo::error=native method not implemented: {}", method);
            }
        } else {
            println!("cargo:warning=");
            println!("cargo:warning=─── native methods needing implementation ───");
            for line in &needed { println!("cargo:warning={}", line); }
            println!("cargo:warning=Add implementations to src/<package>/<class>_impl.rs");
            println!("cargo:warning=");
        }
    }
}

struct NativeMethod { class: String, method: String, #[allow(dead_code)] descriptor: String }

fn scan_native_methods(src_dir: &Path) -> Vec<NativeMethod> {
    let mut result = Vec::new();
    if !src_dir.exists() { return result; }
    for path in walk_rs_files(src_dir) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        parse_native_comments(&content, &mut result);
    }
    result
}

fn parse_native_comments(content: &str, out: &mut Vec<NativeMethod>) {
    let mut current_class = String::new();
    let mut in_class_attr = false;
    let mut class_attr_buf = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        let is_class_attr_start = trimmed.starts_with("#[java_class(")
            || trimmed.starts_with("#[cfg_attr(any(), java_class(");
        if is_class_attr_start {
            in_class_attr = true;
            class_attr_buf = trimmed.to_owned();
        } else if in_class_attr {
            class_attr_buf.push(' ');
            class_attr_buf.push_str(trimmed);
        }
        if in_class_attr && class_attr_buf.contains(")]") {
            if let Some(name) = extract_attr(&class_attr_buf, "binary_name") {
                current_class = name;
            }
            in_class_attr = false;
            class_attr_buf.clear();
        }
        let is_native_attr = trimmed.starts_with("#[java_native(")
            || trimmed.starts_with("#[cfg_attr(any(), java_native(");
        if is_native_attr {
            if let Some(method) = extract_attr(trimmed, "name") {
                let descriptor = extract_attr(trimmed, "descriptor").unwrap_or_default();
                if !current_class.is_empty() {
                    out.push(NativeMethod { class: current_class.clone(), method, descriptor });
                }
            }
        }
    }
}

fn extract_attr(s: &str, key: &str) -> Option<String> {
    let pattern = format!("{} = \"", key);
    let start = s.find(&pattern)? + pattern.len();
    let end = s[start..].find('"')? + start;
    Some(s[start..end].to_owned())
}

fn scan_impls(src_dir: &Path) -> HashSet<String> {
    let mut result = HashSet::new();
    if !src_dir.exists() { return result; }
    for path in walk_rs_files(src_dir) {
        // K-4: 只处理 *_impl.rs 文件（共置的手写 impl 文件）及其私有辅助目录内的文件（归宿主）
        let host = host_of(src_dir, &path);
        let stem = host.file_stem().and_then(|s| s.to_str()).unwrap_or("");
        if !stem.ends_with("_impl") { continue; }
        let content = fs::read_to_string(&path).unwrap_or_default();
        // 去掉 _impl 后缀还原为对应类的路径
        let impl_path = host.with_file_name(format!("{}.rs", &stem[..stem.len()-5]));
        let class = path_to_class(src_dir, &impl_path);
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("pub fn ") || t.starts_with("pub unsafe fn ") {
                if let Some(name) = extract_fn_name(t) {
                    result.insert(format!("{}.{}", class, name));
                    result.insert(format!("{}.{}", class, to_camel(&name)));
                }
            }
            if t.starts_with("/// ") {
                if let Some(rest) = t.strip_prefix("/// ") {
                    if let Some(pos) = rest.find('.') {
                        let m_part = &rest[pos+1..];
                        if let Some(colon) = m_part.find(':') {
                            let mname = m_part[..colon].to_owned();
                            let cls   = rest[..pos].replace('/', ".");
                            result.insert(format!("{}.{}", cls, mname));
                            result.insert(format!("{}.{}", rest[..pos].to_owned(), mname));
                        }
                    }
                }
            }
        }
    }
    result
}

/// 私有辅助目录约定（docs/reference/handwritten-boundary.md §五；与生成器
/// `closure::handwritten::layout` 同口径）：目录 `<stem>/` 旁有宿主 `<stem>.rs`，且宿主是共置手写
/// （`_impl` / `_ext` 结尾）或 crate 根模块文件（`lib.rs` 除外）时，目录子树是宿主的私有模块树。
/// 返回文件所属手写单元的宿主（最外层辅助目录的宿主；不在辅助目录内 → 自身）
fn host_of(src_dir: &Path, path: &Path) -> PathBuf {
    let Ok(rel) = path.strip_prefix(src_dir) else { return path.to_path_buf() };
    let mut cur = src_dir.to_path_buf();
    for c in rel.components() {
        let parent = cur.clone();
        cur.push(c);
        let name = c.as_os_str().to_str().unwrap_or("");
        let host_ok = name.ends_with("_impl") || name.ends_with("_ext") || (parent == src_dir && name != "lib");
        if host_ok && cur.is_dir() && parent.join(format!("{name}.rs")).is_file() {
            return cur.with_extension("rs");
        }
    }
    path.to_path_buf()
}

fn path_to_class(base: &Path, path: &Path) -> String {
    let rel = path.strip_prefix(base).unwrap_or(path);
    rel.with_extension("").to_string_lossy().replace('\\', "/")
}

fn extract_fn_name(line: &str) -> Option<String> {
    let after_fn = line.split("fn ").nth(1)?;
    let name: String = after_fn.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
    if name.is_empty() { None } else { Some(name) }
}

fn to_camel(snake: &str) -> String {
    let mut result = String::new();
    let mut cap_next = false;
    for ch in snake.chars() {
        if ch == '_' { cap_next = true; }
        else if cap_next { result.extend(ch.to_uppercase()); cap_next = false; }
        else { result.push(ch); }
    }
    result
}

fn load_status(path: &Path) -> BTreeMap<String, BTreeMap<String, String>> {
    if !path.exists() { return BTreeMap::new(); }
    parse_toml_status(&fs::read_to_string(path).unwrap_or_default())
}

fn parse_toml_status(s: &str) -> BTreeMap<String, BTreeMap<String, String>> {
    let mut result: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    let mut current = String::new();
    for line in s.lines() {
        let t = line.trim();
        if t.starts_with('[') && t.ends_with(']') && !t.starts_with("[[") {
            current = t[1..t.len()-1].to_owned();
        } else if !current.is_empty() {
            if let Some(eq) = t.find('=') {
                let key = t[..eq].trim().to_owned();
                let val = t[eq+1..].trim().trim_matches('"').to_owned();
                if !key.is_empty() && !val.is_empty() {
                    result.entry(current.clone()).or_default().insert(key, val);
                }
            }
        }
    }
    result
}

fn write_status(path: &Path, status: &BTreeMap<String, BTreeMap<String, String>>) {
    let mut out = String::new();
    out.push_str("# native_status.toml — 由 build.rs 自动维护\n");
    out.push_str("# status: \"implemented\" | \"needed\" | \"stub\" | \"not-needed\"\n\n");
    for (class, methods) in status {
        if methods.is_empty() { continue; }
        out.push_str(&format!("[{}]\n", class));
        for (method, s) in methods { out.push_str(&format!("{} = \"{}\",\n", method, s)); }
        out.push('\n');
    }
    fs::write(path, out).unwrap_or_else(|e| eprintln!("build.rs: cannot write native_status.toml: {}", e));
}

fn walk_rs_files(dir: &Path) -> Vec<PathBuf> {
    let mut result = Vec::new();
    walk_dir(dir, &mut result);
    result
}

fn walk_dir(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else { return };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() { walk_dir(&path, out); }
        else if path.extension().map(|e| e == "rs").unwrap_or(false) { out.push(path); }
    }
}

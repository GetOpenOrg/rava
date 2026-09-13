/// build.rs — 自动扫描生成的 .rs 文件，维护 native_status.toml
///
/// 职责：
///   1. 扫描 src/ 下所有 .rs 文件，提取 #[java_class] / #[java_native] 属性
///   2. 扫描 native_impls/ 目录，对照已实现的方法
///   3. 更新 native_status.toml：implemented / needed / stub / not-needed
///   4. 打印 "needed" 状态的 native 方法清单（警告，不阻断构建）

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let src_dir     = Path::new("src");
    let impls_dir   = Path::new("../native_impls");
    let status_file = Path::new("../native_status.toml");

    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=../native_impls/");
    println!("cargo:rerun-if-env-changed=JAVA_RTA_STRICT");

    let native_methods = scan_native_methods(src_dir);
    let status: BTreeMap<String, BTreeMap<String, String>> = load_status(status_file);
    let implemented = scan_impls(impls_dir);

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

    let strict = std::env::var("JAVA_RTA_STRICT").unwrap_or_default() == "1";
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
            println!("cargo:warning=Add implementations to native_impls/<class>.rs");
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

fn scan_impls(impls_dir: &Path) -> HashSet<String> {
    let mut result = HashSet::new();
    if !impls_dir.exists() { return result; }
    for path in walk_rs_files(impls_dir) {
        let content = fs::read_to_string(&path).unwrap_or_default();
        let class = path_to_class(impls_dir, &path);
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

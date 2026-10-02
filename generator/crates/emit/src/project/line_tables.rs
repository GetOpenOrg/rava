//! Java 栈帧行表（FS-E1 S3）：生成文件「Rust 行 → (类, 方法, 源文件, Java 行)」。
//!
//! 方法体语句行尾带 `// line N` 行标记（`method::lines`）；`java_class!` 宏保留方法体 token 的
//! 原始位置，回溯帧的 `at <文件>:<行>` 即落盘文件的行。落盘前扫描每个生成文件的最终文本
//! （拆层后的声明层 / 实现层各自扫描），得出该文件的行表，汇总写入
//! `<scratch>/closure_input/line_tables.rs`，由 java_meta 以 `__java_meta_LINE_TABLES` 导出，
//! 运行时 `fillInStackTrace` 据帧位置查表（`throwable_impl.rs`）。
//!
//! 行表行 `(rust_line, method, java_line)` 按 Rust 行升序：方法起点（`#[java_method]` 属性行）记
//! `java_line = 0`（方法内首个标记之前），`java_class!` 块结束行记 `method = NO_METHOD`（块外的
//! 反射分派等辅助代码不对应 Java 帧）。

use std::path::Path;

use super::fs::Writer;
use crate::error::Result;

/// 行表文件名（与 closure_tables.rs 同目录）
pub const LINE_TABLES: &str = "line_tables.rs";

/// 块外 / 非 Java 方法的方法下标
const NO_METHOD: u32 = u32::MAX;

const BLOCK_OPENS: [&str; 2] = ["rava_macros::java_class! {", "rava_macros::java_interface! {"];
const MARK: &str = " // line ";

/// 一个生成文件的行表
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileLines {
    /// scratch 根下的相对路径（`/` 分隔）
    pub rel: String,
    /// (类 binary name, 方法名, 源文件)
    pub methods: Vec<(String, String, String)>,
    /// (Rust 行（1 起）, 方法下标, Java 行；0 = 方法内首个标记之前)
    pub rows: Vec<(u32, u32, u32)>,
}

/// 属性行 `#[key = "value"]` 的值（宏头部属性，键与 `=` 之间可有对齐空格）
fn attr_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.trim_start().strip_prefix("#[")?.strip_prefix(key)?;
    let rest = rest.trim_start().strip_prefix('=')?.trim_start().strip_prefix('"')?;
    rest.split_once('"').map(|(v, _)| v)
}

/// `#[java_method(name = "x", ...)]` 的方法名（同一行可有其它属性在前）
fn java_method_name(line: &str) -> Option<&str> {
    let at = line.find("#[java_method(")?;
    let rest = line[at..].strip_prefix("#[java_method(")?.trim_start().strip_prefix("name")?;
    let rest = rest.trim_start().strip_prefix('=')?.trim_start().strip_prefix('"')?;
    rest.split_once('"').map(|(v, _)| v)
}

/// `#[java_method(...)]` 的 `declared_by`（复制进本类的方法体的声明类型：接口 default / 未覆盖的超类虚方法）
fn body_owner(line: &str) -> Option<&str> {
    let at = line.find("#[java_method(")?;
    let rest = &line[at..];
    let k = rest.find(" declared_by = \"")? + " declared_by = \"".len();
    rest[k..].split_once('"').map(|(v, _)| v)
}

/// 行尾 ` // line N` 的 N
fn line_mark(line: &str) -> Option<u32> {
    let (_, n) = line.rsplit_once(MARK)?;
    if n.is_empty() || !n.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    n.parse().ok()
}

/// 扫描一个生成文件的最终文本；无行标记 → None
pub fn scan(rel: &str, text: &str) -> Option<FileLines> {
    let mut out = FileLines { rel: rel.to_string(), ..Default::default() };
    let (mut class, mut source) = (String::new(), String::new());
    let mut in_block = false;
    let mut method: Option<u32> = None;
    let mut marks = 0usize;
    for (i, line) in text.lines().enumerate() {
        let no = i as u32 + 1;
        if !in_block {
            if BLOCK_OPENS.iter().any(|o| line.trim_end().ends_with(o)) {
                in_block = true;
                (class, source, method) = (String::new(), String::new(), None);
            }
            continue;
        }
        if line == "}" {
            // 块以第 0 列的 `}` 结束（块内各项至少缩进 4 列）
            in_block = false;
            if method.is_some() {
                out.rows.push((no, NO_METHOD, 0));
            }
            method = None;
            continue;
        }
        if let Some(v) = attr_value(line, "binary_name") {
            class = v.to_string();
        } else if let Some(v) = attr_value(line, "source") {
            source = v.to_string();
        }
        if let Some(name) = java_method_name(line) {
            // 帧归属方法体的声明类型：复制进本类的接口 default / 超类虚方法体归声明类型（HotSpot 帧的
            // method holder）；源文件在 `write` 汇总时按声明类型校正
            let owner = body_owner(line).unwrap_or(&class);
            let entry = (owner.to_string(), name.to_string(), source.clone());
            let idx = match out.methods.iter().position(|m| *m == entry) {
                Some(p) => p as u32,
                None => {
                    out.methods.push(entry);
                    out.methods.len() as u32 - 1
                }
            };
            method = Some(idx);
            out.rows.push((no, idx, 0));
            continue;
        }
        if let (Some(m), Some(n)) = (method, line_mark(line)) {
            out.rows.push((no, m, n));
            marks += 1;
        }
    }
    (marks > 0).then_some(out)
}

/// 文件内各 `java_class!` / `java_interface!` 块的 (类 binary name, 源文件)
fn class_sources(text: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let mut class: Option<&str> = None;
    for line in text.lines() {
        if let Some(v) = attr_value(line, "binary_name") {
            class = Some(v);
        } else if let (Some(c), Some(v)) = (class, attr_value(line, "source")) {
            out.push((c.to_string(), v.to_string()));
            class = None;
        }
    }
    out
}

/// 行表源文本（java_meta 的 lib.rs 以 `include!` 引入）
pub fn render(tables: &[FileLines]) -> String {
    let mut src = String::from(
        "// 生成：Java 栈帧行表（FS-E1），由 java_meta 的 lib.rs 引入。\n\
         // (scratch 相对路径, [(类, 方法, 源文件)], [(Rust 行, 方法下标, Java 行)])\n\n\
         #[export_name = \"__java_meta_LINE_TABLES\"] pub static LINE_TABLES: \
         &[(&str, &[(&str, &str, &str)], &[(u32, u32, u32)])] = &[\n",
    );
    for t in tables {
        src += &format!("    ({:?}, &[", t.rel);
        for (c, m, s) in &t.methods {
            src += &format!("({c:?}, {m:?}, {s:?}), ");
        }
        src += "], &[";
        for (r, m, j) in &t.rows {
            src += &format!("({r}, {m}, {j}), ");
        }
        src += "]),\n";
    }
    src += "];\n";
    src
}

/// 汇总写入 `<out_dir>/closure_input/line_tables.rs`（`files` 为 (绝对路径, 最终文本)）
pub fn write(w: &mut Writer, out_dir: &Path, files: &[(&Path, &str)]) -> Result<()> {
    let mut tables: Vec<FileLines> = files
        .iter()
        .filter_map(|(path, text)| {
            let rel = path.strip_prefix(out_dir).ok()?.to_string_lossy().replace('\\', "/");
            scan(&rel, text)
        })
        .collect();
    // 复制进他类的方法体（`declared_by`）可能来自另一源文件：源文件取声明类型自己的 `source`
    let sources: std::collections::HashMap<String, String> =
        files.iter().flat_map(|(_, text)| class_sources(text)).collect();
    for t in &mut tables {
        for (c, _, s) in &mut t.methods {
            if let Some(own) = sources.get(c.as_str()) {
                own.clone_into(s);
            }
        }
    }
    tables.sort_by(|a, b| a.rel.cmp(&b.rel));
    w.write(&out_dir.join("closure_input").join(LINE_TABLES), &render(&tables))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_maps_marks_to_methods() {
        // 逐行给出（`\` 续行会吞掉缩进，块结束判定依赖第 0 列的 `}`）
        let text = [
            "use x;",
            "rava_macros::java_class! {",
            "    #[binary_name       = \"p/A\"]",
            "    #[source            = \"A.java\"]",
            "    pub struct A: Object;",
            "    impl A {",
            "        #[java_method(name = \"f\", descriptor = \"()V\")]",
            "        pub fn f() -> Result<()> {",
            "            g()?; // line 7",
            "            h()?;",
            "            k()?; // line 8",
            "        }",
            "        #[rava_moved = \"plain\"] #[java_method(name = \"<init>\", descriptor = \"()V\")]",
            "        pub fn new() -> Result<Self> { Ok(x) } // line 3",
            "    }",
            "}",
            "fn helper() {} // line 99",
        ]
        .join("\n");
        let text = text.as_str();
        let t = scan("user/src/a.rs", text).expect("有标记");
        assert_eq!(
            t.methods,
            vec![
                ("p/A".to_string(), "f".to_string(), "A.java".to_string()),
                ("p/A".to_string(), "<init>".to_string(), "A.java".to_string())
            ]
        );
        assert_eq!(t.rows, vec![(7, 0, 0), (9, 0, 7), (11, 0, 8), (13, 1, 0), (14, 1, 3), (16, NO_METHOD, 0)]);
        assert!(scan("x.rs", "fn main() {}\n").is_none());
        assert!(render(&[t]).contains("(\"user/src/a.rs\", &[(\"p/A\", \"f\", \"A.java\"), "));
    }

    #[test]
    fn injected_default_body_belongs_to_interface() {
        let text = [
            "rava_macros::java_class! {",
            "    #[binary_name       = \"p/C\"]",
            "    #[source            = \"C.java\"]",
            "    impl C {",
            "        #[java_method(name = \"m\", descriptor = \"()V\", access = \"public\", virtual_in = \"C\", declared_by = \"p/I$J\")]",
            "        pub fn m(&self) -> Result<()> {",
            "            g()?; // line 4",
            "        }",
            "    }",
            "}",
        ]
        .join("\n");
        let t = scan("user/src/c.rs", &text).expect("有标记");
        assert_eq!(t.methods, vec![("p/I$J".to_string(), "m".to_string(), "C.java".to_string())]);
    }

    #[test]
    fn class_sources_pairs_blocks() {
        let text = [
            "rava_macros::java_class! {",
            "    #[binary_name       = \"p/A\"]",
            "    #[source            = \"A.java\"]",
            "}",
            "rava_macros::java_class! {",
            "    #[binary_name       = \"p/B\"]",
            "    #[source            = \"B.java\"]",
            "}",
        ]
        .join("\n");
        assert_eq!(class_sources(&text), vec![("p/A".into(), "A.java".into()), ("p/B".into(), "B.java".into())]);
    }
}

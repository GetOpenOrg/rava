//! Java 栈帧行表（FS-E1 S3）：生成文件「Rust 行 → (类, 方法, 源文件, Java 行)」。
//!
//! 方法体语句行尾带 `// line N` 行标记（`method::lines`）；`java_class!` 宏保留方法体 token 的
//! 原始位置，回溯帧的 `at <文件>:<行>` 即落盘文件的行。落盘前扫描每个生成文件的最终文本
//! （拆层后的声明层 / 实现层各自扫描），得出该文件的行表，汇总写入
//! `<scratch>/closure_input/line_tables.rs`，由 java_meta 以 `__java_meta_LINE_TABLES` 导出，
//! 运行时 `fillInStackTrace` 据帧位置查表（`throwable_impl.rs`）。
//!
//! 行表行 `(rust_line, method, java_line)` 按 Rust 行升序：方法起点（`#[java_method]` 属性行）记
//! `java_line = 0`（方法内首个标记之前），`java_class!` 块结束行与手写方法的 `// [meta]` 行记
//! `method = NO_METHOD`（块外的反射分派等辅助代码不对应 Java 帧）。手写方法体在共置伴生文件，
//! 其行表项见 [`handwritten`]（Java 行取哨兵 [`handwritten::LINE_NATIVE`] / [`handwritten::LINE_UNKNOWN`]）。
//!
//! 方法项带描述符（运行时据 (类, 方法名, 描述符) 取 `MethodMeta`）与宿主类（方法体复制进他类时
//! 的所在类，`declared_by` 注入；同所在类时为空）——这是 `vm_stack` 栈帧的唯一来源。

mod handwritten;

use std::path::Path;

use super::fs::Writer;
use crate::error::Result;

/// 行表文件名（与 closure_tables.rs 同目录）
pub const LINE_TABLES: &str = "line_tables.rs";

/// 块外 / 非 Java 方法的方法下标
const NO_METHOD: u32 = u32::MAX;

const BLOCK_OPENS: [&str; 2] = ["rava_macros::java_class! {", "rava_macros::java_interface! {"];
const MARK: &str = " // line ";

/// 行表方法项
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method {
    /// 帧归属类（方法体的声明类型，binary name）
    pub class: String,
    pub name: String,
    pub descriptor: String,
    /// 源文件（`write` 汇总时按归属类校正）
    pub source: String,
    /// 方法体所在类；与归属类相同时为空
    pub host: String,
}

impl Method {
    fn new(class: &str, name: &str, descriptor: &str, source: &str, host: &str) -> Self {
        let host = if host == class { String::new() } else { host.to_string() };
        Method { class: class.into(), name: name.into(), descriptor: descriptor.into(), source: source.into(), host }
    }
}

/// 一个生成文件的行表
#[derive(Debug, Default, PartialEq, Eq)]
pub struct FileLines {
    /// scratch 根下的相对路径（`/` 分隔）
    pub rel: String,
    pub methods: Vec<Method>,
    /// (Rust 行（1 起）, 方法下标, Java 行；0 = 方法内首个标记之前)
    pub rows: Vec<(u32, u32, u32)>,
}

/// 属性行 `#[key = "value"]` 的值（宏头部属性，键与 `=` 之间可有对齐空格）
fn attr_value<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let rest = line.trim_start().strip_prefix("#[")?.strip_prefix(key)?;
    let rest = rest.trim_start().strip_prefix('=')?.trim_start().strip_prefix('"')?;
    rest.split_once('"').map(|(v, _)| v)
}

/// 方法属性 `#[java_method(name = "x", descriptor = "d", ...)]` / `#[java_native(..)]`（同一行可有
/// 其它属性在前）→ (是否 native, 方法名, 描述符)
fn java_method_attr(line: &str) -> Option<(bool, &str, &str)> {
    let (native, rest) = ["#[java_method(", "#[java_native("]
        .iter()
        .enumerate()
        .find_map(|(i, open)| line.find(open).map(|at| (i == 1, &line[at + open.len()..])))?;
    let name = attr_arg(rest, "name")?;
    Some((native, name, attr_arg(rest, "descriptor").unwrap_or("")))
}

/// 属性参数列表中 `key = "value"` 的值（键前为 `(` 起始或 `, `）
fn attr_arg<'a>(args: &'a str, key: &str) -> Option<&'a str> {
    let mut from = 0;
    while let Some(k) = args[from..].find(key).map(|k| k + from) {
        from = k + key.len();
        let boundary = k == 0 || args[..k].ends_with(' ') || args[..k].ends_with(',');
        let value = args[from..].trim_start().strip_prefix('=').map(str::trim_start).and_then(|r| r.strip_prefix('"'));
        if let (true, Some(v)) = (boundary, value) {
            return v.split_once('"').map(|(v, _)| v);
        }
    }
    None
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
        if handwritten::is_meta_comment(line) {
            // 手写方法的元数据注释：体在伴生文件，此处结束上一方法区间
            if method.take().is_some() {
                out.rows.push((no, NO_METHOD, 0));
            }
            continue;
        }
        if let Some((_, name, descriptor)) = java_method_attr(line) {
            // 帧归属方法体的声明类型：复制进本类的接口 default / 超类虚方法体归声明类型（HotSpot 帧的
            // method holder），宿主记本类；源文件在 `write` 汇总时按声明类型校正
            let args = &line[line.find("#[java_").unwrap_or(0)..];
            let owner = attr_arg(args, "declared_by").unwrap_or(&class);
            let entry = Method::new(owner, name, descriptor, &source, &class);
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

/// 方法的 LineNumberTable：(类, 方法名, 描述符) → [(start_pc, 行)]（按 start_pc 升序）
pub type LineNumbers = std::collections::BTreeMap<(String, String, String), Vec<(u16, u16)>>;

/// 行表源文本（java_meta 的 lib.rs 以 `include!` 引入）
pub fn render(tables: &[FileLines], numbers: &LineNumbers) -> String {
    let mut src = String::from(
        "// 生成：Java 栈帧行表（FS-E1），由 java_meta 的 lib.rs 引入。\n\
         // (scratch 相对路径, [(类, 方法, 描述符, 源文件, 宿主类)], [(Rust 行, 方法下标, Java 行)])\n\n\
         #[export_name = \"__java_meta_LINE_TABLES\"] pub static LINE_TABLES: \
         &[(&str, &[(&str, &str, &str, &str, &str)], &[(u32, u32, u32)])] = &[\n",
    );
    for t in tables {
        src += &format!("    ({:?}, &[", t.rel);
        for m in &t.methods {
            src += &format!("({:?}, {:?}, {:?}, {:?}, {:?}), ", m.class, m.name, m.descriptor, m.source, m.host);
        }
        src += "], &[";
        for (r, m, j) in &t.rows {
            src += &format!("({r}, {m}, {j}), ");
        }
        src += "]),\n";
    }
    src += "];\n\n\
        // 帧方法的 LineNumberTable（StackFrameInfo 的 bci → 行号，HotSpot `Method::line_number_from_bci`），按键升序\n\
        #[export_name = \"__java_meta_LINE_NUMBERS\"] pub static LINE_NUMBERS: \
        &[(&str, &str, &str, &[(u16, u16)])] = &[\n";
    for ((c, m, d), lnt) in numbers {
        src += &format!("    ({c:?}, {m:?}, {d:?}, &[");
        for (pc, line) in lnt {
            src += &format!("({pc}, {line}), ");
        }
        src += "]),\n";
    }
    src += "];\n";
    src
}

/// 汇总写入 `<out_dir>/closure_input/line_tables.rs`（`files` 为 (绝对路径, 最终文本)；`lnt` 给出
/// 方法 (类, 名, 描述符) 的 LineNumberTable，行表中出现的每个有 Java 行的方法各写一项）
pub fn write(
    w: &mut Writer,
    out_dir: &Path,
    files: &[(&Path, &str)],
    lnt: &dyn Fn(&str, &str, &str) -> Option<Vec<(u16, u16)>>,
) -> Result<()> {
    let mut tables: Vec<FileLines> = files
        .iter()
        .filter_map(|(path, text)| {
            let rel = path.strip_prefix(out_dir).ok()?.to_string_lossy().replace('\\', "/");
            scan(&rel, text)
        })
        .collect();
    // 手写方法：生成文件登记、体在同目录伴生 `<stem>_impl.rs`（overlay 已落盘）
    for (path, text) in files {
        let hws = handwritten::collect(text);
        let (Some(stem), Some(dir)) = (path.file_stem().and_then(|s| s.to_str()), path.parent()) else { continue };
        if hws.is_empty() {
            continue;
        }
        let companion = dir.join(format!("{stem}_impl.rs"));
        let Ok(rel) = companion.strip_prefix(out_dir).map(|r| r.to_string_lossy().replace('\\', "/")) else { continue };
        if let Some(t) = std::fs::read_to_string(&companion).ok().and_then(|c| handwritten::companion_table(&rel, &c, &hws)) {
            tables.push(t);
        }
    }
    // 复制进他类的方法体（`declared_by`）可能来自另一源文件：源文件取声明类型自己的 `source`
    let sources: std::collections::HashMap<String, String> =
        files.iter().flat_map(|(_, text)| class_sources(text)).collect();
    for t in &mut tables {
        for m in &mut t.methods {
            if let Some(own) = sources.get(m.class.as_str()) {
                own.clone_into(&mut m.source);
            }
        }
    }
    tables.sort_by(|a, b| a.rel.cmp(&b.rel));
    let mut numbers = LineNumbers::new();
    for m in tables.iter().flat_map(|t| &t.methods) {
        let key = (m.class.clone(), m.name.clone(), m.descriptor.clone());
        if numbers.contains_key(&key) {
            continue;
        }
        if let Some(table) = lnt(&m.class, &m.name, &m.descriptor).filter(|t| !t.is_empty()) {
            numbers.insert(key, table);
        }
    }
    w.write(&out_dir.join("closure_input").join(LINE_TABLES), &render(&tables, &numbers))
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
            vec![Method::new("p/A", "f", "()V", "A.java", "p/A"), Method::new("p/A", "<init>", "()V", "A.java", "p/A")]
        );
        assert_eq!(t.rows, vec![(7, 0, 0), (9, 0, 7), (11, 0, 8), (13, 1, 0), (14, 1, 3), (16, NO_METHOD, 0)]);
        assert!(scan("x.rs", "fn main() {}\n").is_none());
        let mut numbers = LineNumbers::new();
        numbers.insert(("p/A".into(), "f".into(), "()V".into()), vec![(0, 7), (4, 8)]);
        let src = render(&[t], &numbers);
        assert!(src.contains("(\"p/A\", \"f\", \"()V\", &[(0, 7), (4, 8), ]),"));
        assert!(src.contains("(\"user/src/a.rs\", &[(\"p/A\", \"f\", \"()V\", \"A.java\", \"\"), "));
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
        assert_eq!(t.methods, vec![Method::new("p/I$J", "m", "()V", "C.java", "p/C")]);
    }

    #[test]
    fn meta_comment_ends_method_region() {
        let text = [
            "rava_macros::java_class! {",
            "    #[binary_name       = \"p/A\"]",
            "    impl A {",
            "        #[java_method(name = \"f\", descriptor = \"()V\")]",
            "        pub fn f() -> Result<()> {",
            "            g()?; // line 7",
            "        }",
            "        // [meta] #[java_native(name = \"n\", descriptor = \"()V\", is_native    = true)]",
            "        // [meta] pub fn n() -> Result<()>;",
            "    }",
            "}",
        ]
        .join("\n");
        let t = scan("a.rs", &text).expect("有标记");
        assert_eq!(t.rows, vec![(4, 0, 0), (6, 0, 7), (8, NO_METHOD, 0)]);
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

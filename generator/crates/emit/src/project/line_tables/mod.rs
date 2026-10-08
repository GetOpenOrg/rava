//! Java 栈帧行表（FS-E1 S3）：生成文件「Rust 行 → (类, 方法, 源文件, Java 行)」。
//!
//! 方法体语句行尾带 `// line N` 行标记（`method::lines`）；`java_class!` 宏保留方法体 token 的
//! 原始位置，DWARF 行号即落盘文件的行。落盘前扫描每个生成文件的最终文本
//! （拆层后的声明层 / 实现层各自扫描），得出该文件的行表，汇总写成旁路文件
//! `<scratch>/closure_input/frame_lines.json`（[`FRAME_LINES_PATH`]），不进二进制：链接器包装 `rava-link`
//! 链接后据它把 DWARF 的（文件, 行, 内联链）预解析为地址 → Java 帧表，嵌入二进制（运行时 `pc_map`，
//! 二进制体积 B2）。二进制内只留帧方法的 LineNumberTable（`LINE_NUMBERS`，bci ↔ 行号）：档案侧进
//! `<scratch>/closure_input/line_tables.rs`（java_meta 导出），用户侧随用户元数据行登记（`project::user_meta`）。
//!
//! 行表行 `(rust_line, method, java_line)` 按 Rust 行升序：方法起点（`#[java_method]` 属性行）记
//! `java_line = 0`（方法内首个标记之前），`java_class!` 块结束行与手写方法的 `// [meta]` 行记
//! `method = NO_METHOD`（块外的反射分派等辅助代码不对应 Java 帧）。手写方法体在共置伴生文件，
//! 其行表项见 [`handwritten`]（Java 行取哨兵 [`handwritten::LINE_NATIVE`] / [`handwritten::LINE_UNKNOWN`]）。
//!
//! 方法项带描述符与帧方法元数据（标志字 + 注解原始属性体）：元数据取帧归属类自身的方法属性行，方法体
//! 复制进他类（`declared_by` 注入）而归属类无行时取宿主类的行——这是 `vm_stack` 栈帧的唯一来源，运行时
//! 不读成员表（成员表按反射事实裁剪，见 `rava_meta_tables::Keep`）。

pub(super) mod handwritten;

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::fs::Writer;
use crate::error::Result;

/// 行表文件名（`closure_input/` 下，与 meta_tables.rs 同目录）
pub const LINE_TABLES: &str = "line_tables.rs";
/// 行表文件（scratch 相对路径）
pub const LINE_TABLES_PATH: &str = "closure_input/line_tables.rs";

/// 旁路行表文件（scratch 相对路径）：`rava-link` 建地址表的输入
pub const FRAME_LINES_PATH: &str = "closure_input/frame_lines.json";

/// 块外 / 非 Java 方法的方法下标
pub const NO_METHOD: u32 = u32::MAX;
pub use handwritten::{LINE_NATIVE, LINE_UNKNOWN};

/// 旁路行表（[`FRAME_LINES_PATH`] 的 JSON 形态）：全部带帧的生成文件与手写伴生文件
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct FrameLines {
    pub files: Vec<FrameFile>,
}

/// 一个文件的行表（[`prune`] 之后）
#[derive(Debug, Serialize, Deserialize)]
pub struct FrameFile {
    /// scratch 根下的相对路径（`/` 分隔）；DWARF 文件路径按 `/` 边界后缀对位
    pub rel: String,
    pub methods: Vec<FrameMethod>,
    /// (Rust 行, 方法下标（[`NO_METHOD`] = 不成帧）, Java 行（[`LINE_NATIVE`] / [`LINE_UNKNOWN`] 为哨兵）)，
    /// 按 Rust 行升序；查表取「Rust 行不大于该行」的最后一行
    pub rows: Vec<(u32, u32, u32)>,
}

/// 帧方法项：运行时 `meta::LineMethod` 的来源
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FrameMethod {
    /// 帧归属类（binary name）
    pub class: String,
    pub name: String,
    pub descriptor: String,
    pub source: String,
    /// Modifier 位集（低 16 位）| static << 16 | native << 17
    pub flags: u32,
    /// RuntimeVisibleAnnotations 原始属性体
    pub annotations: Vec<u8>,
}

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
    /// 帧方法元数据（修饰位 / 注解；`write` 汇总时按归属类 → 宿主类取方法属性行）
    pub frame: Option<rava_meta_tables::FrameMeta>,
}

impl Method {
    fn new(class: &str, name: &str, descriptor: &str, source: &str, host: &str) -> Self {
        let host = if host == class { String::new() } else { host.to_string() };
        Method { class: class.into(), name: name.into(), descriptor: descriptor.into(), source: source.into(), host, frame: None }
    }

    /// 行表方法项的标志字：Modifier 位集（低 16 位）| static << 16 | native << 17（运行时 `meta::FrameMethod`）
    fn flags(&self) -> u32 {
        self.frame.as_ref().map_or(0, |f| (f.modifiers as u32 & 0xFFFF) | (f.is_static as u32) << 16 | (f.is_native as u32) << 17)
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

/// 行表瘦身（二进制体积 B1，`docs/plans/2026-10-04-binary-size.md`）：只留成帧所需的行，查表结果逐行不变。
/// - Java 行 0（方法序言、无行标记的方法——存根与无 LineNumberTable 的方法体）与块外同样不成帧，统一记
///   [`NO_METHOD`]；
/// - 相邻同值（方法下标, Java 行）的行合并为首行，表首的 [`NO_METHOD`] 行删去（首行之前本就不成帧）；
/// - 不再被任何行引用的方法项删去，下标按首次出现重排。行全部删去的文件由调用方丢弃。
pub fn prune(t: &mut FileLines) {
    let mut rows: Vec<(u32, u32, u32)> = Vec::with_capacity(t.rows.len());
    for &(r, m, j) in &t.rows {
        let (m, j) = if m == NO_METHOD || j == 0 { (NO_METHOD, 0) } else { (m, j) };
        match rows.last() {
            Some(&(_, pm, pj)) if pm == m && pj == j => {}
            None if m == NO_METHOD => {}
            _ => rows.push((r, m, j)),
        }
    }
    let mut remap = vec![NO_METHOD; t.methods.len()];
    let mut methods = Vec::new();
    for row in &mut rows {
        if row.1 == NO_METHOD {
            continue;
        }
        let old = row.1 as usize;
        if remap[old] == NO_METHOD {
            remap[old] = methods.len() as u32;
            methods.push(t.methods[old].clone());
        }
        row.1 = remap[old];
    }
    t.rows = rows;
    t.methods = methods;
}

/// 方法项取帧元数据；取不到的方法其行记 [`NO_METHOD`]（与运行期「无元数据不成帧」同义），由 [`prune`] 删去
fn attach_frames(t: &mut FileLines, frames: &rava_meta_tables::FrameIndex) {
    let mut dead = Vec::new();
    for (i, m) in t.methods.iter_mut().enumerate() {
        m.frame = frames.get(&m.class, &m.name, &m.descriptor, &m.host);
        if m.frame.is_none() {
            dead.push(i as u32);
        }
    }
    for r in &mut t.rows {
        if dead.contains(&r.1) {
            r.1 = NO_METHOD;
        }
    }
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

/// 二进制内的行表组源文本（java_meta 的 lib.rs 以 `include!` 引入）：帧方法的 LineNumberTable
pub fn render(numbers: &LineNumbers) -> String {
    let mut g = rava_meta_tables::codec::Group::new("LINE_POOL");
    // LINE_NUMBERS 行：类, 方法名, 描述符, [start_pc 增量, 行号增量（zigzag）]——帧方法的 LineNumberTable
    //（StackFrameInfo 的 bci → 行号，HotSpot `Method::line_number_from_bci`），按键升序
    let (s, p) = g.table("LINE_NUMBERS");
    for ((c, m, d), lnt) in numbers {
        s.str(p, c);
        s.str(p, m);
        s.str(p, d);
        s.len(lnt.len());
        let (mut pc0, mut line0) = (0i32, 0i32);
        for &(pc, line) in lnt {
            s.i32(pc as i32 - pc0);
            s.i32(line as i32 - line0);
            (pc0, line0) = (pc as i32, line as i32);
        }
    }
    format!("// 生成：帧方法的 LineNumberTable（FS-E1；字符串池 + 字节流，字形见发射层 line_tables::render）。\n\n{}", g.render())
}

/// 旁路行表的 JSON 文本
pub fn frame_lines_json(tables: &[FileLines]) -> String {
    let files = tables
        .iter()
        .map(|t| FrameFile {
            rel: t.rel.clone(),
            methods: t
                .methods
                .iter()
                .map(|m| FrameMethod {
                    class: m.class.clone(),
                    name: m.name.clone(),
                    descriptor: m.descriptor.clone(),
                    source: m.source.clone(),
                    flags: m.flags(),
                    annotations: m.frame.as_ref().map_or_else(Vec::new, |f| f.annotations.clone()),
                })
                .collect(),
            rows: t.rows.clone(),
        })
        .collect();
    serde_json::to_string(&FrameLines { files }).expect("行表可序列化")
}

/// 汇总：全部文件的行表写入旁路文件 [`FRAME_LINES_PATH`]；档案侧方法的 LineNumberTable 写入
/// `<out_dir>/closure_input/line_tables.rs`，返回用户侧的同形源文本（由用户元数据行文件收录）。`files` 为 (绝对路径, 最终文本)；`lnt` 给出
/// 方法 (类, 名, 描述符) 的 LineNumberTable，行表中出现的每个有 Java 行的方法各写一项）
pub fn write(
    w: &mut Writer,
    out_dir: &Path,
    files: &[(&Path, &str)],
    lnt: &dyn Fn(&str, &str, &str) -> Option<Vec<(u16, u16)>>,
    root: &[(PathBuf, Vec<handwritten::HwMethod>)],
) -> Result<String> {
    let mut tables: Vec<FileLines> = files
        .iter()
        .filter_map(|(path, text)| {
            let rel = path.strip_prefix(out_dir).ok()?.to_string_lossy().replace('\\', "/");
            scan(&rel, text)
        })
        .collect();
    // 手写方法：生成文件登记、体在同目录伴生 `<stem>_impl.rs` 的手写单元（宿主 + 私有辅助目录内的文件，
    // 逐文件成表；overlay 已落盘）
    for (path, text) in files {
        let hws = handwritten::collect(text);
        let (Some(stem), Some(dir)) = (path.file_stem().and_then(|s| s.to_str()), path.parent()) else { continue };
        if hws.is_empty() {
            continue;
        }
        let companion = dir.join(format!("{stem}_impl.rs"));
        let Some(crate_src) = crate_src_of(out_dir, &companion) else { continue };
        for f in closure::handwritten::layout::unit_files(&crate_src, &companion) {
            let Ok(rel) = f.strip_prefix(out_dir).map(|r| r.to_string_lossy().replace('\\', "/")) else { continue };
            if let Some(t) = std::fs::read_to_string(&f).ok().and_then(|c| handwritten::companion_table(&rel, &c, &hws)) {
                tables.push(t);
            }
        }
    }
    // 手写根类：登记来自其字节码（见 handwritten::root_methods），体在 overlay 落盘的根类手写文件
    for (path, hws) in root {
        let Ok(rel) = path.strip_prefix(out_dir).map(|r| r.to_string_lossy().replace('\\', "/")) else { continue };
        if let Some(t) = std::fs::read_to_string(path).ok().and_then(|c| handwritten::companion_table(&rel, &c, hws)) {
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
    // 帧方法元数据随方法项发射（运行时不读成员表）；取不到元数据的方法不成帧
    let texts: Vec<&str> = files.iter().map(|(_, t)| *t).collect();
    let frames = rava_meta_tables::FrameIndex::new(&texts);
    for t in &mut tables {
        attach_frames(t, &frames);
        prune(t);
    }
    tables.retain(|t| !t.rows.is_empty());
    tables.sort_by(|a, b| a.rel.cmp(&b.rel));
    w.write(&out_dir.join(FRAME_LINES_PATH), &frame_lines_json(&tables))?;
    // LineNumberTable：档案侧（JDK / lib crate 文件）进 java_meta；用户 crate 文件的随用户元数据行登记
    let (user, archive): (Vec<FileLines>, Vec<FileLines>) = tables.into_iter().partition(|t| t.rel.starts_with(USER_PREFIX));
    w.write(&out_dir.join("closure_input").join(LINE_TABLES), &render(&numbers_of(&archive, lnt)))?;
    Ok(render(&numbers_of(&user, lnt)))
}

/// 用户 crate 文件的 scratch 相对路径前缀
const USER_PREFIX: &str = "user/";

/// 行表中出现的每个有 Java 行的方法的 LineNumberTable
fn numbers_of(tables: &[FileLines], lnt: &dyn Fn(&str, &str, &str) -> Option<Vec<(u16, u16)>>) -> LineNumbers {
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
    numbers
}

/// scratch 文件所在 crate 的源码根 `<out_dir>/<crate>/src`
fn crate_src_of(out_dir: &Path, file: &Path) -> Option<PathBuf> {
    let first = file.strip_prefix(out_dir).ok()?.components().next()?;
    Some(out_dir.join(first).join("src"))
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
        let src = render(&numbers);
        // LINE_NUMBERS：3 个池下标 + 长度 + 2 对增量（各 1 字节）
        assert!(src.contains("// [meta-stats] LINE_NUMBERS 8\n"), "{src}");
        assert!(!src.contains("LINE_TABLES"), "{src}");
        let json = frame_lines_json(&[t]);
        let back: FrameLines = serde_json::from_str(&json).expect("回读");
        assert_eq!(back.files[0].rel, "user/src/a.rs");
        assert_eq!(back.files[0].methods[0].name, "f");
        assert_eq!(back.files[0].rows[1], (9, 0, 7));
    }

    #[test]
    fn prune_keeps_frame_rows_only() {
        // 方法 0 无行标记（存根），方法 1 有两段同行标记，方法 2 在其后
        let mut t = FileLines {
            rel: "a.rs".into(),
            methods: vec![
                Method::new("p/A", "s", "()V", "A.java", "p/A"),
                Method::new("p/A", "f", "()V", "A.java", "p/A"),
                Method::new("p/A", "g", "()V", "A.java", "p/A"),
            ],
            rows: vec![(3, 0, 0), (7, 1, 0), (8, 1, 5), (9, 1, 5), (10, 1, 6), (12, 2, 0), (13, 2, 9), (15, NO_METHOD, 0)],
        };
        prune(&mut t);
        assert_eq!(t.methods.iter().map(|m| m.name.as_str()).collect::<Vec<_>>(), vec!["f", "g"]);
        assert_eq!(t.rows, vec![(8, 0, 5), (10, 0, 6), (12, NO_METHOD, 0), (13, 1, 9), (15, NO_METHOD, 0)]);
        let mut stub_only = FileLines {
            rel: "b.rs".into(),
            methods: vec![Method::new("p/B", "s", "()V", "B.java", "p/B")],
            rows: vec![(3, 0, 0), (6, NO_METHOD, 0)],
        };
        prune(&mut stub_only);
        assert!(stub_only.rows.is_empty() && stub_only.methods.is_empty());
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

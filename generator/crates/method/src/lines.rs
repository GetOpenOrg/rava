//! Java 源行标记（FS-E1）：方法体每条来源于字节码的语句 / 控制流头行在行尾带 `// line N`
//! （N 为 LineNumberTable 中该指令所在的 Java 行）。
//!
//! 生成的 Rust 文件经宏保留方法体 token 的原始位置，回溯帧的 `at <文件>:<行>` 据此标记可还原
//! Java 行号；标记同时让读者把 Rust 语句对应回 Java 源码。
//!
//! 流程：渲染时在条目前插入独立的标记行（行级后处理对其透明），方法体收尾时
//! [`attach`] 把标记并入其后第一条内容行的行尾。剥去 ` // line N` 后生成文本与无标记时逐字节一致。

/// 独立标记行前缀
const MARK: &str = "// line ";

/// 标记行文本
pub fn mark(line: u16) -> String {
    format!("{MARK}{line}")
}

/// 是否独立标记行
pub fn is_mark(l: &str) -> bool {
    l.trim_start().strip_prefix(MARK).is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// 渲染期标记器：条目来源 pc 映射的行与上一标记不同时给出新标记
pub struct Marker<'a> {
    table: &'a [(u16, u16)],
    last: Option<u16>,
}

impl<'a> Marker<'a> {
    pub fn new(table: &'a [(u16, u16)]) -> Marker<'a> {
        Marker { table, last: None }
    }

    pub fn before(&mut self, pc: Option<u32>) -> Option<String> {
        let line = classfile::extras::line_at(self.table, pc?)?;
        if self.last == Some(line) {
            return None;
        }
        self.last = Some(line);
        Some(mark(line))
    }
}

/// 标记并入其后第一条非空内容行（多行单元并入首个物理行）的行尾；连续标记取最后一条，
/// 与上一处已并入的行相同则省略，末尾无后继内容的标记丢弃
pub fn attach(lines: Vec<String>) -> Vec<String> {
    let mut out = Vec::with_capacity(lines.len());
    let mut pending: Option<String> = None;
    let mut attached: Option<String> = None;
    for l in lines {
        if is_mark(&l) {
            pending = Some(l.trim_start().to_string());
            continue;
        }
        let Some(m) = pending.take_if(|_| !l.trim().is_empty()) else {
            out.push(l);
            continue;
        };
        if attached.as_ref() == Some(&m) {
            out.push(l);
            continue;
        }
        let merged = match l.split_once('\n') {
            Some((first, rest)) => format!("{first} {m}\n{rest}"),
            None => format!("{l} {m}"),
        };
        attached = Some(m);
        out.push(merged);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_dedups_by_line() {
        let table = [(0u16, 10u16), (5, 11)];
        let mut m = Marker::new(&table);
        assert_eq!(m.before(Some(0)).as_deref(), Some("// line 10"));
        assert_eq!(m.before(Some(3)), None);
        assert_eq!(m.before(None), None);
        assert_eq!(m.before(Some(7)).as_deref(), Some("// line 11"));
    }

    #[test]
    fn attach_merges_into_next_content_line() {
        let v = |xs: &[&str]| xs.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        let out = attach(v(&[
            "// line 3",
            "    let a = 1;",
            "// line 4",
            "// line 5",
            "",
            "    if a {\n        b();",
            "// line 5",
            "    c();",
            "// line 9",
        ]));
        assert_eq!(out, v(&["    let a = 1; // line 3", "", "    if a { // line 5\n        b();", "    c();"]));
        assert!(is_mark("    // line 12") && !is_mark("// line x") && !is_mark("foo(); // line 3"));
    }
}

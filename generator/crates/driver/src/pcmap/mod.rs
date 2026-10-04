//! 地址 → Java 帧表（链接期，二进制体积 B2，`docs/plans/2026-10-04-binary-size.md`）。
//!
//! 输入：第一次链接的产物（含 DWARF 行号与内联链）与发射层旁路行表 `closure_input/frame_lines.json`；
//! 输出：嵌入二进制的表字节（运行时 java_runtime `pc_map` 解码）。对标 Go pclntab / GraalVM CodeInfo：
//! 运行时只按返回地址查表，不读符号表与 DWARF，release 可 strip。
//!
//! 成帧规则与发射层行表一致：(DWARF 文件, 行) 按文件 `/` 边界后缀对位旁路行表，取「Rust 行不大于该行」
//! 的最后一行；方法下标为 NO_METHOD 或 Java 行 0（块外 / 方法序言）不成帧。

pub mod dwarf;
mod object_out;

use std::collections::HashMap;

use emit::project::line_tables::{FrameLines, FrameMethod, LINE_NATIVE, LINE_UNKNOWN, NO_METHOD};
use object::{Object, ObjectSection, ObjectSymbol, SectionKind};
use rava_meta_tables::codec::{Pool, Stream};

pub use object_out::object_file;

/// 表头魔数（运行时 `pc_map::MAGIC`）
pub const MAGIC: &[u8; 8] = b"RAVAPCM1";
/// 地址锚点符号（运行时 `pc_map::__rava_pc_anchor`；Mach-O 符号带前导下划线）
const ANCHOR: &str = "__rava_pc_anchor";
/// 不成帧的区间
const NONE: u32 = u32::MAX;

/// 链接产物的布局要点：锚点地址与代码节 (地址, 大小)。两次链接间须不变（表内地址才有效）
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Layout {
    pub anchor: u64,
    pub text: (u64, u64),
}

/// 产物布局；无锚点符号（非 Java 程序：proc-macro、构建脚本等）→ None
pub fn layout(exe: &object::File<'_>) -> Option<Layout> {
    let anchor = exe.symbols().find(|s| {
        s.name().is_ok_and(|n| n == ANCHOR || n.strip_prefix('_') == Some(ANCHOR))
    })?;
    let text = exe
        .sections()
        .find(|s| s.kind() == SectionKind::Text && matches!(s.name(), Ok(".text" | "__text")))
        .map_or((0, 0), |s| (s.address(), s.size()));
    Some(Layout { anchor: anchor.address(), text })
}

/// 构建结果：表字节与统计行
pub struct Built {
    pub blob: Vec<u8>,
    pub stats: String,
}

/// 由第一次链接的产物与旁路行表构建表
pub fn build(exe: &object::File<'_>, exe_data: &[u8], lines: &FrameLines, anchor: u64) -> Result<Built, String> {
    let mut b = Builder::new(lines);
    let walk = dwarf::walk(exe, exe_data, &mut b)?;
    let ranges = b.ranges();
    let mut pool = Pool::default();
    let mut s = Stream::default();
    s.len(b.methods.len());
    for m in &b.methods {
        s.str(&mut pool, &m.class);
        s.str(&mut pool, &m.name);
        s.str(&mut pool, &m.descriptor);
        s.str(&mut pool, &m.source);
        s.u32(m.flags);
        s.bytes(&mut pool, &m.annotations);
    }
    s.len(b.lists.len());
    for l in &b.lists {
        s.len(l.len());
        for &(method, code) in l {
            s.u32(method);
            s.u32(code);
        }
    }
    s.len(ranges.len());
    let mut prev = 0u64;
    for (i, &(start, list)) in ranges.iter().enumerate() {
        if i == 0 {
            s.i64(start.wrapping_sub(anchor) as i64);
        } else {
            s.u64(start - prev);
        }
        prev = start;
        s.u32(if list == NONE { 0 } else { list + 1 });
    }
    let mut blob = MAGIC.to_vec();
    blob.extend_from_slice(&(pool.bytes().len() as u32).to_le_bytes());
    blob.extend_from_slice(pool.bytes());
    blob.extend_from_slice(&(s.0.len() as u32).to_le_bytes());
    blob.extend_from_slice(&s.0);
    let stats = format!(
        "objects {} (missing {}) functions {} pieces {} unmatched_files {} | methods {} lists {} ranges {} | pool {} stream {} blob {}",
        walk.objects,
        walk.objects_missing,
        walk.functions,
        walk.pieces,
        b.unmatched,
        b.methods.len(),
        b.lists.len(),
        ranges.len(),
        pool.bytes().len(),
        s.0.len(),
        blob.len()
    );
    Ok(Built { blob, stats })
}

/// 行表对位与帧序列去重
struct Builder<'l> {
    lines: &'l FrameLines,
    by_rel: HashMap<&'l str, usize>,
    /// DWARF 文件路径 → 旁路行表文件下标（None = 非生成文件）
    by_path: HashMap<String, Option<usize>>,
    unmatched: usize,
    method_ids: HashMap<&'l FrameMethod, u32>,
    methods: Vec<&'l FrameMethod>,
    list_ids: HashMap<Vec<(u32, u32)>, u32>,
    lists: Vec<Vec<(u32, u32)>>,
    /// (起点, 终点, 帧序列下标)
    pieces: Vec<(u64, u64, u32)>,
    scratch: Vec<(u32, u32)>,
}

impl<'l> Builder<'l> {
    fn new(lines: &'l FrameLines) -> Self {
        Builder {
            lines,
            by_rel: lines.files.iter().enumerate().map(|(i, f)| (f.rel.as_str(), i)).collect(),
            by_path: HashMap::new(),
            unmatched: 0,
            method_ids: HashMap::new(),
            methods: Vec::new(),
            list_ids: HashMap::new(),
            lists: Vec::new(),
            pieces: Vec::new(),
            scratch: Vec::new(),
        }
    }

    /// DWARF 文件路径（绝对、相对编译目录或 `./` 前缀均可）→ 旁路行表文件：依次取各 `/` 边界后缀
    fn file_of(&mut self, path: &str) -> Option<usize> {
        if let Some(&hit) = self.by_path.get(path) {
            return hit;
        }
        let norm = path.replace('\\', "/");
        let mut rest = norm.as_str();
        let hit = loop {
            if let Some(&i) = self.by_rel.get(rest) {
                break Some(i);
            }
            match rest.find('/') {
                Some(at) => rest = &rest[at + 1..],
                None => break None,
            }
        };
        if hit.is_none() {
            self.unmatched += 1;
        }
        self.by_path.insert(path.to_owned(), hit);
        hit
    }

    /// (文件, Rust 行) → (全局方法下标, Java 行码：0 native / 1 无行号 / 行 + 2)
    fn frame(&mut self, path: &str, line: u32) -> Option<(u32, u32)> {
        let file = &self.lines.files[self.file_of(path)?];
        let at = file.rows.partition_point(|r| r.0 <= line).checked_sub(1)?;
        let (_, method, java_line) = file.rows[at];
        if method == NO_METHOD || java_line == 0 {
            return None;
        }
        let code = match java_line {
            LINE_NATIVE => 0,
            LINE_UNKNOWN => 1,
            n => n + 2,
        };
        let m = &file.methods[method as usize];
        let id = match self.method_ids.get(m) {
            Some(&id) => id,
            None => {
                let id = self.methods.len() as u32;
                self.methods.push(m);
                self.method_ids.insert(m, id);
                id
            }
        };
        Some((id, code))
    }

    /// 区间表：按起点排序，补齐段间空隙（不成帧），合并相邻同帧序列
    fn ranges(&mut self) -> Vec<(u64, u32)> {
        self.pieces.sort_unstable();
        let mut out: Vec<(u64, u32)> = Vec::with_capacity(self.pieces.len());
        let mut end = None::<u64>;
        for &(start, stop, list) in &self.pieces {
            if end.is_some_and(|e| e < start) {
                out.push((end.unwrap_or(start), NONE));
            }
            out.push((start, list));
            end = Some(end.map_or(stop, |e| e.max(stop)));
        }
        if let Some(e) = end {
            out.push((e, NONE));
        }
        // 同起点保留最后一项，再合并相邻同帧序列
        out.dedup_by(|later, earlier| {
            if later.0 == earlier.0 {
                earlier.1 = later.1;
                true
            } else {
                false
            }
        });
        out.dedup_by(|later, earlier| later.1 == earlier.1);
        // 首项为空区间时不必记（查表落在首项之前即不成帧）
        while out.first().is_some_and(|r| r.1 == NONE) {
            out.remove(0);
        }
        out
    }
}

impl dwarf::Sink for Builder<'_> {
    fn piece(&mut self, start: u64, len: u64, frames: &[(&str, u32)]) {
        let mut list = std::mem::take(&mut self.scratch);
        list.clear();
        for &(file, line) in frames {
            if let Some(f) = self.frame(file, line) {
                list.push(f);
            }
        }
        let id = if list.is_empty() {
            NONE
        } else if let Some(&id) = self.list_ids.get(&list) {
            id
        } else {
            let id = self.lists.len() as u32;
            self.lists.push(list.clone());
            self.list_ids.insert(list.clone(), id);
            id
        };
        self.scratch = list;
        self.pieces.push((start, start + len, id));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use emit::project::line_tables::FrameFile;

    fn lines() -> FrameLines {
        let m = |name: &str| FrameMethod {
            class: "p/A".into(),
            name: name.into(),
            descriptor: "()V".into(),
            source: "A.java".into(),
            flags: 9,
            annotations: vec![],
        };
        FrameLines {
            files: vec![FrameFile {
                rel: "user/src/p/a.rs".into(),
                methods: vec![m("f"), m("g")],
                rows: vec![(10, 0, 0), (11, 0, 7), (20, NO_METHOD, 0), (30, 1, LINE_UNKNOWN), (40, 1, LINE_NATIVE)],
            }],
        }
    }

    #[test]
    fn frames_follow_rows_and_suffix_match() {
        let l = lines();
        let mut b = Builder::new(&l);
        assert_eq!(b.frame("/x/build/t/user/src/p/a.rs", 10), None, "方法序言不成帧");
        assert_eq!(b.frame("/x/build/t/user/src/p/a.rs", 12), Some((0, 9)));
        assert_eq!(b.frame("./user/src/p/a.rs", 25), None, "块外不成帧");
        assert_eq!(b.frame("user/src/p/a.rs", 31), Some((1, 1)));
        assert_eq!(b.frame("user/src/p/a.rs", 41), Some((1, 0)));
        assert_eq!(b.frame("/rustc/lib/core/src/x.rs", 41), None);
        assert_eq!(b.methods.len(), 2);
    }

    #[test]
    fn ranges_fill_gaps_and_merge() {
        let l = lines();
        let mut b = Builder::new(&l);
        use dwarf::Sink;
        b.piece(0x110, 0x10, &[("user/src/p/a.rs", 12)]);
        b.piece(0x100, 0x10, &[("user/src/p/a.rs", 12)]);
        b.piece(0x120, 0x8, &[("/rustc/core/x.rs", 1)]);
        b.piece(0x140, 0x8, &[("user/src/p/a.rs", 31), ("user/src/p/a.rs", 12)]);
        assert_eq!(b.ranges(), vec![(0x100, 0), (0x120, NONE), (0x140, 1), (0x148, NONE)]);
        assert_eq!(b.lists, vec![vec![(0, 9)], vec![(1, 1), (0, 9)]]);
    }
}

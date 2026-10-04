//! 元数据表编码（二进制体积 B1(c)，`docs/plans/2026-10-04-binary-size.md`）：表不再是 `&str` / 切片字面量
//!（每个 16 B 胖指针 + 每行结构体对齐），而是「字符串池 + 字节流」——
//! - 池：去重后的字符串 / 字节串，各项为「LEB128 长度 + 内容」首尾相接成一个 `b"..."` 字面量；
//! - 表：字节流，整数一律 LEB128（有符号整数先 zigzag），字符串 / 字节串记池下标，列表记长度后接各元素。
//!
//! 同一表组（反射元数据 / 行表 / 闭包派生表）共用一个池。运行时（`java_runtime::meta_codec`）首次查询时把
//! 字节流解码为原元素类型（`&'static str` 指向池内），查询 API 不变。每张表的字形在渲染函数与运行时解码
//! 函数两处一一对应（两侧以表名注释标注字形）。

use std::collections::HashMap;

/// 字符串 / 字节串池
#[derive(Default)]
pub struct Pool {
    bytes: Vec<u8>,
    count: u32,
    index: HashMap<Vec<u8>, u32>,
}

impl Pool {
    /// 字节串的池下标（去重）
    pub fn id(&mut self, b: &[u8]) -> u32 {
        if let Some(&i) = self.index.get(b) {
            return i;
        }
        leb128(&mut self.bytes, b.len() as u64);
        self.bytes.extend_from_slice(b);
        let i = self.count;
        self.count += 1;
        self.index.insert(b.to_vec(), i);
        i
    }

    /// 池的字节数
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// 渲染为导出 static `<name>: &[u8]`
    pub fn render(&self, name: &str) -> String {
        render_bytes(name, &self.bytes)
    }
}

/// 一张表的字节流
#[derive(Default)]
pub struct Stream(pub Vec<u8>);

impl Stream {
    pub fn u32(&mut self, v: u32) {
        leb128(&mut self.0, v as u64);
    }
    pub fn i32(&mut self, v: i32) {
        leb128(&mut self.0, ((v << 1) ^ (v >> 31)) as u32 as u64);
    }
    pub fn i64(&mut self, v: i64) {
        leb128(&mut self.0, ((v << 1) ^ (v >> 63)) as u64);
    }
    pub fn u64(&mut self, v: u64) {
        leb128(&mut self.0, v);
    }
    pub fn bool(&mut self, v: bool) {
        self.0.push(v as u8);
    }
    pub fn len(&mut self, n: usize) {
        self.u32(n as u32);
    }
    pub fn str(&mut self, pool: &mut Pool, s: &str) {
        self.u32(pool.id(s.as_bytes()));
    }
    pub fn bytes(&mut self, pool: &mut Pool, b: &[u8]) {
        self.u32(pool.id(b));
    }
    /// 字符串列表：长度 + 各池下标
    pub fn strs<S: AsRef<str>>(&mut self, pool: &mut Pool, v: &[S]) {
        self.len(v.len());
        for s in v {
            self.str(pool, s.as_ref());
        }
    }
}

/// 一个表组：共用池 + 各表字节流（按发射序），渲染时附 `[meta-stats]` 字节数行
pub struct Group {
    pool_name: String,
    pub pool: Pool,
    tables: Vec<(String, Stream)>,
}

impl Group {
    pub fn new(pool_name: &str) -> Self {
        Group { pool_name: pool_name.to_owned(), pool: Pool::default(), tables: Vec::new() }
    }

    /// 新表：返回其字节流与组池
    pub fn table(&mut self, name: &str) -> (&mut Stream, &mut Pool) {
        self.tables.push((name.to_owned(), Stream::default()));
        let last = self.tables.last_mut().expect("刚压入");
        (&mut last.1, &mut self.pool)
    }

    /// 渲染：`[meta-stats]` 行（各表与池的二进制字节数）+ 池 + 各表
    pub fn render(&self) -> String {
        let mut out = String::new();
        for (name, s) in &self.tables {
            out += &format!("// [meta-stats] {name} {}\n", s.0.len());
        }
        out += &format!("// [meta-stats] {} {}\n", self.pool_name, self.pool.size());
        out += &self.pool.render(&self.pool_name);
        for (name, s) in &self.tables {
            out += &render_bytes(name, &s.0);
        }
        out
    }
}

fn leb128(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

/// 渲染为导出 static `<name>: &[u8] = b"..."`（每 120 源字符折行）
pub fn render_bytes(name: &str, bytes: &[u8]) -> String {
    let mut out = format!("#[export_name = \"__java_meta_{name}\"] pub static {name}: &[u8] = b\"");
    let mut col = 0usize;
    for &b in bytes {
        let before = out.len();
        match b {
            b'"' => out.push_str("\\\""),
            b'\\' => out.push_str("\\\\"),
            // 折行后的行首空格会被续行吞掉：转义
            b' ' if col == 0 => out.push_str("\\x20"),
            0x20..=0x7e => out.push(b as char),
            _ => out.push_str(&format!("\\x{b:02x}")),
        }
        col += out.len() - before;
        if col >= 120 {
            // 字节串字面量内 `\` + 换行：跳过换行与下一行行首空白，不计入内容
            out.push_str("\\\n");
            col = 0;
        }
    }
    out.push_str("\";\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn leb128_and_zigzag() {
        let mut s = Stream::default();
        s.u32(0);
        s.u32(127);
        s.u32(128);
        s.i32(-1);
        s.i32(1);
        s.i64(i64::MIN);
        assert_eq!(&s.0[..6], &[0, 127, 0x80, 1, 1, 2]);
        assert_eq!(s.0.len(), 6 + 10);
    }

    #[test]
    fn pool_dedups_and_prefixes_length() {
        let mut p = Pool::default();
        assert_eq!(p.id(b"ab"), 0);
        assert_eq!(p.id(b""), 1);
        assert_eq!(p.id(b"ab"), 0);
        assert_eq!(p.bytes, vec![2, b'a', b'b', 0]);
        let src = render_bytes("X", &[b'"', 0, b'a']);
        assert!(src.contains("b\"\\\"\\x00a\";"), "{src}");
    }
}

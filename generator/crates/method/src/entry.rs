//! 扁平 entries（← Python `(indent, RsStmt | str)` 列表）：结构树发射的产物，变量提升与
//! 渲染的输入。
//!
//! Python 的 `BlockStmt` 结构块只是「结构行 + 子序列」的分组，全部消费方都先 `flatten`；
//! 这里直接产出扁平序列，结构行以 [`Item::Struct`] 携带嵌套深度增量与结构角色
//! （← `rs_ir.StructLine` 的 `delta` / `tag`）。

use ir::Stmt;

/// 结构行的结构角色（← `StructLine.tag`）
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tag {
    /// 其余结构行（`''`）
    Plain,
    /// loop 关键字头（`loop {` / `'lN: loop {` / 状态机 loop；while 头不在内）
    Loop,
    /// `} else {` / `} else if … {`
    Else,
    /// match 臂头与状态机臂头 `X => {`
    Arm,
    /// `java_try!` 内层 `try {`
    Try,
    /// `} catch … {`
    Catch,
}

/// 条目内容
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// IR 语句（渲染时前缀条目缩进）
    Stmt(Box<Stmt>),
    /// 已缩进的文本行（break / continue、状态机前导等）
    Line(String),
    /// 已缩进的块结构行
    Struct { text: String, delta: i32, tag: Tag },
    /// 已删除（← `_REMOVED_ENTRY`，由 `drop_removed` 统一清理）
    Removed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Entry {
    /// 语句条目的缩进；文本行 / 结构行为空（缩进写在文本里）
    pub indent: String,
    pub item: Item,
}

impl Entry {
    pub fn stmt(indent: &str, s: Stmt) -> Entry {
        Entry { indent: indent.to_string(), item: Item::Stmt(Box::new(s)) }
    }

    pub fn line(text: String) -> Entry {
        Entry { indent: String::new(), item: Item::Line(text) }
    }

    pub fn structure(text: String, delta: i32, tag: Tag) -> Entry {
        Entry { indent: String::new(), item: Item::Struct { text, delta, tag } }
    }

    /// 文本条目（Python `isinstance(item, str)`；已删除条目的 `None` 不在内）
    pub fn is_text(&self) -> bool {
        matches!(self.item, Item::Line(_) | Item::Struct { .. })
    }

    /// 对块嵌套深度的净贡献（← `_entry_delta`）
    pub fn delta(&self) -> i32 {
        match self.item {
            Item::Struct { delta, .. } => delta,
            _ => 0,
        }
    }

    pub fn tag(&self) -> Option<Tag> {
        match self.item {
            Item::Struct { tag, .. } => Some(tag),
            _ => None,
        }
    }

    /// loop 关键字头（← `_is_loop_head`）
    pub fn is_loop_head(&self) -> bool {
        self.tag() == Some(Tag::Loop)
    }

    /// 提升插入点候选：开块行与 else / catch 衔接行（← `_is_block_open`）
    pub fn is_block_open(&self) -> bool {
        match self.item {
            Item::Struct { delta, tag, .. } => delta == 1 || matches!(tag, Tag::Else | Tag::Catch),
            _ => false,
        }
    }

    /// `} else {` / `} else if … {`（← `_is_else_line`）
    pub fn is_else_line(&self) -> bool {
        self.tag() == Some(Tag::Else)
    }

    /// match / 状态机臂头，或 java_try! 内层 `try {`（← `_is_arm_or_try`）
    pub fn is_arm_or_try(&self) -> bool {
        matches!(self.tag(), Some(Tag::Arm | Tag::Try))
    }

    pub fn as_stmt(&self) -> Option<&Stmt> {
        match &self.item {
            Item::Stmt(s) => Some(s),
            _ => None,
        }
    }

    pub fn text(&self) -> Option<&str> {
        match &self.item {
            Item::Line(t) | Item::Struct { text: t, .. } => Some(t),
            _ => None,
        }
    }
}

/// 清理已删除条目（← `_drop_removed`）
pub fn drop_removed(entries: &mut Vec<Entry>) {
    entries.retain(|e| e.item != Item::Removed);
}

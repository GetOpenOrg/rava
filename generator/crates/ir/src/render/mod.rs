//! IR → Rust 源码文本的唯一出口（← `codegen/render.py`）。
//!
//! 需要类名短名的节点（`NewPending` / `StaticField` / catch 子句类型）经注入的
//! [`ShortNames`] 查询；类型渲染不依赖短名，另有自由函数 [`render_type`]。

mod atomic;
mod cast;
mod expr;
mod item;
mod lit;
mod stmt;
mod ty;

#[cfg(test)]
mod tests_expr;
#[cfg(test)]
mod tests_stmt;

pub use lit::escape_str;
pub use ty::render_type;

use crate::{Expr, Item, ShortNames, Stmt, Type};

/// 一级缩进（四空格）。
pub const INDENT: &str = "    ";

/// 渲染器：持有短名查询，无其他状态。
#[derive(Clone, Copy)]
pub struct Renderer<'a> {
    names: &'a dyn ShortNames,
}

impl<'a> Renderer<'a> {
    pub fn new(names: &'a dyn ShortNames) -> Renderer<'a> {
        Renderer { names }
    }

    pub fn ty(&self, ty: &Type) -> String {
        render_type(ty)
    }

    pub fn expr(&self, e: &Expr) -> String {
        let mut out = String::new();
        self.write_expr(&mut out, e);
        out
    }

    /// 语句渲染：首行前加 `indent` 级缩进，复合语句的内部行按嵌套深度缩进。
    pub fn stmt(&self, s: &Stmt, indent: usize) -> String {
        let mut out = String::new();
        self.write_stmt(&mut out, s, indent);
        out
    }

    pub fn item(&self, it: &Item, indent: usize) -> String {
        let mut out = String::new();
        self.write_item(&mut out, it, indent);
        out
    }

    /// 一组条目 → 完整 `.rs` 文件内容：前言（去尾部空白）与各条目以空行分隔，末尾换行。
    pub fn file(&self, items: &[Item], preamble: &str) -> String {
        let mut parts: Vec<String> = Vec::with_capacity(items.len() + 1);
        if !preamble.is_empty() {
            parts.push(preamble.trim_end().to_string());
        }
        parts.extend(items.iter().map(|it| self.item(it, 0)));
        let mut out = parts.join("\n\n");
        out.push('\n');
        out
    }

    fn short(&self, binary: &str) -> String {
        self.names.short_cls(binary)
    }
}

pub(crate) fn pad(out: &mut String, indent: usize) {
    for _ in 0..indent {
        out.push_str(INDENT);
    }
}

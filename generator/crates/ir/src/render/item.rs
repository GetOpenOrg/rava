//! 顶层条目渲染（← `render_fn` / `render_struct` / `render_impl` / `render_item`）。

use super::ty::{write_path, write_type, PathCtx};
use super::{pad, Renderer, INDENT};
use crate::{FnItem, ImplItem, Item, StructItem, UseTree};

impl Renderer<'_> {
    pub(crate) fn write_item(&self, out: &mut String, it: &Item, indent: usize) {
        match it {
            Item::Fn(f) => self.write_fn(out, f, indent),
            Item::Struct(s) => write_struct(out, s, indent),
            Item::Impl(i) => self.write_impl(out, i, indent),
            Item::Use(u) => {
                pad(out, indent);
                out.push_str("use ");
                write_use_tree(out, u);
                out.push(';');
            }
            Item::Mod(m) => {
                pad(out, indent);
                out.push_str(m.vis.prefix());
                out.push_str("mod ");
                out.push_str(m.name.as_str());
                out.push(';');
            }
            Item::TypeAlias(t) => {
                pad(out, indent);
                out.push_str(t.vis.prefix());
                out.push_str("type ");
                out.push_str(t.name.as_str());
                out.push_str(" = ");
                write_type(out, &t.ty);
                out.push(';');
            }
            // Python 原样输出（不加缩进）
            Item::Raw(r) => out.push_str(&r.0),
        }
    }

    fn write_fn(&self, out: &mut String, f: &FnItem, indent: usize) {
        pad(out, indent);
        out.push_str(f.vis.prefix());
        if f.is_unsafe {
            out.push_str("unsafe ");
        }
        out.push_str("fn ");
        out.push_str(f.name.as_str());
        out.push('(');
        for (i, p) in f.params.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            out.push_str(p.name.as_str());
            out.push_str(": ");
            write_type(out, &p.ty);
        }
        out.push(')');
        if let Some(r) = &f.ret {
            out.push_str(" -> ");
            write_type(out, r);
        }
        if f.body.is_empty() {
            out.push_str(" {}");
            return;
        }
        out.push_str(" {");
        for s in &f.body {
            out.push('\n');
            self.write_stmt(out, s, indent + 1);
        }
        out.push('\n');
        pad(out, indent);
        out.push('}');
    }

    fn write_impl(&self, out: &mut String, i: &ImplItem, indent: usize) {
        pad(out, indent);
        out.push_str("impl ");
        if let Some(t) = &i.trait_ {
            write_path(out, t, PathCtx::Type);
            out.push_str(" for ");
        }
        write_type(out, &i.self_ty);
        if i.items.is_empty() {
            out.push_str(" {}");
            return;
        }
        out.push_str(" {\n");
        for (k, f) in i.items.iter().enumerate() {
            if k > 0 {
                out.push_str("\n\n");
            }
            self.write_fn(out, f, indent + 1);
        }
        out.push('\n');
        pad(out, indent);
        out.push('}');
    }
}

fn write_struct(out: &mut String, s: &StructItem, indent: usize) {
    if !s.derives.is_empty() {
        pad(out, indent);
        out.push_str("#[derive(");
        for (i, d) in s.derives.iter().enumerate() {
            if i > 0 {
                out.push_str(", ");
            }
            write_path(out, d, PathCtx::Type);
        }
        out.push_str(")]\n");
    }
    pad(out, indent);
    out.push_str(s.vis.prefix());
    out.push_str("struct ");
    out.push_str(s.name.as_str());
    if s.fields.is_empty() {
        out.push(';');
        return;
    }
    out.push_str(" {\n");
    for f in &s.fields {
        pad(out, indent);
        out.push_str(INDENT);
        out.push_str(f.vis.prefix());
        out.push_str(f.name.as_str());
        out.push_str(": ");
        write_type(out, &f.ty);
        out.push_str(",\n");
    }
    pad(out, indent);
    out.push('}');
}

fn write_use_tree(out: &mut String, u: &UseTree) {
    match u {
        UseTree::Path { prefix, tail } => {
            write_path(out, prefix, PathCtx::Type);
            out.push_str("::");
            write_use_tree(out, tail);
        }
        UseTree::Name(n) => out.push_str(n.as_str()),
        UseTree::Rename { name, alias } => {
            out.push_str(name.as_str());
            out.push_str(" as ");
            out.push_str(alias.as_str());
        }
        UseTree::Glob => out.push('*'),
        UseTree::Group(items) => {
            out.push('{');
            for (i, t) in items.iter().enumerate() {
                if i > 0 {
                    out.push_str(", ");
                }
                write_use_tree(out, t);
            }
            out.push('}');
        }
    }
}

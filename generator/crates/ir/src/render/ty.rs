//! 类型与路径渲染（← `render_type`）。类型渲染不需要短名表，是自由函数。

use crate::{Path, PathSegment, Type};

/// 路径所在位置：类型位置泛型为 `A<T>`，表达式位置为 turbofish `A::<T>`。
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum PathCtx {
    Type,
    Expr,
}

/// 类型 → Rust 源码文本。
pub fn render_type(ty: &Type) -> String {
    let mut out = String::new();
    write_type(&mut out, ty);
    out
}

pub(crate) fn write_type(out: &mut String, ty: &Type) {
    match ty {
        Type::Prim(p) => out.push_str(p.as_str()),
        Type::Path(p) => write_path(out, p, PathCtx::Type),
        Type::Ref { inner, mutable, lifetime } => {
            out.push('&');
            if let Some(lt) = lifetime {
                out.push('\'');
                out.push_str(lt.as_str());
                out.push(' ');
            }
            if *mutable {
                out.push_str("mut ");
            }
            write_type(out, inner);
        }
        Type::Slice(elem) => {
            out.push_str("&[");
            write_type(out, elem);
            out.push(']');
        }
        Type::Tuple(elems) => {
            out.push('(');
            write_types(out, elems);
            out.push(')');
        }
        Type::Infer => out.push('_'),
    }
}

/// 逗号分隔的类型列表（`A, B`）。
pub(crate) fn write_types(out: &mut String, tys: &[Type]) {
    for (i, t) in tys.iter().enumerate() {
        if i > 0 {
            out.push_str(", ");
        }
        write_type(out, t);
    }
}

pub(crate) fn write_path(out: &mut String, path: &Path, ctx: PathCtx) {
    if path.global {
        out.push_str("::");
    }
    write_segments(out, &path.segments, ctx);
}

pub(crate) fn write_segments(out: &mut String, segs: &[PathSegment], ctx: PathCtx) {
    for (i, seg) in segs.iter().enumerate() {
        if i > 0 {
            out.push_str("::");
        }
        out.push_str(seg.ident.as_str());
        write_generics(out, &seg.generics, ctx);
    }
}

/// 泛型实参：空则不输出；表达式位置为 turbofish。
pub(crate) fn write_generics(out: &mut String, generics: &[Type], ctx: PathCtx) {
    if generics.is_empty() {
        return;
    }
    if ctx == PathCtx::Expr {
        out.push_str("::");
    }
    out.push('<');
    write_types(out, generics);
    out.push('>');
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Ident, Prim};

    fn id(s: &str) -> Ident {
        Ident::new(s).unwrap()
    }

    #[test]
    fn primitives_and_unit() {
        assert_eq!(render_type(&Type::I32), "i32");
        assert_eq!(render_type(&Type::UNIT), "()");
        assert_eq!(render_type(&Type::Tuple(vec![])), "()");
        assert_eq!(render_type(&Type::Prim(Prim::U16)), "u16");
        assert_eq!(render_type(&Type::Infer), "_");
    }

    #[test]
    fn named_generic_and_path() {
        let node = Type::named(id("HashMap_Node"), vec![Type::named(id("K"), vec![]), Type::named(id("V"), vec![])]);
        assert_eq!(render_type(&node), "HashMap_Node<K, V>");
        let arr = Type::named(id("JArray"), vec![node.clone()]);
        assert_eq!(render_type(&arr), "JArray<HashMap_Node<K, V>>");
        assert_eq!(arr.array_elem(), Some(&node));
        let p = Type::Path(Path::from_idents([id("java_runtime"), id("java"), id("lang"), id("String")]));
        assert_eq!(render_type(&p), "java_runtime::java::lang::String");
    }

    #[test]
    fn refs_slices_tuples() {
        let r = Type::Ref { inner: Box::new(Type::I32), mutable: true, lifetime: Some(id("a")) };
        assert_eq!(render_type(&r), "&'a mut i32");
        let r2 = Type::Ref { inner: Box::new(Type::BOOL), mutable: false, lifetime: None };
        assert_eq!(render_type(&r2), "&bool");
        assert_eq!(render_type(&Type::Slice(Box::new(Type::I64))), "&[i64]");
        assert_eq!(render_type(&Type::Tuple(vec![Type::I32, Type::BOOL])), "(i32, bool)");
    }
}

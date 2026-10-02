//! 类型文本 ↔ 类型对象的桥（← Python `vars._str_to_rs_type` / `jvm_type.from_rust_type`）。
//!
//! Python 在方法体层以文本承载类型（`RsNamed(text)`）；这里：
//! - [`parse_ir_type`]：渲染文本 → [`ir::Type`]（`LetStmt` 的类型标注；与 `ir` 渲染互逆）；
//! - [`from_rust_text`]：渲染文本 → [`RsType`]（层次查询用；`from_rust_type` 同口径）；
//! - [`forms_alignable`]：后到形态能否对齐到提升声明类型（`vars._forms_alignable`）。

use instr::InstrEnv;
use ir::anchors::OBJECT;
use ir::{Ident, Path, PathSegment, Type};
use ty::{Prim, RsType};

use crate::error::{MethodError, MethodResult};

/// 类型文本 → `ir::Type`（语法：基本类型 / `()` / `_` / `&[T]` / `&'a mut T` / 元组 / 路径 + 泛型）
pub fn parse_ir_type(s: &str) -> Option<Type> {
    let mut p = Parser { s: s.as_bytes(), pos: 0 };
    let t = p.ty()?;
    p.skip_ws();
    (p.pos == p.s.len()).then_some(t)
}

/// 同 [`parse_ir_type`]，解析失败报 IR 构造错误
pub fn ir_type_of(s: &str) -> MethodResult<Type> {
    parse_ir_type(s).ok_or_else(|| MethodError::Ir(format!("类型文本无法解析为 ir::Type：{s}")))
}

struct Parser<'s> {
    s: &'s [u8],
    pos: usize,
}

impl Parser<'_> {
    fn skip_ws(&mut self) {
        while self.s.get(self.pos) == Some(&b' ') {
            self.pos += 1;
        }
    }

    fn eat(&mut self, lit: &str) -> bool {
        self.skip_ws();
        if self.s[self.pos..].starts_with(lit.as_bytes()) {
            self.pos += lit.len();
            true
        } else {
            false
        }
    }

    fn word(&mut self) -> Option<String> {
        self.skip_ws();
        let start = self.pos;
        while self.s.get(self.pos).is_some_and(|c| c.is_ascii_alphanumeric() || *c == b'_') {
            self.pos += 1;
        }
        (self.pos > start).then(|| String::from_utf8_lossy(&self.s[start..self.pos]).into_owned())
    }

    fn list(&mut self, close: &str) -> Option<Vec<Type>> {
        let mut out = Vec::new();
        if self.eat(close) {
            return Some(out);
        }
        loop {
            out.push(self.ty()?);
            if self.eat(close) {
                return Some(out);
            }
            if !self.eat(",") {
                return None;
            }
        }
    }

    fn ty(&mut self) -> Option<Type> {
        if self.eat("&[") {
            let e = self.ty()?;
            return self.eat("]").then(|| Type::Slice(Box::new(e)));
        }
        if self.eat("&") {
            let lifetime = if self.eat("'") { Some(Ident::new(self.word()?).ok()?) } else { None };
            let mutable = self.eat("mut ");
            let inner = self.ty()?;
            return Some(Type::Ref { inner: Box::new(inner), mutable, lifetime });
        }
        if self.eat("(") {
            let elems = self.list(")")?;
            return Some(if elems.is_empty() { Type::UNIT } else { Type::Tuple(elems) });
        }
        let global = self.eat("::");
        let mut segs = Vec::new();
        loop {
            let w = self.word()?;
            if segs.is_empty() && !global {
                if w == "_" {
                    return Some(Type::Infer);
                }
                if let Some(p) = ir::Prim::from_name(&w) {
                    return Some(Type::Prim(p));
                }
            }
            let ident = Ident::new(w).ok()?;
            let generics = if self.eat("<") { self.list(">")? } else { Vec::new() };
            segs.push(PathSegment::with_generics(ident, generics));
            if !self.eat("::") {
                break;
            }
        }
        Some(Type::Path(Path { global, segments: segs }))
    }
}

// ── 文本 → RsType（`from_rust_type`）──────────────────────────────────────

/// 顶层逗号切分（尖括号 / 圆括号内的逗号不切）
pub fn split_top_args(s: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut depth = 0i32;
    let mut cur = String::new();
    for c in s.chars() {
        match c {
            '<' | '(' | '[' => depth += 1,
            '>' | ')' | ']' => depth -= 1,
            ',' if depth == 0 => {
                out.push(cur.trim().to_string());
                cur.clear();
                continue;
            }
            _ => {}
        }
        cur.push(c);
    }
    if !cur.trim().is_empty() {
        out.push(cur.trim().to_string());
    }
    out
}

/// 值形态：JVM 基本类型 / 宿主基本类型（`u8` / `u32` / `u64` / `usize`）不可与引用对齐
pub fn is_value_form(s: &str) -> bool {
    matches!(s, "i8" | "i16" | "i32" | "i64" | "f32" | "f64" | "bool" | "u16" | "u8" | "u32" | "u64" | "usize")
}

/// 渲染文本 → 结构化类型（`from_rust_type`）：基本类型 → Prim；`JArray<E>` → 数组；`Object` → 根类；
/// 作用域内类型形参 → Param；短名表内 → 注册表类；其余以文本占位（`Param(text)`，只参与头名比较）。
/// 宿主基本类型与 `()` 无 RsType 形态 → None
pub fn from_rust_text(env: &InstrEnv, s: &str) -> Option<RsType> {
    let s = s.trim();
    if s == "()" {
        return Some(RsType::Unit);
    }
    if let Some(p) = prim_of(s) {
        return Some(RsType::Prim(p));
    }
    if is_value_form(s) {
        return None;
    }
    if s == OBJECT {
        return Some(RsType::Object);
    }
    let (head, args) = match s.split_once('<') {
        Some((h, rest)) => (h.trim(), rest.strip_suffix('>').map(split_top_args).unwrap_or_default()),
        None => (s, Vec::new()),
    };
    if head == ir::anchors::ARRAY {
        let elem = args.first().and_then(|a| from_rust_text(env, a)).unwrap_or(RsType::Object);
        return Some(RsType::array(elem));
    }
    if args.is_empty() && env.tparams.iter().any(|p| p == head) {
        return Some(RsType::Param(head.to_string()));
    }
    let arg_tys: Vec<RsType> = args.iter().map(|a| from_rust_text(env, a).unwrap_or(RsType::Object)).collect();
    match env.ctx.ty.binary_of(head) {
        Some(b) => Some(RsType::class(b.to_string(), arg_tys)),
        None => Some(RsType::Param(s.to_string())),
    }
}

fn prim_of(s: &str) -> Option<Prim> {
    Some(match s {
        "i8" => Prim::I8,
        "i16" => Prim::I16,
        "i32" => Prim::I32,
        "i64" => Prim::I64,
        "f32" => Prim::F32,
        "f64" => Prim::F64,
        "bool" => Prim::Bool,
        "u16" => Prim::U16,
        _ => return None,
    })
}

/// 后到形态能否对齐到提升声明类型（`_forms_alignable`，文本口径）：
/// 缺类型 / 同文本 → true；任一侧为值形态 → false；否则引用严格子类型
pub fn forms_alignable(env: &InstrEnv, later: Option<&str>, hoisted: Option<&str>) -> bool {
    let (Some(l), Some(h)) = (later, hoisted) else {
        return true;
    };
    if l == h {
        return true;
    }
    if is_value_form(l) || is_value_form(h) {
        return false;
    }
    match (from_rust_text(env, l), from_rust_text(env, h)) {
        (Some(lt), Some(ht)) => {
            !matches!(lt, RsType::Prim(_)) && !matches!(ht, RsType::Prim(_)) && instr::hierarchy::is_subtype(&env.ctx, &lt, &ht)
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rt(s: &str) -> String {
        ir::render::render_type(&parse_ir_type(s).expect(s))
    }

    #[test]
    fn roundtrip() {
        for s in ["i32", "()", "_", "HashMap<K, V>", "JArray<JArray<i8>>", "&[u8]", "&'a mut Foo", "(i32, bool)", "::a::B<C>::D"] {
            assert_eq!(rt(s), s);
        }
        assert!(parse_ir_type("Rc<dyn Foo>").is_none());
        assert_eq!(split_top_args("A<B, C>, D"), vec!["A<B, C>", "D"]);
    }
}

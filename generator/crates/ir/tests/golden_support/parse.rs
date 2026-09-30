//! golden 转换层专用：把 Python IR 的文本载体（`RsNamed.name`、`Call.func`、`Lit.value`、
//! 非标识符 `Var.name`、`CastExpr.target`、`turbofish`）解析成结构化 IR。
//!
//! 只在测试中存在——生产路径上文本载体没有对应物（产出方直接构造 IR）。
//! 覆盖 golden 中出现的 Rust 子集；解析不了的由调用方退回 Raw 并计数。

use ir::{anchors, BinOp, Expr, FloatLit, FloatTy, FnPath, Ident, IntTy, Lit, MacroCall, Path, PathSegment, Prim, Type, UnOp};

#[derive(Debug, Clone, PartialEq)]
enum Tok {
    Ident(String),
    Num(String),
    Str(String),
    Lifetime(String),
    /// 标点；`joint` 为真表示与下一记号之间无空白（`>` `>` 拼成 `>>` 用）
    Punct(&'static str, bool),
}

const PUNCTS: [&str; 27] = [
    "::", "==", "!=", "<=", ">=", "&&", "||", "<<", "->", "=>", "<", ">", "(", ")", "[", "]", "{", "}", ",", ".",
    "?", "!", "&", "*", "+", "-", "/",
];
const PUNCTS_EXTRA: [&str; 6] = ["%", "^", "|", "=", ";", ":"];

fn lex(src: &str) -> Option<Vec<Tok>> {
    let b = src.as_bytes();
    let mut i = 0;
    let mut out = Vec::new();
    while i < b.len() {
        let c = b[i] as char;
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        if c.is_ascii_alphabetic() || c == '_' {
            let st = i;
            while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push(Tok::Ident(src[st..i].to_string()));
        } else if c.is_ascii_digit() {
            let st = i;
            while i < b.len() {
                let d = b[i] as char;
                let exp_sign = matches!(d, '+' | '-') && matches!(b[i - 1], b'e' | b'E') && !src[st..i].contains('x');
                let dot = d == '.' && i + 1 < b.len() && (b[i + 1] as char).is_ascii_digit();
                if d.is_ascii_alphanumeric() || d == '_' || dot || exp_sign {
                    i += 1;
                } else {
                    break;
                }
            }
            out.push(Tok::Num(src[st..i].to_string()));
        } else if c == '"' {
            let (s, n) = lex_str(&src[i..])?;
            out.push(Tok::Str(s));
            i += n;
        } else if c == '\'' {
            let st = i + 1;
            i += 1;
            while i < b.len() && ((b[i] as char).is_ascii_alphanumeric() || b[i] == b'_') {
                i += 1;
            }
            out.push(Tok::Lifetime(src[st..i].to_string()));
        } else {
            let p = PUNCTS.iter().chain(PUNCTS_EXTRA.iter()).find(|p| src[i..].starts_with(**p))?;
            i += p.len();
            let joint = i < b.len() && !(b[i] as char).is_whitespace();
            out.push(Tok::Punct(p, joint));
        }
    }
    Some(out)
}

/// 字符串记号解码（`"` 起始），返回（内容，消耗字节数）。
fn lex_str(s: &str) -> Option<(String, usize)> {
    let mut out = String::new();
    let mut it = s.char_indices().skip(1);
    while let Some((i, c)) = it.next() {
        match c {
            '"' => return Some((out, i + 1)),
            '\\' => {
                let (_, e) = it.next()?;
                match e {
                    'n' => out.push('\n'),
                    'r' => out.push('\r'),
                    't' => out.push('\t'),
                    '0' => out.push('\0'),
                    '\\' | '"' | '\'' => out.push(e),
                    'u' => {
                        let mut hex = String::new();
                        it.next().filter(|(_, c)| *c == '{')?;
                        for (_, h) in it.by_ref() {
                            if h == '}' {
                                break;
                            }
                            hex.push(h);
                        }
                        out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
                    }
                    _ => return None,
                }
            }
            _ => out.push(c),
        }
    }
    None
}

struct Parser {
    toks: Vec<Tok>,
    pos: usize,
}

type Res<T> = Option<T>;

fn id(s: &str) -> Res<Ident> {
    Ident::new(s).ok()
}

impl Parser {
    fn new(src: &str) -> Res<Parser> {
        Some(Parser { toks: lex(src)?, pos: 0 })
    }

    fn peek(&self) -> Option<&Tok> {
        self.toks.get(self.pos)
    }

    fn peek_at(&self, k: usize) -> Option<&Tok> {
        self.toks.get(self.pos + k)
    }

    fn is_punct(&self, p: &str) -> bool {
        matches!(self.peek(), Some(Tok::Punct(q, _)) if *q == p)
    }

    fn is_ident(&self, s: &str) -> bool {
        matches!(self.peek(), Some(Tok::Ident(q)) if q == s)
    }

    fn eat(&mut self, p: &str) -> bool {
        if self.is_punct(p) {
            self.pos += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, p: &str) -> Res<()> {
        self.eat(p).then_some(())
    }

    fn ident(&mut self) -> Res<Ident> {
        match self.peek()? {
            Tok::Ident(s) => {
                let r = id(s)?;
                self.pos += 1;
                Some(r)
            }
            _ => None,
        }
    }

    fn done(&self) -> bool {
        self.pos == self.toks.len()
    }

    // ── 类型 ────────────────────────────────────────────────────────────

    fn ty(&mut self) -> Res<Type> {
        if self.eat("&") {
            let lifetime = match self.peek() {
                Some(Tok::Lifetime(l)) => {
                    let l = id(&l.clone())?;
                    self.pos += 1;
                    Some(l)
                }
                _ => None,
            };
            let mutable = self.is_ident("mut");
            if mutable {
                self.pos += 1;
            }
            if self.eat("[") {
                let e = self.ty()?;
                self.expect("]")?;
                return Some(Type::Slice(Box::new(e)));
            }
            return Some(Type::Ref { inner: Box::new(self.ty()?), mutable, lifetime });
        }
        if self.eat("(") {
            let mut elems = Vec::new();
            while !self.eat(")") {
                elems.push(self.ty()?);
                if !self.eat(",") {
                    self.expect(")")?;
                    break;
                }
            }
            return Some(if elems.is_empty() { Type::UNIT } else { Type::Tuple(elems) });
        }
        if self.is_ident("_") {
            self.pos += 1;
            return Some(Type::Infer);
        }
        let p = self.path(false)?;
        if !p.global && p.segments.len() == 1 && p.segments[0].generics.is_empty() {
            if let Some(prim) = Prim::from_name(p.segments[0].ident.as_str()) {
                return Some(Type::Prim(prim));
            }
        }
        Some(Type::Path(p))
    }

    fn types_until_gt(&mut self) -> Res<Vec<Type>> {
        let mut v = Vec::new();
        loop {
            v.push(self.ty()?);
            if self.eat(">") {
                return Some(v);
            }
            self.expect(",")?;
        }
    }

    /// 路径；`turbofish` 为真时段泛型须写成 `::<..>`（表达式位置），否则 `<..>`（类型位置）。
    fn path(&mut self, turbofish: bool) -> Res<Path> {
        let global = self.eat("::");
        let mut segments = Vec::new();
        loop {
            let ident = self.ident()?;
            let mut generics = Vec::new();
            if turbofish {
                if self.is_punct("::") && matches!(self.peek_at(1), Some(Tok::Punct("<", _))) {
                    self.pos += 2;
                    generics = self.types_until_gt()?;
                }
            } else if self.eat("<") {
                generics = self.types_until_gt()?;
            }
            segments.push(PathSegment::with_generics(ident, generics));
            let more = self.is_punct("::") && matches!(self.peek_at(1), Some(Tok::Ident(_)));
            if !more {
                return Some(Path { global, segments });
            }
            self.pos += 1;
        }
    }

    fn fn_path(&mut self) -> Res<FnPath> {
        if !self.eat("<") {
            return Some(FnPath::Path(self.path(true)?));
        }
        let self_ty = Box::new(self.ty()?);
        let trait_ = if self.is_ident("as") {
            self.pos += 1;
            Some(self.path(false)?)
        } else {
            None
        };
        self.expect(">")?;
        self.expect("::")?;
        let rest = self.path(true)?;
        (!rest.global).then_some(())?;
        Some(FnPath::Qualified { self_ty, trait_, rest: rest.segments })
    }

    // ── 表达式 ──────────────────────────────────────────────────────────

    fn bin_op(&self) -> Option<(BinOp, usize)> {
        match self.peek()? {
            Tok::Punct(">", true) if matches!(self.peek_at(1), Some(Tok::Punct(">", _))) => Some((BinOp::Shr, 2)),
            Tok::Punct(p, _) => BinOp::from_symbol(p).map(|op| (op, 1)),
            _ => None,
        }
    }

    fn expr(&mut self, min_prec: u8) -> Res<Expr> {
        let mut lhs = self.unary()?;
        while let Some((op, n)) = self.bin_op() {
            let prec = precedence(op);
            if prec < min_prec {
                break;
            }
            self.pos += n;
            let rhs = self.expr(prec + 1)?;
            lhs = Expr::binary(op, lhs, rhs);
        }
        Some(lhs)
    }

    fn unary(&mut self) -> Res<Expr> {
        if self.eat("-") {
            return Some(Expr::Unary { op: UnOp::Neg, expr: Box::new(self.unary()?) });
        }
        if self.eat("!") {
            return Some(Expr::Unary { op: UnOp::Not, expr: Box::new(self.unary()?) });
        }
        if self.eat("*") {
            return Some(Expr::Deref(Box::new(self.unary()?)));
        }
        if self.eat("&") {
            let mutable = self.is_ident("mut");
            if mutable {
                self.pos += 1;
            }
            return Some(Expr::Ref { expr: Box::new(self.unary()?), mutable });
        }
        let mut e = self.postfix()?;
        while self.is_ident("as") {
            self.pos += 1;
            e = Expr::Cast { expr: Box::new(e), ty: self.ty()?, outer_paren: false };
        }
        Some(e)
    }

    fn postfix(&mut self) -> Res<Expr> {
        let mut e = self.primary()?;
        loop {
            if self.eat("?") {
                e = Expr::try_(e);
            } else if self.eat("[") {
                let index = Box::new(self.expr(0)?);
                self.expect("]")?;
                e = Expr::Index { recv: Box::new(e), index };
            } else if self.eat(".") {
                let name = self.ident()?;
                let mut turbofish = Vec::new();
                if self.is_punct("::") {
                    self.pos += 1;
                    self.expect("<")?;
                    turbofish = self.types_until_gt()?;
                }
                if self.is_punct("(") {
                    let args = self.args()?;
                    e = Expr::MethodCall { recv: Box::new(e), method: name, turbofish, args };
                } else {
                    turbofish.is_empty().then_some(())?;
                    e = Expr::Field { recv: Box::new(e), name };
                }
            } else {
                return Some(e);
            }
        }
    }

    fn args(&mut self) -> Res<Vec<Expr>> {
        self.expect("(")?;
        let mut v = Vec::new();
        while !self.eat(")") {
            v.push(self.expr(0)?);
            if !self.eat(",") {
                self.expect(")")?;
                break;
            }
        }
        Some(v)
    }

    fn primary(&mut self) -> Res<Expr> {
        match self.peek()?.clone() {
            Tok::Num(n) => {
                self.pos += 1;
                Some(Expr::Lit(num_lit(&n)?))
            }
            Tok::Str(s) => {
                self.pos += 1;
                Some(Expr::Lit(Lit::Str(s)))
            }
            Tok::Ident(s) if s == "true" || s == "false" => {
                self.pos += 1;
                Some(Expr::Lit(Lit::Bool(s == "true")))
            }
            Tok::Punct("(", _) => {
                self.pos += 1;
                if self.eat(")") {
                    return Some(Expr::Lit(Lit::Unit));
                }
                let inner = self.expr(0)?;
                self.expect(")")?;
                Some(Expr::paren(inner))
            }
            Tok::Punct("<", _) => {
                let func = self.fn_path()?;
                Some(fold_call(func, self.args()?))
            }
            Tok::Ident(_) | Tok::Punct("::", _) => {
                let p = self.path(true)?;
                if self.is_punct("!") && p.segments.len() == 1 && !p.global {
                    self.pos += 1;
                    let name = p.segments.into_iter().next()?.ident;
                    return Some(Expr::Macro(MacroCall { name, args: self.args()? }));
                }
                if self.is_punct("(") {
                    return Some(fold_call(FnPath::Path(p), self.args()?));
                }
                let single = !p.global && p.segments.len() == 1 && p.segments[0].generics.is_empty();
                single.then(|| Expr::Var(p.segments.into_iter().next().unwrap().ident))
            }
            _ => None,
        }
    }
}

fn precedence(op: BinOp) -> u8 {
    match op {
        BinOp::Or => 1,
        BinOp::And => 2,
        BinOp::Eq | BinOp::Ne | BinOp::Lt | BinOp::Gt | BinOp::Le | BinOp::Ge => 3,
        BinOp::BitOr => 4,
        BinOp::BitXor => 5,
        BinOp::BitAnd => 6,
        BinOp::Shl | BinOp::Shr => 7,
        BinOp::Add | BinOp::Sub => 8,
        BinOp::Mul | BinOp::Div | BinOp::Rem => 9,
    }
}

/// 数字记号 → 整数 / 浮点字面量（带或不带后缀）。
fn num_lit(tok: &str) -> Res<Lit> {
    const INTS: [(&str, IntTy); 9] = [
        ("usize", IntTy::Usize),
        ("i8", IntTy::I8),
        ("i16", IntTy::I16),
        ("i32", IntTy::I32),
        ("i64", IntTy::I64),
        ("u8", IntTy::U8),
        ("u16", IntTy::U16),
        ("u32", IntTy::U32),
        ("u64", IntTy::U64),
    ];
    for (suf, fty) in [("f32", FloatTy::F32), ("f64", FloatTy::F64)] {
        if let Some(body) = tok.strip_suffix(suf) {
            return Some(Lit::Float { value: FloatLit::parse(body).ok()?, ty: fty });
        }
    }
    for (suf, ity) in INTS {
        if let Some(body) = tok.strip_suffix(suf) {
            return Some(Lit::int(body.parse().ok()?, ity));
        }
    }
    Some(Lit::Int { value: tok.parse().ok()?, ty: None })
}

/// Java 常量的固定形态折回字面量节点：`String::from("..")` → JString、
/// `Class::for_class(String::from(".."))` → ClassRef、`Object::default()` → Null。
fn fold_call(func: FnPath, args: Vec<Expr>) -> Expr {
    let names: Option<Vec<&str>> = match &func {
        FnPath::Path(p) if !p.global && p.segments.iter().all(|s| s.generics.is_empty()) => {
            Some(p.segments.iter().map(|s| s.ident.as_str()).collect())
        }
        _ => None,
    };
    match (names.as_deref(), args.as_slice()) {
        (Some([c, "from"]), [Expr::Lit(Lit::Str(s))]) if *c == anchors::STRING => Expr::Lit(Lit::JString(s.clone())),
        (Some([c, "for_class"]), [Expr::Lit(Lit::JString(s))]) if *c == anchors::CLASS => Expr::Lit(Lit::ClassRef(s.clone())),
        (Some([c, "default"]), []) if *c == anchors::OBJECT => Expr::Lit(Lit::Null),
        _ => Expr::Call { func, args },
    }
}

/// 整段解析为表达式（须消费全部记号）。
pub fn parse_expr(src: &str) -> Option<Expr> {
    let mut p = Parser::new(src)?;
    let e = p.expr(0)?;
    p.done().then_some(e)
}

/// 整段解析为类型。
pub fn parse_type(src: &str) -> Option<Type> {
    let mut p = Parser::new(src)?;
    let t = p.ty()?;
    p.done().then_some(t)
}

/// `Call.func` 文本 → 被调路径。
pub fn parse_fn_path(src: &str) -> Option<FnPath> {
    let mut p = Parser::new(src)?;
    let f = p.fn_path()?;
    p.done().then_some(f)
}

/// `StaticFieldRef.turbofish`（`''` / `'::<A, B>'`）→ 类型实参列表。
pub fn parse_turbofish(src: &str) -> Option<Vec<Type>> {
    if src.is_empty() {
        return Some(Vec::new());
    }
    let mut p = Parser::new(src)?;
    p.expect("::")?;
    p.expect("<")?;
    let v = p.types_until_gt()?;
    p.done().then_some(v)
}

/// 独立字面量形态：整数 / 浮点记号（含负号与 `f32::NAN` 等）、`true` / `false`、`()`、
/// `String::from("..")`、`String::from_utf16_lit(&[0x..])`、`Class::for_class(..)`、`Object::default()`。
pub fn parse_lit(src: &str) -> Option<Lit> {
    for (fty, name) in [(FloatTy::F32, "f32"), (FloatTy::F64, "f64")] {
        for (c, v) in [("NAN", FloatLit::Nan), ("INFINITY", FloatLit::Inf), ("NEG_INFINITY", FloatLit::NegInf)] {
            if src == format!("{name}::{c}") {
                return Some(Lit::Float { value: v, ty: fty });
            }
        }
    }
    if let Some(body) = src.strip_prefix(&format!("{}::from_utf16_lit(&[", anchors::STRING)) {
        let body = body.strip_suffix("])")?;
        let units: Option<Vec<u16>> = body
            .split(", ")
            .map(|h| u16::from_str_radix(h.strip_prefix("0x")?, 16).ok())
            .collect();
        return units.map(Lit::JStringUtf16);
    }
    if src.starts_with('-') || src.starts_with(|c: char| c.is_ascii_digit()) {
        let neg = src.starts_with('-');
        return match num_lit(src.trim_start_matches('-'))? {
            Lit::Int { value, ty } => Some(Lit::Int { value: if neg { -value } else { value }, ty }),
            Lit::Float { ty, .. } => {
                let body = src.strip_suffix(ty.suffix())?;
                Some(Lit::Float { value: FloatLit::parse(body).ok()?, ty })
            }
            _ => None,
        };
    }
    match parse_expr(src)? {
        Expr::Lit(l) => Some(l),
        _ => None,
    }
}

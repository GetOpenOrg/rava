//! 手写层（runtime/java_runtime/src 共置 `_impl.rs` / `_ext.rs`）的语法级扫描（syn）。
//!
//! 与 `codegen/native_upcalls.py` 同一语义，但走真实语法树而非正则：
//! - `#[jvm_native|jvm_boundary|jvm_ext(upcalls = "类.成员:描述符 …")]` 声明的 Rust→Java 回调边；
//! - `pub fn` 名（成员由手写体提供的判定）；
//! - 手写体分配：`let mut x = T::default(); x._init_not_null();`（单独的 `T::default()` 是 Java null）
//!   与构造调用 `T::new*(…)`；沿同文件 fn 调用传递闭包；
//! - `error.rs` 头部 `// vm-upcalls:` 行（VM 基础设施的无条件种子）；
//! - 调用表达式的实参 / 接收者类型（语法推断，见 [`TypedCall`]）：回调边的实参按调用点精确接入，
//!   推断不出时由引擎退回手写方法的值池。
//!
//! 成员匹配：Rust fn 名 = Java 名或 `名_<重载后缀>`；虚方法体前缀 `__impl_`；构造器 `<init>` ↔ `new`。

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use classfile::MemberRef;
use syn::visit::Visit;

const GENERATED_MARK: &str = "rava_macros::java_class";
const SUFFIXES: [&str; 2] = ["_impl.rs", "_ext.rs"];
const CTOR_RUST: &str = "new";
/// prelude 导出的 Java 根类型的 Rust 名（`Object::from(x)` 是保持身份的上转；字符串字面量产出 String）
const OBJECT_RUST: &str = "Object";
const STRING_RUST: &str = "String";
const VIRTUAL_PREFIX: &str = "__impl_";
/// 字段访问器前缀（rava_macros 生成）
const SET_PREFIX: &str = "__set_";
const GET_PREFIX: &str = "__get_";
const RUST_KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "Self",
    "static", "struct", "super", "trait", "true", "type", "union", "unsafe", "use", "where", "while", "abstract",
    "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

/// 回调目标：方法（描述符以 `(` 开头）或静态字段
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Upcall {
    Method(MemberRef),
    Field(MemberRef),
}

pub fn parse_upcall(tok: &str) -> Option<Upcall> {
    let colon = tok.find(':')?;
    let dot = tok[..colon].rfind('.')?;
    if dot == 0 {
        return None;
    }
    let m = MemberRef { owner: tok[..dot].into(), name: tok[dot + 1..colon].into(), desc: tok[colon + 1..].into() };
    Some(if m.desc.starts_with('(') { Upcall::Method(m) } else { Upcall::Field(m) })
}

/// Rust 类型路径（分段）→ binary name 候选（由调用方按类路径验证存在）
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeRef(pub Vec<String>);

/// 手写体里的一次调用：`recv.name(args)` 或 `Path::name(args)`。
///
/// 实参类型只从保持对象身份的形态推断——分配根（`T::new*` / `T::from(非 Java 值)` / 字符串字面量 /
/// `T::default()` + `_init_not_null`）经 `Object::from`、`clone`、`?`、`try_cast`、`into`、局部 `let`
/// 传递；其余一律 None（未知），由引擎退回值池。
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypedCall {
    pub name: String,
    /// 路径调用的类型段（`T::new` 的 `T`）
    pub path_ty: Option<TypeRef>,
    /// 方法调用的接收者类型：外层 None = 路径调用；内层 None = 未知
    pub recv: Option<Option<TypeRef>>,
    pub args: Vec<Option<TypeRef>>,
}

/// 接收者的静态类型（语法推断）：具名类型 / `T::m(…)` 的返回类型 / `x.__get_f()` 的字段类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SType {
    Named(TypeRef),
    Ret(TypeRef, String),
    Field(Box<SType>, String),
}

/// 手写体里的字段访问器调用：`recv.__set_<字段>(v)` / `recv.__get_<字段>()`（字段名即 Java 名）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FieldAccess {
    pub field: String,
    pub write: bool,
    /// 接收者静态类型；推断不出为 None（引擎按字段名在所有声明类上保守处理）
    pub recv: Option<SType>,
    /// 写入值的动态类型（同 [`TypedCall`] 实参规则）
    pub value: Option<TypeRef>,
}

#[derive(Debug, Default, Clone)]
pub struct FnInfo {
    pub is_pub: bool,
    pub upcalls: Vec<Upcall>,
    /// 调用点（含同文件被调 fn 的传递闭包）
    pub calls: Vec<TypedCall>,
    /// 宏调用内出现的标识符（syn 不展开宏：同名回调可能藏在宏里 → 按未知处理）
    pub opaque: HashSet<String>,
    /// 分配的类型（含同文件被调 fn 的传递闭包）
    pub allocs: BTreeSet<TypeRef>,
    /// 构造调用（类型, Rust 构造器名 `new` / `new_<后缀>`），同样传递闭包
    pub ctors: BTreeSet<(TypeRef, String)>,
    /// 字段访问器调用，同样传递闭包
    pub fields: Vec<FieldAccess>,
}

/// 一个类的共置手写文件汇总
#[derive(Debug, Default)]
pub struct ClassHw {
    pub files: Vec<PathBuf>,
    /// fn 名 → 信息（同名 fn 合并）
    pub fns: HashMap<String, FnInfo>,
}

/// 成员（Java 名）对应的手写体汇总
#[derive(Debug, Default)]
pub struct MemberHw {
    pub provided: bool,
    pub upcalls: Vec<Upcall>,
    pub allocs: BTreeSet<TypeRef>,
    pub ctors: BTreeSet<(TypeRef, String)>,
    pub calls: Vec<TypedCall>,
    pub opaque: HashSet<String>,
    pub fields: Vec<FieldAccess>,
    /// 命中的 fn 名（溯源）
    pub fns: Vec<String>,
}

pub struct Handwritten {
    src: PathBuf,
    /// `lib.rs` 的 `mod prelude` 导出（手写文件 `use crate::prelude::*` 引入）
    prelude: HashMap<String, Vec<String>>,
    cache: RefCell<HashMap<String, Rc<ClassHw>>>,
    abbrev: HashMap<String, String>,
    pub errors: RefCell<Vec<String>>,
}

pub fn to_snake(name: &str) -> String {
    let s: Vec<char> = name.replace('$', "_").chars().collect();
    // ([A-Z]+)([A-Z][a-z]) → \1_\2 ；([a-z\d])([A-Z]) → \1_\2
    let mut out = String::new();
    for i in 0..s.len() {
        let c = s[i];
        if i > 0 && c.is_ascii_uppercase() {
            let p = s[i - 1];
            let next_lower = s.get(i + 1).is_some_and(|n| n.is_ascii_lowercase());
            if p.is_ascii_lowercase() || p.is_ascii_digit() || (p.is_ascii_uppercase() && next_lower) {
                out.push('_');
            }
        }
        out.push(c);
    }
    let out = out.to_lowercase();
    if RUST_KEYWORDS.contains(&out.as_str()) {
        out + "_"
    } else {
        out
    }
}

pub fn member_matches(fn_name: &str, member: &str) -> bool {
    let rust = if member == "<init>" { CTOR_RUST } else { member };
    let f = fn_name.strip_prefix(VIRTUAL_PREFIX).unwrap_or(fn_name);
    f == rust || f.strip_prefix(rust).is_some_and(|r| r.starts_with('_'))
}

impl Handwritten {
    /// `runtime_dir` = runtime/java_runtime
    pub fn new(runtime_dir: &Path) -> Self {
        let mut abbrev = HashMap::new();
        if let Ok(s) = std::fs::read_to_string(runtime_dir.join("overload_abbrev.txt")) {
            for line in s.lines() {
                let line = line.trim();
                if line.is_empty() || line.starts_with('#') {
                    continue;
                }
                let mut it = line.split_whitespace();
                if let (Some(k), Some(v)) = (it.next(), it.next()) {
                    abbrev.insert(k.to_string(), v.to_string());
                }
            }
        }
        let prelude = std::fs::read_to_string(runtime_dir.join("src/lib.rs"))
            .ok()
            .and_then(|c| syn::parse_file(&c).ok())
            .map(|f| prelude_uses(&f))
            .unwrap_or_default();
        Handwritten {
            src: runtime_dir.join("src"),
            prelude,
            cache: RefCell::new(HashMap::new()),
            abbrev,
            errors: RefCell::new(Vec::new()),
        }
    }

    pub fn class(&self, cls: &str) -> Rc<ClassHw> {
        if let Some(c) = self.cache.borrow().get(cls) {
            return c.clone();
        }
        let hw = Rc::new(self.load(cls));
        self.cache.borrow_mut().insert(cls.to_string(), hw.clone());
        hw
    }

    fn load(&self, cls: &str) -> ClassHw {
        let (pkg, simple) = cls.rsplit_once('/').unwrap_or(("", cls));
        let mut hw = ClassHw::default();
        let mut calls = HashMap::new();
        for suf in SUFFIXES {
            let path = self.src.join(pkg).join(format!("{}{suf}", to_snake(simple)));
            let Ok(content) = std::fs::read_to_string(&path) else { continue };
            // 自动生成的类文件碰巧以 _impl.rs 结尾（类名含 Impl / Ext）：生成类文件恒含限定宏调用
            if content.contains(GENERATED_MARK) {
                continue;
            }
            match syn::parse_file(&content) {
                Ok(file) => scan_file(&file, &self.prelude, &mut hw.fns, &mut calls),
                Err(e) => self.errors.borrow_mut().push(format!("{}：{e}", path.display())),
            }
            hw.files.push(path);
        }
        close_transitive(&mut hw.fns, &calls);
        hw
    }

    /// 成员 `member`（Java 名；构造器 `<init>`）对应的手写体
    pub fn member(&self, cls: &str, member: &str) -> MemberHw {
        let hw = self.class(cls);
        let mut out = MemberHw::default();
        let mut names: Vec<&String> = hw.fns.keys().filter(|f| member_matches(f, member)).collect();
        names.sort();
        for n in names {
            let f = &hw.fns[n];
            out.provided |= f.is_pub;
            out.upcalls.extend(f.upcalls.iter().cloned());
            out.allocs.extend(f.allocs.iter().cloned());
            out.ctors.extend(f.ctors.iter().cloned());
            out.calls.extend(f.calls.iter().cloned());
            out.opaque.extend(f.opaque.iter().cloned());
            out.fields.extend(f.fields.iter().cloned());
            out.fns.push(n.clone());
        }
        out
    }

    /// 类有共置手写文件（含至少一个 pub fn）
    pub fn has_impls(&self, cls: &str) -> bool {
        self.class(cls).fns.values().any(|f| f.is_pub)
    }

    /// Rust 构造器名：`new` + 描述符重载后缀（codegen/type_map.mangle_name 同规则）
    pub fn ctor_name(&self, desc: &str) -> String {
        let suffix = self.descriptor_suffix(desc);
        if suffix.is_empty() {
            CTOR_RUST.to_string()
        } else {
            format!("{CTOR_RUST}_{suffix}")
        }
    }

    fn short(&self, cls: &str) -> String {
        let s = cls.rsplit('/').next().unwrap_or(cls).to_lowercase().replace('$', "_");
        self.abbrev.get(&s).cloned().unwrap_or(s)
    }

    pub fn descriptor_suffix(&self, desc: &str) -> String {
        let Some(params) = desc.strip_prefix('(').and_then(|d| d.split(')').next()) else { return String::new() };
        let b = params.as_bytes();
        let mut parts = Vec::new();
        let mut i = 0;
        let prim = |c: u8| match c {
            b'I' => Some("i"),
            b'J' => Some("l"),
            b'Z' => Some("z"),
            b'B' => Some("b"),
            b'S' => Some("s"),
            b'F' => Some("f"),
            b'D' => Some("d"),
            b'C' => Some("c"),
            _ => None,
        };
        while i < b.len() {
            let c = b[i];
            if let Some(p) = prim(c) {
                parts.push(p.to_string());
                i += 1;
            } else if c == b'L' {
                let Some(end) = params[i..].find(';').map(|e| e + i) else { break };
                parts.push(self.short(&params[i + 1..end]));
                i = end + 1;
            } else if c == b'[' {
                let mut j = i + 1;
                while j < b.len() && b[j] == b'[' {
                    j += 1;
                }
                if j >= b.len() {
                    break;
                }
                if b[j] == b'L' {
                    let Some(end) = params[j..].find(';').map(|e| e + j) else { break };
                    parts.push(format!("arr_{}", self.short(&params[j + 1..end])));
                    i = end + 1;
                } else {
                    parts.push(format!("arr_{}", prim(b[j]).unwrap_or("x")));
                    i = j + 1;
                }
            } else {
                i += 1;
            }
        }
        parts.join("_")
    }

    /// `error.rs` 的 `// vm-upcalls:` 行
    pub fn vm_upcalls(&self) -> Vec<Upcall> {
        let Ok(s) = std::fs::read_to_string(self.src.join("error.rs")) else { return vec![] };
        s.lines()
            .filter_map(|l| l.trim_start().strip_prefix("//").map(str::trim_start))
            .filter_map(|l| l.strip_prefix("vm-upcalls:"))
            .flat_map(|l| l.split_whitespace().filter_map(parse_upcall).collect::<Vec<_>>())
            .collect()
    }

    /// 类型路径 → binary name 候选（`Self` → 宿主类；单段按 use 表或同包；`_` 可能是 `$`）
    pub fn resolve_type(&self, host: &str, t: &TypeRef) -> Vec<String> {
        let pkg: Vec<&str> = host.rsplit_once('/').map_or(vec![], |(p, _)| p.split('/').collect());
        let segs = &t.0;
        if segs.len() == 1 && segs[0] == "Self" {
            return vec![host.to_string()];
        }
        let ty = segs.last().cloned().unwrap_or_default();
        let (base, rest): (Vec<String>, &[String]) = match segs.first().map(String::as_str) {
            Some("super") => (pkg.iter().map(|s| s.to_string()).collect(), &segs[1..]),
            Some("crate") => (vec![], &segs[1..]),
            _ if segs.len() == 1 => (pkg.iter().map(|s| s.to_string()).collect(), &segs[..]),
            _ => (vec![], &segs[..]),
        };
        let snake = to_snake(&ty);
        let mut full = base;
        for m in &rest[..rest.len().saturating_sub(1)] {
            let m = m.strip_prefix("r#").unwrap_or(m);
            if m != "implref" && m != "self" {
                full.push(m.to_string());
            }
        }
        // 末段模块与类型同名：可能是类文件模块（`stream_decoder::StreamDecoder`），也可能是包
        // （`charset::Charset`）——两种前缀都给出
        let mut prefixes = vec![full.clone()];
        if full.last() == Some(&snake) {
            full.pop();
            prefixes.insert(0, full);
        }
        let mut out = Vec::new();
        for mut p in prefixes {
            if p.is_empty() {
                p = pkg.iter().map(|s| s.to_string()).collect();
            }
            let prefix = p.join("/");
            out.extend(dollar_variants(&ty).into_iter().map(|v| if prefix.is_empty() { v } else { format!("{prefix}/{v}") }));
        }
        out
    }
}

/// `A_B_C` 的 `_` ↔ `$` 组合（嵌套类的 Rust 名把 `$` 换成 `_`）；原名优先
fn dollar_variants(ty: &str) -> Vec<String> {
    let idx: Vec<usize> = ty.match_indices('_').map(|m| m.0).filter(|i| *i > 0).collect();
    if idx.len() > 4 {
        return vec![ty.to_string()];
    }
    let mut out = Vec::new();
    for mask in 0..(1u32 << idx.len()) {
        let mut b = ty.as_bytes().to_vec();
        for (k, i) in idx.iter().enumerate() {
            if mask & (1 << k) != 0 {
                b[*i] = b'$';
            }
        }
        out.push(String::from_utf8(b).unwrap_or_default());
    }
    out
}

// ── syn 扫描 ────────────────────────────────────────────────────────────────

fn upcalls_of(attrs: &[syn::Attribute]) -> Vec<Upcall> {
    let mut out = Vec::new();
    for a in attrs {
        let Some(id) = a.path().get_ident() else { continue };
        if !matches!(id.to_string().as_str(), "jvm_native" | "jvm_boundary" | "jvm_ext") {
            continue;
        }
        let _ = a.parse_nested_meta(|meta| {
            if meta.path.is_ident("upcalls") {
                let s: syn::LitStr = meta.value()?.parse()?;
                out.extend(s.value().split_whitespace().filter_map(parse_upcall));
            }
            Ok(())
        });
    }
    out
}

fn path_segs(p: &syn::Path) -> Vec<String> {
    p.segments.iter().map(|s| s.ident.to_string()).collect()
}

/// 路径按 use 表展开首段
fn expand(uses: &HashMap<String, Vec<String>>, segs: Vec<String>) -> Vec<String> {
    match segs.first().and_then(|f| uses.get(f)) {
        Some(full) => full.iter().cloned().chain(segs.into_iter().skip(1)).collect(),
        None => segs,
    }
}

#[derive(Default)]
struct BodyScan {
    /// let 绑定 → 推断类型（重复绑定且类型不一致 → None）
    locals: HashMap<String, Option<Vec<String>>>,
    /// let mut 变量 → T::default() 的类型路径
    defaults: HashMap<String, Vec<String>>,
    inited: HashSet<String>,
    ctors: Vec<(Vec<String>, String)>,
    calls: HashSet<String>,
}

impl<'ast> Visit<'ast> for BodyScan {
    fn visit_local(&mut self, l: &'ast syn::Local) {
        if let syn::Pat::Ident(pi) = strip_type(&l.pat) {
            let t = l.init.as_ref().and_then(|i| infer(&i.expr, &self.locals));
            bind(&mut self.locals, pi.ident.to_string(), t);
            if pi.mutability.is_some() {
                if let Some(init) = &l.init {
                    if let syn::Expr::Call(c) = &*init.expr {
                        if let syn::Expr::Path(p) = &*c.func {
                            let segs = path_segs(&p.path);
                            if segs.len() >= 2 && segs.last().is_some_and(|s| s == "default") && c.args.is_empty() {
                                self.defaults.insert(pi.ident.to_string(), segs[..segs.len() - 1].to_vec());
                            }
                        }
                    }
                }
            }
        }
        syn::visit::visit_local(self, l);
    }

    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        let name = m.method.to_string();
        if name == "_init_not_null" {
            if let syn::Expr::Path(p) = &*m.receiver {
                if let Some(id) = p.path.get_ident() {
                    self.inited.insert(id.to_string());
                }
            }
        }
        self.calls.insert(name);
        syn::visit::visit_expr_method_call(self, m);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let segs = path_segs(&p.path);
            if let Some(last) = segs.last() {
                self.calls.insert(last.clone());
                let is_ctor = last == CTOR_RUST || last.starts_with("new_");
                let head_is_type = segs.len() >= 2 && segs[segs.len() - 2].starts_with(|ch: char| ch.is_ascii_uppercase());
                if is_ctor && head_is_type {
                    self.ctors.push((segs[..segs.len() - 1].to_vec(), last.clone()));
                }
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    // 嵌套 fn 单独登记（由外层 FileScan 处理）
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {}
}

/// 同名重复绑定且类型不一致 → None
fn bind<T: PartialEq>(env: &mut HashMap<String, Option<T>>, name: String, t: Option<T>) {
    match env.get(&name) {
        Some(old) if *old != t => {
            env.insert(name, None);
        }
        _ => {
            env.insert(name, t);
        }
    }
}

/// 类型注解的路径（剥引用 / 括号；`Self` 原样保留，由解析时换成宿主类）
fn type_path(t: &syn::Type) -> Option<Vec<String>> {
    match t {
        syn::Type::Reference(r) => type_path(&r.elem),
        syn::Type::Paren(p) => type_path(&p.elem),
        syn::Type::Group(g) => type_path(&g.elem),
        syn::Type::Path(p) if p.qself.is_none() => Some(path_segs(&p.path)),
        _ => None,
    }
}

/// 表达式的静态类型：形参 / let 注解、`T::default()` / `T::new*`、`T::m(…)` 的返回、`x.__get_f()` 的字段；
/// 其余退回动态类型推断
fn stype(e: &syn::Expr, statics: &HashMap<String, Option<SType>>, locals: &HashMap<String, Option<Vec<String>>>) -> Option<SType> {
    use syn::Expr;
    let direct = match e {
        Expr::Paren(p) => stype(&p.expr, statics, locals),
        Expr::Group(g) => stype(&g.expr, statics, locals),
        Expr::Reference(r) => stype(&r.expr, statics, locals),
        Expr::Try(t) => stype(&t.expr, statics, locals),
        Expr::Path(p) => p.path.get_ident().and_then(|i| statics.get(&i.to_string()).cloned().flatten()),
        Expr::MethodCall(m) => {
            let name = m.method.to_string();
            match name.strip_prefix(GET_PREFIX) {
                Some(f) if m.args.is_empty() => stype(&m.receiver, statics, locals).map(|r| SType::Field(Box::new(r), f.to_string())),
                _ if matches!(name.as_str(), "clone" | "unwrap" | "expect" | "unwrap_or_else" | "unwrap_or") => {
                    stype(&m.receiver, statics, locals)
                }
                _ => None,
            }
        }
        Expr::Call(c) => match &*c.func {
            Expr::Path(p) => {
                let segs = path_segs(&p.path);
                match segs.split_last() {
                    Some((last, head)) if head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase())) => {
                        let t = TypeRef(head.to_vec());
                        Some(if last == "default" || last == CTOR_RUST || last.starts_with("new_") {
                            SType::Named(t)
                        } else {
                            SType::Ret(t, last.clone())
                        })
                    }
                    _ => None,
                }
            }
            _ => None,
        },
        _ => None,
    };
    direct.or_else(|| infer(e, locals).map(|t| SType::Named(TypeRef(t))))
}

fn expand_s(uses: &HashMap<String, Vec<String>>, s: SType, self_ty: &Option<Vec<String>>) -> SType {
    let tr = |t: TypeRef| match (t.0.as_slice(), self_ty) {
        ([one], Some(st)) if one == "Self" => TypeRef(expand(uses, st.clone())),
        _ => TypeRef(expand(uses, t.0)),
    };
    match s {
        SType::Named(t) => SType::Named(tr(t)),
        SType::Ret(t, m) => SType::Ret(tr(t), m),
        SType::Field(b, f) => SType::Field(Box::new(expand_s(uses, *b, self_ty)), f),
    }
}

/// 表达式的对象类型（语法推断；见 [`TypedCall`]）
fn infer(e: &syn::Expr, locals: &HashMap<String, Option<Vec<String>>>) -> Option<Vec<String>> {
    use syn::Expr;
    match e {
        Expr::Paren(p) => infer(&p.expr, locals),
        Expr::Group(g) => infer(&g.expr, locals),
        Expr::Reference(r) => infer(&r.expr, locals),
        Expr::Try(t) => infer(&t.expr, locals),
        Expr::Lit(l) if matches!(l.lit, syn::Lit::Str(_)) => Some(vec![STRING_RUST.to_string()]),
        Expr::Path(p) => p.path.get_ident().and_then(|i| locals.get(&i.to_string()).cloned().flatten()),
        Expr::Call(c) => {
            let Expr::Path(p) = &*c.func else { return None };
            let segs = path_segs(&p.path);
            let (last, head) = segs.split_last()?;
            let arg0 = || c.args.first().and_then(|a| infer(a, locals));
            if head.last().is_some_and(|h| h == OBJECT_RUST) && last == "from" {
                return arg0();
            }
            if head == ["Clone"] && last == "clone" {
                return arg0();
            }
            let head_is_type = head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase()));
            if head_is_type && (last == CTOR_RUST || last.starts_with("new_")) {
                return Some(head.to_vec());
            }
            if head_is_type && last == "from" {
                // 上转保持动态类型；非 Java 值（Rust 字符串等）→ 转换产出的 T
                return arg0().or_else(|| Some(head.to_vec()));
            }
            None
        }
        Expr::MethodCall(m) => match m.method.to_string().as_str() {
            "clone" | "try_cast" | "into" | "unwrap" | "expect" => infer(&m.receiver, locals),
            _ => None,
        },
        _ => None,
    }
}

/// 第二遍：按第一遍的 let 绑定推断调用点实参；按源码顺序维护静态类型作用域
struct CallScan<'a> {
    locals: &'a HashMap<String, Option<Vec<String>>>,
    /// 形参 / self / let 绑定 → 静态类型（块作用域；其余模式绑定遮蔽为 None）
    scope: HashMap<String, Option<SType>>,
    calls: Vec<(String, Option<Vec<String>>, Option<Option<Vec<String>>>, Vec<Option<Vec<String>>>)>,
    /// (字段, 写, 接收者静态类型, 写入值类型)
    fields: Vec<(String, bool, Option<SType>, Option<Vec<String>>)>,
    opaque: HashSet<String>,
}

fn macro_idents(ts: proc_macro2::TokenStream, out: &mut HashSet<String>) {
    for t in ts {
        match t {
            proc_macro2::TokenTree::Ident(i) => {
                out.insert(i.to_string());
            }
            proc_macro2::TokenTree::Group(g) => macro_idents(g.stream(), out),
            _ => {}
        }
    }
}

impl<'ast> Visit<'ast> for CallScan<'_> {
    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        let name = m.method.to_string();
        let access = match (name.strip_prefix(SET_PREFIX), name.strip_prefix(GET_PREFIX)) {
            (Some(f), _) if m.args.len() == 1 => Some((f, true)),
            (_, Some(f)) if m.args.is_empty() => Some((f, false)),
            _ => None,
        };
        if let Some((f, write)) = access {
            // Java 字段名是 Rust 关键字时访问器带 `_` 后缀（`in` → `__set_in_`）
            let f = f.strip_suffix('_').filter(|k| RUST_KEYWORDS.contains(k)).unwrap_or(f);
            let value = m.args.first().and_then(|a| infer(a, self.locals));
            self.fields.push((f.to_string(), write, stype(&m.receiver, &self.scope, self.locals), value));
        }
        let args = m.args.iter().map(|a| infer(a, self.locals)).collect();
        self.calls.push((name, None, Some(infer(&m.receiver, self.locals)), args));
        syn::visit::visit_expr_method_call(self, m);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let segs = path_segs(&p.path);
            if let Some((last, head)) = segs.split_last() {
                let args = c.args.iter().map(|a| infer(a, self.locals)).collect();
                let ty = (!head.is_empty()).then(|| head.to_vec());
                self.calls.push((last.clone(), ty, None, args));
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        macro_idents(m.tokens.clone(), &mut self.opaque);
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        let outer = self.scope.clone();
        syn::visit::visit_block(self, b);
        self.scope = outer;
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        let outer = self.scope.clone();
        syn::visit::visit_expr_closure(self, c);
        self.scope = outer;
    }

    // for / while let / if let / match 分支的模式绑定只在其内有效
    fn visit_expr_for_loop(&mut self, e: &'ast syn::ExprForLoop) {
        let outer = self.scope.clone();
        syn::visit::visit_expr_for_loop(self, e);
        self.scope = outer;
    }

    fn visit_expr_while(&mut self, e: &'ast syn::ExprWhile) {
        let outer = self.scope.clone();
        syn::visit::visit_expr_while(self, e);
        self.scope = outer;
    }

    fn visit_expr_if(&mut self, e: &'ast syn::ExprIf) {
        let outer = self.scope.clone();
        syn::visit::visit_expr_if(self, e);
        self.scope = outer;
    }

    fn visit_arm(&mut self, a: &'ast syn::Arm) {
        let outer = self.scope.clone();
        syn::visit::visit_arm(self, a);
        self.scope = outer;
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        syn::visit::visit_local(self, l);
        if let syn::Pat::Ident(pi) = strip_type(&l.pat) {
            let st = match &l.pat {
                syn::Pat::Type(pt) => type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))),
                _ => l.init.as_ref().and_then(|i| stype(&i.expr, &self.scope, self.locals)),
            };
            self.scope.insert(pi.ident.to_string(), st);
        }
    }

    // 模式绑定（闭包形参 / match / if let / for）遮蔽同名变量
    fn visit_pat_ident(&mut self, p: &'ast syn::PatIdent) {
        self.scope.insert(p.ident.to_string(), None);
        syn::visit::visit_pat_ident(self, p);
    }

    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {}
}

fn strip_type(p: &syn::Pat) -> &syn::Pat {
    match p {
        syn::Pat::Type(t) => &t.pat,
        other => other,
    }
}

struct RawFn {
    info: FnInfo,
    calls: HashSet<String>,
}

struct FileScan<'a> {
    uses: &'a HashMap<String, Vec<String>>,
    fns: Vec<(String, RawFn)>,
    /// 当前 impl 块的 self 类型
    self_ty: Option<Vec<String>>,
}

impl FileScan<'_> {
    fn add(&mut self, sig: &syn::Signature, is_pub: bool, attrs: &[syn::Attribute], block: &syn::Block) {
        let name = sig.ident.to_string();
        let mut b = BodyScan::default();
        b.visit_block(block);
        let mut scope = HashMap::new();
        for a in &sig.inputs {
            match a {
                syn::FnArg::Receiver(_) => {
                    scope.insert("self".to_string(), Some(SType::Named(TypeRef(vec!["Self".into()]))));
                }
                syn::FnArg::Typed(pt) => {
                    if let syn::Pat::Ident(pi) = &*pt.pat {
                        scope.insert(pi.ident.to_string(), type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))));
                    }
                }
            }
        }
        let mut info = FnInfo { is_pub, upcalls: upcalls_of(attrs), ..Default::default() };
        for (var, ty) in &b.defaults {
            if b.inited.contains(var) {
                info.allocs.insert(TypeRef(expand(self.uses, ty.clone())));
                b.locals.insert(var.clone(), Some(ty.clone()));
            }
        }
        let mut cs = CallScan { locals: &b.locals, scope, calls: Vec::new(), fields: Vec::new(), opaque: HashSet::new() };
        cs.visit_block(block);
        for (field, write, recv, value) in cs.fields {
            info.fields.push(FieldAccess {
                field,
                write,
                recv: recv.map(|r| expand_s(self.uses, r, &self.self_ty)),
                value: value.map(|v| TypeRef(expand(self.uses, v))),
            });
        }
        let tr = |t: Option<Vec<String>>| t.map(|t| TypeRef(expand(self.uses, t)));
        for (name, ty, recv, args) in cs.calls {
            info.calls.push(TypedCall {
                name,
                path_ty: tr(ty),
                recv: recv.map(tr),
                args: args.into_iter().map(tr).collect(),
            });
        }
        info.opaque = cs.opaque;
        for (ty, ctor) in b.ctors {
            info.ctors.insert((TypeRef(expand(self.uses, ty)), ctor));
        }
        self.fns.push((name, RawFn { info, calls: b.calls }));
    }
}

impl<'ast> Visit<'ast> for FileScan<'_> {
    fn visit_item_fn(&mut self, f: &'ast syn::ItemFn) {
        let is_pub = matches!(f.vis, syn::Visibility::Public(_));
        // 自由 fn 内的 Self 无意义：暂离 impl 上下文
        let outer = self.self_ty.take();
        self.add(&f.sig, is_pub, &f.attrs, &f.block);
        syn::visit::visit_item_fn(self, f);
        self.self_ty = outer;
    }

    fn visit_item_impl(&mut self, i: &'ast syn::ItemImpl) {
        let outer = std::mem::replace(&mut self.self_ty, type_path(&i.self_ty));
        syn::visit::visit_item_impl(self, i);
        self.self_ty = outer;
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let is_pub = matches!(f.vis, syn::Visibility::Public(_));
        self.add(&f.sig, is_pub, &f.attrs, &f.block);
        syn::visit::visit_impl_item_fn(self, f);
    }
}

fn collect_uses(tree: &syn::UseTree, prefix: &mut Vec<String>, out: &mut HashMap<String, Vec<String>>) {
    match tree {
        syn::UseTree::Path(p) => {
            prefix.push(p.ident.to_string());
            collect_uses(&p.tree, prefix, out);
            prefix.pop();
        }
        syn::UseTree::Name(n) => {
            let mut full = prefix.clone();
            full.push(n.ident.to_string());
            out.insert(n.ident.to_string(), full);
        }
        syn::UseTree::Rename(r) => {
            let mut full = prefix.clone();
            full.push(r.ident.to_string());
            out.insert(r.rename.to_string(), full);
        }
        syn::UseTree::Group(g) => {
            for t in &g.items {
                collect_uses(t, prefix, out);
            }
        }
        syn::UseTree::Glob(_) => {}
    }
}

struct UseScan(HashMap<String, Vec<String>>);

impl<'ast> Visit<'ast> for UseScan {
    fn visit_item_use(&mut self, u: &'ast syn::ItemUse) {
        collect_uses(&u.tree, &mut Vec::new(), &mut self.0);
    }
}

/// `mod prelude { pub use super::… }` → 名字 → `crate::…` 路径
fn prelude_uses(file: &syn::File) -> HashMap<String, Vec<String>> {
    let mut us = UseScan(HashMap::new());
    for item in &file.items {
        if let syn::Item::Mod(m) = item {
            if m.ident == "prelude" {
                if let Some((_, items)) = &m.content {
                    for i in items {
                        us.visit_item(i);
                    }
                }
            }
        }
    }
    us.0.into_iter()
        .map(|(k, mut v)| {
            if v.first().is_some_and(|f| f == "super") {
                v[0] = "crate".into();
            }
            (k, v)
        })
        .collect()
}

fn scan_file(
    file: &syn::File,
    prelude: &HashMap<String, Vec<String>>,
    fns: &mut HashMap<String, FnInfo>,
    calls: &mut HashMap<String, HashSet<String>>,
) {
    let mut us = UseScan(prelude.clone());
    us.visit_file(file);
    let mut fs = FileScan { uses: &us.0, fns: Vec::new(), self_ty: None };
    fs.visit_file(file);
    for (name, raw) in fs.fns {
        calls.entry(name.clone()).or_default().extend(raw.calls);
        let e = fns.entry(name).or_default();
        e.is_pub |= raw.info.is_pub;
        e.upcalls.extend(raw.info.upcalls);
        e.allocs.extend(raw.info.allocs);
        e.ctors.extend(raw.info.ctors);
        e.calls.extend(raw.info.calls);
        e.opaque.extend(raw.info.opaque);
        e.fields.extend(raw.info.fields);
    }
}

/// 分配 / 构造沿同文件 fn 调用传递（被调 fn 名须在同类手写文件内）
fn close_transitive(fns: &mut HashMap<String, FnInfo>, calls: &HashMap<String, HashSet<String>>) {
    let names: Vec<String> = fns.keys().cloned().collect();
    let mut closed = Vec::new();
    for n in &names {
        let mut seen: HashSet<&str> = HashSet::from([n.as_str()]);
        let mut stack = vec![n.as_str()];
        let (mut allocs, mut ctors) = (BTreeSet::new(), BTreeSet::new());
        let (mut tcalls, mut opaque, mut fields) = (Vec::new(), HashSet::new(), Vec::new());
        while let Some(x) = stack.pop() {
            if let Some(f) = fns.get(x) {
                allocs.extend(f.allocs.iter().cloned());
                ctors.extend(f.ctors.iter().cloned());
                tcalls.extend(f.calls.iter().cloned());
                opaque.extend(f.opaque.iter().cloned());
                fields.extend(f.fields.iter().cloned());
            }
            for c in calls.get(x).into_iter().flatten() {
                if fns.contains_key(c) && seen.insert(c.as_str()) {
                    stack.push(c.as_str());
                }
            }
        }
        closed.push((n.clone(), allocs, ctors, tcalls, opaque, fields));
    }
    for (n, a, c, t, o, fl) in closed {
        let f = fns.get_mut(&n).expect("fn 名来自同一表");
        f.allocs = a;
        f.ctors = c;
        f.calls = t;
        f.opaque = o;
        f.fields = fl;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fields_of(src: &str, f: &str) -> Vec<FieldAccess> {
        let file = syn::parse_file(src).expect("测试源码可解析");
        let (mut fns, mut calls) = (HashMap::new(), HashMap::new());
        scan_file(&file, &HashMap::new(), &mut fns, &mut calls);
        fns.remove(f).map(|i| i.fields).unwrap_or_default()
    }

    fn named(p: &[&str]) -> SType {
        SType::Named(TypeRef(p.iter().map(|s| s.to_string()).collect()))
    }

    #[test]
    fn field_access_receivers() {
        let src = r#"
            impl Decoder {
                pub fn open(&self, x: &Stream) {
                    self.__set_in_(Stream::new());
                    self.__get_fd().__set_fd(1);
                    let h = Holder::new(1).unwrap_or_else(|e| panic!());
                    h.__set_status(2);
                    { let h = other(); h.__set_status(3); }
                    h.__set_status(4);
                    for x in xs { x.__set_a(0); }
                    x.__set_b(0);
                    let v = x.__get_c();
                }
            }
        "#;
        let fs = fields_of(src, "open");
        let get = |i: usize| (fs[i].field.as_str(), fs[i].write, fs[i].recv.clone());
        assert_eq!(get(0), ("in", true, Some(named(&["Decoder"]))));
        assert_eq!(fs[0].value, Some(TypeRef(vec!["Stream".into()])));
        // 外层访问器先登记，再进接收者
        assert_eq!(get(1), ("fd", true, Some(SType::Field(Box::new(named(&["Decoder"])), "fd".into()))));
        assert_eq!(get(2), ("fd", false, Some(named(&["Decoder"]))));
        assert_eq!(get(3), ("status", true, Some(named(&["Holder"]))));
        assert_eq!(get(4), ("status", true, None));
        assert_eq!(get(5), ("status", true, Some(named(&["Holder"]))));
        // for 模式遮蔽形参 x；块外恢复
        assert_eq!(get(6), ("a", true, None));
        assert_eq!(get(7), ("b", true, Some(named(&["Stream"]))));
        assert_eq!(get(8), ("c", false, Some(named(&["Stream"]))));
    }
}

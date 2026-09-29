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
            if m != "implref" && m != "self" && m != snake {
                full.push(m.to_string());
            }
        }
        if full.is_empty() {
            full = pkg.iter().map(|s| s.to_string()).collect();
        }
        let prefix = full.join("/");
        dollar_variants(&ty)
            .into_iter()
            .map(|v| if prefix.is_empty() { v } else { format!("{prefix}/{v}") })
            .collect()
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
            let name = pi.ident.to_string();
            match self.locals.get(&name) {
                Some(old) if *old != t => {
                    self.locals.insert(name, None);
                }
                _ => {
                    self.locals.insert(name, t);
                }
            }
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

/// 第二遍：按第一遍的 let 绑定推断调用点实参
struct CallScan<'a> {
    locals: &'a HashMap<String, Option<Vec<String>>>,
    calls: Vec<(String, Option<Vec<String>>, Option<Option<Vec<String>>>, Vec<Option<Vec<String>>>)>,
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
        let args = m.args.iter().map(|a| infer(a, self.locals)).collect();
        self.calls.push((m.method.to_string(), None, Some(infer(&m.receiver, self.locals)), args));
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
}

impl FileScan<'_> {
    fn add(&mut self, name: String, is_pub: bool, attrs: &[syn::Attribute], block: &syn::Block) {
        let mut b = BodyScan::default();
        b.visit_block(block);
        let mut info = FnInfo { is_pub, upcalls: upcalls_of(attrs), ..Default::default() };
        for (var, ty) in &b.defaults {
            if b.inited.contains(var) {
                info.allocs.insert(TypeRef(expand(self.uses, ty.clone())));
                b.locals.insert(var.clone(), Some(ty.clone()));
            }
        }
        let mut cs = CallScan { locals: &b.locals, calls: Vec::new(), opaque: HashSet::new() };
        cs.visit_block(block);
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
        self.add(f.sig.ident.to_string(), is_pub, &f.attrs, &f.block);
        syn::visit::visit_item_fn(self, f);
    }

    fn visit_impl_item_fn(&mut self, f: &'ast syn::ImplItemFn) {
        let is_pub = matches!(f.vis, syn::Visibility::Public(_));
        self.add(f.sig.ident.to_string(), is_pub, &f.attrs, &f.block);
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
    let mut fs = FileScan { uses: &us.0, fns: Vec::new() };
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
        let (mut tcalls, mut opaque) = (Vec::new(), HashSet::new());
        while let Some(x) = stack.pop() {
            if let Some(f) = fns.get(x) {
                allocs.extend(f.allocs.iter().cloned());
                ctors.extend(f.ctors.iter().cloned());
                tcalls.extend(f.calls.iter().cloned());
                opaque.extend(f.opaque.iter().cloned());
            }
            for c in calls.get(x).into_iter().flatten() {
                if fns.contains_key(c) && seen.insert(c.as_str()) {
                    stack.push(c.as_str());
                }
            }
        }
        closed.push((n.clone(), allocs, ctors, tcalls, opaque));
    }
    for (n, a, c, t, o) in closed {
        let f = fns.get_mut(&n).expect("fn 名来自同一表");
        f.allocs = a;
        f.ctors = c;
        f.calls = t;
        f.opaque = o;
    }
}

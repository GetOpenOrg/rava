//! 手写体的语法推断：let 绑定 / 调用点实参与接收者的类型、静态类型作用域、构造与数组视图识别。

use std::collections::{HashMap, HashSet};

use syn::visit::Visit;

use super::*;

pub(super) fn path_segs(p: &syn::Path) -> Vec<String> {
    p.segments.iter().map(|s| s.ident.to_string()).collect()
}

/// 路径按 use 表展开首段
pub(super) fn expand(uses: &HashMap<String, Vec<String>>, segs: Vec<String>) -> Vec<String> {
    match segs.first().and_then(|f| uses.get(f)) {
        Some(full) => full.iter().cloned().chain(segs.into_iter().skip(1)).collect(),
        None => segs,
    }
}

#[derive(Default)]
pub(super) struct BodyScan {
    /// let 绑定 → 推断类型（重复绑定且类型不一致 → None）
    pub(super) locals: HashMap<String, Option<Vec<String>>>,
    /// let mut 变量 → T::default() 的类型路径
    pub(super) defaults: HashMap<String, Vec<String>>,
    pub(super) inited: HashSet<String>,
    pub(super) ctors: Vec<(Vec<String>, String)>,
    pub(super) calls: HashSet<String>,
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
pub(super) fn bind<T: PartialEq>(env: &mut HashMap<String, Option<T>>, name: String, t: Option<T>) {
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
pub(super) fn type_path(t: &syn::Type) -> Option<Vec<String>> {
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
pub(super) fn stype(e: &syn::Expr, statics: &HashMap<String, Option<SType>>, locals: &HashMap<String, Option<Vec<String>>>) -> Option<SType> {
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
                _ => stype(&m.receiver, statics, locals).map(|r| SType::Call(Box::new(r), name)),
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

pub(super) fn expand_s(uses: &HashMap<String, Vec<String>>, s: SType, self_ty: &Option<Vec<String>>) -> SType {
    let tr = |t: TypeRef| match (t.0.as_slice(), self_ty) {
        ([one], Some(st)) if one == "Self" => TypeRef(expand(uses, st.clone())),
        _ => TypeRef(expand(uses, t.0)),
    };
    match s {
        SType::Named(t) => SType::Named(tr(t)),
        SType::Ret(t, m) => SType::Ret(tr(t), m),
        SType::Field(b, f) => SType::Field(Box::new(expand_s(uses, *b, self_ty)), f),
        SType::Call(b, m) => SType::Call(Box::new(expand_s(uses, *b, self_ty)), m),
    }
}

/// 表达式的对象类型（语法推断；见 [`TypedCall`]）
pub(super) fn infer(e: &syn::Expr, locals: &HashMap<String, Option<Vec<String>>>) -> Option<Vec<String>> {
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
pub(super) struct CallScan<'a> {
    pub(super) locals: &'a HashMap<String, Option<Vec<String>>>,
    /// 形参 / self / let 绑定 → 静态类型（块作用域；其余模式绑定遮蔽为 None）
    pub(super) scope: HashMap<String, Option<SType>>,
    /// 不可变 let 绑定到构造调用 `T::new*(…)` 的局部变量 → `T`（块作用域，遮蔽即移除）
    pub(super) fresh: HashMap<String, Vec<String>>,
    pub(super) calls: Vec<(String, Option<Vec<String>>, Option<Option<Vec<String>>>, Vec<Option<Vec<String>>>, Option<Vec<String>>, Option<SType>)>,
    /// (字段, 写, 接收者静态类型, 写入值类型, 接收者是 self, static 写访问器路径调用)
    pub(super) fields: Vec<(String, bool, Option<SType>, Option<Vec<String>>, bool, bool)>,
    pub(super) opaque: HashSet<String>,
}

pub(super) fn macro_idents(ts: proc_macro2::TokenStream, out: &mut HashSet<String>) {
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
            let on_self = matches!(&*m.receiver, syn::Expr::Path(p) if p.path.is_ident("self"));
            self.fields.push((f.to_string(), write, stype(&m.receiver, &self.scope, self.locals), value, on_self, false));
        }
        let args = m.args.iter().map(|a| infer(a, self.locals)).collect();
        let fresh = match &*m.receiver {
            syn::Expr::Path(p) => p.path.get_ident().and_then(|i| self.fresh.get(&i.to_string()).cloned()),
            _ => None,
        };
        let srecv = stype(&m.receiver, &self.scope, self.locals);
        self.calls.push((name, None, Some(infer(&m.receiver, self.locals)), args, fresh, srecv));
        syn::visit::visit_expr_method_call(self, m);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let segs = path_segs(&p.path);
            if let Some((last, head)) = segs.split_last() {
                // static 字段写访问器 `T::set_<字段>(v)`（类型段首字母大写；关键字字段名带 `_` 后缀）
                if let Some(f) = last.strip_prefix(STATIC_SET_PREFIX).filter(|_| c.args.len() == 1) {
                    if head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase())) {
                        let f = f.strip_suffix('_').filter(|k| RUST_KEYWORDS.contains(k)).unwrap_or(f);
                        let value = c.args.first().and_then(|a| infer(a, self.locals));
                        self.fields.push((f.to_string(), true, Some(SType::Named(TypeRef(head.to_vec()))), value, false, true));
                    }
                }
                let args = c.args.iter().map(|a| infer(a, self.locals)).collect();
                let ty = (!head.is_empty()).then(|| head.to_vec());
                self.calls.push((last.clone(), ty, None, args, None, None));
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        macro_idents(m.tokens.clone(), &mut self.opaque);
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_block(self, b);
        (self.scope, self.fresh) = outer;
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_expr_closure(self, c);
        (self.scope, self.fresh) = outer;
    }

    // for / while let / if let / match 分支的模式绑定只在其内有效
    fn visit_expr_for_loop(&mut self, e: &'ast syn::ExprForLoop) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_expr_for_loop(self, e);
        (self.scope, self.fresh) = outer;
    }

    fn visit_expr_while(&mut self, e: &'ast syn::ExprWhile) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_expr_while(self, e);
        (self.scope, self.fresh) = outer;
    }

    fn visit_expr_if(&mut self, e: &'ast syn::ExprIf) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_expr_if(self, e);
        (self.scope, self.fresh) = outer;
    }

    fn visit_arm(&mut self, a: &'ast syn::Arm) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_arm(self, a);
        (self.scope, self.fresh) = outer;
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        syn::visit::visit_local(self, l);
        if let syn::Pat::Ident(pi) = strip_type(&l.pat) {
            let st = match &l.pat {
                syn::Pat::Type(pt) => type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))),
                _ => l.init.as_ref().and_then(|i| stype(&i.expr, &self.scope, self.locals)),
            };
            self.scope.insert(pi.ident.to_string(), st);
            let name = pi.ident.to_string();
            match l.init.as_ref().filter(|i| pi.mutability.is_none() && i.diverge.is_none()).and_then(|i| ctor_type(&i.expr)) {
                Some(t) => {
                    self.fresh.insert(name, t);
                }
                None => {
                    self.fresh.remove(&name);
                }
            }
        }
    }

    // 模式绑定（闭包形参 / match / if let / for）遮蔽同名变量
    fn visit_pat_ident(&mut self, p: &'ast syn::PatIdent) {
        self.scope.insert(p.ident.to_string(), None);
        self.fresh.remove(&p.ident.to_string());
        syn::visit::visit_pat_ident(self, p);
    }

    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {}
}

/// 构造调用 `T::new*(…)`（可带 `?` / 括号）的类型路径
pub(super) fn ctor_type(e: &syn::Expr) -> Option<Vec<String>> {
    match e {
        syn::Expr::Paren(p) => ctor_type(&p.expr),
        syn::Expr::Group(g) => ctor_type(&g.expr),
        syn::Expr::Try(t) => ctor_type(&t.expr),
        syn::Expr::Call(c) => {
            let syn::Expr::Path(p) = &*c.func else { return None };
            let segs = path_segs(&p.path);
            let (last, head) = segs.split_last()?;
            let head_is_type = head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase()));
            (head_is_type && (last == CTOR_RUST || last.starts_with("new_"))).then(|| head.to_vec())
        }
        _ => None,
    }
}

/// 手写体取得数组视图的标识符：数组类型本身、协变视图、Object 上的数组存取 / 转换
pub(super) fn is_array_ident(s: &str) -> bool {
    s == "JArray" || s == "__view_into" || s == "try_cast_array" || s.starts_with("array_store")
}

pub(super) struct ArrayIdents(pub(super) bool);

impl<'ast> Visit<'ast> for ArrayIdents {
    fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
        self.0 |= is_array_ident(&i.to_string());
    }
}

pub(super) fn strip_type(p: &syn::Pat) -> &syn::Pat {
    match p {
        syn::Pat::Type(t) => &t.pat,
        other => other,
    }
}

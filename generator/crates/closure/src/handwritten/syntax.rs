//! 手写体的语法推断：let 绑定 / 调用点实参与接收者的类型、静态类型作用域、构造与数组视图识别。

use std::collections::{HashMap, HashSet};

use syn::visit::Visit;

use super::*;
use super::generic_fns::*;
use super::stype::*;

pub(super) fn path_segs(p: &syn::Path) -> Vec<String> {
    p.segments.iter().map(|s| s.ident.to_string()).collect()
}

/// 表达式路径的段：限定路径 `<T as Trait>::f` 按 `T::f`（trait 关联函数在 T 上调用）
pub(super) fn expr_path_segs(p: &syn::ExprPath) -> Vec<String> {
    match (&p.qself, p.path.segments.last()) {
        (Some(q), Some(last)) => type_path(&q.ty).map(|mut t| {
            t.push(last.ident.to_string());
            t
        }).unwrap_or_default(),
        _ => path_segs(&p.path),
    }
}

/// 路径按 use 表展开首段
pub(super) fn expand(uses: &HashMap<String, Vec<String>>, segs: Vec<String>) -> Vec<String> {
    match segs.first().and_then(|f| uses.get(f)) {
        Some(full) => full.iter().cloned().chain(segs.into_iter().skip(1)).collect(),
        None => segs,
    }
}

/// 手写层的对象转换入口：结果类型由 turbofish 给出（`x.try_cast::<T>(…)` / `try_checkcast::<T>()` /
/// `catch_as::<T>(…)`，结果经 `Result` / `Option` 包装或直接为 T）
const CAST_METHODS: [&str; 3] = ["try_cast", "try_checkcast", "catch_as"];

/// 转换调用的目标类型路径（turbofish 单个类型实参）
pub(super) fn cast_target(m: &syn::ExprMethodCall) -> Option<Vec<String>> {
    if !CAST_METHODS.contains(&m.method.to_string().as_str()) {
        return None;
    }
    match m.turbofish.as_ref()?.args.iter().collect::<Vec<_>>().as_slice() {
        [syn::GenericArgument::Type(t)] => type_path(t),
        _ => None,
    }
}

/// 模式绑定的变量：`x` / `x: T` / `Ok(x)` / `Some(x)`（包装内的值与被匹配表达式的推断类型相同——
/// 推断对 `Result` / `Option` 透明）
pub(super) fn bound_ident(p: &syn::Pat) -> Option<&syn::PatIdent> {
    match strip_type(p) {
        syn::Pat::Ident(pi) if pi.subpat.is_none() => Some(pi),
        syn::Pat::TupleStruct(ts) if ts.elems.len() == 1 && ts.path.segments.last().is_some_and(|s| s.ident == "Ok" || s.ident == "Some") => {
            bound_ident(&ts.elems[0])
        }
        _ => None,
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
    /// 至少一处不以 `self` 为接收者调用 / 引用的名字（`calls` 中其余名字只经 `self.f(…)` 调用，被调 fn 的 `self` 即本 fn 的 `self`）
    pub(super) nonself: HashSet<String>,
    /// 本文件构造器名形态的辅助 fn（见 [`local_helpers`]）
    pub(super) helpers: HashSet<String>,
}

impl<'ast> Visit<'ast> for BodyScan {
    fn visit_local(&mut self, l: &'ast syn::Local) {
        if let Some(pi) = bound_ident(&l.pat) {
            let t = l.init.as_ref().and_then(|i| infer(&i.expr, &self.locals, &self.helpers));
            bind(&mut self.locals, pi.ident.to_string(), t);
            if pi.mutability.is_some() && matches!(strip_type(&l.pat), syn::Pat::Ident(_)) {
                if let Some(init) = &l.init {
                    if let syn::Expr::Call(c) = &*init.expr {
                        if let syn::Expr::Path(p) = &*c.func {
                            let segs = expr_path_segs(p);
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
        if !is_self_path(&m.receiver) {
            self.nonself.insert(name.clone());
        }
        self.calls.insert(name);
        syn::visit::visit_expr_method_call(self, m);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let segs = expr_path_segs(p);
            if let Some(last) = segs.last() {
                self.calls.insert(last.clone());
                self.nonself.insert(last.clone());
                let head_is_type = segs.len() >= 2 && segs[segs.len() - 2].starts_with(|ch: char| ch.is_ascii_uppercase());
                let is_ctor = is_ctor_call(&segs[..segs.len() - 1], last, &self.helpers);
                if is_ctor && head_is_type {
                    self.ctors.push((segs[..segs.len() - 1].to_vec(), last.clone()));
                }
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    // `if let` / `while let` 的模式绑定
    fn visit_expr_let(&mut self, e: &'ast syn::ExprLet) {
        if let Some(pi) = bound_ident(&e.pat) {
            let t = infer(&e.expr, &self.locals, &self.helpers);
            bind(&mut self.locals, pi.ident.to_string(), t);
        }
        syn::visit::visit_expr_let(self, e);
    }

    // match 分支的模式绑定
    fn visit_expr_match(&mut self, e: &'ast syn::ExprMatch) {
        for a in &e.arms {
            if let Some(pi) = bound_ident(&a.pat) {
                let t = infer(&e.expr, &self.locals, &self.helpers);
                bind(&mut self.locals, pi.ident.to_string(), t);
            }
        }
        syn::visit::visit_expr_match(self, e);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        visit_macro_body(self, m);
    }

    // 作值的 fn 路径（`.map_err(wrap)` 的 fn 指针）：同文件 fn 按被调处理（传递闭包只认本文件 fn 名）
    fn visit_expr_path(&mut self, p: &'ast syn::ExprPath) {
        if let Some(last) = path_segs(&p.path).last() {
            self.calls.insert(last.clone());
            self.nonself.insert(last.clone());
        }
        syn::visit::visit_expr_path(self, p);
    }

    // 嵌套 fn 单独登记（由外层 FileScan 处理）
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {}
}

/// 宏体按 Rust 语法访问：逗号分隔表达式（`format!` / `write!` / `assert!` 等），否则语句序列
/// （块状宏、在宏内声明 static 并以初始化表达式构造值的宏）。都解析不了的宏体只经标识符保守处理
pub(super) fn visit_macro_body<'a, V: for<'ast> Visit<'ast>>(v: &mut V, m: &'a syn::Macro) {
    if let Ok(args) = m.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated) {
        for a in &args {
            v.visit_expr(a);
        }
        return;
    }
    if let Ok(stmts) = m.parse_body_with(syn::Block::parse_within) {
        for st in &stmts {
            v.visit_stmt(st);
        }
    }
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

/// 表达式是 `self` 本身
pub(super) fn is_self_path(e: &syn::Expr) -> bool {
    matches!(e, syn::Expr::Path(p) if p.path.is_ident("self"))
}

/// 表达式的值是 `self` 所指对象：`self` 经保持对象身份的转换（括号 / 引用 / `?` / `Clone::clone` /
/// `Object::from` / `T::from` / `.clone()` 等，同 [`infer`] 的身份规则）
pub(super) fn is_self_value(e: &syn::Expr) -> bool {
    use syn::Expr;
    match e {
        Expr::Paren(p) => is_self_value(&p.expr),
        Expr::Group(g) => is_self_value(&g.expr),
        Expr::Reference(r) => is_self_value(&r.expr),
        Expr::Try(t) => is_self_value(&t.expr),
        Expr::Unary(u) if matches!(u.op, syn::UnOp::Deref(_)) => is_self_value(&u.expr),
        Expr::Path(_) => is_self_path(e),
        Expr::Call(c) => {
            let Expr::Path(p) = &*c.func else { return false };
            let segs = expr_path_segs(p);
            let Some((last, head)) = segs.split_last() else { return false };
            let identity = (head == ["Clone"] && last == "clone") || (head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase())) && last == "from");
            identity && c.args.len() == 1 && c.args.first().is_some_and(is_self_value)
        }
        Expr::MethodCall(m) => {
            matches!(m.method.to_string().as_str(), "clone" | "into" | "unwrap" | "expect") && is_self_value(&m.receiver)
        }
        _ => false,
    }
}

/// 表达式的对象类型（语法推断；见 [`TypedCall`]）
pub(super) fn infer(e: &syn::Expr, locals: &HashMap<String, Option<Vec<String>>>, helpers: &HashSet<String>) -> Option<Vec<String>> {
    use syn::Expr;
    match e {
        Expr::Paren(p) => infer(&p.expr, locals, helpers),
        Expr::Group(g) => infer(&g.expr, locals, helpers),
        Expr::Reference(r) => infer(&r.expr, locals, helpers),
        Expr::Try(t) => infer(&t.expr, locals, helpers),
        Expr::Lit(l) if matches!(l.lit, syn::Lit::Str(_)) => Some(vec![STRING_RUST.to_string()]),
        Expr::Path(p) => p.path.get_ident().and_then(|i| locals.get(&i.to_string()).cloned().flatten()),
        Expr::Call(c) => {
            let Expr::Path(p) = &*c.func else { return None };
            let segs = expr_path_segs(p);
            let (last, head) = segs.split_last()?;
            let arg0 = || c.args.first().and_then(|a| infer(a, locals, helpers));
            if head.last().is_some_and(|h| h == OBJECT_RUST) && last == "from" {
                return arg0();
            }
            if head == ["Clone"] && last == "clone" {
                return arg0();
            }
            let head_is_type = head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase()));
            if head_is_type && is_ctor_call(head, last, helpers) {
                return Some(head.to_vec());
            }
            if head_is_type && last == "from" {
                // 上转保持动态类型；非 Java 值（Rust 字符串等）→ 转换产出的 T
                return arg0().or_else(|| Some(head.to_vec()));
            }
            None
        }
        Expr::MethodCall(m) if cast_target(m).is_some() => cast_target(m),
        Expr::MethodCall(m) => match m.method.to_string().as_str() {
            // 无 turbofish 的转换：目标类型由上下文推出，此处取接收者的动态类型；
            // `unwrap_or_default` 的缺省分支是 Java null（引用的 Default），不添类型
            "clone" | "try_cast" | "into" | "unwrap" | "expect" | "unwrap_or_default" => infer(&m.receiver, locals, helpers),
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
    /// (字段, 写, 接收者静态类型, 写入值类型, 接收者是 self, static 写访问器路径调用, 写入值是 self)
    pub(super) fields: Vec<(String, bool, Option<SType>, Option<Vec<String>>, bool, bool, bool)>,
    pub(super) opaque: HashSet<String>,
    /// 本文件构造器名形态的辅助 fn（见 [`local_helpers`]）
    pub(super) helpers: &'a HashSet<String>,
    /// 本文件带闭包形参的泛型辅助 fn（见 [`generic_fns`]）
    pub(super) generics: &'a GenericFns,
    /// 当前 impl 块 self 类型末段（`Self::f` 按 `<末段>::f` 查泛型辅助 fn）
    pub(super) self_last: Option<String>,
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

/// 根类 vtable（`ObjectVTable`）的运行时内部入口 → 其分派到的 Java 方法名：`__to_string`（可失败形态）与
/// `__obj_str`（Rust 侧 Display / Debug）都是 `toString()` 的虚分派（rava_macros 把覆盖桥接到翻译体）
const ROOT_VTABLE_ALIASES: &[(&str, &str)] = &[("__to_string", JAVA_TO_STRING), ("__obj_str", JAVA_TO_STRING)];
/// Java 对象的 Display 即 `toString()`（`impl Display for Object` 经 `__obj_str`）
const JAVA_TO_STRING: &str = "toString";

/// 格式化宏（首参为含占位符的字符串字面量）里以 Display / Debug 输出的实参：显式实参与内联捕获 `{x}`
fn formatted_args(args: &syn::punctuated::Punctuated<syn::Expr, syn::Token![,]>) -> Option<(Vec<&syn::Expr>, Vec<String>)> {
    let pos = args.iter().position(|a| matches!(a, syn::Expr::Lit(l) if matches!(l.lit, syn::Lit::Str(_))))?;
    let syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(fmt), .. }) = &args[pos] else { return None };
    let fmt = fmt.value();
    if !fmt.contains('{') {
        return None;
    }
    let captured = fmt
        .split('{')
        .skip(1)
        .filter_map(|x| x.split(['}', ':']).next())
        .filter(|n| n.chars().next().is_some_and(|c| c.is_alphabetic() || c == '_') && n.chars().all(|c| c.is_alphanumeric() || c == '_'))
        .map(str::to_string)
        .collect();
    let explicit = args.iter().skip(pos + 1).filter(|a| !matches!(a, syn::Expr::Assign(_))).collect();
    Some((explicit, captured))
}

impl CallScan<'_> {
    /// 闭包形参：带类型注解的取注解类型（`|p: &UnixPath| …`），否则取调用点解出的类型（`known`，
    /// 见 [`GenericSig::closure_arg_types`]），其余遮蔽同名变量
    fn closure_with(&mut self, c: &syn::ExprClosure, known: &[Option<SType>]) {
        let outer = (self.scope.clone(), self.fresh.clone());
        for (k, input) in c.inputs.iter().enumerate() {
            self.visit_pat(input);
            let Some(pi) = bound_ident(input) else { continue };
            let st = match input {
                syn::Pat::Type(pt) => type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))),
                _ => known.get(k).cloned().flatten(),
            };
            if st.is_some() || matches!(input, syn::Pat::Type(_)) {
                self.scope.insert(pi.ident.to_string(), st);
            }
        }
        self.visit_return_type(&c.output);
        self.visit_expr(&c.body);
        (self.scope, self.fresh) = outer;
    }

    /// 静态类型已知的值以 Display 输出：即其 `toString()` 虚调用
    fn display_call(&mut self, recv: Option<Vec<String>>, st: Option<SType>) {
        if st.is_some() {
            self.calls.push((JAVA_TO_STRING.to_string(), None, Some(recv), vec![], None, st));
        }
    }
}

impl<'ast> Visit<'ast> for CallScan<'_> {
    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        if let Some((_, java)) = ROOT_VTABLE_ALIASES.iter().find(|(r, _)| m.method == r).filter(|_| m.args.is_empty()) {
            // 接收者静态类型推不出（`v.0` 等）→ 根类型（按 open 分派到全部覆盖）
            let st = stype(&m.receiver, &self.scope, self.locals).or_else(|| Some(SType::Named(TypeRef(vec![OBJECT_RUST.to_string()]))));
            self.calls.push((java.to_string(), None, Some(infer(&m.receiver, self.locals, self.helpers)), vec![], None, st));
            syn::visit::visit_expr_method_call(self, m);
            return;
        }
        let name = m.method.to_string();
        let access = match (name.strip_prefix(SET_PREFIX), name.strip_prefix(GET_PREFIX)) {
            (Some(f), _) if m.args.len() == 1 => Some((f, true)),
            (_, Some(f)) if m.args.is_empty() => Some((f, false)),
            _ => None,
        };
        if let Some((f, write)) = access {
            // Java 字段名是 Rust 关键字时访问器带 `_` 后缀（`in` → `__set_in_`）
            let f = java_field_name(f);
            let value = m.args.first().and_then(|a| infer(a, self.locals, self.helpers));
            let on_self = is_self_path(&m.receiver);
            let value_self = write && m.args.first().is_some_and(is_self_value);
            self.fields.push((f.to_string(), write, stype(&m.receiver, &self.scope, self.locals), value, on_self, false, value_self));
        }
        // 按名协议 `o.0.__unsafe_ref_set("字段", v)`：接收者是擦除的 vtable 对象，只知字段名
        let by_name = (BY_NAME_WRITES.contains(&name.as_str()), BY_NAME_READS.contains(&name.as_str()));
        if by_name.0 || by_name.1 {
            if let Some(syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(lit), .. })) = m.args.first() {
                let f = lit.value();
                let f = java_field_name(&f).to_string();
                let value = by_name.0.then(|| m.args.iter().nth(1).and_then(|a| infer(a, self.locals, self.helpers))).flatten();
                let value_self = by_name.0 && m.args.iter().nth(1).is_some_and(is_self_value);
                self.fields.push((f, by_name.0, None, value, false, false, value_self));
            }
        }
        let args = m.args.iter().map(|a| infer(a, self.locals, self.helpers)).collect();
        let fresh = match &*m.receiver {
            syn::Expr::Path(p) => p.path.get_ident().and_then(|i| self.fresh.get(&i.to_string()).cloned()),
            _ => None,
        };
        let srecv = stype(&m.receiver, &self.scope, self.locals);
        self.calls.push((name, None, Some(infer(&m.receiver, self.locals, self.helpers)), args, fresh, srecv));
        syn::visit::visit_expr_method_call(self, m);
    }

    fn visit_expr_call(&mut self, c: &'ast syn::ExprCall) {
        if let syn::Expr::Path(p) = &*c.func {
            let segs = expr_path_segs(p);
            if let Some((last, head)) = segs.split_last() {
                // static 字段写访问器 `T::set_<字段>(v)`（类型段首字母大写；关键字字段名带 `_` 后缀）
                if let Some(f) = last.strip_prefix(STATIC_SET_PREFIX).filter(|_| c.args.len() == 1) {
                    if head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase())) {
                        let f = java_field_name(f);
                        let value = c.args.first().and_then(|a| infer(a, self.locals, self.helpers));
                        let value_self = c.args.first().is_some_and(is_self_value);
                        self.fields.push((f.to_string(), true, Some(SType::Named(TypeRef(head.to_vec()))), value, false, true, value_self));
                    }
                }
                let args = c.args.iter().map(|a| infer(a, self.locals, self.helpers)).collect();
                let ty = (!head.is_empty()).then(|| head.to_vec());
                self.calls.push((last.clone(), ty, None, args, None, None));
            }
            // 本文件泛型辅助 fn：闭包实参的形参类型按其余实参解出
            let key = segs.iter().map(|s| if s == "Self" { self.self_last.as_deref().unwrap_or(s) } else { s }).collect::<Vec<_>>().join("::");
            if let Some(g) = self.generics.get(&key) {
                let args: Vec<&syn::Expr> = c.args.iter().collect();
                self.visit_expr(&c.func);
                for (i, a) in args.iter().enumerate() {
                    match (a, g.closure_arg_types(i, &args, &self.scope, self.locals)) {
                        (syn::Expr::Closure(cl), Some(types)) => self.closure_with(cl, &types),
                        _ => self.visit_expr(a),
                    }
                }
                return;
            }
        }
        syn::visit::visit_expr_call(self, c);
    }

    fn visit_macro(&mut self, m: &'ast syn::Macro) {
        macro_idents(m.tokens.clone(), &mut self.opaque);
        // 宏体可按 Rust 语法解析时按表达式 / 语句记调用点（见 [`visit_macro_body`]），
        // 标识符仍记入 opaque，实参来源推断对宏内同名调用保持保守
        visit_macro_body(self, m);
        if let Ok(args) = m.parse_body_with(syn::punctuated::Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated) {
            if let Some((explicit, captured)) = formatted_args(&args) {
                for a in explicit {
                    let st = stype(a, &self.scope, self.locals);
                    self.display_call(infer(a, self.locals, self.helpers), st);
                }
                for n in captured {
                    let st = self.scope.get(&n).cloned().flatten();
                    self.display_call(self.locals.get(&n).cloned().flatten(), st);
                }
            }
        }
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        let outer = (self.scope.clone(), self.fresh.clone());
        syn::visit::visit_block(self, b);
        (self.scope, self.fresh) = outer;
    }

    fn visit_expr_closure(&mut self, c: &'ast syn::ExprClosure) {
        self.closure_with(c, &[]);
    }

    // for / while let / if let / match 分支的模式绑定只在其内有效；`for x in <容器>` 的 x 取容器元素类型
    fn visit_expr_for_loop(&mut self, e: &'ast syn::ExprForLoop) {
        let outer = (self.scope.clone(), self.fresh.clone());
        self.visit_expr(&e.expr);
        self.visit_pat(&e.pat);
        if let syn::Pat::Ident(pi) = strip_type(&e.pat) {
            let el = elem_stype(&e.expr, &self.scope);
            self.scope.insert(pi.ident.to_string(), el);
        }
        self.visit_block(&e.body);
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

    // match 分支的 `Ok(x)` / `Some(x)` / `x` 绑定取被匹配表达式的静态类型（分支内有效）
    fn visit_expr_match(&mut self, e: &'ast syn::ExprMatch) {
        self.visit_expr(&e.expr);
        let st = stype(&e.expr, &self.scope, self.locals);
        for a in &e.arms {
            let outer = (self.scope.clone(), self.fresh.clone());
            self.visit_pat(&a.pat);
            if let Some(pi) = bound_ident(&a.pat) {
                self.scope.insert(pi.ident.to_string(), st.clone());
            }
            if let Some((_, g)) = &a.guard {
                self.visit_expr(g);
            }
            self.visit_expr(&a.body);
            (self.scope, self.fresh) = outer;
        }
    }

    // `if let` / `while let` 的绑定取被匹配表达式的静态类型（作用域由外层 if / while 恢复）
    fn visit_expr_let(&mut self, e: &'ast syn::ExprLet) {
        syn::visit::visit_expr_let(self, e);
        if let Some(pi) = bound_ident(&e.pat) {
            let st = stype(&e.expr, &self.scope, self.locals);
            self.scope.insert(pi.ident.to_string(), st);
        }
    }

    fn visit_local(&mut self, l: &'ast syn::Local) {
        syn::visit::visit_local(self, l);
        if let Some(pi) = bound_ident(&l.pat) {
            let st = match &l.pat {
                syn::Pat::Type(pt) => type_path(&pt.ty).map(|p| SType::Named(TypeRef(p))),
                _ => l.init.as_ref().and_then(|i| stype(&i.expr, &self.scope, self.locals)),
            };
            self.scope.insert(pi.ident.to_string(), st);
            let el = match &l.pat {
                syn::Pat::Type(pt) => elem_type(&pt.ty).map(|p| SType::Named(TypeRef(p))),
                _ => None,
            };
            self.scope.insert(elem_key(&pi.ident.to_string()), el);
            let name = pi.ident.to_string();
            let plain = matches!(strip_type(&l.pat), syn::Pat::Ident(_));
            match l.init.as_ref().filter(|i| plain && pi.mutability.is_none() && i.diverge.is_none()).and_then(|i| ctor_type(&i.expr, self.helpers)) {
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
        self.scope.remove(&elem_key(&p.ident.to_string()));
        self.fresh.remove(&p.ident.to_string());
        syn::visit::visit_pat_ident(self, p);
    }

    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {}
}

/// 构造调用 `T::new*(…)`（可带 `?` / 括号）的类型路径
pub(super) fn ctor_type(e: &syn::Expr, helpers: &HashSet<String>) -> Option<Vec<String>> {
    match e {
        syn::Expr::Paren(p) => ctor_type(&p.expr, helpers),
        syn::Expr::Group(g) => ctor_type(&g.expr, helpers),
        syn::Expr::Try(t) => ctor_type(&t.expr, helpers),
        syn::Expr::Call(c) => {
            let syn::Expr::Path(p) = &*c.func else { return None };
            let segs = expr_path_segs(p);
            let (last, head) = segs.split_last()?;
            let head_is_type = head.last().is_some_and(|h| h.starts_with(|ch: char| ch.is_ascii_uppercase()));
            (head_is_type && is_ctor_call(head, last, helpers)).then(|| head.to_vec())
        }
        _ => None,
    }
}

/// 手写体取得数组视图的标识符：数组类型本身、协变视图、Object 上的数组存取 / 转换（宏内按标识符保守判定）
pub(super) fn is_array_ident(s: &str) -> bool {
    s == ARRAY_TYPE || s == "__view_into" || s == ARRAY_CAST || is_store_ident(s)
}

/// Object 上改写引用元素的数组存取（`array_store_object` 等；只存基本元素的除外）
fn is_store_ident(s: &str) -> bool {
    s.starts_with("array_store") && !PRIMITIVE_STORES.contains(&s)
}

const ARRAY_TYPE: &str = "JArray";
const ARRAY_CAST: &str = "try_cast_array";
/// 只存取基本类型元素的 Object 数组存取（不改写引用元素）
const PRIMITIVE_STORES: &[&str] = &["array_store_byte"];
/// 数组视图上改写元素的方法（`JArray::set` / `__update` / `with_vec` 取可变切片）。接收者类型推不出，按方法名保守计
const ELEMENT_MUTATORS: &[&str] = &["set", "__update", "with_vec"];
/// Java 基本类型在运行时里的 Rust 元素类型
const PRIMITIVE_ELEMS: &[&str] = &["i8", "u16", "i16", "i32", "i64", "f32", "f64", "bool"];

/// 泛型实参恰为一个基本元素类型（`JArray<i8>`、`try_cast_array::<i32>`）
fn primitive_elem(args: Option<&syn::AngleBracketedGenericArguments>) -> bool {
    let Some(a) = args else { return false };
    let mut it = a.args.iter();
    match (it.next(), it.next()) {
        (Some(syn::GenericArgument::Type(syn::Type::Path(t))), None) => {
            t.qself.is_none() && t.path.get_ident().is_some_and(|i| PRIMITIVE_ELEMS.contains(&i.to_string().as_str()))
        }
        _ => false,
    }
}

/// 宏内标识符（不解析的 token 流）是否可能改写引用元素：取得数组视图且出现改写元素的方法名 / 存取
pub(super) fn opaque_array_writes(idents: &HashSet<String>) -> bool {
    idents.iter().any(|i| is_store_ident(i))
        || (idents.iter().any(|i| is_array_ident(i)) && idents.iter().any(|i| ELEMENT_MUTATORS.contains(&i.as_str())))
}

/// 手写体是否可能改写引用元素数组的元素（数组写入建模的前提）：取得引用元素数组视图、且调用了改写元素的
/// 方法（[`ELEMENT_MUTATORS`]），或经 Object 的引用元素存取写入。只读视图（`get` / `len` / `to_vec`）不改写元素。
/// 基本元素数组视图（`JArray<i8>` 等）的元素不携带类型，不计；元素类型未写明（裸 `JArray`、泛型、
/// 宏内标识符）按引用保守计
#[derive(Default)]
pub(super) struct ArrayIdents {
    /// 取得引用元素数组视图
    view: bool,
    /// 调用了改写元素的方法
    mutate: bool,
    /// 经 Object 的引用元素存取写入
    store: bool,
}

impl ArrayIdents {
    /// 体内可能改写引用元素；`opaque` 为宏内标识符：签名 / 体内取得的视图与宏内的改写方法名同样构成写入
    pub(super) fn writes(&self, opaque: &HashSet<String>) -> bool {
        self.store
            || (self.view && (self.mutate || opaque.iter().any(|i| ELEMENT_MUTATORS.contains(&i.as_str()))))
            || opaque_array_writes(opaque)
    }
}

impl<'ast> Visit<'ast> for ArrayIdents {
    fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
        let s = i.to_string();
        self.store |= is_store_ident(&s);
        self.view |= s == "__view_into";
    }

    fn visit_path_segment(&mut self, seg: &'ast syn::PathSegment) {
        if seg.ident == ARRAY_TYPE || seg.ident == ARRAY_CAST {
            let args = match &seg.arguments {
                syn::PathArguments::AngleBracketed(a) => Some(a),
                _ => None,
            };
            self.view |= !primitive_elem(args);
        }
        syn::visit::visit_path_segment(self, seg);
    }

    fn visit_expr_method_call(&mut self, m: &'ast syn::ExprMethodCall) {
        if m.method == ARRAY_CAST {
            self.view |= !primitive_elem(m.turbofish.as_ref());
        }
        self.mutate |= ELEMENT_MUTATORS.iter().any(|x| m.method == x);
        syn::visit::visit_expr_method_call(self, m);
    }
}

pub(super) fn strip_type(p: &syn::Pat) -> &syn::Pat {
    match p {
        syn::Pat::Type(t) => &t.pat,
        other => other,
    }
}

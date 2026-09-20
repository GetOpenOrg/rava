//! 方法体 token 重写：字段访问器重写、base 调用重写、vtable 虚调用重写、
//! Clone::clone(this) 替换。

use std::collections::HashSet;

use quote::format_ident;
use syn::{
    visit_mut::{self, VisitMut},
    Block, Expr, Ident, Member,
};

// ══════════════════════════════════════════════════════════════════════════════
// 方法体 token 重写（§8 的 7 种模式）
// ══════════════════════════════════════════════════════════════════════════════

pub(crate) struct Rewriter {
    pub basic: HashSet<String>,
    pub reference: HashSet<String>,
    pub aliases: HashSet<String>,
}

impl Rewriter {
    pub fn new(basic: &HashSet<String>, reference: &HashSet<String>) -> Self {
        Rewriter {
            basic: basic.clone(),
            reference: reference.clone(),
            aliases: ["self".to_owned(), "this".to_owned()].into_iter().collect(),
        }
    }

    fn as_field(&self, base: &Expr, member: &syn::Ident) -> Option<bool> {
        let Expr::Path(p) = base else { return None };
        if p.qself.is_some() {
            return None;
        }
        let id = p.path.get_ident()?;
        if !self.aliases.contains(&id.to_string()) {
            return None;
        }
        let m = member.to_string();
        if self.basic.contains(&m) {
            Some(true)
        } else if self.reference.contains(&m) {
            Some(false)
        } else {
            None
        }
    }

    fn rewrite_chain(&mut self, e: &mut Expr) {
        match e {
            Expr::Paren(p) => self.rewrite_chain(&mut p.expr),
            Expr::Group(g) => self.rewrite_chain(&mut g.expr),
            Expr::Field(fe) => {
                let named = match &fe.member {
                    Member::Named(id) => Some(id.clone()),
                    Member::Unnamed(_) => None,
                };
                if let Some(id) = named {
                    if let Some(is_basic) = self.as_field(&fe.base, &id) {
                        let base = (*fe.base).clone();
                        let name = id.to_string();
                        let call = if is_basic {
                            format_ident!("__get_{}", name)
                        } else {
                            format_ident!("__borrow_mut_{}", name)
                        };
                        *e = syn::parse_quote! { #base.#call() };
                        return;
                    }
                }
                self.rewrite_chain(&mut fe.base);
            }
            Expr::MethodCall(mc) => {
                self.rewrite_chain(&mut mc.receiver);
                for a in mc.args.iter_mut() {
                    self.visit_expr_mut(a);
                }
            }
            Expr::Index(ix) => {
                self.rewrite_chain(&mut ix.expr);
                self.visit_expr_mut(&mut ix.index);
            }
            Expr::Try(t) => self.rewrite_chain(&mut t.expr),
            Expr::Await(a) => self.rewrite_chain(&mut a.base),
            Expr::Reference(r) => self.rewrite_chain(&mut r.expr),
            Expr::Unary(u) => self.rewrite_chain(&mut u.expr),
            _ => visit_mut::visit_expr_mut(self, e),
        }
    }
}

fn direct_field(e: &Expr) -> Option<(&Expr, &syn::Ident)> {
    match e {
        Expr::Field(fe) => match &fe.member {
            Member::Named(id) => Some((&fe.base, id)),
            Member::Unnamed(_) => None,
        },
        Expr::Paren(p) => direct_field(&p.expr),
        Expr::Group(g) => direct_field(&g.expr),
        _ => None,
    }
}

fn assign_op_to_binop(op: &syn::BinOp) -> Option<proc_macro2::TokenStream> {
    use syn::BinOp::*;
    Some(match op {
        AddAssign(_) => quote::quote! { + },
        SubAssign(_) => quote::quote! { - },
        MulAssign(_) => quote::quote! { * },
        DivAssign(_) => quote::quote! { / },
        RemAssign(_) => quote::quote! { % },
        BitXorAssign(_) => quote::quote! { ^ },
        BitAndAssign(_) => quote::quote! { & },
        BitOrAssign(_) => quote::quote! { | },
        ShlAssign(_) => quote::quote! { << },
        ShrAssign(_) => quote::quote! { >> },
        _ => return None,
    })
}

impl VisitMut for Rewriter {
    fn visit_local_mut(&mut self, local: &mut syn::Local) {
        if let (syn::Pat::Ident(pi), Some(init)) = (&local.pat, &local.init) {
            if let Expr::Path(p) = &*init.expr {
                if p.qself.is_none() {
                    if let Some(id) = p.path.get_ident() {
                        if self.aliases.contains(&id.to_string()) {
                            self.aliases.insert(pi.ident.to_string());
                        }
                    }
                }
            }
        }
        visit_mut::visit_local_mut(self, local);
    }

    fn visit_expr_mut(&mut self, e: &mut Expr) {
        match e {
            Expr::Assign(a) => {
                if let Some((base, member)) = direct_field(&a.left) {
                    if self.as_field(base, member).is_some() {
                        let base = base.clone();
                        let setter = format_ident!("__set_{}", member);
                        let mut rhs = (*a.right).clone();
                        self.visit_expr_mut(&mut rhs);
                        *e = syn::parse_quote! { #base.#setter(#rhs) };
                        return;
                    }
                }
                let left = &mut a.left;
                let right = &mut a.right;
                self.rewrite_chain(left);
                self.visit_expr_mut(right);
            }

            Expr::Binary(b) => {
                if let Some(binop) = assign_op_to_binop(&b.op) {
                    if let Some((base, member)) = direct_field(&b.left) {
                        if let Some(true) = self.as_field(base, member) {
                            let base = base.clone();
                            let getter = format_ident!("__get_{}", member);
                            let setter = format_ident!("__set_{}", member);
                            let mut rhs = (*b.right).clone();
                            self.visit_expr_mut(&mut rhs);
                            *e = syn::parse_quote! { #base.#setter(#base.#getter() #binop #rhs) };
                            return;
                        }
                    }
                    let left = &mut b.left;
                    let right = &mut b.right;
                    self.rewrite_chain(left);
                    self.visit_expr_mut(right);
                    return;
                }
                visit_mut::visit_expr_mut(self, e);
            }

            Expr::MethodCall(_) | Expr::Index(_) => {
                self.rewrite_chain(e);
            }

            Expr::Field(_) => {
                if let Some((base, member)) = direct_field(e) {
                    if self.as_field(base, member).is_some() {
                        let base = base.clone();
                        let getter = format_ident!("__get_{}", member);
                        *e = syn::parse_quote! { #base.#getter() };
                        return;
                    }
                }
                self.rewrite_chain(e);
            }

            _ => visit_mut::visit_expr_mut(self, e),
        }
    }
}

pub(crate) fn rewrite_block(block: &mut Block, basic: &HashSet<String>, reference: &HashSet<String>) {
    let mut prober = Rewriter::new(basic, reference);
    prober.visit_block_mut(block);
    let aliases = prober.aliases;

    let mut rw = Rewriter::new(basic, reference);
    rw.aliases = aliases;
    rw.visit_block_mut(block);
}

/// 在 wrapper 上下文中，将 `ClassName__method_base(this, ...)` 改写为
/// `ClassName__method_base(&*this.vtable, ...)`。
pub(crate) fn rewrite_base_calls_for_wrapper(block: &mut Block) {
    struct BaseCallRewriter;
    impl VisitMut for BaseCallRewriter {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::Call(call) = expr {
                let is_base_fn = if let Expr::Path(p) = &*call.func {
                    // 单段路径，可带 turbofish（`Owner__m_base::<A, _>(this, ..)`）
                    p.qself.is_none() && p.path.segments.len() == 1 && {
                        let s = p.path.segments[0].ident.to_string();
                        s.contains("__") && s.ends_with("_base")
                    }
                } else {
                    false
                };
                if is_base_fn {
                    if let Some(first_arg) = call.args.first_mut() {
                        let is_this_or_self = matches!(first_arg, Expr::Path(p)
                            if p.path.get_ident().map_or(false, |id| id == "this" || id == "self"));
                        if is_this_or_self {
                            let old = first_arg.clone();
                            *first_arg = syn::parse_quote!(&* #old .vtable);
                        }
                    }
                }
            }
        }
    }
    BaseCallRewriter.visit_block_mut(block);
}

/// 在 wrapper impl 的 NeedsWrapper body 中，将未在当前类自有方法集合里的 `this.method(args)`
/// 改写为 `(&*this.vtable).method(args)`。
/// `own_method_names`：当前类所有已声明方法名（VirtualDefine + VirtualOverride + NonVirtual）。
pub(crate) fn rewrite_virtual_calls_for_wrapper(block: &mut Block, own_method_names: &HashSet<String>) {
    struct VirtualCallRewriter<'a>(&'a HashSet<String>);
    impl VisitMut for VirtualCallRewriter<'_> {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::MethodCall(mc) = expr {
                let recv_is_this = matches!(&*mc.receiver, Expr::Path(p)
                    if p.path.get_ident().map_or(false, |id| id == "this"));
                if recv_is_this {
                    let mname = mc.method.to_string();
                    if !mname.starts_with("__") && !self.0.contains(&mname) {
                        mc.receiver = Box::new(syn::parse_quote!(&*this.vtable));
                    }
                }
            }
        }
    }
    VirtualCallRewriter(own_method_names).visit_block_mut(block);
}

/// 在 base 函数体（`this: &__BT: VTable + ?Sized`）中，将 `this.vtable_method(args)` 改写为
/// `VTableTrait::vtable_method(this, args)` UFCS，消除同名方法多 supertrait 来源的 E0034 歧义。
/// `vtable_names`：当前类的 VirtualDefine 方法名集合（即 VTable trait 中声明的方法）。
pub(crate) fn rewrite_vtable_calls_ufcs_for_base(
    block: &mut Block,
    vtable_names: &HashSet<String>,
    vtable_trait: &Ident,
) {
    struct UfcsRewriter<'a>(&'a HashSet<String>, &'a Ident);
    impl VisitMut for UfcsRewriter<'_> {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::MethodCall(mc) = expr {
                let recv_is_this = matches!(&*mc.receiver, Expr::Path(p)
                    if p.path.get_ident().map_or(false, |id| id == "this"));
                if recv_is_this {
                    let mname_str = mc.method.to_string();
                    if !mname_str.starts_with("__") && self.0.contains(&mname_str) {
                        // this.method(a, b) → VTable::method(this, a, b)
                        let vtable_ident = self.1;
                        let method_ident = &mc.method;
                        let receiver = &mc.receiver;
                        let orig_args: Vec<&Expr> = mc.args.iter().collect();
                        *expr = syn::parse_quote! {
                            #vtable_ident::#method_ident(#receiver, #(#orig_args),*)
                        };
                    }
                }
            }
        }
    }
    UfcsRewriter(vtable_names, vtable_trait).visit_block_mut(block);
}

/// 将方法体中 `Ok(Clone::clone(this))` 替换为 `Ok(Default::default())`，
/// 使 NeedsWrapper 体能在 `&(impl VTable + ?Sized)` 的 base 函数上下文中运行。
pub(crate) fn replace_clone_this_in_ok(block: &mut Block) {
    struct CloneThisReplacer;
    impl VisitMut for CloneThisReplacer {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::Call(outer) = expr {
                let outer_is_ok = if let Expr::Path(p) = &*outer.func {
                    p.path.get_ident().map_or(false, |id| id == "Ok")
                } else { false };
                if !outer_is_ok || outer.args.len() != 1 { return; }

                if let Expr::Call(inner) = &outer.args[0] {
                    let func_is_clone = match &*inner.func {
                        Expr::Path(p) => {
                            let segs: Vec<_> = p.path.segments.iter()
                                .map(|s| s.ident.to_string())
                                .collect();
                            segs == ["Clone", "clone"]
                        }
                        _ => false,
                    };
                    let arg_is_bare_this = inner.args.len() == 1 &&
                        matches!(&inner.args[0], Expr::Path(p)
                            if p.path.get_ident().map_or(false, |id| id == "this"));
                    if func_is_clone && arg_is_bare_this {
                        *expr = syn::parse_quote!(
                            ::std::result::Result::Ok(::std::default::Default::default())
                        );
                    }
                }
            }
        }
    }
    CloneThisReplacer.visit_block_mut(block);
}

/// A-1 存储层擦除：祖先 vtable impl 删减了未出现在祖先实参里的类型形参
/// （filtered_anc_impl_header），但继承成员的转发体（Python 生成）在 base 调用的
/// turbofish / 钩子类型实参里可能引用它们。把转发体中这些形参替换为 `Object`——
/// `impl<P..> Owner__VTable<args(P..)> for X__inner` 覆盖全部实例化，P 全取 Object
/// 恒可满足；base 函数体是参数化的，行为一致。
pub(crate) fn rewrite_dropped_params_in_inherited_body(
    block: &mut Block,
    dropped: &HashSet<String>,
) {
    struct DroppedRewriter<'a>(&'a HashSet<String>);
    impl VisitMut for DroppedRewriter<'_> {
        fn visit_expr_mut(&mut self, e: &mut Expr) {
            visit_mut::visit_expr_mut(self, e);
            // `Owner__m_base::<A, B, Self>(...)`：turbofish 实参里被删形参 → Object
            if let Expr::Call(call) = e {
                if let Expr::Path(p) = &mut *call.func {
                    if p.qself.is_none() && p.path.segments.len() == 1 {
                        let seg = &mut p.path.segments[0];
                        let is_base = seg.ident.to_string().ends_with("_base");
                        if is_base {
                            if let syn::PathArguments::AngleBracketed(ab) = &mut seg.arguments {
                                rewrite_dropped_in_args(&mut ab.args, self.0);
                            }
                        }
                    }
                }
            }
            // 手写形态 `<Self as Owner__VTable<A, B>>::__as_Owner(self).__impl_m(..)`：
            // 限定路径的类型实参同样处理
            if let Expr::MethodCall(_) = e {
                // 经 visit 已处理 qself 内部；此处在 token 层兜底处理 qself 泛型
            }
        }
        fn visit_type_path_mut(&mut self, tp: &mut syn::TypePath) {
            visit_mut::visit_type_path_mut(self, tp);
            if let Some(seg) = tp.path.segments.last_mut() {
                if seg.ident.to_string().ends_with("__VTable") {
                    if let syn::PathArguments::AngleBracketed(ab) = &mut seg.arguments {
                        rewrite_dropped_in_args(&mut ab.args, self.0);
                    }
                }
            }
        }
    }
    fn rewrite_dropped_in_args(
        args: &mut syn::punctuated::Punctuated<syn::GenericArgument, syn::token::Comma>,
        dropped: &HashSet<String>,
    ) {
        for a in args.iter_mut() {
            if let syn::GenericArgument::Type(syn::Type::Path(tp)) = a {
                if tp.qself.is_none() && tp.path.segments.len() == 1 {
                    if let Some(id) = tp.path.get_ident() {
                        if dropped.contains(&id.to_string()) {
                            *a = syn::parse_quote!(Object);
                        }
                    }
                }
            }
        }
    }
    DroppedRewriter(dropped).visit_block_mut(block);
}

//! `java_class!` 块级宏实现 — vtable 两指针架构。
//!
//! 展开产物：
//!   - `ClassName__VTable` trait（虚方法分派接口，含 default impl）
//!   - `ClassName__inner` 存储 struct（平铺字段，无 `_super` 嵌套）
//!   - `impl AncestorVTable for ClassName__inner`（字段访问器 + 覆盖方法）
//!   - `pub struct ClassName { vtable: Rc<dyn ClassName__VTable>, any: Rc<dyn Any> }`
//!   - `impl ObjectVTable for ClassName`（委托到 vtable，R-1 blanket 需要）
//!   - 字段访问器委托 + 虚方法委托 + 构造器（on wrapper）
//!   - `ClassName__methodName_base` 自由函数（super() 调用路由）
//!   - `From<ClassName> for DirectParent`（vtable trait upcasting）
//!   - `From<Object> for ClassName`（downcast 路径）
//!
//! ## virtual_in 属性
//!
//! codegen 在 `#[java_method(virtual_in = "RustClassName")]` 中携带：
//!   - 等于当前类名 → VirtualDefine（新虚方法，进 trait default impl）
//!   - 不等于当前类名 → VirtualOverride（覆盖祖先，进 `impl AncestorVTable for __inner`）
//!   - 缺失 → Constructor（`new`/`new_*` 前缀）或 NonVirtual

use std::collections::{HashMap, HashSet};

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    visit_mut::{self, VisitMut},
    Attribute, Block, Expr, GenericParam, Ident, Lit, Member, Signature, Token, Type, Visibility,
};

// ══════════════════════════════════════════════════════════════════════════════
// 输入解析
// ══════════════════════════════════════════════════════════════════════════════

struct FnItem {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    block: Option<Block>,
}

struct ClassInput {
    attrs: Vec<Attribute>,
    struct_ident: Ident,
    generics: syn::Generics,
    fields: Vec<(Ident, Type)>,
    fns: Vec<FnItem>,
}

impl Parse for ClassInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let _struct_vis: Visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let struct_ident: Ident = input.parse()?;
        let mut generics: syn::Generics = input.parse()?;
        if input.peek(Token![where]) {
            generics.where_clause = Some(input.parse()?);
        }

        let mut fields: Vec<(Ident, Type)> = Vec::new();
        if input.peek(syn::token::Brace) {
            let content;
            syn::braced!(content in input);
            while !content.is_empty() {
                let _field_attrs = content.call(Attribute::parse_outer)?;
                let _field_vis: Visibility = content.parse()?;
                let name: Ident = content.parse()?;
                let _colon: Token![:] = content.parse()?;
                let ty: Type = content.parse()?;
                fields.push((name, ty));
                if content.peek(Token![,]) {
                    let _: Token![,] = content.parse()?;
                }
            }
        } else {
            let _: Token![;] = input.parse()?;
        }

        let fns = if input.is_empty() {
            Vec::new()
        } else {
            let _mid_attrs = input.call(Attribute::parse_outer)?;
            let _: Token![impl] = input.parse()?;
            let _impl_generics: syn::Generics = input.parse()?;
            let _self_ty: Type = input.parse()?;
            if input.peek(Token![where]) {
                let _: syn::WhereClause = input.parse()?;
            }
            let body;
            syn::braced!(body in input);
            parse_impl_fns(&body)?
        };

        Ok(ClassInput { attrs, struct_ident, generics, fields, fns })
    }
}

fn parse_impl_fns(input: ParseStream) -> syn::Result<Vec<FnItem>> {
    let mut out = Vec::new();
    while !input.is_empty() {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        let sig: Signature = input.parse()?;
        let block = if input.peek(Token![;]) {
            let _: Token![;] = input.parse()?;
            None
        } else {
            Some(input.parse::<Block>()?)
        };
        out.push(FnItem { attrs, vis, sig, block });
    }
    Ok(out)
}

// ══════════════════════════════════════════════════════════════════════════════
// 类级属性
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Default)]
struct ClassMeta {
    binary_name: String,
    superclass: Option<Type>,
    superclass_fields: Vec<(Ident, Type)>,
    /// 线性超类链（从最深祖先到直接父类），Rust short names，不含 Object 和 self。
    /// 用于 vtable impl 生成。由 codegen attrs.py 的 all_superclasses 属性提供。
    all_superclasses: Vec<String>,
    /// 每个祖先自己声明的字段列表：{ancestor_rust_name → [field_names]}。
    /// 用于将字段 accessor 精确分发到对应祖先的 vtable impl。
    ancestor_fields_layout: HashMap<String, Vec<String>>,
    all_supertypes: Vec<String>,
    is_interface: bool,
    has_to_string_method: bool,
    has_hash_code_method: bool,
}

impl ClassMeta {
    fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
        let mut m = ClassMeta::default();
        for attr in attrs {
            let path = attr.path();
            if path.is_ident("binary_name") {
                m.binary_name = lit_str(attr)?;
            } else if path.is_ident("superclass") {
                let s = lit_str(attr)?;
                if !s.is_empty() && s != "Object" {
                    m.superclass = Some(syn::parse_str::<Type>(&s)?);
                }
            } else if path.is_ident("all_superclasses") {
                let s = lit_str(attr)?;
                m.all_superclasses =
                    s.split(';').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect();
            } else if path.is_ident("ancestor_fields_layout") {
                let s = lit_str(attr)?;
                // 格式：AncName:field1,field2;AncName2:field3
                for seg in s.split(';').filter(|x| !x.is_empty()) {
                    if let Some((anc, fields_str)) = seg.split_once(':') {
                        let fields: Vec<String> = fields_str
                            .split(',')
                            .filter(|x| !x.is_empty())
                            .map(|x| x.to_owned())
                            .collect();
                        m.ancestor_fields_layout.insert(anc.to_owned(), fields);
                    }
                }
            } else if path.is_ident("all_supertypes") {
                let s = lit_str(attr)?;
                m.all_supertypes =
                    s.split(';').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect();
            } else if path.is_ident("is_interface") {
                m.is_interface = lit_bool(attr)?;
            } else if path.is_ident("has_to_string_method") {
                m.has_to_string_method = lit_bool(attr)?;
            } else if path.is_ident("has_hash_code_method") {
                m.has_hash_code_method = lit_bool(attr)?;
            } else if path.is_ident("superclass_fields") {
                let mut items: Vec<(Ident, Type)> = Vec::new();
                attr.parse_nested_meta(|meta| {
                    let ident = meta
                        .path
                        .get_ident()
                        .ok_or_else(|| meta.error("superclass_fields 键必须是字段名"))?
                        .clone();
                    let _: Token![:] = meta.input.parse()?;
                    let ty: Type = meta.input.parse()?;
                    items.push((ident, ty));
                    Ok(())
                })?;
                m.superclass_fields = items;
            }
        }
        Ok(m)
    }
}

fn lit_str(attr: &Attribute) -> syn::Result<String> {
    if let syn::Meta::NameValue(nv) = &attr.meta {
        if let Expr::Lit(el) = &nv.value {
            if let Lit::Str(s) = &el.lit {
                return Ok(s.value());
            }
        }
    }
    Err(syn::Error::new_spanned(attr, "该属性需要字符串字面量值"))
}

fn lit_bool(attr: &Attribute) -> syn::Result<bool> {
    if let syn::Meta::NameValue(nv) = &attr.meta {
        if let Expr::Lit(el) = &nv.value {
            if let Lit::Bool(b) = &el.lit {
                return Ok(b.value);
            }
        }
    }
    Err(syn::Error::new_spanned(attr, "该属性需要布尔字面量值"))
}

// ══════════════════════════════════════════════════════════════════════════════
// 类型分类与属性工具
// ══════════════════════════════════════════════════════════════════════════════

const BASIC_TYPES: &[&str] = &[
    "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    "f32", "f64", "bool", "char",
];

fn is_basic(ty: &Type) -> bool {
    if let Type::Path(tp) = ty {
        if tp.qself.is_none() && tp.path.segments.len() == 1 {
            let seg = &tp.path.segments[0];
            if seg.arguments.is_empty() {
                return BASIC_TYPES.contains(&seg.ident.to_string().as_str());
            }
        }
    }
    false
}

const META_ATTRS: &[&str] = &[
    "descriptor",
    "generic_signature",
    "native",
    "readonly",
    "field_sig",
    "java_method",
    "java_native",
    "jvm_native",
];

fn strip_meta_attrs(attrs: &[Attribute]) -> Vec<&Attribute> {
    attrs
        .iter()
        .filter(|a| {
            let p = a.path();
            !META_ATTRS.iter().any(|n| p.is_ident(n))
        })
        .collect()
}

fn attr_str(attrs: &[Attribute], name: &str) -> Option<String> {
    for a in attrs {
        if a.path().is_ident(name) {
            if let Ok(s) = lit_str(a) {
                return Some(s);
            }
        }
        if let syn::Meta::List(ml) = &a.meta {
            let mut found: Option<String> = None;
            let _ = ml.parse_nested_meta(|m| {
                if m.path.is_ident(name) {
                    if let Ok(v) = m.value().and_then(|v| v.parse::<syn::LitStr>()) {
                        found = Some(v.value());
                    }
                } else if m.input.peek(Token![=]) {
                    let _ = m.value().and_then(|v| v.parse::<Expr>());
                }
                Ok(())
            });
            if found.is_some() {
                return found;
            }
        }
    }
    None
}

// ══════════════════════════════════════════════════════════════════════════════
// 方法分类
// ══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
enum MethodKind {
    /// 新定义的虚方法：进 trait default impl + 生成 base 自由函数
    VirtualDefine,
    /// 覆盖祖先虚方法：进 `impl AncestorVTable for __inner` + 生成 base 自由函数
    /// vtable_class = 祖先 Rust 类名（不带 __VTable 后缀）
    VirtualOverride { vtable_class: String },
    /// 构造器或静态方法：留在 wrapper impl block
    Constructor,
    /// 非虚实例方法：留在 wrapper impl block
    NonVirtual,
}

fn classify_method(attrs: &[Attribute], sig: &Signature, self_name: &str) -> MethodKind {
    let mname = sig.ident.to_string();
    if let Some(virtual_in) = attr_str(attrs, "virtual_in") {
        if virtual_in == self_name {
            MethodKind::VirtualDefine
        } else {
            MethodKind::VirtualOverride { vtable_class: virtual_in }
        }
    } else if mname == "new" || mname.starts_with("new_") || mname == "main" {
        // main 也在 wrapper impl block 中（静态入口）
        MethodKind::Constructor
    } else {
        MethodKind::NonVirtual
    }
}

// ══════════════════════════════════════════════════════════════════════════════
// 方法体 token 重写（§8 的 7 种模式）
// ══════════════════════════════════════════════════════════════════════════════

struct Rewriter {
    basic: HashSet<String>,
    reference: HashSet<String>,
    aliases: HashSet<String>,
}

impl Rewriter {
    fn new(basic: &HashSet<String>, reference: &HashSet<String>) -> Self {
        Rewriter {
            basic: basic.clone(),
            reference: reference.clone(),
            aliases: ["self".to_owned(), "this".to_owned()].into_iter().collect(),
        }
    }

    fn as_field(&self, base: &Expr, member: &Ident) -> Option<bool> {
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

fn direct_field(e: &Expr) -> Option<(&Expr, &Ident)> {
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

fn assign_op_to_binop(op: &syn::BinOp) -> Option<TokenStream2> {
    use syn::BinOp::*;
    Some(match op {
        AddAssign(_) => quote! { + },
        SubAssign(_) => quote! { - },
        MulAssign(_) => quote! { * },
        DivAssign(_) => quote! { / },
        RemAssign(_) => quote! { % },
        BitXorAssign(_) => quote! { ^ },
        BitAndAssign(_) => quote! { & },
        BitOrAssign(_) => quote! { | },
        ShlAssign(_) => quote! { << },
        ShrAssign(_) => quote! { >> },
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

fn rewrite_block(block: &mut Block, basic: &HashSet<String>, reference: &HashSet<String>) {
    let mut prober = Rewriter::new(basic, reference);
    prober.visit_block_mut(block);
    let aliases = prober.aliases;

    let mut rw = Rewriter::new(basic, reference);
    rw.aliases = aliases;
    rw.visit_block_mut(block);
}

/// Base function 中将 `self` 路径表达式替换为 `this`。
/// 在表达式中将 `Self::xxx` 替换为具体类名（如 `Character::xxx`）。
/// 用于 vtable default impl 和 base 自由函数。
struct SelfToConcrete {
    struct_name: Ident,
}
impl VisitMut for SelfToConcrete {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        if let Expr::Path(p) = e {
            if p.qself.is_none() && !p.path.segments.is_empty() {
                if p.path.segments[0].ident == "Self" {
                    p.path.segments[0].ident = self.struct_name.clone();
                }
            }
        }
        visit_mut::visit_expr_mut(self, e);
    }
}

/// 在 base 自由函数体中将 `self` → `this`，`Self` → 具体类名（如 `Character`）。
/// base 函数中不能使用 `self`/`Self` 关键字（自由函数，无 receiver）。
struct SelfToThis {
    struct_name: Ident,
}
impl VisitMut for SelfToThis {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        if let Expr::Path(p) = e {
            if p.qself.is_none() {
                if let Some(id) = p.path.get_ident() {
                    if id == "self" {
                        *e = syn::parse_quote! { this };
                        return;
                    }
                }
                // Self::XXX → StructName::XXX
                if !p.path.segments.is_empty() && p.path.segments[0].ident == "Self" {
                    p.path.segments[0].ident = self.struct_name.clone();
                    // 继续递归处理
                }
            }
        }
        visit_mut::visit_expr_mut(self, e);
    }
}

fn rewrite_block_for_base(block: &mut Block, basic: &HashSet<String>, reference: &HashSet<String>, struct_ident: &Ident) {
    rewrite_block(block, basic, reference);
    SelfToThis { struct_name: struct_ident.clone() }.visit_block_mut(block);
}

/// 在 wrapper 上下文中，将 `ClassName__method_base(this, ...)` 改写为
/// `ClassName__method_base(&*this.vtable, ...)`，使 wrapper（不实现 VTable trait）
/// 能满足 `&impl AncestorVTable` 约束。`this.vtable: Rc<dyn SelfVTable>` 通过
/// supertrait 链实现了所有祖先 vtable trait。
fn rewrite_base_calls_for_wrapper(block: &mut Block) {
    struct BaseCallRewriter;
    impl VisitMut for BaseCallRewriter {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::Call(call) = expr {
                let is_base_fn = if let Expr::Path(p) = &*call.func {
                    p.path.get_ident().map_or(false, |id| {
                        let s = id.to_string();
                        s.contains("__") && s.ends_with("_base")
                    })
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
/// 适用场景：继承的虚方法（如 Writer.write_str → BufferedWriter.write_str）
/// 不在当前类 wrapper struct 上，必须通过 vtable 访问。
/// `__` 前缀调用（字段 accessor）不重写。
///
/// `own_method_names`：当前类所有已声明方法名（VirtualDefine + VirtualOverride + NonVirtual）。
fn rewrite_virtual_calls_for_wrapper(block: &mut Block, own_method_names: &std::collections::HashSet<String>) {
    struct VirtualCallRewriter<'a>(&'a std::collections::HashSet<String>);
    impl VisitMut for VirtualCallRewriter<'_> {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::MethodCall(mc) = expr {
                let recv_is_this = matches!(&*mc.receiver, Expr::Path(p)
                    if p.path.get_ident().map_or(false, |id| id == "this"));
                if recv_is_this {
                    let mname = mc.method.to_string();
                    // 不在当前类且不是 __ 前缀 → 继承虚方法，走 vtable
                    if !mname.starts_with("__") && !self.0.contains(&mname) {
                        mc.receiver = Box::new(syn::parse_quote!(&*this.vtable));
                    }
                }
            }
        }
    }
    VirtualCallRewriter(own_method_names).visit_block_mut(block);
}

/// 将方法体中 `Ok(Clone::clone(this))` 替换为 `Ok(Default::default())`，
/// 使 NeedsWrapper 体能在 `&(impl VTable + ?Sized)` 的 base 函数上下文中运行。
/// base 函数的调用方（wrapper body）持有 this: &Wrapper，负责返回正确的 wrapper 值；
/// base 函数只需完成 mutations，返回值总是被调用方丢弃。
///
/// 使用 AST 精确匹配，避免 proc_macro2 token string 空格不一致问题。
fn replace_clone_this_in_ok(block: &mut Block) {
    struct CloneThisReplacer;
    impl VisitMut for CloneThisReplacer {
        fn visit_expr_mut(&mut self, expr: &mut Expr) {
            visit_mut::visit_expr_mut(self, expr);
            if let Expr::Call(outer) = expr {
                // 外层：Ok(...)
                let outer_is_ok = if let Expr::Path(p) = &*outer.func {
                    p.path.get_ident().map_or(false, |id| id == "Ok")
                } else { false };
                if !outer_is_ok || outer.args.len() != 1 { return; }

                // 内层：Clone::clone(this)
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

/// 判断方法体是否可以安全放入 vtable 上下文（__inner impl）。
/// vtable 上下文中 self 是 &dyn Trait，禁止出现：
///   - Self:: 引用（Self 在 trait 方法中指实现类型，但 __inner 方法中是 __inner 而非 wrapper）
///   - Clone::clone（可能依赖 wrapper 类型的 Clone impl）
///   - this.non_accessor_method()（调用 wrapper 上的方法，__inner 无法访问）
/// VirtualDefine 方法体的 vtable 处理方式
enum VTableBodyKind {
    Safe,         // 直接放入 vtable impl
    NeedsWrapper, // 需要 wrapper 重建（含 Clone::clone，this 需要是 wrapper）
    Skip,         // 跳过（含 Self::，wrapper 重建也无法解决）
}

fn classify_vtable_body(block: &Block) -> VTableBodyKind {
    let s = quote!(#block).to_string();
    if s.contains("Self ::") || s.contains("Self::") {
        return VTableBodyKind::Skip;
    }
    // Clone::clone(this) 无 & 前缀：需要 wrapper 重建
    let clone_bare_this = s.contains("Clone :: clone (this)")
        || s.contains("Clone :: clone(this)")
        || s.contains("Clone::clone (this)")
        || s.contains("Clone::clone(this)");
    if clone_bare_this {
        return VTableBodyKind::NeedsWrapper;
    }
    // this.method() 调用 non-__ 方法：可能是 native 方法（仅在 wrapper 上有实现），
    // 不保证能从 __inner vtable 上下文访问。统一用 wrapper 重建，保证 this: &Wrapper。
    let mut parts: Vec<&str> = Vec::new();
    parts.extend(s.split("this .").skip(1));
    parts.extend(s.split("this.").skip(1));
    for part in parts {
        let trimmed = part.trim_start();
        let mname: String =
            trimmed.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect();
        if !mname.is_empty() && !mname.starts_with("__") {
            let rest = &trimmed[mname.len()..];
            if rest.trim_start().starts_with('(') {
                return VTableBodyKind::NeedsWrapper;
            }
        }
    }
    VTableBodyKind::Safe
}

/// VirtualOverride 安全性检查：委托给 classify_vtable_body（this.method() 检查已整合）。
fn is_vtable_safe_body(block: &Block) -> bool {
    matches!(classify_vtable_body(block), VTableBodyKind::Safe)
}

// ══════════════════════════════════════════════════════════════════════════════
// JVM generic_signature 解析 → Rust 泛型类型
// ══════════════════════════════════════════════════════════════════════════════

/// 解析 JVM generic method signature，提取参数和返回类型。
/// 输入：`"(TK;TV;)TV;"` 或 `"(Ljava/util/List<TE;>;)V"`
/// 返回：`(params, return_type)` 每个元素是 Rust 类型字符串（`"K"`, `"V"`, `"Object"` 等）
fn parse_generic_method_sig(sig: &str) -> Option<(Vec<String>, String)> {
    if !sig.starts_with('(') { return None; }
    let close = sig.find(')')?;
    let params_str = &sig[1..close];
    let ret_str = &sig[close + 1..];
    let params = parse_jvm_type_sequence(params_str);
    let ret = parse_jvm_single_type(ret_str).unwrap_or_else(|| "()".to_owned());
    Some((params, ret))
}

fn parse_jvm_type_sequence(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut pos = 0;
    while pos < s.len() {
        let (ty, consumed) = parse_jvm_type_at(s, pos);
        if consumed == 0 { break; }
        result.push(ty);
        pos += consumed;
    }
    result
}

fn parse_jvm_single_type(s: &str) -> Option<String> {
    if s.is_empty() { return None; }
    let (ty, _) = parse_jvm_type_at(s, 0);
    Some(ty)
}

/// 在位置 pos 解析一个 JVM 泛型类型描述符，返回 (Rust 类型字符串, 消耗字节数)。
fn parse_jvm_type_at(s: &str, pos: usize) -> (String, usize) {
    let bytes = s.as_bytes();
    if pos >= bytes.len() { return ("Object".to_owned(), 0); }
    match bytes[pos] {
        b'T' => {
            // 类型变量：TK; → K
            let end = s[pos + 1..].find(';').map(|i| pos + 1 + i).unwrap_or(s.len());
            (s[pos + 1..end].to_owned(), end - pos + 1)
        }
        b'L' => {
            // 引用类型：Ljava/lang/String; 或 Ljava/util/List<TE;>;
            let mut depth = 0usize;
            let mut i = pos + 1;
            while i < bytes.len() {
                match bytes[i] {
                    b'<' => depth += 1,
                    b'>' => { if depth > 0 { depth -= 1; } }
                    b';' if depth == 0 => { i += 1; break; }
                    _ => {}
                }
                i += 1;
            }
            let inner = &s[pos + 1..i.saturating_sub(1)];
            let simple = inner.split('/').last().unwrap_or(inner);
            let base = simple.split('<').next().unwrap_or(simple);
            let rust_ty = match base {
                "String" => "String", "Object" => "Object",
                "Integer" => "i32", "Long" => "i64", "Boolean" => "bool",
                "Void" => "()", other => other,
            };
            (rust_ty.to_owned(), i - pos)
        }
        b'[' => {
            let (inner, consumed) = parse_jvm_type_at(s, pos + 1);
            (format!("Vec<{}>", inner), consumed + 1)
        }
        b'V' => ("()".to_owned(), 1),
        b'Z' => ("bool".to_owned(), 1),
        b'I' => ("i32".to_owned(), 1),
        b'J' => ("i64".to_owned(), 1),
        b'D' => ("f64".to_owned(), 1),
        b'F' => ("f32".to_owned(), 1),
        b'C' => ("char".to_owned(), 1),
        b'B' => ("i8".to_owned(), 1),
        b'S' => ("i16".to_owned(), 1),
        _ => ("Object".to_owned(), 1),
    }
}

/// 用 generic_signature 的类型变量重建方法的 Rust Signature。
/// 只替换 descriptor 擦除为 Object 的位置中匹配 class_type_params 的类型变量。
/// 返回 None 表示无需替换。
fn rebuild_sig_with_generics(
    sig: &syn::Signature,
    generic_sig_str: &str,
    class_type_params: &[String],
) -> Option<syn::Signature> {
    let (g_params, g_ret) = parse_generic_method_sig(generic_sig_str)?;
    let has_type_var = g_params.iter().any(|p| class_type_params.contains(p))
        || class_type_params.contains(&g_ret);
    if !has_type_var { return None; }

    let mut new_sig = sig.clone();

    // 重建参数（跳过 &self）
    let mut g_iter = g_params.iter();
    for arg in new_sig.inputs.iter_mut() {
        if let syn::FnArg::Typed(pt) = arg {
            if let Some(g_ty) = g_iter.next() {
                if class_type_params.contains(g_ty) {
                    if let Ok(ty) = syn::parse_str::<syn::Type>(g_ty) {
                        *pt.ty = ty;
                    }
                }
            }
        }
    }

    // 重建返回类型
    if class_type_params.contains(&g_ret) {
        if let Ok(ty) = syn::parse_str::<syn::Type>(&format!("crate::error::Result<{}>", g_ret)) {
            new_sig.output = syn::ReturnType::Type(Default::default(), Box::new(ty));
        }
    }

    Some(new_sig)
}

// ══════════════════════════════════════════════════════════════════════════════
// all_supertypes 工具
// ══════════════════════════════════════════════════════════════════════════════

// ══════════════════════════════════════════════════════════════════════════════
// 展开入口
// ══════════════════════════════════════════════════════════════════════════════

pub fn expand(input: TokenStream2) -> TokenStream2 {
    match syn::parse2::<ClassInput>(input) {
        Ok(v) => expand_inner(v),
        Err(e) => e.to_compile_error(),
    }
}

fn expand_inner(input: ClassInput) -> TokenStream2 {
    let ClassInput { attrs, struct_ident, generics, fields, fns } = input;

    let meta = match ClassMeta::from_attrs(&attrs) {
        Ok(m) => m,
        Err(e) => return e.to_compile_error(),
    };

    // ── 接口：Arch-1 语义 ────────────────────────────────────────────────────
    if meta.is_interface {
        return quote! { pub type #struct_ident = Object; };
    }

    // ── 泛型参数补齐 Clone + Default + 'static ──────────────────────────────
    let mut gen = generics.clone();
    for param in &mut gen.params {
        if let GenericParam::Type(tp) = param {
            let mut has_clone = false;
            let mut has_default = false;
            let mut has_static = false;
            for b in &tp.bounds {
                match b {
                    syn::TypeParamBound::Trait(t) => {
                        if let Some(s) = t.path.segments.last() {
                            if s.ident == "Clone" {
                                has_clone = true;
                            } else if s.ident == "Default" {
                                has_default = true;
                            }
                        }
                    }
                    syn::TypeParamBound::Lifetime(l) => {
                        if l.ident == "static" {
                            has_static = true;
                        }
                    }
                    _ => {}
                }
            }
            if !has_clone {
                tp.bounds.push(syn::parse_quote!(Clone));
            }
            if !has_default {
                tp.bounds.push(syn::parse_quote!(Default));
            }
            if !has_static {
                tp.bounds.push(syn::parse_quote!('static));
            }
        }
    }
    let (impl_g, ty_g, where_c) = gen.split_for_impl();

    let self_name = struct_ident.to_string();
    let inner_ident = format_ident!("{}__inner", struct_ident);
    let vtable_trait_ident = format_ident!("{}__VTable", struct_ident);

    // 提取 superclass 的泛型参数（如 Enum<Object> → <Object>），供 vtable 继承使用
    // 注意：AngleBracketedGenericArguments 自带 <> 括号，quote! { #ab } 即为 <Object>
    let superclass_vtable_args: TokenStream2 = meta.superclass.as_ref().map_or(
        quote! {},
        |sup_ty| {
            if let syn::Type::Path(tp) = sup_ty {
                if let Some(seg) = tp.path.segments.last() {
                    if let syn::PathArguments::AngleBracketed(ab) = &seg.arguments {
                        if !ab.args.is_empty() {
                            return quote! { #ab };
                        }
                    }
                }
            }
            quote! {}
        },
    );

    // ── 字段集合 ─────────────────────────────────────────────────────────────
    let mut basic_names: HashSet<String> = HashSet::new();
    let mut ref_names: HashSet<String> = HashSet::new();
    for (name, ty) in fields.iter().chain(meta.superclass_fields.iter()) {
        if is_basic(ty) {
            basic_names.insert(name.to_string());
        } else {
            ref_names.insert(name.to_string());
        }
    }

    // ── 方法分类 ─────────────────────────────────────────────────────────────
    // vtable_defines:  VirtualDefine 方法
    // vtable_overrides: vtable_class → Vec<method>
    // non_virtual:     Constructor / NonVirtual 方法
    let mut vtable_defines: Vec<&FnItem> = Vec::new();
    let mut vtable_overrides: HashMap<String, Vec<&FnItem>> = HashMap::new();
    let mut non_virtual: Vec<&FnItem> = Vec::new();

    for f in &fns {
        match classify_method(&f.attrs, &f.sig, &self_name) {
            MethodKind::VirtualDefine => vtable_defines.push(f),
            MethodKind::VirtualOverride { vtable_class } => {
                vtable_overrides.entry(vtable_class).or_default().push(f);
            }
            MethodKind::Constructor | MethodKind::NonVirtual => non_virtual.push(f),
        }
    }

    // ── PhantomData 检测 ─────────────────────────────────────────────────────
    let mut used_words: HashSet<String> = HashSet::new();
    let mut type_texts: Vec<String> = Vec::new();
    for (_, ty) in fields.iter().chain(meta.superclass_fields.iter()) {
        type_texts.push(quote!(#ty).to_string());
    }
    for text in &type_texts {
        let mut cur = String::new();
        for ch in text.chars() {
            if ch.is_alphanumeric() || ch == '_' {
                cur.push(ch);
            } else if !cur.is_empty() {
                used_words.insert(std::mem::take(&mut cur));
            }
        }
        if !cur.is_empty() {
            used_words.insert(cur);
        }
    }
    let mut phantom_fields: Vec<TokenStream2> = Vec::new();
    for param in &gen.params {
        if let GenericParam::Type(tp) = param {
            if !used_words.contains(&tp.ident.to_string()) {
                let p = &tp.ident;
                phantom_fields.push(quote! { #p });
            }
        }
    }

    // 类级类型参数名（用于 generic_signature 泛型重建）
    let class_type_params: Vec<String> = gen.params.iter().filter_map(|p| {
        if let GenericParam::Type(tp) = p { Some(tp.ident.to_string()) } else { None }
    }).collect();

    // ══════════════════════════════════════════════════════════════════════════
    // 1. VTable trait
    // ══════════════════════════════════════════════════════════════════════════

    // supertrait：有父类 → 父类 __VTable（携带泛型参数）；无父类 → ObjectVTable
    let vtable_supertrait: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        // 提取超类的基础类型名（去掉泛型参数），如 Enum<Object> → Enum
        let base_name = if let Type::Path(tp) = sup_ty {
            tp.path.segments.last().map(|s| s.ident.to_string()).unwrap_or_default()
        } else {
            quote!(#sup_ty).to_string()
        };
        let sup_vtable = format_ident!("{}__VTable", base_name);
        quote! { #sup_vtable #superclass_vtable_args }
    } else {
        quote! { ObjectVTable }
    };

    // 字段 accessor 抽象方法（只有 own fields，不含继承字段）
    let mut vtable_abstract_methods: Vec<TokenStream2> = Vec::new();
    for (name, ty) in &fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> #ty;
                fn #set(&self, v: #ty);
            });
        } else {
            let borm = format_ident!("__borrow_mut_{}", name);
            vtable_abstract_methods.push(quote! {
                fn #get(&self) -> #ty;
                fn #set(&self, v: #ty);
                fn #borm(&self) -> ::std::cell::RefMut<'_, #ty>;
            });
        }
    }

    // VirtualDefine 的 default impl：
    // - 有方法体 → default 委托 ClassName__method_base 自由函数（base 函数含实际体或 stub）
    //   subclass 若未 override，走 base 函数路径（Safe → 实际实现；NeedsWrapper/Skip → stub panic）
    // - 无方法体（abstract）→ default 生成 stub（子类必须覆盖，未覆盖则运行时命中）
    // 注：调用 base 函数时用 turbofish ::<ClassTypeParams..., Self> 避免 E0282 类型推断失败
    let class_ty_idents: Vec<syn::Ident> = gen.params.iter()
        .filter_map(|p| if let syn::GenericParam::Type(tp) = p { Some(tp.ident.clone()) } else { None })
        .collect();
    let mut vtable_default_methods: Vec<TokenStream2> = Vec::new();
    for f in &vtable_defines {
        let mname = &f.sig.ident;
        let fn_base_name = format_ident!("{}__{}_base", self_name, mname);
        let non_self_params_for_default: Vec<_> = f.sig.inputs.iter()
            .filter(|a| matches!(a, syn::FnArg::Typed(_)))
            .collect();
        let param_names_for_default: Vec<syn::Ident> = non_self_params_for_default.iter()
            .filter_map(|a| {
                if let syn::FnArg::Typed(pt) = a {
                    if let syn::Pat::Ident(pi) = &*pt.pat { Some(pi.ident.clone()) }
                    else { None }
                } else { None }
            })
            .collect();
        let mname_str = f.sig.ident.to_string();
        let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
        let binary = &meta.binary_name;
        // generic_signature 重建：将 Object 参数/返回替换为类型变量（K, V 等）
        let effective_sig = attr_str(&f.attrs, "generic_signature")
            .and_then(|gs| rebuild_sig_with_generics(&f.sig, &gs, &class_type_params))
            .unwrap_or_else(|| f.sig.clone());
        if f.block.is_some() {
            // 有方法体 → default 委托 base 函数，subclass 不 override 时走 base 函数
            // turbofish 传类型参数（避免 E0282）：<ClassTypeParams..., Self>
            vtable_default_methods.push(quote! {
                #effective_sig { #fn_base_name::<#(#class_ty_idents,)* Self>(self, #(#param_names_for_default),*) }
            });
        } else {
            // 无方法体（abstract）→ stub，子类必须覆盖
            let msg = format!("stub: {}.{}:{}", binary, mname_str, desc);
            vtable_default_methods.push(quote! {
                #effective_sig { panic!(#msg) }
            });
        }
    }

    let vtable_trait = quote! {
        #[allow(non_camel_case_types)]
        pub trait #vtable_trait_ident #impl_g #where_c: #vtable_supertrait {
            #(#vtable_abstract_methods)*
            #(#vtable_default_methods)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 2. Inner struct（平铺字段：superclass_fields + own fields）
    // ══════════════════════════════════════════════════════════════════════════

    let mut inner_field_tokens: Vec<TokenStream2> = Vec::new();

    // 继承字段（平铺，不再有 _super）
    // 用 Rc<Cell<T>> / Rc<RefCell<...>> 而非裸 Cell/RefCell，保证 NeedsWrapper clone
    // 时共享同一个 Cell，mutations 对原始 inner struct 可见（否则 clone 是值拷贝，
    // __set_xxx 修改的是孤立副本，调用方看不到变化）。
    for (name, ty) in &meta.superclass_fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 自有字段
    for (name, ty) in &fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::rc::Rc<::std::cell::Cell<#ty>> }
        } else {
            quote! { ::std::rc::Rc<::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    if !phantom_fields.is_empty() {
        inner_field_tokens.push(quote! {
            pub(crate) __phantom: ( #( ::std::marker::PhantomData<fn() -> #phantom_fields>, )* )
        });
    }

    let inner_struct = quote! {
        #[doc(hidden)]
        #[derive(Clone, Default, PartialEq, Debug)]
        pub(crate) struct #inner_ident #impl_g #where_c {
            #(#inner_field_tokens,)*
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 3. impl ObjectVTable for __inner
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name = &meta.binary_name;
    let check_types: Vec<String> = if meta.all_supertypes.is_empty() {
        vec![binary_name.clone()]
    } else {
        meta.all_supertypes.clone()
    };
    let patterns = check_types.iter().map(|s| quote! { #s });

    let obj_vtable_for_inner = if !binary_name.is_empty() {
        quote! {
            impl #impl_g ObjectVTable for #inner_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    matches!(type_id, #(#patterns)|*)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 4. impl vtable traits for __inner
    // ══════════════════════════════════════════════════════════════════════════
    //
    // 策略：
    // a) 无父类：impl Self__VTable for __inner（own field accessors）
    // b) 有父类：
    //    - 找出"顶层祖先" vtable（all_supertypes 中第一个非 Object 非 self 用户类）
    //      → 放 superclass_fields 的所有 accessor + VirtualOverride 方法
    //    - 其余祖先 vtable：空 impl（满足 trait 层次要求）
    //    - Self__VTable：impl（own field accessors）
    //
    // 注意：对无父类类，own_fields accessor 放 Self__VTable impl；
    //       对有父类类，顶层祖先的 vtable impl 接管 ALL 继承字段 accessor。

    let mut vtable_impls: Vec<TokenStream2> = Vec::new();

    // 当前类自有方法名（VirtualDefine + VirtualOverride + NonVirtual）
    // 在 NeedsWrapper 路径中，不在此集合的 this.method() 调用均为继承虚方法，需通过 vtable 访问。
    let own_method_names: HashSet<String> = fns.iter()
        .map(|f| f.sig.ident.to_string())
        .collect();

    if meta.superclass.is_none() {
        // ── 无父类：impl Self__VTable for __inner ────────────────────────────
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &fields {
            let get = format_ident!("__get_{}", name);
            let set = format_ident!("__set_{}", name);
            if is_basic(ty) {
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty { self.#name.get() }
                    fn #set(&self, v: #ty) { self.#name.set(v); }
                });
            } else {
                let borm = format_ident!("__borrow_mut_{}", name);
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty {
                        self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                    }
                    fn #set(&self, v: #ty) {
                        *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                    }
                    fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                        ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                            opt.as_deref_mut().expect("field not initialized")
                        })
                    }
                });
            }
        }
        // VirtualDefine 方法体：Safe→直接放入 vtable impl；NeedsWrapper→wrapper 重建；Skip→跳过
        for f in &vtable_defines {
            if let Some(block) = &f.block {
                let sig = &f.sig;
                let keep_attrs = strip_meta_attrs(&f.attrs);
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                match classify_vtable_body(block) {
                    VTableBodyKind::Safe => {
                        own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                    }
                    VTableBodyKind::NeedsWrapper => {
                        if let Some(first) = b.stmts.first() {
                            let fs = quote!(#first).to_string();
                            if fs.contains("this") && fs.contains("self") {
                                b.stmts.remove(0);
                            }
                        }
                        rewrite_virtual_calls_for_wrapper(&mut b, &own_method_names);
                        let stmts = &b.stmts;
                        own_accessor_impls.push(quote! {
                            #(#keep_attrs)*
                            #sig {
                                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                                let __wrapper = #struct_ident {
                                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                                };
                                let this = &__wrapper;
                                #(#stmts)*
                            }
                        });
                    }
                    VTableBodyKind::Skip => {}  // 含 Self::，保留 vtable trait default stub
                }
            }
        }

        vtable_impls.push(quote! {
            impl #impl_g #vtable_trait_ident #ty_g for #inner_ident #ty_g #where_c {
                #(#own_accessor_impls)*
            }
        });
    } else {
        // ── 有父类：顶层祖先 vtable impl + 其余空 impl + Self__VTable impl ──

        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = meta.all_superclasses.clone();

        // 构建字段名 → (Ident, Type) 的查找表（superclass_fields 平铺列表）
        let sc_fields_map: HashMap<String, (Ident, Type)> = meta
            .superclass_fields
            .iter()
            .map(|(n, t)| (n.to_string(), (n.clone(), t.clone())))
            .collect();

        for anc_name in ancestors.iter() {
            let anc_vtable_ident = format_ident!("{}__VTable", anc_name);

            let mut items: Vec<TokenStream2> = Vec::new();

            // 将此祖先自己声明的字段 accessor 放进此 vtable impl
            let anc_own_field_names: Vec<String> =
                if let Some(names) = meta.ancestor_fields_layout.get(anc_name) {
                    names.clone()
                } else if meta.ancestor_fields_layout.is_empty() {
                    // 兜底：无 ancestor_fields_layout 时（旧属性），所有字段放最深祖先（首位）
                    if anc_name == ancestors.first().map(|s| s.as_str()).unwrap_or("") {
                        meta.superclass_fields.iter().map(|(n, _)| n.to_string()).collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

            for field_name in &anc_own_field_names {
                if let Some((name, ty)) = sc_fields_map.get(field_name) {
                    let get = format_ident!("__get_{}", name);
                    let set = format_ident!("__set_{}", name);
                    if is_basic(ty) {
                        items.push(quote! {
                            fn #get(&self) -> #ty { self.#name.get() }
                            fn #set(&self, v: #ty) { self.#name.set(v); }
                        });
                    } else {
                        let borm = format_ident!("__borrow_mut_{}", name);
                        items.push(quote! {
                            fn #get(&self) -> #ty {
                                self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                            }
                            fn #set(&self, v: #ty) {
                                *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                            }
                            fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                                ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                                    opt.as_deref_mut().expect("field not initialized")
                                })
                            }
                        });
                    }
                }
            }

            // VirtualOverride 方法：virtual_in == anc_name
            if let Some(override_fns) = vtable_overrides.get(anc_name) {
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &basic_names, &ref_names);
                            if is_vtable_safe_body(block) {
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #sig #b
                                });
                            } else {
                                // 非 vtable-safe 方法体（含 Clone::clone(this) 等）：
                                // 去除 codegen 生成的首行 `let this = self;`，
                                // 改为在 vtable impl 中重建 wrapper 并绑定为 this。
                                if let Some(first) = b.stmts.first() {
                                    let s = quote!(#first).to_string();
                                    if s.contains("this") && s.contains("self") {
                                        b.stmts.remove(0);
                                    }
                                }
                                let stmts = &b.stmts;
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #sig {
                                        let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                                        let __wrapper = #struct_ident {
                                            vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                                            any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                                        };
                                        let this = &__wrapper;
                                        #(#stmts)*
                                    }
                                });
                            }
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                            let bin = &meta.binary_name;
                            let msg = format!("stub: {}.{}:{}", bin, mname, desc);
                            items.push(quote! {
                                #sig { panic!(#msg) }
                            });
                        }
                    }
                }
            }

            // 祖先 vtable 的类型参数：若当前类有泛型用自己的 ty_g，否则用 superclass 的类型参数
            // 例：Thread_State（无泛型）实现 Enum__VTable<Object>，Object 来自 superclass="Enum<Object>"
            let anc_vtable_args: TokenStream2 = if gen.params.is_empty() {
                superclass_vtable_args.clone()
            } else {
                quote! { #ty_g }
            };

            vtable_impls.push(quote! {
                impl #impl_g #anc_vtable_ident #anc_vtable_args for #inner_ident #ty_g #where_c {
                    #(#items)*
                }
            });
        }

        // Self__VTable impl（own fields 的 accessor）
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &fields {
            let get = format_ident!("__get_{}", name);
            let set = format_ident!("__set_{}", name);
            if is_basic(ty) {
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty { self.#name.get() }
                    fn #set(&self, v: #ty) { self.#name.set(v); }
                });
            } else {
                let borm = format_ident!("__borrow_mut_{}", name);
                own_accessor_impls.push(quote! {
                    fn #get(&self) -> #ty {
                        self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                    }
                    fn #set(&self, v: #ty) {
                        *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                    }
                    fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> {
                        ::std::cell::RefMut::map(self.#name.borrow_mut(), |opt| {
                            opt.as_deref_mut().expect("field not initialized")
                        })
                    }
                });
            }
        }
        // VirtualDefine 方法体（有父类路径）：Safe→直接放，NeedsWrapper→wrapper 重建，Skip→跳过
        for f in &vtable_defines {
            if let Some(block) = &f.block {
                let sig = &f.sig;
                let keep_attrs = strip_meta_attrs(&f.attrs);
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                match classify_vtable_body(block) {
                    VTableBodyKind::Safe => {
                        own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                    }
                    VTableBodyKind::NeedsWrapper => {
                        if let Some(first) = b.stmts.first() {
                            let fs = quote!(#first).to_string();
                            if fs.contains("this") && fs.contains("self") {
                                b.stmts.remove(0);
                            }
                        }
                        rewrite_virtual_calls_for_wrapper(&mut b, &own_method_names);
                        // NeedsWrapper 体在 wrapper 上下文中运行：this = &__wrapper: &Wrapper
                        // 将 __base(this, ...) 改为 __base(&*this.vtable, ...)
                        // （base 函数需要 __BT: AncestorVTable，Wrapper 本身不实现，须走 vtable）
                        rewrite_base_calls_for_wrapper(&mut b);
                        let stmts = &b.stmts;
                        own_accessor_impls.push(quote! {
                            #(#keep_attrs)*
                            #sig {
                                let __rc = ::std::rc::Rc::new(::std::clone::Clone::clone(self));
                                let __wrapper = #struct_ident {
                                    vtable: __rc.clone() as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                                    any: __rc as ::std::rc::Rc<dyn ::std::any::Any>,
                                };
                                let this = &__wrapper;
                                #(#stmts)*
                            }
                        });
                    }
                    VTableBodyKind::Skip => {}
                }
            }
        }
        vtable_impls.push(quote! {
            impl #impl_g #vtable_trait_ident #ty_g for #inner_ident #ty_g #where_c {
                #(#own_accessor_impls)*
            }
        });

        // 处理未被 ancestors 列表覆盖的 VirtualOverride（vtable_class 不在祖先中）
        // 这种情况理论上不应出现，但做兜底生成
        for (vtable_class, override_fns) in &vtable_overrides {
            if !ancestors.contains(vtable_class) {
                let anc_vtable_ident = format_ident!("{}__VTable", vtable_class);
                let mut items: Vec<TokenStream2> = Vec::new();
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &basic_names, &ref_names);
                            items.push(quote! { #(#keep_attrs)* #sig #b });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let msg = format!("stub: {}.{}", meta.binary_name, mname);
                            items.push(quote! { #sig { panic!(#msg) } });
                        }
                    }
                }
                vtable_impls.push(quote! {
                    impl #impl_g #anc_vtable_ident #ty_g for #inner_ident #ty_g #where_c {
                        #(#items)*
                    }
                });
            }
        }
    }

    // ══════════════════════════════════════════════════════════════════════════
    // 5. Wrapper struct + Default + Clone
    // ══════════════════════════════════════════════════════════════════════════

    let wrapper_struct = quote! {
        #[allow(non_camel_case_types)]
        pub struct #struct_ident #impl_g #where_c {
            pub(crate) vtable: ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
            pub(crate) any: ::std::rc::Rc<dyn ::std::any::Any>,
        }
    };

    let wrapper_default = quote! {
        impl #impl_g ::std::default::Default for #struct_ident #ty_g #where_c {
            fn default() -> Self {
                let rc = ::std::rc::Rc::new(<#inner_ident #ty_g as ::std::default::Default>::default());
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                }
            }
        }
    };

    let wrapper_clone = quote! {
        impl #impl_g ::std::clone::Clone for #struct_ident #ty_g #where_c {
            fn clone(&self) -> Self {
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&self.vtable),
                    any: ::std::rc::Rc::clone(&self.any),
                }
            }
        }
    };

    let wrapper_partialeq = quote! {
        impl #impl_g ::std::cmp::PartialEq for #struct_ident #ty_g #where_c {
            fn eq(&self, other: &Self) -> bool {
                ::std::rc::Rc::ptr_eq(&self.vtable, &other.vtable)
            }
        }
    };

    let wrapper_debug = quote! {
        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}({})", stringify!(#struct_ident), ObjectVTable::toString(&*self.vtable))
            }
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 6. impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）
    // ══════════════════════════════════════════════════════════════════════════

    let obj_vtable_for_wrapper = if !binary_name.is_empty() {
        let to_string_fwd: TokenStream2 = if meta.has_to_string_method {
            quote! {
                fn toString(&self) -> ::std::string::String {
                    // 用 UFCS 消歧义：避免与 Java toString() -> Result<String> 冲突
                    ObjectVTable::toString(&*self.vtable)
                }
            }
        } else {
            quote! {}
        };
        let hash_code_fwd: TokenStream2 = if meta.has_hash_code_method {
            quote! {
                fn hashCode(&self) -> i32 { ObjectVTable::hashCode(&*self.vtable) }
            }
        } else {
            quote! {}
        };
        quote! {
            impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    self.vtable.is_instance_of(type_id)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 7. Impl block on wrapper（字段访问器委托 + 虚方法委托 + 构造器 + __new_with_super）
    // ══════════════════════════════════════════════════════════════════════════

    let mut wrapper_methods: Vec<TokenStream2> = Vec::new();

    // 字段访问器委托（own fields）
    for (name, ty) in &fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        }
    }

    // 字段访问器委托（superclass_fields，继承字段）
    for (name, ty) in &meta.superclass_fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            wrapper_methods.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.vtable.#get() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::RefMut<'_, #ty> { self.vtable.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.vtable.#set(v); }
            });
        }
    }

    // VirtualDefine 方法：wrapper 统一委托到 vtable 以保证多态正确性。
    // VirtualDefine wrapper：
    // - 有方法体且体不是 vtable-safe（含 Clone::clone(this)/Self:: 等 wrapper 专属操作）→
    //   直接在 wrapper 上下文执行方法体（__inner 无法持有 Rc，无法重建 wrapper 返回值）
    // - 其余情况（无方法体/vtable-safe 方法体）→ 通过 vtable dispatch，保证子类覆盖生效
    for f in &vtable_defines {
        let sig = &f.sig;
        let mname = &sig.ident;
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let vis = &f.vis;

        // NeedsWrapper 方法体（含 Clone::clone(this) 或 this.method() 调用）：
        // 直接放进 wrapper impl（this: &Wrapper）以保证 this 类型正确。
        // 同时：
        // 1. 将 __base(this, ...) 改为 __base(&*this.vtable, ...)
        // 2. 将 this.method(args) 改为 (&*this.vtable).method(args)，
        //    通过 vtable supertrait 链访问继承但未显式覆盖的虚方法。
        if let Some(block) = &f.block {
            if matches!(classify_vtable_body(block), VTableBodyKind::NeedsWrapper) {
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                rewrite_base_calls_for_wrapper(&mut b);
                rewrite_virtual_calls_for_wrapper(&mut b, &own_method_names);
                wrapper_methods.push(quote! {
                    #(#keep_attrs)*
                    #vis #sig #b
                });
                continue;
            }
        }

        let param_names: Vec<_> = sig.inputs.iter().filter_map(|arg| {
            if let syn::FnArg::Typed(pt) = arg {
                if let syn::Pat::Ident(pi) = &*pt.pat {
                    return Some(pi.ident.clone());
                }
            }
            None
        }).collect();
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { #vtable_trait_ident::#mname(&*self.vtable, #(#param_names),*) }
        });
    }

    // VirtualOverride 委托（wrapper 也需要暴露同名方法，转发到 vtable）
    let mut seen_delegators: HashSet<String> = vtable_defines
        .iter()
        .map(|f| f.sig.ident.to_string())
        .collect();
    for (vtable_class, override_fns) in &vtable_overrides {
        for f in override_fns {
            let mname_str = f.sig.ident.to_string();
            if seen_delegators.contains(&mname_str) {
                continue;
            }
            seen_delegators.insert(mname_str);
            let sig = &f.sig;
            let mname = &sig.ident;
            let param_names: Vec<_> = sig.inputs.iter().filter_map(|arg| {
                if let syn::FnArg::Typed(pt) = arg {
                    if let syn::Pat::Ident(pi) = &*pt.pat {
                        return Some(pi.ident.clone());
                    }
                }
                None
            }).collect();
            let keep_attrs = strip_meta_attrs(&f.attrs);
            let vis = &f.vis;
            // UFCS：用 vtable_class__VTable 消歧义（VirtualOverride 同名方法冲突）
            let anc_vtable = format_ident!("{}__VTable", vtable_class);
            wrapper_methods.push(quote! {
                #(#keep_attrs)*
                #[inline]
                #vis #sig { #anc_vtable::#mname(&*self.vtable, #(#param_names),*) }
            });
        }
    }

    // Constructor / NonVirtual 方法（保持原 body，走 Rewriter）
    for f in &non_virtual {
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let vis = &f.vis;
        let sig = &f.sig;
        match &f.block {
            Some(block) => {
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                wrapper_methods.push(quote! {
                    #(#keep_attrs)*
                    #vis #sig #b
                });
            }
            None => {
                let mname = sig.ident.to_string();
                let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                let bin = &meta.binary_name;
                let msg = if desc.is_empty() {
                    format!("stub: {}.{}", bin, mname)
                } else {
                    format!("stub: {}.{}:{}", bin, mname, desc)
                };
                wrapper_methods.push(quote! {
                    #(#keep_attrs)*
                    #vis #sig { panic!(#msg) }
                });
            }
        }
    }

    // __new_with_super（有父类时生成）
    let new_with_super: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        // 从 parent 的字段访问器拉取 superclass_fields 的值，初始化 __inner
        let mut field_inits: Vec<TokenStream2> = Vec::new();
        for (name, ty) in &meta.superclass_fields {
            let get = format_ident!("__get_{}", name);
            if is_basic(ty) {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::Cell::new(parent.#get())),
                });
            } else {
                field_inits.push(quote! {
                    #name: ::std::rc::Rc::new(::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(parent.#get()))
                    )),
                });
            }
        }
        quote! {
            #[doc(hidden)]
            pub fn __new_with_super(parent: #sup_ty) -> Self {
                let inner = #inner_ident {
                    #(#field_inits)*
                    ..::std::default::Default::default()
                };
                let rc = ::std::rc::Rc::new(inner);
                #struct_ident {
                    vtable: ::std::rc::Rc::clone(&rc) as ::std::rc::Rc<dyn #vtable_trait_ident #ty_g>,
                    any: rc as ::std::rc::Rc<dyn ::std::any::Any>,
                }
            }
        }
    } else {
        quote! {}
    };

    let wrapper_impl = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            #(#wrapper_methods)*
            #new_with_super
        }
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 8. BINARY_NAME 常量
    // ══════════════════════════════════════════════════════════════════════════

    let binary_name_impl: TokenStream2 = if !binary_name.is_empty() {
        quote! {
            impl #impl_g #struct_ident #ty_g #where_c {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 9. From<Object> for ClassName
    // ══════════════════════════════════════════════════════════════════════════

    let obj = quote! { Object };
    let from_object_impl = quote! {
        impl #impl_g From<#obj> for #struct_ident #ty_g #where_c {
            fn from(obj: #obj) -> Self { obj.downcast::<Self>() }
        }
    };

    // R-1: binary_name 为空的类需要手动 Into<Object>
    let into_object_impl: TokenStream2 = if binary_name.is_empty() {
        quote! {
            impl #impl_g Into<#obj> for #struct_ident #ty_g #where_c {
                fn into(self) -> #obj { #obj::from_any(self) }
            }
        }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 10. From<ClassName> for each ancestor（vtable trait upcasting，含多级跳跃）
    // ══════════════════════════════════════════════════════════════════════════

    let from_child_for_parent: TokenStream2 = if meta.superclass.is_some() {
        // 为每个祖先（排除 Object 和自身）生成 From<Self> for Ancestor
        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = meta.all_superclasses.clone();
        // 祖先类型参数：若当前类有泛型用 ty_g，否则用 superclass 的类型参数
        let anc_type_args: TokenStream2 = if gen.params.is_empty() {
            superclass_vtable_args.clone()
        } else {
            quote! { #ty_g }
        };
        let impls: Vec<TokenStream2> = ancestors.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let anc_vtable = format_ident!("{}__VTable", anc_name);
            let atag = anc_type_args.clone();
            quote! {
                impl #impl_g From<#struct_ident #ty_g> for #anc_ident #atag #where_c {
                    fn from(child: #struct_ident #ty_g) -> #anc_ident #atag {
                        // 结构体字面量不能用 Type<E> {...} 语法（被解析为比较链），
                        // 省略泛型参数由返回类型推导
                        #anc_ident {
                            vtable: child.vtable as ::std::rc::Rc<dyn #anc_vtable #atag>,
                            any: child.any,
                        }
                    }
                }
            }
        }).collect();
        quote! { #(#impls)* }
    } else {
        quote! {}
    };

    // ══════════════════════════════════════════════════════════════════════════
    // 11. 自由函数 ClassName__methodName_base（供 invokespecial super() 调用）
    // ══════════════════════════════════════════════════════════════════════════

    let mut base_fns: Vec<TokenStream2> = Vec::new();

    // base 函数用的合并泛型：类泛型 + __BT: ?Sized（无 vtable trait 约束，因为全是 panic stub）
    // ?Sized 允许传入 &dyn VTable（fat pointer），不要求 __BT 实现 Sized
    let mut base_gen = gen.clone();
    base_gen.params.push(syn::parse_quote!(__BT: ?Sized));
    let (base_impl_g, _, _) = base_gen.split_for_impl();

    // VirtualDefine 方法生成 base 函数
    // 策略（两阶段）：
    //   1. 先对 body 做 replace_clone_this_in_ok（Ok(Clone::clone(this)) → Ok(Default::default())）
    //   2. 检查替换后 body 在 this: &__BT 上下文是否安全：
    //      - 仍有 bare Clone::clone(this)（传参用途）→ 不安全 → panic stub
    //      - 仍有 this.method() 且 method 不在 vtable_define_names（NonVirtual/native）→ 不安全 → panic stub
    //      - 否则（VirtualDefine vtable 方法调用 + accessor 调用）→ 安全，使用实际 body
    // 注：VirtualDefine 方法（appendNull / ensureCapacityInternal 等）在 &__BT: VTable 上下文可直接调用

    // 本类 VirtualDefine 方法名集合，供下面 base 函数安全判断
    let vtable_define_names: HashSet<String> = vtable_defines.iter()
        .map(|f| f.sig.ident.to_string())
        .collect();

    for f in &vtable_defines {
        if let Some(block) = &f.block {
            let sig = &f.sig;
            let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
            let non_self_params: Vec<_> = sig.inputs.iter()
                .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                .collect();
            let ret = &sig.output;
            let mname_str = sig.ident.to_string();
            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
            let binary = &meta.binary_name;

            let mut b = block.clone();
            rewrite_block(&mut b, &basic_names, &ref_names);
            // 去掉首句 "let this = self;"（base 函数参数直接就是 this）
            if let Some(first) = b.stmts.first() {
                let fs = quote!(#first).to_string();
                if fs.contains("this") && fs.contains("self") {
                    b.stmts.remove(0);
                }
            }
            // 替换 Ok(Clone::clone(this)) → Ok(Default::default())
            replace_clone_this_in_ok(&mut b);

            // 判断替换后 body 是否对 &__BT 上下文安全
            let bs = quote!(#b).to_string();
            let has_bare_clone_this = bs.contains("Clone :: clone (this)")
                || bs.contains("Clone :: clone(this)")
                || bs.contains("Clone::clone (this)")
                || bs.contains("Clone::clone(this)");
            // 检查 this.method() 中是否有非 vtable / 非 accessor 方法
            let has_non_vtable_call = {
                let mut found = false;
                let parts: Vec<&str> = bs.split("this .").chain(bs.split("this.")).skip(1).collect();
                for part in parts {
                    let trimmed = part.trim_start();
                    let mname: String = trimmed.chars()
                        .take_while(|c| c.is_alphanumeric() || *c == '_')
                        .collect();
                    if !mname.is_empty() && !mname.starts_with("__") {
                        let rest = &trimmed[mname.len()..];
                        if rest.trim_start().starts_with('(') {
                            // 只有不在 vtable_define_names 里的才是 non-vtable 调用
                            if !vtable_define_names.contains(&mname) {
                                found = true;
                                break;
                            }
                        }
                    }
                }
                found
            };

            let has_self_ref = bs.contains("Self ::") || bs.contains("Self::");
            if has_bare_clone_this || has_non_vtable_call || has_self_ref {
                // body 不安全（bare Clone::clone(this) 传参 / non-vtable this.method() / Self::）→ panic stub
                let msg = format!("stub: super {}.{}:{}", binary, mname_str, desc);
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #base_impl_g (this: &__BT #(, #non_self_params)*) #ret {
                        panic!(#msg)
                    }
                });
            } else {
                // body 安全（只有 vtable 方法调用 + accessor），可在 &(impl VTable + ?Sized) 运行
                let mut body_gen = gen.clone();
                body_gen.params.push(syn::parse_quote!(__BT));
                body_gen.make_where_clause().predicates.push(
                    syn::parse_quote!(__BT: #vtable_trait_ident #ty_g + ?Sized)
                );
                let (body_impl_g, _, body_where_c) = body_gen.split_for_impl();
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #body_impl_g (this: &__BT #(, #non_self_params)*) #ret #body_where_c {
                        #b
                    }
                });
            }
        }
    }

    // VirtualOverride 方法生成 base 函数（VTable 约束为 virtual_in__VTable）
    // base 函数的 this: &impl AncestorVTable，而本类自有字段的 __get_xxx / Clone::clone 等
    // 均不在祖先 vtable 中，因此 VirtualOverride base 函数一律生成 panic stub。
    // （base 函数仅在 invokespecial super 调用时触发，HelloWorld 路径不涉及。）
    for (_vtable_class, override_fns) in &vtable_overrides {
        for f in override_fns {
            if f.block.is_some() {
                let sig = &f.sig;
                let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
                let non_self_params: Vec<_> = sig.inputs.iter()
                    .filter(|a| matches!(a, syn::FnArg::Typed(_)))
                    .collect();
                let ret = &sig.output;
                let mname_str = sig.ident.to_string();
                let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                let binary = &meta.binary_name;
                let msg = format!("stub: super {}.{}:{}", binary, mname_str, desc);
                // base 函数全是 panic stub，不需要 vtable trait bound
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case, unused_variables)]
                    pub fn #fn_name #base_impl_g (this: &__BT #(, #non_self_params)*) #ret {
                        panic!(#msg)
                    }
                });
            }
        }
    }

    // ══════════════════════════════════════════════════════════════════════════
    // 最终组合
    // ══════════════════════════════════════════════════════════════════════════

    quote! {
        #vtable_trait
        #inner_struct
        #obj_vtable_for_inner
        #(#vtable_impls)*
        #wrapper_struct
        #wrapper_default
        #wrapper_clone
        #wrapper_partialeq
        #wrapper_debug
        #obj_vtable_for_wrapper
        #wrapper_impl
        #binary_name_impl
        #from_object_impl
        #into_object_impl
        #from_child_for_parent
        #(#base_fns)*
    }
}

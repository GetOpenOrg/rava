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

// ══════════════════════════════════════════════════════════════════════════════
// all_supertypes 工具
// ══════════════════════════════════════════════════════════════════════════════

/// 将 JVM binary name（如 `TestInheritance$Animal` 或 `java/lang/Object`）
/// 转为 Rust 短名（取最后一段，替换 `$` → `_`）。
fn binary_to_rust_short(name: &str) -> String {
    let last = name.rsplit('/').next().unwrap_or(name);
    last.replace('$', "_")
}

/// 提取祖先列表（排除 Object 和自身），保持 all_supertypes 原始顺序。
/// 返回 Rust 短名列表。
fn ancestor_rust_names_ordered(all_supertypes: &[String], self_name: &str) -> Vec<String> {
    all_supertypes
        .iter()
        .filter_map(|s| {
            let short = binary_to_rust_short(s);
            if short == "Object" || short == self_name {
                return None;
            }
            // 跳过含 '/' 的 JDK 包路径（非用户类祖先）
            if s.contains('/') {
                return None;
            }
            Some(short)
        })
        .collect()
}

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
    if let Some(sup) = &meta.superclass {
        type_texts.push(quote!(#sup).to_string());
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

    // ══════════════════════════════════════════════════════════════════════════
    // 1. VTable trait
    // ══════════════════════════════════════════════════════════════════════════

    // supertrait：有父类 → 父类 __VTable；无父类 → ObjectVTable
    let vtable_supertrait: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        let sup_vtable = format_ident!("{}__VTable", quote!(#sup_ty).to_string().replace(' ', ""));
        quote! { #sup_vtable }
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

    // VirtualDefine 的 default impl（方法体经 Rewriter）
    let mut vtable_default_methods: Vec<TokenStream2> = Vec::new();
    for f in &vtable_defines {
        let sig = &f.sig;
        let keep_attrs = strip_meta_attrs(&f.attrs);
        match &f.block {
            Some(block) => {
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                vtable_default_methods.push(quote! {
                    #(#keep_attrs)*
                    #sig #b
                });
            }
            None => {
                let mname = sig.ident.to_string();
                let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                let binary = &meta.binary_name;
                let msg = format!("stub: {}.{}:{}", binary, mname, desc);
                vtable_default_methods.push(quote! {
                    #sig { panic!(#msg) }
                });
            }
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
    for (name, ty) in &meta.superclass_fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::cell::Cell<#ty> }
        } else {
            quote! { ::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 自有字段
    for (name, ty) in &fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::cell::Cell<#ty> }
        } else {
            quote! { ::std::cell::RefCell<::std::option::Option<::std::boxed::Box<#ty>>> }
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
        vtable_impls.push(quote! {
            impl #impl_g #vtable_trait_ident #ty_g for #inner_ident #ty_g #where_c {
                #(#own_accessor_impls)*
            }
        });
    } else {
        // ── 有父类：顶层祖先 vtable impl + 其余空 impl + Self__VTable impl ──

        // 计算有序祖先列表（排除 Object 和 self，保持 all_supertypes 原始顺序）
        let ancestors = ancestor_rust_names_ordered(&meta.all_supertypes, &self_name);

        // 顶层祖先（第一个，如 "TestInheritance_Animal"）负责所有继承字段 + overrides
        let _top_ancestor = ancestors.first().cloned();

        for (idx, anc_name) in ancestors.iter().enumerate() {
            let anc_vtable_ident = format_ident!("{}__VTable", anc_name);

            let mut items: Vec<TokenStream2> = Vec::new();

            if idx == 0 {
                // 顶层：superclass_fields 的所有 accessor
                for (name, ty) in &meta.superclass_fields {
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
                            items.push(quote! {
                                #(#keep_attrs)*
                                #sig #b
                            });
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

            vtable_impls.push(quote! {
                impl #impl_g #anc_vtable_ident #ty_g for #inner_ident #ty_g #where_c {
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
                write!(f, "{}({})", stringify!(#struct_ident), self.vtable.toString())
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
                    self.vtable.toString()
                }
            }
        } else {
            quote! {}
        };
        let hash_code_fwd: TokenStream2 = if meta.has_hash_code_method {
            quote! {
                fn hashCode(&self) -> i32 { self.vtable.hashCode() }
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

    // VirtualDefine 委托
    for f in &vtable_defines {
        let sig = &f.sig;
        let mname = &sig.ident;
        // 提取参数名用于转发
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
        wrapper_methods.push(quote! {
            #(#keep_attrs)*
            #[inline]
            #vis #sig { self.vtable.#mname(#(#param_names),*) }
        });
    }

    // VirtualOverride 委托（wrapper 也需要暴露同名方法，转发到 vtable）
    let mut seen_delegators: HashSet<String> = vtable_defines
        .iter()
        .map(|f| f.sig.ident.to_string())
        .collect();
    for (_vtable_class, override_fns) in &vtable_overrides {
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
            wrapper_methods.push(quote! {
                #(#keep_attrs)*
                #[inline]
                #vis #sig { self.vtable.#mname(#(#param_names),*) }
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
                    #name: ::std::cell::Cell::new(parent.#get()),
                });
            } else {
                field_inits.push(quote! {
                    #name: ::std::cell::RefCell::new(
                        ::std::option::Option::Some(::std::boxed::Box::new(parent.#get()))
                    ),
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
        // 使用 all_supertypes 列表：ancestor_rust_names_ordered 已排除 Object 和 self
        let ancestors = ancestor_rust_names_ordered(&meta.all_supertypes, &self_name);
        let impls: Vec<TokenStream2> = ancestors.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let anc_vtable = format_ident!("{}__VTable", anc_name);
            quote! {
                impl #impl_g From<#struct_ident #ty_g> for #anc_ident #ty_g #where_c {
                    fn from(child: #struct_ident #ty_g) -> #anc_ident #ty_g {
                        #anc_ident #ty_g {
                            vtable: child.vtable as ::std::rc::Rc<dyn #anc_vtable #ty_g>,
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

    // VirtualDefine 方法生成 base 函数（VTable 约束为 Self__VTable）
    for f in &vtable_defines {
        if let Some(block) = &f.block {
            let sig = &f.sig;
            let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
            let mut base_sig = sig.clone();
            // 替换 &self 接收者为 this: &T
            base_sig.inputs = syn::punctuated::Punctuated::new();
            base_sig.inputs.push(syn::parse_quote! { this: &impl #vtable_trait_ident #ty_g });
            // 追加其余参数
            for arg in sig.inputs.iter() {
                if let syn::FnArg::Typed(_) = arg {
                    base_sig.inputs.push(arg.clone());
                }
            }
            let mut b = block.clone();
            rewrite_block(&mut b, &basic_names, &ref_names);
            base_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub fn #fn_name #impl_g #b
            });
            // 修正：不生成带签名的函数，改用简洁方式
            // 实际上我们需要完整的函数签名，上面的写法有问题，重新写
            base_fns.pop();

            // 提取参数（不含 self）
            let non_self_params: Vec<_> = sig.inputs.iter().filter(|a| matches!(a, syn::FnArg::Typed(_))).collect();
            let ret = &sig.output;
            base_fns.push(quote! {
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub fn #fn_name #impl_g (this: &impl #vtable_trait_ident #ty_g #(, #non_self_params)*) #ret #b
            });
        }
    }

    // VirtualOverride 方法生成 base 函数（VTable 约束为 virtual_in__VTable）
    for (vtable_class, override_fns) in &vtable_overrides {
        let vtable_class_ident = format_ident!("{}__VTable", vtable_class);
        for f in override_fns {
            if let Some(block) = &f.block {
                let sig = &f.sig;
                let fn_name = format_ident!("{}__{}_base", self_name, sig.ident);
                let non_self_params: Vec<_> = sig.inputs.iter().filter(|a| matches!(a, syn::FnArg::Typed(_))).collect();
                let ret = &sig.output;
                let mut b = block.clone();
                rewrite_block(&mut b, &basic_names, &ref_names);
                base_fns.push(quote! {
                    #[doc(hidden)]
                    #[allow(non_snake_case)]
                    pub fn #fn_name #impl_g (this: &impl #vtable_class_ident #ty_g #(, #non_self_params)*) #ret #b
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

//! `java_class!` 块级宏实现（方案 Step 2–5）。
//!
//! 输入语法（见 docs/plans/2026-09-16-java-class-macro-unified.md §4）：
//!
//! ```ignore
//! java_class! {
//!     #[binary_name = "java/util/ArrayList"]
//!     #[generic_signature = "<E:...>..."]
//!     #[superclass = "AbstractList<E>"]
//!     #[superclass_fields(modCount: i32)]
//!
//!     pub struct ArrayList<E> {
//!         #[field_sig = "[TE;"]
//!         elementData: Vec<E>,
//!         #[field_sig = "I"]
//!         size: i32,
//!     }
//!
//!     impl<E> ArrayList<E> {
//!         #[descriptor = "(Ljava/lang/Object;)Z"]
//!         pub fn add(&self, e: E) -> Result<bool> { ... }
//!
//!         #[descriptor = "..."]
//!         #[native]
//!         pub fn sort(&self, c: Object) -> Result<()>;
//!     }
//! }
//! ```
//!
//! 展开产物：
//!   - `<Name>__inner` 存储 struct（字段各自 Cell/RefCell 包裹，封装细节不出现在读者视野）
//!   - `<Name>` newtype 包装
//!   - 字段访问器：基本类型 `__get_/__set_`，引用类型 `__get_/__borrow_/__borrow_mut_/__set_`
//!   - 继承字段（`superclass_fields`）的转发访问器，内部走 `_super`
//!   - 方法体 token 重写：`self.field` / `self.field.m()` → 访问器调用
//!   - `BINARY_NAME` / `ObjectVTable` / `Into<Object>` / `From<Object>` / `Debug`
//!
//! ## 与设计文档的三处偏离（均已记录原因）
//!
//! 1. **存储用「每字段独立 Cell/RefCell」，而非单一 `RefCell<Inner>`**。
//!    单一 `RefCell<Inner>` 下，`self.elementData.push(self.size)` 这类同语句跨字段访问
//!    会在参数求值时触发第二次 borrow，运行时 panic（§9 的 block-scoped 只能隔开语句，
//!    隔不开同一语句内的参数求值）。按字段独立加锁后，跨字段访问互不冲突。
//!
//! 2. **`_super` 是真实字段（父类值），不是 PhantomData**。
//!    方案 §6/§11 讨论过的 `unsafe { &*(self as *const _ as *const Parent) }` 前缀转换
//!    在此不成立：`Parent` 是 `Parent(RefCell<Parent__inner>)` 风格的 newtype，首字段是
//!    RefCell/父类值，而子类展平后首字段是 `modCount`，两者前缀类型不同，`repr(C)` 无法救。
//!    保留嵌套父类值后，Upcast 就是 `&self._super`，无需 unsafe。
//!    对读者而言字段仍是展平的（`self.modCount = x`），封装未泄漏。
//!
//! 3. **字段/方法类型解析（含接口擦除）留在 codegen**，宏只消费已解析好的裸 Rust 类型。
//!    §5 的「接口泛型参数擦除」需要跨类 registry（判断某类型是不是接口），而 §6 明确
//!    要求宏侧零 registry 依赖，二者不可兼得；registry 只存在于 codegen Python 侧，
//!    因此类型解析统一在 codegen 完成，宏侧 `field_sig` / `generic_signature` 作为元数据保留。

use std::collections::HashSet;

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

/// impl 块中的一个方法条目。`block == None` 表示 native 声明（以 `;` 结尾）。
struct FnItem {
    attrs: Vec<Attribute>,
    vis: Visibility,
    sig: Signature,
    block: Option<Block>,
}

/// 单个 Java 类的完整输入：类级属性 + struct 定义 + impl 块。
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

        // ── struct 定义 ────────────────────────────────────────────────────
        let _struct_vis: Visibility = input.parse()?;
        let _: Token![struct] = input.parse()?;
        let struct_ident: Ident = input.parse()?;
        let mut generics: syn::Generics = input.parse()?;
        if input.peek(Token![where]) {
            generics.where_clause = Some(input.parse()?);
        }

        let mut fields: Vec<(Ident, Type)> = Vec::new();
        // `pub struct Name;`（无字段）也是合法输入——接口走这条路（Arch-1：接口 = Object 别名）
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
            // `pub struct Name;` —— 无字段（无实例字段的类，以及接口）
            let _: Token![;] = input.parse()?;
        }

        // ── impl 块 ────────────────────────────────────────────────────────
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
    /// 展平后的祖先实例字段（父类在前），由 codegen Python 传入（方案 §6）
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
                        .ok_or_else(|| meta.error("superclass_fields 的键必须是字段名"))?
                        .clone();
                    let _: Token![:] = meta.input.parse()?;
                    let ty: Type = meta.input.parse()?;
                    items.push((ident, ty));
                    Ok(())
                })?;
                m.superclass_fields = items;
            }
            // 其余已知键（descriptor / generic_signature / source 等）忽略
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

/// Java 基本类型对应的 Rust Copy 类型 → 用 `Cell` 承载，访问器只有 get/set 两态。
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

/// 宏内部消费的元数据属性，展开时必须剔除（否则变成未知属性）。
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

/// 读取元数据值。两种来源：
///   1. 直接形式 `#[descriptor = "..."]`
///   2. 嵌套形式 `#[java_method(descriptor = "...", ...)]` 的键
/// 之所以支持嵌套形式：codegen 用 `#[java_method(...)]` 携带完整字节码元数据
/// （name/descriptor/access/modifiers/...），宏只需从中取 descriptor 生成存根消息。
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
// 方法体 token 重写（方案 §8 的 7 种模式）
// ══════════════════════════════════════════════════════════════════════════════

struct Rewriter {
    /// 基本类型字段名（Cell 承载）
    basic: HashSet<String>,
    /// 引用类型字段名（RefCell 承载）
    reference: HashSet<String>,
    /// 指向 self 的接收者别名：self / this / `let x = self;` 引入的 x
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

    /// 判断 `base.member` 是否是「已知接收者别名 + 已知字段」，返回 `Some(is_basic)`。
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

    /// 重写「链式位置」（方法接收者 / 下标基址 / 字段基址）上的表达式：
    /// 找到链上最内层的 `self.<field>` 并替换为访问器调用，其余部分保持结构不变。
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
                        // §9：token 树阶段无法做类型推断，引用类型一律走 borrow_mut。
                        // 单线程 RefCell 下只读方法取可变借用无副作用。
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

/// 去掉透明包裹，取出 `base.member`（仅命名成员）。
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

/// 复合赋值运算符 → 对应的算术运算符（`+=` → `+`）。非复合赋值返回 None。
/// syn 2 把复合赋值并入了 `Expr::Binary`，需要用 BinOp 判别。
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
        // 收集 `let x = self;` / `let x = this;` 形式的接收者别名
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
            // ── 模式 3 / 5：字段整体赋值（基本类型与引用类型同形）────────────
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

            // ── 模式 4：基本类型复合赋值 ────────────────────────────────────
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

            // ── 模式 1 / 6 / 7：链式访问 ────────────────────────────────────
            Expr::MethodCall(_) | Expr::Index(_) => {
                self.rewrite_chain(e);
            }

            // ── 模式 2：基本类型读取 / 引用类型整体读取（clone 语义）────────
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

/// 对单个方法体做字段访问重写（两遍：先收集接收者别名，再重写）。
fn rewrite_block(block: &mut Block, basic: &HashSet<String>, reference: &HashSet<String>) {
    let mut prober = Rewriter::new(basic, reference);
    prober.visit_block_mut(block);
    let aliases = prober.aliases;

    let mut rw = Rewriter::new(basic, reference);
    rw.aliases = aliases;
    rw.visit_block_mut(block);
}

// ══════════════════════════════════════════════════════════════════════════════
// 展开
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

    // ── 接口：Arch-1 语义，接口 = Object 类型别名 ────────────────────────────
    if meta.is_interface {
        return quote! { pub type #struct_ident = Object; };
    }

    // ── 泛型参数补齐 Clone + Default + 'static ─────────────────────────────
    // bound 组合是纯粹的 Rust 能力声明，故意不引入 `JavaType` 这类游离于
    // Java 命名空间之外的 trait 名（CLAUDE.md 命名原则，方案 §4）。
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

    let inner_ident = format_ident!("{}__inner", struct_ident);

    // ── 字段集合（自有 + 继承）────────────────────────────────────────────
    let mut basic_names: HashSet<String> = HashSet::new();
    let mut ref_names: HashSet<String> = HashSet::new();
    for (name, ty) in fields.iter().chain(meta.superclass_fields.iter()) {
        if is_basic(ty) {
            basic_names.insert(name.to_string());
        } else {
            ref_names.insert(name.to_string());
        }
    }

    // ── Inner struct + newtype ─────────────────────────────────────────────
    let mut inner_field_tokens: Vec<TokenStream2> = Vec::new();
    if let Some(sup_ty) = &meta.superclass {
        inner_field_tokens.push(quote! { pub(crate) _super: #sup_ty });
    }
    for (name, ty) in &fields {
        let cell_ty = if is_basic(ty) {
            quote! { ::std::cell::Cell<#ty> }
        } else {
            // 引用类型字段用 `Box<RefCell<T>>`：Box 提供间接层，让自引用/互引用字段
            // （Throwable.cause、Class.classData 等）有确定大小（E0072）。
            // 语义上等价于 Java：字段槽可变 + 对象在堆上。访问器写法不受影响
            // （`Box<RefCell<T>>.borrow()` 自动解引用）。
            quote! { ::std::boxed::Box<::std::cell::RefCell<#ty>> }
        };
        inner_field_tokens.push(quote! { pub(crate) #name: #cell_ty });
    }

    // 未被任何字段 / 父类类型引用的类型参数需要 PhantomData 兜底，否则报 E0392。
    // 判定方式是「标识符精确匹配」：把字段与父类的类型文本切成词，
    // 逐词比对参数名，`E` 不会被 `ElementData` 这样的词误命中。
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
                // `fn() -> T` 形式保持协变且不暗示所有权，避免 dropck 上的意外约束。
                phantom_fields.push(quote! { #p });
            }
        }
    }
    if !phantom_fields.is_empty() {
        inner_field_tokens.push(quote! {
            pub(crate) __phantom: ( #( ::std::marker::PhantomData<fn() -> #phantom_fields>, )* )
        });
    }

    let inner_struct = quote! {
        #[doc(hidden)]
        #[derive(Clone, Default, PartialEq)]
        pub struct #inner_ident #impl_g {
            #(#inner_field_tokens,)*
        }
    };

    let newtype = quote! {
        #[derive(Clone, Default, PartialEq)]
        pub struct #struct_ident #impl_g (#inner_ident #ty_g);
    };

    // ── 字段访问器（方案 §7）──────────────────────────────────────────────
    let mut accessors: Vec<TokenStream2> = Vec::new();

    // 自有字段：直接落到 Cell / RefCell
    for (name, ty) in &fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            accessors.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.0.#name.get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.0.#name.set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            accessors.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.0.#name.borrow().clone() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::Ref<'_, #ty> { self.0.#name.borrow() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.0.#name.borrow_mut() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { *self.0.#name.borrow_mut() = v; }
            });
        }
    }

    // 继承字段：转发到 _super（展平视图）。要求父类暴露同名访问器。
    for (name, ty) in &meta.superclass_fields {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if is_basic(ty) {
            accessors.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.0._super.#get() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.0._super.#set(v); }
            });
        } else {
            let bor = format_ident!("__borrow_{}", name);
            let borm = format_ident!("__borrow_mut_{}", name);
            accessors.push(quote! {
                #[doc(hidden)] #[inline]
                pub fn #get(&self) -> #ty { self.0._super.#get() }
                #[doc(hidden)] #[inline]
                pub fn #bor(&self) -> ::std::cell::Ref<'_, #ty> { self.0._super.#bor() }
                #[doc(hidden)] #[inline]
                pub fn #borm(&self) -> ::std::cell::RefMut<'_, #ty> { self.0._super.#borm() }
                #[doc(hidden)] #[inline]
                pub fn #set(&self, v: #ty) { self.0._super.#set(v); }
            });
        }
    }

    // ── 父类访问器（Upcast / 构造期用）────────────────────────────────────
    let super_helpers: TokenStream2 = if let Some(sup_ty) = &meta.superclass {
        quote! {
            /// 父类状态的所有者（唯一）。**禁止**在宏外直接访问该字段。
            /// 子类只通过访问器看到展平视图，超类方法派发走 `__super()`。
            /// 构造期由 `Self::__new_with_super(...)` 整体写入，不提供单独的 setter。
            #[doc(hidden)] #[inline]
            pub fn __super(&self) -> &#sup_ty { &self.0._super }
            #[doc(hidden)] #[inline]
            pub fn __into_super(self) -> #sup_ty { self.0._super }
            /// 构造期用：`super(...)` 字节码的落点。
            /// 语义上等价于「以已构造好的父类值重建 this」——JVM 校验器保证
            /// `invokespecial <init>` 只可能指向直接父类或同类，因此这里只需要一层。
            #[doc(hidden)]
            pub fn __new_with_super(__super: #sup_ty) -> Self {
                Self(#inner_ident { _super: __super, ..Default::default() })
            }
        }
    } else {
        quote! {}
    };

    // ── 方法展开 ──────────────────────────────────────────────────────────
    let mut method_tokens: Vec<TokenStream2> = Vec::new();
    for f in &fns {
        let keep_attrs = strip_meta_attrs(&f.attrs);
        let vis = &f.vis;
        let sig = &f.sig;

        match &f.block {
            Some(block) => {
                let mut block = block.clone();
                rewrite_block(&mut block, &basic_names, &ref_names);
                method_tokens.push(quote! {
                    #(#keep_attrs)*
                    #vis #sig #block
                });
            }
            None => {
                // native / 声明式方法 → panic 存根，消息格式与现有 codegen 一致
                // （CLAUDE.md 规则 2：命中存根时能精确定位类名+方法名+描述符）
                let mname = sig.ident.to_string();
                let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                let binary = &meta.binary_name;
                let msg = if desc.is_empty() {
                    format!("stub: {}.{}", binary, mname)
                } else {
                    format!("stub: {}.{}:{}", binary, mname, desc)
                };
                method_tokens.push(quote! {
                    #(#keep_attrs)*
                    #vis #sig { panic!(#msg) }
                });
            }
        }
    }

    let impl_block = quote! {
        impl #impl_g #struct_ident #ty_g #where_c {
            #(#accessors)*
            #super_helpers
            #(#method_tokens)*
        }
    };

    // ── BINARY_NAME / ObjectVTable / Into / From / Debug ──────────────────
    let binary_name = &meta.binary_name;
    let binary_name_impl: TokenStream2 = if binary_name.is_empty() {
        quote! {}
    } else {
        quote! {
            impl #impl_g #struct_ident #ty_g #where_c {
                pub const BINARY_NAME: &'static str = #binary_name;
            }
        }
    };

    let vtable_impl: TokenStream2 = if binary_name.is_empty() {
        quote! {}
    } else {
        let check_types: Vec<String> = if meta.all_supertypes.is_empty() {
            vec![binary_name.clone()]
        } else {
            meta.all_supertypes.clone()
        };
        let patterns = check_types.iter().map(|s| quote! { #s });
        let to_string_fwd: TokenStream2 = if meta.has_to_string_method {
            quote! {
                fn toString(&self) -> ::std::string::String {
                    Self::toString(self)
                        .map(|s| ::std::format!("{}", s))
                        .unwrap_or_else(|_| ::std::any::type_name::<Self>().to_owned())
                }
            }
        } else {
            quote! {}
        };
        let hash_code_fwd: TokenStream2 = if meta.has_hash_code_method {
            quote! {
                fn hashCode(&self) -> i32 { Self::hashCode(self).unwrap_or(0) }
            }
        } else {
            quote! {}
        };
        quote! {
            impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
                fn is_instance_of(&self, type_id: &str) -> bool {
                    matches!(type_id, #(#patterns)|*)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    };

    let obj = quote! { Object };
    let into_impl: TokenStream2 = if binary_name.is_empty() {
        quote! {
            impl #impl_g Into<#obj> for #struct_ident #ty_g #where_c {
                fn into(self) -> #obj { #obj::from_any(self) }
            }
        }
    } else {
        quote! {
            impl #impl_g Into<#obj> for #struct_ident #ty_g #where_c {
                fn into(self) -> #obj { #obj(::std::rc::Rc::new(self)) }
            }
        }
    };

    let from_impl = quote! {
        impl #impl_g From<#obj> for #struct_ident #ty_g #where_c {
            fn from(obj: #obj) -> Self { obj.downcast::<Self>() }
        }
    };

    let debug_impl = quote! {
        impl #impl_g ::std::fmt::Debug for #struct_ident #ty_g #where_c {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", ::std::any::type_name::<Self>())
            }
        }
    };

    quote! {
        #inner_struct
        #newtype
        #impl_block
        #binary_name_impl
        #vtable_impl
        #into_impl
        #from_impl
        #debug_impl
    }
}

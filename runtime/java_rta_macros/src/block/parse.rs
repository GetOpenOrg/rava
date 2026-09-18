//! 宏输入解析：ClassInput、FnItem、ClassMeta 及其属性解析辅助函数。

use std::collections::HashMap;

use syn::{
    parse::{Parse, ParseStream},
    Attribute, Block, Expr, Ident, Lit, Signature, Token, Type, Visibility,
};

// ══════════════════════════════════════════════════════════════════════════════
// 输入解析
// ══════════════════════════════════════════════════════════════════════════════

pub(crate) struct FnItem {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub sig: Signature,
    pub block: Option<Block>,
}

/// `impl Iface<Args> for Class<Args> { 擦除签名的方法声明 }` —— 类实现的一个接口（Java `implements`）。
///
/// 块内每条声明对应接口的一个实例方法，签名是该方法的**擦除形态**（与 javac 为泛型
/// 实现类生成的桥接方法同形）；宏据此生成 `impl Iface__VTable for Class__inner`。
pub(crate) struct InterfaceImpl {
    /// 接口载体的 Rust 名（路径末段，不含类型实参）
    pub iface: Ident,
    pub fns: Vec<FnItem>,
}

/// static 字段声明（impl 块内）：
///   - `pub static NAME: Type;`          → 有存储的静态字段（JVM 默认值起步，由 `<clinit>` / putstatic 写入）
///   - `pub const NAME: Type = expr;`    → ConstantValue 编译期常量（访问不触发类初始化，JVMS §5.5）
pub(crate) struct StaticItem {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub const_value: Option<Expr>,
}

pub(crate) struct ClassInput {
    pub attrs: Vec<Attribute>,
    pub struct_ident: Ident,
    pub generics: syn::Generics,
    pub fields: Vec<(Ident, Type)>,
    pub fns: Vec<FnItem>,
    pub iface_impls: Vec<InterfaceImpl>,
    pub statics: Vec<StaticItem>,
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

        // impl 块序列：`impl Class { .. }`（自有成员）与 `impl Iface for Class { .. }`（实现的接口）
        let mut fns: Vec<FnItem> = Vec::new();
        let mut iface_impls: Vec<InterfaceImpl> = Vec::new();
        let mut statics: Vec<StaticItem> = Vec::new();
        while !input.is_empty() {
            let _mid_attrs = input.call(Attribute::parse_outer)?;
            let _: Token![impl] = input.parse()?;
            let _impl_generics: syn::Generics = input.parse()?;
            let first_ty: Type = input.parse()?;
            let iface = if input.peek(Token![for]) {
                let _: Token![for] = input.parse()?;
                let _self_ty: Type = input.parse()?;
                let (name, _) = split_type_name_args(&first_ty);
                Some(Ident::new(&name, proc_macro2::Span::call_site()))
            } else {
                None
            };
            if input.peek(Token![where]) {
                let _: syn::WhereClause = input.parse()?;
            }
            let body;
            syn::braced!(body in input);
            let (items, static_items) = parse_impl_fns(&body)?;
            match iface {
                Some(iface) => iface_impls.push(InterfaceImpl { iface, fns: items }),
                None => {
                    fns.extend(items);
                    statics.extend(static_items);
                }
            }
        }

        Ok(ClassInput { attrs, struct_ident, generics, fields, fns, iface_impls, statics })
    }
}

pub(crate) fn parse_impl_fns(input: ParseStream) -> syn::Result<(Vec<FnItem>, Vec<StaticItem>)> {
    let mut out = Vec::new();
    let mut statics = Vec::new();
    while !input.is_empty() {
        let attrs = input.call(Attribute::parse_outer)?;
        let vis: Visibility = input.parse()?;
        if input.peek(Token![static]) || input.peek(Token![const]) {
            let is_const = input.peek(Token![const]);
            if is_const {
                let _: Token![const] = input.parse()?;
            } else {
                let _: Token![static] = input.parse()?;
            }
            let name: Ident = input.parse()?;
            let _: Token![:] = input.parse()?;
            let ty: Type = input.parse()?;
            let const_value = if input.peek(Token![=]) {
                let _: Token![=] = input.parse()?;
                Some(input.parse::<Expr>()?)
            } else {
                None
            };
            let _: Token![;] = input.parse()?;
            if is_const && const_value.is_none() {
                return Err(syn::Error::new_spanned(&name, "const 静态字段必须带初值"));
            }
            statics.push(StaticItem { attrs, vis, name, ty, const_value });
            continue;
        }
        let sig: Signature = input.parse()?;
        let block = if input.peek(Token![;]) {
            let _: Token![;] = input.parse()?;
            None
        } else {
            let mut body = input.parse::<Block>()?;
            crate::try_macro::expand_in_block(&mut body);
            Some(body)
        };
        out.push(FnItem { attrs, vis, sig, block });
    }
    Ok((out, statics))
}

// ══════════════════════════════════════════════════════════════════════════════
// 类级属性
// ══════════════════════════════════════════════════════════════════════════════

/// `Name<A, B>` → ("Name", `<A, B>`)；无实参时第二项为空 token 流。
pub(crate) fn split_type_name_args(ty: &Type) -> (String, proc_macro2::TokenStream) {
    if let Type::Path(tp) = ty {
        if let Some(seg) = tp.path.segments.last() {
            let args = match &seg.arguments {
                syn::PathArguments::AngleBracketed(ab) if !ab.args.is_empty() => quote::quote! { #ab },
                _ => proc_macro2::TokenStream::new(),
            };
            return (seg.ident.to_string(), args);
        }
    }
    (quote::quote!(#ty).to_string(), proc_macro2::TokenStream::new())
}

#[derive(Default)]
pub(crate) struct ClassMeta {
    pub binary_name: String,
    pub superclass: Option<Type>,
    pub superclass_fields: Vec<(Ident, Type)>,
    /// 祖先按类型变量声明、本类视角代入为基本类型的继承字段：存储与访问器按引用字段处理
    pub superclass_reference_fields: std::collections::HashSet<String>,
    /// 线性超类链（从最深祖先到直接父类），Rust short names，不含 Object 和 self。
    pub all_superclasses: Vec<String>,
    /// 每个祖先在本类视角下的类型实参（含尖括号，如 `<P_IN, P_OUT, Object>`）；非泛型祖先为空。
    /// 各祖先元数不同（`AbstractPipeline<A, B, S>` : `PipelineHelper<B>`），必须逐个祖先给出。
    pub ancestor_type_args: HashMap<String, proc_macro2::TokenStream>,
    /// 每个祖先自己声明的字段列表：{ancestor_rust_name → [field_names]}。
    pub ancestor_fields_layout: HashMap<String, Vec<String>>,
    pub all_supertypes: Vec<String>,
    pub is_interface: bool,
    /// toString() 所属 vtable 的 Rust 类名（本类或祖先）；None = 继承链上没有翻译出的 toString。
    pub to_string_vtable: Option<String>,
    pub has_hash_code_method: bool,
    /// 共置 `_impl.rs` 手写 impl 块提供的 wrapper inherent 方法名（不在宏块内、不进 vtable）。
    pub impl_methods: Vec<String>,
}

impl ClassMeta {
    pub fn from_attrs(attrs: &[Attribute]) -> syn::Result<Self> {
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
                // 格式：Anc1<Args>;Anc2;...（每项是一个 Rust 类型，实参由 codegen 沿
                // SuperclassSignature 逐级代入解析）
                for seg in s.split(';').filter(|x| !x.is_empty()) {
                    let ty = syn::parse_str::<Type>(seg)?;
                    let (name, args) = split_type_name_args(&ty);
                    m.ancestor_type_args.insert(name.clone(), args);
                    m.all_superclasses.push(name);
                }
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
            } else if path.is_ident("to_string_vtable") {
                m.to_string_vtable = Some(lit_str(attr)?);
            } else if path.is_ident("has_hash_code_method") {
                m.has_hash_code_method = lit_bool(attr)?;
            } else if path.is_ident("impl_methods") {
                let s = lit_str(attr)?;
                m.impl_methods =
                    s.split(';').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect();
            } else if path.is_ident("superclass_reference_fields") {
                let s = lit_str(attr)?;
                m.superclass_reference_fields =
                    s.split(';').filter(|x| !x.is_empty()).map(|x| x.to_owned()).collect();
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

pub(crate) fn lit_str(attr: &Attribute) -> syn::Result<String> {
    if let syn::Meta::NameValue(nv) = &attr.meta {
        if let Expr::Lit(el) = &nv.value {
            if let Lit::Str(s) = &el.lit {
                return Ok(s.value());
            }
        }
    }
    Err(syn::Error::new_spanned(attr, "该属性需要字符串字面量值"))
}

pub(crate) fn lit_bool(attr: &Attribute) -> syn::Result<bool> {
    if let syn::Meta::NameValue(nv) = &attr.meta {
        if let Expr::Lit(el) = &nv.value {
            if let Lit::Bool(b) = &el.lit {
                return Ok(b.value);
            }
        }
    }
    Err(syn::Error::new_spanned(attr, "该属性需要布尔字面量值"))
}

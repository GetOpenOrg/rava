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

pub(crate) struct ClassInput {
    pub attrs: Vec<Attribute>,
    pub struct_ident: Ident,
    pub generics: syn::Generics,
    pub fields: Vec<(Ident, Type)>,
    pub fns: Vec<FnItem>,
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

pub(crate) fn parse_impl_fns(input: ParseStream) -> syn::Result<Vec<FnItem>> {
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
    /// 线性超类链（从最深祖先到直接父类），Rust short names，不含 Object 和 self。
    pub all_superclasses: Vec<String>,
    /// 每个祖先在本类视角下的类型实参（含尖括号，如 `<P_IN, P_OUT, Object>`）；非泛型祖先为空。
    /// 各祖先元数不同（`AbstractPipeline<A, B, S>` : `PipelineHelper<B>`），必须逐个祖先给出。
    pub ancestor_type_args: HashMap<String, proc_macro2::TokenStream>,
    /// 每个祖先自己声明的字段列表：{ancestor_rust_name → [field_names]}。
    pub ancestor_fields_layout: HashMap<String, Vec<String>>,
    pub all_supertypes: Vec<String>,
    pub is_interface: bool,
    pub has_to_string_method: bool,
    pub has_hash_code_method: bool,
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

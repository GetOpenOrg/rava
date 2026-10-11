//! §6 wrapper 装入 Object（S7-2b）：Object 直接持有句柄所持存储（运行时类对象），wrapper 不再
//! 实现 `ObjectVTable`、也不再经 `Rc<wrapper>` 包一层。按类只生成一行转交非泛型的
//! `__Handle::into_object(本类描述符)`：非 null 交出存储，null → 带本类描述符的类型化 null
//! （`__class_name` / `__desc` / `is_instance_of` 按静态类应答，数组元素的 null 探针据此取元素类）。

use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::super::context::GenContext;

pub(super) fn generate(ctx: &GenContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    if ctx.meta.binary_name.is_empty() {
        return quote! {};
    }
    quote! {
        impl #impl_g ::std::convert::From<#struct_ident #ty_g> for Object #where_c {
            #[inline]
            fn from(w: #struct_ident #ty_g) -> Object {
                w.__r.into_object(<#struct_ident #ty_g>::__DESC)
            }
        }
    }
}

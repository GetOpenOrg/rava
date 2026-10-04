//! §6 impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）：wrapper 只应答句柄（S7-2）、
//! `as_any`、描述符与按值的接口视图转交；运行时类应答的查询（身份 / 类名 / instanceof / hashCode / equals /
//! toString / 浅拷贝 / 代理 / 接口视图 / 按名字段协议）由 ObjectVTable 缺省实现经句柄转交
//! 句柄所持存储（运行时类 inner）。类型判定读描述符，类目标视图经 `From<Object>` 的句柄重建取得。

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
        impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
            fn __handle(&self) -> ::std::option::Option<&__Handle> {
                ::std::option::Option::Some(self.__r.handle())
            }
            fn as_any(&self) -> &dyn ::std::any::Any { self }
            // 按值 self：未移交的 self 经 ObjectVTable 擦除后释放，Arc<wrapper> 析构不逐类
            // 单态化（emitter-performance §5.5 N4）
            fn __interface(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                self.__r.handle().interface(slot);
                ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
            }
            // null 引用无存储：以静态类描述符应答（数组元素的 null 探针按源元素类型判定
            // 类名与超类型，S-4）；非 null 委托运行时类
            fn __desc(&self) -> ::std::option::Option<&'static __ClassDesc> {
                match self.__r.handle().target() {
                    ::std::option::Option::Some(__t) => __t.__desc(),
                    ::std::option::Option::None => ::std::option::Option::Some(Self::__DESC),
                }
            }
        }
    }
}

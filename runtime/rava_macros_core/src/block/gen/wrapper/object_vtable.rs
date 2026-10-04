//! §6 impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）：虚方法转发、擦除部件导出、
//! 按名字段协议（运行时类应答的查询经 `__view_target` 由缺省实现转交）。类型判定与视图不在
//! wrapper 上按类展开：instanceof / 类名读运行时类描述符，类目标视图经 `From<Object>` 的擦除
//! 重建取得（S7-1）。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Type;

use super::super::super::util::{is_basic, type_is_bool, type_is_int, type_is_long, type_is_word, type_is_dword};
use super::super::context::GenContext;
use super::super::storage_hooks::hook_ident;

pub(super) fn generate(ctx: &GenContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let cells = hook_ident(ctx, "cells");
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let binary_name = &ctx.meta.binary_name;

    // ══════════════════════════════════════════════════════════════════════════
    // 6. impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）
    // ══════════════════════════════════════════════════════════════════════════

    if !binary_name.is_empty() {
        // 按名字段协议（Unsafe 实例字段 long / int / boolean 共享单元、引用槽访问）：
        // 字段名单与分派臂只在 inner 侧生成（inner 平铺持有全部继承字段）。wrapper 侧
        // 先问静态类 inner（any 能 downcast 为本类 inner 时），未命中（any 是运行时子类
        // inner——静态基类视图，如 AQS 视图承载 CountDownLatch$Sync；或字段不在本类名单）
        // 委托 vtable 对象（= 运行时类 inner）的同名覆盖应答（与 `__erased_vtable` 的
        // 委托同型）。本类无该类字段 → inner 不应答，直接委托。
        let flat_fields = || ctx.meta.superclass_fields.iter()
            .map(|(n, t)| (n, t, ctx.inherited_is_basic(n, t)))
            .chain(ctx.fields.iter().map(|(n, t)| (n, t, is_basic(t))));
        let has_prim = |pred: fn(&Type) -> bool| flat_fields()
            .any(|(n, t, _)| !ctx.is_erased(n) && pred(t));
        let cell_query = |method: &str, prim: TokenStream2, has: bool| -> TokenStream2 {
            let method = format_ident!("{}", method);
            let body = if has {
                quote! {
                    ::std::option::Option::or_else(
                        ::std::option::Option::and_then(
                            #cells(&self.any),
                            |i| ObjectVTable::#method(i, field)),
                        || ObjectVTable::#method(&*self.vtable, field))
                }
            } else {
                return quote! {};
            };
            quote! {
                fn #method(
                    &self,
                    field: &str,
                ) -> ::std::option::Option<__Shared<__PrimCell<#prim>>> {
                    #body
                }
            }
        };
        let long_cell_query = cell_query("__unsafe_long_cell", quote! { i64 }, has_prim(type_is_long));
        let int_cell_query = cell_query("__unsafe_int_cell", quote! { i32 }, has_prim(type_is_int));
        let bool_cell_query = cell_query("__unsafe_bool_cell", quote! { bool }, has_prim(type_is_bool));
        // 字 / 双字视图：静态类 inner 先应答，未命中委托 vtable 对象（与 cell_query 同型）
        let view_query = |method: &str, w_ty: TokenStream2, present: bool| -> TokenStream2 {
            let method = format_ident!("{}", method);
            if !present {
                return quote! {};
            }
            let inner = Some(quote! {
                if let ::std::option::Option::Some(i) = #cells(&self.any) {
                    if let __r @ ::std::option::Option::Some(_) = ObjectVTable::#method(i, field, op) {
                        return __r;
                    }
                }
            });
            quote! {
                fn #method(
                    &self,
                    field: &str,
                    op: &mut dyn FnMut(#w_ty) -> ::std::option::Option<#w_ty>,
                ) -> ::std::option::Option<#w_ty> {
                    #inner
                    ObjectVTable::#method(&*self.vtable, field, op)
                }
            }
        };
        let word_query = view_query("__unsafe_word", quote! { i32 }, has_prim(type_is_word));
        let dword_query = view_query("__unsafe_dword", quote! { i64 }, has_prim(type_is_dword));
        let has_ref = flat_fields().any(|(n, _, basic)| ctx.is_erased(n) || !basic);
        let ref_access_query = has_ref.then(|| quote! {
            fn __unsafe_ref_access(
                &self, field: &str, op: &mut __RefAccess<'_>,
            ) -> ::std::option::Option<Object> {
                if let ::std::option::Option::Some(i) = #cells(&self.any) {
                    if let __r @ ::std::option::Option::Some(_) =
                        ObjectVTable::__unsafe_ref_access(i, field, op)
                    {
                        return __r;
                    }
                }
                ObjectVTable::__unsafe_ref_access(&*self.vtable, field, op)
            }
        });
        quote! {
            impl #impl_g ObjectVTable for #struct_ident #ty_g #where_c {
                // 运行时类应答的查询（身份 / 类名 / instanceof / hashCode / equals / toString /
                // 浅拷贝 / 代理 / 按名字段协议的委托部分）由 ObjectVTable 缺省实现经此转交
                // vtable 对象（= 运行时类 inner）
                fn __view_target(&self) -> ::std::option::Option<&dyn ObjectVTable> {
                    ::std::option::Option::Some(&*self.vtable)
                }
                fn as_any(&self) -> &dyn ::std::any::Any { self }
                fn is_jvm_null(&self) -> bool { self._jvm_null }
                // 按值 self 的钩子：未移交的 self 经 ObjectVTable 擦除后释放，Arc<wrapper>
                // 析构不逐类单态化（emitter-performance §5.5 N4；inner 侧同型）
                fn __interface(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    ObjectVTable::__interface(__Shared::clone(&self.vtable), slot);
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                // 擦除存储导出（A-1）：wrapper 持有的非泛型 `Rc<X__inner>`。
                // `From<Object> for X<A>` 的擦除路径据此对任意类型实参重建视图。
                fn __erased_inner(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<__AnyRef>>()
                    {
                        *s = ::std::option::Option::Some(__Shared::clone(&self.any));
                    }
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                // 擦除 vtable 导出（A-1 部件形态）：整体委托 vtable 对象（= 运行时类 inner）。
                // inner 按运行时类应答本类与全部祖先类槽位（struct_layout `erased_vtable_query`），
                // 静态类 X 及其祖先都是运行时类的祖先，槽位集合是 inner 应答集合的子集，填入的
                // 是同一对象——wrapper 侧不再按静态类逐祖先展开臂。供 `From<Object> for X<A>`
                // 重建「运行时类是本类或其子类」的任意实例化视图，及中间型 catch_as 擦除重建。
                fn __erased_vtable(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    ObjectVTable::__erased_vtable(
                        __Shared::clone(&self.vtable) as __Shared<dyn ObjectVTable>, slot);
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                #long_cell_query
                #int_cell_query
                #bool_cell_query
                #word_query
                #dword_query
                #ref_access_query
            }
        }
    } else {
        quote! {}
    }
}

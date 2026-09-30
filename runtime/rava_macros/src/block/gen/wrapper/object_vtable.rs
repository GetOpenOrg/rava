//! §6 impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）：虚方法转发、擦除部件导出、
//! 运行时类视图、浅拷贝、按名字段协议。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::Type;

use super::super::super::util::{is_basic, type_is_bool, type_is_int, type_is_long};
use super::super::context::GenContext;

pub(super) fn generate(ctx: &GenContext) -> TokenStream2 {
    let struct_ident = &ctx.struct_ident;
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let impl_g = &ctx.impl_g;
    let ty_g = &ctx.ty_g;
    let where_c = &ctx.where_c;
    let phantom_init = &ctx.phantom_init;
    let binary_name = &ctx.meta.binary_name;

    // ══════════════════════════════════════════════════════════════════════════
    // 6. impl ObjectVTable for Wrapper（R-1 blanket From<T> 需要）
    // ══════════════════════════════════════════════════════════════════════════

    if !binary_name.is_empty() {
        // toString 在 Java 恒为虚方法：wrapper 一律把字符串化经 vtable 分派到运行时类
        // （祖先视图（From<Child> for Ancestor）的 wrapper 由此获得多态 toString——
        // 如 Number 视图转发 Integer 的 toString；未覆盖类落到 __inner 的默认
        // type_name，与既有输出一致。hashCode/equals 同此形态，本就无条件转发）。
        let to_string_fwd: TokenStream2 = quote! {
            fn __obj_str(&self) -> ::std::string::String {
                ObjectVTable::__obj_str(&*self.vtable)
            }
            fn __to_string(&self) -> Result<::std::string::String> {
                ObjectVTable::__to_string(&*self.vtable)
            }
        };
        let hash_code_fwd: TokenStream2 = quote! {
            fn hashCode(&self) -> i32 { ObjectVTable::hashCode(&*self.vtable) }
            fn equals(&self, other: Object) -> Result<bool> {
                ObjectVTable::equals(&*self.vtable, other)
            }
        };
        // 擦除 vtable 导出的祖先槽位（类祖先的 vtable trait 名；vtable 非泛型，
        // 超类链是其 supertrait —— 子类 vtable 可直接上转）
        let ancestor_vtable_idents: Vec<syn::Ident> = ctx.meta.all_superclasses.iter()
            .map(|anc_name| format_ident!("{}__VTable", anc_name))
            .collect();
        // 运行时类视图（A-1 后落在 wrapper 侧：Object 直接持有 wrapper，类型实参只在
        // wrapper 的 impl 上下文可见）：按 binary name 重建本类 / 任一祖先类型的 wrapper。
        // 祖先视图 = 宏生成的 From<Self> for Ancestor（vtable trait upcasting，保持运行时类）。
        // 异常对象以静态类型（如 Throwable）抛出后，catch 需要按运行时类还原为 catch 类型。
        let view_as_arms: Vec<TokenStream2> = std::iter::once(quote! {
            if type_id == #binary_name {
                return ::std::option::Option::Some(
                    ::std::boxed::Box::new(::std::clone::Clone::clone(self)));
            }
        }).chain(ctx.meta.all_superclasses.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            quote! {
                if type_id == <#anc_ident #atag>::BINARY_NAME {
                    let view: #anc_ident #atag = <#anc_ident #atag as ::std::convert::From<Self>>::from(
                        ::std::clone::Clone::clone(self));
                    return ::std::option::Option::Some(::std::boxed::Box::new(view));
                }
            }
        })).collect();
        // 同一组视图的类型驱动形式（`Object::downcast::<T>()`）：slot 为 `Option<本类/祖先 wrapper>` 时写入
        let view_into_arms: Vec<TokenStream2> = std::iter::once(quote! {
            if let ::std::option::Option::Some(s) =
                slot.downcast_mut::<::std::option::Option<Self>>()
            {
                *s = ::std::option::Option::Some(::std::clone::Clone::clone(self));
                return true;
            }
        }).chain(ctx.meta.all_superclasses.iter().map(|anc_name| {
            let anc_ident = format_ident!("{}", anc_name);
            let atag = ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            quote! {
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<#anc_ident #atag>>()
                {
                    *s = ::std::option::Option::Some(
                        <#anc_ident #atag as ::std::convert::From<Self>>::from(
                            ::std::clone::Clone::clone(self)));
                    return true;
                }
            }
        })).chain(ctx.meta.iface_carrier_views.iter().filter_map(|iface_ty| {
            // A-4 批次 6：接口载体臂——JLS 4.10.3 的子类型关系含接口（数组协变与
            // try_checkcast::<载体> 按此判定），祖先臂只覆盖父类链。成员清单由
            // codegen 过滤并给出擦除载体形态（非泛型裸短名 / 泛型 `I<Object, ..>`，
            // 闭包外接口无生成载体类型，不在此列）；填充走 From<Object> 的载体包装
            // （非受检视图——对象身份保持，分派经运行时类 itable）。
            // UFCS 必须显式：接口载体可能自带 Java `static from(..)` 工厂方法，
            // `Iface::from(..)` 路径解析会被固有方法遮蔽（同 interface_gen upcast）。
            let ty = syn::parse_str::<Type>(iface_ty).ok()?;
            Some(quote! {
                if let ::std::option::Option::Some(s) =
                    slot.downcast_mut::<::std::option::Option<#ty>>()
                {
                    *s = ::std::option::Option::Some(
                        <#ty as ::std::convert::From<Object>>::from(
                            <Object as ::std::convert::From<Self>>::from(
                                ::std::clone::Clone::clone(self))));
                    return true;
                }
            })
        })).collect();
        // Object.clone() 的逐字段浅拷贝：新对象（新标识单元），每个字段新建存储单元，
        // 值按 Java 语义拷贝（基本类型拷贝值，引用类型拷贝引用）。经 wrapper 访问器
        // 完成拷贝（擦除字段的转换在访问器边界发生，与直连字段拷贝等价）。
        let copy_stmts: Vec<TokenStream2> = ctx.meta.superclass_fields.iter()
            .chain(ctx.fields.iter())
            .map(|(name, _)| {
                let get = format_ident!("__get_{}", name);
                let set = format_ident!("__set_{}", name);
                quote! { __copy.#set(self.#get()); }
            })
            .collect();
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
                            self.any.downcast_ref::<#inner_ident>(),
                            |i| ObjectVTable::#method(i, field)),
                        || ObjectVTable::#method(&*self.vtable, field))
                }
            } else {
                quote! { ObjectVTable::#method(&*self.vtable, field) }
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
        let has_ref = flat_fields().any(|(n, _, basic)| ctx.is_erased(n) || !basic);
        let ref_access_inner = if has_ref {
            quote! {
                if let ::std::option::Option::Some(i) = self.any.downcast_ref::<#inner_ident>() {
                    if let __r @ ::std::option::Option::Some(_) =
                        ObjectVTable::__unsafe_ref_access(i, field, op)
                    {
                        return __r;
                    }
                }
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
                fn is_jvm_null(&self) -> bool { self._jvm_null }
                // 按值 self 的钩子：未移交的 self 经 ObjectVTable 擦除后释放，Arc<wrapper>
                // 析构不逐类单态化（emitter-performance §5.5 N4；inner 侧同型）
                fn __interface(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    ObjectVTable::__interface(__Shared::clone(&self.vtable), slot);
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                fn __proxy_invoke(&self, iface: &str, name: &str, desc: &str, args: ::std::vec::Vec<Object>)
                    -> ::std::option::Option<Result<Object>> {
                    self.vtable.__proxy_invoke(iface, name, desc, args)
                }
                fn __class_name(&self) -> &'static str { self.vtable.__class_name() }
                fn __identity(&self) -> *const () { self.vtable.__identity() }
                /// 擦除存储导出（A-1）：wrapper 持有的非泛型 `Rc<X__inner>`。
                /// `From<Object> for X<A>` 的擦除路径据此对任意类型实参重建视图。
                fn __erased_inner(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<__AnyRef>>()
                    {
                        *s = ::std::option::Option::Some(__Shared::clone(&self.any));
                    }
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                /// 擦除 vtable 导出（A-1 部件形态）：按调用方 slot 的（擦除）类 vtable
                /// 类型把自身 vtable 填入——自身槽位直取；祖先类槽位经 supertrait 上转
                /// （类 vtable trait 非泛型，与类型实参无关）。与 `__erased_inner` 配对，
                /// 供 `From<Object> for X<A>` 重建「运行时类是本类或其子类」的任意实例化视图。
                fn __erased_vtable(self: __Shared<Self>, slot: &mut dyn ::std::any::Any) {
                    '__answered: {
                    if let ::std::option::Option::Some(s) =
                        slot.downcast_mut::<::std::option::Option<__Shared<dyn #vtable_trait_ident>>>()
                    {
                        *s = ::std::option::Option::Some(__Shared::clone(&self.vtable));
                        break '__answered;
                    }
                    #(
                        if let ::std::option::Option::Some(s) =
                            slot.downcast_mut::<::std::option::Option<__Shared<dyn #ancestor_vtable_idents>>>()
                        {
                            *s = ::std::option::Option::Some(
                                __Shared::clone(&self.vtable)
                                    as __Shared<dyn #ancestor_vtable_idents>);
                            break '__answered;
                        }
                    )*
                    // 静态类臂未命中 → 委托 vtable 对象（= 运行时类 inner）的擦除查询：
                    // 祖先视图包装（如 Throwable 视图承载 IllegalStateException inner）对
                    // 「运行时类自身/其祖先」槽位的请求由此应答——中间型（Throwable <
                    // catch T < 运行时 R）catch_as 的擦除重建 Path A。vtable trait 链根部
                    // 超 trait 即 ObjectVTable，上转恒可到达 inner 侧的覆盖。
                    ObjectVTable::__erased_vtable(
                        __Shared::clone(&self.vtable)
                            as __Shared<dyn ObjectVTable>,
                        slot,
                    );
                    }
                    ::std::mem::drop::<__Shared<dyn ObjectVTable>>(self);
                }
                fn __view_as(
                    &self,
                    _any: __AnyRef,
                    type_id: &str,
                ) -> ::std::option::Option<::std::boxed::Box<dyn ::std::any::Any>> {
                    #(#view_as_arms)*
                    ::std::option::Option::None
                }
                fn __view_into(
                    &self,
                    _any: __AnyRef,
                    slot: &mut dyn ::std::any::Any,
                ) -> bool {
                    #(#view_into_arms)*
                    false
                }
                fn __shallow_copy(&self) -> ::std::option::Option<Object> {
                    // 运行时类优先（C-1）：vtable 即运行时类 inner，其浅拷贝保留子类字段与
                    // 类名（静态基类视图 Point 承载 Deep 对象时得 Deep 副本）；未应答时
                    // 回退按本（静态）类逐字段拷贝
                    if let ::std::option::Option::Some(__o) = ObjectVTable::__shallow_copy(&*self.vtable) {
                        return ::std::option::Option::Some(__o);
                    }
                    let __rc = __Shared::new(<#inner_ident as ::std::default::Default>::default());
                    // 类型标注：vtable 去形参后字面量的字段不再提及本类形参——全部字段
                    // 为具体类型的类（E 无从钉住）会触发 E0283；以 Self 钉住
                    let __copy: Self = #struct_ident {
                        vtable: __Shared::clone(&__rc) as __Shared<dyn #vtable_trait_ident>,
                        any: __rc as __AnyRef,
                        _jvm_null: false,
                        #phantom_init
                    };
                    #(#copy_stmts)*
                    ::std::option::Option::Some(Object::from(__copy))
                }
                #long_cell_query
                #int_cell_query
                #bool_cell_query
                fn __unsafe_ref_access(
                    &self, field: &str, op: &mut __RefAccess<'_>,
                ) -> ::std::option::Option<Object> {
                    #ref_access_inner
                    ObjectVTable::__unsafe_ref_access(&*self.vtable, field, op)
                }
                #to_string_fwd
                #hash_code_fwd
            }
        }
    } else {
        quote! {}
    }
}

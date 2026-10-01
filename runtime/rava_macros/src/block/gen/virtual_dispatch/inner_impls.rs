//! `impl *__VTable for __inner`：字段访问器、覆盖方法、继承槽位、接口桥接。

use super::*;

/// §4 impl vtable traits for __inner（含实现的接口 `impl Iface__VTable for __inner` 桥接）。
pub(crate) fn vtable_impls(ctx: &GenContext) -> syn::Result<TokenStream2> {
    let struct_ident = &ctx.struct_ident;
    let as_self_hook = &ctx.as_self_hook;
    let inner_ident = &ctx.inner_ident;
    let vtable_trait_ident = &ctx.vtable_trait_ident;
    let erased_ty_args = &ctx.erased_ty_args;
    let phantom_init = &ctx.phantom_init;

    // 实现的接口：impl Iface__VTable for __inner（擦除签名 → 本类成员的桥接）
    let interface_impls: Vec<TokenStream2> = ctx.iface_impls.iter()
        .map(|ii| expand_interface_impl(ii, &ctx.struct_ident, &ctx.inner_ident, &ctx.vtable_trait_ident,
                                        &ctx.erased_ty_args, &ctx.phantom_init))
        .collect();

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

    // 字段访问器 impl 体（三个 vtable impl 生成点共用）。
    // 擦除字段（__inner 中以 Object 存储）：签名 Object 化（与 trait 声明一致），
    // impl 直连存储（Object 进 Object 出）；类型化转换移到 wrapper 委托（β' 边界）。
    let accessor_impl_items = |name: &syn::Ident, ty: &Type, basic: bool| -> TokenStream2 {
        let get = format_ident!("__get_{}", name);
        let set = format_ident!("__set_{}", name);
        if ctx.is_erased(name) {
            quote! {
                fn #get(&self) -> Object {
                    self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                }
                fn #set(&self, v: Object) {
                    *self.#name.borrow_mut() = ::std::option::Option::Some(
                        ::std::boxed::Box::new(v));
                }
            }
        } else if basic {
            quote! {
                fn #get(&self) -> #ty { self.#name.get() }
                fn #set(&self, v: #ty) { self.#name.set(v); }
            }
        } else {
            quote! {
                fn #get(&self) -> #ty {
                    self.#name.borrow().as_deref().map(Clone::clone).unwrap_or_default()
                }
                fn #set(&self, v: #ty) {
                    *self.#name.borrow_mut() = Some(::std::boxed::Box::new(v));
                }
            }
        }
    };

    if ctx.meta.superclass.is_none() {
        // ── 无父类：impl Self__VTable for __inner（非泛型，A-1 去形参）──────────
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in ctx.fields.iter() {
            own_accessor_impls.push(accessor_impl_items(name, ty, is_basic(ty)));
        }
        // VirtualDefine 方法体：Safe（仅非泛型类）→ 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &ctx.vtable_defines {
            if let Some(block) = &f.block {
                if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args {
                let __rc = __Shared::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as __Shared<dyn #vtable_trait_ident>,
                    any: __rc as __AnyRef,
                    _jvm_null: false,
                    #phantom_init
                }
            }
        });

        vtable_impls.push(quote! {
            impl #vtable_trait_ident for #inner_ident {
                #(#own_accessor_impls)*
            }
        });
    } else {
        // ── 有父类：顶层祖先 vtable impl + 其余空 impl + Self__VTable impl ──

        // 使用 all_superclasses（深度优先，最深祖先在前），已是 Rust short names
        let ancestors = ctx.meta.all_superclasses.clone();

        // 构建字段名 → (Ident, Type) 的查找表（superclass_fields 平铺列表）
        let sc_fields_map: HashMap<String, (Ident, Type)> = ctx.meta
            .superclass_fields
            .iter()
            .map(|(n, t)| (n.to_string(), (n.clone(), t.clone())))
            .collect();

        for anc_name in ancestors.iter() {
            let anc_vtable_ident = format_ident!("{}__VTable", anc_name);

            // 祖先 vtable 的类型实参（本类视角）：仅用于推导祖先的形参元数 ——
            // vtable 去形参（A-1）后 impl 头与钩子返回类型均按「该元数的全 Object
            // 擦除实例化」生成（祖先 trait 声明的钩子返回 `Anc<Object, ..>`）。
            let anc_vtable_args: TokenStream2 =
                ctx.meta.ancestor_type_args.get(anc_name).cloned().unwrap_or_default();
            let anc_erased_args = erased_args_of_same_arity(&anc_vtable_args);

            let mut items: Vec<TokenStream2> = Vec::new();

            // 将此祖先自己声明的字段 accessor 放进此 vtable impl
            let anc_own_field_names: Vec<String> =
                if let Some(names) = ctx.meta.ancestor_fields_layout.get(anc_name) {
                    names.clone()
                } else if ctx.meta.ancestor_fields_layout.is_empty() {
                    // 兜底：无 ancestor_fields_layout 时（旧属性），所有字段放最深祖先（首位）
                    if anc_name == ancestors.first().map(|s| s.as_str()).unwrap_or("") {
                        ctx.meta.superclass_fields.iter().map(|(n, _)| n.to_string()).collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                };

            for field_name in &anc_own_field_names {
                if let Some((name, ty)) = sc_fields_map.get(field_name) {
                    items.push(accessor_impl_items(name, ty, ctx.inherited_is_basic(name, ty)));
                }
            }

            // VirtualOverride 方法：virtual_in == anc_name（签名随 trait 整体 Object 化）
            if let Some(override_fns) = ctx.vtable_overrides.get(anc_name) {
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    // 覆盖条目可能直挂方法体（Safe）→ 保留形参 mut（体可能赋值）；
                    // 声明祖先的类型形参位置（vtable_erasure 名集）一并擦除
                    let ov_erasure = erasure_set_of(f, &ctx.type_param_names);
                    let erased_item_sig = erase_item_signature_with(
                        sig, &ctx.type_param_names, &ov_erasure);
                    // K-6（覆盖条目沿用继承成员的解耦机制）：wrapper 成员名（本类重载态）
                    // ≠ 槽位 trait 成员名（声明者态）时，按 vtable_name 属性改写槽位
                    // 条目名——trait impl 与声明者的槽位声明逐字一致
                    let mut erased_item_sig = erased_item_sig;
                    if let Some(vn) = attr_str(&f.attrs, "vtable_name") {
                        erased_item_sig.ident = Ident::new(&vn, proc_macro2::Span::call_site());
                    }
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                            if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                                // K-6b：参数位擦除还原——槽位签名被 vtable_erasure /
                                // 类型形参提及 Object 化的形参，体开头以类型化局部
                                // 遮蔽（From<Object> 还原，与 NeedsWrapper 路径的
                                // erased_impl_call 同一边界转换语义）。判定与
                                // erase_signature_with 同一谓词的镜像（逐位比较原
                                // 签名 / 擦除签名 token）：定义侧「槽位签名」与
                                // 「体执行形态」不再各自决定，调用侧 coerce 预期
                                // （wrapper 公开方法的类型化签名）与之同源。
                                let arg_restores: Vec<TokenStream2> = sig.inputs.iter()
                                    .zip(erased_item_sig.inputs.iter())
                                    .filter_map(|(o, e)| {
                                        let (syn::FnArg::Typed(opt), syn::FnArg::Typed(ept)) = (o, e) else {
                                            return None;
                                        };
                                        let syn::Pat::Ident(opi) = &*opt.pat else {
                                            return None;
                                        };
                                        if same_type_tokens(&opt.ty, &ept.ty) {
                                            return None;
                                        }
                                        let ident = &opi.ident;
                                        let mut_kw = if opi.mutability.is_some() {
                                            quote! { mut }
                                        } else {
                                            quote! {}
                                        };
                                        let orig_ty = &opt.ty;
                                        Some(quote! {
                                            let #mut_kw #ident: #orig_ty =
                                                <#orig_ty as ::std::convert::From<Object>>::from(#ident);
                                        })
                                    })
                                    .collect();
                                // K-6a：擦除命中返回位（协变返回 / 具体实参位置 →
                                // Object）时，Safe 体直挂须在返回位置装箱——体本身
                                // 返回子类精确类型，与槽位的 Object 返回不一致。
                                // 闭包包裹：body 内的 return / ? 语义不变（闭包返回
                                // 原签名的 Result<精确类型>），出口统一 Into<Object>
                                let orig_ret_inner: Option<&Type> = match &sig.output {
                                    syn::ReturnType::Type(_, ty) => result_inner_ty(ty),
                                    syn::ReturnType::Default => None,
                                };
                                let erased_ret_inner: Option<&Type> = match &erased_item_sig.output {
                                    syn::ReturnType::Type(_, ty) => result_inner_ty(ty),
                                    syn::ReturnType::Default => None,
                                };
                                let ret_erased_hit = match (orig_ret_inner, erased_ret_inner) {
                                    (Some(oi), Some(ei)) => !same_type_tokens(oi, ei),
                                    _ => false,
                                };
                                if let (true, Some(orig_inner)) = (ret_erased_hit, orig_ret_inner) {
                                    items.push(quote! {
                                        #(#keep_attrs)*
                                        #erased_item_sig {
                                            #(#arg_restores)*
                                            // Result 经 prelude 可见（java_runtime 与 user crate 同一形态）
                                            let __ret: Result<#orig_inner> = (|| #b)();
                                            Ok(::std::convert::Into::<Object>::into(__ret?))
                                        }
                                    });
                                } else if !arg_restores.is_empty() {
                                    items.push(quote! {
                                        #(#keep_attrs)*
                                        #erased_item_sig {
                                            #(#arg_restores)*
                                            #b
                                        }
                                    });
                                } else {
                                    items.push(quote! {
                                        #(#keep_attrs)*
                                        #erased_item_sig #b
                                    });
                                }
                            } else {
                                // 方法体需要 wrapper 上下文（this 传参 / 非虚方法调用 / Self::）：
                                // 方法体只落在 wrapper 的 `__impl_<method>` 上（见 wrapper 方法生成），
                                // 此处经擦除实例化的 wrapper 执行——与 VirtualDefine 同一路径，
                                // super 调用的 base 函数也复用它（不分派，精确命中本类实现）。
                                // impl 签名已擦除：形参 / 返回值在边界与擦除 wrapper 的
                                // `__impl_<method>`（类型实参全 Object）做 Object ↔ 类型化转换。
                                let impl_name = format_ident!("__impl_{}", sig.ident);
                                let wrapper_call = erased_wrapper_call(
                                    sig, &impl_name, &ctx.struct_ident, &ctx.vtable_trait_ident,
                                    &ctx.erased_ty_args, &ctx.phantom_init, &ctx.type_param_names,
                                    &ov_erasure,
                                );
                                items.push(quote! {
                                    #(#keep_attrs)*
                                    #erased_item_sig { #wrapper_call }
                                });
                            }
                        }
                        None if attr_str(&f.attrs, "body").as_deref() == Some("handwritten") => {
                            let impl_name = format_ident!("__impl_{}", sig.ident);
                            let wrapper_call = erased_wrapper_call(
                                sig, &impl_name, &ctx.struct_ident, &ctx.vtable_trait_ident,
                                &ctx.erased_ty_args, &ctx.phantom_init, &ctx.type_param_names,
                                &ov_erasure,
                            );
                            items.push(quote! {
                                #(#keep_attrs)*
                                #erased_item_sig { #wrapper_call }
                            });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let desc = attr_str(&f.attrs, "descriptor").unwrap_or_default();
                            let bin = &ctx.meta.binary_name;
                            let msg = format!("stub: {}.{}:{}", bin, mname, desc);
                            items.push(quote! {
                                #erased_item_sig { __stub(#msg) }
                            });
                        }
                    }
                }
            }

            // 继承成员填槽（S-16）：带转发体的继承声明把体放进本类对 vtable_owner 的
            // vtable impl。vtable_owner 声明为 abstract（实现位于中间祖先）时，trait
            // default 是 stub panic——不填槽则子类对象经 vtable 虚分派命中空洞声明。
            // 转发体由 emitter 生成（Owner__m_base / __as_Owner 钩子，super 调用同源）。
            for (f, _owner, vtable_owner, _interface_owner) in &ctx.inherited {
                let Some(block) = &f.block else { continue };
                let Some(vo) = vtable_owner else { continue };
                let vo_ty = match syn::parse_str::<Type>(vo) {
                    Ok(t) => t,
                    Err(e) => return Err(e),
                };
                let (vo_name, _) = split_type_name_args(&vo_ty);
                if vo_name != *anc_name {
                    continue;
                }
                let sig = &f.sig;
                let erasure = erasure_set_of(f, &ctx.type_param_names);
                let erased_item_sig = erase_signature_with(sig, &ctx.type_param_names, &erasure);
                // K-6：跨分支重载发散时 wrapper 成员名（接收者态）≠ vtable 槽位名
                //（声明者态）→ 按 vtable_name 属性改写槽位条目名（trait 成员名）
                let mut erased_item_sig = erased_item_sig;
                if let Some(vn) = attr_str(&f.attrs, "vtable_name") {
                    erased_item_sig.ident = Ident::new(&vn, proc_macro2::Span::call_site());
                }
                let conv = forward_conv_spec(sig, &erased_item_sig, &ctx.type_param_names, &erasure);
                let mut b = block.clone();
                // 槽位上下文不携带本类类型形参（vtable 去形参）→ 转发体的 base 调用
                // turbofish 里被删形参 / 擦除形态取 Object（base 函数体是参数化的，
                // 行为参数化一致）；`X__VTable<..>` 路径的实参整体剥除。
                let dropped: HashSet<String> = if ctx.class_is_generic {
                    ctx.gen.type_params().map(|tp| tp.ident.to_string()).collect()
                } else {
                    HashSet::new()
                };
                rewrite_dropped_params_in_inherited_body(&mut b, &dropped, &erasure, &conv);
                items.push(quote! { #erased_item_sig #b });
            }

            // 祖先 wrapper 重建钩子（经 __from_parts：祖先可能在另一个 crate，字段不可见）。
            // 返回祖先的擦除实例化（与祖先 trait 声明的钩子签名一致，A-1 去形参）。
            // turbofish：vtable 非泛型后 __from_parts 的祖先形参不再经实参类型钉住
            // （非泛型祖先无实参，返回类型可钉住，直接调用）。
            let anc_ident = format_ident!("{}", anc_name);
            let anc_hook = format_ident!("__as_{}", anc_name);
            let anc_from_parts: TokenStream2 = if anc_erased_args.is_empty() {
                quote! { #anc_ident::__from_parts }
            } else {
                quote! { #anc_ident::#anc_erased_args::__from_parts }
            };
            items.push(quote! {
                fn #anc_hook(&self) -> #anc_ident #anc_erased_args {
                    let __rc = __Shared::new(::std::clone::Clone::clone(self));
                    #anc_from_parts(
                        __rc.clone() as __Shared<dyn #anc_vtable_ident>,
                        __rc as __AnyRef,
                        false,
                    )
                }
            });

            vtable_impls.push(quote! {
                impl #anc_vtable_ident for #inner_ident {
                    #(#items)*
                }
            });
        }

        // Self__VTable impl（own ctx.fields 的 accessor）
        let mut own_accessor_impls: Vec<TokenStream2> = Vec::new();
        for (name, ty) in ctx.fields.iter() {
            own_accessor_impls.push(accessor_impl_items(name, ty, is_basic(ty)));
        }
        // VirtualDefine 方法体：Safe → 直接放入 vtable impl；
        // 其余（需要 wrapper 上下文）→ 不在此生成，走 trait default 的钩子路径
        for f in &ctx.vtable_defines {
            if let Some(block) = &f.block {
                if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let mut b = block.clone();
                    rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                    own_accessor_impls.push(quote! { #(#keep_attrs)* #sig #b });
                }
            }
        }
        own_accessor_impls.push(quote! {
            fn #as_self_hook(&self) -> #struct_ident #erased_ty_args {
                let __rc = __Shared::new(::std::clone::Clone::clone(self));
                #struct_ident {
                    vtable: __rc.clone() as __Shared<dyn #vtable_trait_ident>,
                    any: __rc as __AnyRef,
                    _jvm_null: false,
                    #phantom_init
                }
            }
        });

        vtable_impls.push(quote! {
            impl #vtable_trait_ident for #inner_ident {
                #(#own_accessor_impls)*
            }
        });

        // 处理未被 ancestors 列表覆盖的 VirtualOverride（vtable_class 不在祖先中）
        // 这种情况理论上不应出现，但做兜底生成
        for (vtable_class, override_fns) in &ctx.vtable_overrides {
            if !ancestors.contains(vtable_class) {
                let anc_vtable_ident = format_ident!("{}__VTable", vtable_class);
                let mut items: Vec<TokenStream2> = Vec::new();
                for f in override_fns {
                    let sig = &f.sig;
                    let keep_attrs = strip_meta_attrs(&f.attrs);
                    let erased_item_sig = erase_signature(sig, &ctx.type_param_names);
                    match &f.block {
                        Some(block) => {
                            let mut b = block.clone();
                            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
                            items.push(quote! { #(#keep_attrs)* #erased_item_sig #b });
                        }
                        None => {
                            let mname = sig.ident.to_string();
                            let msg = format!("stub: {}.{}", ctx.meta.binary_name, mname);
                            items.push(quote! { #erased_item_sig { __stub(#msg) } });
                        }
                    }
                }
                vtable_impls.push(quote! {
                    impl #anc_vtable_ident for #inner_ident {
                        #(#items)*
                    }
                });
            }
        }
    }

    Ok(quote! {
        #(#interface_impls)*
        #(#vtable_impls)*
    })
}

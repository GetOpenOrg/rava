//! 剥体标注（docs/plans/2026-10-01-rustc-memory-and-crate-split.md §7.5.2）：声明层文本里
//! 已下沉方法的体换成 `;`，方法项带 `#[rava_moved = "<标注>"]`。标注就是声明模式展开读方法体
//! 得到的全部结论；声明模式按标注走与有体时相同的分支，不再接收体记号。
//!
//! 判定 [`moved_fact`] 由宏与生成器共用（生成器经 `plan` 特性对同一份块文本调用）。两侧在
//! 不同记号实现（编译器 / proc_macro2 回落）下运行，结论一致性由实现模式的常量断言兜底：
//! 声明模式把读到的标注序列取摘要写成常量，实现模式对完整块重算并断言相等。

use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

use super::classify::{vtable_body_kind_gated, VTableBodyKind};
use super::erasure::prepare_non_virtual_body;
use super::gen::context::GenContext;
use super::gen::virtual_dispatch::define_base_has_body;
use super::gen::wrapper::functionize_applicable;
use super::parse::FnItem;
use super::rewrite::{rewrite_base_calls_for_wrapper, rewrite_block, rewrite_virtual_calls_for_wrapper};
use super::util::{classify_method, MethodKind};

/// 方法体下沉标注
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Moved {
    /// 虚方法，gated 分类 Safe，base 函数为真实体
    Safe,
    /// 虚方法，gated 分类 Safe，base 函数为存根
    SafeStub,
    /// 虚方法，需 wrapper 上下文，`__impl_` 已函数化
    Wrapper,
    /// 构造器 / 非虚方法，已函数化
    Plain,
}

impl Moved {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Moved::Safe => "safe",
            Moved::SafeStub => "safe_stub",
            Moved::Wrapper => "wrapper",
            Moved::Plain => "plain",
        }
    }

    pub(crate) fn parse(s: &str) -> Option<Moved> {
        match s {
            "safe" => Some(Moved::Safe),
            "safe_stub" => Some(Moved::SafeStub),
            "wrapper" => Some(Moved::Wrapper),
            "plain" => Some(Moved::Plain),
            _ => None,
        }
    }
}

/// 方法有体（在本文本里，或已下沉）
pub(crate) fn has_body(f: &FnItem) -> bool {
    f.block.is_some() || f.moved.is_some()
}

/// 有体方法的 gated 分类是否 Safe（调用方保证 [`has_body`]）
pub(crate) fn is_safe(ctx: &GenContext, f: &FnItem) -> bool {
    match (&f.block, f.moved) {
        (Some(block), _) => matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe),
        (None, Some(m)) => matches!(m, Moved::Safe | Moved::SafeStub),
        (None, None) => false,
    }
}

/// 有体方法在声明模式下的下沉标注；None = 体在声明模式下被消费（内联进 wrapper、
/// 泛型 base 真实体等），留在声明层。
///
/// 与声明模式各读体位置逐一对应（§7.5.2 表）：
/// - 虚方法 Safe：wrapper 不放体；缺省方法走 base；base 真实体非泛型时拆入实现层（声明层只有
///   外壳），存根 base 不读体。方法自身无泛型形参 / where 子句时 base 非泛型（类非泛型）。
/// - 虚方法非 Safe：`__impl_` 函数化则声明层只有外壳；base 走钩子不读体。
/// - 构造器 / 非虚方法：函数化则声明层只有外壳。
pub(crate) fn moved_fact(ctx: &GenContext, f: &FnItem) -> Option<Moved> {
    if ctx.class_is_generic {
        return None;
    }
    let block = f.block.as_ref()?;
    if !f.sig.generics.params.is_empty() || f.sig.generics.where_clause.is_some() {
        return None;
    }
    match classify_method(&f.attrs, &f.sig, &ctx.self_name) {
        MethodKind::Inherited { .. } => None,
        MethodKind::VirtualDefine | MethodKind::VirtualOverride { .. } => {
            if matches!(vtable_body_kind_gated(block, ctx.class_is_generic), VTableBodyKind::Safe) {
                return Some(if define_base_has_body(ctx, f) { Moved::Safe } else { Moved::SafeStub });
            }
            // 与 wrapper/methods.rs 的 `__impl_` 路径同一改写序列
            let mut b = block.clone();
            rewrite_block(&mut b, &ctx.basic_names, &ctx.ref_names);
            rewrite_base_calls_for_wrapper(&mut b);
            rewrite_virtual_calls_for_wrapper(&mut b, &ctx.own_method_names, &ctx.vdispatch);
            let mut impl_sig = f.sig.clone();
            impl_sig.ident = format_ident!("__impl_{}", f.sig.ident);
            functionize_applicable(ctx, &impl_sig, &b.stmts).then_some(Moved::Wrapper)
        }
        MethodKind::Constructor | MethodKind::NonVirtual => {
            let (_, b) = prepare_non_virtual_body(f, &ctx.basic_names, &ctx.ref_names, &ctx.own_statics())?;
            functionize_applicable(ctx, &f.sig, &b.stmts).then_some(Moved::Plain)
        }
    }
}

/// 标注序列文本：`<方法序号>:<标注>;`（只列有标注的方法）
fn fact_text(facts: impl Iterator<Item = (usize, Moved)>) -> String {
    facts.map(|(i, m)| format!("{i}:{};", m.as_str())).collect()
}

/// FNV-1a 64
fn fnv1a(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.bytes() {
        h ^= u64::from(b);
        h = h.wrapping_mul(0x0100_0000_01b3);
    }
    h
}

fn digest_ident(ctx: &GenContext) -> syn::Ident {
    format_ident!("__RAVA_MOVED_{}", ctx.struct_ident)
}

/// 声明模式：本块读到的标注摘要常量
pub(crate) fn decl_digest(ctx: &GenContext) -> TokenStream2 {
    let text = fact_text(ctx.fns.iter().enumerate().filter_map(|(i, f)| f.moved.map(|m| (i, m))));
    let h = fnv1a(&text);
    let name = digest_ident(ctx);
    quote! {
        #[allow(non_upper_case_globals)]
        pub const #name: u64 = #h;
    }
}

/// 实现模式：对完整块重算标注，断言与声明层常量一致
pub(crate) fn body_assert(ctx: &GenContext) -> TokenStream2 {
    let text = fact_text(ctx.fns.iter().enumerate().filter_map(|(i, f)| moved_fact(ctx, f).map(|m| (i, m))));
    let h = fnv1a(&text);
    let name = digest_ident(ctx);
    let msg = format!("rava_moved：{} 声明层剥体标注与方法体判定不一致", ctx.meta.binary_name);
    quote! {
        const _: () = ::std::assert!(#name == #h, #msg);
    }
}

/// 全部有体方法的标注（生成器剥体用；序号 = 方法在块内的位置）
#[cfg(feature = "plan")]
pub(crate) fn plan_facts(ctx: &GenContext) -> Vec<(usize, Moved)> {
    ctx.fns.iter().enumerate().filter_map(|(i, f)| moved_fact(ctx, f).map(|m| (i, m))).collect()
}

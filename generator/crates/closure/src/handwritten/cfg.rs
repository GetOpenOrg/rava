//! 手写层条件编译（`#[cfg(...)]`）按构建单元的参考 JDK 求值。
//!
//! 手写层以编译期 cfg 区分签名随 JDK 版本变化的成员（runtime/java_runtime/build.rs 按 `jdk_feature.txt`
//! 注入 `jdk_ge_<N>`）。扫描器所见须与编译器所编同源：cfg 判定为假的项、impl / trait 成员、语句、表达式、
//! match 分支与字段在扫描前剔除，否则另一 JDK 版本的分支也被建模（同名 fn 两个体合并、按名写入的字段
//! 候选放宽，闭包随之膨胀）。
//!
//! 求值是三值的：认得的谓词是 `jdk_ge_<N>`（参考 JDK 主版本 ≥ N）与组合子 `not` / `all` / `any`；
//! 其余谓词（`debug_assertions`、`test`、`target_*`、`feature = …` 等）与参考 JDK 未知时一律「不定」，
//! 不定的项保留（宁宽勿漏）。

use syn::punctuated::Punctuated;
use syn::visit_mut::VisitMut;
use syn::{Attribute, Expr, Meta, Token};

/// 参考 JDK 谓词前缀：`jdk_ge_25` ⇔ 参考 JDK 主版本 ≥ 25
const JDK_GE_PREFIX: &str = "jdk_ge_";

/// 手写层 cfg 求值环境
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct HwCfg {
    /// 参考 JDK 主版本；None = 未知（jdk 谓词不定，全部保留）
    pub jdk_major: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Truth {
    True,
    False,
    Unknown,
}

impl HwCfg {
    pub fn jdk(major: u32) -> Self {
        HwCfg { jdk_major: Some(major) }
    }

    /// 解析手写文件并剔除 cfg 判定为假的部分
    pub fn parse_file(self, content: &str) -> syn::Result<syn::File> {
        let mut file = syn::parse_file(content)?;
        self.strip(&mut file);
        Ok(file)
    }

    pub fn strip(self, file: &mut syn::File) {
        if self.jdk_major.is_some() {
            Strip(self).visit_file_mut(file);
        }
    }

    fn eval(self, meta: &Meta) -> Truth {
        match meta {
            Meta::Path(p) => {
                let Some(id) = p.get_ident().map(|i| i.to_string()) else { return Truth::Unknown };
                let (Some(n), Some(major)) = (id.strip_prefix(JDK_GE_PREFIX).and_then(|n| n.parse::<u32>().ok()), self.jdk_major)
                else {
                    return Truth::Unknown;
                };
                if major >= n {
                    Truth::True
                } else {
                    Truth::False
                }
            }
            Meta::List(l) => {
                let Ok(args) = l.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) else {
                    return Truth::Unknown;
                };
                let vals: Vec<Truth> = args.iter().map(|m| self.eval(m)).collect();
                let op = l.path.get_ident().map(|i| i.to_string());
                match op.as_deref() {
                    Some("not") if vals.len() == 1 => match vals[0] {
                        Truth::True => Truth::False,
                        Truth::False => Truth::True,
                        Truth::Unknown => Truth::Unknown,
                    },
                    Some("all") => {
                        if vals.contains(&Truth::False) {
                            Truth::False
                        } else if vals.iter().all(|v| *v == Truth::True) {
                            Truth::True
                        } else {
                            Truth::Unknown
                        }
                    }
                    Some("any") => {
                        if vals.contains(&Truth::True) {
                            Truth::True
                        } else if vals.iter().all(|v| *v == Truth::False) {
                            Truth::False
                        } else {
                            Truth::Unknown
                        }
                    }
                    _ => Truth::Unknown,
                }
            }
            Meta::NameValue(_) => Truth::Unknown,
        }
    }

    /// 属性表中有判定为假的 `#[cfg(...)]`（多个 cfg 属性为合取）
    fn disabled(self, attrs: &[Attribute]) -> bool {
        attrs.iter().filter(|a| a.path().is_ident("cfg")).any(|a| match a.parse_args::<Meta>() {
            Ok(m) => self.eval(&m) == Truth::False,
            Err(_) => false,
        })
    }
}

fn item_attrs(i: &syn::Item) -> &[Attribute] {
    use syn::Item::*;
    match i {
        Const(x) => &x.attrs,
        Enum(x) => &x.attrs,
        ExternCrate(x) => &x.attrs,
        Fn(x) => &x.attrs,
        ForeignMod(x) => &x.attrs,
        Impl(x) => &x.attrs,
        Macro(x) => &x.attrs,
        Mod(x) => &x.attrs,
        Static(x) => &x.attrs,
        Struct(x) => &x.attrs,
        Trait(x) => &x.attrs,
        TraitAlias(x) => &x.attrs,
        Type(x) => &x.attrs,
        Union(x) => &x.attrs,
        Use(x) => &x.attrs,
        _ => &[],
    }
}

fn impl_item_attrs(i: &syn::ImplItem) -> &[Attribute] {
    match i {
        syn::ImplItem::Const(x) => &x.attrs,
        syn::ImplItem::Fn(x) => &x.attrs,
        syn::ImplItem::Type(x) => &x.attrs,
        syn::ImplItem::Macro(x) => &x.attrs,
        _ => &[],
    }
}

fn trait_item_attrs(i: &syn::TraitItem) -> &[Attribute] {
    match i {
        syn::TraitItem::Const(x) => &x.attrs,
        syn::TraitItem::Fn(x) => &x.attrs,
        syn::TraitItem::Type(x) => &x.attrs,
        syn::TraitItem::Macro(x) => &x.attrs,
        _ => &[],
    }
}

fn expr_attrs(e: &Expr) -> &[Attribute] {
    use syn::Expr::*;
    match e {
        Array(x) => &x.attrs,
        Assign(x) => &x.attrs,
        Async(x) => &x.attrs,
        Await(x) => &x.attrs,
        Binary(x) => &x.attrs,
        Block(x) => &x.attrs,
        Break(x) => &x.attrs,
        Call(x) => &x.attrs,
        Cast(x) => &x.attrs,
        Closure(x) => &x.attrs,
        Const(x) => &x.attrs,
        Continue(x) => &x.attrs,
        Field(x) => &x.attrs,
        ForLoop(x) => &x.attrs,
        Group(x) => &x.attrs,
        If(x) => &x.attrs,
        Index(x) => &x.attrs,
        Infer(x) => &x.attrs,
        Let(x) => &x.attrs,
        Lit(x) => &x.attrs,
        Loop(x) => &x.attrs,
        Macro(x) => &x.attrs,
        Match(x) => &x.attrs,
        MethodCall(x) => &x.attrs,
        Paren(x) => &x.attrs,
        Path(x) => &x.attrs,
        Range(x) => &x.attrs,
        Reference(x) => &x.attrs,
        Repeat(x) => &x.attrs,
        Return(x) => &x.attrs,
        Struct(x) => &x.attrs,
        Try(x) => &x.attrs,
        TryBlock(x) => &x.attrs,
        Tuple(x) => &x.attrs,
        Unary(x) => &x.attrs,
        Unsafe(x) => &x.attrs,
        While(x) => &x.attrs,
        Yield(x) => &x.attrs,
        _ => &[],
    }
}

fn stmt_attrs(s: &syn::Stmt) -> &[Attribute] {
    match s {
        syn::Stmt::Local(l) => &l.attrs,
        syn::Stmt::Item(i) => item_attrs(i),
        syn::Stmt::Expr(e, _) => expr_attrs(e),
        syn::Stmt::Macro(m) => &m.attrs,
    }
}

/// 逐层剔除 cfg 为假的节点（先剔除再下行，被剔除子树不再访问）
struct Strip(HwCfg);

impl Strip {
    fn retain_fields<P>(&self, fields: &mut Punctuated<syn::Field, P>)
    where
        P: Default,
    {
        let kept: Vec<syn::Field> = std::mem::take(fields).into_iter().filter(|f| !self.0.disabled(&f.attrs)).collect();
        fields.extend(kept);
    }
}

impl VisitMut for Strip {
    fn visit_file_mut(&mut self, f: &mut syn::File) {
        f.items.retain(|i| !self.0.disabled(item_attrs(i)));
        syn::visit_mut::visit_file_mut(self, f);
    }

    fn visit_item_mod_mut(&mut self, m: &mut syn::ItemMod) {
        if let Some((_, items)) = &mut m.content {
            items.retain(|i| !self.0.disabled(item_attrs(i)));
        }
        syn::visit_mut::visit_item_mod_mut(self, m);
    }

    fn visit_item_impl_mut(&mut self, i: &mut syn::ItemImpl) {
        i.items.retain(|x| !self.0.disabled(impl_item_attrs(x)));
        syn::visit_mut::visit_item_impl_mut(self, i);
    }

    fn visit_item_trait_mut(&mut self, t: &mut syn::ItemTrait) {
        t.items.retain(|x| !self.0.disabled(trait_item_attrs(x)));
        syn::visit_mut::visit_item_trait_mut(self, t);
    }

    fn visit_block_mut(&mut self, b: &mut syn::Block) {
        b.stmts.retain(|s| !self.0.disabled(stmt_attrs(s)));
        syn::visit_mut::visit_block_mut(self, b);
    }

    fn visit_expr_match_mut(&mut self, m: &mut syn::ExprMatch) {
        m.arms.retain(|a| !self.0.disabled(&a.attrs));
        syn::visit_mut::visit_expr_match_mut(self, m);
    }

    fn visit_expr_struct_mut(&mut self, s: &mut syn::ExprStruct) {
        let kept: Vec<syn::FieldValue> = std::mem::take(&mut s.fields).into_iter().filter(|f| !self.0.disabled(&f.attrs)).collect();
        s.fields.extend(kept);
        syn::visit_mut::visit_expr_struct_mut(self, s);
    }

    fn visit_fields_named_mut(&mut self, f: &mut syn::FieldsNamed) {
        self.retain_fields(&mut f.named);
        syn::visit_mut::visit_fields_named_mut(self, f);
    }

    fn visit_fields_unnamed_mut(&mut self, f: &mut syn::FieldsUnnamed) {
        self.retain_fields(&mut f.unnamed);
        syn::visit_mut::visit_fields_unnamed_mut(self, f);
    }

    fn visit_item_enum_mut(&mut self, e: &mut syn::ItemEnum) {
        let kept: Vec<syn::Variant> = std::mem::take(&mut e.variants).into_iter().filter(|v| !self.0.disabled(&v.attrs)).collect();
        e.variants.extend(kept);
        syn::visit_mut::visit_item_enum_mut(self, e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;
    use syn::visit::Visit;

    struct Idents(BTreeSet<String>);
    impl<'ast> Visit<'ast> for Idents {
        fn visit_ident(&mut self, i: &'ast proc_macro2::Ident) {
            self.0.insert(i.to_string());
        }
    }

    /// 剔除后余下的全部标识符
    fn stripped(jdk: Option<u32>, src: &str) -> BTreeSet<String> {
        let f = HwCfg { jdk_major: jdk }.parse_file(src).expect("可解析");
        let mut v = Idents(BTreeSet::new());
        v.visit_file(&f);
        v.0
    }

    fn has(s: &BTreeSet<String>, words: &[&str]) -> bool {
        words.iter().all(|w| s.contains(*w))
    }

    fn none(s: &BTreeSet<String>, words: &[&str]) -> bool {
        words.iter().all(|w| !s.contains(*w))
    }

    const SRC: &str = r#"
        #[cfg(not(jdk_ge_25))] fn a() { old(); }
        #[cfg(jdk_ge_25)] fn a() { new(); }
        #[cfg(debug_assertions)] fn d() {}
        #[cfg(all(jdk_ge_22, not(jdk_ge_25)))] fn mid() {}
        #[cfg(any(jdk_ge_25, test))] fn either() {}
        impl X {
            #[cfg(jdk_ge_25)] pub fn m(&self, x: i32) {}
            #[cfg(not(jdk_ge_25))] pub fn m(&self, x: i64) {}
            fn body(&self) {
                #[cfg(not(jdk_ge_25))]
                frames.set(i, v)?;
                #[cfg(jdk_ge_25)]
                { fill(frames)?; }
                #[cfg(jdk_ge_25)]
                let y = 1;
                match k { #[cfg(jdk_ge_25)] 1 => p(), _ => q() }
            }
        }
        struct S { #[cfg(jdk_ge_25)] new_f: i32, keep: u8 }
    "#;

    #[test]
    fn jdk21_drops_ge25_branches() {
        let s = stripped(Some(21), SRC);
        assert!(has(&s, &["old", "i64", "set", "q", "keep"]), "{s:?}");
        assert!(none(&s, &["new", "i32", "fill", "y", "p", "new_f", "mid"]), "{s:?}");
        // 不认得的谓词不定 → 保留
        assert!(has(&s, &["d", "either"]), "{s:?}");
    }

    #[test]
    fn jdk25_drops_pre25_branches() {
        let s = stripped(Some(25), SRC);
        assert!(has(&s, &["new", "i32", "fill", "y", "p", "q", "new_f", "keep", "d", "either"]), "{s:?}");
        assert!(none(&s, &["old", "i64", "set", "mid"]), "{s:?}");
    }

    #[test]
    fn mid_range_and_unknown_jdk() {
        assert!(stripped(Some(23), SRC).contains("mid"));
        // 参考 JDK 未知：一律保留
        let s = stripped(None, SRC);
        assert!(has(&s, &["old", "new", "mid", "i32", "i64"]), "{s:?}");
    }
}

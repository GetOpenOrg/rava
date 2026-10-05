//! 字符串常量（ldc）逐调用点缓存。
//!
//! 生成的方法体以 `String::from("...")` 加载 Java 字符串常量（JLS §3.10.5：同内容常量是同一驻留对象）。
//! 该形态每次执行都新建实例、按内容查全局驻留表；展开时把它改写为调用点私有的一次性单元：
//! 首次执行经原表达式取驻留实例存入单元，此后只克隆单元中的同一实例（JVMS §5.4.3 常量池项解析一次、
//! 结果复用）。对象同一性不变（单元里存的就是驻留实例），可读层源码不变。首次加载不持锁、不经
//! `get_or_init`：加载可能触发类初始化而重入同一调用点，并发或重入的各次加载取到的都是同一驻留实例，
//! 先写入者留存。
//!
//! 改写范围：方法体与 ConstantValue 常量的初值表达式；闭包体照改（调用点仍是静态位置），
//! 嵌套项（fn / impl 等）不进入。只认「路径末两段为 `String::from`、唯一实参为字符串字面量」，
//! 单元类型取同一路径前缀（`String` / `crate::java::lang::String` …），与原表达式类型一致。

use syn::visit_mut::{self, VisitMut};
use syn::{Expr, ExprCall, Lit};

use super::parse::ClassInput;

/// 改写整个类输入（方法体、接口实现块内方法体、常量初值）
pub(crate) fn cache_literals(input: &mut ClassInput) {
    let mut rw = LdcRewriter;
    for f in input.fns.iter_mut().chain(input.iface_impls.iter_mut().flat_map(|i| i.fns.iter_mut())) {
        if let Some(b) = f.block.as_mut() {
            rw.visit_block_mut(b);
        }
    }
    for st in input.statics.iter_mut() {
        if let Some(v) = st.const_value.as_mut() {
            rw.visit_expr_mut(v);
        }
    }
}

struct LdcRewriter;

/// `<前缀>::String::from("lit")` → 前缀路径（类型）；其余形态 None
fn literal_load(call: &ExprCall) -> Option<syn::Path> {
    let [arg] = call.args.iter().collect::<Vec<_>>()[..] else { return None };
    let Expr::Lit(l) = arg else { return None };
    if !matches!(l.lit, Lit::Str(_)) {
        return None;
    }
    let Expr::Path(p) = &*call.func else { return None };
    if p.qself.is_some() {
        return None;
    }
    let segs = &p.path.segments;
    let n = segs.len();
    if n < 2 || segs[n - 1].ident != "from" || segs[n - 2].ident != "String" || !segs[n - 2].arguments.is_none() {
        return None;
    }
    let mut ty = p.path.clone();
    ty.segments.pop();
    let last = ty.segments.pop()?.into_value();
    ty.segments.push(last);
    Some(ty)
}

impl VisitMut for LdcRewriter {
    fn visit_expr_mut(&mut self, e: &mut Expr) {
        if let Expr::Call(call) = e {
            if let Some(ty) = literal_load(call) {
                let load = call.clone();
                *e = syn::parse_quote! {{
                    static __LDC: ::std::sync::OnceLock<#ty> = ::std::sync::OnceLock::new();
                    match __LDC.get() {
                        ::std::option::Option::Some(__s) => ::std::clone::Clone::clone(__s),
                        ::std::option::Option::None => {
                            let __s = #load;
                            let _ = __LDC.set(::std::clone::Clone::clone(&__s));
                            __s
                        }
                    }
                }};
                return;
            }
        }
        visit_mut::visit_expr_mut(self, e);
    }

    fn visit_item_mut(&mut self, _: &mut syn::Item) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::ToTokens;

    fn rewrite(src: &str) -> String {
        let mut b: syn::Block = syn::parse_str(src).unwrap();
        LdcRewriter.visit_block_mut(&mut b);
        b.into_token_stream().to_string()
    }

    #[test]
    fn literal_loads_get_site_cells() {
        let s = rewrite(r#"{ let a = f(String::from("x"))?; let b = crate::java::lang::String::from("y"); }"#);
        assert_eq!(s.matches("static __LDC").count(), 2, "{s}");
        assert!(s.contains(":: std :: sync :: OnceLock < String >"), "{s}");
        assert!(s.contains(":: std :: sync :: OnceLock < crate :: java :: lang :: String >"), "{s}");
        assert!(s.contains("let __s = String :: from (\"x\")"), "{s}");
    }

    #[test]
    fn non_literal_and_nested_items_untouched() {
        let s = rewrite(r#"{ let a = String::from(s); let b = Foo::from("x"); fn g() -> String { String::from("z") } }"#);
        assert!(!s.contains("__LDC"), "{s}");
    }
}

//! 手写体的类名判定守卫：运行时类判定成立的区域内，接收者的 Java 类型已知。
//!
//! 守卫形态（`R` 为变量或其 `.0`，类名为 Java binary name 字符串字面量）：
//! - `if R.__class_name() == "c" { … }`（等式两侧可互换；`&&` 合取的任一项）——区域为 then 分支；
//! - `R.is_instance_of("c") && …`——区域为 `&&` 右侧（及 if 的 then 分支）；
//! - `match R.__class_name() { "a" | "b" => …, … }`——区域为该分支（守卫与分支体）。
//!
//! 区域内 `R` 以根类型（`Object` / 推不出）出现的 `toString` 分派（`__obj_str` / `__to_string` / Display）
//! 收窄为对 `c` 的分派：`__class_name` 是运行期确切类、`is_instance_of` 是子类型判定，取 `c` 的 open 集
//! （已逃逸的 `c` 及其子类实例）两者都覆盖。`R` 在区域内被重新绑定 / 赋值时收窄失效（保守）。

/// 运行时根类 vtable 的确切类名入口（`ObjectVTable::__class_name`）
const CLASS_NAME_RUST: &str = "__class_name";
/// 运行时子类型判定入口（`Object::is_instance_of` / `ObjectVTable::is_instance_of`）
const INSTANCE_OF_RUST: &str = "is_instance_of";

/// 接收者表达式 `o` / `o.0`（可带括号 / 引用）的变量名
pub(super) fn guard_var(e: &syn::Expr) -> Option<String> {
    match e {
        syn::Expr::Path(p) => p.path.get_ident().map(|i| i.to_string()),
        syn::Expr::Field(f) if matches!(&f.member, syn::Member::Unnamed(i) if i.index == 0) => guard_var(&f.base),
        syn::Expr::Paren(p) => guard_var(&p.expr),
        syn::Expr::Group(g) => guard_var(&g.expr),
        syn::Expr::Reference(r) => guard_var(&r.expr),
        _ => None,
    }
}

fn plain_str(e: &syn::Expr) -> Option<String> {
    match e {
        syn::Expr::Lit(syn::ExprLit { lit: syn::Lit::Str(l), .. }) => Some(l.value()),
        syn::Expr::Paren(p) => plain_str(&p.expr),
        syn::Expr::Group(g) => plain_str(&g.expr),
        syn::Expr::Reference(r) => plain_str(&r.expr),
        _ => None,
    }
}

/// `R.__class_name()` 的变量 `R`
pub(super) fn class_name_var(e: &syn::Expr) -> Option<String> {
    match e {
        syn::Expr::MethodCall(m) if m.method == CLASS_NAME_RUST && m.args.is_empty() => guard_var(&m.receiver),
        syn::Expr::Paren(p) => class_name_var(&p.expr),
        syn::Expr::Group(g) => class_name_var(&g.expr),
        _ => None,
    }
}

/// 条件成立时可确定的（变量, 类名）：`&&` 合取的各项里的类名等式与子类型判定
pub(super) fn cond_guards(e: &syn::Expr) -> Vec<(String, String)> {
    match e {
        syn::Expr::Paren(p) => cond_guards(&p.expr),
        syn::Expr::Group(g) => cond_guards(&g.expr),
        syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::And(_)) => {
            let mut out = cond_guards(&b.left);
            out.extend(cond_guards(&b.right));
            out
        }
        syn::Expr::Binary(b) if matches!(b.op, syn::BinOp::Eq(_)) => {
            let pair = |x: &syn::Expr, y: &syn::Expr| class_name_var(x).zip(plain_str(y));
            pair(&b.left, &b.right).or_else(|| pair(&b.right, &b.left)).into_iter().collect()
        }
        syn::Expr::MethodCall(m) if m.method == INSTANCE_OF_RUST && m.args.len() == 1 => {
            guard_var(&m.receiver).zip(plain_str(&m.args[0])).into_iter().collect()
        }
        _ => Vec::new(),
    }
}

/// match 分支模式全由字符串字面量组成（`"a" | "b"`）时的字面量；含通配 / 绑定等其他模式 → None
pub(super) fn arm_lits(p: &syn::Pat) -> Option<Vec<String>> {
    match p {
        syn::Pat::Lit(l) => match &l.lit {
            syn::Lit::Str(s) => Some(vec![s.value()]),
            _ => None,
        },
        syn::Pat::Or(o) => {
            let mut out = Vec::new();
            for c in &o.cases {
                out.extend(arm_lits(c)?);
            }
            Some(out)
        }
        syn::Pat::Paren(p) => arm_lits(&p.pat),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(s: &str) -> syn::Expr {
        syn::parse_str(s).unwrap()
    }

    #[test]
    fn guards_from_conditions() {
        assert_eq!(cond_guards(&e(r#"v.0.__class_name() == "a/B""#)), vec![("v".into(), "a/B".into())]);
        assert_eq!(cond_guards(&e(r#""a/B" == (&v).__class_name()"#)), vec![("v".into(), "a/B".into())]);
        assert_eq!(
            cond_guards(&e(r#"x.is_instance_of("a/C") && y.0.__class_name() == "a/D""#)),
            vec![("x".into(), "a/C".into()), ("y".into(), "a/D".into())]
        );
        // 析取 / 不等 / 非字面量都不给守卫
        assert!(cond_guards(&e(r#"v.0.__class_name() == "a/B" || w"#)).is_empty());
        assert!(cond_guards(&e(r#"v.0.__class_name() != "a/B""#)).is_empty());
        assert!(cond_guards(&e(r#"v.0.__class_name() == name"#)).is_empty());
        assert!(cond_guards(&e(r#"f(v).__class_name() == "a/B""#)).is_empty());
    }

    #[test]
    fn match_arms() {
        let m: syn::ExprMatch = syn::parse_str(r#"match v.0.__class_name() { "a/B" | "a/C" => 1, "a/D" => 2, x => 3, _ => 4 }"#).unwrap();
        assert_eq!(class_name_var(&m.expr).as_deref(), Some("v"));
        let lits: Vec<Option<Vec<String>>> = m.arms.iter().map(|a| arm_lits(&a.pat)).collect();
        assert_eq!(lits, vec![Some(vec!["a/B".into(), "a/C".into()]), Some(vec!["a/D".into()]), None, None]);
    }
}

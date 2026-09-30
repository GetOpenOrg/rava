//! 转换类节点的渲染：类祖先按值上转（← `upcast_expr` / `UpcastExpr`）与
//! checkcast / 视图转换（← `render_cast` / `_clone_src`）。形态变更只改此处。

use super::lit::write_str_token;
use super::ty::write_type;
use super::Renderer;
use crate::{anchors, CastExpr, CastMode, Expr, UpcastWrap};

impl Renderer<'_> {
    /// 上转：`src.into()`，按包装方式决定 src 的形态。
    pub(crate) fn write_upcast(&self, out: &mut String, e: &Expr, wrap: UpcastWrap) {
        let form = match wrap {
            UpcastWrap::Owned if matches!(e, Expr::Var(_)) => UpcastWrap::Clone,
            UpcastWrap::Owned => UpcastWrap::Paren,
            UpcastWrap::Auto if self.is_atomic(e) => UpcastWrap::Bare,
            UpcastWrap::Auto => UpcastWrap::Paren,
            w => w,
        };
        match form {
            UpcastWrap::Clone => {
                out.push_str("Clone::clone(&");
                self.write_expr(out, e);
                out.push(')');
            }
            UpcastWrap::Paren => {
                out.push('(');
                self.write_expr(out, e);
                out.push(')');
            }
            _ => self.write_expr(out, e),
        }
        out.push_str(".into()");
    }

    /// 转换源的保活取值：Java 引用无 move 语义，统一 `Clone::clone`。
    /// `this` 在实例方法中是 `&Self`：`Clone::clone(this)` 克隆本体
    /// （`Clone::clone(&this)` 会克隆引用本身）。
    fn write_clone_src(&self, out: &mut String, e: &Expr) {
        if e.is_var_named("this") {
            out.push_str("Clone::clone(this)");
        } else {
            out.push_str("Clone::clone(&");
            self.write_expr(out, e);
            out.push(')');
        }
    }

    /// checkcast（可失败，S-1）与静态合法的 `From<Object>` 视图转换共用的渲染入口（A-3）。
    pub(crate) fn write_checkcast(&self, out: &mut String, c: &CastExpr) {
        let mut src = String::new();
        if c.box_first {
            src.push_str(anchors::OBJECT);
            src.push_str("::from(");
            self.write_clone_src(&mut src, &c.expr);
            src.push(')');
        } else {
            self.write_clone_src(&mut src, &c.expr);
        }
        match &c.mode {
            CastMode::CheckedInterface { binary } => {
                // 目标擦除为 Object：try_cast::<Object> 的 downcast 快路径恒命中，
                // 判定须由 is_instance_of 承担 → 专用入口（null 通过 / 幂等）
                out.push_str(&src);
                out.push_str(".try_cast_iface(");
                write_str_token(out, binary);
                out.push_str(")?");
            }
            CastMode::Checked { binary } => {
                out.push_str(&src);
                // 数组目标：元素类型驱动（try_cast_array，S-4/A-1），以剥一层的元素类型 turbofish 发射
                match c.target.array_elem() {
                    Some(elem) => {
                        out.push_str(".try_cast_array::<");
                        write_type(out, elem);
                    }
                    None => {
                        out.push_str(".try_cast::<");
                        write_type(out, &c.target);
                    }
                }
                out.push_str(">(");
                write_str_token(out, binary);
                out.push_str(")?");
            }
            CastMode::Unchecked => {
                out.push('<');
                write_type(out, &c.target);
                out.push_str(" as ::std::convert::From<");
                out.push_str(anchors::OBJECT);
                out.push_str(">>::from(");
                out.push_str(&src);
                out.push(')');
            }
        }
    }
}

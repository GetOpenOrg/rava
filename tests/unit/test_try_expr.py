"""FS-Q1 Q1-a：TryExpr（`?` 传播）节点渲染。

    python3 -m unittest tests.unit.test_try_expr
"""
import unittest

from codegen.rs_ir import TryExpr, MethodCall, Call, Var, LetStmt, ExprStmt
from codegen.render import render_expr, render_stmt


class TryExprRenderTest(unittest.TestCase):
    def test_try_method_call(self):
        self.assertEqual(render_expr(TryExpr(MethodCall(Var('list'), 'size', []))), 'list.size()?')

    def test_try_static_call_in_let(self):
        e = TryExpr(Call('ArrayList::<Object>::new', []))
        self.assertEqual(render_stmt(LetStmt('_t0', value=e)), 'let _t0 = ArrayList::<Object>::new()?;')

    def test_try_expr_stmt(self):
        s = ExprStmt(TryExpr(MethodCall(Var('sb'), 'append', [Var('x')])))
        self.assertEqual(render_stmt(s), 'sb.append(x)?;')


if __name__ == '__main__':
    unittest.main()

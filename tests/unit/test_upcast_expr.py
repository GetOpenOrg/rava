"""
render.upcast_expr 单元测试（R-2′：类祖先按值上转的唯一形态决策点）：
    python3 -m unittest tests.unit.test_upcast_expr

四种包装与被替换调用点的旧发射文本逐字对拍（生成树零 diff 的单元级保证）：
fields/arrays 的 `Clone::clone(&v).into()`、stack 的变量 clone / 非变量恒加括号、
vars 的原子直接后缀 / 非原子加括号、returns/invoke_sig 的已构造表达式后缀。
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.render import is_atomic_rs, upcast_expr


class UpcastForms(unittest.TestCase):

    def test_clone(self):
        for v in ('x', 'this.field', 'arr_3'):
            self.assertEqual(upcast_expr(v, 'clone'), f"Clone::clone(&{v}).into()")

    def test_paren(self):
        for e in ('a.b()?', 'Foo::new()?', 'x'):
            self.assertEqual(upcast_expr(e, 'paren'), f"({e}).into()")

    def test_auto_matches_old_vars_form(self):
        for e in ('x', 'a.b()?', 'Foo::new()?', 'if c { a } else { b }', 'a + b', '(a)'):
            old = f"{e}.into()" if is_atomic_rs(e) else f"({e}).into()"
            self.assertEqual(upcast_expr(e, 'auto'), old, e)

    def test_none(self):
        for e in ('Clone::clone(this)', 'Clone::clone(&e)', 'foo()?'):
            self.assertEqual(upcast_expr(e), f"{e}.into()")

    def test_is_atomic(self):
        self.assertTrue(is_atomic_rs('a.b::c()?'))
        self.assertTrue(is_atomic_rs('f(a + b)'))
        self.assertFalse(is_atomic_rs('a + b'))
        self.assertFalse(is_atomic_rs('-x'))


if __name__ == '__main__':
    unittest.main()

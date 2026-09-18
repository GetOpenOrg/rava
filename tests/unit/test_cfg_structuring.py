"""
控制流结构化单元测试（不依赖 JDK）：
    python3 -m unittest tests.unit.test_cfg_structuring
"""

import os
import sys
import unittest
from dataclasses import dataclass, field

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.cfg import (analyze, build_structure, simplify, build_dispatch, atom, negate,
                         cond_and, cond_or, render_cond, JumpLedger, CfgAuditError)
from codegen.cfg import structure as st
from codegen.method.emit import emit_tree
from codegen.rs_ir import RawStmt
from codegen.render import render_stmt


@dataclass
class FakeNode:
    id: int
    kind: str                       # goto / cond / switch / exit
    target: int | None = None
    fallthrough: int | None = None
    cond: object = None
    key: str | None = None
    cases: list = field(default_factory=list)
    default: int | None = None
    stmts: list = field(default_factory=list)
    decls: list = field(default_factory=list)

    def successors(self) -> list:
        if self.kind == 'goto':
            return [self.target]
        if self.kind == 'cond':
            return [self.fallthrough, self.target]
        if self.kind == 'switch':
            out = []
            for _, tgt in self.cases:
                if tgt not in out:
                    out.append(tgt)
            if self.default not in out:
                out.append(self.default)
            return out
        return []


def code(text: str) -> list:
    return [RawStmt(text)]


def render(nodes: dict) -> str:
    flow = analyze(0, {i: n.successors() for i, n in nodes.items()})
    assert flow.reducible
    tree = simplify(build_structure(nodes, flow))
    blocks = [it.block for it in st.walk(tree) if isinstance(it, st.Code)]
    assert sorted(blocks) == sorted(flow.rpo), "每个块必须恰好输出一次"
    lines = []
    for indent, item in emit_tree(tree, nodes, indent=''):
        lines.append(item if isinstance(item, str) else indent + render_stmt(item))
    return '\n'.join(lines)


class ConditionAlgebra(unittest.TestCase):
    def test_short_circuit_rendering(self):
        a, b, c = atom('a == b', 'a != b'), atom('key.is_null()', '!key.is_null()'), atom('key.equals(k)')
        self.assertEqual(render_cond(cond_or(a, cond_and(negate(b), c))),
                         'a == b || (!key.is_null() && key.equals(k))')

    def test_de_morgan(self):
        a, b = atom('x < 1', 'x >= 1'), atom('y < 2', 'y >= 2')
        self.assertEqual(render_cond(negate(cond_and(a, b))), 'x >= 1 || y >= 2')


class Structuring(unittest.TestCase):
    def test_while_loop(self):
        nodes = {
            0: FakeNode(0, 'goto', target=1, stmts=code('let mut i = 0;')),
            1: FakeNode(1, 'cond', target=3, fallthrough=2, cond=atom('i >= n', 'i < n')),
            2: FakeNode(2, 'goto', target=1, stmts=code('i += 1;')),
            3: FakeNode(3, 'exit', stmts=code('return Ok(());')),
        }
        text = render(nodes)
        self.assertIn('while i < n {', text)
        self.assertNotIn("'", text)

    def test_nested_break_uses_label(self):
        # outer: loop { inner: loop { if found { break outer } if done { break inner } } step }
        nodes = {
            0: FakeNode(0, 'goto', target=1, stmts=code('init();')),
            1: FakeNode(1, 'goto', target=2, stmts=code('outer_head();')),
            2: FakeNode(2, 'cond', target=5, fallthrough=3, cond=atom('found'), stmts=code('inner_head();')),
            3: FakeNode(3, 'cond', target=4, fallthrough=2, cond=atom('done'), stmts=code('inner_body();')),
            4: FakeNode(4, 'goto', target=1, stmts=code('step();')),
            5: FakeNode(5, 'exit', stmts=code('return Ok(());')),
        }
        text = render(nodes)
        self.assertRegex(text, r"'l\d+: loop \{")
        self.assertRegex(text, r"break 'l\d+;")
        for call in ('init();', 'outer_head();', 'inner_head();', 'inner_body();', 'step();'):
            self.assertEqual(text.count(call), 1)

    def test_merge_node_gets_labeled_block(self):
        # if a { if b { goto join } x } y; join:
        nodes = {
            0: FakeNode(0, 'cond', target=3, fallthrough=1, cond=atom('!a', 'a')),
            1: FakeNode(1, 'cond', target=4, fallthrough=2, cond=atom('b')),
            2: FakeNode(2, 'goto', target=3, stmts=code('x();')),
            3: FakeNode(3, 'goto', target=4, stmts=code('y();')),
            4: FakeNode(4, 'exit', stmts=code('return Ok(());')),
        }
        text = render(nodes)
        self.assertRegex(text, r"break 'b\d+;")
        self.assertEqual(text.count('y();'), 1)

    def test_switch_arms_sharing_a_target_are_grouped(self):
        nodes = {
            0: FakeNode(0, 'switch', key='k', cases=[([1], 1), ([2], 2), ([3], 1), ([4], 3)], default=3),
            1: FakeNode(1, 'goto', target=3, stmts=code('one_or_three();')),
            2: FakeNode(2, 'goto', target=3, stmts=code('two();')),
            3: FakeNode(3, 'exit', stmts=code('return Ok(());')),
        }
        text = render(nodes)
        self.assertIn('1 | 3 => {', text)
        self.assertEqual(text.count('one_or_three();'), 1)
        self.assertNotIn('4 =>', text)

    def test_irreducible_falls_back_to_dispatch(self):
        nodes = {
            0: FakeNode(0, 'cond', target=2, fallthrough=1, cond=atom('c')),
            1: FakeNode(1, 'goto', target=2, stmts=code('a();')),
            2: FakeNode(2, 'cond', target=3, fallthrough=1, cond=atom('d'), stmts=code('b();')),
            3: FakeNode(3, 'exit', stmts=code('return Ok(());')),
        }
        flow = analyze(0, {i: n.successors() for i, n in nodes.items()})
        self.assertFalse(flow.reducible)
        tree = build_dispatch(nodes, 0, list(nodes))
        text = '\n'.join(item if isinstance(item, str) else render_stmt(item)
                         for _, item in emit_tree(tree, nodes))
        self.assertIn('match __pc {', text)
        self.assertIn('__pc = if d { 3 } else { 1 };', text)


class Audit(unittest.TestCase):
    def test_unconsumed_jump_is_an_error(self):
        ledger = JumpLedger('C.m:()V')
        ledger.expect(10)
        ledger.expect(20)
        ledger.consume(10, 'structured')
        with self.assertRaises(CfgAuditError) as ctx:
            ledger.verify()
        self.assertIn('20', str(ctx.exception))
        ledger.consume(20, 'short-circuit')
        ledger.verify()


if __name__ == '__main__':
    unittest.main()

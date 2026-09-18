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
from codegen.method.try_catch import TryCatchPlan, CatchClause


@dataclass
class FakeInstr:
    offset: int
    opcode: str
    operand: str | None = None


def catch_any(handler: int) -> tuple:
    return (CatchClause(handler_pc=0, handler_idx=handler, is_catch_any=True), 'e', 'Throwable')


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
    # try 节点：target = try 体入口，handlers / catches / catch_ends 逐 catch 子句平行
    handlers: list = field(default_factory=list)
    catches: list = field(default_factory=list)
    catch_ends: list = field(default_factory=list)
    group: int | None = None
    ctx: frozenset = frozenset()
    start_pc: int = 0

    def successors(self) -> list:
        if self.kind == 'try':
            return [self.target] + list(self.handlers)
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


class TryRegions(unittest.TestCase):
    def test_exit_block_inside_try_inside_loop_stays_in_try(self):
        # while c { try { if bad { throw } work } catch { handle } tail }
        g = frozenset({1})
        nodes = {
            0: FakeNode(0, 'goto', target=1, stmts=code('init();'), start_pc=0),
            1: FakeNode(1, 'cond', target=7, fallthrough=2, cond=atom('!c', 'c'), start_pc=10),
            2: FakeNode(2, 'try', target=3, handlers=[5], catches=[catch_any(5)], catch_ends=[None],
                        group=1, start_pc=20),
            3: FakeNode(3, 'cond', target=6, fallthrough=4, cond=atom('!bad', 'bad'), ctx=g, start_pc=20),
            4: FakeNode(4, 'exit', stmts=code('return Err(thrown());'), ctx=g, start_pc=30),
            5: FakeNode(5, 'goto', target=8, stmts=code('handle();'), start_pc=50),
            6: FakeNode(6, 'goto', target=8, stmts=code('work();'), ctx=g, start_pc=40),
            7: FakeNode(7, 'exit', stmts=code('return Ok(());'), start_pc=70),
            8: FakeNode(8, 'goto', target=1, stmts=code('tail();'), start_pc=60),
        }
        text = render(nodes)
        self.assertLess(text.index('java_try!'), text.index('thrown()'))
        self.assertLess(text.index('thrown()'), text.index('catch (e)'))
        self.assertLess(text.index('handle();'), text.index('tail();'))
        self.assertLess(text.index('tail();'), text.index('return Ok(());'))

    def _returning_try(self, catch_end):
        # try { return v } catch { handle } after
        g = frozenset({1})
        return {
            0: FakeNode(0, 'try', target=1, handlers=[2], catches=[catch_any(2)], catch_ends=[catch_end],
                        group=1, start_pc=0),
            1: FakeNode(1, 'exit', stmts=code('return Ok(v);'), ctx=g, start_pc=0),
            2: FakeNode(2, 'goto', target=3, stmts=code('handle();'), start_pc=10),
            3: FakeNode(3, 'exit', stmts=code('after();'), start_pc=20),
        }

    def test_code_after_catch_body_end_is_a_sibling_of_the_try(self):
        lines = render(self._returning_try(catch_end=20)).split('\n')
        closing = max(i for i, ln in enumerate(lines) if ln == '}')
        after = next(i for i, ln in enumerate(lines) if 'after();' in ln)
        self.assertGreater(after, closing)
        self.assertEqual(lines[after], 'after();')          # 与 java_try! 同层
        self.assertNotIn("break '", '\n'.join(lines))

    def test_without_scope_info_code_stays_in_catch_body(self):
        text = render(self._returning_try(catch_end=None))
        self.assertLess(text.index('after();'), text.rindex('}'))

    def test_bare_return_adopted_into_try_keeps_return_inside_try_body(self):
        text = render(self._returning_try(catch_end=20))
        self.assertLess(text.index('return Ok(v);'), text.index('catch (e)'))

    def test_misnested_region_is_rejected(self):
        nodes = self._returning_try(catch_end=20)
        nodes[3].ctx = frozenset({1})
        flow = analyze(0, {i: n.successors() for i, n in nodes.items()})
        with self.assertRaises(st.CfgError):
            build_structure(nodes, flow)


class TryPlan(unittest.TestCase):
    INSTRS = [FakeInstr(0, 'iload_0'), FakeInstr(1, 'invokestatic'), FakeInstr(4, 'goto'),
              FakeInstr(7, 'astore_1'), FakeInstr(8, 'aload_1'), FakeInstr(9, 'pop'),
              FakeInstr(10, 'astore_2'), FakeInstr(11, 'aload_2'), FakeInstr(12, 'athrow'),
              FakeInstr(13, 'return')]

    def test_range_is_clipped_at_handler_entry(self):
        # javac 的 finally 条目 (0, 11, 10)：区间越过处理器入口，覆盖处理器开头的 astore
        plan = TryCatchPlan([(0, 4, 7, 'E'), (0, 11, 10, None)], self.INSTRS)
        ranges = sorted(g.ranges for g in plan.groups)
        self.assertEqual(ranges, [((0, 4),), ((0, 10),)])

    def test_self_protecting_entry_is_dropped(self):
        plan = TryCatchPlan([(10, 12, 10, None)], self.INSTRS)
        self.assertFalse(plan)

    def test_catch_body_end_comes_from_exception_variable_scope(self):
        plan = TryCatchPlan([(0, 4, 7, 'E')], self.INSTRS, local_vars=[(1, 8, 2, 'e', 'LE;')])
        self.assertEqual(plan.groups[0].clauses[0].body_end_pc, 10)
        self.assertEqual(plan.catch_body_ends(), [10])

    def test_catch_body_end_absent_without_debug_info(self):
        plan = TryCatchPlan([(0, 4, 7, 'E')], self.INSTRS)
        self.assertIsNone(plan.groups[0].clauses[0].body_end_pc)
        self.assertEqual(plan.catch_body_ends(), [])


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

"""closure.json 折叠点（folds v1）消费的单元测试（不依赖 JDK）：
    python3 -m unittest tests.unit.test_closure_folds
"""

import json
import os
import sys
import tempfile
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen import closure_folds as cf
from codegen.types import Instr

M = 'p/C.m:(I)I'


def load(folds, version=1):
    with tempfile.NamedTemporaryFile('w', suffix='.json', delete=False) as f:
        json.dump({'folds_version': version, 'folds': folds}, f)
    try:
        return cf.load(f.name)
    finally:
        os.unlink(f.name)


def ops(instrs):
    return [(x.offset, x.opcode, x.operand) for x in instrs]


# 0 iload_0 / 1 ifeq 8 / 4 iconst_1 / 5 goto 9 / 8 iconst_2 / 9 ireturn
IF_ELSE = [Instr(0, 'iload_0'), Instr(1, 'ifeq', '8'), Instr(4, 'iconst_1'),
           Instr(5, 'goto', '9'), Instr(8, 'iconst_2'), Instr(9, 'ireturn')]


class Branches(unittest.TestCase):
    def tearDown(self):
        cf.clear()

    def test_no_fold_is_identity(self):
        self.assertIs(cf.apply(M, IF_ELSE, [], 10), IF_ELSE)

    def test_taken_side_dead_becomes_pop(self):
        load([{'method': M, 'dead_pcs': [[8, 9]]}])
        out = cf.apply(M, IF_ELSE, [], 10)
        self.assertEqual(ops(out), [(0, 'iload_0', None), (1, 'pop', None), (4, 'iconst_1', None),
                                    (5, 'goto', '9'), (9, 'ireturn', None)])

    def test_fallthrough_dead_becomes_pop_goto(self):
        load([{'method': M, 'dead_pcs': [[4, 8]]}])
        out = cf.apply(M, IF_ELSE, [], 10)
        self.assertEqual(ops(out), [(0, 'iload_0', None), (1, 'pop', None), (2, 'goto', '8'),
                                    (8, 'iconst_2', None), (9, 'ireturn', None)])

    def test_two_operand_branch(self):
        code = [Instr(0, 'iload_0'), Instr(1, 'iload_1'), Instr(2, 'if_icmpge', '9'),
                Instr(5, 'iconst_1'), Instr(6, 'ireturn'), Instr(9, 'iconst_0'), Instr(10, 'ireturn')]
        load([{'method': M, 'dead_pcs': [[5, 9]]}])
        out = cf.apply(M, code, [], 11)
        self.assertEqual(ops(out)[2:5], [(2, 'pop', None), (3, 'pop', None), (4, 'goto', '9')])

    def test_both_successors_dead_rejected(self):
        load([{'method': M, 'dead_pcs': [[4, 9]]}])
        with self.assertRaises(cf.FoldError):
            cf.apply(M, IF_ELSE, [], 10)

    def test_endpoint_not_instruction_start_rejected(self):
        load([{'method': M, 'dead_pcs': [[3, 8]]}])
        with self.assertRaises(cf.FoldError):
            cf.apply(M, IF_ELSE, [], 10)

    def test_live_fallthrough_into_dead_rejected(self):
        load([{'method': M, 'dead_pcs': [[5, 8]]}])      # iconst_1 顺序落入死区
        with self.assertRaises(cf.FoldError):
            cf.apply(M, IF_ELSE, [], 10)

    def test_unknown_version_ignored(self):
        self.assertEqual(load([{'method': M, 'dead_pcs': [[8, 9]]}], version=2), 0)
        self.assertIs(cf.apply(M, IF_ELSE, [], 10), IF_ELSE)


class Switches(unittest.TestCase):
    def tearDown(self):
        cf.clear()

    CODE = [Instr(0, 'iload_0'),
            Instr(1, 'tableswitch', 'default:40 low:0 high:1 offs:28,34'),
            Instr(28, 'iconst_1'), Instr(29, 'ireturn'),
            Instr(34, 'iconst_2'), Instr(35, 'ireturn'),
            Instr(40, 'iconst_3'), Instr(41, 'ireturn')]

    def test_dead_case_retargets_to_default(self):
        load([{'method': M, 'dead_pcs': [[28, 34]]}])
        out = cf.apply(M, self.CODE, [], 42)
        self.assertEqual(out[1].operand, 'default:40 low:0 high:1 offs:40,34')

    def test_single_live_target_becomes_goto(self):
        load([{'method': M, 'dead_pcs': [[28, 40]]}])
        out = cf.apply(M, self.CODE, [], 42)
        self.assertEqual(ops(out)[:3], [(0, 'iload_0', None), (1, 'pop', None), (2, 'goto', '40')])

    def test_lookupswitch(self):
        code = [Instr(0, 'iload_0'), Instr(1, 'lookupswitch', 'default:40 7:28 9:34')] + self.CODE[2:]
        load([{'method': M, 'dead_pcs': [[40, 42]]}])
        out = cf.apply(M, code, [], 42)
        self.assertEqual(out[1].operand, 'default:28 7:28 9:34')


class ExceptionTable(unittest.TestCase):
    def tearDown(self):
        cf.clear()

    # 0 aload_0 / 1 invokevirtual / 4 iconst_1 / 5 ireturn / 6 astore_1(handler) / 7 iconst_0 / 8 ireturn
    CODE = [Instr(0, 'aload_0'), Instr(1, 'invokevirtual', '2', 'Method p/C.f:()V'),
            Instr(4, 'iconst_1'), Instr(5, 'ireturn'), Instr(6, 'astore_1'),
            Instr(7, 'iconst_0'), Instr(8, 'ireturn')]

    def test_dead_handler_entry_removed(self):
        table = [(0, 4, 6, 'java/lang/Exception')]
        load([{'method': M, 'dead_pcs': [[6, 9]], 'dead_handlers': [6]}])
        out = cf.apply(M, self.CODE, table, 9)
        self.assertEqual(table, [])
        self.assertEqual(out[-1].opcode, 'ireturn')

    def test_undeclared_dead_handler_rejected(self):
        load([{'method': M, 'dead_pcs': [[6, 9]]}])
        with self.assertRaises(cf.FoldError):
            cf.apply(M, self.CODE, [(0, 4, 6, None)], 9)

    def test_range_start_clamped(self):
        code = [Instr(0, 'iload_0'), Instr(1, 'ifeq', '5'), Instr(4, 'nop'), Instr(5, 'aload_0'),
                Instr(6, 'athrow'), Instr(7, 'astore_1'), Instr(8, 'return')]
        table = [(4, 7, 7, None)]
        load([{'method': M, 'dead_pcs': [[4, 5]]}])
        cf.apply(M, code, table, 9)
        self.assertEqual(table, [(5, 7, 7, None)])


class Consts(unittest.TestCase):
    def tearDown(self):
        cf.clear()

    def test_getstatic_becomes_plain_load(self):
        code = [Instr(0, 'getstatic', '2', 'Field p/C.F:Z'), Instr(3, 'ireturn')]
        load([{'method': M, 'consts': [{'pc': 0, 'kind': 'getstatic', 'value': True, 'type': 'Z'}]}])
        out = cf.apply(M, code, [], 4)
        self.assertEqual((out[0].opcode, out[0].comment), ('ldc', 'int 1'))

    def test_getfield_pops_receiver(self):
        code = [Instr(0, 'aload_0'), Instr(1, 'getfield', '2', 'Field p/C.s:Ljava/lang/String;'),
                Instr(4, 'areturn')]
        load([{'method': M, 'consts': [{'pc': 1, 'kind': 'getfield', 'value': None,
                                         'type': 'Ljava/lang/String;'}]}])
        out = cf.apply(M, code, [], 5)
        npop, push = cf.decode_fold_const(out[1])
        self.assertEqual((out[1].opcode, npop, push.opcode), ('fold_const', 1, 'aconst_null'))

    def test_invoke_pops_receiver_and_args(self):
        code = [Instr(0, 'aload_0'), Instr(1, 'lload_1'), Instr(2, 'iload_3'),
                Instr(3, 'invokevirtual', '2', 'Method p/C.g:(JI)J'), Instr(6, 'lreturn')]
        load([{'method': M, 'consts': [{'pc': 3, 'kind': 'invoke', 'value': '9007199254740993',
                                         'type': 'J'}]}])
        out = cf.apply(M, code, [], 7)
        npop, push = cf.decode_fold_const(out[3])
        self.assertEqual((npop, push.opcode, push.comment), (3, 'ldc2_w', 'long 9007199254740993'))

    def test_kind_mismatch_rejected(self):
        code = [Instr(0, 'getstatic', '2', 'Field p/C.F:I'), Instr(3, 'ireturn')]
        load([{'method': M, 'consts': [{'pc': 0, 'kind': 'getfield', 'value': 1, 'type': 'I'}]}])
        with self.assertRaises(cf.FoldError):
            cf.apply(M, code, [], 4)

    def test_bool_type_checked(self):
        code = [Instr(0, 'getstatic', '2', 'Field p/C.F:Z'), Instr(3, 'ireturn')]
        load([{'method': M, 'consts': [{'pc': 0, 'kind': 'getstatic', 'value': 1, 'type': 'Z'}]}])
        with self.assertRaises(cf.FoldError):
            cf.apply(M, code, [], 4)


if __name__ == '__main__':
    unittest.main()

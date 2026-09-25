"""
数组初始化器折叠单元测试（L-1 L1-d，codegen/method/postprocess._fold_array_literals）：
    python3 -m unittest tests.unit.test_fold_array_literals

覆盖折叠条件的每条分支：简单 / 基本类型 / javac 嵌套交错形态 / 部分初始化 /
下标乱序 / 非纯值 / 中间引用本数组 / 中间出现普通语句 / 长度 0 / 自引用。
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.method.postprocess import _fold_array_literals

I = '            '


def decl(v, t, n):
    return f'{I}let mut {v}: JArray<{t}> = JArray::<{t}>::try_new({n}i32)?;'


def sset(v, k, val):
    return f'{I}{v}.set({k}i32, {val})?;'


def folded(v, t, vals):
    return (f'{I}let mut {v}: JArray<{t}> = JArray::from(vec![\n'
            + '\n'.join(f'{I}    {x},' for x in vals) + f'\n{I}]);')


class Fold(unittest.TestCase):

    def test_simple_strings(self):
        vals = ['Clone::clone(&String::from("a"))', 'Clone::clone(&String::from("b\\"q"))']
        src = [decl('_arr0', 'String', 2), sset('_arr0', 0, vals[0]), sset('_arr0', 1, vals[1]),
               f'{I}let mut x = Clone::clone(&_arr0);']
        self.assertEqual(_fold_array_literals(src),
                         [f'{I}let mut _arr0: JArray<String> = JArray::from_strs(&["a", "b\\"q"]);',
                          f'{I}let mut x = Clone::clone(&_arr0);'])

    def test_object_string_literals_compact(self):
        vals = ['Object::from(Clone::clone(&String::from("k")))', 'Object::from(Clone::clone(&String::from("v")))']
        src = [decl('_arr0', 'Object', 2), sset('_arr0', 0, vals[0]), sset('_arr0', 1, vals[1])]
        self.assertEqual(_fold_array_literals(src),
                         [f'{I}let mut _arr0: JArray<Object> = JArray::objects_from_strs(&["k", "v"]);'])

    def test_mixed_elements_stay_vec(self):
        vals = ['Object::from(Clone::clone(&String::from("k")))', 'Object::from(Clone::clone(&_arr9))']
        src = [decl('_arr0', 'Object', 2), sset('_arr0', 0, vals[0]), sset('_arr0', 1, vals[1])]
        self.assertEqual(_fold_array_literals(src), [folded('_arr0', 'Object', vals)])

    def test_string_array_with_variable_stays_vec(self):
        vals = ['Clone::clone(&String::from("a"))', 'Clone::clone(&name)']
        src = [decl('_arr0', 'String', 2), sset('_arr0', 0, vals[0]), sset('_arr0', 1, vals[1])]
        self.assertEqual(_fold_array_literals(src), [folded('_arr0', 'String', vals)])

    def test_primitives(self):
        vals = ['1i32', '-2i32', '3i32']
        src = [decl('_arr1', 'i32', 3)] + [sset('_arr1', k, v) for k, v in enumerate(vals)]
        self.assertEqual(_fold_array_literals(src), [folded('_arr1', 'i32', vals)])
        vals = ['1.5f64', '-2e-3f64', 'true']
        src = [decl('_arr1', 'f64', 3)] + [sset('_arr1', k, v) for k, v in enumerate(vals)]
        self.assertEqual(_fold_array_literals(src), [folded('_arr1', 'f64', vals)])

    def test_nested_interleaved_javac_shape(self):
        # new Object[][]{ {"k1", "v1"}, {"k2", meta} }
        k1 = 'Object::from(Clone::clone(&String::from("k1")))'
        v1 = 'Object::from(Clone::clone(&String::from("v1")))'
        k2 = 'Object::from(Clone::clone(&String::from("k2")))'
        v2 = 'Object::from(Clone::clone(&metaValue_X))'
        src = [
            decl('_arr0', 'JArray<Object>', 2),
            decl('_arr1', 'Object', 2), sset('_arr1', 0, k1), sset('_arr1', 1, v1),
            sset('_arr0', 0, 'Clone::clone(&_arr1)'),
            decl('_arr2', 'Object', 2), sset('_arr2', 0, k2), sset('_arr2', 1, v2),
            sset('_arr0', 1, 'Clone::clone(&_arr2)'),
            f'{I}let mut data: JArray<JArray<Object>> = Clone::clone(&_arr0);',
        ]
        self.assertEqual(_fold_array_literals(src), [
            f'{I}let mut _arr1: JArray<Object> = JArray::objects_from_strs(&["k1", "v1"]);',
            folded('_arr2', 'Object', [k2, v2]),
            folded('_arr0', 'JArray<Object>', ['Clone::clone(&_arr1)', 'Clone::clone(&_arr2)']),
            f'{I}let mut data: JArray<JArray<Object>> = Clone::clone(&_arr0);',
        ])

    def test_partial_init_kept(self):
        src = [decl('_arr0', 'i32', 3), sset('_arr0', 0, '1i32'), sset('_arr0', 1, '2i32'),
               f'{I}return Ok(_arr0);']
        self.assertEqual(_fold_array_literals(src), src)

    def test_partial_at_end_kept(self):
        src = [decl('_arr0', 'i32', 3), sset('_arr0', 0, '1i32')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_out_of_order_kept(self):
        src = [decl('_arr0', 'i32', 2), sset('_arr0', 1, '1i32'), sset('_arr0', 0, '2i32')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_impure_value_kept(self):
        for bad in ('foo()?', 'Clone::clone(&a.b)', '_arr9.get(0i32)?', 'x + 1i32'):
            src = [decl('_arr0', 'i32', 1), sset('_arr0', 0, bad)]
            self.assertEqual(_fold_array_literals(src), src, bad)

    def test_reference_in_between_kept(self):
        src = [decl('_arr0', 'i32', 2), sset('_arr0', 0, '1i32'),
               f'{I}let mut n = _arr0.len()?;', sset('_arr0', 1, '2i32')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_plain_statement_in_between_kept(self):
        src = [decl('_arr0', 'i32', 2), sset('_arr0', 0, 'x'),
               f'{I}x = 5i32;', sset('_arr0', 1, 'x')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_impure_intermediate_set_kept(self):
        src = [decl('_arr0', 'Object', 2), sset('_arr0', 0, 'Object::from(Clone::clone(&a))'),
               decl('_arr1', 'i32', 1), sset('_arr1', 0, 'foo()?'),
               sset('_arr0', 1, 'Clone::clone(&_arr1)')]
        out = _fold_array_literals(src)
        self.assertEqual(out[0], src[0])        # 外层未折叠（中间有非纯 set）

    def test_zero_length_kept(self):
        src = [decl('_arr0', 'i32', 0), f'{I}return Ok(_arr0);']
        self.assertEqual(_fold_array_literals(src), src)

    def test_self_reference_kept(self):
        src = [decl('_arr0', 'Object', 1), sset('_arr0', 0, 'Object::from(Clone::clone(&_arr0))')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_type_mismatch_kept(self):
        src = [f'{I}let mut _arr0: JArray<Object> = JArray::<String>::try_new(1i32)?;',
               sset('_arr0', 0, '1i32')]
        self.assertEqual(_fold_array_literals(src), src)

    def test_non_temp_array_untouched(self):
        src = [f'{I}let mut arr: JArray<i32> = JArray::<i32>::try_new(1i32)?;', f'{I}arr.set(0i32, 1i32)?;']
        self.assertEqual(_fold_array_literals(src), src)


if __name__ == '__main__':
    unittest.main()

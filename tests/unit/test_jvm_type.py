"""
JvmType 类型代数单元测试（不依赖 JDK）：
    python3 -m unittest tests.unit.test_jvm_type

覆盖：代数定律（erasure 幂等 / substitute 组合 / frozen 可 hash）、
子类型判定（数组协变、类型变量经 bound、null 底类型、通配符包含、
registry 闭包 + 缓存失效）、构造入口（描述符 / 泛型签名）。
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.jvm_type import (Array, ClassRef, JvmType, Null, Primitive, TypeVar,
                              Wildcard, from_descriptor, from_signature,
                              _super_closure)
from codegen.types import ClassInfo


def _ci(name: str, super_class: str = 'java/lang/Object', interfaces=(),
        is_interface: bool = False) -> ClassInfo:
    return ClassInfo(name=name, fields=[], methods=[], super_class=super_class,
                     interfaces=list(interfaces), is_interface=is_interface)


def _registry() -> dict:
    """JDK 形态迷你 fixture（binary 全名 → ClassInfo）。"""
    reg = {
        'java/lang/Object':        _ci('java/lang/Object', ''),
        'java/lang/CharSequence':  _ci('java/lang/CharSequence', is_interface=True),
        'java/lang/Number':        _ci('java/lang/Number'),
        'java/lang/String':        _ci('java/lang/String',
                                       interfaces=['java/lang/CharSequence']),
        'java/lang/Integer':       _ci('java/lang/Integer',
                                       super_class='java/lang/Number',
                                       interfaces=['java/lang/Comparable']),
        'java/lang/Comparable':    _ci('java/lang/Comparable', is_interface=True),
        'java/lang/Iterable':      _ci('java/lang/Iterable', is_interface=True),
        'java/util/Collection':    _ci('java/util/Collection',
                                       interfaces=['java/lang/Iterable'],
                                       is_interface=True),
        'java/util/List':          _ci('java/util/List',
                                       interfaces=['java/util/Collection'],
                                       is_interface=True),
        'java/util/RandomAccess':  _ci('java/util/RandomAccess', is_interface=True),
        'java/util/AbstractList':  _ci('java/util/AbstractList',
                                       interfaces=['java/util/List']),
        'java/util/ArrayList':     _ci('java/util/ArrayList',
                                       super_class='java/util/AbstractList',
                                       interfaces=['java/util/List',
                                                   'java/util/RandomAccess']),
    }
    return reg


REG = _registry()

# 类型样例池（定律测试的论域）
def _samples():
    T = TypeVar('T')
    E = TypeVar('E', ClassRef('java/lang/Number'))
    return [
        Primitive('int'),
        Primitive('void'),
        ClassRef('java/lang/String'),
        ClassRef('java/util/ArrayList', (ClassRef('java/lang/String'),)),
        ClassRef('java/util/List', (Wildcard('+', ClassRef('java/lang/Number')),)),
        ClassRef('java/util/List', (Wildcard('-'),)),
        Array(Primitive('int')),
        Array(Array(ClassRef('java/lang/String'))),
        Array(TypeVar('T')),
        T, E,
        Wildcard('*'), Wildcard('+', ClassRef('java/lang/String')),
        Null(),
    ]


class ErasureTests(unittest.TestCase):

    def test_idempotent(self):
        """erasure(erasure(t)) == erasure(t)（幂等定律）。"""
        for t in _samples():
            e1 = t.erasure()
            self.assertEqual(e1, e1.erasure(), msg=f'{t!r}')

    def test_drops_type_args(self):
        t = ClassRef('java/util/ArrayList', (ClassRef('java/lang/String'),))
        self.assertEqual(t.erasure(), ClassRef('java/util/ArrayList'))

    def test_typevar_erases_to_bound(self):
        self.assertEqual(TypeVar('E', ClassRef('java/lang/Number')).erasure(),
                         ClassRef('java/lang/Number'))
        self.assertEqual(TypeVar('T').erasure(), ClassRef('java/lang/Object'))

    def test_array_erases_element(self):
        t = Array(ClassRef('java/util/ArrayList', (ClassRef('java/lang/String'),)))
        self.assertEqual(t.erasure(), Array(ClassRef('java/util/ArrayList')))
        self.assertEqual(Array(TypeVar('E', ClassRef('java/lang/Number'))).erasure(),
                         Array(ClassRef('java/lang/Number')))

    def test_wildcard_erases_to_bound(self):
        self.assertEqual(Wildcard('+', ClassRef('java/lang/String')).erasure(),
                         ClassRef('java/lang/String'))
        self.assertEqual(Wildcard('*').erasure(), ClassRef('java/lang/Object'))


class SubstituteTests(unittest.TestCase):

    def test_composition(self):
        """单遍代入组合律：t∘f 再 ∘g（g 定义域与 f 像不相交）== t∘(f∪g)。"""
        f = {'T': ClassRef('java/lang/String')}
        g = {'E': ClassRef('java/lang/Integer')}
        t = ClassRef('p/Pair', (TypeVar('T'), TypeVar('E'),
                                Array(TypeVar('T')),
                                Wildcard('+', TypeVar('E'))))
        self.assertEqual(t.substitute(f).substitute(g),
                         t.substitute({**f, **g}))

    def test_noop_without_mapping(self):
        for t in _samples():
            self.assertEqual(t.substitute({}), t)
            self.assertEqual(t.substitute({'X': ClassRef('java/lang/String')}), t)

    def test_recurses_into_structure(self):
        f = {'T': ClassRef('java/lang/Integer')}
        t = Array(ClassRef('p/Box', (TypeVar('T'), TypeVar('U'))))
        self.assertEqual(t.substitute(f),
                         Array(ClassRef('p/Box', (ClassRef('java/lang/Integer'),
                                                  TypeVar('U')))))

    def test_substitutes_bound(self):
        """bound 内的其他类型变量被代入；变量本体命中映射时整体替换（含 bound 弃置）。"""
        f = {'F': ClassRef('java/lang/String')}
        t = TypeVar('E', ClassRef('p/Enum', (TypeVar('F'),)))
        self.assertEqual(t.substitute(f),
                         TypeVar('E', ClassRef('p/Enum', (ClassRef('java/lang/String'),))))
        self.assertEqual(TypeVar('E', ClassRef('p/Enum', (TypeVar('E', ),)))
                         .substitute({'E': ClassRef('java/lang/String')}),
                         ClassRef('java/lang/String'))

    def test_hit_value_not_resubstituted(self):
        """单遍语义：命中值里的类型变量不再代入（f: T→S, g: S→String 同时给时）。"""
        t = TypeVar('T')
        both = {'T': TypeVar('S'), 'S': ClassRef('java/lang/String')}
        self.assertEqual(t.substitute(both), TypeVar('S'))


class SubtypeClassTests(unittest.TestCase):

    def setUp(self):
        self.reg = _registry()

    def test_reflexive(self):
        for t in _samples():
            self.assertTrue(t.is_subtype_of(t, self.reg), msg=f'{t!r}')

    def test_class_closure_via_super_chain(self):
        al = ClassRef('java/util/ArrayList')
        self.assertTrue(al.is_subtype_of(ClassRef('java/util/AbstractList'), self.reg))
        self.assertTrue(al.is_subtype_of(ClassRef('java/lang/Object'), self.reg))
        self.assertFalse(ClassRef('java/lang/String').is_subtype_of(
            ClassRef('java/util/ArrayList'), self.reg))

    def test_interface_edges_transitive(self):
        """接口边 + 传递：ArrayList <: List <: Collection <: Iterable；
        接口到接口（List extends Collection）。"""
        al = ClassRef('java/util/ArrayList')
        self.assertTrue(al.is_subtype_of(ClassRef('java/util/List'), self.reg))
        self.assertTrue(al.is_subtype_of(ClassRef('java/util/Collection'), self.reg))
        self.assertTrue(al.is_subtype_of(ClassRef('java/lang/Iterable'), self.reg))
        self.assertTrue(ClassRef('java/util/List').is_subtype_of(
            ClassRef('java/lang/Iterable'), self.reg))

    def test_object_is_top_for_refs(self):
        self.assertTrue(ClassRef('java/lang/Integer').is_subtype_of(
            ClassRef('java/lang/Object'), self.reg))
        self.assertFalse(Primitive('int').is_subtype_of(
            ClassRef('java/lang/Object'), self.reg))

    def test_primitive_only_reflexive(self):
        """隐式拓宽是转换不是子类型。"""
        self.assertTrue(Primitive('int').is_subtype_of(Primitive('int')))
        self.assertFalse(Primitive('int').is_subtype_of(Primitive('long')))
        self.assertFalse(Primitive('int').is_subtype_of(ClassRef('java/lang/Integer')))


class SubtypeArrayTests(unittest.TestCase):

    def setUp(self):
        self.reg = _registry()

    def test_covariance(self):
        self.assertTrue(Array(ClassRef('java/lang/String')).is_subtype_of(
            Array(ClassRef('java/lang/CharSequence')), self.reg))
        self.assertFalse(Array(ClassRef('java/lang/CharSequence')).is_subtype_of(
            Array(ClassRef('java/lang/String')), self.reg))
        self.assertTrue(Array(ClassRef('java/lang/Integer')).is_subtype_of(
            Array(ClassRef('java/lang/Number')), self.reg))

    def test_primitive_arrays_invariant(self):
        self.assertTrue(Array(Primitive('int')).is_subtype_of(Array(Primitive('int'))))
        self.assertFalse(Array(Primitive('int')).is_subtype_of(Array(Primitive('long'))))
        self.assertFalse(Array(Primitive('int')).is_subtype_of(
            Array(ClassRef('java/lang/Integer')), self.reg))

    def test_array_fixed_supertypes(self):
        sarr = Array(ClassRef('java/lang/String'))
        for top in ('java/lang/Object', 'java/lang/Cloneable', 'java/io/Serializable'):
            self.assertTrue(sarr.is_subtype_of(ClassRef(top), self.reg), msg=top)
        self.assertFalse(sarr.is_subtype_of(ClassRef('java/lang/String'), self.reg))
        self.assertFalse(ClassRef('java/lang/String').is_subtype_of(sarr, self.reg))


class SubtypeTypeVarNullTests(unittest.TestCase):

    def setUp(self):
        self.reg = _registry()

    def test_typevar_via_bound(self):
        t = TypeVar('T', ClassRef('java/lang/Number'))
        self.assertTrue(t.is_subtype_of(ClassRef('java/lang/Object'), self.reg))
        self.assertTrue(t.is_subtype_of(ClassRef('java/lang/Number'), self.reg))
        self.assertFalse(t.is_subtype_of(ClassRef('java/lang/Integer'), self.reg))
        # 目标侧：X <: T(Number) 等价于 X <: Number
        self.assertTrue(ClassRef('java/lang/Integer').is_subtype_of(t, self.reg))
        self.assertFalse(ClassRef('java/lang/String').is_subtype_of(t, self.reg))
        # 无界变量：上界为 Object
        self.assertTrue(TypeVar('X').is_subtype_of(
            ClassRef('java/lang/Object'), self.reg))
        self.assertFalse(TypeVar('X').is_subtype_of(
            ClassRef('java/lang/Number'), self.reg))

    def test_null_bottom(self):
        for target in (ClassRef('java/lang/String'),
                       Array(ClassRef('java/lang/String')),
                       TypeVar('T', ClassRef('java/lang/Number')),
                       Wildcard('+', ClassRef('java/lang/String'))):
            self.assertTrue(Null().is_subtype_of(target, self.reg), msg=repr(target))
        self.assertTrue(Null().is_subtype_of(Null()))
        self.assertFalse(Null().is_subtype_of(Primitive('int')))
        self.assertFalse(ClassRef('java/lang/String').is_subtype_of(Null()))


class WildcardTests(unittest.TestCase):

    def setUp(self):
        self.reg = _registry()
        self.list_of = lambda arg: ClassRef('java/util/List', (arg,))

    def test_containment_extends(self):
        self.assertTrue(self.list_of(ClassRef('java/lang/String')).is_subtype_of(
            self.list_of(Wildcard('+', ClassRef('java/lang/CharSequence'))), self.reg))
        self.assertFalse(self.list_of(ClassRef('java/lang/String')).is_subtype_of(
            self.list_of(Wildcard('+', ClassRef('java/lang/Number'))), self.reg))

    def test_containment_unbounded(self):
        self.assertTrue(self.list_of(ClassRef('java/lang/String')).is_subtype_of(
            self.list_of(Wildcard('*')), self.reg))

    def test_containment_super(self):
        """? super B 接受 B 及其超类型：List<Number>/<Object> 属 List<? super Number>，
        List<Integer> 不属（Integer 是 Number 的子类型）。"""
        self.assertTrue(self.list_of(ClassRef('java/lang/Number')).is_subtype_of(
            self.list_of(Wildcard('-', ClassRef('java/lang/Number'))), self.reg))
        self.assertTrue(self.list_of(ClassRef('java/lang/Object')).is_subtype_of(
            self.list_of(Wildcard('-', ClassRef('java/lang/Number'))), self.reg))
        self.assertFalse(self.list_of(ClassRef('java/lang/Integer')).is_subtype_of(
            self.list_of(Wildcard('-', ClassRef('java/lang/Number'))), self.reg))
        self.assertFalse(self.list_of(ClassRef('java/lang/String')).is_subtype_of(
            self.list_of(Wildcard('-', ClassRef('java/lang/Number'))), self.reg))

    def test_args_invariant_without_wildcard(self):
        """Java 泛型不协变：List<String> 不是 List<Object> 的子类型。"""
        self.assertFalse(self.list_of(ClassRef('java/lang/String')).is_subtype_of(
            self.list_of(ClassRef('java/lang/Object')), self.reg))

    def test_raw_target_unchecked(self):
        self.assertTrue(ClassRef('java/util/ArrayList', (ClassRef('java/lang/String'),))
                        .is_subtype_of(ClassRef('java/util/ArrayList'), self.reg))
        self.assertFalse(ClassRef('java/util/ArrayList').is_subtype_of(
            ClassRef('java/util/ArrayList', (ClassRef('java/lang/String'),)), self.reg))

    def test_arity_mismatch(self):
        a1 = ClassRef('p/Box', (ClassRef('java/lang/String'),))
        a2 = ClassRef('p/Box', (ClassRef('java/lang/String'),
                                ClassRef('java/lang/String')))
        self.assertFalse(a1.is_subtype_of(a2, self.reg))
        self.assertFalse(a2.is_subtype_of(a1, self.reg))


class ClosureCacheTests(unittest.TestCase):

    def test_cache_visible_effect_and_invalidation(self):
        """缓存命中不改变判定；registry 增类（长度变化）后判定随之更新。"""
        reg = _registry()
        al = ClassRef('java/util/ArrayList')
        self.assertFalse(al.is_subtype_of(ClassRef('java/util/List2'), reg))
        # 判定后向 registry 增类：ArrayList2 extends ArrayList
        reg['java/util/ArrayList2'] = _ci('java/util/ArrayList2',
                                          super_class='java/util/ArrayList',
                                          interfaces=['java/util/List'])
        al2 = ClassRef('java/util/ArrayList2')
        self.assertTrue(al2.is_subtype_of(ClassRef('java/util/List'), reg))
        self.assertTrue(al2.is_subtype_of(ClassRef('java/lang/Iterable'), reg))
        # 同 registry 身份下闭包集包含自身
        self.assertIn('java/util/ArrayList2', _super_closure('java/util/ArrayList2', reg))

    def test_unregistered_ancestor_is_closure_leaf(self):
        """闭包上的未注册祖先：作为目标可命中（binary / 短名），但无出边。"""
        reg = _registry()
        reg['p/A'] = _ci('p/A', super_class='q/Unknown')
        self.assertIn('q/Unknown', _super_closure('p/A', reg))
        self.assertTrue(ClassRef('p/A').is_subtype_of(ClassRef('q/Unknown'), reg))
        # 短名占位目标（registry 外的 Rust 短名）同样命中——见 _closure_hit 文档
        self.assertTrue(ClassRef('p/A').is_subtype_of(ClassRef('Unknown'), reg))


class ConstructionTests(unittest.TestCase):

    def test_from_descriptor(self):
        self.assertEqual(from_descriptor('I'), Primitive('int'))
        self.assertEqual(from_descriptor('V'), Primitive('void'))
        self.assertEqual(from_descriptor('[I'), Array(Primitive('int')))
        self.assertEqual(from_descriptor('Ljava/lang/String;'),
                         ClassRef('java/lang/String'))
        self.assertEqual(from_descriptor('[[Ljava/lang/Object;'),
                         Array(Array(ClassRef('java/lang/Object'))))
        for bad in ('', 'X', 'LI', 'Ljava/lang/String', 'I['):
            with self.assertRaises(ValueError, msg=repr(bad)):
                from_descriptor(bad)

    def test_from_signature(self):
        self.assertEqual(from_signature('I'), Primitive('int'))
        self.assertEqual(from_signature('Ljava/util/List<Ljava/lang/String;>;'),
                         ClassRef('java/util/List', (ClassRef('java/lang/String'),)))
        self.assertEqual(from_signature('TT;'), TypeVar('T'))
        self.assertEqual(from_signature('[TT;'), Array(TypeVar('T')))
        self.assertEqual(from_signature('*'), Wildcard('*'))
        self.assertEqual(from_signature('+TT;'), Wildcard('+', TypeVar('T')))
        self.assertEqual(from_signature('-Ljava/lang/Number;'),
                         Wildcard('-', ClassRef('java/lang/Number')))
        self.assertEqual(from_signature('[Ljava/lang/String;'),
                         Array(ClassRef('java/lang/String')))
        for bad in ('', 'T', 'Ljava/util/List<;', 'Lp/Foo;X'):
            with self.assertRaises(ValueError, msg=repr(bad)):
                from_signature(bad)

    def test_from_signature_inner_class(self):
        """LOuter<TT;>.Inner<Ljava/lang/String;>; → Outer$Inner，实参取最内段。"""
        t = from_signature('LOuter<TT;>.Inner<Ljava/lang/String;>;')
        self.assertEqual(t, ClassRef('Outer$Inner', (ClassRef('java/lang/String'),)))

    def test_from_signature_is_interface_flag(self):
        reg = {'p/Iface': _ci('p/Iface', is_interface=True)}
        t = from_signature('Lp/Iface;', registry=reg)
        self.assertTrue(t.is_interface)
        self.assertFalse(from_signature('Lp/Iface;').is_interface)

    def test_class_of(self):
        reg = _registry()
        t = JvmType.class_of('java/util/List', reg)
        self.assertTrue(t.is_interface)
        self.assertEqual(t, ClassRef('java/util/List', (), True))
        self.assertEqual(JvmType.class_of('p/Unknown', reg), ClassRef('p/Unknown'))


class DisplayAndHashTests(unittest.TestCase):

    def test_frozen_and_hashable(self):
        a = ClassRef('java/util/List', (ClassRef('java/lang/String'),), True)
        b = ClassRef('java/util/List', (ClassRef('java/lang/String'),), True)
        self.assertEqual(a, b)
        self.assertEqual(hash(a), hash(b))
        self.assertEqual(len({a, b, ClassRef('java/util/List')}), 2)
        with self.assertRaises(Exception):
            a.binary = 'x'  # type: ignore[misc]

    def test_to_display(self):
        self.assertEqual(Primitive('int').to_display(), 'int')
        self.assertEqual(ClassRef('java/util/ArrayList',
                                  (ClassRef('java/lang/String'),)).to_display(),
                         'java.util.ArrayList<java.lang.String>')
        self.assertEqual(Array(Primitive('int')).to_display(), 'int[]')
        self.assertEqual(TypeVar('T', ClassRef('java/lang/Number')).to_display(),
                         'T extends java.lang.Number')
        self.assertEqual(Wildcard('+', ClassRef('java/lang/String')).to_display(),
                         '? extends java.lang.String')
        self.assertEqual(Wildcard('-').to_display(), '? super Object')
        self.assertEqual(Null().to_display(), 'null')


if __name__ == '__main__':
    unittest.main()

"""
stack.py 擦除查询入口单元测试（TypeIR 批次 2 新增消费面，不依赖 JDK）：
    python3 -m unittest tests.unit.test_erased_queries

覆盖：erased_base（erasure 的字符串边界投影 + 与旧 split 形态的逐形等价
——行为零变化契约）、erased_class_of（registry 域内 ClassRef 构造 /
is_interface 补全 / 域外 None 回退）、is_jvm_array（Array 变体发射形态）。
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.jvm_type import ClassRef
from codegen.stack import erased_base, erased_class_of, is_jvm_array
from codegen.types import ClassInfo


def _ci(name: str, super_class: str = 'java/lang/Object', interfaces=(),
        is_interface: bool = False) -> ClassInfo:
    return ClassInfo(name=name, fields=[], methods=[], super_class=super_class,
                     interfaces=list(interfaces), is_interface=is_interface)


def _registry() -> dict:
    """JDK 形态迷你 fixture（与 test_jvm_type 同形）。"""
    return {
        'java/lang/Object':       _ci('java/lang/Object', ''),
        'java/lang/String':       _ci('java/lang/String'),
        'java/lang/CharSequence': _ci('java/lang/CharSequence', is_interface=True),
        'java/util/List':         _ci('java/util/List', is_interface=True),
        'java/util/ArrayList':    _ci('java/util/ArrayList',
                                      interfaces=['java/util/List']),
        'java/util/Map$Entry':    _ci('java/util/Map$Entry',
                                      interfaces=['java/lang/CharSequence']),
    }


REG = _registry()

# 转译管线实际出现的 Rust 类型串形态（render_type / jvm_to_rust / LVTT hint 的输出域）
_RUST_TYPE_SHAPES = [
    'Object', 'String', 'ArrayList', 'i32', 'bool', 'E', 'K',
    'ArrayList<String>', 'HashMap_TreeNode<K, V>', 'List<Consumer<T>>',
    'JArray<i32>', 'JArray<JArray<Object>>', 'JArray<Entry<K, V>>',
    'Rc<RefCell<Vec<E>>>', ' Option<String>', 'Consumer',
]


class ErasedBaseTests(unittest.TestCase):

    def test_ident_projection(self):
        """擦除基名 = 首标识符（泛型实参不参与裸名判定）。"""
        self.assertEqual(erased_base('ArrayList<String>'), 'ArrayList')
        self.assertEqual(erased_base('JArray<Entry<K, V>>'), 'JArray')
        self.assertEqual(erased_base('HashMap_TreeNode<K, V>'), 'HashMap_TreeNode')
        self.assertEqual(erased_base('ArrayList'), 'ArrayList')
        self.assertEqual(erased_base(' Option<String>'), 'Option')

    def test_non_ident_passthrough(self):
        """非标识符开头（`() `等擦除外形态）保持原串——下游 registry 反查同不可解析。"""
        self.assertEqual(erased_base('()'), '()')
        self.assertEqual(erased_base(''), '')

    def test_legacy_equivalence(self):
        """与旧 split 形态逐形等价（行为零变化契约）：所有实际形态上
        erased_base(s) == s.split 首段去空白。"""
        for s in _RUST_TYPE_SHAPES:
            self.assertEqual(erased_base(s), s.split('<')[0].strip(), msg=repr(s))


class ErasedClassOfTests(unittest.TestCase):

    def test_resolved_class(self):
        """registry 域内：binary 身份 + is_interface 补全（JvmType.class_of）。"""
        ref = erased_class_of('ArrayList<String>', REG)
        self.assertEqual(ref, ClassRef('java/util/ArrayList', (), False))
        self.assertFalse(ref.is_interface)

    def test_resolved_interface_flag(self):
        ref = erased_class_of('List<T>', REG)
        self.assertEqual(ref, ClassRef('java/util/List', (), True))
        self.assertTrue(ref.is_interface)

    def test_inner_class_dollar_short(self):
        """$ 短名（Map$Entry → Map_Entry）经短名索引解析。"""
        ref = erased_class_of('Map_Entry<K, V>', REG)
        self.assertEqual(ref, ClassRef('java/util/Map$Entry', (), False))

    def test_out_of_domain_none(self):
        """域外短名 / 非标识符形态 / registry 缺失 → None（调用方回退擦除路径）。"""
        self.assertIsNone(erased_class_of('JArray<Object>', REG))
        self.assertIsNone(erased_class_of('Unknown<T>', REG))
        self.assertIsNone(erased_class_of('()', REG))
        self.assertIsNone(erased_class_of('ArrayList', None))
        self.assertIsNone(erased_class_of('', REG))

    def test_erasure_identity(self):
        """产出的 ClassRef 已是擦除形态：erasure() 幂等不动。"""
        ref = erased_class_of('ArrayList<String>', REG)
        self.assertIs(ref.erasure(), ref)


class IsJvmArrayTests(unittest.TestCase):

    def test_array_shapes(self):
        self.assertTrue(is_jvm_array('JArray<i32>'))
        self.assertTrue(is_jvm_array('JArray<JArray<Object>>'))
        self.assertTrue(is_jvm_array('JArray<Entry<K, V>>'))

    def test_non_array_shapes(self):
        """裸 JArray（无实参）/ Rust 容器 / 类名都不是 JVM 数组形态。"""
        self.assertFalse(is_jvm_array('JArray'))
        self.assertFalse(is_jvm_array('Vec<String>'))
        self.assertFalse(is_jvm_array('Object'))
        self.assertFalse(is_jvm_array(''))


if __name__ == '__main__':
    unittest.main()

"""
TypeIR 余项位点替换的等价性单元测试（清单第 5 项，调研
docs/reports/2026-09-24-typeir-remaining-survey.md）：
    python3 -m unittest tests.unit.test_typeir_sites

每个被替换的类型字符串手术（`split('<')[0]` 头部解剖 / `startswith('JArray<')`
形态探测 / 数组元素切片）与替换后的 TypeIR 边界查询（stack.erased_base /
erased_class_of / is_jvm_array、type_args.split_rust_type_args）逐形对拍——
行为零变化契约（生成树逐字节一致的单元级保证）。
"""

import os
import sys
import unittest

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen.stack import erased_base, erased_class_of, is_jvm_array
from codegen.type_args import split_rust_type_args
from codegen.type_map import _registry_short_index, configure_short_names
from codegen.types import ClassInfo
from codegen.instr.hierarchy import _is_interface


def _ci(name: str, super_class: str = 'java/lang/Object', interfaces=(),
        is_interface: bool = False) -> ClassInfo:
    return ClassInfo(name=name, fields=[], methods=[], super_class=super_class,
                     interfaces=list(interfaces), is_interface=is_interface)


def _registry() -> dict:
    return {
        'java/lang/Object':       _ci('java/lang/Object', ''),
        'java/lang/String':       _ci('java/lang/String'),
        'java/lang/CharSequence': _ci('java/lang/CharSequence', is_interface=True),
        'java/util/List':         _ci('java/util/List', is_interface=True),
        'java/util/ArrayList':    _ci('java/util/ArrayList', interfaces=['java/util/List']),
        'java/util/Map$Entry':    _ci('java/util/Map$Entry', is_interface=True),
    }


# 被替换位点的输入形态全集（报告 §5.1 要求的覆盖面 + 边界形态）
FORMS = [
    'ArrayList', 'ArrayList<String>', 'List<Map_Entry<K, V>>', 'Map_Entry<K, V>',
    'JArray<i32>', 'JArray<JArray<String>>', 'JArray<ArrayList<String>>', 'JArray',
    'Object', 'String', 'CharSequence', 'i32', 'bool', 'usize', 'u8', '()',
    'T', 'K', 'Unknown', 'Unknown<T>', '', '&ArrayList', '&JArray<i32>',
    ' ArrayList<String>',
]


def _old_head(ty: str) -> str:
    return ty.split('<')[0].strip()


class HeadEquivalence(unittest.TestCase):
    """S4/S5/S7/S8：头部解剖 → erased_base。"""

    def test_identifier_heads_identical(self):
        # 标识符开头的形态逐字相同（S7 的 == short_cls 比较、S8 的 != cls 比较与
        # 下游 registry 反查直接消费此串）
        for ty in FORMS:
            if ty.strip()[:1].isalnum() or ty.strip()[:1] == '_':
                self.assertEqual(erased_base(ty), _old_head(ty), ty)

    def test_non_identifier_heads_resolve_alike(self):
        # 非标识符开头（`&X` / `()` / 空串）：文本可不同，但两者都不命中
        # registry 短名索引、都不等于任何类短名——消费面判定一致
        reg = _registry()
        configure_short_names(reg)
        idx = _registry_short_index(reg)
        for ty in ('&ArrayList', '&JArray<i32>', '()', ''):
            self.assertIsNone(idx.get(erased_base(ty)), ty)
            self.assertIsNone(idx.get(_old_head(ty)), ty)


class RegistryMembership(unittest.TestCase):
    """S2/S3：`_registry_short_index(reg).get(head) is not None` →
    `erased_class_of(ty, reg) is not None`；S6：_is_interface。"""

    def test_membership_equivalent(self):
        reg = _registry()
        configure_short_names(reg)
        idx = _registry_short_index(reg)
        for ty in FORMS:
            old = idx.get(_old_head(ty)) is not None
            new = erased_class_of(ty, reg) is not None
            self.assertEqual(old, new, ty)

    def test_is_interface_equivalent(self):
        reg = _registry()
        configure_short_names(reg)
        idx = _registry_short_index(reg)
        for ty in FORMS:
            ci = idx.get(_old_head(ty))
            old = ci is not None and ci.is_interface
            self.assertEqual(_is_interface(ty, reg), old, ty)
        self.assertFalse(_is_interface('List', None))

    def test_no_registry(self):
        for ty in FORMS:
            self.assertIsNone(erased_class_of(ty, None), ty)


class ArrayForm(unittest.TestCase):
    """S1/S9：`startswith('JArray<')` → is_jvm_array；元素切片 →
    split_rust_type_args(target)[0]。"""

    def test_array_probe_equivalent(self):
        for ty in FORMS:
            self.assertEqual(is_jvm_array(ty), ty.startswith('JArray<'), ty)

    def test_element_extraction_equivalent(self):
        for ty in FORMS:
            if ty.startswith('JArray<') and ty.endswith('>'):
                self.assertEqual(split_rust_type_args(ty)[0], ty[len('JArray<'):-1], ty)


if __name__ == '__main__':
    unittest.main()

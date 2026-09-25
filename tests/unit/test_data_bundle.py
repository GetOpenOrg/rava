"""
纯数据资源束结构判定单元测试（L-1 L1-a）：
    python3 -m unittest tests.unit.test_data_bundle

合成 ClassInfo 覆盖判定的每条分支（载体链、方法集、操作码白名单、接口），
真实 jmod 覆盖 CLDR 束（java.base 与 jdk.localedata 两个模块）与非数据 sun/ 类；
另验证 _is_boundary_class 经注册加载器放行、未注册时维持纯前缀规则。
"""

import os
import sys
import unittest
from pathlib import Path
from types import SimpleNamespace as NS

sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))

from codegen import callchain
from codegen.data_bundle import carrier_of, is_pure_data_bundle

CARRIER = 'java/util/ListResourceBundle'
CARRIER_M = ('getContents', '()[[Ljava/lang/Object;')


def _ins(*ops):
    return [NS(opcode=o) for o in ops]


def _m(name, desc, ops, static=False):
    return NS(name=name, descriptor=desc, instrs=_ins(*ops), is_static=static)


def _ctor():
    return _m('<init>', '()V', ('aload_0', 'invokespecial', 'return'))


def _data_body():
    return _m(*CARRIER_M, ('bipush', 'anewarray', 'dup', 'iconst_0', 'ldc', 'aastore', 'areturn'))


def _cls(name, sup, methods, iface=False):
    return NS(name=name, super_class=sup, methods=methods, is_interface=iface)


def _loader(*classes):
    table = {c.name: c for c in classes}
    table[CARRIER] = _cls(CARRIER, 'java/util/ResourceBundle', [])
    table['java/util/ResourceBundle'] = _cls('java/util/ResourceBundle', 'java/lang/Object', [])
    table['java/lang/Object'] = _cls('java/lang/Object', None, [])
    return table.get


class Synthetic(unittest.TestCase):

    def test_direct_subclass_true(self):
        c = _cls('x/Data', CARRIER, [_ctor(), _data_body()])
        self.assertTrue(is_pure_data_bundle(c, _loader(c)))

    def test_indirect_subclass_true(self):
        mid = _cls('x/Mid', CARRIER, [_ctor(), _data_body()])
        c = _cls('x/Leaf', 'x/Mid', [_ctor(), _data_body()])
        self.assertEqual(carrier_of(c, _loader(mid, c)), CARRIER_M)
        self.assertTrue(is_pure_data_bundle(c, _loader(mid, c)))

    def test_clinit_allowed(self):
        c = _cls('x/Data', CARRIER, [_ctor(), _data_body(),
                                     _m('<clinit>', '()V', ('return',), static=True)])
        self.assertTrue(is_pure_data_bundle(c, _loader(c)))

    def test_carrier_itself_false(self):
        load = _loader()
        self.assertFalse(is_pure_data_bundle(load(CARRIER), load))

    def test_not_under_carrier_false(self):
        c = _cls('x/Other', 'java/lang/Object', [_ctor(), _data_body()])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_broken_super_chain_false(self):
        c = _cls('x/Orphan', 'x/Missing', [_ctor(), _data_body()])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_none_and_interface_false(self):
        self.assertFalse(is_pure_data_bundle(None, _loader()))
        c = _cls('x/I', CARRIER, [_data_body()], iface=True)
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_no_carrier_method_false(self):
        c = _cls('x/Empty', CARRIER, [_ctor()])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_extra_method_false(self):
        c = _cls('x/Extra', CARRIER, [_ctor(), _data_body(),
                                      _m('helper', '()V', ('return',))])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_static_carrier_signature_false(self):
        c = _cls('x/Static', CARRIER, [_ctor(), _m(*CARRIER_M, ('aconst_null', 'areturn'),
                                                   static=True)])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_invoke_in_body_false(self):
        body = _m(*CARRIER_M, ('ldc', 'invokestatic', 'areturn'))
        c = _cls('x/Calls', CARRIER, [_ctor(), body])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_field_access_in_body_false(self):
        body = _m(*CARRIER_M, ('getstatic', 'areturn'))
        c = _cls('x/Reads', CARRIER, [_ctor(), body])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))

    def test_ctor_side_effect_false(self):
        ctor = _m('<init>', '()V', ('aload_0', 'invokespecial', 'invokestatic', 'return'))
        c = _cls('x/Ctor', CARRIER, [ctor, _data_body()])
        self.assertFalse(is_pure_data_bundle(c, _loader(c)))


class BoundaryDecision(unittest.TestCase):

    def tearDown(self):
        callchain.set_data_bundle_loader(None)

    def test_prefix_rule_without_loader(self):
        callchain.set_data_bundle_loader(None)
        self.assertTrue(callchain._is_boundary_class('sun/x/Data'))

    def test_exempt_with_loader(self):
        data = _cls('sun/x/Data', CARRIER, [_ctor(), _data_body()])
        logic = _cls('sun/x/Logic', 'java/lang/Object', [_ctor()])
        callchain.set_data_bundle_loader(_loader(data, logic))
        self.assertFalse(callchain._is_boundary_class('sun/x/Data'))
        self.assertTrue(callchain._is_boundary_class('sun/x/Logic'))
        self.assertTrue(callchain._is_boundary_class('sun/x/Unknown'))

    def test_public_package_untouched(self):
        callchain.set_data_bundle_loader(_loader())
        self.assertFalse(callchain._is_boundary_class('java/util/ArrayList'))


def _jdk_homes():
    for major in (21, 25):
        for h in (f'/usr/lib/jvm/java-{major}-openjdk-amd64',):
            if (Path(h) / 'jmods').is_dir():
                yield major, h


class RealJmods(unittest.TestCase):

    def test_cldr_bundles(self):
        from codegen.classfile import parse_class_bytes
        from codegen.jdk_resolver import JdkResolver
        homes = list(_jdk_homes())
        if not homes:
            self.skipTest('本机无 JDK jmods')
        for major, home in homes:
            r = JdkResolver(home)
            cache = {}

            def load(n):
                if n not in cache:
                    d = r.resolve(n)
                    cache[n] = parse_class_bytes(d, n) if d else None
                return cache[n]

            with self.subTest(jdk=major):
                # java.base（ROOT / en）与 jdk.localedata（ext）两模块的 CLDR 束
                for n in ('sun/text/resources/cldr/FormatData',
                          'sun/text/resources/cldr/FormatData_en',
                          'sun/text/resources/cldr/ext/FormatData_fr',
                          'sun/text/resources/cldr/ext/FormatData_it'):
                    self.assertTrue(is_pure_data_bundle(load(n), load), n)
                for n in ('sun/util/locale/BaseLocale',
                          'sun/util/resources/LocaleData',
                          'java/util/ListResourceBundle'):
                    self.assertFalse(is_pure_data_bundle(load(n), load), n)


if __name__ == '__main__':
    unittest.main()

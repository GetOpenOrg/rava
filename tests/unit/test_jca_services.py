"""
JCA 服务清单查询单元测试（K-JCA）：
    python3 -m unittest tests.unit.test_jca_services

覆盖边界放行判定（包前缀 / 类及其嵌套类）与 provider 名查询。服务表抽取与种子选择
已由 Rust 闭包分析器承担（generator/crates/closure/src/seeds/jca.rs）。
"""

import os
import sys
import unittest

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, ROOT)

from codegen.jca_services import JcaManifest, provider_class, released


class Release(unittest.TestCase):
    MF = JcaManifest(release=('x/impl/', 'y/Engine'))

    def test_package_prefix(self):
        self.assertTrue(released('x/impl/Foo', self.MF))
        self.assertTrue(released('x/impl/sub/Bar$1', self.MF))
        self.assertFalse(released('x/implother/Foo', self.MF))

    def test_class_and_nested(self):
        self.assertTrue(released('y/Engine', self.MF))
        self.assertTrue(released('y/Engine$Delegate', self.MF))
        self.assertFalse(released('y/EngineSpi', self.MF))


class Provider(unittest.TestCase):
    MF = JcaManifest(providers=(('P1', 'a/Reg', 'a/P1Provider'), ('P2', 'b/Reg', 'b/P2Provider')))

    def test_lookup(self):
        self.assertEqual(provider_class('P2', self.MF), 'b/P2Provider')
        self.assertIsNone(provider_class('P3', self.MF))


if __name__ == '__main__':
    unittest.main()

"""
JCA 服务注册单元测试（K-JCA）：
    python3 -m unittest tests.unit.test_jca_services

合成字节码覆盖服务三元组抽取（三连常量、类名须可加载、非类名第三项、夹杂非字符串常量）、
算法名收集（大小写、transformation 首段）、种子选择（engine 类型过滤）、边界放行判定；
真实 jmod 验证 JDK21 / JDK25 的 SunJCE / SunEntries 抽取结果含 DES / MD5 / SHA 族。
"""

import os
import sys
import unittest
from pathlib import Path
from types import SimpleNamespace as NS

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, ROOT)

from codegen.jca_services import (JcaManifest, Service, extract_services, released,
                                  select_services, user_algorithm_strings)


def I(op, comment=None):
    return NS(opcode=op, comment=comment, operand=None)


def S(v):
    return I('ldc', f'String {v}')


def M(*instrs, name='m'):
    return NS(name=name, descriptor='()V', instrs=list(instrs))


def C(*methods):
    return NS(methods=list(methods))


class Extract(unittest.TestCase):

    def setUp(self):
        self.classes = {
            'p/Prov': C(M(I('aload_0'), S('Cipher'), S('DES'), S('p.impl.DesCipher'), I('aconst_null'),
                          I('invokevirtual', 'Method p/Prov.ps:(...)V'),
                          # 第三项不是类名（无点）→ 不是服务
                          S('Cipher'), S('AES'), S('AESCipher'),
                          # 类名不可加载 → 丢弃
                          S('Cipher'), S('RC9'), S('p.impl.Missing'),
                          # 中间夹非字符串常量 → 不连续
                          S('Mac'), I('iconst_1'), S('HmacX'), S('p.impl.Hmac'),
                          # 属性键值对后跟服务（滑窗必须只取连续三项）
                          S('SupportedModes'), S('ECB|CBC'), S('KeyGenerator'), S('DES'),
                          S('p.impl.DesKeyGen')),
                        M(S('MessageDigest'), S('MD5'), S('p.impl.Md5'), name='other')),
            'p/impl/DesCipher': C(), 'p/impl/Md5': C(), 'p/impl/Hmac': C(), 'p/impl/DesKeyGen': C(),
        }
        self.mf = JcaManifest(providers=(('P', 'p/Prov'), ('Q', 'p/Missing')))

    def test_triples(self):
        got = extract_services(self.classes.get, self.mf)
        self.assertEqual(got, sorted([
            Service('Cipher', 'DES', 'p/impl/DesCipher', 'P'),
            Service('KeyGenerator', 'DES', 'p/impl/DesKeyGen', 'P'),
            Service('MessageDigest', 'MD5', 'p/impl/Md5', 'P'),
        ]))

    def test_empty_manifest(self):
        self.assertEqual(extract_services(self.classes.get, JcaManifest()), [])


class Select(unittest.TestCase):
    SV = [Service('Cipher', 'DES', 'a/DesCipher', 'P'),
          Service('KeyGenerator', 'DES', 'a/DesKeyGen', 'P'),
          Service('MessageDigest', 'SHA-256', 'a/Sha256', 'Q'),
          Service('MessageDigest', 'MD5', 'a/Md5', 'Q')]

    def test_user_strings(self):
        users = [C(M(S('DES/ECB/PKCS5Padding'), S('sha-256'), S('hello world'), I('iconst_0'))),
                 C(M(S('Md5')))]
        self.assertEqual(user_algorithm_strings(users), {'des', 'sha-256', 'hello world', 'md5'})

    def test_type_filter(self):
        got = select_services(self.SV, {'des', 'md5'}, {'Cipher', 'MessageDigest'})
        self.assertEqual([s.impl for s in got], ['a/DesCipher', 'a/Md5'])

    def test_no_live_engine(self):
        self.assertEqual(select_services(self.SV, {'des', 'md5', 'sha-256'}, set()), [])

    def test_case_insensitive(self):
        got = select_services(self.SV, user_algorithm_strings([C(M(S('Sha-256')))]), {'MessageDigest'})
        self.assertEqual([s.algorithm for s in got], ['SHA-256'])


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


class RealJdk(unittest.TestCase):

    def test_sun_providers_both_jdks(self):
        from codegen.classfile import parse_class_bytes
        from codegen.jca_services import load_manifest
        from codegen.jdk_resolver import JdkResolver
        mf = load_manifest()
        seen = 0
        for major in (21, 25):
            home = Path(f'/usr/lib/jvm/java-{major}-openjdk-amd64')
            if not (home / 'jmods').is_dir():
                continue
            seen += 1
            r, cache = JdkResolver(home), {}

            def load(n):
                if n not in cache:
                    d = r.resolve(n)
                    cache[n] = parse_class_bytes(d, n) if d else None
                return cache[n]

            with self.subTest(jdk=major):
                got = {(s.type, s.algorithm): s for s in extract_services(load, mf)}
                self.assertGreater(len(got), 150)
                self.assertEqual(got[('Cipher', 'DES')].provider, 'SunJCE')
                for key in (('MessageDigest', 'MD5'), ('MessageDigest', 'SHA-1'),
                            ('MessageDigest', 'SHA-256'), ('SecureRandom', 'NativePRNG')):
                    self.assertEqual(got[key].provider, 'SUN')
                    self.assertTrue(released(got[key].impl, mf))
        if not seen:
            self.skipTest('本机无 JDK')


if __name__ == '__main__':
    unittest.main()

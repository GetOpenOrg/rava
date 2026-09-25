"""
Locale 种子单元测试（L-1 L1-b）：
    python3 -m unittest tests.unit.test_locale_seed

合成字节码覆盖三类引用（const 溯源 / factory 字面量 / tag 字面量）与各失败分支，
标签解析与父链覆盖全部形态；真实 jmod + javac 编译的 TestLocaleConstants 验证
JDK21 / JDK25 常量表下标顺序不同时种子集一致，且含 fr_FR / it_IT 链的资源束。
"""

import glob
import os
import shutil
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace as NS

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, ROOT)

from codegen.locale_seed import (LocaleSeeds, collect_locales, parent_chain,
                                 parse_tag)

L = 'x/Loc'
T = 'x/Base'
MF = LocaleSeeds(const_classes=frozenset({L}),
                 factories=frozenset({f'{L}.of:(Ljava/lang/String;Ljava/lang/String;)L{L};',
                                      f'{L}.<init>:(Ljava/lang/String;)V'}),
                 tags=frozenset({f'{L}.forTag:(Ljava/lang/String;)L{L};'}),
                 bundle_bases=())


def I(op, comment=None, operand=None):
    return NS(opcode=op, comment=comment, operand=operand)


def S(v):
    return I('ldc', f'String {v}' if v else 'String')


def M(name, desc, ins):
    return NS(name=name, descriptor=desc, instrs=ins)


def C(name, methods):
    return NS(name=name, methods=methods)


def _jdk_like():
    """常量字段 A 经 `f(B)` → T.arr[k]；B 直接由字面量工厂构造；Z 非字面量。"""
    loc_clinit = M('<clinit>', '()V', [
        I('iconst_1'), I('invokestatic', f'Method {L}.f:(B)L{L};'),
        I('putstatic', f'Field {L}.A:L{L};'),
        S('pt'), S('br'), I('invokestatic', f'Method {L}.mk:(Ljava/lang/String;Ljava/lang/String;)L{L};'),
        I('putstatic', f'Field {L}.B:L{L};'),
        I('aload_0'), I('invokestatic', f'Method {L}.g:(Ljava/lang/Object;)L{L};'),
        I('putstatic', f'Field {L}.Z:L{L};'),
        I('bipush', operand='7'), I('invokestatic', f'Method {L}.f:(B)L{L};'),
        I('putstatic', f'Field {L}.Q:L{L};'),
    ])
    f = M('f', f'(B)L{L};', [
        I('getstatic', f'Field {T}.arr:[L{T};'), I('iload_0'), I('aaload'), I('areturn')])
    base_clinit = M('<clinit>', '()V', [
        I('bipush', operand='9'), I('anewarray', T), I('astore_0'),
        I('aload_0'), I('iconst_0'), S('en'), S(''),
        I('invokestatic', f'Method {T}.mk:(Ljava/lang/String;Ljava/lang/String;)L{T};'), I('aastore'),
        I('aload_0'), I('iconst_1'), S('fr'), S('FR'),
        I('invokestatic', f'Method {T}.mk:(Ljava/lang/String;Ljava/lang/String;)L{T};'), I('aastore'),
        I('aload_0'), I('putstatic', f'Field {T}.arr:[L{T};'), I('return')])
    table = {L: C(L, [loc_clinit, f]), T: C(T, [base_clinit])}
    return table.get


def _user(*ins):
    return C('u/Main', [M('main', '([Ljava/lang/String;)V', list(ins))])


def _tags(locs):
    return ['_'.join(x for x in loc if x) for loc in locs]


class ParseTag(unittest.TestCase):

    def test_forms(self):
        self.assertEqual(parse_tag('fr'), ('fr', '', '', ''))
        self.assertEqual(parse_tag('FR-fr'), ('fr', '', 'FR', ''))
        self.assertEqual(parse_tag('it_IT'), ('it', '', 'IT', ''))
        self.assertEqual(parse_tag('zh-hant-tw'), ('zh', 'Hant', 'TW', ''))
        self.assertEqual(parse_tag('es-419'), ('es', '', '419', ''))
        self.assertEqual(parse_tag('de-DE-1996'), ('de', '', 'DE', '1996'))
        self.assertEqual(parse_tag('sr-Latn'), ('sr', 'Latn', '', ''))
        self.assertIsNone(parse_tag(''))
        self.assertIsNone(parse_tag('123'))


class ParentChain(unittest.TestCase):

    def test_chains(self):
        self.assertEqual(parent_chain(('fr', '', 'FR', '')), ['fr_FR', 'fr'])
        self.assertEqual(parent_chain(('fr', '', '', '')), ['fr'])
        self.assertEqual(parent_chain(('zh', 'Hant', 'TW', '')), ['zh_Hant_TW', 'zh_Hant', 'zh_TW', 'zh'])
        self.assertEqual(parent_chain(('de', '', 'DE', '1996')), ['de_DE_1996', 'de_DE', 'de'])
        self.assertEqual(parent_chain(('sr', 'Latn', 'RS', 'x')),
                         ['sr_Latn_RS_x', 'sr_Latn_RS', 'sr_Latn', 'sr_RS_x', 'sr_RS', 'sr'])
        self.assertEqual(parent_chain(('', '', '', '')), [])


class Collect(unittest.TestCase):

    def run_on(self, *ins, extra=()):
        return _tags(collect_locales([_user(*ins)], _jdk_like(), extra=extra, manifest=MF))

    def test_default_is_en(self):
        self.assertEqual(self.run_on(), ['en'])

    def test_const_traced_through_table(self):
        self.assertIn('fr_FR', self.run_on(I('getstatic', f'Field {L}.A:L{L};')))

    def test_const_direct_literal_factory(self):
        self.assertIn('pt_BR', self.run_on(I('getstatic', f'Field {L}.B:L{L};')))

    def test_const_non_literal_ignored(self):
        self.assertEqual(self.run_on(I('getstatic', f'Field {L}.Z:L{L};')), ['en'])

    def test_const_index_missing_in_table(self):
        self.assertEqual(self.run_on(I('getstatic', f'Field {L}.Q:L{L};')), ['en'])

    def test_const_unknown_field_and_other_type(self):
        self.assertEqual(self.run_on(I('getstatic', f'Field {L}.NOPE:L{L};'),
                                     I('getstatic', f'Field {L}.A:Ljava/lang/Object;'),
                                     I('getstatic', f'Field y/Other.A:Ly/Other;')), ['en'])

    def test_factory_literals(self):
        got = self.run_on(S('it'), S('IT'),
                          I('invokestatic', f'Method {L}.of:(Ljava/lang/String;Ljava/lang/String;)L{L};'))
        self.assertIn('it_IT', got)

    def test_ctor_literal(self):
        got = self.run_on(I('new', L), I('dup'), S('ja'),
                          I('invokespecial', f'Method {L}."<init>":(Ljava/lang/String;)V'))
        self.assertIn('ja', got)

    def test_factory_non_literal_ignored(self):
        got = self.run_on(S('it'), I('aload_1'),
                          I('invokestatic', f'Method {L}.of:(Ljava/lang/String;Ljava/lang/String;)L{L};'))
        self.assertEqual(got, ['en'])

    def test_unlisted_method_ignored(self):
        got = self.run_on(S('ko'), I('invokestatic', f'Method {L}.other:(Ljava/lang/String;)L{L};'))
        self.assertEqual(got, ['en'])

    def test_tag_literal(self):
        got = self.run_on(S('zh-Hant-TW'), I('invokestatic', f'Method {L}.forTag:(Ljava/lang/String;)L{L};'))
        self.assertIn('zh_Hant_TW', got)

    def test_tag_unparseable_ignored(self):
        got = self.run_on(S('42'), I('invokestatic', f'Method {L}.forTag:(Ljava/lang/String;)L{L};'))
        self.assertEqual(got, ['en'])

    def test_extra_locales(self):
        self.assertEqual(self.run_on(extra=('de-CH', 'bogus!', '1')), ['de_CH', 'en'])


class RealCorpus(unittest.TestCase):

    def test_locale_constants_both_jdks(self):
        from codegen.classfile import parse_class_bytes
        from codegen.jdk_resolver import JdkResolver
        from codegen.locale_seed import bundle_classes
        src = os.path.join(ROOT, 'tests/e2e/56_random_format/TestLocaleConstants.java')
        results = {}
        for major in (21, 25):
            home = Path(f'/usr/lib/jvm/java-{major}-openjdk-amd64')
            if not (home / 'jmods').is_dir():
                continue
            out = tempfile.mkdtemp()
            try:
                subprocess.run([str(home / 'bin/javac'), '-d', out, src], check=True,
                               capture_output=True)
                users = [parse_class_bytes(Path(p).read_bytes(), Path(p).stem)
                         for p in glob.glob(f'{out}/*.class')]
            finally:
                shutil.rmtree(out, ignore_errors=True)
            r, cache = JdkResolver(home), {}

            def load(n):
                if n not in cache:
                    d = r.resolve(n)
                    cache[n] = parse_class_bytes(d, n) if d else None
                return cache[n]

            locs = collect_locales(users, load)
            results[major] = locs
            with self.subTest(jdk=major):
                tags = _tags(locs)
                for t in ('en', 'fr_FR', 'it_IT', 'de_DE', 'en_US', 'zh_TW', 'fr_CA'):
                    self.assertIn(t, tags)
                bundles = bundle_classes(locs, load)
                for b in ('sun/text/resources/cldr/FormatData',
                          'sun/text/resources/cldr/ext/FormatData_fr',
                          'sun/text/resources/cldr/ext/FormatData_it'):
                    self.assertIn(b, bundles)
        if not results:
            self.skipTest('本机无 JDK')
        if len(results) == 2:
            self.assertEqual(results[21], results[25])


if __name__ == '__main__':
    unittest.main()

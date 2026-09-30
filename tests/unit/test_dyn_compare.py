"""动态对照（scripts/dyn_compare.py，闭包计划 C5）的单元测试（不依赖 JDK）：
    python3 -m unittest tests.unit.test_dyn_compare
"""

import os
import sys
import unittest

_ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, _ROOT)
sys.path.insert(0, os.path.join(_ROOT, 'scripts'))

import dyn_compare as dc


def rules(**kw):
    base = dict(vm_boundary={'java/lang/Thread', 'jdk/internal/misc/Unsafe'},
                release=['java/lang/Thread$Builder'],
                vm_upcalls=['java/lang/invoke/MethodHandleNatives'],
                user={'Main'})
    base.update(kw)
    return dc.DomainRules(**base)


class DomainTest(unittest.TestCase):
    def test_order(self):
        r = rules()
        self.assertEqual(r.domain('Main'), 'user')
        self.assertEqual(r.domain('java/lang/Object'), 'root')
        # translate_nested 放行优先于 VM 边界类
        self.assertEqual(r.domain('java/lang/Thread$Builder'), 'translate')
        self.assertEqual(r.domain('java/lang/Thread$Builder$1'), 'translate')
        # VM 边界类连同嵌套类
        self.assertEqual(r.domain('java/lang/Thread$State'), 'boundary')
        self.assertEqual(r.domain('jdk/internal/misc/Unsafe'), 'boundary')
        # 其余一律翻译域（无包前缀截断）
        self.assertEqual(r.domain('java/util/ArrayList'), 'translate')
        self.assertEqual(r.domain('sun/nio/cs/UTF_8'), 'translate')
        self.assertEqual(r.domain('com/foo/Bar'), 'translate')

    def test_entry_matches(self):
        self.assertTrue(dc.entry_matches('a/b/', 'a/b/C'))
        self.assertTrue(dc.entry_matches('a/B', 'a/B$1'))
        self.assertFalse(dc.entry_matches('a/B', 'a/BC'))


XLOG = """\
[0.010s][info][class,load] java.lang.Object source: jrt:/java.base
[0.020s][info][class,load] Main source: file:/x/
[0.021s][info][class,load] java.util.Launcher source: jrt:/java.base
[0.030s][info][class,init] 5 Initializing 'Main' (0x1)
[0.031s][info][class,load] java.util.ArrayList source: jrt:/java.base
[0.032s][info][class,load] Main$$Lambda/0x0000 source: Main
[0.033s][info][class,load] java.util.ArrayList source: shared
"""


class XlogTest(unittest.TestCase):
    def test_program_phase_starts_at_main_init(self):
        loaded, program, hidden = dc.parse_xlog(XLOG, 'Main')
        self.assertIn('java/util/Launcher', loaded)
        self.assertNotIn('java/util/Launcher', program)
        self.assertEqual(program[0], 'java/util/ArrayList')
        self.assertEqual(hidden, {'Main$$Lambda/0x0000'})

    def test_fallback_to_main_load(self):
        text = "\n".join(l for l in XLOG.splitlines() if "class,init" not in l)
        _, program, _ = dc.parse_xlog(text, 'Main')
        self.assertEqual(program[0], 'java/util/Launcher')


AGENT = """\
L java/util/ArrayList main
F java/util/List of ()Ljava/util/List; 3
F Main$$Lambda.0x01 run ()V 1
F Main main ([Ljava/lang/String;)V 5
L java/util/ArrayList main
F Other x ()V 0
L java/lang/Void Reference_Handler
"""


class AgentTest(unittest.TestCase):
    def test_parse_first_event_and_skip_hidden_frames(self):
        ev = dc.parse_agent(AGENT)
        self.assertEqual(ev['java/util/ArrayList'].thread, 'main')
        self.assertEqual([f[0] for f in ev['java/util/ArrayList'].frames],
                         ['java/util/List', 'Main'])
        self.assertEqual(ev['java/lang/Void'].frames, [])


MAIN = ('Main', 'main', '([Ljava/lang/String;)V', 5)
MAIN_ID = 'Main.main:([Ljava/lang/String;)V'


def ev(*top_first):
    return dc.LoadEvent('main', list(top_first))


class AttributeTest(unittest.TestCase):
    def setUp(self):
        self.r = rules()
        self.methods = {MAIN_ID: 'bytecode',
                        'java/util/A.f:()V': 'bytecode',
                        'java/util/H.n:()V': 'handwritten:native'}

    def cat(self, e, sigs=frozenset()):
        return dc.attribute(e, self.methods, self.r, sigs)[0]

    def test_all_modeled_is_class_edge_miss(self):
        self.assertEqual(dc.attribute(ev(('java/util/A', 'f', '()V', 1), MAIN),
                                      self.methods, self.r), (dc.MISS, None))

    def test_missed_method(self):
        c, f = dc.attribute(ev(('java/util/B', 'g', '()V', 2), MAIN), self.methods, self.r)
        self.assertEqual((c, f), (dc.MISS, 'java/util/B.g:()V@2'))

    def test_handwritten(self):
        self.assertEqual(self.cat(ev(('java/util/X', 'y', '()V', 0),
                                     ('java/util/H', 'n', '()V', 0), MAIN)), 'handwritten')

    def test_boundary_code(self):
        self.assertEqual(self.cat(ev(('jdk/internal/misc/Unsafe', 'y', '()V', 0), MAIN)), 'boundary-code')

    def test_vm_upcall(self):
        self.assertEqual(self.cat(ev(('java/lang/invoke/X', 'y', '()V', 0),
                                     ('java/lang/invoke/MethodHandleNatives', 'linkCallSite',
                                      '()V', 0), MAIN)), 'vm-upcall')

    def test_vm_entry(self):
        self.assertEqual(self.cat(ev(('java/lang/Thread2', 'exit', '()V', 0))), 'vm-entry')
        self.assertEqual(self.cat(ev()), 'vm-entry')

    def test_boundary_dispatch(self):
        sigs = dc.boundary_ref_sigs(['java/lang/Thread.get:(I)I', 'java/util/A.f:()V'],
                                    self.r)
        self.assertEqual(sigs, {'get:(I)I'})
        self.assertEqual(self.cat(ev(('java/lang/System$2', 'get', '(I)I', 1), MAIN), sigs),
                         'boundary-dispatch')


def closure():
    return {
        'classes': [
            {'name': 'Main', 'level': 'init', 'via': {'kind': 'main', 'from': {'root': 'main'}}},
            {'name': 'java/util/ArrayList', 'level': 'init',
             'via': {'kind': 'new', 'from': {'method': MAIN_ID}}},
            {'name': 'java/util/Unused', 'level': 'type',
             'via': {'kind': 'hw-type', 'from': {'class': 'java/util/ArrayList'}}},
            {'name': 'java/util/Orphan', 'level': 'type',
             'via': {'kind': 'invoke', 'from': {'method': 'java/util/Gone.m:()V'}}},
        ],
        'methods': [{'id': MAIN_ID, 'kind': 'bytecode'}],
        'refs': [],
    }


class CompareTest(unittest.TestCase):
    def test_compare_and_tag(self):
        xlog = XLOG + "[0.04s][info][class,load] java.util.Missed source: jrt:/java.base\n" \
                      "[0.05s][info][class,load] jdk.internal.misc.Unsafe source: jrt:/java.base\n" \
                      "[0.06s][info][class,load] java.util.NoEvent source: jrt:/java.base\n"
        agent = ("L java/util/Missed main\nF Main main ([Ljava/lang/String;)V 9\n"
                 "L jdk/internal/misc/Unsafe main\nF Main main ([Ljava/lang/String;)V 9\n")
        res = dc.compare(closure(), xlog, agent, rules(), 'Main')
        self.assertEqual([m['class'] for m in res['miss']], ['java/util/Missed'])
        self.assertEqual([m['class'] for m in res['unattributed']], ['java/util/NoEvent'])
        self.assertEqual([m['class'] for m in res['attributed']['boundary']], ['jdk/internal/misc/Unsafe'])
        ex = res['extra']
        self.assertEqual(ex['classes'], ['java/util/Orphan', 'java/util/Unused'])
        self.assertEqual(ex['unexplained'], ['java/util/Orphan'])
        self.assertEqual(dc.provenance_pct(res), '50%')
        self.assertEqual(dc.summary_tag(res), 'dyn miss 1 / extra 2 prov 50% / unattr 1')
        self.assertEqual(dc.summary_tag({'error': 'x'}), 'dyn ERR')

    def test_main_class_of(self):
        self.assertEqual(dc.main_class_of(closure()), 'Main')


if __name__ == '__main__':
    unittest.main()

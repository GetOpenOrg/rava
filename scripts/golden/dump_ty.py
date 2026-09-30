#!/usr/bin/env python3
"""类型层 golden 采集：跑一遍 Python 转译前半程（javac → 闭包 → registry），在
write_cargo_project 入口按与发射层相同的方式构建 registry、配置短名，然后对
registry 内每个类 / 方法 / 字段调用 codegen 类型层函数，逐条写出输入与输出。

Rust 侧（generator/crates/ty/tests/golden.rs）按 meta 行重建同一 registry，
对每条记录调用对应的 Rust API 并比较。

用法：
    python3 scripts/golden/dump_ty.py tests/e2e/33_maps/TestHashMapOps.java [...]
输出：build/golden/ty/<Test>.jsonl

口径：
- 折叠（closure_folds）与 VM 常量剪枝置为恒等：两侧比较同一份原始字节码；
- Rust 类型串按 Python 文本输出；Rust 侧渲染 RsType 后比较；
- JvmType 编码为结构化 JSON（见 _jt）。
"""

from __future__ import annotations

import json
import os
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / 'scripts'))

from jdk_select import apply_jdk  # noqa: E402

import codegen.closure_folds as _folds  # noqa: E402
import codegen.vm_constants as _vm_constants  # noqa: E402
import codegen.closure_input as _closure_input  # noqa: E402
import codegen.transpile  # noqa: E402,F401
from codegen import type_map as TM  # noqa: E402
from codegen import sig_parse as SP  # noqa: E402
from codegen import type_args as TA  # noqa: E402
from codegen import sig_types as ST  # noqa: E402
from codegen import jvm_type as JT  # noqa: E402
from codegen.constants import safe_ident  # noqa: E402
from codegen.render import render_type  # noqa: E402

_folds.load = lambda path: 0
_vm_constants.prune_dead_guards = lambda instrs, exception_table: instrs

_RESOLVERS: list = []


class _CapturingResolver(_closure_input.JdkResolver):
    def __init__(self, *a, **kw):
        super().__init__(*a, **kw)
        _RESOLVERS.append(self)


_closure_input.JdkResolver = _CapturingResolver
# codegen/__init__ 以同名函数遮蔽了子模块属性，经 sys.modules 取模块
_transpile = sys.modules['codegen.transpile']
_transpile._write_jdk_scan_report = lambda *a, **kw: None


class _Done(Exception):
    pass


def _jt(t) -> object:
    """JvmType → 结构化 JSON"""
    if t is None:
        return None
    if isinstance(t, JT.Primitive):
        return {'p': t.kind}
    if isinstance(t, JT.ClassRef):
        return {'c': t.binary, 'a': [_jt(a) for a in t.args], 'i': bool(t.is_interface)}
    if isinstance(t, JT.Array):
        return {'arr': _jt(t.elem)}
    if isinstance(t, JT.TypeVar):
        return {'tv': t.name, 'b': _jt(t.bound)}
    if isinstance(t, JT.Wildcard):
        return {'w': t.kind, 'b': _jt(t.bound)}
    if isinstance(t, JT.Null):
        return {'null': True}
    if isinstance(t, JT.HostPrim):
        return {'h': t.name}
    raise TypeError(type(t))


class _Writer:
    def __init__(self, fh):
        self.fh = fh
        self.count = 0

    def rec(self, f: str, inp: dict, fn):
        try:
            out = fn()
        except Exception as e:  # noqa: BLE001 —— Python 侧异常本身就是被比较的行为
            out = {'err': type(e).__name__}
        self.fh.write(json.dumps({'f': f, 'in': inp, 'out': out}, ensure_ascii=False) + '\n')
        self.count += 1


def _dump_class(w: _Writer, ci, reg: dict, subclasses: dict):
    c = ci.name
    tps = list(TM.effective_class_type_params(ci, reg))
    w.rec('short', {'c': c}, lambda: TM.short_cls(c))
    w.rec('effective_params', {'c': c}, lambda: tps)
    w.rec('outer_instance_class', {'c': c}, lambda: TM.outer_instance_class(ci))
    w.rec('is_anonymous', {'c': c}, lambda: ST.is_anonymous_class(ci))
    w.rec('jvm_to_rust', {'d': f'L{c};'}, lambda: TM.jvm_to_rust(f'L{c};', reg))
    w.rec('carrier_type', {'c': c}, lambda: JT.carrier_type(c, reg))
    w.rec('superclass_type_args', {'c': c}, lambda: TA.superclass_type_args(ci, reg))
    w.rec('superinterface_type_args', {'c': c},
          lambda: [[k, v] for k, v in TA.superinterface_type_args(ci, reg).items()])
    w.rec('ancestor_type_args', {'c': c},
          lambda: [[k, list(v)] for k, v in TA.ancestor_type_args(ci, reg)])
    w.rec('ancestor_vtable_args_by_short', {'c': c},
          lambda: TA.ancestor_vtable_args_by_short(ci, TM.jvm_to_rust(f'L{c};', reg), reg))
    w.rec('implemented_interface_views', {'c': c},
          lambda: [[k, list(v)] for k, v in TA.implemented_interface_views(ci, reg)])
    w.rec('supertype_signature_args', {'c': c},
          lambda: [[k, list(v)] for k, v in TA._supertype_signature_args(ci, reg)])
    w.rec('interface_signature_views', {'c': c},
          lambda: [[k, v] for k, v in TA.interface_signature_views(ci, reg).items()])
    w.rec('class_type_param_bounds', {'c': c},
          lambda: {k: list(v) for k, v in TA.class_type_param_bounds(ci, reg).items()})
    w.rec('enclosing_scope_type_args', {'c': c}, lambda: TA.enclosing_scope_type_args(ci, reg, []))
    w.rec('overloaded_names', {'c': c}, lambda: sorted(ST.hierarchy_overloaded_names(ci, reg)))
    w.rec('super_closure', {'c': c}, lambda: sorted(JT._super_closure(c, reg)))
    w.rec('class_of', {'c': c}, lambda: _jt(JT.JvmType.class_of(c, reg)))
    w.rec('class_bounds_bin', {'c': c}, lambda: _bounds_bin(ci.generic_signature, reg, tps))
    for m in ci.methods:
        _dump_method(w, ci, m, reg, tps)
    for f in ci.fields:
        _dump_field(w, ci, f, reg, tps, subclasses)


def _bounds_bin(sig: str, reg: dict, tps: list):
    bb: dict = {}
    bounds = SP._extract_method_tparam_bounds(sig, reg, tps, bb)
    return {'bounds': bounds, 'bin': bb}


def _dump_method(w: _Writer, ci, m, reg: dict, tps: list):
    c, n, d, sig = ci.name, m.name, m.descriptor, m.generic_signature or ''
    key = {'c': c, 'n': n, 'd': d}
    w.rec('mangle', key, lambda: TM.mangle_name(n, d))
    w.rec('method_sig_types', key, lambda: list(ST.method_sig_types(ci, m, tps, reg)))
    w.rec('emitted_method_sig_types', key, lambda: list(ST.emitted_method_sig_types(ci, m, tps, reg)))
    w.rec('receiver_member_name', key, lambda: ST.receiver_member_name(n, d, ci, reg))
    w.rec('interface_member_local_name', key, lambda: ST.interface_member_local_name(ci, n, d, reg))
    w.rec('method_name_is_mangled', key, lambda: ST.method_name_is_mangled(ci, m, reg))
    if sig:
        w.rec('parse_method_param_types', {**key, 's': m.is_static},
              lambda: list(SP.parse_method_param_types(sig, tps, reg, m.is_static)))
        w.rec('method_bounds', key, lambda: _bounds_bin(sig, reg, []))
    for p in TM.parse_descriptor_params(d) + [TM.parse_descriptor_return(d)]:
        w.rec('from_descriptor', {'d': p}, lambda p=p: _jt(JT.from_descriptor(p)))


def _dump_field(w: _Writer, ci, f, reg: dict, tps: list, subclasses: dict):
    c, sig = ci.name, f.generic_signature or ''
    key = {'c': c, 'n': f.name, 'd': f.descriptor}
    w.rec('jvm_to_rust', {'d': f.descriptor}, lambda: TM.jvm_to_rust(f.descriptor, reg))
    w.rec('instance_field_rust_name', key, lambda: ST.instance_field_rust_name(c, safe_ident(f.name), reg))
    w.rec('outer_ref_field_type', key, lambda: TA.outer_ref_field_type(f, tps, reg))
    w.rec('jvm_to_rs_type', {**key, 's': sig}, lambda: render_type(ST.jvm_to_rs_type(f.descriptor, sig, tps, reg)))
    w.rec('from_descriptor', {'d': f.descriptor}, lambda: _jt(JT.from_descriptor(f.descriptor)))
    if not sig:
        return
    w.rec('parse_field_type', key, lambda: SP.parse_field_type(sig, tps, reg))
    w.rec('from_signature', {'s': sig}, lambda: _jt(JT.from_signature(sig, reg)))

    def _bridge():
        t = JT.from_rust_type(SP.parse_field_type(sig, tps, reg), reg, frozenset(tps))
        return {'t': _jt(t), 'head': JT.rust_head_name(t)}
    w.rec('from_rust_type', {**key, 'tps': tps}, _bridge)

    def _subtype():
        a, e = JT.from_signature(sig, reg), JT.from_descriptor(f.descriptor)
        return [a.is_subtype_of(e, reg), JT.strict_erased_subtype(a, e, reg),
                _jt(a.erasure()), a.to_display()]
    w.rec('subtype', {'s': sig, 'd': f.descriptor}, _subtype)
    if sig.startswith('L') and '<' in sig:
        declared = sig[1:sig.index('<')]
        for actual in [declared] + subclasses.get(declared, [])[:3]:
            w.rec('infer_type_args', {'a': actual, 's': sig, 'tps': tps},
                  lambda actual=actual: ST.infer_type_args_from_declared(actual, sig, tps, reg))


def _make_hook(out_path: Path):
    def hook(out_dir, class_infos, jdk_class_infos, java_files, **kw):
        registry: dict = {ci.name: ci for ci in class_infos}
        for classes in (kw.get('lib_crate_classes') or {}).values():
            for ci in classes:
                registry.setdefault(ci.name, ci)
        for ci in jdk_class_infos or []:
            registry.setdefault(ci.name, ci)
        TM.configure_short_names(registry)
        resolver = _RESOLVERS[-1]
        subclasses: dict = {}
        for ci in registry.values():
            for s in [ci.super_class] + list(ci.interfaces or []):
                if s:
                    subclasses.setdefault(s, []).append(ci.name)
        out_path.parent.mkdir(parents=True, exist_ok=True)
        with open(out_path, 'w', encoding='utf-8') as fh:
            meta = {
                'meta': True,
                'java_home': str(resolver._home),
                'user_dir': os.path.join(out_dir, 'closure_input', 'classes'),
                'image_dirs': list(resolver.image_class_dirs()),
                'runtime': str(ROOT / 'runtime' / 'java_runtime'),
                'registry': list(registry),
                'prelude_disambiguated': list(TM._PRELUDE_DISAMBIGUATED),
            }
            fh.write(json.dumps(meta, ensure_ascii=False) + '\n')
            w = _Writer(fh)
            for ci in registry.values():
                _dump_class(w, ci, registry, subclasses)
        print(f"[dump_ty] {len(registry)} 类，{w.count} 条记录 → {out_path}")
        raise _Done()
    return hook


def main(argv: list[str]) -> int:
    if not argv:
        print(__doc__)
        return 2
    apply_jdk(21, quiet=True)
    for java_file in argv:
        stem = Path(java_file).stem
        out_dir = str(ROOT / 'build' / 'golden' / 'ty' / f'{stem}.scratch')
        out_path = ROOT / 'build' / 'golden' / 'ty' / f'{stem}.jsonl'
        _transpile.write_cargo_project = _make_hook(out_path)
        try:
            _transpile.transpile([os.path.abspath(java_file)], out_dir)
        except _Done:
            continue
        print(f"[dump_ty] {java_file}：未到达 write_cargo_project", file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

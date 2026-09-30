#!/usr/bin/env python3
"""发射层输入 golden 采集：跑一遍正常 Python 转译前半程（javac → rava closure → 折叠 → registry），
在 write_cargo_project 入口按与发射层相同的方式构建 registry，写出输入层事实（P1）：

  registry 插入序 / 用户 · lib · JDK 类表、调用链（visited）、field_stubs、反射面、种子、
  模块资源、预检链事实、发射层清单、手写共置扫描、规范化（折叠 + VM 常量剪枝）改动过的方法体、
  逐类逐方法的发射判定。

Rust 侧（generator/crates/input/tests/golden.rs）按 meta 行以同一类路径与 closure.json 重建并逐项比较。

用法：
    python3 scripts/golden/dump_input.py tests/e2e/33_maps/TestHashMapOps.java [...]
输出：build/golden/input/<Test>.jsonl（scratch：build/golden/input/<Test>.scratch，每次清空）

口径：
- 手写扫描对 runtime/java_runtime/src（与 --clean scratch 的 overlay 同一内容）；
- 方法体按投影比较（见 _proj）：指令偏移 + 名字 + 跳转目标 + 常量；
- 发射判定按 class_writer._emit_method_blocks 的判定逻辑重算（复用其辅助函数）。
"""

from __future__ import annotations

import json
import os
import shutil
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
from codegen import callchain as CC  # noqa: E402
from codegen import runtime_manifest as RM  # noqa: E402
from codegen import type_map as TM  # noqa: E402
from codegen import sig_types as ST  # noqa: E402
from codegen.constants import safe_ident  # noqa: E402

_RESOLVERS: list = []
_CHANGED: set = set()
_CUR: list = [None]

_orig_apply = _folds.apply
_orig_prune = _vm_constants.prune_dead_guards


def _apply(method_key, instrs, exception_table, code_len):
    _CUR[0] = method_key
    if method_key in _folds._FOLDS:
        _CHANGED.add(method_key)
    return _orig_apply(method_key, instrs, exception_table, code_len)


def _prune(instrs, exception_table):
    out = _orig_prune(instrs, exception_table)
    if len(out) != len(instrs) and _CUR[0] is not None:
        _CHANGED.add(_CUR[0])
    return out


_folds.apply = _apply
_vm_constants.prune_dead_guards = _prune


class _CapturingResolver(_closure_input.JdkResolver):
    def __init__(self, *a, **kw):
        super().__init__(*a, **kw)
        _RESOLVERS.append(self)


_closure_input.JdkResolver = _CapturingResolver
_transpile = sys.modules['codegen.transpile']
_transpile._write_jdk_scan_report = lambda *a, **kw: None
_FIELD_STUBS: list = [set()]
_orig_discover = _transpile._discover_closure


def _discover(*a, **kw):
    infos, visited, stubs = _orig_discover(*a, **kw)
    _FIELD_STUBS[0] = set(stubs)
    return infos, visited, stubs


_transpile._discover_closure = _discover


class _Done(Exception):
    pass


# ── 方法体投影 ────────────────────────────────────────────────────────────────

_SWITCH = ('tableswitch', 'lookupswitch')


def _const(comment: str) -> str:
    head = comment.split(' ', 1)[0]
    if head in ('String', 'int', 'long', 'class'):
        return comment
    if head in ('float', 'double'):
        return head
    return 'other'


def _proj_one(off, op, operand, comment) -> str:
    s = f'{off} {op}'
    if op in _SWITCH:
        _keep, targets = _folds._switch_parts(operand or '')
        s += ' ->' + ','.join(str(t) for _k, t in targets)
    elif op.startswith('if') or op in ('goto', 'goto_w', 'jsr', 'jsr_w'):
        s += f' ->{operand}'
    elif op in ('ldc', 'ldc_w', 'ldc2_w'):
        s += ' =' + _const(comment or '')
    return s


def _proj(ins) -> str:
    if ins.opcode == 'fold_const':
        _n, inner = _folds.decode_fold_const(ins)
        return f'{ins.offset} fold_field ' + _proj_one(inner.offset, inner.opcode, inner.operand, inner.comment).split(' ', 1)[1]
    if ins.fold is not None:
        load = ins.fold
        return (_proj_one(ins.offset, ins.opcode, ins.operand, ins.comment) + ' fold_call '
                + _proj_one(load.offset, load.opcode, load.operand, load.comment).split(' ', 1)[1])
    return _proj_one(ins.offset, ins.opcode, ins.operand, ins.comment)


# ── 发射判定（class_writer._emit_method_blocks 判定部分的重算）───────────────

def _ref_mname(c: str) -> str:
    rest = c.split(' ', 1)[1] if ' ' in c else c
    dot = rest.find('.')
    colon = rest.find(':', dot)
    return rest[dot + 1:colon] if 0 < dot < colon else ''


def _iface_default_body(ci, m, registry, call_chain) -> bool:
    from codegen.instr.member_owner import _resolve_interface_special_target as rist
    from codegen.instr.member_naming import _method_ref_binary_class as mrbc, _method_ref_descriptor as mrd
    for ins in (m.instrs or []):
        if ins.opcode == 'invokespecial' and ins.comment \
                and rist(mrbc(ins.comment), _ref_mname(ins.comment), mrd(ins.comment), registry):
            return False
    own = {(x.name, x.descriptor) for x in ci.methods}
    for ins in (m.instrs or []):
        if ins.opcode in ('invokevirtual', 'invokeinterface') and ins.comment \
                and (_ref_mname(ins.comment), mrd(ins.comment)) not in own:
            return False
    return (not m.is_abstract and not m.is_synthetic
            and (call_chain is None or (ci.name, m.name, m.descriptor) in call_chain))


def _plan(ci, registry, call_chain, nf_map) -> dict:
    from codegen.emitter.class_writer import _ancestor_iface_member_names
    from codegen.instr.member_owner import _root_virtual_methods
    nf = nf_map.get(ci.name)
    is_iface = ci.is_interface
    type_only = call_chain is not None and not any(
        (ci.name, x.name, x.descriptor) in call_chain for x in ci.methods)
    visible = [x for x in ci.methods if not x.is_synthetic]
    emitted = visible + [x for x in ci.methods if x.is_synthetic and not (x.access_flags & 0x40)
                         and x.name not in ('<init>', '<clinit>')]
    root = _root_virtual_methods()
    overloaded = ST.hierarchy_overloaded_names(ci, registry)
    used: dict = {}
    out = []

    def in_cc(m):
        return call_chain is None or (ci.name, m.name, m.descriptor) in call_chain

    def dedupe(n):
        if n in used:
            used[n] += 1
            return f'{n}_{used[n]}'
        used[n] = 0
        return n

    def rec(m, rust, role, verdict):
        out.append({'n': m.name, 'd': m.descriptor, 'rust': rust, 'role': role, 'v': verdict})

    for m in emitted:
        if m.name == '<clinit>':
            if type_only or CC._is_vm_boundary_class(ci.name):
                continue
            if call_chain is not None and (ci.name, m.name, m.descriptor) not in call_chain:
                rec(m, '__clinit', 'clinit', 'stub_not_in_chain')
            else:
                rec(m, '__clinit', 'clinit', 'bytecode')
            continue
        iface_inst = is_iface and not m.is_static
        if iface_inst and m.is_synthetic and m.name.startswith('lambda$'):
            from codegen.instr.member_naming import lambda_impl_rust_name
            rust = lambda_impl_rust_name(ci.name, m.name, m.descriptor, registry)
            rec(m, rust, 'iface_lambda', 'bytecode' if in_cc(m) else 'stub_not_in_chain')
            continue
        root_keyed = (m.name, m.descriptor[:m.descriptor.index(')') + 1]) in root
        private = bool(m.access_flags & 0x0002)
        if iface_inst and not m.is_synthetic and private and not root_keyed:
            rust = TM.mangle_name(m.name, m.descriptor) if ST.method_name_is_mangled(ci, m, registry) else m.name
            rust = dedupe(rust)
            rec(m, rust, 'iface_private', 'bytecode' if in_cc(m) else 'stub_not_in_chain')
            continue
        if iface_inst and (m.is_synthetic or private or root_keyed):
            continue
        rust = TM.mangle_name(m.name, m.descriptor) if ST.method_name_is_mangled(ci, m, registry) else m.name
        if m.is_constructor:
            rust = TM.mangle_name('new', m.descriptor) if '<init>' in overloaded else 'new'
        rust = dedupe(rust)
        fn = safe_ident(rust or m.name)
        covered = (nf or {}).get('methods', set())
        virtual = not (m.is_constructor or m.is_static or m.is_native)
        if fn in covered:
            v = 'handwritten'
        elif iface_inst:
            v = 'iface_default_body' if _iface_default_body(ci, m, registry, call_chain) else 'iface_decl'
        elif virtual and ('__impl_' + fn) in covered:
            v = 'handwritten_body'
        elif ((nf or {}).get('method_cores', {}) or {}).get(fn) is not None and not m.is_static:
            v = 'core'
        elif m.is_native:
            v = 'stub_native'
        elif m.is_abstract:
            v = 'stub_abstract'
        elif not in_cc(m):
            v = 'stub_not_in_chain'
        else:
            v = 'bytecode'
        rec(m, rust, 'member', v)

    supp = []
    if is_iface:
        sigs = (nf or {}).get('iface_method_sigs', {})
        if sigs:
            skip = set(used) | _ancestor_iface_member_names(ci, registry) | {n for n, _ in root}
            supp = [n for n in sorted(sigs) if n not in skip]
    return {'c': ci.name, 'type_only': bool(type_only), 'methods': out, 'supp': supp}


# ── 采集 ─────────────────────────────────────────────────────────────────────

def _fnv(b: bytes) -> str:
    h = 0xcbf29ce484222325
    for x in b:
        h = ((h ^ x) * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return f'{h:016x}'


def _key(k) -> str:
    return f'{k[0]}.{k[1]}:{k[2]}'


def _manifest() -> dict:
    return {
        'vm_boundary_classes': sorted(RM.vm_boundary_classes()),
        'release': RM.vm_boundary_translate_nested(),
        'module_resource_paths': RM.module_resource_paths(),
        'boot_init_classes': RM.boot_init_classes(),
        'boot_init_calls': [f'{c}.{m}:()V' for c, m in RM.boot_init_calls()],
        'intrinsic_members': sorted(RM.intrinsic_members()),
        'caller_sensitive_annotations': sorted(RM.caller_sensitive_annotations()),
        'sigpoly_callsite_typed': sorted(RM.sigpoly_callsite_typed()),
        'indy_kinds': dict(sorted(RM._indy_kinds().items())),
        'null_returns': sorted(RM.vm_constant_null_returns()),
        'null_to_false': sorted(RM.vm_constant_null_to_false()),
    }


def _scan_handwritten(out_dir: str, registry: dict) -> dict:
    """手写真源扫描：runtime/java_runtime/src 以符号链接挂到独立根下（与 --clean overlay 同内容）"""
    from codegen.emitter.method_gen import _scan_impl_files
    root = os.path.join(out_dir, 'hw_scan')
    shutil.rmtree(root, ignore_errors=True)
    os.makedirs(os.path.join(root, 'java_runtime'))
    os.symlink(str(ROOT / 'runtime' / 'java_runtime' / 'src'), os.path.join(root, 'java_runtime', 'src'))
    nf_map, _ = _scan_impl_files(root, registry=registry)
    return nf_map


def _make_hook(out_path: Path):
    def hook(out_dir, class_infos, jdk_class_infos, java_files, **kw):
        lib_crates = kw.get('lib_crate_classes') or {}
        call_chain = kw.get('visited_methods')
        registry: dict = {ci.name: ci for ci in class_infos}
        for classes in lib_crates.values():
            for ci in classes:
                registry.setdefault(ci.name, ci)
        for ci in jdk_class_infos or []:
            registry.setdefault(ci.name, ci)
        TM.configure_short_names(registry)
        nf_map = _scan_handwritten(out_dir, registry)
        resolver = _RESOLVERS[-1]
        user = [ci.name for ci in class_infos]
        recs: list = []

        def rec(f, **v):
            recs.append({'f': f, **v})

        rec('registry_order', v=list(registry))
        rec('user_classes', v=user)
        rec('lib_crates', v=[[k, [c.name for c in v]] for k, v in lib_crates.items()])
        rec('jdk_classes', v=[ci.name for ci in jdk_class_infos or []])
        rec('visited', v=sorted(_key(k) for k in call_chain))
        rec('field_stubs', v=sorted(_FIELD_STUBS[0]))
        rec('reflect_consts', v={k: sorted(v) for k, v in sorted(CC.REFLECT_CONSTS.items())})
        rec('reflect_all', v=sorted(CC.REFLECT_ALL_MEMBERS))
        rec('reflect_field_names', v=sorted(CC.REFLECT_FIELD_NAMES))
        rec('annotation_enum_seeds', v=list(CC.ANNOTATION_ENUM_SEEDS))
        rec('module_resources', v=[[p, len(b), _fnv(b)] for p, b in CC.MODULE_RESOURCES.items()])
        rec('precheck_visited', v=sorted(CC.PRECHECK_CHAIN['visited']))
        rec('manifest', v=_manifest())
        for cls in sorted(nf_map):
            e = nf_map[cls]
            rec('handwritten', c=cls, v={
                'methods': sorted(e.get('methods', ())),
                'cores': {k: list(v) for k, v in sorted((e.get('method_cores') or {}).items())},
                'sigs': {k: list(v) for k, v in sorted((e.get('iface_method_sigs') or {}).items())},
            })
        changed = []
        for ci in registry.values():
            for m in ci.methods:
                k = f'{ci.name}.{m.name}:{m.descriptor}'
                if k not in _CHANGED or not m.instrs:
                    continue
                changed.append(k)
                rec('code', c=ci.name, n=m.name, d=m.descriptor, v={
                    'insns': [_proj(x) for x in m.instrs],
                    'et': [[s, e, h, t or ''] for s, e, h, t in (m.exception_table or [])],
                })
        rec('normalized_keys', v=sorted(changed))
        user_set = set(user)
        for ci in registry.values():
            cc = None if ci.name in user_set else call_chain
            rec('plan', c=ci.name, v=_plan(ci, registry, cc, nf_map))

        out_path.parent.mkdir(parents=True, exist_ok=True)
        with open(out_path, 'w', encoding='utf-8') as fh:
            meta = {
                'meta': True,
                'java_home': str(resolver._home),
                'user_dir': os.path.join(out_dir, 'closure_input', 'classes'),
                'image_dirs': list(resolver.image_class_dirs()),
                'runtime': str(ROOT / 'runtime' / 'java_runtime'),
                'closure_json': os.path.join(out_dir, 'closure_input', 'closure.json'),
                'user_classes': user,
                'libs': [],
            }
            fh.write(json.dumps(meta, ensure_ascii=False) + '\n')
            for r in recs:
                fh.write(json.dumps(r, ensure_ascii=False) + '\n')
        print(f"[dump_input] {len(registry)} 类，{len(recs)} 条记录 → {out_path}")
        raise _Done()
    return hook


def main(argv: list[str]) -> int:
    if not argv:
        print(__doc__)
        return 2
    apply_jdk(21, quiet=True)
    for java_file in argv:
        stem = Path(java_file).stem
        out_dir = ROOT / 'build' / 'golden' / 'input' / f'{stem}.scratch'
        shutil.rmtree(out_dir, ignore_errors=True)
        out_path = ROOT / 'build' / 'golden' / 'input' / f'{stem}.jsonl'
        _CHANGED.clear()
        _transpile.write_cargo_project = _make_hook(out_path)
        try:
            _transpile.transpile([os.path.abspath(java_file)], str(out_dir))
        except _Done:
            continue
        print(f"[dump_input] {java_file}：未到达 write_cargo_project", file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

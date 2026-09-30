#!/usr/bin/env python3
"""
P4a golden 转储：在一次正常转译中截获 `codegen/stack.py`（StackSim）公开入口的前后状态。

    python3 scripts/golden/dump_sim.py tests/e2e/33_maps/TestHashMapOps.java --jdk 21 [main.py 其余参数]

截获的入口（kind）：`init` / `pop` / `pop_for_store` / `store_local` / `load_local` / `fresh_let`。
只记录最外层调用（store_local 内部的 fresh_let 等不重复记）。每条记录：
- `cfg`：模拟器配置（形参槽、局部变量名、LVT 声明区间、类型形参等）
- `pre` / `post`：调用前后状态（栈带 dup 身份分组 id、局部变量表、计数器、深度账等）
- `args`：调用实参；`ret`：返回值（表达式 / 类型的 JSON 与渲染文本）
- `stmts`：本次调用新追加的语句（JSON + 渲染文本 + VarOrigin 元数据）
- `hooks`：调用期间外部钩子（is_subtype / is_interface / box_object / infer_type_args /
  carrier_type_for_ident）的实参与返回值，供 Rust 侧按实参回放

输出 `build/golden/sim/<Test>.jsonl`（同一记录去重计数）与 `<Test>.names.json`
（registry 全部 binary → 短名、接口短名集合）。只读截获，不修改 codegen 语义。
"""

import dataclasses
import json
import os
import runpy
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, REPO)

from codegen import stack as ST  # noqa: E402  须先于 main.py 导入
from codegen import jvm_type as JT  # noqa: E402
from codegen.render import render_expr, render_stmt, render_type  # noqa: E402
from codegen.type_map import short_cls  # noqa: E402
from codegen.method import codegen as MC  # noqa: E402,F401  触发 codegen 包加载

_records: dict[str, dict] = {}
_depth = 0
_hooks: list | None = None
_registry = {'reg': None}
_unknown_types: dict[str, int] = {}


def to_json(v):
    if dataclasses.is_dataclass(v) and not isinstance(v, type):
        d = {'_t': type(v).__name__}
        for f in dataclasses.fields(v):
            d[f.name] = to_json(getattr(v, f.name))
        return d
    if isinstance(v, (list, tuple)):
        return [to_json(x) for x in v]
    if isinstance(v, (frozenset, set)):
        return sorted(to_json(x) for x in v)
    if isinstance(v, dict):
        return {str(k): to_json(x) for k, x in v.items()}
    if v is None or isinstance(v, (bool, int, float, str)):
        return v
    return {'_t': '__unknown__', 'repr': repr(v)}


def ty_json(t):
    if t is None:
        return None
    tn = type(t).__name__
    _unknown_types[tn] = _unknown_types.get(tn, 0) + 1
    return {'j': to_json(t), 'text': render_type(t)}


def expr_json(e):
    return {'j': to_json(e), 'text': render_expr(e)}


def _ids(sim, extra=()):
    """栈上对象身份 → 分组号（首次出现序）；extra 为额外参与分组的对象（store 的源值）。"""
    ids: dict[int, int] = {}
    for obj in [se for se, _ in sim.stack] + list(extra):
        ids.setdefault(id(obj), len(ids))
    return ids


def state_json(sim, ids):
    return {
        'stack': [[expr_json(se), ty_json(sty), ids.setdefault(id(se), len(ids))]
                  for se, sty in sim.stack],
        'locals': {str(k): [n, ty_json(t), bool(new)] for k, (n, t, new) in sorted(sim.locals.items())},
        'ctr': sim._ctr,
        'synth': {str(k): list(v) for k, v in sorted(sim._synth_slot_types.items())},
        'cur': sim.current_offset, 'next': sim.next_offset, 'depth': sim._current_depth,
        'decl_depth': {str(k): v for k, v in sorted(sim._slot_decl_depth.items())},
        'bind_pos': {str(k): v for k, v in sorted(sim._slot_bind_pos.items())},
        'underflow': bool(sim.underflow_occurred),
        'bounds': {k: v for k, v in sorted(sim.type_var_bounds.items())},
        'n_stmts': len(sim.stmts),
    }


def cfg_json(sim):
    return {
        'is_static': sim.is_static, 'class_name': sim.class_name,
        'class_type_params': sorted(sim.class_type_params),
        'loc_names': {str(k): v for k, v in sorted(sim._loc_names.items())},
        'slot_decls': {str(k): [[e[0], e[1], e[2], ty_json(e[3]), bool(e[4]),
                                 e[5] if len(e) > 5 else '', e[6] if len(e) > 6 else '']
                                for e in v] for k, v in sorted(sim._slot_decls.items())},
        'param_slots': sorted(sim._param_slots),
        'return_type': sim.return_type, 'is_constructor': sim.is_constructor,
        'in_vtable_body': sim.in_vtable_body,
    }


def stmt_json(s):
    return {'j': to_json(s), 'text': render_stmt(s, 0),
            'slot': getattr(s, 'slot', None), 'bind_off': getattr(s, 'bind_off', None),
            'value_ty': ty_json(getattr(s, 'value_ty', None))}


def _record(kind, sim, pre, cfg, args, ret, ids):
    stmts = sim.stmts[pre['n_stmts']:] if len(sim.stmts) >= pre['n_stmts'] else []
    rec = {'kind': kind, 'cfg': cfg, 'pre': pre, 'args': args, 'hooks': _hooks,
           'ret': ret, 'stmts': [stmt_json(s) for s in stmts], 'post': state_json(sim, ids)}
    key = json.dumps(rec, sort_keys=True, ensure_ascii=False)
    old = _records.get(key)
    if old is None:
        rec['count'] = 1
        _records[key] = rec
    else:
        old['count'] += 1


def _hook(name, fn):
    def wrapper(*a):
        out = fn(*a)
        if _hooks is not None:
            _hooks.append([name, [x if isinstance(x, (str, int, type(None))) else repr(x) for x in a], out])
        return out
    return wrapper


# ── 截获 ─────────────────────────────────────────────────────────────────────

_orig_init = ST.StackSim.__init__


def init_wrapper(self, param_rust_types, is_static, class_name, local_names=None, slot_decls=None,
                 infer_type_args=None, is_subtype=None, is_interface=None, return_type='Object',
                 is_constructor=False, class_type_params=None, in_vtable_body=False,
                 box_object=None, registry=None):
    _orig_init(self, param_rust_types, is_static, class_name, local_names, slot_decls,
               infer_type_args, is_subtype, is_interface, return_type, is_constructor,
               class_type_params, in_vtable_body, box_object, registry)
    if registry:
        _registry['reg'] = registry
    self._is_subtype = _hook('is_subtype', self._is_subtype)
    self._is_interface = _hook('is_interface', self._is_interface)
    self._box_object = _hook('box_object', self._box_object)
    if self._infer_type_args is not None:
        self._infer_type_args = _hook('infer_type_args', self._infer_type_args)
    args = {'params': [ty_json(t) for t in param_rust_types],
            'class_type_params': list(class_type_params or [])}
    ids = _ids(self)
    _record('init', self, {'n_stmts': 0}, cfg_json(self), args, None, ids)


def _wrap_method(kind, orig, args_fn, ret_fn):
    def wrapper(self, *a):
        global _depth, _hooks
        if _depth > 0:
            return orig(self, *a)
        extra = [a[1]] if kind == 'store_local' else []
        ids = _ids(self, extra)
        pre = state_json(self, ids)
        cfg = cfg_json(self)
        args = args_fn(a, ids)
        _depth += 1
        _hooks = []
        try:
            out = orig(self, *a)
            ret = ret_fn(out, ids)
            _record(kind, self, pre, cfg, args, ret, ids)
        finally:
            _depth -= 1
            _hooks = None
        return out
    return wrapper


def _pair(out, ids):
    return [expr_json(out[0]), ty_json(out[1]), ids.get(id(out[0]))]


def install():
    ST.StackSim.__init__ = init_wrapper
    S = ST.StackSim
    S.pop = _wrap_method('pop', S.pop, lambda a, ids: {}, _pair)
    S.pop_for_store = _wrap_method('pop_for_store', S.pop_for_store, lambda a, ids: {}, _pair)
    S.store_local = _wrap_method(
        'store_local', S.store_local,
        lambda a, ids: {'slot': a[0], 'expr': expr_json(a[1]), 'ty': ty_json(a[2]), 'id': ids[id(a[1])]},
        lambda out, ids: None)
    S.load_local = _wrap_method('load_local', S.load_local, lambda a, ids: {'slot': a[0]}, _pair)
    S.fresh_let = _wrap_method(
        'fresh_let', S.fresh_let,
        lambda a, ids: {'prefix': a[0], 'value': expr_json(a[1]), 'ty': ty_json(a[2])},
        lambda out, ids: expr_json(out))
    orig_carrier = JT.carrier_type_for_ident
    JT.carrier_type_for_ident = lambda t, reg: _hook('carrier', lambda x: orig_carrier(x, reg))(t)


def main():
    argv = sys.argv[1:]
    java = [a for a in argv if a.endswith('.java')]
    if not java:
        sys.exit('用法：dump_sim.py <Test.java> [main.py 参数]')
    test = os.path.splitext(os.path.basename(java[0]))[0]
    if '--no-run' not in argv:
        argv.append('--no-run')
    install()
    sys.argv = [os.path.join(REPO, 'scripts', 'main.py')] + argv
    try:
        runpy.run_path(sys.argv[0], run_name='__main__')
    except SystemExit as e:
        if e.code not in (None, 0):
            raise
    out_dir = os.path.join(REPO, 'build', 'golden', 'sim')
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, f'{test}.jsonl'), 'w', encoding='utf-8') as f:
        for rec in _records.values():
            f.write(json.dumps(rec, ensure_ascii=False) + '\n')
    reg = _registry['reg'] or {}
    names = {'short': {b: short_cls(b) for b in sorted(reg)},
             'iface': sorted(short_cls(b) for b, ci in reg.items()
                             if getattr(ci, 'is_interface', False))}
    with open(os.path.join(out_dir, f'{test}.names.json'), 'w', encoding='utf-8') as f:
        json.dump(names, f, ensure_ascii=False, indent=1)
    kinds: dict[str, int] = {}
    for rec in _records.values():
        kinds[rec['kind']] = kinds.get(rec['kind'], 0) + 1
    print(f'[golden-sim] {test}: {len(_records)} 条 {kinds}，类型节点 {_unknown_types} → {out_dir}')


if __name__ == '__main__':
    main()

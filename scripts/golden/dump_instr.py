#!/usr/bin/env python3
"""
P4b golden 转储：在一次正常转译中截获指令翻译层入口 `codegen.instr.sim_instr` 的前后状态。

    python3 scripts/golden/dump_instr.py tests/e2e/33_maps/TestHashMapOps.java [main.py 其余参数]

只记录最外层调用（fold 内部的递归调用不重复记）。每条记录：
- `cls` / `m` / `d`：所在类 binary 名、方法名、描述符（Rust 侧按偏移取同一条已解码指令）
- `ins`：Python 指令视图（offset / opcode / operand / comment / fold）
- `cfg` / `pre` / `post`：模拟器配置与调用前后状态（与 dump_sim.py 同形）
- `lets`：调用前语句表中被栈上变量引用的 `let` 语句（[下标, 语句]；指令翻译会回看 / 改写
  这些语句），`lets_post`：同一下标调用后的语句（检测回写）
- `stmts`：本次调用新追加的语句
- `fx`：调用期间的外部副作用 / 外部查询（equiv_audit / instanceof 折叠计数 / 继承成员登记 /
  lambda 名账 / SAM 合成对象站点）及其返回值
- `err`：Python 侧抛出的异常（repr）

首行为 meta：JDK / 用户类目录 / 镜像类目录 / runtime 目录 / registry 顺序（Rust 侧重建真实
registry 与 TyCtx）。输出 `build/golden/instr/<Test>.jsonl`（同一记录去重计数）。只读截获。
"""

import json
import os
import runpy
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, REPO)
sys.path.insert(0, os.path.join(REPO, 'scripts', 'golden'))

import dump_sim as DS  # noqa: E402  复用状态 / 表达式 JSON 编码
from codegen import equiv_audit as EA  # noqa: E402
from codegen import inherited_calls as IC  # noqa: E402
from codegen.cfg import audit as CA  # noqa: E402
from codegen.instr import member_naming as MN  # noqa: E402
from codegen.emitter import sam_objects as SO  # noqa: E402
from codegen.method import blocks as MB  # noqa: E402
from codegen import rs_ir as IR  # noqa: E402
import codegen.closure_input as CI  # noqa: E402

_records: dict[str, dict] = {}
_meta: dict = {}
_depth = 0
_fx: list | None = None
_RESOLVERS: list = []


class _CapturingResolver(CI.JdkResolver):
    def __init__(self, *a, **kw):
        super().__init__(*a, **kw)
        _RESOLVERS.append(self)


CI.JdkResolver = _CapturingResolver


def _ins_json(ins):
    if ins is None:
        return None
    return {'off': ins.offset, 'op': ins.opcode, 'operand': ins.operand,
            'comment': ins.comment, 'fold': _ins_json(ins.fold)}


def _stack_var_names(sim):
    out = set()
    for se, _ in sim.stack:
        if isinstance(se, IR.Var):
            out.add(se.name)
    return out


def _let_name(s):
    if isinstance(s, IR.LetStmt):
        return s.name
    return None


def _lets(sim):
    names = _stack_var_names(sim)
    if not names:
        return []
    seen = set()
    out = []
    for i in range(len(sim.stmts) - 1, -1, -1):
        n = _let_name(sim.stmts[i])
        if n in names and n not in seen:
            seen.add(n)
            out.append(i)
    return sorted(out)


def _fx_hook(name, fn, keep_out=False):
    def wrapper(*a, **kw):
        out = fn(*a, **kw)
        if _fx is not None:
            args = [x if isinstance(x, (str, int, type(None))) else repr(x) for x in a]
            _fx.append([name, args, out if keep_out else None])
        return out
    return wrapper


def _method_of_caller():
    f = sys._getframe(2)
    while f is not None:
        slf = f.f_locals.get('self')
        m = getattr(slf, 'method', None)
        if m is not None and hasattr(m, 'descriptor'):
            return m.name, m.descriptor
        f = f.f_back
    return '', ''


_orig_sim_instr = MB.sim_instr


def sim_instr_wrapper(ins, sim, class_name, registry=None):
    global _depth, _fx
    if _depth > 0:
        return _orig_sim_instr(ins, sim, class_name, registry)
    if registry and 'registry' not in _meta:
        _meta['registry'] = list(registry)
    mname, mdesc = _method_of_caller()
    ids = DS._ids(sim)
    pre = DS.state_json(sim, ids)
    cfg = DS.cfg_json(sim)
    lets_idx = _lets(sim)
    lets = [[i, DS.stmt_json(sim.stmts[i])] for i in lets_idx]
    n_pre = len(sim.stmts)
    _depth += 1
    _fx = []
    err = None
    try:
        _orig_sim_instr(ins, sim, class_name, registry)
    except Exception as e:  # noqa: BLE001  记录后原样抛出
        err = repr(e)
        raise
    finally:
        rec = {'cls': class_name, 'm': mname, 'd': mdesc, 'ins': _ins_json(ins),
               'cfg': cfg, 'pre': pre, 'lets': lets,
               'lets_post': [[i, DS.stmt_json(sim.stmts[i])] for i in lets_idx if i < len(sim.stmts)],
               'stmts': [DS.stmt_json(s) for s in sim.stmts[n_pre:]],
               'post': DS.state_json(sim, ids), 'fx': _fx, 'err': err}
        _depth -= 1
        _fx = None
        key = json.dumps(rec, sort_keys=True, ensure_ascii=False)
        old = _records.get(key)
        if old is None:
            rec['count'] = 1
            _records[key] = rec
        else:
            old['count'] += 1


def install():
    MB.sim_instr = sim_instr_wrapper
    EA.record = _fx_hook('audit', EA.record)
    IC.request = _fx_hook('inherited', IC.request)
    _stats = CA.STATS
    _stats.record_instanceof_fold = _fx_hook('instanceof_fold', _stats.record_instanceof_fold)
    _led = MN.LAMBDA_NAME_LEDGER
    _led.record_reference = _fx_hook('lambda_ref', _led.record_reference)
    SO.site_ctor_path = _fx_hook('sam_ctor', SO.site_ctor_path, keep_out=True)
    SO.record_site = _fx_hook('sam_site', SO.record_site)


def main():
    argv = sys.argv[1:]
    java = [a for a in argv if a.endswith('.java')]
    if not java:
        sys.exit('用法：dump_instr.py <Test.java> [main.py 参数]')
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
    res = _RESOLVERS[-1] if _RESOLVERS else None
    meta = {
        'meta': True,
        'java_home': str(res._home) if res is not None else '',
        'user_dir': os.path.join(REPO, 'build', test, 'closure_input', 'classes'),
        'image_dirs': list(res.image_class_dirs()) if res is not None else [],
        'runtime': os.path.join(REPO, 'runtime', 'java_runtime'),
        'registry': _meta.get('registry', []),
    }
    out_dir = os.path.join(REPO, 'build', 'golden', 'instr')
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, f'{test}.jsonl'), 'w', encoding='utf-8') as f:
        f.write(json.dumps(meta, ensure_ascii=False) + '\n')
        for rec in _records.values():
            f.write(json.dumps(rec, ensure_ascii=False) + '\n')
    ops: dict[str, int] = {}
    for rec in _records.values():
        ops[rec['ins']['op']] = ops.get(rec['ins']['op'], 0) + 1
    print(f'[golden-instr] {test}: {len(_records)} 条，{len(ops)} 种指令 → {out_dir}')


if __name__ == '__main__':
    main()

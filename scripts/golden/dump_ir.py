#!/usr/bin/env python3
"""
P3 golden 转储：在一次正常转译中截获 `codegen/render.py` 公开入口的 IR 输入与渲染文本。

    python3 scripts/golden/dump_ir.py tests/e2e/33_maps/TestHashMapOps.java [main.py 其余参数]

- 替换 render 入口后，把已导入的 codegen 模块里以 `from ..render import render_expr` 绑定的
  同名引用一并改指包装函数（导入 codegen 包本身即已加载大部分模块）；
  只记录最外层调用（render 内部递归不重复记）。
- 输出 `build/golden/ir/<Test>.jsonl`：每行一条 {"kind", "input", "args", "out", "count"}，
  input 为 dataclass → JSON（`_t` 为节点类型标签）；同一 (kind, input, args, out) 去重计数。
- `build/golden/ir/<Test>.short_names.json`：render 实际查询过的 short_cls 输入 → 输出。
- `build/golden/ir/<Test>.stats.json`：Raw 逃生舱规模、渲染文本量与生成物总量等统计。
- 只读截获，不修改 codegen 语义；随 Python 生成器一并退役。
"""

import dataclasses
import json
import os
import runpy
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, REPO)

from codegen import render as R  # noqa: E402  须先于 main.py 导入

_depth = 0
_records: dict[tuple, dict] = {}
_short: dict[str, str] = {}
_stats = {'calls': {}, 'rendered_chars': 0}


def to_json(v):
    """IR 节点 → JSON（`_t` 为类型标签；str 子类如 StructLine 按原样文本输出并保留元数据）。"""
    if dataclasses.is_dataclass(v) and not isinstance(v, type):
        d = {'_t': type(v).__name__}
        for f in dataclasses.fields(v):
            d[f.name] = to_json(getattr(v, f.name))
        return d
    if isinstance(v, (list, tuple)):
        return [to_json(x) for x in v]
    if isinstance(v, str):
        if type(v) is not str:  # StructLine 等 str 子类
            return {'_t': type(v).__name__, 'text': str(v),
                    'delta': getattr(v, 'delta', 0), 'tag': getattr(v, 'tag', '')}
        return v
    if v is None or isinstance(v, (bool, int, float)):
        return v
    return {'_t': '__unknown__', 'repr': repr(v)}


def _wrap(name: str, arg_names: tuple):
    orig = getattr(R, name)

    def wrapper(node, *args, **kwargs):
        global _depth
        _depth += 1
        try:
            out = orig(node, *args, **kwargs)
        finally:
            _depth -= 1
        if _depth == 0:
            extra = {}
            for i, a in enumerate(args):
                extra[arg_names[i]] = a
            extra.update(kwargs)
            try:
                inp = json.dumps(to_json(node), ensure_ascii=False, sort_keys=True)
            except Exception as e:  # pragma: no cover
                inp = json.dumps({'_t': '__error__', 'repr': repr(e)})
            key = (name, inp, json.dumps(extra, sort_keys=True), out)
            rec = _records.get(key)
            if rec is None:
                _records[key] = {'kind': name, 'input': inp, 'args': extra, 'out': out, 'count': 1}
            else:
                rec['count'] += 1
            _stats['calls'][name] = _stats['calls'].get(name, 0) + 1
            if isinstance(out, str):
                _stats['rendered_chars'] += len(out)
        return out

    wrapper.__name__ = name
    setattr(R, name, wrapper)
    _rebind(orig, wrapper)


def _rebind(orig, wrapper):
    """已导入模块中按名字绑定的 orig 引用改指 wrapper（含别名导入）。"""
    for mod_name, mod in list(sys.modules.items()):
        if mod is None or mod is R or not mod_name.startswith('codegen'):
            continue
        for attr, val in list(vars(mod).items()):
            if val is orig:
                setattr(mod, attr, wrapper)


_orig_short = R._short_cls_g


def _short_wrapper(binary, *a, **kw):
    out = _orig_short(binary, *a, **kw)
    _short[binary] = out
    return out


R._short_cls_g = _short_wrapper

for _n, _a in (('render_type', ()), ('render_expr', ()), ('render_stmt', ('indent',)),
               ('render_cast', ()), ('render_fn', ('indent',)), ('render_item', ('indent',)),
               ('render_struct', ('indent',)), ('render_impl', ('indent',)),
               ('render_file', ('preamble',)), ('upcast_expr', ('wrap',)),
               ('is_atomic_rs', ())):
    _wrap(_n, _a)


def _raw_stats(node, acc):
    """递归统计 IR 内 Raw 逃生舱（RawExpr / RawStmt / RsRawItem / str 类型）字符量。"""
    if isinstance(node, dict):
        t = node.get('_t')
        if t in ('RawExpr', 'RawStmt', 'RsRawItem'):
            acc[t] = acc.get(t, 0) + 1
            acc[t + '_chars'] = acc.get(t + '_chars', 0) + len(node.get('code', ''))
        for v in node.values():
            _raw_stats(v, acc)
    elif isinstance(node, list):
        for v in node:
            _raw_stats(v, acc)


def main():
    argv = sys.argv[1:]
    java = [a for a in argv if a.endswith('.java')]
    if not java:
        sys.exit('用法：dump_ir.py <Test.java> [main.py 参数]')
    test = os.path.splitext(os.path.basename(java[0]))[0]
    if '--no-run' not in argv:
        argv.append('--no-run')
    sys.argv = [os.path.join(REPO, 'scripts', 'main.py')] + argv
    try:
        runpy.run_path(sys.argv[0], run_name='__main__')
    except SystemExit as e:
        if e.code not in (None, 0):
            raise
    out_dir = os.path.join(REPO, 'build', 'golden', 'ir')
    os.makedirs(out_dir, exist_ok=True)
    raw = {}
    with open(os.path.join(out_dir, f'{test}.jsonl'), 'w', encoding='utf-8') as f:
        for rec in _records.values():
            _raw_stats(json.loads(rec['input']), raw)
            f.write(json.dumps({'kind': rec['kind'], 'input': json.loads(rec['input']),
                                'args': rec['args'], 'out': rec['out'], 'count': rec['count']},
                               ensure_ascii=False) + '\n')
    with open(os.path.join(out_dir, f'{test}.short_names.json'), 'w', encoding='utf-8') as f:
        json.dump(dict(sorted(_short.items())), f, ensure_ascii=False, indent=1)
    _stats['unique_records'] = len(_records)
    _stats['raw_in_unique_inputs'] = raw
    with open(os.path.join(out_dir, f'{test}.stats.json'), 'w', encoding='utf-8') as f:
        json.dump(_stats, f, ensure_ascii=False, indent=1)
    print(f'[golden-ir] {test}: {len(_records)} 条（调用 {sum(_stats["calls"].values())} 次），'
          f'短名 {len(_short)} 个 → {out_dir}')


if __name__ == '__main__':
    main()

#!/usr/bin/env python3
"""
P4a golden 转储：在一次正常转译中截获 `codegen/cfg/` 公开入口的输入与输出。

    python3 scripts/golden/dump_cfg.py tests/e2e/33_maps/TestHashMapOps.java --jdk 21 [main.py 其余参数]

截获的入口（kind）：
- `blocks`：build_blocks(instrs, exception_table, boundaries) → 基本块
- `analyze`：analyze(entry, succs) → RPO / 支配树 / 回边 / 自然循环 / 可归约性
- `structure`：build_structure(nodes, flow) → 结构树（simplify 前）、S-67 改写后的节点 ctx、
  simplify 后的结构树（在深拷贝上调用 simplify，不影响正常流水线）
- `cmp_op` / `neg_cmp_op`：JVM 比较跳转 → 条件文本

输出 `build/golden/cfg/<Test>.jsonl`：每行 {"kind", "input", "out", "count"}，同一 (kind, input, out)
去重计数。只读截获，不修改 codegen 语义；随 Python 生成器一并退役。
"""

import copy
import json
import os
import runpy
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, REPO)

from codegen.cfg import graph as G  # noqa: E402  须先于 main.py 导入
from codegen.cfg import conditions as C  # noqa: E402
import importlib  # noqa: E402
S = importlib.import_module('codegen.cfg.structure')  # 包级同名函数遮蔽子模块
SP = importlib.import_module('codegen.cfg.simplify')
from codegen.method import codegen as MC  # noqa: E402,F401  触发 codegen 包加载

_records: dict[tuple, dict] = {}


def _record(kind: str, inp, out) -> None:
    key = (kind, json.dumps(inp, sort_keys=True, ensure_ascii=False),
           json.dumps(out, sort_keys=True, ensure_ascii=False))
    rec = _records.get(key)
    if rec is None:
        _records[key] = {'kind': kind, 'input': inp, 'out': out, 'count': 1}
    else:
        rec['count'] += 1


def _rebind(orig, wrapper) -> None:
    """已导入 codegen 模块中按名字绑定的 orig 引用改指 wrapper（含别名导入）。"""
    for mod_name, mod in list(sys.modules.items()):
        if mod is None or not mod_name.startswith('codegen'):
            continue
        for attr, val in list(vars(mod).items()):
            if val is orig:
                setattr(mod, attr, wrapper)


# ── 序列化 ───────────────────────────────────────────────────────────────────

def cond_json(c):
    if c is None:
        return None
    if c.kind == 'atom':
        return {'k': 'atom', 'pos': c.pos, 'neg': c.neg}
    if c.kind == 'const':
        return {'k': 'const', 'v': bool(c.value)}
    return {'k': c.kind, 'items': [cond_json(i) for i in c.items]}


def label_json(lbl):
    if isinstance(lbl, tuple):
        return list(lbl)
    return lbl


def tree_json(seq):
    out = []
    for it in seq:
        if isinstance(it, S.Code):
            out.append(['code', it.block, bool(it.exits), bool(it.empty)])
        elif isinstance(it, S.Decl):
            out.append(['decl', [str(x) for x in it.lines]])
        elif isinstance(it, S.Block):
            out.append(['block', it.label, tree_json(it.body)])
        elif isinstance(it, S.Loop):
            out.append(['loop', it.header, tree_json(it.body), label_json(it.exit_label),
                        cond_json(it.while_cond), it.cond_origin])
        elif isinstance(it, S.If):
            out.append(['if', cond_json(it.cond), tree_json(it.then), tree_json(it.else_), it.origin])
        elif isinstance(it, S.Switch):
            out.append(['switch', it.key, [[v, tree_json(b)] for v, b in it.arms], it.origin])
        elif isinstance(it, S.Try):
            out.append(['try', tree_json(it.body), [[repr(d), tree_json(b)] for d, b in it.catches],
                        it.origin])
        elif isinstance(it, S.Break):
            out.append(['break', label_json(it.label)])
        elif isinstance(it, S.Continue):
            out.append(['continue', it.label])
        else:
            out.append(['unknown', repr(it)])
    return out


def node_json(n):
    return {
        'id': n.id, 'start_pc': n.start_pc, 'kind': n.kind, 'target': n.target,
        'fallthrough': n.fallthrough, 'cases': [[list(v), t] for v, t in n.cases],
        'default': n.default, 'cond': cond_json(n.cond), 'key': n.key, 'pcs': list(n.pcs),
        'has_stmts': bool(n.stmts), 'decls': [str(x) for x in n.decls],
        'ctx': sorted(getattr(n, 'ctx', frozenset())), 'group': n.group,
        'handlers': list(n.handlers), 'catches': [repr(c) for c in n.catches],
        'catch_ends': list(n.catch_ends or []),
    }


def flow_json(flow):
    return {
        'rpo': list(flow.rpo),
        'idom': sorted([k, v] for k, v in flow.idom.items()),
        'back_edges': sorted([u, v] for u, v in flow.back_edges),
        'loops': sorted([h, sorted(b)] for h, b in flow.loops.items()),
        'reducible': bool(flow.reducible),
    }


def succs_json(succs):
    return sorted([k, list(v)] for k, v in succs.items())


# ── 截获 ─────────────────────────────────────────────────────────────────────

_orig_build_blocks = G.build_blocks


def build_blocks_wrapper(instrs, exception_table=None, boundaries=None):
    inp = {
        'instrs': [[i.offset, i.opcode, i.operand] for i in instrs],
        'exc': [[s, e, h, t] for (s, e, h, t) in (exception_table or [])],
        'boundaries': sorted(boundaries or []),
    }
    try:
        blocks = _orig_build_blocks(instrs, exception_table, boundaries)
    except G.CfgError as e:
        _record('blocks', inp, {'error': str(e)})
        raise
    out = [{
        'id': b.id, 'start_idx': b.start_idx, 'end_idx': b.end_idx, 'start_pc': b.start_pc,
        'handler': bool(b.is_handler_entry), 'kind': b.term.kind, 'pc': b.term.pc,
        'target': b.term.target, 'fallthrough': b.term.fallthrough,
        'cases': [[list(v), t] for v, t in b.term.cases], 'default': b.term.default,
    } for b in blocks]
    _record('blocks', inp, out)
    return blocks


_orig_analyze = G.analyze


def analyze_wrapper(entry, succs):
    flow = _orig_analyze(entry, succs)
    _record('analyze', {'entry': entry, 'succs': succs_json(succs)}, flow_json(flow))
    return flow


_orig_structure = S.structure


def structure_wrapper(nodes, flow):
    inp = {'entry': flow.entry, 'succs': succs_json(flow.succs),
           'nodes': [node_json(nodes[k]) for k in sorted(nodes)]}
    try:
        tree = _orig_structure(nodes, flow)
    except G.CfgError as e:
        _record('structure', inp, {'error': str(e)})
        raise
    pre = tree_json(tree)
    ctx_after = [[k, sorted(getattr(nodes[k], 'ctx', frozenset()))] for k in sorted(nodes)]
    post = tree_json(SP.simplify(copy.deepcopy(tree)))
    _record('structure', inp, {'tree': pre, 'ctx_after': ctx_after, 'simplified': post})
    return tree


def _cmp_wrapper(kind, orig):
    def wrapper(opcode, a, b):
        out = orig(opcode, a, b)
        _record(kind, {'op': opcode, 'a': a, 'b': b}, out)
        return out
    return wrapper


def install() -> None:
    for orig, wrapper in ((_orig_build_blocks, build_blocks_wrapper),
                          (_orig_analyze, analyze_wrapper),
                          (_orig_structure, structure_wrapper)):
        _rebind(orig, wrapper)
    for name in ('cmp_op', 'neg_cmp_op'):
        orig = getattr(C, name)
        _rebind(orig, _cmp_wrapper(name, orig))


def main():
    argv = sys.argv[1:]
    java = [a for a in argv if a.endswith('.java')]
    if not java:
        sys.exit('用法：dump_cfg.py <Test.java> [main.py 参数]')
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
    out_dir = os.path.join(REPO, 'build', 'golden', 'cfg')
    os.makedirs(out_dir, exist_ok=True)
    with open(os.path.join(out_dir, f'{test}.jsonl'), 'w', encoding='utf-8') as f:
        for rec in _records.values():
            f.write(json.dumps(rec, ensure_ascii=False) + '\n')
    kinds: dict[str, int] = {}
    for rec in _records.values():
        kinds[rec['kind']] = kinds.get(rec['kind'], 0) + 1
    print(f'[golden-cfg] {test}: {len(_records)} 条 {kinds} → {out_dir}')


if __name__ == '__main__':
    main()

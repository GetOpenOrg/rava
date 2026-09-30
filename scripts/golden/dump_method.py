#!/usr/bin/env python3
"""
P4c golden 转储：在一次正常转译中截获方法体生成入口 `codegen.method.gen_method_body`
的输入视图与输出文本。

    python3 scripts/golden/dump_method.py tests/e2e/33_maps/TestHashMapOps.java --jdk 21 [main.py 其余参数]

（内部以 `scripts/main.py <Test.java> --no-run` 跑一遍正常转译。）

每条记录（一次 gen_method_body 调用）：
- `key`：`<class_info.name>.<方法名>:<描述符>`（与 P5a `dump_emit.py` 的 `/*BODY key*/` 占位同键）
- `cls` / `mcls`：class_info.name / method.class_name（继承展开时二者相同，均为发射所在类）
- `owner`：字节码出处类（指令表对象同一性反查注册表；查不到为 null，Rust 侧按名字沿层次找）
- `name` / `desc` / `acc` / `gsig`：方法视图（适配后的泛型签名）
- `lv`：局部变量表 `[slot, start, len, name, desc, sig]`（适配后）
- `ctp` / `ovl` / `rust_name` / `vt`：class_type_params / 是否重载 / rust_name / in_vtable_body
- `insns`：Python 指令视图投影 `[off, op, fold]`（Rust 侧核对规范化输入一致）；`et`：异常表
- `indy`：invokedynamic 指令 `[off, 常量池下标, 注释]`
- `sam`：本次调用中 `site_ctor_path` 查询及其结果 `[iface, current_class, out]`
- `fx`：本次调用的外部登记（audit / inherited / lambda_ref / sam_site），按发生顺序
- `text`：返回文本；或 `err`：异常 `类型名: 消息`

首行为 meta（与 dump_input 同键：Rust 侧以 BuildInput 重建发射层输入）。输出
`build/golden/method/<Test>.jsonl`。只读截获，不改 codegen 语义；随 Python 生成器一并退役。
"""

import json
import os
import runpy
import sys

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, REPO)

from codegen import equiv_audit as EA  # noqa: E402
from codegen import inherited_calls as IC  # noqa: E402
from codegen.instr import member_naming as MN  # noqa: E402
from codegen.emitter import sam_objects as SO  # noqa: E402
from codegen.emitter import class_writer as CW  # noqa: E402
from codegen.emitter import clinit_extract as CE  # noqa: E402
import codegen.method as M  # noqa: E402
import codegen.closure_input as CI  # noqa: E402

_records: list = []
_stack: list = []
_owner_of: dict = {}
_registry_ref: list = [None]
_RESOLVERS: list = []
_META: dict = {}


class _CapturingResolver(CI.JdkResolver):
    def __init__(self, *a, **kw):
        super().__init__(*a, **kw)
        _RESOLVERS.append(self)


CI.JdkResolver = _CapturingResolver


def _fx_hook(name, fn, keep_out=False, target='fx'):
    def wrapper(*a, **kw):
        out = fn(*a, **kw)
        if _stack:
            args = [x if isinstance(x, (str, int, type(None))) else repr(x) for x in a]
            if 'count' in kw:
                args.append(kw['count'])
            _stack[-1][target].append(args + [out] if keep_out else [name] + args)
        return out
    return wrapper


def _owner(method, registry) -> 'str | None':
    """指令表对象同一性 → 出处类（adapted 副本是浅拷贝，instrs 与原方法同一对象）"""
    if not registry or not method.instrs:
        return None
    key = id(method.instrs)
    hit = _owner_of.get(key)
    if hit is not None and hit[1] is method.instrs:
        return hit[0]
    if _registry_ref[0] is not registry or len(_owner_of) == 0 or hit is None:
        _registry_ref[0] = registry
        _owner_of.clear()
        for ci in registry.values():
            for m in (getattr(ci, 'methods', None) or ()):
                if m.instrs:
                    _owner_of.setdefault(id(m.instrs), (ci.name, m.instrs))
        hit = _owner_of.get(key)
    if hit is not None and hit[1] is method.instrs:
        return hit[0]
    return None


def _insn_view(i):
    fold = getattr(i, 'fold', None)
    return [i.offset, i.opcode, fold.opcode if fold is not None else None]


def _wrap(orig):
    def gen_method_body(method, class_info, registry=None, class_type_params=None,
                        overloaded_names=None, rust_name=None, in_vtable_body=False):
        rec = {
            'key': f'{class_info.name}.{method.name}:{method.descriptor}',
            'cls': class_info.name,
            'mcls': method.class_name,
            'owner': _owner(method, registry),
            'name': method.name,
            'desc': method.descriptor,
            'acc': method.access_flags,
            'gsig': method.generic_signature or '',
            'lv': [list(e) for e in (method.local_vars or [])],
            'ctp': list(class_type_params or []),
            'ovl': bool(overloaded_names is not None and method.name in overloaded_names),
            'rust_name': rust_name,
            'vt': bool(in_vtable_body),
            'insns': [_insn_view(i) for i in (method.instrs or [])],
            'et': [list(e) for e in (method.exception_table or [])],
            'indy': [[i.offset, i.operand, i.comment] for i in (method.instrs or [])
                     if i.opcode == 'invokedynamic'],
            'sam': [],
            'fx': [],
        }
        _stack.append(rec)
        try:
            text = orig(method, class_info, registry=registry,
                        class_type_params=class_type_params,
                        overloaded_names=overloaded_names, rust_name=rust_name,
                        in_vtable_body=in_vtable_body)
            rec['text'] = text
            return text
        except Exception as e:  # noqa: BLE001  记录后原样抛出（发射层据此出 stub）
            rec['err'] = f'{type(e).__name__}: {e}'
            raise
        finally:
            _stack.pop()
            _records.append(rec)
    return gen_method_body


def install():
    EA.record = _fx_hook('audit', EA.record)
    IC.request = _fx_hook('inherited', IC.request)
    _led = MN.LAMBDA_NAME_LEDGER
    _led.record_reference = _fx_hook('lambda_ref', _led.record_reference)
    SO.site_ctor_path = _fx_hook('sam_ctor', SO.site_ctor_path, keep_out=True, target='sam')
    SO.record_site = _fx_hook('sam_site', SO.record_site)
    CW.gen_method_body = _wrap(CW.gen_method_body)
    CE.gen_method_body = _wrap(CE.gen_method_body)
    M.gen_method_body = CW.gen_method_body
    T = sys.modules["codegen.transpile"]
    orig_write = T.write_cargo_project

    def write_hook(out_dir, class_infos, jdk_class_infos, java_files, **kw):
        _META['user_dir'] = os.path.join(out_dir, 'closure_input', 'classes')
        _META['closure_json'] = os.path.join(out_dir, 'closure_input', 'closure.json')
        _META['user_classes'] = [ci.name for ci in class_infos]
        return orig_write(out_dir, class_infos, jdk_class_infos, java_files, **kw)

    T.write_cargo_project = write_hook


def main():
    argv = sys.argv[1:]
    java = [a for a in argv if a.endswith('.java')]
    if not java:
        sys.exit('用法：dump_method.py <Test.java> [main.py 参数]')
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
    out_dir = os.path.join(REPO, 'build', 'golden', 'method')
    os.makedirs(out_dir, exist_ok=True)
    meta = {
        'meta': True,
        'java_home': str(res._home) if res is not None else '',
        'image_dirs': list(res.image_class_dirs()) if res is not None else [],
        'runtime': os.path.join(REPO, 'runtime', 'java_runtime'),
        'libs': [],
    }
    meta.update(_META)
    with open(os.path.join(out_dir, f'{test}.jsonl'), 'w', encoding='utf-8') as f:
        f.write(json.dumps(meta, ensure_ascii=False) + '\n')
        for rec in _records:
            f.write(json.dumps(rec, ensure_ascii=False) + '\n')
    n_err = sum(1 for r in _records if 'err' in r)
    print(f'[golden-method] {test}: {len(_records)} 次方法体生成（异常 {n_err}）→ {out_dir}')


if __name__ == '__main__':
    main()

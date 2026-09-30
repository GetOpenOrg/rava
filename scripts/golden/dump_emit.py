#!/usr/bin/env python3
"""发射层 golden 采集（P5a 骨架）：跑一遍正常 Python 转译（javac → rava closure → 折叠 →
registry → write_cargo_project 真实落盘），截获全部生成文件，把方法体替换为
`/*BODY <类.名:描述符>*/` 占位后落盘为对照真源。

截获方式（只读包装，不改 codegen 语义）：
  - `gen_method_body`（class_writer / clinit_extract 两处引用）包一层：调用原函数，返回文本里
    把「方法体段」包进哨兵行 `//@@BODY_BEGIN <key>` / `//@@BODY_END`。发射层后续基于文本的
    处理（VTable 导入扫描、record 补丁、ClassEmission 记录）看到的仍是真实方法体；
  - 方法体生成期间登记的事实（inherited_calls.request / LAMBDA_NAME_LEDGER.record_reference /
    sam_objects.record_site）与兜底异常按调用逐条记录（facts JSONL），供 Rust 占位
    MethodBodyEmitter 回放；
  - 落盘后遍历 scratch：哨兵段替换为 `{体内缩进}/*BODY <key>*/`，与 runtime/ 手写真源逐字节相同的
    overlay 文件不转储，其余（生成 .rs / mod.rs / main.rs / Cargo.toml / module_resources.rs 等）
    原样写到 build/golden/emit/<Test>/files/。

用法：
    python3 scripts/golden/dump_emit.py tests/e2e/33_maps/TestHashMapOps.java [...]
输出：
    build/golden/emit/<Test>/meta.json     与 dump_input 同键（Rust 侧重建 BuildInput）+ out_dir / pkg_version
    build/golden/emit/<Test>/bodies.jsonl  每次方法体生成的调用记录（顺序 = Python 调用序）
    build/golden/emit/<Test>/files/**      替换方法体后的生成文件
scratch：build/golden/emit/<Test>.scratch（每次清空）
"""

from __future__ import annotations

import json
import os
import re
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(ROOT))
sys.path.insert(0, str(ROOT / 'scripts'))
sys.path.insert(0, str(ROOT / 'scripts' / 'golden'))

from jdk_select import apply_jdk  # noqa: E402

import dump_input as _di  # noqa: E402  （复用 JdkResolver 截获；其折叠包装只记录不改语义）
import main as _main  # noqa: E402  （prepare_scratch：与真实运行同一 overlay）
from codegen import inherited_calls as _ic  # noqa: E402
from codegen.constants import scratch_pkg_version  # noqa: E402
from codegen.emitter import class_writer as _cw  # noqa: E402
from codegen.emitter import clinit_extract as _ce  # noqa: E402
from codegen.emitter import project_writer as _pw  # noqa: E402
from codegen.emitter import sam_objects as _sam  # noqa: E402
from codegen.instr.member_naming import LAMBDA_NAME_LEDGER as _LEDGER  # noqa: E402

_transpile = sys.modules['codegen.transpile']

BODY_BEGIN = '//@@BODY_BEGIN '
BODY_END = '//@@BODY_END'

_CALLS: list = []
_STACK: list = []   # 当前方法体调用的记录（嵌套防御）

# ── 方法体事实登记截获 ───────────────────────────────────────────────────────

_orig_request = _ic.request


def _request(receiver_binary, method_name, param_descriptor):
    if _STACK:
        _STACK[-1]['requests'].append([receiver_binary, method_name, param_descriptor])
    return _orig_request(receiver_binary, method_name, param_descriptor)


_ic.request = _request

_orig_record_reference = _LEDGER.record_reference


def _record_reference(cls_bin, mname, rust_name):
    if _STACK:
        _STACK[-1]['lambda_refs'].append([cls_bin, mname, rust_name])
    return _orig_record_reference(cls_bin, mname, rust_name)


_LEDGER.record_reference = _record_reference

_orig_record_site = _sam.record_site


def _record_site(iface_bin, sam_desc, current_class):
    if _STACK:
        _STACK[-1]['sam_sites'].append([iface_bin, sam_desc, current_class])
    return _orig_record_site(iface_bin, sam_desc, current_class)


_sam.record_site = _record_site

# ── gen_method_body 包装 ─────────────────────────────────────────────────────


def _split_body(text: str) -> 'tuple[str, str, str] | None':
    """gen_method_body 返回文本 → (头部含 ` {\\n`, 方法体, 尾部 `\\n}`)。

    构造器形态有两个 fn（new 包装 + __init_on 实体），方法体在最后一个 fn；
    其余形态方法体在第一个 ` {\\n` 之后。"""
    if not text.endswith('\n}'):
        return None
    hidden = text.rfind('#[doc(hidden)]\n')
    start_from = hidden if hidden >= 0 else 0
    open_idx = text.find(' {\n', start_from)
    if open_idx < 0:
        return None
    head_end = open_idx + len(' {\n')
    body_end = len(text) - len('\n}')
    if body_end < head_end - 1:
        return None
    if body_end < head_end:   # 空体：`sig {\n}`
        return text[:head_end], '', text[head_end:]
    return text[:head_end], text[head_end:body_end], text[body_end:]


def _wrap(orig):
    def gen_method_body(method, class_info, registry=None, class_type_params=None,
                        overloaded_names=None, rust_name=None, in_vtable_body=False):
        key = f'{class_info.name}.{method.name}:{method.descriptor}'
        rec = {'key': key, 'rust_name': rust_name, 'in_vtable_body': bool(in_vtable_body),
               'requests': [], 'lambda_refs': [], 'sam_sites': []}
        _STACK.append(rec)
        try:
            text = orig(method, class_info, registry=registry,
                        class_type_params=class_type_params,
                        overloaded_names=overloaded_names, rust_name=rust_name,
                        in_vtable_body=in_vtable_body)
        except Exception as e:  # noqa: BLE001  兜底异常：记录后原样抛出（发射层据此出 stub）
            rec['fallback'] = f'{type(e).__name__}: {e}'
            _CALLS.append(rec)
            raise
        finally:
            _STACK.pop()
        parts = _split_body(text)
        if parts is None:
            rec['unsplit'] = True
            rec['text'] = text
            _CALLS.append(rec)
            return text
        head, body, tail = parts
        rec['body'] = body
        rec['head'] = head
        _CALLS.append(rec)
        return f'{head}{BODY_BEGIN}{key}\n{body}\n{BODY_END}{tail}' if body else \
            f'{head}{BODY_BEGIN}{key}\n{BODY_END}{tail}'
    return gen_method_body


_cw.gen_method_body = _wrap(_cw.gen_method_body)
_ce.gen_method_body = _wrap(_ce.gen_method_body)

# ── 落盘后替换 ────────────────────────────────────────────────────────────────

_BEGIN_RE = re.compile(r'^([ \t]*)' + re.escape(BODY_BEGIN) + r'(.*)$')


def _replace_bodies(text: str) -> str:
    out: list[str] = []
    lines = text.split('\n')
    i = 0
    while i < len(lines):
        m = _BEGIN_RE.match(lines[i])
        if m is None:
            out.append(lines[i])
            i += 1
            continue
        depth = 1
        j = i + 1
        while j < len(lines):
            s = lines[j].strip()
            if s.startswith(BODY_BEGIN):
                depth += 1
            elif s == BODY_END:
                depth -= 1
                if depth == 0:
                    break
            j += 1
        out.append(f'{m.group(1)}    /*BODY {m.group(2)}*/')  # 哨兵在方法体外层缩进，占位按体内一级缩进
        i = j + 1
    return '\n'.join(out)


def _same_as_runtime(rel: str, data: bytes) -> bool:
    prefix = 'java_runtime/src/'
    if not rel.startswith(prefix):
        return False
    src = ROOT / 'runtime' / 'java_runtime' / 'src' / rel[len(prefix):]
    return src.is_file() and src.read_bytes() == data


_SKIP_TOP = {'closure_input', 'hw_scan', 'target'}


def _dump_files(scratch: Path, dest: Path) -> int:
    n = 0
    for root, dirs, files in os.walk(scratch):
        rel_root = os.path.relpath(root, scratch)
        if rel_root == '.':
            dirs[:] = sorted(d for d in dirs if d not in _SKIP_TOP)
        else:
            dirs.sort()
        for fname in sorted(files):
            path = Path(root) / fname
            rel = os.path.relpath(path, scratch).replace(os.sep, '/')
            data = path.read_bytes()
            if _same_as_runtime(rel, data):
                continue
            if fname.endswith('.rs'):
                text = data.decode('utf-8')
                if BODY_BEGIN in text:
                    text = _replace_bodies(text)
                data = text.encode('utf-8')
            out = dest / rel
            out.parent.mkdir(parents=True, exist_ok=True)
            out.write_bytes(data)
            n += 1
    return n


class _Done(Exception):
    pass


def _make_hook(test_dir: Path, scratch: Path):
    orig_write = _pw.write_cargo_project

    def hook(out_dir, class_infos, jdk_class_infos, java_files, **kw):
        orig_write(out_dir, class_infos, jdk_class_infos, java_files, **kw)
        resolver = _di._RESOLVERS[-1]
        files_dir = test_dir / 'files'
        shutil.rmtree(files_dir, ignore_errors=True)
        n = _dump_files(scratch, files_dir)
        meta = {
            'java_home': str(resolver._home),
            'user_dir': os.path.join(out_dir, 'closure_input', 'classes'),
            'image_dirs': list(resolver.image_class_dirs()),
            'runtime': str(ROOT / 'runtime' / 'java_runtime'),
            'closure_json': os.path.join(out_dir, 'closure_input', 'closure.json'),
            'user_classes': [ci.name for ci in class_infos],
            'libs': [],
            'out_dir': os.path.abspath(out_dir),
            'pkg_version': scratch_pkg_version(out_dir),
            'java_files': [os.path.abspath(f) for f in java_files or []],
        }
        (test_dir / 'meta.json').write_text(json.dumps(meta, ensure_ascii=False, indent=1) + '\n',
                                            encoding='utf-8')
        with open(test_dir / 'bodies.jsonl', 'w', encoding='utf-8') as fh:
            for rec in _CALLS:
                fh.write(json.dumps(rec, ensure_ascii=False) + '\n')
        print(f'[dump_emit] {n} 个文件，{len(_CALLS)} 次方法体生成 → {test_dir}')
        raise _Done()
    return hook


def main(argv: list[str]) -> int:
    if not argv:
        print(__doc__)
        return 2
    apply_jdk(21, quiet=True)
    for java_file in argv:
        stem = Path(java_file).stem
        test_dir = ROOT / 'build' / 'golden' / 'emit' / stem
        scratch = ROOT / 'build' / 'golden' / 'emit' / f'{stem}.scratch'
        shutil.rmtree(scratch, ignore_errors=True)
        test_dir.mkdir(parents=True, exist_ok=True)
        _CALLS.clear()
        _main.prepare_scratch(str(scratch), clean=True)
        _transpile.write_cargo_project = _make_hook(test_dir, scratch)
        try:
            _transpile.transpile([os.path.abspath(java_file)], str(scratch))
        except _Done:
            continue
        print(f'[dump_emit] {java_file}：未到达 write_cargo_project', file=sys.stderr)
        return 1
    return 0


if __name__ == '__main__':
    sys.exit(main(sys.argv[1:]))

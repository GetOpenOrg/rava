#!/usr/bin/env python3
"""classfile 解析 golden 对照：Python（codegen/classfile.py）与 Rust（rava dump-classes）
对同一 jmod 的全部类输出归一形态（每类一行 JSON），逐行比较。

用法：
    python3 scripts/classfile_golden.py --jdk 21 [--module java.base] [--prefix java/lang/]

归一规则与 generator/crates/driver/src/dump.rs 一一对应；float/double 常量只比较类别，
孤立代理项归一为 U+FFFD（Rust String 不容纳孤立代理）。
"""

import argparse
import json
import os
import subprocess
import sys
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from codegen.classfile import parse_class_bytes  # noqa: E402
from codegen.jdk_resolver import find_java_home, _installed_jdks  # noqa: E402
import codegen.vm_constants as _vm_constants  # noqa: E402

# 对照的是原始解码：VM 常量死分支剪除属语义层（Rust 侧由闭包引擎按 [vm_constants] 折叠），
# 这里置为恒等，使两侧比较同一份字节码
_vm_constants.prune_dead_guards = lambda instrs, exception_table: instrs

def _fix(s: str) -> str:
    return ''.join('�' if 0xD800 <= ord(ch) <= 0xDFFF else ch for ch in s)


def _ldc(comment: str) -> str:
    kind, _, val = comment.partition(' ')
    if kind == 'String':
        return 'S:' + _fix(val)
    if kind == 'int':
        return 'I:' + val
    if kind == 'long':
        return 'J:' + val
    if kind == 'class':
        return 'C:' + val
    if kind == 'float':
        return 'F'
    if kind == 'double':
        return 'D'
    return 'O'


def _insn(i) -> str:
    op, c, operand = i.opcode, i.comment or '', i.operand
    if op in ('ldc', 'ldc_w', 'ldc2_w'):
        v = _ldc(c)
    elif c.startswith(('Method ', 'InterfaceMethod ', 'Field ')):
        v = c.split(' ', 1)[1]
    elif op == 'invokedynamic':
        v = c.split(' ')[1]
    elif op in ('new', 'anewarray', 'checkcast', 'instanceof'):
        v = c
    elif op == 'multianewarray':
        v = f"{c} {operand.split()[1]}"
    else:
        v = operand or ''
    return f"{i.offset} {op} {_fix(v)}"


def class_json(ci) -> dict:
    return {
        'name': ci.name,
        'super': ci.super_class or '',
        'interfaces': list(ci.interfaces or []),
        'access': ci.access_flags,
        'major': ci.major_version,
        'fields': [[_fix(f.name), f.descriptor, f.access_flags] for f in ci.fields],
        'methods': [[m.name, m.descriptor, m.access_flags,
                     [_insn(i) for i in (m.instrs or [])],
                     [[e[0], e[1], e[2], e[3] or ''] for e in (m.exception_table or [])]]
                    for m in ci.methods],
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument('--jdk', type=int)
    ap.add_argument('--module', default='java.base')
    ap.add_argument('--prefix', default='')
    ap.add_argument('--keep', help='两侧输出落盘目录（排查用）')
    a = ap.parse_args()

    home = next((h for m, h in _installed_jdks() if m == a.jdk), None) if a.jdk else find_java_home()
    if home is None:
        print(f"找不到 JDK {a.jdk}", file=sys.stderr)
        return 2
    jmod = Path(home) / 'jmods' / f'{a.module}.jmod'

    py_lines = []
    with zipfile.ZipFile(jmod) as zf:
        for n in sorted(zf.namelist()):
            if not (n.startswith('classes/') and n.endswith('.class')) or n.endswith('module-info.class'):
                continue
            bin_name = n[len('classes/'):-len('.class')]
            if not bin_name.startswith(a.prefix):
                continue
            ci = parse_class_bytes(zf.read(n), bin_name)
            py_lines.append(json.dumps(class_json(ci), ensure_ascii=False, sort_keys=True, separators=(',', ':')))

    gen = ROOT / 'generator'
    r = subprocess.run(['cargo', 'run', '--offline', '--release', '-q', '--bin', 'rava', '--',
                        'dump-classes', '--java-home', str(home), '--module', a.module,
                        '--prefix', a.prefix],
                       cwd=gen, capture_output=True, text=True)
    if r.returncode != 0:
        print(r.stderr, file=sys.stderr)
        return 1
    rs_lines = [json.dumps(json.loads(line), ensure_ascii=False, sort_keys=True, separators=(',', ':'))
                for line in r.stdout.split('\n') if line.strip()]  # 不用 splitlines：U+2028 等不转义
    # 两侧按类名对齐（归档遍历顺序不参与比较）
    key = lambda line: json.loads(line)['name']
    py_lines.sort(key=key)
    rs_lines.sort(key=key)

    if a.keep:
        os.makedirs(a.keep, exist_ok=True)
        Path(a.keep, 'py.jsonl').write_text('\n'.join(py_lines) + '\n')
        Path(a.keep, 'rs.jsonl').write_text('\n'.join(rs_lines) + '\n')

    diff = 0
    if len(py_lines) != len(rs_lines):
        print(f"类数不一致：Python {len(py_lines)} / Rust {len(rs_lines)}")
        diff += 1
    for p, q in zip(py_lines, rs_lines):
        if p != q:
            diff += 1
            if diff <= 5:
                pj, qj = json.loads(p), json.loads(q)
                print(f"差异：{pj['name']}")
                for k in pj:
                    if pj[k] != qj.get(k):
                        if k == 'methods':
                            for pm, qm in zip(pj[k], qj[k]):
                                if pm != qm:
                                    print(f"  方法 {pm[0]}{pm[1]}")
                                    for x, y in zip(pm[3], qm[3]):
                                        if x != y:
                                            print(f"    py: {x}\n    rs: {y}")
                                            break
                                    break
                        else:
                            print(f"  {k}: py={pj[k]!r} rs={qj.get(k)!r}")
    print(f"[classfile-golden] {a.module}{(' ' + a.prefix) if a.prefix else ''}："
          f"{len(py_lines)} 类，差异 {diff}")
    return 0 if diff == 0 else 1


if __name__ == '__main__':
    sys.exit(main())

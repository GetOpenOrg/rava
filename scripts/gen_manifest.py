"""
清单生成器：读取 Java 源文件，执行 RTA + 分类，输出 manifest.toml。

用法：
    python3 scripts/gen_manifest.py tests/HelloWorld.java
    python3 scripts/gen_manifest.py tests/TestP1.java --out manifest.toml
"""

import sys
import os
import subprocess
import argparse
from datetime import datetime, timezone
from pathlib import Path

sys.path.insert(0, os.path.dirname(__file__))

from codegen.classfile import parse_class
from codegen.classify import classify_class, MethodCategory

try:
    import tomllib
except ImportError:
    tomllib = None

try:
    import tomli_w as tomli_write
    _HAS_TOMLI_W = True
except ImportError:
    _HAS_TOMLI_W = False


def _to_toml_str(data: dict) -> str:
    """简单的 TOML 序列化（不依赖第三方库）。"""
    lines = []

    # [meta]
    meta = data.get('meta', {})
    lines.append('[meta]')
    for k, v in meta.items():
        if isinstance(v, str):
            lines.append(f'{k} = "{v}"')
        else:
            lines.append(f'{k} = {v}')
    lines.append('')

    # [[method]]
    for m in data.get('method', []):
        lines.append('[[method]]')
        for k, v in m.items():
            if isinstance(v, str):
                lines.append(f'{k} = "{v}"')
            else:
                lines.append(f'{k} = {v}')
        lines.append('')

    return '\n'.join(lines)


def generate_manifest(java_files: list[str], out_path: str):
    # 1. javac
    src_dir   = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    print(f"[1/3] javac {' '.join(java_files)}")
    r = subprocess.run(['javac', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f"javac failed:\n{r.stderr}")

    # 2. 解析 + 分类
    all_methods = []
    for jf in java_files:
        class_name = os.path.splitext(os.path.basename(jf))[0]
        class_file = os.path.join(class_dir, class_name + '.class')
        print(f"[2/3] 解析分类 {class_name}.class")
        ci = parse_class(class_file)
        classified = classify_class(ci.name, ci.methods)
        all_methods.extend(classified)

    # 3. 生成清单
    print(f"[3/3] 生成清单 → {out_path}")
    from collections import Counter
    counts = Counter(m.category.value for m in all_methods)

    meta = {
        'generated_at': datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ'),
        'total_reachable': len(all_methods),
        'auto_transpile':  counts.get('auto_transpile', 0),
        'native_stub':     counts.get('native_stub', 0),
        'unsupported':     counts.get('unsupported', 0),
        'manual_override': counts.get('manual_override', 0),
    }

    method_entries = []
    for m in all_methods:
        entry = {
            'id':          f"{m.class_name.replace('.', '/')}.{m.method_name}#{m.descriptor}",
            'category':    m.category.value,
            'class_name':  m.class_name,
            'method_name': m.method_name,
            'descriptor':  m.descriptor,
            'status':      m.status,
        }
        if m.reason:
            entry['reason'] = m.reason
        if m.suggested_impl:
            entry['suggested_impl'] = m.suggested_impl
        method_entries.append(entry)

    manifest = {'meta': meta, 'method': method_entries}
    content = _to_toml_str(manifest)

    with open(out_path, 'w') as f:
        f.write(content)

    print(f"\n清单摘要：total={len(all_methods)}  "
          f"auto={counts.get('auto_transpile',0)}  "
          f"native={counts.get('native_stub',0)}  "
          f"unsupported={counts.get('unsupported',0)}  "
          f"manual={counts.get('manual_override',0)}")
    print(f"✓ 清单已写入 {out_path}")


def main():
    ap = argparse.ArgumentParser(description='生成 manifest.toml')
    ap.add_argument('java_files', nargs='+', help='.java 源文件')
    ap.add_argument('--out', default='manifest.toml', help='输出路径（默认 manifest.toml）')
    args = ap.parse_args()
    generate_manifest(args.java_files, args.out)


if __name__ == '__main__':
    main()

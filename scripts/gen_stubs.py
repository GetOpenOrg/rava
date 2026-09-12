"""
空壳生成器：读取 manifest.toml，为每个未实现方法生成带 compile_error!/todo!() 的 Rust 空壳。

用法：
    python3 scripts/gen_stubs.py manifest.toml --out output/src/java_runtime
    python3 scripts/gen_stubs.py manifest.toml          # 默认输出到 output/src/java_runtime
"""

import sys
import os
import re
import argparse
from pathlib import Path

try:
    import tomllib
except ImportError:
    sys.exit("需要 Python 3.11+ 内置的 tomllib，或安装 tomli 包")

sys.path.insert(0, os.path.dirname(__file__))
from codegen.type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return


# ── 工具函数 ─────────────────────────────────────────────────────────────────

def _snake(name: str) -> str:
    """PascalCase / camelCase → snake_case。"""
    s = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', name)
    s = re.sub(r'([a-z])([A-Z])', r'\1_\2', s)
    return s.lower()


def _jvm_to_rust_params(descriptor: str) -> list[tuple[str, str]]:
    """从方法描述符生成 (param_name, rust_type) 列表。"""
    types = parse_descriptor_params(descriptor)
    return [(f'arg_{i}', jvm_to_rust(t)) for i, t in enumerate(types)]


def _method_stub(method_id: str, method_name: str, descriptor: str,
                 category: str, reason: str, status: str, suggested_impl: str) -> str:
    """生成单个方法的 Rust 空壳。"""
    params = _jvm_to_rust_params(descriptor)
    ret_jvm = parse_descriptor_return(descriptor)
    ret_rust = jvm_to_rust(ret_jvm)

    param_str = ', '.join(f'{n}: {t}' for n, t in params)
    ret_str   = f' -> {ret_rust}' if ret_rust != '()' else ''

    comment_lines = [
        f'// AUTO-GENERATED stub — do not edit',
        f'// Source: {method_id}',
    ]
    if reason:
        comment_lines.append(f'// Reason: {reason}')
    if suggested_impl:
        comment_lines.append(f'// Suggested: {suggested_impl}')
    if status == 'done':
        comment_lines.append(f'// Status: DONE — linked to manual implementation')
    else:
        comment_lines.append(f'// Status: TODO')

    if status == 'done':
        body = f'    unimplemented!("manual implementation should replace this stub")'
    elif category == 'native_stub':
        escaped = method_id.replace('"', '\\"')
        body = f'    compile_error!("native stub not implemented: {escaped}")'
        # compile_error! 不能在函数体内用（Rust 限制），改用 todo!
        body = f'    todo!("native stub not implemented: {method_id}")'
    else:
        body = f'    todo!("{reason or "unsupported pattern"}: {method_id}")'

    lines = comment_lines + [
        f'#[allow(unused_variables)]',
        f'pub fn {_snake(method_name)}({param_str}){ret_str} {{',
        body,
        f'}}',
    ]
    return '\n'.join(lines)


def _parse_manifest(path: str) -> list[dict]:
    with open(path, 'rb') as f:
        data = tomllib.load(f)
    return data.get('method', [])


def generate_stubs(manifest_path: str, out_dir: str):
    methods = _parse_manifest(manifest_path)

    # 按 class_name 分组
    from collections import defaultdict
    by_class: dict[str, list[dict]] = defaultdict(list)
    for m in methods:
        cat = m.get('category', '')
        if cat in ('native_stub', 'unsupported'):
            by_class[m['class_name']].append(m)

    if not by_class:
        print("没有需要生成空壳的方法（native_stub / unsupported）。")
        return

    generated = 0
    for class_name, ms in by_class.items():
        # class_name 可能是 TestP1 或 java/io/PrintStream
        pkg_path = class_name.replace('.', '/').replace('/', os.sep)
        parts    = pkg_path.rsplit(os.sep, 1)
        if len(parts) == 2:
            pkg_dir, cls_file = parts[0], _snake(parts[1]) + '.rs'
        else:
            pkg_dir, cls_file = '', _snake(parts[0]) + '.rs'

        target_dir = os.path.join(out_dir, pkg_dir) if pkg_dir else out_dir
        os.makedirs(target_dir, exist_ok=True)
        target_file = os.path.join(target_dir, cls_file)

        stubs = []
        for m in ms:
            stub = _method_stub(
                method_id    = m.get('id', ''),
                method_name  = m['method_name'],
                descriptor   = m['descriptor'],
                category     = m['category'],
                reason       = m.get('reason', ''),
                status       = m.get('status', 'todo'),
                suggested_impl = m.get('suggested_impl', ''),
            )
            stubs.append(stub)

        content = (
            '#![allow(unused_variables, dead_code)]\n\n'
            + '\n\n'.join(stubs)
            + '\n'
        )
        with open(target_file, 'w') as f:
            f.write(content)
        print(f"  → {target_file}  ({len(ms)} stubs)")
        generated += len(ms)

    print(f"\n✓ 生成 {generated} 个方法空壳")


def main():
    ap = argparse.ArgumentParser(description='从 manifest.toml 生成 Rust 空壳')
    ap.add_argument('manifest', help='manifest.toml 路径')
    ap.add_argument('--out', default='output/src/java_runtime',
                    help='输出目录（默认 output/src/java_runtime）')
    args = ap.parse_args()
    generate_stubs(args.manifest, args.out)


if __name__ == '__main__':
    main()

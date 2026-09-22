#!/usr/bin/env python3
"""A-4 批次 3+ 测量仪表（口径对齐 docs/reports/2026-09-21-a4-phase0-evidence.md §5/§7）。

对每个测试 `--no-run --clean` 转译后，在带 `java_rta_macros::java_class` 标记的生成文件上计数：

  from_any      Object::from_any 总数（分段：merge-box `_mergedN =` / let-align `let _tN =`
                / inline 其余——阶段 0 §1 的分类口径）
  into_iface    Into::<接口载体>::into( —— 批次 3+ 记分牌（接口短名从生成文件的
                `#[is_interface = true]` + `pub struct` 收集）
  into_object   Into::<Object>::into( —— A-1 边界（勿误伤域，只观察）
  try_cast_iface / downcast_ref / downcast —— 保持项

用法：
  python3 docs/reports/a4b3_measure.py TestIterator TestStreamBasic   # 指定测试
  python3 docs/reports/a4b3_measure.py --set full                     # §7 全 45 测集
输出：逐测试行 + TOTAL，供批次前后对照（tee 到 /tmp 落盘）。
"""
import argparse
import os
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
sys.path.insert(0, str(ROOT))
from codegen.emitter import to_snake  # noqa: E402  与 scripts/main.py 的 scratch 命名同源

# §7 测量集（2026-09-21-a4-phase0-evidence.md §7 逐行）
FULL_SET = """TestArrayList TestArraysUtil TestAutoboxEdge TestAutoboxing TestBoundedGenerics
TestCasting TestCollectionFactory TestCollections TestComparable TestComparator
TestDefaultMethods TestEnumMethods TestForEach TestFunctionalInterface TestGenericMethod
TestHashMapOps TestInheritedMethod TestInterfaceStatic TestInterfaces TestIterator
TestLambda TestLambdaCapture TestLambdaVar TestLinkedHash TestLinkedList TestListOf
TestMapIteration TestMethodRef TestMultiCatch TestOptional TestOptionalFull
TestPatternMatch TestPriorityQueue TestSorting TestStreamAdvanced TestStreamBasic
TestStreamCollectors TestStreamMore TestStringBuilder TestStringBuilderOps
TestStringEdge TestStringRegex TestStringSearch TestTreeMapSet TestVarargs""".split()

_IFACE_RE = re.compile(r'#\[is_interface\s*=\s*true\]')
_STRUCT_RE = re.compile(r'pub struct (\w+)')
_INTO_RE = re.compile(r'Into::<(\w+)')
_MERGED_RE = re.compile(r'_merged\d+\s*=')
_LET_ALIGN_RE = re.compile(r'let _t\d+\s*=')


def _find_java(cls: str) -> Path:
    hits = list((ROOT / 'tests' / 'e2e').rglob(f'{cls}.java'))
    if not hits:
        sys.exit(f'[measure] 找不到测试 {cls}')
    return hits[0]


def _collect(build_dir: Path) -> dict:
    iface_shorts: set[str] = set()
    files: list[tuple[Path, str]] = []
    for crate in ('java_runtime', 'user'):
        src = build_dir / crate / 'src'
        if not src.is_dir():
            continue
        for p in src.rglob('*.rs'):
            try:
                text = p.read_text(encoding='utf-8')
            except Exception:
                continue
            if 'java_rta_macros::java_class' not in text:
                continue
            files.append((p, text))
            if _IFACE_RE.search(text):
                for m in _STRUCT_RE.finditer(text):
                    iface_shorts.add(m.group(1))
    counts = dict(from_any=0, mrg=0, let_align=0, inl=0, into_iface=0,
                  into_object=0, try_cast_iface=0, downcast_ref=0, downcast=0)
    for _p, text in files:
        for i, line in enumerate(text.split('\n')):
            n = line.count('Object::from_any')
            if n:
                counts['from_any'] += n
                if _MERGED_RE.search(line):
                    counts['mrg'] += n
                elif _LET_ALIGN_RE.search(line):
                    counts['let_align'] += n
                else:
                    counts['inl'] += n
            counts['try_cast_iface'] += line.count('try_cast_iface')
            counts['downcast_ref'] += line.count('downcast_ref')
            counts['downcast'] += line.count('.downcast::<')
            for m in _INTO_RE.finditer(line):
                head = m.group(1)
                if head == 'Object':
                    counts['into_object'] += 1
                elif head in iface_shorts:
                    counts['into_iface'] += 1
    return counts


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('tests', nargs='*')
    ap.add_argument('--set', choices=['full'], dest='which_set')
    ap.add_argument('--skip-transpile', action='store_true',
                    help='只统计现有 build/（复测同一生成树）')
    args = ap.parse_args()
    tests = args.tests or (FULL_SET if args.which_set == 'full' else [])
    if not tests:
        ap.error('给出测试名或 --set full')

    rows = []
    for cls in tests:
        java = _find_java(cls)
        out_dir = ROOT / 'build' / to_snake(cls)
        if not args.skip_transpile:
            r = subprocess.run(
                [sys.executable, 'scripts/main.py', str(java), '--no-run', '--clean'],
                cwd=ROOT, capture_output=True, text=True)
            if r.returncode != 0:
                print(f'[measure] {cls} 转译失败:\n{r.stdout[-2000:]}\n{r.stderr[-2000:]}')
                rows.append((cls, None))
                continue
            for line in r.stdout.split('\n'):
                if line.startswith('[readability-audit]') or line.startswith('[raw-audit]'):
                    pass  # 逐测试明细以生成树复扫为准（口径统一）
        c = _collect(out_dir)
        rows.append((cls, c))

    hdr = f"{'test':38s} {'from_any':>8s} {'mrg':>4s} {'let':>4s} {'inl':>4s} {'Into<I>':>8s} {'Into<Obj>':>9s} {'cast_if':>7s}"
    print(hdr)
    tot = {k: 0 for _n, c in rows if c for k in c}
    for cls, c in rows:
        if c is None:
            print(f'{cls:38s} FAIL')
            continue
        for k, v in c.items():
            tot[k] = tot.get(k, 0) + v
        print(f"{cls.replace('Test', 'test_'):38s} {c['from_any']:8d} {c['mrg']:4d} "
              f"{c['let_align']:4d} {c['inl']:4d} {c['into_iface']:8d} {c['into_object']:9d} "
              f"{c['try_cast_iface']:7d}")
    print(f"{'TOTAL':38s} {tot.get('from_any', 0):8d} {tot.get('mrg', 0):4d} "
          f"{tot.get('let_align', 0):4d} {tot.get('inl', 0):4d} {tot.get('into_iface', 0):8d} "
          f"{tot.get('into_object', 0):9d} {tot.get('try_cast_iface', 0):7d}")
    if any(c is None for _n, c in rows):
        sys.exit(1)


if __name__ == '__main__':
    main()

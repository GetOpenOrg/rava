#!/usr/bin/env python3
"""编译前缺口扫描：一次性找出调用链可达、但缺 native 实现 / 落在 panic 存根上的全部成员。

不编译、不运行——转译后扫描生成产物（每例约 3.5 分钟；完整转译 + 编译 + 运行约 13 分钟）。

两种种子：

  API 模式（与测试无关）：指定包的全部 public 类的 public / protected 方法作为调用链入口，
  一次 BFS 得出「从这些公开 API 可达」的全部缺口：

      python3 scripts/gap_scan.py api java/lang java/util java/text java/time java/io

  （`rava build --api-package … --precheck-only`：Rust 侧枚举入口，边界域类跳过）

  语料模式：全部（或过滤后的）e2e 测试逐例 BFS（`main.py --precheck-only`，可并行），
  汇总每个缺口被多少个测试触达，按频次排序：

      python3 scripts/gap_scan.py corpus -j 4
      python3 scripts/gap_scan.py corpus --filter Calendar Date

报告写到 docs/reports/gap-scan-<模式>.md，两类缺口：
  native-missing  调用链上的 native 方法，生成体为 `panic!("native: …")`（缺手写实现）
  boundary-stub   调用链上 / 触达的方法，生成体为 `panic!("stub: …")`（缺手写或未补译）

口径是调用链可达（BFS 过近似），不等于运行期必然执行；手写代码内未声明 upcalls 的调用、
按运行时 locale / 平台才走到的分支不在扫描面内。
"""
from __future__ import annotations

import argparse
import concurrent.futures as cf
import os
import re
import subprocess
import sys
import time
from collections import defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT / 'scripts'))

_PC_RE = re.compile(r'^\[precheck\] (native-missing|boundary-stub): (\S+)$')
_KINDS = ('native-missing', 'boundary-stub')


def _write_report(path: Path, title: str, meta: list[str],
                  hits: dict[str, dict[str, set]], total: int | None) -> None:
    """hits[kind][member] = {触达来源（测试名 / 'api'）}。"""
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, 'w', encoding='utf-8') as f:
        f.write(f"# {title}\n\n")
        for line in meta:
            f.write(f"- {line}\n")
        f.write("\n")
        for kind in _KINDS:
            items = hits.get(kind, {})
            f.write(f"## {kind}（{len(items)}）\n\n")
            if not items:
                f.write("无\n\n")
                continue
            by_cls: dict[str, list] = defaultdict(list)
            for member, srcs in items.items():
                by_cls[member.split(':', 1)[0].rsplit('.', 1)[0]].append((member, srcs))
            # 按类汇总：类内成员按触达数降序；类按总触达数降序
            order = sorted(by_cls.items(),
                           key=lambda kv: (-max(len(s) for _, s in kv[1]), kv[0]))
            if total is not None:
                f.write("| 类 | 成员 | 触达测试数 | 示例 |\n|----|------|-----:|------|\n")
            else:
                f.write("| 类 | 成员 |\n|----|------|\n")
            for cls, members in order:
                for member, srcs in sorted(members, key=lambda x: (-len(x[1]), x[0])):
                    short = member[len(cls) + 1:] if member.startswith(cls + '.') else member
                    if total is not None:
                        ex = ', '.join(sorted(srcs)[:3])
                        f.write(f"| `{cls}` | `{short}` | {len(srcs)} | {ex} |\n")
                    else:
                        f.write(f"| `{cls}` | `{short}` |\n")
            f.write("\n")
    print(f"报告 → {path.relative_to(ROOT)}")


# ── 语料模式 ───────────────────────────────────────────────────────────────────

def _precheck_one(java_file: Path, out_root: Path, jdk: str | None) -> tuple[str, dict, str]:
    name = java_file.stem
    args = [sys.executable, str(ROOT / 'scripts' / 'main.py'), str(java_file),
            '--precheck-only', '--out', str(out_root / name)]
    if jdk:
        args += ['--jdk', jdk]
    r = subprocess.run(args, cwd=ROOT, capture_output=True, text=True, timeout=1800)
    found: dict[str, set] = {k: set() for k in _KINDS}
    for line in r.stdout.splitlines():
        m = _PC_RE.match(line)
        if m and not m.group(2).startswith('…'):
            found[m.group(1)].add(m.group(2))
    err = '' if r.returncode == 0 else (r.stdout + r.stderr)[-400:]
    return name, found, err


def run_corpus(ns) -> None:
    files = sorted((ROOT / 'tests' / 'e2e').glob('*/*.java'))
    if ns.filter:
        files = [f for f in files if any(p in f.stem for p in ns.filter)]
    out_root = ROOT / 'build' / 'gap_scan'
    hits: dict[str, dict[str, set]] = {k: defaultdict(set) for k in _KINDS}
    errors: list[str] = []
    t0 = time.perf_counter()
    print(f"语料模式：{len(files)} 个测试，并行 {ns.jobs}")
    with cf.ThreadPoolExecutor(max_workers=ns.jobs) as ex:
        futs = {ex.submit(_precheck_one, f, out_root, ns.jdk): f for f in files}
        for i, fut in enumerate(cf.as_completed(futs), 1):
            name, found, err = fut.result()
            for kind, members in found.items():
                for m in members:
                    hits[kind][m].add(name)
            if err:
                errors.append(f"{name}: {err.strip().splitlines()[-1] if err.strip() else '?'}")
            n_nat, n_stub = len(found['native-missing']), len(found['boundary-stub'])
            print(f"[{i:>4}/{len(files)}] {name:<40} native-missing={n_nat} boundary-stub={n_stub}"
                  + ('  (失败)' if err else ''), flush=True)
    dt = time.perf_counter() - t0
    meta = [f"测试数 {len(files)}，耗时 {dt / 60:.1f} 分钟，并行 {ns.jobs}",
            f"native-missing 去重 {len(hits['native-missing'])} 个，boundary-stub 去重 {len(hits['boundary-stub'])} 个",
            f"BFS 失败 {len(errors)} 个" + (f"：{'; '.join(errors[:10])}" if errors else '')]
    suffix = '-'.join(ns.filter) if ns.filter else 'all'
    _write_report(ROOT / 'docs' / 'reports' / f'gap-scan-corpus-{suffix}.md',
                  '编译前缺口扫描（语料模式）', meta, hits, total=len(files))


# ── API 模式 ───────────────────────────────────────────────────────────────────

_API_RE = re.compile(r'^\[api\] .*→ (\d+) 个 public 类，(\d+) 个入口方法$')


def run_api(ns) -> None:
    from jdk_select import apply_jdk
    from rava_cli import rava_cmd
    _, home = apply_jdk(int(ns.jdk) if ns.jdk else None, quiet=True)
    scan_dir = ROOT / 'build' / 'gap_scan'
    entry_dir = scan_dir / 'entry'
    entry_dir.mkdir(parents=True, exist_ok=True)
    entry = entry_dir / 'GapScanEntry.java'
    entry.write_text('public class GapScanEntry { public static void main(String[] a) {} }\n')
    out_dir = scan_dir / ('api-' + '-'.join(p.replace('/', '.') for p in ns.packages))
    cmd = rava_cmd('build', str(entry), '--java-home', str(home),
                   '--runtime', str(ROOT / 'runtime' / 'java_runtime'),
                   '--out', str(out_dir), '--clean', '--precheck-only')
    for p in ns.packages:
        cmd += ['--api-package', p]
    if ns.recursive:
        cmd.append('--api-recursive')
    t0 = time.perf_counter()
    r = subprocess.run(cmd, cwd=ROOT, capture_output=True, text=True)
    dt = time.perf_counter() - t0
    if r.returncode != 0:
        sys.exit(f"rava build 失败（{r.returncode}）：{(r.stdout + r.stderr)[-800:]}")
    hits: dict[str, dict[str, set]] = {k: {} for k in _KINDS}
    n_cls = n_seeds = 0
    for line in r.stdout.splitlines():
        if m := _API_RE.match(line):
            n_cls, n_seeds = int(m.group(1)), int(m.group(2))
            print(line)
        elif (m := _PC_RE.match(line)) and not m.group(2).startswith('…'):
            hits[m.group(1)][m.group(2)] = {'api'}
    meta = [f"入口包：{', '.join(ns.packages)}（{'含' if ns.recursive else '不含'}子包），"
            f"{n_cls} 个 public 类 / {n_seeds} 个 public·protected 方法",
            f"BFS 耗时 {dt / 60:.1f} 分钟",
            f"native-missing {len(hits['native-missing'])} 个，boundary-stub {len(hits['boundary-stub'])} 个"]
    suffix = '-'.join(p.replace('/', '.') for p in ns.packages)
    _write_report(ROOT / 'docs' / 'reports' / f'gap-scan-api-{suffix}.md',
                  '编译前缺口扫描（API 模式）', meta, hits, total=None)


def main() -> None:
    ap = argparse.ArgumentParser(description=__doc__.split('\n')[0],
                                 formatter_class=argparse.RawDescriptionHelpFormatter,
                                 epilog=__doc__)
    sub = ap.add_subparsers(dest='mode', required=True)
    a = sub.add_parser('api', help='公开 API 包为入口（与测试无关）')
    a.add_argument('packages', nargs='+', help='包（斜线形态，如 java/util）')
    a.add_argument('--recursive', action='store_true', help='包含子包')
    a.add_argument('--jdk', default=None)
    c = sub.add_parser('corpus', help='e2e 测试逐例 BFS 汇总')
    c.add_argument('-j', '--jobs', type=int, default=max(1, (os.cpu_count() or 2) // 1))
    c.add_argument('--filter', nargs='*', default=[], help='测试名子串过滤')
    c.add_argument('--jdk', default=None)
    ns = ap.parse_args()
    (run_api if ns.mode == 'api' else run_corpus)(ns)


if __name__ == '__main__':
    main()

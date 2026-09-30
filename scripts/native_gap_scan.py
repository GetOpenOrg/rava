#!/usr/bin/env python3
"""全语料 native / 边界方法缺口统计：tests/e2e 逐例 `rava closure`（每例约 1 秒），收集 kind 为
handwritten:native / handwritten:boundary 的方法，与 runtime/ 手写 fn 名交叉比对（含 `__impl_` / `core_`
前缀约定），输出「静态触达但无手写体」清单（按触达用例数排序）→ docs/reports/native-gap-scan.md。

前置：cd generator && cargo build --release -p driver。口径是静态闭包可达（过近似），运行期是否执行
需按 `rava closure <Test.java> --why <方法>` 逐条核对。单例闭包分析超过 120 秒记 timeout 跳过。"""
import json, os, re, subprocess, sys, collections, concurrent.futures as cf
ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
RAVA = ROOT + '/generator/target/release/rava'
OUT = os.path.join(ROOT, 'build', 'native_gap_scan')
os.makedirs(OUT, exist_ok=True)
tests = sorted(subprocess.check_output(['find', ROOT + '/tests/e2e', '-name', '*.java'], text=True).split())

def run(t):
    name = os.path.basename(t)[:-5]
    o = f'{OUT}/{name}.json'
    if not os.path.exists(o):
        try:
            subprocess.run([RAVA, "closure", t, "-o", o], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, timeout=120)
        except subprocess.TimeoutExpired:
            print("timeout", name, flush=True)
    return name, o

hits = collections.defaultdict(set)   # (kind, id) -> tests
with cf.ThreadPoolExecutor(4) as ex:
    for name, o in ex.map(run, tests):
        try:
            d = json.load(open(o))
        except Exception:
            continue
        for m in d['methods']:
            k = m['kind']
            if k.startswith('handwritten:native') or k.startswith('handwritten:boundary'):
                hits[(k, m['id'])].add(name)

# 手写 fn 名全集：按文件记录（文件相对路径 → fn 名集合）
fns = collections.defaultdict(set)
src = ROOT + '/runtime/java_runtime/src'
for dp, _, fs in os.walk(src):
    for f in fs:
        if f.endswith('.rs'):
            p = os.path.join(dp, f)
            rel = os.path.relpath(p, src)
            for n in re.findall(r'\bfn\s+(\w+)', open(p, encoding='utf-8').read()):
                fns[rel].add(n)

def snake(s):
    s = s.replace('$', '_')
    return re.sub(r'(?<=[a-z0-9])(?=[A-Z])', '_', s).lower()

def provided(mid):
    cls, rest = mid.split('.', 1) if '.' in mid.split(':')[0] else (mid, '')
    cls = mid[:mid.rindex('.', 0, mid.index(':'))]
    name = mid[len(cls) + 1:mid.index(':')]
    pkg, simple = (cls.rsplit('/', 1) + [''])[:2] if '/' in cls else ('', cls)
    base = snake(simple)
    cands = [f'{pkg}/{base}_impl.rs', f'{pkg}/{base}.rs', f'{pkg}/{base}__impl.rs', f'{pkg}/{base}_ext.rs']
    ident = name if name not in ('<init>',) else 'new'
    for c in cands:
        for fn in fns.get(c, ()):
            if any(fn == p + ident or fn.startswith(p + ident + '_') for p in ('', '__impl_', 'core_')):
                return True
    return False

rows = []
for (k, mid), ts in hits.items():
    if '<clinit>' in mid:
        continue
    if not provided(mid):
        rows.append((len(ts), k, mid, sorted(ts)[:3]))
rows.sort(key=lambda r: (-r[0], r[2]))
with open(os.path.join(ROOT, 'docs', 'reports', 'native-gap-scan.md'), 'w') as f:
    f.write(f'# 语料缺口（{len(tests)} 例）\n\n| 用例数 | kind | 方法 | 示例 |\n|---:|---|---|---|\n')
    for n, k, mid, ex in rows:
        f.write(f'| {n} | {k} | `{mid}` | {", ".join(ex)} |\n')
print(len(rows), 'missing;', 'native:', sum(1 for r in rows if 'native' in r[1]))

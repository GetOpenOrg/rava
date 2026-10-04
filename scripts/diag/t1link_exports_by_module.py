#!/usr/bin/env python3
"""T1 第 2 步诊断：门面 dylib 导出符号按 JDK 模块归类。
用法：t1link_exports_by_module.py <scratch> <dylib> <out_dir>（需 JAVA_HOME；依赖 t1link_module_edges.py 的映射）
"""
import os, re, subprocess, sys
from collections import Counter
here = os.path.dirname(os.path.abspath(__file__))
src = open(os.path.join(here, 't1link_module_edges.py')).read().split('def main')[0]
ns = {}
exec(src, ns)
scratch, dylib, out = sys.argv[1:4]
cls2mod, req, reach = ns['jdk_maps'](os.environ['JAVA_HOME'], out)
rt = os.path.join(scratch, 'java_runtime', 'src')
key2bin = {}
for d, _, fs in os.walk(rt):
    for fn in fs:
        if fn.endswith('.rs'):
            txt = open(os.path.join(d, fn), errors='replace').read()
            b = ns['BIN_RE'].findall(txt)
            if b:
                pkg = tuple(os.path.relpath(d, rt).split(os.sep)) if d != rt else ()
                key2bin[pkg + (fn[:-3],)] = b[0]
def mod_of(b):
    b = ns['host_class'](b)
    return cls2mod.get(b, cls2mod.get(b.split('$')[0], '<rava>'))
syms = subprocess.run(['nm', '-gU', dylib], capture_output=True, text=True).stdout.split('\n')
names = [l.split()[-1] for l in syms if l.strip()]
dem = subprocess.run(['c++filt'], input='\n'.join(n[1:] for n in names), capture_output=True, text=True).stdout.split('\n')
P = re.compile(r'java_runtime::((?:[a-z_][a-z0-9_]*::)+)')
c = Counter()
for raw, d in zip(names, dem):
    if raw.startswith('___rava_'):
        c['<__rava_ 实现层符号>'] += 1; continue
    if raw.startswith('___java_meta_'):
        c['<__java_meta_ 表>'] += 1; continue
    m = P.search(d)
    if not m:
        c['<非 java_runtime（std / 依赖）>'] += 1; continue
    segs = m.group(1).rstrip(':').split('::')
    b = None
    for i in range(len(segs), 0, -1):
        if tuple(segs[:i]) in key2bin:
            b = key2bin[tuple(segs[:i])]; break
    c[mod_of(b) if b else '<java_runtime 手写基础设施>'] += 1
tot = sum(c.values())
lines = [f'total {tot}'] + [f'{k:36s} {v:8d} {100*v/tot:5.1f}%' for k, v in c.most_common()]
open(os.path.join(out, 'exports_by_module.txt'), 'w').write('\n'.join(lines) + '\n')
print('\n'.join(lines))

#!/usr/bin/env python3
"""T1 第 2 步诊断：按 JDK 模块统计档案生成代码的跨模块引用边。

用法：t1link_module_edges.py <scratch> <out_dir> [--java-home DIR]

- 类 → 模块：`jimage list $JAVA_HOME/lib/modules`；模块依赖：`java --describe-module`（requires，
  java.base 隐含）。两者均从 JDK 动态取得。
- 生成文件 → 类：首个 `#[binary_name = ...]`；手写 `<x>_impl.rs` / `<x>_ext.rs` 归属同目录 `<x>.rs` 的类；
  其余手写基础设施记为 `<infra>`；不在 jimage 中的类（java_support 等 VM 支持类）记为 `<rava>`。
- 引用：源码中 `crate::` / `java_runtime::` 起头、落到已知类（结构体名或文件模块名）的路径。
- 边分类：same（同模块）、fwd（目标在源模块 requires 传递闭包内）、rev（源在目标闭包内，反向）、
  cross（互不可达）。结果写 <out_dir>/module_edges.txt 与 module_edges.json。
"""
import json, os, re, subprocess, sys
from collections import Counter, defaultdict

def jdk_maps(java_home, out):
    jl = os.path.join(out, 'jimage.txt')
    if not os.path.exists(jl):
        with open(jl, 'w') as f:
            subprocess.run([f'{java_home}/bin/jimage', 'list', f'{java_home}/lib/modules'], stdout=f, check=True)
    cls2mod, mod = {}, None
    for line in open(jl):
        if line.startswith('Module: '):
            mod = line.split()[1]
        elif line.strip().endswith('.class') and mod:
            cls2mod[line.strip()[:-6]] = mod
    mods = subprocess.run([f'{java_home}/bin/java', '--list-modules'], capture_output=True, text=True).stdout.split()
    mods = [m.split('@')[0] for m in mods]
    req = {}
    for m in mods:
        d = subprocess.run([f'{java_home}/bin/java', '--describe-module', m], capture_output=True, text=True).stdout
        rs = set()
        for l in d.splitlines():
            if l.startswith('requires '):
                rs.add([t for t in l.split()[1:] if t not in ('transitive', 'static', 'mandated')][0])
        if m != 'java.base':
            rs.add('java.base')
        req[m] = rs
    reach = {}
    def r(m):
        if m in reach:
            return reach[m]
        reach[m] = set()
        s = set()
        for x in req.get(m, ()):
            s.add(x); s |= r(x)
        reach[m] = s
        return s
    for m in mods:
        r(m)
    return cls2mod, req, reach

BIN_RE = re.compile(r'#\[binary_name\s*=\s*"([^"]+)"\]')
STRUCT_RE = re.compile(r'pub struct ([A-Za-z0-9_]+)')
PATH_RE = re.compile(r'\b(?:crate|java_runtime)::((?:r#)?[A-Za-z_][A-Za-z0-9_]*(?:::(?:r#)?[A-Za-z_][A-Za-z0-9_]*)*)')

USE_RE = re.compile(r'^\s*(?:pub\s+)?use\s+((?:crate|java_runtime)::.*?);\s*$')
STR_RE = re.compile(r'"(?:[^"\\]|\\.)*"')
CMT_RE = re.compile(r'//[^\n]*')
WORD_RE = re.compile(r'[A-Za-z_][A-Za-z0-9_]*')

def split_use(u):
    u = u.replace(' ', '')
    if '{' not in u:
        return [u]
    pre, rest = u.split('{', 1)
    return [pre + x for x in rest.rstrip('}').split(',') if x and x != 'self']

def host_class(b):
    return b.split('$$Lambda')[0].split('$Proxy')[0]

def main():
    scratch, out = sys.argv[1], sys.argv[2]
    java_home = os.environ.get('JAVA_HOME')
    if '--java-home' in sys.argv:
        java_home = sys.argv[sys.argv.index('--java-home') + 1]
    os.makedirs(out, exist_ok=True)
    cls2mod, req, reach = jdk_maps(java_home, out)

    def mod_of(b):
        b = host_class(b)
        if b in cls2mod:
            return cls2mod[b]
        outer = b.split('$')[0]
        return cls2mod.get(outer, '<rava>')

    rt = os.path.join(scratch, 'java_runtime', 'src')
    key2bin, file2bin = {}, {}
    for d, _, fs in os.walk(rt):
        for fn in fs:
            if not fn.endswith('.rs'):
                continue
            p = os.path.join(d, fn)
            txt = open(p, encoding='utf-8', errors='replace').read()
            pkg = tuple(os.path.relpath(d, rt).split(os.sep)) if d != rt else ()
            bins = BIN_RE.findall(txt)
            if not bins:
                continue
            file2bin[p] = bins[0]
            key2bin[pkg + (fn[:-3],)] = bins[0]
            # binary_name 与紧随的 pub struct 配对
            for m in BIN_RE.finditer(txt):
                s = STRUCT_RE.search(txt, m.end())
                if s:
                    key2bin.setdefault(pkg + (s.group(1),), m.group(1))

    def resolve(path):
        segs = [s[2:] if s.startswith('r#') else s for s in path.split('::')]
        for i in range(len(segs), 0, -1):
            k = tuple(segs[:i])
            if k in key2bin:
                return key2bin[k]
        return None

    def src_of(p, crate):
        if p in file2bin:
            return file2bin[p], 'generated' if crate != 'rt_hand' else 'generated'
        txt_bins = BIN_RE.findall(open(p, encoding='utf-8', errors='replace').read())
        if txt_bins:
            return txt_bins[0], 'generated'
        d, fn = os.path.split(p)
        stem = fn[:-3]
        for suf in ('_impl', '_ext'):
            if stem.endswith(suf):
                sib = os.path.join(d, stem[:-len(suf)] + '.rs')
                if sib in file2bin:
                    return file2bin[sib], 'handwritten'
        return None, 'infra'

    roots = [('java_runtime', rt)]
    for n in sorted(os.listdir(scratch)):
        if n.startswith('java_body_'):
            roots.append((n, os.path.join(scratch, n, 'src')))
    roots.append(('user', os.path.join(scratch, 'user', 'src')))

    edges = Counter()            # (kind, srcmod, dstmod, cls) -> count
    examples = defaultdict(list)
    per_mod_bytes = Counter()
    per_mod_classes = defaultdict(set)
    for crate, root in roots:
        for d, _, fs in os.walk(root):
            for fn in fs:
                if not fn.endswith('.rs'):
                    continue
                p = os.path.join(d, fn)
                txt = open(p, encoding='utf-8', errors='replace').read()
                if crate == 'user':
                    sb, kind, sm = None, 'user', '<user>'
                else:
                    sb, kind = src_of(p, crate)
                    sm = mod_of(sb) if sb else '<infra>'
                    if crate != 'java_runtime':
                        kind = 'body-' + kind
                    elif kind == 'generated':
                        kind = 'decl-generated'
                    per_mod_bytes[sm] += len(txt)
                    if sb:
                        per_mod_classes[sm].add(host_class(sb))
                seen = set()
                # 生成文件的 use 行有大量未用导入：只计正文中实际出现的名字与全路径（去字符串字面量与注释）
                imports, body_lines = {}, []
                for line in txt.splitlines():
                    um = USE_RE.match(line)
                    if um:
                        for part in split_use(um.group(1)):
                            b = resolve(part.split('::', 1)[1])
                            if b:
                                imports[part.split('::')[-1]] = b
                    else:
                        body_lines.append(line)
                body = STR_RE.sub('""', '\n'.join(body_lines))
                body = CMT_RE.sub('', body)
                words = set(WORD_RE.findall(body))
                refs = [b for n, b in imports.items() if n in words]
                refs += [resolve(m.group(1)) for m in PATH_RE.finditer(body)]
                for db in refs:
                    if not db:
                        continue
                    dm = mod_of(db)
                    if (db, sb) in seen:
                        continue
                    seen.add((db, sb))
                    if dm == sm:
                        cls = 'same'
                    elif sm in ('<user>',):
                        cls = 'user'
                    elif sm in ('<infra>', '<rava>') or dm in ('<rava>',):
                        cls = 'infra'
                    elif dm in reach.get(sm, ()):
                        cls = 'fwd'
                    elif sm in reach.get(dm, ()):
                        cls = 'rev'
                    else:
                        cls = 'cross'
                    edges[(kind, sm, dm, cls)] += 1
                    if cls in ('rev', 'cross', 'infra') and len(examples[(kind, sm, dm)]) < 6:
                        examples[(kind, sm, dm)].append(f'{os.path.relpath(p, scratch)} -> {db}')

    # 用户 main.rs 登记行按模块计数
    reg = Counter()
    mainp = os.path.join(scratch, 'user', 'src', 'main.rs')
    if os.path.exists(mainp):
        for line in open(mainp, encoding='utf-8'):
            ms = {mod_of(b) for b in (resolve(m.group(1)) for m in PATH_RE.finditer(line)) if b}
            for x in ms:
                reg[x] += 1

    lines = []
    lines.append('## 每模块类数 / 档案源码字节（decl + body，含手写）')
    for m, n in sorted(per_mod_bytes.items(), key=lambda x: -x[1]):
        lines.append(f'{m:28s} classes={len(per_mod_classes[m]):5d} bytes={n:12d}')
    agg = Counter()
    for (kind, sm, dm, cls), n in edges.items():
        agg[(kind, cls)] += n
    lines.append('\n## 边分类汇总（类级去重，按源文件）')
    for (kind, cls), n in sorted(agg.items()):
        lines.append(f'{kind:22s} {cls:6s} {n}')
    lines.append('\n## 非 same / fwd / user 边明细')
    for (kind, sm, dm, cls), n in sorted(edges.items(), key=lambda x: -x[1]):
        if cls in ('rev', 'cross', 'infra'):
            lines.append(f'[{cls}] {kind} {sm} -> {dm}: {n}')
            for e in examples[(kind, sm, dm)]:
                lines.append(f'      {e}')
    lines.append('\n## 跨模块 fwd 边（模块对）')
    fwd = Counter()
    for (kind, sm, dm, cls), n in edges.items():
        if cls == 'fwd':
            fwd[(sm, dm)] += n
    for (sm, dm), n in sorted(fwd.items(), key=lambda x: -x[1]):
        lines.append(f'{sm} -> {dm}: {n}')
    lines.append('\n## 用户 main.rs 引用行按目标模块')
    for m, n in reg.most_common():
        lines.append(f'{m:28s} {n}')
    open(os.path.join(out, 'module_edges.txt'), 'w').write('\n'.join(lines) + '\n')
    json.dump({'edges': [list(k) + [v] for k, v in edges.items()],
               'req': {k: sorted(v) for k, v in req.items()}},
              open(os.path.join(out, 'module_edges.json'), 'w'), indent=1)
    print('\n'.join(lines[:80]))

if __name__ == '__main__':
    main()

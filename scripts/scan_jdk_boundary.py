#!/usr/bin/env python3
"""
扫描整个 JDK 公开 API（java/ javax/）中：
  1. 本身是 native 的方法
  2. 调用了内部类（sun/ jdk/ com/sun/ com/oracle/）的方法

并按内部包依赖关系，将需要手写实现的内部类分阶段规划（拓扑排序）。

用法：
    python3 scripts/scan_jdk_boundary.py                   # 全量扫描 + 阶段分析
    python3 scripts/scan_jdk_boundary.py --pkg java/io java/lang
    python3 scripts/scan_jdk_boundary.py --no-phases       # 跳过阶段分析（更快）
    python3 scripts/scan_jdk_boundary.py --top 50
"""

import os
import struct
import subprocess
import sys
import zipfile
from collections import defaultdict, deque
from pathlib import Path

# ─── 常量池标签 ───────────────────────────────────────────────────────────────
_UTF8=1; _INTEGER=3; _FLOAT=4; _LONG=5; _DOUBLE=6; _CLASS=7; _STRING=8
_FIELDREF=9; _METHODREF=10; _IFACE_METHOD=11; _NAME_TYPE=12
_METHOD_HANDLE=15; _METHOD_TYPE=16; _DYNAMIC=17; _INVOKEDYNAMIC=18
_MODULE=19; _PACKAGE=20

_SZ = bytearray(256)
for _i in range(256): _SZ[_i] = 1
for _op in (0x10, 0x12, 0xA9, 0xBC): _SZ[_op] = 2
for _op in range(0x15, 0x1A): _SZ[_op] = 2
for _op in range(0x36, 0x3B): _SZ[_op] = 2
_SZ[0x84] = 3
for _op in (0x11, 0x13, 0x14): _SZ[_op] = 3
for _op in range(0x99, 0xA9): _SZ[_op] = 3
for _op in range(0xB2, 0xB9): _SZ[_op] = 3
for _op in (0xB9, 0xBA): _SZ[_op] = 5
for _op in (0xBB, 0xBD, 0xC0, 0xC1): _SZ[_op] = 3
_SZ[0xC4] = 0; _SZ[0xC5] = 4
for _op in (0xC6, 0xC7): _SZ[_op] = 3
for _op in (0xC8, 0xC9): _SZ[_op] = 5

_PUBLIC_PREFIXES   = ('java/', 'javax/')
_INTERNAL_PREFIXES = ('sun/', 'jdk/', 'com/sun/', 'com/oracle/')

# 内部包的可读别名（用于报告分组）
_PKG_LABELS = [
    ('jdk/internal/misc',           'jdk/internal/misc'),
    ('jdk/internal/reflect',        'jdk/internal/reflect'),
    ('jdk/internal/loader',         'jdk/internal/loader'),
    ('jdk/internal/vm',             'jdk/internal/vm'),
    ('jdk/internal/',               'jdk/internal (other)'),
    ('sun/misc',                    'sun/misc'),
    ('sun/reflect',                 'sun/reflect'),
    ('sun/security',                'sun/security'),
    ('sun/nio',                     'sun/nio'),
    ('sun/util',                    'sun/util'),
    ('sun/',                        'sun (other)'),
    ('com/sun/crypto',              'com/sun/crypto'),
    ('com/sun/security',            'com/sun/security'),
    ('com/sun/',                    'com/sun (other)'),
    ('com/oracle/',                 'com/oracle'),
]

def _pkg_label(cls: str) -> str:
    for prefix, label in _PKG_LABELS:
        if cls.startswith(prefix):
            return label
    return cls.rsplit('/', 1)[0] if '/' in cls else cls


# ─── .class 解析 ─────────────────────────────────────────────────────────────

class _R:
    __slots__ = ('d', 'p')
    def __init__(self, d): self.d = d; self.p = 0
    def u1(self):  v = self.d[self.p]; self.p += 1; return v
    def u2(self):  v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v
    def u4(self):  v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v
    def skip(self, n): self.p += n
    def read(self, n): v = self.d[self.p:self.p+n]; self.p += n; return v


def _parse_class(data: bytes):
    r = _R(data)
    if r.u4() != 0xCAFEBABE: raise ValueError('not a class file')
    r.skip(4)
    cnt = r.u2(); pool = [None] * cnt; i = 1
    while i < cnt:
        tag = r.u1()
        if   tag == _UTF8:         n = r.u2(); pool[i] = (_UTF8, r.read(n).decode('utf-8', errors='replace'))
        elif tag == _INTEGER:      pool[i] = (_INTEGER, r.u4())
        elif tag == _FLOAT:        pool[i] = (_FLOAT,); r.skip(4)
        elif tag in (_LONG, _DOUBLE): pool[i] = (tag,); r.skip(8); i += 1; pool[i] = None
        elif tag == _CLASS:        pool[i] = (_CLASS, r.u2())
        elif tag == _STRING:       pool[i] = (_STRING, r.u2())
        elif tag in (_FIELDREF, _METHODREF, _IFACE_METHOD): pool[i] = (tag, r.u2(), r.u2())
        elif tag == _NAME_TYPE:    pool[i] = (_NAME_TYPE, r.u2(), r.u2())
        elif tag == _METHOD_HANDLE:pool[i] = (_METHOD_HANDLE, r.u1(), r.u2())
        elif tag in (_METHOD_TYPE, _MODULE, _PACKAGE): pool[i] = (tag, r.u2())
        elif tag in (_DYNAMIC, _INVOKEDYNAMIC): pool[i] = (tag, r.u2(), r.u2())
        else: raise ValueError(f'unknown cp tag {tag}')
        i += 1
    r.skip(2)
    this_idx = r.u2(); r.skip(2)
    r.skip(r.u2() * 2)
    for _ in range(r.u2()):
        r.skip(6)
        for _ in range(r.u2()): r.skip(2); r.skip(r.u4())
    methods = []
    for _ in range(r.u2()):
        acc = r.u2()
        e = pool[r.u2()]; mname = e[1] if e and e[0] == _UTF8 else ''
        e = pool[r.u2()]; mdesc = e[1] if e and e[0] == _UTF8 else ''
        is_native = bool(acc & 0x0100)
        bc = None
        for _ in range(r.u2()):
            e = pool[r.u2()]; aname = e[1] if e and e[0] == _UTF8 else ''
            alen = r.u4()
            if aname == 'Code':
                r.skip(4); clen = r.u4(); bc = r.read(clen)
                r.skip(r.u2() * 8)
                for _ in range(r.u2()): r.skip(2); r.skip(r.u4())
            else:
                r.skip(alen)
        methods.append((mname, mdesc, bc, is_native))
    e = pool[pool[this_idx][1]]; cls_name = e[1] if e and e[0] == _UTF8 else ''
    return cls_name, pool, methods


def _utf8(pool, idx):
    e = pool[idx]; return e[1] if e and e[0] == _UTF8 else ''

def _cls_name(pool, idx):
    e = pool[idx]
    if not e or e[0] != _CLASS: return None
    n = _utf8(pool, e[1])
    return n if n and '[' not in n else None

def _mref(pool, idx):
    e = pool[idx]
    if not e or e[0] not in (_METHODREF, _IFACE_METHOD): return None
    cls = _cls_name(pool, e[1])
    if not cls: return None
    nat = pool[e[2]]
    if not nat or nat[0] != _NAME_TYPE: return None
    return cls, _utf8(pool, nat[1]), _utf8(pool, nat[2])


# ─── 字节码扫描 ───────────────────────────────────────────────────────────────

def _scan_internal_calls(bc: bytes, pool) -> list[tuple]:
    """返回 [(internal_cls, method_name, descriptor), ...] 公开API→内部类的调用"""
    results = []
    i, n = 0, len(bc)
    while i < n:
        op = bc[i]
        if op in (0xB6, 0xB7, 0xB8, 0xB9):
            ref = _mref(pool, struct.unpack_from('>H', bc, i+1)[0])
            if ref and ref[0].startswith(_INTERNAL_PREFIXES):
                results.append(ref)
            i += 5 if op == 0xB9 else 3
        elif op == 0xBB:
            cls = _cls_name(pool, struct.unpack_from('>H', bc, i+1)[0])
            if cls and cls.startswith(_INTERNAL_PREFIXES):
                results.append((cls, '<new>', ''))
            i += 3
        elif op in (0xB2, 0xB3, 0xB4, 0xB5):
            e = pool[struct.unpack_from('>H', bc, i+1)[0]]
            if e and e[0] == _FIELDREF:
                cls = _cls_name(pool, e[1])
                if cls and cls.startswith(_INTERNAL_PREFIXES):
                    results.append((cls, '<field>', ''))
            i += 3
        elif op == 0xAA:
            pad = (4 - ((i+1) % 4)) % 4; i += 1+pad+4
            lo = struct.unpack_from('>i', bc, i)[0]; i += 4
            hi = struct.unpack_from('>i', bc, i)[0]; i += 4
            i += (hi-lo+1)*4
        elif op == 0xAB:
            pad = (4 - ((i+1) % 4)) % 4; i += 1+pad+4
            np = struct.unpack_from('>I', bc, i)[0]; i += 4
            i += np*8
        elif op == 0xBA: i += 5
        elif op == 0xC4: i += 6 if bc[i+1] == 0x84 else 4
        else:
            sz = _SZ[op]; i += sz if sz else 1
    return results


def _scan_cls_internal_refs(bc: bytes, pool) -> set[str]:
    """扫描字节码中所有引用的内部类名（用于内部类间依赖图）"""
    refs = set()
    i, n = 0, len(bc)
    while i < n:
        op = bc[i]
        if op in (0xB6, 0xB7, 0xB8, 0xB9):
            try:
                ref = _mref(pool, struct.unpack_from('>H', bc, i+1)[0])
                if ref and ref[0].startswith(_INTERNAL_PREFIXES):
                    refs.add(ref[0])
            except Exception:
                pass
            i += 5 if op == 0xB9 else 3
        elif op in (0xBB, 0xBD, 0xC0, 0xC1):
            try:
                cls = _cls_name(pool, struct.unpack_from('>H', bc, i+1)[0])
                if cls and cls.startswith(_INTERNAL_PREFIXES):
                    refs.add(cls)
            except Exception:
                pass
            i += 3
        elif op in (0xB2, 0xB3, 0xB4, 0xB5):
            try:
                e = pool[struct.unpack_from('>H', bc, i+1)[0]]
                if e and e[0] == _FIELDREF:
                    cls = _cls_name(pool, e[1])
                    if cls and cls.startswith(_INTERNAL_PREFIXES):
                        refs.add(cls)
            except Exception:
                pass
            i += 3
        elif op == 0xAA:
            pad = (4 - ((i+1) % 4)) % 4; i += 1+pad+4
            lo = struct.unpack_from('>i', bc, i)[0]; i += 4
            hi = struct.unpack_from('>i', bc, i)[0]; i += 4
            i += (hi-lo+1)*4
        elif op == 0xAB:
            pad = (4 - ((i+1) % 4)) % 4; i += 1+pad+4
            np = struct.unpack_from('>I', bc, i)[0]; i += 4
            i += np*8
        elif op == 0xBA: i += 5
        elif op == 0xC4: i += 6 if bc[i+1] == 0x84 else 4
        else:
            sz = _SZ[op]; i += sz if sz else 1
    return refs


# ─── JDK 定位 ────────────────────────────────────────────────────────────────

def _find_jmods():
    try:
        out = subprocess.check_output(
            ['java', '-XshowSettings:all', '-version'],
            stderr=subprocess.STDOUT, timeout=10, text=True)
        for line in out.splitlines():
            if 'java.home' in line:
                home = line.split('=', 1)[1].strip()
                p = os.path.join(home, 'jmods')
                if os.path.isdir(p): return p
    except Exception:
        pass
    home = os.environ.get('JAVA_HOME', '')
    if home:
        p = os.path.join(home, 'jmods')
        if os.path.isdir(p): return p
    return None


# ─── 全量扫描：公开 API 边界方法 ─────────────────────────────────────────────

def scan_jdk(jmods_dir: str, pkg_filter: list[str] | None = None) -> dict:
    """
    扫描 JDK 所有公开 API 类（java/ javax/），返回：
      native_methods   : [(cls, name, desc)]
      boundary_methods : [(cls, name, desc, [(internal_cls, nm, desc), ...])]
    """
    native_methods   = []
    boundary_methods = []
    total_classes = 0
    total_methods = 0

    for jmod in sorted(Path(jmods_dir).glob('*.jmod')):
        try:
            zf = zipfile.ZipFile(jmod)
        except Exception:
            continue
        with zf:
            for entry in zf.namelist():
                if not (entry.startswith('classes/') and entry.endswith('.class')):
                    continue
                cls_path = entry[8:-6]
                if not cls_path.startswith(_PUBLIC_PREFIXES):
                    continue
                if pkg_filter and not any(cls_path.startswith(p) for p in pkg_filter):
                    continue
                try:
                    data = zf.read(entry)
                    cls_name, pool, methods = _parse_class(data)
                except Exception:
                    continue

                total_classes += 1
                for mname, mdesc, bc, is_native in methods:
                    if mname == '<clinit>':
                        continue
                    total_methods += 1
                    if is_native:
                        native_methods.append((cls_name, mname, mdesc))
                    elif bc:
                        calls = _scan_internal_calls(bc, pool)
                        if calls:
                            boundary_methods.append((cls_name, mname, mdesc, calls))

    return {
        'total_classes':   total_classes,
        'total_methods':   total_methods,
        'native_methods':  native_methods,
        'boundary_methods': boundary_methods,
    }


# ─── 阶段分析：内部类依赖图 + 方法分类 + 拓扑排序 ───────────────────────────

# 方法分类（每个内部类方法的可实现性）
# native      : ACC_NATIVE，无字节码，必须手写 Rust
# translatable: 有字节码，且不调用任何内部类 → 可直接从字节码翻译
# has_seed_dep: 有字节码，调用 seed_internals 内其他内部类 → 翻译时需先实现依赖
# has_ext_dep : 有字节码，调用 seed_internals 以外的内部类 → 存在更深层传递依赖
# abstract    : 无字节码且非 native（接口/抽象方法）→ 由子类实现，本身无需手写
_M_NATIVE       = 'native'
_M_TRANSLATABLE = 'translatable'
_M_HAS_SEED_DEP = 'has_seed_dep'
_M_HAS_EXT_DEP  = 'has_ext_dep'
_M_ABSTRACT     = 'abstract'


def scan_internal_classes(
    jmods_dir: str,
    seed_internals: set[str],
) -> tuple[dict[str, set[str]], dict[str, dict]]:
    """
    一次扫描 seed_internals 中每个内部类，同时返回：

    dep_graph:    {cls → set[cls_it_depends_on (within seed_internals)]}
    method_stats: {cls → {'native':int, 'translatable':int, 'has_seed_dep':int,
                           'has_ext_dep':int, 'abstract':int, 'total':int}}
    """
    cls_to_pool_methods: dict[str, tuple] = {}
    for jmod in sorted(Path(jmods_dir).glob('*.jmod')):
        try:
            zf = zipfile.ZipFile(jmod)
        except Exception:
            continue
        with zf:
            for entry in zf.namelist():
                if not (entry.startswith('classes/') and entry.endswith('.class')):
                    continue
                cls_path = entry[8:-6]
                if cls_path not in seed_internals:
                    continue
                try:
                    data = zf.read(entry)
                    cls_name, pool, methods = _parse_class(data)
                    cls_to_pool_methods[cls_name] = (pool, methods)
                except Exception:
                    continue

    dep_graph:    dict[str, set[str]] = {}
    method_stats: dict[str, dict]     = {}

    for cls in seed_internals:
        stats = {_M_NATIVE: 0, _M_TRANSLATABLE: 0,
                 _M_HAS_SEED_DEP: 0, _M_HAS_EXT_DEP: 0,
                 _M_ABSTRACT: 0, 'total': 0}

        if cls not in cls_to_pool_methods:
            dep_graph[cls]    = set()
            method_stats[cls] = stats
            continue

        pool, methods = cls_to_pool_methods[cls]
        all_seed_refs: set[str] = set()

        for mname, mdesc, bc, is_native in methods:
            if mname == '<clinit>':
                continue
            stats['total'] += 1
            if is_native:
                stats[_M_NATIVE] += 1
            elif bc is None:
                stats[_M_ABSTRACT] += 1
            else:
                refs = _scan_cls_internal_refs(bc, pool)
                refs.discard(cls)
                seed_refs = refs & seed_internals
                ext_refs  = refs - seed_internals
                all_seed_refs |= seed_refs
                if seed_refs:
                    stats[_M_HAS_SEED_DEP] += 1
                elif ext_refs:
                    stats[_M_HAS_EXT_DEP] += 1
                else:
                    stats[_M_TRANSLATABLE] += 1

        dep_graph[cls]    = all_seed_refs
        method_stats[cls] = stats

    return dep_graph, method_stats


def topo_phases(dep_graph: dict[str, set[str]]) -> tuple[list[frozenset], frozenset]:
    """
    Kahn 算法拓扑排序，将内部类分层。
    返回 (phases_list, cycle_nodes)：
      phases_list[0] = 叶子层（无依赖），phases_list[1] = 只依赖叶子层，以此类推
      cycle_nodes    = 参与循环依赖的类（无法确定顺序，单独放最后）
    """
    in_degree: dict[str, int] = {cls: 0 for cls in dep_graph}
    reverse: dict[str, set[str]] = defaultdict(set)  # B → {A | A depends on B}
    for cls, deps in dep_graph.items():
        for dep in deps:
            in_degree[cls] += 1
            reverse[dep].add(cls)

    phases: list[frozenset] = []
    current_layer = frozenset(cls for cls, deg in in_degree.items() if deg == 0)

    while current_layer:
        phases.append(current_layer)
        next_layer = set()
        for cls in current_layer:
            for dependent in reverse.get(cls, set()):
                in_degree[dependent] -= 1
                if in_degree[dependent] == 0:
                    next_layer.add(dependent)
        current_layer = frozenset(next_layer)

    # 剩余 in_degree > 0 的类都在循环依赖中
    cycle_nodes = frozenset(cls for cls, deg in in_degree.items() if deg > 0)
    return phases, cycle_nodes


def compute_unlock(
    phases: list[frozenset],
    cycle_nodes: frozenset,
    boundary_methods: list,
) -> list[dict]:
    """
    计算每个阶段完成后新解锁的边界方法数。
    "解锁" = 该方法引用的所有内部类均已在本阶段及之前阶段实现完毕。

    返回 list of {
        'phase_idx': int,
        'classes': frozenset,
        'new_methods': [(pub_cls, name, desc)],  # 本阶段新解锁的方法
        'cumulative':  int,
    }
    """
    # 为每个内部类分配阶段编号
    class_phase: dict[str, int] = {}
    for idx, layer in enumerate(phases):
        for cls in layer:
            class_phase[cls] = idx
    for cls in cycle_nodes:
        class_phase[cls] = len(phases)  # 循环依赖放最后

    total_phases = len(phases) + (1 if cycle_nodes else 0)

    # 对每个边界方法，计算它在哪个阶段被解锁
    # 解锁阶段 = max(phase of each internal class it calls)
    unlock_at: dict[int, list] = defaultdict(list)
    for pub_cls, name, desc, calls in boundary_methods:
        internal_classes = {ic for ic, _, _ in calls}
        if not internal_classes:
            continue
        # 找出最大阶段（决定该方法何时全部解锁）
        max_phase = max(class_phase.get(ic, total_phases) for ic in internal_classes)
        unlock_at[max_phase].append((pub_cls, name, desc))

    result = []
    cumulative = 0
    for idx, layer in enumerate(phases):
        new_methods = unlock_at.get(idx, [])
        cumulative += len(new_methods)
        result.append({
            'phase_idx': idx,
            'classes':   layer,
            'new_methods': new_methods,
            'cumulative': cumulative,
        })
    if cycle_nodes:
        new_methods = unlock_at.get(len(phases), [])
        cumulative += len(new_methods)
        result.append({
            'phase_idx':   len(phases),
            'classes':     cycle_nodes,
            'new_methods': new_methods,
            'cumulative':  cumulative,
            'is_cycle':    True,
        })
    return result


# ─── 主程序 ──────────────────────────────────────────────────────────────────

def main():
    import argparse
    from datetime import date

    parser = argparse.ArgumentParser(
        description='扫描 JDK 公开 API 边界方法，并按内部包依赖层次规划实现阶段')
    parser.add_argument('--pkg', nargs='+', metavar='PREFIX',
                        help='只扫描指定包前缀（如 java/io java/lang）')
    parser.add_argument('--top', type=int, default=30,
                        help='Top N 类（默认 30）')
    parser.add_argument('--no-phases', action='store_true',
                        help='跳过阶段分析（更快）')
    args = parser.parse_args()

    jmods = _find_jmods()
    if not jmods:
        sys.exit('找不到 jmods 目录，请设置 JAVA_HOME')
    print(f'JDK jmods: {jmods}')

    pkg_desc = ' '.join(args.pkg) if args.pkg else 'java/ + javax/ 全部'
    print(f'扫描范围: {pkg_desc}')
    print('[1/2] 扫描公开 API 边界方法...', end=' ', flush=True)

    result = scan_jdk(jmods, pkg_filter=args.pkg)
    print('完成')

    native   = result['native_methods']
    boundary = result['boundary_methods']
    total_cls  = result['total_classes']
    total_mth  = result['total_methods']

    native_set   = set((c, n, d) for c, n, d in native)
    boundary_set = set((c, n, d) for c, n, d, _ in boundary)
    all_need_impl = native_set | boundary_set

    # seed_internals：所有被边界方法调用的内部类
    seed_internals: set[str] = set()
    for _, _, _, calls in boundary:
        for ic, _, _ in calls:
            seed_internals.add(ic)

    # 按类分组统计（边界方法）
    by_cls_native:   dict[str, list] = defaultdict(list)
    by_cls_boundary: dict[str, list] = defaultdict(list)
    for c, n, d in native:
        by_cls_native[c].append(f'{n}{d}')
    for c, n, d, calls in boundary:
        internals = sorted({ic for ic, _, _ in calls})
        by_cls_boundary[c].append((f'{n}{d}', internals))

    cls_boundary_count: dict[str, int] = defaultdict(int)
    for c, n, d, _ in boundary:
        cls_boundary_count[c] += 1

    # ── 控制台摘要 ────────────────────────────────────────────────────────────
    print()
    print('─' * 65)
    print(f'  JDK 公开 API 扫描结果（{pkg_desc}）')
    print('─' * 65)
    print(f'  扫描类数              : {total_cls:>7,}')
    print(f'  扫描方法数（含构造）  : {total_mth:>7,}')
    print(f'  ── 分类 ──')
    print(f'  native 方法           : {len(native):>7,}  ({len(native)/total_mth*100:.1f}%)')
    print(f'  边界方法（调内部类）  : {len(boundary):>7,}  ({len(boundary)/total_mth*100:.1f}%)')
    print(f'  两类合集（需手写）    : {len(all_need_impl):>7,}  ({len(all_need_impl)/total_mth*100:.1f}%)')
    print(f'  纯可翻译方法          : {total_mth-len(all_need_impl):>7,}')
    print(f'  ── 内部类 ──')
    print(f'  被引用的内部类数      : {len(seed_internals):>7,}')
    print('─' * 65)

    top_n = args.top
    top_cls = sorted(cls_boundary_count, key=lambda c: cls_boundary_count[c], reverse=True)[:top_n]
    print(f'\n【边界方法最多的 Top {top_n} 公开 API 类】')
    print(f'  {"边界方法":>8}  {"native":>6}  类名')
    print(f'  {"─"*8}  {"─"*6}  {"─"*50}')
    for cls in top_cls:
        nc = len(by_cls_native.get(cls, []))
        bc_ = cls_boundary_count[cls]
        print(f'  {bc_:>8}  {nc:>6}  {cls}')

    # ── 内部包按调用频次排名 ─────────────────────────────────────────────────
    internal_call_count: dict[str, int] = defaultdict(int)
    for _, _, _, calls in boundary:
        for ic, _, _ in calls:
            internal_call_count[ic] += 1

    pkg_call_count: dict[str, int] = defaultdict(int)
    for ic, cnt in internal_call_count.items():
        pkg_call_count[_pkg_label(ic)] += cnt

    print(f'\n【被公开 API 引用最多的内部包（前 20 个）】')
    print(f'  {"引用次数":>8}  {"内部类数":>8}  包')
    print(f'  {"─"*8}  {"─"*8}  {"─"*50}')
    pkg_cls_count: dict[str, set] = defaultdict(set)
    for ic in seed_internals:
        pkg_cls_count[_pkg_label(ic)].add(ic)
    for pkg in sorted(pkg_call_count, key=lambda p: pkg_call_count[p], reverse=True)[:20]:
        print(f'  {pkg_call_count[pkg]:>8}  {len(pkg_cls_count[pkg]):>8}  {pkg}')

    # ── 阶段分析 ─────────────────────────────────────────────────────────────
    phases_result = None
    dep_graph     = None
    cycle_nodes   = frozenset()

    method_stats: dict[str, dict] = {}

    if not args.no_phases:
        print(f'\n[2/2] 扫描内部类方法（{len(seed_internals)} 个类）...', end=' ', flush=True)
        dep_graph, method_stats = scan_internal_classes(jmods, seed_internals)
        phases_list, cycle_nodes = topo_phases(dep_graph)
        phases_result = compute_unlock(phases_list, cycle_nodes, boundary)
        print('完成')

        # ── 内部类方法分类汇总 ──────────────────────────────────────────────
        sum_native     = sum(s[_M_NATIVE]       for s in method_stats.values())
        sum_transl     = sum(s[_M_TRANSLATABLE]  for s in method_stats.values())
        sum_seed_dep   = sum(s[_M_HAS_SEED_DEP]  for s in method_stats.values())
        sum_ext_dep    = sum(s[_M_HAS_EXT_DEP]   for s in method_stats.values())
        sum_abstract   = sum(s[_M_ABSTRACT]       for s in method_stats.values())
        sum_total      = sum(s['total']            for s in method_stats.values())

        def _pct(n): return f'{n/sum_total*100:.1f}%' if sum_total else '0%'

        print(f'\n{"═"*70}')
        print(f'  654 个内部类的方法可实现性分析（共 {sum_total:,} 个方法）')
        print(f'{"═"*70}')
        print(f'  类型            数量      占比   说明')
        print(f'  {"─"*66}')
        print(f'  ACC_NATIVE    {sum_native:>7,}  {_pct(sum_native):>6}  无字节码，必须手写 Rust')
        print(f'  纯字节码      {sum_transl:>7,}  {_pct(sum_transl):>6}  无内部依赖，可直接从字节码翻译')
        print(f'  有内部依赖    {sum_seed_dep:>7,}  {_pct(sum_seed_dep):>6}  调用其他内部类，需先实现依赖')
        print(f'  外部内部依赖  {sum_ext_dep:>7,}  {_pct(sum_ext_dep):>6}  调用更深层内部类（未在本次分析范围）')
        print(f'  抽象/接口     {sum_abstract:>7,}  {_pct(sum_abstract):>6}  由子类实现，本身无需手写')
        print(f'  {"─"*66}')
        # "纯字节码"+"有内部依赖"合为可翻译（不需要手写）
        translatable = sum_transl + sum_seed_dep
        handwrite    = sum_native + sum_ext_dep
        print(f'  ▶ 可从字节码翻译  {translatable:>6,}  {_pct(translatable):>6}  (纯字节码 + 有内部依赖之和)')
        print(f'  ▶ 必须手写 Rust   {handwrite:>6,}  {_pct(handwrite):>6}  (ACC_NATIVE + 外部内部依赖之和)')
        print(f'{"═"*70}')

        # 哪些内部类全是 native？
        pure_native_cls = sorted(
            (cls for cls, s in method_stats.items()
             if s['total'] > 0 and s[_M_NATIVE] == s['total']),
            key=lambda c: -internal_call_count.get(c, 0))
        print(f'\n  全为 ACC_NATIVE 的内部类（共 {len(pure_native_cls)} 个，前 10）：')
        for cls in pure_native_cls[:10]:
            ref = internal_call_count.get(cls, 0)
            n   = method_stats[cls][_M_NATIVE]
            print(f'    {ref:>4} 引用  {n:>3} native 方法  {cls}')

        # 阶段规划表
        total_boundary = len(boundary)
        print(f'\n{"═"*65}')
        print(f'  内部包实现阶段规划  （共 {len(phases_list)} 层 + {"有" if cycle_nodes else "无"}循环依赖）')
        print(f'{"═"*65}')
        print(f'  {"阶段":>4}  {"类数":>6}  {"手写方法":>8}  {"可翻译":>8}  {"新解锁":>8}  {"进度":>6}')
        print(f'  {"─"*4}  {"─"*6}  {"─"*8}  {"─"*8}  {"─"*8}  {"─"*6}')
        for phase_info in phases_result:
            idx    = phase_info['phase_idx']
            clss   = phase_info['classes']
            ph_nat = sum(method_stats.get(c, {}).get(_M_NATIVE, 0) +
                         method_stats.get(c, {}).get(_M_HAS_EXT_DEP, 0) for c in clss)
            ph_tr  = sum(method_stats.get(c, {}).get(_M_TRANSLATABLE, 0) +
                         method_stats.get(c, {}).get(_M_HAS_SEED_DEP, 0) for c in clss)
            new_m  = len(phase_info['new_methods'])
            cum    = phase_info['cumulative']
            pct    = cum / total_boundary * 100 if total_boundary else 0
            label  = '(循环)' if phase_info.get('is_cycle') else ''
            print(f'  {idx+1:>4}  {len(clss):>6}  {ph_nat:>8}  {ph_tr:>8}  {new_m:>8}  {pct:>5.1f}% {label}')
        print(f'  {"─"*4}  {"─"*6}  {"─"*8}  {"─"*8}  {"─"*8}  {"─"*6}')

        # 每个阶段高价值内部类（带方法分类细节）
        print()
        for phase_info in phases_result[:4]:
            idx  = phase_info['phase_idx']
            clss = sorted(phase_info['classes'],
                          key=lambda c: -internal_call_count.get(c, 0))
            label = '(循环依赖)' if phase_info.get('is_cycle') else ''
            print(f'  阶段 {idx+1} {label}— 前 5 高价值内部类：')
            print(f'    {"引用":>5}  {"手写":>5}  {"翻译":>5}  {"抽象":>5}  类名')
            for cls in clss[:5]:
                ref  = internal_call_count.get(cls, 0)
                s    = method_stats.get(cls, {})
                hw   = s.get(_M_NATIVE, 0) + s.get(_M_HAS_EXT_DEP, 0)
                tr   = s.get(_M_TRANSLATABLE, 0) + s.get(_M_HAS_SEED_DEP, 0)
                ab   = s.get(_M_ABSTRACT, 0)
                print(f'    {ref:>5}  {hw:>5}  {tr:>5}  {ab:>5}  {cls}')
            print()

    # ── 写报告 ────────────────────────────────────────────────────────────────
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir  = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    suffix = '-' + args.pkg[0].replace('/', '_') if args.pkg and len(args.pkg) == 1 else ''
    report = os.path.join(reports_dir, f'jdk-boundary{suffix}.md')

    with open(report, 'w', encoding='utf-8') as f:
        f.write(f'# JDK 边界方法扫描报告\n\n生成时间：{date.today()}\n\n')
        f.write(f'扫描范围：`{pkg_desc}`\n\n')

        # 摘要
        f.write('## 摘要\n\n')
        f.write('| 指标 | 数量 | 占比 |\n|------|-----:|-----:|\n')
        f.write(f'| 扫描类数 | {total_cls:,} | 100% |\n')
        f.write(f'| 扫描方法数 | {total_mth:,} | 100% |\n')
        f.write(f'| native 方法 | {len(native):,} | {len(native)/total_mth*100:.1f}% |\n')
        f.write(f'| 边界方法（调内部类） | {len(boundary):,} | {len(boundary)/total_mth*100:.1f}% |\n')
        f.write(f'| **总需手写** | **{len(all_need_impl):,}** | **{len(all_need_impl)/total_mth*100:.1f}%** |\n')
        f.write(f'| 纯可翻译方法 | {total_mth-len(all_need_impl):,} | {(total_mth-len(all_need_impl))/total_mth*100:.1f}% |\n')
        f.write(f'| 被引用的内部类数 | {len(seed_internals):,} | — |\n\n')

        # 内部包分布
        f.write('## 被引用的内部包分布\n\n')
        f.write('| 包 | 引用次数 | 内部类数 |\n|-----|-----:|-----:|\n')
        for pkg in sorted(pkg_call_count, key=lambda p: pkg_call_count[p], reverse=True):
            f.write(f'| `{pkg}` | {pkg_call_count[pkg]:,} | {len(pkg_cls_count[pkg])} |\n')
        f.write('\n')

        # 内部类方法可实现性摘要
        if method_stats:
            sum_nat  = sum(s[_M_NATIVE]      for s in method_stats.values())
            sum_tr   = sum(s[_M_TRANSLATABLE] for s in method_stats.values())
            sum_sdep = sum(s[_M_HAS_SEED_DEP] for s in method_stats.values())
            sum_edep = sum(s[_M_HAS_EXT_DEP]  for s in method_stats.values())
            sum_ab   = sum(s[_M_ABSTRACT]      for s in method_stats.values())
            sum_tot  = sum(s['total']           for s in method_stats.values())
            def _fp(n): return f'{n/sum_tot*100:.1f}%' if sum_tot else '0%'

            f.write('## 内部类方法可实现性分析\n\n')
            f.write(f'共 {len(seed_internals)} 个内部类，{sum_tot:,} 个方法（不含 `<clinit>`）。\n\n')
            f.write('| 类型 | 数量 | 占比 | 说明 |\n|------|-----:|-----:|------|\n')
            f.write(f'| ACC_NATIVE | {sum_nat:,} | {_fp(sum_nat)} | 无字节码，**必须手写 Rust** |\n')
            f.write(f'| 纯字节码（无内部依赖） | {sum_tr:,} | {_fp(sum_tr)} | 可直接从字节码翻译 |\n')
            f.write(f'| 有内部依赖（seed 范围内） | {sum_sdep:,} | {_fp(sum_sdep)} | 翻译时需先实现其依赖 |\n')
            f.write(f'| 外部内部依赖（更深层） | {sum_edep:,} | {_fp(sum_edep)} | 调用未分析的内部类，需额外跟踪 |\n')
            f.write(f'| 抽象/接口方法 | {sum_ab:,} | {_fp(sum_ab)} | 由具体子类实现，本身无需手写 |\n')
            f.write(f'| **可从字节码翻译合计** | **{sum_tr+sum_sdep:,}** | **{_fp(sum_tr+sum_sdep)}** | |\n')
            f.write(f'| **必须手写合计** | **{sum_nat+sum_edep:,}** | **{_fp(sum_nat+sum_edep)}** | |\n\n')

            f.write('### 全为 ACC_NATIVE 的内部类\n\n')
            f.write('| 内部类 | 被引用次数 | native 方法数 |\n|--------|----------:|-------------:|\n')
            for cls in sorted(
                (c for c, s in method_stats.items()
                 if s['total'] > 0 and s[_M_NATIVE] == s['total']),
                key=lambda c: -internal_call_count.get(c, 0)
            ):
                f.write(f'| `{cls}` | {internal_call_count.get(cls,0)} | {method_stats[cls][_M_NATIVE]} |\n')
            f.write('\n')

        # 阶段规划
        if phases_result is not None:
            f.write('## 内部包实现阶段规划\n\n')
            f.write('> 拓扑排序结果。阶段 1 为叶子层（无内部依赖），可最先实现；\n')
            f.write('> "手写"= ACC_NATIVE + 外部内部依赖；"翻译"= 纯字节码 + seed 内依赖。\n\n')

            total_boundary = len(boundary)
            f.write('| 阶段 | 类数 | 手写方法 | 可翻译方法 | 新解锁 | 累计进度 |\n')
            f.write('|------|-----:|-------:|----------:|------:|--------:|\n')
            for phase_info in phases_result:
                idx    = phase_info['phase_idx']
                clss   = phase_info['classes']
                ph_nat = sum(method_stats.get(c, {}).get(_M_NATIVE, 0) +
                             method_stats.get(c, {}).get(_M_HAS_EXT_DEP, 0) for c in clss)
                ph_tr  = sum(method_stats.get(c, {}).get(_M_TRANSLATABLE, 0) +
                             method_stats.get(c, {}).get(_M_HAS_SEED_DEP, 0) for c in clss)
                new_m  = len(phase_info['new_methods'])
                cum    = phase_info['cumulative']
                pct    = cum / total_boundary * 100 if total_boundary else 0
                label  = ' (循环依赖)' if phase_info.get('is_cycle') else ''
                f.write(f'| 阶段 {idx+1}{label} | {len(clss)} | {ph_nat:,} | {ph_tr:,} | {new_m:,} | {pct:.1f}% |\n')
            f.write('\n')

            # 各阶段详情
            f.write('## 各阶段详情\n\n')
            for phase_info in phases_result:
                idx   = phase_info['phase_idx']
                clss  = sorted(phase_info['classes'],
                               key=lambda c: -internal_call_count.get(c, 0))
                label = '（循环依赖组）' if phase_info.get('is_cycle') else ''
                f.write(f'### 阶段 {idx+1} {label}\n\n')
                f.write(f'**{len(phase_info["classes"])} 个内部类，新解锁 {len(phase_info["new_methods"])} 个边界方法**\n\n')

                f.write('| 内部类 | 被引用 | 手写 | 翻译 | 抽象 | 内部依赖数 |\n')
                f.write('|--------|-------:|-----:|-----:|-----:|---------:|\n')
                for cls in clss:
                    ref  = internal_call_count.get(cls, 0)
                    s    = method_stats.get(cls, {})
                    hw   = s.get(_M_NATIVE, 0) + s.get(_M_HAS_EXT_DEP, 0)
                    tr   = s.get(_M_TRANSLATABLE, 0) + s.get(_M_HAS_SEED_DEP, 0)
                    ab   = s.get(_M_ABSTRACT, 0)
                    deps = len(dep_graph.get(cls, set())) if dep_graph else 0
                    f.write(f'| `{cls}` | {ref} | {hw} | {tr} | {ab} | {deps} |\n')
                f.write('\n')

                if phase_info['new_methods']:
                    unlocked_by_cls: dict[str, list] = defaultdict(list)
                    for pc, nm, desc in phase_info['new_methods']:
                        unlocked_by_cls[pc].append(f'{nm}{desc}')
                    f.write('**新解锁的边界方法（按公开 API 类）：**\n\n')
                    shown = 0
                    for pc in sorted(unlocked_by_cls):
                        if shown >= 120:
                            f.write(f'*（还有更多，省略）*\n\n')
                            break
                        f.write(f'`{pc}`：\n')
                        for sig in sorted(unlocked_by_cls[pc])[:10]:
                            f.write(f'- `{sig}`\n')
                        if len(unlocked_by_cls[pc]) > 10:
                            f.write(f'- *（+{len(unlocked_by_cls[pc])-10} 个）*\n')
                        f.write('\n')
                        shown += len(unlocked_by_cls[pc])

        # 边界方法详细列表（按公开API类）
        f.write('## 边界方法详细列表（按公开 API 类）\n\n')
        f.write('> 公开 API 中直接调用了内部类的方法，需要手写 native 实现。\n\n')
        for cls in sorted(by_cls_boundary):
            f.write(f'### `{cls}`\n\n')
            f.write('| 方法签名 | 调用的内部类 |\n|---------|------------|\n')
            for sig, internals in sorted(by_cls_boundary[cls]):
                int_str = ', '.join(f'`{c}`' for c in internals[:4])
                if len(internals) > 4: int_str += f', +{len(internals)-4}'
                f.write(f'| `{sig}` | {int_str} |\n')
            f.write('\n')

        # native 方法列表
        f.write('## native 方法列表（按公开 API 类）\n\n')
        for cls in sorted(by_cls_native):
            f.write(f'### `{cls}`\n\n')
            for sig in sorted(by_cls_native[cls]):
                f.write(f'- `{sig}`\n')
            f.write('\n')

    print(f'\n详细报告 → {report}')


if __name__ == '__main__':
    main()

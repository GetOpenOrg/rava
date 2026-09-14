#!/usr/bin/env python3
"""
方法级调用链追踪（独立脚本，不依赖项目代码）

从用户 Java 类出发，BFS 追踪实际被调用的方法链路，统计所有涉及的 JDK 类。
未被调用的方法不纳入分析，不展开其依赖。

用法:
    python3 scripts/trace_callchain.py tests/e2e/01_basics/HelloWorld.java
"""

import os
import struct
import subprocess
import sys
import zipfile
from collections import deque
from pathlib import Path

# ─── 常量池标签 ──────────────────────────────────────────────────────────────────
_UTF8          = 1
_INTEGER       = 3
_FLOAT         = 4
_LONG          = 5
_DOUBLE        = 6
_CLASS         = 7
_STRING        = 8
_FIELDREF      = 9
_METHODREF     = 10
_IFACE_METHOD  = 11
_NAME_TYPE     = 12
_METHOD_HANDLE = 15
_METHOD_TYPE   = 16
_DYNAMIC       = 17
_INVOKEDYNAMIC = 18
_MODULE        = 19
_PACKAGE       = 20

# ─── 操作码字节长度表（含操作码本身）────────────────────────────────────────────
_SZ = bytearray(256)
for _i in range(256):
    _SZ[_i] = 1                                    # 默认：无操作数指令

for _op in (0x10, 0x12, 0xA9, 0xBC):              # bipush, ldc, ret, newarray
    _SZ[_op] = 2
for _op in range(0x15, 0x1A):                      # iload..aload
    _SZ[_op] = 2
for _op in range(0x36, 0x3B):                      # istore..astore
    _SZ[_op] = 2
_SZ[0x84] = 3                                      # iinc
for _op in (0x11, 0x13, 0x14):                     # sipush, ldc_w, ldc2_w
    _SZ[_op] = 3
for _op in range(0x99, 0xA8 + 1):                  # ifeq..goto
    _SZ[_op] = 3
for _op in range(0xB2, 0xB9):                      # getstatic..invokestatic
    _SZ[_op] = 3
for _op in (0xB9, 0xBA):                           # invokeinterface, invokedynamic
    _SZ[_op] = 5
for _op in (0xBB, 0xBD, 0xC0, 0xC1):             # new, anewarray, checkcast, instanceof
    _SZ[_op] = 3
_SZ[0xC4] = 0                                      # wide: 特殊处理
_SZ[0xC5] = 4                                      # multianewarray
for _op in (0xC6, 0xC7):                           # ifnull, ifnonnull
    _SZ[_op] = 3
for _op in (0xC8, 0xC9):                           # goto_w, jsr_w
    _SZ[_op] = 5


# ─── .class 文件解析 ─────────────────────────────────────────────────────────────
class _R:
    """字节流读取器"""
    __slots__ = ('d', 'p')
    def __init__(self, d: bytes): self.d = d; self.p = 0
    def u1(self):  v = self.d[self.p]; self.p += 1; return v
    def u2(self):  v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v
    def u4(self):  v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v
    def skip(self, n): self.p += n
    def read(self, n): v = self.d[self.p:self.p+n]; self.p += n; return v


def _parse_class(data: bytes):
    """
    解析 .class 文件，返回 (class_name, pool, methods)
      pool    : list，1-indexed，每项 tuple 或 None
      methods : list of (name, descriptor, bytecode | None)
    """
    r = _R(data)
    if r.u4() != 0xCAFEBABE:
        raise ValueError('not a .class file')
    r.skip(4)  # minor + major version

    cnt = r.u2()
    pool = [None] * cnt
    i = 1
    while i < cnt:
        tag = r.u1()
        if tag == _UTF8:
            n = r.u2(); pool[i] = (_UTF8, r.read(n).decode('utf-8', errors='replace'))
        elif tag == _INTEGER:
            pool[i] = (_INTEGER, r.u4())
        elif tag in (_FLOAT,):
            pool[i] = (_FLOAT,); r.skip(4)
        elif tag in (_LONG, _DOUBLE):
            pool[i] = (tag,); r.skip(8); i += 1; pool[i] = None
        elif tag == _CLASS:
            pool[i] = (_CLASS, r.u2())
        elif tag == _STRING:
            pool[i] = (_STRING, r.u2())
        elif tag in (_FIELDREF, _METHODREF, _IFACE_METHOD):
            pool[i] = (tag, r.u2(), r.u2())      # class_idx, name_type_idx
        elif tag == _NAME_TYPE:
            pool[i] = (_NAME_TYPE, r.u2(), r.u2())
        elif tag == _METHOD_HANDLE:
            pool[i] = (_METHOD_HANDLE, r.u1(), r.u2())
        elif tag in (_METHOD_TYPE, _MODULE, _PACKAGE):
            pool[i] = (tag, r.u2())
        elif tag in (_DYNAMIC, _INVOKEDYNAMIC):
            pool[i] = (tag, r.u2(), r.u2())
        else:
            raise ValueError(f'unknown cp tag {tag} at pool[{i}]')
        i += 1

    r.skip(2)  # access_flags
    this_idx = r.u2()
    r.skip(2)  # super
    r.skip(r.u2() * 2)  # interfaces

    # 跳过字段表
    for _ in range(r.u2()):
        r.skip(6)
        for _ in range(r.u2()): r.skip(2); r.skip(r.u4())

    # 解析方法表
    methods = []
    for _ in range(r.u2()):
        r.skip(2)  # access_flags
        mname = pool[r.u2()][1]
        mdesc = pool[r.u2()][1]
        bytecode = None
        for _ in range(r.u2()):
            aname = pool[r.u2()][1]
            alen  = r.u4()
            if aname == 'Code':
                r.skip(4)               # max_stack, max_locals
                clen = r.u4()
                bytecode = r.read(clen)
                r.skip(r.u2() * 8)     # exception table
                for _ in range(r.u2()): r.skip(2); r.skip(r.u4())
            else:
                r.skip(alen)
        methods.append((mname, mdesc, bytecode))

    class_name = pool[pool[this_idx][1]][1]
    return class_name, pool, methods


def _utf8(pool, idx): e = pool[idx]; return e[1] if e and e[0] == _UTF8 else ''

def _cls_name(pool, cls_idx):
    e = pool[cls_idx]
    if not e or e[0] != _CLASS: return None
    n = _utf8(pool, e[1])
    return n if n and '[' not in n else None

def _mref(pool, ref_idx):
    """返回 (class, method_name, descriptor) 或 None"""
    e = pool[ref_idx]
    if not e or e[0] not in (_METHODREF, _IFACE_METHOD): return None
    cls = _cls_name(pool, e[1])
    if not cls: return None
    nat = pool[e[2]]
    if not nat or nat[0] != _NAME_TYPE: return None
    return cls, _utf8(pool, nat[1]), _utf8(pool, nat[2])

def _fref_cls(pool, ref_idx):
    e = pool[ref_idx]
    if not e or e[0] != _FIELDREF: return None
    return _cls_name(pool, e[1])


# ─── 字节码扫描 ──────────────────────────────────────────────────────────────────
def _scan(bytecode: bytes, pool):
    """
    扫描字节码，提取：
      calls : [(cls, name, desc)]  — invoke* 指令调用的方法
      refs  : [cls]                — new / field-access / checkcast 引用的类
    """
    calls, refs = [], []
    i, n = 0, len(bytecode)

    while i < n:
        op = bytecode[i]

        if op in (0xB6, 0xB7, 0xB8):          # invokevirtual / invokespecial / invokestatic
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref: calls.append(ref)
            i += 3

        elif op == 0xB9:                        # invokeinterface
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref: calls.append(ref)
            i += 5

        elif op == 0xBA:                        # invokedynamic（不展开）
            i += 5

        elif op == 0xBB:                        # new
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: refs.append(cls)
            i += 3

        elif op in (0xB2, 0xB3, 0xB4, 0xB5):  # getstatic / putstatic / getfield / putfield
            cls = _fref_cls(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: refs.append(cls)
            i += 3

        elif op in (0xBD, 0xC0, 0xC1):         # anewarray / checkcast / instanceof
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: refs.append(cls)
            i += 3

        elif op == 0xAA:                        # tableswitch
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4                    # skip pad + default
            lo = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            hi = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            i += (hi - lo + 1) * 4

        elif op == 0xAB:                        # lookupswitch
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4                    # skip pad + default
            np = struct.unpack_from('>I', bytecode, i)[0]; i += 4
            i += np * 8

        elif op == 0xC4:                        # wide
            sub = bytecode[i + 1]
            i += 6 if sub == 0x84 else 4        # wide iinc=6, 其余=4

        else:
            sz = _SZ[op]
            i += sz if sz else 1

    return calls, refs


# ─── JDK 类解析器 ────────────────────────────────────────────────────────────────
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


class JdkResolver:
    def __init__(self, jmods_dir: str):
        self._zips: dict[str, zipfile.ZipFile] = {}
        self._idx:  dict[str, str] = {}
        for jmod in Path(jmods_dir).glob('*.jmod'):
            try:
                zf = zipfile.ZipFile(jmod)
                for name in zf.namelist():
                    if name.startswith('classes/') and name.endswith('.class'):
                        self._idx[name[8:-6]] = str(jmod)   # strip "classes/" / ".class"
                self._zips[str(jmod)] = zf
            except Exception:
                pass

    def get(self, cls: str):
        jmod = self._idx.get(cls)
        if not jmod: return None
        try: return self._zips[jmod].read(f'classes/{cls}.class')
        except Exception: return None

    def close(self):
        for zf in self._zips.values():
            try: zf.close()
            except Exception: pass

    def __enter__(self): return self
    def __exit__(self, *_): self.close()


# ─── 目标范围 ────────────────────────────────────────────────────────────────────
_PREFIXES = ('java/', 'javax/', 'sun/', 'com/sun/', 'com/oracle/', 'jdk/')

def _in_scope(cls: str) -> bool:
    return cls.startswith(_PREFIXES)


# ─── 方法级 BFS ──────────────────────────────────────────────────────────────────
def bfs(user_class_files: list[str], resolver: JdkResolver):
    """
    返回 (visited_methods, all_classes)
      visited_methods : set[(cls, name, desc)]
      all_classes     : set[str]  (含 new/field 引用但无方法调用的类)
    """
    visited: set[tuple] = set()
    queue:   deque       = deque()
    refs:    set[str]    = set()   # new / field / checkcast 引用的类
    cache:   dict        = {}

    def enqueue(cls, name, desc):
        key = (cls, name, desc)
        if key not in visited:
            visited.add(key)
            queue.append(key)

    def get(cls):
        if cls in cache: return cache[cls]
        data = resolver.get(cls)
        if data is None: cache[cls] = None; return None
        try:
            result = _parse_class(data); cache[cls] = result; return result
        except Exception: cache[cls] = None; return None

    def process(bytecode, pool):
        calls, class_refs = _scan(bytecode, pool)
        for cls, nm, desc in calls:
            if _in_scope(cls): enqueue(cls, nm, desc)
        for cls in class_refs:
            if _in_scope(cls): refs.add(cls)

    # 种子：扫描所有用户 .class 文件的方法体
    for path in user_class_files:
        with open(path, 'rb') as f: data = f.read()
        _, pool, methods = _parse_class(data)
        for _, _, bc in methods:
            if bc: process(bc, pool)

    # BFS
    while queue:
        cls, name, desc = queue.popleft()
        parsed = get(cls)
        if parsed is None: continue
        _, pool, methods = parsed
        for mname, mdesc, bc in methods:
            if mname == name and mdesc == desc and bc:
                process(bc, pool)

    all_classes = {c for c, _, _ in visited} | refs
    return visited, all_classes


# ─── 主程序 ─────────────────────────────────────────────────────────────────────
def main():
    if len(sys.argv) < 2:
        sys.exit('用法: python3 scripts/trace_callchain.py <File.java> ...')

    java_files = sys.argv[1:]
    src_dir    = os.path.dirname(os.path.abspath(java_files[0]))
    class_dir  = os.path.join(src_dir, 'classes')
    os.makedirs(class_dir, exist_ok=True)

    print('[1/3] javac ...', end=' ', flush=True)
    r = subprocess.run(['javac', '-g', '-d', class_dir] + java_files,
                       capture_output=True, text=True)
    if r.returncode != 0:
        sys.exit(f'\njavac 失败:\n{r.stderr}')
    print('完成')

    user_classes = []
    for jf in java_files:
        stem = os.path.splitext(os.path.basename(jf))[0]
        p    = os.path.join(class_dir, stem + '.class')
        if os.path.exists(p): user_classes.append(p)

    print('[2/3] 定位 JDK jmods ...', end=' ', flush=True)
    jmods = _find_jmods()
    if not jmods:
        sys.exit('\n找不到 jmods 目录（请设置 JAVA_HOME）')
    print(jmods)

    print('[3/3] BFS 追踪调用链 ...', end=' ', flush=True)
    with JdkResolver(jmods) as resolver:
        methods, classes = bfs(user_classes, resolver)
    print('完成')

    # 按类分组
    by_cls: dict[str, list[str]] = {}
    for cls, nm, desc in methods:
        by_cls.setdefault(cls, []).append(f'{nm}{desc}')

    call_cls  = set(by_cls)
    ref_only  = classes - call_cls

    print()
    print('【摘要】')
    print(f'  调用链涉及类（有方法调用）    : {len(call_cls)}')
    print(f'  仅引用类（new/field，无方法调用）: {len(ref_only)}')
    print(f'  合计                           : {len(classes)}')
    print(f'  可达方法数                     : {len(methods)}')

    # 写报告
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir  = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0  = os.path.splitext(os.path.basename(java_files[0]))[0]
    report = os.path.join(reports_dir, f'trace-{stem0}.md')

    from datetime import date
    with open(report, 'w', encoding='utf-8') as f:
        f.write(f'# 调用链追踪：{stem0}\n\n生成时间：{date.today()}\n\n')
        f.write('## 摘要\n\n')
        f.write('| | 数量 |\n|---|---:|\n')
        f.write(f'| 调用链涉及类（有方法调用） | {len(call_cls)} |\n')
        f.write(f'| 仅引用类（new/field，无方法调用） | {len(ref_only)} |\n')
        f.write(f'| 合计 | {len(classes)} |\n')
        f.write(f'| 可达方法数 | {len(methods)} |\n\n')

        f.write('## 调用链方法（按类分组）\n\n')
        for cls in sorted(by_cls):
            f.write(f'### `{cls}`\n\n')
            for sig in sorted(by_cls[cls]):
                f.write(f'- `{sig}`\n')
            f.write('\n')

        if ref_only:
            f.write('## 仅引用类（new/field，无方法调用）\n\n')
            for cls in sorted(ref_only):
                f.write(f'- `{cls}`\n')

    print(f'\n详细报告 → {report}')


if __name__ == '__main__':
    main()

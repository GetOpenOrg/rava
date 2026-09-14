#!/usr/bin/env python3
"""
方法级调用链追踪（独立脚本，不依赖项目代码）

从用户 Java 类出发，BFS 追踪实际被调用的方法链路，统计所有涉及的 JDK 类。
包含三层优化：
  L1 native 边界  — native 方法无字节码，显式标记为边界，不尝试展开
  L2 <clinit> 隔离 — 静态初始化块只在类被 new 实例化后才跟随
  L3 RTA 虚调用  — 虚分发只解析到 instantiated 集合里的具体子类

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
    _SZ[_i] = 1

for _op in (0x10, 0x12, 0xA9, 0xBC):
    _SZ[_op] = 2
for _op in range(0x15, 0x1A):
    _SZ[_op] = 2
for _op in range(0x36, 0x3B):
    _SZ[_op] = 2
_SZ[0x84] = 3
for _op in (0x11, 0x13, 0x14):
    _SZ[_op] = 3
for _op in range(0x99, 0xA8 + 1):
    _SZ[_op] = 3
for _op in range(0xB2, 0xB9):
    _SZ[_op] = 3
for _op in (0xB9, 0xBA):
    _SZ[_op] = 5
for _op in (0xBB, 0xBD, 0xC0, 0xC1):
    _SZ[_op] = 3
_SZ[0xC4] = 0
_SZ[0xC5] = 4
for _op in (0xC6, 0xC7):
    _SZ[_op] = 3
for _op in (0xC8, 0xC9):
    _SZ[_op] = 5


# ─── .class 文件解析 ─────────────────────────────────────────────────────────────
class _R:
    __slots__ = ('d', 'p')
    def __init__(self, d: bytes): self.d = d; self.p = 0
    def u1(self):  v = self.d[self.p]; self.p += 1; return v
    def u2(self):  v = struct.unpack_from('>H', self.d, self.p)[0]; self.p += 2; return v
    def u4(self):  v = struct.unpack_from('>I', self.d, self.p)[0]; self.p += 4; return v
    def skip(self, n): self.p += n
    def read(self, n): v = self.d[self.p:self.p+n]; self.p += n; return v


def _parse_class(data: bytes):
    """
    解析 .class 文件，返回 (class_name, pool, methods, super_name, iface_names)
      methods      : list of (name, descriptor, bytecode | None, is_native)
      super_name   : 父类 binary name（java/lang/Object 返回 None）
      iface_names  : 实现的接口列表
    """
    r = _R(data)
    if r.u4() != 0xCAFEBABE:
        raise ValueError('not a .class file')
    r.skip(4)  # minor + major

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
            pool[i] = (tag, r.u2(), r.u2())
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

    r.skip(2)        # class access_flags（不需要 is_abstract，用 super=Object 判断）
    this_idx  = r.u2()
    super_idx = r.u2()
    ifc_cnt   = r.u2()
    ifc_idxs  = [r.u2() for _ in range(ifc_cnt)]

    # 跳过字段表
    for _ in range(r.u2()):
        r.skip(6)
        for _ in range(r.u2()): r.skip(2); r.skip(r.u4())

    # 解析方法表（含 is_native）
    methods = []
    for _ in range(r.u2()):
        m_acc  = r.u2()                          # L1: 读取 access_flags
        mname  = pool[r.u2()][1]
        mdesc  = pool[r.u2()][1]
        is_native = bool(m_acc & 0x0100)         # ACC_NATIVE
        bytecode  = None
        for _ in range(r.u2()):
            aname = pool[r.u2()][1]
            alen  = r.u4()
            if aname == 'Code':
                r.skip(4)
                clen = r.u4()
                bytecode = r.read(clen)
                r.skip(r.u2() * 8)
                for _ in range(r.u2()): r.skip(2); r.skip(r.u4())
            else:
                r.skip(alen)
        methods.append((mname, mdesc, bytecode, is_native))

    class_name = pool[pool[this_idx][1]][1]

    # L3: 提取父类和接口（用于 RTA 子类型判断）
    super_name = None
    if super_idx != 0:
        e = pool[super_idx]
        if e and e[0] == _CLASS:
            sn = _utf8(pool, e[1])
            if sn and sn != 'java/lang/Object':
                super_name = sn

    iface_names = []
    for idx in ifc_idxs:
        e = pool[idx]
        if e and e[0] == _CLASS:
            n = _utf8(pool, e[1])
            if n: iface_names.append(n)

    return class_name, pool, methods, super_name, iface_names


def _utf8(pool, idx): e = pool[idx]; return e[1] if e and e[0] == _UTF8 else ''

def _cls_name(pool, cls_idx):
    e = pool[cls_idx]
    if not e or e[0] != _CLASS: return None
    n = _utf8(pool, e[1])
    return n if n and '[' not in n else None

def _mref(pool, ref_idx):
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
    返回 (vcalls, dcalls, new_cls, other_refs)
      vcalls     : invokevirtual / invokeinterface → L3 RTA 分析目标
      dcalls     : invokespecial / invokestatic    → 直接跟随
      new_cls    : new 指令实例化的类              → L2 instantiated
      other_refs : field / anewarray / checkcast   → 仅记录引用
    """
    vcalls, dcalls, new_cls, other_refs = [], [], [], []
    i, n = 0, len(bytecode)

    while i < n:
        op = bytecode[i]

        if op in (0xB6, 0xB9):                    # invokevirtual, invokeinterface
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref: vcalls.append(ref)
            i += 3 if op == 0xB6 else 5

        elif op in (0xB7, 0xB8):                   # invokespecial, invokestatic
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref: dcalls.append(ref)
            i += 3

        elif op == 0xBA:                            # invokedynamic（跳过）
            i += 5

        elif op == 0xBB:                            # new → L2 instantiated
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: new_cls.append(cls)
            i += 3

        elif op in (0xB2, 0xB3, 0xB4, 0xB5):      # getstatic/putstatic/getfield/putfield
            cls = _fref_cls(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: other_refs.append(cls)
            i += 3

        elif op in (0xBD, 0xC0, 0xC1):             # anewarray / checkcast / instanceof
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: other_refs.append(cls)
            i += 3

        elif op == 0xAA:                            # tableswitch
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            lo = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            hi = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            i += (hi - lo + 1) * 4

        elif op == 0xAB:                            # lookupswitch
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            np = struct.unpack_from('>I', bytecode, i)[0]; i += 4
            i += np * 8

        elif op == 0xC4:                            # wide
            sub = bytecode[i + 1]
            i += 6 if sub == 0x84 else 4

        else:
            sz = _SZ[op]
            i += sz if sz else 1

    return vcalls, dcalls, new_cls, other_refs


# ─── JDK resolver ────────────────────────────────────────────────────────────────
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
                        self._idx[name[8:-6]] = str(jmod)
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


# ─── BFS（三层优化） ──────────────────────────────────────────────────────────────
def bfs(user_class_files: list[str], resolver: JdkResolver):
    """
    返回 (visited_methods, all_classes, native_stubs, instantiated)
      visited_methods : set[(cls, name, desc)]
      all_classes     : set[str]
      native_stubs    : set[(cls, name, desc)]  L1: native 方法调用
      instantiated    : set[str]                L2/L3: new 指令实例化的类
    """
    visited:     set[tuple] = set()
    queue:       deque       = deque()
    refs:        set[str]    = set()   # field / checkcast 引用
    native_stubs: set[tuple] = set()  # L1: native 方法边界
    instantiated: set[str]   = set()  # L2/L3: new 指令

    # L3 RTA: 类继承结构缓存
    hier_super:  dict[str, str | None] = {}
    hier_ifaces: dict[str, list[str]]  = {}

    cache: dict = {}

    def get(cls):
        if cls in cache: return cache[cls]
        data = resolver.get(cls)
        if data is None: cache[cls] = None; return None
        try:
            result = _parse_class(data)
            cache[cls] = result
            # L3: 记录继承结构
            _, _, _, sn, ifaces = result
            hier_super[cls]  = sn
            hier_ifaces[cls] = ifaces
            return result
        except Exception: cache[cls] = None; return None

    def is_subtype(sub: str, sup: str, _seen: frozenset = frozenset()) -> bool:
        """L3: 判断 sub 是否是 sup 的子类型（可传递）"""
        if sub == sup: return True
        if sub in _seen: return False
        seen2 = _seen | {sub}
        parent = hier_super.get(sub)
        if parent and is_subtype(parent, sup, seen2): return True
        for iface in hier_ifaces.get(sub, []):
            if is_subtype(iface, sup, seen2): return True
        return False

    def enqueue(cls, name, desc):
        # L2: <clinit> 只在类被 new 实例化后才跟随
        if name == '<clinit>' and cls not in instantiated:
            return
        key = (cls, name, desc)
        if key not in visited:
            visited.add(key)
            queue.append(key)

    def process(bytecode, pool):
        vcalls, dcalls, new_refs, other_refs = _scan(bytecode, pool)

        # L2: 记录 new 实例化
        for cls in new_refs:
            if _in_scope(cls):
                instantiated.add(cls)
                refs.add(cls)

        for cls in other_refs:
            if _in_scope(cls): refs.add(cls)

        # 直接调用（invokespecial / invokestatic）
        for cls, nm, desc in dcalls:
            if _in_scope(cls): enqueue(cls, nm, desc)

        # L3 RTA: 虚调用（invokevirtual / invokeinterface）
        for cls, nm, desc in vcalls:
            if not _in_scope(cls): continue

            # 在已知 instantiated 里找子类型
            # 需要先确保 cls 的继承结构已加载
            if cls not in hier_super:
                get(cls)  # 触发继承结构加载
            subtypes = [c for c in instantiated
                        if _in_scope(c) and is_subtype(c, cls)]
            if subtypes:
                for sub in subtypes:
                    enqueue(sub, nm, desc)
            else:
                # 保守回退：没有已知实例化子类，跟随声明类型
                enqueue(cls, nm, desc)

    # 种子：扫描用户 .class 文件所有方法
    for path in user_class_files:
        with open(path, 'rb') as f: data = f.read()
        _, pool, methods, _, _ = _parse_class(data)
        for _, _, bc, _ in methods:
            if bc: process(bc, pool)

    # BFS
    while queue:
        cls, name, desc = queue.popleft()
        parsed = get(cls)
        if parsed is None: continue
        _, pool, methods, _, _ = parsed
        for mname, mdesc, bc, is_native in methods:
            if mname == name and mdesc == desc:
                if is_native:
                    native_stubs.add((cls, name, desc))  # L1: native 边界，停止展开
                elif bc:
                    process(bc, pool)

    all_classes = {c for c, _, _ in visited} | refs
    return visited, all_classes, native_stubs, instantiated


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

    print('[3/3] BFS（L1 native + L2 <clinit> + L3 RTA）...', end=' ', flush=True)
    with JdkResolver(jmods) as resolver:
        methods, classes, native_stubs, instantiated = bfs(user_classes, resolver)
    print('完成')

    by_cls: dict[str, list[str]] = {}
    for cls, nm, desc in methods:
        by_cls.setdefault(cls, []).append(f'{nm}{desc}')

    call_cls = set(by_cls)
    ref_only = classes - call_cls
    native_cls = {c for c, _, _ in native_stubs}

    print()
    print('【结果（三层优化后）】')
    print(f'  调用链涉及类（有方法调用）      : {len(call_cls)}')
    print(f'  仅引用类（new/field，无调用）    : {len(ref_only)}')
    print(f'  合计                             : {len(classes)}')
    print(f'  可达方法数                       : {len(methods)}')
    print(f'  ─────────────────────────────────')
    print(f'  L1 native 边界方法               : {len(native_stubs)}（{len(native_cls)} 个类）')
    print(f'  L2 instantiated（new 实例化类）  : {len(instantiated)}')
    print(f'  L3 RTA 参与过滤的类              : {len(hier_super) if "hier_super" in dir() else "—"}')

    # 写报告
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir  = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0  = os.path.splitext(os.path.basename(java_files[0]))[0]
    report = os.path.join(reports_dir, f'trace-{stem0}.md')

    from datetime import date
    with open(report, 'w', encoding='utf-8') as f:
        f.write(f'# 调用链追踪（三层优化）：{stem0}\n\n生成时间：{date.today()}\n\n')
        f.write('## 摘要\n\n')
        f.write('| | 数量 |\n|---|---:|\n')
        f.write(f'| 调用链涉及类（有方法调用） | {len(call_cls)} |\n')
        f.write(f'| 仅引用类（new/field，无调用） | {len(ref_only)} |\n')
        f.write(f'| 合计 | {len(classes)} |\n')
        f.write(f'| 可达方法数 | {len(methods)} |\n')
        f.write(f'| native 边界方法数 | {len(native_stubs)} |\n')
        f.write(f'| instantiated 类数 | {len(instantiated)} |\n\n')
        f.write('## 调用链方法（按类分组）\n\n')
        for cls in sorted(by_cls):
            f.write(f'### `{cls}`\n\n')
            for sig in sorted(by_cls[cls]):
                f.write(f'- `{sig}`\n')
            f.write('\n')
        if native_stubs:
            f.write('## L1 Native 边界方法\n\n')
            for cls, nm, desc in sorted(native_stubs):
                f.write(f'- `{cls}.{nm}{desc}`\n')
        if ref_only:
            f.write('\n## 仅引用类\n\n')
            for cls in sorted(ref_only):
                f.write(f'- `{cls}`\n')
        if instantiated:
            f.write('\n## L2 Instantiated 类\n\n')
            for cls in sorted(instantiated):
                f.write(f'- `{cls}`\n')

    print(f'\n详细报告 → {report}')


if __name__ == '__main__':
    main()

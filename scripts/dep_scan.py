#!/usr/bin/env python3
"""依赖透视（dep_scan）——pilot 前置侦察：扫描 jar / 字节码 / 源码，回答「它依赖了谁」。

用法：
    python3 scripts/dep_scan.py <path>...           # .jar/.zip、.class、目录（递归找 .class）
    python3 scripts/dep_scan.py --java <srcdir>...   # 源码模式：只解析 package/import（近似）
    python3 scripts/dep_scan.py lib.jar --top 20     # 第三方分组只列前 20（按类数）
    python3 scripts/dep_scan.py lib.jar --json       # 机器可读输出
    python3 scripts/dep_scan.py lib.jar --classes    # 附类级清单（默认包级聚合）

分类口径（与 scan_jdk_boundary.py 对齐）：
  jdk-public    java/ javax/ + JDK 附带（org.w3c.dom / org.xml.sax / org.ietf.jgss）
  jdk-internal  jdk/ sun/ com/sun/ com/oracle/ —— 需伴生/手写排期的信号
  self          被扫描输入自身的包（.class 的 this_class / 源码 package 声明）
  third-party   其余（按顶层两段包名聚合，如 org.hamcrest）

为什么默认扫字节码而非源码：.class 常量池的 CONSTANT_Class 是完备的**直接**引用集
（超类型 / 字段 / 方法 / checkcast / new / ldc 一跳全覆盖）；源码 import 抓不到
FQCN 内联使用、隐式 java.lang、静态成员链。源码模式仅作无 .class 时的快览。

注意：输出是直接引用（一跳）；传递闭包是转译期 BFS 调用链的职责，两者互补——
本脚本的定位是 pilot 选型/排期的**事前**透视（内部类占比高 → 先排反射/伴生）。
"""

import argparse
import json
import re
import struct
import sys
import zipfile
from collections import defaultdict
from pathlib import Path

# ── 常量池标签（只需走池子收集 CONSTANT_Class，不解析属性） ──────────────
_UTF8, _CLASS = 1, 7

# 池中各 tag 的字节数（tag 之外的部分；5/6 占两个槽位在推进时处理）
_TAG_SIZE = {1: -1, 3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4,
             11: 4, 12: 4, 15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2}

_JDK_PUBLIC = ('java/', 'javax/', 'org/w3c/dom/', 'org/xml/sax/', 'org/ietf/jgss/')
_JDK_INTERNAL = ('jdk/', 'sun/', 'com/sun/', 'com/oracle/')
# 借 com.sun 命名空间的第三方（JNA / JavaMail 实现 / JAF 实现）——非 JDK 内部，
# 提前 carve-out 防 jdk-internal 误报
_COM_SUN_THIRDPARTY = ('com/sun/jna/', 'com/sun/mail/', 'com/sun/activation/')


def _mutf8(b: bytes) -> str:
    """常量池 UTF-8（MUTF-8）：包/类名实际均为 ASCII，失败时降级 replace。"""
    try:
        return b.decode('utf-8')
    except UnicodeDecodeError:
        return b.decode('utf-8', errors='replace')


def _normalize(name: str) -> str | None:
    """CONSTANT_Class 名规范化：数组描述符剥维数与 L...; 包装；基本类型返回 None。"""
    name = name.lstrip('[')
    if not name or name[0] in 'IJCZBSDFV':
        return None
    if name.startswith('L') and name.endswith(';'):
        name = name[1:-1]
    return name or None


def _scan_class_bytes(data: bytes, sink: set[str], self_names: set[str]) -> None:
    """单 .class：全量收集 CONSTANT_Class 引用（一跳）。"""
    if data[:4] != b'\xca\xfe\xba\xbe' or len(data) < 10:
        return
    off = 8
    count = struct.unpack_from('>H', data, off)[0]
    off += 2
    i = 1
    utf8: dict[int, str] = {}
    class_at: dict[int, int] = {}   # 池索引 → Class 条目的 name_index（this_class 用）
    while i < count:
        tag = data[off]
        off += 1
        if tag == _UTF8:
            n = struct.unpack_from('>H', data, off)[0]
            utf8[i] = _mutf8(data[off + 2:off + 2 + n])
            off += 2 + n
        elif tag == _CLASS:
            class_at[i] = struct.unpack_from('>H', data, off)[0]
            off += 2
        elif tag in (5, 6):
            off += 8
            i += 1
        else:
            off += _TAG_SIZE.get(tag, 0)
        i += 1
    access, this_class = struct.unpack_from('>HH', data, off)
    own_idx = class_at.get(this_class)
    if own_idx is not None and own_idx in utf8:
        self_names.add(utf8[own_idx])
    for name_index in class_at.values():
        name = _normalize(utf8.get(name_index, ''))
        if name:
            sink.add(name)


def _iter_classfiles(paths: list[Path]):
    """展开输入：.jar/.zip → 逐条目；.class → 单文件；目录 → 递归收集。"""
    for p in paths:
        if p.suffix.lower() in ('.jar', '.zip', '.war'):
            with zipfile.ZipFile(p) as zf:
                for entry in zf.namelist():
                    if entry.endswith('.class') and not entry.endswith('module-info.class'):
                        yield p, entry.split('/')[-1], zf.read(entry)
        elif p.is_file() and p.suffix == '.class':
            yield p, p.name, p.read_bytes()
        elif p.is_dir():
            for f in sorted(p.rglob('*.class')):
                if f.name != 'module-info.class':
                    yield f, f.name, f.read_bytes()


# ── 源码模式（近似） ────────────────────────────────────────────────────
_PKG_RE = re.compile(r'^\s*package\s+([\w.]+)\s*;', re.M)
_IMP_RE = re.compile(r'^\s*import\s+(?:static\s+)?([\w.]+)\s*;', re.M)


def _scan_java_sources(paths: list[Path]) -> tuple[set[str], set[str]]:
    """扫 .java：返回 (依赖类集, 自身类集)。import 的通配尾段（.*）记为包。"""
    deps: set[str] = set()
    self_names: set[str] = set()
    files: list[Path] = []
    for p in paths:
        if p.is_dir():
            files += sorted(p.rglob('*.java'))
        elif p.suffix == '.java':
            files.append(p)
    if not files:
        sys.exit('源码模式下未找到 .java 文件')
    for f in files:
        text = f.read_text(encoding='utf-8', errors='replace')
        m = _PKG_RE.search(text)
        if m:
            self_names.add(m.group(1).replace('.', '/'))
        for im in _IMP_RE.finditer(text):
            seg = im.group(1).replace('.', '/')
            deps.add(seg if seg.endswith('/*') else seg)
    return deps, self_names


# ── 分类与输出 ──────────────────────────────────────────────────────────

def _group_of(name: str, self_pkgs: set[str]) -> str:
    if name.startswith(_JDK_PUBLIC):
        return 'jdk-public'
    if not name.startswith(_COM_SUN_THIRDPARTY) and name.startswith(_JDK_INTERNAL):
        return 'jdk-internal'
    key = _agg_key(name)
    if any(key == s or key.startswith(s + '/') or s.startswith(key + '/')
           for s in self_pkgs):
        return 'self'
    return 'third-party'


def _agg_key(name: str) -> str:
    """包级聚合键：去内部类 $ 尾巴后的包路径（第三方再聚合到顶层两段）。"""
    name = name.split('$', 1)[0]
    parts = name.split('/')
    return '/'.join(parts[:-1]) if len(parts) > 1 else name


def main() -> None:
    ap = argparse.ArgumentParser(description='pilot 前置依赖透视：jar/字节码/源码 → 分组去重排序的包依赖')
    ap.add_argument('paths', nargs='+', type=Path, help='.jar/.zip、.class、目录（字节码模式，默认）或 .java 源（--java）')
    ap.add_argument('--java', action='store_true', help='源码模式：解析 package/import（近似，抓不到 FQCN 内联与隐式 java.lang）')
    ap.add_argument('--top', type=int, default=0, metavar='N', help='第三方分组只列前 N（按引用类数）')
    ap.add_argument('--classes', action='store_true', help='输出附类级清单（默认包级聚合）')
    ap.add_argument('--json', action='store_true', help='机器可读输出')
    args = ap.parse_args()

    refs: set[str] = set()
    self_names: set[str] = set()
    n_classes = 0
    if args.java:
        refs, self_names = _scan_java_sources(args.paths)
        n_classes = -1
    else:
        # 显式给出的归档文件（jar/zip/war）即使 0 个 .class 也是合法空壳
        #（如 guava 的 listenablefuture-9999.0-empty 占位防冲突 jar）
        has_archive = any(p.suffix.lower() in ('.jar', '.zip', '.war') for p in args.paths)
        for _src, _name, data in _iter_classfiles(args.paths):
            n_classes += 1
            _scan_class_bytes(data, refs, self_names)
        if n_classes == 0:
            if has_archive:
                print('[scan] 空壳归档（0 个 .class，占位/防冲突 stub）')
                return
            sys.exit('未找到 .class（字节码模式）——源码树请加 --java')

    self_pkgs = {_agg_key(s) for s in self_names}
    by_group: dict[str, set[str]] = defaultdict(set)
    for name in refs:
        by_group[_group_of(name, self_pkgs)].add(name)

    if args.json:
        out = {g: sorted(s) for g, s in by_group.items()}
        out['_meta'] = {'scanned_classes': n_classes, 'self_packages': sorted(self_pkgs),
                        'mode': 'java-src' if args.java else 'bytecode'}
        print(json.dumps(out, indent=2, ensure_ascii=False, sort_keys=True))
        return

    mode = '源码 import（近似）' if args.java else '字节码常量池（CONSTANT_Class 一跳）'
    n_self = f"{len(self_names)} 个" if n_classes >= 0 else f"{len(self_names)} 个包"
    print(f"[scan] 模式 {mode}｜扫描类 {n_classes if n_classes >= 0 else '?'}｜self {n_self}")
    order = ['jdk-public', 'jdk-internal', 'third-party', 'self']
    label = {'jdk-public': 'JDK 公开 API', 'jdk-internal': 'JDK 内部（伴生/手写信号）',
             'third-party': '第三方', 'self': '自身'}
    for g in order:
        names = by_group.get(g)
        if not names:
            continue
        pkgs = sorted({_agg_key(n) for n in names})
        print(f"[{g:<12}] {len(names)} 类 / {len(pkgs)} 包  # {label[g]}")
        if g == 'third-party' and args.top:
            top_pkgs = sorted(pkgs, key=lambda p: -sum(1 for n in names if _agg_key(n) == p))[:args.top]
            pkgs = top_pkgs
        print('  ' + ', '.join(pkgs))
        if args.classes:
            for n in sorted(names):
                print(f"    {n}")


if __name__ == '__main__':
    main()

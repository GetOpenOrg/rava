#!/usr/bin/env python3
from __future__ import annotations
"""
方法级调用链追踪（独立脚本，不依赖项目代码）

从用户 Java 类出发，BFS 追踪实际被调用的方法链路，统计所有涉及的 JDK 类。
实现 docs/trace_callchain_analysis_principle.md 描述的完整算法：

  五条引用发现通道：
    1 调用链展开   2 方法描述符类型   3 字段声明类型
    4 字节码直接类引用（new/checkcast/instanceof/anewarray/multianewarray/
      字段属主+字段描述符类型/ldc Class）   5 异常表 catch 类型
  三层优化：
    L1 native 边界  — native 方法无字节码，显式标记为边界，不尝试展开
    L2 <clinit> 门控 — 按 JVM 初始化语义触发（new / invokestatic /
                       getstatic/putstatic 三类），沿父类链递归初始化，
                       触发时直接入队（不受 BFS 出队顺序影响）
    L3 RTA 虚调用   — 虚分发只解析到 instantiated 集合里的具体子类型
  方法解析（JVMS §5.4.3.3/§5.4.3.4 近似）：
    出队目标若在本类方法表未命中，沿父类链/接口闭包定位声明者，
    继承方法与 default 方法不再断链
  VTA（可选）     — 方法内操作数栈类型追踪，覆盖 invokevirtual 与 invokeinterface
  Two-pass RTA    — 先无过滤收集全量 instantiated，再带种子精确重跑；
                     层次索引随 shared_cache 重建，跨 pass 不丢失

2026-09-19 完整版相对 0919 版的补全：
  1. 字段指令同时提取字段描述符类型（恢复自基线版，0917 起丢失）
  2. multianewarray 提取元素类型（剥 [ 前缀）
  3. 方法解析：目标类未声明时沿父类链/接口闭包找声明者
  4. 层次索引生命周期：get() 命中 shared_cache 时同步重建 hier_super/hier_ifaces，
     修复 two-pass 等共享缓存模式下 is_subtype 退化为 sub==sup、RTA 失效的问题
  5. <clinit> 触发补全：invokestatic / getstatic/putstatic 属主 + 父类链 + 直接入队
     （修复"类的方法全部出队后才被实例化则 <clinit> 永不展开"的时序缺口）
  6. 方法表提取 ACC_ABSTRACT / ACC_STATIC
  7. VTA 接收者类型追踪覆盖 invokeinterface
  8. unresolved 统计：解析不到声明者的调用目标显式记录并输出

用法:
    python3 scripts/trace_callchain.py tests/e2e/01_basics/HelloWorld.java
    python3 scripts/trace_callchain.py HelloWorld.java --tree
    python3 scripts/trace_callchain.py HelloWorld.java --tree --max-depth 6
    python3 scripts/trace_callchain.py HelloWorld.java --tree-only
"""

import os
import struct
import subprocess
import sys
import zipfile
from collections import deque
from pathlib import Path
from typing import NamedTuple

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
    解析 .class 文件，返回 (class_name, pool, methods, super_name, iface_names, field_type_refs)
      class_name     : 本类二进制名
      pool           : 常量池列表
      methods        : list of (name, descriptor, bytecode | None,
                                is_native, is_abstract, is_static, catch_types)
      super_name     : 父类 binary name（java/lang/Object 返回 None）
      iface_names    : 实现的接口列表
      field_type_refs: 字段声明描述符中引用的类名列表（通道 3）
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

    # 解析字段表，提取字段描述符中引用的类型（通道 3）
    field_type_refs: list[str] = []
    for _ in range(r.u2()):
        r.skip(2)                                # field access_flags
        r.skip(2)                                # name_index
        fdesc = pool[r.u2()][1]                  # descriptor_index → 字段类型描述符
        field_type_refs.extend(_extract_desc_classes(fdesc))
        for _ in range(r.u2()): r.skip(2); r.skip(r.u4())  # attributes

    # 解析方法表（access 标志 + 异常表捕获类型）
    methods = []
    for _ in range(r.u2()):
        m_acc       = r.u2()
        mname       = pool[r.u2()][1]
        mdesc       = pool[r.u2()][1]
        is_native   = bool(m_acc & 0x0100)       # ACC_NATIVE → L1 边界
        is_abstract = bool(m_acc & 0x0400)       # ACC_ABSTRACT → 无方法体，仅描述符依赖
        is_static   = bool(m_acc & 0x0008)       # ACC_STATIC
        bytecode    = None
        catch_types: list[str] = []              # 异常表捕获类型（通道 5）
        for _ in range(r.u2()):
            aname = pool[r.u2()][1]
            alen  = r.u4()
            if aname == 'Code':
                r.skip(4)
                clen = r.u4()
                bytecode = r.read(clen)
                # 异常表：每条 8 字节，最后4字节是 catch_type 常量池索引
                for _ in range(r.u2()):
                    r.skip(6)                    # from_pc, to_pc, handler_pc
                    ct_idx = r.u2()              # catch_type (0 = finally)
                    if ct_idx != 0:
                        e = pool[ct_idx]
                        if e and e[0] == _CLASS:
                            cname = _utf8(pool, e[1])
                            if cname: catch_types.append(cname)
                for _ in range(r.u2()): r.skip(2); r.skip(r.u4())
            else:
                r.skip(alen)
        methods.append((mname, mdesc, bytecode, is_native, is_abstract, is_static, catch_types))

    class_name = pool[pool[this_idx][1]][1]

    # 提取父类和接口（用于 RTA 子类型判断与 JVMS 方法解析）
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

    return class_name, pool, methods, super_name, iface_names, field_type_refs


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

def _fref_type_desc(pool, ref_idx):
    """提取字段引用的类型描述符字符串（如 "Ljava/util/Iterator;"）。"""
    e = pool[ref_idx]
    if not e or e[0] != _FIELDREF: return None
    nat = pool[e[2]]
    if not nat or nat[0] != _NAME_TYPE: return None
    return _utf8(pool, nat[2])

def _elem_cls(pool, cls_idx):
    """从数组类名（如 [[Ljava/lang/String;）提取引用元素类名；基本类型返回 None。"""
    e = pool[cls_idx]
    if not e or e[0] != _CLASS: return None
    n = _utf8(pool, e[1]).lstrip('[')
    if n.startswith('L') and n.endswith(';'):
        n = n[1:-1]
    if not n or len(n) == 1 and n in 'BCDFIJSZ':
        return None
    return n if '[' not in n else None


def _count_args(desc: str) -> int:
    """从方法描述符计算参数个数（不含 this）"""
    i = desc.find('(') + 1
    j = desc.find(')')
    params = desc[i:j]
    count, k = 0, 0
    while k < len(params):
        c = params[k]
        if c in 'BCDFIJSZ':
            count += 1; k += 1
        elif c == 'L':
            count += 1; k = params.index(';', k) + 1
        elif c == '[':
            k += 1
            while k < len(params) and params[k] == '[': k += 1
            if k < len(params):
                if params[k] in 'BCDFIJSZ': k += 1
                elif params[k] == 'L': k = params.index(';', k) + 1
            count += 1
        else:
            k += 1
    return count


def _extract_desc_classes(desc: str) -> list[str]:
    """
    从方法/字段描述符中提取所有引用类型的类名。
    例如 (Ljava/util/List;I)Ljava/lang/String; → ['java/util/List', 'java/lang/String']
    数组类型会剥掉 [ 前缀后再提取，基本类型和 void 忽略。
    """
    result: list[str] = []
    i = 0
    n = len(desc)
    # 跳过开头的 '('，统一扫描参数段和返回段
    while i < n:
        c = desc[i]
        if c in '()':
            i += 1
            continue
        if c == '[':                  # 数组：跳过所有 [ 前缀
            while i < n and desc[i] == '[':
                i += 1
            if i >= n:
                break
            c = desc[i]
        if c == 'L':                  # 对象类型 Lxxx/yyy;
            j = desc.index(';', i)
            cls = desc[i+1:j]
            if cls:
                result.append(cls)
            i = j + 1
        else:                         # 基本类型 / V：单字符跳过
            i += 1
    return result


def _vta_analyze(bytecode: bytes, pool, return_summaries: dict | None = None) -> dict:
    """
    线性操作数栈模拟（VTA），返回 {invokevirtual/invokeinterface 指令偏移量: 接收者具体类名或None}。
    None 表示无法静态确定，调用方应保守回退到声明类型。
    只处理引用类型，忽略分支合并（线性扫描）。
    return_summaries: {(cls,name,desc) → concrete_type} — 跨方法返回类型摘要
    """
    stack = []   # 每格: str(具体类名) | None(未知)
    locs  = {}   # slot → str | None

    def push(t): stack.append(t)
    def pop():   return stack.pop() if stack else None

    def _ret_type(cls, nm, desc):
        """根据摘要或默认推 None，供 invoke* 的返回值使用"""
        if return_summaries:
            t = return_summaries.get((cls, nm, desc))
            if t: return t
        return None

    result = {}
    i, n = 0, len(bytecode)
    while i < n:
        op = bytecode[i]

        if op == 0xBB:                               # new → 已知具体类型
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            push(cls); i += 3

        elif op == 0x01: push(None); i += 1          # aconst_null
        elif op in (0x2A, 0x2B, 0x2C, 0x2D):         # aload_0..3
            push(locs.get(op - 0x2A)); i += 1
        elif op == 0x19:                              # aload
            push(locs.get(bytecode[i+1])); i += 2
        elif op in (0x4B, 0x4C, 0x4D, 0x4E):         # astore_0..3
            locs[op - 0x4B] = pop(); i += 1
        elif op == 0x3A:                              # astore
            locs[bytecode[i+1]] = pop(); i += 2

        elif op == 0x59:                              # dup
            push(stack[-1] if stack else None); i += 1
        elif op == 0x5A:                              # dup_x1
            a = pop(); b = pop(); push(a); push(b); push(a); i += 1
        elif op == 0x5B:                              # dup_x2
            a = pop(); b = pop(); c2 = pop()
            push(a); push(c2); push(b); push(a); i += 1
        elif op == 0x57: pop(); i += 1               # pop
        elif op == 0x58: pop(); pop(); i += 1        # pop2

        elif op == 0xC0:                              # checkcast → 细化类型
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            pop(); push(cls); i += 3

        elif op in (0xB2, 0xB4):                      # getstatic / getfield
            if op == 0xB4: pop()
            push(None); i += 3
        elif op in (0xB3, 0xB5):                      # putstatic / putfield
            pop()
            if op == 0xB5: pop()
            i += 3

        elif op == 0xB6:                              # invokevirtual → 记录接收者
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            if ref:
                cls, nm, desc = ref
                nargs = _count_args(desc)
                idx = len(stack) - 1 - nargs
                result[i] = stack[idx] if 0 <= idx < len(stack) else None
                for _ in range(nargs + 1): pop()
                if ')' in desc and desc[desc.index(')') + 1] != 'V':
                    push(_ret_type(cls, nm, desc))
            i += 3

        elif op == 0xB9:                              # invokeinterface → 记录接收者
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            if ref:
                cls, nm, desc = ref
                nargs = _count_args(desc)
                idx = len(stack) - 1 - nargs
                result[i] = stack[idx] if 0 <= idx < len(stack) else None
                for _ in range(nargs + 1): pop()
                if ')' in desc and desc[desc.index(')') + 1] != 'V':
                    push(_ret_type(cls, nm, desc))
            i += 5

        elif op in (0xB7, 0xB8):                      # invokespecial / invokestatic
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            if ref:
                cls, nm, desc = ref
                nargs = _count_args(desc)
                recv = 0 if op == 0xB8 else 1
                for _ in range(nargs + recv): pop()
                if ')' in desc and desc[desc.index(')') + 1] != 'V':
                    push(_ret_type(cls, nm, desc))
            i += 3

        elif op == 0xBA: i += 5                      # invokedynamic（跳过）

        elif op == 0xAA:                              # tableswitch
            pop()
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            lo = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            hi = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            i += (hi - lo + 1) * 4

        elif op == 0xAB:                              # lookupswitch
            pop()
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            np_ = struct.unpack_from('>I', bytecode, i)[0]; i += 4
            i += np_ * 8

        elif op == 0xC4:                              # wide
            sub = bytecode[i+1]
            i += 6 if sub == 0x84 else 4

        else:
            sz = _SZ[op]
            i += sz if sz else 1

    return result


def _vta_return_type(bytecode: bytes, pool) -> str | None:
    """
    推断方法的具体返回类型（简化版：若方法内仅有唯一 new 类型，则返回该类型）。
    覆盖工厂方法、构造辅助等常见模式。
    """
    new_types: set[str] = set()
    i, n = 0, len(bytecode)
    while i < n:
        op = bytecode[i]
        if op == 0xBB:
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i+1)[0])
            if cls: new_types.add(cls)
            i += 3
        elif op == 0xAA:
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            lo = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            hi = struct.unpack_from('>i', bytecode, i)[0]; i += 4
            i += (hi - lo + 1) * 4
        elif op == 0xAB:
            pad = (4 - ((i + 1) % 4)) % 4
            i += 1 + pad + 4
            np_ = struct.unpack_from('>I', bytecode, i)[0]; i += 4
            i += np_ * 8
        elif op == 0xC4:
            i += 6 if bytecode[i+1] == 0x84 else 4
        else:
            sz = _SZ[op]; i += sz if sz else 1
    return next(iter(new_types)) if len(new_types) == 1 else None


def _compute_return_summaries(visited: set, cache: dict) -> dict:
    """
    对调用链里每个被访问的方法推断返回类型，生成 {(cls,name,desc) → concrete_type}。
    只记录能确定具体类型的方法，供跨方法 VTA 使用。
    """
    summaries: dict = {}
    for cls, name, desc in visited:
        ret = desc[desc.index(')') + 1:] if ')' in desc else ''
        if ret in ('V', 'I', 'J', 'F', 'D', 'Z', 'B', 'C', 'S') or ret.startswith('['):
            continue  # 基本类型或数组，不追踪
        parsed = cache.get(cls)
        if not parsed: continue
        _, pool, methods, _, _, _ = parsed
        for m in methods:
            if m[0] == name and m[1] == desc and m[2]:
                rt = _vta_return_type(m[2], pool)
                if rt: summaries[(cls, name, desc)] = rt
    return summaries


# ─── 字节码扫描 ──────────────────────────────────────────────────────────────────
class ScanResult(NamedTuple):
    """一次字节码扫描的全部发现（对应文档第四节五条通道的原始输入）。"""
    vcalls:     list  # (cls, name, desc) invokevirtual / invokeinterface → RTA/VTA
    dcalls:     list  # (cls, name, desc) invokespecial    → 直接跟随
    scalls:     list  # (cls, name, desc) invokestatic    → 直接跟随 + 触发 <clinit>
    new_cls:    list  # new 实例化的类                     → RTA instantiated + 触发 <clinit>
    sfield_cls: list  # getstatic/putstatic 属主类         → 触发 <clinit>
    other_refs: list  # 其余类型引用（字段属主/字段描述符类型/ldc/anewarray/...）


def _scan(bytecode: bytes, pool, vta_override: dict | None = None) -> ScanResult:
    """
    vta_override: {pc → concrete_cls} — 若提供，invokevirtual/invokeinterface
                  使用 VTA 确定的接收者类型替换声明类型。
    """
    vcalls, dcalls, scalls = [], [], []
    new_cls, sfield_cls, other_refs = [], [], []
    i, n = 0, len(bytecode)

    while i < n:
        op = bytecode[i]

        if op in (0xB6, 0xB9):                    # invokevirtual, invokeinterface
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref:
                cls, nm, desc = ref
                # VTA: 若已知具体接收者类型，替换声明类型
                if vta_override and i in vta_override and vta_override[i]:
                    cls = vta_override[i]
                vcalls.append((cls, nm, desc))
                # 收集描述符中参数/返回值涉及的类型
                other_refs.extend(_extract_desc_classes(desc))
            i += 3 if op == 0xB6 else 5

        elif op == 0xB7:                          # invokespecial → 直接跟随
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref:
                dcalls.append(ref)
                other_refs.extend(_extract_desc_classes(ref[2]))
            i += 3

        elif op == 0xB8:                          # invokestatic → 直接跟随 + 初始化
            ref = _mref(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if ref:
                scalls.append(ref)
                other_refs.extend(_extract_desc_classes(ref[2]))
            i += 3

        elif op == 0xBA:                          # invokedynamic（跳过，已知局限）
            i += 5

        elif op == 0x12:                          # ldc → Class/String/MethodType 常量
            entry = pool[bytecode[i + 1]]
            if entry and entry[0] == _CLASS:
                cls = _utf8(pool, entry[1])
                if cls and '[' not in cls: other_refs.append(cls)
            i += 2

        elif op == 0x13:                          # ldc_w → 宽索引版 ldc
            entry = pool[struct.unpack_from('>H', bytecode, i + 1)[0]]
            if entry and entry[0] == _CLASS:
                cls = _utf8(pool, entry[1])
                if cls and '[' not in cls: other_refs.append(cls)
            i += 3

        elif op == 0xBB:                          # new → RTA instantiated + <clinit>
            cls = _cls_name(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: new_cls.append(cls)
            i += 3

        elif op in (0xB2, 0xB3, 0xB4, 0xB5):      # getstatic/putstatic/getfield/putfield
            ref_idx = struct.unpack_from('>H', bytecode, i + 1)[0]
            cls = _fref_cls(pool, ref_idx)
            if cls:
                other_refs.append(cls)
                if op in (0xB2, 0xB3):            # 静态字段访问属主 → 触发 <clinit>
                    sfield_cls.append(cls)
            # 字段描述符里的类型（如 System.out:Ljava/io/PrintStream; 的 PrintStream）
            fdesc = _fref_type_desc(pool, ref_idx)
            if fdesc:
                other_refs.extend(_extract_desc_classes(fdesc))
            i += 3

        elif op == 0xC5:                          # multianewarray → 元素类型
            cls = _elem_cls(pool, struct.unpack_from('>H', bytecode, i + 1)[0])
            if cls: other_refs.append(cls)
            i += 4

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
            sub = bytecode[i+1]
            i += 6 if sub == 0x84 else 4

        else:
            sz = _SZ[op]
            i += sz if sz else 1

    return ScanResult(vcalls, dcalls, scalls, new_cls, sfield_cls, other_refs)


# ─── 截断候选分析 ──────────────────────────────────────────────────────────────────

def bfs_depth_analysis(user_class_files: list[str], resolver: 'JdkResolver',
                       shared_cache: dict | None = None) -> tuple[dict, dict]:
    """
    逐层 BFS：追踪每个 JDK 类首次出现在调用链的深度（方法调用层数）。
    深度 1 = 直接从用户代码调用的类；深度 2 = 从深度1类的方法调用的类；以此类推。

    返回:
      class_depth : {cls → first_depth}
      layers      : {depth → set[cls]}  按层分组的类集合
    """
    sc = shared_cache if shared_cache is not None else {}

    def get(cls):
        if cls in sc: return sc[cls]
        data = resolver.get(cls)
        if data is None: sc[cls] = None; return None
        try:
            result = _parse_class(data)
            sc[cls] = result
            return result
        except Exception:
            sc[cls] = None; return None

    visited:     set[tuple] = set()
    queue:       deque       = deque()  # (cls, name, desc, depth)
    class_depth: dict[str, int] = {}

    # 种子：从用户 .class 文件的方法体里直接扫出来的调用 → 深度 1
    for path in user_class_files:
        with open(path, 'rb') as f: data = f.read()
        _, pool, methods, _, _, _ = _parse_class(data)
        for m in methods:
            if not m[2]: continue
            sr = _scan(m[2], pool)
            for cls, nm, desc in sr.vcalls + sr.dcalls + sr.scalls:
                if _in_scope(cls):
                    key = (cls, nm, desc)
                    if key not in visited:
                        visited.add(key)
                        queue.append((cls, nm, desc, 1))

    while queue:
        cls, name, desc, depth = queue.popleft()
        # 记录该类首次出现的深度（取最浅）
        if cls not in class_depth or depth < class_depth[cls]:
            class_depth[cls] = depth
        parsed = get(cls)
        if parsed is None: continue
        _, pool, methods, _, _, _ = parsed
        for m in methods:
            if m[0] == name and m[1] == desc and m[2]:
                sr = _scan(m[2], pool)
                for ref_cls, nm2, desc2 in sr.vcalls + sr.dcalls + sr.scalls:
                    if _in_scope(ref_cls):
                        key2 = (ref_cls, nm2, desc2)
                        if key2 not in visited:
                            visited.add(key2)
                            queue.append((ref_cls, nm2, desc2, depth + 1))

    # 按深度分组
    from collections import defaultdict
    layers: dict[int, set] = defaultdict(set)
    for cls, d in class_depth.items():
        layers[d].add(cls)

    return class_depth, dict(layers)


def analyze_internal_boundary(visited: set, cache: dict) -> list:
    """
    找出调用链中"跨越公开API→内部实现边界"的方法：
      - 方法本身属于 java/ / javax/ 公开 API
      - 方法体内直接调用了 sun/ / jdk/ / com/sun/ 等内部类
    这些方法就是需要手写 native 实现的目标列表。

    返回列表，每条记录：
      {
        'cls':           公开 API 类名,
        'name':          方法名,
        'desc':          描述符,
        'calls_internal': [(internal_cls, method_name, desc), ...],  # 调用的内部方法
        'refs_internal':  [internal_cls, ...],                       # 仅引用的内部类
      }
    """
    PUBLIC_PREFIXES = ('java/', 'javax/')
    results = []

    for cls, name, desc in visited:
        if not cls.startswith(PUBLIC_PREFIXES):
            continue
        parsed = cache.get(cls)
        if not parsed:
            continue
        _, pool, methods, _, _, _ = parsed
        for m in methods:
            if m[0] != name or m[1] != desc or not m[2]:
                continue
            sr = _scan(m[2], pool)
            calls_int = [
                (ref_cls, nm, d)
                for ref_cls, nm, d in sr.vcalls + sr.dcalls + sr.scalls
                if _is_internal(ref_cls)
            ]
            refs_int = [c for c in sr.new_cls + sr.sfield_cls + sr.other_refs
                        if _is_internal(c)]
            if calls_int or refs_int:
                results.append({
                    'cls':            cls,
                    'name':           name,
                    'desc':           desc,
                    'calls_internal': calls_int,
                    'refs_internal':  refs_int,
                })

    results.sort(key=lambda x: (x['cls'], x['name'], x['desc']))
    return results


def build_class_dep_graph(visited: set, cache: dict) -> dict:
    """
    从调用链的已访问方法集合里，构建类级依赖图：
      dep_graph[A] = set of classes that A's methods call
    用于后续快速计算"截断 A 能消除哪些类"。
    """
    dep_graph: dict[str, set] = {}
    for cls, name, desc in visited:
        parsed = cache.get(cls)
        if not parsed: continue
        _, pool, methods, _, _, _ = parsed
        for m in methods:
            if m[0] == name and m[1] == desc and m[2]:
                sr = _scan(m[2], pool)
                deps = dep_graph.setdefault(cls, set())
                for ref_cls, _, _ in sr.vcalls + sr.dcalls + sr.scalls:
                    if _in_scope(ref_cls) and ref_cls != cls:
                        deps.add(ref_cls)
                for ref_cls in sr.new_cls + sr.sfield_cls + sr.other_refs:
                    if _in_scope(ref_cls) and ref_cls != cls:
                        deps.add(ref_cls)
    return dep_graph


def _reachable(seeds: set, dep_graph: dict, exclude: str | None = None) -> set:
    """从 seeds 出发，在 dep_graph 上做 BFS，返回可达类集合（可排除 exclude）。"""
    visited: set[str] = set()
    queue = deque(seeds)
    while queue:
        cls = queue.popleft()
        if cls in visited or cls == exclude:
            continue
        visited.add(cls)
        for dep in dep_graph.get(cls, ()):
            if dep not in visited:
                queue.append(dep)
    return visited


def analyze_cutoff_candidates(
    visited: set,
    all_classes: set,
    cache: dict,
    dep_graph: dict,
    seed_classes: set,
    top_n: int = 25,
) -> list:
    """
    对调用链中每个类计算截断价值，返回按"能消除类数"降序排列的候选列表。

    每条记录包含：
      cls            : 候选截断类名
      eliminated     : 截断该类后消除的类集合（精确值，逐个 BFS 模拟）
      entry_methods  : 从调用链其他地方调用该类的方法列表（需手写实现的方法）
      native_count   : 该类已有的 native 方法数（越多 → 越接近天然截断边界）
      is_natural_leaf: 是否为天然叶节点（自身所有被调方法都是 native）
    """
    all_visited_cls = {c for c, _, _ in visited}
    base_reachable = _reachable(seed_classes, dep_graph)

    # 调用入口：从其他类调用了该类哪些方法
    entry_methods: dict[str, list] = {}
    for cls, name, desc in visited:
        entry_methods.setdefault(cls, []).append((name, desc))

    results = []
    for cls in sorted(all_visited_cls):
        reachable_without = _reachable(seed_classes, dep_graph, exclude=cls)
        eliminated = base_reachable - reachable_without - {cls}

        # 该类的 native 方法统计
        native_count = 0
        total_methods = 0
        parsed = cache.get(cls)
        if parsed:
            for m in parsed[2]:
                total_methods += 1
                if m[3]: native_count += 1

        called = entry_methods.get(cls, [])
        is_natural_leaf = bool(called) and all(
            any(m[0] == nm and m[3] for m in (parsed[2] if parsed else []))
            for nm, _ in called
        ) if parsed else False

        results.append({
            'cls':            cls,
            'eliminated':     eliminated,
            'elim_count':     len(eliminated),
            'entry_methods':  called,
            'native_count':   native_count,
            'total_methods':  total_methods,
            'is_natural_leaf': is_natural_leaf,
        })

    results.sort(key=lambda x: x['elim_count'], reverse=True)
    return results[:top_n]


def _method_flag_stats(visited: set, cache: dict) -> tuple[int, int]:
    """统计 visited 目标中声明为 abstract / static 的方法数（按引用类近似）。"""
    n_abstract = n_static = 0
    for cls, name, desc in visited:
        p = cache.get(cls)
        if not p:
            continue
        for m in p[2]:
            if m[0] == name and m[1] == desc:
                n_abstract += bool(m[4])
                n_static  += bool(m[5])
                break
    return n_abstract, n_static


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

# 内部包前缀：出现即截断（不展开方法体，生成 stub）
_INTERNAL_PREFIXES = ('sun/', 'jdk/', 'com/sun/', 'com/oracle/')

def _is_internal(cls: str) -> bool:
    return cls.startswith(_INTERNAL_PREFIXES)


# HelloWorld 路径中的 I/O 编码层截断边界（手动指定版，供对比）
_DEFAULT_CUTOFFS = frozenset({
    'sun/nio/cs/StreamEncoder',
    'java/nio/charset/CharsetEncoder',
    'java/nio/charset/Charset',
    'java/nio/charset/CharsetDecoder',
    'sun/nio/cs/FastCharsetProvider',
    'sun/nio/cs/StandardCharsets',
    'java/io/FileOutputStream',
    'java/io/FileDescriptor',
    'java/lang/ref/ReferenceQueue',
    'java/lang/ref/SoftReference',
    'java/lang/ref/WeakReference',
})


# ─── BFS 核心（三层 + 方法解析 + two-pass RTA + VTA + 截断 可选） ────────────────
class BfsResult(NamedTuple):
    visited:      set   # (cls, name, desc) 已入队的调用目标
    all_classes:  set   # 调用链类 ∪ 仅引用类
    native_stubs: set   # native 边界方法 (声明类, name, desc)
    instantiated: set   # RTA 实例化集合（new 指令填入）
    cache:        dict  # 类解析缓存（shared_cache）
    initialized:  set   # 已触发 <clinit> 的类（JVM 初始化语义，含父类链）
    unresolved:   set   # 解析不到声明者的调用目标


def bfs(user_class_files: list[str], resolver: JdkResolver,
        seed_instantiated: set | None = None,
        use_rta: bool = True,
        use_vta: bool = False,
        cutoff_classes: frozenset | None = None,
        return_summaries: dict | None = None,
        shared_cache: dict | None = None,
        call_graph: dict | None = None) -> BfsResult:
    """
    通用 BFS 入口。

    参数：
      seed_instantiated  : 预置 instantiated 集合（two-pass 第二程用）
      use_rta            : 是否启用 RTA 虚调用过滤
      use_vta            : 是否启用方法内操作数栈类型分析
      cutoff_classes     : 截断边界类集合——到达这些类时记录但不展开其方法体
      return_summaries   : 跨方法返回类型摘要，增强 VTA 精度
      shared_cache       : 多次 BFS 间共享的类解析缓存（层次索引随之重建）
      call_graph         : 若非 None，收集 {调用方方法 → 被调方法集合}
    """
    visited:      set[tuple] = set()
    queue:        deque       = deque()
    refs:         set[str]    = set()
    native_stubs: set[tuple]  = set()
    instantiated: set[str]    = set(seed_instantiated or ())
    initialized:  set[str]    = set()
    unresolved:   set[tuple]  = set()

    hier_super:  dict[str, str | None] = {}
    hier_ifaces: dict[str, list[str]]  = {}
    cache: dict = shared_cache if shared_cache is not None else {}
    _cur: list = [None]   # 当前正在处理的方法 key，用于构建调用图边

    def get(cls):
        if cls in cache:
            r = cache[cls]
            # 命中 shared_cache 时同步重建层次索引，否则 two-pass 第二程
            # hier 为空、is_subtype 退化为 sub==sup，RTA 子类型分派失效
            if r is not None and cls not in hier_super:
                _, _, _, sn, ifaces, _ = r
                hier_super[cls]  = sn
                hier_ifaces[cls] = ifaces
            return r
        data = resolver.get(cls)
        if data is None: cache[cls] = None; return None
        try:
            result = _parse_class(data)
            cache[cls] = result
            _, _, _, sn, ifaces, _ = result
            hier_super[cls]  = sn
            hier_ifaces[cls] = ifaces
            return result
        except Exception: cache[cls] = None; return None

    def is_subtype(sub: str, sup: str, _seen: frozenset = frozenset()) -> bool:
        if sub == sup: return True
        if sub in _seen: return False
        seen2 = _seen | {sub}
        if sub not in hier_super:
            get(sub)                     # 惰性解析：层次未建则先解析
            if sub not in hier_super:    # 解析失败（类不存在/不可读）→ 视为叶节点
                return False
        parent = hier_super.get(sub)
        if parent and is_subtype(parent, sup, seen2): return True
        for iface in hier_ifaces.get(sub, []):
            if is_subtype(iface, sup, seen2): return True
        return False

    def find_declaring(cls, name, desc):
        """
        JVMS §5.4.3.3 / §5.4.3.4 方法解析近似，返回 (声明类, 方法元组) 或 None：
        1. 沿父类链自下而上找 (name, desc) 精确声明者，优先带方法体/native 的声明
           （声明为 abstract 时继续上溯找具体实现，保留首个 abstract 声明者兜底）；
        2. 类链整条找不到时，沿「父类链 + 接口」传递闭包找 default 方法声明者；
        3. 仍找不到时，退化为同名 ACC_NATIVE 声明——覆盖签名多态方法
           （MethodHandle.invokeBasic / VarHandle.get 等，其调用点描述符由 javac
           按实际签名生成，与声明描述符必然不同，精确匹配永远失败），
           这类目标本质是 native/intrinsic 边界，无方法体可展开。
        """
        chain: list[str] = []
        c, seen = cls, set()
        while c is not None and c not in seen:
            seen.add(c)
            chain.append(c)
            get(c)
            c = hier_super.get(c)

        fallback = None
        for k in chain:
            p = get(k)
            if p is None:
                continue
            for m in p[2]:
                if m[0] == name and m[1] == desc:
                    if m[2] or m[3]:
                        return k, m
                    if fallback is None:
                        fallback = (k, m)
                    break

        # 接口传递闭包（default 方法一定带方法体）
        iface_seen: set = set(chain)
        iface_q = deque()
        for k in chain:
            iface_q.extend(hier_ifaces.get(k, ()))
        while iface_q:
            k = iface_q.popleft()
            if k in iface_seen:
                continue
            iface_seen.add(k)
            get(k)
            iface_q.extend(hier_ifaces.get(k, ()))
            p = get(k)
            if p is None:
                continue
            for m in p[2]:
                if m[0] == name and m[1] == desc and (m[2] or m[3]):
                    return k, m

        # 同名 native 兜底（签名多态边界）
        for k in chain:
            p = get(k)
            if p is None:
                continue
            for m in p[2]:
                if m[0] == name and m[3]:
                    return k, m
        return fallback

    def enqueue(cls, name, desc):
        if cutoff_classes and cls in cutoff_classes:
            refs.add(cls)  # 记录截断类，但不展开其方法体
            return
        if name == '<clinit>' and cls not in initialized:
            return
        key = (cls, name, desc)
        if call_graph is not None and _cur[0] is not None:
            call_graph.setdefault(_cur[0], set()).add(key)
        if key not in visited:
            visited.add(key)
            queue.append(key)

    def _init_class(cls: str) -> None:
        """JVM §5.5：初始化一个类前须先初始化其全部父类。幂等；触发即入队。"""
        chain = []
        c, seen = cls, set()
        while c is not None and c not in seen and _in_scope(c):
            seen.add(c)
            chain.append(c)
            get(c)
            c = hier_super.get(c)
        for c in reversed(chain):        # 父类先初始化
            if c not in initialized:
                initialized.add(c)
                enqueue(c, '<clinit>', '()V')

    def process(bytecode, pool):
        vta_hints = _vta_analyze(bytecode, pool, return_summaries) if use_vta else None
        sr = _scan(bytecode, pool, vta_hints)

        for cls in sr.new_cls:
            if _in_scope(cls):
                instantiated.add(cls)     # RTA 实例化集合
                refs.add(cls)
                _init_class(cls)          # new → 触发 <clinit>（含父类链）
        for cls in sr.sfield_cls:         # getstatic/putstatic 属主 → 初始化
            if _in_scope(cls):
                refs.add(cls)
                _init_class(cls)
        for cls in sr.other_refs:
            if _in_scope(cls):
                refs.add(cls)
        for cls, nm, desc in sr.dcalls:   # invokespecial → 直接跟随
            if _in_scope(cls): enqueue(cls, nm, desc)
        for cls, nm, desc in sr.scalls:   # invokestatic → 直接跟随 + 初始化
            if _in_scope(cls):
                enqueue(cls, nm, desc)
                _init_class(cls)

        for cls, nm, desc in sr.vcalls:
            if not _in_scope(cls): continue
            if not use_rta:
                enqueue(cls, nm, desc)
                continue
            if cls not in hier_super: get(cls)
            subtypes = [c for c in instantiated
                        if _in_scope(c) and is_subtype(c, cls)]
            if subtypes:
                for sub in subtypes: enqueue(sub, nm, desc)
            else:
                enqueue(cls, nm, desc)

    for path in user_class_files:
        with open(path, 'rb') as f: data = f.read()
        ucls, pool, methods, _, _, field_refs = _parse_class(data)
        # 用户类字段声明类型（通道 3）
        for dcls in field_refs:
            if _in_scope(dcls): refs.add(dcls)
        for m in methods:
            # 方法描述符里的参数/返回类型（通道 2）
            for dcls in _extract_desc_classes(m[1]):
                if _in_scope(dcls): refs.add(dcls)
            # 异常表捕获类型（通道 5）
            for dcls in m[6]:
                if _in_scope(dcls): refs.add(dcls)
            if m[2]:
                if call_graph is not None:
                    _cur[0] = (ucls, m[0], m[1])
                process(m[2], pool)

    while queue:
        cls, name, desc = queue.popleft()
        if call_graph is not None:
            _cur[0] = (cls, name, desc)
        # <init>/<clinit> 不参与继承解析；其余目标沿父类链/接口闭包定位声明者
        if name in ('<init>', '<clinit>'):
            target, m = cls, None
        else:
            found = find_declaring(cls, name, desc)
            if found is None:
                unresolved.add((cls, name, desc))
                for dcls in _extract_desc_classes(desc):
                    if _in_scope(dcls): refs.add(dcls)
                continue
            target, m = found
        parsed = get(target)
        if parsed is None:
            unresolved.add((cls, name, desc))
            continue
        _, pool, methods, _, _, field_refs = parsed
        # 通道 3：该类字段声明类型（首次解析该类时纳入）
        for dcls in field_refs:
            if _in_scope(dcls): refs.add(dcls)
        if m is None:                # <init>/<clinit>：本类精确匹配
            for mm in methods:
                if mm[0] == name and mm[1] == desc:
                    m = mm
                    break
        if m is not None:
            # 通道 2：声明者描述符类型
            for dcls in _extract_desc_classes(m[1]):
                if _in_scope(dcls): refs.add(dcls)
            # 通道 5：异常表 catch 类型
            for dcls in m[6]:
                if _in_scope(dcls): refs.add(dcls)
            if m[1] == desc and m[2]:
                process(m[2], pool)
            elif m[3]:               # native 边界（含签名多态的同名 native 匹配）
                native_stubs.add((target, name, desc))
            # 其余：abstract 声明 → 无方法体，仅类型依赖已收集

    all_classes = {c for c, _, _ in visited} | refs
    return BfsResult(visited, all_classes, native_stubs, instantiated,
                     cache, initialized, unresolved)


def bfs_two_pass(user_class_files: list[str], resolver: JdkResolver,
                 shared_cache: dict | None = None):
    """Two-pass RTA：先收集全部 instantiated，再带种子做精确 RTA 过滤。"""
    sc = shared_cache if shared_cache is not None else {}
    pass1 = bfs(user_class_files, resolver, use_rta=False, shared_cache=sc)
    return bfs(user_class_files, resolver,
               seed_instantiated=pass1.instantiated, use_rta=True, shared_cache=sc)


def bfs_two_pass_with_graph(user_class_files: list[str], resolver: JdkResolver,
                            shared_cache: dict | None = None):
    """Two-pass RTA + 调用图收集，供树状打印使用。返回 (BfsResult, call_graph)。"""
    sc = shared_cache if shared_cache is not None else {}
    pass1 = bfs(user_class_files, resolver, use_rta=False, shared_cache=sc)
    cg: dict = {}
    result = bfs(user_class_files, resolver,
                 seed_instantiated=pass1.instantiated, use_rta=True,
                 call_graph=cg, shared_cache=sc)
    return result, cg


def bfs_vta(user_class_files: list[str], resolver: JdkResolver,
            shared_cache: dict | None = None):
    """VTA + Two-pass RTA：方法内栈分析 + 精确 RTA。"""
    sc = shared_cache if shared_cache is not None else {}
    pass1 = bfs(user_class_files, resolver, use_rta=False, use_vta=True, shared_cache=sc)
    return bfs(user_class_files, resolver,
               seed_instantiated=pass1.instantiated, use_rta=True, use_vta=True,
               shared_cache=sc)


def bfs_interprocedural(user_class_files: list[str], resolver: JdkResolver,
                        shared_cache: dict | None = None):
    """
    跨方法返回类型传播（简化 Points-to）：
      第一轮：VTA + two-pass RTA，建立调用图并缓存方法字节码
      分析轮：对调用图中每个方法推断具体返回类型（_vta_return_type）
      第二轮：用返回类型摘要增强 VTA，使方法调用返回值也能被精确追踪
    """
    sc = shared_cache if shared_cache is not None else {}
    # 第一轮：建立基础调用图
    pass1 = bfs(user_class_files, resolver, use_rta=False, use_vta=True, shared_cache=sc)
    visited1 = bfs(user_class_files, resolver,
                   seed_instantiated=pass1.instantiated, use_rta=True,
                   use_vta=True, shared_cache=sc)
    # 分析轮：计算返回类型摘要
    summaries = _compute_return_summaries(visited1.visited, sc)
    # 第二轮：带摘要重跑
    pass2 = bfs(user_class_files, resolver, use_rta=False, use_vta=True,
                return_summaries=summaries, shared_cache=sc)
    return bfs(user_class_files, resolver,
               seed_instantiated=pass2.instantiated, use_rta=True, use_vta=True,
               return_summaries=summaries, shared_cache=sc)


def bfs_cutoff(user_class_files: list[str], resolver: JdkResolver,
               cutoff_classes: frozenset = _DEFAULT_CUTOFFS,
               shared_cache: dict | None = None):
    """
    语义截断模式（手动指定边界）：
      在指定的类边界处停止展开方法体。
      默认截断 I/O 编码底层（StreamEncoder / CharsetEncoder 等）。
    """
    sc = shared_cache if shared_cache is not None else {}
    pass1 = bfs(user_class_files, resolver, use_rta=False, use_vta=True,
                cutoff_classes=cutoff_classes, shared_cache=sc)
    return bfs(user_class_files, resolver,
               seed_instantiated=pass1.instantiated, use_rta=True, use_vta=True,
               cutoff_classes=cutoff_classes, shared_cache=sc)


def bfs_internal_boundary(user_class_files: list[str], resolver: JdkResolver,
                          shared_cache: dict | None = None):
    """
    内部包边界截断：凡是 sun/ jdk/ com/sun/ com/oracle/ 的类，
    只记录引用，不展开其方法体（视为 native stub 边界）。
    java/ javax/ 公开 API 正常展开。
    """
    sc = shared_cache if shared_cache is not None else {}

    # 动态构建 cutoff 集合：把 BFS 过程中遇到的所有内部类收集进来
    # 通过在 enqueue 里拦截实现，无需预先知道完整列表
    # 方案：先做一次扫描收集所有内部类，再传入 cutoff_classes

    # 第一步：无截断跑一遍收集全部类（用 Two-pass RTA 基准）
    pass1 = bfs(user_class_files, resolver, use_rta=False, shared_cache=sc)
    internal_classes = frozenset(c for c in pass1.all_classes if _is_internal(c))

    # 第二步：以内部类为截断边界重跑
    pass2 = bfs(user_class_files, resolver, use_rta=False, use_vta=True,
                cutoff_classes=internal_classes, shared_cache=sc)
    return bfs(user_class_files, resolver,
               seed_instantiated=pass2.instantiated, use_rta=True, use_vta=True,
               cutoff_classes=internal_classes, shared_cache=sc)


# ─── 调用树打印 ──────────────────────────────────────────────────────────────────

def _get_user_methods(user_class_files: list[str]) -> list[tuple]:
    """从用户 .class 文件提取所有有字节码的方法签名，作为调用树根节点。"""
    roots = []
    for path in user_class_files:
        with open(path, 'rb') as f: data = f.read()
        cls_name, _, methods, _, _, _ = _parse_class(data)
        for m in methods:
            if m[0] != '<clinit>' and m[2]:
                roots.append((cls_name, m[0], m[1]))
    return roots


def print_call_tree(
    roots: list[tuple],
    call_graph: dict,
    native_stubs: set,
    max_depth: int = 8,
) -> None:
    """以树状图打印方法调用链。"""
    shown: set[tuple] = set()  # 已展开过的节点（跨分支去重）

    def _fmt(key: tuple) -> str:
        cls, name, desc = key
        short = cls.split('/')[-1]
        tags = []
        if key in native_stubs:
            tags.append('native')
        if _is_internal(cls):
            tags.append('internal')
        tag_str = '  [' + ', '.join(tags) + ']' if tags else ''
        return f'{short}.{name}{desc}{tag_str}'

    def _recurse(key: tuple, prefix: str, is_last: bool, depth: int,
                 ancestors: frozenset) -> None:
        conn = '└─ ' if is_last else '├─ '
        print(f'{prefix}{conn}{_fmt(key)}')
        child_prefix = prefix + ('   ' if is_last else '│  ')
        children = sorted(call_graph.get(key, set()))
        if not children:
            return
        if key in ancestors:
            print(f'{child_prefix}└─ ↺ 循环引用，略')
            return
        if depth >= max_depth:
            print(f'{child_prefix}└─ … 超过深度限制（{len(children)} 个调用，'
                  f'用 --max-depth 调整）')
            return
        if key in shown:
            print(f'{child_prefix}└─ → 已展开（{len(children)} 个子调用）')
            return
        shown.add(key)
        new_anc = ancestors | {key}
        for i, child in enumerate(children):
            _recurse(child, child_prefix, i == len(children) - 1,
                     depth + 1, new_anc)

    print()
    print('【方法调用链树状图】')
    print()
    for i, root in enumerate(roots):
        cls, name, desc = root
        short = cls.split('/')[-1]
        is_last = i == len(roots) - 1
        conn = '└─ ' if is_last else '├─ '
        print(f'{conn}{short}.{name}{desc}')
        prefix = '   ' if is_last else '│  '
        children = sorted(call_graph.get(root, set()))
        shown.add(root)
        if children:
            new_anc: frozenset = frozenset({root})
            for j, child in enumerate(children):
                _recurse(child, prefix, j == len(children) - 1, 1, new_anc)
    print()
    print('  标注说明：[native] = ACC_NATIVE 方法，需手写实现')
    print('           [internal] = sun/jdk/com.sun 内部类（截断边界）')
    print('           ↺ = 循环引用   → = 已展开（跳过重复子树）')


# ─── 主程序 ─────────────────────────────────────────────────────────────────────
def main():
    import argparse
    ap = argparse.ArgumentParser(
        description='方法级调用链追踪',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='示例:\n'
               '  python3 scripts/trace_callchain.py tests/e2e/01_basics/HelloWorld.java\n'
               '  python3 scripts/trace_callchain.py HelloWorld.java --tree\n'
               '  python3 scripts/trace_callchain.py HelloWorld.java --tree --max-depth 6\n'
               '  python3 scripts/trace_callchain.py HelloWorld.java --tree-only',
    )
    ap.add_argument('files', nargs='+', metavar='FILE.java')
    ap.add_argument('--tree', action='store_true',
                    help='在统计分析后追加树状调用链图')
    ap.add_argument('--tree-only', action='store_true',
                    help='只显示树状调用链图，跳过统计分析（速度更快）')
    ap.add_argument('--max-depth', type=int, default=8, metavar='N',
                    help='树状图最大展开深度（默认 8）')
    args = ap.parse_args()

    show_tree  = args.tree or args.tree_only
    skip_stats = args.tree_only

    java_files = args.files
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

    with JdkResolver(jmods) as resolver:
        shared = {}  # 所有模式共享类解析缓存，避免重复读取字节码

        if skip_stats:
            # --tree-only：只做两遍 RTA，收集调用图，直接打印树
            print('[3/3] Two-pass RTA + 调用图 ...', end=' ', flush=True)
            tree_result, tree_cg = bfs_two_pass_with_graph(
                user_classes, resolver, shared)
            print('完成')
            roots = _get_user_methods(user_classes)
            print_call_tree(roots, tree_cg, tree_result.native_stubs,
                            max_depth=args.max_depth)
            return

        print('[3/N] 单程 RTA ...', end=' ', flush=True)
        r1 = bfs(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r1.all_classes)} 类 / {len(r1.visited)} 方法')

        print('[3/N] Two-pass RTA ...', end=' ', flush=True)
        r2 = bfs_two_pass(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r2.all_classes)} 类 / {len(r2.visited)} 方法')

        print('[3/N] VTA + Two-pass RTA ...', end=' ', flush=True)
        r3 = bfs_vta(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r3.all_classes)} 类 / {len(r3.visited)} 方法')

        print('[3/N] 跨方法返回类型传播（Inter-proc VTA）...', end=' ', flush=True)
        r4 = bfs_interprocedural(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r4.all_classes)} 类 / {len(r4.visited)} 方法')

        print('[3/N] 语义截断（Cutoff）...', end=' ', flush=True)
        r5 = bfs_cutoff(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r5.all_classes)} 类 / {len(r5.visited)} 方法')

        print('[3/N] 内部包边界截断（sun/jdk/com.sun 自动截断）...', end=' ', flush=True)
        r6 = bfs_internal_boundary(user_classes, resolver, shared_cache=shared)
        print(f'完成  {len(r6.all_classes)} 类 / {len(r6.visited)} 方法')

    # ── 综合统计（基于 Two-pass RTA 结果）────────────────────────────────────────
    def _summary(res: BfsResult):
        call_cls = {c for c, _, _ in res.visited}
        return call_cls, res.all_classes - call_cls

    c1, ro1 = _summary(r1)
    c2, ro2 = _summary(r2)
    c3, ro3 = _summary(r3)
    c4, ro4 = _summary(r4)
    c5, ro5 = _summary(r5)
    c6, ro6 = _summary(r6)

    rows = [
        ('单程 RTA',                       r1, c1, ro1),
        ('Two-pass RTA',                   r2, c2, ro2),
        ('VTA + Two-pass RTA',             r3, c3, ro3),
        ('Inter-proc VTA',                 r4, c4, ro4),
        ('Cutoff（手动 11 类）',            r5, c5, ro5),
        ('Internal Boundary（sun/jdk/…）', r6, c6, ro6),
    ]

    # 使用 Two-pass RTA 作为后续所有分析的基准
    vm, vcls, vnat = r2.visited, r2.all_classes, r2.native_stubs

    # ── §1 模式对比汇总表 ─────────────────────────────────────────────────────
    print()
    print('╔═══════════════════════════════════════════════════════════════════════╗')
    print('║               调用链分析报告  —  模式对比汇总                       ║')
    print('╠═══════════════════════════════════════════════════════════════════════╣')
    print('║  模式                    │  调用链类  │  仅引用  │  合计  │  方法  │ native ║')
    print('╠═══════════════════════════════════════════════════════════════════════╣')
    for rname, res, cc, ro in rows:
        print(f'║  {rname:<22}  │  {len(cc):>6}  │  {len(ro):>6}  │  {len(res.all_classes):>5}  │  {len(res.visited):>5} │  {len(res.native_stubs):>4} ║')
    print('╚═══════════════════════════════════════════════════════════════════════╝')

    # ── §2 全链路触达统计（Two-pass RTA 基准）────────────────────────────────
    all_visited_cls   = {c for c, _, _ in vm}
    ref_only_cls      = vcls - all_visited_cls
    clinit_methods    = {(c, n, d) for c, n, d in vm if n == '<clinit>'}
    init_methods      = {(c, n, d) for c, n, d in vm if n == '<init>'}
    static_methods    = {(c, n, d) for c, n, d in vm
                         if n not in ('<init>', '<clinit>')}
    native_cls        = {c for c, _, _ in vnat}
    internal_cls      = {c for c in vcls if _is_internal(c)}
    public_api_cls    = {c for c in all_visited_cls if not _is_internal(c)}
    n_abstract, n_static = _method_flag_stats(vm, shared)

    # 字段/常量/异常引用类（仅引用通道，来自描述符/字段声明/catch/ldc）
    ref_type_cls = ref_only_cls  # 已经是 vcls - visited_cls

    print()
    print('┌─────────────────────────────────────────────────────────────────────┐')
    print('│              § 全链路触达统计（Two-pass RTA 基准）                  │')
    print('├──────────────────────────────────────────┬──────────────────────────┤')
    print(f'│  触达类总数（调用链 + 仅引用）            │  {len(vcls):>6} 个              │')
    print(f'│    ├ 调用链展开类（有方法被调用）          │  {len(all_visited_cls):>6} 个              │')
    print(f'│    │   ├ 公开 API 类（java/javax/）       │  {len(public_api_cls):>6} 个              │')
    print(f'│    │   └ 内部实现类（sun/jdk/com.sun/）   │  {len(internal_cls):>6} 个              │')
    print(f'│    └ 仅引用类（类型/字段/常量/catch引用） │  {len(ref_type_cls):>6} 个              │')
    print('├──────────────────────────────────────────┼──────────────────────────┤')
    print(f'│  触达方法总数                             │  {len(vm):>6} 个              │')
    print(f'│    ├ 普通方法调用（invokevirtual 等）      │  {len(static_methods):>6} 个              │')
    print(f'│    ├ 构造方法 <init>                      │  {len(init_methods):>6} 个              │')
    print(f'│    └ 静态初始化块 <clinit>                │  {len(clinit_methods):>6} 个              │')
    print(f'│    ├ （其中声明为 abstract）              │  {n_abstract:>6} 个              │')
    print(f'│    └ （其中声明为 static）                │  {n_static:>6} 个              │')
    print('├──────────────────────────────────────────┼──────────────────────────┤')
    print(f'│  已触发 <clinit> 初始化的类               │  {len(r2.initialized):>6} 个              │')
    print(f'│  未解析调用目标（找不到声明者）           │  {len(r2.unresolved):>6} 个              │')
    print(f'│  Native 边界方法                          │  {len(vnat):>6} 个              │')
    print(f'│  Native 边界所在类                        │  {len(native_cls):>6} 个              │')
    print('└──────────────────────────────────────────┴──────────────────────────┘')

    if r2.unresolved:
        print()
        print(f'【未解析调用目标（前 20 / 共 {len(r2.unresolved)} 个）】')
        for cls, nm, d in sorted(r2.unresolved)[:20]:
            print(f'  {cls}.{nm}{d}')

    # ── §3 方法类型细分 ───────────────────────────────────────────────────────
    print()
    print('【§3 方法类型细分（Two-pass RTA）】')
    print()

    # 按类统计 <clinit> / <init> / 普通方法
    cls_method_stats: dict[str, dict] = {}
    for cls2, nm2, desc2 in vm:
        s = cls_method_stats.setdefault(cls2, {'clinit': 0, 'init': 0, 'normal': 0, 'native': 0})
        if nm2 == '<clinit>': s['clinit'] += 1
        elif nm2 == '<init>': s['init'] += 1
        else: s['normal'] += 1
        if (cls2, nm2, desc2) in vnat: s['native'] += 1

    print(f'  <clinit> 静态初始化：{len(clinit_methods)} 个，涉及类：')
    for cls2, nm2, desc2 in sorted(clinit_methods):
        print(f'    {cls2}.<clinit>{desc2}')

    print()
    print(f'  <init> 构造方法：{len(init_methods)} 个，涉及类（前 20）：')
    for cls2, nm2, desc2 in sorted(init_methods)[:20]:
        nat_mark = '  [native]' if (cls2, nm2, desc2) in vnat else ''
        print(f'    {cls2}.<init>{desc2}{nat_mark}')
    if len(init_methods) > 20:
        print(f'    …（+{len(init_methods)-20} 个）')

    # ── §4 仅引用类型来源分析 ─────────────────────────────────────────────────
    print()
    print('【§4 仅引用类（字段声明/描述符/catch/ldc 来源，不展开方法体）】')
    print(f'  共 {len(ref_type_cls)} 个：')
    for c in sorted(ref_type_cls)[:40]:
        print(f'  {c}')
    if len(ref_type_cls) > 40:
        print(f'  …（+{len(ref_type_cls)-40} 个，见报告文件）')

    # ── §5 截断候选分析 ───────────────────────────────────────────────────────
    print()
    print('[分析] 计算截断候选（Two-pass RTA 基准）...', end=' ', flush=True)
    dep_graph = build_class_dep_graph(vm, shared)
    all_vm_cls = {c for c, _, _ in vm}
    candidates = analyze_cutoff_candidates(
        vm, vcls, shared, dep_graph,
        seed_classes=all_vm_cls,
        top_n=25,
    )
    print('完成')

    print()
    print('【§5 截断候选 Top 25（按"截断后消除类数"降序）】')
    print(f'  {"排名":<4} {"消除类数":>6}  {"native/总方法":>12}  {"类名"}')
    print(f'  {"─"*4} {"─"*6}  {"─"*12}  {"─"*50}')
    for rank, c in enumerate(candidates, 1):
        leaf_mark = ' ★' if c['is_natural_leaf'] else ''
        print(f'  {rank:<4} {c["elim_count"]:>6}  '
              f'{c["native_count"]:>5}/{c["total_methods"]:<5}  '
              f'{c["cls"]}{leaf_mark}')
        for nm2, desc2 in c['entry_methods'][:3]:
            print(f'       {"":>6}  {"":>12}    └ {nm2}{desc2}')
        if len(c['entry_methods']) > 3:
            print(f'       {"":>6}  {"":>12}    └ （+{len(c["entry_methods"])-3} 个方法）')
    print()
    print('  ★ = 所有被调方法均为 native（天然截断边界，手写成本最低）')

    # ── §6 边界方法（公开API → 内部实现） ────────────────────────────────────
    print()
    print('[分析] 分析公开API→内部类边界方法...', end=' ', flush=True)
    boundary_methods = analyze_internal_boundary(vm, shared)
    print(f'完成  {len(boundary_methods)} 个边界方法，涉及 '
          f'{len({r["cls"] for r in boundary_methods})} 个公开API类')

    print()
    print('【§6 边界方法（公开API中直接调用内部类，需手写native实现）】')
    cur_cls = None
    for r in boundary_methods:
        if r['cls'] != cur_cls:
            cur_cls = r['cls']
            print(f'\n  {cur_cls}')
        sig = f'{r["name"]}{r["desc"]}'
        internals = ', '.join(c.split('/')[-1] for c, _, _ in r['calls_internal'][:3])
        if not internals:
            internals = ', '.join(c.split('/')[-1] for c in r['refs_internal'][:3])
        print(f'    ├ {sig}')
        print(f'    │  → {internals}')

    # ── §7 调用链深度分析 ─────────────────────────────────────────────────────
    print()
    print('[分析] 调用链深度分析...', end=' ', flush=True)
    with JdkResolver(jmods) as resolver2:
        class_depth, layers = bfs_depth_analysis(user_classes, resolver2, shared)
    chain_max_depth = max(layers) if layers else 0
    print(f'完成  最大深度 {chain_max_depth}，涉及 {len(class_depth)} 个类')

    print()
    print('【§7 调用链深度分层（每层新引入的类数）】')
    print(f'  {"深度":>4}  {"本层新增":>8}  {"累计类数":>8}  代表性类（前 3 个）')
    print(f'  {"─"*4}  {"─"*8}  {"─"*8}  {"─"*55}')
    cumulative = 0
    for d in sorted(layers):
        cls_at_d = layers[d]
        cumulative += len(cls_at_d)
        samples = sorted(cls_at_d)[:3]
        sample_str = ', '.join(c.split('/')[-1] for c in samples)
        if len(cls_at_d) > 3:
            sample_str += f', ...(+{len(cls_at_d)-3})'
        print(f'  {d:>4}  {len(cls_at_d):>8}  {cumulative:>8}  {sample_str}')

    # ── §8 Native 边界方法完整列表 ────────────────────────────────────────────
    print()
    print(f'【§8 Native 边界方法完整列表（共 {len(vnat)} 个）】')
    nat_by_cls: dict[str, list] = {}
    for nc, nn, nd in sorted(vnat): nat_by_cls.setdefault(nc, []).append(f'{nn}{nd}')
    for nc in sorted(nat_by_cls):
        print(f'  {nc}')
        for sig in nat_by_cls[nc]: print(f'    └ {sig}')

    # ── 写 Markdown 报告 ──────────────────────────────────────────────────────
    project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    reports_dir  = os.path.join(project_root, 'docs', 'reports')
    os.makedirs(reports_dir, exist_ok=True)
    stem0  = os.path.splitext(os.path.basename(java_files[0]))[0]
    report = os.path.join(reports_dir, f'trace-{stem0}.md')

    from datetime import date
    with open(report, 'w', encoding='utf-8') as f:
        f.write(f'# 调用链追踪：{stem0}\n\n生成时间：{date.today()}\n\n')

        # 模式对比
        f.write('## § 模式对比\n\n')
        f.write('| 模式 | 调用链类 | 仅引用 | 合计 | 方法数 | native |\n')
        f.write('|------|--------:|------:|-----:|-------:|-------:|\n')
        for rname, res, cc, ro in rows:
            f.write(f'| {rname} | {len(cc)} | {len(ro)} | {len(res.all_classes)} | '
                    f'{len(res.visited)} | {len(res.native_stubs)} |\n')
        f.write('\n')

        # 全链路触达统计
        f.write('## § 全链路触达统计（Two-pass RTA 基准）\n\n')
        f.write(f'| 指标 | 数值 |\n|------|-----:|\n')
        f.write(f'| 触达类总数 | {len(vcls)} |\n')
        f.write(f'| 调用链展开类 | {len(all_visited_cls)} |\n')
        f.write(f'| 公开API类（java/javax） | {len(public_api_cls)} |\n')
        f.write(f'| 内部实现类（sun/jdk） | {len(internal_cls)} |\n')
        f.write(f'| 仅引用类 | {len(ref_type_cls)} |\n')
        f.write(f'| 触达方法总数 | {len(vm)} |\n')
        f.write(f'| 普通方法调用 | {len(static_methods)} |\n')
        f.write(f'| 构造方法 `<init>` | {len(init_methods)} |\n')
        f.write(f'| 静态初始化块 `<clinit>` | {len(clinit_methods)} |\n')
        f.write(f'| 已触发 `<clinit>` 初始化的类 | {len(r2.initialized)} |\n')
        f.write(f'| 未解析调用目标 | {len(r2.unresolved)} |\n')
        f.write(f'| abstract 声明方法 | {n_abstract} |\n')
        f.write(f'| static 声明方法 | {n_static} |\n')
        f.write(f'| Native 边界方法 | {len(vnat)} |\n')
        f.write(f'| Native 边界所在类 | {len(native_cls)} |\n\n')

        # <clinit> 清单
        f.write('## § 静态初始化块 `<clinit>` 触达清单\n\n')
        f.write('| 类 | 描述符 |\n|----|---------|\n')
        for cls2, nm2, desc2 in sorted(clinit_methods):
            f.write(f'| `{cls2}` | `{desc2}` |\n')
        f.write('\n')

        # <init> 清单
        f.write('## § 构造方法 `<init>` 触达清单\n\n')
        f.write('| 类 | 描述符 | native |\n|----|---------|:------:|\n')
        for cls2, nm2, desc2 in sorted(init_methods):
            nat_mark = '✓' if (cls2, nm2, desc2) in vnat else ''
            f.write(f'| `{cls2}` | `{desc2}` | {nat_mark} |\n')
        f.write('\n')

        # 仅引用类
        f.write('## § 仅引用类（字段声明/描述符/catch/ldc 来源）\n\n')
        for c in sorted(ref_type_cls): f.write(f'- `{c}`\n')
        f.write('\n')

        # 未解析目标
        if r2.unresolved:
            f.write('## § 未解析调用目标\n\n')
            for cls, nm, d in sorted(r2.unresolved):
                f.write(f'- `{cls}.{nm}{d}`\n')
            f.write('\n')

        # 边界方法
        f.write('## § 边界方法（公开API直接调用内部类，需手写native实现）\n\n')
        f.write('> 采用内部包边界截断策略时，**这些方法需要手写实现**。\n\n')
        prev_cls2 = None
        for r in boundary_methods:
            if r['cls'] != prev_cls2:
                prev_cls2 = r['cls']
                f.write(f'### `{r["cls"]}`\n\n| 方法签名 | 调用的内部类 |\n|---------|------------|\n')
            internals = set(c for c, _, _ in r['calls_internal']) | set(r['refs_internal'])
            internals_str = ', '.join(f'`{c}`' for c in sorted(internals)[:4])
            if len(internals) > 4: internals_str += f', +{len(internals)-4}'
            f.write(f'| `{r["name"]}{r["desc"]}` | {internals_str} |\n')
        f.write('\n')

        # 深度分层
        f.write('## § 调用链深度分层\n\n')
        f.write('| 深度 | 本层新增 | 累计 | 代表性类（前5） |\n|-----:|---------:|-----:|----------------|\n')
        cum = 0
        for d in sorted(layers):
            cls_at_d = layers[d]; cum += len(cls_at_d)
            samples = ', '.join(f'`{c}`' for c in sorted(cls_at_d)[:5])
            if len(cls_at_d) > 5: samples += f', …+{len(cls_at_d)-5}'
            f.write(f'| {d} | {len(cls_at_d)} | {cum} | {samples} |\n')
        f.write('\n')

        # 截断候选
        f.write('## § 截断候选分析（Top 25）\n\n')
        f.write('| 排名 | 消除类数 | native/总方法 | 天然边界 | 类名 | 需手写方法 |\n')
        f.write('|-----:|---------:|-------------:|:--------:|------|----------|\n')
        for rank, c in enumerate(candidates, 1):
            leaf = '★' if c['is_natural_leaf'] else ''
            ms_str = '<br>'.join(f'`{nm2}{desc2}`' for nm2, desc2 in c['entry_methods'][:5])
            if len(c['entry_methods']) > 5: ms_str += f'<br>+{len(c["entry_methods"])-5} more'
            f.write(f'| {rank} | {c["elim_count"]} | {c["native_count"]}/{c["total_methods"]} '
                    f'| {leaf} | `{c["cls"]}` | {ms_str} |\n')
        f.write('\n')

        # Native 边界完整列表
        f.write('## § Native 边界方法完整列表\n\n')
        for nc in sorted(nat_by_cls):
            f.write(f'### `{nc}`\n\n')
            for sig in nat_by_cls[nc]: f.write(f'- `{sig}`\n')
            f.write('\n')

        # Cutoff 模式详细方法列表
        f.write('## § Cutoff 模式调用链方法\n\n')
        by5: dict[str, list[str]] = {}
        for cls2, nm2, desc2 in r5.visited: by5.setdefault(cls2, []).append(f'{nm2}{desc2}')
        for cls2 in sorted(by5):
            f.write(f'### `{cls2}`\n\n')
            for sig in sorted(by5[cls2]): f.write(f'- `{sig}`\n')
            f.write('\n')
        f.write('## § Cutoff 截断边界\n\n')
        for c in sorted(_DEFAULT_CUTOFFS): f.write(f'- `{c}`\n')
        f.write('\n')
        ro5_list = r5.all_classes - {c for c, _, _ in r5.visited}
        if ro5_list:
            f.write('## § 仅引用类（Cutoff 边界内）\n\n')
            for cls2 in sorted(ro5_list): f.write(f'- `{cls2}`\n')

    print(f'\n详细报告 → {report}')

    # ── 树状调用链图（--tree 模式） ──────────────────────────────────────────────
    if show_tree:
        print()
        print('[分析] 构建调用图（Two-pass RTA）...', end=' ', flush=True)
        with JdkResolver(jmods) as resolver3:
            shared3: dict = {}
            tree_result, tree_cg = bfs_two_pass_with_graph(
                user_classes, resolver3, shared3)
        print('完成')
        roots = _get_user_methods(user_classes)
        print_call_tree(roots, tree_cg, tree_result.native_stubs,
                        max_depth=args.max_depth)


if __name__ == '__main__':
    main()

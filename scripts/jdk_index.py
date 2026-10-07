#!/usr/bin/env python3
"""JDK 类层次索引（jdk_index）——从 $JAVA_HOME/jmods 读全部类的继承关系与方法表，把常量池符号引用
（owner 是静态接收者类型）解析到**声明类**，供 API 面（api_surface.py）与闭包方法键对齐。

为什么要解析：常量池 Methodref 的 owner 是编译期接收者类型（`LinkedHashMap.put`），rava 闭包的方法键是
声明类（`HashMap.put`）；不解析时同一方法在两侧对不上。解析口径取 JVMS §5.4.3.3 / §5.4.3.4 的简化版：
先自身与超类链，再超接口（广度优先，取首个非 abstract 声明，无则取首个声明），接口引用最后落 Object。
签名多态方法（MethodHandle.invoke 等）与解析不到的引用原样保留。

索引缓存：build/api_surface/jdk_index-<主版本>.pickle（jmods 的 mtime 变化即重建）。
"""

import pickle
import struct
import zipfile
from collections import deque
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CACHE_DIR = ROOT / "build/api_surface"

ACC_PUBLIC, ACC_PROTECTED, ACC_STATIC, ACC_INTERFACE, ACC_ABSTRACT = 0x0001, 0x0004, 0x0008, 0x0200, 0x0400

_TAG_SIZE = {3: 4, 4: 4, 5: 8, 6: 8, 7: 2, 8: 2, 9: 4, 10: 4,
             11: 4, 12: 4, 15: 3, 16: 2, 17: 4, 18: 4, 19: 2, 20: 2}


def parse_class(data: bytes):
    """.class → (名, 访问标志, 超类, 接口列表, {(方法名, 描述符): 访问标志})；非类文件返回 None。"""
    if data[:4] != b"\xca\xfe\xba\xbe":
        return None
    pos, count = 10, struct.unpack(">H", data[8:10])[0]
    utf8, cls = {}, {}
    i = 1
    while i < count:
        tag = data[pos]
        pos += 1
        if tag == 1:
            n = struct.unpack(">H", data[pos:pos + 2])[0]
            utf8[i] = data[pos + 2:pos + 2 + n].decode("utf-8", "replace")
            pos += 2 + n
        else:
            if tag == 7:
                cls[i] = struct.unpack(">H", data[pos:pos + 2])[0]
            pos += _TAG_SIZE[tag]
        i += 2 if tag in (5, 6) else 1

    def cname(idx):
        return utf8.get(cls.get(idx, 0)) if idx else None

    access, this_i, super_i, n_if = struct.unpack(">HHHH", data[pos:pos + 8])
    pos += 8
    ifaces = [cname(struct.unpack(">H", data[pos + 2 * k:pos + 2 * k + 2])[0]) for k in range(n_if)]
    pos += 2 * n_if

    def skip_attrs(p):
        n = struct.unpack(">H", data[p:p + 2])[0]
        p += 2
        for _ in range(n):
            p += 6 + struct.unpack(">I", data[p + 2:p + 6])[0]
        return p

    n_fields = struct.unpack(">H", data[pos:pos + 2])[0]
    pos += 2
    for _ in range(n_fields):
        pos = skip_attrs(pos + 6)
    n_methods = struct.unpack(">H", data[pos:pos + 2])[0]
    pos += 2
    methods = {}
    for _ in range(n_methods):
        m_acc, m_name, m_desc = struct.unpack(">HHH", data[pos:pos + 6])
        methods[(utf8[m_name], utf8[m_desc])] = m_acc
        pos = skip_attrs(pos + 6)
    return cname(this_i), access, cname(super_i), ifaces, methods


class JdkIndex:
    """JDK 类 → (访问标志, 超类, 接口, 方法表, 模块)。"""

    def __init__(self, classes: dict):
        self.classes = classes

    @staticmethod
    def load(java_home: Path) -> "JdkIndex":
        jmods = sorted((java_home / "jmods").glob("*.jmod"))
        if not jmods:
            raise SystemExit(f"[jdk-index] {java_home}/jmods 下没有 jmod")
        stamp = max(j.stat().st_mtime for j in jmods)
        CACHE_DIR.mkdir(parents=True, exist_ok=True)
        cache = CACHE_DIR / f"jdk_index-{java_home.name}.pickle"
        if cache.exists():
            st, classes = pickle.loads(cache.read_bytes())
            if st == stamp:
                return JdkIndex(classes)
        classes = {}
        for jm in jmods:
            module = jm.stem
            with zipfile.ZipFile(jm) as z:
                for name in z.namelist():
                    if not (name.startswith("classes/") and name.endswith(".class")) or name.endswith("module-info.class"):
                        continue
                    parsed = parse_class(z.read(name))
                    if parsed:
                        this, acc, sup, ifs, methods = parsed
                        classes[this] = (acc, sup, ifs, methods, module)
        cache.write_bytes(pickle.dumps((stamp, classes)))
        return JdkIndex(classes)

    def has(self, cls: str) -> bool:
        return cls in self.classes

    def module_of(self, cls: str):
        c = self.classes.get(cls)
        return c[4] if c else None

    def method_access(self, cls: str, name: str, desc: str):
        c = self.classes.get(cls)
        return c[3].get((name, desc)) if c else None

    def is_public_api(self, cls: str, name: str, desc: str) -> bool:
        """公开面：public 类的 public / protected 方法（包导出与否不判，内部包由调用方按前缀筛）"""
        c = self.classes.get(cls)
        if not c or not c[0] & ACC_PUBLIC:
            return False
        acc = c[3].get((name, desc))
        return acc is not None and bool(acc & (ACC_PUBLIC | ACC_PROTECTED))

    def resolve(self, owner: str, name: str, desc: str) -> str:
        """符号引用 → 声明类（解析不到原样返回 owner）。构造器 / 类初始化不沿继承链。"""
        if owner not in self.classes or name in ("<init>", "<clinit>"):
            return owner
        cur = owner
        while cur:
            c = self.classes.get(cur)
            if not c:
                break
            if (name, desc) in c[3]:
                return cur
            cur = c[1]
        # 超接口：广度优先，非 abstract（默认方法）优先
        seen, queue, first = set(), deque(), None
        cur = owner
        while cur:
            c = self.classes.get(cur)
            if not c:
                break
            queue.extend(c[2])
            cur = c[1]
        while queue:
            it = queue.popleft()
            if it in seen or it not in self.classes:
                continue
            seen.add(it)
            ic = self.classes[it]
            acc = ic[3].get((name, desc))
            if acc is not None and not acc & ACC_STATIC:
                if not acc & ACC_ABSTRACT:
                    return it
                first = first or it
            queue.extend(ic[2])
        if first:
            return first
        if ("java/lang/Object" in self.classes
                and (name, desc) in self.classes["java/lang/Object"][3]):
            return "java/lang/Object"
        return owner

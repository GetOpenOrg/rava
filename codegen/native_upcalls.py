"""手写 native / 内部边界方法的 Java 回调（upcall）声明扫描。

JVM 里 native 方法经 JNI 回调 Java（`CallVoidMethod` / `NewObject`）；这些调用边
不在任何字节码里，BFS 无从发现。手写层用属性参数显式声明：

    #[jvm_native(upcalls = "pkg/Cls.<init>:(I)V pkg/Other.method:([BII)V")]
    pub fn member(...) { ... }

`upcalls` 为空白分隔的 `类.方法:描述符` 列表（与字节码常量池方法引用同一写法）。
BFS 触达该成员（方法引用或静态字段引用）时，把声明的回调目标入队，
使其字节码被翻译而不是停留在 panic! 存根。

本模块只做「类 binary name → 共置 `_impl.rs` → {fn 名: [回调目标]}」的解析，
不含任何类名常量。
"""
import os
import re

from .emitter.attrs import to_snake

_UPCALL_ATTR_RE = re.compile(
    r'#\[\s*jvm_(?:native|boundary|ext)\s*\(\s*upcalls\s*=\s*"([^"]*)"\s*\)\s*\]'
    r'\s*(?:#\[[^\]]*\]\s*)*pub\s+fn\s+(\w+)',
    re.S,
)
_CTOR_RUST_NAME = 'new'


def _parse_target(tok: str) -> 'tuple[str, str, str] | None':
    colon = tok.find(':')
    dot = tok.rfind('.', 0, colon) if colon > 0 else -1
    if dot <= 0:
        return None
    return tok[:dot], tok[dot + 1:colon], tok[colon + 1:]


class NativeUpcalls:
    """按类惰性解析共置 `_impl.rs` 的 upcall 声明。"""

    def __init__(self, runtime_src: str):
        self._src = runtime_src
        self._cache: dict[str, dict[str, list]] = {}

    def _load(self, cls: str) -> dict:
        if cls in self._cache:
            return self._cache[cls]
        *pkg, simple = cls.split('/')
        table: dict[str, list] = {}
        for suffix in ('_impl.rs', '_ext.rs'):
            path = os.path.join(self._src, *pkg, to_snake(simple) + suffix)
            if not os.path.isfile(path):
                continue
            try:
                content = open(path, encoding='utf-8').read()
            except OSError:
                continue
            for m in _UPCALL_ATTR_RE.finditer(content):
                targets = [t for t in (_parse_target(x) for x in m.group(1).split()) if t]
                table.setdefault(m.group(2), []).extend(targets)
        self._cache[cls] = table
        return table

    def lookup(self, cls: str, member: str) -> list:
        """成员 member（Java 名；构造器为 <init>）对应的手写 fn 声明的全部回调目标。

        手写 fn 名 = Java 名，或 Java 名 + 重载后缀（`name_<suffix>`）。
        """
        table = self._load(cls)
        if not table:
            return []
        rust = _CTOR_RUST_NAME if member == '<init>' else member
        out: list = []
        for fn_name, targets in table.items():
            if fn_name == rust or fn_name.startswith(rust + '_'):
                out.extend(targets)
        return out

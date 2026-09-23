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
_PUB_FN_RE = re.compile(r'\bpub fn\s+(\w+)\s*[(<]')
_CTOR_RUST_NAME = 'new'
_VIRTUAL_BODY_PREFIX = '__impl_'


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
        # 共置手写文件的全部 pub fn 名（成员覆盖判定用，见 provides）
        self._fns: dict[str, set[str]] = {}

    def _load(self, cls: str) -> dict:
        if cls in self._cache:
            return self._cache[cls]
        *pkg, simple = cls.split('/')
        table: dict[str, list] = {}
        fns: set[str] = set()
        for suffix in ('_impl.rs', '_ext.rs'):
            path = os.path.join(self._src, *pkg, to_snake(simple) + suffix)
            if not os.path.isfile(path):
                continue
            try:
                content = open(path, encoding='utf-8').read()
            except OSError:
                continue
            # 自动生成的类文件碰巧以 _impl.rs 结尾（如 Collectors$CollectorImpl，
            # 与 _scan_impl_files 同判据）：scratch 复用模式下上一轮的生成残留
            # 会伪装成手写 impl，污染 has_impls / provides 的成员覆盖判定。
            if 'java_rta_macros::java_class' in content:
                continue
            fns.update(_PUB_FN_RE.findall(content))
            for m in _UPCALL_ATTR_RE.finditer(content):
                targets = [t for t in (_parse_target(x) for x in m.group(1).split()) if t]
                table.setdefault(m.group(2), []).extend(targets)
        self._cache[cls] = table
        self._fns[cls] = fns
        return table

    def lookup(self, cls: str, member: str) -> list:
        """成员 member（Java 名；构造器为 <init>）对应的手写 fn 声明的全部回调目标。

        手写 fn 名 = Java 名，或 Java 名 + 重载后缀（`name_<suffix>`）；虚方法体以
        `__impl_<名>` 形式手写（经 vtable 分派到此），同样计入。
        """
        table = self._load(cls)
        if not table:
            return []
        rust = _CTOR_RUST_NAME if member == '<init>' else member
        out: list = []
        for fn_name, targets in table.items():
            fn_name = fn_name.removeprefix(_VIRTUAL_BODY_PREFIX)
            if fn_name == rust or fn_name.startswith(rust + '_'):
                out.extend(targets)
        return out

    def provides(self, cls: str, member: str) -> bool:
        """成员 member 是否由共置手写文件提供（`pub fn` 名成员级匹配）。

        发射侧的同源判定是 class_writer 的 `_nf_covered`：按 mangle 后的 Rust
        精确名（含重载后缀）跳过生成。此处按成员名前缀过近似（`name` 或
        `name_<重载后缀>`）——不做 mangle 复刻（那需要 emit 期的 registry）。
        过近似方向安全：把「同名其它重载」误判为已覆盖只是维持既有 panic
        存根（现状，无回归、不扩闭包）；漏判才可能扰动，而漏判仅发生在
        手写文件声明了完全无关名字时（不可能匹配到 member 前缀）。
        """
        self._load(cls)
        rust = _CTOR_RUST_NAME if member == '<init>' else member
        for fn_name in self._fns.get(cls, ()):
            fn_name = fn_name.removeprefix(_VIRTUAL_BODY_PREFIX)
            if fn_name == rust or fn_name.startswith(rust + '_'):
                return True
        return False

    def has_impls(self, cls: str) -> bool:
        """类是否有共置手写文件（含至少一个 `pub fn`）。

        「按需推进手写」的边界类（CLAUDE.md 3b）：impl 文件存在 = 该类处于
        增量手写管理中，语料版本演化（如 JDK25 新方法）造成的成员缺口由
        生成器按调用边补译；impl 文件不存在 = 该类仍处整体 panic 存根节奏
        （未实现即如实存根），不因补扫提前展开。
        """
        self._load(cls)
        return bool(self._fns.get(cls))

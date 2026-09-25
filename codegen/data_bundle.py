"""纯数据资源束类的结构判定（L-1：Locale 数据改由 CLDR 字节码翻译供给）。

CLDR 本地化数据在 JDK 中即字节码：`sun/text/resources/cldr/**/FormatData*` 等
ListResourceBundle 子类，只有构造器与 `getContents()`，后者是纯字面量数组构造
（常量装载 + anewarray + aastore，无任何方法调用）。这类「纯数据类」即便位于
内部包也照常翻译——数据不是实现细节，手写复刻反而违反原则 1。

判定按**结构**而非类名（原则 4）：载体（类 + 方法签名）来自 runtime 清单
`data_bundle_carriers.txt`；类本身的方法集与载体方法体的操作码白名单在此检查。
"""
from __future__ import annotations

from .runtime_manifest import read_list

# 载体方法体允许的操作码：常量装载、数组构造与元素存储、局部变量存取、返回
_DATA_OPCODES = frozenset({
    'aconst_null', 'iconst_m1', 'iconst_0', 'iconst_1', 'iconst_2', 'iconst_3',
    'iconst_4', 'iconst_5', 'bipush', 'sipush', 'ldc', 'ldc_w', 'ldc2_w',
    'anewarray', 'dup', 'aastore',
    'aload', 'aload_0', 'aload_1', 'aload_2', 'aload_3',
    'astore', 'astore_0', 'astore_1', 'astore_2', 'astore_3',
    'areturn',
})
# 构造器 / 类初始化器允许的额外操作：调用父类构造器、return
_CTOR_OPCODES = _DATA_OPCODES | frozenset({'invokespecial', 'return'})

_CARRIERS: 'dict[str, tuple[str, str]] | None' = None


def _carriers() -> dict[str, tuple[str, str]]:
    """载体类 binary name → (方法名, 描述符)。"""
    global _CARRIERS
    if _CARRIERS is None:
        _CARRIERS = {}
        for line in read_list('data_bundle_carriers.txt'):
            cls_m, desc = line.split(':', 1)
            cls, mname = cls_m.rsplit('.', 1)
            _CARRIERS[cls] = (mname, desc)
    return _CARRIERS


def _opcodes_ok(method, allowed: frozenset) -> bool:
    for ins in method.instrs or []:
        if (ins.opcode or '') not in allowed:
            return False
    return True


def carrier_of(ci, load) -> 'tuple[str, str] | None':
    """ci 的超类链上的数据载体 (方法名, 描述符)；不经载体 → None。
    load(binary_name) → ClassInfo | None（按需解析，不入生成范围）。"""
    carriers = _carriers()
    cur, seen = ci.super_class, set()
    while cur and cur not in seen:
        seen.add(cur)
        if cur in carriers:
            return carriers[cur]
        sci = load(cur)
        if sci is None:
            return None
        cur = sci.super_class
    return None


def is_pure_data_bundle(ci, load) -> bool:
    """ci 是否为纯数据资源束类（见模块说明）。"""
    if ci is None or ci.is_interface:
        return False
    carrier = carrier_of(ci, load)
    if carrier is None:
        return False
    mname, desc = carrier
    has_carrier = False
    for m in ci.methods:
        if m.name in ('<init>', '<clinit>'):
            if not _opcodes_ok(m, _CTOR_OPCODES):
                return False
        elif m.name == mname and m.descriptor == desc and not m.is_static:
            if not _opcodes_ok(m, _DATA_OPCODES):
                return False
            has_carrier = True
        else:
            return False
    return has_carrier

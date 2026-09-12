"""
方法分类器：对 RTA 可达方法集中每个方法打 MethodCategory 标签。

优先级：MANUAL_OVERRIDE > NATIVE_STUB > UNSUPPORTED > AUTO_TRANSPILE
"""

from __future__ import annotations
from dataclasses import dataclass, field
from enum import Enum
from typing import Optional
from .types import ParsedMethod


class MethodCategory(Enum):
    AUTO_TRANSPILE  = "auto_transpile"   # 字节码规整，可直接转译
    NATIVE_STUB     = "native_stub"      # ACC_NATIVE，需手填
    UNSUPPORTED     = "unsupported"      # invokedynamic/MethodHandle 等
    MANUAL_OVERRIDE = "manual_override"  # java_runtime 手写层已覆盖


# java_runtime 手写层已覆盖的方法（class.method 格式）
# 格式：java/lang/System.println → MANUAL_OVERRIDE
_MANUAL_REGISTRY: set[str] = {
    "java/lang/System.currentTimeMillis",
    "java/lang/System.nanoTime",
    "java/lang/System.exit",
    "java/io/PrintStream.println",
    "java/io/PrintStream.print",
    "java/lang/Math.abs",
    "java/lang/Math.sqrt",
    "java/lang/Math.pow",
    "java/lang/Math.floor",
    "java/lang/Math.ceil",
    "java/lang/Math.round",
    "java/lang/Math.min",
    "java/lang/Math.max",
    "java/lang/Math.log",
    "java/lang/Math.sin",
    "java/lang/Math.cos",
    "java/lang/Math.tan",
}

# invokedynamic / MethodHandle 相关 opcode（触发 UNSUPPORTED）
_UNSUPPORTED_OPCODES = frozenset({
    'invokedynamic',
})


@dataclass
class ClassifiedMethod:
    class_name:  str
    method_name: str
    descriptor:  str
    category:    MethodCategory
    reason:      str = ''       # UNSUPPORTED 时说明原因
    suggested_impl: str = ''    # NATIVE_STUB 时的建议 Rust 实现路径
    status:      str = 'todo'   # todo / done / wont_implement


def classify(method: ParsedMethod, class_name: str) -> ClassifiedMethod:
    """对单个方法进行分类。"""
    key = f"{class_name.replace('.', '/')}.{method.name}"

    # 优先级 1：手写层已覆盖
    if key in _MANUAL_REGISTRY:
        return ClassifiedMethod(
            class_name=class_name,
            method_name=method.name,
            descriptor=method.descriptor,
            category=MethodCategory.MANUAL_OVERRIDE,
            status='done',
        )

    # 优先级 2：扫描字节码，寻找不支持的指令
    for ins in method.instrs:
        if ins.opcode in _UNSUPPORTED_OPCODES:
            return ClassifiedMethod(
                class_name=class_name,
                method_name=method.name,
                descriptor=method.descriptor,
                category=MethodCategory.UNSUPPORTED,
                reason=ins.opcode,
                status='todo',
            )

    # 优先级 3：默认可自动转译
    return ClassifiedMethod(
        class_name=class_name,
        method_name=method.name,
        descriptor=method.descriptor,
        category=MethodCategory.AUTO_TRANSPILE,
        status='todo',
    )


def classify_class(class_name: str, methods: list[ParsedMethod]) -> list[ClassifiedMethod]:
    """对一个类的所有方法批量分类。"""
    return [classify(m, class_name) for m in methods]


def print_classification_table(results: list[ClassifiedMethod]):
    """打印分类结果摘要表（调试/验证用）。"""
    from collections import Counter
    counts = Counter(r.category.value for r in results)
    print(f"\n{'─'*60}")
    print(f"{'方法分类摘要':}")
    print(f"{'─'*60}")
    for cat, n in sorted(counts.items()):
        print(f"  {cat:<20s} {n:>4d} 个")
    print(f"{'─'*60}")
    for r in results:
        tag = r.category.value[:2].upper()
        reason = f"  [{r.reason}]" if r.reason else ''
        print(f"  [{tag}] {r.class_name}.{r.method_name}{r.descriptor}{reason}")

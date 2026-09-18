"""
codegen.instr 子包：JVM 字节码指令 → Rust 语句转换。

对外公开接口：sim_instr（主分发函数）以及常用辅助符号。
"""

from .sim import sim_instr
from .coerce import (
    parse_method_ref, _coerce_from_null, _coerce_to_object,
    _coerce_value, _find_method_super_prefix,
    _find_super_chain_to_class, _parse_field_ref, _class_known,
    _mangle_if_overloaded, BOXING_SKIP_STATIC, UNBOX_VIRTUAL,
)

__all__ = ['sim_instr']

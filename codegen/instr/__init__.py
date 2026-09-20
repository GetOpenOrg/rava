"""
codegen.instr 子包：JVM 字节码指令 → Rust 语句转换。

对外公开接口：sim_instr（主分发函数）以及常用辅助符号。
"""

from .sim import sim_instr
from .coerce import (
    _coerce_from_null, _coerce_to_object,
    _coerce_value,
)
from .hierarchy import _find_super_chain_to_class
from .member_owner import (
    parse_method_ref, _find_method_super_prefix,
)
from .member_naming import (
    _parse_field_ref, _class_known, _mangle_if_overloaded,
)

__all__ = ['sim_instr']

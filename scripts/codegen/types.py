"""
数据结构：JVM 字节码解析结果的核心类型。
"""

from dataclasses import dataclass
from typing import Optional


@dataclass
class Instr:
    offset:  int
    opcode:  str
    operand: Optional[str] = None
    comment: Optional[str] = None


@dataclass
class FieldInfo:
    name:       str
    descriptor: str
    is_static:  bool = False


@dataclass
class ParsedMethod:
    class_name:   str
    name:         str
    descriptor:   str
    is_static:    bool
    locals_count: int
    args_size:    int
    instrs:       list

    @property
    def param_types(self):
        from .type_map import parse_descriptor_params
        return parse_descriptor_params(self.descriptor)

    @property
    def return_type(self):
        from .type_map import parse_descriptor_return
        return parse_descriptor_return(self.descriptor)

    @property
    def is_constructor(self):
        return self.name in ('<init>', self.class_name)


@dataclass
class ClassInfo:
    name:    str
    fields:  list   # list[FieldInfo]
    methods: list   # list[ParsedMethod]


@dataclass
class LoopInfo:
    start_idx:   int
    end_idx:     int
    cond_idx:    int
    exit_offset: int

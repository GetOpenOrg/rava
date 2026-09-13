"""
数据结构：JVM 字节码解析结果的核心类型。
"""

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class Instr:
    offset:  int
    opcode:  str
    operand: Optional[str] = None
    comment: Optional[str] = None


@dataclass
class FieldInfo:
    name:              str
    descriptor:        str
    is_static:         bool = False
    access_flags:      int  = 0
    generic_signature: str  = ''


@dataclass
class ParsedMethod:
    class_name:        str
    name:              str
    descriptor:        str
    is_static:         bool
    locals_count:      int
    args_size:         int
    instrs:            list
    local_names:       dict = None
    local_types:       dict = None   # slot → Signature string (LocalVariableTypeTable)
    access_flags:      int  = 0
    is_native:         bool = False
    is_abstract:       bool = False
    is_synthetic:      bool = False
    exceptions:        list = None   # list[str] binary names
    generic_signature: str  = ''

    def __post_init__(self):
        if self.local_names is None:
            self.local_names = {}
        if self.local_types is None:
            self.local_types = {}
        if self.exceptions is None:
            self.exceptions = []

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
    name:              str
    fields:            list
    methods:           list
    super_class:       str  = ''
    interfaces:        list = None   # list[str] binary names
    access_flags:      int  = 0
    is_interface:      bool = False
    is_abstract:       bool = False
    is_enum:           bool = False
    generic_signature: str  = ''
    source_file:       str  = ''

    def __post_init__(self):
        if self.interfaces is None:
            self.interfaces = []


@dataclass
class LoopInfo:
    start_idx:   int
    end_idx:     int
    cond_idx:    int
    exit_offset: int

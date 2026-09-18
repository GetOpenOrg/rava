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
class InnerClassInfo:
    """内部类关系（InnerClasses attribute 中的一条记录）。"""
    inner_class:  str        # binary name，如 java/util/HashMap$Node
    outer_class:  str = ''   # 外围类 binary name；匿名类为空
    inner_name:   str = ''   # 简单名称；匿名类为空
    access_flags: int = 0


@dataclass
class FieldInfo:
    name:              str
    descriptor:        str
    is_static:         bool = False
    access_flags:      int  = 0
    generic_signature: str  = ''
    constant_value:    str  = ''   # static final 字段的字面量（ConstantValue attribute）
    is_deprecated:     bool = False


@dataclass
class ParsedMethod:
    class_name:         str
    name:               str
    descriptor:         str
    is_static:          bool
    locals_count:       int
    args_size:          int
    instrs:             list
    local_names:        dict = None
    local_types:        dict = None   # slot → (Signature string, start_pc) from LocalVariableTypeTable
    access_flags:       int  = 0
    is_native:          bool = False
    is_abstract:        bool = False
    is_synthetic:       bool = False
    exceptions:         list = None   # list[str] binary names（Exceptions attribute）
    generic_signature:  str  = ''
    is_deprecated:      bool = False
    method_parameters:  list = None   # list of (name: str, access_flags: int)
    # vtable 归属：空串=非虚方法; 等于 class_rust_name=新虚方法定义; 其他=覆盖哪个祖先类的 vtable
    virtual_in:         str  = ''

    def __post_init__(self):
        if self.local_names is None:
            self.local_names = {}
        if self.local_types is None:
            self.local_types = {}
        if self.exceptions is None:
            self.exceptions = []
        if self.method_parameters is None:
            self.method_parameters = []

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
    inner_classes:     list = None   # list[InnerClassInfo]
    is_deprecated:     bool = False

    def __post_init__(self):
        if self.interfaces is None:
            self.interfaces = []
        if self.inner_classes is None:
            self.inner_classes = []


@dataclass
class LoopInfo:
    start_idx:   int
    end_idx:     int
    cond_idx:    int | None   # None 表示无条件循环（for(;;) / while(true)）
    exit_offset: int | None   # None 表示无退出条件

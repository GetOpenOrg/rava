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
class AnnoElem:
    """注解元素值对（RuntimeVisibleAnnotations 的 element_value_pair）。

    tag 为 JVMS element_value 标签字符：z/b/c/s/i/j/f/d（基本类型）/
    g（class）/e（enum）/s（String）/a（数组）/@（嵌套注解）/x（未支持形态）。
    value 为编码后载荷文本（编码在 classfile.encode_element_value —— 转义
    `\\ ; # = "`，数组/嵌套递归同编码；attrs/build.rs/运行时三侧原样透传，
    java_runtime::annotation 的解码端按同一转义表还原）。
    """
    name:  str
    tag:   str
    value: str


@dataclass
class AnnoInfo:
    """单条 RuntimeVisibleAnnotations 记录（挂载于类/方法/字段三处）。"""
    type_bin:  str        # 注解类型 binary name（`L..;` 描述符已剥壳）
    elements:  list = None  # list[AnnoElem]，声明序

    def __post_init__(self):
        if self.elements is None:
            self.elements = []


@dataclass
class FieldInfo:
    name:              str
    descriptor:        str
    is_static:         bool = False
    access_flags:      int  = 0
    generic_signature: str  = ''
    constant_value:    str  = ''   # static final 字段的字面量（ConstantValue attribute）
    is_deprecated:     bool = False
    runtime_annotations: list = None   # list[AnnoInfo]（RuntimeVisibleAnnotations）


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
    # 按作用域区间的局部变量声明表：list of (slot, start_pc, length, name, descriptor, signature)
    local_vars:         list = None
    # 异常表：list of (start_pc, end_pc, handler_pc, catch_type binary name | None)
    exception_table:    list = None
    access_flags:       int  = 0
    is_native:          bool = False
    is_abstract:        bool = False
    is_synthetic:       bool = False
    exceptions:         list = None   # list[str] binary names（Exceptions attribute）
    generic_signature:  str  = ''
    is_deprecated:      bool = False
    method_parameters:  list = None   # list of (name: str, access_flags: int)
    runtime_annotations: list = None  # list[AnnoInfo]（RuntimeVisibleAnnotations）
    annotation_default: tuple = None  # (tag, 编码载荷)（AnnotationDefault，仅注解类型方法）
    # vtable 归属：空串=非虚方法; 等于 class_rust_name=新虚方法定义; 其他=覆盖哪个祖先类的 vtable
    virtual_in:         str  = ''
    # 槽位名解耦（覆盖条目 wrapper 名 ≠ 祖先 vtable trait 槽位名时）：trait 成员名，
    # 与继承成员（inherited_gen）的 vtable_name 同一机制。wrapper 名按本类重载态
    # （hierarchy_overloaded_names 单一权威），trait 槽位名按槽位声明者的重载态。
    vtable_name:        str  = ''
    # Code attribute 的异常表：list of (start_pc, end_pc, handler_pc, catch_type)，
    # catch_type 为类二进制名，空串 = catch-all
    exception_table:    list = None

    def __post_init__(self):
        if self.exception_table is None:
            self.exception_table = []
        if self.runtime_annotations is None:
            self.runtime_annotations = []
        if self.local_names is None:
            self.local_names = {}
        if self.local_types is None:
            self.local_types = {}
        if self.local_vars is None:
            self.local_vars = []
        if self.exception_table is None:
            self.exception_table = []
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
    major_version:     int  = 0      # class 文件主版本（45–70），0 = 未记录
    is_deprecated:     bool = False
    # EnclosingMethod attribute（仅局部类 / 匿名类有）：直接外围类 binary name；
    # enclosing_method = (name, descriptor)，位于初始化器中时为 None
    enclosing_class:   str  = ''
    enclosing_method:  tuple = None
    runtime_annotations: list = None   # list[AnnoInfo]（RuntimeVisibleAnnotations）

    def __post_init__(self):
        if self.interfaces is None:
            self.interfaces = []
        if self.inner_classes is None:
            self.inner_classes = []
        if self.runtime_annotations is None:
            self.runtime_annotations = []

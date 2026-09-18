"""
Rust IR（中间表示）数据结构。

代码生成不直接拼接字符串，而是先构建这些 IR 节点，再由 render.py 统一输出。
这样便于后续做分析、优化和格式化。
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import Optional, Union


# ═══════════════════════════════════════════════════════════════════════════════
# 类型节点
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class RsPrimitive:
    name: str  # i32 / i64 / f32 / f64 / bool / usize / ()


@dataclass
class RsNamed:
    """结构体/枚举名，可带 path 前缀（如 java_runtime::java::lang::String）。"""
    name: str
    path: str = ''   # 非空时输出 path::name

    @property
    def full(self) -> str:
        return f'{self.path}::{self.name}' if self.path else self.name


@dataclass
class RsRef:
    inner: RsType
    mutable: bool = False
    lifetime: str = ''   # 非空时输出 &'lt


@dataclass
class RsSlice:
    elem: RsType         # &[T]


@dataclass
class RsGeneric:
    outer: str           # Vec / Rc / RefCell / Option / HashMap …
    params: list[RsType] = field(default_factory=list)


@dataclass
class RsTuple:
    elems: list[RsType] = field(default_factory=list)


@dataclass
class RsInfer:
    """Rust `_` 类型推断占位符，用于集合泛型参数初始占位。"""
    pass


RsType = Union[RsPrimitive, RsNamed, RsRef, RsSlice, RsGeneric, RsTuple, RsInfer]

# 常用类型快捷构造
I32   = RsPrimitive('i32')
I64   = RsPrimitive('i64')
F32   = RsPrimitive('f32')
F64   = RsPrimitive('f64')
Bool  = RsPrimitive('bool')
Usize = RsPrimitive('usize')
Unit  = RsPrimitive('()')


# ═══════════════════════════════════════════════════════════════════════════════
# 表达式节点
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class Lit:
    """字面量：整数、浮点、bool、字符串。"""
    value: str   # 原样输出，如 "42i32", "true", '"hello".to_string()'


@dataclass
class Var:
    name: str


@dataclass
class BinOp:
    op: str        # + - * / % & | ^ << >> == != < > <= >=
    left: RsExpr
    right: RsExpr


@dataclass
class UnOp:
    op: str        # - !
    expr: RsExpr


@dataclass
class Call:
    """自由函数调用：path(args)。"""
    func: str
    args: list[RsExpr] = field(default_factory=list)


@dataclass
class MethodCall:
    """方法调用：recv.method(args)。"""
    recv: RsExpr
    method: str
    args: list[RsExpr] = field(default_factory=list)


@dataclass
class FieldAccess:
    recv: RsExpr
    field_name: str


@dataclass
class Index:
    recv: RsExpr
    idx: RsExpr


@dataclass
class Cast:
    expr: RsExpr
    ty: RsType


@dataclass
class RefExpr:
    expr: RsExpr
    mutable: bool = False


@dataclass
class DerefExpr:
    expr: RsExpr


@dataclass
class BlockExpr:
    """内联块表达式 { stmts; tail }"""
    stmts: list[RsStmt] = field(default_factory=list)
    tail: Optional[RsExpr] = None


@dataclass
class IfExpr:
    """if/else 表达式（返回值）。"""
    cond: RsExpr
    then: BlockExpr
    else_: Optional[BlockExpr] = None


@dataclass
class MacroExpr:
    """宏调用：println!(...) / todo!() / compile_error!(...)"""
    name: str
    args: list[str] = field(default_factory=list)  # 原始字符串参数


@dataclass
class RawExpr:
    """逃生舱：直接插入原始表达式字符串（过渡期使用）。"""
    code: str


@dataclass
class NewPendingExpr:
    """对应 JVM `new` 指令。等待后续 invokespecial <init> 确定构造参数。
    渲染为 ClassName::new()，其中 ClassName 取 class_name 斜杠分隔的最后一段。
    """
    class_name: str  # JVM 内部格式，如 "java/util/ArrayList"


@dataclass
class StaticFieldRef:
    """对应 `getstatic` 指令。渲染为 ClassName::field_name()。"""
    class_name: str       # JVM 内部格式，如 "java/lang/System"
    field_name: str       # 字段名，如 "out"
    ty: RsType            # 字段类型
    turbofish: str = ''   # 泛型类型参数，如 '::<Object>' 用于消歧 E0283


RsExpr = Union[
    Lit, Var, BinOp, UnOp, Call, MethodCall,
    FieldAccess, Index, Cast, RefExpr, DerefExpr,
    BlockExpr, IfExpr, MacroExpr, RawExpr,
    NewPendingExpr, StaticFieldRef,
]


# ═══════════════════════════════════════════════════════════════════════════════
# 语句节点
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class LetStmt:
    name: str
    ty: Optional[RsType] = None
    mutable: bool = False
    value: Optional[RsExpr] = None
    # 变量的模拟类型（不渲染）：`ty` 省略（交给 Rust 推断）时，变量提升 pass 仍需要类型
    # 为前置声明 `let mut x: T = Default::default();` 作标注
    value_ty: Optional[RsType] = None


@dataclass
class AssignStmt:
    target: RsExpr
    value: RsExpr


@dataclass
class ExprStmt:
    expr: RsExpr


@dataclass
class ReturnStmt:
    value: Optional[RsExpr] = None


@dataclass
class BreakStmt:
    pass


@dataclass
class ContinueStmt:
    pass


@dataclass
class LoopStmt:
    body: list[RsStmt] = field(default_factory=list)


@dataclass
class IfStmt:
    cond: RsExpr
    then: list[RsStmt] = field(default_factory=list)
    else_: list[RsStmt] = field(default_factory=list)


@dataclass
class RawStmt:
    """逃生舱：直接插入一行原始语句字符串（过渡期使用）。"""
    code: str


RsStmt = Union[
    LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    BreakStmt, ContinueStmt, LoopStmt, IfStmt, RawStmt,
]


# ═══════════════════════════════════════════════════════════════════════════════
# 顶层条目（Items）
# ═══════════════════════════════════════════════════════════════════════════════

@dataclass
class RsParam:
    name: str
    ty: RsType


@dataclass
class RsFn:
    name: str
    params: list[RsParam] = field(default_factory=list)
    ret_ty: Optional[RsType] = None
    body: list[RsStmt] = field(default_factory=list)
    vis: str = 'pub'         # 'pub' / '' / 'pub(crate)'
    is_unsafe: bool = False


@dataclass
class RsStructField:
    name: str
    ty: RsType
    vis: str = 'pub'


@dataclass
class RsStruct:
    name: str
    fields: list[RsStructField] = field(default_factory=list)
    vis: str = 'pub'
    derives: list[str] = field(default_factory=list)


@dataclass
class RsImpl:
    ty: str                         # impl 的类型名
    trait_: Optional[str] = None   # impl Trait for Ty
    items: list[RsFn] = field(default_factory=list)


@dataclass
class RsUse:
    path: str


@dataclass
class RsMod:
    name: str
    vis: str = 'pub'


@dataclass
class RsTypeAlias:
    name: str
    ty: RsType
    vis: str = 'pub'


@dataclass
class RsRawItem:
    """逃生舱：原始顶层代码块。"""
    code: str


RsItem = Union[RsFn, RsStruct, RsImpl, RsUse, RsMod, RsTypeAlias, RsRawItem]

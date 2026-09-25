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

    def __post_init__(self) -> None:
        # [raw-audit] L5-b 仪表：构造事件计数（只读，不影响发射）
        from . import raw_audit
        raw_audit.record_raw('raw_expr')


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


@dataclass
class CastExpr:
    """checkcast / 跨实例化转换（A-3 IR 化）：替代 `.downcast::<T>()` 与
    `is_instance_of("...")` 字面量的字符串拼接形态，所有消费方按本节点分派。

    - checked=True（checkcast 语义，运行时可失败，S-1）：
      `Clone::clone(&expr).try_cast::<target>("binary_name")?`
      失败返回 Err(JvmError::class_cast)，可被 java_try 捕获（替代 panic）。
    - checked=False（静态合法的视图转换 / 跨实例化擦除路径，A-1）：
      `<target as From<Object>>::from(Clone::clone(&expr))`
      —— `From<Object> for X<A>` 对任意 A 成立（共享存储与对象标识）。
    - box_first=True：expr 是具体 wrapper（非 Object）时先 `Object::from` 装箱
      （保持对象标识与运行时类），再经上述路径转换。
    - interface_target=True（A-4 批次 1，checkcast 到接口）：目标是 registry 接口、
      Rust 侧擦除记录为 Object——判定经 `try_cast_iface(binary_name)`（null 通过 +
      is_instance_of 按运行时类静态超类型名单，含接口闭包），值不变（同一对象），
      栈类型保持 Object（载体进类型位置后翻转为目标载体形态）。
    binary_name 仅 checked=True 时使用（运行时类族判定依据）。
    """
    expr: RsExpr
    target: str           # 目标 Rust 类型串（含泛型实参，如 'HashMap_Node<K, V>'）
    binary_name: str = '' # 目标 JVM binary 名（checked=True 的运行时判定依据）
    checked: bool = False
    box_first: bool = False
    interface_target: bool = False


@dataclass
class InstanceOfExpr:
    """instanceof 的运行时判定（A-3 IR 化）：接收者的运行时类是否 IS-A binary_name。

    静态可判定的折叠（接收者静态类型与目标互为子类型关系 → 恒真/恒假）在生成侧
    完成（Lit(true)/Lit(false)，计入 instanceof_fold 审计），不进入本节点；
    运行时判定统一经擦除类（ObjectVTable::is_instance_of 按静态超类型名单匹配）。
    """
    expr: RsExpr
    binary_name: str      # 目标 JVM binary 名（如 "java/lang/String"）


RsExpr = Union[
    Lit, Var, BinOp, UnOp, Call, MethodCall,
    FieldAccess, Index, Cast, RefExpr, DerefExpr,
    BlockExpr, IfExpr, MacroExpr, RawExpr,
    NewPendingExpr, StaticFieldRef, CastExpr, InstanceOfExpr,
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
    # 变量身份（不渲染）：创建该绑定的 JVM 局部变量槽与 store 的字节码偏移。
    # 供变量提升 pass 做 LVT 区间驱动的身份判定（vars._same_jvm_var）：
    # 同名语句是否同一 JVM 变量，以 LocalVariableTable 的 (start, length)
    # 活跃区间为证据源
    slot: Optional[int] = None
    bind_off: Optional[int] = None


@dataclass
class AssignStmt:
    target: RsExpr
    value: RsExpr
    # 由声明降级而来的赋值保留值的模拟类型（不渲染）：变量提升 pass 据此在前置声明的类型
    # 与该次赋值的类型不同时补转换
    value_ty: Optional[RsType] = None
    # 变量身份（不渲染）：语义同 LetStmt.slot / bind_off
    slot: Optional[int] = None
    bind_off: Optional[int] = None


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

    def __post_init__(self) -> None:
        # [raw-audit] L5-b 仪表：构造事件计数（只读，不影响发射）
        from . import raw_audit
        raw_audit.record_raw('raw_stmt')


class StructLine(str):
    """块结构行（窗口 3 G-1b）：文本即渲染结果，`delta` 为该行对块嵌套深度的净贡献，
    由 emitter 按结构角色给出（开块 +1、`} else {` / `} catch … {` 等衔接行 0、收尾 −1），
    替代对渲染文本数花括号。str 子类：下游文本处理与渲染不受影响。

    tag：结构角色（替代文本特征判定）——'loop'（loop 关键字头：`loop {` /
    `'lN: loop {` / 状态机 loop）、'else'（`} else {` / `} else if … {`）、'arm'（match
    臂头与状态机臂头 `X => {`）、'try'（java_try! 内层 `try {`）、'catch'（`} catch … {`）；
    其余为 ''。"""
    delta: int
    tag: str

    def __new__(cls, text: str, delta: int, tag: str = ''):
        obj = super().__new__(cls, text)
        obj.delta = delta
        obj.tag = tag
        return obj


@dataclass
class BlockStmt:
    """方法体结构块（窗口 3 G-1）：`method/emit.py` 的结构化产物。

    kind ∈ block（带标签块）/ loop / if / match / try / dispatch。segs 按源码顺序
    交替存放块的**结构行**（`(indent, str)`：块头、`} else {`、match 臂头、catch 头、
    收尾 `}` 等）与**子语句序列**（`list`，元素同 entries：`(indent, RsStmt | str)`
    或嵌套 `('', BlockStmt)`）。`method.emit.flatten` 按序展开即得旧扁平 entries，
    变量提升在树上工作后同样经它渲染。"""
    kind: str
    segs: list = field(default_factory=list)


RsStmt = Union[
    LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    BreakStmt, ContinueStmt, LoopStmt, IfStmt, RawStmt, BlockStmt,
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

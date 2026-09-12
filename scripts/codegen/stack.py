"""
JVM 操作数栈模拟器：将基于栈的字节码转换为 Rust IR 节点（SSA 前驱）。

栈中存储 (RsExpr, RsType) 元组，语句列表存储 RsStmt 节点。
全量使用 IR 节点，不再接受字符串参数。
"""

from typing import get_args

from .rs_ir import (
    RsExpr, RsStmt, RsType,
    Var, Lit, RawExpr,
    LetStmt, AssignStmt,
    RsGeneric, RsPrimitive, RsNamed,
    I32 as _I32, I64 as _I64, F32 as _F32, F64 as _F64,
)

# ── 类型常量（供外部导入使用）────────────────────────────────────────────────
I32  = _I32
I64  = _I64
F32  = _F32
F64  = _F64
BOOL = RsPrimitive('bool')
UNIT = RsPrimitive('()')

# ── isinstance 检查用元组（typing.Union 不能直接用于 isinstance）─────────────
_EXPR_CLASSES = get_args(RsExpr)
_TYPE_CLASSES = get_args(RsType)
_STMT_CLASSES = tuple(get_args(RsStmt))


# Rust 关键字集合（变量名不能与之重名）
_RUST_KEYWORDS = frozenset({
    'as', 'async', 'await', 'break', 'const', 'continue', 'crate', 'dyn',
    'else', 'enum', 'extern', 'false', 'fn', 'for', 'if', 'impl', 'in',
    'let', 'loop', 'match', 'mod', 'move', 'mut', 'pub', 'ref', 'return',
    'self', 'Self', 'static', 'struct', 'super', 'trait', 'true', 'type',
    'union', 'unsafe', 'use', 'where', 'while', 'abstract', 'become',
    'box', 'do', 'final', 'macro', 'override', 'priv', 'try', 'typeof',
    'unsized', 'virtual', 'yield',
})


def _safe_name(name: str) -> str:
    """确保变量名是合法的 Rust 标识符（避开关键字）。"""
    if name in _RUST_KEYWORDS:
        return name + '_'
    # 将 Java 合法但 Rust 不合法的字符替换
    name = name.replace('$', '_')
    if name and name[0].isdigit():
        name = '_' + name
    return name


class StackSim:
    def __init__(self, param_rust_types: list[RsType], is_static: bool, class_name: str,
                 local_names: dict[int, str] | None = None):
        self.stack:      list[tuple[RsExpr, RsType]] = []
        self._ctr:       int                         = 0
        self.locals:     dict[int, tuple]            = {}   # slot → (name, RsType, is_new)
        self.stmts:      list[RsStmt]                = []
        self.is_static   = is_static
        self.class_name  = class_name
        self._loc_names  = local_names or {}  # slot → Java variable name

        if is_static:
            for slot, rt in enumerate(param_rust_types):
                name = _safe_name(self._loc_names.get(slot, f"arg_{slot}"))
                self.locals[slot] = (name, rt, False)
        else:
            # this 是当前类的句柄（不再用 Rc<RefCell<...>>，直接用类名）
            this_ty = RsNamed(class_name) if class_name else RsNamed("Object")
            self.locals[0] = ("this", this_ty, False)
            for slot, rt in enumerate(param_rust_types):
                name = _safe_name(self._loc_names.get(slot + 1, f"arg_{slot}"))
                self.locals[slot + 1] = (name, rt, False)

    # ── 生成新的临时变量名 ───────────────────────────────────────────────────

    def fresh(self, prefix: str = '_t') -> str:
        v = f"{prefix}{self._ctr}"
        self._ctr += 1
        return v

    # ── 栈操作 ───────────────────────────────────────────────────────────────

    def push(self, expr: RsExpr, ty: RsType = I32):
        """压栈。expr 必须是 RsExpr 节点，ty 必须是 RsType 节点。"""
        assert isinstance(expr, _EXPR_CLASSES), \
            f"push() requires RsExpr, got {type(expr)}: {expr!r}"
        assert isinstance(ty, _TYPE_CLASSES), \
            f"push() requires RsType, got {type(ty)}: {ty!r}"
        self.stack.append((expr, ty))

    def pop(self) -> tuple[RsExpr, RsType]:
        """弹栈，返回 (RsExpr, RsType)。"""
        return self.stack.pop() if self.stack else (RawExpr('/* UNDERFLOW */'), I32)

    # ── 语句输出 ─────────────────────────────────────────────────────────────

    def emit(self, stmt: RsStmt):
        """追加语句。stmt 必须是 RsStmt 节点。"""
        assert isinstance(stmt, _STMT_CLASSES), \
            f"emit() requires RsStmt, got {type(stmt)}: {stmt!r}"
        self.stmts.append(stmt)

    # ── 局部变量 ─────────────────────────────────────────────────────────────

    def store_local(self, slot: int, expr: RsExpr, ty: RsType):
        """
        存储到局部变量槽。
        - 若槽已存在：生成 AssignStmt 节点
        - 若槽不存在：生成 LetStmt 节点并注册
        """
        assert isinstance(expr, _EXPR_CLASSES), \
            f"store_local() requires RsExpr, got {type(expr)}: {expr!r}"
        assert isinstance(ty, _TYPE_CLASSES), \
            f"store_local() requires RsType, got {type(ty)}: {ty!r}"
        if slot in self.locals:
            name, _, _ = self.locals[slot]
            self.stmts.append(AssignStmt(Var(name), expr))
        else:
            name = _safe_name(self._loc_names.get(slot, f"local_{slot}"))
            self.locals[slot] = (name, ty, True)
            # mutable=True，后续 mutation 分析会移除不必要的 mut
            self.stmts.append(LetStmt(name, ty, mutable=True, value=expr))

    def load_local(self, slot: int) -> tuple[RsExpr, RsType]:
        """从局部变量槽加载，返回 (RsExpr, RsType)。"""
        if slot in self.locals:
            name, ty, _ = self.locals[slot]
            return (Var(name), ty)
        return (Var(f"local_{slot}"), I32)

    # ── 辅助：生成 let + 临时变量（供 instr.py 中的"计算并绑定"模式）──────

    def fresh_let(self, prefix: str, value_expr: RsExpr, ty: RsType) -> RsExpr:
        """
        生成 LetStmt(vN, ty, mutable=False, value=value_expr)，返回 Var(vN) 供后续压栈。
        """
        assert isinstance(value_expr, _EXPR_CLASSES), \
            f"fresh_let() requires RsExpr, got {type(value_expr)}: {value_expr!r}"
        assert isinstance(ty, _TYPE_CLASSES), \
            f"fresh_let() requires RsType, got {type(ty)}: {ty!r}"
        name = self.fresh(prefix)
        self.stmts.append(LetStmt(name, ty, mutable=False, value=value_expr))
        return Var(name)

"""
JVM 操作数栈模拟器：将基于栈的字节码转换为 Rust IR 节点（SSA 前驱）。

栈中存储 (RsExpr, type_str) 元组，语句列表存储 RsStmt 节点。
过渡期：RawExpr / RawStmt 作为逃生舱包装旧的字符串形式。
"""

from .rs_ir import (
    RsExpr, RsStmt,
    Var, Lit, RawExpr, RawStmt,
    LetStmt, AssignStmt,
    RsGeneric, RsPrimitive,
)
from .render import render_expr, render_type


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
    def __init__(self, param_rust_types: list[str], is_static: bool, class_name: str,
                 local_names: dict[int, str] | None = None):
        self.stack:      list[tuple[RsExpr, str]] = []
        self._ctr:       int                      = 0
        self.locals:     dict[int, tuple]         = {}   # slot → (name, ty_str, is_new)
        self.stmts:      list[RsStmt]             = []
        self.is_static   = is_static
        self.class_name  = class_name
        self._loc_names  = local_names or {}  # slot → Java variable name

        if is_static:
            for slot, rt in enumerate(param_rust_types):
                name = _safe_name(self._loc_names.get(slot, f"arg_{slot}"))
                self.locals[slot] = (name, rt, False)
        else:
            # this 是当前类的句柄（不再用 Rc<RefCell<...>>，直接用类名）
            this_ty = class_name if class_name else "JvmObject"
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

    def push(self, expr, ty: str = 'i32'):
        """
        压栈。expr 可以是：
        - RsExpr 节点（推荐）
        - str（自动包装为 RawExpr，过渡兼容）
        """
        if isinstance(expr, str):
            expr = RawExpr(expr)
        self.stack.append((expr, ty))

    def pop(self) -> tuple[RsExpr, str]:
        """弹栈，返回 (RsExpr, type_str)。"""
        return self.stack.pop() if self.stack else (RawExpr('/* UNDERFLOW */'), 'i32')

    def pop_str(self) -> tuple[str, str]:
        """弹栈并将 expr 渲染为字符串（供旧代码过渡使用）。"""
        expr, ty = self.pop()
        return render_expr(expr), ty

    # ── 语句输出 ─────────────────────────────────────────────────────────────

    def emit(self, stmt):
        """
        追加语句。stmt 可以是：
        - RsStmt 节点（推荐）
        - str（自动包装为 RawStmt，过渡兼容）
        """
        if isinstance(stmt, str):
            stmt = RawStmt(stmt.strip())
        self.stmts.append(stmt)

    # ── 局部变量 ─────────────────────────────────────────────────────────────

    def store_local(self, slot: int, expr, ty: str):
        """
        存储到局部变量槽。
        - 若槽已存在：生成 Assign 节点
        - 若槽不存在：生成 Let 节点并注册
        """
        if isinstance(expr, str):
            expr = RawExpr(expr)
        if slot in self.locals:
            name, _, _ = self.locals[slot]
            self.stmts.append(RawStmt(f'{name} = {render_expr(expr)};'))
        else:
            name = _safe_name(self._loc_names.get(slot, f"local_{slot}"))
            self.locals[slot] = (name, ty, True)
            self.stmts.append(RawStmt(f'let mut {name}: {ty} = {render_expr(expr)};'))

    def load_local(self, slot: int) -> tuple[RsExpr, str]:
        """从局部变量槽加载，返回 (RsExpr, type_str)。"""
        if slot in self.locals:
            name, ty, _ = self.locals[slot]
            return (Var(name), ty)
        return (Var(f"local_{slot}"), 'i32')

    def load_local_str(self, slot: int) -> tuple[str, str]:
        """加载并渲染为字符串（供旧代码过渡使用）。"""
        expr, ty = self.load_local(slot)
        return render_expr(expr), ty

    # ── 辅助：生成 let + 临时变量（供 instr.py 中的"计算并绑定"模式）──────

    def fresh_let(self, prefix: str, value_expr, ty: str) -> RsExpr:
        """
        生成 'let mut vN: ty = expr;'，返回 Var(vN) 供后续压栈。
        """
        if isinstance(value_expr, str):
            value_expr = RawExpr(value_expr)
        name = self.fresh(prefix)
        self.stmts.append(RawStmt(f'let mut {name}: {ty} = {render_expr(value_expr)};'))
        return Var(name)

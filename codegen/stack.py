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
from .render import render_type, render_expr
from .type_map import short_cls as _short_cls
from .constants import safe_ident

# _safe_name 保留为别名，供 method.py 等现有代码导入
_safe_name = safe_ident

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


def _maybe_downcast(expr: RsExpr, ty: RsType) -> RsExpr:
    """当 expr 是 Var（新鲜临时变量）且目标类型含泛型参数时，
    JDK stub 因类型擦除实际返回 Object，需要 downcast 恢复具体类型。
    对工厂方法 / @synthetic（expr 为 RawExpr）不添加 downcast。
    """
    if (isinstance(expr, Var) and isinstance(ty, RsNamed)
            and '<' in ty.name and not ty.name.startswith('Rc<')):
        return RawExpr(f"({render_expr(expr)}).downcast::<{ty.name}>()")
    return expr




class StackSim:
    def __init__(self, param_rust_types: list[RsType], is_static: bool, class_name: str,
                 local_names: dict[int, str] | None = None,
                 slot_hint_types: dict[int, RsType] | None = None,
                 return_type: str = 'Object'):
        self.stack:      list[tuple[RsExpr, RsType]] = []
        self._ctr:       int                         = 0
        self.locals:     dict[int, tuple]            = {}   # slot → (name, RsType, is_new)
        self.stmts:      list[RsStmt]                = []
        self.is_static   = is_static
        self.class_name  = class_name
        self.return_type = return_type
        self._loc_names  = local_names or {}     # slot → Java variable name
        self._hint_types = slot_hint_types or {}  # slot → precise RsType from LocalVariableTypeTable

        def _is_wide(rt: RsType) -> bool:
            """long (i64) 和 double (f64) 在 JVM 中各占 2 个局部变量槽。"""
            return isinstance(rt, RsPrimitive) and rt.name in ('i64', 'f64')

        if is_static:
            slot = 0
            for rt in param_rust_types:
                name = _safe_name(self._loc_names.get(slot, f"arg_{slot}"))
                self.locals[slot] = (name, rt, False)
                slot += 2 if _is_wide(rt) else 1
        else:
            # this 是当前类的句柄，用 short_cls 转换 JVM 二进制名到 Rust 短名
            rust_cls = _short_cls(class_name) if class_name else 'Object'
            this_ty = RsNamed(rust_cls) if rust_cls else RsNamed("Object")
            self.locals[0] = ("this", this_ty, False)
            slot = 1
            for idx, rt in enumerate(param_rust_types):
                name = _safe_name(self._loc_names.get(slot, f"arg_{idx}"))
                self.locals[slot] = (name, rt, False)
                slot += 2 if _is_wide(rt) else 1

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
        if self.stack:
            return self.stack.pop()
        # 栈下溢：常见于 catch 块隐式压栈的异常对象、复杂控制流分析失败
        return (RawExpr('todo!("stack underflow")'), I32)

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
        - 若栈顶类型是 Object 且 _hint_types 有更精确的类型（来自 LocalVariableTypeTable），
          用精确类型替换（如 String 替代 Object）
        """
        assert isinstance(expr, _EXPR_CLASSES), \
            f"store_local() requires RsExpr, got {type(expr)}: {expr!r}"
        assert isinstance(ty, _TYPE_CLASSES), \
            f"store_local() requires RsType, got {type(ty)}: {ty!r}"
        # 用 LocalVariableTypeTable 提供的精确类型覆盖泛型擦除后的 Object
        hint = self._hint_types.get(slot)
        # 记录原始栈类型：只有在栈类型为 Object 时才需要 downcast
        src_is_object = isinstance(ty, RsNamed) and ty.name == 'Object'
        if hint is not None and src_is_object:
            ty = hint
        if slot in self.locals:
            name, old_ty, _ = self.locals[slot]
            # T42: slot 类型发生变化时（如 for-each 迭代器 slot 被后续变量复用），
            # 用 let 阴影（shadowing）而非赋值，避免 Rust 类型不匹配
            if render_type(old_ty) != render_type(ty):
                self.locals[slot] = (name, ty, True)
                value = _maybe_downcast(expr, ty) if src_is_object else expr
                let_ty = None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty)
                self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value))
            else:
                self.stmts.append(AssignStmt(Var(name), expr))
        else:
            name = _safe_name(self._loc_names.get(slot, f"local_{slot}"))
            self.locals[slot] = (name, ty, True)
            value = _maybe_downcast(expr, ty) if src_is_object else expr
            # downcast 时让 Rust 推断类型；RsGeneric 也让 Rust 推断；否则写显式类型
            let_ty = None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty)
            self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value))

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

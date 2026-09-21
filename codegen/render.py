"""
Rust IR → 格式化 Rust 源码。

render_item / render_stmt / render_expr / render_type 递归将 IR 树转为字符串。
所有输出均为完整的有效 Rust 语法（含换行和缩进），由调用方拼装成文件。
"""

from __future__ import annotations
from .type_map import short_cls as _short_cls_g
from .rs_ir import (
    # 类型
    RsPrimitive, RsNamed, RsRef, RsSlice, RsGeneric, RsTuple, RsInfer,
    # 表达式
    Lit, Var, BinOp, UnOp, Call, MethodCall, FieldAccess, Index,
    Cast, RefExpr, DerefExpr, BlockExpr, IfExpr, MacroExpr, RawExpr,
    NewPendingExpr, StaticFieldRef, CastExpr, InstanceOfExpr,
    # 语句
    LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    BreakStmt, ContinueStmt, LoopStmt, IfStmt, RawStmt,
    # 条目
    RsFn, RsStruct, RsImpl, RsUse, RsMod, RsTypeAlias, RsRawItem,
    RsParam, RsStructField,
)

_INDENT = '    '


# ─────────────────────────────────────────────────────────────────────────────
# 类型
# ─────────────────────────────────────────────────────────────────────────────

def render_type(ty) -> str:
    if isinstance(ty, RsPrimitive):
        return ty.name
    if isinstance(ty, RsNamed):
        return ty.full
    if isinstance(ty, RsRef):
        lt = f"'{ty.lifetime} " if ty.lifetime else ''
        mut = 'mut ' if ty.mutable else ''
        return f'&{lt}{mut}{render_type(ty.inner)}'
    if isinstance(ty, RsSlice):
        return f'&[{render_type(ty.elem)}]'
    if isinstance(ty, RsGeneric):
        if ty.params:
            params = ', '.join(render_type(p) for p in ty.params)
            return f'{ty.outer}<{params}>'
        return ty.outer
    if isinstance(ty, RsTuple):
        if not ty.elems:
            return '()'
        return '(' + ', '.join(render_type(e) for e in ty.elems) + ')'
    if isinstance(ty, RsInfer):
        return '_'
    # 兜底：str 原样
    return str(ty)


# ─────────────────────────────────────────────────────────────────────────────
# 表达式（带优先级括号）
# ─────────────────────────────────────────────────────────────────────────────

# 运算符优先级（数字越大越紧）
_PREC: dict[str, int] = {
    '||': 1, '&&': 2,
    '==': 3, '!=': 3, '<': 3, '>': 3, '<=': 3, '>=': 3,
    '|': 4, '^': 5, '&': 6,
    '<<': 7, '>>': 7,
    '+': 8, '-': 8,
    '*': 9, '/': 9, '%': 9,
}


def _expr_prec(expr) -> int:
    if isinstance(expr, BinOp):
        return _PREC.get(expr.op, 0)
    return 99


def _paren_if_needed(expr, parent_prec: int) -> str:
    s = render_expr(expr)
    if isinstance(expr, BinOp) and _expr_prec(expr) < parent_prec:
        return f'({s})'
    return s


def render_expr(expr) -> str:
    if isinstance(expr, Lit):
        return expr.value
    if isinstance(expr, Var):
        return expr.name
    if isinstance(expr, BinOp):
        prec = _PREC.get(expr.op, 0)
        l = _paren_if_needed(expr.left, prec)
        r = _paren_if_needed(expr.right, prec + 1)  # +1 使右结合正确
        return f'{l} {expr.op} {r}'
    if isinstance(expr, UnOp):
        return f'{expr.op}{render_expr(expr.expr)}'
    if isinstance(expr, Call):
        args = ', '.join(render_expr(a) for a in expr.args)
        return f'{expr.func}({args})'
    if isinstance(expr, MethodCall):
        recv = render_expr(expr.recv)
        args = ', '.join(render_expr(a) for a in expr.args)
        return f'{recv}.{expr.method}({args})'
    if isinstance(expr, FieldAccess):
        return f'{render_expr(expr.recv)}.{expr.field_name}'
    if isinstance(expr, Index):
        return f'{render_expr(expr.recv)}[{render_expr(expr.idx)}]'
    if isinstance(expr, Cast):
        return f'({render_expr(expr.expr)} as {render_type(expr.ty)})'
    if isinstance(expr, RefExpr):
        mut = 'mut ' if expr.mutable else ''
        return f'&{mut}{render_expr(expr.expr)}'
    if isinstance(expr, DerefExpr):
        return f'*{render_expr(expr.expr)}'
    if isinstance(expr, BlockExpr):
        return _render_block_expr(expr)
    if isinstance(expr, IfExpr):
        return _render_if_expr(expr)
    if isinstance(expr, MacroExpr):
        if expr.args:
            args = ', '.join(expr.args)
            return f'{expr.name}!({args})'
        return f'{expr.name}!()'
    if isinstance(expr, NewPendingExpr):
        simple_name = _short_cls_g(expr.class_name)
        return f'{simple_name}::new()'
    if isinstance(expr, StaticFieldRef):
        simple_name = _short_cls_g(expr.class_name)
        return f'{simple_name}{expr.turbofish}::{expr.field_name}()?'
    if isinstance(expr, CastExpr):
        return render_cast(expr)
    if isinstance(expr, InstanceOfExpr):
        return f'({render_expr(expr.expr)}).is_instance_of("{expr.binary_name}")'
    if isinstance(expr, RawExpr):
        return expr.code
    return f'/* unknown expr {type(expr).__name__} */'


def _render_block_expr(b: BlockExpr) -> str:
    lines = ['{\n']
    for s in b.stmts:
        lines.append(f'{_INDENT}{render_stmt(s)}\n')
    if b.tail is not None:
        lines.append(f'{_INDENT}{render_expr(b.tail)}\n')
    lines.append('}')
    return ''.join(lines)


def _clone_src(expr) -> str:
    """转换源的保活取值：Java 引用无 move 语义，统一 Clone::clone。
    `this` 在实例方法中是 &Self（Clone::clone(this) 借引用克隆本体，
    Clone::clone(&this) 会克隆引用本身），与 _coerce_arg / _coerce_stored_value 同规则。"""
    if isinstance(expr, Var) and expr.name == 'this':
        return 'Clone::clone(this)'
    return f"Clone::clone(&{render_expr(expr)})"


def render_cast(c: CastExpr) -> str:
    """CastExpr 的统一渲染（A-3）：checkcast 语义（可失败，S-1）与
    静态合法的 From<Object> 视图转换（跨实例化擦除路径）共用本入口。"""
    src = _clone_src(c.expr)
    if c.box_first:
        src = f"Object::from({src})"
    if c.checked:
        if c.target.startswith('JArray<') and c.target.endswith('>'):
            # 数组目标的 checkcast：元素类型驱动（try_cast_array，S-4/A-1 与
            # From<Object> for JArray<E> 的合流决策点）；目标整体类型在 Rust
            # 类型层取不出元素类型，故以剥一层的元素类型 turbofish 发射
            _elem = c.target[len('JArray<'):-1]
            return f'{src}.try_cast_array::<{_elem}>("{c.binary_name}")?'
        return f'{src}.try_cast::<{c.target}>("{c.binary_name}")?'
    return f"<{c.target} as ::std::convert::From<Object>>::from({src})"


def _render_if_expr(e: IfExpr) -> str:
    cond_str = render_expr(e.cond)
    # 条件静态为 false：跳过 then 块，直接渲染 else 块（若有）
    # 避免死代码中的类型擦除不一致导致 E0308
    if cond_str == 'false':
        if e.else_ is not None:
            return _render_block_expr(e.else_)
        return '{}'
    s = f'if {cond_str} {_render_block_expr(e.then)}'
    if e.else_ is not None:
        s += f' else {_render_block_expr(e.else_)}'
    return s


# ─────────────────────────────────────────────────────────────────────────────
# 语句
# ─────────────────────────────────────────────────────────────────────────────

def render_stmt(stmt, indent: int = 0) -> str:
    pad = _INDENT * indent

    if isinstance(stmt, LetStmt):
        mut = 'mut ' if stmt.mutable else ''
        ty  = f': {render_type(stmt.ty)}' if stmt.ty is not None else ''
        val = f' = {render_expr(stmt.value)}' if stmt.value is not None else ''
        return f'{pad}let {mut}{stmt.name}{ty}{val};'

    if isinstance(stmt, AssignStmt):
        return f'{pad}{render_expr(stmt.target)} = {render_expr(stmt.value)};'

    if isinstance(stmt, ExprStmt):
        return f'{pad}{render_expr(stmt.expr)};'

    if isinstance(stmt, ReturnStmt):
        if stmt.value is not None:
            return f'{pad}return {render_expr(stmt.value)};'
        return f'{pad}return;'

    if isinstance(stmt, BreakStmt):
        return f'{pad}break;'

    if isinstance(stmt, ContinueStmt):
        return f'{pad}continue;'

    if isinstance(stmt, LoopStmt):
        body = '\n'.join(render_stmt(s, indent + 1) for s in stmt.body)
        return f'{pad}loop {{\n{body}\n{pad}}}'

    if isinstance(stmt, IfStmt):
        then_body = '\n'.join(render_stmt(s, indent + 1) for s in stmt.then)
        result = f'{pad}if {render_expr(stmt.cond)} {{\n{then_body}\n{pad}}}'
        if stmt.else_:
            else_body = '\n'.join(render_stmt(s, indent + 1) for s in stmt.else_)
            result += f' else {{\n{else_body}\n{pad}}}'
        return result

    if isinstance(stmt, RawStmt):
        return f'{pad}{stmt.code}'

    return f'{pad}/* unknown stmt {type(stmt).__name__} */'


# ─────────────────────────────────────────────────────────────────────────────
# 条目（顶层）
# ─────────────────────────────────────────────────────────────────────────────

def render_fn(fn: RsFn, indent: int = 0) -> str:
    pad = _INDENT * indent
    vis = f'{fn.vis} ' if fn.vis else ''
    unsafe = 'unsafe ' if fn.is_unsafe else ''

    params = ', '.join(
        f'{p.name}: {render_type(p.ty)}' for p in fn.params
    )
    ret = f' -> {render_type(fn.ret_ty)}' if fn.ret_ty else ''

    if not fn.body:
        return f'{pad}{vis}{unsafe}fn {fn.name}({params}){ret} {{}}'

    body_lines = '\n'.join(render_stmt(s, indent + 1) for s in fn.body)
    return (
        f'{pad}{vis}{unsafe}fn {fn.name}({params}){ret} {{\n'
        f'{body_lines}\n'
        f'{pad}}}'
    )


def render_struct(s: RsStruct, indent: int = 0) -> str:
    pad = _INDENT * indent
    vis = f'{s.vis} ' if s.vis else ''
    derives = ''
    if s.derives:
        derives = f'{pad}#[derive({", ".join(s.derives)})]\n'
    if not s.fields:
        return f'{derives}{pad}{vis}struct {s.name};'
    field_lines = ',\n'.join(
        f'{pad}{_INDENT}{f.vis + " " if f.vis else ""}{f.name}: {render_type(f.ty)}'
        for f in s.fields
    )
    return (
        f'{derives}{pad}{vis}struct {s.name} {{\n'
        f'{field_lines},\n'
        f'{pad}}}'
    )


def render_impl(impl: RsImpl, indent: int = 0) -> str:
    pad = _INDENT * indent
    header = f'impl {impl.ty}' if not impl.trait_ else f'impl {impl.trait_} for {impl.ty}'
    if not impl.items:
        return f'{pad}{header} {{}}'
    items = '\n\n'.join(render_fn(fn, indent + 1) for fn in impl.items)
    return f'{pad}{header} {{\n{items}\n{pad}}}'


def render_item(item, indent: int = 0) -> str:
    pad = _INDENT * indent
    if isinstance(item, RsFn):
        return render_fn(item, indent)
    if isinstance(item, RsStruct):
        return render_struct(item, indent)
    if isinstance(item, RsImpl):
        return render_impl(item, indent)
    if isinstance(item, RsUse):
        return f'{pad}use {item.path};'
    if isinstance(item, RsMod):
        vis = f'{item.vis} ' if item.vis else ''
        return f'{pad}{vis}mod {item.name};'
    if isinstance(item, RsTypeAlias):
        vis = f'{item.vis} ' if item.vis else ''
        return f'{pad}{vis}type {item.name} = {render_type(item.ty)};'
    if isinstance(item, RsRawItem):
        return item.code
    return f'/* unknown item {type(item).__name__} */'


def render_file(items: list, preamble: str = '') -> str:
    """将一组 RsItem 渲染为完整的 .rs 文件内容。"""
    parts = []
    if preamble:
        parts.append(preamble.rstrip())
    parts.extend(render_item(it) for it in items)
    return '\n\n'.join(parts) + '\n'

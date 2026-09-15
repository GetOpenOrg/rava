"""
变量分析与提升：纯函数，无外部状态。
"""

import re
from ..render import render_stmt, render_expr, render_type
from ..rs_ir import (
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt,
)


def _coerce_icmp_operand(expr_str: str, ty_node) -> str:
    """为 if_icmpX 比较的操作数做类型强制转换：u16/i8/i16 → i32"""
    ty = render_type(ty_node)
    if ty in ('u16', 'i8', 'i16'):
        return f"({expr_str} as i32)"
    return expr_str


def _coerce_acmp_operand(expr_str: str, ty_node) -> str:
    """为 if_acmpX 对象引用比较做类型强制转换：非 Object 类型统一装入 Object。

    Java if_acmpeq/if_acmpne 是引用相等比较，Rust 中统一转为 Object 然后用 Object::PartialEq。
    """
    ty = render_type(ty_node)
    if ty == 'Object':
        return expr_str
    if ty in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16', 'usize', '()'):
        return expr_str  # 基本类型不应出现在 acmp，原样保留
    # 引用类型或 self 引用：去掉 &，clone 后装入 Object
    clean = expr_str[1:] if expr_str.startswith('&') else expr_str
    return f"Object::from_any({clean}.clone())"


_PRIMITIVE_TYPES = {'i32', 'i64', 'f32', 'f64', 'bool', 'usize', '()'}

def _str_to_rs_type(s: str) -> RsType:
    """将 jvm_to_rust 返回的字符串转换为 RsType 节点。"""
    if s in _PRIMITIVE_TYPES:
        return RsPrimitive(s)
    return RsNamed(s)


def _analyze_mutation(stmts):
    """扫描 AssignStmt 目标，将对应的 LetStmt.mutable 设为 True。

    递归处理 IfStmt / LoopStmt 内部语句。
    """
    assigned: set[str] = set()

    def collect(ss):
        for stmt in ss:
            if isinstance(stmt, AssignStmt) and isinstance(stmt.target, Var):
                assigned.add(stmt.target.name)
            if isinstance(stmt, IfStmt):
                collect(stmt.then)
                collect(stmt.else_)
            if isinstance(stmt, LoopStmt):
                collect(stmt.body)

    collect(stmts)

    def mark(ss):
        for stmt in ss:
            if isinstance(stmt, LetStmt) and stmt.name in assigned:
                stmt.mutable = True
            if isinstance(stmt, IfStmt):
                mark(stmt.then)
                mark(stmt.else_)
            if isinstance(stmt, LoopStmt):
                mark(stmt.body)

    mark(stmts)


def _hoist_loop_vars(entries: list, predeclared: set[str]):
    """将在 loop{} 内 let-声明但在 loop 外被读取的变量提升到 loop 前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 loop 内首次 let-声明，
    但在 loop 退出后被读取，Rust 报 E0425。
    修复：在 loop 前插入 let mut NAME = Default::default();，loop 内改为赋值。
    """
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置
    nesting = 0
    declared_at: dict[str, tuple[int, int]] = {}  # name → (index, nesting_depth)
    loop_entry_indices: list[int] = []  # loop { 的索引

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            if item.rstrip().endswith('loop {'):
                loop_entry_indices.append(k)
            delta = item.count('{') - item.count('}')
            nesting += delta
        elif isinstance(item, LetStmt):
            if nesting > 0 and item.name not in declared_at and item.name not in predeclared:
                declared_at[item.name] = (k, nesting)

    if not declared_at or not loop_entry_indices:
        return

    # Pass 2: 将 entries 渲染为字符串，检测哪些 loop 内变量在 loop 外被引用
    rendered: list[str] = []
    for _, item in entries:
        if isinstance(item, str):
            rendered.append(item)
        else:
            try:
                rendered.append(render_stmt(item))
            except Exception:
                rendered.append('')

    # 追踪每个条目的嵌套深度
    entry_nesting: list[int] = []
    cur = 0
    for text in rendered:
        entry_nesting.append(cur)
        cur += text.count('{') - text.count('}')

    vars_to_hoist: set[str] = set()
    for name, (decl_k, decl_nesting) in declared_at.items():
        # 找到该变量作用域关闭的第一个索引（nesting 降回 decl_nesting - 1）
        scope_close = None
        for k2 in range(decl_k + 1, len(entries)):
            if entry_nesting[k2] < decl_nesting:
                scope_close = k2
                break
        if scope_close is None:
            continue
        # 检查 scope_close 之后是否有该变量名的引用
        word = re.compile(r'\b' + re.escape(name) + r'\b')
        for k2 in range(scope_close, len(rendered)):
            if word.search(rendered[k2]):
                vars_to_hoist.add(name)
                break

    if not vars_to_hoist:
        return

    # Pass 3: 对需要提升的变量进行修改
    # 先收集所有插入操作（在 loop 前插入 let mut NAME = Default::default();）
    # 用倒序插入，避免索引偏移
    insertions: list[tuple[int, tuple]] = []  # (index, entry) to insert BEFORE
    for name in vars_to_hoist:
        decl_k, decl_nesting = declared_at[name]
        # 找到包含此声明的最近的 loop { 索引
        loop_k = None
        for lk in reversed(loop_entry_indices):
            if lk < decl_k:
                loop_k = lk
                break
        if loop_k is None:
            continue
        # 获取 loop 行的 indent 作为插入位置的 indent
        loop_indent = entries[loop_k][0]
        insertions.append((loop_k, (loop_indent, LetStmt(name, None, True, RawExpr('unsafe { std::mem::MaybeUninit::uninit().assume_init() }')))))
        # 将 loop 内的 LetStmt 改为 AssignStmt
        inner_indent, inner_item = entries[decl_k]
        if isinstance(inner_item, LetStmt):
            entries[decl_k] = (inner_indent, AssignStmt(Var(inner_item.name), inner_item.value))

    # 倒序插入，避免索引偏移（同一 loop 有多个变量时均插入到 loop 前）
    for ins_k, ins_entry in sorted(insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)


def _hoist_if_vars(entries: list, predeclared: set[str]):
    """将在 if/else 块内 let-声明但在块外被读取的变量提升到块前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 if/else 内首次 let-声明，
    但在 if-else 结束后被读取，Rust 报 E0425。
    修复：在 if 前插入 let mut NAME: TYPE = Default::default();，
    块内所有同名 LetStmt 改为 AssignStmt。
    """
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置、类型
    nesting = 0
    # name → list of (entry_index, nesting_depth)
    declared_at: dict[str, list[tuple[int, int]]] = {}
    # index of any block-opening entry (ends with '{' and contains 'if' or has any '{')
    block_entry_indices: list[int] = []

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            stripped = item.rstrip()
            if stripped.endswith('{') and not stripped.startswith('}'):
                block_entry_indices.append(k)
            delta = item.count('{') - item.count('}')
            nesting += delta
        elif isinstance(item, LetStmt):
            if nesting > 0 and item.name not in predeclared:
                declared_at.setdefault(item.name, []).append((k, nesting))

    if not declared_at or not block_entry_indices:
        return

    # Pass 2: 渲染所有条目为字符串，追踪嵌套深度
    rendered: list[str] = []
    for _, item in entries:
        if isinstance(item, str):
            rendered.append(item)
        else:
            try:
                rendered.append(render_stmt(item))
            except Exception:
                rendered.append('')

    entry_nesting: list[int] = []
    cur = 0
    for text in rendered:
        entry_nesting.append(cur)
        cur += text.count('{') - text.count('}')

    # Pass 3: 找出需要提升的变量 - 在块外被引用
    vars_to_hoist: dict[str, tuple[str | None, list[tuple[int, int]]]] = {}  # name → (type_str, [(decl_k, depth)])
    word_cache: dict[str, re.Pattern] = {}
    for name, decl_list in declared_at.items():
        # 对于每次声明，检查变量是否在其作用域关闭后被引用
        for decl_k, decl_nesting in decl_list:
            scope_close = None
            for k2 in range(decl_k + 1, len(entries)):
                if entry_nesting[k2] < decl_nesting:
                    scope_close = k2
                    break
            if scope_close is None:
                continue
            if name not in word_cache:
                word_cache[name] = re.compile(r'\b' + re.escape(name) + r'\b')
            word = word_cache[name]
            for k2 in range(scope_close, len(rendered)):
                if word.search(rendered[k2]):
                    # 确定类型：从第一个声明获取
                    ty_str = None
                    _, first_item = entries[decl_list[0][0]]
                    if isinstance(first_item, LetStmt) and first_item.ty is not None:
                        try:
                            ty_str = render_type(first_item.ty)
                        except Exception:
                            ty_str = None
                    vars_to_hoist[name] = (ty_str, decl_list)
                    break

    if not vars_to_hoist:
        return

    # Pass 4: 找到合适的插入位置（最内层包含第一次声明的块的开始处）
    insertions: list[tuple[int, tuple]] = []
    for name, (ty_str, decl_list) in vars_to_hoist.items():
        first_decl_k = decl_list[0][0]
        first_decl_nesting = decl_list[0][1]
        # 找到包含第一次声明的最近的块起始索引
        block_k = None
        for bk in reversed(block_entry_indices):
            if bk < first_decl_k and entry_nesting[bk] < first_decl_nesting:
                block_k = bk
                break
        if block_k is None:
            continue
        # 若 block_k 落在 match arm（含 =>）内，向上找到 match 语句本身，
        # 否则插入位置落在两个 arm 之间，产生"expected pattern, found 'let'"错误
        while '=>' in rendered[block_k]:
            arm_nesting = entry_nesting[block_k]
            parent_k = None
            for bk in reversed(block_entry_indices):
                if bk < block_k and entry_nesting[bk] < arm_nesting:
                    parent_k = bk
                    break
            if parent_k is None:
                break
            block_k = parent_k
        block_indent = entries[block_k][0]
        # 获取类型注解节点（来自第一次声明）
        _, first_let = entries[first_decl_k]
        hoisted_type = first_let.ty if isinstance(first_let, LetStmt) else None
        # 统一用 Default::default()，配合类型注解让 Rust 推断
        default_val = RawExpr('Default::default()')
        insertions.append((block_k, (block_indent, LetStmt(name, hoisted_type, True, default_val))))
        # 将块内所有同名 LetStmt 改为 AssignStmt
        for decl_k, _ in decl_list:
            inner_indent, inner_item = entries[decl_k]
            if isinstance(inner_item, LetStmt) and inner_item.name == name:
                entries[decl_k] = (inner_indent, AssignStmt(Var(inner_item.name), inner_item.value))

    for ins_k, ins_entry in sorted(insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)


def _promote_undeclared_assigns(entries: list, predeclared: set[str]):
    """将在当前词法作用域中无对应 LetStmt 的 AssignStmt 提升为 LetStmt(mutable=True)。

    JVM 局部变量槽可在不同词法作用域中复用同一名字：例如变量 j 在 loop{} 内声明，
    loop 结束后又被赋值 j = high，此时 j 已出作用域，Rust 报 E0425。
    修复：追踪各变量声明时的嵌套深度，退出该层作用域后将同名 AssignStmt 提升为 LetStmt。
    """
    declared: dict[str, int] = {}  # name → 声明时的嵌套深度
    nesting = 0

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            delta = item.count('{') - item.count('}')
            if delta < 0:
                nesting += delta
                # 移除在已退出作用域层声明的变量
                declared = {n: d for n, d in declared.items() if d <= nesting}
            else:
                nesting += delta
            continue

        if isinstance(item, LetStmt):
            declared[item.name] = nesting
        elif isinstance(item, AssignStmt) and isinstance(item.target, Var):
            name = item.target.name
            if name not in predeclared and name not in declared:
                # 变量在当前词法作用域不可见 → 提升为 LetStmt（类型由 Rust 推断）
                entries[k] = (indent, LetStmt(name, None, True, item.value))
                declared[name] = nesting

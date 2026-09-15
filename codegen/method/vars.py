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
        # 若第一个匹配是另一个 let 声明（JVM slot reuse），不算跨作用域引用
        word = re.compile(r'\b' + re.escape(name) + r'\b')
        let_decl_check = re.compile(r'\blet\s+(?:mut\s+)?' + re.escape(name) + r'\b')
        for k2 in range(scope_close, len(rendered)):
            ln = rendered[k2]
            if word.search(ln):
                if let_decl_check.search(ln):
                    break  # 另一个 let 声明，不是跨作用域读取
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
        # 获取 loop 行的 indent：对于 str 条目，indent 嵌入在字符串里，需提取前导空白
        loop_entry_val = entries[loop_k][1]
        if isinstance(loop_entry_val, str):
            loop_indent = loop_entry_val[:len(loop_entry_val) - len(loop_entry_val.lstrip())]
        else:
            loop_indent = entries[loop_k][0]
        # 从声明处取类型注解，生成 Default::default() 带类型注解（避免 E0282 类型推断失败）
        inner_indent, inner_item = entries[decl_k]
        hoisted_type = inner_item.ty if isinstance(inner_item, LetStmt) else None
        insertions.append((loop_k, (loop_indent, LetStmt(name, hoisted_type, True, RawExpr('Default::default()')))))
        # 将 loop 内的 LetStmt 改为 AssignStmt
        if isinstance(inner_item, LetStmt):
            entries[decl_k] = (inner_indent, AssignStmt(Var(inner_item.name), inner_item.value))

    # 倒序插入，避免索引偏移（同一 loop 有多个变量时均插入到 loop 前）
    for ins_k, ins_entry in sorted(insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)


def _hoist_if_vars(entries: list, predeclared: set[str]) -> bool:
    """将在 if/else 块内 let-声明但在块外被读取的变量提升到块前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 if/else 内首次 let-声明，
    但在 if-else 结束后被读取，Rust 报 E0425。
    修复：在 if 前插入 let mut NAME: TYPE = Default::default();，
    块内所有同名 LetStmt 改为 AssignStmt。

    返回 True 表示本次有提升，调用方可循环直到返回 False。
    """
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置、类型
    nesting = 0
    # name → list of (entry_index, nesting_depth)
    declared_at: dict[str, list[tuple[int, int]]] = {}
    # 函数体顶层（nesting=0）声明的变量：已有函数级作用域，不应被 hoist 覆盖
    outer_decls: set[str] = set()
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
            if nesting == 0 and item.name not in predeclared:
                outer_decls.add(item.name)
            elif nesting > 0 and item.name not in predeclared:
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

    # Pass 3: 找出需要提升的变量 - 在块外或 else 兄弟块中被引用
    vars_to_hoist: dict[str, tuple[str | None, list[tuple[int, int]]]] = {}  # name → (type_str, [(decl_k, depth)])
    word_cache: dict[str, re.Pattern] = {}
    for name, decl_list in declared_at.items():
        # 对于每次声明，检查变量是否在其作用域关闭后或 else 兄弟块中被引用
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

            found = False
            # 检查 1：作用域关闭后是否被引用（原有逻辑）
            # 若第一个匹配是另一个 let 声明（同名变量在另一分支的单独绑定），
            # 则不算跨作用域引用（JVM slot reuse：不同分支各有自己的 let）
            let_decl_check = re.compile(r'\blet\s+(?:mut\s+)?' + re.escape(name) + r'\b')
            for k2 in range(scope_close, len(rendered)):
                ln = rendered[k2]
                if word.search(ln):
                    if let_decl_check.search(ln):
                        break  # 另一个 let 声明，不是跨作用域读取
                    found = True
                    break

            if not found:
                # 检查 2：else 兄弟块中是否被引用
                # 当变量声明在 if-then 块内（nesting=N），而 "} else {" 也在 entry_nesting=N，
                # 则 else 块是同一层 if-else 的兄弟块，该变量在 else 块中不可见（Rust 块作用域）
                for k_else in range(decl_k + 1, scope_close):
                    else_text = rendered[k_else].lstrip()
                    if (entry_nesting[k_else] == decl_nesting
                            and else_text.startswith('} else')):
                        # 在 else 块内搜索引用（直到 nesting < decl_nesting）
                        for k_ref in range(k_else + 1, scope_close):
                            if entry_nesting[k_ref] < decl_nesting:
                                break
                            ln_ref = rendered[k_ref]
                            if word.search(ln_ref):
                                if let_decl_check.search(ln_ref):
                                    break  # 另一个 let 声明，不是读取
                                found = True
                                break
                    if found:
                        break

            if found:
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
        return False

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
        # 若 block_k 所在块或其任意外层块后有 "} else {" 且 else 分支中引用了该变量，
        # 则需继续向上提升，否则变量在 else 分支中不可见（E0425）
        # 策略：从 block_k 向外逐层检查每个外层块的 else；找到需要提升的最近层级后
        # 将 block_k 提升至该层，然后重新检查（直到没有更多需要提升为止）
        word_pat = word_cache.get(name) or re.compile(r'\b' + re.escape(name) + r'\b')
        let_decl_pat = re.compile(r'\blet\s+(?:mut\s+)?' + re.escape(name) + r'\b')
        max_hoist = 10  # 防止无限循环
        # next_start: 下一次外层循环从哪个块开始检查（与 block_k 分开跟踪）
        next_start = block_k
        while max_hoist > 0:
            max_hoist -= 1
            moved = False
            # 从 next_start 向外逐层检查每个块的 else 分支
            check_k = next_start
            check_nesting = entry_nesting[check_k]
            while True:
                # 检查 check_k 对应块的直接 } else {
                for k_else in range(check_k + 1, len(entries)):
                    if entry_nesting[k_else] < check_nesting:
                        break
                    if (entry_nesting[k_else] == check_nesting + 1
                            and rendered[k_else].lstrip().startswith('} else')):
                        found_in_else = False
                        for k_ref in range(k_else + 1, len(entries)):
                            if entry_nesting[k_ref] <= check_nesting:
                                break
                            ln = rendered[k_ref]
                            if word_pat.search(ln):
                                if let_decl_pat.search(ln):
                                    break
                                found_in_else = True
                                break
                        if found_in_else:
                            # 插入点是 check_k（在该 if-else 之前），而非其父块
                            block_k = check_k
                            # 下一轮从 check_k 的父块开始，避免重复检查同一 else
                            next_start = None
                            for bk in reversed(block_entry_indices):
                                if bk < check_k and entry_nesting[bk] < check_nesting:
                                    next_start = bk
                                    break
                            moved = True
                        break  # 只看直接 else，结果无论如何都 break
                if moved:
                    break
                # 向上找外层块
                outer_bk = None
                for bk in reversed(block_entry_indices):
                    if bk < check_k and entry_nesting[bk] < check_nesting:
                        outer_bk = bk
                        break
                if outer_bk is None:
                    break
                check_k = outer_bk
                check_nesting = entry_nesting[outer_bk]
            if not moved or next_start is None:
                break
        # 若变量已在函数体顶层声明（outer_decls），不再插入新 let（避免 shadow 类型冲突）
        if name in outer_decls:
            continue
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
    return True


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

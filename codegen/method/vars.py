"""
变量分析与提升：纯函数，无外部状态。
"""

import re
from .. import fallback_audit
from ..render import render_stmt, render_expr, render_type
from ..rs_ir import (
    StructLine,
    RsNamed, RsPrimitive, RsType,
    AssignStmt, LetStmt, Var, IfStmt, LoopStmt, RawExpr, RawStmt, UpcastExpr,
)
from ..constants import PRIMITIVE_RUST_TYPES as _PRIMITIVE_TYPES


def _coerce_icmp_operand(expr_str: str, ty_node) -> str:
    """为 if_icmpX 比较的操作数做类型强制转换：u16/i8/i16/bool → i32。

    boolean 在 JVM 操作数栈上就是 int（JVMS §2.11.1，Z 字段/局部经 getfield /
    iload 推入后与 int 常量 if_icmp 比较）；S-3.1 装箱对象化后 Z 槽位以 Rust
    bool 承载 → 比较前显式加宽，否则 `bool == i32` E0308（TreeMap$Entry.color
    的 `p.color == BLACK` 实证）。非原子表达式先整体加括号再转换。
    """
    from ..instr.coerce import _is_atomic_expr
    ty = render_type(ty_node)
    if ty in ('u16', 'i8', 'i16'):
        return f"({expr_str} as i32)"
    if ty == 'bool':
        # 非原子表达式先整体加括号（`a && b as i32` 会解析为 `a && (b as i32)`）
        if not _is_atomic_expr(expr_str):
            expr_str = f"({expr_str})"
        return f"({expr_str} as i32)"
    return expr_str


def _coerce_acmp_operand(expr_str: str, ty_node, registry=None, class_type_params=()) -> str:
    """为 if_acmpX 对象引用比较做类型强制转换：非 Object 类型统一上转为 Object。

    Java if_acmpeq/if_acmpne 是引用相等比较 → Object 的 PartialEq 按对象标识判定；上转必须保持
    对象标识：类 / 接口载体经 `Object::from`（持有同一对象），不能不透明装箱。
    """
    ty = render_type(ty_node)
    if ty == 'Object':
        return expr_str
    if ty in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16', 'usize', '()'):
        return expr_str  # 基本类型不应出现在 acmp，原样保留
    # 引用类型或 self 引用：去掉 &，clone 后上转
    clean = expr_str[1:] if expr_str.startswith('&') else expr_str
    from ..stack import erased_class_of
    if erased_class_of(ty, registry) is not None:
        # Clone::clone 而非 .clone()：值可能是带 Java clone() 的类（Enum_/HashMap 等）
        return f"Object::from(Clone::clone(&{clean}))"
    from ..instr.coerce import _coerce_to_object
    return _coerce_to_object(clean, ty, registry, class_type_params)


def _str_to_rs_type(s: str) -> RsType:
    """将 jvm_to_rust 返回的字符串转换为 RsType 节点。"""
    if s in _PRIMITIVE_TYPES:
        return RsPrimitive(s)
    return RsNamed(s)


def _entry_delta(text) -> int:
    """条目对块嵌套深度的净贡献（窗口 3 G-1b）：块结构行（StructLine）取 emitter
    给出的 delta，其余文本行与语句恒为 0——替代对渲染文本数花括号（迁移前经
    27 例 / 330 万次判定双算零分歧）。"""
    return text.delta if isinstance(text, StructLine) else 0


def _is_loop_head(text) -> bool:
    """loop 关键字头（`loop {` / `'lN: loop {` / 状态机 loop；while 头不在内）。"""
    return isinstance(text, StructLine) and text.tag == 'loop'


def _is_block_open(text) -> bool:
    """提升插入点候选：开块行，以及 `} else {` / `} else if … {` / `} catch … {`
    衔接行（与迁移前的文本判定逐点一致，见 G1-b 双算记录）。"""
    return isinstance(text, StructLine) and (text.delta == 1 or text.tag in ('else', 'catch'))


def _is_else_line(text) -> bool:
    """`} else {` / `} else if … {`。"""
    return isinstance(text, StructLine) and text.tag == 'else'


def _is_arm_or_try(text) -> bool:
    """match / 状态机臂头，或 java_try! 内层 `try {`（语句不可插在其与外层之间）。"""
    return isinstance(text, StructLine) and text.tag in ('arm', 'try')


# ── 变量引用检测（窗口 3 G1-c）──────────────────────────────────────────────
# 迁移前为渲染整行上的 \\bname\\b（字段名 / 方法名 / 字面量中的同名词亦命中）；
# 改为 IR 收集后经 27 例 6.4 万次提升决策双算零差异（语料中假阳性从未落在决定位置）。
_STR_LIT_RE = re.compile(r'"(?:[^"\\]|\\.)*"')


def _ir_names(node, out: set) -> None:
    from ..rs_ir import (Var, LetStmt, RawExpr, RawStmt, MacroExpr, Call, Lit,
                         FieldAccess, MethodCall, StaticFieldRef, CastExpr)
    if node is None or isinstance(node, (str, int, float, bool)):
        return
    if isinstance(node, list):
        for x in node:
            _ir_names(x, out)
        return
    if isinstance(node, Var):
        out.add(node.name)
        return
    if isinstance(node, (RawExpr, RawStmt)):
        out.update(re.findall(r'[A-Za-z_]\w*', node.code))
        return
    if isinstance(node, Lit):
        out.update(re.findall(r'[A-Za-z_]\w*', _STR_LIT_RE.sub('', node.value)))
        return
    if isinstance(node, MacroExpr):
        for a in node.args:
            out.update(re.findall(r'[A-Za-z_]\w*', _STR_LIT_RE.sub('', a)))
        return
    if isinstance(node, Call):
        out.update(re.findall(r'[A-Za-z_]\w*', node.func))
        _ir_names(node.args, out)
        return
    if isinstance(node, FieldAccess):
        _ir_names(node.recv, out)
        return
    if isinstance(node, MethodCall):
        _ir_names(node.recv, out); _ir_names(node.args, out)
        return
    if isinstance(node, StaticFieldRef):
        return
    if isinstance(node, CastExpr):
        _ir_names(node.expr, out)
        return
    if isinstance(node, LetStmt):
        out.add(node.name); _ir_names(node.value, out)
        return
    if hasattr(node, '__dataclass_fields__'):
        for f in node.__dataclass_fields__:
            v = getattr(node, f)
            if hasattr(v, '__dataclass_fields__') or isinstance(v, list):
                _ir_names(v, out)


def _refs(item, text: str, name: str) -> tuple[bool, bool]:
    """条目是否引用 name、是否为 name 的 let 声明（G1-c）：语句按 IR 收集变量名
    （字段名 / 方法名 / 类型 / 字符串字面量不计，Raw 文本与宏参数仍扫描）；
    结构行与非语句文本行按词边界扫描。"""
    from ..rs_ir import LetStmt
    if item is None or isinstance(item, str):
        hit = re.search(r'\b' + re.escape(name) + r'\b', text) is not None
        return hit, hit and re.search(r'\blet\s+(?:mut\s+)?' + re.escape(name) + r'\b', text) is not None
    names: set = set()
    _ir_names(item, names)
    return name in names, isinstance(item, LetStmt) and item.name == name


_REMOVED_ENTRY = ('', None)


def _hoisted_let_type(item):
    """前置声明的类型标注：声明自带标注优先；省略标注的声明取其模拟类型
    （含待推断实参 `_` 的类型不能作标注，仍交给 Rust 推断）。"""
    if not isinstance(item, LetStmt):
        return None
    if item.ty is not None:
        return item.ty
    if item.value_ty is not None and not re.search(r'(?<![A-Za-z0-9_])_(?![A-Za-z0-9_])',
                                                   render_type(item.value_ty)):
        return item.value_ty
    return None


def _demote_let(entries: list, k: int) -> None:
    """提升后，原位置的 `let x: T = v;` 降为赋值 `x = v;`；无初值的前置声明
    （`let mut x: T;`）已被提升点的声明取代，标记删除（由 _drop_removed 统一清理）。"""
    indent, item = entries[k]
    if item.value is None:
        entries[k] = _REMOVED_ENTRY
    else:
        entries[k] = (indent, AssignStmt(Var(item.name), item.value, _hoisted_let_type(item),
                                         slot=item.slot, bind_off=item.bind_off))


_ROOT_TYPE = 'Object'


def _is_default_value(value) -> bool:
    """占位初值（无值的前置声明同视为无值可对齐，G-2）。"""
    return value is None or (isinstance(value, RawExpr) and value.code == 'Default::default()')


def _merged_slot_type(entries: list, name: str, start: int, end: int, registry=None):
    """(start, end) 内同名声明 / 降级赋值的引用类型不一致时，返回汇合类型；否则 None。

    同一 slot 在兄弟分支存入不同引用类型时，JVM 校验器在控制流合并点取公共祖先
    （JVM 局部变量的静态类型是其声明类型）：汇合类型取继承链上最近的公共类祖先
    （`TreeNode<K,V>` 与 `Node<K,V>` → `Node<K,V>`，各存入侧经 From 上转，保持
    对象标识与运行时类）；无公共类祖先（接口 / 不同实例化 / 互不相干类）时回退
    根类，存入侧按装箱上转（try-finally 暂存槽等只以 Object 形态流动的合成槽）。"""
    seen: list[str] = []
    for k in range(start + 1, end):
        item = entries[k][1]
        if isinstance(item, LetStmt) and item.name == name:
            ty = _hoisted_let_type(item)
        elif (isinstance(item, AssignStmt) and isinstance(item.target, Var)
              and item.target.name == name):
            ty = item.value_ty
        else:
            continue
        if ty is None:
            continue
        rendered = render_type(ty)
        if rendered in _PRIMITIVE_TYPES or rendered == '()':
            return None
        if rendered not in seen:
            seen.append(rendered)
    if len(seen) < 2:
        return None
    from ..instr.hierarchy import _common_ref_type_widening
    common = seen[0]
    for other in seen[1:]:
        common = _common_ref_type_widening(common, other, registry)
        if common is None:
            return RsNamed(_ROOT_TYPE)
    return RsNamed(common)


def _widen_into_merged(entries: list, name: str, start: int, end: int,
                       merged: RsNamed, box_object) -> None:
    """汇合类型为公共类祖先的槽：各次存入按 From 上转（`.into()`，保持对象标识）。
    汇合类型为根类（无公共类祖先）时装箱上转（沿用原语义）。"""
    _is_root = render_type(merged) == _ROOT_TYPE
    for k in range(start + 1, end):
        indent, item = entries[k]
        if isinstance(item, LetStmt) and item.name == name:
            ty = _hoisted_let_type(item)
        elif (isinstance(item, AssignStmt) and isinstance(item.target, Var)
              and item.target.name == name):
            ty = item.value_ty
        else:
            continue
        if ty is None or item.value is None or _is_default_value(item.value):
            if isinstance(item, LetStmt):
                item.ty = merged if item.ty is not None else None
                item.value_ty = merged
            else:
                item.value_ty = merged
            continue
        rendered = render_type(ty)
        if rendered != render_type(merged):
            if _is_root:
                item.value = RawExpr(box_object(render_expr(item.value), rendered))
            else:
                # 公共祖先路径：值是该祖先的子类型，`.into()` 目标由汇合后的声明类型给出
                item.value = UpcastExpr(item.value, 'auto')
        if isinstance(item, LetStmt):
            item.ty = merged if item.ty is not None else None
        item.value_ty = merged


def _lvt_covering_entry(entries: list, off: int, name: str):
    """LVT 声明区间中覆盖 store 偏移 off 的**本名**条目 (start, end, name, ty)。

    覆盖判定同 stack._decl_at：存活区间内（start <= off < end）的读 / 再赋值，
    或初始化 store（javac 约定 LVT start = 初始化 store 的下一条指令偏移，
    store 落 start 前 1~4 字节：一字节 astore_N 为 start-1，两字节 astore N
    为 start-2，ObjectStreamClass.getProtectionDomains 的 pds 槽实证 start-2，
    留 4 字节余量）。覆盖 off 的区间名字不符（槽被别的变量占用）不算——
    无本名覆盖 → None（该存点是 javac 合成绑定，不在 LVT）。
    """
    from ..stack import _safe_name
    for start, end, nm, ty, *_rest in entries:
        if _safe_name(nm) != name:
            continue
        if start <= off < end or (off < start <= off + 4):
            return (start, end, nm, ty)
    return None


def _same_jvm_var(a, b, slot_decls) -> bool | None:
    """LVT 区间驱动的变量身份：同名两语句是否同一 JVM 变量。

    证据源是 LocalVariableTable 的 (start, length) 活跃区间（slot_decls：
    slot → [(start, end, name, ty, ...)]，start/end 为字节码偏移）：
    - 异槽同名必是两个 JVM 变量（`int i` 槽 9 / `long i` 槽 19——long 占
      双槽，javac 给同名异型变量分不同槽）→ False；
    - 存点未被本名区间覆盖（javac 合成绑定：for-each 迭代器、synchronized
      监视对象——G-3 的 local_N 家族，槽上只有**别人**的区间）→ None，
      无身份证据，调用方维持原有名字 + 类型身份模型；
    - 同槽本名覆盖区间声明类型不一致（`Node p` 与复用同槽的 `TreeNode p`：
      区间内 name 均为 p 但 LVT 签名异型）→ False；
    - 同槽本名同型且两存点间该槽无**异名**区间换主（javac 按控制流把一个
      变量的活跃区间拆成多条 LVT 条目：CHM.transfer 的 ln 在 runBit 两
      分支各一段，[420,426) ∪ [433,551) 同槽同名同型）→ True，是同一变量
      的分段活跃区间，必须并入同一 Rust 绑定。
    """
    if a is None or b is None or slot_decls is None:
        return None
    sa = getattr(a, 'slot', None)
    sb = getattr(b, 'slot', None)
    oa = getattr(a, 'bind_off', None)
    ob = getattr(b, 'bind_off', None)
    if sa is None or sb is None or oa is None or ob is None:
        return None
    if sa != sb:
        return False
    entries = slot_decls.get(sa)
    if not entries:
        return None
    from ..stack import _safe_name
    name = getattr(a, 'name', None)
    if name is None and getattr(a, 'target', None) is not None:
        name = getattr(a.target, 'name', None)
    ea = _lvt_covering_entry(entries, oa, name)
    eb = _lvt_covering_entry(entries, ob, name)
    if ea is None or eb is None:
        return None
    ta, tb = ea[3], eb[3]
    try:
        ra = render_type(ta) if ta is not None else None
        rb = render_type(tb) if tb is not None else None
    except Exception:
        return None
    if ra != rb:
        return False
    lo, hi = (oa, ob) if oa <= ob else (ob, oa)
    for start, end, nm, *_rest in entries:
        if _safe_name(nm) != name and start < hi and end > lo:
            return False   # 两存点间该槽被异名区间换主：另一个变量
    return True


def _forms_alignable(later_ty_s: str | None, hoisted_ty_s: str | None, registry=None) -> bool:
    """后到形态能否对齐到提升声明类型（并入同一绑定）：
    同型直接可赋；引用子类型经 `.into()` 上转（JVM 槽位声明类型语义，
    HashSet 存入 Set 声明槽）。不成立（基本类型 vs 引用 / 无关引用，
    如 try 结果暂存 i32 vs synchronized 监视对象 Object、String vs Object）
    时两形态是同槽不同 JVM 变量（G-3 活跃区间分型），必须拆分绑定。"""
    if later_ty_s is None or hoisted_ty_s is None:
        return True   # 无类型信息可判：维持并入（旧行为）
    if later_ty_s == hoisted_ty_s:
        return True
    from ..stack import erased_base
    later_base = erased_base(later_ty_s)
    hoisted_base = erased_base(hoisted_ty_s)
    if later_base in _PRIMITIVE_TYPES or hoisted_base in _PRIMITIVE_TYPES:
        return False
    from ..instr.hierarchy import _is_subtype
    return _is_subtype(later_base, hoisted_base, registry)


def _align_store_value(item, hoisted_type, later_ty_s: str, hoisted_ty_s: str) -> None:
    """把待降级声明的值侧对齐到提升声明类型（子类型 → `.into()`，目标由
    降级赋值的接收者类型给出）。无类型标注的声明同时补 value_ty。"""
    if _is_default_value(item.value):
        item.value_ty = hoisted_type
        return
    if later_ty_s != hoisted_ty_s:
        item.value = UpcastExpr(item.value, 'auto')
    item.value_ty = hoisted_type


def _drop_removed(entries: list) -> None:
    entries[:] = [e for e in entries if e is not _REMOVED_ENTRY]


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


def _hoist_loop_vars(entries: list, predeclared: set[str], slot_decls=None):
    """将在 loop{} 内 let-声明但在 loop 外被读取的变量提升到 loop 前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 loop 内首次 let-声明，
    但在 loop 退出后被读取，Rust 报 E0425。
    修复：在 loop 前插入无初值前置声明 let mut NAME: T;（G-2），loop 内改为赋值。
    slot_decls：LVT 声明表（slot → [(start, end, name, ...)]），驱动
    _same_jvm_var 的变量身份判定。
    """
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置
    nesting = 0
    declared_at: dict[str, tuple[int, int]] = {}  # name → (index, nesting_depth)
    loop_entry_indices: list[int] = []  # loop { 的索引

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            if _is_loop_head(item):
                loop_entry_indices.append(k)
            delta = _entry_delta(item)
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
                # B 组计数（fallback-audit §4.1）：render 失败必是 bug（空串会让
                # 嵌套深度算错、提升判定错乱）——先可观测，strict 下穿透
                if fallback_audit.STRICT:
                    raise
                fallback_audit.record('vars-render-loop')
                rendered.append('')

    # 追踪每个条目的嵌套深度
    entry_nesting: list[int] = []
    cur = 0
    for text in rendered:
        entry_nesting.append(cur)
        cur += _entry_delta(text)

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
        for k2 in range(scope_close, len(rendered)):
            hit, is_let = _refs(entries[k2][1], rendered[k2], name)
            if hit:
                if is_let:
                    break  # 另一个 let 声明，不是跨作用域读取
                vars_to_hoist.add(name)
                break

    if not vars_to_hoist:
        return

    # Pass 3: 对需要提升的变量进行修改
    # 先收集所有插入操作（在 loop 前插入 let mut NAME: T;）
    # 用倒序插入，避免索引偏移
    # G-4 确定性：按名排序遍历——同一 loop 位置的多变量插入序否则随 set 迭代序
    # （PYTHONHASHSEED）漂移，生成物不可 diff
    insertions: list[tuple[int, tuple]] = []  # (index, entry) to insert BEFORE
    for name in sorted(vars_to_hoist):
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
        # 从声明处取类型注解（无初值声明需类型标注或后续赋值推断，避免 E0282）
        inner_indent, inner_item = entries[decl_k]
        hoisted_type = _hoisted_let_type(inner_item)
        # 插入的提升声明继承被提升声明的 JVM 身份（槽位 / store 偏移）：
        # 后续轮次的 _same_jvm_var 身份判定据此把同变量的分段 let 并回本绑定
        insertions.append((loop_k, (loop_indent, LetStmt(
            name, hoisted_type, True, None,
            slot=getattr(inner_item, 'slot', None),
            bind_off=getattr(inner_item, 'bind_off', None)))))
        # 将 loop 内的 LetStmt 改为 AssignStmt
        if isinstance(inner_item, LetStmt):
            _demote_let(entries, decl_k)

    # 倒序插入，避免索引偏移（同一 loop 有多个变量时均插入到 loop 前）
    for ins_k, ins_entry in sorted(insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)
    _drop_removed(entries)


class _HoistState:
    """_hoist_if_vars 分阶段（窗口 3 ⑩）之间传递的局部状态：各阶段函数开头解包所用
    名、结尾写回所赋名，阶段体保持原函数体逐字不变（循环体段去一级缩进，外层
    continue 改为写回后 return True）。"""

    def __init__(self, entries, predeclared, box_object, lvt_names, registry, slot_decls):
        for _n in _HOIST_STATE_NAMES:
            setattr(self, _n, None)
        self.entries, self.predeclared, self.box_object = entries, predeclared, box_object
        self.lvt_names, self.registry, self.slot_decls = lvt_names, registry, slot_decls


_HOIST_STATE_NAMES = ('_', '_dn', '_hoisted_s', '_later', '_later_s', '_outer_s',
    '_outer_split', '_outer_ty', '_outer_ty_s', '_t', 'arm_nesting', 'bk',
    'block_entry_indices', 'block_indent', 'block_item', 'block_k', 'box_object', 'check_k',
    'check_nesting', 'ck', 'ck_indent', 'ck_item', 'cur', 'd_item', 'decl_k', 'decl_list',
    'decl_nesting', 'declared_at', 'default_val', 'delta', 'dk', 'entries',
    'entry_nesting', 'first_decl_k', 'first_decl_nesting', 'first_item', 'first_let', 'found',
    'found_decl', 'found_in_else', 'hoisted_type', 'indent', 'inner_indent', 'inner_item',
    'ins_entry', 'ins_k', 'insertions', 'item', 'k', 'k2', 'k_else', 'k_ref', 'lvt_names', 'max_hoist', 'merged_type', 'moved', 'name',
    'nesting', 'next_start', 'outer_bk', 'outer_decl_first_k', 'outer_decls', 'outer_item',
    'outer_k', 'parent_k', 'predeclared', 'ref_idx', 'ref_nesting', 'registry', 'rendered',
    'scope_close', 'slot_decls', 'span_end', 'text', 'ty_str', 'vars_to_hoist',
    )


def _hoist_if_vars(entries: list, predeclared: set[str], box_object=None,
                   lvt_names: frozenset = frozenset(), registry=None, slot_decls=None) -> bool:
    """将在 if/else 块内 let-声明但在块外被读取的变量提升到块前。

    JVM 局部变量槽是函数级作用域，Rust 是块级。若变量在 if/else 内首次 let-声明，
    但在 if-else 结束后被读取，Rust 报 E0425。
    修复：在 if 前插入无初值前置声明 let mut NAME: TYPE;（G-2），
    块内所有同名 LetStmt 改为 AssignStmt。
    slot_decls：LVT 声明表（slot → [(start, end, name, ...)]），驱动
    _same_jvm_var 的变量身份判定。

    返回 True 表示本次有提升，调用方可循环直到返回 False。
    """
    h = _HoistState(entries, predeclared, box_object, lvt_names, registry, slot_decls)
    _hoist_if_scan(h)
    if not h.declared_at or not h.block_entry_indices:
        return
    _hoist_if_render(h)
    _hoist_if_select(h)
    if not h.vars_to_hoist:
        return False
    # Pass 4（逐变量）：定位提升块 → 顶层声明处理 → 插入提升声明并降级块内同名 let
    h.insertions = []
    for name, (ty_str, decl_list, ref_idx, found_decl) in h.vars_to_hoist.items():
        h.name, h.ty_str, h.decl_list, h.ref_idx, h.found_decl = name, ty_str, decl_list, ref_idx, found_decl
        if _hoist_if_locate(h):
            continue
        if _hoist_if_outer(h):
            continue
        _hoist_if_emit(h)
    for ins_k, ins_entry in sorted(h.insertions, key=lambda x: -x[0]):
        entries.insert(ins_k, ins_entry)
    _drop_removed(entries)
    return True


def _hoist_if_scan(h: "_HoistState"):
    """Pass 1：收集嵌套块内的 let 声明（位置 / 深度）、顶层声明与块起始索引。"""
    (block_entry_indices, declared_at, delta, entries, indent, item, k, nesting,
     outer_decl_first_k, outer_decls, predeclared
    ) = (h.block_entry_indices, h.declared_at, h.delta, h.entries, h.indent, h.item, h.k,
         h.nesting, h.outer_decl_first_k, h.outer_decls, h.predeclared
    )
    # Pass 1: 收集所有在嵌套块中声明的变量及其位置、类型
    nesting = 0
    # name → list of (entry_index, nesting_depth)
    declared_at: dict[str, list[tuple[int, int]]] = {}
    # 函数体顶层（nesting=0）声明的变量：已有函数级作用域，不应被 hoist 覆盖
    outer_decls: set[str] = set()
    # 顶层声明的最早出现索引（用于判断是否晚于 ref_idx）
    outer_decl_first_k: dict[str, int] = {}
    # index of any block-opening entry (ends with '{' and contains 'if' or has any '{')
    block_entry_indices: list[int] = []

    for k, (indent, item) in enumerate(entries):
        if isinstance(item, str):
            if _is_block_open(item):
                block_entry_indices.append(k)
            delta = _entry_delta(item)
            nesting += delta
        elif isinstance(item, LetStmt):
            if nesting == 0 and item.name not in predeclared:
                outer_decls.add(item.name)
                if item.name not in outer_decl_first_k:
                    outer_decl_first_k[item.name] = k
            elif nesting > 0 and item.name not in predeclared:
                declared_at.setdefault(item.name, []).append((k, nesting))
    (h.block_entry_indices, h.declared_at, h.delta, h.indent, h.item, h.k, h.nesting,
     h.outer_decl_first_k, h.outer_decls
    ) = (block_entry_indices, declared_at, delta, indent, item, k, nesting, outer_decl_first_k,
         outer_decls
    )


def _hoist_if_render(h: "_HoistState"):
    """Pass 2：条目渲染为文本并逐条记录嵌套深度。"""
    (_, cur, entries, entry_nesting, item, rendered, text
    ) = (h._, h.cur, h.entries, h.entry_nesting, h.item, h.rendered, h.text
    )
    # Pass 2: 渲染所有条目为字符串，追踪嵌套深度
    rendered: list[str] = []
    for _, item in entries:
        if isinstance(item, str):
            rendered.append(item)
        else:
            try:
                rendered.append(render_stmt(item))
            except Exception:
                # B 组计数（fallback-audit §4.1）：同 _hoist_loop_vars Pass2
                if fallback_audit.STRICT:
                    raise
                fallback_audit.record('vars-render-if')
                rendered.append('')

    entry_nesting: list[int] = []
    cur = 0
    for text in rendered:
        entry_nesting.append(cur)
        cur += _entry_delta(text)

    (h._, h.cur, h.entry_nesting, h.item, h.rendered, h.text
    ) = (_, cur, entry_nesting, item, rendered, text
    )


def _hoist_if_select(h: "_HoistState"):
    """Pass 3：选出在声明块外（或 else 兄弟块）被引用、需要提升的变量。"""
    (_, decl_k, decl_list, decl_nesting, declared_at, entries, entry_nesting,
     first_item, found, k2, k_else, k_ref, name, ref_idx,
     rendered, scope_close, ty_str, vars_to_hoist
    ) = (h._, h.decl_k, h.decl_list, h.decl_nesting, h.declared_at, h.entries,
         h.entry_nesting, h.first_item, h.found, h.k2, h.k_else, h.k_ref, h.name, h.ref_idx, h.rendered, h.scope_close, h.ty_str,
         h.vars_to_hoist
    )
    # Pass 3: 找出需要提升的变量 - 在块外或 else 兄弟块中被引用
    # name → (type_str, decl_list, ref_idx)  ref_idx：触发 found=True 的外部引用位置（-1 表示来自 else 兄弟块）
    vars_to_hoist: dict[str, tuple[str | None, list[tuple[int, int]], int]] = {}
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

            found = False
            ref_idx = -1  # 触发 found=True 的外部引用索引（-1 表示 else 兄弟块引用）
            # 检查 1：作用域关闭后是否被引用（原有逻辑）
            # 若第一个匹配是另一个 let 声明（同名变量在另一分支的单独绑定），
            # 则不算跨作用域引用（JVM slot reuse：不同分支各有自己的 let）
            for k2 in range(scope_close, len(rendered)):
                hit, is_let = _refs(entries[k2][1], rendered[k2], name)
                if hit:
                    if is_let:
                        break  # 另一个 let 声明，不是跨作用域读取
                    found = True
                    ref_idx = k2
                    break

            if not found:
                # 检查 2：else 兄弟块中是否被引用
                # 当变量声明在 if-then 块内（nesting=N），而 "} else {" 也在 entry_nesting=N，
                # 则 else 块是同一层 if-else 的兄弟块，该变量在 else 块中不可见（Rust 块作用域）
                for k_else in range(decl_k + 1, scope_close):
                    if (entry_nesting[k_else] == decl_nesting
                            and _is_else_line(rendered[k_else])):
                        # 在 else 块内搜索引用（直到 nesting < decl_nesting）
                        for k_ref in range(k_else + 1, scope_close):
                            if entry_nesting[k_ref] < decl_nesting:
                                break
                            hit, is_let = _refs(entries[k_ref][1], rendered[k_ref], name)
                            if hit:
                                if is_let:
                                    break  # 另一个 let 声明，不是读取
                                found = True
                                break
                    if found:
                        break

            if found:
                # 确定类型：取触发提升的那次声明（同名槽在互不相干的作用域里可以是
                # 不同的 Java 变量，类型也可能不同，如 switch 各 case 里的 int i / long i）
                ty_str = None
                _, first_item = entries[decl_k]
                if isinstance(first_item, LetStmt) and first_item.ty is not None:
                    try:
                        ty_str = render_type(first_item.ty)
                    except Exception:
                        # B 组计数：类型串 None → 对齐检查跳过（静默降级可观测化）
                        if fallback_audit.STRICT:
                            raise
                        fallback_audit.record('vars-type-decl')
                        ty_str = None
                vars_to_hoist[name] = (ty_str, decl_list, ref_idx, (decl_k, decl_nesting))
                break
    (h._, h.decl_k, h.decl_list, h.decl_nesting, h.first_item, h.found, h.k2,
     h.k_else, h.k_ref, h.name, h.ref_idx, h.scope_close,
     h.ty_str, h.vars_to_hoist
    ) = (_, decl_k, decl_list, decl_nesting, first_item, found, k2, k_else, k_ref,
         name, ref_idx, scope_close, ty_str, vars_to_hoist
    )


def _hoist_if_locate(h: "_HoistState") -> bool:
    """Pass 4a：定位提升块（最内层包含首次声明的块；match 臂 / try 块
    上移；else 分支引用逐层外提）。返回 True = 无可用块，跳过该变量。"""
    (arm_nesting, bk, block_entry_indices, block_k, check_k, check_nesting, entries,
     entry_nesting, first_decl_k, first_decl_nesting, found_decl, found_in_else, k_else,
     k_ref, max_hoist, moved, name, next_start, outer_bk, parent_k,
     rendered
    ) = (h.arm_nesting, h.bk, h.block_entry_indices, h.block_k, h.check_k, h.check_nesting,
         h.entries, h.entry_nesting, h.first_decl_k, h.first_decl_nesting, h.found_decl,
         h.found_in_else, h.k_else, h.k_ref, h.max_hoist, h.moved,
         h.name, h.next_start, h.outer_bk, h.parent_k, h.rendered
    )
    first_decl_k, first_decl_nesting = found_decl
    # 找到包含第一次声明的最近的块起始索引
    block_k = None
    for bk in reversed(block_entry_indices):
        if bk < first_decl_k and entry_nesting[bk] < first_decl_nesting:
            block_k = bk
            break
    if block_k is None:
        (h.arm_nesting, h.bk, h.block_k, h.check_k, h.check_nesting, h.first_decl_k,
         h.first_decl_nesting, h.found_in_else, h.k_else, h.k_ref, h.max_hoist, h.moved, h.next_start, h.outer_bk, h.parent_k
        ) = (arm_nesting, bk, block_k, check_k, check_nesting, first_decl_k,
             first_decl_nesting, found_in_else, k_else, k_ref, max_hoist,
             moved, next_start, outer_bk, parent_k
        )
        return True
    # 若 block_k 落在 match arm（含 =>）内，向上找到 match 语句本身，
    # 否则插入位置落在两个 arm 之间，产生"expected pattern, found 'let'"错误
    # `try {`（java_try! 的内层块）同理：宏语法不允许在 `java_try! {` 与 `try {` 之间出现语句
    while _is_arm_or_try(rendered[block_k]):
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
                        and _is_else_line(rendered[k_else])):
                    found_in_else = False
                    for k_ref in range(k_else + 1, len(entries)):
                        if entry_nesting[k_ref] <= check_nesting:
                            break
                        hit, is_let = _refs(entries[k_ref][1], rendered[k_ref], name)
                        if hit:
                            if is_let:
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
    (h.arm_nesting, h.bk, h.block_k, h.check_k, h.check_nesting, h.first_decl_k,
     h.first_decl_nesting, h.found_in_else, h.k_else, h.k_ref, h.max_hoist, h.moved, h.next_start, h.outer_bk, h.parent_k
    ) = (arm_nesting, bk, block_k, check_k, check_nesting, first_decl_k, first_decl_nesting,
         found_in_else, k_else, k_ref, max_hoist, moved, next_start,
         outer_bk, parent_k
    )
    return False


def _hoist_if_outer(h: "_HoistState") -> bool:
    """Pass 4b：顶层已有同名声明时的处理（已可见 → 分段 let 降级并回同一
    绑定；晚于引用 → 顶层声明一并降级，或按 G-3 分型拆分保留）。返回 True = 已处理。"""
    (_dn, _later, _later_s, _outer_s, _outer_split, _outer_ty, _outer_ty_s, _t, ck,
     ck_indent, ck_item, d_item, decl_list, dk, entries, name, outer_decl_first_k,
     outer_decls, outer_item, outer_k, ref_idx, registry, slot_decls, ty_str
    ) = (h._dn, h._later, h._later_s, h._outer_s, h._outer_split, h._outer_ty, h._outer_ty_s,
         h._t, h.ck, h.ck_indent, h.ck_item, h.d_item, h.decl_list, h.dk, h.entries, h.name,
         h.outer_decl_first_k, h.outer_decls, h.outer_item, h.outer_k, h.ref_idx, h.registry,
         h.slot_decls, h.ty_str
    )
    # 若变量已在函数体顶层声明（outer_decls），通常不再插入新 let（避免 shadow 类型冲突）。
    # 例外：若顶层声明出现在 ref_idx 之后（即声明晚于引用），说明顶层声明本身也在错误位置，
    # 仍需提升，并将该顶层声明一并转为 AssignStmt。
    if name in outer_decls:
        outer_k = outer_decl_first_k.get(name, -1)
        if ref_idx < 0 or outer_k < 0 or outer_k <= ref_idx:
            # 顶层声明在 ref 之前或没有外部 ref → 已可见，无需提升。
            # 但同名分段 let（javac 按控制流把一个 JVM 变量的活跃区间拆成
            # 多条 LVT 条目，兄弟分支各自的绑定；LVT 同槽同名区间证据，
            # _same_jvm_var）会在自己的块内遮蔽顶层绑定吞掉分支内赋值
            # （CHM.transfer 的 ln/hn：resize 丢节点）——降级为赋值并回
            # 同一绑定。类型不可对齐的形态是同槽两个 JVM 变量（G-3 家族，
            # 身份判定 False / 无证据 None），不在降级之列，保留各自 let。
            outer_item = entries[outer_k][1]
            for dk, _dn in decl_list:
                if dk <= outer_k:
                    continue
                d_item = entries[dk][1] if dk < len(entries) else None
                if not (isinstance(d_item, LetStmt) and d_item.name == name):
                    continue
                if _same_jvm_var(outer_item, d_item, slot_decls) is not True:
                    continue
                _outer_ty = _hoisted_let_type(outer_item)
                if _outer_ty is not None:
                    _later = _hoisted_let_type(d_item)
                    try:
                        _later_s = render_type(_later) if _later is not None else None
                    except Exception:
                        # B 组计数：值侧对齐检查跳过（静默降级可观测化）
                        if fallback_audit.STRICT:
                            raise
                        fallback_audit.record('vars-type-later')
                        _later_s = None
                    _outer_s = render_type(_outer_ty)
                    if (_later_s is not None and _later_s != _outer_s
                            and _forms_alignable(_later_s, _outer_s, registry)
                            and not _is_default_value(d_item.value)):
                        _align_store_value(d_item, _outer_ty, _later_s, _outer_s)
                _demote_let(entries, dk)
            (h._dn, h._later, h._later_s, h._outer_s, h._outer_split, h._outer_ty,
             h._outer_ty_s, h._t, h.ck, h.ck_indent, h.ck_item, h.d_item, h.dk,
             h.outer_item, h.outer_k
            ) = (_dn, _later, _later_s, _outer_s, _outer_split, _outer_ty, _outer_ty_s, _t, ck,
                 ck_indent, ck_item, d_item, dk, outer_item, outer_k
            )
            return True
        # 顶层声明在 ref 之后 → 需要提升；同时将该顶层声明也转为 AssignStmt
        # （否则它会成为第二次声明，遮蔽提升后的 let，引发新的 E0425）。
        # 例外（G-3 槽位复用，如 BufferedReader.read 的 slot 6）：顶层后到的
        # let 是同槽另一个 JVM 变量（synchronized 监视对象 Object，活跃区间
        # 与 try 结果暂存 i32/String 不相交，slot 不在 LVT）——类型不可对齐时
        # 保留它自己的 let：词法作用域天然隔离两形态，且后续扫描的
        # let_decl_check 以该 let 为界不再把形态 A 的声明再提升到 if 之外，
        # 消除「单一绑定承载 i32 与 Object 两形态」的 E0308。
        outer_item = entries[outer_k][1]
        _outer_ty_s: str | None = None
        _outer_split = False
        if isinstance(outer_item, LetStmt):
            try:
                _t = _hoisted_let_type(outer_item)
                _outer_ty_s = render_type(_t) if _t is not None else None
            except Exception:
                # B 组计数：拆分判定跳过（G-3 槽位复用形态判定失真可观测化）
                if fallback_audit.STRICT:
                    raise
                fallback_audit.record('vars-type-outer')
                _outer_ty_s = None
            # 类型不可对齐 → 同槽两个 JVM 变量（G-3 活跃区间分型）：
            # 保留顶层后到 let 为独立绑定（词法作用域隔离），不降级、不并入
            _outer_split = (ty_str is not None
                            and not _forms_alignable(_outer_ty_s, ty_str, registry))
        if not _outer_split:
            for ck in range(outer_k, len(entries)):
                ck_indent, ck_item = entries[ck]
                if isinstance(ck_item, LetStmt) and ck_item.name == name:
                    if (_outer_ty_s is not None and ty_str is not None
                            and _outer_ty_s != ty_str and not _is_default_value(ck_item.value)):
                        _align_store_value(ck_item, _str_to_rs_type(ty_str),
                                           _outer_ty_s, ty_str)
                    _demote_let(entries, ck)
    (h._dn, h._later, h._later_s, h._outer_s, h._outer_split, h._outer_ty, h._outer_ty_s,
     h._t, h.ck, h.ck_indent, h.ck_item, h.d_item, h.dk, h.outer_item, h.outer_k
    ) = (_dn, _later, _later_s, _outer_s, _outer_split, _outer_ty, _outer_ty_s, _t, ck,
         ck_indent, ck_item, d_item, dk, outer_item, outer_k
    )
    return False


def _hoist_if_emit(h: "_HoistState") -> bool:
    """Pass 4c：引用可见性上移校验，登记提升声明插入点，块内同名 let 值侧
    对齐后降级为赋值。"""
    (_, _hoisted_s, _later, _later_s, bk, block_entry_indices, block_indent, block_item,
     block_k, box_object, decl_k, decl_list, default_val, entries, entry_nesting,
     first_decl_k, first_let, hoisted_type, inner_indent, inner_item, insertions, k2,
     lvt_names, merged_type, name, parent_k, ref_idx, ref_nesting, registry, slot_decls,
     span_end
    ) = (h._, h._hoisted_s, h._later, h._later_s, h.bk, h.block_entry_indices, h.block_indent,
         h.block_item, h.block_k, h.box_object, h.decl_k, h.decl_list, h.default_val,
         h.entries, h.entry_nesting, h.first_decl_k, h.first_let, h.hoisted_type,
         h.inner_indent, h.inner_item, h.insertions, h.k2, h.lvt_names, h.merged_type, h.name,
         h.parent_k, h.ref_idx, h.ref_nesting, h.registry, h.slot_decls, h.span_end
    )
    # ref_nesting 校验：声明插在 block_k 之前（与 block_k 同层），需保证引用在该层可见。
    if ref_idx >= 0:
        ref_nesting = entry_nesting[ref_idx]
        while block_k is not None:
            # 声明插在 block_k 之前（与 block_k 同层），作用域涵盖 block_k 处及之后所有同层/更深位置。
            # 若引用 nesting >= entry_nesting[block_k]，声明对引用可见，停止上移。
            if ref_nesting >= entry_nesting[block_k]:
                break
            # 引用在 block_k 开启的块的外部，需向上找外层块
            parent_k = None
            for bk in reversed(block_entry_indices):
                if bk < block_k and entry_nesting[bk] < entry_nesting[block_k]:
                    parent_k = bk
                    break
            if parent_k is None:
                break
            block_k = parent_k

    block_indent, block_item = entries[block_k]
    if isinstance(block_item, str):      # 文本行的缩进写在文本里
        block_indent = block_item[:len(block_item) - len(block_item.lstrip())]
    # 获取类型注解节点（来自第一次声明）
    _, first_let = entries[first_decl_k]
    hoisted_type = _hoisted_let_type(first_let)
    # G-2：前置声明不带占位初值——「未初始化即使用」由 Rust 确定赋值分析验证（与 JVM
    # 校验器同语义），不再以 Default::default() 掩盖，亦不要求类型实现 Default
    default_val = None
    # 将提升点所辖语句（block_k 开启的整条 if/else、match、loop 语句）内的同名 LetStmt
    # 改为 AssignStmt；语句之外的同名声明是别的 Java 变量，保持各自的 let
    span_end = len(entries)
    for k2 in range(block_k + 1, len(entries)):
        if entry_nesting[k2] <= entry_nesting[block_k]:
            span_end = k2
            break
    if box_object is not None:
        # 同名异型：按 JVM 合并点语义取公共祖先 widening；无公共类祖先时回退根类装箱。
        # 合成槽（无声明类型）与 LVT 具名变量同规则——后者如声明为接口、兄弟分支存入
        # 两个不同实现类（InetAddress.createBuiltinInetAddressResolver 的 theResolver：
        # HostsFileResolver / PlatformResolver，E0308）。同型 / 含基本类型时返回 None，
        # 不改变既有产物；子类型对齐的旧路径（公共祖先即提升类型）结果相同
        merged_type = _merged_slot_type(entries, name, block_k, span_end, registry)
        if merged_type is not None:
            _widen_into_merged(entries, name, block_k, span_end, merged_type, box_object)
            hoisted_type = merged_type
    insertions.append((block_k, (block_indent, LetStmt(
        name, hoisted_type, True, default_val,
        slot=getattr(first_let, 'slot', None),
        bind_off=getattr(first_let, 'bind_off', None)))))
    for decl_k, _ in decl_list:
        if not (block_k < decl_k < span_end):
            continue
        inner_indent, inner_item = entries[decl_k]
        if isinstance(inner_item, LetStmt) and inner_item.name == name:
            # 身份证据判定的异槽同名（LVT 证据 _same_jvm_var is False）：span 内
            # 的同名声明是另一个 JVM 变量（slot 复用换主），不并入本提升绑定，
            # 保留其自己的 let（词法作用域隔离，G-3 家族语义）；无证据（None，
            # 合成槽 / 未标注）维持原有并入降级行为
            if _same_jvm_var(first_let, inner_item, slot_decls) is False:
                continue
            # 降级前值侧对齐到提升声明类型：兄弟分支同名不同形（javac 三元两臂
            # 拆两条 LVT 区间，stack.py 按槽复用走 let 阴影——else 臂 HashSet 存入
            # Set 声明槽），子类型经 `.into()` 上转（Files.newByteChannel 的 set /
            # FileSystemProvider.newByteChannel 的 opts）。类型不可对齐的形态
            #（基本 vs 引用）不在此路径——那类冲突由顶层后到 let 的保留拆分承载。
            if hoisted_type is not None:
                _later = _hoisted_let_type(inner_item)
                try:
                    _later_s = render_type(_later) if _later is not None else None
                except Exception:
                    # B 组计数：值侧对齐检查跳过（静默降级可观测化）
                    if fallback_audit.STRICT:
                        raise
                    fallback_audit.record('vars-type-later')
                    _later_s = None
                _hoisted_s = render_type(hoisted_type)
                if (_later_s is not None and _later_s != _hoisted_s
                        and _forms_alignable(_later_s, _hoisted_s, registry)
                        and not _is_default_value(inner_item.value)):
                    _align_store_value(inner_item, hoisted_type, _later_s, _hoisted_s)
            _demote_let(entries, decl_k)
    (h._, h._hoisted_s, h._later, h._later_s, h.bk, h.block_indent, h.block_item,
     h.block_k, h.decl_k, h.default_val, h.first_let, h.hoisted_type, h.inner_indent,
     h.inner_item, h.k2, h.merged_type, h.parent_k, h.ref_nesting, h.span_end
    ) = (_, _hoisted_s, _later, _later_s, bk, block_indent, block_item, block_k, decl_k,
         default_val, first_let, hoisted_type, inner_indent, inner_item, k2, merged_type,
         parent_k, ref_nesting, span_end
    )
    return False




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
            delta = _entry_delta(item)
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
                # 变量在当前词法作用域不可见 → 提升为 LetStmt（类型由 Rust 推断）；
                # 继承原赋值的 JVM 身份标注（槽位 / store 偏移）
                entries[k] = (indent, LetStmt(name, None, True, item.value,
                                              slot=item.slot, bind_off=item.bind_off))
                declared[name] = nesting

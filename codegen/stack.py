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
    RsGeneric, RsPrimitive, RsNamed, RsRef, RsSlice, RsInfer,
    I32 as _I32, I64 as _I64, F32 as _F32, F64 as _F64,
)
from .render import render_type, render_expr
from .type_map import short_cls as _short_cls
from .constants import safe_ident, PRIMITIVE_RUST_TYPES as _SCALAR_TYPE_NAMES


def _safe_name(name: str) -> str:
    """局部变量名安全化，在 safe_ident 基础上额外处理：
    以大写字母开头的名（如 IOException、Exception 用作 catch 变量）camelCase 化，
    避免遮蔽 Rust unit struct（E0530）。
    全大写常量风格名（如 ARG_BASE）原样保留。"""
    s = safe_ident(name)
    if not s or not s[0].isupper():
        return s
    # 全大写常量（ALL_CAPS）原样返回
    if s == s.upper() and any(c.isalpha() for c in s):
        return s
    # 找到开头的连续大写字母段（如 IOException 中的 IOE）
    i = 0
    while i < len(s) and s[i].isupper():
        i += 1
    run, rest = s[:i], s[i:]
    # 多字母大写前缀（缩写词）：除最后一个大写字母外全部小写，最后一个保留作下一词首字母
    # 例：IOException → io + E + xception = ioException
    # 单字母大写前缀：直接小写
    if i > 1 and rest:
        return run[:-1].lower() + run[-1] + rest
    return run.lower() + rest

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


def _clone_moved_var(expr: RsExpr, ty: RsType) -> RsExpr:
    """局部变量值出现在 let/assign 右值位置时是 Rust move；
    Java 引用赋值（aload src; astore dst）无 move 语义，源变量后续仍会被使用，
    需包 Clone::clone 保活源值（否则 E0382 use-after-move）。
    - 标量（RsPrimitive/数字类名）与引用（RsRef/RsSlice）是 Copy，无需处理
    - Object 智能指针 Clone 即 Java 别名语义
    - RsInfer 未知类型（可能是迭代器等非 Clone 值）不处理
    注意：用 Clone::clone(&x) 而非 x.clone()，避免被类的 Java clone() 方法遮蔽。
    """
    if not isinstance(expr, Var):
        return expr
    if isinstance(ty, (RsPrimitive, RsRef, RsSlice, RsInfer)):
        return expr
    if isinstance(ty, RsNamed) and ty.name in (
        'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64',
        'f32', 'f64', 'bool', 'char', '()', 'usize',
    ):
        return expr
    return RawExpr(f"Clone::clone(&{expr.name})")




class StackSim:
    def __init__(self, param_rust_types: list[RsType], is_static: bool, class_name: str,
                 local_names: dict[int, str] | None = None,
                 slot_hint_types: dict[int, RsType] | None = None,
                 slot_hint_starts: dict[int, int] | None = None,
                 slot_var_types: dict[int, list] | None = None,
                 return_type: str = 'Object',
                 is_constructor: bool = False,
                 class_type_params: list[str] | None = None,
                 in_vtable_body: bool = False):
        self.stack:      list[tuple[RsExpr, RsType]] = []
        self._ctr:       int                         = 0
        self.locals:     dict[int, tuple]            = {}   # slot → (name, RsType, is_new)
        self.stmts:      list[RsStmt]                = []
        self.is_static   = is_static
        self.class_name  = class_name
        self.return_type = return_type
        self.is_constructor = is_constructor
        self.class_type_params: frozenset[str] = frozenset(class_type_params or [])
        self._loc_names  = local_names or {}     # slot → Java variable name
        self._hint_types = slot_hint_types or {}  # slot → precise RsType from LocalVariableTypeTable
        self._hint_starts = slot_hint_starts or {}  # slot → LVTT start_pc（slot 复用检测）
        # slot → [(start_pc, end_pc, RsType)]：LVT 逐变量声明类型与作用域
        self._var_types = slot_var_types or {}
        self.current_offset: int                     = 0    # 当前正在处理的字节码偏移
        self._current_depth: int                     = 0
        self._slot_decl_depth: dict[int, int]        = {}  # slot → 首次声明时的嵌套深度
        self.underflow_occurred: bool                = False  # 记录是否发生过栈下溢
        self.in_vtable_body: bool                    = in_vtable_body

        def _is_wide(rt: RsType) -> bool:
            """long (i64) 和 double (f64) 在 JVM 中各占 2 个局部变量槽。"""
            return isinstance(rt, RsPrimitive) and rt.name in ('i64', 'f64')

        if is_static:
            slot = 0
            for rt in param_rust_types:
                name = _safe_name(self._loc_names.get(slot, f"arg_{slot}"))
                self.locals[slot] = (name, rt, False)
                self._slot_decl_depth[slot] = 0
                slot += 2 if _is_wide(rt) else 1
        else:
            # this 是当前类的句柄，用 short_cls 转换 JVM 二进制名到 Rust 短名。
            # 泛型类（impl<K,V> HashMap_TreeNode<K,V>）的 this 必须带上类型参数：
            # 裸名会让 `x = this` 之类的赋值以裸类型记录，与 LVTT 精确形态
            # （HashMap_TreeNode<K, V>）不一致，触发 T42 重声明 + 循环提升后
            # 产生 E0107（缺泛型实参）/E0308。
            rust_cls = _short_cls(class_name) if class_name else 'Object'
            if rust_cls and class_type_params:
                rust_cls = f"{rust_cls}<{', '.join(class_type_params)}>"
            this_ty = RsNamed(rust_cls) if rust_cls else RsNamed("Object")
            self.locals[0] = ("this", this_ty, False)
            self._slot_decl_depth[0] = 0
            slot = 1
            for idx, rt in enumerate(param_rust_types):
                name = _safe_name(self._loc_names.get(slot, f"arg_{idx}"))
                self.locals[slot] = (name, rt, False)
                self._slot_decl_depth[slot] = 0
                slot += 2 if _is_wide(rt) else 1

    # ── 作用域深度追踪（T68：JVM slot 复用检测）────────────────────────────

    def enter_scope(self):
        """进入嵌套作用域（循环体 / if 体等）。"""
        self._current_depth += 1

    def exit_scope(self):
        """退出嵌套作用域。"""
        if self._current_depth > 0:
            self._current_depth -= 1

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
        # 标记下溢，gen_method_body 会把整个方法退化为 panic!("stub: ...")
        self.underflow_occurred = True
        return (RawExpr('(panic!("stack underflow") as i32)'), I32)

    # ── 语句输出 ─────────────────────────────────────────────────────────────

    def emit(self, stmt: RsStmt):
        """追加语句。stmt 必须是 RsStmt 节点。"""
        assert isinstance(stmt, _STMT_CLASSES), \
            f"emit() requires RsStmt, got {type(stmt)}: {stmt!r}"
        self.stmts.append(stmt)

    # ── 局部变量 ─────────────────────────────────────────────────────────────

    def _declared_var(self, slot: int) -> tuple | None:
        """当前偏移处 slot 上存活变量的 LVT 条目 (start, end, RsType, name)；无 LVT 信息返回 None。
        LVT start_pc 指向首个 store 之后的指令，store 自身偏移 = start_pc - 指令大小（≤4）：
        即将开始的变量（store 正在初始化它）优先于仍未结束的旧变量。"""
        off = self.current_offset
        entries = self._var_types.get(slot, ())
        for entry in entries:
            if entry[0] - 4 <= off < entry[0]:
                return entry
        for entry in entries:
            if entry[0] <= off < entry[1]:
                return entry
        return None

    def _declared_var_type(self, slot: int) -> RsType | None:
        entry = self._declared_var(slot)
        return entry[2] if entry is not None else None

    def _local_name(self, slot: int) -> str:
        """slot 在当前偏移处的变量名：优先 LVT 作用域内的名字，回退槽级代表名。"""
        entry = self._declared_var(slot)
        if entry is not None:
            return _safe_name(entry[3])
        return _safe_name(self._loc_names.get(slot, f"local_{slot}"))

    def _in_object_var_range(self, slot: int) -> bool:
        """当前偏移是否处于 slot 上某个「LVT 声明类型为 Object/接口」变量的作用域内。"""
        decl_ty = self._declared_var_type(slot)
        return decl_ty is not None and render_type(decl_ty) == 'Object'

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
        # 用 LocalVariableTypeTable 提供的精确类型覆盖泛型擦除后的 Object 或裸类名
        # 仅当当前字节码偏移 >= hint 的 start_pc 时才应用（防止 slot 复用导致
        # 前一变量生命周期内错误套用后继变量的类型，如 for-each 迭代器 slot）
        hint = self._hint_types.get(slot)
        if hint is not None:
            hint_start = self._hint_starts.get(slot, 0)
            # LVTT start_pc 指变量可读的首指令偏移，store 指令本身偏移 = start_pc - 指令大小
            # （astore 最大 4 字节）。若 store 偏移远早于 start_pc（差值 > 4），
            # 说明该 slot 在 hint 变量生命周期之前被复用（如 for-each 迭代器），不应用 hint。
            if hint_start > 0 and self.current_offset < hint_start - 4:
                hint = None
        # 记录原始栈类型：只有在栈类型为 Object 时才需要 downcast
        src_is_object = isinstance(ty, RsNamed) and ty.name == 'Object'
        if hint is not None:
            if src_is_object:
                ty = hint
            elif (isinstance(ty, RsNamed) and isinstance(hint, (RsNamed, RsGeneric))):
                # 裸类名（HashMap_TreeNode）或擦除实例化（HashMap_TreeNode<Object, Object>）
                # → hint 带泛型（HashMap_TreeNode<K, V>）。
                # LVTT hint 的泛型实参嵌在 name 字符串里（如 'HashMap_TreeNode<K, V>'），
                # 比较必须按 base 名（split('<')[0]）进行，否则三种形态永远不相等。
                def _base_of(t: RsType) -> str:
                    return getattr(t, 'name', str(t)).split('<')[0].strip()
                if _base_of(hint) == _base_of(ty):
                    ty = hint
                elif (isinstance(hint, RsNamed)
                      and 'Vec<' in hint.name and 'Vec<' in ty.name
                      and hint.name != ty.name
                      and isinstance(expr, RawExpr) and '.downcast::<' in expr.code):
                    # Vec<Object> (checkcast 擦除) ↔ Vec<E> (LVT 类型参数)：
                    # 替换 downcast 目标为 hint 类型，使字段赋值类型一致
                    dc_pos = expr.code.index('.downcast::<')
                    inner = expr.code[:dc_pos]
                    expr = RawExpr(f"{inner}.downcast::<{hint.name}>()")
                    ty = hint
        # null（aconst_null）赋给非 Object 提示类型时：Object::default() → Default::default()
        # 让显式类型注解决定具体类型，避免类型不匹配
        if src_is_object and hint is not None and isinstance(expr, Lit) and expr.value == 'Object::default()':
            expr = RawExpr('Default::default()')
            src_is_object = False
        # `this` 在 Rust 方法中是 &Self，赋值给类型标注变量时需 clone()
        _is_this = isinstance(expr, Var) and expr.name == 'this'
        _is_ref_ty = isinstance(ty, RsNamed) and ty.name not in (
            'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64',
            'f32', 'f64', 'bool', 'char', '()', 'Object',
        )
        if _is_this and _is_ref_ty:
            # synchronized(this) 模式：dup; astore N; monitorenter
            # 特征：astore 之后栈顶仍有一个 this（dup 留下的，供 monitorenter 消费）。
            # monitor 追踪变量只被 monitorexit（no-op）使用，生成 () 而非 Clone::clone(this)，
            # 避免 vtable default impl 中 Self: Clone + Sized 约束失败（E0277）。
            _top_is_this = (
                len(self.stack) > 0
                and isinstance(self.stack[-1][0], Var)
                and self.stack[-1][0].name == 'this'
            )
            if _top_is_this:
                expr = RawExpr("()")
                ty = RsNamed("()")
            else:
                # Clone::clone 而非 this.clone()：类的 Java clone() 方法会遮蔽 std Clone
                expr = RawExpr("Clone::clone(this)")

        # LVT 声明类型为 Object/接口 的变量（如 `Iface prev = null; ... prev = impl;`）：
        # 已以 Object 声明的槽在同一变量作用域内再次写入具体类值时，是 Java 的隐式
        # 向上转型，不是 slot 复用 → 值装箱为 Object 后赋值，禁止按值类型 let 阴影
        # （阴影会让后续按 Object 记录生成的 dispatch 代码作用在具体 wrapper 上，E0609）。
        if (slot in self.locals and not src_is_object
                and render_type(self.locals[slot][1]) == 'Object'
                and isinstance(ty, (RsNamed, RsGeneric))
                and not (isinstance(ty, RsNamed) and ty.name in _SCALAR_TYPE_NAMES)
                and self._in_object_var_range(slot)):
            expr = RawExpr(f"Object::from_any({render_expr(_clone_moved_var(expr, ty))})")
            ty = RsNamed('Object')

        # slot 复用：当前偏移处 LVT 变量名与槽上已登记的名字不同 → 这是一个新变量，
        # 走首次声明路径（用新名字 let），而不是给旧变量赋值 / 同名阴影。
        if slot in self.locals \
                and self._declared_var(slot) is not None \
                and self._local_name(slot) != self.locals[slot][0]:
            del self.locals[slot]

        if slot in self.locals:
            name, old_ty, _ = self.locals[slot]
            decl_depth = self._slot_decl_depth.get(slot, 0)
            # T42: slot 类型发生变化时（如 for-each 迭代器 slot 被后续变量复用），
            # 用 let 阴影（shadowing）而非赋值，避免 Rust 类型不匹配
            # T68: slot 在内层作用域（depth > current）中首次声明时，在外层访问必须用 let
            if render_type(old_ty) != render_type(ty) or decl_depth > self._current_depth:
                self.locals[slot] = (name, ty, True)
                self._slot_decl_depth[slot] = self._current_depth
                value = _maybe_downcast(expr, ty) if src_is_object else expr
                _is_default = isinstance(value, RawExpr) and value.code == 'Default::default()'
                let_ty = ty if _is_default else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
                # Java 局部变量间赋值在 Rust 中是 move，包 Clone 保活源变量（E0382）
                value = _clone_moved_var(value, ty)
                self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value))
            else:
                # hint 升级后（栈类型 Object → 局部精确类型）赋给已声明局部：
                # 值本身仍是 Object（如 from_any 包装的调用结果），
                # 需要 downcast 对齐局部精确类型（与首次声明路径的 _maybe_downcast 一致），
                # 否则 `kc = _t0`（kc: Class<Object>，_t0: Object）E0308。
                # 仅处理 Var：RawExpr 可能已含 downcast 或本就不是 Object 值。
                if (src_is_object and isinstance(expr, Var) and isinstance(ty, RsNamed)
                        and '<' in ty.name):
                    expr = RawExpr(f"({render_expr(expr)}).downcast::<{ty.name}>()")
                # Java 引用赋值无 move 语义，包 Clone 保活源变量（E0382）
                self.stmts.append(AssignStmt(Var(name), _clone_moved_var(expr, ty)))
        else:
            name = self._local_name(slot)
            self.locals[slot] = (name, ty, True)
            self._slot_decl_depth[slot] = self._current_depth
            value = _maybe_downcast(expr, ty) if src_is_object else expr
            # downcast 时让 Rust 推断类型；Default::default() 需保留类型注解；RsGeneric 也让 Rust 推断
            _is_default = isinstance(value, RawExpr) and value.code == 'Default::default()'
            let_ty = ty if _is_default else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
            # Java 局部变量间赋值（aload src; astore dst）在 Rust 中是 move，
            # 源变量后续仍会被使用，包 Clone::clone 保活（E0382 use-after-move）
            value = _clone_moved_var(value, ty)
            self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value))

        # dup 后 astore：同一个 Var("_tN") 可能还留在 stack 上，但 _tN 已被 move。
        # 把 stack 上残留的同名引用替换为目标变量名，防止 E0382 use-after-move。
        if isinstance(expr, Var) and expr.name != name:
            self.stack = [
                (Var(name), sty) if isinstance(se, Var) and se.name == expr.name else (se, sty)
                for se, sty in self.stack
            ]

    def load_local(self, slot: int) -> tuple[RsExpr, RsType]:
        """从局部变量槽加载，返回 (RsExpr, RsType)。"""
        if slot in self.locals:
            name, ty, _ = self.locals[slot]
            return (Var(name), ty)
        # 槽不在 locals 中（跨 StackSim 路径），仍用 LocalVariableTable 中的名字，
        # 以便 _hoist_if_vars 能将其与同名的 LetStmt 声明关联并正确提升。
        # 类型以当前偏移处的 LVT 声明类型兜底（按作用域区分槽复用）；无 LVT 才退回 i32。
        name = self._local_name(slot)
        decl_ty = self._declared_var_type(slot)
        return (Var(name), decl_ty if decl_ty is not None else I32)

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

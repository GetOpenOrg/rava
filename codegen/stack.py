"""
JVM 操作数栈模拟器：将基于栈的字节码转换为 Rust IR 节点（SSA 前驱）。

栈中存储 (RsExpr, RsType) 元组，语句列表存储 RsStmt 节点。
全量使用 IR 节点，不再接受字符串参数。
"""

from typing import get_args

from .rs_ir import (
    RsExpr, RsStmt, RsType,
    Var, Lit, RawExpr, NewPendingExpr,
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
    # 目标为任意非 Object 引用类型（含裸类型参数 V/K 与非泛型类）都需要 downcast：
    # 源值的静态类型是 Object，直接 `let v: V = _t0` 必然 E0308。
    if (isinstance(expr, Var) and isinstance(ty, RsNamed)
            and ty.name != 'Object' and not ty.name.startswith('Rc<')
            and not ty.name.startswith('&') and ty.name != '()'):
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




_TRIVIAL_RAW_RE = None


def _is_trivial_expr(expr: RsExpr) -> bool:
    """重复求值无副作用且无开销的表达式：变量、字面量、待定 new、默认值、对变量的 Clone。"""
    global _TRIVIAL_RAW_RE
    if isinstance(expr, (Var, Lit, NewPendingExpr)):
        return True
    if isinstance(expr, RawExpr):
        if _TRIVIAL_RAW_RE is None:
            import re
            _TRIVIAL_RAW_RE = re.compile(
                r'^(?:[A-Za-z_]\w*|Clone::clone\(&?[A-Za-z_]\w*\)|Default::default\(\)'
                r'|[A-Za-z_]\w*::default\(\)|-?\d[\w.]*)$')
        return bool(_TRIVIAL_RAW_RE.match(expr.code.strip()))
    return False


def _materialized_let_type(expr: RsExpr, ty: RsType):
    """物化临时变量的类型注解：仅当表达式自身不足以让 Rust 推断类型时才标注。"""
    code = render_expr(expr)
    if code.endswith('.into()') or 'Default::default()' in code:
        return ty
    return None


class StackSim:
    def __init__(self, param_rust_types: list[RsType], is_static: bool, class_name: str,
                 local_names: dict[int, str] | None = None,
                 slot_decls: dict[int, list] | None = None,
                 is_subtype=None,
                 return_type: str = 'Object',
                 is_constructor: bool = False,
                 class_type_params: list[str] | None = None,
                 in_vtable_body: bool = False,
                 box_object=None):
        self.stack:      list[tuple[RsExpr, RsType]] = []
        # (已取得所有权的表达式, Rust 类型) → Object 引用表达式（保持对象身份的向上转型）
        self._box_object = box_object or (lambda e, _t: f"Object::from_any({e})")
        self._ctr:       int                         = 0
        self.locals:     dict[int, tuple]            = {}   # slot → (name, RsType, is_new)
        self.stmts:      list[RsStmt]                = []
        self.is_static   = is_static
        self.class_name  = class_name
        self.return_type = return_type
        self.is_constructor = is_constructor
        self.class_type_params: frozenset[str] = frozenset(class_type_params or [])
        self._loc_names  = local_names or {}     # slot → Java variable name
        # slot → [(start_pc, end_pc, name, RsType|None, from_signature)]：
        # LocalVariableTable/TypeTable 按作用域区间给出的声明名与声明类型（slot 复用按偏移区分）
        self._slot_decls = slot_decls or {}
        self._is_subtype = is_subtype or (lambda _c, _p: False)
        self._param_slots: set[int]                  = set()  # 方法参数占用的 slot（类型由签名决定）
        self.current_offset: int                     = 0    # 当前正在处理的字节码偏移
        self.next_offset: int                        = 0    # 下一条指令偏移（store 后变量作用域起点）
        self._current_depth: int                     = 0
        self._slot_decl_depth: dict[int, int]        = {}  # slot → 首次声明时的嵌套深度
        self.underflow_occurred: bool                = False  # 记录是否发生过栈下溢
        self.in_vtable_body: bool                    = in_vtable_body
        # 方法体用到的类型变量上界转换：类型变量 → 上界 Rust 类型。
        # 方法签名据此声明 `where E: Into<Bound>`；子 sim 与根 sim 共享同一 dict。
        self.type_var_bound_uses: dict[str, str]     = {}
        # 类级类型变量 → 上界 Rust 类型（`K extends Task<.., K>`），由方法生成入口填入
        self.type_var_bounds: dict[str, str]         = {}

        def _is_wide(rt: RsType) -> bool:
            """long (i64) 和 double (f64) 在 JVM 中各占 2 个局部变量槽。"""
            return isinstance(rt, RsPrimitive) and rt.name in ('i64', 'f64')

        if is_static:
            slot = 0
            for rt in param_rust_types:
                name = _safe_name(self._loc_names.get(slot, f"arg_{slot}"))
                self.locals[slot] = (name, rt, False)
                self._slot_decl_depth[slot] = 0
                self._param_slots.add(slot)
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
                self._param_slots.add(slot)
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
        """弹栈，返回 (RsExpr, RsType)。

        被 dup 的表达式在栈上以同一对象出现多次。Java 语义下它只求值一次，
        因此消费其中一份时，若其余副本仍在栈上，先物化为临时变量，所有副本改为引用该变量
        （`++size > threshold` 只自增一次；`dup` 的 `X::new(..)?` 只构造一次）。
        """
        if self.stack:
            expr, ty = self.stack.pop()
            if not _is_trivial_expr(expr) and any(se is expr for se, _ in self.stack):
                name = self.fresh()
                self.stmts.append(LetStmt(name, _materialized_let_type(expr, ty), mutable=False, value=expr))
                var = Var(name)
                self.stack = [(var, sty) if se is expr else (se, sty) for se, sty in self.stack]
                return (var, ty)
            return (expr, ty)
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

    def _decl_at(self, slot: int, for_store: bool):
        """按当前字节码偏移查 slot 的声明条目 (name, RsType|None, from_signature)。
        - 作用域覆盖当前偏移的条目优先（对已存活变量的读/再赋值）
        - store 时：变量作用域从 store 的下一条指令开始，按 next_offset 精确匹配初始化 store
        未命中返回 None（编译器合成的临时 slot，如 for-each 迭代器、synchronized 锁对象）。"""
        entries = self._slot_decls.get(slot)
        if not entries:
            return None
        off = self.current_offset
        for start, end, name, rty, from_sig in entries:
            if start <= off < end:
                return (name, rty, from_sig)
        if for_store:
            nxt = self.next_offset
            for start, end, name, rty, from_sig in entries:
                if start == nxt or (nxt <= off and off < start <= off + 4):
                    return (name, rty, from_sig)
        return None

    def pop_for_store(self) -> tuple[RsExpr, RsType]:
        """供 xstore 使用的弹栈：不物化 dup 副本——store_local 会把局部变量本身作为物化结果，
        栈上剩余副本改为引用该局部变量（`dup; astore` → `let x = X::new(..)?;` 后栈上是 x）。"""
        if self.stack:
            return self.stack.pop()
        return self.pop()

    def store_local(self, slot: int, expr: RsExpr, ty: RsType):
        """存储到局部变量槽；栈上与被存值相同的 dup 副本改为引用该局部变量。"""
        source = expr
        self._store_local(slot, expr, ty)
        if slot in self.locals and not _is_trivial_expr(source):
            name, local_ty, _ = self.locals[slot]
            self.stack = [(Var(name), local_ty) if se is source else (se, sty)
                          for se, sty in self.stack]

    def _store_local(self, slot: int, expr: RsExpr, ty: RsType):
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
        # 声明表按作用域区间查询：slot 复用时（如 for-each 迭代器 slot 被后续变量复用）
        # 每个偏移只会命中当时真正存活的那个 Java 变量，不会错误套用其他变量的名字/类型。
        decl = None if slot in self._param_slots else self._decl_at(slot, for_store=True)
        decl_name = _safe_name(decl[0]) if decl is not None else None
        hint = decl[1] if (decl is not None and decl[2]) else None
        decl_ty = decl[1] if (decl is not None and not decl[2]) else None
        force_let_ty = False
        _INT_FAMILY = ('i32', 'bool', 'u16', 'i8', 'i16')
        ty_name = getattr(ty, 'name', '')
        if ty_name in _INT_FAMILY:
            # JVM 操作数栈上 boolean/byte/char/short 都是 int；局部变量的 Rust 类型以
            # LocalVariableTable 声明为准（int c = s.charAt(i) → c: i32），无声明时按 int。
            target = getattr(decl_ty, 'name', '') if decl_ty is not None else ''
            if slot in self._param_slots and slot in self.locals:
                # 形参的 Rust 类型由方法签名决定（boolean 形参 → bool），回写时对齐
                target = getattr(self.locals[slot][1], 'name', '')
            if target not in _INT_FAMILY:
                target = 'i32' if ty_name == 'bool' else ty_name
            if target != ty_name:
                src = render_expr(expr)
                if target == 'i32':
                    expr = RawExpr(f"({src}) as i32")
                elif target == 'bool':
                    expr = RawExpr(f"(({src}) as i32 != 0i32)")
                else:
                    expr = RawExpr(f"((({src}) as i32) as {target})")
                ty = RsPrimitive(target)
                force_let_ty = True
        elif (decl_ty is not None and isinstance(ty, RsNamed) and isinstance(decl_ty, RsNamed)
              and decl_ty.name != ty.name):
            _decl_base = decl_ty.name.split('<')[0]
            _src_base = ty.name.split('<')[0]
            _is_null = isinstance(expr, Lit) and expr.value == 'Object::default()'
            if ty.name == 'Object':
                # 声明为具体类但栈类型退化为 Object（类型推断缺口）：对齐到声明类型
                if _is_null:
                    expr = RawExpr('Default::default()')
                elif '<' not in decl_ty.name:
                    expr = RawExpr(f"({render_expr(expr)}).downcast::<{decl_ty.name}>()")
                    force_let_ty = True
                hint = decl_ty
            elif _decl_base != _src_base and self._is_subtype(_src_base, _decl_base):
                # 声明为父类、赋入子类值（Node tail = new Pos(head)）：From 上转换保留运行时类型
                if isinstance(expr, Var):
                    expr = RawExpr(f"Clone::clone(&{expr.name}).into()")
                else:
                    expr = RawExpr(f"({render_expr(expr)}).into()")
                ty = decl_ty
                force_let_ty = True
        # 类型变量值赋给声明为其上界类型的局部（`Task<.., K> task = this; task = task.makeChild(..)`，
        # makeChild 返回 K）：Java 隐式上转 → 转换为上界类型（约束 `K: Into<Bound>` 由方法签名声明）
        _tv_bound = self.type_var_bounds.get(ty.name) if isinstance(ty, RsNamed) else None
        _tv_target = hint if hint is not None else decl_ty
        if (_tv_bound is not None and isinstance(_tv_target, (RsNamed, RsGeneric))
                and getattr(_tv_target, 'name', '').split('<')[0].strip() == _tv_bound.split('<')[0].strip()):
            self.type_var_bound_uses[ty.name] = _tv_bound
            expr = RawExpr(f"Into::<{_tv_bound}>::into({render_expr(_clone_moved_var(expr, ty))})")
            ty = RsNamed(_tv_bound)
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
                    # 同一泛型类的不同实例化（通配符 static 字段 X<?> 经 unchecked cast
                    # 赋给 X<T> 局部）：Rust 侧是不同类型，经 Object 边界重新实例化
                    from .instr.coerce import _reinstantiate_generic
                    _src_name = getattr(ty, 'name', '')
                    _hint_name = getattr(hint, 'name', '')
                    if not (isinstance(expr, Lit) and expr.value == 'Object::default()'):
                        _src_code = render_expr(expr)
                        _dc_tail = f".downcast::<{_src_name}>()"
                        _conv = _reinstantiate_generic(_src_code, _src_name, _hint_name)
                        if _conv is not None and _src_code.endswith(_dc_tail):
                            # checkcast 刚产生的擦除形态 downcast：直接改写目标类型，不叠加二次转换
                            _conv = f"{_src_code[:-len(_dc_tail)]}.downcast::<{_hint_name}>()"
                        if _conv is not None:
                            expr = RawExpr(_conv)
                            force_let_ty = True
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

        if (slot in self.locals and not src_is_object and slot not in self._param_slots
                and decl is not None and decl[1] is None and self.locals[slot][0] == decl_name
                and getattr(self.locals[slot][1], 'name', '') == 'Object'
                and isinstance(ty, (RsNamed, RsGeneric))
                and getattr(ty, 'name', '') not in _SCALAR_TYPE_NAMES):
            # 声明为 Object/接口的变量在同一作用域内再赋入具体类值：Java 隐式上转 → 装箱后赋值，
            # 不按值类型 let 阴影（阴影会让按 Object 生成的 dispatch 作用在具体 wrapper 上）
            expr = RawExpr(self._box_object(render_expr(_clone_moved_var(expr, ty)), render_type(ty)))
            ty = RsNamed('Object')
        if slot in self.locals and decl_name is not None and self.locals[slot][0] != decl_name:
            # slot 被另一个 Java 变量复用：按新变量的声明名重新 let 声明
            del self.locals[slot]
        if (slot in self._param_slots and slot in self.locals
                and isinstance(expr, Lit) and expr.value == 'Object::default()'
                and isinstance(self.locals[slot][1], RsNamed)
                and self.locals[slot][1].name not in ('Object', '()')):
            # 形参（具体引用类型）被置 null（unscaledVal = null）：保持形参类型，赋默认值；
            # let 阴影会把类型降级成 Object 且只在当前块内可见
            self.stmts.append(AssignStmt(Var(self.locals[slot][0]), RawExpr('Default::default()')))
            return
        if (slot in self._param_slots and slot in self.locals
                and getattr(self.locals[slot][1], 'name', '') == 'Object'
                and _is_ref_ty and not isinstance(ty, (RsPrimitive, RsRef, RsSlice, RsInfer))):
            # 形参声明为 Object（接口/Object 多态边界），方法体内重新赋入具体类值
            # （if (s == null) s = "null"）：装箱后赋回原形参，
            # 不能用 let 阴影（阴影只在当前块内可见，块外仍读到旧值且类型不一致）。
            _src = render_expr(_clone_moved_var(expr, ty))
            self.stmts.append(AssignStmt(Var(self.locals[slot][0]),
                                         RawExpr(self._box_object(_src, render_type(ty)))))
            return
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
                let_ty = ty if (_is_default or force_let_ty) else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
                # Java 局部变量间赋值在 Rust 中是 move，包 Clone 保活源变量（E0382）
                value = _clone_moved_var(value, ty)
                self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value))
            else:
                # hint 升级后（栈类型 Object → 局部精确类型）赋给已声明局部：
                # 值本身仍是 Object（如 from_any 包装的调用结果），
                # 需要 downcast 对齐局部精确类型（与首次声明路径的 _maybe_downcast 一致），
                # 否则 `kc = _t0`（kc: Class<Object>，_t0: Object）E0308。
                # 仅处理 Var：RawExpr 可能已含 downcast 或本就不是 Object 值。
                if src_is_object:
                    expr = _maybe_downcast(expr, ty)
                # Java 引用赋值无 move 语义，包 Clone 保活源变量（E0382）
                self.stmts.append(AssignStmt(Var(name), _clone_moved_var(expr, ty)))
        else:
            name = decl_name or _safe_name(self._loc_names.get(slot, f"local_{slot}"))
            self.locals[slot] = (name, ty, True)
            self._slot_decl_depth[slot] = self._current_depth
            value = _maybe_downcast(expr, ty) if src_is_object else expr
            # downcast 时让 Rust 推断类型；Default::default() 需保留类型注解；RsGeneric 也让 Rust 推断
            _is_default = isinstance(value, RawExpr) and value.code == 'Default::default()'
            let_ty = ty if (_is_default or force_let_ty) else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
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
        decl = self._decl_at(slot, for_store=False)
        if decl is not None:
            # 声明表中类型为 None 的条目是 Object/接口声明的引用变量（基本类型总有具体类型）
            return (Var(_safe_name(decl[0])), decl[1] if decl[1] is not None else RsNamed('Object'))
        name = _safe_name(self._loc_names.get(slot, f"local_{slot}"))
        return (Var(name), I32)

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

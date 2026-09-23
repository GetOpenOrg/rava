"""
JVM 操作数栈模拟器：将基于栈的字节码转换为 Rust IR 节点（SSA 前驱）。

栈中存储 (RsExpr, RsType) 元组，语句列表存储 RsStmt 节点。
全量使用 IR 节点，不再接受字符串参数。
"""

import re as _re_stack
from typing import get_args

from .rs_ir import (
    RsExpr, RsStmt, RsType,
    Var, Lit, RawExpr, NewPendingExpr, CastExpr,
    LetStmt, AssignStmt,
    RsGeneric, RsPrimitive, RsNamed, RsRef, RsSlice, RsInfer,
    I32 as _I32, I64 as _I64, F32 as _F32, F64 as _F64,
)
from .render import render_type, render_expr
from .type_map import short_cls as _short_cls, _registry_short_index
from .constants import safe_ident, PRIMITIVE_RUST_TYPES as _SCALAR_TYPE_NAMES
from .jvm_type import JvmType, ClassRef


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


# ── TypeIR 擦除查询入口（收敛路线图 L1-a 批次 2）────────────────────────────
#
# 栈侧类型事实以 Rust 类型串（RsType.name）承载；进入类型代数（codegen/jvm_type）
# 查询前先擦除到裸类身份。此前各消费点散布「split 取首段」类文本解剖（control/
# returns/fields/blocks/arrays 等），现统一经本组边界函数——字符串手术位点收敛于
# 此一处，类型问题（子类型 / is_interface / 数组形态）全部在类型对象上求解：
#   erased_base(rust_ty)          → 擦除基名（JvmType.erasure 的边界投影）
#   erased_class_of(rust_ty, reg) → 擦除 ClassRef（registry 域内补 is_interface）
#   is_jvm_array(rust_ty)         → 引用数组形态（JvmType Array 变体的发射形态）

def erased_base(rust_ty: str) -> str:
    """Rust 类型串的擦除基名（`ArrayList<T>` → `ArrayList`、`JArray<Entry<K, V>>`
    → `JArray`）：JvmType.erasure() 在字符串边界的投影——泛型实参不参与裸名判定。
    首标识符提取与 jvm_type.carrier_type_for_ident 同型；非标识符开头（`()`
    等擦除外形态）保持原串，下游 registry 反查同不可解析。"""
    m = _re_stack.match(r'\s*(\w+)', rust_ty)
    return m.group(1) if m else rust_ty


def erased_class_of(rust_ty: str, registry: 'dict | None' = None) -> 'ClassRef | None':
    """Rust 类型串 → 擦除 ClassRef：基名经 registry 短名索引解析为 binary 身份
    （JvmType.class_of 补全 is_interface）。域外短名 / registry 缺失 / 非标识符
    形态 → None（调用方回退既有擦除路径，与 _rust_type_to_binary 的空串约定
    同一语义）。"""
    base = erased_base(rust_ty)
    if not registry or not base:
        return None
    ci = _registry_short_index(registry).get(base)
    if ci is None:
        return None
    return JvmType.class_of(ci.name, registry)


def is_jvm_array(rust_ty: str) -> bool:
    """Rust 类型串是否为引用数组形态 `JArray<..>`（jvm_type Array 变体的发射
    形态探测）。Vec<..> 是 Rust 侧容器、非 JVM 数组概念，不在本查询域内。"""
    return _re_stack.match(r'JArray<', rust_ty) is not None


def _maybe_downcast(expr: RsExpr, ty: RsType) -> RsExpr:
    """当 expr 是 Var（新鲜临时变量）且目标类型含泛型参数时，
    JDK stub 因类型擦除实际返回 Object，需要按目标类型还原视图。
    对工厂方法 / @synthetic（expr 为 RawExpr）不添加转换。
    A-3：发射 CastExpr 节点（unchecked From<Object> 视图路径），消费方按节点分派。
    """
    # 目标为任意非 Object 引用类型（含裸类型参数 V/K 与非泛型类）都需要还原：
    # 源值的静态类型是 Object，直接 `let v: V = _t0` 必然 E0308。
    if (isinstance(expr, Var) and isinstance(ty, RsNamed)
            and ty.name != 'Object' and not ty.name.startswith('Rc<')
            and not ty.name.startswith('&') and ty.name != '()'):
        return CastExpr(expr, ty.name)
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
                 infer_type_args=None,
                 is_subtype=None,
                 is_interface=None,
                 return_type: str = 'Object',
                 is_constructor: bool = False,
                 class_type_params: list[str] | None = None,
                 in_vtable_body: bool = False,
                 box_object=None,
                 registry: 'dict | None' = None):
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
        self._infer_type_args = infer_type_args
        self._is_subtype = is_subtype or (lambda _c, _p: False)
        # 短类名是否为接口：hint 拓宽（声明为父类、赋入子类值）只对类祖先做 From 上转，
        # 接口不在宏的 all_superclasses 链上（无 From<Child> for Iface），需跳过
        self._is_interface = is_interface or (lambda _s: False)
        self._param_slots: set[int]                  = set()  # 方法参数占用的 slot（类型由签名决定）
        self.current_offset: int                     = 0    # 当前正在处理的字节码偏移
        self.next_offset: int                        = 0    # 下一条指令偏移（store 后变量作用域起点）
        self._current_depth: int                     = 0
        self._slot_decl_depth: dict[int, int]        = {}  # slot → 首次声明时的嵌套深度
        # slot → 当前绑定创建时的字节码偏移：区分「同一 Java 变量的再赋值」与
        # 「javac 槽位复用（另一变量的首次赋值）」——同名不同声明区间的两个变量
        # 共用槽位时（catch 变量 e 在多个 handler 间复用），后者必须 let 阴影
        self._slot_bind_pos: dict[int, int]          = {}
        self.underflow_occurred: bool                = False  # 记录是否发生过栈下溢
        self.in_vtable_body: bool                    = in_vtable_body
        # 类型位置载体化判定（A-4 批次 3+）的 registry 视图（只读查询）
        self._registry = registry
        # 方法体用到的类型变量上界转换：类型变量 → 上界 Rust 类型。
        # 方法签名据此声明 `where E: Into<Bound>`；子 sim 与根 sim 共享同一 dict。
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
        for start, end, name, rty, from_sig, raw_sig in entries:
            if start <= off < end:
                return (name, rty, from_sig, raw_sig)
        if for_store:
            nxt = self.next_offset
            for start, end, name, rty, from_sig, raw_sig in entries:
                if start == nxt or (nxt <= off and off < start <= off + 4):
                    return (name, rty, from_sig, raw_sig)
        return None

    def _undeclared_slot_name(self, slot: int) -> str:
        """当前偏移没有声明条目覆盖的 slot 的变量名。
        slot 在别的偏移区间有声明（被 Java 变量复用）→ 此处是编译器合成的临时变量
        （for-each 迭代器、synchronized 锁对象），用 slot 编号命名；借用复用者的 Java 名字
        会让两个不同类型的变量在分支提升时合并成同一个声明。"""
        if self._slot_decls.get(slot) and slot not in self._param_slots:
            return f"local_{slot}"
        return _safe_name(self._loc_names.get(slot, f"local_{slot}"))

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

    def _freeze_stack_var_copies(self, name: str):
        """写入局部变量前，把栈上残留的同名 Var 快照到临时变量（xstore 版 iinc 快照规则）。

        JVM 字节码里加载恒先于存储：`f(value++)` 编译为 `lload; dup2; ladd; lstore;
        invokespecial` —— dup2 留在栈上的副本是递增前的旧值。Rust 侧变量名在赋值后
        指向新值，栈上副本若仍以名字渲染会读到新值 → 先冻结到 `_name_pre` 临时变量。
        Copy 类型直接复制，引用类型经 Clone::clone 保持别名语义。"""
        copies = [(j, sty) for j, (se, sty) in enumerate(self.stack)
                  if isinstance(se, Var) and se.name == name]
        if not copies:
            return
        snap_by_ty: dict[str, Var] = {}
        for j, sty in copies:
            key = render_type(sty)
            snap = snap_by_ty.get(key)
            if snap is None:
                snap = self.fresh_let(f'_{name}_pre', _clone_moved_var(Var(name), sty), sty)
                snap_by_ty[key] = snap
            self.stack[j] = (snap, sty)

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
        # 加载先于存储（dup2 副本等）：写入前冻结栈上同名 Var 的旧值快照
        if slot in self.locals:
            self._freeze_stack_var_copies(self.locals[slot][0])
        force_let_ty = False
        # 菱形构造结果（类型实参待推断的 `X<_>`）存入有泛型声明的局部：按声明签名解出类型实参。
        # 只靠 Rust 从后续用法推断时，若元素只以 Object 形态被使用，`_` 永远无解（E0283）。
        _infer_m = _re_stack.match(r'^(\w+)<(_(?:, _)*)>$', ty.name) if isinstance(ty, RsNamed) else None
        if _infer_m and decl is not None and decl[3] and self._infer_type_args is not None:
            _solved = self._infer_type_args(_infer_m.group(1), decl[3])
            if _solved:
                _solved_args = ', '.join(_solved)
                _open_tf = f"{_infer_m.group(1)}::<{_infer_m.group(2)}>::"
                _src = render_expr(expr)
                if _src.startswith(_open_tf):
                    expr = RawExpr(f"{_infer_m.group(1)}::<{_solved_args}>::" + _src[len(_open_tf):])
                else:
                    force_let_ty = True
                ty = RsNamed(f"{_infer_m.group(1)}<{_solved_args}>")
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
            _decl_base = erased_base(decl_ty.name)
            _src_base = erased_base(ty.name)
            _is_null = isinstance(expr, Lit) and expr.value == 'Object::default()'
            from .jvm_type import carrier_type_for_ident as _carrier_type_for_ident
            _carrier_decl = _carrier_type_for_ident(decl_ty.name, self._registry)
            if _carrier_decl is not None and _carrier_decl == decl_ty.name:
                # A-4 批次 3+：局部声明类型是已铺设的接口载体（LVTT/LVT 经
                # sig_parse/jvm_to_rust 翻转）。栈值 → 载体的边界转换：
                #   - 值已是同载体：Clone 保活（move 语义）；
                #   - 值是 Object / 具体实现类：经 Object 边界的非受检载体包装
                #     （From<_> 的 UFCS 不可被接口自带的静态 from 工厂遮蔽，
                #     与 _coerce_arg 的载体分支同源）。
                if ty.name != _carrier_decl:
                    _src = render_expr(_clone_moved_var(expr, ty))
                    if _is_null:
                        expr = RawExpr('Default::default()')
                    else:
                        expr = RawExpr(
                            f"<{_carrier_decl} as ::std::convert::From<_>>::from({_src})")
                    force_let_ty = True
                ty = decl_ty
            elif ty.name == 'Object':
                # 声明为具体类但栈类型退化为 Object（类型推断缺口）：对齐到声明类型
                if _is_null:
                    expr = RawExpr('Default::default()')
                elif '<' not in decl_ty.name:
                    expr = CastExpr(expr, decl_ty.name)
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
        if (isinstance(decl_ty, RsPrimitive) and isinstance(expr, Lit)
                and expr.value == 'Object::default()'):
            # 声明为包装类（按基本类型建模）的局部赋 null（`Long inNanos = null`）：
            # 变量类型以声明为准，不随 null 字面量退化成根类
            expr = RawExpr('Default::default()')
            ty = decl_ty
            force_let_ty = True
        # 类型变量值赋给声明为其上界类型的局部（`Task<.., K> task = this; task = task.makeChild(..)`，
        # makeChild 返回 K）：Java 隐式上转 → 经 Object 的 checkcast 视图转换为上界类型
        _tv_bound = self.type_var_bounds.get(ty.name) if isinstance(ty, RsNamed) else None
        _tv_target = hint if hint is not None else decl_ty
        if (_tv_bound is not None and isinstance(_tv_target, (RsNamed, RsGeneric))
                and erased_base(getattr(_tv_target, 'name', '')) == erased_base(_tv_bound)):
            expr = RawExpr(
                f"Into::<{_tv_bound}>::into(Into::<Object>::into({render_expr(_clone_moved_var(expr, ty))}))")
            ty = RsNamed(_tv_bound)
            hint = None
        # 记录原始栈类型：只有在栈类型为 Object 时才需要 downcast
        src_is_object = isinstance(ty, RsNamed) and ty.name == 'Object'
        if hint is not None:
            if src_is_object:
                if (getattr(hint, 'name', '') in self.class_type_params
                        and isinstance(expr, (RawExpr, CastExpr))
                        and render_expr(expr) != 'Default::default()'):
                    # `E v = (E) es[i]`：擦除后无 checkcast，Object 值直接存入声明为类型变量的
                    # 局部 → 经宏补的 From<Object> bound 按对象标识取回类型变量视图。
                    # CastExpr（A-4 批次 1 的接口 checkcast，`<D extends Iface> D v = (D) t`
                    # —— javac 按 D 的擦上界发 checkcast 接口）同样返回 Object，
                    # 判定已由 try_cast_iface 完成，这里只做 D 视图取回
                    expr = RawExpr(f"From::from({render_expr(expr)})")
                    force_let_ty = True
                elif (not isinstance(expr, Var)
                      and isinstance(hint, RsNamed) and is_jvm_array(hint.name)
                      and render_expr(expr) not in ('Default::default()', 'Object::default()')):
                    # 栈类型 Object、声明为数组类型（`for (int[] r : objArr)` 的元素经
                    # Object 流转，S-2.2）：值按数组目标还原视图并保留 let 注解。
                    # 非 Var 值（aaload 结果等 RawExpr）不落入 _maybe_downcast 的
                    # Var-only 分支，不在此还原会让记录类型（JArray<T>）与 let 实际
                    # 推断类型（Object）脱节，后续 Clone::clone 赋值 E0308。
                    # null 字面量（Default::default()，此处已是 RawExpr）穿透任何
                    # checkcast，不转换——unchecked From 在 null 接收者上无类型可依。
                    expr = CastExpr(expr, hint.name)
                    force_let_ty = True
                ty = hint
            elif (isinstance(ty, RsNamed) and getattr(hint, 'name', '') in self.class_type_params
                  and ty.name != hint.name and ty.name not in _SCALAR_TYPE_NAMES):
                # 声明类型是类型变量（`S s = (S) x`，checkcast 落在 S 的上界类）：
                # 值经 Object 边界按对象标识取回类型变量视图。
                # 源值若是 checkcast 的 CastExpr（A-3 节点），其待转值即装箱前的
                # Object 值——按节点分派取子表达式（box_first 形态的子值是具体
                # wrapper，仍走统一装箱路径）。
                if isinstance(expr, CastExpr) and expr.target == ty.name and not expr.box_first:
                    expr = RawExpr(f"From::from({render_expr(expr.expr)})")
                else:
                    _boxed = self._box_object(
                        render_expr(_clone_moved_var(expr, ty)), ty.name)
                    expr = RawExpr(f"From::from({_boxed})")
                ty = hint
                force_let_ty = True
            elif (isinstance(ty, RsNamed) and isinstance(hint, (RsNamed, RsGeneric))):
                # 裸类名（HashMap_TreeNode）或擦除实例化（HashMap_TreeNode<Object, Object>）
                # → hint 带泛型（HashMap_TreeNode<K, V>）。
                # LVTT hint 的泛型实参嵌在 name 字符串里（如 'HashMap_TreeNode<K, V>'），
                # 比较必须按擦除基名进行，否则三种形态永远不相等。
                def _base_of(t: RsType) -> str:
                    return erased_base(getattr(t, 'name', str(t)))
                if _base_of(hint) == _base_of(ty):
                    # 同一泛型类的不同实例化（通配符 static 字段 X<?> 经 unchecked cast
                    # 赋给 X<T> 局部）：Rust 侧是不同类型，经 Object 边界构造目标实例化
                    # 的视图（A-1 存储层擦除后 From<Object> for X<A> 对任意 A 成立，
                    # 共享同一存储与对象标识）。注意不能改写成 `.downcast::<目标>()`：
                    # Object::downcast 按精确 TypeId 判定，跨实例化会误抛 ClassCastException。
                    # A-3：发射 CastExpr 节点（unchecked 擦除路径），替代已删除的
                    # `_reinstantiate_generic` 字符串发射。
                    from .instr.coerce import _same_generic_family
                    _src_name = getattr(ty, 'name', '')
                    _hint_name = getattr(hint, 'name', '')
                    if (not (isinstance(expr, Lit) and expr.value == 'Object::default()')
                            and _same_generic_family(_src_name, _hint_name)):
                        expr = CastExpr(expr, _hint_name, box_first=True)
                        force_let_ty = True
                    ty = hint
                elif (not self._is_interface(_base_of(hint))
                      and _base_of(hint) not in ('Object', '()')
                      and self._is_subtype(_base_of(ty), _base_of(hint))):
                    # 声明类型（LVTT/LVT 签名）是值类型的父类（`Node e = putTreeVal(..)`，
                    # Java 隐式拓宽赋值，字节码无 checkcast）：From 上转到声明类型，
                    # 槽位静态类型与 JVM 局部变量的声明类型一致 —— 同一 slot 在兄弟分支
                    # 赋不同子类型时都汇合到声明类型（JVM 校验器在控制流合并点的行为）。
                    # 与 decl_ty（描述符声明的父类）路径同一规则；接口声明不做 From 上转
                    # （宏只为类祖先生成 From<Child>，见 _is_interface 钩子说明）。
                    if isinstance(expr, Var):
                        expr = RawExpr(f"Clone::clone(&{expr.name}).into()")
                    else:
                        expr = RawExpr(f"({render_expr(expr)}).into()")
                    ty = hint
                    force_let_ty = True
                elif (isinstance(hint, RsNamed)
                      and 'Vec<' in hint.name and 'Vec<' in ty.name
                      and hint.name != ty.name
                      and isinstance(expr, CastExpr) and expr.target == 'JArray<Object>'):
                    # Vec<Object> (checkcast 擦除) ↔ Vec<E> (LVT 类型参数)：
                    # 按 CastExpr 节点分派（A-3），重定向转换目标为 hint 类型，
                    # 使字段赋值类型一致（旧字符串改写形态在当前语料零触发，见报告）
                    expr = CastExpr(expr.expr, hint.name,
                                    binary_name=expr.binary_name, checked=expr.checked)
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
            # monitor 追踪变量供配对的 monitorexit（异常处理器路径）使用——存身份等价
            # 的 Object 装箱（wrapper 克隆共享存储 __identity，与 monitorenter 侧
            # 命中同一监视器）。vtable 上下文的 Clone::clone(this) 由
            # classify_vtable_body 路由到 wrapper 重建（NeedsWrapper）。
            _top_is_this = (
                len(self.stack) > 0
                and isinstance(self.stack[-1][0], Var)
                and self.stack[-1][0].name == 'this'
            )
            if _top_is_this:
                expr = RawExpr("Object::from(Clone::clone(this))")
                ty = RsNamed("Object")
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
        elif (slot in self.locals and decl is None and slot not in self._param_slots
                and self.locals[slot][0] != self._undeclared_slot_name(slot)):
            # slot 被编译器合成的临时变量复用：与此前的 Java 变量是两个变量
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
        if (slot in self._param_slots and slot in self.locals
                and isinstance(self.locals[slot][1], RsNamed)
                and self.locals[slot][1].name not in ('Object', '()')
                and isinstance(ty, (RsNamed, RsGeneric))
                and getattr(ty, 'name', '') not in _SCALAR_TYPE_NAMES
                and render_type(self.locals[slot][1]) != render_type(ty)):
            # 形参声明为具体引用类型，方法体内赋入异型引用值（`if (rnd == null)
            # rnd = ThreadLocalRandom.current()`，rnd: Random、值: ThreadLocalRandom）：
            # 形参槽强制 decl=None，decl_ty 子类上转分支不达、槽位复用改名又按名字全域
            # 匹配误判成合成变量 → 落死局部 local_N，形参永不重绑定。与下方同变量分支
            # （LVT 声明区间内漂移）同源：经 Object 边界按形参声明类型 From<Object>
            # 重建（子类值按超类擦除 vtable 重建视图，共享存储与对象标识；家族不符抛
            # CCE，checkcast 语义），赋回原形参——let 阴影只在当前块内可见，块外
            # （循环体/后续语句）仍读到旧值，运行期 NPE。
            _src = render_expr(_clone_moved_var(expr, ty))
            self.stmts.append(AssignStmt(
                Var(self.locals[slot][0]),
                RawExpr(f'<{render_type(self.locals[slot][1])} as ::std::convert::From<Object>>'
                        f'::from(Object::from({_src}))')))
            return
        if slot in self.locals:
            name, old_ty, _ = self.locals[slot]
            decl_depth = self._slot_decl_depth.get(slot, 0)
            # T42: slot 类型发生变化时（如 for-each 迭代器 slot 被后续变量复用），
            # 用 let 阴影（shadowing）而非赋值，避免 Rust 类型不匹配
            # T68: slot 在内层作用域（depth > current）中首次声明时，在外层访问必须用 let
            if render_type(old_ty) != render_type(ty) or decl_depth > self._current_depth:
                _bind_pos = self._slot_bind_pos.get(slot)
                _same_var_decl = next((
                    (_st, _en) for _st, _en, _nm, *_r in self._slot_decls.get(slot, ())
                    if _st <= self.current_offset < _en and _safe_name(_nm) == name), None)
                if (decl is not None and decl_name == name
                        and _same_var_decl is not None
                        and _bind_pos is not None
                        # LVT 区间从初始化 store 的下一条指令开始（javac 约定），绑定
                        # 创建点允许落在区间起点之前（初始化 store 偏移）。逐区间偏移
                        # 算术只覆盖一字节 astore_N（start-1）；两字节 astore N 的初始
                        # 化 store 落 start-2（ObjectStreamClass.getProtectionDomains 的
                        # pds 槽：pc29 初始化 astore / LVT start 31 实证）——改按区间
                        # 判定：绑定点不落入本槽**其他** LVT 声明区间（同名跨区间是
                        # javac 槽位复用的另一个变量 → 维持 let 阴影路径），且先于本
                        # 区间终点。
                        and not any(_st <= _bind_pos < _en and (_st, _en) != _same_var_decl
                                    for _st, _en, _nm, *_r in self._slot_decls.get(slot, ()))
                        and _bind_pos < _same_var_decl[1]
                        and decl_depth <= self._current_depth
                        and isinstance(old_ty, (RsNamed, RsGeneric))
                        and getattr(old_ty, 'name', '') not in _SCALAR_TYPE_NAMES
                        and getattr(old_ty, 'name', '') != '()'
                        and isinstance(ty, (RsNamed, RsGeneric))
                        and getattr(ty, 'name', '') not in _SCALAR_TYPE_NAMES):
                    # 同一 Java 变量（当前绑定与本次存储落在同一 LVT 声明区间内）在
                    # 原声明仍可见处存入渲染不同的引用类型（raw/通配实例化 ↔ 具体实例化
                    # 的漂移，如 AbstractPipeline.wrapSink 的 `p = p.previousStage`：
                    # 变量声明为具体实例化、字段是 Object 实参实例化）。JVM 槽位是
                    # 函数级单一可变变量，let 阴影只在当前块内可见——循环体内赋值会在
                    # 迭代间丢失（循环变量永不前进 → 死循环）。经 Object 边界按声明
                    # 类型重建（From<Object> for X<A> 的擦除路径对任意实例化成立，
                    # 保持对象标识；等价 Java 的 raw/unchecked 赋值），落回原变量赋值。
                    # 引用类型间的家族不符会在 From<Object> 路径抛 CCE（checkcast 语义）。
                    # 同名但跨声明区间（javac 槽位复用：多个 catch handler 的 e）
                    # 不属于同变量 → 维持既有 let 阴影路径
                    _src = render_expr(_clone_moved_var(expr, ty))
                    self.stmts.append(AssignStmt(
                        Var(name),
                        RawExpr(f'<{render_type(old_ty)} as ::std::convert::From<Object>>'
                                f'::from(Object::from({_src}))')))
                    return
                if (decl is None and render_type(old_ty) != render_type(ty)
                        and any(_safe_name(_d[2]) == name and not (_d[0] <= self.current_offset < _d[1])
                                for _d in self._slot_decls.get(slot, ()))):
                    # 存储点不在该槽任何声明变量的作用域内、类型又与槽上已结束作用域的声明变量不同：
                    # javac 合成变量（for-each 的数组副本等）复用了槽位 → 是另一个变量，按槽位另行命名，
                    # 不与原声明变量同名（同名 let 在变量提升后会退化为对原变量的赋值）。
                    # 名字匹配限定「区间不覆盖当前偏移」的声明条目：命中覆盖当前偏移的条目
                    # （如形参自身的 LVT 条目——恒覆盖整个方法体）会把形参重绑定误判成
                    # 合成变量改名，落死局部 local_N。
                    name = f"local_{slot}"
                self.locals[slot] = (name, ty, True)
                self._slot_decl_depth[slot] = self._current_depth
                self._slot_bind_pos[slot] = self.current_offset
                value = _maybe_downcast(expr, ty) if src_is_object else expr
                _is_default = isinstance(value, RawExpr) and value.code == 'Default::default()'
                let_ty = ty if (_is_default or force_let_ty) else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
                # Java 局部变量间赋值在 Rust 中是 move，包 Clone 保活源变量（E0382）
                value = _clone_moved_var(value, ty)
                self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value, value_ty=ty))
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
            name = decl_name or self._undeclared_slot_name(slot)
            self.locals[slot] = (name, ty, True)
            self._slot_decl_depth[slot] = self._current_depth
            self._slot_bind_pos[slot] = self.current_offset
            value = _maybe_downcast(expr, ty) if src_is_object else expr
            # downcast 时让 Rust 推断类型；Default::default() 需保留类型注解；RsGeneric 也让 Rust 推断
            _is_default = isinstance(value, RawExpr) and value.code == 'Default::default()'
            let_ty = ty if (_is_default or force_let_ty) else (None if isinstance(value, RawExpr) else (None if isinstance(expr, Var) and isinstance(ty, RsGeneric) else ty))
            # Java 局部变量间赋值（aload src; astore dst）在 Rust 中是 move，
            # 源变量后续仍会被使用，包 Clone::clone 保活（E0382 use-after-move）
            value = _clone_moved_var(value, ty)
            self.stmts.append(LetStmt(name, let_ty, mutable=True, value=value, value_ty=ty))

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
        return (Var(self._undeclared_slot_name(slot)), I32)

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

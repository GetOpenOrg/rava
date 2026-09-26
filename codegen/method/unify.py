"""
块级值合并与条件构造（窗口 3 ⑧，自 method/blocks.py 纯搬移，零逻辑改动）。

- jump_condition：跳转指令 → Cond（弹操作数、按类型强转比较数）
- unify_pair / unify_values / arm_value：汇合点各前驱栈值的类型统一与合并
- inline_temps：条件块内单用临时量内联进条件
- CondExpr：携带 Cond 的 RawExpr（jump_condition 与 BlockSimulator 共用）
"""

from __future__ import annotations
import re
from dataclasses import dataclass

from ..cfg import (
    TWO_OPERAND_BRANCH_OPS,
    Cond, atom, negate, render_cond, map_atoms, cmp_op, neg_cmp_op,
)
from ..constants import PRIMITIVE_RUST_TYPES as _PRIM_TYPES
from ..instr.hierarchy import _common_ref_type, _common_ref_type_widening
from ..render import render_expr, render_type
from ..rs_ir import RawExpr, RawStmt, Var
from ..stack import StackSim, _clone_moved_var, erased_base
from .vars import _coerce_icmp_operand, _coerce_acmp_operand, _str_to_rs_type


@dataclass
class CondExpr(RawExpr):
    """由条件跳转物化出的布尔值（`return a == b || c;`）。保留 Cond 以便被后续 ifeq/ifne 直接复用。"""
    cond: Cond = None


# ─────────────────────────────────────────────────────────────────────────────
# 条件构造
# ─────────────────────────────────────────────────────────────────────────────

def _uses_jvm_null_method(ty: str, type_params=()) -> bool:
    """类型是否用 .is_jvm_null() 检测 null（java_class! 生成类、JArray 数组引用）；
    其余走 _is_jnull()。
    type_params：当前类的类型变量（`T_BUFFER` 等任意命名），类型变量上没有 wrapper 方法。
    JArray 的 null 是 Repr::Null（S-3.1：null 数组引用与空数组严格区分），由固有方法承载。"""
    if ty in ('Object', '()', '') or ty in _PRIM_TYPES or ty in type_params:
        return False
    if ty.startswith(('Rc<', '__Shared<', 'Vec<', 'Box<', 'std::')):
        return False
    if len(ty) <= 2 and ty[0].isupper() and ty.rstrip('0123456789').isalpha():
        return False
    return True


def _is_bool(ty) -> bool:
    return str(ty) == 'bool' or getattr(ty, 'name', '') == 'bool'


def jump_condition(op: str, sim: StackSim, registry=None) -> Cond:
    """弹出条件跳转的操作数，返回「跳转成立」的条件。"""
    if op in TWO_OPERAND_BRANCH_OPS:
        b_e, b_t = sim.pop()
        a_e, a_t = sim.pop()
        if op in ('if_acmpeq', 'if_acmpne'):
            a_s = _coerce_acmp_operand(render_expr(a_e), a_t, registry, sim.class_type_params)
            b_s = _coerce_acmp_operand(render_expr(b_e), b_t, registry, sim.class_type_params)
        else:
            a_s = _coerce_icmp_operand(render_expr(a_e), a_t)
            b_s = _coerce_icmp_operand(render_expr(b_e), b_t)
        return atom(cmp_op(op, a_s, b_s), neg_cmp_op(op, a_s, b_s))
    a_e, a_t = sim.pop()
    if op in ('ifeq', 'ifne') and _is_bool(a_t):
        base = a_e.cond if isinstance(a_e, CondExpr) else atom(render_expr(a_e))
        return base if op == 'ifne' else negate(base)
    a_s = render_expr(a_e)
    if op in ('ifnull', 'ifnonnull'):
        if _uses_jvm_null_method(render_type(a_t), sim.class_type_params or ()):
            is_null = atom(f'{a_s}.is_jvm_null()', f'!{a_s}.is_jvm_null()')
        else:
            is_null = atom(f'_is_jnull(&{a_s})', f'!_is_jnull(&{a_s})')
        return is_null if op == 'ifnull' else negate(is_null)
    return atom(cmp_op(op, a_s, ''), neg_cmp_op(op, a_s, ''))


# ─────────────────────────────────────────────────────────────────────────────
# 分支臂取值与类型统一
# ─────────────────────────────────────────────────────────────────────────────

_INT_TYPES = frozenset({'i8', 'i16', 'i32', 'i64', 'u8', 'u16', 'u32', 'u64'})
_SCALAR_TYPES = _INT_TYPES | {'bool', 'f32', 'f64'}
# JVM 计算类型为 int 的 Rust 标量（byte / short / char / int）
_JVM_INT_FAMILY = frozenset({'i8', 'i16', 'u16', 'i32'})
_NULL_EXPRS = frozenset({'Object::default()', 'Clone::clone(&Object::default())'})


def arm_value(entry) -> str:
    """留在栈上的值 → 值位置表达式。局部变量出现在值位置是 Rust move，
    Java 引用无 move 语义 → 包 Clone::clone 保活；this 是 &Self → Clone::clone(this)。"""
    expr, ty = entry
    if isinstance(expr, Var) and expr.name == 'this':
        return 'Clone::clone(this)'
    return render_expr(_clone_moved_var(expr, ty))


def unify_pair(tv: str, ty, ev: str, ety, class_tparams, registry):
    """两个汇合值统一到同一 Rust 类型。返回 (tv, ev, ty)。"""
    ty_str, ety_str = render_type(ty), render_type(ety)
    if ty_str == ety_str:
        return tv, ev, ty
    same_base = (erased_base(ty_str) == erased_base(ety_str)
                 and '<' in ty_str and '<' in ety_str)
    if ty_str == 'bool' and ety_str in _INT_TYPES:
        # 汇合点的 JVM 类型两侧必然一致；bool 侧只可能来自 0/1 菱形折叠
        # （比较结果值，_ternary_value），int 侧是同 JVM int 类型的值
        # （`x < y ? -1 : (x == y ? 0 : 1)`）→ 以 int 为准，bool 经 as 还原 0/1。
        # 反方向（int → `!= 0`）会破坏非 0/1 int 值（-1 变 1）。
        tv = f"({tv} as {ety_str})"
        ty = ety
    elif ety_str == 'bool' and ty_str in _INT_TYPES:
        ev = f"({ev} as {ty_str})"
    elif ev in _NULL_EXPRS and ty_str not in _SCALAR_TYPES:
        ev = 'Default::default()'
    elif tv in _NULL_EXPRS and ety_str not in _SCALAR_TYPES:
        tv = 'Default::default()'
        ty = ety
    elif (ety_str == 'Object' and ty_str not in _SCALAR_TYPES and ty_str != 'Object'
          and ty_str not in class_tparams):
        # 具体类型臂与擦除 Object 臂汇合（对端臂字节码真型是接口/根类——如
        # Nodes.builder 的 else 臂调用 builder() 声明返回 Node$Builder，接口擦成
        # Object）：JVM 校验器的合并点是公共祖先，绝不窄于任一臂——异构兄弟类
        # （FixedNodeBuilder vs SpinedNodeBuilder 同父 Builder）不能定型为首臂
        # 具体类并伪造 checkcast（try_cast 制造 Java 没有的 CCE）。合并局部是
        # 合成槽无 LVT 声明形态可依 → 合并点按擦除 Object 落定，具体臂经
        # _coerce_to_object 上转（Object::from 保对象标识与 vtable，后续接口
        # 调用动态分派，与声明接口类型的字节码语义一致）。源码真 checkcast
        # （javac 对显式窄化的发射）已在上游 sim/control.py 定型臂类型，两臂
        # 同型不进此分支——此分支恒为「对端静态类型更宽」的形态。
        from ..instr.coerce import _coerce_to_object
        tv = _coerce_to_object(tv, ty_str, registry, class_tparams, clone=False)
        ty = _str_to_rs_type('Object')
    elif (ty_str == 'Object' and ety_str not in _SCALAR_TYPES and ety_str != 'Object'
          and ety_str not in class_tparams):
        # A-4（阶段 0 证据 merge-box 段）：具体类型臂并入擦除 Object 合并槽（目标的
        # Java 静态类型多为接口——trySplit 等协变三元合并）。装箱经 _coerce_to_object
        # 单一决策点（与下方无公共父类分支同一约定，clone=False——arm_value 已是值
        # 位置表达式）：registry 类走 Object::from 保持对象身份与 vtable（from_any
        # 装成 JvmRef 会丢运行时类 / is_instance_of / __interface 应答）；仅未知形态
        # 回落 from_any。
        from ..instr.coerce import _coerce_to_object
        ev = _coerce_to_object(ev, ety_str, registry, class_tparams, clone=False)
    elif ty_str in _JVM_INT_FAMILY and ety_str in _JVM_INT_FAMILY:
        # 两臂同属 JVM 计算类型 int（byte/short/char 局部加载到栈上即 int，JVMS §2.11.1）
        # 而 Rust 类型不同：汇合值是 int——两臂拓宽到 i32。按首臂收窄会截断另一臂的
        # int 运算结果（`b >= 0 ? b : b + 256`，b 为 byte → 256 偏移被 as i8 灭掉）。
        if ty_str != 'i32':
            tv = f"({tv} as i32)"
        if ety_str != 'i32':
            ev = f"({ev} as i32)"
        ty = _str_to_rs_type('i32')
    elif ty_str in _SCALAR_TYPES or ety_str in _SCALAR_TYPES:
        ev = f"({ev} as {ty_str})"
    elif _common_ref_type(ty_str, ety_str, registry):
        common = _common_ref_type(ty_str, ety_str, registry)
        if ty_str != common:
            tv = f"{common}::from({tv})"
        if ety_str != common:
            ev = f"{common}::from({ev})"
        ty = _str_to_rs_type(common)
    elif _common_ref_type_widening(ty_str, ety_str, registry):
        # 泛型父子类臂（类型实参一致）：TreeNode<K, V> 臂并入 Node<K, V> 臂——基名走
        # 继承链公共类祖先（与槽位 widening _merged_slot_type 同一规则），实参一致
        # 保留实参。合并点取父类，子类臂经宏 all_superclasses 生成的 From<Child> for
        # Ancestor 上转（__from_parts 保对象标识与运行时类，vtable 视图窄化到祖先）。
        # 不补此分支时泛型对落「无公共父类」分支双双上转根类——合并局部是合成槽，
        # 定型退化 Object 后接收者方法全丢（CHM.clear 的三目合并
        # p = (f.hash>=0) ? f : ((f instanceof TreeBin) ? t.first : null)，
        # 两臂字节码真型 Node<K,V> / TreeNode<K,V>，p.is_jvm_null/__get_next E0599）。
        # 接口祖先 / 根类 / 实参不一致由 _common_ref_type_widening 自行拒绝（None）
        # → 维持后续分支语义不变。置于非泛型 _common_ref_type 分支之后：非泛型对
        # 行为逐字不变（回归热区）。
        common = _common_ref_type_widening(ty_str, ety_str, registry)
        if ty_str != common:
            tv = f"<{common} as ::std::convert::From<_>>::from({tv})"
        if ety_str != common:
            ev = f"<{common} as ::std::convert::From<_>>::from({ev})"
        ty = _str_to_rs_type(common)
    elif same_base and 'Object' in ty_str and any(t in ety_str for t in class_tparams):
        # 同基泛型的「擦除 Object 实例化臂 vs 具体泛型臂」：合并点取具体臂类型，
        # Object 实例化臂经 Object 边界重建（From<Object> for X<A> 任意 A 成立，
        # 身份保持——A-4 批次 6 combine 修复 86bf63a 同型），不静默 Default::default()
        # （会灭真值：TreeMap_NavigableSubMap.navigableKeySet 的缓存命中臂 →
        # 第二次起返回 null KeySet → NPE）。null 臂（aconst_null）不进此分支——
        # 已被上方 _NULL_EXPRS 分支接管（hash_map spliterator 的三处合法 null 臂）。
        tv = f"<{ety_str} as ::std::convert::From<Object>>::from(Object::from({tv}))"
        ty = ety
    elif same_base and 'Object' in ety_str and 'Object' not in ty_str:
        # 镜像：tv 是具体泛型臂（保留原值），ev 是擦除 Object 实例化臂 → 经 Object
        # 边界按 tv 的类型重建，合并点取具体臂类型
        ev = f"<{ty_str} as ::std::convert::From<Object>>::from(Object::from({ev}))"
    elif same_base and '<' in ty_str and '<' in ety_str:
        # 同一泛型类的不同实例化（静态泛型方法的类型变量解与擦除元素臂——andTree 的
        # CompletableFuture<Void> 与 cfs.get 的 <Object>）：合并点取擦除实例化
        #（与菱形落定同形态），各臂经 Object 边界重建（From<Object> for X<A> 对任意
        # A 成立，保持对象标识；运行时类校验由视图承载）
        from ..type_args import split_rust_type_args as _split_rust_args
        _base = erased_base(ty_str)
        _arity = max(len(_split_rust_args(ty_str)), len(_split_rust_args(ety_str)))
        _tgt = f"{_base}<{', '.join(['Object'] * _arity)}>"
        if ty_str != _tgt:
            tv = f"<{_tgt} as ::std::convert::From<Object>>::from(Object::from({tv}))"
        if ety_str != _tgt:
            ev = f"<{_tgt} as ::std::convert::From<Object>>::from(Object::from({ev}))"
        ty = _str_to_rs_type(_tgt)
    elif ty_str not in _SCALAR_TYPES and ety_str not in _SCALAR_TYPES and not same_base:
        # 两臂是无公共父类的引用类型（含类型变量）：按 JVM 校验器的类型合并规则，
        # 合并点类型为根类 → 两臂各自上转
        from ..instr.coerce import _coerce_to_object
        if ty_str != 'Object':
            tv = _coerce_to_object(tv, ty_str, registry, class_tparams, clone=False)
        if ety_str != 'Object':
            ev = _coerce_to_object(ev, ety_str, registry, class_tparams, clone=False)
        ty = _str_to_rs_type('Object')
    return tv, ev, ty


_HOLE = '\x00'


def unify_values(entries: list, class_tparams, registry):
    """N 个汇合值统一类型。返回 ([value_str], ty)。"""
    values = [arm_value(entries[0])]
    ty = entries[0][1]
    for entry in entries[1:]:
        hole, ev, ty = unify_pair(_HOLE, ty, arm_value(entry), entry[1], class_tparams, registry)
        if hole != _HOLE:
            # null 臂（Default::default()）可直接成为任何引用类型，不套转换（套了反而使其类型无解）
            values = [v if v == 'Default::default()' else hole.replace(_HOLE, v) for v in values]
        values.append(ev)
    # 汇合值是类型实参待推断的菱形构造结果（`X<_>`）：汇合变量没有声明类型可供反推，
    # 类型实参按擦除形态 Object 落定（与未绑定构造结果直接作接收者同规则）
    ty_str = render_type(ty)
    infer_m = re.match(r'^(\w+)<(_(?:, _)*)>$', ty_str)
    if infer_m:
        base, holes = infer_m.group(1), infer_m.group(2)
        erased = ', '.join('Object' for _ in holes.split(', '))
        values = [v.replace(f"{base}::<{holes}>", f"{base}::<{erased}>")
                   .replace(f"{base}<{holes}>", f"{base}<{erased}>") for v in values]
        ty = _str_to_rs_type(f"{base}<{erased}>")
    return values, ty


def _same_entry(a, b) -> bool:
    if a is b or a[0] is b[0]:
        return True
    return render_expr(a[0]) == render_expr(b[0]) and render_type(a[1]) == render_type(b[1])


def _same_stack(a: list, b: list) -> bool:
    return len(a) == len(b) and all(_same_entry(x, y) for x, y in zip(a, b))


# ─────────────────────────────────────────────────────────────────────────────
# 临时变量回填：`let _tN = call?; if _tN {` → 条件内联
# ─────────────────────────────────────────────────────────────────────────────

_TEMP_LET_RE = re.compile(r'^let (mut )?(_[A-Za-z]\w*?\d+)(?:: (.+?))? = (.*);$', re.S)
_NO_INLINE_MARKERS = ('Default::default()', '.into()', 'panic!', '\n')


def _parse_temp_let(stmt):
    """RawStmt 形式的临时变量声明 → (name, mutable, ty_text | None, value_text)；否则 None。"""
    if not isinstance(stmt, RawStmt):
        return None
    m = _TEMP_LET_RE.match(stmt.code.strip())
    if not m:
        return None
    return m.group(2), bool(m.group(1)), m.group(3), m.group(4)


def _needs_paren(text: str) -> bool:
    depth = 0
    for ch in text:
        if ch in '([{':
            depth += 1
        elif ch in ')]}':
            depth -= 1
        elif depth == 0 and ch in ' <>=|&+-*/%!':
            return True
    return False


def _word_count(name: str, text: str) -> int:
    return len(re.findall(r'(?<![\w.])' + re.escape(name) + r'\b', text))


def inline_temps(stmts: list, cond: Cond, exit_stack: list) -> Cond | None:
    """若 stmts 全部是只用一次的临时变量声明，把它们回填进条件，返回新条件；否则 None。

    保持求值顺序：临时变量在条件文本中的出现顺序必须与声明顺序一致。
    """
    if not stmts:
        return cond
    temps: list[tuple[str, str]] = []
    for stmt in stmts:
        parsed = _parse_temp_let(stmt)
        if parsed is None:
            return None
        name, mutable, _ty, value = parsed
        if mutable or any(mk in value for mk in _NO_INLINE_MARKERS):
            return None
        temps.append((name, value))
    stack_text = ' '.join(render_expr(e) for e, _ in exit_stack)
    resolved: dict[str, str] = {}
    used_in_value: set[str] = set()
    for name, value in temps:
        for prev in list(resolved):
            n = _word_count(prev, value)
            if n == 0:
                continue
            if n != 1 or prev in used_in_value:
                return None
            pv = resolved[prev]
            value = re.sub(r'(?<![\w.])' + re.escape(prev) + r'\b',
                           lambda _m, _pv=pv: f'({_pv})' if _needs_paren(_pv) else _pv, value)
            used_in_value.add(prev)
        resolved[name] = value
    top = [n for n, _ in temps if n not in used_in_value]
    text = render_cond(cond)
    positions = []
    for name in top:
        if _word_count(name, text) != 1 or _word_count(name, stack_text) != 0:
            return None
        positions.append(re.search(r'(?<![\w.])' + re.escape(name) + r'\b', text).start())
    for name in used_in_value:
        if _word_count(name, text) != 0 or _word_count(name, stack_text) != 0:
            return None
    if positions != sorted(positions):
        return None

    def substitute(s: str) -> str:
        for name in top:
            val = resolved[name]
            rep = f'({val})' if _needs_paren(val) else val
            s = re.sub(r'(?<![\w.])' + re.escape(name) + r'\b', lambda _m, _r=rep: _r, s)
        return s

    return map_atoms(cond, substitute)

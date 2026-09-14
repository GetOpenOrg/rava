"""
JVM 字节码指令 → Rust 语句转换。
每条指令操作 StackSim 的栈与语句列表。

新设计：
- String（java.lang.String）而非 Rust std::string::String
- 字段通过 Field<T>::get()/set() 访问
- println → System::out().println(...)?
- 所有用户类方法调用加 ?（返回 Result<T>）
"""

import re
import math
from .stack import StackSim, I32, I64, F32, F64, BOOL, UNIT
from .constants import safe_ident as _safe_field

# Python float → Rust 字面量（处理 nan/inf/-inf 等特殊值）
_FLOAT_SPECIAL = {
    'nan': '{ty}::NAN', 'inf': '{ty}::INFINITY', '-inf': '{ty}::NEG_INFINITY',
    'infinity': '{ty}::INFINITY', '-infinity': '{ty}::NEG_INFINITY',
}

def _float_lit(val_str: str, ty: str) -> str:
    """将 Python float repr 转为合法 Rust 字面量，处理 nan/inf/-inf。"""
    key = val_str.lower()
    if key in _FLOAT_SPECIAL:
        return _FLOAT_SPECIAL[key].format(ty=ty)
    return val_str + ty


def _escape_str(s: str) -> str:
    """将原始字符串内容转义为 Rust 字符串字面量内容（不含两端的 "）。
    处理：\ → \\，" → \"，控制字符，以及无效的 \% 等 Java 格式化符号。"""
    result = []
    i = 0
    while i < len(s):
        c = s[i]
        if c == '\\':
            # 已有反斜杠：检查下一个字符是否构成合法 Rust 转义序列
            if i + 1 < len(s):
                nc = s[i + 1]
                if nc in ('"', "'", '\\', 'n', 'r', 't', '0', 'x', 'u'):
                    result.append('\\')
                    result.append(nc)
                    i += 2
                    continue
                else:
                    # 非法转义（如 \%、\u 后跟非十六进制）→ 转义为 \\
                    result.append('\\\\')
                    i += 1
                    continue
            else:
                result.append('\\\\')
                i += 1
        elif c == '"':
            result.append('\\"')
            i += 1
        elif c == '\n':
            result.append('\\n')
            i += 1
        elif c == '\r':
            result.append('\\r')
            i += 1
        elif c == '\t':
            result.append('\\t')
            i += 1
        else:
            result.append(c)
            i += 1
    return ''.join(result)
from .rs_ir import (
    Lit, Var, BinOp, UnOp, Call, MethodCall, FieldAccess, Index,
    Cast, RawExpr, RawStmt, LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    IfStmt, LoopStmt, BreakStmt, RsNamed, RsGeneric, RsRef, RsSlice, RsInfer,
    NewPendingExpr, StaticFieldRef,
)
from .render import render_expr, render_type
from .type_map import (
    jvm_to_rust, sig_type, short_cls,
    NEWARRAY_TYPES,
    BOXING_SKIP_STATIC, UNBOX_VIRTUAL,
    parse_descriptor_params, parse_descriptor_return,
    mangle_name, get_ergonomic_jvm_rename,
)

from .types import Instr


# ── 辅助：解析 javap 注释中的方法引用 ────────────────────────────

def parse_method_ref(comment: str) -> tuple[str | None, str, list, str]:
    """
    解析 'Method Foo.bar:(II)I' 或 'InterfaceMethod ...' 格式。
    返回 (class_short_name, method_name, param_jvm_types, return_jvm_type)
    """
    comment = comment.strip()
    for prefix in ('Method ', 'InterfaceMethod '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]

    comment = (comment
               .replace('"<init>"',   '__init__')
               .replace('"<clinit>"', '__clinit__')
               .replace('<init>',     '__init__')
               .replace('<clinit>',   '__clinit__'))

    m = re.match(r'(?:([^.]+)\.)?(\w+(?:<\w+>)?):(\([^)]*\).+)', comment)
    if not m:
        return (None, comment, [], 'V')

    raw_cls = m.group(1)
    mname   = m.group(2).replace('__init__', '<init>').replace('__clinit__', '<clinit>')
    desc    = m.group(3)

    if raw_cls:
        raw_cls = raw_cls.split('/')[-1].split('.')[-1].replace('$', '_')

    return (raw_cls, mname, parse_descriptor_params(desc), parse_descriptor_return(desc))


def _parse_slot(op: str, operand: str) -> int:
    if '_' in op:
        return int(op.split('_')[-1])
    return int(operand.strip()) if operand else 0


# java_runtime 手写实现的短类名：这些类的方法名不经过 mangle（hand-written API 已定好名称）
# System/PrintStream/String/Math/ArrayList/HashMap/HashSet/StringBuilder 已迁移到 jdk_classes + native_impls
_JAVA_RUNTIME_SHORT_NAMES: frozenset[str] = frozenset({
    'Object',
})


def _to_i32(expr_str: str, ty: 'RsType') -> str:
    """窄类型（i8/i16/u16/bool）向上转为 i32，避免 JVM int 运算中的类型不匹配。"""
    ty_name = getattr(ty, 'name', '')
    if ty_name in ('i8', 'i16', 'u16', 'bool'):
        return f"({expr_str} as i32)"
    return expr_str


def _coerce_to_object(val_str: str, ty: str) -> str:
    """将任意类型的值强制转换为 Object。
    基本类型用 .into()（有 From<T> for Object 实现）；
    其他类型用 Object::from_any(.clone())（通用装箱）。"""
    if ty in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        return f"{val_str}.into()"
    return f"Object::from_any({val_str}.clone())"


_NULL_OBJECT_EXPRS = frozenset({'Object::default()', 'Object::default().clone()'})

# 接口类型 → 已知实现类型集合（用于接口子类型强制转换）
_INTERFACE_IMPLS: dict[str, frozenset[str]] = {
    'CharSequence': frozenset({'String', 'StringBuilder', 'StringBuffer', 'AbstractStringBuilder'}),
    'Map': frozenset({'HashMap', 'TreeMap', 'LinkedHashMap', 'Hashtable', 'WeakHashMap',
                      'IdentityHashMap', 'ConcurrentHashMap', 'Properties', 'EnumMap',
                      'ImmutableCollections_Map1', 'ImmutableCollections_MapN'}),
    'List': frozenset({'ArrayList', 'LinkedList', 'Vector', 'Stack', 'AbstractList',
                       'ImmutableCollections_List12', 'ImmutableCollections_ListN',
                       'ProviderList_ServiceList', 'ImmutableCollections_SubList'}),
    'Set': frozenset({'HashSet', 'LinkedHashSet', 'TreeSet', 'EnumSet',
                      'ImmutableCollections_Set12', 'ImmutableCollections_SetN',
                      'TreeMap_EntrySet', 'HashMap_KeySet', 'ConcurrentHashMap_KeySetView'}),
    'Collection': frozenset({'List', 'Set', 'ArrayList', 'LinkedList', 'HashSet',
                              'LinkedHashSet', 'TreeSet', 'Vector', 'ArrayDeque'}),
    'Iterable': frozenset({'Collection', 'List', 'Set', 'ArrayList', 'HashSet', 'LinkedList'}),
    'Queue': frozenset({'LinkedList', 'ArrayDeque', 'PriorityQueue'}),
    'Deque': frozenset({'LinkedList', 'ArrayDeque'}),
    'SortedMap': frozenset({'TreeMap'}),
    'SortedSet': frozenset({'TreeSet'}),
    'NavigableMap': frozenset({'TreeMap'}),
    'NavigableSet': frozenset({'TreeSet'}),
}


def _coerce_to_interface(actual: str, expected: str) -> bool:
    """当 actual 需要强制转换为 Default::default() 时返回 True。
    覆盖两类情况：
    1. actual 是 expected 接口的已知实现类（如 HashMap → Map）
    2. Vec 元素类型不匹配（Rc<RefCell<Vec<Object>>> → Rc<RefCell<Vec<T>>>）
    """
    # Vec 元素类型不匹配：两者都是 Rc<RefCell<Vec<T>>> 但元素类型不同
    _VEC_PREFIX = 'Rc<RefCell<Vec<'
    _VEC_SUFFIX = '>>>'
    if (expected.startswith(_VEC_PREFIX) and expected.endswith(_VEC_SUFFIX) and
            actual.startswith(_VEC_PREFIX) and actual.endswith(_VEC_SUFFIX)):
        exp_elem = expected[len(_VEC_PREFIX):-len(_VEC_SUFFIX)]
        act_elem = actual[len(_VEC_PREFIX):-len(_VEC_SUFFIX)]
        if exp_elem != act_elem:
            return True
    exp_base = expected.split('<')[0]
    act_base = actual.split('<')[0]
    if exp_base == act_base:
        return False
    return act_base in _INTERFACE_IMPLS.get(exp_base, frozenset())


def _coerce_from_null(val_str: str, expected: str) -> str | None:
    """若 val_str 是 aconst_null 的结果（Object::default()），
    且 expected 是具体的引用类型，返回 Default::default() 作为替代。
    否则返回 None 表示无需特殊处理。"""
    if val_str not in _NULL_OBJECT_EXPRS:
        return None
    if expected in ('Object', '()') or expected in ('i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16'):
        return None
    # null 作为参数：用 Default::default() 提供类型安全的零值
    return 'Default::default()'


def _coerce_value(val_str: str, val_ty: 'RsType', target: str) -> str:
    """将 val 强制转换为 target 字段/参数类型，避免窄类型与 i32 不匹配。
    只在必要时插入 cast，若类型已匹配则原样返回。"""
    src = getattr(val_ty, 'name', '')
    if target == src:
        return val_str
    if target == 'i32':
        if src == 'bool':
            return f"({val_str}) as i32"
        return val_str
    if target == 'bool':
        if src == 'bool':
            return val_str
        return f"({val_str} != 0i32)"
    if target in ('i8', 'i16', 'u16'):
        if src != 'i32':
            val_str = f"({val_str} as i32)"
        return f"(({val_str}) as {target})"
    return val_str


def _parse_field_ref(comment: str) -> tuple[str, str, str]:
    """解析 'Field java/lang/System.out:Ljava/io/PrintStream;' 格式。
    返回 (class_binary_name, field_name, descriptor)。
    field_name 已经过 _safe_field 处理（$ → _, Rust 关键字加 _）。"""
    comment = comment.strip()
    for prefix in ('Field ', 'InterfaceField '):
        if comment.startswith(prefix):
            comment = comment[len(prefix):]
    # 格式：java/lang/System.out:Ljava/io/PrintStream;
    if ':' in comment:
        ref_part, descriptor = comment.split(':', 1)
    else:
        ref_part, descriptor = comment, 'Ljava/lang/Object;'
    if '.' in ref_part:
        cls, field = ref_part.rsplit('.', 1)
    else:
        cls, field = '', ref_part
    return cls, _safe_field(field), descriptor


# ── 主分发函数 ────────────────────────────────────────────────────

def sim_instr(ins: Instr, sim: StackSim, class_name: str, registry: dict | None = None):
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    # ── 整型常量 ──
    if   op == 'iconst_m1':                      sim.push(Lit('-1i32'), I32)
    elif op.startswith('iconst_'):               sim.push(Lit(f"{op[-1]}i32"), I32)
    elif op in ('lconst_0', 'lconst_1'):         sim.push(Lit(f"{op[-1]}i64"), I64)
    elif op in ('fconst_0', 'fconst_1', 'fconst_2'): sim.push(Lit(f"{op[-1]}f32"), F32)
    elif op in ('dconst_0', 'dconst_1'):         sim.push(Lit(f"{op[-1]}f64"), F64)
    elif op == 'bipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'sipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'ldc':
        if operand.startswith('"'):
            # javap 已经以 "..." 格式给出（operand 是完整的带引号字符串），直接用
            sim.push(Lit(f"String::from({operand})"), RsNamed('String'))
        elif comment.startswith('String '):
            lit = _escape_str(comment[7:].strip())
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('int '):    sim.push(Lit(comment[4:].strip() + 'i32'), I32)
        elif comment.startswith('float '): sim.push(Lit(_float_lit(comment[6:].strip(), 'f32')), F32)
        elif comment.startswith('long '):  sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('class '): sim.push(Lit('Class::<Object>::default()'), RsGeneric('Class', [RsNamed('Object')]))
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op in ('ldc2_w', 'ldc_w'):
        if comment.startswith('long '):   sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('String '):
            lit = _escape_str(comment[7:].strip())
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('class '): sim.push(Lit('Class::<Object>::default()'), RsGeneric('Class', [RsNamed('Object')]))
        else: sim.push(Lit(f"{operand}i32"), I32)

    # ── null ──
    elif op == 'aconst_null': sim.push(Lit('Object::default()'), RsNamed('Object'))

    # ── load ──
    elif op.startswith('iload'): sim.push(*sim.load_local(_parse_slot(op, operand)))
    elif op.startswith('lload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, I64)
    elif op.startswith('fload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F32)
    elif op.startswith('dload'): e, _ = sim.load_local(_parse_slot(op, operand)); sim.push(e, F64)
    elif op.startswith('aload'): sim.push(*sim.load_local(_parse_slot(op, operand)))

    # ── store ──
    elif op.startswith('istore'):
        e, ty = sim.pop()
        # istore 在 JVM 中存储 int；bool 比较结果需要强制转换
        if getattr(ty, 'name', '') == 'bool':
            e = RawExpr(f"({render_expr(e)}) as i32")
            ty = I32
        sim.store_local(_parse_slot(op, operand), e, ty)
    elif op.startswith('lstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, I64)
    elif op.startswith('fstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F32)
    elif op.startswith('dstore'): e, _ = sim.pop(); sim.store_local(_parse_slot(op, operand), e, F64)
    elif op.startswith('astore'):
        e, ty = sim.pop()
        sim.store_local(_parse_slot(op, operand), e, ty)

    # ── 整数算术 ──
    elif op == 'iadd':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_add({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'isub':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_sub({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'imul':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_mul({_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'idiv':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}/{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'irem':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}%{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ineg':
        a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}).wrapping_neg()"), I32)
    elif op == 'ishl':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}<<({_to_i32(render_expr(b), bt)}&0x1f))"), I32)
    elif op == 'ishr':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}>>(({_to_i32(render_expr(b), bt)}&0x1f)))"), I32)
    elif op == 'iushr':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"(({_to_i32(render_expr(a), at)} as u32>>({_to_i32(render_expr(b), bt)}&0x1f)) as i32)"), I32)
    elif op == 'iand':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}&{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ior':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}|{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'ixor':
        b, bt = sim.pop(); a, at = sim.pop()
        sim.push(RawExpr(f"({_to_i32(render_expr(a), at)}^{_to_i32(render_expr(b), bt)})"), I32)
    elif op == 'iinc':
        parts = operand.replace(',', ' ').split()
        slot, delta = int(parts[0]), int(parts[1])
        name, _, _ = sim.locals.get(slot, (f"local_{slot}", I32, True))
        if delta >= 0: sim.emit(RawStmt(f"{name} = {name}.wrapping_add({delta}i32);"))
        else:          sim.emit(RawStmt(f"{name} = {name}.wrapping_sub({-delta}i32);"))
    elif op == 'ladd':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_add({b_s})"), I64)
    elif op == 'lsub':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_sub({b_s})"), I64)
    elif op == 'lmul':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}).wrapping_mul({b_s})"), I64)
    elif op == 'ldiv':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}/{b_s})"), I64)
    elif op == 'fadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F32)
    elif op == 'fsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F32)
    elif op == 'fmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F32)
    elif op == 'fdiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F32)
    elif op == 'dadd':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}+{render_expr(b)})"), F64)
    elif op == 'dsub':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}-{render_expr(b)})"), F64)
    elif op == 'dmul':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}*{render_expr(b)})"), F64)
    elif op == 'ddiv':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}/{render_expr(b)})"), F64)

    # ── long 算术（lrem/lneg/land/lor/lxor/lshl/lshr/lushr）──
    elif op == 'lrem':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}%({b_s}))"), I64)
    elif op == 'lneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_neg()"), I64)
    elif op == 'land':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}&({b_s}))"), I64)
    elif op == 'lor':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s}|({b_s}))"), I64)
    elif op == 'lxor':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"({a_s})^({b_s})"), I64)
    elif op == 'lshl':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_shl(({render_expr(b)}&0x3f) as u32)"), I64)
    elif op == 'lshr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}).wrapping_shr(({render_expr(b)}&0x3f) as u32)"), I64)
    elif op == 'lushr':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"(({render_expr(a)} as u64).wrapping_shr(({render_expr(b)}&0x3f) as u32) as i64)"), I64)

    # ── float/double 取余与取负 ──
    elif op == 'frem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}%({render_expr(b)}))"), F32)
    elif op == 'fneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"(-({render_expr(a)}))"), F32)
    elif op == 'drem':
        b, _ = sim.pop(); a, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(a)}%({render_expr(b)}))"), F64)
    elif op == 'dneg':
        a, _ = sim.pop()
        sim.push(RawExpr(f"(-({render_expr(a)}))"), F64)

    # ── 比较指令（lcmp/fcmpl/fcmpg/dcmpl/dcmpg）→ 压 i32 结果 ──
    elif op == 'lcmp':
        b, b_ty = sim.pop(); a, a_ty = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        if render_type(a_ty) != 'i64': a_s = f"({a_s} as i64)"
        if render_type(b_ty) != 'i64': b_s = f"({b_s} as i64)"
        sim.push(RawExpr(f"(({a_s}>({b_s})) as i32-(({a_s})<({b_s})) as i32)"), I32)
    elif op in ('fcmpl', 'fcmpg', 'dcmpl', 'dcmpg'):
        b, _ = sim.pop(); a, _ = sim.pop()
        a_s = render_expr(a); b_s = render_expr(b)
        sim.push(RawExpr(f"(({a_s}>({b_s})) as i32-(({a_s})<({b_s})) as i32)"), I32)

    # ── 类型转换 ──
    elif op == 'i2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'i2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'i2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    elif op == 'i2b': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as i8 as i32)"), I32)
    elif op == 'i2s': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as i16 as i32)"), I32)
    elif op == 'i2c': a, _ = sim.pop(); sim.push(RawExpr(f"(({render_expr(a)}) as u16 as i32)"), I32)
    elif op == 'l2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'l2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'l2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)
    elif op == 'f2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'f2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'd2i': a, _ = sim.pop(); sim.push(Cast(a, I32), I32)
    elif op == 'd2l': a, _ = sim.pop(); sim.push(Cast(a, I64), I64)
    elif op == 'd2f': a, _ = sim.pop(); sim.push(Cast(a, F32), F32)
    elif op == 'f2d': a, _ = sim.pop(); sim.push(Cast(a, F64), F64)

    # ── dup / pop / swap ──
    elif op == 'dup':
        if sim.stack: sim.stack.append(sim.stack[-1])
    elif op == 'dup_x1':
        if len(sim.stack) >= 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'dup2':
        # 简化：对 category-1 复制前两项；category-2（long/double）复制栈顶一项
        if len(sim.stack) >= 2:
            v1 = sim.stack[-1]; v2 = sim.stack[-2]
            sim.stack += [v2, v1]
        elif sim.stack:
            sim.stack.append(sim.stack[-1])
    elif op == 'dup_x2':
        if len(sim.stack) >= 3:
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v1, v3, v2, v1]
        elif len(sim.stack) == 2:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            sim.stack += [v1, v2, v1]
    elif op == 'dup2_x1':
        if len(sim.stack) >= 3:
            v1 = sim.stack.pop(); v2 = sim.stack.pop(); v3 = sim.stack.pop()
            sim.stack += [v2, v1, v3, v2, v1]
    elif op == 'dup2_x2':
        if len(sim.stack) >= 4:
            v1 = sim.stack.pop(); v2 = sim.stack.pop()
            v3 = sim.stack.pop(); v4 = sim.stack.pop()
            sim.stack += [v2, v1, v4, v3, v2, v1]
    elif op == 'swap':
        if len(sim.stack) >= 2:
            sim.stack[-1], sim.stack[-2] = sim.stack[-2], sim.stack[-1]
    elif op == 'pop':
        if sim.stack:
            e_expr, _ = sim.pop()
            e = render_expr(e_expr)
            # 只有在弹出的是有副作用的表达式时才发出 let _ = ...
            if any(c in e for c in ['(', 'push', 'insert']):
                sim.emit(RawStmt(f"let _ = {e};"))
    elif op == 'pop2':
        sim.pop()
        if sim.stack: sim.pop()

    # ── 对象创建 ──
    elif op == 'new':
        raw = (comment or operand).strip()
        if raw.startswith('class '): raw = raw[6:]
        sim.push(NewPendingExpr(raw), RsNamed(raw.split('/')[-1]))

    # ── invokespecial（含构造器）──
    elif op == 'invokespecial':
        _gen_invokespecial(sim, comment, class_name, registry=registry)

    # ── 字段访问 ──
    elif op == 'getfield':
        obj_expr, obj_ty = sim.pop()
        if comment:
            _, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'Object'
            sim.push(RawExpr(f"{render_expr(obj_expr)}.{fname}.get()"), RsNamed(ftype))
        else:
            sim.push(RawExpr(f"{render_expr(obj_expr)}.field"), I32)

    elif op == 'putfield':
        val_expr, val_ty = sim.pop()
        obj_expr, obj_ty = sim.pop()
        if comment:
            _, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'i32'
            val_str_raw = render_expr(val_expr)
            val_ty_name = render_type(val_ty)
            # null 值（aconst_null → Object::default()）赋给具体类型字段时用 Default::default()
            null_coerce = _coerce_from_null(val_str_raw, ftype)
            if null_coerce is not None:
                val_str = null_coerce
            elif ftype == 'Object' and val_ty_name not in ('Object', '()') and val_str_raw != 'this':
                val_str = _coerce_to_object(val_str_raw, val_ty_name)
            else:
                val_str = _coerce_value(val_str_raw, val_ty, ftype)
            # 引用类型赋值时加 .clone()，避免 E0382（move after use）
            if (val_ty_name not in _PRIMITIVE_RUST_TYPES
                    and not val_str.startswith('Default::')
                    and '.clone()' not in val_str
                    and val_str != 'this'):
                val_str = f'{val_str}.clone()'
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.{fname}.set({val_str});"))
        else:
            sim.emit(RawStmt(f"/* putfield {render_expr(val_expr)} */"))

    elif op == 'getstatic':
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if field_name == '$assertionsDisabled':
            # 合成字段：断言控制标志，始终视为已禁用（= true），等价于以 -da 运行 JVM
            sim.push(Lit('true'), BOOL)
        elif field_name:
            ty_str = jvm_to_rust(descriptor, registry) if descriptor else 'Object'
            sim.push(StaticFieldRef(cls, field_name, RsNamed(ty_str)), RsNamed(ty_str))
        else:
            sim.push(RawExpr(f"/* getstatic {comment} */"), RsNamed('Object'))
    elif op == 'putstatic':
        val_expr, _ = sim.pop()
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        # putstatic: 静态字段写入用注释占位，stub getter 已生成 panic!() 实现
        sim.emit(RawStmt(f"/* putstatic {cls}.{field_name} = {render_expr(val_expr)} */"))

    # ── 数组 ──
    elif op == 'newarray':
        count_expr, _ = sim.pop()
        elem_t, zero = NEWARRAY_TYPES.get(operand.strip(), ('i32', '0i32'))
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Rc<RefCell<Vec<{elem_t}>>> = Rc::new(RefCell::new(vec![{zero}; {render_expr(count_expr)} as usize]));"))
        sim.push(Var(v), RsNamed(f'Rc<RefCell<Vec<{elem_t}>>>'))
    elif op == 'anewarray':
        count_expr, _ = sim.pop()
        cls = short_cls(comment) or 'Object'
        elem_t = jvm_to_rust(f'L{cls};') if cls != 'Object' else 'Object'
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Rc<RefCell<Vec<{elem_t}>>> = Rc::new(RefCell::new(Vec::with_capacity({render_expr(count_expr)} as usize)));"))
        sim.push(Var(v), RsNamed(f'Rc<RefCell<Vec<{elem_t}>>>'))
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [render_expr(sim.pop()[0]) for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];"))
        sim.push(Var(v), RsGeneric('Vec', [RsGeneric('Vec', [I32])]))
    elif op in ('iastore', 'lastore', 'fastore', 'dastore'):
        val_expr, _ = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = {render_expr(val_expr)};"))
    elif op == 'aastore':
        val_expr, val_ty = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        elem_ty = arr_ty_str[4:-1] if arr_ty_str.startswith('Vec<') else 'Object'
        val_str = render_expr(val_expr)
        val_ty_str = render_type(val_ty)
        if elem_ty == 'Object' and val_ty_str not in ('Object', '()'):
            val_str = _coerce_to_object(val_str, val_ty_str)
        elif elem_ty != 'Object' and val_ty_str == 'Object':
            val_str = f"Default::default()"
        elif val_ty_str not in _PRIMITIVE_RUST_TYPES:
            val_str = f"{val_str}.clone()"
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = {val_str};"))
    elif op == 'bastore':
        val_expr, _ = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = ({render_expr(val_expr)}) as i8;"))
    elif op == 'sastore':
        val_expr, _ = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = ({render_expr(val_expr)}) as i16;"))
    elif op == 'castore':
        val_expr, _ = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = ({render_expr(val_expr)}) as u16;"))
    elif op == 'iaload':
        idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"{render_expr(arr_expr)}.borrow()[{render_expr(idx_expr)} as usize]"), I32)
    elif op in ('baload', 'saload', 'caload'):
        idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(arr_expr)}.borrow()[{render_expr(idx_expr)} as usize] as i32)"), I32)
    elif op in ('laload', 'faload', 'daload'):
        idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        ty = {'l': I64, 'f': F32, 'd': F64}.get(op[0], I32)
        sim.push(RawExpr(f"{render_expr(arr_expr)}.borrow()[{render_expr(idx_expr)} as usize]"), ty)
    elif op == 'aaload':
        idx_expr, _ = sim.pop(); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        elem_ty_str = arr_ty_str[4:-1] if arr_ty_str.startswith('Vec<') else 'Object'
        sim.push(RawExpr(f"{render_expr(arr_expr)}.borrow()[{render_expr(idx_expr)} as usize].clone()"), RsNamed(elem_ty_str))
    elif op == 'arraylength':
        arr_expr, _ = sim.pop()
        sim.push(RawExpr(f"({render_expr(arr_expr)}.borrow().len() as i32)"), I32)

    # ── 方法调用 ──
    elif op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name, registry=registry)
    elif op in ('invokevirtual', 'invokeinterface'):
        _gen_invokevirtual(sim, comment, class_name, registry=registry)

    # ── 返回 ──
    elif op == 'return':
        # 构造函数 return 指令：返回 Ok(this) 而非 Ok(())
        if sim.is_constructor:
            sim.emit(RawStmt('return Ok(this);'))
        else:
            sim.emit(RawStmt('return Ok(());'))
    elif op in ('ireturn', 'lreturn', 'freturn', 'dreturn'):
        e_expr, e_ty = sim.pop()
        expr_s = render_expr(e_expr)
        # 若返回类型与栈类型不匹配（窄类型/bool→i32），做显式转换
        ret_ty = getattr(sim, 'return_type', 'i32')
        actual_ty = render_type(e_ty)
        if actual_ty != ret_ty and ret_ty in ('i8', 'i16', 'u16', 'bool', 'i32'):
            expr_s = _coerce_value(expr_s, e_ty, ret_ty)
        sim.emit(RawStmt(f"return Ok({expr_s});"))
    elif op == 'areturn':
        e_expr, e_ty = sim.pop()
        expr_s = render_expr(e_expr)
        actual_ty = render_type(e_ty)
        ret_ty = getattr(sim, 'return_type', 'Object')
        _ctparams = getattr(sim, 'class_type_params', frozenset())
        # 实例方法返回 this 时，this 是 &Self 引用，需要 clone() 才能返回 owned 值
        if expr_s == 'this' and not sim.is_static:
            expr_s = 'this.clone()'
        elif ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            expr_s = _coerce_to_object(expr_s, actual_ty)
        elif ret_ty != 'Object' and actual_ty == 'Object':
            # 泛型类型参数（如 T、K、V、E）不实现 Default，用 panic!("null") 代替
            if ret_ty in _ctparams:
                expr_s = 'panic!("null")'
            else:
                expr_s = f"Default::default()"
        sim.emit(RawStmt(f"return Ok({expr_s});"))

    # ── 控制流（循环由 method.py 处理，此处跳过）──
    elif op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        pass

    # ── checkcast / instanceof ──
    elif op == 'checkcast':
        # 更新栈顶类型为 cast 目标类型；若源类型为 Object，插入运行时 downcast
        if comment and sim.stack:
            if comment.startswith('['):
                cast_rust = jvm_to_rust(comment, registry)
            else:
                cast_rust = jvm_to_rust(f'L{comment};', registry)
            expr, src_ty = sim.pop()
            src_name = getattr(src_ty, 'name', str(src_ty))
            if src_name == 'Object' and cast_rust not in ('Object', '()'):
                expr = RawExpr(f"({render_expr(expr)}).downcast::<{cast_rust}>()")
            sim.push(expr, RsNamed(cast_rust))
    elif op == 'instanceof': sim.push(Lit('true'), BOOL)

    # ── invokedynamic ──
    elif op == 'invokedynamic':
        if comment and 'makeConcatWithConstants' in comment:
            _gen_string_concat(sim, comment)
        else:
            sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))

    # ── 同步（忽略，不支持多线程语义）──
    elif op == 'monitorenter':
        sim.pop()  # pop object reference，忽略 monitor
    elif op == 'monitorexit':
        sim.pop()  # pop object reference，忽略 monitor

    # ── switch（弹出 key，线性继续，不跳转）──
    elif op in ('tableswitch', 'lookupswitch'):
        key, _ = sim.pop()
        key_s = render_expr(key)
        sim.emit(RawStmt(f"let _switch_key = {key_s};"))

    # ── 杂项 ──
    elif op in ('nop', 'wide'): pass
    elif op == 'athrow':
        e_expr, _ = sim.pop()
        # "athrow".to_owned() 使用 std::string::String，避免与 java_runtime::String 遮蔽冲突
        sim.emit(RawStmt(f'return Err(JvmError::Custom("athrow".to_owned()));'))
    else:
        sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))


# ── invoke 生成器 ─────────────────────────────────────────────────

def _gen_string_concat(sim: StackSim, comment: str):
    """处理 invokedynamic makeConcatWithConstants 字符串拼接。
    结果为 java.lang.String（通过 String::from(format!(...)) 转换）。
    """
    desc_m = re.search(r'makeConcatWithConstants:(\([^)]*\))', comment)
    desc = desc_m.group(1) + 'Ljava/lang/String;' if desc_m else '(Ljava/lang/String;)Ljava/lang/String;'
    params = parse_descriptor_params(desc)

    args = []
    for _ in range(len(params)):
        e_expr, _ = sim.pop()
        args.insert(0, render_expr(e_expr))

    tmpl_m = re.search(r' template:(.+)$', comment)
    if tmpl_m:
        template = tmpl_m.group(1)
        parts = template.split('\x01')
        if len(parts) == len(args) + 1:
            fmt_str = ''.join(
                (p.replace('{', '{{').replace('}', '}}') + '{}' if i < len(args)
                 else p.replace('{', '{{').replace('}', '}}'))
                for i, p in enumerate(parts)
            )
            fmt_args = ', '.join(args)
            if fmt_args:
                # 用 from_owned 避免 From<&str> vs From<std::string::String> 歧义
                sim.push(Lit(f'String::from_owned(format!("{fmt_str}", {fmt_args}))'), RsNamed('String'))
            else:
                sim.push(Lit(f'String::from("{fmt_str}")'), RsNamed('String'))
            return

    # fallback
    if not args:
        sim.push(Lit('String::new()'), RsNamed('String'))
    elif len(args) == 1:
        sim.push(Lit(f'String::from_owned(format!("{{}}", {args[0]}))'), RsNamed('String'))
    else:
        fmt = '{}'.join([''] * (len(args) + 1))  # "{}{}{}" for 3 args
        fmt_args = ', '.join(args)
        sim.push(Lit(f'String::from_owned(format!("{fmt}", {fmt_args}))'), RsNamed('String'))


def _gen_invokespecial(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    if '<init>' not in comment and '"<init>"' not in comment:
        # super.method() 调用：用 invokevirtual 语义（弹出 receiver + args）
        _gen_invokevirtual(sim, comment, class_name, registry=registry)
        return

    cls, _, params, _ = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, e_ty_node = sim.pop()
        e = render_expr(e_expr)
        expected = jvm_to_rust(param_jvm, registry)
        ty = render_type(e_ty_node)
        null_coerce = _coerce_from_null(e, expected)
        if null_coerce is not None:
            e = null_coerce
        elif _coerce_to_interface(ty, expected):
            e = 'Default::default()'
        elif expected == 'Object' and ty not in ('Object', '()') and e != 'this':
            e = _coerce_to_object(e, ty)
        elif expected == 'Object' and ty not in ('Object', '()') and e == 'this':
            e = f"Object::from_any(self.clone())"
        elif expected in ('bool', 'i8', 'i16', 'u16') and ty != expected:
            e = _coerce_value(e, e_ty_node, expected)
        elif expected == 'i32' and ty in ('i8', 'i16', 'u16', 'bool'):
            e = f"({e} as i32)"
        elif ty not in _PRIMITIVE_RUST_TYPES:
            e = f"{e}.clone()"
        args.insert(0, e)
    obj_expr, obj_ty_node = sim.pop()

    if isinstance(obj_expr, NewPendingExpr):
        full_cls = obj_expr.class_name          # e.g. 'java/util/ArrayList'
        raw_cls = full_cls.rsplit('/', 1)[-1]
        raw_cls = short_cls(raw_cls) or raw_cls

        if '/' in full_cls:
            # JDK class（含包路径）→ 用 new() 工厂（@synthetic）
            rust_ty_str = jvm_to_rust(f'L{full_cls};', registry)
            if rust_ty_str != 'Object' and '<' in rust_ty_str:
                type_params_str = rust_ty_str[len(raw_cls):]   # '<Object>' / '<Object, Object>'
            else:
                type_params_str = ''
            rust_ty = raw_cls + type_params_str
            rust_ty_node = RsNamed(rust_ty)
            # 重载构造器：用 '<init>' 查重载再替换为 'new'，以匹配 method.py 生成的定义
            _init_mangled = _mangle_if_overloaded(full_cls, '<init>', comment, registry)
            ctor_name = _safe_field(_init_mangled.replace('<init>', 'new'))
            if args:
                init_expr = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
            else:
                turbofish = '::' + type_params_str if type_params_str else ''
                init_expr = f"{raw_cls}{turbofish}::{ctor_name}()?"
        elif raw_cls and '/' not in raw_cls:
            # 用户类：new()? 返回 Result<Self>，同样 mangle 重载构造器
            _init_mangled2 = _mangle_if_overloaded(raw_cls, '<init>', comment, registry)
            ctor_name    = _safe_field(_init_mangled2.replace('<init>', 'new'))
            init_expr    = f"{raw_cls}::{ctor_name}({', '.join(args)})?"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)
        else:
            init_expr    = f"/* {raw_cls}::new() */"
            rust_ty      = raw_cls
            rust_ty_node = RsNamed(rust_ty)

        if sim.stack and isinstance(sim.stack[-1][0], NewPendingExpr):
            sim.stack[-1] = (RawExpr(init_expr), rust_ty_node)
        else:
            v = sim.fresh('_obj')
            sim.emit(RawStmt(f"let mut {v}: {rust_ty} = {init_expr};"))
            sim.push(Var(v), rust_ty_node)
    else:
        sim.emit(RawStmt(f"/* invokespecial {comment} */"))


def _class_known(cls_short: str, registry: dict | None) -> bool:
    """判断短类名是否已知（registry 中存在或是 java_runtime 手写类）。
    T71：比较时统一将 $ 替换为 _，避免 JVM 格式（Outer$Inner）与 Rust 格式（Outer_Inner）不一致。"""
    if not registry:
        return True
    if not cls_short or cls_short in _JAVA_RUNTIME_SHORT_NAMES:
        return True
    if cls_short in registry:
        return True
    norm = cls_short.replace('$', '_')
    for key in registry:
        if key.rsplit('/', 1)[-1].replace('$', '_') == norm:
            return True
    return False


_JAVA_RUST_NAME_CONFLICTS = frozenset({'clone'})
_JAVA_RUST_RENAME = {'clone': 'jvm_clone'}


def _mangle_if_overloaded(cls_name: str, mname: str, comment: str, registry: dict | None) -> str:
    """查找 registry 中 cls_name 类的 mname 方法是否重载，重载则返回 mangled 名，否则原名。
    支持短名（Objects）和全路径名（java/util/Objects）查找。
    java_runtime 手写类（ArrayList/Object 等）不做 mangle，其 API 已固定。"""
    if not registry or not mname or (mname.startswith('<') and mname != '<init>'):
        return mname
    # java_runtime 手写类直接跳过 mangle（其 API 已固定，不走 jdk_classes 重命名逻辑）
    short = cls_name.rsplit('/', 1)[-1]
    if short in _JAVA_RUNTIME_SHORT_NAMES:
        return mname
    # 直接查（可能是全路径）
    target_ci = registry.get(cls_name)
    # 短名查（用全路径反查）；T71：统一 $ → _ 后比较
    if target_ci is None and '/' not in cls_name:
        norm = cls_name.replace('$', '_')
        for key, ci in registry.items():
            if key.rsplit('/', 1)[-1].replace('$', '_') == norm:
                target_ci = ci
                break
    if target_ci is None:
        return _JAVA_RUST_RENAME.get(mname, mname)
    # 排除 java_runtime 类（registry 中仍有其 JDK 字节码副本，但方法名不 mangle）
    if target_ci.name.rsplit('/', 1)[-1] in _JAVA_RUNTIME_SHORT_NAMES:
        return mname
    visible = [m for m in target_ci.methods if not m.is_synthetic]
    same = sum(1 for m in visible if m.name == mname)
    if same <= 1:
        # 非重载方法：检查 T39 ergonomic @jvm_rename 指令
        erg_rename = get_ergonomic_jvm_rename(target_ci.name, mname)
        if erg_rename is not None:
            return erg_rename
        # Java→Rust 名字冲突重命名（如 clone→jvm_clone）
        erg_name = _JAVA_RUST_RENAME.get(mname)
        if erg_name is not None:
            return erg_name
        return mname
    desc_m = re.search(r':(\([^)]*\)\S+)', comment)
    raw_desc = desc_m.group(1) if desc_m else ''
    result = mangle_name(mname, raw_desc) if raw_desc else mname
    # 重载后的名字若与 Rust 原生名字冲突也需重命名
    return _JAVA_RUST_RENAME.get(result, result)


def _gen_invokestatic(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    for skip in BOXING_SKIP_STATIC:
        if skip in comment:
            return  # 自动装箱：栈顶值保留

    if 'String.valueOf' in comment:
        a_expr, _ = sim.pop()
        a = render_expr(a_expr)
        sim.push(Lit(f'String::from_owned(format!("{{}}", {a}))'), RsNamed('String'))
        return

    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, ty_node = sim.pop()
        e = render_expr(e_expr)
        ty = render_type(ty_node)
        expected = jvm_to_rust(param_jvm, registry)
        null_coerce = _coerce_from_null(e, expected)
        if null_coerce is not None:
            e = null_coerce
        elif _coerce_to_interface(ty, expected):
            e = 'Default::default()'
        elif expected == 'Object' and ty not in ('Object', '()'):
            e = _coerce_to_object(e, ty)
        elif expected in ('bool', 'i8', 'i16', 'u16') and ty != expected:
            e = _coerce_value(e, ty_node, expected)
        elif expected == 'i32' and ty in ('i8', 'i16', 'u16', 'bool'):
            e = f"({e} as i32)"
        elif ty not in _PRIMITIVE_RUST_TYPES:
            e = f"{e}.clone()"
        args.insert(0, e)

    needs_q = False  # 是否加 ?（用户类方法返回 Result）

    # 目标类不在 registry（被截断的内部类如 jdk.internal.*）→ 生成 panic 存根
    if cls and not _class_known(cls, registry):
        rust_ret = jvm_to_rust(ret, registry)
        stub_msg = f"stub: {cls}.{mname}"
        if rust_ret == '()':
            sim.emit(RawStmt(f'panic!("{stub_msg}");'))
        else:
            v = sim.fresh()
            sim.emit(RawStmt(f'let {v}: {rust_ret} = panic!("{stub_msg}");'))
            sim.push(Var(v), RsNamed(rust_ret))
        return

    if cls is None or cls == class_name:
        rust_mname = _safe_field(_mangle_if_overloaded(class_name, mname, comment, registry))
        call = f"Self::{rust_mname}({', '.join(args)})"
        needs_q = True
    elif cls and '/' in cls:
        call = f"/* {cls}.{mname}({', '.join(args)}) */"
    else:
        rust_mname = _safe_field(_mangle_if_overloaded(cls, mname, comment, registry))
        call = f"{cls}::{rust_mname}({', '.join(args)})"
        needs_q = True

    q = '?' if needs_q else ''
    rust_ret = jvm_to_rust(ret, registry)
    if rust_ret == '()':
        sim.emit(RawStmt(f"{call}{q};"))
    else:
        v = sim.fresh()
        sim.emit(RawStmt(f"let {v}: {rust_ret} = {call}{q};"))
        sim.push(Var(v), RsNamed(rust_ret))


# Java 中任何对象都可以传递给 Object 参数（引用协变），Rust 需要显式 Into<Object> 转换
_PRIMITIVE_RUST_TYPES: frozenset[str] = frozenset({
    'i32', 'i64', 'f32', 'f64', 'bool', 'i8', 'i16', 'u16', '()'
})


def _gen_invokevirtual(sim: StackSim, comment: str, class_name: str, registry: dict | None = None):
    cls, mname, params, ret = parse_method_ref(comment)
    args = []
    for param_jvm in reversed(params):
        e_expr, e_ty_node = sim.pop()
        e_str = render_expr(e_expr)
        expected_rust = jvm_to_rust(param_jvm, registry)
        actual_rust = render_type(e_ty_node)
        null_coerce = _coerce_from_null(e_str, expected_rust)
        if null_coerce is not None:
            e_str = null_coerce
        elif _coerce_to_interface(actual_rust, expected_rust):
            e_str = 'Default::default()'
        elif expected_rust == 'Object' and actual_rust not in ('Object', '()'):
            e_str = _coerce_to_object(e_str, actual_rust)
        elif expected_rust in ('bool', 'i8', 'i16', 'u16') and actual_rust != expected_rust:
            e_str = _coerce_value(e_str, e_ty_node, expected_rust)
        elif expected_rust == 'i32' and actual_rust in ('i8', 'i16', 'u16'):
            e_str = f"({e_str} as i32)"
        elif actual_rust not in _PRIMITIVE_RUST_TYPES:
            e_str = f"{e_str}.clone()"
        args.insert(0, e_str)
    obj_expr, obj_ty_node = sim.pop()
    obj_e = render_expr(obj_expr)
    obj_ty = render_type(obj_ty_node)

    # 拆箱：identity
    if mname in UNBOX_VIRTUAL:
        sim.push(obj_expr, obj_ty_node)
        return

    # T38：PrintStream.println 有参版本 → 统一生成 println_v(x)（Printable trait 派发）
    # 注：无参 println() 保持原名；println_v<T: Printable> 处理所有有参版本
    if mname == 'println' and cls and cls.endswith('PrintStream') and len(args) == 1:
        sim.emit(RawStmt(f"{obj_e}.println_v({args[0]})?;"))
        return

    # 若接收方 Rust 类型是 java_runtime 手写类，不做 mangle
    obj_base = obj_ty.split('<')[0].strip()  # 去泛型后缀（ArrayList<T> → ArrayList）
    if obj_base in _JAVA_RUNTIME_SHORT_NAMES:
        rust_mname = _safe_field(mname)
    else:
        # 查 registry 确认是否有重载（用户类 + JDK 类均检查），有则 mangle 调用名
        rust_mname = _safe_field(_mangle_if_overloaded(cls or '', mname, comment, registry))

    # 所有方法统一处理：obj.method(args)?（用户类 + JDK 类均走此路径）
    arg_str = ', '.join(args)
    rust_ret = jvm_to_rust(ret, registry)
    # E0599 防护：接收者是 Object 类型时，Object 结构体不定义具体子类方法，
    # 直接调用会产生 E0599。对 void 返回跳过调用，对非 void 用 Default::default()。
    obj_is_bare = (obj_ty == 'Object')
    if rust_ret == '()':
        if not obj_is_bare:
            sim.emit(RawStmt(f"{obj_e}.{rust_mname}({arg_str})?;"))
    else:
        v = sim.fresh()
        if obj_is_bare and rust_ret not in ('Object', '()') and rust_ret not in _PRIMITIVE_RUST_TYPES:
            sim.emit(RawStmt(f"let {v}: {rust_ret} = Default::default();"))
        else:
            sim.emit(RawStmt(f"let {v} = {obj_e}.{rust_mname}({arg_str})?;"))
        sim.push(Var(v), RsNamed(rust_ret))



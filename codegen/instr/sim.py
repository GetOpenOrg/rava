"""
sim_instr：JVM 字节码指令 → Rust 语句转换主分发函数（大 elif 链）。
"""

from ..stack import StackSim, I32, I64, F32, F64, BOOL, UNIT
from ..rs_ir import (
    Lit, Var, BinOp, UnOp, Call, MethodCall, FieldAccess, Index,
    Cast, RawExpr, RawStmt, LetStmt, AssignStmt, ExprStmt, ReturnStmt,
    IfStmt, LoopStmt, BreakStmt, RsNamed, RsGeneric, RsRef, RsSlice, RsInfer,
    NewPendingExpr, StaticFieldRef,
)
from ..render import render_expr, render_type
from ..type_map import (
    jvm_to_rust, short_cls,
    NEWARRAY_TYPES,
    parse_descriptor_params, parse_descriptor_return,
    parse_class_type_params as _parse_class_type_params,
    parse_field_type as _parse_field_type,
)
from ..constants import safe_ident as _safe_ident
from ..types import Instr
from .coerce import (
    _float_lit, _escape_str, _parse_slot, _to_i32,
    _coerce_to_object, _coerce_from_null, _coerce_value,
    _parse_field_ref, _is_subtype, _rust_type_to_binary,
    _get_field_generic_signature, _has_subtypes, _get_all_subtypes_ordered,
    _PRIMITIVE_RUST_TYPES, _into_super_chain,
)
from .invoke import (
    _gen_invokespecial, _gen_invokestatic, _gen_invokevirtual, _gen_string_concat,
)


def _restore_field_declared_type(f_owner: str, fname: str, ftype: str,
                                 class_name: str, registry: dict | None,
                                 sim: 'StackSim') -> str:
    """按字段 generic_signature 恢复声明类型（getfield / putfield 共用）。

    ftype 是 jvm_to_rust(fdesc) 的擦除形态；沿继承链查字段声明，用声明类
    tparams 上下文解析为精确泛型形态（HashMap_Node<Object,Object> →
    HashMap_Node<K,V>、Rc<RefCell<Vec<Object>>> → Rc<RefCell<Vec<E>>>）。
    解析结果中的类型名在调用方不可见（跨类参数名不同）时保持擦除形态。
    """
    if not registry:
        return ftype
    _g_owner = f_owner if f_owner else class_name
    if not _g_owner:
        return ftype
    _g_ci = registry.get(_g_owner)
    _gsig = _get_field_generic_signature(_g_owner, fname, registry)
    if not (_gsig and _g_ci is not None):
        return ftype
    _decl_tparams = (_parse_class_type_params(_g_ci.generic_signature)
                     if _g_ci.generic_signature else [])
    _parsed = _parse_field_type(_gsig, _decl_tparams, registry)
    if _parsed and _parsed != 'Object' and _parsed != ftype:
        # 校验：解析结果中的类型名须在调用方可见
        # （当前 impl 类型参数 / registry 短名 / 内建容器），
        # 跨类不可见（声明类参数名与调用方不同）时降级回擦除形态
        import re as _re_g
        _caller_tparams = set(sim.class_type_params) if sim.class_type_params else set()
        _reg_shorts = {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
        _builtin_g = {'Object', 'String', 'Rc', 'Vec', 'RefCell'}
        if all(_n in _caller_tparams or _n in _reg_shorts or _n in _builtin_g
               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _parsed)):
            return _parsed
    return ftype


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
            lit = _escape_str(comment[7:].rstrip('\n'))
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('int '):    sim.push(Lit(comment[4:].strip() + 'i32'), I32)
        elif comment.startswith('float '): sim.push(Lit(_float_lit(comment[6:].strip(), 'f32')), F32)
        elif comment.startswith('long '):  sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('class '): sim.push(Lit('Object::default()'), RsNamed('Object'))
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op in ('ldc2_w', 'ldc_w'):
        if comment.startswith('long '):   sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('String '):
            lit = _escape_str(comment[7:].rstrip('\n'))
            sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('class '): sim.push(Lit('Object::default()'), RsNamed('Object'))
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
            f_owner, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'Object'
            # 装箱类擦除特例：Integer/Long/Double/Boolean 的签名类型被 type_map
            # 擦除为基本类型（i32/i64/f64/bool）。当 getfield 的接收者已经是
            # 基本类型时（如 compareTo 的 anotherInteger: i32），字段访问就是
            # 值本身 —— 不能生成 __get_value()（基本类型上无此方法，E0599）。
            # 接收者是装箱 struct（如 this: &Integer）时仍走正常访问器。
            _recv_ty = render_type(obj_ty)
            if _recv_ty in _PRIMITIVE_RUST_TYPES:
                sim.push(obj_expr, RsNamed(_recv_ty))
                return
            # 字段声明类型恢复：struct 字段生成（class_writer._resolve_field_rust）
            # 优先字段级 generic_signature（如 interfaces: Vec<Class<Object>>、
            # parent: HashMap_TreeNode<K, V>），宏访问器 __get_xxx() 按声明类型返回。
            # 读取侧必须记录同一类型：否则 sim 记录擦除形态（Object /
            # X<Object,Object> / Rc<RefCell<Vec<Object>>>）而表达式实际是
            # 精确泛型形态，局部变量标注 E0308（expected 擦除, found 精确）。
            if registry:
                _g_owner = f_owner if f_owner else class_name
                _g_ci = registry.get(_g_owner) if _g_owner else None
                _gsig = _get_field_generic_signature(_g_owner, fname, registry) if _g_owner else None
                if _gsig and _g_ci is not None:
                    # 用声明类的类型参数解析（签名中的类型变量属于声明类上下文）
                    _decl_tparams = (_parse_class_type_params(_g_ci.generic_signature)
                                     if _g_ci.generic_signature else [])
                    _parsed = _parse_field_type(_gsig, _decl_tparams, registry)
                    if _parsed and _parsed != 'Object' and _parsed != ftype:
                        # 校验：解析结果中的类型名须在调用方可见
                        # （当前 impl 类型参数 / registry 短名 / 内建容器），
                        # 跨类不可见（声明类参数名与调用方不同）时降级回擦除形态
                        import re as _re_g
                        _caller_tparams = set(sim.class_type_params) if sim.class_type_params else set()
                        _reg_shorts = {_k.rsplit('/', 1)[-1].replace('$', '_') for _k in registry}
                        _builtin_g = {'Object', 'String', 'Rc', 'Vec', 'RefCell'}
                        if all(_n in _caller_tparams or _n in _reg_shorts or _n in _builtin_g
                               for _n in _re_g.findall(r'[A-Za-z_][A-Za-z0-9_]*', _parsed)):
                            ftype = _parsed
            # 字段读取 → 宏生成的访问器（方案 §7）。
            # 继承字段由子类的转发访问器统一暴露（父类字段在前展平，§6），
            # 所以不再需要按接收者静态类型拼 `_super._super.` 路径——
            # 那层复杂度已收拢进宏（见方案 §16：_super 语义边界）。
            sim.push(RawExpr(f"{render_expr(obj_expr)}.__get_{fname}()"), RsNamed(ftype))
        else:
            sim.push(RawExpr(f"{render_expr(obj_expr)}.field"), I32)

    elif op == 'putfield':
        val_expr, val_ty = sim.pop()
        obj_expr, obj_ty = sim.pop()
        if comment:
            cls_owner, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'i32'
            # Fix 15：字段声明类型恢复（与 getfield 对齐，原数组特例泛化）——
            # 擦除形态（HashMap_Node<Object,Object> / Vec<Object>）恢复为声明
            # 类 tparams 上下文的精确泛型形态（HashMap_Node<K,V> / Vec<E>），
            # 与宏访问器 __set_xxx 的参数类型（class_writer 按字段 generic_signature
            # 生成）一致，否则 E0308/E0277（Into 目标是擦除形态、From 不存在）。
            ftype = _restore_field_declared_type(cls_owner, fname, ftype, class_name, registry, sim)
            val_str_raw = render_expr(val_expr)
            val_ty_name = render_type(val_ty)
            # Vec<Object>(擦除) ↔ Vec<E>(泛型)：当 ftype 是参数化 Vec 而 val 是擦除 Vec 时，
            # 将 val 的 downcast 目标类型替换为泛型版本，使字段赋值类型一致
            if (ftype != val_ty_name
                    and 'Vec<' in ftype and 'Vec<Object>' in val_ty_name
                    and isinstance(val_expr, RawExpr)
                    and 'downcast::<Rc<RefCell<Vec<Object>>>>' in val_str_raw):
                val_str_raw = val_str_raw.replace(
                    'downcast::<Rc<RefCell<Vec<Object>>>>',
                    f'downcast::<{ftype}>'
                )
                val_ty_name = ftype
            # null 值（aconst_null → Object::default()）赋给具体类型字段时用 Default::default()
            null_coerce = _coerce_from_null(val_str_raw, ftype)
            if null_coerce is not None:
                val_str = null_coerce
            elif ftype == 'Object' and val_ty_name not in ('Object', '()') and val_str_raw != 'this':
                # 若值的类型是泛型参数（单大写字母如 T/E/K/V），字段槽位类型就是该参数本身，
                # 直接赋值（不再有 JField<T> 包装）
                if len(val_ty_name) <= 2 and val_ty_name[0].isupper() and val_ty_name.rstrip('0123456789').isalpha():
                    val_str = val_str_raw
                else:
                    val_str = _coerce_to_object(val_str_raw, val_ty_name)
            elif (ftype not in _PRIMITIVE_RUST_TYPES and val_ty_name not in _PRIMITIVE_RUST_TYPES
                  and ftype not in ('Object', '()', val_ty_name)
                  and _is_subtype(val_ty_name.split('<')[0], ftype.split('<')[0], registry)):
                # R-2：子类型赋给祖先类型字段，用显式 __into_super() 链（替代已删除的 T55 From impl）
                chain = _into_super_chain(val_ty_name.split('<')[0], ftype.split('<')[0], registry)
                val_str = f"{val_str_raw}{chain}"
            else:
                val_str = _coerce_value(val_str_raw, val_ty, ftype)
            # 引用类型赋值时加 Clone::clone()，避免 E0382（move after use）
            _obj_str = render_expr(obj_expr)
            if (val_ty_name not in _PRIMITIVE_RUST_TYPES
                    and not val_str.startswith('Default::')
                    and '.clone()' not in val_str
                    and 'Clone::clone(' not in val_str):
                if val_str == 'this' and 'this' in _obj_str:
                    val_str = 'Clone::clone(&this)'
                elif val_str != 'this':
                    val_str = f'Clone::clone(&{val_str})'
            # 字段写入 → 宏生成的 `__set_xxx` 访问器（方案 §7）。
            # 继承字段同样由转发访问器承接，无需 `_super` 前缀路径（§16）。
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.__set_{fname}({val_str});"))
        else:
            sim.emit(RawStmt(f"/* putfield {render_expr(val_expr)} */"))

    elif op == 'getstatic':
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if field_name == '$assertionsDisabled':
            # 合成字段：断言控制标志，始终视为已禁用（= true），等价于以 -da 运行 JVM
            sim.push(Lit('true'), BOOL)
        elif field_name:
            ty_str = jvm_to_rust(descriptor, registry) if descriptor else 'Object'
            # 泛型类静态字段访问需要 turbofish，避免 E0283 类型推断歧义
            _getstatic_turbofish = ''
            _getstatic_cls_ci = None
            if registry and cls:
                _cls_bin = _rust_type_to_binary(cls.rsplit('/', 1)[-1].replace('$', '_'), registry) if '/' in cls else _rust_type_to_binary(cls.replace('$', '_'), registry)
                if not _cls_bin and cls in registry:
                    _cls_bin = cls
                if _cls_bin:
                    _getstatic_cls_ci = registry.get(_cls_bin)
                    if _getstatic_cls_ci and _getstatic_cls_ci.generic_signature:
                        _tparams = _parse_class_type_params(_getstatic_cls_ci.generic_signature)
                        if _tparams:
                            _getstatic_turbofish = '::<' + ', '.join('Object' for _ in _tparams) + '>'
            # 若字段名与方法名冲突，emitter 生成了 fieldname_field 后缀，调用方也须一致
            _actual_field_name = field_name
            if _getstatic_cls_ci is not None:
                _method_names = {m.name for m in _getstatic_cls_ci.methods}
                if field_name in _method_names:
                    _actual_field_name = field_name + '_field'
            sim.push(StaticFieldRef(cls, _actual_field_name, RsNamed(ty_str), turbofish=_getstatic_turbofish), RsNamed(ty_str))
        else:
            sim.push(RawExpr(f"/* getstatic {comment} */"), RsNamed('Object'))
    elif op == 'putstatic':
        val_expr, _ = sim.pop()
        cls, field_name, descriptor = _parse_field_ref(comment) if comment else ('', '', '')
        if cls and field_name:
            raw_cls = cls.rsplit('/', 1)[-1].replace('$', '_')
            rust_fname = _safe_ident(field_name)
            sim.emit(RawStmt(f"{raw_cls}::set_{rust_fname}({render_expr(val_expr)});"))
        else:
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
        # 用完整路径（comment）而非 short_cls，避免 'LString;' 等非全限定名映射到 Object
        if comment and comment != 'java/lang/Object':
            elem_t = jvm_to_rust(f'L{comment};', registry)
        else:
            elem_t = 'Object'
        # 多态数组：不再因「元素类有子类」降级为 Vec<Object>。
        # 6b2ee14 的降级与 LVTT/字段声明的精确类型（如 HashMap.table:
        # Vec<HashMap_Node<K,V>>）冲突导致 E0308；
        # 多态存储由 aastore 的子类型 upcast（.into()）处理
        v = sim.fresh('_arr')
        # 用 Default::default() 而非 ElemType::default()，避免泛型类型（如 Node<K,V>）在 vec![] 中产生语法错误
        sim.emit(RawStmt(f"let mut {v}: Rc<RefCell<Vec<{elem_t}>>> = Rc::new(RefCell::new(vec![Default::default(); {render_expr(count_expr)} as usize]));"))
        sim.push(Var(v), RsNamed(f'Rc<RefCell<Vec<{elem_t}>>>'))
    elif op == 'multianewarray':
        dims_str = operand.split()[-1] if operand else '2'
        dims = int(dims_str) if dims_str.isdigit() else 2
        sizes = [render_expr(sim.pop()[0]) for _ in range(dims)][::-1]
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Vec<Vec<i32>> = vec![vec![0i32; {sizes[-1]} as usize]; {sizes[0]} as usize];"))
        sim.push(Var(v), RsGeneric('Vec', [RsGeneric('Vec', [I32])]))
    elif op in ('iastore', 'lastore', 'fastore', 'dastore'):
        val_expr, val_ty = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, _ = sim.pop()
        arr_str = render_expr(arr_expr)
        val_str = render_expr(val_expr)
        # Avoid RefCell double-borrow: if val reads from same array, extract to temp first
        if '.borrow()' in val_str and arr_str in val_str:
            tmp = sim.fresh('_tmp_val')
            sim.emit(RawStmt(f"let {tmp} = {val_str};"))
            val_str = tmp
        sim.emit(RawStmt(f"{arr_str}.borrow_mut()[{render_expr(idx_expr)} as usize] = {val_str};"))
    elif op == 'aastore':
        val_expr, val_ty = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        _VEC_WRAP = 'Rc<RefCell<Vec<'
        _VEC_WRAP_END = '>>>'
        if arr_ty_str.startswith(_VEC_WRAP) and arr_ty_str.endswith(_VEC_WRAP_END):
            elem_ty = arr_ty_str[len(_VEC_WRAP):-len(_VEC_WRAP_END)]
        elif arr_ty_str.startswith('Vec<') and arr_ty_str.endswith('>'):
            elem_ty = arr_ty_str[4:-1]
        else:
            elem_ty = 'Object'
        val_str = render_expr(val_expr)
        val_ty_str = render_type(val_ty)
        if elem_ty == 'Object' and val_ty_str not in ('Object', '()'):
            # Object 数组（含多态容器）：clone 后装箱以保留实际运行时类型
            val_str = _coerce_to_object(val_str, val_ty_str)
        elif elem_ty != 'Object' and val_ty_str == 'Object':
            val_str = f"Default::default()"
        elif val_ty_str not in _PRIMITIVE_RUST_TYPES:
            if (elem_ty != val_ty_str
                    and _is_subtype(val_ty_str.split('<')[0], elem_ty.split('<')[0], registry)):
                # R-2: 子类元素存入父类数组，用 __into_super() 链（替代已删除的 T55 From impl）
                chain = _into_super_chain(val_ty_str.split('<')[0], elem_ty.split('<')[0], registry)
                val_str = f"Clone::clone(&{val_str}){chain}"
            else:
                # 同类型数组：只需 Clone
                val_str = f"Clone::clone(&{val_str})"
        # F-1 fix: 非基本类型 val_str 可能包含 .borrow() 调用（如 aaload 的结果），
        # 若直接写 arr.borrow_mut()[i] = Clone::clone(&arr.borrow()[j]) 会导致
        # RefCell 同时持有 borrow 和 borrow_mut 而 panic。先提取到 tmp 释放 borrow。
        arr_str = render_expr(arr_expr)
        if val_ty_str not in _PRIMITIVE_RUST_TYPES and '.borrow()' in val_str:
            tmp = sim.fresh('_aastore_tmp')
            sim.emit(RawStmt(f"let {tmp} = {val_str};"))
            val_str = tmp
        sim.emit(RawStmt(f"{arr_str}.borrow_mut()[{render_expr(idx_expr)} as usize] = {val_str};"))
    elif op == 'bastore':
        val_expr, val_ty = sim.pop(); idx_expr, _ = sim.pop(); arr_expr, arr_ty = sim.pop()
        arr_ty_str = render_type(arr_ty)
        # boolean[] 在 JVM 中以 bastore 写入，Rust 映射为 Vec<bool>，需要 != 0 转换
        if arr_ty_str in ('Vec<bool>', 'Rc<RefCell<Vec<bool>>>'):
            val_s = render_expr(val_expr)
            val_ty_s = render_type(val_ty)
            if val_ty_s == 'bool':
                coerced = val_s
            else:
                coerced = f"(({val_s}) as i8 != 0)"
            sim.emit(RawStmt(f"{render_expr(arr_expr)}.borrow_mut()[{render_expr(idx_expr)} as usize] = {coerced};"))
        else:
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
        if arr_ty_str.startswith('Vec<'):
            elem_ty_str = arr_ty_str[4:-1]
        else:
            # 处理 Rc<RefCell<Vec<T>>> 形式（java 数组在 Rust 中的标准编码）
            import re as _re
            _m = _re.match(r'Rc<RefCell<Vec<(.+)>>>$', arr_ty_str)
            elem_ty_str = _m.group(1) if _m else 'Object'
        _idx_s = f"{render_expr(idx_expr)} as usize"
        _arr_s = render_expr(arr_expr)
        if elem_ty_str in _PRIMITIVE_RUST_TYPES:
            # 基本类型实现 Copy，borrow()[idx] 自动解引用为 owned 值，直接使用
            _load_expr = f"{_arr_s}.borrow()[{_idx_s}]"
        else:
            # 引用类型：vec[idx] 解引用为 T（move），&vec[idx] 取引用为 &T，Clone::clone(&T) → T
            _load_expr = f"Clone::clone(&{_arr_s}.borrow()[{_idx_s}])"
        sim.push(RawExpr(_load_expr), RsNamed(elem_ty_str))
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
        # 实例方法返回 this 时，this 是 &Self 引用，需要 Clone::clone 才能返回 owned 值
        if expr_s == 'this' and not sim.is_static:
            expr_s = 'Clone::clone(this)'
        elif ret_ty == 'Object' and actual_ty not in ('Object', '()'):
            expr_s = _coerce_to_object(expr_s, actual_ty)
        elif ret_ty != 'Object' and actual_ty == 'Object':
            # 泛型类型参数（如 T、K、V、E）不实现 Default，用 panic!("null") 代替
            if ret_ty in _ctparams:
                expr_s = 'panic!("null")'
            else:
                expr_s = f"Default::default()"
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty)
              and _is_subtype(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)):
            # R-2：返回值是子类型，用显式 __into_super() 链（替代已删除的 T55 From impl）
            chain = _into_super_chain(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)
            expr_s = f"{expr_s}{chain}"
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty) and actual_ty != 'Object'):
            # 类型不兼容（actual 不是 ret 的子类型时，如 checkcast Serializable → return Comparator<Object>）：
            # 条件 4 已处理 actual→ret 子类型，到这里说明 _is_subtype 未匹配，
            # 降级为 Default::default() 保证编译通过
            expr_s = 'Default::default()'
        sim.emit(RawStmt(f"return Ok({expr_s});"))

    # ── 控制流（循环由 method.py 处理，此处跳过）──
    elif op.startswith('if_icmp') or op.startswith('if') or op == 'goto':
        # 未被控制流 map 捕获的分支指令：仍需弹出操作数，防止遗留值污染后续栈状态
        if op.startswith('if_icmp') or op in ('if_acmpeq', 'if_acmpne'):
            if sim.stack: sim.pop()
            if sim.stack: sim.pop()
        elif op != 'goto':
            # 单操作数：ifeq / ifne / iflt / ifge / ifgt / ifle / ifnull / ifnonnull
            if sim.stack: sim.pop()

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
            elif src_name != 'Object' and cast_rust not in ('Object', '()', src_name):
                if _is_subtype(cast_rust.split('<')[0], src_name.split('<')[0], registry):
                    # 合法向下转型（源静态类型是目标的父类，如 Node → TreeNode，E0282）：
                    # 经 Object::from_any 保留运行时值再 downcast 恢复子类型，
                    # 不能用 Default::default() 占位（会丢失接收者类型导致无法推断）
                    expr = RawExpr(f"Object::from_any({render_expr(expr)}).downcast::<{cast_rust}>()")
                else:
                    # 二次 checkcast（非 Object 源类型）：两种具体类型不兼容，用 Default::default() 占位
                    expr = RawExpr('Default::default()')
            sim.push(expr, RsNamed(cast_rust))
    elif op == 'instanceof':
        # JVM 语义: pop objectref, push int(0/1)
        # 通过 registry 继承链做静态类型分析：
        #   obj 静态类型 IS-A target  → true（子类一定是父类）
        #   obj 静态类型 == target    → true
        #   否则（obj 是 target 超类，或无继承关系）→ false
        #   值语义下 Dog.into()→Animal 后类型信息已丢失，超类变量对子类 instanceof 为 false
        val_expr_inst, val_ty_inst = sim.pop() if sim.stack else (None, None)
        if comment and val_ty_inst is not None:
            if comment.startswith('['):
                target_rust = jvm_to_rust(comment, registry)
            else:
                target_rust = jvm_to_rust(f'L{comment};', registry)
            obj_ty_str = render_type(val_ty_inst)
            val_s_inst = render_expr(val_expr_inst)
            if obj_ty_str == 'Object':
                # 运行时多态：通过 ObjectVTable fn 指针（Arch-2）检查类型继承链
                # comment 本身就是 JVM 二进制名（如 java/util/List）
                sim.push(RawExpr(f"({val_s_inst}.is_instance_of(\"{comment}\"))"), BOOL)
            elif obj_ty_str == target_rust:
                sim.push(Lit('true'), BOOL)
            elif _is_subtype(obj_ty_str.split('<')[0], target_rust.split('<')[0], registry):
                sim.push(Lit('true'), BOOL)
            else:
                sim.push(Lit('false'), BOOL)
        else:
            sim.push(Lit('false'), BOOL)

    # ── invokedynamic ──
    elif op == 'invokedynamic':
        if comment and 'makeConcatWithConstants' in comment:
            _gen_string_concat(sim, comment)
        else:
            # 解析 comment 格式：
            # "InvokeDynamic samName:dynDesc [impl:Cls.method:implDesc] [samtype:samDesc]"
            _dyn_desc = ''
            _impl_method_ref = ''  # "Cls.lambda$main$0:desc"
            _sam_type_desc = ''
            if comment:
                _ctoks = comment.split(' ')
                # 第 2 个词是 "samName:dynDesc"
                if len(_ctoks) >= 2:
                    _nd = _ctoks[1]
                    _ci = _nd.find(':')
                    if _ci >= 0:
                        _dyn_desc = _nd[_ci + 1:]
                for _tok in _ctoks[2:]:
                    if _tok.startswith('impl:'):
                        _impl_method_ref = _tok[5:]
                    elif _tok.startswith('samtype:'):
                        _sam_type_desc = _tok[8:]

            # Arch-3: 若有 impl: 和 samtype:，生成真实 Rust 闭包
            _lam_idx = operand or '0'
            _cap_exprs: list[tuple] = []
            if _dyn_desc.startswith('('):
                _cap_descs = parse_descriptor_params(_dyn_desc)
                for _ci_idx, _cd in enumerate(_cap_descs):
                    if sim.stack:
                        _ce, _cty = sim.pop()
                        _cap_exprs.insert(0, (_ce, _cty, _ci_idx))
                    else:
                        _cap_exprs.insert(0, (RawExpr('Object::default()'), RsNamed('Object'), _ci_idx))

            if _impl_method_ref and _sam_type_desc:
                # 解析实现方法：Cls.method:desc 或 pkg/Cls.method:desc
                _dot = _impl_method_ref.rfind('.')
                _impl_colon = _impl_method_ref.find(':', _dot) if _dot >= 0 else -1
                if _dot >= 0 and _impl_colon > _dot:
                    _impl_cls_bin = _impl_method_ref[:_dot]           # "TestLambda" or "pkg/Cls"
                    _impl_mname   = _impl_method_ref[_dot+1:_impl_colon]  # "lambda$main$0"
                    _impl_desc    = _impl_method_ref[_impl_colon+1:]  # "(I)I"
                    # 转换为 Rust 标识符
                    _impl_cls_rust  = _impl_cls_bin.rsplit('/', 1)[-1]
                    _impl_mname_r   = _safe_ident(_impl_mname)
                    # SAM 方法参数/返回类型 → Rust 类型
                    _sam_params = parse_descriptor_params(_sam_type_desc)
                    _sam_ret    = parse_descriptor_return(_sam_type_desc)
                    _sam_ptypes = [jvm_to_rust(p, registry) for p in _sam_params]
                    _sam_rtype  = jvm_to_rust(_sam_ret, registry) if _sam_ret != 'V' else '()'
                    # 捕获变量声明
                    _cap_var_stmts: list[str] = []
                    _cap_var_names: list[str] = []
                    for _cv_idx, (_cexpr, _cty, _) in enumerate(reversed(_cap_exprs)):
                        _cv_name = f'__lam_cap{_lam_idx}_{_cv_idx}'
                        _cap_var_stmts.append(f'let {_cv_name} = {render_expr(_cexpr)};')
                        _cap_var_names.append(_cv_name)
                    # SAM 参数名
                    _sam_anames = [f'_la{i}' for i in range(len(_sam_ptypes))]
                    # Fn 类型签名（Result 用裸名：user crate 里 crate::error 是 E0433，两边均经 prelude 引入）
                    _fn_params_sig = ', '.join(f'{_a}: {_t}' for _a, _t in zip(_sam_anames, _sam_ptypes))
                    _fn_type = f'std::rc::Rc<dyn Fn({", ".join(_sam_ptypes)}) -> Result<{_sam_rtype}>>'
                    # 调用实现方法的参数列表（捕获变量 + SAM 参数）
                    # Clone::clone 而非 .clone()：捕获值可能是带 Java clone() 的类
                    _call_cap_args  = ', '.join(f'Clone::clone(&{v})' for v in _cap_var_names)
                    _call_sam_args  = ', '.join(_sam_anames)
                    _all_call_args  = ', '.join(filter(None, [_call_cap_args, _call_sam_args]))
                    # 生成闭包
                    for _s in _cap_var_stmts:
                        sim.emit(RawStmt(_s))
                    _cap_move = ' '.join(f'Clone::clone(&{v}),' for v in _cap_var_names)
                    _closure_body = f'{_impl_cls_rust}::{_impl_mname_r}({_all_call_args})'
                    _lam_varname = f'__lam_{_lam_idx}'
                    sim.emit(RawStmt(
                        f'let {_lam_varname}: {_fn_type} = std::rc::Rc::new('
                        f'move |{_fn_params_sig}| -> Result<{_sam_rtype}> '
                        f'{{ {_closure_body} }});'
                    ))
                    sim.push(RawExpr(f'Object::from_any({_lam_varname})'), RsNamed('Object'))
                else:
                    sim.emit(RawStmt(f"/* TODO: {op} {operand} (impl parse failed) */"))
                    if _sam_type_desc:
                        _r2 = parse_descriptor_return(_sam_type_desc)
                        if _r2 != 'V':
                            sim.push(RawExpr('Object::default()'), RsNamed('Object'))
            else:
                # 短期占位（无 impl 信息，如方法引用 REF_invokeVirtual 等）
                sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))
                _ret_desc = parse_descriptor_return(_dyn_desc) if _dyn_desc else 'V'
                if _ret_desc != 'V':
                    sim.push(RawExpr('Object::default()'), RsNamed('Object'))

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

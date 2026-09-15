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
from ..sig_parser import parse_class_type_params as _parse_class_type_params
from ..type_map import (
    jvm_to_rust, short_cls,
    NEWARRAY_TYPES,
)
from ..constants import safe_ident as _safe_ident
from ..types import Instr
from .coerce import (
    _float_lit, _escape_str, _parse_slot, _to_i32,
    _coerce_to_object, _coerce_from_null, _coerce_value,
    _find_field_super_prefix, _find_field_super_prefix_for_type,
    _parse_field_ref, _is_subtype, _rust_type_to_binary,
    _PRIMITIVE_RUST_TYPES,
)
from .invoke import (
    _gen_invokespecial, _gen_invokestatic, _gen_invokevirtual, _gen_string_concat,
)


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
        elif comment.startswith('class '): sim.push(Lit('Object::default()'), RsNamed('Object'))
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op in ('ldc2_w', 'ldc_w'):
        if comment.startswith('long '):   sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('String '):
            lit = _escape_str(comment[7:].strip())
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
            _, fname, fdesc = _parse_field_ref(comment)
            ftype = jvm_to_rust(fdesc, registry) if fdesc else 'Object'
            # T76: 基于接收者实际 Rust 类型查找字段的 _super 路径
            recv_base = render_type(obj_ty).split('<')[0].strip()
            cls_short = class_name.rsplit('/', 1)[-1] if '/' in class_name else class_name
            if recv_base == cls_short:
                super_pfx = _find_field_super_prefix(class_name, fname, registry)
            else:
                super_pfx = _find_field_super_prefix_for_type(recv_base, fname, registry)
            sim.push(RawExpr(f"{render_expr(obj_expr)}.{super_pfx}{fname}.get()"), RsNamed(ftype))
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
            elif (ftype not in _PRIMITIVE_RUST_TYPES and val_ty_name not in _PRIMITIVE_RUST_TYPES
                  and ftype not in ('Object', '()', val_ty_name)
                  and _is_subtype(val_ty_name.split('<')[0], ftype.split('<')[0], registry)):
                # T55：子类型赋给父类型字段
                val_str = f"{val_str_raw}.into()"
            else:
                val_str = _coerce_value(val_str_raw, val_ty, ftype)
            # 引用类型赋值时加 .clone()，避免 E0382（move after use）
            _obj_str = render_expr(obj_expr)
            if (val_ty_name not in _PRIMITIVE_RUST_TYPES
                    and not val_str.startswith('Default::')
                    and '.clone()' not in val_str):
                if val_str == 'this' and 'this' in _obj_str:
                    # this.field.set(this) → 借用与移动冲突，需 .clone()
                    val_str = 'this.clone()'
                elif val_str != 'this':
                    val_str = f'{val_str}.clone()'
            # T76: 基于接收者实际 Rust 类型查找字段的 _super 路径
            recv_base = render_type(obj_ty).split('<')[0].strip()
            cls_short = class_name.rsplit('/', 1)[-1] if '/' in class_name else class_name
            if recv_base == cls_short:
                super_pfx = _find_field_super_prefix(class_name, fname, registry)
            else:
                super_pfx = _find_field_super_prefix_for_type(recv_base, fname, registry)
            sim.emit(RawStmt(f"{render_expr(obj_expr)}.{super_pfx}{fname}.set({val_str});"))
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
        cls = short_cls(comment) or 'Object'
        elem_t = jvm_to_rust(f'L{cls};') if cls != 'Object' else 'Object'
        v = sim.fresh('_arr')
        sim.emit(RawStmt(f"let mut {v}: Rc<RefCell<Vec<{elem_t}>>> = Rc::new(RefCell::new(vec![{elem_t}::default(); {render_expr(count_expr)} as usize]));"))
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
        elif (ret_ty not in _PRIMITIVE_RUST_TYPES and actual_ty not in _PRIMITIVE_RUST_TYPES
              and ret_ty not in ('Object', '()', actual_ty)
              and _is_subtype(actual_ty.split('<')[0], ret_ty.split('<')[0], registry)):
            # T55：返回值是子类型，方法声明返回父类型
            expr_s = f"{expr_s}.into()"
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
    elif op == 'instanceof':
        # JVM 语义: pop objectref, push int(0/1)
        # 暂用 true 作为 stub，但必须消耗栈上的对象引用
        if sim.stack:
            sim.pop()
        sim.push(Lit('true'), BOOL)

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

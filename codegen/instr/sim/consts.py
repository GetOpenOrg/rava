# 从 codegen/instr/sim.py 中拆出

from ...stack import I32, I64, F32, F64
from ...rs_ir import Lit, RsNamed
from ..coerce import _float_lit, _escape_str


def sim_consts(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if   op == 'iconst_m1':                      sim.push(Lit('-1i32'), I32)
    elif op.startswith('iconst_'):               sim.push(Lit(f"{op[-1]}i32"), I32)
    elif op in ('lconst_0', 'lconst_1'):         sim.push(Lit(f"{op[-1]}i64"), I64)
    elif op in ('fconst_0', 'fconst_1', 'fconst_2'): sim.push(Lit(f"{op[-1]}f32"), F32)
    elif op in ('dconst_0', 'dconst_1'):         sim.push(Lit(f"{op[-1]}f64"), F64)
    elif op == 'bipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'sipush':                         sim.push(Lit(f"{operand}i32"), I32)
    elif op in ('ldc', 'ldc_w', 'ldc2_w'):
        # 三种宽度的常量装载语义相同，仅常量池索引宽度不同
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
        elif comment.startswith('class '):
            # 类字面量（X.class / X[].class）：生成携带 binary name 的 Class 对象。
            # 过去这里退化为 Object::default()（null），任何对它的调用都 NPE。
            # 第二参数是超类型闭包（父类链 + 全部接口，来自 registry），供
            # Class.isAssignableFrom 做静态可知的指派判定；数组与闭包外类为空。
            _bin = comment[6:].strip()
            _supers: list[str] = []
            if not _bin.startswith('[') and registry:
                _seen: set[str] = set()
                _queue: list[str] = [_bin]
                while _queue:
                    _cur = _queue.pop(0)
                    _ci = registry.get(_cur)
                    if _ci is None:
                        continue
                    for _sup in [_ci.super_class] + list(_ci.interfaces or []):
                        if _sup and _sup not in _seen:
                            _seen.add(_sup)
                            _supers.append(_sup)
                            _queue.append(_sup)
            _supers_lit = ', '.join(f'"{s}"' for s in _supers)
            sim.push(Lit(f'Class::for_class(String::from("{_bin}"), &[{_supers_lit}])'),
                     RsNamed('Class'))
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'aconst_null': sim.push(Lit('Object::default()'), RsNamed('Object'))
    else:
        return False
    return True

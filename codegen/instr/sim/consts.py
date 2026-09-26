# 从 codegen/instr/sim.py 中拆出

from ...stack import I32, I64, F32, F64
from ...rs_ir import Lit, RsNamed
from ..coerce import _float_lit, _escape_str
from ... import equiv_audit


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
            # comment 由 classfile._ldc_str 构造，值为常量池解码原样（无行尾
            # 换行附加）——不得 rstrip：尾部 \n 是常量内容（文本块尾行、
            # joining("\n") 等），剥掉会把 "\n" 常量发射成空串
            _val = comment[7:]
            if any(0xD800 <= ord(ch) <= 0xDFFF for ch in _val):
                # 含孤立代理项：Rust 字符串字面量无法表示（&str 必须是合法 UTF-8），
                # 以 UTF-16 码元数组构造（同样经常量池驻留，JLS §3.10.5）
                _b = _val.encode('utf-16-le', 'surrogatepass')
                _units = ', '.join(f'0x{int.from_bytes(_b[k:k + 2], "little"):04X}'
                                   for k in range(0, len(_b), 2))
                sim.push(Lit(f'String::from_utf16_lit(&[{_units}])'), RsNamed('String'))
            else:
                lit = _escape_str(_val)
                sim.push(Lit(f'String::from("{lit}")'), RsNamed('String'))
        elif comment.startswith('int '):    sim.push(Lit(comment[4:].strip() + 'i32'), I32)
        elif comment.startswith('float '): sim.push(Lit(_float_lit(comment[6:].strip(), 'f32')), F32)
        elif comment.startswith('long '):  sim.push(Lit(comment[5:].strip() + 'i64'), I64)
        elif comment.startswith('double '): sim.push(Lit(_float_lit(comment[7:].strip(), 'f64')), F64)
        elif comment.startswith('class '):
            # 类字面量（X.class / X[].class）：生成携带 binary name 的 Class 对象。
            # 过去这里退化为 Object::default()（null），任何对它的调用都 NPE。
            # isAssignableFrom 的层次查询在运行时进行（build.rs 从 all_supertypes
            # 生成层次表），此处不再静态推导超类型闭包。
            # [equiv-audit] class-literal（S-5）：每次构造新 Class 对象，与
            # getClass() 的同一性是近似等价——只计数，不改发射。
            equiv_audit.record('class-literal')
            _bin = comment[6:].strip()
            sim.push(Lit(f'Class::for_class(String::from("{_bin}"))'), RsNamed('Class'))
        else: sim.push(Lit(f"{operand}i32"), I32)
    elif op == 'aconst_null': sim.push(Lit('Object::default()'), RsNamed('Object'))
    else:
        return False
    return True

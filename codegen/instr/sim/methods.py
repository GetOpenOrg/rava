# 从 codegen/instr/sim.py 中拆出

from ...rs_ir import Var, RawStmt, RsNamed
from ...render import render_expr, render_type
from ...type_map import jvm_to_rust, parse_descriptor_params, parse_descriptor_return
from ...constants import PRIMITIVE_RUST_TYPES
from ..coerce import _coerce_to_object
from ..member_naming import _method_ref_binary_class, _method_ref_descriptor
from ..member_owner import parse_method_ref
from ..invoke import _gen_invokestatic, _gen_invokevirtual
from ... import equiv_audit

_ACC_VARARGS = 0x0080
_ACC_NATIVE = 0x0100


def _signature_polymorphic_descriptor(comment: str, registry) -> str:
    """调用目标是签名多态方法（JVMS §2.9.3）时返回其声明描述符，否则 ''。

    判定完全来自类文件：常量池类上没有与调用点描述符精确匹配的方法，而同名方法唯一、
    带 ACC_VARARGS | ACC_NATIVE、形参恰为一个引用数组。调用点描述符由 javac 按实参
    静态类型合成，与声明描述符无关。"""
    if not registry:
        return ''
    ci = registry.get(_method_ref_binary_class(comment))
    if ci is None:
        return ''
    dot = comment.find('.')
    colon = comment.find(':', dot)
    if dot < 0 or colon < 0:
        return ''
    mname = comment[dot + 1:colon]
    call_desc = _method_ref_descriptor(comment)
    named = [m for m in ci.methods if m.name == mname]
    if len(named) != 1 or named[0].descriptor == call_desc:
        return ''
    m = named[0]
    if (m.access_flags & (_ACC_VARARGS | _ACC_NATIVE)) != (_ACC_VARARGS | _ACC_NATIVE):
        return ''
    decl_params = parse_descriptor_params(m.descriptor)
    if len(decl_params) != 1 or not decl_params[0].startswith('[L'):
        return ''
    return m.descriptor


def _gen_signature_polymorphic(sim, comment: str, decl_desc: str, class_name, registry, is_static: bool) -> None:
    """签名多态调用：实参按 Java 语义装进声明的 Object[] 形参，返回值按调用点描述符还原
    （等价 `(R) mh.invokeBasic(new Object[]{a, b, c})`）。"""
    call_desc = _method_ref_descriptor(comment)
    call_params = parse_descriptor_params(call_desc)
    call_ret = parse_descriptor_return(call_desc)
    packed: list[str] = []
    for _ in call_params:
        e_expr, e_ty = sim.pop()
        ty = render_type(e_ty)
        e = render_expr(e_expr)
        if ty == 'Object':
            packed.insert(0, 'Clone::clone(this)' if e == 'this' else f'Clone::clone(&{e})')
        else:
            packed.insert(0, _coerce_to_object(e, ty, registry, sim.class_type_params))
    elem_ty = jvm_to_rust(parse_descriptor_params(decl_desc)[0], registry)
    arr = sim.fresh()
    sim.emit(RawStmt(f"let {arr}: {elem_ty} = JArray::from(vec![{', '.join(packed)}]);"))
    sim.push(Var(arr), RsNamed(elem_ty))
    decl_comment = comment[:comment.find(':', comment.find('.')) + 1] + decl_desc
    if is_static:
        _gen_invokestatic(sim, decl_comment, class_name, registry=registry)
    else:
        _gen_invokevirtual(sim, decl_comment, class_name, registry=registry)
    if parse_descriptor_return(decl_desc) == 'V':
        return
    r_expr, r_ty = sim.pop()
    if call_ret == 'V':
        return
    target = jvm_to_rust(call_ret, registry)
    if target == render_type(r_ty):
        sim.push(r_expr, r_ty)
        return
    v = sim.fresh()
    sim.emit(RawStmt(f"let {v}: {target} = <{target} as ::std::convert::From<Object>>::from({render_expr(r_expr)});"))
    sim.push(Var(v), RsNamed(target))


def _iface_default_init_gap(comment: str, registry) -> bool:
    """[equiv-audit] class-init（S-10 子缺口 b）的判定：invokeinterface 的目标
    是接口上带体的 default 方法时返回 True。JVMS §5.5 要求访问接口的非抽象方法
    前触发接口自身初始化；当前接口载体分派路径无该触发点。粗口径：只查常量池
    接口自身声明（声明在父接口的经链解析，不追——计数偏保守）。"""
    if not registry:
        return False
    ci = registry.get(_method_ref_binary_class(comment))
    if ci is None or not ci.is_interface:
        return False
    _cls, mname, params, ret = parse_method_ref(comment)
    _desc = f"({''.join(params)}){ret}"
    return any(m.name == mname and m.descriptor == _desc and not m.is_abstract
               for m in ci.methods)


def sim_methods(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op in ('invokestatic', 'invokevirtual'):
        _poly_desc = _signature_polymorphic_descriptor(comment, registry)
        if _poly_desc:
            _gen_signature_polymorphic(sim, comment, _poly_desc, class_name, registry,
                                       is_static=(op == 'invokestatic'))
            return True
    if op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name, registry=registry)
    elif op in ('invokevirtual', 'invokeinterface'):
        # [equiv-audit] class-init（S-10）：invokeinterface → 接口 default 方法，
        # 接口自身初始化未触发（JVMS §5.5 触发点缺口），只计数不改发射
        if op == 'invokeinterface' and _iface_default_init_gap(comment, registry):
            equiv_audit.record('class-init')
        _gen_invokevirtual(sim, comment, class_name, registry=registry)
    else:
        return False
    return True

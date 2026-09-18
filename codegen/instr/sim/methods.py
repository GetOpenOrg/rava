# 从 codegen/instr/sim.py 中拆出

from ..invoke import _gen_invokestatic, _gen_invokevirtual


def sim_methods(ins, sim, class_name, registry) -> bool:
    op      = ins.opcode
    operand = ins.operand or ''
    comment = ins.comment or ''

    if op == 'invokestatic':
        _gen_invokestatic(sim, comment, class_name, registry=registry)
    elif op in ('invokevirtual', 'invokeinterface'):
        _gen_invokevirtual(sim, comment, class_name, registry=registry)
    else:
        return False
    return True

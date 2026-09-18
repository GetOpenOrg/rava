# 从 codegen/instr/sim.py 中拆出
"""
sim_instr：JVM 字节码指令 → Rust 语句转换主分发函数（大 elif 链）。
拆分为 10 个子模块，各自处理一类指令。
"""

from .consts  import sim_consts
from .locals  import sim_locals
from .arith   import sim_arith
from .stack   import sim_stack
from .fields  import sim_fields
from .arrays  import sim_arrays
from .methods import sim_methods
from .returns import sim_returns
from .control import sim_control
from .dynamic import sim_dynamic

_HANDLERS = [
    sim_consts,
    sim_locals,
    sim_arith,
    sim_stack,
    sim_fields,
    sim_arrays,
    sim_methods,
    sim_returns,
    sim_control,
    sim_dynamic,
]


def sim_instr(ins, sim, class_name: str, registry=None):
    for handler in _HANDLERS:
        if handler(ins, sim, class_name, registry):
            return
    # 未知指令
    from ...rs_ir import RawStmt
    op      = ins.opcode
    operand = ins.operand or ''
    sim.emit(RawStmt(f"/* TODO: {op} {operand} */"))

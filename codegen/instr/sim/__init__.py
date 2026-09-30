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
    if ins.fold is not None:
        # closure.json invoke 折叠点（closure_folds）：调用照常翻译（被调方副作用保留），
        # 丢弃返回值，改压折叠常量
        from dataclasses import replace
        from ...rs_ir import RawStmt
        from ...render import render_expr
        from .consts import sim_consts
        from ...type_map import parse_descriptor_params
        nargs = len(parse_descriptor_params((ins.comment or '').split(':', 1)[-1]))
        depth = len(sim.stack) - nargs - (ins.opcode != 'invokestatic')
        sim_instr(replace(ins, fold=None), sim, class_name, registry)
        if len(sim.stack) > depth:
            sim.emit(RawStmt(f"let _ = {render_expr(sim.pop()[0])};"))
        sim_consts(ins.fold, sim, class_name, registry)
        return
    for handler in _HANDLERS:
        if handler(ins, sim, class_name, registry):
            return
    # 未知指令
    from ...rs_ir import RawStmt
    op      = ins.opcode
    operand = ins.operand or ''
    # 未支持的字节码不得静默丢弃（wide_iinc 曾被吞掉致 FdLibm.Sqrt 算术错误）：发射 panic
    # 存根，运行时命中即精确报出指令（原则 2 的存根约定）
    sim.emit(RawStmt(f'panic!("stub: unsupported bytecode {op} {operand}");'))

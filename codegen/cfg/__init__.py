"""
控制流结构化：基本块 CFG → 支配树 / 自然循环 → 结构树；不可归约时退到状态机兜底。
设计见 docs/plans/2026-09-18-cfg-structuring-rewrite.md。
"""

from ._opcodes import (
    TWO_OPERAND_BRANCH_OPS, ONE_OPERAND_BRANCH_OPS, COND_BRANCH_OPS,
    GOTO_OPS, SWITCH_OPS, EXIT_OPS, JUMP_OPS,
)
from .graph import (
    CfgError, Block, Terminator, FlowAnalysis,
    build_blocks, analyze, reachable, function_always_returns,
)
from .conditions import (
    Cond, atom, const, negate, cond_and, cond_or, render_cond, map_atoms,
    cmp_op, neg_cmp_op,
)
from . import structure
from .structure import structure as build_structure, simplify
from .dispatch import Dispatch, build_dispatch, next_pc_lines, PC_VAR
from .audit import CfgAuditError, JumpLedger, AuditStats, STATS

__all__ = [
    'TWO_OPERAND_BRANCH_OPS', 'ONE_OPERAND_BRANCH_OPS', 'COND_BRANCH_OPS',
    'GOTO_OPS', 'SWITCH_OPS', 'EXIT_OPS', 'JUMP_OPS',
    'CfgError', 'Block', 'Terminator', 'FlowAnalysis',
    'build_blocks', 'analyze', 'reachable', 'function_always_returns',
    'Cond', 'atom', 'const', 'negate', 'cond_and', 'cond_or', 'render_cond', 'map_atoms',
    'cmp_op', 'neg_cmp_op',
    'structure', 'build_structure', 'simplify',
    'Dispatch', 'build_dispatch', 'next_pc_lines', 'PC_VAR',
    'CfgAuditError', 'JumpLedger', 'AuditStats', 'STATS',
]

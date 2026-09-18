"""cfg 子模块共用的 JVM opcode 集合。"""

# 双操作数条件跳转
TWO_OPERAND_BRANCH_OPS: frozenset[str] = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
})

# 单操作数条件跳转
ONE_OPERAND_BRANCH_OPS: frozenset[str] = frozenset({
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifle', 'ifgt', 'ifnull', 'ifnonnull',
})

COND_BRANCH_OPS: frozenset[str] = TWO_OPERAND_BRANCH_OPS | ONE_OPERAND_BRANCH_OPS

GOTO_OPS: frozenset[str] = frozenset({'goto', 'goto_w'})

SWITCH_OPS: frozenset[str] = frozenset({'tableswitch', 'lookupswitch'})

# 返回/抛出指令集（所有退出指令）
EXIT_OPS: frozenset[str] = frozenset({
    'return', 'ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn', 'athrow',
})

# 子例程指令（class 文件版本 < 50 的 finally 实现）：不支持，生成期报错
SUBROUTINE_OPS: frozenset[str] = frozenset({'jsr', 'jsr_w', 'ret'})

# 所有需要被结构化消费的跳转指令（自检口径）
JUMP_OPS: frozenset[str] = COND_BRANCH_OPS | GOTO_OPS | SWITCH_OPS

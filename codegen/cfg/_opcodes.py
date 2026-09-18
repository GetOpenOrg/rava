"""cfg 子模块共用的 JVM opcode 集合。"""

# 条件/无条件跳转指令集（basic_blocks.py 与 loops.py 共用；以 basic_blocks.py 版本为准）
_BRANCH_OPS: frozenset[str] = frozenset({
    'if_icmpeq', 'if_icmpne', 'if_icmplt', 'if_icmpge', 'if_icmple', 'if_icmpgt',
    'if_acmpeq', 'if_acmpne',
    'ifeq', 'ifne', 'iflt', 'ifge', 'ifle', 'ifgt', 'ifnull', 'ifnonnull',
    'goto', 'goto_w',
})

# 返回/抛出指令集（所有退出指令）
_EXIT_OPS: frozenset[str] = frozenset({
    'return', 'ireturn', 'lreturn', 'freturn', 'dreturn', 'areturn', 'athrow',
})

"""
跳转消费自检：每个方法的每条 branch / goto / switch 指令必须被某个结构消费。

消费种类（kind）：
  structured     结构树中的 if / match / 内联 / break / continue
  short-circuit  并入前驱的 && / || 条件
  ternary        折叠为条件表达式（含 boolean 物化）
  const-fold     条件为编译期常量，折叠为无条件边
  dead           所在块在常量折叠后不可达（或字节码本身不可达）
  dispatch       不可归约 CFG 的状态机兜底

未被消费的跳转 → CfgAuditError（生成期错误，不允许静默继续）。
"""

from __future__ import annotations
from dataclasses import dataclass, field


class CfgAuditError(Exception):
    """存在未被消费的跳转指令。必须中止转译，不得被 stub 降级吞掉。"""


@dataclass
class JumpLedger:
    """单个方法的跳转账本。"""
    method_id: str
    expected: set = field(default_factory=set)          # 所有跳转指令的 pc
    consumed: dict = field(default_factory=dict)        # pc → kind

    def expect(self, pc: int) -> None:
        self.expected.add(pc)

    def consume(self, pc: int, kind: str) -> None:
        self.consumed[pc] = kind

    def verify(self) -> None:
        missing = sorted(self.expected - set(self.consumed))
        if missing:
            raise CfgAuditError(
                f"{self.method_id}: 未被消费的跳转指令 pc={missing}")


@dataclass
class AuditStats:
    methods: int = 0
    jumps: int = 0
    consumed: int = 0
    by_kind: dict = field(default_factory=dict)
    dispatch_methods: int = 0
    stub_fallbacks: list = field(default_factory=list)  # [(method_id, reason)]
    try_regions: int = 0                                 # 结构化为 java_try! 的 try 区域数
    handler_methods: dict = field(default_factory=dict)  # method_id → 未进入结构化树的异常处理器个数（终态 0）
    instanceof_folds: int = 0                             # instanceof 静态折叠为编译期 false 的次数
    _seen: set = field(default_factory=set)

    def record(self, ledger: JumpLedger, used_dispatch: bool) -> None:
        if ledger.method_id in self._seen:
            return
        self._seen.add(ledger.method_id)
        self.methods += 1
        self.jumps += len(ledger.expected)
        self.consumed += sum(1 for pc in ledger.expected if pc in ledger.consumed)
        for pc in ledger.expected:
            kind = ledger.consumed.get(pc)
            if kind is not None:
                self.by_kind[kind] = self.by_kind.get(kind, 0) + 1
        if used_dispatch:
            self.dispatch_methods += 1

    def record_stub_fallback(self, method_id: str, reason: str) -> None:
        if all(m != method_id for m, _ in self.stub_fallbacks):
            self.stub_fallbacks.append((method_id, reason))

    def record_try_regions(self, method_id: str, count: int, untranslated_handlers: int) -> None:
        self.try_regions += count
        if untranslated_handlers:
            self.handler_methods[method_id] = untranslated_handlers

    def record_instanceof_fold(self) -> None:
        """instanceof 被静态判为编译期 false（接收者与目标静态类型互不为子类型关系）。

        该折叠会把依赖运行时类型的分支当死代码消除——TestCasting 丢失全部
        `if (x instanceof T)` 分支即此（G-9）。审计行展示该计数，超预期时
        优先怀疑 instanceof 折叠误判（接收者静态类型是目标超类的场景已改为
        运行时判定，见 sim/control.py）。
        """
        self.instanceof_folds += 1

    def summary(self) -> str:
        kinds = ' '.join(f"{k}={v}" for k, v in sorted(self.by_kind.items()))
        return (f"[cfg-audit] methods={self.methods} jumps={self.jumps} "
                f"consumed={self.consumed} unconsumed={self.jumps - self.consumed} "
                f"dispatch={self.dispatch_methods} "
                f"try_regions={self.try_regions} "
                f"handler_methods={len(self.handler_methods)} "
                f"handler_jumps={self.by_kind.get('handler', 0)} "
                f"stub_fallback={len(self.stub_fallbacks)} "
                f"instanceof_fold={self.instanceof_folds} | {kinds}")

    def reset(self) -> None:
        self.__init__()


STATS = AuditStats()

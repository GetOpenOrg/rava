"""异常表 → try/catch 区域规划（JVMS §2.10 / §4.7.3）。

输入是 Code attribute 的异常表，输出是与结构化器无关的区域描述：

- **TryGroup**：受保护区间集合完全相同的一组处理器 = 一个 Java `try` 语句的兄弟 catch 子句
  （multi-catch 的多个 catch_type 指向同一 handler_pc，合并为一个子句）。
- 受保护区间之外的指令（javac 内联的 finally 副本、try 体尾部的 goto）不属于 try 体：
  CFG 层按「块是否被区间覆盖」给每个块标注所属 try 组，结构化器把未覆盖的块放在
  `java_try!` 之外（见 method/blocks.py 的 try 节点与 cfg/structure.py 的放置规则）。

本模块只依赖指令偏移，不依赖任何具体的控制流结构化实现。
"""
from __future__ import annotations

from dataclasses import dataclass, field
from typing import Optional


@dataclass
class CatchClause:
    handler_pc: int
    handler_idx: int
    # catch_type binary names；空列表 = catch-any（finally / monitor 兜底）
    catch_types: list = field(default_factory=list)
    is_catch_any: bool = False
    # catch 体的文本终点（异常变量的 LocalVariableTable 作用域终点）；无调试信息时为 None。
    # 只决定「catch 之后的代码」的词法位置（两种位置语义等价），不参与异常语义
    body_end_pc: Optional[int] = None


@dataclass
class TryGroup:
    ranges: tuple            # ((start_pc, end_pc), ...) 已排序
    clauses: list            # list[CatchClause]，按异常表顺序（= 匹配优先级）
    start_idx: int = 0       # try 体首指令下标
    body_end_idx: int = 0    # try 体文本范围终点 = 首个处理器下标

    def covers(self, pc: int) -> bool:
        return any(s <= pc < e for s, e in self.ranges)

    @property
    def first_handler_pc(self) -> int:
        return min(c.handler_pc for c in self.clauses)


class TryCatchPlan:
    """一个方法的全部 try 区域；结构化器按指令下标查询。"""

    def __init__(self, exception_table: list, instrs: list, local_vars: list | None = None):
        self._instrs = instrs
        self._off2idx = {ins.offset: idx for idx, ins in enumerate(instrs)}
        self._local_vars = local_vars or []
        self.groups: list[TryGroup] = self._build(exception_table or [])
        self._by_start: dict[int, list[TryGroup]] = {}
        for g in self.groups:
            self._by_start.setdefault(g.start_idx, []).append(g)
        # 同一起点的多个组：处理器越靠后的越外层
        for lst in self._by_start.values():
            lst.sort(key=lambda g: -g.first_handler_pc)

    # ── 构建 ────────────────────────────────────────────────────────
    def _build(self, table: list) -> list:
        # 1. 按 handler_pc 聚合：同一处理器的全部受保护区间与 catch_type
        by_handler: dict[int, dict] = {}
        order: list[int] = []
        for start_pc, end_pc, handler_pc, catch_type in table:
            if start_pc >= handler_pc:
                # 处理器保护自身（finally / monitorexit 的重试条目）：
                # 该语义由 Rust 侧的作用域释放承担，不构成 try 区域
                continue
            if end_pc > handler_pc:
                # javac 的 finally 区间会越过处理器入口（覆盖 catch-any 处理器开头的 astore）：
                # 越过入口的部分同样是「处理器保护自身」，受保护区间截断到处理器入口
                end_pc = handler_pc
            h = by_handler.get(handler_pc)
            if h is None:
                h = by_handler[handler_pc] = {'ranges': set(), 'types': [], 'any': False}
                order.append(handler_pc)
            h['ranges'].add((start_pc, end_pc))
            if catch_type is None:
                h['any'] = True
            elif catch_type not in h['types']:
                h['types'].append(catch_type)

        # 2. 区间集合相同的处理器 = 同一 try 语句的兄弟 catch
        by_ranges: dict[tuple, TryGroup] = {}
        groups: list[TryGroup] = []
        for handler_pc in order:
            h = by_handler[handler_pc]
            handler_idx = self._off2idx.get(handler_pc)
            if handler_idx is None:
                continue
            key = self._merge_ranges(h['ranges'])
            g = by_ranges.get(key)
            if g is None:
                g = by_ranges[key] = TryGroup(ranges=key, clauses=[])
                groups.append(g)
            g.clauses.append(CatchClause(
                handler_pc=handler_pc, handler_idx=handler_idx,
                catch_types=list(h['types']), is_catch_any=h['any'],
                body_end_pc=self._catch_body_end(handler_idx)))

        result = []
        for g in groups:
            start_idx = self._off2idx.get(g.ranges[0][0])
            if start_idx is None:
                continue
            g.start_idx = start_idx
            g.body_end_idx = min(c.handler_idx for c in g.clauses)
            if g.body_end_idx <= g.start_idx:
                continue
            result.append(g)
        return result

    def _catch_body_end(self, handler_idx: int) -> Optional[int]:
        """处理器首条 astore 的目标槽 → 该异常变量的作用域终点（必须是指令边界）。"""
        first = self._instrs[handler_idx]
        if not first.opcode.startswith('astore') or handler_idx + 1 >= len(self._instrs):
            return None
        op = first.opcode
        slot = int(op.split('_')[-1]) if '_' in op else int((first.operand or '0').strip())
        scope_start = self._instrs[handler_idx + 1].offset
        for lv_slot, lv_start, lv_len, *_rest in self._local_vars:
            if lv_slot == slot and lv_start == scope_start:
                end = lv_start + lv_len
                return end if end in self._off2idx else None
        return None

    def catch_body_ends(self) -> list:
        return sorted({c.body_end_pc for g in self.groups for c in g.clauses if c.body_end_pc is not None})

    @staticmethod
    def _merge_ranges(ranges) -> tuple:
        merged: list[list[int]] = []
        for s, e in sorted(ranges):
            if merged and s <= merged[-1][1]:
                merged[-1][1] = max(merged[-1][1], e)
            else:
                merged.append([s, e])
        return tuple((s, e) for s, e in merged)

    # ── 查询接口 ────────────────────────────────────────────────────
    def __bool__(self) -> bool:
        return bool(self.groups)

    def groups_by_start(self) -> dict:
        """try 体首指令下标 → 在该处开始的组（外层在前）。"""
        return self._by_start


# ── catch 绑定 ──────────────────────────────────────────────────────────

def _throwable_root(registry: dict) -> str:
    """athrow 操作数的静态类型（catch-any 绑定类型）= Throwable。

    此前借 vm_roots.txt 里的异常类沿超类链上溯定位（无字面量的权宜）——该清单
    已删除（VM 依赖声明并入 error.rs 的 vm-upcalls）。现按 constants 的共享
    常量直接取（与 OBJECT_CLASS / CLASS_CLASS 同一先例），registry 校验存在。"""
    from ..constants import THROWABLE_CLASS
    if THROWABLE_CLASS not in registry:
        raise RuntimeError(f"catch-any 绑定类型 {THROWABLE_CLASS} 不在 registry，无法生成 catch 分派")
    return THROWABLE_CLASS


def _binding_type(clause: CatchClause, registry: dict) -> str:
    from ..type_map import short_cls
    from ..instr.coerce import _common_ref_type
    if clause.is_catch_any or not clause.catch_types:
        return short_cls(_throwable_root(registry))
    lub = short_cls(clause.catch_types[0])
    for other in clause.catch_types[1:]:
        lub = _common_ref_type(lub, short_cls(other), registry) or short_cls(_throwable_root(registry))
    return lub


def catch_head(clause: CatchClause, bind: str, bind_ty: str) -> str:
    """`java_try!` 的 catch 子句头：`catch (e)` / `catch (e: A)` / `catch (e: A | B as LUB)`。"""
    from ..type_map import short_cls
    if clause.is_catch_any or not clause.catch_types:
        return f"catch ({bind})"
    names = [short_cls(t) for t in clause.catch_types]
    head = f"catch ({bind}: {' | '.join(names)}"
    if len(names) > 1 or names[0] != bind_ty:
        head += f" as {bind_ty}"
    return head + ")"

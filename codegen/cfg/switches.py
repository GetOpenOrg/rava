"""
switch 检测：tableswitch / lookupswitch。
"""

from dataclasses import dataclass
from ..types import Instr
from .loops import _EXIT_OPS


@dataclass
class SwitchInfo:
    """tableswitch / lookupswitch 结构信息。"""
    cases: list          # list of (value: int, start_idx: int, end_idx: int) end_idx exclusive
    default_start: int   # default case body 起始指令索引
    default_end: int     # default case body 结束（exclusive，不含末尾 goto）
    merge_idx: int       # switch 之后继续执行的指令索引（所有 case goto 的目标）


def _find_case_body_end(instrs: list[Instr], case_start_idx: int, merge_off: int) -> int:
    """从 case_start_idx 向后扫描，找到 goto <merge> 或 merge_off 指令的索引（exclusive）。"""
    for j in range(case_start_idx, len(instrs)):
        ins = instrs[j]
        # 到达 merge 点（fall-through case，无 goto）
        if ins.offset == merge_off:
            return j
        if ins.opcode == 'goto' and ins.operand:
            if int(ins.operand) == merge_off:
                return j  # exclusive: up to but not including the goto
        if ins.opcode in _EXIT_OPS:
            return j + 1
    return len(instrs)


def find_switches(instrs: list[Instr]) -> dict[int, SwitchInfo]:
    """
    检测 tableswitch / lookupswitch 指令，返回 {switch_instr_idx: SwitchInfo}。

    tableswitch operand 格式（classfile.py 生成）：
        'default:<off> low:<n> high:<n> offs:<o1,o2,...>'
    lookupswitch operand 格式：
        'default:<off> <k1>:<v1> <k2>:<v2> ...'
    """
    off2idx = {ins.offset: i for i, ins in enumerate(instrs)}
    result: dict[int, SwitchInfo] = {}

    for i, ins in enumerate(instrs):
        if ins.opcode not in ('tableswitch', 'lookupswitch') or not ins.operand:
            continue

        parts_dict: dict[str, str] = {}
        cases_off: list[tuple[int, int]] = []  # (value, target_offset)

        for part in ins.operand.split():
            if ':' not in part:
                continue
            k, v = part.split(':', 1)
            parts_dict[k] = v

        default_off = int(parts_dict.get('default', 0))

        if ins.opcode == 'tableswitch':
            if 'offs' not in parts_dict:
                continue  # old format without case offsets, skip
            low = int(parts_dict.get('low', 0))
            offsets = [int(x) for x in parts_dict['offs'].split(',') if x]
            for idx, off in enumerate(offsets):
                cases_off.append((low + idx, off))
        else:  # lookupswitch
            for part in ins.operand.split():
                if ':' not in part:
                    continue
                k, v = part.split(':', 1)
                if k == 'default':
                    continue
                try:
                    cases_off.append((int(k), int(v)))
                except ValueError:
                    pass

        # 确定 merge 点：优先扫描非 default case body（有 goto <merge>），
        # default 可能 fall-through 无 goto，不能作为第一扫描目标
        case_targets = [off for _, off in cases_off]
        merge_off: int | None = None
        for tgt_off in case_targets:
            tgt_idx = off2idx.get(tgt_off)
            if tgt_idx is None:
                continue
            for j in range(tgt_idx, len(instrs)):
                inj = instrs[j]
                if inj.opcode == 'goto' and inj.operand:
                    candidate = int(inj.operand)
                    if candidate > ins.offset:
                        merge_off = candidate
                        break
                if inj.opcode in _EXIT_OPS:
                    break
            if merge_off is not None:
                break

        if merge_off is None:
            continue
        merge_idx = off2idx.get(merge_off)
        if merge_idx is None:
            continue

        # 构建每个 case 的 body 范围
        cases: list[tuple[int, int, int]] = []
        for value, case_off in cases_off:
            cs = off2idx.get(case_off)
            if cs is None:
                continue
            ce = _find_case_body_end(instrs, cs, merge_off)
            cases.append((value, cs, ce))

        ds = off2idx.get(default_off)
        de = _find_case_body_end(instrs, ds, merge_off) if ds is not None else 0

        result[i] = SwitchInfo(
            cases=cases,
            default_start=ds if ds is not None else 0,
            default_end=de,
            merge_idx=merge_idx,
        )

    return result

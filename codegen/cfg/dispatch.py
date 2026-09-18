"""
不可归约 CFG 的通用兜底：程序计数器状态机。

    let mut __pc: i32 = <entry>;
    loop {
        match __pc {
            <block> => { <stmts>; __pc = <next>; }
            ...
            _ => unreachable!(),
        }
    }

每个基本块是一个 match 臂，终结指令翻译为对 __pc 的赋值。形态不依赖 CFG 的任何结构性质，
对任意（含不可归约）控制流保证语义正确。javac 产物恒为可归约 CFG，本路径只服务于
其他编译器 / 混淆器产出的字节码。
"""

from __future__ import annotations
from dataclasses import dataclass

from .conditions import render_cond
from .graph import CfgError

PC_VAR = '__pc'


@dataclass
class Dispatch:
    """结构树节点：状态机。blocks 按字节码顺序排列。"""
    entry: int
    blocks: list


def next_pc_lines(node) -> list[str]:
    """块的终结指令 → 对 __pc 赋值的 Rust 语句行（exit 块无后继，返回空）。"""
    if node.kind == 'exit':
        return []
    if node.kind == 'goto':
        return [f"{PC_VAR} = {node.target};"]
    if node.kind == 'cond':
        return [f"{PC_VAR} = if {render_cond(node.cond)} {{ {node.target} }} else {{ {node.fallthrough} }};"]
    if node.kind == 'switch':
        arms = [f"{' | '.join(str(v) for v in vals)} => {tgt}," for vals, tgt in node.cases]
        arms.append(f"_ => {node.default},")
        return [f"{PC_VAR} = match {node.key} {{"] + [f"    {a}" for a in arms] + ["};"]
    raise CfgError(f"未知终结类型 {node.kind}")


def build_dispatch(nodes: dict, entry: int, live: list[int]) -> list:
    return [Dispatch(entry=entry, blocks=sorted(live))]

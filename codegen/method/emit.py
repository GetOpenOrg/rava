"""
结构树 → entries（(indent, RsStmt | str) 列表，供变量提升 pass 与渲染使用）。

标签策略：循环 'lN、带标签块 'bN，按先序编号；只有被 break / continue 显式引用时才输出标签。
break / continue 指向最内层循环且中间没有隔着带标签块时省略标签（Rust E0695：
带标签块内部不允许出现无标签的 break / continue）。
"""

from __future__ import annotations

from ..cfg import render_cond, Dispatch, next_pc_lines, PC_VAR
from ..cfg.structure import Code, Decl, Block, Loop, If, Switch, Break, Continue, _significant
from ..rs_ir import RawStmt

_STEP = '    '


class TreeEmitter:
    def __init__(self, nodes: dict):
        self.nodes = nodes
        self.loop_names: dict[int, str] = {}     # header → 'lN
        self.block_names: dict[int, str] = {}    # follower → 'bN
        self.exit_owner: dict[int, int] = {}     # exit_label → loop header

    # ── 标签编号（先序）────────────────────────────────────────────────────

    def _number(self, seq: list) -> None:
        for it in seq:
            if isinstance(it, Loop):
                self.loop_names[it.header] = f"'l{len(self.loop_names)}"
                if it.exit_label is not None:
                    self.exit_owner[it.exit_label] = it.header
                self._number(it.body)
            elif isinstance(it, Block):
                self.block_names[it.label] = f"'b{len(self.block_names)}"
                self._number(it.body)
            elif isinstance(it, If):
                self._number(it.then)
                self._number(it.else_)
            elif isinstance(it, Switch):
                for _, body in it.arms:
                    self._number(body)

    # ── 输出 ───────────────────────────────────────────────────────────────

    def emit(self, tree: list, indent: str) -> list:
        self._number(tree)
        out: list = []
        self._seq(tree, indent, [], out, set())
        return out

    def _jump(self, keyword: str, header: int, ctx: list, used: set) -> str:
        """ctx：由外到内的 ('loop', header) / ('block', label) 栈。"""
        for kind, ident in reversed(ctx):
            if kind == 'block':
                break
            if kind == 'loop':
                if ident == header:
                    return f"{keyword};"
                break
        used.add(('loop', header))
        return f"{keyword} {self.loop_names[header]};"

    def _seq(self, seq: list, ind: str, ctx: list, out: list, used: set) -> None:
        for it in seq:
            if isinstance(it, Code):
                for stmt in self.nodes[it.block].stmts:
                    out.append((ind, stmt))
            elif isinstance(it, Decl):
                for line in it.lines:
                    out.append(('', f"{ind}{line}"))
            elif isinstance(it, Break):
                if it.label in self.exit_owner:
                    out.append(('', ind + self._jump('break', self.exit_owner[it.label], ctx, used)))
                else:
                    used.add(('block', it.label))
                    out.append(('', f"{ind}break {self.block_names[it.label]};"))
            elif isinstance(it, Continue):
                out.append(('', ind + self._jump('continue', it.label, ctx, used)))
            elif isinstance(it, Block):
                body: list = []
                self._seq(it.body, ind + _STEP, ctx + [('block', it.label)], body, used)
                out.append(('', f"{ind}{self.block_names[it.label]}: {{"))
                out.extend(body)
                out.append(('', f"{ind}}}"))
            elif isinstance(it, Loop):
                body = []
                self._seq(it.body, ind + _STEP, ctx + [('loop', it.header)], body, used)
                label = f"{self.loop_names[it.header]}: " if ('loop', it.header) in used else ''
                head = f"while {render_cond(it.while_cond)} {{" if it.while_cond is not None else "loop {"
                out.append(('', f"{ind}{label}{head}"))
                out.extend(body)
                out.append(('', f"{ind}}}"))
            elif isinstance(it, If):
                self._if(it, ind, ctx, out, used, prefix='')
            elif isinstance(it, Switch):
                out.append(('', f"{ind}match {it.key} {{"))
                for vals, body in it.arms:
                    pattern = '_' if vals is None else ' | '.join(str(v) for v in vals)
                    out.append(('', f"{ind}{_STEP}{pattern} => {{"))
                    self._seq(body, ind + _STEP * 2, ctx, out, used)
                    out.append(('', f"{ind}{_STEP}}}"))
                out.append(('', f"{ind}}}"))
            elif isinstance(it, Dispatch):
                self._dispatch(it, ind, out)
            else:
                raise TypeError(f"未知结构树节点 {type(it).__name__}")

    def _if(self, it: If, ind: str, ctx: list, out: list, used: set, prefix: str) -> None:
        then_sig, else_sig = _significant(it.then), _significant(it.else_)
        if not then_sig and not else_sig:
            # 两臂皆空：条件只为副作用求值
            out.append((ind, RawStmt(f"let _ = {render_cond(it.cond)};")))
            return
        out.append(('', f"{ind}{prefix}if {render_cond(it.cond)} {{"))
        self._seq(it.then, ind + _STEP, ctx, out, used)
        if else_sig:
            if len(else_sig) == 1 and isinstance(else_sig[0], If) \
                    and (_significant(else_sig[0].then) or _significant(else_sig[0].else_)):
                self._if(else_sig[0], ind, ctx, out, used, prefix='} else ')
                return
            out.append(('', f"{ind}}} else {{"))
            self._seq(it.else_, ind + _STEP, ctx, out, used)
        out.append(('', f"{ind}}}"))

    def _dispatch(self, it: Dispatch, ind: str, out: list) -> None:
        out.append(('', f"{ind}let mut {PC_VAR}: i32 = {it.entry};"))
        out.append(('', f"{ind}loop {{"))
        out.append(('', f"{ind}{_STEP}match {PC_VAR} {{"))
        arm_ind = ind + _STEP * 2
        for bid in it.blocks:
            node = self.nodes[bid]
            out.append(('', f"{arm_ind}{bid} => {{"))
            for stmt in node.stmts:
                out.append((arm_ind + _STEP, stmt))
            for line in next_pc_lines(node):
                out.append(('', f"{arm_ind}{_STEP}{line}"))
            out.append(('', f"{arm_ind}}}"))
        out.append(('', f"{arm_ind}_ => unreachable!(),"))
        out.append(('', f"{ind}{_STEP}}}"))
        out.append(('', f"{ind}}}"))


def emit_tree(tree: list, nodes: dict, indent: str = _STEP) -> list:
    return TreeEmitter(nodes).emit(tree, indent)

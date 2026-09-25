"""
结构树 → entries（(indent, RsStmt | str) 列表，供变量提升 pass 与渲染使用）。

窗口 3 G-1：块结构产出为 `('', BlockStmt)` 节点（kind + 交替的结构行 / 子语句序列），
`emit_tree_structured` 返回结构化序列，`flatten` 按序展开为扁平 entries——与此前
直接拍平的输出逐项相同（emit_tree = flatten ∘ emit_tree_structured）。

标签策略：循环 'lN、带标签块 'bN，按先序编号；只有被 break / continue 显式引用时才输出标签。
break / continue 指向最内层循环且中间没有隔着带标签块时省略标签（Rust E0695：
带标签块内部不允许出现无标签的 break / continue）。

try 区域输出为 `java_try! { try { .. } catch (e: T) { .. } }`。跨越 try 边界的 break / continue
一律带标签（宏对带标签跳转原样放行，语义与 Java 一致：离开 try 语句，不经过任何 catch）。
"""

from __future__ import annotations

from ..cfg import render_cond, Dispatch, next_pc_lines, PC_VAR
from ..cfg.structure import Code, Decl, Block, Loop, If, Switch, Try, Break, Continue
from ..cfg.simplify import _significant
from ..render import render_expr, render_type
from ..rs_ir import BlockStmt, LetStmt, RawStmt
from .try_catch import catch_head

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
            elif isinstance(it, Try):
                self._number(it.body)
                for _, body in it.catches:
                    self._number(body)

    # ── 输出 ───────────────────────────────────────────────────────────────

    def emit(self, tree: list, indent: str) -> list:
        self._number(tree)
        out: list = []
        self._seq(tree, indent, [], out, set())
        return out

    @staticmethod
    def _block(kind: str, out: list) -> list:
        """新建块节点挂到 out，返回其 segs（结构行与子序列按序追加）。"""
        blk = BlockStmt(kind)
        out.append(('', blk))
        return blk.segs

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
                for decl in it.lines:
                    out.append((ind, decl))
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
                segs = self._block('block', out)
                segs.append(('', f"{ind}{self.block_names[it.label]}: {{"))
                segs.append(body)
                segs.append(('', f"{ind}}}"))
            elif isinstance(it, Loop):
                body = []
                self._seq(it.body, ind + _STEP, ctx + [('loop', it.header)], body, used)
                label = f"{self.loop_names[it.header]}: " if ('loop', it.header) in used else ''
                head = f"while {render_cond(it.while_cond)} {{" if it.while_cond is not None else "loop {"
                segs = self._block('loop', out)
                segs.append(('', f"{ind}{label}{head}"))
                segs.append(body)
                segs.append(('', f"{ind}}}"))
            elif isinstance(it, If):
                self._if(it, ind, ctx, out, used, prefix='')
            elif isinstance(it, Switch):
                segs = self._block('match', out)
                segs.append(('', f"{ind}match {it.key} {{"))
                for vals, body in it.arms:
                    pattern = '_' if vals is None else ' | '.join(str(v) for v in vals)
                    segs.append(('', f"{ind}{_STEP}{pattern} => {{"))
                    arm: list = []
                    self._seq(body, ind + _STEP * 2, ctx, arm, used)
                    segs.append(arm)
                    segs.append(('', f"{ind}{_STEP}}}"))
                segs.append(('', f"{ind}}}"))
            elif isinstance(it, Try):
                self._try(it, ind, ctx, out, used)
            elif isinstance(it, Dispatch):
                self._dispatch(it, ind, out)
            else:
                raise TypeError(f"未知结构树节点 {type(it).__name__}")

    def _if(self, it: If, ind: str, ctx: list, out: list, used: set, prefix: str,
            segs: 'list | None' = None) -> None:
        """segs 非 None：else-if 链的后继臂，并入链首块节点的 segs（结构行序与旧
        扁平输出一致：`} else if c {` 直接接在前一臂子序列之后）。"""
        then_sig, else_sig = _significant(it.then), _significant(it.else_)
        if not then_sig and not else_sig:
            # 两臂皆空：条件只为副作用求值
            out.append((ind, RawStmt(f"let _ = {render_cond(it.cond)};")))
            return
        if segs is None:
            segs = self._block('if', out)
        segs.append(('', f"{ind}{prefix}if {render_cond(it.cond)} {{"))
        then_body: list = []
        self._seq(it.then, ind + _STEP, ctx, then_body, used)
        segs.append(then_body)
        if else_sig:
            if len(else_sig) == 1 and isinstance(else_sig[0], If) \
                    and (_significant(else_sig[0].then) or _significant(else_sig[0].else_)):
                self._if(else_sig[0], ind, ctx, out, used, prefix='} else ', segs=segs)
                return
            segs.append(('', f"{ind}}} else {{"))
            else_body: list = []
            self._seq(it.else_, ind + _STEP, ctx, else_body, used)
            segs.append(else_body)
        segs.append(('', f"{ind}}}"))

    def _try(self, it: Try, ind: str, ctx: list, out: list, used: set) -> None:
        inner_ctx = ctx + [('block', None)]      # try 边界：其内的 break / continue 必须带标签
        body_ind = ind + _STEP * 2
        segs = self._block('try', out)
        segs.append(('', f"{ind}java_try! {{"))
        segs.append(('', f"{ind}{_STEP}try {{"))
        try_body: list = []
        self._seq(it.body, body_ind, inner_ctx, try_body, used)
        segs.append(try_body)
        for (clause, bind, bind_ty), body in it.catches:
            handler: list = []
            self._seq(body, body_ind, inner_ctx, handler, used)
            # 处理器首条 astore 生成的 `let e: T = _caughtN;` 并入 catch 头
            if handler and isinstance(handler[0][1], LetStmt):
                first = handler[0][1]
                if (first.value is not None and first.ty is not None
                        and render_expr(first.value) in (bind, f"Clone::clone(&{bind})")
                        and render_type(first.ty) == bind_ty):
                    bind = first.name
                    del handler[0]
            segs.append(('', f"{ind}{_STEP}}} {catch_head(clause, bind, bind_ty)} {{"))
            segs.append(handler)
        segs.append(('', f"{ind}{_STEP}}}"))
        segs.append(('', f"{ind}}}"))

    def _dispatch(self, it: Dispatch, ind: str, out: list) -> None:
        # 状态机前导声明是块外语句行（与块同层），块节点从 loop 开始
        out.append(('', f"{ind}let mut {PC_VAR}: i32 = {it.entry};"))
        segs = self._block('dispatch', out)
        segs.append(('', f"{ind}loop {{"))
        segs.append(('', f"{ind}{_STEP}match {PC_VAR} {{"))
        arm_ind = ind + _STEP * 2
        for bid in it.blocks:
            node = self.nodes[bid]
            segs.append(('', f"{arm_ind}{bid} => {{"))
            arm: list = []
            for stmt in node.stmts:
                arm.append((arm_ind + _STEP, stmt))
            for line in next_pc_lines(node):
                arm.append(('', f"{arm_ind}{_STEP}{line}"))
            segs.append(arm)
            segs.append(('', f"{arm_ind}}}"))
        segs.append(('', f"{arm_ind}_ => unreachable!(),"))
        segs.append(('', f"{ind}{_STEP}}}"))
        segs.append(('', f"{ind}}}"))


def flatten(seq: list, out: 'list | None' = None) -> list:
    """结构化序列 → 扁平 entries：块节点按 segs 顺序展开（结构行原样、子序列递归）。"""
    if out is None:
        out = []
    for entry in seq:
        item = entry[1]
        if isinstance(item, BlockStmt):
            for seg in item.segs:
                if isinstance(seg, list):
                    flatten(seg, out)
                else:
                    out.append(seg)
        else:
            out.append(entry)
    return out


def emit_tree_structured(tree: list, nodes: dict, indent: str = _STEP) -> list:
    """结构树 → 结构化 entries（块为 `('', BlockStmt)`）。"""
    return TreeEmitter(nodes).emit(tree, indent)


def emit_tree(tree: list, nodes: dict, indent: str = _STEP) -> list:
    return flatten(emit_tree_structured(tree, nodes, indent))

"""结构树可读性规整（保语义重写通道）——自 structure.py 后半段拆出。

对 structure() 产出的结构树做纯函数化简：尾跳转清理、if/while 形态规整、
break/continue 重定向。全部为无状态纯函数（2026-09-20 窗口 0②，纯搬移）。
"""

from .conditions import negate
from .structure import Code, Decl, Block, Loop, If, Switch, Try, Break, Continue



def _significant(seq: list) -> list:
    return [it for it in seq if not (isinstance(it, Code) and it.empty and not it.exits)]


def completes_normally(seq: list) -> bool:
    """控制是否可能从序列末尾「落出」。"""
    items = _significant(seq)
    if not items:
        return True
    last = items[-1]
    if isinstance(last, (Break, Continue)):
        return False
    if isinstance(last, Code):
        return not last.exits
    if isinstance(last, If):
        return completes_normally(last.then) or completes_normally(last.else_)
    if isinstance(last, Switch):
        return any(completes_normally(body) for _, body in last.arms)
    if isinstance(last, Loop):
        return last.exit_label is not None or last.while_cond is not None
    if isinstance(last, Try):
        return completes_normally(last.body) or any(completes_normally(b) for _, b in last.catches)
    return True   # Block（仍有 break 引用才会保留）/ Decl


def _count_breaks(seq: list, label: int) -> int:
    total = 0
    for it in seq:
        if isinstance(it, Break):
            total += (it.label == label)
        elif isinstance(it, (Block, Loop)):
            total += _count_breaks(it.body, label)
        elif isinstance(it, If):
            total += _count_breaks(it.then, label) + _count_breaks(it.else_, label)
        elif isinstance(it, Switch):
            for _, body in it.arms:
                total += _count_breaks(body, label)
        elif isinstance(it, Try):
            total += _count_breaks(it.body, label)
            for _, body in it.catches:
                total += _count_breaks(body, label)
    return total


def _retarget_breaks(seq: list, labels: set, new_label) -> None:
    for it in seq:
        if isinstance(it, Break):
            if it.label in labels:
                it.label = new_label
        elif isinstance(it, (Block, Loop)):
            _retarget_breaks(it.body, labels, new_label)
        elif isinstance(it, If):
            _retarget_breaks(it.then, labels, new_label)
            _retarget_breaks(it.else_, labels, new_label)
        elif isinstance(it, Switch):
            for _, body in it.arms:
                _retarget_breaks(body, labels, new_label)
        elif isinstance(it, Try):
            _retarget_breaks(it.body, labels, new_label)
            for _, body in it.catches:
                _retarget_breaks(body, labels, new_label)


def _pass_tail(seq: list, tail: frozenset) -> tuple[list, bool]:
    """尾跳转消除 + 无引用 block 拆除 + Block∘Loop 合并。

    tail：与「从本序列末尾落出」等价的跳转集合（('break', L) / ('continue', L)）。
    """
    changed = False
    out: list = []
    n = len(seq)
    for i, it in enumerate(seq):
        rest = _significant(seq[i + 1:])
        if not rest:
            ctx = tail
        elif isinstance(rest[0], Break):
            # 紧随其后就是同一跳转：臂尾的该跳转与「落出本项」等价
            ctx = frozenset({('break', rest[0].label)})
        elif isinstance(rest[0], Continue):
            ctx = frozenset({('continue', rest[0].label)})
        else:
            ctx = frozenset()
        if isinstance(it, Break):
            if ('break', it.label) in ctx:
                changed = True
                continue
            out.append(it)
        elif isinstance(it, Continue):
            if ('continue', it.label) in ctx:
                changed = True
                continue
            out.append(it)
        elif isinstance(it, If):
            it.then, c1 = _pass_tail(it.then, ctx)
            it.else_, c2 = _pass_tail(it.else_, ctx)
            changed |= c1 or c2
            out.append(it)
        elif isinstance(it, Switch):
            new_arms = []
            for vals, body in it.arms:
                body, c = _pass_tail(body, ctx)
                changed |= c
                new_arms.append((vals, body))
            it.arms = new_arms
            out.append(it)
        elif isinstance(it, Try):
            # try 体 / catch 体正常完成 ≡ 落出整个 try 语句
            it.body, c = _pass_tail(it.body, ctx)
            changed |= c
            new_catches = []
            for desc, body in it.catches:
                body, c = _pass_tail(body, ctx)
                changed |= c
                new_catches.append((desc, body))
            it.catches = new_catches
            out.append(it)
        elif isinstance(it, Loop):
            it.body, c = _pass_tail(it.body, frozenset({('continue', it.header)}))
            changed |= c
            # 循环处于尾位置：跳到「与落出本序列等价」的标签 ≡ 跳出本循环（普通 break）
            tail_labels = {lbl for kind, lbl in ctx if kind == 'break'}
            if tail_labels and any(_count_breaks(it.body, lbl) for lbl in tail_labels):
                if it.exit_label is None:
                    it.exit_label = ('loop-exit', it.header)
                _retarget_breaks(it.body, tail_labels, it.exit_label)
                changed = True
            out.append(it)
        elif isinstance(it, Block):
            it.body, c = _pass_tail(it.body, ctx | {('break', it.label)})
            changed |= c
            if _count_breaks(it.body, it.label) == 0:
                out.extend(it.body)
                changed = True
                continue
            sig = [b for b in _significant(it.body) if not isinstance(b, Decl)]
            if (len(sig) == 1 and isinstance(sig[0], Loop) and sig[0].exit_label is None
                    and sig[0].while_cond is None):
                # 'E: { loop {..} } → 循环自身的 break
                sig[0].exit_label = it.label
                out.extend(it.body)
                changed = True
                continue
            out.append(it)
        else:
            out.append(it)
    return out, changed


def _pass_if(seq: list) -> tuple[list, bool]:
    """if 规整：guard 展平、空 then 取反。"""
    changed = False
    out: list = []
    for it in seq:
        if isinstance(it, If):
            it.then, c1 = _pass_if(it.then)
            it.else_, c2 = _pass_if(it.else_)
            changed |= c1 or c2
            then_sig = _significant(it.then)
            else_sig = _significant(it.else_)
            if not then_sig and else_sig:
                it.cond = negate(it.cond)
                it.then, it.else_ = it.else_, it.then
                then_sig, else_sig = else_sig, []
                changed = True
            if else_sig:
                if not completes_normally(it.then):
                    rest, it.else_ = it.else_, []
                    out.append(it)
                    out.extend(rest)
                    changed = True
                    continue
                if not completes_normally(it.else_):
                    it.cond = negate(it.cond)
                    rest, it.then = it.then, it.else_
                    it.else_ = []
                    out.append(it)
                    out.extend(rest)
                    changed = True
                    continue
            out.append(it)
        elif isinstance(it, Switch):
            new_arms = []
            for vals, body in it.arms:
                body, c = _pass_if(body)
                changed |= c
                new_arms.append((vals, body))
            it.arms = new_arms
            out.append(it)
        elif isinstance(it, Try):
            it.body, c = _pass_if(it.body)
            changed |= c
            new_catches = []
            for desc, body in it.catches:
                body, c = _pass_if(body)
                changed |= c
                new_catches.append((desc, body))
            it.catches = new_catches
            out.append(it)
        elif isinstance(it, (Block, Loop)):
            it.body, c = _pass_if(it.body)
            changed |= c
            out.append(it)
        else:
            out.append(it)
    return out, changed


def _pass_while(seq: list) -> None:
    """loop { if c { break; } rest } → while !c { rest }（循环头无语句时）。"""
    for it in seq:
        if isinstance(it, If):
            _pass_while(it.then)
            _pass_while(it.else_)
        elif isinstance(it, Switch):
            for _, body in it.arms:
                _pass_while(body)
        elif isinstance(it, Block):
            _pass_while(it.body)
        elif isinstance(it, Try):
            _pass_while(it.body)
            for _, body in it.catches:
                _pass_while(body)
        elif isinstance(it, Loop):
            _pass_while(it.body)
            if it.while_cond is not None or it.exit_label is None:
                continue
            head = [b for b in it.body if not (isinstance(b, Code) and b.empty and not b.exits)]
            if not head or not isinstance(head[0], If):
                continue
            first = head[0]
            then_sig = _significant(first.then)
            if (not _significant(first.else_) and len(then_sig) == 1
                    and isinstance(then_sig[0], Break) and then_sig[0].label == it.exit_label):
                it.while_cond = negate(first.cond)
                it.cond_origin = first.origin
                it.body = [b for b in it.body if b is not first]


def simplify(tree: list) -> list:
    for _ in range(64):
        tree, c1 = _pass_tail(tree, frozenset())
        tree, c2 = _pass_if(tree)
        if not (c1 or c2):
            break
    _pass_while(tree)
    return tree


# ─────────────────────────────────────────────────────────────────────────────
# 树遍历（供自检使用）
# ─────────────────────────────────────────────────────────────────────────────

def walk(seq: list):
    for it in seq:
        yield it
        if isinstance(it, (Block, Loop)):
            yield from walk(it.body)
        elif isinstance(it, If):
            yield from walk(it.then)
            yield from walk(it.else_)
        elif isinstance(it, Switch):
            for _, body in it.arms:
                yield from walk(body)
        elif isinstance(it, Try):
            yield from walk(it.body)
            for _, body in it.catches:
                yield from walk(body)

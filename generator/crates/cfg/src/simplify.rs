//! 结构树可读性规整（← `cfg/simplify.py`）：尾跳转清理、if / while 形态规整、
//! break / continue 重定向。全部为保语义的纯函数重写。

use std::collections::BTreeSet;

use crate::graph::NodeId;
use crate::tree::{significant, BreakLabel, Item};

/// 控制是否可能从序列末尾「落出」。
pub fn completes_normally(seq: &[Item]) -> bool {
    let items = significant(seq);
    let Some(last) = items.last() else { return true };
    match last {
        Item::Break(_) | Item::Continue(_) => false,
        Item::Code { exits, .. } => !exits,
        Item::If(i) => completes_normally(&i.then) || completes_normally(&i.else_),
        Item::Switch(s) => s.arms.iter().any(|a| completes_normally(&a.body)),
        Item::Loop(l) => l.exit_label.is_some() || l.while_cond.is_some(),
        Item::Try(t) => {
            completes_normally(&t.body) || t.catches.iter().any(|c| completes_normally(&c.body))
        }
        // Block（仍有 break 引用才会保留）/ Decl
        Item::Block { .. } | Item::Decl(_) => true,
    }
}

/// 序列内（含嵌套）指向 `label` 的 break 数。
pub fn count_breaks(seq: &[Item], label: BreakLabel) -> usize {
    let mut total = 0;
    for it in seq {
        match it {
            Item::Break(l) => total += usize::from(*l == label),
            _ => total += it.children().into_iter().map(|c| count_breaks(c, label)).sum::<usize>(),
        }
    }
    total
}

fn for_each_child_mut(it: &mut Item, f: &mut dyn FnMut(&mut Vec<Item>)) {
    match it {
        Item::Block { body, .. } => f(body),
        Item::Loop(l) => f(&mut l.body),
        Item::If(i) => {
            f(&mut i.then);
            f(&mut i.else_);
        }
        Item::Switch(s) => s.arms.iter_mut().for_each(|a| f(&mut a.body)),
        Item::Try(t) => {
            f(&mut t.body);
            t.catches.iter_mut().for_each(|c| f(&mut c.body));
        }
        _ => {}
    }
}

fn retarget_breaks(seq: &mut [Item], labels: &BTreeSet<BreakLabel>, new_label: BreakLabel) {
    for it in seq.iter_mut() {
        if let Item::Break(l) = it {
            if labels.contains(l) {
                *l = new_label;
            }
        } else {
            for_each_child_mut(it, &mut |c| retarget_breaks(c, labels, new_label));
        }
    }
}

/// 与「从序列末尾落出」等价的跳转。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Jump {
    Break(BreakLabel),
    Continue(NodeId),
}

type Tail = BTreeSet<Jump>;

fn tail_after(rest: &[&Item], tail: &Tail) -> Tail {
    match rest.first() {
        None => tail.clone(),
        // 紧随其后就是同一跳转：臂尾的该跳转与「落出本项」等价
        Some(Item::Break(l)) => Tail::from([Jump::Break(*l)]),
        Some(Item::Continue(l)) => Tail::from([Jump::Continue(*l)]),
        Some(_) => Tail::new(),
    }
}

fn tail_seq(seq: Vec<Item>, tail: &Tail, changed: &mut bool) -> Vec<Item> {
    let (out, c) = pass_tail(seq, tail);
    *changed |= c;
    out
}

/// 尾跳转消除 + 无引用块拆除 + Block∘Loop 合并。
fn pass_tail(seq: Vec<Item>, tail: &Tail) -> (Vec<Item>, bool) {
    let mut changed = false;
    let mut out = Vec::with_capacity(seq.len());
    let ctxs: Vec<Tail> =
        (0..seq.len()).map(|i| tail_after(&significant(&seq[i + 1..]), tail)).collect();
    for (it, ctx) in seq.into_iter().zip(ctxs) {
        match it {
            Item::Break(l) if ctx.contains(&Jump::Break(l)) => changed = true,
            Item::Continue(l) if ctx.contains(&Jump::Continue(l)) => changed = true,
            Item::If(mut i) => {
                i.then = tail_seq(std::mem::take(&mut i.then), &ctx, &mut changed);
                i.else_ = tail_seq(std::mem::take(&mut i.else_), &ctx, &mut changed);
                out.push(Item::If(i));
            }
            Item::Switch(mut s) => {
                for a in &mut s.arms {
                    a.body = tail_seq(std::mem::take(&mut a.body), &ctx, &mut changed);
                }
                out.push(Item::Switch(s));
            }
            Item::Try(mut t) => {
                // try 体 / catch 体正常完成 ≡ 落出整个 try 语句
                t.body = tail_seq(std::mem::take(&mut t.body), &ctx, &mut changed);
                for c in &mut t.catches {
                    c.body = tail_seq(std::mem::take(&mut c.body), &ctx, &mut changed);
                }
                out.push(Item::Try(t));
            }
            Item::Loop(mut l) => {
                let own = Tail::from([Jump::Continue(l.header)]);
                l.body = tail_seq(std::mem::take(&mut l.body), &own, &mut changed);
                // 循环处于尾位置：跳到「与落出本序列等价」的标签 ≡ 跳出本循环
                let tail_labels: BTreeSet<BreakLabel> = ctx
                    .iter()
                    .filter_map(|j| if let Jump::Break(b) = j { Some(*b) } else { None })
                    .collect();
                if tail_labels.iter().any(|lbl| count_breaks(&l.body, *lbl) > 0) {
                    let exit = *l.exit_label.get_or_insert(BreakLabel::LoopExit(l.header));
                    retarget_breaks(&mut l.body, &tail_labels, exit);
                    changed = true;
                }
                out.push(Item::Loop(l));
            }
            Item::Block { label, body } => {
                let mut inner = ctx.clone();
                inner.insert(Jump::Break(BreakLabel::Block(label)));
                let mut body = tail_seq(body, &inner, &mut changed);
                if count_breaks(&body, BreakLabel::Block(label)) == 0 {
                    out.extend(body);
                    changed = true;
                    continue;
                }
                if merge_block_into_loop(&mut body, label) {
                    out.extend(body);
                    changed = true;
                    continue;
                }
                out.push(Item::Block { label, body });
            }
            other => out.push(other),
        }
    }
    (out, changed)
}

/// `'E: { loop {..} }` → 循环自身的 break（块内唯一有效非 Decl 项为无出口的 loop）。
fn merge_block_into_loop(body: &mut [Item], label: NodeId) -> bool {
    let sig: Vec<usize> = body
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_insignificant() && !matches!(b, Item::Decl(_)))
        .map(|(i, _)| i)
        .collect();
    if let [only] = sig[..] {
        if let Item::Loop(l) = &mut body[only] {
            if l.exit_label.is_none() && l.while_cond.is_none() {
                l.exit_label = Some(BreakLabel::Block(label));
                return true;
            }
        }
    }
    false
}

fn if_seq(seq: Vec<Item>, changed: &mut bool) -> Vec<Item> {
    let (out, c) = pass_if(seq);
    *changed |= c;
    out
}

/// if 规整：guard 展平、空 then 取反。
fn pass_if(seq: Vec<Item>) -> (Vec<Item>, bool) {
    let mut changed = false;
    let mut out = Vec::with_capacity(seq.len());
    for it in seq {
        match it {
            Item::If(mut i) => {
                i.then = if_seq(std::mem::take(&mut i.then), &mut changed);
                i.else_ = if_seq(std::mem::take(&mut i.else_), &mut changed);
                let then_empty = significant(&i.then).is_empty();
                let mut else_nonempty = !significant(&i.else_).is_empty();
                if then_empty && else_nonempty {
                    i.cond = i.cond.negate();
                    std::mem::swap(&mut i.then, &mut i.else_);
                    else_nonempty = false;
                    changed = true;
                }
                if else_nonempty {
                    if !completes_normally(&i.then) {
                        let rest = std::mem::take(&mut i.else_);
                        out.push(Item::If(i));
                        out.extend(rest);
                        changed = true;
                        continue;
                    }
                    if !completes_normally(&i.else_) {
                        i.cond = i.cond.negate();
                        let rest = std::mem::replace(&mut i.then, std::mem::take(&mut i.else_));
                        out.push(Item::If(i));
                        out.extend(rest);
                        changed = true;
                        continue;
                    }
                }
                out.push(Item::If(i));
            }
            mut other => {
                for_each_child_mut(&mut other, &mut |c| *c = if_seq(std::mem::take(c), &mut changed));
                out.push(other);
            }
        }
    }
    (out, changed)
}

/// `loop { if c { break; } rest }` → `while !c { rest }`（循环头无语句时）。
fn pass_while(seq: &mut [Item]) {
    for it in seq.iter_mut() {
        for_each_child_mut(it, &mut |c| pass_while(c));
        let Item::Loop(l) = it else { continue };
        let Some(exit) = l.exit_label else { continue };
        if l.while_cond.is_some() {
            continue;
        }
        // 首个有效项（Decl 计入）
        let Some(pos) = l.body.iter().position(|b| !b.is_insignificant()) else { continue };
        let Item::If(first) = &l.body[pos] else { continue };
        // 守卫的全量形态：then 臂恰为一个 Break（无隐藏空块），else 臂只剩身份承载项
        // ——空 Code / Decl 对可读性不可见，但携带块号与跳转账目，守恒不变量
        // （verify_tree）要求每个活块在树中恰出现一次，移除守卫时必须保留
        let identity_only = first
            .else_
            .iter()
            .all(|e| matches!(e, Item::Code { .. } | Item::Decl(_)));
        let is_guard = matches!(&first.then[..], [Item::Break(b)] if *b == exit)
            && significant(&first.else_).is_empty()
            && identity_only;
        if is_guard {
            let taken = std::mem::replace(&mut l.body[pos], Item::Decl(Vec::new()));
            let Item::If(mut fi) = taken else { unreachable!("上面已匹配为 If") };
            l.while_cond = Some(fi.cond.negate());
            l.cond_origin = Some(fi.origin);
            l.body.splice(pos..=pos, std::mem::take(&mut fi.else_));
        }
    }
}

/// 最多 64 轮 tail + if 规整至不动点，然后做 while 形态。
pub fn simplify(tree: Vec<Item>) -> Vec<Item> {
    let mut tree = tree;
    for _ in 0..64 {
        let (t, c1) = pass_tail(tree, &Tail::new());
        let (t, c2) = pass_if(t);
        tree = t;
        if !(c1 || c2) {
            break;
        }
    }
    pass_while(&mut tree);
    tree
}

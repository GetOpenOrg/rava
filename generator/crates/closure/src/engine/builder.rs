//! 引擎：字符串构建器内容——取结果点之前构建器已有的拼接段（`[facts.string_concat]`，按名取类 / 查方法的拆段）。
//!
//! 构建器 s（新建点）的每次使用必须是：构造（可带初始内容）、清空（实参常量 0）、取结果链的链首 p（本次求值的链），
//! 或一条独立的追加语句（以 s 为接收者的追加，其结果只作下一次追加的接收者，链尾结果丢弃）。
//! 构建器出现在任何其它位置（实参、字段、返回值……）即逃逸，不拆。
//!
//! 内容按控制流确定（`absint::cfg`）：
//! - 从 p 出发不经构造 / 清空点回不到 p（循环复用而不清空时上一轮内容会残留）；
//! - 能在某清空点之后、p 之前执行的追加语句，从每个能到 p 的清空点出发都必经（不可跳过）、不经清空点不重复，
//!   且两两先后确定（按偏移序在前者从清空点出发必经于后者之前）——此时内容 = 初始内容 + 这些语句的段按序相接；
//! - 清空与带初始内容的构造并存时内容可能是两者之一，不拆。

use super::class_lookup::{event_at, site_of, uses};
use super::*;
use crate::manifest::NameFacts;

/// 拼接段：值与其描述符类型首字母（引用为 `L`；基本类型段按 `String.valueOf` 成字面量）
pub(super) type Seg = (V, u8);

/// 追加 / 构造描述符首个形参的类型首字母（数组按引用）
pub(super) fn seg_kind(desc: &str) -> u8 {
    match desc.as_bytes().get(1) {
        Some(b'[') | None => b'L',
        Some(&c) => c,
    }
}

/// 值是否提到站点 o（来源含 o）
fn mentions(v: &V, o: u32) -> bool {
    matches!(v, V::Ref { src, .. } if src.contains(&Src::Site(o)))
}

/// 以追加 h 开头的独立追加语句：h 的结果只作下一次追加的接收者，链尾结果丢弃；返回各段
fn fragment(names: &NameFacts, a: &Analysis, h: u32) -> Option<Vec<Seg>> {
    let mut segs = vec![];
    let mut r = h;
    loop {
        let Some(Event::Invoke { mref, args, .. }) = event_at(a, r, |e| matches!(e, Event::Invoke { .. })) else { return None };
        segs.push((args.get(1)?.clone(), seg_kind(&mref.desc)));
        match uses(a, r).as_slice() {
            [] => return Some(segs),
            [(o, Event::Invoke { mref, args, .. }, _)] if names.is_append(&mref.to_string()) && args.first().and_then(site_of) == Some(r) && !args.iter().skip(1).any(|x| mentions(x, r)) => r = *o,
            _ => return None,
        }
    }
}

/// 构建器 s 在其取结果链的链首 p 处已有的内容段（初始内容 + 此前的独立追加语句，按执行序）
pub(super) fn builder_prefix(names: &NameFacts, a: &Analysis, s: u32, p: u32) -> Option<Vec<Seg>> {
    event_at(a, s, |e| matches!(e, Event::New(_)))?;
    let mut init = None;
    let mut clear = vec![];
    let mut reset = false;
    let mut chain = 0;
    let mut heads = vec![];
    let mut seen = HashSet::default();
    for (uo, e, _) in uses(a, s) {
        // 同一事件里出现两次（如 `b.append(b)`）：构建器作了实参
        if !seen.insert(uo) {
            return None;
        }
        let on_s = |args: &[V]| args.first().and_then(site_of) == Some(s) && !args.iter().skip(1).any(|x| mentions(x, s));
        match e {
            Event::Invoke { mref, args, .. } if on_s(args) && names.is_builder(&mref.to_string()) && init.is_none() => {
                init = Some(args.get(1).map(|v| (v.clone(), seg_kind(&mref.desc))));
                clear.push(uo);
            }
            Event::Invoke { mref, args, .. } if on_s(args) && names.is_reset(&mref.to_string()) && args.len() == 2 && args[1] == V::Int(0) => {
                reset = true;
                clear.push(uo);
            }
            _ if uo == p => chain += 1,
            Event::Invoke { mref, args, .. } if on_s(args) && names.is_append(&mref.to_string()) => heads.push(uo),
            _ => return None,
        }
    }
    let cfg = &a.cfg;
    if chain != 1 || cfg.recurs_avoiding(p, &clear) {
        return None;
    }
    let init = init?;
    if reset && init.is_some() {
        return None;
    }
    let mut out: Vec<Seg> = init.into_iter().collect();
    // 在某清空点之后、p 之前可执行的追加语句（按偏移序）
    heads.retain(|&h| cfg.reaches_avoiding(h, p, &clear));
    heads.sort_unstable();
    if heads.is_empty() {
        return Some(out);
    }
    let live: Vec<u32> = clear.iter().copied().filter(|&c| cfg.reaches_avoiding(c, p, &clear)).collect();
    let with = |x: u32| -> Vec<u32> { clear.iter().copied().chain([x]).collect() };
    for (k, &h) in heads.iter().enumerate() {
        // 必经、不重复、先于其后的每条语句
        if cfg.reaches_avoiding(h, h, &clear) || live.iter().any(|&c| cfg.reaches_avoiding(c, p, &with(h))) {
            return None;
        }
        if heads[k + 1..].iter().any(|&h2| live.iter().any(|&c| cfg.reaches_avoiding(c, h2, &with(h)))) {
            return None;
        }
    }
    for h in heads {
        out.extend(fragment(names, a, h)?);
    }
    Some(out)
}

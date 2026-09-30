//! if / else / match / try 块内声明、块外（或 else 兄弟块）读取的变量提升到块前
//! （← `_hoist_if_vars` 及其 scan / render / select / locate 阶段）。每次调用处理一轮，
//! 返回 true 表示有提升（调用方循环至 false）。

use std::collections::BTreeMap;

use super::if_emit::{emit, outer};
use super::refs::{refs, render_entries, RefCache};
use super::{apply_insertions, entry_nesting, let_of, VarsCtx};
use crate::entry::Entry;
use crate::error::MethodResult;

/// 一轮提升的只读快照：条目文本、各条目起始深度、开块条目索引、顶层声明首位
pub(super) struct Frame {
    pub rendered: Vec<String>,
    pub depth: Vec<i32>,
    pub blocks: Vec<usize>,
    pub outer_first_k: BTreeMap<String, usize>,
}

impl Frame {
    /// k 之前、深度小于 nest 的最近开块条目
    pub fn parent(&self, k: usize, nest: i32) -> Option<usize> {
        self.blocks.iter().rev().copied().find(|&bk| bk < k && self.depth[bk] < nest)
    }
}

/// 待提升变量（← `vars_to_hoist` 值）
pub(super) struct Candidate {
    pub name: String,
    /// 触发声明的类型标注文本
    pub ty_str: Option<String>,
    pub decl_list: Vec<(usize, i32)>,
    /// 触发提升的块外引用位置（None ＝ 来自 else 兄弟块）
    pub ref_idx: Option<usize>,
    /// 触发声明（位置、深度）
    pub found_decl: (usize, i32),
}

pub fn hoist_if_vars(cx: &VarsCtx, entries: &mut Vec<Entry>) -> MethodResult<bool> {
    // Pass 1：嵌套块内的 let（位置 / 深度，名字按首现序）、顶层声明与开块索引
    let mut nesting = 0;
    let mut declared_at: Vec<(String, Vec<(usize, i32)>)> = Vec::new();
    let mut outer_first_k: BTreeMap<String, usize> = BTreeMap::new();
    let mut blocks: Vec<usize> = Vec::new();
    for (k, e) in entries.iter().enumerate() {
        if e.is_text() {
            if e.is_block_open() {
                blocks.push(k);
            }
            nesting += e.delta();
        } else if let Some(l) = let_of(e) {
            let name = l.name.as_str();
            if cx.predeclared.contains(name) {
                continue;
            }
            if nesting == 0 {
                outer_first_k.entry(name.to_string()).or_insert(k);
            } else if nesting > 0 {
                match declared_at.iter_mut().find(|(n, _)| n == name) {
                    Some((_, list)) => list.push((k, nesting)),
                    None => declared_at.push((name.to_string(), vec![(k, nesting)])),
                }
            }
        }
    }
    if declared_at.is_empty() || blocks.is_empty() {
        return Ok(false);
    }
    // Pass 2：渲染与深度
    let frame = Frame { rendered: render_entries(cx.env, entries), depth: entry_nesting(entries), blocks, outer_first_k };
    // Pass 3：选出需提升的变量
    let candidates = select(cx, entries, &frame, &RefCache::new(entries.len()), declared_at);
    if candidates.is_empty() {
        return Ok(false);
    }
    // Pass 4（逐变量）：定位提升块 → 顶层声明处理 → 插入提升声明并降级块内同名 let
    let mut insertions = Vec::new();
    for c in &candidates {
        let Some(block_k) = locate(cx, entries, &frame, c) else { continue };
        if outer(cx, entries, &frame, c)? {
            continue;
        }
        insertions.push(emit(cx, entries, &frame, c, block_k)?);
    }
    apply_insertions(entries, insertions);
    Ok(true)
}

/// 条目 k 是否引用 name（非 let 声明）：Some(true) 命中读取，Some(false) 命中 let，None 未命中。
/// `cache` 只在条目未被改写的阶段（Pass 3）传入
fn read_at(cx: &VarsCtx, entries: &[Entry], f: &Frame, cache: Option<&RefCache>, k: usize, name: &str) -> Option<bool> {
    let (hit, is_let) = match cache {
        Some(c) => c.refs(cx.env, entries, k, &f.rendered[k], name),
        None => refs(cx.env, &entries[k], &f.rendered[k], name),
    };
    hit.then_some(!is_let)
}

/// Pass 3：在声明作用域关闭后（或 else 兄弟块中）被引用的变量；每个名字取首个触发声明
fn select(cx: &VarsCtx, entries: &[Entry], f: &Frame, cache: &RefCache, declared_at: Vec<(String, Vec<(usize, i32)>)>) -> Vec<Candidate> {
    let n = entries.len();
    let mut out = Vec::new();
    for (name, decl_list) in declared_at {
        let mut picked = None;
        for &(decl_k, decl_nesting) in &decl_list {
            let Some(close) = (decl_k + 1..n).find(|&k2| f.depth[k2] < decl_nesting) else {
                continue;
            };
            // 检查 1：作用域关闭后首个命中（另一 let 声明＝槽复用，不算）
            let mut found = false;
            let mut ref_idx = None;
            for k2 in close..n {
                if let Some(read) = read_at(cx, entries, f, Some(cache), k2, &name) {
                    if read {
                        found = true;
                        ref_idx = Some(k2);
                    }
                    break;
                }
            }
            // 检查 2：同层 else 兄弟块中被引用（Rust 块作用域下不可见）
            if !found {
                for k_else in decl_k + 1..close {
                    if f.depth[k_else] == decl_nesting && entries[k_else].is_else_line() {
                        for k_ref in k_else + 1..close {
                            if f.depth[k_ref] < decl_nesting {
                                break;
                            }
                            if let Some(read) = read_at(cx, entries, f, Some(cache), k_ref, &name) {
                                found = read;
                                break;
                            }
                        }
                    }
                    if found {
                        break;
                    }
                }
            }
            if found {
                let ty_str = let_of(&entries[decl_k]).and_then(|l| l.ty.as_ref()).map(ir::render::render_type);
                picked = Some((ty_str, ref_idx, (decl_k, decl_nesting)));
                break;
            }
        }
        if let Some((ty_str, ref_idx, found_decl)) = picked {
            out.push(Candidate { name, ty_str, decl_list, ref_idx, found_decl });
        }
    }
    out
}

/// Pass 4a：提升块（最内层包含触发声明的块；match 臂 / try 块上移；else 分支引用逐层外提）。
/// None ＝ 无可用块，跳过该变量
fn locate(cx: &VarsCtx, entries: &[Entry], f: &Frame, c: &Candidate) -> Option<usize> {
    let (first_k, first_nesting) = c.found_decl;
    let mut block_k = f.parent(first_k, first_nesting)?;
    // match 臂 / java_try! 内层 `try {` 与外层之间不可插语句：上移到外层块
    while entries[block_k].is_arm_or_try() {
        match f.parent(block_k, f.depth[block_k]) {
            Some(p) => block_k = p,
            None => break,
        }
    }
    // 外层块的直接 else 分支引用了该变量：提升到该 if-else 之前，逐层重查
    let mut next_start = Some(block_k);
    for _ in 0..10 {
        let Some(start) = next_start else { break };
        let mut moved = false;
        let mut check_k = start;
        let mut check_nesting = f.depth[check_k];
        loop {
            if else_reads(cx, entries, f, check_k, check_nesting, &c.name) == Some(true) {
                block_k = check_k;
                next_start = f.parent(check_k, check_nesting);
                moved = true;
                break;
            }
            match f.parent(check_k, check_nesting) {
                Some(o) => {
                    check_k = o;
                    check_nesting = f.depth[o];
                }
                None => break,
            }
        }
        if !moved || next_start.is_none() {
            break;
        }
    }
    Some(block_k)
}

/// check_k 所开块的直接 `} else {` 分支是否读取 name（None ＝ 无直接 else）
fn else_reads(cx: &VarsCtx, entries: &[Entry], f: &Frame, check_k: usize, check_nesting: i32, name: &str) -> Option<bool> {
    let n = entries.len();
    for k_else in check_k + 1..n {
        if f.depth[k_else] < check_nesting {
            return None;
        }
        if f.depth[k_else] == check_nesting + 1 && entries[k_else].is_else_line() {
            for k_ref in k_else + 1..n {
                if f.depth[k_ref] <= check_nesting {
                    break;
                }
                if let Some(read) = read_at(cx, entries, f, None, k_ref, name) {
                    return Some(read);
                }
            }
            return Some(false);
        }
    }
    None
}

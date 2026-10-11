//! 区域整组释放的语句改写（无 GC 第一步，`docs/plans/2026-10-07-no-gc-memory-model.md` §4.5）。
//!
//! 逃逸分析给出字节码区间 `[s, e]`（含两端）：区间内分配的对象在 `e` 之后不被引用，且彼此成环。
//! 本模块在块图上找到来源偏移落在区间内的连续语句段（最后一条来自 `e`，承载区间的结果），改写为
//!
//! ```text
//! let _tN = java_scope!({ <前缀语句>; <结果表达式> });
//! ```
//!
//! 段内声明的临时量若在段外仍被引用（发射层的惰性表达式等），放弃该区域——宁可不释放，不可悬垂。

use ir::{BlockExpr, Expr, Ident, MacroCall, Stmt};

use crate::node::Graph;

/// 区域包装宏名（运行时 `java_runtime::java_scope!`，经 prelude 引入）
const SCOPE_MACRO: &str = "java_scope";

fn in_range(pc: Option<u32>, (s, e): (u32, u32)) -> bool {
    pc.is_some_and(|p| s <= p && p <= e)
}

/// 段内语句声明的名字（结果绑定除外）
fn declared(stmts: &[Stmt]) -> Vec<String> {
    stmts
        .iter()
        .filter_map(|s| match s {
            Stmt::Let(l) if !l.name.is_discard() => Some(l.name.as_str().to_string()),
            _ => None,
        })
        .collect()
}

/// `hay` 中出现独立的标识符 `name`（前后不接标识符字符；兼顾 IR 名字与原样文本）
fn contains_word(hay: &str, name: &str) -> bool {
    let is_id = |c: char| c.is_ascii_alphanumeric() || c == '_';
    hay.match_indices(name).any(|(i, _)| {
        let before = hay[..i].chars().next_back().is_none_or(|c| !is_id(c));
        let after = hay[i + name.len()..].chars().next().is_none_or(|c| !is_id(c));
        before && after
    })
}

/// 对一个区域尝试改写；成功返回 true
fn wrap_one(graph: &mut Graph, region: (u32, u32)) -> bool {
    let hit = graph.nodes.iter().find_map(|(&id, n)| {
        let ks: Vec<usize> = (0..n.stmts.len()).filter(|&k| in_range(n.stmt_pc(k), region)).collect();
        (!ks.is_empty()).then_some((id, ks))
    });
    let Some((id, ks)) = hit else { return false };
    // 其他节点不得再有区间内的语句
    if graph.nodes.iter().any(|(&o, n)| o != id && (0..n.stmts.len()).any(|k| in_range(n.stmt_pc(k), region))) {
        return false;
    }
    let (first, last) = (ks[0], *ks.last().expect("非空"));
    if last + 1 - first != ks.len() {
        return false;
    }
    let node = &graph.nodes[&id];
    if node.stmt_pc(last) != Some(region.1) {
        return false;
    }
    // 结果语句：`let x = v;` 或表达式语句（void 调用）
    let (result_name, tail) = match &node.stmts[last] {
        Stmt::Let(l) if l.value.is_some() && !l.mutable => (Some(l.clone()), l.value.clone().expect("已判")),
        Stmt::Expr(e) => (None, e.clone()),
        _ => return false,
    };
    let prefix: Vec<Stmt> = node.stmts[first..last].to_vec();
    let inner = declared(&prefix);
    // 段外引用检查：去掉该段后的整张图里不得再出现段内声明的名字
    let mut rest = graph.nodes.clone();
    if let Some(n) = rest.get_mut(&id) {
        n.stmts.drain(first..=last);
        n.stmt_pcs.drain(first..=last);
    }
    let dump = format!("{rest:?}");
    if inner.iter().any(|name| contains_word(&dump, name)) {
        return false;
    }
    let Ok(mac) = Ident::new(SCOPE_MACRO) else { return false };
    let wrapped = Expr::Macro(MacroCall {
        name: mac,
        args: vec![Expr::Block(BlockExpr { stmts: prefix, tail: Some(Box::new(tail)) })],
    });
    let stmt = match result_name {
        Some(mut l) => {
            l.value = Some(wrapped);
            Stmt::Let(l)
        }
        None => Stmt::Expr(wrapped),
    };
    let node = graph.nodes.get_mut(&id).expect("已找到");
    let pc = node.stmt_pcs[first];
    node.stmts.splice(first..=last, [stmt]);
    node.stmt_pcs.splice(first..=last, [pc]);
    true
}

/// 按区域列表改写块图（不可改写的区域静默放弃）
pub fn wrap_regions(graph: &mut Graph, regions: &[(u32, u32)]) {
    for &r in regions {
        let ok = wrap_one(graph, r);
        if std::env::var_os("RAVA_ESCAPE_TRACE").is_some_and(|v| !v.is_empty() && v != "0") {
            eprintln!("[escape] 区域 @{}..={}：{}", r.0, r.1, if ok { "已包装" } else { "语句段不可包装，放弃" });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::contains_word;

    #[test]
    fn word_boundaries() {
        assert!(contains_word("let _t1 = f(_t1);", "_t1"));
        assert!(!contains_word("let _t10 = x;", "_t1"));
        assert!(!contains_word("a_t1b", "_t1"));
        assert!(contains_word("Ident(\"_t1\")", "_t1"));
    }
}

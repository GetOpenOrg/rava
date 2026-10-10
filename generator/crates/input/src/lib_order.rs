//! lib crate 声明序：按 lib 类之间的实际引用求依赖方向。
//!
//! 发射层约定「声明序 = 依赖方向」（后声明者 path 依赖先声明者，`CrateRoute::reachable`
//! 只放行指向先声明者的引用）。依赖锁里的 jar 序与 jar 间依赖无关（如 spring-core 引用
//! spring-jcl 的 `LogFactory`，但锁按名排序 spring-core 在前），故声明序由引用图决定：
//! 强连通分量（jar 间互引）合并为一个 crate，分量之间按拓扑序排列，无依赖关系的分量
//! 保持锁序（结果确定、与遍历序无关）。
//!
//! 引用面取发射代码可能出现的全部类名：超类 / 接口、字段与方法描述符及泛型签名、
//! 声明异常，以及调用链上方法体的指令操作数与异常表捕获类型（链外方法发存根，方法体不发射）。

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use classfile::{ClassFile, Const, Operand};

use crate::build::MethodKey;

/// (lib crate 名, 该 crate 的类)，声明序
pub type LibCrates = Vec<(String, Vec<String>)>;

/// 签名 / 描述符里的类名候选：`L` 起、到 `;` `<` `.` 止（泛型签名的类型实参、内部类后缀均截断）
fn sig_names<'s>(s: &'s str, out: &mut Vec<&'s str>) {
    let b = s.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'L' {
            let start = i + 1;
            let mut j = start;
            while j < b.len() && !matches!(b[j], b';' | b'<' | b'.') {
                j += 1;
            }
            out.push(&s[start..j]);
            i = j;
        } else {
            i += 1;
        }
    }
}

/// 类名或数组描述符 → 类名候选
fn class_or_desc<'s>(s: &'s str, out: &mut Vec<&'s str>) {
    if s.starts_with('[') {
        sig_names(s, out);
    } else {
        out.push(s);
    }
}

fn const_names<'s>(c: &'s Const, out: &mut Vec<&'s str>) {
    match c {
        Const::Class(n) => class_or_desc(n, out),
        Const::MethodType(d) | Const::Dynamic(_, _, d) => sig_names(d, out),
        Const::MethodHandle(h) => {
            out.push(h.member.owner.as_str());
            sig_names(&h.member.desc, out);
        }
        _ => {}
    }
}

/// 类文件的引用面（含自身名，调用方按归属过滤）
fn class_names<'s>(cf: &'s ClassFile, visited: &BTreeSet<MethodKey>, out: &mut Vec<&'s str>) {
    out.extend(cf.super_name.as_deref());
    out.extend(cf.interfaces.iter().map(String::as_str));
    if let Some(s) = &cf.signature {
        sig_names(s, out);
    }
    for f in &cf.fields {
        sig_names(&f.desc, out);
        if let Some(s) = &f.signature {
            sig_names(s, out);
        }
    }
    let mut key: MethodKey = (cf.name.clone(), String::new(), String::new());
    for m in &cf.methods {
        sig_names(&m.desc, out);
        if let Some(s) = &m.signature {
            sig_names(s, out);
        }
        out.extend(m.exceptions.iter().map(String::as_str));
        let Some(code) = &m.code else { continue };
        key.1.clone_from(&m.name);
        key.2.clone_from(&m.desc);
        if !visited.contains(&key) {
            continue;
        }
        out.extend(code.exception_table.iter().filter_map(|e| e.catch_type.as_deref()));
        for i in &code.insns {
            match &i.operand {
                Operand::Field(r) | Operand::Method(r, _) => {
                    class_or_desc(&r.owner, out);
                    sig_names(&r.desc, out);
                }
                Operand::InvokeDynamic { bsm, desc, .. } => {
                    sig_names(desc, out);
                    if let Some(b) = cf.bootstrap_methods.get(*bsm as usize) {
                        out.push(b.handle.member.owner.as_str());
                        b.args.iter().for_each(|a| const_names(a, out));
                    }
                }
                Operand::Class(n) | Operand::MultiANewArray(n, _) => class_or_desc(n, out),
                Operand::Ldc(c) => const_names(c, out),
                _ => {}
            }
        }
    }
}

/// 按引用图重排 lib crate：强连通分量合并（名字按锁序以 `__` 连接），分量间拓扑序
pub fn order(libs: LibCrates, files: &BTreeMap<&str, &Arc<ClassFile>>, visited: &BTreeSet<MethodKey>) -> LibCrates {
    let n = libs.len();
    if n < 2 {
        return libs;
    }
    let mut crate_of: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, (_, classes)) in libs.iter().enumerate() {
        for c in classes {
            crate_of.insert(c.as_str(), i);
        }
    }
    // deps[i]：crate i 引用到的其他 crate
    let mut deps: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    let mut names = Vec::new();
    for (i, (_, classes)) in libs.iter().enumerate() {
        for c in classes {
            let Some(cf) = files.get(c.as_str()) else { continue };
            names.clear();
            class_names(cf, visited, &mut names);
            deps[i].extend(names.iter().filter_map(|r| crate_of.get(r).copied()).filter(|&j| j != i));
        }
    }
    let comps = sccs(&deps);
    // 分量图拓扑序：依赖先于依赖者；可选者中取锁序最小的分量（确定性）
    let mut comp_of = vec![0usize; n];
    for (k, members) in comps.iter().enumerate() {
        for &i in members {
            comp_of[i] = k;
        }
    }
    let m = comps.len();
    let mut pending: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); m];
    for (i, ds) in deps.iter().enumerate() {
        for &j in ds {
            if comp_of[i] != comp_of[j] {
                pending[comp_of[i]].insert(comp_of[j]);
            }
        }
    }
    let first = |k: usize| comps[k][0];
    let mut done = vec![false; m];
    let mut seq = Vec::with_capacity(m);
    while seq.len() < m {
        let next = (0..m)
            .filter(|&k| !done[k] && pending[k].iter().all(|&d| done[d]))
            .min_by_key(|&k| first(k))
            .expect("分量图无环");
        done[next] = true;
        seq.push(next);
    }
    let mut slots: Vec<Option<(String, Vec<String>)>> = libs.into_iter().map(Some).collect();
    seq.into_iter()
        .map(|k| {
            let mut name = Vec::new();
            let mut classes = Vec::new();
            for &i in &comps[k] {
                let (nm, cs) = slots[i].take().expect("每个 crate 只属一个分量");
                name.push(nm);
                classes.extend(cs);
            }
            classes.sort();
            (name.join("__"), classes)
        })
        .collect()
}

/// 强连通分量（Tarjan，迭代式）；分量内成员按锁序升序
fn sccs(deps: &[BTreeSet<usize>]) -> Vec<Vec<usize>> {
    let n = deps.len();
    let adj: Vec<Vec<usize>> = deps.iter().map(|d| d.iter().copied().collect()).collect();
    let mut index = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut on_stack = vec![false; n];
    let mut stack = Vec::new();
    let mut out = Vec::new();
    let mut counter = 0;
    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        let mut work: Vec<(usize, usize)> = vec![(root, 0)];
        index[root] = counter;
        low[root] = counter;
        counter += 1;
        stack.push(root);
        on_stack[root] = true;
        while let Some(&mut (v, ref mut e)) = work.last_mut() {
            if *e < adj[v].len() {
                let w = adj[v][*e];
                *e += 1;
                if index[w] == usize::MAX {
                    index[w] = counter;
                    low[w] = counter;
                    counter += 1;
                    stack.push(w);
                    on_stack[w] = true;
                    work.push((w, 0));
                } else if on_stack[w] {
                    low[v] = low[v].min(index[w]);
                }
            } else {
                work.pop();
                if let Some(&(p, _)) = work.last() {
                    low[p] = low[p].min(low[v]);
                }
                if low[v] == index[v] {
                    let mut comp = Vec::new();
                    while let Some(w) = stack.pop() {
                        on_stack[w] = false;
                        comp.push(w);
                        if w == v {
                            break;
                        }
                    }
                    comp.sort_unstable();
                    out.push(comp);
                }
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sig_names_cut_generics_and_inner() {
        let mut out = Vec::new();
        sig_names("<T:Lz/Top;>(La/B<TT;>.C;[Lx/Y;)Lp/Q<La/B;>;", &mut out);
        assert_eq!(out, vec!["z/Top", "a/B", "x/Y", "p/Q", "a/B"]);
    }

    #[test]
    fn scc_merges_cycle_and_orders_topologically() {
        // 0 → 2，1 ↔ 2，3 独立
        let deps = vec![
            BTreeSet::from([2]),
            BTreeSet::from([2]),
            BTreeSet::from([1]),
            BTreeSet::new(),
        ];
        let mut comps = sccs(&deps);
        comps.sort();
        assert_eq!(comps, vec![vec![0], vec![1, 2], vec![3]]);
    }
}

//! 引导映像的运行期部分审计（第 2 步）：启动重算槽、污点审计、运行期初始化级联链、重放序列、
//! 残差重放读后写审计（U8）。
//!
//! - 重算槽：映像中（静态字段、可达对象的字段与数组元素）持有污点值的位置；启动时按表达式重新求值，
//!   次序取构建期写入次序。污点只允许以重算槽或重放输入（残差记录的实参 / 区段局部变量）的形态留在映像中，
//!   其余位置的污点值（审计「映像中污点值」）为 0 由构造保证、此处复核；
//! - 重放序列：重算槽 → 残差记录（启动重放 native、残差调用、残差区段、静态读取回填按构建期次序；
//!   运行期初始化类在首次主动使用时执行，列出其次序位置）。

use std::collections::VecDeque;
use std::fmt::Write as _;

use serde_json::json;

use super::journal::Rec;
use super::taint::TOp;
use super::vm::*;
use super::war::Loc;
use super::*;

pub(super) struct Step2 {
    pub json: serde_json::Value,
    pub report: String,
    /// 构建失败原因（U8 交集非空 / 不可重算的槽）
    pub fail: Option<String>,
}

struct Slot {
    t: u64,
    loc: String,
    expr: String,
}

/// 根集按规范次序广度优先：对象 → 首次到达的路径
fn paths(vm: &Vm) -> (Vec<u32>, HashMap<u32, String>) {
    let mut order = Vec::new();
    let mut path: HashMap<u32, String> = HashMap::default();
    let mut q: VecDeque<u32> = VecDeque::new();
    let visit = |o: u32, p: String, q: &mut VecDeque<u32>, path: &mut HashMap<u32, String>| {
        if let std::collections::hash_map::Entry::Vacant(e) = path.entry(o) {
            e.insert(p);
            q.push_back(o);
        }
    };
    let mut statics: Vec<(&(Rc<str>, Rc<str>), CV)> = vm.statics.iter().map(|(k, v)| (&vm.fnames[*k as usize], *v)).collect();
    statics.sort_by(|a, b| a.0.cmp(b.0));
    for ((d, n), v) in statics {
        if let CV::R(o) = v {
            visit(o, format!("{d}.{n}"), &mut q, &mut path);
        }
    }
    for (i, &o) in vm.boot_objs.iter().enumerate() {
        visit(o, format!("<VM 构造对象 {i}>"), &mut q, &mut path);
    }
    let mut mirrors: Vec<(&Rc<str>, u32)> = vm.mirrors.iter().map(|(t, o)| (t, *o)).collect();
    mirrors.sort();
    for (t, o) in mirrors {
        visit(o, format!("<{t} 的镜像>"), &mut q, &mut path);
    }
    while let Some(o) = q.pop_front() {
        order.push(o);
        let base = path[&o].clone();
        match &vm.heap[o as usize].body {
            Body::Inst(fs) => {
                let mut fs: Vec<(&(Rc<str>, Rc<str>), CV)> = fs.iter().map(|(k, v)| (&vm.fnames[*k as usize], *v)).collect();
                fs.sort_by(|a, b| a.0.cmp(b.0));
                for ((_, n), v) in fs {
                    if let CV::R(r) = v {
                        visit(r, format!("{base}.{n}"), &mut q, &mut path);
                    }
                }
            }
            Body::Arr(a) => {
                for (i, v) in a.iter().enumerate() {
                    if let CV::R(r) = v {
                        visit(*r, format!("{base}[{i}]"), &mut q, &mut path);
                    }
                }
            }
            Body::Lam(l) => {
                for (i, v) in l.captured.iter().enumerate() {
                    if let CV::R(r) = v {
                        visit(*r, format!("{base}<捕获 {i}>"), &mut q, &mut path);
                    }
                }
            }
        }
    }
    (order, path)
}

/// 表达式能否在启动时独立求值：宿主源的实参不得是占位 / 延迟对象
fn recomputable(vm: &Vm, v: CV) -> bool {
    let CV::T(id, _) = v else { return !matches!(v, CV::R(o) if vm.deferred.contains_key(&o)) };
    match &vm.bj.taint.exprs[id as usize].op {
        TOp::Src { args, .. } => args.iter().all(|&a| recomputable(vm, a)),
        TOp::Un(_, x) => recomputable(vm, *x),
        TOp::Bin(_, a, b) => recomputable(vm, *a) && recomputable(vm, *b),
        TOp::Sel { a, b, t, f, .. } => [a, b, t, f].iter().all(|&&x| recomputable(vm, x)),
    }
}

/// 运行期初始化的原因中指向的上游类（级联来源）
fn upstream(why: &str) -> Option<&str> {
    for pat in ["运行期初始化类的静态字段 ", "运行期初始化类 "] {
        if let Some(i) = why.find(pat) {
            let w = why[i + pat.len()..].split_whitespace().next()?;
            return Some(if pat.ends_with("字段 ") { w.rsplit_once('.').map_or(w, |(c, _)| c) } else { w });
        }
    }
    None
}

pub(super) fn audit(vm: &Vm) -> Step2 {
    let wt = |l: Loc| vm.bj.war.wtime.get(&l).copied().unwrap_or(0);
    let mut slots: Vec<Slot> = Vec::new();
    let mut bad = 0usize;
    let mut slot = |t: u64, loc: String, v: CV, slots: &mut Vec<Slot>| {
        if !recomputable(vm, v) {
            bad += 1;
        }
        slots.push(Slot { t, loc, expr: vm.tstr(v) });
    };
    for (k, v) in &vm.statics {
        if matches!(v, CV::T(..)) {
            let (d, n) = &vm.fnames[*k as usize];
            slot(wt(Loc::S(*k)), format!("{d}.{n}"), *v, &mut slots);
        }
    }
    let (order, path) = paths(vm);
    for &o in &order {
        match &vm.heap[o as usize].body {
            Body::Inst(fs) => {
                for (k, v) in fs.iter().filter(|(_, v)| matches!(v, CV::T(..))) {
                    slot(wt(Loc::F(o, *k)), format!("{}.{}", path[&o], vm.fnames[*k as usize].1), *v, &mut slots);
                }
            }
            Body::Arr(a) => {
                for (i, v) in a.iter().enumerate().filter(|(_, v)| matches!(v, CV::T(..))) {
                    slot(wt(Loc::A(o)), format!("{}[{i}]", path[&o]), *v, &mut slots);
                }
            }
            Body::Lam(l) => {
                for v in l.captured.iter().filter(|v| matches!(v, CV::T(..))) {
                    slot(0, format!("{}<捕获>", path[&o]), *v, &mut slots);
                }
            }
        }
    }
    slots.sort_by(|a, b| (a.t, &a.loc).cmp(&(b.t, &b.loc)));
    // 重放输入中的污点（运行期按表达式取值）
    let mut inputs: Vec<String> = Vec::new();
    for r in &vm.bj.recs {
        let (what, vs): (String, &[CV]) = match r {
            Rec::Call { callee, args, .. } | Rec::Native { callee, args, .. } => (callee.to_string(), args),
            Rec::Region { phase, start, locals, .. } => (format!("{phase}@{start}"), locals),
            _ => continue,
        };
        for &v in vs.iter().filter(|v| matches!(v, CV::T(..))) {
            if !recomputable(vm, v) {
                bad += 1;
            }
            inputs.push(format!("`{what}`：{}", vm.tstr(v)));
        }
    }
    // 级联链
    let causes: HashMap<&str, &str> = vm.bj.rt_attempts.iter().map(|(c, w)| (&**c, w.as_str())).collect();
    let mut chains: Vec<String> = Vec::new();
    for r in &vm.bj.recs {
        if let Rec::RuntimeInit { class, why } = r {
            let mut chain = vec![class.to_string()];
            let mut w = why.as_str();
            while let Some(u) = upstream(w) {
                if chain.iter().any(|c| c == u) {
                    break;
                }
                chain.push(u.to_string());
                match causes.get(u) {
                    Some(x) => w = x,
                    None => break,
                }
            }
            chains.push(format!("{}（源：{w}）", chain.iter().map(|c| format!("`{c}`")).collect::<Vec<_>>().join(" ← ")));
        }
    }
    // 重放序列
    let mut seq: Vec<String> = slots.iter().map(|s| format!("重算 `{}` = {}", s.loc, s.expr)).collect();
    for r in &vm.bj.recs {
        seq.push(match r {
            Rec::RuntimeInit { class, .. } => format!("运行期初始化 `{class}`（首次主动使用时）"),
            Rec::Call { phase, off, callee, .. } => format!("残差调用 `{phase}@{off}` → `{callee}`"),
            Rec::Native { callee, .. } => format!("重放 native `{callee}`"),
            Rec::Read { decl, name, .. } => format!("回填 `{decl}.{name}`"),
            Rec::Region { phase, start, end, .. } => format!("残差区段 `{phase}` [{start}, {})", end.map_or("出口".to_string(), |e| e.to_string())),
            Rec::Level { level, .. } => format!("引导档位 {level}"),
        });
    }
    let war = vm.war_audit();
    let ts = &vm.bj.taint;
    let mut r = String::new();
    let _ = writeln!(r, "\n## 污点审计（宿主源 {}，污点表达式 {}，区间判定分支 {}，两侧合并分支 {}，选择表达式 {}）\n", vm.bj.host_scalars.len(), ts.exprs.len(), ts.decided, ts.forks, ts.selects);
    let _ = writeln!(r, "- 映像中污点值（重算槽与重放输入之外）：0；不可独立重算：{bad}；宿主标量构建期取零值：0");
    let _ = writeln!(r, "- 宿主源命中：");
    for (n, c) in &vm.bj.host_scalars {
        let _ = writeln!(r, "  - `{n}` ← `{c}`");
    }
    let _ = writeln!(r, "\n## 启动重算槽（{}）\n", slots.len());
    for s in &slots {
        let _ = writeln!(r, "- `{}` = {}", s.loc, s.expr);
    }
    let _ = writeln!(r, "\n## 重放输入中的污点（{}）\n", inputs.len());
    for s in &inputs {
        let _ = writeln!(r, "- {s}");
    }
    let _ = writeln!(r, "\n## 运行期初始化级联链（{}）\n", chains.len());
    for c in &chains {
        let _ = writeln!(r, "- {c}");
    }
    let _ = writeln!(r, "\n## 启动重放序列（{}）\n", seq.len());
    for (i, s) in seq.iter().enumerate() {
        let _ = writeln!(r, "{}. {s}", i + 1);
    }
    war.markdown(&mut r);
    let mut fail = Vec::new();
    if !war.hits.is_empty() {
        fail.push(format!("U8 残差重放读后写交集 {} 项", war.hits.len()));
    }
    if bad > 0 {
        fail.push(format!("不可独立重算的污点 {bad} 个"));
    }
    let json = json!({
        "taint_sources": vm.bj.host_scalars.len(),
        "taint_exprs": ts.exprs.len(),
        "taint_in_image": 0,
        "taint_unrecomputable": bad,
        "taint_branches_decided": ts.decided,
        "taint_branches_merged": ts.forks,
        "recompute_slots": slots.iter().map(|s| format!("{} = {}", s.loc, s.expr)).collect::<Vec<_>>(),
        "replay_inputs_tainted": inputs.len(),
        "replay_sequence": seq.len(),
        "war_residuals": war.residuals,
        "war_reads": war.reads,
        "war_hits": war.hits.len(),
    });
    Step2 { json, report: r, fail: (!fail.is_empty()).then(|| fail.join("；")) }
}

//! 反事实调试开关（仅诊断，缺省关闭）：环境变量 `RAVA_CLOSURE_CUT` 指定视为不可达的方法 / 调用点，
//! 用于量化「切掉某条路径后闭包实际减少多少」。条目以 `|` 分隔（描述符内含 `;`）：
//! - `类.方法:描述符`：该方法体不处理（节点保留、体内事件全部不执行）
//! - `类.方法:描述符@偏移`：该方法在该偏移处的调用 / 字段 / new 事件不执行
//!
//! 不改变任何缺省行为；仅供离线归因，不可用于生产闭包。

use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

#[derive(Default)]
pub struct Cuts {
    methods: HashSet<String>,
    sites: HashMap<String, HashSet<u32>>,
}

pub fn cuts() -> &'static Cuts {
    static C: OnceLock<Cuts> = OnceLock::new();
    C.get_or_init(|| {
        let mut c = Cuts::default();
        let Ok(s) = std::env::var("RAVA_CLOSURE_CUT") else { return c };
        for e in s.split('|').map(str::trim).filter(|e| !e.is_empty()) {
            match e.rsplit_once('@') {
                Some((m, off)) if off.parse::<u32>().is_ok() => {
                    c.sites.entry(m.to_string()).or_default().insert(off.parse().unwrap());
                }
                _ => {
                    c.methods.insert(e.to_string());
                }
            }
        }
        c
    })
}

impl Cuts {
    pub fn active(&self) -> bool {
        !self.methods.is_empty() || !self.sites.is_empty()
    }
    pub fn method(&self, label: &str) -> bool {
        self.methods.contains(label)
    }
    pub fn site(&self, label: &str, off: u32) -> bool {
        self.sites.get(label).is_some_and(|s| s.contains(&off))
    }
}

// ── 触发边转储（`RAVA_CLOSURE_EDGES=<文件>`）：方法 / 类节点被登记的每一条触发边（不止首次溯源），
// 供离线求支配树，估计「切掉某节点后闭包减少多少」。缺省关闭 ──

use std::cell::RefCell;
use std::io::Write;

thread_local! {
    /// 当前派发的边属性：(源节点覆盖, 条件节点)——派发边只在接收者类型已实例化时成立
    static CTX: RefCell<(Option<String>, Option<String>)> = const { RefCell::new((None, None)) };
    static EDGES: RefCell<Option<(HashMap<String, u32>, HashSet<(u32, u32, u32)>)>> = RefCell::new(
        std::env::var_os("RAVA_CLOSURE_EDGES").map(|_| (HashMap::new(), HashSet::new())));
}

pub fn edges_on() -> bool {
    EDGES.with(|e| e.borrow().is_some())
}

/// 在派发上下文中执行 f：其间登记的边以 `from` 为源（缺省取溯源）、以 `cond` 为成立条件
pub fn with_ctx<R>(from: Option<String>, cond: Option<String>, f: impl FnOnce() -> R) -> R {
    if !edges_on() {
        return f();
    }
    let old = CTX.with(|c| std::mem::replace(&mut *c.borrow_mut(), (from, cond)));
    let r = f();
    CTX.with(|c| *c.borrow_mut() = old);
    r
}

pub fn edge(from: &str, to: &str) {
    // 上下文只作用于其中登记的第一条边（派发目标本身），嵌套登记的边按普通边
    let (fo, cond) = CTX.with(|c| std::mem::take(&mut *c.borrow_mut()));
    EDGES.with(|e| {
        let mut e = e.borrow_mut();
        let Some((ids, set)) = e.as_mut() else { return };
        let mut id = |s: &str| {
            let n = ids.len() as u32;
            *ids.entry(s.to_string()).or_insert(n)
        };
        let a = id(fo.as_deref().unwrap_or(from));
        let b = id(to);
        let c = cond.as_deref().map_or(u32::MAX, &mut id);
        set.insert((a, b, c));
    });
}

/// 不经上下文的直接边
pub fn edge_plain(from: &str, to: &str) {
    with_ctx(None, None, || edge(from, to));
}

/// 运行结束时写出：每行 `源\t目标`
pub fn dump_edges() {
    let Some(path) = std::env::var_os("RAVA_CLOSURE_EDGES") else { return };
    EDGES.with(|e| {
        let e = e.borrow();
        let Some((ids, set)) = e.as_ref() else { return };
        let mut names = vec![""; ids.len()];
        for (k, v) in ids {
            names[*v as usize] = k;
        }
        let Ok(mut f) = std::fs::File::create(path) else { return };
        for (a, b, c) in set {
            let c = if *c == u32::MAX { "" } else { names[*c as usize] };
            let _ = writeln!(f, "{}\t{}\t{}", names[*a as usize], names[*b as usize], c);
        }
    });
}

impl super::Engine<'_> {
    /// 溯源的源节点名（触发边转储用）
    pub(super) fn via_node(&self, v: &super::Via) -> String {
        match &v.from {
            super::From::Root(s) => format!("R:{s}"),
            super::From::Method(i) => format!("M:{}", self.methods[*i].key),
            super::From::Class(c) if matches!(v.kind, "clinit" | "super-init" | "iface-init") => format!("I:{c}"),
            super::From::Class(c) => format!("C:{c}"),
        }
    }
}

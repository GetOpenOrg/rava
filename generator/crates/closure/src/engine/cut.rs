//! 反事实诊断（仅诊断，缺省关闭；由 `closure::Input::diag` 给出，CLI `--cut` / `--cut-file` / `--dump-edges`）：
//! - 切除：条目 `类.方法:描述符` 表示该方法体不处理（节点保留、体内事件全部不执行，读者站点重跑同样挡住）；
//!   `类.方法:描述符@偏移` 表示该方法在该偏移处的调用 / 字段 / new 事件不执行。
//!   用于量化「切掉某条路径后闭包实际减少多少」。只宜切「消费型」节点（方法体、派发点）：切构造器 / 写入点会让
//!   字段值集变空、按初值折叠为恒 null，结果非单调（见 docs/plans/2026-10-01-c1d-closure-bloat.md §7）。
//! - 触发边转储：方法 / 类 / 分配 / 枢纽节点被登记的每一条触发边（不止首次溯源），派发边带接收者分配条件。
//! - 记录型类型流查询：`--flows` 里的 `@grow:` / `@trace:` / `@edge:` 须在分析前登记，传播中逐条记录（见 `diag.rs`）。

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

/// 诊断选项
#[derive(Debug, Default, Clone)]
pub struct Diag {
    /// 切除条目（每项一个 `类.方法:描述符[@偏移]`）
    pub cuts: Vec<String>,
    /// 触发边转储文件（每行 `源\t目标\t条件`）
    pub dump_edges: Option<PathBuf>,
    /// `--flows` 查询全文；其中记录型（`@grow:` / `@trace:` / `@edge:`）在分析前登记，其余在分析后求值
    pub flows: Vec<String>,
    /// `--site-prof`：读者站点重跑剖析（`site_prof.rs`，结果进 `summary.perf.site_prof`）
    pub site_prof: bool,
}

#[derive(Default)]
pub struct Cuts {
    methods: HashSet<String>,
    sites: HashMap<String, HashSet<u32>>,
}

impl Cuts {
    pub fn parse<'s>(entries: impl IntoIterator<Item = &'s String>) -> Cuts {
        let mut c = Cuts::default();
        for e in entries.into_iter().map(|e| e.trim()).filter(|e| !e.is_empty()) {
            match e.rsplit_once('@').and_then(|(m, off)| Some((m, off.parse::<u32>().ok()?))) {
                Some((m, off)) => {
                    c.sites.entry(m.to_string()).or_default().insert(off);
                }
                None => {
                    c.methods.insert(e.to_string());
                }
            }
        }
        c
    }
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

// ── 触发边转储：方法 / 类节点被登记的每一条触发边（不止首次溯源），供离线归因。缺省关闭 ──

use std::cell::RefCell;
use std::io::Write;

thread_local! {
    /// 当前派发的边属性：(源节点覆盖, 条件节点)——派发边只在接收者类型已实例化时成立
    static CTX: RefCell<(Option<String>, Option<String>)> = const { RefCell::new((None, None)) };
    static EDGES: RefCell<Option<(HashMap<String, u32>, HashSet<(u32, u32, u32)>)>> = const { RefCell::new(None) };
}

/// 开始记录触发边（分析开始前调用）
pub fn edges_begin() {
    EDGES.with(|e| *e.borrow_mut() = Some((HashMap::new(), HashSet::new())));
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

/// 运行结束时写出并停止记录：每行 `源\t目标\t条件`
pub fn edges_finish(path: &std::path::Path) -> std::io::Result<()> {
    EDGES.with(|e| {
        let Some((ids, set)) = e.borrow_mut().take() else { return Ok(()) };
        let mut names = vec![""; ids.len()];
        for (k, v) in &ids {
            names[*v as usize] = k.as_str();
        }
        let mut f = std::io::BufWriter::new(std::fs::File::create(path)?);
        for (a, b, c) in &set {
            let c = if *c == u32::MAX { "" } else { names[*c as usize] };
            writeln!(f, "{}\t{}\t{}", names[*a as usize], names[*b as usize], c)?;
        }
        f.flush()
    })
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

#[cfg(test)]
mod tests {
    use super::Cuts;

    #[test]
    fn cut_entries_keep_descriptor_semicolons() {
        let e = ["java/lang/String.format:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;".to_string(),
            "a/B.m:(Ljava/lang/Object;)V@11".to_string(), " ".to_string()];
        let c = Cuts::parse(&e);
        assert!(c.active());
        assert!(c.method("java/lang/String.format:(Ljava/lang/String;[Ljava/lang/Object;)Ljava/lang/String;"));
        assert!(c.site("a/B.m:(Ljava/lang/Object;)V", 11));
        assert!(!c.site("a/B.m:(Ljava/lang/Object;)V", 12));
        assert!(!Cuts::parse(&[]).active());
    }
}

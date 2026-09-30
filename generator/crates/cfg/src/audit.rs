//! 跳转消费自检（← `cfg/audit.py` 与 `method/codegen.py::_verify_tree`）。
//!
//! 每个方法的每条 branch / goto / switch 指令必须被某个结构消费；未被消费 →
//! [`CfgAuditError`]（生成期错误，不得被 stub 降级吞掉）。
//!
//! 与 Python 的差异：统计对象 [`AuditStats`] 不是全局单例，由调用方显式持有并传递。

use std::collections::{BTreeMap, BTreeSet};

use crate::error::CfgAuditError;
use crate::flow::FlowAnalysis;
use crate::graph::NodeId;
use crate::node::{StructNode, Terminal};
use crate::tree::{walk, Item};

/// 跳转的消费种类。
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum JumpKind {
    /// 结构树中的 if / match / 内联 / break / continue
    Structured,
    /// 并入前驱的 && / || 条件
    ShortCircuit,
    /// 折叠为条件表达式（含 boolean 物化）
    Ternary,
    /// 条件为编译期常量，折叠为无条件边
    ConstFold,
    /// 所在块在常量折叠后不可达（或字节码本身不可达）
    Dead,
    /// 不可归约 CFG 的状态机兜底
    Dispatch,
    /// 异常处理器入口
    Handler,
}

impl JumpKind {
    pub fn as_str(self) -> &'static str {
        match self {
            JumpKind::Structured => "structured",
            JumpKind::ShortCircuit => "short-circuit",
            JumpKind::Ternary => "ternary",
            JumpKind::ConstFold => "const-fold",
            JumpKind::Dead => "dead",
            JumpKind::Dispatch => "dispatch",
            JumpKind::Handler => "handler",
        }
    }
}

/// 单个方法的跳转账本。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JumpLedger {
    pub method_id: String,
    /// 所有跳转指令的 pc
    pub expected: BTreeSet<u32>,
    /// pc → 消费种类（后记覆盖先记）
    pub consumed: BTreeMap<u32, JumpKind>,
}

impl JumpLedger {
    pub fn new(method_id: impl Into<String>) -> JumpLedger {
        JumpLedger { method_id: method_id.into(), ..JumpLedger::default() }
    }

    pub fn expect(&mut self, pc: u32) {
        self.expected.insert(pc);
    }

    pub fn consume(&mut self, pc: u32, kind: JumpKind) {
        self.consumed.insert(pc, kind);
    }

    pub fn verify(&self) -> Result<(), CfgAuditError> {
        let missing: Vec<u32> =
            self.expected.iter().copied().filter(|pc| !self.consumed.contains_key(pc)).collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(CfgAuditError(format!("{}: 未被消费的跳转指令 pc={missing:?}", self.method_id)))
        }
    }
}

/// 结构树自检：每个活块恰好出现一次，每个 cond / switch / try 终结都有对应的
/// if / while / match / try。通过后，块终结承载的跳转指令（`jump_pcs`）记为 structured。
pub fn verify_tree<N: StructNode>(
    tree: &[Item],
    nodes: &BTreeMap<NodeId, N>,
    flow: &FlowAnalysis,
    ledger: &mut JumpLedger,
) -> Result<(), CfgAuditError> {
    let mut all = Vec::new();
    walk(tree, &mut all);
    let mut code_blocks = Vec::new();
    let mut origins = BTreeSet::new();
    for item in all {
        match item {
            Item::Code { block, .. } => code_blocks.push(*block),
            Item::If(i) => {
                origins.insert(i.origin);
            }
            Item::Switch(s) => {
                origins.insert(s.origin);
            }
            Item::Try(t) => {
                origins.insert(t.origin);
            }
            Item::Loop(l) => origins.extend(l.cond_origin),
            _ => {}
        }
    }
    code_blocks.sort_unstable();
    let mut live = flow.rpo.clone();
    live.sort_unstable();
    if code_blocks != live {
        return Err(CfgAuditError(format!(
            "{}: 结构树与活块集合不一致 tree={code_blocks:?} live={live:?}",
            ledger.method_id
        )));
    }
    for nid in &flow.rpo {
        let node = nodes
            .get(nid)
            .ok_or_else(|| CfgAuditError(format!("{}: 节点 {nid} 不存在", ledger.method_id)))?;
        let branching = matches!(
            node.terminal(),
            Terminal::Cond { .. } | Terminal::Switch { .. } | Terminal::Try { .. }
        );
        if branching && !origins.contains(nid) {
            return Err(CfgAuditError(format!(
                "{}: 块 pc={} 的分支未出现在结构树中 (jump pc={:?})",
                ledger.method_id,
                node.start_pc(),
                node.jump_pcs()
            )));
        }
        for pc in node.jump_pcs() {
            ledger.consumed.entry(*pc).or_insert(JumpKind::Structured);
        }
    }
    Ok(())
}

/// stub 兜底记录：（方法, 吞点, 原因）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StubFallback {
    pub method_id: String,
    pub site: String,
    pub reason: String,
}

/// 全量转译的跳转消费统计（调用方持有，不是全局状态）。
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AuditStats {
    pub methods: usize,
    pub jumps: usize,
    pub consumed: usize,
    pub by_kind: BTreeMap<&'static str, usize>,
    pub dispatch_methods: usize,
    pub stub_fallbacks: Vec<StubFallback>,
    /// 异常类型名 → 次数（随 stub_fallbacks 去重）
    pub stub_exc: BTreeMap<String, usize>,
    /// 结构化为 java_try! 的 try 区域数
    pub try_regions: usize,
    /// 方法 → 未进入结构化树的异常处理器个数（终态 0）
    pub handler_methods: BTreeMap<String, usize>,
    /// instanceof 静态折叠为编译期 false 的次数
    pub instanceof_folds: usize,
    seen: BTreeSet<String>,
    stub_seen: BTreeSet<(String, String)>,
}

impl AuditStats {
    /// 该方法是否已记账
    pub fn is_seen(&self, method_id: &str) -> bool {
        self.seen.contains(method_id)
    }

    pub fn record(&mut self, ledger: &JumpLedger, used_dispatch: bool) {
        if !self.seen.insert(ledger.method_id.clone()) {
            return;
        }
        self.methods += 1;
        self.jumps += ledger.expected.len();
        for pc in &ledger.expected {
            if let Some(kind) = ledger.consumed.get(pc) {
                self.consumed += 1;
                *self.by_kind.entry(kind.as_str()).or_default() += 1;
            }
        }
        if used_dispatch {
            self.dispatch_methods += 1;
        }
    }

    /// stub 兜底记账，去重键（方法, 吞点）；`exc` 非空时进 summary 的 exc. 分解
    pub fn record_stub_fallback(&mut self, method_id: &str, reason: &str, site: &str, exc: &str) {
        if !self.stub_seen.insert((method_id.to_string(), site.to_string())) {
            return;
        }
        self.stub_fallbacks.push(StubFallback {
            method_id: method_id.to_string(),
            site: site.to_string(),
            reason: reason.to_string(),
        });
        if !exc.is_empty() {
            *self.stub_exc.entry(exc.to_string()).or_default() += 1;
        }
    }

    pub fn record_try_regions(&mut self, method_id: &str, count: usize, untranslated_handlers: usize) {
        self.try_regions += count;
        if untranslated_handlers > 0 {
            self.handler_methods.insert(method_id.to_string(), untranslated_handlers);
        }
    }

    pub fn record_instanceof_fold(&mut self) {
        self.instanceof_folds += 1;
    }

    pub fn summary(&self) -> String {
        let kinds: Vec<String> = self.by_kind.iter().map(|(k, v)| format!("{k}={v}")).collect();
        let mut stub = format!("stub_fallback={}", self.stub_fallbacks.len());
        if !self.stub_fallbacks.is_empty() {
            let mut sites: BTreeMap<&str, usize> = BTreeMap::new();
            for s in &self.stub_fallbacks {
                *sites.entry(s.site.as_str()).or_default() += 1;
            }
            let mut detail =
                sites.iter().map(|(s, n)| format!("{s}={n}")).collect::<Vec<_>>().join(",");
            if !self.stub_exc.is_empty() {
                let exc: Vec<String> = self.stub_exc.iter().map(|(k, v)| format!("exc.{k}={v}")).collect();
                detail = format!("{detail}|{}", exc.join(","));
            }
            stub = format!("{stub}({detail})");
        }
        format!(
            "[cfg-audit] methods={} jumps={} consumed={} unconsumed={} dispatch={} try_regions={} \
             handler_methods={} handler_jumps={} {stub} instanceof_fold={} | {}",
            self.methods,
            self.jumps,
            self.consumed,
            self.jumps - self.consumed,
            self.dispatch_methods,
            self.try_regions,
            self.handler_methods.len(),
            self.by_kind.get("handler").copied().unwrap_or(0),
            self.instanceof_folds,
            kinds.join(" ")
        )
    }
}

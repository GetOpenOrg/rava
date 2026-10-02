//! 方法体生成总流程（← `codegen.gen_method_body` / `_structured_entries`）：签名 → 栈模拟器
//! 初始化 → 逐块模拟与结构化（或状态机兜底）→ 变量分析与提升 → 渲染 → 行级后处理 → 包装。

use std::collections::{BTreeMap, BTreeSet};

use cfg::{analyze, build_dispatch, AuditStats, function_always_returns, simplify, structure, verify_tree, JumpLedger, NodeId, Succs};
use classfile::extras::LocalVar;
use classfile::{acc, Method};
use input::NormCode;
use instr::{Audit, InstrEnv, InstrLog};
use sim::{SimConfig, SlotDecl, StackSim};
use ty::ident::safe_ident;
use ty::{ClassInfo, RsType};

use crate::blocks::{cfg_view, Blocks};
use crate::emit::{emit_dispatch, emit_tree};
use crate::entry::{Entry, Item};
use crate::error::{cfg_err, MethodError, MethodResult};
use crate::fold_array::fold_array_literals;
use crate::node::{Graph, Node};
use crate::sig::{ctor_functions, is_main, local_names, params_with_slot, rust_fn_name, signature_line};
use crate::split_try::split_disjoint_try_ranges;
use crate::unify::CondValues;
use crate::vars::{analyze_mutation, hoist_if_vars, hoist_loop_vars, promote_undeclared_assigns, VarsCtx};
use crate::{postprocess as pp, text};

/// ACC_SYNCHRONIZED（方法标志；与类标志 ACC_SUPER 同值，classfile 未单列）
const ACC_SYNCHRONIZED: u16 = 0x0020;
/// 变量提升轮数上限（每轮只提升一层）
const HOIST_ROUNDS: usize = 64;

/// 一次方法体生成的输入：发射所在类 + 方法视图 + 规范化字节码
#[derive(Clone, Copy)]
pub struct MethodRequest<'a> {
    /// 发射所在类（继承展开时为子类）
    pub class: &'a ClassInfo,
    /// 方法视图：名字 / 描述符 / 访问标志 / 泛型签名（继承展开时为适配后的视图）
    pub method: &'a Method,
    /// 规范化字节码（出处类的；无字节码 → None）
    pub code: Option<&'a NormCode>,
    /// 局部变量表（LVT + LVTT 签名；继承展开时为适配后的视图）
    pub local_vars: &'a [LocalVar],
    /// 出处方法的 LineNumberTable（(起始 pc, 行)，按 pc 升序；无则为空，不出行标记）
    pub line_numbers: &'a [(u16, u16)],
    /// 同名方法在发射类内有重载（名字加描述符后缀，前置 `// java:` 注释）
    pub overloaded: bool,
    /// 调用方给定的已去重 Rust 名
    pub rust_name: Option<&'a str>,
    /// 方法体发射在 vtable trait 的默认实现内
    pub in_vtable_body: bool,
}

/// 生成过程的外部登记（调用方持有：生成失败时已发生的登记同样保留，与 Python 全局登记一致）
#[derive(Debug, Clone, Default)]
pub struct MethodSink {
    /// 审计 / 继承调用请求 / lambda 引用 / SAM 站点
    pub log: InstrLog,
    /// 控制流审计数据（无字节码或控制流校验未通过 → None）
    pub cfg: Option<CfgStats>,
}

/// 单个方法的控制流审计数据（← `_structured_entries` 对全局 `STATS` 的记账，改由调用方汇总）
#[derive(Debug, Clone, Default)]
pub struct CfgStats {
    /// 已校验的跳转账本
    pub ledger: JumpLedger,
    /// 走了状态机兜底
    pub dispatch: bool,
    /// 存活的 try 节点数
    pub try_regions: usize,
    /// 异常表声明、却未挂到任何 try 节点的处理器数
    pub untranslated_handlers: usize,
}

impl CfgStats {
    /// 记入汇总（同一方法只记首次）
    pub fn record(&self, stats: &mut AuditStats) {
        if !stats.is_seen(&self.ledger.method_id) {
            stats.record_try_regions(&self.ledger.method_id, self.try_regions, self.untranslated_handlers);
        }
        stats.record(&self.ledger, self.dispatch);
    }
}

/// try 区域统计：(存活 try 节点数, 未翻译的处理器数)
fn try_region_stats(code: &NormCode, nodes: &Graph) -> (usize, usize) {
    let live: Vec<&Node> = nodes.iter().filter(|n| !n.removed).collect();
    let declared: BTreeSet<u32> = code.exception_table.iter().map(|e| e.handler).collect();
    let translated: BTreeSet<u32> = live
        .iter()
        .filter(|n| n.is_try())
        .flat_map(|n| n.handlers.iter().filter_map(|h| nodes.get(*h).map(|x| x.start_pc)))
        .collect();
    (live.iter().filter(|n| n.is_try()).count(), declared.difference(&translated).count())
}

/// 局部变量声明表：泛型签名（非根类）优先，否则描述符（非根类、非路径形态）；按起点升序
fn slot_decls(env: &InstrEnv, local_vars: &[LocalVar]) -> BTreeMap<u16, Vec<SlotDecl>> {
    let ty_ctx = &env.ctx.ty;
    let usable = |t: &RsType| {
        let s = text::ty(env, t);
        (s != ir::anchors::OBJECT).then_some(s)
    };
    let mut out: BTreeMap<u16, Vec<SlotDecl>> = BTreeMap::new();
    for lv in local_vars {
        let mut ty = None;
        let mut from_sig = false;
        if !lv.signature.is_empty() {
            if let Some(t) = ty_ctx.parse_field_type(&lv.signature, &env.tparams).filter(|t| usable(t).is_some()) {
                ty = Some(t);
                from_sig = true;
            }
        }
        if ty.is_none() && !lv.desc.is_empty() {
            let t = ty_ctx.jvm_to_rust(&lv.desc);
            if usable(&t).is_some_and(|s| !s.contains("::")) {
                ty = Some(t);
            }
        }
        out.entry(lv.slot).or_default().push(SlotDecl {
            start: u32::from(lv.start),
            end: u32::from(lv.start) + u32::from(lv.len),
            name: lv.name.clone(),
            ty,
            from_sig,
            raw_sig: lv.signature.clone(),
            desc: lv.desc.clone(),
        });
    }
    for v in out.values_mut() {
        v.sort_by_key(|d| d.start);
    }
    out
}

/// 方法指令 → entries；全部跳转指令的消费情况记入账本并在此校验（无指令 → 无审计数据）
fn structured_entries<'e>(
    env: &'e InstrEnv<'e>,
    sim: &mut StackSim<'e>,
    sink: &mut MethodSink,
    mut ledger: JumpLedger,
    code: &NormCode,
    local_vars: &[LocalVar],
) -> MethodResult<Vec<Entry>> {
    if code.insns.is_empty() {
        return Ok(Vec::new());
    }
    let ledger = &mut ledger;
    let blocks = Blocks::new(env, sim, &mut sink.log, ledger, &code.insns, &code.exception_table, local_vars, CondValues::default())?;
    let out = blocks.run()?;
    let mut nodes = out.nodes;
    let entries = if out.dispatch {
        let ids: Vec<NodeId> = nodes.order.clone();
        let d = build_dispatch(out.entry, &ids);
        let mut v: Vec<Entry> = out.top_decls.iter().map(|l| Entry::line(format!("    {l}"))).collect();
        v.extend(emit_dispatch(&d, &nodes));
        v
    } else {
        split_disjoint_try_ranges(&out.plan, &mut nodes)?;
        for n in nodes.nodes.values_mut() {
            n.sync_term();
        }
        let succs: Succs = nodes.nodes.iter().map(|(&id, n)| (id, n.successors())).collect();
        let flow = analyze(out.entry, &succs);
        if !flow.reducible {
            return cfg_err("归约后的 CFG 不可归约");
        }
        let tree = simplify(structure(&mut nodes.nodes, &flow)?);
        verify_tree(&tree, &nodes.nodes, &flow, ledger)?;
        emit_tree(env, &tree, &nodes)
    };
    ledger.verify()?;
    let (try_regions, untranslated_handlers) = try_region_stats(code, &nodes);
    sink.cfg = Some(CfgStats { ledger: std::mem::take(ledger), dispatch: out.dispatch, try_regions, untranslated_handlers });
    Ok(entries)
}

/// entries → 行（语句条目按条目缩进渲染；来源行变化处先出独立行标记，见 [`crate::lines`]）
fn render(env: &InstrEnv, entries: &[Entry], line_numbers: &[(u16, u16)]) -> Vec<String> {
    let mut lines = Vec::with_capacity(entries.len());
    let mut marker = crate::lines::Marker::new(line_numbers);
    for e in entries {
        if matches!(e.item, Item::Removed) {
            continue;
        }
        lines.extend(marker.before(e.pc));
        match &e.item {
            Item::Stmt(s) => lines.push(format!("{}{}", e.indent, text::stmt(env, s).trim_start())),
            Item::Line(t) | Item::Struct { text: t, .. } => lines.push(t.clone()),
            Item::Removed => {}
        }
    }
    lines
}

/// 模拟器配置与初始状态（形参 / this 局部、类型变量上界）
fn new_sim<'e>(
    env: &'e InstrEnv<'e>,
    req: &MethodRequest,
    params: Vec<RsType>,
    ret: RsType,
) -> MethodResult<StackSim<'e>> {
    let m = req.method;
    let cfg = SimConfig {
        params,
        is_static: m.access & acc::STATIC != 0,
        class_name: req.class.name().to_string(),
        class_type_params: env.tparams.clone(),
        local_names: local_names(req.local_vars),
        slot_decls: slot_decls(env, req.local_vars),
        return_type: ret,
        is_constructor: m.name == "<init>",
        in_vtable_body: req.in_vtable_body,
    };
    let mut sim = StackSim::new(cfg, env)?;
    if !env.tparams.is_empty() {
        if let Some(ci) = env.ctx.reg().get(req.class.name()) {
            sim.state.type_var_bounds =
                env.ctx.ty.class_type_param_bounds(ci).into_iter().map(|(tv, (b, _))| (tv, b)).collect();
        }
    }
    Ok(sim)
}

/// 函数体前导行：实例方法绑定 `this`；同步方法获取监视器（RAII 守卫）
fn prologue(req: &MethodRequest, log: &mut InstrLog) -> Vec<Entry> {
    let m = req.method;
    let is_ctor = m.name == "<init>";
    let is_static = m.access & acc::STATIC != 0;
    let mut out = Vec::new();
    if !is_ctor && !is_static {
        out.push(Entry::line("    let this = self;".to_string()));
    }
    if !is_ctor && m.name != "<clinit>" && m.access & ACC_SYNCHRONIZED != 0 {
        log.audit(Audit::MonitorMt);
        let target = if is_static {
            format!("&class_monitor(\"{}\")", req.class.name())
        } else {
            "&Object::from(Clone::clone(this))".to_string()
        };
        out.push(Entry::line(format!("    let __sync_guard = MonitorGuard::acquire({target})?;")));
    }
    out
}

/// 变量分析与提升（渲染前，IR 层）
fn vars_passes(
    env: &InstrEnv,
    entries: &mut Vec<Entry>,
    predeclared: &BTreeSet<String>,
    decls: &BTreeMap<u16, Vec<SlotDecl>>,
) -> MethodResult<()> {
    analyze_mutation(entries);
    let cx = VarsCtx::new(env, predeclared, decls);
    hoist_loop_vars(&cx, entries);
    for _ in 0..HOIST_ROUNDS {
        if !hoist_if_vars(&cx, entries)? {
            break;
        }
    }
    promote_undeclared_assigns(entries, predeclared);
    Ok(())
}

/// 类 + 方法 → 完整 Rust 函数文本（见 crate 文档）
pub fn gen_method_body<'e>(env: &'e InstrEnv<'e>, req: &MethodRequest, sink: &mut MethodSink) -> MethodResult<String> {
    let m = req.method;
    if env.ctx.code_owner.is_empty() {
        // 字节码出处类决定常量池 / bootstrap 表的归属，调用方必须给出（无回退推断）
        return Err(MethodError::Runtime(format!("{}.{}:{} 的字节码出处类为空", req.class.name(), m.name, m.desc)));
    }
    let is_ctor = m.name == "<init>";
    let is_static = m.access & acc::STATIC != 0;
    let class_name = req.class.name();
    let sig = env.ctx.ty.emitted_method_sig_types(req.class, m, &env.tparams);
    let ret = text::ty(env, &sig.ret);
    let fn_name = rust_fn_name(env, m, req.overloaded, req.rust_name);
    let names = local_names(req.local_vars);
    let params = params_with_slot(env, m, if is_static { 0 } else { 1 }, &sig.params, &names);
    let sig_line = signature_line(m, &fn_name, &params, &ret);

    let mut sim = new_sim(env, req, sig.params.clone(), sig.ret.clone())?;
    let predeclared: BTreeSet<String> = sim.state.locals.values().map(|l| l.name.as_str().to_string()).collect();
    let mut entries = prologue(req, &mut sink.log);
    let ledger = JumpLedger::new(format!("{class_name}.{}:{}", m.name, m.desc));
    if let Some(code) = req.code {
        entries.extend(structured_entries(env, &mut sim, sink, ledger, code, req.local_vars)?);
    }
    if sim.state.underflow {
        return cfg_err(format!("stack underflow in {class_name}.{}", m.name));
    }
    vars_passes(env, &mut entries, &predeclared, &sim.cfg.slot_decls)?;

    let mut lines = render(env, &entries, req.line_numbers);
    pp::erase_boxed_ctor_type_args(&mut lines);
    let own_short = env.ctx.short(class_name);
    let static_getters: BTreeSet<String> = req
        .class
        .fields()
        .iter()
        .filter(|f| f.access & acc::STATIC != 0)
        .map(|f| format!("{own_short}::{}", safe_ident(&f.name)))
        .collect();
    let mut lines = fold_array_literals(lines, &static_getters);

    if is_ctor {
        while lines.last().is_some_and(|l| {
            crate::lines::is_mark(l) || matches!(text::py_strip(l), "return;" | "return Ok(());" | "return Ok(this);")
        }) {
            lines.pop();
        }
        lines.push("    Ok(this)".to_string());
        pp::normalize_this_clone(&mut lines, true);
    } else {
        if !is_static {
            pp::normalize_this_clone(&mut lines, false);
        }
        if ret == "bool" {
            pp::fix_bool_returns(&mut lines);
        }
        pp::remove_trailing_return_ok(&mut lines);
        pp::add_ok_return(&mut lines, &ret, function_always_returns(&req.code.map(|c| cfg_view(&c.insns)).unwrap_or_default()));
    }

    let args_used = |args: &str| lines.iter().any(|l| !crate::lines::is_mark(l) && text::has_word(l, args));
    let main_args = (!is_ctor && is_main(m))
        .then(|| req.local_vars.iter().find(|lv| lv.slot == 0).map_or("args", |lv| lv.name.as_str()))
        .filter(|a| args_used(a));
    let mut body = crate::lines::attach(lines).join("\n");
    let prefix = if req.overloaded { format!("// java: {}{}\n", m.name, m.desc) } else { String::new() };
    let text = if is_ctor {
        ctor_functions(&prefix, &fn_name, &params, &body)
    } else {
        if let Some(args) = main_args {
            body = format!("    let mut {args}: JArray<{}> = java_runtime::main_args();\n{body}", ir::anchors::STRING);
        }
        format!("{prefix}{sig_line} {{\n{body}\n}}")
    };
    Ok(text)
}

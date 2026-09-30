//! 方法体生成器适配（P5b）：以 P4c `method` crate 实现 [`MethodBodyEmitter`]。
//!
//! 发射层给出 [`BodyRequest`]（发射类 + 方法视图 + 字节码出处类 + 类型变量代换），本模块补齐
//! `method::gen_method_body` 的其余输入：
//! - 规范化字节码与局部变量表取自出处类（`EmitInput::code` / 类补充属性）；接口方法展开时
//!   局部变量表签名按同一代换改写（← `class_writer._adapt_interface_method`）；
//! - 指令层回调 [`InstrHooks`]：SAM 合成对象构造路径经 [`crate::sam::SamLedger::site_ctor_path`]；
//! - 生成期登记（[`MethodSink`]）→ [`BodyEffects`]：继承成员需求 / lambda 引用 / SAM 站点并入
//!   发射层账本；等价性审计与控制流审计留在本适配器（[`BodyAudit`]），供审计行输出。
//!
//! 错误映射（← Python `fallback_audit.fallback_exc`）：`MethodError::Cfg` 为控制流语义限制，
//! 非 strict 退化为存根（[`BodyError::Fallback`]），strict 与其余错误一律 [`BodyError::Fatal`]。

use std::collections::BTreeMap;

use cfg::AuditStats;
use classfile::extras::LocalVar;
use instr::{Audit, Effect, InstrCtx, InstrEnv, InstrFacts, InstrHooks};
use method::{gen_method_body, MethodError, MethodRequest, MethodSink};
use ty::sig_parse::substitute_signature_type_vars;

use crate::body::{BodyEffects, BodyError, BodyOutput, BodyRequest, MethodBodyEmitter};
use crate::ctx::EmitCtx;

/// 方法体生成的审计汇总（等价性审计计数 + 控制流审计）
#[derive(Debug, Default)]
pub struct BodyAudit {
    /// 等价性审计：审计项 → 次数
    pub equiv: BTreeMap<Audit, usize>,
    /// 控制流审计（跳转消费、状态机、try 区域、instanceof 折叠、存根兜底）
    pub cfg: AuditStats,
}

/// 存根兜底的位点名：发射层不区分 Python 的九吞点（iface-lambda / main / clinit …），统一记为方法体
const FALLBACK_SITE: &str = "body";

/// `method` crate 驱动的方法体生成器
pub struct MethodBodies {
    facts: InstrFacts,
    strict: bool,
    pub audit: BodyAudit,
}

impl MethodBodies {
    pub fn new(ctx: &EmitCtx<'_>) -> MethodBodies {
        let root = ctx.cp.get(ty::consts::OBJECT);
        let facts = InstrFacts::build(ctx.ty.reg, root.as_deref(), &ctx.runtime_src());
        MethodBodies { facts, strict: ctx.opts.strict, audit: BodyAudit::default() }
    }

    fn absorb_audit(&mut self, sink: &MethodSink) {
        for e in &sink.log.effects {
            match e {
                Effect::Audit(a) => *self.audit.equiv.entry(*a).or_default() += 1,
                Effect::InstanceofFold => self.audit.cfg.instanceof_folds += 1,
                _ => {}
            }
        }
        if let Some(c) = &sink.cfg {
            c.record(&mut self.audit.cfg);
        }
    }
}

/// 指令层回调：SAM 合成对象构造路径
struct Hooks<'c, 'a> {
    ctx: &'c EmitCtx<'a>,
}

impl InstrHooks for Hooks<'_, '_> {
    fn sam_ctor_path(&self, iface: &str, current_class: &str) -> Option<ir::Path> {
        let text = self.ctx.sam().site_ctor_path(self.ctx, iface, current_class)?;
        let segs = text.split("::").map(|s| ir::Ident::new(s).ok().map(ir::PathSegment::new)).collect::<Option<Vec<_>>>()?;
        Some(ir::Path::new(segs))
    }
}

/// 登记事实 → 发射层账本条目
fn effects_of(sink: &MethodSink) -> BodyEffects {
    let mut fx = BodyEffects::default();
    for e in &sink.log.effects {
        match e {
            Effect::Inherited { receiver, method, param_desc } => {
                fx.requests.push((receiver.clone(), method.clone(), param_desc.clone()));
            }
            Effect::LambdaRef { class, method, rust_name } => fx.lambda_refs.push((class.clone(), method.clone(), rust_name.clone())),
            Effect::SamSite { iface, sam_desc, class } => fx.sam_sites.push((iface.clone(), sam_desc.clone(), class.clone())),
            Effect::Audit(_) | Effect::InstanceofFold => {}
        }
    }
    fx
}

/// 出处类中该方法的局部变量表；接口方法展开时签名里的接口类型变量按视图代换
fn local_vars(ctx: &EmitCtx<'_>, req: &BodyRequest<'_>, index: Option<usize>) -> Vec<LocalVar> {
    let ex = ctx.extras(req.declaring_class);
    let Some(lvs) = index.and_then(|i| ex.methods.get(i)).map(|m| &m.local_vars) else { return Vec::new() };
    match req.type_var_view.filter(|v| !v.is_empty()) {
        Some(view) => lvs
            .iter()
            .map(|lv| {
                let signature = substitute_signature_type_vars(&lv.signature, view).unwrap_or_else(|| lv.signature.clone());
                LocalVar { signature, ..lv.clone() }
            })
            .collect(),
        None => lvs.clone(),
    }
}

impl MethodBodyEmitter for MethodBodies {
    fn emit_body(&mut self, ctx: &EmitCtx<'_>, req: &BodyRequest<'_>) -> Result<BodyOutput, BodyError> {
        let (name, desc) = (&req.method.name, &req.method.desc);
        let key = format!("{}.{name}:{desc}", req.class.name());
        let owner = ctx
            .class(req.declaring_class)
            .ok_or_else(|| BodyError::Fatal(format!("{key}：出处类 {} 不在注册表", req.declaring_class)))?;
        let index = owner.methods().iter().position(|m| &m.name == name && &m.desc == desc);
        let code = index.and_then(|i| ctx.input.code(req.declaring_class, &owner.methods()[i]));
        let lvs = local_vars(ctx, req, index);
        let hooks = Hooks { ctx };
        let ictx = InstrCtx::new(ctx.ty, ctx.manifest, &self.facts, &hooks, req.class.name()).with_code_owner(req.declaring_class);
        let env = InstrEnv::new(ictx, req.class_type_params);
        let mreq = MethodRequest {
            class: req.class,
            method: req.method,
            code: code.as_deref(),
            local_vars: &lvs,
            overloaded: req.overloaded_names.contains(name),
            rust_name: req.rust_name,
            in_vtable_body: req.in_vtable_body,
        };
        let mut sink = MethodSink::default();
        let res = gen_method_body(&env, &mreq, &mut sink);
        self.absorb_audit(&sink);
        match res {
            Ok(text) => Ok(BodyOutput { text, effects: effects_of(&sink) }),
            Err(MethodError::Cfg(msg)) if !self.strict => {
                let text = format!("CfgError: {msg}");
                self.audit.cfg.record_stub_fallback(&key, &text, FALLBACK_SITE, "CfgError");
                Err(BodyError::Fallback(text))
            }
            Err(e) => Err(BodyError::Fatal(format!("{key}：{e}"))),
        }
    }
}

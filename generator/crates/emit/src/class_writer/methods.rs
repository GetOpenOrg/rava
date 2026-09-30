//! 方法翻译主循环（← `class_writer._emit_method_blocks` 与 `clinit_extract._gen_clinit_block`）。
//!
//! 逐方法判定（Rust 名、角色、翻译 / 存根 / 手写 / 声明）取 [`input::Planner`]（与 Python 逐项
//! 一致的判定层）；本模块负责文本：`<clinit>` → `__clinit`；接口私有 lambda 体 / 私有实例方法落
//! java_class! 块外的擦除固有 impl 块；接口实例方法发声明或载体 default 体；类方法按槽位归属
//! 补 `virtual_in` / `vtable_name` / `vtable_erasure` 属性后翻译或出存根；接口伴生契约补发声明。

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

use classfile::Method;
use input::plan::CLINIT_FN;
use input::{MethodPlan, Role, Verdict};
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use super::attrs::{method_attr, MethodAttrExtra};
use super::hw_overrides::HwOverride;
use super::slot::override_vtable_erasure;
use super::stub::{native_stub, Stub};
use crate::body::{BodyError, BodyRequest, MethodBodyEmitter};
use crate::ctx::{EmitCtx, HwAudit, ProjectState};
use crate::error::{EmitError, Result};
use crate::lang;
use crate::text::split_top_level;

const ACC_BRIDGE: u16 = 0x0040;
/// 原语 Rust 类型（伴生核心适配的宽度还原判定）
pub(super) const PRIMITIVE_RUST_TYPES: [&str; 13] = ["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f32", "f64", "bool", "usize", "()"];
const SUPP_NOTE: &str = "// 伴生契约声明（E0407）：手写 _impl 文件实现 Iface__VTable 的成员，类模型不含此方法（JDK 版本演化），\n// 生成 trait 恒含伴生方法集——签名取自伴生 impl，逐字一致保证 trait 相干。";

/// 方法段产物
#[derive(Debug, Default)]
pub struct MethodBlocks {
    /// 宏块 impl 内的方法块（接续 static 字段块）
    pub method_blocks: Vec<String>,
    /// 接口载体擦除固有 impl 块（G-10 lambda 体 / 私有实例方法）
    pub iface_lambda_blocks: Vec<String>,
    /// 接口伴生契约补发声明（同时在 `method_blocks` 中；独立成表供导入兜底扫描）
    pub iface_supp_blocks: Vec<String>,
}

/// 待发射方法：本类声明 / 手写覆盖合成的祖先副本 / 祖先与接口方法的展开副本；
/// `owner` + `index` 定位补充属性（LVT、注解原始字节）与方法体字节码
pub(super) struct Emitted<'c> {
    pub method: Cow<'c, Method>,
    pub owner: &'c ClassInfo,
    pub index: usize,
}

impl<'c> Emitted<'c> {
    /// `owner` 声明的第 `index` 个方法原样
    pub fn declared(owner: &'c ClassInfo, index: usize) -> Emitted<'c> {
        Emitted { method: Cow::Borrowed(&owner.methods()[index]), owner, index }
    }
}

/// 方法体请求的可变部分
pub(super) struct BodySpec<'s> {
    pub ctparams: &'s [String],
    /// None：方法体生成器按自身规则命名
    pub rust_name: Option<&'s str>,
    pub in_vtable_body: bool,
    /// 接口方法展开时的类型变量代换
    pub view: Option<&'s BTreeMap<String, String>>,
    /// 发射位点名（[`BodyRequest::site`]）
    pub site: &'static str,
}

/// 单类方法段的共享参数
pub(super) struct Cx<'a, 'c> {
    pub ctx: &'a EmitCtx<'c>,
    pub ci: &'c ClassInfo,
    pub tps: &'a [String],
    pub overloaded: &'a BTreeSet<String>,
}

impl Cx<'_, '_> {
    pub fn attr(&self, e: &Emitted<'_>, extra: &MethodAttrExtra) -> String {
        let ex = self.ctx.extras(e.owner.name());
        method_attr(&e.method, ex.methods.get(e.index), extra)
    }

    pub fn stub(&self, e: &Emitted<'_>, rust_name: &str, ctparams: &[String]) -> Stub {
        let ex = self.ctx.extras(e.owner.name());
        let names = ex.methods.get(e.index).map(|x| x.local_names()).unwrap_or_default();
        native_stub(self.ctx, self.ci, &e.method, rust_name, ctparams, &names)
    }

    /// 翻译方法体（Rust 名显式、无类型变量代换的常用形态）
    fn body(
        &self,
        state: &mut ProjectState,
        bodies: &mut dyn MethodBodyEmitter,
        e: &Emitted<'_>,
        ctparams: &[String],
        rust_name: &str,
        in_vtable_body: bool,
        site: &'static str,
    ) -> Result<Option<String>> {
        let spec = BodySpec { ctparams, rust_name: Some(rust_name), in_vtable_body, view: None, site };
        self.body_with(state, bodies, e, &spec)
    }

    /// 翻译方法体；兜底类失败返回 None（调用方退化为存根），硬失败穿透
    pub fn body_with(
        &self,
        state: &mut ProjectState,
        bodies: &mut dyn MethodBodyEmitter,
        e: &Emitted<'_>,
        spec: &BodySpec<'_>,
    ) -> Result<Option<String>> {
        let req = BodyRequest {
            class: self.ci,
            method: &e.method,
            declaring_class: e.owner.name(),
            type_var_view: spec.view,
            class_type_params: spec.ctparams,
            overloaded_names: self.overloaded,
            rust_name: spec.rust_name,
            in_vtable_body: spec.in_vtable_body,
            site: spec.site,
        };
        match bodies.emit_body(self.ctx, &req) {
            Ok(out) => {
                state.absorb(&out.effects);
                Ok(Some(out.text))
            }
            Err(BodyError::Fallback(_)) => Ok(None),
            Err(BodyError::Fatal(s)) => Err(EmitError::Body(s)),
        }
    }

    /// FS-H0 审计：公开 API 类的非 native 方法被手写覆盖（`_audit_override`）
    fn audit_override(&self, state: &mut ProjectState, m: &Method) {
        let n = self.ci.name();
        if m.is_native() || m.is_abstract() || !lang::in_public_api(n) {
            return;
        }
        let member = format!("{n}.{}:{}", m.name, m.desc);
        let kind = if self.ctx.boundary.is_boundary_class(n) || self.ctx.boundary.is_vm_boundary_class(n) {
            HwAudit::VmBoundary
        } else if self.ctx.manifest.intrinsic_members.contains(&member) {
            HwAudit::Intrinsic
        } else {
            HwAudit::Override
        };
        state.hw_audit.push((kind, member));
    }

    fn define(&self, state: &mut ProjectState, m: &Method, rust: &str) {
        state
            .lambda_defs
            .entry((self.ci.name().to_string(), m.name.clone()))
            .or_default()
            .insert(rust.to_string());
    }
}

/// 待发射方法表：可见（非 synthetic）+ 手写覆盖副本 + 非桥接 synthetic（`<init>` / `<clinit>` 除外）
fn emitted_methods<'c>(ci: &'c ClassInfo, overrides: &[HwOverride<'c>]) -> Vec<Emitted<'c>> {
    let own = |(i, m): (usize, &'c Method)| Emitted { method: Cow::Borrowed(m), owner: ci, index: i };
    let mut out: Vec<Emitted<'c>> = ci.methods().iter().enumerate().filter(|(_, m)| !m.is_synthetic()).map(own).collect();
    out.extend(overrides.iter().map(|o| Emitted { method: Cow::Owned(o.method.clone()), owner: o.owner, index: o.index }));
    out.extend(
        ci.methods()
            .iter()
            .enumerate()
            .filter(|(_, m)| m.is_synthetic() && m.access & ACC_BRIDGE == 0 && m.name != "<init>" && m.name != "<clinit>")
            .map(own),
    );
    out
}

/// 定义侧的 invokedynamic 实现方法名（`lambda_impl_rust_name` 在「接口自身声明的 synthetic
/// lambda 体」这一调用面上的分支：根类同名重载按手写 API 名面，否则按接收者重载态 mangle）。
/// 方法体侧（P4c）接入后与其调用点命名统一到同一模块。
fn lambda_rust_name(ctx: &EmitCtx<'_>, ci: &ClassInfo, m: &Method) -> String {
    let mangled = mangle_name(&ctx.manifest.ty, &m.name, &m.desc);
    let name = if (mangled != m.name && ctx.root_api().contains(&mangled))
        || ctx.ty.hierarchy_overloaded_names(ci).contains(&m.name)
    {
        mangled
    } else {
        m.name.clone()
    };
    safe_ident(&name.replace("<init>", "new"))
}

/// 方法翻译主循环
pub fn emit_method_blocks(
    ctx: &EmitCtx<'_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    ci: &ClassInfo,
    tps: &[String],
    overrides: &[HwOverride<'_>],
) -> Result<MethodBlocks> {
    let plan = ctx
        .planner
        .plan(ci.name())
        .ok_or_else(|| EmitError::Input(format!("发射类不在注册表：{}", ci.name())))?;
    let overloaded = ctx.ty.hierarchy_overloaded_names(ci);
    let cx = Cx { ctx, ci, tps, overloaded: &overloaded };
    let mut out = MethodBlocks::default();
    let mut plans = plan.methods.iter().peekable();
    for e in emitted_methods(ci, overrides) {
        let Some(p) = plans.next_if(|p| p.name == e.method.name && p.desc == e.method.desc) else { continue };
        match p.role {
            Role::Clinit => out.method_blocks.push(clinit_block(&cx, state, bodies, &e, p)?),
            Role::IfaceLambda | Role::IfacePrivate => {
                let rust = if p.role == Role::IfaceLambda { lambda_rust_name(ctx, ci, &e.method) } else { p.rust_name.clone() };
                let erased: Vec<String> = vec!["Object".to_string(); tps.len()];
                let site = if p.role == Role::IfaceLambda { "iface-lambda" } else { "iface-private" };
                let body = match p.verdict {
                    Verdict::Bytecode => cx.body(state, bodies, &e, &erased, &rust, false, site)?,
                    _ => None,
                };
                out.iface_lambda_blocks.push(body.unwrap_or_else(|| cx.stub(&e, &rust, &erased).text));
                cx.define(state, &e.method, &safe_ident(&rust));
            }
            Role::Member => out.method_blocks.push(member_block(&cx, state, bodies, &e, p)?),
        }
    }
    if let Some(hw) = ctx.input.handwritten.get(ci.name()) {
        for name in &plan.iface_supplement {
            let Some((params, ret)) = hw.iface_method_sigs.get(name) else { continue };
            let recv = if params.is_empty() { "&self".to_string() } else { format!("&self, {params}") };
            let supp = format!("{SUPP_NOTE}\npub fn {name}({recv}) -> {ret};");
            out.method_blocks.push(supp.clone());
            out.iface_supp_blocks.push(supp);
        }
    }
    Ok(out)
}

/// `<clinit>` → `fn __clinit()`（类型存根 / VM 边界类 / 链外边界类不发，由判定层剔除）
fn clinit_block(
    cx: &Cx<'_, '_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    e: &Emitted<'_>,
    p: &MethodPlan,
) -> Result<String> {
    let attr = cx.attr(e, &MethodAttrExtra::default());
    let body = match p.verdict {
        Verdict::Bytecode => cx.body(state, bodies, e, cx.tps, CLINIT_FN, false, "clinit")?,
        _ => None,
    };
    let text = body.unwrap_or_else(|| {
        format!("pub fn {CLINIT_FN}() -> Result<()> {{\n    panic!(\"stub: {}.<clinit>:()V\")\n}}", cx.ci.name())
    });
    Ok(format!("{attr}\n{text}"))
}

/// 类方法的槽位属性：槽位归属、槽位成员名解耦、槽位擦除名单
pub(super) fn slot_extra(cx: &Cx<'_, '_>, m: &Method, rust_name: &str) -> MethodAttrExtra {
    let virtual_in = cx.ctx.resolve_virtual_slot(m, cx.ci);
    let mut extra = MethodAttrExtra { virtual_in, ..Default::default() };
    if !extra.virtual_in.is_empty() && extra.virtual_in != cx.ctx.short(cx.ci.name()) {
        let slot_name = cx.ctx.slot_member_rust_name(m, cx.ci);
        if !slot_name.is_empty() && slot_name != rust_name {
            extra.vtable_name = slot_name;
        }
        extra.vtable_erasure = override_vtable_erasure(cx.ctx, cx.ci, m, &extra.virtual_in);
    }
    extra
}

fn result_inner(t: &str) -> Option<&str> {
    t.strip_prefix("Result<").and_then(|r| r.strip_suffix('>')).map(str::trim)
}

/// 伴生核心适配：`{ let this = self; Ok(this.core(args)?[ as 模型宽度]) }`
fn core_adapter(sig: &str, core: &str, core_ret: &str) -> String {
    let (open, close) = (sig.find('(').map_or(0, |i| i + 1), sig.rfind(')').unwrap_or(sig.len()));
    let ps = sig.get(open..close).unwrap_or("");
    let args: Vec<String> = split_top_level(ps)
        .iter()
        .filter(|p| p.contains(':'))
        .map(|p| p.split(':').next().unwrap_or("").trim().to_string())
        .collect();
    let model_ret = sig.rsplit_once("->").map_or("", |(_, r)| r.trim());
    let cast = match (result_inner(model_ret), result_inner(core_ret)) {
        (Some(mr), Some(cr))
            if !mr.is_empty() && !cr.is_empty() && mr != cr && PRIMITIVE_RUST_TYPES.contains(&mr) && PRIMITIVE_RUST_TYPES.contains(&cr) =>
        {
            format!(" as {mr}")
        }
        _ => String::new(),
    };
    format!("{sig} {{ let this = self; Ok(this.{core}({})?{cast}) }}", args.join(", "))
}

/// 宏块内的普通方法块
fn member_block(
    cx: &Cx<'_, '_>,
    state: &mut ProjectState,
    bodies: &mut dyn MethodBodyEmitter,
    e: &Emitted<'_>,
    p: &MethodPlan,
) -> Result<String> {
    let m: &Method = &e.method;
    let rust = p.rust_name.as_str();
    let fn_check = safe_ident(rust);
    cx.define(state, m, &fn_check);
    let iface_inst = cx.ci.is_interface() && !m.is_static();
    let plain = MethodAttrExtra::default();
    match p.verdict {
        Verdict::Handwritten => {
            cx.audit_override(state, m);
            let sig = cx.stub(e, rust, cx.tps).sig;
            let attr = cx.attr(e, &plain);
            if iface_inst {
                return Ok(format!("{attr}\n{sig};"));
            }
            // 声明跳过（体在 _impl.rs）；元数据行与签名以注释承载（反射表 / 分派闭包协议）
            let mut meta: Vec<String> = attr
                .lines()
                .filter(|l| l.contains("java_method(") || l.contains("java_native("))
                .map(|l| format!("// [meta] {l}"))
                .collect();
            if m.name != "<init>" {
                meta.push(format!("// [meta] {};", sig.trim_end_matches(';')));
            }
            return Ok(meta.join("\n"));
        }
        Verdict::IfaceDefaultBody | Verdict::IfaceDecl => {
            let attr = cx.attr(e, &plain);
            if p.verdict == Verdict::IfaceDefaultBody {
                if let Some(text) = cx.body(state, bodies, e, cx.tps, rust, false, "iface-default")? {
                    return Ok(format!("{attr}\n{text}"));
                }
            }
            return Ok(format!("{attr}\n{};", cx.stub(e, rust, cx.tps).sig));
        }
        _ => {}
    }
    let mut extra = slot_extra(cx, m, rust);
    match p.verdict {
        Verdict::HandwrittenBody => {
            cx.audit_override(state, m);
            extra.handwritten_body = true;
            Ok(format!("{}\n{};", cx.attr(e, &extra), cx.stub(e, rust, cx.tps).sig))
        }
        Verdict::Core => {
            let hw = cx.ctx.input.handwritten.get(cx.ci.name());
            let Some((core, core_ret)) = hw.and_then(|h| h.method_cores.get(&fn_check)) else {
                return Err(EmitError::Input(format!("伴生核心缺失：{}.{fn_check}", cx.ci.name())));
            };
            let sig = cx.stub(e, rust, cx.tps).sig;
            Ok(format!("{}\n{}", cx.attr(e, &extra), core_adapter(&sig, core, core_ret)))
        }
        Verdict::Bytecode => {
            let body = cx.body(state, bodies, e, cx.tps, rust, !extra.virtual_in.is_empty(), "main")?;
            let text = body.unwrap_or_else(|| cx.stub(e, rust, cx.tps).text);
            Ok(format!("{}\n{text}", cx.attr(e, &extra)))
        }
        _ => Ok(format!("{}\n{}", cx.attr(e, &extra), cx.stub(e, rust, cx.tps).text)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn core_adapter_forwards_and_casts() {
        let sig = "pub fn arrayBaseOffset(&self, c: Class<Object>) -> Result<i32>";
        assert_eq!(
            core_adapter(sig, "core_arrayBaseOffset", "Result<i64>"),
            "pub fn arrayBaseOffset(&self, c: Class<Object>) -> Result<i32> { let this = self; Ok(this.core_arrayBaseOffset(c)? as i32) }"
        );
        let unit = "pub fn f(&self, a: i32, b: Map<K, V>) -> Result<()>";
        assert!(core_adapter(unit, "core_f", "Result<()>").contains("this.core_f(a, b)?)"));
    }
}

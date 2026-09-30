//! A-5 函数式接口合成对象的预扫描账本（← `emitter/sam_objects.py` 的 prescan / site_ctor_path /
//! record_site）。
//!
//! lambda 以与 `LambdaMetafactory` 产物同构的合成对象承载：每个被 invokedynamic 站点用作 samtype
//! 的函数式接口 `I` 在其翻译文件尾部生成 `I__Lambda`（文本见 [`crate::phase2::sam_objects`]）。
//! 接口集证据驱动：预扫描本轮全部字节码的 invokedynamic 站点，只有实际作为 samtype 出现、
//! 可合成（已发射、非手写、函数式）的接口进账本。
//!
//! 账本在类文本生成之前即须定案：方法体生成器（P4c）在 invokedynamic 站点经
//! [`SamLedger::site_ctor_path`] 判定走合成对象装箱还是回落闭包装箱，并以
//! [`crate::body::BodyEffects::sam_sites`] 登记站点；发射层收尾时经
//! [`SamLedger::check_sites`] 断言站点 samtype 与预扫描 SAM 描述符恒等。

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use classfile::{Method, Operand};
use indexmap::IndexMap;
use ty::type_map::{parse_descriptor_params, parse_descriptor_return};
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::error::{EmitError, Result};
use crate::phase2::sig::param_part;
use crate::phase2::uses::use_path;
use crate::text::to_snake;

/// 一个可合成函数式接口的预扫描结论
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SamSpec {
    pub iface_bin: String,
    /// 闭包上唯一未被 default 覆盖的抽象方法（可在任一闭包接口上声明）
    pub sam_name: String,
    pub sam_desc: String,
    /// SAM 擦除 Rust 签名（站点闭包与合成对象字段的公共类型）
    pub erased_params: Vec<String>,
    pub erased_ret: String,
    /// `[I]` + 传递超接口（广度优先发现序；含注册表外名字，只进 instanceof 名单）
    pub closure: Vec<String>,
}

/// 本轮全部可合成接口（接口 binary → 结论；键序即合成序）
#[derive(Debug, Default)]
pub struct SamLedger {
    pub specs: BTreeMap<String, SamSpec>,
}

/// `{I}` + 传递超接口（注册表内展开，广度优先发现序去重；注册表外名字保留）
pub fn iface_closure(ctx: &EmitCtx<'_>, iface: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    let mut queue = VecDeque::from([iface.to_string()]);
    while let Some(name) = queue.pop_front() {
        if !seen.insert(name.clone()) {
            continue;
        }
        if let Some(ci) = ctx.ty.reg.get(&name) {
            queue.extend(ci.interfaces().iter().cloned());
        }
        out.push(name);
    }
    out
}

fn root_keyed(ctx: &EmitCtx<'_>, m: &Method) -> bool {
    ctx.root_keys().contains(&(m.name.clone(), param_part(&m.desc).to_string()))
}

/// SAM 判定视角的接口实例方法（含 synthetic 桥接：桥接在 JVM 方法解析里是有效覆盖）
fn effective_view_methods<'c>(ctx: &EmitCtx<'_>, ci: &'c ClassInfo) -> Vec<&'c Method> {
    ci.methods()
        .iter()
        .filter(|m| !m.is_static() && !m.name.starts_with('<') && !m.is_private() && !root_keyed(ctx, m))
        .collect()
}

/// 接口自身声明的契约实例方法（进 `J__VTable` 的方法集；与接口方法块发射同源过滤）
pub fn contract_methods<'c>(ctx: &EmitCtx<'_>, ci: &'c ClassInfo) -> Vec<&'c Method> {
    effective_view_methods(ctx, ci).into_iter().filter(|m| !m.is_synthetic()).collect()
}

/// 接口闭包上的 SAM（JLS §9.8 / §9.4.4）：每个 (名, 参数描述符) 的有效声明取声明者中的
/// 极大元；有效抽象恰 1 个 → SAM
fn functional_sam(ctx: &EmitCtx<'_>, iface: &str) -> Option<SamSpec> {
    let reg = ctx.ty.reg;
    if !reg.get(iface)?.is_interface() {
        return None;
    }
    let closure = iface_closure(ctx, iface);
    // key → {声明接口: 是否有体}（插入序）；rep：key → {声明接口: 抽象声明}
    let mut decls: IndexMap<(String, String), IndexMap<&str, bool>> = IndexMap::new();
    let mut rep: BTreeMap<(String, String), BTreeMap<&str, &Method>> = BTreeMap::new();
    for jbin in &closure {
        let Some(jci) = reg.get(jbin).filter(|c| c.is_interface()) else { continue };
        for m in effective_view_methods(ctx, jci) {
            let key = (m.name.clone(), param_part(&m.desc).to_string());
            let concrete = !m.is_abstract();
            let per = decls.entry(key.clone()).or_default();
            let had = per.get(jbin.as_str()).copied().unwrap_or(false);
            if !had && !concrete {
                rep.entry(key).or_default().insert(jbin.as_str(), m);
            }
            per.insert(jbin.as_str(), had || concrete);
        }
    }
    let anc: BTreeMap<&str, BTreeSet<String>> = closure
        .iter()
        .filter(|j| reg.contains(j))
        .map(|j| (j.as_str(), iface_closure(ctx, j).into_iter().collect()))
        .collect();
    let mut uncovered: Vec<&Method> = Vec::new();
    for (key, per) in &decls {
        let maximal = per.keys().find(|j| !per.keys().any(|k| k != *j && anc.get(k).is_some_and(|a| a.contains(**j))));
        if let Some(j) = maximal {
            if !per[j] {
                if let Some(m) = rep.get(key).and_then(|r| r.get(j)) {
                    uncovered.push(m);
                }
            }
        }
    }
    let [m] = uncovered[..] else { return None };
    let names = ctx.ty.names;
    let erased_params = parse_descriptor_params(&m.desc).iter().map(|p| ctx.ty.jvm_to_rust(p).render(names)).collect();
    let rd = parse_descriptor_return(&m.desc);
    let erased_ret = if rd == "V" { "()".to_string() } else { ctx.ty.jvm_to_rust(rd).render(names) };
    Some(SamSpec { iface_bin: iface.to_string(), sam_name: m.name.clone(), sam_desc: m.desc.clone(), erased_params, erased_ret, closure })
}

/// 接口的目标文件是否被 runtime 真源手写覆盖（`<包>/<snake>.rs` 或 `_t` 后缀布局）
fn runtime_handwritten(ctx: &EmitCtx<'_>, iface: &str) -> bool {
    let (pkg, simple) = iface.rsplit_once('/').unwrap_or(("", iface));
    let dir = pkg.split('/').filter(|p| !p.is_empty()).fold(ctx.runtime_src(), |d, p| d.join(p));
    let stem = to_snake(simple);
    [stem.clone(), format!("{stem}_t")].iter().any(|s| dir.join(format!("{s}.rs")).is_file())
}

impl SamLedger {
    /// 预扫描用户类 + JDK 类全部方法（规范化方法体）的 invokedynamic 站点：
    /// 函数式接口 = 调用点描述符的返回类型
    pub fn prescan(ctx: &EmitCtx<'_>) -> SamLedger {
        let input = ctx.input;
        let mut candidates = BTreeSet::new();
        for cls in input.user_classes.iter().chain(&input.jdk_classes) {
            let Some(ci) = ctx.ty.reg.get(cls) else { continue };
            for m in ci.methods() {
                let Some(code) = input.code(cls, m) else { continue };
                for ni in &code.insns {
                    let Some(Operand::InvokeDynamic { desc, .. }) = ni.insn().map(|i| &i.operand) else { continue };
                    let rd = parse_descriptor_return(desc);
                    if let Some(b) = rd.strip_prefix('L').and_then(|r| r.strip_suffix(';')) {
                        candidates.insert(b.to_string());
                    }
                }
            }
        }
        let mut specs = BTreeMap::new();
        for iface in candidates {
            if !ctx.is_user(&iface) && runtime_handwritten(ctx, &iface) {
                continue;
            }
            if let Some(spec) = functional_sam(ctx, &iface) {
                specs.insert(iface, spec);
            }
        }
        SamLedger { specs }
    }

    pub fn get(&self, iface: &str) -> Option<&SamSpec> {
        self.specs.get(iface)
    }

    /// invokedynamic 站点的合成对象构造路径（全限定，免 import）：当前文件在 user crate →
    /// JDK 接口用 `java_runtime::` 前缀，在 java_runtime crate → `crate::`；用户接口一律
    /// 模块层路径 `crate::<mod>::<Short>`。None：该接口不可合成（站点回落闭包装箱）
    pub fn site_ctor_path(&self, ctx: &EmitCtx<'_>, iface: &str, current_class: &str) -> Option<String> {
        self.specs.get(iface)?;
        let recv_crate = ctx.lib_crate_of(current_class).unwrap_or(if ctx.is_user(current_class) { "user" } else { "java_runtime" });
        let crate_prefix = if recv_crate == "java_runtime" { "crate" } else { "java_runtime" };
        // 目标视图：注册表类的 emission 前缀（用户类 java_runtime / JDK crate）+ lib crate 归属
        let target_prefix = if ctx.is_user(iface) { "java_runtime" } else { "crate" };
        let target_crate = ctx.lib_crate_of(iface).unwrap_or("");
        Some(format!("{}__Lambda::new", use_path(ctx, iface, crate_prefix, Some((target_crate, target_prefix)), recv_crate)))
    }

    /// 站点一致性断言（G-10 同款）：站点 samtype 描述符与预扫描 SAM 描述符恒等
    pub fn check_sites(&self, sites: &[(String, String, String)]) -> Result<()> {
        for (iface, sam_desc, cur) in sites {
            if let Some(spec) = self.specs.get(iface).filter(|s| &s.sam_desc != sam_desc) {
                return Err(EmitError::Assert(format!(
                    "[sam-objects] samtype 描述符发散: {iface} 站点 {sam_desc} vs 预扫描 SAM {}（{cur}）",
                    spec.sam_desc
                )));
            }
        }
        Ok(())
    }
}

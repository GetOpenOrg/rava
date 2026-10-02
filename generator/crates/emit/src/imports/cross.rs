//! 跨类引用的结构化登记与 crate 定向。
//!
//! 本类的引用集（`referenced`，binary 集合）在文件作用域按 binary 序预认领；超类链
//! `__VTable` 与 `__base` 自由函数登记为派生名。各类只读自身引用记录，与发射序无关；
//! 导入行由作用域记录统一生成（[`super::fill`]）。

use std::collections::BTreeSet;

use classfile::{Insn, Method, Operand};
use ty::ident::safe_ident;
use ty::ClassInfo;

use instr::owner::{class_inherits_default_method, resolve_special_method_owner};
use crate::ctx::EmitCtx;
use crate::error::Result;
use crate::lang;
use crate::text::safe_pkg_part;

const INVOKESPECIAL: u8 = 0xb7;

/// 本类所在 crate 的导入参数
pub struct CrossInput<'s> {
    /// 本轮生成集；None = 本 crate 不导入生成类（无 JDK 类的用户 crate）
    pub generated: Option<&'s BTreeSet<String>>,
    /// `crate` / `java_runtime`
    pub prefix: &'s str,
    /// 调用链不约束（用户类）
    pub all_in_chain: bool,
    /// lib 模式（jar 输入）的按目标 crate 定向；None = 单 crate 前缀
    pub route: Option<CrateRoute<'s>>,
}

impl<'s> CrossInput<'s> {
    /// 引用类所用的 crate 前缀
    pub fn prefix_of(&self, bin: &str) -> &'s str {
        Prefix { base: self.prefix, route: self.route }.of(bin)
    }
}

/// lib 模式的 crate 定向：引用类按归属 crate 导入（当前 lib → `crate`，其它 lib → 该 lib 名，
/// 其余 → `java_runtime`）
#[derive(Clone, Copy)]
pub struct CrateRoute<'s> {
    /// lib crate（声明序 = 依赖方向：后声明者依赖先声明者）→ 发射类集合
    pub libs: &'s [(String, BTreeSet<String>)],
    /// 当前 crate：Some(lib 名) / None = user crate
    pub current: Option<&'s str>,
}

impl<'s> CrateRoute<'s> {
    pub fn target(&self, bin: &str) -> &'s str {
        match self.libs.iter().find(|(_, set)| set.contains(bin)) {
            Some((n, _)) if Some(n.as_str()) == self.current => "crate",
            Some((n, _)) => n.as_str(),
            None => "java_runtime",
        }
    }

    /// lib crate 只能引用自身 / java_runtime / 声明序在前的 lib crate（反向引用来自子类型收集，
    /// 导入即 E0433）；user crate 依赖全部 lib
    pub(crate) fn reachable(&self, target: &str) -> bool {
        let pos = |n: &str| self.libs.iter().position(|(l, _)| l == n);
        match (self.current.and_then(pos), pos(target)) {
            (Some(c), Some(t)) => t <= c,
            _ => true,
        }
    }
}

/// 按文本补导（scan）的前缀：单 crate 前缀，或 lib 模式按目标 crate 定向
#[derive(Clone, Copy)]
pub struct Prefix<'s> {
    pub base: &'s str,
    pub route: Option<CrateRoute<'s>>,
}

impl<'s> Prefix<'s> {
    pub fn of(&self, bin: &str) -> &'s str {
        self.route.map_or(self.base, |r| r.target(bin))
    }
}

/// 包路径（`/` 段 → `::`，关键字 `r#`）
pub fn rust_pkg_of(bin: &str) -> Option<String> {
    let (pkg, _) = bin.rsplit_once('/')?;
    Some(pkg.split('/').map(safe_pkg_part).collect::<Vec<_>>().join("::"))
}

/// 结构化引用登记：引用集（binary 序）在本文件作用域预认领 → 超类链 `__VTable` → `__base`
/// 自由函数（派生名登记）。导入行由作用域记录在第二阶段后统一生成（[`super::fill`]）
pub fn claim_structural(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, referenced: &BTreeSet<String>) -> Result<()> {
    ctx.ty.preclaim(referenced.iter().map(String::as_str));
    // 不透明类（L1）无 vtable 实现 / 方法体：不引祖先 vtable 与 `__base`
    if !ci.is_interface() && !ctx.is_opaque(ci.name()) {
        vtable_claims(ctx, ci, inp);
        base_fn_claims(ctx, ci, inp)?;
    }
    Ok(())
}

/// 超类链祖先的 `Ancestor__VTable`（宏生成 `impl Ancestor__VTable for Self`）
fn vtable_claims(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>) {
    let mut cur = ci.super_class();
    while !cur.is_empty() && cur != lang::OBJECT {
        if inp.generated.is_none_or(|g| g.contains(cur)) || !cur.contains('/') {
            ctx.ty.derived(cur, "__VTable");
        }
        match ctx.ty.reg.get(cur) {
            Some(c) => cur = c.super_class(),
            None => break,
        }
    }
}

/// `__base` 扫描面：本类方法 + 祖先未被覆盖的具体实例方法（super-inherit 展开进本类）
fn base_scan_methods<'a>(ctx: &EmitCtx<'a>, ci: &'a ClassInfo) -> Vec<(&'a str, &'a Method)> {
    let mut out: Vec<(&str, &Method)> = ci.methods().iter().map(|m| (ci.name(), m)).collect();
    let mut declared: BTreeSet<(&str, &str)> = ci.methods().iter().map(|m| (m.name.as_str(), m.desc.as_str())).collect();
    let mut anc = ci.super_class();
    while let Some(a) = ctx.ty.reg.get(anc) {
        for m in a.methods() {
            if m.name.starts_with('<') || m.is_static() || m.is_abstract() {
                continue;
            }
            if declared.insert((m.name.as_str(), m.desc.as_str())) {
                out.push((a.name(), m));
            }
        }
        anc = a.super_class();
    }
    out
}

/// 继承链顶（无父类的祖先）
fn chain_top<'a>(ctx: &EmitCtx<'a>, ci: &'a ClassInfo) -> &'a str {
    let mut top = ci.name();
    while let Some(c) = ctx.ty.reg.get(top).filter(|c| !c.super_class().is_empty()) {
        top = c.super_class();
    }
    top
}

/// `__base` 自由函数登记：调用链上方法体里的非 `<init>` invokespecial 落点
fn base_fn_claims(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>) -> Result<()> {
    let reg = ctx.ty.reg;
    for (owner, m) in base_scan_methods(ctx, ci) {
        if !inp.all_in_chain && !ctx.in_chain(owner, &m.name, &m.desc) {
            continue;
        }
        let Some(code) = ctx.input.code_ops(owner, m) else { continue };
        for insn in code.ops() {
            let Insn { opcode: INVOKESPECIAL, operand: Operand::Method(r, _), .. } = insn else { continue };
            if r.name == "<init>" || reg.get(&r.owner).is_some_and(ClassInfo::is_interface) {
                continue;
            }
            let orig = resolve_special_method_owner(reg, &r.owner, &r.name, &r.desc);
            if orig == ci.name() {
                continue;
            }
            // 不占槽的落点无 `__base` 自由函数（调用点改为 wrapper 直接调用）
            let pruned = reg
                .get(&orig)
                .and_then(|oc| oc.methods().iter().find(|am| am.name == r.name && am.desc == r.desc).map(|am| ctx.slot_pruned(am, oc)));
            if pruned == Some(true) {
                continue;
            }
            if orig.contains('/') {
                let is_root = orig == chain_top(ctx, ci) && orig != ci.name();
                if !inp.generated.is_some_and(|g| g.contains(&orig)) && !is_root {
                    continue;
                }
                if let Some(oc) = reg.get(&orig) {
                    if !oc.methods().iter().any(|am| am.name == r.name)
                        && !class_inherits_default_method(reg, &orig, &r.name, &r.desc)
                    {
                        continue;
                    }
                }
            }
            // 与调用点（instr invokespecial `__base` 路由）同一命名函数、同一声明类实参
            let ictx = instr::InstrCtx::new(ctx.ty, ctx.manifest, ctx.instr_facts(), &instr::NoHooks, ci.name());
            let rust_m = safe_ident(&instr::naming::mangle_if_overloaded(&ictx, &orig, &r.name, Some(&r.desc)).map_err(|e| crate::error::EmitError::Body(e.to_string()))?);
            ctx.ty.derived(&orig, &format!("__{rust_m}_base"));
        }
    }
    Ok(())
}

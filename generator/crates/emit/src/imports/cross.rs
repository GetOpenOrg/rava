//! 精确 cross_imports（← `import_gen.gen_cross_imports` / `_add_precise_import`）。
//!
//! 顺序与 Python 一致：JDK 包 → 同包兄弟 → 同名消歧 → 跳过包 → 超类链 `__VTable` →
//! `__base` 自由函数 → 用户类兄弟模块。`seen_simples`（跨包同名冲突守卫）跨类累积，
//! 经 [`ProjectState`] 按发射序传递。

use std::collections::{BTreeMap, BTreeSet};

use classfile::{Insn, Method, Operand};
use ty::ident::safe_ident;
use ty::ClassInfo;

use instr::owner::{class_inherits_default_method, resolve_special_method_owner};
use crate::ctx::{EmitCtx, ProjectState};
use crate::error::Result;
use crate::lang;
use crate::text::{safe_pkg_part, to_snake};

const INVOKESPECIAL: u8 = 0xb7;
/// prelude 同名 newtype：不 use 导入（调用方走全路径）
const PRELUDE_NEWTYPE_NAMES: [&str; 1] = ["JArray"];

/// 本类所在 crate 的导入参数
pub struct CrossInput<'s> {
    /// crate 内跨包导入的包路径（`java::util` 形态）；None / 空 = 不做包集合导入
    pub pkg_paths: Option<&'s [String]>,
    /// 本轮生成集；None = 不过滤
    pub generated: Option<&'s BTreeSet<String>>,
    pub conflict_map: Option<&'s BTreeMap<String, Vec<String>>>,
    pub skipped: Option<&'s BTreeSet<String>>,
    pub sibling_imports: &'s [String],
    /// `crate` / `java_runtime`
    pub prefix: &'s str,
    /// 调用链不约束（用户类）
    pub all_in_chain: bool,
}

/// 包路径（`/` 段 → `::`，关键字 `r#`）
pub fn rust_pkg_of(bin: &str) -> Option<String> {
    let (pkg, _) = bin.rsplit_once('/')?;
    Some(pkg.split('/').map(safe_pkg_part).collect::<Vec<_>>().join("::"))
}

struct Acc<'x> {
    prefix: &'x str,
    self_simple: String,
    seen: BTreeSet<String>,
    lines: Vec<String>,
}

impl Acc<'_> {
    /// `_add_precise_import`
    fn precise(&mut self, ctx: &EmitCtx<'_>, st: &mut ProjectState, full: &str) {
        let Some(pkg) = rust_pkg_of(full) else { return };
        let simple = ctx.short(full);
        if simple == self.self_simple {
            return;
        }
        let key = format!("{pkg}::{simple}");
        if self.seen.contains(&key) || st.seen_simples.get(&simple).is_some_and(|k| *k != key) {
            return;
        }
        self.seen.insert(key.clone());
        st.seen_simples.insert(simple.clone(), key);
        if PRELUDE_NEWTYPE_NAMES.contains(&simple.as_str()) {
            return;
        }
        self.lines.push(format!("use {}::{pkg}::{simple};", self.prefix));
    }
}

fn pkg_of(bin: &str) -> Option<&str> {
    bin.rsplit_once('/').map(|(p, _)| p)
}

/// `gen_cross_imports`
pub fn gen_cross_imports(
    ctx: &EmitCtx<'_>,
    st: &mut ProjectState,
    ci: &ClassInfo,
    inp: &CrossInput<'_>,
    referenced: &BTreeSet<String>,
) -> Result<Vec<String>> {
    let mut acc = Acc { prefix: inp.prefix, self_simple: ctx.short(ci.name()), seen: BTreeSet::new(), lines: Vec::new() };
    let pkg_paths = inp.pkg_paths.filter(|p| !p.is_empty());
    if let Some(paths) = pkg_paths {
        let slash: BTreeSet<String> = paths.iter().map(|p| p.replace("::", "/").replace("r#", "")).collect();
        for full in referenced {
            if pkg_of(full).is_some_and(|p| slash.contains(p)) {
                acc.precise(ctx, st, full);
            }
        }
    }
    if let Some(own_pkg) = pkg_of(ci.name()) {
        let own_path = rust_pkg_of(ci.name()).unwrap_or_default();
        if !pkg_paths.is_some_and(|p| p.contains(&own_path)) {
            for full in referenced {
                if pkg_of(full) == Some(own_pkg) {
                    acc.precise(ctx, st, full);
                }
            }
        }
    }
    for (sn, pkgs) in inp.conflict_map.into_iter().flatten() {
        let mut pkgs = pkgs.clone();
        pkgs.sort();
        for p in pkgs {
            let full = format!("{p}/{sn}");
            if referenced.contains(&full) {
                acc.precise(ctx, st, &full);
            }
        }
    }
    if let Some(skipped) = inp.skipped.filter(|s| !s.is_empty()) {
        for full in referenced {
            let Some(pkg) = rust_pkg_of(full) else { continue };
            let simple = ctx.short(full);
            let key = format!("{pkg}::{simple}");
            if skipped.contains(&key)
                && simple != acc.self_simple
                && !acc.seen.contains(&key)
                && st.seen_simples.get(&simple).is_none_or(|k| *k == key)
            {
                acc.lines.push(format!("use {}::{key};", inp.prefix));
                acc.seen.insert(key.clone());
                st.seen_simples.insert(simple, key);
            }
        }
    }
    if !ci.is_interface() {
        vtable_imports(ctx, ci, inp, &mut acc);
        base_fn_imports(ctx, ci, inp, &mut acc)?;
    }
    acc.lines.extend(inp.sibling_imports.iter().cloned());
    Ok(acc.lines)
}

/// 超类链祖先的 `Ancestor__VTable` 导入（宏生成 `impl Ancestor__VTable for Self`）
fn vtable_imports(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, acc: &mut Acc<'_>) {
    let mut cur = ci.super_class();
    while !cur.is_empty() && cur != lang::OBJECT {
        if inp.generated.is_none_or(|g| g.contains(cur)) || !cur.contains('/') {
            let (key, line) = match rust_pkg_of(cur) {
                Some(pkg) => {
                    let simple = ctx.short(cur);
                    (format!("{pkg}::{simple}__VTable"), format!("use {}::{pkg}::{simple}__VTable;", inp.prefix))
                }
                None => {
                    let (simple, m) = (cur.replace('$', "_"), to_snake(cur));
                    (format!("crate::{m}::{simple}__VTable"), format!("use crate::{m}::{simple}__VTable;"))
                }
            };
            if acc.seen.insert(key) {
                acc.lines.push(line);
            }
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

/// `__base` 自由函数导入：调用链上方法体里的非 `<init>` invokespecial 落点
fn base_fn_imports(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, acc: &mut Acc<'_>) -> Result<()> {
    let reg = ctx.ty.reg;
    for (owner, m) in base_scan_methods(ctx, ci) {
        if !inp.all_in_chain && !ctx.in_chain(owner, &m.name, &m.desc) {
            continue;
        }
        let Some(code) = ctx.input.code(owner, m) else { continue };
        for ni in &code.insns {
            let Some(Insn { opcode: INVOKESPECIAL, operand: Operand::Method(r, _), .. }) = ni.insn() else { continue };
            if r.name == "<init>" || reg.get(&r.owner).is_some_and(ClassInfo::is_interface) {
                continue;
            }
            let orig = resolve_special_method_owner(reg, &r.owner, &r.name, &r.desc);
            if orig == ci.name() {
                continue;
            }
            let is_jdk = orig.contains('/');
            let (base_simple, base_mod) = if is_jdk {
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
                (ctx.short(&orig), rust_pkg_of(&orig).unwrap_or_default())
            } else {
                (orig.replace('$', "_"), to_snake(&orig))
            };
            // 与调用点（instr invokespecial `__base` 路由）同一命名函数、同一声明类实参
            let ictx = instr::InstrCtx::new(ctx.ty, ctx.manifest, ctx.instr_facts(), &instr::NoHooks, ci.name());
            let rust_m = safe_ident(&instr::naming::mangle_if_overloaded(&ictx, &orig, &r.name, Some(&r.desc)).map_err(|e| crate::error::EmitError::Body(e.to_string()))?);
            let base_fn = format!("{base_simple}__{rust_m}_base");
            if acc.seen.insert(format!("crate::{base_mod}::{base_fn}")) {
                let bprefix = if is_jdk { inp.prefix } else { "crate" };
                acc.lines.push(format!("use {bprefix}::{base_mod}::{base_fn};"));
            }
        }
    }
    Ok(())
}

//! 精确 cross_imports（← `import_gen.gen_cross_imports` / `_add_precise_import`）。
//!
//! 顺序与 Python 一致：JDK 包 → 同包兄弟 → 同名消歧 → 跳过包 → 超类链 `__VTable` →
//! `__base` 自由函数 → 用户类兄弟模块。`seen_simples`（跨包同名冲突守卫）跨类累积，
//! 经 [`ProjectState`] 按发射序传递：[`plan_cross_imports`] 逐类（可并行）求出短名候选与
//! 发射序无关的尾段，[`CrossPlan::resolve`] 按发射序串行裁决。

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
    fn reachable(&self, target: &str) -> bool {
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

struct Acc {
    self_simple: String,
    /// 本类已登记的导入键：短名候选键（`pkg::Simple`）与 `__VTable` / `__base` 键格式互不相交
    seen: BTreeSet<String>,
    /// 与发射序相关的短名候选（首次出现序，已按键去重）
    candidates: Vec<Candidate>,
    /// 与发射序无关的尾段（`__VTable` / `__base` / 兄弟模块）
    lines: Vec<String>,
}

/// 短名导入候选：是否采纳取决于 `seen_simples`（跨类累积，首个引入者胜出）
#[derive(Debug, Clone)]
struct Candidate {
    key: String,
    simple: String,
    /// 采纳时输出的 use 行；prelude 同名 newtype 只登记短名、不输出
    line: Option<String>,
}

impl Acc {
    /// 登记短名候选（`_add_precise_import` / 跳过包导入的共同形态）。
    ///
    /// 同键重复出现可在本地去重：`seen_simples` 的值一经写入不再改变（只在缺席或同键时写入），
    /// 首次出现被拒则之后同键必然被拒，首次出现被采纳则之后同键被本类 `seen` 拒绝
    fn candidate(&mut self, key: String, simple: String, line: Option<String>) {
        if simple == self.self_simple || !self.seen.insert(key.clone()) {
            return;
        }
        self.candidates.push(Candidate { key, simple, line });
    }

    /// `_add_precise_import`
    fn precise(&mut self, ctx: &EmitCtx<'_>, full: &str, prefix: &str) {
        let Some(pkg) = rust_pkg_of(full) else { return };
        let simple = ctx.short(full);
        let key = format!("{pkg}::{simple}");
        let line = (!PRELUDE_NEWTYPE_NAMES.contains(&simple.as_str())).then(|| format!("use {prefix}::{key};"));
        self.candidate(key, simple, line);
    }
}

/// 单类跨类导入的发射序无关部分（可按类并行求得）；[`CrossPlan::resolve`] 按发射序串行裁决
#[derive(Debug, Clone)]
pub struct CrossPlan {
    candidates: Vec<Candidate>,
    tail: Vec<String>,
}

impl CrossPlan {
    /// 按发射序裁决短名候选：简单名未被别的路径占用时采纳并登记（首个引入者胜出）
    pub fn resolve(&self, seen_simples: &mut BTreeMap<String, String>) -> Vec<String> {
        let mut lines = Vec::with_capacity(self.candidates.len() + self.tail.len());
        for c in &self.candidates {
            if seen_simples.get(&c.simple).is_some_and(|k| *k != c.key) {
                continue;
            }
            seen_simples.insert(c.simple.clone(), c.key.clone());
            lines.extend(c.line.clone());
        }
        lines.extend(self.tail.iter().cloned());
        lines
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
    Ok(plan_cross_imports(ctx, ci, inp, referenced)?.resolve(&mut st.seen_simples))
}

/// 跨类导入规划：JDK 包 → 同包兄弟 → 同名消歧 → 跳过包 的短名候选（发射序相关），
/// 以及超类链 `__VTable` → `__base` 自由函数 → 用户类兄弟模块（发射序无关）
pub fn plan_cross_imports(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, referenced: &BTreeSet<String>) -> Result<CrossPlan> {
    let mut acc = Acc { self_simple: ctx.short(ci.name()), seen: BTreeSet::new(), candidates: Vec::new(), lines: Vec::new() };
    if let Some(route) = inp.route {
        // lib 模式：生成集内的有包引用类逐个定向，跳过包集合 / 同包 / 消歧 / 跳过包分支
        let empty = BTreeSet::new();
        let generated = inp.generated.unwrap_or(&empty);
        for full in referenced {
            if full == ci.name() || !generated.contains(full) || !full.contains('/') {
                continue;
            }
            let target = route.target(full);
            if route.reachable(target) {
                acc.precise(ctx, full, target);
            }
        }
        return finish(ctx, ci, inp, acc);
    }
    let pkg_paths = inp.pkg_paths.filter(|p| !p.is_empty());
    if let Some(paths) = pkg_paths {
        let slash: BTreeSet<String> = paths.iter().map(|p| p.replace("::", "/").replace("r#", "")).collect();
        for full in referenced {
            if pkg_of(full).is_some_and(|p| slash.contains(p)) {
                acc.precise(ctx, full, inp.prefix);
            }
        }
    }
    if let Some(own_pkg) = pkg_of(ci.name()) {
        let own_path = rust_pkg_of(ci.name()).unwrap_or_default();
        if !pkg_paths.is_some_and(|p| p.contains(&own_path)) {
            for full in referenced {
                if pkg_of(full) == Some(own_pkg) {
                    acc.precise(ctx, full, inp.prefix);
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
                acc.precise(ctx, &full, inp.prefix);
            }
        }
    }
    if let Some(skipped) = inp.skipped.filter(|s| !s.is_empty()) {
        for full in referenced {
            let Some(pkg) = rust_pkg_of(full) else { continue };
            let simple = ctx.short(full);
            let key = format!("{pkg}::{simple}");
            if skipped.contains(&key) {
                let line = format!("use {}::{key};", inp.prefix);
                acc.candidate(key, simple, Some(line));
            }
        }
    }
    finish(ctx, ci, inp, acc)
}

/// 超类链 `__VTable` → `__base` 自由函数 → 用户类兄弟模块
fn finish(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, mut acc: Acc) -> Result<CrossPlan> {
    if !ci.is_interface() {
        vtable_imports(ctx, ci, inp, &mut acc);
        base_fn_imports(ctx, ci, inp, &mut acc)?;
    }
    acc.lines.extend(inp.sibling_imports.iter().cloned());
    Ok(CrossPlan { candidates: acc.candidates, tail: acc.lines })
}

/// 超类链祖先的 `Ancestor__VTable` 导入（宏生成 `impl Ancestor__VTable for Self`）
fn vtable_imports(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, acc: &mut Acc) {
    let mut cur = ci.super_class();
    while !cur.is_empty() && cur != lang::OBJECT {
        if inp.generated.is_none_or(|g| g.contains(cur)) || !cur.contains('/') {
            let (key, line) = match rust_pkg_of(cur) {
                Some(pkg) => {
                    let simple = ctx.short(cur);
                    (format!("{pkg}::{simple}__VTable"), format!("use {}::{pkg}::{simple}__VTable;", inp.prefix_of(cur)))
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
fn base_fn_imports(ctx: &EmitCtx<'_>, ci: &ClassInfo, inp: &CrossInput<'_>, acc: &mut Acc) -> Result<()> {
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
                let bprefix = if is_jdk { inp.prefix_of(&orig) } else { "crate" };
                acc.lines.push(format!("use {bprefix}::{base_mod}::{base_fn};"));
            }
        }
    }
    Ok(())
}

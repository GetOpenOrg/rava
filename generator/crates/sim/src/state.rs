//! 模拟器本体：配置、状态、栈操作、局部变量读取与槽位命名（← `StackSim` 除 store 外的部分）。

use crate::env::{ident, to_ir_type, type_text, SimEnv};
use crate::error::SimResult;
use crate::exprs::{is_trivial, materialized_needs_type, reads_state, underflow_value};
use crate::names::safe_name;
use crate::types::is_scalar;
use ir::{Expr, Ident, LetStmt, Stmt};
use std::collections::{BTreeMap, BTreeSet};
use ty::{Prim, RsType};

/// 栈值身份：`dup` 出的副本共享同一身份（Python 以对象 `is` 判定），
/// 消费其中一份时其余副本物化为临时变量。
pub type ValueId = u32;

/// 操作数栈条目
#[derive(Debug, Clone, PartialEq)]
pub struct StackEntry {
    pub expr: Expr,
    pub ty: RsType,
    pub id: ValueId,
}

/// 局部变量槽的当前绑定（`locals[slot] = (name, ty, is_new)`）
#[derive(Debug, Clone, PartialEq)]
pub struct Local {
    pub name: Ident,
    pub ty: RsType,
    /// 由方法体内 store 新建（形参与 `this` 为 false）
    pub is_new: bool,
}

/// LocalVariableTable / LocalVariableTypeTable 的一个声明区间
#[derive(Debug, Clone, PartialEq)]
pub struct SlotDecl {
    pub start: u32,
    pub end: u32,
    /// Java 变量名（未安全化）
    pub name: String,
    /// 声明类型；None = 根类 / 接口声明的引用变量
    pub ty: Option<RsType>,
    /// `ty` 来自泛型签名（LVTT），否则来自描述符
    pub from_sig: bool,
    /// LVTT 原始签名（菱形构造的类型实参求解用）
    pub raw_sig: String,
    /// LVT 描述符
    pub desc: String,
}

/// 模拟器配置（方法级只读输入）
#[derive(Debug, Clone, PartialEq)]
pub struct SimConfig {
    /// 形参类型（不含 `this`）
    pub params: Vec<RsType>,
    pub is_static: bool,
    /// 所属类 JVM binary 名（空 = 未知，`this` 按 Object）
    pub class_name: String,
    /// 类级类型形参（`this` 的实参按此序）
    pub class_type_params: Vec<String>,
    /// 槽位 → Java 变量名（无调试信息时缺省）
    pub local_names: BTreeMap<u16, String>,
    /// 槽位 → 声明区间（按起点升序）
    pub slot_decls: BTreeMap<u16, Vec<SlotDecl>>,
    pub return_type: RsType,
    pub is_constructor: bool,
    pub in_vtable_body: bool,
}

/// 模拟器可变状态（P4b 按块保存 / 恢复；字段语义同 Python 同名属性）
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SimState {
    pub stack: Vec<StackEntry>,
    pub locals: BTreeMap<u16, Local>,
    pub stmts: Vec<Stmt>,
    /// 临时变量计数（`_ctr`）
    pub counter: u32,
    /// 下一个栈值身份
    pub next_id: ValueId,
    /// 形参占用的槽位（类型由签名决定）
    pub param_slots: BTreeSet<u16>,
    /// 合成槽按类型类别分名：slot → 类别序列（标量渲染名或 `ref`）
    pub synth_slot_kinds: BTreeMap<u16, Vec<String>>,
    /// 当前字节码偏移
    pub current_offset: u32,
    /// 下一条指令偏移（store 后变量作用域起点）
    pub next_offset: u32,
    /// 嵌套作用域深度（T68）
    pub depth: u32,
    /// slot → 当前绑定声明时的嵌套深度
    pub slot_decl_depth: BTreeMap<u16, u32>,
    /// slot → 当前绑定创建时的字节码偏移
    pub slot_bind_pos: BTreeMap<u16, u32>,
    /// 发生过栈下溢（方法体整体退化为存根）
    pub underflow: bool,
    /// 类级类型变量 → 上界类型
    pub type_var_bounds: BTreeMap<String, RsType>,
}

/// JVM 操作数栈模拟器
pub struct StackSim<'e> {
    pub cfg: SimConfig,
    pub state: SimState,
    pub(crate) env: &'e dyn SimEnv,
    pub(crate) class_tparams: BTreeSet<String>,
}

fn is_wide(t: &RsType) -> bool {
    matches!(t, RsType::Prim(Prim::I64 | Prim::F64))
}

impl<'e> StackSim<'e> {
    /// 按配置建立形参 / `this` 绑定（`StackSim.__init__`）
    pub fn new(cfg: SimConfig, env: &'e dyn SimEnv) -> SimResult<StackSim<'e>> {
        let mut sim = StackSim::from_state(cfg, SimState::default(), env);
        let mut slot: u16 = 0;
        if !sim.cfg.is_static {
            let this_ty = if sim.cfg.class_name.is_empty() {
                RsType::Object
            } else {
                let args = sim.cfg.class_type_params.iter().map(|p| RsType::Param(p.clone())).collect();
                RsType::class(sim.cfg.class_name.clone(), args)
            };
            sim.state.locals.insert(0, Local { name: ident("this")?, ty: this_ty, is_new: false });
            sim.state.slot_decl_depth.insert(0, 0);
            slot = 1;
        }
        for (idx, rt) in sim.cfg.params.clone().into_iter().enumerate() {
            // 无调试信息时的缺省名按形参序号（long / double 之后按槽位命名会错位）
            let raw = sim.cfg.local_names.get(&slot).cloned().unwrap_or_else(|| format!("arg_{idx}"));
            let width = if is_wide(&rt) { 2 } else { 1 };
            sim.state.locals.insert(slot, Local { name: ident(&safe_name(&raw))?, ty: rt, is_new: false });
            sim.state.slot_decl_depth.insert(slot, 0);
            sim.state.param_slots.insert(slot);
            slot += width;
        }
        Ok(sim)
    }

    /// 以既有状态恢复模拟器（P4b 子路径 / golden 回放）
    pub fn from_state(cfg: SimConfig, state: SimState, env: &'e dyn SimEnv) -> StackSim<'e> {
        let class_tparams = cfg.class_type_params.iter().cloned().collect();
        StackSim { cfg, state, env, class_tparams }
    }

    pub fn env(&self) -> &'e dyn SimEnv {
        self.env
    }

    // ── 作用域深度（T68：JVM slot 复用检测）──────────────────────────────────

    pub fn enter_scope(&mut self) {
        self.state.depth += 1;
    }

    pub fn exit_scope(&mut self) {
        self.state.depth = self.state.depth.saturating_sub(1);
    }

    /// 新临时变量名 `{prefix}{n}`
    pub fn fresh(&mut self, prefix: &str) -> SimResult<Ident> {
        let name = format!("{prefix}{}", self.state.counter);
        self.state.counter += 1;
        ident(&name)
    }

    // ── 栈操作 ───────────────────────────────────────────────────────────────

    /// 压入新值（新身份）
    pub fn push(&mut self, expr: Expr, ty: RsType) -> ValueId {
        let id = self.state.next_id;
        self.state.next_id += 1;
        self.state.stack.push(StackEntry { expr, ty, id });
        id
    }

    /// 压入既有条目（保留身份：`dup` 系列）
    pub fn push_entry(&mut self, entry: StackEntry) {
        self.state.next_id = self.state.next_id.max(entry.id + 1);
        self.state.stack.push(entry);
    }

    /// 弹栈；被 dup 的非平凡值若仍有副本在栈上，先物化为 `let _tN`，副本改引用该变量。
    /// 栈下溢返回占位值并置 [`SimState::underflow`]。
    pub fn pop(&mut self) -> SimResult<StackEntry> {
        let Some(top) = self.state.stack.pop() else {
            self.state.underflow = true;
            let id = self.state.next_id;
            self.state.next_id += 1;
            return Ok(StackEntry { expr: underflow_value()?, ty: RsType::Prim(Prim::I32), id });
        };
        if is_trivial(&top.expr) || !self.state.stack.iter().any(|e| e.id == top.id) {
            return Ok(top);
        }
        self.spill_stateful(Some(top.id))?;
        let name = self.materialize(top.expr, &top.ty)?;
        for e in self.state.stack.iter_mut().filter(|e| e.id == top.id) {
            e.expr = Expr::Var(name.clone());
        }
        Ok(StackEntry { expr: Expr::Var(name), ty: top.ty, id: top.id })
    }

    /// `let _tN[: ty] = value;`（类型注解按 [`materialized_needs_type`]），返回变量名
    fn materialize(&mut self, value: Expr, ty: &RsType) -> SimResult<Ident> {
        let name = self.fresh("_t")?;
        let ty = if materialized_needs_type(&value) { Some(to_ir_type(ty, self.env)?) } else { None };
        self.state.stmts.push(Stmt::Let(LetStmt::new(name.clone(), ty, Some(value))));
        Ok(name)
    }

    /// 发射语句前，把栈上依赖可变状态 / 带副作用的待求值条目（[`reads_state`]）按栈序物化：
    /// JVM 在该语句之前已求值它们，留在栈上会被推迟到语句之后（`f() + "-" + x + "-" + g()`
    /// 中 `x` 的读取不得晚于 `g()`）。同一身份（dup 副本）只物化一次；`keep` 是正由调用方物化的身份
    pub(crate) fn spill_stateful(&mut self, keep: Option<ValueId>) -> SimResult<()> {
        for i in 0..self.state.stack.len() {
            let e = &self.state.stack[i];
            if Some(e.id) == keep || !reads_state(&e.expr) {
                continue;
            }
            let (id, expr, ty) = (e.id, e.expr.clone(), e.ty.clone());
            let name = self.materialize(expr, &ty)?;
            for e in self.state.stack.iter_mut().filter(|e| e.id == id) {
                e.expr = Expr::Var(name.clone());
            }
        }
        Ok(())
    }

    /// xstore 用弹栈：不物化 dup 副本（store 以局部变量本身作为物化结果）
    pub fn pop_for_store(&mut self) -> SimResult<StackEntry> {
        match self.state.stack.pop() {
            Some(top) => Ok(top),
            None => self.pop(),
        }
    }

    /// 发射语句（先物化栈上待求值的有状态条目，见 [`Self::spill_stateful`]）
    pub fn emit(&mut self, stmt: Stmt) -> SimResult<()> {
        self.spill_stateful(None)?;
        self.state.stmts.push(stmt);
        Ok(())
    }

    /// `let {prefix}N: ty = value;`，返回 `Var(prefixN)`
    pub fn fresh_let(&mut self, prefix: &str, value: Expr, ty: &RsType) -> SimResult<Expr> {
        self.spill_stateful(None)?;
        let name = self.fresh(prefix)?;
        let ir_ty = to_ir_type(ty, self.env)?;
        self.state.stmts.push(Stmt::Let(LetStmt::new(name.clone(), Some(ir_ty), Some(value))));
        Ok(Expr::Var(name))
    }

    // ── 局部变量 ─────────────────────────────────────────────────────────────

    /// 按当前偏移查 slot 的声明：覆盖当前偏移的区间优先；store 时按 next_offset 精确匹配
    /// 初始化 store（`start == next` 或 `next <= off < start <= off + 4`）
    pub fn decl_at(&self, slot: u16, for_store: bool) -> Option<&SlotDecl> {
        let entries = self.cfg.slot_decls.get(&slot)?;
        let off = self.state.current_offset;
        if let Some(d) = entries.iter().find(|d| d.start <= off && off < d.end) {
            return Some(d);
        }
        if !for_store {
            return None;
        }
        let nxt = self.state.next_offset;
        entries.iter().find(|d| d.start == nxt || (nxt <= off && off < d.start && d.start <= off + 4))
    }

    /// 合成槽名：同一槽位的合成临时变量按类型类别（标量各自一类，引用同属 `ref`）分名，
    /// 首个类别 `local_N`，其后 `local_N_k`；ty=None → 首名
    pub fn synth_slot_name(&mut self, slot: u16, ty: Option<&RsType>) -> String {
        let base = format!("local_{slot}");
        let Some(ty) = ty else {
            return base;
        };
        let key = if is_scalar(ty) { type_text(ty, self.env) } else { "ref".to_string() };
        let seen = self.state.synth_slot_kinds.entry(slot).or_default();
        let k = match seen.iter().position(|s| *s == key) {
            Some(k) => k,
            None => {
                seen.push(key);
                seen.len() - 1
            }
        };
        if k == 0 {
            base
        } else {
            format!("{base}_{k}")
        }
    }

    /// 当前偏移无声明覆盖的 slot 的变量名（槽位被别的 Java 变量复用 / 无 LVT 名 → 合成名）
    pub fn undeclared_slot_name(&mut self, slot: u16, ty: Option<&RsType>) -> String {
        let has_decls = self.cfg.slot_decls.get(&slot).is_some_and(|v| !v.is_empty());
        if has_decls && !self.state.param_slots.contains(&slot) {
            return self.synth_slot_name(slot, ty);
        }
        if let Some(n) = self.cfg.local_names.get(&slot) {
            return safe_name(n);
        }
        self.synth_slot_name(slot, ty)
    }

    /// 读取局部变量槽：已绑定 → 变量；否则按声明表（跨模拟路径）或合成名，缺省 i32
    pub fn load_local(&mut self, slot: u16) -> SimResult<(Expr, RsType)> {
        if let Some(l) = self.state.locals.get(&slot) {
            return Ok((Expr::Var(l.name.clone()), l.ty.clone()));
        }
        if let Some(d) = self.decl_at(slot, false) {
            let ty = d.ty.clone().unwrap_or(RsType::Object);
            return Ok((Expr::Var(ident(&safe_name(&d.name))?), ty));
        }
        let name = self.undeclared_slot_name(slot, None);
        Ok((Expr::Var(ident(&name)?), RsType::Prim(Prim::I32)))
    }
}

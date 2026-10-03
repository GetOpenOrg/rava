//! 档案侧继承成员需求（T1 档案化 1b，计划 `docs/plans/2026-10-01-cross-test-compile-reuse.md` §6.4）。
//!
//! 继承成员（接收者类未声明、祖先声明的方法的转发成员）写进接收者类的文件。接收者是非用户类时，
//! 该文件属于档案 crate，其内容只能依赖档案事实：
//! - 档案侧需求 = 手写体需求（`hw_inherited`）+ 调用链上符号引用里属主未声明的实例方法
//!   （`visited` 含 refs 中属主未声明者；档案的 refs 是各入口之并，含各用户程序的调用点）
//!   + 非用户类发射登记的需求；
//! - 用户类方法体对非用户接收者登记的需求不入账：它们应已被档案侧覆盖。未被覆盖的需求
//!   （`[archive-leak]`）仍入账以保证本程序可编译，但会让 JDK crate 随用户程序变化，计数报出。

use std::collections::BTreeSet;

use crate::ctx::{EmitCtx, ProjectState};
use crate::vtable::param_part;

/// 档案侧需求入账：手写体需求 + 调用链符号引用中属主未声明的实例方法
pub(super) fn seed_requests(ctx: &EmitCtx<'_>, state: &mut ProjectState) {
    // 手写体的继承成员需求与生成方法体登记的同一账本（手写文件整体编译，与 fn 可达性无关）
    for (recv, name, desc) in &ctx.input.hw_inherited {
        state.inherited_requests.insert((recv.clone(), name.clone(), param_part(desc).to_string()));
    }
    for (c, n, d) in &ctx.input.visited {
        if ctx.is_user(c) || n.starts_with('<') {
            continue;
        }
        let Some(ci) = ctx.class(c) else { continue };
        if ci.methods().iter().any(|m| m.name == *n && m.desc == *d) {
            continue;
        }
        if !inherited_instance(ctx, ci, n, d) {
            continue;
        }
        state.inherited_requests.insert((c.clone(), n.clone(), param_part(d).to_string()));
    }
}

/// 祖先（超类链或超接口）声明了同名同描述符的实例方法
fn inherited_instance(ctx: &EmitCtx<'_>, ci: &ty::ClassInfo, name: &str, desc: &str) -> bool {
    let reg = ctx.ty.reg;
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut stack: Vec<&str> = vec![ci.super_class()];
    stack.extend(ci.interfaces().iter().map(String::as_str));
    while let Some(c) = stack.pop() {
        if c.is_empty() || !seen.insert(c) {
            continue;
        }
        let Some(x) = reg.get(c) else { continue };
        if let Some(m) = x.methods().iter().find(|m| m.name == name && m.desc == desc) {
            return !m.is_static();
        }
        stack.push(x.super_class());
        stack.extend(x.interfaces().iter().map(String::as_str));
    }
    false
}

/// 用户类的账本增量：非用户接收者的需求摘出，留待 [`check_leaks`] 对照档案侧
pub(super) fn split_user_delta(ctx: &EmitCtx<'_>, delta: &mut ProjectState, held: &mut Vec<(String, String, String)>) {
    let reqs = std::mem::take(&mut delta.inherited_requests);
    for r in reqs {
        if ctx.is_user(&r.0) {
            delta.inherited_requests.insert(r);
        } else {
            held.push(r);
        }
    }
}

/// 用户方法体对非用户接收者的需求：已被档案侧覆盖者丢弃；未覆盖者入账并报 `[archive-leak]`
pub(super) fn check_leaks(state: &mut ProjectState, held: Vec<(String, String, String)>) {
    let leaks: Vec<_> = held.into_iter().filter(|r| !state.inherited_requests.contains(r)).collect();
    if leaks.is_empty() {
        return;
    }
    let sample: Vec<String> = leaks.iter().take(5).map(|(r, n, d)| format!("{r}.{n}{d}")).collect();
    eprintln!("[archive-leak] 用户方法体对非用户接收者的继承成员需求未被档案侧覆盖：{} 项（{}）", leaks.len(), sample.join("；"));
    state.inherited_requests.extend(leaks);
}

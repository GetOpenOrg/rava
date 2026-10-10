//! 引擎：按调用点派发集求值（计划 2026-10-05-boot-image-evaluator §5.9.7 续，日志链缺口 ③）。
//!
//! 常量实参求值（`consteval.rs`）只求唯一目标的调用；虚 / 接口调用没有唯一目标时结果未知。守卫形态的布尔虚调用
//! （`logger.isLoggable(DEBUG)`：非接收者实参含常量、返回 boolean）的结果往往由接收者实际可达的实现决定——
//! 闭包里该调用点派发到的目标集合（`edge` 接上的调用边）。分派求值按调用点派发集逐个目标做常量实参求值并汇合：
//! - 派发集按调用方成员与偏移汇合全部上下文（`vdisp`，只增）；含非字节码目标、枢纽或成员名 / 描述符与调用不符
//!   （lambda 实现等转接）时不求值；目标数超过上限不求值；
//! - 嵌套求值里的调用同样按此求值：唯一目标求其方法体，虚 / 接口调用按嵌套方法该偏移的派发集；
//!   实参（含接收者）全非常量的多目标调用只取各目标的返回常量格，不展开方法体；一次求值内同目标同绑定实参的结果缓存；
//! - 嵌套求值读非 final 字段时取字段值集（初值 ∪ 可达写入，`fvals`），读者登记为发起的方法；
//! - 派发集尚无记录（调用边还没接上）：定论阶段之前按不返回乐观答复（同 `noreturn.rs`），查询过的调用点在派发集
//!   增长时令发起方法失效重算（`Dep::Disp`）；
//! - 各目标结果：全部不返回 → 不返回；全部同一常量 → 该常量；否则未知。
//!
//! 答复只随派发集、字段值集、返回常量格单调上升（不返回 → 常量 → 未知），与处理次序无关；求值不记忆（每次重算），
//! 深度、指令数与一次发起的求值次数有上限。

use super::consteval::{bindable, exportable, is_const};
use super::memo::Inputs;
use super::*;
use classfile::op::{INVOKEINTERFACE, INVOKEVIRTUAL};

/// 嵌套深度上限
const MAX_DEPTH: u32 = 6;
/// 一次发起的求值次数上限
const BUDGET: u32 = 64;
/// 一个调用点的派发目标数上限
const MAX_TARGETS: usize = 4;
/// 被求值方法的指令数上限
const MAX_INSNS: usize = 256;

/// 调用点派发集
#[derive(Default)]
pub(super) struct VDisp {
    /// 字节码目标的成员
    targets: BTreeSet<MemberRef>,
    /// 有非字节码目标或接入枢纽
    opaque: bool,
}

/// 各目标结果汇合：不返回为单位元，未知吸收，常量相同才保留
fn join(a: Ret, b: Ret) -> Ret {
    match (a, b) {
        (Ret::Never, x) | (x, Ret::Never) => x,
        (Ret::Value(x), Ret::Value(y)) if x == y => Ret::Value(x),
        _ => Ret::Unknown,
    }
}

fn virtual_call(opcode: u8) -> bool {
    matches!(opcode, INVOKEVIRTUAL | INVOKEINTERFACE)
}

/// 守卫形态：虚 / 接口调用、返回 boolean、非接收者实参含常量
fn guard_shape(opcode: u8, m: &MemberRef, args: &[V]) -> bool {
    virtual_call(opcode) && m.desc.ends_with(")Z") && args.iter().skip(1).any(is_const)
}

impl Ctx<'_> {
    fn dv_note(&self, f: impl FnOnce() -> String) {
        if let Some(t) = self.dv_trace.borrow_mut().as_mut() {
            t.push(format!("{}{}", "  ".repeat(self.dv_depth.get() as usize), f()));
        }
    }

    /// 诊断：方法节点 me（成员 key）偏移 off 处的虚 / 接口调用按派发集求值的轨迹（不检查守卫形态）
    pub(super) fn deval_diag(&self, me: usize, key: &MemberRef, opcode: u8, off: u32, m: &MemberRef, args: &[V]) -> String {
        if !virtual_call(opcode) || self.dv_top.get().is_some() {
            return String::new();
        }
        *self.dv_trace.borrow_mut() = Some(Vec::new());
        let r = self.deval_top(me, key, off, m, args);
        let t = self.dv_trace.borrow_mut().take().unwrap_or_default();
        let r = match r {
            Ret::Never => "Never".to_string(),
            Ret::Value(v) => format!("{v:?}"),
            Ret::Unknown => "Unknown".to_string(),
        };
        format!(" deval {r} [{}]", t.join("; "))
    }

    /// 发起一次分派求值（发起方法节点 me）
    fn deval_top(&self, me: usize, key: &MemberRef, off: u32, m: &MemberRef, args: &[V]) -> Ret {
        self.dv_top.set(Some(me));
        self.dv_budget.set(BUDGET);
        let r = self.deval_site(key, off, m, args);
        self.dv_top.set(None);
        self.dv_cache.borrow_mut().clear();
        r
    }

    /// 方法体分析（方法节点 me、成员 key）中无唯一目标的守卫形态调用：按调用点派发集求值；求不出为 None
    pub(super) fn deval_guard(&self, me: usize, key: &MemberRef, opcode: u8, off: u32, m: &MemberRef, args: &[V]) -> Option<Ret> {
        if !guard_shape(opcode, m, args) || self.dv_top.get().is_some() {
            return None;
        }
        let r = self.deval_top(me, key, off, m, args);
        match r {
            Ret::Unknown => None,
            r => Some(r),
        }
    }

    /// 分派求值的嵌套分析（成员 key）中的调用：唯一目标 t 求其方法体，否则虚 / 接口调用按调用点派发集
    pub(super) fn deval_invoke(&self, key: &MemberRef, opcode: u8, off: u32, m: &MemberRef, args: &[V], t: Option<&MemberRef>) -> Ret {
        match t {
            Some(t) => self.deval_call(t, args, true),
            None if virtual_call(opcode) => self.deval_site(key, off, m, args),
            None => Ret::Unknown,
        }
    }

    /// 成员 key 偏移 off 处的调用（被调成员 m）按派发集求值
    fn deval_site(&self, key: &MemberRef, off: u32, m: &MemberRef, args: &[V]) -> Ret {
        let Some(top) = self.dv_top.get() else { return Ret::Unknown };
        self.dep(top, Dep::Disp(key.clone(), off));
        self.dv_note(|| match self.vdisp.borrow().get(&(key.clone(), off)) {
            None => format!("{key}@{off} 无派发集"),
            Some(d) => format!("{key}@{off} opaque={} {:?}", d.opaque, d.targets.iter().map(|t| t.to_string()).collect::<Vec<_>>()),
        });
        let ts: Option<Vec<MemberRef>> = match self.vdisp.borrow().get(&(key.clone(), off)) {
            None => None,
            Some(d) if d.opaque || d.targets.len() > MAX_TARGETS || d.targets.iter().any(|t| t.name != m.name || t.desc != m.desc) => return Ret::Unknown,
            Some(d) => Some(d.targets.iter().cloned().collect()),
        };
        let Some(ts) = ts else {
            // 调用边尚未接上：定论阶段之前按不返回乐观答复
            if self.noreturn.borrow().bottom_never() {
                self.dep(top, Dep::Never);
                return Ret::Never;
            }
            return Ret::Unknown;
        };
        // 实参（含接收者）全非常量的多目标调用只取各目标的返回常量格：方法体求值只在常量实参能带来精度时展开
        // （`wrapped()` 之类取包装对象的调用不逐层展开，预算留给守卫链）
        let deep = args.iter().any(is_const);
        let mut acc = Ret::Never;
        for t in &ts {
            acc = join(acc, self.deval_call(t, args, deep));
            if matches!(acc, Ret::Unknown) {
                break;
            }
        }
        acc
    }

    /// 字节码方法 t 在实参 args（含接收者）上的结果；deep = 返回常量格之外展开方法体求值
    fn deval_call(&self, t: &MemberRef, args: &[V], deep: bool) -> Ret {
        let Some(top) = self.dv_top.get() else { return Ret::Unknown };
        // 返回常量格已是常量（各上下文汇合）：不展开时即为本次结果；展开时只作方法体求不出时的退路——
        // 汇合格在不动点途中只是部分上下文之并，先取它会得出与本次实参下精确值不可比的暂时答复
        // （见 `facts/oracle.rs` 同名口径），据此连上的边不可撤回
        self.dep(top, Dep::Ret(t.clone()));
        let rc = match self.rvals.borrow().get(t) {
            Some(PV::Const(v)) if exportable(v) => Some(v.clone()),
            _ => None,
        };
        if !deep {
            if let Some(v) = rc {
                self.dv_note(|| format!("{t} rvals {v:?}"));
                return Ret::Value(v);
            }
            // 返回常量格缺席（目标尚未分析或尚无返回）：同按成员的返回常量答复（`facts/oracle.rs`），按不返回乐观答复；
            // 过早答「未知」会让守卫之后的调用边先接上，而调用边只增不撤
            if !self.rvals.borrow().contains_key(t) && self.noreturn.borrow().answer_never(t) {
                self.dep(top, Dep::Never);
                self.dv_note(|| format!("{t} 尚无返回"));
                return Ret::Never;
            }
            self.dv_note(|| format!("{t} 只取返回常量格"));
            return Ret::Unknown;
        }
        let bound: Vec<Option<V>> = args.iter().map(|a| bindable(a).then(|| a.stripped())).collect();
        let ck = format!("{t}|{bound:?}");
        let cached = self.dv_cache.borrow().get(&ck).cloned();
        let r = match cached {
            Some(None) => Ret::Unknown,
            Some(Some(None)) => Ret::Never,
            Some(Some(Some(v))) => Ret::Value(v),
            None => {
                let r = self.deval_body(t, bound);
                let c = match &r {
                    Ret::Unknown => None,
                    Ret::Never => Some(None),
                    Ret::Value(v) => Some(Some(v.clone())),
                };
                self.dv_cache.borrow_mut().insert(ck, c);
                r
            }
        };
        match (r, rc) {
            (Ret::Unknown, Some(v)) => {
                self.dv_note(|| format!("{t} rvals {v:?}"));
                Ret::Value(v)
            }
            (r, _) => r,
        }
    }

    /// 方法体求值（绑定实参 bound）
    fn deval_body(&self, t: &MemberRef, bound: Vec<Option<V>>) -> Ret {
        let Some(top) = self.dv_top.get() else { return Ret::Unknown };
        let (b, d) = (self.dv_budget.get(), self.dv_depth.get());
        if b == 0 || d >= MAX_DEPTH {
            self.dv_note(|| format!("{t} 预算 / 深度截断"));
            return Ret::Unknown;
        }
        self.dv_budget.set(b - 1);
        let Some(cf) = self.h.class(&t.owner) else { return Ret::Unknown };
        let Some(meth) = cf.method(&t.name, &t.desc) else { return Ret::Unknown };
        let Some(code) = meth.code.as_ref().filter(|c| c.insns.len() <= MAX_INSNS) else {
            self.dv_note(|| format!("{t} 无字节码或超长"));
            return Ret::Unknown;
        };
        let Some(frame) = self.memo_enter(format!("deval:{t}|{bound:?}"), false) else {
            self.dv_note(|| format!("{t} 重入"));
            return Ret::Unknown;
        };
        self.dv_depth.set(d + 1);
        let live = |_: &str| true;
        let facts = Facts { ctx: self, live: &live, m: None, params: bound.clone(), mirrors: vec![], level: None, objs: Default::default(), callers: None, caller_sites: Default::default(), sites: Rc::from([]), key: Some(t.clone()), dv: true };
        let a = self.aux_analyze(&t.owner, &t.desc, meth.is_static(), code, &facts);
        self.dv_depth.set(d);
        // 不记忆：输入并入外层求值，最外层登记给发起方法
        let (_, inp) = self.memo_leave(frame);
        self.memo_use((d == 0).then_some(top), &Inputs { id: 0, ..inp });
        self.dv_note(|| format!("{t} {bound:?} conservative={} rets {:?}", a.conservative, a.events.iter().filter(|(_, e)| matches!(e, Event::Return(_))).collect::<Vec<_>>()));
        if a.conservative {
            return Ret::Unknown;
        }
        let mut r: Option<PV> = None;
        for (_, e) in &a.events {
            if let Event::Return(v) = e {
                r = Some(PV::join(r.as_ref(), &PV::of(v)));
            }
        }
        match r {
            None => Ret::Never,
            Some(PV::Const(v)) if exportable(&v) => Ret::Value(v),
            _ => Ret::Unknown,
        }
    }
}

impl Facts<'_, '_> {
    /// 非 final 字段读的读者：分派求值中是发起的方法节点（取字段值集并登记依赖），否则为被分析方法
    pub(super) fn reader(&self) -> Option<usize> {
        if self.dv { self.ctx.dv_top.get() } else { self.m }
    }
}

impl<'a> Engine<'a> {
    /// 调用点 (m, off) 接上目标节点 t（None = 枢纽）：并入派发集，增长时令查询过它的方法失效
    pub(super) fn vdisp_note(&mut self, m: usize, off: u32, t: Option<usize>) {
        let key = self.methods[m].key.clone();
        let tk = t.filter(|&t| self.methods[t].kind == Kind::Bytecode).map(|t| self.methods[t].key.clone());
        let grown = {
            let mut vd = self.ctx.vdisp.borrow_mut();
            let e = vd.entry((key.clone(), off)).or_default();
            match tk {
                Some(k) => e.targets.insert(k),
                None => !std::mem::replace(&mut e.opaque, true),
            }
        };
        if grown {
            let ws = self.ctx.vwatch.borrow_mut().remove(&(key, off));
            if ws.is_some() {
                self.invalidate_all(ws, Why::RetConst);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_rules() {
        assert!(matches!(join(Ret::Never, Ret::Value(V::Int(0))), Ret::Value(V::Int(0))));
        assert!(matches!(join(Ret::Value(V::Int(0)), Ret::Value(V::Int(0))), Ret::Value(V::Int(0))));
        assert!(matches!(join(Ret::Value(V::Int(0)), Ret::Value(V::Int(1))), Ret::Unknown));
        assert!(matches!(join(Ret::Unknown, Ret::Never), Ret::Unknown));
        assert!(matches!(join(Ret::Never, Ret::Never), Ret::Never));
    }

    #[test]
    fn guard_shape_needs_const_arg_and_boolean() {
        let m = |d: &str| MemberRef { owner: "p/L".into(), name: "on".into(), desc: d.into() };
        let recv = V::Top;
        assert!(guard_shape(INVOKEINTERFACE, &m("(I)Z"), &[recv.clone(), V::Int(2)]));
        assert!(!guard_shape(INVOKEINTERFACE, &m("(I)Z"), &[V::Int(2), V::Top]));
        assert!(!guard_shape(INVOKEINTERFACE, &m("(I)I"), &[recv.clone(), V::Int(2)]));
        assert!(!guard_shape(classfile::op::INVOKESTATIC, &m("(I)Z"), &[V::Int(2)]));
    }
}

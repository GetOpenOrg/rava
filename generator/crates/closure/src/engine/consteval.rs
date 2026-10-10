//! 常量实参求值：唯一目标的字节码被调方法，以调用点的常量实参绑定形参做一次辅助分析，
//! 全部返回路径汇成同一常量时，该调用的结果即此常量（被调方法的返回常量格在所有调用点
//! 上汇合为 Top 时仍能按本调用点折叠，如 `parseBoolean("false")`）。
//!
//! 辅助分析只用清单事实、static final 常量、构造摘要与嵌套的常量实参求值（与 `<clinit>`
//! 常量求值同口径）；结果按 (目标, 常量实参) 记忆。求值中读过的字段随结果登记给外层方法：
//! 字段转为不折叠 / 系统属性转为不稳定时记忆作废、外层方法失效重算。

use super::memo::Inputs;
use crate::absint::ints;
use super::*;

/// 嵌套求值深度上限
const MAX_DEPTH: u32 = 3;
/// 一条嵌套求值链上穿过分派转发方法（不计深度）的次数上限
const MAX_PASS: u32 = 8;

/// 嵌套求值的位置：计深度的层数、穿过分派转发方法的次数（两者都随记忆键，见 `memo.rs`）
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Debug)]
pub(super) struct EvalDepth {
    pub(super) nest: u32,
    pub(super) pass: u32,
}
/// 被求值方法的指令数上限
const MAX_INSNS: usize = 256;

/// 可作为求值输入的常量实参（类字面量：所指类已知的 Class 对象，如 `X.class.desiredAssertionStatus()` 的接收者；
/// 引导映像对象：身份与 final 字段在构建期确定，如映像 Module 上的 `getClassLoader()`）
pub(super) fn is_const(v: &V) -> bool {
    matches!(v, V::Int(_) | V::Long(_) | V::Null | V::Str(..) | V::Class(..)) || image_id(v).is_some()
}

/// 引导映像对象的下标与已知 final 字段数（同一对象带不带 finals 的两种标签分开记忆）
fn image_id(v: &V) -> Option<(u32, usize)> {
    match v.obj().map(|o| &**o) {
        Some(crate::absint::Obj::Image(id, fs)) => Some((*id, fs.len())),
        _ => None,
    }
}

/// 随常量实参一并绑定的实参：系统属性表对象（被调方法里对它的读取按键折叠，如属性读取的包装方法）、
/// 构造完成标签的对象（被调方法里按标签读 final 字段、按标签的类选虚调用目标）
pub(super) fn bindable(v: &V) -> bool {
    is_const(v) || is_sysprops_tag(v) || fields_tag(v)
}

/// 带非空构造完成标签（`Obj::Fields`）的引用
fn fields_tag(v: &V) -> bool {
    matches!(v.obj().map(|o| &**o), Some(crate::absint::Obj::Fields(fs)) if !fs.is_empty())
}

fn is_sysprops_tag(v: &V) -> bool {
    v.obj().is_some_and(|o| **o == crate::absint::Obj::SysProps)
}

/// 可作为求值结果导出的常量
pub(super) fn exportable(v: &V) -> bool {
    matches!(v, V::Int(_) | V::Long(_) | V::Null | V::Str(..)) || image_id(v).is_some()
}

/// 求值记忆：结果与求值的输入（见 `memo.rs`）
pub(super) type CEval = (Option<V>, Inputs);

/// 记忆键里的常量实参（字符串取字面量序号）
#[derive(Clone, PartialEq, Eq, Hash)]
pub(super) enum CArg {
    Int(i32),
    Long(i64),
    Null,
    Str(u32),
    SysProps,
    /// 类字面量（所指类名取字面量序号）
    Class(u32),
    /// 引导映像对象（下标, 已知 final 字段数）
    Image(u32, usize),
    /// 构造完成标签（标签的规范文本）
    Fields(String),
}

/// 记忆键：目标、各实参（非常量 = None）、起始深度
pub(super) type CKey = (MemberRef, Vec<Option<CArg>>, EvalDepth);

fn carg(v: &V) -> Option<CArg> {
    match v {
        V::Int(i) => Some(CArg::Int(*i)),
        V::Long(l) => Some(CArg::Long(*l)),
        V::Null => Some(CArg::Null),
        V::Str(s, _) => Some(CArg::Str(crate::absint::lit_id(s))),
        v if is_sysprops_tag(v) => Some(CArg::SysProps),
        V::Class(c, _) => Some(CArg::Class(crate::absint::lit_id(c))),
        v if fields_tag(v) => v.obj().map(|o| CArg::Fields(format!("{o:?}"))),
        v => image_id(v).map(|(id, n)| CArg::Image(id, n)),
    }
}

impl Ctx<'_> {
    /// 调用 `t`（唯一字节码目标）在实参 `args`（含接收者）上的常量结果；me = 外层被分析的方法
    pub(super) fn const_eval(&self, me: Option<usize>, t: &MemberRef, args: &[V]) -> Option<V> {
        if args.iter().any(|a| matches!(a, V::Ints(_))) {
            return self.const_eval_sets(me, t, args);
        }
        // 无常量实参时只求可能返回属性表对象的方法（如返回持有字段的包装方法）
        let ret = t.desc.rsplit_once(')').map_or("", |x| x.1);
        if !args.iter().any(|a| is_const(a) || fields_tag(a)) && !self.man.sysprops.holder_type(ret) {
            return None;
        }
        // 记忆键带起始深度：嵌套求值的深度上限截断只取决于它
        let memo_key: CKey = (t.clone(), args.iter().map(carg).collect(), self.ceval_depth.get());
        let hit = self.cevals.borrow().get(&memo_key).cloned();
        self.stats.borrow_mut().ceval[usize::from(hit.is_none())] += 1;
        let (v, inp) = match hit {
            Some(e) => e,
            None => {
                let bound: Vec<Option<V>> = args.iter().map(|a| bindable(a).then(|| a.stripped())).collect();
                let key = format!("{t}|{bound:?}");
                let (e, clean) = self.const_eval_fresh(&key, t, bound)?;
                if clean {
                    self.cevals.borrow_mut().insert(memo_key, e.clone());
                }
                e
            }
        };
        // 外层是方法体分析：登记输入依赖；嵌套于另一次辅助分析：输入并入外层记录
        self.memo_use(me, &inp);
        if v.as_ref().is_some_and(is_sysprops_tag) {
            self.note_props(me);
        }
        v
    }

    /// 含整数集实参：逐个取值组合求值（组合数 ≤ `ints::MAX`），结果全部可知时取并
    fn const_eval_sets(&self, me: Option<usize>, t: &MemberRef, args: &[V]) -> Option<V> {
        let mut combos: Vec<Vec<V>> = vec![Vec::with_capacity(args.len())];
        for a in args {
            let vals = match a {
                V::Ints(xs) => xs.iter().map(|x| V::Int(*x)).collect(),
                _ => vec![a.clone()],
            };
            if combos.len() * vals.len() > ints::MAX {
                return None;
            }
            combos = combos.into_iter().flat_map(|c| vals.iter().map(move |v| [c.clone(), vec![v.clone()]].concat())).collect();
        }
        let mut acc: Option<V> = None;
        for c in &combos {
            let v = self.const_eval(me, t, c)?;
            acc = Some(match acc {
                None => v,
                Some(p) if p == v => p,
                Some(p) => ints::union(&p, &v)?,
            });
        }
        acc
    }

    /// 实际求值与是否可记忆（见 `memo.rs`）；None = 超出深度 / 递归中
    fn const_eval_fresh(&self, key: &str, t: &MemberRef, bound: Vec<Option<V>>) -> Option<(CEval, bool)> {
        let empty = || Some(((None, Inputs::default()), true));
        let Some(cf) = self.h.class(&t.owner) else { return empty() };
        let Some(meth) = cf.method(&t.name, &t.desc) else { return empty() };
        let Some(code) = meth.code.as_ref().filter(|c| c.insns.len() <= MAX_INSNS) else { return empty() };
        // 穿过分派转发方法（`forward.rs`：静态方法把引用形参交给虚 / 接口分派）且转发的是构造完成标签对象时
        // 不计深度：转发方法本身不产生值，深度留给被转发的动作体（如特权块 `doPrivileged → executePrivileged
        // → action.run()`）。穿过次数单独计数、有上限：转发方法可以成环（自递归或互相递归的静态方法），
        // 环上其余实参逐层变化（如递减的整数）时每层键都不同，`memo_enter` 的同键截断拦不住，
        // 不设上限则嵌套无界（TestDomResultNode 转译期栈溢出）。超过上限的转发按普通调用计深度
        let cur = self.ceval_depth.get();
        let pass = cur.pass < MAX_PASS && meth.is_static() && {
            let mask = self.dispatch_slots(t);
            bound.iter().enumerate().any(|(i, b)| i < 64 && mask & (1 << i) != 0 && b.as_ref().is_some_and(fields_tag))
        };
        if !pass && cur.nest >= MAX_DEPTH {
            return None;
        }
        let frame = self.memo_enter(format!("ceval:{key}"), false)?;
        self.stats.borrow_mut().ceval[2] += 1;
        self.ceval_depth.set(if pass { EvalDepth { pass: cur.pass + 1, ..cur } } else { EvalDepth { nest: cur.nest + 1, ..cur } });
        let live = |_: &str| true;
        let a = self.aux_analyze(&t.owner, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params: bound, mirrors: vec![], level: None, objs: Default::default(), callers: None, caller_sites: Default::default(), sites: Rc::from([]), key: None, dv: false });
        let (clean, inp) = self.memo_leave(frame);
        let mut r: Option<PV> = None;
        if !a.conservative {
            for (_, e) in &a.events {
                if let Event::Return(v) = e {
                    r = Some(PV::join(r.as_ref(), &PV::of(v)));
                }
            }
        }
        let v = match r {
            Some(PV::Const(v)) if exportable(&v) => Some(v),
            Some(PV::Const(v)) if is_sysprops_tag(&v) => Some(v.stripped()),
            _ => None,
        };
        Some(((v, inp), clean))
    }

    /// 诊断：按实参 args 重做一次求值所用的辅助分析（不记忆），列出保守标志与返回 / 常量 / 调用事件
    pub(super) fn ceval_trace(&self, t: &MemberRef, args: &[V]) -> String {
        let Some(cf) = self.h.class(&t.owner) else { return "无类".into() };
        let Some(meth) = cf.method(&t.name, &t.desc) else { return "无方法".into() };
        let Some(code) = meth.code.as_ref() else { return "无代码".into() };
        let bound: Vec<Option<V>> = args.iter().map(|a| bindable(a).then(|| a.stripped())).collect();
        let live = |_: &str| true;
        let a = self.aux_analyze(&t.owner, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params: bound.clone(), mirrors: vec![], level: None, objs: Default::default(), callers: None, caller_sites: Default::default(), sites: Rc::from([]), key: None, dv: false });
        let evs: Vec<String> = a.events.iter().filter_map(|(o, e)| match e {
            Event::Return(v) => Some(format!("@{o} ret {v:?}")),
            Event::Const { value, .. } => Some(format!("@{o}={value:?}")),
            Event::Invoke { mref, args, .. } => Some(format!("@{o} {}{args:?}", mref.name)),
            Event::Field { mref, value, .. } => Some(format!("@{o} {} = {value:?}", mref.name)),
            _ => None,
        }).collect();
        format!("bound {bound:?} conservative {} insns {} depth {:?} events {}", a.conservative, code.insns.len(), self.ceval_depth.get(), evs.join(" "))
    }

    /// 字段 f 转为不折叠：读过它的求值记忆作废（其余记忆的输入未变，重算结果相同）；返回取用过作废记忆的方法
    pub(super) fn ceval_drop_field(&self, f: &MemberRef) -> BTreeSet<usize> {
        self.ceval_drop(|inp| inp.reads.contains(f))
    }

    /// 名为 name 的字段全部转为不折叠：读过同名字段的求值记忆作废
    pub(super) fn ceval_drop_name(&self, name: &str) -> BTreeSet<usize> {
        self.ceval_drop(|inp| inp.reads.iter().any(|r| r.name == name))
    }

    /// 字段转为不折叠：输入含该字段的记忆全部作废（`<clinit>` 常量、构造器对象、属性摘要、只读判定与常量实参求值），
    /// 返回取用者。只作废求值记忆而留下其余记忆时，旧答复要等到下一次不折叠集合增长（`sysprops.rs`）才刷新，
    /// 终态取决于两者的先后（D1）
    pub(super) fn ceval_drop(&self, mut stale: impl FnMut(&Inputs) -> bool) -> BTreeSet<usize> {
        let mut ids = Vec::new();
        let mut keep = |inp: &Inputs| {
            let s = stale(inp);
            if s {
                ids.push(inp.id);
            }
            !s
        };
        self.cevals.borrow_mut().retain(|_, (_, inp)| keep(inp));
        self.consts.borrow_mut().retain(|_, (_, inp)| keep(inp));
        self.objs.borrow_mut().retain(|_, (_, inp)| keep(inp));
        self.psums.borrow_mut().retain(|_, (_, inp)| keep(inp));
        self.preadonly.borrow_mut().retain(|_, (_, inp)| keep(inp));
        let n = self.cinits.borrow().len();
        self.cinits.borrow_mut().retain(|_, c| keep(&c.inp));
        if self.cinits.borrow().len() != n {
            self.cinit_drop.set(true);
        }
        self.memo_consumers(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn const_inputs() {
        assert!(is_const(&V::lit(Rc::from("x"))));
        assert!(is_const(&V::Null));
        assert!(!is_const(&V::Top));
        assert!(is_const(&V::Class(Rc::from("A"), 0)));
        assert!(!exportable(&V::Class(Rc::from("A"), 0)));
    }

    #[test]
    fn string_ops_on_constants() {
        use super::super::sysprops::string_op;
        use crate::manifest::StrOp;
        let s = |x: &str| V::lit(Rc::from(x));
        // parseBoolean("false") = "true".equalsIgnoreCase("false")
        assert_eq!(string_op(StrOp::EqualsIgnoreCase, &[s("true"), s("false")]), Some(V::Int(0)));
        assert_eq!(string_op(StrOp::EqualsIgnoreCase, &[s("true"), s("TRUE")]), Some(V::Int(1)));
        assert_eq!(string_op(StrOp::EqualsIgnoreCase, &[s("true"), V::Null]), Some(V::Int(0)));
        assert_eq!(string_op(StrOp::EqualsIgnoreCase, &[s("é"), s("É")]), None);
        assert_eq!(string_op(StrOp::EqualsIgnoreCase, &[s("true"), V::Top]), None);
        assert_eq!(string_op(StrOp::Length, &[s("a😀")]), Some(V::Int(3)));
        assert_eq!(string_op(StrOp::IsEmpty, &[s("")]), Some(V::Int(1)));
        // 与 Java String.hashCode 一致（字符串 switch 的键）："file".hashCode() = 3143036
        assert_eq!(string_op(StrOp::HashCode, &[s("file")]), Some(V::Int(3143036)));
        assert_eq!(string_op(StrOp::HashCode, &[s("")]), Some(V::Int(0)));
        // toLowerCase：ASCII 且不含 'I' 时与 locale 无关可折叠，其余（土耳其语 I 等）不折叠
        assert_eq!(string_op(StrOp::ToLowerCase, &[s("JAR"), V::Top]), Some(s("jar")));
        assert_eq!(string_op(StrOp::ToLowerCase, &[s("File")]), Some(s("file")));
        assert_eq!(string_op(StrOp::ToLowerCase, &[s("FTP"), V::Null]), Some(s("ftp")));
        assert_eq!(string_op(StrOp::ToLowerCase, &[s("JRTI")]), None);
        assert_eq!(string_op(StrOp::ToLowerCase, &[s("é")]), None);
        assert_eq!(string_op(StrOp::HashCode, &[s("sun.net.www.protocol.")]), Some(V::Int("sun.net.www.protocol.".encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32)))));
        assert_eq!(string_op(StrOp::CharAt, &[s("a😀"), V::Int(1)]), Some(V::Int(0xd83d)));
        assert_eq!(string_op(StrOp::CharAt, &[s("ab"), V::Int(2)]), None);
        assert_eq!(string_op(StrOp::CharAt, &[s("ab"), V::Int(-1)]), None);
        assert_eq!(string_op(StrOp::CharToLowerCase, &[V::Int('F' as i32)]), Some(V::Int('f' as i32)));
        assert_eq!(string_op(StrOp::CharToLowerCase, &[V::Int(0xc9)]), None);
    }
}

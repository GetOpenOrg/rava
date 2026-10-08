//! 常量查询：系统属性读取折叠（`[facts.system_properties]`）与派生的调用结果（值相等判定）。
//!
//! 折叠规则：
//! - 读取点 = 清单读取锚点（接收者须为属性表对象时按对象标签判定），或其摘要形态的字节码方法
//!   （返回值恰为某读取点的结果、键 / 缺省值来自本方法形参或常量，如 `System.getProperty`）；
//! - 键为字符串常量时：表中键 → 声明值（`values`）；存在但取值启动期才定（`dynamic`）→ 不折叠；
//!   表外键 → 缺省值（无缺省实参即 null）；
//! - 运行期可能被改写的键不折叠：活代码里属性表对象上的按键改写入口（`writers`）使该键不折叠；
//!   改写键推不出、属性表对象逃逸（写入字段 / 数组、作实参 / indy 实参、调用非读写 / 只读查询入口）、
//!   字节码写持有字段、手写体调用改写入口 → 全部不折叠；属性表对象作唯一字节码目标的实参、
//!   且该形参在被调方法里只作只读查询 / 读取入口的接收者或再传给同类只读形参 → 不算逃逸；
//! - 作返回值不算逃逸，只要每个调用方都拿到带标签的结果（其后的使用由调用方自己的扫描判定）：
//!   调用方拿到标签 ⇔ 字节码调用点、调用目标唯一（取被调方法返回常量）、返回常量仍带属性表标签。
//!   以下任一成立即全部不折叠：返回常量合流后丢了标签；方法有非字节码调用点入口（手写 / 反射 /
//!   方法句柄 / VM 根，或创建点不封闭的 lambda，见 `sysprops_lambda.rs`）；某个目标不唯一（虚派发）的
//!   调用点与它同名同描述符。
//!
//! 不折叠集合只增不减；增长时清掉依赖它的缓存（static final 常量、构造器摘要、读取摘要），
//! 折叠过属性读取 / 字段读取的方法失效重算——单调不动点上的折叠只来自最终仍稳定的键。

use super::sysprops_key::key_src;
use super::*;

/// 缺省值来源
#[derive(Clone, Debug, PartialEq)]
pub(super) enum DefArg {
    None,
    /// 实参序号（含接收者）
    Param(usize),
    Const(V),
}

/// 读取入口的形态：键 / 缺省值的实参序号；`receiver` = 接收者须为属性表对象
#[derive(Clone, Debug, PartialEq)]
pub(super) struct PropSum {
    pub(super) receiver: bool,
    pub(super) key: usize,
    pub(super) default: DefArg,
}

/// 返回属性表对象的方法与调用方可见性（均只增不减，两侧先后到达都能判定）
#[derive(Default)]
pub(super) struct SpRet {
    /// 返回值可能是属性表对象的方法
    methods: BTreeSet<MemberRef>,
    /// 有非字节码调用点入口的方法（方法入口逐次查询，只做成员判定）
    untracked: HashSet<MemberRef>,
    /// 目标不唯一的引用返回调用点（名字, 描述符）
    virt: BTreeSet<(String, String)>,
    /// 经字节码 SAM 调用点进入的实现方法（是否可跟踪由 lambda 创建点判定，见 `sysprops_lambda.rs`）
    lambda_in: HashSet<MemberRef>,
    /// 返回属性表对象的方法节点（lambda 入口晚到时重分析重判）
    nodes: HashMap<MemberRef, BTreeSet<usize>>,
}

/// 运行期可能被改写（不折叠）的键
#[derive(Default)]
pub(super) struct PropUnstable {
    pub(super) all: bool,
    pub(super) keys: BTreeSet<String>,
    /// 首个「全部不折叠」的成因（诊断：summary.sysprops_unstable）
    pub(super) cause: Option<String>,
}

fn is_sysprops(v: &V) -> bool {
    v.obj().is_some_and(|o| **o == Obj::SysProps)
}

fn may_be_sysprops(v: &V) -> bool {
    v.obj().is_some_and(|o| o.may_be_sysprops())
}

/// 字符串纯函数在常量实参上的值（非 ASCII 的大小写比较不求值：UTF-16 逐字符大小写映射不在此复刻）
pub(super) fn string_op(op: crate::manifest::StrOp, args: &[V]) -> Option<V> {
    use crate::manifest::StrOp;
    match (op, args) {
        (StrOp::EqualsIgnoreCase, [V::Str(..), V::Null]) => Some(V::Int(0)),
        (StrOp::EqualsIgnoreCase, [V::Str(a, _), V::Str(b, _)]) if a.is_ascii() && b.is_ascii() => Some(V::Int(a.eq_ignore_ascii_case(b) as i32)),
        (StrOp::Length, [V::Str(a, _)]) => Some(V::Int(a.encode_utf16().count() as i32)),
        (StrOp::IsEmpty, [V::Str(a, _)]) => Some(V::Int(a.is_empty() as i32)),
        (StrOp::HashCode, [V::Str(a, _)]) => Some(V::Int(a.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(i32::from(c))))),
        (StrOp::CharAt, [V::Str(a, _), V::Int(i)]) => usize::try_from(*i).ok().and_then(|i| a.encode_utf16().nth(i)).map(|c| V::Int(i32::from(c))),
        (StrOp::CharToLowerCase, [V::Int(c)]) => u8::try_from(*c).ok().filter(u8::is_ascii).map(|b| V::Int(i32::from(b.to_ascii_lowercase()))),
        (StrOp::ToLowerCase, [V::Str(a, _)] | [V::Str(a, _), _]) => ascii_lower(a).map(|l| V::lit(Rc::from(l.as_str()))),
        (StrOp::IsEmpty, [r]) => r.str_shape()?.is_empty().map(|b| V::Int(b as i32)),
        (StrOp::StartsWith, [r, V::Str(l, _)]) => r.str_shape()?.starts_with(l).map(|b| V::Int(b as i32)),
        (StrOp::EndsWith, [r, V::Str(l, _)]) => r.str_shape()?.ends_with(l).map(|b| V::Int(b as i32)),
        (StrOp::Contains, [r, V::Str(l, _)]) => r.str_shape()?.contains(l).map(|b| V::Int(b as i32)),
        (StrOp::IndexOf | StrOp::LastIndexOf, [r, x, rest @ ..]) if rest.len() <= 1 => index_of(op == StrOp::LastIndexOf, r, x, rest.is_empty()),
        _ => None,
    }
}

/// `indexOf` / `lastIndexOf`：所找字符（子串的任一字符）确定不出现 → -1；接收者为常量且不带起点 → 按 UTF-16 下标求值
fn index_of(last: bool, r: &V, x: &V, whole: bool) -> Option<V> {
    let sh = r.str_shape()?;
    let needle: String = match x {
        V::Int(c) => char::from_u32(u32::try_from(*c).ok()?)?.to_string(),
        V::Str(l, _) => l.to_string(),
        _ => return None,
    };
    if sh.contains(&needle) == Some(false) {
        return Some(V::Int(-1));
    }
    let V::Str(a, _) = r else { return None };
    if !whole {
        return None;
    }
    let at = if last { a.rfind(needle.as_str()) } else { a.find(needle.as_str()) };
    Some(V::Int(at.map_or(-1, |i| a[..i].encode_utf16().count() as i32)))
}

/// 与语言无关的小写（`String.toLowerCase` 在 ASCII 且不含 `I` 的串上各语言结果相同）；其余 None
pub(super) fn ascii_lower(s: &str) -> Option<String> {
    (s.is_ascii() && !s.contains('I')).then(|| s.to_ascii_lowercase())
}

/// 返回值来源上溯到的调用偏移：类目标 checkcast 的结果换成其输入的来源（删除并返回原值的入口返回
/// Object，包装方法转型后返回）；来源不是调用点 / checkcast → None
fn read_sites(a: &Analysis, srcs: &[Src]) -> Option<Vec<u32>> {
    let mut out = Vec::new();
    let mut work: Vec<Src> = srcs.to_vec();
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    while let Some(s) = work.pop() {
        let Src::Site(o) = s else { return None };
        if !seen.insert(o) {
            continue;
        }
        let lo = a.events.partition_point(|x| x.0 < o);
        let cast = a.events[lo..].iter().take_while(|x| x.0 == o).find_map(|(_, x)| match x {
            Event::CheckCast(_, Some(inp)) => Some(inp),
            _ => None,
        });
        match cast {
            Some(inp) => {
                let ss = inp.srcs();
                if ss.is_empty() {
                    return None;
                }
                work.extend(ss.iter().cloned());
            }
            None => out.push(o),
        }
    }
    Some(out)
}

/// 值恰为本方法某形参（无其它来源）
fn param_of(v: &V) -> Option<usize> {
    match &*v.srcs() {
        [Src::Param(i)] if matches!(v, V::Ref { .. }) => Some(*i as usize),
        _ => None,
    }
}

impl Ctx<'_> {
    /// 由调用实参派生的结果：值相等判定、属性表持有方法、属性读取
    pub(super) fn derived_result(&self, me: Option<usize>, opcode: u8, m: &MemberRef, iface: bool, args: &[V], c: &CallInfo) -> Option<Ret> {
        if c.value_eq {
            return match args {
                [V::Str(a, _), V::Str(b, _)] => Some(Ret::Value(V::Int((a == b) as i32))),
                [V::Str(..), V::Null] => Some(Ret::Value(V::Int(0))),
                // 一侧为常量、另一侧形状已知：形状排除该常量时不等（实参 null 同样不等）
                [r, V::Str(l, _)] | [V::Str(l, _), r] => r.str_shape().and_then(|sh| sh.equals(l)).filter(|b| !b).map(|_| Ret::Value(V::Int(0))),
                _ => None,
            };
        }
        if let Some(op) = c.str_op {
            return string_op(op, args).map(Ret::Value);
        }
        if self.man.sysprops.is_empty() {
            return None;
        }
        if c.holder {
            let ret = parse_method(&m.desc).and_then(|md| md.ret).map(|t| t.descriptor())?;
            return Some(Ret::Value(self.sysprops_ref(&ret)));
        }
        // 键为拼接值：只在该方法有登记的候选模式时求（`sysprops_key.rs`）
        let patterned = me.is_some_and(|me| args.iter().any(|a| key_src(a).is_some_and(|o| self.pkeys.borrow().contains_key(&(me, o)))));
        let unseen = me.is_some_and(|me| args.iter().any(|a| key_src(a).is_some_and(|o| !self.pkeys_seen.borrow().contains(&(me, o)))));
        if !patterned && !unseen && !args.iter().any(|a| matches!(a, V::Str(..))) {
            return None;
        }
        let spec = self.read_spec(me, opcode, m, iface, Some(c))?;
        let key = args.get(spec.key)?;
        // 读取点尚未登记候选模式（登记在本次分析之后的调用事件里）：答复 ⊥（读取之后暂不可达），登记后失效重算。
        // 不按值未知答复——未知值并入返回常量格 / 形参常量格后无法撤回，登记先后会让结果依赖处理顺序
        if let (Some(me), Some(o)) = (me, key_src(key)) {
            if !self.pkeys_seen.borrow().contains(&(me, o)) && self.noreturn.borrow().bottom_never() {
                self.dep(me, Dep::Never);
                return Some(Ret::Never);
            }
        }
        match key {
            V::Str(..) => self.prop_read(me, &spec, args),
            _ if patterned => self.prop_read_patterns(me, &spec, args),
            _ => None,
        }
    }

    /// 清单属性读取锚点（成员键）的读取形态
    pub(super) fn reader_spec(&self, k: &str) -> Option<PropSum> {
        self.man.sysprops.reader(k).map(|r| PropSum { receiver: r.receiver, key: r.key, default: r.default.map_or(DefArg::None, DefArg::Param) })
    }

    /// 调用是否属性读取（清单锚点 / 摘要形态的字节码方法）
    /// me = 外层被分析的方法（读取摘要的输入登记给它；辅助分析 / 引擎侧查询为 None）
    pub(super) fn read_spec(&self, me: Option<usize>, opcode: u8, m: &MemberRef, iface: bool, c: Option<&CallInfo>) -> Option<PropSum> {
        let c = match c {
            Some(c) => {
                if c.reader.is_some() {
                    return c.reader.clone();
                }
                c.target.clone()
            }
            None => {
                if let Some(r) = self.reader_spec(&m.to_string()) {
                    return Some(r);
                }
                self.call_info(opcode, m, iface).target.clone()
            }
        }?;
        self.prop_summary(me, &c)
    }

    /// 属性读取的折叠值（键不是常量 / 键不稳定 / 取值启动期才定 → None）
    pub(super) fn prop_read(&self, me: Option<usize>, spec: &PropSum, args: &[V]) -> Option<Ret> {
        if spec.receiver && !args.first().is_some_and(is_sysprops) {
            return None;
        }
        let V::Str(key, _) = args.get(spec.key)? else { return None };
        self.note_props(me);
        {
            let u = self.punstable.borrow();
            if u.all || u.keys.contains(&**key) {
                return None;
            }
        }
        let v = match self.man.sysprops.lookup(key) {
            PropValue::Const(s) => V::lit(Rc::from(s)),
            PropValue::Dynamic => return None,
            PropValue::Absent => match &spec.default {
                DefArg::None => V::Null,
                DefArg::Const(v) => v.clone(),
                DefArg::Param(i) => match args.get(*i)? {
                    v @ (V::Str(..) | V::Null) => v.clone(),
                    _ => return None,
                },
            },
        };
        Some(Ret::Value(v))
    }

    /// 字节码方法的读取摘要：全部返回值恰为某读取点的结果，读取键为本方法形参、缺省值为形参或常量
    /// （与调用点无关的独立分析，按成员缓存）
    fn prop_summary(&self, me: Option<usize>, t: &MemberRef) -> Option<PropSum> {
        let hit = self.psums.borrow().get(t).cloned();
        if let Some((s, inp)) = hit {
            self.memo_use(me, &inp);
            return s;
        }
        if !t.desc.ends_with(';') {
            return None;
        }
        let frame = self.memo_enter(format!("psum:{t}"), true)?;
        let s = self.compute_summary(t);
        let (clean, inp) = self.memo_leave(frame);
        self.memo_use(me, &inp);
        if clean {
            self.psums.borrow_mut().insert(t.clone(), (s.clone(), inp));
        }
        s
    }

    fn compute_summary(&self, t: &MemberRef) -> Option<PropSum> {
        let cf = self.h.class(&t.owner)?;
        let meth = cf.method(&t.name, &t.desc)?;
        let code = meth.code.as_ref()?;
        let n = parse_method(&t.desc)?.params.len() + usize::from(!meth.is_static());
        let live = |_: &str| true;
        let a = self.aux_analyze(&cf.name, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params: vec![None; n], mirrors: vec![], level: None, objs: Default::default() });
        if a.conservative {
            return None;
        }
        let mut out: Option<PropSum> = None;
        for (_, e) in &a.events {
            let Event::Return(v) = e else { continue };
            let V::Ref { .. } = v else { return None };
            let srcs = v.srcs();
            if srcs.is_empty() {
                return None;
            }
            for o in read_sites(&a, &srcs)? {
                let lo = a.events.partition_point(|x| x.0 < o);
                let inv = a.events[lo..].iter().take_while(|x| x.0 == o).find_map(|(_, x)| match x {
                    Event::Invoke { opcode, mref, iface, args } => Some((*opcode, mref, *iface, args)),
                    _ => None,
                });
                let (opc, mref, iface, args) = inv?;
                let inner = self.read_spec(None, opc, mref, iface, None)?;
                let sum = Self::compose(&inner, args)?;
                if out.as_ref().is_some_and(|x| *x != sum) {
                    return None;
                }
                out = Some(sum);
            }
        }
        out
    }

    /// 摘要方法里一个读取点（形态 inner、实参 args）换算成摘要方法自身形参上的读取形态
    fn compose(inner: &PropSum, args: &[V]) -> Option<PropSum> {
        if inner.receiver && !args.first().is_some_and(is_sysprops) {
            return None;
        }
        let key = param_of(args.get(inner.key)?)?;
        let default = match &inner.default {
            DefArg::None => DefArg::None,
            DefArg::Const(v) => DefArg::Const(v.clone()),
            DefArg::Param(i) => match args.get(*i)? {
                v @ (V::Str(..) | V::Null) => DefArg::Const(v.clone()),
                v => DefArg::Param(param_of(v)?),
            },
        };
        Some(PropSum { receiver: false, key, default })
    }
}

impl Ctx<'_> {
    /// 属性表对象作调用的第 i 个实参不构成逃逸：只读查询 / 读取入口的接收者，
    /// 或唯一字节码目标上的只读形参（该形参在被调方法里只流向同类不逃逸的用途）
    pub(super) fn sysprops_arg_ok(&self, me: Option<usize>, opcode: u8, mref: &MemberRef, iface: bool, i: usize) -> bool {
        let k = mref.to_string();
        if self.man.sysprops.writer(&k).is_some() {
            return false;
        }
        if i == 0 && (self.man.sysprops.is_query(&k) || self.read_spec(None, opcode, mref, iface, None).is_some_and(|s| s.receiver)) {
            return true;
        }
        let static_call = opcode == classfile::op::INVOKESTATIC;
        if i == 0 && !static_call {
            return false;
        }
        match self.call_info(opcode, mref, iface).target.clone() {
            Some(t) => self.sysprops_readonly(me, &t, i),
            None => false,
        }
    }

    /// 字节码方法第 i 个形参（含接收者序号）为属性表对象时，方法体内不改写、不逃逸（按成员缓存；递归按逃逸）
    fn sysprops_readonly(&self, me: Option<usize>, t: &MemberRef, i: usize) -> bool {
        let ck = (t.clone(), i);
        let hit = self.preadonly.borrow().get(&ck).cloned();
        if let Some((r, inp)) = hit {
            self.memo_use(me, &inp);
            return r;
        }
        let Some(frame) = self.memo_enter(format!("pro:{t}#{i}"), true) else { return false };
        let r = self.compute_readonly(t, i);
        let (clean, inp) = self.memo_leave(frame);
        self.memo_use(me, &inp);
        if clean {
            self.preadonly.borrow_mut().insert(ck, (r, inp));
        }
        r
    }

    fn compute_readonly(&self, t: &MemberRef, i: usize) -> bool {
        let Some(cf) = self.h.class(&t.owner) else { return false };
        let Some(meth) = cf.method(&t.name, &t.desc) else { return false };
        let Some(code) = meth.code.as_ref() else { return false };
        let Some(md) = parse_method(&t.desc) else { return false };
        let base = usize::from(!meth.is_static());
        let Some(p) = i.checked_sub(base).and_then(|j| md.params.get(j)) else { return false };
        let mut params = vec![None; md.params.len() + base];
        params[i] = Some(self.sysprops_ref(&p.descriptor()));
        let live = |_: &str| true;
        let a = self.aux_analyze(&cf.name, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params, mirrors: vec![], level: None, objs: Default::default() });
        if a.conservative {
            return false;
        }
        a.events.iter().all(|(_, e)| match e {
            Event::Invoke { opcode, mref, iface, args } => args
                .iter()
                .enumerate()
                .filter(|(_, v)| may_be_sysprops(v))
                .all(|(j, _)| self.sysprops_arg_ok(None, *opcode, mref, *iface, j)),
            Event::Field { value, .. } => !value.as_ref().is_some_and(may_be_sysprops),
            Event::ArrayStore { value, .. } => !may_be_sysprops(value),
            Event::Return(value) => !may_be_sysprops(value),
            Event::Indy { args, .. } => !args.iter().any(may_be_sysprops),
            _ => true,
        })
    }
}

impl Engine<'_> {
    /// 诊断：不折叠集合与首个全部不折叠的成因
    pub fn sysprops_report(&self) -> serde_json::Value {
        let u = self.ctx.punstable.borrow();
        serde_json::json!({"all": u.all, "cause": u.cause, "keys": u.keys})
    }

    /// 字节码方法里属性表对象的改写 / 逃逸（活代码）：不折叠集合增长
    pub(super) fn sysprops_scan(&mut self, m: usize, a: &Analysis) {
        if self.man.sysprops.is_empty() || self.ctx.punstable.borrow().all {
            return;
        }
        let mut keys: Vec<Option<String>> = Vec::new();
        let mut cause = String::new();
        let here = self.methods[m].key.to_string();
        for (_, e) in &a.events {
            let before = keys.iter().filter(|k| k.is_none()).count();
            match e {
                Event::Invoke { opcode, mref, iface, args } => {
                    if self.sysprops_virtual(*opcode, mref, *iface, a.conservative) {
                        keys.push(None);
                    }
                    let k = self.mref_key(mref);
                    if self.man.sysprops.is_holder(&k) && a.conservative {
                        keys.push(None);
                    }
                    for (i, _) in args.iter().enumerate().filter(|(_, v)| may_be_sysprops(v)) {
                        if self.ctx.sysprops_arg_ok(Some(m), *opcode, mref, *iface, i) {
                            continue;
                        }
                        let w = self.man.sysprops.writer(&k).filter(|_| i == 0);
                        let key = w.map(|w| (w, args.get(w.key).cloned().unwrap_or(V::Top)));
                        match key {
                            Some((w, v)) if w.remove => keys.extend(self.removed_keys(m, a, &v)),
                            Some((_, V::Str(s, _))) => keys.push(Some(s.to_string())),
                            _ => keys.push(None),
                        }
                    }
                    keys.extend(self.remove_wrapper_call(m, a, *opcode, mref, *iface, args));
                }
                Event::Field { opcode, mref, value, .. } => {
                    let holder = match self.ctx.field_info(mref) {
                        Some(fi) => {
                            let k = self.mref_key(&fi.key);
                            self.man.sysprops.is_holder(&k)
                        }
                        None => false,
                    };
                    let put = matches!(*opcode, classfile::op::PUTSTATIC | classfile::op::PUTFIELD);
                    if holder && (put || a.conservative) || value.as_ref().is_some_and(may_be_sysprops) {
                        keys.push(None);
                    }
                }
                Event::ArrayStore { value, .. } if may_be_sysprops(value) => keys.push(None),
                Event::Return(value) if may_be_sysprops(value) && self.sysprops_returner(m) => keys.push(None),
                Event::Indy { args, .. } if args.iter().any(may_be_sysprops) => keys.push(None),
                _ => {}
            }
            if cause.is_empty() && keys.iter().filter(|k| k.is_none()).count() > before {
                cause = format!("{here}：{}", event_brief(e));
            }
        }
        self.sysprops_unstable(keys, || cause);
    }

    /// 方法返回属性表对象：已有非字节码入口或同名同描述符的虚调用点 → 逃逸（true）
    fn sysprops_returner(&mut self, m: usize) -> bool {
        let key = self.methods[m].key.clone();
        let sig = (key.name.to_string(), key.desc.to_string());
        let r = &mut self.spret;
        let esc = r.untracked.contains(&key) || r.virt.contains(&sig);
        r.methods.insert(key.clone());
        r.nodes.entry(key.clone()).or_default().insert(m);
        esc || self.spret.lambda_in.contains(&key) && !self.sysprops_lambda_tracked(m, &key)
    }

    /// 目标不唯一（或分析保守、返回值由清单事实替换）的引用返回调用点：与返回属性表对象的方法同名同描述符 → 逃逸（true）
    fn sysprops_virtual(&mut self, opcode: u8, mref: &MemberRef, iface: bool, conservative: bool) -> bool {
        if !mref.desc.ends_with(';') {
            return false;
        }
        let c = self.ctx.call_info(opcode, mref, iface);
        if !conservative && c.target.is_some() && c.fact.is_none() {
            return false;
        }
        let r = &mut self.spret;
        let esc = r.methods.iter().any(|k| k.name == mref.name && k.desc == mref.desc);
        r.virt.insert((mref.name.to_string(), mref.desc.to_string()));
        esc
    }

    /// 方法入口：非字节码调用点（手写 / 反射 / 方法句柄 / lambda / VM 根）拿不到带标签的返回值
    pub(super) fn sysprops_entry(&mut self, key: &MemberRef, via: &Via) {
        if self.man.sysprops.is_empty() {
            return;
        }
        let bytecode = matches!(via.from, From::Method(c) if self.methods[c].kind == Kind::Bytecode);
        let tracked = matches!(via.kind, "invoke" | "dispatch") && bytecode;
        self.remove_entry(key, tracked);
        if !key.desc.ends_with(';') || tracked || self.spret.untracked.contains(key) {
            return;
        }
        if matches!(via.kind, "lambda" | "indy") && bytecode {
            // SAM 调用点进入（lambda），或静态 / 私有实现在创建点预先入链（indy）：判定推迟到实现方法
            // 返回属性表对象时（按 lambda 创建点；不是 lambda 实现方法的按不可跟踪）；已返回过的节点重分析重判
            if self.spret.lambda_in.insert(key.clone()) {
                self.sysprops_lambda_recheck(key);
            }
            return;
        }
        self.spret.untracked.insert(key.clone());
        if self.spret.methods.contains(key) {
            let (k, kind) = (key.to_string(), via.kind);
            self.sysprops_unstable(vec![None], || format!("{k}：非字节码入口（{kind}）"));
        }
    }

    /// 经 lambda 入口、已返回过属性表对象的实现方法：入口或创建点增加时其节点重分析重判
    pub(super) fn sysprops_lambda_recheck(&mut self, key: &MemberRef) {
        if !self.spret.lambda_in.contains(key) {
            return;
        }
        for n in self.spret.nodes.get(key).cloned().unwrap_or_default() {
            self.invalidate(n, Why::Sysprops);
        }
    }

    /// 返回常量合流后不再带属性表标签：调用方拿不到标签 → 全部不折叠
    pub(super) fn sysprops_rval(&mut self, m: usize, a: &Analysis, r: &PV) {
        if self.man.sysprops.is_empty() || self.ctx.punstable.borrow().all {
            return;
        }
        let ret = a.events.iter().any(|(_, e)| matches!(e, Event::Return(v) if may_be_sysprops(v)));
        let kept = matches!(r, PV::Const(v) if may_be_sysprops(v));
        let cur = self.ctx.rvals.borrow().get(&self.methods[m].key).cloned();
        let joined = PV::join(cur.as_ref(), r);
        if ret && !(kept && matches!(&joined, PV::Const(v) if may_be_sysprops(v))) {
            let k = self.methods[m].key.to_string();
            self.sysprops_unstable(vec![None], || format!("{k}：返回常量合流丢失属性表标签"));
        }
    }

    /// 手写体调用改写入口（接收者推不出）：全部不折叠
    pub(super) fn sysprops_hw(&mut self, host: &str, mh: &MemberHw) {
        if self.man.sysprops.is_empty() {
            return;
        }
        let hit = self.hw_upcalls(host, mh).iter().any(|u| matches!(u, Upcall::Method(r) if self.man.sysprops.writer(&r.to_string()).is_some()));
        if hit {
            self.sysprops_unstable(vec![None], || "手写体调用改写入口".to_string());
        }
    }

    /// 不折叠集合并入（None = 全部）；增长时依赖缓存清空、折叠过的方法失效
    pub(super) fn sysprops_unstable(&mut self, keys: Vec<Option<String>>, cause: impl FnOnce() -> String) {
        let grew = {
            let mut u = self.ctx.punstable.borrow_mut();
            let mut grew = false;
            let mut cause = Some(cause);
            for k in keys {
                match k {
                    None if !u.all => {
                        u.all = true;
                        u.cause = cause.take().map(|f| f());
                        grew = true;
                    }
                    Some(k) if !u.all => grew |= u.keys.insert(k),
                    _ => {}
                }
            }
            grew
        };
        if !grew {
            return;
        }
        // 只作废答复可能变化的记忆（查询过不折叠集合，或读过的字段此后已放开）：其余记忆的输入未变，
        // 重算结果相同。取用过作废记忆的方法经条目编号失效；属性读者（含取用过不可记忆结果的方法）全部失效
        let ctx = &self.ctx;
        let mut open: HashMap<MemberRef, bool> = HashMap::default();
        let mut ids: BTreeSet<u32> = BTreeSet::new();
        let mut keep = |inp: &super::memo::Inputs| {
            let stale = ctx.memo_stale(inp, &mut open);
            if stale {
                ids.insert(inp.id);
            }
            !stale
        };
        ctx.consts.borrow_mut().retain(|_, (_, inp)| keep(inp));
        ctx.objs.borrow_mut().retain(|_, (_, inp)| keep(inp));
        ctx.psums.borrow_mut().retain(|_, (_, inp)| keep(inp));
        ctx.cevals.borrow_mut().retain(|_, (_, inp)| keep(inp));
        ctx.preadonly.borrow_mut().retain(|_, (_, inp)| keep(inp));
        let n = ctx.cinits.borrow().len();
        ctx.cinits.borrow_mut().retain(|_, c| keep(&c.inp));
        if ctx.cinits.borrow().len() != n {
            ctx.cinit_drop.set(true);
        }
        // 删除包装方法的形参摘要只看字节码结构，与不折叠集合无关，不作废
        let mut deps: BTreeSet<usize> = std::mem::take(&mut *ctx.pdeps.borrow_mut());
        deps.extend(ctx.memo_consumers(ids));
        self.invalidate_all(Some(deps), Why::Sysprops);
    }
}

/// 事件简述（诊断）
fn event_brief(e: &Event) -> String {
    match e {
        Event::Invoke { mref, .. } => format!("属性表对象作实参 / 接收者调用 {mref}"),
        Event::Field { mref, .. } => format!("字段 {mref} 读写属性表对象"),
        Event::ArrayStore { .. } => "属性表对象存入数组".into(),
        Event::Return(_) => "返回属性表对象（有非字节码入口、不封闭的 lambda 入口或同名虚调用点）".into(),
        Event::Indy { .. } => "属性表对象作 indy 实参".into(),
        _ => "其他".into(),
    }
}

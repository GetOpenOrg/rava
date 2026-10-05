//! 引擎：按名取类——把字符串值流拼出的类名（各段可确定时折叠为常量串集合）解析成具体类集；推不出时记 top（unknown 兜底）。
//!
//! 形状（全部由清单事实定义，分析器不含类名）：
//! - 名字实参是字符串常量，或一条字符串拼接（`[facts.string_concat]`）：indy `makeConcatWithConstants`，或构建器
//!   取结果链（链首之前的初始内容与独立追加语句按控制流确定顺序，见 `builder.rs`）；各段是字符串常量 / null、
//!   基本类型常量（按 `String.valueOf` 成字面量）、类镜像取名结果（binary name / 简单名，见 `name_eval.rs`）、
//!   引擎方法 String 形参上各调用点流入的名字（见 `pstrs.rs::param_names`）、辅助方法的返回值（在被调帧里递归拆段，
//!   被调形参换成调用点实参），或「常量表读取」的结果（可经一次 checkcast）；
//! - 常量表读取：接收者是 `[facts.reflect] constant_tables` 基类的子类对象、调用其读取入口。常量表子类是生成的
//!   不可变映射，内容即子类自身代码里的字符串常量——候选值取该子类全部方法的 ldc 字符串（超集，安全）；
//!   接收者值集另含 open / 非常量表部分时，给出常量表部分的候选，结果另接所指未知的 Class（按名查方法记为任意串）；
//! - 枚举取值：段是枚举常量上 final 字符串字段的平凡取值方法（见 `method_lookup.rs`）；
//! - 按名查方法（`[facts.reflect] method_lookups`）复用同一拆段，推不出的段记为任意串，见 `method_lookup.rs`；
//! - 辅助方法的返回值：唯一目标，或按接收者值集派发的各目标（`callee_returns`），每个返回值一支（`Part::Alt`），
//!   展开后逐支成候选模式（`expand`，至多 64 个）；
//! - 按名取类（`Gap::Class`）的推不出段记为任意串：含任意串的候选只匹配闭包里的类名（运行期按名只取得到 VM 登记的
//!   生成类），站点模式只增不减，新类进入闭包时站点重跑（`pattern_class_added`）；按名加载（`class_loads`）同此口径；
//! - 候选名笛卡尔积后只保留类路径上存在的类；结果唯一地经实例化入口（`instantiators`）再唯一地 checkcast 到 T 时，
//!   只保留 T 的子类型（其余候选在运行时抛 ClassCastException，不产生可观察的成员调用）。
//!
//! 解析成功的调用点结果只含这些类的镜像（不再流入所指未知的 Class），类随之初始化、其构造器进入反射面。

use super::builder::{builder_prefix, seg_kind, Seg};
use super::method_lookup::{constrained, parts_match};
use super::name_eval::{prim_lit, Frame};
use super::sealed::flatten;
use super::*;

/// 候选名数上限：超出按推不出处理
pub(super) const MAX_NAMES: usize = 4096;

/// 候选模式数上限（拼接段里的多选展开后）：超出按推不出处理
const MAX_PATTERNS: usize = 64;

/// 拼接段：字面量、候选集、任意串（由目标类上的方法名 / 生成范围内的类名反向匹配），或多选（辅助方法的各返回值 /
/// 各派发目标，每一支是一串拼接段）
#[derive(Clone, Debug, PartialEq)]
pub(super) enum Part {
    Lit(Rc<str>),
    Any(BTreeSet<Rc<str>>),
    Wild,
    Alt(Vec<Vec<Part>>),
}

/// 推不出的段如何处理
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Gap {
    /// 整体推不出（封存字段 / 形参名字的内部求值）；常量表读取的接收者含非常量表值时给出常量表部分并记 top
    Fail,
    /// 按名查方法：记为任意串；形参透传的名字不在此求（调用点字符串常量另行点名）
    Method,
    /// 按名取类：记为任意串（含任意串的候选按生成范围内的类名匹配）；常量表部分读取给出「常量表候选 | 任意串」
    Class,
}

impl Gap {
    fn wild(self) -> bool {
        self != Gap::Fail
    }
}

/// 拼接段展开成不含多选的候选模式；超出上限为 None
pub(super) fn expand(parts: &[Part]) -> Option<Vec<Vec<Part>>> {
    let mut out: Vec<Vec<Part>> = vec![vec![]];
    for p in parts {
        match p {
            Part::Alt(alts) => {
                let mut subs = vec![];
                for a in alts {
                    subs.extend(expand(a)?);
                }
                if out.len().saturating_mul(subs.len()) > MAX_PATTERNS {
                    super::stats::cap_hit(super::stats::CAP_PATTERNS);
                    return None;
                }
                out = out.iter().flat_map(|o| subs.iter().map(move |s| o.iter().chain(s.iter()).cloned().collect())).collect();
            }
            p => out.iter_mut().for_each(|o| o.push(p.clone())),
        }
    }
    Some(out)
}

pub(super) fn site_of(v: &V) -> Option<u32> {
    match v {
        V::Ref { src, .. } if src.len() == 1 => match src[0] {
            Src::Site(o) => Some(o),
            _ => None,
        },
        _ => None,
    }
}

/// 事件里出现的全部值
fn event_values(e: &Event) -> Vec<&V> {
    match e {
        Event::Invoke { args, .. } | Event::Indy { args, .. } => args.iter().collect(),
        Event::Field { recv, value, .. } => recv.iter().chain(value.iter()).collect(),
        Event::CheckCast(_, v) | Event::InstanceOf(_, v) => v.iter().collect(),
        Event::NotInstance(_, v) => vec![v],
        Event::ArrayLoad { array, index } => vec![array, index],
        Event::ArrayStore { array, index, value } => vec![array, index, value],
        Event::Throw(v) | Event::Return(v) => vec![v],
        _ => vec![],
    }
}

/// 使用来源站点 o 的（事件, 值）：同一事件里出现多次计多次
pub(super) fn uses(a: &Analysis, o: u32) -> Vec<(u32, &Event, &V)> {
    let mut out = vec![];
    for (off, e) in &a.events {
        for v in event_values(e) {
            if let V::Ref { src, .. } = v {
                if src.contains(&Src::Site(o)) {
                    out.push((*off, e, v));
                }
            }
        }
    }
    out
}

pub(super) fn event_at(a: &Analysis, o: u32, f: impl Fn(&Event) -> bool) -> Option<&Event> {
    a.events.iter().find(|(off, e)| *off == o && f(e)).map(|(_, e)| e)
}

pub(super) fn is_invoke(e: &Event) -> bool {
    matches!(e, Event::Invoke { .. })
}

impl<'a> Engine<'a> {
    /// 按名取类调用点（方法 m、偏移 off、实参 args）的所指类集与是否推不出（top）。
    /// 类集为空且非 top = 候选来源尚未流到（值集增长时重跑）；top = 结果另接被调方法返回的所指未知的 Class。
    ///
    /// 单调：同一调用点一旦 top 即恒为 top（已接的返回边不撤）；类集是当前状态的单调函数——常量表读取的接收者
    /// 含非常量表值时仍给出常量表部分的候选并记 top，而不是整体放弃，因此结果与接收者值到达的先后无关。
    /// 求值读本方法其它偏移的事件（拼接链、常量表接收者、结果用途）与辅助方法读过的字段，登记为跨偏移读者：
    /// 重分析时一并重跑（见 `bytecode.rs::process_bytecode`）
    pub(super) fn class_lookup(&mut self, m: usize, off: u32, args: &[V]) -> (Vec<String>, bool) {
        self.xreaders.entry(m).or_default().insert(off);
        // 服务实现类的反射构造点：所指类由 JCA 规则按被请求的算法补种（实例化 + 构造器入链），站点本身按推不出处理，
        // 不按类名字段的字符串集解析——与求值时机无关，恒为同一结果
        if self.lookup_hosts.contains(&self.methods[m].key) {
            self.lookup_top.insert((m, off));
            return (Vec::new(), true);
        }
        let site = (m, off);
        let trial = self.lookup_trial.remove(&site);
        let sticky = self.lookup_top.contains(&site);
        self.lookup_partial = false;
        self.lookup_incomplete = false;
        let r = self.class_lookup_eval(m, off, args);
        let partial = std::mem::take(&mut self.lookup_partial);
        if std::mem::take(&mut self.lookup_incomplete) {
            self.lookup_unsure.insert(site);
        }
        let unsure = self.lookup_unsure.contains(&site);
        let top = sticky || r.is_none() || partial || unsure;
        if top {
            self.lookup_top.insert(site);
        }
        let names = r.unwrap_or_default();
        // 名字是字符串常量：结果与求值时机无关，直接解析
        if matches!(args.first(), Some(V::Str(..))) || self.lookup_released.contains(&site) {
            return (names, top);
        }
        // 未放行时名字推不出（某支无约束任意串 / 形参或字段名字集不完备）：不按已知名字加载，结果接所指未知的 Class
        if unsure {
            return (Vec::new(), true);
        }
        if names.is_empty() {
            return (names, top);
        }
        // 名字齐全：只在排空时（不动点上）仍齐全才放行（`lookup_release`）。中途齐全、终态推不出的站点不加载，
        // 结果因而与求值先后无关
        if trial {
            self.lookup_released.insert(site);
            return (names, top);
        }
        self.lookup_pending.insert(site);
        (Vec::new(), top)
    }

    /// 工作队列排空时放行挂起的按名取类站点：各站点重跑，求值仍齐全即解析名字（之后按单调口径照常求值）。
    /// 返回是否有站点放行
    pub(super) fn lookup_release(&mut self) -> bool {
        let ready: Vec<(usize, u32)> =
            std::mem::take(&mut self.lookup_pending).into_iter().filter(|w| !self.lookup_unsure.contains(w) && !self.lookup_released.contains(w)).collect();
        if !ready.is_empty() {
            let (edges, adds) = (self.graph.edge_count, self.graph.adds[0]);
            self.ctx.stats.borrow_mut().released(ready.len(), edges, adds);
        }
        for &w in &ready {
            self.lookup_trial.insert(w);
            self.push_site(w, site_prof::TRIG_RELEASE, None);
        }
        !ready.is_empty()
    }

    fn class_lookup_eval(&mut self, m: usize, off: u32, args: &[V]) -> Option<Vec<String>> {
        let a = self.methods[m].analysis.clone()?;
        if a.conservative {
            return None;
        }
        let owner = self.methods[m].key.owner.clone();
        let f = Frame { m: Some(m), a: &a, owner: &owner, up: None };
        let parts = self.name_parts(&f, args.first()?, Gap::Class, 0).unwrap_or_else(|| vec![Part::Wild]);
        let mut names: Vec<String> = vec![];
        let mut wild: Vec<Vec<Part>> = vec![];
        for pat in expand(&parts)? {
            if pat.iter().any(|p| matches!(p, Part::Wild)) {
                // 无约束的任意串：该支推不出（记 top），其余支的已知名字照常解析——结果只并不减
                if !constrained(&pat) {
                    self.lookup_incomplete = true;
                    continue;
                }
                wild.push(pat);
                continue;
            }
            names.extend(flatten(&pat)?.iter().map(|n| n.to_string()));
            if names.len() > MAX_NAMES {
                super::stats::cap_hit(super::stats::CAP_LNAMES);
                return None;
            }
        }
        let expect = self.expected_type(&a, off);
        let mut out: BTreeSet<String> = BTreeSet::new();
        // 含任意串的候选：运行期按名只取得到生成范围内的类（VM 只登记生成的类），按闭包里的类名匹配。
        // 站点的模式只增不减（各次求值的并集，结果单调）；新类进入闭包时由 `pattern_class_added` 重跑本站点
        if !wild.is_empty() {
            let pats = self.class_patterns.entry((m, off)).or_default();
            for p in wild {
                if !pats.contains(&p) {
                    pats.push(p);
                }
            }
        }
        if let Some(pats) = self.class_patterns.get(&(m, off)) {
            for cls in self.classes.keys() {
                let dotted = cls.replace('/', ".");
                if pats.iter().any(|p| parts_match(p, &dotted)) && expect.as_ref().is_none_or(|t| self.h.is_subtype(cls, t)) {
                    out.insert(cls.clone());
                }
            }
        }
        for n in names {
            let cls = n.replace('.', "/");
            // 数组类名（描述符形式）：元素类型可解析时取到数组类镜像；数组类不可实例化，有期望类型时不保留
            let found = if cls.starts_with('[') { expect.is_none() && self.array_name_resolves(&cls) } else { self.h.class(&cls).is_some() };
            if !found {
                continue;
            }
            if !cls.starts_with('[') && expect.as_ref().is_some_and(|t| !self.h.is_subtype(&cls, t)) {
                continue;
            }
            out.insert(cls);
        }
        Some(out.into_iter().collect())
    }

    /// 名字值拆成拼接段；gap = 推不出的段的处理（见 [`Gap`]）
    /// f = 值所在的帧（引擎方法，或被调方法的独立分析——不读常量表与值集，形参换成调用方帧里的实参）；
    /// depth = 已穿过的辅助方法层数（名字由唯一目标的辅助方法拼出并返回时，进入其字节码继续拆）
    pub(super) fn name_parts(&mut self, f: &Frame, v: &V, gap: Gap, depth: u8) -> Option<Vec<Part>> {
        let (f, v) = f.resolve(v);
        if let V::Str(s, _) = &v {
            return Some(vec![Part::Lit(s.clone())]);
        }
        let Some(o) = site_of(&v) else {
            // 引擎方法的形参：各调用点流入的名字（按名取类）
            return self.segment_values(f, &v, gap, depth).map(|p| vec![p]);
        };
        let a = f.a;
        if let Some(segs) = self.indy_concat_segs(f.owner, a, o) {
            return self.seg_parts(f, &segs, gap, depth);
        }
        let Some(Event::Invoke { mref, args, .. }) = event_at(a, o, is_invoke) else {
            // 非调用结果（如直接读字段）：整体作为一段
            return self.segment_values(f, &v, Gap::Fail, depth).map(|p| vec![p]);
        };
        if !self.man.names.is_result(&mref.to_string()) {
            if let Some(r) = self.mirror_name(f, o) {
                return r.map(|set| vec![Part::Any(set)]);
            }
            // 辅助方法（唯一目标，或按接收者值集派发的各目标）的各返回值：每个返回值一支
            let mut alts = self.callee_alts(f, o, args, gap, depth)?;
            return Some(if alts.len() == 1 { alts.pop()? } else { vec![Part::Alt(alts)] });
        }
        let names = &self.man.names;
        let mut segs: Vec<Seg> = vec![];
        let mut cur = args.first()?.clone();
        // 使用 cur 的链上事件偏移（取结果 / 追加）
        let mut p = o;
        loop {
            let s = site_of(&cur)?;
            let append = event_at(a, s, is_invoke).and_then(|e| match e {
                Event::Invoke { mref, args, .. } if names.is_append(&mref.to_string()) => Some((args.clone(), seg_kind(&mref.desc))),
                _ => None,
            });
            if let Some((args, k)) = append {
                if uses(a, s).len() != 1 {
                    return None;
                }
                segs.push((args.get(1)?.clone(), k));
                cur = args.first()?.clone();
                p = s;
                continue;
            }
            // 链首：构建器此前已有的内容（初始内容 + 独立追加语句）
            let mut prefix = builder_prefix(names, a, s, p)?;
            segs.reverse();
            prefix.extend(segs);
            segs = prefix;
            break;
        }
        self.seg_parts(f, &segs, gap, depth)
    }

    /// 站点 o 的辅助方法调用（实参 args）的各返回值拆成的拼接段，每个返回值一支；推不出的支按 gap 记为任意串
    fn callee_alts(&mut self, f: &Frame, o: u32, args: &[V], gap: Gap, depth: u8) -> Option<Vec<Vec<Part>>> {
        let rets = self.callee_returns(f, o, depth)?;
        let mut alts = Vec::with_capacity(rets.len());
        for (ca, rv, owner) in &rets {
            let cf = Frame { m: None, a: ca, owner, up: Some((f, args)) };
            match self.name_parts(&cf, rv, gap, depth + 1) {
                Some(p) => alts.push(p),
                None if gap.wild() => alts.push(vec![Part::Wild]),
                None => return None,
            }
        }
        Some(alts)
    }

    /// 拼接各段的值 → 拼接段
    fn seg_parts(&mut self, f: &Frame, segs: &[Seg], gap: Gap, depth: u8) -> Option<Vec<Part>> {
        let mut parts = Vec::with_capacity(segs.len());
        for (s, k) in segs {
            let (sf, s) = f.resolve(s);
            parts.push(match &s {
                V::Str(x, _) => Part::Lit(x.clone()),
                V::Null => Part::Lit(Rc::from("null")),
                V::Ref { .. } => match self.segment_values(sf, &s, gap, depth) {
                    Some(p) => p,
                    None if gap.wild() => Part::Wild,
                    None => return None,
                },
                _ => match prim_lit(&s, *k) {
                    Some(l) => Part::Lit(l),
                    None if gap.wild() => Part::Wild,
                    None => return None,
                },
            });
        }
        Some(parts)
    }

    /// 站点 o 是 owner 类代码里的字符串拼接 indy（引导方法为清单 `[facts.indy]` 的 concat 类）时按配方拆出的各段值：
    /// 配方取引导静态实参首项（`\u{1}` = 依次取动态实参、`\u{2}` = 依次取其后的静态常量，其余字符为字面量）；
    /// 无配方的引导（各动态实参直接相接）逐个动态实参成段。静态常量非字符串时不拆
    fn indy_concat_segs(&self, owner: &str, a: &Analysis, o: u32) -> Option<Vec<Seg>> {
        let Event::Indy { bsm, desc, args, .. } = event_at(a, o, |e| matches!(e, Event::Indy { .. }))? else { return None };
        let cf = self.h.class(owner)?;
        let b = cf.bootstrap_methods.get(*bsm as usize)?;
        let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
        if self.man.indy_kind(&bkey) != Some(IndyKind::Concat) {
            return None;
        }
        // 动态实参的类型首字母（基本类型段按 `String.valueOf` 成字面量）
        let kinds: Vec<u8> = parse_method(desc)?
            .params
            .iter()
            .map(|p| match p {
                FieldType::Prim(c) => *c,
                _ => b'L',
            })
            .collect();
        let dynamic = |i: usize| -> Option<Seg> { Some((args.get(i)?.clone(), *kinds.get(i)?)) };
        let Some(Const::String(recipe)) = b.args.first() else {
            return b.args.is_empty().then(|| (0..args.len()).map(dynamic).collect::<Option<Vec<_>>>()).flatten();
        };
        let mut segs = vec![];
        let mut lit = String::new();
        let (mut dyn_i, mut const_i) = (0, 1);
        let flush = |lit: &mut String, segs: &mut Vec<Seg>| {
            if !lit.is_empty() {
                segs.push((V::lit(Rc::from(std::mem::take(lit).as_str())), b'L'));
            }
        };
        for ch in recipe.chars() {
            match ch {
                '\u{1}' => {
                    flush(&mut lit, &mut segs);
                    segs.push(dynamic(dyn_i)?);
                    dyn_i += 1;
                }
                '\u{2}' => {
                    let Const::String(c) = b.args.get(const_i)? else { return None };
                    lit.push_str(c);
                    const_i += 1;
                }
                c => lit.push(c),
            }
        }
        flush(&mut lit, &mut segs);
        Some(segs)
    }

    /// 引用值段：引擎方法形参（各调用点流入的名字）→ 类镜像取名 → 封存静态字段（值映射读取 / 常量字符串数组元素，`sealed.rs`）
    /// → 常量表读取 → 枚举取值 → 返回字符串常量的辅助方法 → 辅助方法拼出的名字（按顺序取第一个成形的）
    fn segment_values(&mut self, f: &Frame, v: &V, gap: Gap, depth: u8) -> Option<Part> {
        let a = f.a;
        if let (Some(m), None) = (f.m, site_of(v)) {
            // 按名查方法的形参名字由调用点的字符串常量另行点名（`param_strs`），这里只服务按名取类
            let [Src::Param(i)] = v.srcs()[..] else { return None };
            return if gap == Gap::Method { None } else { self.param_names(m, i as usize, depth).map(|(set, complete)| self.partial_part(set, complete, gap)) };
        }
        let o = site_of(v)?;
        if let Some(r) = self.mirror_name(f, o) {
            return match r {
                Some(set) => Some(Part::Any(set)),
                None => gap.wild().then_some(Part::Wild),
            };
        }
        if let Some(set) = self.sealed_segment(a, v) {
            return Some(Part::Any(set));
        }
        if let Some((set, partial)) = f.m.and_then(|m| self.table_values(m, a, v)) {
            // 接收者含非常量表值：按名查方法记为任意串；按名取类给出「常量表候选 | 任意串」（候选类照常点名，
            // 其余按生成范围内的类名匹配）；内部求值给出常量表部分并记 top
            if !partial {
                return Some(Part::Any(set));
            }
            return Some(match gap {
                Gap::Method => Part::Wild,
                Gap::Class => Part::Alt(vec![vec![Part::Any(set)], vec![Part::Wild]]),
                Gap::Fail => {
                    self.lookup_partial = true;
                    Part::Any(set)
                }
            });
        }
        if let Some(set) = self.enum_field_values(a, v) {
            return Some(Part::Any(set));
        }
        if let Some((set, complete)) = f.m.and_then(|m| self.read_field_names(m, a, o, depth)) {
            return Some(self.partial_part(set, complete, gap));
        }
        if let Some(set) = self.callee_consts(a, o, depth) {
            return Some(Part::Any(set));
        }
        // 辅助方法拼出的段：各返回值都能拍平时合成一个候选集，否则（含任意串）保留为多选
        let Some(Event::Invoke { args, .. }) = event_at(a, o, is_invoke) else { return None };
        let alts = self.callee_alts(f, o, args, gap, depth)?;
        let mut set = BTreeSet::new();
        for p in &alts {
            match flatten(p) {
                Some(s) => set.extend(s),
                None => return gap.wild().then_some(Part::Alt(alts)),
            }
        }
        Some(Part::Any(set))
    }

    /// 槽求值结果（名字集, 是否推得出）→ 拼接段。推不出时不丢弃已知名字（否则结果取决于求值发生在推不出之前
    /// 还是之后）：按名查方法记为任意串；按名取类给出「已知名字 | 任意串」；内部求值给出已知名字并记 top
    fn partial_part(&mut self, set: BTreeSet<Rc<str>>, complete: bool, gap: Gap) -> Part {
        if complete {
            return Part::Any(set);
        }
        match gap {
            Gap::Method => Part::Wild,
            Gap::Class => Part::Alt(vec![vec![Part::Any(set)], vec![Part::Wild]]),
            Gap::Fail => {
                self.lookup_incomplete = true;
                Part::Any(set)
            }
        }
    }

    /// 常量表读取结果的候选字符串（可经一次 checkcast）与接收者是否含非常量表值；接收者尚无值时为空集
    fn table_values(&mut self, m: usize, a: &Analysis, v: &V) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let o = site_of(v)?;
        if let Some(Event::CheckCast(_, Some(inner))) = event_at(a, o, |e| matches!(e, Event::CheckCast(..))) {
            let inner = inner.clone();
            let o2 = site_of(&inner)?;
            return self.table_read(m, a, o2);
        }
        self.table_read(m, a, o)
    }

    fn table_read(&mut self, m: usize, a: &Analysis, o: u32) -> Option<(BTreeSet<Rc<str>>, bool)> {
        let Event::Invoke { opcode, mref, args, .. } = event_at(a, o, is_invoke)? else { return None };
        if *opcode == classfile::op::INVOKESTATIC {
            return None;
        }
        let sig = format!("{}:{}", mref.name, mref.desc);
        let bases: Vec<String> = self.man.names.table_bases(&sig).map(String::from).collect();
        if bases.is_empty() {
            return None;
        }
        let owner = self.id(&mref.owner);
        let recv = args.first()?.clone();
        let fs = self.feeds(m, &recv, owner);
        let s = self.value_set(&fs);
        let mut partial = !s.open.is_empty();
        let mut tables = false;
        let mut out = BTreeSet::new();
        let xs: Vec<u32> = s.classes.iter().collect();
        for x in xs {
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                partial = true;
                continue;
            }
            let t = self.ty(x);
            let cls = self.names[t as usize].to_string();
            let cf = match self.h.class(&cls) {
                Some(cf) if bases.iter().any(|b| self.h.is_subtype(&cls, b)) => cf,
                _ => {
                    partial = true;
                    continue;
                }
            };
            tables = true;
            for mm in &cf.methods {
                for i in mm.code.iter().flat_map(|c| c.insns.iter()) {
                    if let classfile::Operand::Ldc(Const::String(s)) = &i.operand {
                        out.insert(Rc::from(s.as_str()));
                    }
                }
            }
        }
        // 接收者全不是常量表（且非尚无值）：不是常量表读取，交给其余拆段方式
        if partial && !tables {
            return None;
        }
        Some((out, partial))
    }

    /// 按名取类结果唯一地经实例化入口、其结果再唯一地 checkcast 到 T：返回 T
    fn expected_type(&self, a: &Analysis, off: u32) -> Option<String> {
        let us = uses(a, off);
        let [(o1, Event::Invoke { mref, args, .. }, _)] = us.as_slice() else { return None };
        if !self.man.names.is_instantiator(&mref.to_string()) || args.first().and_then(site_of) != Some(off) {
            return None;
        }
        let us2 = uses(a, *o1);
        let [(_, Event::CheckCast(t, Some(_)), _)] = us2.as_slice() else { return None };
        Some(t.clone())
    }

    /// 描述符形式的数组类名（`[I`、`[[Lp/C;`）：元素是单字符基本类型（非 void）或类路径上存在的类
    fn array_name_resolves(&self, cls: &str) -> bool {
        let elem = cls.trim_start_matches('[');
        match elem.strip_prefix('L').and_then(|c| c.strip_suffix(';')) {
            Some(c) => self.h.class(c).is_some(),
            None => elem.len() == 1 && "ZBCSIJFD".contains(elem),
        }
    }

    /// 新类进入闭包：按名取类站点里含任意串的候选模式能匹配该类名时重跑该站点
    pub(super) fn pattern_class_added(&mut self, cls: &str) {
        let dotted = cls.replace('/', ".");
        let hits: Vec<(usize, u32)> = self.class_patterns.iter().filter(|(_, ps)| ps.iter().any(|p| parts_match(p, &dotted))).map(|(w, _)| *w).collect();
        for w in hits {
            self.push_site(w, site_prof::TRIG_PATTERN, None);
        }
    }

    /// 按名取到的类：其构造器进入反射面（类已被构造器枚举且反射构造可达时立即补入）；
    /// 数组类只取镜像（同 ldc 数组类常量），不初始化元素类（JLS §12.4.1）；按名加载（init = false）只取镜像、不初始化
    pub(super) fn named_class(&mut self, m: usize, off: u32, cls: &str, init: bool) {
        let k = self.mirror(cls);
        self.add_to(Node::S(m, off), &TypeSet::exact(k));
        if cls.starts_with('[') || !init {
            self.touch(cls, Level::Type, Via::method("reflect", m, Some(off)));
            if cls.starts_with('[') {
                return;
            }
        } else {
            // 运行期按名取类（`Class.forName(名, true, …)`）经类初始化钩子触发 `<clinit>`：登记为钩子目标，
            // 否则分析上已初始化的类在运行期不跑 `<clinit>`（如 SharedSecrets 惰性访问器所依赖的登记写入）
            self.seeds.mirror_inits.insert(cls.to_string());
            self.init(cls, Via::method("reflect", m, Some(off)));
        }
        let c = self.id(cls);
        if self.named_ctors.insert(c) && self.enumerated.contains(&(Members::Constructors, c)) && self.invokable.contains(&Members::Constructors) {
            self.expose(Members::Constructors, c);
        }
    }
}

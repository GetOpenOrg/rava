//! 引擎：按名取类——把「常量前缀 + 常量表取值」拼出的类名解析成具体类集。
//!
//! 形状（全部由清单事实定义，分析器不含类名）：
//! - 名字实参是字符串常量，或一条线性的字符串拼接链（`[facts.string_concat]`：新建构建器 → 逐段追加 → 取结果，
//!   每个中间值只被链上下一步使用一次）；各段是字符串常量 / null，或「常量表读取」的结果（可经一次 checkcast）；
//! - 常量表读取：接收者是 `[facts.reflect] constant_tables` 基类的子类对象、调用其读取入口。常量表子类是生成的
//!   不可变映射，内容即子类自身代码里的字符串常量——候选值取该子类全部方法的 ldc 字符串（超集，安全）；
//!   接收者值集含 open 部分时推不出，按原样处理（返回所指未知的 Class）；
//! - 枚举取值：段是枚举常量上 final 字符串字段的平凡取值方法（见 `method_lookup.rs`）；
//! - 按名查方法（`[facts.reflect] method_lookups`）复用同一拆段，推不出的段记为任意串，见 `method_lookup.rs`；
//! - 候选名笛卡尔积后只保留类路径上存在的类；结果唯一地经实例化入口（`instantiators`）再唯一地 checkcast 到 T 时，
//!   只保留 T 的子类型（其余候选在运行时抛 ClassCastException，不产生可观察的成员调用）。
//!
//! 解析成功的调用点结果只含这些类的镜像（不再流入所指未知的 Class），类随之初始化、其构造器进入反射面。

use super::*;

/// 候选名数上限：超出按推不出处理
const MAX_NAMES: usize = 4096;

/// 拼接段：字面量、候选集，或任意串（仅按名查方法：由目标类上的方法名反向匹配）
pub(super) enum Part {
    Lit(Rc<str>),
    Any(BTreeSet<Rc<str>>),
    Wild,
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
        Event::CheckCast(_, v) => v.iter().collect(),
        Event::ArrayLoad { array, index } => vec![array, index],
        Event::ArrayStore { array, index, value } => vec![array, index, value],
        Event::Throw(v) | Event::Return(v) => vec![v],
        _ => vec![],
    }
}

/// 使用来源站点 o 的（事件, 值）：同一事件里出现多次计多次
fn uses(a: &Analysis, o: u32) -> Vec<(u32, &Event, &V)> {
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
    /// 按名取类调用点（方法 m、偏移 off、实参 args）的所指类集；Some(空) = 候选来源尚未流到（值集增长时重跑），
    /// None = 形状不符（按原样返回所指未知的 Class）
    pub(super) fn class_lookup(&mut self, m: usize, off: u32, args: &[V]) -> Option<Vec<String>> {
        let a = self.methods[m].analysis.clone()?;
        if a.conservative {
            return None;
        }
        let parts = self.name_parts(m, &a, args.first()?, false)?;
        let mut names: Vec<String> = vec![String::new()];
        for p in &parts {
            names = match p {
                Part::Lit(s) => names.into_iter().map(|n| n + s).collect(),
                Part::Any(set) => {
                    if names.len().saturating_mul(set.len()) > MAX_NAMES {
                        return None;
                    }
                    names.iter().flat_map(|n| set.iter().map(move |s| format!("{n}{s}"))).collect()
                }
                Part::Wild => return None,
            };
        }
        let expect = self.expected_type(&a, off);
        let mut out: BTreeSet<String> = BTreeSet::new();
        for n in names {
            let cls = n.replace('.', "/");
            if cls.starts_with('[') || self.h.class(&cls).is_none() {
                continue;
            }
            if expect.as_ref().is_some_and(|t| !self.h.is_subtype(&cls, t)) {
                continue;
            }
            out.insert(cls);
        }
        Some(out.into_iter().collect())
    }

    /// 名字值拆成拼接段；wild = 推不出的段记为任意串（否则整体推不出）
    pub(super) fn name_parts(&mut self, m: usize, a: &Analysis, v: &V, wild: bool) -> Option<Vec<Part>> {
        if let V::Str(s) = v {
            return Some(vec![Part::Lit(s.clone())]);
        }
        let o = site_of(v)?;
        let names = &self.man.names;
        let Event::Invoke { mref, args, .. } = event_at(a, o, is_invoke)? else { return None };
        if !names.is_result(&mref.to_string()) {
            return None;
        }
        let mut segs: Vec<V> = vec![];
        let mut cur = args.first()?.clone();
        loop {
            let s = site_of(&cur)?;
            let append = event_at(a, s, is_invoke).and_then(|e| match e {
                Event::Invoke { mref, args, .. } if names.is_append(&mref.to_string()) => Some(args.clone()),
                _ => None,
            });
            if let Some(args) = append {
                if uses(a, s).len() != 1 {
                    return None;
                }
                segs.push(args.get(1)?.clone());
                cur = args.first()?.clone();
                continue;
            }
            // 链首：新建构建器，恰被构造器与第一次追加（或取结果）各使用一次
            event_at(a, s, |e| matches!(e, Event::New(_)))?;
            let us = uses(a, s);
            let init = us.iter().find_map(|(_, e, _)| match e {
                Event::Invoke { mref, args, .. } if names.is_builder(&mref.to_string()) && args.first().and_then(site_of) == Some(s) => Some(args.clone()),
                _ => None,
            })?;
            if us.len() != 2 {
                return None;
            }
            if let Some(init_v) = init.get(1) {
                segs.push(init_v.clone());
            }
            break;
        }
        segs.reverse();
        let mut parts = Vec::with_capacity(segs.len());
        for s in &segs {
            parts.push(match s {
                V::Str(x) => Part::Lit(x.clone()),
                V::Null => Part::Lit(Rc::from("null")),
                V::Ref { .. } => match self.table_values(m, a, s).or_else(|| self.enum_field_values(a, s)) {
                    Some(set) => Part::Any(set),
                    None if wild => Part::Wild,
                    None => return None,
                },
                _ if wild => Part::Wild,
                _ => return None,
            });
        }
        Some(parts)
    }

    /// 常量表读取结果的候选字符串（可经一次 checkcast）；接收者尚无值时为空集
    fn table_values(&mut self, m: usize, a: &Analysis, v: &V) -> Option<BTreeSet<Rc<str>>> {
        let o = site_of(v)?;
        if let Some(Event::CheckCast(_, Some(inner))) = event_at(a, o, |e| matches!(e, Event::CheckCast(..))) {
            let inner = inner.clone();
            let o2 = site_of(&inner)?;
            return self.table_read(m, a, o2);
        }
        self.table_read(m, a, o)
    }

    fn table_read(&mut self, m: usize, a: &Analysis, o: u32) -> Option<BTreeSet<Rc<str>>> {
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
        if !s.open.is_empty() {
            return None;
        }
        let mut out = BTreeSet::new();
        let xs: Vec<u32> = s.classes.iter().copied().collect();
        for x in xs {
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                return None;
            }
            let t = self.ty(x);
            let cls = self.names[t as usize].to_string();
            if !bases.iter().any(|b| self.h.is_subtype(&cls, b)) {
                return None;
            }
            let cf = self.h.class(&cls)?;
            for mm in &cf.methods {
                for i in mm.code.iter().flat_map(|c| c.insns.iter()) {
                    if let classfile::Operand::Ldc(Const::String(s)) = &i.operand {
                        out.insert(Rc::from(s.as_str()));
                    }
                }
            }
        }
        Some(out)
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

    /// 按名取到的类：其构造器进入反射面（类已被构造器枚举且反射构造可达时立即补入）
    pub(super) fn named_class(&mut self, m: usize, off: u32, cls: &str) {
        let k = self.mirror(cls);
        self.add_to(Node::S(m, off), &TypeSet::exact(k));
        self.init(cls, Via::method("reflect", m, Some(off)));
        let c = self.id(cls);
        if self.named_ctors.insert(c) && self.enumerated.contains(&(Members::Constructors, c)) && self.invokable.contains(&Members::Constructors) {
            self.expose(Members::Constructors, c);
        }
    }
}

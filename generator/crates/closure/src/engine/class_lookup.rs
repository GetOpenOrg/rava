//! 引擎：按名取类——把「常量前缀 + 常量表取值」拼出的类名解析成具体类集。
//!
//! 形状（全部由清单事实定义，分析器不含类名）：
//! - 名字实参是字符串常量，或一条线性的字符串拼接链（`[facts.string_concat]`：新建构建器 → 逐段追加 → 取结果，
//!   每个中间值只被链上下一步使用一次）；各段是字符串常量 / null，或「常量表读取」的结果（可经一次 checkcast）；
//! - 常量表读取：接收者是 `[facts.reflect] constant_tables` 基类的子类对象、调用其读取入口。常量表子类是生成的
//!   不可变映射，内容即子类自身代码里的字符串常量——候选值取该子类全部方法的 ldc 字符串（超集，安全）；
//!   接收者值集另含 open / 非常量表部分时，给出常量表部分的候选，结果另接所指未知的 Class（按名查方法记为任意串）；
//! - 枚举取值：段是枚举常量上 final 字符串字段的平凡取值方法（见 `method_lookup.rs`）；
//! - 按名查方法（`[facts.reflect] method_lookups`）复用同一拆段，推不出的段记为任意串，见 `method_lookup.rs`；
//! - 候选名笛卡尔积后只保留类路径上存在的类；结果唯一地经实例化入口（`instantiators`）再唯一地 checkcast 到 T 时，
//!   只保留 T 的子类型（其余候选在运行时抛 ClassCastException，不产生可观察的成员调用）。
//!
//! 解析成功的调用点结果只含这些类的镜像（不再流入所指未知的 Class），类随之初始化、其构造器进入反射面。

use super::*;

/// 候选名数上限：超出按推不出处理
pub(super) const MAX_NAMES: usize = 4096;

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
        Event::CheckCast(_, v) | Event::InstanceOf(_, v) => v.iter().collect(),
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

/// 链首：新建构建器 s，其用途只有构造器、清空（实参常量 0）与链上第一次使用 p（追加或取结果）各一；
/// 且从 p 出发不经构造器 / 清空点回不到 p（循环复用而不清空时上一轮内容会残留）。返回构造器实参与是否有清空
pub(super) fn builder_head(names: &crate::manifest::NameFacts, a: &Analysis, s: u32, p: u32) -> Option<(Vec<V>, bool)> {
    event_at(a, s, |e| matches!(e, Event::New(_)))?;
    let mut init = None;
    let mut clear = vec![];
    let mut reset = false;
    let mut chain = 0;
    for (uo, e, _) in uses(a, s) {
        let on_s = |args: &[V]| args.first().and_then(site_of) == Some(s);
        match e {
            Event::Invoke { mref, args, .. } if on_s(args) && names.is_builder(&mref.to_string()) && init.is_none() => {
                init = Some(args.clone());
                clear.push(uo);
            }
            Event::Invoke { mref, args, .. } if on_s(args) && names.is_reset(&mref.to_string()) && args.len() == 2 && args[1] == V::Int(0) => {
                reset = true;
                clear.push(uo);
            }
            _ if uo == p => chain += 1,
            _ => return None,
        }
    }
    if chain != 1 || a.cfg.recurs_avoiding(p, &clear) {
        return None;
    }
    Some((init?, reset))
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
        let sticky = self.lookup_top.contains(&(m, off));
        self.lookup_partial = false;
        let r = self.class_lookup_eval(m, off, args);
        let top = sticky || r.is_none() || std::mem::take(&mut self.lookup_partial);
        if top {
            self.lookup_top.insert((m, off));
        }
        (r.unwrap_or_default(), top)
    }

    fn class_lookup_eval(&mut self, m: usize, off: u32, args: &[V]) -> Option<Vec<String>> {
        let a = self.methods[m].analysis.clone()?;
        if a.conservative {
            return None;
        }
        let parts = self.name_parts(Some(m), &a, args.first()?, false, 0)?;
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

    /// 名字值拆成拼接段；wild = 推不出的段记为任意串（否则整体推不出）
    /// m = 值所在方法（None = 被调方法的独立分析，不读常量表：其接收者值集不在引擎里）；
    /// depth = 已穿过的辅助方法层数（名字由唯一目标的辅助方法拼出并返回时，进入其字节码继续拆）
    pub(super) fn name_parts(&mut self, m: Option<usize>, a: &Analysis, v: &V, wild: bool, depth: u8) -> Option<Vec<Part>> {
        if let V::Str(s) = v {
            return Some(vec![Part::Lit(s.clone())]);
        }
        let o = site_of(v)?;
        if let Some(segs) = m.and_then(|m| self.indy_concat_segs(m, a, o)) {
            return self.seg_parts(m, a, &segs, wild, depth);
        }
        let names = &self.man.names;
        let Some(Event::Invoke { mref, args, .. }) = event_at(a, o, is_invoke) else {
            // 非调用结果（如直接读字段）：整体作为一段
            return self.segment_values(m, a, v, false, depth).map(|p| vec![p]);
        };
        if !names.is_result(&mref.to_string()) {
            let (ca, rv) = self.callee_return(a, o, depth)?;
            return self.name_parts(None, &ca, &rv, wild, depth + 1);
        }
        let mut segs: Vec<V> = vec![];
        let mut cur = args.first()?.clone();
        // 使用 cur 的链上事件偏移（取结果 / 追加）
        let mut p = o;
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
                p = s;
                continue;
            }
            let (init, reset) = builder_head(names, a, s, p)?;
            if let Some(init_v) = init.get(1) {
                // 清空与带初始内容的构造并存：追加点的内容可能是两者之一，不拆
                if reset {
                    return None;
                }
                segs.push(init_v.clone());
            }
            break;
        }
        segs.reverse();
        self.seg_parts(m, a, &segs, wild, depth)
    }

    /// 拼接各段的值 → 拼接段
    fn seg_parts(&mut self, m: Option<usize>, a: &Analysis, segs: &[V], wild: bool, depth: u8) -> Option<Vec<Part>> {
        let mut parts = Vec::with_capacity(segs.len());
        for s in segs {
            parts.push(match s {
                V::Str(x) => Part::Lit(x.clone()),
                V::Null => Part::Lit(Rc::from("null")),
                V::Ref { .. } => match self.segment_values(m, a, s, wild, depth) {
                    Some(p) => p,
                    None if wild => Part::Wild,
                    None => return None,
                },
                _ if wild => Part::Wild,
                _ => return None,
            });
        }
        Some(parts)
    }

    /// 站点 o 是方法 m 里的字符串拼接 indy（引导方法为清单 `[facts.indy]` 的 concat 类）时按配方拆出的各段值：
    /// 配方取引导静态实参首项（`\u{1}` = 依次取动态实参、`\u{2}` = 依次取其后的静态常量，其余字符为字面量）；
    /// 无配方的引导（各动态实参直接相接）逐个动态实参成段。静态常量非字符串时不拆
    fn indy_concat_segs(&self, m: usize, a: &Analysis, o: u32) -> Option<Vec<V>> {
        let Event::Indy { bsm, args, .. } = event_at(a, o, |e| matches!(e, Event::Indy { .. }))? else { return None };
        let cf = self.h.class(&self.methods[m].key.owner)?;
        let b = cf.bootstrap_methods.get(*bsm as usize)?;
        let bkey = format!("{}.{}", b.handle.member.owner, b.handle.member.name);
        if self.man.indy_kind(&bkey) != Some(IndyKind::Concat) {
            return None;
        }
        let Some(Const::String(recipe)) = b.args.first() else {
            return b.args.is_empty().then(|| args.clone());
        };
        let mut segs = vec![];
        let mut lit = String::new();
        let (mut dyn_i, mut const_i) = (0, 1);
        let flush = |lit: &mut String, segs: &mut Vec<V>| {
            if !lit.is_empty() {
                segs.push(V::Str(Rc::from(std::mem::take(lit).as_str())));
            }
        };
        for ch in recipe.chars() {
            match ch {
                '\u{1}' => {
                    flush(&mut lit, &mut segs);
                    segs.push(args.get(dyn_i)?.clone());
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

    /// 引用值段：封存静态字段（值映射读取 / 常量字符串数组元素，`sealed.rs`）→ 常量表读取 → 枚举取值 → 返回字符串常量的辅助方法 → 辅助方法拼出的名字（按顺序取第一个成形的）
    fn segment_values(&mut self, m: Option<usize>, a: &Analysis, v: &V, wild: bool, depth: u8) -> Option<Part> {
        if let Some(set) = self.sealed_segment(a, v) {
            return Some(Part::Any(set));
        }
        if let Some((set, partial)) = m.and_then(|m| self.table_values(m, a, v)) {
            // 接收者含非常量表值：按名查方法（wild）记为任意串；按名取类给出常量表部分并记 top
            if !partial {
                return Some(Part::Any(set));
            }
            if wild {
                return Some(Part::Wild);
            }
            self.lookup_partial = true;
            return Some(Part::Any(set));
        }
        if let Some(set) = self.enum_field_values(a, v) {
            return Some(Part::Any(set));
        }
        let o = site_of(v)?;
        if let Some(set) = self.callee_consts(a, o, depth) {
            return Some(Part::Any(set));
        }
        // 辅助方法拼出的段：拍平成一个候选集（含任意串时整体记为任意串）
        let (ca, rv) = self.callee_return(a, o, depth)?;
        let parts = self.name_parts(None, &ca, &rv, wild, depth + 1)?;
        let mut names: Vec<String> = vec![String::new()];
        for p in &parts {
            names = match p {
                Part::Lit(l) => names.into_iter().map(|n| n + l).collect(),
                Part::Any(set) if names.len().saturating_mul(set.len()) <= MAX_NAMES => {
                    names.iter().flat_map(|n| set.iter().map(move |x| format!("{n}{x}"))).collect()
                }
                _ => return wild.then_some(Part::Wild),
            };
        }
        Some(Part::Any(names.into_iter().map(Rc::from).collect()))
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

    /// 按名取到的类：其构造器进入反射面（类已被构造器枚举且反射构造可达时立即补入）；
    /// 数组类只取镜像（同 ldc 数组类常量），不初始化元素类（JLS §12.4.1）
    pub(super) fn named_class(&mut self, m: usize, off: u32, cls: &str) {
        let k = self.mirror(cls);
        self.add_to(Node::S(m, off), &TypeSet::exact(k));
        if cls.starts_with('[') {
            self.touch(cls, Level::Type, Via::method("reflect", m, Some(off)));
            return;
        }
        // 运行期按名取类（`Class.forName(名, true, …)`）经类初始化钩子触发 `<clinit>`：登记为钩子目标，
        // 否则分析上已初始化的类在运行期不跑 `<clinit>`（如 SharedSecrets 惰性访问器所依赖的登记写入）
        self.seeds.mirror_inits.insert(cls.to_string());
        self.init(cls, Via::method("reflect", m, Some(off)));
        let c = self.id(cls);
        if self.named_ctors.insert(c) && self.enumerated.contains(&(Members::Constructors, c)) && self.invokable.contains(&Members::Constructors) {
            self.expose(Members::Constructors, c);
        }
    }
}

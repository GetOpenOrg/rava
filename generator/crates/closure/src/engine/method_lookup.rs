//! 引擎：按名查方法——名字实参是「常量前缀 + 常量表 / 枚举取值」拼接时，解析出目标类上被查到的方法名。
//!
//! 形状（与按名取类共用拆段，见 `class_lookup.rs`）：
//! - 名字实参是一条线性的字符串拼接链；各段是字符串常量、常量表读取、枚举取值，推不出的段记为任意串；
//! - 枚举取值：段是直接读枚举的 final 字符串实例字段，或对 final 枚举类（或 final 方法）上无参实例方法的调用、
//!   方法体恰为「读本对象的该类字段并返回」；且该字段只在构造器里由形参或字符串常量写入。枚举常量只在本类 `<clinit>` 里构造，
//!   构造实参即本类代码里的字符串常量——候选值取该类全部方法的 ldc 字符串（超集）；
//! - 名字或某段由唯一目标的辅助方法给出时进入其字节码：返回值全是字符串常量（可合流）的，候选取该方法的 ldc
//!   字符串；唯一返回值是拼接的，继续拆段（至多 2 层；辅助方法的独立分析不读常量表）；
//! - 目标类取 Class 形参上的类常量，或其值集里类镜像所指的类（如取自 `static final Class` 字段）；
//! - 候选名只保留目标类上实际声明的方法：逐个方法名按拼接段匹配（字面量逐字、候选集任取其一、任意串任意长），
//!   因此不做笛卡尔积；全部段都是任意串（无任何字面量 / 候选集）时推不出，按原样处理（不补方法名）。

use super::class_lookup::{event_at, is_invoke, site_of, Gap, Part};
use super::name_eval::Frame;
use super::*;

/// 平凡取值方法的指令形态：aload_0 / getfield / areturn
const ALOAD: u8 = 0x19;
const ALOAD_0: u8 = 0x2a;
const ALOAD_3: u8 = 0x2d;
const ARETURN: u8 = 0xb0;
const CTOR: &str = "<init>";
/// 穿过辅助方法的最大层数
const MAX_DEPTH: u8 = 2;
/// 辅助方法调用的派发目标数 / 返回值支数上限：超出按推不出处理
const MAX_TARGETS: usize = 8;

/// 名字 s 是否能由拼接段拼出
pub(super) fn parts_match(parts: &[Part], s: &str) -> bool {
    let Some((p, rest)) = parts.split_first() else { return s.is_empty() };
    match p {
        Part::Lit(l) => s.strip_prefix(&**l).is_some_and(|t| parts_match(rest, t)),
        Part::Any(set) => set.iter().any(|l| s.strip_prefix(&**l).is_some_and(|t| parts_match(rest, t))),
        Part::Wild => s.char_indices().map(|(i, _)| i).chain([s.len()]).any(|i| parts_match(rest, &s[i..])),
        Part::Alt(alts) => alts.iter().any(|alt| parts_match(&[alt.as_slice(), rest].concat(), s)),
    }
}

/// 拼接段是否有任何约束（非全是任意串）；多选须每一支都有约束
pub(super) fn constrained(parts: &[Part]) -> bool {
    parts.iter().any(|p| match p {
        Part::Lit(l) => !l.is_empty(),
        Part::Any(_) => true,
        Part::Wild => false,
        Part::Alt(alts) => alts.iter().all(|a| constrained(a)),
    })
}

/// 方法体是否为 `return this.f`；是则返回 f 的名字与描述符
fn getter_field(code: &classfile::Code, owner: &str) -> Option<(String, String)> {
    let [a, g, r] = code.insns.as_slice() else { return None };
    let this = a.opcode == ALOAD_0 || a.opcode == ALOAD && matches!(a.operand, classfile::Operand::Local(0));
    if !this || g.opcode != classfile::op::GETFIELD || r.opcode != ARETURN {
        return None;
    }
    let classfile::Operand::Field(f) = &g.operand else { return None };
    (f.owner == owner).then(|| (f.name.clone(), f.desc.clone()))
}

/// 构造器里对字段 f 的每次写入，写入值都来自形参或字符串常量（写入指令的前一条）
fn ctor_writes_plain(cf: &ClassFile, name: &str, desc: &str) -> bool {
    cf.methods.iter().filter(|mm| mm.name == CTOR).all(|mm| {
        let Some(code) = &mm.code else { return true };
        code.insns.windows(2).all(|w| {
            let hit = w[1].opcode == classfile::op::PUTFIELD
                && matches!(&w[1].operand, classfile::Operand::Field(f) if f.owner == cf.name && f.name == name && f.desc == desc);
            !hit || match (&w[0].opcode, &w[0].operand) {
                (&ALOAD, classfile::Operand::Local(i)) => *i > 0,
                (op, _) if (ALOAD_0 + 1..=ALOAD_3).contains(op) => true,
                (_, classfile::Operand::Ldc(Const::String(_) | Const::StringUtf16(_))) => true,
                _ => false,
            }
        })
    })
}

impl<'a> Engine<'a> {
    /// 调用的 Class 形参上非常量实参的值集中、类镜像所指的类（值集增长时本站点重跑）
    pub(super) fn class_arg_mirrors(&mut self, m: usize, mref: &MemberRef, opcode: u8, args: &[V]) -> Vec<String> {
        let Some(md) = parse_method(&mref.desc) else { return vec![] };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let class = self.id(CLASS);
        let mut out = vec![];
        for (p, a) in md.params.iter().zip(args.iter().skip(skip)) {
            if !matches!(p, FieldType::Object(c) if c == CLASS) || !matches!(a, V::Ref { .. }) {
                continue;
            }
            let fs = self.feeds(m, a, class);
            let s = self.value_set(&fs);
            for x in s.classes.iter() {
                if let Some(&c) = self.mirrors.get(&x) {
                    out.push(self.names[c as usize].to_string());
                }
            }
        }
        out
    }

    /// 按名查方法调用点的 Class 接收者（非常量，如 `this` / 形参 / 字段）值集中类镜像所指的类
    /// （值集增长时本站点重跑）。所指未知的值（open、非镜像的 Class 值）记为反射缺口：
    /// 该查找点在运行时可能按名取到任意类的方法，分析器不做模糊扩展
    pub(super) fn recv_mirrors(&mut self, m: usize, recv: &V) -> Vec<String> {
        if !matches!(recv, V::Ref { .. }) {
            return vec![];
        }
        let class = self.id(CLASS);
        let fs = self.feeds(m, recv, class);
        let s = self.value_set(&fs);
        let mut out = vec![];
        for x in s.classes.iter() {
            match self.mirrors.get(&x) {
                Some(&c) => out.push(self.names[c as usize].to_string()),
                None => {
                    let what = self.names[x as usize].to_string();
                    self.reflect_gaps.insert(format!("{} <- recv({what})", self.methods[m].key));
                }
            }
        }
        for o in &s.open {
            self.reflect_gaps.insert(format!("{} <- recv(open({}))", self.methods[m].key, self.names[o as usize]));
        }
        out
    }

    /// 按名查方法调用点（方法 m）的名字实参 v 拆成的拼接段；None = 形状不符或无任何约束
    pub(super) fn method_name_parts(&mut self, m: usize, v: &V) -> Option<Vec<Part>> {
        // 拆段读本方法其它偏移的事件：登记为跨偏移读者（同 `class_lookup`）
        if let Some((sm, off)) = self.cur_site.filter(|s| s.0 == m) {
            self.xreaders.entry(sm).or_default().insert(off);
        }
        let a = self.methods[m].analysis.clone()?;
        if a.conservative {
            return None;
        }
        let owner = self.methods[m].key.owner.clone();
        let f = Frame { m: Some(m), a: &a, owner: &owner, up: None };
        let parts = self.name_parts(&f, v, Gap::Method, 0)?;
        constrained(&parts).then_some(parts)
    }

    /// 类 cls 上声明的、名字能由拼接段拼出的方法名
    pub(super) fn declared_matching(&self, cls: &str, parts: &[Part]) -> BTreeSet<Rc<str>> {
        let Some(cf) = self.h.class(cls) else { return BTreeSet::new() };
        cf.methods.iter().filter(|mm| !mm.name.starts_with('<') && parts_match(parts, &mm.name)).map(|mm| Rc::from(mm.name.as_str())).collect()
    }

    /// 站点 o 的调用有唯一目标且有字节码时，目标方法的独立分析（不绑定形参、不入引擎）与其所在类；层数超限为 None
    fn callee_analysis(&self, a: &Analysis, o: u32, depth: u8) -> Option<(Rc<Analysis>, String)> {
        if depth >= MAX_DEPTH {
            return None;
        }
        let Event::Invoke { opcode, mref, iface, .. } = event_at(a, o, is_invoke)? else { return None };
        if !mref.desc.ends_with(&format!(")L{STRING};")) {
            return None;
        }
        let (cf, t) = self.ctx.exact_target(*opcode, mref, *iface)?;
        self.target_analysis(&cf, &t)
    }

    /// 站点 o 的调用的各目标及其独立分析：唯一目标，或虚调用按接收者值集逐个选出的实现（接收者须在引擎帧里
    /// 有值集——本帧或经形参逐层上溯；值集含 open、lambda / 手写对象时推不出；值集增长时站点重跑）
    fn callee_targets(&mut self, f: &Frame, o: u32, depth: u8) -> Option<Vec<(Rc<Analysis>, String)>> {
        if depth >= MAX_DEPTH {
            return None;
        }
        let Event::Invoke { opcode, mref, iface, args, .. } = event_at(f.a, o, is_invoke)? else { return None };
        if !mref.desc.ends_with(&format!(")L{STRING};")) {
            return None;
        }
        if let Some((cf, t)) = self.ctx.exact_target(*opcode, mref, *iface) {
            return Some(vec![self.target_analysis(&cf, &t)?]);
        }
        if !matches!(*opcode, classfile::op::INVOKEVIRTUAL | classfile::op::INVOKEINTERFACE) {
            return None;
        }
        let site = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface)?;
        let (rf, rv) = f.resolve(args.first()?);
        let m = rf.m?;
        if !matches!(rv, V::Ref { .. }) {
            return None;
        }
        let owner = self.id(&mref.owner);
        let fs = self.feeds(m, &rv, owner);
        let s = self.value_set(&fs);
        if !s.open.is_empty() || s.classes.is_empty() {
            return None;
        }
        let mut keys: BTreeSet<(String, String, String)> = BTreeSet::new();
        for x in s.classes.iter() {
            if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                return None;
            }
            let t = self.ty(x);
            let sel = self.h.select(&self.names[t as usize], &site)?;
            keys.insert(sel.key());
            if keys.len() > MAX_TARGETS {
                return None;
            }
        }
        let mut out = vec![];
        for (o, n, d) in keys {
            let cf = self.h.class(&o)?;
            out.push(self.target_analysis(&cf, &MemberRef { owner: o, name: n, desc: d })?);
        }
        Some(out)
    }

    /// 目标方法 t（在类 cf 上声明）的独立分析与其所在类；无字节码或分析保守时为 None
    fn target_analysis(&self, cf: &ClassFile, t: &MemberRef) -> Option<(Rc<Analysis>, String)> {
        let meth = cf.method(&t.name, &t.desc)?;
        let code = meth.code.as_ref()?;
        let live = |_: &str| true;
        let ca = absint::analyze(&t.owner, &t.desc, meth.is_static(), code, &Facts { ctx: &self.ctx, live: &live, m: None, params: vec![], mirrors: vec![] });
        // 独立分析读过的字段登记给当前站点所在方法：字段转为不折叠时该方法失效，重分析时按名查找站点重跑
        if let Some((outer, _)) = self.cur_site {
            for (_, e) in &ca.events {
                let Event::Field { opcode, mref, .. } = e else { continue };
                if matches!(*opcode, classfile::op::GETSTATIC | classfile::op::GETFIELD) {
                    if let Some(fi) = self.ctx.field_info(mref) {
                        self.ctx.fdeps.borrow_mut().entry(fi.key.clone()).or_default().insert(outer);
                    }
                }
            }
        }
        (!ca.conservative).then(|| (Rc::new(ca), t.owner.clone()))
    }

    /// 辅助方法各目标的全部返回值（及其分析、所在类）
    pub(super) fn callee_returns(&mut self, f: &Frame, o: u32, depth: u8) -> Option<Vec<(Rc<Analysis>, V, String)>> {
        let mut out = vec![];
        for (ca, owner) in self.callee_targets(f, o, depth)? {
            let rets: Vec<V> = ca.events.iter().filter_map(|(_, e)| match e {
                Event::Return(v) => Some(v.clone()),
                _ => None,
            }).collect();
            if rets.is_empty() {
                return None;
            }
            out.extend(rets.into_iter().map(|v| (ca.clone(), v, owner.clone())));
        }
        (out.len() <= MAX_TARGETS).then_some(out)
    }

    /// 辅助方法的返回值全是字符串常量（可合流）时的候选：该方法字节码里的全部 ldc 字符串（超集）
    pub(super) fn callee_consts(&self, a: &Analysis, o: u32, depth: u8) -> Option<BTreeSet<Rc<str>>> {
        let (ca, _) = self.callee_analysis(a, o, depth)?;
        let mut any = false;
        for (_, e) in &ca.events {
            let Event::Return(v) = e else { continue };
            any = true;
            let plain = match v {
                V::Str(_) => true,
                V::Ref { src, .. } => !src.is_empty() && src.iter().all(|s| matches!(s, Src::Str(_))),
                _ => false,
            };
            if !plain {
                return None;
            }
        }
        if !any {
            return None;
        }
        let Event::Invoke { opcode, mref, iface, .. } = event_at(a, o, is_invoke)? else { return None };
        let (cf, t) = self.ctx.exact_target(*opcode, mref, *iface)?;
        let code = cf.method(&t.name, &t.desc)?.code.as_ref()?;
        Some(code.insns.iter().filter_map(|i| match &i.operand {
            classfile::Operand::Ldc(Const::String(s)) => Some(Rc::from(s.as_str())),
            _ => None,
        }).collect())
    }

    /// 枚举取值段的候选字符串
    pub(super) fn enum_field_values(&self, a: &Analysis, v: &V) -> Option<BTreeSet<Rc<str>>> {
        let o = site_of(v)?;
        // 取值方法调用，或直接读字段
        let (cf, fname, fdesc) = match event_at(a, o, |e| is_invoke(e) || matches!(e, Event::Field { .. }))? {
            Event::Invoke { opcode, mref, iface, args } => {
                if *opcode == classfile::op::INVOKESTATIC || args.len() != 1 {
                    return None;
                }
                let site = self.h.resolve_method(&mref.owner, &mref.name, &mref.desc, *iface)?;
                let cf = site.class.clone();
                let rm = site.method();
                if cf.access & acc::FINAL == 0 && !rm.is_final() {
                    return None;
                }
                let (n, d) = getter_field(rm.code.as_ref()?, &cf.name)?;
                (cf, n, d)
            }
            Event::Field { opcode: classfile::op::GETFIELD, mref, .. } => (self.h.class(&mref.owner)?, mref.name.clone(), mref.desc.clone()),
            _ => return None,
        };
        if cf.access & acc::ENUM == 0 {
            return None;
        }
        let f = cf.field(&fname, &fdesc)?;
        if f.is_static() || f.access & acc::FINAL == 0 || fdesc != format!("L{STRING};") || !ctor_writes_plain(&cf, &fname, &fdesc) {
            return None;
        }
        let mut out = BTreeSet::new();
        for i in cf.methods.iter().flat_map(|mm| mm.code.iter().flat_map(|c| c.insns.iter())) {
            if let classfile::Operand::Ldc(Const::String(s)) = &i.operand {
                out.insert(Rc::from(s.as_str()));
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit(s: &str) -> Part {
        Part::Lit(Rc::from(s))
    }

    fn any(xs: &[&str]) -> Part {
        Part::Any(xs.iter().map(|s| Rc::from(*s)).collect())
    }

    #[test]
    fn prefix_and_enum_values_pick_declared_names() {
        let parts = [lit("box"), any(&["Integer", "Long", "INT", "int"])];
        assert!(parts_match(&parts, "boxInteger"));
        assert!(parts_match(&parts, "boxLong"));
        assert!(!parts_match(&parts, "boxExact"));
        assert!(!parts_match(&parts, "boxType"));
        assert!(!parts_match(&parts, "unboxInteger"));
    }

    #[test]
    fn wild_segments_match_by_skeleton() {
        let parts = [Part::Wild, lit("To"), Part::Wild];
        assert!(parts_match(&parts, "intToLong"));
        assert!(parts_match(&parts, "To"));
        assert!(!parts_match(&parts, "boxType"));
        assert!(constrained(&parts));
        assert!(!constrained(&[Part::Wild, lit(""), Part::Wild]));
    }

    /// 多选（辅助方法的各返回值）：任一支匹配即可；展开成不含多选的候选模式，超出上限为 None
    #[test]
    fn alternatives_match_and_expand() {
        use super::super::class_lookup::expand;
        let alt = Part::Alt(vec![vec![lit("L")], vec![lit("I"), Part::Wild]]);
        let parts = [alt.clone(), any(&["0", "1"])];
        assert!(parts_match(&parts, "L0") && parts_match(&parts, "Ix1") && !parts_match(&parts, "J0"));
        assert!(constrained(&parts));
        assert!(!constrained(&[Part::Alt(vec![vec![lit("a")], vec![Part::Wild]])]));
        let pats = expand(&[lit("p"), alt]).expect("未超上限");
        assert_eq!(pats, vec![vec![lit("p"), lit("L")], vec![lit("p"), lit("I"), Part::Wild]]);
        let wide = Part::Alt((0..9).map(|i| vec![lit(&i.to_string())]).collect());
        assert!(expand(&[wide.clone(), wide]).is_none());
    }

    #[test]
    fn empty_candidates_match_nothing() {
        let parts = [lit("box"), any(&[])];
        assert!(!parts_match(&parts, "boxInteger"));
        assert!(constrained(&parts));
    }
}

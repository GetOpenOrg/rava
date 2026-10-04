//! 引擎：按名查字段——名字经值流到达「Class 形参 / 接收者 + String 形参」调用（`getDeclaredField`、
//! `findStaticVarHandle`、`objectFieldOffset(Class, String)` 等；清单 `method_lookups` 的按名查方法除外）的名字实参时，
//! 点名目标类（含超类型）上声明的该名字段，发射层据此生成按名字段臂。
//!
//! - 名字：String 形参上的字面量（含合流前的各字面量）、形参透传的各调用点常量、读自 String 字段时该字段各写入处的
//!   常量（字段可被字节码外写入或有非常量写入时不给出）、拼接链 / 拼接 indy 拆出的段（按目标类上的字段名反向匹配，
//!   同按名查方法）；
//! - 目标类：名字实参之前的 Class 常量实参、Class 形参与接收者值集里类镜像所指的类（名字之后的 Class 是字段类型）；值集含所指未知的 Class（open、非镜像值）时
//!   目标类推不出：字面量名按名字点名（任意类的同名字段），拼接名记为反射缺口。
//! - 查到的静态字段：声明类按静态字段句柄处理（初始化并记为句柄 / 反射链接目标，`static_field_owner`），覆盖
//!   `findStaticGetter` 等只读入口（按名写字段的 `name_resolvers` 只列写入口）。

use super::class_lookup::{event_at, Part};
use super::sealed::is_field;
use super::method_lookup::parts_match;
use super::*;

impl<'a> Engine<'a> {
    pub(super) fn field_lookup(&mut self, m: usize, off: u32, mref: &MemberRef, opcode: u8, args: &[V], classes: &[String], class_recv: bool) {
        let Some(md) = parse_method(&mref.desc) else { return };
        let skip = usize::from(opcode != classfile::op::INVOKESTATIC);
        let mut names: BTreeSet<Rc<str>> = BTreeSet::new();
        let mut patterns: Vec<Vec<Part>> = vec![];
        // 目标类实参：接收者，与名字实参之前的 Class 实参（JDK 按名查字段的签名约定「声明类, 名字, 字段类型」）：
        // 名字之后的 Class 是字段类型（`findGetter(refc, name, type)`、`findStaticVarHandle(decl, name, type)`），
        // 不是查找目标，其值集推不出不构成目标缺口
        let mut tvals: Vec<&V> = vec![];
        if class_recv {
            tvals.extend(args.first());
        }
        let mut named = false;
        for (p, a) in md.params.iter().zip(args.iter().skip(skip)) {
            match p {
                FieldType::Object(c) if c == CLASS && !named => tvals.push(a),
                FieldType::Object(c) if c == absint::STRING => {
                    named = true;
                    names.extend(a.lits());
                    // 常量格给出的名字另按其来源求值（同常量格推不出时，见 `V::Str`）
                    if matches!(a, V::Ref { .. }) || a.derived_str() {
                        names.extend(self.param_strs(m, off, a));
                        names.extend(self.field_strs(m, a));
                        if let Some(parts) = self.method_name_parts(m, a) {
                            patterns.push(parts);
                        }
                    }
                }
                _ => {}
            }
        }
        // 查找结果只依赖（目标类, 名字 / 拼接段），效果全部幂等累加：名字与拼接段同上次时只按新增的目标类值查，
        // 否则整体重查（站点重跑由 Class 值集增长驱动，逐次全量换算类名是平方开销）
        let mut seen = self.refl_seen.entry(m).or_default().remove(&off).unwrap_or_default();
        let mut lk = match seen.lookup.take() {
            Some(lk) if lk.names == names && lk.patterns == patterns => lk,
            _ => LookupSeen { names: names.clone(), patterns: patterns.clone(), ..LookupSeen::default() },
        };
        let mut targets: BTreeSet<String> = classes.iter().cloned().collect();
        let mut unknown = lk.unknown;
        for v in tvals {
            unknown |= self.class_values_new(m, v, &mut lk.vals, &mut targets);
        }
        lk.unknown = unknown;
        seen.lookup = Some(lk);
        self.refl_seen.entry(m).or_default().insert(off, seen);
        let mut found: Vec<(String, String)> = vec![];
        for c in &targets {
            for name in &names {
                if let Some((decl, _)) = self.field_by_name(c, name) {
                    found.push((decl, name.to_string()));
                }
            }
            for parts in &patterns {
                found.extend(self.fields_matching(c, parts));
            }
        }
        // 查到的静态字段经句柄 / 反射访问（getStatic / putStatic 句柄、Field.get / set）时声明类初始化
        // （JVMS §5.5；findStaticGetter 这类只读入口不在按名写字段的 name_resolvers 里，由此覆盖）
        let via = Via::method("field-name", m, Some(off));
        for (decl, name) in found {
            let is_static = self.h.class(&decl).is_some_and(|cf| cf.fields.iter().any(|f| f.name == name && f.is_static()));
            if is_static {
                self.static_field_owner(&decl, via.clone());
            }
            self.reflect_fields.insert((decl, name));
        }
        if unknown {
            self.reflect_field_names.extend(names.iter().map(|n| n.to_string()));
            if !patterns.is_empty() {
                self.reflect_gaps.insert(format!("{} <- 按名查字段：目标类推不出、名字为拼接", self.methods[m].key));
            }
        }
    }

    /// String 字段写入值并入该字段的常量集（null 不计；非常量写入置为推不出）；变化时读者失效
    pub(super) fn field_strs_put(&mut self, key: &MemberRef, v: Option<&V>) {
        if key.desc != format!("L{};", absint::STRING) {
            return;
        }
        let add: Option<Vec<Rc<str>>> = match v {
            Some(V::Null) => Some(vec![]),
            Some(V::Str(s, _)) => Some(vec![s.clone()]),
            Some(r @ V::Ref { src, .. }) if !src.is_empty() && src.iter().all(|s| matches!(s, Src::Str(_))) => Some(r.lits()),
            _ => None,
        };
        let cur = self.field_strs.entry(key.clone()).or_insert_with(|| Some(BTreeSet::new()));
        let changed = match (cur.as_mut(), add) {
            (None, _) => false,
            (Some(_), None) => {
                *cur = None;
                true
            }
            (Some(set), Some(xs)) => {
                let n = set.len();
                set.extend(xs);
                set.len() != n
            }
        };
        if changed {
            let deps = self.ctx.fdeps.borrow().get(key).cloned();
            self.invalidate_all(deps, Why::FieldPut);
        }
    }

    /// 值 v 读自 String 字段时，该字段各写入处的字符串常量（字段不折叠——可被字节码外写入——或有非常量写入时不给出）
    pub(super) fn field_strs(&mut self, m: usize, v: &V) -> Vec<Rc<str>> {
        let Some(a) = self.methods[m].analysis.clone() else { return vec![] };
        let mut out = vec![];
        for s in v.srcs().iter() {
            let Src::Site(o) = *s else { continue };
            let Some(Event::Field { opcode, mref, .. }) = event_at(&a, o, is_field) else { continue };
            if !matches!(*opcode, classfile::op::GETSTATIC | classfile::op::GETFIELD) {
                continue;
            }
            let Some(fi) = self.ctx.field_info(mref) else { continue };
            if self.ctx.field_open(&fi) {
                continue;
            }
            if let Some(Some(set)) = self.field_strs.get(&fi.key) {
                out.extend(set.iter().cloned());
            }
        }
        out
    }

    /// Class 值 v 所指的类并入 out（常量直接取；引用值取值集里的类镜像，值集增长时本站点重跑）；
    /// 返回值集是否含所指未知的 Class（非字节码类镜像与基本类型类镜像无 Java 字段，不并入、不算未知）
    pub(super) fn class_values(&mut self, m: usize, v: &V, out: &mut BTreeSet<String>) -> bool {
        match v {
            V::Class(c, _) => {
                out.insert(c.to_string());
                false
            }
            V::Ref { .. } => {
                let class = self.id(CLASS);
                let fs = self.feeds(m, v, class);
                let s = self.value_set(&fs);
                let mut unknown = !s.open.is_empty();
                for x in s.classes.iter() {
                    match self.mirrors.get(&x) {
                        Some(&c) => {
                            out.insert(self.names[c as usize].to_string());
                        }
                        // 非字节码类镜像、基本类型类镜像都没有 Java 字段
                        None if Some(x) == self.synth_mirror || Some(x) == self.prim_mirror => {}
                        None => unknown = true,
                    }
                }
                unknown
            }
            V::Null => false,
            _ => true,
        }
    }

    /// 同 [`Self::class_values`]，只取 v 的值集中不在 seen 里的部分（处理后并入 seen；常量每次给出），
    /// 返回这部分是否含所指未知的 Class
    pub(super) fn class_values_new(&mut self, m: usize, v: &V, seen: &mut TypeSet, out: &mut BTreeSet<String>) -> bool {
        let V::Ref { .. } = v else { return self.class_values(m, v, out) };
        let class = self.id(CLASS);
        let fs = self.feeds(m, v, class);
        let s = self.value_set(&fs);
        let delta = TypeSet { classes: s.classes.minus(&seen.classes), open: s.open.minus(&seen.open) };
        seen.add_all(&delta);
        let mut unknown = !delta.open.is_empty();
        for x in delta.classes.iter() {
            match self.mirrors.get(&x) {
                Some(&c) => {
                    out.insert(self.names[c as usize].to_string());
                }
                None if Some(x) == self.synth_mirror || Some(x) == self.prim_mirror => {}
                None => unknown = true,
            }
        }
        unknown
    }

    /// 类 cls 及其超类型上声明的、名字能由拼接段拼出的字段 → (声明类, 名字)
    fn fields_matching(&self, cls: &str, parts: &[Part]) -> Vec<(String, String)> {
        let mut out = vec![];
        let mut stack = vec![cls.to_string()];
        let mut seen = BTreeSet::new();
        while let Some(c) = stack.pop() {
            if !seen.insert(c.clone()) {
                continue;
            }
            let Some(cf) = self.h.class(&c) else { continue };
            out.extend(cf.fields.iter().filter(|f| parts_match(parts, &f.name)).map(|f| (c.clone(), f.name.clone())));
            stack.extend(cf.super_name.iter().cloned().chain(cf.interfaces.iter().cloned()));
        }
        out
    }
}

/// 反射调用点已处理过的 Class 实参值（同一分析结果下成立，重分析按偏移作废）
#[derive(Default)]
pub(super) struct ReflSeen {
    /// 字段枚举入口：接收者值集中已逐类放开的部分
    pub(super) fenum: TypeSet,
    /// 按名查字段
    lookup: Option<LookupSeen>,
}

/// 按名查字段：已按（名字, 拼接段）查过的目标类值，及其中是否出现过所指未知的 Class
#[derive(Default)]
struct LookupSeen {
    names: BTreeSet<Rc<str>>,
    patterns: Vec<Vec<Part>>,
    vals: TypeSet,
    unknown: bool,
}

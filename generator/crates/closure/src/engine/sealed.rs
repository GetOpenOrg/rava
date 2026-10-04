//! 引擎：封存的静态字段取值——按名取类拆段的两种来源（计划 boundary-narrowing §6.11 项 9）。
//!
//! - **值映射字段**：private static 字段，嵌套（宿主与全部成员）内每次写入都是「新建 `value_maps` 实现类 → 无内容构造
//!   → 若干次 writers 写入 → putstatic 本字段」（或写 null），每次读取的值只作为 readers 的接收者。映射对象不逃逸，
//!   读取结果只能是某次写入的值（或 null）：候选 = 各写入值按拆段规则求得的字符串集的并。
//! - **常量字符串数组字段**：private static 的 `String[]` 字段，每次写入是新建数组 → 元素只存字符串常量 / null → putstatic，
//!   每次读取的值只作为数组元素读取的数组。读取下标奇偶已知时只取同奇偶下标（或下标未知）处存入的常量。
//!
//! 嵌套内的访问方法用不读事实的 Oracle 分析：结果只取决于字节码，与处理次序无关（字段为 private，嵌套外无字节码访问；
//! 反射 / Unsafe 写 private static 字段不在建模范围，与常量折叠的前提相同）。任一条件不满足即不给出候选。

use super::class_lookup::{event_at, is_invoke, site_of, uses, Gap, Part, MAX_NAMES};
use super::name_eval::Frame;
use super::*;
use classfile::op::{GETSTATIC, INVOKESTATIC, PUTSTATIC};

/// 不读任何事实的 Oracle：调用结果未知、字段不折叠、类型全部存活
pub(super) struct Plain;

impl Oracle for Plain {
    fn invoke_result(&self, _: u8, _: &MemberRef, _: bool, _: &[V]) -> Ret {
        Ret::Unknown
    }
    fn field(&self, _: u8, _: &MemberRef, _: Option<&V>) -> Option<V> {
        None
    }
    fn type_live(&self, _: &str) -> bool {
        true
    }
}

/// 值是否（可能）来自站点 n
fn mentions(v: &V, n: u32) -> bool {
    matches!(v, V::Ref { src, .. } if src.contains(&Src::Site(n)))
}

pub(super) fn is_field(e: &Event) -> bool {
    matches!(e, Event::Field { .. })
}

/// 实参（不含接收者）都不是引用值：构造不带入内容
fn no_ref_args(args: &[V]) -> bool {
    args.iter().skip(1).all(|x| !matches!(x, V::Ref { .. } | V::Str(..) | V::Class(..) | V::Null))
}

/// 字段 f 在一个访问方法里的读写：`reads` 为 getstatic 偏移，`writes` 为 putstatic 的写入值
pub(super) struct Access {
    pub a: Analysis,
    /// 访问方法所在类（取引导方法表）
    pub owner: String,
    pub reads: Vec<u32>,
    pub writes: Vec<V>,
}

/// 值映射写入：新建映射对象 n 的使用只有无内容构造、以其为接收者的写入入口、putstatic f；返回写入的值
pub(super) fn map_writes(a: &Analysis, n: u32, f: &MemberRef, maps: &crate::manifest::ValueMaps) -> Option<Vec<V>> {
    let Event::New(cls) = event_at(a, n, |e| matches!(e, Event::New(_)))? else { return None };
    if !maps.classes.contains(cls) {
        return None;
    }
    let mut vals = vec![];
    let mut inited = false;
    for (_, e, _) in uses(a, n) {
        match e {
            Event::Invoke { mref, args, .. } if args.first().and_then(site_of) == Some(n) && args.iter().skip(1).all(|x| !mentions(x, n)) => {
                if mref.name == "<init>" && mref.owner == *cls && no_ref_args(args) && !inited {
                    inited = true;
                } else if maps.writers.contains(&format!("{}:{}", mref.name, mref.desc)) && args.len() == 3 {
                    vals.push(args[2].clone());
                } else {
                    return None;
                }
            }
            Event::Field { opcode: PUTSTATIC, mref, value: Some(v), .. } if mref == f && site_of(v) == Some(n) => {}
            _ => return None,
        }
    }
    inited.then_some(vals)
}

/// 常量字符串数组写入：新建数组 n 的使用只有以其为数组的元素存储（值为字符串常量 / null）、元素读取、putstatic f
/// （f 为 None：方法内局部数组，不逃出本方法）；返回（存入下标, 常量）
pub(super) fn array_writes(a: &Analysis, n: u32, f: Option<&MemberRef>) -> Option<Vec<(V, Rc<str>)>> {
    event_at(a, n, |e| matches!(e, Event::NewArray(..)))?;
    let mut out = vec![];
    for (_, e, _) in uses(a, n) {
        match e {
            Event::ArrayStore { array, index, value } if site_of(array) == Some(n) && !mentions(index, n) && !mentions(value, n) => match value {
                V::Str(s, _) => out.push((index.clone(), s.clone())),
                V::Null => {}
                _ => return None,
            },
            Event::ArrayLoad { array, index } if site_of(array) == Some(n) && !mentions(index, n) => {}
            Event::Field { opcode: PUTSTATIC, mref, value: Some(v), .. } if Some(mref) == f && site_of(v) == Some(n) => {}
            _ => return None,
        }
    }
    Some(out)
}

/// 候选段拍平为字符串集（超出上限或含任意串时推不出）
pub(super) fn flatten(parts: &[Part]) -> Option<BTreeSet<Rc<str>>> {
    let mut names: Vec<String> = vec![String::new()];
    for p in parts {
        let alts;
        let set = match p {
            Part::Lit(l) => {
                names.iter_mut().for_each(|n| n.push_str(l));
                continue;
            }
            Part::Any(set) => set,
            Part::Alt(a) => {
                let mut u = BTreeSet::new();
                for x in a {
                    u.extend(flatten(x)?);
                }
                alts = u;
                &alts
            }
            Part::Wild => return None,
        };
        if names.len().saturating_mul(set.len()) > MAX_NAMES {
            super::stats::cap_hit(super::stats::CAP_SEALED);
            return None;
        }
        names = names.iter().flat_map(|n| set.iter().map(move |x| format!("{n}{x}"))).collect();
    }
    Some(names.into_iter().map(Rc::from).collect())
}

impl<'a> Engine<'a> {
    /// private static 字段 f 在其嵌套内的全部访问（按不读事实的 Oracle 分析）；非 private static、
    /// 嵌套成员缺失或访问方法无法建模时为 None
    fn nest_accesses(&self, f: &MemberRef) -> Option<Vec<Access>> {
        let cf = self.h.class(&f.owner)?;
        let fd = cf.field(&f.name, &f.desc)?;
        if !fd.is_static() || fd.access & acc::PRIVATE == 0 {
            return None;
        }
        let host = cf.nest_host.clone().unwrap_or_else(|| cf.name.clone());
        let hcf = self.h.class(&host)?;
        let mut out = vec![];
        for c in std::iter::once(&host).chain(hcf.nest_members.iter()) {
            let cf = self.h.class(c)?;
            for mm in &cf.methods {
                let Some(code) = mm.code.as_ref() else { continue };
                let hits = code.insns.iter().any(|i| matches!(&i.operand, classfile::Operand::Field(r) if r == f));
                if !hits {
                    continue;
                }
                let a = absint::analyze(&cf.name, &mm.desc, mm.is_static(), code, &Plain);
                if a.conservative {
                    return None;
                }
                let mut acc = Access { a, owner: cf.name.clone(), reads: vec![], writes: vec![] };
                for i in &code.insns {
                    if !matches!(&i.operand, classfile::Operand::Field(r) if r == f) {
                        continue;
                    }
                    // 不可达的访问没有事件
                    let Some(Event::Field { opcode, value, .. }) = event_at(&acc.a, i.offset, is_field) else { continue };
                    match (*opcode, value) {
                        (GETSTATIC, _) => acc.reads.push(i.offset),
                        (PUTSTATIC, Some(v)) => acc.writes.push(v.clone()),
                        _ => return None,
                    }
                }
                out.push(acc);
            }
        }
        Some(out)
    }

    /// 按名取类拆段：值（可经一次 checkcast）是封存值映射字段的读取结果，或常量字符串数组（字段 / 不逃出本方法的局部数组）的元素
    pub(super) fn sealed_segment(&mut self, a: &Analysis, v: &V) -> Option<BTreeSet<Rc<str>>> {
        let mut o = site_of(v)?;
        if let Some(Event::CheckCast(_, Some(inner))) = event_at(a, o, |e| matches!(e, Event::CheckCast(..))) {
            o = site_of(inner)?;
        }
        match event_at(a, o, |e| is_invoke(e) || matches!(e, Event::ArrayLoad { .. }))? {
            Event::Invoke { opcode, mref, args, .. } => {
                let sig = format!("{}:{}", mref.name, mref.desc);
                if *opcode == INVOKESTATIC || !self.man.names.value_maps().readers.contains(&sig) {
                    return None;
                }
                let f = self.static_read(a, args.first()?)?;
                self.map_values(&f)
            }
            Event::ArrayLoad { array, index } => {
                let n = site_of(array)?;
                if event_at(a, n, |e| matches!(e, Event::NewArray(..))).is_some() {
                    return Some(array_writes(a, n, None)?.into_iter().map(|(_, s)| s).collect());
                }
                let f = self.static_read(a, array)?;
                if f.desc != format!("[L{};", absint::STRING) {
                    return None;
                }
                self.array_values(&f, index.parity())
            }
            _ => None,
        }
    }

    /// 值是 getstatic 的结果：所读字段
    fn static_read(&self, a: &Analysis, v: &V) -> Option<MemberRef> {
        match event_at(a, site_of(v)?, is_field)? {
            Event::Field { opcode: GETSTATIC, mref, .. } => Some(mref.clone()),
            _ => None,
        }
    }

    /// 封存值映射字段的读取结果候选
    fn map_values(&mut self, f: &MemberRef) -> Option<BTreeSet<Rc<str>>> {
        let accs = self.nest_accesses(f)?;
        let maps = self.man.names.value_maps();
        let mut vals: Vec<(usize, V)> = vec![];
        for (i, acc) in accs.iter().enumerate() {
            for &g in &acc.reads {
                // 只作读取入口的接收者：值不出现在实参位置
                let ok = uses(&acc.a, g).iter().all(|(_, e, _)| match e {
                    Event::Invoke { opcode, mref, args, .. } => {
                        *opcode != INVOKESTATIC
                            && maps.readers.contains(&format!("{}:{}", mref.name, mref.desc))
                            && args.first().and_then(site_of) == Some(g)
                            && args.iter().skip(1).all(|x| !mentions(x, g))
                    }
                    _ => false,
                });
                if !ok {
                    return None;
                }
            }
            for w in &acc.writes {
                if *w == V::Null {
                    continue;
                }
                let n = site_of(w)?;
                vals.extend(map_writes(&acc.a, n, f, maps)?.into_iter().map(|v| (i, v)));
            }
        }
        let mut out = BTreeSet::new();
        for (i, v) in vals {
            if v == V::Null {
                continue;
            }
            let f = Frame { m: None, a: &accs[i].a, owner: &accs[i].owner, up: None };
            let parts = self.name_parts(&f, &v, Gap::Fail, 0)?;
            out.extend(flatten(&parts)?);
        }
        Some(out)
    }

    /// 常量字符串数组字段在下标奇偶 `parity`（None = 未知）处的元素候选
    fn array_values(&self, f: &MemberRef, parity: Option<bool>) -> Option<BTreeSet<Rc<str>>> {
        let accs = self.nest_accesses(f)?;
        let mut out = BTreeSet::new();
        for acc in &accs {
            for &g in &acc.reads {
                let ok = uses(&acc.a, g).iter().all(|(_, e, _)| matches!(e, Event::ArrayLoad { array, index } if site_of(array) == Some(g) && !mentions(index, g)));
                if !ok {
                    return None;
                }
            }
            for w in &acc.writes {
                if *w == V::Null {
                    continue;
                }
                for (index, s) in array_writes(&acc.a, site_of(w)?, Some(f))? {
                    if parity.is_none() || index.parity().is_none() || index.parity() == parity {
                        out.insert(s);
                    }
                }
            }
        }
        Some(out)
    }
}

#[cfg(test)]
mod tests;

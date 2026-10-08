//! 引擎：封存的静态字段取值——按名取类拆段的两种来源（计划 boundary-narrowing §6.11 项 9）。
//!
//! - **值映射字段**：private 字段（静态或实例），嵌套（宿主与全部成员）内每次写字段都是「新建 `value_maps` 实现类 →
//!   无内容构造 → 若干次 writers 写入 → putstatic / putfield 本字段」（或写 null），每次读字段的值只作为 readers /
//!   writers / keeps 的接收者，或传给只以该形参为 readers / keeps 接收者的静态 / 私有辅助方法。映射对象不逃逸，
//!   读取结果只能是某次写入的值（或 null）：候选 = 新建点各写入值按拆段规则求得的字符串集 ∪ 经读字段写入的值
//!   （引擎在写入点登记到值映射槽，见 `map_slot.rs`）。
//! - **常量字符串数组字段**：private static 的 `String[]` 字段，每次写入是新建数组 → 元素只存字符串常量 / null → putstatic，
//!   每次读取的值只作为数组元素读取的数组。读取下标奇偶已知时只取同奇偶下标（或下标未知）处存入的常量。
//!
//! 嵌套内的访问方法用不读事实的 Oracle 分析：结果只取决于字节码，与处理次序无关（字段为 private，嵌套外无字节码访问；
//! 反射 / Unsafe 写 private static 字段不在建模范围，与常量折叠的前提相同）。任一条件不满足即不给出候选。

use super::class_lookup::{event_at, event_values, site_of, uses, Part, MAX_NAMES};
use super::*;
use classfile::op::{GETFIELD, GETSTATIC, INVOKESPECIAL, INVOKESTATIC, PUTFIELD, PUTSTATIC};

/// 不读任何事实的 Oracle：调用结果未知、字段不折叠、类型全部存活
pub(super) struct Plain;

impl Oracle for Plain {
    fn invoke_result(&self, _: u8, _: u32, _: &MemberRef, _: bool, _: &[V]) -> Ret {
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

/// 字段 f 在一个访问方法里的读写：`reads` 为 getstatic / getfield 偏移，`writes` 为 putstatic / putfield 的写入值
pub(super) struct Access {
    pub a: Analysis,
    /// 访问方法所在类（取引导方法表）
    pub owner: String,
    pub reads: Vec<u32>,
    pub writes: Vec<V>,
}

/// 值映射写入：新建映射对象 n 的使用只有无内容构造（无引用实参，或 `empty_ctors` 所列）、以其为接收者的写入入口、
/// putstatic / putfield f；返回写入的值
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
                let empty = no_ref_args(args) || maps.empty_ctors.contains(&format!("{}:{}", mref.name, mref.desc));
                if mref.name == "<init>" && mref.owner == *cls && empty && !inited {
                    inited = true;
                } else if maps.writers.contains(&format!("{}:{}", mref.name, mref.desc)) && args.len() == 3 {
                    vals.push(args[2].clone());
                } else {
                    return None;
                }
            }
            Event::Field { opcode: PUTSTATIC | PUTFIELD, mref, value: Some(v), .. } if mref == f && site_of(v) == Some(n) => {}
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
    /// private 字段 f（静态或实例）在其嵌套内的全部访问（按不读事实的 Oracle 分析）；非 private、
    /// 嵌套成员缺失或访问方法无法建模时为 None
    pub(super) fn nest_accesses(&self, f: &MemberRef) -> Option<Vec<Access>> {
        let cf = self.h.class(&f.owner)?;
        let fd = cf.field(&f.name, &f.desc)?;
        if fd.access & acc::PRIVATE == 0 {
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
                        (GETSTATIC | GETFIELD, _) => acc.reads.push(i.offset),
                        (PUTSTATIC | PUTFIELD, Some(v)) => acc.writes.push(v.clone()),
                        _ => return None,
                    }
                }
                out.push(acc);
            }
        }
        Some(out)
    }

    /// 按名取类拆段：值是常量字符串数组（字段 / 不逃出本方法的局部数组）的元素（值映射字段的读取见 `map_slot.rs`）
    pub(super) fn sealed_segment(&mut self, a: &Analysis, v: &V) -> Option<BTreeSet<Rc<str>>> {
        let o = site_of(v)?;
        match event_at(a, o, |e| matches!(e, Event::ArrayLoad { .. }))? {
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
    pub(super) fn static_read(&self, a: &Analysis, v: &V) -> Option<MemberRef> {
        match event_at(a, site_of(v)?, is_field)? {
            Event::Field { opcode: GETSTATIC, mref, .. } => Some(mref.clone()),
            _ => None,
        }
    }

    /// 值映射字段 f 的封存判定：嵌套内每次读字段的值只作为 readers / writers / keeps 的接收者（或传给只以其为
    /// readers / keeps 接收者的静态 / 私有辅助方法），每次写字段都是新建点写入（`map_writes`）或 null。
    /// 返回新建点写入的值（访问序号, 值）与是否有经读字段的写入（其写入值由引擎登记到值映射槽）
    pub(super) fn map_seal(&self, f: &MemberRef) -> Option<(Vec<Access>, Vec<(usize, V)>, bool)> {
        let accs = self.nest_accesses(f)?;
        let maps = self.man.names.value_maps();
        let mut vals: Vec<(usize, V)> = vec![];
        let mut read_writes = false;
        for (i, acc) in accs.iter().enumerate() {
            for &g in &acc.reads {
                for (_, e, _) in uses(&acc.a, g) {
                    let Event::Invoke { opcode, mref, args, .. } = e else { return None };
                    let sig = format!("{}:{}", mref.name, mref.desc);
                    let recv = *opcode != INVOKESTATIC && args.first().and_then(site_of) == Some(g) && args.iter().skip(1).all(|x| !mentions(x, g));
                    if recv && (maps.readers.contains(&sig) || maps.keeps.contains(&sig)) {
                        continue;
                    }
                    if recv && maps.writers.contains(&sig) && args.len() == 3 {
                        read_writes = true;
                        continue;
                    }
                    // 传给辅助方法：实参恰为该值（不与其它来源合流），被调方只以对应形参为 readers / keeps 接收者
                    let helper = *opcode == INVOKESTATIC || (*opcode == INVOKESPECIAL && mref.name != "<init>");
                    let at: Vec<usize> = args.iter().enumerate().filter(|(_, x)| mentions(x, g)).map(|(j, _)| j).collect();
                    if !helper || at.iter().any(|&j| site_of(&args[j]) != Some(g)) || !self.helper_keeps(mref, &at) {
                        return None;
                    }
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
        Some((accs, vals, read_writes))
    }

    /// 辅助方法 mref（静态 / 私有，有字节码）的形参 ps（与实参序号对齐，实例方法 0 = this）只作 readers / keeps 的接收者
    fn helper_keeps(&self, mref: &MemberRef, ps: &[usize]) -> bool {
        let Some(cf) = self.h.class(&mref.owner) else { return false };
        let Some(mm) = cf.method(&mref.name, &mref.desc) else { return false };
        if !mm.is_static() && !mm.is_private() {
            return false;
        }
        let Some(code) = mm.code.as_ref() else { return false };
        let a = absint::analyze(&cf.name, &mm.desc, mm.is_static(), code, &Plain);
        if a.conservative {
            return false;
        }
        let maps = self.man.names.value_maps();
        ps.iter().all(|&p| {
            let p = Src::Param(p as u16);
            let has = |x: &V| matches!(x, V::Ref { src, .. } if src.contains(&p));
            let only = |x: &V| matches!(x, V::Ref { src, .. } if src[..] == [p]);
            a.events.iter().all(|(_, e)| match e {
                Event::Invoke { opcode, mref, args, .. } if args.iter().any(has) => {
                    let sig = format!("{}:{}", mref.name, mref.desc);
                    *opcode != INVOKESTATIC
                        && args.first().is_some_and(only)
                        && args.iter().skip(1).all(|x| !has(x))
                        && (maps.readers.contains(&sig) || maps.keeps.contains(&sig))
                }
                Event::Invoke { .. } => true,
                // 形参的其余使用（存字段 / 数组、返回、抛出、合流后另作他用）一概不认
                e => !event_values(e).into_iter().any(has),
            })
        })
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

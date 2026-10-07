//! 引擎：按名取类的名字求值——合流值与取后缀。
//!
//! - 合流值：名字值有多个来源（分支各自赋值后汇合，如「按需截掉包名」），各来源各成一支（`Part::Alt`）：
//!   字面量来源成字面量，其余来源按单来源值递归拆段；拆不出的支按 gap 处理（记为任意串或整体推不出）。
//!   合流嵌套层数有上限（值经循环回到自身时必经合流，层数上限即保证求值终止）；
//! - 取后缀（`[facts.string_concat] suffixes`，`String.substring(int)` 语义）：结果是接收者的某个后缀。接收者的
//!   候选能拍平成名字集时，起点为常量取该后缀，否则取各候选的全部后缀（超集，安全：候选名笛卡尔积后只保留
//!   类路径上存在的类）；接收者含任意串时按 gap 处理。

use super::class_lookup::{event_at, is_invoke, Gap, Part, MAX_NAMES};
use super::name_eval::Frame;
use super::sealed::flatten;
use super::*;

/// 合流值拆支的嵌套层数上限
const MAX_PHI_NEST: u8 = 2;

impl<'a> Engine<'a> {
    /// 多来源引用值的各来源拆成的支；None = 非多来源值（调用方照常处理）；Some(None) = 推不出
    pub(super) fn phi_parts(&mut self, f: &Frame, v: &V, gap: Gap, depth: u8) -> Option<Option<Part>> {
        let V::Ref { ty, nonnull, src, .. } = v else { return None };
        if src.len() < 2 {
            return None;
        }
        if self.phi_nest >= MAX_PHI_NEST {
            return Some(gap.wild().then_some(Part::Wild));
        }
        self.phi_nest += 1;
        let mut alts = Vec::with_capacity(src.len());
        let mut ok = true;
        for s in src.iter() {
            let p = match s {
                Src::Str(l) => Some(vec![Part::Lit(crate::absint::lit_str(*l))]),
                Src::Catch(_) => None,
                s => {
                    let one = V::Ref { ty: ty.clone(), nonnull: *nonnull, src: Rc::from([*s].as_slice()), obj: None };
                    self.name_parts(f, &one, gap, depth)
                }
            };
            match p {
                Some(p) => alts.push(p),
                None if gap.wild() => alts.push(vec![Part::Wild]),
                None => {
                    ok = false;
                    break;
                }
            }
        }
        self.phi_nest -= 1;
        Some(ok.then_some(Part::Alt(alts)))
    }

    /// 站点 o 是取后缀调用时结果的拼接段；None = 非取后缀调用；Some(None) = 推不出
    pub(super) fn suffix_part(&mut self, f: &Frame, o: u32, gap: Gap, depth: u8) -> Option<Option<Part>> {
        let Some(Event::Invoke { mref, args, .. }) = event_at(f.a, o, is_invoke) else { return None };
        if !self.man.names.is_suffix(&mref.to_string()) {
            return None;
        }
        let (recv, start) = (args.first().cloned(), args.get(1).cloned());
        let r = recv.and_then(|v| self.name_parts(f, &v, gap, depth)).and_then(|p| flatten(&p)).and_then(|names| suffixes(&names, start.as_ref()));
        Some(match r {
            Some(set) => Some(Part::Any(set)),
            None => gap.wild().then_some(Part::Wild),
        })
    }
}

/// 名字集各名字的后缀：起点为 int 常量时取该后缀（越界的名字运行期抛异常，不计），否则取全部后缀；超出上限为 None
fn suffixes(names: &BTreeSet<Rc<str>>, start: Option<&V>) -> Option<BTreeSet<Rc<str>>> {
    let mut out: BTreeSet<Rc<str>> = BTreeSet::new();
    for n in names {
        match start {
            Some(V::Int(k)) => {
                let Ok(k) = usize::try_from(*k) else { continue };
                if let Some(s) = utf16_suffix(n, k) {
                    out.insert(Rc::from(s));
                }
            }
            _ => {
                out.extend(n.char_indices().map(|(i, _)| Rc::from(&n[i..])));
                out.insert(Rc::from(""));
            }
        }
        if out.len() > MAX_NAMES {
            stats::cap_hit(stats::CAP_LNAMES);
            return None;
        }
    }
    Some(out)
}

/// 自第 k 个 UTF-16 码元起的后缀（k 落在代理对中间时不计）
fn utf16_suffix(s: &str, k: usize) -> Option<&str> {
    let mut units = 0;
    for (i, c) in s.char_indices() {
        if units == k {
            return Some(&s[i..]);
        }
        units += c.len_utf16();
        if units > k {
            return None;
        }
    }
    (units == k).then_some("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suffix_sets() {
        let names: BTreeSet<Rc<str>> = [Rc::from("a.Bc")].into_iter().collect();
        let all = suffixes(&names, None).unwrap();
        let want: BTreeSet<Rc<str>> = ["a.Bc", ".Bc", "Bc", "c", ""].into_iter().map(Rc::from).collect();
        assert_eq!(all, want);
        let at = suffixes(&names, Some(&V::Int(2))).unwrap();
        assert_eq!(at, [Rc::from("Bc")].into_iter().collect());
        assert!(suffixes(&names, Some(&V::Int(9))).unwrap().is_empty());
        assert_eq!(utf16_suffix("a\u{1F600}b", 3), Some("b"));
        assert_eq!(utf16_suffix("a\u{1F600}b", 2), None);
        assert_eq!(utf16_suffix("ab", 2), Some(""));
    }
}

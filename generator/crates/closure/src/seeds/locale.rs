//! Locale 资源束种子（seeds.toml `[locale]`）：资源束经类名反射装载、无静态调用边，入选 locale
//! 由用户字节码中静态可见的 locale 引用推出（参照 GraalVM `-H:IncludeLocales`）：
//!
//! - 默认集：ROOT + en；`--locale` 显式给出的标签；
//! - `consts` 类的 static 自类型常量字段（`getstatic Locale.FRANCE`）沿 `<clinit>` 溯源到字符串字面量；
//! - `factories` 调用且实参全为字符串字面量（`Locale.of("fr", "FR")`）；
//! - `tags` 调用且实参为字面量语言标签（`Locale.forLanguageTag("fr-FR")`）。
//!
//! 每个 locale 连同父链（fr_FR → fr → ROOT）入选——ResourceBundle 回退语义所需。

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};

use classfile::descriptor::parse_method;
use classfile::insn::Operand;
use classfile::{ClassFile, Const, Insn};
use resolve::ClassPath;

const INVOKESTATIC: u8 = 0xb8;
const GETSTATIC: u8 = 0xb2;
const PUTSTATIC: u8 = 0xb3;
const AALOAD: u8 = 0x32;
const AASTORE: u8 = 0x53;

/// (语言, 文字, 地区, 变体)
pub type Locale = (String, String, String, String);

#[derive(Debug, Default)]
pub struct LocaleCfg {
    pub consts: HashSet<String>,
    pub factories: HashSet<String>,
    pub tags: HashSet<String>,
    /// (束基名, 族触发成员；None = 全局 triggers)
    pub bundles: Vec<(String, Option<Vec<String>>)>,
    pub triggers: Vec<String>,
}

impl LocaleCfg {
    pub fn from_toml(sec: Option<&toml::Value>) -> Self {
        let Some(sec) = sec else { return Self::default() };
        let strs = |k: &str| -> Vec<String> {
            sec.get(k).and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect()).unwrap_or_default()
        };
        let mut bundles = Vec::new();
        for b in sec.get("bundles").and_then(|v| v.as_array()).into_iter().flatten() {
            if let Some(s) = b.as_str() {
                bundles.push((s.to_string(), None));
            } else if let Some(base) = b.get("base").and_then(|v| v.as_str()) {
                let t = b.get("triggers").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_str().map(String::from)).collect());
                bundles.push((base.to_string(), Some(t.unwrap_or_default())));
            }
        }
        LocaleCfg {
            consts: strs("consts").into_iter().collect(),
            factories: strs("factories").into_iter().collect(),
            tags: strs("tags").into_iter().collect(),
            bundles,
            triggers: strs("triggers"),
        }
    }

    /// 束族的触发成员（`类.成员`）：族自带触发成员优先，否则取全局触发成员
    pub fn base_triggers(&self, base: &str) -> &[String] {
        self.bundles.iter().find(|(b, _)| b == base).and_then(|(_, t)| t.as_deref()).unwrap_or(&self.triggers)
    }

    /// 触发成员（`类.成员`）已可达的束族基名
    pub fn triggered_bases(&self, reached: impl Fn(&str) -> bool) -> Vec<String> {
        self.bundles
            .iter()
            .filter(|(_, t)| t.as_ref().unwrap_or(&self.triggers).iter().any(|m| reached(m)))
            .map(|(b, _)| b.clone())
            .collect()
    }
}

fn norm(lang: &str, script: &str, region: &str, variant: &str) -> Locale {
    let mut s = script.to_lowercase();
    if let Some(f) = s.get(..1) {
        s = f.to_uppercase() + &s[1..];
    }
    (lang.to_lowercase(), s, region.to_uppercase(), variant.to_string())
}

/// BCP 47 标签或下划线形式 → Locale
pub fn parse_tag(tag: &str) -> Option<Locale> {
    let parts: Vec<&str> = tag.split(['-', '_']).filter(|p| !p.is_empty()).collect();
    let first = *parts.first()?;
    if !first.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    let mut rest = &parts[1..];
    let mut script = "";
    if rest.first().is_some_and(|r| r.len() == 4 && r.chars().all(|c| c.is_ascii_alphabetic())) {
        script = rest[0];
        rest = &rest[1..];
    }
    let mut region = "";
    if rest.first().is_some_and(|r| (r.len() == 2 && r.chars().all(|c| c.is_ascii_alphabetic())) || (r.len() == 3 && r.chars().all(|c| c.is_ascii_digit()))) {
        region = rest[0];
        rest = &rest[1..];
    }
    Some(norm(first, script, region, &rest.join("_")))
}

/// ResourceBundle 候选后缀（由具体到一般，不含 ROOT）
pub fn parent_chain(l: &Locale) -> Vec<String> {
    let (lang, script, region, variant) = l;
    if lang.is_empty() {
        return vec![];
    }
    let mut c = Vec::new();
    if !script.is_empty() {
        if !region.is_empty() && !variant.is_empty() {
            c.push(format!("{lang}_{script}_{region}_{variant}"));
        }
        if !region.is_empty() {
            c.push(format!("{lang}_{script}_{region}"));
        }
        c.push(format!("{lang}_{script}"));
    }
    if !region.is_empty() && !variant.is_empty() {
        c.push(format!("{lang}_{region}_{variant}"));
    }
    if !region.is_empty() {
        c.push(format!("{lang}_{region}"));
    }
    c.push(lang.clone());
    let mut out: Vec<String> = Vec::new();
    for x in c {
        if !out.contains(&x) {
            out.push(x);
        }
    }
    out
}

fn str_lit(i: &Insn) -> Option<String> {
    match &i.operand {
        Operand::Ldc(Const::String(s)) => Some(s.clone()),
        _ => None,
    }
}

fn int_lit(i: &Insn) -> Option<i32> {
    match (i.opcode, &i.operand) {
        (0x02..=0x08, _) => Some(i.opcode as i32 - 3),
        (0x10 | 0x11, Operand::Int(v)) => Some(*v),
        _ => None,
    }
}

fn iload_index(i: &Insn) -> Option<i32> {
    match (i.opcode, &i.operand) {
        (0x1a..=0x1d, _) => Some(i.opcode as i32 - 0x1a),
        (0x15, Operand::Local(k)) => Some(*k as i32),
        _ => None,
    }
}

fn is_aload(i: &Insn) -> bool {
    i.opcode == 0x19 || (0x2a..=0x2d).contains(&i.opcode)
}

fn nparams(desc: &str) -> usize {
    parse_method(desc).map_or(0, |d| d.params.len())
}

fn insns<'c>(cf: &'c ClassFile, name: &str, desc: Option<&str>) -> Option<&'c [Insn]> {
    cf.methods.iter().find(|m| m.name == name && desc.is_none_or(|d| m.desc == d)).and_then(|m| m.code.as_ref()).map(|c| c.insns.as_slice())
}

#[derive(Clone)]
enum Arg {
    S(String),
    I(i32),
}

/// const 字段溯源（结构匹配；JDK 版本间常量表下标顺序不同，溯源自然携带）
struct Tracer<'c> {
    cp: &'c ClassPath,
    fields: RefCell<HashMap<(String, String), Option<Vec<String>>>>,
    tables: RefCell<HashMap<(String, String), HashMap<i32, Vec<String>>>>,
}

impl<'c> Tracer<'c> {
    fn static_field(&self, cls: &str, f: &str) -> Option<Vec<String>> {
        let k = (cls.to_string(), f.to_string());
        if let Some(v) = self.fields.borrow().get(&k) {
            return v.clone();
        }
        self.fields.borrow_mut().insert(k.clone(), None);
        let v = self.trace_field(cls, f);
        self.fields.borrow_mut().insert(k, v.clone());
        v
    }

    fn trace_field(&self, cls: &str, f: &str) -> Option<Vec<String>> {
        let cf = self.cp.get(cls)?;
        let ins = insns(&cf, "<clinit>", None)?;
        let i = ins.iter().position(|x| x.opcode == PUTSTATIC && matches!(&x.operand, Operand::Field(r) if r.owner == cls && r.name == f))?;
        self.value_before(ins, i)
    }

    /// ins[i] 消费的栈顶值：invokestatic 且实参全为字面量 → 溯源其返回值
    fn value_before(&self, ins: &[Insn], i: usize) -> Option<Vec<String>> {
        let call = ins.get(i.checked_sub(1)?)?;
        let Operand::Method(r, _) = &call.operand else { return None };
        if call.opcode != INVOKESTATIC {
            return None;
        }
        let n = nparams(&r.desc);
        let start = (i - 1).checked_sub(n)?;
        let mut args = Vec::new();
        for a in &ins[start..i - 1] {
            args.push(str_lit(a).map(Arg::S).or_else(|| int_lit(a).map(Arg::I))?);
        }
        if !args.is_empty() && args.iter().all(|a| matches!(a, Arg::S(_))) {
            return Some(args.into_iter().map(|a| if let Arg::S(s) = a { s } else { unreachable!() }).collect());
        }
        self.method_return(&r.owner, &r.name, &r.desc, &args)
    }

    /// 静态方法体 `getstatic T.arr; iload_k; aaload` → 常量表 T.arr[args[k]]
    fn method_return(&self, cls: &str, name: &str, desc: &str, args: &[Arg]) -> Option<Vec<String>> {
        let cf = self.cp.get(cls)?;
        let ins = insns(&cf, name, Some(desc))?;
        for w in ins.windows(3) {
            if w[0].opcode != GETSTATIC || w[2].opcode != AALOAD {
                continue;
            }
            let (Some(k), Operand::Field(r)) = (iload_index(&w[1]), &w[0].operand) else { continue };
            if let Some(Arg::I(idx)) = args.get(k as usize) {
                return self.array_elem(&r.owner, &r.name, *idx);
            }
        }
        None
    }

    fn array_elem(&self, cls: &str, arr: &str, idx: i32) -> Option<Vec<String>> {
        let k = (cls.to_string(), arr.to_string());
        if !self.tables.borrow().contains_key(&k) {
            let t = self.array_table(cls, arr);
            self.tables.borrow_mut().insert(k.clone(), t);
        }
        self.tables.borrow()[&k].get(&idx).cloned()
    }

    /// T.<clinit>：`aload; <idx>; 字面量…; invokestatic; aastore`，且该方法写 T.arr
    fn array_table(&self, cls: &str, arr: &str) -> HashMap<i32, Vec<String>> {
        let mut table = HashMap::new();
        let Some(cf) = self.cp.get(cls) else { return table };
        let Some(ins) = insns(&cf, "<clinit>", None) else { return table };
        if !ins.iter().any(|x| x.opcode == PUTSTATIC && matches!(&x.operand, Operand::Field(r) if r.owner == cls && r.name == arr)) {
            return table;
        }
        for j in 2..ins.len() {
            if ins[j].opcode != AASTORE || ins[j - 1].opcode != INVOKESTATIC {
                continue;
            }
            let Operand::Method(r, _) = &ins[j - 1].operand else { continue };
            let n = nparams(&r.desc);
            let Some(lo) = (j - 1).checked_sub(n) else { continue };
            let lits: Option<Vec<String>> = ins[lo..j - 1].iter().map(str_lit).collect();
            let Some(lits) = lits else { continue };
            let (Some(kk), Some(al)) = (lo.checked_sub(1), lo.checked_sub(2)) else { continue };
            if let Some(k) = int_lit(&ins[kk]) {
                if is_aload(&ins[al]) {
                    table.entry(k).or_insert(lits);
                }
            }
        }
        table
    }
}

fn from_literals(l: &[String]) -> Locale {
    let g = |i: usize| l.get(i).map_or("", |s| s.as_str());
    norm(g(0), "", g(1), g(2))
}

/// 入选 locale 集（不含 ROOT；束基名总是入选）
pub fn collect(cfg: &LocaleCfg, cp: &ClassPath, users: &[std::sync::Arc<ClassFile>], extra: &[String]) -> BTreeSet<Locale> {
    let tracer = Tracer { cp, fields: Default::default(), tables: Default::default() };
    let mut found = BTreeSet::new();
    found.insert(norm("en", "", "", ""));
    found.extend(extra.iter().filter_map(|t| parse_tag(t)));
    for cf in users {
        for m in &cf.methods {
            let Some(code) = &m.code else { continue };
            let ins = &code.insns;
            for (i, x) in ins.iter().enumerate() {
                match &x.operand {
                    Operand::Field(r) if x.opcode == GETSTATIC && cfg.consts.contains(&r.owner) && r.desc == format!("L{};", r.owner) => {
                        if let Some(l) = tracer.static_field(&r.owner, &r.name).map(|v| from_literals(&v)) {
                            if !l.0.is_empty() {
                                found.insert(l);
                            }
                        }
                    }
                    Operand::Method(r, _) => {
                        let key = r.to_string();
                        let is_tag = cfg.tags.contains(&key);
                        if !is_tag && !cfg.factories.contains(&key) {
                            continue;
                        }
                        let n = nparams(&r.desc);
                        let Some(lo) = i.checked_sub(n) else { continue };
                        let Some(lits) = ins[lo..i].iter().map(str_lit).collect::<Option<Vec<_>>>() else { continue };
                        let l = if is_tag { lits.first().and_then(|t| parse_tag(t)) } else { Some(from_literals(&lits)) };
                        if let Some(l) = l.filter(|l| !l.0.is_empty()) {
                            found.insert(l);
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    found
}

/// 入选 locale（含父链）× 束族 → 类路径上存在的资源束类
pub fn bundle_classes(locales: &BTreeSet<Locale>, bases: &[String], cp: &ClassPath) -> BTreeSet<String> {
    let suffixes: BTreeSet<String> = locales.iter().flat_map(parent_chain).collect();
    let mut out = BTreeSet::new();
    for base in bases {
        let (pkg, simple) = base.rsplit_once('/').unwrap_or(("", base));
        let mut names = vec![base.clone()];
        for s in &suffixes {
            names.push(format!("{base}_{s}"));
            names.push(format!("{pkg}/ext/{simple}_{s}"));
        }
        for n in names {
            if cp.contains(&n) {
                out.insert(n);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tags_and_parents() {
        let l = parse_tag("zh-Hant-TW").unwrap();
        assert_eq!(l, ("zh".into(), "Hant".into(), "TW".into(), "".into()));
        assert_eq!(parent_chain(&l), vec!["zh_Hant_TW", "zh_Hant", "zh_TW", "zh"]);
        assert_eq!(parent_chain(&parse_tag("fr_FR").unwrap()), vec!["fr_FR", "fr"]);
        assert!(parse_tag("123").is_none());
    }

    #[test]
    fn base_triggers_family_then_global() {
        let cfg = LocaleCfg {
            bundles: vec![("a/B".into(), None), ("a/C".into(), Some(vec!["a/X.f".into()]))],
            triggers: vec!["a/Y.g".into()],
            ..Default::default()
        };
        assert_eq!(cfg.base_triggers("a/B"), ["a/Y.g".to_string()]);
        assert_eq!(cfg.base_triggers("a/C"), ["a/X.f".to_string()]);
        assert_eq!(cfg.triggered_bases(|m| m == "a/X.f"), vec!["a/C".to_string()]);
    }
}

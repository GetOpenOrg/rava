//! 引擎：按键查找闸门（`[facts.keyed_lookups]`，见 `manifest/keyed.rs`）。
//!
//! 按键查找入口的返回对象，其键等于调用点键实参的值。调用点的结果先进闸门节点 [`Node::K`]，再按键放行到站点节点：
//! - 站点键集：调用点键实参的全部名字（字符串常量 / 拼接段 / 形参与 String 字段槽，见 `pstrs.rs`），推不出为任意；
//! - 类键集：键类子类 X 的对象的键——字节码 `new X` 后的构造器调用点上键形参的全部名字（键类构造器的键形参由清单给出，
//!   子类构造器经 `super(..)` / `this(..)` 链追溯到键类构造器，见 [`Engine::ctor_key_slot`]）；X 经字节码 `new` 以外的
//!   途径实例化（反射 / 手写 / 反序列化等）、构造器链追溯不到、或键实参推不出时为任意；
//! - 不是键类子类型的值、open(非键类子类型) 原样放行；键类子类型的对象在两键集相交（或任一为任意）时放行，否则暂扣；
//!   open(o ⊂ 键类) 展开为已实例化的 o 的子类型逐个判定（G 增长时补判）。站点键集任意时 open 原样放行。
//!
//! 单调：两类键集只并不减，暂扣的值在键集增长 / 变为任意时放行；放行结果与处理顺序无关。

use super::class_lookup::Gap;
use super::name_eval::Frame;
use super::sealed::flatten;
use super::*;

/// 键集：任意，或有限的名字集合
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Keys {
    Any,
    Set(BTreeSet<Rc<str>>),
}

impl Default for Keys {
    fn default() -> Self {
        Keys::Set(BTreeSet::new())
    }
}

impl Keys {
    /// 并入 o；返回是否增长
    fn merge(&mut self, o: Keys) -> bool {
        match (&mut *self, o) {
            (Keys::Any, _) => false,
            (s, Keys::Any) => {
                *s = Keys::Any;
                true
            }
            (Keys::Set(a), Keys::Set(b)) => {
                let n = a.len();
                a.extend(b);
                a.len() > n
            }
        }
    }

    fn meets(&self, o: &Keys, fold: bool) -> bool {
        match (self, o) {
            (Keys::Any, _) | (_, Keys::Any) => true,
            (Keys::Set(a), Keys::Set(b)) if fold => {
                let up: BTreeSet<String> = a.iter().map(|x| x.to_uppercase()).collect();
                b.iter().any(|x| up.contains(&x.to_uppercase()))
            }
            (Keys::Set(a), Keys::Set(b)) => a.iter().any(|x| b.contains(x)),
        }
    }
}

/// 键类构造器（含子类构造器）的键来源
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum KeySlot {
    /// 形参序号（按描述符，0 起，不含接收者）
    Param(usize),
    Lit(Rc<str>),
    Unknown,
}

/// 一个调用点的闸门
#[derive(Debug)]
pub(super) struct KGate {
    m: usize,
    off: u32,
    /// 键类
    kc: u32,
    fold: bool,
    keys: Keys,
    /// 暂扣的值：类型 → 值
    held: BTreeMap<u32, BTreeSet<u32>>,
    /// 已展开的 open 类型
    opens: BTreeSet<u32>,
}

#[derive(Default)]
pub(super) struct KeyedState {
    /// 键类序号 → 其按键查找入口（懒求）
    kcs: Option<Vec<u32>>,
    pub(super) gates: Vec<KGate>,
    at: HashMap<(usize, u32), u32>,
    /// 键类子类型 → 其对象的键
    class_keys: HashMap<u32, Keys>,
    /// 类型 → 暂扣了该类型值的闸门
    holders: HashMap<u32, BTreeSet<u32>>,
    /// open 类型 → 展开了它的闸门（G 增长时补判）
    open_watch: HashMap<u32, BTreeSet<u32>>,
    /// （类, 构造器描述符）→ 键来源
    slots: HashMap<(u32, Rc<str>), KeySlot>,
}

/// 构造器链追溯深度上限
const MAX_CHAIN: u8 = 8;

impl<'a> Engine<'a> {
    /// 清单登记的全部键类（按键查找入口的返回类型）
    fn key_classes(&mut self) -> Vec<u32> {
        if let Some(k) = &self.keyed.kcs {
            return k.clone();
        }
        let man = self.man;
        let mut out: Vec<u32> = vec![];
        for member in man.keyed_lookups.members() {
            let ret = member.split_once(':').and_then(|(_, d)| parse_method(d)).and_then(|md| md.ret);
            if let Some(FieldType::Object(c)) = ret {
                let id = self.id(&c);
                if !out.contains(&id) {
                    out.push(id);
                }
            }
        }
        self.keyed.kcs = Some(out.clone());
        out
    }

    /// 类型 t 落在某个键类之下
    fn under_key_class(&mut self, t: u32) -> bool {
        if self.man.keyed_lookups.is_empty() {
            return false;
        }
        self.key_classes().into_iter().any(|k| self.sub(t, k))
    }

    /// 调用点（m, off）调用已解析成员 resolved 时的结果节点：按键查找入口为闸门节点，否则为站点节点。
    /// 每次处理调用点时重求站点键集（并入）
    pub(super) fn keyed_res(&mut self, m: usize, off: u32, resolved: &MemberRef, pargs: &[V]) -> Node {
        if self.man.keyed_lookups.is_empty() {
            return Node::S(m, off);
        }
        let key = self.mref_key(resolved);
        let man = self.man;
        let Some(spec) = man.keyed_lookups.get(&key) else { return Node::S(m, off) };
        let (ki, fold) = (spec.key, spec.fold_case);
        let Some(Some(FieldType::Object(c))) = parse_method(&resolved.desc).map(|md| md.ret) else { return Node::S(m, off) };
        let kc = self.id(&c);
        let g = match self.keyed.at.get(&(m, off)) {
            Some(&g) => g,
            None => {
                let g = self.keyed.gates.len() as u32;
                self.keyed.gates.push(KGate { m, off, kc, fold, keys: Keys::default(), held: BTreeMap::new(), opens: BTreeSet::new() });
                self.keyed.at.insert((m, off), g);
                g
            }
        };
        // 协议键站点：键是本方法某个 URL 串形参解析出的协议名（见 `keyed_scheme.rs`），不取键实参的写入名字
        let site = self.mref_key(&self.methods[m].key.clone());
        let keys = match spec.scheme_sites.get(&*site) {
            Some(&j) => self.scheme_keys(m, j),
            None => pargs.get(ki).map_or(Keys::Any, |v| self.names_of(m, v)),
        };
        if self.keyed.gates[g as usize].keys.merge(keys) {
            self.kgate_recheck(g, None);
        }
        Node::K(g)
    }

    /// 方法 m 中字符串值 v 的全部名字
    pub(super) fn names_of(&mut self, m: usize, v: &V) -> Keys {
        match v {
            V::Str(s, _) => return Keys::Set([s.clone()].into()),
            V::Null => return Keys::default(),
            _ => {}
        }
        let Some(a) = self.methods[m].analysis.clone() else { return Keys::Any };
        if a.conservative {
            return Keys::Any;
        }
        let owner = self.methods[m].key.owner.clone();
        let f = Frame { m: Some(m), a: &a, owner: &owner, up: None };
        match self.name_parts(&f, v, Gap::Fail, 0).and_then(|p| flatten(&p)) {
            Some(s) => Keys::Set(s),
            None => Keys::Any,
        }
    }

    /// 字节码构造器调用点（方法 m 调用类 x 的构造器 desc，实参不含接收者）：x 是键类子类型时并入其类键集。
    /// 构造器内对 this 的 `super(..)` / `this(..)` 不在此计（由子类构造器链追溯）
    pub(super) fn keyed_ctor(&mut self, m: usize, x: &str, desc: &str, recv: Option<&V>, pargs: &[V]) {
        let xid = self.id(x);
        if !self.under_key_class(xid) {
            return;
        }
        if let Some(k) = self.pattern_keys(xid) {
            self.class_keys_add(xid, k);
            return;
        }
        let this = matches!(recv, Some(V::Ref { src, .. }) if src.contains(&Src::Param(0)));
        if this && self.methods[m].key.name.as_str() == "<init>" {
            return;
        }
        let keys = match self.ctor_key_slot(xid, desc, 0) {
            KeySlot::Lit(s) => Keys::Set([s].into()),
            KeySlot::Param(j) => pargs.get(j).map_or(Keys::Any, |v| self.names_of(m, v)),
            KeySlot::Unknown => Keys::Any,
        };
        self.class_keys_add(xid, keys);
    }

    /// 类 x 经字节码 `new` 以外的途径实例化：其对象的键任意
    pub(super) fn keyed_instantiated(&mut self, x: u32, kind: &str) {
        if kind != "new" && self.under_key_class(x) {
            let k = self.pattern_keys(x).unwrap_or(Keys::Any);
            self.class_keys_add(x, k);
        }
    }

    /// 类名按清单命名约定（`class_pattern`）定出的键：x 落在登记了命名约定的键类之下且类名匹配时为该键，
    /// 与实例化途径（字节码 `new` / 反射）无关；否则 None（按构造器追溯）
    fn pattern_keys(&mut self, x: u32) -> Option<Keys> {
        let man = self.man;
        let name = self.names[x as usize].clone();
        for member in man.keyed_lookups.members() {
            let Some(spec) = man.keyed_lookups.get(member) else { continue };
            if spec.class_pattern.is_none() {
                continue;
            }
            let ret = member.split_once(':').and_then(|(_, d)| parse_method(d)).and_then(|md| md.ret);
            let Some(FieldType::Object(c)) = ret else { continue };
            let kc = self.id(&c);
            if !self.sub(x, kc) {
                continue;
            }
            if let Some(k) = spec.pattern_key(&name) {
                return Some(Keys::Set([Rc::from(k.as_str())].into()));
            }
        }
        None
    }

    fn class_keys_add(&mut self, x: u32, keys: Keys) {
        if self.keyed.class_keys.entry(x).or_default().merge(keys) {
            let gs: Vec<u32> = self.keyed.holders.get(&x).map(|s| s.iter().copied().collect()).unwrap_or_default();
            for g in gs {
                self.kgate_recheck(g, Some(x));
            }
        }
    }

    /// 类 x 构造器 desc 的键来源：键类构造器取清单；子类构造器追溯其对 this 的构造器调用
    fn ctor_key_slot(&mut self, x: u32, desc: &str, depth: u8) -> KeySlot {
        let k = (x, Rc::<str>::from(desc));
        if let Some(s) = self.keyed.slots.get(&k) {
            return s.clone();
        }
        let s = self.ctor_key_slot_uncached(x, desc, depth);
        self.keyed.slots.insert(k, s.clone());
        s
    }

    fn ctor_key_slot_uncached(&mut self, x: u32, desc: &str, depth: u8) -> KeySlot {
        let xname = self.names[x as usize].clone();
        let ctor = format!("<init>:{desc}");
        let man = self.man;
        for member in man.keyed_lookups.members() {
            let ret = member.split_once(':').and_then(|(_, d)| parse_method(d)).and_then(|md| md.ret);
            if matches!(&ret, Some(FieldType::Object(c)) if **c == *xname) {
                let j = man.keyed_lookups.get(member).and_then(|e| e.ctors.get(&ctor).copied());
                return j.map_or(KeySlot::Unknown, KeySlot::Param);
            }
        }
        if depth >= MAX_CHAIN {
            return KeySlot::Unknown;
        }
        let Some(cf) = self.h.class(&xname) else { return KeySlot::Unknown };
        let Some(code) = cf.method("<init>", desc).and_then(|mm| mm.code.as_ref()) else { return KeySlot::Unknown };
        let a = absint::analyze(&xname, desc, false, code, &super::sealed::Plain);
        if a.conservative {
            return KeySlot::Unknown;
        }
        let sup = cf.super_name.clone();
        let mut out: Option<KeySlot> = None;
        for (_, e) in &a.events {
            let Event::Invoke { opcode: classfile::op::INVOKESPECIAL, mref, args, .. } = e else { continue };
            let on_this = matches!(args.first(), Some(V::Ref { src, .. }) if src[..] == [Src::Param(0)]);
            if mref.name != "<init>" || !on_this || (Some(&mref.owner) != sup.as_ref() && *mref.owner != *xname) {
                continue;
            }
            let up = self.id(&mref.owner);
            let s = match self.ctor_key_slot(up, &mref.desc, depth + 1) {
                KeySlot::Param(j) => match args.get(j + 1) {
                    Some(V::Str(s, _)) => KeySlot::Lit(s.clone()),
                    Some(V::Ref { src, .. }) => match src[..] {
                        [Src::Param(k)] if k > 0 => KeySlot::Param(k as usize - 1),
                        _ => KeySlot::Unknown,
                    },
                    _ => KeySlot::Unknown,
                },
                s => s,
            };
            // 多条构造器调用路径（分支）给出不同来源时推不出
            out = Some(match out {
                None => s,
                Some(p) if p == s => p,
                Some(_) => KeySlot::Unknown,
            });
        }
        out.unwrap_or(KeySlot::Unknown)
    }

    /// 闸门节点新增 delta
    pub(super) fn kgate_grown(&mut self, g: u32, delta: &TypeSet) {
        for x in delta.classes.iter() {
            self.kgate_admit(g, x);
        }
        for o in delta.open.iter() {
            self.kgate_open(g, o);
        }
    }

    fn kgate_admit(&mut self, g: u32, x: u32) {
        let t = self.ty(x);
        let kc = self.keyed.gates[g as usize].kc;
        if !self.sub(t, kc) || self.kgate_pass(g, t) {
            self.kgate_push(g, TypeSet::exact(x));
            return;
        }
        self.keyed.gates[g as usize].held.entry(t).or_default().insert(x);
        self.keyed.holders.entry(t).or_default().insert(g);
    }

    fn kgate_pass(&self, g: u32, t: u32) -> bool {
        let gate = &self.keyed.gates[g as usize];
        self.keyed.class_keys.get(&t).is_some_and(|k| gate.keys.meets(k, gate.fold))
    }

    fn kgate_open(&mut self, g: u32, o: u32) {
        let kc = self.keyed.gates[g as usize].kc;
        if !self.sub(o, kc) || self.keyed.gates[g as usize].keys == Keys::Any {
            self.kgate_push(g, TypeSet::open(o));
            return;
        }
        if !self.keyed.gates[g as usize].opens.insert(o) {
            return;
        }
        self.keyed.open_watch.entry(o).or_default().insert(g);
        for x in self.g_of(o).iter() {
            self.kgate_admit(g, *x);
        }
    }

    /// G 新增类型 id：展开过其 open 上界的闸门补判
    pub(super) fn keyed_g_grow(&mut self, id: u32) {
        if self.keyed.open_watch.is_empty() {
            return;
        }
        let ws: Vec<(u32, Vec<u32>)> = self.keyed.open_watch.iter().map(|(o, gs)| (*o, gs.iter().copied().collect())).collect();
        for (o, gs) in ws {
            if self.sub(id, o) {
                for g in gs {
                    self.kgate_admit(g, id);
                }
            }
        }
    }

    /// 键集增长后重判暂扣的值（t = 只重判该类型；None = 站点键集增长，全部重判，任意时 open 原样放行）
    fn kgate_recheck(&mut self, g: u32, t: Option<u32>) {
        let ts: Vec<u32> = match t {
            Some(t) => vec![t],
            None => self.keyed.gates[g as usize].held.keys().copied().collect(),
        };
        for t in ts {
            if !self.kgate_pass(g, t) {
                continue;
            }
            let Some(xs) = self.keyed.gates[g as usize].held.remove(&t) else { continue };
            if let Some(hs) = self.keyed.holders.get_mut(&t) {
                hs.remove(&g);
            }
            self.kgate_push(g, TypeSet { classes: xs.into_iter().collect(), open: IdSet::default() });
        }
        if t.is_none() && self.keyed.gates[g as usize].keys == Keys::Any {
            let os: Vec<u32> = self.keyed.gates[g as usize].opens.iter().copied().collect();
            if !os.is_empty() {
                self.kgate_push(g, TypeSet { classes: IdSet::default(), open: os.into_iter().collect() });
            }
        }
    }

    /// 闸门的显示名（诊断）
    pub(super) fn kgate_label(&self, g: u32) -> String {
        let gate = &self.keyed.gates[g as usize];
        format!("@{} {}", gate.off, self.ctx_label(gate.m))
    }

    fn kgate_push(&mut self, g: u32, s: TypeSet) {
        let (m, off) = (self.keyed.gates[g as usize].m, self.keyed.gates[g as usize].off);
        self.flow_src = self.graph.id(Node::K(g));
        self.add_to(Node::S(m, off), &s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set(xs: &[&str]) -> Keys {
        Keys::Set(xs.iter().map(|x| Rc::from(*x)).collect())
    }

    #[test]
    fn merge_is_monotone() {
        let mut k = Keys::default();
        assert!(k.merge(set(&["A"])));
        assert!(!k.merge(set(&["A"])));
        assert!(k.merge(Keys::Any));
        assert!(!k.merge(set(&["B"])));
        assert_eq!(k, Keys::Any);
    }

    #[test]
    fn scheme_keys_release_matching_handlers_only() {
        use super::super::class_lookup::Part;
        use super::super::keyed_scheme::scheme_keys_of;
        // 处理器类键（命名约定）与站点协议键：常量协议只放行同名处理器；协议推不出时任意键全部放行
        let jar = set(&["jar"]);
        let site = scheme_keys_of(&[vec![Part::Lit(Rc::from("file:")), Part::Wild]]);
        assert_eq!(site, set(&["file"]));
        assert!(!site.meets(&jar, false));
        assert!(site.meets(&set(&["file"]), false));
        let unknown = scheme_keys_of(&[vec![Part::Wild]]);
        assert_eq!(unknown, Keys::Any);
        assert!(unknown.meets(&jar, false));
        // 不带协议的串不给键：不放行任何处理器
        let rel = scheme_keys_of(&[vec![Part::Lit(Rc::from("dir/x.txt"))]]);
        assert!(!rel.meets(&jar, false));
    }

    #[test]
    fn meets_respects_case_folding() {
        assert!(set(&["MessageDigest"]).meets(&set(&["messagedigest"]), true));
        assert!(!set(&["MessageDigest"]).meets(&set(&["messagedigest"]), false));
        assert!(!set(&["Cipher"]).meets(&set(&["Signature"]), true));
        assert!(Keys::Any.meets(&Keys::default(), false));
        assert!(!Keys::default().meets(&set(&["A"]), false));
    }
}

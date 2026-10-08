//! 引擎：反射对象标记——查找入口的结果建模为携带所指方法的标记对象（清单 `[facts.reflect.direct_invokers]` 的
//! `lookups` / `list` / `copies`，见 `manifest/direct.rs`）。
//!
//! 反射对象（Method）在一处查找、经字段 / 数组 / 列表传到别处调用时，调用点的接收者值集只给出「某个 Method 对象」，
//! 分析器无从得知它所指的方法，只能按原入口接边（Method.invoke 体内的 CS 判定、注解解析、访问器链随之入链）。
//! 标记让值集本身携带所指方法：
//! - **查找调用点**：名字、查找类、形参类型都推得出时，结果不接被调方返回值，改为本点解析出的各方法的标记
//!   （形态按返回类型：单个 / 标记数组 / 经 `list` 模型装入列表）。查找类值集里所指未知的部分（open、非字节码类镜像）
//!   给缺口标记（所指不明，调用点不因它回退也不由它产生目标——同 `method_lookup.rs::recv_mirrors` 的反射缺口口径：
//!   缺口类上的成员本就不经查找点进入调用链）。推不出（名字不完整、形参类型数组所指未知不影响——按通配取超集）时
//!   调用点记入 `rmark_fallback`，此后恒接被调方返回值（单调；已并入的标记保留，过近似）。
//! - **复制调用点**：输入值集全是标记时结果即这些标记，否则同上回退。
//! - 标记是反射类型的抽象对象（`objs`，派发、字段读写同该类型的对象）：创建即逃逸，字段读取经未知接收者视图取全部
//!   写入（同运行期真实对象——它们是被调方内分配的类 id）；不作克隆上下文（`ctxsel.rs::recv_ctx`）。
//!
//! 健全性：查找点的被调方照常接边（体内效果与对其它调用方的返回值不变），本点结果只少了「不是本点所指方法的对象」——
//! 运行期该点返回的对象所指方法必在解析出的集合里：名字取候选全集，查找类取值集全部镜像（缺口另记），形参类型
//! 按位置奇偶取元素镜像集（所指未知 / 基本类型类镜像按通配），口径取可见方法的超集（public 取类链首个命中类与全部
//! 超接口的同名同形参方法）。

use super::*;
use crate::manifest::{DirectInvoker, LookupScope, LookupShape};
use class_lookup::Part;
use field_handles::MARK_ARRAY;

/// 列表模型调用的伪调用偏移（与字节码偏移、标记数组偏移不相交）
const LIST_CALL: u32 = 0x2000_0000;

/// 标记所指
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) enum MethodMark {
    /// 解析出的方法
    Member(MemberRef),
    /// 无名字查找点（方法, 偏移）上按口径枚举的全部方法：一个查找点共享一个标记，所指类集记在 `rmark_all`、
    /// 随查找类值集增长（各类一个标记时，遍历全部可序列化类的查找点给出成千个逃逸对象，涌入每个 open 枢纽）。
    /// 同一查找点的标记本就同入本点结果、同进本点标记数组，合并不改变任何值集的去向
    All(u32, u32, LookupScope),
    /// 所指不明（查找类落在反射缺口上）
    Gap,
}

impl std::fmt::Display for MethodMark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MethodMark::Member(k) => write!(f, "{k}"),
            MethodMark::All(m, off, s) => write!(f, "{s:?}@{m}:{off}"),
            MethodMark::Gap => write!(f, "?"),
        }
    }
}

/// 形参类型数组的一个奇偶槽：元素镜像所指的类型描述符（None = 通配）
#[derive(Debug, Default, Clone)]
struct Slot {
    descs: BTreeSet<String>,
    prim: bool,
    any: bool,
}

/// 查找的形参类型约束：None = 无约束（枚举，或无形参类型实参）
#[derive(Debug, Default, Clone)]
struct Ptypes {
    arity: Option<usize>,
    slots: [Slot; 2],
}

impl Ptypes {
    fn wild() -> Ptypes {
        let any = Slot { any: true, ..Slot::default() };
        Ptypes { arity: None, slots: [any.clone(), any] }
    }

    /// 方法描述符 desc 的形参与约束相容
    fn admits(&self, desc: &str) -> bool {
        let Some(md) = parse_method(desc) else { return false };
        if self.arity.is_some_and(|n| n != md.params.len()) {
            return false;
        }
        md.params.iter().enumerate().all(|(i, p)| {
            let s = &self.slots[i % 2];
            s.any || if p.is_reference() { s.descs.contains(&p.descriptor()) } else { s.prim }
        })
    }
}

/// 查找名字：常量 / 推得出的候选全集，或拼接段
enum Names {
    Exact(BTreeSet<Rc<str>>),
    Parts(Vec<Part>),
}

impl Names {
    fn admits(&self, n: &str) -> bool {
        match self {
            Names::Exact(s) => s.contains(n),
            Names::Parts(p) => method_lookup::parts_match(p, n),
        }
    }
}

impl<'a> Engine<'a> {
    /// 字节码调用点 (m, off) 是清单所列查找 / 复制入口且结果按标记建模：true = 结果已由标记给出，被调方返回值不接入
    pub(super) fn method_marks_site(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V]) -> bool {
        if self.man.direct_invokers.is_empty() || self.methods[m].kind != Kind::Bytecode || self.rmark_fallback.contains(&(m, off)) {
            return false;
        }
        let key = self.mref_key(mref);
        let man = self.man;
        let done = if let Some((inv, lk)) = man.direct_invokers.lookup(&key) {
            match self.lookup_marks(m, opcode, mref, args, lk.scope) {
                Some((mut marks, all)) => {
                    if let Some(cs) = all {
                        let id = self.method_mark(&inv.recv, MethodMark::All(m as u32, off, lk.scope));
                        self.all_mark_grow(id, cs);
                        marks.insert(MethodMark::All(m as u32, off, lk.scope));
                    }
                    self.emit_marks(m, off, inv, lk.shape, marks);
                    true
                }
                None => false,
            }
        } else if let Some(inv) = man.direct_invokers.copy(&key) {
            self.copy_marks(m, off, opcode, mref, args, &inv.recv)
        } else {
            return false;
        };
        if !done {
            self.rmark_fallback.insert((m, off));
            self.mark_unfold(m, off, mref);
        }
        done
    }

    /// 调用点 (m, off) 转为回退：此前按标记建模时已接的目标与枢纽（重跑时按已接入去重、不再接结果）补接返回值
    fn mark_unfold(&mut self, m: usize, off: u32, mref: &MemberRef) {
        let Some(rt) = parse_method(&mref.desc).and_then(|md| md.ret).and_then(|r| self.ptype(&r)) else { return };
        let res = Node::S(m, off);
        let ts: Vec<usize> = self.dispatch.get(&(m, off)).into_iter().flatten().copied().collect();
        for t in ts {
            self.flow(Node::R(t), res, rt);
        }
        let hs: Vec<u32> = self.hub_sites.get(&(m, off)).into_iter().flatten().copied().collect();
        for h in hs {
            self.flow(Node::HR(h), res, rt);
        }
    }

    /// 反射类型 ty 上所指 mk 的标记（首次创建即逃逸）
    fn method_mark(&mut self, ty: &str, mk: MethodMark) -> u32 {
        let name = format!("{ty}#<method:{mk}>");
        if let Some(&id) = self.ids.get(name.as_str()) {
            return id;
        }
        let tid = self.id(ty);
        let id = self.id(&name);
        self.objs.insert(id, tid);
        self.rmarks.insert(id, mk);
        self.escape(&TypeSet::exact(id).classes);
        id
    }

    /// 标记并入调用点结果：单个直接并入；数组 / 列表形经本点的标记数组（列表再经 `list` 模型装入）
    fn emit_marks(&mut self, m: usize, off: u32, inv: &DirectInvoker, shape: LookupShape, marks: BTreeSet<MethodMark>) {
        let mut set = TypeSet::default();
        for mk in marks {
            let id = self.method_mark(&inv.recv, mk);
            set.classes.insert(id);
        }
        let res = Node::S(m, off);
        if shape == LookupShape::One {
            self.add_to(res, &set);
            return;
        }
        let via = Via::method("method-marks", m, Some(off));
        let arr_t = format!("[L{};", inv.recv);
        let arr = self.array_site(m, MARK_ARRAY | off, &arr_t, false, via.clone());
        for p in PARITIES {
            self.add_to(Node::E(arr, p), &set);
        }
        if shape == LookupShape::Array {
            self.add_to(res, &TypeSet::exact(arr));
            return;
        }
        let Some(car) = inv.list.clone() else { return };
        let Some(md) = parse_method(&car.desc) else { return };
        self.touch(&car.owner, Level::Layout, via.clone());
        self.init(&car.owner, via.clone());
        let ret = md.ret.as_ref().and_then(|r| self.ptype(r));
        let t = self.method(car, via);
        // 模型调用的实参不是本点的字节码实参：形参常量按未知绑定；伪偏移不进字节码调用点的派发登记
        let cv = self.call_vals.take();
        let outer = std::mem::replace(&mut self.cs.site_wrapped, false);
        let a: Args = vec![Some(vec![Feed::S(TypeSet::exact(arr))])];
        self.edge(m, LIST_CALL | off, t, Recv::None, &a, ret, Some(res));
        self.cs.site_wrapped = outer;
        self.call_vals = cv;
    }

    /// 共享标记 id 的所指类集并入 cs；增长时重跑读取其所指的直连调用点
    fn all_mark_grow(&mut self, id: u32, cs: BTreeSet<String>) {
        let cur = self.rmark_all.entry(id).or_default();
        let n = cur.len();
        cur.extend(cs);
        if cur.len() == n {
            return;
        }
        for w in self.rmark_readers.get(&id).cloned().unwrap_or_default() {
            self.push_site(w, site_prof::TRIG_RELEASE, None);
        }
    }

    /// 查找调用点的标记所指（无名字查找另给出全部方法所指的类集，由调用方并入本点的共享标记）；
    /// None = 推不出（名字不完整、形参形状不符、类不可得）
    #[allow(clippy::type_complexity)]
    fn lookup_marks(&mut self, m: usize, opcode: u8, mref: &MemberRef, args: &[V], scope: LookupScope) -> Option<(BTreeSet<MethodMark>, Option<BTreeSet<String>>)> {
        let md = parse_method(&mref.desc)?;
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let class_arr = format!("[L{CLASS};");
        let mut name = None;
        let mut ptv = None;
        let mut cls_vals = Vec::new();
        for (j, p) in md.params.iter().enumerate() {
            let v = args.get(base + j)?;
            match p {
                FieldType::Object(c) if c == STRING => name = Some(v),
                FieldType::Object(c) if c == CLASS => cls_vals.push(v),
                _ if p.descriptor() == class_arr => ptv = Some(v),
                _ => return None,
            }
        }
        if cls_vals.is_empty() {
            if base == 0 || mref.owner != CLASS {
                return None;
            }
            cls_vals.push(args.first()?);
        }
        let mut classes = BTreeSet::new();
        let mut gap = false;
        for v in cls_vals {
            gap |= self.mark_classes(m, v, &mut classes);
        }
        let mut out = BTreeSet::new();
        if gap {
            out.insert(MethodMark::Gap);
        }
        let Some(nv) = name else {
            // 无名字：枚举类上口径内的全部方法（类集空时不建共享标记）
            let all = (!classes.is_empty()).then_some(classes);
            return Some((out, all));
        };
        if classes.is_empty() {
            return Some((out, None));
        }
        let pts = match ptv {
            Some(v) => self.mark_ptypes(m, v),
            None => Ptypes::wild(),
        };
        let names = self.mark_names(m, nv)?;
        for c in &classes {
            for k in self.mark_resolve(c, &names, &pts, scope)? {
                out.insert(MethodMark::Member(k));
            }
        }
        Some((out, None))
    }

    /// 查找类集：Class 值 v 所指的字节码类并入 out；返回值集是否含所指未知的部分（open、非字节码类镜像等，即反射缺口）。
    /// 基本类型类镜像没有方法（查找恒抛异常），不并入、不算缺口
    fn mark_classes(&mut self, m: usize, v: &V, out: &mut BTreeSet<String>) -> bool {
        match v {
            V::Class(c, _) => {
                out.insert(c.to_string());
                false
            }
            V::Null => false,
            V::Ref { .. } => {
                let class = self.id(CLASS);
                let fs = self.feeds(m, v, class);
                let s = self.value_set(&fs);
                let mut gap = !s.open.is_empty();
                for x in s.classes.iter() {
                    match self.mirrors.get(&x) {
                        Some(&c) => {
                            out.insert(self.names[c as usize].to_string());
                        }
                        None if Some(x) == self.prim_mirror => {}
                        None => gap = true,
                    }
                }
                gap
            }
            _ => true,
        }
    }

    /// 形参类型数组值 v 的约束：长度取本地常量长度（null = 0）；元素按奇偶槽取各数组分配点的元素镜像
    /// （值集增长时本站点重跑）。所指未知的元素与数组（open）按通配
    fn mark_ptypes(&mut self, m: usize, v: &V) -> Ptypes {
        let mut pts = Ptypes::default();
        match v {
            V::Null => {
                pts.arity = Some(0);
                return pts;
            }
            V::Ref { obj: Some(o), .. } => {
                if let Obj::Len(n) = **o {
                    pts.arity = usize::try_from(n).ok();
                }
            }
            V::Ref { .. } => {}
            _ => return Ptypes::wild(),
        }
        let class_arr = format!("[L{CLASS};");
        let tid = self.id(&class_arr);
        let fs = self.feeds(m, v, tid);
        let s = self.value_set(&fs);
        if !s.open.is_empty() {
            return Ptypes { arity: pts.arity, ..Ptypes::wild() };
        }
        for x in s.classes.iter() {
            if !self.arrays.contains_key(&x) {
                return Ptypes { arity: pts.arity, ..Ptypes::wild() };
            }
            for p in PARITIES {
                let es = self.value_set(&[Feed::N(Node::E(x, p))]);
                let slot = &mut pts.slots[p as usize];
                slot.any |= !es.open.is_empty();
                for e in es.classes.iter() {
                    match self.mirrors.get(&e) {
                        Some(&c) => {
                            let n = &self.names[c as usize];
                            let d = if n.starts_with('[') { n.to_string() } else { format!("L{n};") };
                            slot.descs.insert(d);
                        }
                        None if Some(e) == self.prim_mirror => slot.prim = true,
                        None => slot.any = true,
                    }
                }
            }
        }
        pts
    }

    /// 查找名字值 v 的候选：常量；本方法 String 形参（各调用点给出的全部名字，须推得出）；拼接段。None = 推不出
    fn mark_names(&mut self, m: usize, v: &V) -> Option<Names> {
        if let V::Str(s, _) = v {
            return Some(Names::Exact([s.clone()].into()));
        }
        if let [Src::Param(i)] = &*v.srcs() {
            if let Some((names, complete)) = self.param_names(m, *i as usize, 0) {
                return complete.then_some(Names::Exact(names));
            }
        }
        self.method_name_parts(m, v).map(Names::Parts)
    }

    /// 按查找口径在类 cls 上解析名字 names、形参 pts 的方法（超集）；空 = 查找抛 NoSuchMethodException；None = 类不可得
    fn mark_resolve(&mut self, cls: &str, names: &Names, pts: &Ptypes, scope: LookupScope) -> Option<Vec<MemberRef>> {
        if cls.starts_with('[') {
            // 数组类没有声明方法；公开成员即根类的公开成员（JLS §10.7）
            return match scope {
                LookupScope::Public => self.mark_resolve(OBJECT, names, pts, scope),
                _ => Some(Vec::new()),
            };
        }
        let hit = |cf: &ClassFile, public: bool, statics: bool| -> Vec<MemberRef> {
            cf.methods
                .iter()
                .filter(|x| !x.name.starts_with('<') && names.admits(&x.name) && pts.admits(&x.desc))
                .filter(|x| !public || x.access & acc::PUBLIC != 0)
                .filter(|x| statics || !x.is_static())
                .map(|x| MemberRef { owner: cf.name.clone(), name: x.name.clone(), desc: x.desc.clone() })
                .collect()
        };
        let cf = self.h.class(cls)?;
        match scope {
            LookupScope::Declared => Some(hit(&cf, false, true)),
            LookupScope::DeclaredPublic => Some(hit(&cf, true, true)),
            LookupScope::Public => {
                // 类链上首个有命中的类；超接口的公开实例方法同样参与选择（静态接口方法不被继承），一并取入（超集）
                let mut out = Vec::new();
                let mut ifaces: Vec<String> = Vec::new();
                let mut cur = Some(cf);
                let mut hops = 0;
                while let Some(c) = cur {
                    ifaces.extend(c.interfaces.iter().cloned());
                    if out.is_empty() {
                        out = hit(&c, true, true);
                    }
                    hops += 1;
                    if hops > 64 {
                        return None;
                    }
                    cur = match &c.super_name {
                        Some(s) => Some(self.h.class(s)?),
                        None => None,
                    };
                }
                let mut seen = BTreeSet::new();
                while let Some(i) = ifaces.pop() {
                    if !seen.insert(i.clone()) {
                        continue;
                    }
                    let cf = self.h.class(&i)?;
                    out.extend(hit(&cf, true, false));
                    ifaces.extend(cf.interfaces.iter().cloned());
                }
                Some(out)
            }
        }
    }

    /// 复制调用点：输入（反射类型的形参，无则接收者）值集全是标记时并入结果；false = 不满足（回退）
    fn copy_marks(&mut self, m: usize, off: u32, opcode: u8, mref: &MemberRef, args: &[V], recv: &str) -> bool {
        let Some(md) = parse_method(&mref.desc) else { return false };
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let input = match md.params.iter().position(|p| matches!(p, FieldType::Object(c) if c == recv)) {
            Some(j) => args.get(base + j),
            None if base == 1 && mref.owner == recv => args.first(),
            None => None,
        };
        let Some(v) = input else { return false };
        let tid = self.id(recv);
        let fs = self.feeds(m, v, tid);
        let s = self.value_set(&fs);
        if !s.open.is_empty() || s.classes.iter().any(|x| !self.rmarks.contains_key(&x)) {
            return false;
        }
        self.add_to(Node::S(m, off), &s);
        true
    }

    /// 标记 x 所指的方法（缺口 = 空）；None = 不是标记。共享标记登记读取点 w（所指增长时重跑）
    pub(super) fn mark_targets(&mut self, x: u32, w: (usize, u32)) -> Option<Vec<MemberRef>> {
        Some(match self.rmarks.get(&x)? {
            MethodMark::Member(k) => vec![k.clone()],
            MethodMark::Gap => Vec::new(),
            MethodMark::All(_, _, scope) => {
                let scope = *scope;
                self.rmark_readers.entry(x).or_default().insert(w);
                let cs = self.rmark_all.get(&x).cloned().unwrap_or_default();
                cs.iter().flat_map(|c| self.all_methods(c, scope)).collect()
            }
        })
    }

    /// 类 cls 上按口径枚举的全部方法（超集：public 取类链与超接口上的全部公开方法）
    fn all_methods(&self, cls: &str, scope: LookupScope) -> Vec<MemberRef> {
        let cls = if cls.starts_with('[') {
            if scope != LookupScope::Public {
                return Vec::new();
            }
            OBJECT
        } else {
            cls
        };
        let mut out = Vec::new();
        let mut stack = vec![cls.to_string()];
        let mut seen = BTreeSet::new();
        while let Some(c) = stack.pop() {
            if !seen.insert(c.clone()) {
                continue;
            }
            let Some(cf) = self.h.class(&c) else { continue };
            let public = scope != LookupScope::Declared;
            out.extend(
                cf.methods
                    .iter()
                    .filter(|x| !x.name.starts_with('<') && (!public || x.access & acc::PUBLIC != 0))
                    .map(|x| MemberRef { owner: cf.name.clone(), name: x.name.clone(), desc: x.desc.clone() }),
            );
            if scope == LookupScope::Public {
                stack.extend(cf.super_name.iter().chain(cf.interfaces.iter()).cloned());
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(descs: &[&str], prim: bool) -> Slot {
        Slot { descs: descs.iter().map(|s| s.to_string()).collect(), prim, any: false }
    }

    #[test]
    fn ptypes_admit() {
        let p = Ptypes { arity: Some(1), slots: [slot(&["Ljava/io/Foo;"], false), slot(&[], false)] };
        assert!(p.admits("(Ljava/io/Foo;)V"));
        assert!(!p.admits("(Ljava/io/Bar;)V"));
        assert!(!p.admits("()V"));
        let p0 = Ptypes { arity: Some(0), ..Ptypes::default() };
        assert!(p0.admits("()Ljava/lang/Object;"));
        assert!(!p0.admits("(I)V"));
        // 长度不明：按奇偶槽逐位判定，基本类型取基本类型镜像通配
        let p = Ptypes { arity: None, slots: [slot(&["La;"], false), slot(&[], true)] };
        assert!(p.admits("()V"));
        assert!(p.admits("(La;I)V"));
        assert!(!p.admits("(La;Lb;)V"));
        assert!(Ptypes::wild().admits("(IJLa;[B)V"));
    }

    #[test]
    fn names_admit() {
        let n = Names::Exact([Rc::from("readObject")].into());
        assert!(n.admits("readObject"));
        assert!(!n.admits("writeObject"));
    }
}

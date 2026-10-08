//! 引擎：按字段句柄存取的调用点建模（清单 `[facts.field_writes] handle_getters / handle_setters`）。
//!
//! `Field.get(obj)` / `Field.set(obj, v)` 的字节码经 `getFieldAccessor()` 走 `FieldAccessor` 接口派发，
//! 各访问器实现再经 getter / setter 方法句柄的 `invokeExact` 存取。按字节码接边时，对象实参与写入值汇入
//! 接口派发汇合点（全部访问器实现）与 `invokeExact` 的手写值池，被反射读写的全部字段值并成一个集合，
//! 流回每个调用点并整组逃逸（c1d §30.17 成因 4）。
//!
//! 句柄身份由来源标记给出（`field_handles.rs`）：字段枚举的标记带口径类，按名取得的标记带所指字段。
//! 字段句柄只经清单的取得入口（`enumerators` / `handle = true` 的 `name_resolvers`）得到，其字节码调用点给结果
//! 带上标记，所以句柄值集里的标记即其所指字段的全集，句柄对象本身（字节码分配的 Field 副本）不再另给身份。
//! 本模块在调用点上按标记直接建模：
//! - 读：结果 = 对象实参里 ⊂ 声明类的抽象对象在所指引用实例字段上的值（逐对象），非抽象 / open 的对象实参
//!   取字段的未知接收者视图；静态字段取字段节点；基本类型字段取装箱值（被调方返回值中该装箱类的值：装箱在
//!   访问器字节码里分配，与对象实参无关）。枚举口径类 C 的所指字段为 C 及其超类的实例字段与 C 及其超类 /
//!   超接口的静态字段（`getFields` 含继承的公开字段与接口常量），按类粒度取上界；
//! - 写：写入值落到同一组字段（抽象对象逐对象，其余经未知接收者写入）。
//!
//! 对象实参与写入值不流入被调方形参（接收者照常流入：访问器建立、访问检查、类初始化与对象实参无关），
//! 存取效果由本模块给出。被调方字节码运行期照常使用这些形参，它们的空值集按未建模来源处理（不判 `null_recv`，
//! 见 [`Engine::field_access_cut_params`]）。下列情形按字节码接边：接收者是口径推不出的标记、open（非建模代码产出的句柄）；
//! 取得入口经非字节码调用点（反射调用、lambda、手写体）可达时句柄可能不带标记，全部站点改按字节码接边。

use super::*;

/// 站点键标记：调用点的对象实参汇集节点 `S(m, FA_OBJ | off)` 与写入值汇集节点 `S(m, FA_VAL | off)`
const FA_OBJ: u32 = 1 << 29;
const FA_VAL: u32 = (1 << 29) | (1 << 28);

/// 调用边上的句柄接收者
pub(super) enum FaRecv {
    /// 枚举口径类
    Class(u32),
    /// 按名所指字段
    Field(MemberRef),
    /// 字节码分配的句柄对象：身份由同一值集里的标记给出，本身不另存取
    Covered,
    /// 按字节码接边
    Bytecode,
}

/// 一组所指字段：声明类型、引用实例字段、引用静态字段、基本类型字段的装箱类
struct FaFields {
    owner: u32,
    inst: Vec<(usize, u32)>,
    stat: Vec<(usize, u32)>,
    boxes: Vec<u32>,
}

/// 一个按句柄存取的调用点
#[derive(Default)]
pub(super) struct FaSite {
    /// 被调方（同一调用点可有多个上下文节点）
    ts: Vec<usize>,
    write: bool,
    /// 已建模的所指字段组（键：口径类 / 字段节点）
    scopes: Vec<(FaKey, Rc<FaFields>)>,
    /// 读的结果节点
    res: Vec<Node>,
    /// 实参是否为引用（对象实参 / 写入值）
    obj_ref: bool,
    val_ref: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum FaKey {
    Class(u32),
    Field(usize),
}

impl<'a> Engine<'a> {
    /// 调用边 → t（句柄存取入口）上的接收者分类；t 不是句柄存取入口时为 None
    pub(super) fn field_access_recv(&mut self, t: usize, recv: &Recv) -> Option<FaRecv> {
        if !matches!(self.methods[t].ret_model, RetModel::HandleAccess(_)) {
            return None;
        }
        if self.fa_untrusted.is_some() {
            return Some(FaRecv::Bytecode);
        }
        let r = match recv {
            Recv::Exact(r) => r,
            // 非抽象对象的确定句柄值（如手写层产出的句柄类型值）同样由标记给出身份；open 来自非建模代码
            Recv::Feeds(fs) if fs.iter().all(|f| matches!(f, Feed::S(s) if s.open.is_empty())) => return Some(FaRecv::Covered),
            _ => return Some(FaRecv::Bytecode),
        };
        if let Some(sc) = self.fh_marks.get(r) {
            return Some(match sc.1.clone() {
                Some(c) => FaRecv::Class(self.id(&c)),
                None => FaRecv::Bytecode,
            });
        }
        if let Some(f) = self.fh_named.get(r) {
            return Some(match f {
                Some(f) => FaRecv::Field(f.clone()),
                None => FaRecv::Bytecode,
            });
        }
        Some(FaRecv::Covered)
    }

    /// 被切断的被调方形参：已建模站点的被调方（句柄存取入口）上未接实参的引用形参（对象实参 / 写入值）。
    /// 被调方字节码运行期照常执行并使用这些形参（`value.getClass().getName()` 构造异常消息、`value instanceof Xxx`
    /// 分支……），流图里它们的值集只缺这些调用点的实参——空集是「值流缺失」，不是 null。`unmodeled.rs` 把它们
    /// 作为未建模来源：依赖空集的 `null_recv` 折叠在它们及其下游（访问器形参、收窄值……）上一律不判。
    /// 已整体回放为字节码接边（`fa_untrusted`）时实参都已接上，无切断
    pub(super) fn field_access_cut_params(&self) -> Vec<Node> {
        if self.fa_untrusted.is_some() {
            return Vec::new();
        }
        let mut out: Vec<Node> = Vec::new();
        for s in self.fa_sites.values() {
            for &t in &s.ts {
                let base = usize::from(!self.methods[t].is_static);
                let pts = &self.methods[t].ptypes;
                out.extend((base..pts.len()).filter(|&i| pts[i].is_some()).map(|i| Node::P(t, i as u16)));
            }
        }
        out.sort_unstable();
        out.dedup();
        out
    }

    /// 字段句柄取得入口经非字节码调用点可达：句柄可能不带来源标记，已建模的站点改按字节码接边
    pub(super) fn field_source_edge(&mut self, key: &MemberRef, via: &Via) {
        if !self.man.is_field_handle_source(key) {
            return;
        }
        if let From::Method(c) = via.from {
            if via.off.is_some() && matches!(via.kind, "invoke" | "dispatch") && self.methods[c].kind == Kind::Bytecode {
                return;
            }
        }
        let from = match via.from {
            From::Method(c) => self.methods[c].key.to_string(),
            From::Root(ref r) | From::Class(ref r) => r.clone(),
        };
        self.fa_untrusted = Some(format!("{key} <- {} {from}", via.kind));
        let mut keys: Vec<(usize, u32)> = self.fa_sites.keys().copied().collect();
        keys.sort_unstable();
        for (m, off) in keys {
            let s = &self.fa_sites[&(m, off)];
            let (ts, res, write, obj_ref, val_ref) = (s.ts.clone(), s.res.clone(), s.write, s.obj_ref, s.val_ref);
            for t in ts {
                let base = usize::from(!self.methods[t].is_static);
                let pt = |e: &Self, i: usize| e.methods[t].ptypes.get(base + i).copied().flatten();
                if obj_ref {
                    if let Some(p) = pt(self, 0) {
                        self.flow(Node::S(m, FA_OBJ | off), Node::P(t, base as u16), p);
                    }
                }
                if write && val_ref {
                    if let Some(p) = pt(self, 1) {
                        self.flow(Node::S(m, FA_VAL | off), Node::P(t, (base + 1) as u16), p);
                    }
                }
                if let Some(rt) = self.methods[t].rtype {
                    for &r in &res {
                        self.flow(Node::R(t), r, rt);
                    }
                }
            }
        }
    }

    /// 在调用点 (m, off) 上按句柄接收者 k 建模存取（a：实参来源，不含接收者；res：读的结果节点）
    pub(super) fn field_access_site(&mut self, m: usize, off: u32, t: usize, k: FaRecv, a: &[Option<Vec<Feed>>], res: Option<Node>) {
        let RetModel::HandleAccess(write) = self.methods[t].ret_model else { return };
        let obj = self.id(OBJECT);
        let on = Node::S(m, FA_OBJ | off);
        let vn = Node::S(m, FA_VAL | off);
        let key = (m, off);
        if !self.fa_sites.contains_key(&key) {
            self.fa_sites.insert(key, FaSite { write, ..Default::default() });
            self.fa_watch.insert(on, key);
            self.graph.mark_hooked(on);
        }
        let ofs = a.first().cloned().flatten();
        let vfs = if write { a.get(1).cloned().flatten() } else { None };
        {
            let s = self.fa_sites.get_mut(&key).expect("fa site");
            if !s.ts.contains(&t) {
                s.ts.push(t);
            }
            s.obj_ref |= ofs.is_some();
            s.val_ref |= vfs.is_some();
        }
        if let Some(fs) = &ofs {
            self.feed(fs, on, obj);
        }
        if let Some(fs) = &vfs {
            self.feed(fs, vn, obj);
        }
        let new_res = res.filter(|r| !write && !self.fa_sites[&key].res.contains(r));
        if let Some(r) = new_res {
            self.fa_sites.get_mut(&key).expect("fa site").res.push(r);
        }
        let scope = match k {
            FaRecv::Class(c) => Some(FaKey::Class(c)),
            FaRecv::Field(f) => Some(FaKey::Field(self.field_node(f))),
            FaRecv::Covered | FaRecv::Bytecode => None,
        };
        let new_scope = scope.filter(|sk| !self.fa_sites[&key].scopes.iter().any(|(x, _)| x == sk));
        let objs = self.set_of(on);
        if let Some(sk) = new_scope {
            let fields = Rc::new(self.fa_fields(sk));
            self.fa_sites.get_mut(&key).expect("fa site").scopes.push((sk, fields.clone()));
            let outs = if write { vec![vn] } else { self.fa_sites[&key].res.clone() };
            self.field_access_apply(key, &[fields], &outs, &objs, true);
        }
        if let Some(r) = new_res {
            let scopes: Vec<Rc<FaFields>> = self.fa_sites[&key].scopes.iter().filter(|(x, _)| Some(*x) != new_scope).map(|(_, f)| f.clone()).collect();
            self.field_access_apply(key, &scopes, &[r], &objs, true);
        }
    }

    /// 对象实参汇集节点新增 delta
    pub(super) fn field_access_grown(&mut self, n: Node, delta: &TypeSet) {
        let Some(&key) = self.fa_watch.get(&n) else { return };
        let s = &self.fa_sites[&key];
        let scopes: Vec<Rc<FaFields>> = s.scopes.iter().map(|(_, f)| f.clone()).collect();
        let outs = if s.write { vec![Node::S(key.0, FA_VAL | key.1)] } else { s.res.clone() };
        self.field_access_apply(key, &scopes, &outs, delta, false);
    }

    /// 所指字段组 scopes × 输出 outs 在对象实参值 objs 上接边；statics：另接静态字段与装箱值（与对象实参无关）
    fn field_access_apply(&mut self, key: (usize, u32), scopes: &[Rc<FaFields>], outs: &[Node], objs: &TypeSet, statics: bool) {
        if scopes.is_empty() || outs.is_empty() {
            return;
        }
        let (ts, write) = {
            let s = &self.fa_sites[&key];
            (s.ts.clone(), s.write)
        };
        for fs in scopes {
            let c = fs.owner;
            let mut nodes: Vec<(Node, u32)> = Vec::new();
            if !fs.inst.is_empty() {
                // 对象实参：⊂ 声明类的抽象对象逐对象；其余（非抽象对象的类、open）取未知接收者视图
                let mut other = false;
                for x in objs.classes.iter() {
                    if self.arrays.contains_key(&x) || self.mirrors.contains_key(&x) {
                        continue;
                    }
                    match self.objs.get(&x).copied() {
                        Some(cls) => {
                            if self.sub(cls, c) {
                                for &(fi, tid) in &fs.inst {
                                    nodes.push((self.obj_field(x, fi, tid), tid));
                                }
                            }
                        }
                        None => other |= self.sub(x, c),
                    }
                }
                other |= objs.open.iter().any(|o| self.sub(o, c) || self.sub(c, o));
                if other {
                    for &(fi, tid) in &fs.inst {
                        nodes.push((if write { Node::U(fi) } else { Node::F(fi) }, tid));
                    }
                }
            }
            if statics {
                for &(fi, tid) in &fs.stat {
                    nodes.push((if write { Node::U(fi) } else { Node::F(fi) }, tid));
                }
                if !write {
                    for &b in &fs.boxes {
                        for &t in &ts {
                            for &o in outs {
                                self.flow(Node::R(t), o, b);
                            }
                        }
                    }
                }
            }
            for &o in outs {
                for &(n, tid) in &nodes {
                    if write {
                        self.flow(o, n, tid);
                    } else {
                        self.flow(n, o, tid);
                    }
                }
            }
        }
    }

    /// 所指字段组：口径类 c（实例字段取 c 及其超类，静态字段与装箱另含超接口）/ 单个字段
    fn fa_fields(&mut self, k: FaKey) -> FaFields {
        match k {
            FaKey::Class(c) => {
                let inst = self.ref_fields(c).to_vec();
                let mut stat = Vec::new();
                let mut prims: BTreeSet<u8> = BTreeSet::new();
                for h in self.holder_chain(c) {
                    stat.extend(self.static_ref_fields(h));
                    let Some(cf) = self.h.class(&self.names[h as usize].clone()) else { continue };
                    for f in &cf.fields {
                        if let [p] = f.desc.as_bytes() {
                            prims.insert(*p);
                        }
                    }
                }
                let boxes = self.box_ids(prims);
                FaFields { owner: c, inst, stat, boxes }
            }
            FaKey::Field(fi) => {
                let f = self.fields.get_index(fi).map(|(k, _)| k.clone()).expect("field node");
                let owner = self.id(&f.owner);
                let is_static = self.h.class(&f.owner).and_then(|cf| cf.field(&f.name, &f.desc).map(|x| x.is_static())).unwrap_or(false);
                let mut out = FaFields { owner, inst: vec![], stat: vec![], boxes: vec![] };
                match parse_field(&f.desc).and_then(|t| self.ptype(&t)) {
                    Some(tid) if is_static => out.stat.push((fi, tid)),
                    Some(tid) => out.inst.push((fi, tid)),
                    None => out.boxes = self.box_ids(f.desc.bytes().take(1).collect()),
                }
                out
            }
        }
    }

    fn box_ids(&mut self, prims: BTreeSet<u8>) -> Vec<u32> {
        let names: Vec<String> = prims.into_iter().filter_map(|p| self.man.boxed_class(p).map(str::to_string)).collect();
        names.iter().map(|n| self.id(n)).collect()
    }

    /// 类 c 及其超类、超接口
    fn holder_chain(&mut self, c: u32) -> Vec<u32> {
        let mut out = Vec::new();
        let mut todo = vec![self.names[c as usize].to_string()];
        let mut seen: HashSet<String> = HashSet::default();
        while let Some(n) = todo.pop() {
            if !seen.insert(n.clone()) {
                continue;
            }
            if let Some(cf) = self.h.class(&n) {
                todo.extend(cf.super_name.iter().cloned());
                todo.extend(cf.interfaces.iter().cloned());
            }
            out.push(self.id(&n));
        }
        out
    }
}

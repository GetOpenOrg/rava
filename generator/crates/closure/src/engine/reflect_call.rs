//! 引擎：反射调用——按通道的实参池与按接收者派发。
//!
//! 反射成员（按名查找 / 枚举入链的方法）只经反射调用执行：方法反射调用入口（清单 `method_invokers`）。
//! 入口分两条通道，各有实参池 [`Node::RP`]：
//! - 反射对象通道：入口声明了 Object / Object[] 以外的引用形参（反射对象本身，如按 Method 调用的 native）；
//!   按名查找返回该类型（`getMethod` / `getDeclaredMethod`）或成员枚举得到的方法只经此通道调用；
//! - 方法句柄通道：其余入口（签名多态调用点，实参按 Object）；按名查找返回其他类型（方法句柄 / MemberName）的
//!   方法经此通道调用。
//!
//! 两条通道的实参互不流通：反射对象只能经反射对象入口调用，方法句柄只能经句柄入口调用（反射对象内部转成的
//! 句柄调用，其实参与外层反射对象入口的实参相同，已由反射对象通道覆盖）。入口调用点上声明为 Object / Object[]
//! 的实参（数组另取元素）汇入其通道的实参池，再按声明类型流向该通道反射成员的形参。可被覆写的实例方法按池中
//! 接收者逐个选中实现（与字节码虚调用同口径，容器对象按接收者克隆上下文）；池中 open 的部分退回 VM 枢纽。

use super::*;

/// 反射调用通道：反射对象 / 方法句柄
pub(super) const RC_OBJ: u8 = 0;
pub(super) const RC_HANDLE: u8 = 1;

/// 按池中接收者派发的反射实例成员
#[derive(Clone)]
pub(super) struct RcallMember {
    pub(super) key: MemberRef,
    pub(super) iface: bool,
    pub(super) ch: u8,
    /// 可覆写：按接收者选中实现；否则（私有 / final）实现就是成员本身，只按接收者克隆上下文
    pub(super) virt: bool,
}

/// 通道位
pub(super) const fn rc_bit(c: u8) -> u8 {
    1 << c
}

impl<'a> Engine<'a> {
    /// 反射对象类型：方法反射调用入口描述符里 Object 以外的引用形参类型（按 Method 调用的入口的 Method）
    fn rcall_obj_types(&mut self) -> Rc<HashSet<String>> {
        if let Some(s) = &self.rcall_obj_types {
            return s.clone();
        }
        let mut out = HashSet::default();
        for k in self.man.method_invoker_keys() {
            let Some(md) = k.split_once(':').and_then(|(_, d)| parse_method(d)) else { continue };
            for p in &md.params {
                if let FieldType::Object(c) = p {
                    if c != OBJECT {
                        out.insert(c.clone());
                    }
                }
            }
        }
        let s = Rc::new(out);
        self.rcall_obj_types = Some(s.clone());
        s
    }

    /// 按名查找方法 desc 的结果经哪条通道调用：返回反射对象类型的走反射对象通道，其余走方法句柄通道
    pub(super) fn rcall_lookup_channel(&mut self, desc: &str) -> u8 {
        let ret = parse_method(desc).and_then(|md| md.ret);
        let objs = self.rcall_obj_types();
        match ret {
            Some(FieldType::Object(c)) if objs.contains(&c) => RC_OBJ,
            _ => RC_HANDLE,
        }
    }

    /// 方法 t 是反射调用入口时的通道
    fn rcall_entry(&mut self, t: usize) -> Option<u8> {
        if let Some(&b) = self.rcall_entry.get(&t) {
            return b;
        }
        let b = if self.man.member_invoker(&self.methods[t].key.to_string()).contains(&Members::Methods) {
            let objs = self.rcall_obj_types();
            let desc = self.methods[t].key.desc.clone();
            let obj_param = parse_method(&desc).is_some_and(|md| md.params.iter().any(|p| matches!(p, FieldType::Object(c) if objs.contains(c))));
            Some(if obj_param { RC_OBJ } else { RC_HANDLE })
        } else {
            None
        };
        self.rcall_entry.insert(t, b);
        b
    }

    /// 调用边 m@off → t：t 是反射调用入口时，实参接入其通道的实参池（逐调用点一次）
    pub(super) fn rcall_site(&mut self, m: usize, off: u32, t: usize, a: &[Option<Vec<Feed>>]) {
        if self.rcall_sites.contains_key(&(m, off, t)) {
            return;
        }
        let Some(ch) = self.rcall_entry(t) else { return };
        let s = self.rcall_sites.len() as u32;
        self.rcall_sites.insert((m, off, t), s);
        self.rcall_chan.push(ch);
        let obj = self.id(OBJECT);
        let objs = self.id(&format!("[L{OBJECT};"));
        let poly = self.is_poly(t);
        let base = usize::from(!self.methods[t].is_static);
        let ptypes = self.methods[t].ptypes.clone();
        for (j, f) in a.iter().enumerate() {
            let Some(fs) = f else { continue };
            let pt = if poly { Some(obj) } else { ptypes.get(base + j).copied().flatten() };
            if pt.is_some_and(|p| p == obj || p == objs) {
                self.feed(fs, Node::RA(s, j as u16), obj);
            }
        }
    }

    /// 入口调用点 s 的实参新增值：并入该通道的实参池；数组的元素（分配点元素节点；open 数组按元素类型 open）一并并入
    pub(super) fn rcall_arg_grown(&mut self, s: u32, delta: &TypeSet) {
        let pool = Node::RP(self.rcall_chan[s as usize]);
        let obj = self.id(OBJECT);
        self.add_to(pool, delta);
        let xs: Vec<u32> = delta.classes.iter().filter(|x| self.arrays.contains_key(x)).collect();
        for x in xs {
            for p in PARITIES {
                self.flow(Node::E(x, p), pool, obj);
            }
        }
        let mut comps = TypeSet::default();
        for o in delta.open.iter() {
            let name = self.names[o as usize].clone();
            if let Some(c) = absint::component(&name) {
                comps.open.insert(self.id(&c));
            }
        }
        self.add_to(pool, &comps);
    }

    /// 反射成员 t 经通道 ch 入链：形参接该通道实参池；实例方法的接收者由 [`Self::rcall_recv`] 按池中接收者派发
    pub(super) fn rcall_target(&mut self, t: usize, ch: u8) {
        self.rcall_bind(t, ch);
    }

    /// 形参（不含接收者）接通道 ch 的实参池，形参常量为未知
    fn rcall_bind(&mut self, t: usize, ch: u8) {
        if !self.rcall_bound.insert((t, ch)) {
            return;
        }
        let base = usize::from(!self.methods[t].is_static);
        let pts = self.methods[t].ptypes.clone();
        self.bind_pvs(t, base, pts.len(), None);
        for (i, pt) in pts.iter().enumerate().skip(base) {
            if let Some(pt) = pt {
                self.flow(Node::RP(ch), Node::P(t, i as u16), *pt);
            }
        }
    }

    /// 实例反射成员经通道 ch：按该通道实参池里的接收者（含之后新增的）派发——可覆写的选中实现，
    /// 不可覆写的（私有 / final，如序列化回调 `writeObject`）就是成员本身。两者都按接收者克隆上下文
    /// （与字节码调用同口径）：同一成员被多个容器对象经反射调用时各对象的字段 / 数组互不汇合
    pub(super) fn rcall_recv(&mut self, m: RcallMember) {
        let ch = m.ch;
        self.rcall_virt.push(m);
        let i = self.rcall_virt.len() - 1;
        let s = self.set_of(Node::RP(ch));
        self.rcall_dispatch(i, &s);
    }

    /// 通道 ch 的实参池新增值：该通道各可覆写反射成员按新增接收者派发
    pub(super) fn rcall_grown(&mut self, ch: u8, delta: &TypeSet) {
        for i in 0..self.rcall_virt.len() {
            if self.rcall_virt[i].ch == ch {
                self.rcall_dispatch(i, delta);
            }
        }
    }

    fn rcall_dispatch(&mut self, i: usize, s: &TypeSet) {
        let RcallMember { key, iface, ch, virt } = self.rcall_virt[i].clone();
        let owner = self.id(&key.owner);
        let via = Via::class("reflect", &key.owner);
        let site = if virt {
            let Some(site) = self.h.resolve_method(&key.owner, &key.name, &key.desc, iface) else { return };
            Some(site)
        } else {
            None
        };
        let xs: Vec<u32> = s.classes.iter().collect();
        let opens: Vec<u32> = s.open.iter().filter(|&o| self.sub(o, owner) || self.sub(owner, o)).collect();
        let mut fallback = virt && !opens.is_empty();
        if !virt && !opens.is_empty() {
            // 不可覆写成员：所指未知的接收者进成员本体
            let t = self.method(key.clone(), via.clone());
            let o = TypeSet { classes: IdSet::default(), open: IdSet::from_sorted(opens) };
            let o = self.filter(&o, owner);
            self.add_to(Node::P(t, 0), &o);
        }
        for x in xs {
            if !self.sub(x, owner) || !self.rcall_done.insert((i, x)) {
                continue;
            }
            let k = match &site {
                Some(site) => {
                    // lambda / 手写实现对象的反射调用不在字节码层选中实现：退回 VM 枢纽
                    if self.lambdas.contains_key(&x) || self.hwobjs.contains_key(&x) {
                        fallback = true;
                        continue;
                    }
                    let rt = self.ty(x);
                    let rname = self.names[rt as usize].to_string();
                    let Some(sel) = self.h.select(&rname, site) else {
                        self.unresolved.insert(format!("select {rname} {}", key.name));
                        continue;
                    };
                    let (o, n, d) = sel.key();
                    MemberRef { owner: o, name: n, desc: d }
                }
                None => key.clone(),
            };
            let cx = self.recv_ctx(x);
            let t = self.method_ctx(k, cx, via.clone());
            self.vm_targets.insert(t);
            self.add_to(Node::P(t, 0), &TypeSet::exact(x));
            self.rcall_bind(t, ch);
        }
        if fallback {
            self.vm_dispatch(&key, iface, via);
        }
    }
}

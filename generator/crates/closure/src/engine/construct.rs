//! 常量查询：对象字段值——构造器摘要（final 实例字段常量）与按对象标签的字段读折叠。
//!
//! `new C(实参…)` 返回时，absint 以构造器摘要给对象打 `Obj::Fields` 标签：构造器（按调用点的常量实参
//! 单独分析）在 `this` 上写入的本类 final 实例字段值；委托给本类另一构造器（`this(…)`）的按被委托构造器
//! 的摘要合并。final 实例字段只在本类构造器内写入、此后不变（反射 / Unsafe / 反序列化写入由
//! `field_open` 在读取时排除），于是标签对象上的 getfield 可在任意方法中按标签折叠——
//! record 的规范构造器、配置对象（`new Config(常量…)`）即此形态。

use super::*;

impl Ctx<'_> {
    /// `new` 出的对象经构造器 init（实参含接收者）完成后的标签：final 实例字段常量（推不出任何常量 → None）
    pub(super) fn construct(&self, init: &MemberRef, args: &[V]) -> Option<Rc<Obj>> {
        let rest: Vec<V> = args.iter().skip(1).map(V::stripped).collect();
        let key = format!("{init}|{rest:?}");
        if let Some(o) = self.objs.borrow().get(&key) {
            return o.clone();
        }
        let fs = self.ctor_fields(init, &rest)?;
        let vals: Vec<(MemberRef, V)> = fs.into_iter().filter_map(|(k, pv)| pv.value().map(|v| (k, v))).collect();
        let o = (!vals.is_empty()).then(|| Rc::new(Obj::Fields(vals)));
        self.objs.borrow_mut().insert(key, o.clone());
        o
    }

    /// 构造器 init 以实参 rest（不含接收者）写入的本类 final 实例字段（按字段键排序）；
    /// 分析不了（边界 / 手写 / 递归中 / 保守分析）→ None
    fn ctor_fields(&self, init: &MemberRef, rest: &[V]) -> Option<Vec<(MemberRef, PV)>> {
        let cf = self.h.class(&init.owner)?;
        let finals: Vec<MemberRef> = cf
            .fields
            .iter()
            .filter(|f| f.access & acc::FINAL != 0 && f.access & acc::STATIC == 0)
            .map(|f| MemberRef { owner: cf.name.clone(), name: f.name.clone(), desc: f.desc.clone() })
            .collect();
        if finals.is_empty() {
            return None;
        }
        let meth = cf.method(&init.name, &init.desc)?;
        if self.kind_of(&cf, meth) != Kind::Bytecode {
            return None;
        }
        let code = meth.code.as_ref()?;
        let guard = format!("<init>:{init}");
        if !self.in_progress.borrow_mut().insert(guard.clone()) {
            return None;
        }
        let params: Vec<Option<V>> = std::iter::once(None).chain(rest.iter().map(|v| PV::of(v).value())).collect();
        let live = |_: &str| true;
        let a = absint::analyze(&cf.name, &init.desc, false, code, &Facts { ctx: self, live: &live, m: None, params });
        let out = (!a.conservative).then(|| self.ctor_puts(&cf.name, &finals, &a)).flatten();
        self.in_progress.borrow_mut().remove(&guard);
        out
    }

    /// 构造器分析事件里 `this` 上的 final 字段写入（含 `this(…)` 委托），逐字段并值
    fn ctor_puts(&self, cls: &str, finals: &[MemberRef], a: &Analysis) -> Option<Vec<(MemberRef, PV)>> {
        let this = |v: &V| matches!(&*v.srcs(), [Src::Param(0)]);
        let mut vals: BTreeMap<MemberRef, PV> = BTreeMap::new();
        let mut put = |k: MemberRef, pv: PV| {
            let cur = vals.get(&k).cloned();
            vals.insert(k, PV::join(cur.as_ref(), &pv));
        };
        for (_, e) in &a.events {
            match e {
                Event::Field { opcode: classfile::op::PUTFIELD, mref, recv, value } => {
                    let Some(fi) = self.field_info(mref) else { continue };
                    if !finals.contains(&fi.key) {
                        continue;
                    }
                    // 写到本类另一实例上的 final 字段：该字段值不再只由本构造器的 this 决定
                    let pv = match (recv, value) {
                        (Some(r), Some(v)) if this(r) => PV::of(v),
                        _ => PV::Top,
                    };
                    put(fi.key.clone(), pv);
                }
                Event::Invoke { opcode: classfile::op::INVOKESPECIAL, mref, args, .. }
                    if mref.name == "<init>" && mref.owner == cls && args.first().is_some_and(this) =>
                {
                    let rest: Vec<V> = args.iter().skip(1).map(V::stripped).collect();
                    let Some(fs) = self.ctor_fields(mref, &rest) else {
                        finals.iter().for_each(|k| put(k.clone(), PV::Top));
                        continue;
                    };
                    let known: BTreeSet<&MemberRef> = fs.iter().map(|(k, _)| k).collect();
                    for k in finals.iter().filter(|k| !known.contains(k)) {
                        put(k.clone(), PV::Top);
                    }
                    for (k, pv) in fs {
                        put(k, pv);
                    }
                }
                _ => {}
            }
        }
        Some(vals.into_iter().collect())
    }

    /// 带对象标签的字段读：标签对象的 final 实例字段；系统属性表持有字段 → 属性表对象
    pub(super) fn object_field(&self, m: Option<usize>, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        let fi = self.field_info(f)?;
        if opcode == classfile::op::GETSTATIC {
            // 持有字段由 VM 启动时填充（手写写入是 VM 初始化本身，不算改写）；字节码对它的写入由改写判定计入
            return self.man.sysprops.is_holder(&fi.key.to_string()).then(|| self.sysprops_ref(&fi.key.desc));
        }
        let o = recv.and_then(V::obj)?;
        let v = o.field(&fi.key)?.clone();
        if let Some(m) = m {
            self.fdeps.borrow_mut().entry(fi.key.clone()).or_default().insert(m);
            self.pdeps.borrow_mut().insert(m);
        }
        // 反序列化只写它自己分配的对象（不经构造器），构造器建出的标签对象不受其影响：不看 `deser`
        let open = fi.open || self.fopen_all.get() || self.fopen.borrow().contains(&fi.key) || self.fopen_names.borrow().contains(&fi.key.name);
        (!open).then_some(v)
    }

    /// 系统属性表对象（描述符给出静态类型）
    pub(super) fn sysprops_ref(&self, desc: &str) -> V {
        let ty = parse_field(desc).and_then(|ft| match ft {
            FieldType::Object(c) => Some(Rc::from(c.as_str())),
            _ => None,
        });
        V::Ref { ty, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::SysProps)) }
    }
}

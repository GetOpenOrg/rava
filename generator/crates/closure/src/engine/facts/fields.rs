//! 字段事实：开放判定（字节码外写入来源）、字段解析、字段读常量、static final 常量与按名字段偏移。

use super::*;

impl Ctx<'_> {
    /// 字段的写入来源超出字节码（手写 / 边界类 / 反射 / 反序列化）：不折叠
    pub(in crate::engine) fn field_open(&self, fi: &FieldInfo) -> bool {
        self.field_open_under(fi, self.fopen_all.get(), self.deser.get())
    }

    /// 按给定的全局开关（全部放开 / 反序列化可达）判定字段是否不折叠：偏移可得的字段（[`Self::field_offset_under`]）
    /// 加上手写体写入的字段
    pub(in crate::engine) fn field_open_under(&self, fi: &FieldInfo, all: bool, deser: bool) -> bool {
        fi.open || self.field_offset_under(fi, all, deser) || self.hw_written(fi)
    }

    /// 可序列化字段：所属类可序列化、非 static、非 transient（默认序列化与反序列化按偏移读写的字段面）
    pub(in crate::engine) fn serial_field(fi: &FieldInfo) -> bool {
        deser_writes(fi.access, fi.serializable)
    }

    /// 字段被手写体写入（只不折叠，不使偏移可得）
    pub(in crate::engine) fn hw_written(&self, fi: &FieldInfo) -> bool {
        self.fhw.borrow().contains(&fi.key) || self.fhw_names.borrow().contains(&fi.key.name)
    }

    /// 按给定的全局开关判定字段偏移可经字节码外途径取得（按名取得 / 字段枚举 / 反序列化），从而可按偏移读写。
    /// 手写体写入与 [`FieldInfo::open`] 不算：手写 / VM 落地按 Rust 字段直接写入，不产出偏移
    pub(in crate::engine) fn field_offset_under(&self, fi: &FieldInfo, all: bool, deser: bool) -> bool {
        all
            || self.fopen.borrow().contains(&fi.key)
            || self.fopen_names.borrow().contains(&fi.key.name)
            || deser && deser_writes(fi.access, fi.serializable)
    }

    pub(in crate::engine) fn field_info(&self, f: &MemberRef) -> Option<Rc<FieldInfo>> {
        if let Some(fi) = self.fields.borrow().get(f) {
            return fi.clone();
        }
        let fi = self.h.resolve_field(&f.owner, &f.name, &f.desc).map(|site| {
            let fd = site.field();
            let key = MemberRef { owner: site.class.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
            // VM 注入的静态字段：运行期值由 VM 写入，字节码初值 / ConstantValue 均不代表运行期值
            let injected = self.man.is_injected_static(&key.owner, &key.name);
            // VM 状态字段（清单字段钩子）由钩子落地写入，同属字节码外的写入来源
            let open = injected
                || matches!(self.domain(&key.owner), Domain::Boundary | Domain::Root)
                || !self.hw.member(&key.owner, &key.name).fns.is_empty()
                || self.man.vm_state.field_hook(&key.owner, &key.name, &key.desc).is_some();
            let constant = if injected { None } else { fd.constant_value.clone() };
            let markers = self.man.serializable_markers();
            let serializable = markers.is_empty() || markers.iter().any(|x| self.h.is_subtype(&key.owner, x));
            Rc::new(FieldInfo { key, access: fd.access, constant, open, serializable })
        });
        self.fields.borrow_mut().insert(f.clone(), fi.clone());
        fi
    }

    /// 字段读的常量值；方法 m 登记为该字段的读者
    pub(in crate::engine) fn field_value(&self, m: Option<usize>, f: &MemberRef) -> Option<V> {
        let fi = self.field_info(f)?;
        if let Some(m) = m {
            self.dep(m, Dep::Field(fi.key.clone()));
        }
        // VM 注入的字面量值：字节码写入被 VM 值覆盖，读取恒为该值
        if let Some(x) = self.man.injected_literal(&fi.key.owner, &fi.key.name) {
            return Some(if fi.key.desc == "J" { V::Long(x) } else { V::Int(x as i32) });
        }
        if self.field_open(&fi) {
            return None;
        }
        if m.is_none() {
            self.note_aux_read(&fi.key);
        }
        if fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 {
            return self.static_const(m, &fi.key, fi.constant.as_ref());
        }
        m?;
        self.fvals.borrow().get(&fi.key).cloned().unwrap_or_else(|| default_pv(&fi.key.desc)).value()
    }

    /// 按名取字段偏移的调用折叠为符号偏移：Class 实参是类字面量、名字是字符串常量，且该类自身声明了
    /// 同名实例字段（VM 只查声明类本身，查不到即抛出）
    pub(in crate::engine) fn field_offset(&self, opcode: u8, r: crate::manifest::NameResolver, args: &[V]) -> Option<V> {
        let base = usize::from(opcode != classfile::op::INVOKESTATIC);
        let cls = match r.class {
            Some(i) => args.get(i + base)?,
            None => args.first()?,
        };
        let (V::Class(c, _), Some(V::Str(name, _))) = (cls, args.get(r.name + base)) else { return None };
        let cf = self.h.class(c)?;
        let fd = cf.fields.iter().find(|f| f.name == **name && !f.is_static())?;
        Some(V::Offset(Rc::new(MemberRef { owner: cf.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() })))
    }

    /// static final 字段：ConstantValue，或 `<clinit>` 唯一一次常量赋值
    pub(in crate::engine) fn static_const(&self, me: Option<usize>, key: &MemberRef, cv: Option<&Const>) -> Option<V> {
        if let Some(c) = cv {
            return const_value(c);
        }
        let hit = self.consts.borrow().get(key).cloned();
        if let Some((v, inp)) = hit {
            self.memo_use(me, &inp);
            return v;
        }
        // `<clinit>` 唯一一次常量赋值（递归保护：分析中的类不再展开）。
        // 一次分析得出本类全部 static final 字段的答复，与外层无关时逐字段缓存
        let cls = self.h.class(&key.owner)?;
        let frame = self.memo_enter(format!("clinit:{}", cls.name), true)?;
        let mut puts: HashMap<(&str, &str), Vec<Option<V>>> = HashMap::default();
        let a = cls.method("<clinit>", "()V").and_then(|m| m.code.as_ref()).map(|code| {
            let live = |_: &str| true;
            self.aux_analyze(&cls.name, "()V", true, code, &Facts { ctx: self, live: &live, m: None, params: vec![], mirrors: vec![], objs: Default::default() })
        });
        for (_, e) in a.iter().flat_map(|a| &a.events) {
            if let Event::Field { opcode: classfile::op::PUTSTATIC, mref, value, .. } = e {
                if mref.owner == cls.name {
                    puts.entry((&mref.name, &mref.desc)).or_default().push(value.clone());
                }
            }
        }
        let (clean, inp) = self.memo_leave(frame);
        self.memo_use(me, &inp);
        let value_of = |name: &str, desc: &str| match puts.get(&(name, desc)).map(Vec::as_slice) {
            Some([Some(v)]) => PV::of(v).value(),
            _ => None,
        };
        if !clean {
            return value_of(&key.name, &key.desc);
        }
        let mut consts = self.consts.borrow_mut();
        for fd in &cls.fields {
            if fd.access & acc::STATIC == 0 || fd.access & acc::FINAL == 0 || fd.constant_value.is_some() {
                continue;
            }
            let k = MemberRef { owner: cls.name.clone(), name: fd.name.clone(), desc: fd.desc.clone() };
            consts.insert(k, (value_of(&fd.name, &fd.desc), inp.clone()));
        }
        consts.get(key).and_then(|e| e.0.clone())
    }
}

/// 反序列化可写的字段：非 static、非 transient，且声明类可序列化（非可序列化超类的字段由其无参构造器初始化，走字节码）
fn deser_writes(access: u16, serializable: bool) -> bool {
    serializable && access & (acc::STATIC | acc::TRANSIENT) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deser_writes_rule() {
        assert!(deser_writes(acc::PRIVATE, true));
        assert!(!deser_writes(acc::PRIVATE, false));
        assert!(!deser_writes(acc::STATIC, true));
        assert!(!deser_writes(acc::TRANSIENT, true));
    }

    /// 返回常量格合流：常量串 × 带对象标签的新建对象（`StringLatin1.newString` 形态）两侧都非空时取非空引用；
    /// 任一侧可空时仍取 Top
    #[test]
    fn join_ret_keeps_nonnull() {
        let s = PV::Const(V::Str(Rc::from(""), Rc::from([].as_slice())));
        let tagged = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(crate::absint::Obj::Empty)) };
        let o = PV::Const(tagged.clone());
        assert_eq!(PV::join(Some(&s), &o), PV::Top);
        assert_eq!(PV::join_ret(Some(&s), &o), PV::Const(nonnull_ref()));
        let maybe = PV::Const(V::Ref { ty: None, nonnull: false, src: Rc::from([].as_slice()), obj: Some(Rc::new(crate::absint::Obj::Empty)) });
        assert_eq!(PV::join_ret(Some(&s), &maybe), PV::Top);
        assert_eq!(PV::join_ret(None, &o), o);
    }
}

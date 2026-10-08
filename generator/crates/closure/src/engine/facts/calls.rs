//! 调用点的静态摘要（清单事实 + 唯一字节码目标）。

use super::*;

fn fact_value(f: &Fact) -> V {
    match f {
        Fact::Null => V::Null,
        Fact::Int(i) => V::Int(*i),
    }
}

impl Ctx<'_> {
    /// 调用点的静态摘要（清单事实 + 唯一字节码目标）
    pub(in crate::engine) fn call_info(&self, opcode: u8, m: &MemberRef, iface: bool) -> Rc<CallInfo> {
        if let Some(c) = self.calls.borrow().get(m).and_then(|v| v.iter().find(|x| x.0 == opcode && x.1 == iface)) {
            return c.2.clone();
        }
        let k = m.to_string();
        let fact = self.man.return_fact(&k).map(fact_value);
        // 类型取返回描述符（absint 对 ty = None 的调用结果按描述符补齐），来源由 absint 换成本调用点
        let caller_class = self.man.returns_caller_class(&k);
        let nonnull_ret = if self.man.empty.is_factory(&k) {
            Some(V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(crate::absint::Obj::Empty)) })
        } else {
            caller_class.then(nonnull_ref)
        };
        let empty_query = self.man.empty.query(&m.name, &m.desc).map(fact_value);
        let target = self
            .exact_target(opcode, m, iface)
            .filter(|(cf, t)| cf.method(&t.name, &t.desc).is_some_and(|tm| self.kind_of(cf, tm) == Kind::Bytecode))
            .map(|(_, t)| t);
        let tk = target.as_ref().map(|t| t.to_string());
        let value_eq = self.man.is_value_equals(&k) || tk.as_ref().is_some_and(|t| self.man.is_value_equals(t));
        let str_op = self.man.string_op(&k).or_else(|| tk.as_ref().and_then(|t| self.man.string_op(t)));
        let str_kind = {
            use crate::absint::StrKind;
            use crate::manifest::StrOp;
            let names = &self.man.names;
            if names.is_builder(&k) {
                Some(StrKind::Init)
            } else if names.is_append(&k) {
                Some(StrKind::Append)
            } else if names.is_result(&k) {
                Some(StrKind::Result)
            } else {
                match str_op {
                    Some(StrOp::StartsWith) => Some(StrKind::StartsWith),
                    Some(StrOp::EndsWith) => Some(StrKind::EndsWith),
                    _ => None,
                }
            }
        };
        let shape = self.man.string_shape(&k).or_else(|| tk.as_ref().and_then(|t| self.man.string_shape(t))).map(|x| {
            let sh = crate::absint::Shape::prefixed(&x.prefix, &x.excludes);
            V::Ref { ty: None, nonnull: false, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Str(sh))) }
        });
        let c = Rc::new(CallInfo {
            fact,
            null_to_false: self.man.is_null_to_false(&k),
            target,
            value_eq,
            str_op,
            holder: self.man.sysprops.is_holder(&k),
            reader: self.reader_spec(&k),
            offset: self.man.field_name_resolver(&k).filter(|r| r.offset),
            nonnull_ret,
            empty_query,
            str_kind,
            shape,
            caller_class,
        });
        self.calls.borrow_mut().entry(m.clone()).or_default().push((opcode, iface, c.clone()));
        c
    }

    /// 调用的唯一目标（静态 / 构造 / 私有 / final 方法 / final 类）
    pub(in crate::engine) fn exact_target(&self, opcode: u8, m: &MemberRef, iface: bool) -> Option<(std::sync::Arc<ClassFile>, MemberRef)> {
        use classfile::op;
        let site = self.h.resolve_method(&m.owner, &m.name, &m.desc, iface)?;
        let rm = site.method();
        let exact = matches!(opcode, op::INVOKESTATIC | op::INVOKESPECIAL)
            || rm.is_private()
            || rm.is_static()
            || rm.is_final()
            || site.class.access & acc::FINAL != 0 && !site.class.is_interface();
        let (o, n, d) = site.key();
        exact.then(|| (site.class.clone(), MemberRef { owner: o, name: n, desc: d }))
    }
}

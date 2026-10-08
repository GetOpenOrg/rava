//! absint 的 `Oracle` 实现：调用结果、字段读、构造、形参等查询。

use super::*;
use super::super::obj_fields::ObjAns;
use super::super::site_rets::SiteAns;

/// 值带空集合标签（接收者为空的不可修改集合）
fn is_empty_tag(v: Option<&V>) -> bool {
    v.and_then(|v| v.obj()).is_some_and(|o| **o == crate::absint::Obj::Empty)
}

impl Oracle for Facts<'_, '_> {
    fn invoke_result(&self, opcode: u8, off: u32, m: &MemberRef, iface: bool, args: &[V]) -> Ret {
        if let Some(l) = self.level {
            if let Some(&t) = self.ctx.man.concrete.boot.level_queries.get(&m.to_string()) {
                return Ret::Value(V::Int(i32::from(i64::from(l) >= t)));
            }
        }
        let c = self.ctx.call_info(opcode, m, iface);
        if let Some(v) = &c.fact {
            return Ret::Value(v.clone());
        }
        if c.null_to_false && args.contains(&V::Null) {
            return Ret::Value(V::Int(0));
        }
        if let Some(v) = &c.nonnull_ret {
            if c.caller_class && self.callers.is_some() {
                self.caller_sites.borrow_mut().insert(off);
            }
            return Ret::Value(v.clone());
        }
        if let Some(v) = c.empty_query.as_ref().filter(|_| is_empty_tag(args.first())) {
            return Ret::Value(v.clone());
        }
        if let Some(v) = c.offset.and_then(|r| self.ctx.field_offset(opcode, r, args)) {
            return Ret::Value(v);
        }
        // 类字面量接收者上的接收者钩子字段取值方法
        if let (classfile::op::INVOKEVIRTUAL, Some(V::Class(k, _))) = (opcode, args.first()) {
            if let Some(v) = self.ctx.mirrors_call(m, [&**k]) {
                return Ret::Value(v);
            }
        }
        if let Some(r) = self.ctx.derived_result(self.m, opcode, m, iface, args, &c) {
            return r;
        }
        // 返回串形状事实：常量实参能求出常量时取常量
        if let Some(v) = &c.shape {
            let exact = c.target.as_ref().and_then(|t| self.ctx.const_eval(self.m, t, args));
            return Ret::Value(exact.unwrap_or_else(|| v.clone()));
        }
        // 接收者只来自形参且全为抽象对象：按对象的返回值（`obj_rets.rs`），否则取按成员汇合的返回常量
        let per = self.obj_ret(opcode, m, iface, args);
        // 唯一目标且为字节码方法：取其返回常量（被调方法返回常量变化时本方法失效重算）；
        // 返回常量在各调用点上汇合为 Top 时按本调用点的常量实参求值
        let per = match per {
            Some(ObjAns::Value(p)) => Some(p),
            // 有接收者对象的目标尚未定论（对象集只在有方法上下文时查询）
            Some(ObjAns::Never) => {
                if let Some(me) = self.m {
                    self.ctx.dep(me, Dep::Never);
                }
                return Ret::Never;
            }
            None => None,
        };
        let Some(t) = &c.target else {
            return match per {
                Some(PV::Const(v)) => Ret::Value(v),
                _ => Ret::Unknown,
            };
        };
        let eval = || self.ctx.const_eval(self.m, t, args).map_or(Ret::Unknown, Ret::Value);
        let Some(me) = self.m else { return eval() };
        // 静态调用点接克隆节点：取节点返回值（`site_rets.rs`，依赖由引擎登记）
        let site = if opcode == classfile::op::INVOKESTATIC { self.sites.iter().find(|(o, _)| *o == off).map(|(_, a)| a) } else { None };
        let r = match (per, site) {
            (Some(p), _) => Some(p),
            (None, Some(SiteAns::Never)) => {
                self.ctx.dep(me, Dep::Never);
                return Ret::Never;
            }
            (None, Some(SiteAns::Val(v))) => Some(PV::Const(v.clone())),
            (None, None) => {
                self.ctx.dep(me, Dep::Ret(t.clone()));
                self.ctx.rvals.borrow().get(t).cloned()
            }
        };
        match r {
            // 小集合：先按本调用点的常量实参求值，求不出时取集合
            Some(PV::Const(v @ V::Ints(_))) => match eval() {
                Ret::Unknown => Ret::Value(v),
                x => x,
            },
            // 非空 / 带类型的无对象引用：先按本调用点的常量实参求值（可能得出字符串常量、null 或映像对象），
            // 求不出时取该引用（求值结果是本调用点实参下的精确值，比汇合格精确且同样可靠）
            Some(PV::Const(v)) if is_nonnull_ref(&v) || v.shape_tagged() || matches!(v, V::Ref { obj: None, .. }) => match eval() {
                Ret::Unknown => Ret::Value(v),
                x => x,
            },
            Some(PV::Const(v)) => Ret::Value(v),
            Some(PV::Top) => eval(),
            None if self.ctx.noreturn.borrow().answer_never(t) => {
                self.ctx.dep(me, Dep::Never);
                Ret::Never
            }
            None => eval(),
        }
    }
    fn final_static(&self, f: &MemberRef) -> bool {
        self.ctx.field_info(f).is_some_and(|fi| fi.access & acc::STATIC != 0 && fi.access & acc::FINAL != 0 && !self.ctx.field_open(&fi))
    }
    fn field(&self, opcode: u8, f: &MemberRef, recv: Option<&V>) -> Option<V> {
        if let Some(v) = self.ctx.mirror_hook_field(opcode, f, recv) {
            return Some(v);
        }
        if let Some(v) = self.ctx.object_field(self.m, opcode, f, recv) {
            return Some(v);
        }
        self.ctx.field_value(self.m, f)
    }
    fn getfield(&self, f: &MemberRef, recv: Option<&V>) -> Ret {
        let op = classfile::op::GETFIELD;
        if let Some(v) = self.ctx.mirror_hook_field(op, f, recv).or_else(|| self.ctx.object_field(self.m, op, f, recv)) {
            return Ret::Value(v);
        }
        // 按对象读（`obj_fields.rs`）：⊥ 按读取之后暂不可达，排空时重算
        match self.obj_field(f, recv) {
            Some(ObjAns::Value(v)) => return Ret::Value(v),
            Some(ObjAns::Never) => {
                if let Some(me) = self.m {
                    self.ctx.dep(me, Dep::Never);
                }
                return Ret::Never;
            }
            None => {}
        }
        self.ctx.field_value(self.m, f).map_or(Ret::Unknown, Ret::Value)
    }
    fn construct(&self, init: &MemberRef, args: &[V]) -> Option<Rc<Obj>> {
        self.ctx.construct(self.m, init, args)
    }
    fn param(&self, i: u16) -> Option<V> {
        self.params.get(i as usize).cloned().flatten()
    }
    fn mirror_subtype_test(&self, m: &MemberRef) -> bool {
        self.ctx.man.is_mirror_subtype_test(&m.to_string())
    }
    fn key_getter(&self, m: &MemberRef) -> Option<(String, bool)> {
        if self.ctx.man.keyed_lookups.is_empty() {
            return None;
        }
        self.ctx.man.keyed_lookups.key_getter(&m.to_string())
    }
    fn string_equality(&self, m: &MemberRef) -> Option<bool> {
        let k = m.to_string();
        if self.ctx.man.is_value_equals(&k) {
            return Some(false);
        }
        matches!(self.ctx.man.string_op(&k), Some(crate::manifest::StrOp::EqualsIgnoreCase)).then_some(true)
    }
    fn str_kind(&self, opcode: u8, m: &MemberRef, iface: bool) -> Option<crate::absint::StrKind> {
        self.ctx.call_info(opcode, m, iface).str_kind
    }
    fn param_mirror(&self, i: u16, cls: &str) -> Option<bool> {
        self.mirrors.get(i as usize)?.as_ref().map(|s| s.contains(cls))
    }
    fn param_mirror_field(&self, i: u16, f: &MemberRef) -> Option<V> {
        let s = self.mirrors.get(i as usize)?.as_ref()?;
        self.ctx.mirrors_hook_field(f, s.iter().map(|c| &**c))
    }
    fn param_mirror_call(&self, i: u16, m: &MemberRef) -> Option<V> {
        let s = self.mirrors.get(i as usize)?.as_ref()?;
        self.ctx.mirrors_call(m, s.iter().map(|c| &**c))
    }
    fn site_mirror_call(&self, off: u32, m: &MemberRef) -> Option<V> {
        if !self.caller_sites.borrow().contains(&off) {
            return None;
        }
        let s = self.callers.as_ref()?;
        self.ctx.mirrors_call(m, s.iter().map(|c| &**c))
    }
    fn type_live(&self, ty: &str) -> bool {
        (self.live)(ty)
    }
    fn final_field(&self, f: &MemberRef) -> bool {
        // 只看声明的访问标志（与开放判定无关，见 `narrow.rs`）：结果不随分析增长，无须登记依赖
        self.ctx.field_info(f).is_some_and(|fi| fi.access & acc::FINAL != 0 && fi.access & acc::STATIC == 0)
    }
    fn init_key(&self, f: &MemberRef) -> Option<MemberRef> {
        self.ctx.init_key(f)
    }
    fn init_sum(&self, init: &MemberRef) -> Option<Rc<crate::absint::InitSum>> {
        self.ctx.ctor_init(init)
    }
}

#[cfg(test)]
mod empty_tests {
    use super::*;
    use crate::absint::Obj;

    #[test]
    fn empty_tag_recognized_on_receiver() {
        let e = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: Some(Rc::new(Obj::Empty)) };
        let other = V::Ref { ty: None, nonnull: true, src: Rc::from([].as_slice()), obj: None };
        assert!(is_empty_tag(Some(&e)));
        assert!(!is_empty_tag(Some(&other)));
        assert!(!is_empty_tag(Some(&V::Null)));
        assert!(!is_empty_tag(None));
    }
}

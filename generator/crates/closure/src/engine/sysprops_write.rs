//! 系统属性表的按键删除（清单 `writers` 中 `remove = true` 的入口）：删除只可能改变启动时存在的键。
//!
//! - 键为字符串常量：启动时存在（`values` / `dynamic`）→ 该键不折叠；启动时不存在 → 删除后仍不存在，
//!   不影响折叠（若别处写入该键，写入本身已使它不折叠）；
//! - 键恰为本方法形参、且方法只经确定目标调用（静态 / 私有）：删除包装方法，由各调用点按实参换算
//!   （[`Ctx::remove_params`] 给出包装方法里作删除键的形参，可逐层嵌套）；包装方法有非字节码入口（手写 /
//!   反射 / 方法句柄 / lambda / VM 根）时调用点不可见，按键推不出处理；
//! - 键推不出：拆成拼接段（字面量 / 候选集 / 任意串）匹配 `values` 键，匹配到的不折叠；拼接段也推不出
//!   → `values` 全部不折叠。启动时不存在的键都不受影响。
//!
//! 写入（`remove = false`）仍按原规则：键为常量 → 该键不折叠，键推不出 → 全部不折叠。

use super::class_lookup::Gap;
use super::method_lookup::parts_match;
use super::name_eval::Frame;
use super::*;
use classfile::Operand;

/// 删除包装方法的入口可见性（只增不减，两侧先后到达都能判定）
#[derive(Default)]
pub(super) struct RmWrap {
    /// 已按包装方法处理（删除键交给调用点）的方法
    deferred: BTreeSet<MemberRef>,
    /// 有非字节码调用点入口的方法（方法入口逐次查询，只做成员判定）
    untracked: HashSet<MemberRef>,
}

/// 值恰为本方法某形参（实参序号含接收者）
fn param_of(v: &V) -> Option<usize> {
    match &*v.srcs() {
        [Src::Param(i)] if matches!(v, V::Ref { .. }) => Some(*i as usize),
        _ => None,
    }
}

/// 删除包装方法的嵌套层数上限（键经形参逐层转交）：超出的按非包装方法处理（调用点按键推不出，保守）
const WRAP_DEPTH: u32 = 4;

impl Ctx<'_> {
    /// 静态 / 私有字节码方法 t 里作删除键的形参序号：删除入口的键、或另一删除包装方法删除键所在实参，
    /// 恰为 t 的形参（与调用点无关，只看字节码；接收者是否属性表对象由方法自身的扫描判定）
    pub(super) fn remove_params(&self, t: &MemberRef) -> Rc<[usize]> {
        self.remove_params_at(t, 0).0
    }

    /// depth = 已进入的包装层数；返回 (结果, 是否因层数上限截断)。截断的结果不缓存
    fn remove_params_at(&self, t: &MemberRef, depth: u32) -> (Rc<[usize]>, bool) {
        if let Some(r) = self.pwsums.borrow().get(t) {
            return (r.clone(), false);
        }
        if depth >= WRAP_DEPTH {
            return (Rc::from([].as_slice()), true);
        }
        let (r, cut) = self.compute_remove_params(t, depth);
        let r: Rc<[usize]> = r.into();
        if !cut {
            self.pwsums.borrow_mut().insert(t.clone(), r.clone());
        }
        (r, cut)
    }

    fn compute_remove_params(&self, t: &MemberRef, depth: u32) -> (Vec<usize>, bool) {
        let none = (vec![], false);
        let Some(cf) = self.h.class(&t.owner) else { return none };
        let Some(meth) = cf.method(&t.name, &t.desc) else { return none };
        if !(meth.is_static() || meth.is_private()) || !t.desc.contains(';') {
            return none;
        }
        let Some(code) = meth.code.as_ref() else { return none };
        // 递归保护：计算中先占位为空（环上的包装方法按非包装处理，保守）
        self.pwsums.borrow_mut().insert(t.clone(), Rc::from([].as_slice()));
        let mut cut = false;
        // 删除键所在实参序号：删除入口，或确定目标的删除包装方法
        let mut key_args = |opcode: u8, mref: &MemberRef, iface: bool| -> Vec<usize> {
            if let Some(w) = self.man.sysprops.writer(&mref.to_string()).filter(|w| w.remove) {
                return vec![w.key];
            }
            if !matches!(opcode, classfile::op::INVOKESTATIC | classfile::op::INVOKESPECIAL | classfile::op::INVOKEVIRTUAL) {
                return vec![];
            }
            let Some(c) = self.call_info(opcode, mref, iface).target.clone() else { return vec![] };
            let (r, c2) = self.remove_params_at(&c, depth + 1);
            cut |= c2;
            r.to_vec()
        };
        let mut sites: HashMap<u32, Vec<usize>> = HashMap::default();
        for i in &code.insns {
            if let Operand::Method(r, iface) = &i.operand {
                let ks = key_args(i.opcode, r, *iface);
                if !ks.is_empty() {
                    sites.insert(i.offset, ks);
                }
            }
        }
        self.pwsums.borrow_mut().remove(t);
        if sites.is_empty() {
            return (vec![], cut);
        }
        let Some(md) = parse_method(&t.desc) else { return none };
        let n = md.params.len() + usize::from(!meth.is_static());
        let live = |_: &str| true;
        let a = self.aux_analyze(&cf.name, &t.desc, meth.is_static(), code, &Facts { ctx: self, live: &live, m: None, params: vec![None; n], mirrors: vec![], level: None, objs: Default::default(), callers: None, caller_sites: Default::default(), sites: Rc::from([]), key: None, dv: false });
        let mut out: Vec<usize> = a
            .events
            .iter()
            .filter_map(|(off, e)| {
                let Event::Invoke { args, .. } = e else { return None };
                Some(sites.get(off)?.iter().filter_map(|k| param_of(args.get(*k)?)).collect::<Vec<_>>())
            })
            .flatten()
            .collect();
        out.sort_unstable();
        out.dedup();
        (out, cut)
    }
}

impl Engine<'_> {
    /// 方法 m 里删除入口 / 删除包装方法调用的键值 v 使哪些键不折叠
    pub(super) fn removed_keys(&mut self, m: usize, a: &Analysis, v: &V) -> Vec<Option<String>> {
        if let V::Str(s, _) = v {
            return match self.man.sysprops.lookup(s) {
                PropValue::Absent => vec![],
                _ => vec![Some(s.to_string())],
            };
        }
        if let Some(p) = param_of(v) {
            let key = self.methods[m].key.clone();
            if !self.rmwrap.untracked.contains(&key) && self.ctx.remove_params(&key).contains(&p) {
                self.rmwrap.deferred.insert(key);
                return vec![];
            }
        }
        let parts = if a.conservative {
            None
        } else {
            let owner = self.methods[m].key.owner.clone();
            let f = Frame { m: Some(m), a, owner: &owner, up: None };
            self.name_parts(&f, v, Gap::Method, 0)
        };
        let values = self.man.sysprops.values().keys();
        match parts {
            Some(parts) => values.filter(|k| parts_match(&parts, k)).map(|k| Some(k.clone())).collect(),
            None => values.map(|k| Some(k.clone())).collect(),
        }
    }

    /// 调用确定目标的删除包装方法：按本调用点实参换算删除的键
    pub(super) fn remove_wrapper_call(&mut self, m: usize, a: &Analysis, opcode: u8, mref: &MemberRef, iface: bool, args: &[V]) -> Vec<Option<String>> {
        if !matches!(opcode, classfile::op::INVOKESTATIC | classfile::op::INVOKESPECIAL | classfile::op::INVOKEVIRTUAL) {
            return vec![];
        }
        let Some(t) = self.ctx.call_info(opcode, mref, iface).target.clone() else { return vec![] };
        let ps = self.ctx.remove_params(&t);
        let mut out = Vec::new();
        for p in ps.iter() {
            let v = args.get(*p).cloned().unwrap_or(V::Top);
            out.extend(self.removed_keys(m, a, &v));
        }
        out
    }

    /// 方法入口来自非字节码调用点：已按删除包装方法处理过的，键按推不出处理（`values` 全部不折叠）
    pub(super) fn remove_entry(&mut self, key: &MemberRef, tracked: bool) {
        if tracked || self.rmwrap.untracked.contains(key) {
            return;
        }
        self.rmwrap.untracked.insert(key.clone());
        if !self.rmwrap.deferred.contains(key) {
            return;
        }
        let keys = self.man.sysprops.values().keys().map(|k| Some(k.clone())).collect();
        let k = key.to_string();
        self.sysprops_unstable(keys, || format!("{k}：删除包装方法有非字节码入口"));
    }
}

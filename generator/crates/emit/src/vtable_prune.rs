//! vtable 槽按分派结果裁剪（C3 第 5 项）。
//!
//! 分析器导出 `dispatched`（经虚分派 / 枢纽 / VM 反射到达的实现）。槽族 = (族根短名,
//! 方法名, 参数描述符)，族根为不裁剪口径下的槽位声明者（[`EmitCtx::raw_resolve_virtual_slot`]）。
//!
//! - 族需要槽 ⇔ 族内被派发到的实现 ≥ 2，或唯一被派发到的实现不是族根的声明；
//! - 族根在需要槽的族内保留槽；覆盖者 C.m 保留槽 ⇔ C.m 或 C 的某个子类覆盖被派发到
//!   （C 类型接收者上的调用须经 vtable 到达子类实现）；
//! - private 实例方法不占槽（invokespecial / nestmate 直接调用）；
//! - 不裁剪（强制保留）：覆盖根类公开方法（ObjectVTable 桥）、手写参与（共置手写 / 伴生
//!   核心 / 手写子类覆盖 / 手写实现对象的 `impl X__VTable for S`）、synthetic 桥参与、族根不声明（注入的接口 default）、接口方法、
//!   非本类声明的方法（default 注入 / 祖先方法的重发射由声明者口径决定）。
//!
//! 不占槽的方法由宏按 NonVirtual 展开（wrapper 上的直接方法）：super 调用与继承成员
//! 改为上转到声明者 wrapper 直接调用。正确性依赖分析器派发结果的完备性。

use std::collections::{HashMap, HashSet};

use classfile::{acc, Method};
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use crate::ctx::EmitCtx;
use crate::vtable::{param_part, same_slot};

const INIT: &str = "<init>";
const CLINIT: &str = "<clinit>";

/// 槽族键：(族根短名, 方法名, 参数描述符)
type FamKey = (String, String, String);

/// 槽族裁剪计划（全项目一次预计算）
#[derive(Default)]
pub struct SlotPlan {
    /// 槽族 → 被派发到的实现所在类（binary）
    impls: HashMap<FamKey, Vec<String>>,
    /// 强制保留槽的族（synthetic 桥 / 手写参与）
    forced: HashSet<FamKey>,
}

impl<'a> EmitCtx<'a> {
    fn fam_key(&self, m: &Method, ci: &ClassInfo) -> Option<FamKey> {
        let root = self.raw_resolve_virtual_slot(m, ci);
        (!root.is_empty()).then(|| (root, m.name.clone(), param_part(&m.desc).to_string()))
    }

    /// 类的共置手写 / 伴生核心 / 接口伴生是否涉及该方法（原名或 mangle 名）
    fn hw_involved(&self, cls: &str, m: &Method) -> bool {
        let Some(h) = self.input.handwritten.get(cls) else { return false };
        let plain = safe_ident(&m.name);
        let mangled = safe_ident(&mangle_name(&self.manifest.ty, &m.name, &m.desc));
        [plain, mangled].iter().any(|n| {
            let imp = format!("__impl_{n}");
            h.methods.contains(n)
                || h.methods.contains(&imp)
                || h.method_cores.contains_key(n)
                || h.method_cores.contains_key(&imp)
                || h.iface_method_sigs.contains_key(n)
                || h.class_vtable_fns.contains(n)
        })
    }

    fn slot_plan(&self) -> &SlotPlan {
        self.slot_plan.get_or_init(|| {
            let reg = self.ty.reg;
            let mut plan = SlotPlan::default();
            for (owner, name, desc) in &self.input.dispatched {
                let Some(ci) = reg.get(owner).filter(|c| !c.is_interface()) else { continue };
                let Some(m) = ci.methods().iter().find(|x| x.name == *name && x.desc == *desc) else { continue };
                if m.is_static() || m.name == INIT || m.name == CLINIT || m.access & acc::PRIVATE != 0 {
                    continue;
                }
                let Some(key) = self.fam_key(m, ci) else { continue };
                if m.is_synthetic() {
                    plan.forced.insert(key.clone());
                }
                let v = plan.impls.entry(key).or_default();
                if !v.iter().any(|b| b == owner) {
                    v.push(owner.clone());
                }
            }
            for ci in reg.iter().filter(|c| !c.is_interface()) {
                let has_hw = self.input.handwritten.get(ci.name()).is_some();
                for b in ci.methods().iter().filter(|x| !x.is_static() && x.name != INIT && x.name != CLINIT) {
                    let bridge = b.access & acc::BRIDGE != 0;
                    if !bridge && !(has_hw && self.hw_involved(ci.name(), b)) {
                        continue;
                    }
                    // 桥：同类同名方法（真实方法）所在族一并保留
                    for x in ci.methods().iter().filter(|x| !x.is_static() && (x.name == b.name) && (bridge || std::ptr::eq(*x, b))) {
                        if let Some(key) = self.fam_key(x, ci) {
                            plan.forced.insert(key);
                        }
                    }
                }
            }
            plan
        })
    }

    /// `sub` 是否为 `sup` 的严格子类（超类链）
    fn strict_subclass(&self, sub: &str, sup: &str) -> bool {
        let reg = self.ty.reg;
        let mut seen: HashSet<&str> = HashSet::new();
        let mut cur = reg.get(sub).map(ClassInfo::super_class);
        while let Some(c) = cur.filter(|c| !c.is_empty() && seen.insert(c)) {
            if c == sup {
                return true;
            }
            cur = reg.get(c).map(ClassInfo::super_class);
        }
        false
    }

    /// 族根类（`ci` 超类链上短名为 `root` 者）是否声明该槽位
    fn root_declares(&self, root: &str, m: &Method, ci: &ClassInfo) -> bool {
        let reg = self.ty.reg;
        let mut seen: HashSet<&str> = HashSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur.filter(|c| seen.insert(c.name())) {
            if self.short(c.name()) == root {
                return c.methods().iter().any(|x| !x.is_synthetic() && same_slot(x, m));
            }
            cur = reg.get(c.super_class());
        }
        false
    }

    /// 本类声明的实例方法 `m` 是否不占 vtable 槽（按分派结果裁剪）
    pub fn slot_pruned(&self, m: &Method, ci: &ClassInfo) -> bool {
        if ci.is_interface() || ci.is_constructor(m) || m.is_static() || m.is_native() || m.name == CLINIT {
            return false;
        }
        let memo_key = (ci.name().to_string(), m.name.clone(), m.desc.clone());
        if let Some(&v) = self.slot_memo.lock().ok().and_then(|g| g.get(&memo_key).copied()).as_ref() {
            return v;
        }
        let v = self.compute_slot_pruned(m, ci);
        if let Ok(mut g) = self.slot_memo.lock() {
            g.insert(memo_key, v);
        }
        v
    }

    fn compute_slot_pruned(&self, m: &Method, ci: &ClassInfo) -> bool {
        if !ci.methods().iter().any(|x| x.name == m.name && x.desc == m.desc) || m.is_synthetic() {
            return false;
        }
        if self.hw_involved(ci.name(), m) {
            return false;
        }
        if m.access & acc::PRIVATE != 0 {
            return true;
        }
        let Some(key) = self.fam_key(m, ci) else { return false };
        if self.root_keys().contains(&(key.1.clone(), key.2.clone())) {
            return false;
        }
        let plan = self.slot_plan();
        if plan.forced.contains(&key) || !self.root_declares(&key.0, m, ci) {
            return false;
        }
        let Some(impls) = plan.impls.get(&key) else { return true };
        let slotted = impls.len() >= 2 || impls.iter().any(|b| self.short(b) != key.0);
        if !slotted {
            return true;
        }
        if key.0 == self.short(ci.name()) {
            return false;
        }
        !impls.iter().any(|b| b == ci.name() || self.strict_subclass(b, ci.name()))
    }
}

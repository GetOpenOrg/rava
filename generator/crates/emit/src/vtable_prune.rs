//! vtable 槽按实例化类的实际选中实现裁剪，未被派发到的实现发存根（C3 第 5 项）。
//!
//! 槽族 = (族根 binary, 方法名, 参数描述符)，族根为不裁剪口径下的槽位声明者（[`EmitCtx::raw_resolve_virtual_slot`]）。
//! 「有效声明者」= 某个已实例化类 X 沿超类链实际选中的实现所在类（JVM 选择语义，与分析器派发结果无关）。
//!
//! - 族需要槽 ⇔ 存在不在族根的有效声明者（某个实例化类选中了覆盖实现）；
//! - 族根在需要槽的族内保留槽；覆盖者 C.m 保留槽 ⇔ C 或 C 的某个严格子类是有效声明者
//!   （C 类型接收者上的调用须经 vtable 到达子类实现）；
//! - 保留槽的有效覆盖者若不在分析器 `dispatched` 中（被派发的桥方法所桥接的真实方法算作已派发），槽条目发 `__stub`（`slot_stub = "true"`），
//!   方法本身（super 调用 / 直接调用）照常翻译；
//! - private 实例方法不占槽（JVMS §5.4.6：invokevirtual 只选中它本身）；
//! - 不裁剪也不发存根（强制保留）：覆盖根类公开方法（ObjectVTable 桥）、手写参与（共置手写 / 伴生
//!   核心 / 手写子类覆盖（含按祖先声明合成的手写继承覆盖）/ 手写实现对象的 `impl X__VTable for S`）、synthetic 桥参与、族根不声明（注入的接口 default）、接口方法、
//!   非本类声明的方法（default 注入 / 祖先方法的重发射由声明者口径决定）。
//!
//! 正确性：运行期对象只可能是已实例化类的实例。不需要槽的族里，所有实例化类选中的都是族根实现，
//! 直接调用与 JVM 选择一致；需要槽的族里，每个实例化类的槽条目都是它实际选中的实现——被派发到的
//! 照常执行，未被派发到的是存根。分析器漏派发边时，结果是 panic（报出类名、方法名、描述符），
//! 不会悄悄执行祖先实现。

use std::collections::{HashMap, HashSet};

use classfile::{acc, Method};
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::ClassInfo;

use crate::class_writer::hw_overrides::handwritten_inherited_overrides;
use crate::ctx::EmitCtx;
use crate::phase2::bridge::bridge_call_target;
use crate::vtable::{param_part, same_slot};

const INIT: &str = "<init>";
const CLINIT: &str = "<clinit>";

/// 槽族键：(族根短名, 方法名, 参数描述符)
type FamKey = (String, String, String);

/// 槽族裁剪计划（全项目一次预计算）
#[derive(Default)]
pub struct SlotPlan {
    /// 槽族 → 有效声明者（某个实例化类实际选中的实现所在类，binary）
    effective: HashMap<FamKey, HashSet<String>>,
    /// 强制保留槽的族（synthetic 桥 / 手写参与）
    forced: HashSet<FamKey>,
    /// 被派发的桥方法所桥接的真实方法 (声明类, 方法名, 真实描述符)：桥被省略、槽并入继承的
    /// 真实实现时，派发到桥即派发到该实现
    bridged: HashSet<(String, String, String)>,
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
            for x in self.input.instantiated.iter().filter_map(|n| reg.get(n)) {
                if x.is_interface() || x.class_file().access & acc::ABSTRACT != 0 {
                    continue;
                }
                // 自底向上：每个族第一个遇到的声明即 X 选中的实现
                let mut seen: HashSet<FamKey> = HashSet::new();
                let mut chain: HashSet<&str> = HashSet::new();
                let mut cur = Some(x);
                while let Some(c) = cur.filter(|c| chain.insert(c.name())) {
                    for m in c.methods().iter().filter(|m| !m.is_static() && m.name != INIT && m.name != CLINIT && m.access & acc::PRIVATE == 0) {
                        let Some(key) = self.fam_key(m, c) else { continue };
                        if seen.insert(key.clone()) {
                            plan.effective.entry(key).or_default().insert(c.name().to_string());
                        }
                    }
                    cur = reg.get(c.super_class());
                }
            }
            for ci in reg.iter().filter(|c| !c.is_interface()) {
                for m in ci.methods().iter().filter(|m| m.is_synthetic() && !m.is_static()) {
                    if let Some(key) = self.fam_key(m, ci) {
                        plan.forced.insert(key);
                    }
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
                // 手写覆盖的继承虚方法：与类发射同一来源（`handwritten_inherited_overrides`）合成的
                // 覆盖声明照常占祖先槽，所在族一并保留
                if has_hw {
                    let visible: Vec<&Method> = ci.methods().iter().filter(|m| !m.is_synthetic()).collect();
                    for o in handwritten_inherited_overrides(self, ci, &visible) {
                        if let Some(key) = self.fam_key(&o.method, ci) {
                            plan.forced.insert(key);
                        }
                    }
                }
            }
            for (cls, name, desc) in &self.input.dispatched {
                let Some(ci) = reg.get(cls) else { continue };
                let Some(b) = ci.methods().iter().find(|m| m.access & acc::BRIDGE != 0 && &m.name == name && &m.desc == desc) else {
                    continue;
                };
                if let Some((owner, real)) = bridge_call_target(self, ci, b, name) {
                    plan.bridged.insert((owner.name().to_string(), name.clone(), real));
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
        let Some(eff) = plan.effective.get(&key) else { return true };
        if !eff.iter().any(|b| self.short(b) != key.0) {
            return true;
        }
        if key.0 == self.short(ci.name()) {
            return false;
        }
        !eff.iter().any(|b| b == ci.name() || self.strict_subclass(b, ci.name()))
    }

    /// 占槽覆盖者 `m` 的槽条目是否发存根：有效（某实例化类选中）却不在分析器派发结果中
    pub fn slot_stub(&self, m: &Method, ci: &ClassInfo) -> bool {
        if ci.is_interface() || m.is_static() || m.is_native() || m.is_abstract() || m.is_synthetic() || ci.is_constructor(m) {
            return false;
        }
        if self.slot_pruned(m, ci) || self.hw_involved(ci.name(), m) {
            return false;
        }
        let Some(key) = self.fam_key(m, ci) else { return false };
        if key.0 == self.short(ci.name()) || self.root_keys().contains(&(key.1.clone(), key.2.clone())) {
            return false;
        }
        let plan = self.slot_plan();
        if plan.forced.contains(&key) || !plan.effective.get(&key).is_some_and(|e| e.contains(ci.name())) {
            return false;
        }
        let member = (ci.name().to_string(), m.name.clone(), m.desc.clone());
        !self.input.dispatched.contains(&member) && !plan.bridged.contains(&member)
    }
}

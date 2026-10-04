//! vtable 槽按实例化类的实际选中实现裁剪，未被派发到的实现发存根（C3 第 5 项）。
//!
//! 槽族 = (族根 binary, 方法名, 参数描述符)，族根为不裁剪口径下的槽位声明者（[`EmitCtx::raw_resolve_virtual_slot`]）。
//! 「有效声明者」= 某个已实例化类 X 沿超类链实际选中的实现所在类（JVM 选择语义，与分析器派发结果无关）。
//!
//! - 族需要槽 ⇔ 存在不在族根的有效声明者（某个实例化类选中了覆盖实现）；
//! - 档案侧（T1 1b）：非用户类的判定只用非用户侧事实（实例化类、桥、手写、派发结果都只取非用户类），
//!   用户类的影响按开放世界计入——每个可被用户扩展的类 X 视为有一个假想用户子类：不覆盖时选中 X 链上的
//!   实现（有效声明者），覆盖时（选中实现 public / protected 且非 final）是 X 之下的有效声明者。
//!   于是 JDK 类的槽形状与用户程序无关（同一档案下 JDK crate 逐字节相同），且覆盖任何真实用户子类；
//!   用户类按全量事实判定；
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

use crate::ctx::EmitCtx;
use crate::phase2::bridge::bridge_call_target;
use crate::vtable::{param_part, same_slot};

const INIT: &str = "<init>";
const CLINIT: &str = "<clinit>";
/// sealed 许可链的递归深度上限（防御环状许可）
const SEALED_DEPTH: u8 = 8;

/// 槽族键：(族根短名, 方法名, 参数描述符)
type FamKey = (String, String, String);

/// 槽族裁剪计划（全项目一次预计算）：档案侧（非用户类，只依赖档案事实 + 开放世界）与全量（用户类）两份
#[derive(Default)]
pub struct SlotPlan {
    jdk: SidePlan,
    all: SidePlan,
}

/// 单侧的槽族事实
#[derive(Default)]
struct SidePlan {
    /// 槽族 → 有效声明者（某个实例化类实际选中的实现所在类，binary；档案侧另含可扩展类选中的实现）
    effective: HashMap<FamKey, HashSet<String>>,
    /// 槽族 → 可被用户扩展、且选中实现可被覆盖的类 X（档案侧）：假想的用户子类 U ⊂ X 覆盖该方法，
    /// U 是不在族根的有效声明者，且是 X 及其各祖先的严格子类
    open: HashMap<FamKey, HashSet<String>>,
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

    /// 槽族计划（全项目一次）：在无作用域视图上求得——计划是全局事实，求值途中的类型名
    /// 认领不得落进恰好首个查询者的文件作用域
    fn slot_plan(&self) -> &SlotPlan {
        let plain = self.unscoped();
        self.slot_plan.get_or_init(|| SlotPlan { jdk: plain.side_plan(true), all: plain.side_plan(false) })
    }

    /// 非用户类的方法按 `plan.jdk` 判定（档案侧口径），用户类按 `plan.all`
    fn side_of<'p>(&self, plan: &'p SlotPlan, ci: &ClassInfo) -> &'p SidePlan {
        if self.is_user(ci.name()) {
            &plan.all
        } else {
            &plan.jdk
        }
    }

    /// 单侧计划。`archive`：只用非用户侧事实（实例化类、synthetic 桥、手写参与、派发结果），
    /// 另按开放世界补「可被用户扩展的类」的假想子类（[`Self::user_extensible`]）
    fn side_plan(&self, archive: bool) -> SidePlan {
        let reg = self.ty.reg;
        let keep = |c: &ClassInfo| !archive || !self.is_user(c.name());
        let mut plan = SidePlan::default();
        for x in self.input.instantiated.iter().filter_map(|n| reg.get(n)).filter(|c| keep(c)) {
            if x.is_interface() || x.class_file().access & acc::ABSTRACT != 0 {
                continue;
            }
            self.walk_selected(x, |key, c, _| {
                plan.effective.entry(key).or_default().insert(c.name().to_string());
            });
        }
        if archive {
            for x in reg.iter().filter(|c| !self.is_user(c.name()) && self.user_extensible(c)) {
                self.walk_selected(x, |key, c, m| {
                    plan.effective.entry(key.clone()).or_default().insert(c.name().to_string());
                    if m.access & (acc::PUBLIC | acc::PROTECTED) != 0 && m.access & acc::FINAL == 0 {
                        plan.open.entry(key).or_default().insert(x.name().to_string());
                    }
                });
            }
        }
        for ci in reg.iter().filter(|c| !c.is_interface() && keep(c)) {
            for m in ci.methods().iter().filter(|m| m.is_synthetic() && !m.is_static()) {
                if let Some(key) = self.fam_key(m, ci) {
                    plan.forced.insert(key);
                }
            }
        }
        for ci in reg.iter().filter(|c| !c.is_interface() && keep(c)) {
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
        for (cls, name, desc) in &self.input.dispatched {
            let Some(ci) = reg.get(cls).filter(|c| keep(c)) else { continue };
            let Some(b) = ci.methods().iter().find(|m| m.access & acc::BRIDGE != 0 && &m.name == name && &m.desc == desc) else {
                continue;
            };
            if let Some((owner, real)) = bridge_call_target(self, ci, b, name) {
                plan.bridged.insert((owner.name().to_string(), name.clone(), real));
            }
        }
        plan
    }

    /// 自底向上走 `x` 的超类链：每个族第一个遇到的声明即 `x` 选中的实现，回调 (族键, 声明类, 方法)
    fn walk_selected(&self, x: &'a ClassInfo, mut f: impl FnMut(FamKey, &'a ClassInfo, &'a Method)) {
        let reg = self.ty.reg;
        let mut seen: HashSet<FamKey> = HashSet::new();
        let mut chain: HashSet<&str> = HashSet::new();
        let mut cur = Some(x);
        while let Some(c) = cur.filter(|c| chain.insert(c.name())) {
            for m in c.methods().iter().filter(|m| !m.is_static() && m.name != INIT && m.name != CLINIT && m.access & acc::PRIVATE == 0) {
                let Some(key) = self.fam_key(m, c) else { continue };
                if seen.insert(key.clone()) {
                    f(key, c, m);
                }
            }
            cur = reg.get(c.super_class());
        }
    }

    /// 非用户类可被用户程序继承（开放世界，与闭包分析器 `engine/open_world.rs` 同口径）：
    /// 非接口、非 final、有子类可调用的构造器；sealed 看许可子类；JDK 非公开类不可扩展，
    /// 依赖库非公开类按可扩展（同名包可见）
    fn user_extensible(&self, ci: &ClassInfo) -> bool {
        self.extensible_at(ci, 0)
    }

    fn extensible_at(&self, ci: &ClassInfo, depth: u8) -> bool {
        let cf = ci.class_file();
        if ci.is_interface() {
            return false;
        }
        if !cf.permitted_subclasses.is_empty() {
            return depth < SEALED_DEPTH
                && cf.permitted_subclasses.iter().any(|s| self.ty.reg.get(s).is_some_and(|p| self.extensible_at(p, depth + 1)));
        }
        let lib = self.lib_crate_of(ci.name()).is_some();
        if cf.access & acc::FINAL != 0 || (cf.access & acc::PUBLIC == 0 && !lib) {
            return false;
        }
        cf.methods.iter().any(|m| m.name == INIT && (m.access & (acc::PUBLIC | acc::PROTECTED) != 0 || lib && m.access & acc::PRIVATE == 0))
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

    /// 族根类（`ci` 超类链上 binary 为 `root` 者）是否声明该槽位
    fn root_declares(&self, root: &str, m: &Method, ci: &ClassInfo) -> bool {
        let reg = self.ty.reg;
        let mut seen: HashSet<&str> = HashSet::new();
        let mut cur = Some(ci);
        while let Some(c) = cur.filter(|c| seen.insert(c.name())) {
            if c.name() == root {
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
        // 判定是全局事实（逐方法记忆、跨文件共享）：在无作用域视图上求值
        let v = self.unscoped().compute_slot_pruned(m, ci);
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
        let plan = self.side_of(self.slot_plan(), ci);
        if plan.forced.contains(&key) || !self.root_declares(&key.0, m, ci) {
            return false;
        }
        let none = HashSet::new();
        let eff = plan.effective.get(&key).unwrap_or(&none);
        let open = plan.open.get(&key).unwrap_or(&none);
        if open.is_empty() && !eff.iter().any(|b| *b != key.0) {
            return true;
        }
        if key.0 == ci.name() {
            return false;
        }
        let below = |b: &String| b == ci.name() || self.strict_subclass(b, ci.name());
        !eff.iter().any(below) && !open.iter().any(below)
    }

    /// 占槽覆盖者 `m` 的槽条目是否发存根：有效（某实例化类选中）却不在分析器派发结果中
    pub fn slot_stub(&self, m: &Method, ci: &ClassInfo) -> bool {
        self.unscoped().compute_slot_stub(m, ci)
    }

    fn compute_slot_stub(&self, m: &Method, ci: &ClassInfo) -> bool {
        if ci.is_interface() || m.is_static() || m.is_native() || m.is_abstract() || m.is_synthetic() || ci.is_constructor(m) {
            return false;
        }
        if self.slot_pruned(m, ci) || self.hw_involved(ci.name(), m) {
            return false;
        }
        let Some(key) = self.fam_key(m, ci) else { return false };
        if key.0 == ci.name() || self.root_keys().contains(&(key.1.clone(), key.2.clone())) {
            return false;
        }
        let plan = self.side_of(self.slot_plan(), ci);
        if plan.forced.contains(&key) || !plan.effective.get(&key).is_some_and(|e| e.contains(ci.name())) {
            return false;
        }
        let member = (ci.name().to_string(), m.name.clone(), m.desc.clone());
        !self.input.dispatched.contains(&member) && !plan.bridged.contains(&member)
    }
}

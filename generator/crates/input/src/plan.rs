//! 逐方法发射判定（`emitter/class_writer.py` `_emit_method_blocks` 的判定部分）。
//!
//! 对每个发射类给出：类是否仅类型存根（type_only）、逐方法的 Rust 名与判定
//! （翻译字节码 / 手写 / 存根 / 接口声明…）、接口伴生契约补发的方法名。
//! 方法体文本的生成不在本层；判定与 Python 逐项一致（翻译失败退化为存根的兜底属于
//! 翻译层，不在此建模）。

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use classfile::{acc, op, ClassFile, Method, Operand};
use resolve::classpath::ClassPath;
use ty::ident::safe_ident;
use ty::type_map::mangle_name;
use ty::{consts, ClassInfo, Registry, ShortNames, TyCtx};

use crate::boundary::Boundary;
use crate::build::{EmitInput, MethodKey};
use crate::handwritten::HwEntry;
use crate::manifest::RuntimeManifest;
use crate::norm::NInsn;

const IMPL_PREFIX: &str = "__impl_";
/// `<clinit>` 的 Rust 名（宏的类初始化钩子）
pub const CLINIT_FN: &str = "__clinit";

/// 方法在类文件中的角色
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// `<clinit>` → `__clinit`
    Clinit,
    /// 接口私有实例 lambda 体（落接口载体擦除 impl 块）
    IfaceLambda,
    /// 接口私有实例方法（Java 9+，落接口载体擦除 impl 块）
    IfacePrivate,
    /// 其余方法（宏块内）
    Member,
}

/// 发射判定
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// 翻译字节码
    Bytecode,
    /// 不在调用链上：panic 存根
    StubNotInChain,
    /// native：panic 存根（手写未提供）
    StubNative,
    /// abstract：无体存根
    StubAbstract,
    /// 共置手写提供同名 `pub fn`（接口实例方法发声明，其余发 [meta] 行）
    Handwritten,
    /// 虚方法体由共置手写 `__impl_<名>` 提供（声明留在宏块内）
    HandwrittenBody,
    /// 伴生核心 `core_<名>` 适配转发
    Core,
    /// 接口 default 方法体落在接口载体
    IfaceDefaultBody,
    /// 接口实例方法无体声明
    IfaceDecl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodPlan {
    pub name: String,
    pub desc: String,
    pub rust_name: String,
    pub role: Role,
    pub verdict: Verdict,
    /// 按祖先声明合成的手写覆盖（本类字节码未声明）
    pub inherited_override: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassPlan {
    pub name: String,
    /// 全部方法都不在调用链上（仅类型存根：无 `<clinit>`、静态字段不初始化）
    pub type_only: bool,
    pub methods: Vec<MethodPlan>,
    /// 接口伴生契约补发的方法名（手写 vtable 实现、类模型缺席）
    pub iface_supplement: Vec<String>,
}

/// 逐类判定上下文
pub struct Planner<'a> {
    input: &'a EmitInput,
    ty: TyCtx<'a>,
    boundary: Boundary<'a>,
    /// 根类 public 实例方法 (名, 参数描述符部分)
    root_keys: BTreeSet<(String, String)>,
    user: BTreeSet<&'a str>,
}

fn param_part(desc: &str) -> &str {
    desc.find(')').map_or(desc, |i| &desc[..=i])
}

/// 根类的 public 实例方法集（从 JDK 字节码解析）
fn root_virtual_methods(cp: &ClassPath) -> BTreeSet<(String, String)> {
    let Some(cf) = cp.get(consts::OBJECT) else {
        return BTreeSet::new();
    };
    cf.methods
        .iter()
        .filter(|m| !m.is_static() && !m.is_synthetic() && !m.name.starts_with('<') && m.access & acc::PUBLIC != 0)
        .map(|m| (m.name.clone(), param_part(&m.desc).to_string()))
        .collect()
}

/// 已用名去重：重名追加 `_<序号>`（登记键恒为基名）
fn dedupe(used: &mut BTreeMap<String, u32>, name: String) -> String {
    match used.get_mut(&name) {
        Some(n) => {
            *n += 1;
            format!("{name}_{n}")
        }
        None => {
            used.insert(name.clone(), 0);
            name
        }
    }
}

impl<'a> Planner<'a> {
    pub fn new(input: &'a EmitInput, names: &'a ShortNames, manifest: &'a RuntimeManifest, cp: &'a ClassPath) -> Planner<'a> {
        Planner {
            input,
            ty: TyCtx::new(&input.registry, names, &manifest.ty),
            boundary: Boundary::new(manifest, cp),
            root_keys: root_virtual_methods(cp),
            user: input.user_classes.iter().map(String::as_str).collect(),
        }
    }

    fn reg(&self) -> &'a Registry {
        &self.input.registry
    }

    fn in_chain(&self, cls: &str, m: &Method) -> bool {
        if self.user.contains(cls) {
            return true;
        }
        let k: MethodKey = (cls.to_string(), m.name.clone(), m.desc.clone());
        self.input.visited.contains(&k)
    }

    /// 边界类手写覆盖继承虚方法：`__impl_<m>` 而本类未声明 m → 按祖先唯一声明合成（去 abstract）
    fn inherited_overrides(&self, ci: &ClassInfo, hw: Option<&HwEntry>, visible: &[Method]) -> Vec<Method> {
        let Some(hw) = hw else { return Vec::new() };
        let n = ci.name();
        if ci.is_interface() || n.starts_with("java/") || n.starts_with("javax/") {
            return Vec::new();
        }
        let declared: BTreeSet<&str> = visible.iter().map(|m| m.name.as_str()).collect();
        let wanted: BTreeSet<&str> = hw
            .methods
            .iter()
            .filter_map(|x| x.strip_prefix(IMPL_PREFIX))
            .filter(|x| !declared.contains(x))
            .collect();
        let mut out = Vec::new();
        for name in wanted {
            let mut found: Vec<&Method> = Vec::new();
            let mut sup = ci.super_class();
            while let Some(sci) = self.reg().get(sup).filter(|_| !sup.is_empty()) {
                found.extend(sci.methods().iter().filter(|m| {
                    m.name == name && !m.is_static() && !m.is_synthetic() && m.access & acc::PRIVATE == 0
                }));
                if !found.is_empty() {
                    break;
                }
                sup = sci.super_class();
            }
            if let [m] = found.as_slice() {
                let mut c = (*m).clone();
                c.access &= !acc::ABSTRACT;
                out.push(c);
            }
        }
        out
    }

    /// `Iface.super.m()` 的方法体声明者（常量池类为 registry 接口时按广度遍历）
    fn iface_special_target(&self, owner: &str, name: &str, desc: &str) -> bool {
        if !self.reg().get(owner).is_some_and(ClassInfo::is_interface) {
            return false;
        }
        let mut queue = VecDeque::from([owner.to_string()]);
        let mut seen = BTreeSet::new();
        while let Some(cur) = queue.pop_front() {
            if !seen.insert(cur.clone()) {
                continue;
            }
            let Some(ci) = self.reg().get(&cur) else { continue };
            if ci.methods().iter().any(|m| !m.is_static() && !m.is_abstract() && m.name == name && m.desc == desc) {
                return true;
            }
            queue.extend(ci.interfaces().iter().cloned());
        }
        false
    }

    /// 接口 default 方法体能否落到接口载体
    fn iface_default_body(&self, ci: &ClassInfo, m: &Method) -> bool {
        if m.is_abstract() || m.is_synthetic() || !self.in_chain(ci.name(), m) {
            return false;
        }
        let Some(code) = self.input.code(ci.name(), m) else {
            return true;
        };
        let own: BTreeSet<(&str, &str)> = ci.methods().iter().map(|x| (x.name.as_str(), x.desc.as_str())).collect();
        let refs = code.insns.iter().filter_map(NInsn::insn).filter_map(|i| match &i.operand {
            Operand::Method(r, _) => Some((i.opcode, r)),
            _ => None,
        });
        for (opc, r) in refs {
            if opc == op::INVOKESPECIAL && self.iface_special_target(&r.owner, &r.name, &r.desc) {
                return false;
            }
            if matches!(opc, op::INVOKEVIRTUAL | op::INVOKEINTERFACE) && !own.contains(&(r.name.as_str(), r.desc.as_str())) {
                return false;
            }
        }
        true
    }

    fn clinit_plan(&self, ci: &ClassInfo, m: &Method, type_only: bool) -> Option<MethodPlan> {
        let n = ci.name();
        if type_only || self.boundary.is_vm_boundary_class(n) {
            return None;
        }
        let verdict = if self.in_chain(n, m) {
            Verdict::Bytecode
        } else if self.boundary.is_boundary_class(n) {
            return None;
        } else {
            Verdict::StubNotInChain
        };
        Some(plan_of(m, CLINIT_FN.to_string(), Role::Clinit, verdict, false))
    }

    fn chain_verdict(&self, ci: &ClassInfo, m: &Method) -> Verdict {
        if self.in_chain(ci.name(), m) {
            Verdict::Bytecode
        } else {
            Verdict::StubNotInChain
        }
    }

    fn member_rust_name(&self, ci: &ClassInfo, m: &Method, overloaded: &BTreeSet<String>) -> String {
        if ci.is_constructor(m) {
            return if overloaded.contains("<init>") {
                mangle_name(self.ty.manifest, "new", &m.desc)
            } else {
                "new".to_string()
            };
        }
        if overloaded.contains(&m.name) {
            mangle_name(self.ty.manifest, &m.name, &m.desc)
        } else {
            m.name.clone()
        }
    }

    fn member_verdict(&self, ci: &ClassInfo, m: &Method, fn_name: &str, hw: Option<&HwEntry>) -> Verdict {
        let iface_inst = ci.is_interface() && !m.is_static();
        if hw.is_some_and(|h| h.methods.contains(fn_name)) {
            return Verdict::Handwritten;
        }
        if iface_inst {
            return if self.iface_default_body(ci, m) {
                Verdict::IfaceDefaultBody
            } else {
                Verdict::IfaceDecl
            };
        }
        let virtual_slot = !ci.is_constructor(m) && !m.is_static() && !m.is_native();
        if virtual_slot && hw.is_some_and(|h| h.methods.contains(&format!("{IMPL_PREFIX}{fn_name}"))) {
            return Verdict::HandwrittenBody;
        }
        if !m.is_static() && hw.is_some_and(|h| h.method_cores.contains_key(fn_name)) {
            return Verdict::Core;
        }
        if m.is_native() {
            return Verdict::StubNative;
        }
        if m.is_abstract() {
            return Verdict::StubAbstract;
        }
        self.chain_verdict(ci, m)
    }

    /// 本接口全部祖先接口的实例成员名（Java 名 + mangle 形态）
    fn ancestor_iface_member_names(&self, ci: &ClassInfo) -> BTreeSet<String> {
        let mut names = BTreeSet::new();
        let mut sorted: Vec<String> = ci.interfaces().to_vec();
        sorted.sort();
        let mut queue = VecDeque::from(sorted);
        let mut seen = BTreeSet::new();
        while let Some(i) = queue.pop_front() {
            if !seen.insert(i.clone()) {
                continue;
            }
            let Some(ici) = self.reg().get(&i) else { continue };
            let mut sup = ici.interfaces().to_vec();
            sup.sort();
            queue.extend(sup);
            for m in ici.methods() {
                if m.is_static() || m.is_synthetic() || m.name == "<init>" || m.name == "<clinit>" {
                    continue;
                }
                names.insert(m.name.clone());
                names.insert(mangle_name(self.ty.manifest, &m.name, &m.desc));
            }
        }
        names
    }

    fn iface_supplement(&self, ci: &ClassInfo, hw: Option<&HwEntry>, used: &BTreeMap<String, u32>) -> Vec<String> {
        let Some(hw) = hw.filter(|h| ci.is_interface() && !h.iface_method_sigs.is_empty()) else {
            return Vec::new();
        };
        let mut skip: BTreeSet<String> = used.keys().cloned().collect();
        skip.extend(self.ancestor_iface_member_names(ci));
        skip.extend(self.root_keys.iter().map(|(n, _)| n.clone()));
        hw.iface_method_sigs.keys().filter(|n| !skip.contains(*n)).cloned().collect()
    }

    /// 一个发射类的方法判定
    pub fn plan(&self, cls: &str) -> Option<ClassPlan> {
        let ci = self.reg().get(cls)?;
        let cf: &ClassFile = ci.class_file();
        let hw = self.input.handwritten.get(cls);
        let is_iface = ci.is_interface();
        let type_only = !self.user.contains(cls) && !cf.methods.iter().any(|m| self.in_chain(cls, m));
        let mut visible: Vec<Method> = cf.methods.iter().filter(|m| !m.is_synthetic()).cloned().collect();
        let n_declared = visible.len();
        let extra = self.inherited_overrides(ci, hw, &visible);
        visible.extend(extra);
        let synthetic = cf
            .methods
            .iter()
            .filter(|m| m.is_synthetic() && m.access & acc::BRIDGE == 0 && m.name != "<init>" && m.name != "<clinit>");
        let n_visible = visible.len();
        let emitted: Vec<Method> = visible.into_iter().chain(synthetic.cloned()).collect();
        let overloaded = self.ty.hierarchy_overloaded_names(ci);
        let mut used: BTreeMap<String, u32> = BTreeMap::new();
        let mut methods = Vec::new();
        for (idx, m) in emitted.iter().enumerate() {
            let inherited = idx >= n_declared && idx < n_visible;
            if m.name == "<clinit>" {
                methods.extend(self.clinit_plan(ci, m, type_only));
                continue;
            }
            let iface_inst = is_iface && !m.is_static();
            if iface_inst && m.is_synthetic() && m.name.starts_with("lambda$") {
                let v = self.chain_verdict(ci, m);
                methods.push(plan_of(m, safe_ident(&m.name), Role::IfaceLambda, v, inherited));
                continue;
            }
            let root_keyed = self.root_keys.contains(&(m.name.clone(), param_part(&m.desc).to_string()));
            let private = m.access & acc::PRIVATE != 0;
            if iface_inst && !m.is_synthetic() && private && !root_keyed {
                let base = self.member_rust_name(ci, m, &overloaded);
                let rust = dedupe(&mut used, base);
                let v = self.chain_verdict(ci, m);
                methods.push(plan_of(m, rust, Role::IfacePrivate, v, inherited));
                continue;
            }
            if iface_inst && (m.is_synthetic() || private || root_keyed) {
                continue;
            }
            let rust = dedupe(&mut used, self.member_rust_name(ci, m, &overloaded));
            let fn_name = safe_ident(&rust);
            let v = self.member_verdict(ci, m, &fn_name, hw);
            methods.push(plan_of(m, rust, Role::Member, v, inherited));
        }
        let iface_supplement = self.iface_supplement(ci, hw, &used);
        Some(ClassPlan {
            name: cls.to_string(),
            type_only,
            methods,
            iface_supplement,
        })
    }
}

fn plan_of(m: &Method, rust_name: String, role: Role, verdict: Verdict, inherited_override: bool) -> MethodPlan {
    MethodPlan {
        name: m.name.clone(),
        desc: m.desc.clone(),
        rust_name,
        role,
        verdict,
        inherited_override,
    }
}

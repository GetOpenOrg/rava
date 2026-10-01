//! 发射层输入构建（`closure_input._consume` + `transpile.py` 闭包拆分 + `write_cargo_project`
//! registry 构建的移植）。
//!
//! 构建顺序即 registry 插入序（Python dict 插入序，`setdefault` 语义）：
//! 1. 用户类（入口类在前，其余按调用方给定序）；
//! 2. lib crate 类（声明序；整包模式取 jar 全集按名排序，子集模式取闭包触达的 jar 类按名排序）；
//! 3. JDK 类（闭包类序；根域类、本编译单元的用户类、归属 jar 的类除外）。
//!
//! 方法体统一规范化（折叠点 → VM 常量剪枝），只保存与原字节码不同的方法体。

use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;
use std::sync::Arc;

use classfile::{op, ClassFile, Const, Insn, Method, MemberRef, Operand};
use closure::engine::Level;
use closure::manifest::Domain;
use resolve::classpath::{ClassPath, Origin};
use ty::{ClassInfo, Registry};

use crate::facts::ClosureFacts;
use crate::handwritten::HandwrittenMap;
use crate::manifest::RuntimeManifest;
use crate::norm::{apply_fold, CodeOps, NInsn, NormCode};
use crate::par::par_map;
use crate::InputError;

/// 方法键 (类, 方法名, 描述符)
pub type MethodKey = (String, String, String);

fn key_of(r: &MemberRef) -> MethodKey {
    (r.owner.clone(), r.name.clone(), r.desc.clone())
}

/// 一个 lib crate 的输入规格
#[derive(Debug, Clone)]
pub struct LibCrate {
    pub name: String,
    /// jar 内全部类（按名排序，不含 module-info）
    pub jar_classes: Vec<String>,
    /// 整包模式：jar 全部类发射进 crate；否则只发射闭包触达的 jar 类
    pub wholesale: bool,
}

impl LibCrate {
    /// 从 jar 档案枚举类
    pub fn from_jar(name: &str, jar: &Path, wholesale: bool) -> Result<LibCrate, InputError> {
        let a = classfile::archive::Archive::open(jar).map_err(|e| InputError::Io(format!("{}：{e}", jar.display())))?;
        let jar_classes = a
            .class_names()
            .into_iter()
            .filter(|n| n != "module-info" && !n.ends_with("/module-info"))
            .collect();
        Ok(LibCrate {
            name: name.to_string(),
            jar_classes,
            wholesale,
        })
    }
}

/// 构建输入
pub struct BuildInput<'a> {
    /// 用户档案（Origin::User）+ lib jar + JDK 模块 / 镜像
    pub cp: &'a ClassPath,
    pub facts: &'a ClosureFacts,
    pub manifest: &'a RuntimeManifest,
    /// 本编译单元的用户类（入口类在前）
    pub user_classes: &'a [String],
    pub libs: &'a [LibCrate],
    /// 手写层真源 `runtime/java_runtime/src`
    pub runtime_src: &'a Path,
    /// 规范化并行度：0 = 可用核数，1 = 串行（结果与并行度无关）
    pub jobs: usize,
}

/// 反射面事实
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReflectFacts {
    /// 类 → 按名反射访问的成员名
    pub consts: BTreeMap<String, BTreeSet<String>>,
    /// 全成员反射的类
    pub all_members: BTreeSet<String>,
    /// 调用链方法体内的标识符形字符串常量（反射字段名候选）
    pub field_names: BTreeSet<String>,
}

/// 发射层输入
pub struct EmitInput {
    pub registry: Registry,
    pub user_classes: Vec<String>,
    /// lib crate 名 → 发射类（声明序）
    pub lib_crates: Vec<(String, Vec<String>)>,
    /// java_runtime 发射的 JDK 类（闭包序）
    pub jdk_classes: Vec<String>,
    /// 调用链：已解析方法 ∪ 继承槽位需求的调用点符号键 ∪ `<clinit>`
    pub visited: BTreeSet<MethodKey>,
    /// L1（名字级）类：只发不透明类型（`java_class_opaque!`），无字段 / 方法 / vtable
    pub opaque: BTreeSet<String>,
    pub reflect: ReflectFacts,
    pub annotation_enum_seeds: Vec<String>,
    /// 按类镜像强制初始化的目标类（需类初始化钩子）
    pub mirror_init_classes: Vec<String>,
    /// 经虚分派到达的实现（分析器 `dispatched`）：vtable 槽条目对未派发到的实现发存根（C3 第 5 项）
    pub dispatched: BTreeSet<MethodKey>,
    /// 已实例化的类（分析器 `instantiated`）：槽是否保留按实例化类实际选中的实现判定
    pub instantiated: BTreeSet<String>,
    /// 模块资源（清单序；缺失者不列）
    pub module_resources: Vec<(String, Vec<u8>)>,
    /// 预检链事实：分析器方法节点 id（`类.方法:描述符`）
    pub precheck_visited: BTreeSet<String>,
    pub handwritten: HandwrittenMap,
    /// 可观测的告警（无法装载的闭包类、缺失类、反射缺口）
    pub warnings: Vec<String>,
    /// 规范化后与原字节码不同的方法体
    normalized: BTreeMap<MethodKey, NormCode>,
    /// 构建各步耗时（观测用，`--perf` 报告；不影响输出）
    pub timings: Vec<(&'static str, std::time::Duration)>,
}

impl EmitInput {
    /// 方法的规范化方法体（无 Code 属性 → None）
    pub fn code<'s>(&'s self, cls: &str, m: &'s Method) -> Option<Cow<'s, NormCode>> {
        let code = m.code.as_ref()?;
        let k = (cls.to_string(), m.name.clone(), m.desc.clone());
        Some(match self.normalized.get(&k) {
            Some(n) => Cow::Borrowed(n),
            None => Cow::Owned(NormCode::raw(code)),
        })
    }

    /// 方法体指令视图（同 [`EmitInput::code`] 的取值，不复制指令；只读扫描用）
    pub fn code_ops<'s>(&'s self, cls: &str, m: &'s Method) -> Option<CodeOps<'s>> {
        let code = m.code.as_ref()?;
        let k = (cls.to_string(), m.name.clone(), m.desc.clone());
        Some(match self.normalized.get(&k) {
            Some(n) => CodeOps::Norm(n),
            None => CodeOps::Raw(code),
        })
    }

    /// 规范化改动过的方法（golden 对照 / 审计用）
    pub fn normalized(&self) -> &BTreeMap<MethodKey, NormCode> {
        &self.normalized
    }
}

/// 闭包类的装载口径：lib / JDK / 镜像档案（用户档案只经本编译单元进入）
fn load(cp: &ClassPath, name: &str) -> Option<Arc<ClassFile>> {
    if cp.origin(name) == Some(Origin::User) {
        return None;
    }
    cp.get(name)
}

fn declares(cf: &ClassFile, name: &str, desc: &str) -> bool {
    cf.methods.iter().any(|m| m.name == name && m.desc == desc)
}

/// 闭包类 → (非用户域的可装载类序, L1 类（含用户类）, 告警)
fn closure_classes(inp: &BuildInput<'_>, warnings: &mut Vec<String>) -> (Vec<Arc<ClassFile>>, BTreeSet<String>) {
    let users: BTreeSet<&str> = inp.user_classes.iter().map(String::as_str).collect();
    let mut out = Vec::new();
    let mut opaque = BTreeSet::new();
    for c in &inp.facts.classes {
        if c.domain == Domain::Root {
            continue;
        }
        if c.level == Level::Type {
            opaque.insert(c.name.clone());
        }
        if c.domain == Domain::User && users.contains(c.name.as_str()) {
            continue;
        }
        let Some(cf) = load(inp.cp, &c.name) else {
            warnings.push(format!("闭包类无法装载：{}", c.name));
            continue;
        };
        out.push(cf);
    }
    (out, opaque)
}

fn visited_of(inp: &BuildInput<'_>) -> BTreeSet<MethodKey> {
    let f = inp.facts;
    let boundary: BTreeSet<&str> = f
        .classes
        .iter()
        .filter(|c| c.domain == Domain::Boundary)
        .map(|c| c.name.as_str())
        .collect();
    let mut visited: BTreeSet<MethodKey> = f
        .methods
        .iter()
        .filter(|m| !(m.id.name == "<clinit>" && m.kind.is_boundary()))
        .map(|m| key_of(&m.id))
        .collect();
    let clinits: Vec<MethodKey> = f
        .clinit
        .iter()
        .map(|c| (c.clone(), "<clinit>".to_string(), "()V".to_string()))
        .filter(|k| !boundary.contains(k.0.as_str()) || visited.contains(k))
        .collect();
    visited.extend(clinits);
    for r in &f.refs {
        let k = key_of(r);
        if visited.contains(&k) || boundary.contains(k.0.as_str()) {
            continue;
        }
        if load(inp.cp, &k.0).is_some_and(|cf| declares(&cf, &k.1, &k.2)) {
            continue;
        }
        visited.insert(k);
    }
    visited
}

/// Python `str.isidentifier` 近似：首字符字母或 `_`，其余字母数字或 `_`
fn is_identifier(s: &str) -> bool {
    let mut it = s.chars();
    match it.next() {
        Some(c) if c.is_alphabetic() || c == '_' => it.all(|c| c.is_alphanumeric() || c == '_'),
        _ => false,
    }
}

fn ldc_string(i: &Insn) -> Option<&str> {
    match &i.operand {
        Operand::Ldc(Const::String(s)) if matches!(i.opcode, op::LDC | op::LDC_W | op::LDC2_W) => Some(s),
        _ => None,
    }
}

/// (lib crate 名, 该 crate 的类), JDK 类
type LibSplit = (Vec<(String, Vec<String>)>, Vec<String>);

impl<'a> BuildInput<'a> {
    fn lib_split(&self, closure: &[Arc<ClassFile>]) -> Result<LibSplit, InputError> {
        let mut crate_of: BTreeMap<&str, usize> = BTreeMap::new();
        for (i, l) in self.libs.iter().enumerate() {
            for n in &l.jar_classes {
                crate_of.insert(n, i);
            }
        }
        let mut found: Vec<BTreeSet<String>> = vec![BTreeSet::new(); self.libs.len()];
        let mut jdk = Vec::new();
        for cf in closure {
            match crate_of.get(cf.name.as_str()) {
                Some(&i) => {
                    found[i].insert(cf.name.clone());
                }
                None => jdk.push(cf.name.clone()),
            }
        }
        let mut libs = Vec::new();
        for (l, disc) in self.libs.iter().zip(found) {
            let classes = if l.wholesale {
                l.jar_classes.clone()
            } else {
                disc.into_iter().collect()
            };
            libs.push((l.name.clone(), classes));
        }
        Ok((libs, jdk))
    }

    fn registry(&self, libs: &[(String, Vec<String>)], jdk: &[String]) -> Result<Registry, InputError> {
        let mut reg = Registry::new();
        for n in self.user_classes {
            let cf = self
                .cp
                .get(n)
                .ok_or_else(|| InputError::Io(format!("用户类无法装载：{n}")))?;
            reg.insert(cf);
        }
        for n in libs.iter().flat_map(|(_, v)| v).chain(jdk) {
            let cf = load(self.cp, n).ok_or_else(|| InputError::Io(format!("发射类无法装载：{n}")))?;
            reg.insert(cf);
        }
        Ok(reg)
    }

    /// 全部 registry 类的方法体规范化（折叠点 → VM 常量剪枝），只留改动过的。
    ///
    /// 逐类并行：各类方法体规范化互不依赖，结果按类序归并入 BTreeMap（与并行度无关）；
    /// 出错时报类序最先的错误，与串行遍历一致。
    fn normalize(&self, reg: &Registry) -> Result<BTreeMap<MethodKey, NormCode>, InputError> {
        let classes: Vec<&ClassInfo> = reg.iter().collect();
        let per_class = par_map(self.jobs, &classes, |ci| self.normalize_class(ci));
        let mut out = BTreeMap::new();
        for r in per_class {
            out.extend(r?);
        }
        Ok(out)
    }

    fn normalize_class(&self, ci: &ClassInfo) -> Result<Vec<(MethodKey, NormCode)>, InputError> {
        let vmc = &self.manifest.vm_constants;
        let mut out = Vec::new();
        for m in ci.methods() {
            let Some(code) = &m.code else { continue };
            let id = format!("{}.{}:{}", ci.name(), m.name, m.desc);
            let (base, folded) = match self.facts.folds.get(&id) {
                Some(f) => (apply_fold(&id, code, f)?, true),
                None => (NormCode::raw(code), false),
            };
            let before = base.insns.len();
            let NormCode { max_stack, max_locals, code_len, insns, exception_table } = base;
            let insns = vmc.prune(insns, &exception_table);
            if folded || insns.len() != before {
                let n = NormCode { max_stack, max_locals, code_len, insns, exception_table };
                out.push(((ci.name().to_string(), m.name.clone(), m.desc.clone()), n));
            }
        }
        Ok(out)
    }

    fn reflect(&self, closure: &[Arc<ClassFile>], visited: &BTreeSet<MethodKey>, norm: &BTreeMap<MethodKey, NormCode>) -> ReflectFacts {
        let f = self.facts;
        let mut consts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for r in &f.reflect_members {
            consts.entry(r.owner.clone()).or_default().insert(r.name.clone());
        }
        for (owner, names) in &f.seeds.reflect_names {
            consts.entry(owner.clone()).or_default().extend(names.iter().cloned());
        }
        let by_name: BTreeMap<&str, &ClassFile> = closure.iter().map(|c| (c.name.as_str(), &**c)).collect();
        let mut field_names = BTreeSet::new();
        for k in visited {
            let Some(cf) = by_name.get(k.0.as_str()) else { continue };
            for m in cf.methods.iter().filter(|m| m.name == k.1 && m.desc == k.2) {
                let Some(code) = &m.code else { continue };
                // 未改写的方法体直接读原字节码，不复制成 NormCode
                let ops: Box<dyn Iterator<Item = &Insn>> = match norm.get(k) {
                    Some(n) => Box::new(n.insns.iter().filter_map(|x| match x {
                        NInsn::Op(i) => Some(i),
                        _ => None,
                    })),
                    None => Box::new(code.insns.iter()),
                };
                let strs = ops.filter_map(ldc_string).filter(|s| is_identifier(s));
                field_names.extend(strs.map(str::to_string));
            }
        }
        ReflectFacts {
            consts,
            all_members: f.seeds.reflect_all.clone(),
            field_names,
        }
    }

    /// 构建发射层输入
    pub fn build(&self) -> Result<EmitInput, InputError> {
        let f = self.facts;
        let mut timings = Vec::new();
        let mut since = std::time::Instant::now();
        let mut lap = |name: &'static str| {
            let now = std::time::Instant::now();
            timings.push((name, now - since));
            since = now;
        };
        let mut warnings = Vec::new();
        let (closure, opaque) = closure_classes(self, &mut warnings);
        let visited = visited_of(self);
        lap("input.closure");
        let (lib_crates, jdk_classes) = self.lib_split(&closure)?;
        let registry = self.registry(&lib_crates, &jdk_classes)?;
        lap("input.registry");
        let normalized = self.normalize(&registry)?;
        lap("input.normalize");
        let reflect = self.reflect(&closure, &visited, &normalized);
        let module_resources = self
            .manifest
            .module_resource_paths
            .iter()
            .filter_map(|p| self.cp.resource(p).map(|b| (p.clone(), b)))
            .collect();
        warnings.extend(f.missing.iter().map(|m| format!("闭包引用的类不存在：{m}")));
        warnings.extend(f.reflect_gaps.iter().map(|g| format!("反射缺口：{g}")));
        lap("input.reflect");
        let handwritten = HandwrittenMap::scan(self.runtime_src, &registry, self.jobs);
        lap("input.handwritten");
        Ok(EmitInput {
            user_classes: self.user_classes.to_vec(),
            lib_crates,
            jdk_classes,
            visited,
            opaque,
            reflect,
            annotation_enum_seeds: f.seeds.annotation_enums.clone(),
            mirror_init_classes: f.seeds.mirror_inits.clone(),
            dispatched: f.dispatched.iter().map(key_of).collect(),
            instantiated: f.instantiated.iter().cloned().collect(),
            module_resources,
            precheck_visited: f.methods.iter().map(|m| m.id.to_string()).collect(),
            handwritten,
            warnings,
            normalized,
            registry,
            timings,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::is_identifier;

    #[test]
    fn identifier_approximation() {
        assert!(is_identifier("value"));
        assert!(is_identifier("_x1"));
        assert!(!is_identifier("1x"));
        assert!(!is_identifier("a-b"));
        assert!(!is_identifier(""));
    }
}

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
    /// crate 名 = 模块 crate 名（`resolve::modules` 按模块名映射，冲突加 FNV 后缀）
    pub name: String,
    /// jar 内全部类（按名排序，不含 module-info）；发射集 = 闭包触达的子集
    /// （整包翻译以显式种子 `--seed-class` 覆盖全部类达成，不再是 crate 属性）
    pub jar_classes: Vec<String>,
}

impl LibCrate {
    /// 从 jar 档案枚举类（`release`：多版本 jar 视图）
    pub fn from_jar(name: &str, jar: &Path, release: u32) -> Result<LibCrate, InputError> {
        let a = classfile::archive::Archive::open(jar, release).map_err(|e| InputError::Io(format!("{}：{e}", jar.display())))?;
        let jar_classes = a
            .class_names()
            .into_iter()
            .filter(|n| n != "module-info" && !n.ends_with("/module-info"))
            .collect();
        Ok(LibCrate { name: name.to_string(), jar_classes })
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

/// 不经构造器分配的伪成员名（L3 分派闭包的 `<alloc>` 臂；`reflect_dispatch` 协议同名）
pub const ALLOC_MEMBER: &str = "<alloc>";

/// 反射面事实
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ReflectFacts {
    /// 类 → 按名反射访问的成员名
    pub consts: BTreeMap<String, BTreeSet<String>>,
    /// 全成员反射的类
    pub all_members: BTreeSet<String>,
    /// 类 → 按名查字段点到的字段名（闭包分析器按值流求得）
    pub fields: BTreeMap<String, BTreeSet<String>>,
    /// 按名查字段目标类推不出时的字面量名（任意类的同名字段）
    pub field_names: BTreeSet<String>,
    /// 类 → 经字段枚举 / 静态字段句柄常量可按名读写的静态字段名
    pub static_fields: BTreeMap<String, BTreeSet<String>>,
    /// 档案侧方法 / 构造器表保留整表的类（其余类 0 行；用户侧不裁剪）
    pub meta_methods: BTreeSet<String>,
    /// 档案侧字段表保留整表的类
    pub meta_fields: BTreeSet<String>,
}

/// 发射层输入
pub struct EmitInput {
    pub registry: Registry,
    /// 根类（`Domain::Root`，即 Object）：不入注册表（类型结构由手写 `object.rs` 承载），
    /// 非 native 方法按字节码翻译成根方法体（`emit::project::root_bodies`）
    pub root: Option<ClassInfo>,
    pub user_classes: Vec<String>,
    /// lib crate 名 → 发射类（声明序 = 依赖方向，见 `lib_order`）
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
    /// 手写体继承成员需求（分析器 `hw_inherited`）：接收者类须承载的祖先实例方法（接收者, 方法名, 描述符）
    pub hw_inherited: Vec<MethodKey>,
    /// lambda 站点的函数式接口（分析器 `sam_types`；档案发射时 JDK 侧取档案）：`I__Lambda` 合成集
    pub sam_types: BTreeSet<String>,
    /// 构建期引导映像（分析器 `boot_image_data`）：发射层物化映像区与启动序列
    pub boot_image: closure::image::ImageData,
    /// 模块资源（嵌入本程序 jimage，`NativeImageBuffer.getNativeMap` 映射）：档案侧（非用户域调用链上方法体
    /// 推导，并入分析器按名求出的资源）与用户侧（用户类调用链上方法体指名的 JDK 模块资源，如
    /// `ClassLoader.getSystemResourceAsStream("java/lang/String.class")`）取并，按 (资源名, 模块) 有序去重
    pub module_resources: Vec<crate::resources::ModuleResource>,
    /// 类路径资源（§30.15）：应用类路径的全部文件 (资源名, 字节)，按名有序、同名按类路径序；
    /// 读表入口不在调用链上时为空
    pub class_path_resources: Vec<(String, Vec<u8>)>,
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

    /// 规范化改动过的方法（审计用）
    pub fn normalized(&self) -> &BTreeMap<MethodKey, NormCode> {
        &self.normalized
    }

    /// 调用链成员判定：方法键在 [`EmitInput::visited`] 中（闭包分析器的已解析方法 ∪ 调用点符号键 ∪
    /// 类初始化）。用户类与非用户类同一口径——不在链上的方法一律发 `panic!("stub: …")` 存根
    pub fn in_chain(&self, cls: &str, name: &str, desc: &str) -> bool {
        self.visited.contains(&(cls.to_string(), name.to_string(), desc.to_string()))
    }

    /// 类在分析器的类初始化集合中（`visited` 含 `(类, <clinit>, ()V)`；类未声明 `<clinit>` 时同样登记，
    /// 如只经 getstatic / putstatic 默认值静态字段而触发初始化的类）
    pub fn initialized(&self, cls: &str) -> bool {
        self.in_chain(cls, CLINIT_NAME, CLINIT_DESC)
    }

    /// 类型存根（仅类型身份：不发 `__clinit`、静态字段发 panic 存根访问器）：没有任何声明方法在调用链上，
    /// 且类不在初始化集合中。用户类与非用户类同一口径
    pub fn type_only(&self, ci: &ClassInfo) -> bool {
        let n = ci.name();
        !self.initialized(n) && !ci.methods().iter().any(|m| self.in_chain(n, &m.name, &m.desc))
    }
}

/// 类初始化方法名与描述符（[`EmitInput::initialized`] 的键）
const CLINIT_NAME: &str = "<clinit>";
const CLINIT_DESC: &str = "()V";

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

/// 根类（闭包事实中 `Domain::Root` 的类）：方法体按字节码翻译，不入注册表
fn root_class(inp: &BuildInput<'_>, warnings: &mut Vec<String>) -> Option<ClassInfo> {
    let c = inp.facts.classes.iter().find(|c| c.domain == Domain::Root)?;
    match inp.cp.get(&c.name) {
        Some(cf) => Some(ClassInfo::new(cf)),
        None => {
            warnings.push(format!("根类无法装载：{}", c.name));
            None
        }
    }
}

/// 预检链事实：分析器方法节点 id；abstract 方法除外（无方法体，派发落到子类实现，
/// 其存根体不可达——边界类上的 abstract 方法由手写子类经 vtable 实现，不是缺口）
fn precheck_visited(f: &ClosureFacts, closure: &[Arc<ClassFile>]) -> BTreeSet<String> {
    let by_name: BTreeMap<&str, &ClassFile> = closure.iter().map(|c| (c.name.as_str(), &**c)).collect();
    let is_abstract = |r: &MemberRef| {
        by_name.get(r.owner.as_str()).and_then(|c| c.method(&r.name, &r.desc)).is_some_and(|m| m.is_abstract())
    };
    f.methods.iter().filter(|m| !is_abstract(&m.id)).map(|m| m.id.to_string()).collect()
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
        .map(|c| (c.clone(), CLINIT_NAME.to_string(), CLINIT_DESC.to_string()))
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

/// 调用链上方法体的 ldc 字符串常量：(所在类, 字符串)。未改写的方法体直接读原字节码，不复制成 NormCode
fn visited_strings<'c>(
    closure: &'c [Arc<ClassFile>],
    visited: &BTreeSet<MethodKey>,
    norm: &'c BTreeMap<MethodKey, NormCode>,
) -> crate::resources::Strings<'c> {
    let by_name: BTreeMap<&str, &ClassFile> = closure.iter().map(|c| (c.name.as_str(), &**c)).collect();
    let mut out = crate::resources::Strings::default();
    for k in visited {
        let Some(cf) = by_name.get(k.0.as_str()) else { continue };
        for m in cf.methods.iter().filter(|m| m.name == k.1 && m.desc == k.2) {
            let Some(code) = &m.code else { continue };
            let ops: Box<dyn Iterator<Item = &'c Insn>> = match norm.get(k) {
                Some(n) => Box::new(n.insns.iter().filter_map(|x| match x {
                    NInsn::Op(i) => Some(i),
                    _ => None,
                })),
                None => Box::new(code.insns.iter()),
            };
            out.add_method(cf.name.as_str(), ops.filter_map(ldc_string));
        }
    }
    out
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
            // 发射集 = 闭包触达的子集；整包翻译由显式种子覆盖全部类（触达 = 全部）
            libs.push((l.name.clone(), disc.into_iter().collect()));
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

    fn reflect(&self) -> ReflectFacts {
        let f = self.facts;
        let mut consts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for r in &f.reflect_members {
            consts.entry(r.owner.clone()).or_default().insert(r.name.clone());
        }
        for (owner, names) in &f.seeds.reflect_names {
            consts.entry(owner.clone()).or_default().extend(names.iter().cloned());
        }
        for c in &f.reflect_allocations {
            consts.entry(c.clone()).or_default().insert(ALLOC_MEMBER.to_string());
        }
        let mut fields: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (owner, name) in &f.reflect_fields {
            fields.entry(owner.clone()).or_default().insert(name.clone());
        }
        let mut static_fields: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for (owner, name) in &f.reflect_static_fields {
            static_fields.entry(owner.clone()).or_default().insert(name.clone());
        }
        ReflectFacts {
            consts,
            all_members: f.seeds.reflect_all.clone(),
            fields,
            field_names: f.reflect_field_names.iter().cloned().collect(),
            static_fields,
            meta_methods: f.reflect_meta_methods.iter().cloned().collect(),
            meta_fields: f.reflect_meta_fields.iter().cloned().collect(),
        }
    }

    /// 类路径资源表的读取入口是否在调用链上（`[class_path] resource_readers`）
    fn class_path_read(&self, visited: &BTreeSet<MethodKey>) -> bool {
        self.manifest.class_path_readers.iter().any(|m| {
            let Some((cls, rest)) = m.split_once('.') else { return false };
            let Some((name, desc)) = rest.split_once(':') else { return false };
            visited.contains(&(cls.to_string(), name.to_string(), desc.to_string()))
        })
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
        let files: BTreeMap<&str, &Arc<ClassFile>> = closure.iter().map(|c| (c.name.as_str(), c)).collect();
        let lib_crates = crate::lib_order::order(lib_crates, &files, &visited);
        let registry = self.registry(&lib_crates, &jdk_classes)?;
        lap("input.registry");
        let root = root_class(self, &mut warnings);
        let mut normalized = self.normalize(&registry)?;
        if let Some(r) = &root {
            normalized.extend(self.normalize_class(r)?);
        }
        lap("input.normalize");
        let strings = visited_strings(&closure, &visited, &normalized);
        let reflect = self.reflect();
        let user_files: Vec<Arc<ClassFile>> = self.user_classes.iter().filter_map(|c| self.cp.get(c)).collect();
        let user_strings = visited_strings(&user_files, &visited, &normalized);
        let mut module_resources = crate::resources::derive(self.cp, &strings, &f.seeds.named_resources);
        module_resources.extend(crate::resources::derive(self.cp, &user_strings, &BTreeSet::new()));
        module_resources.sort_by(|a, b| (&a.name, &a.module).cmp(&(&b.name, &b.module)));
        module_resources.dedup_by(|a, b| a.name == b.name && a.module == b.module);
        let class_path_resources = if self.class_path_read(&visited) { self.cp.class_path_files() } else { Vec::new() };
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
            hw_inherited: f.hw_inherited.iter().map(key_of).collect(),
            sam_types: f.sam_types.iter().cloned().collect(),
            boot_image: f.boot_image.clone(),
            module_resources,
            class_path_resources,
            precheck_visited: precheck_visited(f, &closure),
            handwritten,
            warnings,
            normalized,
            registry,
            root,
            timings,
        })
    }
}


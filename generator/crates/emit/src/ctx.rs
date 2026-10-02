//! 发射上下文（不可变输入）与每项目状态（Python 模块级全局账本的显式化）。

use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, RwLock};

use classfile::extras::{parse_extras, ClassExtras};
use closure::seeds::SeedCfg;
use input::{Boundary, EmitInput, Planner, RuntimeManifest};
use resolve::classpath::ClassPath;
use ty::{ClassInfo, NameScope, ShortNames, TyCtx};

use crate::error::{EmitError, Result};

/// 发射选项
#[derive(Debug, Clone, Default)]
pub struct EmitOptions {
    /// 严格模式（`java_runtime/strict.txt`）
    pub strict: bool,
    /// 语料 JDK 特性版本（`java_runtime/jdk_feature.txt`；None 不写）
    pub jdk_major: Option<u32>,
    /// 用户 .java 源文件（package 声明来源）
    pub java_files: Vec<PathBuf>,
    /// 批量模式：入口写 `user/src/bin/<bin>.rs`，向 user/Cargo.toml 追加 `[[bin]]`
    pub batch: bool,
    /// 调试：审计逐条明细（存根兜底位点、静默兜底触发）
    pub debug: bool,
    /// 逐类发射的并行度（0 = 可用核数；1 = 串行）。输出与并行度无关
    pub jobs: usize,
}

/// 发射共享上下文：输入事实 + 类型层 + 清单（全部只读；缓存经内部可变性）。
/// 发射代码经 [`EmitCtx`] 视图访问；名字只经视图的 `ty`（带文件作用域）取得
pub struct EmitShared<'a> {
    pub input: &'a EmitInput,
    pub ty: TyCtx<'a>,
    pub manifest: &'a RuntimeManifest,
    pub cp: &'a ClassPath,
    pub planner: Planner<'a>,
    pub boundary: Boundary<'a>,
    /// `runtime/java_runtime`（手写真源）
    pub runtime_dir: PathBuf,
    /// `runtime/rava_macros`（宏 crate 绝对路径）
    pub macros_crate: PathBuf,
    pub seeds: SeedCfg,
    pub opts: EmitOptions,
    /// 静默兜底审计（`[fallback-audit]`）
    pub fallback: crate::fallback::FallbackAudit,
    extras: RwLock<HashMap<String, Arc<ClassExtras>>>,
    subtype_children: OnceLock<BTreeMap<String, Vec<String>>>,
    root_api: OnceLock<BTreeSet<String>>,
    chain_slots: OnceLock<BTreeMap<String, BTreeSet<(String, String)>>>,
    root_keys: OnceLock<BTreeSet<(String, String)>>,
    sam: OnceLock<crate::sam::SamLedger>,
    instr_facts: OnceLock<instr::InstrFacts>,
    lib_crate_of: OnceLock<HashMap<String, String>>,
    /// 内建加载器的模块映射（定义加载器属性，见 `closure::loaders`）
    loaders: OnceLock<closure::loaders::DefiningLoaders>,
    /// vtable 槽族裁剪计划与逐方法判定缓存（C3 第 5 项，见 `vtable_prune`）
    pub(crate) slot_plan: OnceLock<crate::vtable_prune::SlotPlan>,
    pub(crate) slot_memo: Mutex<HashMap<(String, String, String), bool>>,
}

/// 发射上下文视图：共享上下文 + 本文件的类型层（名字经文件作用域认领）
#[derive(Clone, Copy)]
pub struct EmitCtx<'a> {
    sh: &'a EmitShared<'a>,
    pub ty: TyCtx<'a>,
}

impl<'a> std::ops::Deref for EmitCtx<'a> {
    type Target = EmitShared<'a>;
    fn deref(&self) -> &EmitShared<'a> {
        self.sh
    }
}

impl<'a> EmitCtx<'a> {
    /// 换上文件作用域的视图（该文件的全部文本经此视图生成）
    pub fn scoped<'b>(&'b self, scope: &'b NameScope) -> EmitCtx<'b> {
        EmitCtx { sh: self.sh, ty: self.ty.scoped(scope) }
    }

    /// 无作用域视图（分析期查询：结构化引用集等不取名的计算）
    pub fn unscoped(&self) -> EmitCtx<'a> {
        self.sh.view()
    }

    /// 本处的 Rust 类型名（经文件作用域）
    pub fn short(&self, binary: &str) -> String {
        self.ty.short(binary)
    }

    /// 类的定义名（类自己文件里的 struct 名；跨文件全路径末段）
    pub fn declared(&self, binary: &str) -> String {
        self.ty.global_names().declared(binary)
    }
}

impl<'a> EmitShared<'a> {
    /// 无作用域视图
    pub fn view(&self) -> EmitCtx<'_> {
        EmitCtx { sh: self, ty: self.ty }
    }

    pub fn new(
        input: &'a EmitInput,
        names: &'a ShortNames,
        manifest: &'a RuntimeManifest,
        cp: &'a ClassPath,
        runtime_dir: &Path,
        opts: EmitOptions,
    ) -> Result<EmitShared<'a>> {
        let seeds_path = runtime_dir.join("seeds.toml");
        let seeds_text = std::fs::read_to_string(&seeds_path).map_err(|e| EmitError::Io(format!("{}：{e}", seeds_path.display())))?;
        let table: toml::Table = seeds_text
            .parse()
            .map_err(|e| EmitError::Input(format!("{}：{e}", seeds_path.display())))?;
        let runtime_dir = std::path::absolute(runtime_dir).unwrap_or_else(|_| runtime_dir.to_path_buf());
        let macros_crate = runtime_dir.parent().map_or_else(|| PathBuf::from("rava_macros"), |p| p.join("rava_macros"));
        Ok(EmitShared {
            input,
            ty: TyCtx::new(&input.registry, names, &manifest.ty),
            manifest,
            cp,
            planner: Planner::new(input, names, manifest, cp),
            boundary: Boundary::new(manifest, cp),
            runtime_dir,
            macros_crate,
            seeds: SeedCfg::from_toml(&table),
            fallback: crate::fallback::FallbackAudit::new(opts.debug),
            opts,
            extras: RwLock::new(HashMap::new()),
            subtype_children: OnceLock::new(),
            root_api: OnceLock::new(),
            chain_slots: OnceLock::new(),
            root_keys: OnceLock::new(),
            sam: OnceLock::new(),
            instr_facts: OnceLock::new(),
            lib_crate_of: OnceLock::new(),
            loaders: OnceLock::new(),
            slot_plan: OnceLock::new(),
            slot_memo: Mutex::new(HashMap::new()),
        })
    }

    /// 类的定义加载器（类块 `defining_loader` 属性；引导加载器 → None）
    pub fn defining_loader(&self, cls: &str) -> Option<&'static str> {
        let l = self.loaders.get_or_init(|| closure::loaders::DefiningLoaders::new(self.cp, self.manifest.vm_state.loader_map.as_ref()));
        l.loader_of(self.cp, cls).attr()
    }

    /// 手写真源 `runtime/java_runtime/src`
    pub fn runtime_src(&self) -> PathBuf {
        self.runtime_dir.join("src")
    }

    pub fn class(&self, name: &str) -> Option<&'a ClassInfo> {
        self.input.registry.get(name)
    }

    /// 类的补充属性（LVT、注解原始字节、Deprecated 等；按需二次解析并缓存）
    pub fn extras(&self, cls: &str) -> Arc<ClassExtras> {
        if let Some(x) = self.extras.read().unwrap_or_else(|e| e.into_inner()).get(cls) {
            return x.clone();
        }
        // 并发未命中时各自解析（结果只由字节决定），先入者胜出；降级只由入表者计一次
        let parsed = self.cp.bytes(cls).map(|b| parse_extras(&b));
        let mut map = self.extras.write().unwrap_or_else(|e| e.into_inner());
        if let Some(x) = map.get(cls) {
            return x.clone();
        }
        let x = Arc::new(match parsed {
            Some(Ok(x)) => x,
            Some(Err(e)) => {
                self.fallback.record("class-extras", || format!("{cls}: {e}"));
                ClassExtras::default()
            }
            None => ClassExtras::default(),
        });
        map.insert(cls.to_string(), x.clone());
        x
    }

    /// 直接子类型索引：父类 / 直接接口 → 子类型（按 binary 排序；`_get_all_subtypes_ordered`
    /// 逐层遍历排序注册表的等价预计算）
    pub fn subtype_children(&self) -> &BTreeMap<String, Vec<String>> {
        self.subtype_children.get_or_init(|| {
            let mut out: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for ci in self.ty.reg.iter() {
                let mut parents: BTreeSet<&str> = ci.interfaces().iter().map(String::as_str).collect();
                parents.insert(ci.super_class());
                for p in parents.into_iter().filter(|p| !p.is_empty()) {
                    out.entry(p.to_string()).or_default().push(ci.name().to_string());
                }
            }
            out
        })
    }

    /// 手写根类的 API 名面（根类 `object{,_impl,_ext}.rs` 中的 `pub fn` 名；
    /// `member_naming._handwritten_root_api`）
    pub fn root_api(&self) -> &BTreeSet<String> {
        self.root_api.get_or_init(|| {
            let mut names = BTreeSet::new();
            for f in Self::root_files(&self.runtime_src()) {
                if let Ok(text) = std::fs::read_to_string(f) {
                    names.extend(crate::text::pub_fn_names(&text));
                }
            }
            names
        })
    }

    /// 手写根类的源文件 `<src>/<包>/<stem>{_impl,_ext,}.rs`（`src` 为 runtime 真源或 scratch overlay）
    pub fn root_files(src: &Path) -> Vec<PathBuf> {
        let root = ty::consts::OBJECT;
        let (pkg, simple) = root.rsplit_once('/').unwrap_or(("", root));
        let dir = pkg.split('/').fold(src.to_path_buf(), |d, p| d.join(p));
        let stem = crate::text::to_snake(simple);
        ["_impl", "_ext", ""].iter().map(|suffix| dir.join(format!("{stem}{suffix}.rs"))).collect()
    }

    /// 根类的 public 实例方法键 {(名, 参数描述符部分)}（从 JDK 字节码解析；
    /// `member_owner._root_virtual_methods`。与 `input::Planner` 的同名私有集合同源，后续统一）
    pub fn root_keys(&self) -> &BTreeSet<(String, String)> {
        self.root_keys.get_or_init(|| {
            let Some(cf) = self.cp.get(ty::consts::OBJECT) else { return BTreeSet::new() };
            cf.methods
                .iter()
                .filter(|m| !m.is_static() && !m.is_synthetic() && !m.name.starts_with('<'))
                .filter(|m| m.access & classfile::acc::PUBLIC != 0)
                .map(|m| (m.name.clone(), crate::phase2::sig::param_part(&m.desc).to_string()))
                .collect()
        })
    }

    /// 方法体层的全局事实（根类方法集 / 手写根类 API 名面 / 子类索引）：发射层与方法体层
    /// 共用同一成员命名函数时的上下文
    pub fn instr_facts(&self) -> &instr::InstrFacts {
        self.instr_facts.get_or_init(|| {
            let root = self.cp.get(ty::consts::OBJECT);
            instr::InstrFacts::build(self.ty.reg, root.as_deref(), &self.runtime_src())
        })
    }

    /// A-5 可合成函数式接口账本（首次查询时预扫描；方法体生成器在 invokedynamic 站点经
    /// [`crate::sam::SamLedger::site_ctor_path`] 查询构造路径）
    pub fn sam(&self) -> &crate::sam::SamLedger {
        self.sam.get_or_init(|| crate::sam::SamLedger::prescan(&self.view()))
    }

    /// 调用链按类索引的槽位键：类 → {(方法名, 参数描述符部分)}（`_cc_slot_index`）
    pub fn chain_slots(&self) -> &BTreeMap<String, BTreeSet<(String, String)>> {
        self.chain_slots.get_or_init(|| {
            let mut out: BTreeMap<String, BTreeSet<(String, String)>> = BTreeMap::new();
            for (c, n, d) in &self.input.visited {
                let pp = d.find(')').map_or(d.as_str(), |i| &d[..=i]);
                out.entry(c.clone()).or_default().insert((n.clone(), pp.to_string()));
            }
            out
        })
    }

    /// jar 输入模式：类归属的 lib crate 名（非 lib 类 None）
    pub fn lib_crate_of(&self, cls: &str) -> Option<&str> {
        self.lib_crate_of
            .get_or_init(|| {
                let mut m = HashMap::new();
                for (lib, classes) in &self.input.lib_crates {
                    for c in classes {
                        m.entry(c.clone()).or_insert_with(|| lib.clone());
                    }
                }
                m
            })
            .get(cls)
            .map(String::as_str)
    }

    /// 类是否为本编译单元的用户类
    pub fn is_user(&self, cls: &str) -> bool {
        self.input.user_classes.iter().any(|u| u == cls)
    }

    /// 调用链成员判定（用户类恒全量）
    /// L1（名字级）类：发不透明形态（`class_writer::opaque`）
    pub fn is_opaque(&self, cls: &str) -> bool {
        self.input.opaque.contains(cls)
    }

    pub fn in_chain(&self, cls: &str, name: &str, desc: &str) -> bool {
        self.input
            .visited
            .contains(&(cls.to_string(), name.to_string(), desc.to_string()))
    }
}

/// 每项目可变状态：Python 模块级全局（跨类累积）的显式化，按类发射序更新
#[derive(Debug, Default)]
pub struct ProjectState {
    /// inherited_calls：(接收者, 方法名, 参数描述符) 插入序去重
    pub inherited_requests: indexmap::IndexSet<(String, String, String)>,
    /// G-10 账本：调用点引用 (类, 方法名) → Rust 名
    pub lambda_refs: Vec<(String, String, String)>,
    /// G-10 账本：定义 (类, 方法名) → Rust 名集合
    pub lambda_defs: BTreeMap<(String, String), BTreeSet<String>>,
    /// G-10 账本：本轮生成类
    pub generated_classes: BTreeSet<String>,
    /// SAM 合成站点（插入序）
    pub sam_sites: Vec<crate::body::SamSite>,
    /// FS-H0 手写覆盖审计（`raw_audit` 三类登记；发射序）
    pub hw_audit: Vec<(HwAudit, String)>,
    /// 方法体生成日志（审计事实 + 逐方法耗时；发射序）
    pub body_log: crate::body::BodyLog,
}

/// 公开 API 类非 native 方法被手写覆盖的审计类别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HwAudit {
    /// 边界类（`[vm_boundary]` / 未放行的 `[boundary]`）：策略边界
    VmBoundary,
    /// VM 内建函数（`vm_intrinsics.toml` 准入）
    Intrinsic,
    /// 越界覆盖
    Override,
}

impl ProjectState {
    /// 并入一个类的账本增量（按发射序调用：插入序账本与串行发射逐项一致）
    pub fn merge(&mut self, d: ProjectState) {
        self.inherited_requests.extend(d.inherited_requests);
        self.lambda_refs.extend(d.lambda_refs);
        for (k, v) in d.lambda_defs {
            self.lambda_defs.entry(k).or_default().extend(v);
        }
        self.generated_classes.extend(d.generated_classes);
        self.sam_sites.extend(d.sam_sites);
        self.hw_audit.extend(d.hw_audit);
        self.body_log.append(d.body_log);
    }

    /// 方法体生成登记的事实并入账本
    pub fn absorb(&mut self, fx: &crate::body::BodyEffects) {
        for r in &fx.requests {
            self.inherited_requests.insert(r.clone());
        }
        self.lambda_refs.extend(fx.lambda_refs.iter().cloned());
        self.sam_sites.extend(fx.sam_sites.iter().cloned());
    }

    /// G-10 生成期断言：调用点引用的 lambda 实现名须在同类同名方法的定义名集合中；
    /// 被引用类本轮生成过却无该方法定义 → 定义被过滤（`_LambdaNameLedger.check`）
    pub fn check_lambda_ledger(&self) -> Result<()> {
        let mut refs: BTreeMap<(&str, &str), BTreeSet<&str>> = BTreeMap::new();
        for (c, m, r) in &self.lambda_refs {
            refs.entry((c, m)).or_default().insert(r);
        }
        let mut problems = Vec::new();
        for ((cls, mname), names) in &refs {
            if !self.generated_classes.contains(*cls) {
                continue;
            }
            let key = ((*cls).to_string(), (*mname).to_string());
            let Some(defs) = self.lambda_defs.get(&key) else {
                problems.push(format!("{cls}.{mname} → 调用点引用 {names:?}，但该类本轮生成时未输出此方法的任何定义（被过滤/跳过）"));
                continue;
            };
            let drifted: Vec<&&str> = names.iter().filter(|n| !defs.contains(**n)).collect();
            if !drifted.is_empty() {
                problems.push(format!("{cls}.{mname} → 调用点引用 {drifted:?} 不在定义名集合 {defs:?} 中"));
            }
        }
        if problems.is_empty() {
            return Ok(());
        }
        let head: Vec<String> = problems.iter().take(10).map(|p| format!("  [G-10] {p}")).collect();
        let more = if problems.len() > 10 { format!("\n  ...（共 {} 处）", problems.len()) } else { String::new() };
        Err(EmitError::Assert(format!("invokedynamic 实现方法命名/定义不一致（G-10 断言）：\n{}{more}", head.join("\n"))))
    }
}

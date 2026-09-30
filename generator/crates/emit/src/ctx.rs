//! 发射上下文（不可变输入）与每项目状态（Python 模块级全局账本的显式化）。

use std::cell::{OnceCell, RefCell};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use classfile::extras::{parse_extras, ClassExtras};
use closure::seeds::SeedCfg;
use input::{Boundary, EmitInput, Planner, RuntimeManifest};
use resolve::classpath::ClassPath;
use ty::{ClassInfo, ShortNames, TyCtx};

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
}

/// 发射上下文：输入事实 + 类型层 + 清单（全部只读；缓存经内部可变性）
pub struct EmitCtx<'a> {
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
    extras: RefCell<HashMap<String, Rc<ClassExtras>>>,
    subtype_children: OnceCell<BTreeMap<String, Vec<String>>>,
    root_api: OnceCell<BTreeSet<String>>,
}

impl<'a> EmitCtx<'a> {
    pub fn new(
        input: &'a EmitInput,
        names: &'a ShortNames,
        manifest: &'a RuntimeManifest,
        cp: &'a ClassPath,
        runtime_dir: &Path,
        opts: EmitOptions,
    ) -> Result<EmitCtx<'a>> {
        let seeds_path = runtime_dir.join("seeds.toml");
        let seeds_text = std::fs::read_to_string(&seeds_path).map_err(|e| EmitError::Io(format!("{}：{e}", seeds_path.display())))?;
        let table: toml::Table = seeds_text
            .parse()
            .map_err(|e| EmitError::Input(format!("{}：{e}", seeds_path.display())))?;
        let runtime_dir = std::path::absolute(runtime_dir).unwrap_or_else(|_| runtime_dir.to_path_buf());
        let macros_crate = runtime_dir.parent().map_or_else(|| PathBuf::from("rava_macros"), |p| p.join("rava_macros"));
        Ok(EmitCtx {
            input,
            ty: TyCtx::new(&input.registry, names, &manifest.ty),
            manifest,
            cp,
            planner: Planner::new(input, names, manifest, cp),
            boundary: Boundary::new(manifest, cp),
            runtime_dir,
            macros_crate,
            seeds: SeedCfg::from_toml(&table),
            opts,
            extras: RefCell::new(HashMap::new()),
            subtype_children: OnceCell::new(),
            root_api: OnceCell::new(),
        })
    }

    /// 手写真源 `runtime/java_runtime/src`
    pub fn runtime_src(&self) -> PathBuf {
        self.runtime_dir.join("src")
    }

    pub fn class(&self, name: &str) -> Option<&'a ClassInfo> {
        self.input.registry.get(name)
    }

    /// 短名（`ShortNames::short`）
    pub fn short(&self, binary: &str) -> String {
        self.ty.names.short(binary).into_owned()
    }

    /// 类的补充属性（LVT、注解原始字节、Deprecated 等；按需二次解析并缓存）
    pub fn extras(&self, cls: &str) -> Rc<ClassExtras> {
        if let Some(x) = self.extras.borrow().get(cls) {
            return x.clone();
        }
        let x = Rc::new(self.cp.bytes(cls).and_then(|b| parse_extras(&b).ok()).unwrap_or_default());
        self.extras.borrow_mut().insert(cls.to_string(), x.clone());
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
            let root = ty::consts::OBJECT;
            let (pkg, simple) = root.rsplit_once('/').unwrap_or(("", root));
            let dir = pkg.split('/').fold(self.runtime_src(), |d, p| d.join(p));
            let stem = crate::text::to_snake(simple);
            let mut names = BTreeSet::new();
            for f in [format!("{stem}_impl.rs"), format!("{stem}_ext.rs"), format!("{stem}.rs")] {
                if let Ok(text) = std::fs::read_to_string(dir.join(f)) {
                    names.extend(crate::text::pub_fn_names(&text));
                }
            }
            names
        })
    }

    /// 类是否为本编译单元的用户类
    pub fn is_user(&self, cls: &str) -> bool {
        self.input.user_classes.iter().any(|u| u == cls)
    }

    /// 调用链成员判定（用户类恒全量）
    pub fn in_chain(&self, cls: &str, name: &str, desc: &str) -> bool {
        self.input
            .visited
            .contains(&(cls.to_string(), name.to_string(), desc.to_string()))
    }
}

/// 每项目可变状态：Python 模块级全局（跨类累积）的显式化，按类发射序更新
#[derive(Debug, Default)]
pub struct ProjectState {
    /// import_gen `_seen_simples`：简单名 → 首个引入它的完整路径（跨类累积）
    pub seen_simples: BTreeMap<String, String>,
    /// inherited_calls：(接收者, 方法名, 参数描述符) 插入序去重
    pub inherited_requests: indexmap::IndexSet<(String, String, String)>,
    /// G-10 账本：调用点引用 (类, 方法名) → Rust 名
    pub lambda_refs: Vec<(String, String, String)>,
    /// G-10 账本：定义 (类, 方法名) → Rust 名集合
    pub lambda_defs: BTreeMap<(String, String), BTreeSet<String>>,
    /// G-10 账本：本轮生成类
    pub generated_classes: BTreeSet<String>,
    /// SAM 合成站点 (接口, SAM 描述符, 当前类)（插入序）
    pub sam_sites: Vec<(String, String, String)>,
}

impl ProjectState {
    /// 方法体生成登记的事实并入账本
    pub fn absorb(&mut self, fx: &crate::body::BodyEffects) {
        for r in &fx.requests {
            self.inherited_requests.insert(r.clone());
        }
        self.lambda_refs.extend(fx.lambda_refs.iter().cloned());
        self.sam_sites.extend(fx.sam_sites.iter().cloned());
    }
}

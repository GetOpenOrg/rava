//! 发射层读取的 runtime 清单（`codegen/runtime_manifest.py` 发射层消费部分的移植）。
//!
//! 组合类型层清单 [`ty::Manifest`]（txt 清单）与三份结构化清单中发射层需要的部分：
//! - closure.toml：`[boundary]` / `[vm_boundary]` / `[release]`；
//! - seeds.toml：`[jca]` 放行、`[module_resources]`、`[boot_init]`、`[data_bundle]` 载体；
//! - vm_intrinsics.toml：`[[intrinsic]]`、`[caller_sensitive]`、`[sigpoly]`、`[indy]`、`[vm_constants]`。
//!
//! 文件缺失视为空表；格式约定（包条目以 `/` 结尾、类条目不以 `/` 结尾、
//! 内建条目须写 kind 与 reason）违反时返回 [`InputError::Manifest`]。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use toml::{Table, Value};

use crate::prune::VmConstants;
use crate::InputError;

/// invokedynamic 引导方法分类（`[indy]`）
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum IndyKind {
    Native,
    Lambda,
    Concat,
    /// native 的细分；同时出现时优先
    TypeSwitch,
}

impl IndyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            IndyKind::Native => "native",
            IndyKind::Lambda => "lambda",
            IndyKind::Concat => "concat",
            IndyKind::TypeSwitch => "type_switch",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeManifest {
    /// 类型层 txt 清单（签名擦除接口 / 重载缩写）
    pub ty: ty::Manifest,
    /// 内部边界包前缀（`/` 结尾）
    pub boundary_packages: Vec<String>,
    /// VM 耦合边界类
    pub vm_boundary_classes: BTreeSet<String>,
    /// 通用边界放行：包前缀在前、类在后
    pub release: Vec<String>,
    /// K-JCA 放行：包前缀在前、类在后
    pub jca_release: Vec<String>,
    /// 模块资源路径（jmod `classes/` 下相对路径）
    pub module_resource_paths: Vec<String>,
    /// 引导初始化类
    pub boot_init_classes: Vec<String>,
    /// 纯数据资源束载体（`类.方法:描述符`）
    pub data_bundle_carriers: Vec<String>,
    /// VM 内建成员
    pub intrinsic_members: BTreeSet<String>,
    pub caller_sensitive_annotations: BTreeSet<String>,
    pub sigpoly_callsite_typed: BTreeSet<String>,
    /// 引导方法（`类.方法`）→ 分类
    pub indy_kinds: BTreeMap<String, IndyKind>,
    pub vm_constants: VmConstants,
}

fn load_toml(dir: &Path, name: &str) -> Result<Table, InputError> {
    let path = dir.join(name);
    let Ok(text) = std::fs::read_to_string(&path) else {
        return Ok(Table::new());
    };
    text.parse::<Table>()
        .map_err(|e| InputError::Manifest(format!("{name}：{e}")))
}

fn section<'a>(t: &'a Table, name: &str) -> Option<&'a Table> {
    t.get(name).and_then(Value::as_table)
}

fn str_list(sec: Option<&Table>, key: &str, where_: &str) -> Result<Vec<String>, InputError> {
    let Some(v) = sec.and_then(|s| s.get(key)) else {
        return Ok(Vec::new());
    };
    let arr = v
        .as_array()
        .ok_or_else(|| InputError::Manifest(format!("{where_}.{key}：应为字符串数组")))?;
    arr.iter()
        .map(|x| {
            x.as_str()
                .map(str::to_string)
                .ok_or_else(|| InputError::Manifest(format!("{where_}.{key}：条目应为字符串：{x}")))
        })
        .collect()
}

fn packages(sec: Option<&Table>, key: &str, where_: &str) -> Result<Vec<String>, InputError> {
    let v = str_list(sec, key, where_)?;
    if let Some(bad) = v.iter().find(|p| !p.ends_with('/')) {
        return Err(InputError::Manifest(format!("{where_}.{key}：包条目须以 / 结尾：{bad}")));
    }
    Ok(v)
}

fn classes(sec: Option<&Table>, key: &str, where_: &str) -> Result<Vec<String>, InputError> {
    let v = str_list(sec, key, where_)?;
    if let Some(bad) = v.iter().find(|p| p.ends_with('/')) {
        return Err(InputError::Manifest(format!(
            "{where_}.{key}：类条目不得以 / 结尾（包请写入 packages）：{bad}"
        )));
    }
    Ok(v)
}

fn intrinsics(vm: &Table) -> Result<BTreeSet<String>, InputError> {
    let mut out = BTreeSet::new();
    let entries = vm.get("intrinsic").and_then(Value::as_array).cloned().unwrap_or_default();
    for ent in &entries {
        let get = |k: &str| ent.get(k).and_then(Value::as_str).filter(|s| !s.is_empty());
        let member = get("member").unwrap_or_default().to_string();
        if get("kind").is_none() || get("reason").is_none() {
            return Err(InputError::Manifest(format!(
                "vm_intrinsics.toml：内建条目须写明 kind 与 reason：{member}"
            )));
        }
        out.insert(member);
    }
    Ok(out)
}

fn indy_kinds(vm: &Table) -> Result<BTreeMap<String, IndyKind>, InputError> {
    let sec = section(vm, "indy");
    let mut out = BTreeMap::new();
    for kind in [IndyKind::Native, IndyKind::Lambda, IndyKind::Concat, IndyKind::TypeSwitch] {
        for m in str_list(sec, kind.as_str(), "indy")? {
            out.insert(m, kind);
        }
    }
    Ok(out)
}

impl RuntimeManifest {
    /// 从 `runtime/java_runtime` 目录读取
    pub fn load(runtime_dir: &Path) -> Result<RuntimeManifest, InputError> {
        let ty = ty::Manifest::load(runtime_dir).map_err(|e| InputError::Manifest(e.to_string()))?;
        let closure = load_toml(runtime_dir, "closure.toml")?;
        let seeds = load_toml(runtime_dir, "seeds.toml")?;
        let vm = load_toml(runtime_dir, "vm_intrinsics.toml")?;

        let vmb = section(&closure, "vm_boundary");
        let vm_boundary_classes: BTreeSet<String> = classes(vmb, "classes", "vm_boundary")?.into_iter().collect();
        let rel = section(&closure, "release");
        let mut release = packages(rel, "packages", "release")?;
        release.extend(classes(rel, "classes", "release")?);
        let jca = section(&seeds, "jca");
        let mut jca_release = packages(jca, "release_packages", "jca")?;
        jca_release.extend(classes(jca, "release_classes", "jca")?);
        let vmc = section(&vm, "vm_constants");
        Ok(RuntimeManifest {
            ty,
            boundary_packages: packages(section(&closure, "boundary"), "packages", "boundary")?,
            vm_boundary_classes,
            release,
            jca_release,
            module_resource_paths: str_list(section(&seeds, "module_resources"), "paths", "module_resources")?,
            boot_init_classes: classes(section(&seeds, "boot_init"), "classes", "boot_init")?,
            data_bundle_carriers: str_list(section(&seeds, "data_bundle"), "carriers", "data_bundle")?,
            intrinsic_members: intrinsics(&vm)?,
            caller_sensitive_annotations: str_list(section(&vm, "caller_sensitive"), "annotations", "caller_sensitive")?
                .into_iter()
                .collect(),
            sigpoly_callsite_typed: str_list(section(&vm, "sigpoly"), "callsite_typed", "sigpoly")?
                .into_iter()
                .collect(),
            indy_kinds: indy_kinds(&vm)?,
            vm_constants: VmConstants {
                null_returns: str_list(vmc, "null_returns", "vm_constants")?.into_iter().collect(),
                null_to_false: str_list(vmc, "null_to_false", "vm_constants")?.into_iter().collect(),
            },
        })
    }

    /// `[indy]` 分类（未列出 → None，按普通静态调用处理）
    pub fn indy_kind(&self, bsm: &str) -> Option<IndyKind> {
        self.indy_kinds.get(bsm).copied()
    }
}

//! 发射层读取的 runtime 清单。
//!
//! 组合类型层清单 [`ty::Manifest`]（txt 清单）与三份结构化清单中发射层需要的部分：
//! - closure.toml：`[vm_boundary]`（含 `translate_nested` / `clinit_carried`）；
//! - seeds.toml：`[module_resources]`、`[boot_init]`；
//! - vm_intrinsics.toml：`[[intrinsic]]`、`[caller_sensitive]`、`[sigpoly]`、`[indy]`、`[vm_constants]`（含 `injected_statics` 子表）、`[vm_state]`。
//!
//! 文件缺失视为空表；格式约定（类条目不以 `/` 结尾、
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
    /// native 的细分：枚举选择子的模式 switch（标签为枚举常量名 / Class）
    EnumSwitch,
    /// native 的细分：record 的 toString / hashCode / equals
    ObjectMethods,
}

impl IndyKind {
    pub fn as_str(self) -> &'static str {
        match self {
            IndyKind::Native => "native",
            IndyKind::Lambda => "lambda",
            IndyKind::Concat => "concat",
            IndyKind::TypeSwitch => "type_switch",
            IndyKind::EnumSwitch => "enum_switch",
            IndyKind::ObjectMethods => "object_methods",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RuntimeManifest {
    /// 类型层 txt 清单（签名擦除接口 / 重载缩写）
    pub ty: ty::Manifest,
    /// VM 耦合边界类
    pub vm_boundary_classes: BTreeSet<String>,
    /// VM 边界类中按字节码翻译的嵌套类（`[vm_boundary] translate_nested`）
    pub release: Vec<String>,
    /// `<clinit>` 由手写层承载的 VM 边界类（`[vm_boundary] clinit_carried`；其余 VM 边界类的 `<clinit>` 按字节码翻译）
    pub vm_clinit_carried: BTreeSet<String>,
    /// VM 注入的静态字段（`[vm_constants.injected_statics]`）：`类.字段` → crate 根下取值表达式
    pub vm_injected_statics: BTreeMap<String, String>,
    /// 模块资源路径（jmod `classes/` 下相对路径）
    pub module_resource_paths: Vec<String>,
    /// 类路径资源表的读取入口（`[class_path] resource_readers`，`类.方法:描述符`）：其一在调用链上即嵌入类路径资源
    pub class_path_readers: Vec<String>,
    /// 引导初始化类
    pub boot_init_classes: Vec<String>,
    /// 引导期调用的静态方法（`类.方法:()V`）
    pub boot_init_calls: Vec<String>,
    /// VM 引导阶段（`[[boot_init.phases]]`）：`类.方法:描述符` 与实参常量（布尔按 0 / 1）；
    /// 锚点与校验归闭包分析器，入链（锚点可达）的阶段由 main 在引导类初始化之后按序调用
    pub boot_phases: Vec<(String, Vec<i32>)>,
    /// VM 内建成员
    pub intrinsic_members: BTreeSet<String>,
    pub caller_sensitive_annotations: BTreeSet<String>,
    pub sigpoly_callsite_typed: BTreeSet<String>,
    /// 引导方法（`类.方法`）→ 分类
    pub indy_kinds: BTreeMap<String, IndyKind>,
    /// `[indy]` 分量处理入口：`concat_stringify` / `component_hash` / `component_equals` → `类.方法:描述符`
    /// （登记了 concat / object_methods 引导方法时必填，装载时校验）
    pub indy_helpers: BTreeMap<String, String>,
    pub vm_constants: VmConstants,
    /// VM 注入状态的落地（`[vm_state]`：字段访问钩子、模块 → 加载器映射来源）
    pub vm_state: closure::manifest::VmState,
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

fn classes(sec: Option<&Table>, key: &str, where_: &str) -> Result<Vec<String>, InputError> {
    let v = str_list(sec, key, where_)?;
    if let Some(bad) = v.iter().find(|p| p.ends_with('/')) {
        return Err(InputError::Manifest(format!(
            "{where_}.{key}：类条目不得以 / 结尾（清单只收逐类条目）：{bad}"
        )));
    }
    Ok(v)
}

/// `[vm_constants.injected_statics]`：`"类.字段" = "取值表达式"` 或字面量；结果为可直接发射的 Rust 表达式
fn injected_statics(vmc: Option<&Table>) -> Result<BTreeMap<String, String>, InputError> {
    let Some(t) = vmc.and_then(|s| s.get("injected_statics")) else {
        return Ok(BTreeMap::new());
    };
    let t = t
        .as_table()
        .ok_or_else(|| InputError::Manifest("vm_constants.injected_statics：应为表".into()))?;
    t.iter()
        .map(|(k, v)| {
            // 取值：crate 根下的取值表达式（字符串）或字面量（整数 / 布尔，分析器按值折叠）
            let expr = match v {
                Value::String(e) if !e.is_empty() => Some(format!("crate::{e}")),
                Value::Integer(n) => Some(format!("{n}i64")),
                Value::Boolean(b) => Some(b.to_string()),
                _ => None,
            };
            match (k.split_once('.'), expr) {
                (Some((c, f)), Some(e)) if !c.is_empty() && !f.is_empty() => Ok((k.clone(), e)),
                _ => Err(InputError::Manifest(format!(
                    "vm_constants.injected_statics：条目须为 \"类.字段\" = \"取值表达式\" / 字面量：{k}"
                ))),
            }
        })
        .collect()
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
    for kind in [
        IndyKind::Native,
        IndyKind::Lambda,
        IndyKind::Concat,
        IndyKind::TypeSwitch,
        IndyKind::EnumSwitch,
        IndyKind::ObjectMethods,
    ] {
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
        let release = classes(vmb, "translate_nested", "vm_boundary")?;
        let vm_clinit_carried: BTreeSet<String> = classes(vmb, "clinit_carried", "vm_boundary")?.into_iter().collect();
        let boot = section(&seeds, "boot_init");
        let boot_init_calls = str_list(boot, "calls", "boot_init")?;
        if let Some(bad) = boot_init_calls.iter().find(|c| !c.ends_with(":()V") || !c.contains('.')) {
            return Err(InputError::Manifest(format!("boot_init.calls：须为无参静态方法 `类.方法:()V`：{bad}")));
        }
        let vmc = section(&vm, "vm_constants");
        Ok(RuntimeManifest {
            ty,
            vm_boundary_classes,
            vm_clinit_carried,
            release,
            vm_injected_statics: injected_statics(vmc)?,
            module_resource_paths: str_list(section(&seeds, "module_resources"), "paths", "module_resources")?,
            class_path_readers: str_list(section(&vm, "class_path"), "resource_readers", "class_path")?,
            boot_init_classes: classes(boot, "classes", "boot_init")?,
            boot_init_calls,
            boot_phases: boot_phases(&seeds)?,
            intrinsic_members: intrinsics(&vm)?,
            caller_sensitive_annotations: str_list(section(&vm, "caller_sensitive"), "annotations", "caller_sensitive")?
                .into_iter()
                .collect(),
            sigpoly_callsite_typed: str_list(section(&vm, "sigpoly"), "callsite_typed", "sigpoly")?
                .into_iter()
                .collect(),
            indy_kinds: indy_kinds(&vm)?,
            indy_helpers: indy_helpers(&vm)?,
            vm_constants: VmConstants {
                null_returns: str_list(vmc, "null_returns", "vm_constants")?.into_iter().collect(),
                null_to_false: str_list(vmc, "null_to_false", "vm_constants")?.into_iter().collect(),
            },
            vm_state: closure::manifest::VmState::from_toml(&vm).map_err(InputError::Manifest)?,
        })
    }

    /// `[indy]` 分类（未列出 → None，按普通静态调用处理）
    pub fn indy_kind(&self, bsm: &str) -> Option<IndyKind> {
        self.indy_kinds.get(bsm).copied()
    }

    /// `[indy]` 分量处理入口 `(类, 方法, 描述符)`；`key` 为清单键名。
    /// 未登记 → 错误串（装载时已对已登记的引导类别校验必填，只在清单未登记对应引导时出现）
    pub fn indy_helper(&self, key: &str) -> Result<(&str, &str, &str), String> {
        self.indy_helpers
            .get(key)
            .and_then(|m| split_member(m))
            .ok_or_else(|| format!("vm_intrinsics.toml [indy] 缺 {key}"))
    }
}

/// `[indy]` 分量处理入口：值须为 `类.方法:描述符`；登记了 concat（需 concat_stringify）或
/// object_methods（需三项）引导方法而缺项时报错——清单是唯一真源，不做回落
fn indy_helpers(vm: &Table) -> Result<BTreeMap<String, String>, InputError> {
    let sec = section(vm, "indy");
    let has = |kind: IndyKind| -> Result<bool, InputError> { Ok(!str_list(sec, kind.as_str(), "indy")?.is_empty()) };
    let (concat, om) = (has(IndyKind::Concat)?, has(IndyKind::ObjectMethods)?);
    let mut out = BTreeMap::new();
    for (key, need) in [("concat_stringify", concat || om), ("component_hash", om), ("component_equals", om)] {
        match sec.and_then(|s| s.get(key)) {
            Some(v) => {
                let m = v
                    .as_str()
                    .filter(|m| split_member(m).is_some())
                    .ok_or_else(|| InputError::Manifest(format!("indy.{key}：应为 `类.方法:描述符`：{v}")))?;
                out.insert(key.to_string(), m.to_string());
            }
            None if need => {
                return Err(InputError::Manifest(format!(
                    "vm_intrinsics.toml [indy] 缺 {key}（已登记的 concat / object_methods 引导方法需要它）"
                )))
            }
            None => {}
        }
    }
    Ok(out)
}

/// `类.方法:描述符` → `(类, 方法, 描述符)`
fn boot_phases(seeds: &Table) -> Result<Vec<(String, Vec<i32>)>, InputError> {
    let Some(arr) = seeds.get("boot_init").and_then(|s| s.get("phases")).and_then(|v| v.as_array()) else {
        return Ok(Vec::new());
    };
    let bad = |what: &str| InputError::Manifest(format!("boot_init.phases：{what}"));
    let mut out = Vec::new();
    for p in arr {
        let call = p.get("call").and_then(|v| v.as_str()).ok_or_else(|| bad("缺 call"))?;
        let mut args = Vec::new();
        for a in p.get("args").and_then(|v| v.as_array()).into_iter().flatten() {
            args.push(match a {
                toml::Value::Boolean(b) => *b as i32,
                toml::Value::Integer(i) => i32::try_from(*i).map_err(|_| bad(&format!("{call}：整数实参越界")))?,
                _ => return Err(bad(&format!("{call}：args 只能是布尔 / 整数"))),
            });
        }
        out.push((call.to_string(), args));
    }
    Ok(out)
}

fn split_member(m: &str) -> Option<(&str, &str, &str)> {
    let (head, desc) = m.split_once(':')?;
    let (owner, name) = head.rsplit_once('.')?;
    desc.starts_with('(').then_some((owner, name, desc))
}

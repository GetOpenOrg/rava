//! 发射层读取的 runtime 清单。
//!
//! 组合类型层清单 [`ty::Manifest`]（txt 清单）与三份结构化清单中发射层需要的部分。三份结构化清单
//! 只由 [`closure::manifest::Manifest`] 解析与校验（唯一真源）；本模块是它的投影，不另行解析 TOML：
//! - closure.toml：`[vm_boundary]`（含 `translate_nested` / `clinit_carried`）；
//! - seeds.toml：`[module_resources]`；
//! - vm_intrinsics.toml：`[[intrinsic]]`、`[caller_sensitive]`、`[sigpoly]`、`[class_path]`、`[indy]`、`[vm_constants]`（含 `injected_statics` 子表）、`[vm_state]`、`[escape]`、直连反射入口。
//!
//! 文件缺失视为空表；格式约定违反时返回 [`InputError::Manifest`]。

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub use closure::manifest::{EscapeEffect, IndyKind};
use closure::manifest::{InjectedValue, Manifest};

use crate::prune::VmConstants;
use crate::InputError;

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
    /// VM 内建成员
    pub intrinsic_members: BTreeSet<String>,
    /// 逃逸分析效果登记：`类.方法:描述符` → 效果
    pub escape_effects: BTreeMap<String, EscapeEffect>,
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
    /// 栈帧别名：特化入口（`类.方法:描述符`）→ 它所特化的方法。特化入口是调用点改写的产物（直连反射调用
    /// `[facts.reflect.direct_invokers]` 的 helper），执行的就是原方法，其帧对栈遍历的全部消费方（getCallerClass /
    /// getClassContext 的反射帧跳过、StackWalker 的反射帧过滤、Throwable 栈）都呈现为原方法的帧
    pub frame_aliases: BTreeMap<String, String>,
}

impl RuntimeManifest {
    /// 从 `runtime/java_runtime` 目录读取
    pub fn load(runtime_dir: &Path) -> Result<RuntimeManifest, InputError> {
        let ty = ty::Manifest::load(runtime_dir).map_err(|e| InputError::Manifest(e.to_string()))?;
        let m = Manifest::load(runtime_dir).map_err(InputError::Manifest)?;
        Ok(Self::project(ty, &m))
    }

    /// 由分析器清单投影（发射层需要的部分，按确定顺序收集）
    pub fn project(ty: ty::Manifest, m: &Manifest) -> RuntimeManifest {
        let set = |it: &mut dyn Iterator<Item = &str>| it.map(str::to_string).collect::<BTreeSet<_>>();
        let helpers = &m.indy_helpers;
        RuntimeManifest {
            ty,
            vm_boundary_classes: set(&mut m.vm_boundary_classes()),
            release: m.release_entries().to_vec(),
            vm_clinit_carried: set(&mut m.vm_clinit_carried_classes()),
            vm_injected_statics: m
                .injected_statics()
                .map(|(k, v)| {
                    let expr = match v {
                        InjectedValue::Expr(e) => format!("crate::{e}"),
                        InjectedValue::Int(n) => format!("{n}i64"),
                        InjectedValue::Bool(b) => b.to_string(),
                    };
                    (k.to_string(), expr)
                })
                .collect(),
            module_resource_paths: m.emit.module_resource_paths.clone(),
            class_path_readers: m.emit.class_path_readers.clone(),
            intrinsic_members: set(&mut m.intrinsic_members()),
            escape_effects: m.emit.escape_effects.clone(),
            caller_sensitive_annotations: set(&mut m.caller_sensitive_annotations()),
            sigpoly_callsite_typed: m.emit.sigpoly_callsite_typed.iter().cloned().collect(),
            indy_kinds: m.indy_kinds().map(|(k, v)| (k.to_string(), v)).collect(),
            indy_helpers: [
                ("concat_stringify", &helpers.stringify),
                ("component_hash", &helpers.hash),
                ("component_equals", &helpers.equals),
            ]
            .into_iter()
            .filter_map(|(k, v)| v.as_ref().map(|v| (k.to_string(), v.clone())))
            .collect(),
            vm_constants: VmConstants {
                null_returns: m.emit.null_returns.iter().cloned().collect(),
                null_to_false: set(&mut m.null_to_false_members()),
            },
            vm_state: m.vm_state.clone(),
            frame_aliases: m.direct_invokers.entries().map(|(entry, x)| (x.helper.to_string(), entry.to_string())).collect(),
        }
    }

    /// 方法 (类, 名, 描述符) 的栈帧别名（[`Self::frame_aliases`]）：所特化方法的 (类, 名, 描述符)
    pub fn frame_alias(&self, class: &str, name: &str, desc: &str) -> Option<(&str, &str, &str)> {
        self.frame_aliases.get(&format!("{class}.{name}:{desc}")).and_then(|m| split_member(m))
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

/// `类.方法:描述符` → `(类, 方法, 描述符)`
fn split_member(m: &str) -> Option<(&str, &str, &str)> {
    let (head, desc) = m.split_once(':')?;
    let (owner, name) = head.rsplit_once('.')?;
    desc.starts_with('(').then_some((owner, name, desc))
}

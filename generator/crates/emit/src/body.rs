//! 方法体生成接口（P4c/P5b 接入点）。
//!
//! 发射层只负责类骨架：方法体文本经 [`MethodBodyEmitter`] 取得。方法体生成期间登记的
//! 事实（继承成员需求、lambda 实现名引用、SAM 站点）以 [`BodyEffects`] 返回，由发射层
//! 并入每项目状态（Python 中是模块级全局账本）。

use std::collections::{BTreeMap, BTreeSet};

use classfile::Method;
use ty::ClassInfo;

use crate::ctx::EmitCtx;

/// 一次方法体生成请求（Python `gen_method_body(method, class_info, ...)` 的参数）
pub struct BodyRequest<'a> {
    /// 发射类（方法体的 `this` 所属类）
    pub class: &'a ClassInfo,
    /// 被翻译的方法。接口方法展开到实现类时是改写后的副本：
    /// 泛型签名里的接口类型变量已换成实现类视角的类型实参（见 `type_var_view`）
    pub method: &'a Method,
    /// 方法字节码的声明类（展开接口方法时为接口，否则同 `class`）；
    /// `EmitCtx::input.code(declaring_class, method)` 取规范化方法体
    pub declaring_class: &'a str,
    /// 接口方法展开时的类型变量代换（接口形参 → 实现类视角实参）；局部变量表签名同样需要代换
    pub type_var_view: Option<&'a BTreeMap<String, String>>,
    pub class_type_params: &'a [String],
    pub overloaded_names: &'a BTreeSet<String>,
    /// 显式 Rust 名（None：由方法体生成器按自身规则命名）
    pub rust_name: Option<&'a str>,
    pub in_vtable_body: bool,
    /// 发射位点名（存根兜底审计的位点分解：`main` / `clinit` / `iface-default` / `iface-lambda` /
    /// `iface-private` / `iface-inherit` / `iface-special` / `bridge` / `super-inherit`）
    pub site: &'static str,
}

/// 方法体生成期间登记的事实
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct BodyEffects {
    /// 继承成员需求 (接收者 binary, Java 方法名, 参数描述符部分 `(...)`)
    pub requests: Vec<(String, String, String)>,
    /// invokedynamic 实现方法引用 (类, Java 方法名, Rust 名)
    pub lambda_refs: Vec<(String, String, String)>,
    /// SAM 合成对象站点 (接口 binary, SAM 描述符, 当前类)
    pub sam_sites: Vec<(String, String, String)>,
}

/// 方法体生成结果：完整函数文本（签名 + `{` 体 `}`）与登记事实
#[derive(Debug, Clone, Default)]
pub struct BodyOutput {
    pub text: String,
    /// 实例方法的结构化签名（构造器 / 静态方法为 None）
    pub sig: Option<ty::FnSig>,
    pub effects: BodyEffects,
}

/// 方法体生成失败
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyError {
    /// 语义限制（Python fallback 白名单：CfgError 家族）→ 发射层退化为 panic 存根
    Fallback(String),
    /// 代码缺陷 / 未接入 → 穿透为发射失败
    Fatal(String),
}

/// 方法体生成日志：审计事实与逐方法耗时。随类账本增量按发射序合并，汇总结果与串行发射逐项一致
#[derive(Debug, Default)]
pub struct BodyLog {
    pub events: Vec<BodyEvent>,
    /// 逐方法体生成耗时（`类.名:描述符`，发射序；性能观测用）
    pub timings: Vec<(String, std::time::Duration)>,
}

impl BodyLog {
    pub fn append(&mut self, other: BodyLog) {
        self.events.extend(other.events);
        self.timings.extend(other.timings);
    }
}

/// 一条方法体审计事实（汇总见 [`crate::method_bodies::BodyAudit::from_log`]）
#[derive(Debug)]
pub enum BodyEvent {
    /// 一次方法体生成：等价性审计项、instanceof 折叠次数、控制流审计数据
    Method { equiv: Vec<instr::Audit>, instanceof_folds: usize, cfg: Option<method::CfgStats> },
    /// 控制流语义限制退化为存根（方法键, 原因, 发射位点）
    StubFallback { key: String, reason: String, site: &'static str },
}

/// 方法体生成器（生产实现 [`crate::method_bodies::MethodBodies`]）。
/// 并行发射下跨线程共享：生成器本身不可变，生成期事实写入调用方给的 [`BodyLog`]
pub trait MethodBodyEmitter: Sync {
    fn emit_body(&self, ctx: &EmitCtx<'_>, req: &BodyRequest<'_>, log: &mut BodyLog) -> Result<BodyOutput, BodyError>;
}

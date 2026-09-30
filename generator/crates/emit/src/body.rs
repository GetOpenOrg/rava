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
    /// 发射类（方法体的 `this` 所属类；golden 键取其名）
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

/// 方法体生成器（P4c `method` crate 实现；golden 测试以回放实现）
pub trait MethodBodyEmitter {
    fn emit_body(&mut self, ctx: &EmitCtx<'_>, req: &BodyRequest<'_>) -> Result<BodyOutput, BodyError>;
}

/// 未接入方法体生成器：`--skeleton-only` 之外的任何方法体请求都报错
pub struct NoBodies;

impl MethodBodyEmitter for NoBodies {
    fn emit_body(&mut self, _ctx: &EmitCtx<'_>, req: &BodyRequest<'_>) -> Result<BodyOutput, BodyError> {
        Err(BodyError::Fatal(format!(
            "P4c/P5b 未接入：{}.{}:{}",
            req.class.name(),
            req.method.name,
            req.method.desc
        )))
    }
}

/// 骨架模式：签名同存根（形参名取 `argN`），方法体为 `/*BODY key*/` 占位（`rava build --skeleton-only`）
pub struct PlaceholderBodies;

impl MethodBodyEmitter for PlaceholderBodies {
    fn emit_body(&mut self, ctx: &EmitCtx<'_>, req: &BodyRequest<'_>) -> Result<BodyOutput, BodyError> {
        let name = req.rust_name.unwrap_or(&req.method.name);
        let stub = crate::class_writer::stub::native_stub(
            ctx,
            req.class,
            req.method,
            name,
            req.class_type_params,
            &BTreeMap::new(),
        );
        let text = format!(
            "{} {{\n    /*BODY {}.{}:{}*/\n}}",
            stub.sig,
            req.class.name(),
            req.method.name,
            req.method.desc
        );
        Ok(BodyOutput { text, effects: BodyEffects::default() })
    }
}

//! 方法调用翻译（← `instr/invoke.py` / `invoke_virtual.py` / `invoke_sig.py`）。
//!
//! 模块分工：
//! - [`sig`]：被调方法的签名解析（形参 / 返回类型的泛型视角、接收者实参映射、实参强转）；
//! - [`recv`]：虚调用的接收者解析（`member_owner` 中依赖栈状态的部分）；
//! - [`turbofish`]：泛型类静态调用路径的 turbofish；
//! - [`static_call`]：invokestatic；
//! - [`virtual_`]：invokevirtual / invokeinterface。

pub mod recv;
pub mod sig;
pub mod static_call;
pub mod turbofish;
pub mod virtual_;

use classfile::MemberRef;

/// 调用点的方法引用（`parse_method_ref` 的结构化形态：常量池类 binary 名 + 描述符拆分）
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallRef {
    /// 常量池类 binary 名
    pub owner: String,
    pub name: String,
    /// 完整描述符 `(..)R`
    pub desc: String,
    /// 形参描述符（JVM 类型串）
    pub params: Vec<String>,
    /// 返回描述符
    pub ret: String,
}

impl CallRef {
    pub fn new(m: &MemberRef) -> CallRef {
        CallRef::with(&m.owner, &m.name, &m.desc)
    }

    pub fn with(owner: &str, name: &str, desc: &str) -> CallRef {
        CallRef {
            owner: owner.to_string(),
            name: name.to_string(),
            desc: desc.to_string(),
            params: ty::type_map::parse_descriptor_params(desc),
            ret: ty::type_map::parse_descriptor_return(desc).to_string(),
        }
    }

    /// 形参部分 `(..)`
    pub fn param_desc(&self) -> &str {
        crate::owner::param_part(&self.desc)
    }
}

//! 单类发射记录（`inherited_gen.ClassEmission` 的移植）：两阶段生成的第一阶段产物。
//!
//! 第一阶段生成全部类文本（期间方法体登记继承成员需求 / SAM 站点 / lambda 名）；
//! 第二阶段（接口实现、继承成员、SAM 合成、反射分派）按记录补齐文本后统一落盘。

use std::path::PathBuf;

#[derive(Debug, Clone, Default)]
pub struct ClassEmission {
    pub binary_name: String,
    /// 引用本 crate 外类型的前缀（`crate` / `java_runtime`）
    pub crate_prefix: String,
    pub path: PathBuf,
    /// 目标路径是 runtime/ 手写真源（落盘时跳过）
    pub handwritten: bool,
    /// 所属 crate 名（`java_runtime` / `user` / lib crate 名）
    pub crate_name: String,
    pub text: String,
}

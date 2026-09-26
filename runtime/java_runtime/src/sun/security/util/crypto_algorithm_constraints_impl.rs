//! `sun/security/util/CryptoAlgorithmConstraints` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! JDK 25 / 新版 JDK 21 更新（jdk.crypto.disabledAlgorithms 回移）在 `MessageDigest` / `Cipher`
//! 等引擎的 `getInstance` 路径上调用 `permits(service, algorithm)`，按安全属性
//! `jdk.crypto.disabledAlgorithms` 禁用指定服务。JDK 默认 `java.security` 中该属性未设置
//!（注释态）——禁用集为空，`permits` 恒 true。原生二进制无 `java.security` 配置层，取默认。

use crate::prelude::*;
use super::crypto_algorithm_constraints::implref::CryptoAlgorithmConstraints;

impl CryptoAlgorithmConstraints {
    /// `permits(String service, String algo)`：禁用集为空 → 恒允许。
    /// 与继承的实例重载 `permits(Set, String, AlgorithmParameters)` 同名 → 按描述符改名。
    #[jvm_boundary]
    pub fn permits_str_str(_service: String, _algo: String) -> Result<bool> {
        Ok(true)
    }
}

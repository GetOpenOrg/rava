//! `sun/security/jca/JCAUtil` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! K-JCA：`getDefSecureRandom()` 是 `Cipher.init(opmode, key)` 等无随机源重载的缺省随机源。
//! 首版返回已分配、未绑定实现的 SecureRandom 占位对象：ECB 等不消费随机源的模式可用；
//! 真正取随机数（CBC 加密生成 IV、密钥生成）命中 SecureRandom 的精确存根，按需补全
//! （SecureRandom 服务种子见 docs/plans/2026-09-25-jca-service-registry.md §三）。

use crate::prelude::*;
use super::jca_util::JCAUtil;
use crate::java::security::SecureRandom;

impl JCAUtil {
    /// `getDefSecureRandom()`：见模块说明。
    #[jvm_boundary]
    pub fn getDefSecureRandom() -> Result<SecureRandom> {
        let mut r = SecureRandom::default();
        r._init_not_null();
        Ok(r)
    }
}

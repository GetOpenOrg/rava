//! `sun/security/jca/JCAUtil` 手写伴生：内部边界类，按调用链按需实现（K-2 规则）。
//!
//! K-JCA：`getDefSecureRandom()` 是 `Cipher.init(opmode, key)` 等无随机源重载的缺省随机源——
//! JDK 为进程内单例 `new SecureRandom()`（翻译字节码：缺省 PRNG 经 provider 列表取 SUN 的
//! NativePRNG）；此处同为进程内单例。

use crate::prelude::*;
use super::jca_util::JCAUtil;
use crate::java::security::secure_random::SecureRandom;

crate::__process_static! {
    static DEF: crate::sync_model::__RefSlot<Option<SecureRandom>> = crate::sync_model::__RefSlot::new(None);
}

impl JCAUtil {
    /// `getDefSecureRandom()`：见模块说明。
    #[jvm_boundary]
    pub fn getDefSecureRandom() -> Result<SecureRandom> {
        if let Some(r) = DEF.with(|d| d.borrow().as_ref().map(Clone::clone)) {
            return Ok(r);
        }
        let r = SecureRandom::new()?;
        DEF.with(|d| *d.borrow_mut() = Some(Clone::clone(&r)));
        Ok(r)
    }
}

//! `java/security/Security` 手写伴生（`java/security/` 属边界前缀，按调用链按需实现，K-2 规则）。
//!
//! JDK 的安全属性来自 `$JAVA_HOME/conf/security/java.security`（启动时载入 Properties）。
//! 原生二进制以编译期嵌入的生效属性表为只读基线（`jdk_resources::security_property`），
//! `setProperty` 写入进程内覆盖层——读取先覆盖层后基线，与 JDK 的 Properties 语义一致。
//! 消费方：`ObjectInputFilter$Config.<clinit>` 读 `jdk.serialFilter` / `jdk.serialFilterFactory`
//! （S-66 对象序列化链，基线未定义 → null）。

use crate::prelude::*;
use super::security::Security;
use std::collections::HashMap;

crate::__process_static! {
    static OVERRIDES: crate::sync_model::__RefSlot<HashMap<std::string::String, std::string::String>> =
        crate::sync_model::__RefSlot::new(HashMap::new());
}

impl Security {
    /// `getProperty(String)`：覆盖层 → 基线；值按 JDK 语义 trim；未定义 → null。
    #[jvm_boundary]
    pub fn getProperty(key: String) -> Result<String> {
        if key.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let k = format!("{}", key);
        let v = OVERRIDES.with(|o| o.borrow().get(&k).cloned())
            .or_else(|| crate::jdk_resources::security_property(&k).map(str::to_owned));
        Ok(match v {
            Some(v) => String::from(v.trim()),
            None => String::default(),
        })
    }

    /// `setProperty(String, String)`：写入进程内覆盖层（key / datum 为 null → NPE，同 Properties）。
    #[jvm_boundary]
    pub fn setProperty(key: String, datum: String) -> Result<()> {
        if key.is_jvm_null() || datum.is_jvm_null() {
            return Err(JvmError::null_pointer());
        }
        let (k, v) = (format!("{}", key), format!("{}", datum));
        OVERRIDES.with(|o| o.borrow_mut().insert(k, v));
        Ok(())
    }
}

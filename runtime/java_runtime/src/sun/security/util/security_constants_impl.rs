//! `sun/security/util/SecurityConstants` 手写伴生：内部边界类，按调用链按需实现（K-2 规则），
//! 其余静态常量（各类 Permission 实例）保持 panic 存根。

use crate::prelude::*;
use super::security_constants::SecurityConstants;
use crate::sun::security::action::GetPropertyAction;

impl SecurityConstants {
    /// static final `PROVIDER_VER`：JDK `<clinit>` 同源——
    /// `GetPropertyAction.privilegedGetProperty("java.specification.version")`
    ///（provider 版本串，SunJCE / Sun 等内建 provider 构造器传给 `Provider(name, versionStr, info)`）。
    #[jvm_boundary(upcalls = "sun/security/action/GetPropertyAction.privilegedGetProperty:(Ljava/lang/String;)Ljava/lang/String;")]
    pub fn PROVIDER_VER() -> Result<String> {
        GetPropertyAction::privilegedGetProperty_str(String::from("java.specification.version"))
    }
}

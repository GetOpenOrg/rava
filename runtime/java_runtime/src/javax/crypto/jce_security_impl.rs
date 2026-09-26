//! `javax/crypto/JceSecurity` 手写伴生：VM 边界类（vm_boundary.txt），按调用链按需实现（K-2 规则）。
//!
//! K-JCA：JDK 的 JCE 管辖策略在 `<clinit>` 读 JDK 安装目录的 policy 文件、并校验 provider jar
//! 签名。原生二进制等价于 JDK 9+ 默认安装（`crypto.policy=unlimited`，JDK 内建 provider 恒
//! 可信）：`canUseProvider` 恒 true、`isRestricted` 恒 false（Cipher 据此取
//! `CryptoAllPermission.INSTANCE`，密钥长度不设上限）。

use crate::prelude::*;
use super::jce_security::implref::JceSecurity;
use crate::java::security::Provider;
use crate::java::lang::Exception;

impl JceSecurity {
    /// `canUseProvider(Provider)`：内建 provider 恒可用（JDK 对签名 provider 的校验结果）。
    #[jvm_boundary]
    pub fn canUseProvider(_p: Provider) -> Result<bool> {
        Ok(true)
    }

    /// `isRestricted()`：默认 unlimited 策略 → false。
    #[jvm_boundary]
    pub fn isRestricted() -> Result<bool> {
        Ok(false)
    }

    /// `getVerificationResult(Provider)`：内建 provider 校验恒通过 → null。
    #[jvm_boundary]
    pub fn getVerificationResult(_p: Provider) -> Result<Exception> {
        Ok(Exception::default())
    }

    // 以下成员在 unlimited 策略下不可达（isRestricted 恒 false，Cipher 不查策略、不校验
    // 豁免 jar）。显式声明为精确存根：边界静态缺口补译（callchain 门 2）据此不翻译其字节码
    // ——verifyExemptJar → ProviderVerifier 会展开整个 JAR 签名校验栈（JarVerifier /
    // ManifestDigester / SignatureFileVerifier）。
    #[jvm_boundary]
    pub fn getDefaultPolicy() -> Result<super::CryptoPermissions> {
        panic!("stub: javax/crypto/JceSecurity.getDefaultPolicy:()Ljavax/crypto/CryptoPermissions;（unlimited 策略下不可达）")
    }

    #[jvm_boundary]
    pub fn getExemptPolicy() -> Result<super::CryptoPermissions> {
        panic!("stub: javax/crypto/JceSecurity.getExemptPolicy:()Ljavax/crypto/CryptoPermissions;（unlimited 策略下不可达）")
    }

    #[jvm_boundary]
    pub fn verifyExemptJar(_codeBase: crate::java::net::URL) -> Result<super::CryptoPermissions> {
        panic!("stub: javax/crypto/JceSecurity.verifyExemptJar:(Ljava/net/URL;)Ljavax/crypto/CryptoPermissions;（unlimited 策略下不可达）")
    }
}
